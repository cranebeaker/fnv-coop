//! `fallout shared/havok/run_0062a0b0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040fbe0` (engine map name `Error`, a body shared by folding as far as this unit shows):
/// `cdecl`, the block, the count and a byte.
const RELEASE_ELEMENTS: u32 = 0x0040_fbe0;
/// Vtable slot of the allocator's release function.
const ALLOCATOR_RELEASE_SLOT: u32 = 0x10;
/// `004a3b30` (unnamed in the engine map): `thiscall`, plain `RET`; the base constructor.
const BASE_CONSTRUCT: u32 = 0x004a_3b30;
/// `004a3d00` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_CONSTRUCT: u32 = 0x004a_3d00;
/// `004a3b50` (unnamed in the engine map): `thiscall`, plain `RET`.
const BASE_RESET: u32 = 0x004a_3b50;
/// `004a3d60` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_RESET_FIRST: u32 = 0x004a_3d60;
/// `004a3d80` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_RESET_SECOND: u32 = 0x004a_3d80;
/// `008c71b0` (engine map name `BGSChangeFlags::IsForcedChange`, a folded body as `0062a190`
/// uses it: `thiscall`, `RET 4`, a word).
const MEMBER_FROM_WORD: u32 = 0x008c_71b0;
/// `00931ed0` (`fallout/ai/mobileobject.cpp` in the engine map): `thiscall`, `RET 4`, the
/// address of a stack word it fills.
const SOURCE_TO_WORD: u32 = 0x0093_1ed0;
/// `0062a3b0` (`run_0062a3b0.rs`): `thiscall`, `RET 8`.
const ADD_FROM_SOURCE: u32 = 0x0062_a3b0;
/// `0059cee0` (`fallout shared/tesconditionfunctions.cpp` in the engine map): `thiscall`, plain
/// `RET`; the destructor `0062a2f0` runs.
const DESTRUCT: u32 = 0x0059_cee0;
/// `00401030` (unnamed in the engine map): `cdecl`, one word; the delete of a deleting
/// destructor.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vtable `0062a220` stores.
const BASE_VTABLE: u32 = 0x0103_4ec4;
/// The vtable `0062a190` stores.
const COLLECTOR_VTABLE: u32 = 0x0104_e1dc;

// Translated from 0062a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: the reserved element count, the word at `+8` without
/// its top two bits.
pub fn fn_0062a100(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(8)) & 0x3fff_ffff
}

// Translated from 0062a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: hands the block (`+0`), the count (`+4`) and a zero
/// byte to `0040fbe0` (`cdecl`), then clears the count.
pub fn fn_0062a120(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    let count = e.mem.u32(this.addr().wrapping_add(4));
    e.call(RELEASE_ELEMENTS, &args![block, count, 0u32]);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
}

// Translated from 0062a160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. On the allocator `this`, calls the vtable
/// slot `+0x10` with `block` and `count * 4` (`SHL 2`): the release of an array of words.
pub fn fn_0062a160(e: &mut Engine, this: Ptr, block: u32, count: u32) {
    e.vcall(
        this.addr(),
        ALLOCATOR_RELEASE_SLOT,
        &args![block, count << 2],
    );
}

// Translated from 0062a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Clears the array `this` (`0062a120`); unless
/// bit 31 of the word at `+8` is set (the block belongs to someone else), releases the block at
/// `+0` through `allocator` (`0062a160`, with the reserved count `0062a100` answers). Then
/// clears `+0` and sets the word at `+8` to `0x80000000`.
pub fn fn_0062a0b0(e: &mut Engine, this: Ptr, allocator: Ptr) {
    fn_0062a120(e, this);
    if e.mem.u32(this.addr().wrapping_add(8)) & 0x8000_0000 == 0 {
        let reserved = fn_0062a100(e, this);
        let block = e.mem.u32(this.addr());
        fn_0062a160(e, allocator, block, reserved);
    }
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr().wrapping_add(8), 0x8000_0000);
}

