//! `BSMain/bsfurnituremarker.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! x87 note: `FurnitureMark::SetHeading` computes in extended precision and
//! stores `float` results. The translation computes in `f64` and rounds to
//! `f32` at each store; the product it truncates (a `float` times 1000) is
//! exact in `f64`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c54130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSFurnitureMarker` vtable word 010c2ae4, right after
/// `BSFurnitureMarker::LoadBinary` (010c2ae0). Forwards `this` and the stream to `00a7b3b0`
/// (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c54130(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

/// `00a7b290` (named `NiExtraData::RegisterStreamables` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00A7B290: u32 = 0x00a7_b290;

// Translated from 00c54150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::RegisterStreamables` (engine map vtable name, vtable word 010c2ae8):
/// forwards `this` and the stream to `00a7b290` (`NiExtraData::RegisterStreamables`). Returns
/// the `bool` in `AL`: Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns
/// `_N`; `stream` is the `NiStream` reference as an address. Parameter count from the `RET 4`
/// of both bodies.
pub fn bs_furniture_marker_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00A7B290, &args![this, stream])
        .bool()
}

/// The full circle in radians (`float`, `012031cc`): the range the heading
/// is wrapped into.
const FULL_CIRCLE: u32 = 0x0120_31cc;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1000.0` (`double`): the scale of the stored heading.
const HEADING_SCALE: u32 = 0x0101_7b70;

layout! {
    /// `FurnitureMark` (Xbox PDB), 0x10 bytes.
    pub struct FurnitureMark: 0x10 {
        /// `sHeading` (Xbox PDB): the heading in thousandths of a radian.
        0x0c sHeading: u16,
        /// `cNumber` (Xbox PDB).
        0x0e cNumber: u8,
    }
}

/// `FISTP dword` with the control word set to truncate: the integer, or
/// `0x80000000` ("integer indefinite") for a NaN or a value out of range.
fn truncate_to_i32(value: f64) -> i32 {
    let whole = value.trunc();
    if whole.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&whole) {
        i32::MIN
    } else {
        whole as i32
    }
}

// Translated from 00c54550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FurnitureMark::SetHeading` (Xbox PDB): wraps `heading` into the range
/// from 0 up to the full circle (adds the circle while the heading is below
/// `0.0`, subtracts it while the circle is not above the heading; a NaN stays
/// as it is), and stores the heading times 1000, truncated, as the low 16
/// bits at `+0xc`.
pub fn furniture_mark_set_heading(e: &mut Engine, this: Ptr<FurnitureMark>, heading: f32) {
    let circle: f32 = e.global(FULL_CIRCLE);
    let zero: f64 = e.global(ZERO);
    let mut heading = heading;
    while (heading as f64) < zero {
        heading = (heading as f64 + circle as f64) as f32;
    }
    while (circle as f64) <= heading as f64 {
        heading = (heading as f64 - circle as f64) as f32;
    }
    let scale: f64 = e.global(HEADING_SCALE);
    let scaled = truncate_to_i32(heading as f64 * scale);
    e.set(this, FurnitureMark::sHeading, scaled as u16);
}

// Translated from 00c54d60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031c4 without reading the receiver. The map names this body `BSFurnitureMarker::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c54d60(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31c4
}

/// `BSFurnitureMarker` vtable (RTTI).
const VTABLE_FURNITURE_MARKER: u32 = 0x010c_2a94;
/// `NiTObjectArray<FurnitureMark>` vtable (RTTI), the marker's array at +0xc.
const VTABLE_OBJECT_ARRAY: u32 = 0x010c_2a8c;
/// `NiTArray<FurnitureMark, NiTNewInterface<FurnitureMark>>` vtable (RTTI).
const VTABLE_TARRAY: u32 = 0x010c_2b2c;
/// The `NiExtraData` default constructor (`00a7b2e0`, thiscall).
const EXTRA_DATA_DEFAULT_CONSTRUCTOR: u32 = 0x00a7_b2e0;
/// The `NiExtraData` destructor body (`00a7b300`, thiscall).
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// `NiExtraData::LoadBinary` (`00a7b4a0`, engine map; thiscall, `RET 4`: the stream).
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
/// `00a59be0` (named `NiColor::LoadBinary` in the engine map; thiscall on three floats,
/// `RET 4`: the stream).
const THREE_FLOATS_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `00a26df0` (cdecl: the stream, the address of a 32-bit count to read).
const LOAD_COUNT: u32 = 0x00a2_6df0;
/// `00a30370` (cdecl: the stream, the address of a 16-bit value to read).
const LOAD_SHORT: u32 = 0x00a3_0370;
/// `NiStreamLoadBinary<unsigned char>` (`00a643b0`, engine map; cdecl: the stream, the
/// address of the byte).
const LOAD_BYTE: u32 = 0x00a6_43b0;
/// The name setter `00a5b950` (unit `NiMain/niobjectnet.cpp`; thiscall, `RET 4`).
const SET_NAME: u32 = 0x00a5_b950;
/// The word whose address the factory passes to `00a5b950`.
const CREATED_NAME: u32 = 0x011f_49e0;
/// `NiNew` allocation (`00aa13e0`, cdecl: size).
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiDelete` of a sized block (`00aa1460`, cdecl: the block, its size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`, engine map; thiscall, `RET 4`: the name).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `BSFurnitureMarker::ms_RTTI` (the address `00c54d60` returns).
const FURNITURE_MARKER_RTTI: u32 = 0x0120_31c4;
/// The global holding `BSFurnitureMarker`'s extra-data name (an allocated `NiFixedString`).
const FURNITURE_MARKER_NAME: u32 = 0x0120_31c0;
/// `"FRN"`.
const FURNITURE_MARKER_NAME_TEXT: u32 = 0x010c_2b5c;
/// `NiTArray<FurnitureMark>::SetAt`'s function-local `static FurnitureMark` (the empty
/// element) and its initialization guard (bit 0).
const EMPTY_MARK: u32 = 0x0120_31d0;
const EMPTY_MARK_GUARD: u32 = 0x0120_31e0;
/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `NiFixedString` maker (`00a5b690`, cdecl: the text; returns the handle).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The empty `NiFixedString` handle.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Copies the 16 bytes of a `FurnitureMark` as four words.
fn copy_mark(e: &mut Engine, to: u32, from: u32) {
    for i in 0..4u32 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// A `FurnitureMark` temporary on the stack as its default constructor leaves it: heading
/// and number 0, the offset (and the padding byte) whatever the stack slot holds.
fn default_mark(e: &mut Engine, mark: u32) {
    e.mem.set_u16(mark + 0xc, 0);
    e.mem.set_u8(mark + 0xe, 0);
}

// Translated from 00c54cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FurnitureMark::operator==` (engine map): the three offset floats compare equal
/// (`FUCOMPP`, parity tested: an unordered pair is not equal), then the numbers (+0xe) and
/// headings (+0xc) are the same.
pub fn furniture_mark_operator_eq(e: &mut Engine, this: Ptr<FurnitureMark>, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    for i in 0..3u32 {
        let left = f32::from_bits(e.mem.u32(a + 4 * i));
        let right = f32::from_bits(e.mem.u32(b + 4 * i));
        if left != right {
            return false;
        }
    }
    e.mem.u8(a + 0xe) == e.mem.u8(b + 0xe) && e.mem.u16(a + 0xc) == e.mem.u16(b + 0xc)
}

// Translated from 00c54bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTNewInterface<FurnitureMark>` allocation of `count` marks: `count * 16` bytes
/// (`0xffffffff` when that overflows) from the memory manager, each mark's heading set to 0
/// and number to 0xff (the loop counts down while the count stays non-negative as a signed
/// number). Returns the block, or 0.
pub fn fn_00c54bc0(e: &mut Engine, count: u32) -> u32 {
    let bytes = count.checked_mul(0x10).unwrap_or(0xffff_ffff);
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    if block == 0 {
        return 0;
    }
    let mut left = count as i32;
    let mut mark = block;
    loop {
        left = left.wrapping_sub(1);
        if left < 0 {
            break;
        }
        e.mem.set_u16(mark + 0xc, 0);
        e.mem.set_u8(mark + 0xe, 0xff);
        mark = mark.wrapping_add(0x10);
    }
    block
}

// Translated from 00c54c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<FurnitureMark, NiTNewInterface<..>>` constructor (class by the vtable's RTTI):
/// `m_usMaxSize` = `max_size`, `m_usGrowBy` = `grow_by`, the size and count 0, and
/// `m_pBase` from `00c54bc0` (null when `max_size` is 0). Returns `this`.
pub fn ni_t_array_furniture_mark_constructor(
    e: &mut Engine,
    this: Ptr,
    max_size: u16,
    grow_by: u16,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_TARRAY);
    e.mem.set_u16(base + 8, max_size);
    e.mem.set_u16(base + 0xe, grow_by);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let count = u32::from(e.mem.u16(base + 8));
    if count == 0 {
        e.mem.set_u32(base + 4, 0);
    } else {
        let block = fn_00c54bc0(e, count);
        e.mem.set_u32(base + 4, block);
    }
    this
}

