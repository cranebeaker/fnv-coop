//! `fallout shared/havok/run_00620230` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00825c00` (unnamed in the engine map): `thiscall`; the dword at `this + 0x14`.
const NODE_FIELD_14: u32 = 0x0082_5c00;
/// `0057b700` (unnamed in the engine map, `tesobjectrefr.cpp`): `thiscall`, `RET 4`.
const NODE_NOTIFY: u32 = 0x0057_b700;
/// `006205e0` (unnamed in the engine map): `thiscall`, `RET 4`.
const LISTENER_DROP_NODE: u32 = 0x0062_05e0;

// Translated from 006206d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8` (a node and a target). With a non-null
/// target: when the node's field `+0x14` (`00825c00`) is non-zero, hands the target to
/// `0057b700` on the node; then, when that field (read again) is zero, hands the node to
/// `006205e0` on `this`.
pub fn fn_006206d0(e: &mut Engine, this: Ptr, node: Ptr, target: Ptr) {
    if target.is_null() {
        return;
    }
    if e.call(NODE_FIELD_14, &args![node]).u32() != 0 {
        e.call(NODE_NOTIFY, &args![node, target]);
    }
    if e.call(NODE_FIELD_14, &args![node]).u32() == 0 {
        e.call(LISTENER_DROP_NODE, &args![this, node]);
    }
}

