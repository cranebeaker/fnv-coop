//! `NiParticle/nipsysemitterctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiPSysEmitterCtlr` drives a particle emitter from two interpolators: a
//! float interpolator for the birth rate (index 0, `m_spInterpolator`) and a
//! bool interpolator for whether the emitter is active (index 1,
//! `m_spEmitterActiveInterpolator`). Its update walks the active keys
//! between the last and the current time and emits, for each active
//! stretch, the particles the birth rate asks for, with their ages. The
//! unit also holds `NiTimeController::DontDoUpdate`, `NiBoolData`'s deleting
//! destructor and the particle-age set's `Add`, which the compiler emitted
//! here.
//!
//! The code is optimized x87: the translations follow its instructions one
//! by one (`crate::x87`), with `float` stores where it stores floats.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nimainsdm::{
    assign_ref_counted, interlocked_decrement, interlocked_increment,
};
use crate::units::nimain::nitimecontroller::NiTimeController;
use crate::x87::fistp_chop;

/// `NiBoolData::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_DATA_RTTI: u32 = 0x011f_4054;

/// `NiPSysEmitterCtlr::ms_RTTI` (the class's `NiRTTI` record).
const NI_P_SYS_EMITTER_CTLR_RTTI: u32 = 0x0120_24e8;

// Translated from 00a537e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_DATA_RTTI)
}

// Translated from 00c1c2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_p_sys_emitter_ctlr_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_P_SYS_EMITTER_CTLR_RTTI)
}

layout! {
    /// `NiPSysEmitterCtlr` (Xbox PDB), 0x5C bytes on PC as on the Xbox: an
    /// `NiPSysModifierCtlr` (an `NiTimeController`, [`NiTimeController`],
    /// with an interpolator, the modifier name and the modifier) and the
    /// emitter's state. Float fields are their bit patterns.
    pub struct NiPSysEmitterCtlr: 0x5c {
        /// `m_spInterpolator` (Xbox PDB, `NiSingleInterpController`): the
        /// birth-rate interpolator.
        0x34 m_spInterpolator: u32,
        /// `m_kModifierName` (Xbox PDB, `NiPSysModifierCtlr`).
        0x38 m_kModifierName: u32,
        /// `m_pkModifier` (Xbox PDB, `NiPSysModifierCtlr`): the emitter.
        0x3C m_pkModifier: u32,
        /// `m_spEmitterActiveInterpolator` (Xbox PDB).
        0x40 m_spEmitterActiveInterpolator: u32,
        /// `m_pkLastBirthRateInterpolator` (Xbox PDB).
        0x44 m_pkLastBirthRateInterpolator: u32,
        /// `m_fLastScaledTime` (Xbox PDB).
        0x48 m_fLastScaledTime: u32,
        /// `m_bLastActive` (Xbox PDB).
        0x4C m_bLastActive: u8,
        /// `m_kParticleAges` (Xbox PDB): `NiTPrimitiveSet<float>`, buffer
        /// at +0x50, capacity at +0x54, size at +0x58.
        0x50 m_kParticleAges: u32,
        0x54 m_kParticleAgesMax: u32,
        0x58 m_kParticleAgesSize: u32,
    }
}

/// `NiTimeController::ComputeScaledTime` (slot `+0xA4`, Xbox PDB).
const COMPUTE_SCALED_TIME_SLOT: u32 = 0xa4;
/// `NiInterpController::SetInterpolator` (slot `+0xC8`, Xbox PDB).
const SET_INTERPOLATOR_SLOT: u32 = 0xc8;
/// `NiObject::CreateClone` (`+0x48`) and `ProcessClone` (`+0x68`) (Xbox
/// PDB), and `NiRefObject::DeleteThis` (`+4`).
const CREATE_CLONE_SLOT: u32 = 0x48;
const PROCESS_CLONE_SLOT: u32 = 0x68;
const DELETE_THIS_SLOT: u32 = 4;
/// The interpolator slots the update uses: `+0xA0` (a bool update: time,
/// target, out value; a `bool`), `+0x9C` (a float update: time, target,
/// out value; a `bool`), `+0xF4` (one word; a `bool`: whether the
/// interpolator is posed) and `+0xD8` (no words; a pointer).
const INTERPOLATOR_UPDATE_BOOL_SLOT: u32 = 0xa0;
const INTERPOLATOR_UPDATE_FLOAT_SLOT: u32 = 0x9c;
const INTERPOLATOR_POSED_SLOT: u32 = 0xf4;
const INTERPOLATOR_SLOT_D8: u32 = 0xd8;
/// The emitter's emit slot (`+0xA4`: time, count, ages).
const EMITTER_EMIT_SLOT: u32 = 0xa4;

/// `MemoryManager::Deallocate` (`00aa4060`, thiscall on `011f6238`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `NiObject`'s destructor body (`00a5d3d0`).
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// Gamebryo's object allocation (`00aa13e0`, cdecl: size) and release
/// (`00aa1460`, cdecl: object, size), and `NiFree` (`00aa10f0`, cdecl).
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
const NI_FREE: u32 = 0x00aa_10f0;
/// `NiStricmp` (`00aa1360`, Xbox PDB name, cdecl).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `NiBlendFloatInterpolator` (`00a40520`) and `NiBlendBoolInterpolator`
/// (`00a56560`) constructors (Xbox PDB names; thiscall: manager
/// controlled, weight threshold, array size).
const BLEND_FLOAT_CONSTRUCTOR: u32 = 0x00a4_0520;
const BLEND_BOOL_CONSTRUCTOR: u32 = 0x00a5_6560;
/// The `NiPointer` assignment of the emitter-active slot (`00a5f860`,
/// thiscall on the slot).
const POINTER_ASSIGN: u32 = 0x00a5_f860;
/// `NiPSysModifierCtlr`: its member copy (`00c312f0`; the engine map gives
/// the shared body `NiFloatsExtraDataController::CopyMembers`),
/// `ProcessClone` (`00c31340`), `LoadBinary` (`00a571a0`), `LinkObject`
/// (`00a57630`), its constructor (`00c31260`) and destructor body
/// (`00c312a0`).
const MODIFIER_CTLR_COPY_MEMBERS: u32 = 0x00c3_12f0;
const MODIFIER_CTLR_PROCESS_CLONE: u32 = 0x00c3_1340;
const MODIFIER_CTLR_LOAD_BINARY: u32 = 0x00a5_71a0;
const MODIFIER_CTLR_LINK_OBJECT: u32 = 0x00a5_7630;
const MODIFIER_CTLR_CONSTRUCTOR: u32 = 0x00c3_1260;
const MODIFIER_CTLR_DESTRUCTOR: u32 = 0x00c3_12a0;
/// `NiStream`: the link-id read (`00a645b0`) and `GetObjectFromLinkID`
/// (`00a63430`, Xbox PDB name); the stream's file version is at +0xD8.
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// Streams older than this version keep the keys in an
/// `NiPSysEmitterCtlrData`.
const OLD_STREAM_VERSION: u32 = 0x0a01_0068;
/// The `NiFloatInterpolator` (`00a3c340`) and `NiBoolInterpolator`
/// (`00a53280`) constructors (one word, 0 here), and their `ReplaceKeys`
/// (`0052ce40`, `00c1bf10`, Xbox PDB names; three words).
const FLOAT_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_c340;
const BOOL_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_3280;
const FLOAT_REPLACE_KEYS: u32 = 0x0052_ce40;
const BOOL_REPLACE_KEYS: u32 = 0x00c1_bf10;
/// `rand` (`00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// The set's reallocation (`00c1beb0`, thiscall: the new capacity; the
/// engine map names it `NiTSet<float>::Add`).
const SET_REALLOCATE: u32 = 0x00c1_beb0;
/// The run-time type test (`0043b300`, cdecl: `NiRTTI`, object; a `bool`)
/// and the cast (`00653290`, thiscall on the object: `NiRTTI`).
const IS_KIND_OF: u32 = 0x0043_b300;
const DYNAMIC_CAST: u32 = 0x0065_3290;
/// `NiBlendBoolInterpolator::ms_RTTI` (`011f407c`: its static initializer
/// `00fb5480` names it `NiBlendBoolInterpolator`, base `011f36fc`), and the `NiRTTI` the
/// birth-rate interpolator is tested against (`011f3e58`).
const BLEND_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_407c;
const BIRTH_RATE_BLEND_RTTI: u32 = 0x011f_3e58;
/// `NiBlendInterpolator::ComputeNormalizedWeights` (`00a37260`) and
/// `GetHighestWeightedIndex` (`00a368f0`) (Xbox PDB names).
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
const GET_HIGHEST_WEIGHTED_INDEX: u32 = 0x00a3_68f0;
/// `NiPSysEmitterCtlr::GetBirthRateInterpolator` (`00639b60`, Xbox PDB
/// name; the engine map puts the body in `precipitation.cpp`).
const GET_BIRTH_RATE_INTERPOLATOR: u32 = 0x0063_9b60;
/// The keys of the emitter-active (`00a52d20`) and the birth-rate
/// (`00533000`) interpolators (thiscall: out count, out type, out key
/// size; the keys).
const ACTIVE_GET_KEYS: u32 = 0x00a5_2d20;
const BIRTH_GET_KEYS: u32 = 0x0053_3000;
/// `NiBoolKey::GenInterp` (`00a29250`, Xbox PDB name, cdecl: time, keys,
/// type, count, last index, key size; a `bool`).
const BOOL_KEY_GEN_INTERP: u32 = 0x00a2_9250;

/// The interpolator names `GetInterpolatorIndex` compares with (the words
/// at `011ae754` and `011ae758`), and its "no index" answer (`01096320`).
const BIRTH_RATE_NAME: u32 = 0x011a_e754;
const EMITTER_ACTIVE_NAME: u32 = 0x011a_e758;
const INVALID_INDEX: u32 = 0x0109_6320;
/// `NiBlendInterpolator`'s invalid index (`01096968`, a byte).
const INVALID_BLEND_INDEX: u32 = 0x0109_6968;
/// `-FLT_MAX` as a `float` (`01015f5c`, `01096324`) and as a `double`
/// (`010241b0`).
const NO_TIME: u32 = 0x0101_5f5c;
const NO_TIME_FLAG: u32 = 0x0109_6324;
const NO_TIME_DOUBLE: u32 = 0x0102_41b0;
/// `32767.0` (`01029790`) and `100.0` (`01017a40`) (`double`): the random
/// phase is `rand() / 32767 * 100`.
const RAND_MAX: u32 = 0x0102_9790;
const PHASE_RANGE: u32 = 0x0101_7a40;
/// `0.5` (`double`, `01011588`).
const HALF: u32 = 0x0101_1588;
/// The most particles emitted in one step (`011ae75c`, a `u16`).
const MAX_EMIT: u32 = 0x011a_e75c;
/// `NiBoolData`'s and `NiPSysEmitterCtlr`'s vtables (`0109a1d4`,
/// `010bd594`).
const NI_BOOL_DATA_VTABLE: u32 = 0x0109_a1d4;
const NI_P_SYS_EMITTER_CTLR_VTABLE: u32 = 0x010b_d594;

// Translated from 00a36250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::DontDoUpdate` (Xbox PDB): true (skip) for an inactive
/// controller (flag 8 clear), and for an unchanged time (`time` equal to
/// the last time) unless forced (flag `0x80`). Otherwise, without flag
/// `0x40`, clears the force flag and answers false; with it, computes the
/// scaled time (slot `+0xA4`) and again skips an unchanged one unless
/// forced, else stores it, clears the force flag and answers false.
pub fn ni_time_controller_dont_do_update(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    time: f32,
) -> bool {
    let flags = e.get(this, NiTimeController::m_uFlags) as u8;
    if (flags >> 3) & 1 == 0 {
        return true;
    }
    let last = x87_load32(e.get(this, NiTimeController::m_fLastTime));
    let time = x87_load32(time.to_bits());
    if time == last && (flags >> 7) & 1 == 0 {
        return true;
    }
    if (flags >> 6) & 1 == 0 {
        let word = e.get(this, NiTimeController::m_uFlags);
        e.set(this, NiTimeController::m_uFlags, word & 0xff7f);
        return false;
    }
    let scaled = e
        .vcall(
            this.addr(),
            COMPUTE_SCALED_TIME_SLOT,
            &args![x87_store32(time)],
        )
        .f64();
    let scaled = x87_store32(scaled);
    let previous = x87_load32(e.get(this, NiTimeController::m_fScaledTime));
    if x87_load32(scaled) == previous && (e.mem.u8(this.addr() + 8) >> 7) & 1 == 0 {
        return true;
    }
    e.set(
        this,
        NiTimeController::m_fScaledTime,
        x87_quiet_bits(scaled),
    );
    let word = e.get(this, NiTimeController::m_uFlags);
    e.set(this, NiTimeController::m_uFlags, word & 0xff7f);
    false
}

// Translated from 00a53a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData`'s scalar deleting destructor (Xbox PDB): frees the keys
/// (`m_pkKeys` +0xC, an array with its count 4 bytes before) through the
/// memory manager, runs `NiObject`'s destructor and frees the 0x18-byte
/// object when `flags & 1`. Returns `this`.
pub fn ni_bool_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let keys = e.mem.u32(this.addr() + 0x0c);
    e.mem.set_u32(this.addr(), NI_BOOL_DATA_VTABLE);
    if keys != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, keys.wrapping_sub(4)]);
    }
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00c1ba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetInterpolatorCount` (Xbox PDB slot `+0xB4` of the
/// class's vtable): 2.
pub fn ni_p_sys_emitter_ctlr_get_interpolator_count(
    _e: &mut Engine,
    _this: Ptr<NiPSysEmitterCtlr>,
) -> u32 {
    2
}

// Translated from 00c1ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetInterpolatorIndex` (Xbox PDB): 0 for the name in
/// the word at `011ae754`, 1 for the one at `011ae758` (`NiStricmp`), else
/// the `u16` at `01096320`.
pub fn ni_p_sys_emitter_ctlr_get_interpolator_index(
    e: &mut Engine,
    _this: Ptr<NiPSysEmitterCtlr>,
    name: Ptr,
) -> u16 {
    let first = e.global::<u32>(BIRTH_RATE_NAME);
    if e.call(NI_STRICMP, &args![name, first]).u32() == 0 {
        return 0;
    }
    let second = e.global::<u32>(EMITTER_ACTIVE_NAME);
    if e.call(NI_STRICMP, &args![name, second]).u32() == 0 {
        return 1;
    }
    e.global::<u16>(INVALID_INDEX)
}

