//! `(unplaced)/static_init_irregular_d` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: dynamic initializers of FalloutNV.exe 1.4.0.525 that
//! `research/initgen` refused (their `FS:[0]` exception frame matches none of its shapes) and
//! parts 1 to 3 (`static_init_irregular.rs`, `_b.rs`, `_c.rs`) do not have, each translated
//! from its own disassembly. Part 4 of the list: the settings start-up skipped, which left
//! their globals unbuilt and out of their collections (so the INI files could not set them).
//!
//! Each builds one `Setting` (Xbox PDB) in place, in the code's store order: the base vtable
//! `01012120` at +0, the name at +8, the value at +4 (a byte, a word, or a float copied through
//! `FLD`/`FSTP`, `FLD1` and `FLDZ` included), then the setting's own vtable at +0, the
//! collection's getter (no arguments), the collection's virtual function at slot +4 with the
//! setting (`RET 4`), and the setting's destructor given to `_atexit`. Two collections occur:
//! the settings collection (`0044f570`, instance `011f96a0`, vtable `01017720`) and the INI
//! prefs collection (`004de4a0`, instance `011f35a0`, vtable `01022738`).
//!
//! `00fb3560` (`rDebugTextColor:Menu`) builds its colour word first (`0xff << 24`, then the
//! bytes `0xfb`, `0xe9` and `0xff` masked, shifted and or-ed into a local) and stores it as the
//! value.
//!
//! Also here, from the same list: the menu game settings' object runs `00f7a190`,
//! `00f7e000`, `00f7e2b0`, `00f7ef90` and `00f7fbc0` (`psDevice`, `psJoystick`,
//! `psXBoxGamepadText`, `psPS3GamepadText`, `psWeaponTypes`; part 3's [`InitStep`] shape,
//! with zero words stored after the blocks in two of them) and `FaderData` (`00f8c180`).
//!
//! Every one sits in an `FS:[0]` exception frame (the `/GS` cookie word is pushed into it, but
//! `00ec408c` is not called) and is marked `// Translation incomplete`: the frame is the
//! compiler's unwinding on an exception and is not translated; every store and call is.
//! They are cdecl functions with no parameters; `_initterm` ignores what they leave in `EAX`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::static_init_irregular_c::{run_steps, InitStep};

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// The `Setting` base vtable each initializer stores first.
const SETTING_BASE_VTABLE: u32 = 0x0101_2120;

/// Which collection the setting joins: its vtable, the collection's getter
/// (cdecl, no arguments) and the collection's instance pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// `01017720`; `0044f570`; `011f96a0`.
    Settings,
    /// `01022738`; `004de4a0`; `011f35a0`.
    Prefs,
}

impl Kind {
    fn vtable(self) -> u32 {
        match self {
            Kind::Settings => 0x0101_7720,
            Kind::Prefs => 0x0102_2738,
        }
    }
    fn getter(self) -> u32 {
        match self {
            Kind::Settings => 0x0044_f570,
            Kind::Prefs => 0x004d_e4a0,
        }
    }
    fn instance(self) -> u32 {
        match self {
            Kind::Settings => 0x011f_96a0,
            Kind::Prefs => 0x011f_35a0,
        }
    }
}

/// The value stored at +4.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    /// `MOV byte ptr [g+4],imm`.
    Byte(u8),
    /// `MOV dword ptr [g+4],imm`.
    Word(u32),
    /// `FLD float ptr [a]; FSTP float ptr [g+4]` (a signalling NaN quieted).
    Float(u32),
    /// `FLD1`/`FLDZ`; `FSTP float ptr [g+4]`: the float's exact bits.
    Const(u32),
}

/// One setting initializer.
#[derive(Clone, Copy, Debug)]
struct Row {
    global: u32,
    name: u32,
    value: Value,
    kind: Kind,
    destructor: u32,
}

/// The common body (between the frame's set-up and its removal).
fn setting(e: &mut Engine, row: &Row) {
    let g = row.global;
    e.set_global::<u32>(g, SETTING_BASE_VTABLE);
    e.set_global::<u32>(g + 8, row.name);
    match row.value {
        Value::Byte(b) => e.set_global::<u8>(g + 4, b),
        Value::Word(w) => e.set_global::<u32>(g + 4, w),
        Value::Float(a) => {
            let bits = x87_quiet_bits(e.global::<u32>(a));
            e.set_global::<u32>(g + 4, bits);
        }
        Value::Const(bits) => e.set_global::<u32>(g + 4, bits),
    }
    register(e, g, row.kind, row.destructor);
}

/// From frame state 0 on: the setting's vtable, the getter, the collection's slot +4 with the
/// setting, `_atexit(destructor)`.
fn register(e: &mut Engine, g: u32, kind: Kind, destructor: u32) {
    e.set_global::<u32>(g, kind.vtable());
    e.call(kind.getter(), &args![]);
    let collection = e.global::<u32>(kind.instance());
    e.vcall(collection, 4, &args![g]);
    e.call(ATEXIT, &args![destructor]);
}

// Translated from 00fb3560 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"rDebugTextColor:Menu"` (`0109505c`) at `011f33bc` with the colour word built
/// in a local (`0xff << 24 | (0xfb & 0xff) << 16 | (0xe9 & 0xff) << 8 | (0xff & 0xff)`), added
/// to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`); its destructor
/// `00fda2b0` goes to `_atexit`.
// Translation incomplete: 00fb3560 - FS:[0] exception frame (handler 00f1d8da) not translated.
pub fn fn_00fb3560(e: &mut Engine) {
    let mut colour: u32 = 0xff << 0x18;
    colour |= (0xfb & 0xff) << 0x10;
    colour |= (0xe9 & 0xff) << 0x08;
    colour |= 0xff & 0xff;
    let g = 0x011f_33bc;
    e.set_global::<u32>(g, SETTING_BASE_VTABLE);
    e.set_global::<u32>(g + 8, 0x0109_505c);
    e.set_global::<u32>(g + 4, colour);
    register(e, g, Kind::Settings, 0x00fd_a2b0);
}

// Translated from 00fb3640 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"iDebugTextFont:Menu"` (`01095074`) at `011f33c8` with the word `0x3`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fda330` goes to `_atexit`.
// Translation incomplete: 00fb3640 - FS:[0] exception frame (handler 00f1d8aa) not translated.
pub fn fn_00fb3640(e: &mut Engine) {
    setting(e, &ROW_00FB3640);
}

// Translated from 00fb6c20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"bEnableAudio:Audio"` (`010a325c`) at `011f6d9c` with the byte `1`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb2c0` goes to `_atexit`.
// Translation incomplete: 00fb6c20 - FS:[0] exception frame (handler 00f21a3a) not translated.
pub fn fn_00fb6c20(e: &mut Engine) {
    setting(e, &ROW_00FB6C20);
}

