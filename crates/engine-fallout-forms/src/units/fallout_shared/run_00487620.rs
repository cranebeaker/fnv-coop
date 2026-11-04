//! `fallout shared/run_00487620` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00476c00`: returns a function-local static object (guard bit at
/// `011c4b4c`, constructed at `011c4180` on first use).
const GET_STATIC_OBJECT: u32 = 0x0047_6c00;
/// `00476be0`: thiscall on that object; its body is `00aa5230(this, -1)`.
const CALL_WITH_MINUS_ONE: u32 = 0x0047_6be0;

// Translated from 00487f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name; other translations in this crate call it the random
/// number (not confirmed here). Calls `00476c00`, then `00476be0` on the
/// object it returned, and returns that call's result in EAX.
pub fn fn_00487f50(e: &mut Engine) -> u32 {
    let object = e.call(GET_STATIC_OBJECT, &args![]).u32();
    e.call(CALL_WITH_MINUS_ONE, &args![object]).u32()
}

layout! {
    /// `TESLeveledList` (Xbox PDB), 0x1c bytes; the PC code uses the same
    /// offsets.
    pub struct TESLeveledList: 0x1c {
        /// `leveledList` (Xbox PDB): a `BSSimpleList<LEVELED_OBJECT *>`.
        0x04 leveledList: Inline<crate::types::BSSimpleList>,
        /// `cChanceNone` (Xbox PDB).
        0x0C cChanceNone: u8,
        /// `cLLFlags` (Xbox PDB).
        0x0D cLLFlags: u8,
        /// `pChanceGlobal` (Xbox PDB): `TESGlobal *`.
        0x10 pChanceGlobal: u32,
        /// `ScriptAddedList` (Xbox PDB): a `BSSimpleList<LEVELED_OBJECT *>`.
        0x14 ScriptAddedList: Inline<crate::types::BSSimpleList>,
    }

    /// `LEVELED_OBJECT` (Xbox PDB), 0x0c bytes; the PC code uses the same
    /// offsets.
    pub struct LeveledObject: 0x0c {
        /// `pForm` (Xbox PDB).
        0x00 pForm: u32,
        /// `sCount` (Xbox PDB).
        0x04 sCount: u16,
        /// `sLevel` (Xbox PDB).
        0x06 sLevel: u16,
        /// `pItemExtra` (Xbox PDB): `ContainerItemExtra *`.
        0x08 pItemExtra: u32,
    }
}

/// `TESLeveledList`'s vtable (`0101cd04`, RTTI `.?AVTESLeveledList@@`).
const TES_LEVELED_LIST_VTABLE: u32 = 0x0101_cd04;
/// `00402d30` (`thiscall`): the `BaseFormComponent` base constructor (sets
/// its vtable).
const BASE_FORM_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// `0096a2d0` (`thiscall`): a `BSSimpleList` node constructor (item and next 0).
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0046ffb0` (`thiscall`): a `BSSimpleList` destructor (its body calls
/// `00470470`).
const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `00470470` (`thiscall`): frees a `BSSimpleList`'s nodes after the head and
/// clears its item.
const SIMPLE_LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// `00717e50` (`thiscall`): `this + 4`, the address of the leveled list's
/// `leveledList` (folded body; the engine map names it after another class).
const LEVELED_LIST_HEAD: u32 = 0x0071_7e50;
/// `006815c0` (`fastcall`): answers the node itself (the item's address).
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00726070` (`fastcall`): the node's next (`+4`).
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0` (`fastcall`): whether a node is empty (item and next null).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `0063f7b0` (`fastcall`): removes the list's head item.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `005ae3d0` (`thiscall`, `RET 4`, the item's address): inserts before the
/// node's item.
const LIST_INSERT_BEFORE: u32 = 0x005a_e3d0;
/// `00905820` (`thiscall`, `RET 4`, the item's address): appends at the end.
const LIST_APPEND: u32 = 0x0090_5820;
/// `00481760` (`thiscall`, `RET 4`): a `LEVELED_OBJECT`'s deleting destructor.
const LEVELED_OBJECT_DELETE: u32 = 0x0048_1760;
/// `00526ac0` (`thiscall`): the global's value (x87 `ST0`).
const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// `00401030`: free (`cdecl`).
const FREE: u32 = 0x0040_1030;
/// `00401000`: allocate (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
/// `00401170` (`thiscall`): `TESForm::cFormType` (the byte at +4).
const FORM_TYPE: u32 = 0x0040_1170;
/// `00487f50`: a random number (this unit's other translation; called by
/// address as the exe calls it).
const RANDOM: u32 = 0x0048_7f50;
/// `TESContainer::TESContainer` (`00481610`).
const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
/// The container's destructor (`00481680`).
const CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// Adds an entry to a container (`004818e0`, `RET 0xc`: form, count, extra).
const CONTAINER_ADD: u32 = 0x0048_18e0;
/// Multiplies a container's counts (`00482040`, `RET 4`).
const CONTAINER_MULTIPLY: u32 = 0x0048_2040;
/// Moves another container's entries into this one (`00481fd0`, `RET 4`).
const CONTAINER_MERGE: u32 = 0x0048_1fd0;
/// The form vtable slot (byte offset) asked before an item is added to the
/// container (a `bool`; its meaning is not checked here).
const FORM_CAN_BE_ADDED_SLOT: u32 = 0xe4;
/// `TESForm::AddChange` slot (Xbox PDB `TESForm` vtable).
const ADD_CHANGE_SLOT: u32 = 0x48;
/// The change flag `00487a00` passes.
const CHANGE_LEVELED_LIST_ADDED: u32 = 0x8000_0000;
/// `TESLeveledList` vtable slot `GetChanceNone` (Xbox PDB).
const GET_CHANCE_NONE_SLOT: u32 = 0x10;
/// `TESLeveledList` vtable slot `GetMultCalc` (Xbox PDB).
const GET_MULT_CALC_SLOT: u32 = 0x14;
/// `TESLeveledList` vtable slot `GetMaxLevelDifference` (Xbox PDB).
const GET_MAX_LEVEL_DIFFERENCE_SLOT: u32 = 0x18;
/// `TESObjectREFR::GetCalcLevel` (`00567e10`, `RET 4`).
const GET_CALC_LEVEL: u32 = 0x0056_7e10;
/// `005d43c0` (`thiscall`): the reference's `ExtraDataList` (`+0x44`).
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `004214f0` (`thiscall`, `RET 8`): writes the extra list's level scale and
/// level-difference mode to the two addresses.
const EXTRA_LEVEL_MODIFIERS: u32 = 0x0042_14f0;
/// `00406cc0` (`cdecl`, one float word): its result in `ST0`.
const LEVEL_ROUND: u32 = 0x0040_6cc0;
/// `_ftol2` (`00ec62c0`): `ST0` to an integer.
const FTOL: u32 = 0x00ec_62c0;

/// `FISTP dword` with the rounding mode set to truncate (`OR 0xc00`): NaN and
/// values out of range give the integer indefinite `0x8000_0000`.
fn fistp_truncate(value: f64) -> u32 {
    if value > -2_147_483_649.0 && value < 2_147_483_648.0 {
        value.trunc() as i32 as u32
    } else {
        0x8000_0000
    }
}

// Translated from 00487620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (folded under a library name): the
/// `LEVELED_OBJECT` destructor body, frees `pItemExtra` (`00401030`).
pub fn fn_00487620(e: &mut Engine, this: Ptr<LeveledObject>) {
    let extra = e.get(this, LeveledObject::pItemExtra);
    e.call(FREE, &args![extra]);
}

// Translated from 00487740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears `cChanceNone`, `cLLFlags` and
/// `pChanceGlobal`.
pub fn fn_00487740(e: &mut Engine, this: Ptr<TESLeveledList>) {
    e.set(this, TESLeveledList::cChanceNone, 0);
    e.set(this, TESLeveledList::cLLFlags, 0);
    e.set(this, TESLeveledList::pChanceGlobal, 0);
}

// Translated from 00487650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESLeveledList` constructor (unnamed in the engine map; the vtable it
/// sets is `TESLeveledList`'s by its RTTI): the `BaseFormComponent` base
/// (`00402d30`), the vtable, both lists (`0096a2d0` at +4 and +0x14), then the
/// fields cleared (`00487740`). Answers `this`.
// Translation incomplete: 00487650 - the SEH frame (FS:[0], unwind of the two lists) is not translated.
pub fn fn_00487650(e: &mut Engine, this: Ptr<TESLeveledList>) -> Ptr<TESLeveledList> {
    e.call(BASE_FORM_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_LEVELED_LIST_VTABLE);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![this.addr().wrapping_add(4)]);
    e.call(
        SIMPLE_LIST_CONSTRUCT,
        &args![this.addr().wrapping_add(0x14)],
    );
    fn_00487740(e, this);
    this
}

