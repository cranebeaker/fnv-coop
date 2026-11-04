//! `fallout/interface/fohudeffectmanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `FOHUDEffectManager` (Xbox PDB), 0x64 bytes; the PC code uses the
    /// PDB's offsets. Float fields are their bit patterns.
    pub struct FOHUDEffectManager: 0x64 {
        /// `spPartialScreenGeometry` (Xbox PDB): `NiPointer<NiScreenElements>`.
        0x00 spPartialScreenGeometry: u32,
        /// `bShudderActive` (Xbox PDB).
        0x28 bShudderActive: u8,
        /// `ShudderCurrentPosition` (Xbox PDB): `NiPoint2`.
        0x48 ShudderCurrentPositionX: u32,
        0x4C ShudderCurrentPositionY: u32,
        /// `pUnpackShaderParams` (Xbox PDB): `ImageSpaceShaderParam*`.
        0x50 pUnpackShaderParams: u32,
        /// `bHideHUD` (Xbox PDB).
        0x54 bHideHUD: u8,
        /// `fCurrentBlurRadiusHUD` (Xbox PDB).
        0x58 fCurrentBlurRadiusHUD: u32,
        /// `fCurrentScanlineFrequencyHUD` (Xbox PDB).
        0x5C fCurrentScanlineFrequencyHUD: u32,
        /// `fCurrentBlurIntensityHUD` (Xbox PDB).
        0x60 fCurrentBlurIntensityHUD: u32,
    }
}

