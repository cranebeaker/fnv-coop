//! `fallout shared/run_00501b40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00511860` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_00511860: u32 = 0x0051_1860;

// Translated from 00501b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01025968): forwards `this` and one pointer word to
/// `00511860`. Returns nothing: `00511860` has no value-producing path (it runs `00484d60` on
/// `this`, then maybe calls `00865e70` on the word and returns with whatever that call left in
/// `EAX`). Parameter count from the `RET 4` of both.
pub fn fn_00501b90(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_00511860, &args![this, argument]);
}

// Translated from 00501cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x3c` (`SUB ECX,0x3c`) and jumps to `004ffb70`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00501cc0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_fb70, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00501c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// String assignment (no name in the engine map): `BSStringT<char>::Set`
/// (`00438470`, engine map) on `this` with `text`; returns `this`.
pub fn fn_00501c10(e: &mut Engine, this: Ptr, text: u32) -> Ptr {
    e.call(0x0043_8470, &args![this, text]);
    this
}

// Translated from 00501cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor of a six-word object (no name in the engine map): every
/// word 0 except the fifth (+0x10), which is -1, stored in the exe's order
/// (+0, +4, +0xc, +0x10, +0x14, +8). Returns `this`.
pub fn fn_00501cd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0xffff_ffff);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 8, 0);
    this
}

// Translated from 00501b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference (`RET 4`; nothing for null): the extra data list `005d43c0` gives for it
/// goes to `0041bf80` with -1 when `005015d0` answers for `this`, else with -2.
pub fn fn_00501b40(e: &mut Engine, this: Ptr, reference: u32) {
    if reference == 0 {
        return;
    }
    let value = if e.call(0x0050_15d0, &args![this]).bool() {
        0xffff_ffffu32
    } else {
        0xffff_fffe
    };
    let list = e.call(0x005d_43c0, &args![reference]).u32();
    e.call(0x0041_bf80, &args![list, value]);
}

// Translated from 00501bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads from the load buffer `buffer` (`RET 4`): the base load `005118c0`, then, when the
/// change flags (`00428110` into a local word) have 0x80000000 (`004280f0`) and the buffer's
/// vtable slot 0 gives a version below 13, one byte into a local that is not used again
/// (`00864980`).
pub fn fn_00501bb0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0051_18c0, &args![this, buffer]);
    // EBP-0x8: the flags word; EBP-0x1: the byte read.
    e.with_stack(8, |e, frame| {
        let flags = e.call(0x0042_8110, &args![buffer, frame]).u32();
        if !e.call(0x0042_80f0, &args![flags, 0x8000_0000u32]).bool() {
            return;
        }
        let version = e.vcall(buffer, 0, &args![]).u8();
        if i32::from(version) < 0xd {
            e.call(0x0086_4980, &args![buffer, frame.addr() + 7, 1u32]);
        }
    });
}

// Translated from 00501c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The opposite of `00501c60` for `text` (`RET 4`).
pub fn fn_00501c30(e: &mut Engine, this: Ptr, text: u32) -> bool {
    !fn_00501c60(e, this, text)
}

// Translated from 00501c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `00501c90` with 1 answers 0 for `text` (`RET 4`).
pub fn fn_00501c60(e: &mut Engine, this: Ptr, text: u32) -> bool {
    fn_00501c90(e, this, text, 1) == 0
}

