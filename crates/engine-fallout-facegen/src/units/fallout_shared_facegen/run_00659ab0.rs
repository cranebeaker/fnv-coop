//! `fallout shared/facegen/run_00659ab0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor of `00659670`'s object (`00659710`, `thiscall`) and `operator delete`
/// (`00401030`, `cdecl`: the block).
const DESTRUCT: u32 = 0x0065_9710;
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00659ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of `00659670`'s object
/// (`thiscall`, `RET 4`: the flags): its destructor (`00659710`), then `operator delete` when
/// `flags & 1`. Returns `this`.
pub fn fn_00659ab0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_9ab0, fn_00659ab0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleting_destructor_frees_only_with_flag_bit_zero() {
        let mut e = Engine::new();
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(fn_00659ab0(&mut e, this, flags), this);
        }
        let log = e.call_log.take().unwrap();
        let count = |addr: u32| log.iter().filter(|(a, _)| *a == addr).count();
        assert_eq!(count(DESTRUCT), 4);
        assert_eq!(count(OPERATOR_DELETE), 2);
        assert!(log
            .iter()
            .filter(|(a, _)| *a == OPERATOR_DELETE)
            .all(|(_, w)| w == &vec![this.addr()]));
    }
}