// Translated from 00c1bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::CreateBlendInterpolator` (Xbox PDB): a new
/// `NiBlendFloatInterpolator` for index 0 or `NiBlendBoolInterpolator` for
/// index 1 (0x34 bytes) with `manager_controlled`, `weight_threshold` and
/// `array_size`; null for other indices or when the allocation fails. The
/// third word is not read.
pub fn ni_p_sys_emitter_ctlr_create_blend_interpolator(
    e: &mut Engine,
    _this: Ptr<NiPSysEmitterCtlr>,
    index: u16,
    manager_controlled: u32,
    _unused_3: u32,
    weight_threshold: f32,
    array_size: u32,
) -> Ptr {
    let constructor = match index {
        0 => BLEND_FLOAT_CONSTRUCTOR,
        1 => BLEND_BOOL_CONSTRUCTOR,
        _ => return Ptr::NULL,
    };
    let block = e.call(NI_NEW, &args![0x34u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let weight = x87_quiet_bits(weight_threshold.to_bits());
    e.call(
        constructor,
        &args![block, manager_controlled, weight, array_size],
    )
    .ptr()
}

// Translated from 00c1bb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiBlendInterpolator`'s update time of item `index` (no PDB name):
/// `m_fSingleTime` (+0x20) when it has one item (`m_ucInterpCount` +0xE)
/// and `index` is the single index (+0xF), else the item's
/// `m_fUpdateTime` (+0x14 of the 0x18-byte item in `m_pkInterpArray`
/// +0x14).
pub fn fn_00c1bb90(e: &mut Engine, this: Ptr, index: u8) -> f32 {
    let single = e.mem.u8(this.addr() + 0x0e) == 1 && index == e.mem.u8(this.addr() + 0x0f);
    let value = if single {
        e.mem.f32(this.addr() + 0x20)
    } else {
        let items = e.mem.u32(this.addr() + 0x14);
        e.mem.f32(items + u32::from(index) * 0x18 + 0x14)
    };
    x87_quiet(value)
}

// Translated from 00c1bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetInterpolator` (Xbox PDB): the birth-rate
/// interpolator for index 0, the emitter-active one for 1, else null.
pub fn ni_p_sys_emitter_ctlr_get_interpolator(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    index: u16,
) -> Ptr {
    match index {
        0 => Ptr::new(e.get(this, NiPSysEmitterCtlr::m_spInterpolator)),
        1 => Ptr::new(e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator)),
        _ => Ptr::NULL,
    }
}

// Translated from 00c1bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::SetInterpolator` (Xbox PDB): assigns `interpolator`
/// to the birth-rate slot (index 0, the reference-counted assignment
/// inlined) or the emitter-active slot (index 1, `00a5f860`); other
/// indices do nothing.
pub fn ni_p_sys_emitter_ctlr_set_interpolator(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    interpolator: Ptr,
    index: u16,
) {
    match index {
        0 => {
            let slot = this.addr() + NiPSysEmitterCtlr::m_spInterpolator.off;
            assign_ref_counted(e, slot, interpolator.addr());
        }
        1 => {
            e.call(POINTER_ASSIGN, &args![this.byte_add(0x40), interpolator]);
        }
        _ => {}
    }
}

/// The random phase of a clone or a loaded controller: with neither cycle
/// bit (`m_uFlags & 6`) set and a phase of 0, `rand() / 32767 * 100`.
fn randomize_phase(e: &mut Engine, controller: Ptr<NiTimeController>) {
    if e.mem.u8(controller.addr() + 8) & 6 != 0 {
        return;
    }
    let phase = x87_m32(e.get(controller, NiTimeController::m_fPhase));
    if 0.0 == phase {
        let r = e.call(RAND, &args![]).i32();
        let scaled = x87_div(f64::from(r), e.global::<f64>(RAND_MAX));
        let phase = x87_mul(scaled, e.global::<f64>(PHASE_RANGE));
        e.set(controller, NiTimeController::m_fPhase, x87_store32(phase));
    }
}

// Translated from 00c1bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The emitter controller's member copy (no PDB name; `CreateClone` calls
/// it): `NiPSysModifierCtlr`'s copy, then a clone of the emitter-active
/// interpolator (slot `+0x48`) into `clone`'s slot, and a random phase for
/// the clone.
pub fn fn_00c1bd30(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    clone: Ptr<NiPSysEmitterCtlr>,
    cloning: Ptr,
) {
    e.call(MODIFIER_CTLR_COPY_MEMBERS, &args![this, clone, cloning]);
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active != 0 {
        let copy = e.vcall(active, CREATE_CLONE_SLOT, &args![cloning]).u32();
        let slot = clone.addr() + NiPSysEmitterCtlr::m_spEmitterActiveInterpolator.off;
        assign_ref_counted(e, slot, copy);
    }
    randomize_phase(e, clone.cast());
}

// Translated from 00c1bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::ProcessClone` (Xbox PDB): `NiPSysModifierCtlr`'s,
/// then the emitter-active interpolator's (slot `+0x68`).
pub fn ni_p_sys_emitter_ctlr_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    cloning: Ptr,
) {
    e.call(MODIFIER_CTLR_PROCESS_CLONE, &args![this, cloning]);
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active != 0 {
        e.vcall(active, PROCESS_CLONE_SLOT, &args![cloning]);
    }
}

// Translated from 00c1bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::LoadBinary` (Xbox PDB): `NiPSysModifierCtlr`'s, then
/// reads the emitter-active interpolator's link id (`00a645b0`).
pub fn ni_p_sys_emitter_ctlr_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    stream: Ptr,
) {
    e.call(MODIFIER_CTLR_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00c1bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetEmitterActiveInterpolator` (Xbox PDB): the
/// emitter-active interpolator's slot `+0xD8` answer when it is not null;
/// otherwise, for an `NiBlendBoolInterpolator`, the interpolator of its
/// highest-weighted item (the single interpolator +0x18 when that is the
/// single item, else the item's +0); null when there is none.
pub fn ni_p_sys_emitter_ctlr_get_emitter_active_interpolator(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
) -> Ptr {
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    let own = e.vcall(active, INTERPOLATOR_SLOT_D8, &args![]).u32();
    if own != 0 {
        return Ptr::new(own);
    }
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active == 0 {
        return Ptr::NULL;
    }
    let blend = e
        .call(DYNAMIC_CAST, &args![active, BLEND_BOOL_INTERPOLATOR_RTTI])
        .u32();
    if blend == 0 {
        return Ptr::NULL;
    }
    let index = e.call(GET_HIGHEST_WEIGHTED_INDEX, &args![blend]).u8();
    if index == e.global::<u8>(INVALID_BLEND_INDEX) {
        return Ptr::NULL;
    }
    if e.mem.u8(blend + 0x0e) == 1 && index == e.mem.u8(blend + 0x0f) {
        return Ptr::new(e.mem.u32(blend + 0x18));
    }
    let items = e.mem.u32(blend + 0x14);
    Ptr::new(e.mem.u32(items + u32::from(index) * 0x18))
}

// Translated from 00c1c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::LinkObject` (Xbox PDB): `NiPSysModifierCtlr`'s,
/// then, for a stream older than `0x0A010068`, turns the linked
/// `NiPSysEmitterCtlrData` into the two interpolators: a new
/// `NiFloatInterpolator` takes the birth-rate keys (keys +0xC, count +8,
/// type +0x10) and a new `NiBoolInterpolator` the emitter-active keys
/// (+0x1C, type +0x20, count +0x18), the data's fields are cleared and
/// the interpolators set as index 0 and 1 (slot `+0xC8`); the data's
/// reference taken for the duration is dropped. For newer streams it
/// randomizes the phase and links the emitter-active interpolator.
pub fn ni_p_sys_emitter_ctlr_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    stream: Ptr,
) {
    e.call(MODIFIER_CTLR_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) < OLD_STREAM_VERSION {
        let data = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if data == 0 {
            return;
        }
        interlocked_increment(e, data + 4);
        let block = e.call(NI_NEW, &args![0x18u32]).u32();
        let float_interpolator = if block == 0 {
            0
        } else {
            e.call(FLOAT_INTERPOLATOR_CONSTRUCTOR, &args![block, 0u32])
                .u32()
        };
        let (count, keys, kind) = (
            e.mem.u32(data + 0x08),
            e.mem.u32(data + 0x0c),
            e.mem.u32(data + 0x10),
        );
        e.call(
            FLOAT_REPLACE_KEYS,
            &args![float_interpolator, keys, count, kind],
        );
        e.mem.set_u32(data + 0x08, 0);
        e.mem.set_u32(data + 0x0c, 0);
        e.mem.set_u32(data + 0x10, 0);
        e.mem.set_u8(data + 0x14, 0);
        e.vcall(
            this.addr(),
            SET_INTERPOLATOR_SLOT,
            &args![float_interpolator, 0u32],
        );
        let block = e.call(NI_NEW, &args![0x18u32]).u32();
        let bool_interpolator = if block == 0 {
            0
        } else {
            e.call(BOOL_INTERPOLATOR_CONSTRUCTOR, &args![block, 0u32])
                .u32()
        };
        let (count, keys, kind) = (
            e.mem.u32(data + 0x18),
            e.mem.u32(data + 0x1c),
            e.mem.u32(data + 0x20),
        );
        e.call(
            BOOL_REPLACE_KEYS,
            &args![bool_interpolator, keys, kind, count],
        );
        e.mem.set_u32(data + 0x18, 0);
        e.mem.set_u32(data + 0x1c, 0);
        e.mem.set_u8(data + 0x24, 0);
        e.vcall(
            this.addr(),
            SET_INTERPOLATOR_SLOT,
            &args![bool_interpolator, 1u32],
        );
        if interlocked_decrement(e, data + 4) == 0 {
            e.vcall(data, DELETE_THIS_SLOT, &args![]);
        }
        return;
    }
    randomize_phase(e, this.cast());
    let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let slot = this.addr() + NiPSysEmitterCtlr::m_spEmitterActiveInterpolator.off;
    assign_ref_counted(e, slot, linked);
}

/// `NiTPrimitiveSet<float>::Add` as the code inlines it: grows a full set
/// (buffer +0, capacity +4, size +8) to twice its capacity (1 for none)
/// and appends `value` (bits).
fn add_to_set(e: &mut Engine, set: u32, value: u32) {
    let capacity = e.mem.u32(set + 4);
    if e.mem.u32(set + 8) == capacity {
        let wanted = if capacity > 0 {
            capacity.wrapping_add(capacity)
        } else {
            1
        };
        e.call(SET_REALLOCATE, &args![set, wanted]);
    }
    let size = e.mem.u32(set + 8);
    let base = e.mem.u32(set);
    e.mem.set_u32(
        base.wrapping_add(size.wrapping_mul(4)),
        x87_quiet_bits(value),
    );
    e.mem.set_u32(set + 8, size.wrapping_add(1));
}

// Translated from 00c1c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveSet<float>::Add` (no PDB name): appends `*value` to the set
/// (buffer +0, capacity +4, size +8), first growing a full set to twice its
/// capacity, or 1 (`00c1beb0`).
pub fn fn_00c1c240(e: &mut Engine, this: Ptr, value: Ptr) {
    let capacity = e.mem.u32(this.addr() + 4);
    if e.mem.u32(this.addr() + 8) == capacity {
        let wanted = if capacity > 0 {
            capacity.wrapping_add(capacity)
        } else {
            1
        };
        e.call(SET_REALLOCATE, &args![this, wanted]);
    }
    let size = e.mem.u32(this.addr() + 8);
    let base = e.mem.u32(this.addr());
    let word = x87_quiet_bits(e.mem.u32(value.addr()));
    e.mem.set_u32(base.wrapping_add(size.wrapping_mul(4)), word);
    e.mem.set_u32(this.addr() + 8, size.wrapping_add(1));
}

/// The destructor's body shared by `00c1c300` and the deleting destructor:
/// frees the ages, releases the emitter-active interpolator and runs
/// `NiPSysModifierCtlr`'s destructor.
fn destroy(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) {
    let ages = e.get(this, NiPSysEmitterCtlr::m_kParticleAges);
    e.call(NI_FREE, &args![ages]);
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active != 0 && interlocked_decrement(e, active + 4) == 0 {
        e.vcall(active, DELETE_THIS_SLOT, &args![]);
    }
    e.call(MODIFIER_CTLR_DESTRUCTOR, &args![this]);
}

// Translated from 00c1c300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitterCtlr` destructor body (no PDB name): frees the age
/// buffer (`NiFree`), drops the emitter-active interpolator and continues in
/// `NiPSysModifierCtlr`'s destructor (`00c312a0`, a tail jump).
pub fn fn_00c1c300(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) {
    destroy(e, this);
}

// Translated from 00c1c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr`'s scalar deleting destructor (Xbox PDB): the
/// destructor body and, when `flags & 1`, frees the 0x5C bytes. Returns
/// `this`.
pub fn ni_p_sys_emitter_ctlr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    flags: u32,
) -> Ptr<NiPSysEmitterCtlr> {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x5cu32]);
    }
    this
}

/// The constructor's own part: the vtable, no emitter-active
/// interpolator, no last scaled time (`-FLT_MAX`), not active, and an
/// empty age set.
fn construct_members(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) {
    let no_time = x87_quiet_bits(e.global::<u32>(NO_TIME));
    e.mem.set_u32(this.addr(), NI_P_SYS_EMITTER_CTLR_VTABLE);
    e.set(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator, 0);
    e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, no_time);
    e.set(this, NiPSysEmitterCtlr::m_bLastActive, 0);
    e.set(this, NiPSysEmitterCtlr::m_kParticleAges, 0);
    e.set(this, NiPSysEmitterCtlr::m_kParticleAgesMax, 0);
    e.set(this, NiPSysEmitterCtlr::m_kParticleAgesSize, 0);
}

// Translated from 00c1c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitterCtlr` constructor without arguments (no PDB name):
/// `NiPSysModifierCtlr`'s constructor, then the members. Returns `this`.
pub fn fn_00c1c390(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) -> Ptr<NiPSysEmitterCtlr> {
    e.call(MODIFIER_CTLR_CONSTRUCTOR, &args![this]);
    construct_members(e, this);
    this
}

// Translated from 00c1c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::EmitParticles` (Xbox PDB): for a positive `rate`
/// and an emit window `start < stop` that the frame (`last .. current`)
/// overlaps (`current > start`, `last < stop`), counts the particles born
/// between `last` and `min(current, stop)` (both measured from `start`,
/// times `rate`, truncated, as 16-bit counts; at most the `u16` at
/// `011ae75c`), fills the age set with their ages
/// (`(min(current, stop) - start) - k / rate` for each birth `k`), and has
/// the emitter (slot `+0xA4`) emit them at `time`.
pub fn ni_p_sys_emitter_ctlr_emit_particles(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    time: f32,
    current: f32,
    last: f32,
    start: f32,
    stop: f32,
    rate: f32,
) {
    let rate = x87_load32(rate.to_bits());
    if !(rate > 0.0) {
        return;
    }
    let start = x87_load32(start.to_bits());
    let stop = x87_load32(stop.to_bits());
    if !(stop > start) {
        return;
    }
    let current = x87_load32(current.to_bits());
    if !(current > start) {
        return;
    }
    let last = x87_load32(last.to_bits());
    if !(last < stop) {
        return;
    }
    let end = if current > stop { stop } else { current };
    let span = x87_store32(x87_sub(end, start));
    let begin = if start <= last {
        x87_store32(x87_sub(last, start))
    } else {
        0.0f32.to_bits()
    };
    let born_before = fistp_chop(x87_mul(x87_load32(begin), rate)) as u16;
    let born_by_end = fistp_chop(x87_mul(x87_load32(span), rate)) as u16;
    let mut count = born_by_end.wrapping_sub(born_before);
    let most = e.global::<u16>(MAX_EMIT);
    if count > most {
        count = most;
    }
    let step = x87_store32(x87_div(1.0, rate));
    e.set(this, NiPSysEmitterCtlr::m_kParticleAgesSize, 0);
    let set = this.addr() + NiPSysEmitterCtlr::m_kParticleAges.off;
    let mut birth = u32::from(born_before);
    for _ in 0..count {
        birth = birth.wrapping_add(1);
        let k = f64::from(birth & 0xffff);
        let age = x87_store32(x87_sub(x87_load32(span), x87_mul(x87_load32(step), k)));
        add_to_set(e, set, age);
    }
    let emitter = e.get(this, NiPSysEmitterCtlr::m_pkModifier);
    let ages = e.get(this, NiPSysEmitterCtlr::m_kParticleAges);
    e.vcall(
        emitter,
        EMITTER_EMIT_SLOT,
        &args![x87_quiet_bits(time.to_bits()), u32::from(count), ages],
    );
}

