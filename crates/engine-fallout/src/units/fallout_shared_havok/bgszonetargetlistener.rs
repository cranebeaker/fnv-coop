//! `fallout shared/havok/bgszonetargetlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00620110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104d9e0` (RTTI `.?AVBGSZoneTargetListener@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006201c0`. One word, plain `RET`, returns nothing.
pub fn fn_00620110(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_d9e0);
    e.call(0x0062_01c0, &args![this]);
}

/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4` (a list's first
/// node).
const LIST_FIRST_NODE: u32 = 0x0072_6070;
/// `00620180` (`run_00620180.rs`): `thiscall`; the node after a node of the zone target list.
const ZONE_TARGET_NEXT: u32 = 0x0062_0180;
/// `006206d0` (`run_00620230.rs`): `thiscall`, `RET 8`; (node, target).
const ZONE_TARGET_NODE_REMOVE: u32 = 0x0062_06d0;
/// `006207a0` (unnamed in the engine map): `thiscall`, `RET 4`; called on the member at the
/// target's `+0x10` with one pointer, it answers a pointer (what it does is unconfirmed).
const ZONE_TARGET_LIST_FIND: u32 = 0x0062_07a0;
/// `0087e9b0` (unnamed in the engine map, `actor.cpp`): `cdecl`, one stack word.
const ZONE_TARGET_RELEASE: u32 = 0x0087_e9b0;

// Translated from 00620130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSZoneTargetListener::RemoveTarget` (Xbox PDB): `thiscall`, `RET 4`. With a non-null
/// target, walks the node chain that starts at the listener's `+4` list (`00726070`) and hands
/// every node, with the target, to `006206d0`; the next node is fetched (`00620180`) before the
/// node is handed over. The decompiler's `UMSFreeVirtualProcessorRoot::SchedulerProxy` is
/// `00620180`'s library-match name, which its body is not.
pub fn bgs_zone_target_listener_remove_target(e: &mut Engine, this: Ptr, target: Ptr) {
    if target.is_null() {
        return;
    }
    let mut node = e.call(LIST_FIRST_NODE, &args![this]).u32();
    while node != 0 {
        let next = e.call(ZONE_TARGET_NEXT, &args![node]).u32();
        e.call(ZONE_TARGET_NODE_REMOVE, &args![this, node, target]);
        node = next;
    }
}

// Translated from 0061ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. With a non-null target: asks the target's
/// member at `+0x10` (`006207a0`) about the pointer `*(this + 0x14) + 0x10` (null when the
/// listener has none), stores the answer minus `0x10` (or null) back at `this + 0x14`
/// and hands the target to `0087e9b0`.
pub fn fn_0061ff30(e: &mut Engine, this: Ptr, target: Ptr) {
    if target.is_null() {
        return;
    }
    let current = e.mem.u32(this.addr().wrapping_add(0x14));
    let key = if current == 0 {
        0
    } else {
        current.wrapping_add(0x10)
    };
    let found = e
        .call(
            ZONE_TARGET_LIST_FIND,
            &args![target.addr().wrapping_add(0x10), key],
        )
        .u32();
    let updated = if found == 0 {
        0
    } else {
        found.wrapping_sub(0x10)
    };
    e.mem.set_u32(this.addr().wrapping_add(0x14), updated);
    e.call(ZONE_TARGET_RELEASE, &args![target]);
}

