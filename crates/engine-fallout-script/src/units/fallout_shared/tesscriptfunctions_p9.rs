//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 9: its functions from `005e0310` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: the 32 open functions of the range (`005e0310` to `005e0aa0`)
//! are translated; the queue has nothing else open in the range except
//! `00f46120`, which the ledger counts as `library`.
//!
//! The functions of this stretch are small template instances the linker
//! placed among the script functions: constructors and destructors of
//! `NiTArray`-shaped arrays (vtable, element pointer, four 16-bit counts)
//! and `BSSimpleArray`-shaped arrays (vtable, buffer, size, reserved size),
//! their `scalar deleting destructor` wrappers, and a few helpers around
//! them. Which element type each instance holds is not visible in the code
//! (only the names the engine map gives to three of them), so the doc
//! comments describe the shape.

#[allow(unused_imports)]
use super::tesscriptfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiTArray};

// ---- Callees outside this part (by exe address) ----------------------------

/// `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (Xbox PDB), `cdecl`
/// (`count`): a block for `count` pointers.
const ALLOCATE_POINTERS: u32 = 0x005e_0ba0;
/// Allocator used by the other two array constructors (`cdecl`, `count`).
const ALLOCATE_ELEMENTS: u32 = 0x0096_afc0;
/// Constructor body of the array behind `005e0490` (`thiscall`, max size and
/// grow-by as two words).
const ARRAY_CONSTRUCTOR_005E0A30: u32 = 0x005e_0a30;
/// Frees the element block of an array (`cdecl`, the block).
const FREE_ELEMENT_BLOCK: u32 = 0x004e_de70;
/// `BSSimpleArray` constructor body (`thiscall`, two words, both 0 here).
const SIMPLE_ARRAY_INIT: u32 = 0x006b_3eb0;
/// `BSSimpleArray` clear (`thiscall`, a flag byte: 1 also releases the
/// buffer).
const SIMPLE_ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `operator delete` wrapper (`cdecl`, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Returns the word at the start of its object (`fastcall`, `*this`).
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `fastcall`; returns the base address `005e05b0` offsets from (it calls
/// `00559450`, the first-word reader).
const BLOCK_BASE: u32 = 0x0045_8b50;
/// `fastcall` initializer of the object `005e05d0` builds on (returns
/// `this`).
const BLOCK_OBJECT_INIT: u32 = 0x0062_40d0;
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer` (Xbox PDB), `thiscall`
/// with one word.
const UNLOADED_FORM_BUFFER_CONSTRUCTOR: u32 = 0x0053_7e90;
/// Returns the pool object the block allocator hangs on (`cdecl`, no
/// arguments).
const BLOCK_POOL: u32 = 0x00c8_5750;
/// Identity `thiscall` (plain `RET`, returns `ECX`): the size and flag
/// words pushed before it stay on the stack for `005e0ad0`.
const POOL_ALLOCATE: u32 = 0x0068_15c0;
/// Follow-up `thiscall` on the pool (`RET 8`: the size and flag words
/// left by `006815c0`).
const POOL_FINISH: u32 = 0x005e_0ad0;
/// `005e0600`'s callee (`cdecl`, one word).
const RELEASE_BLOCK: u32 = 0x005e_0b60;

// Destructor bodies of other parts that the deleting wrappers call.
const DESTRUCTOR_005E02E0: u32 = 0x005e_02e0;
const DESTRUCTOR_005DD810: u32 = 0x005d_d810;
const DESTRUCTOR_005DD7B0: u32 = 0x005d_d7b0;
const DESTRUCTOR_005DD7D0: u32 = 0x005d_d7d0;
const DESTRUCTOR_005DD7F0: u32 = 0x005d_d7f0;

// Vtables the constructors store (read-only data of the exe).
const VTABLE_0103D3E8: u32 = 0x0103_d3e8;
const VTABLE_0103D3F0: u32 = 0x0103_d3f0;
const VTABLE_0103D3F8: u32 = 0x0103_d3f8;
const VTABLE_0103D400: u32 = 0x0103_d400;
const VTABLE_0103D408: u32 = 0x0103_d408;
const VTABLE_0103D410: u32 = 0x0103_d410;
const VTABLE_0103D418: u32 = 0x0103_d418;
const VTABLE_0103D420: u32 = 0x0103_d420;
const VTABLE_0103D428: u32 = 0x0103_d428;
const VTABLE_0103D43C: u32 = 0x0103_d43c;
const VTABLE_0103D450: u32 = 0x0103_d450;

/// Stores `vtable` in the first word of the object (the `NiTArray` and
/// `BSSimpleArray` layouts have no field for it).
fn set_vtable(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
}

/// The body the three array constructors share: vtable, capacity, grow-by,
/// both counts zero, then the element block (null when the capacity is 0).
fn construct_array(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    vtable: u32,
    allocate: u32,
    max_size: u16,
    grow_by: u16,
) {
    set_vtable(e, this.cast(), vtable);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if e.get(this, NiTArray::m_usMaxSize) > 0 {
        let max = e.get(this, NiTArray::m_usMaxSize) as u32;
        let block = e.call(allocate, &args![max]).u32();
        e.set(this, NiTArray::m_pBase, block);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
}

/// The scalar-deleting-destructor ending: free the object when bit 0 of
/// `flags` is set, return `this`.
fn delete_if_flagged(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 005e0310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an `NiTArray`-shaped array class: the base constructor
/// `005e08e0` (capacity and grow-by), then the class vtable `0103d3f0`.
pub fn fn_005e0310(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_005e08e0(e, this, max_size, grow_by);
    set_vtable(e, this.cast(), VTABLE_0103D3F0);
    this
}

// Translated from 005e0340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `NiTArray`-shaped class with vtable `0103d3f8`: stores
/// the vtable and frees the element block (Ghidra's library name
/// `~basic_streambuf<>` is an identical-code match, not this function).
pub fn fn_005e0340(e: &mut Engine, this: Ptr<NiTArray>) {
    set_vtable(e, this.cast(), VTABLE_0103D3F8);
    let block = e.get(this, NiTArray::m_pBase);
    e.call(FREE_ELEMENT_BLOCK, &args![block]);
}

// Translated from 005e0370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an `NiTArray`-shaped array class: the base constructor
/// `005e0950`, then vtable `0103d400`.
pub fn fn_005e0370(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_005e0950(e, this, max_size, grow_by);
    set_vtable(e, this.cast(), VTABLE_0103D400);
    this
}

// Translated from 005e03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `NiTArray`-shaped class with vtable `0103d408`: stores
/// the vtable and frees the element block.
pub fn fn_005e03a0(e: &mut Engine, this: Ptr<NiTArray>) {
    set_vtable(e, this.cast(), VTABLE_0103D408);
    let block = e.get(this, NiTArray::m_pBase);
    e.call(FREE_ELEMENT_BLOCK, &args![block]);
}

// Translated from 005e03d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray`-shaped `RemoveAll`: nulls each of the `m_usSize` slots, then
/// resets `m_usSize` and `m_usESize` to 0.
pub fn fn_005e03d0(e: &mut Engine, this: Ptr<NiTArray>) {
    let mut index: u16 = 0;
    while (index as u32) < e.get(this, NiTArray::m_usSize) as u32 {
        let base = e.get(this, NiTArray::m_pBase);
        e.mem.set_u32(base.wrapping_add(index as u32 * 4), 0);
        index = index.wrapping_add(1);
    }
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
}

// Translated from 005e0430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an `NiTArray`-shaped array class: the base constructor
/// `005e09c0`, then vtable `0103d410`.
pub fn fn_005e0430(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_005e09c0(e, this, max_size, grow_by);
    set_vtable(e, this.cast(), VTABLE_0103D410);
    this
}

// Translated from 005e0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `NiTArray`-shaped class with vtable `0103d418`: stores
/// the vtable and frees the element block.
pub fn fn_005e0460(e: &mut Engine, this: Ptr<NiTArray>) {
    set_vtable(e, this.cast(), VTABLE_0103D418);
    let block = e.get(this, NiTArray::m_pBase);
    e.call(FREE_ELEMENT_BLOCK, &args![block]);
}

// Translated from 005e0490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an array class: the base constructor `005e0a30` (called by
/// address, it belongs to another unit), then vtable `0103d420`.
pub fn fn_005e0490(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.call(ARRAY_CONSTRUCTOR_005E0A30, &args![this, max_size, grow_by]);
    set_vtable(e, this.cast(), VTABLE_0103D420);
    this
}

// Translated from 005e04c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray`-shaped class: stores vtable `0103d428`
/// and runs the array initializer with zero capacity and zero size.
pub fn fn_005e04c0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    set_vtable(e, this.cast(), VTABLE_0103D428);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 005e04f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray`-shaped class with vtable `0103d428`:
/// stores the vtable and clears the array, releasing the buffer.
pub fn fn_005e04f0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    set_vtable(e, this.cast(), VTABLE_0103D428);
    e.call(SIMPLE_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 005e0510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray`-shaped class: like `005e04c0` with
/// vtable `0103d43c`.
pub fn fn_005e0510(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    set_vtable(e, this.cast(), VTABLE_0103D43C);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 005e0540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray`-shaped class with vtable `0103d43c`.
pub fn fn_005e0540(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    set_vtable(e, this.cast(), VTABLE_0103D43C);
    e.call(SIMPLE_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 005e0560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray`-shaped class: like `005e04c0` with
/// vtable `0103d450`.
pub fn fn_005e0560(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    set_vtable(e, this.cast(), VTABLE_0103D450);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 005e0590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray`-shaped class with vtable `0103d450`.
pub fn fn_005e0590(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    set_vtable(e, this.cast(), VTABLE_0103D450);
    e.call(SIMPLE_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 005e05b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the `index`-th 0x200-byte block after the base address that
/// `00458b50` gives for `this`.
pub fn fn_005e05b0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = e.call(BLOCK_BASE, &args![this]).u32();
    base.wrapping_add(index << 9)
}

// Translated from 005e05d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that initializes `this` (`006240d0`), allocates a pool block
/// of `block_count` 0x200-byte units with `005e0aa0` and hands it to
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer` (`00537e90`, Xbox PDB).
/// The second stack word is never read.
pub fn fn_005e05d0(e: &mut Engine, this: Ptr, block_count: u32, _unused_1: u32) -> Ptr {
    e.call(BLOCK_OBJECT_INIT, &args![this]);
    let block = fn_005e0aa0(e, block_count, 0);
    e.call(UNLOADED_FORM_BUFFER_CONSTRUCTOR, &args![this, block]);
    this
}

// Translated from 005e0600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the first word of `this` (`00559450`) and passes it to `005e0b60`.
pub fn fn_005e0600(e: &mut Engine, this: Ptr) {
    let word = e.call(READ_FIRST_WORD, &args![this]).u32();
    e.call(RELEASE_BLOCK, &args![word]);
}

// Translated from 005e0620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: runs the destructor `005e02e0`, then frees
/// the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e0620(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_005E02E0, &args![this]);
    delete_if_flagged(e, this, flags)
}

// Translated from 005e0650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005dd810`.
pub fn fn_005e0650(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_005DD810, &args![this]);
    delete_if_flagged(e, this, flags)
}

// Translated from 005e0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005e0340`.
pub fn fn_005e0680(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr {
    fn_005e0340(e, this);
    delete_if_flagged(e, this.cast(), flags)
}

// Translated from 005e06b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005dd7b0`.
pub fn fn_005e06b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_005DD7B0, &args![this]);
    delete_if_flagged(e, this, flags)
}

// Translated from 005e06e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005e03a0`.
pub fn fn_005e06e0(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr {
    fn_005e03a0(e, this);
    delete_if_flagged(e, this.cast(), flags)
}

// Translated from 005e0710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005dd7d0`.
pub fn fn_005e0710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_005DD7D0, &args![this]);
    delete_if_flagged(e, this, flags)
}

