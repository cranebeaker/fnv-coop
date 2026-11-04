//! `fallout shared/bgsopencloseform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual slot (byte offset) of the target that takes a `float` and a zero.
const SLOT_FLOAT_AND_ZERO: u32 = 0x8c;

// Translated from 0047b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bgsopencloseform.cpp`). `thiscall`, `RET 8` (an object and a
/// `float`); the `this` register is saved and never used. Calls virtual slot +0x8c of the
/// object with the `float` (copied with `FLD`/`FSTP`, so a signalling NaN is passed quiet) and
/// a zero, in that order. The decompiler's third argument (`this`) is not in the disassembly.
pub fn fn_0047b220(e: &mut Engine, _unused_this: Ptr, object: Ptr, value: f32) {
    e.vcall(
        object.addr(),
        SLOT_FLOAT_AND_ZERO,
        &args![x87_quiet_bits(value.to_bits()), 0u32],
    );
}

/// The form type getter (`00401170`, `thiscall`).
const GET_FORM_TYPE: u32 = 0x0040_1170;
/// The form type that the activation, open state and sequence code treat as
/// a door (0x1C, compared with `00401170`'s answer; also the type
/// `TESObjectDOOR::IsSlidingDoor` is asked about).
const DOOR_FORM_TYPE: u32 = 0x1c;
/// `BGSSaveFormBuffer::GetForm` (`007af430`, engine map; `thiscall`): the
/// reference's base form.
const GET_BASE_FORM: u32 = 0x007a_f430;
/// The two sequence-name globals (`011c61dc`: the open sequence's,
/// `011c61e0`: the close sequence's, by how `GetOpenState` uses them).
const OPEN_SEQUENCE_NAME: u32 = 0x011c_61dc;
const CLOSE_SEQUENCE_NAME: u32 = 0x011c_61e0;
/// `0043b1b0` (`thiscall`, `modelloader.cpp`): a name's string (used both
/// for the key `0047a520` looks up and for a sequence's name).
const NAME_STRING: u32 = 0x0043_b1b0;
/// `00853130` (`thiscall`: key, out value): the map lookup at +0x50.
const MAP_GET_AT: u32 = 0x0085_3130;
/// `00a34f20` (`thiscall`, six words; `nicontrollersequence.cpp`).
const SEQUENCE_ACTIVATE: u32 = 0x00a3_4f20;
/// `-FLT_MAX` source: `FLT_MAX` at `0101ab70`, negated with `FCHS`.
const FLT_MAX_GLOBAL: u32 = 0x0101_ab70;
/// `0098adb0` (`thiscall` on a sequence, one float).
const SEQUENCE_SET_TIME: u32 = 0x0098_adb0;
/// `5.0` (`0101712c`): the float `0047aaf0` stores when the flag is set.
const FIVE: u32 = 0x0101_712c;
/// The TLS slot index (`0126fd98`); `0047b200` reads the word at +0x264 of
/// the thread's block.
const TLS_INDEX_WORD_OFFSET: u32 = 0x264;
/// The scene graph's node lookup (`0043b230`, `thiscall` on a node) and the
/// controller manager lookup (`00653270`, `cdecl`: `011f36ac`, the node).
const NODE_ROOT_0043B230: u32 = 0x0043_b230;
const CONTROLLER_MANAGER_LOOKUP: u32 = 0x0065_3270;
const CONTROLLER_MANAGER_TABLE: u32 = 0x011f_36ac;
/// `LowProcess::GetGenericLocation` (`008041a0`, engine map; `thiscall` on
/// a sequence): the sequence state (1 while it runs, by its use here).
const SEQUENCE_STATE: u32 = 0x0080_41a0;
/// `TESObjectREFR` action flags: `00572d30` (test), `00572d50` (set) and
/// `TESObjectREFR::ClearAction` (`00572db0`), all `thiscall` with one word.
const TEST_ACTION: u32 = 0x0057_2d30;
const SET_ACTION: u32 = 0x0057_2d50;
const CLEAR_ACTION: u32 = 0x0057_2db0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`) and its type descriptors.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_6a3c;
const CAST_TARGET_TYPE: u32 = 0x0118_6530;
/// The open and close sounds of the cast source (`00407820`, `00407840`,
/// `thiscall`).
const OPEN_SOUND: u32 = 0x0040_7820;
const CLOSE_SOUND: u32 = 0x0040_7840;
/// The boolean setting accessor (`00408d60`, `thiscall` on the setting:
/// the address of its byte) and the two settings read here (`011c4c64`,
/// `011d7188`; names not confirmed).
const SETTING_BYTE: u32 = 0x0040_8d60;
const SETTING_011C4C64: u32 = 0x011c_4c64;
const SETTING_011D7188: u32 = 0x011d_7188;
/// The player pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// `TESObjectDOOR::IsSlidingDoor` (`00518080`, engine map; `thiscall`).
const IS_SLIDING_DOOR: u32 = 0x0051_8080;
/// `008c7aa0` (no arguments): whether work goes through the task queue
/// (`004537b0`, no arguments) instead of running here.
const USE_TASK_QUEUE: u32 = 0x008c_7aa0;
const TASK_QUEUE: u32 = 0x0045_37b0;
/// `TaskQueueInterface::QueueHavokRemove` (`0087b140`), `QueueSetOpenState`
/// (`0087b600`), and `0087b0f0`, `0087b540`, `0087b580` (all `thiscall` on
/// the queue).
const QUEUE_HAVOK_REMOVE: u32 = 0x0087_b140;
const QUEUE_SET_OPEN_STATE: u32 = 0x0087_b600;
const QUEUE_0087B0F0: u32 = 0x0087_b0f0;
const QUEUE_0087B540: u32 = 0x0087_b540;
const QUEUE_0087B580: u32 = 0x0087_b580;
/// `bhkWorld::Activate` (`00c6a270`, `cdecl`, four words),
/// `bhkWorld::RemoveObjects` (`00c69ee0`, `cdecl`, three words) and
/// `bhkNiCollisionObject::ResetSim` (`00c6bd00`, `cdecl`, two words).
const HAVOK_ACTIVATE: u32 = 0x00c6_a270;
const HAVOK_REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
const HAVOK_RESET_SIM: u32 = 0x00c6_bd00;
/// The sound handle constructor (`0041a250`), its assignment (`00418900`,
/// one word), an empty function (`00483710`, it returns at once),
/// `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByNumericID`
/// (`00ad73b0`, `thiscall`: out handle, id, flags), `BSSoundHandle::
/// SetPosition` (`00ad8b60`) and `Play` (`00ad8830`); the sound's numeric id
/// (`0084e3a0`, `thiscall`).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const EMPTY_00483710: u32 = 0x0048_3710;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NUMERIC_ID: u32 = 0x00ad_73b0;
const SOUND_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_NUMERIC_ID: u32 = 0x0084_e3a0;
/// The reference's extra data list (`005d43c0`, `thiscall`) with
/// `ExtraDataList::SetOpenCloseActivateRef` (`0041e690`), its getter
/// (`0041e660`) and `ExtraDataList::SetRagDollData` (`0041d490`).
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
const SET_OPEN_CLOSE_ACTIVATE_REF: u32 = 0x0041_e690;
const GET_OPEN_CLOSE_ACTIVATE_REF: u32 = 0x0041_e660;
const SET_RAG_DOLL_DATA: u32 = 0x0041_d490;
/// `Script::SetActionFlag` (`005ac750`, `cdecl`: reference, list, flag).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// The sequence's name holder (`00413f40`, `thiscall`) and `strcmp`
/// (`00ec6da0`); `"Open"` (`01011f30`) and `"Close"` (`0101abac`).
const SEQUENCE_NAME: u32 = 0x0041_3f40;
const STRCMP: u32 = 0x00ec_6da0;
const OPEN_TEXT: u32 = 0x0101_1f30;
const CLOSE_TEXT: u32 = 0x0101_abac;
/// The navmesh obstacle manager (`006c0720`, no arguments) with its door
/// opened (`006c0dc0`) and `NavMeshObstacleManager::OnDoorClose`
/// (`006c0f10`), both one word.
const OBSTACLE_MANAGER: u32 = 0x006c_0720;
const OBSTACLE_DOOR_OPEN: u32 = 0x006c_0dc0;
const OBSTACLE_DOOR_CLOSE: u32 = 0x006c_0f10;
/// `00568e50` (`thiscall` on the reference).
const REFERENCE_00568E50: u32 = 0x0056_8e50;
/// `TESObjectREFR::SetLastFinishedSequence` (`00578a30`, one word).
const SET_LAST_FINISHED_SEQUENCE: u32 = 0x0057_8a30;
/// `00759450` (`thiscall` on a sequence, ST0), `0043d410` (`thiscall` on a
/// 12-byte local: a float and two words) and `00a59c60` (`thiscall` on the
/// 3D, the local).
const SEQUENCE_00759450: u32 = 0x0075_9450;
const UPDATE_DATA_0043D410: u32 = 0x0043_d410;
const NODE_UPDATE_00A59C60: u32 = 0x00a5_9c60;