// Translated from 00fb6cc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDialogMinDistance:Audio"` (`010a3270`) at `011f6da8` with the float at `0105c004` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb340` goes to `_atexit`.
// Translation incomplete: 00fb6cc0 - FS:[0] exception frame (handler 00f21a6a) not translated.
pub fn fn_00fb6cc0(e: &mut Engine) {
    setting(e, &ROW_00FB6CC0);
}

// Translated from 00fb6d60 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDialogMaxDistance:Audio"` (`010a328c`) at `011f6db4` with the float at `0103d24c` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb3c0` goes to `_atexit`.
// Translation incomplete: 00fb6d60 - FS:[0] exception frame (handler 00f21a9a) not translated.
pub fn fn_00fb6d60(e: &mut Engine) {
    setting(e, &ROW_00FB6D60);
}

// Translated from 00fb6e00 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDawnTime:Audio"` (`010a32a8`) at `011f6dc0` with the float at `0101712c` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb440` goes to `_atexit`.
// Translation incomplete: 00fb6e00 - FS:[0] exception frame (handler 00f21aca) not translated.
pub fn fn_00fb6e00(e: &mut Engine) {
    setting(e, &ROW_00FB6E00);
}

// Translated from 00fb6ea0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDayTime:Audio"` (`010a32b8`) at `011f6dcc` with the float at `0102918c` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb4c0` goes to `_atexit`.
// Translation incomplete: 00fb6ea0 - FS:[0] exception frame (handler 00f21afa) not translated.
pub fn fn_00fb6ea0(e: &mut Engine) {
    setting(e, &ROW_00FB6EA0);
}

// Translated from 00fb6f40 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDuskTime:Audio"` (`010a32c8`) at `011f6dd8` with the float at `01047224` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb540` goes to `_atexit`.
// Translation incomplete: 00fb6f40 - FS:[0] exception frame (handler 00f21b2a) not translated.
pub fn fn_00fb6f40(e: &mut Engine) {
    setting(e, &ROW_00FB6F40);
}

// Translated from 00fb6fe0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fNightTime:Audio"` (`010a32d8`) at `011f6de4` with the float at `01017868` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb5c0` goes to `_atexit`.
// Translation incomplete: 00fb6fe0 - FS:[0] exception frame (handler 00f21b5a) not translated.
pub fn fn_00fb6fe0(e: &mut Engine) {
    setting(e, &ROW_00FB6FE0);
}

// Translated from 00fb70b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"bMultiThreadAudio:Audio"` (`010a32ec`) at `011f6dfc` with the byte `0`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb650` goes to `_atexit`.
// Translation incomplete: 00fb70b0 - FS:[0] exception frame (handler 00f21b8a) not translated.
pub fn fn_00fb70b0(e: &mut Engine) {
    setting(e, &ROW_00FB70B0);
}

// Translated from 00fb7150 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDefaultMasterVolume:Audio"` (`010a3758`) at `011f6e14` with 1.0 (`FLD1`/`FSTP`), added to the INI prefs collection (`004de4a0`, `011f35a0`, vtable `01022738`);
/// its destructor `00fdb6d0` goes to `_atexit`.
// Translation incomplete: 00fb7150 - FS:[0] exception frame (handler 00f2242a) not translated.
pub fn fn_00fb7150(e: &mut Engine) {
    setting(e, &ROW_00FB7150);
}

// Translated from 00fb71f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDefaultFootVolume:Audio"` (`010a3774`) at `011f6e20` with the float at `01016248` (`FLD`/`FSTP`), added to the INI prefs collection (`004de4a0`, `011f35a0`, vtable `01022738`);
/// its destructor `00fdb750` goes to `_atexit`.
// Translation incomplete: 00fb71f0 - FS:[0] exception frame (handler 00f2245a) not translated.
pub fn fn_00fb71f0(e: &mut Engine) {
    setting(e, &ROW_00FB71F0);
}

// Translated from 00fb7290 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDefaultVoiceVolume:Audio"` (`010a3790`) at `011f6e2c` with the float at `01018180` (`FLD`/`FSTP`), added to the INI prefs collection (`004de4a0`, `011f35a0`, vtable `01022738`);
/// its destructor `00fdb7d0` goes to `_atexit`.
// Translation incomplete: 00fb7290 - FS:[0] exception frame (handler 00f2248a) not translated.
pub fn fn_00fb7290(e: &mut Engine) {
    setting(e, &ROW_00FB7290);
}

// Translated from 00fb7330 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDefaultEffectsVolume:Audio"` (`010a37ac`) at `011f6e38` with 1.0 (`FLD1`/`FSTP`), added to the INI prefs collection (`004de4a0`, `011f35a0`, vtable `01022738`);
/// its destructor `00fdb850` goes to `_atexit`.
// Translation incomplete: 00fb7330 - FS:[0] exception frame (handler 00f224ba) not translated.
pub fn fn_00fb7330(e: &mut Engine) {
    setting(e, &ROW_00FB7330);
}

// Translated from 00fb73d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDefaultMusicVolume:Audio"` (`010a37c8`) at `011f6e44` with the float at `01018180` (`FLD`/`FSTP`), added to the INI prefs collection (`004de4a0`, `011f35a0`, vtable `01022738`);
/// its destructor `00fdb8d0` goes to `_atexit`.
// Translation incomplete: 00fb73d0 - FS:[0] exception frame (handler 00f224ea) not translated.
pub fn fn_00fb73d0(e: &mut Engine) {
    setting(e, &ROW_00FB73D0);
}

// Translated from 00fb7470 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDefaultRadioVolume:Audio"` (`010a37e4`) at `011f6e50` with the float at `01016248` (`FLD`/`FSTP`), added to the INI prefs collection (`004de4a0`, `011f35a0`, vtable `01022738`);
/// its destructor `00fdb950` goes to `_atexit`.
// Translation incomplete: 00fb7470 - FS:[0] exception frame (handler 00f2251a) not translated.
pub fn fn_00fb7470(e: &mut Engine) {
    setting(e, &ROW_00FB7470);
}

// Translated from 00fb7510 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"bUseAudioDebugInformation:Audio"` (`010a3800`) at `011f6e5c` with the byte `1`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdb9d0` goes to `_atexit`.
// Translation incomplete: 00fb7510 - FS:[0] exception frame (handler 00f2254a) not translated.
pub fn fn_00fb7510(e: &mut Engine) {
    setting(e, &ROW_00FB7510);
}

