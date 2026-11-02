//! `(unplaced)/run_00a5f4d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiZBufferProperty`'s vtable (its RTTI names the class).
const Z_BUFFER_PROPERTY_VTABLE: u32 = 0x0109_b7fc;
/// The base constructor `00a5c200` (thiscall; the engine map puts it in
/// `NiMain/niobjectnet.cpp`).
const BASE_CONSTRUCTOR: u32 = 0x00a5_c200;

// Translated from 00a5f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiZBufferProperty` (the class its vtable's RTTI names;
/// called by `00e750e0`): the base constructor `00a5c200`, then the 16-bit
/// flags at +0x18 set to 0xF and the vtable `0109b7fc`. Returns `this`.
pub fn fn_00a5f4d0(e: &mut Engine, this: u32) -> u32 {
    e.call(BASE_CONSTRUCTOR, &[this]);
    e.mem.set_u16(this + 0x18, 0xf);
    e.mem.set_u32(this, Z_BUFFER_PROPERTY_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_f4d0, fn_00a5f4d0(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_buffer_property_gets_its_flags_and_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_f4d0, &[this]).u32(), this);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (BASE_CONSTRUCTOR, vec![this])
        );
        assert_eq!(e.mem.u16(this + 0x18), 0xf);
        assert_eq!(e.mem.u32(this), Z_BUFFER_PROPERTY_VTABLE);
    }
}
