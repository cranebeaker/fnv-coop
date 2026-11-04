//! `fallout shared/tesfullname.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::AddChunkArray` (`004856f0`, `cdecl (chunk, data, size)`).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// The name's length (`0048cee0`, `thiscall`: `004048e0` on the string at
/// +4) and its text (`00408da0`, `thiscall`: the string's text or `""`).
const NAME_LENGTH: u32 = 0x0048_cee0;
const NAME_TEXT: u32 = 0x0040_8da0;

// Translated from 00487010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFullName::Save` (Xbox PDB): with a non-empty name (`0048cee0`), adds
/// `FULL` holding its text (`00408da0`) and the terminating zero.
pub fn tes_full_name_save(e: &mut Engine, this: Ptr) {
    if e.call(NAME_LENGTH, &args![this]).u32() == 0 {
        return;
    }
    let size = e.call(NAME_LENGTH, &args![this]).u32().wrapping_add(1);
    let text = e.call(NAME_TEXT, &args![this]).u32();
    e.call(ADD_CHUNK_ARRAY, &args![0x4c4c_5546u32, text, size]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_7010, tes_full_name_save(Ptr))]
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    #[test]
    fn save_adds_full_with_the_terminating_zero() {
        let mut e = Engine::new();
        e.register(NAME_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NAME_TEXT, |_, a| (a[0] + 0x100).into_ret());
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 5);
        e.call_log = Some(vec![]);
        e.call(0x0048_7010, &args![this]);
        let added: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == ADD_CHUNK_ARRAY)
            .collect();
        assert_eq!(
            added,
            vec![(ADD_CHUNK_ARRAY, vec![0x4c4c_5546, this + 0x100, 6])]
        );
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_7010, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
