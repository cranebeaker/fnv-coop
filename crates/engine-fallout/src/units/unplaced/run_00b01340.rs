//! `(unplaced)/run_00b01340` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b01340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0 and ignores its two stack words (slot +0x0C of `BSFileCache`'s
/// vtable `010a47cc`, by its RTTI).
pub fn fn_00b01340(_e: &mut Engine, _this: u32, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00b01350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false and ignores its stack word (slot +0x1C and +0x34 of
/// `BSFileCache`'s vtable, +0x48 of `BSWin32GameSound`'s and +0x10 of
/// `BSSystemUtility`'s, by their RTTI).
pub fn fn_00b01350(_e: &mut Engine, _this: u32, _unused_1: u32) -> bool {
    false
}

// Translated from 00b01360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking four words it does not read (slot +0x2C of
/// `BSFileCache`'s vtable, by its RTTI).
pub fn fn_00b01360(
    _e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_1340, fn_00b01340(u32, u32, u32) -> u32),
        entry!(0x00b0_1350, fn_00b01350(u32, u32) -> bool),
        entry!(0x00b0_1360, fn_00b01360(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_answers() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1340, &[0x1000, 1, 2]).u32(), 0);
        assert!(!e.call(0x00b0_1350, &[0x1000, 1]).bool());
        e.call(0x00b0_1360, &[0x1000, 1, 2, 3, 4]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }
}
