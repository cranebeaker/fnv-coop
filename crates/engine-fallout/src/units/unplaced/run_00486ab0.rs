//! `(unplaced)/run_00486ab0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_strlen` (LIBCMT), the allocator (`00aa1070`, `cdecl` (size)) and
/// `strcpy_s` (`00404d80`, `cdecl` (dest, size, src)).
const STRLEN: u32 = 0x00ec_6130;
const ALLOCATE: u32 = 0x00aa_1070;
const COPY_STRING: u32 = 0x0040_4d80;
/// One step of the string hash (`00659210`).
const HASH_STEP: u32 = 0x0065_9210;

// Translated from 00486df0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..>::KeyToHashIndex` (Xbox PDB; `thiscall`,
/// `RET 4`): the bucket of a key, each byte up to the terminator folded in
/// by `00659210` as it is (no lowering), modulo the bucket count (+4,
/// unsigned).
pub fn key_to_hash_index(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut hash = 0u32;
    let mut p = key;
    loop {
        let c = e.mem.u8(p);
        if c == 0 {
            break;
        }
        hash = e.call(HASH_STEP, &args![this, c as u32, hash]).u32();
        p = p.wrapping_add(1);
    }
    hash % e.mem.u32(this.addr() + 4)
}

// Translated from 00486ae0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A string map's `SetValue` for a new node (`thiscall`, `RET 0xC`: node,
/// key, value): when the map copies its keys (byte +0x10), a copy of the key
/// (`strlen + 1` bytes from `00aa1070`, filled by `00404d80`) at node +4,
/// else the key itself; the value at node +8.
pub fn fn_00486ae0(e: &mut Engine, this: Ptr, node: u32, key: u32, value: u32) {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let size = e.call(STRLEN, &args![key]).u32().wrapping_add(1);
        let copy = e.call(ALLOCATE, &args![size]).u32();
        e.mem.set_u32(node + 4, copy);
        let dest = e.mem.u32(node + 4);
        e.call(COPY_STRING, &args![dest, size, key]);
    } else {
        e.mem.set_u32(node + 4, key);
    }
    e.mem.set_u32(node + 8, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00486df0, key_to_hash_index(Ptr, u32) -> u32),
        entry!(0x00486ae0, fn_00486ae0(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_the_key_as_it_is() {
        let mut e = Engine::new();
        let map = e.mem.alloc(8);
        e.mem.set_u32(map + 4, 37);
        let key = e.mem.alloc(8);
        e.mem.write(key, b"fAb\0");
        let mut h = 0u32;
        for c in b"fAb" {
            h = h.wrapping_mul(33).wrapping_add(u32::from(*c));
        }
        assert_eq!(e.call(0x00486df0, &[map, key]).u32(), h % 37);
    }

    #[test]
    fn a_copying_map_keeps_a_copy_of_the_key() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(COPY_STRING, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.write(a[0], &s);
            Ret::default()
        });
        let map = e.mem.alloc(0x14);
        let node = e.mem.alloc(0xc);
        let key = e.mem.alloc(8);
        e.mem.write(key, b"abc\0");
        e.call(0x00486ae0, &[map, node, key, 7]);
        assert_eq!(e.mem.u32(node + 4), key);
        assert_eq!(e.mem.u32(node + 8), 7);
        e.mem.set_u8(map + 0x10, 1);
        e.call(0x00486ae0, &[map, node, key, 9]);
        let copy = e.mem.u32(node + 4);
        assert_ne!(copy, key);
        assert_eq!(e.mem.cstr(copy), b"abc");
        assert_eq!(e.mem.u32(node + 8), 9);
    }
}
