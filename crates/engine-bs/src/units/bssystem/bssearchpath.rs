//! `BSSystem/bssearchpath.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton, its `Allocate` (`00aa3e40`) and `Deallocate` (`00aa4060`),
/// thiscall, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `_strcpy_s` (`00ec65a6`, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// The base constructor and destructor bodies (`00aa24d0`, `00aa24f0`,
/// `NiSystem/run_00aa24a0`, thiscall).
const BASE_CONSTRUCTOR: u32 = 0x00aa_24d0;
const BASE_DESTRUCTOR: u32 = 0x00aa_24f0;
/// The `BSSearchPath` vtable (`010a4810`).
const SEARCH_PATH_VTABLE: u32 = 0x010a_4810;
/// The size of a `BSSearchPath` (0x210 bytes allocated): its vtable, the count of paths
/// handed out (+4) and its path (+8).
const SEARCH_PATH_SIZE: u32 = 0x210;

// Translated from 00b01390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSearchPath::GetNextSearchPath` (engine map; thiscall, two words, the second not read):
/// the first time (the count at +4 is 0) copies the path at +8 into `out` (`strcpy_s`,
/// 0x104), counts it and answers true; false afterwards.
pub fn bs_search_path_get_next_search_path(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    _unused_2: u32,
) -> bool {
    let at = this.addr();
    if e.mem.u32(at.wrapping_add(4)) != 0 {
        return false;
    }
    e.call(STRCPY_S, &args![out, 0x104u32, at.wrapping_add(8)]);
    let count = e.mem.u32(at.wrapping_add(4)).wrapping_add(1);
    e.mem.set_u32(at.wrapping_add(4), count);
    true
}

// Translated from 00b013e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word, neither read): answers true.
pub fn fn_00b013e0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    true
}

// Translated from 00b013f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): a new `BSSearchPath`: 0x210 bytes from the
/// memory manager, the base constructed (`00aa24d0`) and the vtable `010a4810` stored; null
/// when the allocation fails.
// Translation incomplete: 00b013f0 - its FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn fn_00b013f0(e: &mut Engine) -> Ptr {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, SEARCH_PATH_SIZE],
        )
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, SEARCH_PATH_VTABLE);
    Ptr::new(block)
}

// Translated from 00b01480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSearchPath::scalar deleting destructor` (engine map; thiscall, one word): runs the
/// base destructor body (`00aa24f0`) and, with bit 0 of `flags`, gives the object back to
/// the memory manager. Answers `this`.
pub fn bs_search_path_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b0_1390,
            bs_search_path_get_next_search_path(Ptr, u32, u32) -> bool
        ),
        entry!(0x00b0_13e0, fn_00b013e0(Ptr, u32) -> bool),
        entry!(0x00b0_13f0, fn_00b013f0() -> Ptr),
        entry!(
            0x00b0_1480,
            bs_search_path_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
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
    fn the_path_is_handed_out_once() {
        let mut e = Engine::new();
        e.register(STRCPY_S, |_, _| Ret::default());
        let this = e.mem.alloc(SEARCH_PATH_SIZE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b0_1390, &args![this, 0x5000u32, 0u32]).bool());
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![0x5000, 0x104, this + 8]]);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert!(!e.call(0x00b0_1390, &args![this, 0x5000u32, 0u32]).bool());
        assert_eq!(calls_to(&e, STRCPY_S).len(), 1);
        assert!(e.call(0x00b0_13e0, &args![this, 0u32]).bool());
    }

    #[test]
    fn factory_constructs_a_search_path() {
        let mut e = Engine::new();
        let block = e.mem.alloc(SEARCH_PATH_SIZE);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| block.into_ret());
        e.register(BASE_CONSTRUCTOR, |e, w| {
            e.mem.set_u32(w[0], 0x1111);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_13f0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), SEARCH_PATH_VTABLE);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, SEARCH_PATH_SIZE]]
        );
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![block]]);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_13f0, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn deleting_destructor_frees_on_bit_zero() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1480, &args![0x1000u32, 2u32]).u32(), 0x1000);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        e.call(0x00b0_1480, &args![0x1000u32, 1u32]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR).len(), 2);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1000]]
        );
    }
}
