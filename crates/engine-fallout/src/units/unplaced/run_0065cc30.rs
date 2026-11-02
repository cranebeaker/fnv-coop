//! `(unplaced)/run_0065cc30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_invalid_parameter_noinfo`-style report the bounds checks call (`00ec7c56`; no
/// arguments).
const INDEX_FAILURE: u32 = 0x00ec_7c56;
/// Callees of the container functions, by address (all `thiscall`).
const CONSTRUCT_ALLOCATOR: u32 = 0x0068_15c0;
const CONTAINER_INIT: u32 = 0x0065_d3a0;
const RESERVE_EIGHT_BYTE: u32 = 0x0065_d7e0;
const RESERVE_LARGE: u32 = 0x0065_d980;
const DESTROY_SMALL: u32 = 0x0065_d860;
const DESTROY_LARGE: u32 = 0x0065_da00;
const CONTAINER_RELEASE: u32 = 0x0065_cfb0;
const ITERATOR_BEGIN: u32 = 0x0065_f550;
const ITERATOR_END: u32 = 0x0044_a170;
const ERASE_SMALL: u32 = 0x0065_d740;
const ERASE_LARGE: u32 = 0x0065_d8e0;

/// The two-word iterator a getter writes into a stack temporary and answers the address
/// of; read from the answered address.
fn iterator(e: &mut Engine, getter: u32, this: Ptr) -> (u32, u32) {
    e.with_stack(8, |e, slot| {
        let at = e.call(getter, &args![this, slot]).u32();
        (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)))
    })
}

/// The constructor shape `0065cca0` and `0065cdd0` share: the allocator constructed in
/// the pushed word that holds `this` (`006815c0`), the container initialised with that word
/// (`0065d3a0`), then `reserve` with 0.
fn construct(e: &mut Engine, this: Ptr, reserve: u32) -> Ptr {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), this.addr());
        e.call(CONSTRUCT_ALLOCATOR, &args![word]);
        let allocator = e.mem.u32(word.addr());
        e.call(CONTAINER_INIT, &args![this, allocator]);
    });
    e.call(reserve, &args![this, 0u32]);
    this
}

// Translated from 0065cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the byte size of the vector's
/// storage, end (`+0x10`) minus begin (`+0xC`).
pub fn fn_0065cc50(e: &mut Engine, this: Ptr) -> u32 {
    let end = e.mem.u32(this.addr() + 0x10);
    end.wrapping_sub(e.mem.u32(this.addr() + 0xc))
}

// Translated from 0065cc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the element count of a vector of
/// 0x38-byte elements (`(end - begin) / 0x38`, signed division).
pub fn fn_0065cc30(e: &mut Engine, this: Ptr) -> i32 {
    let end = e.mem.u32(this.addr() + 0x10);
    (end.wrapping_sub(e.mem.u32(this.addr() + 0xc)) as i32) / 0x38
}

// Translated from 0065cc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the address of byte `index` of the
/// vector; an index not below [`fn_0065cc50`] first reports the failure (`00ec7c56`).
pub fn fn_0065cc70(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index >= fn_0065cc50(e, this) {
        e.call(INDEX_FAILURE, &args![]);
    }
    e.mem.u32(this.addr() + 0xc).wrapping_add(index)
}

// Translated from 0065cea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the element count of a vector of
/// 0x58-byte elements (`(end - begin) / 0x58`, signed division).
pub fn fn_0065cea0(e: &mut Engine, this: Ptr) -> i32 {
    let end = e.mem.u32(this.addr() + 0x10);
    (end.wrapping_sub(e.mem.u32(this.addr() + 0xc)) as i32) / 0x58
}

// Translated from 0065cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the address of element `index` (0x58
/// bytes each); an index not below [`fn_0065cea0`] (unsigned compare) first reports the
/// failure (`00ec7c56`).
pub fn fn_0065cec0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index >= fn_0065cea0(e, this) as u32 {
        e.call(INDEX_FAILURE, &args![]);
    }
    index
        .wrapping_mul(0x58)
        .wrapping_add(e.mem.u32(this.addr() + 0xc))
}

