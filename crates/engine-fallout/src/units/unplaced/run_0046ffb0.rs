//! `(unplaced)/run_0046ffb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Releases every node chained at `+4` of the container and clears its `+0`
/// word (`00470470`).
const CLEAR_CHAIN: u32 = 0x0047_0470;

// Translated from 0046ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the chained container: empties it (`00470470`).
pub fn fn_0046ffb0(e: &mut Engine, this: Ptr) {
    e.call(CLEAR_CHAIN, &args![this]);
}

/// `004ede70` (`cdecl`, one word): frees the block held at +4.
const FREE_HELD_BLOCK: u32 = 0x004e_de70;

// Translated from 0046ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `01019a14` and passes the word at +4 to `004ede70`.
pub fn fn_0046ffd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_9a14);
    let block = e.mem.u32(this.addr() + 4);
    e.call(FREE_HELD_BLOCK, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0046ffb0, fn_0046ffb0(Ptr)),
        entry!(0x0046_ffd0, fn_0046ffd0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_the_container() {
        let mut e = Engine::new();
        e.register(CLEAR_CHAIN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0046_ffb0, &args![0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (CLEAR_CHAIN, vec![0x1234]));
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_0046ffd0_sets_the_vtable_and_frees_the_block() {
        let mut e = Engine::new();
        e.register(FREE_HELD_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0046_ffd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(FREE_HELD_BLOCK, vec![0x6000])]);
        assert_eq!(e.mem.u32(this), 0x0101_9a14);
    }
}
