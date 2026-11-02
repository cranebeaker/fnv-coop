//! `fallout shared/pathfinding/navmeshpovsearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3B0: u32 = 0x0106_b3b0;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3D4: u32 = 0x0106_b3d4;
/// `0049faf0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_0049FAF0: u32 = 0x0049_faf0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3E8: u32 = 0x0106_b3e8;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchLink@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3FC: u32 = 0x0106_b3fc;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B410: u32 = 0x0106_b410;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@URemoveIndex@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B424: u32 = 0x0106_b424;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B44C: u32 = 0x0106_b44c;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B460: u32 = 0x0106_b460;

// Translated from 006a14e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a14e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3B0);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a1700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `0049faf0(this, 1)`.
pub fn fn_006a1700(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3D4);
    e.call(ARRAY_RELEASE_0049FAF0, &args![this, 1u32]);
}

// Translated from 006a1890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a1890(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a18e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchLink@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a18e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3FC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `0049faf0(this, 1)`.
pub fn fn_006a19e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B410);
    e.call(ARRAY_RELEASE_0049FAF0, &args![this, 1u32]);
}

// Translated from 006a1a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@URemoveIndex@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a1a30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B424);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a2580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a2580(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B44C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a2900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a2900(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B460);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `006240d0` (unnamed in the engine map): the constructor `006a0480` runs on `this`.
const BASE_CTOR_006240D0: u32 = 0x0062_40d0;
/// `006815c0` (unnamed in the engine map): returns its `this` (here run as
/// the constructor of a member with nothing to construct).
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `ScrapHeap::Allocate` (`00aa54a0`, engine map; `thiscall`, `RET 8`: size, alignment).
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
/// The alignment word `006a1f90` passes to the scrap heap.
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `00558dd0` (unnamed in the engine map; `thiscall`, `RET 4`).
const STEP_00558DD0: u32 = 0x0055_8dd0;
/// `00aa5410` (unnamed in the engine map; `thiscall`, `RET 4`): the set-up
/// `006a1f70` runs with 0xD000.
const SCRAP_INIT_00AA5410: u32 = 0x00aa_5410;
/// `0042f5d0` (unnamed in the engine map; cdecl, a pointer and a byte count).
const BYTES_0042F5D0: u32 = 0x0042_f5d0;
/// `004b5570` (unnamed in the engine map; cdecl, two floats): the angle of
/// a 2D vector, called with (y, x); ST0.
const ANGLE_004B5570: u32 = 0x004b_5570;
/// `00401050` (unnamed in the engine map; `RET 0x10`: first, element size,
/// count, constructor): runs a constructor over an array.
const CONSTRUCT_ARRAY_00401050: u32 = 0x0040_1050;
/// `00588c50`, the element constructor `006a0870` passes to `00401050`.
const ELEMENT_CTOR_00588C50: u32 = 0x0058_8c50;
/// `004e4470` (unnamed in the engine map; cdecl, a float): ST0 (the first
/// coordinate of the direction `006a04e0` builds).
const DIRECTION_X_004E4470: u32 = 0x004e_4470;
/// `004e44b0` (unnamed in the engine map; cdecl, a float): ST0 (the second
/// coordinate).
const DIRECTION_Y_004E44B0: u32 = 0x004e_44b0;
/// `006985f0` (unnamed in the engine map; `thiscall`, `RET 4`): copies the
/// 2D point it is given into `this`.
const POINT_SET_006985F0: u32 = 0x0069_85f0;
/// `00698620` (unnamed in the engine map; `thiscall`, `RET 8`: out, other):
/// the 2D difference `this - other` into `out`.
const POINT_MINUS_00698620: u32 = 0x0069_8620;
/// `00698680` (unnamed in the engine map): the quadrant of a 2D vector (an int).
const POINT_QUADRANT_00698680: u32 = 0x0069_8680;
/// `006c4350` (unnamed in the engine map; `thiscall`, `RET 4`): the base
/// part `006a0c10` and `006a0e50` construct first.
const BASE_SET_006C4350: u32 = 0x006c_4350;
/// `0.0` (double).
const ZERO: u32 = 0x0101_2060;
/// `2 pi` as the float closest to it, stored as a double.
const TWO_PI: u32 = 0x0101_ff48;
/// The `BSSimpleArray` slot (byte offset) that allocates a buffer for a count.
const SLOT_ALLOCATE: u32 = 0x04;
/// The `BSSimpleArray` slot (byte offset) that reallocates a buffer (old buffer, count).
const SLOT_REALLOCATE: u32 = 0x0c;
/// `00438b90` (unnamed in the engine map): whether the array is full.
const ARRAY_FULL_00438B90: u32 = 0x0043_8b90;
/// `009a3910` (unnamed in the engine map): the array's grown capacity.
const ARRAY_GROWN_CAPACITY_009A3910: u32 = 0x009a_3910;
/// `006a8500` (unnamed in the engine map): frees the array's buffer.
const ARRAY_FREE_BUFFER_006A8500: u32 = 0x006a_8500;
/// `00692480` (unnamed in the engine map; `thiscall`, `RET 0xC`: new, old,
/// count): moves elements to a new buffer.
const MOVE_ELEMENTS_00692480: u32 = 0x0069_2480;
/// `memmove` (`00ec7230`, cdecl).
const MEMMOVE: u32 = 0x00ec_7230;
/// `006e6da0` (unnamed in the engine map; cdecl, size and place): placement
/// `new` (returns the place).
const PLACEMENT_NEW_006E6DA0: u32 = 0x006e_6da0;
/// `006b1d80` (unnamed in the engine map): the 0x40-byte element constructor.
const ELEMENT_CTOR_006B1D80: u32 = 0x006b_1d80;
/// `006e8f80` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_006E8F80: u32 = 0x006e_8f80;
/// `006e2280` (unnamed in the engine map; `thiscall`, `RET 8`): the
/// 0x3C-byte element constructor with two words.
const NODE_CTOR_006E2280: u32 = 0x006e_2280;

// Translated from 006a1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the address of element `index` of a
/// `BSSimpleArray` of 12-byte elements (buffer at +4).
pub fn fn_006a1440(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0xc)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

// Translated from 006a1490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the address of element `index` of a
/// `BSSimpleArray` of 0x24-byte elements (buffer at +4).
pub fn fn_006a1490(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x24)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

// Translated from 006a1400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): forwards to `006a1490`.
pub fn fn_006a1400(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    fn_006a1490(e, this, index)
}

// Translated from 006a1b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at +0x54.
pub fn fn_006a1b60(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x54)
}

// Translated from 006a0480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor: `006240d0` on `this`, then
/// `006815c0` on +0xC. Returns `this`.
pub fn fn_006a0480(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CTOR_006240D0, &args![this]);
    e.call(IDENTITY_006815C0, &args![this.addr() + 0xc]);
    this
}

// Translated from 006a1f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): `ScrapHeap::Allocate` on
/// `heap` of `size` bytes with the alignment word at `010a2720`.
pub fn fn_006a1f90(e: &mut Engine, size: u32, heap: Ptr) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    e.call(SCRAP_HEAP_ALLOCATE, &args![heap, size, alignment])
        .u32()
}

// Translated from 0069ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at +0x68 (ST0, `FLD`).
pub fn fn_0069ef80(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr() + 0x68))
}

// Translated from 0069e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: what `00559450` returns for +0x94.
pub fn fn_0069e390(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIRST_OF_00559450, &args![this.addr() + 0x94]).u32()
}

// Translated from 006a1420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00558dd0(this, 0)`, its result returned.
pub fn fn_006a1420(e: &mut Engine, this: Ptr) -> u32 {
    e.call(STEP_00558DD0, &args![this, 0u32]).u32()
}

// Translated from 006a1f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00aa5410(this, 0xD000)`. Returns `this`.
pub fn fn_006a1f70(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SCRAP_INIT_00AA5410, &args![this, 0xd000u32]);
    this
}

// Translated from 0069f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): `a + b` (wrapping), `this` unread.
pub fn fn_0069f900(_e: &mut Engine, _this: Ptr, a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

// Translated from 006a28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`; `this` unread): `0042f5d0(first,
/// count * 8)`, its result returned.
pub fn fn_006a28b0(e: &mut Engine, _this: Ptr, first: u32, count: u32) -> u32 {
    e.call(BYTES_0042F5D0, &args![first, count << 3]).u32()
}

// Translated from 006a0840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the angle of the 2D vector at `this`:
/// `004b5570(y, x)` with both floats copied through the FPU (ST0).
pub fn fn_006a0840(e: &mut Engine, this: Ptr) -> f64 {
    let x = x87_quiet_bits(e.mem.u32(this.addr()));
    let y = x87_quiet_bits(e.mem.u32(this.addr() + 4));
    e.call(ANGLE_004B5570, &args![y, x]).f64()
}

// Translated from 006a0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs two 0x14-byte elements at +4
/// (`00401050` with the constructor `00588c50`). Returns `this`.
pub fn fn_006a0870(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        CONSTRUCT_ARRAY_00401050,
        &args![this.addr() + 4, 0x14u32, 2u32, ELEMENT_CTOR_00588C50],
    );
    this
}

// Translated from 006a04b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): copies the three words at `point`
/// to `this` and the float `value` (through the FPU) to +0xC. Returns `this`.
pub fn fn_006a04b0(e: &mut Engine, this: Ptr, point: Ptr, value: f32) -> Ptr {
    for i in 0..3 {
        let word = e.mem.u32(point.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 4 * i, word);
    }
    e.mem
        .set_u32(this.addr() + 0xc, x87_quiet_bits(value.to_bits()));
    this
}

// Translated from 006a04e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the 2D direction of `angle`: +0 from
/// `004e4470(angle)`, +4 from `004e44b0(angle)` (each ST0 stored as a
/// float). Returns `this`.
pub fn fn_006a04e0(e: &mut Engine, this: Ptr, angle: f32) -> Ptr {
    let pushed = x87_quiet_bits(angle.to_bits());
    let x = e.call(DIRECTION_X_004E4470, &args![pushed]).f64();
    e.mem.set_u32(this.addr(), x87_store32(x));
    let pushed = x87_quiet_bits(angle.to_bits());
    let y = e.call(DIRECTION_Y_004E44B0, &args![pushed]).f64();
    e.mem.set_u32(this.addr() + 4, x87_store32(y));
    this
}

// Translated from 006a05e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xC`): sets the point at +0 from
/// `first`, the point at +8 from `second` (`006985f0`) and the word at
/// +0x10 to `flag`. Returns `this`.
pub fn fn_006a05e0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, flag: u32) -> Ptr {
    e.call(POINT_SET_006985F0, &args![this, first]);
    e.call(POINT_SET_006985F0, &args![this.addr() + 8, second]);
    e.mem.set_u32(this.addr() + 0x10, flag);
    this
}

// Translated from 006a0620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the copy constructor of the
/// `006a05e0` object: both points (`006985f0`) and the word at +0x10.
/// Returns `this`.
pub fn fn_006a0620(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(POINT_SET_006985F0, &args![this, source]);
    e.call(
        POINT_SET_006985F0,
        &args![this.addr() + 8, source.addr() + 8],
    );
    let flag = e.mem.u32(source.addr() + 0x10);
    e.mem.set_u32(this.addr() + 0x10, flag);
    this
}

// Translated from 006a06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): orders two 2D vectors by angle.
/// With the quadrants (`00698680`, signed) of `this` and `other`: a lower
/// quadrant is true, a higher one false; in the same quadrant, for
/// quadrants 0 and 3 true when `other`'s second coordinate is not below
/// this one's (false when unordered), else true when it is not above it
/// (false when unordered).
pub fn fn_006a06f0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mine = e.call(POINT_QUADRANT_00698680, &args![this]).i32();
    let theirs = e.call(POINT_QUADRANT_00698680, &args![other]).i32();
    if mine < theirs {
        return true;
    }
    if mine > theirs {
        return false;
    }
    let this_y = x87_load32(e.mem.u32(this.addr() + 4));
    let other_y = x87_load32(e.mem.u32(other.addr() + 4));
    if mine == 0 || mine == 3 {
        other_y >= this_y
    } else {
        other_y <= this_y
    }
}

// Translated from 006a0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): with the segment's two points (+0
/// and +8; their roles swap when the word at +0x10 is non-zero), compares
/// `point - start` against `end - start` by angle (`006a06f0`); the
/// differences are `00698620` into two locals.
pub fn fn_006a0660(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let swapped = e.mem.u32(this.addr() + 0x10) != 0;
    let start = if swapped {
        this.addr() + 8
    } else {
        this.addr()
    };
    let end = if swapped {
        this.addr()
    } else {
        this.addr() + 8
    };
    // The edge difference (EBP-0x10) and the point difference (EBP-8).
    e.with_stack(0x10, |e, locals| {
        let edge = locals.addr();
        let offset = locals.addr() + 8;
        e.call(POINT_MINUS_00698620, &args![end, edge, start]);
        e.call(POINT_MINUS_00698620, &args![point, offset, start]);
        fn_006a06f0(e, Ptr::new(offset), Ptr::new(edge))
    })
}

// Translated from 006a07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the angle of the segment's
/// direction (`(end - start)` by `00698620` with +8 and +0, then
/// `006a0840`), stored as a float `a`, scaled: with the word at +0x10 clear,
/// `(a + 2pi) * scale` for a negative `a`, else `a * scale`; with it set,
/// `-a * scale` for a negative `a`, else `(2pi - a) * scale`. Each result
/// is stored as a float and returned (ST0).
pub fn fn_006a07a0(e: &mut Engine, this: Ptr, scale: f32) -> f64 {
    let scale = x87_m32(scale.to_bits());
    let angle_bits = e.with_stack(8, |e, local| {
        e.call(POINT_MINUS_00698620, &args![this.addr() + 8, local, this]);
        let angle = fn_006a0840(e, local);
        x87_store32(angle)
    });
    let angle = x87_load32(angle_bits);
    let negative = angle < e.global::<f64>(ZERO);
    let two_pi = e.global::<f64>(TWO_PI);
    let result = if e.mem.u32(this.addr() + 0x10) == 0 {
        if negative {
            x87_mul(x87_add(angle, two_pi), scale)
        } else {
            x87_mul(angle, scale)
        }
    } else if negative {
        x87_mul(-angle, scale)
    } else {
        x87_mul(x87_sub(two_pi, angle), scale)
    };
    x87_load32(x87_store32(result))
}

// Translated from 006a0c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): `006c4350(this, base)`, then the
/// three words at `point` copied to +0xC. Returns `this`.
pub fn fn_006a0c10(e: &mut Engine, this: Ptr, base: u32, point: Ptr) -> Ptr {
    e.call(BASE_SET_006C4350, &args![this, base]);
    for i in 0..3 {
        let word = e.mem.u32(point.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    this
}

// Translated from 006a0e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the copy constructor of the
/// `006a0c10` object: `006c4350(this, source)`, then the three words at
/// `source + 0xC`. Returns `this`.
pub fn fn_006a0e50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(BASE_SET_006C4350, &args![this, source]);
    for i in 0..3 {
        let word = e.mem.u32(source.addr() + 0xc + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    this
}

// Translated from 006a0e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the 2D rotation (complex product)
/// of `this` by `other` into `out`: `(x*ox - y*oy, y*ox + x*oy)`, each
/// coordinate computed on the FPU and stored as a float in a local (made
/// by `006815c0`), then copied by `006985f0`. Returns `out`.
pub fn fn_006a0e90(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    e.with_stack(8, |e, local| {
        e.call(IDENTITY_006815C0, &args![local]);
        let x = e.mem.u32(this.addr());
        let y = e.mem.u32(this.addr() + 4);
        let ox = e.mem.u32(other.addr());
        let oy = e.mem.u32(other.addr() + 4);
        let first = x87_mul(x87_load32(x), x87_m32(ox));
        let second = x87_mul(x87_load32(y), x87_m32(oy));
        e.mem
            .set_u32(local.addr(), x87_store32(x87_sub(first, second)));
        let first = x87_mul(x87_load32(y), x87_m32(ox));
        let second = x87_mul(x87_load32(x), x87_m32(oy));
        e.mem
            .set_u32(local.addr() + 4, x87_store32(x87_add(first, second)));
        e.call(POINT_SET_006985F0, &args![out, local]);
    });
    out
}

// Translated from 006a1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`; `this` unread): unlinks `node` from
/// its list. When its +0x58 is set, the word at that address gets the
/// node's +0x5C; when +0x5C is set, that node's +0x58 gets this node's
/// +0x58; then both are cleared.
pub fn fn_006a1fb0(e: &mut Engine, _this: Ptr, node: Ptr) {
    let base = node.addr();
    let back = e.mem.u32(base + 0x58);
    if back != 0 {
        let next = e.mem.u32(base + 0x5c);
        e.mem.set_u32(back, next);
    }
    let next = e.mem.u32(base + 0x5c);
    if next != 0 {
        let back = e.mem.u32(base + 0x58);
        e.mem.set_u32(next + 0x58, back);
    }
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, 0);
}

// Translated from 006a2630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a2630 - its FS:[0] exception frame (the unwind state of each placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`): constructs `count` 0x40-byte
/// elements from `first`: placement `new` (`006e6da0(0x40, place)`) and,
/// when that gives a place, `006b1d80` on it.
pub fn fn_006a2630(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    let mut index: u32 = 0;
    while index < count {
        let place = (index << 6).wrapping_add(first);
        let element = e.call(PLACEMENT_NEW_006E6DA0, &args![0x40u32, place]).u32();
        if element != 0 {
            e.call(ELEMENT_CTOR_006B1D80, &args![element]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006a2130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the initialization of a
/// `BSSimpleArray` of 0x40-byte elements: clears buffer, size and
/// capacity; the reserve is raised to `size` when smaller (unsigned); a
/// non-zero reserve allocates through vtable slot +4 and sets the
/// capacity; a non-zero size constructs the elements (`006a2630`) and sets
/// the size.
pub fn fn_006a2130(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(base, SLOT_ALLOCATE, &args![reserve]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, reserve);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        fn_006a2630(e, this, buffer, size);
        e.mem.set_u32(base + 8, size);
    }
}

/// The shared body of `006a21c0` and `006a2770` (a `BSSimpleArray`'s
/// buffer change to `capacity`, keeping `count` elements, `mover` moving
/// them to a new buffer).
fn array_reserve(
    e: &mut Engine,
    this: Ptr,
    capacity: u32,
    count: u32,
    mover: fn(&mut Engine, Ptr, u32, u32, u32),
) {
    let base = this.addr();
    if e.mem.u32(base + 4) != 0 {
        if count == e.mem.u32(base + 0xc) {
            let old = e.mem.u32(base + 4);
            let buffer = e.vcall(base, SLOT_REALLOCATE, &args![old, capacity]).u32();
            e.mem.set_u32(base + 4, buffer);
        } else {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
            let old = e.mem.u32(base + 4);
            mover(e, this, buffer, old, count);
            e.call(ARRAY_FREE_BUFFER_006A8500, &args![this]);
            e.mem.set_u32(base + 4, buffer);
        }
    } else {
        let buffer = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    }
}

fn move_by_address(e: &mut Engine, this: Ptr, new: u32, old: u32, count: u32) {
    e.call(MOVE_ELEMENTS_00692480, &args![this, new, old, count]);
}

// Translated from 006a2270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xC`; `this` unread): moves `count`
/// 0x3C-byte elements from `source` to `destination` one `memmove` each:
/// forwards when the destination is below the source, backwards (a signed
/// index from `count - 1` down to 0) when above; nothing when equal or for
/// a zero count.
pub fn fn_006a2270(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index: u32 = 0;
        while index < count {
            let to = index.wrapping_mul(0x3c).wrapping_add(destination);
            let from = index.wrapping_mul(0x3c).wrapping_add(source);
            e.call(MEMMOVE, &args![to, from, 0x3cu32]);
            index = index.wrapping_add(1);
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let to = (index as u32).wrapping_mul(0x3c).wrapping_add(destination);
            let from = (index as u32).wrapping_mul(0x3c).wrapping_add(source);
            e.call(MEMMOVE, &args![to, from, 0x3cu32]);
            index = index.wrapping_sub(1);
        }
    }
}

// Translated from 006a21c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): changes the buffer of a
/// `BSSimpleArray` of 0x3C-byte elements to `capacity`. Without a buffer it
/// allocates one (vtable slot +4) and sets the capacity. With one: when
/// `count` equals the capacity it reallocates in place (slot +0xC with the
/// old buffer); otherwise it allocates a new buffer, moves the elements
/// (`006a2270`), frees the old buffer (`006a8500`) and keeps the new one
/// (the capacity is left to the caller).
pub fn fn_006a21c0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    array_reserve(e, this, capacity, count, fn_006a2270);
}

// Translated from 006a2770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): as `006a21c0`, with the elements
/// moved by `00692480`.
pub fn fn_006a2770(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    array_reserve(e, this, capacity, count, move_by_address);
}

// Translated from 006a1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reserves one more element of a `BSSimpleArray`
/// of 0x3C-byte elements. When full (`00438b90`): with no capacity, a
/// buffer of 4 (vtable slot +4); otherwise the grown capacity (`009a3910`)
/// through `006a21c0` with the current size. Then the size grows by one;
/// returns the new element's index.
pub fn fn_006a1920(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.call(ARRAY_FULL_00438B90, &args![this]).u8() != 0 {
        if e.mem.u32(base + 0xc) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, capacity);
        } else {
            let capacity = e.call(ARRAY_GROWN_CAPACITY_009A3910, &args![this]).u32();
            let count = e.mem.u32(base + 8);
            fn_006a21c0(e, this, capacity, count);
            e.mem.set_u32(base + 0xc, capacity);
        }
    }
    let size = e.mem.u32(base + 8).wrapping_add(1);
    e.mem.set_u32(base + 8, size);
    e.mem.u32(base + 8).wrapping_sub(1)
}

// Translated from 0069f920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069f920 - its FS:[0] exception frame (the unwind state of the placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`): appends a 0x3C-byte node to the
/// array at +0x20: reserves an index (`006a1920`), takes its element
/// (`006e8f80`), placement-`new`s it (`006e6da0(0x3C, place)`) and, when that
/// gives a place, constructs it with `first` and `second` (`006e2280`).
/// Returns the index.
pub fn fn_0069f920(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let array = Ptr::new(this.addr() + 0x20);
    let index = fn_006a1920(e, array);
    let place = e.call(ARRAY_AT_006E8F80, &args![array, index]).u32();
    let node = e.call(PLACEMENT_NEW_006E6DA0, &args![0x3cu32, place]).u32();
    if node != 0 {
        e.call(NODE_CTOR_006E2280, &args![node, first, second]);
    }
    index
}

/// `operator new` (`00401000`, cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00413f40` (unnamed in the engine map): returns `this + 8`.
const PLUS_EIGHT_00413F40: u32 = 0x0041_3f40;
/// `0044ddc0` (unnamed in the engine map): the word at +8 (a `BSSimpleArray`'s size).
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `00439e90` (unnamed in the engine map; `RET 8`: out, other): the 3D sum
/// `this + other` into `out`; returns `out`.
const POINT3_PLUS_00439E90: u32 = 0x0043_9e90;
/// `00439ef0` (unnamed in the engine map; `RET 8`: out, other): the 3D
/// difference `this - other` into `out`; returns `out`.
const POINT3_MINUS_00439EF0: u32 = 0x0043_9ef0;
/// `004a7290` (unnamed in the engine map): the squared length of the 3D
/// vector `this` (ST0, rounded to a float first).
const POINT3_LENGTH_SQUARED_004A7290: u32 = 0x004a_7290;
/// `00457990` (unnamed in the engine map): the length of the 3D vector
/// `this` (ST0).
const POINT3_LENGTH_00457990: u32 = 0x0045_7990;
/// `0069e0c0` (unnamed in the engine map; `RET 8`: out, scale): the 3D
/// point `(x * scale, y * scale, 0)` of the 2D vector `this` into `out`;
/// returns `out`.
const SCALED_POINT_0069E0C0: u32 = 0x0069_e0c0;
/// `00877a30` (unnamed in the engine map; `RET 4`): the address of an
/// array's element.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `00719b20` (unnamed in the engine map; `RET 0xC`: key, start, compare):
/// the index of the first element of a `BSSimpleArray` the compare function
/// matches, or -1.
const ARRAY_FIND_00719B20: u32 = 0x0071_9b20;
/// The compare function (`009a3830`) `006a0940` and `006a0ac0` pass to
/// `00719b20`.
const COMPARE_009A3830: u32 = 0x009a_3830;
/// `006cf900` (unnamed in the engine map; `RET 4`): copies the
/// triangle-and-point record it is given into `this`.
const RECORD_COPY_006CF900: u32 = 0x006c_f900;
/// `NavMesh::FindInterectingEdge2` (`00698160`, engine map; `RET 0x14`).
const FIND_INTERSECTING_EDGE_00698160: u32 = 0x0069_8160;
/// `0069a6c0` (unnamed in the engine map; `RET 0xC`): builds an edge
/// handle (navmesh, triangle, edge) into `this`.
const EDGE_HANDLE_0069A6C0: u32 = 0x0069_a6c0;
/// `NavMesh::GetMatchingEdge_ov4` (`0068f670`, engine map; `RET 8`).
const GET_MATCHING_EDGE_0068F670: u32 = 0x0068_f670;
/// `NavMesh::GetEdgeExtraInfo` (`0068f230`, engine map; `RET 8`).
const GET_EDGE_EXTRA_INFO_0068F230: u32 = 0x0068_f230;
/// `NavMesh::GetEdgeVertices` (`0068f040`, engine map; `RET 0xC`).
const GET_EDGE_VERTICES_0068F040: u32 = 0x0068_f040;
/// `00403e20` (unnamed in the engine map): the address of a setting's
/// value (`this` is the setting).
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
/// The setting `006a0ef0` reads through `00403e20`.
const SETTING_011D6EF0: u32 = 0x011d_6ef0;
/// `005582f0` (unnamed in the engine map; `RET 4`): the triangle record of
/// a navmesh.
const TRIANGLE_005582F0: u32 = 0x0055_82f0;
/// `00693b90` (unnamed in the engine map): a flag of a navmesh triangle
/// (AL).
const TRIANGLE_FLAG_00693B90: u32 = 0x0069_3b90;
/// `0045bb20` (unnamed in the engine map; `RET 8`: out, scale): the 3D
/// product `this * scale` into `out`; returns `out`.
const POINT3_SCALE_0045BB20: u32 = 0x0045_bb20;
/// `006ba550` (unnamed in the engine map; cdecl, five words): a point on a
/// segment written to its first argument; returns it.
const SEGMENT_POINT_006BA550: u32 = 0x006b_a550;
/// `0063c8a0` (unnamed in the engine map; `RET 4`): adds a 3D vector to
/// `this`.
const POINT3_ADD_0063C8A0: u32 = 0x0063_c8a0;
/// `006bc870` (unnamed in the engine map; `RET 4`): moves a triangle handle
/// to the one it is given.
const HANDLE_SET_006BC870: u32 = 0x006b_c870;
/// `004a0c10` (unnamed in the engine map): run on the segment's direction
/// before the walk.
const DIRECTION_STEP_004A0C10: u32 = 0x004a_0c10;
/// `006bb8b0` (unnamed in the engine map; cdecl, three pointers).
const CIRCLE_TEST_006BB8B0: u32 = 0x006b_b8b0;
/// `006bab00` (unnamed in the engine map; cdecl: center, radius, two
/// pointers).
const CIRCLE_TEST_006BAB00: u32 = 0x006b_ab00;
/// The two arc steps (floats): `01064f64` for an arc with the flag clear,
/// `0106b3a8` with it set.
const ARC_STEP: u32 = 0x0106_4f64;
const ARC_STEP_REVERSED: u32 = 0x0106_b3a8;
/// The costs `006a0940`/`006a0ac0` return for a blocking POV of type 5
/// (`010162c0`, also the cost factor `0069e3b0` gives an edge vertex's
/// second POV) and of any other type (`01016410`), floats.
const BLOCKED_COST_TYPE_5: u32 = 0x0101_62c0;
const BLOCKED_COST: u32 = 0x0101_6410;
/// The `double` `006a08d0` scales the distance to the goal by.
const GOAL_DISTANCE_SCALE: u32 = 0x0101_de30;
/// The scale (float) `006a0ef0` gives the edge's two vertices' sum.
const EDGE_MIDPOINT_SCALE: u32 = 0x0101_6248;
/// The scale (float) `006a0ef0` gives the pull towards the edge's middle.
const EDGE_PULL_SCALE: u32 = 0x0101_3ea4;
/// The number of steps an arc walk takes at most.
const ARC_STEPS: u32 = 0x18;
/// `004b3890` (unnamed in the engine map; cdecl, five words): builds a
/// debug line between two points with their colours.
const MAKE_LINE_004B3890: u32 = 0x004b_3890;
/// `00633c90` (unnamed in the engine map; `RET 4`): a smart pointer set to
/// the object it is given.
const POINTER_SET_00633C90: u32 = 0x0063_3c90;
/// `0045cec0` (unnamed in the engine map): the smart pointer's destructor.
const POINTER_RELEASE_0045CEC0: u32 = 0x0045_cec0;
/// `0043d410` (unnamed in the engine map; `RET 0xC`): a three-word
/// constructor.
const THREE_WORD_CTOR_0043D410: u32 = 0x0043_d410;
/// `NiAVObject::UpdateProperties` (`00a5a040`, engine map), run on the new
/// line.
const UPDATE_PROPERTIES_00A5A040: u32 = 0x00a5_a040;
/// `00a59c60` (unnamed in the engine map; `RET 4`), run on an object with
/// a record of three zero words.
const ZERO_RECORD_STEP_00A59C60: u32 = 0x00a5_9c60;
/// The smart pointer the debug lines are attached to (`011d6ddc`).
const DEBUG_ROOT_011D6DDC: u32 = 0x011d_6ddc;
/// The vtable slot (byte offset) of the root node `006a1330` attaches the
/// line through.
const SLOT_ATTACH: u32 = 0xdc;
/// The vtable of `NavMeshPOVSearch` (RTTI `.?AVNavMeshPOVSearch@@`).
const POV_SEARCH_VTABLE: u32 = 0x0106_b39c;
/// The vtable of `AStarSearch<NavMeshPOVSearchNode,NavMeshPOVSearchLink>`
/// (RTTI `.?AV?$AStarSearch@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@`).
const ASTAR_SEARCH_VTABLE: u32 = 0x0106_b3c4;
/// The vtable of the `BSScrapArray` of `AStarNode` pointers (RTTI
/// `.?AV?$BSScrapArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@@`).
const SCRAP_ARRAY_VTABLE_0106B438: u32 = 0x0106_b438;
/// `004ff7e0` (unnamed in the engine map): the `PathingLocation`
/// destructor body (stores its vtable).
const LOCATION_DTOR_004FF7E0: u32 = 0x004f_f7e0;
/// `006b3eb0`, `006b3f40`, `006dc440` (unnamed in the engine map; `RET 8`:
/// reserve, size): the initializations of other `BSSimpleArray` instances.
const ARRAY_INIT_006B3EB0: u32 = 0x006b_3eb0;
const ARRAY_INIT_006B3F40: u32 = 0x006b_3f40;
const ARRAY_INIT_006DC440: u32 = 0x006d_c440;
/// `006b3de0`, `006b9470` (unnamed in the engine map): the constructors of
/// the search's node map (+0x14) and queue (+0x1FF4).
const NODE_MAP_CTOR_006B3DE0: u32 = 0x006b_3de0;
const QUEUE_CTOR_006B9470: u32 = 0x006b_9470;
/// `00483710` (unnamed in the engine map): an empty destructor body.
const EMPTY_DTOR_00483710: u32 = 0x0048_3710;
/// `00982610` (unnamed in the engine map): the node heap's destructor.
const NODE_HEAP_DTOR_00982610: u32 = 0x0098_2610;
/// `00b00a00` (unnamed in the engine map; no arguments): the object
/// `006a1630` asks (vtable slot +0xC) whether it runs on a tasklet.
const TASKLET_MANAGER_00B00A00: u32 = 0x00b0_0a00;
/// `0040fc90` (unnamed in the engine map; no arguments): the word
/// `006a1630` passes to that slot.
const THREAD_ID_0040FC90: u32 = 0x0040_fc90;
/// `0040fca0` (unnamed in the engine map; cdecl, one word): run with 0
/// every 50 search steps on a tasklet.
const YIELD_0040FCA0: u32 = 0x0040_fca0;
/// `006b3cb0` (unnamed in the engine map; `RET 0xC`: node, two words): the
/// path build from the found node.
const BUILD_PATH_006B3CB0: u32 = 0x006b_3cb0;
/// `006b4350` (unnamed in the engine map): takes the next node out of the
/// search queue (0 when empty).
const QUEUE_POP_006B4350: u32 = 0x006b_4350;
/// `006b3e10` (unnamed in the engine map; `RET 4`): puts a node into the
/// search queue.
const QUEUE_PUSH_006B3E10: u32 = 0x006b_3e10;
/// `006b9130` (unnamed in the engine map): a node's cost (ST0).
const NODE_COST_006B9130: u32 = 0x006b_9130;
/// `009f6540` (unnamed in the engine map; `RET 8`: capacity, count): the
/// buffer change of a `BSSimpleArray` of 0x10-byte elements.
const ARRAY_RESIZE_009F6540: u32 = 0x009f_6540;
/// `0049fc80` (unnamed in the engine map; `RET 8`: first, count): destroys
/// 0x10-byte elements.
const ARRAY_DESTROY_0049FC80: u32 = 0x0049_fc80;
/// `00845460` (unnamed in the engine map): reserves one element of a
/// `BSSimpleArray`; returns its index.
const ARRAY_RESERVE_00845460: u32 = 0x0084_5460;
/// `006d4ca0` (unnamed in the engine map; `RET 4`): the compare `006a1b80`
/// forwards to.
const COMPARE_006D4CA0: u32 = 0x006d_4ca0;
/// `00401020` (unnamed in the engine map): the scrap-heap holder.
const SCRAP_HOLDER_00401020: u32 = 0x0040_1020;
/// `00aa42e0` (unnamed in the engine map): the scrap heap of the holder.
const SCRAP_HEAP_00AA42E0: u32 = 0x00aa_42e0;
/// The globals `0069e2c0` copies into its +4 point.
const ORIGIN_011F426C: u32 = 0x011f_426c;

/// `AStarSearch` fields (Xbox PDB names, PC offsets: 4 lower than the
/// Xbox's from the node map on).
const SEARCH_NODE_MAP: u32 = 0x14;
const SEARCH_QUEUE: u32 = 0x1ff4;
const BEST_NODE_SO_FAR: u32 = 0x2048;
const LAST_SEARCHED_NODE: u32 = 0x204c;
const START_ASTAR_NODE: u32 = 0x2050;
const RUNNING_IN_TASKLET: u32 = 0x2054;
/// `NavMeshPOVSearch` fields (Xbox PDB names, PC offsets).
const ALL_POVS: u32 = 0x2058;
const POVS: u32 = 0x2068;
const GOAL_POV: u32 = 0x207c;
const REQUEST: u32 = 0x2080;
const START_LOCATION: u32 = 0x2088;
const GOAL_LOCATION: u32 = 0x20b0;
const NODES: u32 = 0x20d8;
const LINKS: u32 = 0x20e8;
/// `AStarNode::pParentAStarNode` (Xbox PDB) +0x54.
const PARENT_ASTAR_NODE: u32 = 0x54;

/// Runs `body` on a block standing in for the function's stack frame:
/// `locals` bytes below EBP, the saved EBP and the return address, then the
/// argument words. `body` gets EBP's address, so the frame's objects sit at
/// the offsets the exe uses and arguments it passes by address are in
/// memory.
fn with_frame<R>(
    e: &mut Engine,
    locals: u32,
    words: &[u32],
    body: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    let size = locals + 8 + 4 * words.len() as u32;
    e.with_stack(size, |e, block| {
        let ebp = block.addr() + locals;
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(ebp + 8 + 4 * i as u32, *word);
        }
        body(e, ebp)
    })
}

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// A scalar deleting destructor's tail: `operator delete` on `this` when
/// bit 0 of `flags` is set; returns `this`.
fn delete_if(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0069ef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the low 14 bits of the 16-bit word at `this`.
pub fn fn_0069ef40(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr())) & 0x3fff
}

