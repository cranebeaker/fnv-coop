//! `Havok SDK/run_00cbfba0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cbfc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00dff470`. `00dff470` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00cbfc00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f470, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00cbfba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an object with table pointers at `+0`, `+8`, `+0xc`, `+0x10` and `+0x14` (the
/// class is unconfirmed): sets the 16-bit word at `+6` to 1, writes the first table
/// pointers (`010c3b80`, `010c3b98`, `010c3bac`, `010c3b8c` at `+8` to `+0x14`), then the final
/// ones (`010c92d4` at `+0`, `010c92c8`, `010c92b4`, `010c92a8`, `010c929c` at `+8` to
/// `+0x14`) and zeroes the word at `+0x20`. Returns `this` (it is left in `EAX`).
pub fn fn_00cbfba0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0x010c_3b80);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base, 0x010c_92d4);
    e.mem.set_u32(base + 8, 0x010c_92c8);
    e.mem.set_u32(base + 0xc, 0x010c_92b4);
    e.mem.set_u32(base + 0x10, 0x010c_92a8);
    e.mem.set_u32(base + 0x14, 0x010c_929c);
    e.mem.set_u32(base + 0x20, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_fba0, fn_00cbfba0(Ptr) -> Ptr),
        entry!(0x00cb_fc00, fn_00cbfc00(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests_00cbfba0 {
    use super::*;

    /// The final table words are in place, the word at `+0x20` is cleared and `this` returned.
    #[test]
    fn constructor_writes_the_final_table_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x20, 0xdead_beef);
        let got = e.call(0x00cb_fba0, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        let words: Vec<u32> = [0, 8, 0xc, 0x10, 0x14, 0x20]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![
                0x010c_92d4,
                0x010c_92c8,
                0x010c_92b4,
                0x010c_92a8,
                0x010c_929c,
                0
            ]
        );
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
    fn le81_thunk_00cbfc00() {
        assert_eq!(
            check(0x00cb_fc00, 0x00df_f470, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
