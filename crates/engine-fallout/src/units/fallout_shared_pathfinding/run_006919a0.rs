//! `fallout shared/pathfinding/run_006919a0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base clean-up both destructors call (`00438af0`, `thiscall`) and
/// the block free (`00aa10f0`, `cdecl (block)`).
const BASE_CLEAN_UP: u32 = 0x0043_8af0;
const FREE: u32 = 0x00aa_10f0;

// Translated from 006919a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006919a0 - its C++ exception frame (FS:[0]) is not translated
/// A destructor body (the engine map has no name): the vtable `0106aba4`,
/// the base clean-up `00438af0`, then the base destructor `00691a00`.
pub fn fn_006919a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_aba4);
    e.call(BASE_CLEAN_UP, &args![this]);
    fn_00691a00(e, this);
}

// Translated from 00691a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor body (the engine map has no name): the vtable `0106abf8`,
/// the clean-up `00438af0`, then the block at +8 freed (`00aa10f0`).
pub fn fn_00691a00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_abf8);
    e.call(BASE_CLEAN_UP, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_19a0, fn_006919a0(Ptr)),
        entry!(0x0069_1a00, fn_00691a00(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_destructors_clean_up_and_free_the_block() {
        let mut e = Engine::new();
        e.register(BASE_CLEAN_UP, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0069_19a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0106_abf8);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BASE_CLEAN_UP, vec![this]),
                (BASE_CLEAN_UP, vec![this]),
                (FREE, vec![0x7000])
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0069_1a00, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }
}