// Translated from 0069ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the top two bits of the 16-bit word at `this`
/// (0 to 3).
pub fn fn_0069ef60(e: &mut Engine, this: Ptr) -> u32 {
    (u32::from(e.mem.u16(this.addr())) & 0xc000) >> 0xe
}

// Translated from 0069efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the object at +0x30 is set, its scalar
/// deleting destructor (vtable slot 0) with 1.
pub fn fn_0069efd0(e: &mut Engine, this: Ptr) {
    let member = e.mem.u32(this.addr() + 0x30);
    if member != 0 {
        e.vcall(member, 0, &args![1u32]);
    }
}

// Translated from 0069efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of the
/// `0069e2c0` object: `0069efd0`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0069efa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069efd0(e, this);
    delete_if(e, this, flags)
}

// Translated from 006a15b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a15b0 - its FS:[0] exception frame (the unwind state of the member destructors) is not translated
/// The destructor body of `AStarSearch<NavMeshPOVSearchNode,NavMeshPOVSearchLink>`
/// (by the vtable it stores): the queue (+0x1FF4) and node map (+0x14)
/// destructors (`00483710`), then the node heap's (`00982610` on +4).
pub fn fn_006a15b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, ASTAR_SEARCH_VTABLE);
    e.call(EMPTY_DTOR_00483710, &args![base + SEARCH_QUEUE]);
    e.call(EMPTY_DTOR_00483710, &args![base + SEARCH_NODE_MAP]);
    e.call(NODE_HEAP_DTOR_00982610, &args![base + 4]);
}

// Translated from 0069e200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069e200 - its FS:[0] exception frame (the unwind state of the member destructors) is not translated
/// The destructor body of `NavMeshPOVSearch` (by the vtable it stores):
/// `Links` (+0x20E8, `006a18e0`), `Nodes` (+0x20D8, `006a1890`), the goal
/// and start `PathingLocation`s (+0x20B0, +0x2088, `004ff7e0`), `POVs`
/// (+0x2068, `006a1700`), `allPOVs` (+0x2058, `006a14e0`), then the
/// `AStarSearch` base (`006a15b0`).
pub fn fn_0069e200(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, POV_SEARCH_VTABLE);
    fn_006a18e0(e, Ptr::new(base + LINKS));
    fn_006a1890(e, Ptr::new(base + NODES));
    e.call(LOCATION_DTOR_004FF7E0, &args![base + GOAL_LOCATION]);
    e.call(LOCATION_DTOR_004FF7E0, &args![base + START_LOCATION]);
    fn_006a1700(e, Ptr::new(base + POVS));
    fn_006a14e0(e, Ptr::new(base + ALL_POVS));
    fn_006a15b0(e, this);
}

// Translated from 0069e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshPOVSearch::scalar deleting destructor` (Xbox PDB): `0069e200`,
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn navmesh_pov_search_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069e200(e, this);
    delete_if(e, this, flags)
}

// Translated from 006a14b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<NavMeshPOV*,1024>` (by the vtable it
/// stores): the vtable, then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_006a14b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3B0);
    e.call(ARRAY_INIT_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0069e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069e2c0 - its FS:[0] exception frame (the unwind state of the array construction) is not translated
/// Unnamed in the engine map: a constructor (the `0069efa0` object, 0x34
/// bytes): -1 at +0, the three words at `011f426c` at +4, 0.0 at +0x10,
/// `006815c0` on +0x14 and +0x20, 1.0 at +0x2C, and at +0x30 a new
/// `BSSimpleArray<NavMeshPOV*,1024>` (`operator new(0x10)`, `006a14b0`;
/// null when the allocation fails). Returns `this`.
pub fn fn_0069e2c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0xffff_ffff);
    copy_words(e, base + 4, ORIGIN_011F426C, 3);
    e.mem.set_u32(base + 0x10, x87_store32(0.0));
    e.call(IDENTITY_006815C0, &args![base + 0x14]);
    e.call(IDENTITY_006815C0, &args![base + 0x20]);
    e.mem.set_u32(base + 0x2c, x87_store32(1.0));
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let array = if block != 0 {
        fn_006a14b0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(base + 0x30, array);
    this
}

// Translated from 006a0520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): rotates the 2D vector `this` by
/// `other` in place (complex product): `(x*ox - y*oy, y*ox + x*oy)`, each
/// coordinate on the FPU from float copies of `x` and `y`. Returns `this`.
pub fn fn_006a0520(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let base = this.addr();
    let x = x87_quiet_bits(e.mem.u32(base));
    let y = x87_quiet_bits(e.mem.u32(base + 4));
    let ox = e.mem.u32(other.addr());
    let oy = e.mem.u32(other.addr() + 4);
    let first = x87_mul(x87_load32(x), x87_m32(ox));
    let second = x87_mul(x87_load32(y), x87_m32(oy));
    e.mem.set_u32(base, x87_store32(x87_sub(first, second)));
    let first = x87_mul(x87_load32(y), x87_m32(ox));
    let second = x87_mul(x87_load32(x), x87_m32(oy));
    e.mem.set_u32(base + 4, x87_store32(x87_add(first, second)));
    this
}

// Translated from 006a0580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): rotates the 2D vector `this` back
/// by `other` in place (product with the conjugate): `(x*ox + y*oy,
/// y*ox - x*oy)`. Returns `this`.
pub fn fn_006a0580(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let base = this.addr();
    let x = x87_quiet_bits(e.mem.u32(base));
    let y = x87_quiet_bits(e.mem.u32(base + 4));
    let ox = e.mem.u32(other.addr());
    let oy = e.mem.u32(other.addr() + 4);
    let first = x87_mul(x87_load32(x), x87_m32(ox));
    let second = x87_mul(x87_load32(y), x87_m32(oy));
    e.mem.set_u32(base, x87_store32(x87_add(first, second)));
    let first = x87_mul(x87_load32(y), x87_m32(ox));
    let second = x87_mul(x87_load32(x), x87_m32(oy));
    e.mem.set_u32(base + 4, x87_store32(x87_sub(first, second)));
    this
}

// Translated from 006a08a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshPOVSearch::IsGoalNode` (Xbox PDB; `RET 4`): whether the POV of
/// `node`'s data (`00413f40` gives `node + 8`, the `NavMeshPOVSearchNode`,
/// whose first word is `pPOV`) has type 4.
pub fn navmesh_pov_search_is_goal_node(e: &mut Engine, _this: Ptr, node: Ptr) -> bool {
    let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
    let pov = e.mem.u32(data);
    e.mem.u32(pov) == 4
}

// Translated from 006a08d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x10`): sets a search node's data
/// (`00413f40`, `node + 8`) to `pov`, `direction` and `index`, and the
/// node's heuristic (+4) to the distance from the POV's center to the goal
/// POV's (`pGoalPov` +0x207C; `00439ef0`, `00457990`) times the double at
/// `0101de30`.
pub fn fn_006a08d0(e: &mut Engine, this: Ptr, node: Ptr, pov: u32, direction: u32, index: u32) {
    let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
    e.mem.set_u32(data, pov);
    let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
    e.mem.set_u32(data + 4, direction);
    let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
    e.mem.set_u32(data + 8, index);
    let goal = e.mem.u32(this.addr() + GOAL_POV);
    let distance = e.with_stack(0xc, |e, local| {
        let difference = e
            .call(POINT3_MINUS_00439EF0, &args![pov + 4, local, goal + 4])
            .u32();
        e.call(POINT3_LENGTH_00457990, &args![difference]).f64()
    });
    let scale: f64 = e.global(GOAL_DISTANCE_SCALE);
    e.mem
        .set_u32(node.addr() + 4, x87_store32(x87_mul(distance, scale)));
}

/// `!(left > right)` as the exe's `FCOMPP; TEST AH,0x41; JNZ` takes it:
/// less, equal or unordered.
fn not_greater(left: f64, right: f64) -> bool {
    left.partial_cmp(&right) != Some(std::cmp::Ordering::Greater)
}

/// The shared walk of `006a0940` and `006a0ac0` over every POV of every
/// list in `POVs` (+0x2068): skips `exclude`, the node's own POV (`EBP+8`,
/// read from memory) and the POVs whose duplicates (`pDuplicates` +0x30)
/// contain it (`00719b20` with `009a3830`); `test` decides on the others and
/// gives the result for the first it accepts.
fn first_blocking_pov(
    e: &mut Engine,
    this: Ptr,
    node_slot: u32,
    exclude: u32,
    mut test: impl FnMut(&mut Engine, u32) -> Option<f64>,
) -> f64 {
    let lists = this.addr() + POVS;
    let mut list = 0u32;
    loop {
        let count = e.call(ARRAY_SIZE_0044DDC0, &args![lists]).u32();
        if list >= count {
            return 1.0;
        }
        let mut index = 0u32;
        loop {
            let inner = e.call(STEP_00558DD0, &args![lists, list]).u32();
            let size = e.call(ARRAY_SIZE_0044DDC0, &args![inner]).u32();
            if index >= size {
                break;
            }
            let inner = e.call(STEP_00558DD0, &args![lists, list]).u32();
            let slot = e.call(ARRAY_AT_00877A30, &args![inner, index]).u32();
            let pov = e.mem.u32(slot);
            if pov != exclude && pov != e.mem.u32(node_slot) {
                let duplicates = e.mem.u32(pov + 0x30);
                let found = e
                    .call(
                        ARRAY_FIND_00719B20,
                        &args![duplicates, node_slot, 0u32, COMPARE_009A3830],
                    )
                    .u32();
                if found == 0xffff_ffff {
                    if let Some(result) = test(e, pov) {
                        return result;
                    }
                }
            }
            index = index.wrapping_add(1);
        }
        list = list.wrapping_add(1);
    }
}

/// The cost of a blocking POV: the float at `010162c0` for type 5, else
/// the one at `01016410` (ST0).
fn blocked_cost(e: &mut Engine, pov: u32) -> f64 {
    let constant = if e.mem.u32(pov) == 5 {
        BLOCKED_COST_TYPE_5
    } else {
        BLOCKED_COST
    };
    x87_load32(e.global::<u32>(constant))
}

// Translated from 006a0940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x1C`: the node POV, the five words of
/// an arc by value, a POV to skip): the cost factor of the arc around `pov`
/// (ST0). The arc's end is `pov`'s center plus the arc's 2D direction
/// scaled by `pov`'s radius (`0069e0c0`, `00439e90`). Every other POV of
/// `POVs` (see `first_blocking_pov`) whose center is not nearer the end
/// than its radius (squared radius not above the squared distance, or
/// unordered: the exe's flag test) and for which `006bb8b0(circle, arc, other)` is false makes the
/// result its blocked cost; otherwise 1.0.
pub fn fn_006a0940(
    e: &mut Engine,
    this: Ptr,
    pov: u32,
    first_x: u32,
    first_y: u32,
    second_x: u32,
    second_y: u32,
    flag: u32,
    exclude: u32,
) -> f64 {
    let words = [pov, first_x, first_y, second_x, second_y, flag, exclude];
    with_frame(e, 0x54, &words, |e, ebp| {
        let circle = ebp - 0x10;
        let radius = x87_quiet_bits(e.mem.u32(pov + 0x10));
        fn_006a04b0(
            e,
            Ptr::new(circle),
            Ptr::new(pov + 4),
            f32::from_bits(radius),
        );
        let radius = x87_quiet_bits(e.mem.u32(ebp - 4));
        let offset = e
            .call(SCALED_POINT_0069E0C0, &args![ebp + 0xc, ebp - 0x44, radius])
            .u32();
        let end = ebp - 0x1c;
        e.call(POINT3_PLUS_00439E90, &args![circle, end, offset]);
        first_blocking_pov(e, this, ebp + 8, exclude, |e, other| {
            let other_circle = ebp - 0x38;
            let radius = x87_quiet_bits(e.mem.u32(other + 0x10));
            fn_006a04b0(
                e,
                Ptr::new(other_circle),
                Ptr::new(other + 4),
                f32::from_bits(radius),
            );
            let difference = e
                .call(POINT3_MINUS_00439EF0, &args![other_circle, ebp - 0x50, end])
                .u32();
            let distance = e
                .call(POINT3_LENGTH_SQUARED_004A7290, &args![difference])
                .f64();
            let radius = e.mem.u32(ebp - 0x2c);
            let squared = x87_mul(x87_load32(radius), x87_m32(radius));
            if not_greater(squared, distance)
                && e.call(
                    CIRCLE_TEST_006BB8B0,
                    &args![circle, ebp + 0xc, other_circle],
                )
                .u8()
                    == 0
            {
                return Some(blocked_cost(e, other));
            }
            None
        })
    })
}

// Translated from 006a0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x10`: the node POV, a point, a word, a
/// POV to skip): the cost factor of the segment to `point` (ST0). Every
/// other POV of `POVs` (see `first_blocking_pov`) whose center is not
/// nearer `point` than its radius (squared radius not above the squared
/// distance, or unordered)
/// and for which `006bab00(center, radius, point, extra)` is true makes the
/// result its blocked cost; otherwise 1.0.
pub fn fn_006a0ac0(
    e: &mut Engine,
    this: Ptr,
    pov: u32,
    point: u32,
    extra: u32,
    exclude: u32,
) -> f64 {
    with_frame(e, 0x1c, &[pov, point, extra, exclude], |e, ebp| {
        first_blocking_pov(e, this, ebp + 8, exclude, |e, other| {
            let difference = e
                .call(POINT3_MINUS_00439EF0, &args![other + 4, ebp - 0x18, point])
                .u32();
            let distance = e
                .call(POINT3_LENGTH_SQUARED_004A7290, &args![difference])
                .f64();
            let radius = e.mem.u32(other + 0x10);
            let squared = x87_mul(x87_load32(radius), x87_m32(radius));
            if not_greater(squared, distance)
                && e.call(
                    CIRCLE_TEST_006BAB00,
                    &args![other + 4, x87_quiet_bits(radius), point, extra],
                )
                .u8()
                    != 0
            {
                return Some(blocked_cost(e, other));
            }
            None
        })
    })
}

/// The arc step for an arc's flag: the float at `01064f64` when clear, at
/// `0106b3a8` when set (copied through the FPU).
fn arc_step(e: &mut Engine, flag: u32) -> f32 {
    let constant = if flag == 0 {
        ARC_STEP
    } else {
        ARC_STEP_REVERSED
    };
    f32::from_bits(x87_quiet_bits(e.global::<u32>(constant)))
}

/// `0069e0c0` then `00439e90`: `center` plus the 2D vector `direction`
/// scaled by `radius` (bits, pushed through the FPU), into `out` (through a
/// temporary at `temp`); returns what `00439e90` returns.
fn arc_point(e: &mut Engine, center: u32, direction: u32, radius: u32, temp: u32, out: u32) -> u32 {
    let offset = e
        .call(
            SCALED_POINT_0069E0C0,
            &args![direction, temp, x87_quiet_bits(radius)],
        )
        .u32();
    e.call(POINT3_PLUS_00439E90, &args![center, out, offset])
        .u32()
}

// Translated from 006a0ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshPOVSearch::CheckSegmentAgainstNavMesh` (Xbox PDB; `RET 0xC`):
/// walks the navmesh from `from` (a triangle handle with a point at +0xC)
/// towards the point `to`, edge by edge (`NavMesh::FindInterectingEdge2`).
/// When no edge is crossed the end record (`006a0c10` with the handle and
/// `to`) goes to `out` (`006cf900`) and the result is true. A crossed edge
/// is passable when it has a matching edge (`GetMatchingEdge_ov4`), unless
/// the setting at `011d6ef0` is below the request's float (+0x68 of
/// `pRequest`) and the next triangle's flag (`005582f0`, `00693b90`) is set,
/// or the edge's extra info (`GetEdgeExtraInfo`) has type 1; an impassable
/// edge writes the record at the crossing to `out` and gives false. On a
/// change of navmesh the walk restarts from a point pulled towards the
/// matching edge's middle (`GetEdgeVertices`, `0045bb20`, `006ba550`,
/// `0063c8a0`), else from the crossing; the handle moves on (`006bc870`).
pub fn navmesh_pov_search_check_segment_against_navmesh(
    e: &mut Engine,
    this: Ptr,
    from: Ptr,
    to: Ptr,
    out: Ptr,
) -> bool {
    with_frame(e, 0xf0, &[from.addr(), to.addr(), out.addr()], |e, ebp| {
        let start = ebp - 0x30;
        let end = ebp - 0x24;
        copy_words(e, start, from.addr() + 0xc, 3);
        copy_words(e, end, to.addr(), 3);
        e.call(POINT3_MINUS_00439EF0, &args![end, ebp - 0x3c, start]);
        e.call(DIRECTION_STEP_004A0C10, &args![ebp - 0x3c]);
        let position = ebp - 0x18;
        copy_words(e, position, start, 3);
        let handle = ebp - 0xc;
        e.call(BASE_SET_006C4350, &args![handle, from]);
        loop {
            let crossing = ebp - 0x50;
            let navmesh = e.mem.u32(handle);
            let triangle = u32::from(e.mem.u16(handle + 4));
            let extra = e.mem.u32(handle + 8);
            e.call(
                FIND_INTERSECTING_EDGE_00698160,
                &args![navmesh, crossing, triangle, extra, position, end],
            );
            if e.mem.u32(crossing) == 0 {
                let record = fn_006a0c10(e, Ptr::new(ebp - 0x9c), handle, Ptr::new(end));
                e.call(RECORD_COPY_006CF900, &args![out, record]);
                return true;
            }
            let edge = ebp - 0x5c;
            let navmesh = e.mem.u32(handle);
            let triangle = u32::from(e.mem.u16(handle + 4));
            let edge_index = e.mem.u32(crossing + 4);
            e.call(
                EDGE_HANDLE_0069A6C0,
                &args![edge, navmesh, triangle, edge_index],
            );
            let matching = ebp - 0x6c;
            e.call(BASE_CTOR_006240D0, &args![matching]);
            let edge_navmesh = e.mem.u32(edge);
            let found = e
                .call(
                    GET_MATCHING_EDGE_0068F670,
                    &args![edge_navmesh, edge, matching],
                )
                .u8();
            e.mem.set_u8(ebp - 0x5d, found);
            if found != 0 {
                let request = e.mem.u32(this.addr() + REQUEST);
                let limit = fn_0069ef80(e, Ptr::new(request));
                let setting = e
                    .call(SETTING_VALUE_00403E20, &args![SETTING_011D6EF0])
                    .u32();
                let value = x87_load32(e.mem.u32(setting));
                if value < limit {
                    let next_navmesh = e.mem.u32(matching);
                    let next_triangle = u32::from(e.mem.u16(matching + 4));
                    let record = e
                        .call(TRIANGLE_005582F0, &args![next_navmesh, next_triangle])
                        .u32();
                    if e.call(TRIANGLE_FLAG_00693B90, &args![record]).u8() != 0 {
                        e.mem.set_u8(ebp - 0x5d, 0);
                    }
                }
                let edge_navmesh = e.mem.u32(edge);
                let edge_triangle = u32::from(e.mem.u16(edge + 4));
                let edge_index = e.mem.u32(edge + 8);
                let info = e
                    .call(
                        GET_EDGE_EXTRA_INFO_0068F230,
                        &args![edge_navmesh, edge_triangle, edge_index],
                    )
                    .u32();
                if info != 0 && e.mem.u32(info) == 1 {
                    e.mem.set_u8(ebp - 0x5d, 0);
                }
            }
            if e.mem.u8(ebp - 0x5d) == 0 {
                let record = fn_006a0c10(e, Ptr::new(ebp - 0xe4), edge, Ptr::new(crossing + 8));
                e.call(RECORD_COPY_006CF900, &args![out, record]);
                return false;
            }
            if e.mem.u32(handle) != e.mem.u32(matching) {
                let next_navmesh = e.mem.u32(matching);
                let next_triangle = u32::from(e.mem.u16(matching + 4));
                let next_edge = e.mem.u32(matching + 8);
                let vertices = ebp - 0x84;
                e.call(
                    GET_EDGE_VERTICES_0068F040,
                    &args![next_navmesh, vertices, next_triangle, next_edge],
                );
                let half = x87_quiet_bits(e.global::<u32>(EDGE_MIDPOINT_SCALE));
                let first = e.mem.u32(vertices);
                let second = e.mem.u32(vertices + 4);
                let middle = ebp - 0x7c;
                let sum = e
                    .call(POINT3_PLUS_00439E90, &args![first, ebp - 0xa8, second])
                    .u32();
                e.call(POINT3_SCALE_0045BB20, &args![sum, middle, half]);
                let first = e.mem.u32(vertices);
                let second = e.mem.u32(vertices + 4);
                let point = e
                    .call(
                        SEGMENT_POINT_006BA550,
                        &args![ebp - 0xb4, first, second, crossing + 8, end],
                    )
                    .u32();
                copy_words(e, position, point, 3);
                let pull = x87_quiet_bits(e.global::<u32>(EDGE_PULL_SCALE));
                let towards = e
                    .call(POINT3_MINUS_00439EF0, &args![middle, ebp - 0xc0, position])
                    .u32();
                let step = e
                    .call(POINT3_SCALE_0045BB20, &args![towards, ebp - 0xcc, pull])
                    .u32();
                e.call(POINT3_ADD_0063C8A0, &args![position, step]);
            } else {
                copy_words(e, position, crossing + 8, 3);
            }
            e.call(HANDLE_SET_006BC870, &args![handle, matching]);
        }
    })
}

