//! `fallout shared/magic/run_00408b20` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00408b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `00407200`. `00407200` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00408b40(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0040_7200, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

/// `strcmp` (`00ec6da0`, cdecl).
const STRCMP: u32 = 0x00ec_6da0;

// Translated from 00408b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: two strings): returns `strcmp(first, second)` (the result is left
/// in `EAX`).
pub fn fn_00408b20(e: &mut Engine, first: u32, second: u32) -> i32 {
    e.call(STRCMP, &args![first, second]).i32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_8b40, fn_00408b40(Ptr, u32) -> u32),
        entry!(0x0040_8b20, fn_00408b20(u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00408b20_returns_strcmp_of_its_arguments() {
        let mut e = Engine::new();
        e.register(STRCMP, |_, a| Ret {
            eax: a[0].wrapping_sub(a[1]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_8b20, &args![0x10u32, 0x30u32]).i32(), -0x20);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == STRCMP).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x10, 0x30]);
    }
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
    fn le81_thunk_00408b40() {
        assert_eq!(
            check(0x0040_8b40, 0x0040_7200, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