// Translated from 00c54850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<FurnitureMark, ..>::SetSize` (engine map): nothing when `new_size` is the
/// capacity. Shrinking below `m_usSize` resets each dropped slot that is not empty (not equal
/// to a default mark, `00c54cc0`) to a default mark and counts one element less; the size
/// becomes `new_size`. The capacity becomes `new_size`, the elements move into a new block
/// (`00c54bc0`; none for 0) with the slots after the size filled with default marks, and the
/// old block goes back to the memory manager. The default marks are stack temporaries:
/// heading and number 0, the rest whatever the stack slot holds.
pub fn ni_t_array_furniture_mark_set_size(e: &mut Engine, this: Ptr, new_size: u32) {
    let base = this.addr();
    if new_size == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    // The 0x5c-byte frame: the temporaries at EBP-0x18, -0x28 and -0x38.
    e.with_stack(0x5c, |e, frame| {
        let compared = frame.addr() + 0x44;
        let shrunk = frame.addr() + 0x34;
        let filled = frame.addr() + 0x24;
        if new_size < u32::from(e.mem.u16(base + 0xa)) {
            let mut index = new_size as u16;
            while u32::from(index) < u32::from(e.mem.u16(base + 0xa)) {
                default_mark(e, compared);
                let slot = (u32::from(index) << 4).wrapping_add(e.mem.u32(base + 4));
                if !furniture_mark_operator_eq(e, Ptr::new(slot), Ptr::new(compared)) {
                    default_mark(e, shrunk);
                    let slot = e.mem.u32(base + 4).wrapping_add(u32::from(index) << 4);
                    copy_mark(e, slot, shrunk);
                    let count = e.mem.u16(base + 0xc);
                    e.mem.set_u16(base + 0xc, count.wrapping_sub(1));
                }
                index = index.wrapping_add(1);
            }
            e.mem.set_u16(base + 0xa, new_size as u16);
        }
        let old = e.mem.u32(base + 4);
        e.mem.set_u16(base + 8, new_size as u16);
        if new_size != 0 {
            let capacity = u32::from(e.mem.u16(base + 8));
            let block = fn_00c54bc0(e, capacity);
            e.mem.set_u32(base + 4, block);
            let mut index = 0u16;
            while u32::from(index) < u32::from(e.mem.u16(base + 0xa)) {
                let from = (u32::from(index) << 4).wrapping_add(old);
                let to = e.mem.u32(base + 4).wrapping_add(u32::from(index) << 4);
                copy_mark(e, to, from);
                index = index.wrapping_add(1);
            }
            let mut index = e.mem.u16(base + 0xa);
            while u32::from(index) < u32::from(e.mem.u16(base + 8)) {
                default_mark(e, filled);
                let to = e.mem.u32(base + 4).wrapping_add(u32::from(index) << 4);
                copy_mark(e, to, filled);
                index = index.wrapping_add(1);
            }
        } else {
            e.mem.set_u32(base + 4, 0);
        }
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
    });
}

// Translated from 00c54a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<FurnitureMark, ..>::SetAt` (engine map): with the function's static empty mark
/// (`012031d0`; heading and number cleared on first use, guard bit 0 of `012031e0`): an
/// index at or past `m_usSize` raises it to index + 1 and counts the element when the new
/// one is not empty; below it, a non-empty item over an empty slot counts one more, an empty
/// item over a non-empty slot one less. Then the item's 16 bytes are copied into the slot.
pub fn ni_t_array_furniture_mark_set_at(e: &mut Engine, this: Ptr, index: u32, item: Ptr) {
    let base = this.addr();
    let guard = e.global::<u32>(EMPTY_MARK_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(EMPTY_MARK_GUARD, guard | 1);
        e.set_global::<u16>(EMPTY_MARK + 0xc, 0);
        e.set_global::<u8>(EMPTY_MARK + 0xe, 0);
    }
    let empty = Ptr::new(EMPTY_MARK);
    let count_by = |e: &mut Engine, delta: i16| {
        let count = e.mem.u16(base + 0xc);
        e.mem.set_u16(base + 0xc, count.wrapping_add(delta as u16));
    };
    if index >= u32::from(e.mem.u16(base + 0xa)) {
        e.mem.set_u16(base + 0xa, index.wrapping_add(1) as u16);
        if !furniture_mark_operator_eq(e, Ptr::new(item.addr()), empty) {
            count_by(e, 1);
        }
    } else {
        let slot = (index << 4).wrapping_add(e.mem.u32(base + 4));
        if !furniture_mark_operator_eq(e, Ptr::new(item.addr()), empty) {
            if furniture_mark_operator_eq(e, Ptr::new(slot), empty) {
                count_by(e, 1);
            }
        } else if !furniture_mark_operator_eq(e, Ptr::new(slot), empty) {
            count_by(e, -1);
        }
    }
    let slot = e.mem.u32(base + 4).wrapping_add(index << 4);
    copy_mark(e, slot, item.addr());
}

