//! `BSMovie/run_00ec2f00` (the engine map's run of unnamed functions from `00ec2f00`),
//! subsystem `BSMovie`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocation (`00aa3e40`, `thiscall`, one word); the
/// map's vtable slots: +4 the key's bucket index (one word), +8 whether two keys are equal
/// (two words; answer `AL`), +0xc an item's set-up (three words).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const KEY_TO_HASH_INDEX: u32 = 0x4;
const IS_KEYS_EQUAL: u32 = 0x8;
const SET_VALUE: u32 = 0xc;

// Translated from 00ec2f00 (disassembled, FalloutNV.exe 1.4.0.525)
/// A string map's set (unnamed in the engine map; `thiscall`, two words: a key and a value,
/// `RET 8`). The key's bucket (slot +4) of the bucket array (+8) is walked along the items'
/// next words (+0): for an item whose key (+4) equals the key (slot +8 with the key and the
/// item's key), the key is stored in it unless the map owns its keys (byte +0x10), and the
/// value (+8) always; done. Otherwise a 12-byte item (`00aa3e40`, not checked for null) is set
/// up (slot +0xc with the item, the key and the value), linked at the head of the bucket, and
/// the count (+0xc) goes up. The exe also counts the items it walks in a local nobody reads.
pub fn fn_00ec2f00(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let at = this.addr();
    let index = e.vcall(at, KEY_TO_HASH_INDEX, &args![key]).u32();
    let buckets = e.mem.u32(at + 8);
    let mut item = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(at, IS_KEYS_EQUAL, &args![key, item_key]).u8() != 0 {
            if e.mem.u8(at + 0x10) == 0 {
                e.mem.set_u32(item + 4, key);
            }
            e.mem.set_u32(item + 8, value);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0xcu32]).u32();
    e.vcall(at, SET_VALUE, &args![item, key, value]);
    let slot = e.mem.u32(at + 8).wrapping_add(index.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(item, head);
    let slot = e.mem.u32(at + 8).wrapping_add(index.wrapping_mul(4));
    e.mem.set_u32(slot, item);
    let count = e.mem.u32(at + 0xc).wrapping_add(1);
    e.mem.set_u32(at + 0xc, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec_2f00, fn_00ec2f00(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0900_0000;
    const INDEX: u32 = 0x0a00_0000;
    const EQUAL: u32 = 0x0a00_0100;
    const SET: u32 = 0x0a00_0200;

    fn map(e: &mut Engine) -> u32 {
        e.put_vtable(VTABLE, &[0, INDEX, EQUAL, SET]);
        // Two buckets; the index is the key's low bit.
        e.register(INDEX, |_, a| (a[1] & 1).into_ret());
        e.register(EQUAL, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(SET, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(this, VTABLE);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, buckets);
        this
    }

    #[test]
    fn new_keys_go_to_the_bucket_head_and_known_ones_change_value() {
        let mut e = Engine::new();
        let this = map(&mut e);
        e.call(0x00ec_2f00, &args![this, 3u32, 30u32]);
        e.call(0x00ec_2f00, &args![this, 5u32, 50u32]);
        let buckets = e.mem.u32(this + 8);
        let head = e.mem.u32(buckets + 4);
        assert_eq!((e.mem.u32(head + 4), e.mem.u32(head + 8)), (5, 50));
        let next = e.mem.u32(head);
        assert_eq!((e.mem.u32(next + 4), e.mem.u32(next + 8)), (3, 30));
        assert_eq!(e.mem.u32(next), 0);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        // A known key: the value changes, nothing is added.
        fn_00ec2f00(&mut e, Ptr::new(this), 3, 31);
        assert_eq!(e.mem.u32(next + 8), 31);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        // Keys owned by the map (byte +0x10): the item keeps its key word.
        e.mem.set_u8(this + 0x10, 1);
        e.register(EQUAL, |_, _| 1u32.into_ret());
        fn_00ec2f00(&mut e, Ptr::new(this), 7, 70);
        assert_eq!((e.mem.u32(head + 4), e.mem.u32(head + 8)), (5, 70));
    }
}
