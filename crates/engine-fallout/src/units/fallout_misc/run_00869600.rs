//! `fallout/misc/run_00869600` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

// Translated from 00869a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869a40`, which stores the vtable `010827f0` at
/// `+0` and frees the block held at `+4` through `008696c0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00869a40`'s disassembly ends with the stack cleanup of that cdecl
/// call (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00869a20(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9a40, &args![this]);
}

// Translated from 00869b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869b90`, which stores the vtable `01082800` at
/// `+0` and frees the block held at `+4` through `008696c0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00869b90`'s disassembly ends with the stack cleanup of that cdecl
/// call (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00869b70(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9b90, &args![this]);
}

// Translated from 00869cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869ce0`, which stores the vtable `01082810` at
/// `+0` and frees the block held at `+4` through `008696c0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00869ce0`'s disassembly ends with the stack cleanup of that cdecl
/// call (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00869cc0(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9ce0, &args![this]);
}

// Translated from 00869e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869e30`, which stores the vtable `01082820` at
/// `+0` and destroys the array held at `+4` through `004b0220` (the vector deleting
/// destructor), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00869e30`'s disassembly ends
/// with the stack cleanup of that call (`ADD ESP,4`); `004b0220` stores its own result only
/// in a frame slot, so no value is returned (Ghidra types both void).
pub fn fn_00869e10(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9e30, &args![this]);
}

/// The five-constructor group: each runs a base body on `this` with its two stack words, then
/// stores a vtable (all four RTTI classes are `NiTObjectArray<...>` instances), except
/// `00869f00`, which stores its vtable first and then calls the shared body `006b3eb0`.
const BASE_BODY_00869AA0: u32 = 0x0086_9aa0;
const BASE_BODY_00869BF0: u32 = 0x0086_9bf0;
const BASE_BODY_00869D40: u32 = 0x0086_9d40;
const BASE_BODY_00869E90: u32 = 0x0086_9e90;
const BASE_BODY_006B3EB0: u32 = 0x006b_3eb0;
const VTABLE_008699C0: u32 = 0x0108_27e8;
const VTABLE_00869B10: u32 = 0x0108_27f8;
const VTABLE_00869C60: u32 = 0x0108_2808;
const VTABLE_00869DB0: u32 = 0x0108_2818;
const VTABLE_00869F00: u32 = 0x0106_ba6c;

// Translated from 008699c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869aa0(this, first, second)`, then stores the vtable
/// `010827e8` (RTTI class `NiTObjectArray<TESObjectREFR*>`). Returns `this`.
pub fn fn_008699c0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869AA0, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_008699C0);
    this
}

// Translated from 00869b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869bf0(this, first, second)`, then stores the vtable
/// `010827f8` (RTTI class `NiTObjectArray<BipedAnim*>`). Returns `this`.
pub fn fn_00869b10(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869BF0, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_00869B10);
    this
}

// Translated from 00869c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869d40(this, first, second)`, then stores the vtable
/// `01082808` (RTTI class `NiTObjectArray<Animation*>`). Returns `this`.
pub fn fn_00869c60(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869D40, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_00869C60);
    this
}

// Translated from 00869db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869e90(this, first, second)`, then stores the vtable
/// `01082818` (RTTI class `NiTObjectArray<NiPointer<BSTempEffect>>`). Returns `this`.
pub fn fn_00869db0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869E90, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_00869DB0);
    this
}

// Translated from 00869f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0106ba6c` (RTTI class
/// `BSSimpleArray<NavMesh*,...>`), then runs `006b3eb0(this, first, second)`. Returns `this`.
pub fn fn_00869f00(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_00869F00);
    e.call(BASE_BODY_006B3EB0, &args![this, first, second]);
    this
}

/// The `NiTArray<T, NiTNewInterface<T>>` vtables of this unit (RTTI
/// `NiTArray<TESObjectREFR*, ..>`, `NiTArray<BipedAnim*, ..>`,
/// `NiTArray<Animation*, ..>`, `NiTArray<NiPointer<BSTempEffect>, ..>`).
const ARRAY_VTABLE_REFERENCES: u32 = 0x0108_27f0;
const ARRAY_VTABLE_BIPED_ANIMS: u32 = 0x0108_2800;
const ARRAY_VTABLE_ANIMATIONS: u32 = 0x0108_2810;
const ARRAY_VTABLE_TEMP_EFFECTS: u32 = 0x0108_2820;
/// `00401000` / `00401030` (`cdecl`, one word): allocate / free a block.
const MEMORY_ALLOCATE: u32 = 0x0040_1000;
const MEMORY_FREE: u32 = 0x0040_1030;
/// `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (engine map,
/// `005e0ba0`, `cdecl`, one word: the count; the `NiPointer` arrays share it).
const POINTER_ARRAY_ALLOCATE: u32 = 0x005e_0ba0;
/// `NiPointer<BSTempEffect>::_vector_deleting_destructor_` (engine map,
/// `004b0220`, `cdecl`, one word: the array).
const POINTER_ARRAY_DELETE: u32 = 0x004b_0220;
/// `0060bef0` (`thiscall`, `RET 4`): resizes an `NiPointer` array to the
/// count in its one word (`NiTArray::SetSize` by its use, not confirmed).
const POINTER_ARRAY_SET_SIZE: u32 = 0x0060_bef0;
/// `0096ae90` (`thiscall`, `RET 8`): stores an element (index, element) of
/// a plain pointer array (`NiTArray::SetAt` by its use, not confirmed).
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The function-local `static` empty element `00869870` compares with
/// (`011de95c`, 4 bytes) and the guard word whose bit 0 says it is made.
const EMPTY_ELEMENT: u32 = 0x011d_e95c;
const EMPTY_ELEMENT_GUARD: u32 = 0x011d_e960;
/// `00633c90` (`thiscall`, `RET 4`): constructs an `NiPointer` from the
/// pointer in its one word.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `_atexit` (`00ec658f`, `cdecl`) and the empty element's destructor
/// (`00fd8df0`) it is given.
const ATEXIT: u32 = 0x00ec_658f;
const EMPTY_ELEMENT_DESTRUCTOR: u32 = 0x00fd_8df0;
/// `00631820` (`thiscall`, `RET 4`): whether the pointer at `this` differs
/// from the one at its argument (true in `AL`).
const POINTER_DIFFERS: u32 = 0x0063_1820;
/// `004b0460` (`thiscall`, `RET 4`): whether the pointer at `this` equals
/// the one at its argument.
const POINTER_EQUALS: u32 = 0x004b_0460;
/// `NiPointer<..>::operator=` (engine map, `006e5cc0`, `RET 4`): assigns the
/// pointer at its argument to the one at `this`.
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `009ad610` (`thiscall`, plain `RET`, `explosion.cpp`): the object
/// `0086a050` asks for its float.
const OBJECT_OF: u32 = 0x009a_d610;

// Translated from 00869600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; unoptimized code): for the word at `+0xc` above
/// 0x800 it returns that word minus 0x400, otherwise the word halved.
pub fn fn_00869600(e: &mut Engine, this: Ptr) -> u32 {
    let count = e.mem.u32(this.addr().wrapping_add(0xc));
    if count > 0x800 {
        count.wrapping_sub(0x400)
    } else {
        count >> 1
    }
}

// Translated from 00869640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetAtGrow` of a plain pointer array (by its shape;
/// `thiscall`, `RET 8`): an index at or past `m_usMaxSize` first grows the
/// array to `index + m_usGrowBy` (`008696e0`); then `0096ae90(this, index,
/// element)`. Returns `index`.
pub fn fn_00869640(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: u32) -> u32 {
    let max = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if index >= max {
        let grow_by = u32::from(e.get(this, NiTArray::m_usGrowBy));
        fn_008696e0(e, this, grow_by.wrapping_add(index));
    }
    e.call(ARRAY_SET_AT, &args![this, index, element]);
    index
}