// Translated from 00c54b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<FurnitureMark, ..>::Add` (engine map): at the size index; when that reaches the
/// capacity, first grows it by `m_usGrowBy` (`SetSize`). Returns the index.
pub fn ni_t_array_furniture_mark_add(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let base = this.addr();
    let index = u32::from(e.mem.u16(base + 0xa));
    if index >= u32::from(e.mem.u16(base + 8)) {
        let grown = u32::from(e.mem.u16(base + 0xe)).wrapping_add(index);
        ni_t_array_furniture_mark_set_size(e, this, grown);
    }
    ni_t_array_furniture_mark_set_at(e, this, index, item);
    index
}

// Translated from 00c545d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FurnitureMark::LoadBinary` (engine map): the offset (three floats, `00a59be0`), the
/// heading as a 16-bit value (`00a30370`), the number byte and one more byte into a local
/// that is dropped (`00a643b0` twice). A negative stored heading is read as thousandths of a
/// radian (`FILD`, `FDIV` by the `double` 1000.0 at `01017b70`, stored as a float) and set
/// through `SetHeading` (`00c54550`).
pub fn furniture_mark_load_binary(e: &mut Engine, this: Ptr<FurnitureMark>, stream: Ptr) {
    let base = this.addr();
    e.call(THREE_FLOATS_LOAD_BINARY, &args![base, stream]);
    e.call(LOAD_SHORT, &args![stream, base + 0xc]);
    e.call(LOAD_BYTE, &args![stream, base + 0xe]);
    e.with_stack(4, |e, dropped| {
        e.call(LOAD_BYTE, &args![stream, dropped]);
    });
    let heading = e.mem.u16(base + 0xc) as i16;
    if heading < 0 {
        let radians = x87_div(f64::from(heading), e.global::<f64>(HEADING_SCALE));
        let stored = x87_quiet_bits(x87_store32(radians));
        furniture_mark_set_heading(e, this, f32::from_bits(stored));
    }
}

// Translated from 00c543e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Names the marker with `BSFurnitureMarker`'s extra-data name (`00a5b950` with the global
/// at `012031c0`).
pub fn fn_00c543e0(e: &mut Engine, this: Ptr) {
    let name = e.global::<u32>(FURNITURE_MARKER_NAME);
    e.call(SET_NAME, &args![this, name]);
}

// Translated from 00c53f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53f10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f553, state variable) is not translated
/// `BSFurnitureMarker` constructor (vtables by RTTI): the `NiExtraData` default constructor
/// (`00a7b2e0`), the marker's vtable, the mark array at +0xc constructed empty with growth 1
/// (`00c54c50`) and given the `NiTObjectArray<FurnitureMark>` vtable, and the marker's name
/// (`00c543e0`). Returns `this`.
pub fn bs_furniture_marker_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EXTRA_DATA_DEFAULT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, VTABLE_FURNITURE_MARKER);
    ni_t_array_furniture_mark_constructor(e, Ptr::new(base + 0xc), 0, 1);
    e.mem.set_u32(base + 0xc, VTABLE_OBJECT_ARRAY);
    fn_00c543e0(e, this);
    this
}

// Translated from 00c53fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53fa0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f576, state variable) is not translated
/// `BSFurnitureMarker` destructor: the marker's vtable, the array's `NiTArray` vtable, the
/// array's block (+0x10) back to the memory manager, then the `NiExtraData` destructor
/// (`00a7b300`).
pub fn bs_furniture_marker_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_FURNITURE_MARKER);
    e.mem.set_u32(base + 0xc, VTABLE_TARRAY);
    let block = e.mem.u32(base + 0x10);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 00c547e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker`'s scalar deleting destructor (engine map): the destructor
/// (`00c53fa0`), then, for bit 0 of `flags`, `00aa1460(this, 0x1c)`. Returns `this`.
pub fn bs_furniture_marker_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bs_furniture_marker_destructor(e, this);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00c540a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::LoadBinary` (engine map): the `NiExtraData` part (`00a7b4a0`), a
/// 32-bit count (`00a26df0`); for a non-zero count the array is sized to it (`SetSize`) and
/// that many marks are read into one stack mark (heading 0 and number 0xff set once before
/// the loop, `FurnitureMark::LoadBinary`) and added (`Add`).
pub fn bs_furniture_marker_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    // The 0xcc-byte frame: the count at EBP-4, the mark at EBP-0x14.
    e.with_stack(0xcc, |e, frame| {
        let count = frame.addr() + 0xc8;
        let mark = frame.addr() + 0xb8;
        e.call(LOAD_COUNT, &args![stream, count]);
        if e.mem.u32(count) == 0 {
            return;
        }
        let total = e.mem.u32(count);
        ni_t_array_furniture_mark_set_size(e, Ptr::new(base + 0xc), total);
        e.mem.set_u16(mark + 0xc, 0);
        e.mem.set_u8(mark + 0xe, 0xff);
        let mut index = 0u32;
        while index < e.mem.u32(count) {
            furniture_mark_load_binary(e, Ptr::new(mark), stream);
            ni_t_array_furniture_mark_add(e, Ptr::new(base + 0xc), Ptr::new(mark));
            index += 1;
        }
    });
}

// Translated from 00c54210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c54210 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f5a0, state variable) is not translated
/// The `BSFurnitureMarker` factory (no arguments): a 0x1c-byte `NiNew` block constructed
/// (`00c53f10`), or null when the allocation fails; then `00a5b950` runs on the result (also
/// when it is null) with `011f49e0`. Returns the result.
pub fn fn_00c54210(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x1cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        bs_furniture_marker_constructor(e, Ptr::new(block)).addr()
    };
    e.call(SET_NAME, &args![object, CREATED_NAME]);
    object
}

