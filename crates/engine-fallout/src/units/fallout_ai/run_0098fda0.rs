//! `fallout/ai/run_0098fda0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0098fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An entry's constructor: `0097ede0` on `this` and `004300a0` on +0x10.
/// Returns `this`.
pub fn fn_0098fda0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0097_ede0, &args![this]);
    e.call(0x0043_00a0, &args![this.addr() + 0x10]);
    this
}

// Translated from 0098fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word of the table at `011a4ed0` (the exe's data) for the index
/// `00726070` gives.
pub fn fn_0098fdd0(e: &mut Engine, this: Ptr) -> u32 {
    let index = e.call(0x0072_6070, &args![this]).u32();
    e.mem.u32(index.wrapping_mul(4).wrapping_add(0x011a_4ed0))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_fda0, fn_0098fda0(Ptr) -> Ptr),
        entry!(0x0098_fdd0, fn_0098fdd0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_0098fda0_constructs_both_members() {
        let mut e = Engine::new();
        e.register(0x0097_ede0, |_, _| Ret::default());
        e.register(0x0043_00a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0098_fda0, &[0x9000]).u32(), 0x9000);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log[1], (0x0097_ede0, vec![0x9000]));
        assert_eq!(log[2], (0x0043_00a0, vec![0x9010]));
    }

    #[test]
    fn fn_0098fdd0_reads_the_table_entry() {
        let mut e = Engine::new();
        e.map(0x011a_4ed0, 0x10);
        e.mem.set_u32(0x011a_4ed0 + 8, 0x1234);
        e.register(0x0072_6070, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0098_fdd0, &[0x9000]).u32(), 0x1234);
    }
}
