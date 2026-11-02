//! `(unplaced)/run_00ad5e20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memmove_s` (CRT, cdecl: destination, its size, source, count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;

// Translated from 00ad5e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the bytes `[first, last)` to `dest` (cdecl, four words, the last
/// never read; a `std::copy` shape for byte-sized items, called by
/// `00ab3370` and `00ad10e0`): `memmove_s(dest, n, first, n)` when `n =
/// last - first` is not 0; answers `dest + n`. The code's other stores are
/// copies of its locals and of uninitialized stack bytes that nothing reads.
pub fn fn_00ad5e20(e: &mut Engine, first: u32, last: u32, dest: u32, _unused_3: u32) -> u32 {
    let count = last.wrapping_sub(first);
    let end = dest.wrapping_add(count);
    if count != 0 {
        e.call(MEMMOVE_S, &[dest, count, first, count]);
    }
    end
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ad_5e20, fn_00ad5e20(u32, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_are_moved_and_the_end_answered() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ad_5e20, &[0x1000, 0x1010, 0x3000, 0]).u32(),
            0x3010
        );
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (MEMMOVE_S, vec![0x3000, 0x10, 0x1000, 0x10])
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ad_5e20, &[0x1000, 0x1000, 0x3000, 0]).u32(),
            0x3000
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
