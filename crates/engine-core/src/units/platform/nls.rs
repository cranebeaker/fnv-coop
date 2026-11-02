//! The platform's character-type call `GetStringTypeW` (KERNEL32 import), which the C runtime's
//! locale start-up reaches (`__initmbctable` > `__setmbcp`, `__init_ctype`, `__isctype_l`,
//! `__iswctype_l`, through `__crtGetStringTypeA_stat` and `__crtGetStringTypeW`). Every caller in
//! the exe asks for `CT_CTYPE1` with a positive count (`__crtGetStringTypeA_stat` first probes
//! with one unit of the empty wide text at `01011d78`).
//!
//! Measured on this machine (2026-10-27, `build43-scratch/measure/platform5`, a 32-bit program
//! calling the real function):
//! - `CT_CTYPE1` for every UTF-16 code unit of the BMP is `ctype1_measured.tsv` (each unit's
//!   word the same in one call over all of them as in one call per unit); a call writes one word
//!   per unit, answers TRUE and leaves the last error as it was;
//! - a negative count reads through the text's zero, whose word is written too (-1 and -2 alike);
//! - count 0, or a null text, answers FALSE with `ERROR_INVALID_PARAMETER` (87), nothing written;
//! - an info type other than `CT_CTYPE1` (1), `CT_CTYPE2` (2) and `CT_CTYPE3` (4) answers FALSE
//!   with `ERROR_INVALID_FLAGS` (1004), nothing written. `CT_CTYPE2` and `CT_CTYPE3` answer TRUE
//!   but their tables are not measured: not provided (no caller in the exe asks for them).

use crate::{Engine, Ptr};
use std::sync::OnceLock;

/// `CT_CTYPE1`, `CT_CTYPE2`, `CT_CTYPE3`.
pub const CT_CTYPE1: u32 = 1;
pub const CT_CTYPE2: u32 = 2;
pub const CT_CTYPE3: u32 = 4;
/// `ERROR_INVALID_PARAMETER` and `ERROR_INVALID_FLAGS`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_INVALID_FLAGS: u32 = 1004;

/// The measured `CT_CTYPE1` word of each BMP code unit (`ctype1_measured.tsv`).
pub fn ctype1(unit: u16) -> u16 {
    static TABLE: OnceLock<Vec<u16>> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut table = vec![0u16; 0x1_0000];
        let mut covered = 0usize;
        for line in include_str!("ctype1_measured.tsv")
            .lines()
            .filter(|l| !l.starts_with('#') && !l.is_empty())
        {
            let f: Vec<u32> = line
                .split('\t')
                .map(|w| u32::from_str_radix(w, 16).expect("a hex word in ctype1_measured.tsv"))
                .collect();
            for unit in f[0]..=f[1] {
                table[unit as usize] = f[2] as u16;
                covered += 1;
            }
        }
        assert_eq!(covered, 0x1_0000, "ctype1_measured.tsv covers the BMP once");
        table
    });
    table[unit as usize]
}

// Platform: replaces KERNEL32.DLL GetStringTypeW (import)
/// `GetStringTypeW(info, text, count, types)`: for `CT_CTYPE1` the measured word of each unit
/// of `text` (`count` units, or through its zero when negative) into `types`, TRUE; the edge
/// cases as measured (module comment).
pub fn get_string_type_w(e: &mut Engine, info: u32, text: Ptr, count: i32, types: Ptr) -> u32 {
    if info != CT_CTYPE1 && info != CT_CTYPE2 && info != CT_CTYPE3 {
        e.kernel.last_error = ERROR_INVALID_FLAGS;
        return 0;
    }
    if count == 0 || text.is_null() {
        e.kernel.last_error = ERROR_INVALID_PARAMETER;
        return 0;
    }
    assert!(
        info == CT_CTYPE1,
        "GetStringTypeW({info}): only CT_CTYPE1 is measured (not provided)"
    );
    let mut i = 0u32;
    loop {
        if count > 0 && i >= count as u32 {
            break;
        }
        let unit = e.mem.u16(text.addr().wrapping_add(2 * i));
        e.mem
            .set_u16(types.addr().wrapping_add(2 * i), ctype1(unit));
        i += 1;
        if count < 0 && unit == 0 {
            break;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(e: &mut Engine, text: &str) -> u32 {
        let units: Vec<u16> = text.encode_utf16().collect();
        let at = e.mem.alloc(2 * units.len() as u32 + 2);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *u);
        }
        at
    }

    /// The measured answers for "Ab 1" and its zero.
    #[test]
    fn ctype1_answers_the_measured_words() {
        let mut e = Engine::new();
        let text = wide(&mut e, "Ab 1\0zz");
        let out = e.mem.alloc(16);
        e.mem.write(out, &[0xaa; 16]);
        e.kernel.last_error = 0x1234;
        assert_eq!(
            get_string_type_w(&mut e, 1, Ptr::new(text), -1, Ptr::new(out)),
            1
        );
        let words: Vec<u16> = (0..6).map(|i| e.mem.u16(out + 2 * i)).collect();
        assert_eq!(words, [0x381, 0x382, 0x248, 0x284, 0x220, 0xaaaa]);
        assert_eq!(e.kernel.last_error, 0x1234);
        assert_eq!(
            get_string_type_w(&mut e, 1, Ptr::new(text), 2, Ptr::new(out)),
            1
        );
        // The table's ends and a run inside it.
        assert_eq!(ctype1(0x0009), 0x268);
        assert_eq!(ctype1(0x000c), 0x228);
        assert_eq!(ctype1(0xfffd), 0x200);
        assert_eq!(ctype1(0xffff), 0);
    }

    #[test]
    fn edge_cases_fail_as_measured() {
        let mut e = Engine::new();
        let text = wide(&mut e, "A");
        let out = e.mem.alloc(4);
        e.mem.set_u16(out, 0xaaaa);
        for (info, at, count, error) in [
            (1u32, text, 0i32, 87u32),
            (1, 0, 1, 87),
            (0, text, 1, 1004),
            (8, text, 1, 1004),
        ] {
            e.kernel.last_error = 0;
            assert_eq!(
                get_string_type_w(&mut e, info, Ptr::new(at), count, Ptr::new(out)),
                0
            );
            assert_eq!(e.kernel.last_error, error);
            assert_eq!(e.mem.u16(out), 0xaaaa);
        }
    }
}
