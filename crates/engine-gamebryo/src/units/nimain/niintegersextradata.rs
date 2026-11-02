//! `NiMain/niintegersextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiIntegersExtraData::ms_RTTI` (the class's `NiRTTI` record; its first word is the
/// class name).
const NI_INTEGERS_EXTRA_DATA_RTTI: u32 = 0x011f_5f24;
/// `NiIntegersExtraData`'s vtable.
const NI_INTEGERS_EXTRA_DATA_VTABLE: u32 = 0x010a_0c7c;
/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiExtraData`'s constructor (`thiscall`, no stack words) and `GetViewerStrings`
/// (`00a7b580`, `thiscall`, the string array).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `00a99870` (unnamed and unplaced in the engine map; `thiscall`: the clone and the cloning
/// process): what `CreateClone` runs on the new object, by the call the member copy.
const COPY_MEMBERS_00A99870: u32 = 0x00a9_9870;
/// `NiGetViewerString` (`00a7d330`, the class name), `NiGetViewerString_ov7` (`00a7d4d0`, a
/// name and an integer) and `NiGetViewerString_ov8` (`00a7d510`, a name and an unsigned
/// word), all `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_INTEGER: u32 = 0x00a7_d4d0;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literals `"m_uiSize"` and `"m_piValue[i]"`.
const TEXT_M_UI_SIZE: u32 = 0x0109_df4c;
const TEXT_M_PI_VALUE_I: u32 = 0x010a_0d10;

/// The number of integers (`m_uiSize`, Xbox PDB) at +0xc and the integers (`m_piValue`) at
/// +0x10.
const SIZE: u32 = 0xc;
const VALUES: u32 = 0x10;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size through `0096ae90` (a stack word holding
/// it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// The same append with the set-at inlined (`NiTArray::SetAt`): after the grow the size is
/// read again; storing at or past it makes the size the index plus one and counts a
/// non-null string in the 16-bit used count (+0xc); storing below it adjusts the used count
/// by the change between the old element and `text`. The string goes into the elements
/// (+0x4) at the old size.
fn push_viewer_string_inline(e: &mut Engine, strings: Ptr, text: u32) {
    let base = strings.addr();
    let index = u32::from(e.mem.u16(base + 0xa));
    if index >= u32::from(e.mem.u16(base + 8)) {
        let step = u32::from(e.mem.u16(base + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + index]);
    }
    let size = u32::from(e.mem.u16(base + 0xa));
    if index >= size {
        e.mem.set_u16(base + 0xa, (index + 1) as u16);
        if text != 0 {
            let used = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, used.wrapping_add(1));
        }
    } else {
        let old = e.mem.u32(e.mem.u32(base + 4).wrapping_add(4 * index));
        if text != 0 {
            if old == 0 {
                let used = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, used.wrapping_add(1));
            }
        } else if old != 0 {
            let used = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, used.wrapping_add(0xffff));
        }
    }
    let elements = e.mem.u32(base + 4);
    e.mem.set_u32(elements.wrapping_add(4 * index), text);
}

// Translated from 00a998e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegersExtraData::IsEqual` (Xbox PDB): false for a null other object or a different
/// count (+0xc); when exactly one side has no integers pointer, false; when neither has
/// one, true; otherwise true exactly when every integer matches. The base class is not
/// consulted.
pub fn ni_integers_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    let count = e.mem.u32(this.addr() + SIZE);
    if count != e.mem.u32(other.addr() + SIZE) {
        return false;
    }
    let mine = e.mem.u32(this.addr() + VALUES);
    let theirs = e.mem.u32(other.addr() + VALUES);
    if mine != 0 {
        if theirs == 0 {
            return false;
        }
    } else {
        return theirs == 0;
    }
    for index in 0..count {
        let a = e.mem.u32(mine.wrapping_add(4 * index));
        if a != e.mem.u32(theirs.wrapping_add(4 * index)) {
            return false;
        }
    }
    true
}

// Translated from 00a999b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegersExtraData::CreateClone` (Xbox PDB): allocates 0x14 bytes and, when that
/// succeeds, builds an empty integers extra data in them (`NiExtraData` constructor, the
/// vtable `010a0c7c`, no integers); then `00a99870` (the member copy) on the clone, null
/// too as the exe does. Answers the clone.
pub fn ni_integers_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if clone != 0 {
        e.call(EXTRA_DATA_CONSTRUCT, &args![clone]);
        e.mem.set_u32(clone, NI_INTEGERS_EXTRA_DATA_VTABLE);
        e.mem.set_u32(clone + VALUES, 0);
        e.mem.set_u32(clone + SIZE, 0);
    }
    e.call(COPY_MEMBERS_00A99870, &args![this, clone, cloning_process]);
    Ptr::new(clone)
}

