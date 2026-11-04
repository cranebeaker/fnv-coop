//! `(unplaced)/run_004b0260` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Unoptimized template code of `NiTArray<NiPointer<NiAVObject>,
//! NiTNewInterface<NiPointer<NiAVObject>>>` (the vtable `0101fbcc` the
//! constructor stores), in the shared `NiTArray` layout.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The vtable (RTTI `.?AV?$NiTArray@V?$NiPointer@VNiAVObject@@@@V?$NiTNewInterface@...`).
const NI_T_ARRAY_VTABLE: u32 = 0x0101_fbcc;
/// `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (engine map,
/// `005e0ba0`, `cdecl`, one word: the element count).
const ALLOCATE: u32 = 0x005e_0ba0;
/// `0060bef0` (`thiscall`, `RET 4`): resizes the array to the count in its
/// one word (`NiTArray::SetSize` by its use, not confirmed).
const SET_SIZE: u32 = 0x0060_bef0;
/// The function-local `static` empty element `SetAt` compares with
/// (`011c5f38`, 4 bytes) and the guard word whose bit 0 says it is made.
const EMPTY_ELEMENT: u32 = 0x011c_5f38;
const EMPTY_ELEMENT_GUARD: u32 = 0x011c_5f3c;
/// `00633c90` (`thiscall`, `RET 4`): constructs the empty element from the
/// pointer in its one word (0 here).
const ELEMENT_CONSTRUCT: u32 = 0x0063_3c90;
/// `_atexit` (`00ec658f`, `cdecl`) and the destructor of the empty element
/// it is given (`00fc90c0`).
const ATEXIT: u32 = 0x00ec_658f;
const EMPTY_ELEMENT_DESTRUCTOR: u32 = 0x00fc_90c0;
/// `00631820` (`thiscall`, `RET 4`, `teswaterlistener.cpp`): whether the
/// element at `this` differs from the one at its argument (true in `AL`).
const ELEMENT_DIFFERS: u32 = 0x0063_1820;
/// `NiPointer<..>::operator=` (engine map, `006e5cc0`, `RET 4`): assigns the
/// pointer at its argument to the one at `this`.
const ELEMENT_ASSIGN: u32 = 0x006e_5cc0;

