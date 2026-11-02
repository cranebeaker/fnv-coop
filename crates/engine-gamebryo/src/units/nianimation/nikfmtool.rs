//! `NiAnimation/nikfmtool.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_strcpy_s` (`00ec65a6`, cdecl: buffer, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `NiPath::Standardize` (`00aa2a00`, cdecl, one word).
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
/// `00a5b690` (cdecl, one word: a C string): the fixed-string handle for it.
const GET_FIXED_STRING: u32 = 0x00a5_b690;
/// The handle of the empty `NiFixedString` (`0109b220`); a handle's count is 8 bytes before
/// it.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The `InterlockedDecrement` import slot.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a57500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a57500 - the /GS stack cookie (stored at frame +0x104 and checked through 00ec408c) is not translated
/// `NiStandardizeFilePath` (map name; cdecl, one word: an `NiFixedString`): copies the
/// string into a 0x104-byte stack buffer (`_strcpy_s`), standardizes it (`NiPath::Standardize`)
/// and, unless the handle is the buffer's own address, makes the buffer the fixed string (a
/// new handle from `00a5b690`, the old one's reference given up unless it is the empty one).
pub fn ni_standardize_file_path(e: &mut Engine, string: Ptr) {
    let slot = string.addr();
    e.with_stack(0x108, |e, buffer| {
        let buffer = buffer.addr();
        let text = e.mem.u32(slot);
        e.call(STRCPY_S, &args![buffer, 0x104u32, text]);
        e.call(PATH_STANDARDIZE, &args![buffer]);
        let old = e.mem.u32(slot);
        if old == buffer {
            return;
        }
        let handle = e.call(GET_FIXED_STRING, &args![buffer]).u32();
        e.mem.set_u32(slot, handle);
        if old != e.mem.u32(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a57500, ni_standardize_file_path(Ptr))]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn ni_standardize_file_path_replaces_the_handle() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING, 4);
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register(GET_FIXED_STRING, |_, _| Ret {
            eax: 0x7708,
            ..Ret::default()
        });
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 2);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, block + 8);
        e.call_log = Some(vec![]);
        e.call(0x00a57500, &args![slot]);
        let buffer = calls_to(&e, STRCPY_S)[0][0];
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![buffer, 0x104, block + 8]]);
        assert_eq!(calls_to(&e, PATH_STANDARDIZE), vec![vec![buffer]]);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(slot), 0x7708);
        assert_eq!(e.mem.u32(block), 1);
        // The empty handle (0 here) is not released.
        e.mem.set_u32(slot, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a57500, &args![slot]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }
}