// Translated from 00487900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the leveled list. Walks `leveledList`
/// (`00717e50`): a node with an item has the `LEVELED_OBJECT` deleted
/// (`00481760` with 1) and its head removed (`0063f7b0`; the walk stays on
/// that node), an empty item moves to the next node. Then
/// `ScriptAddedList` (+0x14) is emptied (`00470470`).
pub fn fn_00487900(e: &mut Engine, this: Ptr<TESLeveledList>) {
    let mut node = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            e.call(LEVELED_OBJECT_DELETE, &args![item, 1u32]);
            e.call(LIST_REMOVE_HEAD, &args![node]);
        } else {
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    e.call(
        SIMPLE_LIST_REMOVE_ALL,
        &args![this.addr().wrapping_add(0x14)],
    );
}

// Translated from 00487770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00487900` (empties both lists).
pub fn fn_00487770(e: &mut Engine, this: Ptr<TESLeveledList>) {
    fn_00487900(e, this);
}

// Translated from 004876d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESLeveledList` destructor body (unnamed in the engine map): sets the
/// vtable, empties the lists (`00487770`), then destroys `ScriptAddedList`
/// and `leveledList` (`0046ffb0`).
// Translation incomplete: 004876d0 - the SEH frame (FS:[0], unwind of the two lists) is not translated.
pub fn fn_004876d0(e: &mut Engine, this: Ptr<TESLeveledList>) {
    e.mem.set_u32(this.addr(), TES_LEVELED_LIST_VTABLE);
    fn_00487770(e, this);
    e.call(SIMPLE_LIST_DESTRUCT, &args![this.addr().wrapping_add(0x14)]);
    e.call(SIMPLE_LIST_DESTRUCT, &args![this.addr().wrapping_add(4)]);
}

// Translated from 00487790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The chance of none (unnamed in the engine map): with a `pChanceGlobal`,
/// the global's value (`00526ac0`) truncated to an integer (`FISTP` with the
/// rounding mode set to chop), low byte; otherwise `cChanceNone`.
pub fn fn_00487790(e: &mut Engine, this: Ptr<TESLeveledList>) -> u8 {
    let global = e.get(this, TESLeveledList::pChanceGlobal);
    if global != 0 {
        let value = e.call(GLOBAL_VALUE, &args![global]).f64();
        fistp_truncate(value) as u8
    } else {
        e.get(this, TESLeveledList::cChanceNone)
    }
}

// Translated from 004877e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 0 of `cLLFlags` is set.
pub fn fn_004877e0(e: &mut Engine, this: Ptr<TESLeveledList>) -> bool {
    e.get(this, TESLeveledList::cLLFlags) & 1 != 0
}

// Translated from 00487840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 1 of `cLLFlags` is set.
pub fn fn_00487840(e: &mut Engine, this: Ptr<TESLeveledList>) -> bool {
    e.get(this, TESLeveledList::cLLFlags) & 2 != 0
}

// Translated from 004878a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 2 of `cLLFlags` is set.
pub fn fn_004878a0(e: &mut Engine, this: Ptr<TESLeveledList>) -> bool {
    e.get(this, TESLeveledList::cLLFlags) & 4 != 0
}

// Translated from 00487ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LEVELED_OBJECT` constructor (unnamed in the engine map): no form, no
/// extra, count 1, level 1. Answers `this`.
pub fn fn_00487ba0(e: &mut Engine, this: Ptr<LeveledObject>) -> Ptr<LeveledObject> {
    e.set(this, LeveledObject::pForm, 0);
    e.set(this, LeveledObject::pItemExtra, 0);
    e.set(this, LeveledObject::sCount, 1);
    e.set(this, LeveledObject::sLevel, 1);
    this
}

// Translated from 00487a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a `LEVELED_OBJECT` to `leveledList` in
/// level order (`thiscall`, `RET 0x10`: level, count, form, extra; level and
/// count are 16-bit). The walk stops at the first item above `level` (empty
/// items are skipped, the last node with an item is remembered). The new
/// object (12 bytes, `00401000`, constructed by `00487ba0`; a failed
/// allocation leaves null, which the exe then writes through) takes the
/// form, level, count and extra, and goes before the stopping node
/// (`005ae3d0`), after the remembered node (`00905820`), or at the end of the
/// list (`00905820` on `00717e50`'s answer). Answers the new object.
// Translation incomplete: 00487a60 - the SEH frame (FS:[0], unwind of the allocation) is not translated.
pub fn fn_00487a60(
    e: &mut Engine,
    this: Ptr<TESLeveledList>,
    level: u16,
    count: u16,
    form: u32,
    extra: u32,
) -> u32 {
    let mut node = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
    let mut last = 0;
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            if e.get(Ptr::<LeveledObject>::new(item), LeveledObject::sLevel) > level {
                break;
            }
            last = node;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let object = if block != 0 {
        fn_00487ba0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let object_ptr = Ptr::<LeveledObject>::new(object);
    e.set(object_ptr, LeveledObject::pForm, form);
    e.set(object_ptr, LeveledObject::sLevel, level);
    e.set(object_ptr, LeveledObject::sCount, count);
    e.set(object_ptr, LeveledObject::pItemExtra, extra);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), object);
        if node != 0 {
            e.call(LIST_INSERT_BEFORE, &args![node, slot]);
        } else if last != 0 {
            e.call(LIST_APPEND, &args![last, slot]);
        } else {
            let head = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
            e.call(LIST_APPEND, &args![head, slot]);
        }
        e.mem.u32(slot.addr())
    })
}

// Translated from 00487a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a script's addition to a leveled list
/// (`thiscall`, `RET 0x14`: the form that owns the list, level, count, form,
/// extra). Adds the object (`00487a60`); when one was made, it also goes
/// first in `ScriptAddedList` (`005ae3d0` on +0x14) and the owner flags the
/// change (vtable `+0x48`, `TESForm::AddChange` in the Xbox PDB, with
/// `0x80000000`).
pub fn fn_00487a00(
    e: &mut Engine,
    this: Ptr<TESLeveledList>,
    owner: Ptr,
    level: u16,
    count: u16,
    form: u32,
    extra: u32,
) {
    let object = fn_00487a60(e, this, level, count, form, extra);
    if object != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), object);
            e.call(
                LIST_INSERT_BEFORE,
                &args![this.addr().wrapping_add(0x14), slot],
            );
        });
        e.vcall(
            owner.addr(),
            ADD_CHANGE_SLOT,
            &args![CHANGE_LEVELED_LIST_ADDED],
        );
    }
}

