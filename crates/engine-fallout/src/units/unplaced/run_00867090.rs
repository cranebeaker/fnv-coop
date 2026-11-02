//! `(unplaced)/run_00867090` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0126fac4`: the object `00867090` disposes of.
const HELD_OBJECT: u32 = 0x0126_fac4;

// Translated from 00867090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when `0126fac4` holds an object, calls its vtable
/// slot 0 with 1 (`thiscall`); then clears the pointer.
pub fn fn_00867090(e: &mut Engine) {
    let object = e.mem.u32(HELD_OBJECT);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(HELD_OBJECT, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0086_7090, fn_00867090())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroys_the_held_object_and_clears_the_pointer() {
        let mut e = Engine::new();
        e.map(HELD_OBJECT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0150_0000);
        e.mem.set_u32(HELD_OBJECT, object);
        e.call_log = Some(vec![]);
        e.call(0x0086_7090, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0150_1000, vec![object, 1])]);
        assert_eq!(e.mem.u32(HELD_OBJECT), 0);
        e.call_log = Some(vec![]);
        e.call(0x0086_7090, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
