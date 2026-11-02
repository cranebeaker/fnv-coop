//! `fallout/ai/run_009f8fa0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memmove` (`00ec7230`, engine map; cdecl: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;
/// The size of a `SpectatorThreatInfo` (Xbox PDB, 0x34 bytes).
const THREAT_SIZE: u32 = 0x34;

// Translated from 009f8fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `BSSimpleArray<SpectatorThreatInfo,1024>`
/// item move; `thiscall`, `RET 0xC`): moves `count` 0x34-byte threats from
/// `source` to `destination` one by one (`memmove` of 0x34 bytes): forward
/// when the destination is below the source, backward (the index compared
/// signed) when above, nothing when they are equal or `count` is 0.
pub fn fn_009f8fa0(e: &mut Engine, _this: Ptr, destination: Ptr, source: Ptr, count: u32) {
    if count == 0 {
        return;
    }
    let (to, from) = (destination.addr(), source.addr());
    let at = |base: u32, index: u32| index.wrapping_mul(THREAT_SIZE).wrapping_add(base);
    if to < from {
        let mut index = 0u32;
        while index < count {
            e.call(MEMMOVE, &args![at(to, index), at(from, index), THREAT_SIZE]);
            index = index.wrapping_add(1);
        }
    } else if to > from {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let i = index as u32;
            e.call(MEMMOVE, &args![at(to, i), at(from, i), THREAT_SIZE]);
            index = index.wrapping_sub(1);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009f_8fa0, fn_009f8fa0(Ptr, Ptr, Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moves(e: &mut Engine, to: u32, from: u32, count: u32) -> Vec<Vec<u32>> {
        e.call_log = Some(vec![]);
        e.call(0x009f_8fa0, &args![0u32, to, from, count]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == MEMMOVE)
            .map(|c| c.1)
            .collect()
    }

    #[test]
    fn at_009f8fa0_moves_forward_backward_or_not_at_all() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, a| a[0].into_ret());
        assert_eq!(
            moves(&mut e, 0x1000, 0x1034, 2),
            vec![vec![0x1000, 0x1034, 0x34], vec![0x1034, 0x1068, 0x34]]
        );
        assert_eq!(
            moves(&mut e, 0x1034, 0x1000, 2),
            vec![vec![0x1068, 0x1034, 0x34], vec![0x1034, 0x1000, 0x34]]
        );
        assert!(moves(&mut e, 0x1000, 0x1000, 2).is_empty());
        assert!(moves(&mut e, 0x1000, 0x2000, 0).is_empty());
    }
}