// Translated from 0065cca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065cca0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A container constructor (unnamed in the engine map; `thiscall`, plain `RET`): the
/// allocator built in the stack word that holds `this` (`006815c0`), the container set up
/// with it (`0065d3a0`), then `0065d7e0` with 0. Answers `this`.
pub fn fn_0065cca0(e: &mut Engine, this: Ptr) -> Ptr {
    construct(e, this, RESERVE_EIGHT_BYTE)
}

// Translated from 0065cdd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065cdd0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A container constructor (unnamed in the engine map; `thiscall`, plain `RET`): as
/// [`fn_0065cca0`] with `0065d980` as the last step. Answers `this`.
pub fn fn_0065cdd0(e: &mut Engine, this: Ptr) -> Ptr {
    construct(e, this, RESERVE_LARGE)
}

// Translated from 0065cd10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065cd10 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A container destructor (unnamed in the engine map; `thiscall`): `0065d860`, then
/// `0065cfb0`.
pub fn fn_0065cd10(e: &mut Engine, this: Ptr) {
    e.call(DESTROY_SMALL, &args![this]);
    e.call(CONTAINER_RELEASE, &args![this]);
}

// Translated from 0065ce40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065ce40 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A container destructor (unnamed in the engine map; `thiscall`): `0065da00`, then
/// `0065cfb0`.
pub fn fn_0065ce40(e: &mut Engine, this: Ptr) {
    e.call(DESTROY_LARGE, &args![this]);
    e.call(CONTAINER_RELEASE, &args![this]);
}

// Translated from 0065cd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A container clear (unnamed in the engine map; `thiscall`, plain `RET`): the iterators
/// `0065f550` and `0044a170` (two words each, through stack temporaries), then the erase
/// `0065d740` with a result temporary, the second iterator, then the first.
pub fn fn_0065cd70(e: &mut Engine, this: Ptr) {
    let first = iterator(e, ITERATOR_BEGIN, this);
    let second = iterator(e, ITERATOR_END, this);
    e.with_stack(8, |e, result| {
        e.call(
            ERASE_SMALL,
            &args![this, result, second.0, second.1, first.0, first.1],
        );
    });
}

