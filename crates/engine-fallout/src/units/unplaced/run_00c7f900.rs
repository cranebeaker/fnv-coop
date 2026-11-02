//! `(unplaced)/run_00c7f900` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_memmove` (`00ec7230`; `cdecl`: destination, source, count).
const MEMMOVE: u32 = 0x00ec_7230;

// Translated from 00c7f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`stdcall`, `RET 0xC`): moves `count` 8-byte elements from
/// `source` to `destination`, one `_memmove` (`00ec7230`) of 8 bytes per element: front to
/// back when the destination lies below the source, back to front when above (the last
/// index counted down while it is not negative), nothing when they are equal or the count
/// is 0.
pub fn fn_00c7f900(e: &mut Engine, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    let distance = source.wrapping_sub(destination);
    if destination < source {
        let mut target = destination;
        let mut remaining = count;
        loop {
            e.call(MEMMOVE, &args![target, distance.wrapping_add(target), 8u32]);
            target = target.wrapping_add(8);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
        return;
    }
    if destination == source {
        return;
    }
    let mut index = count.wrapping_sub(1) as i32;
    if index < 0 {
        return;
    }
    let mut target = destination.wrapping_add((index as u32).wrapping_mul(8));
    while index >= 0 {
        e.call(MEMMOVE, &args![target, target.wrapping_add(distance), 8u32]);
        index = index.wrapping_sub(1);
        target = target.wrapping_sub(8);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c7_f900, fn_00c7f900(u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moves(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MEMMOVE)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn moves_forward_backward_or_not_at_all() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c7_f900, &args![0x1000u32, 0x1010u32, 2u32]);
        assert_eq!(
            moves(&e),
            vec![vec![0x1000, 0x1010, 8], vec![0x1008, 0x1018, 8]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c7_f900, &args![0x1010u32, 0x1000u32, 2u32]);
        assert_eq!(
            moves(&e),
            vec![vec![0x1018, 0x1008, 8], vec![0x1010, 0x1000, 8]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c7_f900, &args![0x1000u32, 0x1000u32, 3u32]);
        e.call(0x00c7_f900, &args![0x1000u32, 0x2000u32, 0u32]);
        assert!(moves(&e).is_empty());
        // A count with the sign bit set moves nothing backward.
        e.call(0x00c7_f900, &args![0x2000u32, 0x1000u32, 0x8000_0001u32]);
        assert!(moves(&e).is_empty());
    }
}