// Translated from 00501c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares this string with the one `text` holds (`00559450`) through
/// `BSStringT<char>::StrCmp` with `flag` (`RET 8`), and answers its value.
pub fn fn_00501c90(e: &mut Engine, this: Ptr, text: u32, flag: u8) -> i32 {
    let other = e.call(0x0055_9450, &args![text]).u32();
    e.call(0x0040_8a80, &args![this, other, u32::from(flag)])
        .i32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_1b90, fn_00501b90(Ptr, Ptr)),
        entry!(0x0050_1cc0, fn_00501cc0(Ptr, u32) -> u32),
        entry!(0x0050_1c10, fn_00501c10(Ptr, u32) -> Ptr),
        entry!(0x0050_1cd0, fn_00501cd0(Ptr) -> Ptr),
        entry!(0x0050_1b40, fn_00501b40(Ptr, u32)),
        entry!(0x0050_1bb0, fn_00501bb0(Ptr, u32)),
        entry!(0x0050_1c30, fn_00501c30(Ptr, u32) -> bool),
        entry!(0x0050_1c60, fn_00501c60(Ptr, u32) -> bool),
        entry!(0x0050_1c90, fn_00501c90(Ptr, u32, u8) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00501b90_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_00511860, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_1b90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_00511860)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00501cc0() {
        assert_eq!(check(0x0050_1cc0, 0x004f_fb70, 0x3c), 0xdead_beef);
    }
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    #[test]
    fn fn_00501c10_sets_the_string_and_returns_this() {
        let mut e = Engine::new();
        e.register(0x0043_8470, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0050_1c10, &args![0x1000u32, 0x2000u32]).u32(),
            0x1000
        );
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0043_8470).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x1000, 0x2000]);
    }

    #[test]
    fn fn_00501cd0_clears_the_words_but_the_fifth() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(this + 4 * i, 0x1234);
        }
        assert_eq!(e.call(0x0050_1cd0, &args![this]).u32(), this);
        for i in 0..6 {
            let expected = if i == 4 { 0xffff_ffff } else { 0 };
            assert_eq!(e.mem.u32(this + 4 * i), expected);
        }
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00501b40_passes_minus_one_or_minus_two() {
        let mut e = Engine::new();
        e.register(0x0050_15d0, |_, _| true.into_ret());
        e.register(0x005d_43c0, |_, a| (a[0] + 4).into_ret());
        e.register(0x0041_bf80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_1b40, &args![0x1000u32, 0x2000u32]);
        e.register(0x0050_15d0, |_, _| false.into_ret());
        e.call(0x0050_1b40, &args![0x1000u32, 0x2000u32]);
        e.call(0x0050_1b40, &args![0x1000u32, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0041_bf80),
            vec![vec![0x2004, 0xffff_ffff], vec![0x2004, 0xffff_fffe]]
        );
    }

    #[test]
    fn fn_00501bb0_reads_the_byte_for_old_versions() {
        let mut e = Engine::new();
        e.register(0x0051_18c0, |_, _| Ret::default());
        e.register(0x0042_8110, |_, a| a[1].into_ret());
        e.register(0x0042_80f0, |_, a| (a[1] == 0x8000_0000).into_ret());
        e.register(0x0086_4980, |_, _| Ret::default());
        e.put_vtable(0x00dc_0000, &[0x00e9_0000]);
        e.register(0x00e9_0000, |_, _| 12u8.into_ret());
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x00dc_0000);
        e.call_log = Some(vec![]);
        e.call(0x0050_1bb0, &args![0x1000u32, buffer]);
        assert_eq!(calls_to(&e, 0x0051_18c0), vec![vec![0x1000, buffer]]);
        let read = calls_to(&e, 0x0086_4980);
        assert_eq!((read.len(), read[0][0], read[0][2]), (1, buffer, 1));
        assert_eq!(read[0][1], calls_to(&e, 0x0042_8110)[0][1] + 7);
        e.register(0x00e9_0000, |_, _| 13u8.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_1bb0, &args![0x1000u32, buffer]);
        assert!(calls_to(&e, 0x0086_4980).is_empty());
    }

    #[test]
    fn fn_00501c30_inverts_the_equality() {
        let mut e = Engine::new();
        e.register(FIRST_WORD_TEXT, |_, a| a[0].into_ret());
        e.register(0x0040_8a80, |_, a| ((a[1] != 0x10) as u32).into_ret());
        assert!(!e.call(0x0050_1c30, &args![0x1000u32, 0x10u32]).bool());
        assert!(e.call(0x0050_1c30, &args![0x1000u32, 0x11u32]).bool());
    }

    const FIRST_WORD_TEXT: u32 = 0x0055_9450;

    #[test]
    fn fn_00501c60_is_true_on_a_zero_compare() {
        let mut e = Engine::new();
        e.register(FIRST_WORD_TEXT, |_, a| a[0].into_ret());
        e.register(0x0040_8a80, |_, a| {
            ((a[1] != 0x10) as u32 * 0x100).into_ret()
        });
        assert!(e.call(0x0050_1c60, &args![0x1000u32, 0x10u32]).bool());
        assert!(!e.call(0x0050_1c60, &args![0x1000u32, 0x11u32]).bool());
    }

    #[test]
    fn fn_00501c90_passes_the_flag_byte() {
        let mut e = Engine::new();
        e.register(FIRST_WORD_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(0x0040_8a80, |_, _| (-1i32).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0050_1c90, &args![0x1000u32, 0x10u32, 0x301u32])
                .i32(),
            -1
        );
        assert_eq!(calls_to(&e, 0x0040_8a80), vec![vec![0x1000, 0x11, 1]]);
    }
}
