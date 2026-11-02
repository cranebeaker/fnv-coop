//! `SpeedTree/windinfo.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CTreeFileAccess` (engine map): `ParseInt` (`00b3ed80`), `ParseVector3`
/// (`00b3ef80`, the result's address; returns it) and `ParseFloat`
/// (`00b3ed20`, ST0); thiscall on the file.
const PARSE_INT: u32 = 0x00b3_ed80;
const PARSE_VECTOR3: u32 = 0x00b3_ef80;
const PARSE_FLOAT: u32 = 0x00b3_ed20;
/// `00b06180` (`speedtreert.cpp`, thiscall on the file): the token 0x138e
/// hands the file to it.
const PARSE_00B06180: u32 = 0x00b0_6180;
/// The error path: `std::basic_string` from a C string (`0044a4c0`,
/// thiscall), the exception's constructor (`00b09480`, thiscall: the
/// string, 0) and `_CxxThrowException` (`00ec7894`: the object, its throw
/// information `01170384`).
const STRING_CONSTRUCT: u32 = 0x0044_a4c0;
const EXCEPTION_CONSTRUCT: u32 = 0x00b0_9480;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
const THROW_INFO: u32 = 0x0117_0384;
/// `"malformed general wind information"` (`010a5924`).
const MALFORMED_MESSAGE: u32 = 0x010a_5924;
/// The token that ends the section (0x3f4).
const END_TOKEN: u32 = 0x3f4;
/// `SIdvWindInfo` (as the code writes it): a vector at +0, a vector at
/// +0xc and a `float` at +0x18.
const VECTOR_0: u32 = 0x00;
const VECTOR_C: u32 = 0x0c;
const FLOAT_18: u32 = 0x18;

/// `ParseVector3` into the stack vector at `local`; its result's address.
fn parse_vector(e: &mut Engine, file: Ptr, local: u32) -> u32 {
    e.call(PARSE_VECTOR3, &args![file, local]).u32()
}

/// Copies the three words at `source` to `target`.
fn copy_vector(e: &mut Engine, source: u32, target: u32) {
    for i in 0..3 {
        let word = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(target + 4 * i, word);
    }
}

// Translated from 00b408b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b408b0 - the C++ exception the default case throws (and the function's FS:[0] frame) is not translated: after `_CxxThrowException` the translation returns
/// `SIdvWindInfo::Parse` (engine map; thiscall, `RET 4`): reads tokens
/// from `file` until the token 0x3f4 (each token handled before the next
/// is read, the first one too): 5000, 5001 and 5003 read a vector into a
/// stack temporary that is dropped; 5002 reads the vector at +0xc, 5004
/// the vector at +0, 5005 the `float` at +0x18, 5006 hands the file to
/// `00b06180`; any other token builds the "malformed general wind
/// information" exception and throws it.
pub fn s_idv_wind_info_parse(e: &mut Engine, this: Ptr, file: Ptr) {
    let base = this.addr();
    // Five stack vectors (12 bytes each), the string and the exception.
    e.with_stack(5 * 12 + 0x1c + 0x28, |e, frame| {
        let vector = |i: u32| frame.addr() + 12 * i;
        let string = frame.addr() + 5 * 12;
        let exception = string + 0x1c;
        let mut token = e.call(PARSE_INT, &args![file]).u32();
        loop {
            match token.wrapping_sub(5000) {
                0 => {
                    parse_vector(e, file, vector(0));
                }
                1 => {
                    parse_vector(e, file, vector(1));
                }
                2 => {
                    let parsed = parse_vector(e, file, vector(2));
                    copy_vector(e, parsed, base + VECTOR_C);
                }
                3 => {
                    parse_vector(e, file, vector(3));
                }
                4 => {
                    let parsed = parse_vector(e, file, vector(4));
                    copy_vector(e, parsed, base + VECTOR_0);
                }
                5 => {
                    let value = e.call(PARSE_FLOAT, &args![file]).f64();
                    e.mem.set_u32(base + FLOAT_18, x87_store32(value));
                }
                6 => {
                    e.call(PARSE_00B06180, &args![file]);
                }
                _ => {
                    e.call(STRING_CONSTRUCT, &args![string, MALFORMED_MESSAGE]);
                    e.call(EXCEPTION_CONSTRUCT, &args![exception, string, 0u32]);
                    e.call(CXX_THROW_EXCEPTION, &args![exception, THROW_INFO]);
                    return;
                }
            }
            token = e.call(PARSE_INT, &args![file]).u32();
            if token == END_TOKEN {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b4_08b0, s_idv_wind_info_parse(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The test file: a list of tokens at +0 (a length word, then the
    /// tokens), read in turn; vectors parse as (token, token + 1, token + 2)
    /// of the last token read.
    fn file(e: &mut Engine, tokens: &[u32]) -> u32 {
        let file = e.mem.alloc(8 + 4 * tokens.len() as u32);
        for (i, token) in tokens.iter().enumerate() {
            e.mem.set_u32(file + 8 + 4 * i as u32, *token);
        }
        e.register(PARSE_INT, |e, a| {
            let next = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], next + 1);
            let token = e.mem.u32(a[0] + 8 + 4 * next);
            e.mem.set_u32(a[0] + 4, token);
            token.into_ret()
        });
        e.register(PARSE_VECTOR3, |e, a| {
            let token = e.mem.u32(a[0] + 4);
            for i in 0..3 {
                e.mem.set_u32(a[1] + 4 * i, token + i);
            }
            a[1].into_ret()
        });
        e.register(PARSE_FLOAT, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        for address in [
            PARSE_00B06180,
            STRING_CONSTRUCT,
            EXCEPTION_CONSTRUCT,
            CXX_THROW_EXCEPTION,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        file
    }

    #[test]
    fn parses_each_token_until_the_end_token() {
        let mut e = Engine::new();
        let file = file(
            &mut e,
            &[5000, 5002, 5004, 5005, 5006, 5001, 5003, END_TOKEN],
        );
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00b4_08b0, &args![this, file]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![5004, 5005, 5006, 5002, 5003, 5004]);
        assert_eq!(e.mem.u32(this + FLOAT_18), 0.25f32.to_bits());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == PARSE_VECTOR3).count(), 5);
        assert_eq!(log.iter().filter(|(a, _)| *a == PARSE_00B06180).count(), 1);
        assert!(!log.iter().any(|(a, _)| *a == CXX_THROW_EXCEPTION));
    }

    #[test]
    fn an_unknown_token_throws_the_malformed_exception() {
        let mut e = Engine::new();
        // The end token first is unknown too.
        let file = file(&mut e, &[END_TOKEN]);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00b4_08b0, &args![this, file]);
        let log = e.call_log.take().unwrap();
        let string = log.iter().find(|(a, _)| *a == STRING_CONSTRUCT).unwrap();
        assert_eq!(string.1[1], MALFORMED_MESSAGE);
        let exception = log.iter().find(|(a, _)| *a == EXCEPTION_CONSTRUCT).unwrap();
        assert_eq!(&exception.1[1..], &[string.1[0], 0]);
        let thrown = log.iter().find(|(a, _)| *a == CXX_THROW_EXCEPTION).unwrap();
        assert_eq!(thrown.1, vec![exception.1[0], THROW_INFO]);
        assert_eq!(log.iter().filter(|(a, _)| *a == PARSE_INT).count(), 1);
    }
}