// Translated from 00869680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetAtGrow` of an `NiPointer` array (by its shape;
/// `thiscall`, `RET 8`; `element` is the address of the new pointer): an
/// index at or past `m_usMaxSize` first grows the array to `index +
/// m_usGrowBy` (`0060bef0`); then `SetAt` (`00869870`). Returns `index`.
pub fn fn_00869680(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: Ptr) -> u32 {
    let max = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if index >= max {
        let grow_by = u32::from(e.get(this, NiTArray::m_usGrowBy));
        e.call(
            POINTER_ARRAY_SET_SIZE,
            &args![this, grow_by.wrapping_add(index)],
        );
    }
    fn_00869870(e, this, index, element);
    index
}

// Translated from 008696c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word; `NiTNewInterface::Deallocate` by its shape):
/// frees `block` through `00401030`.
pub fn fn_008696c0(e: &mut Engine, block: u32) {
    e.call(MEMORY_FREE, &args![block]);
}

// Translated from 008696e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetSize` of a plain pointer array (by its shape; `thiscall`,
/// `RET 4`; unoptimized code with 16-bit counters). Nothing happens when
/// `new_max` is the current `m_usMaxSize`. A size below `m_usSize` clears
/// (and uncounts from `m_usESize`) the non-null elements from `new_max` on
/// and lowers `m_usSize`. `m_usMaxSize` becomes `new_max`; a non-zero one
/// gets a new buffer (`00869840`) holding the old elements below
/// `m_usSize` and nulls up to `m_usMaxSize`, a zero one a null buffer. The
/// old buffer is freed (`008696c0`).
pub fn fn_008696e0(e: &mut Engine, this: Ptr<NiTArray>, new_max: u32) {
    if new_max == u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        return;
    }
    let slot = |e: &Engine, index: u16| {
        e.get(this, NiTArray::m_pBase)
            .wrapping_add(u32::from(index).wrapping_mul(4))
    };
    if new_max < u32::from(e.get(this, NiTArray::m_usSize)) {
        let mut index = new_max as u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let at = slot(e, index);
            if e.mem.u32(at) != 0 {
                e.mem.set_u32(at, 0);
                let count = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, count.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, new_max as u16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, new_max as u16);
    if new_max != 0 {
        let max = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let base = fn_00869840(e, max);
        e.set(this, NiTArray::m_pBase, base);
        let mut index = 0u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let value = e
                .mem
                .u32(old.wrapping_add(u32::from(index).wrapping_mul(4)));
            let at = slot(e, index);
            e.mem.set_u32(at, value);
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        while index < e.get(this, NiTArray::m_usMaxSize) {
            let at = slot(e, index);
            e.mem.set_u32(at, 0);
            index = index.wrapping_add(1);
        }
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    fn_008696c0(e, old);
}

// Translated from 00869840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word; `NiTNewInterface::Allocate` by its shape):
/// allocates `count` pointers through `00401000`; a byte count that
/// overflows 32 bits asks for `0xffffffff`.
pub fn fn_00869840(e: &mut Engine, count: u32) -> u32 {
    let bytes = u64::from(count) * 4;
    let size = if bytes > u64::from(u32::MAX) {
        u32::MAX
    } else {
        bytes as u32
    };
    e.call(MEMORY_ALLOCATE, &args![size]).u32()
}

// Translated from 00869870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00869870 - the compiler's exception frame (FS:[0] chain, handler 00f13dbe) is not translated
/// `NiTArray::SetAt` of an `NiPointer` array (by its shape; `thiscall`,
/// `RET 8`; `element` is the address of the new pointer). On the first call
/// it makes its function-local empty pointer (`011de95c`, guard bit 0 of
/// `011de960`, `00633c90(empty, 0)`, destructor `00fd8df0` given to
/// `_atexit`). Past the used size it raises `m_usSize` to `index + 1` and
/// counts a non-empty element in `m_usESize`; inside, a non-empty element
/// counts when the slot was empty and an empty one uncounts a non-empty
/// slot. Then the slot is assigned (`006e5cc0`).
pub fn fn_00869870(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: Ptr) {
    if e.global::<u32>(EMPTY_ELEMENT_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(EMPTY_ELEMENT_GUARD);
        e.set_global(EMPTY_ELEMENT_GUARD, guard | 1);
        e.call(POINTER_CONSTRUCT, &args![EMPTY_ELEMENT, 0u32]);
        e.call(ATEXIT, &args![EMPTY_ELEMENT_DESTRUCTOR]);
    }
    let slot = |e: &Engine| {
        e.get(this, NiTArray::m_pBase)
            .wrapping_add(index.wrapping_mul(4))
    };
    if index >= u32::from(e.get(this, NiTArray::m_usSize)) {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if e.call(POINTER_DIFFERS, &args![element, EMPTY_ELEMENT])
            .bool()
        {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else if e
        .call(POINTER_DIFFERS, &args![element, EMPTY_ELEMENT])
        .bool()
    {
        let old = slot(e);
        if e.call(POINTER_EQUALS, &args![old, EMPTY_ELEMENT]).bool() {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else {
        let old = slot(e);
        if e.call(POINTER_DIFFERS, &args![old, EMPTY_ELEMENT]).bool() {
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_sub(1));
        }
    }
    let target = slot(e);
    e.call(POINTER_ASSIGN, &args![target, element]);
}

/// The shared constructor body of this unit's arrays: the vtable, the low
/// 16 bits of `max_size` and `grow_by`, `m_usSize`/`m_usESize` cleared, and a
/// buffer from `allocate` for a non-zero maximum (null otherwise).
fn construct_array(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
    vtable: u32,
    allocate: fn(&mut Engine, u32) -> u32,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTArray::m_usMaxSize, max_size as u16);
    e.set(this, NiTArray::m_usGrowBy, grow_by as u16);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if e.get(this, NiTArray::m_usMaxSize) != 0 {
        let max = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let base = allocate(e, max);
        e.set(this, NiTArray::m_pBase, base);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

/// `005e0ba0` by address.
fn allocate_pointer_array(e: &mut Engine, count: u32) -> u32 {
    e.call(POINTER_ARRAY_ALLOCATE, &args![count]).u32()
}

/// The shared destructor body: the vtable, then the buffer freed.
fn destruct_array(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    let base = e.mem.u32(this.addr().wrapping_add(4));
    fn_008696c0(e, base);
}

// Translated from 00869a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<TESObjectREFR*, ..>::~NiTArray` (by its vtable `010827f0`):
/// stores the vtable and frees `m_pBase` (`008696c0`).
pub fn fn_00869a40(e: &mut Engine, this: Ptr) {
    destruct_array(e, this, ARRAY_VTABLE_REFERENCES);
}

// Translated from 00869aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<TESObjectREFR*, ..>::NiTArray(max size, grow by)` (by its
/// vtable; `RET 8`): the shared constructor body with `00869840`.
/// Returns `this`.
pub fn fn_00869aa0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        max_size,
        grow_by,
        ARRAY_VTABLE_REFERENCES,
        fn_00869840,
    )
}

// Translated from 00869b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BipedAnim*, ..>::~NiTArray` (by its vtable `01082800`): stores
/// the vtable and frees `m_pBase` (`008696c0`).
pub fn fn_00869b90(e: &mut Engine, this: Ptr) {
    destruct_array(e, this, ARRAY_VTABLE_BIPED_ANIMS);
}

// Translated from 00869bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BipedAnim*, ..>::NiTArray(max size, grow by)` (by its vtable;
/// `RET 8`): the shared constructor body with `00869840`. Returns `this`.
pub fn fn_00869bf0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        max_size,
        grow_by,
        ARRAY_VTABLE_BIPED_ANIMS,
        fn_00869840,
    )
}

// Translated from 00869ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<Animation*, ..>::~NiTArray` (by its vtable `01082810`): stores
/// the vtable and frees `m_pBase` (`008696c0`).
pub fn fn_00869ce0(e: &mut Engine, this: Ptr) {
    destruct_array(e, this, ARRAY_VTABLE_ANIMATIONS);
}

// Translated from 00869d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<Animation*, ..>::NiTArray(max size, grow by)` (by its vtable;
/// `RET 8`): the shared constructor body with `00869840`. Returns `this`.
pub fn fn_00869d40(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        max_size,
        grow_by,
        ARRAY_VTABLE_ANIMATIONS,
        fn_00869840,
    )
}

