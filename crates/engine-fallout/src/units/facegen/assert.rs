//! `FaceGen/assert.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005b5e40` (unnamed; `cdecl` with a format and its arguments): where the assert's
/// message goes, and the format `"DEFAULT: FR2 ASSERT violation in %s line %i. Code may
/// crash."` (`010a2c9c`).
const PRINT_005B5E40: u32 = 0x005b_5e40;
const ASSERT_FORMAT: u32 = 0x010a_2c9c;
/// The error format `"DEFAULT: FR2 ERROR in %s line %i : '%s'. Code may crash."` (`010a2cdc`).
const ERROR_FORMAT: u32 = 0x010a_2cdc;

// Translated from 00aafb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fr2Assert` (engine map; `cdecl`, two words: the source file name and the line):
/// `005b5e40` with the format at `010a2c9c`, `file` and `line`.
pub fn fr2_assert(e: &mut Engine, file: u32, line: u32) {
    e.call(PRINT_005B5E40, &args![ASSERT_FORMAT, file, line]);
}

// Translated from 00aafbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fr2Error` (engine map; `cdecl`, three words: the source file name, the line and a message):
/// `005b5e40` with the format at `010a2cdc`, `file`, `line` and `message`.
pub fn fr2_error(e: &mut Engine, file: u32, line: u32, message: u32) {
    e.call(PRINT_005B5E40, &args![ERROR_FORMAT, file, line, message]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_fb90, fr2_assert(u32, u32)),
        entry!(0x00aa_fbb0, fr2_error(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_prints_the_file_and_line() {
        let mut e = Engine::new();
        e.register(PRINT_005B5E40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_fb90, &args![0x0106_5830u32, 0x113u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(PRINT_005B5E40, vec![ASSERT_FORMAT, 0x0106_5830, 0x113])]
        );
    }

    #[test]
    fn error_prints_the_file_line_and_message() {
        let mut e = Engine::new();
        e.register(PRINT_005B5E40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_fbb0, &args![0x010a_2f3cu32, 0u32, 0x0104_df38u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(
                PRINT_005B5E40,
                vec![ERROR_FORMAT, 0x010a_2f3c, 0, 0x0104_df38]
            )]
        );
    }
}