// Translated from 00c54400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::Find` (engine map): null for a null object; otherwise the extra data
/// named by the marker name global (`NiObjectNET::GetExtraData`), kept only when its RTTI
/// chain (the virtual at +8, then each record's parent at +4) reaches the marker's
/// (`012031c4`).
pub fn bs_furniture_marker_find(e: &mut Engine, object: Ptr) -> u32 {
    if object.addr() == 0 {
        return 0;
    }
    let name = e.global::<u32>(FURNITURE_MARKER_NAME);
    let data = e.call(GET_EXTRA_DATA, &args![object, name]).u32();
    if data == 0 {
        return 0;
    }
    let mut rtti = e.vcall(data, 8, &args![]).u32();
    while rtti != 0 {
        if rtti == FURNITURE_MARKER_RTTI {
            return data;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    0
}

// Translated from 00c546c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c546c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f5c6, state variable) is not translated
/// The start-up initializer of the marker name: a 4-byte `NiFixedString` from the memory
/// manager set to `"FRN"` (`00a5b690`); the compiler kept a test of the constant text's
/// address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) never runs (translated anyway). The string (or null when the allocation
/// failed) goes to `012031c0`.
pub fn fn_00c546c0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(FURNITURE_MARKER_NAME_TEXT);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![FURNITURE_MARKER_NAME_TEXT])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(FURNITURE_MARKER_NAME, stored);
}

// Translated from 00c54760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the name exists, releases its handle (unless it is the
/// empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the text),
/// frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always true),
/// and clears the global.
pub fn fn_00c54760(e: &mut Engine) {
    let string = e.global::<u32>(FURNITURE_MARKER_NAME);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(FURNITURE_MARKER_NAME, 0);
}

/// `NiExtraData::SaveBinary` (`00a7b2c0`), `IsEqual` (`00a7b3d0`) and
/// `GetViewerStrings` (`00a7b580`): `thiscall`, one word each.
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_IS_EQUAL: u32 = 0x00a7_b3d0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `0044adf0` (unnamed, `cdecl`, `(stream, &value)`): writes a 32-bit
/// count; `NiStreamSaveBinary<unsigned char>` (`00a29550`, `cdecl`,
/// `(stream, &value)`); `NiColor::SaveBinary` (`00a69780`, `thiscall` on
/// three floats, the stream).
const SAVE_COUNT: u32 = 0x0044_adf0;
const SAVE_BYTE: u32 = 0x00a2_9550;
const THREE_FLOATS_SAVE_BINARY: u32 = 0x00a6_9780;
/// `NiGetViewerString` (`00a7d330`, the class name) and its integer
/// (`00a7d510`) and `float` (`00a7d550`) overloads (`cdecl`, `(name,
/// value)`), `NiPoint3::GetViewerString` (`00a59af0`, `thiscall` on the
/// point, the name) and the viewer-string array's `Add` (`0096a610`,
/// `thiscall` on the array, the string's address).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_INT: u32 = 0x00a7_d510;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const STRINGS_ADD: u32 = 0x0096_a610;
/// The viewer-string names: "iMarks" (`010c2b54`), "Mark" (`010c2b4c`), "
/// Number" (`010c2b44`), " Pos" (`010c2b3c`) and " Heading" (`010c2b30`).
const NAME_MARKS: u32 = 0x010c_2b54;
const NAME_MARK: u32 = 0x010c_2b4c;
const NAME_NUMBER: u32 = 0x010c_2b44;
const NAME_POSITION: u32 = 0x010c_2b3c;
const NAME_HEADING: u32 = 0x010c_2b30;
/// `BSFurnitureMarker`'s mark array (`NiTObjectArray<FurnitureMark>` at
/// +0xC): its elements (+0x10, 16 bytes each) and its 16-bit element count
/// (+0x18).
const MARKS: u32 = 0x0c;
const MARK_DATA: u32 = 0x10;
const MARK_COUNT: u32 = 0x18;
/// `NiStream`'s output stream (+0x250); its write function is at +0xC
/// (`cdecl (stream, source, size, &count, 1)`), its position at +4.
const STREAM_OUTPUT: u32 = 0x250;

// Translated from 00a303a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamSaveBinary<unsigned short>` (engine map; `cdecl`, `(stream,
/// &value)`): two bytes from `value` through the stream's output (the write
/// function at +0xC, the count word in the stream argument's slot set to
/// 2), the result added to the output's position.
pub fn ni_stream_save_binary_unsigned_short(e: &mut Engine, stream: Ptr, value: Ptr) {
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0xc);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 2);
        let put = e
            .call(write, &args![output, value, 2u32, count, 1u32])
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(put));
    });
}

// Translated from 00c54170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::SaveBinary` (Xbox PDB): `NiExtraData`'s
/// (`00a7b2c0`), the mark count (the array's 16-bit count widened, through
/// `0044adf0` from a local), then each mark's `SaveBinary` (`00c54660`).
pub fn bs_furniture_marker_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    let count = u32::from(e.mem.u16(this.addr() + MARK_COUNT));
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), count);
        e.call(SAVE_COUNT, &args![stream, local]);
    });
    for index in 0..count {
        let mark = (index << 4).wrapping_add(e.mem.u32(this.addr() + MARK_DATA));
        furniture_mark_save_binary(e, Ptr::new(mark), stream);
    }
}

// Translated from 00c541e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::IsEqual` (Xbox PDB): answers `NiExtraData::IsEqual`
/// (`00a7b3d0`): its AL as it is.
pub fn bs_furniture_marker_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    e.call(EXTRA_DATA_IS_EQUAL, &args![this, other]).u8()
}

// Translated from 00c542a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::GetViewerStrings` (Xbox PDB): `NiExtraData`'s
/// (`00a7b580`), the class name, "iMarks" with the count, then for each
/// mark "Mark" with its index, " Number" with its number byte, " Pos" with
/// its offset (`NiPoint3::GetViewerString`) and " Heading" with the heading
/// (`FILD` of the 16-bit value, `FDIV` by the `double` 1000.0, stored as a
/// float). Each string is added through `0096a610` with its local's address.
pub fn bs_furniture_marker_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let add = |e: &mut Engine, text: u32| {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), text);
            e.call(STRINGS_ADD, &args![strings, slot]);
        });
    };
    let class_name = e.global::<u32>(FURNITURE_MARKER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add(e, text);
    let count = u32::from(e.mem.u16(this.addr() + MARK_COUNT));
    let text = e
        .call(NI_GET_VIEWER_STRING_INT, &args![NAME_MARKS, count])
        .u32();
    add(e, text);
    for index in 0..count {
        let text = e
            .call(NI_GET_VIEWER_STRING_INT, &args![NAME_MARK, index])
            .u32();
        add(e, text);
        let mark = (index << 4).wrapping_add(e.mem.u32(this.addr() + MARK_DATA));
        let number = u32::from(e.mem.u8(mark + 0xe));
        let text = e
            .call(NI_GET_VIEWER_STRING_INT, &args![NAME_NUMBER, number])
            .u32();
        add(e, text);
        let text = e
            .call(POINT_GET_VIEWER_STRING, &args![mark, NAME_POSITION])
            .u32();
        add(e, text);
        let scale: f64 = e.global(HEADING_SCALE);
        let heading = f64::from(i32::from(e.mem.u16(mark + 0xc)));
        let heading = x87_store32(x87_div(heading, scale));
        let text = e
            .call(NI_GET_VIEWER_STRING_FLOAT, &args![NAME_HEADING, heading])
            .u32();
        add(e, text);
    }
}

