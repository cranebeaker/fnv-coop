//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 4: its functions from `005c8450` up to
//! (not including) `005cd990` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: the 120 queue entries of this part (`005c8450` to `005cd910`,
//! the last function of the range) are translated.
//!
//! Notes on the exe's code that the translations rely on:
//! - Script command bodies are `cdecl` and take the eight words of
//!   [`ScriptArgs`]; the parameters are read through
//!   `Script::ParseParameters` into locals passed by address
//!   ([`parse_into`]). The condition-function twins
//!   (`Script::Get...ConditionFunction`, `tesconditionfunctions.cpp`) are
//!   called by address.
//! - `005ca1c0` is the shared body of the "script event flag" commands: it
//!   asks the script event list (the sixth word) whether an event is set.
//! - The HDR commands work on the image space manager's HDR effect
//!   (`ImageSpaceEffectHDR`, Xbox PDB), whose `pfData` array (+0x6c) holds
//!   the 16 `HDRDataType` floats (`HDR_EYE_ADAPT_SPEED = 0` ...
//!   `HDR_TREE_DIMMER = 13`).
//! - The compiler's exception-unwinding frame (`__CxxFrameHandler` states,
//!   the `FS:[0]` chain) of `005c9cf0` is not translated.

#[allow(unused_imports)]
use super::tesscriptfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---- Callees outside the unit (by exe address) -----------------------------

/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs.
const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// The console print: format address first, then the arguments (`cdecl`);
/// a `double` argument takes two words.
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// `__RTDynamicCast` (`object, 0, source type, target type, 0`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor of `TESObjectREFR` (`.?AVTESObjectREFR@@`).
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// RTTI type descriptor of `Actor` (`.?AVActor@@`).
const RTTI_ACTOR: u32 = 0x0118_46d4;

/// `Script::GetDispositionConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, actor, 0, result`).
const GET_DISPOSITION_CONDITION: u32 = 0x0059_fdb0;
/// `Script::GetRandomPercentConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, 0, 0, result`).
const GET_RANDOM_PERCENT_CONDITION: u32 = 0x005a_0040;
/// `Script::GetLevelConditionFunction` (Xbox PDB), same arguments.
const GET_LEVEL_CONDITION: u32 = 0x005a_00c0;
/// `Script::GetArmorRatingConditionFunction` (Xbox PDB), same arguments.
const GET_ARMOR_RATING_CONDITION: u32 = 0x005a_0150;
/// `Script::GetDeadCountConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, form, 0, result`).
const GET_DEAD_COUNT_CONDITION: u32 = 0x005a_01f0;
/// `Script::GetAlertConditionFunction` (Xbox PDB), same arguments as
/// [`GET_RANDOM_PERCENT_CONDITION`].
const GET_ALERT_CONDITION: u32 = 0x005a_0260;
/// Condition function of `tesconditionfunctions.cpp` (`thisObj, 0, 0,
/// result`) that stores a whole number as a `double`.
const FN_005A4340: u32 = 0x005a_4340;

/// `Actor::SetAlert` (Xbox PDB), `thiscall` (`bool`).
const ACTOR_SET_ALERT: u32 = 0x008a_5e40;
/// `Actor::SetLookAtTarget` (Xbox PDB), `thiscall` (the three words of a
/// point passed by value).
const ACTOR_SET_LOOK_AT_TARGET: u32 = 0x008b_3cd0;
/// `Actor::ClearLookAtTarget` (Xbox PDB), `thiscall`.
const ACTOR_CLEAR_LOOK_AT_TARGET: u32 = 0x008b_3d30;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB), `thiscall` on the
/// actor (the name is the engine map's; it returns the actor's process).
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `this + 0x44`: the extra data list of a reference.
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::SetItemDropper` (Xbox PDB), `thiscall` (`reference`).
const SET_ITEM_DROPPER: u32 = 0x0041_9dc0;
/// `TESObjectREFR::SetTargeted` (Xbox PDB), `thiscall` (`bool`).
const SET_TARGETED: u32 = 0x0056_4db0;
/// `TESObjectREFR::GetMapMarkerData` (Xbox PDB), `thiscall`.
const GET_MAP_MARKER_DATA: u32 = 0x0056_9060;
/// `MapMarkerData::GetVisible` (Xbox PDB), `thiscall`.
const MAP_MARKER_GET_VISIBLE: u32 = 0x0043_8ed0;
/// `MapMarkerData::GetTravelLoc` (Xbox PDB), `thiscall`.
const MAP_MARKER_GET_TRAVEL_LOC: u32 = 0x0043_8ef0;
/// `MapMarkerData::SetVisible` (Xbox PDB), `thiscall` (`bool`).
const MAP_MARKER_SET_VISIBLE: u32 = 0x0044_de40;
/// `MapMarkerData::SetTravelLoc` (Xbox PDB), `thiscall` (`bool`).
const MAP_MARKER_SET_TRAVEL_LOC: u32 = 0x0044_de80;
/// `*(this + 4)` of a string global: its text (`thiscall`).
const BS_STRING_TEXT: u32 = 0x0040_3df0;
/// Interface message with icon (`cdecl`: `text, 0, icon path, sound, float,
/// 0`).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `thiscall` on a reference (`action`): whether the action is set.
const HAS_ACTION: u32 = 0x0057_2d30;
/// `TESObjectREFR::ClearAction` (Xbox PDB), `thiscall` (`action`).
const CLEAR_ACTION: u32 = 0x0057_2db0;
/// `NiPointer::operator T*`: `*this`.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `thiscall` on the object `005c9740` parsed (`1`); it walks a list at
/// `+8` of the object and updates each entry (unnamed, no unit).
const FN_0060C950: u32 = 0x0060_c950;
/// `thiscall` on a script event list (`event, mask`): whether the event is
/// in the list with one of the mask's bits set (`tesscript.cpp`).
const SCRIPT_EVENT_LIST_HAS_EVENT: u32 = 0x005a_8ef0;
/// `fabs` of a `float` argument, result in `ST0` (`effectsetting.cpp`).
const FLOAT_ABS: u32 = 0x0040_8840;

/// The image space manager singleton getter (loads the global `011f91ac`).
const GET_IMAGE_SPACE_MANAGER: u32 = 0x004e_3270;
/// `thiscall` on the image space manager (`index`): the effect with that
/// index; the HDR commands ask for index 1, the HDR effect.
const IMAGE_SPACE_GET_EFFECT: u32 = 0x004e_bbc0;

/// `operator new` (`size`), `cdecl`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `Ni` allocator (`size`), `cdecl`.
const NI_ALLOC: u32 = 0x00aa_13e0;
/// `_vector_constructor_iterator_` (`array, element size, count,
/// constructor`), `stdcall`; the constructor is called on each element.
const VECTOR_CONSTRUCT_ITERATOR: u32 = 0x0040_1050;
/// The default `NiPoint3` constructor passed to the iterator (a function
/// returning `this`).
const NI_POINT3_DEFAULT_CONSTRUCT: u32 = 0x0068_15c0;
/// The default `NiColorA` constructor passed to the iterator.
const NI_COLOR_A_DEFAULT_CONSTRUCT: u32 = 0x004a_7800;
/// `NiColorA::NiColorA(r, g, b, a)`, `thiscall`: returns `this`.
const NI_COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// `NiPoint3::NiPoint3(x, y, z)`, `thiscall`: returns `this`.
const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `NiLines::NiLines` (Xbox PDB), `thiscall` (`vertex count, vertices,
/// colors, texture coordinates, texture sets, NBT method, connections`):
/// returns `this`.
const NI_LINES_CONSTRUCT: u32 = 0x00a7_46e0;
/// `NiAVObject::GetWorldBound` (Xbox PDB), `thiscall`: the bound (a default
/// one when the object has none).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// The radius (`float` at `+0xc`) of a bound, in `ST0`.
const BOUND_RADIUS: u32 = 0x0084_d030;
/// Copies an `NiPoint3` into `this + 0x58` (the local translation of an
/// `NiAVObject`; `thiscall`, `RET 4`).
const SET_TRANSLATE: u32 = 0x0044_0460;
/// `TES::AddTempDebugObject` (Xbox PDB), `thiscall` on the `TES` singleton
/// (`object, seconds`).
const ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;

// ---- Globals ---------------------------------------------------------------

/// Pointer to the `TES` singleton.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// Global `float`: the sunlight dimmer HDR value (`fSunlightDimmer`).
const HDR_SUNLIGHT_DIMMER: u32 = 0x011f_9190;
/// Global `float`: the luminance ramp (`Lum Ramp`).
const HDR_LUM_RAMP: u32 = 0x011a_d87c;
/// Global `float`: `fGrassDimmer`.
const HDR_GRASS_DIMMER: u32 = 0x011f_9188;
/// Global `float`: `fTreeDimmer`.
const HDR_TREE_DIMMER: u32 = 0x011f_918c;
/// `float` the message commands pass as the sound volume.
const MESSAGE_VOLUME: u32 = 0x0101_62c0;
/// `10.0f`: the default half length of the debug axes.
const DEFAULT_AXIS_LENGTH: u32 = 0x0101_7b78;
/// `20.0f`: seconds a debug object stays.
const DEBUG_OBJECT_SECONDS: u32 = 0x0101_7868;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// The string global whose text the map marker message shows.
const MAP_MARKER_MESSAGE: u32 = 0x011d_4768;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"Interface\Icons\Message Icons\glow_message_map.dds"`
const ICON_MAP: u32 = 0x0103_b30c;
/// `"UIPopUpMapMarkerAdded"`
const SOUND_MAP_MARKER_ADDED: u32 = 0x0103_b340;
/// `"Current HDR Params:"`
const MSG_HDR_PARAMS: u32 = 0x0103_b464;
/// `"SISG:"`
const MSG_SISG: u32 = 0x0103_b45c;
/// `"    iNumBlurpasses: %d fBlurRadius: %f"`
const FORMAT_BLUR: u32 = 0x0103_b434;
/// `"    fBrightClamp: %f fBrightScale: %f"`
const FORMAT_BRIGHT: u32 = 0x0103_b40c;
/// `"SSP:"`
const MSG_SSP: u32 = 0x0103_b404;
/// `"    fSunlightDimmer: %f Lum Ramp: %f"`
const FORMAT_SUNLIGHT: u32 = 0x0103_b3dc;
/// `"SHP:"`
const MSG_SHP: u32 = 0x0103_b3d4;
/// `"    fEyeAdaptSpeed: %f fEmissiveHDRMult: %f"`
const FORMAT_EYE_ADAPT: u32 = 0x0103_b3a8;
/// `"    fTreeDimmer: %f fGrassDimmer: %f"`
const FORMAT_DIMMERS: u32 = 0x0103_b380;
/// `"    fUpperLUMClamp: %f fTargetLUM: %f"`
const FORMAT_LUM: u32 = 0x0103_b358;

// ---- Small helpers -------------------------------------------------------------

/// `Script::ParseParameters` with `N` word-sized locals (the stack slots the
/// game passes by address) initialised to `init`: `None` when the
/// parameters do not parse, otherwise the values left in the locals.
fn parse_into<const N: usize>(e: &mut Engine, a: ScriptArgs, init: [u32; N]) -> Option<[u32; N]> {
    let (ok, values) = parse_into_unchecked(e, a, init);
    ok.then_some(values)
}

/// [`parse_into`] for the commands that ignore the parser's answer: whether
/// it parsed, and the values left in the locals either way.
fn parse_into_unchecked<const N: usize>(
    e: &mut Engine,
    a: ScriptArgs,
    init: [u32; N],
) -> (bool, [u32; N]) {
    let block = e.mem.alloc(4 * N as u32);
    let mut outs = [0u32; N];
    for (i, value) in init.iter().enumerate() {
        outs[i] = block + 4 * i as u32;
        e.mem.set_u32(outs[i], *value);
    }
    let mut words = args![
        a.param_info,
        a.script_data,
        a.opcode_offset,
        a.this_obj,
        a.containing_obj,
        a.script_obj,
        a.event_list
    ];
    words.extend_from_slice(&outs);
    let ok = e.call(PARSE_PARAMETERS, &words).bool();
    let mut values = init;
    for (i, value) in values.iter_mut().enumerate() {
        *value = e.mem.u32(outs[i]);
    }
    e.mem.free(block);
    (ok, values)
}

/// `__RTDynamicCast(object, TESObjectREFR -> Actor)`: the actor, or 0.
fn cast_to_actor(e: &mut Engine, object: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
    )
    .u32()
}

fn console_print(e: &mut Engine, words: &[u32]) {
    e.call(CONSOLE_PRINT, words);
}

/// The actor's process (`MiddleHighProcess::GetSavedAcquireObject`).
fn saved_acquire_object(e: &mut Engine, actor: u32) -> u32 {
    e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32()
}

/// Copies `words` 32-bit words from `from` to `to`.
fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The twin of a condition function without arguments: calls
/// `callee(thisObj, 0, 0, result)` and returns its `AL`.
fn plain_condition(e: &mut Engine, a: ScriptArgs, callee: u32) -> bool {
    e.call(callee, &args![a.this_obj, 0u32, 0u32, a.result])
        .bool()
}

/// Parses one argument and returns `callee(thisObj, argument, 0, result)`.
fn one_argument_condition(e: &mut Engine, a: ScriptArgs, callee: u32) -> bool {
    let Some([argument]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(callee, &args![a.this_obj, argument, 0u32, a.result])
        .bool()
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005c8450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDisposition` (Xbox PDB): parses one actor argument and
/// returns `Script::GetDispositionConditionFunction(thisObj, actor, 0,
/// result)`.
pub fn script_get_disposition(e: &mut Engine, a: ScriptArgs) -> bool {
    one_argument_condition(e, a, GET_DISPOSITION_CONDITION)
}

// Translated from 005c84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetRandomPercent` command: `Script::GetRandomPercentConditionFunction
/// (thisObj, 0, 0, result)`.
pub fn fn_005c84b0(e: &mut Engine, a: ScriptArgs) -> bool {
    plain_condition(e, a, GET_RANDOM_PERCENT_CONDITION)
}

// Translated from 005c84d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetLevel` command: `Script::GetLevelConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005c84d0(e: &mut Engine, a: ScriptArgs) -> bool {
    plain_condition(e, a, GET_LEVEL_CONDITION)
}

// Translated from 005c84f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetArmorRating` command: `Script::GetArmorRatingConditionFunction
/// (thisObj, 0, 0, result)`.
pub fn fn_005c84f0(e: &mut Engine, a: ScriptArgs) -> bool {
    plain_condition(e, a, GET_ARMOR_RATING_CONDITION)
}

// Translated from 005c8510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModDisposition` (Xbox PDB): parses an actor argument and an
/// integer; when `thisObj` is an actor and the argument is given, calls
/// virtual slot `0x460` of the actor (`actor argument, (float) amount`) and
/// returns `Script::GetDispositionConditionFunction(thisObj, actor, 0,
/// result)`. False when the arguments do not parse or either is missing.
pub fn script_mod_disposition(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([target, amount]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 || target == 0 {
        return false;
    }
    e.vcall(actor, 0x460, &args![target, amount as i32 as f32]);
    e.call(
        GET_DISPOSITION_CONDITION,
        &args![a.this_obj, target, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDeadCount` (Xbox PDB): parses one form argument and returns
/// `Script::GetDeadCountConditionFunction(thisObj, form, 0, result)`.
pub fn script_get_dead_count(e: &mut Engine, a: ScriptArgs) -> bool {
    one_argument_condition(e, a, GET_DEAD_COUNT_CONDITION)
}

// Translated from 005c8620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowMap` (Xbox PDB): parses a map marker reference and a flag.
/// A reference that has map marker data is made visible (and marked
/// changed when it was not), its virtual slot `0x48` called with
/// `0x80000000`; with the flag set its travel location is enabled the same
/// way. When anything changed the "map marker added" message is shown with
/// its icon and sound. `*result` is 1.0.
pub fn script_show_map(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([marker, travel]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    let mut changed = false;
    if marker != 0 && e.call(GET_MAP_MARKER_DATA, &args![marker]).u32() != 0 {
        let data = e.call(GET_MAP_MARKER_DATA, &args![marker]).u32();
        if !e.call(MAP_MARKER_GET_VISIBLE, &args![data]).bool() {
            changed = true;
        }
        let data = e.call(GET_MAP_MARKER_DATA, &args![marker]).u32();
        e.call(MAP_MARKER_SET_VISIBLE, &args![data, 1u32]);
        e.vcall(marker, 0x48, &args![0x8000_0000u32]);
        if travel != 0 {
            let data = e.call(GET_MAP_MARKER_DATA, &args![marker]).u32();
            if !e.call(MAP_MARKER_GET_TRAVEL_LOC, &args![data]).bool() {
                changed = true;
            }
            let data = e.call(GET_MAP_MARKER_DATA, &args![marker]).u32();
            e.call(MAP_MARKER_SET_TRAVEL_LOC, &args![data, 1u32]);
        }
    }
    if changed {
        let text = e.call(BS_STRING_TEXT, &args![MAP_MARKER_MESSAGE]).u32();
        let volume: f32 = e.global(MESSAGE_VOLUME);
        e.call(
            SHOW_MESSAGE,
            &args![text, 0u32, ICON_MAP, SOUND_MAP_MARKER_ADDED, volume, 0u32],
        );
    }
    e.mem.set_f64(a.result.addr(), 1.0);
    true
}

// Translated from 005c9670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetAlertFunction` (Xbox PDB): parses an integer and, when
/// `thisObj` is an actor, sets its alert state to "the integer is positive"
/// (`Actor::SetAlert`).
pub fn script_set_alert_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([alert]) = parse_into(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let actor = cast_to_actor(e, a.this_obj);
        if actor != 0 {
            e.call(ACTOR_SET_ALERT, &args![actor, (alert as i32) > 0]);
        }
    }
    true
}

// Translated from 005c9700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetAlert` command: `Script::GetAlertConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005c9700(e: &mut Engine, a: ScriptArgs) -> bool {
    plain_condition(e, a, GET_ALERT_CONDITION)
}

// Translated from 005c9720 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that is `005a4340(thisObj, 0, 0, result)` of
/// `tesconditionfunctions.cpp`.
pub fn fn_005c9720(e: &mut Engine, a: ScriptArgs) -> bool {
    plain_condition(e, a, FN_005A4340)
}

// Translated from 005c9740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one object argument and calls `0060c950(object, 1)` on it, without
/// a null check.
pub fn fn_005c9740(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(FN_0060C950, &args![object, 1u32]);
    true
}

// Translated from 005c9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a target reference and a flag. When `thisObj` is an actor with a
/// process (`GetSavedAcquireObject`) and the target is given: with the flag
/// zero the process is told about the target (slot `0x62c`) and the actor's
/// extra data list records it as item dropper; otherwise the actor looks at
/// the target's position (virtual slot `0x1f4`, `Actor::SetLookAtTarget`),
/// the process is told (slot `0x7ac`) and the target is marked targeted.
/// Either way the actor's virtual slot `0x48` is then called with
/// `0x80000000`.
pub fn fn_005c9790(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([target, look]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if a.this_obj.is_null() || target == 0 {
        return true;
    }
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 || saved_acquire_object(e, actor) == 0 {
        return true;
    }
    if look == 0 {
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x62c, &args![target]);
        let extra_list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        e.call(SET_ITEM_DROPPER, &args![extra_list, target]);
    } else {
        let position = e.vcall(target, 0x1f4, &args![]).u32();
        let (x, y, z) = (
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        );
        e.call(ACTOR_SET_LOOK_AT_TARGET, &args![actor, x, y, z]);
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x7ac, &args![target]);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
    e.vcall(actor, 0x48, &args![0x8000_0000u32]);
    true
}

// Translated from 005c98e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Undoes [`fn_005c9790`] for `thisObj` when it is an actor: clears the item
/// dropper of its extra data list and, when it has a process, tells it
/// (slot `0x648` with 1), clears the look-at target and tells the process
/// again (slot `0x7ac` with 0).
pub fn fn_005c98e0(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 {
        return true;
    }
    let extra_list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
    e.call(SET_ITEM_DROPPER, &args![extra_list, 0u32]);
    if saved_acquire_object(e, actor) != 0 {
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x648, &args![1u32]);
        e.call(ACTOR_CLEAR_LOOK_AT_TARGET, &args![actor]);
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x7ac, &args![0u32]);
    }
    true
}

// Translated from 005c9980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two floats and stores their absolute values in the HDR globals
/// `fSunlightDimmer` (`011f9190`) and the luminance ramp (`011ad87c`).
pub fn fn_005c9980(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([sunlight, ramp]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    let value = e.call(FLOAT_ABS, &args![f32::from_bits(sunlight)]).f32();
    e.set_global(HDR_SUNLIGHT_DIMMER, value);
    let value = e.call(FLOAT_ABS, &args![f32::from_bits(ramp)]).f32();
    e.set_global(HDR_LUM_RAMP, value);
    true
}

// Translated from 005c9a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PrintHDRParamFunction` (Xbox PDB): prints the current HDR
/// parameters to the console, one line per group, reading the effect's
/// values through the accessors below and the four globals.
///
/// The blur line is printed with one `double` for a format that has `%d`
/// and `%f` (the game's own quirk); it is passed on unchanged.
pub fn script_print_hdr_param_function(e: &mut Engine) -> bool {
    let manager = e.call(GET_IMAGE_SPACE_MANAGER, &args![]).ptr::<()>();
    console_print(e, &args![MSG_HDR_PARAMS]);
    console_print(e, &args![MSG_SISG]);
    let blur_radius = fn_005c9b70(e, manager);
    console_print(e, &args![FORMAT_BLUR, blur_radius as f64]);
    let bright_scale = fn_005c9bf0(e, manager);
    let bright_clamp = fn_005c9bb0(e, manager);
    console_print(
        e,
        &args![FORMAT_BRIGHT, bright_clamp as f64, bright_scale as f64],
    );
    console_print(e, &args![MSG_SSP]);
    let sunlight_dimmer: f32 = e.global(HDR_SUNLIGHT_DIMMER);
    let lum_ramp: f32 = e.global(HDR_LUM_RAMP);
    console_print(
        e,
        &args![FORMAT_SUNLIGHT, sunlight_dimmer as f64, lum_ramp as f64],
    );
    console_print(e, &args![MSG_SHP]);
    let emissive_mult = fn_005c9b30(e, manager);
    let eye_adapt_speed = fn_005c9c30(e, manager);
    console_print(
        e,
        &args![
            FORMAT_EYE_ADAPT,
            eye_adapt_speed as f64,
            emissive_mult as f64
        ],
    );
    let grass_dimmer: f32 = e.global(HDR_GRASS_DIMMER);
    let tree_dimmer: f32 = e.global(HDR_TREE_DIMMER);
    console_print(
        e,
        &args![FORMAT_DIMMERS, tree_dimmer as f64, grass_dimmer as f64],
    );
    let target_lum = fn_005c9c70(e, manager);
    let upper_lum_clamp = fn_005c9cb0(e, manager);
    console_print(
        e,
        &args![FORMAT_LUM, upper_lum_clamp as f64, target_lum as f64],
    );
    true
}

/// The HDR effect of the image space manager (`005c9b30`'s and its
/// siblings' first step).
fn hdr_effect(e: &mut Engine, manager: Ptr) -> Ptr {
    e.call(IMAGE_SPACE_GET_EFFECT, &args![manager, 1u32]).ptr()
}

/// `ImageSpaceEffectHDR::pfData[index]` (`pfData` at +0x6c, Xbox PDB).
fn hdr_data(e: &mut Engine, effect: Ptr, index: u32) -> f32 {
    let data = e.mem.u32(effect.addr() + 0x6c);
    e.mem.f32(data + 4 * index)
}

// Translated from 005c9b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The emissive HDR multiplier (`HDR_EMISSIVE_MULT`) of the manager's HDR
/// effect.
pub fn fn_005c9b30(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9b50(e, effect)
}

// Translated from 005c9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_EMISSIVE_MULT]` (index 3).
pub fn fn_005c9b50(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 3)
}

// Translated from 005c9b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HDR blur radius (`HDR_BLUR_RADIUS`) of the manager's HDR effect.
pub fn fn_005c9b70(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9b90(e, effect)
}

// Translated from 005c9b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_BLUR_RADIUS]` (index 1).
pub fn fn_005c9b90(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 1)
}

// Translated from 005c9bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HDR bright clamp (`HDR_BRIGHT_CLAMP`) of the manager's HDR effect.
pub fn fn_005c9bb0(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9bd0(e, effect)
}

// Translated from 005c9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_BRIGHT_CLAMP]` (index 7).
pub fn fn_005c9bd0(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 7)
}

// Translated from 005c9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HDR bright scale (`HDR_BRIGHT_SCALE`) of the manager's HDR effect.
pub fn fn_005c9bf0(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9c10(e, effect)
}

// Translated from 005c9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_BRIGHT_SCALE]` (index 6).
pub fn fn_005c9c10(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 6)
}

// Translated from 005c9c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HDR eye adapt speed (`HDR_EYE_ADAPT_SPEED`) of the manager's HDR
/// effect.
pub fn fn_005c9c30(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9c50(e, effect)
}

// Translated from 005c9c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_EYE_ADAPT_SPEED]` (index 0).
pub fn fn_005c9c50(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 0)
}

// Translated from 005c9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HDR target luminance (`HDR_TARGET_LUM`) of the manager's HDR effect.
pub fn fn_005c9c70(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9c90(e, effect)
}

// Translated from 005c9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_TARGET_LUM]` (index 4).
pub fn fn_005c9c90(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 4)
}

// Translated from 005c9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HDR upper luminance clamp (`HDR_UPPER_LUM_CLAMP`) of the manager's
/// HDR effect.
pub fn fn_005c9cb0(e: &mut Engine, this: Ptr) -> f32 {
    let effect = hdr_effect(e, this);
    fn_005c9cd0(e, effect)
}

// Translated from 005c9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::pfData[HDR_UPPER_LUM_CLAMP]` (index 5).
pub fn fn_005c9cd0(e: &mut Engine, this: Ptr) -> f32 {
    hdr_data(e, this, 5)
}

// Translated from 005c9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Debug command: draws three yellow axis lines (an `NiLines` of six
/// vertices, +-length along each axis, with the connection flags
/// `1 0 1 0 1 0`) at the position of `thisObj` (virtual slot `0x1f4`) for
/// 20 seconds (`TES::AddTempDebugObject`). The half length is 10.0, or half
/// the radius of the world bound of the reference's 3D (virtual slot
/// `0x1d0`) when that is smaller. Does nothing without a reference.
pub fn fn_005c9cf0(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let this = a.this_obj.addr();
    let mut length: f32 = e.global(DEFAULT_AXIS_LENGTH);
    if e.vcall(this, 0x1d0, &args![]).u32() != 0 {
        let node = e.vcall(this, 0x1d0, &args![]).u32();
        let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
        let radius = e.call(BOUND_RADIUS, &args![bound]).f64();
        let half: f64 = e.global(HALF);
        let half_radius = (radius * half) as f32;
        // `FCOMPP`/`TEST AH,0x41`: the default is kept unless it is greater
        // (or the comparison is unordered).
        if !matches!(
            length.partial_cmp(&half_radius),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ) {
            length = half_radius;
        }
    }

    // Six `NiPoint3` vertices, six `NiColorA` colors and six connection
    // bytes.
    let vertices = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    if vertices != 0 {
        e.call(
            VECTOR_CONSTRUCT_ITERATOR,
            &args![vertices, 0xcu32, 6u32, NI_POINT3_DEFAULT_CONSTRUCT],
        );
    }
    let colors = e.call(OPERATOR_NEW, &args![0x60u32]).u32();
    if colors != 0 {
        e.call(
            VECTOR_CONSTRUCT_ITERATOR,
            &args![colors, 0x10u32, 6u32, NI_COLOR_A_DEFAULT_CONSTRUCT],
        );
    }
    let connections = e.call(OPERATOR_NEW, &args![6u32]).u32();
    for (i, flag) in [1u8, 0, 1, 0, 1, 0].into_iter().enumerate() {
        e.mem.set_u8(connections + i as u32, flag);
    }

    for i in 0..6 {
        e.with_stack(0x10, |e, temp| {
            let color = e
                .call(
                    NI_COLOR_A_CONSTRUCT,
                    &args![temp, 1.0f32, 1.0f32, 0.0f32, 1.0f32],
                )
                .u32();
            copy_words(e, colors + 0x10 * i, color, 4);
        });
    }

    let points = [
        (-length, 0.0f32, 0.0f32),
        (length, 0.0, 0.0),
        (0.0, -length, 0.0),
        (0.0, length, 0.0),
        (0.0, 0.0, -length),
        (0.0, 0.0, length),
    ];
    for (i, (x, y, z)) in points.into_iter().enumerate() {
        e.with_stack(0xc, |e, temp| {
            let point = e.call(NI_POINT3_CONSTRUCT, &args![temp, x, y, z]).u32();
            copy_words(e, vertices + 0xc * i as u32, point, 3);
        });
    }

    let storage = e.call(NI_ALLOC, &args![0xc4u32]).u32();
    let lines = if storage == 0 {
        0
    } else {
        e.call(
            NI_LINES_CONSTRUCT,
            &args![
                storage,
                6u32,
                vertices,
                colors,
                0u32,
                1u32,
                0u32,
                connections
            ],
        )
        .u32()
    };
    let position = e.vcall(this, 0x1f4, &args![]).u32();
    e.call(SET_TRANSLATE, &args![lines, position]);
    let seconds: f32 = e.global(DEBUG_OBJECT_SECONDS);
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(ADD_TEMP_DEBUG_OBJECT, &args![tes, lines, seconds]);
    true
}

// Translated from 005ca1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shared body of the script event flag commands: `*result` is 1.0 when
/// `thisObj` and the script event list are given and the event list
/// reports `event` with one of the bits of `mask` set (`005a8ef0`), else
/// 0.0.
pub fn fn_005ca1c0(
    e: &mut Engine,
    mask: u32,
    this_obj: Ptr,
    event: u32,
    event_list: u32,
    result: Ptr,
) {
    e.mem.set_f64(result.addr(), 0.0);
    if !this_obj.is_null()
        && event_list != 0
        && e.call(SCRIPT_EVENT_LIST_HAS_EVENT, &args![event_list, event, mask])
            .bool()
    {
        e.mem.set_f64(result.addr(), 1.0);
    }
}

/// A command that parses one argument and passes it to [`fn_005ca1c0`]
/// with `mask`.
fn event_flag_command(e: &mut Engine, a: ScriptArgs, mask: u32) -> bool {
    let Some([event]) = parse_into(e, a, [0]) else {
        return false;
    };
    fn_005ca1c0(e, mask, a.this_obj, event, a.event_list, a.result);
    true
}

// Translated from 005ca200 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `1` (see [`fn_005ca1c0`]).
pub fn fn_005ca200(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 1)
}

// Translated from 005ca260 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x4000` (see [`fn_005ca1c0`]).
pub fn fn_005ca260(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x4000)
}

// Translated from 005ca2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `2` (see [`fn_005ca1c0`]).
pub fn fn_005ca2d0(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 2)
}

// Translated from 005ca330 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `8` (see [`fn_005ca1c0`]).
pub fn fn_005ca330(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 8)
}

// Translated from 005ca390 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `4` (see [`fn_005ca1c0`]).
pub fn fn_005ca390(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 4)
}

// Translated from 005ca3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::OnActivateFunction` (Xbox PDB): parses one (unused) argument;
/// `*result` is 1.0 when `thisObj` has action 2 set, and both actions 1 and
/// 2 are cleared afterwards.
pub fn script_on_activate_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    if parse_into(e, a, [0]).is_none() {
        return false;
    }
    if !a.this_obj.is_null() {
        if e.call(HAS_ACTION, &args![a.this_obj, 2u32]).bool() {
            e.mem.set_f64(a.result.addr(), 1.0);
        }
        e.call(CLEAR_ACTION, &args![a.this_obj, 1u32]);
        e.call(CLEAR_ACTION, &args![a.this_obj, 2u32]);
    }
    true
}

// Translated from 005ca470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `*result` is 1.0 unless `thisObj`'s virtual slot `0x168` gives a
/// non-null pointer to a non-null pointer; the slot is asked afresh at each
/// step, as the game does. The game dereferences `thisObj` without a null
/// check (its later check never matters).
pub fn fn_005ca470(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let this = a.this_obj.addr();
    let mut value = 0u32;
    if e.vcall(this, 0x168, &args![]).u32() != 0 {
        let slot = e.vcall(this, 0x168, &args![]).u32();
        if fn_005ca4f0(e, Ptr::new(slot)) {
            let slot = e.vcall(this, 0x168, &args![]).u32();
            value = e.call(NI_POINTER_GET, &args![slot]).u32();
        }
    }
    if !a.this_obj.is_null() && value == 0 {
        e.mem.set_f64(a.result.addr(), 1.0);
    }
    true
}

// Translated from 005ca4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the pointer stored at `this` is non-null.
pub fn fn_005ca4f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) != 0
}

// Translated from 005ca510 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x400000` that reads no argument and
/// asks for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005ca510(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    fn_005ca1c0(e, 0x40_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// ---- Callees, globals and strings of the second batch -----------------------------

/// `Script::GetHeadingAngleConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, target, 0, result`).
const GET_HEADING_ANGLE_CONDITION: u32 = 0x005a_0410;
/// `Script::GetPlayerControlsDisabledConditionFunction` (Xbox PDB), `cdecl`
/// (`0, flags, 0, result`).
const GET_PLAYER_CONTROLS_DISABLED_CONDITION: u32 = 0x005a_02f0;
/// `Script::IsWeaponOutConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, 0, 0, result`).
const IS_WEAPON_OUT_CONDITION: u32 = 0x005a_0550;
/// `Script::IsWeaponInListConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, list, 0, result`).
const IS_WEAPON_IN_LIST_CONDITION: u32 = 0x005a_05e0;
/// Condition function of `tesconditionfunctions.cpp` (unnamed, `cdecl`,
/// `thisObj, 0, 0, result`) that stores 0.0 and returns true.
const FN_005A2A50: u32 = 0x005a_2a50;
/// `Script::IsFacingUpConditionFunction` (Xbox PDB), same arguments.
const IS_FACING_UP_CONDITION: u32 = 0x005a_0710;
/// `Script::IsLeftUpConditionFunction` (Xbox PDB), same arguments.
const IS_LEFT_UP_CONDITION: u32 = 0x005a_0800;
/// `Script::GetKnockedStateConditionFunction` (Xbox PDB), same arguments.
const GET_KNOCKED_STATE_CONDITION: u32 = 0x005a_08c0;
/// `Script::GetWeaponAnimTypeConditionFunction` (Xbox PDB), same arguments.
const GET_WEAPON_ANIM_TYPE_CONDITION: u32 = 0x005a_09b0;
/// `Script::IsWeaponSkillTypeConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, skill, 0, result`).
const IS_WEAPON_SKILL_TYPE_CONDITION: u32 = 0x005a_0ab0;
/// `Script::GetCurrentAIPackageConditionFunction` (Xbox PDB), same
/// arguments as [`IS_WEAPON_OUT_CONDITION`].
const GET_CURRENT_AI_PACKAGE_CONDITION: u32 = 0x005a_0b60;
/// `Script::IsWaitingConditionFunction` (Xbox PDB), same arguments.
const IS_WAITING_CONDITION: u32 = 0x005a_0e80;
/// `Script::IsIdlePlayingConditionFunction` (Xbox PDB), same arguments.
const IS_IDLE_PLAYING_CONDITION: u32 = 0x005a_0f10;

