//! `fallout shared/imagespacemodifierinstance.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `ImageSpaceModifierInstance` is one running screen effect. The game
//! keeps the running instances in a `BSSimpleList<NiPointer<ImageSpaceModifierInstance>>`
//! at `+0x6c` of the object whose address is in the global `011dea10`
//! (`0043b5d0` returns the list; the object's class is not confirmed here).
//! The base class holds the strength, the target and the start time; the
//! `Form` subclass plays an image-space-modifier form (with a blend to the
//! previous one), and `Temp` (duration) is the base of the depth-of-field
//! (`DOF`), radial-blur (`RB`) and depth-radial-blur (`DRB`) instances.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`. Every `FLD`/`FSTP` copy of a `float` turns a
//! signalling NaN into a quiet one; [`quiet`] does the same. Comparisons
//! are written as the exe's flag tests read them: `a > b` and `a < b` are
//! false when either side is NaN (the `TEST AH,0x41` / `TEST AH,5; JP`
//! patterns), and `!= 0` is true for NaN (the `TEST AH,0x44; JNP` pattern).
//!
//! Every function with a C++ exception-unwinding frame (`FS:[0]`) is marked
//! `Translation incomplete`: the unwinding itself is not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject::NiObject` (`00a5d3a0`): sets the vtable, clears the reference
/// count and counts the object.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiObject::~NiObject` (`00a5d3d0`).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiPointer<T>::NiPointer(T*)` (`00633c90`): stores the pointer and adds a reference.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer<T>::operator=(T*)` (`0066b0d0`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::operator=(const NiPointer<T>&)` (`006e5cc0`).
const NI_POINTER_ASSIGN_FROM: u32 = 0x006e_5cc0;
/// `NiPointer<T>::~NiPointer` (`0045cec0`).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `operator new` (`00aa13e0`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// Sized `operator delete` (`00aa1460`): pointer, size.
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
/// `sprintf_s`-style formatter (`00406d00`): buffer, size, format, values.
const FORMAT_STRING: u32 = 0x0040_6d00;

/// The global holding the object whose `+0x6c` is the list of running instances.
const LIST_OWNER: u32 = 0x011d_ea10;
/// Returns the owner's list (its address, `owner + 0x6c`).
const LIST_OF_OWNER: u32 = 0x0043_b5d0;
/// A list node's own address (`this`).
const NODE_SELF: u32 = 0x0068_15c0;
/// A pointer's or list node's item (`*this`, `00559450`).
const NODE_ITEM: u32 = 0x0055_9450;
/// A list node's next node (`node->next`, at `+4`).
const NODE_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList<NiPointer<..>>::Remove` (`00631620`): list node, `NiPointer*`.
const LIST_REMOVE: u32 = 0x0063_1620;
/// Appends an item to the list (`0052a9b0`): list node, `NiPointer*`.
const LIST_ADD: u32 = 0x0052_a9b0;

/// The `float` the game keeps its current time in (read by [`fn_00529990`]).
const CURRENT_TIME: u32 = 0x011c_96fc;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `-10000.0` as a `double`: a start time below this means "not started yet".
const NOT_STARTED_LIMIT: u32 = 0x0102_d268;
/// `-10000.0` as a `float`: the start time a new instance is created with.
const NOT_STARTED: u32 = 0x0102_d270;

/// Vtable of the base `ImageSpaceModifierInstance` (slot 0 is its scalar deleting destructor).
const BASE_VTABLE: u32 = 0x0102_d1cc;
/// Vtable of `ImageSpaceModifierInstanceForm`.
const FORM_VTABLE: u32 = 0x0102_d12c;
/// Vtable of `ImageSpaceModifierInstanceTemp`.
const TEMP_VTABLE: u32 = 0x0102_d50c;
/// Vtable of `ImageSpaceModifierInstanceDOF`.
const DOF_VTABLE: u32 = 0x0102_d2a4;
/// Vtable of `ImageSpaceModifierInstanceRB`.
const RB_VTABLE: u32 = 0x0102_d36c;
/// Vtable of `ImageSpaceModifierInstanceDRB`.
const DRB_VTABLE: u32 = 0x0102_d454;
/// The value `GetRTTI` returns for the base class (the engine map names the function `NiObject::GetRTTI`).
const NI_OBJECT_RTTI: u32 = 0x011f_4418;

/// Vtable slot (byte offset) that answers the instance as an
/// `ImageSpaceModifierInstanceForm` (non-null only for that class).
const SLOT_AS_FORM: u32 = 0x94;
/// Vtable slot of the call the `Apply` functions make on the target.
const SLOT_TARGET_PREPARE: u32 = 0xbc;
/// Vtable slot of a form's name getter (the `PrintInfo` text).
const SLOT_NAME: u32 = 0x130;

/// `TESImageSpaceModifier::IsExpired` (`005316c0`): form, start time.
const FORM_IS_EXPIRED: u32 = 0x0053_16c0;
/// Applies a form as weather (`00531d20`, unnamed in the engine map): form, start time, strength.
const APPLY_FORM_WEATHER_00531D20: u32 = 0x0053_1d20;
/// Applies a form (`00531720`, unnamed): form, start time, strength, target.
const APPLY_FORM_00531720: u32 = 0x0053_1720;
/// Blends two forms (`00532220`, unnamed; cdecl): blend factor, previous
/// form, form, previous strength, strength, previous target, target.
const APPLY_FORM_BLEND_00532220: u32 = 0x0053_2220;
/// Getter of the dword at `+0x1c` (the folded `PathingLocation::GetWorldspace` body).
const GET_FIELD_1C: u32 = 0x0044_1110;
/// Getter of the dword at `+0x18` (`009611e0`).
const GET_FIELD_18: u32 = 0x0096_11e0;
/// Getter of the dword at `+0xc` (the form id of the form it is called on).
const GET_FIELD_0C: u32 = 0x0084_e3a0;
/// The object the `Apply` functions of the depth-of-field family call (`004e3270` returns `[011f91ac]`).
const EFFECT_MANAGER: u32 = 0x004e_3270;
/// The target's data or the shared default when it has none (`0043d450`).
const TARGET_OR_DEFAULT: u32 = 0x0043_d450;
/// Depth of field, no target: strength, mode, mask constant, distance, range.
const MANAGER_DOF_WITHOUT_TARGET: u32 = 0x00b8_b0d0;
/// Depth of field with a target: strength, mode, mask constant, target data.
const MANAGER_DOF_WITH_TARGET: u32 = 0x00b8_b8d0;
/// Radial blur, no target: strength, rampup, start, rampdown, down start, center.
const MANAGER_RB_WITHOUT_TARGET: u32 = 0x00b8_b000;
/// Radial blur with a target: strength, rampup, start, rampdown, down start, target data.
const MANAGER_RB_WITH_TARGET: u32 = 0x00b8_b760;
/// Depth radial blur, no target: strength, mode, distance, range, rampup, start, center.
const MANAGER_DRB_WITHOUT_TARGET: u32 = 0x00b8_b060;
/// Depth radial blur with a target: strength, mode, rampup, start, target data.
const MANAGER_DRB_WITH_TARGET: u32 = 0x00b8_b7d0;

/// The audio instance getter (`00453a70`, `BSAudio::QInstance` in the engine map).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`).
const GET_SOUND_HANDLE: u32 = 0x00ad_73b0;
/// `BSSoundHandle::Play` (`00ad8830`).
const PLAY_SOUND_HANDLE: u32 = 0x00ad_8830;
/// Destructor of the sound handle (`00483710`, an empty body).
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;

/// `"(NULL)"`.
const NULL_TEXT: u32 = 0x0102_d294;
/// `"B %s %s"`.
const BLEND_FORMAT: u32 = 0x0102_d28c;
/// `"%s %s Str: %1.3f "`.
const FORM_FORMAT: u32 = 0x0102_d274;
/// `"W"` (weather).
const WEATHER_TEXT: u32 = 0x0102_d288;
/// `" "`.
const NOT_WEATHER_TEXT: u32 = 0x0102_0770;
/// `"  DOF  Distance: %f  Range: %f  Str: %f"`.
const DOF_FORMAT: u32 = 0x0102_d340;
/// `"  RB  Rampup: %f  Start: %f  Rampdown: %f ..."`.
const RB_FORMAT: u32 = 0x0102_d408;
/// `"  Depth Radial Blur"`.
const DRB_FORMAT: u32 = 0x0102_d4f0;

layout! {
    /// `ImageSpaceModifierInstance` (Xbox PDB), 0x1C bytes.
    pub struct ImageSpaceModifierInstance: 0x1c {
        /// `bIsWeather` (Xbox PDB).
        0x08 bIsWeather: bool,
        /// `fStrength` (Xbox PDB).
        0x0C fStrength: f32,
        /// `spTarget` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x10 spTarget: Ptr,
        /// `fStartTime` (Xbox PDB).
        0x14 fStartTime: f32,
        /// `iFlags` (Xbox PDB).
        0x18 iFlags: u32,
    }

    /// `ImageSpaceModifierInstanceForm` (Xbox PDB), 0x30 bytes (the base
    /// class's fields are in [`ImageSpaceModifierInstance`]).
    pub struct ImageSpaceModifierInstanceForm: 0x30 {
        /// `pImageSpaceMod` (Xbox PDB): `TESImageSpaceModifier*`.
        0x1C pImageSpaceMod: Ptr,
        /// `pLastImageSpaceMod` (Xbox PDB).
        0x20 pLastImageSpaceMod: Ptr,
        /// `fLastStrength` (Xbox PDB).
        0x24 fLastStrength: f32,
        /// `spLastTarget` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x28 spLastTarget: Ptr,
        /// `fTransitionTime` (Xbox PDB).
        0x2C fTransitionTime: f32,
    }

    /// `ImageSpaceModifierInstanceTemp` (Xbox PDB), 0x20 bytes.
    pub struct ImageSpaceModifierInstanceTemp: 0x20 {
        /// `fDuration` (Xbox PDB).
        0x1C fDuration: f32,
    }

    /// `ImageSpaceModifierInstanceDOF` (Xbox PDB), 0x30 bytes.
    pub struct ImageSpaceModifierInstanceDOF: 0x30 {
        /// `fDistance` (Xbox PDB).
        0x20 fDistance: f32,
        /// `fRange` (Xbox PDB).
        0x24 fRange: f32,
        /// `fMaskConst` (Xbox PDB).
        0x28 fMaskConst: f32,
        /// `Mode` (Xbox PDB): `DepthOfFieldMode`.
        0x2C Mode: u32,
    }

    /// `ImageSpaceModifierInstanceRB` (Xbox PDB), 0x38 bytes.
    pub struct ImageSpaceModifierInstanceRB: 0x38 {
        /// `fRampup` (Xbox PDB).
        0x20 fRampup: f32,
        /// `fStart` (Xbox PDB).
        0x24 fStart: f32,
        /// `fRampdown` (Xbox PDB).
        0x28 fRampdown: f32,
        /// `fDownStart` (Xbox PDB).
        0x2C fDownStart: f32,
        /// `Center.x` (Xbox PDB `NiPoint2 Center`), kept as the raw word.
        0x30 CenterX: u32,
        /// `Center.y`, kept as the raw word.
        0x34 CenterY: u32,
    }

    /// `ImageSpaceModifierInstanceDRB` (Xbox PDB), 0x44 bytes.
    pub struct ImageSpaceModifierInstanceDRB: 0x44 {
        /// `fRampup` (Xbox PDB).
        0x20 fRampup: f32,
        /// `fStart` (Xbox PDB).
        0x24 fStart: f32,
        /// `fRampdown` (Xbox PDB).
        0x28 fRampdown: f32,
        /// `fDownStart` (Xbox PDB).
        0x2C fDownStart: f32,
        /// `Center.x` (Xbox PDB `NiPoint2 Center`), kept as the raw word.
        0x30 CenterX: u32,
        /// `Center.y`, kept as the raw word.
        0x34 CenterY: u32,
        /// `fDistance` (Xbox PDB).
        0x38 fDistance: f32,
        /// `fRange` (Xbox PDB).
        0x3C fRange: f32,
        /// `Mode` (Xbox PDB): `DepthOfFieldMode`.
        0x40 Mode: u32,
    }
}