// Translated from 00c54490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the marker, one word: a mark):
/// inserts `mark` into the mark array (+0xC) keeping the number bytes in
/// order: the position is the first mark whose number is above the new
/// one's; the mark is appended (`Add`, `00c54b70`), and when the position is
/// inside the old count every mark from it on moves up one (`SetAt`,
/// `00c54a30`, from the last) and the new one is set at the position.
pub fn fn_00c54490(e: &mut Engine, this: Ptr, mark: Ptr) {
    let t = this.addr();
    let mut count = u32::from(e.mem.u16(t + MARK_COUNT));
    let number = e.mem.u8(mark.addr() + 0xe);
    let mut position = 0u32;
    while position < count {
        let existing = (position << 4).wrapping_add(e.mem.u32(t + MARK_DATA));
        if number < e.mem.u8(existing + 0xe) {
            break;
        }
        position += 1;
    }
    let array = Ptr::new(t + MARKS);
    ni_t_array_furniture_mark_add(e, array, mark);
    if position < count {
        while count > position {
            let previous = ((count - 1) << 4).wrapping_add(e.mem.u32(t + MARK_DATA));
            ni_t_array_furniture_mark_set_at(e, array, count, Ptr::new(previous));
            count -= 1;
        }
        ni_t_array_furniture_mark_set_at(e, array, position, mark);
    }
}

// Translated from 00c54660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FurnitureMark::SaveBinary` (engine map): the offset (`NiColor::SaveBinary`,
/// three floats), the heading (`00a303a0`), then the number byte twice
/// (`00a29550` both times with +0xE, as the exe has it).
pub fn furniture_mark_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(THREE_FLOATS_SAVE_BINARY, &args![this, stream]);
    ni_stream_save_binary_unsigned_short(e, stream, Ptr::new(this.addr() + 0xc));
    e.call(SAVE_BYTE, &args![stream, this.addr() + 0xe]);
    e.call(SAVE_BYTE, &args![stream, this.addr() + 0xe]);
}

