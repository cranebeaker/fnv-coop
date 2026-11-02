//! `fallout shared/run_004d60c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The table of the 0x2b `MiscStat*` pointers (`.data`; `miscstats.cpp`).
const MISC_STAT_TABLE: u32 = 0x011c_6d50;
/// Number of entries in the table.
const MISC_STAT_COUNT: u32 = 0x2b;

// Translated from 004d60c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the `iVal` (+4) of every stat in the `MiscStat` table at
/// `011c6d50` whose entry is set (`cdecl`, no parameters; the engine map
/// gives it no name).
pub fn fn_004d60c0(e: &mut Engine) {
    for index in 0..MISC_STAT_COUNT {
        let stat = e.mem.u32(MISC_STAT_TABLE + index * 4);
        if stat != 0 {
            e.mem.set_u32(stat + 4, 0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004d_60c0, fn_004d60c0())]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_004d60c0_clears_the_set_stats() {
        let mut e = Engine::new();
        e.mem.map(MISC_STAT_TABLE, 0x100);
        let first = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u32(last + 4, 6);
        e.mem.set_u32(MISC_STAT_TABLE, first);
        e.mem.set_u32(MISC_STAT_TABLE + 4 * 0x2a, last);
        e.call(0x004d_60c0, &args![]);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(last + 4), 0);
    }
}
