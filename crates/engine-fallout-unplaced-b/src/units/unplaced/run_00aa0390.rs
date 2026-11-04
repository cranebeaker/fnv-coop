//! `(unplaced)/run_00aa0390` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `011f4400`: a counted word `00aa0390` increments for the object it constructs.
const OBJECT_COUNT: u32 = 0x011f_4400;
/// `InterlockedIncrement` import slot.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00aa0390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; the body of a constructor, `thiscall`, no stack
/// word): sets the vtable `0101dce4` and the word at `+4` to 0, counts `011f4400`, sets
/// the vtable `010a2164` and the words `+8` to `010a2138` and `+0xc` to 0x25 and `+0x14` to
/// 0, allocates a 0x94-byte block (`00aa1070`) for `+0x10` and zeroes it (0x25 words),
/// sets `+8` to `010a2154` and clears `+0x18`, `+0x1c` and `+0x20`. Returns `this`.
pub fn fn_00aa0390(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_dce4);
    e.mem.set_u32(base + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![OBJECT_COUNT]);
    e.mem.set_u32(base, 0x010a_2164);
    e.mem.set_u32(base + 8, 0x010a_2138);
    e.mem.set_u32(base + 0xc, 0x25);
    e.mem.set_u32(base + 0x14, 0);
    let block = e.call(ALLOCATE_BLOCK, &args![0x94u32]).u32();
    let bytes = e.mem.u32(base + 0xc).wrapping_mul(4);
    e.mem.set_u32(base + 0x10, block);
    e.call(MEMSET, &args![block, 0u32, bytes]);
    e.mem.set_u32(base + 8, 0x010a_2154);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_0390, fn_00aa0390(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_object_with_a_zeroed_table() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(OBJECT_COUNT, 3);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(ALLOCATE_BLOCK, |e, a| {
            assert_eq!(a, &[0x94]);
            let block = e.mem.alloc(0x94);
            e.mem.write(block, &[0x55; 0x94]);
            block.into_ret()
        });
        e.register(MEMSET, |e, a| {
            assert_eq!(&a[1..], &[0, 0x94]);
            e.mem.write(a[0], &[0u8; 0x94]);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0xcc; 0x30]);
        assert_eq!(e.call(0x00aa_0390, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(e.mem.u32(this), 0x010a_2164);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x010a_2154);
        assert_eq!(e.mem.u32(this + 0xc), 0x25);
        let block = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.bytes(block, 0x94), vec![0u8; 0x94]);
        for offset in [0x14u32, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
    }
}