/// `operator new` (`00401000`), cdecl, one size word.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`), cdecl, one pointer word.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Block allocator (`00553300`), cdecl, one size word.
const ALLOCATE_BLOCK: u32 = 0x0055_3300;
/// `0057b730` (unnamed in the engine map): `thiscall`, `RET 4`; looks a key up in the zone
/// entry list that starts at `this + 0x14` and answers the entry found (or null).
const ZONE_ENTRY_FIND: u32 = 0x0057_b730;
/// `0061fba0` (`run_0061fba0.rs`): `thiscall`, `RET 8`; constructs a 0x50-byte zone entry.
const ZONE_ENTRY_CONSTRUCT: u32 = 0x0061_fba0;
/// `0061fc10` (`run_0061fba0.rs`): `thiscall`, `RET 8`; constructs a 0x18-byte entry.
const SMALL_ENTRY_CONSTRUCT: u32 = 0x0061_fc10;
/// `0061fe70` in this file: the derived zone entry constructor.
const ZONE_ENTRY_DERIVED_CONSTRUCT: u32 = 0x0061_fe70;
/// `00620760` (`run_00620230.rs`): `thiscall`, `RET 8`, called on `entry + 0x10`.
const ZONE_ENTRY_SET: u32 = 0x0062_0760;
/// `006ecd40` (unnamed in the engine map): `thiscall`, `RET 4`, called on `entry + 0x10` with
/// the previous head's `+0x10` (or null).
const ZONE_ENTRY_LINK: u32 = 0x006e_cd40;
/// `0061fef0` in this file: the entry that follows an entry (`00726070` on `entry + 0x10`,
/// minus 0x10).
const ZONE_ENTRY_STEP: u32 = 0x0061_fef0;
/// `00620230` (`run_00620230.rs`): `thiscall`, `RET 4`: the scalar deleting destructor of a
/// zone target node.
const ZONE_TARGET_NODE_DESTROY: u32 = 0x0062_0230;
/// The listener's vtable (RTTI `.?AVBGSZoneTargetListener@@`).
const ZONE_LISTENER_VTABLE: u32 = 0x0104_d9e0;
/// The derived zone entry's vtable.
const ZONE_ENTRY_DERIVED_VTABLE: u32 = 0x0104_d9d8;

/// The shared body of `0061fc70` and `0061fd70`: looks the key up in the entry list at
/// `this + 0x14` (when the list is not null) and, when nothing is found, builds a new entry
/// (`construct` is the constructor to use), fills it, links it in front of the previous head and
/// makes it the new head. Answers the entry found or built.
fn zone_entry_add_target_with(
    e: &mut Engine,
    this: Ptr,
    construct: u32,
    key: u32,
    second: u32,
    third: u32,
) -> u32 {
    let mut found = if e.mem.u32(this.addr().wrapping_add(0x14)) == 0 {
        0
    } else {
        e.call(ZONE_ENTRY_FIND, &args![this, key]).u32()
    };
    if found == 0 {
        let block = e.call(ALLOCATE_BLOCK, &args![0x50u32]).u32();
        let entry = if block == 0 {
            0
        } else {
            e.call(construct, &args![block, key, second]).u32()
        };
        found = entry;
        e.call(
            ZONE_ENTRY_SET,
            &args![entry.wrapping_add(0x10), third, 1u32],
        );
        let head = e.mem.u32(this.addr().wrapping_add(0x14));
        let previous = if head == 0 {
            0
        } else {
            head.wrapping_add(0x10)
        };
        e.call(ZONE_ENTRY_LINK, &args![entry.wrapping_add(0x10), previous]);
        e.mem.set_u32(this.addr().wrapping_add(0x14), entry);
    }
    found
}

// Translated from 0061fc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061fc70 - C++ exception unwinding (FS:[0] frame) not translated
/// `ZoneEntry::AddTarget` (engine map name): `thiscall`, `RET 0xC`. Looks the key up in the
/// entries at `this + 0x14` (`0057b730`) when there are any; when none matches, allocates 0x50
/// bytes (`00553300`), constructs an entry in it (`0061fba0`, key and `second`), sets it up
/// (`00620760` on `entry + 0x10`, with `third` and 1), links it in front of the previous head
/// (`006ecd40`) and stores it as the new head. Answers the entry found or built.
pub fn zone_entry_add_target(e: &mut Engine, this: Ptr, key: u32, second: u32, third: u32) -> u32 {
    zone_entry_add_target_with(e, this, ZONE_ENTRY_CONSTRUCT, key, second, third)
}

// Translated from 0061fd70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061fd70 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map: the body of `0061fc70` with the derived entry constructor
/// (`0061fe70`) in place of `0061fba0`. `thiscall`, `RET 0xC`.
pub fn fn_0061fd70(e: &mut Engine, this: Ptr, key: u32, second: u32, third: u32) -> u32 {
    zone_entry_add_target_with(e, this, ZONE_ENTRY_DERIVED_CONSTRUCT, key, second, third)
}

