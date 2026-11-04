//! `fallout shared/niformarray.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's element count (`0084e3a0`, on the array).
const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// The address of element `i` (`00877a30`, on the array, one word).
const ARRAY_AT: u32 = 0x0087_7a30;

// Translated from 004d6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFormArray::GetIndexForForm` (Xbox PDB): the index of the first
/// element equal to `form`, or -1 (also for a null `form`).
pub fn ni_form_array_get_index_for_form(e: &mut Engine, this: Ptr, form: u32) -> i32 {
    let mut index = -1i32;
    if form != 0 {
        let count = e.call(ARRAY_COUNT, &args![this]).u32();
        if count != 0 {
            let mut i = 0u32;
            while i < count {
                let slot = e.call(ARRAY_AT, &args![this, i]).u32();
                if e.mem.u32(slot) == form {
                    index = i as i32;
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
    }
    index
}

// Translated from 004d6170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFormArray::GetFormAt` (Xbox PDB): element `index`, or null past the
/// end.
pub fn ni_form_array_get_form_at(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let mut form = 0;
    if index < e.call(ARRAY_COUNT, &args![this]).u32() {
        let slot = e.call(ARRAY_AT, &args![this, index]).u32();
        form = e.mem.u32(slot);
    }
    form
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_6100, ni_form_array_get_index_for_form(Ptr, u32) -> i32),
        entry!(0x004d_6170, ni_form_array_get_form_at(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An array double: the count at `this`, the elements from `this + 4`.
    fn array(e: &mut Engine, forms: &[u32]) -> u32 {
        let this = e.mem.alloc(4 + 4 * forms.len() as u32);
        e.mem.set_u32(this, forms.len() as u32);
        for (i, form) in forms.iter().enumerate() {
            e.mem.set_u32(this + 4 + 4 * i as u32, *form);
        }
        e.register(ARRAY_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(ARRAY_AT, |_, a| Ret {
            eax: a[0] + 4 + 4 * a[1],
            ..Ret::default()
        });
        this
    }

    #[test]
    fn get_index_for_form_finds_the_first_match() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0x10, 0x20, 0x20]);
        assert_eq!(e.call(0x004d_6100, &args![this, 0x20u32]).i32(), 1);
        assert_eq!(e.call(0x004d_6100, &args![this, 0x30u32]).i32(), -1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_6100, &args![this, 0u32]).i32(), -1);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != ARRAY_COUNT));
    }

    #[test]
    fn get_form_at_is_null_past_the_end() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0x10, 0x20]);
        assert_eq!(e.call(0x004d_6170, &args![this, 1u32]).u32(), 0x20);
        assert_eq!(e.call(0x004d_6170, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.call(0x004d_6170, &args![this, 0xffff_ffffu32]).u32(), 0);
    }
}
