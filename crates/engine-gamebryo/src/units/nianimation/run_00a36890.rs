//! `NiAnimation/run_00a36890` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiInterpolator::NiInterpolator` (`thiscall`, no stack words).
const INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_6ff0;
/// The float (`-FLT_MAX`, `ff7fffff`, in the exe) four of the times start as.
const UNSET_TIME_FLOAT: u32 = 0x0109_696c;
/// `NiBlendInterpolator`'s vtable (by its RTTI type descriptor).
const NI_BLEND_INTERPOLATOR_VTABLE: u32 = 0x0109_697c;

// Translated from 00a36890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiBlendInterpolator`'s default constructor; the
/// fields are described by offset): the `NiInterpolator` constructor, the float at +0x1c 0,
/// the floats at +0x20, +0x24, +0x28 and +0x2c the float at `0109696c`, the bytes at +0xc,
/// +0xd and +0xe 0, the words at +0x14 and +0x18 0, the vtable `0109697c`, the byte at +0xf
/// `0xff` and the bytes at +0x10 and +0x11 `0x80`. Answers `this`.
pub fn fn_00a36890(e: &mut Engine, this: Ptr) -> Ptr {
    let a = this.addr();
    e.call(INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(a + 0x1c, x87_store32(0.0));
    for (offset, byte) in [(0x20u32, 0xcu32), (0x24, 0xd), (0x28, 0xe)] {
        let unset = x87_store32(x87_load32(e.mem.u32(UNSET_TIME_FLOAT)));
        e.mem.set_u32(a + offset, unset);
        e.mem.set_u8(a + byte, 0);
    }
    e.mem.set_u32(a + 0x14, 0);
    e.mem.set_u32(a + 0x18, 0);
    let unset = x87_store32(x87_load32(e.mem.u32(UNSET_TIME_FLOAT)));
    e.mem.set_u32(a, NI_BLEND_INTERPOLATOR_VTABLE);
    e.mem.set_u32(a + 0x2c, unset);
    e.mem.set_u8(a + 0xf, 0xff);
    e.mem.set_u8(a + 0x10, 0x80);
    e.mem.set_u8(a + 0x11, 0x80);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_6890, fn_00a36890(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a36890_sets_up_the_blend_interpolator() {
        let mut e = Engine::new();
        e.map(0x0109_6000, 0x1000);
        e.mem.set_u32(UNSET_TIME_FLOAT, 0xff7f_ffff);
        e.register(INTERPOLATOR_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.mem.write(this, &[0xee; 0x34]);
        assert_eq!(e.call(0x00a3_6890, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BLEND_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xeeee_eeee);
        assert_eq!(e.mem.u32(this + 8), 0xeeee_eeee);
        assert_eq!(e.mem.u32(this + 0xc), 0xff00_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0xeeee_8080);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        for offset in [0x20u32, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), 0xff7f_ffff);
        }
        assert_eq!(e.mem.u32(this + 0x30), 0xeeee_eeee);
    }
}