type Base = ImageSpaceModifierInstance;
type Form = ImageSpaceModifierInstanceForm;
type Temp = ImageSpaceModifierInstanceTemp;
type Dof = ImageSpaceModifierInstanceDOF;
type RadialBlur = ImageSpaceModifierInstanceRB;
type DepthRadialBlur = ImageSpaceModifierInstanceDRB;

/// An x87 `FLD`/`FSTP` copy of a `float`: a signalling NaN becomes a quiet one.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// `NiPointer::operator*` (`00559450`) on the pointer or list node at `pointer`.
fn pointer_value(e: &mut Engine, pointer: u32) -> u32 {
    e.call(NODE_ITEM, &args![pointer]).u32()
}

/// The first node of the list of running instances.
fn list_head(e: &mut Engine) -> u32 {
    let owner = e.global::<u32>(LIST_OWNER);
    e.call(LIST_OF_OWNER, &args![owner]).u32()
}

/// Adds a new instance to the list of running instances through a
/// temporary `NiPointer`.
fn add_to_list(e: &mut Engine, instance: Ptr) {
    e.with_stack(4, |e, pointer| {
        e.call(NI_POINTER_CONSTRUCT, &args![pointer, instance]);
        let list = list_head(e);
        e.call(LIST_ADD, &args![list, pointer]);
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// Removes `item` from the list at `node` through a temporary `NiPointer`.
fn remove_from_list(e: &mut Engine, node: u32, item: u32) {
    e.with_stack(4, |e, pointer| {
        e.call(NI_POINTER_CONSTRUCT, &args![pointer, item]);
        e.call(LIST_REMOVE, &args![node, pointer]);
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// Plays the sound of `form`: `getter` (called on `form`) yields the object
/// whose `+0xc` is the numeric id of the sound; its 12-byte handle lives
/// on the stack.
fn play_sound(e: &mut Engine, getter: u32, form: Ptr) {
    let sound = e.call(getter, &args![form]).u32();
    let id = e.call(GET_FIELD_0C, &args![sound]).u32();
    e.with_stack(12, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(GET_SOUND_HANDLE, &args![audio, handle, id, 0x0020_0000u32]);
        e.call(PLAY_SOUND_HANDLE, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// The deleting-destructor tail: frees the object (`size` bytes) when bit 0 of `flags` is set.
fn delete_if_requested(e: &mut Engine, this: u32, flags: u32, size: u32) {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, size]);
    }
}

/// A new `ImageSpaceModifierInstanceForm` (0x30 bytes) for `form`, with
/// the "not started" start time, or null when the allocation fails.
fn new_form_instance(e: &mut Engine, form: Ptr, strength: f32, target: Ptr) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    if memory == 0 {
        return Ptr::NULL;
    }
    let not_started = quiet(e.global::<f32>(NOT_STARTED));
    form_ctor(e, Ptr::new(memory), form, not_started, strength, target).cast()
}

/// The start of every `Apply` of the depth-of-field family: false when
/// the strength is not above zero; otherwise fixes an unset start time.
fn begin_apply(e: &mut Engine, this: Ptr<Base>) -> bool {
    let zero = e.global::<f64>(ZERO);
    let strong = e.get(this, Base::fStrength) as f64 > zero;
    if strong {
        fix_start_time(e, this);
    }
    strong
}

/// `if (fStartTime < -10000.0) fStartTime = <current time>`.
fn fix_start_time(e: &mut Engine, this: Ptr<Base>) {
    let limit = e.global::<f64>(NOT_STARTED_LIMIT);
    if (e.get(this, Base::fStartTime) as f64) < limit {
        let now = fn_00529990(e);
        e.set(this, Base::fStartTime, now);
    }
}

// Translated from 005293f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstance::Stop` (Xbox PDB, cdecl): removes every entry
/// of `instance` from the list of running instances. After a removal the
/// same node is examined again (the list moves the next entry into it).
// Translation incomplete: 005293f0 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn image_space_modifier_instance_stop(e: &mut Engine, instance: Ptr<Base>) {
    let mut node = list_head(e);
    while node != 0 {
        let node_self = e.call(NODE_SELF, &args![node]).u32();
        let item = pointer_value(e, node_self);
        if item == instance.addr() {
            remove_from_list(e, node, item);
        } else {
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 00529490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of `ImageSpaceModifierInstanceForm` (unnamed in the
/// engine map): a zero-strength instance without a form or a target.
// Translation incomplete: 00529490 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn fn_00529490(e: &mut Engine, this: Ptr<Form>) -> Ptr<Form> {
    fn_00529540(e, this.cast(), 0.0, 0.0, Ptr::NULL);
    e.mem.set_u32(this.addr(), FORM_VTABLE);
    e.set(this, Form::pImageSpaceMod, Ptr::NULL);
    e.set(this, Form::pLastImageSpaceMod, Ptr::NULL);
    e.set(this, Form::fLastStrength, 0.0);
    e.call(NI_POINTER_CONSTRUCT, &args![this.byte_add(0x28), 0u32]);
    e.set(this, Form::fTransitionTime, 0.0);
    e.set(this.cast::<Base>(), Base::bIsWeather, false);
    this
}

// Translated from 00529530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiObject::GetRTTI` (engine map name): returns the run-time type
/// information of `NiObject` (`011f4418`), whichever object it is called on.
pub fn ni_object_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    NI_OBJECT_RTTI
}

// Translated from 00529540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the base `ImageSpaceModifierInstance` (unnamed in the
/// engine map): the base object, the strength, the target and the start time.
// Translation incomplete: 00529540 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn fn_00529540(
    e: &mut Engine,
    this: Ptr<Base>,
    strength: f32,
    start_time: f32,
    target: Ptr,
) -> Ptr<Base> {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    e.set(this, Base::fStrength, quiet(strength));
    e.call(NI_POINTER_CONSTRUCT, &args![this.byte_add(0x10), target]);
    e.set(this, Base::fStartTime, quiet(start_time));
    this
}

// Translated from 005295c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the base `ImageSpaceModifierInstance` (unnamed in the
/// engine map): releases the target, then destroys the `NiObject`.
// Translation incomplete: 005295c0 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn fn_005295c0(e: &mut Engine, this: Ptr<Base>) {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x10), 0u32]);
    e.call(NI_POINTER_DESTRUCT, &args![this.byte_add(0x10)]);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00529630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstance::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys the object and frees its 0x1c bytes when bit 0 of `flags` is set.
pub fn image_space_modifier_instance_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Base>,
    flags: u32,
) -> Ptr<Base> {
    fn_005295c0(e, this);
    delete_if_requested(e, this.addr(), flags, 0x1c);
    this
}

// Translated from 00529660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys the object and frees its 0x30 bytes when bit 0 of `flags` is set.
pub fn image_space_modifier_instance_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Form>,
    flags: u32,
) -> Ptr<Form> {
    fn_00529690(e, this);
    delete_if_requested(e, this.addr(), flags, 0x30);
    this
}

// Translated from 00529690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `ImageSpaceModifierInstanceForm` (unnamed in the
/// engine map): releases the previous target, then runs the base destructor.
// Translation incomplete: 00529690 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn fn_00529690(e: &mut Engine, this: Ptr<Form>) {
    e.mem.set_u32(this.addr(), FORM_VTABLE);
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x28), 0u32]);
    e.call(NI_POINTER_DESTRUCT, &args![this.byte_add(0x28)]);
    fn_005295c0(e, this.cast());
}

// Translated from 00529700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::ImageSpaceModifierInstanceForm` (Xbox
/// PDB): an instance of `form` that started at `start_time`.
// Translation incomplete: 00529700 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn form_ctor(
    e: &mut Engine,
    this: Ptr<Form>,
    form: Ptr,
    start_time: f32,
    strength: f32,
    target: Ptr,
) -> Ptr<Form> {
    fn_00529540(e, this.cast(), quiet(strength), quiet(start_time), target);
    e.mem.set_u32(this.addr(), FORM_VTABLE);
    e.set(this, Form::pImageSpaceMod, form);
    e.set(this, Form::pLastImageSpaceMod, Ptr::NULL);
    e.set(this, Form::fLastStrength, 0.0);
    e.call(NI_POINTER_CONSTRUCT, &args![this.byte_add(0x28), 0u32]);
    e.set(this, Form::fTransitionTime, 0.0);
    e.set(this.cast::<Base>(), Base::bIsWeather, false);
    this
}

// Translated from 005297b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::IsExpired` (Xbox PDB): asks the form
/// whether it has expired since the start time; false without a form.
pub fn form_is_expired(e: &mut Engine, this: Ptr<Form>) -> bool {
    let form = e.get(this, Form::pImageSpaceMod);
    if form.is_null() {
        return false;
    }
    let start_time = quiet(e.get(this.cast::<Base>(), Base::fStartTime));
    e.call(FORM_IS_EXPIRED, &args![form, start_time]).bool()
}

// Translated from 005297e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::Apply` (Xbox PDB): applies the form
/// (blending from the previous form during the transition time) while the
/// strength or the previous strength is above zero.
pub fn form_apply(e: &mut Engine, this: Ptr<Form>) {
    let base: Ptr<Base> = this.cast();
    let zero = e.global::<f64>(ZERO);
    let strength = e.get(base, Base::fStrength);
    let strong = strength as f64 > zero;
    let last_strong = e.get(this, Form::fLastStrength) as f64 > zero;
    if !strong && !last_strong {
        return;
    }
    if e.get(this, Form::pImageSpaceMod).is_null() {
        return;
    }
    fix_start_time(e, base);
    let last_form = e.get(this, Form::pLastImageSpaceMod);
    if last_form.is_null() {
        apply_current_form(e, this);
        return;
    }
    let now = fn_00529990(e);
    let start_time = e.get(base, Base::fStartTime);
    let transition = e.get(this, Form::fTransitionTime);
    let blend = ((now as f64 - start_time as f64) / transition as f64) as f32;
    let one = e.global::<f64>(ONE);
    if (blend as f64) < one {
        let target = pointer_value(e, this.addr() + 0x10);
        let last_target = pointer_value(e, this.addr() + 0x28);
        let form = e.get(this, Form::pImageSpaceMod);
        let strength = quiet(e.get(base, Base::fStrength));
        let last_strength = quiet(e.get(this, Form::fLastStrength));
        e.call(
            APPLY_FORM_BLEND_00532220,
            &args![
                quiet(blend),
                last_form,
                form,
                last_strength,
                strength,
                last_target,
                target
            ],
        );
    } else {
        e.set(this, Form::pLastImageSpaceMod, Ptr::NULL);
        e.set(this, Form::fLastStrength, 0.0);
        apply_current_form(e, this);
    }
}

/// The two ways `Apply` applies the current form alone: as weather, or
/// with the target.
fn apply_current_form(e: &mut Engine, this: Ptr<Form>) {
    let base: Ptr<Base> = this.cast();
    let form = e.get(this, Form::pImageSpaceMod);
    if e.get(base, Base::bIsWeather) {
        let strength = quiet(e.get(base, Base::fStrength));
        let start_time = quiet(e.get(base, Base::fStartTime));
        e.call(
            APPLY_FORM_WEATHER_00531D20,
            &args![form, start_time, strength],
        );
    } else {
        let target = pointer_value(e, this.addr() + 0x10);
        let strength = quiet(e.get(base, Base::fStrength));
        let start_time = quiet(e.get(base, Base::fStartTime));
        e.call(
            APPLY_FORM_00531720,
            &args![form, start_time, strength, target],
        );
    }
}

// Translated from 00529990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's current time (the `float` at `011c96fc`, unnamed in the
/// engine map), returned in `ST0`.
pub fn fn_00529990(e: &mut Engine) -> f32 {
    quiet(e.global::<f32>(CURRENT_TIME))
}

