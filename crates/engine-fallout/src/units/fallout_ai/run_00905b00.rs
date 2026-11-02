//! `fallout/ai/run_00905b00` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00905b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `008df980`. `008df980` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00905b90(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x008d_f980, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00905b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00905b00 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A constructor (no name in the engine map; `RET 4`): the base part
/// (`00483710`), then placement `new` of 8 bytes at `this` (`006e6da0`)
/// and, when that gives an address, `006e9c30` on it with `argument`.
pub fn fn_00905b00(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_3710, &args![this]);
    let place = e.call(0x006e_6da0, &args![8u32, this]).u32();
    if place != 0 {
        e.call(0x006e_9c30, &args![place, argument]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0090_5b90, fn_00905b90(Ptr, u32) -> u32),
        entry!(0x0090_5b00, fn_00905b00(Ptr, u32)),
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
    fn le81_thunk_00905b90() {
        assert_eq!(
            check(0x0090_5b90, 0x008d_f980, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    #[test]
    fn fn_00905b00_builds_the_part_at_this() {
        for place in [0u32, 0x6000] {
            let mut e = Engine::new();
            e.register(0x0048_3710, |_, _| Ret::default());
            e.register_double(0x006e_6da0, move |_, _| Ret {
                eax: place,
                ..Ret::default()
            });
            e.register(0x006e_9c30, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0090_5b00, &args![0x5000u32, 0x77u32]);
            let log = e.call_log.take().unwrap();
            let calls = |target: u32| {
                log.iter()
                    .filter(|(a, _)| *a == target)
                    .map(|(_, w)| w.clone())
                    .collect::<Vec<_>>()
            };
            assert_eq!(calls(0x0048_3710), vec![vec![0x5000]]);
            assert_eq!(calls(0x006e_6da0), vec![vec![8, 0x5000]]);
            if place == 0 {
                assert!(calls(0x006e_9c30).is_empty());
            } else {
                assert_eq!(calls(0x006e_9c30), vec![vec![0x6000, 0x77]]);
            }
        }
    }
}