// Translated from 00c1c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::CreateClone` (Xbox PDB): a new controller (0x5C
/// bytes, constructed as by [`fn_00c1c390`]) with this one's members copied
/// ([`fn_00c1bd30`]; the copy runs with a null clone when the allocation
/// fails). Returns the clone.
pub fn ni_p_sys_emitter_ctlr_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    cloning: Ptr,
) -> Ptr<NiPSysEmitterCtlr> {
    let block = e.call(NI_NEW, &args![0x5cu32]).u32();
    if block == 0 {
        fn_00c1bd30(e, this, Ptr::NULL, cloning);
        return Ptr::NULL;
    }
    let clone = Ptr::<NiPSysEmitterCtlr>::new(block);
    e.call(MODIFIER_CTLR_CONSTRUCTOR, &args![clone]);
    construct_members(e, clone);
    fn_00c1bd30(e, this, clone, cloning);
    clone
}

/// The update time of the highest-weighted item of `interpolator` when it
/// is of type `rtti` (after normalizing the weights), else `-FLT_MAX`
/// (bits).
fn blend_time(e: &mut Engine, rtti: u32, interpolator: u32) -> u32 {
    let mut time = x87_quiet_bits(e.global::<u32>(NO_TIME));
    if e.call(IS_KIND_OF, &args![rtti, interpolator]).bool() {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![interpolator]);
        let index = e
            .call(GET_HIGHEST_WEIGHTED_INDEX, &args![interpolator])
            .u8();
        if index != e.global::<u8>(INVALID_BLEND_INDEX) {
            let value = fn_00c1bb90(e, Ptr::new(interpolator), index);
            time = x87_store32(f64::from(value));
        }
    }
    time
}

/// The middle of `start .. end` (bits) as the code computes it:
/// `half * (end - start) + start`, stored as a `float`.
fn middle(start: u32, end: u32, half: f64) -> u32 {
    let difference = x87_sub(x87_load32(end), x87_load32(start));
    x87_store32(x87_add(x87_mul(half, difference), x87_load32(start)))
}

/// Calls [`ni_p_sys_emitter_ctlr_emit_particles`] with `float` bit
/// patterns, each copied through the x87 stack.
fn emit(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>, words: [u32; 6]) {
    let [time, current, last, start, stop, rate] =
        words.map(|word| f32::from_bits(x87_quiet_bits(word)));
    ni_p_sys_emitter_ctlr_emit_particles(e, this, time, current, last, start, stop, rate);
}

// Translated from 00c1c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::Update` (Xbox PDB): resets the last scaled time when
/// `*time` went back before the last time; with a target and an emitter,
/// marks the scaled time unknown for flag `0x20`, follows a time that ran
/// ahead by more than the emitter's +0x48 (no cycle bits), and unless
/// `DontDoUpdate` skips, with both interpolators:
///
/// * takes the time `t`: the birth-rate blend's highest-weighted update
///   time when either blend has one, else the scaled time;
/// * for flag `0x20` with a new birth-rate interpolator, only updates the
///   active state (`m_bLastActive`) at `t`;
/// * with a posed emitter-active interpolator (slot `+0xF4`) that is
///   active at time 0, emits from the last scaled time to `t` with the
///   posed birth rate, or, for a keyed birth rate, over the keys (two
///   stretches when the time wrapped), the rate taken in each stretch's
///   middle;
/// * otherwise walks the emitter-active keys (`NiBoolKey::GenInterp`) from
///   the last scaled time to `t`, at most 20 stretches: each active stretch
///   (when the time wrapped, past the last key, with the controller's key
///   range added to the target's time) is emitted with the birth rate in
///   its middle (or at `t`), the target's time (+0xD4) giving the emit
///   time.
///
/// The last scaled time and active state are kept for the next update.
pub fn ni_p_sys_emitter_ctlr_update(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>, time: Ptr) {
    let base = this.cast::<NiTimeController>();
    let read_time = |e: &Engine| e.mem.u32(time.addr());
    let no_time_double = e.global::<f64>(NO_TIME_DOUBLE);
    if x87_load32(read_time(e)) < x87_load32(e.get(base, NiTimeController::m_fLastTime)) {
        let no_time = x87_quiet_bits(e.global::<u32>(NO_TIME));
        e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, no_time);
    }
    if e.get(base, NiTimeController::m_pkTarget) == 0 {
        return;
    }
    let modifier = e.get(this, NiPSysEmitterCtlr::m_pkModifier);
    if modifier == 0 {
        return;
    }
    let flags = e.get(base, NiTimeController::m_uFlags) as u8;
    if (flags >> 5) & 1 != 0 {
        let unknown = x87_quiet_bits(e.global::<u32>(NO_TIME_FLAG));
        e.set(base, NiTimeController::m_fScaledTime, unknown);
    }
    let last = e.get(base, NiTimeController::m_fLastTime);
    if !(x87_load32(last) == no_time_double) && flags & 6 == 0 {
        let ahead = x87_sub(x87_load32(read_time(e)), x87_m32(last));
        if x87_load32(e.mem.u32(modifier + 0x48)) < ahead {
            let now = x87_quiet_bits(read_time(e));
            e.set(base, NiTimeController::m_fLastTime, now);
        }
    }
    let now = f32::from_bits(x87_quiet_bits(read_time(e)));
    if ni_time_controller_dont_do_update(e, base, now) {
        return;
    }
    let active_slot = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active_slot == 0 || e.get(this, NiPSysEmitterCtlr::m_spInterpolator) == 0 {
        return;
    }
    let active_time = blend_time(e, BLEND_BOOL_INTERPOLATOR_RTTI, active_slot);
    let active = ni_p_sys_emitter_ctlr_get_emitter_active_interpolator(e, this).addr();
    let active_known = !(x87_load32(active_time) == no_time_double);
    let birth_slot = e.get(this, NiPSysEmitterCtlr::m_spInterpolator);
    let birth_time = blend_time(e, BIRTH_RATE_BLEND_RTTI, birth_slot);
    let birth = e.call(GET_BIRTH_RATE_INTERPOLATOR, &args![this]).u32();
    let birth_known = !(x87_load32(birth_time) == no_time_double);
    if active == 0 || birth == 0 {
        return;
    }
    let scaled = x87_quiet_bits(e.get(base, NiTimeController::m_fScaledTime));
    let t = if active_known || birth_known {
        x87_quiet_bits(birth_time)
    } else {
        scaled
    };
    if (e.mem.u8(this.addr() + 8) >> 5) & 1 != 0
        && e.get(this, NiPSysEmitterCtlr::m_pkLastBirthRateInterpolator) != birth
    {
        let target = e.get(base, NiTimeController::m_pkTarget);
        e.set(
            this,
            NiPSysEmitterCtlr::m_fLastScaledTime,
            x87_quiet_bits(t),
        );
        let out = this.addr() + NiPSysEmitterCtlr::m_bLastActive.off;
        e.vcall(
            active,
            INTERPOLATOR_UPDATE_BOOL_SLOT,
            &args![x87_quiet_bits(t), target, out],
        );
        e.set(
            this,
            NiPSysEmitterCtlr::m_pkLastBirthRateInterpolator,
            birth,
        );
        return;
    }

    // The function's frame from E-0x48 to E+8 (E: the stack pointer at
    // entry, E+4 the argument's own word); the words it passes by address
    // live at their offsets in it.
    e.with_stack(0x50, |e, frame| {
        let at = |offset: i32| frame.addr().wrapping_add((0x48 + offset) as u32);
        e.set(
            this,
            NiPSysEmitterCtlr::m_pkLastBirthRateInterpolator,
            birth,
        );
        let keys = e
            .call(ACTIVE_GET_KEYS, &args![active, at(4), at(-4), at(-0x30)])
            .u32();
        let count = e.mem.u32(at(4));
        if count == 0
            && !e
                .vcall(active, INTERPOLATOR_POSED_SLOT, &args![count])
                .bool()
        {
            return;
        }
        if e.vcall(active, INTERPOLATOR_POSED_SLOT, &args![0u32])
            .bool()
        {
            update_posed(e, this, active, birth, t, frame);
        } else {
            update_keyed(e, this, birth, t, keys, count, frame);
        }
    });
}

/// `Update` with a posed emitter-active interpolator (`frame` is the
/// function's frame, see [`ni_p_sys_emitter_ctlr_update`]).
fn update_posed(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    active: u32,
    birth: u32,
    t: u32,
    frame: Ptr,
) {
    let at = |offset: i32| frame.addr().wrapping_add((0x48 + offset) as u32);
    let base = this.cast::<NiTimeController>();
    let target = e.get(base, NiTimeController::m_pkTarget);
    e.vcall(
        active,
        INTERPOLATOR_UPDATE_BOOL_SLOT,
        &args![0.0f32, target, at(4)],
    );
    if e.mem.u8(at(4)) == 0 {
        return;
    }
    let mut previous = x87_quiet_bits(e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime));
    let target = e.get(base, NiTimeController::m_pkTarget);
    // The target's time (`+0xD4`).
    let system_time = x87_quiet_bits(e.mem.u32(target + 0xd4));
    e.set(
        this,
        NiPSysEmitterCtlr::m_fLastScaledTime,
        x87_quiet_bits(t),
    );
    if e.vcall(birth, INTERPOLATOR_POSED_SLOT, &args![0u32]).bool() {
        let mut delta = x87_store32(x87_sub(x87_load32(t), x87_load32(previous)));
        if x87_load32(previous) > x87_load32(t) {
            previous = 0.0f32.to_bits();
            delta = x87_quiet_bits(t);
        }
        let target = e.get(base, NiTimeController::m_pkTarget);
        let ok = e
            .vcall(
                birth,
                INTERPOLATOR_UPDATE_FLOAT_SLOT,
                &args![0.0f32, target, at(-0x30)],
            )
            .bool();
        if !ok {
            return;
        }
        let rate = e.mem.u32(at(-0x30));
        let emit_time = x87_store32(x87_add(x87_load32(delta), x87_m32(system_time)));
        emit(e, this, [emit_time, t, previous, 0.0f32.to_bits(), t, rate]);
        return;
    }
    let keys = e
        .call(BIRTH_GET_KEYS, &args![birth, at(-4), at(-0x10), at(-0x41)])
        .u32();
    let size = u32::from(e.mem.u8(at(-0x41)));
    let count = e.mem.u32(at(-4));
    let first_time = x87_quiet_bits(e.mem.u32(keys));
    let last_offset = count.wrapping_sub(1).wrapping_mul(size);
    let last_time = x87_quiet_bits(e.mem.u32(keys.wrapping_add(last_offset)));
    let half = e.global::<f64>(HALF);
    // What the first stretch adds to the second one's emit time (`float`,
    // 0 unless the time wrapped).
    let mut offset = 0.0f32.to_bits();
    if x87_load32(t) < x87_load32(previous) {
        let stretch = x87_sub(x87_load32(last_time), x87_load32(previous));
        let mid = x87_store32(x87_add(x87_load32(previous), x87_mul(stretch, half)));
        let target = e.get(base, NiTimeController::m_pkTarget);
        e.vcall(
            birth,
            INTERPOLATOR_UPDATE_FLOAT_SLOT,
            &args![x87_quiet_bits(mid), target, at(-0x30)],
        );
        offset = x87_store32(stretch);
        let rate = e.mem.u32(at(-0x30));
        let emit_time = x87_store32(x87_add(x87_load32(offset), x87_m32(system_time)));
        emit(
            e,
            this,
            [emit_time, last_time, previous, first_time, last_time, rate],
        );
        previous = x87_quiet_bits(e.mem.u32(keys));
    }
    let stretch = x87_sub(x87_load32(t), x87_load32(previous));
    let mid = x87_store32(x87_add(x87_load32(previous), x87_mul(stretch, half)));
    let target = e.get(base, NiTimeController::m_pkTarget);
    e.vcall(
        birth,
        INTERPOLATOR_UPDATE_FLOAT_SLOT,
        &args![x87_quiet_bits(mid), target, at(-0x30)],
    );
    let rate = e.mem.u32(at(-0x30));
    let emit_time = x87_store32(x87_add(
        x87_add(x87_load32(system_time), stretch),
        x87_m32(offset),
    ));
    emit(e, this, [emit_time, t, previous, first_time, t, rate]);
}

