//! `fallout shared/run_00525c00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00525c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `0051dad0`. `0051dad0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00525c00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_dad0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00525c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `00523080`. `00523080` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_00525c10(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0052_3080,
        &args![this.addr().wrapping_sub(0x84), argument],
    );
}

// Translated from 00525c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `00523020`. `00523020` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_00525c20(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0052_3020,
        &args![this.addr().wrapping_sub(0x84), argument],
    )
    .u16()
}

// Translated from 00525c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `005230b0`. `005230b0` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_00525c30(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0052_30b0,
        &args![this.addr().wrapping_sub(0x84), first, second],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_5c00, fn_00525c00(Ptr, u32) -> u32),
        entry!(0x0052_5c10, fn_00525c10(Ptr, u32)),
        entry!(0x0052_5c20, fn_00525c20(Ptr, u32) -> u16),
        entry!(0x0052_5c30, fn_00525c30(Ptr, u32, u32)),
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
    fn le81_thunk_00525c00() {
        assert_eq!(
            check(0x0052_5c00, 0x0051_dad0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00525c10() {
        check(0x0052_5c10, 0x0052_3080, 0x84, &[0x77]);
    }

    #[test]
    fn le81_thunk_00525c20() {
        assert_eq!(check(0x0052_5c20, 0x0052_3020, 0x84, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_00525c30() {
        check(0x0052_5c30, 0x0052_30b0, 0x84, &[0x77, 0x88]);
    }
}
