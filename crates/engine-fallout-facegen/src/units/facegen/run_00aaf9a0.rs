//! `FaceGen/run_00aaf9a0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A linear control (unit `fancontrols`): 0x3C bytes, a `FR2MatrixVTC<float>` and its label
/// string at +0x20.
const CONTROL_SIZE: u32 = 0x3c;
const CONTROL_LABEL: u32 = 0x20;
/// `FR2MatrixVTC<float>::operator=` (`006581e0`, `thiscall`: the source), the string's assign
/// from part of another (`0044a580`, `thiscall`: source, offset, count) and the string class's
/// `npos` (`.rdata`).
const MATRIX_ASSIGN: u32 = 0x0065_81e0;
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_NPOS: u32 = 0x0101_73f0;

// Translated from 00aaf9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the linear controls' backward copy (`cdecl`, six words: first,
/// last, the destination's end and three tag words it does not read). From the back, each
/// control before `last` down to `first` is assigned to the one before the destination's
/// end: the matrix (`006581e0`) and the label (`0044a580` with offset 0 and `npos`). Returns
/// the destination's new start, `end - (last - first) / 0x3C * 0x3C` (a signed division).
pub fn fn_00aaf9a0(
    e: &mut Engine,
    first: u32,
    last: u32,
    end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = end.wrapping_sub(
        ((last.wrapping_sub(first) as i32 / CONTROL_SIZE as i32) as u32).wrapping_mul(CONTROL_SIZE),
    );
    let mut target = end;
    let mut source = last;
    while first != source {
        source = source.wrapping_sub(CONTROL_SIZE);
        target = target.wrapping_sub(CONTROL_SIZE);
        e.call(MATRIX_ASSIGN, &args![target, source]);
        let npos: u32 = e.global(STRING_NPOS);
        e.call(
            STRING_ASSIGN,
            &args![
                target.wrapping_add(CONTROL_LABEL),
                source.wrapping_add(CONTROL_LABEL),
                0u32,
                npos
            ],
        );
    }
    start
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00aa_f9a0,
        fn_00aaf9a0(u32, u32, u32, u32, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backward_copy_assigns_from_the_back() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.register(MATRIX_ASSIGN, |_, _| Ret::default());
        e.register(STRING_ASSIGN, |_, _| Ret::default());
        let first = 0x0200_0000u32;
        let last = first + 2 * CONTROL_SIZE;
        let end = first + 3 * CONTROL_SIZE;
        e.call_log = Some(vec![]);
        let start = e
            .call(0x00aa_f9a0, &args![first, last, end, 0u32, 0u32, 0u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(start, first + CONTROL_SIZE);
        let matrices: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == MATRIX_ASSIGN)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            matrices,
            vec![
                vec![first + 2 * CONTROL_SIZE, first + CONTROL_SIZE],
                vec![first + CONTROL_SIZE, first]
            ]
        );
        let labels: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == STRING_ASSIGN)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            labels[1],
            vec![first + CONTROL_SIZE + 0x20, first + 0x20, 0, u32::MAX]
        );
        // An empty range copies nothing.
        assert_eq!(
            e.call(0x00aa_f9a0, &args![first, first, end, 0u32, 0u32, 0u32])
                .u32(),
            end
        );
    }
}