// Translated from 00487be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the leveled list inside a
/// form, at `form + 0x30`, for form types 0x2c, 0x2d, 0x34 and 0x4c (the
/// exe's switch, through its byte map and jump table); null for any other
/// type or a null form.
pub fn fn_00487be0(e: &mut Engine, form: Ptr) -> u32 {
    let mut list = 0;
    if !form.is_null() {
        let form_type = e.call(FORM_TYPE, &args![form]).u32();
        if let 0x00 | 0x01 | 0x08 | 0x20 = form_type.wrapping_sub(0x2c) {
            list = form.addr().wrapping_add(0x30);
        }
    }
    list
}

// Translated from 00487d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: picks one entry of the leveled list for
/// `level` (`thiscall`, `RET 0x18`: level (16-bit), the addresses that
/// receive the form, the count (16-bit) and the extra, the recursion flag
/// (a byte), the level-difference mode). The outputs start cleared. A chance
/// of none above 0 (vtable `+0x10`) wins when a random number (`00487f50`)
/// modulo 100 is below it. The lowest level wanted is `level`, or, when bit 0
/// of the flags is set and the mode is 0 or the mode is above 0,
/// `level - GetMaxLevelDifference()` (vtable `+0x18`; -1 when that is 0).
/// The walk (stopping at the first item above `level`) keeps the node of
/// the highest level seen so far until one at or above the lowest is found,
/// and counts the items from it on. A random number modulo that count picks
/// the entry from that node on (none if the list ends first). When the
/// entry's form holds a leveled list (`00487be0`) and the flag is set, the
/// pick recurses into it: its form is taken, the counts multiply (16-bit)
/// and a non-null extra replaces the entry's.
pub fn fn_00487d00(
    e: &mut Engine,
    this: Ptr<TESLeveledList>,
    level: u16,
    form_out: Ptr,
    count_out: Ptr,
    extra_out: Ptr,
    recurse: u8,
    mode: i32,
) {
    e.mem.set_u32(form_out.addr(), 0);
    e.mem.set_u16(count_out.addr(), 0);
    e.mem.set_u32(extra_out.addr(), 0);
    let chance_none = e.vcall(this.addr(), GET_CHANCE_NONE_SLOT, &args![]).u8();
    if i32::from(chance_none) > 0 {
        let roll = e.call(RANDOM, &args![]).u32() % 100;
        if (roll as i32) < i32::from(chance_none) {
            return;
        }
    }
    let mut lowest = i32::from(level);
    if (fn_004877e0(e, this) && mode == 0) || mode > 0 {
        let difference = e
            .vcall(this.addr(), GET_MAX_LEVEL_DIFFERENCE_SLOT, &args![])
            .i32();
        lowest = if difference != 0 {
            i32::from(level).wrapping_sub(difference)
        } else {
            -1
        };
    }
    let mut same_count: u32 = 1;
    let mut best_level: i32 = 0;
    let mut best_node: u32 = 0;
    let mut reached = false;
    let mut node = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            let item_level =
                i32::from(e.get(Ptr::<LeveledObject>::new(item), LeveledObject::sLevel));
            if item_level > i32::from(level) {
                break;
            }
            if item_level > best_level && (best_level == 0 || !reached) {
                if item_level >= lowest {
                    reached = true;
                }
                same_count = 1;
                best_node = node;
                best_level = item_level;
            } else {
                same_count = same_count.wrapping_add(1);
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if best_node == 0 {
        return;
    }
    let mut steps = e.call(RANDOM, &args![]).u32() % same_count;
    let mut node = best_node;
    while steps != 0 {
        node = e.call(NODE_NEXT, &args![node]).u32();
        steps = steps.wrapping_sub(1);
        if node == 0 {
            return;
        }
    }
    let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
    let item = Ptr::<LeveledObject>::new(e.mem.u32(item_address));
    let mut form = e.get(item, LeveledObject::pForm);
    let mut count = e.get(item, LeveledObject::sCount);
    let mut extra = e.get(item, LeveledObject::pItemExtra);
    let inner = fn_00487be0(e, Ptr::new(e.get(item, LeveledObject::pForm)));
    if inner != 0 && recurse != 0 {
        e.with_stack(0xc, |e, locals| {
            let base = locals.addr();
            fn_00487d00(
                e,
                Ptr::new(inner),
                level,
                Ptr::new(base),
                Ptr::new(base + 4),
                Ptr::new(base + 8),
                recurse,
                mode,
            );
            form = e.mem.u32(base);
            count = e.mem.u16(base + 4).wrapping_mul(count);
            let inner_extra = e.mem.u32(base + 8);
            if inner_extra != 0 {
                extra = inner_extra;
            }
        });
    }
    e.mem.set_u32(form_out.addr(), form);
    e.mem.set_u16(count_out.addr(), count);
    e.mem.set_u32(extra_out.addr(), extra);
}

/// One picked entry of [`fn_00487f70`]: a form holding a leveled list
/// recurses with the caller's level into `container`; any other form whose
/// vtable `+0xe4` answers true is added to it (`004818e0`). Nothing for a
/// null form or a count of 0.
fn add_entry(e: &mut Engine, level: u16, container: u32, entry: (u32, u16, u32), mode: i32) {
    let (form, count, extra) = entry;
    if form == 0 || count == 0 {
        return;
    }
    let inner = fn_00487be0(e, Ptr::new(form));
    if inner != 0 {
        fn_00487f70(e, Ptr::new(inner), level, count, Ptr::new(container), mode);
    } else if e.vcall(form, FORM_CAN_BE_ADDED_SLOT, &args![]).bool() && form != 0 {
        e.call(
            CONTAINER_ADD,
            &args![container, form, u32::from(count), extra],
        );
    }
}

/// One pick of the leveled list (`00487d00`, without recursion) into three
/// stack locals; answers form, count and extra.
fn pick_entry(e: &mut Engine, this: Ptr<TESLeveledList>, level: u16, mode: i32) -> (u32, u16, u32) {
    e.with_stack(0xc, |e, locals| {
        let base = locals.addr();
        fn_00487d00(
            e,
            this,
            level,
            Ptr::new(base),
            Ptr::new(base + 4),
            Ptr::new(base + 8),
            0,
            mode,
        );
        (e.mem.u32(base), e.mem.u16(base + 4), e.mem.u32(base + 8))
    })
}

// Translated from 00487f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: puts the leveled list's result for `level`
/// into a container (`thiscall`, `RET 0x10`: level, count (both 16-bit), the
/// container, the level-difference mode). Level 0 counts as 1; nothing for
/// no container or count 0. With bit 0 of the flags and mode 0, or a mode
/// above 0, the level used for the pick is at most the highest item level
/// in the list. A temporary `TESContainer` (`00481610`) collects: with
/// `GetMultCalc` (vtable `+0x14`) one pick (`00487d00`, no recursion) per
/// count; otherwise, with bit 2 of the flags, every item when a random
/// number modulo 100 is not below the chance of none (vtable `+0x10`), or
/// else one pick, and then the counts multiply by `count` (`00482040`).
/// Entries holding a leveled list recurse with the (at least 1) level. The
/// temporary goes into the container (`00481fd0`) and is destroyed
/// (`00481680`).
// Translation incomplete: 00487f70 - the SEH frame (FS:[0], unwind of the temporary TESContainer) is not translated.
pub fn fn_00487f70(
    e: &mut Engine,
    this: Ptr<TESLeveledList>,
    level: u16,
    count: u16,
    container: Ptr,
    mode: i32,
) {
    let level = if i32::from(level) < 1 { 1 } else { level };
    if container.is_null() || count == 0 {
        return;
    }
    let mut pick_level = u32::from(level);
    if (fn_004877e0(e, this) && mode == 0) || mode > 0 {
        let mut highest: u16 = 0;
        let mut node = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let item = Ptr::<LeveledObject>::new(e.mem.u32(item_address));
            let item_level = e.get(item, LeveledObject::sLevel);
            if item_level > highest {
                highest = item_level;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        if u32::from(highest) < pick_level {
            pick_level = u32::from(highest);
        }
    }
    let pick_level = pick_level as u16;
    e.with_stack(0xc, |e, temporary| {
        let temporary = temporary.addr();
        e.call(CONTAINER_CONSTRUCT, &args![temporary]);
        if e.vcall(this.addr(), GET_MULT_CALC_SLOT, &args![]).bool() {
            let mut index: i32 = 0;
            while index < i32::from(count) {
                let entry = pick_entry(e, this, pick_level, mode);
                add_entry(e, level, temporary, entry, mode);
                index = index.wrapping_add(1);
            }
        } else {
            if fn_004878a0(e, this) {
                let chance_none = e.vcall(this.addr(), GET_CHANCE_NONE_SLOT, &args![]).u8();
                let roll = e.call(RANDOM, &args![]).u32() % 100;
                if roll as i32 >= i32::from(chance_none) {
                    let mut node = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
                    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
                        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
                        let item = Ptr::<LeveledObject>::new(e.mem.u32(item_address));
                        node = e.call(NODE_NEXT, &args![node]).u32();
                        let entry = (
                            e.get(item, LeveledObject::pForm),
                            e.get(item, LeveledObject::sCount),
                            e.get(item, LeveledObject::pItemExtra),
                        );
                        add_entry(e, level, temporary, entry, mode);
                    }
                }
            } else {
                let entry = pick_entry(e, this, pick_level, mode);
                add_entry(e, level, temporary, entry, mode);
            }
            e.call(CONTAINER_MULTIPLY, &args![temporary, u32::from(count)]);
        }
        e.call(CONTAINER_MERGE, &args![container, temporary]);
        e.call(CONTAINER_DESTRUCT, &args![temporary]);
    });
}

// Translated from 00488350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a container from the leveled list for a
/// reference (`thiscall`, `RET 8`: the reference, the container). Nothing
/// without either. The reference's extra list (`005d43c0`) gives a level
/// scale (starting at 1.0) and a level-difference mode (starting at 0,
/// `004214f0`); the level is `GetCalcLevel(1)` (`00567e10`) times the scale
/// (x87 `FILD qword`, `FMUL dword`, stored as a float), through `00406cc0`
/// and `_ftol2`, at least 1; then `00487f70` with that level (16-bit), count
/// 1, the container and the mode.
pub fn fn_00488350(e: &mut Engine, this: Ptr<TESLeveledList>, reference: Ptr, container: Ptr) {
    if reference.is_null() || container.is_null() {
        return;
    }
    e.with_stack(8, |e, locals| {
        let scale_at = locals.addr();
        let mode_at = scale_at + 4;
        e.mem.set_u32(scale_at, x87_store32(1.0));
        e.mem.set_u32(mode_at, 0);
        let extra_list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
        e.call(EXTRA_LEVEL_MODIFIERS, &args![extra_list, scale_at, mode_at]);
        let calc_level = e.call(GET_CALC_LEVEL, &args![reference, 1u32]).u32();
        let scaled = x87_store32(x87_mul(f64::from(calc_level), x87_m32(e.mem.u32(scale_at))));
        let rounded = e.call(LEVEL_ROUND, &args![scaled]).f64();
        let mut level = e.call(FTOL, &args![rounded]).i32();
        if level < 1 {
            level = 1;
        }
        let mode = e.mem.i32(mode_at);
        fn_00487f70(e, this, level as u16, 1, container, mode);
    });
}

/// `00905330` (`thiscall`, `RET 4`): removes the item whose word is at the
/// given address from the list.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `004858f0` (`cdecl`, chunk type and a byte): adds a one-byte chunk.
const ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
/// `00485910` (`cdecl`, chunk type and a word): adds a four-byte chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: type, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `0084e3a0` (`thiscall`, no stack word): a form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `00401500` (no parameters): whether saved data is byte-swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1500;
/// `00407a90` and `00401080` (`cdecl`: address and a byte flag): swap the
/// 16-bit and the 32-bit value at the address in place.
const SWAP_16: u32 = 0x0040_7a90;
const SWAP_32: u32 = 0x0040_1080;
/// `0040e6c0` (`thiscall`, no stack word; unit `containeritemextra.cpp` by
/// the engine map): saves a `ContainerItemExtra`.
const ITEM_EXTRA_SAVE: u32 = 0x0040_e6c0;

/// Sets or clears `bit` of `cLLFlags` (+0xd) by the low byte of `value`.
fn set_list_flag(e: &mut Engine, this: Ptr<TESLeveledList>, bit: u8, value: u32) {
    let flags = e.get(this, TESLeveledList::cLLFlags);
    let flags = if value as u8 != 0 {
        flags | bit
    } else {
        flags & !bit
    };
    e.set(this, TESLeveledList::cLLFlags, flags);
}

// Translated from 00487800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (low byte of `value` non-zero) or clears bit 0x01 of `cLLFlags`
/// (`thiscall`, `RET 4`; the engine map gives it no name).
pub fn fn_00487800(e: &mut Engine, this: Ptr<TESLeveledList>, value: u32) {
    set_list_flag(e, this, 0x01, value);
}

// Translated from 00487860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 0x02 of `cLLFlags` (`thiscall`, `RET 4`; the engine
/// map gives it no name).
pub fn fn_00487860(e: &mut Engine, this: Ptr<TESLeveledList>, value: u32) {
    set_list_flag(e, this, 0x02, value);
}

// Translated from 004878c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 0x04 of `cLLFlags` (`thiscall`, `RET 4`; the engine
/// map gives it no name).
pub fn fn_004878c0(e: &mut Engine, this: Ptr<TESLeveledList>, value: u32) {
    set_list_flag(e, this, 0x04, value);
}

// Translated from 00487980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes every entry out of `ScriptAddedList` (+0x14; no stack word; the
/// engine map gives it no name): while that list is not empty (`008256d0`)
/// its head item (`006815c0`) is read and the head removed (`0063f7b0`),
/// the item is removed from `leveledList` (+4, `00905330` with the address
/// of the stack word holding it) and, when non-null, deleted (`00481760`
/// with 1).
pub fn fn_00487980(e: &mut Engine, this: Ptr<TESLeveledList>) {
    let added = this.addr() + 0x14;
    e.with_stack(4, |e, slot| loop {
        if e.call(NODE_IS_EMPTY, &args![added]).bool() {
            break;
        }
        let item_at = e.call(NODE_ITEM_ADDRESS, &args![added]).u32();
        let item = e.mem.u32(item_at);
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_REMOVE_HEAD, &args![added]);
        e.call(LIST_REMOVE_ITEM, &args![this.addr() + 4, slot]);
        let item = e.mem.u32(slot.addr());
        if item != 0 {
            e.call(LEVELED_OBJECT_DELETE, &args![item, 1u32]);
        }
    });
}

