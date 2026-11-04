//! `fallout shared/tes.cpp` (Xbox PDB source unit), part 3: its functions from `004fd3e0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tes`]; anything public there may be used here.

#[allow(unused_imports)]
use super::tes::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fd3e0, tes_get_world_space(Ptr<TES>) -> u32),
        entry!(0x0088b0c0, fn_0088b0c0(Ptr) -> bool),
    ]
}

// Translated from 004fd3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetWorldSpace` (Xbox PDB): the current world space (`pWorldSpace`).
pub fn tes_get_world_space(e: &mut Engine, this: Ptr<TES>) -> u32 {
    e.get(this, TES::pWorldSpace).addr()
}

// Translated from 0088b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this body `TES::GetWaterHeight`, but it is a flag test on
/// the object embedded at `this + 0x410` (not a `TES`, which is 0xC4 bytes):
/// whether `00621270` (the object's word at +4 masked with `0x100`) is
/// non-zero. The owning class is not confirmed from the exe.
pub fn fn_0088b0c0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![this.addr() + 0x410, 0x100u32])
        .u32()
        != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_world_space_returns_the_field() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        e.set(tes, TES::pWorldSpace, Ptr::new(0x7000));
        assert_eq!(e.call(0x004f_d3e0, &args![tes]).u32(), 0x7000);
    }

    #[test]
    fn flag_test_is_true_when_the_masked_flags_are_set() {
        let mut e = Engine::new();
        let obj = e.mem.alloc(0x420);
        e.register(0x0062_1270, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) & a[1],
            ..Ret::default()
        });
        assert!(!e.call(0x0088_b0c0, &args![obj]).bool());
        e.mem.set_u32(obj + 0x414, 0x100);
        assert!(e.call(0x0088_b0c0, &args![obj]).bool());
    }
}
