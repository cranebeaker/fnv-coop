//! `fallout shared/magic/run_00409060` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The effect setting list (a `BSSimpleList` of `EffectSetting *` at
/// `011c3400`).
const EFFECT_SETTINGS: u32 = 0x011c_3400;
/// Thiscall on the list (address of the item word), `RET 4`: adds the item.
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;

// Translated from 00409060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): a non-null `setting` is added
/// to the effect setting list at `011c3400` (`005ae3d0`, given the address
/// of the argument's stack slot).
pub fn fn_00409060(e: &mut Engine, setting: u32) {
    if setting != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), setting);
            e.call(LIST_ADD_005AE3D0, &args![EFFECT_SETTINGS, slot]);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_9060, fn_00409060(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_a_non_null_setting_to_the_list() {
        let mut e = Engine::new();
        e.register(LIST_ADD_005AE3D0, |e, a| Ret {
            eax: e.mem.u32(a[1]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0040_9060, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x0040_9060, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        let adds: Vec<_> = log.iter().filter(|c| c.0 == LIST_ADD_005AE3D0).collect();
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0].1[0], EFFECT_SETTINGS);
    }
}