/// `operator delete` (`00401030`), cdecl, one pointer word.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0061fc50` (`run_0061fba0.rs`): `thiscall`, plain `RET`; the destructor body of the entry
/// that `00620230` frees.
const ENTRY_DESTRUCT: u32 = 0x0061_fc50;
/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4`.
const LINK_NEXT: u32 = 0x0072_6070;
/// `006ecd40` (unnamed in the engine map): `thiscall`, `RET 4`; stores a link on a node.
const LINK_SET: u32 = 0x006e_cd40;
/// `0044ddc0` (unnamed in the engine map): `thiscall`, no stack words; a node's key.
const NODE_KEY: u32 = 0x0044_ddc0;
/// `GetAVObjectForCollidable` (engine map, `globalfunc.cpp`, `004b5820`): cdecl, one word.
const AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// `TESObjectREFR::FindReferenceFor3D` (engine map, `0056f930`): cdecl, one word.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `006815c0` (unnamed in the engine map): `thiscall`, no stack words.
const MEMBER_SETUP: u32 = 0x0068_15c0;
/// `00538f40` (unnamed in the engine map): `thiscall`, no stack words.
const MEMBER_SETUP_AFTER: u32 = 0x0053_8f40;
/// `004a3f10` (unnamed in the engine map): `thiscall`, `RET 4`; run on `this + 0x20` by
/// `006205c0` and on the local member of `00620340`.
const MEMBER_ASSIGN: u32 = 0x004a_3f10;
/// `004b59f0` (unnamed in the engine map): cdecl, one word; answers a pointer (or null).
const LOOKUP_BY_KEY: u32 = 0x004b_59f0;
/// `00560dc0` (unnamed in the engine map): `thiscall`, no stack words.
const LOOKUP_RESULT_VALUE: u32 = 0x0056_0dc0;
/// `004b5a80` (unnamed in the engine map): cdecl, one word; answers a pointer (or null).
const LOOKUP_RESULT_PARENT: u32 = 0x004b_5a80;
/// `004ae6a0` (unnamed in the engine map): `thiscall`, no stack words; answers a pointer.
const PARENT_OWNER: u32 = 0x004a_e6a0;
/// `0087ce80` (unnamed in the engine map): `thiscall`, `RET 4`.
const ENTRY_RECORD: u32 = 0x0087_ce80;
/// `0056d300` (unnamed in the engine map): `thiscall`, `RET 4`; takes the address of the local
/// member of `00620340`.
const ENTRY_FINISH: u32 = 0x0056_d300;
/// `0061fd70` (`bgszonetargetlistener.rs`): `thiscall`, `RET 0xC`.
const ZONE_ENTRY_ADD_TARGET_DERIVED: u32 = 0x0061_fd70;
/// Offset of the vtable slot `00620260` calls to create an entry.
const LISTENER_CREATE_SLOT: u32 = 4;

// Translated from 00620230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The scalar deleting destructor of an entry: runs the destructor
/// body (`0061fc50`) and, when bit 0 of `flags` is set, frees the object (`00401030`).
/// `thiscall`, `RET 4`; answers `this`.
pub fn fn_00620230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ENTRY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00620810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Starting at `this`, walks the chain
/// (`00726070`) until a node whose key (`0044ddc0`) equals `key`; answers that node, or null at
/// the end of the chain.
pub fn fn_00620810(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut node = this.addr();
    while node != 0 {
        if e.call(NODE_KEY, &args![node]).u32() == key {
            break;
        }
        node = e.call(LINK_NEXT, &args![node]).u32();
    }
    node
}

// Translated from 00620300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Looks `key` up from the node at `this + 4`
/// (`00620810`) and answers the node found minus 4, or null when there is none.
pub fn fn_00620300(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let found = fn_00620810(e, Ptr::new(this.addr().wrapping_add(4)), key);
    if found == 0 {
        0
    } else {
        found.wrapping_sub(4)
    }
}

// Translated from 00620260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. When the list head at `this + 4` is not
/// null, looks `second` up in it (`00620300`). When nothing is found, creates an entry through
/// the object's vtable slot `+4` (`first`, `second`), links it in front of the previous head's
/// `+4` (`006ecd40` on `entry + 4`) and makes it the head. Answers the entry found or created.
pub fn fn_00620260(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let head = e.mem.u32(this.addr().wrapping_add(4));
    let mut found = if head == 0 {
        0
    } else {
        fn_00620300(e, Ptr::new(head), second)
    };
    if found == 0 {
        found = e
            .vcall(this.addr(), LISTENER_CREATE_SLOT, &args![first, second])
            .u32();
        let old_head = e.mem.u32(this.addr().wrapping_add(4));
        let link = if old_head == 0 {
            0
        } else {
            old_head.wrapping_add(4)
        };
        e.call(LINK_SET, &args![found.wrapping_add(4), link]);
        e.mem.set_u32(this.addr().wrapping_add(4), found);
    }
    found
}

// Translated from 006205a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: the low five bits of the dword at `this + 0x10`.
pub fn fn_006205a0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x10)) & 0x1f
}

// Translated from 006205c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: hands the word to `004a3f10` on
/// `this + 0x20`.
pub fn fn_006205c0(e: &mut Engine, this: Ptr, value: u32) {
    e.call(MEMBER_ASSIGN, &args![this.addr().wrapping_add(0x20), value]);
}

// Translated from 006207a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: takes `this` out of the chain that starts at
/// `start`. When `this` is `start`, answers the link after it (`00726070`). Otherwise walks the
/// chain from `start`, and at the node whose link is `this` stores the link after `this`
/// (`006ecd40`) on it. Answers `start` (the new head in the first case).
pub fn fn_006207a0(e: &mut Engine, this: Ptr, start: u32) -> u32 {
    let mut head = start;
    if this.addr() == head {
        head = e.call(LINK_NEXT, &args![this]).u32();
    } else {
        let mut node = head;
        while node != 0 {
            let next = e.call(LINK_NEXT, &args![node]).u32();
            if this.addr() == next {
                let after = e.call(LINK_NEXT, &args![next]).u32();
                e.call(LINK_SET, &args![node, after]);
                break;
            }
            node = next;
        }
    }
    head
}

// Translated from 006205e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. With a non-null `entry`: takes the link at
/// `entry + 4` out of the chain that starts at the head's `+4` (`006207a0`), stores the result
/// minus 4 (or null) as the new head at `this + 4`, and destroys the entry (`00620230`, flag
/// 1).
pub fn fn_006205e0(e: &mut Engine, this: Ptr, entry: u32) {
    if entry == 0 {
        return;
    }
    let head = e.mem.u32(this.addr().wrapping_add(4));
    let start = if head == 0 { 0 } else { head.wrapping_add(4) };
    let rest = fn_006207a0(e, Ptr::new(entry.wrapping_add(4)), start);
    let new_head = if rest == 0 { 0 } else { rest.wrapping_sub(4) };
    e.mem.set_u32(this.addr().wrapping_add(4), new_head);
    // The exe tests the entry again before destroying it; it is not null here.
    if entry != 0 {
        fn_00620230(e, Ptr::new(entry), 1);
    }
}

// Translated from 00620680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. When the list head at `this + 4` is not
/// null and `00620300` finds the entry for `key`, hands it and `target` to `006206d0` and answers
/// it; otherwise answers null.
pub fn fn_00620680(e: &mut Engine, this: Ptr, key: u32, target: u32) -> u32 {
    let head = e.mem.u32(this.addr().wrapping_add(4));
    if head != 0 {
        let found = fn_00620300(e, Ptr::new(head), key);
        if found != 0 {
            fn_006206d0(e, this, Ptr::new(found), Ptr::new(target));
            return found;
        }
    }
    0
}

// Translated from 00620720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The constructor of the node the entries embed: stores `first` at
/// `+0xC` and `second` at `+8` and clears `+0` and `+4`. `thiscall`, `RET 8`; answers `this`.
pub fn fn_00620720(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0xc), first);
    e.mem.set_u32(base.wrapping_add(8), second);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base.wrapping_add(4), 0);
    this
}

// Translated from 00620760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`: sets (`set` true) or clears (`set` false) the
/// bits of `mask` in the dword at `this`.
pub fn fn_00620760(e: &mut Engine, this: Ptr, mask: u32, set: bool) {
    let flags = e.mem.u32(this.addr());
    if set {
        e.mem.set_u32(this.addr(), flags | mask);
    } else {
        e.mem.set_u32(this.addr(), !mask & flags);
    }
}