// Translated from 00fb75b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fAudioDebugDelay:Audio"` (`010a3820`) at `011f6e68` with 0.0 (`FLDZ`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdba50` goes to `_atexit`.
// Translation incomplete: 00fb75b0 - FS:[0] exception frame (handler 00f2257a) not translated.
pub fn fn_00fb75b0(e: &mut Engine) {
    setting(e, &ROW_00FB75B0);
}

// Translated from 00fb7650 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fMasterVolumeMult:Audio"` (`010a3838`) at `011f6e74` with the float at `01022004` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbad0` goes to `_atexit`.
// Translation incomplete: 00fb7650 - FS:[0] exception frame (handler 00f225aa) not translated.
pub fn fn_00fb7650(e: &mut Engine) {
    setting(e, &ROW_00FB7650);
}

// Translated from 00fb76f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDialogueFadeDecibels:Audio"` (`010a3850`) at `011f6e80` with the float at `01050c48` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbb50` goes to `_atexit`.
// Translation incomplete: 00fb76f0 - FS:[0] exception frame (handler 00f225da) not translated.
pub fn fn_00fb76f0(e: &mut Engine) {
    setting(e, &ROW_00FB76F0);
}

// Translated from 00fb7790 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDialogueFadeSecondsIn:Audio"` (`010a386c`) at `011f6e8c` with the float at `010162c0` (`FLD`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbbd0` goes to `_atexit`.
// Translation incomplete: 00fb7790 - FS:[0] exception frame (handler 00f2260a) not translated.
pub fn fn_00fb7790(e: &mut Engine) {
    setting(e, &ROW_00FB7790);
}

// Translated from 00fb7830 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fDialogueFadeSecondsOut:Audio"` (`010a388c`) at `011f6e98` with 1.0 (`FLD1`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbc50` goes to `_atexit`.
// Translation incomplete: 00fb7830 - FS:[0] exception frame (handler 00f2263a) not translated.
pub fn fn_00fb7830(e: &mut Engine) {
    setting(e, &ROW_00FB7830);
}

// Translated from 00fb78d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"iAttenuationEffectiveSilence:Audio"` (`010a38ac`) at `011f6ea4` with the word `0x2328`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbcd0` goes to `_atexit`.
// Translation incomplete: 00fb78d0 - FS:[0] exception frame (handler 00f2266a) not translated.
pub fn fn_00fb78d0(e: &mut Engine) {
    setting(e, &ROW_00FB78D0);
}

// Translated from 00fb7970 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"iAttenuationDialogMenuMute:Audio"` (`010a38d0`) at `011f6eb0` with the word `0xbb8`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbd50` goes to `_atexit`.
// Translation incomplete: 00fb7970 - FS:[0] exception frame (handler 00f2269a) not translated.
pub fn fn_00fb7970(e: &mut Engine) {
    setting(e, &ROW_00FB7970);
}

// Translated from 00fb7a10 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"bEnableAudioCache:Audio"` (`010a38f4`) at `011f6ebc` with the byte `1`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbdd0` goes to `_atexit`.
// Translation incomplete: 00fb7a10 - FS:[0] exception frame (handler 00f226ca) not translated.
pub fn fn_00fb7a10(e: &mut Engine) {
    setting(e, &ROW_00FB7A10);
}

// Translated from 00fb7ab0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"iAudioCacheSize:Audio"` (`010a390c`) at `011f6ec8` with the word `0x800`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbe50` goes to `_atexit`.
// Translation incomplete: 00fb7ab0 - FS:[0] exception frame (handler 00f226fa) not translated.
pub fn fn_00fb7ab0(e: &mut Engine) {
    setting(e, &ROW_00FB7AB0);
}

// Translated from 00fb7b50 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"iMaxSizeForCachedSound:Audio"` (`010a3924`) at `011f6ed4` with the word `0x100`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbed0` goes to `_atexit`.
// Translation incomplete: 00fb7b50 - FS:[0] exception frame (handler 00f2272a) not translated.
pub fn fn_00fb7b50(e: &mut Engine) {
    setting(e, &ROW_00FB7B50);
}

// Translated from 00fb7bf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"iAudioHWThread:Audio"` (`010a3944`) at `011f6ee0` with the word `0x2`, added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdbf50` goes to `_atexit`.
// Translation incomplete: 00fb7bf0 - FS:[0] exception frame (handler 00f2275a) not translated.
pub fn fn_00fb7bf0(e: &mut Engine) {
    setting(e, &ROW_00FB7BF0);
}

// Translated from 00fb7fb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The setting `"fFilterdBAttenuation:Audio"` (`010a4228`) at `011f716c` with 1.0 (`FLD1`/`FSTP`), added to the settings collection (`0044f570`, `011f96a0`, vtable `01017720`);
/// its destructor `00fdc270` goes to `_atexit`.
// Translation incomplete: 00fb7fb0 - FS:[0] exception frame (handler 00f22dca) not translated.
pub fn fn_00fb7fb0(e: &mut Engine) {
    setting(e, &ROW_00FB7FB0);
}

