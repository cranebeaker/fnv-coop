//! `fallout shared/packagelist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (thiscall on a list node): the address of the node's item word
/// (on the package list itself: the address of its first node).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall): the word at +4 (on a list node: the next node).
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (thiscall on a list node): removes the node's item.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `00905330` (thiscall on a list node, `RET 4`: item pointer): the call made
/// on the previous node when a package is missing.
const NODE_REMOVE_AFTER: u32 = 0x0090_5330;
/// `00726c60` (thiscall on a list node, `RET 4`: item pointer): stores the item.
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// `008256d0` (thiscall on a list node): whether the list is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `TESForm` (`01183028`), `TESPackage` (`011846a0`),
/// `TESObjectREFR` (`011841cc`), `Actor` (`011846d4`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_PACKAGE: u32 = 0x0118_46a0;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// The game calendar object (`011de7b8`; `Calendar::GetDay` `00867d60`,
/// `Calendar::GetMonth` `00867d20`, `Calendar::GetHour` `00867da0`, Xbox PDB).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
const CALENDAR_GET_MONTH: u32 = 0x0086_7d20;
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// `005d43c0` (thiscall on a reference): its extra data list.
const REF_EXTRA_LIST: u32 = 0x005d_43c0;
/// `008d8520` (thiscall on a reference): its process.
const REF_PROCESS: u32 = 0x008d_8520;
/// `0041ca90` (thiscall on a package): its type.
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `00678d40` (thiscall on a package, `RET 4`).
const PACKAGE_FLAG_TEST: u32 = 0x0067_8d40;
/// `Script::SetActionFlag` (Xbox PDB name, `005ac750`, cdecl: package, extra
/// list, flag).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `_ftol2` (`00ec62c0`): ST0 to an integer.
const FTOL2: u32 = 0x00ec_62c0;

// Translated from 0067e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageList::FreePackageList` (Xbox PDB): empties the list
/// (`00470470` on its first node).
pub fn package_list_free_package_list(e: &mut Engine, this: Ptr) {
    let list = e.call(NODE_ITEM, &args![this]).u32();
    e.call(0x0047_0470, &args![list]);
}

// Translated from 0067e560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067e560 - the exception-unwinding frame (FS:[0], destroys the name string if a step throws) is not translated.
/// `PackageList::InitItem` (Xbox PDB, `RET 4`: the owning form or null):
/// turns the list's form ids into packages. For each node of a non-empty
/// list (`008256d0`): the id gets the owner's compile index (only with an
/// owner; `GetFile(-1)`), is looked up (`004839c0`) and cast to `TESPackage`.
/// A package is stored in the node (`00726c60`), which becomes the previous
/// node. Otherwise a `MASTERFILE: Could not find Package (%08X) for %s.`
/// warning (`005b5e40`) names the owner (its vtable slot 0x90 fills a string;
/// "UNKNOWN form" without an owner) and the node is dropped: through the
/// previous node's `00905330` and its next node when there is a previous
/// node, else in place (`0063f7b0`).
pub fn package_list_init_item(e: &mut Engine, this: Ptr, owner: u32) {
    let mut node = e.call(NODE_ITEM, &args![this]).u32();
    let mut previous = 0u32;
    let file = if owner != 0 {
        e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32()
    } else {
        0
    };
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let id = e.with_stack(4, |e, slot| {
            let id = e.mem.u32(item);
            e.mem.set_u32(slot.addr(), id);
            if owner != 0 {
                e.call(0x0048_5d50, &args![slot, file]);
            }
            e.mem.u32(slot.addr())
        });
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let package = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_PACKAGE, 0u32],
            )
            .u32();
        if package == 0 {
            // `BSString` on the stack (8 bytes): constructor `004037b0`, set
            // `004037f0` (text, 0), text `00559450`, destructor `004037d0`.
            e.with_stack(8, |e, name| {
                e.call(0x0040_37b0, &args![name]);
                if owner != 0 {
                    e.vcall(owner, 0x90, &args![name]);
                } else {
                    e.call(0x0040_37f0, &args![name, 0x0106_8cc8u32, 0u32]);
                }
                let text = e.call(0x0055_9450, &args![name]).u32();
                e.call(0x005b_5e40, &args![0x0106_8c94u32, id, text]);
                if previous != 0 {
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), id);
                        e.call(NODE_REMOVE_AFTER, &args![previous, slot]);
                    });
                    node = e.call(NODE_NEXT, &args![previous]).u32();
                } else {
                    e.call(NODE_REMOVE, &args![node]);
                }
                e.call(0x0040_37d0, &args![name]);
            });
        } else {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), package);
                e.call(NODE_SET_ITEM, &args![node, slot]);
            });
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 0067e700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `hour` (truncated by `_ftol2`) falls in the schedule window of
/// `duration` hours from `start` (cdecl, three words): true at `start`
/// itself; for a window past hour 23 (`start + duration > 0x17`), true when
/// the hour is before the end and either after `start` or before the end
/// less 24; otherwise true strictly between `start` and the end. Signed
/// compares.
pub fn fn_0067e700(e: &mut Engine, start: i32, duration: i32, hour: f32) -> bool {
    let h = e.call(FTOL2, &args![x87_load32(hour.to_bits())]).i32();
    let end = start.wrapping_add(duration);
    if h == start {
        return true;
    }
    if end > 0x17 {
        let wrapped = end.wrapping_sub(0x18);
        (h > start || h < wrapped) && h < end
    } else {
        h > start && h < end
    }
}