/// `Update` walking the emitter-active keys (`frame` is the function's
/// frame, see [`ni_p_sys_emitter_ctlr_update`]).
fn update_keyed(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    birth: u32,
    t: u32,
    keys: u32,
    count: u32,
    frame: Ptr,
) {
    let at = |offset: i32| frame.addr().wrapping_add((0x48 + offset) as u32);
    let base = this.cast::<NiTimeController>();
    let no_time_double = e.global::<f64>(NO_TIME_DOUBLE);
    e.mem.set_u32(at(-8), 0);
    let gen_interp = |e: &mut Engine, time: u32| {
        let kind = e.mem.u32(at(-4));
        let size = e.mem.u32(at(-0x30));
        e.call(
            BOOL_KEY_GEN_INTERP,
            &args![x87_quiet_bits(time), keys, kind, count, at(-8), size],
        )
        .u8()
    };
    let now_active = gen_interp(e, t);
    if x87_load32(e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime)) == no_time_double {
        e.set(
            this,
            NiPSysEmitterCtlr::m_fLastScaledTime,
            x87_quiet_bits(t),
        );
        e.set(this, NiPSysEmitterCtlr::m_bLastActive, now_active);
        return;
    }
    let mut previous = x87_quiet_bits(e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime));
    let mut previous_active = e.get(this, NiPSysEmitterCtlr::m_bLastActive);
    e.set(this, NiPSysEmitterCtlr::m_bLastActive, now_active);
    e.set(
        this,
        NiPSysEmitterCtlr::m_fLastScaledTime,
        x87_quiet_bits(t),
    );
    let size = u32::from(e.mem.u8(at(-0x30)));
    let key_at = |i: u32| keys.wrapping_add(i.wrapping_mul(size));
    let key_time = |e: &Engine, i: u32| x87_load32(e.mem.u32(key_at(i)));
    let key_on = |e: &Engine, i: u32| e.mem.u8(key_at(i).wrapping_add(4)) != 0;
    let half = e.global::<f64>(HALF);
    let mut stretches = 0i32;
    loop {
        if stretches >= 0x14 {
            return;
        }
        let current = x87_load32(previous);
        let now = x87_load32(t);
        let wrapped = current > now;
        let start = if previous_active != 0 {
            // Back from the last key: one past the last inactive key at or
            // before the current time.
            let mut i = count;
            while i > 0 {
                if key_time(e, i - 1) <= current && !key_on(e, i - 1) {
                    break;
                }
                i -= 1;
            }
            i
        } else {
            let mut i = 0;
            while i < count {
                let k = key_time(e, i);
                if k > current && (wrapped || k <= now) && key_on(e, i) {
                    break;
                }
                i += 1;
            }
            if i == count {
                if !wrapped {
                    return;
                }
                i = 0;
                while i < count {
                    if key_time(e, i) < now && key_on(e, i) {
                        break;
                    }
                    i += 1;
                }
                if i == count {
                    return;
                }
            }
            i
        };
        let mut end = start;
        if start < count {
            loop {
                if !key_on(e, end) {
                    break;
                }
                end += 1;
                if end >= count {
                    break;
                }
            }
        }
        if end == count {
            end = end.wrapping_sub(1);
        }
        let end_time = key_time(e, end);
        let start_word = || key_at(start);
        let end_word = key_at(end);
        let (emit_current, emit_last, from, to, sample);
        if wrapped {
            if end_time > current {
                emit_current = x87_quiet_bits(e.mem.u32(key_at(count.wrapping_sub(1))));
                emit_last = x87_quiet_bits(previous);
                from = x87_quiet_bits(e.mem.u32(start_word()));
                to = x87_quiet_bits(e.mem.u32(end_word));
                sample = middle(from, to, half);
            } else {
                emit_current = x87_quiet_bits(t);
                emit_last = x87_quiet_bits(e.mem.u32(keys));
                from = x87_quiet_bits(e.mem.u32(start_word()));
                to = if x87_load32(e.mem.u32(end_word)) < now {
                    x87_quiet_bits(e.mem.u32(end_word))
                } else {
                    x87_quiet_bits(t)
                };
                sample = middle(from, to, half);
            }
        } else if end_time < now {
            emit_current = x87_quiet_bits(e.mem.u32(end_word));
            emit_last = x87_quiet_bits(previous);
            from = x87_quiet_bits(e.mem.u32(start_word()));
            to = x87_quiet_bits(e.mem.u32(end_word));
            sample = middle(from, to, half);
        } else {
            emit_current = x87_quiet_bits(t);
            emit_last = x87_quiet_bits(previous);
            from = x87_quiet_bits(e.mem.u32(start_word()));
            to = x87_quiet_bits(e.mem.u32(end_word));
            sample = x87_quiet_bits(t);
        }
        let target = e.get(base, NiTimeController::m_pkTarget);
        e.vcall(
            birth,
            INTERPOLATOR_UPDATE_FLOAT_SLOT,
            &args![x87_quiet_bits(sample), target, at(-0x20)],
        );
        let target = e.get(base, NiTimeController::m_pkTarget);
        let mut emit_base = x87_quiet_bits(e.mem.u32(target + 0xd4));
        if wrapped {
            let high = e.get(base, NiTimeController::m_fHiKeyTime);
            let low = e.get(base, NiTimeController::m_fLoKeyTime);
            let range = x87_sub(x87_load32(high), x87_m32(low));
            emit_base = x87_store32(x87_add(range, x87_m32(emit_base)));
        }
        let rate = e.mem.u32(at(-0x20));
        let emit_time = x87_store32(x87_sub(
            x87_add(x87_load32(emit_base), x87_m32(t)),
            x87_m32(previous),
        ));
        emit(
            e,
            this,
            [emit_time, emit_current, emit_last, from, to, rate],
        );
        if !(x87_load32(t) > key_time(e, end)) {
            return;
        }
        previous = x87_quiet_bits(emit_current);
        previous_active = gen_interp(e, emit_current);
        stretches += 1;
    }
}

