//! `fallout shared/run_00644400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Unoptimized code (frame pointer, every value through the stack) for an
//! array of 16-byte elements in the `BSSimpleArray` layout (vtable,
//! `pBuffer` +4, `iSize` +8, `iReservedSize` +0xc; the vtable slot `+0x04`
//! allocates the buffer). None of these functions has an exception frame
//! or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The element size.
const ELEMENT_SIZE: u32 = 0x10;
/// The array's vtable slot `+0x04`: allocates a buffer for the count in its
/// one word and returns it.
const ALLOCATE_SLOT: u32 = 0x04;
/// `006e6da0` (`cdecl`, two words): returns its second word (the
/// placement `operator new(size, where)` shape).
const PLACEMENT_NEW: u32 = 0x006e_6da0;

// Translated from 00644400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`; a `BSSimpleArray` constructor body by its
/// shape): clears `pBuffer`, `iSize` and `iReservedSize`; the capacity is
/// `reserve`, raised to `size` when that is larger (unsigned). A non-zero
/// capacity gets a buffer (vtable `+0x04`) and becomes `iReservedSize`; a
/// non-zero `size` has its elements constructed (`00644490`) and becomes
/// `iSize`.
pub fn fn_00644400(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if size > reserve { size } else { reserve };
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), ALLOCATE_SLOT, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_00644490(e, this, buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 00644490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`; `this` is not read): "constructs" `count`
/// 16-byte elements from `buffer` on, each through `006e6da0(16, element)`
/// (a placement new whose element has no constructor).
pub fn fn_00644490(e: &mut Engine, _this: Ptr<BSSimpleArray>, buffer: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let element = (index << 4).wrapping_add(buffer);
        e.call(PLACEMENT_NEW, &args![ELEMENT_SIZE, element]);
        index = index.wrapping_add(1);
    }
}

// Translated from 006444e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0x10`): stores its four words at `+0x0`,
/// `+0x4`, `+0x8` and `+0xc` (a 16-byte element's constructor by its
/// shape) and returns `this`.
pub fn fn_006444e0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, first);
    e.mem.set_u32(base.wrapping_add(4), second);
    e.mem.set_u32(base.wrapping_add(8), third);
    e.mem.set_u32(base.wrapping_add(0xc), fourth);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_4400, fn_00644400(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0064_4490, fn_00644490(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0064_44e0, fn_006444e0(Ptr, u32, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE: u32 = 0x00ee_0004;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn fixture() -> (Engine, u32) {
        let mut e = Engine::new();
        e.put_vtable(0x0300_0000, &[0, ALLOCATE]);
        e.register(ALLOCATE, |_, a| Ret {
            eax: 0x5000 + a[1],
            ..Ret::default()
        });
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 4, 0xdead);
        e.mem.set_u32(this + 8, 0xdead);
        e.mem.set_u32(this + 12, 0xdead);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn constructor_reserves_the_larger_count_and_constructs_the_size() {
        let (mut e, this) = fixture();
        e.call(0x0064_4400, &args![this, 2u32, 3u32]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![this, 3]]);
        assert_eq!(e.mem.u32(this + 4), 0x5003);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 12), 3);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x10, 0x5003], vec![0x10, 0x5013], vec![0x10, 0x5023]]
        );
    }

    #[test]
    fn constructor_reserves_without_a_size() {
        let (mut e, this) = fixture();
        e.call(0x0064_4400, &args![this, 4u32, 0u32]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![this, 4]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 12), 4);
        assert!(calls_to(&e, PLACEMENT_NEW).is_empty());
    }

    #[test]
    fn constructor_with_nothing_allocates_nothing() {
        let (mut e, this) = fixture();
        e.call(0x0064_4400, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.bytes(this + 4, 12), vec![0; 12]);
    }

    #[test]
    fn construct_items_runs_once_per_element() {
        let (mut e, this) = fixture();
        e.call(0x0064_4490, &args![this, 0x8000u32, 2u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x10, 0x8000], vec![0x10, 0x8010]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0064_4490, &args![this, 0x8000u32, 0u32]);
        assert!(calls_to(&e, PLACEMENT_NEW).is_empty());
    }

    #[test]
    fn element_constructor_stores_four_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x0064_44e0, &args![this, 1u32, 2u32, 3u32, 4u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 12), 4);
    }
}
