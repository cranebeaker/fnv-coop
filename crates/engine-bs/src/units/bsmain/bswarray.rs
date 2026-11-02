//! `BSMain/bswarray.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiIntegersExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTEGERS_EXTRA_DATA_RTTI: u32 = 0x011f_5f24;

// Translated from 00a99940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegersExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_integers_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTEGERS_EXTRA_DATA_RTTI)
}

/// `NiIntegersExtraData`'s virtual table (`010a0c7c`).
const NI_INTEGERS_EXTRA_DATA_VTABLE: u32 = 0x010a_0c7c;
/// The block free (`00aa10f0`, `cdecl`), the sized delete (`00aa1460`, `cdecl`: block,
/// size) and `NiExtraData`'s destructor body (`00a7b300`, `thiscall`).
const NI_FREE: u32 = 0x00aa_10f0;
const NI_DELETE: u32 = 0x00aa_1460;
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// `NiIntegersExtraData::GetViewerStrings` (`00a99b20`, engine map; `thiscall`, `RET 4`),
/// `NiGetViewerString` (`00a7d330`, `cdecl`: text), its overload `00a7d510` (`cdecl`: name,
/// value) and the viewer string array's add (`0096a610`, `thiscall`, `RET 4`: address of the
/// string).
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a9_9b20;
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_OV8: u32 = 0x00a7_d510;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// `BSWArray::ms_RTTI` (`012043a4`; its first word is the class name) and `"m_uiSize"`
/// (`0109df4c`).
const BS_W_ARRAY_RTTI: u32 = 0x0120_43a4;
const SIZE_TEXT: u32 = 0x0109_df4c;

// Translated from 00a99a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegersExtraData::_scalar_deleting_destructor_` (engine map; `thiscall`, `RET 4`):
/// the class's virtual table, the value array (+0x10) freed (`00aa10f0`) and cleared,
/// `NiExtraData`'s destructor (`00a7b300`) and, when bit 0 of `flags` is set, the sized delete
/// of 0x14 bytes. Answers `this`.
pub fn ni_integers_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let values = e.mem.u32(this.addr().wrapping_add(0x10));
    e.mem.set_u32(this.addr(), NI_INTEGERS_EXTRA_DATA_VTABLE);
    e.call(NI_FREE, &args![values]);
    e.mem.set_u32(this.addr().wrapping_add(0x10), 0);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x14u32]);
    }
    this
}

/// Adds the viewer string `text` (by address, as the exe passes it) to `strings`.
fn add_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
    });
}

// Translated from 00c615a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWArray::GetViewerStrings` (engine map; `thiscall`, `RET 4`: the viewer string array):
/// `NiIntegersExtraData::GetViewerStrings` (`00a99b20`), then the class name (the first word
/// of `012043a4`, through `NiGetViewerString`) and `"m_uiSize"` with the count at +0xC
/// (`00a7d510`), each added to the array.
pub fn bs_w_array_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(BS_W_ARRAY_RTTI);
    let text = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let size = e.mem.u32(this.addr().wrapping_add(0xc));
    let text = e.call(VIEWER_STRING_OV8, &args![SIZE_TEXT, size]).u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a99940, ni_integers_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00a9_9a50,
            ni_integers_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c6_15a0, bs_w_array_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_integers_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a99940, &args![]).u32(),
            NI_INTEGERS_EXTRA_DATA_RTTI
        );
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn deleting_destructor_frees_the_values_then_the_object_on_bit_zero() {
        let mut e = Engine::new();
        for address in [NI_FREE, NI_DELETE, EXTRA_DATA_DESTRUCTOR] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9a50, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), NI_INTEGERS_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert!(log.contains(&(NI_FREE, vec![0x7000])));
        assert!(log.contains(&(EXTRA_DATA_DESTRUCTOR, vec![this])));
        assert!(!log.iter().any(|c| c.0 == NI_DELETE));
        e.call_log = Some(vec![]);
        e.call(0x00a9_9a50, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_DELETE, vec![this, 0x14])));
    }

    #[test]
    fn viewer_strings_follow_the_base_ones() {
        let mut e = Engine::new();
        e.map(BS_W_ARRAY_RTTI, 8);
        e.mem.set_u32(BS_W_ARRAY_RTTI, 0x0400_0000);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, _| 0x51u32.into_ret());
        e.register(VIEWER_STRING_OV8, |_, a| (0x60 + a[1]).into_ret());
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = added.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        e.call(0x00c6_15a0, &args![this, 0x900u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_GET_VIEWER_STRINGS, vec![this, 0x900])));
        assert!(log.contains(&(VIEWER_STRING, vec![0x0400_0000])));
        assert!(log.contains(&(VIEWER_STRING_OV8, vec![SIZE_TEXT, 5])));
        assert_eq!(*added.borrow(), vec![(0x900, 0x51), (0x900, 0x65)]);
    }
}
