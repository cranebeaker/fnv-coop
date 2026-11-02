//! `BSMenu/xuserinterface.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global object `XUIIsUp` asks (`011daac0`).
const XUI_GLOBAL_011DAAC0: u32 = 0x011d_aac0;
/// `004a4080` (`thiscall`, one stack word; answers a byte in `AL`).
const QUERY_004A4080: u32 = 0x004a_4080;

// Translated from 0070edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `XUserInterface::XUIIsUp` (Xbox PDB): true when the object at `011daac0` exists and
/// `004a4080(1)` on it answers 0 (the low byte); false otherwise.
pub fn x_user_interface_xui_is_up(e: &mut Engine) -> bool {
    let object = e.mem.u32(XUI_GLOBAL_011DAAC0);
    if object == 0 {
        return false;
    }
    !e.call(QUERY_004A4080, &args![object, 1u32]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_edf0, x_user_interface_xui_is_up() -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xui_is_up_needs_the_object_and_a_zero_answer() {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.register(QUERY_004A4080, |e, a| {
            assert_eq!(a[1], 1);
            // Only the low byte counts.
            (e.mem.u32(a[0]) | 0x100).into_ret()
        });
        assert!(!e.call(0x0070_edf0, &[]).bool());
        let object = e.mem.alloc(4);
        e.mem.set_u32(XUI_GLOBAL_011DAAC0, object);
        assert!(e.call(0x0070_edf0, &[]).bool());
        e.mem.set_u32(object, 1);
        assert!(!e.call(0x0070_edf0, &[]).bool());
    }
}
