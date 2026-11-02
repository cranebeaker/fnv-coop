//! `Havok SDK/run_00dd0420` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function that fills the table the constructor leaves at `this + 0x1c` (`00d51760`, cdecl:
/// the table, the object).
const FILL_TABLE: u32 = 0x00d5_1760;

// Translated from 00dd0420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `flag`). Does nothing but answer
/// `this` when `flag` is 0. Otherwise clears the words at `+0`, `+8` and `+0xc`, sets the word
/// at `+4` to 0x10, has `00d51760` fill the table at `this + 0x1c` for the object (arguments:
/// the table, `this`) and copies the word at `+4` to `+0`. Answers `this`.
pub fn fn_00dd0420(e: &mut Engine, this: u32, flag: u32) -> u32 {
    if flag != 0 {
        e.mem.set_u32(this, 0);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 4, 0x10);
        e.call(FILL_TABLE, &args![this + 0x1c, this]);
        let word = e.mem.u32(this + 4);
        e.mem.set_u32(this, word);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dd0420, fn_00dd0420(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn a_zero_flag_changes_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, 0x77);
        assert_eq!(e.call(0x00dd_0420, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 0x77);
    }

    #[test]
    fn a_flag_initializes_the_object_and_has_its_table_filled() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        for word in 0..0x10 {
            e.mem.set_u32(this + 4 * word, 0xdead_0000);
        }
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(FILL_TABLE, move |e, a| {
            // The words `0` and `4` as the filler sees them.
            let mut words = a.to_vec();
            words.push(e.mem.u32(a[1]));
            words.push(e.mem.u32(a[1] + 4));
            seen.borrow_mut().push(words);
            Ret::default()
        });
        assert_eq!(e.call(0x00dd_0420, &args![this, 1u32]).u32(), this);
        assert_eq!(*log.borrow(), vec![vec![this + 0x1c, this, 0, 0x10]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0x10, 0x10, 0, 0]);
    }
}
