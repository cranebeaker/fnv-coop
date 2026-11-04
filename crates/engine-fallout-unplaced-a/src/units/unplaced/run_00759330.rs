//! `(unplaced)/run_00759330` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSStringT` constructor (`004037b0`, `thiscall`) and its set from a C
/// string (`00438390`, `thiscall`, one word).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_SET: u32 = 0x0043_8390;
/// The empty C string at `01011584`.
const EMPTY_STRING: u32 = 0x0101_1584;
/// The `BSSimpleList<int>` constructor (`0096a2d0`, `thiscall`).
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;

// Translation incomplete: 00759330 - the FS:[0] exception frame (destroying the string when the list constructor throws) is not translated
// Translated from 00759330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Font::TextData` constructor (Xbox PDB layout; unnamed in the engine
/// map; `thiscall`, five words, `RET 0x14`): constructs `xNewText` (+0) and
/// `xLineWidths` (+0x20), sets the text to `""`, then `iWidth` (+8),
/// `iHeight` (+0xC), `iLineStart` (+0x10), `iLineEnd` (+0x14), clears
/// `iCharCount` (+0x18) and stores the low byte of `line_separator` in
/// `cLineSep` (+0x1C). Returns `this`.
pub fn fn_00759330(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    line_start: u32,
    line_end: u32,
    line_separator: u32,
) -> Ptr {
    let at = this.addr();
    e.call(STRING_CONSTRUCT, &args![at]);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![at + 0x20]);
    e.call(STRING_SET, &args![at, EMPTY_STRING]);
    e.mem.set_u32(at + 8, width);
    e.mem.set_u32(at + 0xc, height);
    e.mem.set_u32(at + 0x10, line_start);
    e.mem.set_u32(at + 0x14, line_end);
    e.mem.set_u32(at + 0x18, 0);
    e.mem.set_u8(at + 0x1c, line_separator as u8);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00759330,
        fn_00759330(Ptr, u32, u32, u32, u32, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_data_constructor_sets_the_fields() {
        let mut e = Engine::new();
        for address in [STRING_CONSTRUCT, STRING_SET, SIMPLE_LIST_CONSTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x18, 9);
        e.call_log = Some(vec![]);
        let made = e
            .call(
                0x0075_9330,
                &args![this, 10u32, 20u32, 1u32, 2u32, 0x17cu32],
            )
            .u32();
        assert_eq!(made, this);
        assert_eq!(e.mem.u32(this + 8), 10);
        assert_eq!(e.mem.u32(this + 0xc), 20);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u8(this + 0x1c), 0x7c);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (STRING_CONSTRUCT, vec![this]),
                (SIMPLE_LIST_CONSTRUCT, vec![this + 0x20]),
                (STRING_SET, vec![this, EMPTY_STRING]),
            ]
        );
    }
}
