//! `fallout shared/run_005ff2e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ff880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `005fe130`. `005fe130` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_005ff880(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005f_e130, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

/// `00440d80` (`ECX` = form): a byte; true makes `005ff2e0` give 0.
const FORM_00440D80: u32 = 0x0044_0d80;
/// `00436aa0` (`ECX` = idle): its conditions.
const CONDITIONS: u32 = 0x0043_6aa0;
/// `TESCondition::IsTrue` (Xbox PDB name of `00680c30`; `ECX` =
/// conditions; two words; `RET 8`): a byte.
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;
/// `TESIdleForm::CheckIdleFile` (Xbox PDB name of `005ff280`; `ECX` = idle).
const CHECK_IDLE_FILE: u32 = 0x005f_f280;
/// `005ff1d0` (`ECX` = idle): bit 7 of the anim data byte.
const IDLE_005FF1D0: u32 = 0x005f_f1d0;
/// `008256d0` (`ECX` = list node): a byte; true at the end of the list.
const NODE_IS_END: u32 = 0x0082_56d0;
/// `006815c0` (`ECX` = list node): the address of its item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = list node): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `LowProcess::GetGenericLocation` (engine map name of `008041a0`; `ECX` =
/// idle): the word at +0x44 (`pParentIdle`, Xbox PDB).
const IDLE_PARENT: u32 = 0x0080_41a0;
/// `0084e3a0` (`ECX` = array): the word at +0xc, its count.
const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// `NiFormArray::GetFormAt` (Xbox PDB name of `004d6170`; `ECX` = array, an
/// index; `RET 4`).
const FORM_AT: u32 = 0x004d_6170;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` and `TESIdleForm` type descriptors.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_IDLE_FORM: u32 = 0x0118_6a18;
/// `00944460` (cdecl, two words).
const RANGE_00944460: u32 = 0x0094_4460;
/// `0044a670` (cdecl: string): its length.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `00406d30` (cdecl: destination, size, source): copies the string.
const STRING_COPY: u32 = 0x0040_6d30;
/// `BSStringT<char>::StrCmp` (Xbox PDB name of `00408a80`; `ECX` = string;
/// characters, a word; `RET 8`): 0 when equal.
const STRING_COMPARE: u32 = 0x0040_8a80;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets it.
const STRING_SET: u32 = 0x0040_37f0;

// Translated from 005ff2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the idle to play (`thiscall`; two words for the conditions and a
/// list; `RET 0xc`; the engine map gives it no name). 0 when `00440d80`
/// reports true or the conditions (`TESCondition::IsTrue` with the two
/// words) fail. Otherwise the idle itself is the candidate when its file
/// is an idle file (`CheckIdleFile`) and it is not the item of a node of
/// the list (walked while `008256d0` reports false), or, when the file
/// check fails, when `005ff1d0` reports true. Then the first child
/// (`005ff4a0`, `005ff4d0`) whose own search gives an idle wins over the
/// candidate.
pub fn fn_005ff2e0(e: &mut Engine, this: Ptr, first: u32, second: u32, list: u32) -> u32 {
    let mut result = 0u32;
    if e.call(FORM_00440D80, &args![this]).bool() {
        return result;
    }
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    if !e
        .call(CONDITION_IS_TRUE, &args![conditions, first, second])
        .bool()
    {
        return result;
    }
    if e.call(CHECK_IDLE_FILE, &args![this]).bool() {
        let mut node = list;
        let mut found = false;
        while node != 0 && !found && !e.call(NODE_IS_END, &args![node]).bool() {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let entry = e.mem.u32(item);
            if e.mem.u32(entry) == this.addr() {
                found = true;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        if !found {
            result = this.addr();
        }
    } else if e.call(IDLE_005FF1D0, &args![this]).bool() {
        result = this.addr();
    }
    let count = fn_005ff4a0(e, this);
    if count != 0 {
        let mut chosen = 0u32;
        let mut index = 0u32;
        while index < count {
            let child = fn_005ff4d0(e, this, index);
            if child != 0 {
                chosen = fn_005ff2e0(e, Ptr::new(child), first, second, list);
            }
            if chosen != 0 {
                break;
            }
            index += 1;
        }
        if chosen != 0 {
            result = chosen;
        }
    }
    result
}

// Translated from 005ff430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the idle's conditions hold (`TESCondition::IsTrue` with the two
/// words; `RET 0xc`); with the flag byte and a parent (`008041a0`), the
/// parent's answer (asked the same way) instead of true. The engine map
/// gives it no name.
pub fn fn_005ff430(e: &mut Engine, this: Ptr, first: u32, second: u32, flag: u8) -> bool {
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    if !e
        .call(CONDITION_IS_TRUE, &args![conditions, first, second])
        .bool()
    {
        return false;
    }
    if flag != 0 && e.call(IDLE_PARENT, &args![this]).u32() != 0 {
        let parent = e.call(IDLE_PARENT, &args![this]).u32();
        return fn_005ff430(e, Ptr::new(parent), first, second, flag);
    }
    true
}

// Translated from 005ff4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The idle's child count: the count (`0084e3a0`) of its child array
/// (+0x40, `pChildIdles` in the Xbox PDB), 0 without one. The engine map
/// gives it no name.
pub fn fn_005ff4a0(e: &mut Engine, this: Ptr) -> u32 {
    let children = e.mem.u32(this.addr() + 0x40);
    if children == 0 {
        return 0;
    }
    e.call(ARRAY_COUNT, &args![children]).u32()
}

// Translated from 005ff4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The idle's child at an index (`RET 4`): the child array's form there
/// (`NiFormArray::GetFormAt`) cast to `TESIdleForm`, 0 without an array.
/// The engine map gives it no name.
pub fn fn_005ff4d0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let children = e.mem.u32(this.addr() + 0x40);
    if children == 0 {
        return 0;
    }
    let form = e.call(FORM_AT, &args![children, index]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_IDLE_FORM, 0u32],
    )
    .u32()
}

// Translated from 005ff770 (decompiled, FalloutNV.exe 1.4.0.525)
/// From the bytes at +0x39 and +0x3a (`cLoopMin`, `cLoopMax` in the Xbox
/// PDB's `IDLE_DATA`): 0 when either is 0; 0xff when the first is 0xff;
/// `00944460(first, second) - 1` when the second is the larger; otherwise
/// the second minus one. The engine map gives it no name.
pub fn fn_005ff770(e: &mut Engine, this: Ptr) -> u8 {
    let low = e.mem.u8(this.addr() + 0x39);
    let high = e.mem.u8(this.addr() + 0x3a);
    if low == 0 || high == 0 {
        return 0;
    }
    if low == 0xff {
        return low;
    }
    if high > low {
        let value = e
            .call(RANGE_00944460, &args![u32::from(low), u32::from(high)])
            .u32();
        return value.wrapping_sub(1) as u8;
    }
    high.wrapping_sub(1)
}

// Translated from 005ff7f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ff7f0 - the security-cookie check (00ec408c) is not translated.
/// Sets the string at +0x4c to the text given (one word; `RET 4`) unless it
/// already equals it: the text is copied (`00406d30`) into a stack buffer
/// of its length plus one (`0044a670`; the `_alloca` rounds it to 16),
/// compared (`BSStringT::StrCmp` with 1), and set (`004037f0` with 0) when
/// different. True when it was set. The engine map gives it no name.
pub fn fn_005ff7f0(e: &mut Engine, this: Ptr, text: u32) -> bool {
    let size = e.call(STRING_LENGTH, &args![text]).u32().wrapping_add(1);
    e.with_stack(size.wrapping_add(15) & !15, |e, buffer| {
        e.call(STRING_COPY, &args![buffer, size, text]);
        let string = this.addr() + 0x4c;
        if e.call(STRING_COMPARE, &args![string, buffer, 1u32]).u32() == 0 {
            return false;
        }
        e.call(STRING_SET, &args![string, buffer, 0u32]);
        true
    })
}

/// `NiFormArray::GetIndexForForm` (engine map name of `004d6100`; `ECX` =
/// array, the form; `RET 4`): the index, -1 when absent.
const INDEX_FOR_FORM: u32 = 0x004d_6100;
/// `00486b80` (`ECX` = array, an index; `RET 4`): removes that element;
/// `0061bdc0` (`ECX` = array) runs after each removal.
const ARRAY_REMOVE: u32 = 0x0048_6b80;
const ARRAY_0061BDC0: u32 = 0x0061_bdc0;
/// `LowProcess::SetGenericLocation` (engine map name of `008d8040`) and
/// `LowProcess::SetSecondGenericLocation` (`00673400`): set the idle's
/// parent (+0x44) and previous idle (+0x48); `006733e0` reads the
/// previous one. All thiscall on the idle.
const SET_IDLE_PARENT: u32 = 0x008d_8040;
const SET_IDLE_PREVIOUS: u32 = 0x0067_3400;
const IDLE_PREVIOUS: u32 = 0x0067_33e0;
/// The idle manager (`[011cb6a0]`) and its root and loose idle arrays for
/// an idle (`006005c0`, `00600690`; thiscall, the idle).
const IDLE_MANAGER: u32 = 0x011c_b6a0;
const ROOT_IDLE_ARRAY: u32 = 0x0060_05c0;
const LOOSE_IDLE_ARRAY: u32 = 0x0060_0690;

// Translated from 005ff520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of `form` in the idle's child array (+0x40) through
/// `NiFormArray::GetIndexForForm` (`RET 4`), -1 without an array. The
/// engine map gives it no name.
pub fn fn_005ff520(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    let children = e.mem.u32(this.addr() + 0x40);
    if children == 0 {
        return 0xffff_ffff;
    }
    e.call(INDEX_FOR_FORM, &args![children, form]).u32()
}

// Translated from 005ff560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the child at `index` (`RET 4`; the engine map has no name):
/// with a child array, the next child ([`fn_005ff4d0`] at `index + 1`)
/// takes the one before (`index - 1`) as its previous idle (`00673400`)
/// when there is a next one, then the element is removed (`00486b80`) and
/// `0061bdc0` runs on the array.
pub fn fn_005ff560(e: &mut Engine, this: Ptr, index: u32) {
    if e.mem.u32(this.addr() + 0x40) == 0 {
        return;
    }
    let next = fn_005ff4d0(e, this, index.wrapping_add(1));
    let previous = fn_005ff4d0(e, this, index.wrapping_sub(1));
    if next != 0 {
        e.call(SET_IDLE_PREVIOUS, &args![next, previous]);
    }
    let children = e.mem.u32(this.addr() + 0x40);
    e.call(ARRAY_REMOVE, &args![children, index]);
    let children = e.mem.u32(this.addr() + 0x40);
    e.call(ARRAY_0061BDC0, &args![children]);
}

// Translated from 005ff5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the child `form` (`RET 4`; the engine map has no name): with a
/// form and a child array, [`fn_005ff560`] at its index
/// ([`fn_005ff520`]).
pub fn fn_005ff5d0(e: &mut Engine, this: Ptr, form: u32) {
    if form != 0 && e.mem.u32(this.addr() + 0x40) != 0 {
        let index = fn_005ff520(e, this, form);
        fn_005ff560(e, this, index);
    }
}

/// Removes the element at `index` from `array` (`00486b80`), then runs
/// `0061bdc0` on it.
fn remove_at(e: &mut Engine, array: u32, index: u32) {
    e.call(ARRAY_REMOVE, &args![array, index]);
    e.call(ARRAY_0061BDC0, &args![array]);
}

// Translated from 005ff610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the idle out of its array (no stack words; the engine map has no
/// name). A loose idle (`005ff200` answers true) leaves the manager's
/// loose array (`00600690` on `[011cb6a0]`) when found
/// (`NiFormArray::GetIndexForForm`) and is no longer loose
/// (`005ff220(0)`). Otherwise its array is its parent's child array
/// (`008d6f30` on `008041a0`'s answer) or the manager's root array
/// (`006005c0`); when found there it is removed, its parent cleared
/// (`008d8040(0)`), and the element now at its index, when set, takes its
/// previous idle (`006733e0`; through `0084e3a0`, 0 for none, unless
/// `004013e0` answers true for that element) before the idle's own
/// previous idle is cleared (`00673400(0)`).
pub fn fn_005ff610(e: &mut Engine, this: Ptr) {
    let idle = this.addr();
    if e.call(0x005f_f200, &args![idle]).bool() {
        let manager = e.global::<u32>(IDLE_MANAGER);
        let array = e.call(LOOSE_IDLE_ARRAY, &args![manager, idle]).u32();
        let index = e.call(INDEX_FOR_FORM, &args![array, idle]).i32();
        if index != -1 {
            remove_at(e, array, index as u32);
        }
        e.call(0x005f_f220, &args![idle, 0u32]);
        return;
    }
    let array = if e.call(IDLE_PARENT, &args![idle]).u32() != 0 {
        let parent = e.call(IDLE_PARENT, &args![idle]).u32();
        e.call(0x008d_6f30, &args![parent]).u32()
    } else {
        let manager = e.global::<u32>(IDLE_MANAGER);
        e.call(ROOT_IDLE_ARRAY, &args![manager, idle]).u32()
    };
    if array == 0 {
        return;
    }
    let index = e.call(INDEX_FOR_FORM, &args![array, idle]).i32();
    if index == -1 {
        return;
    }
    remove_at(e, array, index as u32);
    e.call(SET_IDLE_PARENT, &args![idle, 0u32]);
    let slot = e.call(0x0087_7a30, &args![array, index as u32]).u32();
    let next = e.mem.u32(slot);
    if next == 0 {
        return;
    }
    if e.call(0x0040_13e0, &args![next]).bool() {
        let previous = e.call(IDLE_PREVIOUS, &args![idle]).u32();
        e.call(SET_IDLE_PREVIOUS, &args![next, previous]);
    } else {
        let previous = e.call(IDLE_PREVIOUS, &args![idle]).u32();
        let id = if previous != 0 {
            e.call(0x0084_e3a0, &args![previous]).u32()
        } else {
            0
        };
        e.call(SET_IDLE_PREVIOUS, &args![next, id]);
    }
    e.call(SET_IDLE_PREVIOUS, &args![idle, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_f520, fn_005ff520(Ptr, u32) -> u32),
        entry!(0x005f_f560, fn_005ff560(Ptr, u32)),
        entry!(0x005f_f5d0, fn_005ff5d0(Ptr, u32)),
        entry!(0x005f_f610, fn_005ff610(Ptr)),
        entry!(0x005f_f880, fn_005ff880(Ptr, u32) -> u32),
        entry!(0x005f_f2e0, fn_005ff2e0(Ptr, u32, u32, u32) -> u32),
        entry!(0x005f_f430, fn_005ff430(Ptr, u32, u32, u8) -> bool),
        entry!(0x005f_f4a0, fn_005ff4a0(Ptr) -> u32),
        entry!(0x005f_f4d0, fn_005ff4d0(Ptr, u32) -> u32),
        entry!(0x005f_f770, fn_005ff770(Ptr) -> u8),
        entry!(0x005f_f7f0, fn_005ff7f0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_005ff880() {
        assert_eq!(
            check(0x005f_f880, 0x005f_e130, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        e
    }

    #[test]
    fn children_through_the_array() {
        let mut e = engine(&[]);
        e.register(ARRAY_COUNT, |_, _| ret(3));
        e.register(FORM_AT, |_, a| ret(0x5000 + a[1]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 0x100));
        let idle = e.mem.alloc(0x54);
        assert_eq!(e.call(0x005f_f4a0, &args![idle]).u32(), 0);
        assert_eq!(e.call(0x005f_f4d0, &args![idle, 1u32]).u32(), 0);
        e.mem.set_u32(idle + 0x40, 0x7000);
        assert_eq!(e.call(0x005f_f4a0, &args![idle]).u32(), 3);
        let (result, log) = run(&mut e, 0x005f_f4d0, &[idle, 1]);
        assert_eq!(result.u32(), 0x5101);
        assert_eq!(calls(&log, FORM_AT), vec![vec![0x7000, 1]]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x5001, 0, TYPE_TES_FORM, TYPE_TES_IDLE_FORM, 0]]
        );
    }

    #[test]
    fn loop_count_from_the_two_bytes() {
        let mut e = engine(&[]);
        e.register(RANGE_00944460, |_, a| ret(a[0] * 0x10 + a[1]));
        let idle = e.mem.alloc(0x54);
        let cases = [
            (0u8, 5u8, 0u8),
            (5, 0, 0),
            (0xff, 3, 0xff),
            (2, 3, 0x22),
            (3, 3, 2),
            (4, 1, 0),
        ];
        for (low, high, expected) in cases {
            e.mem.set_u8(idle + 0x39, low);
            e.mem.set_u8(idle + 0x3a, high);
            assert_eq!(e.call(0x005f_f770, &args![idle]).u8(), expected);
        }
    }

    #[test]
    fn conditions_climb_to_the_parent_with_the_flag() {
        let mut e = engine(&[]);
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x30));
        e.register(CONDITION_IS_TRUE, |_, a| ret(u32::from(a[0] != 0x5030)));
        e.register(IDLE_PARENT, |_, a| {
            ret(if a[0] == 0x4000 { 0x5000 } else { 0 })
        });
        let (result, log) = run(&mut e, 0x005f_f430, &[0x4000, 7, 8, 0]);
        assert!(result.bool());
        assert_eq!(calls(&log, CONDITION_IS_TRUE), vec![vec![0x4030, 7, 8]]);
        assert!(calls(&log, IDLE_PARENT).is_empty());
        let (result, log) = run(&mut e, 0x005f_f430, &[0x4000, 7, 8, 0x101]);
        assert!(!result.bool());
        assert_eq!(
            calls(&log, CONDITION_IS_TRUE),
            vec![vec![0x4030, 7, 8], vec![0x5030, 7, 8]]
        );
        let (result, _) = run(&mut e, 0x005f_f430, &[0x6000, 7, 8, 1]);
        assert!(result.bool());
    }

    #[test]
    fn search_takes_the_idle_unless_it_is_in_the_list() {
        let mut e = engine(&[FORM_00440D80, IDLE_005FF1D0, NODE_IS_END]);
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x30));
        e.register(CONDITION_IS_TRUE, |_, _| ret(1));
        e.register(CHECK_IDLE_FILE, |_, _| ret(1));
        let idle = e.mem.alloc(0x54);
        // A list of one node whose item points at an entry naming the idle.
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, idle);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, entry);
        e.register_double(NODE_ITEM, move |_, _| ret(item));
        e.register(NODE_NEXT, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x005f_f2e0, &[idle, 7, 8, 0x8000]);
        assert_eq!(result.u32(), 0);
        assert_eq!(
            calls(&log, CONDITION_IS_TRUE),
            vec![vec![idle + 0x30, 7, 8]]
        );
        assert_eq!(calls(&log, NODE_NEXT), vec![vec![0x8000]]);
        let (result, _) = run(&mut e, 0x005f_f2e0, &[idle, 7, 8, 0]);
        assert_eq!(result.u32(), idle);
        e.mem.set_u32(entry, 0x1234);
        let (result, _) = run(&mut e, 0x005f_f2e0, &[idle, 7, 8, 0x8000]);
        assert_eq!(result.u32(), idle);
        // Without an idle file, `005ff1d0` decides.
        e.register(CHECK_IDLE_FILE, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x005f_f2e0, &[idle, 7, 8, 0x8000]);
        assert_eq!(result.u32(), 0);
        assert!(calls(&log, NODE_ITEM).is_empty());
        e.register(IDLE_005FF1D0, |_, _| ret(1));
        let (result, _) = run(&mut e, 0x005f_f2e0, &[idle, 7, 8, 0x8000]);
        assert_eq!(result.u32(), idle);
        // `00440d80` or failing conditions give 0 at once.
        e.register(CONDITION_IS_TRUE, |_, _| ret(0));
        assert_eq!(e.call(0x005f_f2e0, &args![idle, 7u32, 8u32, 0u32]).u32(), 0);
        e.register(FORM_00440D80, |_, _| ret(1));
        let (result, log) = run(&mut e, 0x005f_f2e0, &[idle, 7, 8, 0]);
        assert_eq!(result.u32(), 0);
        assert_eq!(log, vec![(FORM_00440D80, vec![idle])]);
    }

    #[test]
    fn search_prefers_the_first_child_that_finds_one() {
        let mut e = engine(&[FORM_00440D80, NODE_IS_END, NODE_NEXT, NODE_ITEM]);
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x30));
        let parent = e.mem.alloc(0x54);
        let first = e.mem.alloc(0x54);
        let second = e.mem.alloc(0x54);
        e.mem.set_u32(parent + 0x40, 0x7000);
        e.register(ARRAY_COUNT, |_, _| ret(3));
        e.register_double(FORM_AT, move |_, a| ret([0, first, second][a[1] as usize]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        // The first child fails its conditions; the second passes.
        e.register_double(CONDITION_IS_TRUE, move |_, a| {
            ret(u32::from(a[0] != first + 0x30))
        });
        e.register_double(IDLE_005FF1D0, move |_, a| ret(u32::from(a[0] == second)));
        e.register(CHECK_IDLE_FILE, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x005f_f2e0, &[parent, 7, 8, 0]);
        assert_eq!(result.u32(), second);
        assert_eq!(calls(&log, FORM_AT).len(), 3);
        assert_eq!(calls(&log, CONDITION_IS_TRUE).len(), 3);
    }

    #[test]
    fn name_is_set_only_when_different() {
        let mut e = engine(&[STRING_COPY, STRING_SET]);
        e.register(STRING_LENGTH, |_, _| ret(0x1f));
        e.register(STRING_COMPARE, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x005f_f7f0, &[0x4000, 0x5000]);
        assert!(!result.bool());
        let copy = calls(&log, STRING_COPY);
        assert_eq!(copy[0][1..], [0x20, 0x5000]);
        assert_eq!(
            calls(&log, STRING_COMPARE),
            vec![vec![0x404c, copy[0][0], 1]]
        );
        assert!(calls(&log, STRING_SET).is_empty());
        e.register(STRING_COMPARE, |_, _| ret(0xffff_ffff));
        let (result, log) = run(&mut e, 0x005f_f7f0, &[0x4000, 0x5000]);
        assert!(result.bool());
        let buffer = calls(&log, STRING_COPY)[0][0];
        assert_eq!(calls(&log, STRING_SET), vec![vec![0x404c, buffer, 0]]);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
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

    /// Arrays as `{data, .., count at +0xC}` of form words.
    fn array_doubles(e: &mut Engine) {
        e.register(INDEX_FOR_FORM, |e, a| {
            let data = e.mem.u32(a[0]);
            let count = e.mem.u32(a[0] + 0xc);
            let found = (0..count).find(|&i| e.mem.u32(data + 4 * i) == a[1]);
            ret(found.unwrap_or(0xffff_ffff))
        });
        e.register(FORM_AT, |e, a| ret(e.mem.u32(e.mem.u32(a[0]) + 4 * a[1])));
        e.register(0x0087_7a30, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(ARRAY_REMOVE, |_, _| Ret::default());
        e.register(ARRAY_0061BDC0, |_, _| Ret::default());
        e.register(SET_IDLE_PREVIOUS, |_, _| Ret::default());
        e.register(SET_IDLE_PARENT, |_, _| Ret::default());
    }

    fn array(e: &mut Engine, forms: &[u32]) -> u32 {
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(4 * forms.len() as u32 + 4);
        for (i, &form) in forms.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, form);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 0xc, forms.len() as u32);
        array
    }

    #[test]
    fn child_index_and_removal() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let idle = e.mem.alloc(0x50);
        assert_eq!(
            e.call(0x005f_f520, &args![idle, 0x22u32]).u32(),
            0xffff_ffff
        );
        e.call_log = Some(vec![]);
        e.call(0x005f_f560, &args![idle, 1u32]);
        e.call(0x005f_f5d0, &args![idle, 0x22u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        let children = array(&mut e, &[0x11, 0x22, 0x33]);
        e.mem.set_u32(idle + 0x40, children);
        assert_eq!(e.call(0x005f_f520, &args![idle, 0x22u32]).u32(), 1);
        e.call_log = Some(vec![]);
        e.call(0x005f_f5d0, &args![idle, 0x22u32]);
        assert_eq!(calls(&e, SET_IDLE_PREVIOUS), vec![vec![0x33, 0x11]]);
        assert_eq!(calls(&e, ARRAY_REMOVE), vec![vec![children, 1]]);
        assert_eq!(calls(&e, ARRAY_0061BDC0), vec![vec![children]]);
        // The last child: no next one.
        e.call_log = Some(vec![]);
        e.call(0x005f_f560, &args![idle, 2u32]);
        assert!(calls(&e, SET_IDLE_PREVIOUS).is_empty());
        assert_eq!(calls(&e, ARRAY_REMOVE), vec![vec![children, 2]]);
        e.call_log = Some(vec![]);
        e.call(0x005f_f5d0, &args![idle, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_005ff610_takes_a_loose_idle_out() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        e.map(0x011c_b000, 0x1000);
        e.set_global(IDLE_MANAGER, 0x6000u32);
        let idle = e.mem.alloc(0x50);
        let loose = array(&mut e, &[0x5, idle]);
        e.register(0x005f_f200, |_, _| ret(1));
        e.register_double(LOOSE_IDLE_ARRAY, move |_, _| ret(loose));
        e.register(0x005f_f220, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_f610, &args![idle]);
        assert_eq!(calls(&e, LOOSE_IDLE_ARRAY), vec![vec![0x6000, idle]]);
        assert_eq!(calls(&e, ARRAY_REMOVE), vec![vec![loose, 1]]);
        assert_eq!(calls(&e, 0x005f_f220), vec![vec![idle, 0]]);
        // Not in the array: only the flag.
        let empty = array(&mut e, &[]);
        e.register_double(LOOSE_IDLE_ARRAY, move |_, _| ret(empty));
        e.call_log = Some(vec![]);
        e.call(0x005f_f610, &args![idle]);
        assert!(calls(&e, ARRAY_REMOVE).is_empty());
        assert_eq!(calls(&e, 0x005f_f220).len(), 1);
    }

    #[test]
    fn fn_005ff610_takes_a_child_out_of_its_parent() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        e.map(0x011c_b000, 0x1000);
        e.set_global(IDLE_MANAGER, 0x6000u32);
        e.register(0x005f_f200, |_, _| ret(0));
        let idle = e.mem.alloc(0x50);
        let next = e.mem.alloc(0x50);
        // After the removal the next idle sits at the idle's index.
        let children = array(&mut e, &[next, next]);
        e.register_double(0x0080_41a0, move |_, a| {
            ret(if a[0] == idle { 0x7000 } else { 0 })
        });
        e.register_double(0x008d_6f30, move |_, _| ret(children));
        e.register(IDLE_PREVIOUS, |_, _| ret(0x4321));
        e.register(0x0084_e3a0, |_, a| ret(a[0] + 1));
        for (temporary, previous) in [(0u32, 0x4322u32), (1, 0x4321)] {
            e.register_double(0x0040_13e0, move |_, _| ret(temporary));
            // The index lookup finds the idle at 1.
            e.register_double(INDEX_FOR_FORM, |_, _| ret(1));
            e.call_log = Some(vec![]);
            e.call(0x005f_f610, &args![idle]);
            assert_eq!(calls(&e, 0x008d_6f30), vec![vec![0x7000]]);
            assert_eq!(calls(&e, ARRAY_REMOVE), vec![vec![children, 1]]);
            assert_eq!(calls(&e, SET_IDLE_PARENT), vec![vec![idle, 0]]);
            assert_eq!(
                calls(&e, SET_IDLE_PREVIOUS),
                vec![vec![next, previous], vec![idle, 0]]
            );
        }
        // No parent: the root array; not found there: nothing removed.
        e.register(0x0080_41a0, |_, _| ret(0));
        e.register(ROOT_IDLE_ARRAY, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x005f_f610, &args![idle]);
        assert_eq!(calls(&e, ROOT_IDLE_ARRAY), vec![vec![0x6000, idle]]);
        assert!(calls(&e, INDEX_FOR_FORM).is_empty());
    }
}
