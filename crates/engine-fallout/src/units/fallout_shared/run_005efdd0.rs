//! `fallout shared/run_005efdd0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005effb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `005edac0`. `005edac0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_005effb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005e_dac0, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

/// `memmove` (`00ec7230`, `cdecl`: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;

// Translated from 005efdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; `this` is stored but never read): moves
/// `count` 16-bit elements from `source` to `destination` one element at a time (`memmove` of
/// 2 bytes each): forwards from the first element when `destination` is below `source`,
/// backwards from the last one when it is above, nothing when they are equal or `count` is 0.
pub fn fn_005efdd0(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(index.wrapping_mul(2)),
                    source.wrapping_add(index.wrapping_mul(2)),
                    2u32
                ],
            );
            index = index.wrapping_add(1);
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1);
        while index as i32 >= 0 {
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(index.wrapping_mul(2)),
                    source.wrapping_add(index.wrapping_mul(2)),
                    2u32
                ],
            );
            index = index.wrapping_sub(1);
        }
    }
}

/// The shape of `005efdd0` for elements of `size` bytes: one `memmove` per
/// element, forwards when `destination` is below `source`, backwards
/// (signed index) when it is above, nothing when equal or `count` is 0.
fn move_elements(e: &mut Engine, destination: u32, source: u32, count: u32, size: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(size);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    size
                ],
            );
            index = index.wrapping_add(1);
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1);
        while index as i32 >= 0 {
            let offset = index.wrapping_mul(size);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    size
                ],
            );
            index = index.wrapping_sub(1);
        }
    }
}

// Translated from 005efe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x38-byte foot IK entries from `source` to `destination`
/// (`thiscall`, `RET 0xC`; `this` is not read; the engine map has no
/// name), as [`fn_005efdd0`] does for 16-bit elements.
pub fn fn_005efe70(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    move_elements(e, destination, source, count, 0x38);
}

// Translated from 005eff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 6-byte grab IK entries from `source` to `destination`
/// (`thiscall`, `RET 0xC`; `this` is not read; the engine map has no
/// name), as [`fn_005efdd0`] does for 16-bit elements.
pub fn fn_005eff10(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    move_elements(e, destination, source, count, 6);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_fe70, fn_005efe70(Ptr, u32, u32, u32)),
        entry!(0x005e_ff10, fn_005eff10(Ptr, u32, u32, u32)),
        entry!(0x005e_ffb0, fn_005effb0(Ptr, u32) -> u32),
        entry!(0x005e_fdd0, fn_005efdd0(Ptr, u32, u32, u32)),
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
    fn le81_thunk_005effb0() {
        assert_eq!(
            check(0x005e_ffb0, 0x005e_dac0, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn moves(e: &mut Engine, destination: u32, source: u32, count: u32) -> Vec<Vec<u32>> {
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_fdd0, &args![0u32, destination, source, count]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 == MEMMOVE)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn fn_005efdd0_moves_forwards_or_backwards() {
        let mut e = Engine::new();
        assert_eq!(
            moves(&mut e, 0x100, 0x200, 2),
            vec![vec![0x100, 0x200, 2], vec![0x102, 0x202, 2]]
        );
        assert_eq!(
            moves(&mut e, 0x200, 0x100, 2),
            vec![vec![0x202, 0x102, 2], vec![0x200, 0x100, 2]]
        );
        assert!(moves(&mut e, 0x100, 0x100, 2).is_empty());
        assert!(moves(&mut e, 0x100, 0x200, 0).is_empty());
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn moves(
        e: &mut Engine,
        addr: u32,
        destination: u32,
        source: u32,
        count: u32,
    ) -> Vec<Vec<u32>> {
        e.call_log = Some(vec![]);
        e.call(addr, &args![0u32, destination, source, count]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == MEMMOVE)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn entries_move_forwards_or_backwards() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        for (addr, size) in [(0x005e_fe70u32, 0x38u32), (0x005e_ff10, 6)] {
            assert_eq!(
                moves(&mut e, addr, 0x1000, 0x2000, 2),
                vec![
                    vec![0x1000, 0x2000, size],
                    vec![0x1000 + size, 0x2000 + size, size]
                ]
            );
            assert_eq!(
                moves(&mut e, addr, 0x2000, 0x1000, 2),
                vec![
                    vec![0x2000 + size, 0x1000 + size, size],
                    vec![0x2000, 0x1000, size]
                ]
            );
            assert!(moves(&mut e, addr, 0x1000, 0x1000, 2).is_empty());
            assert!(moves(&mut e, addr, 0x1000, 0x2000, 0).is_empty());
        }
    }
}
