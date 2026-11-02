//! `(unplaced)/run_00bc3080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ba8a80`: the jump target of `00bc3770`.
const JUMP_TARGET_00BC3770: u32 = 0x00ba_8a80;

// Translated from 00bc3770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00ba8a80`, passing on `this` and its one argument; returns the target result. Return type (bool): its jump chain ends in `00a5c180` (`NiObjectNET::RegisterStreamables`, engine map, `bool` in the Xbox PDB).
pub fn fn_00bc3770(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00BC3770, &args![this, stream]).bool()
}

// The three float interpolator controllers of this unit (each 0x38 bytes,
// an `NiFloatInterpController` base; class names from the `NiRTTI` records
// their static initializers `00fbaa30`, `00fbaa60`, `00fbaa90` build):
// `BSMaterialEmittanceMultController` (vtable `010ba858`, `NiRTTI` at
// `01201f90`), `BSRefractionFirePeriodController` (`010ba948`, `01201f9c`)
// and `BSRefractionStrengthController` (`010baa38`, `01201fa8`). They share
// the deleting destructor `00bc3350` and `LoadBinary` `00bc3540`.

/// The three vtables.
const EMITTANCE_MULT_VTABLE: u32 = 0x010b_a858;
const FIRE_PERIOD_VTABLE: u32 = 0x010b_a948;
const REFRACTION_STRENGTH_VTABLE: u32 = 0x010b_aa38;
/// The three `NiRTTI` records.
const EMITTANCE_MULT_RTTI: u32 = 0x0120_1f90;
const FIRE_PERIOD_RTTI: u32 = 0x0120_1f9c;
const REFRACTION_STRENGTH_RTTI: u32 = 0x0120_1fa8;
/// The controllers' size.
const CONTROLLER_SIZE: u32 = 0x38;
/// `NiTimeController::m_uFlags` (+8), `m_fScaledTime` (+0x28), `m_pkTarget`
/// (+0x2C) and `NiSingleInterpController::m_spInterpolator` (+0x34), Xbox
/// PDB names, the same offsets on PC.
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
/// The float stored as the scaled time while flag bit 5 is set (`01096324`).
const DISABLED_TIME: u32 = 0x0109_6324;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// Allocation (`00aa13e0`, `cdecl`, size) and the sized free (`00aa1460`,
/// `cdecl`, pointer and size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The base constructor (`00a57640`, `thiscall`), the base member copy
/// (`00a57620`, `thiscall`: clone, cloning process), the base `LoadBinary`
/// (`00a57630`, `thiscall`: stream) and the destructor body (`00a360d0`:
/// releases the interpolator, then the `NiTimeController` destructor).
const BASE_CONSTRUCTOR: u32 = 0x00a5_7640;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
const BASE_LOAD_BINARY: u32 = 0x00a5_7630;
const DESTRUCTOR_BODY: u32 = 0x00a3_60d0;
/// `00a63430` (`thiscall` on the stream, no stack word; the next linked
/// object, by its use here) and `00a3c340` (`thiscall`, one word: a 0x18-byte
/// interpolator built on that object).
const STREAM_NEXT_LINKED_OBJECT: u32 = 0x00a6_3430;
const INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_c340;
/// `InterlockedDecrement` / `InterlockedIncrement` (import slots).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `_ftol2` (`00ec62c0`): the value in ST0 as a leading `f64`.
const FTOL: u32 = 0x00ec_62c0;
/// `0.5` and `-0.5` (floats): the rounding offsets of the fire period.
const HALF: u32 = 0x0101_6248;
const MINUS_HALF: u32 = 0x0102_295c;
/// The refraction strength setter on the target (`00b66700`, `thiscall`,
/// one float).
const SET_REFRACTION_STRENGTH: u32 = 0x00b6_6700;

/// The update every controller of this unit shares (`thiscall`, one word:
/// the update data, whose first float is the time). With flag bit 5 set the
/// scaled time becomes the float at `01096324`; otherwise, when
/// `DontDoUpdate(time)` answers true, it goes on only with an interpolator
/// whose vtable slot +0xD4 answers true. Then the interpolator's float update
/// (slot +0x9C: scaled time, target, the address of the argument's own stack
/// slot, which receives the value) runs; `apply` gets the slot's bits when it
/// succeeds.
fn update_with(
    e: &mut Engine,
    this: u32,
    update_data: u32,
    apply: impl FnOnce(&mut Engine, u32, u32),
) {
    if (e.mem.u8(this + FLAGS) >> 5) & 1 != 0 {
        let time = x87_quiet_bits(e.mem.u32(DISABLED_TIME));
        e.mem.set_u32(this + SCALED_TIME, time);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update_data));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(this + INTERPOLATOR);
            if interpolator == 0 || !e.vcall(interpolator, 0xd4, &[]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, update_data);
        let time = x87_quiet_bits(e.mem.u32(this + SCALED_TIME));
        let target = e.mem.u32(this + TARGET);
        if e.vcall(interpolator, 0x9c, &args![time, target, slot])
            .bool()
        {
            apply(e, this, slot);
        }
    });
}

/// The shared `CreateClone` body: a new controller (0x38 bytes; the base
/// constructor and `vtable`, or null when the allocation fails), then the
/// base members copied from `this`.
fn clone_with(e: &mut Engine, this: u32, cloning: u32, vtable: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![CONTROLLER_SIZE]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, vtable);
        block
    };
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    clone
}

/// The shared `CreateObject` body: a new controller (base constructor and
/// `vtable`), or null when the allocation fails.
fn create_with(e: &mut Engine, vtable: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![CONTROLLER_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, vtable);
    block
}

// Translated from 00bc3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::Update` (vtable slot +0x94; name
/// from its `NiRTTI` record): the shared update; the value goes to the
/// target's float at +0x40 (through the x87 stack) and the target's counter
/// at +0x44 goes up by one (fields of the target property unnamed here).
pub fn fn_00bc3080(e: &mut Engine, this: Ptr, update_data: u32) {
    update_with(e, this.addr(), update_data, |e, this, slot| {
        let target = e.mem.u32(this + TARGET);
        let value = x87_quiet_bits(e.mem.u32(slot));
        let count = e.mem.u32(target + 0x44);
        e.mem.set_u32(target + 0x44, count.wrapping_add(1));
        e.mem.set_u32(target + 0x40, value);
    });
}

// Translated from 00bc3100 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::GetRTTI` (vtable slot +8; Ghidra has
/// no function here): the class's `NiRTTI` record.
pub fn fn_00bc3100(_e: &mut Engine) -> u32 {
    EMITTANCE_MULT_RTTI
}

