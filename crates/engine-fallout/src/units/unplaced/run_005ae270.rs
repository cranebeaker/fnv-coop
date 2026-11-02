//! `(unplaced)/run_005ae270` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ae270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears four words of the thread's TLS block (`*(FS:[0x2c] +
/// _tls_index * 4)`, `_tls_index` at `0126fd98`; `Engine::tls`): `+0x288` and `+0x284` to 0,
/// `+0x280` to -1 and `+0x27c` to 0, in that order.
pub fn fn_005ae270(e: &mut Engine) {
    let block = e.tls();
    e.mem.set_u32(block + 0x288, 0);
    e.mem.set_u32(block + 0x284, 0);
    e.mem.set_u32(block + 0x280, u32::MAX);
    e.mem.set_u32(block + 0x27c, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005a_e270, fn_005ae270())]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    #[test]
    fn fn_005ae270_resets_the_thread_words() {
        let mut e = Engine::new();
        let block = e.tls();
        for offset in (0x27c..0x28c).step_by(4) {
            e.mem.set_u32(block + offset, 0x1234);
        }
        e.call(0x005a_e270, &[]);
        assert_eq!(e.mem.u32(block + 0x288), 0);
        assert_eq!(e.mem.u32(block + 0x284), 0);
        assert_eq!(e.mem.u32(block + 0x280), u32::MAX);
        assert_eq!(e.mem.u32(block + 0x27c), 0);
    }
}