// Translated from 005299a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::Trigger` (Xbox PDB, cdecl): starts a new
/// instance of `form` (null: nothing), adds it to the running list and
/// plays the form's sound. Returns the instance.
// Translation incomplete: 005299a0 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn form_trigger(e: &mut Engine, form: Ptr, strength: f32, target: Ptr) -> Ptr {
    if form.is_null() {
        return Ptr::NULL;
    }
    start_instance(e, form, strength, target)
}

/// The common tail of `Trigger` and `TriggerIfNotActive`: creates the
/// instance, lists it and plays the form's sound when it has one.
fn start_instance(e: &mut Engine, form: Ptr, strength: f32, target: Ptr) -> Ptr {
    let instance = new_form_instance(e, form, strength, target);
    add_to_list(e, instance);
    if e.call(GET_FIELD_1C, &args![form]).u32() != 0 {
        play_sound(e, GET_FIELD_1C, form);
    }
    instance
}

// Translated from 00529ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::TriggerIfNotActive` (Xbox PDB, cdecl):
/// like `Trigger`, but nothing happens (null is returned) when an instance
/// of `form` is already running.
// Translation incomplete: 00529ae0 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn form_trigger_if_not_active(e: &mut Engine, form: Ptr, strength: f32, target: Ptr) -> Ptr {
    if form.is_null() {
        return Ptr::NULL;
    }
    let mut node = list_head(e);
    while node != 0 {
        let node_self = e.call(NODE_SELF, &args![node]).u32();
        let item = pointer_value(e, node_self);
        if item != 0 && e.vcall(item, SLOT_AS_FORM, &args![]).u32() != 0 {
            let as_form = e.vcall(item, SLOT_AS_FORM, &args![]).u32();
            if e.mem.u32(as_form + 0x1c) == form.addr() {
                return Ptr::NULL;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    start_instance(e, form, strength, target)
}

// Translated from 00529c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::Stop` (Xbox PDB, cdecl): removes every
/// running instance of `form` and plays the sound of `form` once.
// Translation incomplete: 00529c90 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn form_stop(e: &mut Engine, form: Ptr) {
    if form.is_null() {
        return;
    }
    let mut played = false;
    let mut node = list_head(e);
    while node != 0 {
        let node_self = e.call(NODE_SELF, &args![node]).u32();
        let item = pointer_value(e, node_self);
        if item != 0 && e.vcall(item, SLOT_AS_FORM, &args![]).u32() != 0 {
            let as_form = e.vcall(item, SLOT_AS_FORM, &args![]).u32();
            if e.call(GET_FIELD_1C, &args![as_form]).u32() == form.addr() {
                remove_from_list(e, node, item);
                if e.call(GET_FIELD_18, &args![form]).u32() != 0 && !played {
                    played = true;
                    play_sound(e, GET_FIELD_18, form);
                }
                continue;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00529df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Switches a running instance to another form (unnamed in the engine map):
/// the current form and strength become the previous ones for the blend.
/// Refused (false) when the current or the new form has its flag byte at
/// `+0x20` set (`00529ea0`).
pub fn fn_00529df0(
    e: &mut Engine,
    this: Ptr<Form>,
    new_form: Ptr,
    transition_time: f32,
    strength: f32,
    target: Ptr,
) -> bool {
    let base: Ptr<Base> = this.cast();
    let current = e.get(this, Form::pImageSpaceMod);
    if !current.is_null() && fn_00529ea0(e, current) != 0 {
        return false;
    }
    if !new_form.is_null() && fn_00529ea0(e, new_form) != 0 {
        return false;
    }
    e.set(this, Form::pLastImageSpaceMod, current);
    let old_strength = quiet(e.get(base, Base::fStrength));
    e.set(this, Form::fLastStrength, old_strength);
    e.call(
        NI_POINTER_ASSIGN_FROM,
        &args![this.byte_add(0x28), this.byte_add(0x10)],
    );
    let now = fn_00529990(e);
    e.set(base, Base::fStartTime, now);
    e.set(this, Form::fTransitionTime, quiet(transition_time));
    e.set(this, Form::pImageSpaceMod, new_form);
    e.set(base, Base::fStrength, quiet(strength));
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x10), target]);
    true
}

// Translated from 00529ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte getter at `+0x20` of the object it is called on (unnamed in the
/// engine map; called on the image-space-modifier forms by `00529df0`).
pub fn fn_00529ea0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x20)
}

// Translated from 00529ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceForm::PrintInfo` (Xbox PDB): writes the form
/// names (`B <form> <previous form>` during a blend, otherwise
/// `<W or space> <form> Str: <strength>`) into `buffer` (0x800 bytes).
pub fn form_print_info(e: &mut Engine, this: Ptr<Form>, buffer: Ptr) {
    let base: Ptr<Base> = this.cast();
    if !e.get(this, Form::pLastImageSpaceMod).is_null() {
        let form = e.get(this, Form::pImageSpaceMod);
        let name = if form.is_null() {
            NULL_TEXT
        } else {
            e.vcall(form.addr(), SLOT_NAME, &args![]).u32()
        };
        let last_form = e.get(this, Form::pLastImageSpaceMod);
        let last_name = e.vcall(last_form.addr(), SLOT_NAME, &args![]).u32();
        e.call(
            FORMAT_STRING,
            &args![buffer, 0x800u32, BLEND_FORMAT, name, last_name],
        );
    } else {
        let form = e.get(this, Form::pImageSpaceMod);
        let name = if form.is_null() {
            NULL_TEXT
        } else {
            e.vcall(form.addr(), SLOT_NAME, &args![]).u32()
        };
        let weather = if e.get(base, Base::bIsWeather) {
            WEATHER_TEXT
        } else {
            NOT_WEATHER_TEXT
        };
        let strength = quiet(e.get(base, Base::fStrength)) as f64;
        e.call(
            FORMAT_STRING,
            &args![buffer, 0x800u32, FORM_FORMAT, weather, name, strength],
        );
    }
}

// Translated from 00529fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ImageSpaceModifierInstanceDOF` (unnamed in the engine
/// map): a depth-of-field instance. `mode` is the `DepthOfFieldMode`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00529fb0(
    e: &mut Engine,
    this: Ptr<Dof>,
    strength: f32,
    start_time: f32,
    mode: u32,
    distance: f32,
    range: f32,
    mask_constant: f32,
    target: Ptr,
    duration: f32,
) -> Ptr<Dof> {
    temp_ctor(
        e,
        this.cast(),
        quiet(start_time),
        quiet(strength),
        target,
        quiet(duration),
    );
    e.mem.set_u32(this.addr(), DOF_VTABLE);
    e.set(this, Dof::fDistance, quiet(distance));
    e.set(this, Dof::fRange, quiet(range));
    e.set(this, Dof::fMaskConst, quiet(mask_constant));
    e.set(this, Dof::Mode, mode);
    this
}

// Translated from 0052a010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDOF::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys the object and frees its 0x30 bytes when bit 0 of `flags` is set.
pub fn image_space_modifier_instance_dof_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Dof>,
    flags: u32,
) -> Ptr<Dof> {
    fn_0052a650(e, this.cast());
    delete_if_requested(e, this.addr(), flags, 0x30);
    this
}

// Translated from 0052a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `ImageSpaceModifierInstanceTemp` (unnamed in the
/// engine map): runs the base destructor.
pub fn fn_0052a040(e: &mut Engine, this: Ptr<Temp>) {
    fn_005295c0(e, this.cast());
}

// Translated from 0052a060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDOF::Apply` (Xbox PDB): hands the depth-of-field
/// settings to the effect manager while the strength is above zero.
pub fn dof_apply(e: &mut Engine, this: Ptr<Dof>) {
    let base: Ptr<Base> = this.cast();
    if !begin_apply(e, base) {
        return;
    }
    let manager = e.call(EFFECT_MANAGER, &args![]).u32();
    if pointer_value(e, this.addr() + 0x10) != 0 {
        let target = pointer_value(e, this.addr() + 0x10);
        e.vcall(target, SLOT_TARGET_PREPARE, &args![]);
        let target = pointer_value(e, this.addr() + 0x10);
        let data = e.call(TARGET_OR_DEFAULT, &args![target]).u32();
        let mask = quiet(e.get(this, Dof::fMaskConst));
        let mode = e.get(this, Dof::Mode);
        let strength = quiet(e.get(base, Base::fStrength));
        e.call(
            MANAGER_DOF_WITH_TARGET,
            &args![manager, strength, mode, mask, data],
        );
    } else {
        let range = quiet(e.get(this, Dof::fRange));
        let distance = quiet(e.get(this, Dof::fDistance));
        let mask = quiet(e.get(this, Dof::fMaskConst));
        let mode = e.get(this, Dof::Mode);
        let strength = quiet(e.get(base, Base::fStrength));
        e.call(
            MANAGER_DOF_WITHOUT_TARGET,
            &args![manager, strength, mode, mask, distance, range],
        );
    }
}

// Translated from 0052a150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDOF::Trigger` (Xbox PDB, cdecl): starts a
/// depth-of-field instance (no target, "not started" start time) and adds
/// it to the running list. Returns the instance.
// Translation incomplete: 0052a150 - C++ exception-unwinding frame (FS:[0]) not translated.
pub fn dof_trigger(
    e: &mut Engine,
    distance: f32,
    range: f32,
    mode: u32,
    strength: f32,
    mask_constant: f32,
    duration: f32,
) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    let instance: Ptr = if memory != 0 {
        let not_started = quiet(e.global::<f32>(NOT_STARTED));
        fn_00529fb0(
            e,
            Ptr::new(memory),
            strength,
            not_started,
            mode,
            distance,
            range,
            mask_constant,
            Ptr::NULL,
            duration,
        )
        .cast()
    } else {
        Ptr::NULL
    };
    add_to_list(e, instance);
    instance
}

// Translated from 0052a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDOF::PrintInfo` (Xbox PDB): writes the
/// distance, range and strength into `buffer` (0x800 bytes).
pub fn dof_print_info(e: &mut Engine, this: Ptr<Dof>, buffer: Ptr) {
    let distance = quiet(e.get(this, Dof::fDistance)) as f64;
    let range = quiet(e.get(this, Dof::fRange)) as f64;
    let strength = quiet(e.get(this.cast::<Base>(), Base::fStrength)) as f64;
    e.call(
        FORMAT_STRING,
        &args![buffer, 0x800u32, DOF_FORMAT, distance, range, strength],
    );
}

// Translated from 0052a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ImageSpaceModifierInstanceRB` (unnamed in the engine
/// map): a radial-blur instance; `center` points at the `NiPoint2`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0052a290(
    e: &mut Engine,
    this: Ptr<RadialBlur>,
    strength: f32,
    start_time: f32,
    target: Ptr,
    duration: f32,
    rampup: f32,
    start: f32,
    rampdown: f32,
    down_start: f32,
    center: Ptr,
) -> Ptr<RadialBlur> {
    temp_ctor(
        e,
        this.cast(),
        quiet(start_time),
        quiet(strength),
        target,
        quiet(duration),
    );
    e.mem.set_u32(this.addr(), RB_VTABLE);
    e.set(this, RadialBlur::fRampup, quiet(rampup));
    e.set(this, RadialBlur::fStart, quiet(start));
    e.set(this, RadialBlur::fRampdown, quiet(rampdown));
    e.set(this, RadialBlur::fDownStart, quiet(down_start));
    let center_x = e.mem.u32(center.addr());
    let center_y = e.mem.u32(center.addr() + 4);
    e.set(this, RadialBlur::CenterX, center_x);
    e.set(this, RadialBlur::CenterY, center_y);
    this
}

// Translated from 0052a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceRB::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys the object and frees its 0x38 bytes when bit 0 of `flags` is set.
pub fn image_space_modifier_instance_rb_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<RadialBlur>,
    flags: u32,
) -> Ptr<RadialBlur> {
    fn_0052a650(e, this.cast());
    delete_if_requested(e, this.addr(), flags, 0x38);
    this
}

