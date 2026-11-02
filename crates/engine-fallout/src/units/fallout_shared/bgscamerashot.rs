//! `fallout shared/bgscamerashot.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSCameraShot` (Xbox PDB); the PC `TESForm` is 0x10 smaller, so
    /// every field below sits 0x10 lower than in the PDB.
    pub struct BgsCameraShot: 0x78 {
        /// `Data.eCameraAction` (Xbox PDB, `CAMERA_SHOT_DATA` +0x00).
        0x38 eCameraAction: u32,
        /// `Data.eLocation` (Xbox PDB).
        0x3C eLocation: u32,
        /// `Data.eTarget` (Xbox PDB).
        0x40 eTarget: u32,
        /// `Data.iFlags` (Xbox PDB).
        0x44 iFlags: u32,
        /// `Data.fPlayerTimeMult` (Xbox PDB).
        0x48 fPlayerTimeMult: f32,
        /// `Data.fTargetTimeMult` (Xbox PDB).
        0x4C fTargetTimeMult: f32,
        /// `Data.fGlobalTimeMult` (Xbox PDB).
        0x50 fGlobalTimeMult: f32,
        /// `Data.fMaxTime` (Xbox PDB).
        0x54 fMaxTime: f32,
        /// `Data.fMinTime` (Xbox PDB).
        0x58 fMinTime: f32,
        /// `Data.fTargetPercentBetweenActors` (Xbox PDB).
        0x5C fTargetPercentBetweenActors: f32,
        /// `pTargetRef` (Xbox PDB).
        0x68 pTargetRef: u32,
        /// `bCameraTooClose` (Xbox PDB).
        0x75 bCameraTooClose: u8,
    }
}

/// The `TESModel` base (Xbox PDB +0x28, PC +0x18) and the
/// `TESImageSpaceModifiableForm` base (PC +0x30).
const MODEL: u32 = 0x18;
const IMAGE_SPACE_FORM: u32 = 0x30;
/// The `NiPointer` members (Xbox PDB `spLocationNode`, `spTargetNode`,
/// `spCameraNode`, `spImagespaceTarget`).
const LOCATION_NODE: u32 = 0x60;
const TARGET_NODE: u32 = 0x64;
const CAMERA_NODE: u32 = 0x6c;
const IMAGESPACE_TARGET: u32 = 0x70;

/// `004013e0`: true makes `InitItem` do nothing; `00484ab0`: the flag
/// setter `InitItem` and `Load` end with.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// `"Bad VATS CameraShot Data: '%s' has a Global Time Multiplier that is
/// too small."`
const SMALL_TIME_MULTIPLIER_FORMAT: u32 = 0x0103_2930;
/// The smallest global time multiplier `InitItem` accepts (`double`) and
/// the `float` it puts in its place.
const MIN_GLOBAL_TIME_MULT: u32 = 0x0103_2980;
const DEFAULT_GLOBAL_TIME_MULT: u32 = 0x0101_e2bc;
/// `NiPointer` assignment (`0066b0d0`, `RET 4`) and its raw read
/// (`00559450`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer` destructor (`0045cec0`).
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// `"UIVATSCameraOut"`.
const CAMERA_OUT_SOUND: u32 = 0x0103_2988;
/// The byte that says the VATS camera sound is due when the camera is
/// freed, and the list (`011f2250`) whose count (`005ae380`) must be 0.
const CAMERA_OUT_PENDING: u32 = 0x011c_a729;
const CAMERA_OUT_LIST: u32 = 0x011f_2250;
const LIST_COUNT: u32 = 0x005a_e380;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `memset` (`00403d30`, cdecl, three words).
const MEMSET: u32 = 0x0040_3d30;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 0058be90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058be90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BGSCameraShot::~BGSCameraShot` (Xbox PDB), the body: puts back the
/// three vtables, frees the camera (`BGSCameraShot::FreeCamera`), destroys
/// the `NiPointer` members at +0x70, +0x6C, +0x64 and +0x60, the model
/// (`00489070`) and the `TESForm` base (`00483630`).
pub fn bgs_camera_shot_destructor(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_27f4);
    e.mem.set_u32(base + 0x18, 0x0103_27cc);
    e.mem.set_u32(base + 0x30, 0x0103_27b8);
    bgs_camera_shot_free_camera(e, this);
    for member in [IMAGESPACE_TARGET, CAMERA_NODE, TARGET_NODE, LOCATION_NODE] {
        e.call(NI_POINTER_DESTROY, &args![base + member]);
    }
    e.call(0x0048_9070, &args![base + MODEL]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The camera shot's data defaults (the engine map has no name): action
/// and location 2, target 0, flags 3, global time multiplier 1.0, the
/// other multipliers, times and percentage 0.0.
pub fn fn_0058bf50(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    e.set(this, BgsCameraShot::eCameraAction, 2);
    e.set(this, BgsCameraShot::eLocation, 2);
    e.set(this, BgsCameraShot::eTarget, 0);
    e.set(this, BgsCameraShot::fGlobalTimeMult, 1.0);
    e.set(this, BgsCameraShot::fPlayerTimeMult, 0.0);
    e.set(this, BgsCameraShot::fTargetTimeMult, 0.0);
    e.set(this, BgsCameraShot::iFlags, 3);
    e.set(this, BgsCameraShot::fMaxTime, 0.0);
    e.set(this, BgsCameraShot::fMinTime, 0.0);
    e.set(this, BgsCameraShot::fTargetPercentBetweenActors, 0.0);
}

// Translated from 0058c090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058c090 - the /GS stack cookie check (00ec408c) is not translated.
/// The camera shot's record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the form type (all of `EAX`) is 0x5B;
/// otherwise `TESForm::LoadForm` and each chunk: `EDID` (vtable +0x134),
/// `OBND` (vtable +0xE0), `MODL`/`MODT` (`TESModel::LoadModelChunk`),
/// `MNAM` (the image space modifier's form id, `006ecd40` on +0x30) and
/// `DATA`: a 0x20-byte record (the older form, `TESForm::LoadData`, swapped
/// by `0058c2e0`) spread into the data without the target time multiplier
/// and the percentage, or the 0x28-byte data read in place (swapped by
/// `0058c3c0`). Returns true.
pub fn fn_0058c090(e: &mut Engine, this: Ptr<BgsCameraShot>, file: Ptr) -> bool {
    if e.call(FILE_FORM_TYPE, &args![file]).u32() != 0x5b {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODT => {
                let model = if this.is_null() { 0 } else { base + MODEL };
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_DATA => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 0x20 {
                    let old = e.with_stack(0x20, |e, data| {
                        e.call(MEMSET, &args![data, 0u32, 0x20u32]);
                        e.call(0x0048_61f0, &args![this, file, data, 0x20u32]);
                        if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                            e.call(0x0058_c2e0, &args![data]);
                        }
                        let mut words = [0u32; 8];
                        for (i, word) in words.iter_mut().enumerate() {
                            *word = e.mem.u32(data.addr() + 4 * i as u32);
                        }
                        words
                    });
                    e.set(this, BgsCameraShot::eCameraAction, old[0]);
                    e.set(this, BgsCameraShot::eLocation, old[1]);
                    e.set(this, BgsCameraShot::eTarget, old[2]);
                    e.set(this, BgsCameraShot::iFlags, old[3]);
                    // FLD/FSTP through the x87 stack.
                    let floats = [
                        BgsCameraShot::fPlayerTimeMult,
                        BgsCameraShot::fGlobalTimeMult,
                        BgsCameraShot::fMaxTime,
                        BgsCameraShot::fMinTime,
                    ];
                    for (field, word) in floats.into_iter().zip(&old[4..]) {
                        e.set(this, field, f32::from_bits(x87_quiet_bits(*word)));
                    }
                } else {
                    e.call(MEMSET, &args![base + 0x38, 0u32, 0x28u32]);
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, base + 0x38, 0x28u32]);
                    if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                        e.call(0x0058_c3c0, &args![base + 0x38]);
                    }
                }
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_MNAM => {
                let id = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.call(0x006e_cd40, &args![base + IMAGE_SPACE_FORM, id]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0058c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCameraShot::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// `TESImageSpaceModifiableForm::InitItem`, then a global time multiplier
/// below the minimum (`double` at `01032980`; a NaN passes) is replaced by
/// the `float` at `0101e2bc` after a warning with the shot's name (vtable
/// +0x130), and `00484ab0(1)`.
pub fn bgs_camera_shot_init_item(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    e.call(0x0048_7490, &args![this.addr() + IMAGE_SPACE_FORM, this]);
    let multiplier = e.get(this, BgsCameraShot::fGlobalTimeMult);
    let minimum = e.global::<f64>(MIN_GLOBAL_TIME_MULT);
    // FCOMP; FNSTSW; TEST AH,5; JP: the warning runs only when below.
    if f64::from(multiplier) < minimum {
        let name = e.vcall(this.addr(), 0x130, &args![]).u32();
        e.call(WARNING, &args![SMALL_TIME_MULTIPLIER_FORMAT, name]);
        let default = e.global::<f32>(DEFAULT_GLOBAL_TIME_MULT);
        e.set(this, BgsCameraShot::fGlobalTimeMult, x87_quiet(default));
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 0058ce50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ce50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BGSCameraShot::FreeCamera` (Xbox PDB): clears the location and target
/// nodes, the target reference, the image space target and the too-close
/// flag; a camera node is cleared and the model's +0x14 slot result given
/// to `0045a5e0` on the object at `011c3b3c`. When the camera-out sound is
/// due and the list at `011f2250` is empty, it is played
/// (`BSAudio::GetSoundHandleByName` `"UIVATSCameraOut"` with flags 0x121,
/// `BSSoundHandle::Play`) and no longer due.
pub fn bgs_camera_shot_free_camera(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    let base = this.addr();
    e.call(NI_POINTER_ASSIGN, &args![base + LOCATION_NODE, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + TARGET_NODE, 0u32]);
    e.set(this, BgsCameraShot::pTargetRef, 0);
    e.call(NI_POINTER_ASSIGN, &args![base + IMAGESPACE_TARGET, 0u32]);
    e.set(this, BgsCameraShot::bCameraTooClose, 0);
    if e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() != 0 {
        e.call(NI_POINTER_ASSIGN, &args![base + CAMERA_NODE, 0u32]);
        let value = e.vcall(base + MODEL, 0x14, &args![]).u32();
        let target = e.global::<u32>(0x011c_3b3c);
        e.call(0x0045_a5e0, &args![target, value]);
    }
    if e.global::<u8>(CAMERA_OUT_PENDING) != 0
        && e.call(LIST_COUNT, &args![CAMERA_OUT_LIST]).u32() == 0
    {
        e.set_global(CAMERA_OUT_PENDING, 0u8);
        e.with_stack(0xc, |e, handle| {
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.call(
                0x00ad_7550,
                &args![audio, handle, CAMERA_OUT_SOUND, 0x121u32],
            );
            e.call(0x00ad_8830, &args![handle, 0u32]);
            e.call(0x0048_3710, &args![handle]);
        });
    }
}