// Translated from 005e0740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005e0460`.
pub fn fn_005e0740(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr {
    fn_005e0460(e, this);
    delete_if_flagged(e, this.cast(), flags)
}

// Translated from 005e0770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor around `005dd7f0`.
pub fn fn_005e0770(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_005DD7F0, &args![this]);
    delete_if_flagged(e, this, flags)
}

// Translated from 005e07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESObjectREFR *,1024>::scalar deleting destructor`
/// (Xbox PDB): runs `005e04f0`, then frees the object when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_tes_object_refr_p_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr {
    fn_005e04f0(e, this);
    delete_if_flagged(e, this.cast(), flags)
}

// Translated from 005e07d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshInfo const *,1024>::scalar deleting destructor`
/// (Xbox PDB): runs `005e0540`, then frees the object when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_nav_mesh_info_const_p_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr {
    fn_005e0540(e, this);
    delete_if_flagged(e, this.cast(), flags)
}

// Translated from 005e0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESNPC *,1024>::scalar deleting destructor` (Xbox PDB):
/// runs `005e0590`, then frees the object when bit 0 of `flags` is set.
pub fn bs_simple_array_tesnpc_p_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr {
    fn_005e0590(e, this);
    delete_if_flagged(e, this.cast(), flags)
}

// Translated from 005e08e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of an `NiTArray` of `NiPointer<NiAVObject>`-style
/// pointers: vtable `0103d3e8`, capacity `max_size`, grow-by `grow_by`,
/// counts 0, and an element block from
/// `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (`005e0ba0`, Xbox PDB)
/// when the capacity is not 0 (otherwise a null block).
pub fn fn_005e08e0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        VTABLE_0103D3E8,
        ALLOCATE_POINTERS,
        max_size,
        grow_by,
    );
    this
}

