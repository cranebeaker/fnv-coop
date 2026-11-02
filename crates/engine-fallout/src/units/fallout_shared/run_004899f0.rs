//! `fallout shared/run_004899f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual table the constructor stores last (`0101d0d4`), over the one its
/// base constructor stored.
const VTABLE: u32 = 0x0101_d0d4;

// Translated from 00489a60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00489a60 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the component at `+0x114` of `TESCreature` (`005f7230` calls it
/// there). The engine map gives it no name, so the class is unconfirmed. `ECX` is
/// the object. It runs the base constructor `00402d30` (which stores the
/// virtual table `01011d68`), stores the virtual table `0101d0d4`, constructs the
/// two pairs of words at `+4` and `+0xc` (`0096a2d0`: both cleared) and runs the
/// empty `00483710`. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_00489a60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(base, VTABLE);
    e.call(0x0096_a2d0, &args![base + 4]);
    e.call(0x0096_a2d0, &args![base + 0xc]);
    e.call(0x0048_3710, &args![this]);
    this
}

/// `00489620` (`thiscall`): frees the array at +4 (`00401030`), clears it
/// and the count byte at +0.
const ARRAY_CLEAR_00489620: u32 = 0x0048_9620;
/// The game's `operator new` (`00401000`, `cdecl`, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `00401460` (`cdecl`: destination, source, size): `memcpy` through
/// `00ec44d0`.
const MEMCPY_00401460: u32 = 0x0040_1460;
/// `TESModelList::ClearDataComponent` (`00489b40`, engine map; `thiscall`).
const MODEL_LIST_CLEAR_DATA_COMPONENT: u32 = 0x0048_9b40;
/// `0046ffb0` (`thiscall`): runs `00470470` on `this` (a simple list's
/// clean-up).
const LIST_DESTROY_0046FFB0: u32 = 0x0046_ffb0;

// Translated from 004899f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a byte-counted array of words (`thiscall (this; other)`; no Xbox
/// PDB name): clears `this` (`00489620`), copies the count byte at +0,
/// allocates `count * 4` bytes (all ones when that overflows, as the
/// compiler's checked `new[]` does) into +4 and copies `other`'s words there.
pub fn fn_004899f0(e: &mut Engine, this: Ptr, other: Ptr) {
    e.call(ARRAY_CLEAR_00489620, &args![this]);
    let count = e.mem.u8(other.addr());
    e.mem.set_u8(this.addr(), count);
    let count = u32::from(e.mem.u8(this.addr()));
    // MUL by 4, SETO / NEG / OR: an overflowing size becomes 0xFFFFFFFF
    // (a byte count never overflows; translated as the exe has it).
    let (bytes, overflow) = count.overflowing_mul(4);
    let size = if overflow { 0xffff_ffff } else { bytes };
    let block = e.call(ALLOCATE, &args![size]).u32();
    e.mem.set_u32(this.addr() + 4, block);
    let length = u32::from(e.mem.u8(this.addr())) << 2;
    let source = e.mem.u32(other.addr() + 4);
    let destination = e.mem.u32(this.addr() + 4);
    e.call(MEMCPY_00401460, &args![destination, source, length]);
}

// Translated from 00489ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00489ae0 - the C++ exception unwinding frame (FS:[0], handler 00f08f1b) is not translated
/// The destructor of the component whose vtable is `0101d0d4` (RTTI
/// `.?AVTESModelList@@`; `thiscall`; no Xbox PDB name): stores that vtable,
/// runs `TESModelList::ClearDataComponent` and destroys the list at +4
/// (`0046ffb0`). The exception-unwinding frame is not translated.
pub fn fn_00489ae0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(MODEL_LIST_CLEAR_DATA_COMPONENT, &args![this]);
    e.call(LIST_DESTROY_0046FFB0, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00489a60, fn_00489a60(Ptr) -> Ptr),
        entry!(0x0048_99f0, fn_004899f0(Ptr, Ptr)),
        entry!(0x0048_9ae0, fn_00489ae0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00489a60_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_9a60,
            size: 0x14,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d30, &[A::T(0x0)]),
                (0x0096_a2d0, &[A::T(0x4)]),
                (0x0096_a2d0, &[A::T(0xc)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_d0d4)],
        });
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_004899f0_copies_the_counted_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u8(other, 3);
        e.mem.set_u32(other + 4, 0x7000);
        e.register(ARRAY_CLEAR_00489620, |_, _| Ret::default());
        e.register(ALLOCATE, |_, a| word(0x9000 + a[0]));
        e.register(MEMCPY_00401460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_99f0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(this), 3);
        assert_eq!(e.mem.u32(this + 4), 0x900c);
        assert_eq!(
            log[1..],
            [
                (ARRAY_CLEAR_00489620, vec![this]),
                (ALLOCATE, vec![12]),
                (MEMCPY_00401460, vec![0x900c, 0x7000, 12])
            ]
        );
    }

    #[test]
    fn fn_00489ae0_clears_and_destroys_the_component() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(MODEL_LIST_CLEAR_DATA_COMPONENT, |_, _| Ret::default());
        e.register(LIST_DESTROY_0046FFB0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_9ae0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            e.call_log.take().unwrap()[1..],
            [
                (MODEL_LIST_CLEAR_DATA_COMPONENT, vec![this]),
                (LIST_DESTROY_0046FFB0, vec![this + 4])
            ]
        );
    }
}
