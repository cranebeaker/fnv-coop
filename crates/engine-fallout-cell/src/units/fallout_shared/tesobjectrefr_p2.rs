//! `fallout shared/tesobjectrefr.cpp` (Xbox PDB source unit), part 2: its functions from `00563d80` up to
//! (not including) `0056a860` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectrefr`]; anything public there may be used here.
//!
//! # Where this file stops
//!
//! The first batch holds the 40 functions from `00563d80` to `00567400`: the
//! sequence save/load helpers of a reference's controller manager, the
//! reference factory, the form-flag accessors (persistent, targeted, ...),
//! `InitScript`, `RunScript`, `InitAnimation` and `BuildKFFileList`.
//!
//! The second batch holds the 40 functions from `00567470` to `00568fa0`: the
//! scale setter, the ownership accessors (owner, global, rank, evil faction),
//! the encounter zone and calc level, the furniture marker accessors
//! (used / reserved bits, first free, closest free, marker at index), the
//! radius, and the teleport extras.
//!
//! The third batch holds the last 40 functions of the range, `00569000` to
//! `0056a4a0`: the teleport rotation, the lock accessors (`AddLock`,
//! `GetLock`, `Lock`, `UnLock`), the emittance check, the multi-bound
//! accessors, the linked-reference list accessors, the link and activate
//! line drawing, the activate children timers and the decal creation. With
//! it every function of the range from `00563d80` up to `0056a860` is
//! translated.
//!
//! # Conventions of the exe worth knowing
//!
//! - MSVC pushes a `float` argument as `PUSH ECX; FSTP [ESP]`: the `PUSH ECX`
//!   only reserves the four bytes, the value is the stored `float`.
//! - A word pushed before a call to a function that pops fewer words
//!   belongs to a later call; every call here was matched against the
//!   callee's `RET n` (so `GetModel` `005715d0`, `Get3D` (virtual `+0x1d0`)
//!   and the `NiControllerManager` getters take no stack words).
//! - Scope guards (`00404eb0`, 4 bytes on the stack) and C++ exception
//!   unwinding are modelled only as far as the guard's constructor and
//!   destructor calls; the unwinding states and the stack-protector
//!   cookies are not translated.

#[allow(unused_imports)]
use super::tesobjectrefr::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees outside this file, by exe address.

/// `TESObjectREFR::GetBaseObject` as the map names `007af430`: the
/// reference's base form (`pObjectReference`), 0 when it has none.
const GET_BASE_FORM: u32 = 0x007a_f430;
/// `TESForm::cFormType` (`00401170`, the byte at +4), `iFormID`
/// (`0084e3a0`, +0x0c), `iFormFlags` getter (`0044ddc0`, +8) and setter
/// (`00403550`).
const FORM_TYPE: u32 = 0x0040_1170;
const FORM_ID: u32 = 0x0084_e3a0;
const FORM_GET_FLAGS: u32 = 0x0044_ddc0;
const FORM_SET_FLAGS: u32 = 0x0040_3550;
/// `iFormFlags & 0x20` (deleted) and `iFormFlags & 0x800` (disabled).
const FORM_IS_DELETED: u32 = 0x0044_0d80;
const FORM_IS_DISABLED: u32 = 0x0044_0da0;
/// `this + 0x44`: the reference's embedded `ExtraDataList`.
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `operator new(size)` and the scope guard the allocation tag is set by.
const OPERATOR_NEW: u32 = 0x0040_1000;
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
const SCOPE_GUARD_SIZE: u32 = 4;
/// Path of `TESObjectREFR.cpp` as the allocation scope guard records it.
const SOURCE_FILE: u32 = 0x0102_fc20;
/// The error log (`005b5e40`, cdecl: format, then its arguments).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `TESObjectREFR::GetModel` (`005715d0`): the base form's model path.
const GET_MODEL: u32 = 0x0057_15d0;
/// The reference's loaded 3D (`0043fcd0`), a 3D object's first controller
/// (`0043b230`), and the checked cast of a controller to a controller
/// manager (`00653270`, cdecl, with the record at `011f36ac`).
const GET_LOADED_3D: u32 = 0x0043_fcd0;
const GET_CONTROLLER: u32 = 0x0043_b230;
const CHECKED_CAST: u32 = 0x0065_3270;
const MANAGER_TYPE: u32 = 0x011f_36ac;
/// `NiControllerManager` methods: sequence count (`00495d00`), sequence at
/// index (`00495d20`), `DeactivateAll(time)` (`0048fef0`), the manager
/// switch (`0047aa40(flag)`), activate (`0047aab0`), "has sequences"
/// (`004f05a0`), lookup by `NiFixedString` (`0047a520`) and
/// `0047b220(sequence, time)`.
const MANAGER_SEQUENCE_COUNT: u32 = 0x0049_5d00;
const MANAGER_SEQUENCE_AT: u32 = 0x0049_5d20;
const MANAGER_DEACTIVATE_ALL: u32 = 0x0048_fef0;
const MANAGER_SET_FLAG: u32 = 0x0047_aa40;
const MANAGER_ACTIVATE: u32 = 0x0047_aab0;
const MANAGER_HAS_SEQUENCES: u32 = 0x004f_05a0;
const MANAGER_FIND_SEQUENCE: u32 = 0x0047_a520;
const MANAGER_FINISH_SEQUENCE: u32 = 0x0047_b220;
/// A sequence's methods: its name holder (`00413f40`, `this + 8`), the C
/// string of a name holder (`0043b1b0`), `LowProcess::GetGenericLocation` as
/// the map names `008041a0` (non-zero for the sequences that are saved),
/// the time offset (`00759450`, ST0), two more `float` getters (`00508100`
/// and `00639aa0`, ST0), the offset setter (`0098adb0`), and the save and
/// load through a `BGSSaveGameBuffer` (`004efc90`, `004efd10`).
const SEQUENCE_NAME_HOLDER: u32 = 0x0041_3f40;
const NAME_TEXT: u32 = 0x0043_b1b0;
const SEQUENCE_IS_GENERIC: u32 = 0x0080_41a0;
const SEQUENCE_OFFSET_TIME: u32 = 0x0075_9450;
const SEQUENCE_LIMIT_TIME: u32 = 0x0050_8100;
const SEQUENCE_DURATION: u32 = 0x0063_9aa0;
const SEQUENCE_SET_OFFSET: u32 = 0x0098_adb0;
const SEQUENCE_SAVE_TO_BUFFER: u32 = 0x004e_fc90;
const SEQUENCE_LOAD_FROM_BUFFER: u32 = 0x004e_fd10;
/// A `BSAnimGroupSequence` on the stack: its constructor (`0049baa0`) and
/// destructor (`004eeb00`); the object is 0x84 bytes.
const SEQUENCE_CONSTRUCT: u32 = 0x0049_baa0;
const SEQUENCE_DESTRUCT: u32 = 0x004e_eb00;
const SEQUENCE_OBJECT_SIZE: u32 = 0x84;
/// `bhkNiCollisionObject::ResetSim(node, 1)` (`00c6bd00`, cdecl), the 12-byte
/// velocity record maker (`0043d410(record, value, a, b)`) and the two
/// functions that give a 3D object such a record (set `00a59c60`, add
/// `00a59c90`).
const RESET_SIM: u32 = 0x00c6_bd00;
const MAKE_VELOCITY: u32 = 0x0043_d410;
const SET_3D_VELOCITY: u32 = 0x00a5_9c60;
const ADD_3D_VELOCITY: u32 = 0x00a5_9c90;
/// Whether a 3D object is an actor's (`004b5c80`, cdecl).
const IS_ACTOR_3D: u32 = 0x004b_5c80;
/// `strcmp` (`00ec6da0`, cdecl) and the exe's own string compare
/// (`00408b20`, cdecl; 0 when equal).
const STRCMP: u32 = 0x00ec_6da0;
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `_strrchr` (`00ec6e30`) and the unit's wrapper over it (`0040ab30`),
/// `_strnicmp` (`00ec7ec0`), `strstr` (`00ec7750`), `sprintf` (`00ec623a`),
/// and the bounded copy and append (`00406d30(dest, size, source)`,
/// `00406d50`), all cdecl.
const STRRCHR: u32 = 0x00ec_6e30;
const STRRCHR_WRAPPER: u32 = 0x0040_ab30;
const STRNICMP: u32 = 0x00ec_7ec0;
const STRSTR: u32 = 0x00ec_7750;
const SPRINTF: u32 = 0x00ec_623a;
const BOUNDED_COPY: u32 = 0x0040_6d30;
const BOUNDED_APPEND: u32 = 0x0040_6d50;
/// `NiFixedString` constructor from a C string (`00438170`) and destructor
/// (`004381b0`); the C string pointers of the two idle sequence names and of
/// "Unequip" are kept in the exe's data.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const NAME_UNEQUIP: u32 = 0x0119_7b5c;
const NAME_IDLE_A: u32 = 0x0119_77d8;
const NAME_IDLE_B: u32 = 0x0119_7820;
/// `float` constants of the exe: `FLT_MAX` (`0102f510`), `-1.0f`
/// (`01012054`), and the `double`s `-1.0` (`0101a6b0`), `0.0` (`01012060`),
/// `20.0` (`0102fc70`) and `0.0166666` (`0102fc68`, with the `float` at
/// `0102fc64`); `011c3c08` is the default blend time.
const FLOAT_MAX: u32 = 0x0102_f510;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const STEP_DIVISOR: u32 = 0x0102_fc70;
const MIN_STEP_DOUBLE: u32 = 0x0102_fc68;
const MIN_STEP_FLOAT: u32 = 0x0102_fc64;
const DEFAULT_BLEND_TIME: u32 = 0x011c_3c08;
/// The `BGSSaveGameBuffer` / `BGSLoadGameBuffer` methods: start and end of a
/// variable sized value (`00865f20`, `00865ff0`; `00864a60` loads the
/// count), the save of a string (`00865e70(text, 0)`) and the load of one
/// (`008649a0(buffer)`).
const SAVE_START_SIZED: u32 = 0x0086_5f20;
const SAVE_END_SIZED: u32 = 0x0086_5ff0;
const SAVE_STRING: u32 = 0x0086_5e70;
const LOAD_START_SIZED: u32 = 0x0086_4a60;
const LOAD_STRING: u32 = 0x0086_49a0;
/// Globals: the data handler (`011c3f2c`), the object with the loading flags
/// (`011ddf38`), the save/load game object (`011de45c`) and the player
/// (`011dea3c`).
const GLOBAL_DATA_HANDLER: u32 = 0x011c_3f2c;
const GLOBAL_LOADING_OBJECT: u32 = 0x011d_df38;
const GLOBAL_SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const GLOBAL_PLAYER: u32 = 0x011d_ea3c;
/// The reference's pointer to its base object (`OBJ_REFR::pObjectReference`
/// inside `data`, +0x20) and its parent cell (+0x40) are read through the
/// layout; `TESChildCell` is the base subobject at +0x18.
const CHILD_CELL_OFFSET: u32 = 0x18;

// ---------------------------------------------------------------------------
// Small helpers.

/// The base form of `this` and its `cFormType`, `None` when it has no base
/// form (the exe asks for the base form twice, as `if (GetBase()) {
/// GetBase()->GetType() }`).
fn base_type(e: &mut Engine, this: u32) -> Option<u32> {
    if e.call(GET_BASE_FORM, &args![this]).u32() == 0 {
        return None;
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    Some(e.call(FORM_TYPE, &args![base]).u32())
}

/// Whether `iFormFlags` has any of `mask`.
fn form_flag_set(e: &mut Engine, this: u32, mask: u32) -> bool {
    e.call(FORM_GET_FLAGS, &args![this]).u32() & mask != 0
}

/// Sets or clears `mask` in `iFormFlags`: read (`0044ddc0`), change, write
/// (`00403550`).
fn update_form_flags(e: &mut Engine, this: u32, mask: u32, set: bool) {
    let flags = e.call(FORM_GET_FLAGS, &args![this]).u32();
    let flags = if set { flags | mask } else { flags & !mask };
    e.call(FORM_SET_FLAGS, &args![this, flags]);
}

/// The C string of a sequence's name (`0043b1b0(00413f40(sequence))`).
fn sequence_name(e: &mut Engine, sequence: u32) -> u32 {
    let holder = e.call(SEQUENCE_NAME_HOLDER, &args![sequence]).u32();
    e.call(NAME_TEXT, &args![holder]).u32()
}

/// The sequence of `manager` whose name is the C string the pointer at
/// `name_global` points to: builds the `NiFixedString`, looks it up and
/// destroys it.
fn find_sequence(e: &mut Engine, manager: u32, name_global: u32) -> u32 {
    let name = e.global::<u32>(name_global);
    e.with_stack(4, |e, fixed| {
        let handle = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
        let sequence = e.call(MANAGER_FIND_SEQUENCE, &args![manager, handle]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        sequence
    })
}

/// Gives the 3D object `node` a velocity record built from `value` and the
/// two flag words: `0043d410(record, value, a, b)`, then `00a59c60`.
fn set_velocity(e: &mut Engine, node: u32, value: f32, a: u32, b: u32) {
    e.with_stack(0xc, |e, record| {
        e.call(MAKE_VELOCITY, &args![record, value, a, b]);
        e.call(SET_3D_VELOCITY, &args![node, record]);
    });
}

/// `blend == -1.0` (the `double` at `0101a6b0`) stands for the default blend
/// time (`float` at `011c3c08`), as the three save/load helpers start.
fn resolve_blend(e: &mut Engine, blend: f32) -> f32 {
    let marker: f64 = e.global(MINUS_ONE_DOUBLE);
    if f64::from(blend) == marker {
        e.global(DEFAULT_BLEND_TIME)
    } else {
        blend
    }
}

/// Activates `sequence` on `manager` with the arguments every caller here
/// uses: `(sequence, 0, 0, 1.0, 0.0, 0)`.
fn activate_sequence(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(
        MANAGER_ACTIVATE,
        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
    );
}

// Translated from 00563d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restarts the sequence of `manager` whose name is `name` and plays it
/// from `time`: returns false when the reference has no loaded 3D or no
/// manager is given, or no sequence has that name. A generic-location
/// sequence is not activated again. The sequence's offset is set from where
/// `time` falls against its offset time (`00759450`) and the other limit
/// (`00508100`), the 3D object's simulation is reset, the 3D gets a velocity
/// record of the first value, the sequence is ended (`005672c0`) and the
/// manager switched off. Not named in the Xbox PDB.
pub fn fn_00563d80(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    manager: Ptr,
    name: Ptr,
    time: f32,
) -> bool {
    let manager = manager.addr();
    let node = e.call(GET_LOADED_3D, &args![this]).u32();
    if manager == 0 || node == 0 {
        return false;
    }
    let count = e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32();
    for index in 0..count {
        let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
        if sequence == 0 {
            continue;
        }
        let text = sequence_name(e, sequence);
        if e.call(STRCMP, &args![text, name]).i32() != 0 {
            continue;
        }
        e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
        e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
        if e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() == 0 {
            activate_sequence(e, manager, sequence);
        }
        let mut record_time = time;
        let mut offset = time;
        let offset_time = e.call(SEQUENCE_OFFSET_TIME, &args![sequence]).f32();
        if time < offset_time {
            record_time = e.call(SEQUENCE_OFFSET_TIME, &args![sequence]).f32();
            let most: f32 = e.global(FLOAT_MAX);
            offset = -most;
        } else {
            let limit = e.call(SEQUENCE_LIMIT_TIME, &args![sequence]).f32();
            if limit < time {
                record_time = e.call(SEQUENCE_LIMIT_TIME, &args![sequence]).f32();
                offset = e.call(SEQUENCE_LIMIT_TIME, &args![sequence]).f32();
            }
        }
        e.call(SEQUENCE_SET_OFFSET, &args![sequence, offset]);
        e.call(RESET_SIM, &args![node, 1u32]);
        set_velocity(e, node, record_time, 1, 0);
        tes_object_refr_end_sequence(e, this, Ptr::new(sequence));
        e.call(MANAGER_FINISH_SEQUENCE, &args![manager, sequence, 0.0f32]);
        e.call(MANAGER_SET_FLAG, &args![manager, 0u32]);
        return true;
    }
    false
}

// Translated from 00563f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SaveControllerManager` (Xbox PDB), a cdecl function of
/// three words (the reference is not used): writes the sequences of
/// `manager` that `008041a0` accepts into the save buffer as a variable
/// sized value holding their count, each as its name (`SaveString`) and its
/// own data (`BSAnimGroupSequence::SaveGame`, `004efc90(buffer, blend)`).
/// A `blend` of `-1.0` stands for the default at `011c3c08`.
pub fn tes_object_refr_save_controller_manager(
    e: &mut Engine,
    manager: Ptr,
    buffer: Ptr,
    blend: f32,
) {
    let manager = manager.addr();
    let buffer = buffer.addr();
    let blend = resolve_blend(e, blend);
    let mut saved = 0u32;
    let start = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    if manager != 0 && e.call(MANAGER_HAS_SEQUENCES, &args![manager]).bool() {
        let mut index = 0u32;
        while index < e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32() {
            let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
            if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0 {
                let name = sequence_name(e, sequence);
                e.call(SAVE_STRING, &args![buffer, name, 0u32]);
                e.call(SEQUENCE_SAVE_TO_BUFFER, &args![sequence, buffer, blend]);
                saved += 1;
            }
            index += 1;
        }
    }
    e.call(SAVE_END_SIZED, &args![buffer, saved, start]);
}

// Translated from 00564010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads what [`tes_object_refr_save_controller_manager`] saved (cdecl,
/// `(manager, buffer, blend)`; the C++ exception state is not translated).
/// With a manager all sequences are deactivated and, when the buffer holds
/// entries, the manager is switched on. For each saved entry the name is
/// read; the sequence of that name is found in the manager, activated if it
/// is not a generic-location one, loaded with `blend`, and, when the buffer's
/// owner (virtual `+8`) has a loaded 3D that `004b5c80` accepts, the 3D gets
/// a run of velocity records stepping through the sequence's duration. An
/// entry without a sequence is loaded into a temporary `BSAnimGroupSequence`
/// (to consume its data). When any entry was found the 3D object's simulation
/// is reset and it gets one record of `blend`. Not named in the Xbox PDB.
pub fn fn_00564010(e: &mut Engine, manager: Ptr, buffer: Ptr, blend: f32) {
    let manager = manager.addr();
    let buffer = buffer.addr();
    let blend = resolve_blend(e, blend);
    let count = e.call(LOAD_START_SIZED, &args![buffer]).u32();
    if manager != 0 {
        e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
        if count != 0 {
            e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
        }
    }
    let mut any = false;
    let owner = e.vcall(buffer, 8, &args![]).u32();
    let node = if owner != 0 {
        e.call(GET_LOADED_3D, &args![owner]).u32()
    } else {
        0
    };
    e.with_stack(0x104, |e, text| {
        for _ in 0..count {
            e.call(LOAD_STRING, &args![buffer, text]);
            let mut found = false;
            if manager != 0 {
                let sequences = e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32();
                for position in 0..sequences {
                    let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, position]).u32();
                    if sequence == 0 {
                        continue;
                    }
                    let name = sequence_name(e, sequence);
                    if e.call(STRING_COMPARE, &args![name, text]).u32() != 0 {
                        continue;
                    }
                    if e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() == 0 {
                        activate_sequence(e, manager, sequence);
                    }
                    e.call(SEQUENCE_LOAD_FROM_BUFFER, &args![sequence, buffer, blend]);
                    if node != 0 && e.call(IS_ACTOR_3D, &args![node]).bool() {
                        add_velocity_run(e, node, sequence, blend);
                    }
                    found = true;
                    any = true;
                    break;
                }
            }
            if !found {
                e.with_stack(SEQUENCE_OBJECT_SIZE, |e, temporary| {
                    e.call(SEQUENCE_CONSTRUCT, &args![temporary]);
                    e.call(SEQUENCE_LOAD_FROM_BUFFER, &args![temporary, buffer, blend]);
                    e.call(SEQUENCE_DESTRUCT, &args![temporary]);
                });
            }
        }
    });
    if any && node != 0 {
        e.call(RESET_SIM, &args![node, 1u32]);
        set_velocity(e, node, blend, 1, 0);
    }
}

/// The run of velocity records `fn_00564010` gives an actor's 3D after
/// loading `sequence`: from `max(blend - total, 0)` to `blend` in steps of
/// `total / 20` (at least `0.0166666`), where `total` is the sequence's
/// duration plus `blend`; every record is added with `00a59c90`.
fn add_velocity_run(e: &mut Engine, node: u32, sequence: u32, blend: f32) {
    let duration = e.call(SEQUENCE_DURATION, &args![sequence]).f64();
    let total = (duration + f64::from(blend)) as f32;
    let mut start = (f64::from(blend) - f64::from(total)) as f32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(start) < zero {
        start = 0.0;
    }
    let divisor: f64 = e.global(STEP_DIVISOR);
    let mut step = (f64::from(total) / divisor) as f32;
    let smallest: f64 = e.global(MIN_STEP_DOUBLE);
    if f64::from(step) < smallest {
        step = e.global(MIN_STEP_FLOAT);
    }
    e.with_stack(0xc, |e, record| {
        let mut time = start;
        while time < blend {
            e.call(MAKE_VELOCITY, &args![record, time, 0u32, 0u32]);
            e.call(ADD_3D_VELOCITY, &args![node, record]);
            time = (f64::from(time) + f64::from(step)) as f32;
        }
    });
}

// Translated from 00564360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Classifies the base form of the reference that owns the 3D object `node`
/// (`0056f930` finds the reference): 0 when there is none; otherwise by the
/// base form's type 1 (types 0x15, 0x1c, 0x20, 0x21), 2 (0x1b, 0x27), 3
/// (0x2a, 0x2b), 4 (0x18, 0x1a, 0x28, 0x29), 5 (0x1d, 0x1f, 0x2e, 0x32,
/// 0x67, 0x6c, 0x73, 0x74) or 6 (every other type). cdecl, one word. Not
/// named in the Xbox PDB.
pub fn fn_00564360(e: &mut Engine, node: Ptr) -> u32 {
    let node = node.addr();
    let mut class = 0;
    if node != 0 {
        let refr = e.call(0x0056_f930, &args![node]).u32();
        if refr != 0 && e.call(GET_BASE_FORM, &args![refr]).u32() != 0 {
            let base = e.call(GET_BASE_FORM, &args![refr]).u32();
            class = match e.call(FORM_TYPE, &args![base]).u32() {
                0x15 | 0x1c | 0x20 | 0x21 => 1,
                0x1b | 0x27 => 2,
                0x2a | 0x2b => 3,
                0x18 | 0x1a | 0x28 | 0x29 => 4,
                0x1d | 0x1f | 0x2e | 0x32 | 0x67 | 0x6c | 0x73 | 0x74 => 5,
                _ => 6,
            };
        }
    }
    class
}

// Translated from 00564480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::CreateReference` (Xbox PDB), cdecl `(kind, flag)`: allocates
/// and constructs the reference object for the reference kind `kind` (the
/// values `fn_00564820` returns), inside an allocation scope guard (tag
/// 0x31). Kind 0x3a is a plain `TESObjectREFR` (0x68 bytes), 0x16 an object
/// of 0x88 bytes; kinds 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40 and 0x69 are
/// objects of 0x1c8, 0x1c0, 0x160, 0x154, 0x154, 0x158 and 0x158 bytes whose
/// constructors take `flag`. Any other kind gives null.
pub fn tes_object_refr_create_reference(e: &mut Engine, kind: u8, flag: u8) -> Ptr {
    let flag = u32::from(flag);
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x31u32, 1u32, SOURCE_FILE, 0x1493u32],
        );
        // (size, constructor, whether the constructor takes `flag`)
        let spec: Option<(u32, u32, bool)> = match kind {
            0x16 => Some((0x88, 0x0092_eb50, false)),
            0x3a => Some((0x68, 0x0055_a2f0, false)),
            0x3b => Some((0x1c8, 0x008d_1f40, true)),
            0x3c => Some((0x1c0, 0x008d_4500, true)),
            0x3d => Some((0x160, 0x009b_7a50, true)),
            0x3e => Some((0x154, 0x009b_3fa0, true)),
            0x3f => Some((0x154, 0x0097_8fd0, true)),
            0x40 => Some((0x158, 0x009b_3160, true)),
            0x69 => Some((0x158, 0x009a_aff0, true)),
            _ => None,
        };
        let mut created = 0;
        if let Some((size, constructor, takes_flag)) = spec {
            let block = e.call(OPERATOR_NEW, &args![size]).u32();
            if block != 0 {
                created = if takes_flag {
                    e.call(constructor, &args![block, flag]).u32()
                } else {
                    e.call(constructor, &args![block]).u32()
                };
            }
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
        Ptr::new(created)
    })
}

// Translated from 00564820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference kind to create for `this`'s base form (the argument of
/// [`tes_object_refr_create_reference`]): 0x3a without a base form or for
/// most types, 0x3b for type 0x2a, 0x3c for 0x2b, and for type 0x33 by the
/// base form's `004fd4f0` value: 0x10000 gives 0x3d, 0x20000 0x3e, 0x40000
/// 0x3f, 0x80000 0x40, 0x100000 0x69. Not named in the Xbox PDB.
pub fn fn_00564820(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut kind = 0x3a;
    if e.call(GET_BASE_FORM, &args![me]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        match e.call(FORM_TYPE, &args![base]).u32() {
            0x2a => kind = 0x3b,
            0x2b => kind = 0x3c,
            0x33 => {
                let base = e.call(GET_BASE_FORM, &args![me]).u32();
                match e.call(0x004f_d4f0, &args![base]).i32() {
                    0x10000 => kind = 0x3d,
                    0x20000 => kind = 0x3e,
                    0x40000 => kind = 0x3f,
                    0x80000 => kind = 0x40,
                    0x100000 => kind = 0x69,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    kind
}

// Translated from 00564900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `kind` (a value of [`fn_00564820`]) is one of the kinds
/// 0x3a to 0x40 or 0x69. cdecl, one word. Not named in the Xbox PDB.
pub fn fn_00564900(_e: &mut Engine, kind: i32) -> bool {
    (0x3a..=0x40).contains(&kind) || kind == 0x69
}

// Translated from 00564930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetDelete` (Xbox PDB): when `deleted` is set, the data
/// handler is not in its state `004226e0` and knows this form's ID
/// (`00469860`), the form does not have flag 0x4000 (`004077c0`) and the
/// loading object (`011ddf38`) does not report `0042ce10`, the loading
/// object is told about this reference (`0084a7c0`). Then `00484530(this,
/// deleted)` does the form-level work.
pub fn tes_object_refr_set_delete(e: &mut Engine, this: Ptr<TESObjectREFR>, deleted: u8) {
    let handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
    let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
    if deleted != 0 && !e.call(0x0042_26e0, &args![handler]).bool() {
        let id = e.call(FORM_ID, &args![this]).u32();
        if e.call(0x0046_9860, &args![handler, id]).bool()
            && !e.call(0x0040_77c0, &args![this]).bool()
            && !e.call(0x0042_ce10, &args![loading]).bool()
        {
            e.call(0x0084_a7c0, &args![loading, this]);
        }
    }
    e.call(0x0048_4530, &args![this, u32::from(deleted)]);
}

// Translated from 005649b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag 0x200 is set, for a reference whose base form is
/// absent or of type 0x1e; false for any other base form type. Not named in
/// the Xbox PDB.
pub fn fn_005649b0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if let Some(kind) = base_type(e, this.addr()) {
        if kind != 0x1e {
            return false;
        }
    }
    form_flag_set(e, this.addr(), 0x200)
}

// Translated from 00564a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears the form flag 0x200 (persistent) on a reference whose base
/// form is absent or of type 0x1e, and keeps the object that
/// `005725f0` returns in step: with a non-null result, clearing the flag
/// first calls `00b4f9a0`, then `00b5c450(handle, flag)` is called on the
/// object `00450b80(0)` returns, where the handle is the first dword of
/// `005725f0`'s result. The flags are stored last (`00403550`). Not named in
/// the Xbox PDB.
pub fn fn_00564a00(e: &mut Engine, this: Ptr<TESObjectREFR>, persist: u8) {
    let me = this.addr();
    if let Some(kind) = base_type(e, me) {
        if kind != 0x1e {
            return;
        }
    }
    let mut flags = e.call(FORM_GET_FLAGS, &args![me]).u32();
    if persist != 0 {
        flags |= 0x200;
    } else {
        flags &= !0x200;
    }
    let tracked = e.call(0x0057_25f0, &args![me]).u32();
    if tracked != 0 {
        if persist == 0 {
            e.call(0x00b4_f9a0, &args![]);
        }
        let handle = e.call(0x0055_9450, &args![tracked]).u32();
        let manager = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_c450, &args![manager, handle, u32::from(persist)]);
    }
    e.call(FORM_SET_FLAGS, &args![me, flags]);
}

// Translated from 00564ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag 0x200 is set, for a reference whose base form is
/// absent or of type 0x22; false for any other base form type. Not named in
/// the Xbox PDB.
pub fn fn_00564ab0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if let Some(kind) = base_type(e, this.addr()) {
        if kind != 0x22 {
            return false;
        }
    }
    form_flag_set(e, this.addr(), 0x200)
}

// Translated from 00564b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears the form flag 0x200 on a reference whose base form is
/// absent or of type 0x22; does nothing for any other base form type. Not
/// named in the Xbox PDB.
pub fn fn_00564b00(e: &mut Engine, this: Ptr<TESObjectREFR>, set: u8) {
    if let Some(kind) = base_type(e, this.addr()) {
        if kind != 0x22 {
            return;
        }
    }
    update_form_flags(e, this.addr(), 0x200, set != 0);
}

// Translated from 00564b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base form's virtual at `+0xa8` (no stack words), false without a base
/// form. Not named in the Xbox PDB.
pub fn fn_00564b70(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        return e.vcall(base, 0xa8, &args![]).bool();
    }
    false
}

// Translated from 00564bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base form's virtual at `+0xb0` (no stack words), false without a base
/// form. Not named in the Xbox PDB.
pub fn fn_00564bc0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        return e.vcall(base, 0xb0, &args![]).bool();
    }
    false
}

// Translated from 00564c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag 0x100 is set; false without a base form and for a
/// base form of type 0x1c. Not named in the Xbox PDB.
pub fn fn_00564c10(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    match base_type(e, this.addr()) {
        None | Some(0x1c) => false,
        Some(_) => form_flag_set(e, this.addr(), 0x100),
    }
}

// Translated from 00564c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears the form flag 0x100 on a reference that has a base form of
/// a type other than 0x1c. Not named in the Xbox PDB.
pub fn fn_00564c60(e: &mut Engine, this: Ptr<TESObjectREFR>, set: u8) {
    if let Some(kind) = base_type(e, this.addr()) {
        if kind != 0x1c {
            update_form_flags(e, this.addr(), 0x100, set != 0);
        }
    }
}

// Translated from 00564cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag 0x100000 is set; false for an actor (virtual
/// `+0x100`, `IsActor`). Not named in the Xbox PDB.
pub fn fn_00564cd0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if e.vcall(this.addr(), 0x100, &args![]).bool() {
        return false;
    }
    form_flag_set(e, this.addr(), 0x10_0000)
}

// Translated from 00564d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears the form flag 0x100000 on a reference that is not an actor
/// (virtual `+0x100`). Not named in the Xbox PDB.
pub fn fn_00564d20(e: &mut Engine, this: Ptr<TESObjectREFR>, set: u8) {
    if !e.vcall(this.addr(), 0x100, &args![]).bool() {
        update_form_flags(e, this.addr(), 0x10_0000, set != 0);
    }
}

// Translated from 00564d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag 0x40000 (targeted) is set. Not named in the Xbox
/// PDB.
pub fn fn_00564d80(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    form_flag_set(e, this.addr(), 0x4_0000)
}

// Translated from 00564db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetTargeted` (Xbox PDB): sets or clears the form flag
/// 0x40000.
pub fn tes_object_refr_set_targeted(e: &mut Engine, this: Ptr<TESObjectREFR>, targeted: u8) {
    update_form_flags(e, this.addr(), 0x4_0000, targeted != 0);
}

// Translated from 00565050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 is set in the `iFormID`-style dword (`0084e3a0`) of the
/// object `00527080(this, 0x92)` returns; false when that is null. Not named
/// in the Xbox PDB.
pub fn fn_00565050(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let object = e.call(0x0052_7080, &args![this, 0x92u32]).u32();
    object != 0 && e.call(FORM_ID, &args![object]).u32() & 4 != 0
}

// Translated from 00565260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::MustRefPersist` (Xbox PDB): whether the reference has to
/// stay persistent because of its base form. False without a base form. True
/// for the base forms with the form IDs 4, 5, 6, 0x10, 0x12, 0x15, 0x1f,
/// 0x23, 0x24, 0x34 and 0x3b, and by the base form's type: 4 and 0x23
/// always; 0x16 when `004fede0(base, 0)` holds; 0x1c when `005194d0(base)`
/// or `00568e50(this)` holds; 0x2a and 0x2b when `0055e200(base + 0x30)`
/// holds.
pub fn tes_object_refr_must_ref_persist(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    if base == 0 {
        return false;
    }
    let id = e.call(FORM_ID, &args![base]).u32();
    if matches!(
        id,
        4 | 5 | 6 | 0x10 | 0x12 | 0x15 | 0x1f | 0x23 | 0x24 | 0x34 | 0x3b
    ) {
        return true;
    }
    match e.call(FORM_TYPE, &args![base]).u32() {
        4 | 0x23 => true,
        0x16 => e.call(0x004f_ede0, &args![base, 0u32]).bool(),
        0x1c => {
            e.call(0x0051_94d0, &args![base]).bool() || e.call(0x0056_8e50, &args![me]).u32() != 0
        }
        0x2a | 0x2b => e.call(0x0055_e200, &args![base + 0x30]).bool(),
        _ => false,
    }
}

// Translated from 005653d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetRefPersists` (Xbox PDB): true when `00484e60(this,
/// -1)` finds an object and neither the data handler (`004516b0`,
/// `004226e0`), the save/load game object (`0047c850`) nor that object
/// (`00471c20`) objects; otherwise whether the form flag 0x400 is set
/// ([`fn_00565450`]).
pub fn tes_object_refr_get_ref_persists(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let found = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
    if found != 0 {
        let handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
        let save_load = e.global::<u32>(GLOBAL_SAVE_LOAD_GAME);
        if !e.call(0x0045_16b0, &args![handler]).bool()
            && !e.call(0x0047_c850, &args![save_load]).bool()
            && !e.call(0x0042_26e0, &args![handler]).bool()
            && !e.call(0x0047_1c20, &args![found]).bool()
        {
            return true;
        }
    }
    fn_00565450(e, this)
}

// Translated from 00565450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag 0x400 (persistent reference) is set. Not named in
/// the Xbox PDB.
pub fn fn_00565450(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    form_flag_set(e, this.addr(), 0x400)
}

// Translated from 00565480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetRefPersists` (Xbox PDB): sets or clears the form flag
/// 0x400. Setting it also calls the virtual `+0x48` with 1 and, when the
/// parent cell (the `TESChildCell` base's virtual 0) exists and neither
/// `00425fd0` nor `005516c0` holds for it, adds the reference to the cell's
/// persistent list (`00587ff0` on `0054ddd0(cell)`). Clearing it, when the
/// cell exists and `005516c0` holds, removes the reference from that list
/// (`00588030`) and calls the virtual `+0xc8` with 1.
pub fn tes_object_refr_set_ref_persists(e: &mut Engine, this: Ptr<TESObjectREFR>, persists: u8) {
    let me = this.addr();
    let cell = e.vcall(me + CHILD_CELL_OFFSET, 0, &args![]).u32();
    update_form_flags(e, me, 0x400, persists != 0);
    if persists != 0 {
        e.vcall(me, 0x48, &args![1u32]);
        if cell != 0
            && !e.call(0x0042_5fd0, &args![cell]).bool()
            && !e.call(0x0055_16c0, &args![cell]).bool()
        {
            let list = e.call(0x0054_ddd0, &args![cell]).u32();
            e.call(0x0058_7ff0, &args![list, me]);
        }
    } else if cell != 0 && e.call(0x0055_16c0, &args![cell]).bool() {
        let list = e.call(0x0054_ddd0, &args![cell]).u32();
        e.call(0x0058_8030, &args![list, me]);
        e.vcall(me, 0xc8, &args![1u32]);
    }
}

// Translated from 00565580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::HasTimeControllers` (Xbox PDB), cdecl on a 3D object:
/// whether `node` or any of its descendants has a controller (`0043b230`).
/// The node's own list (`00430830`, walked with `0057cbe0`) is searched
/// first, then its children (the virtual `+0xc`, `0043b480` counts and
/// `0043b4a0` fetches them) recursively.
pub fn tes_object_refr_has_time_controllers(e: &mut Engine, node: Ptr) -> bool {
    let node = node.addr();
    let mut found = false;
    if node == 0 {
        return false;
    }
    if e.call(GET_CONTROLLER, &args![node]).u32() != 0 {
        found = true;
    }
    if !found {
        let list = e.call(0x0043_0830, &args![node]).u32();
        let first = e.call(0x0055_9450, &args![list]).u32();
        e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), first);
            while e.mem.u32(position.addr()) != 0 && !found {
                let list = e.call(0x0043_0830, &args![node]).u32();
                let slot = e.call(0x0057_cbe0, &args![list, position]).u32();
                let item = e.call(0x0055_9450, &args![slot]).u32();
                if item != 0 {
                    found = e.call(GET_CONTROLLER, &args![item]).u32() != 0;
                }
            }
        });
    }
    if !found {
        let children = e.vcall(node, 0xc, &args![]).u32();
        if children != 0 {
            let count = e.call(0x0043_b480, &args![children]).u32();
            for index in 0..count {
                let child = e.call(0x0043_b4a0, &args![children, index]).u32();
                if tes_object_refr_has_time_controllers(e, Ptr::new(child)) {
                    return true;
                }
            }
        }
    }
    found
}

// Translated from 00565670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `key` from the map object at `011ca0e0` (`00405430`, a map
/// remove) and returns whether it was there. cdecl, one word. Not named in
/// the Xbox PDB.
pub fn fn_00565670(e: &mut Engine, key: u32) -> bool {
    e.call(0x0040_5430, &args![0x011c_a0e0u32, key]).bool()
}

// Translated from 00565690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base form's virtual at `+0x94` (no stack words), false without a base
/// form. Not named in the Xbox PDB.
pub fn fn_00565690(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        return e.vcall(base, 0x94, &args![]).bool();
    }
    false
}

// Translated from 005656d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `004826d0` accepts the base form, or when the reference has a
/// container (`0055d310`) whose inventory changes (`004bf220`, cdecl) report
/// `004d0490`. Not named in the Xbox PDB.
pub fn fn_005656d0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(0x0048_26d0, &args![base]).u32() != 0 {
        return true;
    }
    if e.call(0x0055_d310, &args![this]).u32() != 0 {
        let changes = e.call(0x004b_f220, &args![this]).u32();
        return e.call(0x004d_0490, &args![changes]).bool();
    }
    false
}

// Translated from 00565730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::InitScript` (Xbox PDB): for a reference that is not
/// deleted, inside an allocation scope guard (tag 0x15), and only when it has
/// no script extra data (type 0xd with a non-null +0xc), looks for the script
/// of the reference (virtual `+0x1a4`, else its base form), turns it into the
/// form `004826d0` returns and, when there is one, stores it and the
/// value `005abf60` derives from it in the reference's extra data
/// (`00419ed0`, `00419f80`). A reference with a container also has its
/// inventory changes (`004bf220`) initialised (`004d1960`).
pub fn tes_object_refr_init_script(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if e.call(FORM_IS_DELETED, &args![me]).bool() {
        return;
    }
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x15u32, 1u32, SOURCE_FILE, 0x196eu32],
        );
        let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
        let script_extra = e.call(0x0041_0220, &args![list, 0xdu32]).u32();
        if script_extra != 0 && e.mem.u32(script_extra + 0xc) != 0 {
            e.call(SCOPE_GUARD_DTOR, &args![guard]);
            return;
        }
        let mut script = e.vcall(me, 0x1a4, &args![]).u32();
        if script == 0 {
            script = e.call(GET_BASE_FORM, &args![me]).u32();
        }
        let form = e.call(0x0048_26d0, &args![script]).u32();
        if form != 0 {
            let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
            e.call(0x0041_9ed0, &args![list, form]);
            let derived = e.call(0x005a_bf60, &args![form]).u32();
            let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
            e.call(0x0041_9f80, &args![list, derived]);
        }
        if e.call(0x0055_d310, &args![me]).u32() != 0 {
            let changes = e.call(0x004b_f220, &args![me]).u32();
            e.call(0x004d_1960, &args![changes]);
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 00565870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RunScript` (Xbox PDB): runs the scripts of the reference
/// and returns whether any ran. Nothing happens for a deleted reference or
/// one without a base form. For the base form types 0x2a and 0x2b a
/// reference whose process (`008d8520`) exists, is not `00931850` and has no
/// loaded 3D is skipped entirely (false). A reference with a container runs
/// the scripts of its inventory changes (`004d2480`). Then the reference's
/// own script (`00418800` on the extra list) runs with its variables
/// (`00418830`) through `005ac1e0`; when it did not run and there are
/// variables, the virtual `+0x48` is called with `0x80000000`. A "say to"
/// extra (`0042ede0`) on a reference that is not an actor-like (virtual
/// `+0xfc`) calls the virtual `+0x140`.
pub fn tes_object_refr_run_script(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let mut ran = false;
    if e.call(FORM_IS_DELETED, &args![me]).bool() || e.call(GET_BASE_FORM, &args![me]).u32() == 0 {
        return ran;
    }
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    let kind = e.call(FORM_TYPE, &args![base]).u32();
    let actor_like = (0x2a..=0x2b).contains(&kind);
    if actor_like
        && e.call(0x008d_8520, &args![me]).u32() != 0
        && e.call(0x0093_1850, &args![me]).u32() == 0
        && e.vcall(me, 0x1d0, &args![]).u32() == 0
    {
        return false;
    }
    if (kind == 0x1b || actor_like) && e.call(0x0055_d310, &args![me]).u32() != 0 {
        let changes = e.call(0x004b_f220, &args![me]).u32();
        if changes != 0 && e.call(0x004d_2480, &args![changes, me]).bool() {
            ran = true;
        }
    }
    let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
    let script = e.call(0x0041_8800, &args![list]).u32();
    if script != 0 {
        let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
        let variables = e.call(0x0041_8830, &args![list]).u32();
        if e.call(0x005a_c1e0, &args![script, me, variables, 0u32, 0u32])
            .bool()
        {
            ran = true;
        } else if variables != 0 {
            e.vcall(me, 0x48, &args![0x8000_0000u32]);
        }
        let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
        let say_to = e.call(0x0042_ede0, &args![list]).u32();
        if say_to != 0 && !e.vcall(me, 0xfc, &args![]).bool() {
            e.vcall(me, 0x140, &args![]);
        }
    }
    ran
}

// ---------------------------------------------------------------------------
// InitAnimation and what it uses.

/// `TESObjectREFR::SetAnimation` (Xbox PDB, `00572e50(this, animation)`):
/// installs the animation object (0 clears it) and returns the current one.
const SET_ANIMATION: u32 = 0x0057_2e50;
/// `Animation::Update` (`00491180(animation, reference, delta, -1.0f)`).
const ANIMATION_UPDATE: u32 = 0x0049_1180;
/// The `Animation` constructor (`0048f810`, object of 0x13c bytes).
const ANIMATION_CONSTRUCT: u32 = 0x0048_f810;
const ANIMATION_SIZE: u32 = 0x13c;
/// The actor's process object (`008d8520`; the map names it
/// `MiddleHighProcess::GetSavedAcquireObject`, which does not describe it)
/// and its level (`0045cd60`): 0 or 1 for the processes the animation
/// calls below need.
const ACTOR_PROCESS: u32 = 0x008d_8520;
const PROCESS_LEVEL: u32 = 0x0045_cd60;
/// `Actor::IsWeaponDrawn` as `008a16d0` (no stack words), the actor's
/// animation getter `008b70d0` (no stack words) and the player's
/// `GetBiped(flag)` (`00950b00`) and `GetAnimation(flag)` (`00950a60`).
const ACTOR_WEAPON_DRAWN: u32 = 0x008a_16d0;
const ACTOR_ANIMATION: u32 = 0x008b_70d0;
const PLAYER_BIPED: u32 = 0x0095_0b00;
const PLAYER_ANIMATION: u32 = 0x0095_0a60;
/// `MiddleHighProcess::GetForceNextUpdate` (`00566950`), a byte test on the
/// actor's process data.
const FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// The strings the animation set-up reports with.
const SKELETON_PREFIX: u32 = 0x0101_3458;
const SKELETON_MESSAGE: u32 = 0x0102_ff48;
const CUMULATIVE_MESSAGE: u32 = 0x0102_fee8;
const NOT_LOOPING_MESSAGE: u32 = 0x0102_feb4;
const MOVED_MESSAGE: u32 = 0x0102_fe74;
const NPC_ANIMATION_MESSAGE: u32 = 0x0102_fdd0;
const CREATURE_ANIMATION_MESSAGE: u32 = 0x0102_fe20;
const LILY_NAME: u32 = 0x0102_fdc8;
/// The table of type names (3 dwords per form type, the name pointer first)
/// at `01187004`.
const TYPE_NAME_TABLE: u32 = 0x0118_7004;
/// The `NiTimeController` record `00a5c570` looks for on the 3D object
/// (`011f36bc`).
const CONTROLLER_RECORD: u32 = 0x011f_36bc;

// Translated from 005659f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::InitAnimation` (Xbox PDB): builds the animation of a
/// reference from its loaded 3D. The C++ exception state and the stack
/// cookie are not translated. Outline:
///
/// - Clears the current animation (`SetAnimation(this, 0)`) and stops for a
///   reference without a 3D node (virtual `+0xc` of the loaded 3D) or a
///   deleted one.
/// - Base form types 0x2a and 0x2b: warns when the model path's file name
///   does not start with "Skeleton", and asks `008a1760` for the actor's
///   skeleton.
/// - Type 0x2a (with a biped, virtual `+0x1e8`) and 0x2b build a new
///   `Animation`, its KF file list ([`tes_object_refr_build_kf_file_list`])
///   and special animations ([`init_character_animation`]); type 0x28 (a
///   light, `004c0bf0`) sets its light. Every other type (and 0x28) sets
///   up the node's controller manager ([`init_controller_manager`]).
/// - Updates the new animation once (`Animation::Update(this, 0, -1.0)`),
///   then for actors ([`init_actor_state`]) refreshes the process's
///   weapon and animation state.
/// - Finally restores a saved animation time (the extra data `00418200`)
///   into the 3D and removes that extra data.
pub fn tes_object_refr_init_animation(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let data = this.at(TESObjectREFR::data);
    let loaded = e.call(GET_LOADED_3D, &args![this]).u32();
    let mut node = if loaded == 0 {
        0
    } else {
        e.vcall(loaded, 0xc, &args![]).u32()
    };
    let biped = e.vcall(me, 0x1e8, &args![]).u32();
    let mut animation = 0u32;
    e.call(SET_ANIMATION, &args![this, animation]);
    if node == 0 || e.call(FORM_IS_DELETED, &args![this]).bool() {
        return;
    }
    let base = e.get(data, OBJ_REFR::pObjectReference).addr();
    let actor = if e.vcall(me, 0x100, &args![]).bool() {
        me
    } else {
        0
    };
    let mut skeleton = 0u32;
    if e.call(FORM_TYPE, &args![base]).u32() == 0x2a
        || e.call(FORM_TYPE, &args![base]).u32() == 0x2b
    {
        let model = e.call(GET_MODEL, &args![this]).u32();
        let slash = e.call(STRRCHR, &args![model, 0x5cu32]).u32();
        if slash != 0
            && e.call(STRNICMP, &args![slash + 1, SKELETON_PREFIX, 8u32])
                .i32()
                != 0
        {
            let name = e.vcall(base, 0x130, &args![]).u32();
            e.call(LOG_MESSAGE, &args![SKELETON_MESSAGE, name, model]);
        }
        skeleton = e.call(0x008a_1760, &args![actor]).u32();
    }
    let kind = e.call(FORM_TYPE, &args![base]).u32();
    match kind {
        0x2a => {
            if biped != 0 {
                match init_character_animation(e, me, base, skeleton, biped, false) {
                    Some((created, created_node)) => {
                        animation = created;
                        node = created_node;
                    }
                    None => return,
                }
            }
        }
        0x2b => match init_character_animation(e, me, base, skeleton, 0, true) {
            Some((created, created_node)) => {
                animation = created;
                node = created_node;
            }
            None => return,
        },
        _ => {
            if kind == 0x28 && e.call(0x004c_0bf0, &args![base]).bool() {
                e.call(0x0057_26e0, &args![this, node, 0u32]);
            }
            init_controller_manager(e, this, base, node);
        }
    }
    if animation != 0 {
        let minus_one: f32 = e.global(MINUS_ONE_FLOAT);
        e.call(ANIMATION_UPDATE, &args![animation, me, 0.0f32, minus_one]);
    }
    if actor != 0 {
        init_actor_state(e, me, actor, biped, animation, node);
    }
    let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
    let saved = e.call(0x0041_8200, &args![list]).u32();
    if saved != 0 && animation != 0 {
        // the saved animation time, a `float` at +0x10 of the extra data
        let time = e.mem.f32(saved + 0x10);
        e.with_stack(0xc, |e, record| {
            e.call(MAKE_VELOCITY, &args![record, time, 0u32, 0u32]);
            let current = e.call(GET_LOADED_3D, &args![me]).u32();
            e.call(SET_3D_VELOCITY, &args![current, record]);
        });
        let list = e.call(GET_EXTRA_LIST, &args![me]).u32();
        e.call(0x0041_ae10, &args![list]);
    }
}

/// The part of `InitAnimation` for base form types 0x2a (`creature` false;
/// `biped` is the actor's biped, whose parts are removed first,
/// `004aae70`) and 0x2b (`creature` true): installs a new `Animation`
/// (allocated with `operator new` of 0x13c bytes and constructed with
/// `0048f810`), re-reads the 3D node (stopping with the "SetAnimation
/// cleared 3D" message when it is gone), builds the KF file list and has
/// the animation load it (`0048ffd0(list, node, this, 1)`, reporting the
/// missing 'Idle' animation when it fails). When the base form's value
/// `00568ad0` is positive and its animation set (embedded at +0xc4) has KF
/// files (`0047fd90`), the model's directory is cut off the path and the
/// special animations are added (`00490330`). A creature's base form is
/// finally given the 3D (`005fbdd0`). Returns the new animation and the new
/// node, `None` when the 3D is gone.
fn init_character_animation(
    e: &mut Engine,
    me: u32,
    base: u32,
    skeleton: u32,
    biped: u32,
    creature: bool,
) -> Option<(u32, u32)> {
    if !creature {
        e.call(0x004a_ae70, &args![biped]);
    }
    let block = e.call(OPERATOR_NEW, &args![ANIMATION_SIZE]).u32();
    let object = if block != 0 {
        e.call(ANIMATION_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let animation = e.call(SET_ANIMATION, &args![me, object]).u32();
    let loaded = e.call(GET_LOADED_3D, &args![me]).u32();
    let node = if loaded == 0 {
        0
    } else {
        e.vcall(loaded, 0xc, &args![]).u32()
    };
    if node == 0 {
        let id = e.call(FORM_ID, &args![base]).u32();
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(LOG_MESSAGE, &args![MOVED_MESSAGE, name, id]);
        return None;
    }
    e.call(0x0057_10c0, &args![me]);
    let list = tes_object_refr_build_kf_file_list(e, Ptr::new(me), skeleton);
    if !e
        .call(0x0048_ffd0, &args![animation, list, node, me, 1u32])
        .bool()
    {
        let id = e.call(FORM_ID, &args![base]).u32();
        let name = e.vcall(base, 0x130, &args![]).u32();
        let message = if creature {
            CREATURE_ANIMATION_MESSAGE
        } else {
            NPC_ANIMATION_MESSAGE
        };
        e.call(LOG_MESSAGE, &args![message, name, id]);
    }
    let value = e.call(0x0056_8ad0, &args![me]).f64();
    if value > 0.0 && e.call(0x0047_fd90, &args![base + 0xc4]).bool() {
        let model = e.call(GET_MODEL, &args![me]).u32();
        e.with_stack(0x104, |e, path| {
            e.call(BOUNDED_COPY, &args![path, 0x104u32, model]);
            let slash = e.call(STRRCHR_WRAPPER, &args![path, 0x5cu32]).u32();
            if slash != 0 {
                e.mem.set_u8(slash, 0);
                let package = e.call(0x0071_7e50, &args![base + 0xc4]).u32();
                e.call(0x0049_0330, &args![animation, package, path]);
            }
        });
    }
    if creature {
        e.call(0x005f_bdd0, &args![base, loaded]);
    }
    Some((animation, node))
}

/// Gives the node an empty velocity record through its virtual `+0xa4`
/// (`(record, 0)`), as `InitAnimation` does when the node has no
/// controller manager.
fn clear_node_motion(e: &mut Engine, node: u32) {
    e.with_stack(0xc, |e, record| {
        e.call(MAKE_VELOCITY, &args![record, 0.0f32, 0u32, 0u32]);
        e.vcall(node, 0xa4, &args![record, 0u32]);
    });
}

/// The part of `InitAnimation` for every base form type but 0x2a and 0x2b:
/// finds the node's controller manager (the first controller cast to the
/// manager type) and, without one, clears the node's motion. With one: the
/// form is marked changed (`TESForm::ForceChange(0x10000000)`), a cumulative
/// manager (`00566910`) is reported, an open/close form has its open state
/// restored (`0047aec0`: the state is the default state, or, unless the
/// loading object is busy, the default state changed by the saved change
/// 0x800000), and the idle sequences (the two names kept in the exe's data)
/// are started: with none, and the open state not set, the first sequence
/// is started from its offset time; with them, an idle that is not looping
/// (`0059bb30`) is reported and the others started. The manager is then
/// switched off again when no idle was found.
fn init_controller_manager(e: &mut Engine, this: Ptr<TESObjectREFR>, base: u32, node: u32) {
    let me = this.addr();
    if e.call(GET_CONTROLLER, &args![node]).u32() == 0 {
        clear_node_motion(e, node);
        return;
    }
    let controller = e.call(GET_CONTROLLER, &args![node]).u32();
    let manager = e.call(CHECKED_CAST, &args![MANAGER_TYPE, controller]).u32();
    if manager == 0 {
        clear_node_motion(e, node);
        return;
    }
    e.call(0x0048_4b90, &args![this, 0x1000_0000u32]);
    if fn_00566910(e, Ptr::new(manager)) != 0 {
        let model = e.call(GET_MODEL, &args![this]).u32();
        let kind = e.call(FORM_TYPE, &args![base]).u32();
        let type_name = e.global::<u32>(TYPE_NAME_TABLE.wrapping_add(kind.wrapping_mul(12)));
        e.call(LOG_MESSAGE, &args![CUMULATIVE_MESSAGE, type_name, model]);
    }
    let mut open_state_set = false;
    let base_form = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(0x0047_a490, &args![base_form]).bool() {
        let mut open = e.call(0x0057_2d30, &args![this, 8u32]).u8();
        let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
        if !e.call(0x0042_ce10, &args![loading]).bool() {
            let default_open = e.call(0x0056_1d90, &args![this]).u8();
            let changed = e.with_stack(4, |e, flags| {
                e.call(0x008c_71b0, &args![flags, 0x80_0000u32]);
                let flags = e.mem.u32(flags.addr());
                e.call(0x0084_a6d0, &args![loading, me, flags]).bool()
            });
            open = u8::from(u32::from(default_open) != u32::from(changed));
        }
        open_state_set = e.call(0x0047_aec0, &args![me, u32::from(open), 1u32]).u8() != 0;
    }
    let idle_a = find_sequence(e, manager, NAME_IDLE_A);
    let idle_b = find_sequence(e, manager, NAME_IDLE_B);
    let most: f32 = e.global(FLOAT_MAX);
    if idle_a == 0 && idle_b == 0 {
        if !open_state_set {
            e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
            e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
            let first = e.call(MANAGER_SEQUENCE_AT, &args![manager, 0u32]).u32();
            activate_sequence(e, manager, first);
            e.call(SEQUENCE_SET_OFFSET, &args![first, -most]);
            let time = e.call(SEQUENCE_OFFSET_TIME, &args![first]).f32();
            set_velocity(e, node, time, 1, 0);
        }
        e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
        e.call(MANAGER_SET_FLAG, &args![manager, 0u32]);
    } else {
        let mut time = 0.0f32;
        if idle_a != 0 {
            if e.call(0x0059_bb30, &args![idle_a]).u32() == 0 {
                activate_sequence(e, manager, idle_a);
                e.call(SEQUENCE_SET_OFFSET, &args![idle_a, -most]);
                time = e.call(SEQUENCE_OFFSET_TIME, &args![idle_a]).f32();
            } else {
                let model = e.call(GET_MODEL, &args![this]).u32();
                e.call(LOG_MESSAGE, &args![NOT_LOOPING_MESSAGE, model]);
            }
        }
        if idle_b != 0 {
            activate_sequence(e, manager, idle_b);
            e.call(SEQUENCE_SET_OFFSET, &args![idle_b, -most]);
            time = e.call(SEQUENCE_OFFSET_TIME, &args![idle_b]).f32();
        }
        set_velocity(e, node, time, 1, 0);
    }
}

/// The last part of `InitAnimation` for an actor: refreshes what the actor's
/// process knows about its weapon and animation. `actor` is the reference
/// itself (it is an actor here), `biped` its biped (virtual `+0x1e8`),
/// `animation` the new animation (0 when there is none) and `node` its 3D
/// node. The process (`008d8520`) counts when its level (`0045cd60`) is zero or
/// one. A process of an actor that is not the player, while the loading
/// object is not busy, is given the worn item of slot 5 and the actor's 3D
/// (virtual `+0x160`), has the worn weapon attached (`0087b360`, or
/// `00571760` for the actor itself, unless the actor is named "Lily"), has its
/// animation state (`00496940`, virtual `+0x18c`) and its drawn-weapon state
/// (virtual `+0x1cc`, with the biped, the animation and the actor) refreshed.
/// The 3D's controller of type `011f36bc`, if any, is switched on
/// ([`fn_00566930`]).
fn init_actor_state(e: &mut Engine, me: u32, actor: u32, biped: u32, animation: u32, node: u32) {
    let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
    let player = e.global::<u32>(GLOBAL_PLAYER);
    let mut acquire = 0u32;
    if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        let level = e.call(PROCESS_LEVEL, &args![process]).i32();
        if (0..=1).contains(&level) {
            acquire = e.call(ACTOR_PROCESS, &args![actor]).u32();
        }
    }
    if acquire != 0 && me != player && !e.call(0x0042_ce10, &args![loading]).bool() {
        let changes = e.call(0x004b_f220, &args![me]).u32();
        let worn = if changes != 0 {
            e.call(0x004c_8c10, &args![changes, 5u32, 0u32]).u32()
        } else {
            0
        };
        let worn_flags = if worn != 0 {
            e.call(FORM_GET_FLAGS, &args![worn]).u32()
        } else {
            0
        };
        let worn_again = if changes != 0 {
            e.call(0x004c_8c10, &args![changes, 5u32, 0u32]).u32()
        } else {
            0
        };
        let node_3d = e.vcall(me, 0x1d0, &args![]).u32();
        e.vcall(acquire, 0x160, &args![worn_again, node_3d, 0u32]);
        let actor_again = if e.vcall(me, 0x100, &args![]).bool() {
            me
        } else {
            0
        };
        let mut lily = false;
        if e.vcall(me, 0x100, &args![]).bool()
            && e.call(FORCE_NEXT_UPDATE, &args![actor_again]).bool()
        {
            let text = e.call(0x0055_d520, &args![actor_again]).u32();
            lily = e.call(STRSTR, &args![text, LILY_NAME]).u32() != 0;
        }
        if e.vcall(me, 0x100, &args![]).bool()
            && (e.vcall(me, 0x218, &args![]).bool() || lily)
            && worn_flags != 0
            && !e.vcall(acquire, 0x474, &args![1u32]).bool()
            && worn != 0
        {
            if e.call(0x008c_7aa0, &args![]).bool() && me != e.call(0x0047_b200, &args![]).u32() {
                let flags = e.call(FORM_GET_FLAGS, &args![worn]).u32();
                let manager = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_b360, &args![manager, me, flags]);
            } else if !lily {
                let flags = e.call(FORM_GET_FLAGS, &args![worn]).u32();
                e.call(0x0057_1760, &args![me, flags]);
            }
        }
        if animation != 0 && e.call(0x0055_85e0, &args![animation]).u32() != 0 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let first = e.call(0x0055_85e0, &args![animation]).u32();
            let second = e.call(0x0049_6940, &args![animation]).u32();
            e.vcall(process, 0x18c, &args![second, first]);
        }
        if !e.call(0x0042_ce10, &args![loading]).bool() {
            e.vcall(acquire, 0x22c, &args![]);
            if e.call(0x008a_6970, &args![actor]).bool() {
                e.vcall(acquire, 0x458, &args![actor, 1u32]);
            }
            if me == player && biped == e.call(PLAYER_BIPED, &args![player, 1u32]).u32() {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                let animation_of = e.call(PLAYER_ANIMATION, &args![player, 1u32]).u32();
                let drawn = e.call(ACTOR_WEAPON_DRAWN, &args![actor]).u8();
                e.vcall(
                    process,
                    0x1cc,
                    &args![u32::from(drawn), biped, animation_of, actor],
                );
            } else {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                let animation_of = e.call(ACTOR_ANIMATION, &args![actor]).u32();
                let drawn = e.call(ACTOR_WEAPON_DRAWN, &args![actor]).u8();
                e.vcall(
                    process,
                    0x1cc,
                    &args![u32::from(drawn), biped, animation_of, actor],
                );
            }
        }
    }
    if e.vcall(me, 0x21c, &args![]).bool()
        && e.call(FORCE_NEXT_UPDATE, &args![actor]).bool()
        && e.call(0x0042_ce10, &args![loading]).bool()
    {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        let animation_of = e.call(ACTOR_ANIMATION, &args![actor]).u32();
        let drawn = e.call(ACTOR_WEAPON_DRAWN, &args![actor]).u8();
        e.vcall(
            process,
            0x1cc,
            &args![u32::from(drawn), biped, animation_of, actor],
        );
    }
    let controller = e.call(0x00a5_c570, &args![node, CONTROLLER_RECORD]).u32();
    if controller != 0 {
        fn_00566930(e, Ptr::new(controller), 1);
    }
}

// Translated from 00566910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x68` of its object (called on a controller manager by
/// `InitAnimation`; the object is larger than a reference). Not named in
/// the Xbox PDB.
pub fn fn_00566910(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x68)
}

// Translated from 00566930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0047aa60(this, flag, 0x40)`: the flag-setter of the object
/// `InitAnimation` calls it on (the controller of type `011f36bc`). Not
/// named in the Xbox PDB.
pub fn fn_00566930(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x0047_aa60, &args![this, u32::from(flag), 0x40u32]);
}

// ---------------------------------------------------------------------------
// BuildKFFileList and what it uses.

/// The model loader object (`011c3b3c`) and its methods: `BuildKFFileList`
/// (`00447330(model, 1, 1, selector)`), `BuildFileList` (`00447300(path,
/// model, list)`) and `CopyFilenameList` (`00447850(list, into)`); the idle
/// manager (`011cb6a0`) and its `GetRootFilenameList` (`00600700(path,
/// 0)`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const BUILD_KF_FILE_LIST: u32 = 0x0044_7330;
const BUILD_FILE_LIST: u32 = 0x0044_7300;
const COPY_FILENAME_LIST: u32 = 0x0044_7850;
const IDLE_MANAGER: u32 = 0x011c_b6a0;
const ROOT_FILENAME_LIST: u32 = 0x0060_0700;
/// `Actor::GetCurrentWeapon` (`008a1710`), the weapon's index (`00446390`)
/// and the two tables it indexes: `0118a838` (dwords) and `011977a4` (C
/// string pointers, the file name prefix of the holster animation).
const ACTOR_CURRENT_WEAPON: u32 = 0x008a_1710;
const WEAPON_INDEX: u32 = 0x0044_6390;
const WEAPON_INDEX_TABLE: u32 = 0x0118_a838;
const HOLSTER_PREFIX_TABLE: u32 = 0x0119_77a4;
/// The strings the file lists are built from.
const PATH_DATA: u32 = 0x0101_6fb4; // "Data\"
const PATH_MESHES: u32 = 0x0101_dccc; // "Meshes"
const PATH_BACKSLASH: u32 = 0x0101_3444; // "\"
const PATH_MT_IDLE: u32 = 0x0101_6fa0; // "\MTIdle.KF"
const PATH_HOLSTER: u32 = 0x0101_6f90; // "\%sHolster.KF"
const PATH_POWER_ARMOR_HOLSTER: u32 = 0x0101_6f80; // "\PA%sHolster.KF"
const PATH_DEATH: u32 = 0x0101_6f74; // "\Death.KF"
const PATH_CHILD_IDLES: u32 = 0x0101_6f58; // "\Locomotion\Child\IdleAnims"
const PATH_FEMALE_IDLES: u32 = 0x0101_6f38; // "\Locomotion\Female\IdleAnims"
const PATH_MALE_IDLES: u32 = 0x0101_6f1c; // "\Locomotion\Male\IdleAnims"
const PATH_FULL_MALE_LOCOMOTION: u32 = 0x0102_ffe4; // "Data\Meshes\Characters\_Male\Locomotion\MTIdle.kf"
const PATH_MALE_LOCOMOTION: u32 = 0x0102_ffbc; // "Characters\_Male\Locomotion\MTIdle.kf"
const PATH_FULL_MALE_IDLE: u32 = 0x0102_ff94; // "Data\Meshes\Characters\_Male\MTIdle.kf"

/// `fn_00567050`'s globals: the object at `011f2250` (the map calls its
/// methods `VATS::...`; `0044ddc0` on it gives the mode, `009c8d60` the
/// target update multiplier), the frame time object at `011f6394` (`0084d030`
/// returns its `float` at +0xc) and the pointer at `011f21cc` the VATS mode
/// is compared against.
const VATS_OBJECT: u32 = 0x011f_2250;
const TARGET_UPDATE_MULT: u32 = 0x009c_8d60;
const FRAME_TIME_OBJECT: u32 = 0x011f_6394;
const FRAME_TIME: u32 = 0x0084_d030;
const REFERENCE_COMPARED_WITH_VATS: u32 = 0x011f_21cc;

/// What follows the idle list in both character paths: the holster and
/// power armor holster animations of the actor's current weapon (the
/// prefix from the two tables, when it is not empty) and the death
/// animation. `slash` is the last backslash of `path` (the file name is
/// rewritten from there); every list is added to `list`.
fn add_holster_and_death_files(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    path: u32,
    slash: u32,
    list: u32,
) {
    let me = this.addr();
    let loader = e.global::<u32>(MODEL_LOADER);
    let weapon = e.call(ACTOR_CURRENT_WEAPON, &args![this]).u32();
    let index = if weapon == 0 {
        0
    } else {
        e.call(WEAPON_INDEX, &args![weapon]).u32()
    };
    let slot = e.global::<u32>(WEAPON_INDEX_TABLE.wrapping_add(index.wrapping_mul(4)));
    let prefix = e.global::<u32>(HOLSTER_PREFIX_TABLE.wrapping_add(slot.wrapping_mul(4)));
    if prefix != 0 && e.mem.u8(prefix) != 0 {
        e.call(SPRINTF, &args![slash, PATH_HOLSTER, prefix]);
        let model = e.call(GET_MODEL, &args![me]).u32();
        e.call(BUILD_FILE_LIST, &args![loader, path, model, list]);
        e.call(SPRINTF, &args![slash, PATH_POWER_ARMOR_HOLSTER, prefix]);
        let model = e.call(GET_MODEL, &args![me]).u32();
        e.call(BUILD_FILE_LIST, &args![loader, path, model, list]);
    }
    let room = 0x104u32.wrapping_sub(slash.wrapping_sub(path));
    e.call(BOUNDED_COPY, &args![slash, room, PATH_DEATH]);
    let model = e.call(GET_MODEL, &args![me]).u32();
    e.call(BUILD_FILE_LIST, &args![loader, path, model, list]);
}

// Translated from 00566970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::BuildKFFileList` (Xbox PDB): the list of KF animation
/// files to load for the reference, 0 when its base form has none of the
/// types below. The C++ exception state and the stack cookie are not
/// translated. `selector` is the word the model loader's list builder takes
/// (the callers pass the actor's skeleton).
///
/// - Base form type 0x2b: `selector` becomes the weapon table entry of the
///   actor's weapon (virtual `+0x148` of its process, then `0044ddc0` and
///   `00446390`), or 1, when the process level (`0045cd60`) is 0 or 1 and the
///   reference does not report the virtual `+0x22c(0)`. Without the virtual
///   `+0x22c(1)` the loader builds the list from the model (`00447330`).
///   With it, the list is built from `Data\Meshes\<model>` with
///   `\MTIdle.KF`, followed by the holster and death files
///   ([`add_holster_and_death_files`]).
/// - Base form type 0x2a: with form ID 7, or without the virtual
///   `+0x22c(1)`, the loader builds the list from the base form's model
///   (`+0xdc`, virtual `+0x14`) and the idle lists of the child (virtual
///   `+0x1a0(1)`), female (`005f0cc0` is 1) or male actors are copied into
///   it. Otherwise the male locomotion list is built and the holster and
///   death files are added.
pub fn tes_object_refr_build_kf_file_list(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    selector: u32,
) -> u32 {
    let me = this.addr();
    let mut selector = selector;
    let mut list = 0u32;
    if e.call(GET_BASE_FORM, &args![this]).u32() == 0 {
        return list;
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    let kind = e.call(FORM_TYPE, &args![base]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    if kind == 0x2b {
        if e.call(ACTOR_PROCESS, &args![me]).u32() != 0 && !e.vcall(me, 0x22c, &args![0u32]).bool()
        {
            let process = e.call(ACTOR_PROCESS, &args![me]).u32();
            let level = e.call(PROCESS_LEVEL, &args![process]).i32();
            if (0..=1).contains(&level) {
                let process = e.call(ACTOR_PROCESS, &args![me]).u32();
                if e.vcall(process, 0x148, &args![]).u32() != 0 {
                    let weapon = e.vcall(process, 0x148, &args![]).u32();
                    let flags = e.call(FORM_GET_FLAGS, &args![weapon]).u32();
                    let index = e.call(WEAPON_INDEX, &args![flags]).u32();
                    selector = e.global(WEAPON_INDEX_TABLE.wrapping_add(index.wrapping_mul(4)));
                } else if e.call(0x008a_6970, &args![me]).bool() {
                    selector = 1;
                }
            }
        }
        if !e.vcall(me, 0x22c, &args![1u32]).bool() {
            let model = e.call(GET_MODEL, &args![this]).u32();
            list = e
                .call(
                    BUILD_KF_FILE_LIST,
                    &args![loader, model, 1u32, 1u32, selector],
                )
                .u32();
        } else {
            e.with_stack(0x104, |e, path| {
                let path = path.addr();
                e.call(BOUNDED_COPY, &args![path, 0x104u32, PATH_DATA]);
                e.call(BOUNDED_APPEND, &args![path, 0x104u32, PATH_MESHES]);
                e.call(BOUNDED_APPEND, &args![path, 0x104u32, PATH_BACKSLASH]);
                let model = e.call(GET_MODEL, &args![this]).u32();
                e.call(BOUNDED_APPEND, &args![path, 0x104u32, model]);
                let slash = e.call(STRRCHR_WRAPPER, &args![path, 0x5cu32]).u32();
                let room = 0x104u32.wrapping_sub(slash.wrapping_sub(path));
                e.call(BOUNDED_COPY, &args![slash, room, PATH_MT_IDLE]);
                let model = e.call(GET_MODEL, &args![this]).u32();
                list = e
                    .call(BUILD_FILE_LIST, &args![loader, path, model, 0u32])
                    .u32();
                add_holster_and_death_files(e, this, path, slash, list);
            });
        }
    } else if kind == 0x2a {
        if e.call(FORM_ID, &args![base]).u32() == 7 || !e.vcall(me, 0x22c, &args![1u32]).bool() {
            let model = e.vcall(base + 0xdc, 0x14, &args![]).u32();
            list = e
                .call(
                    BUILD_KF_FILE_LIST,
                    &args![loader, model, 1u32, 1u32, selector],
                )
                .u32();
            let model = e.vcall(base + 0xdc, 0x14, &args![]).u32();
            e.with_stack(0x104, |e, path| {
                let path = path.addr();
                e.call(BOUNDED_COPY, &args![path, 0x104u32, model]);
                let slash = e.call(STRRCHR_WRAPPER, &args![path, 0x5cu32]).u32();
                let room = 0x104u32
                    .wrapping_sub(slash.wrapping_sub(path))
                    .wrapping_sub(1);
                let idles = if e.vcall(me, 0x1a0, &args![1u32]).bool() {
                    PATH_CHILD_IDLES
                } else if e.call(0x005f_0cc0, &args![base]).i32() == 1 {
                    PATH_FEMALE_IDLES
                } else {
                    PATH_MALE_IDLES
                };
                e.call(BOUNDED_COPY, &args![slash, room, idles]);
                let idle_manager = e.global::<u32>(IDLE_MANAGER);
                let idle_list = e
                    .call(ROOT_FILENAME_LIST, &args![idle_manager, path, 0u32])
                    .u32();
                e.call(COPY_FILENAME_LIST, &args![loader, idle_list, list]);
            });
        } else {
            e.with_stack(0x104, |e, full| {
                e.with_stack(0x104, |e, short| {
                    let full = full.addr();
                    e.call(
                        BOUNDED_COPY,
                        &args![full, 0x104u32, PATH_FULL_MALE_LOCOMOTION],
                    );
                    e.call(BOUNDED_COPY, &args![short, 0x104u32, PATH_MALE_LOCOMOTION]);
                    list = e
                        .call(BUILD_FILE_LIST, &args![loader, full, short, 0u32])
                        .u32();
                    e.call(BOUNDED_COPY, &args![full, 0x104u32, PATH_FULL_MALE_IDLE]);
                    let slash = e.call(STRRCHR_WRAPPER, &args![full, 0x5cu32]).u32();
                    add_holster_and_death_files(e, this, full, slash, list);
                });
            });
        }
    }
    list
}

// Translated from 00567050 (decompiled, FalloutNV.exe 1.4.0.525)
/// A per-frame step of a reference's animation (the Xbox PDB has no name for
/// it): nothing for a deleted or disabled reference. For a base form of type
/// 0x1e the virtual `+0x178` is called. Unless the base form is of type 0x2a
/// or 0x2b, the sequences of the 3D's controller manager (virtual `+0x88` of
/// its first controller) that `008041a0` accepts are asked to play their
/// sounds (`004eef00`); one that is finished is ended
/// ([`tes_object_refr_end_sequence`]), finished in the manager
/// (`0047b220`), recorded as the last finished sequence (`00578a30` with
/// its name) and announced (`0047ac70`); when none is left unfinished the
/// manager is switched off. For types 0x2a and 0x2b the animation
/// (virtual `+0x1e4`) is updated by the frame time (`011f6394`, scaled by
/// the VATS target mult when VATS is in mode 4 for the player's own
/// reference) if the 3D can be updated (`009611e0`), or [`tes_object_refr_init_animation`]
/// is run when there is no animation yet but a 3D.
pub fn fn_00567050(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let data = this.at(TESObjectREFR::data);
    let disabled = e.call(FORM_IS_DISABLED, &args![this]).bool();
    if e.call(FORM_IS_DELETED, &args![this]).bool() || disabled {
        return;
    }
    let base = e.get(data, OBJ_REFR::pObjectReference).addr();
    let kind = e.call(FORM_TYPE, &args![base]).u32();
    let mut updates_animation = false;
    if kind == 0x1e {
        e.vcall(me, 0x178, &args![]);
    } else if (0x2a..=0x2b).contains(&kind) {
        updates_animation = true;
    }
    if !updates_animation {
        let node = e.call(GET_LOADED_3D, &args![this]).u32();
        if node != 0 {
            let controller = e.call(GET_CONTROLLER, &args![node]).u32();
            if controller != 0 {
                let manager = e.vcall(controller, 0x88, &args![]).u32();
                let mut unfinished = 0u32;
                if manager != 0 && e.call(MANAGER_HAS_SEQUENCES, &args![manager]).bool() {
                    let mut index = 0u32;
                    while index < e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32() {
                        let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
                        if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0
                        {
                            if e.call(0x004e_ef00, &args![sequence, me]).bool() {
                                tes_object_refr_end_sequence(e, this, Ptr::new(sequence));
                                e.call(MANAGER_FINISH_SEQUENCE, &args![manager, sequence, 0.0f32]);
                                let name = sequence_name(e, sequence);
                                e.call(0x0057_8a30, &args![me, name]);
                                e.call(0x0047_ac70, &args![me, sequence]);
                            } else {
                                unfinished += 1;
                            }
                        }
                        index += 1;
                    }
                    if unfinished == 0 {
                        e.call(MANAGER_SET_FLAG, &args![manager, 0u32]);
                    }
                }
            }
        }
    }
    if updates_animation {
        let animation = e.vcall(me, 0x1e4, &args![]).u32();
        let node = e.call(GET_LOADED_3D, &args![this]).u32();
        if animation == 0 {
            if node != 0 {
                tes_object_refr_init_animation(e, this);
            }
        } else if node != 0 && e.call(0x0096_11e0, &args![node]).u32() != 0 {
            let minus_one: f32 = e.global(MINUS_ONE_FLOAT);
            if e.call(FORM_GET_FLAGS, &args![VATS_OBJECT]).u32() == 4
                && me == e.global::<u32>(REFERENCE_COMPARED_WITH_VATS)
            {
                let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_OBJECT]).f64();
                let scale = e.call(TARGET_UPDATE_MULT, &args![VATS_OBJECT]).f64();
                let delta = (scale * frame_time) as f32;
                e.call(ANIMATION_UPDATE, &args![animation, me, delta, minus_one]);
            } else {
                let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_OBJECT]).f64();
                e.call(
                    ANIMATION_UPDATE,
                    &args![animation, me, frame_time as f32, minus_one],
                );
            }
        }
    }
}

// Translated from 005672c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::EndSequence` (Xbox PDB): for a base form of type 0x15 or
/// 0x1c and the sequence named "Unequip" (the pointer at `01197b5c`, compared
/// with `00404dc0`), when the reference's parent cell passes `004543c0`: the
/// 3D gets `00c6a270(3d, 1, 1, 1)`, then either (`008c7aa0`) the manager
/// object `004537b0` is told about it (`0087b1e0(3d, 1)`) or the 3D's motion
/// is set (`00c6a350(3d, 1, 1, 0, 1)`). The virtual `+0x48` is then called
/// with 4.
pub fn tes_object_refr_end_sequence(e: &mut Engine, this: Ptr<TESObjectREFR>, sequence: Ptr) {
    let me = this.addr();
    let data = this.at(TESObjectREFR::data);
    let base = e.get(data, OBJ_REFR::pObjectReference).addr();
    let kind = e.call(FORM_TYPE, &args![base]).u32();
    if kind != 0x15 && kind != 0x1c {
        return;
    }
    let unequip = e.global::<u32>(NAME_UNEQUIP);
    let name = sequence_name(e, sequence.addr());
    if e.call(0x0040_4dc0, &args![name, unequip]).u32() != 0 {
        return;
    }
    let cell = e.get(this, TESObjectREFR::pParentCell).addr();
    if cell == 0 || e.call(0x0045_43c0, &args![cell]).u32() == 0 {
        return;
    }
    let node = e.vcall(me, 0x1d0, &args![]).u32();
    e.call(0x00c6_a270, &args![node, 1u32, 1u32, 1u32]);
    if e.call(0x008c_7aa0, &args![]).bool() {
        let node = e.vcall(me, 0x1d0, &args![]).u32();
        let manager = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_b1e0, &args![manager, node, 1u32]);
    } else {
        e.call(0x0045_43c0, &args![cell]);
        let node = e.vcall(me, 0x1d0, &args![]).u32();
        e.call(0x00c6_a350, &args![node, 1u32, 1u32, 0u32, 1u32]);
    }
    e.vcall(me, 0x48, &args![4u32]);
}

// Translated from 005673c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetScriptPointer` (Xbox PDB): `ExtraDataList::GetScript`
/// (`00418800`) of the reference's extra list.
pub fn tes_object_refr_get_script_pointer(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    e.call(0x0041_8800, &args![list]).u32()
}

// Translated from 005673e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetScriptVariables` (Xbox PDB): the script locals
/// (`00418830`) of the reference's extra list.
pub fn tes_object_refr_get_script_variables(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    e.call(0x0041_8830, &args![list]).u32()
}

// Translated from 00567400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetScale` (Xbox PDB): the reference's scale (`fRefScale`)
/// times the base form's own scale for the base form types 0x2a
/// (`00944300`) and 0x2b (`00567470`); the plain `fRefScale` otherwise.
pub fn tes_object_refr_get_scale(e: &mut Engine, this: Ptr<TESObjectREFR>) -> f32 {
    let mut scale = e.get(this, TESObjectREFR::fRefScale);
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if base != 0 {
        let own = match e.call(FORM_TYPE, &args![base]).u32() {
            0x2a => Some(e.call(0x0094_4300, &args![base]).f64()),
            0x2b => Some(f64::from(fn_00567470(e, Ptr::new(base)))),
            _ => None,
        };
        if let Some(own) = own {
            scale = (own * f64::from(scale)) as f32;
        }
    }
    scale
}

// ---------------------------------------------------------------------------
// Second batch: the functions from `00567470` to `00568fa0`.

/// Callees and data of the second batch, by exe address.
///
/// Extra data kinds the markers use: `0x12` (the used-marker bits) and
/// `0x82` (the reserved-marker bits); `0x25` carries a float next to the base
/// form's own (see `fn_00568ad0`).
const EXTRA_KIND_USED_MARKERS: u32 = 0x12;
const EXTRA_KIND_RESERVED_MARKERS: u32 = 0x82;
const EXTRA_KIND_HEALTH: u32 = 0x25;
/// Getters on the extra list (`this` is the list): the owner (`00418660`,
/// extra kind 0x21), the global (`00418690`, 0x22) and the rank (`004186c0`,
/// 0x23, -1 when absent); their setters `00419700`, `004197d0`, `004198a0`.
const EXTRA_GET_OWNER: u32 = 0x0041_8660;
const EXTRA_GET_GLOBAL: u32 = 0x0041_8690;
const EXTRA_GET_RANK: u32 = 0x0041_86c0;
const EXTRA_SET_OWNER: u32 = 0x0041_9700;
const EXTRA_SET_GLOBAL: u32 = 0x0041_97d0;
const EXTRA_SET_RANK: u32 = 0x0041_98a0;
/// The extra list's link to another reference (`00418460`, the pointer the
/// teleport data `0043a160` is kept in), its setter `00419120`, and the
/// removals of the teleport pointer (`0041ae90`) and of the navmesh portal
/// extra (`0042e730`, found with `0042e2a0`).
const EXTRA_GET_LINK: u32 = 0x0041_8460;
const EXTRA_SET_LINK: u32 = 0x0041_9120;
const EXTRA_REMOVE_LINK: u32 = 0x0041_ae90;
const EXTRA_GET_PORTAL: u32 = 0x0042_e2a0;
const EXTRA_REMOVE_PORTAL: u32 = 0x0042_e730;
/// The portal's target is told about the reference (`004534f0(target,
/// reference)`, an empty function in this build).
const PORTAL_TARGET_NOTIFY: u32 = 0x0045_34f0;
/// `DoorTeleportData::DoorTeleportData` (Xbox PDB), 0x20 bytes.
const DOOR_TELEPORT_DATA_CONSTRUCT: u32 = 0x0043_a160;
const DOOR_TELEPORT_DATA_SIZE: u32 = 0x20;
/// The extra list's encounter zone: getter (`00421c30`) and setter
/// (`00421c60`); and two more extras the exe handles in the same way
/// (`0042e910` getter, `0042e930` setter).
const EXTRA_GET_ZONE: u32 = 0x0042_1c30;
const EXTRA_SET_ZONE: u32 = 0x0042_1c60;
const EXTRA_GET_SECOND: u32 = 0x0042_e910;
const EXTRA_SET_SECOND: u32 = 0x0042_e930;
/// Cell and world space accessors: `TESObjectCELL::GetEncounterZone` as the
/// map names `00546c20`, the cell's owner (`00546a40`), global (`00546aa0`)
/// and rank (`00546ac0`), `TESObjectREFR::GetWorldSpace` (`00575d70`) and
/// the world space's zone (`00458400`, the dword at +0xd0).
const CELL_GET_ZONE: u32 = 0x0054_6c20;
const CELL_GET_OWNER: u32 = 0x0054_6a40;
const CELL_GET_GLOBAL: u32 = 0x0054_6aa0;
const CELL_GET_RANK: u32 = 0x0054_6ac0;
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
const WORLD_SPACE_ZONE: u32 = 0x0045_8400;
/// The encounter zone object the exe treats as "none" (`00546a90`, a static
/// getter of the pointer at `011c9520`), the owner stored in a zone
/// (`009611e0`, +0x18), the zone's level fallback (`00526190`) and the
/// player's level (`0087f9f0`, low word).
const DEFAULT_ZONE: u32 = 0x0054_6a90;
const ZONE_OWNER: u32 = 0x0096_11e0;
const ZONE_LEVEL_FALLBACK: u32 = 0x0052_6190;
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
/// A setting's value (`0043d4d0(setting)`: pointer to its integer) and the
/// two settings and the counter `GetCalcLevel` uses.
const SETTING_VALUE: u32 = 0x0043_d4d0;
const SETTING_COUNTER_LIMIT: u32 = 0x011c_a2b0;
const SETTING_COUNTER_RESTART: u32 = 0x011c_a31c;
const CALC_LEVEL_COUNTER: u32 = 0x011c_a420;
/// Faction tests: `0047d7c0(faction)` and `0047d740(actor base data)`.
const FACTION_IS_EVIL: u32 = 0x0047_d7c0;
const ACTOR_BASE_EVIL_ONLY: u32 = 0x0047_d740;
/// `ExtraUsedMarkers` methods (Xbox PDB `GetMarkerUsed`, `SetMarkerUsed`) and
/// the two constructors (0x10 bytes each) of the used and the reserved
/// extra.
const MARKERS_GET_USED: u32 = 0x0043_2f80;
const MARKERS_SET_USED: u32 = 0x0043_2fc0;
const USED_MARKERS_CONSTRUCT: u32 = 0x0043_2f00;
const RESERVED_MARKERS_CONSTRUCT: u32 = 0x0043_2f50;
const MARKERS_EXTRA_SIZE: u32 = 0x10;
/// Furniture form: `TESFurniture::GetMarkerEnabled` (`00509450`), the
/// marker count (`00509490`) and entry at index (`005094d0`) of its marker
/// table, an entry's kind (`005094b0`, the byte at +0xe) and the kind tests
/// `TESFurniture::IsSitMarker` (`00509510`) and `IsSleepMarker`
/// (`005094f0`), both cdecl.
const FURNITURE_MARKER_ENABLED: u32 = 0x0050_9450;
const FURNITURE_MARKER_COUNT: u32 = 0x0050_9490;
const FURNITURE_MARKER_AT: u32 = 0x0050_94d0;
const FURNITURE_MARKER_KIND: u32 = 0x0050_94b0;
const IS_SIT_MARKER: u32 = 0x0050_9510;
const IS_SLEEP_MARKER: u32 = 0x0050_94f0;
const FURNITURE_MARKER_LIMIT: u32 = 0x1e;
/// `BSFurnitureMarker::Find` (`00c54400`, cdecl, from a 3D object),
/// `FurnitureMark::SetHeading` (`00c54550`, `this` and a float), the mark's
/// kind setter (`00568ab0`, in this file), its position copy (`0098ddd0`),
/// `TESObjectREFR::GetOrientation` (`0056fa00`, a 0x24-byte matrix),
/// `NiMatrix3::TransformVertices` (`00a582f0`, cdecl: matrix, translation,
/// count, source, destination), the vector difference (`00439ef0`), the
/// vector length (`00457990`) and the identity getter of a list node's item
/// (`006815c0`), and the reference's rotation (`00430830`, `this + 0x24`).
const FURNITURE_FIND: u32 = 0x00c5_4400;
const MARK_SET_HEADING: u32 = 0x00c5_4550;
const MARK_SET_POSITION: u32 = 0x0098_ddd0;
const GET_ORIENTATION: u32 = 0x0056_fa00;
const TRANSFORM_VERTICES: u32 = 0x00a5_82f0;
const VECTOR_DIFFERENCE: u32 = 0x0043_9ef0;
const VECTOR_LENGTH: u32 = 0x0045_7990;
const POINT_OF: u32 = 0x0068_15c0;
const ROTATION_OF: u32 = 0x0043_0830;
/// Type descriptors of `TESBoundObject` (`01183108`) and `TESHealthForm`
/// (`01186c3c`) for `__RTDynamicCast` (`00ec43fb`, cdecl).
const TYPE_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_HEALTH_FORM: u32 = 0x0118_6c3c;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `float` helpers: `0040eb10(a, b, epsilon)` is true when `|a - b| <=
/// epsilon` (false for a negative epsilon); `ExtraDataList::GetRadius`
/// (`00422320`) and `SetHealth` (`00419970`, a float), and the base form's
/// own dword at +0xa0 (`004fd400`).
const NEARLY_EQUAL: u32 = 0x0040_eb10;
const EXTRA_GET_RADIUS: u32 = 0x0042_2320;
const EXTRA_SET_HEALTH: u32 = 0x0041_9970;
const FORM_FIELD_A0: u32 = 0x004f_d400;
/// `ExtraDataList::CopyList` (`00411ec0`).
const EXTRA_COPY_LIST: u32 = 0x0041_1ec0;
/// `BaseProcess::GetActorPackageThatIsRunning` as the map names `00717e50`
/// (`this + 4`) and the position it returns when there is no linked door:
/// the record at `011f426c`.
const LINKED_DOOR_POSITION: u32 = 0x0071_7e50;
const NO_DOOR_POSITION: u32 = 0x011f_426c;
/// Forms whose radius the exe special-cases (pointers at `011ca264`,
/// `011ca224`, `011ca228`) and the constants of `GetRadius`: the epsilon
/// float `01017d00`, the setting at `011d0bd8` and the float `5000.0` at
/// `01030020`.
const GLOBAL_RADIUS_FORM_A: u32 = 0x011c_a264;
const GLOBAL_RADIUS_FORM_B: u32 = 0x011c_a224;
const GLOBAL_RADIUS_FORM_C: u32 = 0x011c_a228;
const RADIUS_EPSILON: u32 = 0x0101_7d00;
const RADIUS_SETTING: u32 = 0x011d_0bd8;
const RADIUS_FAR: u32 = 0x0103_0020;
/// Constants of `fn_00568ad0`: 100.0f (`01016410`).
const VALUE_FULL: u32 = 0x0101_6410;
/// Constants of the marker heading: `1000.0` (`01017b70`, double) and
/// `FLT_MAX` (`01016970`).
const HEADING_DIVISOR: u32 = 0x0101_7b70;
const FLOAT_MAX_COPY: u32 = 0x0101_6970;
/// The scale setter: the checks before it runs (`00444ed0`), the formatting
/// through the scrap heap (`00406d00` is `sprintf` with a size, format
/// `"%.2f"` at `01030018`, `atof` `00eca573`), the memory manager getter
/// (`00401020`), the thread's scrap heap (`00aa42e0`), its allocation
/// (`00aa54a0(heap, size, alignment)`, alignment from `010a2720`) and
/// release (`00aa5610`), the limits (0.01 as `double` `01016408` and
/// `float` `01013ea4`; 10.0 as `double` `01020758` and `float`
/// `01017b78`), the node scale setter (`00440490`), the player's 3D by view
/// (`00950bb0(player, first_person)`), the character controller
/// (`009306d0`), `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`),
/// a one-word handle constructor (`00633c90`) and the player refresh
/// (`00962450`).
const GATE_00444ED0: u32 = 0x0044_4ed0;
const MEMORY_MANAGER_GET: u32 = 0x0040_1020;
const SCRAP_HEAP_OF_THREAD: u32 = 0x00aa_42e0;
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
const FORMAT_TO_BUFFER: u32 = 0x0040_6d00;
const ATOF: u32 = 0x00ec_a573;
const FORMAT_TWO_DECIMALS: u32 = 0x0103_0018;
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
const SCALE_MIN_DOUBLE: u32 = 0x0101_6408;
const SCALE_MIN_FLOAT: u32 = 0x0101_3ea4;
const SCALE_MAX_DOUBLE: u32 = 0x0102_0758;
const SCALE_MAX_FLOAT: u32 = 0x0101_7b78;
const SET_NODE_SCALE: u32 = 0x0044_0490;
const PLAYER_3D: u32 = 0x0095_0bb0;
const CHAR_CONTROLLER: u32 = 0x0093_06d0;
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const HANDLE_CONSTRUCT: u32 = 0x0063_3c90;
const REFRESH_PLAYER: u32 = 0x0096_2450;
/// The dword at +0x108 of an actor (`004f8960`).
const ACTOR_FIELD_108: u32 = 0x004f_8960;

// Translated from 00567470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the float at +0x13c of a base form (the
/// `TESObjectREFR::GetScale` of an actor base uses it for form type 0x2b).
pub fn fn_00567470(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x13c)
}

// Translated from 00567490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: sets the reference's scale (`fRefScale`) from
/// `value` rounded through its two decimals as text (`"%.2f"`, then `atof`),
/// clamped to 0.01 .. 10.0, and applies it to the 3D: the node scale is set
/// and a zero velocity record given. For an actor the character
/// controller's two floats at +0x550 / +0x554 are kept across the
/// virtual `+0x2a0` / `+0x1c4` calls, the saved acquire object is told
/// (virtual `+0x290` with a one-word handle holding 0) and the player is
/// refreshed. Returns at once when `00444ed0` holds and the base form is
/// not of type 4. The 0x20-byte scrap-heap block holds the text.
pub fn fn_00567490(e: &mut Engine, this: Ptr<TESObjectREFR>, value: f32) {
    let me = this.addr();
    if e.call(GATE_00444ED0, &args![me]).bool() && e.call(GET_BASE_FORM, &args![me]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() != 4 {
            return;
        }
    }
    let manager = e.call(MEMORY_MANAGER_GET, &args![]).u32();
    let heap = e.call(SCRAP_HEAP_OF_THREAD, &args![manager]).u32();
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let text = e
        .call(SCRAP_HEAP_ALLOCATE, &args![heap, 0x20u32, alignment])
        .u32();
    e.call(
        FORMAT_TO_BUFFER,
        &args![text, 0x200u32, FORMAT_TWO_DECIMALS, f64::from(value)],
    );
    let mut value = e.call(ATOF, &args![text]).f64() as f32;
    let low: f64 = e.global(SCALE_MIN_DOUBLE);
    if f64::from(value) < low {
        value = e.global(SCALE_MIN_FLOAT);
    }
    let high: f64 = e.global(SCALE_MAX_DOUBLE);
    if f64::from(value) > high {
        value = e.global(SCALE_MAX_FLOAT);
    }
    e.set(this, TESObjectREFR::fRefScale, value);
    e.vcall(me, 0x48, &args![0x10u32]);
    let scale = tes_object_refr_get_scale(e, this);
    let player = e.global::<u32>(GLOBAL_PLAYER);
    let node = if me == player {
        let first_person = e.call(PLAYER_3D, &args![player, 1u32]).u32();
        if first_person != 0 {
            e.call(SET_NODE_SCALE, &args![first_person, scale]);
            set_velocity(e, first_person, 0.0, 0, 0);
        }
        e.call(PLAYER_3D, &args![player, 0u32]).u32()
    } else {
        e.vcall(me, 0x1d0, &args![]).u32()
    };
    if node != 0 {
        e.call(SET_NODE_SCALE, &args![node, scale]);
        set_velocity(e, node, 0.0, 0, 0);
        if e.vcall(me, 0x100, &args![]).bool() {
            let controller = e.call(CHAR_CONTROLLER, &args![me]).u32();
            let mut first = 0.0f32;
            let mut second = 0.0f32;
            if controller != 0 {
                first = fn_00567730(e, Ptr::new(controller));
                second = fn_00567750(e, Ptr::new(controller));
            }
            e.vcall(me, 0x2a0, &args![]);
            if e.call(SAVED_ACQUIRE_OBJECT, &args![me]).u32() != 0 {
                let acquire = e.call(SAVED_ACQUIRE_OBJECT, &args![me]).u32();
                e.with_stack(4, |e, handle| {
                    e.call(HANDLE_CONSTRUCT, &args![handle, 0u32]);
                    let word = e.mem.u32(handle.addr());
                    e.vcall(acquire, 0x290, &args![word]);
                });
            }
            e.vcall(me, 0x1c4, &args![]);
            let controller = e.call(CHAR_CONTROLLER, &args![me]).u32();
            if controller != 0 {
                e.mem.set_f32(controller + 0x550, first);
                e.mem.set_f32(controller + 0x554, second);
            }
            if me == player {
                e.call(REFRESH_PLAYER, &args![player]);
            }
        }
    }
    let manager = e.call(MEMORY_MANAGER_GET, &args![]).u32();
    let heap = e.call(SCRAP_HEAP_OF_THREAD, &args![manager]).u32();
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, text]);
}

// Translated from 00567730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the float at +0x550 of a character
/// controller.
pub fn fn_00567730(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x550)
}

// Translated from 00567750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the float at +0x554 of a character
/// controller.
pub fn fn_00567750(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x554)
}

// Translated from 00567770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the reference's own owner extra (`00418660`
/// on its extra list), 0 when it has none.
pub fn fn_00567770(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_OWNER, &args![list]).u32()
}

/// The reference a reference links to (`00568e50`) and the first dword of
/// that link (`00559450`); 0 as soon as one of them is. The exe asks for the
/// dword twice (`if (f(x)) { use f(x) }`).
fn linked_reference(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let link = fn_00568e50(e, this);
    if link == 0 || e.call(READ_FIRST_DWORD, &args![link]).u32() == 0 {
        return 0;
    }
    e.call(READ_FIRST_DWORD, &args![link]).u32()
}

// Translated from 00567790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetOwner` (Xbox PDB): the owner extra of the reference;
/// for a non-actor without one, that of its linked reference, and, when it
/// is not furniture and its base form is not of type 0x1c or 0x15, the
/// owner of its encounter zone (unless that is the default zone), else that
/// of its parent cell. An actor answers its own extra only.
pub fn tes_object_refr_get_owner(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut owner = fn_00567770(e, this);
    if e.vcall(me, 0x100, &args![]).bool() {
        return owner;
    }
    if owner == 0 {
        let link = fn_00568e50(e, this);
        if link != 0 && e.call(READ_FIRST_DWORD, &args![link]).u32() != 0 {
            let target = e.call(READ_FIRST_DWORD, &args![link]).u32();
            owner = fn_00567770(e, Ptr::new(target));
        }
    }
    if owner == 0
        && !tes_object_refr_is_furniture(e, this)
        && e.call(GET_BASE_FORM, &args![me]).u32() != 0
    {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() != 0x1c {
            let base = e.call(GET_BASE_FORM, &args![me]).u32();
            if e.call(FORM_TYPE, &args![base]).u32() != 0x15 {
                let zone = tes_object_refr_get_encounter_zone(e, this);
                if zone != 0 && zone != e.call(DEFAULT_ZONE, &args![]).u32() {
                    owner = e.call(ZONE_OWNER, &args![zone]).u32();
                }
                if owner == 0 && e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
                    let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
                    owner = e.call(CELL_GET_OWNER, &args![cell]).u32();
                }
            }
        }
    }
    owner
}

// Translated from 005678a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::IsPartofEvilFaction` (Xbox PDB): with an owner, whether
/// it is evil (a form of type 8 through `0047d7c0`, an actor base of type
/// 0x2a through `0047d740` on its data at +0x30); without one, whether the
/// reference's actor base (virtual `+0x1a4`, else its base form) is of type
/// 0x2a and evil in that same way.
pub fn tes_object_refr_is_partof_evil_faction(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u8 {
    let me = this.addr();
    let owner = tes_object_refr_get_owner(e, this);
    let mut evil = 0u8;
    if owner != 0 {
        if e.call(FORM_TYPE, &args![owner]).u32() == 8 {
            evil = e.call(FACTION_IS_EVIL, &args![owner]).u8();
        } else if e.call(FORM_TYPE, &args![owner]).u32() == 0x2a {
            evil = e.call(ACTOR_BASE_EVIL_ONLY, &args![owner + 0x30]).u8();
        }
    } else {
        let mut form = e.vcall(me, 0x1a4, &args![]).u32();
        if form == 0 {
            form = e.call(GET_BASE_FORM, &args![me]).u32();
        }
        let mut actor_base = 0;
        if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == 0x2a {
            actor_base = form;
        }
        if actor_base != 0 {
            evil = e.call(ACTOR_BASE_EVIL_ONLY, &args![actor_base + 0x30]).u8();
        }
    }
    evil
}

// Translated from 00567960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetOwnershipGlobal` (Xbox PDB): the global extra of the
/// reference, else that of its linked reference, else the parent cell's
/// (`00546aa0`).
pub fn tes_object_refr_get_ownership_global(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let list = extra_list(e, me);
    let mut global = e.call(EXTRA_GET_GLOBAL, &args![list]).u32();
    if global == 0 {
        let target = linked_reference(e, this);
        if target != 0 {
            let list = extra_list(e, target);
            global = e.call(EXTRA_GET_GLOBAL, &args![list]).u32();
        }
    }
    if global == 0 && e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        global = e.call(CELL_GET_GLOBAL, &args![cell]).u32();
    }
    global
}

// Translated from 005679f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetOwnershipRank` (Xbox PDB): the rank extra (-1 when
/// absent) of the reference, else of its linked reference, else the rank
/// stored in its encounter zone (`00567ab0`, unless that is the default
/// zone), else the parent cell's (`00546ac0`); 0 when none has one.
pub fn tes_object_refr_get_ownership_rank(e: &mut Engine, this: Ptr<TESObjectREFR>) -> i32 {
    let me = this.addr();
    let list = extra_list(e, me);
    let mut rank = e.call(EXTRA_GET_RANK, &args![list]).i32();
    if rank == -1 {
        let target = linked_reference(e, this);
        if target != 0 {
            let list = extra_list(e, target);
            rank = e.call(EXTRA_GET_RANK, &args![list]).i32();
        }
    }
    if rank == -1 {
        let zone = tes_object_refr_get_encounter_zone(e, this);
        if zone != 0 && zone != e.call(DEFAULT_ZONE, &args![]).u32() {
            rank = fn_00567ab0(e, Ptr::new(zone));
        }
        if rank == -1 && e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
            rank = e.call(CELL_GET_RANK, &args![cell]).i32();
        }
    }
    if rank == -1 {
        rank = 0;
    }
    rank
}

// Translated from 00567ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the signed byte at +0x1c of an encounter zone
/// (its ownership rank), sign-extended.
pub fn fn_00567ab0(e: &mut Engine, this: Ptr) -> i32 {
    i32::from(e.mem.u8(this.addr() + 0x1c) as i8)
}

/// Runs `apply` on the reference that `00568e50` links to and tells the
/// target about the change (virtual `+0x48` with 0x40), as the three owner
/// setters do after changing themselves: `if (link && link->first) {
/// apply(link); link->first->virtual48(0x40) }`. `apply` is handed the link
/// and reads the dword itself, the number of times the exe does.
fn mirror_on_linked(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    apply: impl FnOnce(&mut Engine, u32),
) {
    let link = fn_00568e50(e, this);
    if link != 0 && e.call(READ_FIRST_DWORD, &args![link]).u32() != 0 {
        apply(e, link);
        let target = e.call(READ_FIRST_DWORD, &args![link]).u32();
        e.vcall(target, 0x48, &args![0x40u32]);
    }
}

/// The extra list of the reference at the first dword of `link`.
fn list_of_link_target(e: &mut Engine, link: u32) -> u32 {
    let target = e.call(READ_FIRST_DWORD, &args![link]).u32();
    extra_list(e, target)
}

// Translated from 00567ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: sets `owner` through the setter `00419700`
/// (the one that pairs with the owner getter `00418660`) on the reference's
/// extra list and marks it changed (virtual `+0x48`, 0x40); the linked
/// reference gets the owner 0.
pub fn fn_00567ad0(e: &mut Engine, this: Ptr<TESObjectREFR>, owner: u32) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_SET_OWNER, &args![list, owner]);
    e.vcall(me, 0x48, &args![0x40u32]);
    mirror_on_linked(e, this, |e, link| {
        let list = list_of_link_target(e, link);
        e.call(EXTRA_SET_OWNER, &args![list, 0u32]);
    });
}

// Translated from 00567b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetGlobal` (Xbox PDB): sets the ownership `global`
/// (`004197d0`) on the reference's extra list and marks it changed
/// (virtual `+0x48`, 0x40); the linked reference gets the global 0.
pub fn tes_object_refr_set_global(e: &mut Engine, this: Ptr<TESObjectREFR>, global: u32) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_SET_GLOBAL, &args![list, global]);
    e.vcall(me, 0x48, &args![0x40u32]);
    mirror_on_linked(e, this, |e, link| {
        let list = list_of_link_target(e, link);
        e.call(EXTRA_SET_GLOBAL, &args![list, 0u32]);
    });
}

// Translated from 00567bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB (the map's `ExtraDataList::SetRadiation` name
/// on `004198a0` is a folded body): sets the ownership `rank` (`004198a0`,
/// the setter that pairs with the rank getter `004186c0`) and marks the
/// reference changed (virtual `+0x48`, 0x40); the linked reference gets the
/// rank -1.
pub fn fn_00567bd0(e: &mut Engine, this: Ptr<TESObjectREFR>, rank: i32) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_SET_RANK, &args![list, rank]);
    e.vcall(me, 0x48, &args![0x40u32]);
    mirror_on_linked(e, this, |e, link| {
        let list = list_of_link_target(e, link);
        e.call(EXTRA_SET_RANK, &args![list, -1i32]);
    });
}

// Translated from 00567c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: clears the ownership of the reference (owner
/// 0, global 0, rank -1) and marks it changed (virtual `+0x48`, 0x40); the
/// linked reference is cleared in the same way.
pub fn fn_00567c50(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_SET_OWNER, &args![list, 0u32]);
    let list = extra_list(e, me);
    e.call(EXTRA_SET_GLOBAL, &args![list, 0u32]);
    let list = extra_list(e, me);
    e.call(EXTRA_SET_RANK, &args![list, -1i32]);
    e.vcall(me, 0x48, &args![0x40u32]);
    mirror_on_linked(e, this, |e, link| {
        let list = list_of_link_target(e, link);
        e.call(EXTRA_SET_OWNER, &args![list, 0u32]);
        let list = list_of_link_target(e, link);
        e.call(EXTRA_SET_GLOBAL, &args![list, 0u32]);
        let list = list_of_link_target(e, link);
        e.call(EXTRA_SET_RANK, &args![list, -1i32]);
    });
}

// Translated from 00567d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetEncounterZone` (Xbox PDB): the zone of the reference's
/// extra list (`00421c30`), else of its parent cell (`00546c20`), else of
/// its world space (`00458400`).
pub fn tes_object_refr_get_encounter_zone(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let list = extra_list(e, me);
    let mut zone = e.call(EXTRA_GET_ZONE, &args![list]).u32();
    if zone == 0 {
        if e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
            zone = e.call(CELL_GET_ZONE, &args![cell]).u32();
        }
        if zone == 0 && e.call(GET_WORLD_SPACE, &args![me]).u32() != 0 {
            let world = e.call(GET_WORLD_SPACE, &args![me]).u32();
            zone = e.call(WORLD_SPACE_ZONE, &args![world]).u32();
        }
    }
    zone
}

// Translated from 00567d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the encounter zone of the parent cell
/// (`pParentCell`, `00546c20`), else of the world space, else 0 (it does not
/// ask the extra list).
pub fn fn_00567d90(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let cell = e.get(this, TESObjectREFR::pParentCell).addr();
    if cell != 0 {
        return e.call(CELL_GET_ZONE, &args![cell]).u32();
    }
    let world = e.call(GET_WORLD_SPACE, &args![this.addr()]).u32();
    if world == 0 {
        return 0;
    }
    e.call(WORLD_SPACE_ZONE, &args![world]).u32()
}

// Translated from 00567dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetEncounterZone` (Xbox PDB): stores `zone` in the extra
/// list (`00421c60`) and marks the reference changed (virtual `+0x48`,
/// 0x20000000).
pub fn tes_object_refr_set_encounter_zone(e: &mut Engine, this: Ptr<TESObjectREFR>, zone: u32) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_SET_ZONE, &args![list, zone]);
    e.vcall(me, 0x48, &args![0x2000_0000u32]);
}

// Translated from 00567e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetCalcLevel` (Xbox PDB): the level of the reference's
/// encounter zone (`00567ec0`) when it has one that is not the default zone,
/// else the player's level (low word of `0087f9f0`). With `vary` the global
/// counter at `011ca420` is stepped (and wrapped to minus the restart
/// setting when it passes the limit setting) and added. At least 1.
pub fn tes_object_refr_get_calc_level(e: &mut Engine, this: Ptr<TESObjectREFR>, vary: u8) -> u32 {
    let zone = tes_object_refr_get_encounter_zone(e, this);
    let mut level: u32;
    if zone != 0 && zone != e.call(DEFAULT_ZONE, &args![]).u32() {
        level = u32::from(fn_00567ec0(e, Ptr::new(zone)));
    } else {
        let player = e.global::<u32>(GLOBAL_PLAYER);
        level = e.call(PLAYER_LEVEL, &args![player]).u32() & 0xffff;
    }
    if vary != 0 {
        let mut counter = e.global::<i32>(CALC_LEVEL_COUNTER).wrapping_add(1);
        e.set_global(CALC_LEVEL_COUNTER, counter);
        let limit = e.call(SETTING_VALUE, &args![SETTING_COUNTER_LIMIT]).u32();
        if counter > e.mem.i32(limit) {
            let restart = e.call(SETTING_VALUE, &args![SETTING_COUNTER_RESTART]).u32();
            counter = e.mem.i32(restart).wrapping_mul(-1);
            e.set_global(CALC_LEVEL_COUNTER, counter);
        }
        level = level.wrapping_add(counter as u32);
    }
    if level == 0 {
        level = 1;
    }
    level
}

// Translated from 00567ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the word at +0x2c of an encounter zone (its
/// level), or `00526190` when that is 0.
pub fn fn_00567ec0(e: &mut Engine, this: Ptr) -> u16 {
    let level = e.mem.u16(this.addr() + 0x2c);
    if level == 0 {
        return e.call(ZONE_LEVEL_FALLBACK, &args![this.addr()]).u32() as u16;
    }
    level
}

// Translated from 00567f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the second extra's (`0042e910` on the
/// reference's list) dword at +0xc (`0043b230`), 0 when it has none.
pub fn fn_00567f00(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    let extra = e.call(EXTRA_GET_SECOND, &args![list]).u32();
    if extra == 0 {
        return 0;
    }
    e.call(GET_CONTROLLER, &args![extra]).u32()
}

// Translated from 00567f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: sets the second extra (`0042e930`) on the
/// reference's list and marks it changed (virtual `+0x48`, 0x80000000).
pub fn fn_00567f40(e: &mut Engine, this: Ptr<TESObjectREFR>, value: u32) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_SET_SECOND, &args![list, value]);
    e.vcall(me, 0x48, &args![0x8000_0000u32]);
}

/// `BaseExtraList::GetExtraData(kind)` on the reference's list.
fn extra_data_of(e: &mut Engine, this: u32, kind: u32) -> u32 {
    let list = extra_list(e, this);
    e.call(EXTRA_GET_DATA, &args![list, kind]).u32()
}

// Translated from 00567f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMarkerUsed` (Xbox PDB): whether marker `index` is used:
/// by the used-marker extra when the base form is furniture (type 0x27), else
/// (unless `ignore_reserved`) by the reserved-marker extra.
pub fn tes_object_refr_get_marker_used(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    index: u32,
    ignore_reserved: u8,
) -> u8 {
    let me = this.addr();
    let mut used = 0u8;
    let used_extra = extra_data_of(e, me, EXTRA_KIND_USED_MARKERS);
    let reserved_extra = extra_data_of(e, me, EXTRA_KIND_RESERVED_MARKERS);
    if used_extra != 0 && base_type(e, me) == Some(0x27) {
        used = e.call(MARKERS_GET_USED, &args![used_extra, index]).u8();
    }
    if reserved_extra != 0 && used == 0 && ignore_reserved == 0 {
        used = e.call(MARKERS_GET_USED, &args![reserved_extra, index]).u8();
    }
    used
}

/// What `SetMarkerUsed` and `SetMarkerReserved` share after their checks:
/// marks the reference changed (virtual `+0x48`, 0x80000000), then sets the
/// marker `index` in the extra of `kind` (creating it with `construct` when
/// `value` is set and there is none) and removes an extra that ends up with
/// no marker (its dword at +0xc is 0). The unwinding frame of the exe around
/// the construction is not translated.
fn update_marker_extra(
    e: &mut Engine,
    this: u32,
    kind: u32,
    construct: u32,
    index: u32,
    value: u8,
) {
    e.vcall(this, 0x48, &args![0x8000_0000u32]);
    let extra = extra_data_of(e, this, kind);
    if extra != 0 {
        e.call(MARKERS_SET_USED, &args![extra, index, value]);
        if e.mem.u32(extra + 0xc) == 0 {
            let list = extra_list(e, this);
            e.call(EXTRA_REMOVE, &args![list, extra, 1u32]);
        }
    } else if value != 0 {
        let block = e.call(OPERATOR_NEW, &args![MARKERS_EXTRA_SIZE]).u32();
        let created = if block == 0 {
            0
        } else {
            e.call(construct, &args![block]).u32()
        };
        let list = extra_list(e, this);
        e.call(EXTRA_ADD, &args![list, created]);
        e.call(MARKERS_SET_USED, &args![created, index, value]);
    }
}

// Translated from 00568020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetMarkerUsed` (Xbox PDB): for a furniture reference
/// (base form type 0x27) sets marker `index` used or free in the used-marker
/// extra (kind 0x12), see [`update_marker_extra`].
pub fn tes_object_refr_set_marker_used(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    index: u32,
    used: u8,
) {
    let me = this.addr();
    if base_type(e, me) == Some(0x27) {
        update_marker_extra(
            e,
            me,
            EXTRA_KIND_USED_MARKERS,
            USED_MARKERS_CONSTRUCT,
            index,
            used,
        );
    }
}

// Translated from 00568150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetMarkerReserved` (Xbox PDB): sets marker `index`
/// reserved or free in the reserved-marker extra (kind 0x82), see
/// [`update_marker_extra`]; no check of the base form.
pub fn tes_object_refr_set_marker_reserved(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    index: u32,
    reserved: u8,
) {
    update_marker_extra(
        e,
        this.addr(),
        EXTRA_KIND_RESERVED_MARKERS,
        RESERVED_MARKERS_CONSTRUCT,
        index,
        reserved,
    );
}

// Translated from 00568260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::HasFreeMarker` (Xbox PDB): whether
/// [`tes_object_refr_get_first_free_marker_index`] finds a marker; false
/// without a base form, and for a base form that is not furniture when
/// `used_only` is set.
pub fn tes_object_refr_has_free_marker(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    used_only: u8,
) -> bool {
    let mut found = false;
    if let Some(kind) = base_type(e, this.addr()) {
        if kind == 0x27 || used_only == 0 {
            found = tes_object_refr_get_first_free_marker_index(e, this, used_only) != -1;
        }
    }
    found
}

// Translated from 005682c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetFirstFreeMarkerIndex` (Xbox PDB): for furniture the
/// first of the 0x1e markers that the furniture form enables and that is not
/// used (nor, unless `used_only`, reserved); for another base form 0 when
/// the single marker 0 is free (and `used_only` is not set, and one of the
/// two extras exists, or neither does); -1 when there is none or no base
/// form.
pub fn tes_object_refr_get_first_free_marker_index(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    used_only: u8,
) -> i32 {
    let me = this.addr();
    let used = extra_data_of(e, me, EXTRA_KIND_USED_MARKERS);
    let reserved = extra_data_of(e, me, EXTRA_KIND_RESERVED_MARKERS);
    let Some(kind) = base_type(e, me) else {
        return -1;
    };
    if kind == 0x27 {
        let furniture = e.call(GET_BASE_FORM, &args![me]).u32();
        for index in 0..FURNITURE_MARKER_LIMIT {
            if e.call(FURNITURE_MARKER_ENABLED, &args![furniture, index])
                .u8()
                == 0
            {
                continue;
            }
            if used != 0 && e.call(MARKERS_GET_USED, &args![used, index]).u8() != 0 {
                continue;
            }
            if used_only == 0
                && reserved != 0
                && e.call(MARKERS_GET_USED, &args![reserved, index]).u8() != 0
            {
                continue;
            }
            return index as i32;
        }
        return -1;
    }
    if used_only != 0 || (used == 0 && reserved == 0) {
        return 0;
    }
    // A single marker, index 0.
    if (used == 0 || e.call(MARKERS_GET_USED, &args![used, 0u32]).u8() == 0)
        && (reserved == 0 || e.call(MARKERS_GET_USED, &args![reserved, 0u32]).u8() == 0)
    {
        return 0;
    }
    -1
}

// Translated from 00568480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: frees all 0x1e markers of a furniture
/// reference (`SetMarkerUsed(i, 0)` and `SetMarkerReserved(i, 0)` for each);
/// nothing for another base form.
pub fn fn_00568480(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if base_type(e, me) == Some(0x27) && e.call(GET_BASE_FORM, &args![me]).u32() != 0 {
        for index in 0..FURNITURE_MARKER_LIMIT {
            tes_object_refr_set_marker_used(e, this, index, 0);
            tes_object_refr_set_marker_reserved(e, this, index, 0);
        }
    }
}

/// The heading `fn_00568650(entry)` of a marker entry plus the reference's
/// rotation about z (`00430830` + 8), summed in `double` and rounded to
/// `float` as the exe does.
fn marker_heading(e: &mut Engine, this: u32, entry: u32) -> f32 {
    let own = fn_00568650(e, Ptr::new(entry));
    let rotation = e.call(ROTATION_OF, &args![this]).u32();
    (f64::from(e.mem.f32(rotation + 8)) + f64::from(own)) as f32
}

// Translated from 00568500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMarkerAtIndex` (Xbox PDB): for a furniture reference
/// with a loaded 3D whose furniture markers (`BSFurnitureMarker::Find`) have
/// an entry `index`: copies the entry's four words to `out`, moves its
/// position (the vector `006815c0(out)` gives) by the reference's orientation
/// and location (`NiMatrix3::TransformVertices`, one vertex, written back to
/// `out`), and sets the entry's heading (`FurnitureMark::SetHeading`) to the
/// reference's rotation about z plus the entry's own. Returns whether the
/// entry exists.
pub fn tes_object_refr_get_marker_at_index(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    index: u32,
    out: Ptr,
) -> bool {
    let me = this.addr();
    let out = out.addr();
    let mut found = false;
    if base_type(e, me) != Some(0x27) {
        return found;
    }
    if e.vcall(me, 0x1d0, &args![]).u32() == 0 {
        return found;
    }
    let furniture = e.call(GET_BASE_FORM, &args![me]).u32();
    if furniture == 0 {
        return found;
    }
    let node = e.vcall(me, 0x1d0, &args![]).u32();
    let markers = e.call(FURNITURE_FIND, &args![node]).u32();
    if markers == 0 {
        return found;
    }
    let count = e.call(FURNITURE_MARKER_COUNT, &args![markers]).u32();
    if count <= index {
        return found;
    }
    found = true;
    let entry = e.call(FURNITURE_MARKER_AT, &args![markers, index]).u32();
    for word in 0..4 {
        let value = e.mem.u32(entry + 4 * word);
        e.mem.set_u32(out + 4 * word, value);
    }
    let point = e.call(POINT_OF, &args![out]).u32();
    e.with_stack(12, |e, source| {
        for word in 0..3 {
            let value = e.mem.u32(point + 4 * word);
            e.mem.set_u32(source.addr() + 4 * word, value);
        }
        let location = e.vcall(me, 0x1f4, &args![]).u32();
        e.with_stack(0x24, |e, matrix| {
            let matrix = e.call(GET_ORIENTATION, &args![me, matrix]).u32();
            e.call(
                TRANSFORM_VERTICES,
                &args![matrix, location, 1u32, source, out],
            );
        });
    });
    let heading = marker_heading(e, me, entry);
    e.call(MARK_SET_HEADING, &args![out, heading]);
    found
}

// Translated from 00568650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the heading of a furniture marker entry, its
/// word at +0xc divided by 1000.
pub fn fn_00568650(e: &mut Engine, this: Ptr) -> f32 {
    let raw = e.mem.u16(this.addr() + 0xc);
    let divisor: f64 = e.global(HEADING_DIVISOR);
    (f64::from(raw) / divisor) as f32
}

// Translated from 00568680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::IsFurniture` (Xbox PDB): whether the reference has a base
/// form (`data.pObjectReference`) of type 0x27.
pub fn tes_object_refr_is_furniture(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let data = this.at(TESObjectREFR::data);
    if e.get(data, OBJ_REFR::pObjectReference).addr() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this.addr()]).u32();
        return e.call(FORM_TYPE, &args![base]).u32() == 0x27;
    }
    false
}

// Translated from 005686b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetClosestFreeMarker` (Xbox PDB), `RET 0x18`: of the free
/// furniture markers the closest to `from` (distance of the transformed
/// marker positions, `00457990` of the difference `00439ef0`). A free
/// marker is one the furniture enables, `GetMarkerUsed(i, ignore_reserved)`
/// does not report, and whose kind is a sit marker with `sit` set or a sleep
/// marker with `sleep` set, or neither kind. The chosen index is stored in
/// `*index_out`, and `mark` (a `FurnitureMark`) gets the kind, the position
/// and the heading. Without a loaded 3D, and for a reference that is not
/// furniture and `ignore_reserved` is 0, `mark` gets the reference's own
/// location and heading 0 and `*index_out` is -1 (true for the non-furniture
/// case, false for furniture without 3D). The scratch arrays of the exe's
/// stack (`alloca`) are heap blocks here.
#[allow(clippy::too_many_arguments)]
pub fn tes_object_refr_get_closest_free_marker(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    from: Ptr,
    sit: u8,
    sleep: u8,
    mark: Ptr,
    index_out: Ptr,
    ignore_reserved: u8,
) -> bool {
    let me = this.addr();
    let mark = mark.addr();
    let index_out = index_out.addr();
    let mut found = false;
    if base_type(e, me) == Some(0x27) {
        if e.vcall(me, 0x1d0, &args![]).u32() == 0 {
            let location = e.vcall(me, 0x1f4, &args![]).u32();
            e.call(MARK_SET_POSITION, &args![mark, location]);
            fn_00568ab0(e, Ptr::new(mark), 0);
            e.call(MARK_SET_HEADING, &args![mark, 0.0f32]);
            e.mem.set_u32(index_out, u32::MAX);
            return false;
        }
        let furniture = e.call(GET_BASE_FORM, &args![me]).u32();
        if furniture == 0 {
            return found;
        }
        let node = e.vcall(me, 0x1d0, &args![]).u32();
        let markers = e.call(FURNITURE_FIND, &args![node]).u32();
        if markers == 0 {
            return found;
        }
        let count = e.call(FURNITURE_MARKER_COUNT, &args![markers]).u32();
        if count == 0 {
            return found;
        }
        let positions = e.mem.alloc(count * 12);
        let indices = e.mem.alloc(count * 4);
        let mut candidates = 0u32;
        for index in 0..count {
            if e.call(FURNITURE_MARKER_ENABLED, &args![furniture, index])
                .u8()
                == 0
            {
                continue;
            }
            if tes_object_refr_get_marker_used(e, this, index, ignore_reserved) != 0 {
                continue;
            }
            let entry = e.call(FURNITURE_MARKER_AT, &args![markers, index]).u32();
            let kind = e.call(FURNITURE_MARKER_KIND, &args![entry]).u32();
            let wanted = (e.call(IS_SIT_MARKER, &args![kind]).u8() != 0 && sit != 0)
                || (e.call(IS_SLEEP_MARKER, &args![kind]).u8() != 0 && sleep != 0)
                || (e.call(IS_SIT_MARKER, &args![kind]).u8() == 0
                    && e.call(IS_SLEEP_MARKER, &args![kind]).u8() == 0);
            if !wanted {
                continue;
            }
            let entry = e.call(FURNITURE_MARKER_AT, &args![markers, index]).u32();
            let point = e.call(POINT_OF, &args![entry]).u32();
            for word in 0..3 {
                let value = e.mem.u32(point + 4 * word);
                e.mem.set_u32(positions + candidates * 12 + 4 * word, value);
            }
            e.mem.set_u32(indices + candidates * 4, index);
            candidates += 1;
        }
        if candidates != 0 {
            let moved = e.mem.alloc(count * 12);
            let location = e.vcall(me, 0x1f4, &args![]).u32();
            e.with_stack(0x24, |e, matrix| {
                let matrix = e.call(GET_ORIENTATION, &args![me, matrix]).u32();
                e.call(
                    TRANSFORM_VERTICES,
                    &args![matrix, location, candidates, positions, moved],
                );
            });
            let mut closest = e.global::<f32>(FLOAT_MAX_COPY);
            let mut closest_slot = 0x7fu32;
            for slot in 0..candidates {
                let distance = e.with_stack(12, |e, difference| {
                    let difference = e
                        .call(
                            VECTOR_DIFFERENCE,
                            &args![from, difference, moved + slot * 12],
                        )
                        .u32();
                    e.call(VECTOR_LENGTH, &args![difference]).f32()
                });
                if distance < closest {
                    closest = distance;
                    closest_slot = slot;
                }
            }
            if closest_slot != 0x7f {
                let index = e.mem.u32(indices + closest_slot * 4);
                e.mem.set_u32(index_out, index);
                let entry = e.call(FURNITURE_MARKER_AT, &args![markers, index]).u32();
                let kind = e.call(FURNITURE_MARKER_KIND, &args![entry]).u32();
                fn_00568ab0(e, Ptr::new(mark), kind as u8);
                e.call(MARK_SET_POSITION, &args![mark, moved + closest_slot * 12]);
                let entry = e.call(FURNITURE_MARKER_AT, &args![markers, index]).u32();
                let heading = marker_heading(e, me, entry);
                e.call(MARK_SET_HEADING, &args![mark, heading]);
                found = true;
            }
            e.mem.free(moved);
        }
        e.mem.free(positions);
        e.mem.free(indices);
        return found;
    }
    if ignore_reserved == 0 {
        let reserved = extra_data_of(e, me, EXTRA_KIND_RESERVED_MARKERS);
        if reserved != 0 {
            // (the exe computes `!GetMarkerUsed(reserved, 0)` here and then
            // overwrites it with true below)
            e.call(MARKERS_GET_USED, &args![reserved, 0u32]);
        }
        let location = e.vcall(me, 0x1f4, &args![]).u32();
        e.call(MARK_SET_POSITION, &args![mark, location]);
        fn_00568ab0(e, Ptr::new(mark), 0);
        e.call(MARK_SET_HEADING, &args![mark, 0.0f32]);
        e.mem.set_u32(index_out, u32::MAX);
        found = true;
    }
    found
}

// Translated from 00568ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: sets the kind byte (+0xe) of a
/// `FurnitureMark`.
pub fn fn_00568ab0(e: &mut Engine, this: Ptr, kind: u8) {
    e.mem.set_u8(this.addr() + 0xe, kind);
}

// Translated from 00568ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB, returns a float: -1.0 by default. An actor
/// (virtual `+0x100`) answers 100.0 when its dword at +0x108 (`004f8960`) is
/// 6, else the integer its embedded object at +0xa4 gives for 0x10 (virtual
/// `+8`). Another reference whose base form casts to the 0x25-extra's
/// owner type (`TESBoundObject` to `TESHealthForm`) answers the float of its
/// extra 0x25 (+0xc), else the base form's own value (virtual `+0x10`).
pub fn fn_00568ad0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> f32 {
    let me = this.addr();
    let mut result: f32 = e.global(MINUS_ONE_FLOAT);
    if e.vcall(me, 0x100, &args![]).bool() {
        if e.call(ACTOR_FIELD_108, &args![me]).i32() == 6 {
            result = e.global(VALUE_FULL);
        } else {
            let value = e.vcall(me + 0xa4, 8, &args![0x10u32]).i32();
            result = value as f32;
        }
    } else {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        let health_form = e
            .call(
                RT_DYNAMIC_CAST,
                &args![base, 0u32, TYPE_BOUND_OBJECT, TYPE_HEALTH_FORM, 0u32],
            )
            .u32();
        if health_form != 0 {
            let extra = extra_data_of(e, me, EXTRA_KIND_HEALTH);
            if extra != 0 {
                result = e.mem.f32(extra + 0xc);
            } else {
                let value = e.vcall(health_form, 0x10, &args![]).u32();
                result = value as f32;
            }
        }
    }
    result
}

// Translated from 00568bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetExtra` (Xbox PDB): `ExtraDataList::CopyList`
/// (`00411ec0`) of `source` onto the reference's list.
pub fn tes_object_refr_set_extra(e: &mut Engine, this: Ptr<TESObjectREFR>, source: Ptr) {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_COPY_LIST, &args![list, source]);
}

// Translated from 00568bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: stores `value` as the extra 0x25 float next to
/// the base form's (see `fn_00568ad0`), when the base form casts to
/// `TESHealthForm`. An existing extra is removed when `value` equals the
/// form's own (virtual `+0x10`), else updated; without one a differing value
/// is set through `00419970`.
pub fn fn_00568bd0(e: &mut Engine, this: Ptr<TESObjectREFR>, value: f32) {
    let me = this.addr();
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    let health_form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![base, 0u32, TYPE_BOUND_OBJECT, TYPE_HEALTH_FORM, 0u32],
        )
        .u32();
    if health_form == 0 {
        return;
    }
    let extra = extra_data_of(e, me, EXTRA_KIND_HEALTH);
    let own = f64::from(e.vcall(health_form, 0x10, &args![]).u32());
    if extra != 0 {
        if own == f64::from(value) {
            let list = extra_list(e, me);
            e.call(EXTRA_REMOVE, &args![list, extra, 1u32]);
        } else {
            e.mem.set_f32(extra + 0xc, value);
        }
    } else if own != f64::from(value) {
        let list = extra_list(e, me);
        e.call(EXTRA_SET_HEALTH, &args![list, value]);
    }
}

// Translated from 00568cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetRadius` (Xbox PDB): the radius cached in the loaded
/// data (`pLoadedData->fCachedRadius`) when it is not negative; else it is
/// worked out from the base form and cached (when there is loaded data):
/// type 0x1e: the base form's dword at +0xa0 plus the extra's radius
/// (`00422320`); type 0x20: the extra's radius for the form at `011ca264`;
/// for the form at `011ca224` the same, replaced by the setting at
/// `011d0bd8` when it is nearly 0; for the form at `011ca228` replaced by
/// 5000.0 when nearly 0. 0 without a base form (not cached).
pub fn tes_object_refr_get_radius(e: &mut Engine, this: Ptr<TESObjectREFR>) -> f32 {
    let me = this.addr();
    let loaded = e.get(this, TESObjectREFR::pLoadedData).addr();
    if loaded != 0 {
        let cached = e.get(
            Ptr::<LOADED_REF_DATA>::new(loaded),
            LOADED_REF_DATA::fCachedRadius,
        );
        if cached >= 0.0 {
            return cached;
        }
    }
    let mut radius = 0.0f32;
    if e.call(GET_BASE_FORM, &args![me]).u32() == 0 {
        return radius;
    }
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    let kind = e.call(FORM_TYPE, &args![base]).u32();
    if kind == 0x1e {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        radius = e.call(FORM_FIELD_A0, &args![base]).u32() as f32;
        let list = extra_list(e, me);
        let extra = e.call(EXTRA_GET_RADIUS, &args![list]).f32();
        radius = (f64::from(extra) + f64::from(radius)) as f32;
    } else if kind == 0x20 {
        let epsilon: f32 = e.global(RADIUS_EPSILON);
        if e.call(GET_BASE_FORM, &args![me]).u32() == e.global::<u32>(GLOBAL_RADIUS_FORM_A) {
            let list = extra_list(e, me);
            radius = e.call(EXTRA_GET_RADIUS, &args![list]).f32();
        } else if e.call(GET_BASE_FORM, &args![me]).u32() == e.global::<u32>(GLOBAL_RADIUS_FORM_B) {
            let list = extra_list(e, me);
            radius = e.call(EXTRA_GET_RADIUS, &args![list]).f32();
            if e.call(NEARLY_EQUAL, &args![radius, 0.0f32, epsilon]).bool() {
                let setting = e.call(SETTING_VALUE, &args![RADIUS_SETTING]).u32();
                radius = e.mem.i32(setting) as f32;
            }
        } else if e.call(GET_BASE_FORM, &args![me]).u32() == e.global::<u32>(GLOBAL_RADIUS_FORM_C) {
            let list = extra_list(e, me);
            radius = e.call(EXTRA_GET_RADIUS, &args![list]).f32();
            if e.call(NEARLY_EQUAL, &args![radius, 0.0f32, epsilon]).bool() {
                radius = e.global(RADIUS_FAR);
            }
        }
    }
    let loaded = e.get(this, TESObjectREFR::pLoadedData).addr();
    if loaded != 0 {
        e.set(
            Ptr::<LOADED_REF_DATA>::new(loaded),
            LOADED_REF_DATA::fCachedRadius,
            radius,
        );
    }
    radius
}

// Translated from 00568e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the link (`00418460`) of the reference's extra
/// list: the teleport data that `AddTeleport` creates.
pub fn fn_00568e50(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_LINK, &args![list]).u32()
}

// Translated from 00568e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddTeleport` (Xbox PDB): the reference's teleport data
/// (`00568e50`); when it has none a `DoorTeleportData` (0x20 bytes) is
/// created, stored in the list (`00419120`) and the reference marked changed
/// (virtual `+0x48`, 0x20000). The unwinding frame of the exe around the
/// construction is not translated.
pub fn tes_object_refr_add_teleport(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut teleport = fn_00568e50(e, this);
    if teleport == 0 {
        let block = e.call(OPERATOR_NEW, &args![DOOR_TELEPORT_DATA_SIZE]).u32();
        teleport = if block == 0 {
            0
        } else {
            e.call(DOOR_TELEPORT_DATA_CONSTRUCT, &args![block]).u32()
        };
        let list = extra_list(e, me);
        e.call(EXTRA_SET_LINK, &args![list, teleport]);
        e.vcall(me, 0x48, &args![0x20000u32]);
    }
    teleport
}

// Translated from 00568f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RemoveTeleport` (Xbox PDB): removes the teleport pointer
/// (`0041ae90`) of the reference's list; a navmesh portal extra
/// (`0042e2a0`) is told about the reference when its dword at +0xc is set
/// (`004534f0`) and removed (`0042e730`); the reference is marked changed
/// (virtual `+0x4c`, 0x20000).
pub fn tes_object_refr_remove_teleport(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let list = extra_list(e, me);
    e.call(EXTRA_REMOVE_LINK, &args![list]);
    let list = extra_list(e, me);
    let portal = e.call(EXTRA_GET_PORTAL, &args![list]).u32();
    if portal != 0 {
        let target = e.mem.u32(portal + 0xc);
        if target != 0 {
            e.call(PORTAL_TARGET_NOTIFY, &args![target, me]);
        }
        let list = extra_list(e, me);
        e.call(EXTRA_REMOVE_PORTAL, &args![list]);
    }
    e.vcall(me, 0x4c, &args![0x20000u32]);
}

// Translated from 00568fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetLinkedDoorTeleportPosition` (Xbox PDB): when the
/// reference has teleport data, whose first dword (the linked door) has
/// teleport data too, that data's position (`00717e50`, its address + 4);
/// else the record at `011f426c`.
pub fn tes_object_refr_get_linked_door_teleport_position(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> u32 {
    let link = fn_00568e50(e, this);
    if link != 0 {
        let door = e.call(READ_FIRST_DWORD, &args![link]).u32();
        if door != 0 {
            let door_link = fn_00568e50(e, Ptr::new(door));
            if door_link != 0 {
                return e.call(LINKED_DOOR_POSITION, &args![door_link]).u32();
            }
        }
    }
    NO_DOOR_POSITION
}

// ---------------------------------------------------------------------------
// Third batch: the functions from `00569000` to `0056a4a0`.

/// Callees and data of the third batch, by exe address.
///
/// The list getters (`this` is the reference's `ExtraDataList`): two
/// unnamed pointer getters (`00418490`, which the reference's
/// `GetMapMarkerData` returns, and `004184c0`), the lock record pointer of a
/// reference (`00569140`, `ExtraDataList::GetLock`) and the setter of the
/// lock (`00419050`, `ExtraDataList::SetLockPtr`).
const EXTRA_GET_MAP_MARKER_DATA: u32 = 0x0041_8490;
const EXTRA_GET_UNNAMED_B: u32 = 0x0041_84c0;
const LOCK_OF_REFERENCE: u32 = 0x0056_9140;
const EXTRA_SET_LOCK: u32 = 0x0041_9050;
/// The lock extra data constructor (`00411b00`, 0x14 bytes from
/// `operator new`), `REFR_LOCK::GetLevel(lock, reference)` (`00430a10`),
/// `REFR_LOCK::SetLocked(lock, flag)` (`00430a90`), the extra data of a
/// reference by kind (`00527080(reference, kind)`; kind `0x2a` is the lock),
/// the container's unlock body (`00517360`) and
/// `TESObjectREFR::AddLockChange` (`005789b0`).
const LOCK_CONSTRUCT: u32 = 0x0041_1b00;
const LOCK_EXTRA_SIZE: u32 = 0x14;
const LOCK_GET_LEVEL: u32 = 0x0043_0a10;
const LOCK_SET_LOCKED: u32 = 0x0043_0a90;
const EXTRA_OF_REFERENCE: u32 = 0x0052_7080;
const EXTRA_KIND_LOCK: u32 = 0x2a;
const LOCK_EXTRA_UNLOCK: u32 = 0x0051_7360;
const ADD_LOCK_CHANGE: u32 = 0x0057_89b0;
/// The rotation record of a door's teleport data (`00460140`).
const LINKED_DOOR_ROTATION: u32 = 0x0046_0140;
/// The byte of the list that `fn_005692d0` and `fn_00569350` pass on to the
/// base form (`00418b40`, set by `0041ac30`); the form type those two
/// functions require in the base form is `0x25`.
const EXTRA_GET_BYTE: u32 = 0x0041_8b40;
const EXTRA_SET_BYTE: u32 = 0x0041_ac30;
const BASE_FORM_TYPE_WITH_BYTE: u32 = 0x25;
/// `ExtraDataList::GetXTarget` (`00421b40`), and the two forms in globals
/// (`011ca244`, `011ca248`) `fn_005693d0` compares the base form with.
const EXTRA_GET_X_TARGET: u32 = 0x0042_1b40;
const GLOBAL_TARGET_FORM_A: u32 = 0x011c_a244;
const GLOBAL_TARGET_FORM_B: u32 = 0x011c_a248;
/// The emittance helpers: the property type the search asks for
/// (`00438220`, no arguments), `NiAVObject::GetProperty(node, type)`
/// (`00a59d30`), the property kind test (`004a2020(property, kind)`, kind
/// `0x1d`), a node's child count (`0043b480`) and child at an index
/// (`0043b4a0`), the list's emittance flag (`00421d20`) and the message
/// format the check logs (`01030028`).
const EMITTANCE_PROPERTY_TYPE: u32 = 0x0043_8220;
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_IS_KIND: u32 = 0x004a_2020;
const EMITTANCE_PROPERTY_KIND: u32 = 0x1d;
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
const EXTRA_HAS_EMITTANCE: u32 = 0x0042_1d20;
const EMITTANCE_MESSAGE: u32 = 0x0103_0028;
/// Virtual slots of the reference and of 3D objects: `Get3D` (`+0x1d0`),
/// the node cast of a 3D object (`+0xc`, null when it is not a node), a name
/// getter of the reference (`+0x130`, used for the log), the position getter
/// (`+0x1f4`), and the form-changed notification (`+0x4c`).
const SLOT_GET_3D: u32 = 0x1d0;
const SLOT_AS_NODE: u32 = 0xc;
const SLOT_NAME: u32 = 0x130;
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_MARK_CHANGED: u32 = 0x4c;
/// The base form type of the emittance check (`0x1e`).
const EMITTANCE_FORM_TYPE: u32 = 0x1e;
/// `fn_005695a0`: a test of a 3D object (`004b2960`), its world bound
/// (`0043d450`) and the controller slot (`+0x98`) that must answer 2.
const NODE_IS_VALID: u32 = 0x004b_2960;
const NODE_WORLD_BOUND: u32 = 0x0043_d450;
const SLOT_BOUND_TEST: u32 = 0x98;
const BOUND_TEST_INSIDE: u32 = 2;
/// `fn_00569670`: the "is kind of" test (`0045bad0(record, object)`, cdecl,
/// with the record at `011c7d34`), the two casts' records (`011f4acc`,
/// `011f4428`; the cast is `CHECKED_CAST`), and the object `005495f0` gives
/// for the first kind, with its range functions: the cursor constructor
/// (`00534550`), `a67360(range, 1)`, the fill (`00a67410(range, cursor)`),
/// the release (`00a673d0`), the point count (`00456650`, a word), the
/// point reader (`00534570(cursor, index, out)`) and the point transform
/// (`00461130`, then `00524c40(this, out, point)`), the child count of the
/// second kind (`00453470`).
const KIND_TEST: u32 = 0x0045_bad0;
const KIND_RECORD_TEST: u32 = 0x011c_7d34;
const CAST_RECORD_POINTS: u32 = 0x011f_4acc;
const CAST_RECORD_GROUP: u32 = 0x011f_4428;
const POINTS_RANGE_OF: u32 = 0x0054_95f0;
const CURSOR_CONSTRUCT: u32 = 0x0053_4550;
const RANGE_BEGIN: u32 = 0x00a6_7360;
const RANGE_FILL: u32 = 0x00a6_7410;
const RANGE_END: u32 = 0x00a6_73d0;
const POINTS_COUNT: u32 = 0x0045_6650;
const CURSOR_READ_POINT: u32 = 0x0053_4570;
const POINTS_TRANSFORM_OF: u32 = 0x0046_1130;
const TRANSFORM_POINT: u32 = 0x0052_4c40;
const GROUP_CHILD_COUNT: u32 = 0x0045_3470;
const SLOT_VISIT_POINT: u32 = 0x8c;
/// The multi-bound functions: "has multi-bound data" (`00439f90`), the list's
/// two accessors (`00421f00`, the multi-bound reference getter used by
/// `fn_00569840`; `00422020` get and `00422050` set the multi-bound), the
/// creator (`00439490`, cdecl, the reference), the room creator (`004396e0`,
/// cdecl), the list's room functions (`00420810` master flag, `00420ed0`
/// room, `00420f00` set room, `004207e0` the room's linked reference list,
/// `00420c10(list, reference)` links two rooms), the primitive
/// (`0041fbe0`), its centre copy (`00413fc0(primitive, out)`), the half
/// extent address (`00500940`), the vector comparison (`00439090(this,
/// other)`, true when they differ), the vector copy (`0098ddd0(this,
/// source)`) and the default half extent record (`011c3ae4`).
const HAS_MULTI_BOUND_DATA: u32 = 0x0043_9f90;
const EXTRA_GET_MULTI_BOUND_REFERENCE: u32 = 0x0042_1f00;
const EXTRA_GET_MULTI_BOUND: u32 = 0x0042_2020;
const EXTRA_SET_MULTI_BOUND: u32 = 0x0042_2050;
const CREATE_MULTI_BOUND: u32 = 0x0043_9490;
const CREATE_ROOM: u32 = 0x0043_96e0;
const EXTRA_GET_ROOM_IS_MASTER: u32 = 0x0042_0810;
const EXTRA_GET_ROOM: u32 = 0x0042_0ed0;
const EXTRA_SET_ROOM: u32 = 0x0042_0f00;
const EXTRA_GET_ROOM_LINKED: u32 = 0x0042_07e0;
const EXTRA_LINK_ROOM: u32 = 0x0042_0c10;
const EXTRA_GET_PRIMITIVE: u32 = 0x0041_fbe0;
const PRIMITIVE_CENTRE: u32 = 0x0041_3fc0;
const PRIMITIVE_HALF_EXTENT: u32 = 0x0050_0940;
const VECTORS_DIFFER: u32 = 0x0043_9090;
const VECTOR_COPY: u32 = 0x0098_ddd0;
const DEFAULT_HALF_EXTENT: u32 = 0x011c_3ae4;
/// The list functions of the link and activate chains: the list's
/// "linked reference" getter and setter pairs (`00421e10`, `00421e40`;
/// `0041e220`, `0041e250`; `0041e410`, `0041e440`), the linked children list
/// (`0041e500`), the activate parent list (`0041e780`), the activate
/// children list (`0041eda0`), the activate timer (`0041f020` get,
/// `0041eff0` set), the child timer (`0041e9e0(list, reference)`, a float),
/// and the decal list (`0041f050`).
const EXTRA_GET_LINK_A: u32 = 0x0042_1e10;
const EXTRA_SET_LINK_A: u32 = 0x0042_1e40;
const EXTRA_GET_LINK_B: u32 = 0x0041_e220;
const EXTRA_SET_LINK_B: u32 = 0x0041_e250;
const EXTRA_GET_LINK_C: u32 = 0x0041_e410;
const EXTRA_SET_LINK_C: u32 = 0x0041_e440;
const EXTRA_GET_LINKED_CHILDREN: u32 = 0x0041_e500;
const EXTRA_GET_ACTIVATE_PARENTS: u32 = 0x0041_e780;
const EXTRA_GET_ACTIVATE_CHILDREN: u32 = 0x0041_eda0;
const EXTRA_GET_ACTIVATE_TIMER: u32 = 0x0041_f020;
const EXTRA_SET_ACTIVATE_TIMER: u32 = 0x0041_eff0;
const EXTRA_GET_CHILD_TIMER: u32 = 0x0041_e9e0;
const EXTRA_GET_DECALS: u32 = 0x0041_f050;
const EXTRA_SET_THREE_WORDS: u32 = 0x0041_f080;
/// The link drawing: the visited set (`0057c440(set, 0x25)` constructor,
/// `0057c850(set, reference, out byte)` find, `005594e0(set, reference,
/// byte)` insert, `00438af0` remove all, `0057c780` destructor; 0x10
/// bytes), the node names (`"LinkLine"` at `01030088`, `"ActivateLine"` at
/// `01030094`), `NiFixedString` is `FIXED_STRING_CONSTRUCT` /
/// `FIXED_STRING_DESTRUCT`, a node's child by name (virtual `+0x9c`),
/// remove child (`+0xe8`), attach child (`+0xdc(child, flag)`).
const VISITED_CONSTRUCT: u32 = 0x0057_c440;
const VISITED_FIND: u32 = 0x0057_c850;
const VISITED_INSERT: u32 = 0x0055_94e0;
const VISITED_REMOVE_ALL: u32 = 0x0043_8af0;
const VISITED_DESTRUCT: u32 = 0x0057_c780;
const VISITED_SIZE: u32 = 0x10;
const VISITED_BUCKETS: u32 = 0x25;
const LINK_LINE_NAME: u32 = 0x0103_0088;
const ACTIVATE_LINE_NAME: u32 = 0x0103_0094;
const SLOT_FIND_CHILD: u32 = 0x9c;
const SLOT_REMOVE_CHILD: u32 = 0xe8;
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// The activate line: the line's temp effects removal (`004def90`), the
/// vector maths of `NiMatrix3` (`Inverse` is `004b45b0(matrix, out)`; the
/// product with a vector is `004b4500(matrix, out, vector)`), the vector
/// scale (`0045bb20(vector, out, factor)`), the colour constructor
/// (`00414430(colour, r, g, b, a)`), the line maker (`004b3890`, cdecl:
/// start, end colour, end, start colour, 1), the `NiNode`-sized allocation
/// (`00aa13e0(0xac)`, cdecl), its constructor (`00a5ecb0(object, 0)`), the
/// name setter (`00a5b950(object, name)`) and `NiAVObject::UpdateProperties`
/// (`00a5a040`).
const REMOVE_ALL_EFFECTS: u32 = 0x004d_ef90;
const MATRIX_INVERSE: u32 = 0x004b_45b0;
const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
const VECTOR_SCALE: u32 = 0x0045_bb20;
const COLOUR_CONSTRUCT: u32 = 0x0041_4430;
const MAKE_LINE: u32 = 0x004b_3890;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const LINE_NODE_SIZE: u32 = 0xac;
const LINE_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const SET_NODE_NAME: u32 = 0x00a5_b950;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `TESObjectREFR::Activate(reference, activator, 0, 0, 1)` (`00573170`) and
/// `TESObjectCELL::AddActivatingRef(cell, reference)` (`005455c0`), the two
/// forms in globals `fn_0056a290` compares the base form with, and the form
/// flag bit `fn_0056a380` clears through `+0x4c`.
const ACTIVATE_REFERENCE: u32 = 0x0057_3170;
const CELL_ADD_ACTIVATING_REF: u32 = 0x0054_55c0;
const GLOBAL_ACTIVATOR_FORM_A: u32 = 0x011c_a27c;
const GLOBAL_ACTIVATOR_FORM_B: u32 = 0x011c_a280;
const ACTIVATING_CHANGED_FLAG: u32 = 0x0400_0000;
/// The decals: `fn_0056a8d0` (refresh), the decal data constructor
/// (`004a37b0`, a 0x80 byte record), the smart pointer functions
/// (`00633c90(pointer, 0)` constructor, `0045cec0` destructor), the
/// "is this a worn kind" test (`0056a860`) and the helper that gives the
/// holder object (`0056a880`), the random float (`00476b70(a, b)`, cdecl),
/// the holder's float getters (`006b9130`, `006a7f50`, `00488d50`,
/// `00621b00`, `0047c860`), its flag getters (`004a41a0`, `004a4160`,
/// `004a4120`, `004a4200`) and distance (`004a7bd0`), the colour maker
/// (`00416870(out, r, g, b)`) and the decal manager entry (`004a3fe0(cell,
/// data, 4, 0)`).
const DECAL_REFRESH: u32 = 0x0056_a8d0;
const DECAL_DATA_CONSTRUCT: u32 = 0x004a_37b0;
const DECAL_DATA_SIZE: u32 = 0x80;
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const DECAL_HAS_HOLDER: u32 = 0x0056_a860;
const DECAL_HOLDER: u32 = 0x0056_a880;
const RANDOM_FLOAT: u32 = 0x0047_6b70;
const HOLDER_FLOAT_A: u32 = 0x006b_9130;
const HOLDER_FLOAT_B: u32 = 0x006a_7f50;
const HOLDER_FLOAT_C: u32 = 0x0048_8d50;
const HOLDER_FLOAT_D: u32 = 0x0062_1b00;
const HOLDER_FLOAT_E: u32 = 0x0047_c860;
const HOLDER_FLOAT_F: u32 = 0x0084_d030;
const HOLDER_FLAG_A: u32 = 0x004a_41a0;
const HOLDER_FLAG_B: u32 = 0x004a_4160;
const HOLDER_FLAG_C: u32 = 0x004a_4120;
const HOLDER_FLAG_D: u32 = 0x004a_4200;
const HOLDER_DISTANCE: u32 = 0x004a_7bd0;
const MAKE_COLOUR: u32 = 0x0041_6870;
const DECAL_MANAGER_ADD: u32 = 0x004a_3fe0;
const BYTE_DIVISOR: u32 = 0x0101_e568;
const DECAL_FLOAT_40_DEFAULT: u32 = 0x0101_e574;
const DECAL_FLOAT_58_SOURCE: u32 = 0x0102_49d8;
const DECAL_SIZE_FACTOR: u32 = 0x0102_e430;
/// Offsets inside the decal data record the decal function fills (the
/// constructor writes the rest).
const DECAL_FIRST_VECTOR: u32 = 0x00;
const DECAL_SECOND_VECTOR: u32 = 0x0c;
const DECAL_NODE: u32 = 0x28;
const DECAL_BASE_FORM: u32 = 0x30;
const DECAL_FLOAT_38: u32 = 0x38;
const DECAL_FLOAT_3C: u32 = 0x3c;
const DECAL_FLOAT_40: u32 = 0x40;
const DECAL_FLOAT_58: u32 = 0x58;
const DECAL_FLOAT_4C: u32 = 0x4c;
const DECAL_FLOAT_54: u32 = 0x54;
const DECAL_COLOUR: u32 = 0x60;
const DECAL_BYTE_70: u32 = 0x70;
const DECAL_BYTE_72: u32 = 0x72;
const DECAL_BYTE_73: u32 = 0x73;
const DECAL_FLAG_A: u32 = 0x74;
const DECAL_FLAG_B: u32 = 0x75;
const DECAL_FLAG_D: u32 = 0x76;
/// The reference's decal list entries: the position (+4) and rotation
/// (+0x10) of a decal record, whose first dword is the decal reference.
const DECAL_RECORD_POSITION: u32 = 0x04;
const DECAL_RECORD_ROTATION: u32 = 0x10;

/// Copies the three dwords at `from` to `to`.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 00569000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetLinkedDoorTeleportRotation` (Xbox PDB): as the
/// position, but the rotation record (`00460140`) of the linked door's
/// teleport data; else the record at `011f426c`.
pub fn tes_object_refr_get_linked_door_teleport_rotation(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> u32 {
    let link = fn_00568e50(e, this);
    if link != 0 {
        let door = e.call(READ_FIRST_DWORD, &args![link]).u32();
        if door != 0 {
            let door_link = fn_00568e50(e, Ptr::new(door));
            if door_link != 0 {
                return e.call(LINKED_DOOR_ROTATION, &args![door_link]).u32();
            }
        }
    }
    ZERO_VECTOR
}

// Translated from 00569060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMapMarkerData` (Xbox PDB): the list's map marker data
/// (`00418490`).
pub fn tes_object_refr_get_map_marker_data(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_MAP_MARKER_DATA, &args![list]).u32()
}

// Translated from 00569080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the sibling of `GetMapMarkerData`, the list's
/// pointer from the unnamed getter `004184c0`.
pub fn fn_00569080(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_UNNAMED_B, &args![list]).u32()
}

// Translated from 005690a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddLock` (Xbox PDB): the reference's lock
/// (`00569140`); when it has none a lock extra data (0x14 bytes, constructor
/// `00411b00`) is created and stored in the list (`00419050`). A failed
/// allocation is carried on as a null lock, as the code does; the unwinding
/// frame is not translated.
pub fn tes_object_refr_add_lock(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut lock = e.call(LOCK_OF_REFERENCE, &args![me]).u32();
    if lock == 0 {
        let block = e.call(OPERATOR_NEW, &args![LOCK_EXTRA_SIZE]).u32();
        lock = if block == 0 {
            0
        } else {
            e.call(LOCK_CONSTRUCT, &args![block]).u32()
        };
        let list = extra_list(e, me);
        e.call(EXTRA_SET_LOCK, &args![list, lock]);
    }
    lock
}

// Translated from 00569160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetLock` (Xbox PDB): the reference's lock (`00569140`);
/// without one, the lock of the door its teleport data links to.
pub fn tes_object_refr_get_lock(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let mut lock = e.call(LOCK_OF_REFERENCE, &args![this.addr()]).u32();
    if lock == 0 {
        let link = fn_00568e50(e, this);
        if link != 0 && e.call(READ_FIRST_DWORD, &args![link]).u32() != 0 {
            let door = e.call(READ_FIRST_DWORD, &args![link]).u32();
            lock = e.call(LOCK_OF_REFERENCE, &args![door]).u32();
        }
    }
    lock
}

// Translated from 005691b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetLockLevel` (Xbox PDB): 0 without a lock, else
/// `REFR_LOCK::GetLevel` (`00430a10`) of it for this reference.
pub fn tes_object_refr_get_lock_level(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let lock = tes_object_refr_get_lock(e, this);
    if lock == 0 {
        return 0;
    }
    e.call(LOCK_GET_LEVEL, &args![lock, this.addr()]).u32()
}

// Translated from 005691f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetExtraLock` (Xbox PDB): the reference's lock extra data
/// (kind `0x2a`, `00527080`); without one, that of the door its teleport
/// data links to (recursively).
pub fn tes_object_refr_get_extra_lock(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let mut lock = e
        .call(EXTRA_OF_REFERENCE, &args![this.addr(), EXTRA_KIND_LOCK])
        .u32();
    if lock == 0 {
        let link = fn_00568e50(e, this);
        if link != 0 && e.call(READ_FIRST_DWORD, &args![link]).u32() != 0 {
            let door = e.call(READ_FIRST_DWORD, &args![link]).u32();
            lock = tes_object_refr_get_extra_lock(e, Ptr::new(door));
        }
    }
    lock
}

// Translated from 00569250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Lock` (Xbox PDB): sets the lock extra data locked
/// (`00569280`) and records the change (`AddLockChange`, `005789b0`); does
/// nothing without a lock.
pub fn tes_object_refr_lock(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let lock = tes_object_refr_get_extra_lock(e, this);
    if lock != 0 {
        fn_00569280(e, Ptr::new(lock));
        e.call(ADD_LOCK_CHANGE, &args![this.addr()]);
    }
}

// Translated from 00569280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: `REFR_LOCK::SetLocked(true)` (`00430a90`) on
/// the lock record the lock extra data holds at +0xc.
pub fn fn_00569280(e: &mut Engine, this: Ptr) {
    let record = e.mem.u32(this.addr() + 0xc);
    e.call(LOCK_SET_LOCKED, &args![record, 1u32]);
}

// Translated from 005692a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::UnLock` (Xbox PDB): unlocks the lock extra data
/// (`00517360`) and records the change; does nothing without a lock.
pub fn tes_object_refr_un_lock(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let lock = tes_object_refr_get_extra_lock(e, this);
    if lock != 0 {
        e.call(LOCK_EXTRA_UNLOCK, &args![lock]);
        e.call(ADD_LOCK_CHANGE, &args![this.addr()]);
    }
}

// Translated from 005692d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: when the base form is of type `0x25`, its
/// virtual `+0x188` called with the list's byte (`00418b40`); else 0.
pub fn fn_005692d0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut result = 0;
    if base_type(e, me) == Some(BASE_FORM_TYPE_WITH_BYTE) {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        if base != 0 {
            let list = extra_list(e, me);
            let value = e.call(EXTRA_GET_BYTE, &args![list]).u8();
            result = e.vcall(base, 0x188, &args![value]).u32();
        }
    }
    result
}

// Translated from 00569350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the setter of the above. With a list, and a
/// base form that has none or is of type `0x25`, the base form's virtual
/// `+0x184` is called with `value` and its byte result is stored in the
/// list (`0041ac30`).
pub fn fn_00569350(e: &mut Engine, this: Ptr<TESObjectREFR>, value: u32) {
    let me = this.addr();
    if extra_list(e, me) == 0 {
        return;
    }
    if let Some(kind) = base_type(e, me) {
        if kind != BASE_FORM_TYPE_WITH_BYTE {
            return;
        }
    }
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    if base != 0 {
        let stored = e.vcall(base, 0x184, &args![value]).u8();
        let list = extra_list(e, me);
        e.call(EXTRA_SET_BYTE, &args![list, stored]);
    }
}

// Translated from 005693d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: when the base form is one of the two forms in
/// the globals `011ca244` and `011ca248`, the list's `GetXTarget`
/// (`00421b40`); else 0.
pub fn fn_005693d0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let first = e.call(GET_BASE_FORM, &args![me]).u32();
    if first != e.global::<u32>(GLOBAL_TARGET_FORM_A) {
        let second = e.call(GET_BASE_FORM, &args![me]).u32();
        if second != e.global::<u32>(GLOBAL_TARGET_FORM_B) {
            return 0;
        }
    }
    let list = extra_list(e, me);
    e.call(EXTRA_GET_X_TARGET, &args![list]).u32()
}

// Translated from 00569410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FindEmittanceProperty` (Xbox PDB), cdecl: whether the 3D object has the
/// emittance property (the property of the type `00438220` gives, of kind
/// `0x1d`), or any node below it does (recursively through the children of
/// the node cast, virtual `+0xc`).
pub fn find_emittance_property(e: &mut Engine, node: Ptr) -> bool {
    let node = node.addr();
    if node == 0 {
        return false;
    }
    let property_type = e.call(EMITTANCE_PROPERTY_TYPE, &args![]).u32();
    let property = e.call(NODE_GET_PROPERTY, &args![node, property_type]).u32();
    if property != 0
        && e.call(PROPERTY_IS_KIND, &args![property, EMITTANCE_PROPERTY_KIND])
            .bool()
    {
        return true;
    }
    let as_node = e.vcall(node, SLOT_AS_NODE, &args![]).u32();
    if as_node != 0 {
        let count = e.call(NODE_CHILD_COUNT, &args![as_node]).u32();
        for index in 0..count {
            let child = e.call(NODE_CHILD_AT, &args![as_node, index]).u32();
            if find_emittance_property(e, Ptr::new(child)) {
                return true;
            }
        }
    }
    false
}

// Translated from 005694c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::CheckUsesExternalEmittance` (Xbox PDB): false without a
/// base form; true for a base form of type `0x1e` whose list has emittance
/// data (`00569580`); otherwise whether the loaded 3D (virtual `+0x1d0`)
/// supports emittance (`FindEmittanceProperty`). When it does not but the
/// reference has explicit emittance data, the mismatch is logged with the
/// name (virtual `+0x130`) and form ID.
pub fn tes_object_refr_check_uses_external_emittance(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> bool {
    let me = this.addr();
    let Some(kind) = base_type(e, me) else {
        return false;
    };
    if kind == EMITTANCE_FORM_TYPE && fn_00569580(e, this) != 0 {
        return true;
    }
    let mut found = false;
    if e.vcall(me, SLOT_GET_3D, &args![]).u32() != 0 {
        let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
        found = find_emittance_property(e, Ptr::new(node));
        if !found && fn_00569580(e, this) != 0 {
            let id = e.call(FORM_ID, &args![me]).u32();
            let name = e.vcall(me, SLOT_NAME, &args![]).u32();
            e.call(LOG_MESSAGE, &args![EMITTANCE_MESSAGE, name, id]);
        }
    }
    found
}

// Translated from 00569580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the list's emittance flag (`00421d20`).
pub fn fn_00569580(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_HAS_EMITTANCE, &args![list]).u32()
}

// Translated from 005695a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: whether the loaded 3D passes `004b2960`, is
/// not null, and the virtual `+0x98` of `other`'s controller (`0043b230`)
/// answers 2 for the 3D's world bound (`0043d450`).
pub fn fn_005695a0(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr) -> bool {
    let node = e.vcall(this.addr(), SLOT_GET_3D, &args![]).u32();
    if e.call(NODE_IS_VALID, &args![node]).u32() == 0 || node == 0 {
        return false;
    }
    let controller = e.call(GET_CONTROLLER, &args![other.addr()]).u32();
    let bound = e.call(NODE_WORLD_BOUND, &args![node]).u32();
    e.vcall(controller, SLOT_BOUND_TEST, &args![bound]).u32() == BOUND_TEST_INSIDE
}

// Translated from 00569620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: copies the position (virtual `+0x1f4`) of the
/// reference to a local and hands it to `other`'s virtual `+0x8c`, answering
/// that virtual's value (the exe ends on that call and leaves its `EAX`; the
/// caller tests `AL`).
pub fn fn_00569620(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr) -> u32 {
    let position = e.vcall(this.addr(), SLOT_POSITION, &args![]).u32();
    e.with_stack(0xc, |e, copy| {
        copy_vector(e, position, copy.addr());
        e.vcall(other.addr(), SLOT_VISIT_POINT, &args![copy.addr()])
            .u32()
    })
}

// Translated from 00569670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB, cdecl: walks the object `object`: when it is
/// of the kind `011c7d34` the byte at `found` is the result; when it casts
/// to the points kind (record `011f4acc`), `*found = 1` and each of its
/// points, transformed (`00461130`, `00524c40`), is handed to the visitor's
/// virtual `+0x8c` (a point it refuses ends the walk with 0 when `stop` is
/// set); when it casts to the group kind (`011f4428`) every child is walked
/// the same way and a 0 from one ends it. The result is `*found`. The
/// counter of accepted points the exe keeps is never read and not kept.
pub fn fn_00569670(e: &mut Engine, object: Ptr, visitor: Ptr, found: Ptr, stop: u8) -> u8 {
    let (object, visitor, found) = (object.addr(), visitor.addr(), found.addr());
    if e.call(KIND_TEST, &args![KIND_RECORD_TEST, object]).bool() {
        return e.mem.u8(found);
    }
    let points = e
        .call(CHECKED_CAST, &args![CAST_RECORD_POINTS, object])
        .u32();
    if points != 0 {
        e.mem.set_u8(found, 1);
        let range = e.call(POINTS_RANGE_OF, &args![points]).u32();
        let early = e.with_stack(0x30, |e, block| -> Option<u8> {
            let cursor = block.addr();
            let point = cursor + 0x10;
            let transformed = cursor + 0x20;
            e.call(CURSOR_CONSTRUCT, &args![cursor]);
            if !e.call(RANGE_BEGIN, &args![range, 1u32]).bool() {
                return Some(0);
            }
            e.call(RANGE_FILL, &args![range, cursor]);
            let mut index = 0i32;
            while index < i32::from(e.call(POINTS_COUNT, &args![points]).u16()) {
                copy_vector(e, ZERO_VECTOR, point);
                e.call(CURSOR_READ_POINT, &args![cursor, index, point]);
                let transform = e.call(POINTS_TRANSFORM_OF, &args![points]).u32();
                let result = e
                    .call(TRANSFORM_POINT, &args![transform, transformed, point])
                    .u32();
                copy_vector(e, result, point);
                if !e.vcall(visitor, SLOT_VISIT_POINT, &args![point]).bool() && stop != 0 {
                    e.call(RANGE_END, &args![range]);
                    return Some(0);
                }
                index += 1;
            }
            e.call(RANGE_END, &args![range]);
            if stop == 0 {
                return Some(0);
            }
            None
        });
        if let Some(result) = early {
            return result;
        }
    }
    let group = e
        .call(CHECKED_CAST, &args![CAST_RECORD_GROUP, object])
        .u32();
    if group != 0 {
        let mut index = 0u32;
        while index < e.call(GROUP_CHILD_COUNT, &args![group]).u32() {
            let child = e.call(NODE_CHILD_AT, &args![group, index]).u32();
            if fn_00569670(e, Ptr::new(child), Ptr::new(visitor), Ptr::new(found), stop) == 0 {
                return 0;
            }
            index += 1;
        }
    }
    e.mem.u8(found)
}

// Translated from 00569840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: for a reference with multi-bound data
/// (`00439f90`), the list's multi-bound reference (`00421f00`); else 0.
pub fn fn_00569840(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    if !e.call(HAS_MULTI_BOUND_DATA, &args![me]).bool() {
        return 0;
    }
    let list = extra_list(e, me);
    e.call(EXTRA_GET_MULTI_BOUND_REFERENCE, &args![list]).u32()
}

// Translated from 00569880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMultiBoundHalfExtent` (Xbox PDB): the address of the
/// half extent vector. With a primitive in the list that is the primitive's
/// (`00500940`), after the multi-bound reference (`00569840`), when it
/// exists and its vector differs from the primitive's centre, is given the
/// centre; without a primitive it is the multi-bound reference's own record,
/// and without both the default record at `011c3ae4`.
pub fn tes_object_refr_get_multi_bound_half_extent(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> u32 {
    let me = this.addr();
    let bound = fn_00569840(e, this);
    let list = extra_list(e, me);
    let primitive = e.call(EXTRA_GET_PRIMITIVE, &args![list]).u32();
    if primitive == 0 {
        if bound != 0 {
            return e.call(LIST_ITEM_SLOT, &args![bound]).u32();
        }
        return fn_00569910(e);
    }
    if bound != 0 {
        e.with_stack(0x18, |e, buffers| {
            let first = e
                .call(PRIMITIVE_CENTRE, &args![primitive, buffers.addr()])
                .u32();
            let slot = e.call(LIST_ITEM_SLOT, &args![bound]).u32();
            if e.call(VECTORS_DIFFER, &args![slot, first]).bool() {
                let second = e
                    .call(PRIMITIVE_CENTRE, &args![primitive, buffers.addr() + 0xc])
                    .u32();
                e.call(VECTOR_COPY, &args![bound, second]);
            }
        });
    }
    e.call(PRIMITIVE_HALF_EXTENT, &args![primitive]).u32()
}

// Translated from 00569910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the address of the default half extent record
/// (`011c3ae4`).
pub fn fn_00569910(_e: &mut Engine) -> u32 {
    DEFAULT_HALF_EXTENT
}

// Translated from 00569920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMultiBound` (Xbox PDB): null for a reference without
/// multi-bound data (`00439f90`) or a deleted one; otherwise the list's
/// multi-bound (`00422020`), created for the reference
/// (`MultiBoundMarkerData::CreateMultiBoundForRef`, `00439490`) and stored
/// (`00422050`) when it has none.
pub fn tes_object_refr_get_multi_bound(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    if !e.call(HAS_MULTI_BOUND_DATA, &args![me]).bool() {
        return 0;
    }
    if e.call(FORM_IS_DELETED, &args![me]).bool() {
        return 0;
    }
    let list = extra_list(e, me);
    let mut bound = e.call(EXTRA_GET_MULTI_BOUND, &args![list]).u32();
    if bound == 0 {
        bound = e.call(CREATE_MULTI_BOUND, &args![me]).u32();
        let list = extra_list(e, me);
        e.call(EXTRA_SET_MULTI_BOUND, &args![list, bound]);
    }
    bound
}

// Translated from 00569990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB (the map's name for this address belongs to
/// folded library code): clears the list's multi-bound (`00422050(list,
/// 0)`).
pub fn fn_00569990(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_SET_MULTI_BOUND, &args![list, 0u32]);
}

// Translated from 005699b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMultiBoundRoom` (Xbox PDB): null for a reference
/// without multi-bound data or a deleted one, or whose list is not a room
/// master (`00420810`); otherwise the list's room (`00420ed0`). A room that
/// does not exist yet is created (`004396e0`), stored (`00420f00`) and
/// linked both ways (`00420c10`) with every reference of the room's linked
/// reference list (`004207e0`, nodes from +8) that is not null.
pub fn tes_object_refr_get_multi_bound_room(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    if !e.call(HAS_MULTI_BOUND_DATA, &args![me]).bool() {
        return 0;
    }
    if e.call(FORM_IS_DELETED, &args![me]).bool() {
        return 0;
    }
    let list = extra_list(e, me);
    if !e.call(EXTRA_GET_ROOM_IS_MASTER, &args![list]).bool() {
        return 0;
    }
    let list = extra_list(e, me);
    let mut room = e.call(EXTRA_GET_ROOM, &args![list]).u32();
    if room == 0 {
        room = e.call(CREATE_ROOM, &args![me]).u32();
        let list = extra_list(e, me);
        e.call(EXTRA_SET_ROOM, &args![list, room]);
        let list = extra_list(e, me);
        let linked = e.call(EXTRA_GET_ROOM_LINKED, &args![list]).u32();
        if linked != 0 {
            let mut node = linked + 8;
            while node != 0 {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let other = e.mem.u32(slot);
                let other_list = extra_list(e, other);
                e.call(EXTRA_LINK_ROOM, &args![other_list, me]);
                let own_list = extra_list(e, me);
                e.call(EXTRA_LINK_ROOM, &args![own_list, other]);
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
        }
    }
    room
}

// Translated from 00569ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the first linked reference getter of the list
/// (`00421e10`).
pub fn fn_00569ac0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_LINK_A, &args![list]).u32()
}

// Translated from 00569ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: stores `other` with the list's setter
/// (`00421e40`) unless it is this reference, or is not null and has no
/// multi-bound data (`00439f90`).
pub fn fn_00569ae0(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr<TESObjectREFR>) {
    let (me, other) = (this.addr(), other.addr());
    if other != 0 && !e.call(HAS_MULTI_BOUND_DATA, &args![other]).bool() {
        return;
    }
    if me != other {
        let list = extra_list(e, me);
        e.call(EXTRA_SET_LINK_A, &args![list, other]);
    }
}

// Translated from 00569b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the second linked reference getter of the list
/// (`0041e220`).
pub fn fn_00569b20(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_LINK_B, &args![list]).u32()
}

// Translated from 00569b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: stores `other` with the list's setter
/// (`0041e250`) unless it is not null and does not persist.
pub fn fn_00569b40(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr<TESObjectREFR>) {
    if other.addr() != 0 && !tes_object_refr_get_ref_persists(e, other) {
        return;
    }
    let list = extra_list(e, this.addr());
    e.call(EXTRA_SET_LINK_B, &args![list, other.addr()]);
}

// Translated from 00569b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: the third linked reference getter of the list
/// (`0041e410`): the reference the link line goes to.
pub fn fn_00569b80(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(EXTRA_GET_LINK_C, &args![list]).u32()
}

// Translated from 00569ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: stores `other` with the list's setter
/// (`0041e440`) unless it is not null and does not persist.
pub fn fn_00569ba0(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr<TESObjectREFR>) {
    if other.addr() != 0 && !tes_object_refr_get_ref_persists(e, other) {
        return;
    }
    let list = extra_list(e, this.addr());
    e.call(EXTRA_SET_LINK_C, &args![list, other.addr()]);
}

// Translated from 00569be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::DrawRefLink` (Xbox PDB): removes the `"LinkLine"` child
/// (virtual `+0x9c` by `NiFixedString`, removal `+0xe8`) from the node cast
/// (virtual `+0xc`) of the loaded 3D (virtual `+0x1d0`). The byte the callers
/// pass is never read. The unwinding frame is not translated.
pub fn tes_object_refr_draw_ref_link(e: &mut Engine, this: Ptr<TESObjectREFR>, _unused_1: u8) {
    let node = e.vcall(this.addr(), SLOT_GET_3D, &args![]).u32();
    if node == 0 {
        return;
    }
    let as_node = e.vcall(node, SLOT_AS_NODE, &args![]).u32();
    if as_node == 0 {
        return;
    }
    let child = e.with_stack(4, |e, fixed| {
        let name = e
            .call(FIXED_STRING_CONSTRUCT, &args![fixed, LINK_LINE_NAME])
            .u32();
        let child = e.vcall(as_node, SLOT_FIND_CHILD, &args![name]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        child
    });
    if child != 0 {
        e.vcall(as_node, SLOT_REMOVE_CHILD, &args![child]);
    }
}

// Translated from 00569cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RedrawRefLinks` (Xbox PDB): a reference already in the
/// visited set (`visited`, or a set of its own when null) is skipped;
/// otherwise it is added (`005594e0`, with the byte the failed lookup left
/// in the exe's stack slot; zero here), its link line is removed
/// (`DrawRefLink`) and the walk goes on through the reference its list
/// links to (`00569b80`) and the linked children (`0041e500`, nodes until an
/// empty one). The set is emptied and destroyed at the end. The unwinding
/// frame is not translated.
pub fn tes_object_refr_redraw_ref_links(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    flag: u8,
    visited: Ptr,
) {
    let me = this.addr();
    e.with_stack(VISITED_SIZE + 4, |e, block| {
        let own = block.addr();
        let byte = own + VISITED_SIZE;
        e.call(VISITED_CONSTRUCT, &args![own, VISITED_BUCKETS]);
        let visited = if visited.addr() == 0 {
            own
        } else {
            visited.addr()
        };
        if e.call(VISITED_FIND, &args![visited, me, byte]).bool() {
            e.call(VISITED_DESTRUCT, &args![own]);
            return;
        }
        let stored = e.mem.u8(byte);
        e.call(VISITED_INSERT, &args![visited, me, stored]);
        let target = fn_00569b80(e, this);
        if target != 0 {
            tes_object_refr_draw_ref_link(e, this, flag);
            tes_object_refr_redraw_ref_links(e, Ptr::new(target), flag, Ptr::new(visited));
        }
        let list = extra_list(e, me);
        let mut node = e.call(EXTRA_GET_LINKED_CHILDREN, &args![list]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let child = e.mem.u32(slot);
            tes_object_refr_draw_ref_link(e, Ptr::new(child), flag);
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let child = e.mem.u32(slot);
            tes_object_refr_redraw_ref_links(e, Ptr::new(child), flag, Ptr::new(visited));
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        e.call(VISITED_REMOVE_ALL, &args![own]);
        e.call(VISITED_DESTRUCT, &args![own]);
    });
}

// Translated from 00569df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::DrawActivateRefLink` (Xbox PDB): on the node cast of the
/// loaded 3D, looks up the `"ActivateLine"` child; an existing one has its
/// effects removed (`004def90`) and, when `draw` is 0, is removed from the
/// node. With `draw` set, for every reference in the activate parent list
/// (`0041e780`) a line from this reference to it is made: the offset of the
/// positions (`00439ef0`) is taken into this reference's frame (inverse of
/// its orientation, then divided by its scale), and `004b3890` makes the
/// line geometry (black start colour, white end colour); a missing child
/// node is created (0xac bytes, `00a5ecb0`, named `"ActivateLine"`) and
/// attached once, and each line is attached to it. When anything was drawn
/// the node's properties are updated (`00a5a040`) and given a velocity
/// record of 0.0. The unwinding frames are not translated.
pub fn tes_object_refr_draw_activate_ref_link(e: &mut Engine, this: Ptr<TESObjectREFR>, draw: u8) {
    let me = this.addr();
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    if node == 0 {
        return;
    }
    let as_node = e.vcall(node, SLOT_AS_NODE, &args![]).u32();
    if as_node == 0 {
        return;
    }
    let mut drawn = false;
    let line = e.with_stack(4, |e, fixed| {
        let name = e
            .call(FIXED_STRING_CONSTRUCT, &args![fixed, ACTIVATE_LINE_NAME])
            .u32();
        let line = e.vcall(as_node, SLOT_FIND_CHILD, &args![name]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        line
    });
    let mut line_node = if line == 0 {
        0
    } else {
        e.vcall(line, SLOT_AS_NODE, &args![]).u32()
    };
    if line_node != 0 {
        e.call(REMOVE_ALL_EFFECTS, &args![line_node]);
        if draw == 0 {
            e.vcall(as_node, SLOT_REMOVE_CHILD, &args![line_node]);
            line_node = 0;
        }
    }
    if draw != 0 {
        let list = extra_list(e, me);
        let mut entry = e.call(EXTRA_GET_ACTIVATE_PARENTS, &args![list]).u32();
        while entry != 0 && !e.call(LIST_IS_EMPTY, &args![entry]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![entry]).u32();
            let record = e.mem.u32(slot);
            let target = e.mem.u32(record);
            entry = e.call(LIST_NEXT, &args![entry]).u32();
            // The locals of the exe's frame, in one block: the offset vector,
            // the orientation and its inverse, two vector results, the two
            // colours, the zero vector and the velocity record.
            let made = e.with_stack(0xe0, |e, block| {
                let block = block.addr();
                let offset = block;
                let orientation = block + 0x10;
                let inverse = block + 0x40;
                let result_a = block + 0x70;
                let result_b = block + 0x80;
                let colour_a = block + 0x90;
                let colour_b = block + 0xa0;
                let zero = block + 0xb0;
                copy_vector(e, ZERO_VECTOR, zero);
                let own_position = e.vcall(me, SLOT_POSITION, &args![]).u32();
                let target_position = e.vcall(target, SLOT_POSITION, &args![]).u32();
                e.call(
                    VECTOR_DIFFERENCE,
                    &args![target_position, offset, own_position],
                );
                e.call(GET_ORIENTATION, &args![me, orientation]);
                e.call(MATRIX_INVERSE, &args![orientation, inverse]);
                let rotated = e
                    .call(MATRIX_TIMES_VECTOR, &args![inverse, result_a, offset])
                    .u32();
                copy_vector(e, rotated, offset);
                let scale = tes_object_refr_get_scale(e, this);
                let factor = (1.0f64 / f64::from(scale)) as f32;
                let scaled = e.call(VECTOR_SCALE, &args![offset, result_b, factor]).u32();
                copy_vector(e, scaled, offset);
                let start = e
                    .call(
                        COLOUR_CONSTRUCT,
                        &args![colour_a, 0.0f32, 0.0f32, 0.0f32, 1.0f32],
                    )
                    .u32();
                let end = e
                    .call(
                        COLOUR_CONSTRUCT,
                        &args![colour_b, 1.0f32, 1.0f32, 1.0f32, 1.0f32],
                    )
                    .u32();
                e.call(MAKE_LINE, &args![zero, end, offset, start, 1u32])
                    .u32()
            });
            if line_node == 0 {
                let block = e.call(NI_ALLOCATE, &args![LINE_NODE_SIZE]).u32();
                line_node = if block == 0 {
                    0
                } else {
                    e.call(LINE_NODE_CONSTRUCT, &args![block, 0u32]).u32()
                };
                e.with_stack(4, |e, fixed| {
                    let name = e
                        .call(FIXED_STRING_CONSTRUCT, &args![fixed, ACTIVATE_LINE_NAME])
                        .u32();
                    e.call(SET_NODE_NAME, &args![line_node, name]);
                    e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
                });
                e.vcall(as_node, SLOT_ATTACH_CHILD, &args![line_node, 1u32]);
            }
            e.vcall(line_node, SLOT_ATTACH_CHILD, &args![made, 1u32]);
            drawn = true;
        }
    }
    if drawn {
        e.call(UPDATE_PROPERTIES, &args![as_node]);
        set_velocity(e, as_node, 0.0, 0, 0);
    }
}

// Translated from 0056a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RedrawActivateRefLinks` (Xbox PDB): draws this
/// reference's activate links and those of each of its activate children
/// (`0041eda0`, nodes until an empty one).
pub fn tes_object_refr_redraw_activate_ref_links(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    draw: u8,
) {
    tes_object_refr_draw_activate_ref_link(e, this, draw);
    let list = extra_list(e, this.addr());
    let mut node = e.call(EXTRA_GET_ACTIVATE_CHILDREN, &args![list]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let record = e.mem.u32(slot);
        let child = e.mem.u32(record);
        tes_object_refr_draw_activate_ref_link(e, Ptr::new(child), draw);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 0056a250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::IsActivatingChildren` (Xbox PDB): whether the list's
/// activate children timer (`0041f020`) is greater than 0.0 (the `double` at
/// `01012060`); a NaN is not.
pub fn tes_object_refr_is_activating_children(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let list = extra_list(e, this.addr());
    let timer = e.call(EXTRA_GET_ACTIVATE_TIMER, &args![list]).f32();
    let zero: f64 = e.global(ZERO_DOUBLE);
    f64::from(timer) > zero
}

// Translated from 0056a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: for every activate child record (`0041eda0`)
/// the child's timer for this reference (`0041e9e0`) is stored at +4 of the
/// record; a timer of 0.0 or less (a child that is due) activates the child
/// (`TESObjectREFR::Activate(child, activator, 0, 0, 1)`; the activator is
/// `activator` when this reference's base form is one of the two forms in
/// `011ca27c`/`011ca280`, else this reference); a greater (or NaN) timer
/// leaves the child waiting, and then the parent cell is told to keep this
/// reference activating (`005455c0`).
pub fn fn_0056a290(e: &mut Engine, this: Ptr<TESObjectREFR>, activator: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let mut waiting = false;
    let list = extra_list(e, me);
    let mut node = e.call(EXTRA_GET_ACTIVATE_CHILDREN, &args![list]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let record = e.mem.u32(slot);
        let child = e.mem.u32(record);
        let child_list = extra_list(e, child);
        let timer = e.call(EXTRA_GET_CHILD_TIMER, &args![child_list, me]).f32();
        e.mem.set_f32(record + 4, timer);
        let timer = f64::from(e.mem.f32(record + 4));
        let zero: f64 = e.global(ZERO_DOUBLE);
        if timer <= zero {
            let first = e.call(GET_BASE_FORM, &args![me]).u32();
            let by_activator = first == e.global::<u32>(GLOBAL_ACTIVATOR_FORM_A) || {
                let second = e.call(GET_BASE_FORM, &args![me]).u32();
                second == e.global::<u32>(GLOBAL_ACTIVATOR_FORM_B)
            };
            let who = if by_activator { activator.addr() } else { me };
            e.call(ACTIVATE_REFERENCE, &args![child, who, 0u32, 0u32, 1u32]);
        } else {
            waiting = true;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if waiting {
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        e.call(CELL_ADD_ACTIVATING_REF, &args![cell, me]);
    }
}

// Translated from 0056a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::UpdateChildActivates` (Xbox PDB): advances the activate
/// children timer by the frame time (`0084d030` on `011f6394`). Each child
/// record (`0041eda0`) whose time (+4) is after the old timer is activated
/// (`Activate(child, this, 0, 0, 1)`) when it is not after the new timer,
/// else the update stays pending. When nothing is pending the reference is
/// told (virtual `+0x4c`, `0x4000000`) and the timer restarts from 0.0; the
/// timer is stored (`0041eff0`). Returns whether children are still pending.
pub fn tes_object_refr_update_child_activates(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let mut pending = false;
    let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_OBJECT]).f32();
    let list = extra_list(e, me);
    let mut node = e.call(EXTRA_GET_ACTIVATE_CHILDREN, &args![list]).u32();
    let list = extra_list(e, me);
    let timer = e.call(EXTRA_GET_ACTIVATE_TIMER, &args![list]).f32();
    let mut new_timer = (f64::from(timer) + f64::from(frame_time)) as f32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let record = e.mem.u32(slot);
        let due = e.mem.f32(record + 4);
        if timer < due {
            if new_timer >= due {
                let child = e.mem.u32(record);
                e.call(ACTIVATE_REFERENCE, &args![child, me, 0u32, 0u32, 1u32]);
            } else {
                pending = true;
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if !pending {
        e.vcall(me, SLOT_MARK_CHANGED, &args![ACTIVATING_CHANGED_FLAG]);
        new_timer = 0.0;
    }
    let list = extra_list(e, me);
    e.call(EXTRA_SET_ACTIVATE_TIMER, &args![list, new_timer]);
    pending
}

// Translated from 0056a470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: when `first` is not 0, the list's three-word
/// setter (`0041f080(first, second, third)`).
pub fn fn_0056a470(e: &mut Engine, this: Ptr<TESObjectREFR>, first: u32, second: u32, third: u32) {
    if first != 0 {
        let list = extra_list(e, this.addr());
        e.call(EXTRA_SET_THREE_WORDS, &args![list, first, second, third]);
    }
}

// Translated from 0056a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the Xbox PDB: refreshes the decals (`0056a8d0`), then for
/// every decal reference record of the list (`0041f050`) fills a decal data
/// record (`004a37b0`, 0x80 bytes) and hands it to the parent cell's decal
/// manager (`004a3fe0(cell, data, 4, 0)`). The record takes the position
/// and rotation from the decal record (+4, +0x10), the 3D of this reference
/// and the decal reference's base form. With a holder (`0056a860`) the size
/// comes from random floats, the colour from the bytes of the holder's base
/// form, and the flags and karma from the holder; without one the size is
/// the quotient of the base form's virtuals `+0x98` and `+0x94` (an
/// unsigned division, as the exe does), times 128.0 and the decal
/// reference's scale. The unwinding frames are not translated. The exe
/// pops one stack word it never reads (`RET 4`; its one caller, Load3D at
/// 0056bf95, pushes 0).
pub fn fn_0056a4a0(e: &mut Engine, this: Ptr<TESObjectREFR>, _unused: u32) {
    let me = this.addr();
    e.call(DECAL_REFRESH, &args![me]);
    let list = extra_list(e, me);
    let mut node = e.call(EXTRA_GET_DECALS, &args![list]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let decal = e.mem.u32(slot);
        node = e.call(LIST_NEXT, &args![node]).u32();
        e.with_stack(DECAL_DATA_SIZE + 0x20, |e, block| {
            let data = block.addr();
            let pointer = data + DECAL_DATA_SIZE;
            e.call(DECAL_DATA_CONSTRUCT, &args![data]);
            copy_vector(e, decal + DECAL_RECORD_POSITION, data + DECAL_FIRST_VECTOR);
            copy_vector(e, decal + DECAL_RECORD_ROTATION, data + DECAL_SECOND_VECTOR);
            let node_3d = e.vcall(me, SLOT_GET_3D, &args![]).u32();
            e.mem.set_u32(data + DECAL_NODE, node_3d);
            let decal_reference = e.mem.u32(decal);
            let base = e.call(GET_BASE_FORM, &args![decal_reference]).u32();
            e.mem.set_u32(data + DECAL_BASE_FORM, base);
            e.call(SMART_POINTER_CONSTRUCT, &args![pointer, 0u32]);
            e.vcall(base + 0x30, 0x90, &args![0u32, pointer]);
            let model = e.call(READ_FIRST_DWORD, &args![pointer]).u32();
            if e.call(DECAL_HAS_HOLDER, &args![base]).bool() {
                let holder = e.call(DECAL_HOLDER, &args![base]).u32();
                let a = e.call(HOLDER_FLOAT_A, &args![holder]).f32();
                let b = e.call(HOLDER_FLOAT_B, &args![holder]).f32();
                let width = e.call(RANDOM_FLOAT, &args![b, a]).f32();
                e.mem.set_f32(data + DECAL_FLOAT_38, width);
                let c = e.call(HOLDER_FLOAT_F, &args![holder]).f32();
                let d = e.call(HOLDER_FLOAT_C, &args![holder]).f32();
                let height = e.call(RANDOM_FLOAT, &args![d, c]).f32();
                e.mem.set_f32(data + DECAL_FLOAT_3C, height);
                let divisor: f64 = e.global(BYTE_DIVISOR);
                let form = e.call(GET_BASE_FORM, &args![holder]).u32();
                let high = (f64::from((form >> 16) & 0xff) / divisor) as f32;
                let form = e.call(GET_BASE_FORM, &args![holder]).u32();
                let middle = (f64::from(((form & 0xffff) >> 8) & 0xff) / divisor) as f32;
                let form = e.call(GET_BASE_FORM, &args![holder]).u32();
                let low = (f64::from(form & 0xff) / divisor) as f32;
                let colour = e
                    .call(MAKE_COLOUR, &args![pointer + 0x10, low, middle, high])
                    .u32();
                copy_vector(e, colour, data + DECAL_COLOUR);
                let depth = e.call(HOLDER_FLOAT_D, &args![holder]).f32();
                e.mem.set_f32(data + DECAL_FLOAT_40, depth);
                let karma = e.call(HOLDER_FLOAT_E, &args![holder]).f32();
                e.mem.set_f32(data + DECAL_FLOAT_54, karma);
                let flag_a = e.call(HOLDER_FLAG_A, &args![holder]).u8();
                e.mem.set_u8(data + DECAL_FLAG_A, flag_a);
                let flag_b = e.call(HOLDER_FLAG_B, &args![holder]).u8();
                e.mem.set_u8(data + DECAL_FLAG_B, flag_b);
                if e.call(HOLDER_FLAG_C, &args![holder]).bool() {
                    e.mem.set_u8(data + DECAL_BYTE_73, 1);
                    let distance = e.call(HOLDER_DISTANCE, &args![holder]).f32();
                    e.mem.set_f32(data + DECAL_FLOAT_4C, distance);
                    let flag_d = e.call(HOLDER_FLAG_D, &args![holder]).u8();
                    e.mem.set_u8(data + DECAL_FLAG_D, flag_d);
                }
            } else {
                let width = e.vcall(model, 0x98, &args![]).u32();
                let height = e.vcall(model, 0x94, &args![]).u32();
                // A zero divisor faults in the exe; here it panics.
                let cells = (width / height) as f32;
                let scale = tes_object_refr_get_scale(e, Ptr::new(decal_reference));
                let factor: f64 = e.global(DECAL_SIZE_FACTOR);
                let size = ((f64::from(cells) * factor) * f64::from(scale)) as f32;
                e.mem.set_f32(data + DECAL_FLOAT_38, size);
                e.mem.set_f32(data + DECAL_FLOAT_3C, size);
                let depth: f32 = e.global(DECAL_FLOAT_40_DEFAULT);
                e.mem.set_f32(data + DECAL_FLOAT_40, depth);
            }
            let lifetime: f32 = e.global(DECAL_FLOAT_58_SOURCE);
            e.mem.set_f32(data + DECAL_FLOAT_58, lifetime);
            e.mem.set_u8(data + DECAL_BYTE_70, 0xff);
            e.mem.set_u8(data + DECAL_BYTE_72, 0);
            let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
            e.call(DECAL_MANAGER_ADD, &args![cell, data, 4u32, 0u32]);
            e.call(SMART_POINTER_DESTRUCT, &args![pointer]);
        });
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00563d80,
            fn_00563d80(Ptr<TESObjectREFR>, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x00563f30,
            tes_object_refr_save_controller_manager(Ptr, Ptr, f32)
        ),
        entry!(0x00564010, fn_00564010(Ptr, Ptr, f32)),
        entry!(0x00564360, fn_00564360(Ptr) -> u32),
        entry!(0x00564480, tes_object_refr_create_reference(u8, u8) -> Ptr),
        entry!(0x00564820, fn_00564820(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00564900, fn_00564900(i32) -> bool),
        entry!(
            0x00564930,
            tes_object_refr_set_delete(Ptr<TESObjectREFR>, u8)
        ),
        entry!(0x005649b0, fn_005649b0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00564a00, fn_00564a00(Ptr<TESObjectREFR>, u8)),
        entry!(0x00564ab0, fn_00564ab0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00564b00, fn_00564b00(Ptr<TESObjectREFR>, u8)),
        entry!(0x00564b70, fn_00564b70(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00564bc0, fn_00564bc0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00564c10, fn_00564c10(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00564c60, fn_00564c60(Ptr<TESObjectREFR>, u8)),
        entry!(0x00564cd0, fn_00564cd0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00564d20, fn_00564d20(Ptr<TESObjectREFR>, u8)),
        entry!(0x00564d80, fn_00564d80(Ptr<TESObjectREFR>) -> bool),
        entry!(
            0x00564db0,
            tes_object_refr_set_targeted(Ptr<TESObjectREFR>, u8)
        ),
        entry!(0x00565050, fn_00565050(Ptr<TESObjectREFR>) -> bool),
        entry!(
            0x00565260,
            tes_object_refr_must_ref_persist(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(
            0x005653d0,
            tes_object_refr_get_ref_persists(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(0x00565450, fn_00565450(Ptr<TESObjectREFR>) -> bool),
        entry!(
            0x00565480,
            tes_object_refr_set_ref_persists(Ptr<TESObjectREFR>, u8)
        ),
        entry!(
            0x00565580,
            tes_object_refr_has_time_controllers(Ptr) -> bool
        ),
        entry!(0x00565670, fn_00565670(u32) -> bool),
        entry!(0x00565690, fn_00565690(Ptr<TESObjectREFR>) -> bool),
        entry!(0x005656d0, fn_005656d0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00565730, tes_object_refr_init_script(Ptr<TESObjectREFR>)),
        entry!(
            0x00565870,
            tes_object_refr_run_script(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(
            0x005659f0,
            tes_object_refr_init_animation(Ptr<TESObjectREFR>)
        ),
        entry!(0x00566910, fn_00566910(Ptr) -> u8),
        entry!(0x00566930, fn_00566930(Ptr, u8)),
        entry!(
            0x00566970,
            tes_object_refr_build_kf_file_list(Ptr<TESObjectREFR>, u32) -> u32
        ),
        entry!(0x00567050, fn_00567050(Ptr<TESObjectREFR>)),
        entry!(
            0x005672c0,
            tes_object_refr_end_sequence(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(
            0x005673c0,
            tes_object_refr_get_script_pointer(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x005673e0,
            tes_object_refr_get_script_variables(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00567400,
            tes_object_refr_get_scale(Ptr<TESObjectREFR>) -> f32
        ),
        entry!(0x00567470, fn_00567470(Ptr) -> f32),
        entry!(0x00567490, fn_00567490(Ptr<TESObjectREFR>, f32)),
        entry!(0x00567730, fn_00567730(Ptr) -> f32),
        entry!(0x00567750, fn_00567750(Ptr) -> f32),
        entry!(0x00567770, fn_00567770(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00567790,
            tes_object_refr_get_owner(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x005678a0,
            tes_object_refr_is_partof_evil_faction(Ptr<TESObjectREFR>) -> u8
        ),
        entry!(
            0x00567960,
            tes_object_refr_get_ownership_global(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x005679f0,
            tes_object_refr_get_ownership_rank(Ptr<TESObjectREFR>) -> i32
        ),
        entry!(0x00567ab0, fn_00567ab0(Ptr) -> i32),
        entry!(0x00567ad0, fn_00567ad0(Ptr<TESObjectREFR>, u32)),
        entry!(
            0x00567b50,
            tes_object_refr_set_global(Ptr<TESObjectREFR>, u32)
        ),
        entry!(0x00567bd0, fn_00567bd0(Ptr<TESObjectREFR>, i32)),
        entry!(0x00567c50, fn_00567c50(Ptr<TESObjectREFR>)),
        entry!(
            0x00567d20,
            tes_object_refr_get_encounter_zone(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00567d90, fn_00567d90(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00567dd0,
            tes_object_refr_set_encounter_zone(Ptr<TESObjectREFR>, u32)
        ),
        entry!(
            0x00567e10,
            tes_object_refr_get_calc_level(Ptr<TESObjectREFR>, u8) -> u32
        ),
        entry!(0x00567ec0, fn_00567ec0(Ptr) -> u16),
        entry!(0x00567f00, fn_00567f00(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00567f40, fn_00567f40(Ptr<TESObjectREFR>, u32)),
        entry!(
            0x00567f80,
            tes_object_refr_get_marker_used(Ptr<TESObjectREFR>, u32, u8) -> u8
        ),
        entry!(
            0x00568020,
            tes_object_refr_set_marker_used(Ptr<TESObjectREFR>, u32, u8)
        ),
        entry!(
            0x00568150,
            tes_object_refr_set_marker_reserved(Ptr<TESObjectREFR>, u32, u8)
        ),
        entry!(
            0x00568260,
            tes_object_refr_has_free_marker(Ptr<TESObjectREFR>, u8) -> bool
        ),
        entry!(
            0x005682c0,
            tes_object_refr_get_first_free_marker_index(Ptr<TESObjectREFR>, u8) -> i32
        ),
        entry!(0x00568480, fn_00568480(Ptr<TESObjectREFR>)),
        entry!(
            0x00568500,
            tes_object_refr_get_marker_at_index(Ptr<TESObjectREFR>, u32, Ptr) -> bool
        ),
        entry!(0x00568650, fn_00568650(Ptr) -> f32),
        entry!(
            0x00568680,
            tes_object_refr_is_furniture(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(
            0x005686b0,
            tes_object_refr_get_closest_free_marker(
                Ptr<TESObjectREFR>,
                Ptr,
                u8,
                u8,
                Ptr,
                Ptr,
                u8,
            ) -> bool
        ),
        entry!(0x00568ab0, fn_00568ab0(Ptr, u8)),
        entry!(0x00568ad0, fn_00568ad0(Ptr<TESObjectREFR>) -> f32),
        entry!(
            0x00568bb0,
            tes_object_refr_set_extra(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(0x00568bd0, fn_00568bd0(Ptr<TESObjectREFR>, f32)),
        entry!(
            0x00568cb0,
            tes_object_refr_get_radius(Ptr<TESObjectREFR>) -> f32
        ),
        entry!(0x00568e50, fn_00568e50(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00568e70,
            tes_object_refr_add_teleport(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00568f30,
            tes_object_refr_remove_teleport(Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00568fa0,
            tes_object_refr_get_linked_door_teleport_position(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00569000,
            tes_object_refr_get_linked_door_teleport_rotation(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00569060,
            tes_object_refr_get_map_marker_data(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00569080, fn_00569080(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x005690a0,
            tes_object_refr_add_lock(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00569160,
            tes_object_refr_get_lock(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x005691b0,
            tes_object_refr_get_lock_level(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x005691f0,
            tes_object_refr_get_extra_lock(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00569250, tes_object_refr_lock(Ptr<TESObjectREFR>)),
        entry!(0x00569280, fn_00569280(Ptr)),
        entry!(0x005692a0, tes_object_refr_un_lock(Ptr<TESObjectREFR>)),
        entry!(0x005692d0, fn_005692d0(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00569350, fn_00569350(Ptr<TESObjectREFR>, u32)),
        entry!(0x005693d0, fn_005693d0(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00569410, find_emittance_property(Ptr) -> bool),
        entry!(
            0x005694c0,
            tes_object_refr_check_uses_external_emittance(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(0x00569580, fn_00569580(Ptr<TESObjectREFR>) -> u32),
        entry!(0x005695a0, fn_005695a0(Ptr<TESObjectREFR>, Ptr) -> bool),
        entry!(0x00569620, fn_00569620(Ptr<TESObjectREFR>, Ptr) -> u32),
        entry!(0x00569670, fn_00569670(Ptr, Ptr, Ptr, u8) -> u8),
        entry!(0x00569840, fn_00569840(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00569880,
            tes_object_refr_get_multi_bound_half_extent(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00569910, fn_00569910() -> u32),
        entry!(
            0x00569920,
            tes_object_refr_get_multi_bound(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00569990, fn_00569990(Ptr<TESObjectREFR>)),
        entry!(
            0x005699b0,
            tes_object_refr_get_multi_bound_room(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00569ac0, fn_00569ac0(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00569ae0,
            fn_00569ae0(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(0x00569b20, fn_00569b20(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00569b40,
            fn_00569b40(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(0x00569b80, fn_00569b80(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00569ba0,
            fn_00569ba0(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00569be0,
            tes_object_refr_draw_ref_link(Ptr<TESObjectREFR>, u8)
        ),
        entry!(
            0x00569cb0,
            tes_object_refr_redraw_ref_links(Ptr<TESObjectREFR>, u8, Ptr)
        ),
        entry!(
            0x00569df0,
            tes_object_refr_draw_activate_ref_link(Ptr<TESObjectREFR>, u8)
        ),
        entry!(
            0x0056a1e0,
            tes_object_refr_redraw_activate_ref_links(Ptr<TESObjectREFR>, u8)
        ),
        entry!(
            0x0056a250,
            tes_object_refr_is_activating_children(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(
            0x0056a290,
            fn_0056a290(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x0056a380,
            tes_object_refr_update_child_activates(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(0x0056a470, fn_0056a470(Ptr<TESObjectREFR>, u32, u32, u32)),
        entry!(0x0056a4a0, fn_0056a4a0(Ptr<TESObjectREFR>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Registers `addrs` as doubles that do nothing and return 0.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A double that returns `value` in `eax`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that returns `value` in `ST0`.
    fn returns_float(e: &mut Engine, addr: u32, value: f32) {
        e.register_double(addr, move |_, _| Ret {
            st0: f64::from(value),
            ..Ret::default()
        });
    }

    /// Runs `call` with the call log on and gives back the log.
    fn logged(e: &mut Engine, call: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        call(e);
        e.call_log.take().unwrap()
    }

    /// An engine with the accessors every function here leans on registered
    /// with the bodies the exe gives them (a form is: type byte at +4, flags
    /// at +8, ID at +0xc; the base form of a reference is at +0x20), and the
    /// pages of the globals mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_a000,
            0x0101_3000,
            0x0102_f000,
            0x0118_7000,
            0x0118_a000,
            0x0119_7000,
            0x011c_3000,
            0x011c_b000,
            0x011d_d000,
            0x011d_e000,
            0x011f_2000,
            0x011f_3000,
            0x011f_6000,
            0x0101_6000,
            0x0101_7000,
            0x0102_0000,
            0x0103_0000,
            0x010a_2000,
            0x011c_a000,
            0x011d_0000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(FORM_GET_FLAGS, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FORM_SET_FLAGS, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(FORM_IS_DELETED, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register(FORM_IS_DISABLED, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x800 != 0).into_ret()
        });
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(STEP_DIVISOR, 20.0f64);
        e.set_global(MIN_STEP_DOUBLE, 0.0166666f64);
        e.set_global(MIN_STEP_FLOAT, 0.0166666f32);
        e.set_global(DEFAULT_BLEND_TIME, 0.25f32);
        e
    }

    /// A fresh fake address with `f` registered at it, for vtable slots.
    fn fake_function(e: &mut Engine, f: AbiFn) -> u32 {
        static NEXT: AtomicU32 = AtomicU32::new(0x00fd_0000);
        let addr = NEXT.fetch_add(4, Ordering::Relaxed);
        e.register(addr, f);
        addr
    }

    /// An object whose vtable has the given (byte offset, function) slots.
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, AbiFn)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x500);
        for (offset, f) in slots {
            let target = fake_function(e, *f);
            e.mem.set_u32(vtable + offset, target);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// A form with a type byte, ID and flags.
    fn form(e: &mut Engine, kind: u8, id: u32, flags: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u8(form + 4, kind);
        e.mem.set_u32(form + 8, flags);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    /// A reference (with a vtable made of `slots`) over the base form `base`.
    fn refr_with(
        e: &mut Engine,
        base: u32,
        flags: u32,
        slots: &[(u32, AbiFn)],
    ) -> Ptr<TESObjectREFR> {
        let object = object_with_vtable(e, 0x100, slots);
        e.mem.set_u32(object + 8, flags);
        e.mem.set_u32(object + 0x20, base);
        Ptr::new(object)
    }

    /// A reference whose base form has the type `kind`.
    fn refr_of_kind(e: &mut Engine, kind: u8) -> Ptr<TESObjectREFR> {
        let base = form(e, kind, 0x77, 0);
        refr_with(e, base, 0, &[])
    }

    fn cstr(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(address, text.as_bytes());
        address
    }

    /// The doubles of the sequence and manager methods. A sequence object
    /// (see [`sequence`]) has +4 non-zero when generic, +8 the name's C
    /// string and +0x2c, +0x30, +0x34 its offset time, limit time and
    /// duration; a manager (see [`manager`]) +4 the count and the sequences
    /// from +8.
    fn sequence_api(e: &mut Engine) {
        e.register(MANAGER_SEQUENCE_COUNT, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(MANAGER_SEQUENCE_AT, |e, a| {
            e.mem.u32(a[0] + 8 + 4 * a[1]).into_ret()
        });
        e.register(MANAGER_HAS_SEQUENCES, |_, _| true.into_ret());
        e.register(SEQUENCE_NAME_HOLDER, |_, a| (a[0] + 8).into_ret());
        e.register(NAME_TEXT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SEQUENCE_IS_GENERIC, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SEQUENCE_OFFSET_TIME, |e, a| {
            e.mem.f32(a[0] + 0x2c).into_ret()
        });
        e.register(SEQUENCE_LIMIT_TIME, |e, a| {
            e.mem.f32(a[0] + 0x30).into_ret()
        });
        e.register(SEQUENCE_DURATION, |e, a| e.mem.f32(a[0] + 0x34).into_ret());
        let compare: AbiFn = |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            (first.cmp(&second) as i32).into_ret()
        };
        e.register(STRCMP, compare);
        e.register(STRING_COMPARE, compare);
        e.register(MAKE_VELOCITY, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
    }

    fn sequence(e: &mut Engine, name: &str, generic: bool) -> u32 {
        let sequence = e.mem.alloc(0x40);
        let text = cstr(e, name);
        e.mem.set_u32(sequence + 8, text);
        e.mem.set_u32(sequence + 4, u32::from(generic));
        sequence
    }

    fn manager(e: &mut Engine, sequences: &[u32]) -> u32 {
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 4, sequences.len() as u32);
        for (index, sequence) in sequences.iter().enumerate() {
            e.mem.set_u32(manager + 8 + 4 * index as u32, *sequence);
        }
        manager
    }

    // ---- 00563d80 ---------------------------------------------------------

    fn restart_fixture() -> (Engine, Ptr<TESObjectREFR>, u32, u32, u32, u32) {
        let mut e = engine();
        sequence_api(&mut e);
        stub(
            &mut e,
            &[
                MANAGER_DEACTIVATE_ALL,
                MANAGER_SET_FLAG,
                MANAGER_ACTIVATE,
                SEQUENCE_SET_OFFSET,
                RESET_SIM,
                SET_3D_VELOCITY,
                MANAGER_FINISH_SEQUENCE,
            ],
        );
        let node = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, node);
        let idle = sequence(&mut e, "Idle", false);
        let walk = sequence(&mut e, "Walk", true);
        let manager = manager(&mut e, &[idle, walk]);
        let refr = refr_of_kind(&mut e, 0);
        (e, refr, manager, idle, walk, node)
    }

    #[test]
    fn fn_00563d80_plays_a_sequence_from_a_time_before_its_offset() {
        let (mut e, refr, manager, idle, _, node) = restart_fixture();
        e.mem.set_f32(idle + 0x2c, 2.0);
        let name = cstr(&mut e, "Idle");
        let log = logged(&mut e, |e| {
            let played = e
                .call(0x0056_3d80, &args![refr, manager, name, 1.0f32])
                .bool();
            assert!(played);
        });
        // the sequence is not a generic one, so it is activated
        assert_eq!(
            calls_to(&log, MANAGER_ACTIVATE),
            vec![vec![
                manager,
                idle,
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0
            ]]
        );
        // offset -FLT_MAX, the record holds the offset time
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![vec![idle, (-f32::MAX).to_bits()]]
        );
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(
            (made[0][1], made[0][2], made[0][3]),
            (2.0f32.to_bits(), 1, 0)
        );
        assert_eq!(calls_to(&log, RESET_SIM), vec![vec![node, 1]]);
        assert_eq!(
            calls_to(&log, MANAGER_FINISH_SEQUENCE),
            vec![vec![manager, idle, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, MANAGER_SET_FLAG),
            vec![vec![manager, 1], vec![manager, 0]]
        );
    }

    #[test]
    fn fn_00563d80_clamps_to_the_limit_when_the_time_is_past_it() {
        let (mut e, refr, manager, idle, _, _) = restart_fixture();
        e.mem.set_f32(idle + 0x2c, 0.5);
        e.mem.set_f32(idle + 0x30, 0.75);
        let name = cstr(&mut e, "Idle");
        let log = logged(&mut e, |e| {
            e.call(0x0056_3d80, &args![refr, manager, name, 1.0f32]);
        });
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![vec![idle, 0.75f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, MAKE_VELOCITY)[0][1], 0.75f32.to_bits());
    }

    #[test]
    fn fn_00563d80_keeps_the_time_inside_the_limits_and_skips_activating_generic() {
        let (mut e, refr, manager, _, walk, _) = restart_fixture();
        e.mem.set_f32(walk + 0x2c, 0.5);
        e.mem.set_f32(walk + 0x30, 5.0);
        let name = cstr(&mut e, "Walk");
        let log = logged(&mut e, |e| {
            e.call(0x0056_3d80, &args![refr, manager, name, 1.0f32]);
        });
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![vec![walk, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, MAKE_VELOCITY)[0][1], 1.0f32.to_bits());
        // a generic sequence is not activated again
        assert!(calls_to(&log, MANAGER_ACTIVATE).is_empty());
    }

    #[test]
    fn fn_00563d80_reports_false_without_a_match_a_manager_or_a_3d() {
        let (mut e, refr, manager, _, _, _) = restart_fixture();
        let missing = cstr(&mut e, "Missing");
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(0x0056_3d80, &args![refr, manager, missing, 1.0f32])
                .bool());
            assert!(!e
                .call(0x0056_3d80, &args![refr, 0u32, missing, 1.0f32])
                .bool());
        });
        assert!(calls_to(&log, MANAGER_DEACTIVATE_ALL).is_empty());
        returns(&mut e, GET_LOADED_3D, 0);
        let name = cstr(&mut e, "Idle");
        assert!(!e
            .call(0x0056_3d80, &args![refr, manager, name, 1.0f32])
            .bool());
    }

    // ---- 00563f30 ---------------------------------------------------------

    #[test]
    fn save_controller_manager_writes_the_generic_sequences() {
        let mut e = engine();
        sequence_api(&mut e);
        returns(&mut e, SAVE_START_SIZED, 77);
        stub(
            &mut e,
            &[SAVE_STRING, SEQUENCE_SAVE_TO_BUFFER, SAVE_END_SIZED],
        );
        let idle = sequence(&mut e, "Idle", false);
        let walk = sequence(&mut e, "Walk", true);
        let run = sequence(&mut e, "Run", true);
        let manager = manager(&mut e, &[idle, walk, run]);
        let buffer = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            e.call(0x0056_3f30, &args![manager, buffer, 0.5f32]);
        });
        let walk_name = e.mem.u32(walk + 8);
        let run_name = e.mem.u32(run + 8);
        assert_eq!(
            calls_to(&log, SAVE_STRING),
            vec![vec![buffer, walk_name, 0], vec![buffer, run_name, 0]]
        );
        assert_eq!(
            calls_to(&log, SEQUENCE_SAVE_TO_BUFFER),
            vec![
                vec![walk, buffer, 0.5f32.to_bits()],
                vec![run, buffer, 0.5f32.to_bits()]
            ]
        );
        // the count and the start returned by the start call
        assert_eq!(calls_to(&log, SAVE_END_SIZED), vec![vec![buffer, 2, 77]]);
    }

    #[test]
    fn save_controller_manager_uses_the_default_blend_and_copes_without_a_manager() {
        let mut e = engine();
        sequence_api(&mut e);
        returns(&mut e, SAVE_START_SIZED, 5);
        stub(
            &mut e,
            &[SAVE_STRING, SEQUENCE_SAVE_TO_BUFFER, SAVE_END_SIZED],
        );
        let walk = sequence(&mut e, "Walk", true);
        let manager = manager(&mut e, &[walk]);
        let buffer = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            e.call(0x0056_3f30, &args![manager, buffer, -1.0f32]);
            e.call(0x0056_3f30, &args![0u32, buffer, 1.0f32]);
        });
        assert_eq!(
            calls_to(&log, SEQUENCE_SAVE_TO_BUFFER),
            vec![vec![walk, buffer, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, SAVE_END_SIZED),
            vec![vec![buffer, 1, 5], vec![buffer, 0, 5]]
        );
    }

    // ---- 00564010 ---------------------------------------------------------

    /// A load of two entries: the first names "Walk" (in the manager), the
    /// second "Missing".
    fn load_fixture(actor_3d: bool) -> (Engine, u32, u32, u32, u32) {
        let mut e = engine();
        sequence_api(&mut e);
        stub(
            &mut e,
            &[
                MANAGER_DEACTIVATE_ALL,
                MANAGER_SET_FLAG,
                MANAGER_ACTIVATE,
                SEQUENCE_LOAD_FROM_BUFFER,
                SEQUENCE_CONSTRUCT,
                SEQUENCE_DESTRUCT,
                RESET_SIM,
                SET_3D_VELOCITY,
                ADD_3D_VELOCITY,
            ],
        );
        returns(&mut e, LOAD_START_SIZED, 2);
        let mut names = vec!["Walk", "Missing"].into_iter();
        e.register_double(LOAD_STRING, move |e, a| {
            let name = names.next().unwrap();
            e.mem.set_cstr(a[1], name.as_bytes());
            Ret::default()
        });
        let node = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, node);
        returns(&mut e, IS_ACTOR_3D, u32::from(actor_3d));
        let owner = e.mem.alloc(0x20);
        // the buffer's virtual +8 gives the owner stored at +4
        let buffer =
            object_with_vtable(&mut e, 0x20, &[(8, |e, a| e.mem.u32(a[0] + 4).into_ret())]);
        e.mem.set_u32(buffer + 4, owner);
        let walk = sequence(&mut e, "Walk", false);
        e.mem.set_f32(walk + 0x34, 2.0);
        let manager = manager(&mut e, &[walk]);
        (e, buffer, manager, walk, node)
    }

    #[test]
    fn fn_00564010_loads_known_sequences_and_consumes_unknown_ones() {
        let (mut e, buffer, manager, walk, node) = load_fixture(false);
        let log = logged(&mut e, |e| {
            e.call(0x0056_4010, &args![manager, buffer, 1.0f32]);
        });
        // the manager is cleared and switched on because entries follow
        assert_eq!(
            calls_to(&log, MANAGER_DEACTIVATE_ALL),
            vec![vec![manager, 0.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, MANAGER_SET_FLAG), vec![vec![manager, 1]]);
        // the known sequence is activated and loaded
        assert_eq!(calls_to(&log, MANAGER_ACTIVATE).len(), 1);
        let loads = calls_to(&log, SEQUENCE_LOAD_FROM_BUFFER);
        assert_eq!(loads.len(), 2);
        assert_eq!(loads[0], vec![walk, buffer, 1.0f32.to_bits()]);
        // the unknown one goes into a temporary sequence, built and destroyed
        let built = calls_to(&log, SEQUENCE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(loads[1][0], built[0][0]);
        assert_eq!(calls_to(&log, SEQUENCE_DESTRUCT), built);
        // not an actor's 3D: only the final record, with the blend time
        assert!(calls_to(&log, ADD_3D_VELOCITY).is_empty());
        assert_eq!(calls_to(&log, RESET_SIM), vec![vec![node, 1]]);
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(made.len(), 1);
        assert_eq!(
            (made[0][1], made[0][2], made[0][3]),
            (1.0f32.to_bits(), 1, 0)
        );
    }

    #[test]
    fn fn_00564010_steps_an_actors_3d_through_the_sequence() {
        let (mut e, buffer, manager, _, node) = load_fixture(true);
        let log = logged(&mut e, |e| {
            e.call(0x0056_4010, &args![manager, buffer, 1.0f32]);
        });
        // duration 2 + blend 1 = 3; from 0 in steps of 0.15 up to 1
        let made = calls_to(&log, MAKE_VELOCITY);
        let times: Vec<f32> = made.iter().map(|m| f32::from_bits(m[1])).collect();
        assert_eq!(calls_to(&log, ADD_3D_VELOCITY).len(), 7);
        assert_eq!(times.len(), 8);
        assert_eq!(times[0], 0.0);
        assert!((times[6] - 0.9).abs() < 1e-5);
        assert!(calls_to(&log, ADD_3D_VELOCITY)
            .iter()
            .all(|call| call[0] == node));
        // the last record is the blend time
        assert_eq!(times[7], 1.0);
    }

    #[test]
    fn fn_00564010_without_a_manager_only_consumes_the_data() {
        let (mut e, buffer, _, _, node) = load_fixture(true);
        let log = logged(&mut e, |e| {
            e.call(0x0056_4010, &args![0u32, buffer, -1.0f32]);
        });
        assert!(calls_to(&log, MANAGER_DEACTIVATE_ALL).is_empty());
        // both entries go to temporaries, nothing was found: no record
        assert_eq!(calls_to(&log, SEQUENCE_CONSTRUCT).len(), 2);
        assert!(calls_to(&log, RESET_SIM).is_empty());
        let _ = node;
        // the default blend stands for -1.0
        assert_eq!(
            calls_to(&log, SEQUENCE_LOAD_FROM_BUFFER)[0][2],
            0.25f32.to_bits()
        );
    }

    // ---- 00564360 ---------------------------------------------------------

    #[test]
    fn fn_00564360_classifies_the_base_form_of_the_owning_reference() {
        let mut e = engine();
        let base_kind = |e: &mut Engine, kind: u8| {
            let base = form(e, kind, 1, 0);
            refr_with(e, base, 0, &[]).addr()
        };
        // 0056f930 finds the reference owning a 3D node; here the node is
        // the reference itself
        e.register(0x0056_f930, |_, a| a[0].into_ret());
        for (kind, class) in [
            (0x15u8, 1u32),
            (0x1c, 1),
            (0x20, 1),
            (0x21, 1),
            (0x1b, 2),
            (0x27, 2),
            (0x2a, 3),
            (0x2b, 3),
            (0x18, 4),
            (0x1a, 4),
            (0x28, 4),
            (0x29, 4),
            (0x1d, 5),
            (0x74, 5),
            (0x67, 5),
            (0x16, 6),
            (0x75, 6),
            (0x01, 6),
        ] {
            let node = base_kind(&mut e, kind);
            assert_eq!(
                e.call(0x0056_4360, &args![node]).u32(),
                class,
                "type {kind:#x}"
            );
        }
        // no node, no reference or no base form
        assert_eq!(e.call(0x0056_4360, &args![0u32]).u32(), 0);
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_4360, &args![bare]).u32(), 0);
        returns(&mut e, 0x0056_f930, 0);
        let node = base_kind(&mut e, 0x15);
        assert_eq!(e.call(0x0056_4360, &args![node]).u32(), 0);
    }

    // ---- 00564480 ---------------------------------------------------------

    #[test]
    fn create_reference_allocates_and_constructs_by_kind() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        // every constructor returns its this (+1, so a mix-up shows)
        for ctor in [
            0x0092_eb50u32,
            0x0055_a2f0,
            0x008d_1f40,
            0x008d_4500,
            0x009b_7a50,
            0x009b_3fa0,
            0x0097_8fd0,
            0x009b_3160,
            0x009a_aff0,
        ] {
            e.register(ctor, |_, a| (a[0] + 1).into_ret());
        }
        stub(&mut e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        for (kind, size, ctor, takes_flag) in [
            (0x16u8, 0x88u32, 0x0092_eb50u32, false),
            (0x3a, 0x68, 0x0055_a2f0, false),
            (0x3b, 0x1c8, 0x008d_1f40, true),
            (0x3c, 0x1c0, 0x008d_4500, true),
            (0x3d, 0x160, 0x009b_7a50, true),
            (0x3e, 0x154, 0x009b_3fa0, true),
            (0x3f, 0x154, 0x0097_8fd0, true),
            (0x40, 0x158, 0x009b_3160, true),
            (0x69, 0x158, 0x009a_aff0, true),
        ] {
            let log = logged(&mut e, |e| {
                let made = e.call(0x0056_4480, &args![u32::from(kind), 1u32]).u32();
                assert_ne!(made, 0);
                assert_eq!(e.mem.block_size(made - 1), Some(size.div_ceil(8) * 8));
            });
            let constructed = calls_to(&log, ctor);
            assert_eq!(constructed.len(), 1, "kind {kind:#x}");
            assert_eq!(constructed[0].len(), if takes_flag { 2 } else { 1 });
            if takes_flag {
                assert_eq!(constructed[0][1], 1);
            }
            // the scope guard (tag 0x31) surrounds the allocation
            let guard = calls_to(&log, SCOPE_GUARD_CTOR);
            assert_eq!(guard[0][1..], [0x31, 1, SOURCE_FILE, 0x1493]);
            assert_eq!(calls_to(&log, SCOPE_GUARD_DTOR).len(), 1);
        }
        // another kind gives null without allocating
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_4480, &args![0x17u32, 0u32]).u32(), 0);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // a failed allocation gives null without constructing
        returns(&mut e, OPERATOR_NEW, 0);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_4480, &args![0x3bu32, 0u32]).u32(), 0);
        });
        assert!(calls_to(&log, 0x008d_1f40).is_empty());
    }

    // ---- 00564820 and 00564900 -------------------------------------------

    #[test]
    fn fn_00564820_picks_the_reference_kind() {
        let mut e = engine();
        e.register(0x004f_d4f0, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        for (kind, value, expected) in [
            (0x2au8, 0u32, 0x3bu32),
            (0x2b, 0, 0x3c),
            (0x33, 0x10000, 0x3d),
            (0x33, 0x20000, 0x3e),
            (0x33, 0x40000, 0x3f),
            (0x33, 0x80000, 0x40),
            (0x33, 0x100000, 0x69),
            (0x33, 0x1, 0x3a),
            (0x10, 0, 0x3a),
        ] {
            let base = form(&mut e, kind, 1, 0);
            e.mem.set_u32(base + 0x40, value);
            let refr = refr_with(&mut e, base, 0, &[]);
            assert_eq!(
                e.call(0x0056_4820, &args![refr]).u32(),
                expected,
                "type {kind:#x} value {value:#x}"
            );
        }
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_4820, &args![bare]).u32(), 0x3a);
    }

    #[test]
    fn fn_00564900_accepts_the_reference_kinds() {
        let mut e = engine();
        for (kind, expected) in [
            (0x39i32, false),
            (0x3a, true),
            (0x40, true),
            (0x41, false),
            (0x69, true),
            (0x6a, false),
            (-1, false),
        ] {
            assert_eq!(
                e.call(0x0056_4900, &args![kind]).bool(),
                expected,
                "kind {kind:#x}"
            );
        }
    }

    // ---- 00564930 ---------------------------------------------------------

    fn delete_fixture() -> (Engine, Ptr<TESObjectREFR>, u32) {
        let mut e = engine();
        let handler = e.mem.alloc(0x20);
        let loading = e.mem.alloc(0x20);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        e.set_global(GLOBAL_LOADING_OBJECT, loading);
        returns(&mut e, 0x0042_26e0, 0);
        returns(&mut e, 0x0046_9860, 1);
        returns(&mut e, 0x0040_77c0, 0);
        returns(&mut e, 0x0042_ce10, 0);
        stub(&mut e, &[0x0084_a7c0, 0x0048_4530]);
        let refr = refr_of_kind(&mut e, 0);
        (e, refr, loading)
    }

    #[test]
    fn set_delete_tells_the_loading_object_when_every_check_passes() {
        let (mut e, refr, loading) = delete_fixture();
        let log = logged(&mut e, |e| {
            e.call(0x0056_4930, &args![refr, 1u32]);
        });
        assert_eq!(
            calls_to(&log, 0x0084_a7c0),
            vec![vec![loading, refr.addr()]]
        );
        assert_eq!(calls_to(&log, 0x0048_4530), vec![vec![refr.addr(), 1]]);
    }

    #[test]
    fn set_delete_skips_the_notification_when_a_check_fails() {
        for (blocker, value) in [
            (0x0042_26e0u32, 1u32),
            (0x0046_9860, 0),
            (0x0040_77c0, 1),
            (0x0042_ce10, 1),
        ] {
            let (mut e, refr, _) = delete_fixture();
            returns(&mut e, blocker, value);
            let log = logged(&mut e, |e| {
                e.call(0x0056_4930, &args![refr, 1u32]);
            });
            assert!(calls_to(&log, 0x0084_a7c0).is_empty(), "{blocker:#x}");
            assert_eq!(calls_to(&log, 0x0048_4530).len(), 1);
        }
        // an undelete never notifies
        let (mut e, refr, _) = delete_fixture();
        let log = logged(&mut e, |e| {
            e.call(0x0056_4930, &args![refr, 0u32]);
        });
        assert!(calls_to(&log, 0x0084_a7c0).is_empty());
        assert_eq!(calls_to(&log, 0x0048_4530), vec![vec![refr.addr(), 0]]);
    }

    // ---- the flag accessors ----------------------------------------------

    #[test]
    fn persistence_flag_getters_check_the_base_form_type() {
        let mut e = engine();
        // 005649b0 (base absent or type 0x1e) and 00564ab0 (type 0x22)
        for (addr, accepted) in [(0x0056_49b0u32, 0x1eu8), (0x0056_4ab0, 0x22)] {
            for (kind, flags, expected) in [
                (Some(accepted), 0x200u32, true),
                (Some(accepted), 0x1ff, false),
                (Some(0x40), 0x200, false),
                (None, 0x200, true),
                (None, 0, false),
            ] {
                let base = kind.map_or(0, |k| form(&mut e, k, 1, 0));
                let refr = refr_with(&mut e, base, flags, &[]);
                assert_eq!(
                    e.call(addr, &args![refr]).bool(),
                    expected,
                    "{addr:#x} {kind:?} {flags:#x}"
                );
            }
        }
    }

    #[test]
    fn fn_00564a00_sets_the_flag_and_keeps_the_tracked_object_in_step() {
        let mut e = engine();
        let tracked = e.mem.alloc(0x10);
        e.mem.set_u32(tracked, 0x1234);
        let manager = e.mem.alloc(0x10);
        returns(&mut e, 0x0057_25f0, tracked);
        returns(&mut e, 0x0045_0b80, manager);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        stub(&mut e, &[0x00b4_f9a0, 0x00b5_c450]);
        let base = form(&mut e, 0x1e, 1, 0);
        let refr = refr_with(&mut e, base, 0x41, &[]);

        let log = logged(&mut e, |e| {
            e.call(0x0056_4a00, &args![refr, 1u32]);
        });
        assert_eq!(e.mem.u32(refr.addr() + 8), 0x241);
        assert!(calls_to(&log, 0x00b4_f9a0).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_c450), vec![vec![manager, 0x1234, 1]]);
        assert_eq!(calls_to(&log, 0x0045_0b80), vec![vec![0]]);

        // clearing also calls 00b4f9a0 first
        let log = logged(&mut e, |e| {
            e.call(0x0056_4a00, &args![refr, 0u32]);
        });
        assert_eq!(e.mem.u32(refr.addr() + 8), 0x41);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        let cleanup = order.iter().position(|a| *a == 0x00b4_f9a0).unwrap();
        let notify = order.iter().position(|a| *a == 0x00b5_c450).unwrap();
        assert!(cleanup < notify);
        assert_eq!(calls_to(&log, 0x00b5_c450), vec![vec![manager, 0x1234, 0]]);

        // nothing tracked: only the flags change
        returns(&mut e, 0x0057_25f0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_4a00, &args![refr, 1u32]);
        });
        assert_eq!(e.mem.u32(refr.addr() + 8), 0x241);
        assert!(calls_to(&log, 0x00b5_c450).is_empty());

        // another base form type does nothing
        let other = form(&mut e, 0x40, 1, 0);
        let refr = refr_with(&mut e, other, 0x41, &[]);
        e.call(0x0056_4a00, &args![refr, 1u32]);
        assert_eq!(e.mem.u32(refr.addr() + 8), 0x41);
    }

    #[test]
    fn flag_setters_change_one_bit_under_their_conditions() {
        let mut e = engine();
        let actor_slot: [(u32, AbiFn); 1] = [(0x100, |_, _| true.into_ret())];
        let plain_slot: [(u32, AbiFn); 1] = [(0x100, |_, _| false.into_ret())];
        // 00564b00: bit 0x200 for a base form of type 0x22 (or none)
        for (kind, set, flags, expected) in [
            (Some(0x22u8), 1u32, 0u32, 0x200u32),
            (Some(0x22), 0, 0x3ff, 0x1ff),
            (None, 1, 0, 0x200),
            (Some(0x23), 1, 0, 0),
        ] {
            let base = kind.map_or(0, |k| form(&mut e, k, 1, 0));
            let refr = refr_with(&mut e, base, flags, &[]);
            e.call(0x0056_4b00, &args![refr, set]);
            assert_eq!(e.mem.u32(refr.addr() + 8), expected, "{kind:?}");
        }
        // 00564c60: bit 0x100 unless there is no base form or it is type 0x1c
        for (kind, set, flags, expected) in [
            (Some(0x20u8), 1u32, 0u32, 0x100u32),
            (Some(0x20), 0, 0x1ff, 0xff),
            (Some(0x1c), 1, 0, 0),
            (None, 1, 0, 0),
        ] {
            let base = kind.map_or(0, |k| form(&mut e, k, 1, 0));
            let refr = refr_with(&mut e, base, flags, &[]);
            e.call(0x0056_4c60, &args![refr, set]);
            assert_eq!(e.mem.u32(refr.addr() + 8), expected, "{kind:?}");
        }
        // 00564d20: bit 0x100000 for a reference that is not an actor
        let refr = refr_with(&mut e, 0, 0, &plain_slot);
        e.call(0x0056_4d20, &args![refr, 1u32]);
        assert_eq!(e.mem.u32(refr.addr() + 8), 0x10_0000);
        e.call(0x0056_4d20, &args![refr, 0u32]);
        assert_eq!(e.mem.u32(refr.addr() + 8), 0);
        let actor = refr_with(&mut e, 0, 0, &actor_slot);
        e.call(0x0056_4d20, &args![actor, 1u32]);
        assert_eq!(e.mem.u32(actor.addr() + 8), 0);
        // 00564db0: bit 0x40000, whatever the reference
        e.call(0x0056_4db0, &args![actor, 5u32]);
        assert_eq!(e.mem.u32(actor.addr() + 8), 0x4_0000);
        e.call(0x0056_4db0, &args![actor, 0u32]);
        assert_eq!(e.mem.u32(actor.addr() + 8), 0);
    }

    #[test]
    fn flag_getters_read_one_bit_under_their_conditions() {
        let mut e = engine();
        // 00564c10: bit 0x100 unless there is no base form or it is type 0x1c
        for (kind, flags, expected) in [
            (Some(0x20u8), 0x100u32, true),
            (Some(0x20), 0xff, false),
            (Some(0x1c), 0x100, false),
            (None, 0x100, false),
        ] {
            let base = kind.map_or(0, |k| form(&mut e, k, 1, 0));
            let refr = refr_with(&mut e, base, flags, &[]);
            assert_eq!(e.call(0x0056_4c10, &args![refr]).bool(), expected);
        }
        // 00564cd0: bit 0x100000, false for an actor
        let plain = refr_with(&mut e, 0, 0x10_0000, &[(0x100, |_, _| false.into_ret())]);
        let actor = refr_with(&mut e, 0, 0x10_0000, &[(0x100, |_, _| true.into_ret())]);
        assert!(e.call(0x0056_4cd0, &args![plain]).bool());
        assert!(!e.call(0x0056_4cd0, &args![actor]).bool());
        e.mem.set_u32(plain.addr() + 8, 0xf_ffff);
        assert!(!e.call(0x0056_4cd0, &args![plain]).bool());
        // 00564d80 (bit 0x40000) and 00565450 (bit 0x400)
        let refr = refr_with(&mut e, 0, 0x4_0400, &[]);
        assert!(e.call(0x0056_4d80, &args![refr]).bool());
        assert!(e.call(0x0056_5450, &args![refr]).bool());
        e.mem.set_u32(refr.addr() + 8, 0x3_fbff);
        assert!(!e.call(0x0056_4d80, &args![refr]).bool());
        assert!(!e.call(0x0056_5450, &args![refr]).bool());
    }

    #[test]
    fn base_form_virtuals_are_forwarded_and_default_to_false() {
        let mut e = engine();
        // the base form answers through its virtuals +0xa8, +0xb0 and +0x94
        let base = object_with_vtable(
            &mut e,
            0x100,
            &[
                (0xa8, |_, _| true.into_ret()),
                (0xb0, |_, _| false.into_ret()),
                (0x94, |_, _| true.into_ret()),
            ],
        );
        let refr = refr_with(&mut e, base, 0, &[]);
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert!(e.call(0x0056_4b70, &args![refr]).bool());
        assert!(!e.call(0x0056_4bc0, &args![refr]).bool());
        assert!(e.call(0x0056_5690, &args![refr]).bool());
        for addr in [0x0056_4b70u32, 0x0056_4bc0, 0x0056_5690] {
            assert!(!e.call(addr, &args![bare]).bool());
        }
    }

    #[test]
    fn fn_00565050_tests_bit_4_of_the_object_it_finds() {
        let mut e = engine();
        e.register(0x0052_7080, |e, a| {
            assert_eq!(a[1], 0x92);
            e.mem.u32(a[0] + 0x30).into_ret()
        });
        let refr = refr_with(&mut e, 0, 0, &[]);
        assert!(!e.call(0x0056_5050, &args![refr]).bool());
        let found = form(&mut e, 0, 5, 0);
        e.mem.set_u32(refr.addr() + 0x30, found);
        assert!(e.call(0x0056_5050, &args![refr]).bool());
        e.mem.set_u32(found + 0xc, 3);
        assert!(!e.call(0x0056_5050, &args![refr]).bool());
    }

    // ---- 00565260 ---------------------------------------------------------

    #[test]
    fn must_ref_persist_goes_by_the_base_form() {
        let mut e = engine();
        returns(&mut e, 0x004f_ede0, 0);
        returns(&mut e, 0x0051_94d0, 0);
        returns(&mut e, 0x0056_8e50, 0);
        returns(&mut e, 0x0055_e200, 0);
        let check = |e: &mut Engine, kind: u8, id: u32| {
            let base = form(e, kind, id, 0);
            let refr = refr_with(e, base, 0, &[]);
            e.call(0x0056_5260, &args![refr]).bool()
        };
        // the listed form IDs persist whatever the type
        for id in [4u32, 5, 6, 0x10, 0x12, 0x15, 0x1f, 0x23, 0x24, 0x34, 0x3b] {
            assert!(check(&mut e, 0x01, id), "id {id:#x}");
        }
        for id in [7u32, 0x0f, 0x11, 0x3c, 0x1000] {
            assert!(!check(&mut e, 0x01, id), "id {id:#x}");
        }
        // types 4 and 0x23 always persist
        assert!(check(&mut e, 4, 0x1000));
        assert!(check(&mut e, 0x23, 0x1000));
        // type 0x16 asks 004fede0(base, 0)
        assert!(!check(&mut e, 0x16, 0x1000));
        returns(&mut e, 0x004f_ede0, 1);
        assert!(check(&mut e, 0x16, 0x1000));
        // type 0x1c asks 005194d0(base), then 00568e50(reference)
        assert!(!check(&mut e, 0x1c, 0x1000));
        returns(&mut e, 0x0056_8e50, 1);
        assert!(check(&mut e, 0x1c, 0x1000));
        returns(&mut e, 0x0056_8e50, 0);
        returns(&mut e, 0x0051_94d0, 1);
        assert!(check(&mut e, 0x1c, 0x1000));
        // types 0x2a and 0x2b ask 0055e200(base + 0x30)
        assert!(!check(&mut e, 0x2a, 0x1000));
        returns(&mut e, 0x0055_e200, 1);
        let base = form(&mut e, 0x2b, 0x1000, 0);
        let refr = refr_with(&mut e, base, 0, &[]);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_5260, &args![refr]).bool());
        });
        assert_eq!(calls_to(&log, 0x0055_e200), vec![vec![base + 0x30]]);
        // no base form: false
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert!(!e.call(0x0056_5260, &args![bare]).bool());
    }

    // ---- 005653d0 ---------------------------------------------------------

    #[test]
    fn get_ref_persists_trusts_the_loaded_state_unless_a_check_objects() {
        let mut e = engine();
        let handler = e.mem.alloc(0x20);
        let save_load = e.mem.alloc(0x20);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        e.set_global(GLOBAL_SAVE_LOAD_GAME, save_load);
        let found = e.mem.alloc(0x20);
        returns(&mut e, 0x0048_4e60, found);
        for addr in [0x0045_16b0u32, 0x0047_c850, 0x0042_26e0, 0x0047_1c20] {
            returns(&mut e, addr, 0);
        }
        let refr = refr_with(&mut e, 0, 0, &[]);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_53d0, &args![refr]).bool());
        });
        assert_eq!(
            calls_to(&log, 0x0048_4e60),
            vec![vec![refr.addr(), u32::MAX]]
        );
        assert_eq!(calls_to(&log, 0x0045_16b0), vec![vec![handler]]);
        assert_eq!(calls_to(&log, 0x0047_c850), vec![vec![save_load]]);
        assert_eq!(calls_to(&log, 0x0047_1c20), vec![vec![found]]);
        // any check that objects falls back to the flag (bit 0x400)
        for addr in [0x0045_16b0u32, 0x0047_c850, 0x0042_26e0, 0x0047_1c20] {
            returns(&mut e, addr, 1);
            assert!(!e.call(0x0056_53d0, &args![refr]).bool());
            e.mem.set_u32(refr.addr() + 8, 0x400);
            assert!(e.call(0x0056_53d0, &args![refr]).bool());
            e.mem.set_u32(refr.addr() + 8, 0);
            returns(&mut e, addr, 0);
        }
        // nothing found: the flag decides
        returns(&mut e, 0x0048_4e60, 0);
        assert!(!e.call(0x0056_53d0, &args![refr]).bool());
        e.mem.set_u32(refr.addr() + 8, 0x400);
        assert!(e.call(0x0056_53d0, &args![refr]).bool());
    }

    // ---- 00565480 ---------------------------------------------------------

    fn persist_fixture() -> (Engine, Ptr<TESObjectREFR>, u32, u32) {
        let mut e = engine();
        stub(&mut e, &[0x0058_7ff0, 0x0058_8030]);
        let cell = e.mem.alloc(0x40);
        let list = e.mem.alloc(0x40);
        returns(&mut e, 0x0042_5fd0, 0);
        returns(&mut e, 0x0055_16c0, 0);
        returns(&mut e, 0x0054_ddd0, list);
        let refr = refr_with(
            &mut e,
            0,
            0,
            &[(0x48, |_, _| Ret::default()), (0xc8, |_, _| Ret::default())],
        );
        // the TESChildCell base at +0x18 has its own vtable whose slot 0
        // gives the cell stored right after the vtable pointer
        let child = object_with_vtable(&mut e, 0x10, &[(0, |e, a| e.mem.u32(a[0] + 4).into_ret())]);
        let child_vtable = e.mem.u32(child);
        e.mem.set_u32(refr.addr() + 0x18, child_vtable);
        e.mem.set_u32(refr.addr() + 0x1c, cell);
        (e, refr, cell, list)
    }

    #[test]
    fn set_ref_persists_adds_the_reference_to_the_cells_persistent_list() {
        let (mut e, refr, cell, list) = persist_fixture();
        let log = logged(&mut e, |e| {
            e.call(0x0056_5480, &args![refr, 1u32]);
        });
        assert_eq!(e.mem.u32(refr.addr() + 8), 0x400);
        assert_eq!(calls_to(&log, 0x0042_5fd0), vec![vec![cell]]);
        assert_eq!(calls_to(&log, 0x0054_ddd0), vec![vec![cell]]);
        assert_eq!(calls_to(&log, 0x0058_7ff0), vec![vec![list, refr.addr()]]);
        assert!(calls_to(&log, 0x0058_8030).is_empty());
        // a cell that fails one of the checks is left alone
        returns(&mut e, 0x0042_5fd0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5480, &args![refr, 1u32]);
        });
        assert!(calls_to(&log, 0x0058_7ff0).is_empty());
    }

    #[test]
    fn set_ref_persists_removes_the_reference_when_cleared() {
        let (mut e, refr, _, list) = persist_fixture();
        e.mem.set_u32(refr.addr() + 8, 0x400);
        // 005516c0 false: nothing to remove, only the flag changes
        let log = logged(&mut e, |e| {
            e.call(0x0056_5480, &args![refr, 0u32]);
        });
        assert_eq!(e.mem.u32(refr.addr() + 8), 0);
        assert!(calls_to(&log, 0x0058_8030).is_empty());
        returns(&mut e, 0x0055_16c0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5480, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0058_8030), vec![vec![list, refr.addr()]]);
        assert!(calls_to(&log, 0x0058_7ff0).is_empty());
    }

    // ---- 00565580 ---------------------------------------------------------

    /// A 3D node: +0x10 its controller, +0x14 its children container (+0 the
    /// count, the children from +4), +0x20 the head of its linked list of
    /// attached objects (cells of `next, item`).
    fn node(e: &mut Engine, controller: u32, items: &[u32], children: &[u32]) -> u32 {
        let node = object_with_vtable(e, 0x40, &[(0xc, |e, a| e.mem.u32(a[0] + 0x14).into_ret())]);
        e.mem.set_u32(node + 0x10, controller);
        let mut head = 0;
        for item in items.iter().rev() {
            let cell = e.mem.alloc(8);
            e.mem.set_u32(cell, head);
            e.mem.set_u32(cell + 4, *item);
            head = cell;
        }
        e.mem.set_u32(node + 0x20, head);
        if !children.is_empty() {
            let container = e.mem.alloc(0x40);
            e.mem.set_u32(container, children.len() as u32);
            for (index, child) in children.iter().enumerate() {
                e.mem.set_u32(container + 4 + 4 * index as u32, *child);
            }
            e.mem.set_u32(node + 0x14, container);
        }
        node
    }

    fn controller_api(e: &mut Engine) {
        e.register(GET_CONTROLLER, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0043_0830, |_, a| (a[0] + 0x20).into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        // 0057cbe0(list, &position): the item slot at the position, which
        // moves on to the next cell
        e.register(0x0057_cbe0, |e, a| {
            let cell = e.mem.u32(a[1]);
            let next = e.mem.u32(cell);
            e.mem.set_u32(a[1], next);
            (cell + 4).into_ret()
        });
        e.register(0x0043_b480, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0043_b4a0, |e, a| {
            e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
        });
    }

    #[test]
    fn has_time_controllers_searches_the_node_its_list_and_its_children() {
        let mut e = engine();
        controller_api(&mut e);
        let with_controller = node(&mut e, 1, &[], &[]);
        let bare = node(&mut e, 0, &[], &[]);
        // the node's own controller
        assert!(e.call(0x0056_5580, &args![with_controller]).bool());
        // an attached object with a controller, found after one without
        let in_list = node(&mut e, 0, &[bare, with_controller], &[]);
        assert!(e.call(0x0056_5580, &args![in_list]).bool());
        // a deeper child
        let child = node(&mut e, 0, &[], &[bare, with_controller]);
        let parent = node(&mut e, 0, &[bare], &[bare, child]);
        assert!(e.call(0x0056_5580, &args![parent]).bool());
        // nothing anywhere
        let empty_child = node(&mut e, 0, &[bare], &[bare]);
        let nothing = node(&mut e, 0, &[bare, bare], &[bare, empty_child]);
        assert!(!e.call(0x0056_5580, &args![nothing]).bool());
        assert!(!e.call(0x0056_5580, &args![0u32]).bool());
    }

    #[test]
    fn has_time_controllers_stops_at_the_first_controller() {
        let mut e = engine();
        controller_api(&mut e);
        let with_controller = node(&mut e, 1, &[], &[]);
        let bare = node(&mut e, 0, &[], &[]);
        let parent = node(&mut e, 0, &[with_controller], &[bare]);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_5580, &args![parent]).bool());
        });
        // the list item decided: the children were never asked
        assert!(calls_to(&log, 0x0043_b480).is_empty());
    }

    // ---- 00565670, 005656d0 -----------------------------------------------

    #[test]
    fn fn_00565670_removes_the_key_from_the_map_at_011ca0e0() {
        let mut e = engine();
        e.register(0x0040_5430, |_, a| (a[1] == 7).into_ret());
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_5670, &args![7u32]).bool());
            assert!(!e.call(0x0056_5670, &args![8u32]).bool());
        });
        assert_eq!(
            calls_to(&log, 0x0040_5430),
            vec![vec![0x011c_a0e0, 7], vec![0x011c_a0e0, 8]]
        );
    }

    #[test]
    fn fn_005656d0_accepts_the_base_form_or_the_inventory_changes() {
        let mut e = engine();
        let accepted = form(&mut e, 0x40, 1, 0);
        let refr = refr_with(&mut e, accepted, 0, &[]);
        let changes = e.mem.alloc(0x10);
        // 004826d0 accepts an object when it is the one made above
        e.register_double(0x0048_26d0, move |_, a| Ret {
            eax: u32::from(a[0] == accepted),
            ..Ret::default()
        });
        returns(&mut e, 0x0055_d310, 1);
        returns(&mut e, 0x004b_f220, changes);
        returns(&mut e, 0x004d_0490, 1);
        // the base form is accepted: no need to look further
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_56d0, &args![refr]).bool());
        });
        assert!(calls_to(&log, 0x0055_d310).is_empty());
        // otherwise a container whose changes agree
        let other = form(&mut e, 0x40, 2, 0);
        e.mem.set_u32(refr.addr() + 0x20, other);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_56d0, &args![refr]).bool());
        });
        assert_eq!(calls_to(&log, 0x004b_f220), vec![vec![refr.addr()]]);
        assert_eq!(calls_to(&log, 0x004d_0490), vec![vec![changes]]);
        returns(&mut e, 0x004d_0490, 0);
        assert!(!e.call(0x0056_56d0, &args![refr]).bool());
        returns(&mut e, 0x0055_d310, 0);
        assert!(!e.call(0x0056_56d0, &args![refr]).bool());
    }

    // ---- a reference with the virtuals the rest of the tests steer ---------

    /// A virtual that answers with the dword at `OFFSET` of its object, so a
    /// test steers the answer through memory.
    fn slot_value<const OFFSET: u32>(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.u32(a[0] + OFFSET).into_ret()
    }

    /// The virtual +0x22c takes a flag: it answers with the dword at `0xa0`
    /// for 0 and at `0xa4` for 1.
    fn slot_22c(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.u32(a[0] + 0xa0 + 4 * a[1]).into_ret()
    }

    /// Where `full_refr` keeps the answers of its virtuals.
    const IS_ACTOR_ANSWER: u32 = 0x70;
    const GET_3D_ANSWER: u32 = 0x74;
    const BIPED_ANSWER: u32 = 0x78;
    const ANIMATION_ANSWER: u32 = 0x7c;
    const SLOT_21C_ANSWER: u32 = 0x80;
    const SLOT_218_ANSWER: u32 = 0x84;
    const SLOT_22C_ARG0_ANSWER: u32 = 0xa0;
    const SLOT_22C_ARG1_ANSWER: u32 = 0xa4;
    const SLOT_1A0_ANSWER: u32 = 0x8c;
    const SLOT_FC_ANSWER: u32 = 0x90;
    const SCRIPT_ANSWER: u32 = 0x94;

    /// A reference over `base` whose virtuals `IsActor` (+0x100), `Get3D`
    /// (+0x1d0), the biped (+0x1e8), the animation (+0x1e4), +0x21c, +0x218,
    /// +0x22c, +0x1a0, +0xfc and the script (+0x1a4) answer with the dwords
    /// the constants above name, and +0x48, +0x140, +0x178 do nothing.
    fn full_refr(e: &mut Engine, base: u32) -> Ptr<TESObjectREFR> {
        let slots: [(u32, AbiFn); 13] = [
            (0x100, slot_value::<IS_ACTOR_ANSWER>),
            (0x1d0, slot_value::<GET_3D_ANSWER>),
            (0x1e8, slot_value::<BIPED_ANSWER>),
            (0x1e4, slot_value::<ANIMATION_ANSWER>),
            (0x21c, slot_value::<SLOT_21C_ANSWER>),
            (0x218, slot_value::<SLOT_218_ANSWER>),
            (0x22c, slot_22c),
            (0x1a0, slot_value::<SLOT_1A0_ANSWER>),
            (0xfc, slot_value::<SLOT_FC_ANSWER>),
            (0x1a4, slot_value::<SCRIPT_ANSWER>),
            (0x48, |_, _| Ret::default()),
            (0x140, |_, _| Ret::default()),
            (0x178, |_, _| Ret::default()),
        ];
        refr_with(e, base, 0, &slots)
    }

    /// The fake address a vtable slot of `object` points to (for call logs).
    fn slot_address(e: &Engine, object: u32, offset: u32) -> u32 {
        let vtable = e.mem.u32(object);
        e.mem.u32(vtable + offset)
    }

    /// A base form that is an object with a vtable (name at +0x130).
    fn named_base(e: &mut Engine, kind: u8, id: u32, name: &str) -> u32 {
        let text = cstr(e, name);
        let base = object_with_vtable(e, 0x200, &[(0x130, slot_value::<0x100>)]);
        e.mem.set_u8(base + 4, kind);
        e.mem.set_u32(base + 0xc, id);
        e.mem.set_u32(base + 0x100, text);
        base
    }

    // ---- 00565730 ---------------------------------------------------------

    fn init_script_fixture() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                SCOPE_GUARD_CTOR,
                SCOPE_GUARD_DTOR,
                0x0041_9ed0,
                0x0041_9f80,
                0x004d_1960,
            ],
        );
        // 004826d0: the script of a form; here the form itself
        e.register(0x0048_26d0, |_, a| a[0].into_ret());
        e.register(0x005a_bf60, |_, a| (a[0] + 1).into_ret());
        returns(&mut e, 0x0041_0220, 0);
        returns(&mut e, 0x0055_d310, 0);
        let base = form(&mut e, 0x40, 1, 0);
        let refr = full_refr(&mut e, base);
        (e, refr)
    }

    #[test]
    fn init_script_stores_the_script_found_for_the_reference() {
        let (mut e, refr) = init_script_fixture();
        let me = refr.addr();
        let base = e.mem.u32(me + 0x20);
        let script = form(&mut e, 0x11, 9, 0);
        // the virtual +0x1a4 gives the script
        e.mem.set_u32(me + SCRIPT_ANSWER, script);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5730, &args![refr]);
        });
        let list = me + 0x44;
        assert_eq!(calls_to(&log, 0x0041_9ed0), vec![vec![list, script]]);
        assert_eq!(calls_to(&log, 0x0041_9f80), vec![vec![list, script + 1]]);
        // the guard (tag 0x15) surrounds it
        let guard = calls_to(&log, SCOPE_GUARD_CTOR);
        assert_eq!(guard[0][1..], [0x15, 1, SOURCE_FILE, 0x196e]);
        assert_eq!(calls_to(&log, SCOPE_GUARD_DTOR).len(), 1);
        // without a script of its own the base form is used
        e.mem.set_u32(me + SCRIPT_ANSWER, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5730, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0041_9ed0), vec![vec![list, base]]);
    }

    #[test]
    fn init_script_does_nothing_when_deleted_or_already_scripted() {
        let (mut e, refr) = init_script_fixture();
        let me = refr.addr();
        e.mem.set_u32(me + SCRIPT_ANSWER, 0x1000);
        // a script extra with a script instance (+0xc) is left alone
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 0xc, 5);
        returns(&mut e, 0x0041_0220, extra);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5730, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0041_0220), vec![vec![me + 0x44, 0xd]]);
        assert!(calls_to(&log, 0x0041_9ed0).is_empty());
        assert_eq!(calls_to(&log, SCOPE_GUARD_DTOR).len(), 1);
        // one without an instance is not
        e.mem.set_u32(extra + 0xc, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5730, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0041_9ed0).len(), 1);
        // a deleted reference does nothing, not even the guard
        e.mem.set_u32(me + 8, 0x20);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5730, &args![refr]);
        });
        assert!(calls_to(&log, SCOPE_GUARD_CTOR).is_empty());
    }

    #[test]
    fn init_script_initialises_the_inventory_changes_of_a_container() {
        let (mut e, refr) = init_script_fixture();
        let changes = e.mem.alloc(0x10);
        returns(&mut e, 0x0055_d310, 1);
        returns(&mut e, 0x004b_f220, changes);
        // no script at all
        e.register(0x0048_26d0, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            e.call(0x0056_5730, &args![refr]);
        });
        assert!(calls_to(&log, 0x0041_9ed0).is_empty());
        assert_eq!(calls_to(&log, 0x004d_1960), vec![vec![changes]]);
    }

    // ---- 00565870 ---------------------------------------------------------

    fn run_script_fixture(kind: u8) -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = engine();
        let base = form(&mut e, kind, 1, 0);
        let refr = full_refr(&mut e, base);
        returns(&mut e, 0x008d_8520, 0);
        returns(&mut e, 0x0093_1850, 0);
        returns(&mut e, 0x0055_d310, 0);
        returns(&mut e, 0x004b_f220, 0);
        returns(&mut e, 0x004d_2480, 0);
        returns(&mut e, 0x0041_8800, 0);
        returns(&mut e, 0x0041_8830, 0);
        returns(&mut e, 0x005a_c1e0, 0);
        returns(&mut e, 0x0042_ede0, 0);
        (e, refr)
    }

    #[test]
    fn run_script_runs_the_reference_script_and_reports_it() {
        let (mut e, refr) = run_script_fixture(0x40);
        let me = refr.addr();
        let script = e.mem.alloc(0x10);
        let variables = e.mem.alloc(0x10);
        returns(&mut e, 0x0041_8800, script);
        returns(&mut e, 0x0041_8830, variables);
        returns(&mut e, 0x005a_c1e0, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_5870, &args![refr]).bool());
        });
        assert_eq!(
            calls_to(&log, 0x005a_c1e0),
            vec![vec![script, me, variables, 0, 0]]
        );
        // a script that did not run, with variables, flags the reference
        returns(&mut e, 0x005a_c1e0, 0);
        let flagged = slot_address(&e, me, 0x48);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0056_5870, &args![refr]).bool());
        });
        assert_eq!(calls_to(&log, flagged), vec![vec![me, 0x8000_0000]]);
        // ... without variables it does not
        returns(&mut e, 0x0041_8830, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5870, &args![refr]);
        });
        assert!(calls_to(&log, flagged).is_empty());
    }

    #[test]
    fn run_script_calls_the_say_to_handler_unless_the_virtual_fc_holds() {
        let (mut e, refr) = run_script_fixture(0x40);
        let me = refr.addr();
        let script = e.mem.alloc(0x10);
        returns(&mut e, 0x0041_8800, script);
        returns(&mut e, 0x0042_ede0, 5);
        let handler = slot_address(&e, me, 0x140);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5870, &args![refr]);
        });
        assert_eq!(calls_to(&log, handler), vec![vec![me]]);
        e.mem.set_u32(me + SLOT_FC_ANSWER, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5870, &args![refr]);
        });
        assert!(calls_to(&log, handler).is_empty());
        // no script: none of it
        returns(&mut e, 0x0041_8800, 0);
        e.mem.set_u32(me + SLOT_FC_ANSWER, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_5870, &args![refr]);
        });
        assert!(calls_to(&log, handler).is_empty());
    }

    #[test]
    fn run_script_runs_the_inventory_scripts_for_containers_of_some_types() {
        for (kind, expected_container_scan) in [(0x1bu8, true), (0x2a, true), (0x40, false)] {
            let (mut e, refr) = run_script_fixture(kind);
            let me = refr.addr();
            let changes = e.mem.alloc(0x10);
            returns(&mut e, 0x0055_d310, 1);
            returns(&mut e, 0x004b_f220, changes);
            returns(&mut e, 0x004d_2480, 1);
            // the process of an actor-like reference has a 3D (virtual +0x1d0)
            returns(&mut e, 0x008d_8520, 1);
            e.mem.set_u32(me + GET_3D_ANSWER, 0x55);
            let log = logged(&mut e, |e| {
                assert_eq!(
                    e.call(0x0056_5870, &args![refr]).bool(),
                    expected_container_scan,
                    "type {kind:#x}"
                );
            });
            assert_eq!(
                calls_to(&log, 0x004d_2480),
                if expected_container_scan {
                    vec![vec![changes, me]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn run_script_gives_up_on_an_actor_like_reference_without_3d() {
        let (mut e, refr) = run_script_fixture(0x2b);
        let me = refr.addr();
        returns(&mut e, 0x008d_8520, 1);
        returns(&mut e, 0x0055_d310, 1);
        returns(&mut e, 0x004b_f220, 0x99);
        returns(&mut e, 0x004d_2480, 1);
        // no 3D (virtual +0x1d0 gives 0): nothing runs
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0056_5870, &args![refr]).bool());
        });
        assert!(calls_to(&log, 0x004d_2480).is_empty());
        assert!(calls_to(&log, 0x0041_8800).is_empty());
        // 00931850 true, or a 3D, lets it go on
        returns(&mut e, 0x0093_1850, 1);
        assert!(e.call(0x0056_5870, &args![refr]).bool());
        returns(&mut e, 0x0093_1850, 0);
        e.mem.set_u32(me + GET_3D_ANSWER, 5);
        assert!(e.call(0x0056_5870, &args![refr]).bool());
        // no process at all: it goes on too
        e.mem.set_u32(me + GET_3D_ANSWER, 0);
        returns(&mut e, 0x008d_8520, 0);
        assert!(e.call(0x0056_5870, &args![refr]).bool());
        // deleted, or without a base form: nothing
        e.mem.set_u32(me + 8, 0x20);
        assert!(!e.call(0x0056_5870, &args![refr]).bool());
        e.mem.set_u32(me + 8, 0);
        e.mem.set_u32(me + 0x20, 0);
        assert!(!e.call(0x0056_5870, &args![refr]).bool());
    }

    // ---- 00566910, 00566930 -----------------------------------------------

    #[test]
    fn fn_00566910_reads_the_byte_at_0x68() {
        let mut e = engine();
        let object = e.mem.alloc(0x80);
        assert_eq!(e.call(0x0056_6910, &args![object]).u8(), 0);
        e.mem.set_u8(object + 0x68, 0x5a);
        assert_eq!(e.call(0x0056_6910, &args![object]).u8(), 0x5a);
    }

    #[test]
    fn fn_00566930_passes_the_flag_and_0x40_on() {
        let mut e = engine();
        stub(&mut e, &[0x0047_aa60]);
        let object = e.mem.alloc(0x80);
        let log = logged(&mut e, |e| {
            e.call(0x0056_6930, &args![object, 1u32]);
        });
        assert_eq!(calls_to(&log, 0x0047_aa60), vec![vec![object, 1, 0x40]]);
    }

    // ---- 005672c0 ---------------------------------------------------------

    fn end_sequence_fixture(kind: u8) -> (Engine, Ptr<TESObjectREFR>, u32, u32) {
        let mut e = engine();
        sequence_api(&mut e);
        let unequip = cstr(&mut e, "Unequip");
        e.set_global(NAME_UNEQUIP, unequip);
        e.register(0x0040_4dc0, |e, a| {
            (e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        let cell = e.mem.alloc(0x20);
        returns(&mut e, 0x0045_43c0, 1);
        let node = e.mem.alloc(0x20);
        let manager = e.mem.alloc(0x20);
        returns(&mut e, 0x0045_37b0, manager);
        returns(&mut e, 0x008c_7aa0, 0);
        stub(&mut e, &[0x00c6_a270, 0x00c6_a350, 0x0087_b1e0]);
        let base = form(&mut e, kind, 1, 0);
        let refr = full_refr(&mut e, base);
        e.mem.set_u32(refr.addr() + 0x40, cell);
        e.mem.set_u32(refr.addr() + GET_3D_ANSWER, node);
        (e, refr, node, manager)
    }

    #[test]
    fn end_sequence_resets_the_3d_when_the_unequip_sequence_ends() {
        let (mut e, refr, node, _) = end_sequence_fixture(0x15);
        let sequence = sequence(&mut e, "Unequip", false);
        let notified = slot_address(&e, refr.addr(), 0x48);
        let log = logged(&mut e, |e| {
            e.call(0x0056_72c0, &args![refr, sequence]);
        });
        assert_eq!(calls_to(&log, 0x00c6_a270), vec![vec![node, 1, 1, 1]]);
        // without 008c7aa0 the 3D's motion is set
        assert_eq!(calls_to(&log, 0x00c6_a350), vec![vec![node, 1, 1, 0, 1]]);
        assert!(calls_to(&log, 0x0087_b1e0).is_empty());
        assert_eq!(calls_to(&log, notified), vec![vec![refr.addr(), 4]]);
    }

    #[test]
    fn end_sequence_tells_the_manager_object_when_008c7aa0_holds() {
        let (mut e, refr, node, manager) = end_sequence_fixture(0x1c);
        returns(&mut e, 0x008c_7aa0, 1);
        let sequence = sequence(&mut e, "Unequip", false);
        let log = logged(&mut e, |e| {
            e.call(0x0056_72c0, &args![refr, sequence]);
        });
        assert_eq!(calls_to(&log, 0x0087_b1e0), vec![vec![manager, node, 1]]);
        assert!(calls_to(&log, 0x00c6_a350).is_empty());
    }

    #[test]
    fn end_sequence_ignores_other_sequences_types_and_cells() {
        let (mut e, refr, _, _) = end_sequence_fixture(0x15);
        let other = sequence(&mut e, "Equip", false);
        let log = logged(&mut e, |e| {
            e.call(0x0056_72c0, &args![refr, other]);
        });
        assert!(calls_to(&log, 0x00c6_a270).is_empty());
        // another base form type
        let (mut e, refr, _, _) = end_sequence_fixture(0x16);
        let unequip = sequence(&mut e, "Unequip", false);
        let log = logged(&mut e, |e| {
            e.call(0x0056_72c0, &args![refr, unequip]);
        });
        assert!(calls_to(&log, 0x00c6_a270).is_empty());
        // a cell that 004543c0 refuses, or none
        let (mut e, refr, _, _) = end_sequence_fixture(0x15);
        let unequip = sequence(&mut e, "Unequip", false);
        returns(&mut e, 0x0045_43c0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_72c0, &args![refr, unequip]);
        });
        assert!(calls_to(&log, 0x00c6_a270).is_empty());
        e.mem.set_u32(refr.addr() + 0x40, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_72c0, &args![refr, unequip]);
        });
        assert!(calls_to(&log, 0x0045_43c0).is_empty());
    }

    // ---- 005673c0, 005673e0, 00567400 ---------------------------------------

    #[test]
    fn script_getters_read_the_extra_list() {
        let mut e = engine();
        let refr = refr_of_kind(&mut e, 0x40);
        returns(&mut e, 0x0041_8800, 0xa1);
        returns(&mut e, 0x0041_8830, 0xb2);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_73c0, &args![refr]).u32(), 0xa1);
            assert_eq!(e.call(0x0056_73e0, &args![refr]).u32(), 0xb2);
        });
        assert_eq!(calls_to(&log, 0x0041_8800), vec![vec![refr.addr() + 0x44]]);
        assert_eq!(calls_to(&log, 0x0041_8830), vec![vec![refr.addr() + 0x44]]);
    }

    #[test]
    fn get_scale_multiplies_by_the_base_scale_of_actors() {
        let mut e = engine();
        returns_float(&mut e, 0x0094_4300, 1.5);
        for (kind, expected) in [(0x2au8, 3.0f32), (0x2b, 1.0), (0x40, 2.0)] {
            let refr = refr_of_kind(&mut e, kind);
            // the base form of type 0x2b keeps its own scale at +0x13c
            let base = e.mem.alloc(0x200);
            e.mem.set_u8(base + 4, kind);
            e.mem.set_f32(base + 0x13c, 0.5);
            e.mem.set_u32(refr.addr() + 0x20, base);
            e.mem.set_f32(refr.addr() + 0x3c, 2.0);
            assert_eq!(e.call(0x0056_7400, &args![refr]).f32(), expected);
        }
        // no base form: the plain scale
        let bare = refr_with(&mut e, 0, 0, &[]);
        e.mem.set_f32(bare.addr() + 0x3c, 0.75);
        assert_eq!(e.call(0x0056_7400, &args![bare]).f32(), 0.75);
    }

    // ---- 00567050 ---------------------------------------------------------

    /// A reference over a base form of `kind` whose loaded 3D has a controller;
    /// the controller's virtual +0x88 gives the dword at +0x10 of the
    /// controller (the test stores its manager there). The 3D's own virtual
    /// +0xc (its node) gives 0.
    fn step_fixture(kind: u8) -> (Engine, Ptr<TESObjectREFR>, u32) {
        let mut e = engine();
        sequence_api(&mut e);
        let base = form(&mut e, kind, 1, 0);
        let refr = full_refr(&mut e, base);
        let loaded = object_with_vtable(&mut e, 0x40, &[(0xc, |_, _| Ret::default())]);
        let controller = object_with_vtable(&mut e, 0x20, &[(0x88, slot_value::<0x10>)]);
        returns(&mut e, GET_LOADED_3D, loaded);
        e.register(GET_CONTROLLER, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.mem.set_u32(loaded + 0x10, controller);
        stub(
            &mut e,
            &[
                MANAGER_FINISH_SEQUENCE,
                MANAGER_SET_FLAG,
                0x0057_8a30,
                0x0047_ac70,
                SET_ANIMATION,
                ANIMATION_UPDATE,
            ],
        );
        // 004eef00(sequence, reference): the dword at +0x38 says "finished"
        e.register(0x004e_ef00, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        (e, refr, controller)
    }

    #[test]
    fn fn_00567050_ends_the_finished_sequences_and_keeps_the_manager_on_for_the_rest() {
        let (mut e, refr, controller) = step_fixture(0x40);
        let me = refr.addr();
        let finished = sequence(&mut e, "Done", true);
        e.mem.set_u32(finished + 0x38, 1);
        let running = sequence(&mut e, "Running", true);
        let plain = sequence(&mut e, "Plain", false);
        let manager = manager(&mut e, &[finished, running, plain]);
        e.mem.set_u32(controller + 0x10, manager);
        let name = e.mem.u32(finished + 8);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        // only the generic sequences are asked
        assert_eq!(
            calls_to(&log, 0x004e_ef00),
            vec![vec![finished, me], vec![running, me]]
        );
        assert_eq!(
            calls_to(&log, MANAGER_FINISH_SEQUENCE),
            vec![vec![manager, finished, 0.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x0057_8a30), vec![vec![me, name]]);
        assert_eq!(calls_to(&log, 0x0047_ac70), vec![vec![me, finished]]);
        // one is still running: the manager stays on
        assert!(calls_to(&log, MANAGER_SET_FLAG).is_empty());

        // all finished: the manager is switched off
        e.mem.set_u32(running + 0x38, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        assert_eq!(calls_to(&log, MANAGER_SET_FLAG), vec![vec![manager, 0]]);
        assert_eq!(calls_to(&log, 0x0047_ac70).len(), 2);
    }

    #[test]
    fn fn_00567050_calls_the_virtual_178_for_type_1e_and_skips_deleted_references() {
        let (mut e, refr, _) = step_fixture(0x1e);
        let me = refr.addr();
        let hook = slot_address(&e, me, 0x178);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        assert_eq!(calls_to(&log, hook), vec![vec![me]]);
        for flags in [0x20u32, 0x800] {
            e.mem.set_u32(me + 8, flags);
            let log = logged(&mut e, |e| {
                e.call(0x0056_7050, &args![refr]);
            });
            assert!(calls_to(&log, GET_LOADED_3D).is_empty(), "{flags:#x}");
            assert!(calls_to(&log, hook).is_empty());
        }
    }

    #[test]
    fn fn_00567050_updates_the_animation_of_types_2a_and_2b() {
        let (mut e, refr, _) = step_fixture(0x2a);
        let me = refr.addr();
        returns(&mut e, 0x0096_11e0, 1);
        returns_float(&mut e, FRAME_TIME, 0.5);
        returns_float(&mut e, TARGET_UPDATE_MULT, 2.0);
        let animation = e.mem.alloc(0x20);
        e.mem.set_u32(me + ANIMATION_ANSWER, animation);
        let minus_one = (-1.0f32).to_bits();
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        // the frame time is passed on as it is
        assert_eq!(
            calls_to(&log, ANIMATION_UPDATE),
            vec![vec![animation, me, 0.5f32.to_bits(), minus_one]]
        );
        // the sequences of the controller manager are not looked at
        assert!(calls_to(&log, GET_CONTROLLER).is_empty());

        // VATS in mode 4 with this reference scales the step
        e.mem.set_u32(VATS_OBJECT + 8, 4);
        e.set_global(REFERENCE_COMPARED_WITH_VATS, me);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        assert_eq!(
            calls_to(&log, ANIMATION_UPDATE),
            vec![vec![animation, me, 1.0f32.to_bits(), minus_one]]
        );
        // another reference is not scaled
        e.set_global(REFERENCE_COMPARED_WITH_VATS, me + 4);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        assert_eq!(calls_to(&log, ANIMATION_UPDATE)[0][2], 0.5f32.to_bits());

        // a 3D that cannot be updated: nothing
        returns(&mut e, 0x0096_11e0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        assert!(calls_to(&log, ANIMATION_UPDATE).is_empty());
    }

    #[test]
    fn fn_00567050_initialises_a_missing_animation() {
        let (mut e, refr, _) = step_fixture(0x2b);
        let me = refr.addr();
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        // InitAnimation clears the animation first, then finds no node
        assert_eq!(calls_to(&log, SET_ANIMATION), vec![vec![me, 0]]);
        assert!(calls_to(&log, ANIMATION_UPDATE).is_empty());
        // no 3D at all: no initialisation
        returns(&mut e, GET_LOADED_3D, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7050, &args![refr]);
        });
        assert!(calls_to(&log, SET_ANIMATION).is_empty());
    }

    // ---- the file lists of 00566970 ---------------------------------------

    /// The doubles of the KF file list builders. The model of a reference is
    /// the C string at +0x98; every file list added is recorded as (path as
    /// it is at that moment, model, list) in the returned log.
    fn kf_doubles(e: &mut Engine) -> std::rc::Rc<std::cell::RefCell<Vec<(String, String, u32)>>> {
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        for page in [0x0101_6000, 0x0101_d000, 0x0103_0000] {
            e.map(page, 0x1000);
        }
        for (address, text) in [
            (PATH_DATA, "Data\\"),
            (PATH_MESHES, "Meshes"),
            (PATH_BACKSLASH, "\\"),
            (PATH_MT_IDLE, "\\MTIdle.KF"),
            (PATH_HOLSTER, "\\%sHolster.KF"),
            (PATH_POWER_ARMOR_HOLSTER, "\\PA%sHolster.KF"),
            (PATH_DEATH, "\\Death.KF"),
            (PATH_CHILD_IDLES, "\\Locomotion\\Child\\IdleAnims"),
            (PATH_FEMALE_IDLES, "\\Locomotion\\Female\\IdleAnims"),
            (PATH_MALE_IDLES, "\\Locomotion\\Male\\IdleAnims"),
            (
                PATH_FULL_MALE_LOCOMOTION,
                "Data\\Meshes\\Locomotion\\MTIdle.kf",
            ),
            (PATH_MALE_LOCOMOTION, "Locomotion\\MTIdle.kf"),
            (
                PATH_FULL_MALE_IDLE,
                "Data\\Meshes\\Characters\\_Male\\MTIdle.kf",
            ),
        ] {
            e.mem.set_cstr(address, text.as_bytes());
        }
        let loader = e.mem.alloc(0x20);
        e.set_global(MODEL_LOADER, loader);
        let idle_manager = e.mem.alloc(0x20);
        e.set_global(IDLE_MANAGER, idle_manager);
        e.register(GET_MODEL, |e, a| e.mem.u32(a[0] + 0x98).into_ret());
        e.register(BOUNDED_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(BOUNDED_APPEND, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRRCHR_WRAPPER, |e, a| {
            let text = e.mem.cstr(a[0]);
            text.iter()
                .rposition(|c| *c == b'\\')
                .map_or(0, |at| a[0] + at as u32)
                .into_ret()
        });
        // sprintf with one %s
        e.register(SPRINTF, |e, a| {
            let format = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            let argument = String::from_utf8(e.mem.cstr(a[2])).unwrap();
            e.mem
                .set_cstr(a[0], format.replace("%s", &argument).as_bytes());
            Ret::default()
        });
        returns(e, BUILD_KF_FILE_LIST, 0x1111);
        let log = added.clone();
        e.register_double(BUILD_FILE_LIST, move |e, a| {
            let path = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            let model = String::from_utf8(e.mem.cstr(a[2])).unwrap_or_default();
            log.borrow_mut().push((path, model, a[3]));
            Ret {
                eax: 0x2222,
                ..Ret::default()
            }
        });
        returns(e, ROOT_FILENAME_LIST, 0x3333);
        stub(e, &[COPY_FILENAME_LIST]);
        returns(e, ACTOR_CURRENT_WEAPON, 0);
        e.register(WEAPON_INDEX, |_, a| a[0].into_ret());
        added
    }

    /// A base form that is an object with a vtable (name pointer at +0x100);
    /// its embedded model object at +0xdc has a vtable whose +0x14 gives the
    /// model path stored at +0xe4.
    fn kf_base(e: &mut Engine, kind: u8, id: u32, model: &str) -> u32 {
        let base = named_base(e, kind, id, "Base");
        let embedded = object_with_vtable(e, 0x10, &[(0x14, slot_value::<0x8>)]);
        let vtable = e.mem.u32(embedded);
        e.mem.set_u32(base + 0xdc, vtable);
        let text = cstr(e, model);
        e.mem.set_u32(base + 0xe4, text);
        base
    }

    fn set_model(e: &mut Engine, refr: Ptr<TESObjectREFR>, model: &str) {
        let text = cstr(e, model);
        e.mem.set_u32(refr.addr() + 0x98, text);
    }

    #[test]
    fn build_kf_file_list_for_type_2a_with_id_7_copies_the_idle_lists() {
        for (child, female, idles) in [
            (1u32, 0u32, "\\Locomotion\\Child\\IdleAnims"),
            (0, 1, "\\Locomotion\\Female\\IdleAnims"),
            (0, 0, "\\Locomotion\\Male\\IdleAnims"),
        ] {
            let mut e = engine();
            let _ = kf_doubles(&mut e);
            let base = kf_base(&mut e, 0x2a, 7, "Characters\\Actor\\skeleton.nif");
            let refr = full_refr(&mut e, base);
            e.mem.set_u32(refr.addr() + SLOT_1A0_ANSWER, child);
            returns(&mut e, 0x005f_0cc0, female);
            // the idle list builder sees the path as it is then
            let paths = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let seen = paths.clone();
            e.register_double(ROOT_FILENAME_LIST, move |e, a| {
                seen.borrow_mut()
                    .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
                Ret {
                    eax: 0x3333,
                    ..Ret::default()
                }
            });
            let log = logged(&mut e, |e| {
                let list = e.call(0x0056_6970, &args![refr, 5u32]).u32();
                assert_eq!(list, 0x1111);
            });
            let loader = e.global::<u32>(MODEL_LOADER);
            // the base form's model, with the selector the caller gave
            let model = e.mem.u32(base + 0xe4);
            assert_eq!(
                calls_to(&log, BUILD_KF_FILE_LIST),
                vec![vec![loader, model, 1, 1, 5]]
            );
            assert_eq!(*paths.borrow(), vec![format!("Characters\\Actor{idles}")]);
            let idle_manager = e.global::<u32>(IDLE_MANAGER);
            assert_eq!(calls_to(&log, ROOT_FILENAME_LIST)[0][0..1], [idle_manager]);
            assert_eq!(
                calls_to(&log, COPY_FILENAME_LIST),
                vec![vec![loader, 0x3333, 0x1111]]
            );
            // the room left in the buffer: 0x104 less the directory, less 1
            let copies = calls_to(&log, BOUNDED_COPY);
            assert_eq!(copies[1][1], 0x104 - "Characters\\Actor".len() as u32 - 1);
        }
    }

    #[test]
    fn build_kf_file_list_for_type_2a_takes_the_idle_path_without_the_virtual_22c() {
        let mut e = engine();
        let _ = kf_doubles(&mut e);
        let base = kf_base(&mut e, 0x2a, 9, "Characters\\Actor\\skeleton.nif");
        let refr = full_refr(&mut e, base);
        returns(&mut e, 0x005f_0cc0, 0);
        // the virtual +0x22c(1) says no: the same path as for form ID 7
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_6970, &args![refr, 3u32]).u32(), 0x1111);
        });
        assert_eq!(calls_to(&log, BUILD_KF_FILE_LIST).len(), 1);
        assert!(calls_to(&log, BUILD_FILE_LIST).is_empty());
        assert_eq!(calls_to(&log, COPY_FILENAME_LIST).len(), 1);
    }

    #[test]
    fn build_kf_file_list_for_type_2a_builds_the_male_locomotion_and_the_weapon_files() {
        let mut e = engine();
        let added = kf_doubles(&mut e);
        let base = kf_base(&mut e, 0x2a, 9, "x");
        let refr = full_refr(&mut e, base);
        set_model(&mut e, refr, "m");
        e.mem.set_u32(refr.addr() + SLOT_22C_ARG1_ANSWER, 1);
        // weapon 2: table index -> table entry 3 -> holster prefix "Rifle"
        let prefix = cstr(&mut e, "Rifle");
        e.mem.set_u32(WEAPON_INDEX_TABLE + 8, 3);
        e.mem.set_u32(HOLSTER_PREFIX_TABLE + 12, prefix);
        returns(&mut e, ACTOR_CURRENT_WEAPON, 2);
        let log = logged(&mut e, |e| {
            // the list the first call builds is the result
            assert_eq!(e.call(0x0056_6970, &args![refr, 0u32]).u32(), 0x2222);
        });
        let loader = e.global::<u32>(MODEL_LOADER);
        assert!(calls_to(&log, BUILD_KF_FILE_LIST).is_empty());
        let paths: Vec<(String, u32)> = added
            .borrow()
            .iter()
            .map(|(path, model, list)| (format!("{path}|{model}"), *list))
            .collect();
        // (path, second path) first; the weapon and death files are added to it
        assert_eq!(
            paths[0],
            (
                "Data\\Meshes\\Locomotion\\MTIdle.kf|Locomotion\\MTIdle.kf".to_string(),
                0
            )
        );
        let holsters: Vec<&str> = paths[1..].iter().map(|(p, _)| p.as_str()).collect();
        assert_eq!(
            holsters,
            vec![
                "Data\\Meshes\\Characters\\_Male\\RifleHolster.KF|m",
                "Data\\Meshes\\Characters\\_Male\\PARifleHolster.KF|m",
                "Data\\Meshes\\Characters\\_Male\\Death.KF|m",
            ]
        );
        assert!(paths[1..].iter().all(|(_, list)| *list == 0x2222));
        let _ = loader;
        // without a weapon prefix only the death file follows
        let mut e = engine();
        let added = kf_doubles(&mut e);
        let base = kf_base(&mut e, 0x2a, 9, "x");
        let refr = full_refr(&mut e, base);
        set_model(&mut e, refr, "m");
        e.mem.set_u32(refr.addr() + SLOT_22C_ARG1_ANSWER, 1);
        e.call(0x0056_6970, &args![refr, 0u32]);
        assert_eq!(added.borrow().len(), 2);
        assert!(added.borrow()[1].0.ends_with("Death.KF"));
    }

    /// A type 0x2b reference whose process (virtual +0x148 gives the dword
    /// at +0x10 of it, the weapon) is at `process`.
    fn kf_actor(e: &mut Engine) -> (Ptr<TESObjectREFR>, u32, u32) {
        let base = kf_base(e, 0x2b, 9, "Characters\\Dog\\skeleton.nif");
        let refr = full_refr(e, base);
        set_model(e, refr, "Characters\\Dog\\skeleton.nif");
        let process = object_with_vtable(e, 0x20, &[(0x148, slot_value::<0x10>)]);
        returns(e, ACTOR_PROCESS, process);
        returns(e, PROCESS_LEVEL, 0);
        returns(e, 0x008a_6970, 0);
        (refr, base, process)
    }

    #[test]
    fn build_kf_file_list_for_type_2b_picks_the_selector_from_the_weapon() {
        let mut e = engine();
        let _ = kf_doubles(&mut e);
        let (refr, _, process) = kf_actor(&mut e);
        // a weapon whose flags (+8) are 2: the table entry 0x66
        let weapon = form(&mut e, 0, 0, 2);
        e.mem.set_u32(process + 0x10, weapon);
        e.mem.set_u32(WEAPON_INDEX_TABLE + 8, 0x66);
        let model = e.mem.u32(refr.addr() + 0x98);
        let loader = e.global::<u32>(MODEL_LOADER);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_6970, &args![refr, 5u32]).u32(), 0x1111);
        });
        assert_eq!(
            calls_to(&log, BUILD_KF_FILE_LIST),
            vec![vec![loader, model, 1, 1, 0x66]]
        );
        // no weapon, but 008a6970: the selector 1
        e.mem.set_u32(process + 0x10, 0);
        returns(&mut e, 0x008a_6970, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_6970, &args![refr, 5u32]);
        });
        assert_eq!(calls_to(&log, BUILD_KF_FILE_LIST)[0][4], 1);
        // neither: the caller's selector; and with the virtual +0x22c(0) true,
        // or a process level out of range, it is never changed
        returns(&mut e, 0x008a_6970, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_6970, &args![refr, 5u32]);
        });
        assert_eq!(calls_to(&log, BUILD_KF_FILE_LIST)[0][4], 5);
        e.mem.set_u32(process + 0x10, weapon);
        e.mem.set_u32(refr.addr() + SLOT_22C_ARG0_ANSWER, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_6970, &args![refr, 5u32]);
        });
        assert_eq!(calls_to(&log, BUILD_KF_FILE_LIST)[0][4], 5);
        e.mem.set_u32(refr.addr() + SLOT_22C_ARG0_ANSWER, 0);
        returns(&mut e, PROCESS_LEVEL, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0056_6970, &args![refr, 5u32]);
        });
        assert_eq!(calls_to(&log, BUILD_KF_FILE_LIST)[0][4], 5);
    }

    #[test]
    fn build_kf_file_list_for_type_2b_builds_the_mesh_path_when_the_virtual_22c_holds() {
        let mut e = engine();
        let added = kf_doubles(&mut e);
        let (refr, _, _) = kf_actor(&mut e);
        e.mem.set_u32(refr.addr() + SLOT_22C_ARG1_ANSWER, 1);
        let prefix = cstr(&mut e, "Pistol");
        e.mem.set_u32(WEAPON_INDEX_TABLE, 1);
        e.mem.set_u32(HOLSTER_PREFIX_TABLE + 4, prefix);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_6970, &args![refr, 0u32]).u32(), 0x2222);
        });
        assert!(calls_to(&log, BUILD_KF_FILE_LIST).is_empty());
        let paths: Vec<(String, String, u32)> = added.borrow().clone();
        let model = "Characters\\Dog\\skeleton.nif".to_string();
        // Data\Meshes\<model> with the file name replaced by \MTIdle.KF
        assert_eq!(
            paths[0],
            (
                "Data\\Meshes\\Characters\\Dog\\MTIdle.KF".to_string(),
                model.clone(),
                0
            )
        );
        assert_eq!(
            paths[1].0,
            "Data\\Meshes\\Characters\\Dog\\PistolHolster.KF"
        );
        assert_eq!(
            paths[2].0,
            "Data\\Meshes\\Characters\\Dog\\PAPistolHolster.KF"
        );
        assert_eq!(paths[3].0, "Data\\Meshes\\Characters\\Dog\\Death.KF");
        assert!(paths[1..].iter().all(|p| p.1 == model && p.2 == 0x2222));
    }

    #[test]
    fn build_kf_file_list_is_empty_for_other_types_and_without_a_base_form() {
        let mut e = engine();
        let _ = kf_doubles(&mut e);
        let other = kf_base(&mut e, 0x40, 9, "x");
        let refr = full_refr(&mut e, other);
        assert_eq!(e.call(0x0056_6970, &args![refr, 0u32]).u32(), 0);
        let bare = full_refr(&mut e, 0);
        assert_eq!(e.call(0x0056_6970, &args![bare, 0u32]).u32(), 0);
    }

    // ---- 005659f0 ---------------------------------------------------------

    /// The 3D object the world's `0043fcd0` gives: its virtual +0xc gives the
    /// node stored at +0x14.
    struct AnimationWorld {
        e: Engine,
        refr: Ptr<TESObjectREFR>,
        base: u32,
        loaded: u32,
        node: u32,
    }

    /// A reference over a base form of `kind` (ID 0x1234, name "Base") with
    /// a loaded 3D and a node, every callee of `InitAnimation` registered
    /// as a harmless double. The node's controller is at +0x10 of the node
    /// (none); `0042ce10` (the loading object's busy test) says no.
    fn animation_world(kind: u8) -> AnimationWorld {
        let mut e = engine();
        sequence_api(&mut e);
        let _ = kf_doubles(&mut e);
        e.mem.set_cstr(SKELETON_PREFIX, b"Skeleton");
        let base = kf_base(&mut e, kind, 0x1234, "Characters\\Actor\\skeleton.nif");
        let refr = full_refr(&mut e, base);
        set_model(&mut e, refr, "Characters\\Actor\\skeleton.nif");
        let node = object_with_vtable(&mut e, 0x40, &[(0xa4, |_, _| Ret::default())]);
        let loaded = object_with_vtable(&mut e, 0x40, &[(0xc, slot_value::<0x14>)]);
        e.mem.set_u32(loaded + 0x14, node);
        returns(&mut e, GET_LOADED_3D, loaded);
        e.register(SET_ANIMATION, |_, a| a[1].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ANIMATION_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            text.iter()
                .rposition(|c| *c == a[1] as u8)
                .map_or(0, |at| a[0] + at as u32)
                .into_ret()
        });
        e.register(STRNICMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            let count = a[2] as usize;
            let lower = |text: &[u8]| {
                text.iter()
                    .take(count)
                    .map(u8::to_ascii_lowercase)
                    .collect::<Vec<_>>()
            };
            u32::from(lower(&first) != lower(&second)).into_ret()
        });
        returns(&mut e, 0x008a_1760, 0x5151);
        returns(&mut e, 0x0048_ffd0, 1);
        returns_float(&mut e, 0x0056_8ad0, 0.0);
        returns(&mut e, 0x0047_fd90, 0);
        returns(&mut e, 0x0071_7e50, 0x7777);
        stub(
            &mut e,
            &[
                LOG_MESSAGE,
                0x004a_ae70,
                0x0057_10c0,
                0x0049_0330,
                0x005f_bdd0,
                ANIMATION_UPDATE,
                0x0048_4b90,
                0x0057_26e0,
                MANAGER_SET_FLAG,
                MANAGER_DEACTIVATE_ALL,
                MANAGER_ACTIVATE,
                SEQUENCE_SET_OFFSET,
                SET_3D_VELOCITY,
                FIXED_STRING_DESTRUCT,
                0x0041_ae10,
            ],
        );
        e.register(GET_CONTROLLER, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(CHECKED_CAST, |_, a| a[1].into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[1].into_ret());
        // the sequence of the manager whose name is the handle
        e.register(MANAGER_FIND_SEQUENCE, |e, a| {
            let count = e.mem.u32(a[0] + 4);
            for index in 0..count {
                let sequence = e.mem.u32(a[0] + 8 + 4 * index);
                if e.mem.u32(sequence + 8) == a[1] {
                    return sequence.into_ret();
                }
            }
            Ret::default()
        });
        returns(&mut e, 0x0047_a490, 0);
        returns(&mut e, 0x004c_0bf0, 0);
        returns(&mut e, 0x0059_bb30, 0);
        returns(&mut e, 0x0041_8200, 0);
        returns(&mut e, 0x0042_ce10, 0);
        returns(&mut e, ACTOR_PROCESS, 0);
        let loading = e.mem.alloc(0x20);
        e.set_global(GLOBAL_LOADING_OBJECT, loading);
        AnimationWorld {
            e,
            refr,
            base,
            loaded,
            node,
        }
    }

    #[test]
    fn init_animation_clears_the_animation_and_stops_without_a_node_or_when_deleted() {
        let mut world = animation_world(0x2a);
        let (me, loaded) = (world.refr.addr(), world.loaded);
        let e = &mut world.e;
        // no loaded 3D
        returns(e, GET_LOADED_3D, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, SET_ANIMATION), vec![vec![me, 0]]);
        assert!(calls_to(&log, GET_MODEL).is_empty());
        // a 3D without a node
        returns(e, GET_LOADED_3D, loaded);
        e.mem.set_u32(loaded + 0x14, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, SET_ANIMATION), vec![vec![me, 0]]);
        // a deleted reference
        e.mem.set_u32(loaded + 0x14, world.node);
        e.mem.set_u32(me + 8, 0x20);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, SET_ANIMATION), vec![vec![me, 0]]);
        assert!(calls_to(&log, GET_MODEL).is_empty());
    }

    #[test]
    fn init_animation_builds_the_npc_animation_and_its_special_animations() {
        let mut world = animation_world(0x2a);
        let (me, node, base) = (world.refr.addr(), world.node, world.base);
        let e = &mut world.e;
        e.mem.set_u32(me + BIPED_ANSWER, 0xb1);
        // base form value > 0 and KF files: the special animations are added
        returns_float(e, 0x0056_8ad0, 1.0);
        returns(e, 0x0047_fd90, 1);
        returns(e, 0x005f_0cc0, 0);
        let paths = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = paths.clone();
        e.register_double(0x0049_0330, move |e, a| {
            seen.borrow_mut()
                .push((a[1], String::from_utf8(e.mem.cstr(a[2])).unwrap()));
            Ret::default()
        });
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        let animation = calls_to(&log, ANIMATION_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x13c]]);
        assert_eq!(
            calls_to(&log, SET_ANIMATION),
            vec![vec![me, 0], vec![me, animation]]
        );
        // the biped loses its parts first; the skeleton is asked of the
        // actor (here none: the reference is not an actor)
        assert_eq!(calls_to(&log, 0x004a_ae70), vec![vec![0xb1]]);
        assert_eq!(calls_to(&log, 0x008a_1760), vec![vec![0]]);
        // the KF files: the base form's model, the skeleton as selector
        let loader = e.global::<u32>(MODEL_LOADER);
        let model = e.mem.u32(base + 0xe4);
        assert_eq!(
            calls_to(&log, BUILD_KF_FILE_LIST),
            vec![vec![loader, model, 1, 1, 0x5151]]
        );
        assert_eq!(
            calls_to(&log, 0x0048_ffd0),
            vec![vec![animation, 0x1111, node, me, 1]]
        );
        // the model's directory cut from the path
        assert_eq!(
            *paths.borrow(),
            vec![(0x7777, "Characters\\Actor".to_string())]
        );
        assert_eq!(calls_to(&log, 0x0071_7e50), vec![vec![base + 0xc4]]);
        // the animation is updated once, with a zero step
        assert_eq!(
            calls_to(&log, ANIMATION_UPDATE),
            vec![vec![animation, me, 0.0f32.to_bits(), (-1.0f32).to_bits()]]
        );
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
    }

    #[test]
    fn init_animation_reports_a_misnamed_skeleton_and_a_missing_idle() {
        let mut world = animation_world(0x2a);
        let (me, base) = (world.refr.addr(), world.base);
        let e = &mut world.e;
        e.mem.set_u32(me + BIPED_ANSWER, 0xb1);
        returns(e, 0x005f_0cc0, 0);
        returns(e, 0x0048_ffd0, 0);
        set_model(e, world.refr, "Characters\\Actor\\body.nif");
        let model = e.mem.u32(me + 0x98);
        let name = e.mem.u32(base + 0x100);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![
                vec![SKELETON_MESSAGE, name, model],
                vec![NPC_ANIMATION_MESSAGE, name, 0x1234]
            ]
        );
        // an NPC without a biped builds nothing
        e.mem.set_u32(me + BIPED_ANSWER, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, SET_ANIMATION), vec![vec![me, 0]]);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, ANIMATION_UPDATE).is_empty());
    }

    #[test]
    fn init_animation_builds_the_creature_animation_and_notices_a_lost_node() {
        let mut world = animation_world(0x2b);
        let (me, node, base, loaded) = (world.refr.addr(), world.node, world.base, world.loaded);
        let e = &mut world.e;
        returns_float(e, 0x0056_8ad0, 2.0);
        returns(e, 0x0047_fd90, 1);
        returns(e, 0x0048_ffd0, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        let animation = calls_to(&log, ANIMATION_CONSTRUCT)[0][0];
        // creatures have no biped parts to remove; the KF list comes from the
        // reference's own model
        assert!(calls_to(&log, 0x004a_ae70).is_empty());
        let model = e.mem.u32(me + 0x98);
        let loader = e.global::<u32>(MODEL_LOADER);
        assert_eq!(
            calls_to(&log, BUILD_KF_FILE_LIST),
            vec![vec![loader, model, 1, 1, 0x5151]]
        );
        assert_eq!(
            calls_to(&log, 0x0048_ffd0),
            vec![vec![animation, 0x1111, node, me, 1]]
        );
        let name = e.mem.u32(base + 0x100);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![CREATURE_ANIMATION_MESSAGE, name, 0x1234]]
        );
        // the base form gets the 3D at the end
        assert_eq!(calls_to(&log, 0x005f_bdd0), vec![vec![base, loaded]]);
        assert_eq!(calls_to(&log, ANIMATION_UPDATE).len(), 1);

        // the node vanishes when the animation is installed
        let mut calls = 0;
        e.register_double(GET_LOADED_3D, move |_, _| {
            calls += 1;
            Ret {
                eax: if calls == 1 { loaded } else { 0 },
                ..Ret::default()
            }
        });
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MOVED_MESSAGE, name, 0x1234]]
        );
        assert!(calls_to(&log, 0x005f_bdd0).is_empty());
        assert!(calls_to(&log, ANIMATION_UPDATE).is_empty());
    }

    #[test]
    fn init_animation_clears_the_motion_of_a_node_without_a_manager() {
        let mut world = animation_world(0x40);
        let (me, node) = (world.refr.addr(), world.node);
        let e = &mut world.e;
        let clear = slot_address(e, node, 0xa4);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        // an empty record is given to the node through its virtual +0xa4
        let record = calls_to(&log, MAKE_VELOCITY)[0][0];
        assert_eq!(
            calls_to(&log, MAKE_VELOCITY),
            vec![vec![record, 0.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(calls_to(&log, clear), vec![vec![node, record, 0]]);
        assert!(calls_to(&log, 0x0048_4b90).is_empty());
        assert!(calls_to(&log, ANIMATION_UPDATE).is_empty());
        // a controller that is not a manager is the same
        e.mem.set_u32(node + 0x10, 0x99);
        e.register(CHECKED_CAST, |_, _| Ret::default());
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, clear).len(), 1);
        assert!(calls_to(&log, 0x0048_4b90).is_empty());
        // a light base form (type 0x28) accepted by 004c0bf0 sets its light
        let light = animation_world(0x28);
        let mut e = light.e;
        returns(&mut e, 0x004c_0bf0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_59f0, &args![light.refr]);
        });
        assert_eq!(
            calls_to(&log, 0x0057_26e0),
            vec![vec![light.refr.addr(), light.node, 0]]
        );
        returns(&mut e, 0x004c_0bf0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_59f0, &args![light.refr]);
        });
        assert!(calls_to(&log, 0x0057_26e0).is_empty());
        let _ = me;
    }

    /// A world whose node has a controller (which the checked cast turns
    /// into itself) that is a manager with the sequences `idle_a`, `idle_b`
    /// and `other`; the idle names in the exe's data are those of the first
    /// two. Returns the world, the manager and the three sequences.
    fn manager_world(kind: u8) -> (AnimationWorld, u32, [u32; 3]) {
        let mut world = animation_world(kind);
        let e = &mut world.e;
        let idle_a = sequence(e, "IdleA", false);
        let idle_b = sequence(e, "IdleB", false);
        let other = sequence(e, "Other", false);
        e.mem.set_f32(idle_a + 0x2c, 1.5);
        e.mem.set_f32(idle_b + 0x2c, 3.0);
        e.mem.set_f32(other + 0x2c, 7.0);
        let manager = manager(e, &[other, idle_a, idle_b]);
        let name_a = e.mem.u32(idle_a + 8);
        let name_b = e.mem.u32(idle_b + 8);
        e.set_global(NAME_IDLE_A, name_a);
        e.set_global(NAME_IDLE_B, name_b);
        // the node's controller is the manager itself
        e.mem.set_u32(world.node + 0x10, manager);
        (world, manager, [idle_a, idle_b, other])
    }

    #[test]
    fn init_animation_starts_the_idle_sequences() {
        let (mut world, manager, [idle_a, idle_b, _]) = manager_world(0x40);
        let (me, node) = (world.refr.addr(), world.node);
        let e = &mut world.e;
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        // the form is marked changed
        assert_eq!(calls_to(&log, 0x0048_4b90), vec![vec![me, 0x1000_0000]]);
        let activate = |sequence: u32| {
            vec![
                manager,
                sequence,
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0,
            ]
        };
        assert_eq!(
            calls_to(&log, MANAGER_ACTIVATE),
            vec![activate(idle_a), activate(idle_b)]
        );
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![
                vec![idle_a, (-f32::MAX).to_bits()],
                vec![idle_b, (-f32::MAX).to_bits()]
            ]
        );
        // the node gets the record of the last idle's offset time
        let record = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(
            (record[0][1], record[0][2], record[0][3]),
            (3.0f32.to_bits(), 1, 0)
        );
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![node, record[0][0]]]
        );
        // the idle names were turned into fixed strings and looked up
        assert_eq!(calls_to(&log, MANAGER_FIND_SEQUENCE).len(), 2);
        assert_eq!(calls_to(&log, FIXED_STRING_DESTRUCT).len(), 2);
        // the manager is not switched on or off in this case
        assert!(calls_to(&log, MANAGER_SET_FLAG).is_empty());
    }

    #[test]
    fn init_animation_reports_an_idle_that_is_not_looping() {
        let (mut world, manager, [idle_a, idle_b, _]) = manager_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        e.register(0x0059_bb30, |_, _| 1u32.into_ret());
        let model = e.mem.u32(me + 0x98);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        // only the second idle is started; the first is reported
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![NOT_LOOPING_MESSAGE, model]]
        );
        let started: Vec<u32> = calls_to(&log, MANAGER_ACTIVATE)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(started, vec![idle_b]);
        assert_eq!(calls_to(&log, MAKE_VELOCITY)[0][1], 3.0f32.to_bits());
        // with only the first idle present (and reported), the record is zero
        e.set_global(NAME_IDLE_B, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, MAKE_VELOCITY)[0][1], 0.0f32.to_bits());
        let _ = (manager, idle_a);
    }

    #[test]
    fn init_animation_reports_a_cumulative_manager() {
        let (mut world, manager, _) = manager_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        let type_name = cstr(e, "Static");
        e.mem.set_u32(TYPE_NAME_TABLE + 0x40 * 12, type_name);
        let model = e.mem.u32(me + 0x98);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        e.mem.set_u8(manager + 0x68, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![CUMULATIVE_MESSAGE, type_name, model]]
        );
    }

    #[test]
    fn init_animation_starts_the_first_sequence_when_there_is_no_idle() {
        let (mut world, manager, [_, _, other]) = manager_world(0x40);
        let node = world.node;
        let e = &mut world.e;
        e.set_global(NAME_IDLE_A, 0);
        e.set_global(NAME_IDLE_B, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        // switched on, cleared, the first sequence (the manager's index 0)
        // started from its offset time, then switched off again
        assert_eq!(
            calls_to(&log, MANAGER_SET_FLAG),
            vec![vec![manager, 1], vec![manager, 0]]
        );
        assert_eq!(
            calls_to(&log, MANAGER_DEACTIVATE_ALL),
            vec![
                vec![manager, 0.0f32.to_bits()],
                vec![manager, 0.0f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&log, MANAGER_ACTIVATE)[0][1], other);
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![vec![other, (-f32::MAX).to_bits()]]
        );
        assert_eq!(calls_to(&log, MAKE_VELOCITY)[0][1], 7.0f32.to_bits());
        assert_eq!(calls_to(&log, SET_3D_VELOCITY)[0][0], node);
    }

    #[test]
    fn init_animation_restores_the_open_state_of_an_open_close_form() {
        let (mut world, manager, _) = manager_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        e.set_global(NAME_IDLE_A, 0);
        e.set_global(NAME_IDLE_B, 0);
        returns(e, 0x0047_a490, 1);
        // 00572d30(reference, 8): the extra flag; 00561d90: the default state
        returns(e, 0x0057_2d30, 0);
        returns(e, 0x0056_1d90, 1);
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        // 0084a6d0(loading, reference, flags): the saved change says "closed"
        returns(e, 0x0084_a6d0, 0);
        returns(e, 0x0047_aec0, 0);
        let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        // default open, saved change not set: the open state is 1
        assert_eq!(
            calls_to(&log, 0x0084_a6d0),
            vec![vec![loading, me, 0x80_0000]]
        );
        assert_eq!(calls_to(&log, 0x0047_aec0), vec![vec![me, 1, 1]]);
        // SetOpenState said no, so the first sequence is started as well
        assert_eq!(calls_to(&log, MANAGER_ACTIVATE).len(), 1);
        // when it says yes, the sequence start is skipped
        returns(e, 0x0047_aec0, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, MANAGER_ACTIVATE).is_empty());
        assert_eq!(calls_to(&log, MANAGER_SET_FLAG), vec![vec![manager, 0]]);
        // a busy loading object keeps the extra flag as the state
        returns(e, 0x0042_ce10, 1);
        returns(e, 0x0057_2d30, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, 0x0084_a6d0).is_empty());
        assert_eq!(calls_to(&log, 0x0047_aec0), vec![vec![me, 1, 1]]);
    }

    #[test]
    fn init_animation_restores_a_saved_animation_time() {
        let mut world = animation_world(0x2a);
        let (me, node, loaded) = (world.refr.addr(), world.node, world.loaded);
        let e = &mut world.e;
        e.mem.set_u32(me + BIPED_ANSWER, 0xb1);
        returns(e, 0x005f_0cc0, 0);
        let saved = e.mem.alloc(0x20);
        e.mem.set_f32(saved + 0x10, 2.5);
        returns(e, 0x0041_8200, saved);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(
            (made[0][1], made[0][2], made[0][3]),
            (2.5f32.to_bits(), 0, 0)
        );
        // given to the reference's loaded 3D, and the extra data removed
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![loaded, made[0][0]]]
        );
        assert_eq!(calls_to(&log, 0x0041_ae10), vec![vec![me + 0x44]]);
        // without a new animation (no biped) nothing is restored
        e.mem.set_u32(me + BIPED_ANSWER, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, 0x0041_ae10).is_empty());
        let _ = node;
    }

    /// The world of the actor tests: an actor (virtual +0x100 true) whose
    /// process is an object with the virtuals `InitAnimation` calls, at
    /// level 0, with a worn item of flags 0x33 and no loading in progress.
    fn actor_world(kind: u8) -> (AnimationWorld, u32, u32) {
        let mut world = animation_world(kind);
        let me = world.refr.addr();
        let e = &mut world.e;
        e.mem.set_u32(me + IS_ACTOR_ANSWER, 1);
        e.mem.set_u32(me + GET_3D_ANSWER, 0x5555);
        e.mem.set_u32(me + BIPED_ANSWER, 0xb1);
        let noop: AbiFn = |_, _| Ret::default();
        let process = object_with_vtable(
            e,
            0x40,
            &[
                (0x160, noop),
                (0x18c, noop),
                (0x1cc, noop),
                (0x22c, noop),
                (0x458, noop),
                (0x474, noop),
            ],
        );
        returns(e, ACTOR_PROCESS, process);
        returns(e, PROCESS_LEVEL, 0);
        let worn = form(e, 0x30, 1, 0x33);
        returns(e, 0x004b_f220, 0x77);
        returns(e, 0x004c_8c10, worn);
        returns(e, FORCE_NEXT_UPDATE, 0);
        returns(e, 0x008c_7aa0, 0);
        returns(e, 0x008a_6970, 0);
        returns(e, ACTOR_ANIMATION, 0xa1);
        returns(e, ACTOR_WEAPON_DRAWN, 1);
        returns(e, 0x00a5_c570, 0);
        e.register(0x0055_d520, |e, a| e.mem.u32(a[0] + 0xb0).into_ret());
        e.register(STRSTR, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            let needle = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            u32::from(text.contains(&needle)).into_ret()
        });
        e.mem.set_cstr(LILY_NAME, b"Lily");
        stub(e, &[0x0057_1760, 0x0087_b360, 0x0047_aa60]);
        let manager_object = e.mem.alloc(0x20);
        returns(e, 0x0045_37b0, manager_object);
        returns(e, 0x0047_b200, 0x4242);
        (world, process, worn)
    }

    #[test]
    fn init_animation_gives_an_actor_its_worn_weapon() {
        let (mut world, process, _) = actor_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        // the virtual +0x218 holds
        e.mem.set_u32(me + SLOT_218_ANSWER, 1);
        let gives = slot_address(e, process, 0x160);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        let worn = e.call(0x004c_8c10, &args![0x77u32, 5u32, 0u32]).u32();
        // the worn item (slot 5 of the inventory changes) and the 3D
        assert_eq!(calls_to(&log, 0x004c_8c10)[0], vec![0x77, 5, 0]);
        assert_eq!(calls_to(&log, gives), vec![vec![process, worn, 0x5555, 0]]);
        // not the manager object's attach: the plain weapon attach with the
        // worn item's flags
        assert_eq!(calls_to(&log, 0x0057_1760), vec![vec![me, 0x33]]);
        assert!(calls_to(&log, 0x0087_b360).is_empty());
        // the process refreshes its drawn state with the biped, the actor's
        // animation and the actor
        let drawn = slot_address(e, process, 0x1cc);
        assert_eq!(
            calls_to(&log, drawn),
            vec![vec![process, 1, 0xb1, 0xa1, me]]
        );
        assert_eq!(
            calls_to(&log, slot_address(e, process, 0x22c)),
            vec![vec![process]]
        );
        assert!(calls_to(&log, slot_address(e, process, 0x458)).is_empty());
    }

    #[test]
    fn init_animation_queues_the_weapon_attach_when_008c7aa0_holds() {
        let (mut world, _, _) = actor_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        e.mem.set_u32(me + SLOT_218_ANSWER, 1);
        returns(e, 0x008c_7aa0, 1);
        let manager_object = e.call(0x0045_37b0, &args![]).u32();
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(
            calls_to(&log, 0x0087_b360),
            vec![vec![manager_object, me, 0x33]]
        );
        assert!(calls_to(&log, 0x0057_1760).is_empty());
        // ... unless the reference is the one 0047b200 names
        returns(e, 0x0047_b200, me);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, 0x0087_b360).is_empty());
        assert_eq!(calls_to(&log, 0x0057_1760), vec![vec![me, 0x33]]);
    }

    #[test]
    fn init_animation_leaves_the_weapon_alone_for_lily_and_for_other_actors() {
        let (mut world, process, worn) = actor_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        // not +0x218, but the force-update test and the name "Lily"
        returns(e, FORCE_NEXT_UPDATE, 1);
        let name = cstr(e, "Lily Belle");
        e.mem.set_u32(me + 0xb0, name);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, STRSTR), vec![vec![name, LILY_NAME]]);
        assert!(calls_to(&log, 0x0057_1760).is_empty());
        assert!(calls_to(&log, 0x0087_b360).is_empty());
        // another name: neither condition holds, nothing is attached
        let other = cstr(e, "Someone");
        e.mem.set_u32(me + 0xb0, other);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, 0x0057_1760).is_empty());
        // a worn item without flags, or none at all, is not attached either
        e.mem.set_u32(me + SLOT_218_ANSWER, 1);
        e.mem.set_u32(worn + 8, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, 0x0057_1760).is_empty());
        e.mem.set_u32(worn + 8, 0x33);
        returns(e, 0x004c_8c10, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, 0x0057_1760).is_empty());
        let _ = process;
    }

    #[test]
    fn init_animation_skips_the_process_updates_for_the_player_and_while_loading() {
        let (mut world, process, _) = actor_world(0x40);
        let me = world.refr.addr();
        let e = &mut world.e;
        e.mem.set_u32(me + SLOT_218_ANSWER, 1);
        let gives = slot_address(e, process, 0x160);
        // the player is not given a worn item
        e.set_global(GLOBAL_PLAYER, me);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, gives).is_empty());
        e.set_global(GLOBAL_PLAYER, 0);
        // a process outside the levels 0 and 1 counts as none
        returns(e, PROCESS_LEVEL, 2);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, gives).is_empty());
        returns(e, PROCESS_LEVEL, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(calls_to(&log, gives).len(), 1);
        // while the loading object is busy only the +0x21c refresh remains
        returns(e, 0x0042_ce10, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, gives).is_empty());
        assert!(calls_to(&log, slot_address(e, process, 0x1cc)).is_empty());
        e.mem.set_u32(me + SLOT_21C_ANSWER, 1);
        returns(e, FORCE_NEXT_UPDATE, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(
            calls_to(&log, slot_address(e, process, 0x1cc)),
            vec![vec![process, 1, 0xb1, 0xa1, me]]
        );
    }

    #[test]
    fn init_animation_tells_the_process_about_a_new_animation_and_a_controller() {
        let (mut world, process, _) = actor_world(0x2a);
        let me = world.refr.addr();
        let e = &mut world.e;
        returns(e, 0x005f_0cc0, 0);
        returns(e, 0x0055_85e0, 0x61);
        returns(e, 0x0049_6940, 0x62);
        returns(e, 0x008a_6970, 1);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        // the animation's two words reach the process (virtual +0x18c)
        assert_eq!(
            calls_to(&log, slot_address(e, process, 0x18c)),
            vec![vec![process, 0x62, 0x61]]
        );
        assert_eq!(calls_to(&log, 0x0049_6940).len(), 1);
        // the actor flag of the process (virtual +0x458)
        assert_eq!(
            calls_to(&log, slot_address(e, process, 0x458)),
            vec![vec![process, me, 1]]
        );
        // no new animation or a null word: nothing
        returns(e, 0x0055_85e0, 0);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert!(calls_to(&log, slot_address(e, process, 0x18c)).is_empty());
        // the controller of the 3D is switched on
        returns(e, 0x00a5_c570, 0x66);
        let log = logged(e, |e| {
            e.call(0x0056_59f0, &args![world.refr]);
        });
        assert_eq!(
            calls_to(&log, 0x00a5_c570),
            vec![vec![world.node, CONTROLLER_RECORD]]
        );
        assert_eq!(calls_to(&log, 0x0047_aa60), vec![vec![0x66, 1, 0x40]]);
    }

    // ======================================================================
    // Second batch: 00567470 .. 00568fa0.

    /// Where the doubles of this batch keep what the extra list answers, in
    /// the list at `reference + 0x44`: owner +0, global +4, rank +8, link
    /// +0xc, encounter zone +0x10, the used / reserved / health extras
    /// +0x20 / +0x24 / +0x28.
    const LIST_OWNER: u32 = 0x44;
    const LIST_GLOBAL: u32 = 0x48;
    const LIST_RANK: u32 = 0x4c;
    const LIST_LINK: u32 = 0x50;
    const LIST_ZONE: u32 = 0x54;
    const LIST_USED: u32 = 0x64;
    const LIST_RESERVED: u32 = 0x68;
    const LIST_HEALTH: u32 = 0x6c;
    /// The cell pointer of a reference (`+0x40`), and where the doubles keep
    /// the world space (`+0x58`).
    const REFR_CELL: u32 = 0x40;
    const REFR_WORLD: u32 = 0x58;
    /// Where `ownership_refr` keeps the answers of its virtuals.
    const OWN_IS_ACTOR: u32 = 0x70;
    const OWN_NODE: u32 = 0x74;
    const OWN_ACTOR_BASE: u32 = 0x78;
    const DEFAULT_ZONE_ADDRESS: u32 = 0x7777_0000;

    fn noop_slot(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The location a reference answers (virtual `+0x1f4`): `data.Location`.
    fn location_slot(_: &mut Engine, a: &[u32]) -> Ret {
        (a[0] + 0x30).into_ret()
    }

    /// An engine whose extra list, cell, zone and world space accessors
    /// answer from the memory layout the constants above name.
    fn ownership_engine() -> Engine {
        let mut e = engine();
        e.register(EXTRA_GET_OWNER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(EXTRA_GET_GLOBAL, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EXTRA_GET_RANK, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(EXTRA_GET_LINK, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(EXTRA_GET_ZONE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(EXTRA_SET_OWNER, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(EXTRA_SET_GLOBAL, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(EXTRA_SET_RANK, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(EXTRA_SET_ZONE, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        e.register(EXTRA_SET_LINK, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        e.register(READ_FIRST_DWORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_PARENT_CELL, |e, a| {
            e.mem.u32(a[0] + REFR_CELL).into_ret()
        });
        e.register(GET_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + REFR_WORLD).into_ret()
        });
        e.register(WORLD_SPACE_ZONE, |e, a| e.mem.u32(a[0] + 0xd0).into_ret());
        e.register(CELL_GET_ZONE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(CELL_GET_OWNER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CELL_GET_GLOBAL, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CELL_GET_RANK, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ZONE_OWNER, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(DEFAULT_ZONE, |_, _| DEFAULT_ZONE_ADDRESS.into_ret());
        e
    }

    /// A reference over a base form of `kind` with an empty list (rank -1),
    /// whose virtuals IsActor (+0x100), Get3D (+0x1d0) and +0x1a4 answer with
    /// the dwords at `OWN_IS_ACTOR`, `OWN_NODE` and `OWN_ACTOR_BASE`; the
    /// slots +0x48 and +0x4c do nothing and +0x1f4 gives `data.Location`.
    fn ownership_refr(e: &mut Engine, kind: u8) -> Ptr<TESObjectREFR> {
        let base = form(e, kind, 1, 0);
        let slots: [(u32, AbiFn); 6] = [
            (0x100, slot_value::<OWN_IS_ACTOR>),
            (0x1d0, slot_value::<OWN_NODE>),
            (0x1a4, slot_value::<OWN_ACTOR_BASE>),
            (0x48, noop_slot),
            (0x4c, noop_slot),
            (0x1f4, location_slot),
        ];
        let refr = refr_with(e, base, 0, &slots);
        e.mem.set_u32(refr.addr() + LIST_RANK, u32::MAX);
        refr
    }

    // ---- 00567470, 00567730, 00567750, 00567ab0, 00568650, 00568ab0 -------

    #[test]
    fn small_field_getters_read_their_offsets() {
        let mut e = engine();
        let block = e.mem.alloc(0x600);
        e.set_global(HEADING_DIVISOR, 1000.0f64);
        e.mem.set_f32(block + 0x13c, 2.5);
        e.mem.set_f32(block + 0x550, 3.5);
        e.mem.set_f32(block + 0x554, 4.5);
        e.mem.set_u8(block + 0x1c, 0xfe);
        e.mem.set_u16(block + 0xc, 1500);
        assert_eq!(e.call(0x0056_7470, &args![block]).f32(), 2.5);
        assert_eq!(e.call(0x0056_7730, &args![block]).f32(), 3.5);
        assert_eq!(e.call(0x0056_7750, &args![block]).f32(), 4.5);
        // the rank byte is signed
        assert_eq!(e.call(0x0056_7ab0, &args![block]).i32(), -2);
        // heading = word / 1000.0
        assert_eq!(e.call(0x0056_8650, &args![block]).f32(), 1.5);
        e.call(0x0056_8ab0, &args![block, 7u32]);
        assert_eq!(e.mem.u8(block + 0xe), 7);
    }

    // ---- 00567490 ---------------------------------------------------------

    /// The scale setter's world: a reference of base kind 0x40 whose
    /// virtuals IsActor / Get3D answer from `OWN_IS_ACTOR` / `OWN_NODE`, the
    /// scrap heap and the text conversion as doubles that do what the C
    /// library does for "%.2f" and `atof`, and the limits in memory.
    fn scale_fixture() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = ownership_engine();
        e.set_global(SCALE_MIN_DOUBLE, f64::from(0.01f32));
        e.set_global(SCALE_MIN_FLOAT, 0.01f32);
        e.set_global(SCALE_MAX_DOUBLE, 10.0f64);
        e.set_global(SCALE_MAX_FLOAT, 10.0f32);
        e.set_global(SCRAP_ALIGNMENT, 4u32);
        returns(&mut e, GATE_00444ED0, 0);
        returns(&mut e, MEMORY_MANAGER_GET, 0x1111);
        returns(&mut e, SCRAP_HEAP_OF_THREAD, 0x2222);
        e.register(SCRAP_HEAP_ALLOCATE, |e, _| e.mem.alloc(0x40).into_ret());
        e.register(FORMAT_TO_BUFFER, |e, a| {
            let value = f64::from_bits(u64::from(a[3]) | u64::from(a[4]) << 32);
            e.mem.set_cstr(a[0], format!("{value:.2}").as_bytes());
            Ret::default()
        });
        e.register(ATOF, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            Ret {
                st0: text.parse::<f64>().unwrap(),
                ..Ret::default()
            }
        });
        stub(
            &mut e,
            &[
                SCRAP_HEAP_DEALLOCATE,
                SET_NODE_SCALE,
                MAKE_VELOCITY,
                SET_3D_VELOCITY,
            ],
        );
        let refr = ownership_refr(&mut e, 0x40);
        e.mem.set_u32(refr.addr() + OWN_NODE, 0x5000);
        (e, refr)
    }

    #[test]
    fn set_scale_rounds_through_text_and_clamps() {
        let (mut e, refr) = scale_fixture();
        let me = refr.addr();
        for (input, expected) in [
            (1.2345f32, 1.23f32),
            (0.001, 0.01),
            (50.0, 10.0),
            (3.0, 3.0),
        ] {
            let log = logged(&mut e, |e| {
                e.call(0x0056_7490, &args![refr, input]);
            });
            assert_eq!(e.get(refr, TESObjectREFR::fRefScale), expected);
            // the node gets the scale; the changed flag 0x10 goes out
            let scale = calls_to(&log, SET_NODE_SCALE);
            assert_eq!(scale.len(), 1);
            assert_eq!(scale[0][0], 0x5000);
            assert_eq!(f32::from_bits(scale[0][1]), expected);
            assert_eq!(
                calls_to(&log, slot_address(&e, me, 0x48)),
                vec![vec![me, 0x10]]
            );
            // a zero velocity record is given to the node
            assert_eq!(calls_to(&log, SET_3D_VELOCITY).len(), 1);
            // the text block (0x20 bytes, alignment 4) is released
            let allocated = calls_to(&log, SCRAP_HEAP_ALLOCATE);
            assert_eq!(allocated, vec![vec![0x2222, 0x20, 4]]);
            let released = calls_to(&log, SCRAP_HEAP_DEALLOCATE);
            assert_eq!(released.len(), 1);
            assert_eq!(released[0][0], 0x2222);
        }
    }

    #[test]
    fn set_scale_stops_early_for_other_base_forms_when_the_gate_holds() {
        let (mut e, refr) = scale_fixture();
        returns(&mut e, GATE_00444ED0, 1);
        e.set(refr, TESObjectREFR::fRefScale, 1.0f32);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7490, &args![refr, 2.0f32]);
        });
        assert!(calls_to(&log, SCRAP_HEAP_ALLOCATE).is_empty());
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 1.0);
        // base form type 4 goes on
        let base = e.mem.u32(refr.addr() + 0x20);
        e.mem.set_u8(base + 4, 4);
        e.call(0x0056_7490, &args![refr, 2.0f32]);
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 2.0);
    }

    #[test]
    fn set_scale_without_a_3d_only_stores_the_scale() {
        let (mut e, refr) = scale_fixture();
        e.mem.set_u32(refr.addr() + OWN_NODE, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7490, &args![refr, 2.0f32]);
        });
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 2.0);
        assert!(calls_to(&log, SET_NODE_SCALE).is_empty());
        assert_eq!(calls_to(&log, SCRAP_HEAP_DEALLOCATE).len(), 1);
    }

    #[test]
    fn set_scale_keeps_an_actors_controller_floats_and_refreshes_the_player() {
        let (mut e, refr) = scale_fixture();
        let me = refr.addr();
        e.mem.set_u32(me + OWN_IS_ACTOR, 1);
        let controller = e.mem.alloc(0x600);
        e.mem.set_f32(controller + 0x550, 1.5);
        e.mem.set_f32(controller + 0x554, 2.5);
        e.mem.set_u32(me + 0x80, controller);
        // the virtual +0x2a0 clobbers the two floats; they come back after +0x1c4
        let vtable = e.mem.u32(me);
        let clobber = fake_function(&mut e, |e, a| {
            let controller = e.mem.u32(a[0] + 0x80);
            e.mem.set_f32(controller + 0x550, 9.0);
            e.mem.set_f32(controller + 0x554, 9.0);
            Ret::default()
        });
        e.mem.set_u32(vtable + 0x2a0, clobber);
        let after = fake_function(&mut e, noop_slot);
        e.mem.set_u32(vtable + 0x1c4, after);
        returns(&mut e, CHAR_CONTROLLER, controller);
        let acquire = object_with_vtable(&mut e, 0x40, &[(0x290, noop_slot)]);
        returns(&mut e, SAVED_ACQUIRE_OBJECT, acquire);
        e.register(HANDLE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        stub(&mut e, &[REFRESH_PLAYER]);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7490, &args![refr, 2.0f32]);
        });
        assert_eq!(e.mem.f32(controller + 0x550), 1.5);
        assert_eq!(e.mem.f32(controller + 0x554), 2.5);
        // the saved acquire object gets the one-word handle (holding 0)
        assert_eq!(
            calls_to(&log, slot_address(&e, acquire, 0x290)),
            vec![vec![acquire, 0]]
        );
        assert!(calls_to(&log, REFRESH_PLAYER).is_empty());
        // as the player: both views get the scale and the player is refreshed
        e.set_global(GLOBAL_PLAYER, me);
        e.register(PLAYER_3D, |_, a| {
            (if a[1] == 1 { 0x6000u32 } else { 0x5000 }).into_ret()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0056_7490, &args![refr, 2.0f32]);
        });
        let nodes: Vec<u32> = calls_to(&log, SET_NODE_SCALE)
            .iter()
            .map(|call| call[0])
            .collect();
        assert_eq!(nodes, vec![0x6000, 0x5000]);
        assert_eq!(calls_to(&log, REFRESH_PLAYER), vec![vec![me]]);
    }

    // ---- 00567770, 00567790, 005678a0 -------------------------------------

    #[test]
    fn owner_extra_and_the_actor_shortcut() {
        let mut e = ownership_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_7770, &args![refr]).u32(), 0);
        e.mem.set_u32(me + LIST_OWNER, 0xaaaa);
        assert_eq!(e.call(0x0056_7770, &args![refr]).u32(), 0xaaaa);
        assert_eq!(e.call(0x0056_7790, &args![refr]).u32(), 0xaaaa);
        // an actor without the extra answers 0 whatever its cell says
        let actor = ownership_refr(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + OWN_IS_ACTOR, 1);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell, 0xcccc);
        e.mem.set_u32(actor.addr() + REFR_CELL, cell);
        assert_eq!(e.call(0x0056_7790, &args![actor]).u32(), 0);
    }

    #[test]
    fn owner_comes_from_the_link_the_zone_or_the_cell() {
        let mut e = ownership_engine();
        // the link: teleport data whose first dword is another reference
        let refr = ownership_refr(&mut e, 0x40);
        let other = ownership_refr(&mut e, 0x40);
        e.mem.set_u32(other.addr() + LIST_OWNER, 0xbbbb);
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(link, other.addr());
        e.mem.set_u32(refr.addr() + LIST_LINK, link);
        assert_eq!(e.call(0x0056_7790, &args![refr]).u32(), 0xbbbb);
        // the zone (not the default one) gives its owner at +0x18
        let refr = ownership_refr(&mut e, 0x40);
        let zone = e.mem.alloc(0x40);
        e.mem.set_u32(zone + 0x18, 0xdddd);
        e.mem.set_u32(refr.addr() + LIST_ZONE, zone);
        assert_eq!(e.call(0x0056_7790, &args![refr]).u32(), 0xdddd);
        // the default zone is skipped, the cell answers
        e.mem.set_u32(refr.addr() + LIST_ZONE, DEFAULT_ZONE_ADDRESS);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell, 0xcccc);
        e.mem.set_u32(refr.addr() + REFR_CELL, cell);
        assert_eq!(e.call(0x0056_7790, &args![refr]).u32(), 0xcccc);
        // furniture and the base form types 0x1c and 0x15 never look further
        for kind in [0x27u8, 0x1c, 0x15] {
            let refr = ownership_refr(&mut e, kind);
            e.mem.set_u32(refr.addr() + REFR_CELL, cell);
            assert_eq!(e.call(0x0056_7790, &args![refr]).u32(), 0, "kind {kind:#x}");
        }
        // no base form: nothing either
        let bare = refr_with(&mut e, 0, 0, &[(0x100, slot_value::<OWN_IS_ACTOR>)]);
        e.mem.set_u32(bare.addr() + REFR_CELL, cell);
        assert_eq!(e.call(0x0056_7790, &args![bare]).u32(), 0);
    }

    #[test]
    fn evil_faction_follows_the_owner_or_the_actor_base() {
        let mut e = ownership_engine();
        returns(&mut e, FACTION_IS_EVIL, 1);
        e.register(ACTOR_BASE_EVIL_ONLY, |e, a| {
            u32::from(e.mem.u8(a[0] + 1) != 0).into_ret()
        });
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        // an owner that is a faction (type 8)
        let faction = form(&mut e, 8, 2, 0);
        e.mem.set_u32(me + LIST_OWNER, faction);
        assert_eq!(e.call(0x0056_78a0, &args![refr]).u8(), 1);
        // an owner that is an actor base (type 0x2a): the data at +0x30
        let owner = form(&mut e, 0x2a, 3, 0);
        e.mem.set_u8(owner + 0x31, 1);
        e.mem.set_u32(me + LIST_OWNER, owner);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_78a0, &args![refr]).u8(), 1);
        });
        assert_eq!(
            calls_to(&log, ACTOR_BASE_EVIL_ONLY),
            vec![vec![owner + 0x30]]
        );
        // another owner type: not evil
        let other = form(&mut e, 0x30, 4, 0);
        e.mem.set_u32(me + LIST_OWNER, other);
        assert_eq!(e.call(0x0056_78a0, &args![refr]).u8(), 0);
        // no owner: the actor base of the virtual +0x1a4, else the base form
        e.mem.set_u32(me + LIST_OWNER, 0);
        let actor_base = form(&mut e, 0x2a, 5, 0);
        e.mem.set_u8(actor_base + 0x31, 1);
        e.mem.set_u32(me + OWN_ACTOR_BASE, actor_base);
        assert_eq!(e.call(0x0056_78a0, &args![refr]).u8(), 1);
        e.mem.set_u32(me + OWN_ACTOR_BASE, 0);
        assert_eq!(e.call(0x0056_78a0, &args![refr]).u8(), 0);
        let base = e.mem.u32(me + 0x20);
        e.mem.set_u8(base + 4, 0x2a);
        e.mem.set_u8(base + 0x31, 1);
        assert_eq!(e.call(0x0056_78a0, &args![refr]).u8(), 1);
    }

    // ---- 00567960, 005679f0 -----------------------------------------------

    #[test]
    fn ownership_global_falls_back_to_the_link_then_the_cell() {
        let mut e = ownership_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_7960, &args![refr]).u32(), 0);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell + 4, 0xc1);
        e.mem.set_u32(me + REFR_CELL, cell);
        assert_eq!(e.call(0x0056_7960, &args![refr]).u32(), 0xc1);
        let other = ownership_refr(&mut e, 0x40);
        e.mem.set_u32(other.addr() + LIST_GLOBAL, 0xa1);
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(link, other.addr());
        e.mem.set_u32(me + LIST_LINK, link);
        assert_eq!(e.call(0x0056_7960, &args![refr]).u32(), 0xa1);
        e.mem.set_u32(me + LIST_GLOBAL, 0x91);
        assert_eq!(e.call(0x0056_7960, &args![refr]).u32(), 0x91);
    }

    #[test]
    fn ownership_rank_falls_back_to_link_zone_cell_and_defaults_to_zero() {
        let mut e = ownership_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_79f0, &args![refr]).i32(), 0);
        e.mem.set_u32(me + LIST_RANK, 3);
        assert_eq!(e.call(0x0056_79f0, &args![refr]).i32(), 3);
        // the link
        e.mem.set_u32(me + LIST_RANK, u32::MAX);
        let other = ownership_refr(&mut e, 0x40);
        e.mem.set_u32(other.addr() + LIST_RANK, 4);
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(link, other.addr());
        e.mem.set_u32(me + LIST_LINK, link);
        assert_eq!(e.call(0x0056_79f0, &args![refr]).i32(), 4);
        // the zone's signed byte at +0x1c
        e.mem.set_u32(me + LIST_LINK, 0);
        let zone = e.mem.alloc(0x40);
        e.mem.set_u8(zone + 0x1c, 5);
        e.mem.set_u32(me + LIST_ZONE, zone);
        assert_eq!(e.call(0x0056_79f0, &args![refr]).i32(), 5);
        // a zone byte of -1 and the default zone both leave the cell
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell + 8, 6);
        e.mem.set_u32(me + REFR_CELL, cell);
        e.mem.set_u8(zone + 0x1c, 0xff);
        assert_eq!(e.call(0x0056_79f0, &args![refr]).i32(), 6);
        e.mem.set_u32(me + LIST_ZONE, DEFAULT_ZONE_ADDRESS);
        assert_eq!(e.call(0x0056_79f0, &args![refr]).i32(), 6);
    }

    // ---- 00567ad0, 00567b50, 00567bd0, 00567c50 ---------------------------

    /// A reference linked to `other`, as the owner setters see it.
    fn linked_pair(e: &mut Engine) -> (Ptr<TESObjectREFR>, Ptr<TESObjectREFR>) {
        let refr = ownership_refr(e, 0x40);
        let other = ownership_refr(e, 0x40);
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(link, other.addr());
        e.mem.set_u32(refr.addr() + LIST_LINK, link);
        (refr, other)
    }

    #[test]
    fn owner_setters_mark_the_reference_and_clear_the_linked_one() {
        let mut e = ownership_engine();
        let (refr, other) = linked_pair(&mut e);
        let (me, them) = (refr.addr(), other.addr());
        for (addr, field, value, cleared) in [
            (0x0056_7ad0u32, LIST_OWNER, 0x1234u32, 0u32),
            (0x0056_7b50, LIST_GLOBAL, 0x2345, 0),
            (0x0056_7bd0, LIST_RANK, 2, u32::MAX),
        ] {
            e.mem.set_u32(them + field, 0x99);
            let log = logged(&mut e, |e| {
                e.call(addr, &args![refr, value]);
            });
            assert_eq!(e.mem.u32(me + field), value, "{addr:#x}");
            assert_eq!(e.mem.u32(them + field), cleared, "{addr:#x}");
            assert_eq!(
                calls_to(&log, slot_address(&e, me, 0x48)),
                vec![vec![me, 0x40]]
            );
            assert_eq!(
                calls_to(&log, slot_address(&e, them, 0x48)),
                vec![vec![them, 0x40]]
            );
        }
        // no link: only the reference itself
        let alone = ownership_refr(&mut e, 0x40);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7ad0, &args![alone, 5u32]);
        });
        assert_eq!(e.mem.u32(alone.addr() + LIST_OWNER), 5);
        assert_eq!(calls_to(&log, EXTRA_SET_OWNER).len(), 1);
    }

    #[test]
    fn clearing_the_ownership_resets_owner_global_and_rank_on_both() {
        let mut e = ownership_engine();
        let (refr, other) = linked_pair(&mut e);
        for r in [refr, other] {
            e.mem.set_u32(r.addr() + LIST_OWNER, 1);
            e.mem.set_u32(r.addr() + LIST_GLOBAL, 2);
            e.mem.set_u32(r.addr() + LIST_RANK, 3);
        }
        let log = logged(&mut e, |e| {
            e.call(0x0056_7c50, &args![refr]);
        });
        for r in [refr, other] {
            assert_eq!(e.mem.u32(r.addr() + LIST_OWNER), 0);
            assert_eq!(e.mem.u32(r.addr() + LIST_GLOBAL), 0);
            assert_eq!(e.mem.u32(r.addr() + LIST_RANK), u32::MAX);
        }
        assert_eq!(calls_to(&log, slot_address(&e, refr.addr(), 0x48)).len(), 1);
        assert_eq!(
            calls_to(&log, slot_address(&e, other.addr(), 0x48)).len(),
            1
        );
    }

    // ---- 00567d20, 00567d90, 00567dd0 -------------------------------------

    #[test]
    fn encounter_zone_comes_from_the_list_the_cell_or_the_world() {
        let mut e = ownership_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_7d20, &args![refr]).u32(), 0);
        assert_eq!(e.call(0x0056_7d90, &args![refr]).u32(), 0);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0xd0, 0x3333);
        e.mem.set_u32(me + REFR_WORLD, world);
        assert_eq!(e.call(0x0056_7d20, &args![refr]).u32(), 0x3333);
        assert_eq!(e.call(0x0056_7d90, &args![refr]).u32(), 0x3333);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell + 0x10, 0x2222);
        e.mem.set_u32(me + REFR_CELL, cell);
        assert_eq!(e.call(0x0056_7d20, &args![refr]).u32(), 0x2222);
        // a cell without a zone falls through to the world; 00567d90 does not
        e.mem.set_u32(cell + 0x10, 0);
        assert_eq!(e.call(0x0056_7d20, &args![refr]).u32(), 0x3333);
        assert_eq!(e.call(0x0056_7d90, &args![refr]).u32(), 0);
        e.mem.set_u32(me + LIST_ZONE, 0x1111);
        assert_eq!(e.call(0x0056_7d20, &args![refr]).u32(), 0x1111);
    }

    #[test]
    fn set_encounter_zone_stores_it_and_marks_the_reference() {
        let mut e = ownership_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        let log = logged(&mut e, |e| {
            e.call(0x0056_7dd0, &args![refr, 0x4444u32]);
        });
        assert_eq!(e.mem.u32(me + LIST_ZONE), 0x4444);
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x48)),
            vec![vec![me, 0x2000_0000]]
        );
    }

    // ---- 00567e10, 00567ec0 -----------------------------------------------

    #[test]
    fn calc_level_uses_the_zone_or_the_player_and_the_counter() {
        let mut e = ownership_engine();
        e.register(ZONE_LEVEL_FALLBACK, |_, _| 7u32.into_ret());
        returns(&mut e, PLAYER_LEVEL, 0x0001_0014);
        e.set_global(GLOBAL_PLAYER, 0x55u32);
        let limit = e.mem.alloc(4);
        let restart = e.mem.alloc(4);
        e.mem.set_i32(limit, 3);
        e.mem.set_i32(restart, 2);
        e.register_double(SETTING_VALUE, move |_, a| {
            (if a[0] == SETTING_COUNTER_LIMIT {
                limit
            } else {
                restart
            })
            .into_ret()
        });
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        // no zone: the player's level (low word)
        assert_eq!(e.call(0x0056_7e10, &args![refr, 0u32]).u32(), 0x14);
        // a zone with its own level; the default zone is not one
        let zone = e.mem.alloc(0x40);
        e.mem.set_u16(zone + 0x2c, 9);
        e.mem.set_u32(me + LIST_ZONE, zone);
        assert_eq!(e.call(0x0056_7e10, &args![refr, 0u32]).u32(), 9);
        e.mem.set_u16(zone + 0x2c, 0);
        assert_eq!(e.call(0x0056_7e10, &args![refr, 0u32]).u32(), 7);
        e.mem.set_u16(zone + 0x2c, 9);
        e.mem.set_u32(me + LIST_ZONE, DEFAULT_ZONE_ADDRESS);
        assert_eq!(e.call(0x0056_7e10, &args![refr, 0u32]).u32(), 0x14);
        // the counter steps by one each call and restarts below minus 2
        e.set_global(CALC_LEVEL_COUNTER, 0i32);
        let mut seen = vec![];
        for _ in 0..5 {
            seen.push(e.call(0x0056_7e10, &args![refr, 1u32]).u32());
        }
        // 1, 2, 3, then 4 > 3 wraps to -2, then -1
        assert_eq!(seen, vec![0x15, 0x16, 0x17, 0x12, 0x13]);
        // never below 1
        e.mem.set_u32(me + LIST_ZONE, zone);
        e.mem.set_u16(zone + 0x2c, 1);
        e.set_global(CALC_LEVEL_COUNTER, -2i32);
        assert_eq!(e.call(0x0056_7e10, &args![refr, 1u32]).u32(), 1);
    }

    // ---- 00567f00, 00567f40 -----------------------------------------------

    #[test]
    fn second_extra_getter_and_setter() {
        let mut e = ownership_engine();
        e.register(EXTRA_GET_SECOND, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(GET_CONTROLLER, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(EXTRA_SET_SECOND, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, a[1]);
            Ret::default()
        });
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_7f00, &args![refr]).u32(), 0);
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 0xc, 0x777);
        e.mem.set_u32(me + 0x44 + 0x1c, extra);
        assert_eq!(e.call(0x0056_7f00, &args![refr]).u32(), 0x777);
        let log = logged(&mut e, |e| {
            e.call(0x0056_7f40, &args![refr, 0x42u32]);
        });
        assert_eq!(e.mem.u32(me + 0x44 + 0x1c), 0x42);
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x48)),
            vec![vec![me, 0x8000_0000]]
        );
    }

    // ---- markers ----------------------------------------------------------

    /// The marker extras as doubles. An extra object has its kind tag at +4
    /// and its marker bits at +0xc; the list keeps the extras at
    /// `LIST_USED`, `LIST_RESERVED` and `LIST_HEALTH`.
    fn marker_engine() -> Engine {
        let mut e = ownership_engine();
        e.register(EXTRA_GET_DATA, |e, a| {
            let slot = match a[1] {
                0x12 => LIST_USED,
                0x82 => LIST_RESERVED,
                _ => LIST_HEALTH,
            } - 0x44;
            e.mem.u32(a[0] + slot).into_ret()
        });
        e.register(EXTRA_ADD, |e, a| {
            let slot = match e.mem.u32(a[1] + 4) {
                0x12 => LIST_USED,
                0x82 => LIST_RESERVED,
                _ => LIST_HEALTH,
            } - 0x44;
            e.mem.set_u32(a[0] + slot, a[1]);
            Ret::default()
        });
        e.register(EXTRA_REMOVE, |e, a| {
            for slot in [LIST_USED, LIST_RESERVED, LIST_HEALTH] {
                if e.mem.u32(a[0] + slot - 0x44) == a[1] {
                    e.mem.set_u32(a[0] + slot - 0x44, 0);
                }
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(USED_MARKERS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x12);
            a[0].into_ret()
        });
        e.register(RESERVED_MARKERS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x82);
            a[0].into_ret()
        });
        e.register(MARKERS_GET_USED, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xc) >> a[1] & 1 != 0).into_ret()
        });
        e.register(MARKERS_SET_USED, |e, a| {
            let bits = e.mem.u32(a[0] + 0xc);
            let bits = if a[2] as u8 != 0 {
                bits | 1 << a[1]
            } else {
                bits & !(1 << a[1])
            };
            e.mem.set_u32(a[0] + 0xc, bits);
            Ret::default()
        });
        // the furniture form (base object): enabled markers as a mask at +0xf0
        e.register(FURNITURE_MARKER_ENABLED, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xf0) >> a[1] & 1 != 0).into_ret()
        });
        e
    }

    /// A furniture reference (base form type 0x27) with `enabled` markers.
    fn furniture_refr(e: &mut Engine, enabled: u32) -> Ptr<TESObjectREFR> {
        let refr = ownership_refr(e, 0x27);
        let base = e.mem.u32(refr.addr() + 0x20);
        e.mem.set_u32(base + 0xf0, enabled);
        refr
    }

    fn extra_bits(e: &Engine, refr: Ptr<TESObjectREFR>, slot: u32) -> Option<u32> {
        let extra = e.mem.u32(refr.addr() + slot);
        (extra != 0).then(|| e.mem.u32(extra + 0xc))
    }

    #[test]
    fn marker_used_reads_the_used_extra_for_furniture_and_the_reserved_one() {
        let mut e = marker_engine();
        let refr = furniture_refr(&mut e, 0xff);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_7f80, &args![refr, 3u32, 0u32]).u8(), 0);
        let used = e.mem.alloc(0x20);
        e.mem.set_u32(used + 0xc, 1 << 3);
        e.mem.set_u32(me + LIST_USED, used);
        let reserved = e.mem.alloc(0x20);
        e.mem.set_u32(reserved + 0xc, 1 << 4);
        e.mem.set_u32(me + LIST_RESERVED, reserved);
        assert_eq!(e.call(0x0056_7f80, &args![refr, 3u32, 0u32]).u8(), 1);
        assert_eq!(e.call(0x0056_7f80, &args![refr, 4u32, 0u32]).u8(), 1);
        // the flag skips the reserved extra
        assert_eq!(e.call(0x0056_7f80, &args![refr, 4u32, 1u32]).u8(), 0);
        // a base form that is not furniture ignores the used extra
        let plain = ownership_refr(&mut e, 0x40);
        e.mem.set_u32(plain.addr() + LIST_USED, used);
        assert_eq!(e.call(0x0056_7f80, &args![plain, 3u32, 0u32]).u8(), 0);
    }

    #[test]
    fn set_marker_used_creates_updates_and_removes_the_extra() {
        let mut e = marker_engine();
        let refr = furniture_refr(&mut e, 0xff);
        let me = refr.addr();
        // freeing a marker without an extra does nothing
        e.call(0x0056_8020, &args![refr, 2u32, 0u32]);
        assert_eq!(extra_bits(&e, refr, LIST_USED), None);
        // using one creates the extra (0x10 bytes) and marks the reference
        let log = logged(&mut e, |e| {
            e.call(0x0056_8020, &args![refr, 2u32, 1u32]);
        });
        assert_eq!(extra_bits(&e, refr, LIST_USED), Some(1 << 2));
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x48)),
            vec![vec![me, 0x8000_0000]]
        );
        // a second marker joins it
        e.call(0x0056_8020, &args![refr, 5u32, 1u32]);
        assert_eq!(extra_bits(&e, refr, LIST_USED), Some(1 << 2 | 1 << 5));
        // freeing both removes the extra
        e.call(0x0056_8020, &args![refr, 2u32, 0u32]);
        e.call(0x0056_8020, &args![refr, 5u32, 0u32]);
        assert_eq!(extra_bits(&e, refr, LIST_USED), None);
        // not furniture: nothing at all
        let plain = ownership_refr(&mut e, 0x40);
        e.call(0x0056_8020, &args![plain, 2u32, 1u32]);
        assert_eq!(extra_bits(&e, plain, LIST_USED), None);
    }

    #[test]
    fn set_marker_reserved_works_on_the_reserved_extra_for_any_reference() {
        let mut e = marker_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        let log = logged(&mut e, |e| {
            e.call(0x0056_8150, &args![refr, 1u32, 1u32]);
        });
        assert_eq!(extra_bits(&e, refr, LIST_RESERVED), Some(1 << 1));
        assert_eq!(extra_bits(&e, refr, LIST_USED), None);
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x48)),
            vec![vec![me, 0x8000_0000]]
        );
        e.call(0x0056_8150, &args![refr, 1u32, 0u32]);
        assert_eq!(extra_bits(&e, refr, LIST_RESERVED), None);
        // freeing with no extra still marks the reference
        let log = logged(&mut e, |e| {
            e.call(0x0056_8150, &args![refr, 1u32, 0u32]);
        });
        assert_eq!(calls_to(&log, slot_address(&e, me, 0x48)).len(), 1);
    }

    #[test]
    fn first_free_marker_index_of_furniture() {
        let mut e = marker_engine();
        let refr = furniture_refr(&mut e, 0b1111_0110);
        let me = refr.addr();
        // enabled: 1, 2, 4, 5, 6, 7
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), 1);
        let used = e.mem.alloc(0x20);
        e.mem.set_u32(used + 0xc, 1 << 1);
        e.mem.set_u32(me + LIST_USED, used);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), 2);
        let reserved = e.mem.alloc(0x20);
        e.mem.set_u32(reserved + 0xc, 1 << 2);
        e.mem.set_u32(me + LIST_RESERVED, reserved);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), 4);
        // with the flag the reserved extra does not count
        assert_eq!(e.call(0x0056_82c0, &args![refr, 1u32]).i32(), 2);
        // nothing enabled: -1
        let none = furniture_refr(&mut e, 0);
        assert_eq!(e.call(0x0056_82c0, &args![none, 0u32]).i32(), -1);
        // no base form: -1
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_82c0, &args![bare, 0u32]).i32(), -1);
    }

    #[test]
    fn first_free_marker_index_of_other_references() {
        let mut e = marker_engine();
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        // no extras: marker 0; the flag also gives 0
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), 0);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 1u32]).i32(), 0);
        // a used marker 0 leaves none (unless the flag is set)
        let used = e.mem.alloc(0x20);
        e.mem.set_u32(used + 0xc, 1);
        e.mem.set_u32(me + LIST_USED, used);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), -1);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 1u32]).i32(), 0);
        // a free marker 0 in an existing extra
        e.mem.set_u32(used + 0xc, 0);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), 0);
        // reserved marker 0 counts too
        let reserved = e.mem.alloc(0x20);
        e.mem.set_u32(reserved + 0xc, 1);
        e.mem.set_u32(me + LIST_RESERVED, reserved);
        assert_eq!(e.call(0x0056_82c0, &args![refr, 0u32]).i32(), -1);
    }

    #[test]
    fn has_free_marker_follows_the_first_free_index() {
        let mut e = marker_engine();
        let furniture = furniture_refr(&mut e, 0b10);
        assert_eq!(e.call(0x0056_8260, &args![furniture, 0u32]).u8(), 1);
        assert_eq!(e.call(0x0056_8260, &args![furniture, 1u32]).u8(), 1);
        let none = furniture_refr(&mut e, 0);
        assert_eq!(e.call(0x0056_8260, &args![none, 0u32]).u8(), 0);
        // other base forms: only without the flag
        let plain = ownership_refr(&mut e, 0x40);
        assert_eq!(e.call(0x0056_8260, &args![plain, 0u32]).u8(), 1);
        assert_eq!(e.call(0x0056_8260, &args![plain, 1u32]).u8(), 0);
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_8260, &args![bare, 0u32]).u8(), 0);
    }

    #[test]
    fn clearing_all_markers_frees_both_extras() {
        let mut e = marker_engine();
        let refr = furniture_refr(&mut e, 0xff);
        let me = refr.addr();
        e.call(0x0056_8020, &args![refr, 2u32, 1u32]);
        e.call(0x0056_8150, &args![refr, 3u32, 1u32]);
        assert!(extra_bits(&e, refr, LIST_USED).is_some());
        assert!(extra_bits(&e, refr, LIST_RESERVED).is_some());
        e.call(0x0056_8480, &args![refr]);
        assert_eq!(extra_bits(&e, refr, LIST_USED), None);
        assert_eq!(extra_bits(&e, refr, LIST_RESERVED), None);
        // 0x1e markers, each freed in both extras: 60 marks of the reference
        let log = logged(&mut e, |e| {
            e.call(0x0056_8480, &args![refr]);
        });
        assert_eq!(calls_to(&log, slot_address(&e, me, 0x48)).len(), 60);
        // not furniture: nothing
        let plain = ownership_refr(&mut e, 0x40);
        let log = logged(&mut e, |e| {
            e.call(0x0056_8480, &args![plain]);
        });
        assert!(calls_to(&log, slot_address(&e, plain.addr(), 0x48)).is_empty());
    }

    #[test]
    fn is_furniture_checks_the_base_form_type() {
        let mut e = engine();
        let furniture = refr_of_kind(&mut e, 0x27);
        assert_eq!(e.call(0x0056_8680, &args![furniture]).u8(), 1);
        let other = refr_of_kind(&mut e, 0x28);
        assert_eq!(e.call(0x0056_8680, &args![other]).u8(), 0);
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_8680, &args![bare]).u8(), 0);
    }

    // ---- 00568500, 005686b0: furniture marker tables ----------------------

    /// The furniture marker table as doubles. A table (`FURNITURE_FIND` of a
    /// node reads its pointer at +0) holds its count at +0 and its 16-byte
    /// entries from +0x10: three position floats, the heading word at +0xc
    /// and the kind byte at +0xe. Transformations add the location; lengths
    /// are euclidean; marks keep their position at +0, heading at +0xc.
    fn table_engine() -> Engine {
        let mut e = marker_engine();
        e.register(FURNITURE_FIND, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FURNITURE_MARKER_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FURNITURE_MARKER_AT, |_, a| {
            (a[0] + 0x10 + 0x10 * a[1]).into_ret()
        });
        e.register(FURNITURE_MARKER_KIND, |e, a| {
            u32::from(e.mem.u8(a[0] + 0xe)).into_ret()
        });
        e.register(IS_SIT_MARKER, |_, a| {
            u32::from((0xa..=0x14).contains(&a[0]) || a[0] == 0x1a).into_ret()
        });
        e.register(IS_SLEEP_MARKER, |_, a| u32::from(a[0] < 0xa).into_ret());
        e.register(POINT_OF, |_, a| a[0].into_ret());
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(ROTATION_OF, |_, a| (a[0] + 0x24).into_ret());
        e.register(TRANSFORM_VERTICES, |e, a| {
            for vertex in 0..a[2] {
                for axis in 0..3 {
                    let value =
                        e.mem.f32(a[3] + 12 * vertex + 4 * axis) + e.mem.f32(a[1] + 4 * axis);
                    e.mem.set_f32(a[4] + 12 * vertex + 4 * axis, value);
                }
            }
            Ret::default()
        });
        e.register(VECTOR_DIFFERENCE, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) - e.mem.f32(a[2] + 4 * axis);
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            a[1].into_ret()
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let sum: f32 = (0..3).map(|axis| e.mem.f32(a[0] + 4 * axis).powi(2)).sum();
            Ret {
                st0: f64::from(sum.sqrt()),
                ..Ret::default()
            }
        });
        e.register(MARK_SET_POSITION, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(MARK_SET_HEADING, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        e.set_global(HEADING_DIVISOR, 1000.0f64);
        e.set_global(FLOAT_MAX_COPY, f32::MAX);
        e
    }

    /// A furniture reference with a 3D whose table holds `entries`
    /// (position, heading word, kind); the reference sits at (10, 0, 0) and
    /// is rotated by 0.5 about z.
    fn table_refr(e: &mut Engine, entries: &[([f32; 3], u16, u8)]) -> Ptr<TESObjectREFR> {
        let refr = furniture_refr(e, 0xffff_ffff);
        let me = refr.addr();
        let table = e.mem.alloc(0x10 + 0x10 * entries.len() as u32);
        e.mem.set_u32(table, entries.len() as u32);
        for (i, (position, heading, kind)) in entries.iter().enumerate() {
            let entry = table + 0x10 + 0x10 * i as u32;
            for (axis, value) in position.iter().enumerate() {
                e.mem.set_f32(entry + 4 * axis as u32, *value);
            }
            e.mem.set_u16(entry + 0xc, *heading);
            e.mem.set_u8(entry + 0xe, *kind);
        }
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, table);
        e.mem.set_u32(me + OWN_NODE, node);
        e.mem.set_f32(me + 0x30, 10.0);
        e.mem.set_f32(me + 0x2c, 0.5);
        refr
    }

    #[test]
    fn marker_at_index_transforms_the_entry_and_sets_the_heading() {
        let mut e = table_engine();
        let refr = table_refr(
            &mut e,
            &[([1.0, 2.0, 3.0], 250, 0x5), ([4.0, 5.0, 6.0], 1500, 0xb)],
        );
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0056_8500, &args![refr, 1u32, out]).u8(), 1);
        // the entry's words arrive in `out`: the position moved by (10, 0, 0)
        assert_eq!(e.mem.f32(out), 14.0);
        assert_eq!(e.mem.f32(out + 4), 5.0);
        assert_eq!(e.mem.f32(out + 8), 6.0);
        // heading 1.5 + rotation 0.5
        assert_eq!(f32::from_bits(e.mem.u32(out + 0x10)), 2.0);
        // out of range, no 3D, no base form, not furniture
        assert_eq!(e.call(0x0056_8500, &args![refr, 2u32, out]).u8(), 0);
        e.mem.set_u32(refr.addr() + OWN_NODE, 0);
        assert_eq!(e.call(0x0056_8500, &args![refr, 0u32, out]).u8(), 0);
        let plain = ownership_refr(&mut e, 0x40);
        assert_eq!(e.call(0x0056_8500, &args![plain, 0u32, out]).u8(), 0);
    }

    #[test]
    fn closest_free_marker_picks_the_nearest_wanted_marker() {
        let mut e = table_engine();
        // kinds: 0x1 sleep, 0xb sit, 0x20 neither
        let refr = table_refr(
            &mut e,
            &[
                ([0.0, 0.0, 0.0], 0, 0x1),
                ([5.0, 0.0, 0.0], 500, 0xb),
                ([9.0, 0.0, 0.0], 0, 0x20),
                ([20.0, 0.0, 0.0], 0, 0x20),
            ],
        );
        let me = refr.addr();
        let from = e.mem.alloc(0x10);
        e.mem.set_f32(from, 10.0);
        let mark = e.mem.alloc(0x20);
        let index = e.mem.alloc(4);
        let closest = |e: &mut Engine, sit: u32, sleep: u32, flag: u32| {
            let found = e
                .call(
                    0x0056_86b0,
                    &args![refr, from, sit, sleep, mark, index, flag],
                )
                .u8();
            (found, e.mem.u32(index))
        };
        // (marker positions are moved by the location (10, 0, 0); `from` is
        // at x = 10 so the nearest transformed position wins: entry 0 at 10,
        // but it is a sleep marker, wanted only with the sleep flag)
        assert_eq!(closest(&mut e, 0, 0, 0), (1, 2));
        assert_eq!(e.mem.u8(mark + 0xe), 0x20);
        assert_eq!(e.mem.f32(mark), 19.0);
        // heading 0 + rotation 0.5
        assert_eq!(f32::from_bits(e.mem.u32(mark + 0x10)), 0.5);
        assert_eq!(closest(&mut e, 0, 1, 0), (1, 0));
        assert_eq!(e.mem.u8(mark + 0xe), 0x1);
        // the sit marker at 15 loses to the neither-kind at 19? no: 15 is
        // nearer than 19 from x = 10
        assert_eq!(closest(&mut e, 1, 0, 0), (1, 1));
        assert_eq!(f32::from_bits(e.mem.u32(mark + 0x10)), 1.0);
        // a used marker is skipped (the used extra of the reference)
        let used = e.mem.alloc(0x20);
        e.mem.set_u32(used + 0xc, 1 << 2);
        e.mem.set_u32(me + LIST_USED, used);
        assert_eq!(closest(&mut e, 0, 0, 0), (1, 3));
        // nothing wanted: false, the outputs stay
        let all_used = e.mem.alloc(0x20);
        e.mem.set_u32(all_used + 0xc, 0b1111);
        e.mem.set_u32(me + LIST_USED, all_used);
        e.mem.set_u32(index, 77);
        assert_eq!(closest(&mut e, 1, 1, 0), (0, 77));
    }

    #[test]
    fn closest_free_marker_without_3d_or_for_other_forms() {
        let mut e = table_engine();
        let from = e.mem.alloc(0x10);
        let mark = e.mem.alloc(0x20);
        let index = e.mem.alloc(4);
        // furniture without a loaded 3D: the mark gets the location, -1, false
        let refr = table_refr(&mut e, &[([1.0, 0.0, 0.0], 0, 0x20)]);
        e.mem.set_u32(refr.addr() + OWN_NODE, 0);
        e.mem.set_f32(refr.addr() + 0x30, 3.0);
        e.mem.set_u8(mark + 0xe, 9);
        let args_for =
            |r: Ptr<TESObjectREFR>, flag: u32| args![r, from, 0u32, 0u32, mark, index, flag];
        assert_eq!(e.call(0x0056_86b0, &args_for(refr, 0)).u8(), 0);
        assert_eq!(e.mem.u32(index), u32::MAX);
        assert_eq!(e.mem.f32(mark), 3.0);
        assert_eq!(e.mem.u8(mark + 0xe), 0);
        // another base form: true with the same output unless the flag is set
        let plain = ownership_refr(&mut e, 0x40);
        e.mem.set_f32(plain.addr() + 0x30, 8.0);
        e.mem.set_u32(index, 5);
        assert_eq!(e.call(0x0056_86b0, &args_for(plain, 1)).u8(), 0);
        assert_eq!(e.mem.u32(index), 5);
        assert_eq!(e.call(0x0056_86b0, &args_for(plain, 0)).u8(), 1);
        assert_eq!(e.mem.u32(index), u32::MAX);
        assert_eq!(e.mem.f32(mark), 8.0);
    }

    // ---- 00568ad0, 00568bb0, 00568bd0 -------------------------------------

    #[test]
    fn health_like_value_of_actors_and_forms_with_a_health_form() {
        let mut e = marker_engine();
        returns(&mut e, ACTOR_FIELD_108, 6);
        e.register(RT_DYNAMIC_CAST, |e, a| {
            // the base form casts when its flags dword says so
            (if e.mem.u32(a[0] + 8) != 0 { a[0] } else { 0 }).into_ret()
        });
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        // an actor of the special state: 100
        e.mem.set_u32(me + OWN_IS_ACTOR, 1);
        e.set_global(VALUE_FULL, 100.0f32);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        assert_eq!(e.call(0x0056_8ad0, &args![refr]).f32(), 100.0);
        // another state: the embedded object at +0xa4 answers for 0x10
        returns(&mut e, ACTOR_FIELD_108, 1);
        let vtable = e.mem.u32(me);
        let embedded = fake_function(&mut e, |_, a| (a[1] + 5).into_ret());
        e.mem.set_u32(vtable + 8, embedded);
        // the object at +0xa4 needs its own vtable pointer
        e.mem.set_u32(me + 0xa4, vtable);
        assert_eq!(e.call(0x0056_8ad0, &args![refr]).f32(), 21.0);
        // not an actor, no health form: -1
        e.mem.set_u32(me + OWN_IS_ACTOR, 0);
        assert_eq!(e.call(0x0056_8ad0, &args![refr]).f32(), -1.0);
        // with a health form: its extra (+0xc), else its own value (slot +0x10)
        let base = e.mem.u32(me + 0x20);
        e.mem.set_u32(base + 8, 1);
        let health = fake_function(&mut e, |_, _| 42u32.into_ret());
        let base_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(base_vtable + 0x10, health);
        e.mem.set_u32(base, base_vtable);
        assert_eq!(e.call(0x0056_8ad0, &args![refr]).f32(), 42.0);
        let extra = e.mem.alloc(0x20);
        e.mem.set_f32(extra + 0xc, 7.5);
        e.mem.set_u32(me + LIST_HEALTH, extra);
        assert_eq!(e.call(0x0056_8ad0, &args![refr]).f32(), 7.5);
    }

    #[test]
    fn set_extra_copies_the_list_onto_the_reference() {
        let mut e = engine();
        let refr = refr_of_kind(&mut e, 0x40);
        stub(&mut e, &[EXTRA_COPY_LIST]);
        let log = logged(&mut e, |e| {
            e.call(0x0056_8bb0, &args![refr, 0x1234u32]);
        });
        assert_eq!(
            calls_to(&log, EXTRA_COPY_LIST),
            vec![vec![refr.addr() + 0x44, 0x1234]]
        );
    }

    #[test]
    fn set_health_extra_updates_removes_or_adds() {
        let mut e = marker_engine();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            (if e.mem.u32(a[0] + 8) != 0 { a[0] } else { 0 }).into_ret()
        });
        stub(&mut e, &[EXTRA_SET_HEALTH]);
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        // no health form: nothing
        let log = logged(&mut e, |e| {
            e.call(0x0056_8bd0, &args![refr, 5.0f32]);
        });
        assert!(calls_to(&log, EXTRA_SET_HEALTH).is_empty());
        let base = e.mem.u32(me + 0x20);
        e.mem.set_u32(base + 8, 1);
        let health = fake_function(&mut e, |_, _| 10u32.into_ret());
        let base_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(base_vtable + 0x10, health);
        e.mem.set_u32(base, base_vtable);
        // no extra and a different value: the setter; the same value: nothing
        let log = logged(&mut e, |e| {
            e.call(0x0056_8bd0, &args![refr, 5.0f32]);
            e.call(0x0056_8bd0, &args![refr, 10.0f32]);
        });
        let sets = calls_to(&log, EXTRA_SET_HEALTH);
        assert_eq!(sets.len(), 1);
        assert_eq!(f32::from_bits(sets[0][1]), 5.0);
        // an extra: a different value is stored, the form's value removes it
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 4, 0x25);
        e.mem.set_u32(me + LIST_HEALTH, extra);
        e.call(0x0056_8bd0, &args![refr, 3.0f32]);
        assert_eq!(e.mem.f32(extra + 0xc), 3.0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_8bd0, &args![refr, 10.0f32]);
        });
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE),
            vec![vec![me + 0x44, extra, 1]]
        );
    }

    // ---- 00568cb0 ---------------------------------------------------------

    fn radius_fixture(kind: u8) -> (Engine, Ptr<TESObjectREFR>, u32) {
        let mut e = ownership_engine();
        e.set_global(RADIUS_EPSILON, 0.001f32);
        e.set_global(RADIUS_FAR, 5000.0f32);
        e.set_global(GLOBAL_RADIUS_FORM_A, 0xa0a0u32);
        e.set_global(GLOBAL_RADIUS_FORM_B, 0xb0b0u32);
        e.set_global(GLOBAL_RADIUS_FORM_C, 0xc0c0u32);
        e.register(EXTRA_GET_RADIUS, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x40)),
            ..Ret::default()
        });
        e.register(FORM_FIELD_A0, |e, a| e.mem.u32(a[0] + 0xa0).into_ret());
        e.register(NEARLY_EQUAL, |_, a| {
            u32::from((f32::from_bits(a[0]) - f32::from_bits(a[1])).abs() <= f32::from_bits(a[2]))
                .into_ret()
        });
        let setting = e.mem.alloc(4);
        e.mem.set_i32(setting, 1234);
        e.register_double(SETTING_VALUE, move |_, _| setting.into_ret());
        let refr = ownership_refr(&mut e, kind);
        let loaded = e.mem.alloc(0x20);
        e.mem.set_f32(loaded + 0xc, -1.0);
        e.mem.set_u32(refr.addr() + 0x64, loaded);
        (e, refr, loaded)
    }

    #[test]
    fn radius_is_cached_in_the_loaded_data() {
        let (mut e, refr, loaded) = radius_fixture(0x1e);
        let base = e.mem.u32(refr.addr() + 0x20);
        e.mem.set_u32(base + 0xa0, 100);
        e.mem.set_f32(refr.addr() + 0x44 + 0x40, 2.5);
        // type 0x1e: the base form's dword plus the extra's radius, cached
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 102.5);
        assert_eq!(e.mem.f32(loaded + 0xc), 102.5);
        // a cached value is returned without asking again
        e.mem.set_f32(refr.addr() + 0x44 + 0x40, 9.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 102.5);
        // zero is a valid cache; a negative one is recomputed
        e.mem.set_f32(loaded + 0xc, 0.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 0.0);
        e.mem.set_f32(loaded + 0xc, -3.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 109.0);
    }

    #[test]
    fn radius_of_the_special_type_0x20_forms() {
        let (mut e, refr, loaded) = radius_fixture(0x20);
        let me = refr.addr();
        let base = e.mem.u32(me + 0x20);
        e.mem.set_f32(me + 0x44 + 0x40, 12.0);
        // an unknown form of that type: radius 0, still cached
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 0.0);
        assert_eq!(e.mem.f32(loaded + 0xc), 0.0);
        // the three special forms
        e.set_global(GLOBAL_RADIUS_FORM_A, base);
        e.mem.set_f32(loaded + 0xc, -1.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 12.0);
        e.set_global(GLOBAL_RADIUS_FORM_A, 0xa0a0u32);
        e.set_global(GLOBAL_RADIUS_FORM_B, base);
        e.mem.set_f32(loaded + 0xc, -1.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 12.0);
        // nearly zero: the setting
        e.mem.set_f32(me + 0x44 + 0x40, 0.0);
        e.mem.set_f32(loaded + 0xc, -1.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 1234.0);
        e.set_global(GLOBAL_RADIUS_FORM_B, 0xb0b0u32);
        e.set_global(GLOBAL_RADIUS_FORM_C, base);
        e.mem.set_f32(loaded + 0xc, -1.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 5000.0);
        e.mem.set_f32(me + 0x44 + 0x40, 8.0);
        e.mem.set_f32(loaded + 0xc, -1.0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 8.0);
    }

    #[test]
    fn radius_without_loaded_data_or_base_form() {
        let (mut e, refr, _) = radius_fixture(0x40);
        e.mem.set_u32(refr.addr() + 0x64, 0);
        // other types: 0, and nothing to cache into
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 0.0);
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_8cb0, &args![bare]).f32(), 0.0);
        let (mut e, refr, loaded) = radius_fixture(0x1e);
        let base = e.mem.u32(refr.addr() + 0x20);
        e.mem.set_u32(base + 0xa0, 4);
        e.mem.set_u32(refr.addr() + 0x64, 0);
        assert_eq!(e.call(0x0056_8cb0, &args![refr]).f32(), 4.0);
        assert_eq!(e.mem.f32(loaded + 0xc), -1.0);
    }

    // ---- 00568e50, 00568e70, 00568f30, 00568fa0 ---------------------------

    #[test]
    fn teleport_data_is_created_once_and_removed_with_its_portal() {
        let mut e = ownership_engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DOOR_TELEPORT_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        assert_eq!(e.call(0x0056_8e50, &args![refr]).u32(), 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_8e70, &args![refr]);
        });
        let teleport = e.mem.u32(me + LIST_LINK);
        assert_ne!(teleport, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x48)),
            vec![vec![me, 0x20000]]
        );
        // it exists now: returned as it is, nothing created
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_8e70, &args![refr]).u32(), teleport);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.call(0x0056_8e50, &args![refr]).u32(), teleport);
    }

    #[test]
    fn remove_teleport_notifies_a_portal_target_and_removes_the_portal() {
        let mut e = ownership_engine();
        stub(
            &mut e,
            &[EXTRA_REMOVE_LINK, EXTRA_REMOVE_PORTAL, PORTAL_TARGET_NOTIFY],
        );
        returns(&mut e, EXTRA_GET_PORTAL, 0);
        let refr = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        // without a portal only the pointer goes
        let log = logged(&mut e, |e| {
            e.call(0x0056_8f30, &args![refr]);
        });
        assert_eq!(calls_to(&log, EXTRA_REMOVE_LINK), vec![vec![me + 0x44]]);
        assert!(calls_to(&log, EXTRA_REMOVE_PORTAL).is_empty());
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x4c)),
            vec![vec![me, 0x20000]]
        );
        // a portal with a target: the target is told, the portal removed
        let portal = e.mem.alloc(0x20);
        e.mem.set_u32(portal + 0xc, 0x888);
        returns(&mut e, EXTRA_GET_PORTAL, portal);
        let log = logged(&mut e, |e| {
            e.call(0x0056_8f30, &args![refr]);
        });
        assert_eq!(calls_to(&log, PORTAL_TARGET_NOTIFY), vec![vec![0x888, me]]);
        assert_eq!(calls_to(&log, EXTRA_REMOVE_PORTAL), vec![vec![me + 0x44]]);
        // a portal without a target is only removed
        e.mem.set_u32(portal + 0xc, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_8f30, &args![refr]);
        });
        assert!(calls_to(&log, PORTAL_TARGET_NOTIFY).is_empty());
        assert_eq!(calls_to(&log, EXTRA_REMOVE_PORTAL).len(), 1);
    }

    #[test]
    fn linked_door_position_follows_two_teleport_links() {
        let mut e = ownership_engine();
        e.register(LINKED_DOOR_POSITION, |_, a| (a[0] + 4).into_ret());
        let refr = ownership_refr(&mut e, 0x40);
        let door = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        let position = |e: &mut Engine| e.call(0x0056_8fa0, &args![refr]).u32();
        // nothing linked: the exe's default record
        assert_eq!(position(&mut e), NO_DOOR_POSITION);
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(me + LIST_LINK, link);
        assert_eq!(position(&mut e), NO_DOOR_POSITION);
        // the link points at a door without teleport data
        e.mem.set_u32(link, door.addr());
        assert_eq!(position(&mut e), NO_DOOR_POSITION);
        // the door has teleport data: its position
        let door_link = e.mem.alloc(0x20);
        e.mem.set_u32(door.addr() + LIST_LINK, door_link);
        assert_eq!(position(&mut e), door_link + 4);
    }

    // ======================================================================
    // Third batch: 00569000 .. 0056a4a0.

    /// Where the doubles of this batch keep the answers of the extra list
    /// accessors, relative to the list (`reference + 0x44`): the third link,
    /// the activate children, activate parents, linked children and decal
    /// lists, the emittance flag, the activate timer, the multi-bound
    /// answers and the primitive.
    const L_LINK_C: u32 = 0x38;
    const L_ACTIVATE_CHILDREN: u32 = 0x3c;
    const L_ACTIVATE_PARENTS: u32 = 0x40;
    const L_LINKED_CHILDREN: u32 = 0x44;
    const L_DECALS: u32 = 0x48;
    const L_EMITTANCE: u32 = 0x4c;
    /// Relative to the reference: its lock (`00569140`), its lock extra data,
    /// the name its virtual `+0x130` gives, its multi-bound answers.
    const R_LOCK: u32 = 0x90;
    const R_EXTRA_LOCK: u32 = 0x94;
    const R_NAME: u32 = 0x98;
    const R_HAS_MULTI_BOUND: u32 = 0x9c;
    const R_MULTI_BOUND: u32 = 0xa0;
    const R_PRIMITIVE: u32 = 0xa4;
    const R_ROOM: u32 = 0xa8;
    const R_ROOM_MASTER: u32 = 0xac;
    const R_ROOM_LINKED: u32 = 0xb0;

    fn set_list(e: &mut Engine, refr: Ptr<TESObjectREFR>, offset: u32, value: u32) {
        e.mem.set_u32(refr.addr() + 0x44 + offset, value);
    }

    fn self_slot(_: &mut Engine, a: &[u32]) -> Ret {
        a[0].into_ret()
    }

    /// An engine for this batch: the ownership accessors, the list helpers
    /// (a node is `[item, next]`; it is empty when its item is 0) and the
    /// pages of the globals it reads.
    fn third_engine() -> Engine {
        let mut e = ownership_engine();
        for page in [
            0x011c_a000,
            0x011f_4000,
            0x0101_e000,
            0x0102_4000,
            0x0102_e000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(LIST_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e
    }

    /// A reference over a base form of `kind` (ID 0x1234) whose virtuals
    /// answer: Get3D (`+0x1d0`) the dword at `OWN_NODE`, the position
    /// (`+0x1f4`) `data.Location`, the name (`+0x130`) the dword at
    /// `R_NAME`; `+0x48` and `+0x4c` do nothing. The reference's own ID is
    /// 0x5678.
    fn third_refr(e: &mut Engine, kind: u8) -> Ptr<TESObjectREFR> {
        let base = form(e, kind, 0x1234, 0);
        let slots: [(u32, AbiFn); 5] = [
            (0x1d0, slot_value::<OWN_NODE>),
            (0x1f4, location_slot),
            (0x130, slot_value::<R_NAME>),
            (0x48, noop_slot),
            (0x4c, noop_slot),
        ];
        let refr = refr_with(e, base, 0, &slots);
        e.mem.set_u32(refr.addr() + 0xc, 0x5678);
        refr
    }

    /// A list of nodes `[item, next]` over `items`; its head, 0 when empty.
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// A record `[reference, float]`, the entry of the activate children
    /// lists (and the decal list for `fn_0056a4a0`).
    fn record_of(e: &mut Engine, reference: u32, due: f32) -> u32 {
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record, reference);
        e.mem.set_f32(record + 4, due);
        record
    }

    // ---- 00569000, 00569060, 00569080 --------------------------------------

    #[test]
    fn linked_door_rotation_follows_two_teleport_links() {
        let mut e = ownership_engine();
        e.register(LINKED_DOOR_ROTATION, |_, a| (a[0] + 8).into_ret());
        let refr = ownership_refr(&mut e, 0x40);
        let door = ownership_refr(&mut e, 0x40);
        let me = refr.addr();
        let rotation = |e: &mut Engine| e.call(0x0056_9000, &args![refr]).u32();
        // nothing linked: the zero vector record
        assert_eq!(rotation(&mut e), ZERO_VECTOR);
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(me + LIST_LINK, link);
        assert_eq!(rotation(&mut e), ZERO_VECTOR);
        // the door has no teleport data of its own
        e.mem.set_u32(link, door.addr());
        assert_eq!(rotation(&mut e), ZERO_VECTOR);
        // it has: its rotation record
        let door_link = e.mem.alloc(0x20);
        e.mem.set_u32(door.addr() + LIST_LINK, door_link);
        assert_eq!(rotation(&mut e), door_link + 8);
    }

    #[test]
    fn map_marker_data_and_its_sibling_read_the_list() {
        let mut e = engine();
        e.register(EXTRA_GET_MAP_MARKER_DATA, |_, a| (a[0] + 1).into_ret());
        e.register(EXTRA_GET_UNNAMED_B, |_, a| (a[0] + 2).into_ret());
        let refr = refr_of_kind(&mut e, 0x40);
        let list = refr.addr() + 0x44;
        assert_eq!(e.call(0x0056_9060, &args![refr]).u32(), list + 1);
        assert_eq!(e.call(0x0056_9080, &args![refr]).u32(), list + 2);
    }

    // ---- 005690a0 .. 005692d0: the lock -------------------------------------

    #[test]
    fn add_lock_creates_a_lock_only_when_there_is_none() {
        let mut e = third_engine();
        e.register(LOCK_OF_REFERENCE, |e, a| {
            e.mem.u32(a[0] + R_LOCK).into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOCK_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xc0de);
            a[0].into_ret()
        });
        e.register(EXTRA_SET_LOCK, |e, a| {
            e.mem.set_u32(a[0] - 0x44 + R_LOCK, a[1]);
            Ret::default()
        });
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        // an existing lock is returned as it is
        e.mem.set_u32(me + R_LOCK, 0x555);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_90a0, &args![refr]).u32(), 0x555);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // none: a 0x14 byte lock is made, constructed and stored
        e.mem.set_u32(me + R_LOCK, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_90a0, &args![refr]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let lock = e.mem.u32(me + R_LOCK);
        assert_ne!(lock, 0);
        assert_eq!(e.mem.u32(lock), 0xc0de);
        assert_eq!(calls_to(&log, EXTRA_SET_LOCK), vec![vec![me + 0x44, lock]]);
        // a failed allocation is carried on as a null lock
        e.mem.set_u32(me + R_LOCK, 0);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_90a0, &args![refr]).u32(), 0);
        });
        assert!(calls_to(&log, LOCK_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, EXTRA_SET_LOCK), vec![vec![me + 0x44, 0]]);
    }

    #[test]
    fn get_lock_falls_back_to_the_linked_door() {
        let mut e = third_engine();
        e.register(LOCK_OF_REFERENCE, |e, a| {
            e.mem.u32(a[0] + R_LOCK).into_ret()
        });
        let refr = third_refr(&mut e, 0x40);
        let door = third_refr(&mut e, 0x40);
        let me = refr.addr();
        // own lock first
        e.mem.set_u32(me + R_LOCK, 0x111);
        assert_eq!(e.call(0x0056_9160, &args![refr]).u32(), 0x111);
        e.mem.set_u32(me + R_LOCK, 0);
        // no teleport data
        assert_eq!(e.call(0x0056_9160, &args![refr]).u32(), 0);
        // teleport data whose door is null
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(me + LIST_LINK, link);
        assert_eq!(e.call(0x0056_9160, &args![refr]).u32(), 0);
        // a door: its lock
        e.mem.set_u32(link, door.addr());
        e.mem.set_u32(door.addr() + R_LOCK, 0x222);
        assert_eq!(e.call(0x0056_9160, &args![refr]).u32(), 0x222);
    }

    #[test]
    fn get_lock_level_asks_the_lock_for_this_reference() {
        let mut e = third_engine();
        e.register(LOCK_OF_REFERENCE, |e, a| {
            e.mem.u32(a[0] + R_LOCK).into_ret()
        });
        e.register(LOCK_GET_LEVEL, |_, a| (a[0] + a[1]).into_ret());
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_91b0, &args![refr]).u32(), 0);
        });
        assert!(calls_to(&log, LOCK_GET_LEVEL).is_empty());
        e.mem.set_u32(me + R_LOCK, 0x100);
        assert_eq!(e.call(0x0056_91b0, &args![refr]).u32(), 0x100 + me);
    }

    #[test]
    fn get_extra_lock_follows_the_linked_door() {
        let mut e = third_engine();
        e.register(EXTRA_OF_REFERENCE, |e, a| {
            if a[1] == 0x2a {
                e.mem.u32(a[0] + R_EXTRA_LOCK).into_ret()
            } else {
                Ret::default()
            }
        });
        let refr = third_refr(&mut e, 0x40);
        let door = third_refr(&mut e, 0x40);
        let me = refr.addr();
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(me + LIST_LINK, link);
        e.mem.set_u32(link, door.addr());
        assert_eq!(e.call(0x0056_91f0, &args![refr]).u32(), 0);
        e.mem.set_u32(door.addr() + R_EXTRA_LOCK, 0x333);
        assert_eq!(e.call(0x0056_91f0, &args![refr]).u32(), 0x333);
        // its own comes first
        e.mem.set_u32(me + R_EXTRA_LOCK, 0x444);
        assert_eq!(e.call(0x0056_91f0, &args![refr]).u32(), 0x444);
    }

    #[test]
    fn lock_and_unlock_act_on_the_lock_extra_data() {
        let mut e = third_engine();
        e.register(EXTRA_OF_REFERENCE, |e, a| {
            e.mem.u32(a[0] + R_EXTRA_LOCK).into_ret()
        });
        stub(
            &mut e,
            &[LOCK_SET_LOCKED, LOCK_EXTRA_UNLOCK, ADD_LOCK_CHANGE],
        );
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        // without lock extra data nothing happens
        let log = logged(&mut e, |e| {
            e.call(0x0056_9250, &args![refr]);
            e.call(0x0056_92a0, &args![refr]);
        });
        assert!(calls_to(&log, LOCK_SET_LOCKED).is_empty());
        assert!(calls_to(&log, LOCK_EXTRA_UNLOCK).is_empty());
        assert!(calls_to(&log, ADD_LOCK_CHANGE).is_empty());
        // with it: locked through the record at +0xc, or unlocked, and noted
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 0xc, 0x777);
        e.mem.set_u32(me + R_EXTRA_LOCK, extra);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9250, &args![refr]);
        });
        assert_eq!(calls_to(&log, LOCK_SET_LOCKED), vec![vec![0x777, 1]]);
        assert_eq!(calls_to(&log, ADD_LOCK_CHANGE), vec![vec![me]]);
        let log = logged(&mut e, |e| {
            e.call(0x0056_92a0, &args![refr]);
        });
        assert_eq!(calls_to(&log, LOCK_EXTRA_UNLOCK), vec![vec![extra]]);
        assert_eq!(calls_to(&log, ADD_LOCK_CHANGE), vec![vec![me]]);
        // the helper on its own
        let log = logged(&mut e, |e| {
            e.call(0x0056_9280, &args![extra]);
        });
        assert_eq!(calls_to(&log, LOCK_SET_LOCKED), vec![vec![0x777, 1]]);
    }

    // ---- 005692d0, 00569350, 005693d0 --------------------------------------

    /// A reference over a base form of `kind` with a vtable giving the
    /// getter (`+0x188`: argument + 100) and setter (`+0x184`: argument * 2).
    fn health_refr(e: &mut Engine, kind: u8) -> Ptr<TESObjectREFR> {
        let base = object_with_vtable(
            e,
            0x100,
            &[
                (0x188, |_, a| (a[1] + 100).into_ret()),
                (0x184, |_, a| (a[1] * 2).into_ret()),
            ],
        );
        e.mem.set_u8(base + 4, kind);
        refr_with(e, base, 0, &[])
    }

    #[test]
    fn base_form_byte_getter_asks_forms_of_type_0x25() {
        let mut e = third_engine();
        returns(&mut e, EXTRA_GET_BYTE, 5);
        let health = health_refr(&mut e, 0x25);
        assert_eq!(e.call(0x0056_92d0, &args![health]).u32(), 105);
        let other = health_refr(&mut e, 0x24);
        assert_eq!(e.call(0x0056_92d0, &args![other]).u32(), 0);
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert_eq!(e.call(0x0056_92d0, &args![bare]).u32(), 0);
    }

    #[test]
    fn base_form_byte_setter_stores_the_forms_answer() {
        let mut e = third_engine();
        stub(&mut e, &[EXTRA_SET_BYTE]);
        let health = health_refr(&mut e, 0x25);
        let list = health.addr() + 0x44;
        let log = logged(&mut e, |e| {
            e.call(0x0056_9350, &args![health, 0x21u32]);
        });
        assert_eq!(calls_to(&log, EXTRA_SET_BYTE), vec![vec![list, 0x42]]);
        // another type of base form: nothing
        let other = health_refr(&mut e, 0x24);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9350, &args![other, 0x21u32]);
        });
        assert!(calls_to(&log, EXTRA_SET_BYTE).is_empty());
        // no base form: nothing to ask
        let bare = refr_with(&mut e, 0, 0, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9350, &args![bare, 0x21u32]);
        });
        assert!(calls_to(&log, EXTRA_SET_BYTE).is_empty());
        // no list
        let nulled = health.addr();
        e.register_double(GET_EXTRA_LIST, move |_, a| {
            if a[0] == nulled {
                Ret::default()
            } else {
                (a[0] + 0x44).into_ret()
            }
        });
        let log = logged(&mut e, |e| {
            e.call(0x0056_9350, &args![health, 0x21u32]);
        });
        assert!(calls_to(&log, EXTRA_SET_BYTE).is_empty());
    }

    #[test]
    fn x_target_is_read_for_the_two_globals_forms_only() {
        let mut e = third_engine();
        returns(&mut e, EXTRA_GET_X_TARGET, 0x1234);
        let first = form(&mut e, 0x40, 1, 0);
        let second = form(&mut e, 0x40, 2, 0);
        let third = form(&mut e, 0x40, 3, 0);
        e.set_global(GLOBAL_TARGET_FORM_A, first);
        e.set_global(GLOBAL_TARGET_FORM_B, second);
        let refr = refr_with(&mut e, first, 0, &[]);
        assert_eq!(e.call(0x0056_93d0, &args![refr]).u32(), 0x1234);
        let refr = refr_with(&mut e, second, 0, &[]);
        assert_eq!(e.call(0x0056_93d0, &args![refr]).u32(), 0x1234);
        let refr = refr_with(&mut e, third, 0, &[]);
        assert_eq!(e.call(0x0056_93d0, &args![refr]).u32(), 0);
    }

    // ---- 00569410, 005694c0, 00569580 --------------------------------------

    /// A 3D node for the emittance search: +0x20 its property, +0x24 its
    /// child count, +0x28.. its children; its virtual `+0xc` gives itself
    /// when `is_node`.
    fn emittance_node(e: &mut Engine, has_property: bool, is_node: bool, children: &[u32]) -> u32 {
        let node = if is_node {
            object_with_vtable(e, 0x100, &[(0xc, self_slot)])
        } else {
            object_with_vtable(e, 0x100, &[(0xc, noop_slot)])
        };
        e.mem.set_u32(node + 0x20, u32::from(has_property));
        e.mem.set_u32(node + 0x24, children.len() as u32);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(node + 0x28 + 4 * i as u32, *child);
        }
        node
    }

    fn emittance_engine() -> Engine {
        let mut e = third_engine();
        returns(&mut e, EMITTANCE_PROPERTY_TYPE, 9);
        e.register(NODE_GET_PROPERTY, |e, a| {
            // the property is the node's +0x20 word when the type matches
            if a[1] == 9 {
                e.mem.u32(a[0] + 0x20).into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(PROPERTY_IS_KIND, |_, a| {
            (a[1] == 0x1d && a[0] == 1).into_ret()
        });
        e.register(NODE_CHILD_COUNT, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(NODE_CHILD_AT, |e, a| {
            e.mem.u32(a[0] + 0x28 + 4 * a[1]).into_ret()
        });
        e
    }

    #[test]
    fn emittance_search_looks_at_the_node_then_its_children() {
        let mut e = emittance_engine();
        // null
        assert!(!e.call(0x0056_9410, &args![0u32]).bool());
        // the node itself has the property: its children are not asked
        let child = emittance_node(&mut e, true, false, &[]);
        let node = emittance_node(&mut e, true, true, &[child]);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_9410, &args![node]).bool());
        });
        assert!(calls_to(&log, NODE_CHILD_COUNT).is_empty());
        // a child has it
        let leaf = emittance_node(&mut e, false, false, &[]);
        let node = emittance_node(&mut e, false, true, &[leaf, child]);
        assert!(e.call(0x0056_9410, &args![node]).bool());
        // a grandchild has it
        let middle = emittance_node(&mut e, false, true, &[child]);
        let node = emittance_node(&mut e, false, true, &[middle]);
        assert!(e.call(0x0056_9410, &args![node]).bool());
        // nothing has it
        let leaf = emittance_node(&mut e, false, false, &[]);
        let plain = emittance_node(&mut e, false, true, &[leaf]);
        assert!(!e.call(0x0056_9410, &args![plain]).bool());
        // a node that does not cast to a node is not searched below
        let not_a_node = emittance_node(&mut e, false, false, &[child]);
        assert!(!e.call(0x0056_9410, &args![not_a_node]).bool());
    }

    #[test]
    fn emittance_flag_is_the_lists() {
        let mut e = third_engine();
        e.register(EXTRA_HAS_EMITTANCE, |e, a| {
            e.mem.u32(a[0] + L_EMITTANCE).into_ret()
        });
        let refr = third_refr(&mut e, 0x40);
        assert_eq!(e.call(0x0056_9580, &args![refr]).u32(), 0);
        set_list(&mut e, refr, L_EMITTANCE, 1);
        assert_eq!(e.call(0x0056_9580, &args![refr]).u32(), 1);
    }

    #[test]
    fn external_emittance_check_covers_the_forms_and_the_3d() {
        let mut e = emittance_engine();
        e.register(EXTRA_HAS_EMITTANCE, |e, a| {
            e.mem.u32(a[0] + L_EMITTANCE).into_ret()
        });
        stub(&mut e, &[LOG_MESSAGE]);
        let name = cstr(&mut e, "Thing");
        // no base form: false
        let bare = refr_with(&mut e, 0, 0, &[]);
        assert!(!e.call(0x0056_94c0, &args![bare]).bool());
        // base form type 0x1e with emittance data: true at once
        let refr = third_refr(&mut e, 0x1e);
        set_list(&mut e, refr, L_EMITTANCE, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_94c0, &args![refr]).bool());
        });
        assert!(calls_to(&log, NODE_GET_PROPERTY).is_empty());
        // any other type without a 3D: false, silently
        let refr = third_refr(&mut e, 0x40);
        e.mem.set_u32(refr.addr() + R_NAME, name);
        set_list(&mut e, refr, L_EMITTANCE, 1);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0056_94c0, &args![refr]).bool());
        });
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        // a 3D without the property while the data exists: false and logged
        let plain = emittance_node(&mut e, false, true, &[]);
        e.mem.set_u32(refr.addr() + OWN_NODE, plain);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0056_94c0, &args![refr]).bool());
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![EMITTANCE_MESSAGE, name, 0x5678]]
        );
        // without the data: false, silently
        set_list(&mut e, refr, L_EMITTANCE, 0);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0056_94c0, &args![refr]).bool());
        });
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        // a 3D with the property: true
        let lit = emittance_node(&mut e, true, true, &[]);
        e.mem.set_u32(refr.addr() + OWN_NODE, lit);
        assert!(e.call(0x0056_94c0, &args![refr]).bool());
    }

    // ---- 005695a0, 00569620 -------------------------------------------------

    #[test]
    fn bound_test_needs_a_valid_3d_and_the_answer_two() {
        let mut e = third_engine();
        e.register(NODE_IS_VALID, |_, a| (a[0] & 1).into_ret());
        e.register(GET_CONTROLLER, |_, a| a[0].into_ret());
        e.register(NODE_WORLD_BOUND, |_, a| (a[0] + 0x10).into_ret());
        let refr = third_refr(&mut e, 0x40);
        let controller =
            object_with_vtable(&mut e, 0x100, &[(0x98, |e, a| e.mem.u32(a[1]).into_ret())]);
        // no 3D
        assert!(!e.call(0x0056_95a0, &args![refr, controller]).bool());
        // a 3D that is not valid (even address)
        let node = e.mem.alloc(0x40);
        let even = node & !1;
        e.mem.set_u32(refr.addr() + OWN_NODE, even);
        assert!(!e.call(0x0056_95a0, &args![refr, controller]).bool());
        // a valid one: the controller's answer for the world bound decides
        let odd = even | 1;
        e.mem.set_u32(refr.addr() + OWN_NODE, odd);
        e.mem.set_u32(odd + 0x10, 2);
        assert!(e.call(0x0056_95a0, &args![refr, controller]).bool());
        e.mem.set_u32(odd + 0x10, 1);
        assert!(!e.call(0x0056_95a0, &args![refr, controller]).bool());
    }

    #[test]
    fn position_is_copied_to_the_other_object() {
        let mut e = third_engine();
        let refr = third_refr(&mut e, 0x40);
        let other = object_with_vtable(
            &mut e,
            0x100,
            &[(0x8c, |e, a| {
                for i in 0..3 {
                    let word = e.mem.u32(a[1] + 4 * i);
                    e.mem.set_u32(a[0] + 0x20 + 4 * i, word);
                }
                Ret::default()
            })],
        );
        for i in 0..3 {
            e.mem.set_u32(refr.addr() + 0x30 + 4 * i, 0x10 + i);
        }
        e.call(0x0056_9620, &args![refr, other]);
        assert_eq!(e.mem.u32(other + 0x20), 0x10);
        assert_eq!(e.mem.u32(other + 0x24), 0x11);
        assert_eq!(e.mem.u32(other + 0x28), 0x12);
    }

    // ---- 00569670 -----------------------------------------------------------

    /// An object for the walk: +0 the "skipped kind" flag, +4 the cast
    /// record it answers to, +8 the range (its first dword: begin works),
    /// +0xc the point count, +0x10 the child count, +0x14.. the children.
    fn walk_object(e: &mut Engine, kind: u32, cast: u32, points: u32, children: &[u32]) -> u32 {
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, kind);
        e.mem.set_u32(object + 4, cast);
        e.mem.set_u32(object + 8, 1);
        e.mem.set_u32(object + 0xc, points);
        e.mem.set_u32(object + 0x10, children.len() as u32);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(object + 0x14 + 4 * i as u32, *child);
        }
        object
    }

    fn walk_engine() -> Engine {
        let mut e = third_engine();
        e.register(KIND_TEST, |e, a| (e.mem.u32(a[1]) == 1).into_ret());
        e.register(CHECKED_CAST, |e, a| {
            if e.mem.u32(a[1] + 4) == a[0] {
                a[1].into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(POINTS_RANGE_OF, |_, a| (a[0] + 8).into_ret());
        e.register(CURSOR_CONSTRUCT, |_, _| Ret::default());
        e.register(RANGE_BEGIN, |e, a| e.mem.u32(a[0]).into_ret());
        stub(&mut e, &[RANGE_FILL, RANGE_END]);
        e.register(POINTS_COUNT, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(CURSOR_READ_POINT, |e, a| {
            e.mem.set_u32(a[2], a[1]);
            Ret::default()
        });
        e.register(POINTS_TRANSFORM_OF, |_, a| a[0].into_ret());
        // the transform adds 1000 to the first word
        e.register(TRANSFORM_POINT, |e, a| {
            let first = e.mem.u32(a[2]);
            e.mem.set_u32(a[1], first + 1000);
            a[1].into_ret()
        });
        e.register(GROUP_CHILD_COUNT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NODE_CHILD_AT, |e, a| {
            e.mem.u32(a[0] + 0x14 + 4 * a[1]).into_ret()
        });
        e
    }

    /// A visitor that records the first word of each point it gets (from
    /// +0x28) and refuses the point whose word is the dword at +0x20.
    fn visitor(e: &mut Engine, refuse: u32) -> u32 {
        let visitor = object_with_vtable(
            e,
            0x100,
            &[(0x8c, |e, a| {
                let word = e.mem.u32(a[1]);
                let count = e.mem.u32(a[0] + 0x24);
                e.mem.set_u32(a[0] + 0x28 + 4 * count, word);
                e.mem.set_u32(a[0] + 0x24, count + 1);
                (word != e.mem.u32(a[0] + 0x20)).into_ret()
            })],
        );
        e.mem.set_u32(visitor + 0x20, refuse);
        visitor
    }

    fn seen(e: &Engine, visitor: u32) -> Vec<u32> {
        (0..e.mem.u32(visitor + 0x24))
            .map(|i| e.mem.u32(visitor + 0x28 + 4 * i))
            .collect()
    }

    #[test]
    fn walk_of_the_skipped_kind_returns_the_found_byte() {
        let mut e = walk_engine();
        let object = walk_object(&mut e, 1, CAST_RECORD_POINTS, 3, &[]);
        let visitor = visitor(&mut e, 0);
        let found = e.mem.alloc(4);
        e.mem.set_u8(found, 7);
        assert_eq!(
            e.call(0x0056_9670, &args![object, visitor, found, 1u32])
                .u8(),
            7
        );
        assert!(seen(&e, visitor).is_empty());
    }

    #[test]
    fn walk_hands_the_transformed_points_to_the_visitor() {
        let mut e = walk_engine();
        let object = walk_object(&mut e, 0, CAST_RECORD_POINTS, 3, &[]);
        let found = e.mem.alloc(4);
        // all accepted
        let accepting = visitor(&mut e, 99);
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0056_9670, &args![object, accepting, found, 1u32])
                    .u8(),
                1
            );
        });
        assert_eq!(seen(&e, accepting), vec![1000, 1001, 1002]);
        assert_eq!(calls_to(&log, RANGE_BEGIN).len(), 1);
        assert_eq!(calls_to(&log, RANGE_END).len(), 1);
        // one refused with stop set: the walk ends with 0 at that point
        let refusing = visitor(&mut e, 1001);
        e.mem.set_u8(found, 0);
        assert_eq!(
            e.call(0x0056_9670, &args![object, refusing, found, 1u32])
                .u8(),
            0
        );
        assert_eq!(seen(&e, refusing), vec![1000, 1001]);
        // without stop every point is visited, and the walk answers 0
        let refusing = visitor(&mut e, 1001);
        assert_eq!(
            e.call(0x0056_9670, &args![object, refusing, found, 0u32])
                .u8(),
            0
        );
        assert_eq!(seen(&e, refusing), vec![1000, 1001, 1002]);
        assert_eq!(e.mem.u8(found), 1);
        // the range cannot begin
        e.mem.set_u32(object + 8, 0);
        let accepting = visitor(&mut e, 99);
        assert_eq!(
            e.call(0x0056_9670, &args![object, accepting, found, 1u32])
                .u8(),
            0
        );
        assert!(seen(&e, accepting).is_empty());
    }

    #[test]
    fn walk_goes_through_the_children_of_a_group() {
        let mut e = walk_engine();
        let found = e.mem.alloc(4);
        e.mem.set_u8(found, 3);
        let skipped = walk_object(&mut e, 1, 0, 0, &[]);
        let two_points = walk_object(&mut e, 0, CAST_RECORD_POINTS, 2, &[]);
        let group = walk_object(&mut e, 0, CAST_RECORD_GROUP, 0, &[skipped, two_points]);
        let accepting = visitor(&mut e, 99);
        assert_eq!(
            e.call(0x0056_9670, &args![group, accepting, found, 1u32])
                .u8(),
            1
        );
        assert_eq!(seen(&e, accepting), vec![1000, 1001]);
        // a child that answers 0 ends the group's walk
        let refusing = visitor(&mut e, 1000);
        let group = walk_object(&mut e, 0, CAST_RECORD_GROUP, 0, &[two_points, skipped]);
        assert_eq!(
            e.call(0x0056_9670, &args![group, refusing, found, 1u32])
                .u8(),
            0
        );
        assert_eq!(seen(&e, refusing), vec![1000]);
        // an object of neither kind is just the found byte
        let plain = walk_object(&mut e, 0, 0, 0, &[]);
        e.mem.set_u8(found, 9);
        assert_eq!(
            e.call(0x0056_9670, &args![plain, accepting, found, 1u32])
                .u8(),
            9
        );
    }

    // ---- 00569840 .. 00569ba0: multi-bound and list accessors ---------------

    fn multi_bound_engine() -> Engine {
        let mut e = third_engine();
        e.register(HAS_MULTI_BOUND_DATA, |e, a| {
            e.mem.u32(a[0] + R_HAS_MULTI_BOUND).into_ret()
        });
        e.register(EXTRA_GET_MULTI_BOUND_REFERENCE, |e, a| {
            e.mem.u32(a[0] - 0x44 + R_MULTI_BOUND).into_ret()
        });
        e.register(EXTRA_GET_MULTI_BOUND, |e, a| {
            e.mem.u32(a[0] - 0x44 + R_MULTI_BOUND).into_ret()
        });
        e.register(EXTRA_SET_MULTI_BOUND, |e, a| {
            e.mem.set_u32(a[0] - 0x44 + R_MULTI_BOUND, a[1]);
            Ret::default()
        });
        e.register(EXTRA_GET_PRIMITIVE, |e, a| {
            e.mem.u32(a[0] - 0x44 + R_PRIMITIVE).into_ret()
        });
        e.register(EXTRA_GET_ROOM_IS_MASTER, |e, a| {
            e.mem.u32(a[0] - 0x44 + R_ROOM_MASTER).into_ret()
        });
        e.register(EXTRA_GET_ROOM, |e, a| {
            e.mem.u32(a[0] - 0x44 + R_ROOM).into_ret()
        });
        e.register(EXTRA_SET_ROOM, |e, a| {
            e.mem.set_u32(a[0] - 0x44 + R_ROOM, a[1]);
            Ret::default()
        });
        e.register(EXTRA_GET_ROOM_LINKED, |e, a| {
            e.mem.u32(a[0] - 0x44 + R_ROOM_LINKED).into_ret()
        });
        e
    }

    #[test]
    fn multi_bound_reference_needs_multi_bound_data() {
        let mut e = multi_bound_engine();
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        e.mem.set_u32(me + R_MULTI_BOUND, 0x4242);
        assert_eq!(e.call(0x0056_9840, &args![refr]).u32(), 0);
        e.mem.set_u32(me + R_HAS_MULTI_BOUND, 1);
        assert_eq!(e.call(0x0056_9840, &args![refr]).u32(), 0x4242);
    }

    #[test]
    fn half_extent_comes_from_the_primitive_the_reference_or_the_default() {
        let mut e = multi_bound_engine();
        e.register(PRIMITIVE_CENTRE, |e, a| {
            e.mem.set_u32(a[1], 0xce);
            a[1].into_ret()
        });
        e.register(PRIMITIVE_HALF_EXTENT, |_, a| (a[0] + 0x18).into_ret());
        e.register(VECTORS_DIFFER, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        e.register(VECTOR_COPY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        // nothing: the default record
        assert_eq!(e.call(0x0056_9880, &args![refr]).u32(), DEFAULT_HALF_EXTENT);
        // only the multi-bound reference: its own record (the node itself)
        let bound = e.mem.alloc(0x20);
        e.mem.set_u32(me + R_HAS_MULTI_BOUND, 1);
        e.mem.set_u32(me + R_MULTI_BOUND, bound);
        assert_eq!(e.call(0x0056_9880, &args![refr]).u32(), bound);
        // a primitive alone: its half extent
        let primitive = e.mem.alloc(0x40);
        e.mem.set_u32(me + R_PRIMITIVE, primitive);
        e.mem.set_u32(me + R_MULTI_BOUND, 0);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_9880, &args![refr]).u32(), primitive + 0x18);
        });
        assert!(calls_to(&log, VECTOR_COPY).is_empty());
        // both: a differing centre is copied into the reference's record
        e.mem.set_u32(me + R_MULTI_BOUND, bound);
        e.mem.set_u32(bound, 0x1);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_9880, &args![refr]).u32(), primitive + 0x18);
        });
        assert_eq!(calls_to(&log, VECTOR_COPY).len(), 1);
        assert_eq!(e.mem.u32(bound), 0xce);
        // an equal one is left alone
        let log = logged(&mut e, |e| {
            e.call(0x0056_9880, &args![refr]);
        });
        assert!(calls_to(&log, VECTOR_COPY).is_empty());
        assert_eq!(e.call(0x0056_9910, &args![]).u32(), DEFAULT_HALF_EXTENT);
    }

    #[test]
    fn multi_bound_is_created_once() {
        let mut e = multi_bound_engine();
        e.register(CREATE_MULTI_BOUND, |_, _| 0x9999u32.into_ret());
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        // without multi-bound data
        assert_eq!(e.call(0x0056_9920, &args![refr]).u32(), 0);
        e.mem.set_u32(me + R_HAS_MULTI_BOUND, 1);
        // deleted
        e.mem.set_u32(me + 8, 0x20);
        assert_eq!(e.call(0x0056_9920, &args![refr]).u32(), 0);
        e.mem.set_u32(me + 8, 0);
        // none yet: created and stored
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_9920, &args![refr]).u32(), 0x9999);
        });
        assert_eq!(calls_to(&log, CREATE_MULTI_BOUND), vec![vec![me]]);
        assert_eq!(e.mem.u32(me + R_MULTI_BOUND), 0x9999);
        // it exists: no new one
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_9920, &args![refr]).u32(), 0x9999);
        });
        assert!(calls_to(&log, CREATE_MULTI_BOUND).is_empty());
        // clearing
        e.call(0x0056_9990, &args![refr]);
        assert_eq!(e.mem.u32(me + R_MULTI_BOUND), 0);
    }

    #[test]
    fn multi_bound_room_is_created_and_linked_with_the_listed_references() {
        let mut e = multi_bound_engine();
        e.register(CREATE_ROOM, |_, _| 0x7007u32.into_ret());
        stub(&mut e, &[EXTRA_LINK_ROOM]);
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        let other_a = third_refr(&mut e, 0x40).addr();
        let other_b = third_refr(&mut e, 0x40).addr();
        // without multi-bound data, deleted, or not a room master
        assert_eq!(e.call(0x0056_99b0, &args![refr]).u32(), 0);
        e.mem.set_u32(me + R_HAS_MULTI_BOUND, 1);
        e.mem.set_u32(me + 8, 0x20);
        assert_eq!(e.call(0x0056_99b0, &args![refr]).u32(), 0);
        e.mem.set_u32(me + 8, 0);
        assert_eq!(e.call(0x0056_99b0, &args![refr]).u32(), 0);
        e.mem.set_u32(me + R_ROOM_MASTER, 1);
        // an existing room is returned as it is
        e.mem.set_u32(me + R_ROOM, 0x6006);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_99b0, &args![refr]).u32(), 0x6006);
        });
        assert!(calls_to(&log, CREATE_ROOM).is_empty());
        e.mem.set_u32(me + R_ROOM, 0);
        // none: made, stored, and every reference of the linked list (its
        // nodes start at +8) linked both ways; a null item ends the walk
        let nodes = list_of(&mut e, &[other_a, other_b, 0, other_a]);
        let linked = e.mem.alloc(0x20);
        // the list is embedded at +8 of the linked data
        e.mem.set_u32(linked + 8, e.mem.u32(nodes));
        e.mem.set_u32(linked + 12, e.mem.u32(nodes + 4));
        e.mem.set_u32(me + R_ROOM_LINKED, linked);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_99b0, &args![refr]).u32(), 0x7007);
        });
        assert_eq!(e.mem.u32(me + R_ROOM), 0x7007);
        assert_eq!(calls_to(&log, CREATE_ROOM), vec![vec![me]]);
        assert_eq!(
            calls_to(&log, EXTRA_LINK_ROOM),
            vec![
                vec![other_a + 0x44, me],
                vec![me + 0x44, other_a],
                vec![other_b + 0x44, me],
                vec![me + 0x44, other_b],
            ]
        );
    }

    #[test]
    fn linked_reference_accessors_use_their_list_functions() {
        let mut e = multi_bound_engine();
        returns(&mut e, EXTRA_GET_LINK_A, 0xa1);
        returns(&mut e, EXTRA_GET_LINK_B, 0xb1);
        returns(&mut e, EXTRA_GET_LINK_C, 0xc1);
        stub(
            &mut e,
            &[EXTRA_SET_LINK_A, EXTRA_SET_LINK_B, EXTRA_SET_LINK_C],
        );
        returns(&mut e, 0x0048_4e60, 0);
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        let list = me + 0x44;
        assert_eq!(e.call(0x0056_9ac0, &args![refr]).u32(), 0xa1);
        assert_eq!(e.call(0x0056_9b20, &args![refr]).u32(), 0xb1);
        assert_eq!(e.call(0x0056_9b80, &args![refr]).u32(), 0xc1);
        // 00569ae0: stores another reference only if it has multi-bound data
        // and is not this one; null is stored too
        let other = third_refr(&mut e, 0x40).addr();
        let log = logged(&mut e, |e| {
            e.call(0x0056_9ae0, &args![refr, other]);
            e.call(0x0056_9ae0, &args![refr, 0u32]);
            e.call(0x0056_9ae0, &args![refr, me]);
        });
        assert_eq!(calls_to(&log, EXTRA_SET_LINK_A), vec![vec![list, 0]]);
        e.mem.set_u32(other + R_HAS_MULTI_BOUND, 1);
        e.mem.set_u32(me + R_HAS_MULTI_BOUND, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9ae0, &args![refr, other]);
            e.call(0x0056_9ae0, &args![refr, me]);
        });
        assert_eq!(calls_to(&log, EXTRA_SET_LINK_A), vec![vec![list, other]]);
        // 00569b40 and 00569ba0: stored unless a non-null reference does not
        // persist (form flag 0x400)
        let log = logged(&mut e, |e| {
            e.call(0x0056_9b40, &args![refr, other]);
            e.call(0x0056_9ba0, &args![refr, other]);
            e.call(0x0056_9b40, &args![refr, 0u32]);
            e.call(0x0056_9ba0, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, EXTRA_SET_LINK_B), vec![vec![list, 0]]);
        assert_eq!(calls_to(&log, EXTRA_SET_LINK_C), vec![vec![list, 0]]);
        e.mem.set_u32(other + 8, 0x400);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9b40, &args![refr, other]);
            e.call(0x0056_9ba0, &args![refr, other]);
        });
        assert_eq!(calls_to(&log, EXTRA_SET_LINK_B), vec![vec![list, other]]);
        assert_eq!(calls_to(&log, EXTRA_SET_LINK_C), vec![vec![list, other]]);
    }

    // ---- 00569be0, 00569cb0: the link line ---------------------------------

    /// A node whose children lookup (`+0x9c`) answers the dword at +0x30
    /// and whose casts (`+0xc`) answer itself; its removals and
    /// attachments are logged through their slot addresses.
    fn line_node(e: &mut Engine, child: u32) -> u32 {
        let node = object_with_vtable(
            e,
            0x100,
            &[
                (0xc, self_slot),
                (0x9c, slot_value::<0x30>),
                (0xe8, noop_slot),
                (0xdc, noop_slot),
            ],
        );
        e.mem.set_u32(node + 0x30, child);
        node
    }

    fn line_engine() -> Engine {
        let mut e = third_engine();
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[1].into_ret());
        stub(&mut e, &[FIXED_STRING_DESTRUCT]);
        e
    }

    #[test]
    fn draw_ref_link_removes_the_link_line_child() {
        let mut e = line_engine();
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        // no 3D
        let log = logged(&mut e, |e| {
            e.call(0x0056_9be0, &args![refr, 1u32]);
        });
        assert!(calls_to(&log, FIXED_STRING_CONSTRUCT).is_empty());
        // a node without the child: looked up by name, nothing removed
        let node = line_node(&mut e, 0);
        e.mem.set_u32(me + OWN_NODE, node);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9be0, &args![refr, 1u32]);
        });
        assert_eq!(calls_to(&log, FIXED_STRING_CONSTRUCT)[0][1], LINK_LINE_NAME);
        assert!(calls_to(&log, slot_address(&e, node, 0xe8)).is_empty());
        assert_eq!(calls_to(&log, FIXED_STRING_DESTRUCT).len(), 1);
        // with it: removed
        e.mem.set_u32(node + 0x30, 0x1ee);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9be0, &args![refr, 1u32]);
        });
        assert_eq!(
            calls_to(&log, slot_address(&e, node, 0xe8)),
            vec![vec![node, 0x1ee]]
        );
    }

    /// The doubles of the visited set: the object is `[count, entries...]`.
    fn visited_set(e: &mut Engine) {
        e.register(VISITED_CONSTRUCT, |_, _| Ret::default());
        e.register(VISITED_FIND, |e, a| {
            let count = e.mem.u32(a[0]);
            (0..count)
                .any(|i| e.mem.u32(a[0] + 4 + 4 * i) == a[1])
                .into_ret()
        });
        e.register(VISITED_INSERT, |e, a| {
            let count = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4 + 4 * count, a[1]);
            e.mem.set_u32(a[0], count + 1);
            Ret::default()
        });
        stub(e, &[VISITED_REMOVE_ALL, VISITED_DESTRUCT]);
    }

    #[test]
    fn redraw_ref_links_walks_the_link_and_the_children_once() {
        let mut e = line_engine();
        visited_set(&mut e);
        e.register(EXTRA_GET_LINK_C, |e, a| {
            e.mem.u32(a[0] + L_LINK_C).into_ret()
        });
        e.register(EXTRA_GET_LINKED_CHILDREN, |e, a| {
            e.mem.u32(a[0] + L_LINKED_CHILDREN).into_ret()
        });
        let a = third_refr(&mut e, 0x40);
        let b = third_refr(&mut e, 0x40);
        let c = third_refr(&mut e, 0x40);
        let mut nodes = vec![];
        for refr in [a, b, c] {
            let node = line_node(&mut e, 0x1ee);
            e.mem.set_u32(refr.addr() + OWN_NODE, node);
            nodes.push(node);
        }
        // a links to b; c is a linked child of a
        set_list(&mut e, a, L_LINK_C, b.addr());
        let children = list_of(&mut e, &[c.addr()]);
        set_list(&mut e, a, L_LINKED_CHILDREN, children);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9cb0, &args![a, 1u32, 0u32]);
        });
        // a and c have their line removed (b links to nothing)
        let removed: Vec<u32> = nodes
            .iter()
            .map(|node| calls_to(&log, slot_address(&e, *node, 0xe8)).len() as u32)
            .collect();
        assert_eq!(removed, vec![1, 0, 1]);
        // each reference once, in walking order
        let inserted: Vec<u32> = calls_to(&log, VISITED_INSERT)
            .iter()
            .map(|args| args[1])
            .collect();
        assert_eq!(inserted, vec![a.addr(), b.addr(), c.addr()]);
        // every frame builds, empties and destroys its own set
        assert_eq!(calls_to(&log, VISITED_CONSTRUCT).len(), 3);
        assert_eq!(calls_to(&log, VISITED_REMOVE_ALL).len(), 3);
        assert_eq!(calls_to(&log, VISITED_DESTRUCT).len(), 3);
        assert_eq!(calls_to(&log, VISITED_CONSTRUCT)[0][1], VISITED_BUCKETS);
        // a reference already in the given set is skipped
        let set = e.mem.alloc(0x40);
        e.mem.set_u32(set, 1);
        e.mem.set_u32(set + 4, a.addr());
        let log = logged(&mut e, |e| {
            e.call(0x0056_9cb0, &args![a, 1u32, set]);
        });
        assert!(calls_to(&log, VISITED_INSERT).is_empty());
        assert_eq!(calls_to(&log, VISITED_DESTRUCT).len(), 1);
        assert!(calls_to(&log, VISITED_REMOVE_ALL).is_empty());
    }

    // ---- 00569df0, 0056a1e0: the activate line ------------------------------

    fn activate_engine() -> Engine {
        let mut e = line_engine();
        e.register(EXTRA_GET_ACTIVATE_PARENTS, |e, a| {
            e.mem.u32(a[0] + L_ACTIVATE_PARENTS).into_ret()
        });
        e.register(EXTRA_GET_ACTIVATE_CHILDREN, |e, a| {
            e.mem.u32(a[0] + L_ACTIVATE_CHILDREN).into_ret()
        });
        stub(
            &mut e,
            &[
                REMOVE_ALL_EFFECTS,
                UPDATE_PROPERTIES,
                GET_ORIENTATION,
                MATRIX_INVERSE,
                VECTOR_DIFFERENCE,
                MAKE_VELOCITY,
                SET_3D_VELOCITY,
                SET_NODE_NAME,
            ],
        );
        e.register(MATRIX_TIMES_VECTOR, |_, a| a[1].into_ret());
        e.register(VECTOR_SCALE, |_, a| a[1].into_ret());
        e.register(COLOUR_CONSTRUCT, |_, a| a[0].into_ret());
        returns(&mut e, MAKE_LINE, 0xbeef);
        e
    }

    #[test]
    fn activate_line_without_drawing_removes_an_existing_line() {
        let mut e = activate_engine();
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        let line = line_node(&mut e, 0);
        let node = line_node(&mut e, line);
        e.mem.set_u32(me + OWN_NODE, node);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9df0, &args![refr, 0u32]);
        });
        assert_eq!(
            calls_to(&log, FIXED_STRING_CONSTRUCT)[0][1],
            ACTIVATE_LINE_NAME
        );
        assert_eq!(calls_to(&log, REMOVE_ALL_EFFECTS), vec![vec![line]]);
        assert_eq!(
            calls_to(&log, slot_address(&e, node, 0xe8)),
            vec![vec![node, line]]
        );
        assert!(calls_to(&log, UPDATE_PROPERTIES).is_empty());
        // no 3D: nothing at all
        e.mem.set_u32(me + OWN_NODE, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9df0, &args![refr, 1u32]);
        });
        assert!(calls_to(&log, FIXED_STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn activate_line_is_made_for_every_parent() {
        let mut e = activate_engine();
        let refr = third_refr(&mut e, 0x40);
        let target = third_refr(&mut e, 0x40);
        let me = refr.addr();
        e.mem.set_f32(me + 0x3c, 2.0);
        let node = line_node(&mut e, 0);
        e.mem.set_u32(me + OWN_NODE, node);
        // no parents: nothing drawn, no update
        let log = logged(&mut e, |e| {
            e.call(0x0056_9df0, &args![refr, 1u32]);
        });
        assert!(calls_to(&log, UPDATE_PROPERTIES).is_empty());
        // one parent and no line yet: the line node is created, named and
        // attached once, the line geometry attached to it
        let created = line_node(&mut e, 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(LINE_NODE_CONSTRUCT, move |_, _| Ret {
            eax: created,
            ..Ret::default()
        });
        let record = record_of(&mut e, target.addr(), 0.0);
        let parents = list_of(&mut e, &[record]);
        set_list(&mut e, refr, L_ACTIVATE_PARENTS, parents);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9df0, &args![refr, 1u32]);
        });
        assert_eq!(calls_to(&log, NI_ALLOCATE), vec![vec![LINE_NODE_SIZE]]);
        assert_eq!(
            calls_to(&log, SET_NODE_NAME),
            vec![vec![created, ACTIVATE_LINE_NAME]]
        );
        assert_eq!(
            calls_to(&log, slot_address(&e, node, 0xdc)),
            vec![vec![node, created, 1]]
        );
        assert_eq!(
            calls_to(&log, slot_address(&e, created, 0xdc)),
            vec![vec![created, 0xbeef, 1]]
        );
        // the offset is divided by the scale (1 / 2.0)
        assert_eq!(calls_to(&log, VECTOR_SCALE)[0][2], 0.5f32.to_bits());
        // the line is made from the zero vector, the white and black ends
        let line_args = &calls_to(&log, MAKE_LINE)[0];
        assert_eq!(line_args.len(), 5);
        assert_eq!(line_args[4], 1);
        assert_eq!(calls_to(&log, COLOUR_CONSTRUCT).len(), 2);
        assert_eq!(
            calls_to(&log, COLOUR_CONSTRUCT)[0][1..],
            [
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                1.0f32.to_bits()
            ]
        );
        assert_eq!(
            calls_to(&log, COLOUR_CONSTRUCT)[1][1..],
            [1.0f32.to_bits(); 4]
        );
        // the 3D is updated and given a velocity record of 0.0
        assert_eq!(calls_to(&log, UPDATE_PROPERTIES), vec![vec![node]]);
        assert_eq!(
            calls_to(&log, MAKE_VELOCITY)[0][1..],
            [0.0f32.to_bits(), 0, 0]
        );
        assert_eq!(calls_to(&log, SET_3D_VELOCITY).len(), 1);
        // with a line already there, no new node is made
        let line = line_node(&mut e, 0);
        e.mem.set_u32(node + 0x30, line);
        let log = logged(&mut e, |e| {
            e.call(0x0056_9df0, &args![refr, 1u32]);
        });
        assert!(calls_to(&log, NI_ALLOCATE).is_empty());
        assert_eq!(calls_to(&log, REMOVE_ALL_EFFECTS), vec![vec![line]]);
        assert_eq!(
            calls_to(&log, slot_address(&e, line, 0xdc)),
            vec![vec![line, 0xbeef, 1]]
        );
    }

    #[test]
    fn redraw_activate_links_covers_the_children() {
        let mut e = activate_engine();
        let parent = third_refr(&mut e, 0x40);
        let child = third_refr(&mut e, 0x40);
        let mut lines = vec![];
        for refr in [parent, child] {
            let line = line_node(&mut e, 0);
            let node = line_node(&mut e, line);
            e.mem.set_u32(refr.addr() + OWN_NODE, node);
            lines.push(line);
        }
        let record = record_of(&mut e, child.addr(), 0.0);
        let children = list_of(&mut e, &[record]);
        set_list(&mut e, parent, L_ACTIVATE_CHILDREN, children);
        let log = logged(&mut e, |e| {
            e.call(0x0056_a1e0, &args![parent, 0u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_ALL_EFFECTS),
            vec![vec![lines[0]], vec![lines[1]]]
        );
    }

    // ---- 0056a250, 0056a290, 0056a380, 0056a470 -----------------------------

    #[test]
    fn activating_children_means_a_positive_timer() {
        let mut e = third_engine();
        let refr = third_refr(&mut e, 0x40);
        returns_float(&mut e, EXTRA_GET_ACTIVATE_TIMER, 1.5);
        assert!(e.call(0x0056_a250, &args![refr]).bool());
        returns_float(&mut e, EXTRA_GET_ACTIVATE_TIMER, 0.0);
        assert!(!e.call(0x0056_a250, &args![refr]).bool());
        returns_float(&mut e, EXTRA_GET_ACTIVATE_TIMER, -1.0);
        assert!(!e.call(0x0056_a250, &args![refr]).bool());
        returns_float(&mut e, EXTRA_GET_ACTIVATE_TIMER, f32::NAN);
        assert!(!e.call(0x0056_a250, &args![refr]).bool());
    }

    #[test]
    fn due_children_are_activated_and_the_others_keep_the_cell_watching() {
        let mut e = activate_engine();
        stub(&mut e, &[ACTIVATE_REFERENCE, CELL_ADD_ACTIVATING_REF]);
        let refr = third_refr(&mut e, 0x40);
        let activator = third_refr(&mut e, 0x40);
        let cell = e.mem.alloc(0x20);
        e.mem.set_u32(refr.addr() + REFR_CELL, cell);
        let due = third_refr(&mut e, 0x40);
        let waiting = third_refr(&mut e, 0x40);
        let records = [
            record_of(&mut e, due.addr(), 0.0),
            record_of(&mut e, waiting.addr(), 0.0),
        ];
        let children = list_of(&mut e, &records);
        set_list(&mut e, refr, L_ACTIVATE_CHILDREN, children);
        let mut timers = vec![-1.0f32, 2.5].into_iter();
        e.register_double(EXTRA_GET_CHILD_TIMER, move |_, _| Ret {
            st0: f64::from(timers.next().unwrap()),
            ..Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0056_a290, &args![refr, activator]);
        });
        // the timers are stored in the records
        assert_eq!(e.mem.f32(records[0] + 4), -1.0);
        assert_eq!(e.mem.f32(records[1] + 4), 2.5);
        // the due child is activated by this reference; the cell is told
        assert_eq!(
            calls_to(&log, ACTIVATE_REFERENCE),
            vec![vec![due.addr(), refr.addr(), 0, 0, 1]]
        );
        assert_eq!(
            calls_to(&log, CELL_ADD_ACTIVATING_REF),
            vec![vec![cell, refr.addr()]]
        );
        // the timer is asked of each child's list for this reference
        // (0.0 counts as due, and the activator form changes the activator)
        let special = form(&mut e, 0x40, 9, 0);
        e.set_global(GLOBAL_ACTIVATOR_FORM_B, special);
        e.mem.set_u32(refr.addr() + 0x20, special);
        let mut timers = vec![0.0f32].into_iter();
        e.register_double(EXTRA_GET_CHILD_TIMER, move |_, _| Ret {
            st0: f64::from(timers.next().unwrap()),
            ..Ret::default()
        });
        let children = list_of(&mut e, &records[..1]);
        set_list(&mut e, refr, L_ACTIVATE_CHILDREN, children);
        let log = logged(&mut e, |e| {
            e.call(0x0056_a290, &args![refr, activator]);
        });
        assert_eq!(
            calls_to(&log, ACTIVATE_REFERENCE),
            vec![vec![due.addr(), activator.addr(), 0, 0, 1]]
        );
        assert!(calls_to(&log, CELL_ADD_ACTIVATING_REF).is_empty());
    }

    #[test]
    fn child_activates_advance_the_timer_by_the_frame_time() {
        let mut e = activate_engine();
        stub(&mut e, &[ACTIVATE_REFERENCE]);
        stub(&mut e, &[EXTRA_SET_ACTIVATE_TIMER]);
        returns_float(&mut e, FRAME_TIME, 0.5);
        returns_float(&mut e, EXTRA_GET_ACTIVATE_TIMER, 1.0);
        let refr = third_refr(&mut e, 0x40);
        let me = refr.addr();
        let old = third_refr(&mut e, 0x40);
        let due = third_refr(&mut e, 0x40);
        let later = third_refr(&mut e, 0x40);
        let records = [
            record_of(&mut e, old.addr(), 0.5),
            record_of(&mut e, due.addr(), 1.25),
            record_of(&mut e, later.addr(), 2.0),
        ];
        let children = list_of(&mut e, &records);
        set_list(&mut e, refr, L_ACTIVATE_CHILDREN, children);
        // the old one is skipped, the due one activated, the later one keeps
        // the update pending and the timer moves on to 1.5
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0056_a380, &args![refr]).bool());
        });
        assert_eq!(
            calls_to(&log, ACTIVATE_REFERENCE),
            vec![vec![due.addr(), me, 0, 0, 1]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SET_ACTIVATE_TIMER),
            vec![vec![me + 0x44, 1.5f32.to_bits()]]
        );
        assert!(calls_to(&log, slot_address(&e, me, 0x4c)).is_empty());
        // nothing pending: the reference is told and the timer restarts
        let children = list_of(&mut e, &records[..2]);
        set_list(&mut e, refr, L_ACTIVATE_CHILDREN, children);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0056_a380, &args![refr]).bool());
        });
        assert_eq!(
            calls_to(&log, slot_address(&e, me, 0x4c)),
            vec![vec![me, 0x0400_0000]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SET_ACTIVATE_TIMER),
            vec![vec![me + 0x44, 0.0f32.to_bits()]]
        );
    }

    #[test]
    fn three_word_setter_ignores_a_null_first_word() {
        let mut e = third_engine();
        stub(&mut e, &[EXTRA_SET_THREE_WORDS]);
        let refr = third_refr(&mut e, 0x40);
        let list = refr.addr() + 0x44;
        let log = logged(&mut e, |e| {
            e.call(0x0056_a470, &args![refr, 0u32, 2u32, 3u32]);
            e.call(0x0056_a470, &args![refr, 1u32, 2u32, 3u32]);
        });
        assert_eq!(
            calls_to(&log, EXTRA_SET_THREE_WORDS),
            vec![vec![list, 1, 2, 3]]
        );
    }

    // ---- 0056a4a0: decals ----------------------------------------------------

    /// Everything the decal function reads, with the decal manager entry
    /// double copying the 0x80 byte data record to `kept`. Returns the
    /// engine, the reference, the decal reference, the base form of the
    /// decal reference and `kept`.
    fn decal_fixture() -> (Engine, Ptr<TESObjectREFR>, Ptr<TESObjectREFR>, u32, u32) {
        let mut e = activate_engine();
        e.set_global(BYTE_DIVISOR, 255.0f64);
        e.set_global(DECAL_FLOAT_40_DEFAULT, 48.0f32);
        e.set_global(DECAL_FLOAT_58_SOURCE, 90.0f32);
        e.set_global(DECAL_SIZE_FACTOR, 128.0f64);
        stub(
            &mut e,
            &[
                DECAL_REFRESH,
                DECAL_DATA_CONSTRUCT,
                SMART_POINTER_CONSTRUCT,
                SMART_POINTER_DESTRUCT,
            ],
        );
        e.register(EXTRA_GET_DECALS, |e, a| {
            e.mem.u32(a[0] + L_DECALS).into_ret()
        });
        e.register(DECAL_HAS_HOLDER, |e, a| {
            (e.mem.u32(a[0] + 0x80) != 0).into_ret()
        });
        e.register(DECAL_HOLDER, |e, a| e.mem.u32(a[0] + 0x80).into_ret());
        let kept = e.mem.alloc(0x80);
        e.register_double(DECAL_MANAGER_ADD, move |e, a| {
            for i in 0..0x20 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(kept + 4 * i, word);
            }
            Ret::default()
        });
        let refr = third_refr(&mut e, 0x40);
        let node = line_node(&mut e, 0);
        e.mem.set_u32(refr.addr() + OWN_NODE, node);
        let cell = e.mem.alloc(0x20);
        e.mem.set_u32(refr.addr() + REFR_CELL, cell);
        // the decal reference: scale 2.0; its base form fills the model
        // pointer through the virtual at +0x90 of its +0x30 subobject
        let decal_reference = third_refr(&mut e, 0x40);
        e.mem.set_f32(decal_reference.addr() + 0x3c, 2.0);
        let model = object_with_vtable(
            &mut e,
            0x100,
            &[
                (0x98, |_, _| 640u32.into_ret()),
                (0x94, |_, _| 128u32.into_ret()),
            ],
        );
        let base = e.mem.alloc(0x200);
        let subobject_vtable = e.mem.alloc(0x200);
        let fill = fake_function(&mut e, |e, a| {
            let model = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[2], model);
            Ret::default()
        });
        e.mem.set_u32(subobject_vtable + 0x90, fill);
        e.mem.set_u32(base + 0x30, subobject_vtable);
        e.mem.set_u32(base + 0x40, model);
        e.mem.set_u32(decal_reference.addr() + 0x20, base);
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record, decal_reference.addr());
        for i in 0..3 {
            e.mem.set_f32(record + 4 + 4 * i, 10.0 + i as f32);
            e.mem.set_f32(record + 0x10 + 4 * i, 20.0 + i as f32);
        }
        let decals = list_of(&mut e, &[record]);
        set_list(&mut e, refr, L_DECALS, decals);
        (e, refr, decal_reference, base, kept)
    }

    #[test]
    fn decal_without_a_holder_is_sized_from_the_model_and_the_scale() {
        let (mut e, refr, decal_reference, base, kept) = decal_fixture();
        let node = e.mem.u32(refr.addr() + OWN_NODE);
        let cell = e.mem.u32(refr.addr() + REFR_CELL);
        let log = logged(&mut e, |e| {
            e.call(0x0056_a4a0, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, DECAL_REFRESH).len(), 1);
        assert_eq!(
            calls_to(&log, DECAL_MANAGER_ADD)[0][0],
            cell,
            "the parent cell's manager"
        );
        assert_eq!(calls_to(&log, DECAL_MANAGER_ADD)[0][2..], [4, 0]);
        // position and rotation come from the decal record
        assert_eq!(e.mem.f32(kept), 10.0);
        assert_eq!(e.mem.f32(kept + 8), 12.0);
        assert_eq!(e.mem.f32(kept + 0xc), 20.0);
        assert_eq!(e.mem.f32(kept + 0x14), 22.0);
        assert_eq!(e.mem.u32(kept + DECAL_NODE), node);
        assert_eq!(e.mem.u32(kept + DECAL_BASE_FORM), base);
        // (640 / 128) * 128.0 * 2.0
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_38), 1280.0);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_3C), 1280.0);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_40), 48.0);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_58), 90.0);
        assert_eq!(e.mem.u8(kept + DECAL_BYTE_70), 0xff);
        assert_eq!(e.mem.u8(kept + DECAL_BYTE_72), 0);
        let _ = decal_reference;
        // the smart pointer is built and released around it
        assert_eq!(calls_to(&log, SMART_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn decal_with_a_holder_takes_its_values() {
        let (mut e, refr, _decal_reference, base, kept) = decal_fixture();
        // the holder answers the floats and flags, the base form dword gives
        // the colour bytes
        let holder = e.mem.alloc(0x80);
        e.mem.set_u32(base + 0x80, holder);
        e.mem.set_u32(holder + 0x20, 0x00ff_8040);
        returns_float(&mut e, HOLDER_FLOAT_A, 1.0);
        returns_float(&mut e, HOLDER_FLOAT_B, 3.0);
        returns_float(&mut e, HOLDER_FLOAT_F, 5.0);
        returns_float(&mut e, HOLDER_FLOAT_C, 7.0);
        returns_float(&mut e, HOLDER_FLOAT_D, 9.0);
        returns_float(&mut e, HOLDER_FLOAT_E, -2.0);
        returns_float(&mut e, HOLDER_DISTANCE, 4.5);
        e.register(RANDOM_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]) * 100.0 + f32::from_bits(a[1])),
            ..Ret::default()
        });
        returns(&mut e, HOLDER_FLAG_A, 1);
        returns(&mut e, HOLDER_FLAG_B, 2);
        returns(&mut e, HOLDER_FLAG_C, 1);
        returns(&mut e, HOLDER_FLAG_D, 3);
        e.register(MAKE_COLOUR, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.call(0x0056_a4a0, &args![refr, 0u32]);
        // the random floats take (second getter, first getter) and (the
        // fourth, the third)
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_38), 301.0);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_3C), 705.0);
        // the colour is (low, middle, high) byte / 255
        assert_eq!(
            e.mem.f32(kept + DECAL_COLOUR),
            (f64::from(0x40) / 255.0) as f32
        );
        assert_eq!(
            e.mem.f32(kept + DECAL_COLOUR + 4),
            (f64::from(0x80) / 255.0) as f32
        );
        assert_eq!(e.mem.f32(kept + DECAL_COLOUR + 8), 1.0);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_40), 9.0);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_54), -2.0);
        assert_eq!(e.mem.u8(kept + DECAL_FLAG_A), 1);
        assert_eq!(e.mem.u8(kept + DECAL_FLAG_B), 2);
        assert_eq!(e.mem.u8(kept + DECAL_BYTE_73), 1);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_4C), 4.5);
        assert_eq!(e.mem.u8(kept + DECAL_FLAG_D), 3);
        assert_eq!(e.mem.f32(kept + DECAL_FLOAT_58), 90.0);
        // without the distance flag the distance and its flags stay
        returns(&mut e, HOLDER_FLAG_C, 0);
        e.call(0x0056_a4a0, &args![refr, 0u32]);
        assert_eq!(e.mem.u8(kept + DECAL_BYTE_73), 0);
    }

    #[test]
    fn position_visit_answers_the_visitors_value() {
        let mut e = third_engine();
        let refr = third_refr(&mut e, 0x40);
        let other = object_with_vtable(&mut e, 0x100, &[(0x8c, |_, _| 1u32.into_ret())]);
        assert_eq!(e.call(0x0056_9620, &args![refr, other]).u32(), 1);
    }
}
