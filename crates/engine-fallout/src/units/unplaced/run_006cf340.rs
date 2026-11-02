//! `(unplaced)/run_006cf340` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006cf340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf340 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// An array destructor body (no name in the engine map): stores the vtable
/// `0106c968`, releases the buffer (`008454f0` with 1), then runs the base
/// destructor `005e0540`.
pub fn fn_006cf340(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_c968);
    e.call(0x0084_54f0, &args![this, 1u32]);
    e.call(0x005e_0540, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_f340, fn_006cf340(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006cf340_releases_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(0x0084_54f0, |_, _| Ret::default());
        e.register(0x005e_0540, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006c_f340, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0106_c968);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0084_54f0 || *a == 0x005e_0540)
            .collect();
        assert_eq!(
            log,
            vec![(0x0084_54f0, vec![this, 1]), (0x005e_0540, vec![this])]
        );
    }
}
