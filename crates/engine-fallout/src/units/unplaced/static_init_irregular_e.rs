//! `(unplaced)/static_init_irregular_e` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 37 float-setting initializers of FalloutNV.exe
//! 1.4.0.525 that `research/initgen` refused (its helper pattern does not take the
//! `PUSH ECX; FSTP float ptr [ESP]` push of a constructor argument) and start-up skipped, so
//! their settings (the HAVOK, RagdollAnim, LookIK, FootIK, GrabIK, LOD and Imagespace ones)
//! stayed unbuilt. Part 5 of the list. Each translated from its own bytes (Ghidra made no
//! function of them; decoded from `read_memory`, every instruction checked against the shape).
//!
//! Each is `<the float pushed>; PUSH name; MOV ECX,global; CALL 0044f440; PUSH destructor;
//! CALL _atexit; POP ECX; RET`: the float `Setting` constructor (thiscall, `RET 8`) on the
//! global with the name and the float, then the destructor given to `_atexit`. No frame, no
//! `/GS` check. They are cdecl functions with no parameters; `_initterm` ignores `EAX`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;

/// The float `Setting` constructor (`0044f440`, thiscall, `RET 8`: `this`, the name, the float
/// as its bit pattern).
const FLOAT_SETTING_CONSTRUCT: u32 = 0x0044_f440;

/// The float a float-setting initializer pushes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FloatValue {
    /// `FLD float ptr [a]; PUSH ECX; FSTP float ptr [ESP]` (a signalling NaN quieted).
    At(u32),
    /// `FLD1; PUSH ECX; FSTP float ptr [ESP]`.
    One,
}

/// One float-setting initializer: `<the float pushed>; PUSH name; MOV ECX,global; CALL
/// 0044f440; PUSH destructor; CALL _atexit; POP ECX; RET` (the `FLD1` ones load `ECX` before
/// pushing the name; the same calls).
#[derive(Clone, Copy, Debug)]
struct FloatRow {
    global: u32,
    name: u32,
    value: FloatValue,
    destructor: u32,
}

/// The common body of the float-setting initializers.
fn float_setting(e: &mut Engine, row: &FloatRow) {
    let bits = match row.value {
        FloatValue::At(a) => x87_quiet_bits(e.global::<u32>(a)),
        FloatValue::One => 0x3f80_0000,
    };
    e.call(FLOAT_SETTING_CONSTRUCT, &args![row.global, row.name, bits]);
    e.call(ATEXIT, &args![row.destructor]);
}

// Translated from 00fb9520 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fLODMultTrees:LOD"` (`010a91c0`) at `011f9158` with the float at `01018204` (`0044f440`); its
/// destructor `00fdc790` goes to `_atexit`.
pub fn fn_00fb9520(e: &mut Engine) {
    float_setting(e, &SETTING_00FB9520);
}

// Translated from 00fbae00 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fRenderDepthMaxDepth:Imagespace"` (`010bc44c`) at `012023dc` with the float at `01022958` (`0044f440`); its
/// destructor `00fdd370` goes to `_atexit`.
pub fn fn_00fbae00(e: &mut Engine) {
    float_setting(e, &SETTING_00FBAE00);
}

// Translated from 00fbcba0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fTimePerSubStep:HAVOK"` (`010c4290`) at `01267b10` with the float at `010c42a8` (`0044f440`); its
/// destructor `00fdddd0` goes to `_atexit`.
pub fn fn_00fbcba0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCBA0);
}

// Translated from 00fbcc10 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fMaxTime:HAVOK"` (`010c42d8`) at `01267b34` with the float at `010c42e8` (`0044f440`); its
/// destructor `00fdde00` goes to `_atexit`.
pub fn fn_00fbcc10(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCC10);
}

// Translated from 00fbce30 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fCameraDist:RagdollAnim"` (`010c4e90`) at `01267c6c` with the float at `01013974` (`0044f440`); its
/// destructor `00fddef0` goes to `_atexit`.
pub fn fn_00fbce30(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCE30);
}

