//! `BSMain/run_00c4f360` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c4f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object at `this + 0xC4` through its vtable slot +0xA0 with no
/// arguments (the engine map has no name). The exe leaves that call's
/// result in place as its own; its one caller (`00b5e89f`) ignores it, so
/// nothing is answered here.
pub fn fn_00c4f360(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr() + 0xc4);
    e.vcall(object, 0xa0, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c4_f360, fn_00c4f360(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_slot_a0_of_the_object_at_c4() {
        const TARGET: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let vtable = e.mem.alloc(0xa4);
        e.mem.set_u32(vtable + 0xa0, TARGET);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0xc8);
        e.mem.set_u32(this + 0xc4, object);
        e.call(0x00c4_f360, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == TARGET && call.1 == vec![object]));
    }
}
