//! `(unplaced)/run_004887a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Copies the save buffer's change flags (the word at +0x17 of the
/// buffer) into a local (`00428110`) and tests a mask on them
/// (`004280f0`).
const COPY_CHANGE_FLAGS: u32 = 0x0042_8110;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `BGSSaveGameBuffer`: the byte save (`00865e50`), `SaveFormID_ov2`
/// (`00865df0`, two words), `StartVariableSizedValue` (`00865f20`) and
/// `SaveVariableSizedValue_ov2` (`00865ff0`).
const SAVE_BYTES: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const FINISH_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// List helpers: the item pointer's address of a node (`006815c0`) and the
/// next node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `BSFaceGenKeyframe::GetTime` (`00488d50`, engine map; thiscall, the
/// time in ST0).
const KEYFRAME_GET_TIME: u32 = 0x0048_8d50;
/// The `float` saved when an entry has no keyframe (`01012054`, -1.0).
const NO_TIME: u32 = 0x0101_2054;
/// The list whose first node sits at +0x14 of `this`.
const LIST_14: u32 = 0x14;
/// An entry: its form (+0), two 16-bit fields (+4, +6) and its keyframe
/// (+8).
const ENTRY_FIELD_4: u32 = 4;
const ENTRY_FIELD_6: u32 = 6;
const ENTRY_KEYFRAME: u32 = 8;

// Translated from 00488c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the list at +0x14 into the save buffer when its change flags have
/// bit 31 (thiscall, `RET 4`; unnamed in the engine map): a counted list
/// (variable-sized count patched in at the end) of the non-null entries,
/// each its form id (`SaveFormID_ov2(form, 0)`), the 16-bit fields at +6
/// and +4, and a `float` time: -1.0 (`01012054`) or, with a keyframe at +8,
/// its time (`00488d50`, stored as a `float`).
pub fn fn_00488c50(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let wanted = e.with_stack(4, |e, local| {
        let flags = e.call(COPY_CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, 0x8000_0000u32]).u32() & 0xff != 0
    });
    if !wanted {
        return;
    }
    let mut count = 0u32;
    let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = this.addr() + LIST_14;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            let form = e.mem.u32(entry);
            e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
            e.call(
                SAVE_BYTES,
                &args![buffer, entry + ENTRY_FIELD_6, 2u32, 0u32],
            );
            e.call(
                SAVE_BYTES,
                &args![buffer, entry + ENTRY_FIELD_4, 2u32, 0u32],
            );
            let mut time = x87_quiet_bits(e.global::<u32>(NO_TIME));
            let keyframe = e.mem.u32(entry + ENTRY_KEYFRAME);
            if keyframe != 0 {
                let value = e.call(KEYFRAME_GET_TIME, &args![keyframe]).f64();
                time = x87_store32(value);
            }
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), time);
                e.call(SAVE_BYTES, &args![buffer, local, 4u32, 0u32]);
            });
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(FINISH_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVBaseFormComponent@@` and `.?AVTESLeveledList@@`.
const TYPE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_LEVELED_LIST: u32 = 0x0118_6c1c;
/// `00487900` (`ECX` = the leveled list): empties its list.
const CLEAR_LIST_00487900: u32 = 0x0048_7900;
/// `00717e50` (`ECX` = the leveled list): the first node of its list.
const LIST_HEAD_00717E50: u32 = 0x0071_7e50;
/// `008256d0` (`ECX` = a node): whether the list is empty (`AL`).
const NODE_EMPTY_008256D0: u32 = 0x0082_56d0;
/// The allocator (`00401000`, `cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
/// `00487ba0` (`ECX`): constructs an entry (0xc bytes).
const ENTRY_CONSTRUCT_00487BA0: u32 = 0x0048_7ba0;
/// `0040e9c0` (`ECX` = an entry's extra data): whether it holds nothing (`AL`).
const EXTRA_EMPTY_0040E9C0: u32 = 0x0040_e9c0;
/// `0040e690` (`ECX`): `ContainerItemExtra::ContainerItemExtra` (engine map), 0xc bytes.
const EXTRA_CONSTRUCT_0040E690: u32 = 0x0040_e690;
/// `0040e980` (`ECX` = the new extra data, one stack word: the source).
const EXTRA_COPY_0040E980: u32 = 0x0040_e980;
/// `0040e920` (`ECX` = an extra data, one stack word: another), `AL`: whether they differ.
const EXTRA_DIFFER_0040E920: u32 = 0x0040_e920;
/// `005ae3d0` and `00905820` (`ECX` = a node, one stack word: the address of an item
/// word): store the item into an empty list, and insert it after the node.
const LIST_SET_FIRST_005AE3D0: u32 = 0x005a_e3d0;
const LIST_INSERT_AFTER_00905820: u32 = 0x0090_5820;
/// The leveled list's flag getters `004877e0` and `004878a0` (`ECX`, `AL`) and setters
/// `0042e680`, `00487800`, `00487860` and `004878c0` (`ECX`, one byte word).
const FLAG_GET_004877E0: u32 = 0x0048_77e0;
const FLAG_GET_004878A0: u32 = 0x0048_78a0;
const FLAG_SET_0042E680: u32 = 0x0042_e680;
const FLAG_SET_00487800: u32 = 0x0048_7800;
const FLAG_SET_00487860: u32 = 0x0048_7860;
const FLAG_SET_004878C0: u32 = 0x0048_78c0;

/// The source cast from `BaseFormComponent` to `TESLeveledList`, or null.
fn as_leveled_list(e: &mut Engine, component: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![
            component,
            0u32,
            TYPE_FORM_COMPONENT,
            TYPE_LEVELED_LIST,
            0u32
        ],
    )
    .u32()
}