/// The boolean setting at `setting`, through its accessor.
fn setting_on(e: &mut Engine, setting: u32) -> bool {
    let byte = e.call(SETTING_BYTE, &args![setting]).u32();
    e.mem.u8(byte) != 0
}

/// The reference's 3D (`TESObjectREFR` vtable slot +0x1D0).
fn reference_3d(e: &mut Engine, reference: u32) -> u32 {
    e.vcall(reference, 0x1d0, &[]).u32()
}

/// The form type of the reference's base form.
fn base_form_type(e: &mut Engine, reference: u32) -> u32 {
    let base = e.call(GET_BASE_FORM, &args![reference]).u32();
    e.call(GET_FORM_TYPE, &args![base]).u32()
}

/// `strcmp(name of the sequence, text) == 0`.
fn sequence_named(e: &mut Engine, sequence: u32, text: u32) -> bool {
    let holder = e.call(SEQUENCE_NAME, &args![sequence]).u32();
    let name = e.call(NAME_STRING, &args![holder]).u32();
    e.call(STRCMP, &args![name, text]).u32() == 0
}

/// The controller manager of the reference's 3D (`0043b230` on the 3D, then
/// `00653270`), or 0 without 3D.
fn controller_manager(e: &mut Engine, reference: u32) -> u32 {
    let node = reference_3d(e, reference);
    let root = if node == 0 {
        0
    } else {
        e.call(NODE_ROOT_0043B230, &args![node]).u32()
    };
    e.call(
        CONTROLLER_MANAGER_LOOKUP,
        &args![CONTROLLER_MANAGER_TABLE, root],
    )
    .u32()
}

/// `-FLT_MAX` as `FLD`, `FCHS`, `FSTP` leave it.
fn minus_flt_max(e: &Engine) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(FLT_MAX_GLOBAL)) ^ 0x8000_0000)
}

/// The sequence restart both `SetOpenState` branches run on `sequence`:
/// its time to `-FLT_MAX` (`0098adb0`), a 12-byte update block built from
/// `00759450`'s float with 1 and 0 (`0043d410`) and the reference's 3D
/// updated with it (`00a59c60`).
fn restart_sequence(e: &mut Engine, reference: u32, sequence: u32) {
    let time = minus_flt_max(e);
    e.call(SEQUENCE_SET_TIME, &args![sequence, time]);
    let value = e.call(SEQUENCE_00759450, &args![sequence]).f64();
    e.with_stack(0xc, |e, block| {
        e.call(
            UPDATE_DATA_0043D410,
            &args![block, x87_store32(value), 1u32, 0u32],
        );
        let node = reference_3d(e, reference);
        e.call(NODE_UPDATE_00A59C60, &args![node, block]);
    });
}

// Translated from 0047a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSOpenCloseForm::GetAsOpenCloseForm` (Xbox PDB; `cdecl`): for a form
/// `IsOpenCloseForm` accepts, the address of its `BGSOpenCloseForm` part by
/// form type: +0x70 for types 0x15, 0x17 and 0x1C, +0x84 for type 0x1B;
/// otherwise null.
pub fn bgs_open_close_form_get_as_open_close_form(e: &mut Engine, form: u32) -> u32 {
    if form == 0 || !bgs_open_close_form_is_open_close_form(e, form) {
        return 0;
    }
    match e.call(GET_FORM_TYPE, &args![form]).u32().wrapping_sub(0x15) {
        0 | 2 | 7 => form + 0x70,
        6 => form + 0x84,
        _ => 0,
    }
}

// Translated from 0047a490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSOpenCloseForm::IsOpenCloseForm` (Xbox PDB; `cdecl`): true for a form
/// of type 0x15, 0x17, 0x1B or 0x1C. (Types 0x16, 0x26 and 0x27 set a
/// second local the function never returns.)
pub fn bgs_open_close_form_is_open_close_form(e: &mut Engine, form: u32) -> bool {
    if form == 0 {
        return false;
    }
    matches!(
        e.call(GET_FORM_TYPE, &args![form]).u32().wrapping_sub(0x15),
        0 | 2 | 6 | 7
    )
}

// Translated from 0047a500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at `011c61dc` (the
/// open sequence's name).
pub fn fn_0047a500(e: &mut Engine) -> u32 {
    e.mem.u32(OPEN_SEQUENCE_NAME)
}

// Translated from 0047a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at `011c61e0` (the
/// close sequence's name).
pub fn fn_0047a510(e: &mut Engine) -> u32 {
    e.mem.u32(CLOSE_SEQUENCE_NAME)
}

// Translated from 0047a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a controller manager, one word,
/// `RET 4`): looks `name`'s string (`0043b1b0`) up in the map at +0x50
/// (`00853130`) and answers the value found, or null.
pub fn fn_0047a520(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let key = e.call(NAME_STRING, &args![name]).u32();
    e.with_stack(4, |e, value| {
        let found = e
            .call(MAP_GET_AT, &args![this.addr() + 0x50, key, value])
            .bool();
        if found {
            e.mem.u32(value.addr())
        } else {
            0
        }
    })
}

