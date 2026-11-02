//! `fallout/ai/camerashake.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The camera shake keeps a model (`Meshes\Characters\CameraShake.nif`) in
//! a global and, on a shake, stores its strength and duration in two more
//! globals and rumbles the controller.
//!
//! x87: comparisons follow the flag tests of the disassembly (strict `<`
//! and `>`, a NaN fails them); `float` stores round once, the rest is
//! formed in `f64`. A float copied through `FLD`/`FSTP` goes through
//! [`x87_quiet`]; floats pushed as arguments are passed as their bit
//! pattern.

#[allow(unused_imports)]
use crate::prelude::*;

/// The loaded camera-shake model (0 when there is none).
const SHAKE_MODEL: u32 = 0x011d_fecc;
/// The strength of the current shake (`float`).
const SHAKE_STRENGTH: u32 = 0x011d_fed4;
/// The duration of the current shake (`float`, from `fCameraShakeTime`).
const SHAKE_TIME: u32 = 0x011d_fed8;
/// The model loader singleton (`ECX` of `ModelLoader::LoadFile`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The object `00877720` is called on (a pointer global).
const MAIN_OBJECT: u32 = 0x011d_ea0c;

/// `"%s\Characters\CameraShake.nif"`.
const MODEL_PATH_FORMAT: u32 = 0x0108_6900;
/// `"Meshes"`.
const MESHES: u32 = 0x0101_dccc;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `1000.0` (`double`).
const THOUSAND: u32 = 0x0101_7b70;

/// The `Setting`s read here (names from their static initializers).
const CAMERA_SHAKE_DIST_FADE_START: u32 = 0x011d_0a48;
const CAMERA_SHAKE_DIST_FADE_DELTA: u32 = 0x011d_1434;
const CAMERA_SHAKE_DIST_MIN: u32 = 0x011d_0d80;
const CAMERA_SHAKE_MULT_MIN: u32 = 0x011d_09dc;
const CAMERA_SHAKE_TIME: u32 = 0x011d_0df8;
const RUMBLE_SHAKE_TIME_MULT: u32 = 0x011d_064c;
const RUMBLE_SHAKE_RADIUS_MULT: u32 = 0x011d_0b3c;

/// `Setting`'s value pointer (`00403e20`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `_ftol2_sse`: the value in ST0 is a leading `f64` argument.
const FTOL: u32 = 0x00ec_62c0;

