//! `Havok SDK/run_00cfa7a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cfa7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_86d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cfa7a0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_86d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cfa7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, one word (`RET 4`): copies a 0x74-byte record
/// from `source` to `this` field by field. Words at `+0`, `+4`, `+8`, `+0xc`,
/// `+0x10` and `+0x14`; 16-byte blocks at `+0x20`, `+0x30` and `+0x40`;
/// words at `+0x50`, `+0x54`, `+0x58`, `+0x5c`, `+0x60` and `+0x68`; the
/// bytes at `+0x6c` and `+0x6d`; the word at `+0x70`; and the word at `+4`
/// once more. The bytes in between (`+0x18` to `+0x1f`, `+0x64`, `+0x6e`,
/// `+0x6f`) are not copied.
pub fn fn_00cfa7d0(e: &mut Engine, this: Ptr, source: Ptr) {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0x20u32, 0x30, 0x40] {
        for lane in [0u32, 4, 8, 12] {
            let word = e.mem.u32(from + offset + lane);
            e.mem.set_u32(to + offset + lane, word);
        }
    }
    for offset in [0x50u32, 0x54, 0x58, 0x5c, 0x60, 0x68] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0x6cu32, 0x6d] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    let word = e.mem.u32(from + 0x70);
    e.mem.set_u32(to + 0x70, word);
    let word = e.mem.u32(from + 4);
    e.mem.set_u32(to + 4, word);
}

// Translated from 00cfa860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of a class with several table pointers (the class is unconfirmed): writes the
/// table words `010c3b8c` at `+0x14`, `010c3bac` at `+0x10`, `010c3b98` at `+0xc` and `010c3b80`
/// at `+8`, then the base table `0102e388` at `+0`.
pub fn fn_00cfa860(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 8, 0x010c_3b80);
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cfa7d0, fn_00cfa7d0(Ptr, Ptr)),
        entry!(0x00cf_a860, fn_00cfa860(Ptr)),
        entry!(0x00cf_a7a0, deleting_destructor_00cfa7a0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00cfa7a0_frees_only_when_bit_zero_is_set() {
        check(0x00cf_a7a0, 0x00cf_86d0);
    }

    #[test]
    fn fn_00cfa7d0_copies_the_listed_fields_and_nothing_else() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x74);
        let target = e.mem.alloc(0x74);
        for offset in (0..0x74).step_by(4) {
            e.mem.set_u32(source + offset, 0x1000_0000 + offset);
            e.mem.set_u32(target + offset, 0xdddd_dddd);
        }
        e.call(
            0x00cf_a7d0,
            &args![Ptr::<()>::new(target), Ptr::<()>::new(source)],
        );
        for offset in (0..0x74).step_by(4).filter(|offset| *offset != 0x6c) {
            let copied = [
                0, 4, 8, 0xc, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
                0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x68, 0x70,
            ]
            .contains(&offset);
            let expected = if copied {
                0x1000_0000 + offset
            } else {
                0xdddd_dddd
            };
            assert_eq!(e.mem.u32(target + offset), expected, "offset {offset:#x}");
        }
        // Only the bytes at 0x6c and 0x6d of that word are copied.
        assert_eq!(e.mem.u8(target + 0x6c), e.mem.u8(source + 0x6c));
        assert_eq!(e.mem.u8(target + 0x6d), e.mem.u8(source + 0x6d));
        assert_eq!(e.mem.u8(target + 0x6e), 0xdd);
        assert_eq!(e.mem.u8(target + 0x6f), 0xdd);
    }
}

#[cfg(test)]
mod tests_00cfa860 {
    use super::*;

    /// The five table words are written and nothing else.
    #[test]
    fn writes_the_table_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xaa; 0x20]);
        e.call(0x00cf_a860, &args![this]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                0x0102_e388,
                0xaaaa_aaaa,
                0x010c_3b80,
                0x010c_3b98,
                0x010c_3bac,
                0x010c_3b8c
            ]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0xaaaa_aaaa);
    }
}