/// Constructs an object of 0xc bytes with `constructor`, or answers null when the
/// allocation fails.
fn construct(e: &mut Engine, constructor: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(constructor, &args![block]).u32()
    }
}

// Translated from 004887a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004887a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eef206, state variable around the two constructors) and the frame's cookie word are not translated.
/// `TESLeveledList`'s `CopyComponent` slot (+8 of the `BaseFormComponent` vtables at
/// `0101cd04`, `0102862c`, ...; unnamed in the engine map; `ECX` and one stack word,
/// `RET 4`): when `source` casts to a `TESLeveledList`, empties this list (`00487900`) and
/// appends a copy of each of the source's entries up to the first null one: a new entry
/// (`00487ba0`, 0xc bytes) with the form word and the 16-bit fields at +6 and +4, and,
/// when the source entry has an extra data at +8 that `0040e9c0` does not call empty, a
/// new `ContainerItemExtra` (`0040e690`) copied from it (`0040e980`). The first entry goes
/// in through `005ae3d0` while this list is empty, the others after the last one
/// (`00905820`). Then the word at +0x10 and the four flags (vtable slots +0x10 and +0x14,
/// `004877e0`, `004878a0`) are copied.
pub fn fn_004887a0(e: &mut Engine, this: Ptr, source: Ptr) {
    let list = as_leveled_list(e, source);
    if list == 0 {
        return;
    }
    e.call(CLEAR_LIST_00487900, &args![this]);
    let mut from = e.call(LIST_HEAD_00717E50, &args![list]).u32();
    let mut to = e.call(LIST_HEAD_00717E50, &args![this]).u32();
    e.with_stack(4, |e, copy_slot| {
        while from != 0 {
            let item = e.call(NODE_ITEM, &args![from]).u32();
            let entry = e.mem.u32(item);
            if entry == 0 {
                break;
            }
            let copy = construct(e, ENTRY_CONSTRUCT_00487BA0);
            e.mem.set_u32(copy_slot.addr(), copy);
            let copy = e.mem.u32(copy_slot.addr());
            let form = e.mem.u32(entry);
            e.mem.set_u32(copy, form);
            let field = e.mem.u16(entry.wrapping_add(6));
            e.mem.set_u16(copy.wrapping_add(6), field);
            let field = e.mem.u16(entry.wrapping_add(4));
            e.mem.set_u16(copy.wrapping_add(4), field);
            let extra = e.mem.u32(entry.wrapping_add(8));
            if extra != 0 && e.call(EXTRA_EMPTY_0040E9C0, &args![extra]).u8() == 0 {
                let new_extra = construct(e, EXTRA_CONSTRUCT_0040E690);
                e.mem.set_u32(copy.wrapping_add(8), new_extra);
                let extra = e.mem.u32(entry.wrapping_add(8));
                let new_extra = e.mem.u32(copy.wrapping_add(8));
                e.call(EXTRA_COPY_0040E980, &args![new_extra, extra]);
            }
            let head = e.call(LIST_HEAD_00717E50, &args![this]).u32();
            if e.call(NODE_EMPTY_008256D0, &args![head]).u8() != 0 {
                e.call(LIST_SET_FIRST_005AE3D0, &args![to, copy_slot]);
            } else {
                e.call(LIST_INSERT_AFTER_00905820, &args![to, copy_slot]);
                to = e.call(NODE_NEXT, &args![to]).u32();
            }
            from = e.call(NODE_NEXT, &args![from]).u32();
        }
    });
    let word = e.mem.u32(list + 0x10);
    e.mem.set_u32(this.addr() + 0x10, word);
    let flag = e.vcall(list, 0x10, &args![]).u8();
    e.call(FLAG_SET_0042E680, &args![this, u32::from(flag)]);
    let flag = e.call(FLAG_GET_004877E0, &args![list]).u8();
    e.call(FLAG_SET_00487800, &args![this, u32::from(flag)]);
    let flag = e.vcall(list, 0x14, &args![]).u8();
    e.call(FLAG_SET_00487860, &args![this, u32::from(flag)]);
    let flag = e.call(FLAG_GET_004878A0, &args![list]).u8();
    e.call(FLAG_SET_004878C0, &args![this, u32::from(flag)]);
}