/// `thiscall` on the player (`playercharacter.cpp`, unnamed; `set, mask`):
/// adds the bits of `mask` to the byte at `+0x680` of the player (when `set`
/// is non-zero) or removes them (when zero) and passes the result to
/// `PlayerCharacter::SetControlsDisabled` (Xbox PDB).
const PLAYER_CHANGE_CONTROL_BITS: u32 = 0x0095_f530;
/// `PlayerCharacter::GetCurrentTargetList` (Xbox PDB), `thiscall` on the
/// player: the head node of its current target list, or 0.
const PLAYER_GET_CURRENT_TARGET_LIST: u32 = 0x0095_2ba0;
/// `thiscall` on the player: the dword at `+0x6b8`, the active quest
/// (translated here as `fn_005cbb50`).
const PLAYER_GET_ACTIVE_QUEST: u32 = 0x005c_bb50;
/// `thiscall` on a quest target (translated here as `fn_005cbb70`): the
/// reference of the first door of its teleport path, else its own reference.
const QUEST_TARGET_GET_LOAD_DOOR: u32 = 0x005c_bb70;
/// `TESQuestTarget::GetReference` (Xbox PDB), `thiscall` (`which`, 0 or 1).
const QUEST_TARGET_GET_REFERENCE: u32 = 0x0061_01b0;
/// `VATS::GetCount` (the engine map's name; the body walks a node list and
/// counts the nodes whose data pointer is non-null).
const LIST_COUNT_NON_EMPTY: u32 = 0x005a_e380;
/// `BGSSaveFormBuffer::GetForm` (the engine map's name; the body returns the
/// dword at `+0x20`), `thiscall`.
const BGS_SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;
/// `TESIdleManager::GetIdleToPlay` (Xbox PDB), `thiscall` on the idle
/// manager (`actor, word`).
const GET_IDLE_TO_PLAY: u32 = 0x0060_0950;
/// `TESForm::GetFormByEditorID` (Xbox PDB), `cdecl` (`text`).
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
/// `Animation::SpecialIdleDonePlaying` (Xbox PDB), `thiscall`.
const SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
/// `thiscall` (`animation.cpp`, unnamed): the object the pointer at `+0x18`
/// of the argument points to.
const FN_00490E40: u32 = 0x0049_0e40;
/// `thiscall` (`tesobjectrefr.cpp` range, unnamed): the dword at `+0x2c`.
const FN_0055B980: u32 = 0x0055_b980;
/// `thiscall` (the engine map calls it `D3DTexture_LockRect`, a name
/// identical-code folding put on it): the dword at `+0x24`.
const FN_0059BB30: u32 = 0x0059_bb30;
/// `thiscall` (unnamed): the dword at `+0xc`; the form ID of a form (the
/// idle commands also compare it with 3 on the object an animation holds).
const GET_DWORD_0C: u32 = 0x0084_e3a0;
/// `thiscall` (unnamed): the byte at `+4` of a form, its form type.
const FORM_GET_TYPE: u32 = 0x0040_1170;
/// `thiscall` (unnamed): `this + 0x18`, the head node of the list a form of
/// type [`FORM_TYPE_LIST`] keeps.
const LIST_FORM_FIRST_NODE: u32 = 0x0050_0940;
/// `thiscall` (unnamed): whether a list node is empty (no data pointer and
/// no next node).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `thiscall` (unnamed): returns `this`; the node's data pointer is read
/// through it.
const NODE_DATA_POINTER: u32 = 0x0068_15c0;
/// `thiscall` (unnamed): the next node, the dword at `+4`.
const NODE_NEXT: u32 = 0x0072_6070;
/// `thiscall` on a string object (unnamed): constructs an empty string.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `thiscall` on a string object (unnamed; `text, 0`): assigns the text.
const STRING_ASSIGN: u32 = 0x0040_37f0;
/// `thiscall` on a string object (unnamed): destroys it.
const STRING_DESTROY: u32 = 0x0040_37d0;
/// `cdecl` formatter (unnamed; `string, format, ...`): formats into the
/// string object.
const STRING_FORMAT: u32 = 0x0040_6f60;

/// RTTI type descriptor of `TESForm` (`.?AVTESForm@@`).
const RTTI_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor of `TESIdleForm` (`.?AVTESIdleForm@@`).
const RTTI_TES_IDLE_FORM: u32 = 0x0118_6a18;
/// RTTI type descriptor of `BGSCameraPath` (`.?AVBGSCameraPath@@`).
const RTTI_BGS_CAMERA_PATH: u32 = 0x0118_c268;

/// Pointer to the idle manager (`TESIdleManager`).
const IDLE_MANAGER: u32 = 0x011c_b6a0;
/// Pointer to the player (`PlayerCharacter`).
const PLAYER: u32 = 0x011d_ea3c;
/// Pointer to the camera path `PlayVATSCameras` queues for the next VATS
/// playback.
const QUEUED_VATS_CAMERA_PATH: u32 = 0x011f_21c0;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;

/// Form type number a form must have to be walked as a list by the command
/// at `005ca6e0` (the name of the class is not confirmed).
const FORM_TYPE_LIST: u32 = 0x55;
/// Form type number the form a quest target's load door holds at `+0x20`
/// must have for the door to be described (the class is not confirmed).
const FORM_TYPE_LOAD_DOOR: u32 = 0x1c;
/// Offset in an animation of the pointer `start_idle` reads first.
const ANIMATION_POINTER_128: u32 = 0x128;
/// Offset in an animation of the pointer `start_idle` examines when a
/// special idle is playing.
const ANIMATION_POINTER_124: u32 = 0x124;

/// `"Picked Idle '%s' (%08X) file: %s"`
const FORMAT_PICKED_IDLE: u32 = 0x0103_b478;
/// `"PlayIdle found '%s' (%08X) file: %s"`
const FORMAT_PLAY_IDLE_FOUND: u32 = 0x0103_b49c;
/// `"PlayVATSCameras '%s' not found."`
const FORMAT_VATS_NOT_FOUND: u32 = 0x0103_b4c0;
/// `"PlayVATSCameras '%s' queued for next VATS playback."`
const FORMAT_VATS_QUEUED: u32 = 0x0103_b4e0;
/// `"No active quest"`
const MSG_NO_ACTIVE_QUEST: u32 = 0x0103_b514;
/// `"Target %d:  Reference: %s, load door: %s"`
const FORMAT_TARGET: u32 = 0x0103_b524;
/// `"%s (%08X) (carrying %s (%08X))"`
const FORMAT_CARRYING: u32 = 0x0103_b550;
/// `"%s (%08X)"`
const FORMAT_NAME_AND_ID: u32 = 0x0103_b570;
/// `"Same cell/exterior"`
const TEXT_SAME_CELL: u32 = 0x0103_b57c;
/// `"%d current targets"`
const FORMAT_CURRENT_TARGETS: u32 = 0x0103_b590;
/// `"No current targets"`
const MSG_NO_CURRENT_TARGETS: u32 = 0x0103_b5a4;
/// `"Active quest: %s"`
const FORMAT_ACTIVE_QUEST: u32 = 0x0103_b5b8;

/// Size of the string objects the game keeps on its stack in
/// `ShowQuestTargets` (the eight bytes between the neighbouring locals).
const STRING_OBJECT_SIZE: u32 = 8;
/// Size of the stack buffer the editor ID commands parse their text into.
const TEXT_BUFFER_SIZE: u32 = 0x200;

// ---- Helpers of the second batch --------------------------------------------------

/// Whether the commands echo their result to the console: the byte at
/// `+0x268` of the TLS block.
fn echo_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_ECHO) != 0
}

/// `__RTDynamicCast(object, 0, source, target, 0)`: the cast object, or 0.
fn dynamic_cast(e: &mut Engine, object: u32, source: u32, target: u32) -> u32 {
    e.call(DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .u32()
}

/// `Script::ParseParameters` with one text parameter written into `buffer`
/// (the game's stack buffer): whether it parsed.
fn parse_text(e: &mut Engine, a: ScriptArgs, buffer: Ptr) -> bool {
    e.call(
        PARSE_PARAMETERS,
        &args![
            a.param_info,
            a.script_data,
            a.opcode_offset,
            a.this_obj,
            a.containing_obj,
            a.script_obj,
            a.event_list,
            buffer
        ],
    )
    .bool()
}

/// The command is the null check of `thisObj` followed by
/// `callee(thisObj, 0, 0, result)`: true without a call when there is no
/// `thisObj`.
fn this_condition(e: &mut Engine, a: ScriptArgs, callee: u32) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    plain_condition(e, a, callee)
}

/// The bit each of the seven flags of the player controls commands sets, in
/// the order the flags are parsed.
const CONTROL_BITS: [u32; 7] = [1, 4, 8, 0x10, 2, 0x20, 0x40];

/// Parses the seven flags of the player controls commands (initialised to
/// `init`) and combines the non-zero ones into the control mask; `None`
/// when the parameters do not parse.
fn parse_control_mask(e: &mut Engine, a: ScriptArgs, init: [u32; 7]) -> Option<u32> {
    let flags = parse_into(e, a, init)?;
    let mut mask = 0;
    for (flag, bit) in flags.iter().zip(CONTROL_BITS) {
        if *flag != 0 {
            mask |= bit;
        }
    }
    Some(mask)
}

/// What `PickIdle` and `PlayIdle` do once they hold an actor (with a saved
/// process) and the idle form: unless the actor's animation is busy with a
/// special idle (or already runs this idle), the process is told to play
/// the idle (slot `0x71c`, then slot `0x614` with `0x80`); with the echo
/// flag set `message` is printed with the idle's editor ID, form ID and
/// file.
fn start_idle(e: &mut Engine, actor: u32, idle: u32, message: u32) {
    // The actor's animation (virtual slot 0x1e4).
    let animation = e.vcall(actor, 0x1e4, &args![]).u32();
    if animation == 0 {
        return;
    }
    // Busy while a special idle is still playing ...
    let mut busy = !e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool();
    let pointer_128 = animation + ANIMATION_POINTER_128;
    if e.call(NI_POINTER_GET, &args![pointer_128]).u32() != 0 {
        let current = e.call(NI_POINTER_GET, &args![pointer_128]).u32();
        // ... and nothing happens when this idle is the one already held.
        if e.call(FN_0055B980, &args![current]).u32() == idle {
            return;
        }
    }
    let pointer_124 = animation + ANIMATION_POINTER_124;
    if busy && e.call(NI_POINTER_GET, &args![pointer_124]).u32() != 0 {
        let held = e.call(NI_POINTER_GET, &args![pointer_124]).u32();
        // Still busy only when the held object has the value 3 and the
        // object two steps behind it is absent or has a non-zero `+0x24`.
        busy = if e.call(GET_DWORD_0C, &args![held]).u32() == 3 {
            let held = e.call(NI_POINTER_GET, &args![pointer_124]).u32();
            if e.call(FN_00490E40, &args![held]).u32() == 0 {
                true
            } else {
                let held = e.call(NI_POINTER_GET, &args![pointer_124]).u32();
                let inner = e.call(FN_00490E40, &args![held]).u32();
                e.call(FN_0059BB30, &args![inner]).u32() != 0
            }
        } else {
            false
        };
    }
    if busy {
        return;
    }
    if saved_acquire_object(e, actor) != 0 {
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x71c, &args![idle]);
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x614, &args![0x80u32]);
    }
    if echo_enabled(e) {
        // The sub-object at `+0x18` of the idle form answers slot 0x14 with
        // the file; the game asks for it before the form ID and editor ID.
        let file = e.vcall(idle + 0x18, 0x14, &args![]).u32();
        let form_id = e.call(GET_DWORD_0C, &args![idle]).u32();
        let editor_id = e.vcall(idle, 0x130, &args![]).u32();
        console_print(e, &[message, editor_id, form_id, file]);
    }
}

/// The `ShowQuestTargets` description of one quest target, built in the two
/// string objects of the game's stack frame (`reference_text`,
/// `door_text`): the reference text (editor ID and form ID of what
/// `GetReference(1)` returns, with "carrying" what `GetReference(0)`
/// returns when they differ) and the load door text, printed with the
/// target's number.
fn show_quest_target(e: &mut Engine, item: u32, number: u32, reference_text: Ptr, door_text: Ptr) {
    e.call(STRING_CONSTRUCT, &args![reference_text]);
    e.call(STRING_CONSTRUCT, &args![door_text]);
    e.call(STRING_ASSIGN, &args![door_text, TEXT_SAME_CELL, 0u32]);
    let reference_a = e.call(QUEST_TARGET_GET_REFERENCE, &args![item, 0u32]).u32();
    let reference_b = e.call(QUEST_TARGET_GET_REFERENCE, &args![item, 1u32]).u32();
    if reference_b != 0 && reference_a != 0 {
        if reference_b == reference_a {
            let form_id = e.call(GET_DWORD_0C, &args![reference_a]).u32();
            let editor_id = e.vcall(reference_a, 0x130, &args![]).u32();
            e.call(
                STRING_FORMAT,
                &args![reference_text, FORMAT_NAME_AND_ID, editor_id, form_id],
            );
        } else {
            let id_a = e.call(GET_DWORD_0C, &args![reference_a]).u32();
            let name_a = e.vcall(reference_a, 0x130, &args![]).u32();
            let id_b = e.call(GET_DWORD_0C, &args![reference_b]).u32();
            let name_b = e.vcall(reference_b, 0x130, &args![]).u32();
            e.call(
                STRING_FORMAT,
                &args![reference_text, FORMAT_CARRYING, name_b, id_b, name_a, id_a],
            );
        }
    }
    let door = e.call(QUEST_TARGET_GET_LOAD_DOOR, &args![item]).u32();
    if door != 0 {
        let door_form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![door]).u32();
        if e.call(FORM_GET_TYPE, &args![door_form]).u32() == FORM_TYPE_LOAD_DOOR {
            let form_id = e.call(GET_DWORD_0C, &args![door]).u32();
            let editor_id = e.vcall(door, 0x130, &args![]).u32();
            e.call(
                STRING_FORMAT,
                &args![door_text, FORMAT_NAME_AND_ID, editor_id, form_id],
            );
        }
    }
    let door_words = e.call(NI_POINTER_GET, &args![door_text]).u32();
    let reference_words = e.call(NI_POINTER_GET, &args![reference_text]).u32();
    console_print(e, &[FORMAT_TARGET, number, reference_words, door_words]);
    e.call(STRING_DESTROY, &args![door_text]);
    e.call(STRING_DESTROY, &args![reference_text]);
}

// ---- Translated functions, second batch ---------------------------------------------

// Translated from 005ca540 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x10000` that reads no argument and asks
/// for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005ca540(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    fn_005ca1c0(e, 0x1_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005ca570 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x20000` that reads no argument and asks
/// for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005ca570(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    fn_005ca1c0(e, 0x2_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005ca5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x80000` that reads no argument and asks
/// for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005ca5a0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    fn_005ca1c0(e, 0x8_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005ca5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x100000` that reads no argument and
/// asks for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005ca5d0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    fn_005ca1c0(e, 0x10_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005ca600 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x200000` that asks for event 0. The
/// result is zeroed first and one argument is parsed (preset to
/// `0x80000000`) but never used; when it does not parse the command fails
/// with the result 0.0.
pub fn fn_005ca600(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    if parse_into(e, a, [0x8000_0000]).is_none() {
        return false;
    }
    fn_005ca1c0(e, 0x20_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005ca670 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x80` (see [`fn_005ca1c0`]).
pub fn fn_005ca670(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x80)
}

// Translated from 005ca6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x100` whose argument may be a list
/// form (form type `0x55`). A plain argument (or none) is asked for as the
/// event. For a list form, each entry is visited in order until the result
/// is no longer 0.0: the entry whose virtual slot `0xe4` answers true (else
/// 0) is the event asked for.
pub fn fn_005ca6e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form]) = parse_into(e, a, [0]) else {
        return false;
    };
    if form == 0 || e.call(FORM_GET_TYPE, &args![form]).u32() != FORM_TYPE_LIST {
        fn_005ca1c0(e, 0x100, a.this_obj, form, a.event_list, a.result);
        return true;
    }
    let mut node = e.call(LIST_FORM_FIRST_NODE, &args![form]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let data = e.call(NODE_DATA_POINTER, &args![node]).u32();
        let item = e.mem.u32(data);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let entry = if e.vcall(item, 0xe4, &args![]).bool() {
            item
        } else {
            0
        };
        fn_005ca1c0(e, 0x100, a.this_obj, entry, a.event_list, a.result);
        // The loop goes on while the result compares equal to (or
        // unordered with) 0.0.
        let result = e.mem.f64(a.result.addr());
        if !(result == 0.0 || result.is_nan()) {
            break;
        }
    }
    true
}

// Translated from 005ca800 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x10` (see [`fn_005ca1c0`]).
pub fn fn_005ca800(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x10)
}

// Translated from 005ca860 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x8000` (see [`fn_005ca1c0`]).
pub fn fn_005ca860(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x8000)
}

// Translated from 005ca8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x20` (see [`fn_005ca1c0`]).
pub fn fn_005ca8d0(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x20)
}

// Translated from 005ca930 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x40` that reads no argument and asks
/// for event 0 (see [`fn_005ca1c0`], which zeroes the result itself).
pub fn fn_005ca930(e: &mut Engine, a: ScriptArgs) -> bool {
    fn_005ca1c0(e, 0x40, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005ca950 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x40000`. The argument is parsed but
/// the parser's answer is ignored: whatever the local holds afterwards (0
/// when nothing was written) is the event asked for, and the command always
/// succeeds.
pub fn fn_005ca950(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [event]) = parse_into_unchecked(e, a, [0]);
    fn_005ca1c0(e, 0x4_0000, a.this_obj, event, a.event_list, a.result);
    true
}

// Translated from 005ca9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x200` (see [`fn_005ca1c0`]).
pub fn fn_005ca9b0(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x200)
}

// Translated from 005caa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x400` (see [`fn_005ca1c0`]).
pub fn fn_005caa20(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x400)
}

// Translated from 005caa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x800` (see [`fn_005ca1c0`]).
pub fn fn_005caa90(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x800)
}

// Translated from 005cab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x1000` that reads no argument and asks
/// for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005cab00(e: &mut Engine, a: ScriptArgs) -> bool {
    fn_005ca1c0(e, 0x1000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005cab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x80000000` that reads no argument and
/// asks for event 0 (see [`fn_005ca1c0`]).
pub fn fn_005cab30(e: &mut Engine, a: ScriptArgs) -> bool {
    fn_005ca1c0(e, 0x8000_0000, a.this_obj, 0, a.event_list, a.result);
    true
}

// Translated from 005cab60 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x2000` (see [`fn_005ca1c0`]).
pub fn fn_005cab60(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x2000)
}

// Translated from 005cabd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x10000000` (see [`fn_005ca1c0`]).
pub fn fn_005cabd0(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x1000_0000)
}

// Translated from 005cac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x20000000` (see [`fn_005ca1c0`]).
pub fn fn_005cac40(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x2000_0000)
}

// Translated from 005cacb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An event flag command with mask `0x40000000` (see [`fn_005ca1c0`]).
pub fn fn_005cacb0(e: &mut Engine, a: ScriptArgs) -> bool {
    event_flag_command(e, a, 0x4000_0000)
}

// Translated from 005cad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses seven flags (all preset to 1) and combines the non-zero ones into
/// a control mask (see [`CONTROL_BITS`]); calls
/// [`PLAYER_CHANGE_CONTROL_BITS`]`(player, 0, mask)`.
pub fn fn_005cad20(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some(mask) = parse_control_mask(e, a, [1; 7]) else {
        return false;
    };
    let player = e.mem.u32(PLAYER);
    e.call(PLAYER_CHANGE_CONTROL_BITS, &args![player, 0u32, mask]);
    true
}

// Translated from 005cae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005cad20`] but only the first three flags are preset to 1 (the
/// other four to 0), and the call passes 1 as the first argument:
/// [`PLAYER_CHANGE_CONTROL_BITS`]`(player, 1, mask)`.
pub fn fn_005cae30(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some(mask) = parse_control_mask(e, a, [1, 1, 1, 0, 0, 0, 0]) else {
        return false;
    };
    let player = e.mem.u32(PLAYER);
    e.call(PLAYER_CHANGE_CONTROL_BITS, &args![player, 1u32, mask]);
    true
}

// Translated from 005caf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerControlsDisabledFunction` (Xbox PDB): parses the seven
/// flags (all preset to 1), combines them into the control mask and returns
/// true after `Script::GetPlayerControlsDisabledConditionFunction(0, mask,
/// 0, result)`.
pub fn script_get_player_controls_disabled_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some(mask) = parse_control_mask(e, a, [1; 7]) else {
        return false;
    };
    e.call(
        GET_PLAYER_CONTROLS_DISABLED_CONDITION,
        &args![0u32, mask, 0u32, a.result],
    );
    true
}

// Translated from 005cb050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetHeadingAngleFunction` (Xbox PDB): parses one reference
/// argument; without `thisObj` or without the argument the command
/// succeeds doing nothing, otherwise it returns
/// `Script::GetHeadingAngleConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_heading_angle_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([target]) = parse_into(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() || target == 0 {
        return true;
    }
    e.call(
        GET_HEADING_ANGLE_CONDITION,
        &args![a.this_obj, target, 0u32, a.result],
    )
    .bool()
}

// Translated from 005cb0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PickIdleFunction` (Xbox PDB): when `thisObj` is an actor with a
/// saved process, asks the idle manager which idle to play
/// (`TESIdleManager::GetIdleToPlay(actor, process slot 0x128)`) and, when
/// there is one, starts it (see [`start_idle`]) with the "Picked Idle"
/// message. Always true.
pub fn script_pick_idle_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 || saved_acquire_object(e, actor) == 0 {
        return true;
    }
    let process = saved_acquire_object(e, actor);
    let word = e.vcall(process, 0x128, &args![]).u32();
    let manager = e.mem.u32(IDLE_MANAGER);
    let idle = e.call(GET_IDLE_TO_PLAY, &args![manager, actor, word]).u32();
    if idle == 0 {
        return true;
    }
    start_idle(e, actor, idle, FORMAT_PICKED_IDLE);
    true
}

// Translated from 005cb2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PlayIdleFunction` (Xbox PDB): parses an editor ID (a stack
/// buffer of 512 bytes); when `thisObj` is an actor with a saved process and
/// the editor ID names an idle form (`TESIdleForm`), starts the idle (see
/// [`start_idle`]) with the "PlayIdle found" message. False only when the
/// text does not parse. The stack protector cookie is not translated.
pub fn script_play_idle_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = cast_to_actor(e, a.this_obj);
    e.with_stack(TEXT_BUFFER_SIZE, |e, buffer| {
        if !parse_text(e, a, buffer) {
            return false;
        }
        if actor == 0 || saved_acquire_object(e, actor) == 0 {
            return true;
        }
        let form = e.call(GET_FORM_BY_EDITOR_ID, &args![buffer]).u32();
        let idle = dynamic_cast(e, form, RTTI_TES_FORM, RTTI_TES_IDLE_FORM);
        if idle != 0 {
            start_idle(e, actor, idle, FORMAT_PLAY_IDLE_FOUND);
        }
        true
    })
}

// Translated from 005cb580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PlayVATSCamerasFunction` (Xbox PDB): parses an editor ID (512
/// byte stack buffer) and stores the `BGSCameraPath` it names (or 0) in the
/// global camera path for the next VATS playback; with the echo flag it
/// says whether the path was found. False only when the text does not
/// parse. The stack protector cookie is not translated.
pub fn script_play_vats_cameras_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(TEXT_BUFFER_SIZE, |e, buffer| {
        if !parse_text(e, a, buffer) {
            return false;
        }
        let form = e.call(GET_FORM_BY_EDITOR_ID, &args![buffer]).u32();
        let path = dynamic_cast(e, form, RTTI_TES_FORM, RTTI_BGS_CAMERA_PATH);
        e.mem.set_u32(QUEUED_VATS_CAMERA_PATH, path);
        if echo_enabled(e) {
            let format = if path == 0 {
                FORMAT_VATS_NOT_FOUND
            } else {
                FORMAT_VATS_QUEUED
            };
            console_print(e, &[format, buffer.addr()]);
        }
        true
    })
}

// Translated from 005cb660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsWeaponOutConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb660(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_WEAPON_OUT_CONDITION)
}

// Translated from 005cb690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWeaponInListFunction` (Xbox PDB): parses one list argument
/// and returns `Script::IsWeaponInListConditionFunction(thisObj, list, 0,
/// result)` (no null check of `thisObj`).
pub fn script_is_weapon_in_list_function(e: &mut Engine, a: ScriptArgs) -> bool {
    one_argument_condition(e, a, IS_WEAPON_IN_LIST_CONDITION)
}

// Translated from 005cb6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `005a2a50(thisObj, 0, 0, result)`; true without a
/// call when there is no `thisObj`.
pub fn fn_005cb6f0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, FN_005A2A50)
}

// Translated from 005cb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsFacingUpConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb720(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_FACING_UP_CONDITION)
}

// Translated from 005cb750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsLeftUpConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb750(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_LEFT_UP_CONDITION)
}

// Translated from 005cb780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::GetKnockedStateConditionFunction(thisObj,
/// 0, 0, result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb780(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, GET_KNOCKED_STATE_CONDITION)
}

// Translated from 005cb7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::GetWeaponAnimTypeConditionFunction(thisObj,
/// 0, 0, result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb7b0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, GET_WEAPON_ANIM_TYPE_CONDITION)
}

// Translated from 005cb7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWeaponSkillTypeFunction` (Xbox PDB): parses one argument;
/// without `thisObj` the command succeeds doing nothing, otherwise it
/// returns `Script::IsWeaponSkillTypeConditionFunction(thisObj, argument,
/// 0, result)`.
pub fn script_is_weapon_skill_type_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([skill]) = parse_into(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    e.call(
        IS_WEAPON_SKILL_TYPE_CONDITION,
        &args![a.this_obj, skill, 0u32, a.result],
    )
    .bool()
}

// Translated from 005cb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is
/// `Script::GetCurrentAIPackageConditionFunction(thisObj, 0, 0, result)`;
/// true without a call when there is no `thisObj`.
pub fn fn_005cb850(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, GET_CURRENT_AI_PACKAGE_CONDITION)
}

// Translated from 005cb880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsWaitingConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb880(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_WAITING_CONDITION)
}

// Translated from 005cb8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsIdlePlayingConditionFunction(thisObj, 0,
/// 0, result)`; true without a call when there is no `thisObj`.
pub fn fn_005cb8b0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_IDLE_PLAYING_CONDITION)
}

// Translated from 005cb8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowQuestTargetsFunction` (Xbox PDB): prints the player's
/// active quest ("Active quest: %s" with the quest's virtual slot `0x138`
/// text, or "No active quest"), then the number of current targets and, for
/// each target of the current target list, "Target %d:  Reference: %s, load
/// door: %s" (see [`show_quest_target`]). Always true. The compiler's
/// exception-unwinding frame is not translated; the command reads no
/// argument.
pub fn script_show_quest_targets_function(e: &mut Engine, _a: ScriptArgs) -> bool {
    let player = e.mem.u32(PLAYER);
    if e.call(PLAYER_GET_ACTIVE_QUEST, &args![player]).u32() == 0 {
        console_print(e, &[MSG_NO_ACTIVE_QUEST]);
        return true;
    }
    let quest = e.call(PLAYER_GET_ACTIVE_QUEST, &args![player]).u32();
    let quest_name = e.vcall(quest, 0x138, &args![]).u32();
    console_print(e, &[FORMAT_ACTIVE_QUEST, quest_name]);
    let mut list = e.call(PLAYER_GET_CURRENT_TARGET_LIST, &args![player]).u32();
    if list == 0 {
        console_print(e, &[MSG_NO_CURRENT_TARGETS]);
    } else {
        let count = e.call(LIST_COUNT_NON_EMPTY, &args![list]).u32();
        console_print(e, &[FORMAT_CURRENT_TARGETS, count]);
    }
    let mut number = 1;
    while list != 0 {
        let slot = e.call(NODE_DATA_POINTER, &args![list]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_DATA_POINTER, &args![list]).u32();
        let item = e.mem.u32(slot);
        list = e.call(NODE_NEXT, &args![list]).u32();
        e.with_stack(STRING_OBJECT_SIZE, |e, reference_text| {
            e.with_stack(STRING_OBJECT_SIZE, |e, door_text| {
                show_quest_target(e, item, number, reference_text, door_text);
            });
        });
        number += 1;
    }
    true
}

// ---- Third batch: callees outside the unit and constants ------------------------------

/// `__mbsnbcpy` (`dest, source, count`), `cdecl`: returns `dest`.
const MBS_NB_CPY: u32 = 0x00ec_a903;
/// `memset` (`dest, value, count`), `cdecl`.
const MEMSET: u32 = 0x0040_3d30;
/// `thiscall` on a two-word list head (`modelloader`-style unnamed
/// constructor of `playercharacter.cpp`): sets both words to 0.
const LIST_HEAD_CONSTRUCT: u32 = 0x0096_a2d0;
/// `thiscall` on a list head (unnamed): frees its nodes.
const LIST_HEAD_DESTROY: u32 = 0x0046_ffb0;
/// `cdecl` (unnamed; `list head, quest, flag, 0, 0`): fills the list head
/// with the entries of the player's quest log; `quest` (0 for all quests)
/// restricts them to one quest.
const QUEST_LOG_FILL_LIST: u32 = 0x0060_d8f0;
/// `thiscall` (unnamed): the dword at `+0x6c` of a quest log entry.
const ENTRY_GET_FIELD_6C: u32 = 0x005e_3fa0;
/// `TESQuestStageItem::GetLogEntry` (the engine map's name; `thiscall` on
/// the log entry, `RET 4`, the dword at `+0x6c` as argument): the text of
/// the entry.
const ENTRY_GET_TEXT: u32 = 0x0060_fdb0;
/// `thiscall` (unnamed): the word at `+2` of a date.
const DATE_GET_WORD_2: u32 = 0x004b_1050;
/// `thiscall` (unnamed): the first result of `004b1190` applied to the word
/// at `+0` of a date, an index into the table of string objects at
/// [`MONTH_TABLE`].
const DATE_GET_MONTH_INDEX: u32 = 0x004b_10a0;
/// `thiscall` (unnamed): the second result (a word) of `004b1190` applied to
/// the word at `+0` of a date.
const DATE_GET_DAY: u32 = 0x004b_1070;
/// `thiscall` (the engine map calls it `MapMarkerData::GetLocationName`, a
/// name identical-code folding put on it): the text of a string object
/// (`NiFixedString`-like, read through `NiPointer`) or the empty string.
const STRING_TEXT_OR_EMPTY: u32 = 0x0040_8da0;
/// `thiscall` on a `BSSimpleArray` (unnamed): its element count (the dword
/// at `+8`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `thiscall` on a `BSSimpleArray` of 0x10-byte elements (unnamed; `index`):
/// the address of the element.
const ARRAY_ELEMENT_ADDRESS: u32 = 0x0055_8dd0;

