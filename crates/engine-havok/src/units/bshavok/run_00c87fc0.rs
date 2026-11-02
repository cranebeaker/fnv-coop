//! `BSHavok/run_00c87fc0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::HEAP_ALLOCATE;

/// Size of the object the function creates.
const OBJECT_SIZE: u32 = 0x14;
/// The constructor of the object's base (`00c6b910`, `this` only).
const BASE_CONSTRUCT: u32 = 0x00c6_b910;
/// The class vtable the function stores after the base constructor has run.
const CLASS_VTABLE: u32 = 0x0106_6b94;

// Translated from 00c87fc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87fc0 - the FS:[0] exception frame (unwinding that frees the block when the constructor throws) is not modeled.
/// `fn_00c87fc0` (unnamed in the Xbox PDB): takes no arguments, allocates 0x14 bytes
/// (`00aa13e0`) and, when that gives a block, runs the base constructor (`00c6b910`) on it and
/// stores the class vtable (0x01066b94). Answers the block, or 0 when the allocation failed.
/// The class is not named by the exe data read so far.
pub fn fn_00c87fc0(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, CLASS_VTABLE);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c87fc0, fn_00c87fc0() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn builds_the_object_after_a_successful_allocation() {
        let mut e = logging_engine();
        let block = e.mem.alloc(0x14);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(BASE_CONSTRUCT, nothing);
        assert_eq!(e.call(0x00c8_7fc0, &args![]).u32(), block);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), [[0x14]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), CLASS_VTABLE);
    }

    #[test]
    fn answers_zero_without_constructing_when_the_allocation_fails() {
        let mut e = logging_engine();
        e.register(HEAP_ALLOCATE, nothing);
        e.register(BASE_CONSTRUCT, nothing);
        assert_eq!(e.call(0x00c8_7fc0, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
