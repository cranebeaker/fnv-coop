//! `fallout shared/tesanimation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `008256d0` (thiscall): true when both words of the list at `this` (+0
/// and +4) are zero.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;

// Translated from 0047fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimation::HasKFFiles` (Xbox PDB name in the engine map): the
/// inverse of `008256d0` (both words zero) on the list at `this + 4`.
pub fn tes_animation_has_kf_files(e: &mut Engine, this: Ptr) -> bool {
    !e.call(LIST_IS_EMPTY, &args![this.byte_add(4)]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047_fd90, tes_animation_has_kf_files(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_kf_files_inverts_the_list_test() {
        let mut e = Engine::new();
        e.register(LIST_IS_EMPTY, |e, a| Ret {
            eax: (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32 | 0x100,
            ..Ret::default()
        });
        let this = e.mem.alloc(0xc);
        assert!(!e.call(0x0047_fd90, &args![this]).bool());
        e.mem.set_u32(this + 8, 1);
        assert!(e.call(0x0047_fd90, &args![this]).bool());
    }
}
