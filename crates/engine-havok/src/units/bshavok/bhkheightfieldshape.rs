//! `BSHavok/bhkheightfieldshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The one function carries an `FS:[0]` exception-unwinding frame that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word (`TLS + 0x2b4`) that the function sets to 0x10 around its work.
const SCOPE_WORD: u32 = 0x2b4;
/// `bhkShape::GetViewerStrings` (Xbox PDB, `00c851c0`, `(this, strings)`).
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c8_51c0;
/// `NiGetViewerString` and the two array helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The word the class passes to `NiGetViewerString`.
const CLASS_NAME_POINTER: u32 = 0x0126_8f90;

// Translated from 00d54ed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d54ed0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHeightFieldShape::GetViewerStrings` (Xbox PDB), with the allocation scope word set to
/// 0x10: runs `bhkShape::GetViewerStrings` (`00c851c0`) on `(this, strings)`, then appends
/// the string `NiGetViewerString` makes of the word at `01268f90` to the array (grown by
/// the step at `+0xE` plus the size when the 16-bit size at `+0xA` has reached the capacity
/// at `+0x8`).
pub fn bhk_height_field_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    let capacity = u32::from(e.mem.u16(strings.addr() + 8));
    if size >= capacity {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d54ed0,
        bhk_height_field_shape_get_viewer_strings(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array_before_storing() {
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(CLASS_NAME_POINTER, 4);
            e.set_global(CLASS_NAME_POINTER, 0x5151_0001u32);
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x66);
            e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(STRINGS_GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(STRINGS_SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            let this = e.mem.alloc(0x20);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.call_log = Some(vec![]);
            e.call(0x00d54ed0, &args![this, strings]);
            assert_eq!(calls(&e, SHAPE_GET_VIEWER_STRINGS), [[this, strings]]);
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            let grown = calls(&e, STRINGS_GROW);
            if size >= capacity {
                assert_eq!(grown, [[strings, 5 + u32::from(size)]]);
            } else {
                assert!(grown.is_empty());
            }
            assert_eq!(
                &calls(&e, STRINGS_SET_AT)[0][..2],
                [strings, u32::from(size)]
            );
            assert_eq!(stored.get(), 0x7777_0001);
            assert_eq!(e.mem.u32(word), 0x66);
        }
    }
}
