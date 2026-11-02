//! `fallout/ai/run_009a3910` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009a3910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the next capacity for the word at `+0xc`:
/// doubled up to 0x400, plus 0x400 above it.
pub fn fn_009a3910(e: &mut Engine, this: Ptr) -> u32 {
    let size = e.mem.u32(this.addr() + 0xc);
    if size > 0x400 {
        size.wrapping_add(0x400)
    } else {
        size << 1
    }
}

/// `006e6da0` (unnamed): `cdecl`, size and block; returns the block (placement).
const PLACE_BLOCK: u32 = 0x006e_6da0;
/// `009a3d90` (unnamed, `run_009a3c50`): constructs one 8-byte item (`ECX` only).
const CONSTRUCT_ITEM: u32 = 0x009a_3d90;

// Translated from 009a3950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a3950 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is stored but never read):
/// constructs `count` 8-byte items at `buffer`, each placed by `006e6da0` and, when that
/// gives a block, constructed by `009a3d90`.
pub fn fn_009a3950(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let item = buffer.wrapping_add(index.wrapping_mul(8));
        let placed = e.call(PLACE_BLOCK, &args![8u32, item]).u32();
        if placed != 0 {
            e.call(CONSTRUCT_ITEM, &args![placed]);
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_3910, fn_009a3910(Ptr) -> u32),
        entry!(0x009a_3950, fn_009a3950(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_then_steps_by_0x400() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for (size, next) in [(0u32, 0u32), (0x10, 0x20), (0x400, 0x800), (0x401, 0x801)] {
            e.mem.set_u32(this + 0xc, size);
            assert_eq!(e.call(0x009a_3910, &args![this]).u32(), next);
        }
    }
}

#[cfg(test)]
mod construct_tests {
    use super::*;

    #[test]
    fn fn_009a3950_places_and_constructs_each_item() {
        let mut e = Engine::new();
        // The placement gives null for the second item, which is then not constructed.
        e.register(PLACE_BLOCK, |_, a| Ret {
            eax: if a[1] == 0x1008 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(CONSTRUCT_ITEM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_3950, &args![0u32, 0x1000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let placed: Vec<_> = log
            .iter()
            .filter(|call| call.0 == PLACE_BLOCK)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(
            placed,
            vec![vec![8, 0x1000], vec![8, 0x1008], vec![8, 0x1010]]
        );
        let constructed: Vec<_> = log
            .iter()
            .filter(|call| call.0 == CONSTRUCT_ITEM)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(constructed, vec![vec![0x1000], vec![0x1010]]);
    }
}