// Translated from 0062a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: runs `004a3d60` and `004a3d80` on `this`.
pub fn fn_0062a2d0(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_RESET_FIRST, &args![this]);
    e.call(MEMBER_RESET_SECOND, &args![this]);
}

// Translated from 0062a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: clears the word at `+0x50`, then `0062a2d0`.
pub fn fn_0062a2b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr().wrapping_add(0x50), 0);
    fn_0062a2d0(e, this);
}

// Translated from 0062a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: `0062a2b0` on the member at `+0x10`, then `004a3b50`
/// on `this`.
pub fn fn_0062a290(e: &mut Engine, this: Ptr) {
    fn_0062a2b0(e, Ptr::new(this.addr().wrapping_add(0x10)));
    e.call(BASE_RESET, &args![this]);
}

// Translated from 0062a220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062a220 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `fastcall`, plain `RET`. A constructor: the base (`004a3b30`), the
/// vtable `01034ec4`, the member at `+0x10` (`004a3d00`), then the reset `0062a290`. Answers
/// `this`.
pub fn fn_0062a220(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    e.call(MEMBER_CONSTRUCT, &args![this.addr().wrapping_add(0x10)]);
    fn_0062a290(e, this);
    this
}

// Translated from 0062a190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062a190 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `thiscall`, `RET 8`. A constructor: `0062a220`, the vtable
/// `0104e1dc`, the member at `+0x70` from 0 (`008c71b0`); with a `source`, has it fill a stack
/// word (`00931ed0`) and hands `first` and that word's address to `0062a3b0`. Answers `this`.
pub fn fn_0062a190(e: &mut Engine, this: Ptr, first: u32, source: u32) -> Ptr {
    fn_0062a220(e, this);
    e.mem.set_u32(this.addr(), COLLECTOR_VTABLE);
    e.call(
        MEMBER_FROM_WORD,
        &args![this.addr().wrapping_add(0x70), 0u32],
    );
    if source != 0 {
        e.with_stack(4, |e, word| {
            e.call(SOURCE_TO_WORD, &args![source, word]);
            e.call(ADD_FROM_SOURCE, &args![this, first, word]);
        });
    }
    this
}