// Translated from 006a0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshPOVSearch::CheckArcAgainstNavMesh2` (Xbox PDB; `RET 0x20`: the
/// start record, the POV, the five words of an arc by value, the out
/// record): walks the arc around `pov` (center and radius +0xC) in at most
/// 0x18 steps of the arc step angle (`006a04e0`, rotations `006a0e90`),
/// checking each chord against the navmesh (`006a0ef0`) from the record
/// built from `start` (`006a0e50`); the last chord runs to the arc's end
/// (its second point). A failed chord, or a walk that takes all 0x18 steps,
/// gives false; the record reached goes to `out` (`006cf900`) on every
/// exit.
pub fn navmesh_pov_search_check_arc_against_navmesh2(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    pov: Ptr,
    first_x: u32,
    first_y: u32,
    second_x: u32,
    second_y: u32,
    flag: u32,
    out: Ptr,
) -> bool {
    let words = [
        start.addr(),
        pov.addr(),
        first_x,
        first_y,
        second_x,
        second_y,
        flag,
        out.addr(),
    ];
    with_frame(e, 0xa8, &words, |e, ebp| {
        let center = pov.addr();
        let step = arc_step(e, flag);
        let rotation = ebp - 0x68;
        fn_006a04e0(e, Ptr::new(rotation), step);
        let current = ebp - 0x38;
        let next = ebp - 0x5c;
        e.call(POINT_SET_006985F0, &args![current, ebp + 0x10]);
        fn_006a0e90(e, Ptr::new(current), Ptr::new(next), Ptr::new(rotation));
        let radius = e.mem.u32(center + 0xc);
        let point = ebp - 0x24;
        arc_point(e, center, current, radius, ebp - 0x74, point);
        let record = ebp - 0x54;
        fn_006a0e50(e, Ptr::new(record), start);
        let reached = ebp - 0x18;
        fn_006a0480(e, Ptr::new(reached));
        e.mem.set_u32(ebp - 0x3c, 0xffff_ffff);
        let mut steps = 0u32;
        while fn_006a0660(e, Ptr::new(ebp + 0x10), Ptr::new(next)) && steps < ARC_STEPS {
            e.mem.set_u32(ebp - 0x4c, 0xffff_ffff);
            let clear = navmesh_pov_search_check_segment_against_navmesh(
                e,
                this,
                Ptr::new(record),
                Ptr::new(point),
                Ptr::new(reached),
            );
            if !clear {
                e.call(RECORD_COPY_006CF900, &args![out, reached]);
                return false;
            }
            e.call(RECORD_COPY_006CF900, &args![record, reached]);
            e.call(POINT_SET_006985F0, &args![current, next]);
            let rotated = fn_006a0e90(
                e,
                Ptr::new(current),
                Ptr::new(ebp - 0x7c),
                Ptr::new(rotation),
            );
            e.call(POINT_SET_006985F0, &args![next, rotated]);
            let radius = e.mem.u32(center + 0xc);
            let sum = arc_point(e, center, next, radius, ebp - 0x88, ebp - 0x94);
            copy_words(e, point, sum, 3);
            let reached_word = e.mem.u32(ebp - 0x10);
            e.mem.set_u32(ebp - 0x3c, reached_word);
            steps = steps.wrapping_add(1);
        }
        if steps == ARC_STEPS {
            e.call(RECORD_COPY_006CF900, &args![out, reached]);
            return false;
        }
        let radius = e.mem.u32(center + 0xc);
        let last = ebp - 0x30;
        arc_point(e, center, ebp + 0x18, radius, ebp - 0xa0, last);
        e.mem.set_u32(ebp - 0x4c, 0xffff_ffff);
        let clear = navmesh_pov_search_check_segment_against_navmesh(
            e,
            this,
            Ptr::new(record),
            Ptr::new(last),
            Ptr::new(reached),
        );
        e.call(RECORD_COPY_006CF900, &args![out, reached]);
        clear
    })
}

// Translated from 006a1330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a1330 - its FS:[0] exception frame (the unwind state of the line's smart pointer) is not translated
/// `NavMeshPOVSearch::DrawLine` (Xbox PDB; cdecl: two points, a colour):
/// builds a debug line (`004b3890(from, colour, to, colour, 1)`) into a
/// smart pointer (`00633c90`), runs `00a5a040` and `00a59c60` (with a zero
/// point from `0043d410(0.0, 0, 0)`) on it, attaches it to the node of the
/// smart pointer at `011d6ddc` (vtable slot +0xDC with the line and 1) and
/// releases the pointer (`0045cec0`).
pub fn navmesh_pov_search_draw_line(e: &mut Engine, from: Ptr, to: Ptr, colour: u32) {
    let line = e
        .call(MAKE_LINE_004B3890, &args![from, colour, to, colour, 1u32])
        .u32();
    with_frame(e, 0x20, &[from.addr(), to.addr(), colour], |e, ebp| {
        let pointer = ebp - 0x10;
        let zero = ebp - 0x1c;
        e.call(POINTER_SET_00633C90, &args![pointer, line]);
        e.call(
            THREE_WORD_CTOR_0043D410,
            &args![zero, x87_store32(0.0), 0u32, 0u32],
        );
        let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
        e.call(UPDATE_PROPERTIES_00A5A040, &args![object]);
        let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
        e.call(ZERO_RECORD_STEP_00A59C60, &args![object, zero]);
        let root = e.call(FIRST_OF_00559450, &args![DEBUG_ROOT_011D6DDC]).u32();
        let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
        e.vcall(root, SLOT_ATTACH, &args![object, 1u32]);
        e.call(POINTER_RELEASE_0045CEC0, &args![pointer]);
    });
}

// Translated from 006a1180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshPOVSearch::DrawArc` (Xbox PDB; cdecl: center, radius, the arc's
/// two 2D points and flag, a colour): draws the arc around `center` as up
/// to 0x18 chords (`006a1330`), stepping by the arc step angle as
/// `006a0c50` does. When the walk stops before 0x18 steps the exe computes
/// the point at the arc's end (into a local it never reads) and draws the
/// last chord again between the two points it already has.
pub fn navmesh_pov_search_draw_arc(
    e: &mut Engine,
    center: Ptr,
    radius: f32,
    first: Ptr,
    second: Ptr,
    flag: u32,
    colour: u32,
) {
    with_frame(e, 0x9c, &[], |e, ebp| {
        let center = center.addr();
        let radius = radius.to_bits();
        let segment = ebp - 0x40;
        fn_006a05e0(e, Ptr::new(segment), first, second, flag);
        let step = arc_step(e, flag);
        let rotation = ebp - 0x54;
        fn_006a04e0(e, Ptr::new(rotation), step);
        let current = ebp - 0x2c;
        let next = ebp - 0x48;
        e.call(POINT_SET_006985F0, &args![current, first]);
        fn_006a0e90(e, Ptr::new(current), Ptr::new(next), Ptr::new(rotation));
        let from = ebp - 0x18;
        let to = ebp - 0xc;
        arc_point(e, center, current, radius, ebp - 0x60, from);
        arc_point(e, center, next, radius, ebp - 0x6c, to);
        let mut steps = 0u32;
        while fn_006a0660(e, Ptr::new(segment), Ptr::new(next)) && steps < ARC_STEPS {
            navmesh_pov_search_draw_line(e, Ptr::new(from), Ptr::new(to), colour);
            copy_words(e, from, to, 3);
            e.call(POINT_SET_006985F0, &args![current, next]);
            let rotated = fn_006a0e90(
                e,
                Ptr::new(current),
                Ptr::new(ebp - 0x74),
                Ptr::new(rotation),
            );
            e.call(POINT_SET_006985F0, &args![next, rotated]);
            let sum = arc_point(e, center, next, radius, ebp - 0x80, ebp - 0x8c);
            copy_words(e, to, sum, 3);
            steps = steps.wrapping_add(1);
        }
        if steps != ARC_STEPS {
            arc_point(e, center, second.addr(), radius, ebp - 0x98, ebp - 0x24);
            navmesh_pov_search_draw_line(e, Ptr::new(from), Ptr::new(to), colour);
        }
    });
}

// Translated from 006a1900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`; `this` unread): `0042f5d0(first,
/// count * 0x40)`, its result returned.
pub fn fn_006a1900(e: &mut Engine, _this: Ptr, first: u32, count: u32) -> u32 {
    e.call(BYTES_0042F5D0, &args![first, count << 6]).u32()
}

// Translated from 006a1b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): whether the two words at `this`
/// equal those at `other`.
pub fn fn_006a1b20(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.u32(other.addr()) == e.mem.u32(this.addr())
        && e.mem.u32(other.addr() + 4) == e.mem.u32(this.addr() + 4)
}

// Translated from 006a1b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): `006a1b20(first, second)`.
pub fn fn_006a1b00(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    fn_006a1b20(e, first, second)
}

// Translated from 006a1b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): `006d4ca0(first, second)`,
/// its EAX returned.
pub fn fn_006a1b80(e: &mut Engine, first: u32, second: u32) -> u32 {
    e.call(COMPARE_006D4CA0, &args![first, second]).u32()
}

// Translated from 006a1aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xC`): the index of the first element
/// from `start` of a `BSSimpleArray` of 8-byte elements for which the cdecl
/// `compare(element, key)` returns a non-zero byte (elements are only
/// tested while the buffer is set), or -1.
pub fn fn_006a1aa0(e: &mut Engine, this: Ptr, key: u32, start: u32, compare: u32) -> u32 {
    let base = this.addr();
    let mut index = start;
    while index < e.mem.u32(base + 8) {
        let buffer = e.mem.u32(base + 4);
        if buffer != 0 {
            let element = buffer.wrapping_add(index.wrapping_mul(8));
            if e.call(compare, &args![element, key]).u8() != 0 {
                return index;
            }
        }
        index = index.wrapping_add(1);
    }
    0xffff_ffff
}

// Translated from 006a1460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): whether the array holds an element
/// `006a1b80` matches with `key` (`006a1aa0` from 0).
pub fn fn_006a1460(e: &mut Engine, this: Ptr, key: u32) -> bool {
    fn_006a1aa0(e, this, key, 0, 0x006a_1b80) != 0xffff_ffff
}

// Translated from 006a2440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor setting both words of the
/// 8-byte element to -1. Returns `this`.
pub fn fn_006a2440(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0xffff_ffff);
    e.mem.set_u32(this.addr() + 4, 0xffff_ffff);
    this
}

/// The shared body of the element-constructing loops `006a2000`,
/// `006a23a0` and `006a26d0`: for each of `count` elements of `size` bytes
/// from `first`, placement `new` (`006e6da0(size, place)`) and, when that
/// gives a place, `construct` on it.
fn construct_elements(
    e: &mut Engine,
    first: u32,
    count: u32,
    size: u32,
    construct: fn(&mut Engine, Ptr) -> Ptr,
) {
    let mut index = 0u32;
    while index < count {
        let place = index.wrapping_mul(size).wrapping_add(first);
        let element = e.call(PLACEMENT_NEW_006E6DA0, &args![size, place]).u32();
        if element != 0 {
            construct(e, Ptr::new(element));
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006a2000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a2000 - its FS:[0] exception frame (the unwind state of each placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`; `this` unread): constructs `count`
/// `BSSimpleArray<NavMeshPOV*,1024>` elements (0x10 bytes, `006a14b0`) from
/// `first`, each after placement `new`.
pub fn fn_006a2000(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    construct_elements(e, first, count, 0x10, fn_006a14b0);
}

// Translated from 006a23a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a23a0 - its FS:[0] exception frame (the unwind state of each placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`; `this` unread): constructs `count`
/// 8-byte elements (`006a2440`) from `first`, each after placement `new`.
pub fn fn_006a23a0(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    construct_elements(e, first, count, 8, fn_006a2440);
}

// Translated from 006a2880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<FastNavMeshVertHandle,1024>` (by the
/// vtable it stores): the vtable, then `006dc440(this, 0, 0)`. Returns
/// `this`.
pub fn fn_006a2880(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B460);
    e.call(ARRAY_INIT_006DC440, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006a26d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a26d0 - its FS:[0] exception frame (the unwind state of each placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`; `this` unread): constructs `count`
/// `BSSimpleArray<FastNavMeshVertHandle,1024>` elements (0x10 bytes,
/// `006a2880`) from `first`, each after placement `new`.
pub fn fn_006a26d0(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    construct_elements(e, first, count, 0x10, fn_006a2880);
}

/// The shared body of the `BSSimpleArray` initializations `006a20a0`,
/// `006a2310` and `006a2470` (as `006a2130`): clears buffer, size and
/// capacity; the reserve is raised to `size` when smaller; a non-zero
/// reserve allocates through vtable slot +4 and sets the capacity; a
/// non-zero size constructs the elements and sets the size.
fn array_initialize(
    e: &mut Engine,
    this: Ptr,
    reserve: u32,
    size: u32,
    construct: fn(&mut Engine, Ptr, u32, u32),
) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(base, SLOT_ALLOCATE, &args![reserve]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, reserve);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        construct(e, this, buffer, size);
        e.mem.set_u32(base + 8, size);
    }
}

// Translated from 006a20a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the initialization of a
/// `BSSimpleArray` of `BSSimpleArray<NavMeshPOV*,1024>` elements (see
/// `array_initialize`; elements by `006a2000`).
pub fn fn_006a20a0(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    array_initialize(e, this, reserve, size, fn_006a2000);
}

// Translated from 006a2310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the initialization of a
/// `BSSimpleArray` of `BSSimpleArray<FastNavMeshVertHandle,1024>` elements
/// (see `array_initialize`; elements by `006a26d0`).
pub fn fn_006a2310(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    array_initialize(e, this, reserve, size, fn_006a26d0);
}

// Translated from 006a2470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the initialization of a
/// `BSSimpleArray` of 8-byte elements (see `array_initialize`; elements by
/// `006a23a0`).
pub fn fn_006a2470(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    array_initialize(e, this, reserve, size, fn_006a23a0);
}

// Translated from 006a16d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<BSSimpleArray<NavMeshPOV*,1024>,1024>`
/// (by the vtable it stores): the vtable, then `006a20a0(this, 0, 0)`.
/// Returns `this`.
pub fn fn_006a16d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3D4);
    fn_006a20a0(e, this, 0, 0);
    this
}

// Translated from 006a1860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<NavMeshPOVSearchNode,1024>` (by the
/// vtable it stores): the vtable, then `006b3f40(this, 0, 0)`. Returns
/// `this`.
pub fn fn_006a1860(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3E8);
    e.call(ARRAY_INIT_006B3F40, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006a18b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<NavMeshPOVSearchLink,1024>` (by the
/// vtable it stores): the vtable, then `006a2130(this, 0, 0)`. Returns
/// `this`.
pub fn fn_006a18b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3FC);
    fn_006a2130(e, this, 0, 0);
    this
}

// Translated from 006a19b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of
/// `BSSimpleArray<BSSimpleArray<FastNavMeshVertHandle,1024>,1024>` with a
/// size (`RET 4`; by the vtable it stores): the vtable, then
/// `006a2310(this, count, count)`. Returns `this`.
pub fn fn_006a19b0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B410);
    fn_006a2310(e, this, count, count);
    this
}

// Translated from 006a1a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<RemoveIndex,1024>` (by the vtable it
/// stores): the vtable, then `006a2470(this, 0, 0)`. Returns `this`.
pub fn fn_006a1a00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B424);
    fn_006a2470(e, this, 0, 0);
    this
}

// Translated from 006a2850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of
/// `BSSimpleArray<AStarNode<NavMeshPOVSearchNode,NavMeshPOVSearchLink>*,1024>`
/// (by the vtable it stores): the vtable, then `006b3eb0(this, 0, 0)`.
/// Returns `this`.
pub fn fn_006a2850(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B44C);
    e.call(ARRAY_INIT_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006a2500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a2500 - its FS:[0] exception frame (the unwind state of the base construction) is not translated
/// The constructor of the `BSScrapArray` of `AStarNode` pointers (by the
/// vtable it stores): the `BSSimpleArray` base (`006a2850`), its own
/// vtable, the scrap heap at +0x10 (`00aa42e0` on the holder `00401020`
/// returns), then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_006a2500(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006a2850(e, this);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106B438);
    let holder = e.call(SCRAP_HOLDER_00401020, &args![]).u32();
    let heap = e.call(SCRAP_HEAP_00AA42E0, &args![holder]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(ARRAY_INIT_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006a25d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a25d0 - its FS:[0] exception frame (the unwind state of the base destruction) is not translated
/// The destructor body of the `BSScrapArray` of `AStarNode` pointers (by
/// the vtable it stores): its vtable, `008454f0(this, 1)`, then the
/// `BSSimpleArray` base destructor body (`006a2580`).
pub fn fn_006a25d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106B438);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006a2580(e, this);
}

// Translated from 006a1500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a1500 - its FS:[0] exception frame (the unwind state of the member constructors) is not translated
/// The constructor of `AStarSearch<NavMeshPOVSearchNode,NavMeshPOVSearchLink>`
/// (by the vtable it stores): the node heap (+4, `006a1f70`), node map
/// (+0x14, `006b3de0`) and queue (+0x1FF4, `006b9470`), then
/// `BestNodeSoFar`, `pLastSearchedNode`, `pStartAStarNode` and
/// `bIsRunningInTasklet` cleared. Returns `this`.
pub fn fn_006a1500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, ASTAR_SEARCH_VTABLE);
    fn_006a1f70(e, Ptr::new(base + 4));
    e.call(NODE_MAP_CTOR_006B3DE0, &args![base + SEARCH_NODE_MAP]);
    e.call(QUEUE_CTOR_006B9470, &args![base + SEARCH_QUEUE]);
    e.mem.set_u32(base + BEST_NODE_SO_FAR, 0);
    e.mem.set_u32(base + LAST_SEARCHED_NODE, 0);
    e.mem.set_u32(base + START_ASTAR_NODE, 0);
    e.mem.set_u8(base + RUNNING_IN_TASKLET, 0);
    this
}

// Translated from 006a1cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a1cf0 - its FS:[0] exception frame (the unwind state of the connection array) is not translated
/// Unnamed in the engine map: the A* search loop of
/// `AStarSearch<NavMeshPOVSearchNode,NavMeshPOVSearchLink>`. Takes nodes
/// from the queue (`006b4350`) into `pLastSearchedNode` until it is empty;
/// a goal node (vtable slot +8) becomes `BestNodeSoFar` and is returned.
/// For the others the connections (vtable slot +4, into a local
/// `BSScrapArray`, emptied by `008454f0(array, 0)` first) are unlinked
/// (`006a1fb0`), parented to the current node and queued (`006b3e10`); a
/// connection that is the current node's parent ends the search with 0,
/// and one cheaper than `BestNodeSoFar` (`006b9130`, strictly, ordered)
/// becomes it. On a tasklet every 50th node runs `0040fca0(0)`. An empty
/// queue restores the last node and returns 0.
pub fn fn_006a1cf0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let queue = base + SEARCH_QUEUE;
    e.with_stack(0x14, |e, array| {
        fn_006a2500(e, array);
        let mut previous = e.mem.u32(base + LAST_SEARCHED_NODE);
        let next = e.call(QUEUE_POP_006B4350, &args![queue]).u32();
        e.mem.set_u32(base + LAST_SEARCHED_NODE, next);
        if e.mem.u32(base + LAST_SEARCHED_NODE) == 0 {
            fn_006a25d0(e, array);
            return 0;
        }
        let current = e.mem.u32(base + LAST_SEARCHED_NODE);
        e.mem.set_u32(base + BEST_NODE_SO_FAR, current);
        let mut ticks = 0u32;
        while e.mem.u32(base + LAST_SEARCHED_NODE) != 0 {
            let current = e.mem.u32(base + LAST_SEARCHED_NODE);
            if e.vcall(base, 8, &args![current]).u8() != 0 {
                let current = e.mem.u32(base + LAST_SEARCHED_NODE);
                e.mem.set_u32(base + BEST_NODE_SO_FAR, current);
                let found = e.mem.u32(base + LAST_SEARCHED_NODE);
                fn_006a25d0(e, array);
                return found;
            }
            e.call(ARRAY_RELEASE_008454F0, &args![array, 0u32]);
            let current = e.mem.u32(base + LAST_SEARCHED_NODE);
            e.vcall(base, 4, &args![current, array]);
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
                let slot = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
                let node = e.mem.u32(slot);
                fn_006a1fb0(e, Ptr::new(queue), Ptr::new(node));
                let current = e.mem.u32(base + LAST_SEARCHED_NODE);
                if e.mem.u32(current + PARENT_ASTAR_NODE) == node {
                    fn_006a25d0(e, array);
                    return 0;
                }
                let current = e.mem.u32(base + LAST_SEARCHED_NODE);
                e.mem.set_u32(node + PARENT_ASTAR_NODE, current);
                e.call(QUEUE_PUSH_006B3E10, &args![queue, node]);
                let cost = e.call(NODE_COST_006B9130, &args![node]).f64();
                let best = e.mem.u32(base + BEST_NODE_SO_FAR);
                let best_cost = e.call(NODE_COST_006B9130, &args![best]).f64();
                if best_cost > cost {
                    e.mem.set_u32(base + BEST_NODE_SO_FAR, node);
                }
                index = index.wrapping_add(1);
            }
            if e.mem.u8(base + RUNNING_IN_TASKLET) != 0 {
                ticks = ticks.wrapping_add(1);
                if ticks == 0x32 {
                    ticks = 0;
                    e.call(YIELD_0040FCA0, &args![0u32]);
                }
            }
            previous = e.mem.u32(base + LAST_SEARCHED_NODE);
            let next = e.call(QUEUE_POP_006B4350, &args![queue]).u32();
            e.mem.set_u32(base + LAST_SEARCHED_NODE, next);
        }
        e.mem.set_u32(base + LAST_SEARCHED_NODE, previous);
        fn_006a25d0(e, array);
        0
    })
}

// Translated from 006a1630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xC`): runs the search. Sets
/// `bIsRunningInTasklet` from the object `00b00a00` returns (vtable slot
/// +0xC with `0040fc90`'s word), runs the loop (`006a1cf0`); with a found
/// node (result true), or without one when `use_best` is set (result false,
/// `BestNodeSoFar`), builds the path from it (`006b3cb0(this, node, first,
/// second)`).
pub fn fn_006a1630(e: &mut Engine, this: Ptr, first: u32, second: u32, use_best: u8) -> bool {
    let base = this.addr();
    let manager = e.call(TASKLET_MANAGER_00B00A00, &args![]).u32();
    let word = e.call(THREAD_ID_0040FC90, &args![]).u32();
    let tasklet = e.vcall(manager, 0xc, &args![word]).u8();
    e.mem.set_u8(base + RUNNING_IN_TASKLET, tasklet);
    let found = fn_006a1cf0(e, this);
    let (result, node) = if found != 0 {
        (true, found)
    } else if use_best != 0 {
        (false, e.mem.u32(base + BEST_NODE_SO_FAR))
    } else {
        (false, 0)
    };
    if node != 0 {
        e.call(BUILD_PATH_006B3CB0, &args![this, node, first, second]);
    }
    result
}

// Translated from 006a1720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): sets the size of a `BSSimpleArray`
/// of `BSSimpleArray<NavMeshPOV*,1024>` elements (0x10 bytes) to `count`.
/// A zero count releases it (`0049faf0(this, shrink)`). Above the capacity
/// it allocates (vtable slot +4, no capacity yet) or grows (`009f6540` with
/// the size) and sets the capacity, then constructs the new elements
/// (`006a2000`). Below the size it destroys the rest (`0049fc80`) and, with
/// `shrink` set and the count at most a quarter of the capacity, shrinks
/// the buffer to the count (`009f6540`). Otherwise it constructs the new
/// elements. The size becomes `count`.
pub fn fn_006a1720(e: &mut Engine, this: Ptr, count: u32, shrink: u8) {
    let base = this.addr();
    if count == 0 {
        e.call(ARRAY_RELEASE_0049FAF0, &args![this, u32::from(shrink)]);
        return;
    }
    if count > e.mem.u32(base + 0xc) {
        if e.mem.u32(base + 0xc) == 0 {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![count]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, count);
        } else {
            let size = e.mem.u32(base + 8);
            e.call(ARRAY_RESIZE_009F6540, &args![this, count, size]);
            e.mem.set_u32(base + 0xc, count);
        }
        let size = e.mem.u32(base + 8);
        let first = (size << 4).wrapping_add(e.mem.u32(base + 4));
        fn_006a2000(e, this, first, count.wrapping_sub(size));
        e.mem.set_u32(base + 8, count);
    } else if count < e.mem.u32(base + 8) {
        let rest = e.mem.u32(base + 8).wrapping_sub(count);
        let first = (count << 4).wrapping_add(e.mem.u32(base + 4));
        e.call(ARRAY_DESTROY_0049FC80, &args![this, first, rest]);
        e.mem.set_u32(base + 8, count);
        if shrink != 0 && count <= e.mem.u32(base + 0xc) >> 2 {
            e.call(ARRAY_RESIZE_009F6540, &args![this, count, count]);
            e.mem.set_u32(base + 0xc, count);
        }
    } else {
        let size = e.mem.u32(base + 8);
        let first = (size << 4).wrapping_add(e.mem.u32(base + 4));
        fn_006a2000(e, this, first, count.wrapping_sub(size));
        e.mem.set_u32(base + 8, count);
    }
}

