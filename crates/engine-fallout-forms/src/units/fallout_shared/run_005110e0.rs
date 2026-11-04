//! `fallout shared/run_005110e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00511950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00511950 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4d (0x3c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00511950(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0102_a0a4);
    e.mem.set_u32(base + 0x18, 0x0102_a07c);
    e.call(0x004f_15a0, &args![this, 0x4d]);
    e.mem.set_u32(base + 0x38, 0x0);
    this
}

// Translated from 00511940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `005105f0`. `005105f0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00511940(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_05f0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 005119e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00511a10` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005119e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_1a10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `BSStringT` copy constructor (`004047f0`, `thiscall`: the source).
const STRING_COPY: u32 = 0x0040_47f0;
/// `MapMarkerData::GetLocationName` (`00408da0`, engine map; `thiscall`).
const GET_LOCATION_NAME: u32 = 0x0040_8da0;
/// `TESIdleManager::SetUsedItem`, `SetUsedItemLevel`, `SetUsedItemActivate`
/// (`00600900`, `00600920`, `00600940`, engine map; `cdecl`, one word).
const SET_USED_ITEM: u32 = 0x0060_0900;
const SET_USED_ITEM_LEVEL: u32 = 0x0060_0920;
const SET_USED_ITEM_ACTIVATE: u32 = 0x0060_0940;
/// `Actor::GetEyeLevel` (`008be940`, engine map; `thiscall`, ST0).
const ACTOR_GET_EYE_LEVEL: u32 = 0x008b_e940;
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`, engine map;
/// `thiscall` on the actor).
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// The player pointer (`011dea3c`) and the word `005113f0` stores the
/// form's +0x80 in when the player activates (`011e0788`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_ACTIVATED_WORD: u32 = 0x011e_0788;
/// `004fcbd0` (`thiscall` on the player, one word).
const PLAYER_004FCBD0: u32 = 0x004f_cbd0;
/// `PlayerCharacter::GetAnimation` (`00950a60`, `thiscall`, one word) and
/// `Animation::ClearGroup` (`00496080`, `thiscall`, a word and a float).
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
const ANIMATION_CLEAR_GROUP: u32 = 0x0049_6080;
/// `TESObjectREFR::IsAnOwner` (`005785e0`, `thiscall`, two words),
/// `TESObjectREFR::GetOwner` (`00567790`, `thiscall`, no stack words) and
/// `008c0ec0` (`thiscall` on the actor, three words; `actor.cpp`).
const IS_AN_OWNER: u32 = 0x0057_85e0;
const GET_OWNER: u32 = 0x0056_7790;
const ACTOR_008C0EC0: u32 = 0x008c_0ec0;
/// `00407820` (`thiscall`, no stack words): the form's sound (the object
/// the sound code works on).
const FORM_SOUND: u32 = 0x0040_7820;
/// `004ef990` (no arguments): the word at `011c61c8`, the name the 3D node
/// is asked for (slot +0x9C).
const SOUND_NODE_NAME: u32 = 0x004e_f990;
/// `0045bb80` (`thiscall`): a node's world position.
const NODE_WORLD_POSITION: u32 = 0x0045_bb80;
/// `004e75d0` (`thiscall`, one word, an out block) and `005e39b0`
/// (`cdecl`, one word): the extra sound flags for the player.
const SOUND_004E75D0: u32 = 0x004e_75d0;
const SOUND_FLAGS_005E39B0: u32 = 0x005e_39b0;
/// `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByFilename`
/// (`00ad7480`, `thiscall`: out handle, name, flags, sound),
/// `BSSoundHandle::SetPosition` (`00ad8b60`, three floats),
/// `SetObjectToFollow` (`00ad8f20`) and `Play` (`00ad8830`); `00483710` is an
/// empty `thiscall` function (it returns at once), called on the handle
/// here and on `this` in the destructor.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_FILENAME: u32 = 0x00ad_7480;
const SOUND_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
const SOUND_PLAY: u32 = 0x00ad_8830;
const EMPTY_00483710: u32 = 0x0048_3710;
/// `005d43c0` (`thiscall` on the player) and `0041a6a0` (`thiscall`, one
/// word: the sound handle).
const PLAYER_005D43C0: u32 = 0x005d_43c0;
const EXTRA_0041A6A0: u32 = 0x0041_a6a0;
/// `BGSOpenCloseForm::HandleActivate` (`0047a560`, `cdecl`, three words)
/// and `GetOpenState` (`0047b250`, `cdecl`).
const OPEN_CLOSE_HANDLE_ACTIVATE: u32 = 0x0047_a560;
const OPEN_CLOSE_GET_OPEN_STATE: u32 = 0x0047_b250;
/// `004fd3c0` (`thiscall`) and `008325b0` (`cdecl`, two words;
/// `falloutradio.cpp`).
const FORM_004FD3C0: u32 = 0x004f_d3c0;
const RADIO_008325B0: u32 = 0x0083_25b0;
/// The destructor's vtables and calls (`00483710` on `this`, `0048a490` on
/// +0x18, `00483630` on `this`).
const VTABLE_0102A0A4: u32 = 0x0102_a0a4;
const VTABLE_0102A07C: u32 = 0x0102_a07c;
const DESTRUCT_0048A490: u32 = 0x0048_a490;
const DESTRUCT_00483630: u32 = 0x0048_3630;

// Translated from 005110e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): copies the
/// string at +0x88 into `result` (`004047f0`) and answers `result` (a string
/// returned by value).
pub fn fn_005110e0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    e.call(STRING_COPY, &args![result, this.addr() + 0x88]);
    result
}

// Translation incomplete: 005113f0 - the FS:[0] exception frame (cleaning up the sound handle when a call throws) is not translated
// Translated from 005113f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, five words, `RET 0x14`): a form's
/// activation of `reference` by `activator`. When the activator is an actor
/// (vtable +0x100): marks the used item (`TESIdleManager`), its level 1 when
/// the reference's slot +0x218 answers true or the actor's eye height
/// (`z + GetEyeLevel - slot +0x380`, as a float) is below the reference's
/// `z` (ordered), and asks the actor's saved acquire object (slot +0x44 with
/// the actor, 0, 3, 1, 0, 1); the marks are cleared again. A refusal with a
/// script-like object at +0x80 answers false. The player succeeding stores
/// +0x80 at `011e0788`, runs `004fcbd0(1)` and clears group 1 of both
/// animations; an activator that does not own the reference gets
/// `008c0ec0(reference, owner, -1)`. With a sound (`00407820`) it plays it at
/// the reference's 3D node position (slot +0x1D0, the named node through
/// slot +0x9C, else slot +0x1F4), following the 3D, flags `0x40000102` (plus
/// the player's extra flags). The object at +0x80 then gets slot +0x124 with
/// all five words, `BGSOpenCloseForm::HandleActivate(reference, activator,
/// 0)` runs, and with `004fd3c0` the radio hook `008325b0` gets whether the
/// reference is opening or open (states 1 and 2). Answers true.
pub fn fn_005113f0(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    activator: u32,
    flag: u8,
    word_4: u32,
    word_5: u32,
) -> bool {
    let at = this.addr();
    let actor = if activator != 0 && e.vcall(activator, 0x100, &[]).bool() {
        activator
    } else {
        0
    };
    let mut player_activated = false;
    if actor != 0 {
        e.call(SET_USED_ITEM, &args![at]);
        e.call(SET_USED_ITEM_ACTIVATE, &args![1u32]);
        let eye = e.call(ACTOR_GET_EYE_LEVEL, &args![actor]).f64();
        let position = e.vcall(actor, 0x1f4, &[]).u32();
        let sum = x87_add(x87_load32(e.mem.u32(position + 8)), eye);
        let offset = e.vcall(actor, 0x380, &[]).f64();
        let eye_height = x87_store32(x87_sub(sum, offset));
        let position = e.vcall(reference, 0x1f4, &[]).u32();
        let reference_z = x87_quiet_bits(e.mem.u32(position + 8));
        let level = if e.vcall(reference, 0x218, &[]).bool()
            || x87_load32(eye_height) < x87_load32(reference_z)
        {
            1u32
        } else {
            0
        };
        e.call(SET_USED_ITEM_LEVEL, &args![level]);
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        let accepted = e
            .vcall(acquire, 0x44, &args![actor, 0u32, 3u32, 1u32, 0u32, 1u32])
            .u8();
        e.call(SET_USED_ITEM, &args![0u32]);
        e.call(SET_USED_ITEM_ACTIVATE, &args![0u32]);
        e.call(SET_USED_ITEM_LEVEL, &args![0xffff_ffffu32]);
        if accepted == 0 && e.mem.u32(at + 0x80) != 0 {
            return false;
        }
        if activator == e.mem.u32(PLAYER) && accepted != 0 {
            let script = e.mem.u32(at + 0x80);
            e.mem.set_u32(PLAYER_ACTIVATED_WORD, script);
            let player = e.mem.u32(PLAYER);
            e.call(PLAYER_004FCBD0, &args![player, 1u32]);
            player_activated = true;
            for first_person in [1u32, 0] {
                let player = e.mem.u32(PLAYER);
                let animation = e
                    .call(PLAYER_GET_ANIMATION, &args![player, first_person])
                    .u32();
                e.call(ANIMATION_CLEAR_GROUP, &args![animation, 1u32, 0.0f32]);
            }
        }
        if !e.call(IS_AN_OWNER, &args![reference, actor, 1u32]).bool() {
            let owner = e.call(GET_OWNER, &args![reference]).u32();
            e.call(
                ACTOR_008C0EC0,
                &args![actor, reference, owner, 0xffff_ffffu32],
            );
        }
    }
    if e.call(FORM_SOUND, &args![at]).u32() != 0 {
        let mut node = 0u32;
        if reference != 0 && e.vcall(reference, 0x1d0, &[]).u32() != 0 {
            let root = e.vcall(reference, 0x1d0, &[]).u32();
            let name = e.call(SOUND_NODE_NAME, &[]).u32();
            node = e.vcall(root, 0x9c, &args![name]).u32();
        }
        let position = if node != 0 {
            e.call(NODE_WORLD_POSITION, &args![node]).u32()
        } else {
            e.vcall(reference, 0x1f4, &[]).u32()
        };
        let point = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        let mut extra_flags = 0u32;
        if player_activated {
            extra_flags = e.with_stack(0x24, |e, block| {
                let sound = e.call(FORM_SOUND, &args![at]).u32();
                let info = e.call(SOUND_004E75D0, &args![sound, block]).u32();
                let word = e.mem.u32(info + 4);
                e.call(SOUND_FLAGS_005E39B0, &args![word]).u32()
            });
        }
        let sound = e.call(FORM_SOUND, &args![at]).u32();
        let flags = extra_flags | 0x4000_0102;
        let named = e.call(FORM_SOUND, &args![at]).u32();
        let name = fn_00511840(e, Ptr::new(named));
        e.with_stack(0xc, |e, handle| {
            let audio = e.call(AUDIO_INSTANCE, &[]).u32();
            e.call(
                GET_SOUND_HANDLE_BY_FILENAME,
                &args![audio, handle, name, flags, sound],
            );
            if player_activated {
                let player = e.mem.u32(PLAYER);
                let list = e.call(PLAYER_005D43C0, &args![player]).u32();
                e.call(EXTRA_0041A6A0, &args![list, handle]);
            }
            let [x, y, z] = point.map(x87_quiet_bits);
            e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
            let root = e.vcall(reference, 0x1d0, &[]).u32();
            e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![handle, root]);
            e.call(SOUND_PLAY, &args![handle, 0u32]);
            e.call(EMPTY_00483710, &args![handle]);
        });
    }
    let script = e.mem.u32(at + 0x80);
    if script != 0 {
        e.vcall(
            script,
            0x124,
            &args![reference, activator, u32::from(flag), word_4, word_5],
        );
    }
    e.call(
        OPEN_CLOSE_HANDLE_ACTIVATE,
        &args![reference, activator, 0u32],
    );
    if e.call(FORM_004FD3C0, &args![at]).u32() != 0 {
        let state = e.call(OPEN_CLOSE_GET_OPEN_STATE, &args![reference]).u32();
        let opening = u32::from(state == 1 || state == 2);
        e.call(RADIO_008325B0, &args![reference, opening]);
    }
    true
}

