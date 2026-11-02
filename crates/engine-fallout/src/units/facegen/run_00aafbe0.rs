//! `FaceGen/run_00aafbe0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{BSFaceGenBinaryFile, StringArg};

/// The string from a C string (`0044a4c0`, `thiscall`: the characters), the EGM file id
/// `"FREGM002"` (`010a2d24`), `FutBinaryFileC`'s constructor (`00ac3df0`, `thiscall`: the id
/// string by value), its check of a named file (`00ac3fe0`, `thiscall`: the address of a name
/// string; opens the file with `fopen` and compares its id; answers a byte) and its destructor
/// (`00aaaa80`, `thiscall`, unit `fanctlfile`).
const STRING_FROM_C: u32 = 0x0044_a4c0;
const EGM_FILE_ID: u32 = 0x010a_2d24;
const FUT_BINARY_FILE_CONSTRUCT: u32 = 0x00ac_3df0;
const FUT_BINARY_FILE_CHECK: u32 = 0x00ac_3fe0;
const FUT_BINARY_FILE_DESTRUCT: u32 = 0x00aa_aa80;

// Translated from 00aafbe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aafbe0 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map (`cdecl`, one word: the address of a file name string): whether the
/// file is an EGM file. A `FutBinaryFileC` is made on the stack with the id `"FREGM002"`
/// (`0044a4c0` into the by-value argument, `00ac3df0`), checks the name (`00ac3fe0`) and is
/// destroyed (`00aaaa80`); the answer is the check's byte.
pub fn fn_00aafbe0(e: &mut Engine, name: u32) -> bool {
    e.with_stack(0x40, |e, file| {
        let file: Ptr<BSFaceGenBinaryFile> = Ptr::new(file.addr());
        let id = e.with_stack(0x1c, |e, id| {
            e.call(STRING_FROM_C, &args![id, EGM_FILE_ID]);
            StringArg::load(e, id.addr())
        });
        e.call(FUT_BINARY_FILE_CONSTRUCT, &args![file, id]);
        let found = e.call(FUT_BINARY_FILE_CHECK, &args![file, name]).u8();
        e.call(FUT_BINARY_FILE_DESTRUCT, &args![file]);
        found != 0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_fbe0, fn_00aafbe0(u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egm_check_builds_checks_and_destroys_a_file() {
        let mut e = Engine::new();
        for addr in [
            STRING_FROM_C,
            FUT_BINARY_FILE_CONSTRUCT,
            FUT_BINARY_FILE_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        for answer in [0u32, 0x101] {
            e.register_double(FUT_BINARY_FILE_CHECK, move |_, _| answer.into_ret());
            e.call_log = Some(vec![]);
            let found = e.call(0x00aa_fbe0, &args![0x1234u32]).bool();
            let log = e.call_log.take().unwrap();
            assert_eq!(found, answer != 0);
            let addrs: Vec<u32> = log[1..].iter().map(|(a, _)| *a).collect();
            assert_eq!(
                addrs,
                vec![
                    STRING_FROM_C,
                    FUT_BINARY_FILE_CONSTRUCT,
                    FUT_BINARY_FILE_CHECK,
                    FUT_BINARY_FILE_DESTRUCT
                ]
            );
            let file = log[2].1[0];
            assert_eq!(log[1].1[1], EGM_FILE_ID);
            assert_eq!(log[2].1.len(), 8);
            assert_eq!(log[3].1, vec![file, 0x1234]);
            assert_eq!(log[4].1, vec![file]);
        }
    }
}
