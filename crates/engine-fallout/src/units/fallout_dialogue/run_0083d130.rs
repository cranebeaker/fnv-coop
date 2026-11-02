//! `fallout/dialogue/run_0083d130` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// thiscall on a list node: whether it is empty (item and next both 0).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// thiscall on a list node (or any object): answers it unchanged.
const IDENTITY: u32 = 0x0068_15c0;
/// thiscall on a list node: the next node (+4).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// thiscall: the first word of the object (a string's text).
const FIRST_WORD: u32 = 0x0055_9450;
/// thiscall: the object's address plus 0x10.
const MEMBER_AT_10: u32 = 0x0046_0140;
/// cdecl (text): the string's length.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `BSAudio::QInstance` (engine map), cdecl.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::Precache_ov3` (engine map), thiscall (audio, path, flags, 0).
const AUDIO_PRECACHE: u32 = 0x00ad_8100;
/// The flags the precache gets.
const PRECACHE_FLAGS: u32 = 0x2010_0001;

// Translated from 0083d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called by `Conversation::PrecacheSounds` on
/// each dialogue item): walks the `BSSimpleList` at `this` until an empty
/// node; for each item whose first string (the word at +0) is set and
/// longer than one character, precaches the sound named by the string at
/// +0x10 through `BSAudio` (flags `0x20100001`). The item's class is not
/// confirmed.
pub fn fn_0083d130(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let item = item_of(e, node);
        if e.call(FIRST_WORD, &args![item]).u32() != 0 {
            let item = item_of(e, node);
            let text = e.call(FIRST_WORD, &args![item]).u32();
            if e.call(STRING_LENGTH, &args![text]).u32() > 1 {
                let item = e.call(IDENTITY, &args![node]).u32();
                let item = e.mem.u32(item);
                let member = e.call(MEMBER_AT_10, &args![item]).u32();
                let path = e.call(FIRST_WORD, &args![member]).u32();
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                e.call(AUDIO_PRECACHE, &args![audio, path, PRECACHE_FLAGS, 0u32]);
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// The node's item, through `006815c0` on the node and again on the item,
/// as the code reaches it.
fn item_of(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(IDENTITY, &args![node]).u32();
    let item = e.mem.u32(at);
    e.call(IDENTITY, &args![item]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083d130, fn_0083d130(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn precaches_the_sound_of_items_with_a_long_enough_text() {
        let mut e = Engine::new();
        e.register(IDENTITY, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(MEMBER_AT_10, |_, a| ret(a[0] + 0x10));
        e.register(STRING_LENGTH, |_, a| ret(a[0] & 0xf));
        e.register(AUDIO_INSTANCE, |_, _| ret(0xa0d0));
        e.register(AUDIO_PRECACHE, |_, _| Ret::default());
        // Three items: text length 2 (precached), no text, text length 1.
        let items: Vec<u32> = [0x1002u32, 0, 0x1001]
            .iter()
            .enumerate()
            .map(|(index, &text)| {
                let item = e.mem.alloc(0x20);
                e.mem.set_u32(item, text);
                e.mem.set_u32(item + 0x10, 0x5000 + index as u32);
                item
            })
            .collect();
        let list = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(second, items[1]);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, items[2]);
        e.call_log = Some(vec![]);
        e.call(0x0083_d130, &args![list]);
        let log = e.call_log.take().unwrap();
        let precached: Vec<_> = log.iter().filter(|c| c.0 == AUDIO_PRECACHE).collect();
        assert_eq!(precached.len(), 1);
        assert_eq!(precached[0].1, vec![0xa0d0, 0x5000, PRECACHE_FLAGS, 0]);
    }
}
