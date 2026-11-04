//! `fallout shared/pathfinding/run_006e3e70` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `006e3a50`. `006e3a50` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_006e3f10(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x006e_3a50, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 006e3f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `006e3880`. `006e3880` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_006e3f20(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x006e_3880, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// `memmove` (`00ec7230`, cdecl: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;

// Translated from 006e3e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x24-byte elements from `source` to `destination`
/// (`thiscall`, `ECX` not read; `RET 0xc`; no name in the engine map), one
/// `memmove` of 0x24 bytes each: first to last when the destination is
/// below the source, last to first (a signed index, so nothing for a count
/// above 0x8000_0000) when it is above, nothing when they are the same.
pub fn fn_006e3e70(e: &mut Engine, _unused_0: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(0x24);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    0x24u32
                ],
            );
            index += 1;
        }
    } else if source < destination {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(0x24);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    0x24u32
                ],
            );
            index -= 1;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_3f10, fn_006e3f10(Ptr, u32) -> u32),
        entry!(0x006e_3f20, fn_006e3f20(Ptr, u32) -> u32),
        entry!(0x006e_3e70, fn_006e3e70(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_006e3f10() {
        assert_eq!(
            check(0x006e_3f10, 0x006e_3a50, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_006e3f20() {
        assert_eq!(
            check(0x006e_3f20, 0x006e_3880, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn at_006e3e70_moves_in_the_safe_direction() {
        use super::*;
        let moves = |e: &mut Engine, words: &[u32]| -> Vec<Vec<u32>> {
            e.call_log = Some(vec![]);
            e.call(0x006e_3e70, words);
            e.call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == MEMMOVE)
                .map(|c| c.1)
                .collect()
        };
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        assert_eq!(
            moves(&mut e, &[0, 0x1000, 0x2000, 2]),
            vec![vec![0x1000, 0x2000, 0x24], vec![0x1024, 0x2024, 0x24]]
        );
        assert_eq!(
            moves(&mut e, &[0, 0x2000, 0x1000, 2]),
            vec![vec![0x2024, 0x1024, 0x24], vec![0x2000, 0x1000, 0x24]]
        );
        assert!(moves(&mut e, &[0, 0x2000, 0x2000, 2]).is_empty());
        assert!(moves(&mut e, &[0, 0x2000, 0x1000, 0]).is_empty());
        assert!(moves(&mut e, &[0, 0x2000, 0x1000, 0x8000_0001]).is_empty());
    }
}
