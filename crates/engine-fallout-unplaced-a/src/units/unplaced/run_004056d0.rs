//! `(unplaced)/run_004056d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`ECX`): answers the address of an entry's item word.
const ITEM_ADDRESS_006815C0: u32 = 0x0068_15c0;
/// `00825c00` (`ECX` = effect item): the word it answers is handed to `0084e3a0`.
const EFFECT_ITEM_WORD_00825C00: u32 = 0x0082_5c00;
/// `0084e3a0` (`ECX`): a word of the object `00825c00` answers (written after `EFID`).
const RECORD_WORD_0084E3A0: u32 = 0x0084_e3a0;
/// `00485910` (`cdecl`, two words: chunk code, value): writes one chunk of a form record.
const WRITE_CHUNK_00485910: u32 = 0x0048_5910;
/// `EffectItem::Save` (Xbox PDB, `004040b0`, `ECX`).
const EFFECT_ITEM_SAVE: u32 = 0x0040_40b0;
/// `00726070` (`ECX`): the next entry's link word.
const NEXT_LINK_00726070: u32 = 0x0072_6070;
/// The chunk code `EFID` (the four characters, first byte lowest).
const CHUNK_EFID: u32 = 0x4449_4645;

// Translated from 004056d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`; called by `0040c1d0` and
/// `0040c5f0`): walks a list of effect items starting at the entry `ECX`. For each entry,
/// the item word whose address `006815c0` answers for the entry's +4; when it is not null,
/// writes an `EFID` chunk (`00485910`) with the word `0084e3a0` answers for the object
/// `00825c00` answers for the item, then `EffectItem::Save` on the item. The next entry is
/// the link `00726070` answers for the entry's +4, minus 4 (null stays null).
pub fn fn_004056d0(e: &mut Engine, list: Ptr) {
    let mut entry = list.addr();
    while entry != 0 {
        let item_address = e
            .call(ITEM_ADDRESS_006815C0, &args![entry.wrapping_add(4)])
            .u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            let owner = e.call(EFFECT_ITEM_WORD_00825C00, &args![item]).u32();
            let value = e.call(RECORD_WORD_0084E3A0, &args![owner]).u32();
            e.call(WRITE_CHUNK_00485910, &args![CHUNK_EFID, value]);
            e.call(EFFECT_ITEM_SAVE, &args![item]);
        }
        let next = e
            .call(NEXT_LINK_00726070, &args![entry.wrapping_add(4)])
            .u32();
        entry = if next == 0 { 0 } else { next.wrapping_sub(4) };
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_56d0, fn_004056d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004056d0_saves_every_item_of_the_list() {
        let mut e = Engine::new();
        // Entries: [vtable-ish word, item, next link]; the link points at the next entry's +4.
        let first = e.mem.alloc(12);
        let second = e.mem.alloc(12);
        e.mem.set_u32(first + 4, 0x7000);
        e.mem.set_u32(first + 8, second + 4);
        e.mem.set_u32(second + 4, 0);
        e.mem.set_u32(second + 8, 0);
        e.register(ITEM_ADDRESS_006815C0, |_, a| a[0].into_ret());
        e.register(NEXT_LINK_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_ITEM_WORD_00825C00, |_, a| (a[0] + 1).into_ret());
        e.register(RECORD_WORD_0084E3A0, |_, a| (a[0] + 2).into_ret());
        e.register(WRITE_CHUNK_00485910, |_, _| Ret::default());
        e.register(EFFECT_ITEM_SAVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_56d0, &args![first]);
        let log = e.call_log.take().unwrap();
        let writes: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == WRITE_CHUNK_00485910 || *a == EFFECT_ITEM_SAVE)
            .cloned()
            .collect();
        assert_eq!(
            writes,
            vec![
                (WRITE_CHUNK_00485910, vec![CHUNK_EFID, 0x7003]),
                (EFFECT_ITEM_SAVE, vec![0x7000]),
            ]
        );
        let links = log.iter().filter(|(a, _)| *a == NEXT_LINK_00726070).count();
        assert_eq!(links, 2);
        // An empty start does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0040_56d0, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
