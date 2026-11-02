//! `fallout/audio/run_0082f760` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `memcpy` wrapper `00401460` (cdecl: destination, source, size).
const MEMCPY: u32 = 0x0040_1460;
/// `SetEvent` (KERNEL32 import slot).
const SET_EVENT: u32 = 0x00fd_f0d0;

// Translated from 0082f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 8 bytes at `source` to `this + 0x34` and signals the event
/// whose handle is at `this + 0x30` (`SetEvent`). The class is not
/// confirmed (its caller is `008300c0`).
pub fn fn_0082f760(e: &mut Engine, this: Ptr, source: Ptr) {
    e.call(MEMCPY, &args![this.addr().wrapping_add(0x34), source, 8u32]);
    let event = e.mem.u32(this.addr().wrapping_add(0x30));
    e.call(SET_EVENT, &args![event]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_f760, fn_0082f760(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_eight_bytes_then_signals() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(SET_EVENT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0082_f760, &args![this, 0x0050_0000u32]);
        assert_eq!(
            *e.call_log.as_ref().unwrap(),
            vec![
                (0x0082_f760, vec![this, 0x0050_0000]),
                (MEMCPY, vec![this + 0x34, 0x0050_0000, 8]),
                (SET_EVENT, vec![0x1234]),
            ]
        );
    }
}
