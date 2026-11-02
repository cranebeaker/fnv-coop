//! `fallout/interface/run_00705fc0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The callee the member at +0x14 is handed to (`this` + one word, `RET 4`).
const MEMBER_CALLEE: u32 = 0x0066_b0d0;

// Translated from 00705fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: calls `0066b0d0` on the member at `this + 0x14` with the one
/// stack word and returns its `EAX`. The classes involved are not
/// confirmed.
pub fn fn_00705fc0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    e.call(MEMBER_CALLEE, &args![this.byte_add(0x14), value])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_5fc0, fn_00705fc0(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_the_member_and_the_word() {
        let mut e = Engine::new();
        e.register(MEMBER_CALLEE, |_, a| Ret {
            eax: a[0] ^ a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0070_5fc0, &args![0x1000u32, 0x77u32]).u32();
        assert_eq!(result, 0x1014 ^ 0x77);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == MEMBER_CALLEE).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x1014, 0x77]);
    }
}
