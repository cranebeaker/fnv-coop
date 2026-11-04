//! `(unplaced)/run_00a2f8c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a6d220` (unit `nitimecontroller.cpp`, `thiscall`): `NiTimeController`'s constructor.
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
/// `NiInterpController`'s vtable (by its RTTI type descriptor).
const NI_INTERP_CONTROLLER_VTABLE: u32 = 0x0109_632c;

// Translated from 00a2f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiInterpController`'s constructor): the
/// `NiTimeController` constructor (`00a6d220`), flag bit 5 of the controller's flags (+8)
/// cleared, the vtable `0109632c`. Answers `this`.
pub fn fn_00a2f8c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![this]);
    // NiTimeController::m_uFlags (Xbox PDB) +0x08
    let flags = e.mem.u16(this.addr() + 8) & 0xffdf;
    e.mem.set_u16(this.addr() + 8, flags);
    e.mem.set_u32(this.addr(), NI_INTERP_CONTROLLER_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a2_f8c0, fn_00a2f8c0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a2f8c0_constructs_and_clears_flag_bit_5() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u16(a[0] + 8, 0x006c);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00a2_f8c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 8), 0x004c);
        assert_eq!(e.mem.u32(this), NI_INTERP_CONTROLLER_VTABLE);
    }
}