// Translation incomplete: 00bc3110 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::CreateClone` (vtable slot +0x48): a
/// new controller (0x38 bytes, `00aa13e0`, the base constructor `00a57640`
/// and this class's vtable; null when the allocation fails) with the base
/// members copied (`00a57620`).
pub fn fn_00bc3110(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    clone_with(e, this.addr(), cloning, EMITTANCE_MULT_VTABLE)
}

// Translation incomplete: 00bc3190 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::CreateObject` (by its body): a new
/// controller with this class's vtable, or null when the allocation fails.
pub fn fn_00bc3190(e: &mut Engine) -> u32 {
    create_with(e, EMITTANCE_MULT_VTABLE)
}

// Translated from 00bc3290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::Update` (vtable slot +0x94): the
/// shared update; the value is rounded half away from zero (`+0.5` when
/// above 0, else `-0.5`, stored as a float, then truncated by `_ftol2`) into
/// the target's integer at +0xE4.
pub fn fn_00bc3290(e: &mut Engine, this: Ptr, update_data: u32) {
    update_with(e, this.addr(), update_data, |e, this, slot| {
        let value = x87_load32(e.mem.u32(slot));
        // FLDZ; FLD value; FCOM: C0 or C3 (or unordered) picks -0.5.
        let offset = if value > 0.0 { HALF } else { MINUS_HALF };
        let offset = x87_quiet_bits(e.mem.u32(offset));
        e.mem.set_u32(slot, offset);
        let sum = x87_add(value, x87_m32(e.mem.u32(slot)));
        e.mem.set_u32(slot, x87_store32(sum));
        let rounded = x87_load32(e.mem.u32(slot));
        let whole = e.call(FTOL, &args![rounded]).u32();
        let target = e.mem.u32(this + TARGET);
        e.mem.set_u32(target + 0xe4, whole);
    });
}

// Translated from 00bc3340 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::GetRTTI` (vtable slot +8; Ghidra has
/// no function here): the class's `NiRTTI` record.
pub fn fn_00bc3340(_e: &mut Engine) -> u32 {
    FIRE_PERIOD_RTTI
}

// Translated from 00bc3350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor the three controllers share (vtable slot
/// 0): the destructor body (`00a360d0`), then with bit 0 of `flags` the
/// 0x38 bytes freed (`00aa1460`). Returns `this`.
pub fn fn_00bc3350(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, CONTROLLER_SIZE]);
    }
    this
}

// Translation incomplete: 00bc3370 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::CreateClone` (vtable slot +0x48), as
/// `00bc3110` with this class's vtable.
pub fn fn_00bc3370(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    clone_with(e, this.addr(), cloning, FIRE_PERIOD_VTABLE)
}

// Translation incomplete: 00bc33f0 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc33f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::CreateObject` (by its body), as
/// `00bc3190` with this class's vtable.
pub fn fn_00bc33f0(e: &mut Engine) -> u32 {
    create_with(e, FIRE_PERIOD_VTABLE)
}

// Translated from 00bc34c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::Update` (vtable slot +0x94): the shared
/// update; the value goes to the target through `00b66700` (`thiscall`, one
/// float).
pub fn fn_00bc34c0(e: &mut Engine, this: Ptr, update_data: u32) {
    update_with(e, this.addr(), update_data, |e, this, slot| {
        let value = x87_quiet_bits(e.mem.u32(slot));
        let target = e.mem.u32(this + TARGET);
        e.call(SET_REFRACTION_STRENGTH, &args![target, value]);
    });
}

// Translation incomplete: 00bc3540 - the FS:[0] exception frame (unwinding when the interpolator constructor throws) is not translated
// Translated from 00bc3540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoadBinary` the three controllers share (vtable slot +0x50): the
/// base `LoadBinary` (`00a57630`); for files older than `0x0A010068` (the
/// stream's version at +0xD8) a 0x18-byte interpolator is built
/// (`00a3c340`) on the stream's next linked object (`00a63430`), or null
/// when the allocation fails, and becomes `m_spInterpolator` (the old one
/// released, `DeleteThis` at a zero count, the new one's count raised),
/// whose vtable slot +0xBC then runs (no arguments; through a null
/// interpolator as the exe does).
pub fn fn_00bc3540(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream + 0xd8) >= 0x0a01_0068 {
        return;
    }
    let data = e.call(STREAM_NEXT_LINKED_OBJECT, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(INTERPOLATOR_CONSTRUCTOR, &args![block, data]).u32()
    };
    let old = e.mem.u32(this + INTERPOLATOR);
    if old != interpolator {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 0x4, &[]);
        }
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator + 4]);
        }
    }
    let current = e.mem.u32(this + INTERPOLATOR);
    e.vcall(current, 0xbc, &[]);
}

// Translated from 00bc3610 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::GetRTTI` (vtable slot +8; Ghidra has no
/// function here): the class's `NiRTTI` record.
pub fn fn_00bc3610(_e: &mut Engine) -> u32 {
    REFRACTION_STRENGTH_RTTI
}

// Translation incomplete: 00bc3620 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::CreateClone` (vtable slot +0x48), as
/// `00bc3110` with this class's vtable.
pub fn fn_00bc3620(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    clone_with(e, this.addr(), cloning, REFRACTION_STRENGTH_VTABLE)
}

// Translation incomplete: 00bc36a0 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc36a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::CreateObject` (by its body), as
/// `00bc3190` with this class's vtable.
pub fn fn_00bc36a0(e: &mut Engine) -> u32 {
    create_with(e, REFRACTION_STRENGTH_VTABLE)
}

/// `NiFloatInterpController::GetViewerStrings` (`00a576c0`, engine map; `thiscall`, one
/// word), `NiGetViewerString` (`00a7d330`, `cdecl`, one word) and the `NiTArray` growth
/// `0096ad30` and element store `0096ae90` (`thiscall`, one and two words).
const CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a5_76c0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// The `NiRTTI` record named `BSShaderPPLightingProperty` (`011fa010`, built by
/// `00fb9960`): the type the fire-period and refraction-strength targets must derive from.
const PP_LIGHTING_PROPERTY_RTTI: u32 = 0x011f_a010;