/// `00fb3640`: `iDebugTextFont:Menu`.
const ROW_00FB3640: Row = Row {
    global: 0x011f_33c8,
    name: 0x0109_5074,
    value: Value::Word(0x0000_0003),
    kind: Kind::Settings,
    destructor: 0x00fd_a330,
};
/// `00fb6c20`: `bEnableAudio:Audio`.
const ROW_00FB6C20: Row = Row {
    global: 0x011f_6d9c,
    name: 0x010a_325c,
    value: Value::Byte(1),
    kind: Kind::Settings,
    destructor: 0x00fd_b2c0,
};
/// `00fb6cc0`: `fDialogMinDistance:Audio`.
const ROW_00FB6CC0: Row = Row {
    global: 0x011f_6da8,
    name: 0x010a_3270,
    value: Value::Float(0x0105_c004),
    kind: Kind::Settings,
    destructor: 0x00fd_b340,
};
/// `00fb6d60`: `fDialogMaxDistance:Audio`.
const ROW_00FB6D60: Row = Row {
    global: 0x011f_6db4,
    name: 0x010a_328c,
    value: Value::Float(0x0103_d24c),
    kind: Kind::Settings,
    destructor: 0x00fd_b3c0,
};
/// `00fb6e00`: `fDawnTime:Audio`.
const ROW_00FB6E00: Row = Row {
    global: 0x011f_6dc0,
    name: 0x010a_32a8,
    value: Value::Float(0x0101_712c),
    kind: Kind::Settings,
    destructor: 0x00fd_b440,
};
/// `00fb6ea0`: `fDayTime:Audio`.
const ROW_00FB6EA0: Row = Row {
    global: 0x011f_6dcc,
    name: 0x010a_32b8,
    value: Value::Float(0x0102_918c),
    kind: Kind::Settings,
    destructor: 0x00fd_b4c0,
};
/// `00fb6f40`: `fDuskTime:Audio`.
const ROW_00FB6F40: Row = Row {
    global: 0x011f_6dd8,
    name: 0x010a_32c8,
    value: Value::Float(0x0104_7224),
    kind: Kind::Settings,
    destructor: 0x00fd_b540,
};
/// `00fb6fe0`: `fNightTime:Audio`.
const ROW_00FB6FE0: Row = Row {
    global: 0x011f_6de4,
    name: 0x010a_32d8,
    value: Value::Float(0x0101_7868),
    kind: Kind::Settings,
    destructor: 0x00fd_b5c0,
};
/// `00fb70b0`: `bMultiThreadAudio:Audio`.
const ROW_00FB70B0: Row = Row {
    global: 0x011f_6dfc,
    name: 0x010a_32ec,
    value: Value::Byte(0),
    kind: Kind::Settings,
    destructor: 0x00fd_b650,
};
/// `00fb7150`: `fDefaultMasterVolume:Audio`.
const ROW_00FB7150: Row = Row {
    global: 0x011f_6e14,
    name: 0x010a_3758,
    value: Value::Const(0x3f80_0000),
    kind: Kind::Prefs,
    destructor: 0x00fd_b6d0,
};
/// `00fb71f0`: `fDefaultFootVolume:Audio`.
const ROW_00FB71F0: Row = Row {
    global: 0x011f_6e20,
    name: 0x010a_3774,
    value: Value::Float(0x0101_6248),
    kind: Kind::Prefs,
    destructor: 0x00fd_b750,
};
/// `00fb7290`: `fDefaultVoiceVolume:Audio`.
const ROW_00FB7290: Row = Row {
    global: 0x011f_6e2c,
    name: 0x010a_3790,
    value: Value::Float(0x0101_8180),
    kind: Kind::Prefs,
    destructor: 0x00fd_b7d0,
};
/// `00fb7330`: `fDefaultEffectsVolume:Audio`.
const ROW_00FB7330: Row = Row {
    global: 0x011f_6e38,
    name: 0x010a_37ac,
    value: Value::Const(0x3f80_0000),
    kind: Kind::Prefs,
    destructor: 0x00fd_b850,
};
/// `00fb73d0`: `fDefaultMusicVolume:Audio`.
const ROW_00FB73D0: Row = Row {
    global: 0x011f_6e44,
    name: 0x010a_37c8,
    value: Value::Float(0x0101_8180),
    kind: Kind::Prefs,
    destructor: 0x00fd_b8d0,
};
/// `00fb7470`: `fDefaultRadioVolume:Audio`.
const ROW_00FB7470: Row = Row {
    global: 0x011f_6e50,
    name: 0x010a_37e4,
    value: Value::Float(0x0101_6248),
    kind: Kind::Prefs,
    destructor: 0x00fd_b950,
};
/// `00fb7510`: `bUseAudioDebugInformation:Audio`.
const ROW_00FB7510: Row = Row {
    global: 0x011f_6e5c,
    name: 0x010a_3800,
    value: Value::Byte(1),
    kind: Kind::Settings,
    destructor: 0x00fd_b9d0,
};
/// `00fb75b0`: `fAudioDebugDelay:Audio`.
const ROW_00FB75B0: Row = Row {
    global: 0x011f_6e68,
    name: 0x010a_3820,
    value: Value::Const(0),
    kind: Kind::Settings,
    destructor: 0x00fd_ba50,
};
/// `00fb7650`: `fMasterVolumeMult:Audio`.
const ROW_00FB7650: Row = Row {
    global: 0x011f_6e74,
    name: 0x010a_3838,
    value: Value::Float(0x0102_2004),
    kind: Kind::Settings,
    destructor: 0x00fd_bad0,
};
/// `00fb76f0`: `fDialogueFadeDecibels:Audio`.
const ROW_00FB76F0: Row = Row {
    global: 0x011f_6e80,
    name: 0x010a_3850,
    value: Value::Float(0x0105_0c48),
    kind: Kind::Settings,
    destructor: 0x00fd_bb50,
};
/// `00fb7790`: `fDialogueFadeSecondsIn:Audio`.
const ROW_00FB7790: Row = Row {
    global: 0x011f_6e8c,
    name: 0x010a_386c,
    value: Value::Float(0x0101_62c0),
    kind: Kind::Settings,
    destructor: 0x00fd_bbd0,
};
/// `00fb7830`: `fDialogueFadeSecondsOut:Audio`.
const ROW_00FB7830: Row = Row {
    global: 0x011f_6e98,
    name: 0x010a_388c,
    value: Value::Const(0x3f80_0000),
    kind: Kind::Settings,
    destructor: 0x00fd_bc50,
};
/// `00fb78d0`: `iAttenuationEffectiveSilence:Audio`.
const ROW_00FB78D0: Row = Row {
    global: 0x011f_6ea4,
    name: 0x010a_38ac,
    value: Value::Word(0x0000_2328),
    kind: Kind::Settings,
    destructor: 0x00fd_bcd0,
};
/// `00fb7970`: `iAttenuationDialogMenuMute:Audio`.
const ROW_00FB7970: Row = Row {
    global: 0x011f_6eb0,
    name: 0x010a_38d0,
    value: Value::Word(0x0000_0bb8),
    kind: Kind::Settings,
    destructor: 0x00fd_bd50,
};
/// `00fb7a10`: `bEnableAudioCache:Audio`.
const ROW_00FB7A10: Row = Row {
    global: 0x011f_6ebc,
    name: 0x010a_38f4,
    value: Value::Byte(1),
    kind: Kind::Settings,
    destructor: 0x00fd_bdd0,
};
/// `00fb7ab0`: `iAudioCacheSize:Audio`.
const ROW_00FB7AB0: Row = Row {
    global: 0x011f_6ec8,
    name: 0x010a_390c,
    value: Value::Word(0x0000_0800),
    kind: Kind::Settings,
    destructor: 0x00fd_be50,
};
/// `00fb7b50`: `iMaxSizeForCachedSound:Audio`.
const ROW_00FB7B50: Row = Row {
    global: 0x011f_6ed4,
    name: 0x010a_3924,
    value: Value::Word(0x0000_0100),
    kind: Kind::Settings,
    destructor: 0x00fd_bed0,
};
/// `00fb7bf0`: `iAudioHWThread:Audio`.
const ROW_00FB7BF0: Row = Row {
    global: 0x011f_6ee0,
    name: 0x010a_3944,
    value: Value::Word(0x0000_0002),
    kind: Kind::Settings,
    destructor: 0x00fd_bf50,
};
/// `00fb7fb0`: `fFilterdBAttenuation:Audio`.
const ROW_00FB7FB0: Row = Row {
    global: 0x011f_716c,
    name: 0x010a_4228,
    value: Value::Const(0x3f80_0000),
    kind: Kind::Settings,
    destructor: 0x00fd_c270,
};

