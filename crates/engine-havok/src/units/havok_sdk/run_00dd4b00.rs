//! `Havok SDK/run_00dd4b00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dd4b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a bare `RET` (no arguments, no result).
pub fn fn_00dd4b00(_e: &mut Engine) {}

// Translated from 00dd4b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). Copies the six words at
/// `source` to `this + 8` through `this + 0x1c`.
pub fn fn_00dd4b10(e: &mut Engine, this: u32, source: u32) {
    for i in 0..6 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(this + 8 + 4 * i, word);
    }
}

// Translated from 00dd4b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). Copies the six words at
/// `source` to `this + 0x20` through `this + 0x34`.
pub fn fn_00dd4b40(e: &mut Engine, this: u32, source: u32) {
    for i in 0..6 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(this + 0x20 + 4 * i, word);
    }
}

// Translated from 00dd4b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `object`). With `n` the answer of
/// `object`'s virtual function at `+0x10` plus the word at `this + 0xc`, and `m` the constant
/// 0x1000 when the byte at `this + 0x1c` is not 0 (else `n`), answers `m * 0xe0 + n * 0x14`.
pub fn fn_00dd4b80(e: &mut Engine, this: u32, object: u32) -> i32 {
    let first = e.vcall(object, 0x10, &args![]).i32();
    let n = first.wrapping_add(e.mem.i32(this + 0xc));
    let m = if e.mem.u8(this + 0x1c) != 0 {
        0x1000
    } else {
        n
    };
    m.wrapping_mul(0xe0).wrapping_add(n.wrapping_mul(0x14))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd4b00, fn_00dd4b00()),
        entry!(0x00dd4b10, fn_00dd4b10(u32, u32)),
        entry!(0x00dd4b40, fn_00dd4b40(u32, u32)),
        entry!(0x00dd4b80, fn_00dd4b80(u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bare_return_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00dd_4b00, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    fn source(e: &mut Engine) -> u32 {
        let at = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(at + 4 * i, 0x10 + i);
        }
        at
    }

    #[test]
    fn six_words_are_copied_to_the_two_offsets() {
        let mut e = Engine::new();
        let src = source(&mut e);
        let this = e.mem.alloc(0x60);
        e.call(0x00dd_4b10, &args![this, src]);
        e.call(0x00dd_4b40, &args![this, src]);
        let first: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 8 + 4 * i)).collect();
        assert_eq!(first, vec![0x10, 0x11, 0x12, 0x13, 0x14, 0x15]);
        let second: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 0x20 + 4 * i)).collect();
        assert_eq!(second, vec![0x10, 0x11, 0x12, 0x13, 0x14, 0x15]);
        // Nothing before the first block or after the second is written.
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0x38)), (0, 0));
    }

    #[test]
    fn the_size_uses_the_object_count_and_the_fixed_capacity_flag() {
        let mut e = Engine::new();
        e.put_vtable(0x0075_0000, &[0, 0, 0, 0, 0x0075_0100]);
        e.register(0x0075_0100, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0075_0000);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 3);
        // n = 10: 10 * 0xe0 + 10 * 0x14.
        assert_eq!(e.call(0x00dd_4b80, &args![this, object]).i32(), 2440);
        e.mem.set_u8(this + 0x1c, 1);
        assert_eq!(
            e.call(0x00dd_4b80, &args![this, object]).i32(),
            0x1000 * 0xe0 + 200
        );
    }
}
