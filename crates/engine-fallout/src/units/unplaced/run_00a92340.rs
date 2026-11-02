//! `(unplaced)/run_00a92340` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriStripsDynamicData`'s vtable (its class confirmed by the RTTI before it) and the
/// base constructor `00a75dc0` (`thiscall`, eleven words, `RET 0x2C`).
const TRI_STRIPS_DYNAMIC_DATA_VTABLE: u32 = 0x0109_fc54;
const BASE_CONSTRUCTOR: u32 = 0x00a7_5dc0;

// Translated from 00a92340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTriStripsDynamicData` constructor (unnamed in the map; `thiscall`, thirteen words,
/// `RET 0x34`): the base constructor `00a75dc0` with the first eleven words, then the active
/// triangle count (+0x52) from the thirteenth word, the class's vtable, and the active
/// vertex count (+0x50) from the twelfth. Answers `this`.
pub fn fn_00a92340(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
    word_10: u32,
    word_11: u32,
    vertices: u16,
    triangles: u16,
) -> Ptr {
    e.call(
        BASE_CONSTRUCTOR,
        &args![
            this, word_1, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9, word_10,
            word_11
        ],
    );
    e.mem.set_u16(this.addr() + 0x52, triangles);
    e.mem.set_u32(this.addr(), TRI_STRIPS_DYNAMIC_DATA_VTABLE);
    e.mem.set_u16(this.addr() + 0x50, vertices);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a92340, fn_00a92340(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u16, u16) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a92340_passes_eleven_words_and_sets_the_counts() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x54);
        let mut words = vec![this];
        words.extend(1..=11u32);
        words.push(0x1_0007);
        words.push(0x2_0009);
        assert_eq!(e.call(0x00a92340, &words).u32(), this);
        assert_eq!(e.mem.u32(this), TRI_STRIPS_DYNAMIC_DATA_VTABLE);
        assert_eq!(e.mem.u16(this + 0x50), 7);
        assert_eq!(e.mem.u16(this + 0x52), 9);
        let base: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == BASE_CONSTRUCTOR)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![words[..12].to_vec()]);
    }
}
