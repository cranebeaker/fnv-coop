//! `fallout/ai/run_008d05c0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d05e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty method (no name in the engine map; it sits in a vtable):
/// `thiscall` with three stack words (`RET 0xc`), none of them read.
pub fn fn_008d05e0(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
}

// Translated from 008d05c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the object passed by value from the second stack word on
/// (`00483710` on the address of that word; `RET 0x10`, four words): the
/// three words from the second on are laid out as the exe has them on the
/// stack and the destructor gets their address. The first word is not read.
pub fn fn_008d05c0(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    value_1: u32,
    value_2: u32,
    value_3: u32,
) {
    e.with_stack(0xc, |e, words| {
        for (i, word) in [value_1, value_2, value_3].into_iter().enumerate() {
            e.mem.set_u32(words.addr() + 4 * i as u32, word);
        }
        e.call(0x0048_3710, &args![words]);
    });
}

// Translated from 008d05f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0.0 (`FLDZ`); the stack word (`RET 4`) is not read.
pub fn fn_008d05f0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> f32 {
    0.0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_05e0, fn_008d05e0(Ptr, u32, u32, u32)),
        entry!(0x008d_05c0, fn_008d05c0(Ptr, u32, u32, u32, u32)),
        entry!(0x008d_05f0, fn_008d05f0(Ptr, u32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_008d05e0_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        let r = e.call(0x008d_05e0, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(r, Ret::default());
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_008d05c0_destroys_the_value_from_the_second_word() {
        let mut e = Engine::new();
        let seen = e.mem.alloc(12);
        e.register_double(0x0048_3710, move |e, a| {
            let words = e.mem.bytes(a[0], 12);
            e.mem.write(seen, &words);
            Ret::default()
        });
        e.call(0x008d_05c0, &[0x9000, 1, 0x22, 0x33, 0x44]);
        assert_eq!(e.mem.u32(seen), 0x22);
        assert_eq!(e.mem.u32(seen + 4), 0x33);
        assert_eq!(e.mem.u32(seen + 8), 0x44);
    }

    #[test]
    fn fn_008d05f0_answers_zero() {
        let mut e = Engine::new();
        let r = e.call(0x008d_05f0, &[0x9000, 5]);
        assert_eq!(r.f64().to_bits(), 0);
    }
}
