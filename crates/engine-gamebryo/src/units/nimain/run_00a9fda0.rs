//! `NiMain/run_00a9fda0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton (`thiscall` object of `00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the singleton, one stack word).
const DEALLOCATE: u32 = 0x00aa_4060;
/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The two arrays `00a9fda0` frees, in its order: element pointers with the element count in
/// the word in front of them.
const FIRST_ARRAY: u32 = 0x011f_602c;
const SECOND_ARRAY: u32 = 0x011f_6024;

/// Frees one array of 8-byte elements whose second word is an `NiFixedString` handle: the
/// handles from the last element to the first are released (unless empty), then the block,
/// count word included, goes back to the memory manager.
fn free_string_pair_array(e: &mut Engine, elements: u32) {
    let block = elements.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut cursor = elements.wrapping_add(count.wrapping_mul(8)).wrapping_add(4);
    let mut remaining = count.wrapping_sub(1) as i32;
    while remaining >= 0 {
        let handle = e.mem.u32(cursor.wrapping_sub(8));
        cursor = cursor.wrapping_sub(8);
        if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        remaining = remaining.wrapping_sub(1);
    }
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00a9fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): frees the arrays at `011f602c` and then `011f6024` (8-byte elements whose
/// second word is an `NiFixedString` handle, released from the last element to the first
/// unless it is the empty string's; the count is the word in front of the elements) and
/// clears both pointers. The decompiler shows the empty-string comparison as a comparison
/// with 0; the disassembly compares with the global `0109b220`.
pub fn fn_00a9fda0(e: &mut Engine) {
    let first = e.mem.u32(FIRST_ARRAY);
    if first != 0 {
        free_string_pair_array(e, first);
    }
    e.mem.set_u32(FIRST_ARRAY, 0);
    let second = e.mem.u32(SECOND_ARRAY);
    if second != 0 {
        free_string_pair_array(e, second);
    }
    e.mem.set_u32(SECOND_ARRAY, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_fda0, fn_00a9fda0())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn releases_the_second_words_last_first_and_frees_both_arrays() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.map(FIRST_ARRAY & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x4444);
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let block = e.mem.alloc(4 + 3 * 8);
        e.mem.set_u32(block, 3);
        let elements = block + 4;
        for (index, handle) in [0x1008u32, 0x4444, 0x3008].into_iter().enumerate() {
            e.mem.set_u32(elements + 8 * index as u32, 0x9999);
            e.mem.set_u32(elements + 8 * index as u32 + 4, handle);
        }
        e.mem.set_u32(FIRST_ARRAY, elements);
        e.mem.set_u32(SECOND_ARRAY, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_fda0, &args![]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00a9_fda0)
            .collect();
        assert_eq!(
            log,
            vec![
                (INTERLOCKED_DECREMENT, vec![0x3000]),
                (INTERLOCKED_DECREMENT, vec![0x1000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(FIRST_ARRAY), 0);

        // An empty array (count 0) is freed without releases; the second array too.
        let empty = e.mem.alloc(4);
        e.mem.set_u32(SECOND_ARRAY, empty + 4);
        e.call_log = Some(vec![]);
        e.call(0x00a9_fda0, &args![]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00a9_fda0)
            .collect();
        assert_eq!(log, vec![(DEALLOCATE, vec![MEMORY_MANAGER, empty])]);
        assert_eq!(e.mem.u32(SECOND_ARRAY), 0);
    }
}