// Translated from 00869e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<BSTempEffect>, ..>::~NiTArray` (by its vtable
/// `01082820`): stores the vtable and deletes `m_pBase` through the
/// `NiPointer` vector deleting destructor `004b0220`.
pub fn fn_00869e30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_TEMP_EFFECTS);
    let base = e.mem.u32(this.addr().wrapping_add(4));
    e.call(POINTER_ARRAY_DELETE, &args![base]);
}

// Translated from 00869e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<BSTempEffect>, ..>::NiTArray(max size, grow by)` (by
/// its vtable; `RET 8`): the shared constructor body with `005e0ba0`.
/// Returns `this`.
pub fn fn_00869e90(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        max_size,
        grow_by,
        ARRAY_VTABLE_TEMP_EFFECTS,
        allocate_pointer_array,
    )
}

// Translated from 0086a050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word, the `this` of `009ad610`): the float of the
/// object `009ad610` returns, when there is one and its vtable slot `+0x100`
/// answers true: its slot `+0x420` (the x87 `ST0` it returns, unchanged);
/// otherwise 1.0 (`FLD1`).
pub fn fn_0086a050(e: &mut Engine, source: u32) -> f64 {
    let object = e.call(OBJECT_OF, &args![source]).u32();
    if object != 0 && e.vcall(object, 0x100, &args![]).bool() {
        return e.vcall(object, 0x420, &args![]).f64();
    }
    1.0
}

// Translated from 0086a460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; unoptimized code): the byte at `+3` (in `AL`; the
/// rest of `EAX` keeps the upper bytes of `this`, which callers do not read
/// as part of a `bool`/byte result).
pub fn fn_0086a460(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(3))
}

/// The `Main` singleton (`011dea0c`, `Main*`; `0086a0a0` reads it at every
/// use).
const MAIN: u32 = 0x011d_ea0c;
/// Import slots of `DefWindowProcA` (4 words) and `ShowCursor` (1 word).
const DEF_WINDOW_PROC: u32 = 0x00fd_f2e0;
const SHOW_CURSOR: u32 = 0x00fd_f2ac;
/// The flag `0086a0a0` sets on `WM_NCACTIVATE` with a zero `wParam`.
const NC_DEACTIVATED: u32 = 0x011d_eef4;
/// Whether the window procedure has shown the cursor (`ShowCursor(1)`) over
/// the non-client area.
const CURSOR_SHOWN: u32 = 0x011a_3014;
/// `Main::SetActive` (engine map, `00871c90`, `RET 4`).
const MAIN_SET_ACTIVE: u32 = 0x0087_1c90;
/// `00877720` (`thiscall` on `Main`, plain `RET`, `main.cpp`): a pointer
/// the window procedure tests and hands to `00a23c00`.
const MAIN_GET_00877720: u32 = 0x0087_7720;
/// `00a23c00` (`thiscall`, `RET 4`).
const ACTIVATE_00A23C00: u32 = 0x00a2_3c00;
/// `004f1540` (`thiscall` on `Main`, plain `RET`): tested on deactivation.
const MAIN_TEST_004F1540: u32 = 0x004f_1540;
/// `0070ec00` (plain `RET`, no stack words): tested on deactivation.
const TEST_0070EC00: u32 = 0x0070_ec00;
/// `Tile::GetMenuByClass` (engine map, `00a09030`, `cdecl`, one word).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Interface::QueueMenuCreate` (engine map, `00709470`, `cdecl`, 6 words).
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `00559450` (`thiscall`, plain `RET`) on the object at `011deb30`.
const TEST_00559450: u32 = 0x0055_9450;
const OBJECT_011DEB30: u32 = 0x011d_eb30;
/// `00871d10` (`thiscall` on `Main`, `RET 8`, `main.cpp`): the mouse
/// position (x, y) of `WM_MOUSEMOVE`; true when it handled it.
const MAIN_MOUSE_MOVE: u32 = 0x0087_1d10;
/// `004bc3f0` (plain `RET`, no stack words): tested on
/// `WM_WINDOWPOSCHANGED`.
const TEST_004BC3F0: u32 = 0x004b_c3f0;
/// `004dc6e0` (`cdecl`, two words, `renderer.cpp`): given the new window
/// position (`WINDOWPOS::x`, `y`).
const WINDOW_MOVED: u32 = 0x004d_c6e0;
/// `00446e10` (plain `RET`, no stack words): tested when `004bc3f0` is 0.
const TEST_00446E10: u32 = 0x0044_6e10;
/// `005b6cb0` (`thiscall` on `Main`, plain `RET`): run on `WM_DESTROY`.
const MAIN_ON_DESTROY: u32 = 0x005b_6cb0;
/// `004dc1e0` (plain `RET`, `renderer.cpp`): the renderer's window handle,
/// compared on `WM_ERASEBKGND`.
const RENDERER_WINDOW: u32 = 0x004d_c1e0;
/// `004b7210` / `004b71d0` (plain `RET`, `globalfunc.cpp`): run and tested
/// on `WM_SYSCOMMAND`.
const SYSCOMMAND_004B7210: u32 = 0x004b_7210;
const SYSCOMMAND_TEST_004B71D0: u32 = 0x004b_71d0;

/// `DefWindowProcA` with the four words.
fn default_window_proc(e: &mut Engine, window: u32, message: u32, wparam: u32, lparam: u32) -> u32 {
    e.call(DEF_WINDOW_PROC, &args![window, message, wparam, lparam])
        .u32()
}