// Translated from 006a1a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): appends the 8-byte value at
/// `value` to a `BSSimpleArray` of 8-byte elements: reserves an index
/// (`00845460`), constructs the element (`006a23a0` with one) and copies
/// both words. Returns the index.
pub fn fn_006a1a50(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    let base = this.addr();
    let index = e.call(ARRAY_RESERVE_00845460, &args![this]).u32();
    let place = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(8));
    fn_006a23a0(e, this, place, 1);
    let low = e.mem.u32(value.addr());
    let high = e.mem.u32(value.addr() + 4);
    let element = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(8));
    e.mem.set_u32(element, low);
    e.mem.set_u32(element + 4, high);
    index
}

/// The scalar deleting destructors of this unit: `destroy` on `this`,
/// then `operator delete` when bit 0 of `flags` is set; returns `this`.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    delete_if(e, this, flags)
}

// Translated from 006a1ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshPOV*,1024>::scalar deleting destructor` (Xbox
/// PDB): `006a14e0`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_navmesh_pov_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a14e0)
}

// Translated from 006a1bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<NavMeshPOVSearchNode,NavMeshPOVSearchLink>::scalar
/// deleting destructor` (Xbox PDB): `006a15b0`, then `operator delete`
/// when bit 0 of `flags` is set.
pub fn astar_search_navmesh_pov_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a15b0)
}

// Translated from 006a1c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSSimpleArray<NavMeshPOV*,1024>,1024>::scalar deleting
/// destructor` (Xbox PDB): `006a1700`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn bs_simple_array_navmesh_pov_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a1700)
}

// Translated from 006a1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshPOVSearchNode,1024>::scalar deleting destructor`
/// (Xbox PDB): `006a1890`, then `operator delete` when bit 0 of `flags` is
/// set.
pub fn bs_simple_array_navmesh_pov_search_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a1890)
}

// Translated from 006a1c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshPOVSearchLink,1024>::scalar deleting destructor`
/// (Xbox PDB): `006a18e0`, then `operator delete` when bit 0 of `flags` is
/// set.
pub fn bs_simple_array_navmesh_pov_search_link_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a18e0)
}

// Translated from 006a1c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSSimpleArray<FastNavMeshVertHandle,1024>,1024>::scalar
/// deleting destructor` (Xbox PDB): `006a19e0`, then `operator delete`
/// when bit 0 of `flags` is set.
pub fn bs_simple_array_vert_handle_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a19e0)
}

// Translated from 006a1cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<RemoveIndex,1024>::scalar deleting destructor` (Xbox
/// PDB): `006a1a30`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_remove_index_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a1a30)
}

// Translated from 006a25a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<AStarNode<NavMeshPOVSearchNode,NavMeshPOVSearchLink>*,1024>::scalar
/// deleting destructor` (Xbox PDB): `006a2580`, then `operator delete`
/// when bit 0 of `flags` is set.
pub fn bs_simple_array_astar_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a2580)
}

// Translated from 006a2820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<AStarNode<NavMeshPOVSearchNode,NavMeshPOVSearchLink>*,1024>::scalar
/// deleting destructor` (Xbox PDB): `006a25d0`, then `operator delete`
/// when bit 0 of `flags` is set.
pub fn bs_scrap_array_astar_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a25d0)
}

// Translated from 006a28d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FastNavMeshVertHandle,1024>::scalar deleting destructor`
/// (Xbox PDB): `006a2900`, then `operator delete` when bit 0 of `flags` is
/// set.
pub fn bs_simple_array_vert_handle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006a2900)
}

/// The debug-draw switch (byte) `0069f010` and `0069f9d0` test.
const DEBUG_DRAW_011D6DCC: u32 = 0x011d_6dcc;
/// The colours (addresses) the connection checks draw with: blocked by
/// the navmesh (`011a9c20`), blocked by another POV (`011a9bf0`), clear
/// (`011a9c00`).
const COLOUR_BLOCKED: u32 = 0x011a_9c20;
const COLOUR_COVERED: u32 = 0x011a_9bf0;
const COLOUR_CLEAR: u32 = 0x011a_9c00;
/// `006950d0` (unnamed in the engine map; `RET 4`): the address of an
/// element of an array of 8-byte records.
const RECORD_AT_006950D0: u32 = 0x0069_50d0;
/// `006917d0`, `00691800` (unnamed in the engine map): the constructor and
/// destructor of the vertex list `0069e3b0` fills.
const VERTEX_LIST_CTOR_006917D0: u32 = 0x0069_17d0;
const VERTEX_LIST_DTOR_00691800: u32 = 0x0069_1800;
/// `0069a830`, `0069a940` (unnamed in the engine map; `RET 8`: triangle,
/// list): fill the list with a triangle's vertices of two kinds; return the
/// count.
const CORNER_VERTICES_0069A830: u32 = 0x0069_a830;
const EDGE_VERTICES_0069A940: u32 = 0x0069_a940;
/// `005eeda0` (unnamed in the engine map; `RET 4`): the address of a
/// 16-bit element of the vertex list.
const VERTEX_AT_005EEDA0: u32 = 0x005e_eda0;
/// `0069a660` (unnamed in the engine map; `RET 8`: navmesh, vertex): a
/// vertex handle into `this`.
const VERTEX_HANDLE_0069A660: u32 = 0x0069_a660;
/// `006dba30` (unnamed in the engine map; `RET 4`): appends a vertex
/// handle.
const HANDLE_APPEND_006DBA30: u32 = 0x006d_ba30;
/// `0069a9d0` (unnamed in the engine map; `RET 8`: vertex, out): the two
/// edge references of a corner vertex (AL: whether it has them).
const CORNER_EDGES_0069A9D0: u32 = 0x0069_a9d0;
/// `0068f0a0` (unnamed in the engine map; `RET 4`): the address of a
/// navmesh vertex's position.
const VERTEX_POSITION_0068F0A0: u32 = 0x0068_f0a0;
/// `0068f1d0` (unnamed in the engine map; `RET 4`): a flag of a triangle's
/// edge (AL).
const EDGE_FLAG_0068F1D0: u32 = 0x0068_f1d0;
/// `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized` (`007cb2e0`, engine
/// map name of the folded body; `RET 4`): appends the word at the address
/// it is given.
const ARRAY_PUSH_007CB2E0: u32 = 0x007c_b2e0;
/// `004b0680` (unnamed in the engine map; `RET 8`): a two-word record
/// into `this`; returns `this`.
const PAIR_004B0680: u32 = 0x004b_0680;
/// `BSSimpleArray<ItemChange*,1024>::Remove` (`006bf8f0`, engine map name
/// of the folded body; `RET 8`: index, count).
const ARRAY_REMOVE_006BF8F0: u32 = 0x006b_f8f0;
/// The radius (float) of the second POV of an edge vertex.
const EDGE_POV_RADIUS: u32 = 0x0101_7b78;
/// The squared distance (float) under which two POVs are duplicates.
const DUPLICATE_DISTANCE_SQUARED: u32 = 0x0101_81e8;
/// `PathingLocation::operator=` (`006dcce0`, engine map; `RET 4`).
const LOCATION_ASSIGN_006DCCE0: u32 = 0x006d_cce0;
/// `PathingLocation::ResolveNavMeshInfo` (`006dd6f0`, engine map; `RET 4`).
const RESOLVE_NAVMESH_INFO_006DD6F0: u32 = 0x006d_d6f0;
/// `006dd6a0` (unnamed in the engine map): whether a `PathingLocation` has
/// navmesh info (AL).
const LOCATION_RESOLVED_006DD6A0: u32 = 0x006d_d6a0;
/// `PathingLocation::GetNavMeshAndTriangle` (`006dd640`, engine map; `RET 8`).
const GET_NAVMESH_AND_TRIANGLE_006DD640: u32 = 0x006d_d640;
/// `005c3420` (unnamed in the engine map; `RET 4`: out): a location's
/// position into `out`; returns `out`.
const LOCATION_POSITION_005C3420: u32 = 0x005c_3420;
/// `006dcf20` (unnamed in the engine map; `RET 8`): a `PathingLocation`
/// constructor from a position and navmesh info.
const LOCATION_CTOR_006DCF20: u32 = 0x006d_cf20;
/// `0068efb0` (unnamed in the engine map): the navmesh's info.
const NAVMESH_INFO_0068EFB0: u32 = 0x0068_efb0;
/// `006b30e0` (unnamed in the engine map; `RET 4`): the address of a link
/// of the `Links` array.
const LINK_AT_006B30E0: u32 = 0x006b_30e0;
/// `006b32f0` (unnamed in the engine map; `RET 0xC`: key, node data,
/// heuristic): puts the start node into the search.
const START_NODE_006B32F0: u32 = 0x006b_32f0;
/// `006b3420` (unnamed in the engine map; `RET 0xC`: key, cost, out node):
/// finds or makes the node for a key (0 new, 1 cheaper, other: keep the
/// old one).
const NODE_FOR_KEY_006B3420: u32 = 0x006b_3420;
/// `0052aa80` (unnamed in the engine map; `RET 4`), on the debug root.
const ROOT_TEST_0052AA80: u32 = 0x0052_aa80;
/// `PathManager::QInstance` (`0047d0b0`, engine map; no arguments).
const PATH_MANAGER_INSTANCE: u32 = 0x0047_d0b0;
/// `006ebbd0` (unnamed in the engine map; `RET 8`: root, flag): hands the
/// debug root to the path manager.
const PATH_MANAGER_DEBUG_006EBBD0: u32 = 0x006e_bbd0;
/// `00aa13e0` (unnamed in the engine map; cdecl, a size): an allocation.
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `00a5ecb0` (unnamed in the engine map, `NiMain/ninode.cpp`; `RET 4`): a
/// node constructor.
const NODE_CTOR_00A5ECB0: u32 = 0x00a5_ecb0;
/// `0066b0d0` (unnamed in the engine map; `RET 4`): sets the debug root's
/// smart pointer.
const ROOT_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `00414430` (unnamed in the engine map; `RET 0x10`): a colour from four
/// floats.
const COLOUR_00414430: u32 = 0x0041_4430;
/// The fourth component (float) of the debug spheres' colour.
const SPHERE_ALPHA: u32 = 0x0103_0ff4;
/// `BSTestObjects::CreateSphere` (`00c56680`, engine map; cdecl, four words).
const CREATE_SPHERE_00C56680: u32 = 0x00c5_6680;
/// `00440460` (unnamed in the engine map; `RET 4`): sets an object's
/// translation.
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
/// `004391c0` (unnamed in the engine map): the 0x1C-byte property
/// constructor.
const PROPERTY_CTOR_004391C0: u32 = 0x0043_91c0;
/// `0049ed90` (unnamed in the engine map; `RET 4`), on the property with 1.
const PROPERTY_SET_0049ED90: u32 = 0x0049_ed90;
/// `NiAVObject::AttachProperty` (`00439410`, engine map; `RET 4`).
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
/// `AddNoLightingPropertyRecurse` (`004b6e90`, engine map; cdecl).
const NO_LIGHTING_004B6E90: u32 = 0x004b_6e90;
/// `BSShaderManager::PrepareObject` (`00b57e30`, engine map; cdecl, three
/// words).
const PREPARE_OBJECT_00B57E30: u32 = 0x00b5_7e30;
/// `007d6bb0` (unnamed in the engine map): the search node's parent link.
const PARENT_LINK_007D6BB0: u32 = 0x007d_6bb0;
/// `00595c80` (unnamed in the engine map): a 3D vector's horizontal length
/// (ST0).
const LENGTH_2D_00595C80: u32 = 0x0059_5c80;
/// The `double` the horizontal distance is scaled by twice before the
/// height test.
const HEIGHT_SCALE: u32 = 0x0102_90f0;
/// `006bb570` (unnamed in the engine map; cdecl, four words): the tangent
/// segments between two circles; returns their count.
const TANGENTS_006BB570: u32 = 0x006b_b570;
/// The angle step (float) for the parent link's wedge.
const WEDGE_STEP: u32 = 0x0101_ff34;
/// `004b3800` (unnamed in the engine map; `RET 8`: out, other): the 3D
/// cross product `this x other` into `out`.
const CROSS_004B3800: u32 = 0x004b_3800;
/// The `double` limit of the cross products' height.
const CROSS_LIMIT: u32 = 0x0106_b170;
/// `0042fb00`, `0042fa40` (unnamed in the engine map): the constructor and
/// destructor of the navmesh holder `0069f9d0` passes to
/// `GetNavMeshAndTriangle`.
const HOLDER_CTOR_0042FB00: u32 = 0x0042_fb00;
const HOLDER_DTOR_0042FA40: u32 = 0x0042_fa40;
/// `00408840` (unnamed in the engine map; cdecl, a float): its absolute
/// value (ST0).
const ABS_00408840: u32 = 0x0040_8840;
/// The `double` height limit of a connection's ends.
const HEIGHT_LIMIT: u32 = 0x0104_ed58;
/// The `double` cost meaning "blocked" (the largest float).
const BLOCKED: u32 = 0x0102_31b0;
/// `006a7f50` (unnamed in the engine map): a search node's cost so far
/// (ST0).
const NODE_COST_006A7F50: u32 = 0x006a_7f50;
/// `0052ce20` (unnamed in the engine map; `RET 4`): sets a node's cost.
const SET_COST_0052CE20: u32 = 0x0052_ce20;
/// `NavMeshPOVSearch::pStartPov` (Xbox PDB) +0x2078 and `pSolution` +0x2084.
const START_POV: u32 = 0x2078;
const SOLUTION: u32 = 0x2084;

fn debug_draw(e: &mut Engine) -> bool {
    e.global::<u8>(DEBUG_DRAW_011D6DCC) != 0
}

