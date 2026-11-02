//! `Havok SDK/run_00d56e00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the arrays are freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// The allocator word of a fresh `00c85750` call (`router + 0x10`).
fn allocator_00d56e00(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00d56e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (`__fastcall`, no stack words; no map name, class unconfirmed): writes the vtable
/// words `+0` = `0x010d_0ecc` and `+0x10` = `0x010d_0ea8` and releases three `hkArray`s, each time
/// asking `00c85750` for the allocator first: the 16-bit array at `+0xcc` (size `+0xd0`, capacity
/// word `+0xd4`; the size is cleared and the data freed through slot `0x10` when the sign bit of
/// the capacity word is clear, `2 * (capacity & 0x3fffffff)` bytes); the array of 0x40-byte
/// elements at `+0xc0` (size `+0xc4`, capacity word `+0xc8`: `00d56800` is called on each element
/// first, then the size is cleared and the data freed, `(capacity & 0x3fffffff) * 0x40` bytes);
/// the array of 0x60-byte elements at `+0xb4` (size `+0xb8`, capacity word `+0xbc`,
/// `(capacity & 0x3fffffff) * 0x60` bytes). Each array ends with a null data pointer and the
/// capacity word `0x80000000`. Last, `+0x10` = `0x010c_6fb0` and `+0` = `0x0102_e388`.
pub fn fn_00d56e00(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, 0x010d_0ecc);
    e.mem.set_u32(me + 0x10, 0x010d_0ea8);
    let allocator = allocator_00d56e00(e);
    let capacity = e.mem.u32(me + 0xd4);
    e.mem.set_u32(me + 0xd0, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xcc);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(2);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0xcc, 0);
    e.mem.set_u32(me + 0xd4, 0x8000_0000);
    let allocator = allocator_00d56e00(e);
    let count = e.mem.i32(me + 0xc4);
    let mut element = e.mem.u32(me + 0xc0);
    if count > 0 {
        for _ in 0..count {
            e.call(0x00d5_6800, &args![element]);
            element = element.wrapping_add(0x40);
        }
    }
    let capacity = e.mem.u32(me + 0xc8);
    e.mem.set_u32(me + 0xc4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xc0);
        let bytes = (capacity & 0x3fff_ffff) << 6;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0xc0, 0);
    e.mem.set_u32(me + 0xc8, 0x8000_0000);
    let allocator = allocator_00d56e00(e);
    let capacity = e.mem.u32(me + 0xbc);
    e.mem.set_u32(me + 0xb8, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xb4);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(3) << 5;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0xbc, 0x8000_0000);
    e.mem.set_u32(me + 0xb4, 0);
    e.mem.set_u32(me + 0x10, 0x010c_6fb0);
    e.mem.set_u32(me, 0x0102_e388);
}