// Translated from 004b0260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiAVObject>, ..>::NiTArray(max size, grow by)` (by
/// its vtable; `thiscall`, `RET 8`): stores the vtable, the low 16 bits of
/// both words as `m_usMaxSize` and `m_usGrowBy`, clears `m_usSize` and
/// `m_usESize`, and allocates `m_pBase` for a non-zero maximum size (null
/// otherwise). Returns `this`.
pub fn fn_004b0260(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), NI_T_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size as u16);
    e.set(this, NiTArray::m_usGrowBy, grow_by as u16);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let max = e.get(this, NiTArray::m_usMaxSize);
    if max != 0 {
        let max = e.get(this, NiTArray::m_usMaxSize);
        let base = e.call(ALLOCATE, &args![u32::from(max)]).u32();
        e.set(this, NiTArray::m_pBase, base);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

// Translated from 004b02d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiAVObject>, ..>::SetAtGrow` (by its shape;
/// `thiscall`, `RET 8`): an index at or past `m_usMaxSize` first grows the
/// array to `index + m_usGrowBy` (`0060bef0`); then `SetAt` (`004b0310`).
/// Returns `index`.
pub fn fn_004b02d0(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: Ptr) -> u32 {
    let max = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if index >= max {
        let grow_by = u32::from(e.get(this, NiTArray::m_usGrowBy));
        e.call(SET_SIZE, &args![this, grow_by.wrapping_add(index)]);
    }
    fn_004b0310(e, this, index, element);
    index
}

// Translated from 004b0310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b0310 - the compiler's exception frame (FS:[0] chain, handler 00ef07de) is not translated
/// `NiTArray<NiPointer<NiAVObject>, ..>::SetAt` (by its shape; `thiscall`,
/// `RET 8`; `element` is the address of the new `NiPointer`). On the first
/// call it makes its function-local empty element (`011c5f38`, guard bit 0
/// of `011c5f3c`, `00633c90(empty, 0)`, destructor `00fc90c0` given to
/// `_atexit`). Past the used size it raises `m_usSize` to `index + 1` and
/// counts the element in `m_usESize` when it is not empty; inside, a
/// non-empty element counts when the slot was empty and an empty one
/// uncounts a non-empty slot. Then the slot is assigned (`006e5cc0`).
pub fn fn_004b0310(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: Ptr) {
    let guard = e.global::<u32>(EMPTY_ELEMENT_GUARD);
    if guard & 1 == 0 {
        let guard = e.global::<u32>(EMPTY_ELEMENT_GUARD);
        e.set_global(EMPTY_ELEMENT_GUARD, guard | 1);
        e.call(ELEMENT_CONSTRUCT, &args![EMPTY_ELEMENT, 0u32]);
        e.call(ATEXIT, &args![EMPTY_ELEMENT_DESTRUCTOR]);
    }
    let size = u32::from(e.get(this, NiTArray::m_usSize));
    let slot = |e: &Engine| {
        e.get(this, NiTArray::m_pBase)
            .wrapping_add(index.wrapping_mul(4))
    };
    if index >= size {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if e.call(ELEMENT_DIFFERS, &args![element, EMPTY_ELEMENT])
            .bool()
        {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else if e
        .call(ELEMENT_DIFFERS, &args![element, EMPTY_ELEMENT])
        .bool()
    {
        let old = slot(e);
        if fn_004b0460(e, Ptr::new(old), Ptr::new(EMPTY_ELEMENT)) {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else {
        let old = slot(e);
        if e.call(ELEMENT_DIFFERS, &args![old, EMPTY_ELEMENT]).bool() {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_sub(1));
        }
    }
    let target = slot(e);
    e.call(ELEMENT_ASSIGN, &args![target, element]);
}

// Translated from 004b0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; an `NiPointer` `operator==` by its shape):
/// whether the word at `this` equals the word at `other`.
pub fn fn_004b0460(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.u32(this.addr()) == e.mem.u32(other.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004b_0260,
            fn_004b0260(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x004b_02d0, fn_004b02d0(Ptr<NiTArray>, u32, Ptr) -> u32),
        entry!(0x004b_0310, fn_004b0310(Ptr<NiTArray>, u32, Ptr)),
        entry!(0x004b_0460, fn_004b0460(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn constructor_takes_the_sizes_and_allocates_for_a_maximum() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| ret(0x5000 + a[0]));
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004b_0260, &args![this, 0x0001_0003u32, 0x0002_0005u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), NI_T_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x5003);
        assert_eq!(e.mem.u16(this + 8), 3);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 5);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![3]]);
        // The low 16 bits of 0x10000 are zero: no buffer.
        e.call_log = Some(vec![]);
        e.call(0x004b_0260, &args![this, 0x0001_0000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    /// An array of capacity 4 with `size` used slots and `esize` counted;
    /// a slot is empty when its word is 0, the empty element is word 0.
    fn array(size: u16, esize: u16) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(EMPTY_ELEMENT, 8);
        e.register(ELEMENT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(ATEXIT, |_, _| ret(0));
        e.register(ELEMENT_DIFFERS, |e, a| {
            ret((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        e.register(ELEMENT_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            ret(0)
        });
        let this = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, 4);
        e.mem.set_u16(this + 0xa, size);
        e.mem.set_u16(this + 0xc, esize);
        e.mem.set_u16(this + 0xe, 2);
        (e, this)
    }

    fn element(e: &mut Engine, value: u32) -> u32 {
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, value);
        p
    }

    #[test]
    fn set_at_makes_the_empty_element_once() {
        let (mut e, this) = array(0, 0);
        let item = element(&mut e, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004b_0310, &args![this, 0u32, item]);
        e.call(0x004b_0310, &args![this, 1u32, item]);
        assert_eq!(
            calls_to(&e, ELEMENT_CONSTRUCT),
            vec![vec![EMPTY_ELEMENT, 0]]
        );
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![EMPTY_ELEMENT_DESTRUCTOR]]);
        assert_eq!(e.mem.u32(EMPTY_ELEMENT_GUARD) & 1, 1);
    }

    #[test]
    fn set_at_past_the_size_raises_it_and_counts_non_empty_elements() {
        let (mut e, this) = array(1, 1);
        let item = element(&mut e, 0x77);
        e.call(0x004b_0310, &args![this, 2u32, item]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        let base = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(base + 8), 0x77);
        let empty = element(&mut e, 0);
        e.call(0x004b_0310, &args![this, 3u32, empty]);
        assert_eq!(e.mem.u16(this + 0xa), 4);
        assert_eq!(e.mem.u16(this + 0xc), 2);
    }

    #[test]
    fn set_at_inside_the_size_keeps_the_count_right() {
        let (mut e, this) = array(3, 1);
        let base = e.mem.u32(this + 4);
        e.mem.set_u32(base + 4, 0x55);
        let item = element(&mut e, 0x77);
        let empty = element(&mut e, 0);
        // Non-empty into an empty slot: counted.
        e.call(0x004b_0310, &args![this, 0u32, item]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        // Non-empty over a non-empty slot: unchanged.
        e.call(0x004b_0310, &args![this, 1u32, item]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        assert_eq!(e.mem.u32(base + 4), 0x77);
        // Empty over a non-empty slot: uncounted; over an empty one: unchanged.
        e.call(0x004b_0310, &args![this, 1u32, empty]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        e.call(0x004b_0310, &args![this, 2u32, empty]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 0xa), 3);
    }

    #[test]
    fn set_at_grow_grows_past_the_maximum() {
        let (mut e, this) = array(0, 0);
        let item = element(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_02d0, &args![this, 3u32, item]).u32(), 3);
        assert!(calls_to(&e, SET_SIZE).is_empty());
        assert_eq!(e.call(0x004b_02d0, &args![this, 4u32, item]).u32(), 4);
        assert_eq!(calls_to(&e, SET_SIZE), vec![vec![this, 6]]);
        assert_eq!(e.mem.u16(this + 0xa), 5);
    }

    #[test]
    fn equals_compares_the_first_words() {
        let mut e = Engine::new();
        let a = element(&mut e, 5);
        let b = element(&mut e, 5);
        let c = element(&mut e, 6);
        assert!(e.call(0x004b_0460, &args![a, b]).bool());
        assert!(!e.call(0x004b_0460, &args![a, c]).bool());
    }
}