/// A float `Setting`'s value, as `FLD` reads it.
fn setting(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

// Translated from 008d1820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d1820 - the FS:[0] exception frame (its unwind destroys the temporary at EBP-0x120) and the /GS cookie check 00ec408c are not translated
/// `InitCameraShake` (Xbox PDB): clears the shake model, and if
/// `Meshes\Characters\CameraShake.nif` exists (`FileFinder::Exist`) loads
/// it (`ModelLoader::LoadFile`) into the global and hands it a temporary
/// made from the path (`00438170`, given to the model's `00a5b950`,
/// destroyed by `004381b0`).
pub fn init_camera_shake(e: &mut Engine) {
    e.with_stack(0x108, |e, frame| {
        let temporary = frame.addr();
        let path = frame.addr() + 4;
        e.set_global(SHAKE_MODEL, 0u32);
        e.call(
            0x0040_6d00,
            &args![path, 0x104u32, MODEL_PATH_FORMAT, MESHES],
        );
        // `FileFinder::Exist` (Xbox PDB).
        let exists = e
            .call(0x0045_6a20, &args![path, 0u32, 0u32, u32::MAX])
            .u32();
        if exists != 0 {
            let loader = e.global::<u32>(MODEL_LOADER);
            // `ModelLoader::LoadFile` (Xbox PDB).
            let model = e
                .call(
                    0x0044_7080,
                    &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
                )
                .u32();
            e.set_global(SHAKE_MODEL, model);
            let made = e.call(0x0043_8170, &args![temporary, path]).u32();
            let model = e.global::<u32>(SHAKE_MODEL);
            e.call(0x00a5_b950, &args![model, made]);
            e.call(0x0043_81b0, &args![temporary]);
        }
    });
}

// Translated from 008d1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the shake model: hands the value `00413f40` / `0043b1b0` get
/// from it to the model loader's `0045a5e0` and clears the global.
pub fn fn_008d1920(e: &mut Engine) {
    let model = e.global::<u32>(SHAKE_MODEL);
    if model == 0 {
        return;
    }
    let first = e.call(0x0041_3f40, &args![model]).u32();
    let second = e.call(0x0043_b1b0, &args![first]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(0x0045_a5e0, &args![loader, second]);
    e.set_global(SHAKE_MODEL, 0u32);
}

// Translated from 008d1960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShakeCamera` (Xbox PDB): starts a shake of `strength` from `source`
/// (a point the vector routine `00439ef0` takes).
///
/// Nothing happens without the shake model or with a strength of 0 or
/// less. The distance from the camera position (`0045c670` →
/// `00558310` → `0043c490`, three plain words) to `source` (`00439ef0`,
/// `00457990`) fades the strength: the start radius is
/// `fCameraShakeDistFadeStart * strength`, at least `fCameraShakeDistMin`,
/// the fade length `fCameraShakeDistFadeDelta * strength`. Inside the
/// radius plus the fade, the strength (faded, at most 1, at least
/// `fCameraShakeMultMin`) and `fCameraShakeTime` are stored, and outside
/// menu mode the controller rumbles (`Controls::Rumble`) with
/// `strength * fRumbleShakeRadiusMult` twice and the time in milliseconds
/// times `fRumbleShakeTimeMult`.
pub fn shake_camera(e: &mut Engine, mut strength: f32, source: u32) {
    let zero: f64 = e.global(ZERO);
    if e.global::<u32>(SHAKE_MODEL) == 0 || strength as f64 <= zero || strength.is_nan() {
        return;
    }
    e.with_stack(0x18, |e, frame| {
        let camera = frame.addr();
        let difference = frame.addr() + 0x0c;
        let first = e.call(0x0045_c670, &[]).u32();
        let second = e.call(0x0055_8310, &args![first]).u32();
        let position = e.call(0x0043_c490, &args![second]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(camera + 4 * i, word);
        }
        let vector = e
            .call(0x0043_9ef0, &args![camera, difference, source])
            .u32();
        let distance = e.call(0x0045_7990, &args![vector]).f32();

        let fade_start = setting(e, CAMERA_SHAKE_DIST_FADE_START);
        let mut radius = (fade_start as f64 * strength as f64) as f32;
        let fade_delta = setting(e, CAMERA_SHAKE_DIST_FADE_DELTA);
        let fade = (fade_delta as f64 * strength as f64) as f32;
        let minimum = setting(e, CAMERA_SHAKE_DIST_MIN);
        if minimum > radius {
            radius = x87_quiet(setting(e, CAMERA_SHAKE_DIST_MIN));
        }
        // `TEST AH,0x5` / `JP`: only a strict `<` goes on (a NaN does not).
        let in_range = (distance as f64) < radius as f64 + fade as f64;
        if !in_range {
            return;
        }
        if radius < distance {
            let faded = 1.0 - (distance as f64 - radius as f64) / fade as f64;
            strength = (faded * strength as f64) as f32;
        }
        let one: f64 = e.global(ONE);
        if strength as f64 > one {
            strength = 1.0;
        } else {
            let minimum = setting(e, CAMERA_SHAKE_MULT_MIN);
            if minimum > strength {
                strength = x87_quiet(setting(e, CAMERA_SHAKE_MULT_MIN));
            }
        }
        e.set_global(SHAKE_STRENGTH, x87_quiet(strength));
        let time = x87_quiet(setting(e, CAMERA_SHAKE_TIME));
        e.set_global(SHAKE_TIME, time);

        // `Interface::IsInMenuMode` (Xbox PDB).
        if e.call(0x0070_2360, &[]).bool() {
            return;
        }
        let main = e.global::<u32>(MAIN_OBJECT);
        let controls = e.call(0x0087_7720, &args![main]).u32();
        let time: f32 = e.global(SHAKE_TIME);
        let thousand: f64 = e.global(THOUSAND);
        let milliseconds = time as f64 * thousand;
        let time_mult = setting(e, RUMBLE_SHAKE_TIME_MULT);
        let duration = e.call(FTOL, &args![time_mult as f64 * milliseconds]).u32();
        let radius_mult = setting(e, RUMBLE_SHAKE_RADIUS_MULT);
        let shake: f32 = e.global(SHAKE_STRENGTH);
        let first_strength = (shake as f64 * radius_mult as f64) as f32;
        let radius_mult = setting(e, RUMBLE_SHAKE_RADIUS_MULT);
        let shake: f32 = e.global(SHAKE_STRENGTH);
        let second_strength = (shake as f64 * radius_mult as f64) as f32;
        // `Controls::Rumble` (Xbox PDB).
        e.call(
            0x00a2_55b0,
            &args![
                controls,
                second_strength.to_bits(),
                first_strength.to_bits(),
                duration,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d1820, init_camera_shake()),
        entry!(0x008d1920, fn_008d1920()),
        entry!(0x008d1960, shake_camera(f32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCRATCH: u32 = 0x0600_0000;
    const CAMERA_POSITION: u32 = SCRATCH + 0x100;
    const CONTROLS: u32 = 0x0510_0000;

    const CALLEES: &[u32] = &[
        SETTING_VALUE,
        FTOL,
        0x0040_6d00,
        0x0045_6a20,
        0x0044_7080,
        0x0043_8170,
        0x00a5_b950,
        0x0043_81b0,
        0x0041_3f40,
        0x0043_b1b0,
        0x0045_a5e0,
        0x0045_c670,
        0x0055_8310,
        0x0043_c490,
        0x0043_9ef0,
        0x0045_7990,
        0x0070_2360,
        0x0087_7720,
        0x00a2_55b0,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &a in CALLEES {
            e.register(a, |_, _| Ret::default());
        }
        for page in [
            0x011d_f000,
            0x011c_3000,
            0x011d_e000,
            0x0101_2000,
            0x0101_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x011d_0000, 0x2000);
        e.map(SCRATCH, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(THOUSAND, 1000.0f64);
        e.set_global(MODEL_LOADER, 0x0520_0000u32);
        e.set_global(MAIN_OBJECT, 0x0530_0000u32);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn init_loads_the_model_only_when_the_file_exists() {
        let mut e = engine();
        e.set_global(SHAKE_MODEL, 0x1234u32);
        e.call_log = Some(vec![]);
        e.call(0x008d_1820, &[]);
        assert_eq!(e.global::<u32>(SHAKE_MODEL), 0);
        let format = logged(&e, 0x0040_6d00);
        assert_eq!(&format[0][1..], &[0x104, MODEL_PATH_FORMAT, MESHES]);
        let path = format[0][0];
        assert_eq!(logged(&e, 0x0045_6a20), vec![vec![path, 0, 0, u32::MAX]]);
        assert!(logged(&e, 0x0044_7080).is_empty());

        e.register(0x0045_6a20, |_, _| 1u32.into_ret());
        e.register(0x0044_7080, |_, _| 0x0540_0000u32.into_ret());
        e.register(0x0043_8170, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008d_1820, &[]);
        assert_eq!(e.global::<u32>(SHAKE_MODEL), 0x0540_0000);
        let path = logged(&e, 0x0040_6d00)[0][0];
        assert_eq!(
            logged(&e, 0x0044_7080),
            vec![vec![0x0520_0000, path, 0, 1, 0, 0, 0]]
        );
        let made = logged(&e, 0x0043_8170);
        assert_eq!(made[0][1], path);
        assert_eq!(logged(&e, 0x00a5_b950), vec![vec![0x0540_0000, 0x77]]);
        assert_eq!(logged(&e, 0x0043_81b0), vec![vec![made[0][0]]]);
    }

    #[test]
    fn release_hands_the_model_back_and_clears_it() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x008d_1920, &[]);
        assert!(logged(&e, 0x0041_3f40).is_empty());

        e.set_global(SHAKE_MODEL, 0x0540_0000u32);
        e.register(0x0041_3f40, |_, _| 0x11u32.into_ret());
        e.register(0x0043_b1b0, |_, _| 0x22u32.into_ret());
        e.call(0x008d_1920, &[]);
        assert_eq!(logged(&e, 0x0041_3f40), vec![vec![0x0540_0000]]);
        assert_eq!(logged(&e, 0x0043_b1b0), vec![vec![0x11]]);
        assert_eq!(logged(&e, 0x0045_a5e0), vec![vec![0x0520_0000, 0x22]]);
        assert_eq!(e.global::<u32>(SHAKE_MODEL), 0);
    }

    /// A shake set-up: settings, the camera position at the origin and
    /// `00457990` returning `distance`.
    fn shake_engine(distance: f32) -> Engine {
        let mut e = engine();
        e.set_global(SHAKE_MODEL, 0x0540_0000u32);
        e.set_global(CAMERA_SHAKE_DIST_FADE_START + 4, 100.0f32);
        e.set_global(CAMERA_SHAKE_DIST_FADE_DELTA + 4, 200.0f32);
        e.set_global(CAMERA_SHAKE_DIST_MIN + 4, 50.0f32);
        e.set_global(CAMERA_SHAKE_MULT_MIN + 4, 0.1f32);
        e.set_global(CAMERA_SHAKE_TIME + 4, 0.5f32);
        e.set_global(RUMBLE_SHAKE_TIME_MULT + 4, 2.0f32);
        e.set_global(RUMBLE_SHAKE_RADIUS_MULT + 4, 0.25f32);
        e.register(0x0043_c490, |_, _| CAMERA_POSITION.into_ret());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register_double(0x0045_7990, move |_, _| distance.into_ret());
        e.register(0x0087_7720, |_, _| CONTROLS.into_ret());
        e.register(FTOL, |_, a| {
            (f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32).into_ret()
        });
        e
    }

    #[test]
    fn shake_fades_with_distance_and_rumbles() {
        let mut e = shake_engine(149.0);
        e.call_log = Some(vec![]);
        // Radius 50 (0.5 * 100, not below the minimum 50), fade 100: the
        // strength fades to 0.005, then is raised to fCameraShakeMultMin.
        e.call(0x008d_1960, &args![0.5f32, 0x99u32]);
        assert_eq!(e.global::<f32>(SHAKE_STRENGTH), 0.1);
        assert_eq!(e.global::<f32>(SHAKE_TIME), 0.5);
        let vector = logged(&e, 0x0043_9ef0);
        assert_eq!(vector[0][2], 0x99);
        let rumble = (0.1f32 as f64 * 0.25) as f32;
        assert_eq!(
            logged(&e, 0x00a2_55b0),
            vec![vec![
                CONTROLS,
                rumble.to_bits(),
                rumble.to_bits(),
                1000,
                0,
                0,
                0,
                0
            ]]
        );
    }

    #[test]
    fn shake_inside_the_radius_is_capped_at_one() {
        let mut e = shake_engine(10.0);
        e.call(0x008d_1960, &args![3.0f32, 0x99u32]);
        assert_eq!(e.global::<f32>(SHAKE_STRENGTH), 1.0);
    }

    #[test]
    fn shake_partly_faded_keeps_the_faded_strength() {
        let mut e = shake_engine(100.0);
        // Radius 50, fade 100: 1 - 50/100 = 0.5 of 0.5.
        e.call(0x008d_1960, &args![0.5f32, 0x99u32]);
        assert_eq!(e.global::<f32>(SHAKE_STRENGTH), 0.25);
    }

    #[test]
    fn shake_out_of_range_or_in_menu_mode_does_not_rumble() {
        let mut e = shake_engine(150.0);
        e.set_global(SHAKE_STRENGTH, 7.0f32);
        e.call_log = Some(vec![]);
        // Radius 50 + fade 100 = 150: not strictly inside.
        e.call(0x008d_1960, &args![0.5f32, 0x99u32]);
        assert_eq!(e.global::<f32>(SHAKE_STRENGTH), 7.0);

        let mut e = shake_engine(f32::NAN);
        e.set_global(SHAKE_STRENGTH, 7.0f32);
        e.call(0x008d_1960, &args![0.5f32, 0x99u32]);
        assert_eq!(e.global::<f32>(SHAKE_STRENGTH), 7.0);

        let mut e = shake_engine(10.0);
        e.register(0x0070_2360, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008d_1960, &args![0.5f32, 0x99u32]);
        assert_eq!(e.global::<f32>(SHAKE_STRENGTH), 0.5);
        assert!(logged(&e, 0x00a2_55b0).is_empty());
    }

    #[test]
    fn shake_needs_the_model_and_a_positive_strength() {
        let mut e = shake_engine(10.0);
        e.call_log = Some(vec![]);
        e.call(0x008d_1960, &args![0.0f32, 0x99u32]);
        e.call(0x008d_1960, &args![f32::NAN, 0x99u32]);
        e.set_global(SHAKE_MODEL, 0u32);
        e.call(0x008d_1960, &args![0.5f32, 0x99u32]);
        assert!(logged(&e, 0x0045_c670).is_empty());
    }
}