const STEPS_00F7A190: [InitStep; 4] = [
    InitStep::Object(0x011d_5730, 0x0106_36e4, 0x0106_36f4),
    InitStep::Object(0x011d_5734, 0x0106_36cc, 0x0106_36dc),
    InitStep::Object(0x011d_5738, 0x0106_36b0, 0x0106_36c0),
    InitStep::Object(0x011d_573c, 0x0106_368c, 0x0106_369c),
];

// Translated from 00f7a190 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_dynamic_initializer_for_psDevice_` (engine map; fallout shared/menugamesettings.cpp): 4 objects
/// (`00401000(0xc)`, then `0040c150` on the block with its two addresses; the answer or 0 in the
/// global), in the code's order ([`STEPS_00F7A190`], `011d5730` .. `011d573c`).
// Translation incomplete: 00f7a190 - FS:[0] exception frame (handler 00efd74c, one state per block, -1 between the blocks) not translated.
pub fn fn_00f7a190(e: &mut Engine) {
    run_steps(e, &STEPS_00F7A190);
}

const STEPS_00F7E000: [InitStep; 8] = [
    InitStep::Object(0x011d_51b0, 0x0106_40d0, 0x0101_d92c),
    InitStep::Object(0x011d_51b4, 0x0106_40bc, 0x0106_40c8),
    InitStep::Object(0x011d_51b8, 0x0106_40b0, 0x0104_843c),
    InitStep::Object(0x011d_51bc, 0x0106_4098, 0x0106_40a8),
    InitStep::Object(0x011d_51c0, 0x0106_408c, 0x0101_d924),
    InitStep::Object(0x011d_51c4, 0x0106_4074, 0x0106_4084),
    InitStep::Object(0x011d_51c8, 0x0106_4068, 0x0104_8444),
    InitStep::Object(0x011d_51cc, 0x0106_4054, 0x0106_4060),
];

// Translated from 00f7e000 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_dynamic_initializer_for_psJoystick_` (engine map; fallout shared/menugamesettings.cpp): 8 objects
/// (`00401000(0xc)`, then `0040c150` on the block with its two addresses; the answer or 0 in the
/// global), in the code's order ([`STEPS_00F7E000`], `011d51b0` .. `011d51cc`).
// Translation incomplete: 00f7e000 - FS:[0] exception frame (handler 00efdfc8, one state per block, -1 between the blocks) not translated.
pub fn fn_00f7e000(e: &mut Engine) {
    run_steps(e, &STEPS_00F7E000);
}

const STEPS_00F7E2B0: [InitStep; 34] = [
    InitStep::Object(0x011d_5268, 0x0106_44d8, 0x0106_44e4),
    InitStep::Object(0x011d_526c, 0x0106_44c0, 0x0106_44cc),
    InitStep::Object(0x011d_5270, 0x0106_44a8, 0x0106_44b4),
    InitStep::Object(0x011d_5274, 0x0106_4488, 0x0106_4494),
    InitStep::Object(0x011d_5278, 0x0106_446c, 0x0106_447c),
    InitStep::Object(0x011d_527c, 0x0106_4454, 0x0106_4460),
    InitStep::Object(0x011d_5280, 0x0106_4440, 0x0106_444c),
    InitStep::Object(0x011d_5284, 0x0106_4434, 0x0105_9d60),
    InitStep::Object(0x011d_5288, 0x0106_4410, 0x0106_441c),
    InitStep::Object(0x011d_528c, 0x0106_43ec, 0x0106_43f8),
    InitStep::Object(0x011d_5290, 0x0106_43cc, 0x0106_43d4),
    InitStep::Object(0x011d_5294, 0x0106_43ac, 0x0106_43b4),
    InitStep::Object(0x011d_5298, 0x0106_438c, 0x0106_4394),
    InitStep::Object(0x011d_529c, 0x0106_436c, 0x0106_4374),
    InitStep::Object(0x011d_52a0, 0x0106_4348, 0x0106_4354),
    InitStep::Object(0x011d_52a4, 0x0106_4324, 0x0106_4330),
    InitStep::Object(0x011d_52a8, 0x0106_4300, 0x0106_430c),
    InitStep::Object(0x011d_52ac, 0x0106_42dc, 0x0106_42e8),
    InitStep::Object(0x011d_52b0, 0x0106_42d0, 0x0106_441c),
    InitStep::Object(0x011d_52b4, 0x0106_42c4, 0x0106_43f8),
    InitStep::Object(0x011d_52b8, 0x0106_429c, 0x0106_42a8),
    InitStep::Object(0x011d_52bc, 0x0106_427c, 0x0106_4288),
    InitStep::Object(0x011d_52c0, 0x0106_4258, 0x0106_4268),
    InitStep::Object(0x011d_52c4, 0x0106_4230, 0x0106_423c),
    InitStep::Object(0x011d_52c8, 0x0106_4208, 0x0106_4218),
    InitStep::Object(0x011d_52cc, 0x0106_41e4, 0x0106_41f4),
    InitStep::Object(0x011d_52d0, 0x0106_41bc, 0x0106_41c8),
    InitStep::Object(0x011d_52d4, 0x0106_419c, 0x0106_41a8),
    InitStep::Object(0x011d_52d8, 0x0106_4174, 0x0106_4184),
    InitStep::Object(0x011d_52dc, 0x0106_4148, 0x0106_4154),
    InitStep::Object(0x011d_52e0, 0x0106_4120, 0x0106_4130),
    InitStep::Object(0x011d_52e4, 0x0106_40f8, 0x0106_4108),
    InitStep::Object(0x011d_52e8, 0x0106_40e8, 0x0106_3348),
    InitStep::Object(0x011d_52ec, 0x0106_40d8, 0x0106_40e0),
];

// Translated from 00f7e2b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_dynamic_initializer_for_psXBoxGamepadText_` (engine map; fallout shared/menugamesettings.cpp): 34 objects
/// (`00401000(0xc)`, then `0040c150` on the block with its two addresses; the answer or 0 in the
/// global), in the code's order ([`STEPS_00F7E2B0`], `011d5268` .. `011d52ec`).
// Translation incomplete: 00f7e2b0 - FS:[0] exception frame (handler 00efe1a2, one state per block, -1 between the blocks) not translated.
pub fn fn_00f7e2b0(e: &mut Engine) {
    run_steps(e, &STEPS_00F7E2B0);
}

