//! `Havok SDK/run_00d1fa10` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator word `00c908c0` grows an
/// array through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

// Translated from 00d1fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, no stack words): does nothing. `00ca1fb0` calls it on every 0x30-byte
/// element of an array it empties.
pub fn fn_00d1fa10(_e: &mut Engine, _this: Ptr) {}

// Translated from 00d1fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, no stack words): sets every word of the array whose data pointer is at
/// `+0x18` and size at `+0x1c` to 1 (the data pointer is read for each element, the size is
/// compared signed after each store), then clears the byte at `+0x24`.
pub fn fn_00d1fa20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(base + 0x1c) {
        let data = e.mem.u32(base + 0x18);
        e.mem.set_u32(data.wrapping_add(index.wrapping_mul(4)), 1);
        index = index.wrapping_add(1);
    }
    e.mem.set_u8(base + 0x24, 0);
}

// Translated from 00d1fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, one stack word `source`; class unconfirmed): vtable `0x010c_e1a8` at
/// `+0`, the `u16` at `+6` set to 1, the three words at `source + 0` .. `+8` copied to `+8`,
/// `+0xc`, `+0x10`; an empty `hkArray` at `+0x18` (data 0, size 0, capacity word `0x80000000`);
/// the count `n` is the word at `(+8 object) + 0x1c`, stored at `+0x14`. When the array's capacity
/// is below `n` it is reserved through `00c908c0` for `max(n, 2 * capacity)` words (4 bytes
/// each); then the words from the old size up to `n` are set to 1 (a null element address is
/// skipped), the size becomes `n` and the byte at `+0x24` is cleared. Returns `this`.
pub fn fn_00d1fa50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_e1a8);
    e.mem.set_u16(base + 6, 1);
    for word in 0..3 {
        let value = e.mem.u32(source.addr() + word * 4);
        e.mem.set_u32(base + 8 + word * 4, value);
    }
    let array = base + 0x18;
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let owner = e.mem.u32(base + 8);
    let count = e.mem.i32(owner + 0x1c);
    e.mem.set_i32(base + 0x14, count);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let mut remaining = count.wrapping_sub(size as i32);
    let mut element = data.wrapping_add(size.wrapping_mul(4));
    if remaining > 0 {
        loop {
            if element != 0 {
                e.mem.set_u32(element, 1);
            }
            element = element.wrapping_add(4);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    e.mem.set_i32(array + 4, count);
    e.mem.set_u8(base + 0x24, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_fa10, fn_00d1fa10(Ptr)),
        entry!(0x00d1_fa20, fn_00d1fa20(Ptr)),
        entry!(0x00d1_fa50, fn_00d1fa50(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d1fa10_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, 0x1234_5678);
        e.call_log = Some(vec![]);
        e.call(0x00d1_fa10, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this), 0x1234_5678);
    }

    #[test]
    fn fn_00d1fa20_fills_the_array_with_ones_and_clears_the_flag() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u8(this + 0x24, 1);
        for index in 0..8 {
            e.mem.set_u32(data + index * 4, 0xffff_ffff);
        }
        e.call(0x00d1_fa20, &args![this]);
        let words: Vec<u32> = (0..8).map(|index| e.mem.u32(data + index * 4)).collect();
        assert_eq!(
            words,
            vec![
                1,
                1,
                1,
                0xffff_ffff,
                0xffff_ffff,
                0xffff_ffff,
                0xffff_ffff,
                0xffff_ffff
            ]
        );
        assert_eq!(e.mem.u8(this + 0x24), 0);
        // A size that is not positive stores nothing but still clears the flag.
        e.mem.set_u32(data, 7);
        e.mem.set_i32(this + 0x1c, -2);
        e.mem.set_u8(this + 0x24, 1);
        e.call(0x00d1_fa20, &args![this]);
        assert_eq!(e.mem.u32(data), 7);
        assert_eq!(e.mem.u8(this + 0x24), 0);
    }

    /// A router whose allocator word is `0x7777` and a reserve double that gives the array a
    /// buffer of `capacity` words.
    fn install_reserve(e: &mut Engine) -> u32 {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x7777);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let buffer = e.mem.alloc(0x100);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, a[2] | 0x8000_0000);
            Ret::default()
        });
        buffer
    }

    #[test]
    fn fn_00d1fa50_copies_the_source_and_fills_the_array_with_ones() {
        let mut e = Engine::new();
        let buffer = install_reserve(&mut e);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x1c, 3);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, owner);
        e.mem.set_u32(source + 4, 0x2222);
        e.mem.set_u32(source + 8, 0x3333);
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x24, 1);
        e.call_log = Some(vec![]);
        let r = e.call(0x00d1_fa50, &args![this, source]).u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), 0x010c_e1a8);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), owner);
        assert_eq!(e.mem.u32(this + 0xc), 0x2222);
        assert_eq!(e.mem.u32(this + 0x10), 0x3333);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        assert_eq!(e.mem.u32(this + 0x18), buffer);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        assert_eq!(e.mem.u8(this + 0x24), 0);
        for index in 0..3 {
            assert_eq!(e.mem.u32(buffer + index * 4), 1);
        }
        assert_eq!(e.mem.u32(buffer + 12), 0);
        let log = e.call_log.take().unwrap();
        let reserves: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_RESERVE).collect();
        // max(3, 2 * 0) words of 4 bytes with the router's allocator word.
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0].1, vec![0x7777, this + 0x18, 3, 4]);
    }

    #[test]
    fn fn_00d1fa50_with_no_elements_reserves_nothing() {
        let mut e = Engine::new();
        install_reserve(&mut e);
        let owner = e.mem.alloc(0x40);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, owner);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x00d1_fa50, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != ARRAY_RESERVE));
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
    }
}