// Translated from 004889d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLeveledList`'s `CompareComponent` slot (+0xc of the same vtables; unnamed in the
/// engine map; `ECX` and one stack word, `RET 4`): true (different) when `other` is not a
/// `TESLeveledList`, when a flag (`004877e0`, vtable slots +0x14 and +0x10, `004878a0`, in
/// that order) or the word at +0x10 differs, when one list is empty (`008256d0`) and the
/// other not, or when walking both lists side by side (while both nodes are non-null) a
/// pair of non-null entries differs in form, the 16-bit field at +6 or +4, or extra data:
/// only one has one and `0040e9c0` does not call it empty, or both have one and
/// `0040e920` answers true. Lists of different lengths differ too. False otherwise.
pub fn fn_004889d0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let list = as_leveled_list(e, other);
    if list == 0 {
        return true;
    }
    let mine = e.call(FLAG_GET_004877E0, &args![this]).u8();
    if mine != e.call(FLAG_GET_004877E0, &args![list]).u8() {
        return true;
    }
    for slot in [0x14u32, 0x10] {
        let mine = e.vcall(this.addr(), slot, &args![]).u8();
        if mine != e.vcall(list, slot, &args![]).u8() {
            return true;
        }
    }
    let mine = e.call(FLAG_GET_004878A0, &args![this]).u8();
    if mine != e.call(FLAG_GET_004878A0, &args![list]).u8() {
        return true;
    }
    if e.mem.u32(this.addr() + 0x10) != e.mem.u32(list + 0x10) {
        return true;
    }
    let mut node = e.call(LIST_HEAD_00717E50, &args![this]).u32();
    let mut other_node = e.call(LIST_HEAD_00717E50, &args![list]).u32();
    let empty = e.call(NODE_EMPTY_008256D0, &args![node]).u8();
    if empty != e.call(NODE_EMPTY_008256D0, &args![other_node]).u8() {
        return true;
    }
    if e.call(NODE_EMPTY_008256D0, &args![node]).u8() != 0 {
        return e.call(NODE_EMPTY_008256D0, &args![other_node]).u8() == 0;
    }
    while node != 0 && other_node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        let item = e.call(NODE_ITEM, &args![other_node]).u32();
        let other_entry = e.mem.u32(item);
        if entry != 0 && other_entry != 0 {
            if e.mem.u32(entry) != e.mem.u32(other_entry) {
                return true;
            }
            if e.mem.u16(entry + 6) != e.mem.u16(other_entry + 6) {
                return true;
            }
            if e.mem.u16(entry + 4) != e.mem.u16(other_entry + 4) {
                return true;
            }
            let extra = e.mem.u32(entry + 8);
            let other_extra = e.mem.u32(other_entry + 8);
            if extra != 0
                && other_extra == 0
                && e.call(EXTRA_EMPTY_0040E9C0, &args![extra]).u8() == 0
            {
                return true;
            }
            let extra = e.mem.u32(entry + 8);
            let other_extra = e.mem.u32(other_entry + 8);
            if extra == 0
                && other_extra != 0
                && e.call(EXTRA_EMPTY_0040E9C0, &args![other_extra]).u8() == 0
            {
                return true;
            }
            let extra = e.mem.u32(entry + 8);
            let other_extra = e.mem.u32(other_entry + 8);
            if extra != 0
                && other_extra != 0
                && e.call(EXTRA_DIFFER_0040E920, &args![extra, other_extra])
                    .u8()
                    != 0
            {
                return true;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
        other_node = e.call(NODE_NEXT, &args![other_node]).u32();
    }
    node != 0 || other_node != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_87a0, fn_004887a0(Ptr, Ptr)),
        entry!(0x0048_89d0, fn_004889d0(Ptr, Ptr) -> bool),
        entry!(0x0048_8c50, fn_00488c50(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORD: u32 = 0x0342_0000;

    fn setup(e: &mut Engine, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 0x17, flags);
        e.register(COPY_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(FLAGS_TEST, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| 0x40u32.into_ret());
        e.register(FINISH_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(KEYFRAME_GET_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.map(NO_TIME, 4);
        e.mem.set_u32(NO_TIME, (-1.0f32).to_bits());
        e.map(RECORD, 0x100);
        e.register(SAVE_BYTES, |e, a| {
            let used = e.mem.u32(RECORD);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(RECORD + 4 + used, &bytes);
            e.mem.set_u32(RECORD, used + a[2]);
            Ret::default()
        });
        buffer
    }

    #[test]
    fn saves_each_entry_with_its_time() {
        let mut e = Engine::new();
        let buffer = setup(&mut e, 0x8000_0000);
        let with_keyframe = e.mem.alloc(12);
        e.mem.set_u32(with_keyframe, 0x0700_0001);
        e.mem.set_u16(with_keyframe + 4, 0x0102);
        e.mem.set_u16(with_keyframe + 6, 0x0304);
        e.mem.set_u32(with_keyframe + 8, 0x0800_0000);
        let without = e.mem.alloc(12);
        e.mem.set_u32(without, 0x0700_0002);
        e.mem.set_u16(without + 4, 0x0506);
        e.mem.set_u16(without + 6, 0x0708);
        let null_node = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_u32(last, without);
        e.mem.set_u32(null_node + 4, last);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + LIST_14, with_keyframe);
        e.mem.set_u32(this + LIST_14 + 4, null_node);
        e.call_log = Some(vec![]);
        e.call(0x0048_8c50, &args![this, buffer]);
        let mut expected = vec![0x04, 0x03, 0x02, 0x01];
        expected.extend(2.5f32.to_le_bytes());
        expected.extend([0x08, 0x07, 0x06, 0x05]);
        expected.extend((-1.0f32).to_le_bytes());
        assert_eq!(e.mem.bytes(RECORD + 4, e.mem.u32(RECORD)), expected);
        let log = e.call_log.take().unwrap();
        let forms: Vec<_> = log.iter().filter(|(a, _)| *a == SAVE_FORM_ID).collect();
        assert_eq!(forms[0].1, vec![buffer, 0x0700_0001, 0]);
        assert_eq!(forms[1].1, vec![buffer, 0x0700_0002, 0]);
        let finish: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == FINISH_VARIABLE_SIZED_VALUE)
            .collect();
        assert_eq!(finish[0].1, vec![buffer, 2, 0x40]);
    }

    #[test]
    fn saves_nothing_without_the_flag() {
        let mut e = Engine::new();
        let buffer = setup(&mut e, 0x7fff_ffff);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x0048_8c50, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == START_VARIABLE_SIZED_VALUE));
        assert_eq!(e.mem.u32(RECORD), 0);
    }
}