// Translated from 00511840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): `MapMarkerData::GetLocationName`
/// (by the engine map's name for `00408da0`) on the member at +0x30.
pub fn fn_00511840(e: &mut Engine, this: Ptr) -> u32 {
    e.call(GET_LOCATION_NAME, &args![this.addr() + 0x30]).u32()
}

// Translation incomplete: 00511a10 - the FS:[0] exception frame (running the remaining base destructors when one throws) is not translated
// Translated from 00511a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor body `005119e0` runs (the
/// class of the form type 0x4D constructor `00511950`): stores the two
/// vtables (`0102a0a4` at +0, `0102a07c` at +0x18), then `00483710` on
/// `this`, `0048a490` on +0x18 and `00483630` on `this`.
pub fn fn_00511a10(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, VTABLE_0102A0A4);
    e.mem.set_u32(at + 0x18, VTABLE_0102A07C);
    e.call(EMPTY_00483710, &args![at]);
    e.call(DESTRUCT_0048A490, &args![at + 0x18]);
    e.call(DESTRUCT_00483630, &args![at]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the type
/// descriptors it is given: `.?AVTESForm@@` (`01183028`) to
/// `.?AVTESObjectACTI@@` (`01186584`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_ACTI: u32 = 0x0118_6584;
/// `004fd3a0` (`thiscall`, no stack words) and `00407840` (`thiscall`, no
/// stack words): two more getters both `00511120` and `005111e0` read.
const FORM_004FD3A0: u32 = 0x004f_d3a0;
const FORM_00407840: u32 = 0x0040_7840;
/// `TESForm::CopyAllComponents` (`004851b0`) and
/// `TESForm::CompareAllComponents` (`00485270`) (engine map; `thiscall`,
/// one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// The `BSStringT` destructor (`004037d0`, `thiscall`).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `0044ddc0` (`thiscall`): the form's flag word; `00403550` stores it
/// (`thiscall`, one word); `00484b60` (`thiscall`, one word).
const FORM_FLAGS: u32 = 0x0044_ddc0;
const SET_FORM_FLAGS: u32 = 0x0040_3550;
const FORM_00484B60: u32 = 0x0048_4b60;
/// `00428110` (`thiscall` on the save or load buffer, one word: an out
/// word), then `004280f0` (`thiscall` on its result, one word).
const BUFFER_00428110: u32 = 0x0042_8110;
const CHECK_004280F0: u32 = 0x0042_80f0;
/// `BGSSaveGameBuffer::SaveString` (`00865e70`, two words),
/// `BGSLoadGameBuffer::LoadString` (`008649a0`, one word) and
/// `TESSoundFile::SetSoundFile` (`00489100`, one word) (engine map).
const SAVE_STRING: u32 = 0x0086_5e70;
const LOAD_STRING: u32 = 0x0086_49a0;
const SET_SOUND_FILE: u32 = 0x0048_9100;

/// Casts `form` to `TESObjectACTI` (`__RTDynamicCast(form, 0, TESForm,
/// TESObjectACTI, 0)`).
fn cast_to_activator(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_ACTI, 0u32],
    )
    .u32()
}

