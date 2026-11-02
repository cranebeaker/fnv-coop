//! `BSSystemUtilities/run_00af3020` (the engine map's run of unnamed functions from
//! `00af3020`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). `00af3020` itself is in `engine-core`'s `platform.rs`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSSystemUtility`'s vtable (`010a42b4`).
const SYSTEM_UTILITY_VTABLE: u32 = 0x010a_42b4;

// Translated from 00af3120 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtility`'s destructor body (no label; `thiscall`, plain `RET`; the scalar deleting
/// destructor `00af33c0` calls it): stores the vtable `010a42b4` again and does nothing else.
pub fn fn_00af3120(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SYSTEM_UTILITY_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_3120, fn_00af3120(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x00af_3120, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010a_42b4);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}