// Translated from 005e0950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of an `NiTArray`-shaped array: like `005e08e0` with
/// vtable `0103d3f8` and the element allocator `0096afc0`.
pub fn fn_005e0950(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        VTABLE_0103D3F8,
        ALLOCATE_ELEMENTS,
        max_size,
        grow_by,
    );
    this
}

// Translated from 005e09c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of an `NiTArray`-shaped array: like `005e08e0` with
/// vtable `0103d408` and the element allocator `0096afc0`.
pub fn fn_005e09c0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    construct_array(
        e,
        this,
        VTABLE_0103D408,
        ALLOCATE_ELEMENTS,
        max_size,
        grow_by,
    );
    this
}

// Translated from 005e0aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a pool block of `block_count` 0x200-byte units (rounded up to
/// 128 bytes): gets the pool (`00c85750`), allocates from it (`006815c0`,
/// an identity call whose two pushed words, the size and `flag`, stay on
/// the stack) and runs the follow-up `005e0ad0` on the result with those
/// two words (`RET 8`), returning its value.
pub fn fn_005e0aa0(e: &mut Engine, block_count: u32, flag: u32) -> u32 {
    let pool = e.call(BLOCK_POOL, &args![]).u32();
    let size = (block_count << 9).wrapping_add(0x7f) & 0xffff_ff80;
    let block = e.call(POOL_ALLOCATE, &args![pool]).u32();
    e.call(POOL_FINISH, &args![block, size, flag]).u32()
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005e0310,
            fn_005e0310(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x005e0340, fn_005e0340(Ptr<NiTArray>)),
        entry!(
            0x005e0370,
            fn_005e0370(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x005e03a0, fn_005e03a0(Ptr<NiTArray>)),
        entry!(0x005e03d0, fn_005e03d0(Ptr<NiTArray>)),
        entry!(
            0x005e0430,
            fn_005e0430(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x005e0460, fn_005e0460(Ptr<NiTArray>)),
        entry!(
            0x005e0490,
            fn_005e0490(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x005e04c0,
            fn_005e04c0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x005e04f0, fn_005e04f0(Ptr<BSSimpleArray>)),
        entry!(
            0x005e0510,
            fn_005e0510(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x005e0540, fn_005e0540(Ptr<BSSimpleArray>)),
        entry!(
            0x005e0560,
            fn_005e0560(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x005e0590, fn_005e0590(Ptr<BSSimpleArray>)),
        entry!(0x005e05b0, fn_005e05b0(Ptr, u32) -> u32),
        entry!(0x005e05d0, fn_005e05d0(Ptr, u32, u32) -> Ptr),
        entry!(0x005e0600, fn_005e0600(Ptr)),
        entry!(0x005e0620, fn_005e0620(Ptr, u32) -> Ptr),
        entry!(0x005e0650, fn_005e0650(Ptr, u32) -> Ptr),
        entry!(0x005e0680, fn_005e0680(Ptr<NiTArray>, u32) -> Ptr),
        entry!(0x005e06b0, fn_005e06b0(Ptr, u32) -> Ptr),
        entry!(0x005e06e0, fn_005e06e0(Ptr<NiTArray>, u32) -> Ptr),
        entry!(0x005e0710, fn_005e0710(Ptr, u32) -> Ptr),
        entry!(0x005e0740, fn_005e0740(Ptr<NiTArray>, u32) -> Ptr),
        entry!(0x005e0770, fn_005e0770(Ptr, u32) -> Ptr),
        entry!(
            0x005e07a0,
            bs_simple_array_tes_object_refr_p_1024_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            ) -> Ptr
        ),
        entry!(
            0x005e07d0,
            bs_simple_array_nav_mesh_info_const_p_1024_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr
        ),
        entry!(
            0x005e0800,
            bs_simple_array_tesnpc_p_1024_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            ) -> Ptr
        ),
        entry!(
            0x005e08e0,
            fn_005e08e0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x005e0950,
            fn_005e0950(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x005e09c0,
            fn_005e09c0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x005e0aa0, fn_005e0aa0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// An engine that logs every call, with the given callees replaced by
    /// doubles that do nothing and return 0.
    fn engine(stubs: &[u32]) -> Engine {
        let mut e = Engine::new();
        for stub in stubs {
            e.register(*stub, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn log(e: &Engine) -> Log {
        e.call_log.clone().unwrap()
    }

    fn array(e: &mut Engine) -> Ptr<NiTArray> {
        e.new_object::<NiTArray>()
    }

    fn simple_array(e: &mut Engine) -> Ptr<BSSimpleArray> {
        e.new_object::<BSSimpleArray>()
    }

    /// A base constructor (`005e08e0`, `005e0950`, `005e09c0`): the fields
    /// it sets, the allocation when the capacity is positive and the null
    /// block when it is 0.
    fn check_base_constructor(
        construct: fn(&mut Engine, Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>,
        vtable: u32,
        allocate: u32,
    ) {
        let mut e = engine(&[]);
        e.register(allocate, |_, _| 0x5000u32.into_ret());
        let this = array(&mut e);
        e.set(this, NiTArray::m_usSize, 7);
        e.set(this, NiTArray::m_usESize, 7);
        assert_eq!(construct(&mut e, this, 8, 4), this);
        assert_eq!(e.mem.u32(this.addr()), vtable);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 8);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 4);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 0);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0x5000);
        assert_eq!(log(&e), vec![(allocate, vec![8])]);

        let mut e = engine(&[]);
        let this = array(&mut e);
        e.set(this, NiTArray::m_pBase, 0x1234);
        construct(&mut e, this, 0, 2);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 2);
        assert!(log(&e).is_empty());
    }

    /// A derived constructor: the base constructor's work, then its own
    /// vtable.
    fn check_derived_constructor(
        construct: fn(&mut Engine, Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>,
        vtable: u32,
        allocate: u32,
    ) {
        let mut e = engine(&[]);
        e.register(allocate, |_, _| 0x5000u32.into_ret());
        let this = array(&mut e);
        assert_eq!(construct(&mut e, this, 3, 1), this);
        assert_eq!(e.mem.u32(this.addr()), vtable);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 1);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0x5000);
        assert_eq!(log(&e), vec![(allocate, vec![3])]);
    }

    /// An array destructor: vtable stored, element block freed.
    fn check_array_destructor(destroy: fn(&mut Engine, Ptr<NiTArray>), vtable: u32) {
        let mut e = engine(&[FREE_ELEMENT_BLOCK]);
        let this = array(&mut e);
        e.set(this, NiTArray::m_pBase, 0x6000);
        destroy(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), vtable);
        assert_eq!(log(&e), vec![(FREE_ELEMENT_BLOCK, vec![0x6000])]);
    }

    fn check_simple_constructor(
        construct: fn(&mut Engine, Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>,
        vtable: u32,
    ) {
        let mut e = engine(&[SIMPLE_ARRAY_INIT]);
        let this = simple_array(&mut e);
        assert_eq!(construct(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), vtable);
        assert_eq!(log(&e), vec![(SIMPLE_ARRAY_INIT, vec![this.addr(), 0, 0])]);
    }

    fn check_simple_destructor(destroy: fn(&mut Engine, Ptr<BSSimpleArray>), vtable: u32) {
        let mut e = engine(&[SIMPLE_ARRAY_CLEAR]);
        let this = simple_array(&mut e);
        destroy(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), vtable);
        assert_eq!(log(&e), vec![(SIMPLE_ARRAY_CLEAR, vec![this.addr(), 1])]);
    }

    /// A scalar deleting destructor: `first` is the first callee it makes;
    /// the object is freed only when bit 0 of the flags is set.
    fn check_deleting(first: u32, run: impl Fn(&mut Engine, Ptr, u32) -> Ptr) {
        let stubs = [
            first,
            FREE_ELEMENT_BLOCK,
            SIMPLE_ARRAY_CLEAR,
            OPERATOR_DELETE,
        ];
        let mut e = engine(&stubs);
        let this = Ptr::new(e.mem.alloc(0x20));
        assert_eq!(run(&mut e, this, 1), this);
        let calls = log(&e);
        assert_eq!(calls.first().unwrap().0, first);
        assert_eq!(calls.last().unwrap(), &(OPERATOR_DELETE, vec![this.addr()]));

        let mut e = engine(&stubs);
        let this = Ptr::new(e.mem.alloc(0x20));
        assert_eq!(run(&mut e, this, 2), this);
        assert!(log(&e)
            .iter()
            .all(|(address, _)| *address != OPERATOR_DELETE));
        assert_eq!(log(&e).first().unwrap().0, first);
    }

    #[test]
    fn t005e0310_base_then_vtable() {
        check_derived_constructor(fn_005e0310, VTABLE_0103D3F0, ALLOCATE_POINTERS);
    }

    #[test]
    fn t005e0340_destructor() {
        check_array_destructor(fn_005e0340, VTABLE_0103D3F8);
    }

    #[test]
    fn t005e0370_base_then_vtable() {
        check_derived_constructor(fn_005e0370, VTABLE_0103D400, ALLOCATE_ELEMENTS);
    }

    #[test]
    fn t005e03a0_destructor() {
        check_array_destructor(fn_005e03a0, VTABLE_0103D408);
    }

    #[test]
    fn t005e03d0_remove_all() {
        let mut e = engine(&[]);
        let this = array(&mut e);
        let block = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(block + 4 * i, 0x100 + i);
        }
        e.set(this, NiTArray::m_pBase, block);
        e.set(this, NiTArray::m_usSize, 3);
        e.set(this, NiTArray::m_usESize, 4);
        fn_005e03d0(&mut e, this);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 0);
        // Only `m_usSize` slots are cleared.
        assert_eq!(e.mem.u32(block + 12), 0x103);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 0);

        // An empty array with no block touches nothing but the counts.
        let empty = array(&mut e);
        e.set(empty, NiTArray::m_usESize, 2);
        fn_005e03d0(&mut e, empty);
        assert_eq!(e.get(empty, NiTArray::m_usESize), 0);
    }

    #[test]
    fn t005e0430_base_then_vtable() {
        check_derived_constructor(fn_005e0430, VTABLE_0103D410, ALLOCATE_ELEMENTS);
    }

    #[test]
    fn t005e0460_destructor() {
        check_array_destructor(fn_005e0460, VTABLE_0103D418);
    }

    #[test]
    fn t005e0490_calls_the_other_unit_then_sets_the_vtable() {
        let mut e = engine(&[ARRAY_CONSTRUCTOR_005E0A30]);
        let this = array(&mut e);
        assert_eq!(fn_005e0490(&mut e, this, 6, 3), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_0103D420);
        assert_eq!(
            log(&e),
            vec![(ARRAY_CONSTRUCTOR_005E0A30, vec![this.addr(), 6, 3])]
        );
    }

    #[test]
    fn t005e04c0_simple_constructor() {
        check_simple_constructor(fn_005e04c0, VTABLE_0103D428);
    }

    #[test]
    fn t005e04f0_simple_destructor() {
        check_simple_destructor(fn_005e04f0, VTABLE_0103D428);
    }

    #[test]
    fn t005e0510_simple_constructor() {
        check_simple_constructor(fn_005e0510, VTABLE_0103D43C);
    }

    #[test]
    fn t005e0540_simple_destructor() {
        check_simple_destructor(fn_005e0540, VTABLE_0103D43C);
    }

    #[test]
    fn t005e0560_simple_constructor() {
        check_simple_constructor(fn_005e0560, VTABLE_0103D450);
    }

    #[test]
    fn t005e0590_simple_destructor() {
        check_simple_destructor(fn_005e0590, VTABLE_0103D450);
    }

    #[test]
    fn t005e05b0_block_address() {
        let mut e = engine(&[]);
        e.register(BLOCK_BASE, |_, _| 0x1000u32.into_ret());
        assert_eq!(fn_005e05b0(&mut e, Ptr::new(0x40), 3), 0x1000 + 0x600);
        assert_eq!(fn_005e05b0(&mut e, Ptr::new(0x40), 0), 0x1000);
        assert_eq!(log(&e)[0], (BLOCK_BASE, vec![0x40]));
    }

    #[test]
    fn t005e05d0_builds_on_a_pool_block() {
        let mut e = engine(&[BLOCK_OBJECT_INIT, UNLOADED_FORM_BUFFER_CONSTRUCTOR]);
        e.register(BLOCK_POOL, |_, _| 0x7000u32.into_ret());
        e.register(POOL_ALLOCATE, |_, _| 0x8000u32.into_ret());
        e.register(POOL_FINISH, |_, _| 0x9000u32.into_ret());
        let this = Ptr::new(0x40);
        assert_eq!(fn_005e05d0(&mut e, this, 2, 0xdead), this);
        assert_eq!(
            log(&e),
            vec![
                (BLOCK_OBJECT_INIT, vec![0x40]),
                (BLOCK_POOL, vec![]),
                (POOL_ALLOCATE, vec![0x7000]),
                (POOL_FINISH, vec![0x8000, 0x400, 0]),
                (UNLOADED_FORM_BUFFER_CONSTRUCTOR, vec![0x40, 0x9000]),
            ]
        );
    }

    #[test]
    fn t005e0600_releases_the_first_word() {
        let mut e = engine(&[RELEASE_BLOCK]);
        e.register(READ_FIRST_WORD, |_, _| 0xabcdu32.into_ret());
        fn_005e0600(&mut e, Ptr::new(0x40));
        assert_eq!(
            log(&e),
            vec![(READ_FIRST_WORD, vec![0x40]), (RELEASE_BLOCK, vec![0xabcd])]
        );
    }

    #[test]
    fn t005e0620_deleting_destructor() {
        check_deleting(DESTRUCTOR_005E02E0, fn_005e0620);
    }

    #[test]
    fn t005e0650_deleting_destructor() {
        check_deleting(DESTRUCTOR_005DD810, fn_005e0650);
    }

    #[test]
    fn t005e0680_deleting_destructor() {
        check_deleting(FREE_ELEMENT_BLOCK, |e, this, flags| {
            fn_005e0680(e, this.cast(), flags)
        });
    }

    #[test]
    fn t005e06b0_deleting_destructor() {
        check_deleting(DESTRUCTOR_005DD7B0, fn_005e06b0);
    }

    #[test]
    fn t005e06e0_deleting_destructor() {
        check_deleting(FREE_ELEMENT_BLOCK, |e, this, flags| {
            fn_005e06e0(e, this.cast(), flags)
        });
    }

    #[test]
    fn t005e0710_deleting_destructor() {
        check_deleting(DESTRUCTOR_005DD7D0, fn_005e0710);
    }

    #[test]
    fn t005e0740_deleting_destructor() {
        check_deleting(FREE_ELEMENT_BLOCK, |e, this, flags| {
            fn_005e0740(e, this.cast(), flags)
        });
    }

    #[test]
    fn t005e0770_deleting_destructor() {
        check_deleting(DESTRUCTOR_005DD7F0, fn_005e0770);
    }

    #[test]
    fn t005e07a0_deleting_destructor() {
        check_deleting(SIMPLE_ARRAY_CLEAR, |e, this, flags| {
            bs_simple_array_tes_object_refr_p_1024_scalar_deleting_destructor(e, this.cast(), flags)
        });
    }

    #[test]
    fn t005e07d0_deleting_destructor() {
        check_deleting(SIMPLE_ARRAY_CLEAR, |e, this, flags| {
            bs_simple_array_nav_mesh_info_const_p_1024_scalar_deleting_destructor(
                e,
                this.cast(),
                flags,
            )
        });
    }

    #[test]
    fn t005e0800_deleting_destructor() {
        check_deleting(SIMPLE_ARRAY_CLEAR, |e, this, flags| {
            bs_simple_array_tesnpc_p_1024_scalar_deleting_destructor(e, this.cast(), flags)
        });
    }

    #[test]
    fn t005e08e0_pointer_array_constructor() {
        check_base_constructor(fn_005e08e0, VTABLE_0103D3E8, ALLOCATE_POINTERS);
    }

    #[test]
    fn t005e0950_array_constructor() {
        check_base_constructor(fn_005e0950, VTABLE_0103D3F8, ALLOCATE_ELEMENTS);
    }

    #[test]
    fn t005e09c0_array_constructor() {
        check_base_constructor(fn_005e09c0, VTABLE_0103D408, ALLOCATE_ELEMENTS);
    }

    #[test]
    fn t005e0aa0_pool_block() {
        let mut e = engine(&[]);
        e.register(BLOCK_POOL, |_, _| 0x7000u32.into_ret());
        e.register(POOL_ALLOCATE, |_, _| 0x8000u32.into_ret());
        e.register(POOL_FINISH, |_, _| 0x9000u32.into_ret());
        assert_eq!(fn_005e0aa0(&mut e, 3, 5), 0x9000);
        assert_eq!(
            log(&e),
            vec![
                (BLOCK_POOL, vec![]),
                (POOL_ALLOCATE, vec![0x7000]),
                (POOL_FINISH, vec![0x8000, 0x600, 5]),
            ]
        );
    }

    #[test]
    fn registered_in_funcs() {
        let table = funcs();
        assert_eq!(table.len(), 32);
        assert!(table.iter().any(|(address, _)| *address == 0x005e0aa0));
    }
}
