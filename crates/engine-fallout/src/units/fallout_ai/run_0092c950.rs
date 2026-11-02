//! `fallout/ai/run_0092c950` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0092c950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092c950 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructor derived from the one at `00906dc0`; `ECX` is the object and the answer is the
/// object. It runs the base constructor, stores the vtable pointer `0x01089bcc`, runs `00937000`
/// on `+0xb8` and `0061cc40` with 2. The engine map gives it no name. The exception-unwinding
/// frame is not translated.
pub fn fn_0092c950(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0090_6dc0, &args![this]);
    e.mem.set_u32(base, 0x0108_9bcc);
    e.call(0x0093_7000, &args![base + 0xb8]);
    e.call(0x0061_cc40, &args![this, 2u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0092_c950, fn_0092c950(Ptr) -> Ptr)]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0092c950_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0092_c950,
            size: 0xd0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0090_6dc0, &[A::T(0)]),
                (0x0093_7000, &[A::T(0xb8)]),
                (0x0061_cc40, &[A::T(0), A::V(2)]),
            ],
            stores: &[(0, 4, 0x0108_9bcc)],
        });
    }
}