// Translated from 0086a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The main window's procedure (unnamed; `stdcall`, `RET 0x10`;
/// unoptimized code; the `switch` on the message is translated from its
/// compare chain and its jump table at `0086a3fc` with the byte map at
/// `0086a414`). Message numbers are the Win32 ones:
/// - `WM_DESTROY` (2): `005b6cb0` on `Main` (when set), then the default.
/// - `WM_SIZE` (5) and `WM_SYSKEYUP` (0x105): 0.
/// - `WM_ACTIVATE` (6): for 1 or 2 in the low word of `wParam` and 0 in its
///   high word, with `Main` set: `Main::SetActive(1)` and
///   `00a23c00(00877720(Main), 0)`. For 0 with `Main` set and its byte `+3`
///   (`0086a460`) set: when `004f1540(Main)` holds, `0070ec00()` does not,
///   and neither menu class 0x3f5 nor 0x3ef is open, the menu of class 6 is
///   queued (`QueueMenuCreate(6, 0, 0, 0, 1, 0)`); then, with `Main` still
///   set and `00559450(011deb30)` 0, `Main::SetActive(0)`. Returns 0.
/// - `WM_ERASEBKGND` (0x14): 1 when the window is the renderer's
///   (`004dc1e0`), else the default.
/// - `WM_WINDOWPOSCHANGED` (0x47): with `004bc3f0()` non-zero, `004dc6e0`
///   gets the new position and the default follows; otherwise 0 unless
///   `00446e10()` holds (then the default).
/// - `WM_NCHITTEST` (0x84): the default's answer; with `Main` and
///   `00877720(Main)` set, outside the client area (`!= HTCLIENT`) the
///   cursor is shown once (`ShowCursor(1)`, flag `011a3014`), inside it is
///   hidden again (`ShowCursor(0)` until the count is not positive).
/// - `WM_NCACTIVATE` (0x86): a zero `wParam` sets the byte `011deef4`;
///   then the default.
/// - `WM_SYSCOMMAND` (0x112): after `004b7210()`, when `004b71d0()` holds,
///   `SC_SCREENSAVE` (0xf140) and `SC_MONITORPOWER` (0xf170) answer 1;
///   otherwise the default.
/// - `WM_MOUSEMOVE` (0x200): 0 when `00871d10(Main, x, y)` handles it
///   (`Main` is not checked here), else the default.
/// - `WM_IME_SETCONTEXT` (0x281): the default with `lParam` 0.
/// - Every other message: `DefWindowProcA`.
pub fn fn_0086a0a0(e: &mut Engine, window: u32, message: u32, wparam: u32, lparam: u32) -> u32 {
    match message {
        0x281 => default_window_proc(e, window, message, wparam, 0),
        0x6 => {
            let state = wparam & 0xffff;
            if state == 0 {
                deactivate(e);
            } else if state <= 2 && (wparam >> 16) & 0xffff == 0 && e.global::<u32>(MAIN) != 0 {
                let main = e.global::<u32>(MAIN);
                e.call(MAIN_SET_ACTIVE, &args![main, 1u32]);
                let main = e.global::<u32>(MAIN);
                let target = e.call(MAIN_GET_00877720, &args![main]).u32();
                e.call(ACTIVATE_00A23C00, &args![target, 0u32]);
            }
            0
        }
        0x86 => {
            if wparam == 0 {
                e.set_global(NC_DEACTIVATED, 1u8);
            }
            default_window_proc(e, window, message, wparam, lparam)
        }
        0x84 => {
            let hit = default_window_proc(e, window, message, wparam, lparam);
            if e.global::<u32>(MAIN) != 0 {
                let main = e.global::<u32>(MAIN);
                if e.call(MAIN_GET_00877720, &args![main]).u32() != 0 {
                    if hit != 1 {
                        if e.global::<u8>(CURSOR_SHOWN) == 0 {
                            e.call(SHOW_CURSOR, &args![1u32]);
                            e.set_global(CURSOR_SHOWN, 1u8);
                        }
                    } else if e.global::<u8>(CURSOR_SHOWN) != 0 {
                        while e.call(SHOW_CURSOR, &args![0u32]).i32() > 0 {}
                        e.set_global(CURSOR_SHOWN, 0u8);
                    }
                }
            }
            hit
        }
        0x200 => {
            let main = e.global::<u32>(MAIN);
            let handled = e
                .call(
                    MAIN_MOUSE_MOVE,
                    &args![main, lparam & 0xffff, (lparam >> 16) & 0xffff],
                )
                .bool();
            if handled {
                0
            } else {
                default_window_proc(e, window, message, wparam, lparam)
            }
        }
        0x5 | 0x105 => 0,
        0x47 => {
            if e.call(TEST_004BC3F0, &args![]).u32() != 0 {
                let x = e.mem.u32(lparam.wrapping_add(8));
                let y = e.mem.u32(lparam.wrapping_add(0xc));
                e.call(WINDOW_MOVED, &args![x, y]);
            } else if !e.call(TEST_00446E10, &args![]).bool() {
                return 0;
            }
            default_window_proc(e, window, message, wparam, lparam)
        }
        0x2 => {
            if e.global::<u32>(MAIN) != 0 {
                let main = e.global::<u32>(MAIN);
                e.call(MAIN_ON_DESTROY, &args![main]);
            }
            default_window_proc(e, window, message, wparam, lparam)
        }
        0x14 => {
            if e.call(RENDERER_WINDOW, &args![]).u32() == window {
                return 1;
            }
            default_window_proc(e, window, message, wparam, lparam)
        }
        0x112 => {
            e.call(SYSCOMMAND_004B7210, &args![]);
            if e.call(SYSCOMMAND_TEST_004B71D0, &args![]).bool() {
                let command = wparam & 0xffff;
                if command == 0xf140 || command == 0xf170 {
                    return 1;
                }
            }
            default_window_proc(e, window, message, wparam, lparam)
        }
        _ => default_window_proc(e, window, message, wparam, lparam),
    }
}

/// `WM_ACTIVATE` with the low word of `wParam` 0 (see `0086a0a0`).
fn deactivate(e: &mut Engine) {
    if e.global::<u32>(MAIN) == 0 {
        return;
    }
    let main = e.global::<u32>(MAIN);
    if fn_0086a460(e, Ptr::new(main)) == 0 {
        return;
    }
    let main = e.global::<u32>(MAIN);
    if e.call(MAIN_TEST_004F1540, &args![main]).bool()
        && !e.call(TEST_0070EC00, &args![]).bool()
        && e.call(GET_MENU_BY_CLASS, &args![0x3f5u32]).u32() == 0
        && e.call(GET_MENU_BY_CLASS, &args![0x3efu32]).u32() == 0
    {
        e.call(
            QUEUE_MENU_CREATE,
            &args![6u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
    }
    if e.global::<u32>(MAIN) != 0 && e.call(TEST_00559450, &args![OBJECT_011DEB30]).u32() == 0 {
        let main = e.global::<u32>(MAIN);
        e.call(MAIN_SET_ACTIVE, &args![main, 0u32]);
    }
}

/// The shared body of the eight scalar deleting destructors: `destroy` on
/// `this`, then `operator delete` (`00401030`) on flag bit 0. Answers
/// `this`.
fn scalar_delete(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 008699f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869a20`, then
/// deleted on flag bit 0.
pub fn fn_008699f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869a20)
}

// Translated from 00869a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869a40`, then
/// deleted on flag bit 0.
pub fn fn_00869a70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869a40)
}

// Translated from 00869b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869b70`, then
/// deleted on flag bit 0.
pub fn fn_00869b40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869b70)
}

// Translated from 00869bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869b90`, then
/// deleted on flag bit 0.
pub fn fn_00869bc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869b90)
}

// Translated from 00869c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869cc0`, then
/// deleted on flag bit 0.
pub fn fn_00869c90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869cc0)
}

// Translated from 00869d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869ce0`, then
/// deleted on flag bit 0.
pub fn fn_00869d10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869ce0)
}

// Translated from 00869de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869e10`, then
/// deleted on flag bit 0.
pub fn fn_00869de0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869e10)
}

// Translated from 00869e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`, `RET 4`): `00869e30`, then
/// deleted on flag bit 0.
pub fn fn_00869e60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00869e30)
}

/// The callees of the shut-down callback `00869f30`, in its order.
const MODEL_LOADER_INSTANCE: u32 = 0x0043_c4b0;
const MODEL_LOADER_SHUTDOWN_00483710: u32 = 0x0048_3710;
const SHUTDOWN_00867090: u32 = 0x0086_7090;
/// The `IOManager` pointer (`01202d98`) and its calls `00c3e310`,
/// `00877d30`, `00c3e340`.
const IO_MANAGER: u32 = 0x0120_2d98;
const IO_MANAGER_00C3E310: u32 = 0x00c3_e310;
const IO_MANAGER_00877D30: u32 = 0x0087_7d30;
const IO_MANAGER_00C3E340: u32 = 0x00c3_e340;
/// The object at `011c3b3c` and its call `00449030`.
const OBJECT_011C3B3C: u32 = 0x011c_3b3c;
const OBJECT_00449030: u32 = 0x0044_9030;
/// `00aff100` (no argument): an instance; `00b01130` runs on it, then its
/// vtable `+8`.
const INSTANCE_00AFF100: u32 = 0x00af_f100;
const INSTANCE_00B01130: u32 = 0x00b0_1130;
/// `FalloutAudioMedia::MediaClose` (`008304a0`), `0082f9c0`,
/// `BSAudio::QInstance` (`00453a70`), `0078d050`, `0070ad30`,
/// `BSTaskletManager::QInstance` (`00b00a00`): all without arguments.
const MEDIA_CLOSE: u32 = 0x0083_04a0;
const AUDIO_MEDIA_0082F9C0: u32 = 0x0082_f9c0;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const LOADING_MENU_0078D050: u32 = 0x0078_d050;
const INTERFACE_0070AD30: u32 = 0x0070_ad30;
const TASKLET_MANAGER_INSTANCE: u32 = 0x00b0_0a00;

