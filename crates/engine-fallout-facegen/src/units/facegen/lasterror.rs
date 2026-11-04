//! `FaceGen/lasterror.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::facegen::binaryfile::{
    string_chars, with_string_arg, StringArg, INVALID_PARAMETER, STRING_LENGTH, STRING_TIDY,
};

/// The last error code (`011f6c88`, 0: none yet) and its message buffer
/// (`011f6c8c`, 0x100 bytes).
const LAST_ERROR: u32 = 0x011f_6c88;
const LAST_ERROR_MESSAGE: u32 = 0x011f_6c8c;
/// `fr2Assert` (`00aafb90`, `cdecl`: source file name, line) and this
/// unit's source file name (`010a2de0`).
const FR2_ASSERT: u32 = 0x00aa_fb90;
const LAST_ERROR_SOURCE: u32 = 0x010a_2de0;
/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00ac6430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `futSetLastError` (Xbox PDB; `cdecl`, the code and a string by value):
/// keeps the first error only: when no error is set yet, the code and the
/// string's characters (at most 0xFF, with `fr2Assert` for a longer one;
/// the library's check of a negative length can never fail) are copied to
/// the last-error globals, with a terminating zero. The string argument is
/// destroyed (`0044a730(1, 0)`) either way.
// Translation incomplete: 00ac6430 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f20638, state variable) and the security-cookie bookkeeping are not translated.
pub fn fut_set_last_error(e: &mut Engine, code: u32, message: StringArg) {
    with_string_arg(e, message, |e, string| {
        if e.global::<u32>(LAST_ERROR) == 0 {
            let length = e.mem.u32(string + STRING_LENGTH);
            e.set_global(LAST_ERROR, code);
            if length >= 0x100 {
                e.call(FR2_ASSERT, &args![LAST_ERROR_SOURCE, 0x41u32]);
            }
            let length = if length < 0x100 { length } else { 0xff };
            // `CMP [length],0; JNC`: an unsigned length is never below 0.
            #[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
            let negative = e.mem.u32(string + STRING_LENGTH) < 0;
            if negative {
                e.call(INVALID_PARAMETER, &[]);
            }
            let source = string_chars(e, string);
            e.call(MEMCPY, &args![LAST_ERROR_MESSAGE, source, length]);
            e.mem.set_u8(LAST_ERROR_MESSAGE + length, 0);
        }
        e.call(STRING_TIDY, &args![string, 1u32, 0u32]);
    });
}

/// `strlen` (`00ec6130`, `cdecl`), the string's assign from characters (`0044a7f0`,
/// `thiscall`: characters, count) and its append of characters (`00a0c430`, `thiscall`:
/// characters, count).
const STRLEN: u32 = 0x00ec_6130;
const STRING_ASSIGN_CHARS: u32 = 0x0044_a7f0;
const STRING_APPEND_CHARS: u32 = 0x00a0_c430;
/// The separator `" : "` (`0104df70`) put between an error's text and its message.
const SEPARATOR: u32 = 0x0104_df70;
/// `fr2Error` (`00aafbb0`, `cdecl`: source, line, message), the source `"FUT System"`
/// (`010a2f3c`) and the message `"Error"` (`0104df38`) `futPrintLastError` passes.
const FR2_ERROR: u32 = 0x00aa_fbb0;
const FUT_SYSTEM: u32 = 0x010a_2f3c;
const ERROR_TEXT: u32 = 0x0104_df38;

/// The text of an error code (the code's `switch`; its jump table's cases in order): `"No
/// errors"`, `"File corrupt"`, `"File of incorrect type"`, `"File not found"`, `"File not
/// readable"`, `"File not writeable"`, `"File write error"`, `"File seek error"`, `"OpenGL
/// initialization error"`, `"Cannot load incompatible model"`, `"Out of memory"`, `"Error"`,
/// `"Dimension of the image is not power of 2"`, and `"Undocumented error"` past 12.
fn error_text(code: u32) -> u32 {
    match code {
        0 => 0x010a_2f30,
        1 => 0x010a_2f20,
        2 => 0x010a_2f08,
        3 => 0x010a_2ef8,
        4 => 0x010a_2ee4,
        5 => 0x010a_2ed0,
        6 => 0x010a_2ebc,
        7 => 0x010a_2eac,
        8 => 0x010a_2e90,
        9 => 0x010a_2e70,
        10 => 0x010a_2e60,
        11 => ERROR_TEXT,
        12 => 0x010a_2e34,
        _ => 0x010a_2e20,
    }
}