// Translated from 00fbce60 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fHierarchyGain:RagdollAnim"` (`010c4ea8`) at `01267c78` with the float at `0104722c` (`0044f440`); its
/// destructor `00fddf00` goes to `_atexit`.
pub fn fn_00fbce60(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCE60);
}

// Translated from 00fbcec0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fAccelerationGain:RagdollAnim"` (`010c4ee4`) at `01267c90` with 1.0 (`FLD1`) (`0044f440`); its
/// destructor `00fddf20` goes to `_atexit`.
pub fn fn_00fbcec0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCEC0);
}

// Translated from 00fbcef0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fVelocityGain:RagdollAnim"` (`010c4f04`) at `01267c9c` with the float at `01018180` (`0044f440`); its
/// destructor `00fddf30` goes to `_atexit`.
pub fn fn_00fbcef0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCEF0);
}

// Translated from 00fbcf20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fPositionGain:RagdollAnim"` (`010c4f20`) at `01267ca8` with the float at `010181bc` (`0044f440`); its
/// destructor `00fddf40` goes to `_atexit`.
pub fn fn_00fbcf20(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCF20);
}

// Translated from 00fbcf50 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fPositionMaxLinearVelocity:RagdollAnim"` (`010c4f3c`) at `01267cb4` with the float at `01047228` (`0044f440`); its
/// destructor `00fddf50` goes to `_atexit`.
pub fn fn_00fbcf50(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCF50);
}

// Translated from 00fbcf80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fPositionMaxAngularVelocity:RagdollAnim"` (`010c4f64`) at `01267cc0` with the float at `01047224` (`0044f440`); its
/// destructor `00fddf60` goes to `_atexit`.
pub fn fn_00fbcf80(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCF80);
}

// Translated from 00fbcfb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fSnapGain:RagdollAnim"` (`010c4f8c`) at `01267ccc` with the float at `0101e2bc` (`0044f440`); its
/// destructor `00fddf70` goes to `_atexit`.
pub fn fn_00fbcfb0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCFB0);
}

// Translated from 00fbcfe0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fSnapMaxLinearVelocity:RagdollAnim"` (`010c4fa4`) at `01267cd8` with the float at `01017718` (`0044f440`); its
/// destructor `00fddf80` goes to `_atexit`.
pub fn fn_00fbcfe0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBCFE0);
}

// Translated from 00fbd010 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fSnapMaxAngularVelocity:RagdollAnim"` (`010c4fc8`) at `01267ce4` with the float at `01017870` (`0044f440`); its
/// destructor `00fddf90` goes to `_atexit`.
pub fn fn_00fbd010(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD010);
}

// Translated from 00fbd040 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fSnapMaxLinearDistance:RagdollAnim"` (`010c4fec`) at `01267cf0` with the float at `01017870` (`0044f440`); its
/// destructor `00fddfa0` goes to `_atexit`.
pub fn fn_00fbd040(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD040);
}

// Translated from 00fbd070 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fSnapMaxAngularDistance:RagdollAnim"` (`010c5010`) at `01267cfc` with 1.0 (`FLD1`) (`0044f440`); its
/// destructor `00fddfb0` goes to `_atexit`.
pub fn fn_00fbd070(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD070);
}

// Translated from 00fbd0d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fMinTrackingDist:LookIK"` (`010c504c`) at `01267d14` with the float at `01018f7c` (`0044f440`); its
/// destructor `00fddfd0` goes to `_atexit`.
pub fn fn_00fbd0d0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD0D0);
}

// Translated from 00fbd100 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fAngleMax:LookIK"` (`010c5064`) at `01267d20` with the float at `010586fc` (`0044f440`); its
/// destructor `00fddfe0` goes to `_atexit`.
pub fn fn_00fbd100(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD100);
}

