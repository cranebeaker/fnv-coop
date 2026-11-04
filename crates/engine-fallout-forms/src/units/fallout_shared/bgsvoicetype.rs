//! `fallout shared/bgsvoicetype.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005effc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005effc0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5d (0x24 bytes); `ECX` is the object.
/// The engine map names it `BGSVoiceType::BGSVoiceType` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0104_733c`) is `BGSVoiceType`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005effc0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0104_733c);
    e.call(0x0040_37b0, &args![base + 0x1c]);
    e.call(0x004f_15a0, &args![this, 0x5du32]);
    e.mem.set_u8(base + 0x18, 0x0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_ffc0, fn_005effc0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005effc0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_ffc0,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_37b0, &[A::T(0x1c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5d)]),
            ],
            stores: &[(0x0, 4, 0x0104_733c), (0x18, 1, 0x0)],
        });
    }
}