// Translated from 00c54810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no words): the
/// `NiTArray<FurnitureMark>` destructor body: its vtable (`010c2b2c`), then
/// the element block (+4) freed through the memory manager (`00aa4060`).
pub fn fn_00c54810(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TARRAY);
    let block = e.mem.u32(this.addr() + 4);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00c54d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTObjectArray<FurnitureMark>::_scalar_deleting_destructor_` (Xbox PDB):
/// the `NiTArray` vtable, the element block freed through the memory
/// manager, and with bit 0 of `flags` the array itself freed the same way.
/// Answers `this`.
pub fn ni_t_object_array_furniture_mark_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_TARRAY);
    let block = e.mem.u32(this.addr() + 4);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_03a0, ni_stream_save_binary_unsigned_short(Ptr, Ptr)),
        entry!(0x00c5_4170, bs_furniture_marker_save_binary(Ptr, Ptr)),
        entry!(0x00c5_41e0, bs_furniture_marker_is_equal(Ptr, Ptr) -> u8),
        entry!(
            0x00c5_42a0,
            bs_furniture_marker_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c5_4490, fn_00c54490(Ptr, Ptr)),
        entry!(0x00c5_4660, furniture_mark_save_binary(Ptr, Ptr)),
        entry!(0x00c5_4810, fn_00c54810(Ptr)),
        entry!(
            0x00c5_4d70,
            ni_t_object_array_furniture_mark_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c5_4d60, fn_00c54d60(Ptr) -> u32),
        entry!(0x00c5_4130, fn_00c54130(Ptr, Ptr)),
        entry!(0x00c5_4150, bs_furniture_marker_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c54550,
            furniture_mark_set_heading(Ptr<FurnitureMark>, f32)
        ),
        entry!(
            0x00c5_4cc0,
            furniture_mark_operator_eq(Ptr<FurnitureMark>, Ptr) -> bool
        ),
        entry!(0x00c5_4bc0, fn_00c54bc0(u32) -> u32),
        entry!(
            0x00c5_4c50,
            ni_t_array_furniture_mark_constructor(Ptr, u16, u16) -> Ptr
        ),
        entry!(0x00c5_4850, ni_t_array_furniture_mark_set_size(Ptr, u32)),
        entry!(0x00c5_4a30, ni_t_array_furniture_mark_set_at(Ptr, u32, Ptr)),
        entry!(0x00c5_4b70, ni_t_array_furniture_mark_add(Ptr, Ptr) -> u32),
        entry!(
            0x00c5_45d0,
            furniture_mark_load_binary(Ptr<FurnitureMark>, Ptr)
        ),
        entry!(0x00c5_43e0, fn_00c543e0(Ptr)),
        entry!(0x00c5_3f10, bs_furniture_marker_constructor(Ptr) -> Ptr),
        entry!(0x00c5_3fa0, bs_furniture_marker_destructor(Ptr)),
        entry!(
            0x00c5_47e0,
            bs_furniture_marker_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c5_40a0, bs_furniture_marker_load_binary(Ptr, Ptr)),
        entry!(0x00c5_4210, fn_00c54210() -> u32),
        entry!(0x00c5_4400, bs_furniture_marker_find(Ptr) -> u32),
        entry!(0x00c5_46c0, fn_00c546c0()),
        entry!(0x00c5_4760, fn_00c54760()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c54130_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_4130, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bs_furniture_marker_register_streamables_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00A7B290, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c5_4150, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00A7B290)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod tests_b {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FULL_CIRCLE, 4);
        e.set_global(FULL_CIRCLE, 6.0f32);
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e.map(HEADING_SCALE, 8);
        e.set_global(HEADING_SCALE, 1000.0f64);
        e
    }

    fn heading_of(e: &mut Engine, heading: f32) -> u16 {
        let mark = e.mem.alloc(0x10);
        e.mem.write(mark, &[0xee; 0x10]);
        e.call(0x00c54550, &args![mark, heading]);
        assert_eq!(e.mem.u16(mark + 0xe), 0xeeee, "only the heading is written");
        e.mem.u16(mark + 0xc)
    }

    #[test]
    fn a_heading_inside_the_circle_is_scaled_and_truncated() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, 1.25), 1250);
        assert_eq!(heading_of(&mut e, 0.0), 0);
        assert_eq!(heading_of(&mut e, 0.0009), 0);
    }

    #[test]
    fn a_negative_heading_is_wrapped_up() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, -1.0), 5000);
        assert_eq!(heading_of(&mut e, -13.0), 5000);
    }

    #[test]
    fn a_heading_at_or_past_the_circle_is_wrapped_down() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, 6.0), 0);
        assert_eq!(heading_of(&mut e, 7.5), 1500);
        assert_eq!(heading_of(&mut e, 20.0), 2000);
    }

    #[test]
    fn a_nan_heading_stores_the_low_half_of_the_integer_indefinite() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, f32::NAN), 0);
    }

    #[test]
    fn the_stored_value_is_the_low_16_bits() {
        let mut e = engine();
        e.set_global(FULL_CIRCLE, 100.0f32);
        // 70.0 * 1000 = 70000 = 0x11170.
        assert_eq!(heading_of(&mut e, 70.0), 0x1170);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c54d60_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_4d60, &args![0x1000u32]).u32(), 0x0120_31c4);
        assert_eq!(e.call(0x00c5_4d60, &args![0u32]).u32(), 0x0120_31c4);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    fn mark(e: &mut Engine, offset: [f32; 3], heading: u16, number: u8) -> u32 {
        let m = e.mem.alloc(0x10);
        for (i, v) in offset.iter().enumerate() {
            e.mem.set_u32(m + 4 * i as u32, v.to_bits());
        }
        e.mem.set_u16(m + 0xc, heading);
        e.mem.set_u8(m + 0xe, number);
        m
    }

    /// The memory manager doubled with the engine heap.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.map(EMPTY_MARK, 0x14);
        e
    }

    /// An array with `marks` (all counted) and capacity `capacity`, growth 2.
    fn array(e: &mut Engine, marks: &[u32], capacity: u16) -> u32 {
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10 * u32::from(capacity));
        for (i, m) in marks.iter().enumerate() {
            copy_mark(e, data + 0x10 * i as u32, *m);
        }
        e.mem.set_u32(this + 4, data);
        e.mem.set_u16(this + 8, capacity);
        e.mem.set_u16(this + 0xa, marks.len() as u16);
        e.mem.set_u16(this + 0xc, marks.len() as u16);
        e.mem.set_u16(this + 0xe, 2);
        this
    }

    #[test]
    fn mark_equality_compares_offset_number_and_heading() {
        let mut e = Engine::new();
        let a = mark(&mut e, [1.0, -0.0, 3.0], 5, 2);
        let b = mark(&mut e, [1.0, 0.0, 3.0], 5, 2);
        assert!(e.call(0x00c5_4cc0, &args![a, b]).bool());
        for (offset, heading, number) in [
            ([1.0, 0.0, 4.0], 5u16, 2u8),
            ([1.0, 0.0, 3.0], 6, 2),
            ([1.0, 0.0, 3.0], 5, 3),
            ([f32::NAN, 0.0, 3.0], 5, 2),
        ] {
            let c = mark(&mut e, offset, heading, number);
            assert!(!e.call(0x00c5_4cc0, &args![a, c]).bool());
        }
        let nan = mark(&mut e, [f32::NAN, 0.0, 3.0], 5, 2);
        assert!(!e.call(0x00c5_4cc0, &args![nan, nan]).bool());
    }

    #[test]
    fn mark_allocation_initializes_headings_and_numbers() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let block = e.call(0x00c5_4bc0, &args![3u32]).u32();
        assert_eq!(
            log_of(&mut e, 0x00c5_4bc0),
            vec![(ALLOCATE, vec![MEMORY_MANAGER, 0x30])]
        );
        for i in 0..3 {
            assert_eq!(e.mem.u16(block + 0x10 * i + 0xc), 0);
            assert_eq!(e.mem.u8(block + 0x10 * i + 0xe), 0xff);
        }
        // An overflowing size asks for 0xffffffff bytes; a failed allocation gives 0.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_4bc0, &args![0x1000_0000u32]).u32(), 0);
        assert_eq!(
            log_of(&mut e, 0x00c5_4bc0),
            vec![(ALLOCATE, vec![MEMORY_MANAGER, 0xffff_ffff])]
        );
    }

    #[test]
    fn array_constructor_allocates_unless_empty() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00c5_4c50, &args![this, 2u32, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_TARRAY);
        assert_ne!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(e.mem.u32(this + 4) + 0x1e), 0xff);
        assert_eq!((e.mem.u16(this + 8), e.mem.u16(this + 0xe)), (2, 1));
        e.call(0x00c5_4c50, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn set_size_grows_and_shrinks() {
        let mut e = engine();
        let a = mark(&mut e, [1.0, 2.0, 3.0], 7, 1);
        let b = mark(&mut e, [4.0, 5.0, 6.0], 8, 2);
        let this = array(&mut e, &[a, b], 2);
        let old = e.mem.u32(this + 4);
        // Same capacity: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c5_4850, &args![this, 2u32]);
        assert!(log_of(&mut e, 0x00c5_4850).is_empty());
        // Grow to 4: the marks move, the new slots are default marks.
        e.call_log = Some(vec![]);
        e.call(0x00c5_4850, &args![this, 4u32]);
        let log = log_of(&mut e, 0x00c5_4850);
        assert_eq!(
            log.last().unwrap(),
            &(DEALLOCATE, vec![MEMORY_MANAGER, old])
        );
        let data = e.mem.u32(this + 4);
        assert_ne!(data, old);
        assert_eq!(e.mem.u16(this + 8), 4);
        assert!(furniture_mark_operator_eq(
            &mut e,
            Ptr::new(data),
            Ptr::new(a)
        ));
        assert!(furniture_mark_operator_eq(
            &mut e,
            Ptr::new(data + 0x10),
            Ptr::new(b)
        ));
        assert_eq!(e.mem.u16(data + 0x2c), 0);
        assert_eq!(e.mem.u8(data + 0x3e), 0);
        // Shrink to 1: the dropped non-empty mark counts one less.
        e.call(0x00c5_4850, &args![this, 1u32]);
        assert_eq!(
            (
                e.mem.u16(this + 8),
                e.mem.u16(this + 0xa),
                e.mem.u16(this + 0xc)
            ),
            (1, 1, 1)
        );
        // To 0: no block.
        e.call(0x00c5_4850, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn set_at_counts_non_empty_elements() {
        let mut e = engine();
        e.mem.set_u32(EMPTY_MARK_GUARD, 0);
        e.mem.set_u16(EMPTY_MARK + 0xc, 0x5555);
        let full = mark(&mut e, [1.0, 2.0, 3.0], 7, 1);
        let empty = mark(&mut e, [0.0, 0.0, 0.0], 0, 0);
        let this = array(&mut e, &[], 4);
        e.call(0x00c5_4a30, &args![this, 1u32, full]);
        // The static empty mark was initialized on first use.
        assert_eq!(e.mem.u32(EMPTY_MARK_GUARD), 1);
        assert_eq!(e.mem.u16(EMPTY_MARK + 0xc), 0);
        assert_eq!((e.mem.u16(this + 0xa), e.mem.u16(this + 0xc)), (2, 1));
        let data = e.mem.u32(this + 4);
        assert!(furniture_mark_operator_eq(
            &mut e,
            Ptr::new(data + 0x10),
            Ptr::new(full)
        ));
        // Below the size: full over empty +1, full over full 0, empty over full -1.
        e.call(0x00c5_4a30, &args![this, 0u32, full]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        e.call(0x00c5_4a30, &args![this, 0u32, full]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        e.call(0x00c5_4a30, &args![this, 1u32, empty]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        // Past the size with an empty mark: the size grows, the count does not.
        e.call(0x00c5_4a30, &args![this, 3u32, empty]);
        assert_eq!((e.mem.u16(this + 0xa), e.mem.u16(this + 0xc)), (4, 1));
    }

    #[test]
    fn add_grows_by_the_growth_when_full() {
        let mut e = engine();
        let a = mark(&mut e, [1.0, 2.0, 3.0], 7, 1);
        let this = array(&mut e, &[a], 1);
        assert_eq!(e.call(0x00c5_4b70, &args![this, a]).u32(), 1);
        assert_eq!(e.mem.u16(this + 8), 3);
        assert_eq!((e.mem.u16(this + 0xa), e.mem.u16(this + 0xc)), (2, 2));
        assert_eq!(e.call(0x00c5_4b70, &args![this, a]).u32(), 2);
        assert_eq!(e.mem.u16(this + 8), 3);
    }

    #[test]
    fn mark_load_binary_reads_its_fields_and_converts_a_negative_heading() {
        for (stored, converted) in [(1500i16, false), (-1500, true)] {
            let mut e = Engine::new();
            e.map(HEADING_SCALE, 8);
            e.set_global::<f64>(HEADING_SCALE, 1000.0);
            e.map(ZERO, 8);
            e.map(FULL_CIRCLE, 4);
            e.set_global::<f32>(FULL_CIRCLE, std::f32::consts::TAU);
            e.register(THREE_FLOATS_LOAD_BINARY, |_, _| Ret::default());
            e.register_double(LOAD_SHORT, move |e, a| {
                e.mem.set_u16(a[1], stored as u16);
                Ret::default()
            });
            e.register(LOAD_BYTE, |e, a| {
                e.mem.set_u8(a[1], 9);
                Ret::default()
            });
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            e.call(0x00c5_45d0, &args![this, 0x900u32]);
            let log = log_of(&mut e, 0x00c5_45d0);
            assert_eq!(log[0], (THREE_FLOATS_LOAD_BINARY, vec![this, 0x900]));
            assert_eq!(log[1], (LOAD_SHORT, vec![0x900, this + 0xc]));
            assert_eq!(log[2], (LOAD_BYTE, vec![0x900, this + 0xe]));
            assert_eq!(log[3].0, LOAD_BYTE);
            assert_ne!(log[3].1[1], this + 0xe);
            assert_eq!(e.mem.u8(this + 0xe), 9);
            let heading = e.mem.u16(this + 0xc);
            if converted {
                // -1.5 radians wrapped by the full circle, back in thousandths.
                let wrapped = (f64::from(-1.5f32) + f64::from(std::f32::consts::TAU)) as f32;
                assert_eq!(heading, (f64::from(wrapped) * 1000.0) as i32 as u16);
            } else {
                assert_eq!(heading, 1500);
            }
        }
    }

    #[test]
    fn marker_constructor_destructor_and_factory() {
        let mut e = engine();
        e.map(FURNITURE_MARKER_NAME, 4);
        e.mem.set_u32(FURNITURE_MARKER_NAME, 0x4321);
        e.register(EXTRA_DATA_DEFAULT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_3f10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_FURNITURE_MARKER);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_OBJECT_ARRAY);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!((e.mem.u16(this + 0x14), e.mem.u16(this + 0x1a)), (0, 1));
        let log = log_of(&mut e, 0x00c5_3f10);
        assert_eq!(log[0], (EXTRA_DATA_DEFAULT_CONSTRUCTOR, vec![this]));
        assert_eq!(log.last().unwrap(), &(SET_NAME, vec![this, 0x4321]));
        e.mem.set_u32(this + 0x10, 0x7700);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_47e0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            log_of(&mut e, 0x00c5_47e0),
            vec![
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x7700]),
                (EXTRA_DATA_DESTRUCTOR, vec![this]),
                (NI_DELETE, vec![this, 0x1c]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_TARRAY);
        let block = e.mem.alloc(0x1c);
        e.register_double(NI_NEW, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_4210, &args![]).u32(), block);
        let log = log_of(&mut e, 0x00c5_4210);
        assert_eq!(log.last().unwrap(), &(SET_NAME, vec![block, CREATED_NAME]));
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_4210, &args![]).u32(), 0);
        assert_eq!(
            log_of(&mut e, 0x00c5_4210),
            vec![(NI_NEW, vec![0x1c]), (SET_NAME, vec![0, CREATED_NAME])]
        );
    }

    #[test]
    fn marker_load_binary_reads_the_marks_into_the_array() {
        for count in [0u32, 2] {
            let mut e = engine();
            e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
            e.register_double(LOAD_COUNT, move |e, a| {
                e.mem.set_u32(a[1], count);
                Ret::default()
            });
            e.register(THREE_FLOATS_LOAD_BINARY, |e, a| {
                e.mem.set_u32(a[0], 2.0f32.to_bits());
                Ret::default()
            });
            e.register(LOAD_SHORT, |e, a| {
                e.mem.set_u16(a[1], 300);
                Ret::default()
            });
            e.register(LOAD_BYTE, |e, a| {
                e.mem.set_u8(a[1], 4);
                Ret::default()
            });
            let this = e.mem.alloc(0x1c);
            e.mem.set_u16(this + 0x1a, 1);
            e.call(0x00c5_40a0, &args![this, 0x900u32]);
            let array = this + 0xc;
            assert_eq!(e.mem.u16(array + 0xa), count as u16);
            if count == 0 {
                assert_eq!(e.mem.u32(array + 4), 0);
                continue;
            }
            assert_eq!(e.mem.u16(array + 8), 2);
            assert_eq!(e.mem.u16(array + 0xc), 2);
            let data = e.mem.u32(array + 4);
            for i in 0..2 {
                assert_eq!(e.mem.u32(data + 0x10 * i), 2.0f32.to_bits());
                assert_eq!(e.mem.u16(data + 0x10 * i + 0xc), 300);
                assert_eq!(e.mem.u8(data + 0x10 * i + 0xe), 4);
            }
        }
    }

    #[test]
    fn marker_find_and_name_globals() {
        let mut e = engine();
        e.map(FURNITURE_MARKER_NAME, 4);
        e.mem.set_u32(FURNITURE_MARKER_NAME, 0x4321);
        let data = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x00aa_0009);
        e.mem.set_u32(data, vtable);
        let other = e.mem.alloc(8);
        for (rtti, found) in [(FURNITURE_MARKER_RTTI, true), (other, false)] {
            e.register_double(0x00aa_0009, move |_, _| rtti.into_ret());
            e.register_double(GET_EXTRA_DATA, move |_, a| {
                assert_eq!(a[1], 0x4321);
                data.into_ret()
            });
            let want = if found { data } else { 0 };
            assert_eq!(e.call(0x00c5_4400, &args![0x900u32]).u32(), want);
        }
        assert_eq!(e.call(0x00c5_4400, &args![0u32]).u32(), 0);
        // The name initializer and destructor.
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
        e.register(FIXED_STRING_CREATE, |_, _| 0x8008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call(0x00c5_46c0, &args![]);
        let string = e.mem.u32(FURNITURE_MARKER_NAME);
        assert_eq!(e.mem.u32(string), 0x8008);
        e.call_log = Some(vec![]);
        e.call(0x00c5_4760, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_4760),
            vec![
                (INTERLOCKED_DECREMENT, vec![0x8000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, string]),
            ]
        );
        assert_eq!(e.mem.u32(FURNITURE_MARKER_NAME), 0);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A marker whose mark array holds `numbers` (mark i at offset (i, 0, 0),
    /// heading 1500 + i), with room for eight.
    fn marker(e: &mut Engine, numbers: &[u8]) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8 * 16);
        for (i, number) in numbers.iter().enumerate() {
            let mark = data + 16 * i as u32;
            e.mem.set_f32(mark, i as f32);
            e.mem.set_u16(mark + 0xc, 1500 + i as u16);
            e.mem.set_u8(mark + 0xe, *number);
        }
        e.mem.set_u32(this + MARK_DATA, data);
        e.mem.set_u16(this + MARKS + 8, 8);
        e.mem.set_u16(this + MARKS + 0xa, numbers.len() as u16);
        e.mem.set_u16(this + MARK_COUNT, numbers.len() as u16);
        this
    }

    #[test]
    fn ni_stream_save_binary_unsigned_short_writes_two_bytes() {
        const WRITE: u32 = 0x0100_0040;
        let mut e = Engine::new();
        e.register(WRITE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 2);
            word(a[2])
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 4, 0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.call_log = Some(vec![]);
        e.call(0x00a3_03a0, &args![stream, 0x5000u32]);
        assert_eq!(calls_to(&e, WRITE)[0][..3], [output, 0x5000, 2]);
        assert_eq!(e.mem.u32(output + 4), 0x12);
    }

    #[test]
    fn save_binary_writes_the_count_and_every_mark() {
        const WRITE: u32 = 0x0100_0040;
        let mut e = Engine::new();
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_COUNT, |e, a| word(e.mem.u32(a[1])));
        e.register(THREE_FLOATS_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_BYTE, |_, _| Ret::default());
        e.register(WRITE, |_, a| word(a[2]));
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = marker(&mut e, &[4, 9]);
        let data = e.mem.u32(this + MARK_DATA);
        e.call_log = Some(vec![]);
        e.call(0x00c5_4170, &args![this, stream]);
        let counts: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == SAVE_COUNT)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(counts, [stream]);
        assert_eq!(
            calls_to(&e, THREE_FLOATS_SAVE_BINARY),
            vec![vec![data, stream], vec![data + 16, stream]]
        );
        assert_eq!(
            calls_to(&e, SAVE_BYTE),
            vec![
                vec![stream, data + 0xe],
                vec![stream, data + 0xe],
                vec![stream, data + 0x1e],
                vec![stream, data + 0x1e]
            ]
        );
        assert_eq!(e.mem.u32(output + 4), 4);
    }

    #[test]
    fn is_equal_passes_on_the_extra_data_answer() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_IS_EQUAL, |_, a| word(a[1] & 0xff));
        assert_eq!(e.call(0x00c5_41e0, &args![0x1000u32, 0x2002u32]).u32(), 2);
    }

    #[test]
    fn get_viewer_strings_lists_every_mark() {
        let mut e = Engine::new();
        e.map(FURNITURE_MARKER_RTTI, 4);
        e.set_global(FURNITURE_MARKER_RTTI, 0x0555_0000u32);
        e.map(HEADING_SCALE, 8);
        e.set_global(HEADING_SCALE, 1000.0f64);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(NI_GET_VIEWER_STRING_INT, |_, a| word(0x6000_0000 | a[1]));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| word(a[1]));
        e.register(POINT_GET_VIEWER_STRING, |_, a| word(a[0]));
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(STRINGS_ADD, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let this = marker(&mut e, &[4]);
        let data = e.mem.u32(this + MARK_DATA);
        e.call(0x00c5_42a0, &args![this, 0x2000u32]);
        assert_eq!(
            *added.borrow(),
            vec![
                0x7000_0001,
                0x6000_0001,
                0x6000_0000,
                0x6000_0004,
                data,
                1.5f32.to_bits()
            ]
        );
    }

    #[test]
    fn fn_00c54490_inserts_by_number() {
        let mut e = Engine::new();
        e.map(EMPTY_MARK, 0x14);
        let this = marker(&mut e, &[1, 5, 9]);
        let mark = e.mem.alloc(16);
        e.mem.set_f32(mark, 42.0);
        e.mem.set_u8(mark + 0xe, 5);
        e.call(0x00c5_4490, &args![this, mark]);
        let data = e.mem.u32(this + MARK_DATA);
        let numbers: Vec<u8> = (0..4).map(|i| e.mem.u8(data + 16 * i + 0xe)).collect();
        assert_eq!(numbers, [1, 5, 5, 9]);
        assert_eq!(e.mem.f32(data + 32), 42.0);
        assert_eq!(e.mem.f32(data + 48), 2.0);
        // A number above every mark's is only appended.
        e.mem.set_u8(mark + 0xe, 200);
        e.call(0x00c5_4490, &args![this, mark]);
        assert_eq!(e.mem.u8(data + 16 * 4 + 0xe), 200);
    }

    #[test]
    fn array_destructors_free_the_block_and_maybe_the_array() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x00c5_4810, &args![array]);
        assert_eq!(e.mem.u32(array), VTABLE_TARRAY);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x7777]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_4d70, &args![array, 1u32]).u32(), array);
        assert_eq!(
            calls_to(&e, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7777], vec![MEMORY_MANAGER, array]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c5_4d70, &args![array, 0u32]);
        assert_eq!(calls_to(&e, DEALLOCATE).len(), 1);
    }
}