/// `0048cee0` (`thiscall` on a `TESModel`: its file name's length, 0 without one).
const MODEL_NAME_LENGTH: u32 = 0x0048_cee0;

// Translated from 0058d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCameraShot::IsInitialized` (engine map; `ECX` only, the answer in `AL`): false for
/// camera action 3; true when the shot has no model file name; otherwise whether the
/// camera node (`NiPointer` at +0x6C) is set.
pub fn bgs_camera_shot_is_initialized(e: &mut Engine, this: Ptr<BgsCameraShot>) -> bool {
    if e.get(this, BgsCameraShot::eCameraAction) == 3 {
        return false;
    }
    let base = this.addr();
    if e.call(MODEL_NAME_LENGTH, &args![base + MODEL]).u32() == 0 {
        return true;
    }
    e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() != 0
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors of `TESForm` (`01183028`) and `BGSCameraShot` (`01186334`),
/// read from their RTTI names.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_DESCRIPTOR: u32 = 0x0118_3028;
const CAMERA_SHOT_DESCRIPTOR: u32 = 0x0118_6334;
/// `00401080` (cdecl, two words): reverses the bytes of the word at its
/// first argument; the second is a zero byte the body does not read.
const SWAP_WORD: u32 = 0x0040_1080;
/// `00401460`: `memcpy` (cdecl, three words); `00ec4835`: `_memcmp`.
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// `00401500`: the byte at `011c54ba` (whether saved data is byte-swapped).
const SAVE_SWAPS_ENDIAN: u32 = 0x0040_1500;
/// `0047b4d0` (`RET 4`), the flag test the shot's `0058cae0` passes 0x20 to.
const FLAG_TEST: u32 = 0x0047_b4d0;
/// `ModelLoader::LoadFile` (`00447080`, `RET 0x18`) on the loader at
/// `011c3b3c`.
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// `0045bc00` (`RET 4`): a node's child (the first, for 0) as a camera.
const CHILD_AS_CAMERA: u32 = 0x0045_bc00;
/// `00653270` (cdecl, two words): `NiRTTI` check of an object against the
/// record at the first argument (the `NiCamera` record `011f4a18`, and the
/// controller records `011f3e50`, `011f3e48`).
const RTTI_CAST: u32 = 0x0065_3270;
const CAMERA_RTTI: u32 = 0x011f_4a18;
/// `"DEFAULT: VATS Camera '%s' doesn't have an NiCamera as the first child. ..."`.
const NO_CAMERA_FORMAT: u32 = 0x0103_2a48;
/// `"DEFAULT: VATS Camera '%s' doesn't have any position data."`.
const NO_POSITION_FORMAT: u32 = 0x0103_2a08;
/// `"DEFAULT: VATS Camera '%s' has a bad FOV that was not created for a 4x3
/// aspect ratio."`.
const BAD_FOV_FORMAT: u32 = 0x0103_29a8;
/// The smallest frustum ratio (`double`, 0.74) that passes without warning.
const MIN_FRUSTUM_RATIO: u32 = 0x0103_2a00;
/// `"UIVATSCameraIn"`.
const CAMERA_IN_SOUND: u32 = 0x0103_2998;
/// The VATS camera globals: the byte that says a timed blend runs, its
/// start (`float`), its fraction and its length.
const BLEND_ACTIVE: u32 = 0x011c_a728;
const BLEND_START: u32 = 0x011c_a72c;
const BLEND_FRACTION: u32 = 0x011c_a730;
const BLEND_LENGTH: u32 = 0x011c_a734;
/// The `NiPoint3`-sized globals the camera code copies: the zero point
/// (`011f426c`), the blend's start and offset (`011ca75c`, `011ca7a8`),
/// the camera position (`011ca77c`), the Euler angles (`011ca738`) and the
/// camera's world position (`011e0808`).
const ZERO_POINT: u32 = 0x011f_426c;
const BLEND_FROM: u32 = 0x011c_a75c;
const BLEND_OFFSET: u32 = 0x011c_a7a8;
const CAMERA_POSITION: u32 = 0x011c_a77c;
const EULER_ANGLES: u32 = 0x011c_a738;
const CAMERA_WORLD: u32 = 0x011e_0808;
/// `00403e20` (thiscall, no words): the address of a setting's `float`.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// The player character (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `0.0` (`double`, `01012060`) and `1.0` (`double`, `01012070`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `00639b20` (thiscall, the answer in `ST0`) and the VATS object
/// (`011f2250`) with its `009c9fe0` (`RET 8`: out point, `float`).
const SHOT_DISTANCE: u32 = 0x0063_9b20;
const VATS_POINT: u32 = 0x009c_9fe0;
/// `0045bb80` (a node's world translation), `006a9540` (its world
/// rotation), `NiMatrix3::ToEulerAnglesXYZ` (`00a592c0`, `RET 0xC`).
const WORLD_TRANSLATE: u32 = 0x0045_bb80;
const WORLD_ROTATE: u32 = 0x006a_9540;
const TO_EULER_ANGLES: u32 = 0x00a5_92c0;
/// `0043d410` (`RET 0xC`: a `float`, two words): the update data on the
/// stack that `00a59c60` (`RET 4`) runs the camera node's update with;
/// `0043c490` the node's world translation.
const UPDATE_DATA: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const NODE_TRANSLATE: u32 = 0x0043_c490;
/// `004390c0` (`RET 4`): whether a point equals the one at its argument.
const POINT_EQUALS: u32 = 0x0043_90c0;
/// `0045bbe0`: a camera's frustum (seven `float`s).
const CAMERA_FRUSTUM: u32 = 0x0045_bbe0;
/// `NiCamera::SetViewFrustum` (`00a6faf0`, `RET 4`).
const SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `NiMatrix3::TransformVertices` (`00a582f0`, cdecl, five words) and
/// `TESObjectREFR::GetOrientation` (`0056fa00`, `RET 4`).
const TRANSFORM_VERTICES: u32 = 0x00a5_82f0;
const GET_ORIENTATION: u32 = 0x0056_fa00;
/// The `NiPoint3` difference (`00439ef0`, `RET 8`), scale (`0045bb20`,
/// `RET 8`), sum (`00439e90`, `RET 8`) and length (`00457990`, `ST0`).
const POINT_SUB: u32 = 0x0043_9ef0;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_LENGTH: u32 = 0x0045_7990;

/// Plays the sound named by `name` (flags 0x121) as `InitCamera` and the
/// update do: `BSAudio::QInstance`, `GetSoundHandleByName` into a handle on
/// the stack, `BSSoundHandle::Play(0)` and the handle's destructor.
fn play_camera_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(0x0045_3a70, &args![]).u32();
        e.call(0x00ad_7550, &args![audio, handle, name, 0x121u32]);
        e.call(0x00ad_8830, &args![handle, 0u32]);
        e.call(0x0048_3710, &args![handle]);
    });
}

/// The three words of a `NiPoint3` at `from`, written to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 0058bfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The camera shot's copy (`RET 4`: the source form; the engine map has no
/// name): when the source casts to a `BGSCameraShot`,
/// `TESForm::CopyAllComponents` and the 0x28 data bytes at +0x38.
pub fn fn_0058bfc0(e: &mut Engine, this: Ptr<BgsCameraShot>, source: u32) {
    let shot = e
        .call(
            DYNAMIC_CAST,
            &args![
                source,
                0u32,
                TES_FORM_DESCRIPTOR,
                CAMERA_SHOT_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    if shot == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, shot]);
    e.call(MEMCPY, &args![this.addr() + 0x38, shot + 0x38, 0x28u32]);
}

// Translated from 0058c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The camera shot's compare (`RET 4`: the other form; the engine map has
/// no name): true (different) unless the other form casts to a
/// `BGSCameraShot`, `TESForm::CompareAllComponents` finds no difference and
/// the 0x28 data bytes match.
pub fn fn_0058c020(e: &mut Engine, this: Ptr<BgsCameraShot>, other: u32) -> bool {
    let shot = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_FORM_DESCRIPTOR,
                CAMERA_SHOT_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    if shot == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, shot]).bool() {
        return true;
    }
    e.call(MEMCMP, &args![this.addr() + 0x38, shot + 0x38, 0x28u32])
        .u32()
        != 0
}

// Translated from 0058c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the older 0x20-byte camera shot data (`ECX`: the data; the
/// engine map has no name), word by word in the order +0x10, +0x14, +0x00,
/// +0x04, +0x08, +0x0C, +0x18, +0x1C.
pub fn fn_0058c2e0(e: &mut Engine, data: Ptr) {
    for offset in [0x10u32, 0x14, 0, 4, 8, 0xc, 0x18, 0x1c] {
        e.call(SWAP_WORD, &args![data.addr() + offset, 0u32]);
    }
}

// Translated from 0058c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the 0x28-byte camera shot data (`ECX`: the data; the engine
/// map has no name), word by word in the order +0x10, +0x14, +0x18, +0x00,
/// +0x04, +0x08, +0x0C, +0x1C, +0x20, +0x24.
pub fn fn_0058c3c0(e: &mut Engine, data: Ptr) {
    for offset in [0x10u32, 0x14, 0x18, 0, 4, 8, 0xc, 0x1c, 0x20, 0x24] {
        e.call(SWAP_WORD, &args![data.addr() + offset, 0u32]);
    }
}

