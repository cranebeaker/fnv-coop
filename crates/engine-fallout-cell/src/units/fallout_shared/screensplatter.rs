//! `fallout shared/screensplatter.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004deb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00a7b300`, which stores the vtable `0109da1c` at
/// `+0`, decrements the counter in the header of the block at `+8` through the import slot
/// `00fdf1c4` when `+8` is set, and tail-jumps into `00a5d3d0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00a7b300` ends in `JMP 00a5d3d0` (a base-destructor tail call
/// whose own result nothing here uses); Ghidra types it void.
pub fn fn_004deb10(e: &mut Engine, this: Ptr) {
    e.call(0x00a7_b300, &args![this]);
}

/// `BSSplatterExtraData`'s RTTI record (`011c7844`), what `GetRTTI` answers.
const SPLATTER_EXTRA_DATA_RTTI: u32 = 0x011c_7844;
/// `operator delete` with a size (`00aa1460`, cdecl: block, size).
const DELETE_SIZED: u32 = 0x00aa_1460;

// Translated from 004deae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSplatterExtraData::scalar deleting destructor` (Xbox PDB; the flags;
/// `RET 4`): runs the destructor (`004deb10`) and, with flag 1, frees the
/// 0x1C-byte object (`00aa1460`). Returns `this`.
pub fn bs_splatter_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004deb10(e, this);
    if flags & 1 != 0 {
        e.call(DELETE_SIZED, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 004e0530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSplatterExtraData::GetRTTI` (Xbox PDB): its RTTI record `011c7844`.
pub fn bs_splatter_extra_data_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    SPLATTER_EXTRA_DATA_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_eb10, fn_004deb10(Ptr)),
        entry!(
            0x004d_eae0,
            bs_splatter_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004e_0530, bs_splatter_extra_data_get_rtti(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_004deb10_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00a7_b300, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004d_eb10, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00a7_b300), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn deleting_destructor_frees_with_flag_1() {
        let mut e = Engine::new();
        e.register(0x00a7_b300, |_, _| Ret::default());
        e.register(DELETE_SIZED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.call(0x004d_eae0, &args![0x5000u32, 3u32]).u32();
        assert_eq!(this, 0x5000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x00a7_b300, vec![0x5000]),
                (DELETE_SIZED, vec![0x5000, 0x1c])
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x004d_eae0, &args![0x5000u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != DELETE_SIZED));
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004e_0530, &args![0x5000u32]).u32(), 0x011c_7844);
    }
}