// Translated from 0062a2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. A deleting destructor: `0059cee0`, then with
/// bit 0 of `flags` the delete `00401030`. Answers `this`.
pub fn fn_0062a2f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_a0b0, fn_0062a0b0(Ptr, Ptr)),
        entry!(0x0062_a100, fn_0062a100(Ptr) -> u32),
        entry!(0x0062_a120, fn_0062a120(Ptr)),
        entry!(0x0062_a160, fn_0062a160(Ptr, u32, u32)),
        entry!(0x0062_a190, fn_0062a190(Ptr, u32, u32) -> Ptr),
        entry!(0x0062_a220, fn_0062a220(Ptr) -> Ptr),
        entry!(0x0062_a290, fn_0062a290(Ptr)),
        entry!(0x0062_a2b0, fn_0062a2b0(Ptr)),
        entry!(0x0062_a2d0, fn_0062a2d0(Ptr)),
        entry!(0x0062_a2f0, fn_0062a2f0(Ptr, u32) -> Ptr),
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
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// An allocator whose vtable slot `+0x10` is a double at `0x0700_0010`.
    fn allocator(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0700_0010);
        e.register(0x0700_0010, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn at_0062a100_masks_the_top_two_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xc000_0007);
        assert_eq!(e.call(0x0062_a100, &args![this]).u32(), 7);
    }

    #[test]
    fn at_0062a120_releases_the_elements_and_clears_the_count() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(RELEASE_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x9000);
        e.mem.set_u32(this + 4, 3);
        e.call(0x0062_a120, &args![this]);
        assert_eq!(calls(&e, RELEASE_ELEMENTS), vec![vec![0x9000, 3, 0]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this), 0x9000);
    }

    #[test]
    fn at_0062a160_releases_four_bytes_per_element() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.call(0x0062_a160, &args![allocator, 0x9000u32, 5u32]);
        e.call(0x0062_a160, &args![allocator, 0x9000u32, 0x4000_0001u32]);
        assert_eq!(
            calls(&e, 0x0700_0010),
            vec![vec![allocator, 0x9000, 20], vec![allocator, 0x9000, 4]]
        );
    }

    #[test]
    fn at_0062a0b0_releases_an_owned_block() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.register(RELEASE_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x9000);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, 0x4000_0006);
        e.call(0x0062_a0b0, &args![this, allocator]);
        assert_eq!(calls(&e, 0x0700_0010), vec![vec![allocator, 0x9000, 24]]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
    }

    #[test]
    fn at_0062a0b0_keeps_a_block_it_does_not_own() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.register(RELEASE_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x9000);
        e.mem.set_u32(this + 8, 0x8000_0006);
        e.call(0x0062_a0b0, &args![this, allocator]);
        assert!(calls(&e, 0x0700_0010).is_empty());
        assert_eq!(calls(&e, RELEASE_ELEMENTS).len(), 1);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
    }

    fn reset_doubles(e: &mut Engine) {
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCT, |_, _| Ret::default());
        e.register(BASE_RESET, |_, _| Ret::default());
        e.register(MEMBER_RESET_FIRST, |_, _| Ret::default());
        e.register(MEMBER_RESET_SECOND, |_, _| Ret::default());
    }

    #[test]
    fn at_0062a290_resets_the_member_then_the_base() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        reset_doubles(&mut e);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x60, 0x55);
        e.call(0x0062_a290, &args![this]);
        assert_eq!(e.mem.u32(this + 0x60), 0, "the member's word at +0x50");
        assert_eq!(
            order(&e),
            vec![
                0x0062_a290,
                MEMBER_RESET_FIRST,
                MEMBER_RESET_SECOND,
                BASE_RESET
            ]
        );
        assert_eq!(calls(&e, MEMBER_RESET_FIRST), vec![vec![this + 0x10]]);
        assert_eq!(calls(&e, BASE_RESET), vec![vec![this]]);
    }

    #[test]
    fn at_0062a2b0_and_0062a2d0_reset_this() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        reset_doubles(&mut e);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x50, 0x55);
        e.call(0x0062_a2b0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        e.call(0x0062_a2d0, &args![this]);
        assert_eq!(calls(&e, MEMBER_RESET_SECOND), vec![vec![this]; 2]);
    }

    #[test]
    fn at_0062a220_constructs_in_order() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        reset_doubles(&mut e);
        let this = e.mem.alloc(0x80);
        assert_eq!(e.call(0x0062_a220, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(
            order(&e),
            vec![
                0x0062_a220,
                BASE_CONSTRUCT,
                MEMBER_CONSTRUCT,
                MEMBER_RESET_FIRST,
                MEMBER_RESET_SECOND,
                BASE_RESET
            ]
        );
        assert_eq!(calls(&e, MEMBER_CONSTRUCT), vec![vec![this + 0x10]]);
    }

    #[test]
    fn at_0062a190_adds_from_the_source_only_when_given_one() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        reset_doubles(&mut e);
        e.register(MEMBER_FROM_WORD, |_, _| Ret::default());
        e.register(SOURCE_TO_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x4242);
            Ret::default()
        });
        e.register(ADD_FROM_SOURCE, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x4242);
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        assert_eq!(e.call(0x0062_a190, &args![this, 0x11u32, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), COLLECTOR_VTABLE);
        assert_eq!(calls(&e, MEMBER_FROM_WORD), vec![vec![this + 0x70, 0]]);
        assert!(calls(&e, SOURCE_TO_WORD).is_empty());
        e.call(0x0062_a190, &args![this, 0x11u32, 0x5000u32]);
        let filled = calls(&e, SOURCE_TO_WORD);
        assert_eq!(filled.len(), 1);
        assert_eq!(filled[0][0], 0x5000);
        assert_eq!(
            calls(&e, ADD_FROM_SOURCE),
            vec![vec![this, 0x11, filled[0][1]]]
        );
    }

    #[test]
    fn at_0062a2f0_deletes_only_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_a2f0, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0062_a2f0, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4000]]);
    }
}