// Translated from 00a99b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegersExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`, the
/// class name (`NiGetViewerString` of `ms_RTTI`'s name word), `"m_uiSize"` with the count,
/// then `"m_piValue[i]"` with each integer (the count read again each round), the last ones
/// appended with the array's set-at inlined.
pub fn ni_integers_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_INTEGERS_EXTRA_DATA_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let count = e.mem.u32(this.addr() + SIZE);
    let text = e
        .call(VIEWER_STRING_UNSIGNED, &args![TEXT_M_UI_SIZE, count])
        .u32();
    push_viewer_string(e, strings, text);
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + SIZE) {
        let values = e.mem.u32(this.addr() + VALUES);
        let value = e.mem.u32(values.wrapping_add(4 * index));
        let text = e
            .call(VIEWER_STRING_INTEGER, &args![TEXT_M_PI_VALUE_I, value])
            .u32();
        push_viewer_string_inline(e, strings, text);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_98e0, ni_integers_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_99b0,
            ni_integers_extra_data_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a9_9b20,
            ni_integers_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An integers extra data holding `values`.
    fn integers(e: &mut Engine, values: &[u32]) -> u32 {
        let this = e.mem.alloc(0x14);
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + SIZE, values.len() as u32);
        e.mem.set_u32(this + VALUES, array);
        this
    }

    #[test]
    fn is_equal_compares_counts_pointers_and_integers() {
        let mut e = Engine::new();
        let this = integers(&mut e, &[1, 2]);
        let other = integers(&mut e, &[1, 2]);
        assert!(e.call(0x00a9_98e0, &args![this, other]).bool());
        assert!(!e.call(0x00a9_98e0, &args![this, 0u32]).bool());
        e.mem.set_u32(e.mem.u32(other + VALUES) + 4, 3);
        assert!(!e.call(0x00a9_98e0, &args![this, other]).bool());
        let short = integers(&mut e, &[1]);
        assert!(!e.call(0x00a9_98e0, &args![this, short]).bool());
        e.mem.set_u32(other + VALUES, 0);
        assert!(!e.call(0x00a9_98e0, &args![this, other]).bool());
        assert!(!e.call(0x00a9_98e0, &args![other, this]).bool());
        e.mem.set_u32(this + VALUES, 0);
        assert!(e.call(0x00a9_98e0, &args![this, other]).bool());
    }

    #[test]
    fn create_clone_builds_then_copies() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x14]);
            block.into_ret()
        });
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS_00A99870, |e, a| {
            if a[1] != 0 {
                assert_eq!(e.mem.u32(a[1]), NI_INTEGERS_EXTRA_DATA_VTABLE);
                assert_eq!((e.mem.u32(a[1] + SIZE), e.mem.u32(a[1] + VALUES)), (0, 0));
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = integers(&mut e, &[5]);
        let clone = e.call(0x00a9_99b0, &args![this, 0x77u32]).u32();
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, COPY_MEMBERS_00A99870),
            vec![vec![this, clone, 0x77]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_99b0, &args![this, 0x77u32]).u32(), 0);
        assert_eq!(calls_to(&e, COPY_MEMBERS_00A99870)[1], vec![this, 0, 0x77]);
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn get_viewer_strings_lists_each_integer() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_INTEGERS_EXTRA_DATA_RTTI, 0x0123_0000);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, _| 0x0e00_0002u32.into_ret());
        e.register(VIEWER_STRING_INTEGER, |_, a| {
            (0x0e10_0000 + a[1]).into_ret()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = integers(&mut e, &[7, 9]);
        let strings = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, elements);
        e.mem.set_u16(strings + 8, 8);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a9_9b20, &args![this, strings]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_UNSIGNED),
            vec![vec![TEXT_M_UI_SIZE, 2]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_INTEGER),
            vec![vec![TEXT_M_PI_VALUE_I, 7], vec![TEXT_M_PI_VALUE_I, 9]]
        );
        assert_eq!(e.mem.u16(strings + 0xa), 4);
        assert_eq!(e.mem.u16(strings + 0xc), 2);
        assert_eq!(e.mem.u32(elements + 8), 0x0e10_0007);
        assert_eq!(e.mem.u32(elements + 12), 0x0e10_0009);
        assert!(calls_to(&e, STRINGS_GROW).is_empty());
    }
}