// Translation incomplete: 0047a560 - the FS:[0] exception frame (cleaning up the sound handle when a call throws) is not translated
// Translated from 0047a560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSOpenCloseForm::HandleActivate` (Xbox PDB; `cdecl`, three words).
/// With 3D, a controller manager and both sequences (open, close) found,
/// and neither sequence running (state 1): picks the close sequence when
/// the reference's action flag 4 is set (clearing it) and the open one
/// otherwise (setting it), with the matching sound of `source` cast to the
/// sound holder type (`00407840`/`00407820`); unless the setting at
/// `011c4c64` is on, an actor activator other than the player removes the
/// reference's 3D from Havok (not for a sliding door; through the task
/// queue when `008c7aa0` says so); starts the sequence (`0047aa40`,
/// `0047aab0`, time `-FLT_MAX`); plays the sound (id flags `0x80000121` for
/// the player, `0x102` positioned at the reference otherwise); records the
/// activator in the extra data (`0041e690`), marks the reference changed
/// (slot +0x48 with `0x80000000`) and, for the player opening a form whose
/// slot +8 answers true, runs `0047aaf0(1)` on the player. Without a
/// controller path the open state decides: states 1 and 2 clear action 4
/// and call the form's slot +4, others set it and call slot 0 (both with
/// the reference and the activator).
pub fn bgs_open_close_form_handle_activate(
    e: &mut Engine,
    reference: u32,
    activator: u32,
    source: u32,
) {
    if reference == 0 {
        return;
    }
    let mut managed = false;
    'controller: {
        if reference_3d(e, reference) == 0 {
            break 'controller;
        }
        let node = reference_3d(e, reference);
        if e.call(NODE_ROOT_0043B230, &args![node]).u32() == 0 {
            break 'controller;
        }
        // No null test of the 3D here (the exe has none).
        let node = reference_3d(e, reference);
        let root = e.call(NODE_ROOT_0043B230, &args![node]).u32();
        let manager = e
            .call(
                CONTROLLER_MANAGER_LOOKUP,
                &args![CONTROLLER_MANAGER_TABLE, root],
            )
            .u32();
        if manager == 0 {
            break 'controller;
        }
        let name = fn_0047a500(e);
        let open = fn_0047a520(e, Ptr::new(manager), name);
        let name = fn_0047a510(e);
        let close = fn_0047a520(e, Ptr::new(manager), name);
        if open == 0 || close == 0 {
            break 'controller;
        }
        managed = true;
        if e.call(SEQUENCE_STATE, &args![open]).u32() == 1
            || e.call(SEQUENCE_STATE, &args![close]).u32() == 1
        {
            break 'controller;
        }
        let holder = e
            .call(
                DYNAMIC_CAST,
                &args![source, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
            )
            .u32();
        let mut sound = 0u32;
        let sequence;
        if !e.call(TEST_ACTION, &args![reference, 4u32]).bool() {
            if holder != 0 {
                sound = e.call(OPEN_SOUND, &args![holder]).u32();
            }
            sequence = open;
            e.call(SET_ACTION, &args![reference, 4u32]);
        } else {
            if holder != 0 {
                sound = e.call(CLOSE_SOUND, &args![holder]).u32();
            }
            sequence = close;
            e.call(CLEAR_ACTION, &args![reference, 4u32]);
        }
        if !setting_on(e, SETTING_011C4C64)
            && activator != 0
            && e.vcall(activator, 0x100, &[]).bool()
            && activator != e.mem.u32(PLAYER)
            && reference_3d(e, reference) != 0
        {
            let mut remove = true;
            if base_form_type(e, reference) == DOOR_FORM_TYPE {
                let door = e.call(GET_BASE_FORM, &args![reference]).u32();
                if e.call(IS_SLIDING_DOOR, &args![door]).bool() {
                    remove = false;
                }
            }
            if remove {
                let node = reference_3d(e, reference);
                if e.call(USE_TASK_QUEUE, &[]).bool() {
                    let queue = e.call(TASK_QUEUE, &[]).u32();
                    e.call(QUEUE_HAVOK_REMOVE, &args![queue, node]);
                } else {
                    e.call(HAVOK_ACTIVATE, &args![node, 1u32, 1u32, 0u32]);
                    e.call(HAVOK_REMOVE_OBJECTS, &args![node, 1u32, 1u32]);
                }
            }
        }
        fn_0047aa40(e, Ptr::new(manager), 1);
        fn_0047aab0(e, Ptr::new(manager), sequence, 0, 0, 1.0, 0.0, 0);
        let time = minus_flt_max(e);
        e.call(SEQUENCE_SET_TIME, &args![sequence, time]);
        if sound != 0 {
            e.with_stack(0x18, |e, locals| {
                let handle = locals.addr();
                let temporary = locals.addr() + 0xc;
                e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
                let by_player = activator == e.mem.u32(PLAYER);
                let flags = if by_player { 0x8000_0121u32 } else { 0x102 };
                let id = e.call(SOUND_NUMERIC_ID, &args![sound]).u32();
                let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                let made = e
                    .call(
                        GET_SOUND_HANDLE_BY_NUMERIC_ID,
                        &args![audio, temporary, id, flags],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
                e.call(EMPTY_00483710, &args![temporary]);
                if !by_player {
                    let position = e.vcall(reference, 0x1f4, &[]).u32();
                    let x = x87_quiet_bits(e.mem.u32(position));
                    let y = x87_quiet_bits(e.mem.u32(position + 4));
                    let z = x87_quiet_bits(e.mem.u32(position + 8));
                    e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
                }
                e.call(SOUND_PLAY, &args![handle, 0u32]);
                e.call(EMPTY_00483710, &args![handle]);
            });
        }
        let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
        e.call(SET_OPEN_CLOSE_ACTIVATE_REF, &args![list, activator]);
        e.vcall(reference, 0x48, &args![0x8000_0000u32]);
        if sequence == open && activator == e.mem.u32(PLAYER) {
            let base = e.call(GET_BASE_FORM, &args![reference]).u32();
            let form = bgs_open_close_form_get_as_open_close_form(e, base);
            if form != 0 && e.vcall(form, 0x8, &[]).bool() {
                let player = e.mem.u32(PLAYER);
                fn_0047aaf0(e, Ptr::new(player), 1);
            }
        }
    }
    if managed {
        return;
    }
    let state = bgs_open_close_form_get_open_state(e, reference) as i32;
    let opening = !(1..=2).contains(&state);
    if opening {
        e.call(SET_ACTION, &args![reference, 4u32]);
    } else {
        e.call(CLEAR_ACTION, &args![reference, 4u32]);
    }
    let base = e.call(GET_BASE_FORM, &args![reference]).u32();
    let form = bgs_open_close_form_get_as_open_close_form(e, base);
    if form != 0 {
        let slot = if opening { 0 } else { 4 };
        e.vcall(form, slot, &args![reference, activator]);
    }
}

// Translated from 0047aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`): `0047aa60`
/// with `set` and the mask 8.
pub fn fn_0047aa40(e: &mut Engine, this: Ptr, set: u8) {
    fn_0047aa60(e, this, set, 8);
}

// Translated from 0047aa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): sets (`set` non-zero)
/// or clears the bits of `mask` in the 16-bit flags at +8.
pub fn fn_0047aa60(e: &mut Engine, this: Ptr, set: u8, mask: u16) {
    let flags = e.mem.u16(this.addr() + 8);
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u16(this.addr() + 8, flags);
}

// Translated from 0047aab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, six words, `RET 0x18`; `this` is
/// not read): `00a34f20` on `sequence` with the two bytes, the two floats
/// (through the x87 stack), `word` and 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_0047aab0(
    e: &mut Engine,
    _unused_this: Ptr,
    sequence: u32,
    first: u8,
    second: u8,
    weight: f32,
    time: f32,
    word: u32,
) {
    e.call(
        SEQUENCE_ACTIVATE,
        &args![
            sequence,
            u32::from(first),
            u32::from(second),
            x87_quiet_bits(weight.to_bits()),
            x87_quiet_bits(time.to_bits()),
            word,
            0u32
        ],
    );
}