// Translated from 00ac6510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the address of a string): the string becomes the last
/// error's message (`strlen`, `0044a7f0`); answers the last error code.
pub fn fn_00ac6510(e: &mut Engine, string: u32) -> u32 {
    let length = e.call(STRLEN, &args![LAST_ERROR_MESSAGE]).u32();
    e.call(
        STRING_ASSIGN_CHARS,
        &args![string, LAST_ERROR_MESSAGE, length],
    );
    e.global(LAST_ERROR)
}

// Translated from 00ac6540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `futGetLastErrorMessage` (engine map; `cdecl`, the address of a string): the string becomes
/// the text of the last error code (`0044a7f0`). Without an error the answer is true; otherwise
/// a non-empty last-error message is appended after `" : "` (`00a0c430`) and the answer is
/// false.
pub fn fut_get_last_error_message(e: &mut Engine, string: u32) -> bool {
    let code: u32 = e.global(LAST_ERROR);
    let text = error_text(code);
    let length = e.call(STRLEN, &args![text]).u32();
    e.call(STRING_ASSIGN_CHARS, &args![string, text, length]);
    if e.global::<u32>(LAST_ERROR) == 0 {
        return true;
    }
    if e.call(STRLEN, &args![LAST_ERROR_MESSAGE]).u32() != 0 {
        let length = e.call(STRLEN, &args![SEPARATOR]).u32();
        e.call(STRING_APPEND_CHARS, &args![string, SEPARATOR, length]);
        let length = e.call(STRLEN, &args![LAST_ERROR_MESSAGE]).u32();
        e.call(
            STRING_APPEND_CHARS,
            &args![string, LAST_ERROR_MESSAGE, length],
        );
    }
    false
}

// Translated from 00ac6830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): clears the last error (an empty message, code 0).
pub fn fn_00ac6830(e: &mut Engine) {
    e.mem.set_u8(LAST_ERROR_MESSAGE, 0);
    e.set_global(LAST_ERROR, 0u32);
}