// Translated from 00620340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00620340 - /GS stack cookie check (00ec408c) not translated
/// Unnamed in the engine map. `thiscall`, `RET 0x14`, with the stack realigned to 16 bytes.
/// `pair` points at a record holding two collidables, at `+0x20` and `+0x48`, and a word at
/// `+0x4c`. Each collidable is turned into its reference (`004b5820` then `0056f930`) and
/// `ref_a` and `ref_b` are matched against them: the matching collidable is the one passed on
/// for it (null when neither matches). The entry for `ref_a` and its collidable comes from
/// `00620260` on `this`; when there is one, an entry is made for `ref_b` (through
/// `0061fd70` on it when `flag` is set, else through its vtable slot 0, with `ref_b`, its
/// collidable and `extra`). When that gives an entry, the record's `006815c0` result is handed
/// to `006205c0`, the key of the first entry (`0044ddc0` on `+4`) is looked up (`004b59f0`) and,
/// with the local member of the stack (a word `0x24` just before it), a value is worked out: from
/// the lookup's parent through `004ae6a0` and two virtual calls (slots `0xCC` and `0xE0`, the
/// latter given the record's `+0x4c` word unless it is `-1`) or the owner's low five bits
/// (`006205a0`); the value goes to `0087ce80` on the made entry, which is finished with the
/// local member (`0056d300`). Answers the entry made for `ref_a`.
pub fn fn_00620340(
    e: &mut Engine,
    this: Ptr,
    pair: Ptr,
    ref_a: u32,
    ref_b: u32,
    extra: u32,
    flag: bool,
) -> u32 {
    let pair_base = pair.addr();
    let first_collidable = e.mem.u32(pair_base.wrapping_add(0x20));
    let second_collidable = e.mem.u32(pair_base.wrapping_add(0x48));
    let first_object = e
        .call(AV_OBJECT_FOR_COLLIDABLE, &args![first_collidable])
        .u32();
    let second_object = e
        .call(AV_OBJECT_FOR_COLLIDABLE, &args![second_collidable])
        .u32();
    let first_reference = e.call(FIND_REFERENCE_FOR_3D, &args![first_object]).u32();
    let second_reference = e.call(FIND_REFERENCE_FOR_3D, &args![second_object]).u32();

    let pick = |wanted: u32| {
        if first_reference == wanted {
            first_collidable
        } else if second_reference == wanted {
            second_collidable
        } else {
            0
        }
    };
    let collidable_a = pick(ref_a);
    let collidable_b = pick(ref_b);

    let made = fn_00620260(e, this, ref_a, collidable_a);
    if made != 0 {
        let created = if flag {
            e.call(
                ZONE_ENTRY_ADD_TARGET_DERIVED,
                &args![made, ref_b, collidable_b, extra],
            )
            .u32()
        } else {
            e.vcall(made, 0, &args![ref_b, collidable_b, extra]).u32()
        };
        if created != 0 {
            let setup = e.call(MEMBER_SETUP, &args![pair]).u32();
            fn_006205c0(e, Ptr::new(created), setup);
            let key = e.call(NODE_KEY, &args![made.wrapping_add(4)]).u32();
            let found = e.call(LOOKUP_BY_KEY, &args![key]).u32();
            // The word `0x24` sits just before the 0x14-byte local member: `EBP-0x54` and `EBP-0x50`
            // of the exe's frame (`EBP` on a 16-byte boundary, `SUB ESP,0x70`).
            e.with_stack(0x70 + 0x10, |e, raw| {
                let block = ((raw.addr() + 0x70 + 0xf) & !0xf) - 0x54;
                let local = block.wrapping_add(4);
                e.call(MEMBER_SETUP, &args![local]);
                e.mem.set_u32(block, 0x24);
                if found != 0 {
                    let value = e.call(LOOKUP_RESULT_VALUE, &args![found]).u32();
                    e.call(MEMBER_ASSIGN, &args![local, value]);
                    let parent = e.call(LOOKUP_RESULT_PARENT, &args![found]).u32();
                    let owner = if parent == 0 {
                        0
                    } else {
                        e.call(PARENT_OWNER, &args![parent]).u32()
                    };
                    let source = if owner == 0 {
                        0
                    } else {
                        e.vcall(owner, 0xcc, &args![]).u32()
                    };
                    let record_word = e.mem.u32(pair_base.wrapping_add(0x4c));
                    if source != 0 && record_word != u32::MAX {
                        let value = e.vcall(source, 0xe0, &args![record_word]).u32();
                        e.call(ENTRY_RECORD, &args![created, value]);
                    } else if owner != 0 {
                        let low_bits = fn_006205a0(e, Ptr::new(owner));
                        e.call(ENTRY_RECORD, &args![created, low_bits]);
                    }
                } else {
                    e.call(MEMBER_SETUP_AFTER, &args![local]);
                }
                e.call(ENTRY_FINISH, &args![created, local]);
            });
        }
    }
    made
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_06d0, fn_006206d0(Ptr, Ptr, Ptr)),
        entry!(0x0062_0230, fn_00620230(Ptr, u32) -> Ptr),
        entry!(0x0062_0260, fn_00620260(Ptr, u32, u32) -> u32),
        entry!(0x0062_0300, fn_00620300(Ptr, u32) -> u32),
        entry!(0x0062_0340, fn_00620340(Ptr, Ptr, u32, u32, u32, bool) -> u32),
        entry!(0x0062_05a0, fn_006205a0(Ptr) -> u32),
        entry!(0x0062_05c0, fn_006205c0(Ptr, u32)),
        entry!(0x0062_05e0, fn_006205e0(Ptr, u32)),
        entry!(0x0062_0680, fn_00620680(Ptr, u32, u32) -> u32),
        entry!(0x0062_0720, fn_00620720(Ptr, u32, u32) -> Ptr),
        entry!(0x0062_0760, fn_00620760(Ptr, u32, bool)),
        entry!(0x0062_07a0, fn_006207a0(Ptr, u32) -> u32),
        entry!(0x0062_0810, fn_00620810(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NODE_FIELD_14, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(NODE_NOTIFY, |_, _| Ret::default());
        e.register(LISTENER_DROP_NODE, |_, _| Ret::default());
        e
    }

    #[test]
    fn at_006206d0_notifies_a_node_that_has_the_field_and_keeps_it() {
        let mut e = rig();
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(node + 0x14, 5);
        e.call(0x0062_06d0, &args![0x4000u32, node, 0x9000u32]);
        assert_eq!(calls(&e, NODE_NOTIFY), vec![vec![node, 0x9000]]);
        assert!(calls(&e, LISTENER_DROP_NODE).is_empty());
        assert_eq!(calls(&e, NODE_FIELD_14).len(), 2);
    }

    #[test]
    fn at_006206d0_drops_a_node_without_the_field() {
        let mut e = rig();
        let node = e.mem.alloc(0x20);
        e.call(0x0062_06d0, &args![0x4000u32, node, 0x9000u32]);
        assert!(calls(&e, NODE_NOTIFY).is_empty());
        assert_eq!(calls(&e, LISTENER_DROP_NODE), vec![vec![0x4000, node]]);
    }

    #[test]
    fn at_006206d0_does_nothing_without_a_target() {
        let mut e = rig();
        let node = e.mem.alloc(0x20);
        e.call(0x0062_06d0, &args![0x4000u32, node, 0u32]);
        assert!(calls(&e, NODE_FIELD_14).is_empty());
        assert!(calls(&e, LISTENER_DROP_NODE).is_empty());
    }
}