// Translated from 0047aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`; called on the
/// player): stores `value` in the byte at +0x681 and 5.0 (`0101712c`) or
/// 0.0 in the float at +0x684 (a local copy of the same value is never
/// read).
pub fn fn_0047aaf0(e: &mut Engine, this: Ptr, value: u8) {
    let at = this.addr();
    e.mem.set_u8(at + 0x681, value);
    let stored = if value != 0 {
        x87_quiet_bits(e.mem.u32(FIVE))
    } else {
        0.0f32.to_bits()
    };
    e.mem.set_u32(at + 0x684, stored);
}

// Translated from 0047ab40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): whether `sequence` is
/// named `"Open"` or `"Close"` (false for a null reference or sequence).
/// For such a sequence, while both reads of the setting at `011c4c64` are
/// off and the reference has 3D: through the task queue (`0087b0f0`) when
/// `008c7aa0` says so, otherwise the object `004543c0` gives for the
/// reference's `008d6f30` answer gets slot +0xD0 with the 3D, 1, 1, 0, 0.
pub fn fn_0047ab40(e: &mut Engine, reference: u32, sequence: u32) -> u8 {
    if reference == 0 || sequence == 0 {
        return 0;
    }
    let named = sequence_named(e, sequence, OPEN_TEXT) || sequence_named(e, sequence, CLOSE_TEXT);
    if setting_on(e, SETTING_011C4C64) || !named || setting_on(e, SETTING_011C4C64) {
        return u8::from(named);
    }
    let node = reference_3d(e, reference);
    if node == 0 {
        return 1;
    }
    if e.call(USE_TASK_QUEUE, &[]).bool() {
        let queue = e.call(TASK_QUEUE, &[]).u32();
        e.call(QUEUE_0087B0F0, &args![queue, node]);
    } else {
        let owner = e.call(0x008d_6f30, &args![reference]).u32();
        let object = if owner == 0 {
            0
        } else {
            e.call(0x0045_43c0, &args![owner]).u32()
        };
        if object != 0 {
            e.vcall(object, 0xd0, &args![node, 1u32, 1u32, 0u32, 0u32]);
        }
    }
    1
}

// Translated from 0047ac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): a finished open or close
/// sequence of a reference. Runs `0047ab40`; for `"Open"` sets the script
/// action flag 0x10000, for `"Close"` 0x20000 (`Script::SetActionFlag`);
/// with an activator recorded in the extra data, the open-close form's slot
/// 0 (open) or +4 (close) runs with the reference and the activator, the
/// record is cleared, and the player opening a form whose slot +8 answers
/// true gets `0047aaf0(0)`. With the setting at `011d7188` on and a door
/// (type 0x1C): the navmesh obstacle manager hears of the door opening or
/// closing (through the task queue when `008c7aa0` says so).
pub fn fn_0047ac70(e: &mut Engine, reference: u32, sequence: u32) {
    if reference == 0 || sequence == 0 {
        return;
    }
    fn_0047ab40(e, reference, sequence);
    let opened = if sequence_named(e, sequence, OPEN_TEXT) {
        true
    } else if sequence_named(e, sequence, CLOSE_TEXT) {
        false
    } else {
        return;
    };
    let flag = if opened { 0x10000u32 } else { 0x20000 };
    let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
    e.call(SET_ACTION_FLAG, &args![reference, list, flag]);
    let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
    let activator = e.call(GET_OPEN_CLOSE_ACTIVATE_REF, &args![list]).u32();
    if activator != 0 {
        let base = e.call(GET_BASE_FORM, &args![reference]).u32();
        let form = bgs_open_close_form_get_as_open_close_form(e, base);
        if form != 0 {
            let slot = if opened { 0 } else { 4 };
            e.vcall(form, slot, &args![reference, activator]);
        }
        let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
        e.call(SET_OPEN_CLOSE_ACTIVATE_REF, &args![list, 0u32]);
        if opened && activator == e.mem.u32(PLAYER) && form != 0 && e.vcall(form, 0x8, &[]).bool() {
            let player = e.mem.u32(PLAYER);
            fn_0047aaf0(e, Ptr::new(player), 0);
        }
    }
    if setting_on(e, SETTING_011D7188) && base_form_type(e, reference) == DOOR_FORM_TYPE {
        if e.call(USE_TASK_QUEUE, &[]).bool() {
            let queue = e.call(TASK_QUEUE, &[]).u32();
            let target = if opened {
                QUEUE_0087B540
            } else {
                QUEUE_0087B580
            };
            e.call(target, &args![queue, reference]);
        } else {
            let manager = e.call(OBSTACLE_MANAGER, &[]).u32();
            let target = if opened {
                OBSTACLE_DOOR_OPEN
            } else {
                OBSTACLE_DOOR_CLOSE
            };
            e.call(target, &args![manager, reference]);
        }
    }
}

// Translated from 0047aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSOpenCloseForm::SetOpenState` (Xbox PDB; `cdecl`, three words): for
/// an open-close reference. When work goes through the task queue and this
/// is not the thread's reference (`0047b200`) it queues the change
/// (`QueueSetOpenState`) and answers true. Otherwise sets (open) or clears
/// action flag 4; with a controller manager and both sequences it plays
/// the target sequence (open or close) from the start: immediately (the
/// rag doll data cleared, the target stopped at 0.0 (`0047b220`), the
/// other activated, restarted and stopped, the name of the target recorded as the last finished
/// sequence and the 3D's physics reset) or not (the other stopped, the
/// target activated and restarted), answering true; without a manager it
/// records `"Open"` or `"Close"` as the last finished sequence. An
/// immediate change of a door with 3D, when the setting at `011d7188` is on
/// and `00568e50` answers 0, tells the navmesh obstacle manager.
pub fn bgs_open_close_form_set_open_state(
    e: &mut Engine,
    reference: u32,
    open: u8,
    immediate: u8,
) -> u8 {
    if reference == 0 {
        return 0;
    }
    let base = e.call(GET_BASE_FORM, &args![reference]).u32();
    if !bgs_open_close_form_is_open_close_form(e, base) {
        return 0;
    }
    if e.call(USE_TASK_QUEUE, &[]).bool() && fn_0047b200(e) != reference {
        let queue = e.call(TASK_QUEUE, &[]).u32();
        e.call(
            QUEUE_SET_OPEN_STATE,
            &args![queue, reference, u32::from(open), u32::from(immediate)],
        );
        return 1;
    }
    let action = if open != 0 { SET_ACTION } else { CLEAR_ACTION };
    e.call(action, &args![reference, 4u32]);
    let manager = controller_manager(e, reference);
    let mut done = 0u8;
    if manager != 0 {
        let name = fn_0047a500(e);
        let open_sequence = fn_0047a520(e, Ptr::new(manager), name);
        let name = fn_0047a510(e);
        let close_sequence = fn_0047a520(e, Ptr::new(manager), name);
        if open_sequence != 0 && close_sequence != 0 {
            let (target, other) = if open != 0 {
                (open_sequence, close_sequence)
            } else {
                (close_sequence, open_sequence)
            };
            let manager = Ptr::new(manager);
            fn_0047aa40(e, manager, 1);
            if immediate != 0 {
                let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
                e.call(SET_RAG_DOLL_DATA, &args![list, 0u32]);
                fn_0047b220(e, manager, Ptr::new(target), 0.0);
                fn_0047aab0(e, manager, other, 0, 0, 1.0, 0.0, 0);
                restart_sequence(e, reference, other);
                fn_0047b220(e, manager, Ptr::new(other), 0.0);
                let holder = e.call(SEQUENCE_NAME, &args![target]).u32();
                let name = e.call(NAME_STRING, &args![holder]).u32();
                e.call(SET_LAST_FINISHED_SEQUENCE, &args![reference, name]);
                let node = reference_3d(e, reference);
                e.call(HAVOK_RESET_SIM, &args![node, 1u32]);
            } else {
                fn_0047b220(e, manager, Ptr::new(other), 0.0);
                fn_0047aab0(e, manager, target, 0, 0, 1.0, 0.0, 0);
                restart_sequence(e, reference, target);
            }
            done = 1;
        }
    } else {
        let text = if open != 0 { OPEN_TEXT } else { CLOSE_TEXT };
        e.call(SET_LAST_FINISHED_SEQUENCE, &args![reference, text]);
    }
    if immediate != 0
        && setting_on(e, SETTING_011D7188)
        && base_form_type(e, reference) == DOOR_FORM_TYPE
        && reference_3d(e, reference) != 0
        && e.call(REFERENCE_00568E50, &args![reference]).u32() == 0
    {
        let manager = e.call(OBSTACLE_MANAGER, &[]).u32();
        let target = if open != 0 {
            OBSTACLE_DOOR_OPEN
        } else {
            OBSTACLE_DOOR_CLOSE
        };
        e.call(target, &args![manager, reference]);
    }
    done
}

