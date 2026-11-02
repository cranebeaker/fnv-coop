//! `fallout/ai/run_009a3a80` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memmove` (`00ec7230`, `cdecl`: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;

// Translated from 009a3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; `this` is stored but never read): moves
/// `count` 0x14-byte elements from `source` to `destination` one element at a time (`memmove`
/// of 0x14 bytes each): forwards from the first element when `destination` is below
/// `source`, backwards from the last one when it is above, nothing when they are equal or
/// `count` is 0.
pub fn fn_009a3b20(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(0x14);
            e.call(
                MEMMOVE,
                &args![
                    offset.wrapping_add(destination),
                    offset.wrapping_add(source),
                    0x14u32
                ],
            );
            index = index.wrapping_add(1);
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1);
        while index as i32 >= 0 {
            let offset = index.wrapping_mul(0x14);
            e.call(
                MEMMOVE,
                &args![
                    offset.wrapping_add(destination),
                    offset.wrapping_add(source),
                    0x14u32
                ],
            );
            index = index.wrapping_sub(1);
        }
    }
}

/// `006e6da0` (unnamed): `cdecl`, size and block; returns the block (placement).
const PLACE_BLOCK: u32 = 0x006e_6da0;
/// `0098fda0` (unnamed): constructs one 0x14-byte item (`ECX` only).
const CONSTRUCT_ITEM: u32 = 0x0098_fda0;

// Translated from 009a3a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a3a80 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is stored but never read):
/// constructs `count` 0x14-byte items at `buffer`, each placed by `006e6da0` and, when that
/// gives a block, constructed by `0098fda0`.
pub fn fn_009a3a80(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let item = index.wrapping_mul(0x14).wrapping_add(buffer);
        let placed = e.call(PLACE_BLOCK, &args![0x14u32, item]).u32();
        if placed != 0 {
            e.call(CONSTRUCT_ITEM, &args![placed]);
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_3b20, fn_009a3b20(Ptr, u32, u32, u32)),
        entry!(0x009a_3a80, fn_009a3a80(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn moves(e: &mut Engine, destination: u32, source: u32, count: u32) -> Vec<Vec<u32>> {
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_3b20, &args![0u32, destination, source, count]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 == MEMMOVE)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn fn_009a3b20_moves_forwards_or_backwards() {
        let mut e = Engine::new();
        assert_eq!(
            moves(&mut e, 0x100, 0x200, 2),
            vec![vec![0x100, 0x200, 0x14], vec![0x114, 0x214, 0x14]]
        );
        assert_eq!(
            moves(&mut e, 0x200, 0x100, 2),
            vec![vec![0x214, 0x114, 0x14], vec![0x200, 0x100, 0x14]]
        );
        assert!(moves(&mut e, 0x100, 0x100, 2).is_empty());
        assert!(moves(&mut e, 0x100, 0x200, 0).is_empty());
    }
}

#[cfg(test)]
mod construct_tests {
    use super::*;

    #[test]
    fn fn_009a3a80_places_and_constructs_each_item() {
        let mut e = Engine::new();
        // The placement gives null for the second item, which is then not constructed.
        e.register(PLACE_BLOCK, |_, a| Ret {
            eax: if a[1] == 0x1014 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(CONSTRUCT_ITEM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_3a80, &args![0u32, 0x1000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let placed: Vec<_> = log
            .iter()
            .filter(|call| call.0 == PLACE_BLOCK)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(
            placed,
            vec![vec![0x14, 0x1000], vec![0x14, 0x1014], vec![0x14, 0x1028]]
        );
        let constructed: Vec<_> = log
            .iter()
            .filter(|call| call.0 == CONSTRUCT_ITEM)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(constructed, vec![vec![0x1000], vec![0x1028]]);
    }
}
