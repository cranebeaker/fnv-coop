//! `fallout shared/run_0060f240` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Clean-up of the object (`0060f3a0`, `this`), run first by
/// [`fn_0060f260`].
const CLEAR_FIRST: u32 = 0x0060_f3a0;
/// Clean-up of the member at `+4` (`0046ffb0`, `this + 4`).
const DESTRUCT_MEMBER_AT_4: u32 = 0x0046_ffb0;
/// The list walk used by [`fn_0060f2c0`]: the first node of the list at
/// `this` (`00717e50`), a node's item slot (`006815c0`, answers the address
/// of the item pointer) and the next node (`00726070`). The engine map's
/// names for these addresses belong to folded code.
const LIST_FIRST_NODE: u32 = 0x0071_7e50;
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// The byte an item answers (`005dc980`, `this` = the item), compared
/// with the wanted value.
const ITEM_BYTE: u32 = 0x005d_c980;

// Translated from 0060f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes the two bytes at `this` and `this + 1` and returns `this`: the
/// first call of the quest stage constructor `0060f210` (on the stage's
/// two-byte data).
pub fn fn_0060f240(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.mem.set_u8(this.addr() + 1, 0);
    this
}

// Translated from 0060f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor-shaped clean-up: `0060f3a0` on `this`, then `0046ffb0` on the
/// member at `this + 4`.
// Translation incomplete: 0060f260 - SEH exception unwinding (FS:[0] frame state) is not translated.
pub fn fn_0060f260(e: &mut Engine, this: Ptr) {
    e.call(CLEAR_FIRST, &args![this]);
    e.call(DESTRUCT_MEMBER_AT_4, &args![this.addr() + 4]);
}

// Translated from 0060f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list at `this` and answers the first non-null item whose byte
/// (`005dc980`) equals `value`, or 0 when none does.
pub fn fn_0060f2c0(e: &mut Engine, this: Ptr, value: u8) -> u32 {
    let mut node = e.call(LIST_FIRST_NODE, &args![this]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 && e.call(ITEM_BYTE, &args![item]).u32() as u8 == value {
            return item;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060f240, fn_0060f240(Ptr) -> Ptr),
        entry!(0x0060f260, fn_0060f260(Ptr)),
        entry!(0x0060f2c0, fn_0060f2c0(Ptr, u8) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn stage_data_constructor_zeroes_two_bytes() {
        let mut e = Engine::new();
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0xaabb_ccdd);
        assert_eq!(e.call(0x0060_f240, &[data]).u32(), data);
        assert_eq!(e.mem.u32(data), 0xaabb_0000);
    }

    #[test]
    fn clean_up_runs_both_parts_in_order() {
        let mut e = Engine::new();
        e.register(CLEAR_FIRST, |_, _| Ret::default());
        e.register(DESTRUCT_MEMBER_AT_4, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_f260, &[0x5000]);
        let log: Vec<_> = e.call_log.take().unwrap();
        assert_eq!(
            log,
            [
                (0x0060_f260, vec![0x5000]),
                (CLEAR_FIRST, vec![0x5000]),
                (DESTRUCT_MEMBER_AT_4, vec![0x5004]),
            ]
        );
    }

    /// A list of nodes of 8 bytes: the item pointer, then the next node.
    /// The list object holds its first node; an item's byte is its first
    /// byte.
    #[test]
    fn find_answers_the_first_item_with_the_byte() {
        let mut e = Engine::new();
        e.register(LIST_FIRST_NODE, |e, a| eax(e.mem.u32(a[0])));
        e.register(NODE_ITEM_SLOT, |_, a| eax(a[0]));
        e.register(NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        // Upper bytes set: only AL is compared.
        e.register(ITEM_BYTE, |e, a| eax(0xffff_ff00 | e.mem.u8(a[0]) as u32));
        let list = e.mem.alloc(4);
        let nodes = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        let second = e.mem.alloc(4);
        let third = e.mem.alloc(4);
        e.mem.set_u8(second, 5);
        e.mem.set_u8(third, 5);
        e.mem.set_u32(list, nodes[0]);
        // First node: null item; then two items with byte 5.
        e.mem.set_u32(nodes[0] + 4, nodes[1]);
        e.mem.set_u32(nodes[1], second);
        e.mem.set_u32(nodes[1] + 4, nodes[2]);
        e.mem.set_u32(nodes[2], third);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060_f2c0, &[list, 5]).u32(), second);
        assert_eq!(calls_to(&e, ITEM_BYTE), [[second]]);
        assert_eq!(e.call(0x0060_f2c0, &[list, 7]).u32(), 0);
        assert_eq!(calls_to(&e, NODE_NEXT).len(), 1 + 3);
        e.mem.set_u32(list, 0);
        assert_eq!(e.call(0x0060_f2c0, &[list, 5]).u32(), 0);
    }
}
