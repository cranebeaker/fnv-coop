//! `(unplaced)/run_00e80390` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The allocator used for a non-empty array (`cdecl`, the size).
const ARRAY_ALLOCATE: u32 = 0x00aa_1420;
/// The compiler's array constructor iterator (`stdcall`, (array, element size, count,
/// constructor)).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The element constructor (`00e800f0`, unit `NiXenonRenderer/nid3dpass.cpp` in the engine
/// map) and the element size.
const ELEMENT_CONSTRUCT: u32 = 0x00e8_00f0;
const ELEMENT_SIZE: u32 = 0x68;

/// `count * 0x68 + 4`, the size of the array with its count word, all ones when either step
/// overflows (the compiler's `SETO`/`SETC`, `NEG`, `OR`).
fn array_bytes(count: u32) -> u32 {
    count
        .checked_mul(ELEMENT_SIZE)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX)
}

// Translated from 00e80390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): an array `new[]` of `count` elements of
/// 0x68 bytes, each built by `00e800f0`. An empty array allocates the 4-byte count word
/// from the memory manager (`00aa3e40`), a non-empty one `count * 0x68 + 4` bytes through
/// `00aa1420`. Stores the count in the first word, runs the array constructor iterator
/// over the elements (also for an empty array) and answers the first element, or 0 when
/// the allocation failed.
pub fn fn_00e80390(e: &mut Engine, count: u32) -> u32 {
    let block = if count == 0 {
        e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32()
    } else {
        e.call(ARRAY_ALLOCATE, &args![array_bytes(count)]).u32()
    };
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![block + 4, ELEMENT_SIZE, count, ELEMENT_CONSTRUCT],
    );
    block + 4
}

/// The global object [`fn_00e803f0`] releases (`0126f72c`), the destructor body it runs on
/// it first (`00a90ac0`, unit `NiMain/nioldscreenspace.cpp`, unnamed; `thiscall`, no stack
/// words) and `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, the
/// block).
const GLOBAL_OBJECT_0126F72C: u32 = 0x0126_f72c;
const OBJECT_DESTRUCT: u32 = 0x00a9_0ac0;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00e803f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when the global at `0126f72c` holds an object,
/// destroys it (`00a90ac0`) and frees it (`MemoryManager::Deallocate`); then clears the
/// global.
pub fn fn_00e803f0(e: &mut Engine) {
    let object = e.global::<u32>(GLOBAL_OBJECT_0126F72C);
    if object != 0 {
        e.call(OBJECT_DESTRUCT, &args![object]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
    }
    e.set_global(GLOBAL_OBJECT_0126F72C, 0u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_0390, fn_00e80390(u32) -> u32),
        entry!(0x00e8_03f0, fn_00e803f0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e80390_allocates_counts_and_constructs_the_elements() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x200);
        e.register_double(MEMORY_ALLOCATE, move |_, _| block.into_ret());
        e.register_double(ARRAY_ALLOCATE, move |_, _| block.into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());

        // An empty array: the 4-byte count word from the memory manager.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0390, &[0]).u32(), block + 4);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 4]]);
        assert!(calls_to(&e, ARRAY_ALLOCATE).is_empty());
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![block + 4, 0x68, 0, ELEMENT_CONSTRUCT]]
        );
        assert_eq!(e.mem.u32(block), 0);

        // Three elements: 3 * 0x68 + 4 bytes from the array allocator.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0390, &[3]).u32(), block + 4);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![3 * 0x68 + 4]]);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(block), 3);

        // An overflowing count asks for all ones.
        e.call_log = Some(vec![]);
        e.call(0x00e8_0390, &[0x0300_0000]);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![u32::MAX]]);

        // A failed allocation constructs nothing and answers 0.
        e.register(ARRAY_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0390, &[2]).u32(), 0);
        assert!(calls_to(&e, VECTOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn fn_00e803f0_destroys_frees_and_clears_the_global() {
        let mut e = Engine::new();
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.map(GLOBAL_OBJECT_0126F72C, 4);
        e.set_global(GLOBAL_OBJECT_0126F72C, 0x7700u32);
        e.call_log = Some(vec![]);
        e.call(0x00e8_03f0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (OBJECT_DESTRUCT, vec![0x7700]),
                (MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x7700])
            ]
        );
        assert_eq!(e.global::<u32>(GLOBAL_OBJECT_0126F72C), 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_03f0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
