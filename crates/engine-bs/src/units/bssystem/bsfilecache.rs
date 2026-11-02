//! `BSSystem/bsfilecache.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton and its `Deallocate` (`00aa4060`, thiscall, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The cache's destructor body (`00b01020`, `BSSystem/run_00b00ec0`, thiscall).
const CACHE_DESTRUCTOR: u32 = 0x00b0_1020;

// Translated from 00b01310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFileCache::scalar deleting destructor` (engine map; thiscall, one word): runs the
/// destructor body (`00b01020`) and, with bit 0 of `flags`, gives the cache back to the
/// memory manager. Answers `this`.
pub fn bs_file_cache_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CACHE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b01370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed for this unit (the engine map gives the folded body the name
/// `LEAPCORE::CBaseSWFilter::DisableEffect`; thiscall, no stack words): clears the word at
/// +0x60.
pub fn fn_00b01370(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr().wrapping_add(0x60), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b0_1310,
            bs_file_cache_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00b0_1370, fn_00b01370(Ptr)),
    ]
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn deleting_destructor_frees_on_bit_zero() {
        let mut e = Engine::new();
        e.register(CACHE_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1310, &args![0x1000u32, 0u32]).u32(), 0x1000);
        assert_eq!(calls_to(&e, CACHE_DESTRUCTOR), vec![vec![0x1000]]);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        e.call(0x00b0_1310, &args![0x1000u32, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1000]]
        );
    }

    #[test]
    fn clears_the_word_at_0x60() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        e.mem.set_u32(this + 0x60, 7);
        e.mem.set_u32(this + 0x64, 7);
        e.call(0x00b0_1370, &args![this]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 7);
    }
}