// Translated from 0047b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at +0x264 of the
/// running thread's TLS block (`FS:[0x2C]`, index at `0126fd98`).
pub fn fn_0047b200(e: &mut Engine) -> u32 {
    let block = e.tls();
    e.mem.u32(block + TLS_INDEX_WORD_OFFSET)
}

// Translated from 0047b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSOpenCloseForm::GetOpenState` (Xbox PDB; `cdecl`): 0 for a null or
/// non-open-close reference; otherwise 1 with action flag 4 set, else 3,
/// replaced by 2 while the open sequence runs or 4 while the close one
/// runs (state 1 of `008041a0`), when the 3D has a controller manager with
/// both sequences.
pub fn bgs_open_close_form_get_open_state(e: &mut Engine, reference: u32) -> u32 {
    if reference == 0 {
        return 0;
    }
    let base = e.call(GET_BASE_FORM, &args![reference]).u32();
    if !bgs_open_close_form_is_open_close_form(e, base) {
        return 0;
    }
    let mut state = if e.call(TEST_ACTION, &args![reference, 4u32]).bool() {
        1
    } else {
        3
    };
    let manager = controller_manager(e, reference);
    if manager != 0 {
        let name = fn_0047a500(e);
        let open = fn_0047a520(e, Ptr::new(manager), name);
        let name = fn_0047a510(e);
        let close = fn_0047a520(e, Ptr::new(manager), name);
        if open != 0 && close != 0 {
            if e.call(SEQUENCE_STATE, &args![open]).u32() == 1 {
                state = 2;
            } else if e.call(SEQUENCE_STATE, &args![close]).u32() == 1 {
                state = 4;
            }
        }
    }
    state
}

/// `00633c90` (`ECX` = smart pointer, `RET 4`): `NiPointer` construction from an object
/// (takes a reference when non-null).
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `00559450` (`ECX` = smart pointer): the object it holds.
const SMART_POINTER_GET: u32 = 0x0055_9450;
/// `0045cec0` (`ECX` = smart pointer): `NiPointer` destruction (drops the reference).
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `bhkUtilFunctions::FindNextCollisionObject` (`00c802d0`, `cdecl`, a node).
const FIND_NEXT_COLLISION_OBJECT: u32 = 0x00c8_02d0;
/// `0047b470` (`ECX` = collision object, `AL`): true when bit 0x20 of the word at +0xC is
/// set and `0047b4b0` answers false.
const COLLISION_OBJECT_USES_VELOCITY: u32 = 0x0047_b470;
/// `bhkUtilFunctions::SetCOUseVel` (`00c80330`, `cdecl`: flag, node).
const SET_CO_USE_VEL: u32 = 0x00c8_0330;
/// `0043d410` (`ECX` = 12-byte update record, `RET 0xc`: a `float` time and two bytes):
/// fills the record (the remaining three bytes cleared).
const UPDATE_RECORD_CONSTRUCT: u32 = 0x0043_d410;
/// `00a59c60` (`ECX` = node, `RET 4`: the update record): runs the node's update (its
/// virtual slot 0xA4, then slot 0xFC of the object at +0x18 when there is one).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// The reference's virtual slot returning its 3D root node.
const SLOT_GET_3D: u32 = 0x1d0;