// Translated from 00869f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word; `main` registers it as a
/// callback with `004fb090`): for `kind` 2 only, shuts systems down in
/// order: the model loader (`0043c4b0`, `00483710` when there is one),
/// `00867090`, the `IOManager` (`00c3e310` when there is one), `00449030`
/// on `011c3b3c`, `00877d30` and `00c3e340` on the `IOManager`, `00b01130`
/// and vtable `+8` on `00aff100`'s instance, `MediaClose`, `0082f9c0`, the
/// audio instance's vtable `+8`, `0078d050`, `0070ad30`, the tasklet
/// manager's vtable `+8`, and again `00c3e310` on the `IOManager`.
pub fn fn_00869f30(e: &mut Engine, kind: u32) {
    if kind != 2 {
        return;
    }
    if e.call(MODEL_LOADER_INSTANCE, &args![]).u32() != 0 {
        let loader = e.call(MODEL_LOADER_INSTANCE, &args![]).u32();
        e.call(MODEL_LOADER_SHUTDOWN_00483710, &args![loader]);
    }
    e.call(SHUTDOWN_00867090, &args![]);
    if e.global::<u32>(IO_MANAGER) != 0 {
        let io = e.global::<u32>(IO_MANAGER);
        e.call(IO_MANAGER_00C3E310, &args![io]);
    }
    let object = e.global::<u32>(OBJECT_011C3B3C);
    e.call(OBJECT_00449030, &args![object]);
    let io = e.global::<u32>(IO_MANAGER);
    e.call(IO_MANAGER_00877D30, &args![io]);
    let io = e.global::<u32>(IO_MANAGER);
    e.call(IO_MANAGER_00C3E340, &args![io]);
    let instance = e.call(INSTANCE_00AFF100, &args![]).u32();
    e.call(INSTANCE_00B01130, &args![instance]);
    let instance = e.call(INSTANCE_00AFF100, &args![]).u32();
    e.vcall(instance, 8, &args![]);
    e.call(MEDIA_CLOSE, &args![]);
    e.call(AUDIO_MEDIA_0082F9C0, &args![]);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.vcall(audio, 8, &args![]);
    e.call(LOADING_MENU_0078D050, &args![]);
    e.call(INTERFACE_0070AD30, &args![]);
    let tasklets = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
    e.vcall(tasklets, 8, &args![]);
    if e.global::<u32>(IO_MANAGER) != 0 {
        let io = e.global::<u32>(IO_MANAGER);
        e.call(IO_MANAGER_00C3E310, &args![io]);
    }
}

