//! `(unplaced)/run_00aaa730` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `std::basic_string<char>` constructor from a C string (engine map),
/// `0044a4c0` (thiscall: the literal).
const STRING_FROM_LITERAL: u32 = 0x0044_a4c0;
/// `"FRCTL001"`.
const CONTROL_FILE_TAG: u32 = 0x010a_2c10;
/// The local object's constructor `00ac3df0` (thiscall; the tag string by
/// value, seven words), its file check `00ac3fe0` (thiscall: the path;
/// answers in AL) and its destructor `00aaaa80` (thiscall), all placed in
/// `FaceGen` by the engine map.
const CONTROL_FILE_CONSTRUCTOR: u32 = 0x00ac_3df0;
const CONTROL_FILE_CHECK: u32 = 0x00ac_3fe0;
const CONTROL_FILE_DESTRUCTOR: u32 = 0x00aa_aa80;
/// The size of the `std::string` the code builds in its outgoing arguments.
const STRING_SIZE: u32 = 0x1c;

// Translated from 00aaa730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks a FaceGen control file (cdecl; no caller or vtable in the exe):
/// builds a local object (`00ac3df0`, given the `std::string` `"FRCTL001"`
/// by value, built in place by `0044a4c0`), asks it about `path`
/// (`00ac3fe0`), destroys it (`00aaaa80`) and answers that byte.
// Translation incomplete: 00aaa730 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_00aaa730(e: &mut Engine, path: u32) -> bool {
    e.with_stack(0x44, |e, object| {
        let object = object.addr();
        let tag: Vec<u32> = e.with_stack(STRING_SIZE, |e, text| {
            e.call(STRING_FROM_LITERAL, &[text.addr(), CONTROL_FILE_TAG]);
            (0..STRING_SIZE / 4)
                .map(|i| e.mem.u32(text.addr() + 4 * i))
                .collect()
        });
        let mut words = vec![object];
        words.extend(tag);
        e.call(CONTROL_FILE_CONSTRUCTOR, &words);
        let answer = e.call(CONTROL_FILE_CHECK, &[object, path]).bool();
        e.call(CONTROL_FILE_DESTRUCTOR, &[object]);
        answer
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_a730, fn_00aaa730(u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_file_check_answers_the_object() {
        for answer in [0x101u32, 0x100] {
            let mut e = Engine::new();
            e.register(STRING_FROM_LITERAL, |e, a| {
                e.mem.set_u32(a[0] + 4, a[1]);
                e.mem.set_u32(a[0] + 0x18, 0xf);
                a[0].into_ret()
            });
            e.register(CONTROL_FILE_CONSTRUCTOR, |_, a| a[0].into_ret());
            e.register_double(CONTROL_FILE_CHECK, move |_, _| answer.into_ret());
            e.register(CONTROL_FILE_DESTRUCTOR, |_, _| 0u32.into_ret());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_a730, &[0x7000]).bool(), answer & 1 != 0);
            let log = e.call_log.take().unwrap();
            let object = log[2].1[0];
            assert_eq!(log[2].0, CONTROL_FILE_CONSTRUCTOR);
            assert_eq!(log[2].1[1..], [0, CONTROL_FILE_TAG, 0, 0, 0, 0, 0xf]);
            assert_eq!(log[3], (CONTROL_FILE_CHECK, vec![object, 0x7000]));
            assert_eq!(log[4], (CONTROL_FILE_DESTRUCTOR, vec![object]));
        }
    }
}
