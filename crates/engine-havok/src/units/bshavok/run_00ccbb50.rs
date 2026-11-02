//! `BSHavok/run_00ccbb50` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, MEMORY_MANAGER};

/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the manager, one argument).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00ccbb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccbb50 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `fn_00ccbb50` (unnamed in the Xbox PDB): with the allocation-scope word set to 0x10, frees
/// through the memory manager the blocks kept at `+4` and `+0xC` (each only when not 0) and
/// the block at `+0x10` (when not 0; that pointer is 16-byte aligned, and the byte before it is
/// the distance back to the start of the block), then puts the scope word back. The words are
/// not cleared.
pub fn fn_00ccbb50(e: &mut Engine, this: u32) {
    let (word, saved) = enter_scope(e);
    for offset in [4, 0xc] {
        let block = e.mem.u32(this + offset);
        if block != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
        }
    }
    let aligned = e.mem.u32(this + 0x10);
    if aligned != 0 {
        let distance = u32::from(e.mem.u8(aligned.wrapping_sub(1)));
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, aligned.wrapping_sub(distance)],
        );
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ccbb50, fn_00ccbb50(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::SCOPE_WORD;

    fn setup() -> (Engine, u32) {
        let mut e = logging_engine();
        e.register_double(MEMORY_DEALLOCATE, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        (e, this)
    }

    #[test]
    fn frees_the_three_blocks_in_order() {
        let (mut e, this) = setup();
        let aligned = e.mem.alloc(0x40) + 0x10;
        e.mem.set_u8(aligned - 1, 5);
        e.mem.set_u32(this + 4, 0x1000);
        e.mem.set_u32(this + 0xc, 0x2000);
        e.mem.set_u32(this + 0x10, aligned);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 3);
        e.call(0x00cc_bb50, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            [
                [MEMORY_MANAGER, 0x1000],
                [MEMORY_MANAGER, 0x2000],
                [MEMORY_MANAGER, aligned - 5]
            ]
        );
        assert_eq!(e.mem.u32(word), 3);
        assert_eq!(e.mem.u32(this + 4), 0x1000);
    }

    #[test]
    fn empty_words_free_nothing() {
        let (mut e, this) = setup();
        e.call(0x00cc_bb50, &args![this]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
    }

    #[test]
    fn only_the_aligned_block_can_be_freed_alone() {
        let (mut e, this) = setup();
        let aligned = e.mem.alloc(0x40) + 0x10;
        e.mem.set_u8(aligned - 1, 16);
        e.mem.set_u32(this + 0x10, aligned);
        e.call(0x00cc_bb50, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            [[MEMORY_MANAGER, aligned - 16]]
        );
    }
}