// Translated from 0058c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The camera shot's save (the engine map has no name):
/// `TESForm::StartForm`, `TESModel::Save` (`MODL`, `MODT`), the 0x28 data
/// bytes (`TESForm::__SaveData`) with the data swapped before and after
/// when saving swaps bytes, the image space modifier (`00487450` on +0x30)
/// and `TESForm::CloseForm`.
pub fn fn_0058c4d0(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_91d0, &args![base + MODEL, CHUNK_MODL, CHUNK_MODT]);
    if e.call(SAVE_SWAPS_ENDIAN, &args![]).bool() {
        fn_0058c3c0(e, Ptr::new(base + 0x38));
    }
    e.call(0x0048_5e20, &args![this, base + 0x38, 0x28u32]);
    if e.call(SAVE_SWAPS_ENDIAN, &args![]).bool() {
        fn_0058c3c0(e, Ptr::new(base + 0x38));
    }
    e.call(0x0048_7450, &args![base + IMAGE_SPACE_FORM]);
    e.call(0x0048_5680, &args![this]);
}

// Translated from 0058c5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058c5d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BGSCameraShot::InitCamera` (Xbox PDB; `RET 8`: the target reference
/// and the blend mode). With a model file name the camera node (+0x6C) is
/// loaded (`ModelLoader::LoadFile`). Its first child must be an `NiCamera`
/// (else a warning, the node cleared and the model given to `0045a5e0`).
/// With a node: one update at the time at `011f21d4` (and, with flag 0x20,
/// `004b70f0` with the negated time first), the ±1 "too close" byte (+0x74)
/// settled from the node's controller when +0x76 is clear, warnings for no
/// position data and for a frustum ratio below 0.74. Then the target is
/// stored, `011e07c3` set, the blend mode picks the blend (0: on with the
/// setting at `011ca79c`; 1: off; 2 on action 3: on with the shot's minimum
/// time when above 0, else the setting), the blend start and fraction
/// cleared, the blend points reset, the VATS camera sounds played as the
/// current action needs and `PlayerCharacter::Setup3rdPersonCameraCaster`.
pub fn bgs_camera_shot_init_camera(
    e: &mut Engine,
    this: Ptr<BgsCameraShot>,
    target: u32,
    blend_mode: u32,
) {
    let base = this.addr();
    if e.call(MODEL_NAME_LENGTH, &args![base + MODEL]).u32() != 0 {
        let name = e.vcall(base + MODEL, 0x14, &args![]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        let node = e
            .call(
                MODEL_LOADER_LOAD_FILE,
                &args![loader, name, 0u32, 1u32, 1u32, 0u32, 0u32],
            )
            .u32();
        e.call(NI_POINTER_ASSIGN, &args![base + CAMERA_NODE, node]);
    }
    let camera = if e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() == 0 {
        0
    } else {
        let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
        let child = e.call(CHILD_AS_CAMERA, &args![node, 0u32]).u32();
        e.call(RTTI_CAST, &args![CAMERA_RTTI, child]).u32()
    };
    if camera == 0 {
        let name = e.vcall(base + MODEL, 0x14, &args![]).u32();
        e.call(WARNING, &args![NO_CAMERA_FORMAT, name]);
        e.call(NI_POINTER_ASSIGN, &args![base + CAMERA_NODE, 0u32]);
        let name = e.vcall(base + MODEL, 0x14, &args![]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(0x0045_a5e0, &args![loader, name]);
    }
    if e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() != 0 {
        let time = e.global::<u32>(0x011f_21d4);
        e.with_stack(0xc, |e, update| {
            e.call(
                UPDATE_DATA,
                &args![update, x87_quiet_bits(time), 1u32, 0u32],
            );
            if fn_0058cac0(e, this) {
                // FLD; FCHS; FSTP: the time negated.
                let negated = x87_quiet_bits(time) ^ 0x8000_0000;
                let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
                e.call(0x004b_70f0, &args![node, negated]);
            }
            let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
            e.call(NODE_UPDATE, &args![node, update]);
        });
        if e.mem.u8(base + 0x74) as i8 == -1 && e.mem.u8(base + 0x76) == 0 {
            let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
            let controller = e.call(0x00a5_c570, &args![node, 0x011f_3714u32]).u32();
            if controller != 0 {
                let interpolator = e.vcall(controller, 0xc4, &args![0u32]).u32();
                let first = e
                    .call(RTTI_CAST, &args![0x011f_3e50u32, interpolator])
                    .u32();
                let mut second = 0;
                if first == 0 {
                    second = e
                        .call(RTTI_CAST, &args![0x011f_3e48u32, interpolator])
                        .u32();
                } else {
                    let found = e.with_stack(0xc, |e, out| {
                        let word = out.addr();
                        e.call(0x0058_cb00, &args![first, word, word + 4, word + 0xb]);
                        e.mem.u32(word)
                    });
                    if found != 0 {
                        e.mem.set_u8(base + 0x74, 1);
                    }
                }
                if second != 0 && fn_0058cae0(e, Ptr::new(second)) as u8 != 0 {
                    e.mem.set_u8(base + 0x74, 1);
                }
                if e.mem.u8(base + 0x74) as i8 == -1 {
                    e.mem.set_u8(base + 0x74, 0);
                }
            }
        }
        let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
        let position = e.call(NODE_TRANSLATE, &args![node]).u32();
        if e.call(POINT_EQUALS, &args![position, ZERO_POINT]).bool() {
            let name = e.vcall(base + MODEL, 0x14, &args![]).u32();
            e.call(WARNING, &args![NO_POSITION_FORMAT, name]);
        }
        let frustum = e.call(CAMERA_FRUSTUM, &args![camera]).u32();
        let right = e.mem.u32(frustum + 4);
        let top = e.mem.u32(frustum + 8);
        let ratio = x87_div(x87_load32(top), x87_m32(right));
        // FCOMP; FNSTSW; TEST AH,5; JP: the warning runs only when below.
        if ratio < e.global::<f64>(MIN_FRUSTUM_RATIO) {
            let name = e.vcall(base + MODEL, 0x14, &args![]).u32();
            e.call(WARNING, &args![BAD_FOV_FORMAT, name]);
        }
    }
    e.set(this, BgsCameraShot::pTargetRef, target);
    e.set_global(0x011e_07c3u32, 1u8);
    match blend_mode {
        0 => {
            e.set_global(BLEND_ACTIVE, 1u8);
            let length = e.call(SETTING_FLOAT, &args![0x011c_a79cu32]).u32();
            let value = e.mem.u32(length);
            e.set_global(BLEND_LENGTH, x87_quiet_bits(value));
        }
        1 => e.set_global(BLEND_ACTIVE, 0u8),
        2 if e.get(this, BgsCameraShot::eCameraAction) == 3 => {
            e.set_global(BLEND_ACTIVE, 1u8);
            let min_time = e.mem.u32(base + 0x58);
            // FCOMP; FNSTSW; TEST AH,0x41; JNZ: the setting when not above 0
            // (NaN included).
            if x87_load32(min_time) > e.global::<f64>(ZERO_DOUBLE) {
                e.set_global(BLEND_LENGTH, x87_quiet_bits(min_time));
            } else {
                let length = e.call(SETTING_FLOAT, &args![0x011c_a79cu32]).u32();
                let value = e.mem.u32(length);
                e.set_global(BLEND_LENGTH, x87_quiet_bits(value));
            }
        }
        _ => {}
    }
    e.set_global(BLEND_START, 0.0f32);
    e.set_global(BLEND_FRACTION, 0.0f32);
    copy_point(e, BLEND_OFFSET, ZERO_POINT);
    copy_point(e, BLEND_FROM, ZERO_POINT);
    let action = e.call(0x009c_71c0, &args![CAMERA_OUT_LIST]).u32();
    let wants_in = action != 0 && e.global::<u8>(CAMERA_OUT_PENDING) == 0 && {
        let kind = e.mem.u32(action + 0x14);
        e.call(0x0058_cba0, &args![kind, 4u32]).bool()
            || e.call(0x0058_cba0, &args![kind, 8u32]).bool()
            || e.call(0x0058_cba0, &args![kind, 0x10u32]).bool()
    };
    if wants_in {
        if e.get(this, BgsCameraShot::eCameraAction) == 2 {
            e.set_global(CAMERA_OUT_PENDING, 1u8);
            play_camera_sound(e, CAMERA_IN_SOUND);
        }
    } else if e.global::<u8>(CAMERA_OUT_PENDING) != 0
        && e.get(this, BgsCameraShot::eCameraAction) != 2
    {
        e.set_global(CAMERA_OUT_PENDING, 0u8);
        play_camera_sound(e, CAMERA_OUT_SOUND);
    }
    let player = e.global::<u32>(PLAYER);
    e.call(0x0095_edf0, &args![player]);
}

// Translated from 0058cac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the shot's flags (+0x44) have bit 0x20 (the engine map has no
/// name).
pub fn fn_0058cac0(e: &mut Engine, this: Ptr<BgsCameraShot>) -> bool {
    e.get(this, BgsCameraShot::iFlags) & 0x20 != 0
}

// Translated from 0058cae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0047b4d0(0x20)` on `ECX` (the engine map has no name); the callee's
/// `EAX` is the answer.
pub fn fn_0058cae0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FLAG_TEST, &args![this, 0x20u32]).u32()
}