// Translated from 00c1c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitterCtlr` loader the stream registers under the name
/// `"NiPSysEmitterCtlr"` (`010bd27c`; Ghidra has no function here, so this
/// is read from the bytes): a new 0x5C-byte block (null when the
/// allocation fails) given `NiPSysModifierCtlr`'s constructor and the
/// members of [`fn_00c1c390`]. Returns the block.
pub fn fn_00c1c5e0(e: &mut Engine) -> Ptr<NiPSysEmitterCtlr> {
    let block = e.call(NI_NEW, &args![0x5cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let this = Ptr::new(block);
    e.call(MODIFIER_CTLR_CONSTRUCTOR, &args![this]);
    construct_members(e, this);
    this
}

use crate::units::niparticle::nipsysdata::add_viewer_string;
use crate::units::niparticle::nipsysmodifier::fixed_string_release;

/// `NiTimeController::Stop` (`00a6cd30`, Xbox PDB name; `thiscall`).
const TIME_CONTROLLER_STOP: u32 = 0x00a6_cd30;
/// The `NiFloatInterpolator` (`00a3c390`, a `float`) and
/// `NiBoolInterpolator` (`00a532d0`, one word) constructors with a value
/// (Xbox PDB names; `thiscall`).
const FLOAT_INTERPOLATOR_WITH_VALUE: u32 = 0x00a3_c390;
const BOOL_INTERPOLATOR_WITH_VALUE: u32 = 0x00a5_32d0;
/// `NiSingleInterpController::GuaranteeTimeRange` (`00a38e90`, two
/// `float`s) and `TargetIsRequiredType` (`00a38df0`, no words) (Xbox PDB
/// names; `thiscall`).
const SINGLE_GUARANTEE_TIME_RANGE: u32 = 0x00a3_8e90;
const SINGLE_TARGET_IS_REQUIRED_TYPE: u32 = 0x00a3_8df0;
/// The interpolator slots `+0xC0` (two out `float` pointers;
/// `GetActiveTimeRange` in the Xbox PDB) and `+0xC4` (two `float`s;
/// `GuaranteeTimeRange` in the Xbox PDB).
const INTERPOLATOR_ACTIVE_TIME_RANGE_SLOT: u32 = 0xc0;
const INTERPOLATOR_GUARANTEE_TIME_RANGE_SLOT: u32 = 0xc4;
/// `NiObject`'s `GetRTTI` (`+8`), `RegisterStreamables` (`+0x54`),
/// `IsEqual` (`+0x5C`) and `GetViewerStrings` (`+0x60`) slots, and
/// `NiStream::SaveLinkID` (`+0x30`) (Xbox PDB).
const GET_RTTI_SLOT: u32 = 0x8;
const REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const IS_EQUAL_SLOT: u32 = 0x5c;
const GET_VIEWER_STRINGS_SLOT: u32 = 0x60;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// `NiPSysModifierCtlr`'s `RegisterStreamables` (`00a57780`), `SaveBinary`
/// (`00a57130`), `IsEqual` (`00c313a0`), `GetViewerStrings` (`00c315b0`)
/// and `InterpTargetIsCorrectType` (`00c31530`) (Xbox PDB names; `thiscall`,
/// one word), and its constructor with a name (`00c31440`, `thiscall`: the
/// address of an `NiFixedString`).
const MODIFIER_CTLR_REGISTER_STREAMABLES: u32 = 0x00a5_7780;
const MODIFIER_CTLR_SAVE_BINARY: u32 = 0x00a5_7130;
const MODIFIER_CTLR_IS_EQUAL: u32 = 0x00c3_13a0;
const MODIFIER_CTLR_GET_VIEWER_STRINGS: u32 = 0x00c3_15b0;
const MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE: u32 = 0x00c3_1530;
const MODIFIER_CTLR_NAMED_CONSTRUCTOR: u32 = 0x00c3_1440;
/// `00c1b0f0` (unnamed; `thiscall` on a particle system, the address of a
/// modifier name; returns a modifier or null).
const FIND_MODIFIER: u32 = 0x00c1_b0f0;
/// `00a5b690` (unnamed, `cdecl`, a C string): the `NiFixedString` handle
/// for a C string.
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// The null `NiFixedString` handle's global (`0109b220`).
const NULL_FIXED_STRING: u32 = 0x0109_b220;
/// `NiObject::NiObject` (`00a5d3a0`, `thiscall`).
const OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `NiBoolData::ReplaceAnim` (`00a538b0`, Xbox PDB name; `thiscall`: keys,
/// count, key type).
const BOOL_DATA_REPLACE_ANIM: u32 = 0x00a5_38b0;
/// The size `NiNew` is asked for an `NiBoolData`, an `NiFloatInterpolator`
/// and an `NiBoolInterpolator`.
const DATA_SIZE: u32 = 0x18;
/// The `NiRTTI` records the type tests look for: `NiPSysEmitter`
/// (`01202540`), `NiFloatInterpolator` (`011f3738`), `NiBoolInterpolator`
/// (`011f404c`) and `NiBlendBoolInterpolator` (`011f407c`) (each the
/// address its class's `GetRTTI` returns); `NiBlendFloatInterpolator`'s
/// is `BIRTH_RATE_BLEND_RTTI` (`011f3e58`).
const NI_P_SYS_EMITTER_RTTI: u32 = 0x0120_2540;
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;
const NI_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_404c;
const NI_BLEND_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_407c;
/// `NiGetViewerString` (`00a7d330`, Xbox PDB name; `cdecl`, the text).
const VIEWER_STRING: u32 = 0x00a7_d330;

/// Whether the `NiRTTI` chain from `rtti` (each record's base at +4)
/// reaches `wanted`.
fn rtti_chain_has(e: &Engine, mut rtti: u32, wanted: u32) -> bool {
    while rtti != 0 {
        if rtti == wanted {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00c1ba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::Stop` (Xbox PDB): no last scaled time (`-FLT_MAX`
/// from `01015f5c`), then `NiTimeController::Stop` (a tail jump).
pub fn ni_p_sys_emitter_ctlr_stop(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) {
    let no_time = x87_quiet_bits(e.global::<u32>(NO_TIME));
    e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, no_time);
    e.call(TIME_CONTROLLER_STOP, &args![this]);
}

// Translated from 00c1ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetInterpolatorID` (Xbox PDB): the name in the word
/// at `011ae754` for index 0, the one at `011ae758` for 1, else null.
pub fn ni_p_sys_emitter_ctlr_get_interpolator_id(
    e: &mut Engine,
    _this: Ptr<NiPSysEmitterCtlr>,
    index: u16,
) -> u32 {
    if index == 0 {
        return e.global::<u32>(BIRTH_RATE_NAME);
    }
    let mask = u32::from(index != 1).wrapping_sub(1);
    mask & e.global::<u32>(EMITTER_ACTIVE_NAME)
}

// Translated from 00c1bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::CreatePoseInterpolator` (Xbox PDB): a new 0x18-byte
/// `NiFloatInterpolator` with 0.0 for index 0 or `NiBoolInterpolator` with
/// 0 for index 1 (the constructor's answer comes back); null for other
/// indices or when the allocation fails.
pub fn ni_p_sys_emitter_ctlr_create_pose_interpolator(
    e: &mut Engine,
    _this: Ptr<NiPSysEmitterCtlr>,
    index: u16,
) -> Ptr {
    if index == 0 {
        let block = e.call(NI_NEW, &args![DATA_SIZE]).u32();
        if block == 0 {
            return Ptr::NULL;
        }
        return e
            .call(
                FLOAT_INTERPOLATOR_WITH_VALUE,
                &args![block, 0.0f32.to_bits()],
            )
            .ptr();
    }
    if index != 1 {
        return Ptr::NULL;
    }
    let block = e.call(NI_NEW, &args![DATA_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(BOOL_INTERPOLATOR_WITH_VALUE, &args![block, 0u32])
        .ptr()
}

// Translated from 00c1bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GuaranteeTimeRange` (Xbox PDB):
/// `NiSingleInterpController`'s, then the emitter-active interpolator's
/// (slot `+0xC4`) when there is one, both with `start` and `end`.
pub fn ni_p_sys_emitter_ctlr_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    start: f32,
    end: f32,
) {
    let start = x87_quiet_bits(start.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    e.call(SINGLE_GUARANTEE_TIME_RANGE, &args![this, start, end]);
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active != 0 {
        e.vcall(
            active,
            INTERPOLATOR_GUARANTEE_TIME_RANGE_SLOT,
            &args![start, end],
        );
    }
}

// Translated from 00c1bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::ResetTimeExtrema` (Xbox PDB): unless flag `0x20` of
/// `m_uFlags` is set, the birth-rate interpolator's time range (slot
/// `+0xC0`) into the low and high key times, then the emitter-active
/// interpolator's into two locals, which widen the range: the low time
/// takes the second start when it is above it, the high time the second
/// end when it is below it (a NaN changes nothing).
pub fn ni_p_sys_emitter_ctlr_reset_time_extrema(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) {
    let controller: Ptr<NiTimeController> = this.cast();
    if e.mem.u8(this.addr() + 8) & 0x20 != 0 {
        return;
    }
    let low = this.addr() + NiTimeController::m_fLoKeyTime.off;
    let high = this.addr() + NiTimeController::m_fHiKeyTime.off;
    let birth_rate = e.get(this, NiPSysEmitterCtlr::m_spInterpolator);
    e.vcall(
        birth_rate,
        INTERPOLATOR_ACTIVE_TIME_RANGE_SLOT,
        &args![low, high],
    );
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    e.with_stack(8, |e, locals| {
        let (start, end) = (locals.addr(), locals.addr() + 4);
        e.vcall(
            active,
            INTERPOLATOR_ACTIVE_TIME_RANGE_SLOT,
            &args![start, end],
        );
        let second_start = e.mem.u32(start);
        if x87_m32(e.get(controller, NiTimeController::m_fLoKeyTime)) > x87_m32(second_start) {
            e.set(
                controller,
                NiTimeController::m_fLoKeyTime,
                x87_quiet_bits(second_start),
            );
        }
        let second_end = e.mem.u32(end);
        if x87_m32(e.get(controller, NiTimeController::m_fHiKeyTime)) < x87_m32(second_end) {
            e.set(
                controller,
                NiTimeController::m_fHiKeyTime,
                x87_quiet_bits(second_end),
            );
        }
    });
}

// Translated from 00c1be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::RegisterStreamables` (Xbox PDB):
/// `NiPSysModifierCtlr`'s; when that succeeds the emitter-active
/// interpolator's (slot `+0x54`, its answer not read) and true.
pub fn ni_p_sys_emitter_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    stream: Ptr,
) -> bool {
    if !e
        .call(MODIFIER_CTLR_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active != 0 {
        e.vcall(active, REGISTER_STREAMABLES_SLOT, &args![stream]);
    }
    true
}

// Translated from 00c1be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::SaveBinary` (Xbox PDB): `NiPSysModifierCtlr`'s, then
/// the emitter-active interpolator's link id (`NiStream::SaveLinkID`, slot
/// `+0x30`).
pub fn ni_p_sys_emitter_ctlr_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    stream: Ptr,
) {
    e.call(MODIFIER_CTLR_SAVE_BINARY, &args![this, stream]);
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![active]);
}

// Translated from 00c1be60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::IsEqual` (Xbox PDB): `NiPSysModifierCtlr`'s, then
/// both or neither must have an emitter-active interpolator, and two must
/// be equal (the other's `IsEqual`, slot `+0x5C`, given this one's).
pub fn ni_p_sys_emitter_ctlr_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    other: Ptr<NiPSysEmitterCtlr>,
) -> bool {
    if !e.call(MODIFIER_CTLR_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let theirs = e.get(other, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    let ours = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if theirs == 0 {
        return ours == 0;
    }
    if ours == 0 {
        return false;
    }
    e.vcall(theirs, IS_EQUAL_SLOT, &args![ours]).bool()
}

// Translated from 00c1bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::ReplaceKeys` (Xbox PDB), emitted in this unit: with
/// keys (`count` not 0), makes the interpolator's `NiBoolData` (+0x10) when
/// it has none (a new 0x18-byte `NiBoolData`: `NiObject`'s constructor, its
/// vtable, no keys; null when the allocation fails; assigned through
/// `00a5f860`), hands it the keys (`ReplaceAnim` with keys, count, type) and
/// clears +0x14; without keys, empties the data it has (`ReplaceAnim` with
/// zeros).
pub fn ni_bool_interpolator_replace_keys(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    key_type: u32,
    count: u32,
) {
    let slot = this.addr() + 0x10;
    if count == 0 {
        let data = e.mem.u32(slot);
        if data != 0 {
            e.call(BOOL_DATA_REPLACE_ANIM, &args![data, 0u32, 0u32, 0u32]);
        }
        return;
    }
    if e.mem.u32(slot) == 0 {
        let data = e.call(NI_NEW, &args![DATA_SIZE]).u32();
        if data != 0 {
            e.call(OBJECT_CONSTRUCTOR, &args![data]);
            e.mem.set_u32(data, NI_BOOL_DATA_VTABLE);
            e.mem.set_u32(data + 0x8, 0);
            e.mem.set_u32(data + 0xc, 0);
            e.mem.set_u32(data + 0x10, 0);
            e.mem.set_u8(data + 0x14, 0);
        }
        e.call(POINTER_ASSIGN, &args![slot, data]);
    }
    let data = e.mem.u32(slot);
    e.call(BOOL_DATA_REPLACE_ANIM, &args![data, keys, count, key_type]);
    e.mem.set_u32(this.addr() + 0x14, 0);
}

// Translated from 00c1bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::TargetIsRequiredType` (Xbox PDB):
/// `NiSingleInterpController`'s test, then the target's modifier of this
/// controller's name (`00c1b0f0`) must exist and be an `NiPSysEmitter`
/// (its `NiRTTI` chain).
pub fn ni_p_sys_emitter_ctlr_target_is_required_type(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
) -> bool {
    if !e.call(SINGLE_TARGET_IS_REQUIRED_TYPE, &args![this]).bool() {
        return false;
    }
    let target = e.mem.u32(this.addr() + NiTimeController::m_pkTarget.off);
    let name = this.addr() + NiPSysEmitterCtlr::m_kModifierName.off;
    let modifier = e.call(FIND_MODIFIER, &args![target, name]).u32();
    if modifier == 0 {
        return false;
    }
    let rtti = e.vcall(modifier, GET_RTTI_SLOT, &args![]).u32();
    rtti_chain_has(e, rtti, NI_P_SYS_EMITTER_RTTI)
}

// Translated from 00c1c040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::InterpolatorIsCorrectType` (Xbox PDB): for index 0 a
/// non-null `NiFloatInterpolator` or `NiBlendFloatInterpolator` (two walks
/// of its `NiRTTI` chain, `GetRTTI` asked each time); for index 1 an
/// `NiBoolInterpolator` (null skips that walk) or, by the type test
/// `0043b300`, an `NiBlendBoolInterpolator`; false otherwise.
pub fn ni_p_sys_emitter_ctlr_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr<NiPSysEmitterCtlr>,
    interpolator: Ptr,
    index: u16,
) -> bool {
    let interpolator = interpolator.addr();
    match index {
        0 => {
            if interpolator == 0 {
                return false;
            }
            let rtti = e.vcall(interpolator, GET_RTTI_SLOT, &args![]).u32();
            if rtti_chain_has(e, rtti, NI_FLOAT_INTERPOLATOR_RTTI) {
                return true;
            }
            let rtti = e.vcall(interpolator, GET_RTTI_SLOT, &args![]).u32();
            rtti_chain_has(e, rtti, BIRTH_RATE_BLEND_RTTI)
        }
        1 => {
            if interpolator != 0 {
                let rtti = e.vcall(interpolator, GET_RTTI_SLOT, &args![]).u32();
                if rtti_chain_has(e, rtti, NI_BOOL_INTERPOLATOR_RTTI) {
                    return true;
                }
            }
            e.call(
                IS_KIND_OF,
                &args![NI_BLEND_BOOL_INTERPOLATOR_RTTI, interpolator],
            )
            .bool()
        }
        _ => false,
    }
}

// Translated from 00c1c280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitterCtlr` constructor with a modifier name (no PDB name):
/// the name's `NiFixedString` (the null handle for a null name, else
/// `00a5b690`) in a local, `NiPSysModifierCtlr`'s constructor with it
/// (`00c31440`), the local released, then the members. Returns `this`.
pub fn fn_00c1c280(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    name: u32,
) -> Ptr<NiPSysEmitterCtlr> {
    let handle = if name == 0 {
        e.global::<u32>(NULL_FIXED_STRING)
    } else {
        e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
    };
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), handle);
        e.call(MODIFIER_CTLR_NAMED_CONSTRUCTOR, &args![this, local]);
        fixed_string_release(e, local.addr());
    });
    construct_members(e, this);
    this
}

// Translated from 00c1ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetViewerStrings` (Xbox PDB): `NiPSysModifierCtlr`'s
/// strings, the class name (the word at `012024e8`), then the
/// emitter-active interpolator's strings (slot `+0x60`) when there is one.
pub fn ni_p_sys_emitter_ctlr_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    strings: Ptr,
) {
    e.call(MODIFIER_CTLR_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(NI_P_SYS_EMITTER_CTLR_RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings.addr(), string);
    let active = e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator);
    if active != 0 {
        e.vcall(active, GET_VIEWER_STRINGS_SLOT, &args![strings]);
    }
}

// Translated from 00c2b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::InterpTargetIsCorrectType` (Xbox PDB), emitted with
/// the float controllers: `NiPSysModifierCtlr`'s test, then `target`'s
/// modifier of this controller's name (`00c1b0f0`) must exist and be an
/// `NiPSysEmitter` (its `NiRTTI` chain).
pub fn ni_p_sys_emitter_ctlr_interp_target_is_correct_type(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlr>,
    target: Ptr,
) -> bool {
    if !e
        .call(
            MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE,
            &args![this, target],
        )
        .bool()
    {
        return false;
    }
    let name = this.addr() + NiPSysEmitterCtlr::m_kModifierName.off;
    let modifier = e.call(FIND_MODIFIER, &args![target, name]).u32();
    if modifier == 0 {
        return false;
    }
    let rtti = e.vcall(modifier, GET_RTTI_SLOT, &args![]).u32();
    rtti_chain_has(e, rtti, NI_P_SYS_EMITTER_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1_c5e0, fn_00c1c5e0() -> Ptr<NiPSysEmitterCtlr>),
        entry!(0x00a537e0, ni_bool_data_get_rtti() -> Ptr),
        entry!(0x00c1c2f0, ni_p_sys_emitter_ctlr_get_rtti() -> Ptr),
        entry!(
            0x00a3_6250,
            ni_time_controller_dont_do_update(Ptr<NiTimeController>, f32) -> bool
        ),
        entry!(0x00a5_3a10, ni_bool_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c1_ba40,
            ni_p_sys_emitter_ctlr_get_interpolator_count(Ptr<NiPSysEmitterCtlr>) -> u32
        ),
        entry!(
            0x00c1_ba80,
            ni_p_sys_emitter_ctlr_get_interpolator_index(Ptr<NiPSysEmitterCtlr>, Ptr) -> u16
        ),
        entry!(
            0x00c1_bb20,
            ni_p_sys_emitter_ctlr_create_blend_interpolator(
                Ptr<NiPSysEmitterCtlr>,
                u16,
                u32,
                u32,
                f32,
                u32,
            ) -> Ptr
        ),
        entry!(0x00c1_bb90, fn_00c1bb90(Ptr, u8) -> f32),
        entry!(
            0x00c1_bc90,
            ni_p_sys_emitter_ctlr_get_interpolator(Ptr<NiPSysEmitterCtlr>, u16) -> Ptr
        ),
        entry!(
            0x00c1_bcc0,
            ni_p_sys_emitter_ctlr_set_interpolator(Ptr<NiPSysEmitterCtlr>, Ptr, u16)
        ),
        entry!(
            0x00c1_bd30,
            fn_00c1bd30(Ptr<NiPSysEmitterCtlr>, Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c1_bdc0,
            ni_p_sys_emitter_ctlr_process_clone(Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c1_bdf0,
            ni_p_sys_emitter_ctlr_load_binary(Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c1_bf90,
            ni_p_sys_emitter_ctlr_get_emitter_active_interpolator(Ptr<NiPSysEmitterCtlr>) -> Ptr
        ),
        entry!(
            0x00c1_c0e0,
            ni_p_sys_emitter_ctlr_link_object(Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(0x00c1_c240, fn_00c1c240(Ptr, Ptr)),
        entry!(0x00c1_c300, fn_00c1c300(Ptr<NiPSysEmitterCtlr>)),
        entry!(
            0x00c1_c340,
            ni_p_sys_emitter_ctlr_scalar_deleting_destructor(
                Ptr<NiPSysEmitterCtlr>,
                u32,
            ) -> Ptr<NiPSysEmitterCtlr>
        ),
        entry!(
            0x00c1_c390,
            fn_00c1c390(Ptr<NiPSysEmitterCtlr>) -> Ptr<NiPSysEmitterCtlr>
        ),
        entry!(
            0x00c1_c3c0,
            ni_p_sys_emitter_ctlr_emit_particles(
                Ptr<NiPSysEmitterCtlr>,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
            )
        ),
        entry!(
            0x00c1_c570,
            ni_p_sys_emitter_ctlr_create_clone(
                Ptr<NiPSysEmitterCtlr>,
                Ptr,
            ) -> Ptr<NiPSysEmitterCtlr>
        ),
        entry!(
            0x00c1_c630,
            ni_p_sys_emitter_ctlr_update(Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c1_ba30,
            ni_p_sys_emitter_ctlr_stop(Ptr<NiPSysEmitterCtlr>)
        ),
        entry!(
            0x00c1_ba50,
            ni_p_sys_emitter_ctlr_get_interpolator_id(Ptr<NiPSysEmitterCtlr>, u16) -> u32
        ),
        entry!(
            0x00c1_bad0,
            ni_p_sys_emitter_ctlr_create_pose_interpolator(Ptr<NiPSysEmitterCtlr>, u16) -> Ptr
        ),
        entry!(
            0x00c1_bbc0,
            ni_p_sys_emitter_ctlr_guarantee_time_range(Ptr<NiPSysEmitterCtlr>, f32, f32)
        ),
        entry!(
            0x00c1_bc10,
            ni_p_sys_emitter_ctlr_reset_time_extrema(Ptr<NiPSysEmitterCtlr>)
        ),
        entry!(
            0x00c1_be10,
            ni_p_sys_emitter_ctlr_register_streamables(Ptr<NiPSysEmitterCtlr>, Ptr) -> bool
        ),
        entry!(
            0x00c1_be40,
            ni_p_sys_emitter_ctlr_save_binary(Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c1_be60,
            ni_p_sys_emitter_ctlr_is_equal(Ptr<NiPSysEmitterCtlr>, Ptr<NiPSysEmitterCtlr>) -> bool
        ),
        entry!(
            0x00c1_bf10,
            ni_bool_interpolator_replace_keys(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00c1_bff0,
            ni_p_sys_emitter_ctlr_target_is_required_type(Ptr<NiPSysEmitterCtlr>) -> bool
        ),
        entry!(
            0x00c1_c040,
            ni_p_sys_emitter_ctlr_interpolator_is_correct_type(
                Ptr<NiPSysEmitterCtlr>,
                Ptr,
                u16,
            ) -> bool
        ),
        entry!(
            0x00c1_c280,
            fn_00c1c280(Ptr<NiPSysEmitterCtlr>, u32) -> Ptr<NiPSysEmitterCtlr>
        ),
        entry!(
            0x00c1_ce70,
            ni_p_sys_emitter_ctlr_get_viewer_strings(Ptr<NiPSysEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c2_b7d0,
            ni_p_sys_emitter_ctlr_interp_target_is_correct_type(
                Ptr<NiPSysEmitterCtlr>,
                Ptr,
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a537e0, &args![]).u32(), NI_BOOL_DATA_RTTI);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c1c2f0, &args![]).u32(),
            NI_P_SYS_EMITTER_CTLR_RTTI
        );
    }
}

#[cfg(test)]
mod emitter_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const OBJECT_VTABLE: u32 = 0x0003_0000;
    const DELETE_THIS: u32 = 0x0000_1004;
    const SCALED_TIME: u32 = 0x0000_10a4;
    const UPDATE_BOOL: u32 = 0x0000_10a0;
    const UPDATE_FLOAT: u32 = 0x0000_109c;
    const POSED: u32 = 0x0000_10f4;
    const SLOT_D8: u32 = 0x0000_10d8;
    const CREATE_CLONE: u32 = 0x0000_1048;
    const PROCESS_CLONE: u32 = 0x0000_1068;
    const SET_INTERPOLATOR: u32 = 0x0000_10c8;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// One vtable for every test object; every slot's double is quiet
    /// unless a test registers another.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x40];
        for (slot, target) in [
            (DELETE_THIS_SLOT, DELETE_THIS),
            (COMPUTE_SCALED_TIME_SLOT, SCALED_TIME),
            (INTERPOLATOR_UPDATE_BOOL_SLOT, UPDATE_BOOL),
            (INTERPOLATOR_UPDATE_FLOAT_SLOT, UPDATE_FLOAT),
            (INTERPOLATOR_POSED_SLOT, POSED),
            (INTERPOLATOR_SLOT_D8, SLOT_D8),
            (CREATE_CLONE_SLOT, CREATE_CLONE),
            (PROCESS_CLONE_SLOT, PROCESS_CLONE),
            (SET_INTERPOLATOR_SLOT, SET_INTERPOLATOR),
        ] {
            slots[(slot / 4) as usize] = target;
        }
        e.put_vtable(OBJECT_VTABLE, &slots);
        // `InterlockedIncrement` and `InterlockedDecrement` (the import
        // slots), counting in memory.
        e.register(0x00fd_f1c0, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register(0x00fd_f1c4, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        quiet(
            &mut e,
            &[
                DELETE_THIS,
                SCALED_TIME,
                UPDATE_BOOL,
                UPDATE_FLOAT,
                POSED,
                SLOT_D8,
                CREATE_CLONE,
                PROCESS_CLONE,
                SET_INTERPOLATOR,
            ],
        );
        e
    }

    /// A reference-counted test object (vtable [`OBJECT_VTABLE`]).
    fn object(e: &mut Engine, size: u32, count: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn controller(e: &mut Engine) -> Ptr<NiPSysEmitterCtlr> {
        Ptr::<NiPSysEmitterCtlr>::new(object(e, 0x5c, 1))
    }

    fn base(this: Ptr<NiPSysEmitterCtlr>) -> Ptr<NiTimeController> {
        this.cast()
    }

    #[test]
    fn ni_time_controller_dont_do_update_follows_the_flags() {
        let mut e = engine();
        e.register(SCALED_TIME, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[1])) * 2.0,
            ..Ret::default()
        });
        let this = base(controller(&mut e));
        let run = |e: &mut Engine, flags: u16, time: f32| {
            e.set(this, NiTimeController::m_uFlags, flags);
            e.call(0x00a3_6250, &args![this, time]).bool()
        };
        e.set(this, NiTimeController::m_fLastTime, bits(1.0));
        assert!(run(&mut e, 0, 2.0));
        assert!(run(&mut e, 8, 1.0));
        // No scaled time wanted: the force flag is cleared.
        assert!(!run(&mut e, 0x88, 1.0));
        assert_eq!(e.get(this, NiTimeController::m_uFlags), 8);
        assert!(!run(&mut e, 0x1c8, 2.0));
        assert_eq!(e.get(this, NiTimeController::m_fScaledTime), bits(4.0));
        assert_eq!(e.get(this, NiTimeController::m_uFlags), 0x148);
        // The same scaled time again: skipped.
        assert!(run(&mut e, 0x48, 2.0));
        assert!(!run(&mut e, 0x48, 3.0));
        assert_eq!(e.get(this, NiTimeController::m_fScaledTime), bits(6.0));
    }

    #[test]
    fn ni_bool_data_scalar_deleting_destructor_frees_the_keys() {
        let mut e = Engine::new();
        quiet(&mut e, &[DEALLOCATE, OBJECT_DESTRUCT, NI_DELETE]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x0c, 0x6004);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a5_3a10, &args![this, 1u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), NI_BOOL_DATA_VTABLE);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x6000]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this, 0x18]]);
        e.mem.set_u32(this + 0x0c, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3a10, &args![this, 0u32]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        assert!(calls_to(&e, NI_DELETE).is_empty());
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_get_interpolator_count_is_two() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1_ba40, &args![0x5000u32]).u32(), 2);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_get_interpolator_index_matches_the_names() {
        let mut e = Engine::new();
        e.map(BIRTH_RATE_NAME, 8);
        e.set_global(BIRTH_RATE_NAME, 0x0000_7100u32);
        e.set_global(EMITTER_ACTIVE_NAME, 0x0000_7200u32);
        e.map(INVALID_INDEX, 2);
        e.set_global(INVALID_INDEX, 0xffffu16);
        e.register(NI_STRICMP, |_, a| Ret {
            eax: (a[0] != a[1]) as u32,
            ..Ret::default()
        });
        let index = |e: &mut Engine, name: u32| e.call(0x00c1_ba80, &args![0x5000u32, name]).u16();
        assert_eq!(index(&mut e, 0x7100), 0);
        assert_eq!(index(&mut e, 0x7200), 1);
        assert_eq!(index(&mut e, 0x7300), 0xffff);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_create_blend_interpolator_by_index() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        for constructor in [BLEND_FLOAT_CONSTRUCTOR, BLEND_BOOL_CONSTRUCTOR] {
            e.register(constructor, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        let create = |e: &mut Engine, index: u32| {
            e.call(
                0x00c1_bb20,
                &args![0x5000u32, index, 1u32, 9u32, 0.25f32, 4u32],
            )
            .u32()
        };
        e.call_log = Some(vec![]);
        let float = create(&mut e, 0);
        let boolean = create(&mut e, 1);
        assert_eq!(create(&mut e, 2), 0);
        assert_eq!(
            calls_to(&e, BLEND_FLOAT_CONSTRUCTOR),
            vec![vec![float, 1, bits(0.25), 4]]
        );
        assert_eq!(
            calls_to(&e, BLEND_BOOL_CONSTRUCTOR),
            vec![vec![boolean, 1, bits(0.25), 4]]
        );
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34], vec![0x34]]);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(create(&mut e, 0), 0);
    }

    #[test]
    fn fn_00c1bb90_answers_the_single_or_item_time() {
        let mut e = Engine::new();
        let blend = e.mem.alloc(0x30);
        let items = e.mem.alloc(0x30);
        e.mem.set_u32(blend + 0x14, items);
        e.mem.set_f32(blend + 0x20, 7.0);
        e.mem.set_f32(items + 0x18 + 0x14, 3.5);
        e.mem.set_u8(blend + 0x0e, 1);
        e.mem.set_u8(blend + 0x0f, 1);
        assert_eq!(e.call(0x00c1_bb90, &args![blend, 1u32]).f64(), 7.0);
        e.mem.set_u8(blend + 0x0e, 2);
        assert_eq!(e.call(0x00c1_bb90, &args![blend, 1u32]).f64(), 3.5);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_get_and_set_interpolator() {
        let mut e = engine();
        quiet(&mut e, &[POINTER_ASSIGN]);
        let this = controller(&mut e);
        let old = object(&mut e, 0x10, 1);
        let new = object(&mut e, 0x10, 1);
        e.set(this, NiPSysEmitterCtlr::m_spInterpolator, old);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            0x7777,
        );
        assert_eq!(e.call(0x00c1_bc90, &args![this, 0u32]).u32(), old);
        assert_eq!(e.call(0x00c1_bc90, &args![this, 1u32]).u32(), 0x7777);
        assert_eq!(e.call(0x00c1_bc90, &args![this, 2u32]).u32(), 0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_bcc0, &args![this, new, 0u32]);
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_spInterpolator), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old]]);
        e.call(0x00c1_bcc0, &args![this, new, 1u32]);
        e.call(0x00c1_bcc0, &args![this, new, 2u32]);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this.addr() + 0x40, new]]
        );
    }

    fn phase_globals(e: &mut Engine) {
        e.map(RAND_MAX, 8);
        e.set_global(RAND_MAX, 32767.0f64);
        e.map(PHASE_RANGE, 8);
        e.set_global(PHASE_RANGE, 100.0f64);
        e.register(RAND, |_, _| Ret {
            eax: 32767,
            ..Ret::default()
        });
    }

    #[test]
    fn fn_00c1bd30_copies_the_active_interpolator_and_randomizes_the_phase() {
        let mut e = engine();
        phase_globals(&mut e);
        quiet(&mut e, &[MODIFIER_CTLR_COPY_MEMBERS]);
        let copy = object(&mut e, 0x10, 0);
        e.register_double(CREATE_CLONE, move |_, _| Ret {
            eax: copy,
            ..Ret::default()
        });
        let this = controller(&mut e);
        let clone = controller(&mut e);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            0x0003_1000,
        );
        e.map(0x0003_1000, 8);
        e.mem.set_u32(0x0003_1000, OBJECT_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00c1_bd30, &args![this, clone, 0x9000u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x9000]]
        );
        assert_eq!(calls_to(&e, CREATE_CLONE), vec![vec![0x0003_1000, 0x9000]]);
        assert_eq!(
            e.get(clone, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator),
            copy
        );
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert_eq!(e.get(base(clone), NiTimeController::m_fPhase), bits(100.0));
        // A cycle bit, or a phase already set: no random phase.
        e.set(base(clone), NiTimeController::m_fPhase, 0);
        e.set(base(clone), NiTimeController::m_uFlags, 2);
        e.call(0x00c1_bd30, &args![this, clone, 0x9000u32]);
        assert_eq!(e.get(base(clone), NiTimeController::m_fPhase), 0);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_process_clone_and_load_binary() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MODIFIER_CTLR_PROCESS_CLONE,
                MODIFIER_CTLR_LOAD_BINARY,
                STREAM_READ_LINK_ID,
            ],
        );
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c1_bdc0, &args![this, 0x9000u32]);
        assert!(calls_to(&e, PROCESS_CLONE).is_empty());
        let active = object(&mut e, 0x10, 1);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.call(0x00c1_bdc0, &args![this, 0x9000u32]);
        assert_eq!(calls_to(&e, PROCESS_CLONE), vec![vec![active, 0x9000]]);
        assert_eq!(calls_to(&e, MODIFIER_CTLR_PROCESS_CLONE).len(), 2);
        e.call(0x00c1_bdf0, &args![this, 0x8000u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_LOAD_BINARY),
            vec![vec![this.addr(), 0x8000]]
        );
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID), vec![vec![0x8000]]);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_get_emitter_active_interpolator_finds_the_blend_item() {
        let mut e = engine();
        e.map(INVALID_BLEND_INDEX, 1);
        e.set_global(INVALID_BLEND_INDEX, 0xffu8);
        let this = controller(&mut e);
        let active = object(&mut e, 0x30, 1);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.register(SLOT_D8, |_, _| Ret {
            eax: 0x4444,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00c1_bf90, &args![this]).u32(), 0x4444);
        e.register(SLOT_D8, |_, _| Ret::default());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert_eq!(e.call(0x00c1_bf90, &args![this]).u32(), 0);
        e.register_double(DYNAMIC_CAST, move |_, _| Ret {
            eax: active,
            ..Ret::default()
        });
        let index = Rc::new(RefCell::new(0xffu32));
        let seen = index.clone();
        e.register_double(GET_HIGHEST_WEIGHTED_INDEX, move |_, _| Ret {
            eax: *seen.borrow(),
            ..Ret::default()
        });
        assert_eq!(e.call(0x00c1_bf90, &args![this]).u32(), 0);
        *index.borrow_mut() = 1;
        let items = e.mem.alloc(0x30);
        e.mem.set_u32(items + 0x18, 0x5555);
        e.mem.set_u32(active + 0x14, items);
        e.mem.set_u32(active + 0x18, 0x6666);
        assert_eq!(e.call(0x00c1_bf90, &args![this]).u32(), 0x5555);
        e.mem.set_u8(active + 0x0e, 1);
        e.mem.set_u8(active + 0x0f, 1);
        assert_eq!(e.call(0x00c1_bf90, &args![this]).u32(), 0x6666);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_link_object_converts_old_data() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MODIFIER_CTLR_LINK_OBJECT,
                FLOAT_REPLACE_KEYS,
                BOOL_REPLACE_KEYS,
            ],
        );
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        for constructor in [
            FLOAT_INTERPOLATOR_CONSTRUCTOR,
            BOOL_INTERPOLATOR_CONSTRUCTOR,
        ] {
            e.register(constructor, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        let data = object(&mut e, 0x28, 1);
        for (offset, value) in [
            (0x08, 3u32),
            (0x0c, 0x6100),
            (0x10, 1),
            (0x18, 2),
            (0x1c, 0x6200),
            (0x20, 4),
        ] {
            e.mem.set_u32(data + offset, value);
        }
        e.mem.set_u8(data + 0x14, 9);
        e.mem.set_u8(data + 0x24, 9);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| Ret {
            eax: data,
            ..Ret::default()
        });
        let stream = e.mem.alloc(0xe0);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0067);
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c1_c0e0, &args![this, stream]);
        let float = calls_to(&e, FLOAT_REPLACE_KEYS)[0].clone();
        assert_eq!(float[1..], [0x6100, 3, 1]);
        let boolean = calls_to(&e, BOOL_REPLACE_KEYS)[0].clone();
        assert_eq!(boolean[1..], [0x6200, 4, 2]);
        assert_eq!(
            calls_to(&e, SET_INTERPOLATOR),
            vec![
                vec![this.addr(), float[0], 0],
                vec![this.addr(), boolean[0], 1]
            ]
        );
        for offset in [0x08, 0x0c, 0x10, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(data + offset), 0);
        }
        assert_eq!(e.mem.u8(data + 0x14), 0);
        assert_eq!(e.mem.u8(data + 0x24), 0);
        // The data's count: taken and dropped (1 -> 2 -> 1).
        assert_eq!(e.mem.u32(data + 4), 1);
        assert!(calls_to(&e, DELETE_THIS).is_empty());
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_link_object_links_the_active_interpolator() {
        let mut e = engine();
        phase_globals(&mut e);
        quiet(&mut e, &[MODIFIER_CTLR_LINK_OBJECT]);
        let linked = object(&mut e, 0x10, 0);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| Ret {
            eax: linked,
            ..Ret::default()
        });
        let stream = e.mem.alloc(0xe0);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        let this = controller(&mut e);
        e.call(0x00c1_c0e0, &args![this, stream]);
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator),
            linked
        );
        assert_eq!(e.mem.u32(linked + 4), 1);
        assert_eq!(e.get(base(this), NiTimeController::m_fPhase), bits(100.0));
        // Old data that is not there: nothing more.
        e.register(STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| Ret::default());
        e.mem.set_u32(stream + 0xd8, 0x0a01_0000);
        e.call_log = Some(vec![]);
        e.call(0x00c1_c0e0, &args![this, stream]);
        assert!(calls_to(&e, NI_NEW).is_empty());
    }

    #[test]
    fn fn_00c1c240_appends_and_grows() {
        let mut e = Engine::new();
        let grown: Log = Rc::new(RefCell::new(vec![]));
        let seen = grown.clone();
        e.register_double(SET_REALLOCATE, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            let buffer = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 8);
            if size > 0 {
                let words = e.mem.bytes(old, 4 * size);
                e.mem.write(buffer, &words);
            }
            e.mem.set_u32(a[0], buffer);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let set = e.mem.alloc(12);
        let value = e.mem.alloc(4);
        for (i, v) in [1.5f32, 2.5, 3.5].iter().enumerate() {
            e.mem.set_f32(value, *v);
            e.call(0x00c1_c240, &args![set, value]);
            assert_eq!(e.mem.u32(set + 8), i as u32 + 1);
        }
        assert_eq!(
            *grown.borrow(),
            vec![vec![set, 1], vec![set, 2], vec![set, 4]]
        );
        let buffer = e.mem.u32(set);
        assert_eq!(e.mem.f32(buffer + 8), 3.5);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_destructors_free_and_release() {
        let mut e = engine();
        quiet(&mut e, &[NI_FREE, MODIFIER_CTLR_DESTRUCTOR, NI_DELETE]);
        let this = controller(&mut e);
        let active = object(&mut e, 0x10, 1);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.set(this, NiPSysEmitterCtlr::m_kParticleAges, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x00c1_c300, &args![this]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x6600]]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![active]]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
        e.set(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator, 0);
        e.call_log = Some(vec![]);
        let result = e.call(0x00c1_c340, &args![this, 1u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x5c]]);
        e.call_log = Some(vec![]);
        e.call(0x00c1_c340, &args![this, 0u32]);
        assert!(calls_to(&e, NI_DELETE).is_empty());
    }

    #[test]
    fn fn_00c1c390_and_create_clone_construct_the_controller() {
        let mut e = engine();
        e.map(NO_TIME, 4);
        e.set_global(NO_TIME, -f32::MAX);
        quiet(
            &mut e,
            &[MODIFIER_CTLR_CONSTRUCTOR, MODIFIER_CTLR_COPY_MEMBERS],
        );
        phase_globals(&mut e);
        let this = controller(&mut e);
        e.set(this, NiPSysEmitterCtlr::m_bLastActive, 1);
        assert_eq!(e.call(0x00c1_c390, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_P_SYS_EMITTER_CTLR_VTABLE);
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime),
            bits(-f32::MAX)
        );
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_bLastActive), 0);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c1_c570, &args![this, 0x9000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), NI_P_SYS_EMITTER_CTLR_VTABLE);
        assert_eq!(calls_to(&e, MODIFIER_CTLR_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x9000]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        // The copy runs on a null clone: its flags and phase are read at 0.
        e.map(0, 0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_c570, &args![this, 0x9000u32]).u32(), 0);
        assert!(calls_to(&e, MODIFIER_CTLR_CONSTRUCTOR).is_empty());
        let copied = calls_to(&e, MODIFIER_CTLR_COPY_MEMBERS);
        assert_eq!(copied, vec![vec![this.addr(), 0, 0x9000]]);
    }

    #[test]
    fn fn_00c1c5e0_creates_a_constructed_controller() {
        let mut e = engine();
        e.map(NO_TIME, 4);
        e.set_global(NO_TIME, -f32::MAX);
        quiet(&mut e, &[MODIFIER_CTLR_CONSTRUCTOR]);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00c1_c5e0, &args![]).u32();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x5c]]);
        assert_eq!(calls_to(&e, MODIFIER_CTLR_CONSTRUCTOR), vec![vec![made]]);
        let this = Ptr::<NiPSysEmitterCtlr>::new(made);
        assert_eq!(e.mem.u32(made), NI_P_SYS_EMITTER_CTLR_VTABLE);
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime),
            bits(-f32::MAX)
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_c5e0, &args![]).u32(), 0);
        assert!(calls_to(&e, MODIFIER_CTLR_CONSTRUCTOR).is_empty());
    }

    /// An emitter whose emit slot records (time, count, ages), and a set
    /// that grows in a heap buffer.
    fn emitting(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>) -> Log {
        e.map(MAX_EMIT, 2);
        e.set_global(MAX_EMIT, 15u16);
        e.register(SET_REALLOCATE, |e, a| {
            let buffer = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 8);
            if size > 0 {
                let words = e.mem.bytes(old, 4 * size);
                e.mem.write(buffer, &words);
            }
            e.mem.set_u32(a[0], buffer);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let emitter = object(e, 0x50, 1);
        let mut slots = vec![0u32; 0x30];
        slots[(EMITTER_EMIT_SLOT / 4) as usize] = 0x0000_1ea4;
        e.put_vtable(0x0003_2000, &slots);
        e.mem.set_u32(emitter, 0x0003_2000);
        e.set(this, NiPSysEmitterCtlr::m_pkModifier, emitter);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let seen = log.clone();
        e.register_double(0x0000_1ea4, move |e, a| {
            let mut words = vec![a[1], a[2]];
            for i in 0..a[2] {
                words.push(e.mem.u32(a[3] + 4 * i));
            }
            seen.borrow_mut().push(words);
            Ret::default()
        });
        log
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_emit_particles_counts_and_ages_the_births() {
        let mut e = engine();
        let this = controller(&mut e);
        let log = emitting(&mut e, this);
        let emit = |e: &mut Engine, words: [f32; 6]| {
            let [a, b, c, d, f, g] = words;
            e.call(0x00c1_c3c0, &args![this, a, b, c, d, f, g]);
        };
        // Window 1 .. 3, frame 1.5 .. 2.5, rate 4: births 2 .. 6.
        emit(&mut e, [9.0, 2.5, 1.5, 1.0, 3.0, 4.0]);
        // The frame past the stop: up to the stop; before the start: from it.
        emit(&mut e, [9.0, 5.0, 0.0, 1.0, 3.0, 2.0]);
        // A count above the limit is capped at 15.
        emit(&mut e, [9.0, 3.0, 1.0, 1.0, 3.0, 100.0]);
        // Nothing for a non-positive rate or an empty window or frame.
        emit(&mut e, [9.0, 2.5, 1.5, 1.0, 3.0, 0.0]);
        emit(&mut e, [9.0, 2.5, 1.5, 3.0, 3.0, 4.0]);
        emit(&mut e, [9.0, 1.0, 0.0, 1.0, 3.0, 4.0]);
        emit(&mut e, [9.0, 4.0, 3.0, 1.0, 3.0, 4.0]);
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        let ages = |row: &Vec<u32>| {
            row[2..]
                .iter()
                .map(|w| f32::from_bits(*w))
                .collect::<Vec<_>>()
        };
        assert_eq!(log[0][..2], [bits(9.0), 4]);
        assert_eq!(ages(&log[0]), vec![0.75, 0.5, 0.25, 0.0]);
        assert_eq!(log[1][1], 4);
        assert_eq!(ages(&log[1]), vec![1.5, 1.0, 0.5, 0.0]);
        assert_eq!(log[2][1], 15);
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_kParticleAgesSize), 15);
    }

    /// The globals `Update` reads, and doubles that make neither
    /// interpolator a blend.
    type Scaled = Rc<RefCell<Option<f64>>>;
    type Setup = (Engine, Ptr<NiPSysEmitterCtlr>, u32, u32, Log, Scaled);

    fn update_engine() -> Setup {
        let mut e = engine();
        for (address, value) in [(NO_TIME_DOUBLE, -f64::from(f32::MAX)), (HALF, 0.5)] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        for address in [NO_TIME, NO_TIME_FLAG] {
            e.map(address, 4);
            e.set_global(address, -f32::MAX);
        }
        e.map(INVALID_BLEND_INDEX, 1);
        e.set_global(INVALID_BLEND_INDEX, 0xffu8);
        e.register(IS_KIND_OF, |_, _| Ret::default());
        let this = controller(&mut e);
        let log = emitting(&mut e, this);
        let target = e.mem.alloc(0xe0);
        e.mem.set_f32(target + 0xd4, 100.0);
        e.set(base(this), NiTimeController::m_pkTarget, target);
        e.set(base(this), NiTimeController::m_uFlags, 0x48);
        e.set(base(this), NiTimeController::m_fLastTime, bits(0.0));
        // The emitter does not let the last time follow (`+0x48`).
        let emitter = e.get(this, NiPSysEmitterCtlr::m_pkModifier);
        e.mem.set_f32(emitter + 0x48, 1000.0);
        let scaled: Scaled = Rc::new(RefCell::new(None));
        let wanted = scaled.clone();
        e.register_double(SCALED_TIME, move |_, a| Ret {
            st0: wanted.borrow().unwrap_or(f64::from(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        let active = object(&mut e, 0x30, 1);
        let birth = object(&mut e, 0x30, 1);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.set(this, NiPSysEmitterCtlr::m_spInterpolator, birth);
        e.register_double(SLOT_D8, move |_, _| Ret {
            eax: active,
            ..Ret::default()
        });
        e.register_double(GET_BIRTH_RATE_INTERPOLATOR, move |_, _| Ret {
            eax: birth,
            ..Ret::default()
        });
        (e, this, active, birth, log, scaled)
    }

    fn run_update(e: &mut Engine, this: Ptr<NiPSysEmitterCtlr>, time: f32) {
        let slot = e.mem.alloc(4);
        e.mem.set_f32(slot, time);
        e.call(0x00c1_c630, &args![this, slot]);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_update_stops_early() {
        let (mut e, this, _, _, log, _) = update_engine();
        // No target.
        let target = e.get(base(this), NiTimeController::m_pkTarget);
        e.set(base(this), NiTimeController::m_pkTarget, 0);
        run_update(&mut e, this, 1.0);
        // An inactive controller.
        e.set(base(this), NiTimeController::m_pkTarget, target);
        e.set(base(this), NiTimeController::m_uFlags, 0);
        e.call_log = Some(vec![]);
        run_update(&mut e, this, 1.0);
        assert!(calls_to(&e, SLOT_D8).is_empty());
        assert!(log.borrow().is_empty());
        // A time before the last time resets the last scaled time.
        e.set(base(this), NiTimeController::m_fLastTime, bits(5.0));
        e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, bits(1.0));
        run_update(&mut e, this, 1.0);
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime),
            bits(-f32::MAX)
        );
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_update_with_flag_0x20_only_updates_the_state() {
        let (mut e, this, active, birth, log, _) = update_engine();
        e.set(base(this), NiTimeController::m_uFlags, 0x68);
        e.call_log = Some(vec![]);
        run_update(&mut e, this, 2.0);
        let target = e.get(base(this), NiTimeController::m_pkTarget);
        // The scaled time (marked unknown, then computed) is `t`.
        assert_eq!(
            calls_to(&e, UPDATE_BOOL),
            vec![vec![active, bits(2.0), target, this.addr() + 0x4c]]
        );
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_pkLastBirthRateInterpolator),
            birth
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_update_emits_with_a_posed_rate() {
        let (mut e, this, _, birth, log, _) = update_engine();
        e.register_double(POSED, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(UPDATE_BOOL, |e, a| {
            e.mem.set_u8(a[3], 1);
            Ret::default()
        });
        e.register(UPDATE_FLOAT, |e, a| {
            e.mem.set_f32(a[3], 4.0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(ACTIVE_GET_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, bits(1.0));
        e.set(base(this), NiTimeController::m_fScaledTime, bits(0.0));
        run_update(&mut e, this, 2.0);
        // t is the scaled time (2.0); from 1.0: 4 births at rate 4.
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime), bits(2.0));
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][..2], [bits(101.0), 4]);
        let _ = birth;
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_update_emits_over_keyed_birth_rates() {
        let (mut e, this, active, birth, log, _) = update_engine();
        e.register_double(POSED, move |_, a| Ret {
            eax: (a[0] == active) as u32,
            ..Ret::default()
        });
        e.register(UPDATE_BOOL, |e, a| {
            e.mem.set_u8(a[3], 1);
            Ret::default()
        });
        let samples: Log = Rc::new(RefCell::new(vec![]));
        let seen = samples.clone();
        e.register_double(UPDATE_FLOAT, move |e, a| {
            seen.borrow_mut().push(vec![a[1]]);
            e.mem.set_f32(a[3], 2.0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(ACTIVE_GET_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let keys = e.mem.alloc(0x20);
        e.mem.set_f32(keys, 0.0);
        e.mem.set_f32(keys + 8, 4.0);
        e.register_double(BIRTH_GET_KEYS, move |e, a| {
            e.mem.set_u32(a[1], 2);
            e.mem.set_u8(a[3], 8);
            Ret {
                eax: keys,
                ..Ret::default()
            }
        });
        // The time wrapped: 3.0 -> 1.0 over keys 0 .. 4.
        e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, bits(3.0));
        e.set(base(this), NiTimeController::m_fLastTime, bits(0.0));
        run_update(&mut e, this, 1.0);
        assert_eq!(*samples.borrow(), vec![vec![bits(3.5)], vec![bits(0.5)]]);
        let log = log.borrow();
        assert_eq!(log.len(), 2);
        // 3 .. 4 at rate 2: two births; 0 .. 1: two births, emitted after
        // the first stretch's length (1) plus the target's time.
        assert_eq!(log[0][..2], [bits(101.0), 2]);
        assert_eq!(log[1][..2], [bits(102.0), 2]);
        let _ = birth;
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_update_walks_the_active_keys() {
        let (mut e, this, _, _, log, scaled) = update_engine();
        e.register(POSED, |_, _| Ret::default());
        let keys = e.mem.alloc(0x20);
        // Keys: off at 0, on at 1, off at 3.
        for (i, (time, on)) in [(0.0f32, 0u8), (1.0, 1), (3.0, 0)].iter().enumerate() {
            e.mem.set_f32(keys + 8 * i as u32, *time);
            e.mem.set_u8(keys + 8 * i as u32 + 4, *on);
        }
        e.register_double(ACTIVE_GET_KEYS, move |e, a| {
            e.mem.set_u32(a[1], 3);
            e.mem.set_u32(a[2], 1);
            e.mem.set_u8(a[3], 8);
            Ret {
                eax: keys,
                ..Ret::default()
            }
        });
        e.register(BOOL_KEY_GEN_INTERP, |_, a| {
            let t = f32::from_bits(a[0]);
            Ret {
                eax: (1.0..3.0).contains(&t) as u32,
                ..Ret::default()
            }
        });
        let samples: Log = Rc::new(RefCell::new(vec![]));
        let seen = samples.clone();
        e.register_double(UPDATE_FLOAT, move |e, a| {
            seen.borrow_mut().push(vec![a[1]]);
            e.mem.set_f32(a[3], 2.0);
            Ret::default()
        });
        // First update: no last scaled time yet, only the state is kept.
        e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, bits(-f32::MAX));
        *scaled.borrow_mut() = Some(0.5);
        run_update(&mut e, this, 1.0);
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime), bits(0.5));
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_bLastActive), 0);
        assert!(log.borrow().is_empty());
        // 0.5 -> 4.0: the stretch 1 .. 3 is emitted at rate 2 (sampled at
        // its middle), then the walk ends past the last key.
        *scaled.borrow_mut() = Some(4.0);
        run_update(&mut e, this, 2.0);
        assert_eq!(*samples.borrow(), vec![vec![bits(2.0)]]);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][1], 4);
        assert_eq!(log[0][0], bits(100.0 + 4.0 - 0.5));
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime), bits(4.0));
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test;
    use crate::units::niparticle::nipsysmodifier::{test_stream, INTERLOCKED_DECREMENT};

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An object whose vtable (0x100 bytes of its own) has `slots` pointing
    /// at the given addresses.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x60);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        object
    }

    /// An `NiRTTI` chain: a record at each address of `records`, in order,
    /// ending in a record whose base is null; returns the first record.
    fn rtti_chain(e: &mut Engine, records: &[u32]) -> u32 {
        let mut base = 0;
        for &record in records.iter().rev() {
            e.map(record, 8);
            e.mem.set_u32(record + 4, base);
            base = record;
        }
        base
    }

    fn controller(e: &mut Engine) -> Ptr<NiPSysEmitterCtlr> {
        e.new_object::<NiPSysEmitterCtlr>()
    }

    #[test]
    fn stop_clears_the_last_scaled_time_and_stops_the_controller() {
        let mut e = Engine::new();
        e.map(NO_TIME, 4);
        e.set_global::<u32>(NO_TIME, 0xff7f_ffff);
        e.register(TIME_CONTROLLER_STOP, |_, _| Ret::default());
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c1_ba30, &args![this]);
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime),
            0xff7f_ffff
        );
        assert_eq!(calls_to(&e, TIME_CONTROLLER_STOP), vec![vec![this.addr()]]);
    }

    #[test]
    fn interpolator_ids_are_the_two_names() {
        let mut e = Engine::new();
        e.map(BIRTH_RATE_NAME, 8);
        e.set_global::<u32>(BIRTH_RATE_NAME, 0x0100_0010);
        e.set_global::<u32>(EMITTER_ACTIVE_NAME, 0x0100_0020);
        let id = |e: &mut Engine, index: u32| e.call(0x00c1_ba50, &args![0u32, index]).u32();
        assert_eq!(id(&mut e, 0), 0x0100_0010);
        assert_eq!(id(&mut e, 1), 0x0100_0020);
        assert_eq!(id(&mut e, 2), 0);
        // Only the low 16 bits of the index word count.
        assert_eq!(id(&mut e, 0x0001_0001), 0x0100_0020);
    }

    #[test]
    fn pose_interpolators_are_made_for_the_two_indices() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.register(FLOAT_INTERPOLATOR_WITH_VALUE, |_, a| answer(a[0] + 1));
        e.register(BOOL_INTERPOLATOR_WITH_VALUE, |_, a| answer(a[0] + 2));
        e.call_log = Some(vec![]);
        let made = e.call(0x00c1_bad0, &args![0u32, 0u32]).u32();
        let float = calls_to(&e, FLOAT_INTERPOLATOR_WITH_VALUE);
        assert_eq!(float, vec![vec![made - 1, 0]]);
        let made = e.call(0x00c1_bad0, &args![0u32, 1u32]).u32();
        assert_eq!(
            calls_to(&e, BOOL_INTERPOLATOR_WITH_VALUE),
            vec![vec![made - 2, 0]]
        );
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x18], vec![0x18]]);
        assert_eq!(e.call(0x00c1_bad0, &args![0u32, 2u32]).u32(), 0);
        assert_eq!(calls_to(&e, NI_NEW).len(), 2);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c1_bad0, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00c1_bad0, &args![0u32, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, FLOAT_INTERPOLATOR_WITH_VALUE).len(), 1);
        assert_eq!(calls_to(&e, BOOL_INTERPOLATOR_WITH_VALUE).len(), 1);
    }

    #[test]
    fn guarantee_time_range_reaches_the_emitter_active_interpolator() {
        const GUARANTEE: u32 = 0x0000_20c4;
        let mut e = Engine::new();
        e.register(SINGLE_GUARANTEE_TIME_RANGE, |_, _| Ret::default());
        e.register(GUARANTEE, |_, _| Ret::default());
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        // A signalling NaN start comes through the x87 stack quiet.
        e.call(0x00c1_bbc0, &args![this, 0x7f80_0001u32, 2.0f32]);
        assert_eq!(
            calls_to(&e, SINGLE_GUARANTEE_TIME_RANGE),
            vec![vec![this.addr(), 0x7fc0_0001, 2.0f32.to_bits()]]
        );
        assert!(calls_to(&e, GUARANTEE).is_empty());
        let active = object(&mut e, &[(0xc4, GUARANTEE)]);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.call(0x00c1_bbc0, &args![this, 1.0f32, 2.0f32]);
        assert_eq!(
            calls_to(&e, GUARANTEE),
            vec![vec![active, 1.0f32.to_bits(), 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn reset_time_extrema_widens_the_range_with_the_active_interpolator() {
        const BIRTH_RANGE: u32 = 0x0000_30c0;
        const ACTIVE_RANGE: u32 = 0x0000_31c0;
        let mut e = Engine::new();
        e.register(BIRTH_RANGE, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[2], 2.0);
            Ret::default()
        });
        // The active range comes from the words at 0x0f00_0000.
        e.map(0x0f00_0000, 8);
        e.register(ACTIVE_RANGE, |e, a| {
            let (start, end) = (e.mem.u32(0x0f00_0000), e.mem.u32(0x0f00_0004));
            e.mem.set_u32(a[1], start);
            e.mem.set_u32(a[2], end);
            Ret::default()
        });
        let this = controller(&mut e);
        let birth = object(&mut e, &[(0xc0, BIRTH_RANGE)]);
        let active = object(&mut e, &[(0xc0, ACTIVE_RANGE)]);
        e.set(this, NiPSysEmitterCtlr::m_spInterpolator, birth);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        let range = |e: &mut Engine, start: u32, end: u32| {
            e.mem.set_u32(0x0f00_0000, start);
            e.mem.set_u32(0x0f00_0004, end);
            e.call(0x00c1_bc10, &args![this]);
            let lo = e.mem.u32(this.addr() + 0x14);
            let hi = e.mem.u32(this.addr() + 0x18);
            (lo, hi)
        };
        let (one, two) = (1.0f32.to_bits(), 2.0f32.to_bits());
        assert_eq!(
            range(&mut e, 0.5f32.to_bits(), 3.0f32.to_bits()),
            (0.5f32.to_bits(), 3.0f32.to_bits())
        );
        assert_eq!(
            range(&mut e, 1.5f32.to_bits(), 1.0f32.to_bits()),
            (one, two)
        );
        assert_eq!(range(&mut e, 0x7fc0_0000, 0xffc0_0000), (one, two));
        // Flag 0x20 skips everything.
        e.set(this, NiPSysEmitterCtlr::m_fLastScaledTime, 0);
        e.mem.set_u8(this.addr() + 8, 0x20);
        e.mem.set_u32(this.addr() + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_bc10, &args![this]);
        assert!(calls_to(&e, BIRTH_RANGE).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
    }

    #[test]
    fn register_streamables_adds_the_emitter_active_interpolator() {
        const REGISTER: u32 = 0x0000_4054;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_REGISTER_STREAMABLES, |_, _| answer(0));
        e.register(REGISTER, |_, _| answer(0));
        let this = controller(&mut e);
        let active = object(&mut e, &[(0x54, REGISTER)]);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c1_be10, &args![this, 0x500u32]).bool());
        assert!(calls_to(&e, REGISTER).is_empty());
        e.register(MODIFIER_CTLR_REGISTER_STREAMABLES, |_, _| answer(1));
        assert!(e.call(0x00c1_be10, &args![this, 0x500u32]).bool());
        assert_eq!(calls_to(&e, REGISTER), vec![vec![active, 0x500]]);
        e.set(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator, 0);
        assert!(e.call(0x00c1_be10, &args![this, 0x500u32]).bool());
        assert_eq!(calls_to(&e, REGISTER).len(), 1);
    }

    #[test]
    fn save_binary_saves_the_emitter_active_link() {
        const SAVE_LINK: u32 = 0x0000_5030;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        let this = controller(&mut e);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            0x777,
        );
        let stream = object(&mut e, &[(0x30, SAVE_LINK)]);
        e.call_log = Some(vec![]);
        e.call(0x00c1_be40, &args![this, stream]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x777]]);
    }

    #[test]
    fn is_equal_compares_the_emitter_active_interpolators() {
        const EQUAL: u32 = 0x0000_605c;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_IS_EQUAL, |_, _| answer(1));
        e.register(EQUAL, |_, a| answer(u32::from(a[1] == 0x999)));
        let this = controller(&mut e);
        let other = controller(&mut e);
        let call = |e: &mut Engine| e.call(0x00c1_be60, &args![this, other]).bool();
        assert!(call(&mut e));
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            0x999,
        );
        assert!(!call(&mut e));
        let theirs = object(&mut e, &[(0x5c, EQUAL)]);
        e.set(
            other,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            theirs,
        );
        e.call_log = Some(vec![]);
        assert!(call(&mut e));
        assert_eq!(calls_to(&e, EQUAL), vec![vec![theirs, 0x999]]);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            0x998,
        );
        assert!(!call(&mut e));
        e.set(this, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator, 0);
        assert!(!call(&mut e));
        e.register(MODIFIER_CTLR_IS_EQUAL, |_, _| answer(0));
        e.set(other, NiPSysEmitterCtlr::m_spEmitterActiveInterpolator, 0);
        assert!(!call(&mut e));
    }

    #[test]
    fn replace_keys_makes_bool_data_when_needed() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.register(OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BOOL_DATA_REPLACE_ANIM, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        e.mem.set_u32(this.addr() + 0x14, 0x55);
        e.call_log = Some(vec![]);
        // No keys and no data: nothing.
        e.call(0x00c1_bf10, &args![this, 0x100u32, 2u32, 0u32]);
        assert!(calls_to(&e, BOOL_DATA_REPLACE_ANIM).is_empty());
        e.call(0x00c1_bf10, &args![this, 0x100u32, 2u32, 3u32]);
        let data = e.mem.u32(this.addr() + 0x10);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x18]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCTOR), vec![vec![data]]);
        assert_eq!(e.mem.u32(data), NI_BOOL_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this.addr() + 0x10, data]]
        );
        assert_eq!(
            calls_to(&e, BOOL_DATA_REPLACE_ANIM),
            vec![vec![data, 0x100, 3, 2]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
        // Data already there: no new one. Then no keys empties it.
        e.call(0x00c1_bf10, &args![this, 0x200u32, 1u32, 4u32]);
        assert_eq!(calls_to(&e, NI_NEW).len(), 1);
        e.call(0x00c1_bf10, &args![this, 0x200u32, 1u32, 0u32]);
        let replaced = calls_to(&e, BOOL_DATA_REPLACE_ANIM);
        assert_eq!(replaced[1], vec![data, 0x200, 4, 1]);
        assert_eq!(replaced[2], vec![data, 0, 0, 0]);
    }

    #[test]
    fn replace_keys_assigns_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(NI_NEW, |_, _| Ret::default());
        e.register(BOOL_DATA_REPLACE_ANIM, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        e.call(0x00c1_bf10, &args![this, 0x100u32, 2u32, 3u32]);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this.addr() + 0x10, 0]]
        );
        assert_eq!(
            calls_to(&e, BOOL_DATA_REPLACE_ANIM),
            vec![vec![0, 0x100, 3, 2]]
        );
    }

    /// A target whose modifier lookup (`00c1b0f0`) answers a modifier with
    /// `rtti` (null for 0); the RTTI and the answer live at `0f100000` and
    /// `0f100004` for the tests to change.
    fn modifier_target(e: &mut Engine, rtti: u32) -> u32 {
        const GET_RTTI: u32 = 0x0000_7008;
        e.map(0x0f10_0000, 4);
        e.mem.set_u32(0x0f10_0000, rtti);
        e.register(GET_RTTI, |e, _| answer(e.mem.u32(0x0f10_0000)));
        let modifier = object(e, &[(0x8, GET_RTTI)]);
        e.map(0x0f10_0004, 4);
        e.mem
            .set_u32(0x0f10_0004, if rtti == 0 { 0 } else { modifier });
        e.register(FIND_MODIFIER, |e, _| answer(e.mem.u32(0x0f10_0004)));
        e.mem.alloc(0x10)
    }

    #[test]
    fn target_is_required_type_needs_an_emitter_modifier() {
        let mut e = Engine::new();
        e.register(SINGLE_TARGET_IS_REQUIRED_TYPE, |_, _| answer(1));
        let chain = rtti_chain(&mut e, &[0x0f20_0000, NI_P_SYS_EMITTER_RTTI, 0x0f20_0010]);
        let target = modifier_target(&mut e, chain);
        let this = controller(&mut e);
        e.mem.set_u32(this.addr() + 0x2c, target);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c1_bff0, &args![this]).bool());
        assert_eq!(
            calls_to(&e, FIND_MODIFIER),
            vec![vec![target, this.addr() + 0x38]]
        );
        e.mem.set_u32(0x0f10_0000, 0x0f20_0010);
        assert!(!e.call(0x00c1_bff0, &args![this]).bool());
        e.mem.set_u32(0x0f10_0000, chain);
        e.mem.set_u32(0x0f10_0004, 0);
        assert!(!e.call(0x00c1_bff0, &args![this]).bool());
        e.register(SINGLE_TARGET_IS_REQUIRED_TYPE, |_, _| answer(0));
        assert!(!e.call(0x00c1_bff0, &args![this]).bool());
        assert_eq!(calls_to(&e, FIND_MODIFIER).len(), 3);
    }

    #[test]
    fn interp_target_is_correct_type_needs_an_emitter_modifier() {
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE, |_, _| {
            answer(1)
        });
        let chain = rtti_chain(&mut e, &[0x0f20_0000, NI_P_SYS_EMITTER_RTTI]);
        let target = modifier_target(&mut e, chain);
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_b7d0, &args![this, target]).bool());
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE),
            vec![vec![this.addr(), target]]
        );
        assert_eq!(
            calls_to(&e, FIND_MODIFIER),
            vec![vec![target, this.addr() + 0x38]]
        );
        e.mem.set_u32(0x0f10_0000, 0x0f20_0000 + 0x100);
        e.map(0x0f20_0100, 8);
        assert!(!e.call(0x00c2_b7d0, &args![this, target]).bool());
        e.mem.set_u32(0x0f10_0004, 0);
        assert!(!e.call(0x00c2_b7d0, &args![this, target]).bool());
        e.register(MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE, |_, _| {
            answer(0)
        });
        assert!(!e.call(0x00c2_b7d0, &args![this, target]).bool());
        assert_eq!(calls_to(&e, FIND_MODIFIER).len(), 3);
    }

    #[test]
    fn interpolator_types_follow_the_index() {
        const GET_RTTI: u32 = 0x0000_8008;
        let mut e = Engine::new();
        e.map(0x0f30_0000, 4);
        e.register(GET_RTTI, |e, _| answer(e.mem.u32(0x0f30_0000)));
        e.register(IS_KIND_OF, |_, _| answer(0));
        let interpolator = object(&mut e, &[(0x8, GET_RTTI)]);
        let float = rtti_chain(&mut e, &[0x0f40_0000, NI_FLOAT_INTERPOLATOR_RTTI]);
        let blend = rtti_chain(&mut e, &[0x0f40_0100, BIRTH_RATE_BLEND_RTTI]);
        let boolean = rtti_chain(&mut e, &[NI_BOOL_INTERPOLATOR_RTTI]);
        let other = rtti_chain(&mut e, &[0x0f40_0200]);
        let check = |e: &mut Engine, rtti: u32, object: u32, index: u32| {
            e.mem.set_u32(0x0f30_0000, rtti);
            e.call(0x00c1_c040, &args![0u32, object, index]).bool()
        };
        assert!(check(&mut e, float, interpolator, 0));
        assert!(check(&mut e, blend, interpolator, 0));
        assert!(!check(&mut e, other, interpolator, 0));
        assert!(!check(&mut e, boolean, interpolator, 0));
        assert!(!check(&mut e, float, 0, 0));
        assert!(check(&mut e, boolean, interpolator, 1));
        assert!(!check(&mut e, float, interpolator, 1));
        assert!(!check(&mut e, float, interpolator, 2));
        e.call_log = Some(vec![]);
        assert!(!check(&mut e, float, 0, 1));
        assert_eq!(
            calls_to(&e, IS_KIND_OF),
            vec![vec![NI_BLEND_BOOL_INTERPOLATOR_RTTI, 0]]
        );
        e.register(IS_KIND_OF, |_, _| answer(1));
        assert!(check(&mut e, float, interpolator, 1));
        // A null GetRTTI answer ends a walk.
        assert!(!check(&mut e, 0, interpolator, 0));
    }

    #[test]
    fn named_constructor_makes_and_releases_the_name() {
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        e.map(NULL_FIXED_STRING, 4);
        e.map(NO_TIME, 4);
        e.set_global::<u32>(NO_TIME, 0xff7f_ffff);
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 2);
        e.map(0x0f50_0000, 4);
        e.mem.set_u32(0x0f50_0000, handle);
        e.register(FIXED_STRING_FROM_TEXT, |e, _| {
            answer(e.mem.u32(0x0f50_0000))
        });
        e.map(0x0f50_0004, 4);
        e.register(MODIFIER_CTLR_NAMED_CONSTRUCTOR, |e, a| {
            let name = e.mem.u32(a[1]);
            e.mem.set_u32(0x0f50_0004, name);
            Ret::default()
        });
        let this = controller(&mut e);
        e.set(this, NiPSysEmitterCtlr::m_kParticleAgesSize, 9);
        e.call_log = Some(vec![]);
        let made = e.call(0x00c1_c280, &args![this, 0x0100_0000u32]).u32();
        assert_eq!(made, this.addr());
        assert_eq!(
            calls_to(&e, FIXED_STRING_FROM_TEXT),
            vec![vec![0x0100_0000]]
        );
        assert_eq!(e.mem.u32(0x0f50_0004), handle);
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.mem.u32(this.addr()), NI_P_SYS_EMITTER_CTLR_VTABLE);
        assert_eq!(
            e.get(this, NiPSysEmitterCtlr::m_fLastScaledTime),
            0xff7f_ffff
        );
        assert_eq!(e.get(this, NiPSysEmitterCtlr::m_kParticleAgesSize), 0);
        // A null name passes the null handle, which is not released.
        e.set_global::<u32>(NULL_FIXED_STRING, 0x0f50_0100);
        e.call(0x00c1_c280, &args![this, 0u32]);
        assert_eq!(calls_to(&e, FIXED_STRING_FROM_TEXT).len(), 1);
        assert_eq!(e.mem.u32(0x0f50_0004), 0x0f50_0100);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_active_interpolator() {
        const STRINGS: u32 = 0x0000_9060;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(STRINGS, |_, _| Ret::default());
        viewer_test::counting(&mut e, &[VIEWER_STRING]);
        e.map(NI_P_SYS_EMITTER_CTLR_RTTI, 4);
        e.set_global::<u32>(NI_P_SYS_EMITTER_CTLR_RTTI, 0x0100_0040);
        let strings = viewer_test::strings(&mut e, 8);
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c1_ce70, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1]);
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_0040]]);
        assert!(calls_to(&e, STRINGS).is_empty());
        let active = object(&mut e, &[(0x60, STRINGS)]);
        e.set(
            this,
            NiPSysEmitterCtlr::m_spEmitterActiveInterpolator,
            active,
        );
        e.call(0x00c1_ce70, &args![this, strings]);
        assert_eq!(calls_to(&e, STRINGS), vec![vec![active, strings]]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings], vec![this.addr(), strings]]
        );
    }
}