#[cfg(test)]
mod leveled_list_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Lists are nodes of [item, next]; a list's head node sits at +0x20 of its object.
    /// The flag getters answer the object's byte at +0x30 (all four), the vtable slots
    /// +0x10 and +0x14 too.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(LIST_HEAD_00717E50, |_, a| (a[0] + 0x20).into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_EMPTY_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        for getter in [
            FLAG_GET_004877E0,
            FLAG_GET_004878A0,
            0x0060_0010,
            0x0060_0014,
        ] {
            e.register(getter, |e, a| u32::from(e.mem.u8(a[0] + 0x30)).into_ret());
        }
        e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0x0060_0010, 0x0060_0014]);
        e.register(EXTRA_EMPTY_0040E9C0, |_, _| 0u32.into_ret());
        e.register(EXTRA_DIFFER_0040E920, |_, a| {
            u32::from(a[0] != a[1]).into_ret()
        });
        e
    }

    fn list(e: &mut Engine, entries: &[u32], flag: u8) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x0070_0000);
        e.mem.set_u8(object + 0x30, flag);
        let mut node = object + 0x20;
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(node, *entry);
            if i + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        object
    }

    fn entry(e: &mut Engine, form: u32, a: u16, b: u16, extra: u32) -> u32 {
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, form);
        e.mem.set_u16(entry + 4, a);
        e.mem.set_u16(entry + 6, b);
        e.mem.set_u32(entry + 8, extra);
        entry
    }

    #[test]
    fn fn_004887a0_copies_entries_and_flags() {
        let mut e = engine();
        let with_extra = entry(&mut e, 0x111, 1, 2, 0x9000);
        let plain = entry(&mut e, 0x222, 3, 4, 0);
        let source = list(&mut e, &[with_extra, plain], 1);
        e.mem.set_u32(source + 0x10, 0x4444);
        let this = list(&mut e, &[], 0);
        let blocks = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = blocks.clone();
        e.register_double(ALLOCATE, move |e, _| {
            let block = e.mem.alloc(0xc);
            seen.borrow_mut().push(block);
            block.into_ret()
        });
        e.register(ENTRY_CONSTRUCT_00487BA0, |_, a| a[0].into_ret());
        e.register(EXTRA_CONSTRUCT_0040E690, |_, a| a[0].into_ret());
        for callee in [
            CLEAR_LIST_00487900,
            EXTRA_COPY_0040E980,
            FLAG_SET_0042E680,
            FLAG_SET_00487800,
            FLAG_SET_00487860,
            FLAG_SET_004878C0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        // The first insert fills the empty head; the next goes after it.
        e.register(LIST_SET_FIRST_005AE3D0, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(LIST_INSERT_AFTER_00905820, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_87a0, &args![this, source]);
        let log = e.call_log.take().unwrap();
        let blocks = blocks.borrow().clone();
        assert_eq!(blocks.len(), 3);
        let (first, extra, second) = (blocks[0], blocks[1], blocks[2]);
        assert_eq!(e.mem.u32(first), 0x111);
        assert_eq!(e.mem.u16(first + 4), 1);
        assert_eq!(e.mem.u16(first + 6), 2);
        assert_eq!(e.mem.u32(first + 8), extra);
        assert_eq!(e.mem.u32(second), 0x222);
        assert_eq!(calls(&log, EXTRA_COPY_0040E980), vec![vec![extra, 0x9000]]);
        assert_eq!(calls(&log, LIST_SET_FIRST_005AE3D0).len(), 1);
        assert_eq!(calls(&log, LIST_INSERT_AFTER_00905820).len(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0x4444);
        for setter in [
            FLAG_SET_0042E680,
            FLAG_SET_00487800,
            FLAG_SET_00487860,
            FLAG_SET_004878C0,
        ] {
            assert_eq!(calls(&log, setter), vec![vec![this, 1]]);
        }
    }

    #[test]
    fn fn_004887a0_ignores_other_components() {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0048_87a0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_004889d0_compares_flags_and_entries() {
        let mut e = engine();
        let a = entry(&mut e, 0x111, 1, 2, 0);
        let b = entry(&mut e, 0x111, 1, 2, 0);
        let c = entry(&mut e, 0x111, 1, 3, 0);
        let d = entry(&mut e, 0x111, 1, 2, 0x9000);
        let one = list(&mut e, &[a], 0);
        let same = list(&mut e, &[b], 0);
        assert!(!e.call(0x0048_89d0, &args![one, same]).bool());
        let field = list(&mut e, &[c], 0);
        assert!(e.call(0x0048_89d0, &args![one, field]).bool());
        let with_extra = list(&mut e, &[d], 0);
        assert!(e.call(0x0048_89d0, &args![one, with_extra]).bool());
        let longer = list(&mut e, &[b, b], 0);
        assert!(e.call(0x0048_89d0, &args![one, longer]).bool());
        let flagged = list(&mut e, &[b], 1);
        assert!(e.call(0x0048_89d0, &args![one, flagged]).bool());
        e.mem.set_u32(same + 0x10, 5);
        assert!(e.call(0x0048_89d0, &args![one, same]).bool());
        let empty = list(&mut e, &[], 0);
        let empty2 = list(&mut e, &[], 0);
        assert!(e.call(0x0048_89d0, &args![one, empty]).bool());
        assert!(!e.call(0x0048_89d0, &args![empty, empty2]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0048_89d0, &args![one, same]).bool());
    }
}