/// The `NiPointer` constructor (`00633c90`, thiscall: the pointer) and
/// assignment (`0066b0d0`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The `BGSMenuPacker` constructor (`007002d0`, unit run_007001f0).
const MENU_PACKER_CONSTRUCTOR: u32 = 0x0070_02d0;
/// The `NiPoint2` constructor (`006815c0`: nothing, returns `this`).
const POINT2_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ImageSpaceShaderParam::ImageSpaceShaderParam` (`00b8a9e0`) and
/// `ImageSpaceShaderParam::InitConstantMap` (`00b8ab10`, two words) (Xbox
/// PDB names).
const SHADER_PARAM_CONSTRUCTOR: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
/// A float setting's value (`00403e20`, thiscall on the setting: a pointer
/// to the value).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The settings the HUD's scanline frequency, blur intensity and blur
/// radius start from (`011db25c`, `011db238`, `011db200`).
const SCANLINE_FREQUENCY_SETTING: u32 = 0x011d_b25c;
const BLUR_INTENSITY_SETTING: u32 = 0x011d_b238;
const BLUR_RADIUS_SETTING: u32 = 0x011d_b200;
/// The `NiPoint2` the shudder position starts at (`011f4980`, two words).
const START_POSITION: u32 = 0x011f_4980;

/// A float setting's value, copied through the x87 stack.
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

// Translated from 007f7610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOHUDEffectManager::FOHUDEffectManager` (Xbox PDB): constructs the
/// geometry pointer, the menu packer (+4) and the four shudder points; a
/// new `ImageSpaceShaderParam` (0x88 bytes, null when the allocation fails)
/// becomes `pUnpackShaderParams`, with its constant map (`(0, 0)`), its
/// bytes +0x55 and +0x56 cleared and its word +0x24 set to 1; the geometry
/// is dropped, the shudder is off and positioned at the point at
/// `011f4980`, the HUD is shown, and the scanline frequency, blur intensity
/// and blur radius come from their settings. Returns `this`.
// Translation incomplete: 007f7610 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fohud_effect_manager_constructor(
    e: &mut Engine,
    this: Ptr<FOHUDEffectManager>,
) -> Ptr<FOHUDEffectManager> {
    e.call(NI_POINTER_CONSTRUCT, &args![this, 0u32]);
    e.call(MENU_PACKER_CONSTRUCTOR, &args![this.byte_add(4)]);
    for offset in [0x30u32, 0x38, 0x40, 0x48] {
        e.call(POINT2_CONSTRUCTOR, &args![this.byte_add(offset)]);
    }
    let block = e.call(OPERATOR_NEW, &args![0x88u32]).u32();
    let params = if block == 0 {
        0
    } else {
        e.call(SHADER_PARAM_CONSTRUCTOR, &args![block]).u32()
    };
    e.set(this, FOHUDEffectManager::pUnpackShaderParams, params);
    let params = e.get(this, FOHUDEffectManager::pUnpackShaderParams);
    e.call(SHADER_PARAM_INIT_CONSTANT_MAP, &args![params, 0u32, 0u32]);
    let params = e.get(this, FOHUDEffectManager::pUnpackShaderParams);
    e.mem.set_u8(params + 0x55, 0);
    let params = e.get(this, FOHUDEffectManager::pUnpackShaderParams);
    e.mem.set_u8(params + 0x56, 0);
    let params = e.get(this, FOHUDEffectManager::pUnpackShaderParams);
    e.mem.set_u32(params + 0x24, 1);
    e.call(NI_POINTER_ASSIGN, &args![this, 0u32]);
    e.set(this, FOHUDEffectManager::bShudderActive, 0);
    let x = e.global::<u32>(START_POSITION);
    e.set(this, FOHUDEffectManager::ShudderCurrentPositionX, x);
    let y = e.global::<u32>(START_POSITION + 4);
    e.set(this, FOHUDEffectManager::ShudderCurrentPositionY, y);
    e.set(this, FOHUDEffectManager::bHideHUD, 0);
    let frequency = setting_value(e, SCANLINE_FREQUENCY_SETTING);
    e.set(
        this,
        FOHUDEffectManager::fCurrentScanlineFrequencyHUD,
        frequency,
    );
    let intensity = setting_value(e, BLUR_INTENSITY_SETTING);
    e.set(
        this,
        FOHUDEffectManager::fCurrentBlurIntensityHUD,
        intensity,
    );
    let radius = setting_value(e, BLUR_RADIUS_SETTING);
    e.set(this, FOHUDEffectManager::fCurrentBlurRadiusHUD, radius);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x007f_7610,
        fohud_effect_manager_constructor(Ptr<FOHUDEffectManager>) -> Ptr<FOHUDEffectManager>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            NI_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            MENU_PACKER_CONSTRUCTOR,
            POINT2_CONSTRUCTOR,
            SHADER_PARAM_INIT_CONSTANT_MAP,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(SHADER_PARAM_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        // Each setting's value is the float at the setting + 4.
        e.register(SETTING_FLOAT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        for (setting, value) in [
            (SCANLINE_FREQUENCY_SETTING, 1.5f32),
            (BLUR_INTENSITY_SETTING, 2.5),
            (BLUR_RADIUS_SETTING, 3.5),
        ] {
            e.map(setting, 8);
            e.mem.set_f32(setting + 4, value);
        }
        e.map(START_POSITION, 8);
        e.set_global(START_POSITION, 0x1111_1111u32);
        e.set_global(START_POSITION + 4, 0x2222_2222u32);
        e
    }

    #[test]
    fn fohud_effect_manager_constructor_sets_up_the_effects() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let this = e.new_object::<FOHUDEffectManager>();
        e.set(this, FOHUDEffectManager::bHideHUD, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_7610, &args![this]).u32(), this.addr());
        let t = this.addr();
        assert_eq!(calls_to(&e, NI_POINTER_CONSTRUCT), vec![vec![t, 0]]);
        assert_eq!(calls_to(&e, MENU_PACKER_CONSTRUCTOR), vec![vec![t + 4]]);
        assert_eq!(
            calls_to(&e, POINT2_CONSTRUCTOR),
            vec![
                vec![t + 0x30],
                vec![t + 0x38],
                vec![t + 0x40],
                vec![t + 0x48]
            ]
        );
        let params = e.get(this, FOHUDEffectManager::pUnpackShaderParams);
        assert_ne!(params, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x88]]);
        assert_eq!(
            calls_to(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![vec![params, 0, 0]]
        );
        assert_eq!(e.mem.u32(params + 0x24), 1);
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN), vec![vec![t, 0]]);
        assert_eq!(
            e.get(this, FOHUDEffectManager::ShudderCurrentPositionX),
            0x1111_1111
        );
        assert_eq!(
            e.get(this, FOHUDEffectManager::ShudderCurrentPositionY),
            0x2222_2222
        );
        assert_eq!(e.get(this, FOHUDEffectManager::bHideHUD), 0);
        assert_eq!(
            e.get(this, FOHUDEffectManager::fCurrentScanlineFrequencyHUD),
            1.5f32.to_bits()
        );
        assert_eq!(
            e.get(this, FOHUDEffectManager::fCurrentBlurIntensityHUD),
            2.5f32.to_bits()
        );
        assert_eq!(
            e.get(this, FOHUDEffectManager::fCurrentBlurRadiusHUD),
            3.5f32.to_bits()
        );
    }

    #[test]
    fn fohud_effect_manager_constructor_writes_through_a_null_parameter_block() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        // The code writes the parameter block's fields without a null test.
        e.map(0, 0x60);
        let this = e.new_object::<FOHUDEffectManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_7610, &args![this]);
        assert!(calls_to(&e, SHADER_PARAM_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u32(0x24), 1);
    }
}
