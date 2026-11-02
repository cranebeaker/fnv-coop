//! `fallout shared/run_0061c330` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0061c330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): runs `00470470` on the three
/// members at +0x8, +0 and +0x10, in that order.
pub fn fn_0061c330(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0047_0470, &args![base + 0x8]);
    e.call(0x0047_0470, &args![base]);
    e.call(0x0047_0470, &args![base + 0x10]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_c330, fn_0061c330(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0061c330_releases_the_three_members_in_order() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x18);
        e.register(0x0047_0470, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_c330, &args![object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0047_0470, vec![object + 0x8]),
                (0x0047_0470, vec![object]),
                (0x0047_0470, vec![object + 0x10]),
            ]
        );
    }
}