// Translated from 0058cf60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058cf60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The camera shot's per-frame update (`RET 4`: the time, a `float`; the
/// engine map has no name). Without a camera node nothing. Otherwise the
/// node is updated at that time and its translation kept (`011ca77c` and a
/// local point). When the shot is past its first frame (+0x75) and the
/// player's vtable +0x22C answers false, the player goes first person
/// (`PlayerCharacter::SetFirstPerson(1)`). Otherwise the point is moved
/// into the world: by the orientation of the target or of the reference at
/// `011f21f0` (flag 1: the target's when flag 4 is clear, there is a target
/// and the location is 2) or by the location node's rotation; with a location node the blend
/// start is taken on the first frame, the point is offset by the location
/// node's translation (`0063c8a0`, `0094a0c0` on the player) and becomes
/// the camera's world position, the timed blend (`011ca728`) runs or ends
/// (sound "UIVATSCameraIn" on its first frame) and the camera's frustum is
/// scaled to the screen's aspect ratio over 4/3. Last, the shot counts as
/// past its first frame once the camera is no farther than the setting at
/// `011ca7d0` from the shot's point (`0058d510`).
pub fn fn_0058cf60(e: &mut Engine, this: Ptr<BgsCameraShot>, time: u32) {
    let base = this.addr();
    if e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() == 0 {
        return;
    }
    e.with_stack(0xc, |e, update| {
        e.call(
            UPDATE_DATA,
            &args![update, x87_quiet_bits(time), 1u32, 0u32],
        );
        let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
        e.call(NODE_UPDATE, &args![node, update]);
    });
    let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
    let translate = e.call(NODE_TRANSLATE, &args![node]).u32();
    copy_point(e, CAMERA_POSITION, translate);
    e.with_stack(0xc, |e, point| {
        let point = point.addr();
        copy_point(e, point, CAMERA_POSITION);
        let player = e.global::<u32>(PLAYER);
        if e.mem.u8(base + 0x75) != 0 && !e.vcall(player, 0x22c, &args![0u32]).bool() {
            e.call(0x0095_0110, &args![player, 1u32]);
        } else {
            camera_update_world(e, this, point, time);
        }
        let reached = e.with_stack(0x18, |e, scratch| {
            let shot_point = scratch.addr();
            let difference = scratch.addr() + 0xc;
            let at = fn_0058d510(e, this, Ptr::new(shot_point)).addr();
            let offset = e.call(POINT_SUB, &args![at, difference, point]).u32();
            let distance = e.call(POINT_LENGTH, &args![offset]).f64();
            let limit = e.call(SETTING_FLOAT, &args![0x011c_a7d0u32]).u32();
            let limit = e.mem.u32(limit);
            // FCOMP; FNSTSW; TEST AH,1; JNZ: reached unless the setting is
            // below the distance (NaN included).
            !(x87_load32(limit) < distance)
        });
        if reached {
            e.mem.set_u8(base + 0x75, 1);
        }
    });
}

/// `0058cf60`'s middle: the point at `point` moved into the world, the
/// blend and the frustum (see there).
fn camera_update_world(e: &mut Engine, this: Ptr<BgsCameraShot>, point: u32, time: u32) {
    let base = this.addr();
    e.with_stack(0x24, |e, copy| {
        let copy = copy.addr();
        copy_point(e, copy, point);
        // `0058ccd0`: whether the flags (+0x44) have bit 1.
        if e.call(0x0058_ccd0, &args![this]).bool() {
            let by_target = fn_0058d4f0(e, this)
                && e.get(this, BgsCameraShot::pTargetRef) != 0
                && e.get(this, BgsCameraShot::eLocation) == 2;
            let reference = if by_target {
                e.get(this, BgsCameraShot::pTargetRef)
            } else {
                e.global::<u32>(0x011f_21f0)
            };
            e.with_stack(0x24, |e, matrix| {
                let rotation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
                e.call(
                    TRANSFORM_VERTICES,
                    &args![rotation, ZERO_POINT, 1u32, copy, point],
                );
            });
        } else if e.call(NI_POINTER_GET, &args![base + LOCATION_NODE]).u32() != 0 {
            let node = e.call(NI_POINTER_GET, &args![base + LOCATION_NODE]).u32();
            let rotation = e.call(WORLD_ROTATE, &args![node]).u32();
            e.call(
                TRANSFORM_VERTICES,
                &args![rotation, ZERO_POINT, 1u32, copy, point],
            );
        }
    });
    if e.call(NI_POINTER_GET, &args![base + LOCATION_NODE]).u32() == 0 {
        return;
    }
    let mut first_frame = false;
    if e.call(POINT_EQUALS, &args![BLEND_FROM, ZERO_POINT]).bool() {
        copy_point(e, BLEND_FROM, CAMERA_WORLD);
        first_frame = true;
    }
    e.with_stack(0xc, |e, location| {
        let location = location.addr();
        let node = e.call(NI_POINTER_GET, &args![base + LOCATION_NODE]).u32();
        let translate = e.call(WORLD_TRANSLATE, &args![node]).u32();
        copy_point(e, location, translate);
        e.call(0x0063_c8a0, &args![point, location]);
        let player = e.global::<u32>(PLAYER);
        e.call(0x0094_a0c0, &args![player, point, location, 0u32]);
    });
    copy_point(e, CAMERA_WORLD, point);
    if e.global::<u8>(BLEND_ACTIVE) != 0 {
        camera_blend(e, this, first_frame, time);
    }
    let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
    let camera = e.call(CHILD_AS_CAMERA, &args![node, 0u32]).u32();
    if camera == 0 {
        return;
    }
    let mut own = [0u32; 7];
    let frustum = e.call(CAMERA_FRUSTUM, &args![camera]).u32();
    for (i, word) in own.iter_mut().enumerate() {
        *word = e.mem.u32(frustum + 4 * i as u32);
    }
    let holder = e.call(0x0045_c670, &args![]).u32();
    let target = e.call(0x0066_29f0, &args![holder]).u32();
    let mut scaled = [0u32; 7];
    let frustum = e.call(CAMERA_FRUSTUM, &args![target]).u32();
    for (i, word) in scaled.iter_mut().enumerate() {
        *word = e.mem.u32(frustum + 4 * i as u32);
    }
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    let width = e.call(0x004d_ee10, &args![renderer]).u32();
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    let height = e.call(0x004d_ee70, &args![renderer]).u32();
    // FILD qword (zero-extended); FDIVR; FDIV double [01021790]; FSTP.
    let aspect = x87_div(f64::from(width), f64::from(height));
    let ratio = x87_store32(x87_div(aspect, e.global::<f64>(0x0102_1790)));
    scaled[0] = x87_store32(x87_mul(x87_load32(own[0]), x87_m32(ratio)));
    scaled[1] = x87_store32(x87_mul(x87_load32(own[1]), x87_m32(ratio)));
    scaled[2] = x87_quiet_bits(own[2]);
    scaled[3] = x87_quiet_bits(own[3]);
    e.with_stack(0x1c, |e, frustum| {
        for (i, word) in scaled.iter().enumerate() {
            e.mem.set_u32(frustum.addr() + 4 * i as u32, *word);
        }
        e.call(SET_VIEW_FRUSTUM, &args![target, frustum]);
    });
}

/// `0058cf60`'s timed blend (see there).
fn camera_blend(e: &mut Engine, this: Ptr<BgsCameraShot>, first_frame: bool, time: u32) {
    let end_blend = |e: &mut Engine| {
        e.set_global(BLEND_ACTIVE, 0u8);
        e.set_global(BLEND_START, 0.0f32);
    };
    e.with_stack(0xc, |e, out| {
        let offset = e
            .call(POINT_SUB, &args![CAMERA_WORLD, out, BLEND_FROM])
            .u32();
        copy_point(e, BLEND_OFFSET, offset);
    });
    let distance = e.call(POINT_LENGTH, &args![BLEND_OFFSET]).f64();
    let limit = e.call(SETTING_FLOAT, &args![0x011c_a76cu32]).u32();
    let limit = e.mem.u32(limit);
    // FCOMP; FNSTSW; TEST AH,1; JNZ: the blend goes on only while the
    // setting is below the distance.
    if !(x87_load32(limit) < distance) {
        end_blend(e);
        return;
    }
    if e.get(this, BgsCameraShot::eCameraAction) == 0 {
        end_blend(e);
        return;
    }
    if first_frame && e.global::<u8>(CAMERA_OUT_PENDING) == 0 {
        play_camera_sound(e, CAMERA_IN_SOUND);
    }
    // FLD [734]; FLD1; FDIVRP (DE F1); FLD time; FSUB [72c]; FMULP (DE C9).
    let inverse = x87_div(1.0, x87_load32(e.global::<u32>(BLEND_LENGTH)));
    let elapsed = x87_sub(x87_load32(time), x87_m32(e.global::<u32>(BLEND_START)));
    let fraction = x87_store32(x87_mul(inverse, elapsed));
    e.set_global(BLEND_FRACTION, fraction);
    if !(x87_load32(fraction) < e.global::<f64>(ONE_DOUBLE)) {
        end_blend(e);
        return;
    }
    e.with_stack(0x18, |e, out| {
        let scaled = e
            .call(
                POINT_SCALE,
                &args![BLEND_OFFSET, out, x87_quiet_bits(fraction)],
            )
            .u32();
        let sum = e
            .call(POINT_ADD, &args![BLEND_FROM, out.addr() + 0xc, scaled])
            .u32();
        copy_point(e, CAMERA_WORLD, sum);
    });
}

// Translated from 0058d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the shot's flags (+0x44) lack bit 4 (the engine map has no
/// name).
pub fn fn_0058d4f0(e: &mut Engine, this: Ptr<BgsCameraShot>) -> bool {
    e.get(this, BgsCameraShot::iFlags) & 4 == 0
}