/// `operator new(0x38)` and, when that gives a block, the `NavMeshPOV`
/// constructor (`0069e2c0`); 0 otherwise.
fn new_pov(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
    if block != 0 {
        fn_0069e2c0(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32()
}

fn list_at(e: &mut Engine, lists: u32, index: u32) -> u32 {
    e.call(STEP_00558DD0, &args![lists, index]).u32()
}

/// The size of list `index` of `lists`.
fn inner_size(e: &mut Engine, lists: u32, index: u32) -> u32 {
    let list = list_at(e, lists, index);
    array_size(e, list)
}

/// The POV at `index` of list `list` of `lists`.
fn pov_at(e: &mut Engine, lists: u32, list: u32, index: u32) -> u32 {
    let inner = list_at(e, lists, list);
    let slot = e.call(ARRAY_AT_00877A30, &args![inner, index]).u32();
    e.mem.u32(slot)
}

/// The radius float of the request (`0069ef80` on `pRequest`), stored.
fn request_radius(e: &mut Engine, this: u32) -> u32 {
    let request = e.mem.u32(this + REQUEST);
    x87_store32(fn_0069ef80(e, Ptr::new(request)))
}

/// The first loop of `0069e3b0` over one triangle: the vertices `fill`
/// lists (`0069a830` or `0069a940`) go, as handles built at `handle`, into
/// list `index` of `target` unless list `index` of `first` (or, with
/// `second`, of that too) has them already.
fn collect_vertices(
    e: &mut Engine,
    triangles: u32,
    index: u32,
    vertices: u32,
    fill: u32,
    handle: u32,
    lists: (u32, Option<u32>),
) {
    let element = e.call(RECORD_AT_006950D0, &args![triangles, index]).u32();
    let triangle = u32::from(e.mem.u16(element + 4));
    let element = e.call(RECORD_AT_006950D0, &args![triangles, index]).u32();
    let navmesh = e.mem.u32(element);
    let count = e.call(fill, &args![navmesh, triangle, vertices]).u32();
    let (first, second) = lists;
    let mut vertex = 0u32;
    while vertex < count {
        let slot = e.call(VERTEX_AT_005EEDA0, &args![vertices, vertex]).u32();
        let id = u32::from(e.mem.u16(slot));
        let element = e.call(RECORD_AT_006950D0, &args![triangles, index]).u32();
        let navmesh = e.mem.u32(element);
        e.call(VERTEX_HANDLE_0069A660, &args![handle, navmesh, id]);
        let list = list_at(e, first, index);
        if !fn_006a1460(e, Ptr::new(list), handle) {
            let target = match second {
                None => first,
                Some(second) => {
                    let list = list_at(e, second, index);
                    if fn_006a1460(e, Ptr::new(list), handle) {
                        vertex = vertex.wrapping_add(1);
                        continue;
                    }
                    second
                }
            };
            let list = list_at(e, target, index);
            e.call(HANDLE_APPEND_006DBA30, &args![list, handle]);
        }
        vertex = vertex.wrapping_add(1);
    }
}

/// The duplicate test of `0069e3b0` for POV `l` of list `i` against POV
/// `k` of list `j`: closer than the squared distance at `EBP-0x40`
/// (strictly, ordered), the pair `(i, l)` (`004b0680`) is added to the
/// removals (`006a1a50`) unless they hold it (`006a1aa0` with `006a1b00`).
fn mark_duplicate(
    e: &mut Engine,
    ebp: u32,
    povs: u32,
    at: (u32, u32, u32, u32),
    temps: (u32, u32, u32),
) {
    let (i, l, j, k) = at;
    let (difference, first_key, second_key) = temps;
    let other = pov_at(e, povs, j, k);
    let mine = pov_at(e, povs, i, l);
    let delta = e
        .call(
            POINT3_MINUS_00439EF0,
            &args![mine + 4, difference, other + 4],
        )
        .u32();
    let distance = e.call(POINT3_LENGTH_SQUARED_004A7290, &args![delta]).f64();
    let limit = x87_load32(e.mem.u32(ebp - 0x40));
    if limit > distance {
        let removals = ebp - 0x1c;
        let key = e.call(PAIR_004B0680, &args![first_key, i, l]).u32();
        if fn_006a1aa0(e, Ptr::new(removals), key, 0, 0x006a_1b00) == 0xffff_ffff {
            let key = e.call(PAIR_004B0680, &args![second_key, i, l]).u32();
            fn_006a1a50(e, Ptr::new(removals), Ptr::new(key));
        }
    }
}

// Translated from 0069e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069e3b0 - its FS:[0] exception frame (the unwind state of the local arrays) is not translated
/// `NavMeshPOVSearch::BuildPOVListForTriangles` (Xbox PDB; `RET 8`:
/// triangles, the POV lists): sizes `POVs` to one list per triangle and
/// one more (`006a1720`), collects each triangle's corner vertices
/// (`0069a830`) and edge vertices (`0069a940`, those not corners) per list,
/// then makes the POVs (`0069e2c0`): for a corner with two edges
/// (`0069a9d0`) one of type 0 (type 1 when either edge's flag `0068f1d0`
/// is set) with the request's radius and both edges' directions (flat:
/// z 0); for an edge vertex two of type 2 (the request's radius, and the
/// radius at `01017b78` with the cost factor at `010162c0`), each in the
/// other's duplicates. POVs closer than the squared distance at `010181e8`
/// to a later one are deleted (`0069efa0`) and removed (`006bf8f0`), last
/// first. Returns true.
pub fn navmesh_pov_search_build_pov_list_for_triangles(
    e: &mut Engine,
    this: Ptr,
    triangles: Ptr,
    povs: Ptr,
) -> bool {
    with_frame(e, 0x150, &[triangles.addr(), povs.addr()], |e, ebp| {
        let base = this.addr();
        let all = base + POVS;
        let triangles = triangles.addr();
        let povs = povs.addr();
        let count = array_size(e, triangles);
        fn_006a1720(e, Ptr::new(all), count.wrapping_add(1), 1);
        let corners = ebp - 0x50;
        let edges = ebp - 0x2c;
        let size = array_size(e, all);
        fn_006a19b0(e, Ptr::new(corners), size);
        let size = array_size(e, all);
        fn_006a19b0(e, Ptr::new(edges), size);
        let vertices = ebp - 0x3c;
        e.call(VERTEX_LIST_CTOR_006917D0, &args![vertices]);

        let mut index = 0u32;
        while index < array_size(e, triangles) {
            collect_vertices(
                e,
                triangles,
                index,
                vertices,
                CORNER_VERTICES_0069A830,
                ebp - 0x64,
                (corners, None),
            );
            collect_vertices(
                e,
                triangles,
                index,
                vertices,
                EDGE_VERTICES_0069A940,
                ebp - 0x70,
                (corners, Some(edges)),
            );
            index = index.wrapping_add(1);
        }

        let mut list = 0u32;
        while list < array_size(e, corners) {
            let mut entry = 0u32;
            while entry < inner_size(e, corners, list) {
                let inner = list_at(e, corners, list);
                let handle = e.call(RECORD_AT_006950D0, &args![inner, entry]).u32();
                let edge_refs = ebp - 0x9c;
                let navmesh = e.mem.u32(handle);
                let vertex = u32::from(e.mem.u16(handle + 4));
                if e.call(CORNER_EDGES_0069A9D0, &args![navmesh, vertex, edge_refs])
                    .u8()
                    == 0
                {
                    entry = entry.wrapping_add(1);
                    continue;
                }
                let pov = new_pov(e);
                e.mem.set_u32(ebp - 0x98, pov);
                e.mem.set_u32(pov, 0);
                let navmesh = e.mem.u32(handle);
                let vertex = u32::from(e.mem.u16(handle + 4));
                let position = e
                    .call(VERTEX_POSITION_0068F0A0, &args![navmesh, vertex])
                    .u32();
                copy_words(e, pov + 4, position, 3);
                let radius = request_radius(e, base);
                e.mem.set_u32(pov + 0x10, radius);
                let vertex = e.mem.u16(handle + 4);
                e.mem.set_u16(pov + 0x34, vertex);
                let first_triangle = fn_0069ef40(e, Ptr::new(edge_refs)) & 0xffff;
                let navmesh = e.mem.u32(handle);
                let first = e
                    .call(TRIANGLE_005582F0, &args![navmesh, first_triangle])
                    .u32();
                e.mem.set_u32(ebp - 0x80, first);
                let second_triangle = fn_0069ef40(e, Ptr::new(edge_refs + 2)) & 0xffff;
                let navmesh = e.mem.u32(handle);
                let second = e
                    .call(TRIANGLE_005582F0, &args![navmesh, second_triangle])
                    .u32();
                e.mem.set_u32(ebp - 0x7c, second);
                let first_edge = fn_0069ef60(e, Ptr::new(edge_refs));
                let flagged = e
                    .call(
                        EDGE_FLAG_0068F1D0,
                        &args![e.mem.u32(ebp - 0x80), first_edge],
                    )
                    .u8()
                    != 0
                    || {
                        let second_edge = fn_0069ef60(e, Ptr::new(edge_refs + 2));
                        e.call(
                            EDGE_FLAG_0068F1D0,
                            &args![e.mem.u32(ebp - 0x7c), second_edge],
                        )
                        .u8()
                            != 0
                    };
                if flagged {
                    e.mem.set_u32(pov, 1);
                }
                for (refs, out) in [(edge_refs, ebp - 0x8c), (edge_refs + 2, ebp - 0x94)] {
                    let edge = fn_0069ef60(e, Ptr::new(refs));
                    let triangle = fn_0069ef40(e, Ptr::new(refs)) & 0xffff;
                    let navmesh = e.mem.u32(handle);
                    e.call(
                        GET_EDGE_VERTICES_0068F040,
                        &args![navmesh, out, triangle, edge],
                    );
                }
                for (out, temp, field) in [
                    (ebp - 0x8c, ebp - 0xdc, 0x14u32),
                    (ebp - 0x94, ebp - 0xe8, 0x20),
                ] {
                    let start = e.mem.u32(out);
                    let end = e.mem.u32(out + 4);
                    let direction = e
                        .call(POINT3_MINUS_00439EF0, &args![end, temp, start])
                        .u32();
                    copy_words(e, pov + field, direction, 3);
                }
                e.mem.set_u32(pov + 0x1c, x87_store32(0.0));
                e.mem.set_u32(pov + 0x28, x87_store32(0.0));
                let target = list_at(e, povs, list);
                e.call(ARRAY_PUSH_007CB2E0, &args![target, ebp - 0x98]);
                entry = entry.wrapping_add(1);
            }
            list = list.wrapping_add(1);
        }

        let mut list = 0u32;
        while list < array_size(e, edges) {
            let mut entry = 0u32;
            while entry < inner_size(e, edges, list) {
                let inner = list_at(e, edges, list);
                let handle = e.call(RECORD_AT_006950D0, &args![inner, entry]).u32();
                let near = new_pov(e);
                e.mem.set_u32(ebp - 0xb0, near);
                e.mem.set_u32(near, 2);
                let navmesh = e.mem.u32(handle);
                let vertex = u32::from(e.mem.u16(handle + 4));
                let position = e
                    .call(VERTEX_POSITION_0068F0A0, &args![navmesh, vertex])
                    .u32();
                copy_words(e, near + 4, position, 3);
                let radius = request_radius(e, base);
                e.mem.set_u32(near + 0x10, radius);
                let vertex = e.mem.u16(handle + 4);
                e.mem.set_u16(near + 0x34, vertex);
                let target = list_at(e, povs, list);
                e.call(ARRAY_PUSH_007CB2E0, &args![target, ebp - 0xb0]);

                let far = new_pov(e);
                e.mem.set_u32(ebp - 0xac, far);
                e.mem.set_u32(far, 2);
                let navmesh = e.mem.u32(handle);
                let vertex = u32::from(e.mem.u16(handle + 4));
                let position = e
                    .call(VERTEX_POSITION_0068F0A0, &args![navmesh, vertex])
                    .u32();
                copy_words(e, far + 4, position, 3);
                let radius = x87_quiet_bits(e.global::<u32>(EDGE_POV_RADIUS));
                e.mem.set_u32(far + 0x10, radius);
                let vertex = e.mem.u16(handle + 4);
                e.mem.set_u16(far + 0x34, vertex);
                let factor = x87_quiet_bits(e.global::<u32>(BLOCKED_COST_TYPE_5));
                e.mem.set_u32(far + 0x2c, factor);
                let target = list_at(e, povs, list);
                e.call(ARRAY_PUSH_007CB2E0, &args![target, ebp - 0xac]);
                let near_duplicates = e.mem.u32(e.mem.u32(ebp - 0xb0) + 0x30);
                e.call(ARRAY_PUSH_007CB2E0, &args![near_duplicates, ebp - 0xac]);
                let far_duplicates = e.mem.u32(e.mem.u32(ebp - 0xac) + 0x30);
                e.call(ARRAY_PUSH_007CB2E0, &args![far_duplicates, ebp - 0xb0]);
                entry = entry.wrapping_add(1);
            }
            list = list.wrapping_add(1);
        }

        let removals = ebp - 0x1c;
        fn_006a1a00(e, Ptr::new(removals));
        let limit = x87_quiet_bits(e.global::<u32>(DUPLICATE_DISTANCE_SQUARED));
        e.mem.set_u32(ebp - 0x40, limit);
        let mut i = 0u32;
        while i < array_size(e, povs) {
            let mut l = 0u32;
            while l < inner_size(e, povs, i) {
                let mut j = i;
                while j < array_size(e, povs) {
                    if j == i {
                        let mut k = l.wrapping_add(1);
                        while k < inner_size(e, povs, j) {
                            mark_duplicate(
                                e,
                                ebp,
                                povs,
                                (i, l, j, k),
                                (ebp - 0x104, ebp - 0x10c, ebp - 0x114),
                            );
                            k = k.wrapping_add(1);
                        }
                    } else {
                        let mut k = 0u32;
                        while k < inner_size(e, povs, j) {
                            mark_duplicate(
                                e,
                                ebp,
                                povs,
                                (i, l, j, k),
                                (ebp - 0x120, ebp - 0x128, ebp - 0x130),
                            );
                            k = k.wrapping_add(1);
                        }
                    }
                    j = j.wrapping_add(1);
                }
                l = l.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }

        let mut index = array_size(e, removals).wrapping_sub(1) as i32;
        while index >= 0 {
            let record = e
                .call(RECORD_AT_006950D0, &args![removals, index as u32])
                .u32();
            let position = e.mem.u32(record + 4);
            let record = e
                .call(RECORD_AT_006950D0, &args![removals, index as u32])
                .u32();
            let list = e.mem.u32(record);
            let pov = pov_at(e, all, list, position);
            if pov != 0 {
                fn_0069efa0(e, Ptr::new(pov), 1);
            }
            let record = e
                .call(RECORD_AT_006950D0, &args![removals, index as u32])
                .u32();
            let position = e.mem.u32(record + 4);
            let record = e
                .call(RECORD_AT_006950D0, &args![removals, index as u32])
                .u32();
            let list = e.mem.u32(record);
            let inner = list_at(e, all, list);
            e.call(ARRAY_REMOVE_006BF8F0, &args![inner, position, 1u32]);
            index = index.wrapping_sub(1);
        }

        fn_006a1a30(e, Ptr::new(removals));
        e.call(VERTEX_LIST_DTOR_00691800, &args![vertices]);
        fn_006a19e0(e, Ptr::new(edges));
        fn_006a19e0(e, Ptr::new(corners));
        true
    })
}

/// The debug drawing of one POV in `0069f010`: a sphere of the POV's
/// radius (`BSTestObjects::CreateSphere`, 10 by 10, coloured by the list
/// index) at its center, with a new property (`004391c0`) attached, no
/// lighting, prepared and updated, attached to the debug root.
fn draw_pov(e: &mut Engine, ebp: u32, lists: u32, list: u32, index: u32) {
    let alpha = x87_quiet_bits(e.global::<u32>(SPHERE_ALPHA));
    let count = array_size(e, lists);
    let fraction = x87_store32(x87_div(f64::from(list), f64::from(count)));
    let count = array_size(e, lists);
    let rest = x87_store32(x87_sub(1.0, x87_div(f64::from(list), f64::from(count))));
    let colour = ebp - 0x34;
    e.call(
        COLOUR_00414430,
        &args![colour, x87_store32(0.0), rest, fraction, alpha],
    );
    let pov = pov_at(e, lists, list, index);
    let radius = x87_quiet_bits(e.mem.u32(pov + 0x10));
    let sphere = e
        .call(CREATE_SPHERE_00C56680, &args![radius, 10u32, 10u32, colour])
        .u32();
    let pointer = ebp - 0x48;
    e.call(POINTER_SET_00633C90, &args![pointer, sphere]);
    let pov = pov_at(e, lists, list, index);
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.call(SET_TRANSLATE_00440460, &args![object, pov + 4]);
    let block = e.call(ALLOCATE_00AA13E0, &args![0x1cu32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CTOR_004391C0, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(ebp - 0x38, property);
    e.call(PROPERTY_SET_0049ED90, &args![property, 1u32]);
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.call(ATTACH_PROPERTY_00439410, &args![object, property]);
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.call(NO_LIGHTING_004B6E90, &args![object]);
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.call(PREPARE_OBJECT_00B57E30, &args![object, 0u32, 0u32]);
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.call(UPDATE_PROPERTIES_00A5A040, &args![object]);
    let zero = ebp - 0x44;
    e.call(
        THREE_WORD_CTOR_0043D410,
        &args![zero, x87_store32(0.0), 0u32, 0u32],
    );
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.call(ZERO_RECORD_STEP_00A59C60, &args![object, zero]);
    let root = e.call(FIRST_OF_00559450, &args![DEBUG_ROOT_011D6DDC]).u32();
    let object = e.call(FIRST_OF_00559450, &args![pointer]).u32();
    e.vcall(root, SLOT_ATTACH, &args![object, 1u32]);
    e.call(POINTER_RELEASE_0045CEC0, &args![pointer]);
}

/// The start and goal POVs of `0069f010`: a new POV of type `kind` at the
/// location's position (`005c3420` into `temp`), radius 0, vertex 0xFFFF,
/// stored at `this + field` and appended (`007cb2e0`) to the first list
/// (`006a1420`) or, with `next_to_last`, to the next to last.
fn place_end_pov(
    e: &mut Engine,
    base: u32,
    field: u32,
    kind: u32,
    location: u32,
    temp: u32,
    next_to_last: bool,
) {
    let pov = new_pov(e);
    e.mem.set_u32(base + field, pov);
    let pov = e.mem.u32(base + field);
    e.mem.set_u32(pov, kind);
    let position = e
        .call(LOCATION_POSITION_005C3420, &args![location, temp])
        .u32();
    let pov = e.mem.u32(base + field);
    copy_words(e, pov + 4, position, 3);
    let pov = e.mem.u32(base + field);
    e.mem.set_u32(pov + 0x10, x87_store32(0.0));
    let pov = e.mem.u32(base + field);
    e.mem.set_u16(pov + 0x34, 0xffff);
    let lists = base + POVS;
    let list = if next_to_last {
        let count = array_size(e, lists);
        list_at(e, lists, count.wrapping_sub(2))
    } else {
        fn_006a1420(e, Ptr::new(lists))
    };
    e.call(ARRAY_PUSH_007CB2E0, &args![list, base + field]);
}

// Translated from 0069f010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069f010 - its FS:[0] exception frame (the unwind state of the allocations and locations) is not translated
/// Unnamed in the engine map (`RET 0x14`: start, goal, triangles,
/// request, solution): runs the POV search. Keeps `pRequest` and
/// `pSolution`; builds the POV lists (`0069e3b0`; with debug drawing on,
/// a new debug root node first and a sphere per POV); sets the start and
/// goal locations (`006dcce0`) and resolves the start (`006dd6f0`): an
/// unresolved start (`006dd6a0`) gives false. Adds the start POV (type 3,
/// first list) and the goal POV (type 4, the next to last list), puts the
/// start node in (`006b32f0`, heuristic the distance between the
/// locations times the double at `0101de30`), searches (`006a1630` with
/// `Nodes`, `Links`, 1), appends both ends of every link to the solution
/// (`0069f920` with `PathingLocation`s made by `006dcf20`), hands the
/// debug root back when drawing, deletes every POV of `allPOVs`
/// (`0069efa0`) and returns the search's result.
pub fn fn_0069f010(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    goal: Ptr,
    triangles: Ptr,
    request: u32,
    solution: u32,
) -> bool {
    let words = [
        start.addr(),
        goal.addr(),
        triangles.addr(),
        request,
        solution,
    ];
    with_frame(e, 0x168, &words, |e, ebp| {
        let base = this.addr();
        e.mem.set_u32(base + REQUEST, request);
        e.mem.set_u32(base + SOLUTION, solution);
        if debug_draw(e) {
            if e.call(ROOT_TEST_0052AA80, &args![DEBUG_ROOT_011D6DDC, 0u32])
                .u8()
                != 0
            {
                let manager = e.call(PATH_MANAGER_INSTANCE, &args![]).u32();
                e.call(
                    PATH_MANAGER_DEBUG_006EBBD0,
                    &args![manager, DEBUG_ROOT_011D6DDC, 0u32],
                );
            }
            let block = e.call(ALLOCATE_00AA13E0, &args![0xacu32]).u32();
            let node = if block != 0 {
                e.call(NODE_CTOR_00A5ECB0, &args![block, 0u32]).u32()
            } else {
                0
            };
            e.call(ROOT_SET_0066B0D0, &args![DEBUG_ROOT_011D6DDC, node]);
        }
        let lists = base + POVS;
        navmesh_pov_search_build_pov_list_for_triangles(e, this, triangles, Ptr::new(lists));
        if debug_draw(e) {
            let mut list = 0u32;
            while list < array_size(e, lists) {
                let mut index = 0u32;
                while index < inner_size(e, lists, list) {
                    draw_pov(e, ebp, lists, list, index);
                    index = index.wrapping_add(1);
                }
                list = list.wrapping_add(1);
            }
        }
        e.call(
            LOCATION_ASSIGN_006DCCE0,
            &args![base + START_LOCATION, start],
        );
        e.call(LOCATION_ASSIGN_006DCCE0, &args![base + GOAL_LOCATION, goal]);
        e.call(
            RESOLVE_NAVMESH_INFO_006DD6F0,
            &args![base + START_LOCATION, 0u32],
        );
        if e.call(LOCATION_RESOLVED_006DD6A0, &args![base + START_LOCATION])
            .u8()
            == 0
        {
            return false;
        }
        place_end_pov(e, base, START_POV, 3, start.addr(), ebp - 0xd0, false);
        place_end_pov(e, base, GOAL_POV, 4, goal.addr(), ebp - 0xe4, true);

        let start_data = ebp - 0x18;
        let start_pov = e.mem.u32(base + START_POV);
        e.mem.set_u32(start_data, start_pov);
        e.mem.set_u32(start_data + 4, 0);
        e.mem.set_u32(start_data + 8, 0);
        let from = e
            .call(
                LOCATION_POSITION_005C3420,
                &args![base + START_LOCATION, ebp - 0xfc],
            )
            .u32();
        let to = e
            .call(
                LOCATION_POSITION_005C3420,
                &args![base + GOAL_LOCATION, ebp - 0xf0],
            )
            .u32();
        let difference = e
            .call(POINT3_MINUS_00439EF0, &args![to, ebp - 0x108, from])
            .u32();
        let distance = e.call(POINT3_LENGTH_00457990, &args![difference]).f64();
        let scale: f64 = e.global(GOAL_DISTANCE_SCALE);
        let heuristic = x87_quiet_bits(x87_store32(x87_mul(distance, scale)));
        let start_pov = e.mem.u32(base + START_POV);
        let key = fn_0069f900(e, this, start_pov, 0);
        e.call(
            START_NODE_006B32F0,
            &args![this, key, start_data, heuristic],
        );
        let found = fn_006a1630(e, this, base + NODES, base + LINKS, 1);
        e.mem.set_u8(ebp - 0x19, u8::from(found));

        let links = base + LINKS;
        let mut index = 0u32;
        while index < array_size(e, links) {
            for (navmesh_field, position_field, location) in
                [(0x10u32, 0x1cu32, ebp - 0x74), (0x28, 0x34, ebp - 0x9c)]
            {
                let link = e.call(LINK_AT_006B30E0, &args![links, index]).u32();
                let navmesh = e.mem.u32(link + navmesh_field);
                let info = e.call(NAVMESH_INFO_0068EFB0, &args![navmesh]).u32();
                let link = e.call(LINK_AT_006B30E0, &args![links, index]).u32();
                e.call(
                    LOCATION_CTOR_006DCF20,
                    &args![location, link + position_field, info],
                );
                fn_0069f920(e, Ptr::new(solution), location, ORIGIN_011F426C);
            }
            e.call(LOCATION_DTOR_004FF7E0, &args![ebp - 0x9c]);
            e.call(LOCATION_DTOR_004FF7E0, &args![ebp - 0x74]);
            index = index.wrapping_add(1);
        }
        if debug_draw(e) {
            let root = e.call(FIRST_OF_00559450, &args![DEBUG_ROOT_011D6DDC]).u32();
            e.call(UPDATE_PROPERTIES_00A5A040, &args![root]);
            let zero = ebp - 0xa8;
            e.call(
                THREE_WORD_CTOR_0043D410,
                &args![zero, x87_store32(0.0), 0u32, 0u32],
            );
            let root = e.call(FIRST_OF_00559450, &args![DEBUG_ROOT_011D6DDC]).u32();
            e.call(ZERO_RECORD_STEP_00A59C60, &args![root, zero]);
            let manager = e.call(PATH_MANAGER_INSTANCE, &args![]).u32();
            e.call(
                PATH_MANAGER_DEBUG_006EBBD0,
                &args![manager, DEBUG_ROOT_011D6DDC, 1u32],
            );
        }
        let all = base + ALL_POVS;
        let mut index = 0u32;
        while index < array_size(e, all) {
            let slot = e.call(ARRAY_AT_00877A30, &args![all, index]).u32();
            let pov = e.mem.u32(slot);
            if pov != 0 {
                fn_0069efa0(e, Ptr::new(pov), 1);
            }
            index = index.wrapping_add(1);
        }
        e.mem.u8(ebp - 0x19) != 0
    })
}

/// The values `connect_tangent` shares across a POV's tangents in
/// `0069f9d0`.
struct Connection {
    ebp: u32,
    node: u32,
    data: u32,
    link: u32,
    other: u32,
    list: u32,
    connections: u32,
}

/// The five words of the 0x14-byte segment at `segment`, copied as
/// `006a0620` copies it into the outgoing arguments.
fn segment_words(e: &mut Engine, segment: u32) -> [u32; 5] {
    e.with_stack(0x14, |e, copy| {
        fn_006a0620(e, copy, Ptr::new(segment));
        let mut words = [0u32; 5];
        for (i, word) in words.iter_mut().enumerate() {
            *word = e.mem.u32(copy.addr() + 4 * i as u32);
        }
        words
    })
}

/// The arc drawn for the wedge segment at `EBP-0x130` around circle A.
fn draw_wedge_arc(e: &mut Engine, ebp: u32, colour: u32) {
    let radius = f32::from_bits(x87_quiet_bits(e.mem.u32(ebp - 0x38)));
    let flag = e.mem.u32(ebp - 0x120);
    navmesh_pov_search_draw_arc(
        e,
        Ptr::new(ebp - 0x44),
        radius,
        Ptr::new(ebp - 0x130),
        Ptr::new(ebp - 0x128),
        flag,
        colour,
    );
}

/// The absolute height (`00408840` on the float at +8) of `point - other`
/// (`00439ef0` into `temp`): whether it is below the limit at `0104ed58`
/// or unordered (the exe's `TEST AH,1`).
fn height_ok(e: &mut Engine, point: u32, temp: u32, other: u32) -> bool {
    let difference = e
        .call(POINT3_MINUS_00439EF0, &args![point, temp, other])
        .u32();
    let height = x87_quiet_bits(e.mem.u32(difference + 8));
    let height = e.call(ABS_00408840, &args![height]).f64();
    let limit: f64 = e.global(HEIGHT_LIMIT);
    !(height >= limit)
}

/// One tangent `k` of `0069f9d0` between the node's POV circle (A at
/// `EBP-0x44`) and `other`'s (B at `EBP-0x88`); false when the whole
/// function returns false.
fn connect_tangent(e: &mut Engine, this: Ptr, c: &Connection, k: u32) -> bool {
    let base = this.addr();
    let ebp = c.ebp;
    let node = Ptr::new(c.node);
    let entry = (ebp - 0x74).wrapping_add(k.wrapping_mul(0x14));
    let first = entry + 4;
    let second = entry + 0xc;
    if fn_006a1b60(e, node) != 0 {
        let side = ebp - 0x138;
        e.call(IDENTITY_006815C0, &args![side]);
        e.call(POINT_SET_006985F0, &args![side, c.link + 8]);
        let step = f32::from_bits(x87_quiet_bits(e.global::<u32>(WEDGE_STEP)));
        if e.mem.u32(c.data + 4) == 0 {
            let rotation = fn_006a04e0(e, Ptr::new(ebp - 0x178), step);
            fn_006a0520(e, Ptr::new(side), rotation);
        } else {
            let rotation = fn_006a04e0(e, Ptr::new(ebp - 0x180), step);
            fn_006a0580(e, Ptr::new(side), rotation);
        }
        let wedge = ebp - 0x14c;
        let direction = e.mem.u32(c.data + 4);
        fn_006a05e0(
            e,
            Ptr::new(wedge),
            Ptr::new(side),
            Ptr::new(c.link + 8),
            direction,
        );
        if fn_006a0660(e, Ptr::new(wedge), Ptr::new(first)) {
            return true;
        }
    }
    copy_words(e, ebp - 0xec, ORIGIN_011F426C, 3);
    copy_words(e, ebp - 0xa0, ORIGIN_011F426C, 3);
    let radius_a = x87_quiet_bits(e.mem.u32(ebp - 0x38));
    let on_a = e
        .call(SCALED_POINT_0069E0C0, &args![first, ebp - 0x1b0, radius_a])
        .u32();
    let radius_b = x87_quiet_bits(e.mem.u32(ebp - 0x7c));
    let on_b = e
        .call(SCALED_POINT_0069E0C0, &args![second, ebp - 0x18c, radius_b])
        .u32();
    let touch_b = e
        .call(POINT3_PLUS_00439E90, &args![ebp - 0x88, ebp - 0x198, on_b])
        .u32();
    let from_a = e
        .call(
            POINT3_MINUS_00439EF0,
            &args![touch_b, ebp - 0x1a4, ebp - 0x44],
        )
        .u32();
    let tangent = ebp - 0x11c;
    e.call(POINT3_MINUS_00439EF0, &args![from_a, tangent, on_a]);
    let other = c.other;
    let pov = e.mem.u32(c.data);
    if e.mem.u32(pov) != 3 && e.mem.u32(other) == 0 {
        e.mem.set_u32(ebp - 0x168, x87_store32(1.0));
        e.mem.set_u32(ebp - 0x2c, x87_store32(0.0));
        copy_words(e, ebp - 0x164, other + 0x14, 3);
        e.mem.set_u32(ebp - 0x15c, x87_store32(0.0));
        copy_words(e, ebp - 0x158, other + 0x20, 3);
        e.mem.set_u32(ebp - 0x150, x87_store32(0.0));
        let cross = e
            .call(CROSS_004B3800, &args![ebp - 0x164, ebp - 0x1bc, tangent])
            .u32();
        copy_words(e, ebp - 0xec, cross, 3);
        let cross = e
            .call(CROSS_004B3800, &args![ebp - 0x158, ebp - 0x1c8, tangent])
            .u32();
        copy_words(e, ebp - 0xa0, cross, 3);
        let one = x87_load32(e.mem.u32(ebp - 0x168));
        if one < x87_load32(e.mem.u32(ebp - 0xe4)) && one < x87_load32(e.mem.u32(ebp - 0x98)) {
            return true;
        }
    }
    let pov = e.mem.u32(c.data);
    if e.mem.u32(pov) != 3 && e.mem.u32(other) == 0 {
        let height = if e.mem.u32(entry) == 0 {
            x87_load32(e.mem.u32(ebp - 0x98))
        } else {
            x87_load32(e.mem.u32(ebp - 0xe4))
        };
        let limit: f64 = e.global(CROSS_LIMIT);
        if height < limit {
            return true;
        }
    }
    let segment = ebp - 0x130;
    let direction = e.mem.u32(c.data + 4);
    fn_006a05e0(
        e,
        Ptr::new(segment),
        Ptr::new(c.link + 8),
        Ptr::new(first),
        direction,
    );
    let from = ebp - 0x110;
    fn_006a0480(e, Ptr::new(from));
    if fn_006a1b60(e, node) == 0 {
        let holder = ebp - 0x170;
        e.call(HOLDER_CTOR_0042FB00, &args![holder]);
        let found = e
            .call(
                GET_NAVMESH_AND_TRIANGLE_006DD640,
                &args![base + START_LOCATION, holder, ebp - 0x16c],
            )
            .u8();
        if found == 0 {
            e.call(HOLDER_DTOR_0042FA40, &args![holder]);
            return false;
        }
        let navmesh = e.call(FIRST_OF_00559450, &args![holder]).u32();
        e.mem.set_u32(from, navmesh);
        let triangle = e.mem.u16(ebp - 0x16c);
        e.mem.set_u16(from + 4, triangle);
        e.mem.set_u32(from + 8, 0xffff_ffff);
        let position = e
            .call(
                LOCATION_POSITION_005C3420,
                &args![base + START_LOCATION, ebp - 0x1d8],
            )
            .u32();
        copy_words(e, from + 0xc, position, 3);
        e.call(HOLDER_DTOR_0042FA40, &args![holder]);
    } else {
        e.call(RECORD_COPY_006CF900, &args![from, c.link + 0x28]);
    }
    e.mem.set_u32(ebp - 0xe0, x87_store32(1.0));
    let reached = ebp - 0xdc;
    fn_006a0480(e, Ptr::new(reached));
    let blocked: f64 = e.global(BLOCKED);
    if fn_006a1b60(e, node) == 0 {
        e.call(RECORD_COPY_006CF900, &args![reached, from]);
    } else {
        let words = segment_words(e, segment);
        let clear = navmesh_pov_search_check_arc_against_navmesh2(
            e,
            this,
            Ptr::new(from),
            Ptr::new(ebp - 0x44),
            words[0],
            words[1],
            words[2],
            words[3],
            words[4],
            Ptr::new(reached),
        );
        if !clear {
            if debug_draw(e) {
                draw_wedge_arc(e, ebp, COLOUR_BLOCKED);
            }
            return true;
        }
        if !height_ok(e, reached + 0xc, ebp - 0x1e8, ebp - 0x44) {
            return true;
        }
        let words = segment_words(e, segment);
        let pov = e.mem.u32(c.data);
        let cost = fn_006a0940(
            e, this, pov, words[0], words[1], words[2], words[3], words[4], other,
        );
        e.mem.set_u32(ebp - 0xe0, x87_store32(cost));
        if x87_load32(e.mem.u32(ebp - 0xe0)) == blocked {
            if debug_draw(e) {
                draw_wedge_arc(e, ebp, COLOUR_COVERED);
            }
            return true;
        }
    }
    let target = ebp - 0xc4;
    fn_006a0480(e, Ptr::new(target));
    let radius_b = x87_quiet_bits(e.mem.u32(ebp - 0x7c));
    let on_b = e
        .call(SCALED_POINT_0069E0C0, &args![second, ebp - 0x1f8, radius_b])
        .u32();
    let touch_b = ebp - 0xf8;
    e.call(POINT3_PLUS_00439E90, &args![ebp - 0x88, touch_b, on_b]);
    let clear = navmesh_pov_search_check_segment_against_navmesh(
        e,
        this,
        Ptr::new(reached),
        Ptr::new(touch_b),
        Ptr::new(target),
    );
    let start_point = reached + 0xc;
    let end_point = target + 0xc;
    if !clear {
        if debug_draw(e) {
            navmesh_pov_search_draw_line(
                e,
                Ptr::new(start_point),
                Ptr::new(end_point),
                COLOUR_BLOCKED,
            );
        }
        return true;
    }
    if !height_ok(e, end_point, ebp - 0x204, ebp - 0x88) {
        return true;
    }
    let pov = e.mem.u32(c.data);
    let cost = fn_006a0ac0(e, this, pov, start_point, end_point, other);
    e.mem.set_u32(ebp - 0xa4, x87_store32(cost));
    if x87_load32(e.mem.u32(ebp - 0xa4)) == blocked {
        if debug_draw(e) {
            navmesh_pov_search_draw_line(
                e,
                Ptr::new(start_point),
                Ptr::new(end_point),
                COLOUR_COVERED,
            );
        }
        return true;
    }
    if debug_draw(e) {
        if fn_006a1b60(e, node) != 0 {
            draw_wedge_arc(e, ebp, COLOUR_CLEAR);
        }
        navmesh_pov_search_draw_line(e, Ptr::new(start_point), Ptr::new(end_point), COLOUR_CLEAR);
    }
    let so_far = e.call(NODE_COST_006A7F50, &args![node]).f64();
    e.mem.set_u32(ebp - 0xac, x87_store32(so_far));
    if fn_006a1b60(e, node) != 0 {
        let pov = e.mem.u32(c.data);
        let radius = f32::from_bits(x87_quiet_bits(e.mem.u32(pov + 0x10)));
        let arc = fn_006a07a0(e, Ptr::new(segment), radius);
        let factor = x87_load32(e.mem.u32(other + 0x2c));
        let sum = x87_add(factor, x87_m32(e.mem.u32(ebp - 0xe0)));
        let sum = x87_add(sum, x87_m32(e.mem.u32(ebp - 0xa4)));
        let total = x87_add(x87_mul(arc, sum), x87_m32(e.mem.u32(ebp - 0xac)));
        e.mem.set_u32(ebp - 0xac, x87_store32(total));
    }
    let difference = e
        .call(
            POINT3_MINUS_00439EF0,
            &args![end_point, ebp - 0x210, start_point],
        )
        .u32();
    let length = e.call(POINT3_LENGTH_00457990, &args![difference]).f64();
    let total = x87_add(length, x87_m32(e.mem.u32(ebp - 0xac)));
    e.mem.set_u32(ebp - 0xac, x87_store32(total));
    let cost = x87_quiet_bits(e.mem.u32(ebp - 0xac));
    let direction = e.mem.u32(entry);
    let key = fn_0069f900(e, this, other, direction);
    let result = e
        .call(NODE_FOR_KEY_006B3420, &args![this, key, cost, ebp - 0xa8])
        .u32();
    match result {
        0 => {
            let new_node = e.mem.u32(ebp - 0xa8);
            let direction = e.mem.u32(entry);
            fn_006a08d0(e, this, Ptr::new(new_node), other, direction, c.list);
        }
        1 => {}
        _ => return true,
    }
    let new_node = e.mem.u32(ebp - 0xa8);
    e.call(RECORD_COPY_006CF900, &args![new_node + 0x24, reached]);
    let new_node = e.mem.u32(ebp - 0xa8);
    e.call(RECORD_COPY_006CF900, &args![new_node + 0x3c, target]);
    let new_node = e.mem.u32(ebp - 0xa8);
    e.call(POINT_SET_006985F0, &args![new_node + 0x14, first]);
    let new_node = e.mem.u32(ebp - 0xa8);
    e.call(POINT_SET_006985F0, &args![new_node + 0x1c, second]);
    let new_node = e.mem.u32(ebp - 0xa8);
    let cost = x87_quiet_bits(e.mem.u32(ebp - 0xac));
    e.call(SET_COST_0052CE20, &args![new_node, cost]);
    e.call(ARRAY_PUSH_007CB2E0, &args![c.connections, ebp - 0xa8]);
    true
}

/// The connections of the node's POV to `other` in `0069f9d0`; false when
/// the whole function returns false.
fn connect_pov(e: &mut Engine, this: Ptr, c: &Connection) -> bool {
    let ebp = c.ebp;
    let node = Ptr::new(c.node);
    let other = c.other;
    if other == e.mem.u32(c.data) {
        return true;
    }
    if fn_006a1b60(e, node) != 0 {
        let parent = fn_006a1b60(e, node);
        let parent_data = e.call(PLUS_EIGHT_00413F40, &args![parent]).u32();
        if other == e.mem.u32(parent_data) {
            return true;
        }
    }
    let pov = e.mem.u32(c.data);
    e.call(
        POINT3_MINUS_00439EF0,
        &args![other + 4, ebp - 0x34, pov + 4],
    );
    let horizontal = e.call(LENGTH_2D_00595C80, &args![ebp - 0x34]).f64();
    e.mem.set_u32(ebp - 0x48, x87_store32(horizontal));
    let height = e.mem.u32(ebp - 0x2c);
    e.mem.set_u32(
        ebp - 0x90,
        x87_store32(x87_mul(x87_load32(height), x87_m32(height))),
    );
    let scale: f64 = e.global(HEIGHT_SCALE);
    let limit = x87_mul(x87_mul(x87_load32(e.mem.u32(ebp - 0x48)), scale), scale);
    let squared = x87_load32(e.mem.u32(ebp - 0x90));
    if squared > limit {
        return true;
    }
    let pov = e.mem.u32(c.data);
    let radius = f32::from_bits(x87_quiet_bits(e.mem.u32(pov + 0x10)));
    fn_006a04b0(e, Ptr::new(ebp - 0x44), Ptr::new(pov + 4), radius);
    let radius = f32::from_bits(x87_quiet_bits(e.mem.u32(other + 0x10)));
    fn_006a04b0(e, Ptr::new(ebp - 0x88), Ptr::new(other + 4), radius);
    fn_006a0870(e, Ptr::new(ebp - 0x78));
    let direction = e.mem.u32(c.data + 4);
    let count = e
        .call(
            TANGENTS_006BB570,
            &args![ebp - 0x44, ebp - 0x88, direction, ebp - 0x78],
        )
        .u32();
    e.mem.set_u32(ebp - 0x4c, count);
    if count == 0 {
        return true;
    }
    let mut k = 0i32;
    while k < e.mem.u32(ebp - 0x4c) as i32 {
        if !connect_tangent(e, this, c, k as u32) {
            return false;
        }
        k = k.wrapping_add(1);
    }
    true
}

// Translated from 0069f9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069f9d0 - its FS:[0] exception frame (the unwind state of the navmesh holder) is not translated
/// `NavMeshPOVSearch::GetNodeConnections` (Xbox PDB; `RET 8`: node,
/// connections): for every POV of the node's list (`iMasterIndex`) and the
/// lists after it, other than the node's own POV and its parent's, whose
/// height difference squared is not above the horizontal distance times
/// the double at `010290f0` twice: the tangents between the two circles
/// (`006bb570`), each skipped when it starts inside the parent link's
/// wedge or (for a corner POV and not the start) points into the corner's
/// edges (`004b3800` against the limits). The path along the arc (from the
/// parent link, `006a0c50`, costed by `006a0940`) and the tangent
/// (`006a0ef0`, costed by `006a0ac0`) must be clear of the navmesh, below
/// the height limit at `0104ed58` and not blocked; the cost (`006a7f50`,
/// the arc's angle `006a07a0` times the cost factors, plus the length)
/// goes to the node for the key (`006b3420`; a new one is set up by
/// `006a08d0`), with both records, both tangent points and the cost
/// (`0052ce20`), appended to `connections`. Without a parent the arc
/// starts at the start location (`GetNavMeshAndTriangle`; when that fails
/// the result is false). With debug drawing on, every check is drawn.
/// Returns true.
pub fn navmesh_pov_search_get_node_connections(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    connections: Ptr,
) -> bool {
    with_frame(e, 0x220, &[node.addr(), connections.addr()], |e, ebp| {
        let base = this.addr();
        let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
        let link = e.call(PARENT_LINK_007D6BB0, &args![node]).u32();
        let lists = base + POVS;
        let count = array_size(e, lists);
        let mut list = e.mem.u32(data + 8);
        while list < count {
            let inner = list_at(e, lists, list);
            let size = array_size(e, inner);
            let mut index = 0u32;
            while index < size {
                let slot = e.call(ARRAY_AT_00877A30, &args![inner, index]).u32();
                let other = e.mem.u32(slot);
                let connection = Connection {
                    ebp,
                    node: node.addr(),
                    data,
                    link,
                    other,
                    list,
                    connections: connections.addr(),
                };
                if !connect_pov(e, this, &connection) {
                    return false;
                }
                index = index.wrapping_add(1);
            }
            list = list.wrapping_add(1);
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_1440, fn_006a1440(Ptr, u32) -> u32),
        entry!(0x006a_1490, fn_006a1490(Ptr, u32) -> u32),
        entry!(0x006a_1400, fn_006a1400(Ptr, u32) -> u32),
        entry!(0x006a_1b60, fn_006a1b60(Ptr) -> u32),
        entry!(0x006a_0480, fn_006a0480(Ptr) -> Ptr),
        entry!(0x006a_1f90, fn_006a1f90(u32, Ptr) -> u32),
        entry!(0x0069_ef80, fn_0069ef80(Ptr) -> f64),
        entry!(0x0069_e390, fn_0069e390(Ptr) -> u32),
        entry!(0x006a_1420, fn_006a1420(Ptr) -> u32),
        entry!(0x006a_1f70, fn_006a1f70(Ptr) -> Ptr),
        entry!(0x0069_f900, fn_0069f900(Ptr, u32, u32) -> u32),
        entry!(0x006a_28b0, fn_006a28b0(Ptr, u32, u32) -> u32),
        entry!(0x006a_0840, fn_006a0840(Ptr) -> f64),
        entry!(0x006a_0870, fn_006a0870(Ptr) -> Ptr),
        entry!(0x006a_04b0, fn_006a04b0(Ptr, Ptr, f32) -> Ptr),
        entry!(0x006a_04e0, fn_006a04e0(Ptr, f32) -> Ptr),
        entry!(0x006a_05e0, fn_006a05e0(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x006a_0620, fn_006a0620(Ptr, Ptr) -> Ptr),
        entry!(0x006a_06f0, fn_006a06f0(Ptr, Ptr) -> bool),
        entry!(0x006a_0660, fn_006a0660(Ptr, Ptr) -> bool),
        entry!(0x006a_07a0, fn_006a07a0(Ptr, f32) -> f64),
        entry!(0x006a_0c10, fn_006a0c10(Ptr, u32, Ptr) -> Ptr),
        entry!(0x006a_0e50, fn_006a0e50(Ptr, Ptr) -> Ptr),
        entry!(0x006a_0e90, fn_006a0e90(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006a_1fb0, fn_006a1fb0(Ptr, Ptr)),
        entry!(0x006a_2630, fn_006a2630(Ptr, u32, u32)),
        entry!(0x006a_2130, fn_006a2130(Ptr, u32, u32)),
        entry!(0x006a_2270, fn_006a2270(Ptr, u32, u32, u32)),
        entry!(0x006a_21c0, fn_006a21c0(Ptr, u32, u32)),
        entry!(0x006a_2770, fn_006a2770(Ptr, u32, u32)),
        entry!(0x006a_1920, fn_006a1920(Ptr) -> u32),
        entry!(0x0069_f920, fn_0069f920(Ptr, u32, u32) -> u32),
        entry!(0x006a_14e0, fn_006a14e0(Ptr)),
        entry!(0x006a_1700, fn_006a1700(Ptr)),
        entry!(0x006a_1890, fn_006a1890(Ptr)),
        entry!(0x006a_18e0, fn_006a18e0(Ptr)),
        entry!(0x006a_19e0, fn_006a19e0(Ptr)),
        entry!(0x006a_1a30, fn_006a1a30(Ptr)),
        entry!(0x006a_2580, fn_006a2580(Ptr)),
        entry!(0x006a_2900, fn_006a2900(Ptr)),
        entry!(0x0069_ef40, fn_0069ef40(Ptr) -> u32),
        entry!(0x0069_ef60, fn_0069ef60(Ptr) -> u32),
        entry!(0x0069_efd0, fn_0069efd0(Ptr)),
        entry!(0x0069_efa0, fn_0069efa0(Ptr, u32) -> Ptr),
        entry!(0x006a_15b0, fn_006a15b0(Ptr)),
        entry!(0x0069_e200, fn_0069e200(Ptr)),
        entry!(
            0x0069_e1d0,
            navmesh_pov_search_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006a_14b0, fn_006a14b0(Ptr) -> Ptr),
        entry!(0x0069_e2c0, fn_0069e2c0(Ptr) -> Ptr),
        entry!(0x006a_0520, fn_006a0520(Ptr, Ptr) -> Ptr),
        entry!(0x006a_0580, fn_006a0580(Ptr, Ptr) -> Ptr),
        entry!(0x006a_08a0, navmesh_pov_search_is_goal_node(Ptr, Ptr) -> bool),
        entry!(0x006a_08d0, fn_006a08d0(Ptr, Ptr, u32, u32, u32)),
        entry!(0x006a_0940, fn_006a0940(Ptr, u32, u32, u32, u32, u32, u32, u32) -> f64),
        entry!(0x006a_0ac0, fn_006a0ac0(Ptr, u32, u32, u32, u32) -> f64),
        entry!(
            0x006a_0ef0,
            navmesh_pov_search_check_segment_against_navmesh(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006a_0c50,
            navmesh_pov_search_check_arc_against_navmesh2(Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, Ptr) -> bool
        ),
        entry!(0x006a_1330, navmesh_pov_search_draw_line(Ptr, Ptr, u32)),
        entry!(
            0x006a_1180,
            navmesh_pov_search_draw_arc(Ptr, f32, Ptr, Ptr, u32, u32)
        ),
        entry!(0x006a_1900, fn_006a1900(Ptr, u32, u32) -> u32),
        entry!(0x006a_1b20, fn_006a1b20(Ptr, Ptr) -> bool),
        entry!(0x006a_1b00, fn_006a1b00(Ptr, Ptr) -> bool),
        entry!(0x006a_1b80, fn_006a1b80(u32, u32) -> u32),
        entry!(0x006a_1aa0, fn_006a1aa0(Ptr, u32, u32, u32) -> u32),
        entry!(0x006a_1460, fn_006a1460(Ptr, u32) -> bool),
        entry!(0x006a_2440, fn_006a2440(Ptr) -> Ptr),
        entry!(0x006a_2000, fn_006a2000(Ptr, u32, u32)),
        entry!(0x006a_23a0, fn_006a23a0(Ptr, u32, u32)),
        entry!(0x006a_2880, fn_006a2880(Ptr) -> Ptr),
        entry!(0x006a_26d0, fn_006a26d0(Ptr, u32, u32)),
        entry!(0x006a_20a0, fn_006a20a0(Ptr, u32, u32)),
        entry!(0x006a_2310, fn_006a2310(Ptr, u32, u32)),
        entry!(0x006a_2470, fn_006a2470(Ptr, u32, u32)),
        entry!(0x006a_16d0, fn_006a16d0(Ptr) -> Ptr),
        entry!(0x006a_1860, fn_006a1860(Ptr) -> Ptr),
        entry!(0x006a_18b0, fn_006a18b0(Ptr) -> Ptr),
        entry!(0x006a_19b0, fn_006a19b0(Ptr, u32) -> Ptr),
        entry!(0x006a_1a00, fn_006a1a00(Ptr) -> Ptr),
        entry!(0x006a_2850, fn_006a2850(Ptr) -> Ptr),
        entry!(0x006a_2500, fn_006a2500(Ptr) -> Ptr),
        entry!(0x006a_25d0, fn_006a25d0(Ptr)),
        entry!(0x006a_1500, fn_006a1500(Ptr) -> Ptr),
        entry!(0x006a_1cf0, fn_006a1cf0(Ptr) -> u32),
        entry!(0x006a_1630, fn_006a1630(Ptr, u32, u32, u8) -> bool),
        entry!(0x006a_1720, fn_006a1720(Ptr, u32, u8)),
        entry!(0x006a_1a50, fn_006a1a50(Ptr, Ptr) -> u32),
        entry!(
            0x006a_1ba0,
            bs_simple_array_navmesh_pov_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_1bd0,
            astar_search_navmesh_pov_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_1c00,
            bs_simple_array_navmesh_pov_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_1c30,
            bs_simple_array_navmesh_pov_search_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_1c60,
            bs_simple_array_navmesh_pov_search_link_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_1c90,
            bs_simple_array_vert_handle_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_1cc0,
            bs_simple_array_remove_index_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_25a0,
            bs_simple_array_astar_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_2820,
            bs_scrap_array_astar_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_28d0,
            bs_simple_array_vert_handle_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0069_e3b0,
            navmesh_pov_search_build_pov_list_for_triangles(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x0069_f010, fn_0069f010(Ptr, Ptr, Ptr, Ptr, u32, u32) -> bool),
        entry!(
            0x0069_f9d0,
            navmesh_pov_search_get_node_connections(Ptr, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006a14e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_14e0, ARRAY_VTABLE_0106B3B0, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a1700_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1700, ARRAY_VTABLE_0106B3D4, ARRAY_RELEASE_0049FAF0);
    }

    #[test]
    fn fn_006a1890_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1890, ARRAY_VTABLE_0106B3E8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a18e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_18e0, ARRAY_VTABLE_0106B3FC, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a19e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_19e0, ARRAY_VTABLE_0106B410, ARRAY_RELEASE_0049FAF0);
    }

    #[test]
    fn fn_006a1a30_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1a30, ARRAY_VTABLE_0106B424, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a2580_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_2580, ARRAY_VTABLE_0106B44C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a2900_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_2900, ARRAY_VTABLE_0106B460, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod pov_search_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    /// The 2D helpers: copy, difference and a stand-in quadrant (0 for a
    /// non-negative first coordinate, else 1).
    fn points(e: &mut Engine) {
        e.register(POINT_SET_006985F0, |e, a| {
            let (x, y) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], x);
            e.mem.set_u32(a[0] + 4, y);
            Ret::default()
        });
        e.register(POINT_MINUS_00698620, |e, a| {
            let x = e.mem.f32(a[0]) - e.mem.f32(a[2]);
            let y = e.mem.f32(a[0] + 4) - e.mem.f32(a[2] + 4);
            e.mem.set_f32(a[1], x);
            e.mem.set_f32(a[1] + 4, y);
            Ret::default()
        });
        e.register(POINT_QUADRANT_00698680, |e, a| {
            let quadrant = u32::from(e.mem.f32(a[0]) < 0.0);
            quadrant.into_ret()
        });
    }

    #[test]
    fn element_addresses_and_small_getters() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x1000);
        assert_eq!(
            e.call(0x006a_1440, &args![array, 3u32]).u32(),
            0x1000 + 0x24
        );
        assert_eq!(
            e.call(0x006a_1490, &args![array, 3u32]).u32(),
            0x1000 + 0x6c
        );
        assert_eq!(
            e.call(0x006a_1400, &args![array, 2u32]).u32(),
            0x1000 + 0x48
        );
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x54, 0x5454);
        assert_eq!(e.call(0x006a_1b60, &args![object]).u32(), 0x5454);
        e.mem.set_u32(object + 0x68, 0xff80_0001);
        let loaded = e.call(0x0069_ef80, &args![object]).f64();
        assert!(loaded.is_nan());
        e.mem.set_f32(object + 0x68, 1.5);
        assert_eq!(e.call(0x0069_ef80, &args![object]).f64(), 1.5);
        assert_eq!(
            e.call(0x0069_f900, &args![0u32, 0xffff_fff0u32, 0x20u32])
                .u32(),
            0x10
        );
    }

    #[test]
    fn forwarding_wrappers_pass_their_words() {
        let mut e = Engine::new();
        e.register(BASE_CTOR_006240D0, |_, _| Ret::default());
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        e.register(SCRAP_HEAP_ALLOCATE, |_, _| 0x7777u32.into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STEP_00558DD0, |_, _| 0x5858u32.into_ret());
        e.register(SCRAP_INIT_00AA5410, |_, _| Ret::default());
        e.register(BYTES_0042F5D0, |_, a| (a[0] + a[1]).into_ret());
        e.register(CONSTRUCT_ARRAY_00401050, |_, _| Ret::default());
        e.map(0x010a_2000, 0x1000);
        e.set_global::<u32>(SCRAP_ALIGNMENT, 4);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x94, 0x9494);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_0480, &args![this]).u32(), this);
        assert_eq!(
            e.call(0x006a_1f90, &args![0x30u32, 0x4000u32]).u32(),
            0x7777
        );
        assert_eq!(e.call(0x0069_e390, &args![this]).u32(), 0x9494);
        assert_eq!(e.call(0x006a_1420, &args![this]).u32(), 0x5858);
        assert_eq!(e.call(0x006a_1f70, &args![this]).u32(), this);
        assert_eq!(
            e.call(0x006a_28b0, &args![0u32, 0x100u32, 3u32]).u32(),
            0x118
        );
        assert_eq!(e.call(0x006a_0870, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BASE_CTOR_006240D0), vec![vec![this]]);
        assert_eq!(calls_to(&log, IDENTITY_006815C0), vec![vec![this + 0xc]]);
        assert_eq!(
            calls_to(&log, SCRAP_HEAP_ALLOCATE),
            vec![vec![0x4000, 0x30, 4]]
        );
        assert_eq!(calls_to(&log, STEP_00558DD0), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&log, SCRAP_INIT_00AA5410),
            vec![vec![this, 0xd000]]
        );
        assert_eq!(
            calls_to(&log, CONSTRUCT_ARRAY_00401050),
            vec![vec![this + 4, 0x14, 2, ELEMENT_CTOR_00588C50]]
        );
    }

    #[test]
    fn at_006a0840_asks_the_angle_with_y_first() {
        let mut e = Engine::new();
        e.register(ANGLE_004B5570, |_, a| {
            f32::from_bits(a[0]).atan2(f32::from_bits(a[1])).into_ret()
        });
        let p = point(&mut e, 0.0, 1.0);
        let angle = e.call(0x006a_0840, &args![p]).f64();
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
        e.mem.set_u32(p, 0xff80_0001);
        e.call_log = Some(vec![]);
        e.call(0x006a_0840, &args![p]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ANGLE_004B5570),
            vec![vec![1.0f32.to_bits(), 0xffc0_0001]]
        );
    }

    #[test]
    fn point_record_constructors() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(BASE_SET_006C4350, |_, _| Ret::default());
        let source = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        let this = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x006a_04b0, &args![this, source, 0xff80_0002u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 8), 0x102);
        assert_eq!(e.mem.u32(this + 0xc), 0xffc0_0002);

        let this = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x006a_05e0, &args![this, source, source + 8, 7u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 4), 0x101);
        assert_eq!(e.mem.u32(this + 0xc), 0x103);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        let copy = e.mem.alloc(0x18);
        assert_eq!(e.call(0x006a_0620, &args![copy, this]).u32(), copy);
        for offset in [0u32, 4, 8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(copy + offset), e.mem.u32(this + offset));
        }

        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006a_0c10, &args![this, 0x55u32, source]).u32(),
            this
        );
        assert_eq!(e.call(0x006a_0e50, &args![copy, this]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x14), 0x102);
        assert_eq!(e.mem.u32(copy + 0x10), 0x101);
        assert_eq!(
            calls_to(&log, BASE_SET_006C4350),
            vec![vec![this, 0x55], vec![copy, this]]
        );
    }

    #[test]
    fn at_006a04e0_stores_both_direction_coordinates() {
        let mut e = Engine::new();
        e.register(DIRECTION_X_004E4470, |_, a| {
            f32::from_bits(a[0]).cos().into_ret()
        });
        e.register(DIRECTION_Y_004E44B0, |_, a| {
            f32::from_bits(a[0]).sin().into_ret()
        });
        let this = e.mem.alloc(8);
        assert_eq!(
            e.call(0x006a_04e0, &args![this, 0.0f32.to_bits()]).u32(),
            this
        );
        assert_eq!(e.mem.f32(this), 1.0);
        assert_eq!(e.mem.f32(this + 4), 0.0);
    }

    #[test]
    fn at_006a06f0_orders_by_quadrant_then_coordinate() {
        let mut e = Engine::new();
        points(&mut e);
        let call = |e: &mut Engine, a: (f32, f32), b: (f32, f32)| {
            let p = point(e, a.0, a.1);
            let q = point(e, b.0, b.1);
            e.call(0x006a_06f0, &args![p, q]).bool()
        };
        assert!(call(&mut e, (1.0, 1.0), (-1.0, 1.0)));
        assert!(!call(&mut e, (-1.0, 1.0), (1.0, 1.0)));
        // Quadrant 0: true when the other's y is not below.
        assert!(call(&mut e, (1.0, 1.0), (1.0, 2.0)));
        assert!(call(&mut e, (1.0, 1.0), (1.0, 1.0)));
        assert!(!call(&mut e, (1.0, 2.0), (1.0, 1.0)));
        // Quadrant 1: true when the other's y is not above.
        assert!(call(&mut e, (-1.0, 2.0), (-1.0, 1.0)));
        assert!(!call(&mut e, (-1.0, 1.0), (-1.0, 2.0)));
        // Unordered: false in both kinds of quadrant.
        assert!(!call(&mut e, (1.0, f32::NAN), (1.0, 1.0)));
        assert!(!call(&mut e, (-1.0, 1.0), (-1.0, f32::NAN)));
    }

    #[test]
    fn at_006a0660_compares_point_and_edge_directions() {
        let mut e = Engine::new();
        points(&mut e);
        let segment = e.mem.alloc(0x14);
        e.mem.set_f32(segment + 8, 1.0);
        e.mem.set_f32(segment + 0xc, 1.0);
        // Edge (0,0)->(1,1); the point (1,2) is at a larger angle.
        let p = point(&mut e, 1.0, 2.0);
        assert!(!e.call(0x006a_0660, &args![segment, p]).bool());
        let q = point(&mut e, 2.0, 1.0);
        assert!(e.call(0x006a_0660, &args![segment, q]).bool());
        // Swapped: the edge runs (1,1)->(0,0).
        e.mem.set_u32(segment + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x006a_0660, &args![segment, p]);
        let log = e.call_log.take().unwrap();
        let minus = calls_to(&log, POINT_MINUS_00698620);
        assert_eq!(minus[0][0], segment);
        assert_eq!(minus[0][2], segment + 8);
        assert_eq!(minus[1][0], p);
    }

    #[test]
    fn at_006a07a0_scales_the_angle_by_side() {
        let mut e = Engine::new();
        points(&mut e);
        e.map(0x0101_2000, 0x1_0000);
        e.set_global::<f64>(ZERO, 0.0);
        e.set_global::<f64>(TWO_PI, f64::from(std::f32::consts::TAU));
        e.register(ANGLE_004B5570, |_, a| {
            f64::from(f32::from_bits(a[0]))
                .atan2(f64::from(f32::from_bits(a[1])))
                .into_ret()
        });
        let segment = e.mem.alloc(0x14);
        let run = |e: &mut Engine, x: f32, y: f32, flag: u32| {
            e.mem.set_f32(segment + 8, x);
            e.mem.set_f32(segment + 0xc, y);
            e.mem.set_u32(segment + 0x10, flag);
            e.call(0x006a_07a0, &args![segment, 2.0f32.to_bits()]).f64()
        };
        let tau = f64::from(std::f32::consts::TAU);
        let quarter = f64::from(std::f32::consts::FRAC_PI_2);
        assert_eq!(run(&mut e, 0.0, 1.0, 0), quarter * 2.0);
        let below = f64::from((-std::f32::consts::FRAC_PI_2 as f64 + tau) as f32 * 2.0);
        assert_eq!(run(&mut e, 0.0, -1.0, 0), below);
        assert_eq!(run(&mut e, 0.0, -1.0, 1), quarter * 2.0);
        let above = f64::from(((tau - quarter) as f32) * 2.0);
        assert_eq!(run(&mut e, 0.0, 1.0, 1), above);
    }

    #[test]
    fn at_006a0e90_rotates_into_out() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        let a = point(&mut e, 1.0, 2.0);
        let b = point(&mut e, 3.0, 4.0);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x006a_0e90, &args![a, out, b]).u32(), out);
        assert_eq!(e.mem.f32(out), 1.0 * 3.0 - 2.0 * 4.0);
        assert_eq!(e.mem.f32(out + 4), 2.0 * 3.0 + 1.0 * 4.0);
    }

    #[test]
    fn at_006a1fb0_unlinks_a_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x60);
        let next = e.mem.alloc(0x60);
        let back = e.mem.alloc(4);
        e.mem.set_u32(node + 0x58, back);
        e.mem.set_u32(node + 0x5c, next);
        e.mem.set_u32(next + 0x58, node + 0x5c);
        e.call(0x006a_1fb0, &args![0u32, node]);
        assert_eq!(e.mem.u32(back), next);
        assert_eq!(e.mem.u32(next + 0x58), back);
        assert_eq!(e.mem.u32(node + 0x58), 0);
        assert_eq!(e.mem.u32(node + 0x5c), 0);
        // A lone node only clears its links.
        e.call(0x006a_1fb0, &args![0u32, node]);
        assert_eq!(e.mem.u32(node + 0x58), 0);
    }

    const ARRAY_SLOTS: u32 = 0x0f05_0000;
    const ALLOCATE_SLOT: u32 = 0x0f05_1000;
    const REALLOCATE_SLOT: u32 = 0x0f05_2000;

    fn array(e: &mut Engine, buffer: u32, size: u32, capacity: u32) -> u32 {
        e.put_vtable(ARRAY_SLOTS, &[0, ALLOCATE_SLOT, 0, REALLOCATE_SLOT]);
        e.register(ALLOCATE_SLOT, |_, a| (0x1000 + a[1]).into_ret());
        e.register(REALLOCATE_SLOT, |_, a| (0x2000 + a[2]).into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, ARRAY_SLOTS);
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 8, size);
        e.mem.set_u32(this + 0xc, capacity);
        this
    }

    #[test]
    fn at_006a2630_constructs_each_placed_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| {
            let place = if a[1] == 0x5040 { 0 } else { a[1] };
            place.into_ret()
        });
        e.register(ELEMENT_CTOR_006B1D80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a_2630, &args![0u32, 0x5000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW_006E6DA0),
            vec![vec![0x40, 0x5000], vec![0x40, 0x5040], vec![0x40, 0x5080]]
        );
        assert_eq!(
            calls_to(&log, ELEMENT_CTOR_006B1D80),
            vec![vec![0x5000], vec![0x5080]]
        );
    }

    #[test]
    fn at_006a2130_reserves_and_constructs() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        let this = array(&mut e, 0xdead, 0xdead, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x006a_2130, &args![this, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this, 2]]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0x1002, 2, 2)
        );
        assert_eq!(calls_to(&log, PLACEMENT_NEW_006E6DA0).len(), 2);
        e.call(0x006a_2130, &args![this, 0u32, 0u32]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn at_006a2270_moves_in_the_safe_direction() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        let moves = |e: &mut Engine, to: u32, from: u32, count: u32| {
            e.call_log = Some(vec![]);
            e.call(0x006a_2270, &args![0u32, to, from, count]);
            let log = e.call_log.take().unwrap();
            calls_to(&log, MEMMOVE)
        };
        assert_eq!(
            moves(&mut e, 0x1000, 0x2000, 2),
            vec![vec![0x1000, 0x2000, 0x3c], vec![0x103c, 0x203c, 0x3c]]
        );
        assert_eq!(
            moves(&mut e, 0x2000, 0x1000, 2),
            vec![vec![0x203c, 0x103c, 0x3c], vec![0x2000, 0x1000, 0x3c]]
        );
        assert!(moves(&mut e, 0x1000, 0x1000, 2).is_empty());
        assert!(moves(&mut e, 0x1000, 0x2000, 0).is_empty());
        // A count above i32::MAX starts the backward index negative.
        assert!(moves(&mut e, 0x2000, 0x1000, 0x8000_0001).is_empty());
    }

    #[test]
    fn at_006a21c0_and_006a2770_change_the_buffer() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(ARRAY_FREE_BUFFER_006A8500, |_, _| Ret::default());
        e.register(MOVE_ELEMENTS_00692480, |_, _| Ret::default());
        // No buffer: allocate and set the capacity.
        let this = array(&mut e, 0, 0, 0);
        e.call(0x006a_21c0, &args![this, 8u32, 0u32]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x1008, 8));
        // Count equals the capacity: reallocate in place.
        let this = array(&mut e, 0x5000, 4, 4);
        e.call_log = Some(vec![]);
        e.call(0x006a_21c0, &args![this, 8u32, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, REALLOCATE_SLOT), vec![vec![this, 0x5000, 8]]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x2008, 4));
        // Otherwise: new buffer, move, free the old one.
        let this = array(&mut e, 0x5000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x006a_21c0, &args![this, 8u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMMOVE).len(), 2);
        assert_eq!(calls_to(&log, MEMMOVE)[0], vec![0x1008, 0x5000, 0x3c]);
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER_006A8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0x1008);
        let this = array(&mut e, 0x5000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x006a_2770, &args![this, 8u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MOVE_ELEMENTS_00692480),
            vec![vec![this, 0x1008, 0x5000, 2]]
        );
    }

    #[test]
    fn at_006a1920_grows_when_full_and_returns_the_index() {
        let mut e = Engine::new();
        e.register(ARRAY_FULL_00438B90, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) >= e.mem.u32(a[0] + 0xc)).into_ret()
        });
        e.register(ARRAY_GROWN_CAPACITY_009A3910, |e, a| {
            (e.mem.u32(a[0] + 0xc) * 2).into_ret()
        });
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(ARRAY_FREE_BUFFER_006A8500, |_, _| Ret::default());
        let this = array(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x006a_1920, &args![this]).u32(), 0);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0x1004, 1, 4)
        );
        assert_eq!(e.call(0x006a_1920, &args![this]).u32(), 1);
        e.mem.set_u32(this + 8, 4);
        assert_eq!(e.call(0x006a_1920, &args![this]).u32(), 4);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        // Full at its capacity: reallocated in place through slot +0xC.
        assert_eq!(e.mem.u32(this + 4), 0x2008);
    }

    #[test]
    fn at_0069f920_appends_and_constructs_a_node() {
        let mut e = Engine::new();
        e.register(ARRAY_FULL_00438B90, |_, _| Ret::default());
        e.register(ARRAY_AT_006E8F80, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x3c).into_ret()
        });
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        e.register(NODE_CTOR_006E2280, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x8000);
        e.mem.set_u32(this + 0x28, 2);
        e.mem.set_u32(this + 0x2c, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_f920, &args![this, 0x11u32, 0x22u32]).u32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_AT_006E8F80),
            vec![vec![this + 0x20, 2]]
        );
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW_006E6DA0),
            vec![vec![0x3c, 0x8078]]
        );
        assert_eq!(
            calls_to(&log, NODE_CTOR_006E2280),
            vec![vec![0x8078, 0x11, 0x22]]
        );
        // No place: no construction.
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_f920, &args![this, 0x11u32, 0x22u32]).u32(), 3);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, NODE_CTOR_006E2280).is_empty());
    }
}