/// The string append the viewer-string slots inline (`NiTArray::Add`): with
/// `m_usSize` (+0xa) at least `m_usMaxSize` (+8) the array grows to `m_usSize +
/// m_usGrowBy` (+0xe), then `0096ae90` stores the string, passed by the address of a
/// stack word, at the old size. Answers that call's `EAX`.
fn append_viewer_string(e: &mut Engine, strings: u32, text: u32) -> u32 {
    let size = u32::from(e.mem.u16(strings + 0xa));
    let max = u32::from(e.mem.u16(strings + 0x8));
    if size >= max {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(ARRAY_SET_SIZE_0096AD30, &args![strings, grow + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, slot])
            .u32()
    })
}

/// The controllers' shared `GetViewerStrings` shape: the base's, then the string of the
/// class's `NiRTTI` name (the word at `rtti`) appended.
fn controller_viewer_strings(e: &mut Engine, this: u32, strings: u32, rtti: u32) -> u32 {
    e.call(CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(rtti);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    append_viewer_string(e, strings, text)
}

// Translated from 00bc31f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::GetViewerStrings` (vtable slot +0x60, the Xbox
/// PDB's name for that slot; `thiscall`, one word: an `NiTArray` of strings): the base's
/// (`00a576c0`), then the string of the class's `NiRTTI` name appended (`NiTArray::Add`
/// inlined). Ends in the element store, whose `EAX` is answered.
pub fn fn_00bc31f0(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    controller_viewer_strings(e, this.addr(), strings, EMITTANCE_MULT_RTTI)
}

// Translated from 00bc3240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `BSRefractionFirePeriodController` vtable slot +0xec (`GetTargetFloatValue` in the
/// Xbox PDB's slot order; `thiscall`, one word: where the float goes): the target's
/// integer at +0xe4 loaded (`FILD`) and stored as a float.
pub fn fn_00bc3240(e: &mut Engine, this: Ptr, value: u32) {
    let target = e.mem.u32(this.addr() + TARGET);
    let whole = e.mem.i32(target + 0xe4);
    e.mem.set_u32(value, x87_store32(f64::from(whole)));
}

// Translated from 00bc3260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0xe8 of the fire-period and refraction-strength vtables (`InterpTargetIsCorrectType`
/// in the Xbox PDB's slot order; `thiscall`, one word: the target, `this` not read).
/// False without a target; otherwise walks the target's `NiRTTI` (its vtable slot +8)
/// and the parents (+4) and answers whether the record `011fa010`
/// (`BSShaderPPLightingProperty`) is among them.
pub fn fn_00bc3260(e: &mut Engine, _this: Ptr, target: u32) -> bool {
    if target == 0 {
        return false;
    }
    let mut rtti = e.vcall(target, 0x8, &[]).u32();
    while rtti != 0 {
        if rtti == PP_LIGHTING_PROPERTY_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00bc3450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::GetViewerStrings` (vtable slot +0x60), as
/// `fn_00bc31f0` with this class's `NiRTTI` (`01201f9c`).
pub fn fn_00bc3450(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    controller_viewer_strings(e, this.addr(), strings, FIRE_PERIOD_RTTI)
}

// Translated from 00bc34a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `BSRefractionStrengthController` vtable slot +0xec (`GetTargetFloatValue` in the Xbox
/// PDB's slot order; `thiscall`, one word: where the float goes): the target's float at
/// +0xe0, through the FPU (a signalling NaN quieted).
pub fn fn_00bc34a0(e: &mut Engine, this: Ptr, value: u32) {
    let target = e.mem.u32(this.addr() + TARGET);
    let bits = x87_quiet_bits(e.mem.u32(target + 0xe0));
    e.mem.set_u32(value, bits);
}

// Translated from 00bc3700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::GetViewerStrings` (vtable slot +0x60), as
/// `fn_00bc31f0` with this class's `NiRTTI` (`01201fa8`).
pub fn fn_00bc3700(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    controller_viewer_strings(e, this.addr(), strings, REFRACTION_STRENGTH_RTTI)
}

// `VolumetricFogShaderProperty` (vtable `010bab28`, `NiRTTI` at `01201fb0` built by
// `00fbaab0` with the name `010a92d4`; 0x70 bytes, a `BSShaderProperty` base with an
// `NiColorA` at +0x60) and `HairShaderProperty` (vtable `010babf8`, `NiRTTI` at
// `01201fb8` built by `00fbaad0`; 0x180 bytes, a `BSShaderPPLightingProperty` base).

/// The two vtables and `NiRTTI` records, and the objects' sizes.
const FOG_VTABLE: u32 = 0x010b_ab28;
const FOG_RTTI: u32 = 0x0120_1fb0;
const FOG_SIZE: u32 = 0x70;
const HAIR_VTABLE: u32 = 0x010b_abf8;
const HAIR_RTTI: u32 = 0x0120_1fb8;
const HAIR_SIZE: u32 = 0x180;
/// The colour the fog constructor copies (three words at `011a9be0`, then the float at
/// `01016410` as the fourth).
const FOG_COLOR_011A9BE0: u32 = 0x011a_9be0;
const FOG_ALPHA_01016410: u32 = 0x0101_6410;
/// The point the hair destructor and constructor copy (three words at `011f426c`).
const POINT_011F426C: u32 = 0x011f_426c;
/// The fog colour's label for the viewer strings (`010babec`).
const FOG_COLOR_LABEL: u32 = 0x010b_abec;
/// `BSShaderProperty` code (`bsshaderproperty.cpp`, engine map): the constructor
/// `00ba8e50`, `LoadBinary`-shaped `00ba92c0` and `SaveBinary`-shaped `00ba9380`
/// (`thiscall`, one word), the member copy `00ba8b10` (`thiscall`, two words),
/// `BSShaderProperty::GetViewerStrings` `00ba9890`; the destructor `00baa0e0`
/// (`thiscall`); the base comparison `00c9a420` (`thiscall`, one word).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
const SHADER_PROPERTY_LOAD_00BA92C0: u32 = 0x00ba_92c0;
const SHADER_PROPERTY_SAVE_00BA9380: u32 = 0x00ba_9380;
const SHADER_PROPERTY_COPY_00BA8B10: u32 = 0x00ba_8b10;
const SHADER_PROPERTY_VIEWER_STRINGS: u32 = 0x00ba_9890;
const SHADER_PROPERTY_DESTRUCT_00BAA0E0: u32 = 0x00ba_a0e0;
const BASE_IS_EQUAL_00C9A420: u32 = 0x00c9_a420;
/// `NiColorA::LoadBinary` (`00a69800`), `SaveBinary` (`00a698a0`), the comparison the
/// engine map names `NiColorA::operator_` (`004a4c90`) and `GetViewerString`
/// (`00a69720`), all `thiscall` with one word.
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
const COLOR_COMPARE_004A4C90: u32 = 0x004a_4c90;
const COLOR_GET_VIEWER_STRING: u32 = 0x00a6_9720;
/// `BSShaderPPLightingProperty` code (engine map): the constructor `00b66f50`, the
/// destructor `00b67380` and the member copy `00b665d0` (`thiscall`, two words).
const PP_LIGHTING_CONSTRUCT: u32 = 0x00b6_6f50;
const PP_LIGHTING_DESTRUCT: u32 = 0x00b6_7380;
const PP_LIGHTING_COPY_MEMBERS: u32 = 0x00b6_65d0;

/// An `NiPointer` member's release: with an object whose count
/// (`InterlockedDecrement` on +4) drops to zero, its slot +4 (`DeleteThis`) runs.
fn release_pointer(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 0x4, &[]);
    }
}

/// The hair property's inlined reset of its three points (+0x10c, +0x118, +0x124:
/// the three words at `011f426c` each) and the floats after them (+0x130..+0x138: 0.0).
fn hair_reset_points(e: &mut Engine, this: u32) {
    let zero = 0.0f32.to_bits();
    for (point, scalar) in [(0x10cu32, 0x130u32), (0x118, 0x134), (0x124, 0x138)] {
        for word in 0..3 {
            let value = e.mem.u32(POINT_011F426C + 4 * word);
            e.mem.set_u32(this + point + 4 * word, value);
        }
        e.mem.set_u32(this + scalar, zero);
    }
}

// Translated from 00bc3750 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `VolumetricFogShaderProperty` vtable slot +0x4c (`LoadBinary` in the Xbox PDB's slot
/// order; `thiscall`, one word: the stream): the base's `00ba92c0`, then
/// `NiColorA::LoadBinary` on the colour at +0x60. Ends in that call; its `EAX` is
/// answered.
pub fn fn_00bc3750(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(SHADER_PROPERTY_LOAD_00BA92C0, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x60, stream])
        .u32()
}