// Translated from 00d56f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor wrapper (thiscall, `RET 4`: one stack word `build`; no map name, class
/// unconfirmed). It writes a chain of vtable words (`+0` = `0x010305a8`, then `0x010cc364`, then
/// `0x010d_0ecc`; `+0x10` = `0x010c6fb0`, `0x010cc344`, `0x010d_0ea8`), with `+0xc` = 0 after the
/// first and `+0xc` = 7, the byte at `+0x15` = 3 after the second, only when `build` is not 0. With
/// `build` set it also marks every 0x60-byte element of the array at `+0xb4` (size `+0xb8`) and
/// every 0x40-byte element of the one at `+0xc0` (size `+0xc4`) whose byte at `+1` is 0 as 1
/// (the byte at `+0x15` becomes 1); when the first array holds exactly one element it moves that
/// element's 96 bytes into the inline storage of this object at `+0x20` (freeing the old data,
/// then size 1 and capacity word `0x80000001` there). `+0xc` ends as 0xe, and a word at `+0xe0`
/// equal to `0xffffffff` becomes `0x80000000`. Returns `this`.
pub fn fn_00d56f10(e: &mut Engine, this: Ptr, build: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0x0103_05a8);
    if build != 0 {
        e.mem.set_u32(me + 0xc, 0);
    }
    e.mem.set_u32(me + 0x10, 0x010c_6fb0);
    e.mem.set_u32(me, 0x010c_c364);
    e.mem.set_u32(me + 0x10, 0x010c_c344);
    if build != 0 {
        e.mem.set_u32(me + 0xc, 7);
        e.mem.set_u8(me + 0x15, 3);
    }
    e.mem.set_u32(me, 0x010d_0ecc);
    e.mem.set_u32(me + 0x10, 0x010d_0ea8);
    if build == 0 {
        e.mem.set_u32(me + 0xc, 0xe);
        return this;
    }
    e.mem.set_u8(me + 0x15, 1);
    if e.mem.i32(me + 0xb8) > 0 {
        let mut offset = 0u32;
        let mut index = 0i32;
        loop {
            let data = e.mem.u32(me + 0xb4);
            let flag_at = offset.wrapping_add(data).wrapping_add(1);
            if e.mem.u8(flag_at) == 0 {
                e.mem.set_u8(flag_at, 1);
            }
            index += 1;
            offset = offset.wrapping_add(0x60);
            if index >= e.mem.i32(me + 0xb8) {
                break;
            }
        }
    }
    if e.mem.i32(me + 0xc4) > 0 {
        let mut offset = 0u32;
        let mut index = 0i32;
        loop {
            let data = e.mem.u32(me + 0xc0);
            let flag_at = offset.wrapping_add(data).wrapping_add(1);
            if e.mem.u8(flag_at) == 0 {
                e.mem.set_u8(flag_at, 1);
            }
            index += 1;
            offset = offset.wrapping_add(0x40);
            if index >= e.mem.i32(me + 0xc4) {
                break;
            }
        }
    }
    if e.mem.u32(me + 0xb8) == 1 {
        let element = e.mem.u32(me + 0xb4);
        let inline_storage = me + 0x20;
        for word in 0..0x18u32 {
            let value = e.mem.u32(element.wrapping_add(word * 4));
            e.mem.set_u32(inline_storage + word * 4, value);
        }
        let allocator = allocator_00d56e00(e);
        let capacity = e.mem.u32(me + 0xbc);
        e.mem.set_u32(me + 0xb8, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(me + 0xb4);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(3) << 5;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(me + 0xbc, 0x8000_0000);
        e.mem.set_u32(me + 0xb4, inline_storage);
        e.mem.set_u32(me + 0xb8, 1);
        e.mem.set_u32(me + 0xbc, 0x8000_0001);
    }
    let marker = e.mem.u32(me + 0xe0);
    e.mem.set_u32(me + 0xc, 0xe);
    if marker == 0xffff_ffff {
        e.mem.set_u32(me + 0xe0, 0x8000_0000);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_6e00, fn_00d56e00(Ptr)),
        entry!(0x00d5_6f10, fn_00d56f10(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    fn install(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0111);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_0111, |_, _| Ret::default());
        allocator
    }

    #[test]
    fn fn_00d56e00_destroys_the_elements_and_frees_the_three_arrays() {
        let mut e = Engine::new();
        let allocator = install(&mut e);
        e.register(0x00d5_6800, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        // 16-bit array (cap 3), 0x40-byte elements (2, cap 5), 0x60-byte elements (cap 4).
        e.mem.set_u32(this + 0xcc, 0x1000);
        e.mem.set_u32(this + 0xd0, 2);
        e.mem.set_u32(this + 0xd4, 0x4000_0003);
        e.mem.set_u32(this + 0xc0, 0x2000);
        e.mem.set_u32(this + 0xc4, 2);
        e.mem.set_u32(this + 0xc8, 0x4000_0005);
        e.mem.set_u32(this + 0xb4, 0x3000);
        e.mem.set_u32(this + 0xb8, 1);
        e.mem.set_u32(this + 0xbc, 0x4000_0004);
        e.call_log = Some(vec![]);
        e.call(0x00d5_6e00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMORY_ROUTER, vec![]),
                (0x0900_0111, vec![allocator, 0x1000, 6]),
                (MEMORY_ROUTER, vec![]),
                (0x00d5_6800, vec![0x2000]),
                (0x00d5_6800, vec![0x2040]),
                (0x0900_0111, vec![allocator, 0x2000, 5 * 0x40]),
                (MEMORY_ROUTER, vec![]),
                (0x0900_0111, vec![allocator, 0x3000, 4 * 0x60]),
            ]
        );
        for (data, size, capacity) in [(0xcc, 0xd0, 0xd4), (0xc0, 0xc4, 0xc8), (0xb4, 0xb8, 0xbc)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
    }

    #[test]
    fn fn_00d56e00_frees_nothing_for_arrays_that_do_not_own_their_data() {
        let mut e = Engine::new();
        install(&mut e);
        let this = e.mem.alloc(0x100);
        for capacity in [0xd4u32, 0xc8, 0xbc] {
            e.mem.set_u32(this + capacity, 0x8000_0002);
        }
        e.call_log = Some(vec![]);
        e.call(0x00d5_6e00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 4);
        assert!(log[1..]
            .iter()
            .all(|(address, _)| *address == MEMORY_ROUTER));
    }

    #[test]
    fn fn_00d56f10_without_build_only_writes_the_vtables() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xc, 0x55);
        let result = e.call(0x00d5_6f10, &args![this, 0u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010d_0ecc);
        assert_eq!(e.mem.u32(this + 0x10), 0x010d_0ea8);
        assert_eq!(e.mem.u32(this + 0xc), 0xe);
        assert_eq!(e.mem.u8(this + 0x15), 0);
    }

    #[test]
    fn fn_00d56f10_with_build_marks_the_elements_and_moves_a_single_one_inline() {
        let mut e = Engine::new();
        let allocator = install(&mut e);
        let this = e.mem.alloc(0x100);
        // One 0x60-byte element (byte +1 clear), two 0x40-byte elements (bytes +1: 0 and 5).
        let big = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(big + word * 4, 0x4000 + word);
        }
        e.mem.set_u8(big + 1, 0);
        e.mem.set_u32(this + 0xb4, big);
        e.mem.set_u32(this + 0xb8, 1);
        e.mem.set_u32(this + 0xbc, 0x4000_0001);
        let small = e.mem.alloc(0x80);
        e.mem.set_u8(small + 1, 0);
        e.mem.set_u8(small + 0x40 + 1, 5);
        e.mem.set_u32(this + 0xc0, small);
        e.mem.set_u32(this + 0xc4, 2);
        e.mem.set_u32(this + 0xe0, 0xffff_ffff);
        e.call_log = Some(vec![]);
        let result = e.call(0x00d5_6f10, &args![this, 1u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u8(small + 1), 1);
        assert_eq!(e.mem.u8(small + 0x41), 5);
        // The big element was marked (byte +1 became 1) before it was copied inline.
        for word in 0..0x18u32 {
            let expected = if word == 0 { 0x100 } else { 0x4000 + word };
            assert_eq!(e.mem.u32(this + 0x20 + word * 4), expected);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMORY_ROUTER, vec![]),
                (0x0900_0111, vec![allocator, big, 0x60])
            ]
        );
        assert_eq!(e.mem.u32(this + 0xb4), this + 0x20);
        assert_eq!(e.mem.u32(this + 0xb8), 1);
        assert_eq!(e.mem.u32(this + 0xbc), 0x8000_0001);
        assert_eq!(e.mem.u32(this + 0xc), 0xe);
        assert_eq!(e.mem.u8(this + 0x15), 1);
        assert_eq!(e.mem.u32(this + 0xe0), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x010d_0ecc);
    }
}