/// `"%s Picked Package %s (%s)"` (`01068cd8`).
const TEXT_PICKED_PACKAGE: u32 = 0x0106_8cd8;

/// Whether `FindCurrentPackage` passes over this package before its schedule
/// (the three tests at `0067e802`..`0067e866`): the result of vtable slot 0x138
/// (with the reference), `TESPackage::IsNeverToRun` (`00674e40`),
/// `TESPackage::GetOncePerDay` (`00670f90`) and the reference's run-once-a-day
/// list (`ExtraDataList::IsInRunOnceDayPackageList`, `0041d8c0`).
fn skipped_before_schedule(e: &mut Engine, package: u32, reference: u32) -> bool {
    let considered = e.vcall(package, 0x138, &args![reference]).bool()
        && !e.call(0x0067_4e40, &args![package]).bool()
        && !e.call(0x0067_0f90, &args![package]).bool();
    if considered {
        return false;
    }
    if !e.call(0x0067_0f90, &args![package]).bool() {
        return true;
    }
    let list = e.call(REF_EXTRA_LIST, &args![reference]).u32();
    e.call(0x0041_d8c0, &args![list, package]).bool()
}

/// The schedule tests of `FindCurrentPackage` (`0067e881`..`0067ea11`) on the
/// package's schedule `schedule` (`0041d8a0`): true when the package may run
/// now.
fn schedule_allows(e: &mut Engine, package: u32, schedule: u32) -> bool {
    let mut any_flag = false;
    let mut day_matches = false;
    let mut month_matches = false;
    let start = e.call(0x0086_a460, &args![schedule]).u8() as i8 as i32;
    let duration = e.call(NODE_NEXT, &args![schedule]).u32() as i32;
    if start.wrapping_add(duration) > 0x18 {
        any_flag = e.call(PACKAGE_FLAG_TEST, &args![package, 1u32]).bool();
        let mut day =
            (e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as i8 as i32).wrapping_sub(1);
        if day < 0 {
            day = 0x1e;
        }
        if e.call(0x0067_8ce0, &args![schedule]).u8() as i8 as i32 == day {
            day_matches = true;
        }
        let mut month = e
            .call(CALENDAR_GET_MONTH, &args![CALENDAR])
            .i32()
            .wrapping_sub(1);
        if month < 0 {
            month = 0xc;
        }
        if e.call(0x0067_8ca0, &args![schedule]).i32() == month {
            month_matches = true;
        }
    }
    if e.call(0x0067_8ca0, &args![schedule]).i32() != -1 {
        let month = e.call(0x0067_8ca0, &args![schedule]).i32();
        if month != e.call(CALENDAR_GET_MONTH, &args![CALENDAR]).i32() && !month_matches {
            return false;
        }
    }
    if e.call(0x0067_8ce0, &args![schedule]).u8() as i8 != 0 {
        let day = e.call(0x0067_8ce0, &args![schedule]).u8() as i8 as i32;
        if day != e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as i8 as i32 && !day_matches {
            return false;
        }
    }
    if e.call(0x0067_8cc0, &args![schedule]).i32() != -1
        && !e.call(PACKAGE_FLAG_TEST, &args![package, 0u32]).bool()
        && !any_flag
    {
        return false;
    }
    if e.call(0x0086_a460, &args![schedule]).u8() as i8 as i32 != -1 {
        // `FSTP float ptr [ESP]`: the hour is passed as a float.
        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
        let hour = f32::from_bits(x87_store32(hour));
        let duration = e.call(NODE_NEXT, &args![schedule]).u32() as i32;
        let start = e.call(0x0086_a460, &args![schedule]).u8() as i8 as i32;
        if !fn_0067e700(e, start, duration, hour) {
            return false;
        }
    }
    true
}