// Translated from 0047b360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047b360 - the C++ exception frame (FS:[0], unwinding the NiPointer temporary) is not translated
/// `BGSOpenCloseForm::SnapHavokTo3D` (Xbox PDB; `cdecl`, the reference): holds the
/// reference's 3D (virtual slot 0x1D0) in an `NiPointer` temporary; when there is one, a
/// collision object found under it (`FindNextCollisionObject`) that uses velocity
/// (`0047b470`) has that turned off (`SetCOUseVel(0, node)`) around an update of the node
/// (`00a59c60` with an update record of time 0.0 and flags 0, `0043d410`) and turned
/// back on afterwards (`SetCOUseVel(1, node)`). The temporary is released at the end.
pub fn bgs_open_close_form_snap_havok_to_3d(e: &mut Engine, reference: Ptr) {
    let root = e.vcall(reference.addr(), SLOT_GET_3D, &[]).u32();
    e.with_stack(0x10, |e, frame| {
        let record = frame.addr();
        let holder = frame.addr() + 0xc;
        e.call(SMART_POINTER_CONSTRUCT, &args![holder, root]);
        if e.call(SMART_POINTER_GET, &args![holder]).u32() != 0 {
            let node = e.call(SMART_POINTER_GET, &args![holder]).u32();
            let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node]).u32();
            let uses_velocity = if collision != 0 {
                e.call(COLLISION_OBJECT_USES_VELOCITY, &args![collision])
                    .u8()
            } else {
                0
            };
            if uses_velocity != 0 {
                let node = e.call(SMART_POINTER_GET, &args![holder]).u32();
                e.call(SET_CO_USE_VEL, &args![0u32, node]);
            }
            e.call(
                UPDATE_RECORD_CONSTRUCT,
                &args![record, 0.0f32.to_bits(), 0u32, 0u32],
            );
            let node = e.call(SMART_POINTER_GET, &args![holder]).u32();
            e.call(NODE_UPDATE, &args![node, record]);
            if uses_velocity != 0 {
                let node = e.call(SMART_POINTER_GET, &args![holder]).u32();
                e.call(SET_CO_USE_VEL, &args![u32::from(uses_velocity), node]);
            }
        }
        e.call(SMART_POINTER_DESTRUCT, &args![holder]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_b220, fn_0047b220(Ptr, Ptr, f32)),
        entry!(
            0x0047a3a0,
            bgs_open_close_form_get_as_open_close_form(u32) -> u32
        ),
        entry!(0x0047a490, bgs_open_close_form_is_open_close_form(u32) -> bool),
        entry!(0x0047a500, fn_0047a500() -> u32),
        entry!(0x0047a510, fn_0047a510() -> u32),
        entry!(0x0047a520, fn_0047a520(Ptr, u32) -> u32),
        entry!(
            0x0047a560,
            bgs_open_close_form_handle_activate(u32, u32, u32)
        ),
        entry!(0x0047aa40, fn_0047aa40(Ptr, u8)),
        entry!(0x0047aa60, fn_0047aa60(Ptr, u8, u16)),
        entry!(0x0047aab0, fn_0047aab0(Ptr, u32, u8, u8, f32, f32, u32)),
        entry!(0x0047aaf0, fn_0047aaf0(Ptr, u8)),
        entry!(0x0047ab40, fn_0047ab40(u32, u32) -> u8),
        entry!(0x0047ac70, fn_0047ac70(u32, u32)),
        entry!(
            0x0047aec0,
            bgs_open_close_form_set_open_state(u32, u8, u8) -> u8
        ),
        entry!(0x0047b200, fn_0047b200() -> u32),
        entry!(0x0047b250, bgs_open_close_form_get_open_state(u32) -> u32),
        entry!(0x0047_b360, bgs_open_close_form_snap_havok_to_3d(Ptr)),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 0x30];
        slots[SLOT_FLOAT_AND_ZERO as usize / 4] = 0x0f00_2000;
        e.put_vtable(0x0f00_2100, &slots);
        e.register(0x0f00_2000, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_2100);
        (e, object)
    }

    fn slot_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x0f00_2000)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_0047b220_calls_slot_0x8c_with_the_float_and_zero() {
        let (mut e, object) = rig();
        e.call(0x0047_b220, &args![0x1111u32, object, 0.75f32]);
        assert_eq!(slot_calls(&e), vec![vec![object, 0.75f32.to_bits(), 0]]);
    }

    #[test]
    fn at_0047b220_passes_a_signalling_nan_quiet() {
        let (mut e, object) = rig();
        e.call(
            0x0047_b220,
            &args![0u32, object, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(slot_calls(&e), vec![vec![object, 0x7fc0_0001, 0]]);
    }
}

#[cfg(test)]
mod open_close_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable slots up to +0x200 are recording doubles at
    /// `base + slot`, answering what `answers` gives the slot (0 otherwise).
    fn object(e: &mut Engine, base: u32, answers: Vec<(u32, u32)>) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            let value = answers
                .iter()
                .find(|(s, _)| *s == slot)
                .map_or(0, |(_, v)| *v);
            e.register_double(base + slot, move |_, _| value.into_ret());
        }
        let object = e.mem.alloc(0x700);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Every callee a no-op answering 0; the form type is the word at +4 of
    /// the form, the base form the word at +0x20 of the reference, a name's
    /// string the name itself, a sequence's name holder the word at +0 of
    /// the sequence, the action flag 4 the byte at +0x40 of the reference.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for address in [
            SEQUENCE_ACTIVATE,
            SEQUENCE_SET_TIME,
            NODE_ROOT_0043B230,
            CONTROLLER_MANAGER_LOOKUP,
            SEQUENCE_STATE,
            DYNAMIC_CAST,
            OPEN_SOUND,
            CLOSE_SOUND,
            IS_SLIDING_DOOR,
            USE_TASK_QUEUE,
            TASK_QUEUE,
            QUEUE_HAVOK_REMOVE,
            QUEUE_SET_OPEN_STATE,
            QUEUE_0087B0F0,
            QUEUE_0087B540,
            QUEUE_0087B580,
            HAVOK_ACTIVATE,
            HAVOK_REMOVE_OBJECTS,
            HAVOK_RESET_SIM,
            SOUND_HANDLE_CONSTRUCT,
            SOUND_HANDLE_ASSIGN,
            EMPTY_00483710,
            AUDIO_INSTANCE,
            GET_SOUND_HANDLE_BY_NUMERIC_ID,
            SOUND_SET_POSITION,
            SOUND_PLAY,
            SOUND_NUMERIC_ID,
            EXTRA_DATA_LIST,
            SET_OPEN_CLOSE_ACTIVATE_REF,
            GET_OPEN_CLOSE_ACTIVATE_REF,
            SET_RAG_DOLL_DATA,
            SET_ACTION_FLAG,
            OBSTACLE_MANAGER,
            OBSTACLE_DOOR_OPEN,
            OBSTACLE_DOOR_CLOSE,
            REFERENCE_00568E50,
            SET_LAST_FINISHED_SEQUENCE,
            SEQUENCE_00759450,
            UPDATE_DATA_0043D410,
            NODE_UPDATE_00A59C60,
            0x008d_6f30,
            0x0045_43c0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(GET_FORM_TYPE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(NAME_STRING, |_, a| a[0].into_ret());
        e.register(SEQUENCE_NAME, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRCMP, |_, a| u32::from(a[0] != a[1]).into_ret());
        e.register(TEST_ACTION, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x40)).into_ret()
        });
        e.register(SET_ACTION, |e, a| {
            e.mem.set_u8(a[0] + 0x40, 1);
            Ret::default()
        });
        e.register(CLEAR_ACTION, |e, a| {
            e.mem.set_u8(a[0] + 0x40, 0);
            Ret::default()
        });
        e.register(SETTING_BYTE, |_, a| a[0].into_ret());
        e.map(0x011c_4000, 0x3000);
        e.map(0x011d_7000, 0x8000);
        e.map(0x0101_1000, 0xa000);
        e.mem.set_u32(FLT_MAX_GLOBAL, f32::MAX.to_bits());
        e.mem.set_u32(FIVE, 5.0f32.to_bits());
        e.mem.set_u32(OPEN_SEQUENCE_NAME, 0x0e0e);
        e.mem.set_u32(CLOSE_SEQUENCE_NAME, 0x0c0c);
        e
    }

    /// A base form of `form_type` (its open-close part at +0x70 sharing its
    /// vtable, slot +8 answering true) and a reference over it with 3D
    /// `node` (0 for none) and the position (1, 2, 3).
    fn reference(e: &mut Engine, form_type: u32, node: u32) -> (u32, u32) {
        let base = object(e, 0x0040_0000, vec![(0x8, 1)]);
        e.mem.set_u32(base + 4, form_type);
        let vtable = e.mem.u32(base);
        e.mem.set_u32(base + 0x70, vtable);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let reference = object(e, 0x0041_0000, vec![(0x1d0, node), (0x1f4, position)]);
        e.mem.set_u32(reference + 0x20, base);
        (reference, base)
    }

    /// A controller manager answering the open sequence `0x50e0` for the name
    /// `0x0e0e` and the close sequence `0x50c0` for `0x0c0c`.
    fn with_manager(e: &mut Engine) -> u32 {
        // Both sequences share a vtable whose slot +0x8C is a no-op.
        e.map(0x5000, 0x1000);
        let mut slots = [0u32; 0x24];
        slots[0x8c / 4] = 0x0043_008c;
        e.put_vtable(0x5800, &slots);
        e.register(0x0043_008c, |_, _| Ret::default());
        e.mem.set_u32(0x50e0, 0x5800);
        e.mem.set_u32(0x50c0, 0x5800);
        let manager = e.mem.alloc(0x60);
        e.register_double(CONTROLLER_MANAGER_LOOKUP, move |_, a| {
            if a[1] != 0 {
                manager.into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(NODE_ROOT_0043B230, |_, a| (a[0] + 1).into_ret());
        e.register(MAP_GET_AT, |e, a| {
            let found = match a[1] {
                0x0e0e => 0x50e0,
                0x0c0c => 0x50c0,
                _ => 0,
            };
            e.mem.set_u32(a[2], found);
            u32::from(found != 0).into_ret()
        });
        manager
    }

    #[test]
    fn open_close_types_and_parts() {
        let mut e = rig();
        for (form_type, expected) in [(0x15u32, 0x70u32), (0x17, 0x70), (0x1c, 0x70), (0x1b, 0x84)]
        {
            let form = e.mem.alloc(0x90);
            e.mem.set_u32(form + 4, form_type);
            assert!(e.call(0x0047_a490, &args![form]).bool());
            assert_eq!(e.call(0x0047_a3a0, &args![form]).u32(), form + expected);
        }
        for form_type in [0x16u32, 0x18, 0x26, 0x14, 0x30] {
            let form = e.mem.alloc(0x90);
            e.mem.set_u32(form + 4, form_type);
            assert!(!e.call(0x0047_a490, &args![form]).bool());
            assert_eq!(e.call(0x0047_a3a0, &args![form]).u32(), 0);
        }
        assert!(!e.call(0x0047_a490, &args![0u32]).bool());
        assert_eq!(e.call(0x0047_a3a0, &args![0u32]).u32(), 0);
    }

    #[test]
    fn small_accessors() {
        let mut e = rig();
        assert_eq!(e.call(0x0047_a500, &[]).u32(), 0x0e0e);
        assert_eq!(e.call(0x0047_a510, &[]).u32(), 0x0c0c);
        let manager = with_manager(&mut e);
        assert_eq!(
            e.call(0x0047_a520, &args![manager, 0x0c0cu32]).u32(),
            0x50c0
        );
        assert_eq!(e.call(0x0047_a520, &args![manager, 0x1234u32]).u32(), 0);
        e.mem.set_u16(manager + 8, 0x11);
        e.call(0x0047_aa40, &args![manager, 1u32]);
        assert_eq!(e.mem.u16(manager + 8), 0x19);
        e.call(0x0047_aa60, &args![manager, 0u32, 0x10u32]);
        assert_eq!(e.mem.u16(manager + 8), 0x09);
        e.call_log = Some(vec![]);
        e.call(
            0x0047_aab0,
            &args![0u32, 0x50e0u32, 1u32, 0u32, 1.0f32, 0.5f32, 7u32],
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_ACTIVATE),
            vec![vec![0x50e0, 1, 0, 1.0f32.to_bits(), 0.5f32.to_bits(), 7, 0]]
        );
        let player = e.mem.alloc(0x700);
        e.call(0x0047_aaf0, &args![player, 1u32]);
        assert_eq!(e.mem.u8(player + 0x681), 1);
        assert_eq!(e.mem.f32(player + 0x684), 5.0);
        e.call(0x0047_aaf0, &args![player, 0u32]);
        assert_eq!(e.mem.f32(player + 0x684), 0.0);
        let block = e.tls();
        e.mem.set_u32(block + 0x264, 0x7777);
        assert_eq!(e.call(0x0047_b200, &[]).u32(), 0x7777);
    }

    #[test]
    fn open_state_follows_flag_and_running_sequences() {
        let mut e = rig();
        assert_eq!(e.call(0x0047_b250, &args![0u32]).u32(), 0);
        let (plain, _) = reference(&mut e, 0x30, 0);
        assert_eq!(e.call(0x0047_b250, &args![plain]).u32(), 0);
        let (door, _) = reference(&mut e, 0x1c, 0);
        assert_eq!(e.call(0x0047_b250, &args![door]).u32(), 3);
        e.mem.set_u8(door + 0x40, 1);
        assert_eq!(e.call(0x0047_b250, &args![door]).u32(), 1);
        let (door, _) = reference(&mut e, 0x1c, 0x9000);
        with_manager(&mut e);
        e.register(SEQUENCE_STATE, |_, a| u32::from(a[0] == 0x50c0).into_ret());
        assert_eq!(e.call(0x0047_b250, &args![door]).u32(), 4);
        e.register(SEQUENCE_STATE, |_, a| u32::from(a[0] == 0x50e0).into_ret());
        assert_eq!(e.call(0x0047_b250, &args![door]).u32(), 2);
    }

    #[test]
    fn set_open_state_queues_or_records() {
        let mut e = rig();
        let (door, _) = reference(&mut e, 0x1c, 0);
        e.register(USE_TASK_QUEUE, |_, _| 1u32.into_ret());
        e.register(TASK_QUEUE, |_, _| 0x7a7au32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_aec0, &args![door, 1u32, 0u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, QUEUE_SET_OPEN_STATE),
            vec![vec![0x7a7a, door, 1, 0]]
        );
        // On the thread that owns it: no manager, the sequence name only.
        let block = e.tls();
        e.mem.set_u32(block + 0x264, door);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_aec0, &args![door, 1u32, 0u32]).u8(), 0);
        assert_eq!(e.mem.u8(door + 0x40), 1);
        assert_eq!(
            calls_to(&e, SET_LAST_FINISHED_SEQUENCE),
            vec![vec![door, OPEN_TEXT]]
        );
        e.call(0x0047_aec0, &args![door, 0u32, 0u32]);
        assert_eq!(e.mem.u8(door + 0x40), 0);
    }

    #[test]
    fn set_open_state_plays_the_sequences() {
        let mut e = rig();
        let (door, _) = reference(&mut e, 0x1c, 0x9000);
        let manager = with_manager(&mut e);
        e.register(SEQUENCE_00759450, |_, _| 0.25f64.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_aec0, &args![door, 1u32, 0u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, SEQUENCE_ACTIVATE),
            vec![vec![0x50e0, 0, 0, 1.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_SET_TIME),
            vec![vec![0x50e0, (-f32::MAX).to_bits()]]
        );
        let update = calls_to(&e, UPDATE_DATA_0043D410);
        assert_eq!(&update[0][1..], [0.25f32.to_bits(), 1, 0]);
        assert_eq!(calls_to(&e, NODE_UPDATE_00A59C60)[0][0], 0x9000);
        assert_eq!(e.mem.u16(manager + 8) & 8, 8);
        assert!(calls_to(&e, HAVOK_RESET_SIM).is_empty());
        // Immediately, with the door setting on: the other sequence runs and
        // the obstacle manager hears of it.
        e.mem.set_u8(SETTING_011D7188, 1);
        e.register(OBSTACLE_MANAGER, |_, _| 0x0b0bu32.into_ret());
        e.register(SEQUENCE_NAME, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_aec0, &args![door, 0u32, 1u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, SEQUENCE_ACTIVATE),
            vec![vec![0x50e0, 0, 0, 1.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_LAST_FINISHED_SEQUENCE),
            vec![vec![door, 0x50c0]]
        );
        assert_eq!(calls_to(&e, HAVOK_RESET_SIM), vec![vec![0x9000, 1]]);
        assert_eq!(calls_to(&e, OBSTACLE_DOOR_CLOSE), vec![vec![0x0b0b, door]]);
    }

    #[test]
    fn handle_activate_without_3d_uses_the_open_state() {
        let mut e = rig();
        let (door, base) = reference(&mut e, 0x1c, 0);
        let part = base + 0x70;
        e.call_log = Some(vec![]);
        e.call(0x0047_a560, &args![door, 0x4444u32, 0u32]);
        // State 3: opening, slot 0 of the form's open-close part.
        assert_eq!(e.mem.u8(door + 0x40), 1);
        assert_eq!(calls_to(&e, 0x0040_0000), vec![vec![part, door, 0x4444]]);
        e.call_log = Some(vec![]);
        e.call(0x0047_a560, &args![door, 0x4444u32, 0u32]);
        assert_eq!(e.mem.u8(door + 0x40), 0);
        assert_eq!(calls_to(&e, 0x0040_0004), vec![vec![part, door, 0x4444]]);
    }

    #[test]
    fn handle_activate_plays_a_sequence_and_its_sound() {
        let mut e = rig();
        let (door, _) = reference(&mut e, 0x1c, 0x9000);
        with_manager(&mut e);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(OPEN_SOUND, |_, _| 0x5050u32.into_ret());
        e.register(SOUND_NUMERIC_ID, |_, _| 0x1d1du32.into_ret());
        e.register(GET_SOUND_HANDLE_BY_NUMERIC_ID, |_, a| a[1].into_ret());
        let actor = object(&mut e, 0x0042_0000, vec![(0x100, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0047_a560, &args![door, actor, 0x3333u32]);
        assert_eq!(e.mem.u8(door + 0x40), 1);
        assert_eq!(
            calls_to(&e, SEQUENCE_ACTIVATE),
            vec![vec![0x50e0, 0, 0, 1.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, HAVOK_ACTIVATE), vec![vec![0x9000, 1, 1, 0]]);
        let handle = calls_to(&e, GET_SOUND_HANDLE_BY_NUMERIC_ID);
        assert_eq!(&handle[0][2..], [0x1d1d, 0x102]);
        let positions = calls_to(&e, SOUND_SET_POSITION);
        assert_eq!(
            &positions[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(calls_to(&e, SOUND_PLAY).len(), 1);
        assert_eq!(calls_to(&e, 0x0041_0048), vec![vec![door, 0x8000_0000]]);
        // A running sequence: nothing more happens, and the open state is
        // not consulted.
        e.register(SEQUENCE_STATE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_a560, &args![door, actor, 0u32]);
        assert!(calls_to(&e, SEQUENCE_ACTIVATE).is_empty());
        assert!(calls_to(&e, 0x0040_0000).is_empty());
    }

    #[test]
    fn finished_sequence_reports_to_the_form() {
        let mut e = rig();
        let (door, base) = reference(&mut e, 0x1c, 0x9000);
        let sequence = e.mem.alloc(8);
        e.mem.set_u32(sequence, OPEN_TEXT);
        let player = e.mem.alloc(0x700);
        e.mem.set_u32(PLAYER, player);
        e.register_double(GET_OPEN_CLOSE_ACTIVATE_REF, move |_, _| player.into_ret());
        e.register(EXTRA_DATA_LIST, |_, _| 0x1e1eu32.into_ret());
        let queued = Rc::new(Cell::new(0u32));
        let seen = queued.clone();
        e.register_double(QUEUE_0087B540, move |_, a| {
            seen.set(a[1]);
            Ret::default()
        });
        e.register(USE_TASK_QUEUE, |_, _| 1u32.into_ret());
        e.mem.set_u8(SETTING_011D7188, 1);
        e.mem.set_u8(player + 0x681, 1);
        e.call_log = Some(vec![]);
        e.call(0x0047_ac70, &args![door, sequence]);
        assert_eq!(
            calls_to(&e, SET_ACTION_FLAG),
            vec![vec![door, 0x1e1e, 0x10000]]
        );
        let part = base + 0x70;
        assert_eq!(calls_to(&e, 0x0040_0000), vec![vec![part, door, player]]);
        assert_eq!(
            calls_to(&e, SET_OPEN_CLOSE_ACTIVATE_REF),
            vec![vec![0x1e1e, 0]]
        );
        // The part's slot +8 answers true: the player's flag is cleared.
        assert_eq!(e.mem.u8(player + 0x681), 0);
        assert_eq!(queued.get(), door);
        // `0047ab40` with the setting off and 3D: the queue gets the node.
        assert_eq!(calls_to(&e, QUEUE_0087B0F0)[0][1], 0x9000);
        // A sequence with another name: only `0047ab40` runs.
        e.mem.set_u32(sequence, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0047_ac70, &args![door, sequence]);
        assert!(calls_to(&e, SET_ACTION_FLAG).is_empty());
        assert_eq!(e.call(0x0047_ab40, &args![door, sequence]).u8(), 0);
        assert_eq!(e.call(0x0047_ab40, &args![0u32, sequence]).u8(), 0);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn run(root: u32, collision: u32, uses_velocity: u8) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        let reference = e.mem.alloc(0x10);
        e.put_vtable(0x0160_0000, &[0; 0x76]);
        e.mem.set_u32(0x0160_0000 + SLOT_GET_3D, 0x0000_2000);
        e.register_double(0x0000_2000, move |_, _| root.into_ret());
        e.mem.set_u32(reference, 0x0160_0000);
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(SMART_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SMART_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register_double(FIND_NEXT_COLLISION_OBJECT, move |_, _| collision.into_ret());
        e.register_double(COLLISION_OBJECT_USES_VELOCITY, move |_, _| {
            u32::from(uses_velocity).into_ret()
        });
        e.register(SET_CO_USE_VEL, |_, _| Ret::default());
        e.register(UPDATE_RECORD_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_UPDATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_b360, &args![reference]);
        e.call_log.take().unwrap()
    }

    fn to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn snap_havok_turns_velocity_off_around_the_update() {
        let log = run(0x7000, 0x7100, 1);
        assert_eq!(to(&log, FIND_NEXT_COLLISION_OBJECT), vec![vec![0x7000]]);
        assert_eq!(
            to(&log, SET_CO_USE_VEL),
            vec![vec![0, 0x7000], vec![1, 0x7000]]
        );
        let record = to(&log, UPDATE_RECORD_CONSTRUCT);
        assert_eq!(record.len(), 1);
        assert_eq!(&record[0][1..], &[0, 0, 0]);
        assert_eq!(to(&log, NODE_UPDATE), vec![vec![0x7000, record[0][0]]]);
        assert_eq!(to(&log, SMART_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn snap_havok_without_velocity_or_3d() {
        let log = run(0x7000, 0x7100, 0);
        assert!(to(&log, SET_CO_USE_VEL).is_empty());
        assert_eq!(to(&log, NODE_UPDATE).len(), 1);
        let log = run(0x7000, 0, 1);
        assert!(to(&log, COLLISION_OBJECT_USES_VELOCITY).is_empty());
        assert!(to(&log, SET_CO_USE_VEL).is_empty());
        let log = run(0, 0x7100, 1);
        assert!(to(&log, FIND_NEXT_COLLISION_OBJECT).is_empty());
        assert!(to(&log, NODE_UPDATE).is_empty());
        assert_eq!(to(&log, SMART_POINTER_DESTRUCT).len(), 1);
    }
}
