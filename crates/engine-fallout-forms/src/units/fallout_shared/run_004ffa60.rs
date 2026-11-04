//! `fallout shared/run_004ffa60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004ffaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ffaa0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x17 (0xa8 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004ffaa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.call(0x0048_2c50, &args![base + 0x90]);
    e.mem.set_u32(base, 0x0102_5914);
    e.mem.set_u32(base + 0x30, 0x0102_58fc);
    e.mem.set_u32(base + 0x3c, 0x0102_58d8);
    e.mem.set_u32(base + 0x5c, 0x0102_58c4);
    e.mem.set_u32(base + 0x68, 0x0102_58b0);
    e.mem.set_u32(base + 0x70, 0x0102_58a0);
    e.mem.set_u32(base + 0x90, 0x0102_5888);
    e.call(0x0096_a2d0, &args![base + 0x98]);
    e.call(0x004f_15a0, &args![this, 0x17]);
    e.call(0x004f_fc50, &args![this]);
    this
}

// Translated from 004ffa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `004fe6e0`. `004fe6e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004ffa90(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_e6e0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 004ffb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004ffba0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004ffb70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_fba0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004ffa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (its one caller is `004ff980`): copies the `float` `source`
/// points at into the first word of `this` (`FLD`/`FSTP`, a signalling NaN
/// coming out quiet), clears the second word and returns `this`; `RET 4`.
pub fn fn_004ffa60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(this.addr(), value);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 004ffba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ffba0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `004ffb70` runs (vtable `01025914`, RTTI
/// `.?AVBGSTerminal@@`): stores the object's and its components' vtables as
/// the constructor `004ffaa0` does, runs `004ffc90`, destroys the list at
/// `+0x98` (`0046ffb0`) and runs `00510640` on the object.
pub fn fn_004ffba0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_5914);
    e.mem.set_u32(base + 0x30, 0x0102_58fc);
    e.mem.set_u32(base + 0x3c, 0x0102_58d8);
    e.mem.set_u32(base + 0x5c, 0x0102_58c4);
    e.mem.set_u32(base + 0x68, 0x0102_58b0);
    e.mem.set_u32(base + 0x70, 0x0102_58a0);
    e.mem.set_u32(base + 0x90, 0x0102_5888);
    fn_004ffc90(e, this);
    e.call(0x0046_ffb0, &args![base + 0x98]);
    e.call(0x0051_0640, &args![this]);
}

// Translated from 004ffc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start values of the constructor `004ffaa0`: `00510710` on the
/// object, the word at `+0xA0` cleared, and 4 bytes at `+0xA4` set to 0
/// (`00403d30`, cdecl, a `memset` wrapper the engine map names
/// `std::_Fiopen` through identical-code folding).
pub fn fn_004ffc50(e: &mut Engine, this: Ptr) {
    e.call(0x0051_0710, &args![this]);
    e.mem.set_u32(this.addr() + 0xa0, 0);
    e.call(0x0040_3d30, &args![this.addr() + 0xa4, 0u32, 4u32]);
}

// Translated from 004ffc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00483710` and `00501050` on the object (the destructor body
/// `004ffba0` calls it first; the engine map's `PreSubclassWindow` name is
/// a folded body's).
pub fn fn_004ffc90(e: &mut Engine, this: Ptr) {
    e.call(0x0048_3710, &args![this]);
    e.call(0x0050_1050, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_faa0, fn_004ffaa0(Ptr) -> Ptr),
        entry!(0x004f_fa90, fn_004ffa90(Ptr, u32) -> u32),
        entry!(0x004f_fb70, fn_004ffb70(Ptr, u32) -> Ptr),
        entry!(0x004f_fa60, fn_004ffa60(Ptr, Ptr) -> Ptr),
        entry!(0x004f_fba0, fn_004ffba0(Ptr)),
        entry!(0x004f_fc50, fn_004ffc50(Ptr)),
        entry!(0x004f_fc90, fn_004ffc90(Ptr)),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != skip)
            .collect()
    }

    #[test]
    fn copies_the_float_and_clears_the_second_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        let source = e.mem.alloc(4);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        e.mem.set_f32(source, -2.5);
        assert_eq!(e.call(0x004f_fa60, &[this, source]).u32(), this);
        assert_eq!(e.mem.f32(this), -2.5);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.mem.set_u32(source, 0xff80_0001);
        e.call(0x004f_fa60, &[this, source]);
        assert_eq!(e.mem.u32(this), 0xffc0_0001);
    }

    #[test]
    fn destructor_body_start_values_and_teardown() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x0048_3710,
                0x0050_1050,
                0x0046_ffb0,
                0x0051_0640,
                0x0051_0710,
                0x0040_3d30,
            ],
        );
        let this = e.mem.alloc(0xa8);
        e.call_log = Some(vec![]);
        e.call(0x004f_fba0, &[this]);
        assert_eq!(
            calls(&mut e, 0x004f_fba0),
            vec![
                (0x0048_3710, vec![this]),
                (0x0050_1050, vec![this]),
                (0x0046_ffb0, vec![this + 0x98]),
                (0x0051_0640, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_5914);
        assert_eq!(e.mem.u32(this + 0x90), 0x0102_5888);
        assert_eq!(e.mem.u32(this + 0x70), 0x0102_58a0);

        e.mem.set_u32(this + 0xa0, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x004f_fc50, &[this]);
        assert_eq!(
            calls(&mut e, 0x004f_fc50),
            vec![
                (0x0051_0710, vec![this]),
                (0x0040_3d30, vec![this + 0xa4, 0, 4])
            ]
        );
        assert_eq!(e.mem.u32(this + 0xa0), 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004ffaa0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_faa0,
            size: 0xa8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x0048_2c50, &[A::T(0x90)]),
                (0x0096_a2d0, &[A::T(0x98)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x17)]),
                (0x004f_fc50, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_5914),
                (0x30, 4, 0x0102_58fc),
                (0x3c, 4, 0x0102_58d8),
                (0x5c, 4, 0x0102_58c4),
                (0x68, 4, 0x0102_58b0),
                (0x70, 4, 0x0102_58a0),
                (0x90, 4, 0x0102_5888),
            ],
        });
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
    fn le81_thunk_004ffa90() {
        assert_eq!(
            check(0x004f_fa90, 0x004f_e6e0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_004ffb70_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_fba0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_fb70, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_fb70)
                .collect();
            let mut expected = vec![(0x004f_fba0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