// Translated from 00511120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; a form's
/// copy from `source`): when `source` casts to `TESObjectACTI`, copies its
/// parts into `this`: `004febb0(004fd3a0)`, `004febd0(00407840)`,
/// `00442a80(00407820)`, `008ac890(slot +0x140)`, `004fb090(004fd3c0)`, and
/// the string `005110e0` copies (a `BSStringT` built in the 8 bytes the
/// call passes by value) to `00510ea0`; then
/// `TESForm::CopyAllComponents(source)`. Nothing happens otherwise.
pub fn fn_00511120(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_activator(e, source);
    if other == 0 {
        return;
    }
    let value = e.call(FORM_004FD3A0, &args![other]).u32();
    e.call(0x004f_ebb0, &args![this, value]);
    let value = e.call(FORM_00407840, &args![other]).u32();
    e.call(0x004f_ebd0, &args![this, value]);
    let value = e.call(FORM_SOUND, &args![other]).u32();
    e.call(0x0044_2a80, &args![this, value]);
    let value = e.vcall(other, 0x140, &args![]).u32();
    e.call(0x008a_c890, &args![this, value]);
    let value = e.call(FORM_004FD3C0, &args![other]).u32();
    e.call(0x004f_b090, &args![this, value]);
    e.with_stack(8, |e, string| {
        let string = string.addr();
        fn_005110e0(e, Ptr::new(other), Ptr::new(string));
        let first = e.mem.u32(string);
        let second = e.mem.u32(string + 4);
        e.call(0x0051_0ea0, &args![this, first, second]);
    });
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
}

