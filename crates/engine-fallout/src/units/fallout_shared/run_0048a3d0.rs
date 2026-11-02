//! `fallout shared/run_0048a3d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048a3d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The constructor of the form component `TESModelTextureSwap` (the engine
/// map gives it no name; the RTTI name of the vtable it stores, `0101d124`,
/// is `.?AVTESModelTextureSwap@@`): runs the `TESModel` constructor
/// (`00488f50`), stores the vtable at `+0`, runs `0096a2d0` on the list at
/// `+0x18`, then `TESModelTextureSwap::InitializeDataComponent` (Xbox PDB
/// name of `0048a470`). Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_0048a3d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0048_8f50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_d124);
    e.call(0x0096_a2d0, &args![this.byte_add(0x18)]);
    e.call(0x0048_a470, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048a3d0, fn_0048a3d0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0048a3d0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_a3d0,
            size: 0x28,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_8f50, &[A::T(0x0)]),
                (0x0096_a2d0, &[A::T(0x18)]),
                (0x0048_a470, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_d124)],
        });
    }
}
