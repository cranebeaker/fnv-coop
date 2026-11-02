//! `fallout shared/tempeffects/run_0068ab60` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

// Translated from 0068ab60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0068ab80`, which stores the vtable `01069d7c` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0068ab80`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_0068ab60(e: &mut Engine, this: Ptr) {
    e.call(0x0068_ab80, &args![this]);
}

/// The vtable of `NiTArray<NiAVObject *, NiTMallocInterface<NiAVObject *>>`
/// (RTTI `.?AV?$NiTArray@PAVNiAVObject@@V?$NiTMallocInterface@...`).
const NI_T_ARRAY_VTABLE: u32 = 0x0106_9d7c;
/// The vtable of `NiTPrimitiveArray<NiAVObject *>` (RTTI `.?AV?$NiTPrimitiveArray@PAVNiAVObject@@@@`).
const NI_T_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0106_9d84;
/// cdecl (count): allocates the element block (`count * 4` bytes through `NiAlloc`).
const ALLOCATE_ELEMENTS: u32 = 0x0096_afc0;
/// cdecl (block): frees the element block (through `NiFree`).
const FREE_ELEMENTS: u32 = 0x004e_de70;

// Translated from 0068ab80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiAVObject *, NiTMallocInterface<NiAVObject *>>::~NiTArray`
/// (by the vtable it sets; unnamed in the engine map): sets the vtable and
/// frees `m_pBase` (+4).
pub fn fn_0068ab80(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), NI_T_ARRAY_VTABLE);
    let elements = e.get(this, NiTArray::m_pBase);
    e.call(FREE_ELEMENTS, &args![elements]);
}

// Translated from 0068abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiAVObject *>::NiTPrimitiveArray` (by the vtable it
/// sets; unnamed in the engine map), thiscall `(max size, grow by)`:
/// constructs the `NiTArray` base (`0068ac40`) and sets the
/// `NiTPrimitiveArray` vtable. Answers `this`.
pub fn fn_0068abb0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_0068ac40(e, this, max_size, grow_by);
    e.mem.set_u32(this.addr(), NI_T_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 0068ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiAVObject *, NiTMallocInterface<NiAVObject *>>::NiTArray`
/// (by the vtable it sets; unnamed in the engine map), thiscall `(max
/// size, grow by)` (the low 16 bits of each word): sets the vtable, the
/// capacity and growth, clears the two counts and allocates the elements
/// (`m_pBase` is 0 for a capacity of 0). Answers `this`.
pub fn fn_0068ac40(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), NI_T_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let max_size = e.get(this, NiTArray::m_usMaxSize);
    let elements = if max_size > 0 {
        e.call(ALLOCATE_ELEMENTS, &args![u32::from(max_size)]).u32()
    } else {
        0
    };
    e.set(this, NiTArray::m_pBase, elements);
    this
}

/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 0068abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a scalar deleting destructor (`thiscall`, `RET 4`): the body
/// [`fn_0068ab80`], then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_0068abe0(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    fn_0068ab80(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0068ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a scalar deleting destructor (`thiscall`, `RET 4`): the body
/// [`fn_0068ab60`], then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_0068ac10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0068ab60(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0068_abe0,
            fn_0068abe0(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0068_ac10, fn_0068ac10(Ptr, u32) -> Ptr),
        entry!(0x0068_ab60, fn_0068ab60(Ptr)),
        entry!(0x0068ab80, fn_0068ab80(Ptr<NiTArray>)),
        entry!(
            0x0068abb0,
            fn_0068abb0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x0068ac40,
            fn_0068ac40(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
    ]
}

#[cfg(test)]
mod array_tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn destructor_frees_the_elements() {
        let mut e = Engine::new();
        e.register(FREE_ELEMENTS, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0068_ab80, &args![array]);
        assert_eq!(e.mem.u32(array), NI_T_ARRAY_VTABLE);
        assert_eq!(calls(&e, FREE_ELEMENTS), vec![vec![0x7000]]);
    }

    #[test]
    fn constructors_set_the_counts_and_allocate() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, a| Ret {
            eax: 0x8000 + a[0],
            ..Ret::default()
        });
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 0xa, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0068_abb0, &args![array, 0x1_0003u32, 0x2_0004u32])
                .u32(),
            array
        );
        assert_eq!(e.mem.u32(array), NI_T_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(array + 8), 3);
        assert_eq!(e.mem.u16(array + 0xe), 4);
        assert_eq!(e.mem.u32(array + 0xa), 0);
        assert_eq!(e.mem.u32(array + 4), 0x8003);
        assert_eq!(calls(&e, ALLOCATE_ELEMENTS), vec![vec![3]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_ac40, &args![array, 0u32, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), NI_T_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert!(calls(&e, ALLOCATE_ELEMENTS).is_empty());
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0068ab60_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0068_ab80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_ab60, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0068_ab80), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    #[test]
    fn array_deleting_destructors_free_on_flag_one() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(FREE_ELEMENTS, |_, _| Ret::default());
        for address in [0x0068_abe0u32, 0x0068_ac10] {
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
            e.call(address, &args![this, 3u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), NI_T_ARRAY_VTABLE);
            let frees: Vec<_> = log.iter().filter(|(a, _)| *a == DEALLOCATE).collect();
            assert_eq!(frees, vec![&(DEALLOCATE, vec![this])]);
        }
    }
}