#[cfg(test)]
mod n031_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn logged(e: &mut Engine, run: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        run(e);
        e.call_log.take().unwrap()
    }

    /// The array helpers as `BSSimpleArray`s: size at +8, buffer at +4;
    /// lists of 0x10-byte arrays, word elements, 8-byte records.
    fn arrays(e: &mut Engine) {
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(STEP_00558DD0, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x10).into_ret()
        });
        e.register(ARRAY_AT_00877A30, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        e.register(RECORD_AT_006950D0, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 8).into_ret()
        });
        e.register(PLUS_EIGHT_00413F40, |_, a| (a[0] + 8).into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
    }

    /// An array object with `words` as its elements (buffer, size and
    /// capacity set).
    fn word_array(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *word);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, words.len() as u32);
        e.mem.set_u32(array + 0xc, words.len() as u32);
        array
    }

    /// A list of arrays (0x10-byte elements) holding the given words.
    fn list_of(e: &mut Engine, lists: &[&[u32]]) -> u32 {
        let outer = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x10 * lists.len().max(1) as u32);
        for (i, words) in lists.iter().enumerate() {
            let inner = word_array(e, words);
            for k in 0..4 {
                let word = e.mem.u32(inner + 4 * k);
                e.mem.set_u32(buffer + 0x10 * i as u32 + 4 * k, word);
            }
        }
        e.mem.set_u32(outer + 4, buffer);
        e.mem.set_u32(outer + 8, lists.len() as u32);
        outer
    }

    /// The 3D vector helpers on floats.
    fn vectors(e: &mut Engine) {
        e.register(POINT3_MINUS_00439EF0, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT3_PLUS_00439E90, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(SCALED_POINT_0069E0C0, |e, a| {
            let scale = f32::from_bits(a[2]);
            let x = e.mem.f32(a[0]) * scale;
            let y = e.mem.f32(a[0] + 4) * scale;
            e.mem.set_f32(a[1], x);
            e.mem.set_f32(a[1] + 4, y);
            e.mem.set_f32(a[1] + 8, 0.0);
            a[1].into_ret()
        });
        e.register(POINT3_LENGTH_SQUARED_004A7290, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            f64::from(x * x + y * y + z * z).into_ret()
        });
        e.register(POINT3_LENGTH_00457990, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            f64::from((x * x + y * y + z * z).sqrt()).into_ret()
        });
        e.register(POINT_SET_006985F0, |e, a| {
            let (x, y) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], x);
            e.mem.set_u32(a[0] + 4, y);
            Ret::default()
        });
    }

    fn pov(e: &mut Engine, kind: u32, center: (f32, f32, f32), radius: f32) -> u32 {
        let p = e.mem.alloc(0x38);
        e.mem.set_u32(p, kind);
        e.mem.set_f32(p + 4, center.0);
        e.mem.set_f32(p + 8, center.1);
        e.mem.set_f32(p + 0xc, center.2);
        e.mem.set_f32(p + 0x10, radius);
        let duplicates = word_array(e, &[]);
        e.mem.set_u32(p + 0x30, duplicates);
        p
    }

    fn map_constants(e: &mut Engine) {
        e.map(0x0101_0000, 0x10_0000);
        e.set_global::<f32>(BLOCKED_COST_TYPE_5, 2.0);
        e.set_global::<f32>(BLOCKED_COST, 100.0);
        e.set_global::<f64>(GOAL_DISTANCE_SCALE, 1.5);
        e.set_global::<f32>(ARC_STEP, 0.25);
        e.set_global::<f32>(ARC_STEP_REVERSED, -0.25);
        e.set_global::<f32>(EDGE_MIDPOINT_SCALE, 0.5);
        e.set_global::<f32>(EDGE_PULL_SCALE, 0.25);
        e.map(0x011f_4000, 0x1000);
    }

    #[test]
    fn at_0069ef40_and_0069ef60_split_the_word() {
        let mut e = Engine::new();
        let word = e.mem.alloc(4);
        e.mem.set_u16(word, 0xc123);
        assert_eq!(e.call(0x0069_ef40, &args![word]).u32(), 0x0123);
        assert_eq!(e.call(0x0069_ef60, &args![word]).u32(), 3);
        e.mem.set_u16(word, 0x4fff);
        assert_eq!(e.call(0x0069_ef40, &args![word]).u32(), 0x0fff);
        assert_eq!(e.call(0x0069_ef60, &args![word]).u32(), 1);
    }

    const DTOR_SLOT: u32 = 0x0f31_0000;

    #[test]
    fn at_0069efd0_deletes_the_member_when_set() {
        let mut e = Engine::new();
        stub(&mut e, &[DTOR_SLOT]);
        e.put_vtable(0x0f30_0000, &[DTOR_SLOT]);
        let this = e.mem.alloc(0x34);
        let log = logged(&mut e, |e| {
            e.call(0x0069_efd0, &args![this]);
        });
        assert!(calls_to(&log, DTOR_SLOT).is_empty());
        let member = e.mem.alloc(8);
        e.mem.set_u32(member, 0x0f30_0000);
        e.mem.set_u32(this + 0x30, member);
        let log = logged(&mut e, |e| {
            e.call(0x0069_efd0, &args![this]);
        });
        assert_eq!(calls_to(&log, DTOR_SLOT), vec![vec![member, 1]]);
    }

    #[test]
    fn at_0069efa0_destroys_and_deletes_by_flag() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        let this = e.mem.alloc(0x34);
        for flags in [0u32, 1] {
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(0x0069_efa0, &args![this, flags]).u32(), this);
            });
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn at_006a15b0_destroys_the_search_members() {
        let mut e = Engine::new();
        stub(&mut e, &[EMPTY_DTOR_00483710, NODE_HEAP_DTOR_00982610]);
        let this = e.mem.alloc(0x2058);
        let log = logged(&mut e, |e| {
            e.call(0x006a_15b0, &args![this]);
        });
        assert_eq!(e.mem.u32(this), ASTAR_SEARCH_VTABLE);
        assert_eq!(
            calls_to(&log, EMPTY_DTOR_00483710),
            vec![vec![this + 0x1ff4], vec![this + 0x14]]
        );
        assert_eq!(
            calls_to(&log, NODE_HEAP_DTOR_00982610),
            vec![vec![this + 4]]
        );
    }

    #[test]
    fn at_0069e200_destroys_members_in_order() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                ARRAY_RELEASE_008454F0,
                ARRAY_RELEASE_0049FAF0,
                LOCATION_DTOR_004FF7E0,
                EMPTY_DTOR_00483710,
                NODE_HEAP_DTOR_00982610,
            ],
        );
        let this = e.mem.alloc(0x20f8);
        let log = logged(&mut e, |e| {
            e.call(0x0069_e200, &args![this]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_RELEASE_008454F0),
            vec![
                vec![this + 0x20e8, 1],
                vec![this + 0x20d8, 1],
                vec![this + 0x2058, 1]
            ]
        );
        assert_eq!(
            calls_to(&log, LOCATION_DTOR_004FF7E0),
            vec![vec![this + 0x20b0], vec![this + 0x2088]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_RELEASE_0049FAF0),
            vec![vec![this + 0x2068, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x20e8), ARRAY_VTABLE_0106B3FC);
        assert_eq!(e.mem.u32(this), ASTAR_SEARCH_VTABLE);
    }

    fn check_scalar_deleting(address: u32, destroyed: &[u32]) {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        stub(&mut e, destroyed);
        let this = e.mem.alloc(0x20f8);
        for flags in [0u32, 1] {
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
            });
            for callee in destroyed {
                assert!(!calls_to(&log, *callee).is_empty());
            }
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn at_0069e1d0_deletes_the_pov_search() {
        check_scalar_deleting(
            0x0069_e1d0,
            &[
                ARRAY_RELEASE_008454F0,
                ARRAY_RELEASE_0049FAF0,
                LOCATION_DTOR_004FF7E0,
                EMPTY_DTOR_00483710,
                NODE_HEAP_DTOR_00982610,
            ],
        );
    }

    #[test]
    fn at_006a1ba0_deletes_a_pov_array() {
        check_scalar_deleting(0x006a_1ba0, &[ARRAY_RELEASE_008454F0]);
    }

    #[test]
    fn at_006a1bd0_deletes_the_astar_search() {
        check_scalar_deleting(0x006a_1bd0, &[EMPTY_DTOR_00483710, NODE_HEAP_DTOR_00982610]);
    }

    #[test]
    fn at_006a1c00_deletes_a_pov_array_array() {
        check_scalar_deleting(0x006a_1c00, &[ARRAY_RELEASE_0049FAF0]);
    }

    #[test]
    fn at_006a1c30_deletes_a_node_array() {
        check_scalar_deleting(0x006a_1c30, &[ARRAY_RELEASE_008454F0]);
    }

    #[test]
    fn at_006a1c60_deletes_a_link_array() {
        check_scalar_deleting(0x006a_1c60, &[ARRAY_RELEASE_008454F0]);
    }

    #[test]
    fn at_006a1c90_deletes_a_vertex_handle_array_array() {
        check_scalar_deleting(0x006a_1c90, &[ARRAY_RELEASE_0049FAF0]);
    }

    #[test]
    fn at_006a1cc0_deletes_a_remove_index_array() {
        check_scalar_deleting(0x006a_1cc0, &[ARRAY_RELEASE_008454F0]);
    }

    #[test]
    fn at_006a25a0_deletes_an_astar_node_array() {
        check_scalar_deleting(0x006a_25a0, &[ARRAY_RELEASE_008454F0]);
    }

    #[test]
    fn at_006a2820_deletes_an_astar_node_scrap_array() {
        check_scalar_deleting(0x006a_2820, &[ARRAY_RELEASE_008454F0]);
    }

    #[test]
    fn at_006a28d0_deletes_a_vertex_handle_array() {
        check_scalar_deleting(0x006a_28d0, &[ARRAY_RELEASE_008454F0]);
    }

    fn check_array_ctor(address: u32, vtable: u32, init: u32) {
        let mut e = Engine::new();
        stub(&mut e, &[init]);
        let this = e.mem.alloc(0x14);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(address, &args![this]).u32(), this);
        });
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(calls_to(&log, init), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn at_006a14b0_constructs_a_pov_array() {
        check_array_ctor(0x006a_14b0, ARRAY_VTABLE_0106B3B0, ARRAY_INIT_006B3EB0);
    }

    #[test]
    fn at_006a1860_constructs_a_node_array() {
        check_array_ctor(0x006a_1860, ARRAY_VTABLE_0106B3E8, ARRAY_INIT_006B3F40);
    }

    #[test]
    fn at_006a2850_constructs_an_astar_node_array() {
        check_array_ctor(0x006a_2850, ARRAY_VTABLE_0106B44C, ARRAY_INIT_006B3EB0);
    }

    #[test]
    fn at_006a2880_constructs_a_vertex_handle_array() {
        check_array_ctor(0x006a_2880, ARRAY_VTABLE_0106B460, ARRAY_INIT_006DC440);
    }

    #[test]
    fn at_006a16d0_006a18b0_006a1a00_construct_empty_arrays() {
        let mut e = Engine::new();
        for (address, vtable) in [
            (0x006a_16d0, ARRAY_VTABLE_0106B3D4),
            (0x006a_18b0, ARRAY_VTABLE_0106B3FC),
            (0x006a_1a00, ARRAY_VTABLE_0106B424),
        ] {
            let this = e.mem.alloc(0x10);
            e.mem.write(this, &[0xcd; 0x10]);
            assert_eq!(e.call(address, &args![this]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            for offset in [4u32, 8, 0xc] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
        }
    }

    const ALLOCATE: u32 = 0x0f32_0000;
    const ARRAY_SLOTS: u32 = 0x0f33_0000;

    fn allocating_vtable(e: &mut Engine, at: u32) {
        e.put_vtable(at, &[0, ALLOCATE, 0, 0]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * 0x10).into_ret());
    }

    #[test]
    fn at_006a19b0_constructs_sized_handle_arrays() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        stub(&mut e, &[ARRAY_INIT_006DC440]);
        allocating_vtable(&mut e, ARRAY_VTABLE_0106B410);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x006a_19b0, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let buffer = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(buffer), ARRAY_VTABLE_0106B460);
        assert_eq!(e.mem.u32(buffer + 0x10), ARRAY_VTABLE_0106B460);
    }

    #[test]
    fn at_006a20a0_006a2310_006a2470_initialize_with_their_elements() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        stub(&mut e, &[ARRAY_INIT_006B3EB0, ARRAY_INIT_006DC440]);
        allocating_vtable(&mut e, ARRAY_SLOTS);
        for (address, check) in [
            (0x006a_20a0u32, ARRAY_VTABLE_0106B3B0),
            (0x006a_2310, ARRAY_VTABLE_0106B460),
            (0x006a_2470, 0xffff_ffff),
        ] {
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this, ARRAY_SLOTS);
            e.call(address, &args![this, 1u32, 2u32]);
            assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (2, 2));
            let buffer = e.mem.u32(this + 4);
            assert_eq!(e.mem.u32(buffer), check);
            // A zero reserve and size allocates nothing.
            let empty = e.mem.alloc(0x10);
            e.mem.set_u32(empty, ARRAY_SLOTS);
            e.call(address, &args![empty, 0u32, 0u32]);
            assert_eq!(e.mem.u32(empty + 4), 0);
        }
    }

    #[test]
    fn at_006a2000_006a23a0_006a26d0_construct_placed_elements() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| {
            let place = if a[1] & 0xff == 0x10 { 0 } else { a[1] };
            place.into_ret()
        });
        stub(&mut e, &[ARRAY_INIT_006B3EB0, ARRAY_INIT_006DC440]);
        let block = e.mem.alloc(0x200);
        let first = (block + 0xff) & !0xff;
        let log = logged(&mut e, |e| {
            e.call(0x006a_2000, &args![0u32, first, 3u32]);
        });
        assert_eq!(calls_to(&log, PLACEMENT_NEW_006E6DA0).len(), 3);
        assert_eq!(e.mem.u32(first), ARRAY_VTABLE_0106B3B0);
        // The second element got no place: not constructed.
        assert_eq!(e.mem.u32(first + 0x10), 0);
        assert_eq!(e.mem.u32(first + 0x20), ARRAY_VTABLE_0106B3B0);
        e.call(0x006a_23a0, &args![0u32, first + 0x80, 2u32]);
        assert_eq!(e.mem.u32(first + 0x80), 0xffff_ffff);
        assert_eq!(e.mem.u32(first + 0x8c), 0xffff_ffff);
        e.call(0x006a_26d0, &args![0u32, first + 0xa0, 1u32]);
        assert_eq!(e.mem.u32(first + 0xa0), ARRAY_VTABLE_0106B460);
    }

    #[test]
    fn at_006a2440_sets_both_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x006a_2440, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
    }

    #[test]
    fn at_006a2500_and_006a25d0_build_and_destroy_the_scrap_array() {
        let mut e = Engine::new();
        stub(&mut e, &[ARRAY_INIT_006B3EB0, ARRAY_RELEASE_008454F0]);
        e.register(SCRAP_HOLDER_00401020, |_, _| 0x011f_6238u32.into_ret());
        e.register(SCRAP_HEAP_00AA42E0, |_, a| (a[0] + 0x10).into_ret());
        let this = e.mem.alloc(0x14);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x006a_2500, &args![this]).u32(), this);
        });
        assert_eq!(e.mem.u32(this), SCRAP_ARRAY_VTABLE_0106B438);
        assert_eq!(e.mem.u32(this + 0x10), 0x011f_6248);
        assert_eq!(calls_to(&log, ARRAY_INIT_006B3EB0).len(), 2);
        let log = logged(&mut e, |e| {
            e.call(0x006a_25d0, &args![this]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_RELEASE_008454F0),
            vec![vec![this, 1], vec![this, 1]]
        );
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106B44C);
    }

    #[test]
    fn at_006a1500_constructs_the_astar_search() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                SCRAP_INIT_00AA5410,
                NODE_MAP_CTOR_006B3DE0,
                QUEUE_CTOR_006B9470,
            ],
        );
        let this = e.mem.alloc(0x2058);
        e.mem.write(this + 0x2048, &[0xcd; 0x10]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x006a_1500, &args![this]).u32(), this);
        });
        assert_eq!(e.mem.u32(this), ASTAR_SEARCH_VTABLE);
        assert_eq!(
            calls_to(&log, SCRAP_INIT_00AA5410),
            vec![vec![this + 4, 0xd000]]
        );
        assert_eq!(
            calls_to(&log, NODE_MAP_CTOR_006B3DE0),
            vec![vec![this + 0x14]]
        );
        assert_eq!(
            calls_to(&log, QUEUE_CTOR_006B9470),
            vec![vec![this + 0x1ff4]]
        );
        for offset in [0x2048u32, 0x204c, 0x2050] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x2054), 0);
    }

    #[test]
    fn at_0069e2c0_constructs_a_pov() {
        let mut e = Engine::new();
        map_constants(&mut e);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.set_global::<f32>(ORIGIN_011F426C + 4 * i as u32, *v);
        }
        stub(&mut e, &[IDENTITY_006815C0, ARRAY_INIT_006B3EB0]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x0069_e2c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        assert_eq!(e.mem.f32(this + 8), 2.0);
        assert_eq!(e.mem.f32(this + 0x10), 0.0);
        assert_eq!(e.mem.f32(this + 0x2c), 1.0);
        let array = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0106B3B0);
        // A failed allocation leaves a null array.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call(0x0069_e2c0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn at_006a0520_and_006a0580_rotate_in_place() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_f32(other, 3.0);
        e.mem.set_f32(other + 4, 4.0);
        e.mem.set_f32(this, 1.0);
        e.mem.set_f32(this + 4, 2.0);
        assert_eq!(e.call(0x006a_0520, &args![this, other]).u32(), this);
        assert_eq!((e.mem.f32(this), e.mem.f32(this + 4)), (-5.0, 10.0));
        e.mem.set_f32(this, 1.0);
        e.mem.set_f32(this + 4, 2.0);
        e.call(0x006a_0580, &args![this, other]);
        assert_eq!((e.mem.f32(this), e.mem.f32(this + 4)), (11.0, 2.0));
    }

    #[test]
    fn at_006a08a0_tests_the_pov_type() {
        let mut e = Engine::new();
        arrays(&mut e);
        let node = e.mem.alloc(0x68);
        let goal = pov(&mut e, 4, (0.0, 0.0, 0.0), 1.0);
        e.mem.set_u32(node + 8, goal);
        assert!(e.call(0x006a_08a0, &args![0u32, node]).bool());
        e.mem.set_u32(goal, 3);
        assert!(!e.call(0x006a_08a0, &args![0u32, node]).bool());
    }

    #[test]
    fn at_006a08d0_sets_the_node_and_its_heuristic() {
        let mut e = Engine::new();
        map_constants(&mut e);
        arrays(&mut e);
        vectors(&mut e);
        let this = e.mem.alloc(0x20f8);
        let goal = pov(&mut e, 4, (0.0, 4.0, 0.0), 1.0);
        e.mem.set_u32(this + GOAL_POV, goal);
        let start = pov(&mut e, 0, (3.0, 0.0, 0.0), 1.0);
        let node = e.mem.alloc(0x68);
        e.call(0x006a_08d0, &args![this, node, start, 1u32, 7u32]);
        assert_eq!(e.mem.u32(node + 8), start);
        assert_eq!(e.mem.u32(node + 0xc), 1);
        assert_eq!(e.mem.u32(node + 0x10), 7);
        assert_eq!(e.mem.f32(node + 4), 7.5);
    }

    /// The POV search with `others` in its only list and the walk's doubles.
    fn search_with(e: &mut Engine, others: &[u32]) -> u32 {
        map_constants(e);
        arrays(e);
        vectors(e);
        e.register(ARRAY_FIND_00719B20, |_, _| 0xffff_ffffu32.into_ret());
        let this = e.mem.alloc(0x20f8);
        let lists = list_of(e, &[others]);
        copy_words(e, this + POVS, lists, 4);
        this
    }

    #[test]
    fn at_006a0940_costs_the_arc_by_blocking_povs() {
        let mut e = Engine::new();
        let node_pov = pov(&mut e, 0, (0.0, 0.0, 0.0), 1.0);
        let blocker = pov(&mut e, 5, (5.0, 0.0, 0.0), 2.0);
        let this = search_with(&mut e, &[node_pov, blocker]);
        e.register(CIRCLE_TEST_006BB8B0, |_, _| Ret::default());
        let arc = [1.0f32.to_bits(), 0, 0, 0, 0];
        let run = |e: &mut Engine, exclude: u32| {
            e.call(
                0x006a_0940,
                &args![this, node_pov, arc[0], arc[1], arc[2], arc[3], arc[4], exclude],
            )
            .f64()
        };
        assert_eq!(run(&mut e, 0), 2.0);
        e.mem.set_u32(blocker, 1);
        assert_eq!(run(&mut e, 0), 100.0);
        // Skipped POV, or a circle test that clears it: 1.0.
        assert_eq!(run(&mut e, blocker), 1.0);
        e.register(CIRCLE_TEST_006BB8B0, |_, _| 1u32.into_ret());
        assert_eq!(run(&mut e, 0), 1.0);
        // A POV whose duplicates hold the node's is skipped.
        e.register(CIRCLE_TEST_006BB8B0, |_, _| Ret::default());
        e.register(ARRAY_FIND_00719B20, |_, _| 0u32.into_ret());
        assert_eq!(run(&mut e, 0), 1.0);
    }

    #[test]
    fn at_006a0ac0_costs_the_segment_by_blocking_povs() {
        let mut e = Engine::new();
        let node_pov = pov(&mut e, 0, (0.0, 0.0, 0.0), 1.0);
        let blocker = pov(&mut e, 2, (5.0, 0.0, 0.0), 2.0);
        let this = search_with(&mut e, &[blocker]);
        e.register(CIRCLE_TEST_006BAB00, |_, _| 1u32.into_ret());
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 9.0);
        e.call_log = Some(vec![]);
        let cost = e
            .call(0x006a_0ac0, &args![this, node_pov, point, 0x55u32, 0u32])
            .f64();
        let log = e.call_log.take().unwrap();
        assert_eq!(cost, 100.0);
        assert_eq!(
            calls_to(&log, CIRCLE_TEST_006BAB00),
            vec![vec![blocker + 4, 2.0f32.to_bits(), point, 0x55]]
        );
        // The point inside the radius (squared distance below it): skipped.
        e.mem.set_f32(point, 6.0);
        assert_eq!(
            e.call(0x006a_0ac0, &args![this, node_pov, point, 0u32, 0u32])
                .f64(),
            1.0
        );
        e.mem.set_f32(point, 9.0);
        e.register(CIRCLE_TEST_006BAB00, |_, _| Ret::default());
        assert_eq!(
            e.call(0x006a_0ac0, &args![this, node_pov, point, 0u32, 0u32])
                .f64(),
            1.0
        );
    }

    /// The navmesh walk's doubles: `crossings` edges crossed before the end,
    /// matching edges as `matched` says.
    fn walk(e: &mut Engine, crossings: u32, matched: bool) {
        map_constants(e);
        vectors(e);
        stub(
            e,
            &[
                DIRECTION_STEP_004A0C10,
                BASE_SET_006C4350,
                EDGE_HANDLE_0069A6C0,
                BASE_CTOR_006240D0,
                RECORD_COPY_006CF900,
                HANDLE_SET_006BC870,
                TRIANGLE_005582F0,
                GET_EDGE_EXTRA_INFO_0068F230,
            ],
        );
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        let mut left = crossings;
        e.register_double(FIND_INTERSECTING_EDGE_00698160, move |e, a| {
            let found = u32::from(left > 0);
            left = left.saturating_sub(1);
            e.mem.set_u32(a[1], found);
            Ret::default()
        });
        if matched {
            e.register(GET_MATCHING_EDGE_0068F670, |_, _| 1u32.into_ret());
        } else {
            e.register(GET_MATCHING_EDGE_0068F670, |_, _| Ret::default());
        }
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 0x100).into_ret());
        e.map(0x011d_6000, 0x1000);
        e.set_global::<f32>(SETTING_011D6EF0 + 0x100, 5.0);
    }

    fn pov_search(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20f8);
        let request = e.mem.alloc(0x70);
        e.mem.set_f32(request + 0x68, 1.0);
        e.mem.set_u32(this + REQUEST, request);
        this
    }

    #[test]
    fn at_006a0ef0_walks_until_clear_or_blocked() {
        let mut e = Engine::new();
        walk(&mut e, 0, true);
        let this = pov_search(&mut e);
        let from = e.mem.alloc(0x18);
        let to = e.mem.alloc(0xc);
        let out = e.mem.alloc(0x18);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_0ef0, &args![this, from, to, out]).bool());
        });
        assert_eq!(calls_to(&log, RECORD_COPY_006CF900).len(), 1);
        assert_eq!(calls_to(&log, RECORD_COPY_006CF900)[0][0], out);
        // One crossing over a matching edge on the same navmesh, then clear.
        walk(&mut e, 1, true);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_0ef0, &args![this, from, to, out]).bool());
        });
        assert_eq!(calls_to(&log, HANDLE_SET_006BC870).len(), 1);
        assert!(calls_to(&log, GET_EDGE_VERTICES_0068F040).is_empty());
        // A crossing without a matching edge blocks.
        walk(&mut e, 1, false);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x006a_0ef0, &args![this, from, to, out]).bool());
        });
        assert_eq!(calls_to(&log, RECORD_COPY_006CF900)[0][0], out);
        assert!(calls_to(&log, HANDLE_SET_006BC870).is_empty());
        // An extra info of type 1 blocks too.
        walk(&mut e, 1, true);
        let info = e.mem.alloc(4);
        e.mem.set_u32(info, 1);
        e.register_double(GET_EDGE_EXTRA_INFO_0068F230, move |_, _| info.into_ret());
        assert!(!e.call(0x006a_0ef0, &args![this, from, to, out]).bool());
    }

    #[test]
    fn at_006a0ef0_pulls_towards_the_edge_on_a_navmesh_change() {
        let mut e = Engine::new();
        walk(&mut e, 1, true);
        e.register(GET_MATCHING_EDGE_0068F670, |e, a| {
            e.mem.set_u32(a[2], 0x77);
            1u32.into_ret()
        });
        stub(&mut e, &[POINT3_ADD_0063C8A0]);
        let ends = e.mem.alloc(0x18);
        e.register_double(GET_EDGE_VERTICES_0068F040, move |e, a| {
            e.mem.set_u32(a[1], ends);
            e.mem.set_u32(a[1] + 4, ends + 0xc);
            Ret::default()
        });
        let point = e.mem.alloc(0xc);
        e.register_double(SEGMENT_POINT_006BA550, move |_, _| point.into_ret());
        e.register(POINT3_SCALE_0045BB20, |_, a| a[1].into_ret());
        let this = pov_search(&mut e);
        let from = e.mem.alloc(0x18);
        let to = e.mem.alloc(0xc);
        let out = e.mem.alloc(0x18);
        e.mem.set_u32(from, 0x66);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_0ef0, &args![this, from, to, out]).bool());
        });
        assert_eq!(calls_to(&log, GET_EDGE_VERTICES_0068F040).len(), 1);
        assert_eq!(calls_to(&log, GET_EDGE_VERTICES_0068F040)[0][0], 0x77);
        assert_eq!(calls_to(&log, POINT3_SCALE_0045BB20).len(), 2);
        assert_eq!(calls_to(&log, POINT3_ADD_0063C8A0).len(), 1);
    }

    /// The quadrant double: `ordered` makes `006a0660` say the arc goes on.
    fn quadrants(e: &mut Engine, ordered: bool) {
        e.register(POINT_MINUS_00698620, |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[1] + 4, 0);
            Ret::default()
        });
        if ordered {
            e.register(POINT_QUADRANT_00698680, |_, _| Ret::default());
        } else {
            let mut calls = 0u32;
            e.register_double(POINT_QUADRANT_00698680, move |_, _| {
                calls += 1;
                u32::from(calls % 2 == 1).into_ret()
            });
        }
        e.register(DIRECTION_X_004E4470, |_, _| 1.0f64.into_ret());
        e.register(DIRECTION_Y_004E44B0, |_, _| 0.0f64.into_ret());
    }

    #[test]
    fn at_006a0c50_checks_each_chord_of_the_arc() {
        let mut e = Engine::new();
        walk(&mut e, 0, true);
        quadrants(&mut e, false);
        stub(&mut e, &[BASE_CTOR_006240D0]);
        let this = pov_search(&mut e);
        let start = e.mem.alloc(0x18);
        let center = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x18);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x006a_0c50,
                    &args![this, start, center, 0u32, 0u32, 0u32, 0u32, 0u32, out]
                )
                .bool());
        });
        // No chord in the loop: only the last one, its record to `out`.
        assert_eq!(calls_to(&log, FIND_INTERSECTING_EDGE_00698160).len(), 1);
        assert_eq!(calls_to(&log, RECORD_COPY_006CF900).last().unwrap()[0], out);
        // Every step on: 0x18 chords, then false.
        quadrants(&mut e, true);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x006a_0c50,
                    &args![this, start, center, 0u32, 0u32, 0u32, 0u32, 1u32, out]
                )
                .bool());
        });
        assert_eq!(calls_to(&log, FIND_INTERSECTING_EDGE_00698160).len(), 0x18);
        // A blocked first chord gives false at once.
        walk(&mut e, 1, false);
        quadrants(&mut e, true);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x006a_0c50,
                    &args![this, start, center, 0u32, 0u32, 0u32, 0u32, 0u32, out]
                )
                .bool());
        });
        assert_eq!(calls_to(&log, FIND_INTERSECTING_EDGE_00698160).len(), 1);
    }

    const ATTACH: u32 = 0x0f34_0000;

    fn drawing(e: &mut Engine) -> u32 {
        map_constants(e);
        vectors(e);
        stub(
            e,
            &[
                POINTER_SET_00633C90,
                THREE_WORD_CTOR_0043D410,
                UPDATE_PROPERTIES_00A5A040,
                ZERO_RECORD_STEP_00A59C60,
                POINTER_RELEASE_0045CEC0,
                ATTACH,
            ],
        );
        e.register(MAKE_LINE_004B3890, |_, _| 0x4444u32.into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        let vtable = 0x0f35_0000;
        let mut slots = vec![0u32; 0x38];
        slots[(SLOT_ATTACH / 4) as usize] = ATTACH;
        e.put_vtable(vtable, &slots);
        let root = e.mem.alloc(8);
        e.mem.set_u32(root, vtable);
        e.map(DEBUG_ROOT_011D6DDC, 4);
        e.set_global::<u32>(DEBUG_ROOT_011D6DDC, root);
        root
    }

    #[test]
    fn at_006a1330_builds_and_attaches_a_line() {
        let mut e = Engine::new();
        let root = drawing(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x006a_1330, &args![0x10u32, 0x20u32, 0x30u32]);
        });
        assert_eq!(
            calls_to(&log, MAKE_LINE_004B3890),
            vec![vec![0x10, 0x30, 0x20, 0x30, 1]]
        );
        let attached = calls_to(&log, ATTACH);
        assert_eq!(attached.len(), 1);
        assert_eq!((attached[0][0], attached[0][2]), (root, 1));
        assert_eq!(calls_to(&log, POINTER_RELEASE_0045CEC0).len(), 1);
    }

    #[test]
    fn at_006a1180_draws_the_arc_chords() {
        let mut e = Engine::new();
        drawing(&mut e);
        quadrants(&mut e, false);
        let center = e.mem.alloc(0xc);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let log = logged(&mut e, |e| {
            e.call(
                0x006a_1180,
                &args![center, 1.0f32.to_bits(), first, second, 0u32, 7u32],
            );
        });
        assert_eq!(calls_to(&log, MAKE_LINE_004B3890).len(), 1);
        quadrants(&mut e, true);
        let log = logged(&mut e, |e| {
            e.call(
                0x006a_1180,
                &args![center, 1.0f32.to_bits(), first, second, 1u32, 7u32],
            );
        });
        assert_eq!(calls_to(&log, MAKE_LINE_004B3890).len(), 0x18);
    }

    #[test]
    fn at_006a1900_passes_the_byte_count() {
        let mut e = Engine::new();
        e.register(BYTES_0042F5D0, |_, a| (a[0] ^ a[1]).into_ret());
        assert_eq!(
            e.call(0x006a_1900, &args![0u32, 0x1000u32, 3u32]).u32(),
            0x1000 ^ 0xc0
        );
    }

    #[test]
    fn at_006a1b20_and_006a1b00_compare_two_words() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a, 1);
        e.mem.set_u32(b, 1);
        e.mem.set_u32(a + 4, 2);
        e.mem.set_u32(b + 4, 2);
        assert!(e.call(0x006a_1b20, &args![a, b]).bool());
        assert!(e.call(0x006a_1b00, &args![a, b]).bool());
        e.mem.set_u32(b + 4, 3);
        assert!(!e.call(0x006a_1b20, &args![a, b]).bool());
        e.mem.set_u32(b, 3);
        assert!(!e.call(0x006a_1b00, &args![a, b]).bool());
    }

    #[test]
    fn at_006a1b80_forwards_to_the_compare() {
        let mut e = Engine::new();
        e.register(COMPARE_006D4CA0, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x006a_1b80, &args![5u32, 6u32]).u32(), 11);
    }

    const COMPARE: u32 = 0x0f36_0000;

    #[test]
    fn at_006a1aa0_and_006a1460_find_a_matching_element() {
        let mut e = Engine::new();
        e.register(COMPARE, |e, a| {
            u32::from(e.mem.u32(a[0]) == a[1]).into_ret()
        });
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x18);
        for i in 0..3u32 {
            e.mem.set_u32(buffer + 8 * i, 10 + i);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, 3);
        assert_eq!(
            e.call(0x006a_1aa0, &args![array, 12u32, 0u32, COMPARE])
                .u32(),
            2
        );
        assert_eq!(
            e.call(0x006a_1aa0, &args![array, 10u32, 1u32, COMPARE])
                .u32(),
            0xffff_ffff
        );
        e.mem.set_u32(array + 4, 0);
        assert_eq!(
            e.call(0x006a_1aa0, &args![array, 10u32, 0u32, COMPARE])
                .u32(),
            0xffff_ffff
        );
        e.mem.set_u32(array + 4, buffer);
        e.register(COMPARE_006D4CA0, |e, a| {
            u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let key = e.mem.alloc(8);
        e.mem.set_u32(key, 11);
        assert!(e.call(0x006a_1460, &args![array, key]).bool());
        e.mem.set_u32(key, 99);
        assert!(!e.call(0x006a_1460, &args![array, key]).bool());
    }

    #[test]
    fn at_006a1a50_appends_an_eight_byte_value() {
        let mut e = Engine::new();
        e.register(ARRAY_RESERVE_00845460, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, 1);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x1234);
        e.mem.set_u32(value + 4, 0x5678);
        assert_eq!(e.call(0x006a_1a50, &args![array, value]).u32(), 1);
        assert_eq!(e.mem.u32(buffer + 8), 0x1234);
        assert_eq!(e.mem.u32(buffer + 0xc), 0x5678);
    }

    #[test]
    fn at_006a1720_sets_the_size() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        stub(
            &mut e,
            &[
                ARRAY_INIT_006B3EB0,
                ARRAY_RELEASE_0049FAF0,
                ARRAY_RESIZE_009F6540,
                ARRAY_DESTROY_0049FC80,
            ],
        );
        allocating_vtable(&mut e, ARRAY_SLOTS);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, ARRAY_SLOTS);
        // Zero: released with the shrink flag.
        let log = logged(&mut e, |e| {
            e.call(0x006a_1720, &args![this, 0u32, 1u32]);
        });
        assert_eq!(calls_to(&log, ARRAY_RELEASE_0049FAF0), vec![vec![this, 1]]);
        // Growing from no capacity allocates and constructs.
        e.call(0x006a_1720, &args![this, 2u32, 0u32]);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (2, 2));
        let buffer = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(buffer + 0x10), ARRAY_VTABLE_0106B3B0);
        // Growing past the capacity resizes.
        let log = logged(&mut e, |e| {
            e.call(0x006a_1720, &args![this, 3u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_RESIZE_009F6540),
            vec![vec![this, 3, 2]]
        );
        // Shrinking destroys the rest; with the flag and a quarter left,
        // the buffer shrinks too.
        e.mem.set_u32(this + 0xc, 8);
        let log = logged(&mut e, |e| {
            e.call(0x006a_1720, &args![this, 1u32, 1u32]);
        });
        assert_eq!(
            calls_to(&log, ARRAY_DESTROY_0049FC80),
            vec![vec![this, buffer + 0x10, 2]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_RESIZE_009F6540),
            vec![vec![this, 1, 1]]
        );
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (1, 1));
    }

    const GOAL_SLOT: u32 = 0x0f37_0000;
    const CONNECT_SLOT: u32 = 0x0f37_1000;

    /// A POV search whose queue pops `nodes` in order (then 0), goal test
    /// true for `goal`, connections from the double `connect`.
    fn searching(e: &mut Engine, nodes: Vec<u32>, goal: u32) -> u32 {
        arrays(e);
        stub(
            e,
            &[
                ARRAY_INIT_006B3EB0,
                ARRAY_RELEASE_008454F0,
                QUEUE_PUSH_006B3E10,
                YIELD_0040FCA0,
            ],
        );
        e.register(SCRAP_HOLDER_00401020, |_, _| Ret::default());
        e.register(SCRAP_HEAP_00AA42E0, |_, _| Ret::default());
        let mut queue = nodes.into_iter();
        e.register_double(QUEUE_POP_006B4350, move |_, _| {
            queue.next().unwrap_or(0).into_ret()
        });
        e.register_double(GOAL_SLOT, move |_, a| u32::from(a[1] == goal).into_ret());
        e.register(NODE_COST_006B9130, |e, a| {
            f64::from(e.mem.f32(a[0])).into_ret()
        });
        e.put_vtable(0x0f38_0000, &[0, CONNECT_SLOT, GOAL_SLOT]);
        let this = e.mem.alloc(0x20f8);
        e.mem.set_u32(this, 0x0f38_0000);
        this
    }

    #[test]
    fn at_006a1cf0_searches_until_a_goal() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x68);
        let b = e.mem.alloc(0x68);
        e.mem.set_f32(a, 5.0);
        e.mem.set_f32(b, 2.0);
        let this = searching(&mut e, vec![a, b], b);
        let connections = e.mem.alloc(8);
        e.mem.set_u32(connections, b);
        e.register_double(CONNECT_SLOT, move |e, x| {
            e.mem.set_u32(x[2] + 4, connections);
            e.mem.set_u32(x[2] + 8, 1);
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x006a_1cf0, &args![this]).u32(), b);
        });
        assert_eq!(e.mem.u32(b + PARENT_ASTAR_NODE), a);
        assert_eq!(e.mem.u32(this + BEST_NODE_SO_FAR), b);
        assert_eq!(
            calls_to(&log, QUEUE_PUSH_006B3E10),
            vec![vec![this + 0x1ff4, b]]
        );
        // An empty queue: 0, with the last node kept.
        let this = searching(&mut e, vec![], 0);
        e.mem.set_u32(this + LAST_SEARCHED_NODE, 0x99);
        assert_eq!(e.call(0x006a_1cf0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + LAST_SEARCHED_NODE), 0);
        // A connection that is the current node's parent ends with 0.
        let this = searching(&mut e, vec![a], 0);
        e.mem.set_u32(a + PARENT_ASTAR_NODE, b);
        assert_eq!(e.call(0x006a_1cf0, &args![this]).u32(), 0);
    }

    const TASKLET_SLOT: u32 = 0x0f39_0000;

    #[test]
    fn at_006a1630_builds_the_path_from_the_best_node() {
        let mut e = Engine::new();
        let this = searching(&mut e, vec![], 0);
        e.put_vtable(0x0f3a_0000, &[0, 0, 0, TASKLET_SLOT]);
        let manager = e.mem.alloc(4);
        e.mem.set_u32(manager, 0x0f3a_0000);
        e.register_double(TASKLET_MANAGER_00B00A00, move |_, _| manager.into_ret());
        e.register(THREAD_ID_0040FC90, |_, _| 0x77u32.into_ret());
        e.register(TASKLET_SLOT, |_, _| 1u32.into_ret());
        stub(&mut e, &[BUILD_PATH_006B3CB0]);
        e.mem.set_u32(this + BEST_NODE_SO_FAR, 0x5150);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x006a_1630, &args![this, 1u32, 2u32, 1u32]).bool());
        });
        assert_eq!(calls_to(&log, TASKLET_SLOT), vec![vec![manager, 0x77]]);
        assert_eq!(e.mem.u8(this + RUNNING_IN_TASKLET), 1);
        assert_eq!(
            calls_to(&log, BUILD_PATH_006B3CB0),
            vec![vec![this, 0x5150, 1, 2]]
        );
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x006a_1630, &args![this, 1u32, 2u32, 0u32]).bool());
        });
        assert!(calls_to(&log, BUILD_PATH_006B3CB0).is_empty());
    }

    /// Stubs for a POV list build over empty or small inputs.
    fn building(e: &mut Engine) {
        map_constants(e);
        arrays(e);
        vectors(e);
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        stub(
            e,
            &[
                ARRAY_INIT_006B3EB0,
                ARRAY_INIT_006DC440,
                VERTEX_LIST_CTOR_006917D0,
                VERTEX_LIST_DTOR_00691800,
                ARRAY_RELEASE_008454F0,
                ARRAY_RELEASE_0049FAF0,
                ARRAY_RESIZE_009F6540,
            ],
        );
        allocating_vtable(e, ARRAY_VTABLE_0106B410);
        e.register(ARRAY_PUSH_007CB2E0, |e, a| {
            let value = e.mem.u32(a[1]);
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
    }

    fn building_search(e: &mut Engine) -> u32 {
        let this = pov_search(e);
        e.put_vtable(0x0f3b_0000, &[0, ALLOCATE]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * 0x10).into_ret());
        e.mem.set_u32(this + POVS, 0x0f3b_0000);
        this
    }

    #[test]
    fn at_0069e3b0_makes_a_corner_pov() {
        let mut e = Engine::new();
        building(&mut e);
        let this = building_search(&mut e);
        // One triangle with one corner vertex (id 9) with flagged edges.
        let triangles = e.mem.alloc(0x10);
        let records = e.mem.alloc(8);
        e.mem.set_u32(records, 0x0a00);
        e.mem.set_u16(records + 4, 3);
        e.mem.set_u32(triangles + 4, records);
        e.mem.set_u32(triangles + 8, 1);
        e.register(CORNER_VERTICES_0069A830, |_, _| 1u32.into_ret());
        e.register(EDGE_VERTICES_0069A940, |_, _| Ret::default());
        let ids = e.mem.alloc(2);
        e.mem.set_u16(ids, 9);
        e.register_double(VERTEX_AT_005EEDA0, move |_, _| ids.into_ret());
        e.register(VERTEX_HANDLE_0069A660, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        e.register(COMPARE_006D4CA0, |_, _| Ret::default());
        e.register(HANDLE_APPEND_006DBA30, |e, a| {
            let record = e.mem.alloc(8);
            let (navmesh, id) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(record, navmesh);
            e.mem.set_u32(record + 4, id);
            e.mem.set_u32(a[0] + 4, record);
            e.mem.set_u32(a[0] + 8, 1);
            Ret::default()
        });
        e.register(CORNER_EDGES_0069A9D0, |_, _| 1u32.into_ret());
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 7.0);
        e.register_double(VERTEX_POSITION_0068F0A0, move |_, _| position.into_ret());
        stub(&mut e, &[TRIANGLE_005582F0]);
        e.register(EDGE_FLAG_0068F1D0, |_, _| 1u32.into_ret());
        e.register(GET_EDGE_VERTICES_0068F040, |e, a| {
            e.mem.set_u32(a[1], a[1]);
            e.mem.set_u32(a[1] + 4, a[1]);
            Ret::default()
        });
        let povs = list_of(&mut e, &[&[], &[]]);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0069_e3b0, &args![this, triangles, povs]).bool());
        });
        let pushes = calls_to(&log, ARRAY_PUSH_007CB2E0);
        assert_eq!(pushes.len(), 1);
        let list = e.mem.u32(povs + 4);
        assert_eq!(pushes[0][0], list);
        let made = e.mem.u32(e.mem.u32(list + 4));
        assert_eq!(e.mem.u32(made), 1);
        assert_eq!(e.mem.f32(made + 4), 7.0);
        assert_eq!(e.mem.f32(made + 0x10), 1.0);
        assert_eq!(e.mem.u16(made + 0x34), 9);
        // `POVs` was sized to the triangles and one more.
        assert_eq!(e.mem.u32(this + POVS + 8), 2);
    }

    #[test]
    fn at_0069e3b0_removes_duplicates_last_first() {
        let mut e = Engine::new();
        building(&mut e);
        let this = building_search(&mut e);
        let triangles = e.mem.alloc(0x10);
        let near = pov(&mut e, 0, (0.0, 0.0, 0.0), 1.0);
        let close = pov(&mut e, 0, (1.0, 0.0, 0.0), 1.0);
        let far = pov(&mut e, 0, (9.0, 0.0, 0.0), 1.0);
        for p in [near, close, far] {
            e.mem.set_u32(p + 0x30, 0);
        }
        e.set_global::<f32>(DUPLICATE_DISTANCE_SQUARED, 25.0);
        // `POVs` already holds one list with the three POVs.
        let lists = list_of(&mut e, &[&[near, close, far]]);
        let vtable = e.mem.u32(this + POVS);
        copy_words(&mut e, this + POVS, lists, 4);
        e.mem.set_u32(this + POVS, vtable);
        e.mem.set_u32(this + POVS + 0xc, 1);
        e.register(PAIR_004B0680, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(ARRAY_RESERVE_00845460, |e, a| {
            if e.mem.u32(a[0] + 4) == 0 {
                let buffer = e.mem.alloc(0x40);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        stub(&mut e, &[ARRAY_REMOVE_006BF8F0, OPERATOR_DELETE]);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0069_e3b0, &args![this, triangles, this + POVS])
                .bool());
        });
        // Only the first POV is near a later one: deleted and removed.
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![near]]);
        let list = e.mem.u32(this + POVS + 4);
        assert_eq!(
            calls_to(&log, ARRAY_REMOVE_006BF8F0),
            vec![vec![list, 0, 1]]
        );
    }

    /// A POV search ready for `0069f010` with one empty triangle.
    fn running(e: &mut Engine) -> (u32, u32) {
        building(e);
        e.register(ARRAY_PUSH_007CB2E0, |e, a| {
            let value = e.mem.u32(a[1]);
            if e.mem.u32(a[0] + 4) == 0 {
                let buffer = e.mem.alloc(0x40);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(CORNER_VERTICES_0069A830, |_, _| Ret::default());
        e.register(EDGE_VERTICES_0069A940, |_, _| Ret::default());
        e.register(LOCATION_POSITION_005C3420, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        stub(
            e,
            &[
                LOCATION_ASSIGN_006DCCE0,
                RESOLVE_NAVMESH_INFO_006DD6F0,
                START_NODE_006B32F0,
                QUEUE_PUSH_006B3E10,
                BUILD_PATH_006B3CB0,
                SCRAP_HOLDER_00401020,
                SCRAP_HEAP_00AA42E0,
            ],
        );
        e.register(QUEUE_POP_006B4350, |_, _| Ret::default());
        e.put_vtable(0x0f3a_0000, &[0, 0, 0, TASKLET_SLOT]);
        let manager = e.mem.alloc(4);
        e.mem.set_u32(manager, 0x0f3a_0000);
        e.register_double(TASKLET_MANAGER_00B00A00, move |_, _| manager.into_ret());
        stub(e, &[THREAD_ID_0040FC90, TASKLET_SLOT]);
        e.map(0x011d_6000, 0x1000);
        let this = building_search(e);
        let triangles = e.mem.alloc(0x10);
        let record = e.mem.alloc(8);
        e.mem.set_u32(triangles + 4, record);
        e.mem.set_u32(triangles + 8, 1);
        (this, triangles)
    }

    #[test]
    fn at_0069f010_stops_on_an_unresolved_start() {
        let mut e = Engine::new();
        let (this, triangles) = running(&mut e);
        e.register(LOCATION_RESOLVED_006DD6A0, |_, _| Ret::default());
        let start = e.mem.alloc(0x28);
        let goal = e.mem.alloc(0x28);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0069_f010,
                    &args![this, start, goal, triangles, 0x51u32, 0x52u32]
                )
                .bool());
        });
        assert_eq!(e.mem.u32(this + REQUEST), 0x51);
        assert_eq!(e.mem.u32(this + SOLUTION), 0x52);
        assert_eq!(
            calls_to(&log, LOCATION_ASSIGN_006DCCE0),
            vec![
                vec![this + START_LOCATION, start],
                vec![this + GOAL_LOCATION, goal]
            ]
        );
        assert!(calls_to(&log, START_NODE_006B32F0).is_empty());
    }

    #[test]
    fn at_0069f010_places_the_ends_and_searches() {
        let mut e = Engine::new();
        let (this, triangles) = running(&mut e);
        e.register(LOCATION_RESOLVED_006DD6A0, |_, _| 1u32.into_ret());
        let start = e.mem.alloc(0x28);
        let goal = e.mem.alloc(0x28);
        e.mem.set_f32(goal, 9.0);
        e.mem.set_f32(this + GOAL_LOCATION, 3.0);
        e.mem.set_f32(this + GOAL_LOCATION + 4, 4.0);
        let request = e.mem.alloc(0x70);
        e.mem.set_f32(request + 0x68, 1.0);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0069_f010,
                    &args![this, start, goal, triangles, request, 0u32]
                )
                .bool());
        });
        let start_pov = e.mem.u32(this + START_POV);
        let goal_pov = e.mem.u32(this + GOAL_POV);
        assert_eq!(e.mem.u32(start_pov), 3);
        assert_eq!(e.mem.u32(goal_pov), 4);
        assert_eq!(e.mem.f32(goal_pov + 4), 9.0);
        assert_eq!(e.mem.u16(goal_pov + 0x34), 0xffff);
        let started = calls_to(&log, START_NODE_006B32F0);
        assert_eq!(started.len(), 1);
        assert_eq!((started[0][0], started[0][1]), (this, start_pov));
        assert_eq!(started[0][3], 7.5f32.to_bits());
        assert_eq!(calls_to(&log, ARRAY_PUSH_007CB2E0).len(), 2);
    }

    #[test]
    fn at_0069f9d0_filters_the_other_povs() {
        let mut e = Engine::new();
        map_constants(&mut e);
        arrays(&mut e);
        vectors(&mut e);
        e.set_global::<f64>(HEIGHT_SCALE, 1.0);
        e.register(LENGTH_2D_00595C80, |e, a| {
            let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            f64::from((x * x + y * y).sqrt()).into_ret()
        });
        let link = e.mem.alloc(0x40);
        e.register_double(PARENT_LINK_007D6BB0, move |_, _| link.into_ret());
        stub(
            &mut e,
            &[
                CONSTRUCT_ARRAY_00401050,
                BASE_CTOR_006240D0,
                HOLDER_CTOR_0042FB00,
                HOLDER_DTOR_0042FA40,
                GET_NAVMESH_AND_TRIANGLE_006DD640,
            ],
        );
        e.register(TANGENTS_006BB570, |_, _| Ret::default());
        let own = pov(&mut e, 3, (0.0, 0.0, 0.0), 1.0);
        let high = pov(&mut e, 0, (10.0, 0.0, 20.0), 1.0);
        let level = pov(&mut e, 0, (10.0, 0.0, 1.0), 1.0);
        let this = pov_search(&mut e);
        let lists = list_of(&mut e, &[&[own, high, level]]);
        copy_words(&mut e, this + POVS, lists, 4);
        let node = e.mem.alloc(0x68);
        e.mem.set_u32(node + 8, own);
        let connections = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0069_f9d0, &args![this, node, connections]).bool());
        });
        // Only the level POV gets to the tangents.
        let asked = calls_to(&log, TANGENTS_006BB570);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][2], 0);
        // A tangent without a parent needs the start location's navmesh:
        // none, so false.
        e.register(TANGENTS_006BB570, |_, _| 1u32.into_ret());
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0069_f9d0, &args![this, node, connections]).bool());
        });
        assert_eq!(calls_to(&log, GET_NAVMESH_AND_TRIANGLE_006DD640).len(), 1);
        assert_eq!(calls_to(&log, HOLDER_DTOR_0042FA40).len(), 1);
    }
}