const STEPS_00F7EF90: [InitStep; 34] = [
    InitStep::Object(0x011d_56a8, 0x0106_4718, 0x0106_4724),
    InitStep::Object(0x011d_56ac, 0x0106_46f8, 0x0106_4704),
    InitStep::Object(0x011d_56b0, 0x0106_46d0, 0x0106_46e0),
    InitStep::Object(0x011d_56b4, 0x0106_46a8, 0x0106_46b4),
    InitStep::Object(0x011d_56b8, 0x0106_4680, 0x0106_4690),
    InitStep::Object(0x011d_56bc, 0x0106_4658, 0x0106_4668),
    InitStep::Object(0x011d_56c0, 0x0106_4648, 0x0104_77f4),
    InitStep::Object(0x011d_56c4, 0x0106_4638, 0x0105_acb4),
    InitStep::Object(0x011d_56c8, 0x0106_4630, 0x0106_441c),
    InitStep::Object(0x011d_56cc, 0x0106_4628, 0x0106_43f8),
    InitStep::Object(0x011d_56d0, 0x0106_461c, 0x0106_43d4),
    InitStep::Object(0x011d_56d4, 0x0106_4610, 0x0106_43b4),
    InitStep::Object(0x011d_56d8, 0x0106_4604, 0x0106_4394),
    InitStep::Object(0x011d_56dc, 0x0106_45f4, 0x0106_4374),
    InitStep::Object(0x011d_56e0, 0x0106_45ec, 0x0106_4354),
    InitStep::Object(0x011d_56e4, 0x0106_45e4, 0x0106_4330),
    InitStep::Object(0x011d_56e8, 0x0106_45dc, 0x0106_42e8),
    InitStep::Object(0x011d_56ec, 0x0106_45d4, 0x0106_430c),
    InitStep::Object(0x011d_56f0, 0x0106_45c8, 0x0106_42a8),
    InitStep::Object(0x011d_56f4, 0x0106_45b8, 0x0106_4288),
    InitStep::Object(0x011d_56f8, 0x0106_45a8, 0x0106_4268),
    InitStep::Object(0x011d_56fc, 0x0106_459c, 0x0106_423c),
    InitStep::Object(0x011d_5700, 0x0106_458c, 0x0106_4218),
    InitStep::Object(0x011d_5704, 0x0106_457c, 0x0106_41f4),
    InitStep::Object(0x011d_5708, 0x0106_4570, 0x0106_41c8),
    InitStep::Object(0x011d_570c, 0x0106_4560, 0x0106_41a8),
    InitStep::Object(0x011d_5710, 0x0106_4550, 0x0106_4184),
    InitStep::Object(0x011d_5714, 0x0106_4544, 0x0106_4154),
    InitStep::Object(0x011d_5718, 0x0106_4534, 0x0106_4130),
    InitStep::Object(0x011d_571c, 0x0106_4524, 0x0106_4108),
    InitStep::Object(0x011d_5720, 0x0106_4514, 0x0106_3348),
    InitStep::Object(0x011d_5724, 0x0106_44f4, 0x0106_4500),
    InitStep::Store(0x011d_5728, 0),
    InitStep::Store(0x011d_572c, 0),
];

// Translated from 00f7ef90 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_dynamic_initializer_for_psPS3GamepadText_` (engine map; fallout shared/menugamesettings.cpp): 32 objects
/// (`00401000(0xc)`, then `0040c150` on the block with its two addresses; the answer or 0 in the
/// global) and 2 zero words stored after them, in the code's order ([`STEPS_00F7EF90`], `011d56a8` .. `011d572c`).
// Translation incomplete: 00f7ef90 - FS:[0] exception frame (handler 00efe356, one state per block, -1 between the blocks) not translated.
pub fn fn_00f7ef90(e: &mut Engine) {
    run_steps(e, &STEPS_00F7EF90);
}

const STEPS_00F7FBC0: [InitStep; 14] = [
    InitStep::Object(0x011d_5128, 0x0106_4920, 0x0106_493c),
    InitStep::Object(0x011d_512c, 0x0106_48f4, 0x0106_490c),
    InitStep::Object(0x011d_5130, 0x0106_48c8, 0x0106_48e0),
    InitStep::Object(0x011d_5134, 0x0106_48a4, 0x0106_48c0),
    InitStep::Object(0x011d_5138, 0x0106_4874, 0x0106_4894),
    InitStep::Object(0x011d_513c, 0x0106_4854, 0x0106_486c),
    InitStep::Object(0x011d_5140, 0x0106_482c, 0x0106_4848),
    InitStep::Object(0x011d_5144, 0x0106_47fc, 0x0106_481c),
    InitStep::Object(0x011d_5148, 0x0106_47d0, 0x0106_47ec),
    InitStep::Object(0x011d_514c, 0x0106_47a8, 0x0106_47c4),
    InitStep::Object(0x011d_5150, 0x0106_4784, 0x0106_47a0),
    InitStep::Object(0x011d_5154, 0x0106_476c, 0x0101_a980),
    InitStep::Object(0x011d_5158, 0x0106_4740, 0x0106_475c),
    InitStep::Store(0x011d_515c, 0),
];

// Translated from 00f7fbc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_dynamic_initializer_for_psWeaponTypes_` (engine map; fallout shared/menugamesettings.cpp): 13 objects
/// (`00401000(0xc)`, then `0040c150` on the block with its two addresses; the answer or 0 in the
/// global) and 1 zero words stored after them, in the code's order ([`STEPS_00F7FBC0`], `011d5128` .. `011d515c`).
// Translation incomplete: 00f7fbc0 - FS:[0] exception frame (handler 00efe40f, one state per block, -1 between the blocks) not translated.
pub fn fn_00f7fbc0(e: &mut Engine) {
    run_steps(e, &STEPS_00F7FBC0);
}

/// `NiPointer` construction on a global (`00633c90`, thiscall, `RET 4`: `this`, the pointer).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;

