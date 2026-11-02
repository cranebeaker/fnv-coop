//! `(unplaced)/run_005594e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005594e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A hash map's `SetAt` for a one-byte value, in the shape of Gamebryo's `NiTMapBase::SetAt`
/// (unnamed in the engine map; the map's class is not confirmed; `ECX` and two stack words,
/// `RET 8`; called by `0054a070`, `004370f0` and `RedrawRefLinks`). The bucket is the map's
/// vtable slot +4 for `key`, in the bucket array at +8; the items are `next`, key, value.
/// An item whose key slot +8 calls equal to `key` is cleared (slot +0x10) and set again
/// (slot +0xc with `key` and `value`); otherwise a new item (slot +0x14) is set, linked at
/// the bucket's head, and the count at +0xc grows by one.
pub fn fn_005594e0(e: &mut Engine, this: Ptr, key: u32, value: u8) {
    let map = this.addr();
    let bucket = e.vcall(map, 4, &args![key]).u32();
    let slot = |e: &Engine| e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    let mut item = e.mem.u32(slot(e));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(map, 8, &args![key, item_key]).u8() != 0 {
            e.vcall(map, 0x10, &args![item]);
            e.vcall(map, 0xc, &args![item, key, u32::from(value)]);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(map, 0x14, &args![]).u32();
    e.vcall(map, 0xc, &args![item, key, u32::from(value)]);
    let head = e.mem.u32(slot(e));
    e.mem.set_u32(item, head);
    let at = slot(e);
    e.mem.set_u32(at, item);
    let count = e.mem.u32(map + 0xc).wrapping_add(1);
    e.mem.set_u32(map + 0xc, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0055_94e0, fn_005594e0(Ptr, u32, u8))]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// A map of 4 buckets keyed by `key % 4`, keys equal when the words are.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(0x0060_0004, |_, a| (a[1] % 4).into_ret());
        e.register(0x0060_0008, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x0060_000c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(0x0060_0010, |_, _| Ret::default());
        e.register(0x0060_0014, |e, _| e.mem.alloc(0xc).into_ret());
        e.put_vtable(
            0x0070_0000,
            &[
                0,
                0x0060_0004,
                0x0060_0008,
                0x0060_000c,
                0x0060_0010,
                0x0060_0014,
            ],
        );
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map, 0x0070_0000);
        e.mem.set_u32(map + 8, buckets);
        e.call_log = Some(vec![]);
        (e, map)
    }

    fn calls(log: &Log, callee: u32) -> usize {
        log.iter().filter(|(a, _)| *a == callee).count()
    }

    #[test]
    fn fn_005594e0_adds_new_keys_and_replaces_existing_ones() {
        let (mut e, map) = engine();
        e.call(0x0055_94e0, &args![map, 5u32, 0x101u32]);
        e.call(0x0055_94e0, &args![map, 9u32, 0u32]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        let buckets = e.mem.u32(map + 8);
        let head = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(head + 4), 9);
        let next = e.mem.u32(head);
        assert_eq!(e.mem.u32(next + 4), 5);
        assert_eq!(e.mem.u32(next + 8), 1);
        e.call(0x0055_94e0, &args![map, 5u32, 7u32]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert_eq!(e.mem.u32(next + 8), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0060_0014), 2);
        assert_eq!(calls(&log, 0x0060_0010), 1);
    }
}
