//! `fallout/magic/run_00806c20` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActiveEffect`'s constructor body (`00803d30`, `thiscall`, three words:
/// caster, magic item, effect item).
const ACTIVE_EFFECT_CONSTRUCT: u32 = 0x0080_3d30;
/// The vtable the constructor stores (`0107951c`, RTTI
/// `.?AVAssociatedItemEffect@@`).
const ASSOCIATED_ITEM_EFFECT_VTABLE: u32 = 0x0107_951c;
/// `00825c00` (`thiscall`): the effect item's word at +0x14 (its effect
/// setting).
const EFFECT_ITEM_SETTING_00825C00: u32 = 0x0082_5c00;
/// `005f5f80` (`thiscall`): the effect setting's word at +0x60.
const SETTING_WORD_60_005F5F80: u32 = 0x005f_5f80;

// Translated from 00806c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the class whose vtable is `0107951c` (RTTI
/// `AssociatedItemEffect`; `thiscall`, `RET 0xC`: caster, magic item,
/// effect item): constructs the `ActiveEffect` base (`00803d30`) with the
/// three words, stores the vtable, and stores at +0x48 the word +0x60 of
/// the effect item's effect setting (`00825c00`, then `005f5f80`). Returns
/// `this`.
// Translation incomplete: 00806c20 - the FS:[0] exception frame (unwinding the base on a throw) is not translated
pub fn fn_00806c20(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(ACTIVE_EFFECT_CONSTRUCT, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), ASSOCIATED_ITEM_EFFECT_VTABLE);
    let setting = e.call(EFFECT_ITEM_SETTING_00825C00, &args![effect]).u32();
    let item = e.call(SETTING_WORD_60_005F5F80, &args![setting]).u32();
    e.mem.set_u32(this.addr().wrapping_add(0x48), item);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_6c20, fn_00806c20(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_the_vtable_and_the_associated_item() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(EFFECT_ITEM_SETTING_00825C00, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(SETTING_WORD_60_005F5F80, |_, a| Ret {
            eax: a[0] + 0x60,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        let r = e.call(0x0080_6c20, &args![this, 0x10u32, 0x20u32, 0x3000u32]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ACTIVE_EFFECT_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![this, 0x10, 0x20, 0x3000]);
        assert_eq!(e.mem.u32(this), ASSOCIATED_ITEM_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x48), 0x3160);
    }
}
