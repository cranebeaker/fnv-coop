//! `(unplaced)/run_004f10d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two slots of `TESRegion`'s vtable (`0102397c`, by its RTTI name), at +0x138 and +0x13c.

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator (`00401000`, `cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
/// The `TESRegion` constructor (`004f0770`, `ECX`), 0x38 bytes.
const REGION_CONSTRUCT: u32 = 0x004f_0770;
/// `00441110` and `009611e0` (`ECX` = the region): the entry list and the second list it
/// copies.
const ENTRY_LIST_00441110: u32 = 0x0044_1110;
const OTHER_LIST_009611E0: u32 = 0x0096_11e0;
/// `TESForm::Copy` (engine map, `00485130`, `ECX`, one stack word: the destination).
const FORM_COPY: u32 = 0x0048_5130;
/// `004f3470` (`ECX`, one stack word): copies the second list into another.
const COPY_LIST_004F3470: u32 = 0x004f_3470;
/// The list helpers: an entry's item word address (`006815c0`), the next entry
/// (`00726070`), and the store into an entry (`005ae3d0`, one stack word: the address of
/// the item word).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_STORE: u32 = 0x005a_e3d0;
/// `004f80c0` (`ECX` = an entry, one stack word): a copy of the entry.
const ENTRY_COPY: u32 = 0x004f_80c0;
/// `004f3610` (`ECX` = the +0x18 object), `AL`.
const OBJECT_TEST_004F3610: u32 = 0x004f_3610;
/// `004f81e0` (`ECX` = an entry, one stack word), `AL`.
const ENTRY_TEST_004F81E0: u32 = 0x004f_81e0;

// Translated from 004f10d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f10d0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef2e1b, state variable around the constructor) and the frame's cookie word are not translated.
/// Slot +0x138 of `TESRegion`'s vtable (unnamed in the engine map; `ECX` and one stack
/// word, `RET 4`): copies this region into `target`, or into a new region (0x38 bytes,
/// `004f0770`; null when the allocation fails) when `target` is null. `TESForm::Copy`
/// into it, the second list copied (`004f3470` on this one's with the target's), and each
/// entry of this one's entry list up to the first null one copied (`004f80c0` with 0) and
/// stored into the target's list (`005ae3d0`). Answers the target.
pub fn fn_004f10d0(e: &mut Engine, this: Ptr, target: u32) -> u32 {
    let target = if target != 0 {
        target
    } else {
        let block = e.call(ALLOCATE, &args![0x38u32]).u32();
        if block == 0 {
            0
        } else {
            e.call(REGION_CONSTRUCT, &args![block]).u32()
        }
    };
    let mut node = e.call(ENTRY_LIST_00441110, &args![this]).u32();
    e.call(FORM_COPY, &args![this, target]);
    let theirs = e.call(OTHER_LIST_009611E0, &args![target]).u32();
    let mine = e.call(OTHER_LIST_009611E0, &args![this]).u32();
    e.call(COPY_LIST_004F3470, &args![mine, theirs]);
    e.with_stack(4, |e, slot| {
        while node != 0 {
            let item_address = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item_address) == 0 {
                break;
            }
            let item_address = e.call(NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(item_address);
            let copy = e.call(ENTRY_COPY, &args![item, 0u32]).u32();
            e.mem.set_u32(slot.addr(), copy);
            let list = e.call(ENTRY_LIST_00441110, &args![target]).u32();
            e.call(NODE_STORE, &args![list, slot]);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    target
}

// Translated from 004f11e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x13c of `TESRegion`'s vtable (unnamed in the engine map; `ECX`, plain `RET`):
/// false without the list at +0x1c or the object at +0x18, or when `004f3610` answers false
/// for that object, or when an entry of the list (up to the first null one) answers false
/// to `004f81e0(1)`; true otherwise.
pub fn fn_004f11e0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let mut node = e.mem.u32(base + 0x1c);
    if e.mem.u32(base + 0x1c) == 0 || e.mem.u32(base + 0x18) == 0 {
        return false;
    }
    let object = e.mem.u32(base + 0x18);
    if e.call(OBJECT_TEST_004F3610, &args![object]).u8() == 0 {
        return false;
    }
    while node != 0 {
        let item_address = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if e.call(ENTRY_TEST_004F81E0, &args![item, 1u32]).u8() == 0 {
            return false;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_10d0, fn_004f10d0(Ptr, u32) -> u32),
        entry!(0x004f_11e0, fn_004f11e0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A list of nodes [item, next].
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let nodes: Vec<u32> = items.iter().map(|_| e.mem.alloc(8)).collect();
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(nodes[i], *item);
            e.mem
                .set_u32(nodes[i] + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        nodes[0]
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e
    }

    #[test]
    fn fn_004f10d0_copies_into_the_target_or_a_new_region() {
        let mut e = engine();
        let entries = list(&mut e, &[0x111, 0x222, 0]);
        e.register_double(ENTRY_LIST_00441110, move |_, a| {
            (if a[0] == 0x9000 { entries } else { a[0] + 0x1c }).into_ret()
        });
        e.register(OTHER_LIST_009611E0, |_, a| (a[0] + 0x18).into_ret());
        e.register(ENTRY_COPY, |_, a| (a[0] + 1).into_ret());
        e.register(NODE_STORE, |_, _| Ret::default());
        e.register(FORM_COPY, |_, _| Ret::default());
        e.register(COPY_LIST_004F3470, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(REGION_CONSTRUCT, |_, a| (a[0] + 0x10).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004f_10d0, &args![0x9000u32, 0x7000u32]).u32(),
            0x7000
        );
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ALLOCATE).is_empty());
        assert_eq!(calls(&log, FORM_COPY), vec![vec![0x9000, 0x7000]]);
        assert_eq!(calls(&log, COPY_LIST_004F3470), vec![vec![0x9018, 0x7018]]);
        assert_eq!(
            calls(&log, ENTRY_COPY),
            vec![vec![0x111, 0], vec![0x222, 0]]
        );
        let stores = calls(&log, NODE_STORE);
        assert_eq!(stores.len(), 2);
        assert!(stores.iter().all(|w| w[0] == 0x701c));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_10d0, &args![0x9000u32, 0u32]).u32(), 0x5010);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REGION_CONSTRUCT), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_004f11e0_tests_the_object_and_each_entry() {
        let mut e = engine();
        let region = e.mem.alloc(0x20);
        assert!(!e.call(0x004f_11e0, &args![region]).bool());
        let entries = list(&mut e, &[0x111, 0x222, 0]);
        e.mem.set_u32(region + 0x1c, entries);
        e.mem.set_u32(region + 0x18, 0x6000);
        e.register(OBJECT_TEST_004F3610, |_, _| 0u32.into_ret());
        assert!(!e.call(0x004f_11e0, &args![region]).bool());
        e.register(OBJECT_TEST_004F3610, |_, _| 1u32.into_ret());
        e.register(ENTRY_TEST_004F81E0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_11e0, &args![region]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ENTRY_TEST_004F81E0),
            vec![vec![0x111, 1], vec![0x222, 1]]
        );
        e.register(ENTRY_TEST_004F81E0, |_, a| {
            u32::from(a[0] != 0x222).into_ret()
        });
        assert!(!e.call(0x004f_11e0, &args![region]).bool());
    }
}