// Translated from 0086a020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no stack word read, plain `RET`; `main`
/// registers it as a callback with `006ecd40`): the model loader's vtable
/// `+0x8c` when there is one (`0043c4b0`); answers 0.
pub fn fn_0086a020(e: &mut Engine) -> u32 {
    if e.call(MODEL_LOADER_INSTANCE, &args![]).u32() != 0 {
        let loader = e.call(MODEL_LOADER_INSTANCE, &args![]).u32();
        e.vcall(loader, 0x8c, &args![]);
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_99f0, fn_008699f0(Ptr, u32) -> Ptr),
        entry!(0x0086_9a70, fn_00869a70(Ptr, u32) -> Ptr),
        entry!(0x0086_9b40, fn_00869b40(Ptr, u32) -> Ptr),
        entry!(0x0086_9bc0, fn_00869bc0(Ptr, u32) -> Ptr),
        entry!(0x0086_9c90, fn_00869c90(Ptr, u32) -> Ptr),
        entry!(0x0086_9d10, fn_00869d10(Ptr, u32) -> Ptr),
        entry!(0x0086_9de0, fn_00869de0(Ptr, u32) -> Ptr),
        entry!(0x0086_9e60, fn_00869e60(Ptr, u32) -> Ptr),
        entry!(0x0086_9f30, fn_00869f30(u32)),
        entry!(0x0086_a020, fn_0086a020() -> u32),
        entry!(0x0086_9600, fn_00869600(Ptr) -> u32),
        entry!(0x0086_9640, fn_00869640(Ptr<NiTArray>, u32, u32) -> u32),
        entry!(0x0086_9680, fn_00869680(Ptr<NiTArray>, u32, Ptr) -> u32),
        entry!(0x0086_96c0, fn_008696c0(u32)),
        entry!(0x0086_96e0, fn_008696e0(Ptr<NiTArray>, u32)),
        entry!(0x0086_9840, fn_00869840(u32) -> u32),
        entry!(0x0086_9870, fn_00869870(Ptr<NiTArray>, u32, Ptr)),
        entry!(0x0086_9a40, fn_00869a40(Ptr)),
        entry!(
            0x0086_9aa0,
            fn_00869aa0(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0086_9b90, fn_00869b90(Ptr)),
        entry!(
            0x0086_9bf0,
            fn_00869bf0(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0086_9ce0, fn_00869ce0(Ptr)),
        entry!(
            0x0086_9d40,
            fn_00869d40(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0086_9e30, fn_00869e30(Ptr)),
        entry!(
            0x0086_9e90,
            fn_00869e90(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0086_a050, fn_0086a050(u32) -> f64),
        entry!(0x0086_a0a0, fn_0086a0a0(u32, u32, u32, u32) -> u32),
        entry!(0x0086_a460, fn_0086a460(Ptr) -> u8),
        entry!(0x0086_9a20, fn_00869a20(Ptr)),
        entry!(0x0086_9b70, fn_00869b70(Ptr)),
        entry!(0x0086_9cc0, fn_00869cc0(Ptr)),
        entry!(0x0086_9e10, fn_00869e10(Ptr)),
        entry!(0x0086_99c0, fn_008699c0(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9b10, fn_00869b10(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9c60, fn_00869c60(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9db0, fn_00869db0(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9f00, fn_00869f00(Ptr, u32, u32) -> Ptr),
    ]
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
    fn fwd_a03_00869a20_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9a40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9a20, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9a40), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00869b70_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9b90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9b70, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9b90), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00869cc0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9ce0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9cc0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9ce0), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00869e10_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9e30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9e10, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9e30), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn four_constructors_run_the_body_then_store_their_vtable() {
        for (entry, body, vtable) in [
            (0x0086_99c0, BASE_BODY_00869AA0, VTABLE_008699C0),
            (0x0086_9b10, BASE_BODY_00869BF0, VTABLE_00869B10),
            (0x0086_9c60, BASE_BODY_00869D40, VTABLE_00869C60),
            (0x0086_9db0, BASE_BODY_00869E90, VTABLE_00869DB0),
        ] {
            let mut e = Engine::new();
            e.register(body, |e, a| {
                e.mem.set_u32(a[0], 0x1234);
                Ret::default()
            });
            let this = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(entry, &args![this, 3u32, 4u32]).u32(), this);
            assert_eq!(calls_to(&mut e, body), vec![vec![this, 3, 4]]);
            assert_eq!(e.mem.u32(this), vtable);
        }
    }

    #[test]
    fn the_fifth_stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(BASE_BODY_006B3EB0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_9f00, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(calls_to(&mut e, BASE_BODY_006B3EB0), vec![vec![this, 5, 6]]);
        assert_eq!(e.mem.u32(this + 8), VTABLE_00869F00);
    }
}

#[cfg(test)]
mod array_tests {
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

    /// An array (capacity `max`, `size` used, `esize` counted, grow by 2)
    /// whose slots hold `values`.
    fn array(e: &mut Engine, max: u16, size: u16, esize: u16, values: &[u32]) -> u32 {
        let this = e.mem.alloc(0x10);
        let base = e.mem.alloc(4 * u32::from(max).max(1));
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, max);
        e.mem.set_u16(this + 0xa, size);
        e.mem.set_u16(this + 0xc, esize);
        e.mem.set_u16(this + 0xe, 2);
        this
    }

    fn memory(e: &mut Engine) {
        e.register(MEMORY_ALLOCATE, |e, a| ret(e.mem.alloc(a[0].max(4))));
        e.register(MEMORY_FREE, |_, _| ret(0));
    }

    #[test]
    fn shrink_size_halves_or_drops_0x400() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for (count, expected) in [
            (0x800u32, 0x400u32),
            (0x801, 0x401),
            (0x1000, 0xc00),
            (7, 3),
        ] {
            e.mem.set_u32(this + 0xc, count);
            assert_eq!(e.call(0x0086_9600, &args![this]).u32(), expected);
        }
    }

    #[test]
    fn allocate_asks_for_four_bytes_per_element_and_saturates() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |_, a| ret(a[0]));
        assert_eq!(e.call(0x0086_9840, &args![3u32]).u32(), 12);
        assert_eq!(e.call(0x0086_9840, &args![0x4000_0000u32]).u32(), u32::MAX);
        assert_eq!(
            e.call(0x0086_9840, &args![0x3fff_ffffu32]).u32(),
            0xffff_fffc
        );
    }

    #[test]
    fn deallocate_frees_the_block() {
        let mut e = Engine::new();
        e.register(MEMORY_FREE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0086_96c0, &args![0x1234u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![0x1234]]);
    }

    #[test]
    fn set_size_keeps_the_same_capacity() {
        let mut e = Engine::new();
        memory(&mut e);
        let this = array(&mut e, 4, 2, 2, &[1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x0086_96e0, &args![this, 4u32]);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    #[test]
    fn set_size_grows_copying_and_nulling() {
        let mut e = Engine::new();
        memory(&mut e);
        let this = array(&mut e, 2, 2, 2, &[0x11, 0x22]);
        let old = e.mem.u32(this + 4);
        e.call_log = Some(vec![]);
        e.call(0x0086_96e0, &args![this, 4u32]);
        let base = e.mem.u32(this + 4);
        assert_ne!(base, old);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), vec![vec![16]]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![old]]);
        assert_eq!(
            [0, 4, 8, 12].map(|o| e.mem.u32(base + o)),
            [0x11, 0x22, 0, 0]
        );
        assert_eq!(e.mem.u16(this + 8), 4);
        assert_eq!(e.mem.u16(this + 0xa), 2);
    }

    #[test]
    fn set_size_shrinks_below_the_size_uncounting_cleared_elements() {
        let mut e = Engine::new();
        memory(&mut e);
        let this = array(&mut e, 4, 4, 3, &[0x11, 0, 0x33, 0x44]);
        e.call(0x0086_96e0, &args![this, 1u32]);
        assert_eq!(e.mem.u16(this + 0xa), 1);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 8), 1);
        let base = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(base), 0x11);
        // To zero: a null buffer, the old one freed.
        let old = base;
        e.call_log = Some(vec![]);
        e.call(0x0086_96e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![old]]);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn set_at_grow_grows_first_when_needed() {
        let mut e = Engine::new();
        memory(&mut e);
        e.register(ARRAY_SET_AT, |_, _| ret(0));
        let this = array(&mut e, 2, 0, 0, &[]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_9640, &args![this, 1u32, 0x77u32]).u32(), 1);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(e.call(0x0086_9640, &args![this, 2u32, 0x77u32]).u32(), 2);
        assert_eq!(e.mem.u16(this + 8), 4);
        assert_eq!(
            calls_to(&e, ARRAY_SET_AT),
            vec![vec![this, 1, 0x77], vec![this, 2, 0x77]]
        );
    }

    /// Doubles for the pointer-array helpers: a pointer is empty when its
    /// word is 0.
    fn pointers(e: &mut Engine) {
        e.map(EMPTY_ELEMENT, 8);
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(ATEXIT, |_, _| ret(0));
        e.register(POINTER_DIFFERS, |e, a| {
            ret((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        e.register(POINTER_EQUALS, |e, a| {
            ret((e.mem.u32(a[0]) == e.mem.u32(a[1])) as u32)
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(POINTER_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            ret(0)
        });
    }

    fn element(e: &mut Engine, value: u32) -> u32 {
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, value);
        p
    }

    #[test]
    fn pointer_set_at_makes_the_empty_pointer_once_and_counts() {
        let mut e = Engine::new();
        pointers(&mut e);
        let this = array(&mut e, 4, 1, 1, &[0x55]);
        let item = element(&mut e, 0x77);
        let empty = element(&mut e, 0);
        e.call_log = Some(vec![]);
        // Past the size: raised and counted.
        e.call(0x0086_9870, &args![this, 2u32, item]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        // Into the empty slot 1: counted.
        e.call(0x0086_9870, &args![this, 1u32, item]);
        assert_eq!(e.mem.u16(this + 0xc), 3);
        // Empty over slot 0: uncounted; again: unchanged.
        e.call(0x0086_9870, &args![this, 0u32, empty]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        e.call(0x0086_9870, &args![this, 0u32, empty]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        // Past the size with an empty pointer: raised, not counted.
        e.call(0x0086_9870, &args![this, 3u32, empty]);
        assert_eq!(e.mem.u16(this + 0xa), 4);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        let base = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(base + 8), 0x77);
        assert_eq!(
            calls_to(&e, POINTER_CONSTRUCT),
            vec![vec![EMPTY_ELEMENT, 0]]
        );
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![EMPTY_ELEMENT_DESTRUCTOR]]);
    }

    #[test]
    fn pointer_set_at_grow_grows_then_sets() {
        let mut e = Engine::new();
        pointers(&mut e);
        let this = array(&mut e, 2, 0, 0, &[]);
        let base = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, base);
        let item = element(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_9680, &args![this, 3u32, item]).u32(), 3);
        assert_eq!(calls_to(&e, POINTER_ARRAY_SET_SIZE), vec![vec![this, 5]]);
        assert_eq!(e.mem.u32(base + 12), 0x77);
        assert_eq!(e.mem.u16(this + 0xa), 4);
    }

    #[test]
    fn constructors_store_their_vtables_and_allocate_for_a_maximum() {
        for (entry, vtable) in [
            (0x0086_9aa0u32, ARRAY_VTABLE_REFERENCES),
            (0x0086_9bf0, ARRAY_VTABLE_BIPED_ANIMS),
            (0x0086_9d40, ARRAY_VTABLE_ANIMATIONS),
        ] {
            let mut e = Engine::new();
            e.register(MEMORY_ALLOCATE, |_, a| ret(0x5000 + a[0]));
            let this = e.mem.alloc(0x10);
            e.mem.write(this, &[0xaa; 0x10]);
            assert_eq!(
                e.call(entry, &args![this, 0x0001_0003u32, 7u32]).u32(),
                this
            );
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 4), 0x500c);
            assert_eq!(e.mem.u16(this + 8), 3);
            assert_eq!(e.mem.u16(this + 0xa), 0);
            assert_eq!(e.mem.u16(this + 0xc), 0);
            assert_eq!(e.mem.u16(this + 0xe), 7);
            e.call(entry, &args![this, 0u32, 7u32]);
            assert_eq!(e.mem.u32(this + 4), 0);
        }
        let mut e = Engine::new();
        e.register(POINTER_ARRAY_ALLOCATE, |_, a| ret(0x6000 + a[0]));
        let this = e.mem.alloc(0x10);
        e.call(0x0086_9e90, &args![this, 5u32, 1u32]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_TEMP_EFFECTS);
        assert_eq!(e.mem.u32(this + 4), 0x6005);
    }

    #[test]
    fn destructors_store_their_vtables_and_free_the_buffer() {
        for (entry, vtable, free) in [
            (0x0086_9a40u32, ARRAY_VTABLE_REFERENCES, MEMORY_FREE),
            (0x0086_9b90, ARRAY_VTABLE_BIPED_ANIMS, MEMORY_FREE),
            (0x0086_9ce0, ARRAY_VTABLE_ANIMATIONS, MEMORY_FREE),
            (0x0086_9e30, ARRAY_VTABLE_TEMP_EFFECTS, POINTER_ARRAY_DELETE),
        ] {
            let mut e = Engine::new();
            e.register(free, |_, _| ret(0));
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 4, 0x4321);
            e.call_log = Some(vec![]);
            e.call(entry, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls_to(&e, free), vec![vec![0x4321]]);
        }
    }

    #[test]
    fn float_of_the_object_or_one() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x420 / 4 + 1];
        slots[0x100 / 4] = 0x00ee_0100;
        slots[0x420 / 4] = 0x00ee_0420;
        e.put_vtable(0x0300_0000, &slots);
        e.mem.set_u32(object, 0x0300_0000);
        e.register_double(OBJECT_OF, move |_, a| {
            ret(if a[0] == 1 { object } else { 0 })
        });
        e.register(0x00ee_0100, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x00ee_0420, |_, _| Ret {
            st0: 2.75,
            ..Ret::default()
        });
        e.mem.set_u32(object + 4, 1);
        assert_eq!(e.call(0x0086_a050, &args![1u32]).f64(), 2.75);
        e.mem.set_u32(object + 4, 0x100);
        assert_eq!(e.call(0x0086_a050, &args![1u32]).f64(), 1.0);
        assert_eq!(e.call(0x0086_a050, &args![0u32]).f64(), 1.0);
    }

    #[test]
    fn byte_three_of_the_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x5a00_0000);
        assert_eq!(e.call(0x0086_a460, &args![this]).u8(), 0x5a);
    }
}