// Translated from 005111e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005111e0 - the FS:[0] exception frame (destroying the two string copies when a call throws) is not translated
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether
/// `this` differs from `other` (true) or matches it (false). A form that
/// does not cast to `TESObjectACTI` differs; so does any of `004fd3a0`,
/// `00407820`, `00407840`, slot +0x140 and `004fd3c0` answering differently
/// for the two (compared in that order); then the two strings `005110e0`
/// copies are compared by `00501c90` (on this form's copy, with the other's
/// and 1; both copies destroyed after), and a non-zero answer differs;
/// last, `TESForm::CompareAllComponents(other)` decides.
pub fn fn_005111e0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_activator(e, form);
    if other == 0 {
        return true;
    }
    for getter in [FORM_004FD3A0, FORM_SOUND, FORM_00407840] {
        let mine = e.call(getter, &args![this]).u32();
        if mine != e.call(getter, &args![other]).u32() {
            return true;
        }
    }
    let mine = e.vcall(this.addr(), 0x140, &args![]).u32();
    if mine != e.vcall(other, 0x140, &args![]).u32() {
        return true;
    }
    let mine = e.call(FORM_004FD3C0, &args![this]).u32();
    if mine != e.call(FORM_004FD3C0, &args![other]).u32() {
        return true;
    }
    let compared = e.with_stack(0x10, |e, strings| {
        let strings = strings.addr();
        let theirs = fn_005110e0(e, Ptr::new(other), Ptr::new(strings));
        let mine = fn_005110e0(e, this, Ptr::new(strings + 8));
        let compared = e.call(0x0050_1c90, &args![mine, theirs, 1u32]).u32();
        e.call(STRING_DESTRUCT, &args![strings + 8]);
        e.call(STRING_DESTRUCT, &args![strings]);
        compared
    });
    if compared != 0 {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool()
}

