//! `Havok SDK/run_00cfcb80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cfcc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_c590`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cfcc30(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_c590, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cfcb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(a, b)` on two unsigned 16-bit words: the smaller one (`a` when `a < b`, else `b`).
pub fn fn_00cfcb80(_e: &mut Engine, a: u16, b: u16) -> u16 {
    a.min(b)
}

// Translated from 00cfcba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a 0x34-byte record (the class is unconfirmed) from ten stack words `(w0, p1, w2, w3, w4,
/// w5, w6, w7, w8, f9)` and returns `this`: `+0` takes `w4`, `+4` `w5`, `+8` the value 1,
/// `+0x10` `w6`, `+0x14` `w7`, `+0x18` `w8`, the float `f9` is copied through the x87 stack to
/// `+0x1c` (a signalling NaN is quieted), `+0x20` is 0, `+0x24` takes the pointer `p1`, `+0x28`
/// `w2`, `+0x2c` `w3` and the 16-bit word at `+0x30` the low word of `w0`. The 16-bit word at
/// `+0x32` is the 16-bit word at `p1 + 0x12` when the word at `p1 + 8` is 0, otherwise 0.
pub fn fn_00cfcba0(
    e: &mut Engine,
    this: Ptr,
    w0: u16,
    p1: Ptr,
    w2: u32,
    w3: u32,
    w4: u32,
    w5: u32,
    w6: u32,
    w7: u32,
    w8: u32,
    f9: f32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 4, w5);
    e.mem.set_u32(base, w4);
    e.mem.set_u32(base + 8, 1);
    e.mem.set_u32(base + 0x1c, x87_quiet_bits(f9.to_bits()));
    e.mem.set_u32(base + 0x14, w7);
    e.mem.set_u32(base + 0x10, w6);
    e.mem.set_u32(base + 0x28, w2);
    e.mem.set_u32(base + 0x18, w8);
    e.mem.set_u32(base + 0x2c, w3);
    e.mem.set_u32(base + 0x24, p1.addr());
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u16(base + 0x30, w0);
    if e.mem.u32(p1.addr() + 8) == 0 {
        let word = e.mem.u16(p1.addr() + 0x12);
        e.mem.set_u16(base + 0x32, word);
    } else {
        e.mem.set_u16(base + 0x32, 0);
    }
    this
}

// Translated from 00cfcc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `this` is not null, calls its virtual method `+0` (the deleting destructor) with the
/// flag 1.
pub fn fn_00cfcc20(e: &mut Engine, this: Ptr) {
    if !this.is_null() {
        e.vcall(this.addr(), 0, &args![1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_cb80, fn_00cfcb80(u16, u16) -> u16),
        entry!(0x00cf_cba0, fn_00cfcba0(Ptr, u16, Ptr, u32, u32, u32, u32, u32, u32, u32, f32) -> Ptr),
        entry!(0x00cf_cc20, fn_00cfcc20(Ptr)),
        entry!(0x00cf_cc30, deleting_destructor_00cfcc30(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cfcc30_frees_only_when_bit_zero_is_set() {
        check(0x00cf_cc30, 0x00cf_c590);
    }
}

#[cfg(test)]
mod tests_00cfcb80 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The smaller word wins, also when only the low 16 bits of the words differ.
    #[test]
    fn minimum_of_two_words() {
        let mut e = Engine::new();
        for (a, b, expected) in [(1u32, 2u32, 1u32), (2, 1, 1), (7, 7, 7), (0xffff, 0, 0)] {
            assert_eq!(e.call(0x00cf_cb80, &args![a, b]).u32(), expected);
        }
        // The high halves of the stack words are not read.
        assert_eq!(
            e.call(0x00cf_cb80, &args![0x1_0005u32, 0x7_0003u32]).u32(),
            3
        );
    }

    /// Every field lands where the exe puts it; the word at `+0x32` follows the word at
    /// `p1 + 8`.
    #[test]
    fn record_is_filled_from_the_stack_words() {
        for (flag, expected) in [(0u32, 0x1234u16), (5, 0)] {
            let mut e = Engine::new();
            let source = e.mem.alloc(0x20);
            e.mem.set_u32(source + 8, flag);
            e.mem.set_u16(source + 0x12, 0x1234);
            let this = e.mem.alloc(0x40);
            e.mem.write(this, &[0xaa; 0x40]);
            let got = e
                .call(
                    0x00cf_cba0,
                    &args![
                        this,
                        0x1_beefu32,
                        source,
                        0x22u32,
                        0x33u32,
                        0x44u32,
                        0x55u32,
                        0x66u32,
                        0x77u32,
                        0x88u32,
                        f32::from_bits(0x7f80_0001)
                    ],
                )
                .u32();
            assert_eq!(got, this);
            let mut image = [0xaau8; 0x40];
            for (offset, word) in [
                (0x00usize, 0x44u32),
                (0x04, 0x55),
                (0x08, 1),
                (0x10, 0x66),
                (0x14, 0x77),
                (0x18, 0x88),
                (0x1c, 0x7fc0_0001),
                (0x20, 0),
                (0x24, source),
                (0x28, 0x22),
                (0x2c, 0x33),
            ] {
                image[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
            }
            image[0x30..0x32].copy_from_slice(&0xbeefu16.to_le_bytes());
            image[0x32..0x34].copy_from_slice(&expected.to_le_bytes());
            assert_eq!(e.mem.bytes(this, 0x40), image.to_vec());
        }
    }

    /// A non-null object has its slot 0 called with 1; null does nothing.
    #[test]
    fn deleting_through_the_vtable_only_when_not_null() {
        const SLOT_DOUBLE: u32 = 0x0900_0301;
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, SLOT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call(0x00cf_cc20, &args![object]);
        e.call(0x00cf_cc20, &args![0u32]);
        assert_eq!(*seen.borrow(), vec![vec![object, 1]]);
    }
}
