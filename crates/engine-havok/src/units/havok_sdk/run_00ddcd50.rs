//! `Havok SDK/run_00ddcd50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::compare_flags;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ddcd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_cc60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ddcd70(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_cc60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00ddcd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00ddcd70`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00ddcd50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dd_cd70, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00ddcd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00ddcd70`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00ddcd60(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dd_cd70, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00ddcda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `out` (a pointer to a byte), `a`, `b` (pointers to records
/// whose `float` at `+8` is compared)). Writes 1 at `out` when `a.f8 < b.f8` (x87 compare; a NaN
/// gives 0), else 0, and answers `out` (the hidden result pointer of a returned `hkBool`).
pub fn fn_00ddcda0(e: &mut Engine, out: u32, a: u32, b: u32) -> u32 {
    let first = x87_load32(e.mem.u32(a + 8));
    let second = x87_load32(e.mem.u32(b + 8));
    let flags = compare_flags(second, first);
    e.mem.set_u8(out, u8::from(flags & 0x41 == 0));
    out
}

// Translated from 00ddcdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `word`, `index`, `value`). Replaces the 5-bit field number
/// `index` of `word` by `value`: with `shift = (index * 5) & 0x1f` the answer is
/// `(word & !(0x1f << shift)) | (value << shift)`.
pub fn fn_00ddcdd0(_e: &mut Engine, word: u32, index: u32, value: u32) -> u32 {
    let shift = index.wrapping_mul(5) & 0x1f;
    (!(0x1fu32 << shift) & word) | (value << shift)
}

// Translated from 00ddcdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `word` (its low 16 bits are used), `index` (a `char`: its low
/// byte)). Answers the 5-bit field number `index` of the `u16`: `(word >> shift) & 0x1f` with
/// `shift = (index as char * 5) & 0x1f` (the code multiplies the byte with `IMUL CL`; only the
/// low five bits of the product are used).
pub fn fn_00ddcdf0(_e: &mut Engine, word: u32, index: u32) -> u32 {
    let shift = (i32::from(index as u8 as i8) * 5) as u32 & 0x1f;
    (u32::from(word as u16) >> shift) & 0x1f
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_cd70, deleting_destructor_00ddcd70(Ptr, u8) -> Ptr),
        entry!(0x00dd_cd50, fn_00ddcd50(Ptr, u32) -> u32),
        entry!(0x00dd_cd60, fn_00ddcd60(Ptr, u32) -> u32),
        entry!(0x00ddcda0, fn_00ddcda0(u32, u32, u32) -> u32),
        entry!(0x00ddcdd0, fn_00ddcdd0(u32, u32, u32) -> u32),
        entry!(0x00ddcdf0, fn_00ddcdf0(u32, u32) -> u32),
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
    fn deleting_destructor_00ddcd70_frees_only_when_bit_zero_is_set() {
        check(0x00dd_cd70, 0x00dd_cc60);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00ddcd50() {
        assert_eq!(check(0x00dd_cd50, 0x00dd_cd70, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00ddcd60() {
        assert_eq!(check(0x00dd_cd60, 0x00dd_cd70, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod a07_bitfield_tests {
    use super::*;

    fn record(e: &mut Engine, bits: u32) -> u32 {
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at + 8, bits);
        at
    }

    #[test]
    fn the_comparison_writes_a_byte_and_answers_its_address() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let one = record(&mut e, 1.0f32.to_bits());
        let two = record(&mut e, 2.0f32.to_bits());
        let nan = record(&mut e, f32::NAN.to_bits());
        for (a, b, expected) in [
            (one, two, 1u8),
            (two, one, 0),
            (one, one, 0),
            (one, nan, 0),
            (nan, one, 0),
        ] {
            e.mem.set_u8(out, 9);
            assert_eq!(e.call(0x00dd_cda0, &args![out, a, b]).u32(), out);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn a_five_bit_field_is_replaced_and_read() {
        let mut e = Engine::new();
        // Field 1 occupies bits 5 to 9.
        assert_eq!(
            e.call(0x00dd_cdd0, &args![0xffffu32, 1u32, 3u32]).u32(),
            0xfc7f
        );
        // The shift is (index * 5) mod 32: index 7 is shift 3.
        assert_eq!(
            e.call(0x00dd_cdd0, &args![0u32, 7u32, 0x1fu32]).u32(),
            0x1f << 3
        );
        assert_eq!(e.call(0x00dd_cdf0, &args![0x1234u32, 2u32]).u32(), 4);
        // Only the low word is used, and the index is a signed byte.
        assert_eq!(e.call(0x00dd_cdf0, &args![0xabcd_1234u32, 2u32]).u32(), 4);
        assert_eq!(e.call(0x00dd_cdf0, &args![0xffffu32, 0xffu32]).u32(), 0);
        assert_eq!(e.call(0x00dd_cdf0, &args![0x8000u32, 3u32]).u32(), 1);
    }
}
