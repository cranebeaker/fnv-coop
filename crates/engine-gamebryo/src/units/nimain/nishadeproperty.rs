//! `NiMain/nishadeproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiShadeProperty` (Xbox PDB), 0x20 bytes; the PC offsets match.
    pub struct NiShadeProperty: 0x20 {
        /// `m_uFlags` (Xbox PDB): bit 0 is `m_bSmooth`.
        0x18 m_uFlags: u16,
        /// `iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: i32,
    }
}

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00a5c200` (`thiscall`, unit `niobjectnet.cpp`): the object base constructor.
const OBJECT_BASE_CONSTRUCTOR: u32 = 0x00a5_c200;
/// `NiShadeProperty`'s vtable (its RTTI names `NiShadeProperty`).
const NI_SHADE_PROPERTY_VTABLE: u32 = 0x0109_e274;
/// `00a5d7a0` (`thiscall`, (clone, cloning process)): the property base's member copy.
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `NiProperty::GetViewerStrings` (Xbox PDB, `thiscall`, the string array).
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word) and its `bool` overload
/// `NiGetViewerString_ov2` (`cdecl`, name and value word).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `011f5ae0`: the word the exe passes to `NiGetViewerString` as the first string (the
/// first word of the class's `NiRTTI` record, its name, by the pattern of the other
/// properties; not confirmed here).
const SHADE_PROPERTY_RTTI_NAME: u32 = 0x011f_5ae0;
/// `"m_bSmooth"`.
const NAME_SMOOTH: u32 = 0x0109_e308;

// Translated from 00a81480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShadeProperty::CreateClone` (Xbox PDB): a new shade property (0x20 bytes from
/// `00aa13e0`; the base constructor, the vtable `0109e274`, `iShaderPropertyType` -1,
/// flags 1; null when the allocation fails), given the base's members (`00a5d7a0`, called
/// on null too) and this one's flags (+0x18, written even through a null clone, as the exe
/// does). Answers the clone.
pub fn ni_shade_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, NI_SHADE_PROPERTY_VTABLE);
        e.mem.set_u32(block + 0x1c, 0xffff_ffff);
        e.mem.set_u16(block + 0x18, 1);
        block
    };
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.wrapping_add(0x18), flags);
    Ptr::new(clone)
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array: when
/// the count (`m_usSize`, +0xa) is not below the capacity (`m_usMaxSize`, +8) the array first
/// grows to count + `m_usGrowBy` (+0xe, `0096ad30`); then `0096ae90` stores it at the old
/// count.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a81600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShadeProperty::GetViewerStrings` (Xbox PDB): the property base's strings
/// (`00a5d850`), then two appended to `strings`: `NiGetViewerString` of the word at
/// `011f5ae0` and `m_bSmooth` (flag bit 0).
///
/// The game keeps each string pointer in the `strings` argument's stack word and passes the
/// boolean by writing its byte into the low byte of that same word, so the word pushed holds
/// the first string pointer's upper three bytes, as here.
pub fn ni_shade_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(SHADE_PROPERTY_RTTI_NAME);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let smooth = e.mem.u8(this.addr() + 0x18) & 1;
        e.mem.set_u8(slot, smooth);
        let word = e.mem.u32(slot);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_SMOOTH, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_1480, ni_shade_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_1600, ni_shade_property_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn create_clone_builds_a_property_with_this_ones_flags() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PROPERTY_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 0x18, 0);
        let process = 0x4444u32;
        let clone = e.call(0x00a8_1480, &args![this, process]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x20]]);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, process]]
        );
        assert_eq!(e.mem.u32(clone), NI_SHADE_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x1c), 0xffff_ffff);
        // The constructor's flags 1 are replaced by this one's.
        assert_eq!(e.mem.u16(clone + 0x18), 0);
    }

    #[test]
    #[should_panic]
    fn create_clone_writes_through_a_failed_allocation() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(PROPERTY_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call(0x00a8_1480, &args![this, 0u32]);
    }

    #[test]
    fn viewer_strings_add_the_class_and_smooth_flag() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(SHADE_PROPERTY_RTTI_NAME, 0x0109_e000);
        e.register(PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1234_5600u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, a| {
            (0x0700_0000 | (a[1] & 0xff)).into_ret()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 4);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 0x18, 0xfff1);
        e.call(0x00a8_1600, &args![this, strings]);
        assert_eq!(
            calls_to(&e, PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_e000]]);
        // The byte goes over the first string pointer's upper bytes.
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_BOOL),
            vec![vec![NAME_SMOOTH, 0x1234_5601]]
        );
        // Only the second append grows the array (count 1, capacity 1).
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 5]]);
        assert_eq!(e.mem.u32(base), 0x1234_5600);
        assert_eq!(e.mem.u32(base + 4), 0x0700_0001);
    }
}