/// `Script::SetActionFlag` (Xbox PDB), `cdecl` (`target, extra data list,
/// flag`).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `MobileObject::IsinDialogue` (Xbox PDB), `thiscall`.
const IS_IN_DIALOGUE: u32 = 0x0093_36c0;
/// `Actor::GetPackageSetAsPcurrent` (Xbox PDB), `thiscall`.
const GET_PACKAGE_SET_AS_PCURRENT: u32 = 0x0088_1510;
/// `TESPackage::GetIsCreated` (Xbox PDB), `thiscall`.
const PACKAGE_GET_IS_CREATED: u32 = 0x0067_4d40;
/// `ExtraDataList::GetPackageExtra` (Xbox PDB), `thiscall`.
const GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;
/// `ExtraDataList::RemovePackageExtra` (Xbox PDB), `thiscall`.
const REMOVE_PACKAGE_EXTRA: u32 = 0x0041_cc60;
/// `thiscall` on a package (unnamed): the dword at `+0x18`, its procedure
/// type (-1 when not yet calculated).
const PACKAGE_GET_PROCEDURE_TYPE: u32 = 0x0096_11e0;
/// `thiscall` on a reference (unnamed, `tesobjectrefr.cpp`; `RET 0`): the
/// value `TESPackage::CalculateProcedureType` is given.
const FN_00569B80: u32 = 0x0056_9b80;
/// `TESPackage::CalculateProcedureType` (Xbox PDB), `thiscall` on the
/// package (`value`).
const PACKAGE_CALCULATE_PROCEDURE_TYPE: u32 = 0x0067_77b0;
/// `thiscall` on a package (unnamed, `package.cpp`; `flag`).
const FN_00674E00: u32 = 0x0067_4e00;
/// `thiscall` on a reference (unnamed): the address of the dword at `+0x24`,
/// the reference's rotation (three floats).
const REFERENCE_ROTATION: u32 = 0x0043_0830;
/// `thiscall` on a reference (unnamed): the dword at `+0x40`, its cell.
const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB), `thiscall`.
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `TESObjectREFR::SetPackageStartLocation` (Xbox PDB), `thiscall` (`world
/// space, cell, position, angle`).
const SET_PACKAGE_START_LOCATION: u32 = 0x0056_ae10;
/// `thiscall` on a process (unnamed): the dword at `+0x28`.
const PROCESS_GET_FIELD_28: u32 = 0x0045_cd60;
/// `Calendar::GetHour` (Xbox PDB), `thiscall` on [`CALENDAR`]: the hour in
/// `ST0`.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// `thiscall` on a process (unnamed, `pathfinding`; `float`).
const FN_00693D50: u32 = 0x0069_3d50;
/// `ProcessLists::RemoveReference` (Xbox PDB), `thiscall` on
/// [`PROCESS_LISTS`] (`reference, value`).
const PROCESS_LISTS_REMOVE_REFERENCE: u32 = 0x0096_d470;
/// `ProcessLists::AddReference` (Xbox PDB), `thiscall` on [`PROCESS_LISTS`]
/// (`reference, value, 0, 0, 0`).
const PROCESS_LISTS_ADD_REFERENCE: u32 = 0x0096_d450;
/// `thiscall` on [`PROCESS_LISTS`] (unnamed, `processlists.cpp`; `actor,
/// 0`).
const PROCESS_LISTS_FN_00972840: u32 = 0x0097_2840;
/// `thiscall` on [`PROCESS_LISTS`] (unnamed, `processlists.cpp`; `v2, v1,
/// actor, v0, 0, -1`): answers `AL`.
const PROCESS_LISTS_FN_00972600: u32 = 0x0097_2600;
/// `PlayerCharacter::GetAnimation` (Xbox PDB), `thiscall` (`first person`).
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `Animation::SpecialIdleFree` (Xbox PDB), `thiscall` (`1, 0`).
const SPECIAL_IDLE_FREE: u32 = 0x0049_8910;
/// `Actor::GetOutofFurnitureQuick` (Xbox PDB), `thiscall`.
const GET_OUT_OF_FURNITURE_QUICK: u32 = 0x0088_d640;
/// `TESObjectREFR::AddAddonNodes` (Xbox PDB), `cdecl` (`node`): `AL`.
const ADD_ADDON_NODES: u32 = 0x0057_7e20;
/// `TESObjectREFR::RemoveAddonNodes` (Xbox PDB), `cdecl` (`node`): `AL`.
const REMOVE_ADDON_NODES: u32 = 0x0057_8300;
/// `NiAVObject::UpdateProperties` (Xbox PDB), `thiscall`.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiUpdateData` constructor (`modelloader.cpp`, unnamed), `thiscall`
/// (`time`, two bytes), `RET 0xc`.
const NI_UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `NiAVObject::Update`-style call (unnamed), `thiscall` (`update data`).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `thiscall` on an `NiAVObject` (unnamed, `modelloader.cpp`): copies the
/// `NiMatrix3` argument into it.
const SET_ROTATE: u32 = 0x0043_fa80;
/// `TESObjectREFR::GetOrientation` (Xbox PDB), `thiscall` (`NiMatrix3`
/// buffer): returns the buffer.
const GET_ORIENTATION: u32 = 0x0056_fa00;
/// `bhkNiCollisionObject::ResetSim` (Xbox PDB), `cdecl` (`object, 1`).
const RESET_SIM: u32 = 0x00c6_bd00;
/// `TES::IsCellLoaded` (Xbox PDB), `thiscall` on the `TES` singleton
/// (`cell, 0`).
const IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `TESObjectREFR::SetLocationOnReference` (Xbox PDB), `thiscall` (the
/// address of three floats).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `thiscall` on a reference (unnamed, `tesobjectrefr.cpp`): stores the
/// angle argument at `+0x2c`, clamping it for an actor, and calls virtual
/// slot `0x48` (`2`).
const SET_ANGLE_Z: u32 = 0x0057_57d0;
/// `thiscall` on a reference (unnamed, `tesobjectrefr.cpp`): stores the
/// `float` argument at `+0x24` and calls virtual slot `0x48` (`2`).
const SET_FIELD_24: u32 = 0x0057_5770;
/// `TESObjectREFR::MoveRefToNewSpace` (Xbox PDB), `cdecl` (`reference, cell,
/// world space`).
const MOVE_REF_TO_NEW_SPACE: u32 = 0x0057_3800;
/// `MobileObject::GetCharController` (Xbox PDB), `thiscall`.
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `bhkCharacterController::SetPosition` (Xbox PDB), `thiscall` (the address
/// of three floats).
const CHAR_CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
/// `Actor::StopMoving` (Xbox PDB), `thiscall`.
const ACTOR_STOP_MOVING: u32 = 0x008b_3ab0;
/// `Actor::FadeIn` (Xbox PDB), `thiscall`.
const ACTOR_FADE_IN: u32 = 0x008b_cd20;
/// `Actor::UpdateAlpha` (Xbox PDB), `thiscall`.
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
/// `PlayerCharacter::CheckForQuestTargetUpdate` (Xbox PDB), `thiscall`
/// (`reference`).
const CHECK_FOR_QUEST_TARGET_UPDATE: u32 = 0x0095_2c30;
/// `PlayerCharacter::RequestPositionPlayer` (Xbox PDB), `thiscall`
/// (`request`).
const REQUEST_POSITION_PLAYER: u32 = 0x0093_be30;
/// The constructor (unnamed) of the 0x34-byte position request, `thiscall`:
/// returns `this`.
const POSITION_REQUEST_CONSTRUCT: u32 = 0x0045_6540;
/// `thiscall` on a cell (unnamed): whether the cell is an interior cell
/// (the request then names the cell instead of a world space).
const CELL_FN_00425FD0: u32 = 0x0042_5fd0;
/// `Interface::CloseConsole` (Xbox PDB), `cdecl`.
const CLOSE_CONSOLE: u32 = 0x0070_3da0;
/// The error report of `tesscriptfunctions.cpp` (`005b5e40`, `cdecl`,
/// format first).
const SCRIPT_ERROR: u32 = 0x005b_5e40;
/// `thiscall` on a reference (unnamed, `tesobjectrefr.cpp`): whether the
/// reference can be moved by a script.
const CAN_BE_MOVED: u32 = 0x0057_2c80;
/// `thiscall` on the data handler (`tesdatahandler.cpp`, unnamed; `form
/// ID`): whether the form ID is in a loaded file.
const DATA_HANDLER_HAS_FORM_ID: u32 = 0x0046_9860;
/// The data handler singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The task queue (`004537b0`, no arguments: the word at `011df1a8`) and its queueing of a
/// command (`0087a8b0`, `cdecl`: queue, opcode, reference, target and the three offsets as
/// `double`s).
const TASK_QUEUE_INTERFACE: u32 = 0x0045_37b0;
const QUEUE_COMMAND: u32 = 0x0087_a8b0;
/// `thiscall` (unnamed, `modelloader.cpp`; `ECX`): `ECX + 4`, or the address
/// of a static zeroed dword when `ECX` is 0.
const GET_FIELD_4_ADDRESS: u32 = 0x0043_d4d0;
/// `HighProcess::FadeOutAndMove` (Xbox PDB), `thiscall` on the process
/// (`actor, target, x, y, z`).
const FADE_OUT_AND_MOVE: u32 = 0x008f_e9e0;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB), `thiscall`.
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `BGSOpenCloseForm::SetOpenState` (Xbox PDB), `cdecl` (`reference, 0, 1`).
const SET_OPEN_STATE: u32 = 0x0047_aec0;
/// `TESObjectREFR::GetLock` (Xbox PDB), `thiscall`.
const GET_LOCK: u32 = 0x0056_9160;
/// `TESObjectREFR::AddLock` (Xbox PDB), `thiscall`.
const ADD_LOCK: u32 = 0x0056_90a0;
/// `REFR_LOCK::SetLocked` (Xbox PDB), `thiscall` on the lock (`locked`).
const LOCK_SET_LOCKED: u32 = 0x0043_0a90;
/// `TESObjectREFR::AddLockChange` (Xbox PDB), `thiscall`.
const ADD_LOCK_CHANGE: u32 = 0x0057_89b0;
/// `thiscall` on a reference (unnamed, `tesobjectrefr.cpp`): the door
/// teleport data, or 0.
const GET_DOOR_TELEPORT_DATA: u32 = 0x0056_8e50;
/// `DoorTeleportData::SetConnectedCellsPublic` (Xbox PDB), `thiscall`
/// (`reference, public`).
const SET_CONNECTED_CELLS_PUBLIC: u32 = 0x0043_a210;
/// `thiscall` (unnamed, `bgsterminal.cpp`) on the terminal form (`reference,
/// level`).
const TERMINAL_FN_00501A80: u32 = 0x0050_1a80;
/// `BGSTerminal::SetHacked` (Xbox PDB), `cdecl` (`reference, 0`).
const TERMINAL_SET_HACKED: u32 = 0x0050_18b0;
/// `thiscall` (unnamed, `bgsterminal.cpp`) on the terminal form
/// (`reference`).
const TERMINAL_FN_00501B40: u32 = 0x0050_1b40;
/// `BGSTerminal::LockOut` (Xbox PDB), `cdecl` (`reference, 0`).
const TERMINAL_LOCK_OUT: u32 = 0x0050_1930;
/// `thiscall` on a reference (unnamed, `tesobjectrefr.cpp`): its full name.
const REFERENCE_GET_NAME: u32 = 0x0055_d520;
/// `GetCrimeConditionFunction`'s twin of `Script::GetCrimeFunction`
/// (`Script::GetCrimeConditionFunction`, Xbox PDB), `cdecl` (`thisObj,
/// crime, victim, result`).
const GET_CRIME_CONDITION: u32 = 0x005a_0f90;
/// `Script::GetIsCurrentPackageConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, package, 0, result`).
const GET_IS_CURRENT_PACKAGE_CONDITION: u32 = 0x005a_1c90;
/// `Script::IsGuardConditionFunction` (Xbox PDB), same arguments as
/// [`GET_ALERT_CONDITION`].
const IS_GUARD_CONDITION: u32 = 0x005a_10b0;
/// `Script::IsTalkingConditionFunction` (Xbox PDB), same arguments.
const IS_TALKING_CONDITION: u32 = 0x005a_1150;
/// `Script::GetWalkSpeedConditionFunction` (Xbox PDB), same arguments.
const GET_WALK_SPEED_CONDITION: u32 = 0x0059_c860;
/// Condition function of `tesconditionfunctions.cpp` (unnamed), same
/// arguments.
const FN_005A1210: u32 = 0x005a_1210;
/// Condition function of `tesconditionfunctions.cpp` (unnamed), same
/// arguments.
const FN_005A1770: u32 = 0x005a_1770;
/// `Script::IsTrespassingConditionFunction` (Xbox PDB), same arguments.
const IS_TRESPASSING_CONDITION: u32 = 0x005a_1900;
/// `Script::IsInMyOwnedCellConditionFunction` (Xbox PDB), same arguments.
const IS_IN_MY_OWNED_CELL_CONDITION: u32 = 0x005a_19a0;
/// `Script::IsContinuingPackagePCNearConditionFunction` (Xbox PDB), same
/// arguments.
const IS_CONTINUING_PACKAGE_PC_NEAR_CONDITION: u32 = 0x005a_1a40;
/// `Script::CanHaveFlamesConditionFunction` (Xbox PDB), same arguments.
const CAN_HAVE_FLAMES_CONDITION: u32 = 0x005a_1ad0;
/// `Script::HasFlamesConditionFunction` (Xbox PDB), same arguments.
const HAS_FLAMES_CONDITION: u32 = 0x005a_1b80;
/// `Script::GetOpenStateConditionFunction` (Xbox PDB), same arguments.
const GET_OPEN_STATE_CONDITION: u32 = 0x005a_1c30;
/// `Script::GetDayOfWeekConditionFunction` (Xbox PDB), same arguments.
const GET_DAY_OF_WEEK_CONDITION: u32 = 0x005a_1d80;
/// `Script::IsPCSleepingConditionFunction` (Xbox PDB), same arguments.
const IS_PC_SLEEPING_CONDITION: u32 = 0x005a_1de0;
/// `Script::IsPCAMurdererConditionFunction` (Xbox PDB), same arguments.
const IS_PC_A_MURDERER_CONDITION: u32 = 0x005a_1e70;
/// `thiscall` on the player (unnamed): the byte at `+0x658`, set while the
/// player sleeps or waits.
const PLAYER_GET_SLEEP_FLAG: u32 = 0x005a_1e50;
/// `thiscall` on the player (unnamed, `tesscriptfunctions.cpp`; `hours,
/// 1`): stores `hours` at `+0x654` and the byte `1` at `+0x658`.
const PLAYER_SET_SLEEP_HOURS: u32 = 0x005c_1a00;
/// `Interface::CloseSleepMenu` (Xbox PDB), `cdecl`.
const CLOSE_SLEEP_MENU: u32 = 0x0070_55c0;
/// `PlayerCharacter::SetSelectedSpell` (Xbox PDB), `thiscall` (`magic item`
/// or 0).
const PLAYER_SET_SELECTED_SPELL: u32 = 0x0094_c8e0;
/// `thiscall` on the player (unnamed, `playercharacter.cpp`; `book`):
/// selects a book (the command passes a `TESObjectBOOK`).
const PLAYER_SELECT_BOOK: u32 = 0x0094_c950;
/// `TESActorBaseData::SetFlagBit` (Xbox PDB), `thiscall` (`bit, value,
/// 1`).
const SET_FLAG_BIT: u32 = 0x0047_dd50;
/// `thiscall` on an actor (unnamed, `actor.cpp`; `value`).
const ACTOR_FN_008A8010: u32 = 0x008a_8010;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB), `thiscall` (the
/// engine map's name; it is called on the actor here).
const GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `TESCreature::InitDefaultWorn` (Xbox PDB), `thiscall` on the creature
/// form (`actor, 1, 1, 1`).
const INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// `thiscall` on the NPC form (unnamed, `tesnpc.cpp`; `actor, value, 1`).
const NPC_FN_00606540: u32 = 0x0060_6540;

/// The `ProcessLists` object (the address the methods get as `this`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The calendar object (the address `Calendar::GetHour` gets as `this`).
const CALENDAR: u32 = 0x011d_e7b8;
/// An object (the address `0043d4d0` gets as `this`) whose dword at `+4` the
/// `MoveTo` commands compare with 1: above it they send a message instead of
/// moving (unnamed).
const OBJECT_011C3EA4: u32 = 0x011c_3ea4;
/// Table of pointers to string objects (the month names, indexed by
/// [`DATE_GET_MONTH_INDEX`]).
const MONTH_TABLE: u32 = 0x0118_9588;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `FLT_MAX` (`0x7f7fffff`), the `float` the move commands store when a
/// moved actor's cell is not loaded.
const FLOAT_MAX: u32 = 0x0101_6970;
/// RTTI type descriptor of `SpellItem` (`.?AVSpellItem@@`).
const RTTI_SPELL_ITEM: u32 = 0x0118_3060;
/// RTTI type descriptor of `TESObjectBOOK` (`.?AVTESObjectBOOK@@`).
const RTTI_TES_OBJECT_BOOK: u32 = 0x0118_a650;

/// `"------------------------------------------------"`
const MSG_DASHES: u32 = 0x0103_b5dc;
/// `"%d of %s, %d"`
const FORMAT_DATE: u32 = 0x0103_b5cc;
/// `"%s"`
const FORMAT_TEXT: u32 = 0x0101_9f08;
/// `"%s cannot be moved."`
const FORMAT_CANNOT_BE_MOVED: u32 = 0x0103_a598;
/// `"SCRIPTS: Script '%s' (%08X) is attempting to move reference '%s'
/// (%08X), but it cannot be moved"`
const FORMAT_SCRIPT_CANNOT_MOVE: u32 = 0x0103_a538;
/// `"AI: Actor being moved is Disabled he has no process"`
const MSG_MOVED_ACTOR_HAS_NO_PROCESS: u32 = 0x0103_b66c;
/// `"AddFlames >> %0.2f"`
const FORMAT_ADD_FLAMES: u32 = 0x0103_b640;
/// `"RemoveFlames >> %0.2f"`
const FORMAT_REMOVE_FLAMES: u32 = 0x0103_b654;
/// `"Locked %s with lock level %d"`
const FORMAT_LOCKED: u32 = 0x0103_b610;
/// `"Unlocked %s "`
const FORMAT_UNLOCKED: u32 = 0x0103_b630;
/// `"Player Spell set to %s"`
const FORMAT_PLAYER_SPELL_SET: u32 = 0x0103_b6a0;
/// `"NONE"`
const TEXT_NONE: u32 = 0x0101_a6c0;

/// Size of the two-word list head the quest log commands keep on the stack.
const LIST_HEAD_SIZE: u32 = 8;
/// Size of the text buffer (`0x104` bytes) of the quest log commands.
const LOG_TEXT_BUFFER_SIZE: u32 = 0x104;
/// Longest log text copied (`0x103` bytes).
const LOG_TEXT_LENGTH: u32 = 0x103;
/// Size of the `NiUpdateData` the commands keep on the stack.
const UPDATE_DATA_SIZE: u32 = 0xc;
/// Size of an `NiMatrix3`.
const MATRIX_SIZE: u32 = 0x24;
/// Size of the position request `MoveToFunctionBase` allocates.
const POSITION_REQUEST_SIZE: u32 = 0x34;

// ---- Helpers of the third batch -----------------------------------------------------

/// Like [`this_condition`], but false (instead of true) when there is no
/// `thisObj`.
fn this_condition_or_false(e: &mut Engine, a: ScriptArgs, callee: u32) -> bool {
    if a.this_obj.is_null() {
        return false;
    }
    plain_condition(e, a, callee)
}

/// What the two quest log commands do once their arguments are parsed:
/// fills a list head with `QUEST_LOG_FILL_LIST(list, quest, flag, 0, 0)`
/// and prints, for each entry, a separator line, the entry's date ("%d of
/// %s, %d"), the name at `+0x30` of the dword at `+0x6c` of the entry and
/// the entry's text (copied into a 0x104-byte buffer, at most 0x103 bytes).
fn print_quest_log(e: &mut Engine, quest: u32, flag: u32) {
    e.with_stack(LOG_TEXT_BUFFER_SIZE, |e, buffer| {
        e.with_stack(LIST_HEAD_SIZE, |e, list| {
            e.call(LIST_HEAD_CONSTRUCT, &args![list]);
            e.call(QUEST_LOG_FILL_LIST, &args![list, quest, flag, 0u32, 0u32]);
            let mut node = list.addr();
            while node != 0 {
                let slot = e.call(NODE_DATA_POINTER, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(NODE_DATA_POINTER, &args![node]).u32();
                let entry = e.mem.u32(slot);
                let field_6c = e.call(ENTRY_GET_FIELD_6C, &args![entry]).u32();
                node = e.call(NODE_NEXT, &args![node]).u32();
                e.call(MEMSET, &args![buffer, 0u32, LOG_TEXT_BUFFER_SIZE]);
                let text = e.call(ENTRY_GET_TEXT, &args![entry, field_6c]).u32();
                e.call(MBS_NB_CPY, &args![buffer, text, LOG_TEXT_LENGTH]);
                let date = saved_acquire_object(e, entry);
                console_print(e, &[MSG_DASHES]);
                let word_2 = e.call(DATE_GET_WORD_2, &args![date]).u16() as u32;
                let month = e.call(DATE_GET_MONTH_INDEX, &args![date]).u32();
                let month_string = e.mem.u32(MONTH_TABLE + 4 * month);
                let month_name = e.call(BS_STRING_TEXT, &args![month_string]).u32();
                let day = e.call(DATE_GET_DAY, &args![date]).u16() as u32;
                console_print(e, &[FORMAT_DATE, day, month_name, word_2]);
                let field_6c = e.call(ENTRY_GET_FIELD_6C, &args![entry]).u32();
                let name = e.call(STRING_TEXT_OR_EMPTY, &args![field_6c + 0x30]).u32();
                console_print(e, &[FORMAT_TEXT, name]);
                console_print(e, &[FORMAT_TEXT, buffer.addr()]);
            }
            e.call(LIST_HEAD_DESTROY, &args![list]);
        });
    });
}

/// `f32` sum the x87 code makes: added in extended precision, stored as a
/// `float` (the bits).
fn float_sum_bits(word: u32, addend: f32) -> u32 {
    ((f32::from_bits(word) as f64 + addend as f64) as f32).to_bits()
}

/// The part `MoveTo` and the command at `005cd0e0` share: parses the target
/// and the offsets, refuses (with messages) a `thisObj` that cannot be
/// moved. `None` when the parameters do not parse, otherwise the target and
/// the offsets as `float` bits, the target 0 when nothing is to be moved.
fn move_to_prologue(e: &mut Engine, a: ScriptArgs) -> Option<(u32, [u32; 3])> {
    let [target, x, y, z] = parse_into(e, a, [0, 0, 0, 0])?;
    if !a.this_obj.is_null() && !e.call(CAN_BE_MOVED, &args![a.this_obj]).bool() {
        if echo_enabled(e) {
            let name = e.vcall(a.this_obj.addr(), 0x130, &args![]).u32();
            console_print(e, &[FORMAT_CANNOT_BE_MOVED, name]);
        }
        let handler = e.mem.u32(DATA_HANDLER);
        let script_id = e.call(GET_DWORD_0C, &args![a.script_obj]).u32();
        if !e
            .call(DATA_HANDLER_HAS_FORM_ID, &args![handler, script_id])
            .bool()
        {
            let this_id = e.call(GET_DWORD_0C, &args![a.this_obj]).u32();
            let this_name = e.vcall(a.this_obj.addr(), 0x130, &args![]).u32();
            let script_id = e.call(GET_DWORD_0C, &args![a.script_obj]).u32();
            let script_name = e.vcall(a.script_obj.addr(), 0x130, &args![]).u32();
            e.call(
                SCRIPT_ERROR,
                &args![
                    FORMAT_SCRIPT_CANNOT_MOVE,
                    script_name,
                    script_id,
                    this_name,
                    this_id
                ],
            );
        }
        return Some((0, [x, y, z]));
    }
    Some((target, [x, y, z]))
}

/// Whether the object at [`OBJECT_011C3EA4`] says to send the move as a
/// message (its dword at `+4` above 1).
fn move_is_messaged(e: &mut Engine) -> bool {
    let counter = e.call(GET_FIELD_4_ADDRESS, &args![OBJECT_011C3EA4]).u32();
    (e.mem.u32(counter) as i32) > 1
}

/// Builds and sends the message for a messaged move (see
/// [`move_is_messaged`]): `opcode` is `0x109e` or `0x124f`.
fn send_move_message(e: &mut Engine, opcode: u32, a: ScriptArgs, target: u32, offsets: [u32; 3]) {
    let [x, y, z] = offsets.map(|word| f32::from_bits(word) as f64);
    // 005cd09c..005cd0a7 (and 005cd233..005cd23e): the words are pushed before the getter
    // 004537b0, which takes none; they and its answer go to 0087a8b0.
    let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
    e.call(
        QUEUE_COMMAND,
        &args![queue, opcode, a.this_obj, target, x, y, z],
    );
}

// ---- Translated functions, third batch ------------------------------------------------

// Translated from 005cbb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the player: the active quest, the dword at `+0x6b8`
/// (`PlayerCharacter::pActiveQuest` at `+0x6c8` in the Xbox PDB).
pub fn fn_005cbb50(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x6b8)
}

// Translated from 005cbb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on a quest target: the reference of the first door of its
/// teleport path (the `BSSimpleArray<TeleportLink>` at `+0x20`) when the path
/// has doors, else the target's own reference (`m_pTargetRef`, `+0xc`).
pub fn fn_005cbb70(e: &mut Engine, this: Ptr) -> u32 {
    let doors = this.addr() + 0x20;
    if e.call(ARRAY_SIZE, &args![doors]).u32() != 0 {
        let link = e.call(ARRAY_ELEMENT_ADDRESS, &args![doors, 0u32]).u32();
        e.mem.u32(link)
    } else {
        e.mem.u32(this.addr() + 0xc)
    }
}

// Translated from 005cbbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A quest log command: parses one flag; prints the player's quest log
/// entries selected by the flag (see [`print_quest_log`]). False only when
/// the parameters do not parse. The compiler's exception frame and stack
/// protector are not translated.
pub fn fn_005cbbb0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_into(e, a, [0]) else {
        return false;
    };
    print_quest_log(e, 0, (flag != 0) as u32);
    true
}

// Translated from 005cbd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A wrapper of `__mbsnbcpy(dest, source, count)`; returns what it returns.
pub fn fn_005cbd90(e: &mut Engine, dest: u32, source: u32, count: u32) -> u32 {
    e.call(MBS_NB_CPY, &args![dest, source, count]).u32()
}

// Translated from 005cbdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A quest log command: parses one quest argument; prints the player's quest
/// log entries of that quest (see [`print_quest_log`]). False only when the
/// parameters do not parse. The compiler's exception frame and stack
/// protector are not translated.
pub fn fn_005cbdb0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_into(e, a, [0]) else {
        return false;
    };
    print_quest_log(e, quest, 0);
    true
}

// Translated from 005cbf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::LockFunction` (Xbox PDB): false without `thisObj` or when the
/// parameters (lock level, "connected cells" flag) do not parse. When
/// [`HAS_ACTION`] says the reference has action 4, `BGSOpenCloseForm::
/// SetOpenState(reference, 0, 1)` is called first. Without a lock, a
/// reference whose form has type `0x17` is handed to the terminal functions
/// (`BGSTerminal::SetHacked`) with the level clamped to `0..=5`; any other reference gets a new lock. With a lock, the level
/// (when non-zero) is stored in its first byte, it is set locked, the change
/// is recorded, the connected cells are made non-public when the flag is
/// above 0, and virtual slot `0xc8` is called with `1`. With the echo flag it
/// prints "Locked %s with lock level %d". Always true after the parse.
pub fn script_lock_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return false;
    }
    let this = a.this_obj.addr();
    let Some([mut level, connected]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if e.call(HAS_ACTION, &args![this, 4u32]).bool() {
        e.call(SET_OPEN_STATE, &args![this, 0u32, 1u32]);
    }
    let mut lock = e.call(GET_LOCK, &args![this]).u32();
    if lock == 0 {
        let form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
        if e.call(FORM_GET_TYPE, &args![form]).u32() == 0x17 {
            let form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
            let clamped = (level as i32).clamp(0, 5) as u32;
            e.call(TERMINAL_FN_00501A80, &args![form, this, clamped]);
            e.call(TERMINAL_SET_HACKED, &args![this, 0u32]);
            level = clamped;
        } else {
            lock = e.call(ADD_LOCK, &args![this]).u32();
        }
    }
    if lock != 0 {
        if level != 0 {
            e.mem.set_u8(lock, level as u8);
        }
        e.call(LOCK_SET_LOCKED, &args![lock, 1u32]);
        e.call(ADD_LOCK_CHANGE, &args![this]);
        if (connected as i32) > 0 {
            let teleport = e.call(GET_DOOR_TELEPORT_DATA, &args![this]).u32();
            if teleport != 0 {
                e.call(SET_CONNECTED_CELLS_PUBLIC, &args![teleport, this, 0u32]);
            }
        }
        e.vcall(this, 0xc8, &args![1u32]);
    }
    if echo_enabled(e) {
        let name = e.call(REFERENCE_GET_NAME, &args![this]).u32();
        console_print(e, &[FORMAT_LOCKED, name, level]);
    }
    true
}

// Translated from 005cc120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::UnlockFunction` (Xbox PDB): false without `thisObj` or when the
/// parameter (the "connected cells" flag) does not parse. A reference with a
/// lock gets it set unlocked, the change recorded, the connected cells made
/// public when the flag is above 0, and virtual slot `0xc8` called with `1`;
/// a reference without one whose form has type `0x17` is passed to the
/// terminal's lock-out functions. With the echo flag it prints "Unlocked %s
/// ". Always true after the parse.
pub fn script_unlock_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return false;
    }
    let this = a.this_obj.addr();
    let Some([connected]) = parse_into(e, a, [0]) else {
        return false;
    };
    let lock = e.call(GET_LOCK, &args![this]).u32();
    if lock != 0 {
        e.call(LOCK_SET_LOCKED, &args![lock, 0u32]);
        e.call(ADD_LOCK_CHANGE, &args![this]);
        if (connected as i32) > 0 {
            let teleport = e.call(GET_DOOR_TELEPORT_DATA, &args![this]).u32();
            if teleport != 0 {
                e.call(SET_CONNECTED_CELLS_PUBLIC, &args![teleport, this, 1u32]);
            }
        }
        e.vcall(this, 0xc8, &args![1u32]);
    } else {
        let form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
        if e.call(FORM_GET_TYPE, &args![form]).u32() == 0x17 {
            let form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
            e.call(TERMINAL_FN_00501B40, &args![form, this]);
            e.call(TERMINAL_LOCK_OUT, &args![this, 0u32]);
        }
    }
    if echo_enabled(e) {
        let name = e.call(REFERENCE_GET_NAME, &args![this]).u32();
        console_print(e, &[FORMAT_UNLOCKED, name]);
    }
    true
}

// Translated from 005cc240 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that zeroes the result, parses three arguments and answers
/// 1.0 when the `ProcessLists` object's `00972600` says so for them and
/// `thisObj` cast to an actor (`v2, v1, actor, v0, 0, -1`, with `v0..v2` the
/// arguments in order). False only when the parameters do not parse.
pub fn fn_005cc240(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([v0, v1, v2]) = parse_into(e, a, [0, 0, 0]) else {
        return false;
    };
    let actor = cast_to_actor(e, a.this_obj);
    if e.call(
        PROCESS_LISTS_FN_00972600,
        &args![PROCESS_LISTS, v2, v1, actor, v0, 0u32, 0xffff_ffffu32],
    )
    .bool()
    {
        e.mem.set_f64(a.result.addr(), 1.0);
    }
    true
}

// Translated from 005cc2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCrimeFunction` (Xbox PDB): zeroes the result and parses two
/// arguments (the second preset to -1); true without `thisObj`, else
/// `Script::GetCrimeConditionFunction(thisObj, first, second, result)`.
/// False when the parameters do not parse.
pub fn script_get_crime_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([crime, victim]) = parse_into(e, a, [0, 0xffff_ffff]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    e.call(
        GET_CRIME_CONDITION,
        &args![a.this_obj, crime, victim, a.result],
    )
    .bool()
}

// Translated from 005cc370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsGuardConditionFunction(thisObj, 0, 0,
/// result)`; false without a call when there is no `thisObj`.
pub fn fn_005cc370(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition_or_false(e, a, IS_GUARD_CONDITION)
}

// Translated from 005cc3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsTalkingConditionFunction(thisObj, 0, 0,
/// result)`; false without a call when there is no `thisObj`.
pub fn fn_005cc3a0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition_or_false(e, a, IS_TALKING_CONDITION)
}

// Translated from 005cc3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::GetWalkSpeedConditionFunction(thisObj, 0, 0,
/// result)`; false without a call when there is no `thisObj`.
pub fn fn_005cc3d0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition_or_false(e, a, GET_WALK_SPEED_CONDITION)
}

// Translated from 005cc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is the condition function `005a1210(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc400(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, FN_005A1210)
}

// Translated from 005cc430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is the condition function `005a1770(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc430(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, FN_005A1770)
}

// Translated from 005cc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsTrespassingConditionFunction(thisObj, 0,
/// 0, result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc460(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_TRESPASSING_CONDITION)
}

// Translated from 005cc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsInMyOwnedCellConditionFunction(thisObj, 0,
/// 0, result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc490(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_IN_MY_OWNED_CELL_CONDITION)
}

// Translated from 005cc4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is
/// `Script::IsContinuingPackagePCNearConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc4c0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, IS_CONTINUING_PACKAGE_PC_NEAR_CONDITION)
}

// Translated from 005cc4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A package command: parses a package; when `thisObj` is an actor (a
/// dialogue in progress is first ended through slot `0x288`) with a process
/// and the package is given, the actor's current package (process slot
/// `0x22c`) gets the action flag `0x800` and is handed back to the process
/// (slot `0x59c`), the process is told slot `0x5a8(0)`, the new package gets
/// the action flag `0x200` and is given to the process (slot `0x598`), its
/// procedure type is calculated when still -1, the package start location
/// is set from the actor's position and angle, and the actor is evaluated
/// (slot `0x2f4`). With a process holding a value at `+0x28` the process is
/// told the hour minus 1 and, when that value is 1, the actor is removed
/// from and added again to the process lists; without one, the player gets
/// the flag byte at `+0x79b` set. Always true after the parse.
pub fn fn_005cc4f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([package]) = parse_into(e, a, [0]) else {
        return false;
    };
    let actor = cast_to_actor(e, a.this_obj);
    if actor != 0 && e.call(IS_IN_DIALOGUE, &args![actor]).bool() {
        e.vcall(actor, 0x288, &args![]);
    }
    if actor == 0 || package == 0 {
        return true;
    }
    if saved_acquire_object(e, actor) == 0 {
        return true;
    }
    let process = saved_acquire_object(e, actor);
    let current = e.vcall(process, 0x22c, &args![]).u32();
    if current != 0 {
        let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        e.call(SET_ACTION_FLAG, &args![current, list, 0x800u32]);
        let process = saved_acquire_object(e, actor);
        e.vcall(process, 0x59c, &args![actor, current]);
    }
    let process = saved_acquire_object(e, actor);
    e.vcall(process, 0x5a8, &args![0u32]);
    let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
    e.call(SET_ACTION_FLAG, &args![package, list, 0x200u32]);
    let process = saved_acquire_object(e, actor);
    e.vcall(process, 0x598, &args![actor, package]);
    if e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).i32() == -1 {
        let value = e.call(FN_00569B80, &args![actor]).u32();
        e.call(PACKAGE_CALCULATE_PROCEDURE_TYPE, &args![package, value]);
    }
    let rotation = e.call(REFERENCE_ROTATION, &args![actor]).u32();
    let angle = e.mem.u32(rotation + 8);
    let position = e.vcall(actor, 0x1f4, &args![]).u32();
    let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
    let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
    e.call(
        SET_PACKAGE_START_LOCATION,
        &args![actor, world_space, cell, position, angle],
    );
    e.call(FN_00674E00, &args![package, 1u32]);
    e.vcall(actor, 0x2f4, &args![package, 0u32, 0u32]);
    if saved_acquire_object(e, actor) != 0 {
        let process = saved_acquire_object(e, actor);
        if e.call(PROCESS_GET_FIELD_28, &args![process]).u32() != 0 {
            let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
            let one: f64 = e.global(ONE);
            let value = (hour - one) as f32;
            let process = saved_acquire_object(e, actor);
            e.call(FN_00693D50, &args![process, value]);
            let process = saved_acquire_object(e, actor);
            if e.call(PROCESS_GET_FIELD_28, &args![process]).u32() == 1 {
                let process = saved_acquire_object(e, actor);
                let state = e.call(PROCESS_GET_FIELD_28, &args![process]).u32();
                e.call(
                    PROCESS_LISTS_REMOVE_REFERENCE,
                    &args![PROCESS_LISTS, actor, state],
                );
                let process = saved_acquire_object(e, actor);
                let state = e.call(PROCESS_GET_FIELD_28, &args![process]).u32();
                e.call(
                    PROCESS_LISTS_ADD_REFERENCE,
                    &args![PROCESS_LISTS, actor, state, 0u32, 0u32, 0u32],
                );
            }
            return true;
        }
    }
    let player = e.mem.u32(PLAYER);
    if actor == player {
        fn_005cc7a0(e, Ptr::new(player), 1);
    }
    true
}

// Translated from 005cc7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the player: stores the byte at `+0x79b`
/// (`bAiControlledPackage` at `+0x7ab` in the Xbox PDB).
pub fn fn_005cc7a0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x79b, value);
}