// Translated from 0058d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shot's point (`RET 4`: the `NiPoint3` it fills and returns; the
/// engine map has no name): the zero point, or with a positive `00639b20`
/// the VATS point at that distance (`009c9fe0` on `011f2250`), or else the
/// target node's world translation when there is one. With the "too close"
/// byte (+0x74) at 1 and a camera node, the Euler angles of the camera
/// node's rotation go to `011ca738`; otherwise those three are zeroed.
pub fn fn_0058d510(e: &mut Engine, this: Ptr<BgsCameraShot>, out: Ptr) -> Ptr {
    let base = this.addr();
    let mut point = [0u32; 3];
    for (i, word) in point.iter_mut().enumerate() {
        *word = e.mem.u32(ZERO_POINT + 4 * i as u32);
    }
    let read = |e: &Engine, from: u32| [e.mem.u32(from), e.mem.u32(from + 4), e.mem.u32(from + 8)];
    let distance = e.call(SHOT_DISTANCE, &args![this]).f64();
    // FCOMP; FNSTSW; TEST AH,0x41; JNZ: the VATS point only above 0.
    if distance > e.global::<f64>(ZERO_DOUBLE) {
        let distance = e.call(SHOT_DISTANCE, &args![this]).f64();
        let at = e.with_stack(0xc, |e, local| {
            let at = e
                .call(
                    VATS_POINT,
                    &args![CAMERA_OUT_LIST, local, x87_store32(distance)],
                )
                .u32();
            read(e, at)
        });
        point = at;
    } else if e.call(NI_POINTER_GET, &args![base + TARGET_NODE]).u32() != 0 {
        let node = e.call(NI_POINTER_GET, &args![base + TARGET_NODE]).u32();
        let at = e.call(WORLD_TRANSLATE, &args![node]).u32();
        point = read(e, at);
    }
    let angles =
        e.mem.u8(base + 0x74) == 1 && e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() != 0;
    if angles {
        let node = e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32();
        let rotation = e.call(WORLD_ROTATE, &args![node]).u32();
        e.call(
            TO_EULER_ANGLES,
            &args![rotation, EULER_ANGLES, EULER_ANGLES + 4, EULER_ANGLES + 8],
        );
    } else {
        for i in 0..3 {
            e.set_global(EULER_ANGLES + 4 * i, 0.0f32);
        }
    }
    for (i, word) in point.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_bfc0, fn_0058bfc0(Ptr<BgsCameraShot>, u32)),
        entry!(0x0058_c020, fn_0058c020(Ptr<BgsCameraShot>, u32) -> bool),
        entry!(0x0058_c2e0, fn_0058c2e0(Ptr)),
        entry!(0x0058_c3c0, fn_0058c3c0(Ptr)),
        entry!(0x0058_c4d0, fn_0058c4d0(Ptr<BgsCameraShot>)),
        entry!(
            0x0058_c5d0,
            bgs_camera_shot_init_camera(Ptr<BgsCameraShot>, u32, u32)
        ),
        entry!(0x0058_cac0, fn_0058cac0(Ptr<BgsCameraShot>) -> bool),
        entry!(0x0058_cae0, fn_0058cae0(Ptr) -> u32),
        entry!(0x0058_cf60, fn_0058cf60(Ptr<BgsCameraShot>, u32)),
        entry!(0x0058_d4f0, fn_0058d4f0(Ptr<BgsCameraShot>) -> bool),
        entry!(0x0058_d510, fn_0058d510(Ptr<BgsCameraShot>, Ptr) -> Ptr),
        entry!(
            0x0058_d630,
            bgs_camera_shot_is_initialized(Ptr<BgsCameraShot>) -> bool
        ),
        entry!(0x0058_be90, bgs_camera_shot_destructor(Ptr<BgsCameraShot>)),
        entry!(0x0058_bf50, fn_0058bf50(Ptr<BgsCameraShot>)),
        entry!(0x0058_c090, fn_0058c090(Ptr<BgsCameraShot>, Ptr) -> bool),
        entry!(0x0058_c550, bgs_camera_shot_init_item(Ptr<BgsCameraShot>)),
        entry!(0x0058_ce50, bgs_camera_shot_free_camera(Ptr<BgsCameraShot>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A shot whose own vtable (+0x130, +0x134, +0xE0) and model vtable
    /// (+0x14) slots are doubles.
    fn shot(e: &mut Engine) -> Ptr<BgsCameraShot> {
        let this: Ptr<BgsCameraShot> = e.new_object();
        let vtable = e.mem.alloc(0x140);
        for slot in [0x130u32, 0x134, 0xe0] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this.addr(), vtable);
        let model = e.mem.alloc(0x20);
        e.mem.set_u32(model + 0x14, 0x7100_0014);
        e.mem.set_u32(this.addr() + MODEL, model);
        e.register(0x7000_0130, |_, _| ret(0x5555));
        e.register(0x7100_0014, |_, _| ret(0x6666));
        quiet(e, &[0x7000_0134, 0x7000_00e0]);
        this
    }

    fn free_camera_doubles(e: &mut Engine, camera: u32, pending: u8, listed: u32) {
        e.map(0x011c_3000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.set_global(0x011c_3b3cu32, 0x4444u32);
        e.set_global(CAMERA_OUT_PENDING, pending);
        quiet(
            e,
            &[
                NI_POINTER_ASSIGN,
                0x0045_a5e0,
                0x00ad_7550,
                0x00ad_8830,
                0x0048_3710,
                NI_POINTER_DESTROY,
                0x0048_9070,
                0x0048_3630,
            ],
        );
        e.register_double(NI_POINTER_GET, move |_, _| ret(camera));
        e.register_double(LIST_COUNT, move |_, _| ret(listed));
        e.register(0x0045_3a70, |_, _| ret(0x3333));
    }

    #[test]
    fn free_camera_clears_the_members_and_plays_the_sound() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        e.set(this, BgsCameraShot::pTargetRef, 9);
        e.set(this, BgsCameraShot::bCameraTooClose, 1);
        free_camera_doubles(&mut e, 0x8000, 1, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_ce50, &args![this]);
        let base = this.addr();
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![
                vec![base + 0x60, 0],
                vec![base + 0x64, 0],
                vec![base + 0x70, 0],
                vec![base + 0x6c, 0]
            ]
        );
        assert_eq!(e.get(this, BgsCameraShot::pTargetRef), 0);
        assert_eq!(e.get(this, BgsCameraShot::bCameraTooClose), 0);
        assert_eq!(calls(&e, 0x0045_a5e0), vec![vec![0x4444, 0x6666]]);
        assert_eq!(calls(&e, LIST_COUNT), vec![vec![CAMERA_OUT_LIST]]);
        let handle = calls(&e, 0x00ad_7550);
        assert_eq!(handle.len(), 1);
        assert_eq!(handle[0][0], 0x3333);
        assert_eq!(&handle[0][2..], &[CAMERA_OUT_SOUND, 0x121]);
        assert_eq!(calls(&e, 0x00ad_8830), vec![vec![handle[0][1], 0]]);
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![handle[0][1]]]);
        assert_eq!(e.global::<u8>(CAMERA_OUT_PENDING), 0);
    }

    #[test]
    fn free_camera_without_camera_or_with_listed_entries_skips_both() {
        for (pending, listed) in [(0u8, 0u32), (1, 2)] {
            let mut e = Engine::new();
            let this = shot(&mut e);
            free_camera_doubles(&mut e, 0, pending, listed);
            e.call_log = Some(vec![]);
            e.call(0x0058_ce50, &args![this]);
            assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 3);
            assert!(calls(&e, 0x0045_a5e0).is_empty());
            assert!(calls(&e, 0x00ad_7550).is_empty());
            assert_eq!(e.global::<u8>(CAMERA_OUT_PENDING), pending);
        }
    }

    #[test]
    fn destructor_frees_the_camera_and_destroys_the_members() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        free_camera_doubles(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_be90, &args![this]);
        let base = this.addr();
        assert_eq!(e.mem.u32(base), 0x0103_27f4);
        assert_eq!(e.mem.u32(base + 0x18), 0x0103_27cc);
        assert_eq!(e.mem.u32(base + 0x30), 0x0103_27b8);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 3);
        assert_eq!(
            calls(&e, NI_POINTER_DESTROY),
            vec![
                vec![base + 0x70],
                vec![base + 0x6c],
                vec![base + 0x64],
                vec![base + 0x60]
            ]
        );
        assert_eq!(calls(&e, 0x0048_9070), vec![vec![base + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![base]]);
    }

    #[test]
    fn defaults_fill_the_data() {
        let mut e = Engine::new();
        let this: Ptr<BgsCameraShot> = e.new_object();
        for offset in (0x38..0x60).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call(0x0058_bf50, &args![this]);
        let words: Vec<u32> = (0x38..0x60)
            .step_by(4)
            .map(|offset| e.mem.u32(this.addr() + offset))
            .collect();
        assert_eq!(words, vec![2, 2, 0, 3, 0, 0, 0x3f80_0000, 0, 0, 0]);
    }

    #[test]
    fn init_item_replaces_a_too_small_global_multiplier() {
        for (value, warned) in [(0.00005f32, true), (0.5, false), (f32::NAN, false)] {
            let mut e = Engine::new();
            let this = shot(&mut e);
            e.map(0x0103_2000, 0x1000);
            e.map(0x0101_e000, 0x1000);
            e.set_global(MIN_GLOBAL_TIME_MULT, 0.0001f64);
            e.set_global(DEFAULT_GLOBAL_TIME_MULT, 0.1f32);
            e.set(this, BgsCameraShot::fGlobalTimeMult, value);
            e.register(FORM_INIT_SKIP, |_, _| ret(0));
            quiet(&mut e, &[0x0048_7490, WARNING, FORM_SET_FLAG]);
            e.call_log = Some(vec![]);
            e.call(0x0058_c550, &args![this]);
            assert_eq!(
                calls(&e, 0x0048_7490),
                vec![vec![this.addr() + 0x30, this.addr()]]
            );
            if warned {
                assert_eq!(
                    calls(&e, WARNING),
                    vec![vec![SMALL_TIME_MULTIPLIER_FORMAT, 0x5555]]
                );
                assert_eq!(e.get(this, BgsCameraShot::fGlobalTimeMult), 0.1);
            } else {
                assert!(calls(&e, WARNING).is_empty());
                assert_eq!(
                    e.get(this, BgsCameraShot::fGlobalTimeMult).to_bits(),
                    value.to_bits()
                );
            }
            assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this.addr(), 1]]);
        }
        // Skipped: nothing.
        let mut e = Engine::new();
        let this = shot(&mut e);
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        quiet(&mut e, &[0x0048_7490]);
        e.call_log = Some(vec![]);
        e.call(0x0058_c550, &args![this]);
        assert!(calls(&e, 0x0048_7490).is_empty());
    }

    /// A file whose chunks are `chunks` (type, size, data words).
    fn file(e: &mut Engine, form_type: u32, chunks: Vec<(u32, u32, Vec<u32>)>, swap: bool) -> Ptr {
        let file = Ptr::new(e.mem.alloc(0x10));
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            let c = c.borrow();
            ret(c.0[c.1].1)
        });
        let write = |e: &mut Engine, at: u32, words: &[u32]| {
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, *w);
            }
        };
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            write(e, a[1], &c.0[c.1].2);
            Ret::default()
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            write(e, a[1], &c.0[c.1].2);
            Ret::default()
        });
        let c = chunks;
        e.register_double(0x0048_61f0, move |e, a| {
            let c = c.borrow();
            write(e, a[2], &c.0[c.1].2);
            Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        quiet(
            e,
            &[
                0x0048_5110,
                0x0048_92d0,
                0x006e_cd40,
                0x0058_c2e0,
                0x0058_c3c0,
            ],
        );
        file
    }

    #[test]
    fn load_reads_the_older_data_record() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        e.set(this, BgsCameraShot::fTargetTimeMult, 7.0);
        let data = vec![
            1,
            2,
            3,
            4,
            0x7f80_0001,
            1.5f32.to_bits(),
            2.5f32.to_bits(),
            3.5f32.to_bits(),
        ];
        let f = file(
            &mut e,
            0x5b,
            vec![
                (CHUNK_EDID, 8, vec![0x6f6c]),
                (CHUNK_OBND, 12, vec![]),
                (CHUNK_MODL, 8, vec![]),
                (CHUNK_MODT, 8, vec![]),
                (CHUNK_MNAM, 4, vec![0x1234]),
                (CHUNK_DATA, 0x20, data),
            ],
            true,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_c090, &args![this, f]).bool());
        let base = this.addr();
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![base, f.addr()]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![base, f.addr()]]);
        assert_eq!(calls(&e, 0x0048_92d0), vec![vec![base + 0x18, f.addr()]; 2]);
        assert_eq!(calls(&e, 0x006e_cd40), vec![vec![base + 0x30, 0x1234]]);
        let loaded = calls(&e, 0x0048_61f0);
        assert_eq!(loaded.len(), 1);
        assert_eq!(calls(&e, 0x0058_c2e0), vec![vec![loaded[0][2]]]);
        assert_eq!(e.get(this, BgsCameraShot::eCameraAction), 1);
        assert_eq!(e.get(this, BgsCameraShot::eLocation), 2);
        assert_eq!(e.get(this, BgsCameraShot::eTarget), 3);
        assert_eq!(e.get(this, BgsCameraShot::iFlags), 4);
        // The signalling NaN comes back quiet through FLD/FSTP.
        assert_eq!(e.mem.u32(base + 0x48), 0x7fc0_0001);
        assert_eq!(e.get(this, BgsCameraShot::fTargetTimeMult), 7.0);
        assert_eq!(e.get(this, BgsCameraShot::fGlobalTimeMult), 1.5);
        assert_eq!(e.get(this, BgsCameraShot::fMaxTime), 2.5);
        assert_eq!(e.get(this, BgsCameraShot::fMinTime), 3.5);
    }

    #[test]
    fn load_reads_the_full_data_in_place_and_refuses_other_records() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let f = file(&mut e, 0x5b, vec![(CHUNK_DATA, 0x28, vec![9, 8, 7])], true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_c090, &args![this, f]).bool());
        let base = this.addr();
        assert_eq!(calls(&e, MEMSET), vec![vec![base + 0x38, 0, 0x28]]);
        assert_eq!(
            calls(&e, FILE_CHUNK_DATA_SIZED),
            vec![vec![f.addr(), base + 0x38, 0x28]]
        );
        assert_eq!(calls(&e, 0x0058_c3c0), vec![vec![base + 0x38]]);
        assert_eq!(e.get(this, BgsCameraShot::eTarget), 7);

        // A form type with the right low byte but more in `EAX`: refused.
        let mut e = Engine::new();
        let this = shot(&mut e);
        let f = file(&mut e, 0x015b, vec![(CHUNK_DATA, 0x28, vec![])], false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0058_c090, &args![this, f]).bool());
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn is_initialized_checks_the_action_the_model_and_the_node() {
        let mut e = Engine::new();
        e.register(MODEL_NAME_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let shot: Ptr<BgsCameraShot> = e.new_object();
        let ask = |e: &mut Engine| e.call(0x0058_d630, &args![shot]).bool();
        // No model name: initialized.
        assert!(ask(&mut e));
        // A model without its node: not yet.
        e.mem.set_u32(shot.addr() + MODEL + 4, 5);
        assert!(!ask(&mut e));
        e.mem.set_u32(shot.addr() + CAMERA_NODE, 0x7000);
        assert!(ask(&mut e));
        // Action 3 is never initialized.
        e.set(shot, BgsCameraShot::eCameraAction, 3);
        assert!(!ask(&mut e));
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_point(e: &mut Engine, at: u32, point: [f32; 3]) {
        for (i, v) in point.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [0, 4, 8].map(|o| f32::from_bits(e.mem.u32(at + o)))
    }

    #[test]
    fn copy_takes_the_components_and_the_data_from_a_camera_shot() {
        for cast in [0u32, 0x9000] {
            let mut e = Engine::new();
            let this: Ptr<BgsCameraShot> = e.new_object();
            e.register_double(DYNAMIC_CAST, move |_, _| ret(cast));
            quiet(&mut e, &[0x0048_51b0, MEMCPY]);
            e.call_log = Some(vec![]);
            e.call(0x0058_bfc0, &args![this, 0x5000u32]);
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![
                    0x5000,
                    0,
                    TES_FORM_DESCRIPTOR,
                    CAMERA_SHOT_DESCRIPTOR,
                    0
                ]]
            );
            if cast == 0 {
                assert!(calls(&e, MEMCPY).is_empty());
            } else {
                assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this.addr(), cast]]);
                assert_eq!(
                    calls(&e, MEMCPY),
                    vec![vec![this.addr() + 0x38, cast + 0x38, 0x28]]
                );
            }
        }
    }

    #[test]
    fn compare_is_different_unless_components_and_data_match() {
        // (cast, components differ, memcmp) -> different
        for (cast, differ, cmp, expected) in [
            (0u32, 0u32, 0u32, true),
            (0x9000, 1, 0, true),
            (0x9000, 0, 0xffff_ffff, true),
            (0x9000, 0, 0, false),
        ] {
            let mut e = Engine::new();
            let this: Ptr<BgsCameraShot> = e.new_object();
            e.register_double(DYNAMIC_CAST, move |_, _| ret(cast));
            e.register_double(0x0048_5270, move |_, _| ret(differ));
            e.register_double(MEMCMP, move |_, _| ret(cmp));
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x0058_c020, &args![this, 0x5000u32]).bool(),
                expected
            );
            if cast != 0 && differ == 0 {
                assert_eq!(
                    calls(&e, MEMCMP),
                    vec![vec![this.addr() + 0x38, cast + 0x38, 0x28]]
                );
            }
        }
    }

    #[test]
    fn swaps_visit_every_word_in_the_exe_order() {
        let mut e = Engine::new();
        quiet(&mut e, &[SWAP_WORD]);
        e.call_log = Some(vec![]);
        e.call(0x0058_c2e0, &args![0x1000u32]);
        let order: Vec<u32> = calls(&e, SWAP_WORD).iter().map(|a| a[0] - 0x1000).collect();
        assert_eq!(order, vec![0x10, 0x14, 0, 4, 8, 0xc, 0x18, 0x1c]);
        assert!(calls(&e, SWAP_WORD).iter().all(|a| a[1] == 0));
        e.call_log = Some(vec![]);
        e.call(0x0058_c3c0, &args![0x1000u32]);
        let order: Vec<u32> = calls(&e, SWAP_WORD).iter().map(|a| a[0] - 0x1000).collect();
        assert_eq!(
            order,
            vec![0x10, 0x14, 0x18, 0, 4, 8, 0xc, 0x1c, 0x20, 0x24]
        );
    }

    #[test]
    fn save_writes_the_model_and_the_data_swapped_around_the_write() {
        for swap in [false, true] {
            let mut e = Engine::new();
            let this: Ptr<BgsCameraShot> = e.new_object();
            let base = this.addr();
            quiet(
                &mut e,
                &[
                    0x0048_55a0,
                    0x0048_91d0,
                    0x0048_5e20,
                    0x0048_7450,
                    0x0048_5680,
                    SWAP_WORD,
                ],
            );
            e.register_double(SAVE_SWAPS_ENDIAN, move |_, _| ret(u32::from(swap)));
            e.call_log = Some(vec![]);
            e.call(0x0058_c4d0, &args![this]);
            assert_eq!(calls(&e, 0x0048_55a0), vec![vec![base]]);
            assert_eq!(
                calls(&e, 0x0048_91d0),
                vec![vec![base + 0x18, CHUNK_MODL, CHUNK_MODT]]
            );
            assert_eq!(calls(&e, 0x0048_5e20), vec![vec![base, base + 0x38, 0x28]]);
            assert_eq!(calls(&e, 0x0048_7450), vec![vec![base + 0x30]]);
            assert_eq!(calls(&e, 0x0048_5680), vec![vec![base]]);
            assert_eq!(calls(&e, SWAP_WORD).len(), if swap { 20 } else { 0 });
        }
    }

    #[test]
    fn flag_tests_read_bits_0x20_and_4() {
        let mut e = Engine::new();
        let this: Ptr<BgsCameraShot> = e.new_object();
        e.set(this, BgsCameraShot::iFlags, 0x20);
        assert!(e.call(0x0058_cac0, &args![this]).bool());
        assert!(e.call(0x0058_d4f0, &args![this]).bool());
        e.set(this, BgsCameraShot::iFlags, 4);
        assert!(!e.call(0x0058_cac0, &args![this]).bool());
        assert!(!e.call(0x0058_d4f0, &args![this]).bool());
    }

    #[test]
    fn flag_wrapper_passes_on_the_callee_answer() {
        let mut e = Engine::new();
        e.register(FLAG_TEST, |_, a| ret(0x1200 + a[1]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_cae0, &args![0x4000u32]).u32(), 0x1220);
        assert_eq!(calls(&e, FLAG_TEST), vec![vec![0x4000, 0x20]]);
    }

    /// A shot with a model vtable (+0x14 gives 0x6666) and globals mapped.
    fn shot(e: &mut Engine) -> Ptr<BgsCameraShot> {
        let this: Ptr<BgsCameraShot> = e.new_object();
        let model_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(model_vtable + 0x14, 0x7100_0014);
        e.mem.set_u32(this.addr() + MODEL, model_vtable);
        e.register(0x7100_0014, |_, _| ret(0x6666));
        for page in [
            0x011c_3000u32,
            0x011c_a000,
            0x011d_e000,
            0x011e_0000,
            0x011f_2000,
            0x011f_4000,
            0x0101_2000,
            0x0102_1000,
            0x0103_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        this
    }

    #[test]
    fn shot_point_uses_the_vats_point_or_the_target_node() {
        // Distance above 0: the VATS point; angles off.
        let mut e = Engine::new();
        let this = shot(&mut e);
        put_point(&mut e, ZERO_POINT, [0.0; 3]);
        put_point(&mut e, EULER_ANGLES, [5.0; 3]);
        e.register(SHOT_DISTANCE, |_, _| st0(2.5));
        e.register(VATS_POINT, |e, a| {
            put_point(e, a[1], [1.0, 2.0, 3.0]);
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0058_d510, &args![this, out]).u32(), out);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
        let vats = calls(&e, VATS_POINT);
        assert_eq!(vats[0][0], CAMERA_OUT_LIST);
        assert_eq!(vats[0][2], 2.5f32.to_bits());
        assert_eq!(point(&e, EULER_ANGLES), [0.0; 3]);

        // Distance 0 with a target node; too close with a camera: angles.
        let mut e = Engine::new();
        let this = shot(&mut e);
        put_point(&mut e, ZERO_POINT, [9.0; 3]);
        e.mem.set_u32(this.addr() + TARGET_NODE, 0x8000);
        e.mem.set_u32(this.addr() + CAMERA_NODE, 0x8100);
        e.mem.set_u8(this.addr() + 0x74, 1);
        e.register(SHOT_DISTANCE, |_, _| st0(0.0));
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        let translate = e.mem.alloc(0xc);
        put_point(&mut e, translate, [4.0, 5.0, 6.0]);
        e.register_double(WORLD_TRANSLATE, move |_, _| ret(translate));
        e.register(WORLD_ROTATE, |_, a| ret(a[0] + 1));
        quiet(&mut e, &[TO_EULER_ANGLES]);
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(0xc);
        e.call(0x0058_d510, &args![this, out]);
        assert_eq!(point(&e, out), [4.0, 5.0, 6.0]);
        assert_eq!(calls(&e, WORLD_TRANSLATE), vec![vec![0x8000]]);
        assert_eq!(
            calls(&e, TO_EULER_ANGLES),
            vec![vec![
                0x8101,
                EULER_ANGLES,
                EULER_ANGLES + 4,
                EULER_ANGLES + 8
            ]]
        );

        // Neither: the zero point.
        e.mem.set_u32(this.addr() + TARGET_NODE, 0);
        let out = e.mem.alloc(0xc);
        e.call(0x0058_d510, &args![this, out]);
        assert_eq!(point(&e, out), [9.0; 3]);
    }

    /// Doubles for `InitCamera`'s callees; `camera` is the `NiCamera` cast's
    /// answer.
    fn init_doubles(e: &mut Engine, camera: u32) {
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| ret(0x8800));
        e.register(CHILD_AS_CAMERA, |_, a| ret(a[0] + 0x10));
        e.register_double(RTTI_CAST, move |_, a| {
            ret(if a[0] == CAMERA_RTTI { camera } else { 0 })
        });
        let frustum = e.mem.alloc(0x1c);
        for (i, v) in [0.0f32, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0].iter().enumerate() {
            e.mem.set_u32(frustum + 4 * i as u32, v.to_bits());
        }
        e.register_double(CAMERA_FRUSTUM, move |_, _| ret(frustum));
        e.register(NODE_TRANSLATE, |_, _| ret(0x0aaa));
        e.register(POINT_EQUALS, |_, _| ret(0));
        e.register(0x00a5_c570, |_, _| ret(0));
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 0.75f32.to_bits());
        e.register_double(SETTING_FLOAT, move |_, _| ret(setting));
        e.register(0x009c_71c0, |_, _| ret(0));
        e.register(0x0058_cba0, |_, _| ret(0));
        e.register(0x0045_3a70, |_, _| ret(0x3333));
        quiet(
            e,
            &[
                WARNING,
                0x0045_a5e0,
                UPDATE_DATA,
                NODE_UPDATE,
                0x004b_70f0,
                0x00ad_7550,
                0x00ad_8830,
                0x0048_3710,
                0x0095_edf0,
            ],
        );
        e.set_global(MIN_FRUSTUM_RATIO, 0.74f64);
        e.set_global(0x011f_21d4u32, 0.25f32);
        e.set_global(PLAYER, 0x4321u32);
    }

    #[test]
    fn init_camera_loads_the_node_and_plays_the_in_sound() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        e.register(MODEL_NAME_LENGTH, |_, _| ret(5));
        init_doubles(&mut e, 0x9900);
        let action = e.mem.alloc(0x18);
        e.mem.set_u32(action + 0x14, 0x77);
        e.register_double(0x009c_71c0, move |_, _| ret(action));
        e.register(0x0058_cba0, |_, a| ret(u32::from(a[1] == 8)));
        e.set_global(MODEL_LOADER, 0x1234u32);
        e.set(this, BgsCameraShot::eCameraAction, 2);
        e.set(this, BgsCameraShot::iFlags, 0x20);
        e.set_global(BLEND_START, 3.0f32);
        put_point(&mut e, ZERO_POINT, [1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        e.call(0x0058_c5d0, &args![this, 0x5150u32, 0u32]);
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0x1234, 0x6666, 0, 1, 1, 0, 0]]
        );
        assert_eq!(e.mem.u32(base + CAMERA_NODE), 0x8800);
        assert_eq!(calls(&e, CHILD_AS_CAMERA), vec![vec![0x8800, 0]]);
        assert_eq!(calls(&e, RTTI_CAST)[0], vec![CAMERA_RTTI, 0x8810]);
        let update = calls(&e, UPDATE_DATA);
        assert_eq!(&update[0][1..], &[0.25f32.to_bits(), 1, 0]);
        assert_eq!(
            calls(&e, 0x004b_70f0),
            vec![vec![0x8800, (-0.25f32).to_bits()]]
        );
        assert_eq!(calls(&e, NODE_UPDATE), vec![vec![0x8800, update[0][0]]]);
        // 0.5 / 1.0 is below 0.74: the FOV warning only.
        assert_eq!(calls(&e, WARNING), vec![vec![BAD_FOV_FORMAT, 0x6666]]);
        assert_eq!(e.get(this, BgsCameraShot::pTargetRef), 0x5150);
        assert_eq!(e.global::<u8>(0x011e_07c3), 1);
        assert_eq!(e.global::<u8>(BLEND_ACTIVE), 1);
        assert_eq!(e.global::<f32>(BLEND_LENGTH), 0.75);
        assert_eq!(e.global::<f32>(BLEND_START), 0.0);
        assert_eq!(point(&e, BLEND_FROM), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, BLEND_OFFSET), [1.0, 2.0, 3.0]);
        assert_eq!(calls(&e, 0x0058_cba0), vec![vec![0x77, 4], vec![0x77, 8]]);
        assert_eq!(e.global::<u8>(CAMERA_OUT_PENDING), 1);
        assert_eq!(calls(&e, 0x00ad_7550)[0][2..], [CAMERA_IN_SOUND, 0x121]);
        assert_eq!(calls(&e, 0x0095_edf0), vec![vec![0x4321]]);
    }

    #[test]
    fn init_camera_without_a_camera_warns_and_plays_the_out_sound() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        e.register(MODEL_NAME_LENGTH, |_, _| ret(0));
        init_doubles(&mut e, 0);
        e.set_global(MODEL_LOADER, 0x1234u32);
        e.mem.set_u32(base + CAMERA_NODE, 0x8800);
        e.set(this, BgsCameraShot::eCameraAction, 3);
        e.mem.set_u32(base + 0x58, 2.0f32.to_bits());
        e.set_global(CAMERA_OUT_PENDING, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0058_c5d0, &args![this, 0u32, 2u32]);
        assert!(calls(&e, MODEL_LOADER_LOAD_FILE).is_empty());
        assert_eq!(calls(&e, WARNING), vec![vec![NO_CAMERA_FORMAT, 0x6666]]);
        assert_eq!(calls(&e, 0x0045_a5e0), vec![vec![0x1234, 0x6666]]);
        assert_eq!(e.mem.u32(base + CAMERA_NODE), 0);
        assert!(calls(&e, UPDATE_DATA).is_empty());
        // Mode 2 on action 3 with a positive minimum time.
        assert_eq!(e.global::<u8>(BLEND_ACTIVE), 1);
        assert_eq!(e.global::<f32>(BLEND_LENGTH), 2.0);
        assert_eq!(e.global::<u8>(CAMERA_OUT_PENDING), 0);
        assert_eq!(calls(&e, 0x00ad_7550)[0][2..], [CAMERA_OUT_SOUND, 0x121]);

        // Mode 2 with a NaN minimum time takes the setting; mode 1 turns off.
        for (mode, min_time, active, length) in [(2u32, f32::NAN, 1u8, 0.75f32), (1, 2.0, 0, 9.0)] {
            let mut e = Engine::new();
            let this = shot(&mut e);
            e.register(MODEL_NAME_LENGTH, |_, _| ret(0));
            init_doubles(&mut e, 0);
            e.set(this, BgsCameraShot::eCameraAction, 3);
            e.mem.set_u32(this.addr() + 0x58, min_time.to_bits());
            e.set_global(BLEND_ACTIVE, 7u8);
            e.set_global(BLEND_LENGTH, 9.0f32);
            e.call(0x0058_c5d0, &args![this, 0u32, mode]);
            assert_eq!(e.global::<u8>(BLEND_ACTIVE), active);
            assert_eq!(e.global::<f32>(BLEND_LENGTH), length);
        }
    }

    #[test]
    fn init_camera_settles_the_too_close_byte_from_the_controller() {
        // (first cast, found, second cast, 0058cae0's answer) -> byte
        for (first, found, second, flag, expected) in [
            (0x6000u32, 1u32, 0u32, 0u32, 1u8),
            (0x6000, 0, 0, 0, 0),
            (0, 0, 0x6100, 1, 1),
            (0, 0, 0x6100, 0x100, 0),
        ] {
            let mut e = Engine::new();
            let this = shot(&mut e);
            let base = this.addr();
            e.register(MODEL_NAME_LENGTH, |_, _| ret(0));
            init_doubles(&mut e, 0x9900);
            e.mem.set_u32(base + CAMERA_NODE, 0x8800);
            e.mem.set_u8(base + 0x74, 0xff);
            let controller = e.mem.alloc(8);
            let vtable = e.mem.alloc(0xc8);
            e.mem.set_u32(controller, vtable);
            e.mem.set_u32(vtable + 0xc4, 0x7200_00c4);
            e.register(0x7200_00c4, |_, _| ret(0x6500));
            e.register_double(0x00a5_c570, move |_, _| ret(controller));
            e.register_double(RTTI_CAST, move |_, a| {
                ret(match a[0] {
                    CAMERA_RTTI => 0x9900,
                    0x011f_3e50 => first,
                    _ => second,
                })
            });
            e.register_double(0x0058_cb00, move |e, a| {
                e.mem.set_u32(a[1], found);
                Ret::default()
            });
            e.register_double(FLAG_TEST, move |_, _| ret(flag));
            e.call_log = Some(vec![]);
            e.call(0x0058_c5d0, &args![this, 0u32, 3u32]);
            assert_eq!(e.mem.u8(base + 0x74), expected);
            assert_eq!(calls(&e, 0x00a5_c570), vec![vec![0x8800, 0x011f_3714]]);
            assert_eq!(calls(&e, 0x7200_00c4), vec![vec![controller, 0]]);
            if second != 0 {
                assert_eq!(calls(&e, FLAG_TEST), vec![vec![second, 0x20]]);
            }
        }
    }

    /// Doubles for the update (`0058cf60`).
    fn update_doubles(e: &mut Engine) {
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        let translate = e.mem.alloc(0xc);
        put_point(e, translate, [1.0, 2.0, 3.0]);
        e.register_double(NODE_TRANSLATE, move |_, _| ret(translate));
        let location = e.mem.alloc(0xc);
        put_point(e, location, [10.0, 20.0, 30.0]);
        e.register_double(WORLD_TRANSLATE, move |_, _| ret(location));
        e.register(WORLD_ROTATE, |_, a| ret(a[0] + 2));
        e.register(GET_ORIENTATION, |_, a| ret(a[1]));
        e.register(POINT_EQUALS, |_, _| ret(1));
        e.register(CHILD_AS_CAMERA, |_, _| ret(0));
        // The point difference and sum write their out point.
        e.register(POINT_SUB, |e, a| {
            put_point(e, a[1], [3.0, 4.0, 0.0]);
            ret(a[1])
        });
        e.register(POINT_ADD, |e, a| {
            put_point(e, a[1], [7.0, 7.0, 7.0]);
            ret(a[1])
        });
        e.register(POINT_SCALE, |_, a| ret(a[1]));
        e.register(POINT_LENGTH, |_, _| st0(5.0));
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 1.0f32.to_bits());
        e.register_double(SETTING_FLOAT, move |_, _| ret(setting));
        e.register(SHOT_DISTANCE, |_, _| st0(0.0));
        e.register(0x0058_ccd0, |e, a| ret(e.mem.u32(a[0] + 0x44) & 1));
        e.register(0x0045_3a70, |_, _| ret(0x3333));
        quiet(
            e,
            &[
                UPDATE_DATA,
                NODE_UPDATE,
                TRANSFORM_VERTICES,
                0x0063_c8a0,
                0x0094_a0c0,
                0x0095_0110,
                0x00ad_7550,
                0x00ad_8830,
                0x0048_3710,
                TO_EULER_ANGLES,
            ],
        );
        e.set_global(PLAYER, 0x4321u32);
    }

    #[test]
    fn update_moves_the_camera_and_runs_the_blend() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        update_doubles(&mut e);
        e.mem.set_u32(base + CAMERA_NODE, 0x8800);
        e.mem.set_u32(base + LOCATION_NODE, 0x8a00);
        e.set(this, BgsCameraShot::iFlags, 1);
        e.set(this, BgsCameraShot::pTargetRef, 0x4400);
        e.set(this, BgsCameraShot::eLocation, 2);
        e.set(this, BgsCameraShot::eCameraAction, 1);
        e.set_global(BLEND_ACTIVE, 1u8);
        e.set_global(BLEND_START, 1.0f32);
        e.set_global(BLEND_LENGTH, 4.0f32);
        put_point(&mut e, CAMERA_WORLD, [8.0, 8.0, 8.0]);
        e.call_log = Some(vec![]);
        e.call(0x0058_cf60, &args![this, 3.0f32.to_bits()]);
        assert_eq!(&calls(&e, UPDATE_DATA)[0][1..], &[3.0f32.to_bits(), 1, 0]);
        assert_eq!(point(&e, CAMERA_POSITION), [1.0, 2.0, 3.0]);
        // Flag 1, flag 4 clear, a target and location 2: the target's orientation.
        assert_eq!(calls(&e, GET_ORIENTATION)[0][0], 0x4400);
        let transform = calls(&e, TRANSFORM_VERTICES);
        assert_eq!(&transform[0][1..3], &[ZERO_POINT, 1]);
        // First frame: the blend starts from the old world position.
        assert_eq!(point(&e, BLEND_FROM), [8.0, 8.0, 8.0]);
        let offset = calls(&e, 0x0063_c8a0);
        assert_eq!(offset.len(), 1);
        assert_eq!(
            calls(&e, 0x0094_a0c0),
            vec![vec![0x4321, offset[0][0], offset[0][1], 0]]
        );
        // (3 - 1) / 4 = 0.5: still blending, the sound on the first frame.
        assert_eq!(e.global::<f32>(BLEND_FRACTION), 0.5);
        assert_eq!(e.global::<u8>(BLEND_ACTIVE), 1);
        assert_eq!(calls(&e, POINT_SCALE)[0][2], 0.5f32.to_bits());
        assert_eq!(point(&e, CAMERA_WORLD), [7.0, 7.0, 7.0]);
        assert_eq!(calls(&e, 0x00ad_7550)[0][2], CAMERA_IN_SOUND);
        // The distance 5 is beyond the setting 1: not past the first frame.
        assert_eq!(e.mem.u8(base + 0x75), 0);
    }

    #[test]
    fn update_ends_the_blend_and_marks_the_first_frame_done() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        update_doubles(&mut e);
        e.register(POINT_LENGTH, |_, _| st0(0.5));
        e.mem.set_u32(base + CAMERA_NODE, 0x8800);
        e.mem.set_u32(base + LOCATION_NODE, 0x8a00);
        e.set(this, BgsCameraShot::eCameraAction, 1);
        e.set_global(BLEND_ACTIVE, 1u8);
        e.set_global(BLEND_START, 1.0f32);
        e.set_global(BLEND_LENGTH, 1.0f32);
        e.call_log = Some(vec![]);
        e.call(0x0058_cf60, &args![this, 3.0f32.to_bits()]);
        // Flag 1 clear: the location node's rotation.
        assert_eq!(calls(&e, TRANSFORM_VERTICES)[0][0], 0x8a02);
        // The distance 0.5 is not beyond the setting 1: the blend ends.
        assert_eq!(e.global::<u8>(BLEND_ACTIVE), 0);
        assert_eq!(e.global::<f32>(BLEND_START), 0.0);
        assert!(calls(&e, POINT_SCALE).is_empty());
        assert_eq!(e.mem.u8(base + 0x75), 1);

        // A fraction of 1 or more ends the blend too.
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        update_doubles(&mut e);
        e.mem.set_u32(base + CAMERA_NODE, 0x8800);
        e.mem.set_u32(base + LOCATION_NODE, 0x8a00);
        e.set(this, BgsCameraShot::eCameraAction, 1);
        e.set_global(BLEND_ACTIVE, 1u8);
        e.set_global(BLEND_START, 1.0f32);
        e.set_global(BLEND_LENGTH, 2.0f32);
        e.call_log = Some(vec![]);
        e.call(0x0058_cf60, &args![this, 3.0f32.to_bits()]);
        assert_eq!(e.global::<f32>(BLEND_FRACTION), 1.0);
        assert_eq!(e.global::<u8>(BLEND_ACTIVE), 0);
        assert!(calls(&e, POINT_SCALE).is_empty());

        // Past the first frame with the player's +0x22C false: first person.
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        update_doubles(&mut e);
        e.mem.set_u32(base + CAMERA_NODE, 0x8800);
        e.mem.set_u8(base + 0x75, 1);
        let player = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x230);
        e.mem.set_u32(player, vtable);
        e.mem.set_u32(vtable + 0x22c, 0x7300_022c);
        e.register(0x7300_022c, |_, _| ret(0));
        e.set_global(PLAYER, player);
        e.call_log = Some(vec![]);
        e.call(0x0058_cf60, &args![this, 3.0f32.to_bits()]);
        assert_eq!(calls(&e, 0x0095_0110), vec![vec![player, 1]]);
        assert!(calls(&e, TRANSFORM_VERTICES).is_empty());
        assert_eq!(calls(&e, POINT_LENGTH).len(), 1);

        // Without a camera node: nothing.
        let mut e = Engine::new();
        let this = shot(&mut e);
        update_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0058_cf60, &args![this, 0u32]);
        assert!(calls(&e, UPDATE_DATA).is_empty());
    }

    #[test]
    fn update_scales_the_frustum_to_the_screen() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let base = this.addr();
        update_doubles(&mut e);
        e.register(POINT_EQUALS, |_, _| ret(0));
        e.mem.set_u32(base + CAMERA_NODE, 0x8800);
        e.mem.set_u32(base + LOCATION_NODE, 0x8a00);
        e.register(CHILD_AS_CAMERA, |_, _| ret(0x9900));
        let own = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        for i in 0..7u32 {
            e.mem.set_u32(own + 4 * i, (i as f32 + 1.0).to_bits());
            e.mem.set_u32(other + 4 * i, (i as f32 + 10.0).to_bits());
        }
        e.register_double(CAMERA_FRUSTUM, move |_, a| {
            ret(if a[0] == 0x9900 { own } else { other })
        });
        e.register(0x0045_c670, |_, _| ret(0x5a00));
        e.register(0x0066_29f0, |_, _| ret(0x5b00));
        e.register(0x0043_c4b0, |_, _| ret(0x5c00));
        e.register(0x004d_ee10, |_, _| ret(1600));
        e.register(0x004d_ee70, |_, _| ret(900));
        e.set_global(0x0102_1790u32, 4.0f64 / 3.0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let s = seen.clone();
        e.register_double(SET_VIEW_FRUSTUM, move |e, a| {
            s.borrow_mut()
                .extend((0..7).map(|i| f32::from_bits(e.mem.u32(a[1] + 4 * i))));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_cf60, &args![this, 3.0f32.to_bits()]);
        assert_eq!(calls(&e, SET_VIEW_FRUSTUM)[0][0], 0x5b00);
        let ratio = f64::from(((1600.0f64 / 900.0) / (4.0 / 3.0)) as f32);
        assert_eq!(
            *seen.borrow(),
            vec![
                ratio as f32,
                (2.0 * ratio) as f32,
                3.0,
                4.0,
                14.0,
                15.0,
                16.0
            ]
        );
    }
}
