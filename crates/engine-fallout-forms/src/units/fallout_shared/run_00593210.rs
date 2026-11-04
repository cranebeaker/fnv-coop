//! `fallout shared/run_00593210` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00593230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00592380`. `00592380` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00593230(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0059_2380, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

/// `float` constants the constructor copies (`.rdata`); each copy is an `FLD`/`FSTP` pair.
const CONSTANT_0102918C: u32 = 0x0102_918c;
const CONSTANT_0101E340: u32 = 0x0101_e340;
const CONSTANT_0101E570: u32 = 0x0101_e570;

/// Copies the `float` at `source` to `destination` as `FLD`/`FSTP` does (a signalling NaN
/// comes out quiet).
fn copy_float(e: &mut Engine, source: u32, destination: u32) {
    let bits = x87_quiet_bits(e.global::<u32>(source));
    e.mem.set_u32(destination, bits);
}

// Translated from 00593240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor that fills a 0x24-byte record with constants:
/// the floats at `+0x08` and `+0x00` from `0102918c`, `+0x0c`, `+0x04` and `+0x10` from
/// `0101e340`, `+0x14` from `0101e570`, `1.0` at `+0x18`, the byte 4 at `+0x1c` and
/// `0xffffffff` at `+0x20`. Returns `this`.
pub fn fn_00593240(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    copy_float(e, CONSTANT_0102918C, base + 0x8);
    copy_float(e, CONSTANT_0101E340, base + 0xc);
    copy_float(e, CONSTANT_0102918C, base);
    copy_float(e, CONSTANT_0101E340, base + 0x4);
    e.mem.set_u8(base + 0x1c, 4);
    e.mem.set_f32(base + 0x18, 1.0);
    copy_float(e, CONSTANT_0101E340, base + 0x10);
    copy_float(e, CONSTANT_0101E570, base + 0x14);
    e.mem.set_u32(base + 0x20, 0xffff_ffff);
    this
}

/// The word `00593210` clears and `00593220` reads.
const VALUE_011CA898: u32 = 0x011c_a898;

// Translated from 00593210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): clears the word at `011ca898`.
pub fn fn_00593210(e: &mut Engine) {
    e.set_global(VALUE_011CA898, 0u32);
}

// Translated from 00593220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at `011ca898`.
pub fn fn_00593220(e: &mut Engine) -> u32 {
    e.global(VALUE_011CA898)
}

// Translated from 005932b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): when `source` is not null,
/// copies 0x24 bytes from it over this record through `00401460` (cdecl: destination,
/// source, size).
pub fn fn_005932b0(e: &mut Engine, this: Ptr, source: Ptr) {
    if !source.is_null() {
        e.call(0x0040_1460, &args![this, source, 0x24u32]);
    }
}

/// `memcmp` (`00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;

// Translated from 005932e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The decal data's comparison (`thiscall`, one word, `RET 4`): true for a
/// null `other`, else whether `memcmp` finds their 0x24 bytes different.
pub fn fn_005932e0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if other == 0 {
        return true;
    }
    e.call(MEMCMP, &args![this, other, 0x24u32]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_3230, fn_00593230(Ptr, u32) -> u32),
        entry!(0x0059_3240, fn_00593240(Ptr) -> Ptr),
        entry!(0x0059_3210, fn_00593210()),
        entry!(0x0059_3220, fn_00593220() -> u32),
        entry!(0x0059_32b0, fn_005932b0(Ptr, Ptr)),
        entry!(0x0059_32e0, fn_005932e0(Ptr, u32) -> bool),
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
    fn le81_thunk_00593230() {
        assert_eq!(
            check(0x0059_3230, 0x0059_2380, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    #[test]
    fn fills_the_record_and_quiets_signalling_nans() {
        let mut e = Engine::new();
        e.map(0x0102_9000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.set_global(CONSTANT_0102918C, 0x7f80_0001u32);
        e.set_global(CONSTANT_0101E340, 0x3e80_0000u32);
        e.set_global(CONSTANT_0101E570, 0xff80_0002u32);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0xaaaa_aaaa);
        assert_eq!(e.call(0x0059_3240, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x4), 0x3e80_0000);
        assert_eq!(e.mem.u32(this + 0x8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0xc), 0x3e80_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0x3e80_0000);
        assert_eq!(e.mem.u32(this + 0x14), 0xffc0_0002);
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert_eq!(e.mem.u8(this + 0x1c), 4);
        assert_eq!(e.mem.u32(this + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x24), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00593210_and_fn_00593220_clear_and_read_the_word() {
        let mut e = Engine::new();
        e.map(VALUE_011CA898, 4);
        e.set_global(VALUE_011CA898, 0x1234u32);
        assert_eq!(e.call(0x0059_3220, &args![]).u32(), 0x1234);
        e.call(0x0059_3210, &args![]);
        assert_eq!(e.global::<u32>(VALUE_011CA898), 0);
        assert_eq!(e.call(0x0059_3220, &args![]).u32(), 0);
    }

    #[test]
    fn fn_005932b0_copies_from_a_source_only() {
        for source in [0u32, 0x6000] {
            let mut e = Engine::new();
            e.register(0x0040_1460, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0059_32b0, &args![0x5000u32, source]);
            let log = e.call_log.take().unwrap();
            if source == 0 {
                assert_eq!(log.len(), 1);
            } else {
                assert_eq!(log[1..], [(0x0040_1460, vec![0x5000, source, 0x24])]);
            }
        }
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_005932e0_compares_the_decal_data() {
        let mut e = Engine::new();
        e.register(MEMCMP, |e, a| {
            assert_eq!(a[2], 0x24);
            Ret {
                eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x24);
        let other = e.mem.alloc(0x24);
        assert!(e.call(0x0059_32e0, &[this, 0]).bool());
        assert!(!e.call(0x0059_32e0, &[this, other]).bool());
        e.mem.set_u32(other, 1);
        assert!(e.call(0x0059_32e0, &[this, other]).bool());
    }
}