// Translated from 00fbd130 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fAngleMaxEase:LookIK"` (`010c5078`) at `01267d2c` with 1.0 (`FLD1`) (`0044f440`); its
/// destructor `00fddff0` goes to `_atexit`.
pub fn fn_00fbd130(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD130);
}

// Translated from 00fbd190 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fOnOffGain:FootIK"` (`010c50ac`) at `01267d44` with the float at `01013ea4` (`0044f440`); its
/// destructor `00fde010` goes to `_atexit`.
pub fn fn_00fbd190(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD190);
}

// Translated from 00fbd1c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fGroundAscendingGain:FootIK"` (`010c50c0`) at `01267d50` with the float at `01021f44` (`0044f440`); its
/// destructor `00fde020` goes to `_atexit`.
pub fn fn_00fbd1c0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD1C0);
}

// Translated from 00fbd1f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fGroundDescendingGain:FootIK"` (`010c50dc`) at `01267d5c` with the float at `01017870` (`0044f440`); its
/// destructor `00fde030` goes to `_atexit`.
pub fn fn_00fbd1f0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD1F0);
}

// Translated from 00fbd220 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fFootRaisedGain:FootIK"` (`010c50fc`) at `01267d68` with the float at `0101df44` (`0044f440`); its
/// destructor `00fde040` goes to `_atexit`.
pub fn fn_00fbd220(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD220);
}

// Translated from 00fbd250 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fFootPlantedGain:FootIK"` (`010c5114`) at `01267d74` with 1.0 (`FLD1`) (`0044f440`); its
/// destructor `00fde050` goes to `_atexit`.
pub fn fn_00fbd250(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD250);
}

// Translated from 00fbd2a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fOriginalGroundHeightMS:FootIK"` (`010c5144`) at `01267d8c` with the float at `010c5164` (`0044f440`); its
/// destructor `00fde070` goes to `_atexit`.
pub fn fn_00fbd2a0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD2A0);
}

// Translated from 00fbd2d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fAnkleOffset:FootIK"` (`010c5168`) at `01267d98` with the float at `0101df44` (`0044f440`); its
/// destructor `00fde080` goes to `_atexit`.
pub fn fn_00fbd2d0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD2D0);
}

// Translated from 00fbd300 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fPelvisUpDownBias:FootIK"` (`010c517c`) at `01267da4` with the float at `01016264` (`0044f440`); its
/// destructor `00fde090` goes to `_atexit`.
pub fn fn_00fbd300(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD300);
}

// Translated from 00fbd330 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fPelvisOffsetDamping:FootIK"` (`010c5198`) at `01267db0` with the float at `0101df44` (`0044f440`); its
/// destructor `00fde0a0` goes to `_atexit`.
pub fn fn_00fbd330(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD330);
}

// Translated from 00fbd360 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fVertErrorGain:FootIK"` (`010c51b4`) at `01267dbc` with the float at `01016248` (`0044f440`); its
/// destructor `00fde0b0` goes to `_atexit`.
pub fn fn_00fbd360(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD360);
}

// Translated from 00fbd3b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fControllerTetherLen:FootIK"` (`010c51ec`) at `01267dd4` with the float at `01050c48` (`0044f440`); its
/// destructor `00fde0d0` goes to `_atexit`.
pub fn fn_00fbd3b0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD3B0);
}

// Translated from 00fbd3e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fMaxStepVertError:FootIK"` (`010c5208`) at `01267de0` with the float at `010586fc` (`0044f440`); its
/// destructor `00fde0e0` goes to `_atexit`.
pub fn fn_00fbd3e0(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD3E0);
}

// Translated from 00fbd410 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fMaxFootCastMilliSec:FootIK"` (`010c5224`) at `01267dec` with the float at `01018180` (`0044f440`); its
/// destructor `00fde0f0` goes to `_atexit`.
pub fn fn_00fbd410(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD410);
}