// Translated from 00bc3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `VolumetricFogShaderProperty` vtable slot +0x58 (`SaveBinary` in the Xbox PDB's slot
/// order; `thiscall`, one word: the stream): the base's `00ba9380`, then
/// `NiColorA::SaveBinary` on the colour at +0x60; its `EAX` is answered.
pub fn fn_00bc3780(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(SHADER_PROPERTY_SAVE_00BA9380, &args![this, stream]);
    e.call(COLOR_SAVE_BINARY, &args![this.addr() + 0x60, stream])
        .u32()
}

// Translated from 00bc37a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `VolumetricFogShaderProperty` vtable slot +8 (`GetRTTI`): its `NiRTTI` record.
pub fn fn_00bc37a0(_e: &mut Engine) -> u32 {
    FOG_RTTI
}

// Translated from 00bc37b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `VolumetricFogShaderProperty` constructor (`thiscall`,
/// no words). Runs `BSShaderProperty::BSShaderProperty`, stores the vtable, zeroes the
/// colour at +0x60..+0x6c, then sets it to the three words at `011a9be0` and the float
/// at `01016410` (through the FPU, quieted), and 0x16 at +0x1c (`iShaderPropertyType`
/// in the Xbox PDB). Answers `this`.
pub fn fn_00bc37b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, FOG_VTABLE);
    let zero = 0.0f32.to_bits();
    for offset in [0x60u32, 0x64, 0x68, 0x6c] {
        e.mem.set_u32(base + offset, zero);
    }
    let alpha = x87_quiet_bits(e.mem.u32(FOG_ALPHA_01016410));
    for word in 0..3 {
        let value = e.mem.u32(FOG_COLOR_011A9BE0 + 4 * word);
        e.mem.set_u32(base + 0x60 + 4 * word, value);
    }
    e.mem.set_u32(base + 0x6c, alpha);
    e.mem.set_u32(base + 0x1c, 0x16);
    this
}

// Translated from 00bc3800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `VolumetricFogShaderProperty` vtable slot 0, the deleting
/// destructor (`thiscall`, one word: flags): the vtable stored, the destructor `00baa0e0`,
/// and with bit 0 the 0x70 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00bc3800(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), FOG_VTABLE);
    e.call(SHADER_PROPERTY_DESTRUCT_00BAA0E0, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, FOG_SIZE]);
    }
    this
}

// Translated from 00bc3830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `VolumetricFogShaderProperty` vtable slot +0x48
/// (`CreateClone` in the Xbox PDB's slot order; `thiscall`, one word: the cloning
/// process). A new 0x70-byte property (`00aa13e0`, constructed by `fn_00bc37b0`; null
/// when the allocation fails), the base member copy `00ba8b10` from `this`, then the
/// four colour words at +0x60 copied into it (through a null clone as the exe does).
/// Answers the clone.
// Translation incomplete: 00bc3830 - the C++ exception frame (FS:[0] chain, unwind state 0 freeing the block if the constructor throws) is not translated.
pub fn fn_00bc3830(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![FOG_SIZE]).u32();
    let clone = if block != 0 {
        fn_00bc37b0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(SHADER_PROPERTY_COPY_00BA8B10, &args![this, clone, process]);
    for word in 0..4 {
        let value = e.mem.u32(this.addr() + 0x60 + 4 * word);
        e.mem.set_u32(clone.wrapping_add(0x60 + 4 * word), value);
    }
    clone
}