// Translated from 005cc7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A package command that reads no argument: when `thisObj` is an actor,
/// virtual slot `0x434(0)` is called and its package set as current (if any)
/// gets the action flag `0x400`: a created package leaves its package extra
/// (flagged and removed from the extra data list), another is told to stop
/// through its process (slots `0x234`, then `0x24(actor, 1)`). For the
/// player the flag byte at `+0x79b` is cleared and the first and third
/// person animations free their special idle. Always true.
pub fn fn_005cc7c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 {
        return true;
    }
    e.vcall(actor, 0x434, &args![0u32]);
    let package = e.call(GET_PACKAGE_SET_AS_PCURRENT, &args![actor]).u32();
    if package != 0 {
        if e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool() {
            let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
            let package_extra = e.call(GET_PACKAGE_EXTRA, &args![list]).u32();
            let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
            e.call(SET_ACTION_FLAG, &args![package_extra, list, 0x400u32]);
            let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
            e.call(REMOVE_PACKAGE_EXTRA, &args![list]);
        } else {
            let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
            e.call(SET_ACTION_FLAG, &args![package, list, 0x400u32]);
            let process = saved_acquire_object(e, actor);
            e.vcall(process, 0x234, &args![]);
            let process = saved_acquire_object(e, actor);
            e.vcall(process, 0x24, &args![actor, 1u32]);
        }
    }
    let player = e.mem.u32(PLAYER);
    if actor == player {
        fn_005cc7a0(e, Ptr::new(player), 0);
        let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
        e.call(SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
        let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
        e.call(SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
    }
    true
}

// Translated from 005cc910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::CanHaveFlamesConditionFunction(thisObj, 0,
/// 0, result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc910(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, CAN_HAVE_FLAMES_CONDITION)
}

// Translated from 005cc940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::HasFlamesConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005cc940(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, HAS_FLAMES_CONDITION)
}

// Translated from 005cc970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddFlamesFunction` (Xbox PDB): when `thisObj` has a 3D object
/// (virtual slot `0x1d0`), asks its node (slot `0xc` of that object, if any)
/// to get its addon nodes (`TESObjectREFR::AddAddonNodes`), stores the
/// answer as the result and, when it is not 0, updates the node's
/// properties and updates it. With the echo flag it prints "AddFlames >>
/// %0.2f". Always true; the result stays untouched without a 3D object.
pub fn script_add_flames_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let this = a.this_obj.addr();
    if e.vcall(this, 0x1d0, &args![]).u32() == 0 {
        return true;
    }
    let object = e.vcall(this, 0x1d0, &args![]).u32();
    let mut node = 0;
    if object != 0 {
        node = e.vcall(object, 0xc, &args![]).u32();
    }
    let added = e.call(ADD_ADDON_NODES, &args![node]).u8();
    e.mem.set_f64(a.result.addr(), added as f64);
    if added != 0 {
        e.call(UPDATE_PROPERTIES, &args![node]);
        e.with_stack(UPDATE_DATA_SIZE, |e, update_data| {
            e.call(
                NI_UPDATE_DATA_CONSTRUCT,
                &args![update_data, 0.0f32, 1u32, 0u32],
            );
            e.call(NODE_UPDATE, &args![node, update_data]);
        });
    }
    if echo_enabled(e) {
        let result = e.mem.f64(a.result.addr());
        console_print(e, &args![FORMAT_ADD_FLAMES, result]);
    }
    true
}

// Translated from 005cca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RemoveFlamesFunction` (Xbox PDB): when `thisObj` has a 3D object
/// (virtual slot `0x1d0`), asks its node (slot `0xc` of that object) to
/// lose its addon nodes (`TESObjectREFR::RemoveAddonNodes`) and stores the
/// answer as the result. With the echo flag it prints "RemoveFlames >>
/// %0.2f" (the result as it stands). Always true.
pub fn script_remove_flames_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let this = a.this_obj.addr();
    if e.vcall(this, 0x1d0, &args![]).u32() == 0 {
        return true;
    }
    let object = e.vcall(this, 0x1d0, &args![]).u32();
    if object != 0 {
        let node = e.vcall(object, 0xc, &args![]).u32();
        let removed = e.call(REMOVE_ADDON_NODES, &args![node]).u8();
        e.mem.set_f64(a.result.addr(), removed as f64);
    }
    if echo_enabled(e) {
        let result = e.mem.f64(a.result.addr());
        console_print(e, &args![FORMAT_REMOVE_FLAMES, result]);
    }
    true
}

// Translated from 005ccaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::GetOpenStateConditionFunction(thisObj, 0, 0,
/// result)`; true without a call when there is no `thisObj`.
pub fn fn_005ccaf0(e: &mut Engine, a: ScriptArgs) -> bool {
    this_condition(e, a, GET_OPEN_STATE_CONDITION)
}

// Translated from 005ccb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::MoveToFunctionBase` (Xbox PDB), `cdecl` (`reference, target,
/// dx, dy, dz`): moves `reference` to the position of `target` plus the
/// offsets. Does nothing when `reference` is the player and its virtual
/// slot `0x22c(0)` answers true, or when either is missing. An actor
/// (slot `0x100`) is first removed from the process lists (`00972840(actor,
/// 0)`) and gotten out of furniture when its process says so (slot
/// `0x4bc`). The target's rotation and cell/world space are read; the new
/// position is the target's (slot `0x1f4`) plus the offsets. The player is
/// moved by a `0x34`-byte position request (world space or interior cell,
/// position, the target's X and Z rotation, the target) given to
/// `PlayerCharacter::RequestPositionPlayer`, then the console is closed.
/// Anything else is placed (`SetLocationOnReference`); when the target's
/// cell is loaded its angle, 3D position and orientation are updated and
/// its collision object is reset, else (for an actor) a far value is set;
/// then the reference moves to the new space, a loaded cell is notified
/// (slot `0x1c4`), the character controller is positioned and an actor's
/// process is stopped, faded in and updated (with an error message when it
/// has none). `PlayerCharacter::CheckForQuestTargetUpdate(reference)` ends
/// every path that moved something. The compiler's exception frame is not
/// translated.
pub fn script_move_to_function_base(
    e: &mut Engine,
    reference: Ptr,
    target: Ptr,
    dx: f32,
    dy: f32,
    dz: f32,
) {
    let reference = reference.addr();
    let target = target.addr();
    let player = e.mem.u32(PLAYER);
    if reference == player && e.vcall(player, 0x22c, &args![0u32]).bool() {
        return;
    }
    if reference == 0 || target == 0 {
        return;
    }
    let is_actor = e.vcall(reference, 0x100, &args![]).bool();
    if is_actor {
        e.call(
            PROCESS_LISTS_FN_00972840,
            &args![PROCESS_LISTS, reference, 0u32],
        );
        let process = saved_acquire_object(e, reference);
        if process != 0 && e.vcall(process, 0x4bc, &args![]).u32() != 0 {
            e.call(GET_OUT_OF_FURNITURE_QUICK, &args![reference]);
        }
    }
    let cell = e.call(REFERENCE_PARENT_CELL, &args![target]).u32();
    let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![target]).u32();
    let rotation_address = e.call(REFERENCE_ROTATION, &args![target]).u32();
    let rotation = [
        e.mem.u32(rotation_address),
        e.mem.u32(rotation_address + 4),
        e.mem.u32(rotation_address + 8),
    ];
    let position_address = e.vcall(target, 0x1f4, &args![]).u32();
    let position = [
        e.mem.u32(position_address),
        e.mem.u32(position_address + 4),
        e.mem.u32(position_address + 8),
    ];
    let new_position = [
        float_sum_bits(position[0], dx),
        float_sum_bits(position[1], dy),
        float_sum_bits(position[2], dz),
    ];
    if reference == player {
        let request = e.call(OPERATOR_NEW, &args![POSITION_REQUEST_SIZE]).u32();
        // The game constructs the request only when the allocation worked
        // and then writes into it without looking.
        let request = if request != 0 {
            e.call(POSITION_REQUEST_CONSTRUCT, &args![request]).u32()
        } else {
            0
        };
        for (i, word) in new_position.iter().enumerate() {
            e.mem.set_u32(request + 8 + 4 * i as u32, *word);
        }
        e.mem.set_u32(request + 0x1c, rotation[2]);
        e.mem.set_u32(request + 0x14, rotation[0]);
        e.mem.set_u8(request + 0x20, 1);
        if world_space != 0 {
            e.mem.set_u32(request, world_space);
        } else if cell != 0 && e.call(CELL_FN_00425FD0, &args![cell]).bool() {
            e.mem.set_u32(request + 4, cell);
        }
        e.mem.set_u32(request + 0x2c, target);
        e.call(REQUEST_POSITION_PLAYER, &args![player, request]);
        e.call(CLOSE_CONSOLE, &args![]);
    } else {
        e.with_stack(12, |e, location| {
            let flag = e.vcall(reference, 0xfc, &args![]).bool();
            for (i, word) in new_position.iter().enumerate() {
                e.mem.set_u32(location.addr() + 4 * i as u32, *word);
            }
            e.call(SET_LOCATION_ON_REFERENCE, &args![reference, location]);
            let tes = e.mem.u32(TES_SINGLETON);
            if cell != 0 && e.call(IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
                e.call(SET_ANGLE_Z, &args![reference, rotation[2]]);
                e.call(SET_FIELD_24, &args![reference, 0.0f32]);
                let node = e.vcall(reference, 0x1d0, &args![]).u32();
                if node != 0 {
                    let position = e.vcall(reference, 0x1f4, &args![]).u32();
                    e.call(SET_TRANSLATE, &args![node, position]);
                    e.with_stack(MATRIX_SIZE, |e, matrix| {
                        let orientation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
                        e.call(SET_ROTATE, &args![node, orientation]);
                    });
                    e.call(RESET_SIM, &args![node, 1u32]);
                    e.with_stack(UPDATE_DATA_SIZE, |e, update_data| {
                        e.call(
                            NI_UPDATE_DATA_CONSTRUCT,
                            &args![update_data, 0.0f32, 0u32, 0u32],
                        );
                        e.call(NODE_UPDATE, &args![node, update_data]);
                    });
                }
            } else if reference != e.mem.u32(PLAYER) && flag {
                let far: u32 = e.global(FLOAT_MAX);
                e.call(SET_FIELD_24, &args![reference, far]);
            }
            e.call(MOVE_REF_TO_NEW_SPACE, &args![reference, cell, world_space]);
            if e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() != 0 {
                let own_cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                let tes = e.mem.u32(TES_SINGLETON);
                if e.call(IS_CELL_LOADED, &args![tes, own_cell, 0u32]).bool() {
                    e.vcall(reference, 0x1c4, &args![]);
                }
            }
            if flag {
                let controller = e.call(GET_CHAR_CONTROLLER, &args![reference]).u32();
                if controller != 0 {
                    e.call(CHAR_CONTROLLER_SET_POSITION, &args![controller, location]);
                }
            }
            if is_actor {
                if saved_acquire_object(e, reference) != 0 {
                    let process = saved_acquire_object(e, reference);
                    e.vcall(process, 0x28, &args![]);
                    e.call(ACTOR_STOP_MOVING, &args![reference]);
                    let process = saved_acquire_object(e, reference);
                    e.vcall(process, 0x24, &args![reference, 1u32]);
                    e.call(ACTOR_FADE_IN, &args![reference]);
                    e.call(ACTOR_UPDATE_ALPHA, &args![reference]);
                } else {
                    e.call(SCRIPT_ERROR, &args![MSG_MOVED_ACTOR_HAS_NO_PROCESS]);
                }
            }
        });
    }
    e.call(CHECK_FOR_QUEST_TARGET_UPDATE, &args![player, reference]);
}

// Translated from 005ccf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::MoveToFunction` (Xbox PDB): parses a target reference and three
/// offsets (see [`move_to_prologue`]; opcode `0x109e` for the message). With
/// a target: when the object at `011c3ea4` says to send messages the move is
/// sent as one, else `Script::MoveToFunctionBase(thisObj, target, offsets)`
/// does it. Always true after the parse.
pub fn script_move_to_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some((target, offsets)) = move_to_prologue(e, a) else {
        return false;
    };
    if target != 0 {
        if move_is_messaged(e) {
            send_move_message(e, 0x109e, a, target, offsets);
        } else {
            let [x, y, z] = offsets.map(f32::from_bits);
            script_move_to_function_base(e, a.this_obj, Ptr::new(target), x, y, z);
        }
    }
    true
}

// Translated from 005cd0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MoveTo` variant that fades an actor out: same parameters and
/// messages as [`script_move_to_function`] (opcode `0x124f`), but when
/// `thisObj` is an actor with a process whose current process type is 0, the
/// process is asked to `HighProcess::FadeOutAndMove(actor, target,
/// offsets)` instead of moving at once. Always true after the parse.
pub fn fn_005cd0e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some((target, offsets)) = move_to_prologue(e, a) else {
        return false;
    };
    if target == 0 {
        return true;
    }
    if move_is_messaged(e) {
        send_move_message(e, 0x124f, a, target, offsets);
        return true;
    }
    let [x, y, z] = offsets.map(f32::from_bits);
    let this = a.this_obj.addr();
    // An actor with a process of type 0 fades out; everything else moves at
    // once.
    let fades = e.vcall(this, 0x100, &args![]).bool()
        && saved_acquire_object(e, this) != 0
        && e.call(GET_CURRENT_PROCESS_TYPE, &args![this]).u32() == 0;
    if fades {
        let process = saved_acquire_object(e, this);
        e.call(
            FADE_OUT_AND_MOVE,
            &args![process, this, target, offsets[0], offsets[1], offsets[2]],
        );
    } else {
        script_move_to_function_base(e, a.this_obj, Ptr::new(target), x, y, z);
    }
    true
}

// Translated from 005cd330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::MoveToQuestTargetFunction` (Xbox PDB): parses an index (preset
/// to 1). With an active quest, takes the second reference of the n-th
/// current target of the player's target list (messages "No current
/// targets" when the list is empty, "No active quest" without a quest) and
/// moves the player to it with `Script::MoveToFunctionBase`. False only when
/// the parameter does not parse.
pub fn script_move_to_quest_target_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([count]) = parse_into(e, a, [1]) else {
        return false;
    };
    let player = e.mem.u32(PLAYER);
    let mut target = 0;
    if fn_005cbb50(e, Ptr::new(player)) != 0 {
        let mut list = e.call(PLAYER_GET_CURRENT_TARGET_LIST, &args![player]).u32();
        if list == 0 {
            console_print(e, &[MSG_NO_CURRENT_TARGETS]);
        }
        let mut index = 0i32;
        while list != 0 {
            let slot = e.call(NODE_DATA_POINTER, &args![list]).u32();
            if e.mem.u32(slot) == 0 || index >= count as i32 {
                break;
            }
            let slot = e.call(NODE_DATA_POINTER, &args![list]).u32();
            let item = e.mem.u32(slot);
            list = e.call(NODE_NEXT, &args![list]).u32();
            target = e.call(QUEST_TARGET_GET_REFERENCE, &args![item, 1u32]).u32();
            index += 1;
        }
    } else {
        console_print(e, &[MSG_NO_ACTIVE_QUEST]);
    }
    if target != 0 {
        script_move_to_function_base(e, Ptr::new(player), Ptr::new(target), 0.0, 0.0, 0.0);
    }
    true
}

// Translated from 005cd440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCurrentPackageFunction` (Xbox PDB): parses a package; true
/// without `thisObj` or without a package, else
/// `Script::GetIsCurrentPackageConditionFunction(thisObj, package, 0,
/// result)`. False when the parameter does not parse.
pub fn script_get_is_current_package_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([package]) = parse_into(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() || package == 0 {
        return true;
    }
    e.call(
        GET_IS_CURRENT_PACKAGE_CONDITION,
        &args![a.this_obj, package, 0u32, a.result],
    )
    .bool()
}

// Translated from 005cd4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that parses a text (a 512-byte stack buffer) and, when
/// `thisObj` is set, hands it to the object returned by `thisObj`'s virtual
/// slot `0x1b8(0)` through that object's slot `0x124(text)`. False only when
/// the text does not parse. The stack protector cookie is not translated.
pub fn fn_005cd4b0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(TEXT_BUFFER_SIZE, |e, buffer| {
        if !parse_text(e, a, buffer) {
            return false;
        }
        if !a.this_obj.is_null() {
            let object = e.vcall(a.this_obj.addr(), 0x1b8, &args![0u32]).u32();
            if object != 0 {
                e.vcall(object, 0x124, &args![buffer]);
            }
        }
        true
    })
}

// Translated from 005cd570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::GetDayOfWeekConditionFunction(0, 0, 0,
/// result)` and answers what it answers.
pub fn fn_005cd570(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_DAY_OF_WEEK_CONDITION,
        &args![0u32, 0u32, 0u32, a.result],
    )
    .bool()
}

// Translated from 005cd590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsPCSleepingConditionFunction(0, 0, 0,
/// result)` and answers what it answers.
pub fn fn_005cd590(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(IS_PC_SLEEPING_CONDITION, &args![0u32, 0u32, 0u32, a.result])
        .bool()
}

// Translated from 005cd5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::WakeUpPCFunction` (Xbox PDB): parses a number of hours. When the
/// player's sleep flag (byte at `+0x658`) is set, the hours (negative ones
/// as 0) are stored with the flag set again and, for 0, the sleep menu is
/// closed. False only when the parameter does not parse.
pub fn script_wake_up_pc_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([hours]) = parse_into(e, a, [0]) else {
        return false;
    };
    let player = e.mem.u32(PLAYER);
    if e.call(PLAYER_GET_SLEEP_FLAG, &args![player]).bool() {
        let hours = (hours as i32).max(0) as u32;
        e.call(PLAYER_SET_SLEEP_HOURS, &args![player, hours, 1u32]);
        if hours == 0 {
            e.call(CLOSE_SLEEP_MENU, &args![]);
        }
    }
    true
}

// Translated from 005cd630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The command that is `Script::IsPCAMurdererConditionFunction(0, 0, 0,
/// result)` and answers what it answers.
pub fn fn_005cd630(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        IS_PC_A_MURDERER_CONDITION,
        &args![0u32, 0u32, 0u32, a.result],
    )
    .bool()
}

// Translated from 005cd650 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that parses one number and, when `thisObj` is an actor, passes
/// it to `008a8010`; if the actor's `GetForceNextUpdate` says so, the
/// actor's base form re-initialises its worn items: a creature (slot
/// `0x21c`) through `TESCreature::InitDefaultWorn(actor, 1, 1, 1)`, any
/// other through `00606540(actor, slot 0x1e8 result, 1)`. False only when
/// the parameter does not parse.
pub fn fn_005cd650(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_into(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 {
        return true;
    }
    e.call(ACTOR_FN_008A8010, &args![actor, value]);
    if !e.call(GET_FORCE_NEXT_UPDATE, &args![actor]).bool() {
        return true;
    }
    if e.vcall(actor, 0x21c, &args![]).bool() {
        let form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![actor]).u32();
        e.call(INIT_DEFAULT_WORN, &args![form, actor, 1u32, 1u32, 1u32]);
    } else {
        let form = e.call(BGS_SAVE_FORM_BUFFER_GET_FORM, &args![actor]).u32();
        let value = e.vcall(actor, 0x1e8, &args![]).u32();
        e.call(NPC_FN_00606540, &args![form, actor, value, 1u32]);
    }
    true
}

// Translated from 005cd750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SelectPlayerSpellFunction` (Xbox PDB): parses a form. A
/// `TESObjectBOOK` is selected through `PlayerCharacter` (`0094c950`); any
/// other form is cast to `SpellItem` and the address of its magic item part
/// (`+0x18`, or 0) becomes the player's selected spell. With the echo flag it
/// prints "Player Spell set to %s" with the spell's name or "NONE". False
/// only when the parameter does not parse.
pub fn script_select_player_spell_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form]) = parse_into(e, a, [0]) else {
        return false;
    };
    if form == 0 {
        return true;
    }
    let player = e.mem.u32(PLAYER);
    let spell = dynamic_cast(e, form, RTTI_TES_FORM, RTTI_SPELL_ITEM);
    let book = dynamic_cast(e, form, RTTI_TES_FORM, RTTI_TES_OBJECT_BOOK);
    if book == 0 {
        let item = if spell != 0 { spell + 0x18 } else { 0 };
        e.call(PLAYER_SET_SELECTED_SPELL, &args![player, item]);
    } else {
        e.call(PLAYER_SELECT_BOOK, &args![player, book]);
    }
    if echo_enabled(e) {
        let name = if spell == 0 {
            TEXT_NONE
        } else {
            e.call(STRING_TEXT_OR_EMPTY, &args![spell + 0x18]).u32()
        };
        console_print(e, &[FORMAT_PLAYER_SPELL_SET, name]);
    }
    true
}

// Translated from 005cd870 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that zeroes the result and parses an actor base and a number;
/// with the base set, its flag bit 2 (`TESActorBaseData` at `+0x30`) is set
/// when the number is above 0, else cleared (see [`fn_005cd8f0`]). False
/// only when the parameters do not parse.
pub fn fn_005cd870(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([base, number]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if base != 0 {
        fn_005cd8f0(e, Ptr::new(base + 0x30), ((number as i32) > 0) as u8);
    }
    true
}

// Translated from 005cd8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on a `TESActorBaseData`: `TESActorBaseData::SetFlagBit(2,
/// value, 1)`.
pub fn fn_005cd8f0(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BIT, &args![this, 2u32, value, 1u32]);
}

