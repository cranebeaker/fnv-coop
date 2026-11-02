//! `fallout shared/pathfinding/run_006e9700` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0` (cdecl: size, memory): placement new, returns the memory.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `006e2210` (`thiscall`): the 0x3c-byte element's constructor.
const ELEMENT_CONSTRUCT: u32 = 0x006e_2210;

// Translated from 006e9700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e9700 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 0x3c-byte elements at `first` (`thiscall`, `ECX` not
/// read; first, count; `RET 8`; no name in the engine map): each through
/// placement new (`006e6da0` with 0x3c) and, when that gives memory,
/// `006e2210`.
pub fn fn_006e9700(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x3cu32, index.wrapping_mul(0x3c).wrapping_add(first)],
            )
            .u32();
        if place != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![place]);
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_9700, fn_006e9700(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006e9700_constructs_where_the_placement_gives() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| {
            (if a[1] == 0x9000 + 0x3c { 0 } else { a[1] }).into_ret()
        });
        e.register(ELEMENT_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_9700, &args![0u32, 0x9000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let calls = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(
            calls(PLACEMENT_NEW),
            vec![
                vec![0x3c, 0x9000],
                vec![0x3c, 0x9000 + 0x3c],
                vec![0x3c, 0x9000 + 2 * 0x3c]
            ]
        );
        assert_eq!(
            calls(ELEMENT_CONSTRUCT),
            vec![vec![0x9000], vec![0x9000 + 2 * 0x3c]]
        );
    }
}
