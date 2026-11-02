//! `NiMain/run_00a6fae0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5a390` (unit `NiMain/niavobject.cpp`): reached with a `JMP` (same `this`).
const AV_OBJECT_TAIL: u32 = 0x00a5_a390;

// Translated from 00a6fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): sets the vtable `0109cb9c` and
/// continues in `00a5a390` with the same `this` (a `JMP`; its result is passed on).
pub fn fn_00a6fae0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0109_cb9c);
    e.call(AV_OBJECT_TAIL, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_fae0, fn_00a6fae0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_then_continues_in_the_base() {
        let mut e = Engine::new();
        e.register(AV_OBJECT_TAIL, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x0109_cb9c);
            0x55u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_fae0, &args![this]).u32(), 0x55);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(AV_OBJECT_TAIL, vec![this])]);
    }
}