#[cfg(test)]
mod havok_glue_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    /// A chain of nodes whose link is the word at `+4` (what `00726070` answers) and whose key
    /// is the word at `+0`.
    fn chain_doubles(e: &mut Engine) {
        e.register(LINK_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LINK_SET, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
    }

    fn node(e: &mut Engine, key: u32, next: u32) -> u32 {
        let n = e.mem.alloc(0x10);
        e.mem.set_u32(n, key);
        e.mem.set_u32(n + 4, next);
        n
    }

    #[test]
    fn at_00620230_destroys_and_frees_only_with_bit_0() {
        let mut e = quiet();
        e.register(ENTRY_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_0230, &args![0x500u32, 1u32]).u32(), 0x500);
        assert_eq!(calls(&e, ENTRY_DESTRUCT), vec![vec![0x500]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x500]]);
        assert_eq!(e.call(0x0062_0230, &args![0x600u32, 2u32]).u32(), 0x600);
        assert_eq!(calls(&e, ENTRY_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn at_00620810_walks_to_the_node_with_the_key_or_to_the_end() {
        let mut e = quiet();
        chain_doubles(&mut e);
        let c = node(&mut e, 30, 0);
        let b = node(&mut e, 20, c);
        let a = node(&mut e, 10, b);
        assert_eq!(e.call(0x0062_0810, &args![a, 20u32]).u32(), b);
        assert_eq!(e.call(0x0062_0810, &args![a, 10u32]).u32(), a);
        assert_eq!(e.call(0x0062_0810, &args![a, 99u32]).u32(), 0);
        assert_eq!(e.call(0x0062_0810, &args![0u32, 10u32]).u32(), 0);
    }

    #[test]
    fn at_00620300_searches_from_plus_4_and_steps_back() {
        let mut e = quiet();
        chain_doubles(&mut e);
        let b = node(&mut e, 20, 0);
        let a = node(&mut e, 10, b);
        // The holder's `+4` is the chain: the holder sits 4 bytes before the node `a`.
        assert_eq!(e.call(0x0062_0300, &args![a - 4, 20u32]).u32(), b - 4);
        assert_eq!(e.call(0x0062_0300, &args![a - 4, 99u32]).u32(), 0);
    }

    #[test]
    fn at_00620260_answers_a_found_entry_without_creating() {
        let mut e = quiet();
        chain_doubles(&mut e);
        e.register(LINK_SET, |_, _| Ret::default());
        let entry = node(&mut e, 20, 0);
        let this = e.mem.alloc(0x10);
        // `this + 4` holds a pointer to the node `entry - 4`'s successor chain start.
        e.mem.set_u32(this + 4, entry - 4);
        assert_eq!(
            e.call(0x0062_0260, &args![this, 1u32, 20u32]).u32(),
            entry - 4
        );
        assert!(calls(&e, LINK_SET).is_empty());
        assert_eq!(e.mem.u32(this + 4), entry - 4);
    }

    #[test]
    fn at_00620260_creates_through_the_vtable_when_nothing_is_found() {
        let mut e = quiet();
        chain_doubles(&mut e);
        let created = e.mem.alloc(0x10);
        e.register_double(0x0300_0004, move |_, _| created.into_ret());
        e.put_vtable(0x0300_0000, &[0, 0x0300_0004]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0300_0000);
        // Empty list: no lookup, no previous head.
        assert_eq!(e.call(0x0062_0260, &args![this, 7u32, 8u32]).u32(), created);
        assert_eq!(calls(&e, 0x0300_0004), vec![vec![this, 7, 8]]);
        assert_eq!(calls(&e, LINK_SET), vec![vec![created + 4, 0]]);
        assert_eq!(e.mem.u32(this + 4), created);
        // A head whose chain lacks the key: the new entry links to the old head's `+4`.
        let other = node(&mut e, 5, 0);
        let old = other - 4;
        e.mem.set_u32(this + 4, old);
        e.call(0x0062_0260, &args![this, 7u32, 8u32]);
        assert_eq!(calls(&e, LINK_SET)[1], vec![created + 4, old + 4]);
        assert_eq!(e.mem.u32(this + 4), created);
    }

    #[test]
    fn at_006205a0_keeps_the_low_five_bits() {
        let mut e = quiet();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0xffff_ffe5);
        assert_eq!(e.call(0x0062_05a0, &args![this]).u32(), 5);
    }

    #[test]
    fn at_006205c0_hands_the_word_to_the_member_at_plus_0x20() {
        let mut e = quiet();
        e.register(MEMBER_ASSIGN, |_, _| Ret::default());
        e.call(0x0062_05c0, &args![0x1000u32, 0x55u32]);
        assert_eq!(calls(&e, MEMBER_ASSIGN), vec![vec![0x1020, 0x55]]);
    }

    #[test]
    fn at_006207a0_removes_the_head_or_a_later_node() {
        let mut e = quiet();
        chain_doubles(&mut e);
        // Removing the head answers the link after it and links nothing.
        let c = node(&mut e, 3, 0);
        let b = node(&mut e, 2, c);
        let a = node(&mut e, 1, b);
        assert_eq!(e.call(0x0062_07a0, &args![a, a]).u32(), b);
        assert!(calls(&e, LINK_SET).is_empty());
        // Removing the last node links its predecessor to what follows it.
        assert_eq!(e.call(0x0062_07a0, &args![c, a]).u32(), a);
        assert_eq!(calls(&e, LINK_SET), vec![vec![b, 0]]);
        assert_eq!(e.mem.u32(b + 4), 0);
        // A node that is not in the chain changes nothing.
        let stray = node(&mut e, 9, 0);
        assert_eq!(e.call(0x0062_07a0, &args![stray, a]).u32(), a);
        assert_eq!(calls(&e, LINK_SET).len(), 1);
        // An empty start answers null.
        assert_eq!(e.call(0x0062_07a0, &args![stray, 0u32]).u32(), 0);
    }

    #[test]
    fn at_006205e0_unlinks_the_entry_and_destroys_it() {
        let mut e = quiet();
        chain_doubles(&mut e);
        e.register(ENTRY_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        // Entries are chain nodes at `+4`: entry `x` has its node at `x + 4`, whose link is
        // the word at `x + 8`.
        let second = e.mem.alloc(0x20);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(second + 8, 0);
        e.mem.set_u32(first + 8, second + 4);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, first);
        // Dropping the head entry: the head becomes the entry after it.
        e.call(0x0062_05e0, &args![this, first]);
        assert_eq!(e.mem.u32(this + 4), second);
        assert_eq!(calls(&e, ENTRY_DESTRUCT), vec![vec![first]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![first]]);
        // Dropping the last entry empties the head.
        e.call(0x0062_05e0, &args![this, second]);
        assert_eq!(e.mem.u32(this + 4), 0);
        // A null entry does nothing.
        e.call(0x0062_05e0, &args![this, 0u32]);
        assert_eq!(calls(&e, ENTRY_DESTRUCT).len(), 2);
    }

    #[test]
    fn at_00620680_hands_the_found_entry_and_target_to_the_node_handler() {
        let mut e = quiet();
        chain_doubles(&mut e);
        e.register(NODE_FIELD_14, |_, _| Ret::default());
        e.register(NODE_NOTIFY, |_, _| Ret::default());
        e.register(LISTENER_DROP_NODE, |_, _| Ret::default());
        let entry = node(&mut e, 20, 0);
        let this = e.mem.alloc(0x10);
        // No head: nothing is looked up.
        assert_eq!(e.call(0x0062_0680, &args![this, 20u32, 0x9000u32]).u32(), 0);
        assert!(calls(&e, NODE_FIELD_14).is_empty());
        // The head's chain holds the key: the entry is handed on and answered.
        e.mem.set_u32(this + 4, entry - 4);
        let found = e.call(0x0062_0680, &args![this, 20u32, 0x9000u32]).u32();
        assert_eq!(found, entry - 4);
        assert_eq!(calls(&e, NODE_FIELD_14), vec![vec![found], vec![found]]);
        // A key that is not there answers null.
        assert_eq!(e.call(0x0062_0680, &args![this, 77u32, 0x9000u32]).u32(), 0);
    }

    #[test]
    fn at_00620720_fills_the_node() {
        let mut e = quiet();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        assert_eq!(
            e.call(0x0062_0720, &args![this, 0x11u32, 0x22u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 0xc), 0x11);
        assert_eq!(e.mem.u32(this + 8), 0x22);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn at_00620760_sets_or_clears_the_mask() {
        let mut e = quiet();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0f);
        e.call(0x0062_0760, &args![this, 0xf0u32, true]);
        assert_eq!(e.mem.u32(this), 0xff);
        e.call(0x0062_0760, &args![this, 0x3cu32, false]);
        assert_eq!(e.mem.u32(this), 0xc3);
    }
}

/// `00620340`, with every callee doubled: the record at `pair` holds the collidables `0x1000`
/// (at `+0x20`) and `0x2000` (at `+0x48`) and the word `0x77` at `+0x4c`; references `0xa1` and
/// `0xa2` own them. The listener has no entries, so `00620260` creates one through the vtable.
#[cfg(test)]
mod pair_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CREATE_ENTRY: u32 = 0x0300_0010;
    const ENTRY_SLOT_0: u32 = 0x0300_0020;
    const OWNER_SLOT_CC: u32 = 0x0300_0030;
    const SOURCE_SLOT_E0: u32 = 0x0300_0040;

    struct Rig {
        e: Engine,
        this: u32,
        pair: u32,
        made: u32,
        created: u32,
        owner: u32,
        source: u32,
        /// What `0056d300` saw: the word just before the local member it was given.
        finished: Rc<RefCell<Vec<u32>>>,
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn table(e: &mut Engine, base: u32, slot_offset: u32, target: u32) {
        let mut slots = vec![0u32; (slot_offset / 4 + 1) as usize];
        slots[(slot_offset / 4) as usize] = target;
        e.put_vtable(base, &slots);
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(AV_OBJECT_FOR_COLLIDABLE, |_, a| (a[0] + 1).into_ret());
        e.register(FIND_REFERENCE_FOR_3D, |_, a| match a[0] {
            0x1001 => 0xa1u32.into_ret(),
            0x2001 => 0xa2u32.into_ret(),
            _ => Ret::default(),
        });
        for addr in [
            MEMBER_ASSIGN,
            ENTRY_RECORD,
            MEMBER_SETUP_AFTER,
            LOOKUP_BY_KEY,
            LOOKUP_RESULT_PARENT,
            PARENT_OWNER,
            ZONE_ENTRY_ADD_TARGET_DERIVED,
            LINK_SET,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMBER_SETUP, |_, a| (a[0] + 1).into_ret());
        e.register(NODE_KEY, |_, a| (a[0] + 2).into_ret());
        e.register(LOOKUP_RESULT_VALUE, |_, _| 0x66u32.into_ret());
        let finished = Rc::new(RefCell::new(vec![]));
        let seen = Rc::clone(&finished);
        e.register_double(ENTRY_FINISH, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1] - 4));
            Ret::default()
        });
        let pair = e.mem.alloc(0x60);
        e.mem.set_u32(pair + 0x20, 0x1000);
        e.mem.set_u32(pair + 0x48, 0x2000);
        e.mem.set_u32(pair + 0x4c, 0x77);
        let this = e.mem.alloc(0x10);
        let made = e.mem.alloc(0x20);
        let created = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x20);
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0x10, 0x1234_5679);
        // The listener creates `made` (vtable slot 4), `made` creates `created` (slot 0), the
        // owner answers `source` (slot 0xcc) and the source a value (slot 0xe0).
        table(&mut e, 0x0300_0100, 4, CREATE_ENTRY);
        table(&mut e, 0x0300_0200, 0, ENTRY_SLOT_0);
        table(&mut e, 0x0300_0300, 0xcc, OWNER_SLOT_CC);
        table(&mut e, 0x0300_0400, 0xe0, SOURCE_SLOT_E0);
        e.mem.set_u32(this, 0x0300_0100);
        e.mem.set_u32(made, 0x0300_0200);
        e.mem.set_u32(owner, 0x0300_0300);
        e.mem.set_u32(source, 0x0300_0400);
        e.register_double(CREATE_ENTRY, move |_, _| made.into_ret());
        e.register_double(ENTRY_SLOT_0, move |_, _| created.into_ret());
        e.register_double(OWNER_SLOT_CC, move |_, _| source.into_ret());
        e.register(SOURCE_SLOT_E0, |_, a| (a[1] + 0x1000).into_ret());
        Rig {
            e,
            this,
            pair,
            made,
            created,
            owner,
            source,
            finished,
        }
    }

    fn run(r: &mut Rig, ref_a: u32, ref_b: u32, flag: bool) -> u32 {
        r.e.call(
            0x0062_0340,
            &args![r.this, r.pair, ref_a, ref_b, 0x55u32, flag],
        )
        .u32()
    }

    #[test]
    fn at_00620340_matches_the_references_to_their_collidables() {
        let mut r = rig();
        r.e.register(CREATE_ENTRY, |_, _| Ret::default());
        assert_eq!(run(&mut r, 0xa2, 0xa1, false), 0);
        assert_eq!(
            calls(&r.e, AV_OBJECT_FOR_COLLIDABLE),
            vec![vec![0x1000], vec![0x2000]]
        );
        assert_eq!(
            calls(&r.e, FIND_REFERENCE_FOR_3D),
            vec![vec![0x1001], vec![0x2001]]
        );
        // `00620260` asked the listener to create the entry for `ref_a` with its collidable.
        assert_eq!(calls(&r.e, CREATE_ENTRY), vec![vec![r.this, 0xa2, 0x2000]]);
        // No entry was made: the link is stored with a null target and nothing else runs.
        assert_eq!(calls(&r.e, LINK_SET), vec![vec![4, 0]]);
        assert!(calls(&r.e, ENTRY_SLOT_0).is_empty());
        assert_eq!(r.e.mem.u32(r.this + 4), 0);
        // A reference that owns neither collidable gets a null collidable.
        let mut r = rig();
        assert_eq!(run(&mut r, 0xee, 0xa1, false), r.made);
        assert_eq!(calls(&r.e, CREATE_ENTRY), vec![vec![r.this, 0xee, 0]]);
        assert_eq!(
            calls(&r.e, ENTRY_SLOT_0),
            vec![vec![r.made, 0xa1, 0x1000, 0x55]]
        );
    }

    #[test]
    fn at_00620340_uses_the_derived_add_target_when_the_flag_is_set() {
        let mut r = rig();
        assert_eq!(run(&mut r, 0xa1, 0xee, true), r.made);
        assert_eq!(
            calls(&r.e, ZONE_ENTRY_ADD_TARGET_DERIVED),
            vec![vec![r.made, 0xee, 0, 0x55]]
        );
        assert!(calls(&r.e, ENTRY_SLOT_0).is_empty());
        // The derived call answered null: the made entry is returned, nothing finished.
        assert!(calls(&r.e, ENTRY_FINISH).is_empty());
    }

    #[test]
    fn at_00620340_finishes_with_a_fresh_local_when_the_lookup_fails() {
        let mut r = rig();
        assert_eq!(run(&mut r, 0xa1, 0xa2, false), r.made);
        let setup = calls(&r.e, MEMBER_SETUP);
        assert_eq!(setup[0], vec![r.pair]);
        let local = setup[1][0];
        assert_eq!(
            calls(&r.e, MEMBER_ASSIGN),
            vec![vec![r.created + 0x20, r.pair + 1]]
        );
        assert_eq!(calls(&r.e, NODE_KEY), vec![vec![r.made + 4]]);
        assert_eq!(calls(&r.e, LOOKUP_BY_KEY), vec![vec![r.made + 6]]);
        assert_eq!(calls(&r.e, MEMBER_SETUP_AFTER), vec![vec![local]]);
        assert_eq!(calls(&r.e, ENTRY_FINISH), vec![vec![r.created, local]]);
        assert_eq!(*r.finished.borrow(), vec![0x24]);
        assert!(calls(&r.e, LOOKUP_RESULT_VALUE).is_empty());
        assert!(calls(&r.e, ENTRY_RECORD).is_empty());
    }

    #[test]
    fn at_00620340_records_the_value_from_the_source_when_the_word_is_set() {
        let mut r = rig();
        let owner = r.owner;
        r.e.register(LOOKUP_BY_KEY, |_, _| 0x9100u32.into_ret());
        r.e.register(LOOKUP_RESULT_PARENT, |_, _| 0x9200u32.into_ret());
        r.e.register_double(PARENT_OWNER, move |_, _| owner.into_ret());
        assert_eq!(run(&mut r, 0xa1, 0xa2, false), r.made);
        let local = calls(&r.e, MEMBER_SETUP)[1][0];
        assert_eq!(calls(&r.e, LOOKUP_RESULT_VALUE), vec![vec![0x9100]]);
        assert_eq!(calls(&r.e, MEMBER_ASSIGN)[1], vec![local, 0x66]);
        assert_eq!(calls(&r.e, LOOKUP_RESULT_PARENT), vec![vec![0x9100]]);
        assert_eq!(calls(&r.e, PARENT_OWNER), vec![vec![0x9200]]);
        assert_eq!(calls(&r.e, OWNER_SLOT_CC), vec![vec![r.owner]]);
        assert_eq!(calls(&r.e, SOURCE_SLOT_E0), vec![vec![r.source, 0x77]]);
        assert_eq!(calls(&r.e, ENTRY_RECORD), vec![vec![r.created, 0x1077]]);
        assert!(calls(&r.e, MEMBER_SETUP_AFTER).is_empty());
        assert_eq!(calls(&r.e, ENTRY_FINISH), vec![vec![r.created, local]]);
    }

    #[test]
    fn at_00620340_records_the_owners_low_bits_without_a_source_or_word() {
        // The record's word is -1: the owner's low five bits are used even with a source.
        let mut r = rig();
        let owner = r.owner;
        r.e.register(LOOKUP_BY_KEY, |_, _| 0x9100u32.into_ret());
        r.e.register(LOOKUP_RESULT_PARENT, |_, _| 0x9200u32.into_ret());
        r.e.register_double(PARENT_OWNER, move |_, _| owner.into_ret());
        r.e.mem.set_u32(r.pair + 0x4c, u32::MAX);
        run(&mut r, 0xa1, 0xa2, false);
        assert_eq!(calls(&r.e, ENTRY_RECORD), vec![vec![r.created, 0x19]]);
        assert!(calls(&r.e, SOURCE_SLOT_E0).is_empty());
        // The owner has no source (slot 0xcc answers null).
        let mut r = rig();
        let owner = r.owner;
        r.e.register(LOOKUP_BY_KEY, |_, _| 0x9100u32.into_ret());
        r.e.register(LOOKUP_RESULT_PARENT, |_, _| 0x9200u32.into_ret());
        r.e.register_double(PARENT_OWNER, move |_, _| owner.into_ret());
        r.e.register(OWNER_SLOT_CC, |_, _| Ret::default());
        run(&mut r, 0xa1, 0xa2, false);
        assert_eq!(calls(&r.e, ENTRY_RECORD), vec![vec![r.created, 0x19]]);
    }

    #[test]
    fn at_00620340_records_nothing_without_a_parent_or_owner() {
        let mut r = rig();
        r.e.register(LOOKUP_BY_KEY, |_, _| 0x9100u32.into_ret());
        // The parent is null: the owner is not asked for.
        run(&mut r, 0xa1, 0xa2, false);
        assert!(calls(&r.e, PARENT_OWNER).is_empty());
        assert!(calls(&r.e, ENTRY_RECORD).is_empty());
        assert_eq!(calls(&r.e, ENTRY_FINISH).len(), 1);
        // A parent with no owner records nothing either.
        let mut r = rig();
        r.e.register(LOOKUP_BY_KEY, |_, _| 0x9100u32.into_ret());
        r.e.register(LOOKUP_RESULT_PARENT, |_, _| 0x9200u32.into_ret());
        run(&mut r, 0xa1, 0xa2, false);
        assert_eq!(calls(&r.e, PARENT_OWNER), vec![vec![0x9200]]);
        assert!(calls(&r.e, ENTRY_RECORD).is_empty());
    }

    #[test]
    fn at_00620340_keeps_the_local_member_below_an_aligned_ebp() {
        let mut r = rig();
        let raw = r.e.next_stack_local(0x70 + 0x10);
        run(&mut r, 0xa1, 0xa2, false);
        // The word 0x24 at EBP-0x54, the member at EBP-0x50.
        let ebp = (raw + 0x70 + 0xf) & !0xf;
        assert_eq!(calls(&r.e, ENTRY_FINISH), vec![vec![r.created, ebp - 0x50]]);
        assert_eq!(r.e.mem.u32(ebp - 0x54), 0x24);
    }
}
