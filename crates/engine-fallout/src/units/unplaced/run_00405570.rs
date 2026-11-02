//! `(unplaced)/run_00405570` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructors of a string-keyed hash map, base to derived (the game
//! setting collection's map, built by `00404d50`): the bucket array, then
//! the copy flag, then the vtables.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtables the four constructors set, outermost first.
const MAP_VTABLE: u32 = 0x0101_2298;
const STRING_MAP_VTABLE: u32 = 0x0101_2238;
const MAP_BASE_VTABLE: u32 = 0x0101_2258;
const HASH_MAP_VTABLE: u32 = 0x0101_2278;
/// The allocator of the bucket array (`00aa1070`, `cdecl` (size)).
const ALLOCATE: u32 = 0x00aa_1070;
/// `memset` wrapper (`00403d30`, `cdecl` (dest, value, count)).
const FILL: u32 = 0x0040_3d30;
/// The map's destructor body (`00405230`, `thiscall`, no stack words).
const DESTROY: u32 = 0x0040_5230;
/// The deallocator (`00401030`, `cdecl` (block)).
const FREE: u32 = 0x0040_1030;

// Translated from 00405570 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The outermost constructor (`thiscall`, `RET 8`): [`fn_004055d0`] with the
/// bucket count and the copy flag (a byte, `MOVZX`), then the vtable
/// `01012298`. Returns `this`.
pub fn fn_00405570(e: &mut Engine, this: Ptr, buckets: u32, copy: u8) -> Ptr {
    fn_004055d0(e, this, buckets, copy);
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    this
}

// Translated from 004055a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor in the vtable `01012298` (`thiscall`,
/// `RET 4`): the destructor body `00405230`, then, when bit 0 of `flags` is
/// set, frees `this` (`00401030`). Returns `this`.
pub fn fn_004055a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTROY, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 004055d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The string map's constructor (`thiscall`, `RET 8`): [`fn_00405600`] with
/// the bucket count, the vtable `01012238`, and the copy flag (whether keys
/// are copied) at +0x10. Returns `this`.
pub fn fn_004055d0(e: &mut Engine, this: Ptr, buckets: u32, copy: u8) -> Ptr {
    fn_00405600(e, this, buckets);
    e.mem.set_u32(this.addr(), STRING_MAP_VTABLE);
    e.mem.set_u8(this.addr() + 0x10, copy);
    this
}

// Translated from 00405600 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A map base constructor (`thiscall`, `RET 4`): [`fn_00405630`] with the
/// bucket count, then the vtable `01012258`. Returns `this`.
pub fn fn_00405600(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_00405630(e, this, buckets);
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    this
}

// Translated from 00405630 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The hash map's base constructor (`thiscall`, `RET 4`): the vtable
/// `01012278`, the bucket count at +4, no entries (+0xC), and a bucket array
/// of `count * 4` bytes (`00aa1070`) at +8, zeroed (`00403d30`). Returns
/// `this`.
pub fn fn_00405630(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, HASH_MAP_VTABLE);
    e.mem.set_u32(t + 4, buckets);
    e.mem.set_u32(t + 0xc, 0);
    let size = e.mem.u32(t + 4) << 2;
    let array = e.call(ALLOCATE, &args![size]).u32();
    e.mem.set_u32(t + 8, array);
    let size = e.mem.u32(t + 4) << 2;
    let array = e.mem.u32(t + 8);
    e.call(FILL, &args![array, 0u32, size]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00405570, fn_00405570(Ptr, u32, u8) -> Ptr),
        entry!(0x004055a0, fn_004055a0(Ptr, u32) -> Ptr),
        entry!(0x004055d0, fn_004055d0(Ptr, u32, u8) -> Ptr),
        entry!(0x00405600, fn_00405600(Ptr, u32) -> Ptr),
        entry!(0x00405630, fn_00405630(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_map_gets_a_zeroed_bucket_array_and_its_vtables() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FILL, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        let m = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00405570, &[m, 37, 1]).u32(), m);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![37 * 4]));
        assert_eq!(log[2].0, FILL);
        assert_eq!(e.mem.u32(m), MAP_VTABLE);
        assert_eq!(e.mem.u32(m + 4), 37);
        assert_eq!(e.mem.u32(m + 0xc), 0);
        assert_eq!(e.mem.u8(m + 0x10), 1);
        let array = e.mem.u32(m + 8);
        assert_eq!(log[2].1, vec![array, 0, 37 * 4]);
    }

    #[test]
    fn the_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(DESTROY, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_55a0, &[0x5000, 1]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(DESTROY, vec![0x5000])));
        assert!(log.contains(&(FREE, vec![0x5000])));

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_55a0, &[0x5000, 2]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(DESTROY, vec![0x5000])));
        assert!(!log.iter().any(|c| c.0 == FREE));
    }
}
