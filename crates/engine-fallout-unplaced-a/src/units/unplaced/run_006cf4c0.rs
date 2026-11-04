//! `(unplaced)/run_006cf4c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00978bc0` (`ECX` = array): the index of a new element.
const NEW_INDEX: u32 = 0x0097_8bc0;
/// `006d03d0` (`ECX` = array; the element, a word; `RET 8`).
const ELEMENT_006D03D0: u32 = 0x006d_03d0;

// Translated from 006cf4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a 12-byte element (`thiscall`, the address of the value; `RET 4`):
/// the index from `00978bc0`, `006d03d0` with that element (the storage at
/// +4 plus 12 times the index) and 1, then the three words copied in (the
/// storage read again). Returns the index. The engine map gives it no name.
pub fn fn_006cf4c0(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    let base = this.addr();
    let index = e.call(NEW_INDEX, &args![this]).u32();
    let element = index.wrapping_mul(0xc).wrapping_add(e.mem.u32(base + 4));
    e.call(ELEMENT_006D03D0, &args![this, element, 1u32]);
    let element = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(0xc));
    for i in 0..3 {
        let word = e.mem.u32(value.addr() + 4 * i);
        e.mem.set_u32(element + 4 * i, word);
    }
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_f4c0, fn_006cf4c0(Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006cf4c0_copies_the_value_into_the_new_element() {
        let mut e = Engine::new();
        e.register(NEW_INDEX, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(ELEMENT_006D03D0, |_, _| Ret::default());
        let storage = e.mem.alloc(0x30);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, storage);
        let value = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(value + 4 * i, 0x50 + i);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c_f4c0, &args![this, value]).u32(), 2);
        let marks: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == ELEMENT_006D03D0)
            .collect();
        assert_eq!(
            marks,
            vec![(ELEMENT_006D03D0, vec![this, storage + 0x18, 1])]
        );
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(storage + 0x18 + 4 * i)).collect();
        assert_eq!(words, vec![0x50, 0x51, 0x52]);
    }
}