// Translated from 00ac6850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac6850 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// `futPrintLastError` (engine map; no arguments): a local string gets the last error's message
/// (`futGetLastErrorMessage`), then `fr2Error("FUT System", 0, "Error")` reports it (the message
/// itself is not passed) and the string is destroyed.
pub fn fut_print_last_error(e: &mut Engine) {
    e.with_stack(0x1c, |e, string| {
        let string = string.addr();
        e.call(STRING_TIDY, &args![string, 0u32, 0u32]);
        fut_get_last_error_message(e, string);
        e.call(FR2_ERROR, &args![FUT_SYSTEM, 0u32, ERROR_TEXT]);
        e.call(STRING_TIDY, &args![string, 1u32, 0u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ac_6430, fut_set_last_error(u32, StringArg)),
        entry!(0x00ac_6510, fn_00ac6510(u32) -> u32),
        entry!(0x00ac_6540, fut_get_last_error_message(u32) -> bool),
        entry!(0x00ac_6830, fn_00ac6830()),
        entry!(0x00ac_6850, fut_print_last_error()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRING_TIDY, |_, _| Ret::default());
        e.register(FR2_ASSERT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A short string (inline characters) by value.
    fn short(text: &[u8]) -> StringArg {
        let mut words = [0u32; 7];
        let mut bytes = [0u8; 16];
        bytes[..text.len()].copy_from_slice(text);
        for i in 0..4 {
            words[1 + i] = u32::from_le_bytes([
                bytes[4 * i],
                bytes[4 * i + 1],
                bytes[4 * i + 2],
                bytes[4 * i + 3],
            ]);
        }
        words[5] = text.len() as u32;
        words[6] = 0xf;
        StringArg(words)
    }

    #[test]
    fn keeps_the_first_error_and_destroys_the_argument() {
        let mut e = rig();
        let mut words = vec![7u32];
        words.extend_from_slice(&short(b"bad file").0);
        e.call(0x00ac_6430, &words);
        assert_eq!(e.global::<u32>(LAST_ERROR), 7);
        assert_eq!(e.mem.cstr(LAST_ERROR_MESSAGE), b"bad file".to_vec());
        let mut words = vec![9u32];
        words.extend_from_slice(&short(b"other").0);
        e.call(0x00ac_6430, &words);
        assert_eq!(e.global::<u32>(LAST_ERROR), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == STRING_TIDY).count(), 2);
        assert_eq!(log.iter().filter(|c| c.0 == MEMCPY).count(), 1);
    }

    #[test]
    fn long_messages_are_cut_to_0xff_bytes_with_an_assert() {
        let mut e = rig();
        let text = e.mem.alloc(0x200);
        e.mem.write(text, &[b'x'; 0x180]);
        let words = vec![3u32, 0, text, 0, 0, 0, 0x180, 0x1ff];
        e.call(0x00ac_6430, &words);
        assert_eq!(e.mem.cstr(LAST_ERROR_MESSAGE).len(), 0xff);
        let log = e.call_log.take().unwrap();
        let asserts: Vec<_> = log.iter().filter(|c| c.0 == FR2_ASSERT).collect();
        assert_eq!(asserts[0].1, vec![LAST_ERROR_SOURCE, 0x41]);
    }

    fn message_rig() -> Engine {
        let mut e = rig();
        e.call_log = None;
        e.map(0x0104_d000, 0x1000);
        e.map(0x010a_2000, 0x1000);
        e.mem.write(SEPARATOR, b" : \0");
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        for addr in [STRING_ASSIGN_CHARS, STRING_APPEND_CHARS, FR2_ERROR] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn error_message_and_code_read_back() {
        let mut e = message_rig();
        e.set_global(LAST_ERROR, 6u32);
        e.mem.write(LAST_ERROR_MESSAGE, b"a.tri\0");
        let string = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_6510, &args![string]).u32(), 6);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STRING_ASSIGN_CHARS),
            vec![vec![string, LAST_ERROR_MESSAGE, 5]]
        );
        e.call(0x00ac_6830, &[]);
        assert_eq!(e.global::<u32>(LAST_ERROR), 0);
        assert_eq!(e.mem.u8(LAST_ERROR_MESSAGE), 0);
    }

    #[test]
    fn last_error_message_text_by_code() {
        let mut e = message_rig();
        let string = e.mem.alloc(0x1c);
        // No error: "No errors", true.
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ac_6540, &args![string]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_ASSIGN_CHARS)[0][1], 0x010a_2f30);
        assert!(calls_to(&log, STRING_APPEND_CHARS).is_empty());
        // An error with a message: its text, then " : " and the message.
        e.set_global(LAST_ERROR, 11u32);
        e.mem.write(LAST_ERROR_MESSAGE, b"x.egm\0");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ac_6540, &args![string]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_ASSIGN_CHARS)[0][1], ERROR_TEXT);
        assert_eq!(
            calls_to(&log, STRING_APPEND_CHARS),
            vec![
                vec![string, SEPARATOR, 3],
                vec![string, LAST_ERROR_MESSAGE, 5]
            ]
        );
        // Past 12 the text is "Undocumented error"; an empty message adds nothing.
        e.set_global(LAST_ERROR, 40u32);
        e.mem.set_u8(LAST_ERROR_MESSAGE, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ac_6540, &args![string]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_ASSIGN_CHARS)[0][1], 0x010a_2e20);
        assert!(calls_to(&log, STRING_APPEND_CHARS).is_empty());
        assert_eq!(error_text(2), 0x010a_2f08);
        assert_eq!(error_text(12), 0x010a_2e34);
    }

    #[test]
    fn print_last_error_reports_through_fr2_error() {
        let mut e = message_rig();
        e.call_log = Some(vec![]);
        e.call(0x00ac_6850, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FR2_ERROR),
            vec![vec![FUT_SYSTEM, 0, ERROR_TEXT]]
        );
        let tidies = calls_to(&log, STRING_TIDY);
        assert_eq!(tidies.len(), 2);
        assert_eq!(tidies[0][1..], [0, 0]);
        assert_eq!(tidies[1][1..], [1, 0]);
        assert_eq!(calls_to(&log, STRING_ASSIGN_CHARS)[0][0], tidies[0][0]);
    }
}