// Translated from 00488400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00488400 - bytes 2-3 and 10-11 of each LVLO record are uninitialised stack in the exe; the model's fresh stack block holds zeros there.
/// `TESLeveledList`'s save (no stack word; the engine map gives it no
/// name): `cChanceNone` as chunk `LVLD` and `cLLFlags` as `LVLF`
/// (`004858f0`), the chance global's id as `LVLG` when set (`00485910`);
/// then for each node of `leveledList` (`00717e50`, `00726070`) until a node
/// without an item: a 12-byte `LVLO` record (`00485990`) holding the
/// level (`u16` at +0), the item form's id (`0084e3a0`, at +4) and the
/// count (`u16` at +8), byte-swapped before and after (`00488510`) when
/// `00401500` says so, then the item's extra data saved (`0040e6c0`) when
/// it has one.
pub fn fn_00488400(e: &mut Engine, this: Ptr<TESLeveledList>) {
    let chance = e.get(this, TESLeveledList::cChanceNone);
    e.call(ADD_CHUNK_BYTE, &args![0x444c_564cu32, u32::from(chance)]);
    let flags = e.get(this, TESLeveledList::cLLFlags);
    e.call(ADD_CHUNK_BYTE, &args![0x464c_564cu32, u32::from(flags)]);
    let global = e.get(this, TESLeveledList::pChanceGlobal);
    if global != 0 {
        let id = e.call(FORM_ID, &args![global]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x474c_564cu32, id]);
    }
    let mut node = e.call(LEVELED_LIST_HEAD, &args![this]).u32();
    e.with_stack(0xc, |e, record| {
        let record = record.addr();
        while node != 0 {
            let item_at = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let item = e.mem.u32(item_at);
            if item == 0 {
                break;
            }
            let form = e.mem.u32(item);
            let id = e.call(FORM_ID, &args![form]).u32();
            e.mem.set_u32(record + 4, id);
            let level = e.mem.u16(item + 6);
            e.mem.set_u16(record, level);
            let count = e.mem.u16(item + 4);
            e.mem.set_u16(record + 8, count);
            if e.call(NEEDS_SWAP, &args![]).bool() {
                fn_00488510(e, Ptr::new(record));
            }
            e.call(ADD_CHUNK_DATA, &args![0x4f4c_564cu32, record, 0xcu32]);
            if e.call(NEEDS_SWAP, &args![]).bool() {
                fn_00488510(e, Ptr::new(record));
            }
            let extra = e.mem.u32(item + 8);
            if extra != 0 {
                e.call(ITEM_EXTRA_SAVE, &args![extra]);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 00488510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps an `LVLO` record in place (no stack word; the engine map
/// gives it no name): the 16-bit value at +0 (`00407a90`), the word at +4
/// (`00401080`) and the 16-bit value at +8, each with the flag 0.
pub fn fn_00488510(e: &mut Engine, this: Ptr) {
    e.call(SWAP_16, &args![this, 0u32]);
    e.call(SWAP_32, &args![this.addr().wrapping_add(4), 0u32]);
    e.call(SWAP_16, &args![this.addr().wrapping_add(8), 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_7800, fn_00487800(Ptr<TESLeveledList>, u32)),
        entry!(0x0048_7860, fn_00487860(Ptr<TESLeveledList>, u32)),
        entry!(0x0048_78c0, fn_004878c0(Ptr<TESLeveledList>, u32)),
        entry!(0x0048_7980, fn_00487980(Ptr<TESLeveledList>)),
        entry!(0x0048_8400, fn_00488400(Ptr<TESLeveledList>)),
        entry!(0x0048_8510, fn_00488510(Ptr)),
        entry!(0x00487f50, fn_00487f50() -> u32),
        entry!(0x0048_7620, fn_00487620(Ptr<LeveledObject>)),
        entry!(0x0048_7740, fn_00487740(Ptr<TESLeveledList>)),
        entry!(
            0x0048_7650,
            fn_00487650(Ptr<TESLeveledList>) -> Ptr<TESLeveledList>
        ),
        entry!(0x0048_7900, fn_00487900(Ptr<TESLeveledList>)),
        entry!(0x0048_7770, fn_00487770(Ptr<TESLeveledList>)),
        entry!(0x0048_76d0, fn_004876d0(Ptr<TESLeveledList>)),
        entry!(0x0048_7790, fn_00487790(Ptr<TESLeveledList>) -> u8),
        entry!(0x0048_77e0, fn_004877e0(Ptr<TESLeveledList>) -> bool),
        entry!(0x0048_7840, fn_00487840(Ptr<TESLeveledList>) -> bool),
        entry!(0x0048_78a0, fn_004878a0(Ptr<TESLeveledList>) -> bool),
        entry!(
            0x0048_7ba0,
            fn_00487ba0(Ptr<LeveledObject>) -> Ptr<LeveledObject>
        ),
        entry!(
            0x0048_7a60,
            fn_00487a60(Ptr<TESLeveledList>, u16, u16, u32, u32) -> u32
        ),
        entry!(
            0x0048_7a00,
            fn_00487a00(Ptr<TESLeveledList>, Ptr, u16, u16, u32, u32)
        ),
        entry!(0x0048_7be0, fn_00487be0(Ptr) -> u32),
        entry!(
            0x0048_7d00,
            fn_00487d00(Ptr<TESLeveledList>, u16, Ptr, Ptr, Ptr, u8, i32)
        ),
        entry!(
            0x0048_7f70,
            fn_00487f70(Ptr<TESLeveledList>, u16, u16, Ptr, i32)
        ),
        entry!(0x0048_8350, fn_00488350(Ptr<TESLeveledList>, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_second_call_on_the_first_calls_result() {
        let mut e = Engine::new();
        e.register(GET_STATIC_OBJECT, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(CALL_WITH_MINUS_ONE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0048_7f50, &args![]).u32(), 0x1235);
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    const LIST_VTABLE: u32 = 0x00ab_0000;
    const CHANCE_FN: u32 = 0x00ab_1000;
    const MULT_FN: u32 = 0x00ab_1004;
    const DIFFERENCE_FN: u32 = 0x00ab_1008;
    const FORM_VTABLE: u32 = 0x00ac_0000;
    const CAN_ADD_FN: u32 = 0x00ac_1000;
    const ADD_CHANGE_FN: u32 = 0x00ac_1004;
    /// Where the doubles keep their answers: chance of none, mult calc, max
    /// level difference, and the random numbers to hand out (count first).
    const KNOBS: u32 = 0x00ad_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles that act on two-word list nodes (item, next) as the exe's
    /// bodies do, a list vtable whose slots answer the knobs, and forms
    /// whose type is the byte at +4.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(KNOBS, 0x100);
        e.register(LEVELED_LIST_HEAD, |_, a| (a[0] + 4).into_ret());
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(RANDOM, |e, _| {
            let n = e.mem.u32(KNOBS + 0x10);
            e.mem.set_u32(KNOBS + 0x10, n + 1);
            e.mem.u32(KNOBS + 0x14 + 4 * n).into_ret()
        });
        e.register(CHANCE_FN, |e, _| u32::from(e.mem.u8(KNOBS)).into_ret());
        e.register(MULT_FN, |e, _| u32::from(e.mem.u8(KNOBS + 4)).into_ret());
        e.register(DIFFERENCE_FN, |e, _| e.mem.u32(KNOBS + 8).into_ret());
        e.register(CAN_ADD_FN, |_, _| 1u32.into_ret());
        e.register(ADD_CHANGE_FN, |_, _| Ret::default());
        let mut slots = vec![0u32; 8];
        slots[4] = CHANCE_FN;
        slots[5] = MULT_FN;
        slots[6] = DIFFERENCE_FN;
        e.put_vtable(LIST_VTABLE, &slots);
        let mut slots = vec![0u32; 0x3a];
        slots[0x48 / 4] = ADD_CHANGE_FN;
        slots[0xe4 / 4] = CAN_ADD_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        for addr in [
            LIST_INSERT_BEFORE,
            LIST_APPEND,
            LEVELED_OBJECT_DELETE,
            CONTAINER_CONSTRUCT,
            CONTAINER_DESTRUCT,
            CONTAINER_ADD,
            CONTAINER_MULTIPLY,
            CONTAINER_MERGE,
            SIMPLE_LIST_CONSTRUCT,
            SIMPLE_LIST_DESTRUCT,
            SIMPLE_LIST_REMOVE_ALL,
            BASE_FORM_COMPONENT_CONSTRUCT,
            FREE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn randoms(e: &mut Engine, values: &[u32]) {
        e.mem.set_u32(KNOBS + 0x10, 0);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(KNOBS + 0x14 + 4 * i as u32, *v);
        }
    }

    fn form(e: &mut Engine, form_type: u8) -> u32 {
        let f = e.mem.alloc(0x50);
        e.mem.set_u32(f, FORM_VTABLE);
        e.mem.set_u8(f + 4, form_type);
        f
    }

    fn object(e: &mut Engine, form: u32, count: u16, level: u16, extra: u32) -> u32 {
        let o = e.mem.alloc(0xc);
        e.mem.set_u32(o, form);
        e.mem.set_u16(o + 4, count);
        e.mem.set_u16(o + 6, level);
        e.mem.set_u32(o + 8, extra);
        o
    }

    /// A leveled list at `at` whose `leveledList` holds `items` (node chain
    /// starting in the list itself).
    fn fill(e: &mut Engine, at: u32, items: &[u32], flags: u8) -> Ptr<TESLeveledList> {
        e.mem.set_u32(at, LIST_VTABLE);
        e.mem.set_u8(at + 0xd, flags);
        let mut node = at + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        Ptr::new(at)
    }

    fn list(e: &mut Engine, items: &[u32], flags: u8) -> Ptr<TESLeveledList> {
        let at = e.mem.alloc(0x1c);
        fill(e, at, items, flags)
    }

    #[test]
    fn fn_00487620_frees_the_extra() {
        let mut e = engine();
        let o = object(&mut e, 0, 1, 1, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0048_7620, &args![o]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x77]]);
    }

    #[test]
    fn fn_00487650_constructs_and_00487740_clears() {
        let mut e = engine();
        let l = e.mem.alloc(0x1c);
        e.mem.write(l, &[0xff; 0x1c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_7650, &args![l]).u32(), l);
        assert_eq!(e.mem.u32(l), TES_LEVELED_LIST_VTABLE);
        assert_eq!(calls_to(&e, BASE_FORM_COMPONENT_CONSTRUCT), vec![vec![l]]);
        assert_eq!(
            calls_to(&e, SIMPLE_LIST_CONSTRUCT),
            vec![vec![l + 4], vec![l + 0x14]]
        );
        assert_eq!(e.mem.u8(l + 0xc), 0);
        assert_eq!(e.mem.u8(l + 0xd), 0);
        assert_eq!(e.mem.u32(l + 0x10), 0);
        e.mem.set_u32(l + 0x10, 5);
        e.call(0x0048_7740, &args![l]);
        assert_eq!(e.mem.u32(l + 0x10), 0);
    }

    #[test]
    fn fn_00487900_and_its_callers_empty_the_lists() {
        let mut e = engine();
        // The remove-head double pops the head node's item.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        for address in [0x0048_7900u32, 0x0048_7770, 0x0048_76d0] {
            let l = list(&mut e, &[0x100, 0x200], 0);
            e.call_log = Some(vec![]);
            e.call(address, &args![l]);
            assert_eq!(
                calls_to(&e, LEVELED_OBJECT_DELETE),
                vec![vec![0x100, 1], vec![0x200, 1]]
            );
            assert_eq!(
                calls_to(&e, SIMPLE_LIST_REMOVE_ALL),
                vec![vec![l.addr() + 0x14]]
            );
            if address == 0x0048_76d0 {
                assert_eq!(e.mem.u32(l.addr()), TES_LEVELED_LIST_VTABLE);
                assert_eq!(
                    calls_to(&e, SIMPLE_LIST_DESTRUCT),
                    vec![vec![l.addr() + 0x14], vec![l.addr() + 4]]
                );
            }
        }
        // An empty item moves on to the next node.
        let l = list(&mut e, &[0, 0x300], 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_7900, &args![l]);
        assert_eq!(calls_to(&e, LEVELED_OBJECT_DELETE), vec![vec![0x300, 1]]);
    }

    #[test]
    fn fn_00487790_reads_the_global_truncated_or_the_byte() {
        let mut e = engine();
        let l = list(&mut e, &[], 0);
        e.mem.set_u8(l.addr() + 0xc, 33);
        assert_eq!(e.call(0x0048_7790, &args![l]).u32() & 0xff, 33);
        e.mem.set_u32(l.addr() + 0x10, 0x4000);
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: 57.9,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0048_7790, &args![l]).u32() & 0xff, 57);
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: -1.5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0048_7790, &args![l]).u32() & 0xff, 0xff);
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: f64::NAN,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0048_7790, &args![l]).u32() & 0xff, 0);
    }

    #[test]
    fn flag_bits_0_1_and_2() {
        let mut e = engine();
        for (flags, expected) in [
            (1u8, [true, false, false]),
            (6, [false, true, true]),
            (0xf8, [false; 3]),
        ] {
            let l = list(&mut e, &[], flags);
            let got =
                [0x0048_77e0u32, 0x0048_7840, 0x0048_78a0].map(|a| e.call(a, &args![l]).bool());
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn fn_00487ba0_constructs_a_leveled_object() {
        let mut e = engine();
        let o = e.mem.alloc(0xc);
        e.mem.write(o, &[0xee; 0xc]);
        assert_eq!(e.call(0x0048_7ba0, &args![o]).u32(), o);
        assert_eq!(
            (
                e.mem.u32(o),
                e.mem.u16(o + 4),
                e.mem.u16(o + 6),
                e.mem.u32(o + 8)
            ),
            (0, 1, 1, 0)
        );
    }

    #[test]
    fn fn_00487a60_inserts_in_level_order() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let low = object(&mut e, 1, 1, 2, 0);
        let high = object(&mut e, 2, 1, 9, 0);
        let l = list(&mut e, &[low, 0, high], 0);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x0048_7a60, &args![l, 5u32, 3u32, 0x777u32, 0x888u32])
            .u32();
        assert_eq!(
            (
                e.mem.u32(made),
                e.mem.u16(made + 4),
                e.mem.u16(made + 6),
                e.mem.u32(made + 8)
            ),
            (0x777, 3, 5, 0x888)
        );
        let third = e.mem.u32(e.mem.u32(l.addr() + 8) + 4);
        assert_eq!(calls_to(&e, LIST_INSERT_BEFORE).len(), 1);
        assert_eq!(calls_to(&e, LIST_INSERT_BEFORE)[0][0], third);
        // Above all: after the last node with an item.
        e.call_log = Some(vec![]);
        e.call(0x0048_7a60, &args![l, 10u32, 1u32, 0u32, 0u32]);
        assert_eq!(calls_to(&e, LIST_APPEND)[0][0], third);
        // An empty list: appended to the head.
        let l = list(&mut e, &[], 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_7a60, &args![l, 10u32, 1u32, 0u32, 0u32]);
        assert_eq!(calls_to(&e, LIST_APPEND)[0][0], l.addr() + 4);
    }

    #[test]
    fn fn_00487a00_also_lists_the_script_addition_and_flags_the_owner() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let l = list(&mut e, &[], 0);
        let owner = form(&mut e, 0x2c);
        e.call_log = Some(vec![]);
        e.call(0x0048_7a00, &args![l, owner, 4u32, 2u32, 0x55u32, 0u32]);
        let inserts = calls_to(&e, LIST_INSERT_BEFORE);
        assert_eq!(inserts.len(), 1);
        assert_eq!(inserts[0][0], l.addr() + 0x14);
        assert_eq!(calls_to(&e, ADD_CHANGE_FN), vec![vec![owner, 0x8000_0000]]);
    }

    #[test]
    fn fn_00487be0_finds_leveled_lists_of_four_types() {
        let mut e = engine();
        for t in 0..=0xffu8 {
            let f = form(&mut e, t);
            let expected = if matches!(t, 0x2c | 0x2d | 0x34 | 0x4c) {
                f + 0x30
            } else {
                0
            };
            assert_eq!(
                e.call(0x0048_7be0, &args![f]).u32(),
                expected,
                "type {t:#x}"
            );
        }
        assert_eq!(e.call(0x0048_7be0, &args![0u32]).u32(), 0);
    }

    fn pick(
        e: &mut Engine,
        l: Ptr<TESLeveledList>,
        level: u32,
        recurse: u32,
        mode: i32,
    ) -> (u32, u16, u32) {
        let out = e.mem.alloc(0xc);
        e.mem.write(out, &[0xcc; 0xc]);
        e.call(
            0x0048_7d00,
            &args![l, level, out, out + 4, out + 8, recurse, mode],
        );
        (e.mem.u32(out), e.mem.u16(out + 4), e.mem.u32(out + 8))
    }

    #[test]
    fn fn_00487d00_picks_by_level_chance_and_recursion() {
        let mut e = engine();
        let [fa, fb, fc, fd] = [0u8; 4].map(|_| form(&mut e, 0x10));
        let a = object(&mut e, fa, 1, 1, 0);
        let b = object(&mut e, fb, 2, 3, 0xb0);
        let c = object(&mut e, fc, 3, 3, 0);
        let d = object(&mut e, fd, 4, 8, 0);
        let l = list(&mut e, &[a, b, c, d], 0);
        // Chance of none 30: a roll of 129 (29) gives nothing.
        e.mem.set_u8(KNOBS, 30);
        randoms(&mut e, &[129]);
        assert_eq!(pick(&mut e, l, 5, 0, 0), (0, 0, 0));
        // A roll of 30 passes; level 5: the highest level 3 has two items; 1 picks c.
        randoms(&mut e, &[30, 1]);
        assert_eq!(pick(&mut e, l, 5, 0, 0), (fc, 3, 0));
        e.mem.set_u8(KNOBS, 0);
        randoms(&mut e, &[2]);
        assert_eq!(pick(&mut e, l, 5, 0, 0), (fb, 2, 0xb0));
        // Level 0: nothing at or below it.
        randoms(&mut e, &[0]);
        assert_eq!(pick(&mut e, l, 0, 0, 0), (0, 0, 0));
        // With a level difference of 4 (mode 1), level 5 wants 1 or more: a
        // reaches it, so b and c only count after it.
        e.mem.set_u32(KNOBS + 8, 4);
        randoms(&mut e, &[1]);
        assert_eq!(pick(&mut e, l, 5, 0, 1), (fb, 2, 0xb0));
        // Recursion into a form that holds a leveled list.
        let inner_form = form(&mut e, 0x2d);
        let f99 = form(&mut e, 0x10);
        let inner_item = object(&mut e, f99, 5, 1, 0x91);
        fill(&mut e, inner_form + 0x30, &[inner_item], 0);
        let outer_item = object(&mut e, inner_form, 3, 1, 0x50);
        let outer = list(&mut e, &[outer_item], 0);
        randoms(&mut e, &[0, 0]);
        assert_eq!(pick(&mut e, outer, 1, 1, 0), (f99, 15, 0x91));
        randoms(&mut e, &[0]);
        assert_eq!(pick(&mut e, outer, 1, 0, 0), (inner_form, 3, 0x50));
    }

    #[test]
    fn fn_00487f70_fills_a_container() {
        let mut e = engine();
        let item = form(&mut e, 0x10);
        let a = object(&mut e, item, 2, 1, 0x70);
        let l = list(&mut e, &[a], 0);
        let container = e.mem.alloc(0xc);
        // No container or count 0: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0048_7f70, &args![l, 1u32, 0u32, container, 0u32]);
        e.call(0x0048_7f70, &args![l, 1u32, 1u32, 0u32, 0u32]);
        assert!(calls_to(&e, CONTAINER_CONSTRUCT).is_empty());
        // One pick, multiplied by the count, merged and destroyed.
        randoms(&mut e, &[0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_7f70, &args![l, 0u32, 3u32, container, 0u32]);
        let temporary = calls_to(&e, CONTAINER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, CONTAINER_ADD),
            vec![vec![temporary, item, 2, 0x70]]
        );
        assert_eq!(calls_to(&e, CONTAINER_MULTIPLY), vec![vec![temporary, 3]]);
        assert_eq!(
            calls_to(&e, CONTAINER_MERGE),
            vec![vec![container, temporary]]
        );
        assert_eq!(calls_to(&e, CONTAINER_DESTRUCT), vec![vec![temporary]]);
        // Mult calc: one pick per count, no multiply.
        e.mem.set_u8(KNOBS + 4, 1);
        randoms(&mut e, &[0, 0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_7f70, &args![l, 1u32, 3u32, container, 0u32]);
        assert_eq!(calls_to(&e, CONTAINER_ADD).len(), 3);
        assert!(calls_to(&e, CONTAINER_MULTIPLY).is_empty());
        e.mem.set_u8(KNOBS + 4, 0);
        // Use all (bit 2): every item when the roll is not below the chance.
        let b = object(&mut e, item, 1, 50, 0);
        let all = list(&mut e, &[a, b], 4);
        e.mem.set_u8(KNOBS, 10);
        randoms(&mut e, &[10]);
        e.call_log = Some(vec![]);
        e.call(0x0048_7f70, &args![all, 1u32, 1u32, container, 0u32]);
        assert_eq!(calls_to(&e, CONTAINER_ADD).len(), 2);
        randoms(&mut e, &[9]);
        e.call_log = Some(vec![]);
        e.call(0x0048_7f70, &args![all, 1u32, 1u32, container, 0u32]);
        assert!(calls_to(&e, CONTAINER_ADD).is_empty());
        e.mem.set_u8(KNOBS, 0);
        // A nested leveled list recurses into the temporary with the level.
        let inner_form = form(&mut e, 0x34);
        fill(&mut e, inner_form + 0x30, &[a], 0);
        let nested = object(&mut e, inner_form, 2, 1, 0);
        let outer = list(&mut e, &[nested], 0);
        randoms(&mut e, &[0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_7f70, &args![outer, 7u32, 1u32, container, 0u32]);
        let temporaries = calls_to(&e, CONTAINER_CONSTRUCT);
        assert_eq!(temporaries.len(), 2);
        assert_eq!(
            calls_to(&e, CONTAINER_MERGE)[0],
            vec![temporaries[0][0], temporaries[1][0]]
        );
        assert_eq!(
            calls_to(&e, CONTAINER_MULTIPLY),
            vec![vec![temporaries[1][0], 2], vec![temporaries[0][0], 1]]
        );
    }

    #[test]
    fn fn_00488350_scales_the_reference_level() {
        let mut e = engine();
        let item = form(&mut e, 0x10);
        let a = object(&mut e, item, 1, 1, 0);
        let l = list(&mut e, &[a], 0);
        let reference = e.mem.alloc(0x60);
        let container = e.mem.alloc(0xc);
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_LEVEL_MODIFIERS, |e, a| {
            e.mem.set_f32(a[1], 0.5);
            e.mem.set_u32(a[2], 0);
            Ret::default()
        });
        e.register(GET_CALC_LEVEL, |_, _| 7u32.into_ret());
        e.register(LEVEL_ROUND, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).ceil(),
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        randoms(&mut e, &[0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_8350, &args![l, 0u32, container]);
        assert!(calls_to(&e, GET_CALC_LEVEL).is_empty());
        e.call(0x0048_8350, &args![l, reference, container]);
        assert_eq!(calls_to(&e, EXTRA_LEVEL_MODIFIERS)[0][0], reference + 0x44);
        assert_eq!(calls_to(&e, GET_CALC_LEVEL), vec![vec![reference, 1]]);
        assert_eq!(calls_to(&e, LEVEL_ROUND), vec![vec![3.5f32.to_bits()]]);
        assert_eq!(calls_to(&e, CONTAINER_ADD).len(), 1);
        // A level below 1 becomes 1.
        e.register(GET_CALC_LEVEL, |_, _| 0u32.into_ret());
        randoms(&mut e, &[0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_8350, &args![l, reference, container]);
        assert_eq!(calls_to(&e, CONTAINER_ADD).len(), 1);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> Log {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn flag_setters_set_and_clear_their_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        for (addr, bit) in [(0x0048_7800, 1u8), (0x0048_7860, 2), (0x0048_78c0, 4)] {
            e.mem.set_u8(this + 0xd, 0x80);
            e.call(addr, &args![this, 0x100u32 | 1]);
            assert_eq!(e.mem.u8(this + 0xd), 0x80 | bit);
            // Only the low byte counts.
            e.call(addr, &args![this, 0x100u32]);
            assert_eq!(e.mem.u8(this + 0xd), 0x80);
            e.mem.set_u8(this + 0xd, 0xff);
            e.call(addr, &args![this, 0u32]);
            assert_eq!(e.mem.u8(this + 0xd), !bit);
        }
    }

    #[test]
    fn at_00487980_drains_the_script_added_list() {
        let mut e = Engine::new();
        // The list at +0x14 is a count at +0x14 and items at +0x18, +0x1c;
        // the doubles pop from the end.
        e.register(NODE_IS_EMPTY, |e, a| word(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(NODE_ITEM_ADDRESS, |e, a| {
            let n = e.mem.u32(a[0]);
            word(a[0] + 4 * n)
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let n = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], n - 1);
            Ret::default()
        });
        e.register(LIST_REMOVE_ITEM, |_, _| Ret::default());
        e.register(LEVELED_OBJECT_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x18, 0x5100);
        e.mem.set_u32(this + 0x1c, 0);
        let log = run(&mut e, 0x0048_7980, &[this]);
        let removed: Vec<_> = log.iter().filter(|c| c.0 == LIST_REMOVE_ITEM).collect();
        assert_eq!(removed.len(), 2);
        assert_eq!(removed[0].1[0], this + 4);
        let deleted: Vec<_> = log
            .iter()
            .filter(|c| c.0 == LEVELED_OBJECT_DELETE)
            .cloned()
            .collect();
        assert_eq!(deleted, vec![(LEVELED_OBJECT_DELETE, vec![0x5100, 1])]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn at_00488400_saves_header_and_records() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 8);
        e.register(ADD_CHUNK_BYTE, |_, _| Ret::default());
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        e.register(FORM_ID, |_, a| word(a[0] + 1));
        e.register(NEEDS_SWAP, |e, _| word(e.mem.u32(0x0f00_0000)));
        e.register(SWAP_16, |_, _| Ret::default());
        e.register(SWAP_32, |_, _| Ret::default());
        e.register(ITEM_EXTRA_SAVE, |_, _| Ret::default());
        // The record's bytes at the time of the chunk call.
        e.register(ADD_CHUNK_DATA, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            let at = 0x0f00_0004;
            let n = e.mem.u32(at);
            e.mem.set_u32(at, n + 1);
            if n == 0 {
                assert_eq!(bytes, vec![5, 0, 0, 0, 0x01, 0x70, 0, 0, 2, 0, 0, 0]);
            }
            Ret::default()
        });
        // Nodes: a node is [item, next].
        e.register(LEVELED_LIST_HEAD, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(NODE_ITEM_ADDRESS, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        let this = e.mem.alloc(0x1c);
        e.mem.set_u8(this + 0xc, 0x10);
        e.mem.set_u8(this + 0xd, 0x3);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, 0x7000);
        e.mem.set_u16(item + 4, 2);
        e.mem.set_u16(item + 6, 5);
        let empty = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, empty);
        e.mem.set_u32(this + 4, node);
        let log = run(&mut e, 0x0048_8400, &[this]);
        assert_eq!(log[0], (ADD_CHUNK_BYTE, vec![0x444c_564c, 0x10]));
        assert_eq!(log[1], (ADD_CHUNK_BYTE, vec![0x464c_564c, 3]));
        assert_eq!(log[2], (LEVELED_LIST_HEAD, vec![this]));
        assert!(!log.iter().any(|c| c.0 == ADD_CHUNK_WORD || c.0 == SWAP_16));
        assert!(!log.iter().any(|c| c.0 == ITEM_EXTRA_SAVE));
        assert_eq!(e.mem.u32(0x0f00_0004), 1);
        let chunk = log.iter().find(|c| c.0 == ADD_CHUNK_DATA).unwrap();
        assert_eq!((chunk.1[0], chunk.1[2]), (0x4f4c_564c, 0xc));
        // With a global, swapping and an extra.
        e.mem.set_u32(this + 0x10, 0x9000);
        e.mem.set_u32(item + 8, 0x6000);
        e.mem.set_u32(0x0f00_0000, 1);
        let log = run(&mut e, 0x0048_8400, &[this]);
        assert_eq!(log[3], (ADD_CHUNK_WORD, vec![0x474c_564c, 0x9001]));
        assert_eq!(log.iter().filter(|c| c.0 == SWAP_32).count(), 2);
        assert!(log.contains(&(ITEM_EXTRA_SAVE, vec![0x6000])));
    }

    #[test]
    fn at_00488510_swaps_the_three_fields() {
        let mut e = Engine::new();
        e.register(SWAP_16, |_, _| Ret::default());
        e.register(SWAP_32, |_, _| Ret::default());
        let log = run(&mut e, 0x0048_8510, &[0x5000]);
        assert_eq!(
            log,
            vec![
                (SWAP_16, vec![0x5000, 0]),
                (SWAP_32, vec![0x5004, 0]),
                (SWAP_16, vec![0x5008, 0]),
            ]
        );
    }
}