// Translated from 0052a330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceRB::Apply` (Xbox PDB): hands the radial-blur
/// settings to the effect manager while the strength is above zero.
pub fn rb_apply(e: &mut Engine, this: Ptr<RadialBlur>) {
    let base: Ptr<Base> = this.cast();
    if !begin_apply(e, base) {
        return;
    }
    let manager = e.call(EFFECT_MANAGER, &args![]).u32();
    if pointer_value(e, this.addr() + 0x10) != 0 {
        let target = pointer_value(e, this.addr() + 0x10);
        e.vcall(target, SLOT_TARGET_PREPARE, &args![]);
        let target = pointer_value(e, this.addr() + 0x10);
        let data = e.call(TARGET_OR_DEFAULT, &args![target]).u32();
        let down_start = quiet(e.get(this, RadialBlur::fDownStart));
        let rampdown = quiet(e.get(this, RadialBlur::fRampdown));
        let start = quiet(e.get(this, RadialBlur::fStart));
        let rampup = quiet(e.get(this, RadialBlur::fRampup));
        let strength = quiet(e.get(base, Base::fStrength));
        e.call(
            MANAGER_RB_WITH_TARGET,
            &args![manager, strength, rampup, start, rampdown, down_start, data],
        );
    } else {
        let down_start = quiet(e.get(this, RadialBlur::fDownStart));
        let rampdown = quiet(e.get(this, RadialBlur::fRampdown));
        let start = quiet(e.get(this, RadialBlur::fStart));
        let rampup = quiet(e.get(this, RadialBlur::fRampup));
        let strength = quiet(e.get(base, Base::fStrength));
        e.call(
            MANAGER_RB_WITHOUT_TARGET,
            &args![
                manager,
                strength,
                rampup,
                start,
                rampdown,
                down_start,
                this.byte_add(0x30)
            ],
        );
    }
}

// Translated from 0052a440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceRB::Trigger` (Xbox PDB, cdecl): starts a
/// radial-blur instance (no target, "not started" start time) and adds it
/// to the running list. Returns the instance.
// Translation incomplete: 0052a440 - C++ exception-unwinding frame (FS:[0]) not translated.
#[allow(clippy::too_many_arguments)]
pub fn rb_trigger(
    e: &mut Engine,
    strength: f32,
    rampup: f32,
    start: f32,
    rampdown: f32,
    down_start: f32,
    center: Ptr,
    duration: f32,
) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
    let instance: Ptr = if memory != 0 {
        let not_started = quiet(e.global::<f32>(NOT_STARTED));
        fn_0052a290(
            e,
            Ptr::new(memory),
            strength,
            not_started,
            Ptr::NULL,
            duration,
            rampup,
            start,
            rampdown,
            down_start,
            center,
        )
        .cast()
    } else {
        Ptr::NULL
    };
    add_to_list(e, instance);
    instance
}

// Translated from 0052a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceRB::PrintInfo` (Xbox PDB): writes the ramp
/// settings and the strength into `buffer` (0x800 bytes).
pub fn rb_print_info(e: &mut Engine, this: Ptr<RadialBlur>, buffer: Ptr) {
    let strength = quiet(e.get(this.cast::<Base>(), Base::fStrength)) as f64;
    let down_start = quiet(e.get(this, RadialBlur::fDownStart)) as f64;
    let rampdown = quiet(e.get(this, RadialBlur::fRampdown)) as f64;
    let start = quiet(e.get(this, RadialBlur::fStart)) as f64;
    let rampup = quiet(e.get(this, RadialBlur::fRampup)) as f64;
    e.call(
        FORMAT_STRING,
        &args![buffer, 0x800u32, RB_FORMAT, rampup, start, rampdown, down_start, strength],
    );
}

// Translated from 0052a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ImageSpaceModifierInstanceDRB` (unnamed in the engine
/// map): a depth-radial-blur instance; `center` points at the `NiPoint2`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0052a590(
    e: &mut Engine,
    this: Ptr<DepthRadialBlur>,
    strength: f32,
    start_time: f32,
    target: Ptr,
    duration: f32,
    mode: u32,
    distance: f32,
    range: f32,
    rampup: f32,
    start: f32,
    rampdown: f32,
    down_start: f32,
    center: Ptr,
) -> Ptr<DepthRadialBlur> {
    temp_ctor(
        e,
        this.cast(),
        quiet(start_time),
        quiet(strength),
        target,
        quiet(duration),
    );
    e.mem.set_u32(this.addr(), DRB_VTABLE);
    e.set(this, DepthRadialBlur::fRampup, quiet(rampup));
    e.set(this, DepthRadialBlur::fStart, quiet(start));
    e.set(this, DepthRadialBlur::fRampdown, quiet(rampdown));
    e.set(this, DepthRadialBlur::fDownStart, quiet(down_start));
    let center_x = e.mem.u32(center.addr());
    let center_y = e.mem.u32(center.addr() + 4);
    e.set(this, DepthRadialBlur::CenterX, center_x);
    e.set(this, DepthRadialBlur::CenterY, center_y);
    e.set(this, DepthRadialBlur::fDistance, quiet(distance));
    e.set(this, DepthRadialBlur::fRange, quiet(range));
    e.set(this, DepthRadialBlur::Mode, mode);
    this
}

// Translated from 0052a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDRB::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys the object and frees its 0x44 bytes when bit 0 of `flags` is set.
pub fn image_space_modifier_instance_drb_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<DepthRadialBlur>,
    flags: u32,
) -> Ptr<DepthRadialBlur> {
    fn_0052a650(e, this.cast());
    delete_if_requested(e, this.addr(), flags, 0x44);
    this
}

// Translated from 0052a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body shared by the depth-of-field, radial-blur and
/// depth-radial-blur instances (unnamed in the engine map): runs the
/// `Temp` destructor.
pub fn fn_0052a650(e: &mut Engine, this: Ptr<Temp>) {
    fn_0052a040(e, this);
}

// Translated from 0052a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDRB::Apply` (Xbox PDB): hands the depth-radial-blur
/// settings to the effect manager while the strength is above zero.
pub fn drb_apply(e: &mut Engine, this: Ptr<DepthRadialBlur>) {
    let base: Ptr<Base> = this.cast();
    if !begin_apply(e, base) {
        return;
    }
    let manager = e.call(EFFECT_MANAGER, &args![]).u32();
    if pointer_value(e, this.addr() + 0x10) != 0 {
        let target = pointer_value(e, this.addr() + 0x10);
        e.vcall(target, SLOT_TARGET_PREPARE, &args![]);
        let target = pointer_value(e, this.addr() + 0x10);
        let data = e.call(TARGET_OR_DEFAULT, &args![target]).u32();
        let start = quiet(e.get(this, DepthRadialBlur::fStart));
        let rampup = quiet(e.get(this, DepthRadialBlur::fRampup));
        let mode = e.get(this, DepthRadialBlur::Mode);
        let strength = quiet(e.get(base, Base::fStrength));
        e.call(
            MANAGER_DRB_WITH_TARGET,
            &args![manager, strength, mode, rampup, start, data],
        );
    } else {
        let start = quiet(e.get(this, DepthRadialBlur::fStart));
        let rampup = quiet(e.get(this, DepthRadialBlur::fRampup));
        let range = quiet(e.get(this, DepthRadialBlur::fRange));
        let distance = quiet(e.get(this, DepthRadialBlur::fDistance));
        let mode = e.get(this, DepthRadialBlur::Mode);
        let strength = quiet(e.get(base, Base::fStrength));
        e.call(
            MANAGER_DRB_WITHOUT_TARGET,
            &args![
                manager,
                strength,
                mode,
                distance,
                range,
                rampup,
                start,
                this.byte_add(0x30)
            ],
        );
    }
}

// Translated from 0052a780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDRB::Trigger` (Xbox PDB, cdecl): starts a
/// depth-radial-blur instance (no target, "not started" start time) and
/// adds it to the running list. Returns the instance.
// Translation incomplete: 0052a780 - C++ exception-unwinding frame (FS:[0]) not translated.
#[allow(clippy::too_many_arguments)]
pub fn drb_trigger(
    e: &mut Engine,
    distance: f32,
    range: f32,
    mode: u32,
    strength: f32,
    rampup: f32,
    start: f32,
    rampdown: f32,
    down_start: f32,
    center: Ptr,
    duration: f32,
) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x44u32]).u32();
    let instance: Ptr = if memory != 0 {
        let not_started = quiet(e.global::<f32>(NOT_STARTED));
        fn_0052a590(
            e,
            Ptr::new(memory),
            strength,
            not_started,
            Ptr::NULL,
            duration,
            mode,
            distance,
            range,
            rampup,
            start,
            rampdown,
            down_start,
            center,
        )
        .cast()
    } else {
        Ptr::NULL
    };
    add_to_list(e, instance);
    instance
}

// Translated from 0052a880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceDRB::PrintInfo` (Xbox PDB): writes the fixed
/// text `  Depth Radial Blur` into `buffer` (0x800 bytes).
pub fn drb_print_info(e: &mut Engine, _this: Ptr<DepthRadialBlur>, buffer: Ptr) {
    e.call(FORMAT_STRING, &args![buffer, 0x800u32, DRB_FORMAT]);
}

// Translated from 0052a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ImageSpaceModifierInstanceTemp` (unnamed in the engine
/// map): the base object and the duration.
pub fn temp_ctor(
    e: &mut Engine,
    this: Ptr<Temp>,
    start_time: f32,
    strength: f32,
    target: Ptr,
    duration: f32,
) -> Ptr<Temp> {
    fn_00529540(e, this.cast(), quiet(strength), quiet(start_time), target);
    e.mem.set_u32(this.addr(), TEMP_VTABLE);
    e.set(this, Temp::fDuration, quiet(duration));
    this
}

// Translated from 0052a8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `ImageSpaceModifierInstanceTemp` (unnamed in
/// the engine map): destroys the object and frees its 0x20 bytes when bit 0
/// of `flags` is set.
pub fn fn_0052a8f0(e: &mut Engine, this: Ptr<Temp>, flags: u32) -> Ptr<Temp> {
    fn_0052a040(e, this);
    delete_if_requested(e, this.addr(), flags, 0x20);
    this
}

