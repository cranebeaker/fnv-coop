//! `(unplaced)/run_00624bb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00624be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4 (`SUB ECX,0x4`) and jumps to `00624190`. `00624190` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00624be0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_4190, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00624bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, plain `RET`; called by `00624720`): `0047b4d0` on
/// `this` with 8, answering its `EAX`.
pub fn fn_00624bb0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0047_b4d0, &args![this, 8u32]).u32()
}

// Translated from 00624bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the counter at `011f942c` (unnamed in the engine map; no arguments, plain
/// `RET`; called by `00624720`).
pub fn fn_00624bd0(e: &mut Engine) {
    e.set_global(0x011f_942cu32, 0u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_4be0, fn_00624be0(Ptr, u32) -> u32),
        entry!(0x0062_4bb0, fn_00624bb0(Ptr) -> u32),
        entry!(0x0062_4bd0, fn_00624bd0()),
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
    fn le81_thunk_00624be0() {
        assert_eq!(
            check(0x0062_4be0, 0x0062_4190, 0x4, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00624bb0_and_00624bd0() {
        let mut e = Engine::new();
        e.register(0x0047_b4d0, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x0062_4bb0, &args![0x1000u32]).u32(), 0x1008);
        e.map(0x011f_9000, 0x1000);
        e.set_global(0x011f_942cu32, 7u32);
        e.call(0x0062_4bd0, &args![]);
        assert_eq!(e.global::<u32>(0x011f_942c), 0);
    }
}
