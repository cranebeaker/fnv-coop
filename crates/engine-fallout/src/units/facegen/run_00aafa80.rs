//! `FaceGen/run_00aafa80` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The uninitialized copy of FaceGen's 0x3C-byte linear controls (a `FR2MatrixVTC<float>` and
//! a label string), which `fancontrols.obj`'s vector code calls.

#[allow(unused_imports)]
use crate::prelude::*;

/// The linear control's copy constructor (`00aaec10`, `thiscall`: the source), unit
/// `fancontrols`.
const CONTROL_COPY: u32 = 0x00aa_ec10;

// Translated from 00aafa80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aafa80 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination, allocator and two
/// tags): copy-constructs (`00aaec10`) the controls from first to last at the destination,
/// skipping null slots. Returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00aafa80(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            e.call(CONTROL_COPY, &args![destination, first]);
        }
        destination = destination.wrapping_add(0x3c);
        first = first.wrapping_add(0x3c);
    }
    destination
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00aa_fa80,
        fn_00aafa80(u32, u32, u32, u32, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_each_control_and_returns_the_end() {
        let mut e = Engine::new();
        e.register(CONTROL_COPY, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x00aa_fa80,
                &args![0x1000u32, 0x1078u32, 0x2000u32, 0u32, 0u32, 0u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(end, 0x2078);
        let copies: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == CONTROL_COPY)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(copies, vec![vec![0x2000, 0x1000], vec![0x203c, 0x103c]]);
        // A null destination copies nothing but still advances.
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x00aa_fa80,
                &args![0x1000u32, 0x103cu32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, 0x3c);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
    }
}
