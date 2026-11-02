//! `fallout shared/run_005fbf70` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fbf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `005f18c0`. `005f18c0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fbf70(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x005f_18c0,
        &args![this.addr().wrapping_sub(0x7c), argument],
    );
}

// Translated from 005fbf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `005fb270`. `005fb270` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005fbf80(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x005f_b270,
        &args![this.addr().wrapping_sub(0x7c), first, second],
    );
}

// Translated from 005fbf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `005f16f0`. `005f16f0` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fbf90(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x005f_16f0,
        &args![this.addr().wrapping_sub(0x90), argument],
    )
    .u16()
}

// Translated from 005fbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xdc (`SUB ECX,0xdc`) and jumps to `005f78b0`. `005f78b0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_005fbfa0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005f_78b0, &args![this.addr().wrapping_sub(0xdc), flags])
        .u32()
}

// Translated from 005fbfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `005fb1e0`. `005fb1e0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fbfb0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x005f_b1e0,
        &args![this.addr().wrapping_sub(0x7c), argument],
    );
}

// Translated from 005fbfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `005f1b30`. `005f1b30` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005fbfc0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x005f_1b30,
        &args![this.addr().wrapping_sub(0x90), first, second],
    );
}

// Translated from 005fbfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `005fb180`. `005fb180` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fbfd0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x005f_b180,
        &args![this.addr().wrapping_sub(0x90), argument],
    )
    .u16()
}

// Translated from 005fbfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `005f16f0`. `005f16f0` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fbfe0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x005f_16f0,
        &args![this.addr().wrapping_sub(0x7c), argument],
    )
    .u16()
}

// Translated from 005fbff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xdc (`SUB ECX,0xdc`) and jumps to `005f75b0`. `005f75b0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_005fbff0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005f_75b0, &args![this.addr().wrapping_sub(0xdc), flags])
        .u32()
}

// Translated from 005fc000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `005f18c0`. `005f18c0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fc000(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x005f_18c0,
        &args![this.addr().wrapping_sub(0x90), argument],
    );
}

// Translated from 005fc010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `005fb270`. `005fb270` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005fc010(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x005f_b270,
        &args![this.addr().wrapping_sub(0x90), first, second],
    );
}

// Translated from 005fc020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `005f1b30`. `005f1b30` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005fc020(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x005f_1b30,
        &args![this.addr().wrapping_sub(0x7c), first, second],
    );
}

// Translated from 005fc030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `005fb180`. `005fb180` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fc030(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x005f_b180,
        &args![this.addr().wrapping_sub(0x7c), argument],
    )
    .u16()
}

// Translated from 005fc040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `005fb1e0`. `005fb1e0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005fc040(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x005f_b1e0,
        &args![this.addr().wrapping_sub(0x90), argument],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_bf70, fn_005fbf70(Ptr, u32)),
        entry!(0x005f_bf80, fn_005fbf80(Ptr, u32, u32)),
        entry!(0x005f_bf90, fn_005fbf90(Ptr, u32) -> u16),
        entry!(0x005f_bfa0, fn_005fbfa0(Ptr, u32) -> u32),
        entry!(0x005f_bfb0, fn_005fbfb0(Ptr, u32)),
        entry!(0x005f_bfc0, fn_005fbfc0(Ptr, u32, u32)),
        entry!(0x005f_bfd0, fn_005fbfd0(Ptr, u32) -> u16),
        entry!(0x005f_bfe0, fn_005fbfe0(Ptr, u32) -> u16),
        entry!(0x005f_bff0, fn_005fbff0(Ptr, u32) -> u32),
        entry!(0x005f_c000, fn_005fc000(Ptr, u32)),
        entry!(0x005f_c010, fn_005fc010(Ptr, u32, u32)),
        entry!(0x005f_c020, fn_005fc020(Ptr, u32, u32)),
        entry!(0x005f_c030, fn_005fc030(Ptr, u32) -> u16),
        entry!(0x005f_c040, fn_005fc040(Ptr, u32)),
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
    fn le81_thunk_005fbf70() {
        check(0x005f_bf70, 0x005f_18c0, 0x7c, &[0x77]);
    }

    #[test]
    fn le81_thunk_005fbf80() {
        check(0x005f_bf80, 0x005f_b270, 0x7c, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005fbf90() {
        assert_eq!(check(0x005f_bf90, 0x005f_16f0, 0x90, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_005fbfa0() {
        assert_eq!(
            check(0x005f_bfa0, 0x005f_78b0, 0xdc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_005fbfb0() {
        check(0x005f_bfb0, 0x005f_b1e0, 0x7c, &[0x77]);
    }

    #[test]
    fn le81_thunk_005fbfc0() {
        check(0x005f_bfc0, 0x005f_1b30, 0x90, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005fbfd0() {
        assert_eq!(check(0x005f_bfd0, 0x005f_b180, 0x90, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_005fbfe0() {
        assert_eq!(check(0x005f_bfe0, 0x005f_16f0, 0x7c, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_005fbff0() {
        assert_eq!(
            check(0x005f_bff0, 0x005f_75b0, 0xdc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_005fc000() {
        check(0x005f_c000, 0x005f_18c0, 0x90, &[0x77]);
    }

    #[test]
    fn le81_thunk_005fc010() {
        check(0x005f_c010, 0x005f_b270, 0x90, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005fc020() {
        check(0x005f_c020, 0x005f_1b30, 0x7c, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005fc030() {
        assert_eq!(check(0x005f_c030, 0x005f_b180, 0x7c, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_005fc040() {
        check(0x005f_c040, 0x005f_b1e0, 0x90, &[0x77]);
    }
}
