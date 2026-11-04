//! `fallout shared/run_00449f80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0044d5c0` (`thiscall`, one word, `RET 4`; no engine-map name).
const MEMBER_CALL: u32 = 0x0044_d5c0;

// Translated from 00449f80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): calls `0044d5c0` on the object
/// at `this` +0x14 with `this` and answers what it answers (`EAX` is left as it returns).
pub fn fn_00449f80(e: &mut Engine, this: u32) -> u32 {
    let member = e.mem.u32(this + 0x14);
    e.call(MEMBER_CALL, &args![member, this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044_9f80, fn_00449f80(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00449f80_passes_this_to_its_member() {
        let mut e = Engine::new();
        e.register(MEMBER_CALL, |_, _| 0x66u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_9f80, &[this]).u32(), 0x66);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x0044_9f80, vec![this]), (MEMBER_CALL, vec![0x5000, this])]
        );
    }
}
