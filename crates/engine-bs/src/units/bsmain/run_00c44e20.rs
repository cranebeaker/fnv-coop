//! `BSMain/run_00c44e20` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5e930` (`thiscall`, no stack word; unit `NiMain/ninode.cpp`): the base destructor.
const NODE_DESTRUCTOR: u32 = 0x00a5_e930;

// Translated from 00c44e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `010178bc` and runs the base destructor `00a5e930` (unit `ninode.cpp`).
pub fn fn_00c44e20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_78bc);
    e.call(NODE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c4_4e20, fn_00c44e20(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(NODE_DESTRUCTOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x0101_78bc);
            Ret::default()
        });
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4e20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(NODE_DESTRUCTOR, vec![this])]);
    }
}
