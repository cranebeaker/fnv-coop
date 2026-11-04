//! `fallout shared/run_006167d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here work on a `TESReputation` form (fields read at their PC
//! offsets, which are 0x10 lower than the Xbox PDB's: `TESForm` is 0x18 bytes
//! on PC): `REPUTATION_DATA::fMaximumValue` at +0x40, `fPositiveValue` at
//! +0x44 and `fNegativeValue` at +0x48.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` value accessor (`00403e20`, `this` = the setting): the
/// address of its value (+4), or of a zeroed static when `this` is null.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `fReputationThresholdOne` (the name its static initializer `00f53580`
/// gives it).
const THRESHOLD_ONE: u32 = 0x011c_ba7c;
/// `fReputationThresholdTwo` (static initializer `00f535b0`).
const THRESHOLD_TWO: u32 = 0x011c_bc70;
/// `fReputationThresholdThree` (static initializer `00f535e0`).
const THRESHOLD_THREE: u32 = 0x011c_bd68;

/// The float value of a `Setting`, read through its accessor (`FLD dword`).
fn setting_float(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

// Translated from 00616950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation level (0 to 3) of one side of a `TESReputation`:
/// `positive` 1 uses the positive fraction (`00616a20`), anything else the
/// negative one (`00616a40`). Level 1 when the fraction is at least
/// `fReputationThresholdOne` and below `...Two`, 2 when at least `...Two` and
/// below `...Three`, 3 when at least `...Three`, otherwise 0 (a NaN on
/// either side of a comparison fails it).
pub fn fn_00616950(e: &mut Engine, this: Ptr, positive: i32) -> u8 {
    let fraction = if positive == 1 {
        fn_00616a20(e, this)
    } else {
        fn_00616a40(e, this)
    };
    // FSTP float, then FLD float for each comparison.
    let fraction = x87_load32(fraction.to_bits());
    if setting_float(e, THRESHOLD_ONE) <= fraction && setting_float(e, THRESHOLD_TWO) > fraction {
        return 1;
    }
    if setting_float(e, THRESHOLD_TWO) <= fraction && setting_float(e, THRESHOLD_THREE) > fraction {
        return 2;
    }
    if setting_float(e, THRESHOLD_THREE) <= fraction {
        return 3;
    }
    0
}

// Translated from 00616a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fPositiveValue / fMaximumValue` of a `TESReputation`, stored as a float.
pub fn fn_00616a20(e: &mut Engine, this: Ptr) -> f32 {
    // TESReputation::fPositiveValue (Xbox PDB +0x54) +0x44, fMaximumValue +0x40.
    let positive = x87_load32(e.mem.u32(this.addr() + 0x44));
    let maximum = x87_m32(e.mem.u32(this.addr() + 0x40));
    f32::from_bits(x87_quiet_bits(x87_store32(x87_div(positive, maximum))))
}

// Translated from 00616a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fNegativeValue / fMaximumValue` of a `TESReputation`, stored as a float.
pub fn fn_00616a40(e: &mut Engine, this: Ptr) -> f32 {
    // TESReputation::fNegativeValue (Xbox PDB +0x58) +0x48, fMaximumValue +0x40.
    let negative = x87_load32(e.mem.u32(this.addr() + 0x48));
    let maximum = x87_m32(e.mem.u32(this.addr() + 0x40));
    f32::from_bits(x87_quiet_bits(x87_store32(x87_div(negative, maximum))))
}

// Translated from 00616a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reputation title index (0 to 6) from the positive level (`00616950`
/// with 1) and the negative level (with 0), for one of three tables picked
/// by `table` (0, 1 or 2; any other value gives 0 unless both levels are 0).
/// Both levels 0 give 1; a combination a table does not list gives 0.
pub fn fn_00616a90(e: &mut Engine, this: Ptr, table: i8) -> u8 {
    let positive = fn_00616950(e, this, 1) as i8;
    let negative = fn_00616950(e, this, 0) as i8;
    if positive == 0 && negative == 0 {
        return 1;
    }
    match table {
        0 => {
            if positive == 1 && negative == 1 {
                return 3;
            }
            if positive == 3 {
                if negative == 2 {
                    return 2;
                }
                if negative == 3 {
                    return 5;
                }
            }
            if positive == 2 {
                if negative == 3 {
                    return 2;
                }
                if negative == 2 {
                    return 4;
                }
            }
        }
        1 => {
            if positive == 1 && negative == 0 {
                return 4;
            }
            if positive == 2 {
                if negative == 0 {
                    return 5;
                }
                if negative == 1 {
                    return 2;
                }
            }
            if positive == 3 {
                if negative == 0 {
                    return 6;
                }
                if negative == 1 {
                    return 3;
                }
            }
        }
        2 => {
            if negative == 1 && positive == 0 {
                return 4;
            }
            if negative == 2 {
                if positive == 0 {
                    return 5;
                }
                if positive == 1 {
                    return 2;
                }
            }
            if negative == 3 {
                if positive == 0 {
                    return 6;
                }
                if positive == 1 {
                    return 3;
                }
            }
        }
        _ => {}
    }
    0
}

/// `00403df0` (`ECX` = setting): the setting's string.
const SETTING_STRING: u32 = 0x0040_3df0;
/// The 16 settings `006167d0` picks from (row: the negative level, column:
/// the positive level), and the 16 `00616890` picks from.
const TABLE_006167D0: [u32; 16] = [
    0x011c_bbfc,
    0x011c_bbcc,
    0x011c_baec,
    0x011c_bd44,
    0x011c_bcf0,
    0x011c_ba4c,
    0x011c_bb4c,
    0x011c_bb8c,
    0x011c_bb80,
    0x011c_ba18,
    0x011c_bab0,
    0x011c_bd08,
    0x011c_bb40,
    0x011c_baa0,
    0x011c_ba64,
    0x011c_bd38,
];
const TABLE_00616890: [u32; 16] = [
    0x011c_bd94,
    0x011c_bba4,
    0x011c_bc94,
    0x011c_ba40,
    0x011c_bbe4,
    0x011c_bcd4,
    0x011c_bd18,
    0x011c_bc7c,
    0x011c_bb64,
    0x011c_bc28,
    0x011c_bb10,
    0x011c_bbbc,
    0x011c_bc1c,
    0x011c_bae0,
    0x011c_bd50,
    0x011c_bca0,
];

/// The string of the setting a 4x4 table on the stack (at `EBP - 0x48`)
/// gives for the reputation's levels: the word at `negative * 16 +
/// positive * 4` from the table (the levels as signed bytes; the exe makes
/// no range check, so the read is translated as it is).
fn level_table_string(e: &mut Engine, this: Ptr, table: &[u32; 16], frame_size: u32) -> u32 {
    e.with_stack(frame_size, |e, frame| {
        let ebp = frame.addr() + frame_size;
        let start = ebp - 0x48;
        for (i, setting) in table.iter().enumerate() {
            e.mem.set_u32(start + 4 * i as u32, *setting);
        }
        let positive = fn_00616950(e, this, 1) as i8;
        let negative = fn_00616950(e, this, 0) as i8;
        let at = start
            .wrapping_add((i32::from(negative) << 4) as u32)
            .wrapping_add((i32::from(positive) * 4) as u32);
        let setting = e.mem.u32(at);
        e.call(SETTING_STRING, &args![setting]).u32()
    })
}

// Translated from 006167d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's picture string: the setting a 4x4 table gives for the
/// negative (row) and positive (column) levels (`00616950`), through
/// `00403df0`.
pub fn fn_006167d0(e: &mut Engine, this: Ptr) -> u32 {
    level_table_string(e, this, &TABLE_006167D0, 0x4c)
}

// Translated from 00616890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's level text: as `006167d0` with the second table.
pub fn fn_00616890(e: &mut Engine, this: Ptr) -> u32 {
    level_table_string(e, this, &TABLE_00616890, 0x50)
}

// Translated from 00616a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The positive fraction (`00616a20`) for `which` 1, else the negative one
/// (`00616a40`), in ST0 (`RET 4`).
pub fn fn_00616a60(e: &mut Engine, this: Ptr, which: i32) -> f32 {
    if which == 1 {
        fn_00616a20(e, this)
    } else {
        fn_00616a40(e, this)
    }
}

// Translated from 00616c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Raises the reputation's negative side (`00615730` with 0) by the
/// setting `011cbaf8` when `large` is set, else `011cbb1c` (`RET 4`).
pub fn fn_00616c20(e: &mut Engine, this: Ptr, large: u8) {
    let setting = if large != 0 {
        0x011c_baf8u32
    } else {
        0x011c_bb1cu32
    };
    let at = e.call(SETTING_VALUE, &args![setting]).u32();
    let amount = x87_quiet_bits(e.mem.u32(at));
    e.call(0x0061_5730, &args![this, 0u32, amount]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00616950, fn_00616950(Ptr, i32) -> u8),
        entry!(0x00616a20, fn_00616a20(Ptr) -> f32),
        entry!(0x00616a40, fn_00616a40(Ptr) -> f32),
        entry!(0x00616a90, fn_00616a90(Ptr, i8) -> u8),
        entry!(0x0061_67d0, fn_006167d0(Ptr) -> u32),
        entry!(0x0061_6890, fn_00616890(Ptr) -> u32),
        entry!(0x0061_6a60, fn_00616a60(Ptr, i32) -> f32),
        entry!(0x0061_6c20, fn_00616c20(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A reputation with the given maximum, positive and negative values,
    /// and the three thresholds 0.25, 0.5 and 0.75 behind a double of the
    /// setting accessor.
    fn reputation(e: &mut Engine, maximum: f32, positive: f32, negative: f32) -> Ptr {
        e.register(SETTING_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.map(0x011c_b000, 0x1000);
        e.set_global(THRESHOLD_ONE + 4, 0.25f32);
        e.set_global(THRESHOLD_TWO + 4, 0.5f32);
        e.set_global(THRESHOLD_THREE + 4, 0.75f32);
        let form = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_f32(form.addr() + 0x40, maximum);
        e.mem.set_f32(form.addr() + 0x44, positive);
        e.mem.set_f32(form.addr() + 0x48, negative);
        form
    }

    #[test]
    fn fractions_divide_by_the_maximum() {
        let mut e = Engine::new();
        let form = reputation(&mut e, 40.0, 10.0, 30.0);
        assert_eq!(e.call(0x0061_6a20, &args![form]).f32(), 0.25);
        assert_eq!(e.call(0x0061_6a40, &args![form]).f32(), 0.75);
    }

    #[test]
    fn levels_follow_the_thresholds() {
        let mut e = Engine::new();
        let form = reputation(&mut e, 100.0, 10.0, 25.0);
        assert_eq!(e.call(0x0061_6950, &args![form, 1u32]).u8(), 0);
        assert_eq!(e.call(0x0061_6950, &args![form, 0u32]).u8(), 1);
        e.mem.set_f32(form.addr() + 0x44, 50.0);
        assert_eq!(e.call(0x0061_6950, &args![form, 1u32]).u8(), 2);
        e.mem.set_f32(form.addr() + 0x44, 90.0);
        assert_eq!(e.call(0x0061_6950, &args![form, 1u32]).u8(), 3);
        // 0 / 0 is NaN: every comparison fails.
        let empty = reputation(&mut e, 0.0, 0.0, 0.0);
        assert_eq!(e.call(0x0061_6950, &args![empty, 1u32]).u8(), 0);
    }

    #[test]
    fn titles_come_from_the_two_levels() {
        let mut e = Engine::new();
        let form = reputation(&mut e, 100.0, 0.0, 0.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 0u32]).u8(), 1);
        // Positive level 3, negative level 2.
        e.mem.set_f32(form.addr() + 0x44, 80.0);
        e.mem.set_f32(form.addr() + 0x48, 60.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 0u32]).u8(), 2);
        assert_eq!(e.call(0x0061_6a90, &args![form, 1u32]).u8(), 0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 7u32]).u8(), 0);
        // Positive 2, negative 0.
        e.mem.set_f32(form.addr() + 0x44, 60.0);
        e.mem.set_f32(form.addr() + 0x48, 0.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 1u32]).u8(), 5);
        // Positive 0, negative 3.
        e.mem.set_f32(form.addr() + 0x44, 0.0);
        e.mem.set_f32(form.addr() + 0x48, 80.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 2u32]).u8(), 6);
        // Positive 1, negative 1.
        e.mem.set_f32(form.addr() + 0x44, 30.0);
        e.mem.set_f32(form.addr() + 0x48, 30.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 0u32]).u8(), 3);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    /// A reputation with the fractions `positive` / 100 and `negative` /
    /// 100 and thresholds 0.25, 0.5 and 0.75.
    fn engine(positive: f32, negative: f32) -> (Engine, u32) {
        let mut e = Engine::new();
        let values = e.mem.alloc(0x10);
        for (i, threshold) in [0.25f32, 0.5, 0.75].into_iter().enumerate() {
            e.mem.set_u32(values + 4 * i as u32, threshold.to_bits());
        }
        e.register_double(SETTING_VALUE, move |_, a| {
            let i = match a[0] {
                THRESHOLD_ONE => 0,
                THRESHOLD_TWO => 1,
                THRESHOLD_THREE => 2,
                _ => 3,
            };
            (values + 4 * i).into_ret()
        });
        e.register(SETTING_STRING, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x40, 100.0f32.to_bits());
        e.mem.set_u32(this + 0x44, positive.to_bits());
        e.mem.set_u32(this + 0x48, negative.to_bits());
        (e, this)
    }

    #[test]
    fn level_tables_pick_by_both_levels() {
        // Positive level 2 (0.6), negative level 1 (0.3): row 1, column 2.
        let (mut e, this) = engine(60.0, 30.0);
        assert_eq!(e.call(0x0061_67d0, &args![this]).u32(), 0x011c_bb4d);
        assert_eq!(e.call(0x0061_6890, &args![this]).u32(), 0x011c_bd19);
        // Both 0: the first entry; both 3: the last.
        let (mut e, this) = engine(0.0, 0.0);
        assert_eq!(e.call(0x0061_67d0, &args![this]).u32(), 0x011c_bbfd);
        let (mut e, this) = engine(90.0, 80.0);
        assert_eq!(e.call(0x0061_6890, &args![this]).u32(), 0x011c_bca1);
    }

    #[test]
    fn fraction_by_side() {
        let (mut e, this) = engine(50.0, 25.0);
        assert_eq!(e.call(0x0061_6a60, &args![this, 1u32]).f32(), 0.5);
        assert_eq!(e.call(0x0061_6a60, &args![this, 0u32]).f32(), 0.25);
        assert_eq!(e.call(0x0061_6a60, &args![this, 2u32]).f32(), 0.25);
    }

    #[test]
    fn raise_by_setting_passes_the_float() {
        let mut e = Engine::new();
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x7f80_0001);
        e.register_double(SETTING_VALUE, move |_, _| value.into_ret());
        e.register(0x0061_5730, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_6c20, &args![0x5000u32, 0x100u32]);
        e.call(0x0061_6c20, &args![0x5000u32, 1u32]);
        let log = e.call_log.take().unwrap();
        let settings: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == SETTING_VALUE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(settings, vec![0x011c_bb1c, 0x011c_baf8]);
        let raised: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0061_5730)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(raised, vec![vec![0x5000, 0, 0x7fc0_0001]; 2]);
    }
}