// Translated from 00fbd440 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fDriveGain:GrabIK"` (`010c5240`) at `01267df8` with the float at `0101622c` (`0044f440`); its
/// destructor `00fde100` goes to `_atexit`.
pub fn fn_00fbd440(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD440);
}

// Translated from 00fbd490 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fRagdollFeedback:FootIK"` (`010c5278`) at `01267e10` with the float at `010177e8` (`0044f440`); its
/// destructor `00fde120` goes to `_atexit`.
pub fn fn_00fbd490(e: &mut Engine) {
    float_setting(e, &SETTING_00FBD490);
}

// Translated from 00fbdb20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fDesiredVel:RagdollAnim"` (`010c73a8`) at `01268298` with the float at `0101e580` (`0044f440`); its
/// destructor `00fde1f0` goes to `_atexit`.
pub fn fn_00fbdb20(e: &mut Engine) {
    float_setting(e, &SETTING_00FBDB20);
}

// Translated from 00fbdb50 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fImpulseLimit:RagdollAnim"` (`010c73c0`) at `012682a4` with the float at `010223ec` (`0044f440`); its
/// destructor `00fde200` goes to `_atexit`.
pub fn fn_00fbdb50(e: &mut Engine) {
    float_setting(e, &SETTING_00FBDB50);
}

// Translated from 00fbdb80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting `"fDetectionUpdateTimeSec:RagdollAnim"` (`010c73dc`) at `012682b0` with the float at `0101712c` (`0044f440`); its
/// destructor `00fde210` goes to `_atexit`.
pub fn fn_00fbdb80(e: &mut Engine) {
    float_setting(e, &SETTING_00FBDB80);
}