// Translated from 0061fe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The derived zone entry's constructor: runs `0061fba0` with the two arguments, then stores
/// the vtable `0104d9d8` at `+0`. `thiscall`, `RET 8`; answers `this`.
pub fn fn_0061fe70(e: &mut Engine, this: Ptr, key: u32, second: u32) -> Ptr {
    e.call(ZONE_ENTRY_CONSTRUCT, &args![this, key, second]);
    e.mem.set_u32(this.addr(), ZONE_ENTRY_DERIVED_VTABLE);
    this
}

// Translated from 0061fea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall` (`this` in `ECX`), plain `RET`. Walks the entries from
/// `this + 0x14`: takes each entry's following one (`0061fef0`) before handing the entry to
/// `0087e9b0`, then clears `this + 0x14`.
pub fn fn_0061fea0(e: &mut Engine, this: Ptr) {
    let mut node = e.mem.u32(this.addr().wrapping_add(0x14));
    while node != 0 {
        let next = e.call(ZONE_ENTRY_STEP, &args![node]).u32();
        e.call(ZONE_TARGET_RELEASE, &args![node]);
        node = next;
    }
    e.mem.set_u32(this.addr().wrapping_add(0x14), 0);
}

// Translated from 0061fef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the decompiler's library match
/// `UMSFreeVirtualProcessorRoot::SchedulerProxy` is not its body). `thiscall`, plain `RET`.
/// Asks `00726070` on `this + 0x10` and answers its result minus 0x10, or null when it is null.
pub fn fn_0061fef0(e: &mut Engine, this: Ptr) -> u32 {
    let step = e
        .call(LIST_FIRST_NODE, &args![this.addr().wrapping_add(0x10)])
        .u32();
    if step == 0 {
        0
    } else {
        step.wrapping_sub(0x10)
    }
}

// Translated from 0061ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener's base constructor: stores the vtable `0104d9e0` at `+0` and clears `+4`.
/// `fastcall`, plain `RET`; answers `this`.
pub fn fn_0061ffb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ZONE_LISTENER_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    this
}

// Translated from 0061ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061ffe0 - C++ exception unwinding (FS:[0] frame) not translated
/// `FreeThreadProxyFactory::Create` is the decompiler's library match; the body allocates 0x18
/// bytes (`00401000`) and, when that worked, constructs them with `0061fc10`
/// (`key`, `second`). `thiscall`, `RET 8`; answers the object or null.
pub fn fn_0061ffe0(e: &mut Engine, _this: Ptr, key: u32, second: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(SMALL_ENTRY_CONSTRUCT, &args![block, key, second])
            .u32()
    }
}