// Translated from 00bc38c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `VolumetricFogShaderProperty`'s `CreateObject` (by its
/// body; no words): a new 0x70-byte property constructed by `fn_00bc37b0`, or null when
/// the allocation fails.
// Translation incomplete: 00bc38c0 - the C++ exception frame (FS:[0] chain, unwind state 0 freeing the block if the constructor throws) is not translated.
pub fn fn_00bc38c0(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![FOG_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    fn_00bc37b0(e, Ptr::new(block)).addr()
}

// Translated from 00bc3930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `VolumetricFogShaderProperty` vtable slot +0x5c (`IsEqual`
/// in the Xbox PDB's slot order; `thiscall`, one word: the other object). When
/// `004a4c90` on the other's colour (+0x60) with this colour answers true, false;
/// otherwise the answer of the base comparison `00c9a420` with the other object.
pub fn fn_00bc3930(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let differs = e
        .call(
            COLOR_COMPARE_004A4C90,
            &args![other.wrapping_add(0x60), this.addr() + 0x60],
        )
        .bool();
    if differs {
        return false;
    }
    e.call(BASE_IS_EQUAL_00C9A420, &args![this, other]).bool()
}

// Translated from 00bc3960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `VolumetricFogShaderProperty` vtable slot +0x60 (`GetViewerStrings`; `thiscall`, one
/// word: an `NiTArray` of strings): `BSShaderProperty::GetViewerStrings`, then the string
/// of the class's `NiRTTI` name and the colour's string (`NiColorA::GetViewerString`
/// with the label `FogProps` at `010babec`) appended in turn. Ends in the second
/// element store, whose `EAX` is answered.
pub fn fn_00bc3960(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    e.call(SHADER_PROPERTY_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(FOG_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    append_viewer_string(e, strings, text);
    let text = e
        .call(
            COLOR_GET_VIEWER_STRING,
            &args![this.addr() + 0x60, FOG_COLOR_LABEL],
        )
        .u32();
    append_viewer_string(e, strings, text)
}

// Translated from 00bc39f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `HairShaderProperty` destructor body (`thiscall`, no
/// words; called by the deleting destructor `fn_00bc3d80`). Stores the vtable, releases
/// and clears the pointers at +0x17c, +0x104 and +0x108, resets the three points
/// (`011f426c`) and their floats, then the members' own destructors release +0x17c,
/// +0x108 and +0x104 again (read again; null after the clears) and the
/// `BSShaderPPLightingProperty` destructor `00b67380` runs.
// Translation incomplete: 00bc39f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the members and the base) is not translated.
pub fn fn_00bc39f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, HAIR_VTABLE);
    for offset in [0x17cu32, 0x104, 0x108] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release_pointer(e, object);
            e.mem.set_u32(base + offset, 0);
        }
    }
    hair_reset_points(e, base);
    for offset in [0x17cu32, 0x108, 0x104] {
        let object = e.mem.u32(base + offset);
        release_pointer(e, object);
    }
    e.call(PP_LIGHTING_DESTRUCT, &args![this]);
}

// Translated from 00bc3ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `HairShaderProperty` vtable slot +8 (`GetRTTI`): its `NiRTTI` record.
pub fn fn_00bc3ba0(_e: &mut Engine) -> u32 {
    HAIR_RTTI
}

// Translated from 00bc3bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `HairShaderProperty` constructor (`thiscall`, no
/// words; called by `CreateObject` `00bc4380` and `CreateClone` `00bc43b0`). Runs the
/// `BSShaderPPLightingProperty` constructor, stores the vtable, clears the pointers at
/// +0x104 and +0x108, zeroes the sixteen floats +0x13c..+0x178 and the pointer at +0x17c,
/// then the inlined pointer resets: +0x17c is known null to the compiler (its release
/// is folded away), +0x104 and +0x108 are read again and released and cleared when set;
/// the three points are reset (`011f426c`, floats 0.0) and 9 stored at +0x1c
/// (`iShaderPropertyType` in the Xbox PDB). Answers `this`.
// Translation incomplete: 00bc3bb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the members) is not translated.
pub fn fn_00bc3bb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(PP_LIGHTING_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, HAIR_VTABLE);
    e.mem.set_u32(base + 0x104, 0);
    e.mem.set_u32(base + 0x108, 0);
    let zero = 0.0f32.to_bits();
    for index in 0..16 {
        e.mem.set_u32(base + 0x13c + 4 * index, zero);
    }
    e.mem.set_u32(base + 0x17c, 0);
    for offset in [0x104u32, 0x108] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release_pointer(e, object);
            e.mem.set_u32(base + offset, 0);
        }
    }
    hair_reset_points(e, base);
    e.mem.set_u32(base + 0x1c, 9);
    this
}

// Translated from 00bc3d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `HairShaderProperty` vtable slot 0, the deleting
/// destructor (`thiscall`, one word: flags): the destructor body `fn_00bc39f0`, then with
/// bit 0 the 0x180 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00bc3d80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bc39f0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, HAIR_SIZE]);
    }
    this
}

