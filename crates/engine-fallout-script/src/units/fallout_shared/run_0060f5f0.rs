//! `fallout shared/run_0060f5f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::tesquest::TESQuestStageItem;

/// A global word cleared first (`011cb86c`) and a global string emptied
/// next (`011cb8a0`, through `004037f0`, which sets a string from a text
/// and a length; a null text empties it).
const CLEARED_WORD: u32 = 0x011c_b86c;
const EMPTIED_STRING: u32 = 0x011c_b8a0;
const STRING_SET: u32 = 0x0040_37f0;
/// The log date object's deleting destructor (`007b3fa0`, one stack word:
/// the delete flag).
const LOG_DATE_DELETE: u32 = 0x007b_3fa0;
/// The embedded `Script`'s destructor (`005aa1a0`, `this + 0xc`) and the
/// condition list's (`006808b0`, `this + 4`).
const SCRIPT_DESTRUCT: u32 = 0x005a_a1a0;
const CONDITION_DESTRUCT: u32 = 0x0068_08b0;

// Translated from 0060f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of a quest stage item (`TESQuestStageItem`, the
/// layout its constructor `0060f520` builds; called by its deleting
/// destructor `0060f410`): clears the global word at `011cb86c`, empties
/// the global string at `011cb8a0` (`004037f0` with a null text and 0),
/// deletes the log date (`m_pLogDate`, `+0x68`) when there is one
/// (`007b3fa0` with 1), then destroys the embedded script (`+0xc`) and the
/// condition list (`+4`).
// Translation incomplete: 0060f5f0 - SEH exception unwinding (FS:[0] frame state) is not translated.
pub fn fn_0060f5f0(e: &mut Engine, this: Ptr<TESQuestStageItem>) {
    e.set_global::<u32>(CLEARED_WORD, 0);
    e.call(STRING_SET, &args![EMPTIED_STRING, 0u32, 0u32]);
    let log_date = e.get(this, TESQuestStageItem::m_pLogDate);
    if log_date != Ptr::NULL {
        e.call(LOG_DATE_DELETE, &args![log_date, 1u32]);
    }
    e.call(SCRIPT_DESTRUCT, &args![this.addr() + 0xc]);
    e.call(CONDITION_DESTRUCT, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0060f5f0, fn_0060f5f0(Ptr<TESQuestStageItem>))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(log_date: u32) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        e.map(CLEARED_WORD & !0xfff, 0x1000);
        e.set_global::<u32>(CLEARED_WORD, 0x1234);
        for addr in [
            STRING_SET,
            LOG_DATE_DELETE,
            SCRIPT_DESTRUCT,
            CONDITION_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let item: Ptr<TESQuestStageItem> = e.new_object();
        e.set(item, TESQuestStageItem::m_pLogDate, Ptr::new(log_date));
        e.call_log = Some(vec![]);
        e.call(0x0060_f5f0, &[item.addr()]);
        assert_eq!(e.global::<u32>(CLEARED_WORD), 0);
        let base = item.addr();
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .skip(1)
            .map(|(a, args)| (a, args.into_iter().map(|w| w.wrapping_sub(base)).collect()))
            .collect()
    }

    #[test]
    fn destroys_the_parts_and_the_log_date() {
        let log = run(0x9000);
        // Words relative to the item's address (the string and the log
        // date are not item fields, so only their order is checked here).
        let addrs: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addrs,
            [
                STRING_SET,
                LOG_DATE_DELETE,
                SCRIPT_DESTRUCT,
                CONDITION_DESTRUCT
            ]
        );
        assert_eq!(log[2].1, [0xc]);
        assert_eq!(log[3].1, [4]);
    }

    #[test]
    fn without_a_log_date_skips_its_delete() {
        let log = run(0);
        let addrs: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(addrs, [STRING_SET, SCRIPT_DESTRUCT, CONDITION_DESTRUCT]);
    }

    #[test]
    fn empties_the_string_and_deletes_with_flag_1() {
        let mut e = Engine::new();
        e.map(CLEARED_WORD & !0xfff, 0x1000);
        for addr in [
            STRING_SET,
            LOG_DATE_DELETE,
            SCRIPT_DESTRUCT,
            CONDITION_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let item: Ptr<TESQuestStageItem> = e.new_object();
        e.set(item, TESQuestStageItem::m_pLogDate, Ptr::new(0x9000));
        e.call_log = Some(vec![]);
        e.call(0x0060_f5f0, &[item.addr()]);
        let log = e.call_log.take().unwrap();
        let find = |addr: u32| log.iter().find(|(a, _)| *a == addr).unwrap().1.clone();
        assert_eq!(find(STRING_SET), [EMPTIED_STRING, 0, 0]);
        assert_eq!(find(LOG_DATE_DELETE), [0x9000, 1]);
    }
}
