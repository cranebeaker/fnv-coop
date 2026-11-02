//! `NiMain/run_00a8fa50` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `011f5e34`: where `00a8fa50` keeps its object.
const TABLE_OBJECT_SLOT: u32 = 0x011f_5e34;
/// Byte `011f5d30`, cleared by `00a8fa50`.
const TABLE_OBJECT_FLAG: u32 = 0x011f_5d30;

// Translated from 00a8fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x10 bytes from the memory manager and, when that
/// succeeds, sets the vtable `0109f72c`, the word at `+4` to 0x25 and the word at `+0xc` to
/// 0, allocates 0x94 bytes (`00aa1070`) for `+8` and zeroes them (0x25 words), sets the
/// vtable `0109f798`, keeps the object in `011f5e34`; null there when the allocation
/// fails. The byte at `011f5d30` is cleared either way.
pub fn fn_00a8fa50(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x10u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(TABLE_OBJECT_SLOT, 0);
        e.mem.set_u8(TABLE_OBJECT_FLAG, 0);
        return;
    }
    e.mem.set_u32(block, 0x0109_f72c);
    e.mem.set_u32(block + 4, 0x25);
    e.mem.set_u32(block + 0xc, 0);
    let buffer = e.call(ALLOCATE_BLOCK, &args![0x94u32]).u32();
    let count = e.mem.u32(block + 4);
    e.mem.set_u32(block + 8, buffer);
    let bytes = count.wrapping_mul(4);
    e.call(MEMSET, &args![buffer, 0u32, bytes]);
    e.mem.set_u32(block, 0x0109_f798);
    e.mem.set_u32(TABLE_OBJECT_SLOT, block);
    e.mem.set_u8(TABLE_OBJECT_FLAG, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_fa50, fn_00a8fa50())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x2000);
        e.mem.set_u8(TABLE_OBJECT_FLAG, 1);
        e
    }

    #[test]
    fn builds_the_object_with_a_zeroed_table() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x94);
        e.mem.write(buffer, &[0x77; 0x94]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x10]);
            block.into_ret()
        });
        e.register_double(ALLOCATE_BLOCK, move |_, a| {
            assert_eq!(a, &[0x94]);
            buffer.into_ret()
        });
        e.register_double(MEMSET, |e, a| {
            assert_eq!(&a[1..], &[0, 0x94]);
            e.mem.write(a[0], &[0u8; 0x94]);
            Ret::default()
        });
        e.call(0x00a8_fa50, &args![]);
        assert_eq!(e.mem.u32(TABLE_OBJECT_SLOT), block);
        assert_eq!(e.mem.u32(block), 0x0109_f798);
        assert_eq!(e.mem.u32(block + 4), 0x25);
        assert_eq!(e.mem.u32(block + 8), buffer);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.bytes(buffer, 0x94), vec![0u8; 0x94]);
        assert_eq!(e.mem.u8(TABLE_OBJECT_FLAG), 0);
    }

    #[test]
    fn a_failed_allocation_stores_null_and_clears_the_flag() {
        let mut e = engine();
        e.mem.set_u32(TABLE_OBJECT_SLOT, 0x1234);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00a8_fa50, &args![]);
        assert_eq!(e.mem.u32(TABLE_OBJECT_SLOT), 0);
        assert_eq!(e.mem.u8(TABLE_OBJECT_FLAG), 0);
    }
}
