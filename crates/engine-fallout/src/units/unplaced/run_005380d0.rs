//! `(unplaced)/run_005380d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005380d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` is stored but never read): 0 for a
/// null `object`, otherwise the low byte of what the object's vtable slot +0xa0 returns.
pub fn fn_005380d0(e: &mut Engine, _this: Ptr, object: u32) -> u8 {
    if object == 0 {
        0
    } else {
        e.vcall(object, 0xa0, &args![]).u8()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0053_80d0, fn_005380d0(Ptr, u32) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asks_the_object_or_answers_zero() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0; 0x29]);
        e.mem.set_u32(0x0150_00a0, 0x0150_1000);
        e.register(0x0150_1000, |_, _| 0x1234_5602u32.into_ret());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        assert_eq!(e.call(0x0053_80d0, &args![0u32, object]).u32(), 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0053_80d0, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
