//! `fallout/ai/arrowprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object at `011e0e80` the high and middle-high process loops read
/// (`00717e50` on it returns its address + 4, the list object the counts and
/// items below are read from).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `00717e50` (`thiscall`, no stack words): `this + 4`. The engine map's
/// name for it (`BaseProcess::GetActorPackageThatIsRunning`) is a folded
/// body.
const PROCESS_LIST_OBJECT: u32 = 0x0071_7e50;
/// On that object, with a list index (0 or 1): the end index
/// (`005be5c0`, `thiscall(index)`), the first index (`005de0f0`,
/// `thiscall(index)`) and the reference at a position (`00968670`,
/// `thiscall(position)`).
const LIST_END: u32 = 0x005b_e5c0;
const LIST_BEGIN: u32 = 0x005d_e0f0;
const LIST_ITEM: u32 = 0x0096_8670;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map; `008d8520`,
/// `thiscall`), which the code calls on the target and then uses as an
/// object with a vtable.
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `0045cd60` (`thiscall`) on that object: compared with 0 and 1.
const OBJECT_KIND: u32 = 0x0045_cd60;
/// The node walk of the list that slot 0x6B0 returns: done test (`008256d0`),
/// the node's item address (`006815c0`), the next node (`00726070`), and the
/// list's remove-all (`00470470`).
const NODE_DONE: u32 = 0x0082_56d0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// On an arrow: the reference it is attached to (`0085f810`), the call made
/// when it is (`008d0240`), a test (`00440d80`, `bool`) and the call with 3
/// (`008cc6d0`, `thiscall(word)`).
const ARROW_ATTACHED_TO: u32 = 0x0085_f810;
const ARROW_DETACH: u32 = 0x008d_0240;
const REFERENCE_TEST: u32 = 0x0044_0d80;
const ARROW_REMOVE: u32 = 0x008c_c6d0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl(object, vfdelta, source type,
/// target type, is reference)`), with the type descriptors of `MobileObject`
/// (`01184920`) and `ArrowProjectile` (`011a28e0`; names read at the
/// descriptors).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const MOBILE_OBJECT_TYPE: u32 = 0x0118_4920;
const ARROW_PROJECTILE_TYPE: u32 = 0x011a_28e0;

/// The arrow steps of one reference, shared by the three loops: when `arrow`
/// is attached to `target` it is detached (`008d0240`) and, unless the test
/// `00440d80` on `tested` holds, removed (slot 0xC4 with 1 when `by_slot`,
/// else `008cc6d0` with 3); then, when the test on `arrow` holds, slot 0x1CC
/// with (0, 1).
fn handle_arrow(e: &mut Engine, target: Ptr, arrow: u32, tested: u32, by_slot: bool) {
    if e.call(ARROW_ATTACHED_TO, &args![arrow]).u32() == target.addr() {
        e.call(ARROW_DETACH, &args![arrow]);
        if !e.call(REFERENCE_TEST, &args![tested]).bool() {
            if by_slot {
                e.vcall(arrow, 0xc4, &args![1u32]);
            } else {
                e.call(ARROW_REMOVE, &args![arrow, 3u32]);
            }
        }
    }
    if e.call(REFERENCE_TEST, &args![arrow]).bool() {
        e.vcall(arrow, 0x1cc, &args![0u32, 1u32]);
    }
}

/// The reference at `position` of the list object of `011e0e80` (`00968670`), cast to an
/// `ArrowProjectile` when its slot 0x224 says so (null otherwise), with the
/// uncast reference.
fn arrow_at(e: &mut Engine, position: u32) -> (u32, u32) {
    let list = e.call(PROCESS_LIST_OBJECT, &args![PROCESS_LISTS]).u32();
    let reference = e.call(LIST_ITEM, &args![list, position]).u32();
    if !e.vcall(reference, 0x224, &[]).bool() {
        return (reference, 0);
    }
    let arrow = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                reference,
                0u32,
                MOBILE_OBJECT_TYPE,
                ARROW_PROJECTILE_TYPE,
                0u32
            ],
        )
        .u32();
    (reference, arrow)
}

/// The end index of list `index` of `011e0e80`.
fn list_end(e: &mut Engine, index: u32) -> u32 {
    let list = e.call(PROCESS_LIST_OBJECT, &args![PROCESS_LISTS]).u32();
    e.call(LIST_END, &args![list, index]).u32()
}