// Translated from 00511370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`): sets bit
/// 0x40000000 of the form's flag word (`0044ddc0`) when `set` is non-zero,
/// clears it otherwise, stores the word (`00403550`) and runs
/// `00484b60(1)`.
pub fn fn_00511370(e: &mut Engine, this: Ptr, set: u8) {
    let mut flags = e.call(FORM_FLAGS, &args![this]).u32();
    if set != 0 {
        flags |= 0x4000_0000;
    } else {
        flags &= 0xbfff_ffff;
    }
    e.call(SET_FORM_FLAGS, &args![this, flags]);
    e.call(FORM_00484B60, &args![this, 1u32]);
}

// Translated from 005113d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 0x10000000 of the form's flag
/// word (`0044ddc0`) is set (0 or 1 in `EAX`).
pub fn fn_005113d0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS, &args![this]).u32() & 0x1000_0000 != 0
}

// Translated from 00511860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; a save):
/// `00484d60(buffer)`, then when `004280f0(4)` on what `00428110` (given an
/// out word on the stack) answers for the buffer is true, saves the
/// location name of the member at +0x30 (`00408da0`) with
/// `BGSSaveGameBuffer::SaveString(name, 0)`.
pub fn fn_00511860(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4d60, &args![this, buffer]);
    e.with_stack(4, |e, word| {
        let checker = e.call(BUFFER_00428110, &args![buffer, word]).u32();
        if e.call(CHECK_004280F0, &args![checker, 4u32]).bool() {
            let name = e.call(GET_LOCATION_NAME, &args![this.addr() + 0x30]).u32();
            e.call(SAVE_STRING, &args![buffer, name, 0u32]);
        }
    });
}

