//! `(unplaced)/run_00864100` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004ede70` (`cdecl`, one word): frees the block held at +4.
const FREE_HELD_BLOCK: u32 = 0x004e_de70;

// Translated from 00864100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `01081fc8` and passes the word at +4 to `004ede70`.
pub fn fn_00864100(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_1fc8);
    let block = e.mem.u32(this.addr() + 4);
    e.call(FREE_HELD_BLOCK, &args![block]);
}

/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00864130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the scalar deleting destructor over
/// [`fn_00864100`]: `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00864130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00864100(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_4100, fn_00864100(Ptr)),
        entry!(0x0086_4130, fn_00864130(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_and_frees_the_block() {
        let mut e = Engine::new();
        e.register(FREE_HELD_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0086_4100, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(FREE_HELD_BLOCK, vec![0x6000])]);
        assert_eq!(e.mem.u32(this), 0x0108_1fc8);
    }
}

#[cfg(test)]
mod tests_n077 {
    use super::*;

    #[test]
    fn fn_00864130_destroys_and_deletes_with_bit_0() {
        let mut e = Engine::new();
        e.register(FREE_HELD_BLOCK, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_4130, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(FREE_HELD_BLOCK, vec![0x6000])]);
        e.call_log = Some(vec![]);
        e.call(0x0086_4130, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (FREE_HELD_BLOCK, vec![0x6000]),
                (OPERATOR_DELETE, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0108_1fc8);
    }
}