/// `00fb9520`: `fLODMultTrees:LOD`.
const SETTING_00FB9520: FloatRow = FloatRow {
    global: 0x011f_9158,
    name: 0x010a_91c0,
    value: FloatValue::At(0x0101_8204),
    destructor: 0x00fd_c790,
};
/// `00fbae00`: `fRenderDepthMaxDepth:Imagespace`.
const SETTING_00FBAE00: FloatRow = FloatRow {
    global: 0x0120_23dc,
    name: 0x010b_c44c,
    value: FloatValue::At(0x0102_2958),
    destructor: 0x00fd_d370,
};
/// `00fbcba0`: `fTimePerSubStep:HAVOK`.
const SETTING_00FBCBA0: FloatRow = FloatRow {
    global: 0x0126_7b10,
    name: 0x010c_4290,
    value: FloatValue::At(0x010c_42a8),
    destructor: 0x00fd_ddd0,
};
/// `00fbcc10`: `fMaxTime:HAVOK`.
const SETTING_00FBCC10: FloatRow = FloatRow {
    global: 0x0126_7b34,
    name: 0x010c_42d8,
    value: FloatValue::At(0x010c_42e8),
    destructor: 0x00fd_de00,
};
/// `00fbce30`: `fCameraDist:RagdollAnim`.
const SETTING_00FBCE30: FloatRow = FloatRow {
    global: 0x0126_7c6c,
    name: 0x010c_4e90,
    value: FloatValue::At(0x0101_3974),
    destructor: 0x00fd_def0,
};
/// `00fbce60`: `fHierarchyGain:RagdollAnim`.
const SETTING_00FBCE60: FloatRow = FloatRow {
    global: 0x0126_7c78,
    name: 0x010c_4ea8,
    value: FloatValue::At(0x0104_722c),
    destructor: 0x00fd_df00,
};
/// `00fbcec0`: `fAccelerationGain:RagdollAnim`.
const SETTING_00FBCEC0: FloatRow = FloatRow {
    global: 0x0126_7c90,
    name: 0x010c_4ee4,
    value: FloatValue::One,
    destructor: 0x00fd_df20,
};
/// `00fbcef0`: `fVelocityGain:RagdollAnim`.
const SETTING_00FBCEF0: FloatRow = FloatRow {
    global: 0x0126_7c9c,
    name: 0x010c_4f04,
    value: FloatValue::At(0x0101_8180),
    destructor: 0x00fd_df30,
};
/// `00fbcf20`: `fPositionGain:RagdollAnim`.
const SETTING_00FBCF20: FloatRow = FloatRow {
    global: 0x0126_7ca8,
    name: 0x010c_4f20,
    value: FloatValue::At(0x0101_81bc),
    destructor: 0x00fd_df40,
};
/// `00fbcf50`: `fPositionMaxLinearVelocity:RagdollAnim`.
const SETTING_00FBCF50: FloatRow = FloatRow {
    global: 0x0126_7cb4,
    name: 0x010c_4f3c,
    value: FloatValue::At(0x0104_7228),
    destructor: 0x00fd_df50,
};
/// `00fbcf80`: `fPositionMaxAngularVelocity:RagdollAnim`.
const SETTING_00FBCF80: FloatRow = FloatRow {
    global: 0x0126_7cc0,
    name: 0x010c_4f64,
    value: FloatValue::At(0x0104_7224),
    destructor: 0x00fd_df60,
};
/// `00fbcfb0`: `fSnapGain:RagdollAnim`.
const SETTING_00FBCFB0: FloatRow = FloatRow {
    global: 0x0126_7ccc,
    name: 0x010c_4f8c,
    value: FloatValue::At(0x0101_e2bc),
    destructor: 0x00fd_df70,
};
/// `00fbcfe0`: `fSnapMaxLinearVelocity:RagdollAnim`.
const SETTING_00FBCFE0: FloatRow = FloatRow {
    global: 0x0126_7cd8,
    name: 0x010c_4fa4,
    value: FloatValue::At(0x0101_7718),
    destructor: 0x00fd_df80,
};
/// `00fbd010`: `fSnapMaxAngularVelocity:RagdollAnim`.
const SETTING_00FBD010: FloatRow = FloatRow {
    global: 0x0126_7ce4,
    name: 0x010c_4fc8,
    value: FloatValue::At(0x0101_7870),
    destructor: 0x00fd_df90,
};
/// `00fbd040`: `fSnapMaxLinearDistance:RagdollAnim`.
const SETTING_00FBD040: FloatRow = FloatRow {
    global: 0x0126_7cf0,
    name: 0x010c_4fec,
    value: FloatValue::At(0x0101_7870),
    destructor: 0x00fd_dfa0,
};
/// `00fbd070`: `fSnapMaxAngularDistance:RagdollAnim`.
const SETTING_00FBD070: FloatRow = FloatRow {
    global: 0x0126_7cfc,
    name: 0x010c_5010,
    value: FloatValue::One,
    destructor: 0x00fd_dfb0,
};
/// `00fbd0d0`: `fMinTrackingDist:LookIK`.
const SETTING_00FBD0D0: FloatRow = FloatRow {
    global: 0x0126_7d14,
    name: 0x010c_504c,
    value: FloatValue::At(0x0101_8f7c),
    destructor: 0x00fd_dfd0,
};
/// `00fbd100`: `fAngleMax:LookIK`.
const SETTING_00FBD100: FloatRow = FloatRow {
    global: 0x0126_7d20,
    name: 0x010c_5064,
    value: FloatValue::At(0x0105_86fc),
    destructor: 0x00fd_dfe0,
};
/// `00fbd130`: `fAngleMaxEase:LookIK`.
const SETTING_00FBD130: FloatRow = FloatRow {
    global: 0x0126_7d2c,
    name: 0x010c_5078,
    value: FloatValue::One,
    destructor: 0x00fd_dff0,
};
/// `00fbd190`: `fOnOffGain:FootIK`.
const SETTING_00FBD190: FloatRow = FloatRow {
    global: 0x0126_7d44,
    name: 0x010c_50ac,
    value: FloatValue::At(0x0101_3ea4),
    destructor: 0x00fd_e010,
};
/// `00fbd1c0`: `fGroundAscendingGain:FootIK`.
const SETTING_00FBD1C0: FloatRow = FloatRow {
    global: 0x0126_7d50,
    name: 0x010c_50c0,
    value: FloatValue::At(0x0102_1f44),
    destructor: 0x00fd_e020,
};
/// `00fbd1f0`: `fGroundDescendingGain:FootIK`.
const SETTING_00FBD1F0: FloatRow = FloatRow {
    global: 0x0126_7d5c,
    name: 0x010c_50dc,
    value: FloatValue::At(0x0101_7870),
    destructor: 0x00fd_e030,
};
/// `00fbd220`: `fFootRaisedGain:FootIK`.
const SETTING_00FBD220: FloatRow = FloatRow {
    global: 0x0126_7d68,
    name: 0x010c_50fc,
    value: FloatValue::At(0x0101_df44),
    destructor: 0x00fd_e040,
};
/// `00fbd250`: `fFootPlantedGain:FootIK`.
const SETTING_00FBD250: FloatRow = FloatRow {
    global: 0x0126_7d74,
    name: 0x010c_5114,
    value: FloatValue::One,
    destructor: 0x00fd_e050,
};
/// `00fbd2a0`: `fOriginalGroundHeightMS:FootIK`.
const SETTING_00FBD2A0: FloatRow = FloatRow {
    global: 0x0126_7d8c,
    name: 0x010c_5144,
    value: FloatValue::At(0x010c_5164),
    destructor: 0x00fd_e070,
};
/// `00fbd2d0`: `fAnkleOffset:FootIK`.
const SETTING_00FBD2D0: FloatRow = FloatRow {
    global: 0x0126_7d98,
    name: 0x010c_5168,
    value: FloatValue::At(0x0101_df44),
    destructor: 0x00fd_e080,
};
/// `00fbd300`: `fPelvisUpDownBias:FootIK`.
const SETTING_00FBD300: FloatRow = FloatRow {
    global: 0x0126_7da4,
    name: 0x010c_517c,
    value: FloatValue::At(0x0101_6264),
    destructor: 0x00fd_e090,
};
/// `00fbd330`: `fPelvisOffsetDamping:FootIK`.
const SETTING_00FBD330: FloatRow = FloatRow {
    global: 0x0126_7db0,
    name: 0x010c_5198,
    value: FloatValue::At(0x0101_df44),
    destructor: 0x00fd_e0a0,
};
/// `00fbd360`: `fVertErrorGain:FootIK`.
const SETTING_00FBD360: FloatRow = FloatRow {
    global: 0x0126_7dbc,
    name: 0x010c_51b4,
    value: FloatValue::At(0x0101_6248),
    destructor: 0x00fd_e0b0,
};
/// `00fbd3b0`: `fControllerTetherLen:FootIK`.
const SETTING_00FBD3B0: FloatRow = FloatRow {
    global: 0x0126_7dd4,
    name: 0x010c_51ec,
    value: FloatValue::At(0x0105_0c48),
    destructor: 0x00fd_e0d0,
};
/// `00fbd3e0`: `fMaxStepVertError:FootIK`.
const SETTING_00FBD3E0: FloatRow = FloatRow {
    global: 0x0126_7de0,
    name: 0x010c_5208,
    value: FloatValue::At(0x0105_86fc),
    destructor: 0x00fd_e0e0,
};
/// `00fbd410`: `fMaxFootCastMilliSec:FootIK`.
const SETTING_00FBD410: FloatRow = FloatRow {
    global: 0x0126_7dec,
    name: 0x010c_5224,
    value: FloatValue::At(0x0101_8180),
    destructor: 0x00fd_e0f0,
};
/// `00fbd440`: `fDriveGain:GrabIK`.
const SETTING_00FBD440: FloatRow = FloatRow {
    global: 0x0126_7df8,
    name: 0x010c_5240,
    value: FloatValue::At(0x0101_622c),
    destructor: 0x00fd_e100,
};
/// `00fbd490`: `fRagdollFeedback:FootIK`.
const SETTING_00FBD490: FloatRow = FloatRow {
    global: 0x0126_7e10,
    name: 0x010c_5278,
    value: FloatValue::At(0x0101_77e8),
    destructor: 0x00fd_e120,
};
/// `00fbdb20`: `fDesiredVel:RagdollAnim`.
const SETTING_00FBDB20: FloatRow = FloatRow {
    global: 0x0126_8298,
    name: 0x010c_73a8,
    value: FloatValue::At(0x0101_e580),
    destructor: 0x00fd_e1f0,
};
/// `00fbdb50`: `fImpulseLimit:RagdollAnim`.
const SETTING_00FBDB50: FloatRow = FloatRow {
    global: 0x0126_82a4,
    name: 0x010c_73c0,
    value: FloatValue::At(0x0102_23ec),
    destructor: 0x00fd_e200,
};
/// `00fbdb80`: `fDetectionUpdateTimeSec:RagdollAnim`.
const SETTING_00FBDB80: FloatRow = FloatRow {
    global: 0x0126_82b0,
    name: 0x010c_73dc,
    value: FloatValue::At(0x0101_712c),
    destructor: 0x00fd_e210,
};

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fb_9520, fn_00fb9520()),
        entry!(0x00fb_ae00, fn_00fbae00()),
        entry!(0x00fb_cba0, fn_00fbcba0()),
        entry!(0x00fb_cc10, fn_00fbcc10()),
        entry!(0x00fb_ce30, fn_00fbce30()),
        entry!(0x00fb_ce60, fn_00fbce60()),
        entry!(0x00fb_cec0, fn_00fbcec0()),
        entry!(0x00fb_cef0, fn_00fbcef0()),
        entry!(0x00fb_cf20, fn_00fbcf20()),
        entry!(0x00fb_cf50, fn_00fbcf50()),
        entry!(0x00fb_cf80, fn_00fbcf80()),
        entry!(0x00fb_cfb0, fn_00fbcfb0()),
        entry!(0x00fb_cfe0, fn_00fbcfe0()),
        entry!(0x00fb_d010, fn_00fbd010()),
        entry!(0x00fb_d040, fn_00fbd040()),
        entry!(0x00fb_d070, fn_00fbd070()),
        entry!(0x00fb_d0d0, fn_00fbd0d0()),
        entry!(0x00fb_d100, fn_00fbd100()),
        entry!(0x00fb_d130, fn_00fbd130()),
        entry!(0x00fb_d190, fn_00fbd190()),
        entry!(0x00fb_d1c0, fn_00fbd1c0()),
        entry!(0x00fb_d1f0, fn_00fbd1f0()),
        entry!(0x00fb_d220, fn_00fbd220()),
        entry!(0x00fb_d250, fn_00fbd250()),
        entry!(0x00fb_d2a0, fn_00fbd2a0()),
        entry!(0x00fb_d2d0, fn_00fbd2d0()),
        entry!(0x00fb_d300, fn_00fbd300()),
        entry!(0x00fb_d330, fn_00fbd330()),
        entry!(0x00fb_d360, fn_00fbd360()),
        entry!(0x00fb_d3b0, fn_00fbd3b0()),
        entry!(0x00fb_d3e0, fn_00fbd3e0()),
        entry!(0x00fb_d410, fn_00fbd410()),
        entry!(0x00fb_d440, fn_00fbd440()),
        entry!(0x00fb_d490, fn_00fbd490()),
        entry!(0x00fb_db20, fn_00fbdb20()),
        entry!(0x00fb_db50, fn_00fbdb50()),
        entry!(0x00fb_db80, fn_00fbdb80()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// Every float-setting initializer with its row.
    const FLOAT_ROWS: &[(u32, FloatRow)] = &[
        (0x00fb_9520, SETTING_00FB9520),
        (0x00fb_ae00, SETTING_00FBAE00),
        (0x00fb_cba0, SETTING_00FBCBA0),
        (0x00fb_cc10, SETTING_00FBCC10),
        (0x00fb_ce30, SETTING_00FBCE30),
        (0x00fb_ce60, SETTING_00FBCE60),
        (0x00fb_cec0, SETTING_00FBCEC0),
        (0x00fb_cef0, SETTING_00FBCEF0),
        (0x00fb_cf20, SETTING_00FBCF20),
        (0x00fb_cf50, SETTING_00FBCF50),
        (0x00fb_cf80, SETTING_00FBCF80),
        (0x00fb_cfb0, SETTING_00FBCFB0),
        (0x00fb_cfe0, SETTING_00FBCFE0),
        (0x00fb_d010, SETTING_00FBD010),
        (0x00fb_d040, SETTING_00FBD040),
        (0x00fb_d070, SETTING_00FBD070),
        (0x00fb_d0d0, SETTING_00FBD0D0),
        (0x00fb_d100, SETTING_00FBD100),
        (0x00fb_d130, SETTING_00FBD130),
        (0x00fb_d190, SETTING_00FBD190),
        (0x00fb_d1c0, SETTING_00FBD1C0),
        (0x00fb_d1f0, SETTING_00FBD1F0),
        (0x00fb_d220, SETTING_00FBD220),
        (0x00fb_d250, SETTING_00FBD250),
        (0x00fb_d2a0, SETTING_00FBD2A0),
        (0x00fb_d2d0, SETTING_00FBD2D0),
        (0x00fb_d300, SETTING_00FBD300),
        (0x00fb_d330, SETTING_00FBD330),
        (0x00fb_d360, SETTING_00FBD360),
        (0x00fb_d3b0, SETTING_00FBD3B0),
        (0x00fb_d3e0, SETTING_00FBD3E0),
        (0x00fb_d410, SETTING_00FBD410),
        (0x00fb_d440, SETTING_00FBD440),
        (0x00fb_d490, SETTING_00FBD490),
        (0x00fb_db20, SETTING_00FBDB20),
        (0x00fb_db50, SETTING_00FBDB50),
        (0x00fb_db80, SETTING_00FBDB80),
    ];

    #[test]
    fn each_float_setting_is_constructed_with_its_name_and_float() {
        for &(entry, row) in FLOAT_ROWS {
            let mut e = Engine::new();
            if let FloatValue::At(a) = row.value {
                e.map(a, 4);
                // A signalling NaN comes out of FLD/FSTP quieted.
                e.mem.set_u32(a, 0x7f80_0001);
            }
            e.register(FLOAT_SETTING_CONSTRUCT, |_, _| Ret::default());
            e.register(ATEXIT, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(entry, &args![]);
            let calls: Calls = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != entry)
                .collect();
            let bits = match row.value {
                FloatValue::At(_) => 0x7fc0_0001,
                FloatValue::One => 0x3f80_0000,
            };
            assert_eq!(
                calls,
                vec![
                    (FLOAT_SETTING_CONSTRUCT, vec![row.global, row.name, bits]),
                    (ATEXIT, vec![row.destructor]),
                ],
                "{entry:08x}"
            );
        }
        assert_eq!(FLOAT_ROWS.len(), 37);
        // fTimePerSubStep:HAVOK, from the disassembly.
        assert_eq!(SETTING_00FBCBA0.global, 0x0126_7b10);
        assert_eq!(SETTING_00FBCBA0.name, 0x010c_4290);
        assert_eq!(SETTING_00FBCBA0.value, FloatValue::At(0x010c_42a8));
        assert_eq!(SETTING_00FBCEC0.value, FloatValue::One);
    }
}