// Translated from 0067e780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067e780 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated.
/// `PackageList::FindCurrentPackage` (Xbox PDB, `RET 4`: the reference or
/// null): the first package of the list that may run for the reference now,
/// or the reference's current package (`MobileObject::GetCurrentPackage`,
/// `009344a0`; 0 without a reference) when none does or the list is empty.
/// A package is passed over by the pre-schedule tests, without a schedule
/// (`0041d8a0`), by its schedule (month, day, a location-type word, the hour
/// window through `0067e700`, with the wrap-around flags past hour 24), or
/// when its conditions (`0087eaa0`, `TESCondition::IsTrue` with the
/// reference and the package target's reference) fail. For the package
/// found: a `"%s Picked Package %s (%s)"` line (`sprintf`, `00703c00`) when
/// the reference is the interface's target (`00703180`); its procedure type
/// computed (`TESPackage::CalculateProcedureType` with `00569b80`) when
/// `009611e0` gives -1; when it replaces another current package: a type-1
/// current package removes the follower from the actor of the process's
/// slot-0x128 object (`ExtraDataList::RemoveFollower`), the old package gets
/// action flag 0x800 and the process's slots 0x59C (reference, old package)
/// and 0x5A8 (0) run; the new package gets action flag 0x200 when it differs
/// from the current one.
pub fn package_list_find_current_package(e: &mut Engine, this: Ptr, reference: u32) -> u32 {
    let mut node = e.call(NODE_ITEM, &args![this]).u32();
    let current = if reference != 0 {
        e.call(0x0093_44a0, &args![reference]).u32()
    } else {
        0
    };
    if e.call(LIST_IS_EMPTY, &args![node]).bool() {
        return current;
    }
    loop {
        if node == 0 {
            return current;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let package = e.mem.u32(item);
        if package == 0 {
            return current;
        }
        if !skipped_before_schedule(e, package, reference) {
            let schedule = e.call(0x0041_d8a0, &args![package]).u32();
            if schedule != 0
                && schedule_allows(e, package, schedule)
                && conditions_hold(e, package, reference)
            {
                picked(e, package, reference, current);
                return package;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// The package's conditions (`0087eaa0`) with the reference and the
/// reference of the package's target (`00671d10`, `PackageTarget::GetTargReference`
/// `00680020`; 0 without a target).
fn conditions_hold(e: &mut Engine, package: u32, reference: u32) -> bool {
    let target = e.call(0x0067_1d10, &args![package]).u32();
    let target_reference = if target != 0 {
        e.call(0x0068_0020, &args![target]).u32()
    } else {
        0
    };
    let conditions = e.call(0x0087_eaa0, &args![package]).u32();
    e.call(0x0068_0c30, &args![conditions, reference, target_reference])
        .bool()
}

/// What `FindCurrentPackage` does with the package it picked (`0067ea6e`..`0067ec16`).
fn picked(e: &mut Engine, package: u32, reference: u32, current: u32) {
    if e.call(0x0070_3180, &[]).u32() == reference {
        let name = e.vcall(package, 0x130, &[]).u32();
        let package_name = e.call(0x0046_4f30, &args![name, 0u32]).u32();
        let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
        let kind_text = e.mem.u32(0x0119_bcb0u32.wrapping_add(kind.wrapping_mul(4)));
        let kind_name = e.call(0x0046_4f30, &args![kind_text, 0u32]).u32();
        let reference_name = e.call(0x0055_d520, &args![reference]).u32();
        // The line buffer on the stack (0x134 bytes).
        e.with_stack(0x134, |e, line| {
            e.call(
                0x00ec_623a,
                &args![
                    line,
                    TEXT_PICKED_PACKAGE,
                    reference_name,
                    kind_name,
                    package_name
                ],
            );
            e.call(0x0070_3c00, &args![line]);
        });
    }
    if e.call(0x0096_11e0, &args![package]).i32() == -1 {
        let value = e.call(0x0056_9b80, &args![reference]).u32();
        e.call(0x0067_77b0, &args![package, value]);
    }
    if current != 0 && current != package && e.call(PACKAGE_TYPE, &args![current]).u32() == 1 {
        let process = e.call(REF_PROCESS, &args![reference]).u32();
        let object = e.vcall(process, 0x128, &[]).u32();
        let actor = e
            .call(
                RT_DYNAMIC_CAST,
                &args![object, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
            )
            .u32();
        if actor != 0 {
            let list = e.call(REF_EXTRA_LIST, &args![actor]).u32();
            e.call(0x0042_2690, &args![list, reference]);
        }
    }
    if current != 0 && current != package {
        let list = e.call(REF_EXTRA_LIST, &args![reference]).u32();
        e.call(SET_ACTION_FLAG, &args![current, list, 0x800u32]);
        let process = e.call(REF_PROCESS, &args![reference]).u32();
        e.vcall(process, 0x59c, &args![reference, current]);
        let process = e.call(REF_PROCESS, &args![reference]).u32();
        e.vcall(process, 0x5a8, &args![0u32]);
    }
    if package != 0 && package != current {
        let list = e.call(REF_EXTRA_LIST, &args![reference]).u32();
        e.call(SET_ACTION_FLAG, &args![package, list, 0x200u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_e400, package_list_free_package_list(Ptr)),
        entry!(0x0067_e560, package_list_init_item(Ptr, u32)),
        entry!(0x0067_e700, fn_0067e700(i32, i32, f32) -> bool),
        entry!(0x0067_e780, package_list_find_current_package(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_0067e400_frees_the_first_node() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| ret(a[0] + 0x10));
        e.register(0x0047_0470, ok);
        e.call_log = Some(vec![]);
        e.call(0x0067_e400, &[0x3000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0047_0470), vec![vec![0x3010]]);
    }

    /// List doubles: a node is two words (item, next); the list object is its
    /// first node.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0082_56d0, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(0x0090_5330, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(0x0072_6c60, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e
    }

    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn items(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    fn init_engine() -> Engine {
        let mut e = list_engine();
        e.register(0x0048_4e60, |_, _| ret(0x5000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| ret(a[0] << 4));
        // Forms whose id ends in 1 are packages.
        e.register(0x00ec_43fb, |_, a| {
            ret(if a[0] & 0xf0 == 0x10 { a[0] + 1 } else { 0 })
        });
        e.register(0x0040_37b0, ok);
        e.register(0x0040_37f0, ok);
        e.register(0x0040_37d0, ok);
        e.register(0x0055_9450, |_, _| ret(0x7777));
        e.register(0x005b_5e40, ok);
        e
    }

    #[test]
    fn at_0067e560_resolves_packages_and_drops_missing_ones() {
        // No owner: ids stay as they are; "UNKNOWN form".
        let mut e = init_engine();
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[2, 1, 3, 1]);
        e.call_log = Some(vec![]);
        e.call(0x0067_e560, &[list, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, list), vec![0x11, 0x11]);
        assert!(only(&log, 0x0048_5d50).is_empty());
        assert_eq!(
            only(&log, 0x0040_37f0).len(),
            2,
            "the name is set to UNKNOWN form for each miss"
        );
        assert_eq!(only(&log, 0x0040_37f0)[0][1..], [0x0106_8cc8, 0]);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![0x0106_8c94, 2, 0x7777], vec![0x0106_8c94, 3, 0x7777]]
        );
        // The first miss had no previous node, the second did.
        assert_eq!(only(&log, 0x0063_f7b0).len(), 1);
        assert_eq!(only(&log, 0x0090_5330).len(), 1);
        assert_eq!(only(&log, 0x0040_37d0).len(), 2);

        // With an owner: compile index, the owner's slot 0x90 names it.
        let mut e = init_engine();
        e.register(0x0048_39c0, |_, a| ret((a[0] & 0xff) << 4));
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x30]);
        e.mem.set_u32(0x0090_0090, 0x00a0_0090);
        e.register(0x00a0_0090, ok);
        e.map(0x0088_0000, 0x1000);
        e.mem.set_u32(0x0088_0000, 0x0090_0000);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[2]);
        e.call_log = Some(vec![]);
        e.call(0x0067_e560, &[list, 0x0088_0000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x0048_4e60),
            vec![vec![0x0088_0000, 0xffff_ffff]]
        );
        assert_eq!(only(&log, 0x0048_5d50)[0][1], 0x5000);
        assert_eq!(only(&log, 0x00a0_0090).len(), 1);
        assert!(only(&log, 0x0040_37f0).is_empty());
        assert_eq!(only(&log, 0x005b_5e40)[0][1], 0x0100_0002);
        assert!(items(&e, list).is_empty());
    }

    fn window(start: i32, duration: i32, hour: f32) -> bool {
        let mut e = Engine::new();
        // `_ftol2` truncates.
        e.register(0x00ec_62c0, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(v.trunc() as i32 as u32)
        });
        e.call(
            0x0067_e700,
            &[start as u32, duration as u32, hour.to_bits()],
        )
        .bool()
    }

    #[test]
    fn at_0067e700_tests_the_hour_window() {
        assert!(window(8, 4, 8.9));
        assert!(window(8, 4, 11.5));
        assert!(!window(8, 4, 12.0));
        assert!(!window(8, 4, 7.9));
        // Past midnight: 22 for 4 hours ends at 26, wraps to 2.
        assert!(window(22, 4, 23.0));
        assert!(window(22, 4, 1.5));
        assert!(!window(22, 4, 2.0));
        assert!(!window(22, 4, 21.0));
        // The end not past 23: strict on both sides.
        assert!(!window(20, 3, 23.0));
        assert!(window(20, 4, 23.0));
    }

    /// A package with its test answers in words at +0x10 onwards (see the
    /// doubles in `find_engine`) and a schedule.
    fn package(e: &mut Engine, words: [u32; 8], schedule: [u32; 5]) -> u32 {
        let p = e.mem.alloc(0x40);
        e.mem.set_u32(p, 0x0090_0000);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(p + 0x10 + 4 * i as u32, *w);
        }
        let s = e.mem.alloc(0x20);
        for (i, w) in schedule.iter().enumerate() {
            e.mem.set_u32(s + 4 * i as u32, *w);
        }
        e.mem.set_u32(p + 0x1c, s);
        p
    }

    /// Package words: +0x10 slot 0x138, +0x14 never-to-run, +0x18 once a day,
    /// +0x1c schedule, +0x20 conditions, +0x24 type, +0x28 `009611e0`,
    /// +0x2c `00678d40`. Schedule words: +0 start byte, +4 duration, +8 day
    /// byte, +0xc month, +0x10 `00678cc0`.
    fn find_engine(day: u32, month: u32, hour: f64) -> Engine {
        let mut e = list_engine();
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x200]);
        for slot in [0x128u32, 0x130, 0x138, 0x59c, 0x5a8] {
            e.mem.set_u32(0x0090_0000 + slot, 0x00a0_0000 + slot);
        }
        e.register(0x00a0_0138, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00a0_0130, |_, _| ret(0x0bb0));
        e.register(0x00a0_0128, |_, _| ret(0x0cc0));
        e.register(0x00a0_059c, ok);
        e.register(0x00a0_05a8, ok);
        e.register(0x0067_4e40, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(0x0067_0f90, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(0x0041_d8a0, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(0x0087_eaa0, |_, a| ret(a[0]));
        e.register(0x0068_0c30, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(0x0041_ca90, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(0x0096_11e0, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(0x0067_8d40, |e, a| ret(e.mem.u32(a[0] + 0x2c) >> a[1]));
        e.register(0x0086_a460, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0067_8ce0, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(0x0067_8ca0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(0x0067_8cc0, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register_double(0x0086_7d60, move |_, _| ret(day));
        e.register_double(0x0086_7d20, move |_, _| ret(month));
        e.register_double(0x0086_7da0, move |_, _| Ret {
            st0: hour,
            ..Ret::default()
        });
        e.register(0x00ec_62c0, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(v.trunc() as i32 as u32)
        });
        e.register(0x0067_1d10, |_, _| ret(0));
        e.register(0x0068_0020, |_, a| ret(a[0] + 1));
        e.register(0x0070_3180, |_, _| ret(0x0999_0000));
        e.register(0x005d_43c0, |_, a| ret(a[0] + 0x44));
        e.register(0x0041_d8c0, |_, _| ret(0));
        e.register(0x0093_44a0, |_, _| ret(0));
        e.register(0x0056_9b80, |_, _| ret(0x31));
        e.register(0x0067_77b0, ok);
        e.register(0x008d_8520, |_, _| ret(0x0098_0000));
        e.map(0x0098_0000, 0x1000);
        e.mem.set_u32(0x0098_0000, 0x0090_0000);
        e.register(0x00ec_43fb, |_, a| ret(a[0] + 1));
        e.register(0x0042_2690, ok);
        e.register(0x005a_c750, ok);
        e.register(0x0046_4f30, |_, a| ret(a[0] + 0x1000));
        e.register(0x0055_d520, |_, _| ret(0x0ee0));
        e.register(0x00ec_623a, ok);
        e.register(0x0070_3c00, ok);
        e
    }

    /// A schedule that always allows (no start hour, any month and day).
    const ANY_TIME: [u32; 5] = [0xff, 0, 0, 0xffff_ffff, 0xffff_ffff];

    #[test]
    fn at_0067e780_returns_the_current_package_for_an_empty_list() {
        let mut e = find_engine(1, 1, 0.0);
        e.register(0x0093_44a0, |_, _| ret(0x4321));
        let list = e.mem.alloc(8);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), 0x4321);
        // No reference: no current package.
        assert_eq!(e.call(0x0067_e780, &[list, 0]).u32(), 0);
    }

    #[test]
    fn at_0067e780_picks_the_first_package_that_may_run() {
        let mut e = find_engine(5, 3, 13.5);
        e.map(0x0333_0000, 0x1000);
        // Skipped: slot 0x138 false and not once a day.
        let a = package(&mut e, [0, 0, 0, 0, 1, 0, 0, 0], ANY_TIME);
        // Skipped: its month is 2 (the calendar's is 3).
        let b = package(
            &mut e,
            [1, 0, 0, 0, 1, 0, 0, 0],
            [0xff, 0, 0, 2, 0xffff_ffff],
        );
        // Skipped: conditions fail.
        let c = package(&mut e, [1, 0, 0, 0, 0, 0, 0, 0], ANY_TIME);
        // Skipped: outside its hours (8 for 4).
        let d = package(
            &mut e,
            [1, 0, 0, 0, 1, 0, 0, 0],
            [8, 4, 0, 0xffff_ffff, 0xffff_ffff],
        );
        // Runs: 13 for 2 hours, day 5, month 3.
        let f = package(
            &mut e,
            [1, 0, 0, 0, 1, 0, 0xffff_ffff, 0],
            [13, 2, 5, 3, 0xffff_ffff],
        );
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[a, b, c, d, f]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), f);
        let log = e.call_log.take().unwrap();
        // The procedure type is computed (`009611e0` gave -1).
        assert_eq!(only(&log, 0x0067_77b0), vec![vec![f, 0x31]]);
        // New package, no current one: action flag 0x200 only.
        assert_eq!(only(&log, 0x005a_c750), vec![vec![f, 0x0333_0044, 0x200]]);
        assert!(only(&log, 0x0070_3c00).is_empty());
        // The hour went through `_ftol2` as the float 13.5.
        let ftol = only(&log, 0x00ec_62c0);
        let hour = f64::from_bits(
            u64::from(ftol[ftol.len() - 1][0]) | (u64::from(ftol[ftol.len() - 1][1]) << 32),
        );
        assert_eq!(hour, 13.5);
        // Conditions get the reference and no target reference.
        assert_eq!(only(&log, 0x0068_0c30)[0], vec![c, 0x0333_0000, 0]);
    }

    #[test]
    fn at_0067e780_replaces_the_current_package() {
        let mut e = find_engine(5, 3, 13.5);
        // The reference is the interface's target: the debug line is printed.
        e.register(0x0070_3180, |_, _| ret(0x0333_0000));
        // The package type names (`0119bcb0`, indexed by type).
        e.map(0x0119_b000, 0x1000);
        let old = package(&mut e, [0, 0, 0, 0, 0, 1, 0, 0], ANY_TIME);
        e.register_double(0x0093_44a0, move |_, _| ret(old));
        // A target reference: 00671d10 gives 0x50, its reference 0x51.
        e.register(0x0067_1d10, |_, _| ret(0x50));
        let p = package(&mut e, [1, 0, 0, 0, 1, 2, 0, 0], ANY_TIME);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[p]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), p);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0068_0c30)[0], vec![p, 0x0333_0000, 0x51]);
        assert_eq!(
            only(&log, 0x00ec_623a)[0][1..],
            [0x0106_8cd8, 0x0ee0, 0x1000, 0x1bb0]
        );
        assert!(only(&log, 0x0067_77b0).is_empty());
        // The old package is of type 1: the follower is removed.
        assert_eq!(
            only(&log, 0x00ec_43fb),
            vec![vec![0x0cc0, 0, 0x0118_41cc, 0x0118_46d4, 0]]
        );
        assert_eq!(
            only(&log, 0x0042_2690),
            vec![vec![0x0cc1 + 0x44, 0x0333_0000]]
        );
        assert_eq!(
            only(&log, 0x005a_c750),
            vec![vec![old, 0x0333_0044, 0x800], vec![p, 0x0333_0044, 0x200]]
        );
        assert_eq!(
            only(&log, 0x00a0_059c),
            vec![vec![0x0098_0000, 0x0333_0000, old]]
        );
        assert_eq!(only(&log, 0x00a0_05a8), vec![vec![0x0098_0000, 0]]);
    }

    #[test]
    fn at_0067e780_schedule_flags_past_hour_24() {
        // Day 0 and month 0: the window past 24 hours lets the previous day
        // (day 0 less 1 is 0x1E) and the previous month (0xC) match.
        let mut e = find_engine(0, 0, 1.0);
        let p = package(
            &mut e,
            [1, 0, 0, 0, 1, 0, 0, 0],
            [22, 4, 0x1e, 0xc, 0xffff_ffff],
        );
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[p]);
        e.map(0x0333_0000, 0x1000);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), p);

        // A location-type word not -1 needs a package flag: none set.
        let mut e = find_engine(1, 1, 1.0);
        let p = package(
            &mut e,
            [1, 0, 0, 0, 1, 0, 0, 0],
            [0xff, 0, 0, 0xffff_ffff, 4],
        );
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[p]);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), 0);
        // Flag bit 0 of the package's word: allowed.
        e.mem.set_u32(p + 0x2c, 1);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), p);

        // Once a day and in the run-once list: skipped.
        let mut e = find_engine(1, 1, 1.0);
        e.register(0x0041_d8c0, |_, _| ret(1));
        let p = package(&mut e, [1, 0, 1, 0, 1, 0, 0, 0], ANY_TIME);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[p]);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), 0);
        // Not in the list: runs even with slot 0x138 false.
        e.register(0x0041_d8c0, |_, _| ret(0));
        e.mem.set_u32(p + 0x10, 0);
        assert_eq!(e.call(0x0067_e780, &[list, 0x0333_0000]).u32(), p);
    }
}
