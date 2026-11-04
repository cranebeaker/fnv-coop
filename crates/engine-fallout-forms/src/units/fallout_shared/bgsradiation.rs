//! `fallout shared/bgsradiation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The radiation stage array: its count (`011ca87c`) and its pointer array
/// (`011ca880`).
const STAGE_COUNT: u32 = 0x011c_a87c;
const STAGES: u32 = 0x011c_a880;
/// `009611e0` (`thiscall` on a stage): its threshold, an unsigned word.
const STAGE_THRESHOLD: u32 = 0x0096_11e0;

// Translated from 00592220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRadiationStage::FindStageForRadLevel` (Xbox PDB; `cdecl`, one float):
/// the first stage, in array order, whose threshold (`009611e0`, as an
/// unsigned integer loaded with `FILD qword`) is at most `rad_level` (a NaN
/// level matches none); null when none does.
pub fn bgs_radiation_stage_find_stage_for_rad_level(e: &mut Engine, rad_level: f32) -> u32 {
    let mut found = 0u32;
    let mut index = 0u32;
    while index < e.mem.u32(STAGE_COUNT) && found == 0 {
        let stage = fn_005922a0(e, index);
        if stage != 0 {
            let threshold = e.call(STAGE_THRESHOLD, &args![stage]).u32();
            if x87_load32(rad_level.to_bits()) >= f64::from(threshold) {
                found = stage;
            }
        }
        index += 1;
    }
    found
}

// Translated from 005922a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the radiation stage at
/// `index`, or null past the end. (The exe's first test, `index < 0`
/// unsigned, never holds.)
pub fn fn_005922a0(e: &mut Engine, index: u32) -> u32 {
    if index < e.mem.u32(STAGE_COUNT) {
        let stages = e.mem.u32(STAGES);
        e.mem.u32(stages.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00592220,
            bgs_radiation_stage_find_stage_for_rad_level(f32) -> u32
        ),
        entry!(0x005922a0, fn_005922a0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Three stages with thresholds 600, 200 and 400 (the double reads the
    /// word at +0 of the stage).
    fn engine() -> (Engine, [u32; 3]) {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.register(STAGE_THRESHOLD, |e, a| e.mem.u32(a[0]).into_ret());
        let array = e.mem.alloc(12);
        let mut stages = [0u32; 3];
        for (i, threshold) in [600u32, 200, 400].into_iter().enumerate() {
            let stage = e.mem.alloc(4);
            e.mem.set_u32(stage, threshold);
            e.mem.set_u32(array + 4 * i as u32, stage);
            stages[i] = stage;
        }
        e.set_global(STAGE_COUNT, 3u32);
        e.set_global(STAGES, array);
        (e, stages)
    }

    #[test]
    fn stage_lookup_by_index() {
        let (mut e, stages) = engine();
        assert_eq!(e.call(0x0059_22a0, &args![2u32]).u32(), stages[2]);
        assert_eq!(e.call(0x0059_22a0, &args![3u32]).u32(), 0);
    }

    #[test]
    fn first_stage_at_or_below_the_level() {
        let (mut e, stages) = engine();
        assert_eq!(e.call(0x0059_2220, &args![250.0f32]).u32(), stages[1]);
        assert_eq!(e.call(0x0059_2220, &args![600.0f32]).u32(), stages[0]);
        assert_eq!(e.call(0x0059_2220, &args![199.5f32]).u32(), 0);
        assert_eq!(e.call(0x0059_2220, &args![f32::NAN]).u32(), 0);
    }
}
