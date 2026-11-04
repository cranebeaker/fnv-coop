//! `fallout shared/run_00508d70` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00508e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508e00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x27 (0x94 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00508e00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.mem.set_u32(base, 0x0102_6d0c);
    e.mem.set_u32(base + 0x30, 0x0102_6cf4);
    e.mem.set_u32(base + 0x3c, 0x0102_6cd0);
    e.mem.set_u32(base + 0x5c, 0x0102_6cbc);
    e.mem.set_u32(base + 0x68, 0x0102_6ca8);
    e.mem.set_u32(base + 0x70, 0x0102_6c98);
    e.call(0x004f_15a0, &args![this, 0x27]);
    e.call(0x0050_8f70, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 00508db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x48 (`SUB ECX,0x48`) and jumps to `00508dc0`. `00508dc0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00508db0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_8dc0, &args![this.addr().wrapping_sub(0x48), flags])
        .u32()
}

// Translated from 00508df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00508dc0`. `00508dc0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00508df0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_8dc0, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00508f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall`, no stack arguments: clears
/// the word at `this + 0x90`, then calls `00510710` on the same object (the
/// constructor `00508e00` calls it). Returns nothing.
pub fn fn_00508f70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x90, 0);
    e.call(0x0051_0710, &args![this]);
}

// Translated from 00508dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00508510` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00508dc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_8510, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00508eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00508ee0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00508eb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_8ee0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00508d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x2000 of the flags word at +8 is set (the engine map gives
/// it no name).
pub fn fn_00508d70(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & 0x2000 != 0
}

// Translated from 00508d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + index + 8` (the engine map gives it no name).
pub fn fn_00508d90(e: &mut Engine, this: Ptr, index: u32) -> u8 {
    e.mem.u8(this.addr().wrapping_add(index).wrapping_add(8))
}

// Translated from 00508ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508ee0 - C++ exception unwinding (FS:[0] frame) not translated
/// The destructor body of the form `00508e00` constructs (the class is
/// unconfirmed): the six vtable pointers `00508e00` stores, then the
/// component destructor `00483710` and the base destructor `00510640`.
pub fn fn_00508ee0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_6d0c);
    e.mem.set_u32(base + 0x30, 0x0102_6cf4);
    e.mem.set_u32(base + 0x3c, 0x0102_6cd0);
    e.mem.set_u32(base + 0x5c, 0x0102_6cbc);
    e.mem.set_u32(base + 0x68, 0x0102_6ca8);
    e.mem.set_u32(base + 0x70, 0x0102_6c98);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0051_0640, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_8e00, fn_00508e00(Ptr) -> Ptr),
        entry!(0x0050_8db0, fn_00508db0(Ptr, u32) -> u32),
        entry!(0x0050_8df0, fn_00508df0(Ptr, u32) -> u32),
        entry!(0x0050_8f70, fn_00508f70(Ptr)),
        entry!(0x0050_8dc0, fn_00508dc0(Ptr, u32) -> Ptr),
        entry!(0x0050_8eb0, fn_00508eb0(Ptr, u32) -> Ptr),
        entry!(0x00508d70, fn_00508d70(Ptr) -> bool),
        entry!(0x00508d90, fn_00508d90(Ptr, u32) -> u8),
        entry!(0x00508ee0, fn_00508ee0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00508e00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_8e00,
            size: 0x94,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x27)]),
                (0x0050_8f70, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_6d0c),
                (0x30, 4, 0x0102_6cf4),
                (0x3c, 4, 0x0102_6cd0),
                (0x5c, 4, 0x0102_6cbc),
                (0x68, 4, 0x0102_6ca8),
                (0x70, 4, 0x0102_6c98),
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
    fn le81_thunk_00508db0() {
        assert_eq!(
            check(0x0050_8db0, 0x0050_8dc0, 0x48, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00508df0() {
        assert_eq!(
            check(0x0050_8df0, 0x0050_8dc0, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod clear_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00508f70_clears_the_word_and_calls_00510710() {
        let mut e = Engine::new();
        e.register(0x0051_0710, |_, _| Ret::default());
        let this = e.mem.alloc(0x94);
        e.mem.write(this, &[0xcd; 0x94]);
        e.call_log = Some(vec![]);
        e.call(0x0050_8f70, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0051_0710, vec![this])]);
        let bytes = e.mem.bytes(this, 0x94);
        assert_eq!(bytes[0x90..0x94], [0; 4]);
        assert!(bytes[..0x90].iter().all(|b| *b == 0xcd));
    }
    #[test]
    fn at_00508dc0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_8510, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_8dc0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_8dc0)
                .collect();
            let mut expected = vec![(0x0050_8510, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00508eb0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_8ee0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_8eb0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_8eb0)
                .collect();
            let mut expected = vec![(0x0050_8ee0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn flag_0x2000_and_byte_at_index() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        assert!(!e.call(0x0050_8d70, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x2000);
        assert!(e.call(0x0050_8d70, &args![this]).bool());
        e.mem.set_u8(this + 0xb, 0x5a);
        assert_eq!(e.call(0x0050_8d90, &args![this, 3u32]).u8(), 0x5a);
    }

    #[test]
    fn destructor_body_restores_the_vtables() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |_, _| Ret::default());
        e.register(0x0051_0640, |_, _| Ret::default());
        let this = e.mem.alloc(0x94);
        e.call_log = Some(vec![]);
        e.call(0x0050_8ee0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_6d0c);
        assert_eq!(e.mem.u32(this + 0x70), 0x0102_6c98);
        let log = e.call_log.take().unwrap();
        let callees: Vec<_> = log.iter().skip(1).map(|(a, w)| (*a, w.clone())).collect();
        assert_eq!(
            callees,
            vec![(0x0048_3710, vec![this]), (0x0051_0640, vec![this])]
        );
    }
}