// Translated from 0065cef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A container clear (unnamed in the engine map; `thiscall`, plain `RET`): as
/// [`fn_0065cd70`] with the erase `0065d8e0`.
pub fn fn_0065cef0(e: &mut Engine, this: Ptr) {
    let first = iterator(e, ITERATOR_BEGIN, this);
    let second = iterator(e, ITERATOR_END, this);
    e.with_stack(8, |e, result| {
        e.call(
            ERASE_LARGE,
            &args![this, result, second.0, second.1, first.0, first.1],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_cc30, fn_0065cc30(Ptr) -> i32),
        entry!(0x0065_cc50, fn_0065cc50(Ptr) -> u32),
        entry!(0x0065_cc70, fn_0065cc70(Ptr, u32) -> u32),
        entry!(0x0065_cea0, fn_0065cea0(Ptr) -> i32),
        entry!(0x0065_cec0, fn_0065cec0(Ptr, u32) -> u32),
        entry!(0x0065_cca0, fn_0065cca0(Ptr) -> Ptr),
        entry!(0x0065_cdd0, fn_0065cdd0(Ptr) -> Ptr),
        entry!(0x0065_cd10, fn_0065cd10(Ptr)),
        entry!(0x0065_ce40, fn_0065ce40(Ptr)),
        entry!(0x0065_cd70, fn_0065cd70(Ptr)),
        entry!(0x0065_cef0, fn_0065cef0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn vector(e: &mut Engine, begin: u32, end: u32) -> u32 {
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, begin);
        e.mem.set_u32(this + 0x10, end);
        this
    }

    #[test]
    fn counts_divide_the_byte_size() {
        let mut e = Engine::new();
        let this = vector(&mut e, 0x1000, 0x1000 + 3 * 0x38);
        assert_eq!(e.call(0x0065_cc30, &args![this]).u32(), 3);
        assert_eq!(e.call(0x0065_cc50, &args![this]).u32(), 3 * 0x38);
        let this = vector(&mut e, 0x1000, 0x1000 + 2 * 0x58 + 4);
        assert_eq!(e.call(0x0065_cea0, &args![this]).u32(), 2);
        // A reversed vector divides toward zero.
        let this = vector(&mut e, 0x1000 + 0x60, 0x1000);
        assert_eq!(e.call(0x0065_cea0, &args![this]).i32(), -1);
    }

    #[test]
    fn element_addresses_report_bad_indices() {
        let mut e = Engine::new();
        e.register(INDEX_FAILURE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = vector(&mut e, 0x2000, 0x2010);
        assert_eq!(e.call(0x0065_cc70, &args![this, 5u32]).u32(), 0x2005);
        assert!(calls(&e, INDEX_FAILURE).is_empty());
        assert_eq!(e.call(0x0065_cc70, &args![this, 0x10u32]).u32(), 0x2010);
        assert_eq!(calls(&e, INDEX_FAILURE).len(), 1);
        let this = vector(&mut e, 0x3000, 0x3000 + 2 * 0x58);
        assert_eq!(e.call(0x0065_cec0, &args![this, 1u32]).u32(), 0x3058);
        assert_eq!(calls(&e, INDEX_FAILURE).len(), 1);
        assert_eq!(e.call(0x0065_cec0, &args![this, 2u32]).u32(), 0x30b0);
        assert_eq!(calls(&e, INDEX_FAILURE).len(), 2);
    }

    #[test]
    fn constructors_set_up_and_reserve() {
        let mut e = Engine::new();
        e.register(CONSTRUCT_ALLOCATOR, |e, a| {
            assert_ne!(e.mem.u32(a[0]), 0);
            Ret::default()
        });
        for address in [CONTAINER_INIT, RESERVE_EIGHT_BYTE, RESERVE_LARGE] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_cca0, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(e.call(0x0065_cdd0, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(
            calls(&e, CONTAINER_INIT),
            vec![vec![0x4000, 0x4000], vec![0x5000, 0x5000]]
        );
        assert_eq!(calls(&e, RESERVE_EIGHT_BYTE), vec![vec![0x4000, 0]]);
        assert_eq!(calls(&e, RESERVE_LARGE), vec![vec![0x5000, 0]]);
    }

    #[test]
    fn destructors_run_both_steps() {
        let mut e = Engine::new();
        for address in [DESTROY_SMALL, DESTROY_LARGE, CONTAINER_RELEASE] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0065_cd10, &args![0x4000u32]);
        e.call(0x0065_ce40, &args![0x5000u32]);
        assert_eq!(calls(&e, DESTROY_SMALL), vec![vec![0x4000]]);
        assert_eq!(calls(&e, DESTROY_LARGE), vec![vec![0x5000]]);
        assert_eq!(
            calls(&e, CONTAINER_RELEASE),
            vec![vec![0x4000], vec![0x5000]]
        );
    }

    #[test]
    fn clears_erase_between_the_iterators() {
        let mut e = Engine::new();
        e.register(ITERATOR_BEGIN, |e, a| {
            e.mem.set_u32(a[1], 0x11);
            e.mem.set_u32(a[1] + 4, 0x12);
            a[1].into_ret()
        });
        e.register(ITERATOR_END, |e, a| {
            e.mem.set_u32(a[1], 0x21);
            e.mem.set_u32(a[1] + 4, 0x22);
            a[1].into_ret()
        });
        e.register(ERASE_SMALL, |_, _| Ret::default());
        e.register(ERASE_LARGE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_cd70, &args![0x4000u32]);
        e.call(0x0065_cef0, &args![0x5000u32]);
        let small = &calls(&e, ERASE_SMALL)[0];
        assert_eq!(
            (small[0], &small[2..]),
            (0x4000, &[0x21, 0x22, 0x11, 0x12][..])
        );
        let large = &calls(&e, ERASE_LARGE)[0];
        assert_eq!(
            (large[0], &large[2..]),
            (0x5000, &[0x21, 0x22, 0x11, 0x12][..])
        );
    }
}
