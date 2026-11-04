//! `fallout shared/havok/run_00620180` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4`.
const NEXT_FIELD: u32 = 0x0072_6070;

// Translated from 00620180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the decompiler gives it the library name
/// `UMSFreeVirtualProcessorRoot::SchedulerProxy`, which its body is not). `thiscall`: the dword
/// at `+4` of the member at `this + 4` (`00726070`), minus 4, or null when that is null.
pub fn fn_00620180(e: &mut Engine, this: Ptr) -> u32 {
    let next = e
        .call(NEXT_FIELD, &args![this.addr().wrapping_add(4)])
        .u32();
    if next == 0 {
        0
    } else {
        next.wrapping_sub(4)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_0180, fn_00620180(Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00620180_answers_the_next_node_minus_4_or_null() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NEXT_FIELD, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node + 8, 0x7004);
        assert_eq!(e.call(0x0062_0180, &args![node]).u32(), 0x7000);
        let logged = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == NEXT_FIELD)
            .map(|c| c.1.clone())
            .collect::<Vec<_>>();
        assert_eq!(logged, vec![vec![node + 4]]);
        e.mem.set_u32(node + 8, 0);
        assert_eq!(e.call(0x0062_0180, &args![node]).u32(), 0);
    }
}
