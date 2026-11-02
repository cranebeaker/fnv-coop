//! `fallout shared/teslevcreature.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0` (`thiscall`): whether bit 3 (0x8) of the form's flag word at
/// +8 is set.
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`thiscall`, one byte): sets (non-zero) or clears bit 3 of the
/// form's flag word at +8.
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
/// `TESLeveledList::InitItem` (`00488570`, Xbox PDB; `thiscall`, one word:
/// the owner).
const LEVELED_LIST_INIT_ITEM: u32 = 0x0048_8570;
/// `TESModelTextureSwap::InitItem` (`0048aa80`, engine map; `thiscall`, one
/// word: the owner).
const MODEL_TEXTURE_SWAP_INIT_ITEM: u32 = 0x0048_aa80;

// Translated from 0050ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLevCreature::InitItem` (Xbox PDB), `thiscall`: unless the form's flag
/// bit 0x8 (+8) is already set, initializes its `TESLeveledList` (+0x30) and
/// `TESModelTextureSwap` (+0x4C) components with the form as their owner,
/// then sets the bit.
pub fn tes_lev_creature_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.call(LEVELED_LIST_INIT_ITEM, &args![this.addr() + 0x30, this]);
    e.call(
        MODEL_TEXTURE_SWAP_INIT_ITEM,
        &args![this.addr() + 0x4c, this],
    );
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_ad40, tes_lev_creature_init_item(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn init_item_initializes_the_components_once() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.register(FORM_FLAG_8_SET, |_, _| word(0));
        for callee in [
            LEVELED_LIST_INIT_ITEM,
            MODEL_TEXTURE_SWAP_INIT_ITEM,
            FORM_SET_FLAG_8,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_ad40, &args![this]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0050_ad40 && c.0 != FORM_FLAG_8_SET)
            .collect();
        assert_eq!(
            log,
            vec![
                (LEVELED_LIST_INIT_ITEM, vec![this + 0x30, this]),
                (MODEL_TEXTURE_SWAP_INIT_ITEM, vec![this + 0x4c, this]),
                (FORM_SET_FLAG_8, vec![this, 1]),
            ]
        );

        e.register(FORM_FLAG_8_SET, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x0050_ad40, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