// Translated from 00bc3db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `HairShaderProperty` member copy (`thiscall`, two
/// words: the clone, the cloning process; called by `CreateClone` `00bc43b0`). The
/// `BSShaderPPLightingProperty` copy `00b665d0`, then the clone's pointer at +0x17c set
/// to this one's (the old one released, the new one's count raised, when they differ)
/// and the four words at +0x16c copied.
pub fn fn_00bc3db0(e: &mut Engine, this: Ptr, clone: u32, process: u32) {
    let base = this.addr();
    e.call(PP_LIGHTING_COPY_MEMBERS, &args![this, clone, process]);
    let old = e.mem.u32(clone + 0x17c);
    if old != e.mem.u32(base + 0x17c) {
        release_pointer(e, old);
        let new = e.mem.u32(base + 0x17c);
        e.mem.set_u32(clone + 0x17c, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
        }
    }
    for word in 0..4 {
        let value = e.mem.u32(base + 0x16c + 4 * word);
        e.mem.set_u32(clone + 0x16c + 4 * word, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_3770, fn_00bc3770(Ptr, Ptr) -> bool),
        entry!(0x00bc3080, fn_00bc3080(Ptr, u32)),
        entry!(0x00bc3100, fn_00bc3100() -> u32),
        entry!(0x00bc3110, fn_00bc3110(Ptr, u32) -> u32),
        entry!(0x00bc3190, fn_00bc3190() -> u32),
        entry!(0x00bc3290, fn_00bc3290(Ptr, u32)),
        entry!(0x00bc3340, fn_00bc3340() -> u32),
        entry!(0x00bc3350, fn_00bc3350(Ptr, u32) -> Ptr),
        entry!(0x00bc3370, fn_00bc3370(Ptr, u32) -> u32),
        entry!(0x00bc33f0, fn_00bc33f0() -> u32),
        entry!(0x00bc34c0, fn_00bc34c0(Ptr, u32)),
        entry!(0x00bc3540, fn_00bc3540(Ptr, u32)),
        entry!(0x00bc3610, fn_00bc3610() -> u32),
        entry!(0x00bc3620, fn_00bc3620(Ptr, u32) -> u32),
        entry!(0x00bc36a0, fn_00bc36a0() -> u32),
        entry!(0x00bc_31f0, fn_00bc31f0(Ptr, u32) -> u32),
        entry!(0x00bc_3240, fn_00bc3240(Ptr, u32)),
        entry!(0x00bc_3260, fn_00bc3260(Ptr, u32) -> bool),
        entry!(0x00bc_3450, fn_00bc3450(Ptr, u32) -> u32),
        entry!(0x00bc_34a0, fn_00bc34a0(Ptr, u32)),
        entry!(0x00bc_3700, fn_00bc3700(Ptr, u32) -> u32),
        entry!(0x00bc_3750, fn_00bc3750(Ptr, u32) -> u32),
        entry!(0x00bc_3780, fn_00bc3780(Ptr, u32) -> u32),
        entry!(0x00bc_37a0, fn_00bc37a0() -> u32),
        entry!(0x00bc_37b0, fn_00bc37b0(Ptr) -> Ptr),
        entry!(0x00bc_3800, fn_00bc3800(Ptr, u32) -> Ptr),
        entry!(0x00bc_3830, fn_00bc3830(Ptr, u32) -> u32),
        entry!(0x00bc_38c0, fn_00bc38c0() -> u32),
        entry!(0x00bc_3930, fn_00bc3930(Ptr, u32) -> bool),
        entry!(0x00bc_3960, fn_00bc3960(Ptr, u32) -> u32),
        entry!(0x00bc_39f0, fn_00bc39f0(Ptr)),
        entry!(0x00bc_3ba0, fn_00bc3ba0() -> u32),
        entry!(0x00bc_3bb0, fn_00bc3bb0(Ptr) -> Ptr),
        entry!(0x00bc_3d80, fn_00bc3d80(Ptr, u32) -> Ptr),
        entry!(0x00bc_3db0, fn_00bc3db0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00bc3770_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00bc_3770;
        const TARGET: u32 = 0x00ba_8a80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A controller with an interpolator whose float update (slot +0x9C)
    /// writes `value` into its out slot and answers true, and a target.
    fn controller(e: &mut Engine, value: f32) -> (u32, u32) {
        e.map(0x0109_6000, 0x1000);
        e.set_global(DISABLED_TIME, -2.0f32);
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        e.register_double(0x0015_0000, move |e, a| {
            e.mem.set_f32(a[3], value);
            1u32.into_ret()
        });
        e.register(0x0015_0010, |_, _| 0u32.into_ret());
        let vtable = e.mem.alloc(0xd8);
        e.mem.set_u32(vtable + 0x9c, 0x0015_0000);
        e.mem.set_u32(vtable + 0xd4, 0x0015_0010);
        let interpolator = e.mem.alloc(8);
        e.mem.set_u32(interpolator, vtable);
        let this = e.mem.alloc(CONTROLLER_SIZE);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_f32(this + SCALED_TIME, 1.5);
        (this, target)
    }

    #[test]
    fn emittance_update_stores_the_value_and_counts() {
        let mut e = Engine::new();
        let (this, target) = controller(&mut e, 0.25);
        let data = e.mem.alloc(4);
        e.mem.set_f32(data, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3080, &args![this, data]);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this, 4.0f32.to_bits()]]
        );
        let update = calls_to(&e, 0x0015_0000);
        assert_eq!(
            &update[0][..3],
            [e.mem.u32(this + INTERPOLATOR), 1.5f32.to_bits(), target]
        );
        assert_eq!(e.mem.f32(target + 0x40), 0.25);
        assert_eq!(e.mem.u32(target + 0x44), 1);
        // Flag bit 5: the disabled time is the scaled time.
        e.mem.set_u16(this + FLAGS, 0x20);
        e.call(0x00bc_3080, &args![this, data]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), -2.0);
        assert_eq!(e.mem.u32(target + 0x44), 2);
        // DontDoUpdate true and an inactive interpolator: nothing.
        e.mem.set_u16(this + FLAGS, 0);
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        e.call(0x00bc_3080, &args![this, data]);
        assert_eq!(e.mem.u32(target + 0x44), 2);
    }

    #[test]
    fn fire_period_update_rounds_half_away_from_zero() {
        for (value, expected) in [(2.5f32, 3i32), (2.4, 2), (-2.5, -3), (0.0, 0)] {
            let mut e = Engine::new();
            e.map(0x0101_6000, 0x1000);
            e.set_global(HALF, 0.5f32);
            e.map(0x0102_2000, 0x1000);
            e.set_global(MINUS_HALF, -0.5f32);
            e.register(FTOL, |_, a| {
                let value = f64::take(a, &mut 0);
                (value as i32).into_ret()
            });
            let (this, target) = controller(&mut e, value);
            let data = e.mem.alloc(4);
            e.call(0x00bc_3290, &args![this, data]);
            assert_eq!(e.mem.u32(target + 0xe4) as i32, expected, "{value}");
        }
    }

    #[test]
    fn strength_update_calls_the_setter() {
        let mut e = Engine::new();
        let (this, target) = controller(&mut e, 0.75);
        e.register(SET_REFRACTION_STRENGTH, |_, _| Ret::default());
        let data = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00bc_34c0, &args![this, data]);
        assert_eq!(
            calls_to(&e, SET_REFRACTION_STRENGTH),
            vec![vec![target, 0.75f32.to_bits()]]
        );
    }

    #[test]
    fn rtti_create_and_clone_use_each_class_vtable() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bc_3100, &[]).u32(), EMITTANCE_MULT_RTTI);
        assert_eq!(e.call(0x00bc_3340, &[]).u32(), FIRE_PERIOD_RTTI);
        assert_eq!(e.call(0x00bc_3610, &[]).u32(), REFRACTION_STRENGTH_RTTI);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for (create, clone, vtable) in [
            (0x00bc_3190u32, 0x00bc_3110u32, EMITTANCE_MULT_VTABLE),
            (0x00bc_33f0, 0x00bc_3370, FIRE_PERIOD_VTABLE),
            (0x00bc_36a0, 0x00bc_3620, REFRACTION_STRENGTH_VTABLE),
        ] {
            let object = e.call(create, &[]).u32();
            assert_eq!(e.mem.u32(object), vtable);
            let copy = e.call(clone, &args![object, 0x77u32]).u32();
            assert_eq!(e.mem.u32(copy), vtable);
            assert!(calls_to(&e, BASE_COPY_MEMBERS).contains(&vec![object, copy, 0x77]));
        }
        // A failed allocation: null, and the clone copies into null.
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00bc_3190, &[]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_3110, &args![0x1234u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, BASE_COPY_MEMBERS), vec![vec![0x1234, 0, 0]]);
    }

    #[test]
    fn destructor_frees_on_flag() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_3350, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(
            calls_to(&e, NI_FREE_SIZED),
            vec![vec![0x5000, CONTROLLER_SIZE]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00bc_3350, &args![0x5000u32, 0u32]);
        assert_eq!(calls_to(&e, DESTRUCTOR_BODY).len(), 1);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
    }

    #[test]
    fn load_binary_builds_an_interpolator_for_old_files() {
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_NEXT_LINKED_OBJECT, |_, _| 0x6000u32.into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERPOLATOR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0016_0000);
            a[0].into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0016_1000, |_, _| Ret::default());
        e.register(0x0016_1004, |_, _| Ret::default());
        e.put_vtable(0x0016_0000, &[0, 0x0016_1004]);
        e.mem.map(0x0016_00bc, 4);
        e.mem.set_u32(0x0016_00bc, 0x0016_1000);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x0016_0000);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(CONTROLLER_SIZE);
        e.mem.set_u32(this + INTERPOLATOR, old);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0067);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3540, &args![this, stream]);
        let made = e.mem.u32(this + INTERPOLATOR);
        assert_ne!(made, old);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(
            calls_to(&e, INTERPOLATOR_CONSTRUCTOR),
            vec![vec![made, 0x6000]]
        );
        assert_eq!(calls_to(&e, 0x0016_1004), vec![vec![old]]);
        assert_eq!(calls_to(&e, 0x0016_1000), vec![vec![made]]);
        // A current file: only the base LoadBinary.
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3540, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY).len(), 1);
        assert!(calls_to(&e, STREAM_NEXT_LINKED_OBJECT).is_empty());
    }

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles for the string append; the store answers the stored word.
    fn viewer_doubles(e: &mut Engine) -> u32 {
        e.map(0x0120_1000, 0x1000);
        e.register(NI_GET_VIEWER_STRING, |_, a| answer(a[0] + 1));
        e.register(ARRAY_SET_SIZE_0096AD30, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_0096AE90, |e, a| answer(e.mem.u32(a[2])));
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0x8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 5);
        strings
    }

    #[test]
    fn controller_viewer_strings_append_each_class_name() {
        let mut e = Engine::new();
        let strings = viewer_doubles(&mut e);
        e.register(CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        for (address, rtti) in [
            (0x00bc_31f0u32, EMITTANCE_MULT_RTTI),
            (0x00bc_3450, FIRE_PERIOD_RTTI),
            (0x00bc_3700, REFRACTION_STRENGTH_RTTI),
        ] {
            e.mem.set_u32(rtti, rtti + 0x100);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(address, &args![0x9000u32, strings]).u32(),
                rtti + 0x101
            );
            assert_eq!(
                calls_to(&e, CONTROLLER_GET_VIEWER_STRINGS),
                vec![vec![0x9000, strings]]
            );
            assert_eq!(
                calls_to(&e, ARRAY_SET_SIZE_0096AD30),
                vec![vec![strings, 7]]
            );
            assert_eq!(calls_to(&e, ARRAY_SET_AT_0096AE90)[0][..2], [strings, 2]);
        }
        e.mem.set_u16(strings + 0x8, 3);
        e.call_log = Some(vec![]);
        e.call(0x00bc_31f0, &args![0x9000u32, strings]);
        assert!(calls_to(&e, ARRAY_SET_SIZE_0096AD30).is_empty());
    }

    #[test]
    fn target_float_values_read_the_integer_and_the_float() {
        let mut e = Engine::new();
        let this = e.mem.alloc(CONTROLLER_SIZE);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_i32(target + 0xe4, -7);
        e.mem.set_u32(target + 0xe0, 0x7f80_0002);
        let out = e.mem.alloc(4);
        e.call(0x00bc_3240, &args![this, out]);
        assert_eq!(e.mem.f32(out), -7.0);
        e.call(0x00bc_34a0, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0002);
    }

    #[test]
    fn target_type_walks_the_rtti_parents() {
        let mut e = Engine::new();
        e.map(0x011f_a000, 0x1000);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent + 4, PP_LIGHTING_PROPERTY_RTTI);
        let own = e.mem.alloc(8);
        e.mem.set_u32(own + 4, parent);
        e.register_double(0x0017_0000, move |e, a| answer(e.mem.u32(a[0] + 4)));
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, 0x0017_0000);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, vtable);
        e.mem.set_u32(target + 4, own);
        assert!(e.call(0x00bc_3260, &args![0u32, target]).bool());
        e.mem.set_u32(parent + 4, 0);
        assert!(!e.call(0x00bc_3260, &args![0u32, target]).bool());
        assert!(!e.call(0x00bc_3260, &args![0u32, 0u32]).bool());
    }

    #[test]
    fn fog_property_streams_its_colour_after_the_base() {
        let mut e = Engine::new();
        for address in [SHADER_PROPERTY_LOAD_00BA92C0, SHADER_PROPERTY_SAVE_00BA9380] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(COLOR_LOAD_BINARY, |_, _| answer(0x31));
        e.register(COLOR_SAVE_BINARY, |_, _| answer(0x32));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00bc_3750, &args![0x9000u32, 0x5000u32]).u32(),
            0x31
        );
        assert_eq!(
            e.call(0x00bc_3780, &args![0x9000u32, 0x5000u32]).u32(),
            0x32
        );
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_LOAD_00BA92C0),
            vec![vec![0x9000, 0x5000]]
        );
        assert_eq!(calls_to(&e, COLOR_LOAD_BINARY), vec![vec![0x9060, 0x5000]]);
        assert_eq!(calls_to(&e, COLOR_SAVE_BINARY), vec![vec![0x9060, 0x5000]]);
        assert_eq!(e.call(0x00bc_37a0, &[]).u32(), FOG_RTTI);
        assert_eq!(e.call(0x00bc_3ba0, &[]).u32(), HAIR_RTTI);
    }

    fn fog_setup(e: &mut Engine) {
        e.map(0x011a_9000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        for word in 0..3 {
            e.set_global(FOG_COLOR_011A9BE0 + 4 * word, 0x100 + word);
        }
        e.set_global(FOG_ALPHA_01016410, 0x7f80_0003u32);
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn fog_constructor_copies_the_default_colour() {
        let mut e = Engine::new();
        fog_setup(&mut e);
        let this = e.mem.alloc(FOG_SIZE);
        assert_eq!(e.call(0x00bc_37b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FOG_VTABLE);
        assert_eq!(
            [0x60u32, 0x64, 0x68, 0x6c].map(|o| e.mem.u32(this + o)),
            [0x100, 0x101, 0x102, 0x7fc0_0003]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0x16);
    }

    #[test]
    fn fog_create_and_clone_build_a_new_property() {
        let mut e = Engine::new();
        fog_setup(&mut e);
        let block = e.mem.alloc(FOG_SIZE);
        e.register_double(NI_ALLOCATE, move |_, a| {
            assert_eq!(a, &[FOG_SIZE]);
            answer(block)
        });
        e.register(SHADER_PROPERTY_COPY_00BA8B10, |_, _| Ret::default());
        assert_eq!(e.call(0x00bc_38c0, &[]).u32(), block);
        assert_eq!(e.mem.u32(block), FOG_VTABLE);
        let this = e.mem.alloc(FOG_SIZE);
        for word in 0..4 {
            e.mem.set_u32(this + 0x60 + 4 * word, 0x200 + word);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_3830, &args![this, 0x8000u32]).u32(), block);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_COPY_00BA8B10),
            vec![vec![this, block, 0x8000]]
        );
        assert_eq!(e.mem.u32(block + 0x6c), 0x203);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00bc_38c0, &[]).u32(), 0);
    }

    #[test]
    fn fog_destructor_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_DESTRUCT_00BAA0E0, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(FOG_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3800, &args![this, 0u32]);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00bc_3800, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this, FOG_SIZE]]);
        assert_eq!(e.mem.u32(this), FOG_VTABLE);
    }

    #[test]
    fn fog_equality_needs_equal_colours_then_the_base() {
        let mut e = Engine::new();
        e.register(COLOR_COMPARE_004A4C90, |_, a| {
            answer(u32::from(a[0] == 0x7060))
        });
        e.register(BASE_IS_EQUAL_00C9A420, |_, _| answer(1));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00bc_3930, &args![0x9000u32, 0x7000u32]).bool());
        assert!(calls_to(&e, BASE_IS_EQUAL_00C9A420).is_empty());
        assert_eq!(
            calls_to(&e, COLOR_COMPARE_004A4C90),
            vec![vec![0x7060, 0x9060]]
        );
        assert!(e.call(0x00bc_3930, &args![0x9000u32, 0x7100u32]).bool());
        assert_eq!(
            calls_to(&e, BASE_IS_EQUAL_00C9A420),
            vec![vec![0x9000, 0x7100]]
        );
    }

    #[test]
    fn fog_viewer_strings_append_the_name_then_the_colour() {
        let mut e = Engine::new();
        let strings = viewer_doubles(&mut e);
        e.mem.set_u16(strings + 0x8, 9);
        e.mem.set_u32(FOG_RTTI, 0x4400);
        e.register(SHADER_PROPERTY_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(COLOR_GET_VIEWER_STRING, |_, _| answer(0x6600));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00bc_3960, &args![0x9000u32, strings]).u32(),
            0x6600
        );
        let stored: Vec<_> = calls_to(&e, ARRAY_SET_AT_0096AE90)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(stored, vec![2, 2]);
        assert_eq!(
            calls_to(&e, COLOR_GET_VIEWER_STRING),
            vec![vec![0x9060, FOG_COLOR_LABEL]]
        );
    }

    /// A reference-counted object whose `DeleteThis` (slot +4) is a double.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0018_0004);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn hair_doubles(e: &mut Engine) {
        e.map(0x011f_4000, 0x1000);
        for word in 0..3 {
            e.set_global(POINT_011F426C + 4 * word, 0x300 + word);
        }
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            answer(count)
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            answer(count)
        });
        e.register(0x0018_0004, |_, _| Ret::default());
        for address in [
            PP_LIGHTING_CONSTRUCT,
            PP_LIGHTING_DESTRUCT,
            PP_LIGHTING_COPY_MEMBERS,
            NI_FREE_SIZED,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn hair_destructor_releases_its_pointers_and_resets_its_points() {
        let mut e = Engine::new();
        hair_doubles(&mut e);
        let this = e.mem.alloc(HAIR_SIZE);
        let last = counted(&mut e, 1);
        let kept = counted(&mut e, 2);
        e.mem.set_u32(this + 0x17c, last);
        e.mem.set_u32(this + 0x104, kept);
        e.mem.set_f32(this + 0x134, 4.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_3d80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0018_0004), vec![vec![last]]);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(e.mem.u32(this), HAIR_VTABLE);
        assert_eq!(
            [0x17cu32, 0x104, 0x108].map(|o| e.mem.u32(this + o)),
            [0; 3]
        );
        assert_eq!(e.mem.u32(this + 0x118 + 8), 0x302);
        assert_eq!(e.mem.f32(this + 0x134), 0.0);
        assert_eq!(calls_to(&e, PP_LIGHTING_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this, HAIR_SIZE]]);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3d80, &args![this, 0u32]);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
    }

    #[test]
    fn hair_constructor_clears_its_members() {
        let mut e = Engine::new();
        hair_doubles(&mut e);
        let this = e.mem.alloc(HAIR_SIZE);
        e.mem.set_f32(this + 0x178, 2.0);
        e.mem.set_u32(this + 0x17c, 0x1234);
        assert_eq!(e.call(0x00bc_3bb0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), HAIR_VTABLE);
        assert_eq!(e.mem.f32(this + 0x178), 0.0);
        assert_eq!(e.mem.u32(this + 0x17c), 0);
        assert_eq!(e.mem.u32(this + 0x124), 0x300);
        assert_eq!(e.mem.u32(this + 0x1c), 9);
    }

    #[test]
    fn hair_member_copy_shares_the_pointer_and_copies_four_words() {
        let mut e = Engine::new();
        hair_doubles(&mut e);
        let this = e.mem.alloc(HAIR_SIZE);
        let clone = e.mem.alloc(HAIR_SIZE);
        let shared = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.mem.set_u32(this + 0x17c, shared);
        e.mem.set_u32(clone + 0x17c, old);
        for word in 0..4 {
            e.mem.set_u32(this + 0x16c + 4 * word, 0x400 + word);
        }
        e.call_log = Some(vec![]);
        e.call(0x00bc_3db0, &args![this, clone, 0x8000u32]);
        assert_eq!(
            calls_to(&e, PP_LIGHTING_COPY_MEMBERS),
            vec![vec![this, clone, 0x8000]]
        );
        assert_eq!(e.mem.u32(clone + 0x17c), shared);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(calls_to(&e, 0x0018_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(clone + 0x178), 0x403);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3db0, &args![this, clone, 0x8000u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }
}