// Translated from 00620060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00620060 - C++ exception unwinding (FS:[0] frame) not translated
/// `BGSZoneTargetListener::CreateTargetEntry` (engine map name): allocates 0x50 bytes
/// (`00553300`) and, when that worked, constructs them with `0061fba0` (`key`, `second`).
/// `thiscall`, `RET 8`; answers the object or null.
pub fn bgs_zone_target_listener_create_target_entry(
    e: &mut Engine,
    _this: Ptr,
    key: u32,
    second: u32,
) -> u32 {
    let block = e.call(ALLOCATE_BLOCK, &args![0x50u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(ZONE_ENTRY_CONSTRUCT, &args![block, key, second])
            .u32()
    }
}

// Translated from 006200e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSZoneTargetListener::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body
/// (`00620110`) and, when bit 0 of `flags` is set, frees the object (`00401030`). `thiscall`,
/// `RET 4`; answers `this`.
pub fn bgs_zone_target_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00620110(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006201c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`, plain `RET`. Walks the nodes from `this + 4`: takes
/// each node's successor (`00620180`) and then destroys the node with `00620230` (flag 1),
/// then clears `this + 4`.
pub fn fn_006201c0(e: &mut Engine, this: Ptr) {
    let mut node = e.mem.u32(this.addr().wrapping_add(4));
    while node != 0 {
        let next = e.call(ZONE_TARGET_NEXT, &args![node]).u32();
        // The exe tests the node again before destroying it; it is never null here.
        if node != 0 {
            e.call(ZONE_TARGET_NODE_DESTROY, &args![node, 1u32]);
        }
        node = next;
    }
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0062_0130,
            bgs_zone_target_listener_remove_target(Ptr, Ptr)
        ),
        entry!(0x0061_ff30, fn_0061ff30(Ptr, Ptr)),
        entry!(0x0062_0110, fn_00620110(Ptr)),
        entry!(
            0x0061_fc70,
            zone_entry_add_target(Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x0061_fd70, fn_0061fd70(Ptr, u32, u32, u32) -> u32),
        entry!(0x0061_fe70, fn_0061fe70(Ptr, u32, u32) -> Ptr),
        entry!(0x0061_fea0, fn_0061fea0(Ptr)),
        entry!(0x0061_fef0, fn_0061fef0(Ptr) -> u32),
        entry!(0x0061_ffb0, fn_0061ffb0(Ptr) -> Ptr),
        entry!(0x0061_ffe0, fn_0061ffe0(Ptr, u32, u32) -> u32),
        entry!(
            0x0062_0060,
            bgs_zone_target_listener_create_target_entry(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x0062_00e0,
            bgs_zone_target_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0062_01c0, fn_006201c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00620110_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0062_01c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0062_0110, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0062_01c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_d9e0);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_d9e0,
            "the vtable was stored before the call"
        );
    }
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

    #[test]
    fn at_00620130_hands_every_node_to_the_remove_function_with_the_target() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // A chain 0x100 -> 0x200 -> 0x300 -> end, kept at +4 of the node's own storage.
        e.register(LIST_FIRST_NODE, |_, _| 0x100u32.into_ret());
        e.register(ZONE_TARGET_NEXT, |_, a| match a[0] {
            0x100 => 0x200u32.into_ret(),
            0x200 => 0x300u32.into_ret(),
            _ => Ret::default(),
        });
        e.register(ZONE_TARGET_NODE_REMOVE, |_, _| Ret::default());
        e.call(0x0062_0130, &args![0x4000u32, 0x9000u32]);
        assert_eq!(calls(&e, LIST_FIRST_NODE), vec![vec![0x4000]]);
        assert_eq!(
            calls(&e, ZONE_TARGET_NEXT),
            vec![vec![0x100], vec![0x200], vec![0x300]]
        );
        assert_eq!(
            calls(&e, ZONE_TARGET_NODE_REMOVE),
            vec![
                vec![0x4000, 0x100, 0x9000],
                vec![0x4000, 0x200, 0x9000],
                vec![0x4000, 0x300, 0x9000]
            ]
        );
    }

    #[test]
    fn at_00620130_does_nothing_without_a_target_or_nodes() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_FIRST_NODE, |_, _| Ret::default());
        e.call(0x0062_0130, &args![0x4000u32, 0u32]);
        assert!(calls(&e, LIST_FIRST_NODE).is_empty());
        e.call(0x0062_0130, &args![0x4000u32, 0x9000u32]);
        assert_eq!(calls(&e, LIST_FIRST_NODE).len(), 1);
        assert!(calls(&e, ZONE_TARGET_NODE_REMOVE).is_empty());
    }

    #[test]
    fn at_0061ff30_replaces_the_stored_node_by_the_one_found_in_the_target() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_TARGET_LIST_FIND, |_, a| (a[1] + 0x100).into_ret());
        e.register(ZONE_TARGET_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x0061_ff30, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, ZONE_TARGET_LIST_FIND), vec![vec![0x9010, 0x5010]]);
        // found = 0x5110, minus 0x10.
        assert_eq!(e.mem.u32(this + 0x14), 0x5100);
        assert_eq!(calls(&e, ZONE_TARGET_RELEASE), vec![vec![0x9000]]);
    }

    #[test]
    fn at_0061ff30_handles_an_empty_listener_and_a_miss() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_TARGET_LIST_FIND, |_, _| Ret::default());
        e.register(ZONE_TARGET_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0);
        e.call(0x0061_ff30, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, ZONE_TARGET_LIST_FIND), vec![vec![0x9010, 0]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x0061_ff30, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn at_0061ff30_ignores_a_null_target() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x0061_ff30, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0x5000);
        assert!(calls(&e, ZONE_TARGET_RELEASE).is_empty());
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_ENTRY_FIND, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |e, _| e.mem.alloc(0x50).into_ret());
        e.register(ZONE_ENTRY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ZONE_ENTRY_DERIVED_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ZONE_ENTRY_SET, |_, _| Ret::default());
        e.register(ZONE_ENTRY_LINK, |_, _| Ret::default());
        e
    }

    #[test]
    fn at_0061fc70_answers_an_entry_that_is_already_there() {
        let mut e = rig();
        e.register(ZONE_ENTRY_FIND, |_, _| 0x7000u32.into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        let r = e.call(0x0061_fc70, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_eq!(r, 0x7000);
        assert_eq!(calls(&e, ZONE_ENTRY_FIND), vec![vec![this, 1]]);
        assert!(calls(&e, ALLOCATE_BLOCK).is_empty());
        assert_eq!(e.mem.u32(this + 0x14), 0x5000);
    }

    #[test]
    fn at_0061fc70_builds_an_entry_in_front_of_the_old_head() {
        let mut e = rig();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        let r = e.call(0x0061_fc70, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_ne!(r, 0);
        assert_eq!(calls(&e, ALLOCATE_BLOCK), vec![vec![0x50]]);
        assert_eq!(calls(&e, ZONE_ENTRY_CONSTRUCT), vec![vec![r, 1, 2]]);
        assert_eq!(calls(&e, ZONE_ENTRY_SET), vec![vec![r + 0x10, 3, 1]]);
        assert_eq!(calls(&e, ZONE_ENTRY_LINK), vec![vec![r + 0x10, 0x5010]]);
        assert_eq!(e.mem.u32(this + 0x14), r);
    }

    #[test]
    fn at_0061fc70_skips_the_lookup_and_the_link_target_for_an_empty_list() {
        let mut e = rig();
        let this = e.mem.alloc(0x20);
        let r = e.call(0x0061_fc70, &args![this, 1u32, 2u32, 3u32]).u32();
        assert!(calls(&e, ZONE_ENTRY_FIND).is_empty());
        assert_eq!(calls(&e, ZONE_ENTRY_LINK), vec![vec![r + 0x10, 0]]);
        assert_eq!(e.mem.u32(this + 0x14), r);
    }

    #[test]
    fn at_0061fc70_goes_on_with_a_null_entry_when_the_allocation_fails() {
        let mut e = rig();
        e.register(ALLOCATE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let r = e.call(0x0061_fc70, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_eq!(r, 0);
        assert!(calls(&e, ZONE_ENTRY_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, ZONE_ENTRY_SET), vec![vec![0x10, 3, 1]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn at_0061fd70_builds_with_the_derived_constructor() {
        let mut e = rig();
        let this = e.mem.alloc(0x20);
        let r = e.call(0x0061_fd70, &args![this, 4u32, 5u32, 6u32]).u32();
        assert_eq!(calls(&e, ZONE_ENTRY_DERIVED_CONSTRUCT), vec![vec![r, 4, 5]]);
        assert!(calls(&e, ZONE_ENTRY_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, ZONE_ENTRY_SET), vec![vec![r + 0x10, 6, 1]]);
        assert_eq!(e.mem.u32(this + 0x14), r);
    }

    #[test]
    fn at_0061fd70_answers_an_entry_that_is_already_there() {
        let mut e = rig();
        e.register(ZONE_ENTRY_FIND, |_, _| 0x7100u32.into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        let r = e.call(0x0061_fd70, &args![this, 4u32, 5u32, 6u32]).u32();
        assert_eq!(r, 0x7100);
        assert!(calls(&e, ALLOCATE_BLOCK).is_empty());
    }

    #[test]
    fn at_0061fe70_runs_the_base_constructor_then_stores_its_vtable() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // The base constructor leaves its own vtable; the derived one replaces it.
        e.register(ZONE_ENTRY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x10);
        let r = e.call(0x0061_fe70, &args![this, 7u32, 8u32]).u32();
        assert_eq!(r, this);
        assert_eq!(calls(&e, ZONE_ENTRY_CONSTRUCT), vec![vec![this, 7, 8]]);
        assert_eq!(e.mem.u32(this), 0x0104_d9d8);
    }

    #[test]
    fn at_0061fea0_releases_every_entry_and_clears_the_head() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_ENTRY_STEP, |_, a| match a[0] {
            0x100 => 0x200u32.into_ret(),
            _ => Ret::default(),
        });
        e.register(ZONE_TARGET_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x100);
        e.call(0x0061_fea0, &args![this]);
        assert_eq!(calls(&e, ZONE_ENTRY_STEP), vec![vec![0x100], vec![0x200]]);
        assert_eq!(
            calls(&e, ZONE_TARGET_RELEASE),
            vec![vec![0x100], vec![0x200]]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn at_0061fea0_does_nothing_for_an_empty_list() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x0061_fea0, &args![this]);
        assert!(calls(&e, ZONE_TARGET_RELEASE).is_empty());
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn at_0061fef0_steps_back_by_0x10_from_what_the_list_answers() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_FIRST_NODE, |_, a| (a[0] + 0x1000).into_ret());
        assert_eq!(e.call(0x0061_fef0, &args![0x400u32]).u32(), 0x1400);
        assert_eq!(calls(&e, LIST_FIRST_NODE), vec![vec![0x410]]);
        e.register(LIST_FIRST_NODE, |_, _| Ret::default());
        assert_eq!(e.call(0x0061_fef0, &args![0x400u32]).u32(), 0);
    }

    #[test]
    fn at_0061ffb0_stores_the_vtable_and_clears_the_list() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        assert_eq!(e.call(0x0061_ffb0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_d9e0);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn at_0061ffe0_constructs_a_small_entry_or_answers_null() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_NEW, |e, _| e.mem.alloc(0x18).into_ret());
        e.register(SMALL_ENTRY_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        let r = e.call(0x0061_ffe0, &args![0x9000u32, 3u32, 4u32]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x18]]);
        assert_eq!(calls(&e, SMALL_ENTRY_CONSTRUCT), vec![vec![r - 1, 3, 4]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x0061_ffe0, &args![0x9000u32, 3u32, 4u32]).u32(), 0);
        assert_eq!(calls(&e, SMALL_ENTRY_CONSTRUCT).len(), 1);
    }

    #[test]
    fn at_00620060_constructs_a_target_entry_or_answers_null() {
        let mut e = rig();
        let r = e.call(0x0062_0060, &args![0x9000u32, 3u32, 4u32]).u32();
        assert_eq!(calls(&e, ALLOCATE_BLOCK), vec![vec![0x50]]);
        assert_eq!(calls(&e, ZONE_ENTRY_CONSTRUCT), vec![vec![r, 3, 4]]);
        e.register(ALLOCATE_BLOCK, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_0060, &args![0x9000u32, 3u32, 4u32]).u32(), 0);
        assert_eq!(calls(&e, ZONE_ENTRY_CONSTRUCT).len(), 1);
    }

    #[test]
    fn at_006200e0_frees_only_when_bit_0_is_set() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0062_01c0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x0062_00e0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_d9e0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(e.call(0x0062_00e0, &args![this, 2u32]).u32(), this);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn at_006201c0_destroys_every_node_and_clears_the_head() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_TARGET_NEXT, |_, a| match a[0] {
            0x100 => 0x200u32.into_ret(),
            _ => Ret::default(),
        });
        e.register(ZONE_TARGET_NODE_DESTROY, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x100);
        e.call(0x0062_01c0, &args![this]);
        assert_eq!(
            calls(&e, ZONE_TARGET_NODE_DESTROY),
            vec![vec![0x100, 1], vec![0x200, 1]]
        );
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call(0x0062_01c0, &args![this]);
        assert_eq!(calls(&e, ZONE_TARGET_NODE_DESTROY).len(), 2);
    }
}
