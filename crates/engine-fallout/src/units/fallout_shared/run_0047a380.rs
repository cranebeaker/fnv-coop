//! `fallout shared/run_0047a380` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the armor and book loaders give it a message icon's
/// `MICO` text): hands `text` to `00489100` on `this + 4` (`RET 4`).
pub fn fn_0047a380(e: &mut Engine, this: Ptr, text: u32) {
    e.call(0x0048_9100, &args![this.addr() + 4, text]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047_a380, fn_0047a380(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_the_text_to_the_icon_at_4() {
        let mut e = Engine::new();
        e.register(0x0048_9100, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_a380, &[0x1000, 0x2222]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0048_9100).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x1004, 0x2222]);
    }
}
