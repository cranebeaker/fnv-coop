//! `BSHavok/run_00cc2a00` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{aligned_allocate, enter_scope};

/// Size of the block the function creates.
const BLOCK_SIZE: u32 = 0x70;
/// The constructor run on the new block (`0056e0b0`, `this` only).
const BLOCK_CONSTRUCT: u32 = 0x0056_e0b0;
/// The routine told about the new block when the object has a parent (`00c9ecb0`, `this` plus
/// the block).
const ATTACH_BLOCK: u32 = 0x00c9_ecb0;

// Translated from 00cc2a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2a00 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `fn_00cc2a00` (unnamed in the Xbox PDB): takes the address of a byte. With the
/// allocation-scope word set to 0x10, stores in that byte whether the word at `+0xC` is 0.
/// When it is, allocates a 0x70-byte 16-byte-aligned block from the memory manager, runs
/// `0056e0b0` on it, stores it at `+0xC` and, when the word at `+8` is not 0, runs
/// `00c9ecb0` on `(this, block)`. Puts the scope word back and answers the word at `+0xC`.
pub fn fn_00cc2a00(e: &mut Engine, this: u32, created: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this + 0xc) == 0;
    e.mem.set_u8(created, missing as u8);
    if missing {
        let block = aligned_allocate(e, BLOCK_SIZE);
        e.call(BLOCK_CONSTRUCT, &args![block]);
        e.mem.set_u32(this + 0xc, block);
        if e.mem.u32(this + 8) != 0 {
            e.call(ATTACH_BLOCK, &args![this, block]);
        }
    }
    let answer = e.mem.u32(this + 0xc);
    e.mem.set_u32(word, saved);
    answer
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cc2a00, fn_00cc2a00(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{allocator_doubles, calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::{MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, SCOPE_WORD};

    fn setup() -> (Engine, u32, u32) {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register_double(BLOCK_CONSTRUCT, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        e.register_double(ATTACH_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let flag = e.mem.alloc(4);
        (e, this, flag)
    }

    #[test]
    fn creates_the_block_and_tells_the_parent() {
        let (mut e, this, flag) = setup();
        e.mem.set_u32(this + 8, 0x4444);
        let block = e.call(0x00cc_2a00, &args![this, flag]).u32();
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(e.mem.u32(this + 0xc), block);
        assert_eq!(block % 16, 0);
        let distance = u32::from(e.mem.u8(block - 1));
        assert!((1..=16).contains(&distance));
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            [[MEMORY_MANAGER, BLOCK_SIZE]]
        );
        assert_eq!(calls_to(&e, BLOCK_CONSTRUCT), [[block]]);
        assert_eq!(calls_to(&e, ATTACH_BLOCK), [[this, block]]);
        let word = e.tls() + SCOPE_WORD;
        assert_eq!(e.mem.u32(word), 0);
    }

    #[test]
    fn without_a_parent_nothing_is_attached() {
        let (mut e, this, flag) = setup();
        e.call(0x00cc_2a00, &args![this, flag]);
        assert!(calls_to(&e, ATTACH_BLOCK).is_empty());
        assert_eq!(e.mem.u8(flag), 1);
    }

    #[test]
    fn an_existing_block_is_answered_unchanged() {
        let (mut e, this, flag) = setup();
        e.mem.set_u32(this + 0xc, 0x5555);
        e.mem.set_u8(flag, 9);
        assert_eq!(e.call(0x00cc_2a00, &args![this, flag]).u32(), 0x5555);
        assert_eq!(e.mem.u8(flag), 0);
        assert!(calls_to(&e, BLOCK_CONSTRUCT).is_empty());
    }
}