// Translated from 005118c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005118c0 - the /GS security cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; a load):
/// `00484da0(buffer)`, then when `004280f0(4)` on what `00428110` (given an
/// out word on the stack) answers for the buffer is true, loads a string
/// into a 0x10C-byte stack buffer (`BGSLoadGameBuffer::LoadString`) and
/// gives it to `TESSoundFile::SetSoundFile` on the member at +0x30.
pub fn fn_005118c0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4da0, &args![this, buffer]);
    e.with_stack(0x110, |e, block| {
        let block = block.addr();
        let checker = e.call(BUFFER_00428110, &args![buffer, block]).u32();
        if e.call(CHECK_004280F0, &args![checker, 4u32]).bool() {
            let name = block + 4;
            e.call(LOAD_STRING, &args![buffer, name]);
            e.call(SET_SOUND_FILE, &args![this.addr() + 0x30, name]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_1120, fn_00511120(Ptr, u32)),
        entry!(0x0051_11e0, fn_005111e0(Ptr, u32) -> bool),
        entry!(0x0051_1370, fn_00511370(Ptr, u8)),
        entry!(0x0051_13d0, fn_005113d0(Ptr) -> bool),
        entry!(0x0051_1860, fn_00511860(Ptr, u32)),
        entry!(0x0051_18c0, fn_005118c0(Ptr, u32)),
        entry!(0x0051_1950, fn_00511950(Ptr) -> Ptr),
        entry!(0x0051_1940, fn_00511940(Ptr, u32) -> u32),
        entry!(0x0051_19e0, fn_005119e0(Ptr, u32) -> Ptr),
        entry!(0x005110e0, fn_005110e0(Ptr, Ptr) -> Ptr),
        entry!(
            0x005113f0,
            fn_005113f0(Ptr, u32, u32, u8, u32, u32) -> bool
        ),
        entry!(0x00511840, fn_00511840(Ptr) -> u32),
        entry!(0x00511a10, fn_00511a10(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00511950_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_1950,
            size: 0x3c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4d)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_a0a4),
                (0x18, 4, 0x0102_a07c),
                (0x38, 4, 0x0),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00511940() {
        assert_eq!(
            check(0x0051_1940, 0x0051_05f0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_005119e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_1a10, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_19e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_19e0)
                .collect();
            let mut expected = vec![(0x0051_1a10, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod activation_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn string_and_location_name_helpers() {
        let mut e = Engine::new();
        e.register(STRING_COPY, |_, a| a[0].into_ret());
        e.register(GET_LOCATION_NAME, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0051_10e0, &args![0x1000u32, 0x2000u32]).u32(),
            0x2000
        );
        assert_eq!(calls_to(&e, STRING_COPY), vec![vec![0x2000, 0x1088]]);
        assert_eq!(e.call(0x0051_1840, &args![0x1000u32]).u32(), 0x1031);
    }

    #[test]
    fn destructor_body_stores_the_vtables() {
        let mut e = Engine::new();
        for address in [EMPTY_00483710, DESTRUCT_0048A490, DESTRUCT_00483630] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x3c);
        e.call_log = Some(vec![]);
        e.call(0x0051_1a10, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_0102A0A4);
        assert_eq!(e.mem.u32(this + 0x18), VTABLE_0102A07C);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (EMPTY_00483710, vec![this]),
                (DESTRUCT_0048A490, vec![this + 0x18]),
                (DESTRUCT_00483630, vec![this]),
            ]
        );
    }

    /// An object whose vtable slots up to +0x400 are recording doubles at
    /// `base + slot`, answering the value `answers` gives the slot (0
    /// otherwise).
    fn object(e: &mut Engine, base: u32, answers: Vec<(u32, u32)>) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for slot in (0..0x400u32).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            let value = answers
                .iter()
                .find(|(s, _)| *s == slot)
                .map_or(0, |(_, v)| *v);
            e.register_double(base + slot, move |_, _| value.into_ret());
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            SET_USED_ITEM,
            SET_USED_ITEM_LEVEL,
            SET_USED_ITEM_ACTIVATE,
            PLAYER_004FCBD0,
            ANIMATION_CLEAR_GROUP,
            ACTOR_008C0EC0,
            GET_SOUND_HANDLE_BY_FILENAME,
            SOUND_SET_POSITION,
            SOUND_SET_OBJECT_TO_FOLLOW,
            SOUND_PLAY,
            EMPTY_00483710,
            EXTRA_0041A6A0,
            OPEN_CLOSE_HANDLE_ACTIVATE,
            RADIO_008325B0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ACTOR_GET_EYE_LEVEL, |_, _| 60.0f64.into_ret());
        e.register(GET_OWNER, |_, _| 0x0077u32.into_ret());
        e.register(IS_AN_OWNER, |_, _| Ret::default());
        e.register(PLAYER_GET_ANIMATION, |_, a| (0x0a00 + a[1]).into_ret());
        e.register(OPEN_CLOSE_GET_OPEN_STATE, |_, _| 2u32.into_ret());
        e.register(FORM_004FD3C0, |_, _| 1u32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0x0a0du32.into_ret());
        e.register(GET_LOCATION_NAME, |_, _| 0x4e4du32.into_ret());
        e.map(0x011d_e000, 0x3000);
        e
    }

    #[test]
    fn actor_activation_without_a_sound() {
        let mut e = engine();
        e.register(FORM_SOUND, |_, _| Ret::default());
        // The actor at z 100 with a height offset of 10: eye height 150.
        let position = e.mem.alloc(12);
        e.mem.set_f32(position + 8, 100.0);
        let actor = object(&mut e, 0x0030_0000, vec![(0x100, 1), (0x1f4, position)]);
        e.register(0x0030_0380, |_, _| 10.0f64.into_ret());
        // The reference at z 200: the eye is below it, level 1.
        let reference_position = e.mem.alloc(12);
        e.mem.set_f32(reference_position + 8, 200.0);
        let reference = object(&mut e, 0x0031_0000, vec![(0x1f4, reference_position)]);
        let acquire = object(&mut e, 0x0032_0000, vec![(0x44, 1)]);
        e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |_, _| acquire.into_ret());
        let script = object(&mut e, 0x0033_0000, vec![]);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x80, script);
        e.call_log = Some(vec![]);
        let done = e
            .call(
                0x0051_13f0,
                &args![this, reference, actor, 0x101u32, 4u32, 5u32],
            )
            .bool();
        assert!(done);
        assert_eq!(
            calls_to(&e, SET_USED_ITEM_LEVEL),
            vec![vec![1], vec![0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, 0x0032_0044),
            vec![vec![acquire, actor, 0, 3, 1, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_008C0EC0),
            vec![vec![actor, reference, 0x77, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, 0x0033_0124),
            vec![vec![script, reference, actor, 1, 4, 5]]
        );
        assert_eq!(
            calls_to(&e, OPEN_CLOSE_HANDLE_ACTIVATE),
            vec![vec![reference, actor, 0]]
        );
        assert_eq!(calls_to(&e, RADIO_008325B0), vec![vec![reference, 1]]);
        assert!(calls_to(&e, PLAYER_004FCBD0).is_empty());
        assert!(calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME).is_empty());
    }

    #[test]
    fn refused_activation_with_a_script_answers_false() {
        let mut e = engine();
        e.register(FORM_SOUND, |_, _| Ret::default());
        let position = e.mem.alloc(12);
        let actor = object(&mut e, 0x0030_0000, vec![(0x100, 1), (0x1f4, position)]);
        let reference = object(&mut e, 0x0031_0000, vec![(0x1f4, position), (0x218, 1)]);
        let acquire = object(&mut e, 0x0032_0000, vec![]);
        e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |_, _| acquire.into_ret());
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x80, 0x1234);
        e.call_log = Some(vec![]);
        let done = e
            .call(
                0x0051_13f0,
                &args![this, reference, actor, 0u32, 0u32, 0u32],
            )
            .bool();
        assert!(!done);
        assert_eq!(
            calls_to(&e, SET_USED_ITEM_LEVEL),
            vec![vec![1], vec![0xffff_ffff]]
        );
        assert!(calls_to(&e, OPEN_CLOSE_HANDLE_ACTIVATE).is_empty());
    }

    #[test]
    fn player_activation_plays_the_sound_at_the_reference() {
        let mut e = engine();
        let sound = e.mem.alloc(0x40);
        e.register_double(FORM_SOUND, move |_, _| sound.into_ret());
        e.register(SOUND_004E75D0, |e, a| {
            e.mem.set_u32(a[1] + 4, 0x200);
            a[1].into_ret()
        });
        e.register(SOUND_FLAGS_005E39B0, |_, a| (a[0] << 4).into_ret());
        e.register(PLAYER_005D43C0, |_, _| 0x5d5du32.into_ret());
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let player = object(&mut e, 0x0030_0000, vec![(0x100, 1), (0x1f4, position)]);
        e.set_global(PLAYER, player);
        let reference = object(&mut e, 0x0031_0000, vec![(0x1f4, position)]);
        let acquire = object(&mut e, 0x0032_0000, vec![(0x44, 1)]);
        e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |_, _| acquire.into_ret());
        e.register(IS_AN_OWNER, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x0051_13f0,
                &args![this, reference, player, 0u32, 0u32, 0u32]
            )
            .bool());
        assert_eq!(e.mem.u32(PLAYER_ACTIVATED_WORD), 0);
        assert_eq!(calls_to(&e, PLAYER_004FCBD0), vec![vec![player, 1]]);
        assert_eq!(
            calls_to(&e, ANIMATION_CLEAR_GROUP),
            vec![vec![0x0a01, 1, 0], vec![0x0a00, 1, 0]]
        );
        assert!(calls_to(&e, ACTOR_008C0EC0).is_empty());
        let handle = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(handle[0][0], 0x0a0d);
        assert_eq!(&handle[0][2..], [0x4e4d, 0x4000_2102, sound]);
        let handle = handle[0][1];
        assert_eq!(calls_to(&e, EXTRA_0041A6A0), vec![vec![0x5d5d, handle]]);
        assert_eq!(
            calls_to(&e, SOUND_SET_POSITION),
            vec![vec![
                handle,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&e, SOUND_PLAY), vec![vec![handle, 0]]);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    /// A form whose getters answer the word at +4 and whose slot +0x140
    /// answers the word at +8.
    fn form(e: &mut Engine, vtable: u32, key: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u32(form, vtable);
        e.mem.set_u32(form + 4, key);
        e.mem.set_u32(form + 8, 0x140);
        form
    }

    fn setup(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x140, 0x0090_0140);
        e.register(0x0090_0140, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        for getter in [FORM_004FD3A0, FORM_SOUND, FORM_00407840, FORM_004FD3C0] {
            e.register(getter, |e, a| Ret {
                eax: e.mem.u32(a[0] + 4),
                ..Ret::default()
            });
        }
        // The cast answers the form when its word at +0xc is zero.
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_ACTI, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(STRING_COPY, |e, a| {
            e.mem.set_u32(a[0], e.mem.u32(a[1]));
            e.mem.set_u32(a[0] + 4, 0x55);
            Ret::default()
        });
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        vtable
    }

    #[test]
    fn fn_00511120_copies_from_an_activator_only() {
        let mut e = Engine::new();
        let vtable = setup(&mut e);
        let this = form(&mut e, vtable, 1);
        let source = form(&mut e, vtable, 7);
        e.mem.set_u32(source + 0x88, 0x88);
        let setters = [
            0x004f_ebb0,
            0x004f_ebd0,
            0x0044_2a80,
            0x008a_c890,
            0x004f_b090,
            0x0051_0ea0,
            COPY_ALL_COMPONENTS,
        ];
        for s in setters {
            e.register(s, |_, _| Ret::default());
        }
        e.mem.set_u32(source + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_1120, &[this, source]);
        assert!(calls_to(&mut e, &setters).is_empty());
        e.mem.set_u32(source + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_1120, &[this, source]);
        assert_eq!(
            calls_to(&mut e, &setters),
            vec![
                (0x004f_ebb0, vec![this, 7]),
                (0x004f_ebd0, vec![this, 7]),
                (0x0044_2a80, vec![this, 7]),
                (0x008a_c890, vec![this, 0x140]),
                (0x004f_b090, vec![this, 7]),
                (0x0051_0ea0, vec![this, 0x88, 0x55]),
                (COPY_ALL_COMPONENTS, vec![this, source]),
            ]
        );
    }

    #[test]
    fn fn_005111e0_compares_parts_then_strings_then_components() {
        let mut e = Engine::new();
        let vtable = setup(&mut e);
        e.register(0x0050_1c90, |e, a| {
            assert_eq!(a[2], 1);
            Ret {
                eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
                ..Ret::default()
            }
        });
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        let this = form(&mut e, vtable, 3);
        let other = form(&mut e, vtable, 3);
        // Not an activator: differs.
        e.mem.set_u32(other + 0xc, 1);
        assert!(e.call(0x0051_11e0, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        // A getter differs.
        e.mem.set_u32(other + 4, 4);
        assert!(e.call(0x0051_11e0, &[this, other]).bool());
        e.mem.set_u32(other + 4, 3);
        // Slot +0x140 differs.
        e.mem.set_u32(other + 8, 9);
        assert!(e.call(0x0051_11e0, &[this, other]).bool());
        e.mem.set_u32(other + 8, 0x140);
        // The strings differ.
        e.mem.set_u32(other + 0x88, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_11e0, &[this, other]).bool());
        let log = calls_to(&mut e, &[STRING_DESTRUCT, COMPARE_ALL_COMPONENTS]);
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].1[0], log[1].1[0] + 8);
        e.mem.set_u32(other + 0x88, 0);
        // Components decide.
        assert!(!e.call(0x0051_11e0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_11e0, &[this, other]).bool());
    }

    #[test]
    fn fn_00511370_sets_or_clears_the_flag() {
        let mut e = Engine::new();
        e.register(FORM_FLAGS, |_, _| Ret {
            eax: 0x8000_0001,
            ..Ret::default()
        });
        e.register(SET_FORM_FLAGS, |_, _| Ret::default());
        e.register(FORM_00484B60, |_, _| Ret::default());
        for (set, flags) in [(0x01u32, 0xc000_0001u32), (0x100, 0x8000_0001)] {
            e.call_log = Some(vec![]);
            e.call(0x0051_1370, &[0x1000, set]);
            assert_eq!(
                calls_to(&mut e, &[SET_FORM_FLAGS, FORM_00484B60]),
                vec![
                    (SET_FORM_FLAGS, vec![0x1000, flags]),
                    (FORM_00484B60, vec![0x1000, 1])
                ]
            );
        }
    }

    #[test]
    fn fn_005113d0_tests_bit_28() {
        let mut e = Engine::new();
        e.register(FORM_FLAGS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert!(e.call(0x0051_13d0, &[0x1000_0000]).bool());
        assert!(!e.call(0x0051_13d0, &[0xefff_ffff]).bool());
    }

    #[test]
    fn fn_00511860_and_005118c0_save_and_load_the_name() {
        let mut e = Engine::new();
        e.register(0x0048_4d60, |_, _| Ret::default());
        e.register(0x0048_4da0, |_, _| Ret::default());
        e.register(BUFFER_00428110, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CHECK_004280F0, |_, a| {
            assert_eq!(a[1], 4);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(GET_LOCATION_NAME, |_, _| Ret {
            eax: 0x0077_0000,
            ..Ret::default()
        });
        e.register(SAVE_STRING, |_, _| Ret::default());
        e.register(LOAD_STRING, |_, _| Ret::default());
        e.register(SET_SOUND_FILE, |_, _| Ret::default());
        let buffer = e.mem.alloc(4);
        let targets = [
            0x0048_4d60,
            0x0048_4da0,
            BUFFER_00428110,
            SAVE_STRING,
            LOAD_STRING,
            SET_SOUND_FILE,
        ];
        for flag in [0u32, 1] {
            e.mem.set_u32(buffer, flag);
            e.call_log = Some(vec![]);
            e.call(0x0051_1860, &[0x1000, buffer]);
            let log = calls_to(&mut e, &targets);
            assert_eq!(log[0], (0x0048_4d60, vec![0x1000, buffer]));
            if flag == 0 {
                assert_eq!(log.len(), 2);
            } else {
                assert_eq!(log[2], (SAVE_STRING, vec![buffer, 0x0077_0000, 0]));
            }
            e.call_log = Some(vec![]);
            e.call(0x0051_18c0, &[0x1000, buffer]);
            let log = calls_to(&mut e, &targets);
            assert_eq!(log[0], (0x0048_4da0, vec![0x1000, buffer]));
            if flag == 0 {
                assert_eq!(log.len(), 2);
            } else {
                let block = log[1].1[1];
                assert_eq!(log[2], (LOAD_STRING, vec![buffer, block + 4]));
                assert_eq!(log[3], (SET_SOUND_FILE, vec![0x1030, block + 4]));
            }
        }
    }
}
