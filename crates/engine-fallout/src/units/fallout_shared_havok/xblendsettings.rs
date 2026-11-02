//! `fallout shared/havok/xblendsettings.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The class's destructor body (`00632f90`; it stores the vtable `0104e6e8`
/// and has an exception frame, so it is translated on its own).
const DESTRUCTOR: u32 = 0x0063_2f90;
/// `00401030(block)`: the game's `operator delete` (cdecl, one argument).
const FREE: u32 = 0x0040_1030;

// Translated from 00632f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<BlendSettingCollection>::_scalar_deleting_destructor_` (Xbox
/// PDB name, from the engine map): runs the destructor body and, when bit 0
/// of `flags` is set, frees `this`. Returns `this`.
pub fn setting_t_blend_setting_collection_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0063_2f60,
        setting_t_blend_setting_collection_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn bit_zero_clear_destroys_without_freeing() {
        let mut e = engine();
        let result = e.call(0x0063_2f60, &args![0x5000u32, 0u32]).u32();
        assert_eq!(result, 0x5000);
        assert_eq!(logged(&e, DESTRUCTOR), vec![vec![0x5000]]);
        assert!(logged(&e, FREE).is_empty());
        // Only bit 0 counts.
        e.call(0x0063_2f60, &args![0x5000u32, 6u32]);
        assert!(logged(&e, FREE).is_empty());
    }

    #[test]
    fn bit_zero_set_frees_after_the_destructor() {
        let mut e = engine();
        let result = e.call(0x0063_2f60, &args![0x5000u32, 1u32]).u32();
        assert_eq!(result, 0x5000);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a == DESTRUCTOR || *a == FREE)
            .collect();
        assert_eq!(order, vec![DESTRUCTOR, FREE]);
        assert_eq!(logged(&e, FREE), vec![vec![0x5000]]);
    }
}