// Translated from 0052a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceModifierInstanceTemp::IsExpired` (Xbox PDB): true when the
/// duration is not zero and shorter than the time since the start.
pub fn temp_is_expired(e: &mut Engine, this: Ptr<Temp>) -> bool {
    let zero = e.global::<f64>(ZERO);
    let duration = e.get(this, Temp::fDuration) as f64;
    if duration == zero {
        return false;
    }
    let now = fn_00529990(e);
    let elapsed = now as f64 - e.get(this.cast::<Base>(), Base::fStartTime) as f64;
    duration < elapsed
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005293f0, image_space_modifier_instance_stop(Ptr<Base>)),
        entry!(0x00529490, fn_00529490(Ptr<Form>) -> Ptr<Form>),
        entry!(0x00529530, ni_object_get_rtti(Ptr) -> u32),
        entry!(
            0x00529540,
            fn_00529540(Ptr<Base>, f32, f32, Ptr) -> Ptr<Base>
        ),
        entry!(0x005295c0, fn_005295c0(Ptr<Base>)),
        entry!(
            0x00529630,
            image_space_modifier_instance_scalar_deleting_destructor(Ptr<Base>, u32) -> Ptr<Base>
        ),
        entry!(
            0x00529660,
            image_space_modifier_instance_form_scalar_deleting_destructor(
                Ptr<Form>,
                u32,
            ) -> Ptr<Form>
        ),
        entry!(0x00529690, fn_00529690(Ptr<Form>)),
        entry!(
            0x00529700,
            form_ctor(Ptr<Form>, Ptr, f32, f32, Ptr) -> Ptr<Form>
        ),
        entry!(0x005297b0, form_is_expired(Ptr<Form>) -> bool),
        entry!(0x005297e0, form_apply(Ptr<Form>)),
        entry!(0x00529990, fn_00529990() -> f32),
        entry!(0x005299a0, form_trigger(Ptr, f32, Ptr) -> Ptr),
        entry!(0x00529ae0, form_trigger_if_not_active(Ptr, f32, Ptr) -> Ptr),
        entry!(0x00529c90, form_stop(Ptr)),
        entry!(
            0x00529df0,
            fn_00529df0(Ptr<Form>, Ptr, f32, f32, Ptr) -> bool
        ),
        entry!(0x00529ea0, fn_00529ea0(Ptr) -> u8),
        entry!(0x00529ec0, form_print_info(Ptr<Form>, Ptr)),
        entry!(
            0x00529fb0,
            fn_00529fb0(Ptr<Dof>, f32, f32, u32, f32, f32, f32, Ptr, f32) -> Ptr<Dof>
        ),
        entry!(
            0x0052a010,
            image_space_modifier_instance_dof_scalar_deleting_destructor(Ptr<Dof>, u32) -> Ptr<Dof>
        ),
        entry!(0x0052a040, fn_0052a040(Ptr<Temp>)),
        entry!(0x0052a060, dof_apply(Ptr<Dof>)),
        entry!(0x0052a150, dof_trigger(f32, f32, u32, f32, f32, f32) -> Ptr),
        entry!(0x0052a240, dof_print_info(Ptr<Dof>, Ptr)),
        entry!(
            0x0052a290,
            fn_0052a290(
                Ptr<RadialBlur>,
                f32,
                f32,
                Ptr,
                f32,
                f32,
                f32,
                f32,
                f32,
                Ptr,
            ) -> Ptr<RadialBlur>
        ),
        entry!(
            0x0052a300,
            image_space_modifier_instance_rb_scalar_deleting_destructor(
                Ptr<RadialBlur>,
                u32,
            )
                -> Ptr<RadialBlur>
        ),
        entry!(0x0052a330, rb_apply(Ptr<RadialBlur>)),
        entry!(0x0052a440, rb_trigger(f32, f32, f32, f32, f32, Ptr, f32) -> Ptr),
        entry!(0x0052a530, rb_print_info(Ptr<RadialBlur>, Ptr)),
        entry!(
            0x0052a590,
            fn_0052a590(
                Ptr<DepthRadialBlur>,
                f32,
                f32,
                Ptr,
                f32,
                u32,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                Ptr,
            ) -> Ptr<DepthRadialBlur>
        ),
        entry!(
            0x0052a620,
            image_space_modifier_instance_drb_scalar_deleting_destructor(
                Ptr<DepthRadialBlur>,
                u32,
            )
                -> Ptr<DepthRadialBlur>
        ),
        entry!(0x0052a650, fn_0052a650(Ptr<Temp>)),
        entry!(0x0052a670, drb_apply(Ptr<DepthRadialBlur>)),
        entry!(0x0052a780, drb_trigger(f32, f32, u32, f32, f32, f32, f32, f32, Ptr, f32) -> Ptr),
        entry!(0x0052a880, drb_print_info(Ptr<DepthRadialBlur>, Ptr)),
        entry!(
            0x0052a8b0,
            temp_ctor(Ptr<Temp>, f32, f32, Ptr, f32) -> Ptr<Temp>
        ),
        entry!(0x0052a8f0, fn_0052a8f0(Ptr<Temp>, u32) -> Ptr<Temp>),
        entry!(0x0052a920, temp_is_expired(Ptr<Temp>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A recorded call: the callee and its argument words.
    type Call = (u32, Vec<u32>);

    /// A vtable whose `+0x94` slot answers the object itself (a form instance).
    const VTABLE_FORM_INSTANCE: u32 = 0x0300_0000;
    /// A vtable whose `+0x94` slot answers null (any other instance).
    const VTABLE_PLAIN_INSTANCE: u32 = 0x0300_1000;
    const AS_FORM_ITSELF: u32 = 0x0070_0001;
    const AS_FORM_NULL: u32 = 0x0070_0002;
    const TARGET_PREPARE: u32 = 0x0070_0003;
    const NAME_OF: u32 = 0x0070_0004;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    fn double_words(value: f64) -> Vec<u32> {
        let raw = value.to_bits();
        vec![raw as u32, (raw >> 32) as u32]
    }

    /// `BSSimpleList::Remove` on the node: the first node's item is dropped
    /// by pulling the next node into it (an emptied last node keeps a null item).
    fn list_remove(e: &mut Engine, a: &[u32]) -> Ret {
        let node = a[0];
        let item = e.mem.u32(a[1]);
        if e.mem.u32(node) == item {
            let next = e.mem.u32(node + 4);
            if next != 0 {
                let next_item = e.mem.u32(next);
                let next_next = e.mem.u32(next + 4);
                e.mem.set_u32(node, next_item);
                e.mem.set_u32(node + 4, next_next);
            } else {
                e.mem.set_u32(node, 0);
            }
        }
        ret(1)
    }

    /// Appends the `NiPointer`'s item to the list at the node.
    fn list_add(e: &mut Engine, a: &[u32]) -> Ret {
        let mut node = a[0];
        let item = e.mem.u32(a[1]);
        if e.mem.u32(node) == 0 && e.mem.u32(node + 4) == 0 {
            e.mem.set_u32(node, item);
        } else {
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, item);
            e.mem.set_u32(node + 4, fresh);
        }
        ret(1)
    }

    /// An engine with the doubles of every callee outside this file, the
    /// constants the code reads and an empty list of running instances.
    fn world() -> Engine {
        let mut e = Engine::new();
        for (address, length) in [
            (LIST_OWNER, 4),
            (CURRENT_TIME, 4),
            (ZERO, 8),
            (ONE, 8),
            (NOT_STARTED_LIMIT, 8),
            (NOT_STARTED, 4),
            (VTABLE_FORM_INSTANCE, 0x2000),
        ] {
            e.map(address, length);
        }
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_f64(NOT_STARTED_LIMIT, -10000.0);
        e.mem.set_f32(NOT_STARTED, -10000.0);
        e.mem.set_f32(CURRENT_TIME, 7.5);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(LIST_OWNER, owner);
        for vtable in [VTABLE_FORM_INSTANCE, VTABLE_PLAIN_INSTANCE] {
            e.mem.set_u32(vtable + SLOT_TARGET_PREPARE, TARGET_PREPARE);
            e.mem.set_u32(vtable + SLOT_NAME, NAME_OF);
        }
        e.mem
            .set_u32(VTABLE_FORM_INSTANCE + SLOT_AS_FORM, AS_FORM_ITSELF);
        e.mem
            .set_u32(VTABLE_PLAIN_INSTANCE + SLOT_AS_FORM, AS_FORM_NULL);
        e.register(AS_FORM_ITSELF, |_, a| ret(a[0]));
        e.register(AS_FORM_NULL, |_, _| ret(0));
        e.register(TARGET_PREPARE, nothing);
        e.register(NAME_OF, |_, a| ret(a[0] + 0x1000));
        e.register(NI_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_OBJECT_DESTRUCT, nothing);
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_ASSIGN_FROM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(NI_POINTER_DESTRUCT, nothing);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE_SIZED, nothing);
        e.register(FORMAT_STRING, nothing);
        e.register(LIST_OF_OWNER, |_, a| ret(a[0] + 0x6c));
        e.register(NODE_SELF, |_, a| ret(a[0]));
        e.register(NODE_ITEM, |e, a| ret(e.mem.u32(a[0])));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_REMOVE, list_remove);
        e.register(LIST_ADD, list_add);
        e.register(FORM_IS_EXPIRED, |_, _| ret(1));
        e.register(APPLY_FORM_WEATHER_00531D20, nothing);
        e.register(APPLY_FORM_00531720, nothing);
        e.register(APPLY_FORM_BLEND_00532220, nothing);
        e.register(GET_FIELD_1C, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(GET_FIELD_18, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(GET_FIELD_0C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(EFFECT_MANAGER, |_, _| ret(0x4141));
        e.register(TARGET_OR_DEFAULT, |_, _| ret(0x7777));
        for manager_call in [
            MANAGER_DOF_WITHOUT_TARGET,
            MANAGER_DOF_WITH_TARGET,
            MANAGER_RB_WITHOUT_TARGET,
            MANAGER_RB_WITH_TARGET,
            MANAGER_DRB_WITHOUT_TARGET,
            MANAGER_DRB_WITH_TARGET,
        ] {
            e.register(manager_call, nothing);
        }
        e.register(AUDIO_INSTANCE, |_, _| ret(0xa0d1));
        e.register(GET_SOUND_HANDLE, |_, a| ret(a[1]));
        e.register(PLAY_SOUND_HANDLE, |_, _| ret(1));
        e.register(SOUND_HANDLE_DESTRUCT, nothing);
        e
    }

    fn record(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The recorded calls after the top-level one.
    fn recorded(e: &mut Engine) -> Vec<Call> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    fn calls_to(calls: &[Call], address: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Makes the list of running instances hold `items`.
    fn set_list(e: &mut Engine, items: &[u32]) {
        let mut node = e.mem.u32(LIST_OWNER) + 0x6c;
        for (index, item) in items.iter().enumerate() {
            if index > 0 {
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(node + 4, fresh);
                node = fresh;
            }
            e.mem.set_u32(node, *item);
        }
    }

    fn list_items(e: &Engine) -> Vec<u32> {
        let mut node = e.mem.u32(LIST_OWNER) + 0x6c;
        let mut items = vec![];
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// A form instance (answers itself as a form) running `form`.
    fn form_instance(e: &mut Engine, form: u32) -> u32 {
        let instance = e.mem.alloc(0x30);
        e.mem.set_u32(instance, VTABLE_FORM_INSTANCE);
        e.mem.set_u32(instance + 0x1c, form);
        instance
    }

    /// An instance that is not a form instance (also a valid target object).
    fn plain_instance(e: &mut Engine) -> u32 {
        let instance = e.mem.alloc(0x48);
        e.mem.set_u32(instance, VTABLE_PLAIN_INSTANCE);
        instance
    }

    /// A form whose sound getter at `+offset` yields an object with id `id`.
    fn form_with_sound(e: &mut Engine, offset: u32, id: u32) -> u32 {
        let form = e.mem.alloc(0x40);
        let sound = e.mem.alloc(0x10);
        e.mem.set_u32(sound + 0xc, id);
        e.mem.set_u32(form + offset, sound);
        form
    }

    fn dtor_calls(this: u32) -> Vec<Call> {
        vec![
            (NI_POINTER_ASSIGN, vec![this + 0x10, 0]),
            (NI_POINTER_DESTRUCT, vec![this + 0x10]),
            (NI_OBJECT_DESTRUCT, vec![this]),
        ]
    }

    #[test]
    fn stop_removes_every_entry_of_the_instance() {
        let mut e = world();
        let wanted = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        set_list(&mut e, &[wanted, other, wanted]);
        record(&mut e);
        e.call(0x005293f0, &args![wanted]);
        let log = recorded(&mut e);
        assert_eq!(list_items(&e), vec![other, 0]);
        assert_eq!(calls_to(&log, LIST_REMOVE).len(), 2);
        assert_eq!(calls_to(&log, NI_POINTER_CONSTRUCT)[0][1], wanted);
    }

    #[test]
    fn stop_without_a_match_leaves_the_list() {
        let mut e = world();
        let other = e.mem.alloc(0x20);
        set_list(&mut e, &[other]);
        record(&mut e);
        e.call(0x005293f0, &args![0x1234u32]);
        assert!(calls_to(&recorded(&mut e), LIST_REMOVE).is_empty());
        assert_eq!(list_items(&e), vec![other]);
    }

    #[test]
    fn default_form_constructor_clears_the_instance() {
        let mut e = world();
        let this = e.mem.alloc(0x30);
        for offset in (4..0x30).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        record(&mut e);
        assert_eq!(e.call(0x00529490, &args![this]).u32(), this);
        assert_eq!(
            recorded(&mut e),
            vec![
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_POINTER_CONSTRUCT, vec![this + 0x10, 0]),
                (NI_POINTER_CONSTRUCT, vec![this + 0x28, 0]),
            ]
        );
        assert_eq!(e.mem.u32(this), FORM_VTABLE);
        for offset in [0xc, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 8), 0);
    }

    #[test]
    fn get_rtti_answers_the_ni_object_type() {
        let mut e = world();
        assert_eq!(e.call(0x00529530, &args![0x10u32]).u32(), NI_OBJECT_RTTI);
    }

    #[test]
    fn base_constructor_stores_the_arguments_and_quiets_nans() {
        let mut e = world();
        let this = e.mem.alloc(0x1c);
        let target = e.mem.alloc(0x10);
        let signalling = f32::from_bits(0x7fa0_0000);
        record(&mut e);
        let result = e.call(0x00529540, &args![this, signalling, 2.5f32, target]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            recorded(&mut e),
            vec![
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_POINTER_CONSTRUCT, vec![this + 0x10, target]),
            ]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0x14), bits(2.5));
        assert_eq!(e.mem.u32(this + 0x10), target);
    }

    #[test]
    fn base_destructor_releases_the_target_then_the_object() {
        let mut e = world();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x10, 0x5555);
        record(&mut e);
        e.call(0x005295c0, &args![this]);
        assert_eq!(recorded(&mut e), dtor_calls(this));
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn base_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = world();
        let this = e.mem.alloc(0x1c);
        record(&mut e);
        assert_eq!(e.call(0x00529630, &args![this, 0u32]).u32(), this);
        assert_eq!(recorded(&mut e), dtor_calls(this));
        record(&mut e);
        e.call(0x00529630, &args![this, 3u32]);
        let mut expected = dtor_calls(this);
        expected.push((OPERATOR_DELETE_SIZED, vec![this, 0x1c]));
        assert_eq!(recorded(&mut e), expected);
    }

    #[test]
    fn form_destructor_releases_the_previous_target_first() {
        let mut e = world();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x28, 0x6666);
        record(&mut e);
        e.call(0x00529690, &args![this]);
        let mut expected = vec![
            (NI_POINTER_ASSIGN, vec![this + 0x28, 0]),
            (NI_POINTER_DESTRUCT, vec![this + 0x28]),
        ];
        expected.extend(dtor_calls(this));
        assert_eq!(recorded(&mut e), expected);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn form_scalar_deleting_destructor_frees_0x30_bytes() {
        let mut e = world();
        let this = e.mem.alloc(0x30);
        record(&mut e);
        assert_eq!(e.call(0x00529660, &args![this, 1u32]).u32(), this);
        let log = recorded(&mut e);
        assert_eq!(log.len(), 6);
        assert_eq!(log[5], (OPERATOR_DELETE_SIZED, vec![this, 0x30]));
        record(&mut e);
        e.call(0x00529660, &args![this, 0u32]);
        assert!(calls_to(&recorded(&mut e), OPERATOR_DELETE_SIZED).is_empty());
    }

    #[test]
    fn form_constructor_stores_the_form_and_clears_the_blend() {
        let mut e = world();
        let this = e.mem.alloc(0x30);
        for offset in (0x1c..0x30).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let target = e.mem.alloc(0x10);
        let result = e.call(
            0x00529700,
            &args![this, 0xf0f0u32, -5.0f32, 0.75f32, target],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), FORM_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), bits(0.75));
        assert_eq!(e.mem.u32(this + 0x14), bits(-5.0));
        assert_eq!(e.mem.u32(this + 0x10), target);
        assert_eq!(e.mem.u32(this + 0x1c), 0xf0f0);
        for offset in [0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 8), 0);
    }

    #[test]
    fn form_is_expired_asks_the_form() {
        let mut e = world();
        let this = form_instance(&mut e, 0);
        assert!(!e.call(0x005297b0, &args![this]).bool());
        let this = form_instance(&mut e, 0xf0f0);
        e.mem.set_f32(this + 0x14, 3.5);
        record(&mut e);
        assert!(e.call(0x005297b0, &args![this]).bool());
        assert_eq!(
            recorded(&mut e),
            vec![(FORM_IS_EXPIRED, vec![0xf0f0, bits(3.5)])]
        );
        e.register(FORM_IS_EXPIRED, |_, _| ret(0));
        assert!(!e.call(0x005297b0, &args![this]).bool());
    }

    /// A form instance running form `0xf0f0` with strength 1.0, a target
    /// and the start time 5.5.
    fn running_form(e: &mut Engine) -> (u32, u32) {
        let this = form_instance(e, 0xf0f0);
        let target = plain_instance(e);
        e.mem.set_f32(this + 0xc, 1.0);
        e.mem.set_u32(this + 0x10, target);
        e.mem.set_f32(this + 0x14, 5.5);
        (this, target)
    }

    #[test]
    fn form_apply_does_nothing_without_strength_or_form() {
        let mut e = world();
        let (this, _) = running_form(&mut e);
        for strength in [0.0f32, -1.0, f32::NAN] {
            e.mem.set_f32(this + 0xc, strength);
            record(&mut e);
            e.call(0x005297e0, &args![this]);
            assert!(recorded(&mut e).is_empty(), "{strength}");
        }
        e.mem.set_f32(this + 0xc, 1.0);
        e.mem.set_u32(this + 0x1c, 0);
        record(&mut e);
        e.call(0x005297e0, &args![this]);
        assert!(recorded(&mut e).is_empty());
    }

    #[test]
    fn form_apply_applies_the_form_with_its_target() {
        let mut e = world();
        let (this, target) = running_form(&mut e);
        record(&mut e);
        e.call(0x005297e0, &args![this]);
        assert_eq!(
            recorded(&mut e),
            vec![
                (NODE_ITEM, vec![this + 0x10]),
                (
                    APPLY_FORM_00531720,
                    vec![0xf0f0, bits(5.5), bits(1.0), target]
                ),
            ]
        );
    }

    #[test]
    fn form_apply_sets_an_unset_start_time_but_not_the_limit_itself() {
        let mut e = world();
        let (this, _) = running_form(&mut e);
        e.mem.set_f32(this + 0x14, -10000.0);
        e.call(0x005297e0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x14), -10000.0);
        e.mem.set_f32(this + 0x14, -10001.0);
        e.call(0x005297e0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x14), 7.5);
        e.mem.set_f32(this + 0x14, f32::NAN);
        e.call(0x005297e0, &args![this]);
        assert!(e.mem.f32(this + 0x14).is_nan());
    }

    #[test]
    fn form_apply_as_weather_skips_the_target() {
        let mut e = world();
        let (this, _) = running_form(&mut e);
        e.mem.set_u8(this + 8, 1);
        record(&mut e);
        e.call(0x005297e0, &args![this]);
        assert_eq!(
            recorded(&mut e),
            vec![(
                APPLY_FORM_WEATHER_00531D20,
                vec![0xf0f0, bits(5.5), bits(1.0)]
            )]
        );
    }

    #[test]
    fn form_apply_runs_on_the_previous_strength_alone() {
        let mut e = world();
        let (this, _) = running_form(&mut e);
        e.mem.set_f32(this + 0xc, 0.0);
        e.mem.set_f32(this + 0x24, 0.25);
        record(&mut e);
        e.call(0x005297e0, &args![this]);
        assert_eq!(calls_to(&recorded(&mut e), APPLY_FORM_00531720).len(), 1);
    }

    #[test]
    fn form_apply_blends_from_the_previous_form_during_the_transition() {
        let mut e = world();
        let (this, target) = running_form(&mut e);
        let last_target = plain_instance(&mut e);
        e.mem.set_u32(this + 0x20, 0xabab);
        e.mem.set_f32(this + 0x24, 0.5);
        e.mem.set_u32(this + 0x28, last_target);
        e.mem.set_f32(this + 0x2c, 4.0);
        record(&mut e);
        e.call(0x005297e0, &args![this]);
        assert_eq!(
            recorded(&mut e),
            vec![
                (NODE_ITEM, vec![this + 0x10]),
                (NODE_ITEM, vec![this + 0x28]),
                (
                    APPLY_FORM_BLEND_00532220,
                    vec![
                        bits(0.5),
                        0xabab,
                        0xf0f0,
                        bits(0.5),
                        bits(1.0),
                        last_target,
                        target
                    ]
                ),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x20), 0xabab);
    }

    #[test]
    fn form_apply_ends_the_transition_when_the_blend_reaches_one() {
        let mut e = world();
        let (this, target) = running_form(&mut e);
        e.mem.set_u32(this + 0x20, 0xabab);
        e.mem.set_f32(this + 0x24, 0.5);
        e.mem.set_f32(this + 0x2c, 2.0);
        record(&mut e);
        e.call(0x005297e0, &args![this]);
        let log = recorded(&mut e);
        assert!(calls_to(&log, APPLY_FORM_BLEND_00532220).is_empty());
        assert_eq!(
            calls_to(&log, APPLY_FORM_00531720),
            vec![vec![0xf0f0, bits(5.5), bits(1.0), target]]
        );
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        // A transition time of zero divides by zero: infinity is not below 1.0.
        e.mem.set_u32(this + 0x20, 0xabab);
        e.mem.set_f32(this + 0x2c, 0.0);
        e.call(0x005297e0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }

    #[test]
    fn current_time_is_the_game_time_float() {
        let mut e = world();
        assert_eq!(fn_00529990(&mut e), 7.5);
        e.mem.set_u32(CURRENT_TIME, 0x7fa0_0000);
        assert_eq!(fn_00529990(&mut e).to_bits(), 0x7fe0_0000);
        e.mem.set_f32(CURRENT_TIME, 2.0);
        assert_eq!(e.call(0x00529990, &args![]).f32(), 2.0);
    }

    #[test]
    fn form_trigger_starts_an_instance_lists_it_and_plays_the_sound() {
        let mut e = world();
        let form = form_with_sound(&mut e, 0x1c, 0x77);
        let target = plain_instance(&mut e);
        record(&mut e);
        let instance = e.call(0x005299a0, &args![form, 0.75f32, target]).u32();
        let log = recorded(&mut e);
        assert_ne!(instance, 0);
        assert_eq!(list_items(&e), vec![instance]);
        assert_eq!(e.mem.u32(instance), FORM_VTABLE);
        assert_eq!(e.mem.u32(instance + 0x1c), form);
        assert_eq!(e.mem.u32(instance + 0xc), bits(0.75));
        assert_eq!(e.mem.u32(instance + 0x14), bits(-10000.0));
        assert_eq!(e.mem.u32(instance + 0x10), target);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x30]]);
        let handles = calls_to(&log, GET_SOUND_HANDLE);
        assert_eq!(handles.len(), 1);
        assert_eq!(handles[0][0], 0xa0d1);
        assert_eq!(&handles[0][2..], &[0x77, 0x0020_0000]);
        assert_eq!(
            calls_to(&log, PLAY_SOUND_HANDLE),
            vec![vec![handles[0][1], 0]]
        );
        assert_eq!(calls_to(&log, SOUND_HANDLE_DESTRUCT).len(), 1);
    }

    #[test]
    fn form_trigger_without_a_form_or_a_sound() {
        let mut e = world();
        record(&mut e);
        assert_eq!(e.call(0x005299a0, &args![0u32, 1.0f32, 0u32]).u32(), 0);
        assert!(recorded(&mut e).is_empty());
        let form = e.mem.alloc(0x40);
        record(&mut e);
        let instance = e.call(0x005299a0, &args![form, 1.0f32, 0u32]).u32();
        assert!(calls_to(&recorded(&mut e), GET_SOUND_HANDLE).is_empty());
        assert_eq!(list_items(&e), vec![instance]);
    }

    #[test]
    fn form_trigger_with_a_failed_allocation_lists_a_null_pointer() {
        let mut e = world();
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let form = e.mem.alloc(0x40);
        assert_eq!(e.call(0x005299a0, &args![form, 1.0f32, 0u32]).u32(), 0);
        assert_eq!(list_items(&e), vec![0]);
    }

    #[test]
    fn trigger_if_not_active_does_nothing_when_the_form_is_running() {
        let mut e = world();
        let form = form_with_sound(&mut e, 0x1c, 0x77);
        let other_form = e.mem.alloc(0x40);
        let plain = plain_instance(&mut e);
        let other = form_instance(&mut e, other_form);
        let running = form_instance(&mut e, form);
        set_list(&mut e, &[plain, other, running]);
        record(&mut e);
        assert_eq!(e.call(0x00529ae0, &args![form, 1.0f32, 0u32]).u32(), 0);
        let log = recorded(&mut e);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(list_items(&e), vec![plain, other, running]);
    }

    #[test]
    fn trigger_if_not_active_starts_the_form_otherwise() {
        let mut e = world();
        let form = form_with_sound(&mut e, 0x1c, 0x77);
        let other_form = e.mem.alloc(0x40);
        let plain = plain_instance(&mut e);
        let other = form_instance(&mut e, other_form);
        set_list(&mut e, &[plain, other]);
        record(&mut e);
        let instance = e.call(0x00529ae0, &args![form, 0.5f32, 0u32]).u32();
        let log = recorded(&mut e);
        assert_ne!(instance, 0);
        assert_eq!(list_items(&e), vec![plain, other, instance]);
        assert_eq!(e.mem.u32(instance + 0x1c), form);
        assert_eq!(calls_to(&log, GET_SOUND_HANDLE).len(), 1);
        assert_eq!(e.call(0x00529ae0, &args![0u32, 0.5f32, 0u32]).u32(), 0);
    }

    #[test]
    fn form_stop_removes_the_instances_and_plays_the_sound_once() {
        let mut e = world();
        let form = form_with_sound(&mut e, 0x18, 0x99);
        let plain = plain_instance(&mut e);
        let first = form_instance(&mut e, form);
        let second = form_instance(&mut e, form);
        set_list(&mut e, &[first, plain, second]);
        record(&mut e);
        e.call(0x00529c90, &args![form]);
        let log = recorded(&mut e);
        assert_eq!(list_items(&e), vec![plain, 0]);
        assert_eq!(calls_to(&log, LIST_REMOVE).len(), 2);
        assert_eq!(calls_to(&log, GET_FIELD_18).len(), 3);
        let handles = calls_to(&log, GET_SOUND_HANDLE);
        assert_eq!(handles.len(), 1);
        assert_eq!(&handles[0][2..], &[0x99, 0x0020_0000]);
    }

    #[test]
    fn form_stop_without_a_sound_or_a_form() {
        let mut e = world();
        let form = e.mem.alloc(0x40);
        let running = form_instance(&mut e, form);
        set_list(&mut e, &[running]);
        record(&mut e);
        e.call(0x00529c90, &args![form]);
        let log = recorded(&mut e);
        assert_eq!(calls_to(&log, LIST_REMOVE).len(), 1);
        assert!(calls_to(&log, GET_SOUND_HANDLE).is_empty());
        record(&mut e);
        e.call(0x00529c90, &args![0u32]);
        assert!(recorded(&mut e).is_empty());
    }

    #[test]
    fn switching_forms_keeps_the_previous_one_for_the_blend() {
        let mut e = world();
        let current = e.mem.alloc(0x30);
        let next = e.mem.alloc(0x30);
        let this = form_instance(&mut e, current);
        let old_target = plain_instance(&mut e);
        let new_target = plain_instance(&mut e);
        e.mem.set_f32(this + 0xc, 0.8);
        e.mem.set_u32(this + 0x10, old_target);
        record(&mut e);
        let done = e.call(0x00529df0, &args![this, next, 2.0f32, 0.3f32, new_target]);
        assert!(done.bool());
        assert_eq!(
            recorded(&mut e),
            vec![
                (NI_POINTER_ASSIGN_FROM, vec![this + 0x28, this + 0x10]),
                (NI_POINTER_ASSIGN, vec![this + 0x10, new_target]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x20), current);
        assert_eq!(e.mem.f32(this + 0x24), 0.8);
        assert_eq!(e.mem.u32(this + 0x28), old_target);
        assert_eq!(e.mem.f32(this + 0x14), 7.5);
        assert_eq!(e.mem.f32(this + 0x2c), 2.0);
        assert_eq!(e.mem.u32(this + 0x1c), next);
        assert_eq!(e.mem.f32(this + 0xc), 0.3);
        assert_eq!(e.mem.u32(this + 0x10), new_target);
    }

    #[test]
    fn switching_forms_is_refused_for_flagged_forms() {
        let mut e = world();
        let flagged = e.mem.alloc(0x30);
        e.mem.set_u8(flagged + 0x20, 1);
        let free = e.mem.alloc(0x30);
        let this = form_instance(&mut e, flagged);
        let refused = e.call(0x00529df0, &args![this, free, 1.0f32, 1.0f32, 0u32]);
        assert!(!refused.bool());
        assert_eq!(e.mem.u32(this + 0x1c), flagged);
        let this = form_instance(&mut e, free);
        let refused = e.call(0x00529df0, &args![this, flagged, 1.0f32, 1.0f32, 0u32]);
        assert!(!refused.bool());
        assert_eq!(e.mem.u32(this + 0x1c), free);
        // Neither form: allowed.
        let this = form_instance(&mut e, 0);
        let allowed = e.call(0x00529df0, &args![this, 0u32, 1.0f32, 1.0f32, 0u32]);
        assert!(allowed.bool());
    }

    #[test]
    fn flag_byte_getter_reads_plus_0x20() {
        let mut e = world();
        let form = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00529ea0, &args![form]).u8(), 0);
        e.mem.set_u8(form + 0x20, 7);
        assert_eq!(e.call(0x00529ea0, &args![form]).u8(), 7);
    }

    #[test]
    fn form_print_info_describes_the_blend_or_the_form() {
        let mut e = world();
        let buffer = e.mem.alloc(0x800);
        let form = plain_instance(&mut e);
        let last_form = plain_instance(&mut e);
        let this = form_instance(&mut e, form);
        e.mem.set_u32(this + 0x20, last_form);
        record(&mut e);
        e.call(0x00529ec0, &args![this, buffer]);
        let blend = calls_to(&recorded(&mut e), FORMAT_STRING);
        assert_eq!(
            blend,
            vec![vec![
                buffer,
                0x800,
                BLEND_FORMAT,
                form + 0x1000,
                last_form + 0x1000
            ]]
        );
        e.mem.set_u32(this + 0x1c, 0);
        record(&mut e);
        e.call(0x00529ec0, &args![this, buffer]);
        let blend = calls_to(&recorded(&mut e), FORMAT_STRING);
        assert_eq!(blend[0][3..], [NULL_TEXT, last_form + 0x1000]);
        e.mem.set_u32(this + 0x20, 0);
        e.mem.set_f32(this + 0xc, 0.5);
        record(&mut e);
        e.call(0x00529ec0, &args![this, buffer]);
        let mut expected = vec![buffer, 0x800, FORM_FORMAT, NOT_WEATHER_TEXT, NULL_TEXT];
        expected.extend(double_words(0.5));
        assert_eq!(calls_to(&recorded(&mut e), FORMAT_STRING), vec![expected]);
        e.mem.set_u32(this + 0x1c, form);
        e.mem.set_u8(this + 8, 1);
        record(&mut e);
        e.call(0x00529ec0, &args![this, buffer]);
        let weather = calls_to(&recorded(&mut e), FORMAT_STRING);
        assert_eq!(weather[0][3..5], [WEATHER_TEXT, form + 0x1000]);
    }

    #[test]
    fn dof_constructor_stores_the_depth_of_field_settings() {
        let mut e = world();
        let this = e.mem.alloc(0x30);
        let target = e.mem.alloc(0x10);
        let result = e.call(
            0x00529fb0,
            &args![this, 0.9f32, -3.0f32, 2u32, 100.0f32, 20.0f32, 0.4f32, target, 6.0f32],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), DOF_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(this + 0x14), bits(-3.0));
        assert_eq!(e.mem.u32(this + 0x10), target);
        assert_eq!(e.mem.u32(this + 0x1c), bits(6.0));
        assert_eq!(e.mem.u32(this + 0x20), bits(100.0));
        assert_eq!(e.mem.u32(this + 0x24), bits(20.0));
        assert_eq!(e.mem.u32(this + 0x28), bits(0.4));
        assert_eq!(e.mem.u32(this + 0x2c), 2);
    }

    #[test]
    fn temp_family_destructors_free_their_own_sizes() {
        for (address, size) in [
            (0x0052a010u32, 0x30u32),
            (0x0052a300, 0x38),
            (0x0052a620, 0x44),
            (0x0052a8f0, 0x20),
        ] {
            let mut e = world();
            let this = e.mem.alloc(size);
            record(&mut e);
            assert_eq!(e.call(address, &args![this, 1u32]).u32(), this);
            let mut expected = dtor_calls(this);
            expected.push((OPERATOR_DELETE_SIZED, vec![this, size]));
            assert_eq!(recorded(&mut e), expected, "{address:08x}");
            record(&mut e);
            e.call(address, &args![this, 0u32]);
            assert_eq!(recorded(&mut e), dtor_calls(this), "{address:08x}");
        }
    }

    #[test]
    fn temp_and_shared_destructor_bodies_run_the_base_destructor() {
        for address in [0x0052a040u32, 0x0052a650] {
            let mut e = world();
            let this = e.mem.alloc(0x20);
            record(&mut e);
            e.call(address, &args![this]);
            assert_eq!(recorded(&mut e), dtor_calls(this), "{address:08x}");
        }
    }

    /// An instance with strength 0.6, no target and the start time 5.5.
    fn effect_instance(e: &mut Engine, size: u32) -> u32 {
        let this = e.mem.alloc(size);
        e.mem.set_f32(this + 0xc, 0.6);
        e.mem.set_f32(this + 0x14, 5.5);
        this
    }

    #[test]
    fn dof_apply_hands_the_settings_to_the_manager() {
        let mut e = world();
        let this = effect_instance(&mut e, 0x30);
        e.mem.set_f32(this + 0x20, 100.0);
        e.mem.set_f32(this + 0x24, 20.0);
        e.mem.set_f32(this + 0x28, 0.4);
        e.mem.set_u32(this + 0x2c, 2);
        record(&mut e);
        e.call(0x0052a060, &args![this]);
        assert_eq!(
            recorded(&mut e),
            vec![
                (EFFECT_MANAGER, vec![]),
                (NODE_ITEM, vec![this + 0x10]),
                (
                    MANAGER_DOF_WITHOUT_TARGET,
                    vec![0x4141, bits(0.6), 2, bits(0.4), bits(100.0), bits(20.0)]
                ),
            ]
        );
        let target = plain_instance(&mut e);
        e.mem.set_u32(this + 0x10, target);
        e.mem.set_f32(this + 0x14, -20000.0);
        record(&mut e);
        e.call(0x0052a060, &args![this]);
        assert_eq!(
            recorded(&mut e),
            vec![
                (EFFECT_MANAGER, vec![]),
                (NODE_ITEM, vec![this + 0x10]),
                (NODE_ITEM, vec![this + 0x10]),
                (TARGET_PREPARE, vec![target]),
                (NODE_ITEM, vec![this + 0x10]),
                (TARGET_OR_DEFAULT, vec![target]),
                (
                    MANAGER_DOF_WITH_TARGET,
                    vec![0x4141, bits(0.6), 2, bits(0.4), 0x7777]
                ),
            ]
        );
        assert_eq!(e.mem.f32(this + 0x14), 7.5);
        e.mem.set_f32(this + 0xc, f32::NAN);
        record(&mut e);
        e.call(0x0052a060, &args![this]);
        assert!(recorded(&mut e).is_empty());
    }

    #[test]
    fn dof_trigger_starts_a_depth_of_field_instance() {
        let mut e = world();
        let instance = e
            .call(
                0x0052a150,
                &args![100.0f32, 20.0f32, 2u32, 0.9f32, 0.4f32, 6.0f32],
            )
            .u32();
        assert_eq!(list_items(&e), vec![instance]);
        assert_eq!(e.mem.u32(instance), DOF_VTABLE);
        assert_eq!(e.mem.u32(instance + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(instance + 0x14), bits(-10000.0));
        assert_eq!(e.mem.u32(instance + 0x10), 0);
        assert_eq!(e.mem.u32(instance + 0x1c), bits(6.0));
        assert_eq!(e.mem.u32(instance + 0x20), bits(100.0));
        assert_eq!(e.mem.u32(instance + 0x24), bits(20.0));
        assert_eq!(e.mem.u32(instance + 0x28), bits(0.4));
        assert_eq!(e.mem.u32(instance + 0x2c), 2);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let none = e.call(
            0x0052a150,
            &args![0.0f32, 0.0f32, 0u32, 0.0f32, 0.0f32, 0.0f32],
        );
        assert_eq!(none.u32(), 0);
    }

    #[test]
    fn dof_print_info_writes_distance_range_and_strength() {
        let mut e = world();
        let this = effect_instance(&mut e, 0x30);
        e.mem.set_f32(this + 0x20, 100.0);
        e.mem.set_f32(this + 0x24, 20.0);
        record(&mut e);
        e.call(0x0052a240, &args![this, 0x9000u32]);
        let mut expected = vec![0x9000, 0x800, DOF_FORMAT];
        expected.extend(double_words(100.0));
        expected.extend(double_words(20.0));
        expected.extend(double_words(0.6f32 as f64));
        assert_eq!(recorded(&mut e), vec![(FORMAT_STRING, expected)]);
    }

    #[test]
    fn rb_constructor_copies_the_center_words() {
        let mut e = world();
        let this = e.mem.alloc(0x38);
        let center = e.mem.alloc(8);
        e.mem.set_f32(center, 0.25);
        e.mem.set_f32(center + 4, 0.75);
        let result = e.call(
            0x0052a290,
            &args![this, 0.9f32, -3.0f32, 0u32, 6.0f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32, center],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), RB_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(this + 0x14), bits(-3.0));
        assert_eq!(e.mem.u32(this + 0x1c), bits(6.0));
        for (offset, value) in [(0x20, 1.0f32), (0x24, 2.0), (0x28, 3.0), (0x2c, 4.0)] {
            assert_eq!(e.mem.u32(this + offset), bits(value), "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x30), bits(0.25));
        assert_eq!(e.mem.u32(this + 0x34), bits(0.75));
    }

    #[test]
    fn rb_apply_hands_the_settings_to_the_manager() {
        let mut e = world();
        let this = effect_instance(&mut e, 0x38);
        for (offset, value) in [(0x20, 1.0f32), (0x24, 2.0), (0x28, 3.0), (0x2c, 4.0)] {
            e.mem.set_f32(this + offset, value);
        }
        record(&mut e);
        e.call(0x0052a330, &args![this]);
        assert_eq!(
            recorded(&mut e)[2],
            (
                MANAGER_RB_WITHOUT_TARGET,
                vec![
                    0x4141,
                    bits(0.6),
                    bits(1.0),
                    bits(2.0),
                    bits(3.0),
                    bits(4.0),
                    this + 0x30
                ]
            )
        );
        let target = plain_instance(&mut e);
        e.mem.set_u32(this + 0x10, target);
        record(&mut e);
        e.call(0x0052a330, &args![this]);
        let log = recorded(&mut e);
        assert_eq!(calls_to(&log, TARGET_PREPARE), vec![vec![target]]);
        assert_eq!(
            log.last().unwrap(),
            &(
                MANAGER_RB_WITH_TARGET,
                vec![
                    0x4141,
                    bits(0.6),
                    bits(1.0),
                    bits(2.0),
                    bits(3.0),
                    bits(4.0),
                    0x7777
                ]
            )
        );
        e.mem.set_f32(this + 0xc, 0.0);
        record(&mut e);
        e.call(0x0052a330, &args![this]);
        assert!(recorded(&mut e).is_empty());
    }

    #[test]
    fn rb_trigger_starts_a_radial_blur_instance() {
        let mut e = world();
        let center = e.mem.alloc(8);
        e.mem.set_f32(center, 0.25);
        e.mem.set_f32(center + 4, 0.75);
        let instance = e
            .call(
                0x0052a440,
                &args![0.9f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32, center, 6.0f32],
            )
            .u32();
        assert_eq!(list_items(&e), vec![instance]);
        assert_eq!(e.mem.u32(instance), RB_VTABLE);
        assert_eq!(e.mem.u32(instance + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(instance + 0x14), bits(-10000.0));
        assert_eq!(e.mem.u32(instance + 0x10), 0);
        assert_eq!(e.mem.u32(instance + 0x1c), bits(6.0));
        for (offset, value) in [(0x20, 1.0f32), (0x24, 2.0), (0x28, 3.0), (0x2c, 4.0)] {
            assert_eq!(e.mem.u32(instance + offset), bits(value), "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(instance + 0x30), bits(0.25));
        assert_eq!(e.mem.u32(instance + 0x34), bits(0.75));
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let none = e.call(
            0x0052a440,
            &args![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, center, 0.0f32],
        );
        assert_eq!(none.u32(), 0);
    }

    #[test]
    fn rb_print_info_writes_the_ramps_and_the_strength() {
        let mut e = world();
        let this = effect_instance(&mut e, 0x38);
        for (offset, value) in [(0x20, 1.0f32), (0x24, 2.0), (0x28, 3.0), (0x2c, 4.0)] {
            e.mem.set_f32(this + offset, value);
        }
        record(&mut e);
        e.call(0x0052a530, &args![this, 0x9000u32]);
        let mut expected = vec![0x9000, 0x800, RB_FORMAT];
        for value in [1.0, 2.0, 3.0, 4.0, 0.6f32 as f64] {
            expected.extend(double_words(value));
        }
        assert_eq!(recorded(&mut e), vec![(FORMAT_STRING, expected)]);
    }

    #[test]
    fn drb_constructor_stores_all_settings() {
        let mut e = world();
        let this = e.mem.alloc(0x44);
        let center = e.mem.alloc(8);
        e.mem.set_f32(center, 0.25);
        e.mem.set_f32(center + 4, 0.75);
        let result = e.call(
            0x0052a590,
            &args![
                this, 0.9f32, -3.0f32, 0u32, 6.0f32, 3u32, 50.0f32, 8.0f32, 1.0f32, 2.0f32, 3.0f32,
                4.0f32, center
            ],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), DRB_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(this + 0x14), bits(-3.0));
        assert_eq!(e.mem.u32(this + 0x1c), bits(6.0));
        for (offset, value) in [
            (0x20, 1.0f32),
            (0x24, 2.0),
            (0x28, 3.0),
            (0x2c, 4.0),
            (0x38, 50.0),
            (0x3c, 8.0),
        ] {
            assert_eq!(e.mem.u32(this + offset), bits(value), "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x30), bits(0.25));
        assert_eq!(e.mem.u32(this + 0x34), bits(0.75));
        assert_eq!(e.mem.u32(this + 0x40), 3);
    }

    #[test]
    fn drb_apply_hands_the_settings_to_the_manager() {
        let mut e = world();
        let this = effect_instance(&mut e, 0x44);
        for (offset, value) in [(0x20, 1.0f32), (0x24, 2.0), (0x38, 50.0), (0x3c, 8.0)] {
            e.mem.set_f32(this + offset, value);
        }
        e.mem.set_u32(this + 0x40, 3);
        record(&mut e);
        e.call(0x0052a670, &args![this]);
        assert_eq!(
            recorded(&mut e)[2],
            (
                MANAGER_DRB_WITHOUT_TARGET,
                vec![
                    0x4141,
                    bits(0.6),
                    3,
                    bits(50.0),
                    bits(8.0),
                    bits(1.0),
                    bits(2.0),
                    this + 0x30
                ]
            )
        );
        let target = plain_instance(&mut e);
        e.mem.set_u32(this + 0x10, target);
        record(&mut e);
        e.call(0x0052a670, &args![this]);
        let log = recorded(&mut e);
        assert_eq!(calls_to(&log, TARGET_PREPARE), vec![vec![target]]);
        assert_eq!(
            log.last().unwrap(),
            &(
                MANAGER_DRB_WITH_TARGET,
                vec![0x4141, bits(0.6), 3, bits(1.0), bits(2.0), 0x7777]
            )
        );
        e.mem.set_f32(this + 0xc, -0.5);
        record(&mut e);
        e.call(0x0052a670, &args![this]);
        assert!(recorded(&mut e).is_empty());
    }

    #[test]
    fn drb_trigger_starts_a_depth_radial_blur_instance() {
        let mut e = world();
        let center = e.mem.alloc(8);
        e.mem.set_f32(center, 0.25);
        e.mem.set_f32(center + 4, 0.75);
        let instance = e
            .call(
                0x0052a780,
                &args![
                    50.0f32, 8.0f32, 3u32, 0.9f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32, center, 6.0f32
                ],
            )
            .u32();
        assert_eq!(list_items(&e), vec![instance]);
        assert_eq!(e.mem.u32(instance), DRB_VTABLE);
        assert_eq!(e.mem.u32(instance + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(instance + 0x14), bits(-10000.0));
        assert_eq!(e.mem.u32(instance + 0x10), 0);
        assert_eq!(e.mem.u32(instance + 0x1c), bits(6.0));
        for (offset, value) in [
            (0x20, 1.0f32),
            (0x24, 2.0),
            (0x28, 3.0),
            (0x2c, 4.0),
            (0x38, 50.0),
            (0x3c, 8.0),
        ] {
            assert_eq!(e.mem.u32(instance + offset), bits(value), "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(instance + 0x40), 3);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let none = e.call(
            0x0052a780,
            &args![0.0f32, 0.0f32, 0u32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, center, 0.0f32],
        );
        assert_eq!(none.u32(), 0);
    }

    #[test]
    fn drb_print_info_writes_the_fixed_text() {
        let mut e = world();
        let this = e.mem.alloc(0x44);
        record(&mut e);
        e.call(0x0052a880, &args![this, 0x9000u32]);
        assert_eq!(
            recorded(&mut e),
            vec![(FORMAT_STRING, vec![0x9000, 0x800, DRB_FORMAT])]
        );
    }

    #[test]
    fn temp_constructor_stores_the_duration() {
        let mut e = world();
        let this = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x10);
        record(&mut e);
        let result = e.call(0x0052a8b0, &args![this, -3.0f32, 0.9f32, target, 6.0f32]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            recorded(&mut e),
            vec![
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_POINTER_CONSTRUCT, vec![this + 0x10, target]),
            ]
        );
        assert_eq!(e.mem.u32(this), TEMP_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), bits(0.9));
        assert_eq!(e.mem.u32(this + 0x14), bits(-3.0));
        assert_eq!(e.mem.u32(this + 0x1c), bits(6.0));
    }

    #[test]
    fn temp_is_expired_compares_the_elapsed_time_with_the_duration() {
        let mut e = world();
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0x14, 1.0);
        // Duration zero: never expires.
        assert!(!e.call(0x0052a920, &args![this]).bool());
        // Elapsed 6.5 against durations 6.0 (expired), 6.5 and 7.0 (not).
        e.mem.set_f32(this + 0x1c, 6.0);
        assert!(e.call(0x0052a920, &args![this]).bool());
        e.mem.set_f32(this + 0x1c, 6.5);
        assert!(!e.call(0x0052a920, &args![this]).bool());
        e.mem.set_f32(this + 0x1c, 7.0);
        assert!(!e.call(0x0052a920, &args![this]).bool());
        // A negative duration is not zero and is below the elapsed time.
        e.mem.set_f32(this + 0x1c, -1.0);
        assert!(e.call(0x0052a920, &args![this]).bool());
        // NaN is "not zero" but compares false.
        e.mem.set_f32(this + 0x1c, f32::NAN);
        assert!(!e.call(0x0052a920, &args![this]).bool());
    }
}