#[cfg(test)]
mod window_proc_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const WINDOW: u32 = 0x0000_1234;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Every callee doubled and logged; `answers` gives each address's
    /// `EAX`. `Main` is set when `main` (its byte `+3` is 1).
    fn fixture(main: bool, answers: &[(u32, u32)]) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(MAIN, 4);
        e.map(NC_DEACTIVATED, 4);
        e.map(CURSOR_SHOWN, 4);
        let main = if main {
            let p = e.mem.alloc(8);
            e.mem.set_u8(p + 3, 1);
            p
        } else {
            0
        };
        e.mem.set_u32(MAIN, main);
        for address in [
            DEF_WINDOW_PROC,
            SHOW_CURSOR,
            MAIN_SET_ACTIVE,
            MAIN_GET_00877720,
            ACTIVATE_00A23C00,
            MAIN_TEST_004F1540,
            TEST_0070EC00,
            GET_MENU_BY_CLASS,
            QUEUE_MENU_CREATE,
            TEST_00559450,
            MAIN_MOUSE_MOVE,
            TEST_004BC3F0,
            WINDOW_MOVED,
            TEST_00446E10,
            MAIN_ON_DESTROY,
            RENDERER_WINDOW,
            SYSCOMMAND_004B7210,
            SYSCOMMAND_TEST_004B71D0,
        ] {
            let answer = answers
                .iter()
                .find(|(a, _)| *a == address)
                .map_or(0, |(_, v)| *v);
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                ret(answer)
            });
        }
        (e, log, main)
    }

    fn window_proc(e: &mut Engine, message: u32, wparam: u32, lparam: u32) -> u32 {
        e.call(0x0086_a0a0, &args![WINDOW, message, wparam, lparam])
            .u32()
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(a, _)| *a).collect()
    }

    #[test]
    fn other_messages_go_to_the_default_procedure() {
        let (mut e, log, _) = fixture(true, &[(DEF_WINDOW_PROC, 0x99)]);
        for message in [0u32, 1, 3, 0x10, 0x48, 0x85, 0x113, 0x201, 0x280] {
            log.borrow_mut().clear();
            assert_eq!(window_proc(&mut e, message, 7, 8), 0x99);
            assert_eq!(
                *log.borrow(),
                [(DEF_WINDOW_PROC, vec![WINDOW, message, 7, 8])]
            );
        }
        // WM_IME_SETCONTEXT passes lParam 0.
        log.borrow_mut().clear();
        assert_eq!(window_proc(&mut e, 0x281, 7, 8), 0x99);
        assert_eq!(
            *log.borrow(),
            [(DEF_WINDOW_PROC, vec![WINDOW, 0x281, 7, 0])]
        );
        // WM_SIZE and WM_SYSKEYUP answer 0 directly.
        log.borrow_mut().clear();
        assert_eq!(window_proc(&mut e, 5, 0, 0), 0);
        assert_eq!(window_proc(&mut e, 0x105, 0, 0), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn activation_makes_main_active() {
        let (mut e, log, main) = fixture(true, &[(MAIN_GET_00877720, 0x4000)]);
        assert_eq!(window_proc(&mut e, 6, 1, 0), 0);
        assert_eq!(
            *log.borrow(),
            [
                (MAIN_SET_ACTIVE, vec![main, 1]),
                (MAIN_GET_00877720, vec![main]),
                (ACTIVATE_00A23C00, vec![0x4000, 0]),
            ]
        );
        // Minimized (high word set) or an unknown state: nothing.
        log.borrow_mut().clear();
        assert_eq!(window_proc(&mut e, 6, 0x0001_0002, 0), 0);
        assert_eq!(window_proc(&mut e, 6, 3, 0), 0);
        assert!(log.borrow().is_empty());
        // No Main: nothing.
        let (mut e, log, _) = fixture(false, &[]);
        assert_eq!(window_proc(&mut e, 6, 2, 0), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn deactivation_queues_the_menu_and_makes_main_inactive() {
        let (mut e, log, main) = fixture(true, &[(MAIN_TEST_004F1540, 1)]);
        assert_eq!(window_proc(&mut e, 6, 0, 0), 0);
        assert_eq!(
            addresses(&log),
            [
                MAIN_TEST_004F1540,
                TEST_0070EC00,
                GET_MENU_BY_CLASS,
                GET_MENU_BY_CLASS,
                QUEUE_MENU_CREATE,
                TEST_00559450,
                MAIN_SET_ACTIVE,
            ]
        );
        let log = log.borrow();
        assert_eq!(log[2].1, [0x3f5]);
        assert_eq!(log[3].1, [0x3ef]);
        assert_eq!(log[4].1, [6, 0, 0, 0, 1, 0]);
        assert_eq!(log[5].1, [OBJECT_011DEB30]);
        assert_eq!(log[6].1, [main, 0]);
    }

    #[test]
    fn deactivation_skips_the_menu_when_one_is_open_or_main_is_not_ready() {
        let (mut e, log, _) = fixture(
            true,
            &[
                (MAIN_TEST_004F1540, 1),
                (GET_MENU_BY_CLASS, 0x77),
                (TEST_00559450, 1),
            ],
        );
        assert_eq!(window_proc(&mut e, 6, 0, 0), 0);
        assert_eq!(
            addresses(&log),
            [
                MAIN_TEST_004F1540,
                TEST_0070EC00,
                GET_MENU_BY_CLASS,
                TEST_00559450
            ]
        );
        let (mut e, log, _) = fixture(true, &[]);
        assert_eq!(window_proc(&mut e, 6, 0, 0), 0);
        assert_eq!(
            addresses(&log),
            [MAIN_TEST_004F1540, TEST_00559450, MAIN_SET_ACTIVE]
        );
        // Main's byte +3 clear: nothing at all.
        let (mut e, log, main) = fixture(true, &[]);
        e.mem.set_u8(main + 3, 0);
        assert_eq!(window_proc(&mut e, 6, 0, 0), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn hit_test_shows_and_hides_the_cursor() {
        let (mut e, log, main) = fixture(
            true,
            &[
                (DEF_WINDOW_PROC, 2),
                (MAIN_GET_00877720, 1),
                (SHOW_CURSOR, 0),
            ],
        );
        assert_eq!(window_proc(&mut e, 0x84, 0, 0), 2);
        assert_eq!(e.mem.u8(CURSOR_SHOWN), 1);
        assert_eq!(
            *log.borrow(),
            [
                (DEF_WINDOW_PROC, vec![WINDOW, 0x84, 0, 0]),
                (MAIN_GET_00877720, vec![main]),
                (SHOW_CURSOR, vec![1]),
            ]
        );
        // Shown already: not again.
        log.borrow_mut().clear();
        window_proc(&mut e, 0x84, 0, 0);
        assert!(!addresses(&log).contains(&SHOW_CURSOR));
        // Inside the client area: hidden until the count is not positive.
        let (mut e, _, _) = fixture(true, &[(DEF_WINDOW_PROC, 1), (MAIN_GET_00877720, 1)]);
        e.mem.set_u8(CURSOR_SHOWN, 1);
        let counts: std::rc::Rc<std::cell::RefCell<Vec<u32>>> = Default::default();
        let seen = counts.clone();
        let mut left = 2i32;
        e.register_double(SHOW_CURSOR, move |_, a| {
            seen.borrow_mut().push(a[0]);
            left -= 1;
            ret(left as u32)
        });
        assert_eq!(window_proc(&mut e, 0x84, 0, 0), 1);
        assert_eq!(*counts.borrow(), [0, 0]);
        assert_eq!(e.mem.u8(CURSOR_SHOWN), 0);
    }

    #[test]
    fn non_client_deactivation_sets_the_flag() {
        let (mut e, _, _) = fixture(true, &[(DEF_WINDOW_PROC, 5)]);
        assert_eq!(window_proc(&mut e, 0x86, 1, 0), 5);
        assert_eq!(e.mem.u8(NC_DEACTIVATED), 0);
        assert_eq!(window_proc(&mut e, 0x86, 0, 0), 5);
        assert_eq!(e.mem.u8(NC_DEACTIVATED), 1);
    }

    #[test]
    fn mouse_move_goes_to_main_first() {
        let (mut e, log, main) = fixture(true, &[(MAIN_MOUSE_MOVE, 1)]);
        assert_eq!(window_proc(&mut e, 0x200, 0, 0x0020_0010), 0);
        assert_eq!(*log.borrow(), [(MAIN_MOUSE_MOVE, vec![main, 0x10, 0x20])]);
        let (mut e, log, _) = fixture(true, &[(MAIN_MOUSE_MOVE, 0x100), (DEF_WINDOW_PROC, 3)]);
        assert_eq!(window_proc(&mut e, 0x200, 0, 0), 3);
        assert_eq!(addresses(&log), [MAIN_MOUSE_MOVE, DEF_WINDOW_PROC]);
    }

    #[test]
    fn window_position_changes_reach_the_renderer() {
        let (mut e, log, _) = fixture(true, &[(TEST_004BC3F0, 1), (DEF_WINDOW_PROC, 4)]);
        let position = e.mem.alloc(0x1c);
        e.mem.set_u32(position + 8, 100);
        e.mem.set_u32(position + 0xc, 200);
        assert_eq!(window_proc(&mut e, 0x47, 0, position), 4);
        assert_eq!(log.borrow()[1], (WINDOW_MOVED, vec![100, 200]));
        let (mut e, log, _) = fixture(true, &[]);
        assert_eq!(window_proc(&mut e, 0x47, 0, 0), 0);
        assert_eq!(addresses(&log), [TEST_004BC3F0, TEST_00446E10]);
        let (mut e, log, _) = fixture(true, &[(TEST_00446E10, 1), (DEF_WINDOW_PROC, 4)]);
        assert_eq!(window_proc(&mut e, 0x47, 0, 0), 4);
        assert_eq!(
            addresses(&log),
            [TEST_004BC3F0, TEST_00446E10, DEF_WINDOW_PROC]
        );
    }

    #[test]
    fn destroy_erase_and_system_commands() {
        let (mut e, log, main) = fixture(true, &[(DEF_WINDOW_PROC, 9)]);
        assert_eq!(window_proc(&mut e, 2, 0, 0), 9);
        assert_eq!(log.borrow()[0], (MAIN_ON_DESTROY, vec![main]));
        let (mut e, log, _) = fixture(false, &[(DEF_WINDOW_PROC, 9)]);
        assert_eq!(window_proc(&mut e, 2, 0, 0), 9);
        assert_eq!(addresses(&log), [DEF_WINDOW_PROC]);
        let (mut e, _, _) = fixture(true, &[(RENDERER_WINDOW, WINDOW), (DEF_WINDOW_PROC, 9)]);
        assert_eq!(window_proc(&mut e, 0x14, 0, 0), 1);
        let (mut e, _, _) = fixture(true, &[(RENDERER_WINDOW, 1), (DEF_WINDOW_PROC, 9)]);
        assert_eq!(window_proc(&mut e, 0x14, 0, 0), 9);
        let (mut e, log, _) = fixture(true, &[(SYSCOMMAND_TEST_004B71D0, 1), (DEF_WINDOW_PROC, 9)]);
        assert_eq!(window_proc(&mut e, 0x112, 0xf140, 0), 1);
        assert_eq!(window_proc(&mut e, 0x112, 0x0001_f170, 0), 1);
        assert_eq!(window_proc(&mut e, 0x112, 0xf020, 0), 9);
        assert_eq!(
            addresses(&log)[..2],
            [SYSCOMMAND_004B7210, SYSCOMMAND_TEST_004B71D0]
        );
        let (mut e, _, _) = fixture(true, &[(DEF_WINDOW_PROC, 9)]);
        assert_eq!(window_proc(&mut e, 0x112, 0xf140, 0), 9);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_scalar_deleting_destructors_store_their_vtables() {
        for (address, vtable) in [
            (0x0086_99f0u32, ARRAY_VTABLE_REFERENCES),
            (0x0086_9a70, ARRAY_VTABLE_REFERENCES),
            (0x0086_9b40, ARRAY_VTABLE_BIPED_ANIMS),
            (0x0086_9bc0, ARRAY_VTABLE_BIPED_ANIMS),
            (0x0086_9c90, ARRAY_VTABLE_ANIMATIONS),
            (0x0086_9d10, ARRAY_VTABLE_ANIMATIONS),
            (0x0086_9de0, ARRAY_VTABLE_TEMP_EFFECTS),
            (0x0086_9e60, ARRAY_VTABLE_TEMP_EFFECTS),
        ] {
            for flags in [0u32, 1] {
                let mut e = Engine::new();
                e.register(MEMORY_FREE, |_, _| Ret::default());
                e.register(POINTER_ARRAY_DELETE, |_, _| Ret::default());
                let this = e.mem.alloc(0x10);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(e.mem.u32(this), vtable);
                let deleted = calls(&e, MEMORY_FREE).contains(&vec![this]);
                assert_eq!(deleted, flags == 1);
            }
        }
    }

    fn shutdown_engine(loader: u32, io: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global::<u32>(IO_MANAGER, io);
        e.set_global::<u32>(OBJECT_011C3B3C, 0x3b3c);
        e.register_double(MODEL_LOADER_INSTANCE, move |_, _| loader.into_ret());
        e.put_vtable(0x0150_0000, &[0, 0, 0x0150_1008]);
        e.register(0x0150_1008, |_, _| Ret::default());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0150_0000);
        for a in [INSTANCE_00AFF100, AUDIO_INSTANCE, TASKLET_MANAGER_INSTANCE] {
            e.register_double(a, move |_, _| object.into_ret());
        }
        for a in [
            MODEL_LOADER_SHUTDOWN_00483710,
            SHUTDOWN_00867090,
            IO_MANAGER_00C3E310,
            OBJECT_00449030,
            IO_MANAGER_00877D30,
            IO_MANAGER_00C3E340,
            INSTANCE_00B01130,
            MEDIA_CLOSE,
            AUDIO_MEDIA_0082F9C0,
            LOADING_MENU_0078D050,
            INTERFACE_0070AD30,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00869f30_shuts_down_on_kind_2_only() {
        let mut e = shutdown_engine(0x4400, 0x9800);
        e.call(0x0086_9f30, &args![1u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x0086_9f30, &args![2u32]);
        assert_eq!(
            calls(&e, MODEL_LOADER_SHUTDOWN_00483710),
            vec![vec![0x4400]]
        );
        assert_eq!(calls(&e, IO_MANAGER_00C3E310), vec![vec![0x9800]; 2]);
        assert_eq!(calls(&e, OBJECT_00449030), vec![vec![0x3b3c]]);
        assert_eq!(calls(&e, 0x0150_1008).len(), 3);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        let at = |a: u32| order.iter().position(|x| *x == a).unwrap();
        assert!(at(SHUTDOWN_00867090) < at(OBJECT_00449030));
        assert!(at(MEDIA_CLOSE) < at(LOADING_MENU_0078D050));
    }

    #[test]
    fn fn_00869f30_without_loader_or_io_manager() {
        let mut e = shutdown_engine(0, 0);
        e.call(0x0086_9f30, &args![2u32]);
        assert!(calls(&e, MODEL_LOADER_SHUTDOWN_00483710).is_empty());
        assert!(calls(&e, IO_MANAGER_00C3E310).is_empty());
        assert_eq!(calls(&e, IO_MANAGER_00877D30), vec![vec![0]]);
    }

    #[test]
    fn fn_0086a020_runs_the_loaders_slot() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x24];
        slots[0x8c / 4] = 0x0150_108c;
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_108c, |_, _| 7u32.into_ret());
        let loader = e.mem.alloc(4);
        e.mem.set_u32(loader, 0x0150_0000);
        e.register_double(MODEL_LOADER_INSTANCE, move |_, _| loader.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_a020, &args![]).u32(), 0);
        assert_eq!(calls(&e, 0x0150_108c), vec![vec![loader]]);
        e.register(MODEL_LOADER_INSTANCE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_a020, &args![]).u32(), 0);
        assert!(calls(&e, 0x0150_108c).is_empty());
    }
}
