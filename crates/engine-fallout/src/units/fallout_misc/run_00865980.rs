//! `fallout/misc/run_00865980` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0044dee0` (`thiscall`): clears the word at `this` (the sub-buffer's
/// constructor).
const SUB_BUFFER_CONSTRUCT: u32 = 0x0044_dee0;

// Translated from 00865980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the map item's constructor
/// (`008659a0`); answers `this`.
pub fn fn_00865980(e: &mut Engine, this: Ptr) -> Ptr {
    fn_008659a0(e, this);
    this
}

// Translated from 008659a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): constructs the sub-buffer value
/// at +8 (`0044dee0`); answers `this`.
pub fn fn_008659a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SUB_BUFFER_CONSTRUCT, &args![this.addr().wrapping_add(8)]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_5980, fn_00865980(Ptr) -> Ptr),
        entry!(0x0086_59a0, fn_008659a0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn the_item_constructor_builds_the_value_at_8() {
        let mut e = Engine::new();
        e.register(SUB_BUFFER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x1234);
        e.mem.set_u32(this, 0x55);
        assert_eq!(e.call(0x0086_5980, &args![this]).u32(), this);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 8)), (0x55, 0));
        e.mem.set_u32(this + 8, 0x1234);
        assert_eq!(e.call(0x0086_59a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0);
    }
}