// Translated from 00f8c180 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_dynamic_initializer_for_FaderData_` (engine map): the byte `011d8828` = 1, then three
/// 0x1c-byte records at `011d882c`, `011d8848` and `011d8864`, each: its vtable word
/// (`0106ea2c`, `0106ea2c`, `0106ea20`), four zero words, and `00633c90(0)` on its pointer at
/// +0x14 (`011d8840`, `011d885c`, `011d8878`); after the first two calls the byte that follows
/// the pointer (`011d8844`, `011d8860`) is cleared. Then `_atexit(00fd7ad0)`.
// Translation incomplete: 00f8c180 - FS:[0] exception frame (handler 00f078d4, states 0 and 1) not translated.
pub fn fn_00f8c180(e: &mut Engine) {
    e.set_global::<u8>(0x011d_8828, 1);
    e.set_global::<u32>(0x011d_882c, 0x0106_ea2c);
    for g in [0x011d_8830u32, 0x011d_8834, 0x011d_8838, 0x011d_883c] {
        e.set_global::<u32>(g, 0);
    }
    e.call(NI_POINTER_CONSTRUCT, &args![0x011d_8840u32, 0u32]);
    e.set_global::<u8>(0x011d_8844, 0);
    e.set_global::<u32>(0x011d_8848, 0x0106_ea2c);
    for g in [0x011d_884cu32, 0x011d_8850, 0x011d_8854, 0x011d_8858] {
        e.set_global::<u32>(g, 0);
    }
    e.call(NI_POINTER_CONSTRUCT, &args![0x011d_885cu32, 0u32]);
    e.set_global::<u8>(0x011d_8860, 0);
    e.set_global::<u32>(0x011d_8864, 0x0106_ea20);
    for g in [0x011d_8868u32, 0x011d_886c, 0x011d_8870, 0x011d_8874] {
        e.set_global::<u32>(g, 0);
    }
    e.call(NI_POINTER_CONSTRUCT, &args![0x011d_8878u32, 0u32]);
    e.call(ATEXIT, &args![0x00fd_7ad0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00f7_a190, fn_00f7a190()),
        entry!(0x00f7_e000, fn_00f7e000()),
        entry!(0x00f7_e2b0, fn_00f7e2b0()),
        entry!(0x00f7_ef90, fn_00f7ef90()),
        entry!(0x00f7_fbc0, fn_00f7fbc0()),
        entry!(0x00f8_c180, fn_00f8c180()),
        entry!(0x00fb_3560, fn_00fb3560()),
        entry!(0x00fb_3640, fn_00fb3640()),
        entry!(0x00fb_6c20, fn_00fb6c20()),
        entry!(0x00fb_6cc0, fn_00fb6cc0()),
        entry!(0x00fb_6d60, fn_00fb6d60()),
        entry!(0x00fb_6e00, fn_00fb6e00()),
        entry!(0x00fb_6ea0, fn_00fb6ea0()),
        entry!(0x00fb_6f40, fn_00fb6f40()),
        entry!(0x00fb_6fe0, fn_00fb6fe0()),
        entry!(0x00fb_70b0, fn_00fb70b0()),
        entry!(0x00fb_7150, fn_00fb7150()),
        entry!(0x00fb_71f0, fn_00fb71f0()),
        entry!(0x00fb_7290, fn_00fb7290()),
        entry!(0x00fb_7330, fn_00fb7330()),
        entry!(0x00fb_73d0, fn_00fb73d0()),
        entry!(0x00fb_7470, fn_00fb7470()),
        entry!(0x00fb_7510, fn_00fb7510()),
        entry!(0x00fb_75b0, fn_00fb75b0()),
        entry!(0x00fb_7650, fn_00fb7650()),
        entry!(0x00fb_76f0, fn_00fb76f0()),
        entry!(0x00fb_7790, fn_00fb7790()),
        entry!(0x00fb_7830, fn_00fb7830()),
        entry!(0x00fb_78d0, fn_00fb78d0()),
        entry!(0x00fb_7970, fn_00fb7970()),
        entry!(0x00fb_7a10, fn_00fb7a10()),
        entry!(0x00fb_7ab0, fn_00fb7ab0()),
        entry!(0x00fb_7b50, fn_00fb7b50()),
        entry!(0x00fb_7bf0, fn_00fb7bf0()),
        entry!(0x00fb_7fb0, fn_00fb7fb0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// Every row initializer with its row (the table the functions above read).
    const ROWS: &[(u32, Row)] = &[
        (0x00fb_3640, ROW_00FB3640),
        (0x00fb_6c20, ROW_00FB6C20),
        (0x00fb_6cc0, ROW_00FB6CC0),
        (0x00fb_6d60, ROW_00FB6D60),
        (0x00fb_6e00, ROW_00FB6E00),
        (0x00fb_6ea0, ROW_00FB6EA0),
        (0x00fb_6f40, ROW_00FB6F40),
        (0x00fb_6fe0, ROW_00FB6FE0),
        (0x00fb_70b0, ROW_00FB70B0),
        (0x00fb_7150, ROW_00FB7150),
        (0x00fb_71f0, ROW_00FB71F0),
        (0x00fb_7290, ROW_00FB7290),
        (0x00fb_7330, ROW_00FB7330),
        (0x00fb_73d0, ROW_00FB73D0),
        (0x00fb_7470, ROW_00FB7470),
        (0x00fb_7510, ROW_00FB7510),
        (0x00fb_75b0, ROW_00FB75B0),
        (0x00fb_7650, ROW_00FB7650),
        (0x00fb_76f0, ROW_00FB76F0),
        (0x00fb_7790, ROW_00FB7790),
        (0x00fb_7830, ROW_00FB7830),
        (0x00fb_78d0, ROW_00FB78D0),
        (0x00fb_7970, ROW_00FB7970),
        (0x00fb_7a10, ROW_00FB7A10),
        (0x00fb_7ab0, ROW_00FB7AB0),
        (0x00fb_7b50, ROW_00FB7B50),
        (0x00fb_7bf0, ROW_00FB7BF0),
        (0x00fb_7fb0, ROW_00FB7FB0),
    ];

    /// The slot +4 double of the collection's vtable.
    const ADD: u32 = 0x00aa_0004;

    /// Runs `entry` with the collection of `kind` doubled; returns the calls, the top-level
    /// call filtered out, and the collection's address.
    fn run(e: &mut Engine, entry: u32, kind: Kind) -> (Calls, u32) {
        e.map(kind.instance(), 4);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, ADD);
        let collection = e.mem.alloc(8);
        e.mem.set_u32(collection, vtable);
        e.set_global::<u32>(kind.instance(), collection);
        e.register(kind.getter(), |_, _| Ret::default());
        e.register(ADD, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        let log = e.call_log.take().unwrap();
        (
            log.into_iter().filter(|(a, _)| *a != entry).collect(),
            collection,
        )
    }

    #[test]
    fn each_setting_is_built_and_added_to_its_collection() {
        for &(entry, row) in ROWS {
            let mut e = Engine::new();
            e.map(row.global, 0x0c);
            if let Value::Float(a) = row.value {
                e.map(a, 4);
                // A signalling NaN comes out of FLD/FSTP quieted.
                e.mem.set_u32(a, 0x7f80_0001);
            }
            let (calls, collection) = run(&mut e, entry, row.kind);
            assert_eq!(
                calls,
                vec![
                    (row.kind.getter(), vec![]),
                    (ADD, vec![collection, row.global]),
                    (ATEXIT, vec![row.destructor]),
                ],
                "{entry:08x}"
            );
            assert_eq!(e.mem.u32(row.global), row.kind.vtable(), "{entry:08x}");
            assert_eq!(e.mem.u32(row.global + 8), row.name, "{entry:08x}");
            let want = match row.value {
                Value::Byte(b) => u32::from(b),
                Value::Word(w) => w,
                Value::Float(_) => 0x7fc0_0001,
                Value::Const(bits) => bits,
            };
            let got = match row.value {
                Value::Byte(_) => u32::from(e.mem.u8(row.global + 4)),
                _ => e.mem.u32(row.global + 4),
            };
            assert_eq!(got, want, "{entry:08x}");
        }
    }

    #[test]
    fn the_rows_match_the_disassembly() {
        // Spot checks against the instructions (Ghidra, 1.4.0.525).
        assert_eq!(ROW_00FB6C20.value, Value::Byte(1)); // bEnableAudio: MOV byte ptr [011f6da0],1
        assert_eq!(ROW_00FB7150.kind, Kind::Prefs); // MOV [011f6e14],01022738; CALL 004de4a0
        assert_eq!(ROW_00FB7150.value, Value::Const(0x3f80_0000)); // FLD1
        assert_eq!(ROW_00FB75B0.value, Value::Const(0)); // FLDZ
        assert_eq!(ROW_00FB78D0.value, Value::Word(0x2328));
        assert_eq!(ROW_00FB7B50.name, 0x010a_3924); // iMaxSizeForCachedSound:Audio
        assert_eq!(ROWS.len(), 28);
    }

    #[test]
    fn the_debug_text_colour_is_built_in_a_local_first() {
        let mut e = Engine::new();
        e.map(0x011f_33bc, 0x0c);
        let (calls, collection) = run(&mut e, 0x00fb_3560, Kind::Settings);
        assert_eq!(
            calls,
            vec![
                (0x0044_f570, vec![]),
                (ADD, vec![collection, 0x011f_33bc]),
                (ATEXIT, vec![0x00fd_a2b0]),
            ]
        );
        assert_eq!(e.mem.u32(0x011f_33bc), 0x0101_7720);
        assert_eq!(e.mem.u32(0x011f_33c0), 0xfffb_e9ff);
        assert_eq!(e.mem.u32(0x011f_33c4), 0x0109_505c);
    }

    /// The calls a run of `entry` made with `00401000` answering a block and `0040c150`
    /// answering `block + 1` (the top-level call filtered out).
    fn run_objects(e: &mut Engine, entry: u32) -> Calls {
        e.register(0x0040_1000, |e, _| Ret {
            eax: e.mem.alloc(0x0c),
            ..Ret::default()
        });
        e.register(0x0040_c150, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        let log = e.call_log.take().unwrap();
        log.into_iter().filter(|(a, _)| *a != entry).collect()
    }

    #[test]
    fn the_menu_setting_object_initializers_run_their_steps_in_order() {
        type Case = (u32, &'static [InitStep], usize, usize);
        let cases: [Case; 5] = [
            (0x00f7_a190, &STEPS_00F7A190, 4, 0),
            (0x00f7_e000, &STEPS_00F7E000, 8, 0),
            (0x00f7_e2b0, &STEPS_00F7E2B0, 34, 0),
            (0x00f7_ef90, &STEPS_00F7EF90, 32, 2),
            (0x00f7_fbc0, &STEPS_00F7FBC0, 13, 1),
        ];
        for (entry, steps, objects, stores) in cases {
            let mut e = Engine::new();
            e.map(0x011d_5000, 0x1000);
            for step in steps {
                if let InitStep::Store(g, _) = *step {
                    // A store of 0 shows over a non-zero word.
                    e.mem.set_u32(g, 0xdead_beef);
                }
            }
            let calls = run_objects(&mut e, entry);
            let n_objects = steps
                .iter()
                .filter(|s| matches!(s, InitStep::Object(..)))
                .count();
            assert_eq!((n_objects, steps.len() - n_objects), (objects, stores));
            let mut at = 0;
            for step in steps {
                match *step {
                    InitStep::Object(g, first, second) => {
                        assert_eq!(calls[at], (0x0040_1000, vec![0x0c]), "{entry:08x}");
                        let block = calls[at + 1].1[0];
                        assert_eq!(
                            calls[at + 1],
                            (0x0040_c150, vec![block, first, second]),
                            "{entry:08x}"
                        );
                        assert_eq!(e.mem.u32(g), block + 1, "{entry:08x} {g:08x}");
                        at += 2;
                    }
                    InitStep::Store(g, v) => assert_eq!(e.mem.u32(g), v, "{entry:08x}"),
                }
            }
            assert_eq!(at, calls.len(), "{entry:08x}");
        }
        // First and last steps as the code has them (PUSH second; PUSH first; MOV ECX,block).
        assert_eq!(
            STEPS_00F7A190[0],
            InitStep::Object(0x011d_5730, 0x0106_36e4, 0x0106_36f4)
        );
        assert_eq!(STEPS_00F7FBC0[13], InitStep::Store(0x011d_515c, 0));
    }

    #[test]
    fn fader_data_builds_three_records() {
        let mut e = Engine::new();
        e.map(0x011d_8820, 0x60);
        e.mem.set_u8(0x011d_8844, 0xff);
        e.mem.set_u8(0x011d_8860, 0xff);
        e.register(NI_POINTER_CONSTRUCT, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00f8_c180, &args![]);
        let calls: Calls = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x00f8_c180)
            .collect();
        assert_eq!(
            calls,
            vec![
                (NI_POINTER_CONSTRUCT, vec![0x011d_8840, 0]),
                (NI_POINTER_CONSTRUCT, vec![0x011d_885c, 0]),
                (NI_POINTER_CONSTRUCT, vec![0x011d_8878, 0]),
                (ATEXIT, vec![0x00fd_7ad0]),
            ]
        );
        assert_eq!(e.mem.u8(0x011d_8828), 1);
        assert_eq!(e.mem.u32(0x011d_882c), 0x0106_ea2c);
        assert_eq!(e.mem.u32(0x011d_8848), 0x0106_ea2c);
        assert_eq!(e.mem.u32(0x011d_8864), 0x0106_ea20);
        assert_eq!(e.mem.u8(0x011d_8844), 0);
        assert_eq!(e.mem.u8(0x011d_8860), 0);
    }
}
