//! `(unplaced)/run_006cf700` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The search of a byte array (a `BSSimpleArray<bool>`-shaped object:
//! buffer at `+4`, size at `+8`) and its default comparison.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006cf700 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the index of the first byte at or after `start` that
/// `compare(address of the byte, item)` (a `cdecl` function, `bool`)
/// accepts; `0xffffffff` when none does (or the buffer is null).
pub fn fn_006cf700(e: &mut Engine, this: Ptr, item: u32, start: u32, compare: u32) -> u32 {
    let mut index = start;
    while index < e.mem.u32(this.addr() + 8) {
        let buffer = e.mem.u32(this.addr() + 4);
        if buffer != 0
            && e.call(compare, &args![buffer.wrapping_add(index), item])
                .bool()
        {
            return index;
        }
        index = index.wrapping_add(1);
    }
    0xffff_ffff
}

// Translated from 006cf760 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: whether the bytes at `a` and `b` are equal.
pub fn fn_006cf760(e: &mut Engine, a: Ptr, b: Ptr) -> bool {
    e.mem.u8(a.addr()) == e.mem.u8(b.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_f700, fn_006cf700(Ptr, u32, u32, u32) -> u32),
        entry!(0x006c_f760, fn_006cf760(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_first_equal_byte_from_start() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        let buffer = e.mem.alloc(5);
        e.mem.write(buffer, &[1, 0, 1, 0, 1]);
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 8, 5);
        let item = e.mem.alloc(1);
        e.mem.set_u8(item, 0);
        let find = |e: &mut Engine, start: u32| {
            e.call(0x006c_f700, &args![this, item, start, 0x006c_f760u32])
                .u32()
        };
        assert_eq!(find(&mut e, 0), 1);
        assert_eq!(find(&mut e, 2), 3);
        assert_eq!(find(&mut e, 4), 0xffff_ffff);
        e.mem.set_u32(this + 4, 0);
        assert_eq!(find(&mut e, 0), 0xffff_ffff);
    }
}
