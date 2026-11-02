//! `NiMain/niintegerextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a7b2e0`: the `NiExtraData` constructor (`thiscall`, no stack word).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
/// `00a7b330`: `NiExtraData`'s member copy (`thiscall` on the source: clone, cloning
/// process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
/// The `NiIntegerExtraData` virtual table (`010a0d24`).
const INTEGER_EXTRA_DATA_VTABLE: u32 = 0x010a_0d24;

// Translated from 00a99c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegerExtraData::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process):
/// allocates 0x10 bytes and, when that succeeds, constructs an `NiIntegerExtraData` in them
/// (`NiExtraData` constructor `00a7b2e0`, virtual table `010a0d24`, value 0). The source's
/// `NiExtraData` member copy (`00a7b330`) fills the clone (null when the allocation failed),
/// then the value at `+0xC` is copied. Answers the clone.
pub fn ni_integer_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if clone != 0 {
        e.call(EXTRA_DATA_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone, INTEGER_EXTRA_DATA_VTABLE);
        e.mem.set_u32(clone + 0xc, 0);
    }
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, clone, cloning]);
    // NiIntegerExtraData::m_iValue (Xbox PDB) +0x0C.
    let value = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(clone.wrapping_add(0xc), value);
    Ptr::new(clone)
}

/// `NiIntegerExtraData::ms_RTTI` (the class's `NiRTTI` record; its first word is the class
/// name).
const NI_INTEGER_EXTRA_DATA_RTTI: u32 = 0x011f_5f2c;
/// `NiExtraData::GetViewerStrings` (`00a7b580`, `thiscall`, the string array).
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiGetViewerString` (`00a7d330`, the class name) and `NiGetViewerString_ov7`
/// (`00a7d4d0`, a name and an integer), both `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_INTEGER: u32 = 0x00a7_d4d0;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literal `"m_iValue"`.
const TEXT_M_I_VALUE: u32 = 0x010a_0db8;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
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

// Translated from 00a99d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegerExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`, then
/// the class name (`NiGetViewerString` of `ms_RTTI`'s name word) and `"m_iValue"` with the
/// value (+0xc, `NiGetViewerString_ov7`) appended to the array.
pub fn ni_integer_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_INTEGER_EXTRA_DATA_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    // NiIntegerExtraData::m_iValue (Xbox PDB) +0x0C.
    let value = e.mem.u32(this.addr() + 0xc);
    let text = e
        .call(VIEWER_STRING_INTEGER, &args![TEXT_M_I_VALUE, value])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_9c80, ni_integer_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a9_9d70,
            ni_integer_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_clone_constructs_copies_and_takes_the_value() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 0xc, 0x55);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x10);
            block.into_ret()
        });
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |e, a| {
            // The value is still the constructor's 0 when the base copy runs.
            assert_eq!(e.mem.u32(a[1] + 0xc), 0);
            Ret::default()
        });
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, 0xabcd);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9c80, &args![source, 0x7000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), INTEGER_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0xabcd);
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_COPY_MEMBERS)
            .collect();
        assert_eq!(copies[0].1, vec![source, block, 0x7000]);
        let built: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_CONSTRUCTOR)
            .collect();
        assert_eq!(built[0].1, vec![block]);
    }

    #[test]
    fn create_clone_copies_into_a_null_clone_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.map(0, 0x10);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9c80, &args![source, 0x7000u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == EXTRA_DATA_CONSTRUCTOR));
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_COPY_MEMBERS)
            .collect();
        assert_eq!(copies[0].1, vec![source, 0, 0x7000]);
        // The exe then writes the value through the null clone (address 0x0C).
        assert_eq!(e.mem.u32(0xc), 9);
    }
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

    #[test]
    fn get_viewer_strings_adds_the_class_and_value() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_INTEGER_EXTRA_DATA_RTTI, 0x0123_0000);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_INTEGER, |_, _| 0x0e00_0002u32.into_ret());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xffff_fffe);
        // Size 1 of capacity 2, step 3: the second string grows the array to 3 + 2.
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00a9_9d70, &args![this, strings]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_INTEGER),
            vec![vec![TEXT_M_I_VALUE, 0xffff_fffe]]
        );
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        let set: Vec<u32> = calls_to(&e, STRINGS_SET_AT).iter().map(|c| c[1]).collect();
        assert_eq!(set, vec![1, 2]);
    }
}
