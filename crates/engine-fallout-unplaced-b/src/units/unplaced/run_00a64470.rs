//! `(unplaced)/run_00a64470` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::platform::MEMORY_MANAGER;

/// Import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the singleton, one
/// stack word: the block).
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00a644a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: frees the storage of an array of `NiFixedString`
/// handles (`cdecl`, one stack word: the first element, with the element count in
/// the word in front of it; nothing for null). Walks the elements from the last
/// to the first and, for each handle that is not the empty string handle
/// (`0109b220`), drops one reference of the pooled string (`InterlockedDecrement`
/// of the count 8 bytes before the handle; the string is not freed here). Then
/// the block, count word included, is given back to the memory manager
/// (`00aa4060`). The decompiler shows the comparison with the empty string
/// handle as a comparison with 0; the disassembly compares with the global.
pub fn fn_00a644a0(e: &mut Engine, elements: Ptr) {
    if elements.is_null() {
        return;
    }
    let count = e.mem.u32(elements.addr().wrapping_sub(4));
    let mut cursor = elements.addr().wrapping_add(count.wrapping_mul(4));
    let mut remaining = count.wrapping_sub(1) as i32;
    while remaining >= 0 {
        let handle = e.mem.u32(cursor.wrapping_sub(4));
        cursor = cursor.wrapping_sub(4);
        if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        remaining = remaining.wrapping_sub(1);
    }
    e.call(
        MEMORY_MANAGER_DEALLOCATE,
        &args![MEMORY_MANAGER, elements.addr().wrapping_sub(4)],
    );
}

/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 00a64470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a scalar deleting destructor. Sets the
/// vtable `0109bce4`, frees the block at +4 (`00aa10f0`), and for bit 0 of `flags` gives
/// `this` back to the memory manager (`00aa4060`). Returns `this`.
pub fn fn_00a64470(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let block = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), 0x0109_bce4);
    e.call(FREE_BLOCK, &args![block]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_44a0, fn_00a644a0(Ptr)),
        entry!(0x00a6_4470, fn_00a64470(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: u32 = 0x0109_b220;

    fn fixture() -> Engine {
        let mut e = Engine::new();
        e.map(EMPTY & !0xfff, 0x1000);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e
    }

    #[test]
    fn null_does_nothing() {
        let mut e = fixture();
        e.call_log = Some(vec![]);
        e.call(0x00a6_44a0, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn every_counted_handle_loses_a_reference_from_the_last_to_the_first() {
        let mut e = fixture();
        e.set_global(EMPTY, 0x5008u32);
        let block = e.mem.alloc(16);
        e.mem.set_u32(block, 3);
        // handles: a string, the empty string, a string
        for (i, handle) in [0x9008u32, 0x5008, 0xa008].iter().enumerate() {
            e.mem.set_u32(block + 4 + i as u32 * 4, *handle);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a6_44a0, &args![block + 4]);
        let log = e.call_log.take().unwrap();
        let decrements: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INTERLOCKED_DECREMENT)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(decrements, vec![args![0xa000u32], args![0x9000u32]]);
        let frees: Vec<_> = log
            .iter()
            .filter(|call| call.0 == MEMORY_MANAGER_DEALLOCATE)
            .collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![MEMORY_MANAGER, block]);
    }

    #[test]
    fn an_empty_array_only_frees_the_block() {
        let mut e = fixture();
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a6_44a0, &args![block + 4]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != INTERLOCKED_DECREMENT));
        assert_eq!(
            log.iter()
                .filter(|call| call.0 == MEMORY_MANAGER_DEALLOCATE)
                .count(),
            1
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a64470_frees_the_block_and_this_on_bit_0() {
        let mut e = Engine::new();
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        for flags in [1u32, 0] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a6_4470, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(FREE_BLOCK, vec![0x6000])];
            if flags == 1 {
                expected.push((MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, this]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(this), 0x0109_bce4);
        }
    }
}
