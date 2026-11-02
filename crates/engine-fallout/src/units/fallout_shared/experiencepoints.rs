//! `fallout shared/experiencepoints.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043d4d0` (`thiscall`, no stack word; unit `fallout shared/modelloader.cpp`): the address
/// of the integer value of the object it is called on.
const VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// The five tables of five (threshold, value) object pairs `006705b0` chooses from, by kind.
const EXPERIENCE_TABLES: [u32; 5] = [
    0x0119_bbcc,
    0x0119_bbf4,
    0x0119_bc1c,
    0x0119_bc44,
    0x0119_bc6c,
];

// Translated from 006705b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExperiencePoints::GetExperiencePoints` (Xbox PDB; `cdecl`, two words): for `kind` 0 to 4
/// (the compiler's jump table at `00670654`), the experience [`fn_00670670`] picks for
/// `level` from that kind's table of five pairs (`0119bbcc`, `0119bbf4`, `0119bc1c`,
/// `0119bc44`, `0119bc6c`); 0 for any other kind.
pub fn experience_points_get_experience_points(e: &mut Engine, kind: u32, level: i32) -> i32 {
    match kind {
        0 => fn_00670670(e, level, EXPERIENCE_TABLES[0], 5),
        1 => fn_00670670(e, level, EXPERIENCE_TABLES[1], 5),
        2 => fn_00670670(e, level, EXPERIENCE_TABLES[2], 5),
        3 => fn_00670670(e, level, EXPERIENCE_TABLES[3], 5),
        4 => fn_00670670(e, level, EXPERIENCE_TABLES[4], 5),
        _ => 0,
    }
}

/// The integer value of the object at `object` (`0043d4d0`).
fn object_value(e: &mut Engine, object: u32) -> i32 {
    let address = e.call(VALUE_ADDRESS, &args![object]).u32();
    e.mem.i32(address)
}

// Translated from 00670670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `experiencepoints.cpp`; `cdecl`, three words): walks the
/// first `count` (threshold, value) object pairs of `table` (8 bytes each) until one is
/// chosen: the first pair whose threshold's value is at least `level` gives the result (its
/// value's value); every pair passed on the way (threshold below `level`) leaves its value's
/// value as the fallback. Returns the chosen value when it is not negative, otherwise the last
/// fallback (0 when no pair was passed). All comparisons are signed.
pub fn fn_00670670(e: &mut Engine, level: i32, table: u32, count: i32) -> i32 {
    let mut chosen = -1i32;
    let mut fallback = 0i32;
    let mut index = 0i32;
    while index < count && chosen < 0 {
        let pair = table.wrapping_add((index as u32).wrapping_mul(8));
        let threshold_object = e.mem.u32(pair);
        let threshold = object_value(e, threshold_object);
        let value_object = e.mem.u32(pair + 4);
        if level > threshold {
            fallback = object_value(e, value_object);
        } else {
            chosen = object_value(e, value_object);
        }
        index = index.wrapping_add(1);
    }
    if chosen < 0 {
        fallback
    } else {
        chosen
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_05b0, experience_points_get_experience_points(u32, i32) -> i32),
        entry!(0x0067_0670, fn_00670670(i32, u32, i32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pairs whose objects are their own value addresses: object `x` holds its value.
    fn table(e: &mut Engine, pairs: &[(i32, i32)]) -> u32 {
        let table = e.mem.alloc(8 * pairs.len() as u32);
        for (index, &(threshold, value)) in pairs.iter().enumerate() {
            let threshold_object = e.mem.alloc(4);
            let value_object = e.mem.alloc(4);
            e.mem.set_i32(threshold_object, threshold);
            e.mem.set_i32(value_object, value);
            e.mem.set_u32(table + 8 * index as u32, threshold_object);
            e.mem.set_u32(table + 8 * index as u32 + 4, value_object);
        }
        table
    }

    #[test]
    fn picks_the_first_threshold_at_or_above_the_level() {
        let mut e = Engine::new();
        e.register(VALUE_ADDRESS, |_, a| a[0].into_ret());
        let pairs = table(&mut e, &[(2, 10), (5, 20), (9, 30)]);
        let pick = |e: &mut Engine, level: i32, count: i32| {
            e.call(0x0067_0670, &args![level, pairs, count]).i32()
        };
        assert_eq!(pick(&mut e, 1, 3), 10);
        assert_eq!(pick(&mut e, 5, 3), 20);
        assert_eq!(pick(&mut e, 6, 3), 30);
        // Past every threshold: the last value passed.
        assert_eq!(pick(&mut e, 12, 3), 30);
        assert_eq!(pick(&mut e, 12, 2), 20);
        assert_eq!(pick(&mut e, 1, 0), 0);
        // A negative chosen value keeps walking.
        let negative = table(&mut e, &[(5, -1), (9, 40)]);
        assert_eq!(e.call(0x0067_0670, &args![1i32, negative, 2i32]).i32(), 40);
    }

    #[test]
    fn get_experience_points_uses_the_kinds_table() {
        let mut e = Engine::new();
        e.map(0x0119_b000, 0x1000);
        e.register(VALUE_ADDRESS, |_, a| a[0].into_ret());
        // Every table of kind 2: thresholds and values in a scratch block.
        let values = e.mem.alloc(8);
        e.mem.set_i32(values, 100);
        e.mem.set_i32(values + 4, 7);
        for index in 0..5u32 {
            e.mem.set_u32(EXPERIENCE_TABLES[2] + 8 * index, values);
            e.mem
                .set_u32(EXPERIENCE_TABLES[2] + 8 * index + 4, values + 4);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_05b0, &args![2u32, 3i32]).i32(), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == VALUE_ADDRESS).count(), 2);
        assert_eq!(e.call(0x0067_05b0, &args![5u32, 3i32]).i32(), 0);
    }
}