// Translated from 005cd910 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that zeroes the result and parses a form and a number; with the
/// form set, calls its virtual slot `0xcc` with whether the number is above
/// 0. False only when the parameters do not parse.
pub fn fn_005cd910(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([form, number]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if form != 0 {
        e.vcall(form, 0xcc, &args![((number as i32) > 0) as u32]);
    }
    true
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005c8450, script_get_disposition(ScriptArgs) -> bool),
        entry!(0x005c84b0, fn_005c84b0(ScriptArgs) -> bool),
        entry!(0x005c84d0, fn_005c84d0(ScriptArgs) -> bool),
        entry!(0x005c84f0, fn_005c84f0(ScriptArgs) -> bool),
        entry!(0x005c8510, script_mod_disposition(ScriptArgs) -> bool),
        entry!(0x005c85c0, script_get_dead_count(ScriptArgs) -> bool),
        entry!(0x005c8620, script_show_map(ScriptArgs) -> bool),
        entry!(0x005c9670, script_set_alert_function(ScriptArgs) -> bool),
        entry!(0x005c9700, fn_005c9700(ScriptArgs) -> bool),
        entry!(0x005c9720, fn_005c9720(ScriptArgs) -> bool),
        entry!(0x005c9740, fn_005c9740(ScriptArgs) -> bool),
        entry!(0x005c9790, fn_005c9790(ScriptArgs) -> bool),
        entry!(0x005c98e0, fn_005c98e0(ScriptArgs) -> bool),
        entry!(0x005c9980, fn_005c9980(ScriptArgs) -> bool),
        entry!(0x005c9a00, script_print_hdr_param_function() -> bool),
        entry!(0x005c9b30, fn_005c9b30(Ptr) -> f32),
        entry!(0x005c9b50, fn_005c9b50(Ptr) -> f32),
        entry!(0x005c9b70, fn_005c9b70(Ptr) -> f32),
        entry!(0x005c9b90, fn_005c9b90(Ptr) -> f32),
        entry!(0x005c9bb0, fn_005c9bb0(Ptr) -> f32),
        entry!(0x005c9bd0, fn_005c9bd0(Ptr) -> f32),
        entry!(0x005c9bf0, fn_005c9bf0(Ptr) -> f32),
        entry!(0x005c9c10, fn_005c9c10(Ptr) -> f32),
        entry!(0x005c9c30, fn_005c9c30(Ptr) -> f32),
        entry!(0x005c9c50, fn_005c9c50(Ptr) -> f32),
        entry!(0x005c9c70, fn_005c9c70(Ptr) -> f32),
        entry!(0x005c9c90, fn_005c9c90(Ptr) -> f32),
        entry!(0x005c9cb0, fn_005c9cb0(Ptr) -> f32),
        entry!(0x005c9cd0, fn_005c9cd0(Ptr) -> f32),
        entry!(0x005c9cf0, fn_005c9cf0(ScriptArgs) -> bool),
        entry!(0x005ca1c0, fn_005ca1c0(u32, Ptr, u32, u32, Ptr)),
        entry!(0x005ca200, fn_005ca200(ScriptArgs) -> bool),
        entry!(0x005ca260, fn_005ca260(ScriptArgs) -> bool),
        entry!(0x005ca2d0, fn_005ca2d0(ScriptArgs) -> bool),
        entry!(0x005ca330, fn_005ca330(ScriptArgs) -> bool),
        entry!(0x005ca390, fn_005ca390(ScriptArgs) -> bool),
        entry!(0x005ca3f0, script_on_activate_function(ScriptArgs) -> bool),
        entry!(0x005ca470, fn_005ca470(ScriptArgs) -> bool),
        entry!(0x005ca4f0, fn_005ca4f0(Ptr) -> bool),
        entry!(0x005ca510, fn_005ca510(ScriptArgs) -> bool),
        entry!(0x005ca540, fn_005ca540(ScriptArgs) -> bool),
        entry!(0x005ca570, fn_005ca570(ScriptArgs) -> bool),
        entry!(0x005ca5a0, fn_005ca5a0(ScriptArgs) -> bool),
        entry!(0x005ca5d0, fn_005ca5d0(ScriptArgs) -> bool),
        entry!(0x005ca600, fn_005ca600(ScriptArgs) -> bool),
        entry!(0x005ca670, fn_005ca670(ScriptArgs) -> bool),
        entry!(0x005ca6e0, fn_005ca6e0(ScriptArgs) -> bool),
        entry!(0x005ca800, fn_005ca800(ScriptArgs) -> bool),
        entry!(0x005ca860, fn_005ca860(ScriptArgs) -> bool),
        entry!(0x005ca8d0, fn_005ca8d0(ScriptArgs) -> bool),
        entry!(0x005ca930, fn_005ca930(ScriptArgs) -> bool),
        entry!(0x005ca950, fn_005ca950(ScriptArgs) -> bool),
        entry!(0x005ca9b0, fn_005ca9b0(ScriptArgs) -> bool),
        entry!(0x005caa20, fn_005caa20(ScriptArgs) -> bool),
        entry!(0x005caa90, fn_005caa90(ScriptArgs) -> bool),
        entry!(0x005cab00, fn_005cab00(ScriptArgs) -> bool),
        entry!(0x005cab30, fn_005cab30(ScriptArgs) -> bool),
        entry!(0x005cab60, fn_005cab60(ScriptArgs) -> bool),
        entry!(0x005cabd0, fn_005cabd0(ScriptArgs) -> bool),
        entry!(0x005cac40, fn_005cac40(ScriptArgs) -> bool),
        entry!(0x005cacb0, fn_005cacb0(ScriptArgs) -> bool),
        entry!(0x005cad20, fn_005cad20(ScriptArgs) -> bool),
        entry!(0x005cae30, fn_005cae30(ScriptArgs) -> bool),
        entry!(0x005caf40, script_get_player_controls_disabled_function(ScriptArgs) -> bool),
        entry!(0x005cb050, script_get_heading_angle_function(ScriptArgs) -> bool),
        entry!(0x005cb0c0, script_pick_idle_function(ScriptArgs) -> bool),
        entry!(0x005cb2d0, script_play_idle_function(ScriptArgs) -> bool),
        entry!(0x005cb580, script_play_vats_cameras_function(ScriptArgs) -> bool),
        entry!(0x005cb660, fn_005cb660(ScriptArgs) -> bool),
        entry!(0x005cb690, script_is_weapon_in_list_function(ScriptArgs) -> bool),
        entry!(0x005cb6f0, fn_005cb6f0(ScriptArgs) -> bool),
        entry!(0x005cb720, fn_005cb720(ScriptArgs) -> bool),
        entry!(0x005cb750, fn_005cb750(ScriptArgs) -> bool),
        entry!(0x005cb780, fn_005cb780(ScriptArgs) -> bool),
        entry!(0x005cb7b0, fn_005cb7b0(ScriptArgs) -> bool),
        entry!(0x005cb7e0, script_is_weapon_skill_type_function(ScriptArgs) -> bool),
        entry!(0x005cb850, fn_005cb850(ScriptArgs) -> bool),
        entry!(0x005cb880, fn_005cb880(ScriptArgs) -> bool),
        entry!(0x005cb8b0, fn_005cb8b0(ScriptArgs) -> bool),
        entry!(0x005cb8e0, script_show_quest_targets_function(ScriptArgs) -> bool),
        entry!(0x005cbb50, fn_005cbb50(Ptr) -> u32),
        entry!(0x005cbb70, fn_005cbb70(Ptr) -> u32),
        entry!(0x005cbbb0, fn_005cbbb0(ScriptArgs) -> bool),
        entry!(0x005cbd90, fn_005cbd90(u32, u32, u32) -> u32),
        entry!(0x005cbdb0, fn_005cbdb0(ScriptArgs) -> bool),
        entry!(0x005cbf80, script_lock_function(ScriptArgs) -> bool),
        entry!(0x005cc120, script_unlock_function(ScriptArgs) -> bool),
        entry!(0x005cc240, fn_005cc240(ScriptArgs) -> bool),
        entry!(0x005cc2f0, script_get_crime_function(ScriptArgs) -> bool),
        entry!(0x005cc370, fn_005cc370(ScriptArgs) -> bool),
        entry!(0x005cc3a0, fn_005cc3a0(ScriptArgs) -> bool),
        entry!(0x005cc3d0, fn_005cc3d0(ScriptArgs) -> bool),
        entry!(0x005cc400, fn_005cc400(ScriptArgs) -> bool),
        entry!(0x005cc430, fn_005cc430(ScriptArgs) -> bool),
        entry!(0x005cc460, fn_005cc460(ScriptArgs) -> bool),
        entry!(0x005cc490, fn_005cc490(ScriptArgs) -> bool),
        entry!(0x005cc4c0, fn_005cc4c0(ScriptArgs) -> bool),
        entry!(0x005cc4f0, fn_005cc4f0(ScriptArgs) -> bool),
        entry!(0x005cc7a0, fn_005cc7a0(Ptr, u8)),
        entry!(0x005cc7c0, fn_005cc7c0(ScriptArgs) -> bool),
        entry!(0x005cc910, fn_005cc910(ScriptArgs) -> bool),
        entry!(0x005cc940, fn_005cc940(ScriptArgs) -> bool),
        entry!(0x005cc970, script_add_flames_function(ScriptArgs) -> bool),
        entry!(0x005cca50, script_remove_flames_function(ScriptArgs) -> bool),
        entry!(0x005ccaf0, fn_005ccaf0(ScriptArgs) -> bool),
        entry!(
            0x005ccb20,
            script_move_to_function_base(Ptr, Ptr, f32, f32, f32)
        ),
        entry!(0x005ccf50, script_move_to_function(ScriptArgs) -> bool),
        entry!(0x005cd0e0, fn_005cd0e0(ScriptArgs) -> bool),
        entry!(0x005cd330, script_move_to_quest_target_function(ScriptArgs) -> bool),
        entry!(0x005cd440, script_get_is_current_package_function(ScriptArgs) -> bool),
        entry!(0x005cd4b0, fn_005cd4b0(ScriptArgs) -> bool),
        entry!(0x005cd570, fn_005cd570(ScriptArgs) -> bool),
        entry!(0x005cd590, fn_005cd590(ScriptArgs) -> bool),
        entry!(0x005cd5b0, script_wake_up_pc_function(ScriptArgs) -> bool),
        entry!(0x005cd630, fn_005cd630(ScriptArgs) -> bool),
        entry!(0x005cd650, fn_005cd650(ScriptArgs) -> bool),
        entry!(0x005cd750, script_select_player_spell_function(ScriptArgs) -> bool),
        entry!(0x005cd870, fn_005cd870(ScriptArgs) -> bool),
        entry!(0x005cd8f0, fn_005cd8f0(Ptr, u8)),
        entry!(0x005cd910, fn_005cd910(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    /// Returns the address `object + 0xe4`.
    const V_ADDRESS: u32 = 0x0900_0002;

    /// An engine with the pages of the globals these commands touch mapped
    /// and the dynamic cast replaced by a double: an object is an actor when
    /// its byte at `+0xff` is 1.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x011a_d000,
            0x011d_e000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[2..], [RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]);
            let actor = a[0] != 0 && e.mem.u8(a[0] + 0xff) == 1;
            (if actor { a[0] } else { 0 }).into_ret()
        });
        e.register(V_ADDRESS, |_, a| (a[0] + 0xe4).into_ret());
        e
    }

    /// An object whose vtable (in the heap) has the given `(byte offset,
    /// function)` slots; `actor` marks it for the cast double.
    fn object_with(e: &mut Engine, actor: bool, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        e.mem.set_u8(object + 0xff, actor as u8);
        object
    }

    /// The standard eight words with `this_obj` and `result` set (the result
    /// is a heap `double` preset to 7.0).
    fn script(e: &mut Engine, this_obj: u32) -> ScriptArgs {
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 7.0);
        ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: Ptr::new(this_obj),
            containing_obj: Ptr::new(4),
            script_obj: Ptr::new(5),
            event_list: 6,
            result: Ptr::new(result),
            opcode_offset: 8,
        }
    }

    /// `ParseParameters` double: returns `ok` and stores `outs` through its
    /// output pointers (after the seven fixed words).
    fn parse_gives(e: &mut Engine, ok: bool, outs: &[u32]) {
        let outs = outs.to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, value) in outs.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            ok.into_ret()
        });
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Registers doubles that just accept the call.
    fn accept(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// The command-body entry for `addr` called with `a`.
    fn run(e: &mut Engine, addr: u32, a: ScriptArgs) -> bool {
        e.call(addr, &args![a]).bool()
    }

    fn result_of(e: &Engine, a: ScriptArgs) -> f64 {
        e.mem.f64(a.result.addr())
    }

    // ---- the condition-function twins --------------------------------------

    /// A command that only forwards to `callee(thisObj, 0, 0, result)`.
    fn check_plain_condition(addr: u32, callee: u32) {
        let mut e = engine();
        e.register(callee, |_, a| (a[0] == 0x40).into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        assert_eq!(calls(&e, callee), vec![vec![0x40, 0, 0, a.result.addr()]]);
        // The callee's answer is the command's answer.
        let other = script(&mut e, 0x41);
        assert!(!run(&mut e, addr, other));
    }

    /// A command that parses one argument and forwards to `callee(thisObj,
    /// argument, 0, result)`.
    fn check_one_argument_condition(addr: u32, callee: u32) {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x77]);
        e.register(callee, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        // ParseParameters gets: info, data, opcode offset, thisObj,
        // containing, script, event list, then the address of the local.
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x40, 4, 5, 6]
        );
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x40, 0x77, 0, a.result.addr()]]
        );
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, addr, a));
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn get_disposition_passes_the_parsed_actor() {
        check_one_argument_condition(0x005c_8450, GET_DISPOSITION_CONDITION);
    }

    #[test]
    fn get_dead_count_passes_the_parsed_form() {
        check_one_argument_condition(0x005c_85c0, GET_DEAD_COUNT_CONDITION);
    }

    #[test]
    fn get_random_percent_forwards_this_obj_and_result() {
        check_plain_condition(0x005c_84b0, GET_RANDOM_PERCENT_CONDITION);
    }

    #[test]
    fn get_level_forwards_this_obj_and_result() {
        check_plain_condition(0x005c_84d0, GET_LEVEL_CONDITION);
    }

    #[test]
    fn get_armor_rating_forwards_this_obj_and_result() {
        check_plain_condition(0x005c_84f0, GET_ARMOR_RATING_CONDITION);
    }

    #[test]
    fn get_alert_forwards_this_obj_and_result() {
        check_plain_condition(0x005c_9700, GET_ALERT_CONDITION);
    }

    #[test]
    fn the_005a4340_command_forwards_this_obj_and_result() {
        check_plain_condition(0x005c_9720, FN_005A4340);
    }

    #[test]
    fn mod_disposition_changes_the_actor_then_reads_it_back() {
        let mut e = engine();
        // The actor argument is 0x77, the amount -5.
        parse_gives(&mut e, true, &[0x77, (-5i32) as u32]);
        e.register(GET_DISPOSITION_CONDITION, |_, _| true.into_ret());
        e.register(0x0900_0100, |_, _| Ret::default());
        let actor = object_with(&mut e, true, &[(0x460, 0x0900_0100)]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_8510, a));
        // Slot 0x460 gets the argument and the amount as a float.
        assert_eq!(
            calls(&e, 0x0900_0100),
            vec![vec![actor, 0x77, (-5.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&e, GET_DISPOSITION_CONDITION),
            vec![vec![actor, 0x77, 0, a.result.addr()]]
        );

        // Not an actor: false, no slot call.
        let thing = object_with(&mut e, false, &[(0x460, 0x0900_0100)]);
        let a = script(&mut e, thing);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_8510, a));
        assert!(calls(&e, 0x0900_0100).is_empty());

        // No actor argument: false.
        parse_gives(&mut e, true, &[0, 3]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_8510, a));
        assert!(calls(&e, 0x0900_0100).is_empty());

        // Bad parameters: false, no cast.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_8510, a));
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
    }

    // ---- map markers ---------------------------------------------------------

    /// A marker reference whose map marker data (at `+0xd0`, bytes: `+0`
    /// visible, `+1` has a travel location) is as given, with slot `0x48`
    /// recorded at `0x0900_0100`.
    fn marker_ref(e: &mut Engine, visible: bool, travel: bool) -> u32 {
        let marker = object_with(e, false, &[(0x48, 0x0900_0100)]);
        e.mem.set_u8(marker + 0xd0, visible as u8);
        e.mem.set_u8(marker + 0xd1, travel as u8);
        marker
    }

    /// Doubles for the map marker data accessors; the reference `0x66` has
    /// no map marker data.
    fn map_engine() -> Engine {
        let mut e = engine();
        e.register(GET_MAP_MARKER_DATA, |_, a| {
            (if a[0] == 0x66 { 0 } else { a[0] + 0xd0 }).into_ret()
        });
        e.register(MAP_MARKER_GET_VISIBLE, |e, a| {
            (e.mem.u8(a[0]) != 0).into_ret()
        });
        e.register(MAP_MARKER_GET_TRAVEL_LOC, |e, a| {
            (e.mem.u8(a[0] + 1) != 0).into_ret()
        });
        e.register(MAP_MARKER_SET_VISIBLE, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        e.register(MAP_MARKER_SET_TRAVEL_LOC, |e, a| {
            e.mem.set_u8(a[0] + 1, a[1] as u8);
            Ret::default()
        });
        e.register(0x0900_0100, |_, _| Ret::default());
        e.register(BS_STRING_TEXT, |_, a| (a[0] + 1).into_ret());
        accept(&mut e, &[SHOW_MESSAGE]);
        e.set_global(MESSAGE_VOLUME, 2.0f32);
        e
    }

    #[test]
    fn show_map_makes_a_hidden_marker_visible_and_announces_it() {
        let mut e = map_engine();
        let marker = marker_ref(&mut e, false, false);
        parse_gives(&mut e, true, &[marker, 1]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_8620, a));
        // Visible and with a travel location afterwards.
        assert_eq!(e.mem.u8(marker + 0xd0), 1);
        assert_eq!(e.mem.u8(marker + 0xd1), 1);
        assert_eq!(calls(&e, 0x0900_0100), vec![vec![marker, 0x8000_0000]]);
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![
                MAP_MARKER_MESSAGE + 1,
                0,
                ICON_MAP,
                SOUND_MAP_MARKER_ADDED,
                2.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(result_of(&e, a), 1.0);
    }

    #[test]
    fn show_map_announces_only_what_changed() {
        let mut e = map_engine();
        // Already visible, travel location already set: silent.
        let marker = marker_ref(&mut e, true, true);
        parse_gives(&mut e, true, &[marker, 1]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_8620, a));
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // Visible but no travel location: without the flag it is left alone
        // and nothing is announced; with the flag it is set and announced.
        let marker = marker_ref(&mut e, true, false);
        parse_gives(&mut e, true, &[marker, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_8620, a));
        assert_eq!(e.mem.u8(marker + 0xd1), 0);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        parse_gives(&mut e, true, &[marker, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_8620, a));
        assert_eq!(e.mem.u8(marker + 0xd1), 1);
        assert_eq!(calls(&e, SHOW_MESSAGE).len(), 1);
    }

    #[test]
    fn show_map_without_marker_data_or_reference_only_sets_the_result() {
        let mut e = map_engine();
        for reference in [0x66u32, 0] {
            parse_gives(&mut e, true, &[reference, 1]);
            let a = script(&mut e, 0);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_8620, a));
            assert!(calls(&e, MAP_MARKER_SET_VISIBLE).is_empty());
            assert!(calls(&e, SHOW_MESSAGE).is_empty());
            assert_eq!(result_of(&e, a), 1.0);
        }
        // Bad parameters: false and the result untouched.
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, 0);
        assert!(!run(&mut e, 0x005c_8620, a));
        assert_eq!(result_of(&e, a), 7.0);
    }

    // ---- actors ------------------------------------------------------------------

    #[test]
    fn set_alert_passes_whether_the_integer_is_positive() {
        let mut e = engine();
        accept(&mut e, &[ACTOR_SET_ALERT]);
        let actor = object_with(&mut e, true, &[]);
        for (value, alert) in [(3i32, 1), (0, 0), (-1, 0)] {
            parse_gives(&mut e, true, &[value as u32]);
            let a = script(&mut e, actor);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_9670, a));
            assert_eq!(calls(&e, ACTOR_SET_ALERT), vec![vec![actor, alert]]);
        }
        // Not an actor, or no reference: nothing set, still true.
        parse_gives(&mut e, true, &[1]);
        let thing = object_with(&mut e, false, &[]);
        for this_obj in [thing, 0] {
            let a = script(&mut e, this_obj);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_9670, a));
            assert!(calls(&e, ACTOR_SET_ALERT).is_empty());
        }
        // Bad parameters: false.
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, actor);
        assert!(!run(&mut e, 0x005c_9670, a));
    }

    #[test]
    fn the_005c9740_command_calls_the_parsed_object() {
        let mut e = engine();
        accept(&mut e, &[FN_0060C950]);
        parse_gives(&mut e, true, &[0x1234]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9740, a));
        assert_eq!(calls(&e, FN_0060C950), vec![vec![0x1234, 1]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_9740, a));
        assert!(calls(&e, FN_0060C950).is_empty());
    }

    /// Doubles for the process/extra data callees of the target commands:
    /// returns the engine, an actor with a process and the process. The
    /// actor's slot `0x48` is `0x0900_0100`; the process's slots `0x62c`,
    /// `0x648` and `0x7ac` are `0x0900_0101` to `0x0900_0103`.
    fn target_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        accept(
            &mut e,
            &[
                SET_ITEM_DROPPER,
                SET_TARGETED,
                ACTOR_SET_LOOK_AT_TARGET,
                ACTOR_CLEAR_LOOK_AT_TARGET,
                0x0900_0100,
                0x0900_0101,
                0x0900_0102,
                0x0900_0103,
            ],
        );
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        let process = object_with(
            &mut e,
            false,
            &[
                (0x62c, 0x0900_0101),
                (0x648, 0x0900_0102),
                (0x7ac, 0x0900_0103),
            ],
        );
        // Actors flagged at `+0xfe` have a process.
        e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |e, a| {
            (if e.mem.u8(a[0] + 0xfe) != 0 {
                process
            } else {
                0
            })
            .into_ret()
        });
        let actor = object_with(&mut e, true, &[(0x48, 0x0900_0100)]);
        e.mem.set_u8(actor + 0xfe, 1);
        (e, actor, process)
    }

    #[test]
    fn target_command_without_the_flag_sets_the_item_dropper() {
        let (mut e, actor, process) = target_engine();
        let target = object_with(&mut e, false, &[]);
        parse_gives(&mut e, true, &[target, 0]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9790, a));
        assert_eq!(calls(&e, 0x0900_0101), vec![vec![process, target]]);
        assert_eq!(
            calls(&e, SET_ITEM_DROPPER),
            vec![vec![actor + 0x44, target]]
        );
        assert!(calls(&e, ACTOR_SET_LOOK_AT_TARGET).is_empty());
        assert_eq!(calls(&e, 0x0900_0100), vec![vec![actor, 0x8000_0000]]);
    }

    #[test]
    fn target_command_with_the_flag_looks_at_the_target() {
        let (mut e, actor, process) = target_engine();
        // The target's position (slot 0x1f4) is three floats.
        let target = object_with(&mut e, false, &[(0x1f4, V_ADDRESS)]);
        e.mem.set_f32(target + 0xe4, 1.0);
        e.mem.set_f32(target + 0xe8, 2.0);
        e.mem.set_f32(target + 0xec, 3.0);
        parse_gives(&mut e, true, &[target, 1]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9790, a));
        assert_eq!(
            calls(&e, ACTOR_SET_LOOK_AT_TARGET),
            vec![vec![
                actor,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, 0x0900_0103), vec![vec![process, target]]);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![target, 1]]);
        assert!(calls(&e, SET_ITEM_DROPPER).is_empty());
        assert_eq!(calls(&e, 0x0900_0100), vec![vec![actor, 0x8000_0000]]);
    }

    #[test]
    fn target_command_needs_an_actor_with_a_process_and_a_target() {
        let (mut e, actor, _) = target_engine();
        let target = object_with(&mut e, false, &[]);
        let idle = object_with(&mut e, true, &[(0x48, 0x0900_0100)]);
        let thing = object_with(&mut e, false, &[]);
        for (this_obj, argument) in [(idle, target), (thing, target), (0, target), (actor, 0)] {
            parse_gives(&mut e, true, &[argument, 0]);
            let a = script(&mut e, this_obj);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_9790, a));
            assert!(calls(&e, 0x0900_0100).is_empty());
            assert!(calls(&e, SET_ITEM_DROPPER).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, actor);
        assert!(!run(&mut e, 0x005c_9790, a));
    }

    #[test]
    fn release_command_clears_the_target_state_of_an_actor_with_a_process() {
        let (mut e, actor, process) = target_engine();
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_98e0, a));
        let wanted = [
            SET_ITEM_DROPPER,
            0x0900_0102,
            ACTOR_CLEAR_LOOK_AT_TARGET,
            0x0900_0103,
        ];
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .filter(|addr| wanted.contains(addr))
            .collect();
        assert_eq!(order, wanted);
        assert_eq!(calls(&e, SET_ITEM_DROPPER), vec![vec![actor + 0x44, 0]]);
        assert_eq!(calls(&e, 0x0900_0102), vec![vec![process, 1]]);
        assert_eq!(calls(&e, ACTOR_CLEAR_LOOK_AT_TARGET), vec![vec![actor]]);
        assert_eq!(calls(&e, 0x0900_0103), vec![vec![process, 0]]);

        // An actor without a process only loses its item dropper; other
        // references, and none, do nothing.
        let idle = object_with(&mut e, true, &[]);
        let thing = object_with(&mut e, false, &[]);
        let a = script(&mut e, idle);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_98e0, a));
        assert_eq!(calls(&e, SET_ITEM_DROPPER).len(), 1);
        assert!(calls(&e, ACTOR_CLEAR_LOOK_AT_TARGET).is_empty());
        for this_obj in [thing, 0] {
            let a = script(&mut e, this_obj);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_98e0, a));
            assert!(calls(&e, SET_ITEM_DROPPER).is_empty());
        }
    }

    // ---- HDR ------------------------------------------------------------------------

    #[test]
    fn set_hdr_values_stores_the_absolute_values() {
        let mut e = engine();
        e.register(FLOAT_ABS, |_, a| f32::from_bits(a[0]).abs().into_ret());
        parse_gives(&mut e, true, &[(-1.5f32).to_bits(), 2.5f32.to_bits()]);
        let a = script(&mut e, 0);
        assert!(run(&mut e, 0x005c_9980, a));
        assert_eq!(e.global::<f32>(HDR_SUNLIGHT_DIMMER), 1.5);
        assert_eq!(e.global::<f32>(HDR_LUM_RAMP), 2.5);
        // Bad parameters: false and the globals untouched.
        e.set_global(HDR_SUNLIGHT_DIMMER, 9.0f32);
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_9980, a));
        assert_eq!(e.global::<f32>(HDR_SUNLIGHT_DIMMER), 9.0);
    }

    /// An image space manager double: effect 1 is an HDR effect whose
    /// `pfData` (at `+0x6c`) holds the numbers 10.0, 11.0, ... 25.0, so that
    /// index `i` reads `10 + i`.
    fn hdr_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        let effect = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_f32(data + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(effect + 0x6c, data);
        let manager = e.mem.alloc(0x10);
        e.register_double(GET_IMAGE_SPACE_MANAGER, move |_, _| manager.into_ret());
        e.register_double(IMAGE_SPACE_GET_EFFECT, move |_, a| {
            assert_eq!((a[0], a[1]), (manager, 1));
            effect.into_ret()
        });
        (e, manager, effect)
    }

    /// `via_manager` reads index `index` of the HDR data through the
    /// manager; `direct` reads it from the effect.
    fn check_hdr_accessors(via_manager: u32, direct: u32, index: u32) {
        let (mut e, manager, effect) = hdr_engine();
        let expected = 10.0 + index as f32;
        assert_eq!(e.call(direct, &args![effect]).f32(), expected);
        assert_eq!(e.call(via_manager, &args![manager]).f32(), expected);
    }

    #[test]
    fn emissive_mult_accessors_read_index_3() {
        check_hdr_accessors(0x005c_9b30, 0x005c_9b50, 3);
    }

    #[test]
    fn blur_radius_accessors_read_index_1() {
        check_hdr_accessors(0x005c_9b70, 0x005c_9b90, 1);
    }

    #[test]
    fn bright_clamp_accessors_read_index_7() {
        check_hdr_accessors(0x005c_9bb0, 0x005c_9bd0, 7);
    }

    #[test]
    fn bright_scale_accessors_read_index_6() {
        check_hdr_accessors(0x005c_9bf0, 0x005c_9c10, 6);
    }

    #[test]
    fn eye_adapt_speed_accessors_read_index_0() {
        check_hdr_accessors(0x005c_9c30, 0x005c_9c50, 0);
    }

    #[test]
    fn target_lum_accessors_read_index_4() {
        check_hdr_accessors(0x005c_9c70, 0x005c_9c90, 4);
    }

    #[test]
    fn upper_lum_clamp_accessors_read_index_5() {
        check_hdr_accessors(0x005c_9cb0, 0x005c_9cd0, 5);
    }

    /// The `double`s in a list of argument words.
    fn doubles(words: &[u32]) -> Vec<f64> {
        words
            .chunks(2)
            .map(|pair| f64::from_bits(pair[0] as u64 | (pair[1] as u64) << 32))
            .collect()
    }

    #[test]
    fn print_hdr_params_prints_the_lines_in_order() {
        let (mut e, _, _) = hdr_engine();
        e.set_global(HDR_SUNLIGHT_DIMMER, 0.5f32);
        e.set_global(HDR_LUM_RAMP, 0.25f32);
        e.set_global(HDR_GRASS_DIMMER, 0.75f32);
        e.set_global(HDR_TREE_DIMMER, 0.125f32);
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005c_9a00, &args![]).bool());
        let prints = calls(&e, CONSOLE_PRINT);
        let formats: Vec<u32> = prints.iter().map(|words| words[0]).collect();
        assert_eq!(
            formats,
            vec![
                MSG_HDR_PARAMS,
                MSG_SISG,
                FORMAT_BLUR,
                FORMAT_BRIGHT,
                MSG_SSP,
                FORMAT_SUNLIGHT,
                MSG_SHP,
                FORMAT_EYE_ADAPT,
                FORMAT_DIMMERS,
                FORMAT_LUM
            ]
        );
        // The values (index i of the HDR data is 10 + i): blur radius (1);
        // bright clamp (7) and scale (6); sunlight dimmer and ramp; eye
        // adapt speed (0) and emissive multiplier (3); tree and grass
        // dimmers; upper luminance clamp (5) and target (4).
        assert_eq!(doubles(&prints[2][1..]), vec![11.0]);
        assert_eq!(doubles(&prints[3][1..]), vec![17.0, 16.0]);
        assert_eq!(doubles(&prints[5][1..]), vec![0.5, 0.25]);
        assert_eq!(doubles(&prints[7][1..]), vec![10.0, 13.0]);
        assert_eq!(doubles(&prints[8][1..]), vec![0.125, 0.75]);
        assert_eq!(doubles(&prints[9][1..]), vec![15.0, 14.0]);
    }

    // ---- debug axes -------------------------------------------------------------------

    /// Doubles for the callees of `005c9cf0`. The reference has a 3D when
    /// `bound_radius` is given (slot `0x1d0`), and its position (slot
    /// `0x1f4`) is at `reference + 0xe4`.
    fn axes_engine(bound_radius: Option<f32>) -> (Engine, u32) {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        accept(
            &mut e,
            &[
                VECTOR_CONSTRUCT_ITERATOR,
                SET_TRANSLATE,
                ADD_TEMP_DEBUG_OBJECT,
            ],
        );
        e.register(NI_COLOR_A_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(NI_POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(NI_LINES_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(GET_WORLD_BOUND, |_, a| (a[0] + 0x20).into_ret());
        e.register(BOUND_RADIUS, |e, a| e.mem.f32(a[0] + 0xc).into_ret());
        let node = e.mem.alloc(0x40);
        if let Some(radius) = bound_radius {
            e.mem.set_f32(node + 0x20 + 0xc, radius);
        }
        let has_3d = bound_radius.is_some();
        e.register_double(0x0900_0200, move |_, _| {
            (if has_3d { node } else { 0 }).into_ret()
        });
        e.set_global(DEFAULT_AXIS_LENGTH, 10.0f32);
        e.set_global(DEBUG_OBJECT_SECONDS, 20.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(TES_SINGLETON, 0x5555u32);
        let reference = object_with(&mut e, false, &[(0x1d0, 0x0900_0200), (0x1f4, V_ADDRESS)]);
        (e, reference)
    }

    /// The lines the command made: its vertices, colors, connections and
    /// the `NiLines` object.
    fn lines_made(e: &Engine) -> ([f32; 18], [[f32; 4]; 6], Vec<u8>, u32) {
        let constructed = calls(e, NI_LINES_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        let words = &constructed[0];
        // storage, count, vertices, colors, texture coordinates, texture
        // sets, NBT method, connections.
        assert_eq!(words[1], 6);
        assert_eq!(words[4..7], [0, 1, 0]);
        let mut vertices = [0.0; 18];
        for (i, v) in vertices.iter_mut().enumerate() {
            *v = e.mem.f32(words[2] + 4 * i as u32);
        }
        let mut colors = [[0.0; 4]; 6];
        for (i, color) in colors.iter_mut().enumerate() {
            for (j, c) in color.iter_mut().enumerate() {
                *c = e.mem.f32(words[3] + 16 * i as u32 + 4 * j as u32);
            }
        }
        let connections = (0..6).map(|i| e.mem.u8(words[7] + i)).collect();
        (vertices, colors, connections, words[0])
    }

    #[test]
    fn debug_axes_use_the_default_length_when_the_bound_is_large() {
        let (mut e, reference) = axes_engine(Some(100.0));
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9cf0, a));
        let (vertices, colors, connections, lines) = lines_made(&e);
        assert_eq!(
            vertices,
            [
                -10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, -10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, -10.0,
                0.0, 0.0, 10.0
            ]
        );
        assert_eq!(colors, [[1.0, 1.0, 0.0, 1.0]; 6]);
        assert_eq!(connections, vec![1, 0, 1, 0, 1, 0]);
        // Placed at the reference's position and kept for 20 seconds.
        assert_eq!(
            calls(&e, SET_TRANSLATE),
            vec![vec![lines, reference + 0xe4]]
        );
        assert_eq!(
            calls(&e, ADD_TEMP_DEBUG_OBJECT),
            vec![vec![0x5555, lines, 20.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, VECTOR_CONSTRUCT_ITERATOR).len(), 2);
    }

    #[test]
    fn debug_axes_shrink_to_half_the_bound_radius() {
        let (mut e, reference) = axes_engine(Some(8.0));
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9cf0, a));
        let (vertices, ..) = lines_made(&e);
        assert_eq!(vertices[..6], [-4.0, 0.0, 0.0, 4.0, 0.0, 0.0]);
        assert_eq!(vertices[12..], [0.0, 0.0, -4.0, 0.0, 0.0, 4.0]);
        // A bound exactly as large as the default keeps the default.
        let (mut e, reference) = axes_engine(Some(20.0));
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9cf0, a));
        assert_eq!(lines_made(&e).0[0], -10.0);
    }

    #[test]
    fn debug_axes_without_3d_or_reference() {
        let (mut e, reference) = axes_engine(None);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9cf0, a));
        assert_eq!(lines_made(&e).0[0], -10.0);
        assert!(calls(&e, GET_WORLD_BOUND).is_empty());
        // No reference: nothing made.
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_9cf0, a));
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert!(calls(&e, ADD_TEMP_DEBUG_OBJECT).is_empty());
    }

    // ---- script event flags ----------------------------------------------------------

    #[test]
    fn event_flag_body_asks_the_event_list_for_the_event_and_mask() {
        let mut e = engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, a| {
            (a[1] == 0x77).into_ret()
        });
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        e.call(
            0x005c_a1c0,
            &args![0x20u32, 0x40u32, 0x77u32, 0x99u32, a.result],
        );
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![0x99, 0x77, 0x20]]
        );
        assert_eq!(result_of(&e, a), 1.0);
        // Event not in the list: 0.0.
        e.call(
            0x005c_a1c0,
            &args![0x20u32, 0x40u32, 0x78u32, 0x99u32, a.result],
        );
        assert_eq!(result_of(&e, a), 0.0);
        // No reference or no event list: 0.0 and no question asked.
        start_log(&mut e);
        e.mem.set_f64(a.result.addr(), 7.0);
        e.call(
            0x005c_a1c0,
            &args![0x20u32, 0u32, 0x77u32, 0x99u32, a.result],
        );
        assert_eq!(result_of(&e, a), 0.0);
        e.mem.set_f64(a.result.addr(), 7.0);
        e.call(
            0x005c_a1c0,
            &args![0x20u32, 0x40u32, 0x77u32, 0u32, a.result],
        );
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT).is_empty());
    }

    /// An event flag command parses one argument and asks for it with `mask`.
    fn check_event_flag_command(addr: u32, mask: u32) {
        let mut e = engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| true.into_ret());
        parse_gives(&mut e, true, &[0x77]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        // The event list is the sixth word (6 in `script`).
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0x77, mask]]
        );
        assert_eq!(result_of(&e, a), 1.0);
        // Bad parameters: false and the result untouched.
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, 0x40);
        assert!(!run(&mut e, addr, a));
        assert_eq!(result_of(&e, a), 7.0);
    }

    #[test]
    fn event_flag_command_005ca200_uses_mask_1() {
        check_event_flag_command(0x005c_a200, 1);
    }

    #[test]
    fn event_flag_command_005ca260_uses_mask_0x4000() {
        check_event_flag_command(0x005c_a260, 0x4000);
    }

    #[test]
    fn event_flag_command_005ca2d0_uses_mask_2() {
        check_event_flag_command(0x005c_a2d0, 2);
    }

    #[test]
    fn event_flag_command_005ca330_uses_mask_8() {
        check_event_flag_command(0x005c_a330, 8);
    }

    #[test]
    fn event_flag_command_005ca390_uses_mask_4() {
        check_event_flag_command(0x005c_a390, 4);
    }

    #[test]
    fn the_argumentless_event_flag_command_asks_event_0_with_mask_0x400000() {
        let mut e = engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| false.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a510, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0, 0x40_0000]]
        );
        assert_eq!(result_of(&e, a), 0.0);
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| true.into_ret());
        assert!(run(&mut e, 0x005c_a510, a));
        assert_eq!(result_of(&e, a), 1.0);
        // No parameters are read.
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    // ---- actions and the 0x168 slot ---------------------------------------------------

    #[test]
    fn on_activate_reports_action_2_and_clears_both_actions() {
        let mut e = engine();
        e.register(HAS_ACTION, |_, a| (a[1] == 2 && a[0] == 0x40).into_ret());
        accept(&mut e, &[CLEAR_ACTION]);
        parse_gives(&mut e, true, &[0]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a3f0, a));
        assert_eq!(result_of(&e, a), 1.0);
        assert_eq!(calls(&e, CLEAR_ACTION), vec![vec![0x40, 1], vec![0x40, 2]]);
        // Action 2 not set (a different reference): 0.0, still cleared.
        let a = script(&mut e, 0x41);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a3f0, a));
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(calls(&e, CLEAR_ACTION).len(), 2);
        // No reference: 0.0, nothing touched.
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a3f0, a));
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, CLEAR_ACTION).is_empty());
        // Bad parameters: false, result already zeroed.
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, 0x40);
        assert!(!run(&mut e, 0x005c_a3f0, a));
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn pointer_test_is_true_for_a_non_null_pointer() {
        let mut e = engine();
        let cell = e.mem.alloc(4);
        assert!(!e.call(0x005c_a4f0, &args![cell]).bool());
        e.mem.set_u32(cell, 0x1234);
        assert!(e.call(0x005c_a4f0, &args![cell]).bool());
    }

    /// An object whose slot `0x168` gives `pointer`, a pointer to a cell
    /// holding `inner` (or null).
    fn slot_168_object(e: &mut Engine, pointer: bool, inner: u32) -> u32 {
        let object = object_with(e, false, &[(0x168, 0x0900_0300)]);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, inner);
        e.mem.set_u32(object + 0xe0, if pointer { cell } else { 0 });
        object
    }

    #[test]
    fn slot_168_command_is_1_unless_the_double_pointer_is_set() {
        let mut e = engine();
        e.register(0x0900_0300, |e, a| e.mem.u32(a[0] + 0xe0).into_ret());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        // Slot gives nothing: 1.0.
        let object = slot_168_object(&mut e, false, 0);
        let a = script(&mut e, object);
        assert!(run(&mut e, 0x005c_a470, a));
        assert_eq!(result_of(&e, a), 1.0);
        // Slot gives a pointer to null: 1.0.
        let object = slot_168_object(&mut e, true, 0);
        let a = script(&mut e, object);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a470, a));
        assert_eq!(result_of(&e, a), 1.0);
        // The slot is asked twice before the null cell is found.
        assert_eq!(calls(&e, 0x0900_0300).len(), 2);
        // Slot gives a pointer to a non-null pointer: 0.0.
        let object = slot_168_object(&mut e, true, 0x4444);
        let a = script(&mut e, object);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a470, a));
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(calls(&e, 0x0900_0300).len(), 3);
    }

    // ---- second batch: argument-less event flag commands -------------------------------

    /// An event flag command that reads no argument: it asks event 0 with
    /// `mask`, and its answer decides the result.
    fn check_argless_event_flag(addr: u32, mask: u32) {
        let mut e = engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0, mask]]
        );
        assert_eq!(result_of(&e, a), 1.0);
        // No parameters are read.
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        // The event is not set: 0.0.
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| false.into_ret());
        assert!(run(&mut e, addr, a));
        assert_eq!(result_of(&e, a), 0.0);
        // Without a reference the list is not asked and the result is 0.0.
        let other = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, addr, other));
        assert_eq!(result_of(&e, other), 0.0);
        assert!(calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT).is_empty());
    }

    #[test]
    fn event_flag_command_005ca540_uses_mask_0x10000() {
        check_argless_event_flag(0x005c_a540, 0x1_0000);
    }

    #[test]
    fn event_flag_command_005ca570_uses_mask_0x20000() {
        check_argless_event_flag(0x005c_a570, 0x2_0000);
    }

    #[test]
    fn event_flag_command_005ca5a0_uses_mask_0x80000() {
        check_argless_event_flag(0x005c_a5a0, 0x8_0000);
    }

    #[test]
    fn event_flag_command_005ca5d0_uses_mask_0x100000() {
        check_argless_event_flag(0x005c_a5d0, 0x10_0000);
    }

    #[test]
    fn event_flag_command_005ca930_uses_mask_0x40() {
        check_argless_event_flag(0x005c_a930, 0x40);
    }

    #[test]
    fn event_flag_command_005cab00_uses_mask_0x1000() {
        check_argless_event_flag(0x005c_ab00, 0x1000);
    }

    #[test]
    fn event_flag_command_005cab30_uses_mask_0x80000000() {
        check_argless_event_flag(0x005c_ab30, 0x8000_0000);
    }

    #[test]
    fn event_flag_command_005ca600_parses_an_unused_argument_and_asks_event_0() {
        let mut e = engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| true.into_ret());
        parse_gives(&mut e, true, &[0x1234]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a600, a));
        // The parsed value is never used: event 0.
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0, 0x20_0000]]
        );
        assert_eq!(result_of(&e, a), 1.0);
        // The parser got a local preset to 0x80000000.
        e.register(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), 0x8000_0000);
            true.into_ret()
        });
        assert!(run(&mut e, 0x005c_a600, a));
        // Parameters that do not parse: false, the result is already 0.0.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_a600, a));
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT).is_empty());
    }

    // ---- second batch: event flag commands with one argument ----------------------------

    #[test]
    fn event_flag_command_005ca670_uses_mask_0x80() {
        check_event_flag_command(0x005c_a670, 0x80);
    }

    #[test]
    fn event_flag_command_005ca800_uses_mask_0x10() {
        check_event_flag_command(0x005c_a800, 0x10);
    }

    #[test]
    fn event_flag_command_005ca860_uses_mask_0x8000() {
        check_event_flag_command(0x005c_a860, 0x8000);
    }

    #[test]
    fn event_flag_command_005ca8d0_uses_mask_0x20() {
        check_event_flag_command(0x005c_a8d0, 0x20);
    }

    #[test]
    fn event_flag_command_005ca9b0_uses_mask_0x200() {
        check_event_flag_command(0x005c_a9b0, 0x200);
    }

    #[test]
    fn event_flag_command_005caa20_uses_mask_0x400() {
        check_event_flag_command(0x005c_aa20, 0x400);
    }

    #[test]
    fn event_flag_command_005caa90_uses_mask_0x800() {
        check_event_flag_command(0x005c_aa90, 0x800);
    }

    #[test]
    fn event_flag_command_005cab60_uses_mask_0x2000() {
        check_event_flag_command(0x005c_ab60, 0x2000);
    }

    #[test]
    fn event_flag_command_005cabd0_uses_mask_0x10000000() {
        check_event_flag_command(0x005c_abd0, 0x1000_0000);
    }

    #[test]
    fn event_flag_command_005cac40_uses_mask_0x20000000() {
        check_event_flag_command(0x005c_ac40, 0x2000_0000);
    }

    #[test]
    fn event_flag_command_005cacb0_uses_mask_0x40000000() {
        check_event_flag_command(0x005c_acb0, 0x4000_0000);
    }

    #[test]
    fn event_flag_command_005ca950_ignores_whether_the_argument_parsed() {
        let mut e = engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| true.into_ret());
        parse_gives(&mut e, true, &[0x77]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a950, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0x77, 0x4_0000]]
        );
        assert_eq!(result_of(&e, a), 1.0);
        // A failed parse still succeeds; the local keeps what the parser
        // wrote (0x55) or its preset (0).
        parse_gives(&mut e, false, &[0x55]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a950, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0x55, 0x4_0000]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a950, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0, 0x4_0000]]
        );
    }

    // ---- second batch: the list form event flag command ----------------------------------

    /// Fake virtual function of the list entries: answers whether the byte
    /// at `+0xff` is 1.
    const V_ENTRY_ANSWERS: u32 = 0x0900_0400;

    /// An engine with the node walking functions of `005ca6e0` replaced by
    /// doubles that read the nodes as the game lays them out: data pointer
    /// at `+0`, next node at `+4`; a form's list head is at `+0x18`, its
    /// type byte at `+4`.
    fn list_engine() -> Engine {
        let mut e = engine();
        e.register(FORM_GET_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(LIST_FORM_FIRST_NODE, |_, a| (a[0] + 0x18).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_DATA_POINTER, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(V_ENTRY_ANSWERS, |e, a| {
            (e.mem.u8(a[0] + 0xff) == 1).into_ret()
        });
        e
    }

    /// A list form with the given entries (`(answers slot 0xe4, address)`)
    /// chained from its head node at `+0x18`: returns the form and the
    /// entries' addresses.
    fn list_form(e: &mut Engine, form_type: u8, answers: &[bool]) -> (u32, Vec<u32>) {
        let form = e.mem.alloc(0x40);
        e.mem.set_u8(form + 4, form_type);
        let mut items = vec![];
        let mut node = form + 0x18;
        for (i, answer) in answers.iter().enumerate() {
            let item = object_with(e, false, &[(0xe4, V_ENTRY_ANSWERS)]);
            e.mem.set_u8(item + 0xff, *answer as u8);
            e.mem.set_u32(node, item);
            if i + 1 < answers.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
            items.push(item);
        }
        (form, items)
    }

    #[test]
    fn the_list_form_command_walks_entries_until_the_result_is_not_zero() {
        let mut e = list_engine();
        let (form, items) = list_form(&mut e, 0x55, &[false, true, true]);
        parse_gives(&mut e, true, &[form]);
        // The event list knows the second entry only.
        e.register_double(SCRIPT_EVENT_LIST_HAS_EVENT, {
            let second = items[1];
            move |_, a| (a[1] == second).into_ret()
        });
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a6e0, a));
        // First entry does not answer slot 0xe4: event 0 is asked; the
        // second is found, the third is never visited.
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0, 0x100], vec![6, items[1], 0x100]]
        );
        assert_eq!(calls(&e, V_ENTRY_ANSWERS).len(), 2);
        assert_eq!(result_of(&e, a), 1.0);
    }

    #[test]
    fn the_list_form_command_visits_every_entry_when_none_is_found() {
        let mut e = list_engine();
        let (form, items) = list_form(&mut e, 0x55, &[true, true]);
        parse_gives(&mut e, true, &[form]);
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| false.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a6e0, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, items[0], 0x100], vec![6, items[1], 0x100]]
        );
        assert_eq!(result_of(&e, a), 0.0);
        // An empty list asks nothing and leaves the result alone.
        let (empty, _) = list_form(&mut e, 0x55, &[]);
        parse_gives(&mut e, true, &[empty]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a6e0, a));
        assert!(calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT).is_empty());
        assert_eq!(result_of(&e, a), 7.0);
    }

    #[test]
    fn the_list_form_command_asks_other_arguments_directly() {
        let mut e = list_engine();
        e.register(SCRIPT_EVENT_LIST_HAS_EVENT, |_, _| true.into_ret());
        // A form of another type is the event itself.
        let (other, _) = list_form(&mut e, 0x10, &[true]);
        parse_gives(&mut e, true, &[other]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a6e0, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, other, 0x100]]
        );
        // No argument: event 0, the type is not asked.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_a6e0, a));
        assert_eq!(
            calls(&e, SCRIPT_EVENT_LIST_HAS_EVENT),
            vec![vec![6, 0, 0x100]]
        );
        assert!(calls(&e, FORM_GET_TYPE).is_empty());
        // Parameters that do not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_a6e0, a));
    }

    // ---- second batch: player controls --------------------------------------------------

    /// An engine whose player global points to a heap block.
    fn player_engine() -> (Engine, u32) {
        let mut e = engine();
        let player = e.mem.alloc(0x10);
        e.mem.set_u32(PLAYER, player);
        accept(&mut e, &[PLAYER_CHANGE_CONTROL_BITS]);
        (e, player)
    }

    #[test]
    fn the_controls_command_005cad20_combines_the_seven_flags_preset_to_1() {
        let (mut e, player) = player_engine();
        // The parser leaves the presets alone: all seven bits.
        e.register(PARSE_PARAMETERS, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ad20, a));
        assert_eq!(
            calls(&e, PLAYER_CHANGE_CONTROL_BITS),
            vec![vec![player, 0, 0x7f]]
        );
        // Each flag has its own bit, in the order they are parsed.
        let bits = [1, 4, 8, 0x10, 2, 0x20, 0x40];
        for (i, bit) in bits.iter().enumerate() {
            let mut outs = [0u32; 7];
            outs[i] = 5;
            parse_gives(&mut e, true, &outs);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_ad20, a));
            assert_eq!(
                calls(&e, PLAYER_CHANGE_CONTROL_BITS),
                vec![vec![player, 0, *bit]]
            );
        }
        // Parameters that do not parse: false, nothing changed.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_ad20, a));
        assert!(calls(&e, PLAYER_CHANGE_CONTROL_BITS).is_empty());
    }

    #[test]
    fn the_controls_command_005cae30_presets_only_the_first_three_flags() {
        let (mut e, player) = player_engine();
        e.register(PARSE_PARAMETERS, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ae30, a));
        // Flags 1, 2 and 3 are on: bits 1 | 4 | 8.
        assert_eq!(
            calls(&e, PLAYER_CHANGE_CONTROL_BITS),
            vec![vec![player, 1, 0xd]]
        );
        // The other four flags give their own bits when parsed as non-zero.
        parse_gives(&mut e, true, &[1, 1, 1, 1, 1, 1, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ae30, a));
        assert_eq!(
            calls(&e, PLAYER_CHANGE_CONTROL_BITS),
            vec![vec![player, 1, 0x7f]]
        );
        parse_gives(&mut e, true, &[0, 0, 0, 0, 0, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ae30, a));
        assert_eq!(
            calls(&e, PLAYER_CHANGE_CONTROL_BITS),
            vec![vec![player, 1, 0]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_ae30, a));
        assert!(calls(&e, PLAYER_CHANGE_CONTROL_BITS).is_empty());
    }

    #[test]
    fn get_player_controls_disabled_asks_the_condition_with_the_mask() {
        let (mut e, _) = player_engine();
        accept(&mut e, &[GET_PLAYER_CONTROLS_DISABLED_CONDITION]);
        e.register(PARSE_PARAMETERS, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_af40, a));
        assert_eq!(
            calls(&e, GET_PLAYER_CONTROLS_DISABLED_CONDITION),
            vec![vec![0, 0x7f, 0, a.result.addr()]]
        );
        // Only the second flag set: bit 4.
        parse_gives(&mut e, true, &[0, 1, 0, 0, 0, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_af40, a));
        assert_eq!(
            calls(&e, GET_PLAYER_CONTROLS_DISABLED_CONDITION),
            vec![vec![0, 4, 0, a.result.addr()]]
        );
        // The player is never touched; bad parameters: false.
        assert!(calls(&e, PLAYER_CHANGE_CONTROL_BITS).is_empty());
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_af40, a));
        assert!(calls(&e, GET_PLAYER_CONTROLS_DISABLED_CONDITION).is_empty());
    }

    // ---- second batch: condition twins --------------------------------------------------

    #[test]
    fn get_heading_angle_needs_this_obj_and_the_argument() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x77]);
        e.register(GET_HEADING_ANGLE_CONDITION, |_, a| {
            (a[1] == 0x77).into_ret()
        });
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b050, a));
        assert_eq!(
            calls(&e, GET_HEADING_ANGLE_CONDITION),
            vec![vec![0x40, 0x77, 0, a.result.addr()]]
        );
        // The condition's answer is the command's answer.
        parse_gives(&mut e, true, &[0x78]);
        assert!(!run(&mut e, 0x005c_b050, a));
        // Without `thisObj`, or without the argument: true, no call.
        start_log(&mut e);
        let no_this = script(&mut e, 0);
        parse_gives(&mut e, true, &[0x77]);
        assert!(run(&mut e, 0x005c_b050, no_this));
        parse_gives(&mut e, true, &[0]);
        assert!(run(&mut e, 0x005c_b050, a));
        assert!(calls(&e, GET_HEADING_ANGLE_CONDITION).is_empty());
        // Parameters that do not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_b050, a));
    }

    /// A command that forwards `callee(thisObj, 0, 0, result)` but is true
    /// without a call when there is no `thisObj`.
    fn check_this_condition(addr: u32, callee: u32) {
        check_plain_condition(addr, callee);
        let mut e = engine();
        e.register(callee, |_, _| false.into_ret());
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn the_005cb660_command_forwards_to_is_weapon_out() {
        check_this_condition(0x005c_b660, IS_WEAPON_OUT_CONDITION);
    }

    #[test]
    fn the_005cb6f0_command_forwards_to_005a2a50() {
        check_this_condition(0x005c_b6f0, FN_005A2A50);
    }

    #[test]
    fn the_005cb720_command_forwards_to_is_facing_up() {
        check_this_condition(0x005c_b720, IS_FACING_UP_CONDITION);
    }

    #[test]
    fn the_005cb750_command_forwards_to_is_left_up() {
        check_this_condition(0x005c_b750, IS_LEFT_UP_CONDITION);
    }

    #[test]
    fn the_005cb780_command_forwards_to_get_knocked_state() {
        check_this_condition(0x005c_b780, GET_KNOCKED_STATE_CONDITION);
    }

    #[test]
    fn the_005cb7b0_command_forwards_to_get_weapon_anim_type() {
        check_this_condition(0x005c_b7b0, GET_WEAPON_ANIM_TYPE_CONDITION);
    }

    #[test]
    fn the_005cb850_command_forwards_to_get_current_ai_package() {
        check_this_condition(0x005c_b850, GET_CURRENT_AI_PACKAGE_CONDITION);
    }

    #[test]
    fn the_005cb880_command_forwards_to_is_waiting() {
        check_this_condition(0x005c_b880, IS_WAITING_CONDITION);
    }

    #[test]
    fn the_005cb8b0_command_forwards_to_is_idle_playing() {
        check_this_condition(0x005c_b8b0, IS_IDLE_PLAYING_CONDITION);
    }

    #[test]
    fn is_weapon_in_list_passes_the_parsed_list() {
        check_one_argument_condition(0x005c_b690, IS_WEAPON_IN_LIST_CONDITION);
        // `thisObj` is not checked: the condition is called with 0.
        let mut e = engine();
        parse_gives(&mut e, true, &[0x77]);
        e.register(IS_WEAPON_IN_LIST_CONDITION, |_, _| true.into_ret());
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b690, a));
        assert_eq!(
            calls(&e, IS_WEAPON_IN_LIST_CONDITION),
            vec![vec![0, 0x77, 0, a.result.addr()]]
        );
    }

    #[test]
    fn is_weapon_skill_type_needs_this_obj() {
        check_one_argument_condition(0x005c_b7e0, IS_WEAPON_SKILL_TYPE_CONDITION);
        // Without `thisObj`: true and no call, whatever was parsed.
        let mut e = engine();
        parse_gives(&mut e, true, &[0x77]);
        e.register(IS_WEAPON_SKILL_TYPE_CONDITION, |_, _| false.into_ret());
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b7e0, a));
        assert!(calls(&e, IS_WEAPON_SKILL_TYPE_CONDITION).is_empty());
        // The argument may be 0: the condition is still asked.
        parse_gives(&mut e, true, &[0]);
        let a = script(&mut e, 0x40);
        assert!(!run(&mut e, 0x005c_b7e0, a));
        assert_eq!(calls(&e, IS_WEAPON_SKILL_TYPE_CONDITION).len(), 1);
    }

    // ---- second batch: PickIdle, PlayIdle and PlayVATSCameras ---------------------------

    // Fake virtual functions of the idle scene.
    /// Actor slot `0x1e4`: the animation stored at `+0xe0`.
    const V_ANIMATION: u32 = 0x0900_0500;
    /// Process slot `0x128`: `0x77`.
    const V_PROCESS_WORD: u32 = 0x0900_0501;
    /// Process slot `0x71c`.
    const V_PLAY_IDLE: u32 = 0x0900_0502;
    /// Process slot `0x614`.
    const V_PROCESS_FLAGS: u32 = 0x0900_0503;
    /// Idle slot `0x130`: the editor ID stored at `+0xe0`.
    const V_EDITOR_ID: u32 = 0x0900_0504;
    /// Slot `0x14` of the sub-object at `+0x18` of the idle: the file.
    const V_FILE: u32 = 0x0900_0505;

    /// An idle scene: an actor with a process and an animation, an idle form
    /// and the doubles that read them.
    struct IdleScene {
        actor: u32,
        process: u32,
        animation: u32,
        idle: u32,
    }

    /// Sets up the scene in `e`: the idle manager answers `idle`; the
    /// animation has no special idle playing (`done playing`), holds
    /// nothing at `+0x124` / `+0x128`.
    fn idle_scene(e: &mut Engine) -> IdleScene {
        e.map(0x011c_b000, 0x1000);
        e.map(0x011f_2000, 0x1000);
        e.register(V_ANIMATION, |e, a| e.mem.u32(a[0] + 0xe0).into_ret());
        e.register(V_PROCESS_WORD, |_, _| 0x77u32.into_ret());
        e.register(V_PLAY_IDLE, |_, _| Ret::default());
        e.register(V_PROCESS_FLAGS, |_, _| Ret::default());
        e.register(V_EDITOR_ID, |e, a| e.mem.u32(a[0] + 0xe0).into_ret());
        e.register(V_FILE, |_, _| 0x5555u32.into_ret());
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0xe4).into_ret()
        });
        e.register(GET_IDLE_TO_PLAY, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SPECIAL_IDLE_DONE_PLAYING, |e, a| {
            (e.mem.u8(a[0] + 0x10) == 1).into_ret()
        });
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FN_0055B980, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(GET_DWORD_0C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(FN_00490E40, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(FN_0059BB30, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        accept(e, &[CONSOLE_PRINT]);

        let process = object_with(
            e,
            false,
            &[
                (0x128, V_PROCESS_WORD),
                (0x71c, V_PLAY_IDLE),
                (0x614, V_PROCESS_FLAGS),
            ],
        );
        let actor = object_with(e, true, &[(0x1e4, V_ANIMATION)]);
        e.mem.set_u32(actor + 0xe4, process);
        let animation = e.mem.alloc(0x200);
        e.mem.set_u8(animation + 0x10, 1);
        e.mem.set_u32(actor + 0xe0, animation);
        let idle = object_with(e, false, &[(0x130, V_EDITOR_ID)]);
        let sub_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(sub_vtable + 0x14, V_FILE);
        e.mem.set_u32(idle + 0x18, sub_vtable);
        e.mem.set_u32(idle + 0xc, 0xfe01);
        e.mem.set_u32(idle + 0xe0, 0x4444);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager, idle);
        e.mem.set_u32(IDLE_MANAGER, manager);
        IdleScene {
            actor,
            process,
            animation,
            idle,
        }
    }

    /// Switches the echo flag of the TLS block.
    fn set_echo(e: &mut Engine, on: bool) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, on as u8);
    }

    /// An object the animation holds: `+0xc` is `value`, `+0x18` the
    /// `inner` object, `+0x2c` the `idle` it plays.
    fn held_object(e: &mut Engine, value: u32, inner: u32, idle: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xc, value);
        e.mem.set_u32(object + 0x18, inner);
        e.mem.set_u32(object + 0x2c, idle);
        object
    }

    /// Whether the process was told to play the idle.
    fn idle_started(e: &Engine, scene: &IdleScene) -> bool {
        let started = calls(e, V_PLAY_IDLE);
        let flags = calls(e, V_PROCESS_FLAGS);
        if started.is_empty() {
            assert!(flags.is_empty());
            return false;
        }
        assert_eq!(started, vec![vec![scene.process, scene.idle]]);
        assert_eq!(flags, vec![vec![scene.process, 0x80]]);
        true
    }

    #[test]
    fn pick_idle_starts_the_idle_the_manager_picks() {
        let mut e = engine();
        let scene = idle_scene(&mut e);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        // The manager is asked with the actor and the process's word.
        let manager = e.mem.u32(IDLE_MANAGER);
        assert_eq!(
            calls(&e, GET_IDLE_TO_PLAY),
            vec![vec![manager, scene.actor, 0x77]]
        );
        assert!(idle_started(&e, &scene));
        // The echo flag is off: no message.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn pick_idle_prints_the_picked_idle_when_the_echo_flag_is_set() {
        let mut e = engine();
        let scene = idle_scene(&mut e);
        set_echo(&mut e, true);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(idle_started(&e, &scene));
        // Editor ID, form ID, file, in the order of the format string.
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_PICKED_IDLE, 0x4444, 0xfe01, 0x5555]]
        );
        // The file comes from slot 0x14 of the sub-object at +0x18.
        assert_eq!(calls(&e, V_FILE), vec![vec![scene.idle + 0x18]]);
    }

    #[test]
    fn pick_idle_does_nothing_without_an_actor_a_process_an_idle_or_an_animation() {
        // Not an actor: not even the process is asked.
        let mut e = engine();
        let scene = idle_scene(&mut e);
        e.mem.set_u8(scene.actor + 0xff, 0);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(calls(&e, GET_SAVED_ACQUIRE_OBJECT).is_empty());
        // No `thisObj` at all.
        let a = script(&mut e, 0);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(calls(&e, GET_SAVED_ACQUIRE_OBJECT).is_empty());
        // No process.
        let mut e = engine();
        let scene = idle_scene(&mut e);
        e.mem.set_u32(scene.actor + 0xe4, 0);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(calls(&e, GET_IDLE_TO_PLAY).is_empty());
        // The manager picks nothing.
        let mut e = engine();
        let scene = idle_scene(&mut e);
        let manager = e.mem.u32(IDLE_MANAGER);
        e.mem.set_u32(manager, 0);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(calls(&e, V_ANIMATION).is_empty());
        assert!(!idle_started(&e, &scene));
        // The actor has no animation.
        let mut e = engine();
        let scene = idle_scene(&mut e);
        e.mem.set_u32(scene.actor + 0xe0, 0);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(calls(&e, SPECIAL_IDLE_DONE_PLAYING).is_empty());
        assert!(!idle_started(&e, &scene));
    }

    #[test]
    fn pick_idle_does_not_restart_the_idle_the_animation_already_holds() {
        let mut e = engine();
        let scene = idle_scene(&mut e);
        // +0x128 points to a cell holding an object whose +0x2c is the idle.
        let object = held_object(&mut e, 0, 0, scene.idle);
        e.mem.set_u32(scene.animation + 0x128, object);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(!idle_started(&e, &scene));
        // A different held idle does not stop it.
        let other = held_object(&mut e, 0, 0, 0x9999);
        e.mem.set_u32(scene.animation + 0x128, other);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(idle_started(&e, &scene));
    }

    #[test]
    fn pick_idle_waits_while_a_special_idle_plays() {
        let mut e = engine();
        let scene = idle_scene(&mut e);
        // A special idle is playing and nothing is held at +0x124: wait.
        e.mem.set_u8(scene.animation + 0x10, 0);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(!idle_started(&e, &scene));
        // The held object's +0xc is not 3: go ahead.
        let held = held_object(&mut e, 2, 0, 0);
        e.mem.set_u32(scene.animation + 0x124, held);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(idle_started(&e, &scene));
        // It is 3 and nothing sits behind it (+0x18 is 0): wait.
        let held = held_object(&mut e, 3, 0, 0);
        e.mem.set_u32(scene.animation + 0x124, held);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(!idle_started(&e, &scene));
        // It is 3, the object behind it has a non-zero +0x24: wait.
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0x24, 1);
        let held = held_object(&mut e, 3, inner, 0);
        e.mem.set_u32(scene.animation + 0x124, held);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(!idle_started(&e, &scene));
        // That +0x24 is 0: go ahead.
        e.mem.set_u32(inner + 0x24, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b0c0, a));
        assert!(idle_started(&e, &scene));
    }

    /// The doubles of the editor ID commands: the parser accepts, the form
    /// lookup gives `form`, and the cast gives a form whose byte `+0xfe` is 1
    /// as an idle and `+0xfd` as a camera path.
    fn editor_id_engine(form: u32) -> Engine {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |e, a| {
            let flag = if a[2] == RTTI_TES_OBJECT_REFR {
                assert_eq!(a[3], RTTI_ACTOR);
                0xff
            } else {
                assert_eq!(a[2], RTTI_TES_FORM);
                match a[3] {
                    RTTI_TES_IDLE_FORM => 0xfe,
                    RTTI_BGS_CAMERA_PATH => 0xfd,
                    other => panic!("unexpected cast target {other:x}"),
                }
            };
            let ok = a[0] != 0 && e.mem.u8(a[0] + flag) == 1;
            (if ok { a[0] } else { 0 }).into_ret()
        });
        e.register_double(GET_FORM_BY_EDITOR_ID, move |_, _| form.into_ret());
        e
    }

    #[test]
    fn play_idle_starts_the_idle_the_editor_id_names() {
        let mut e = editor_id_engine(0);
        let scene = idle_scene(&mut e);
        // The editor ID names the idle form of the scene.
        e.mem.set_u8(scene.idle + 0xfe, 1);
        e.register_double(GET_FORM_BY_EDITOR_ID, {
            let idle = scene.idle;
            move |_, _| idle.into_ret()
        });
        parse_gives(&mut e, true, &[]);
        set_echo(&mut e, true);
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b2d0, a));
        assert!(idle_started(&e, &scene));
        // The parser wrote into a 512-byte buffer that the lookup got.
        let buffer = calls(&e, PARSE_PARAMETERS)[0][7];
        assert_eq!(calls(&e, GET_FORM_BY_EDITOR_ID), vec![vec![buffer]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_PLAY_IDLE_FOUND, 0x4444, 0xfe01, 0x5555]]
        );
        // The manager is not asked: the idle comes from the text.
        assert!(calls(&e, GET_IDLE_TO_PLAY).is_empty());
    }

    #[test]
    fn play_idle_ignores_forms_that_are_not_idles_and_missing_actors() {
        let mut e = editor_id_engine(0);
        let scene = idle_scene(&mut e);
        parse_gives(&mut e, true, &[]);
        // The lookup finds a form that is not an idle (+0xfe is 0).
        e.register_double(GET_FORM_BY_EDITOR_ID, {
            let idle = scene.idle;
            move |_, _| idle.into_ret()
        });
        let a = script(&mut e, scene.actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b2d0, a));
        assert!(!idle_started(&e, &scene));
        // Nothing found.
        e.register_double(GET_FORM_BY_EDITOR_ID, |_, _| 0u32.into_ret());
        assert!(run(&mut e, 0x005c_b2d0, a));
        assert!(!idle_started(&e, &scene));
        // Not an actor / no process: the lookup is not made.
        e.mem.set_u8(scene.actor + 0xff, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b2d0, a));
        assert!(calls(&e, GET_FORM_BY_EDITOR_ID).is_empty());
        e.mem.set_u8(scene.actor + 0xff, 1);
        e.mem.set_u32(scene.actor + 0xe4, 0);
        assert!(run(&mut e, 0x005c_b2d0, a));
        assert!(calls(&e, GET_FORM_BY_EDITOR_ID).is_empty());
        // Text that does not parse: false, even without an actor.
        parse_gives(&mut e, false, &[]);
        let nobody = script(&mut e, 0);
        assert!(!run(&mut e, 0x005c_b2d0, nobody));
    }

    #[test]
    fn play_vats_cameras_queues_the_camera_path_the_editor_id_names() {
        let mut e = editor_id_engine(0);
        e.map(0x011f_2000, 0x1000);
        let path = e.mem.alloc(0x100);
        e.mem.set_u8(path + 0xfd, 1);
        e.register_double(GET_FORM_BY_EDITOR_ID, move |_, _| path.into_ret());
        parse_gives(&mut e, true, &[]);
        accept(&mut e, &[CONSOLE_PRINT]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b580, a));
        assert_eq!(e.mem.u32(QUEUED_VATS_CAMERA_PATH), path);
        // The echo flag is off: no message.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // With it on the message names the text buffer.
        set_echo(&mut e, true);
        assert!(run(&mut e, 0x005c_b580, a));
        let buffer = calls(&e, PARSE_PARAMETERS)[0][7];
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_VATS_QUEUED, buffer]]
        );
    }

    #[test]
    fn play_vats_cameras_clears_the_queue_when_nothing_is_found() {
        let mut e = editor_id_engine(0);
        e.map(0x011f_2000, 0x1000);
        e.mem.set_u32(QUEUED_VATS_CAMERA_PATH, 0x1234);
        // The form exists but is not a camera path.
        let other = e.mem.alloc(0x100);
        e.register_double(GET_FORM_BY_EDITOR_ID, move |_, _| other.into_ret());
        parse_gives(&mut e, true, &[]);
        accept(&mut e, &[CONSOLE_PRINT]);
        set_echo(&mut e, true);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b580, a));
        assert_eq!(e.mem.u32(QUEUED_VATS_CAMERA_PATH), 0);
        let buffer = calls(&e, PARSE_PARAMETERS)[0][7];
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_VATS_NOT_FOUND, buffer]]
        );
        // Text that does not parse: false and the queue is not touched.
        e.mem.set_u32(QUEUED_VATS_CAMERA_PATH, 0x1234);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_b580, a));
        assert_eq!(e.mem.u32(QUEUED_VATS_CAMERA_PATH), 0x1234);
        assert!(calls(&e, GET_FORM_BY_EDITOR_ID).is_empty());
    }

    // ---- second batch: ShowQuestTargets -------------------------------------------------

    /// Fake virtual function of the quest targets scene: the text stored at
    /// `+0xe0` of the object (quest name, editor ID).
    const V_TEXT: u32 = 0x0900_0600;

    /// A quest target: references at `+0x10` and `+0x14`, load door at
    /// `+0x18`.
    fn quest_target(e: &mut Engine, reference_a: u32, reference_b: u32, door: u32) -> u32 {
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0x10, reference_a);
        e.mem.set_u32(target + 0x14, reference_b);
        e.mem.set_u32(target + 0x18, door);
        target
    }

    /// A form with a form ID at `+0xc` and the editor ID text `text`.
    fn named_form(e: &mut Engine, form_id: u32, text: u32) -> u32 {
        let form = object_with(e, false, &[(0x130, V_TEXT)]);
        e.mem.set_u32(form + 0xc, form_id);
        e.mem.set_u32(form + 0xe0, text);
        form
    }

    /// The scene: the player's active quest (named by `V_TEXT`, text
    /// `0x6001`) and a target list chained from `targets` (a node holds its
    /// target at `+0`, the next node at `+4`; a node with data 0 ends the
    /// walk). The string object doubles keep the last text assigned.
    fn quest_scene(e: &mut Engine, quest: bool, targets: &[u32]) -> u32 {
        let player = e.mem.alloc(0x10);
        e.mem.set_u32(PLAYER, player);
        // The quest answers slot 0x138 with its name text.
        let quest_form = if quest {
            let form = object_with(e, false, &[(0x138, V_TEXT)]);
            e.mem.set_u32(form + 0xe0, 0x6001);
            form
        } else {
            0
        };
        e.mem.set_u32(player + 4, quest_form);
        e.register(V_TEXT, |e, a| e.mem.u32(a[0] + 0xe0).into_ret());
        e.register(PLAYER_GET_ACTIVE_QUEST, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(PLAYER_GET_CURRENT_TARGET_LIST, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(LIST_COUNT_NON_EMPTY, |_, _| 2u32.into_ret());
        e.register(NODE_DATA_POINTER, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(QUEST_TARGET_GET_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 0x10 + 4 * a[1]).into_ret()
        });
        e.register(QUEST_TARGET_GET_LOAD_DOOR, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(BGS_SAVE_FORM_BUFFER_GET_FORM, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(FORM_GET_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(GET_DWORD_0C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            a[0].into_ret()
        });
        e.register(STRING_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STRING_FORMAT, |e, a| {
            // The string object remembers the format it was built with.
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STRING_DESTROY, |_, _| Ret::default());
        accept(e, &[CONSOLE_PRINT]);
        // The target list nodes.
        let mut head = 0;
        for target in targets.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *target);
            e.mem.set_u32(node + 4, head);
            head = node;
        }
        e.mem.set_u32(player + 8, head);
        player
    }

    #[test]
    fn show_quest_targets_says_so_without_an_active_quest() {
        let mut e = engine();
        quest_scene(&mut e, false, &[]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b8e0, a));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_NO_ACTIVE_QUEST]]);
        assert!(calls(&e, PLAYER_GET_CURRENT_TARGET_LIST).is_empty());
    }

    #[test]
    fn show_quest_targets_names_the_quest_and_the_missing_targets() {
        let mut e = engine();
        quest_scene(&mut e, true, &[]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b8e0, a));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![
                vec![FORMAT_ACTIVE_QUEST, 0x6001],
                vec![MSG_NO_CURRENT_TARGETS]
            ]
        );
        assert!(calls(&e, STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn show_quest_targets_describes_each_target() {
        let mut e = engine();
        // Targets: one reference that is also its own carrier, no load door;
        // one reference carried by another with a load door of type 0x1c;
        // one with a load door of another type and a missing reference.
        let thing = named_form(&mut e, 0x200, 0x6101);
        let holder = named_form(&mut e, 0x201, 0x6102);
        let door = named_form(&mut e, 0x202, 0x6103);
        let door_form = e.mem.alloc(0x40);
        e.mem.set_u8(door_form + 4, 0x1c);
        e.mem.set_u32(door + 0x20, door_form);
        let wrong_door = named_form(&mut e, 0x203, 0x6104);
        let wrong_form = e.mem.alloc(0x40);
        e.mem.set_u8(wrong_form + 4, 0x1d);
        e.mem.set_u32(wrong_door + 0x20, wrong_form);
        let first = quest_target(&mut e, thing, thing, 0);
        let second = quest_target(&mut e, thing, holder, door);
        let third = quest_target(&mut e, 0, holder, wrong_door);
        quest_scene(&mut e, true, &[first, second, third]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b8e0, a));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![
                vec![FORMAT_ACTIVE_QUEST, 0x6001],
                vec![FORMAT_CURRENT_TARGETS, 2],
                // Target 1: the name and ID only; the door text stays the
                // "same cell" text.
                vec![FORMAT_TARGET, 1, FORMAT_NAME_AND_ID, TEXT_SAME_CELL],
                // Target 2: the carrier first, "carrying" the reference;
                // the load door is described.
                vec![FORMAT_TARGET, 2, FORMAT_CARRYING, FORMAT_NAME_AND_ID],
                // Target 3: a reference is missing so no text was built,
                // and the load door has the wrong type.
                vec![FORMAT_TARGET, 3, 0, TEXT_SAME_CELL],
            ]
        );
        let formats = calls(&e, STRING_FORMAT);
        assert_eq!(formats.len(), 3);
        assert_eq!(formats[0][1..], [FORMAT_NAME_AND_ID, 0x6101, 0x200]);
        assert_eq!(
            formats[1][1..],
            [FORMAT_CARRYING, 0x6102, 0x201, 0x6101, 0x200]
        );
        assert_eq!(formats[2][1..], [FORMAT_NAME_AND_ID, 0x6103, 0x202]);
        // Both string objects of every target are built and destroyed,
        // the door text after the reference text.
        assert_eq!(calls(&e, STRING_CONSTRUCT).len(), 6);
        let destroyed = calls(&e, STRING_DESTROY);
        assert_eq!(destroyed.len(), 6);
        assert_eq!(calls(&e, STRING_ASSIGN)[0][1..], [TEXT_SAME_CELL, 0]);
    }

    #[test]
    fn show_quest_targets_stops_at_a_node_without_data() {
        let mut e = engine();
        let thing = named_form(&mut e, 0x200, 0x6101);
        let first = quest_target(&mut e, thing, thing, 0);
        // The second node holds no target: the walk ends there.
        quest_scene(&mut e, true, &[first, 0, first]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_b8e0, a));
        let targets: Vec<_> = calls(&e, CONSOLE_PRINT)
            .into_iter()
            .filter(|words| words[0] == FORMAT_TARGET)
            .collect();
        assert_eq!(targets.len(), 1);
    }

    // ---- third batch (005cbb50 to 005cd910) -----------------------------------------

    /// Fake virtual functions of the third batch: they only record the call
    /// (their arguments show in the call log).
    const V_A: u32 = 0x0900_0700;
    const V_B: u32 = 0x0900_0701;
    const V_C: u32 = 0x0900_0702;
    const V_D: u32 = 0x0900_0703;
    /// Answers whether the byte at `+0xff` of the object is 1 (an actor).
    const V_IS_ACTOR: u32 = 0x0900_0704;
    /// Answers the byte at `+0xfd` of the object.
    const V_FLAG_FD: u32 = 0x0900_0705;
    /// Answers the dword at `+0xe0` of the object.
    const V_FIELD_E0: u32 = 0x0900_0706;

    /// An object of `size` bytes with a vtable (in the heap) holding the
    /// given `(byte offset, function)` slots; `actor` marks it for the cast
    /// double.
    fn object_sized(e: &mut Engine, size: u32, actor: bool, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        e.mem.set_u8(object + 0xff, actor as u8);
        object
    }

    /// `engine()` with the pages of the globals the third batch reads
    /// mapped, the player (an actor of 0x900 bytes whose slot `0x22c`
    /// answers its dword at `+0xe0`) set and the fake virtual functions
    /// registered.
    fn engine3() -> Engine {
        let mut e = engine();
        for page in [0x0101_2000, 0x0118_9000, 0x011c_3000] {
            e.map(page, 0x1000);
        }
        let player = object_sized(
            &mut e,
            0x900,
            true,
            &[(0x22c, V_FIELD_E0), (0x100, V_IS_ACTOR)],
        );
        e.mem.set_u32(PLAYER, player);
        accept(&mut e, &[V_A, V_B, V_C, V_D]);
        e.register(V_IS_ACTOR, |e, a| (e.mem.u8(a[0] + 0xff) == 1).into_ret());
        e.register(V_FLAG_FD, |e, a| e.mem.u8(a[0] + 0xfd).into_ret());
        e.register(V_FIELD_E0, |e, a| e.mem.u32(a[0] + 0xe0).into_ret());
        e
    }

    // ---- the quest target and log commands ------------------------------------------

    #[test]
    fn the_active_quest_is_read_from_the_player() {
        let mut e = engine3();
        let player = e.mem.u32(PLAYER);
        e.mem.set_u32(player + 0x6b8, 0x1234);
        assert_eq!(e.call(0x005c_bb50, &args![player]).u32(), 0x1234);
    }

    #[test]
    fn a_quest_target_gives_its_first_door_or_its_own_reference() {
        let mut e = engine3();
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + 0x10 * a[1]).into_ret()
        });
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0xc, 0xaaa);
        // No doors: the target's own reference.
        assert_eq!(e.call(0x005c_bb70, &args![target]).u32(), 0xaaa);
        // Doors (the array at +0x20, buffer at +4, size at +8): the first.
        let links = e.mem.alloc(0x20);
        e.mem.set_u32(links, 0xd01);
        e.mem.set_u32(links + 0x10, 0xd02);
        e.mem.set_u32(target + 0x24, links);
        e.mem.set_u32(target + 0x28, 2);
        assert_eq!(e.call(0x005c_bb70, &args![target]).u32(), 0xd01);
    }

    #[test]
    fn the_text_copy_forwards_to_the_multibyte_copy() {
        let mut e = engine3();
        e.register(MBS_NB_CPY, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x005c_bd90, &args![0x10u32, 0x20u32, 0x103u32])
                .u32(),
            0x10
        );
        assert_eq!(calls(&e, MBS_NB_CPY), vec![vec![0x10, 0x20, 0x103]]);
    }

    /// Doubles for the quest log commands. A list node holds its entry at
    /// `+0` and the next node at `+4` (the list head is the first node). An
    /// entry has its date at `+0x68` (see [`log_entry`]), a holder at `+0x6c`
    /// (keeping the name text at `+0x30`) and its text pointer at `+0x70`.
    fn quest_log_engine() -> Engine {
        let mut e = engine3();
        accept(
            &mut e,
            &[
                LIST_HEAD_CONSTRUCT,
                LIST_HEAD_DESTROY,
                MEMSET,
                MBS_NB_CPY,
                CONSOLE_PRINT,
            ],
        );
        e.register(NODE_DATA_POINTER, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTRY_GET_FIELD_6C, |e, a| e.mem.u32(a[0] + 0x6c).into_ret());
        e.register(ENTRY_GET_TEXT, |e, a| {
            assert_eq!(a[1], e.mem.u32(a[0] + 0x6c));
            e.mem.u32(a[0] + 0x70).into_ret()
        });
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x68).into_ret()
        });
        e.register(DATE_GET_WORD_2, |e, a| e.mem.u16(a[0] + 2).into_ret());
        e.register(DATE_GET_MONTH_INDEX, |e, a| e.mem.u16(a[0]).into_ret());
        e.register(DATE_GET_DAY, |e, a| e.mem.u16(a[0] + 4).into_ret());
        e.register(BS_STRING_TEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(STRING_TEXT_OR_EMPTY, |e, a| e.mem.u32(a[0]).into_ret());
        // Month 2 is named by a string object whose text is 0x7777.
        let month = e.mem.alloc(8);
        e.mem.set_u32(month + 4, 0x7777);
        e.mem.set_u32(MONTH_TABLE + 8, month);
        e
    }

    /// A quest log entry: its date is month 2, day 15, second word `year`.
    fn log_entry(e: &mut Engine, year: u16, name: u32, text: u32) -> u32 {
        let entry = e.mem.alloc(0x100);
        let date = e.mem.alloc(8);
        e.mem.set_u16(date, 2);
        e.mem.set_u16(date + 2, year);
        e.mem.set_u16(date + 4, 15);
        e.mem.set_u32(entry + 0x68, date);
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(holder + 0x30, name);
        e.mem.set_u32(entry + 0x6c, holder);
        e.mem.set_u32(entry + 0x70, text);
        entry
    }

    #[test]
    fn the_quest_log_command_prints_every_entry() {
        let mut e = quest_log_engine();
        let first = log_entry(&mut e, 277, 0x5001, 0x6001);
        let second = log_entry(&mut e, 278, 0x5002, 0x6002);
        e.register_double(QUEST_LOG_FILL_LIST, move |e, a| {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, second);
            e.mem.set_u32(a[0], first);
            e.mem.set_u32(a[0] + 4, node);
            Ret::default()
        });
        parse_gives(&mut e, true, &[1]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bbb0, a));
        // The list is filled for all quests, with the flag.
        let fill = calls(&e, QUEST_LOG_FILL_LIST);
        assert_eq!(fill.len(), 1);
        assert_eq!(fill[0][1..], [0, 1, 0, 0]);
        // Each entry: the text buffer is cleared (0x104 bytes) and filled
        // with at most 0x103 bytes of the entry's text.
        let memset = calls(&e, MEMSET);
        let copies = calls(&e, MBS_NB_CPY);
        assert_eq!(memset.len(), 2);
        assert_eq!(memset[0][1..], [0, 0x104]);
        assert_eq!(copies[0][1..], [0x6001, 0x103]);
        assert_eq!(copies[1][1..], [0x6002, 0x103]);
        let buffer = memset[0][0];
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![
                vec![MSG_DASHES],
                vec![FORMAT_DATE, 15, 0x7777, 277],
                vec![FORMAT_TEXT, 0x5001],
                vec![FORMAT_TEXT, buffer],
                vec![MSG_DASHES],
                vec![FORMAT_DATE, 15, 0x7777, 278],
                vec![FORMAT_TEXT, 0x5002],
                vec![FORMAT_TEXT, buffer],
            ]
        );
        assert_eq!(calls(&e, LIST_HEAD_DESTROY).len(), 1);
        // A flag of 0 is passed as 0; parameters that do not parse: false.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bbb0, a));
        assert_eq!(calls(&e, QUEST_LOG_FILL_LIST)[0][1..], [0, 0, 0, 0]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_bbb0, a));
        assert!(calls(&e, LIST_HEAD_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_single_quest_log_command_restricts_the_list_to_the_quest() {
        let mut e = quest_log_engine();
        let entry = log_entry(&mut e, 300, 0x5003, 0x6003);
        e.register_double(QUEST_LOG_FILL_LIST, move |e, a| {
            e.mem.set_u32(a[0], entry);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        parse_gives(&mut e, true, &[0x4242]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bdb0, a));
        let fill = calls(&e, QUEST_LOG_FILL_LIST);
        assert_eq!(fill[0][1..], [0x4242, 0, 0, 0]);
        let prints = calls(&e, CONSOLE_PRINT);
        assert_eq!(prints.len(), 4);
        assert_eq!(prints[1], vec![FORMAT_DATE, 15, 0x7777, 300]);
        assert_eq!(prints[2], vec![FORMAT_TEXT, 0x5003]);
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_bdb0, a));
    }

    #[test]
    fn the_quest_log_walk_ends_at_a_node_without_data() {
        let mut e = quest_log_engine();
        let entry = log_entry(&mut e, 300, 0x5003, 0x6003);
        e.register_double(QUEST_LOG_FILL_LIST, move |e, a| {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, node);
            Ret::default()
        });
        parse_gives(&mut e, true, &[0]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bbb0, a));
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        assert_eq!(calls(&e, LIST_HEAD_DESTROY).len(), 1);
    }

    // ---- Lock and Unlock -------------------------------------------------------------

    /// Doubles of the lock commands. The reference's form is the dword at
    /// `+0x20`; its type is the byte at `+4` of the form; the lock of a
    /// reference is the dword at `+0xd0`; the door teleport data the dword at
    /// `+0xd4`.
    fn lock_engine() -> Engine {
        let mut e = engine3();
        accept(
            &mut e,
            &[
                SET_OPEN_STATE,
                TERMINAL_FN_00501A80,
                TERMINAL_SET_HACKED,
                TERMINAL_FN_00501B40,
                TERMINAL_LOCK_OUT,
                LOCK_SET_LOCKED,
                ADD_LOCK_CHANGE,
                SET_CONNECTED_CELLS_PUBLIC,
                CONSOLE_PRINT,
            ],
        );
        e.register(HAS_ACTION, |_, a| (a[1] == 4).into_ret());
        e.register(GET_LOCK, |e, a| e.mem.u32(a[0] + 0xd0).into_ret());
        e.register(ADD_LOCK, |e, a| {
            let lock = e.mem.alloc(8);
            e.mem.set_u8(lock, 0xff);
            e.mem.set_u32(a[0] + 0xd0, lock);
            lock.into_ret()
        });
        e.register(BGS_SAVE_FORM_BUFFER_GET_FORM, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(FORM_GET_TYPE, |e, a| e.mem.u8(a[0] + 4).into_ret());
        e.register(GET_DOOR_TELEPORT_DATA, |e, a| {
            e.mem.u32(a[0] + 0xd4).into_ret()
        });
        e.register(REFERENCE_GET_NAME, |_, _| 0x5eed.into_ret());
        e
    }

    /// A reference with a form of the given type, optionally with a lock
    /// whose first byte is `0xff` and a teleport data.
    fn lockable(e: &mut Engine, form_type: u8, locked: bool, teleport: u32) -> u32 {
        let reference = object_sized(e, 0x100, false, &[(0xc8, V_A)]);
        let form = e.mem.alloc(0x10);
        e.mem.set_u8(form + 4, form_type);
        e.mem.set_u32(reference + 0x20, form);
        if locked {
            let lock = e.mem.alloc(8);
            e.mem.set_u8(lock, 0xff);
            e.mem.set_u32(reference + 0xd0, lock);
        }
        e.mem.set_u32(reference + 0xd4, teleport);
        reference
    }

    #[test]
    fn lock_sets_the_level_of_an_existing_lock() {
        let mut e = lock_engine();
        let reference = lockable(&mut e, 0x10, true, 0x7e1);
        let lock = e.mem.u32(reference + 0xd0);
        parse_gives(&mut e, true, &[3, 1]);
        set_echo(&mut e, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bf80, a));
        assert_eq!(e.mem.u8(lock), 3);
        assert_eq!(calls(&e, HAS_ACTION), vec![vec![reference, 4]]);
        assert_eq!(calls(&e, SET_OPEN_STATE), vec![vec![reference, 0, 1]]);
        assert_eq!(calls(&e, LOCK_SET_LOCKED), vec![vec![lock, 1]]);
        assert_eq!(calls(&e, ADD_LOCK_CHANGE), vec![vec![reference]]);
        // The flag is above 0: the connected cells are made non-public.
        assert_eq!(
            calls(&e, SET_CONNECTED_CELLS_PUBLIC),
            vec![vec![0x7e1, reference, 0]]
        );
        assert_eq!(calls(&e, V_A), vec![vec![reference, 1]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_LOCKED, 0x5eed, 3]]
        );
        assert!(calls(&e, ADD_LOCK).is_empty());
    }

    #[test]
    fn lock_adds_a_lock_and_keeps_the_old_level_for_a_zero_level() {
        let mut e = lock_engine();
        // No teleport data, level 0, flag 0 (not above 0).
        let reference = lockable(&mut e, 0x10, false, 0);
        parse_gives(&mut e, true, &[0, 0]);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bf80, a));
        let lock = e.mem.u32(reference + 0xd0);
        assert_eq!(calls(&e, ADD_LOCK), vec![vec![reference]]);
        assert_eq!(e.mem.u8(lock), 0xff);
        assert_eq!(calls(&e, LOCK_SET_LOCKED), vec![vec![lock, 1]]);
        assert!(calls(&e, GET_DOOR_TELEPORT_DATA).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // A flag above 0 but no teleport data: nothing made non-public.
        let reference = lockable(&mut e, 0x10, true, 0);
        parse_gives(&mut e, true, &[0, 5]);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_bf80, a));
        assert_eq!(calls(&e, GET_DOOR_TELEPORT_DATA).len(), 1);
        assert!(calls(&e, SET_CONNECTED_CELLS_PUBLIC).is_empty());
    }

    #[test]
    fn lock_hands_a_terminal_its_clamped_level() {
        let mut e = lock_engine();
        for (level, clamped) in [(9i32, 5u32), (-4, 0), (2, 2)] {
            let terminal = lockable(&mut e, 0x17, false, 0);
            let form = e.mem.u32(terminal + 0x20);
            parse_gives(&mut e, true, &[level as u32, 1]);
            set_echo(&mut e, true);
            let a = script(&mut e, terminal);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_bf80, a));
            assert_eq!(
                calls(&e, TERMINAL_FN_00501A80),
                vec![vec![form, terminal, clamped]]
            );
            assert_eq!(calls(&e, TERMINAL_SET_HACKED), vec![vec![terminal, 0]]);
            // No lock is added and nothing else happens to it.
            assert!(calls(&e, ADD_LOCK).is_empty());
            assert!(calls(&e, LOCK_SET_LOCKED).is_empty());
            assert!(calls(&e, V_A).is_empty());
            assert_eq!(
                calls(&e, CONSOLE_PRINT),
                vec![vec![FORMAT_LOCKED, 0x5eed, clamped]]
            );
        }
    }

    #[test]
    fn lock_needs_a_reference_and_parameters() {
        let mut e = lock_engine();
        parse_gives(&mut e, true, &[1, 1]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_bf80, a));
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        let reference = lockable(&mut e, 0x10, true, 0);
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_bf80, a));
        assert!(calls(&e, GET_LOCK).is_empty());
    }

    #[test]
    fn unlock_unlocks_an_existing_lock() {
        let mut e = lock_engine();
        let reference = lockable(&mut e, 0x10, true, 0x7e1);
        let lock = e.mem.u32(reference + 0xd0);
        parse_gives(&mut e, true, &[1]);
        set_echo(&mut e, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c120, a));
        assert_eq!(calls(&e, LOCK_SET_LOCKED), vec![vec![lock, 0]]);
        assert_eq!(calls(&e, ADD_LOCK_CHANGE), vec![vec![reference]]);
        assert_eq!(
            calls(&e, SET_CONNECTED_CELLS_PUBLIC),
            vec![vec![0x7e1, reference, 1]]
        );
        assert_eq!(calls(&e, V_A), vec![vec![reference, 1]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_UNLOCKED, 0x5eed]]
        );
        // A flag of 0 leaves the connected cells alone.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c120, a));
        assert!(calls(&e, SET_CONNECTED_CELLS_PUBLIC).is_empty());
        assert!(calls(&e, GET_DOOR_TELEPORT_DATA).is_empty());
    }

    #[test]
    fn unlock_hands_an_unlocked_terminal_to_the_lock_out_functions() {
        let mut e = lock_engine();
        let terminal = lockable(&mut e, 0x17, false, 0);
        let form = e.mem.u32(terminal + 0x20);
        parse_gives(&mut e, true, &[0]);
        let a = script(&mut e, terminal);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c120, a));
        assert_eq!(calls(&e, TERMINAL_FN_00501B40), vec![vec![form, terminal]]);
        assert_eq!(calls(&e, TERMINAL_LOCK_OUT), vec![vec![terminal, 0]]);
        assert!(calls(&e, LOCK_SET_LOCKED).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Any other unlocked reference: nothing happens.
        let thing = lockable(&mut e, 0x10, false, 0);
        let a = script(&mut e, thing);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c120, a));
        assert!(calls(&e, TERMINAL_LOCK_OUT).is_empty());
        assert!(calls(&e, LOCK_SET_LOCKED).is_empty());
    }

    #[test]
    fn unlock_needs_a_reference_and_a_parameter() {
        let mut e = lock_engine();
        parse_gives(&mut e, true, &[1]);
        let a = script(&mut e, 0);
        assert!(!run(&mut e, 0x005c_c120, a));
        let reference = lockable(&mut e, 0x10, true, 0);
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_c120, a));
        assert!(calls(&e, GET_LOCK).is_empty());
    }

    // ---- small commands ---------------------------------------------------------------

    #[test]
    fn the_005cc240_command_asks_the_process_lists() {
        let mut e = engine3();
        e.register(PROCESS_LISTS_FN_00972600, |_, a| (a[1] == 9).into_ret());
        let actor = object_sized(&mut e, 0x100, true, &[]);
        parse_gives(&mut e, true, &[1, 2, 9]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c240, a));
        // v2, v1, the actor, v0, 0, -1 (the arguments in order are v0..v2).
        assert_eq!(
            calls(&e, PROCESS_LISTS_FN_00972600),
            vec![vec![PROCESS_LISTS, 9, 2, actor, 1, 0, 0xffff_ffff]]
        );
        assert_eq!(result_of(&e, a), 1.0);
        // The process lists say no: the result stays 0.0.
        parse_gives(&mut e, true, &[1, 2, 8]);
        let a = script(&mut e, actor);
        assert!(run(&mut e, 0x005c_c240, a));
        assert_eq!(result_of(&e, a), 0.0);
        // Not an actor: the cast gives 0.
        let thing = object_sized(&mut e, 0x100, false, &[]);
        let a = script(&mut e, thing);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c240, a));
        assert_eq!(calls(&e, PROCESS_LISTS_FN_00972600)[0][3], 0);
        // Parameters that do not parse: false, result zeroed.
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, actor);
        assert!(!run(&mut e, 0x005c_c240, a));
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn get_crime_forwards_to_its_condition_function() {
        let mut e = engine3();
        e.register(GET_CRIME_CONDITION, |_, a| (a[1] == 5).into_ret());
        parse_gives(&mut e, true, &[5, 6]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c2f0, a));
        assert_eq!(
            calls(&e, GET_CRIME_CONDITION),
            vec![vec![0x40, 5, 6, a.result.addr()]]
        );
        assert_eq!(result_of(&e, a), 0.0);
        // The condition function answers false.
        parse_gives(&mut e, true, &[4, 6]);
        assert!(!run(&mut e, 0x005c_c2f0, a));
        // The second argument is preset to -1.
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), 0);
            assert_eq!(e.mem.u32(a[8]), 0xffff_ffff);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(!run(&mut e, 0x005c_c2f0, a));
        // No reference: true without a call. Unparsed: false.
        parse_gives(&mut e, true, &[5, 6]);
        let none = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c2f0, none));
        assert!(calls(&e, GET_CRIME_CONDITION).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_c2f0, a));
    }

    /// A command that is a condition function called with `(thisObj, 0, 0,
    /// result)`; `default` is its answer without `thisObj`.
    fn check_this_or_default(addr: u32, callee: u32, default: bool) {
        let mut e = engine3();
        e.register(callee, |_, a| (a[0] == 0x40).into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        assert_eq!(calls(&e, callee), vec![vec![0x40, 0, 0, a.result.addr()]]);
        // The callee's answer is the command's.
        let other = script(&mut e, 0x41);
        assert!(!run(&mut e, addr, other));
        // Without thisObj nothing is called and the default is answered.
        let none = script(&mut e, 0);
        start_log(&mut e);
        assert_eq!(run(&mut e, addr, none), default);
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn the_005cc370_command_is_is_guard_or_false() {
        check_this_or_default(0x005c_c370, IS_GUARD_CONDITION, false);
    }

    #[test]
    fn the_005cc3a0_command_is_is_talking_or_false() {
        check_this_or_default(0x005c_c3a0, IS_TALKING_CONDITION, false);
    }

    #[test]
    fn the_005cc3d0_command_is_get_walk_speed_or_false() {
        check_this_or_default(0x005c_c3d0, GET_WALK_SPEED_CONDITION, false);
    }

    #[test]
    fn the_005cc400_command_is_its_condition_function_or_true() {
        check_this_or_default(0x005c_c400, FN_005A1210, true);
    }

    #[test]
    fn the_005cc430_command_is_its_condition_function_or_true() {
        check_this_or_default(0x005c_c430, FN_005A1770, true);
    }

    #[test]
    fn the_005cc460_command_is_is_trespassing_or_true() {
        check_this_or_default(0x005c_c460, IS_TRESPASSING_CONDITION, true);
    }

    #[test]
    fn the_005cc490_command_is_is_in_my_owned_cell_or_true() {
        check_this_or_default(0x005c_c490, IS_IN_MY_OWNED_CELL_CONDITION, true);
    }

    #[test]
    fn the_005cc4c0_command_is_is_continuing_package_pc_near_or_true() {
        check_this_or_default(0x005c_c4c0, IS_CONTINUING_PACKAGE_PC_NEAR_CONDITION, true);
    }

    #[test]
    fn the_005cc910_command_is_can_have_flames_or_true() {
        check_this_or_default(0x005c_c910, CAN_HAVE_FLAMES_CONDITION, true);
    }

    #[test]
    fn the_005cc940_command_is_has_flames_or_true() {
        check_this_or_default(0x005c_c940, HAS_FLAMES_CONDITION, true);
    }

    #[test]
    fn the_005ccaf0_command_is_get_open_state_or_true() {
        check_this_or_default(0x005c_caf0, GET_OPEN_STATE_CONDITION, true);
    }

    /// A command that calls `callee(0, 0, 0, result)` and answers what it
    /// answers.
    fn check_null_condition(addr: u32, callee: u32) {
        let mut e = engine3();
        e.register(callee, |_, a| (a[3] != 0).into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, addr, a));
        assert_eq!(calls(&e, callee), vec![vec![0, 0, 0, a.result.addr()]]);
        e.register(callee, |_, _| false.into_ret());
        assert!(!run(&mut e, addr, a));
    }

    #[test]
    fn the_005cd570_command_is_get_day_of_week() {
        check_null_condition(0x005c_d570, GET_DAY_OF_WEEK_CONDITION);
    }

    #[test]
    fn the_005cd590_command_is_is_pc_sleeping() {
        check_null_condition(0x005c_d590, IS_PC_SLEEPING_CONDITION);
    }

    #[test]
    fn the_005cd630_command_is_is_pc_a_murderer() {
        check_null_condition(0x005c_d630, IS_PC_A_MURDERER_CONDITION);
    }

    // ---- the package commands ---------------------------------------------------------

    /// The scene of the package commands: an actor (flag at `+0xff`) whose
    /// process (found through `GetSavedAcquireObject`, the dword at `+0xf8`)
    /// answers slot `0x22c` with its dword at `+0xe0` (the current package).
    /// The actor's position (slot `0x1f4`) is `+0xe4`, its rotation `+0x24`,
    /// its cell the dword at `+0x40`, its world space the dword at `+0xd0`.
    /// The package keeps its procedure type at `+0x18`.
    fn package_engine() -> (Engine, u32, u32, u32) {
        let mut e = engine3();
        accept(
            &mut e,
            &[
                SET_ACTION_FLAG,
                PACKAGE_CALCULATE_PROCEDURE_TYPE,
                SET_PACKAGE_START_LOCATION,
                FN_00674E00,
                FN_00693D50,
                PROCESS_LISTS_REMOVE_REFERENCE,
                PROCESS_LISTS_ADD_REFERENCE,
                V_A,
                V_B,
            ],
        );
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(IS_IN_DIALOGUE, |e, a| {
            (e.mem.u8(a[0] + 0xfc) != 0).into_ret()
        });
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0xf8).into_ret()
        });
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(FN_00569B80, |_, _| 0x1357.into_ret());
        e.register(REFERENCE_ROTATION, |_, a| (a[0] + 0x24).into_ret());
        e.register(REFERENCE_PARENT_CELL, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        e.register(REFERENCE_GET_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + 0xd0).into_ret()
        });
        e.register(PROCESS_GET_FIELD_28, |e, a| {
            e.mem.u32(a[0] + 0x28).into_ret()
        });
        e.register(CALENDAR_GET_HOUR, |_, a| {
            assert_eq!(a, [CALENDAR]);
            14.5f64.into_ret()
        });
        e.set_global::<f64>(ONE, 1.0);
        let process = object_sized(
            &mut e,
            0x100,
            false,
            &[
                (0x22c, V_FIELD_E0),
                (0x59c, V_C),
                (0x5a8, V_D),
                (0x598, V_A),
            ],
        );
        let actor = object_sized(
            &mut e,
            0x100,
            true,
            &[(0x288, V_B), (0x1f4, 0x0900_0002), (0x2f4, V_B)],
        );
        e.register(0x0900_0002, |_, a| (a[0] + 0xe4).into_ret());
        e.mem.set_u32(actor + 0xf8, process);
        e.mem.set_f32(actor + 0x24 + 8, 1.5);
        e.mem.set_u32(actor + 0x40, 0xce11);
        e.mem.set_u32(actor + 0xd0, 0x5ace);
        let package = e.mem.alloc(0x40);
        e.mem.set_u32(package + 0x18, 0xffff_ffff);
        (e, actor, process, package)
    }

    #[test]
    fn the_005cc4f0_command_replaces_the_actors_package() {
        let (mut e, actor, process, package) = package_engine();
        let old = e.mem.alloc(0x10);
        e.mem.set_u32(process + 0xe0, old);
        e.mem.set_u32(process + 0x28, 1);
        e.mem.set_u8(actor + 0xfc, 1);
        parse_gives(&mut e, true, &[package]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c4f0, a));
        // A dialogue in progress is ended first (slot 0x288).
        assert_eq!(calls(&e, V_B)[0], vec![actor]);
        let list = actor + 0x44;
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![vec![old, list, 0x800], vec![package, list, 0x200]]
        );
        // The old package goes back to the process (slot 0x59c), the
        // process is told 0 (slot 0x5a8), the new package is given (0x598).
        assert_eq!(calls(&e, V_C), vec![vec![process, actor, old]]);
        assert_eq!(calls(&e, V_D), vec![vec![process, 0]]);
        assert_eq!(calls(&e, V_A), vec![vec![process, actor, package]]);
        // The procedure type was -1: calculated.
        assert_eq!(
            calls(&e, PACKAGE_CALCULATE_PROCEDURE_TYPE),
            vec![vec![package, 0x1357]]
        );
        // The start location: world space, cell, position (slot 0x1f4) and
        // the third rotation word of the actor.
        assert_eq!(
            calls(&e, SET_PACKAGE_START_LOCATION),
            vec![vec![actor, 0x5ace, 0xce11, actor + 0xe4, 1.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, FN_00674E00), vec![vec![package, 1]]);
        assert_eq!(calls(&e, V_B)[1], vec![actor, package, 0, 0]);
        // The process field is 1: the hour minus one goes to the process and
        // the actor leaves and joins the process lists again.
        assert_eq!(
            calls(&e, FN_00693D50),
            vec![vec![process, 13.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, actor, 1]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, actor, 1, 0, 0, 0]]
        );
    }

    #[test]
    fn the_005cc4f0_command_keeps_a_calculated_type_and_other_process_states() {
        let (mut e, actor, process, package) = package_engine();
        // No current package, a known procedure type, process field 2.
        e.mem.set_u32(package + 0x18, 4);
        e.mem.set_u32(process + 0x28, 2);
        parse_gives(&mut e, true, &[package]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c4f0, a));
        assert_eq!(calls(&e, SET_ACTION_FLAG).len(), 1);
        assert!(calls(&e, V_C).is_empty());
        assert!(calls(&e, PACKAGE_CALCULATE_PROCEDURE_TYPE).is_empty());
        assert!(calls(&e, V_B).len() == 1);
        assert_eq!(calls(&e, FN_00693D50).len(), 1);
        assert!(calls(&e, PROCESS_LISTS_REMOVE_REFERENCE).is_empty());
        assert!(calls(&e, PROCESS_LISTS_ADD_REFERENCE).is_empty());
    }

    #[test]
    fn the_005cc4f0_command_flags_the_player_when_the_process_has_no_value() {
        let (mut e, actor, process, package) = package_engine();
        e.mem.set_u32(process + 0x28, 0);
        parse_gives(&mut e, true, &[package]);
        // Another actor: no flag byte anywhere.
        let a = script(&mut e, actor);
        assert!(run(&mut e, 0x005c_c4f0, a));
        // The player: the byte at +0x79b is set.
        e.mem.set_u32(PLAYER, actor);
        e.mem.set_u8(actor + 0x79b, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c4f0, a));
        assert_eq!(e.mem.u8(actor + 0x79b), 1);
        assert!(calls(&e, FN_00693D50).is_empty());
    }

    #[test]
    fn the_005cc4f0_command_does_nothing_without_an_actor_a_package_or_a_process() {
        let (mut e, actor, _, package) = package_engine();
        let thing = object_sized(&mut e, 0x100, false, &[]);
        // A package that is missing, then a thisObj that is no actor, then
        // an actor without a process.
        parse_gives(&mut e, true, &[0]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c4f0, a));
        parse_gives(&mut e, true, &[package]);
        let a = script(&mut e, thing);
        assert!(run(&mut e, 0x005c_c4f0, a));
        e.mem.set_u32(actor + 0xf8, 0);
        let a = script(&mut e, actor);
        assert!(run(&mut e, 0x005c_c4f0, a));
        assert!(calls(&e, SET_ACTION_FLAG).is_empty());
        assert!(calls(&e, V_A).is_empty());
        // Parameters that do not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_c4f0, a));
    }

    #[test]
    fn the_player_ai_flag_setter_stores_the_byte() {
        let mut e = engine3();
        let player = e.mem.u32(PLAYER);
        e.call(0x005c_c7a0, &args![player, 1u8]);
        assert_eq!(e.mem.u8(player + 0x79b), 1);
        e.call(0x005c_c7a0, &args![player, 0u8]);
        assert_eq!(e.mem.u8(player + 0x79b), 0);
    }

    /// The scene of the stop-package command: the actor's package set as
    /// current is the dword at `+0xd0`, created when the byte at `+0xd4` is
    /// set; its process is the dword at `+0xf8` with slots `0x234` and
    /// `0x24`.
    fn stop_package_engine() -> (Engine, u32, u32, u32) {
        let mut e = engine3();
        accept(
            &mut e,
            &[
                SET_ACTION_FLAG,
                REMOVE_PACKAGE_EXTRA,
                SPECIAL_IDLE_FREE,
                V_A,
                V_B,
                V_C,
            ],
        );
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(GET_PACKAGE_SET_AS_PCURRENT, |e, a| {
            e.mem.u32(a[0] + 0xd0).into_ret()
        });
        e.register(PACKAGE_GET_IS_CREATED, |e, a| {
            (e.mem.u8(a[0] + 0xd4) != 0).into_ret()
        });
        e.register(GET_PACKAGE_EXTRA, |_, a| (a[0] + 0x1000).into_ret());
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0xf8).into_ret()
        });
        e.register(PLAYER_GET_ANIMATION, |_, a| (0xa00 + a[1]).into_ret());
        let process = object_sized(&mut e, 0x100, false, &[(0x234, V_A), (0x24, V_B)]);
        let actor = object_sized(&mut e, 0x100, true, &[(0x434, V_C)]);
        e.mem.set_u32(actor + 0xf8, process);
        let package = e.mem.alloc(0x40);
        e.mem.set_u32(actor + 0xd0, package);
        (e, actor, process, package)
    }

    #[test]
    fn the_005cc7c0_command_stops_the_package_through_the_process() {
        let (mut e, actor, process, package) = stop_package_engine();
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c7c0, a));
        assert_eq!(calls(&e, V_C), vec![vec![actor, 0]]);
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![vec![package, actor + 0x44, 0x400]]
        );
        assert_eq!(calls(&e, V_A), vec![vec![process]]);
        assert_eq!(calls(&e, V_B), vec![vec![process, actor, 1]]);
        assert!(calls(&e, REMOVE_PACKAGE_EXTRA).is_empty());
        // Not the player: no special idles freed.
        assert!(calls(&e, SPECIAL_IDLE_FREE).is_empty());
    }

    #[test]
    fn the_005cc7c0_command_flags_the_extra_of_a_created_package() {
        let (mut e, actor, _, package) = stop_package_engine();
        e.mem.set_u8(package + 0xd4, 1);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c7c0, a));
        // The package's extra is flagged and removed from the actor's list.
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![vec![actor + 0x44 + 0x1000, actor + 0x44, 0x400]]
        );
        assert_eq!(calls(&e, REMOVE_PACKAGE_EXTRA), vec![vec![actor + 0x44]]);
        assert!(calls(&e, V_A).is_empty());
        assert!(calls(&e, V_B).is_empty());
    }

    #[test]
    fn the_005cc7c0_command_frees_the_special_idles_of_the_player() {
        let (mut e, actor, _, _) = stop_package_engine();
        e.mem.set_u32(PLAYER, actor);
        e.mem.set_u8(actor + 0x79b, 1);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c7c0, a));
        assert_eq!(e.mem.u8(actor + 0x79b), 0);
        assert_eq!(
            calls(&e, PLAYER_GET_ANIMATION),
            vec![vec![actor, 1], vec![actor, 0]]
        );
        assert_eq!(
            calls(&e, SPECIAL_IDLE_FREE),
            vec![vec![0xa01, 1, 0], vec![0xa00, 1, 0]]
        );
        // No actor: nothing happens, still true.
        let thing = object_sized(&mut e, 0x100, false, &[]);
        let a = script(&mut e, thing);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c7c0, a));
        assert!(calls(&e, V_C).is_empty());
    }

    // ---- flames -------------------------------------------------------------------------

    /// A reference whose 3D object (slot `0x1d0`) is the dword at `+0xe0`,
    /// itself an object whose slot `0xc` answers its dword at `+0xe0`: the
    /// node.
    fn flames_engine(with_object: bool, node: u32) -> (Engine, u32) {
        let mut e = engine3();
        accept(
            &mut e,
            &[
                UPDATE_PROPERTIES,
                NI_UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                CONSOLE_PRINT,
            ],
        );
        let object = object_sized(&mut e, 0x100, false, &[(0xc, V_FIELD_E0)]);
        e.mem.set_u32(object + 0xe0, node);
        let reference = object_sized(&mut e, 0x100, false, &[(0x1d0, V_FIELD_E0)]);
        e.mem
            .set_u32(reference + 0xe0, if with_object { object } else { 0 });
        (e, reference)
    }

    #[test]
    fn add_flames_updates_the_node_when_nodes_were_added() {
        let (mut e, reference) = flames_engine(true, 0x40de);
        e.register(ADD_ADDON_NODES, |_, _| 1u8.into_ret());
        set_echo(&mut e, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c970, a));
        assert_eq!(result_of(&e, a), 1.0);
        assert_eq!(calls(&e, ADD_ADDON_NODES), vec![vec![0x40de]]);
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![0x40de]]);
        let construct = calls(&e, NI_UPDATE_DATA_CONSTRUCT);
        assert_eq!(construct[0][1..], [0, 1, 0]);
        assert_eq!(calls(&e, NODE_UPDATE), vec![vec![0x40de, construct[0][0]]]);
        // "AddFlames >> %0.2f" with the result as a double (two words).
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_ADD_FLAMES, 0, 0x3ff0_0000]]
        );
    }

    #[test]
    fn add_flames_does_not_update_when_nothing_was_added() {
        let (mut e, reference) = flames_engine(true, 0x40de);
        e.register(ADD_ADDON_NODES, |_, _| 0u8.into_ret());
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c970, a));
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, UPDATE_PROPERTIES).is_empty());
        assert!(calls(&e, NODE_UPDATE).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn add_flames_without_a_3d_object_or_a_reference_only_answers_true() {
        let (mut e, reference) = flames_engine(false, 0x40de);
        e.register(ADD_ADDON_NODES, |_, _| 1u8.into_ret());
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c970, a));
        assert_eq!(result_of(&e, a), 7.0);
        let none = script(&mut e, 0);
        assert!(run(&mut e, 0x005c_c970, none));
        assert!(calls(&e, ADD_ADDON_NODES).is_empty());
        // An object without a node: the add function gets 0.
        let (mut e, reference) = flames_engine(true, 0);
        e.register(ADD_ADDON_NODES, |_, _| 0u8.into_ret());
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_c970, a));
        assert_eq!(calls(&e, ADD_ADDON_NODES), vec![vec![0]]);
    }

    #[test]
    fn remove_flames_stores_the_answer_and_echoes_it() {
        let (mut e, reference) = flames_engine(true, 0x40de);
        e.register(REMOVE_ADDON_NODES, |_, _| 1u8.into_ret());
        set_echo(&mut e, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ca50, a));
        assert_eq!(result_of(&e, a), 1.0);
        assert_eq!(calls(&e, REMOVE_ADDON_NODES), vec![vec![0x40de]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_REMOVE_FLAMES, 0, 0x3ff0_0000]]
        );
    }

    #[test]
    fn remove_flames_needs_a_3d_object_and_echoes_the_untouched_result_otherwise() {
        let (mut e, reference) = flames_engine(false, 0x40de);
        e.register(REMOVE_ADDON_NODES, |_, _| 1u8.into_ret());
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ca50, a));
        assert!(calls(&e, REMOVE_ADDON_NODES).is_empty());
        assert_eq!(result_of(&e, a), 7.0);
        let none = script(&mut e, 0);
        assert!(run(&mut e, 0x005c_ca50, none));
        // An object whose node slot answers 0: removal gets 0 and the echo
        // shows the answer.
        let (mut e, reference) = flames_engine(true, 0);
        e.register(REMOVE_ADDON_NODES, |_, _| 0u8.into_ret());
        set_echo(&mut e, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_ca50, a));
        assert_eq!(calls(&e, REMOVE_ADDON_NODES), vec![vec![0]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_REMOVE_FLAMES, 0, 0]]
        );
    }

    // ---- MoveTo ---------------------------------------------------------------------------

    /// The callees of `MoveToFunctionBase`, as doubles. A reference keeps its
    /// cell at `+0x40`, world space at `+0xd0`, a process at `+0xf8`, a
    /// character controller at `+0xf4`; the answer of slot `0xfc` at `+0xfd`
    /// and of slot `0x100` (actor) at `+0xff`; its 3D object (slot `0x1d0`) at
    /// `+0xe0`. Its rotation is at `+0x24`, its position (slot `0x1f4`) at
    /// `+0xe4`. A cell is loaded when its byte at `+0` is 1. The doubles that
    /// receive an address of three floats copy them to `+0xb0` of the first
    /// argument (`SetLocationOnReference`) or of the controller.
    fn move_engine() -> Engine {
        let mut e = engine3();
        accept(
            &mut e,
            &[
                PROCESS_LISTS_FN_00972840,
                GET_OUT_OF_FURNITURE_QUICK,
                SET_ANGLE_Z,
                SET_FIELD_24,
                SET_TRANSLATE,
                SET_ROTATE,
                RESET_SIM,
                NI_UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                MOVE_REF_TO_NEW_SPACE,
                ACTOR_STOP_MOVING,
                ACTOR_FADE_IN,
                ACTOR_UPDATE_ALPHA,
                SCRIPT_ERROR,
                CHECK_FOR_QUEST_TARGET_UPDATE,
                CLOSE_CONSOLE,
                CONSOLE_PRINT,
                REQUEST_POSITION_PLAYER,
                V_A,
                V_B,
                V_C,
                V_D,
            ],
        );
        e.register(REFERENCE_PARENT_CELL, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        e.register(REFERENCE_GET_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + 0xd0).into_ret()
        });
        e.register(REFERENCE_ROTATION, |_, a| (a[0] + 0x24).into_ret());
        e.register(0x0900_0002, |_, a| (a[0] + 0xe4).into_ret());
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0xf8).into_ret()
        });
        e.register(IS_CELL_LOADED, |e, a| {
            (a[1] != 0 && e.mem.u8(a[1]) == 1).into_ret()
        });
        e.register(SET_LOCATION_ON_REFERENCE, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 0xb0 + 4 * i, word);
            }
            Ret::default()
        });
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(GET_CHAR_CONTROLLER, |e, a| {
            e.mem.u32(a[0] + 0xf4).into_ret()
        });
        e.register(CHAR_CONTROLLER_SET_POSITION, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 0xb0 + 4 * i, word);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(POSITION_REQUEST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CELL_FN_00425FD0, |e, a| {
            (e.mem.u8(a[0] + 1) == 1).into_ret()
        });
        e
    }

    /// A reference for the move tests with the given slots; `actor` sets the
    /// answer of slot `0x100`.
    fn movable(e: &mut Engine, actor: bool, flag: bool) -> u32 {
        let reference = object_sized(
            e,
            0x100,
            actor,
            &[
                (0xfc, V_FLAG_FD),
                (0x100, V_IS_ACTOR),
                (0x1d0, V_FIELD_E0),
                (0x1f4, 0x0900_0002),
                (0x1c4, V_A),
            ],
        );
        e.mem.set_u8(reference + 0xfd, flag as u8);
        reference
    }

    /// A target at cell `cell` (a heap object whose byte at `+0` is
    /// `loaded`), world space `0x5ace` (when `world`), rotation (0.5, 0.75,
    /// 1.25) and position (10, 20, 30).
    fn move_target(e: &mut Engine, loaded: bool, world: bool) -> u32 {
        let target = movable(e, false, false);
        let cell = e.mem.alloc(8);
        e.mem.set_u8(cell, loaded as u8);
        e.mem.set_u32(target + 0x40, cell);
        e.mem.set_u32(target + 0xd0, if world { 0x5ace } else { 0 });
        for (i, value) in [0.5f32, 0.75, 1.25].iter().enumerate() {
            e.mem.set_f32(target + 0x24 + 4 * i as u32, *value);
        }
        for (i, value) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(target + 0xe4 + 4 * i as u32, *value);
        }
        target
    }

    fn base_move(e: &mut Engine, reference: u32, target: u32) {
        e.call(
            0x005c_cb20,
            &args![reference, target, 1.0f32, 2.0f32, 3.0f32],
        );
    }

    #[test]
    fn move_base_places_a_reference_in_a_loaded_cell() {
        let mut e = move_engine();
        let player = e.mem.u32(PLAYER);
        let target = move_target(&mut e, true, true);
        let cell = e.mem.u32(target + 0x40);
        let reference = movable(&mut e, false, true);
        let node = object_sized(&mut e, 0x100, false, &[]);
        e.mem.set_u32(reference + 0xe0, node);
        let controller = e.mem.alloc(0x100);
        e.mem.set_u32(reference + 0xf4, controller);
        e.mem.set_u32(reference + 0x40, cell);
        start_log(&mut e);
        base_move(&mut e, reference, target);
        // The new position (target's plus the offsets) is handed to
        // SetLocationOnReference and the character controller.
        let expected = [11.0f32, 22.0, 33.0].map(f32::to_bits);
        for i in 0..3 {
            assert_eq!(e.mem.u32(reference + 0xb0 + 4 * i), expected[i as usize]);
            assert_eq!(e.mem.u32(controller + 0xb0 + 4 * i), expected[i as usize]);
        }
        // The target's cell is loaded: angle (third rotation word), field
        // 0x24 cleared, the node gets the translation and orientation, the
        // collision object is reset and the node updated.
        assert_eq!(
            calls(&e, SET_ANGLE_Z),
            vec![vec![reference, 1.25f32.to_bits()]]
        );
        assert_eq!(calls(&e, SET_FIELD_24), vec![vec![reference, 0]]);
        assert_eq!(calls(&e, SET_TRANSLATE), vec![vec![node, reference + 0xe4]]);
        let orientation = calls(&e, GET_ORIENTATION);
        assert_eq!(calls(&e, SET_ROTATE), vec![vec![node, orientation[0][1]]]);
        assert_eq!(calls(&e, RESET_SIM), vec![vec![node, 1]]);
        let construct = calls(&e, NI_UPDATE_DATA_CONSTRUCT);
        assert_eq!(construct[0][1..], [0, 0, 0]);
        assert_eq!(calls(&e, NODE_UPDATE), vec![vec![node, construct[0][0]]]);
        assert_eq!(
            calls(&e, MOVE_REF_TO_NEW_SPACE),
            vec![vec![reference, cell, 0x5ace]]
        );
        // The reference's own cell is loaded: slot 0x1c4.
        assert_eq!(calls(&e, V_A), vec![vec![reference]]);
        assert_eq!(
            calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE),
            vec![vec![player, reference]]
        );
        assert!(calls(&e, PROCESS_LISTS_FN_00972840).is_empty());
        assert!(calls(&e, REQUEST_POSITION_PLAYER).is_empty());
    }

    #[test]
    fn move_base_sends_a_reference_to_a_cell_that_is_not_loaded_far_away() {
        let mut e = move_engine();
        e.set_global::<u32>(FLOAT_MAX, 0x7f7f_ffff);
        let target = move_target(&mut e, false, false);
        let reference = movable(&mut e, false, true);
        let own_cell = e.mem.alloc(8);
        e.mem.set_u32(reference + 0x40, own_cell);
        start_log(&mut e);
        base_move(&mut e, reference, target);
        assert!(calls(&e, SET_ANGLE_Z).is_empty());
        assert!(calls(&e, SET_TRANSLATE).is_empty());
        // The flag of slot 0xfc is set: the far value goes into field 0x24.
        assert_eq!(calls(&e, SET_FIELD_24), vec![vec![reference, 0x7f7f_ffff]]);
        // Neither cell is loaded: no notification. The controller is absent.
        assert!(calls(&e, V_A).is_empty());
        assert!(calls(&e, ACTOR_STOP_MOVING).is_empty());
        assert_eq!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).len(), 1);
        // Without the flag nothing is set.
        let reference = movable(&mut e, false, false);
        start_log(&mut e);
        base_move(&mut e, reference, target);
        assert!(calls(&e, SET_FIELD_24).is_empty());
    }

    #[test]
    fn move_base_stops_and_fades_in_a_moved_actor() {
        let mut e = move_engine();
        let target = move_target(&mut e, true, false);
        let actor = movable(&mut e, true, false);
        let process = object_sized(
            &mut e,
            0x100,
            false,
            &[(0x28, V_B), (0x24, V_C), (0x4bc, V_FIELD_E0)],
        );
        e.mem.set_u32(process + 0xe0, 1);
        e.mem.set_u32(actor + 0xf8, process);
        start_log(&mut e);
        base_move(&mut e, actor, target);
        // The actor is removed from the process lists, got out of
        // furniture (the process says so), stopped, faded in and updated.
        assert_eq!(
            calls(&e, PROCESS_LISTS_FN_00972840),
            vec![vec![PROCESS_LISTS, actor, 0]]
        );
        assert_eq!(calls(&e, GET_OUT_OF_FURNITURE_QUICK), vec![vec![actor]]);
        assert_eq!(calls(&e, V_B), vec![vec![process]]);
        assert_eq!(calls(&e, ACTOR_STOP_MOVING), vec![vec![actor]]);
        assert_eq!(calls(&e, V_C), vec![vec![process, actor, 1]]);
        assert_eq!(calls(&e, ACTOR_FADE_IN), vec![vec![actor]]);
        assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![actor]]);
        assert!(calls(&e, SCRIPT_ERROR).is_empty());
        // A process that does not say so: no furniture call.
        e.mem.set_u32(process + 0xe0, 0);
        start_log(&mut e);
        base_move(&mut e, actor, target);
        assert!(calls(&e, GET_OUT_OF_FURNITURE_QUICK).is_empty());
    }

    #[test]
    fn move_base_reports_an_actor_without_a_process() {
        let mut e = move_engine();
        let target = move_target(&mut e, true, false);
        let actor = movable(&mut e, true, false);
        start_log(&mut e);
        base_move(&mut e, actor, target);
        assert_eq!(
            calls(&e, SCRIPT_ERROR),
            vec![vec![MSG_MOVED_ACTOR_HAS_NO_PROCESS]]
        );
        assert!(calls(&e, ACTOR_STOP_MOVING).is_empty());
        assert_eq!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).len(), 1);
    }

    #[test]
    fn move_base_requests_the_player_position() {
        let mut e = move_engine();
        let player = e.mem.u32(PLAYER);
        let target = move_target(&mut e, true, true);
        start_log(&mut e);
        base_move(&mut e, player, target);
        let request = calls(&e, REQUEST_POSITION_PLAYER);
        assert_eq!(request.len(), 1);
        assert_eq!(request[0][0], player);
        let request = request[0][1];
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x34]]);
        // World space, the new position, the target's first and third
        // rotation words, the flag byte and the target.
        assert_eq!(e.mem.u32(request), 0x5ace);
        assert_eq!(e.mem.u32(request + 4), 0);
        assert_eq!(e.mem.f32(request + 8), 11.0);
        assert_eq!(e.mem.f32(request + 0xc), 22.0);
        assert_eq!(e.mem.f32(request + 0x10), 33.0);
        assert_eq!(e.mem.f32(request + 0x14), 0.5);
        assert_eq!(e.mem.f32(request + 0x1c), 1.25);
        assert_eq!(e.mem.u8(request + 0x20), 1);
        assert_eq!(e.mem.u32(request + 0x2c), target);
        assert_eq!(calls(&e, CLOSE_CONSOLE).len(), 1);
        // The player is an actor: removed from the process lists first.
        assert_eq!(
            calls(&e, PROCESS_LISTS_FN_00972840),
            vec![vec![PROCESS_LISTS, player, 0]]
        );
        // The quest targets are checked at the end; no placing happened.
        assert_eq!(
            calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE),
            vec![vec![player, player]]
        );
        assert!(calls(&e, SET_LOCATION_ON_REFERENCE).is_empty());
    }

    #[test]
    fn move_base_names_the_cell_for_the_player_without_a_world_space() {
        let mut e = move_engine();
        let player = e.mem.u32(PLAYER);
        let target = move_target(&mut e, true, false);
        let cell = e.mem.u32(target + 0x40);
        start_log(&mut e);
        // A cell for which `00425fd0` says no: neither is named.
        base_move(&mut e, player, target);
        let request = calls(&e, REQUEST_POSITION_PLAYER)[0][1];
        assert_eq!(e.mem.u32(request), 0);
        assert_eq!(e.mem.u32(request + 4), 0);
        // An interior cell (the double's byte at +1).
        e.mem.set_u8(cell + 1, 1);
        start_log(&mut e);
        base_move(&mut e, player, target);
        let request = calls(&e, REQUEST_POSITION_PLAYER)[0][1];
        assert_eq!(e.mem.u32(request), 0);
        assert_eq!(e.mem.u32(request + 4), cell);
    }

    #[test]
    fn move_base_leaves_the_player_alone_when_its_slot_0x22c_says_so() {
        let mut e = move_engine();
        let player = e.mem.u32(PLAYER);
        let target = move_target(&mut e, true, true);
        e.mem.set_u32(player + 0xe0, 1);
        start_log(&mut e);
        base_move(&mut e, player, target);
        assert!(calls(&e, REQUEST_POSITION_PLAYER).is_empty());
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
        // A missing reference or target: nothing happens.
        e.mem.set_u32(player + 0xe0, 0);
        base_move(&mut e, 0, target);
        base_move(&mut e, player, 0);
        assert!(calls(&e, REQUEST_POSITION_PLAYER).is_empty());
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
    }

    /// Doubles of the `MoveTo` commands on top of [`move_engine`]: the form
    /// ID of an object is its dword at `+0xc`; its slot `0x130` is the text at
    /// `+0xe0`.
    fn move_to_engine() -> (Engine, u32, u32) {
        let mut e = move_engine();
        accept(
            &mut e,
            &[TASK_QUEUE_INTERFACE, QUEUE_COMMAND, FADE_OUT_AND_MOVE],
        );
        e.register(CAN_BE_MOVED, |e, a| (e.mem.u8(a[0] + 0xfa) == 0).into_ret());
        e.register(GET_DWORD_0C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(DATA_HANDLER_HAS_FORM_ID, |_, a| (a[1] == 0x77).into_ret());
        e.register(GET_FIELD_4_ADDRESS, |_, a| {
            assert_eq!(a, [OBJECT_011C3EA4]);
            (OBJECT_011C3EA4 + 4).into_ret()
        });
        e.register(TASK_QUEUE_INTERFACE, |_, _| 0x3333.into_ret());
        e.register(GET_CURRENT_PROCESS_TYPE, |e, a| {
            e.mem.u32(a[0] + 0xe8).into_ret()
        });
        e.mem.set_u32(DATA_HANDLER, 0x4444);
        let script_form = object_sized(&mut e, 0x100, false, &[(0x130, V_FIELD_E0)]);
        e.mem.set_u32(script_form + 0xc, 0x66);
        e.mem.set_u32(script_form + 0xe0, 0x7001);
        let target = move_target(&mut e, true, true);
        (e, script_form, target)
    }

    /// The arguments of a move command: `thisObj`, target and offsets
    /// (1, 2, 3) as parse results, the script form as the sixth word.
    fn move_args(e: &mut Engine, this_obj: u32, script_form: u32, target: u32) -> ScriptArgs {
        parse_gives(
            e,
            true,
            &[target, 1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()],
        );
        let mut a = script(e, this_obj);
        a.script_obj = Ptr::new(script_form);
        a
    }

    #[test]
    fn move_to_moves_the_reference_next_to_the_target() {
        let (mut e, script_form, target) = move_to_engine();
        let reference = movable(&mut e, false, false);
        let a = move_args(&mut e, reference, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        // The four outputs are preset to 0 / 0.0.
        assert_eq!(calls(&e, PARSE_PARAMETERS).len(), 1);
        assert_eq!(calls(&e, CAN_BE_MOVED), vec![vec![reference]]);
        assert_eq!(calls(&e, GET_FIELD_4_ADDRESS), vec![vec![OBJECT_011C3EA4]]);
        let player = e.mem.u32(PLAYER);
        assert_eq!(
            calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE),
            vec![vec![player, reference]]
        );
        let expected = [11.0f32, 22.0, 33.0].map(f32::to_bits);
        for i in 0..3 {
            assert_eq!(e.mem.u32(reference + 0xb0 + 4 * i), expected[i as usize]);
        }
        assert!(calls(&e, TASK_QUEUE_INTERFACE).is_empty());
    }

    #[test]
    fn move_to_sends_a_message_when_the_object_says_so() {
        let (mut e, script_form, target) = move_to_engine();
        e.mem.set_u32(OBJECT_011C3EA4 + 4, 2);
        let reference = movable(&mut e, false, false);
        let a = move_args(&mut e, reference, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        // Opcode, thisObj, target and the offsets as three doubles.
        // The queue getter takes no words; they all go with its answer to 0087a8b0.
        assert_eq!(calls(&e, TASK_QUEUE_INTERFACE), vec![Vec::<u32>::new()]);
        let mut expected = vec![0x3333, 0x109e, reference, target];
        for value in [1.0f64, 2.0, 3.0] {
            expected.push(value.to_bits() as u32);
            expected.push((value.to_bits() >> 32) as u32);
        }
        assert_eq!(calls(&e, QUEUE_COMMAND), vec![expected]);
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
        // A counter of 1 is not above 1.
        e.mem.set_u32(OBJECT_011C3EA4 + 4, 1);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        assert!(calls(&e, TASK_QUEUE_INTERFACE).is_empty());
    }

    #[test]
    fn move_to_refuses_a_reference_that_cannot_be_moved() {
        let (mut e, script_form, target) = move_to_engine();
        let reference = movable(&mut e, false, false);
        e.mem.set_u8(reference + 0xfa, 1);
        e.mem.set_u32(reference + 0xc, 0x55);
        e.mem.set_u32(reference + 0xe0, 0x7002);
        // The reference's slot 0x130 gives its text.
        let vtable = e.mem.u32(reference);
        e.mem.set_u32(vtable + 0x130, V_FIELD_E0);
        // The data handler does not know the script's form ID: the report.
        let a = move_args(&mut e, reference, script_form, target);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_CANNOT_BE_MOVED, 0x7002]]
        );
        assert_eq!(
            calls(&e, DATA_HANDLER_HAS_FORM_ID),
            vec![vec![0x4444, 0x66]]
        );
        assert_eq!(
            calls(&e, SCRIPT_ERROR),
            vec![vec![FORMAT_SCRIPT_CANNOT_MOVE, 0x7001, 0x66, 0x7002, 0x55]]
        );
        // Nothing is moved, still true.
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
        // A script form the data handler knows: no report.
        e.mem.set_u32(script_form + 0xc, 0x77);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        assert!(calls(&e, SCRIPT_ERROR).is_empty());
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
    }

    #[test]
    fn move_to_needs_a_target_and_parameters() {
        let (mut e, script_form, target) = move_to_engine();
        let reference = movable(&mut e, false, false);
        let a = move_args(&mut e, reference, script_form, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        assert!(calls(&e, GET_FIELD_4_ADDRESS).is_empty());
        // No thisObj: the parse and the move check run, the base does
        // nothing.
        let a = move_args(&mut e, 0, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_cf50, a));
        assert!(calls(&e, CAN_BE_MOVED).is_empty());
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_cf50, a));
    }

    #[test]
    fn the_005cd0e0_command_moves_a_plain_reference_at_once() {
        let (mut e, script_form, target) = move_to_engine();
        let reference = movable(&mut e, false, false);
        let a = move_args(&mut e, reference, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d0e0, a));
        let player = e.mem.u32(PLAYER);
        assert_eq!(
            calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE),
            vec![vec![player, reference]]
        );
        assert!(calls(&e, FADE_OUT_AND_MOVE).is_empty());
        // An actor without a process is also moved at once.
        let actor = movable(&mut e, true, false);
        let a = move_args(&mut e, actor, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d0e0, a));
        assert!(calls(&e, FADE_OUT_AND_MOVE).is_empty());
        assert_eq!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).len(), 1);
    }

    #[test]
    fn the_005cd0e0_command_fades_an_actor_with_process_type_0_out() {
        let (mut e, script_form, target) = move_to_engine();
        let actor = movable(&mut e, true, false);
        let process = object_sized(
            &mut e,
            0x100,
            false,
            &[(0x4bc, V_FIELD_E0), (0x28, V_B), (0x24, V_C)],
        );
        e.mem.set_u32(actor + 0xf8, process);
        e.mem.set_u32(actor + 0xe8, 0);
        let a = move_args(&mut e, actor, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d0e0, a));
        assert_eq!(
            calls(&e, FADE_OUT_AND_MOVE),
            vec![vec![
                process,
                actor,
                target,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
        // Another process type: moved at once.
        e.mem.set_u32(actor + 0xe8, 3);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d0e0, a));
        assert!(calls(&e, FADE_OUT_AND_MOVE).is_empty());
        assert_eq!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).len(), 1);
    }

    #[test]
    fn the_005cd0e0_command_sends_its_own_message_and_refuses_unmovable_references() {
        let (mut e, script_form, target) = move_to_engine();
        e.mem.set_u32(OBJECT_011C3EA4 + 4, 5);
        let reference = movable(&mut e, false, false);
        let a = move_args(&mut e, reference, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d0e0, a));
        assert_eq!(calls(&e, QUEUE_COMMAND)[0][..2], [0x3333, 0x124f]);
        // An unmovable reference: reported, not moved.
        e.mem.set_u32(OBJECT_011C3EA4 + 4, 0);
        e.mem.set_u8(reference + 0xfa, 1);
        e.mem.set_u32(reference + 0xe0, 0x7002);
        let vtable = e.mem.u32(reference);
        e.mem.set_u32(vtable + 0x130, V_FIELD_E0);
        let a = move_args(&mut e, reference, script_form, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d0e0, a));
        assert_eq!(calls(&e, SCRIPT_ERROR).len(), 1);
        assert!(calls(&e, CHECK_FOR_QUEST_TARGET_UPDATE).is_empty());
        // No target: nothing; no parse: false.
        let a = move_args(&mut e, reference, script_form, 0);
        assert!(run(&mut e, 0x005c_d0e0, a));
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d0e0, a));
    }

    /// The scene of `MoveToQuestTarget`: the player's active quest (the
    /// dword at `+0x6b8`) and target list (the dword at `+0x6b0`, nodes as in
    /// [`quest_scene`]); a quest target keeps its references at `+0x10` and
    /// `+0x14`.
    fn quest_target_engine(quest: bool, targets: &[u32]) -> Engine {
        let mut e = move_engine();
        let player = e.mem.u32(PLAYER);
        e.mem.set_u32(player + 0x6b8, quest as u32);
        e.register(PLAYER_GET_CURRENT_TARGET_LIST, |e, a| {
            e.mem.u32(a[0] + 0x6b0).into_ret()
        });
        e.register(NODE_DATA_POINTER, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(QUEST_TARGET_GET_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 0x10 + 4 * a[1]).into_ret()
        });
        let mut head = 0;
        for target in targets.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *target);
            e.mem.set_u32(node + 4, head);
            head = node;
        }
        e.mem.set_u32(player + 0x6b0, head);
        e
    }

    #[test]
    fn move_to_quest_target_moves_the_player_to_the_chosen_target() {
        let mut e = quest_target_engine(true, &[]);
        let player = e.mem.u32(PLAYER);
        let reference_a = move_target(&mut e, true, true);
        let reference_b = move_target(&mut e, true, true);
        let first = e.mem.alloc(0x40);
        e.mem.set_u32(first + 0x14, reference_a);
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(second + 0x14, reference_b);
        let mut list = 0;
        for target in [second, first] {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, target);
            e.mem.set_u32(node + 4, list);
            list = node;
        }
        e.mem.set_u32(player + 0x6b0, list);
        // The index is preset to 1: the first target.
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), 1);
            true.into_ret()
        });
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d330, a));
        let request = calls(&e, REQUEST_POSITION_PLAYER);
        assert_eq!(e.mem.u32(request[0][1] + 0x2c), reference_a);
        // The offsets are 0: the position is the target's.
        assert_eq!(e.mem.f32(request[0][1] + 8), 10.0);
        // Index 2: the second target.
        parse_gives(&mut e, true, &[2]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d330, a));
        let request = calls(&e, REQUEST_POSITION_PLAYER);
        assert_eq!(e.mem.u32(request[0][1] + 0x2c), reference_b);
        // Index 5: the walk stops at the end of the list at the last target.
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d330, a));
        let request = calls(&e, REQUEST_POSITION_PLAYER);
        assert_eq!(e.mem.u32(request[0][1] + 0x2c), reference_b);
    }

    #[test]
    fn move_to_quest_target_says_what_is_missing() {
        let mut e = quest_target_engine(false, &[]);
        parse_gives(&mut e, true, &[1]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d330, a));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_NO_ACTIVE_QUEST]]);
        assert!(calls(&e, REQUEST_POSITION_PLAYER).is_empty());
        // An active quest without targets.
        let mut e = quest_target_engine(true, &[]);
        parse_gives(&mut e, true, &[1]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d330, a));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_NO_CURRENT_TARGETS]]);
        assert!(calls(&e, REQUEST_POSITION_PLAYER).is_empty());
        // An index of 0 takes no target; parameters that do not parse: false.
        parse_gives(&mut e, true, &[0]);
        assert!(run(&mut e, 0x005c_d330, a));
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d330, a));
    }

    // ---- the remaining commands ----------------------------------------------------------

    #[test]
    fn get_is_current_package_forwards_the_package() {
        let mut e = engine3();
        e.register(GET_IS_CURRENT_PACKAGE_CONDITION, |_, a| {
            (a[0] == 0x40).into_ret()
        });
        parse_gives(&mut e, true, &[0x99]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d440, a));
        assert_eq!(
            calls(&e, GET_IS_CURRENT_PACKAGE_CONDITION),
            vec![vec![0x40, 0x99, 0, a.result.addr()]]
        );
        // The condition function's answer is returned.
        let other = script(&mut e, 0x41);
        assert!(!run(&mut e, 0x005c_d440, other));
        // No thisObj or no package: true without a call. No parse: false.
        let none = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d440, none));
        parse_gives(&mut e, true, &[0]);
        assert!(run(&mut e, 0x005c_d440, a));
        assert!(calls(&e, GET_IS_CURRENT_PACKAGE_CONDITION).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d440, a));
    }

    #[test]
    fn the_005cd4b0_command_hands_the_text_to_the_object_of_slot_0x1b8() {
        let mut e = engine3();
        let target = object_sized(&mut e, 0x100, false, &[(0x124, V_B)]);
        let this_obj = object_sized(&mut e, 0x100, false, &[(0x1b8, V_FIELD_E0)]);
        e.mem.set_u32(this_obj + 0xe0, target);
        parse_gives(&mut e, true, &[]);
        let a = script(&mut e, this_obj);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d4b0, a));
        let buffer = calls(&e, PARSE_PARAMETERS)[0][7];
        assert_eq!(calls(&e, V_B), vec![vec![target, buffer]]);
        assert_eq!(calls(&e, V_FIELD_E0), vec![vec![this_obj, 0]]);
        // No object from the slot: nothing; no thisObj: still true.
        e.mem.set_u32(this_obj + 0xe0, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d4b0, a));
        assert!(calls(&e, V_B).is_empty());
        let none = script(&mut e, 0);
        assert!(run(&mut e, 0x005c_d4b0, none));
        // Text that does not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d4b0, a));
    }

    #[test]
    fn wake_up_pc_stores_the_hours_for_a_sleeping_player() {
        let mut e = engine3();
        accept(&mut e, &[PLAYER_SET_SLEEP_HOURS, CLOSE_SLEEP_MENU]);
        let player = e.mem.u32(PLAYER);
        e.register(PLAYER_GET_SLEEP_FLAG, |e, a| {
            e.mem.u8(a[0] + 0x658).into_ret()
        });
        e.mem.set_u8(player + 0x658, 1);
        parse_gives(&mut e, true, &[6]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d5b0, a));
        assert_eq!(calls(&e, PLAYER_SET_SLEEP_HOURS), vec![vec![player, 6, 1]]);
        assert!(calls(&e, CLOSE_SLEEP_MENU).is_empty());
        // A negative number counts as 0 and closes the sleep menu.
        parse_gives(&mut e, true, &[(-3i32) as u32]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d5b0, a));
        assert_eq!(calls(&e, PLAYER_SET_SLEEP_HOURS), vec![vec![player, 0, 1]]);
        assert_eq!(calls(&e, CLOSE_SLEEP_MENU).len(), 1);
        // A player who is not sleeping: nothing. No parse: false.
        e.mem.set_u8(player + 0x658, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d5b0, a));
        assert!(calls(&e, PLAYER_SET_SLEEP_HOURS).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d5b0, a));
    }

    /// The scene of the `005cd650` command: an actor (flag at `+0xff`)
    /// whose `GetForceNextUpdate` is the byte at `+0xfc`, slot `0x21c` the
    /// byte at `+0xfd`, slot `0x1e8` the dword at `+0xe0`; its form is the
    /// dword at `+0x20`.
    fn worn_engine() -> (Engine, u32, u32) {
        let mut e = engine3();
        accept(
            &mut e,
            &[ACTOR_FN_008A8010, INIT_DEFAULT_WORN, NPC_FN_00606540],
        );
        e.register(GET_FORCE_NEXT_UPDATE, |e, a| {
            (e.mem.u8(a[0] + 0xfc) != 0).into_ret()
        });
        e.register(BGS_SAVE_FORM_BUFFER_GET_FORM, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        let actor = object_sized(
            &mut e,
            0x100,
            true,
            &[(0x21c, V_FLAG_FD), (0x1e8, V_FIELD_E0)],
        );
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(actor + 0x20, form);
        e.mem.set_u32(actor + 0xe0, 0x0ccc);
        (e, actor, form)
    }

    #[test]
    fn the_005cd650_command_renews_the_worn_items_of_a_creature() {
        let (mut e, actor, form) = worn_engine();
        e.mem.set_u8(actor + 0xfc, 1);
        e.mem.set_u8(actor + 0xfd, 1);
        parse_gives(&mut e, true, &[9]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d650, a));
        assert_eq!(calls(&e, ACTOR_FN_008A8010), vec![vec![actor, 9]]);
        assert_eq!(
            calls(&e, INIT_DEFAULT_WORN),
            vec![vec![form, actor, 1, 1, 1]]
        );
        assert!(calls(&e, NPC_FN_00606540).is_empty());
    }

    #[test]
    fn the_005cd650_command_renews_the_worn_items_of_another_actor() {
        let (mut e, actor, form) = worn_engine();
        e.mem.set_u8(actor + 0xfc, 1);
        parse_gives(&mut e, true, &[9]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d650, a));
        assert_eq!(
            calls(&e, NPC_FN_00606540),
            vec![vec![form, actor, 0x0ccc, 1]]
        );
        assert!(calls(&e, INIT_DEFAULT_WORN).is_empty());
    }

    #[test]
    fn the_005cd650_command_needs_an_actor_and_a_forced_update() {
        let (mut e, actor, _) = worn_engine();
        parse_gives(&mut e, true, &[9]);
        // No forced update: the number is passed, nothing renewed.
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d650, a));
        assert_eq!(calls(&e, ACTOR_FN_008A8010).len(), 1);
        assert!(calls(&e, INIT_DEFAULT_WORN).is_empty());
        assert!(calls(&e, NPC_FN_00606540).is_empty());
        // Not an actor, and no thisObj.
        let thing = object_sized(&mut e, 0x100, false, &[]);
        let a = script(&mut e, thing);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d650, a));
        let none = script(&mut e, 0);
        assert!(run(&mut e, 0x005c_d650, none));
        assert!(calls(&e, ACTOR_FN_008A8010).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d650, none));
    }

    /// The scene of `SelectPlayerSpell`: a form is a spell when its byte at
    /// `+0xf0` is 1 and a book when its byte at `+0xf1` is 1.
    fn spell_engine() -> Engine {
        let mut e = engine3();
        accept(
            &mut e,
            &[PLAYER_SET_SELECTED_SPELL, PLAYER_SELECT_BOOK, CONSOLE_PRINT],
        );
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[2], RTTI_TES_FORM);
            let offset = match a[3] {
                RTTI_SPELL_ITEM => 0xf0,
                RTTI_TES_OBJECT_BOOK => 0xf1,
                other => panic!("unexpected target type {other:08x}"),
            };
            (if e.mem.u8(a[0] + offset) == 1 {
                a[0]
            } else {
                0
            })
            .into_ret()
        });
        e.register(STRING_TEXT_OR_EMPTY, |_, a| (a[0] + 0x100).into_ret());
        e
    }

    #[test]
    fn select_player_spell_selects_a_spell() {
        let mut e = spell_engine();
        let player = e.mem.u32(PLAYER);
        let spell = e.mem.alloc(0x100);
        e.mem.set_u8(spell + 0xf0, 1);
        parse_gives(&mut e, true, &[spell]);
        set_echo(&mut e, true);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d750, a));
        assert_eq!(
            calls(&e, PLAYER_SET_SELECTED_SPELL),
            vec![vec![player, spell + 0x18]]
        );
        assert!(calls(&e, PLAYER_SELECT_BOOK).is_empty());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_PLAYER_SPELL_SET, spell + 0x18 + 0x100]]
        );
    }

    #[test]
    fn select_player_spell_selects_a_book() {
        let mut e = spell_engine();
        let player = e.mem.u32(PLAYER);
        let book = e.mem.alloc(0x100);
        e.mem.set_u8(book + 0xf1, 1);
        parse_gives(&mut e, true, &[book]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d750, a));
        assert_eq!(calls(&e, PLAYER_SELECT_BOOK), vec![vec![player, book]]);
        assert!(calls(&e, PLAYER_SET_SELECTED_SPELL).is_empty());
        // The echo flag is off: no message.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn select_player_spell_clears_the_spell_for_another_form() {
        let mut e = spell_engine();
        let player = e.mem.u32(PLAYER);
        let other = e.mem.alloc(0x100);
        parse_gives(&mut e, true, &[other]);
        set_echo(&mut e, true);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d750, a));
        assert_eq!(calls(&e, PLAYER_SET_SELECTED_SPELL), vec![vec![player, 0]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_PLAYER_SPELL_SET, TEXT_NONE]]
        );
        // No form: nothing; parameters that do not parse: false.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d750, a));
        assert!(calls(&e, PLAYER_SET_SELECTED_SPELL).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005c_d750, a));
    }

    #[test]
    fn the_flag_bit_setter_sets_bit_2() {
        let mut e = engine3();
        accept(&mut e, &[SET_FLAG_BIT]);
        start_log(&mut e);
        e.call(0x005c_d8f0, &args![0x1230u32, 1u8]);
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![0x1230, 2, 1, 1]]);
    }

    #[test]
    fn the_005cd870_command_sets_the_actor_base_flag_by_the_sign() {
        let mut e = engine3();
        accept(&mut e, &[SET_FLAG_BIT]);
        for (number, flag) in [(3i32, 1u32), (0, 0), (-2, 0)] {
            parse_gives(&mut e, true, &[0x1000, number as u32]);
            let a = script(&mut e, 0x40);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_d870, a));
            assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![0x1030, 2, flag, 1]]);
            assert_eq!(result_of(&e, a), 0.0);
        }
        // No base: nothing. No parse: false, result zeroed.
        parse_gives(&mut e, true, &[0, 1]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d870, a));
        assert!(calls(&e, SET_FLAG_BIT).is_empty());
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, 0x40);
        assert!(!run(&mut e, 0x005c_d870, a));
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn the_005cd910_command_calls_slot_0xcc_with_the_sign() {
        let mut e = engine3();
        let form = object_sized(&mut e, 0x100, false, &[(0xcc, V_A)]);
        for (number, flag) in [(3i32, 1u32), (0, 0), (-2, 0)] {
            parse_gives(&mut e, true, &[form, number as u32]);
            let a = script(&mut e, 0x40);
            start_log(&mut e);
            assert!(run(&mut e, 0x005c_d910, a));
            assert_eq!(calls(&e, V_A), vec![vec![form, flag]]);
            assert_eq!(result_of(&e, a), 0.0);
        }
        parse_gives(&mut e, true, &[0, 1]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005c_d910, a));
        assert!(calls(&e, V_A).is_empty());
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, 0x40);
        assert!(!run(&mut e, 0x005c_d910, a));
        assert_eq!(result_of(&e, a), 0.0);
    }
}