// Translated from 008cddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrowProjectile::DeleteAttachedArrows` (Xbox PDB; `cdecl(target,
/// by_slot)`): nothing for a null `target` or one whose slot 0x224 is true.
/// Otherwise, when its slot 0x100 is true and `008d8520` gives an object,
/// walks the list that object's slot 0x6B0 returns (when non-null) handling
/// each arrow (see `handle_arrow`), empties that list, and picks the lists to
/// scan by `0045cd60`: 0 scans list 0 only, 1 list 1 only, anything else
/// (and no object) both. List 0 of `011e0e80` is scanned from position 0,
/// list 1 from its first index (`005de0f0`); each reference whose slot 0x224
/// is true and that casts to `ArrowProjectile` is handled (list 1 always
/// through slot 0xC4). In the list loops the removal test reads the uncast
/// reference, as the exe does.
pub fn arrow_projectile_delete_attached_arrows(e: &mut Engine, target: Ptr, by_slot: bool) {
    if target.is_null() || e.vcall(target.addr(), 0x224, &[]).bool() {
        return;
    }
    let mut scan_first = true;
    let mut scan_second = true;
    if e.vcall(target.addr(), 0x100, &[]).bool() {
        let object = e.call(SAVED_ACQUIRE_OBJECT, &args![target]).u32();
        if object != 0 {
            if e.vcall(object, 0x6b0, &[]).u32() != 0 {
                let mut node = e.vcall(object, 0x6b0, &[]).u32();
                while node != 0 && !e.call(NODE_DONE, &args![node]).bool() {
                    let slot = e.call(NODE_ITEM, &args![node]).u32();
                    let arrow = e.mem.u32(slot);
                    if arrow != 0 {
                        handle_arrow(e, target, arrow, arrow, by_slot);
                    }
                    node = e.call(NODE_NEXT, &args![node]).u32();
                }
                let list = e.vcall(object, 0x6b0, &[]).u32();
                e.call(LIST_REMOVE_ALL, &args![list]);
            }
            if e.call(OBJECT_KIND, &args![object]).u32() == 0 {
                scan_first = true;
                scan_second = false;
            } else if e.call(OBJECT_KIND, &args![object]).u32() == 1 {
                scan_first = false;
                scan_second = true;
            }
        }
    }
    if scan_first {
        let mut position = 0u32;
        while position < list_end(e, 0) {
            let (reference, arrow) = arrow_at(e, position);
            if arrow != 0 {
                handle_arrow(e, target, arrow, reference, by_slot);
            }
            position = position.wrapping_add(1);
        }
    }
    if scan_second {
        let list = e.call(PROCESS_LIST_OBJECT, &args![PROCESS_LISTS]).u32();
        let mut position = e.call(LIST_BEGIN, &args![list, 1u32]).u32();
        while position < list_end(e, 1) {
            let (reference, arrow) = arrow_at(e, position);
            if arrow != 0 {
                handle_arrow(e, target, arrow, reference, true);
            }
            position = position.wrapping_add(1);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x008c_ddd0,
        arrow_projectile_delete_attached_arrows(Ptr, bool)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const VTABLE: u32 = 0x0200_0000;
    const SLOT_FLAG_224: u32 = 0x0300_0224;
    const SLOT_FLAG_100: u32 = 0x0300_0100;
    const SLOT_LIST: u32 = 0x0300_06b0;
    const SLOT_C4: u32 = 0x0300_00c4;
    const SLOT_1CC: u32 = 0x0300_01cc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object with the shared vtable; +4 holds its slot 0x224 answer, +8
    /// its slot 0x100 answer, +0xC its slot 0x6B0 list, +0x10 its attached
    /// target, +0x14 its `00440d80` answer.
    fn object(e: &mut Engine, flag_224: bool, flag_100: bool) -> u32 {
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p, VTABLE);
        e.mem.set_u32(p + 4, flag_224 as u32);
        e.mem.set_u32(p + 8, flag_100 as u32);
        p
    }

    fn engine(kind: u32) -> (Engine, Log) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x6b4 / 4];
        slots[0x224 / 4] = SLOT_FLAG_224;
        slots[0x100 / 4] = SLOT_FLAG_100;
        slots[0x6b0 / 4] = SLOT_LIST;
        slots[0xc4 / 4] = SLOT_C4;
        slots[0x1cc / 4] = SLOT_1CC;
        e.put_vtable(VTABLE, &slots);
        e.register(SLOT_FLAG_224, |e, a| (e.mem.u32(a[0] + 4) as u8).into_ret());
        e.register(SLOT_FLAG_100, |e, a| (e.mem.u32(a[0] + 8) as u8).into_ret());
        e.register(SLOT_LIST, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ARROW_ATTACHED_TO, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(REFERENCE_TEST, |e, a| {
            (e.mem.u32(a[0] + 0x14) as u8).into_ret()
        });
        e.register_double(OBJECT_KIND, move |_, _| kind.into_ret());
        // The list's nodes are 8-byte cells: item, next.
        e.register(NODE_DONE, |_, _| false.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PROCESS_LIST_OBJECT, |_, a| (a[0] + 4).into_ret());
        // Lists 0 and 1: list 0 holds the references at 0x0400_0000 + 4*i
        // (count at 0x0400_0100), list 1 those at 0x0400_0200 (first index at
        // 0x0400_0300, end at 0x0400_0304).
        e.map(0x0400_0000, 0x1000);
        e.register(LIST_END, |e, a| {
            if a[1] == 0 {
                e.mem.u32(0x0400_0100).into_ret()
            } else {
                e.mem.u32(0x0400_0304).into_ret()
            }
        });
        e.register(LIST_BEGIN, |e, _| e.mem.u32(0x0400_0300).into_ret());
        e.register(LIST_ITEM, |e, a| {
            let list_one = e.mem.u32(0x0400_0308) != 0;
            let base = if list_one { 0x0400_0200 } else { 0x0400_0000 };
            e.mem.u32(base + 4 * a[1]).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let log: Log = Rc::default();
        for addr in [
            ARROW_DETACH,
            ARROW_REMOVE,
            SLOT_C4,
            SLOT_1CC,
            LIST_REMOVE_ALL,
        ] {
            let record = log.clone();
            e.register_double(addr, move |_, a| {
                record.borrow_mut().push((addr, a.to_vec()));
                Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        (e, log)
    }

    #[test]
    fn null_or_flagged_target_does_nothing() {
        let (mut e, log) = engine(2);
        e.call(0x008c_ddd0, &args![0u32, 1u32]);
        let target = object(&mut e, true, false);
        e.call(0x008c_ddd0, &args![target, 1u32]);
        assert!(log.borrow().is_empty());
        assert!(calls(&e, SLOT_FLAG_100).is_empty());
    }

    #[test]
    fn process_list_arrows_are_handled_and_the_list_emptied() {
        let (mut e, log) = engine(0);
        let target = object(&mut e, false, true);
        let process = object(&mut e, false, false);
        e.register_double(SAVED_ACQUIRE_OBJECT, move |_, _| process.into_ret());
        // Two arrows attached to the target (one whose test holds), one to
        // another reference, and an empty cell.
        let a1 = object(&mut e, false, false);
        let a2 = object(&mut e, false, false);
        let a3 = object(&mut e, false, false);
        e.mem.set_u32(a1 + 0x10, target);
        e.mem.set_u32(a2 + 0x10, target);
        e.mem.set_u32(a2 + 0x14, 1);
        e.mem.set_u32(a3 + 0x10, 0x9999);
        let cells: Vec<u32> = (0..4).map(|_| e.mem.alloc(8)).collect();
        for (i, item) in [a1, 0, a2, a3].into_iter().enumerate() {
            e.mem.set_u32(cells[i], item);
            e.mem.set_u32(cells[i] + 4, *cells.get(i + 1).unwrap_or(&0));
        }
        e.mem.set_u32(process + 0xc, cells[0]);
        // Kind 0: list 0 only, which is empty.
        e.call(0x008c_ddd0, &args![target, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (ARROW_DETACH, vec![a1]),
                (ARROW_REMOVE, vec![a1, 3]),
                (ARROW_DETACH, vec![a2]),
                (SLOT_1CC, vec![a2, 0, 1]),
                (LIST_REMOVE_ALL, vec![cells[0]]),
            ]
        );
        assert_eq!(calls(&e, LIST_END), vec![vec![PROCESS_LISTS + 4, 0]]);
        assert!(calls(&e, LIST_BEGIN).is_empty());
    }

    #[test]
    fn both_lists_are_scanned_without_an_object() {
        let (mut e, log) = engine(5);
        let target = object(&mut e, false, false);
        // List 0: a reference whose slot 0x224 is false, then an arrow.
        let plain = object(&mut e, false, false);
        let arrow = object(&mut e, true, false);
        e.mem.set_u32(arrow + 0x10, target);
        e.mem.set_u32(0x0400_0000, plain);
        e.mem.set_u32(0x0400_0004, arrow);
        e.mem.set_u32(0x0400_0100, 2);
        e.call(0x008c_ddd0, &args![target, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![(ARROW_DETACH, vec![arrow]), (ARROW_REMOVE, vec![arrow, 3])]
        );
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![arrow, 0, MOBILE_OBJECT_TYPE, ARROW_PROJECTILE_TYPE, 0]]
        );
        // List 1 starts at its first index and always removes through slot
        // 0xC4.
        log.borrow_mut().clear();
        e.mem.set_u32(0x0400_0100, 0);
        e.mem.set_u32(0x0400_0308, 1);
        e.mem.set_u32(0x0400_0300, 1);
        e.mem.set_u32(0x0400_0304, 2);
        e.mem.set_u32(0x0400_0204, arrow);
        e.call(0x008c_ddd0, &args![target, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![(ARROW_DETACH, vec![arrow]), (SLOT_C4, vec![arrow, 1])]
        );
    }

    #[test]
    fn object_kind_one_scans_only_list_one() {
        let (mut e, _log) = engine(1);
        let target = object(&mut e, false, true);
        let process = object(&mut e, false, false);
        e.register_double(SAVED_ACQUIRE_OBJECT, move |_, _| process.into_ret());
        e.call(0x008c_ddd0, &args![target, 1u32]);
        assert_eq!(calls(&e, LIST_BEGIN), vec![vec![PROCESS_LISTS + 4, 1]]);
        assert_eq!(calls(&e, LIST_END), vec![vec![PROCESS_LISTS + 4, 1]]);
        assert_eq!(calls(&e, OBJECT_KIND).len(), 2);
        assert!(calls(&e, LIST_REMOVE_ALL).is_empty());
    }
}
