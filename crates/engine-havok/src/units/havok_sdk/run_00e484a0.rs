//! `Havok SDK/run_00e484a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{load_vec, maxps, minps, mulps, store_vec};

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`): cdecl, `(allocator, array, element size)`.
const RESERVE_MORE: u32 = 0x00c9_0940;
/// The base constructor (`00cbfba0`): thiscall, `this` only.
const BASE_CONSTRUCTOR: u32 = 0x00cb_fba0;
/// `00e5d040`: cdecl, three words, called by both constructors with an AABB (two vectors, the
/// minimum then the maximum).
const BUILD_FROM_AABB: u32 = 0x00e5_d040;
/// `00d2d4b0`: cdecl, `(node, 0x200 byte buffer)`, returns a pointer to an object.
const RESOLVE_NODE: u32 = 0x00d2_d4b0;
/// The transform `00e485d0` passes to its AABB queries (`01268370`).
const QUERY_TRANSFORM: u32 = 0x0126_8370;
/// The `float`s of the starting AABB `00e487a0` builds (`010ec1b0` for every lane of the
/// minimum, `010ea628` for the maximum).
const AABB_MINIMUM: u32 = 0x010e_c1b0;
const AABB_MAXIMUM: u32 = 0x010e_a628;
/// The vtable written to the object `00e48cb0` fills (`010c749c`).
const SHAPE_VTABLE: u32 = 0x010c_749c;

/// The value at `+0x1c` of the last node of the chain that starts at `node` (the chain follows
/// `+0xc` while it is not null).
fn chain_root_value(e: &Engine, node: u32) -> u32 {
    let mut root = node;
    loop {
        let parent = e.mem.u32(root.wrapping_add(0xc));
        if parent == 0 {
            break;
        }
        root = parent;
    }
    e.mem.u32(root.wrapping_add(0x1c))
}

// Translated from 00e484a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: for each of the `[+0x10]` words of the array at `*(this + 0xc)`: the word's
/// high 24 bits are an offset into the byte block at `*(other + 0x20)`; the byte there moves
/// into the low byte of the word and is cleared in the block.
pub fn fn_00e484a0(e: &mut Engine, this: Ptr, other: u32) {
    let this = this.addr();
    let mut index = 0i32;
    if e.mem.i32(this.wrapping_add(0x10)) <= 0 {
        return;
    }
    loop {
        let words = e.mem.u32(this.wrapping_add(0xc));
        let slot = words.wrapping_add((index as u32).wrapping_mul(4));
        let word = e.mem.u32(slot);
        let byte_address = (word >> 8).wrapping_add(e.mem.u32(other.wrapping_add(0x20)));
        index += 1;
        let byte = e.mem.u8(byte_address);
        e.mem.set_u8(slot, 0);
        let merged = e.mem.u32(slot) | u32::from(byte);
        e.mem.set_u32(slot, merged);
        e.mem.set_u8(byte_address, 0);
        if index >= e.mem.i32(this.wrapping_add(0x10)) {
            break;
        }
    }
}

// Translated from 00e48510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: writes to `out` (a byte) whether `item` is one of the pointers of the
/// `hkArray` at `*(this + 0x10)` (`data`, `size`). The third stack word is not read.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e48510(e: &mut Engine, this: Ptr, out: u32, item: u32, _unused_3: u32) -> u32 {
    let array = e.mem.u32(this.addr().wrapping_add(0x10));
    let size = e.mem.i32(array.wrapping_add(4));
    let mut found = false;
    if size > 0 {
        let data = e.mem.u32(array);
        for index in 0..size as u32 {
            if e.mem.u32(data.wrapping_add(index.wrapping_mul(4))) == item {
                found = true;
                break;
            }
        }
    }
    e.mem.set_u8(out, u8::from(found));
    out
}

// Translated from 00e48550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: writes to `out` (a byte) 1 minus bit `index` (counted from the start,
/// 32 per word, `index` signed) of the bit array whose data pointer is at `**(this + 0x14)`.
/// The third stack word is not read.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e48550(e: &mut Engine, this: Ptr, out: u32, index: i32, _unused_3: u32) -> u32 {
    let holder = e.mem.u32(this.addr().wrapping_add(0x14));
    let data = e.mem.u32(holder);
    let word_address = data.wrapping_add((index >> 5).wrapping_mul(4) as u32);
    let word = e.mem.u32(word_address);
    let bit = word.wrapping_shr(index as u32);
    e.mem.set_u8(out, !(bit as u8) & 1);
    out
}

/// Makes room for one more element of `element_size` bytes in the `hkArray` at `array` when it is
/// full (size equal to the capacity bits), through `hkArrayUtil::_reserveMore` (`00c90940`).
fn grow_if_full(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let capacity = e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff;
    if e.mem.u32(array.wrapping_add(4)) == capacity {
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        e.call(RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

// Translated from 00e48580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: appends `value << 8` to the word array at `this + 4` (`data`, `size`,
/// `capacity | flags`), growing it through `00c90940` when it is full.
pub fn fn_00e48580(e: &mut Engine, this: Ptr, value: u32) {
    let array = this.addr().wrapping_add(4);
    grow_if_full(e, array, 4);
    let size = e.mem.u32(array.wrapping_add(4));
    let data = e.mem.u32(array);
    let slot = data.wrapping_add(size.wrapping_mul(4));
    e.mem.set_u32(array.wrapping_add(4), size.wrapping_add(1));
    e.mem.set_u8(slot, 0);
    let low = u32::from(e.mem.u8(slot));
    e.mem.set_u32(slot, low | (value << 8));
}

/// Writes the second form `0x10e88b4` / `0x10e88a4` vtables used while the object is built or
/// destroyed.
fn set_working_vtables(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010e_88b4);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_88a4);
}

// Translated from 00e485d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 0xc`. Stores `1` at `+6`, the vtables (`010e88b4` at `+0` and, after
/// `010e8894`, `010e88a4` at `+8`) and an empty unowned `hkArray` at `+0xc`. When the pointer
/// array `list` (`data`, `size`) is not empty, the AABB of its objects is found: for each object
/// `resolver` (virtual slot `0x14`, with the object and a 0x200-byte stack buffer) gives a
/// second object whose virtual slot `0x1c` is asked, with the transform `01268370`, the `float`
/// 0 and an output pair, for the object's minimum and maximum; they are merged with `MINPS` and
/// `MAXPS`. Then `+0x18` is set to `list`, `00e5d040` runs with `source`, the AABB and
/// `this + 8`, and `+0x18` is cleared. Returns `this`.
pub fn fn_00e485d0(e: &mut Engine, this: Ptr, source: u32, resolver: u32, list: u32) -> u32 {
    let this = this.addr();
    e.mem.set_u16(this.wrapping_add(6), 1);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_8894);
    e.mem.set_u32(this, 0x010e_88b4);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_88a4);
    e.mem.set_u32(this.wrapping_add(0xc), 0);
    e.mem.set_u32(this.wrapping_add(0x10), 0);
    e.mem.set_u32(this.wrapping_add(0x14), 0x8000_0000);
    if e.mem.u32(list.wrapping_add(4)) == 0 {
        return this;
    }
    e.with_stack(0x250, |e, frame| {
        let frame = frame.addr();
        let (accumulated, item_box, buffer) = (
            frame.wrapping_add(0x10),
            frame.wrapping_add(0x30),
            frame.wrapping_add(0x50),
        );
        let data = e.mem.u32(list);
        let first = e.mem.u32(data);
        let query = e.vcall(resolver, 0x14, &args![first, buffer]).u32();
        e.vcall(query, 0x1c, &args![QUERY_TRANSFORM, 0.0f32, accumulated]);
        let mut index = 1i32;
        while e.mem.i32(list.wrapping_add(4)) > index {
            let data = e.mem.u32(list);
            let item = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let query = e.vcall(resolver, 0x14, &args![item, buffer]).u32();
            e.vcall(query, 0x1c, &args![QUERY_TRANSFORM, 0.0f32, item_box]);
            let minimum = load_vec(e, item_box);
            let merged = minps(load_vec(e, accumulated), minimum);
            store_vec(e, accumulated, merged);
            let maximum = load_vec(e, item_box.wrapping_add(0x10));
            let merged = maxps(load_vec(e, accumulated.wrapping_add(0x10)), maximum);
            index += 1;
            store_vec(e, accumulated.wrapping_add(0x10), merged);
        }
        e.mem.set_u32(this.wrapping_add(0x18), list);
        e.call(
            BUILD_FROM_AABB,
            &args![source, accumulated, this.wrapping_add(8)],
        );
        e.mem.set_u32(this.wrapping_add(0x18), 0);
    });
    this
}

// Translated from 00e48740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall destructor: stores the working vtables (`010e88b4` at `+0`, `010e88a4` at `+8`),
/// sets the size at `+0x10` to 0, frees the block at `+0xc` through the allocator (virtual slot
/// `0x10`, with the pointer and `4 * capacity`) unless the array is unowned (sign bit of
/// `+0x14`), empties the array (`0x80000000` at `+0x14`, 0 at `+0xc`) and stores the final
/// vtables (`010e8894` at `+8`, `0102e388` at `+0`).
pub fn fn_00e48740(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    set_working_vtables(e, this);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let flags = e.mem.u32(this.wrapping_add(0x14));
    e.mem.set_u32(this.wrapping_add(0x10), 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(this.wrapping_add(0xc));
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(this.wrapping_add(0xc), 0);
    e.mem.set_u32(this.wrapping_add(0x14), 0x8000_0000);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_8894);
    e.mem.set_u32(this, 0x0102_e388);
}

// Translated from 00e487a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 8`: stores `1` at `+6`, an empty unowned `hkArray` at `+0xc`, the
/// vtables (`010e8894`, then `010ec1b8` at `+8`; `010ec1a4` at `+0`), 0 at `+0x18` and `value`
/// at `+0x1c`, calls `00e5d040` with `source`, an AABB whose minimum has every lane equal to
/// the `float` at `010ec1b0` and whose maximum has the `float` at `010ea628` in every lane, and
/// `this + 8`, then clears `+0x1c`. Returns `this`.
pub fn fn_00e487a0(e: &mut Engine, this: Ptr, source: u32, value: u32) -> u32 {
    let this = this.addr();
    let maximum = f32::from_bits(e.mem.u32(AABB_MAXIMUM));
    e.mem.set_u16(this.wrapping_add(6), 1);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_8894);
    e.mem.set_u32(this.wrapping_add(0xc), 0);
    e.mem.set_u32(this.wrapping_add(0x10), 0);
    e.mem.set_u32(this.wrapping_add(0x14), 0x8000_0000);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_c1b8);
    let minimum = f32::from_bits(e.mem.u32(AABB_MINIMUM));
    e.mem.set_u32(this.wrapping_add(0x18), 0);
    e.mem.set_u32(this, 0x010e_c1a4);
    e.mem.set_u32(this.wrapping_add(0x1c), value);
    e.with_stack(0x20, |e, aabb| {
        let aabb = aabb.addr();
        store_vec(e, aabb, [minimum; 4]);
        store_vec(e, aabb.wrapping_add(0x10), [maximum; 4]);
        e.call(BUILD_FROM_AABB, &args![source, aabb, this.wrapping_add(8)]);
    });
    e.mem.set_u32(this.wrapping_add(0x1c), 0);
    this
}

// Translated from 00e48830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: a collision-group filter test; writes a byte to `out`. When `first` and
/// `second` differ in their high 16 bits, or `first` has none: whether bit `second & 0x1f` is
/// set in the table word `*(this + 0x34 + 4 * (first & 0x1f))`. Otherwise (same nonzero high
/// half) 1 when both `((second >> 5) ^ first) & 0x3e0` and `((first >> 5) ^ second) & 0x3e0`
/// are nonzero, else 0.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e48830(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let this = this.addr();
    if (first ^ second) & 0xffff_0000 != 0 || first & 0xffff_0000 == 0 {
        let word = e.mem.u32(this.wrapping_add(0x34 + 4 * (first & 0x1f)));
        e.mem
            .set_u8(out, u8::from(word & (1 << (second & 0x1f)) != 0));
        return out;
    }
    let hit = (((second >> 5) ^ first) & 0x3e0 != 0) && (((first >> 5) ^ second) & 0x3e0 != 0);
    e.mem.set_u8(out, u8::from(hit));
    out
}

// Translated from 00e488a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x14` (an adjuster thunk for the interface at `+0x10`): the first value is the
/// word at `+0x20` of `holder`, the second comes from `object` (virtual slot `0x10`, one stack
/// word `argument`); `00e48830` runs on `this - 0x10`. The third stack word is not read.
/// Returns `out`.
pub fn fn_00e488a0(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    holder: u32,
    _unused_3: u32,
    object: u32,
    argument: u32,
) -> u32 {
    let second = e.vcall(object, 0x10, &args![argument]).u32();
    let first = e.mem.u32(holder.wrapping_add(0x20));
    fn_00e48830(
        e,
        Ptr::new(this.addr().wrapping_sub(0x10)),
        out,
        first,
        second,
    );
    out
}

/// Stores `0xffffffff` in the 32 words of the table from `+0x34` to `+0xb0`.
fn fill_table_with_ones(e: &mut Engine, this: u32) {
    for word in 0..32u32 {
        e.mem.set_u32(this.wrapping_add(0x34 + 4 * word), u32::MAX);
    }
}

// Translated from 00e489b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall constructor: runs the base constructor (`00cbfba0`), stores the vtables (`010e8a6c`
/// at `+0`, `010e8a60`, `010e8a4c`, `010e8a40`, `010e8a34` at `+8`, `+0xc`, `+0x10`, `+0x14`),
/// 2 at `+0x20`, fills the 32 words from `+0x34` to `+0xb0` with `0xffffffff` and stores 0 at
/// `+0x30`. Returns `this`.
pub fn fn_00e489b0(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this, 0x010e_8a6c);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_8a60);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010e_8a4c);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010e_8a40);
    e.mem.set_u32(this.wrapping_add(0x14), 0x010e_8a34);
    e.mem.set_u32(this.wrapping_add(0x20), 2);
    fill_table_with_ones(e, this);
    e.mem.set_u32(this.wrapping_add(0x30), 0);
    this
}

// Translated from 00e48a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall: stores the vtables `010c3b8c` at `+0x14`, `010c3bac` at `+0x10`, `010c3b98` at
/// `+0xc`, `010c3b80` at `+8` and `0102e388` at `+0`.
pub fn fn_00e48a80(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this.wrapping_add(0x14), 0x010c_3b8c);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010c_3bac);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010c_3b98);
    e.mem.set_u32(this.wrapping_add(8), 0x010c_3b80);
    e.mem.set_u32(this, 0x0102_e388);
}

// Translated from 00e48ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (an adjuster thunk for the interface at `+8`): `00e48830` on `this - 8`
/// with the words at `+0x1c` of `first` and `second`. Returns `out`.
pub fn fn_00e48ab0(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let first_value = e.mem.u32(first.wrapping_add(0x1c));
    let second_value = e.mem.u32(second.wrapping_add(0x1c));
    fn_00e48830(
        e,
        Ptr::new(this.addr().wrapping_sub(8)),
        out,
        first_value,
        second_value,
    );
    out
}

// Translated from 00e48ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x20` (an adjuster thunk for the interface at `+0xc`): `first_object` and
/// `second_object` are asked (virtual slot `0x10`, with `first_argument` and `second_argument`)
/// for two values; an answer of `-1` is replaced by the word at `+0x1c` of the root of the chain
/// of `first_node` (resp. `second_node`, following `+0xc`). `00e48830` runs on `this - 0xc` with
/// them. The second stack word is not read. Returns `out`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e48ae0(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    _unused_2: u32,
    first_node: u32,
    second_node: u32,
    first_object: u32,
    second_object: u32,
    first_argument: u32,
    second_argument: u32,
) -> u32 {
    let mut first = e.vcall(first_object, 0x10, &args![first_argument]).u32();
    if first == u32::MAX {
        first = chain_root_value(e, first_node);
    }
    let mut second = e.vcall(second_object, 0x10, &args![second_argument]).u32();
    if second == u32::MAX {
        second = chain_root_value(e, second_node);
    }
    fn_00e48830(
        e,
        Ptr::new(this.addr().wrapping_sub(0xc)),
        out,
        first,
        second,
    );
    out
}

// Translated from 00e48b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (an adjuster thunk for the interface at `+0x14`): `00e48830` on
/// `this - 0x14` with the word at `+0x24` of `first` and the word at `+0x1c` of `second`.
/// Returns `out`.
pub fn fn_00e48b60(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let first_value = e.mem.u32(first.wrapping_add(0x24));
    let second_value = e.mem.u32(second.wrapping_add(0x1c));
    fn_00e48830(
        e,
        Ptr::new(this.addr().wrapping_sub(0x14)),
        out,
        first_value,
        second_value,
    );
    out
}

/// What `00e48b90` finds in the chain: the first value for `00e48830`, or the early answer 1.
enum ChainResult {
    Value(u32),
    Accept,
}

/// The walk of `00e48b90` along the chain of nodes below `start`: `table_owner` points at an
/// object whose first word is the base of the flags table (`+0x110 + 4 * kind`), `start` is the
/// first node and `buffer` is the 0x200-byte stack buffer.
fn walk_chain(e: &mut Engine, table_owner: u32, start: u32, buffer: u32) -> ChainResult {
    let mut link = e.mem.u32(start.wrapping_add(0xc));
    if link == 0 {
        return ChainResult::Value(e.mem.u32(start.wrapping_add(0x1c)));
    }
    let table = e.mem.u32(table_owner);
    let mut previous = start;
    loop {
        let object = e.mem.u32(link);
        let kind = e.mem.u32(object.wrapping_add(0xc));
        let flags = e
            .mem
            .u32(table.wrapping_add(kind.wrapping_mul(4)).wrapping_add(0x110));
        if flags as u8 & 0x80 != 0 {
            let object = e.mem.u32(link);
            let argument = e.mem.u32(previous.wrapping_add(4));
            let value = e
                .vcall(object.wrapping_add(0x10), 0x10, &args![argument])
                .u32();
            return ChainResult::Value(value);
        }
        if flags & 0x100 != 0 {
            let resolved = e.call(RESOLVE_NODE, &args![link, buffer]).u32();
            let argument = e.mem.u32(previous.wrapping_add(4));
            let value = e.vcall(resolved, 0x10, &args![argument]).u32();
            return ChainResult::Value(value);
        }
        if flags & 0x20_0000 != 0 {
            return ChainResult::Value(chain_root_value(e, start));
        }
        if flags & 0x40_0000 != 0 {
            return ChainResult::Accept;
        }
        previous = link;
        link = e.mem.u32(link.wrapping_add(0xc));
        if link == 0 {
            return ChainResult::Value(e.mem.u32(previous.wrapping_add(0x1c)));
        }
    }
}

// Translated from 00e48b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x18` (an adjuster thunk for the interface at `+0xc`): the second value comes
/// from `object` (virtual slot `0x10`, one stack word `argument`; `-1` is replaced by the word
/// at `+0x1c` of the root of the chain of `fallback_node`). The first comes from walking the
/// chain of nodes from `start` (its `+0xc` links): a node whose object's kind (`*(*node +
/// 0xc)`) has flags `0x80` in the table `**table_owner + 0x110 + 4 * kind` asks that object's
/// sub-object at `+0x10`, flags `0x100` asks the object `00d2d4b0` makes of the node,
/// flags `0x200000` takes the root's word at `+0x1c` and flags `0x400000` writes 1 to `out`
/// and returns at once (virtual slot `0x10` with the previous node's word at `+4`); a node
/// without flags moves on, and the end of the chain gives the last node's word at `+0x1c`. A
/// start without a link gives its own word. `00e48830` then runs on `this - 0xc`. Returns `out`.
pub fn fn_00e48b90(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    table_owner: u32,
    start: u32,
    fallback_node: u32,
    object: u32,
    argument: u32,
) -> u32 {
    let mut second = e.vcall(object, 0x10, &args![argument]).u32();
    if second == u32::MAX {
        second = chain_root_value(e, fallback_node);
    }
    let first = e.with_stack(0x204, |e, frame| {
        walk_chain(e, table_owner, start, frame.addr().wrapping_add(4))
    });
    match first {
        ChainResult::Accept => {
            e.mem.set_u8(out, 1);
        }
        ChainResult::Value(first) => {
            fn_00e48830(
                e,
                Ptr::new(this.addr().wrapping_sub(0xc)),
                out,
                first,
                second,
            );
        }
    }
    out
}

// Translated from 00e48cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: builds a triangle shape for record `index` of the mesh at `*(this + 0x24)`
/// (vertex base at `+0`, vertex stride at `+4`, `u16` index records at `+0xc` with stride `+0x14`,
/// slot mask `+0x18`, user-data offset `+0x34`) into the object at `out` (not null in practice;
/// the exe then writes through address 0x20, which the memory model stops). The record's first
/// vertex and the two it names for `index & mask` and `(index & mask) ^ 1` are multiplied lane
/// by lane by the vector at `this + 0x10` and stored at `out + 0x20`, `0x30` and `0x40`. The
/// object gets the vtable `010c749c`, the radius `float` at `this + 0x40` (through the x87
/// stack), the user data word (0 when `[this + 0x34]` is null, else the `u16` at
/// `*(this + 0x30) + 2 * ([mesh + 0x34] + index)`) and the byte at `this + 0x3c`.
/// Answers `out` (`hkpShapeContainer::getChildShape` returns `const hkpShape*` in the Xbox PDB;
/// `EAX` is `out`, 0 through `XOR EAX,EAX` at 00e48d69 when it is null, at the RET 00e48d9f):
/// hkpShapeCollection::getAabbImpl reads the child through it (`MOV EDX,[EAX]` at 00d21657
/// after the container's slot +0x14 call at 00d21652).
pub fn fn_00e48cb0(e: &mut Engine, this: Ptr, index: u32, out: u32) -> u32 {
    let this = this.addr();
    let mesh = e.mem.u32(this.wrapping_add(0x24));
    let record_stride = e.mem.u32(mesh.wrapping_add(0x14));
    let mask = e.mem.u32(mesh.wrapping_add(0x18));
    let vertex_stride = e.mem.u32(mesh.wrapping_add(4));
    let record = record_stride
        .wrapping_mul(index)
        .wrapping_add(e.mem.u32(mesh.wrapping_add(0xc)));
    let slot = mask & index;
    let vertex_base = e.mem.u32(mesh);
    let first = u32::from(e.mem.u16(record))
        .wrapping_mul(vertex_stride)
        .wrapping_add(vertex_base);
    let second_word = e
        .mem
        .u16(record.wrapping_add(slot.wrapping_mul(2)).wrapping_add(2));
    let second = u32::from(second_word)
        .wrapping_mul(vertex_stride)
        .wrapping_add(vertex_base);
    let third_word = e.mem.u16(
        record
            .wrapping_add((slot ^ 1).wrapping_mul(2))
            .wrapping_add(2),
    );
    let third = u32::from(third_word)
        .wrapping_mul(e.mem.u32(mesh.wrapping_add(4)))
        .wrapping_add(e.mem.u32(mesh));
    let user_data = if e.mem.u32(this.wrapping_add(0x34)) == 0 {
        0
    } else {
        let at = e
            .mem
            .u32(mesh.wrapping_add(0x34))
            .wrapping_add(index)
            .wrapping_mul(2)
            .wrapping_add(e.mem.u32(this.wrapping_add(0x30)));
        e.mem.u16(at)
    };
    if out != 0 {
        let radius = x87_quiet(e.mem.f32(this.wrapping_add(0x40)));
        let flag = e.mem.u8(this.wrapping_add(0x3c));
        e.mem.set_u16(out.wrapping_add(6), 1);
        e.mem.set_f32(out.wrapping_add(0x10), radius);
        e.mem.set_u32(out.wrapping_add(8), 0);
        e.mem.set_u32(out.wrapping_add(0xc), 3);
        e.mem.set_u32(out, SHAPE_VTABLE);
        e.mem.set_u16(out.wrapping_add(0x14), user_data);
        e.mem.set_u8(out.wrapping_add(0x16), flag);
        store_vec(e, out.wrapping_add(0x50), [0.0; 4]);
        e.mem.set_u8(out.wrapping_add(0x17), 0);
    }
    let factor = load_vec(e, this.wrapping_add(0x10));
    let product = mulps(load_vec(e, first), factor);
    store_vec(e, out.wrapping_add(0x20), product);
    let product = mulps(load_vec(e, second), load_vec(e, this.wrapping_add(0x10)));
    store_vec(e, out.wrapping_add(0x30), product);
    let product = mulps(load_vec(e, third), load_vec(e, this.wrapping_add(0x10)));
    store_vec(e, out.wrapping_add(0x40), product);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_84a0, fn_00e484a0(Ptr, u32)),
        entry!(0x00e4_8510, fn_00e48510(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_8550, fn_00e48550(Ptr, u32, i32, u32) -> u32),
        entry!(0x00e4_8580, fn_00e48580(Ptr, u32)),
        entry!(0x00e4_85d0, fn_00e485d0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_8740, fn_00e48740(Ptr)),
        entry!(0x00e4_87a0, fn_00e487a0(Ptr, u32, u32) -> u32),
        entry!(0x00e4_8830, fn_00e48830(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_88a0, fn_00e488a0(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e4_89b0, fn_00e489b0(Ptr) -> u32),
        entry!(0x00e4_8a80, fn_00e48a80(Ptr)),
        entry!(0x00e4_8ab0, fn_00e48ab0(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x00e4_8ae0,
            fn_00e48ae0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e4_8b60, fn_00e48b60(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x00e4_8b90,
            fn_00e48b90(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e4_8cb0, fn_00e48cb0(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The argument words each double saw, tagged with the address.
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    /// What the AABB builder double saw: the words, the eight lanes and the word at `+0x10`.
    type BuildCalls = Rc<RefCell<Vec<(Vec<u32>, [f32; 8], u32)>>>;
    /// What the fixed-AABB double saw: the words, both vectors and the word at `+0x14`.
    type FixedCalls = Rc<RefCell<Vec<(Vec<u32>, Vec4, Vec4, u32)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn put_u32s(e: &mut Engine, address: u32, values: &[(u32, u32)]) {
        for (offset, value) in values {
            e.mem.set_u32(address + offset, *value);
        }
    }

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    #[test]
    fn bytes_move_into_the_low_byte_of_each_word() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x20);
        e.mem.set_u8(block + 2, 0x11);
        e.mem.set_u8(block + 5, 0x22);
        let words = e.mem.alloc(8);
        put_u32s(&mut e, words, &[(0, (2 << 8) | 0xaa), (4, (5 << 8) | 0xbb)]);
        put_u32s(&mut e, this, &[(0xc, words), (0x10, 2)]);
        e.mem.set_u32(other + 0x20, block);
        e.call(0x00e4_84a0, &args![this, other]);
        assert_eq!(e.mem.u32(words), (2 << 8) | 0x11);
        assert_eq!(e.mem.u32(words + 4), (5 << 8) | 0x22);
        assert_eq!(e.mem.u8(block + 2), 0);
        assert_eq!(e.mem.u8(block + 5), 0);
        // No words: nothing happens.
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_u8(block + 2, 9);
        e.call(0x00e4_84a0, &args![this, other]);
        assert_eq!(e.mem.u8(block + 2), 9);
    }

    #[test]
    fn the_group_filter_answers_its_out_pointer() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        let out = e.mem.alloc(4);
        // EAX is `out` at every RET (the hkBool result pointer the callers read through).
        assert_eq!(
            e.call(0x00e4_8830, &args![this, out, 1u32, 2u32]).u32(),
            out
        );
    }

    #[test]
    fn membership_and_bit_tests_write_bytes() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(8);
        let data = e.mem.alloc(16);
        for (index, value) in [5u32, 6, 7].iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *value);
        }
        put_u32s(&mut e, array, &[(0, data), (4, 3)]);
        e.mem.set_u32(this + 0x10, array);
        let out = e.mem.alloc(8);
        for (item, expected) in [(6u32, 1u8), (9, 0), (7, 1)] {
            e.call(0x00e4_8510, &args![this, out, item, 0u32]);
            assert_eq!(e.mem.u8(out), expected);
            assert_eq!(e.mem.u8(out + 1), 0);
        }
        e.mem.set_u32(array + 4, 0);
        e.call(0x00e4_8510, &args![this, out, 5u32, 0u32]);
        assert_eq!(e.mem.u8(out), 0);
        // Bit array: bits 0 and 2 of word 0 and bit 31 of word 1 are set.
        let holder = e.mem.alloc(8);
        let bits = e.mem.alloc(16);
        put_u32s(&mut e, bits, &[(0, 0b101), (4, 0x8000_0000)]);
        e.mem.set_u32(holder, bits);
        e.mem.set_u32(this + 0x14, holder);
        for (index, expected) in [(0i32, 0u8), (1, 1), (2, 0), (3, 1), (63, 0), (32, 1)] {
            e.call(0x00e4_8550, &args![this, out, index, 0u32]);
            assert_eq!(e.mem.u8(out), expected, "bit {index}");
        }
    }

    #[test]
    fn membership_and_bit_tests_answer_their_out_pointer() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(8);
        e.mem.set_u32(this + 0x10, array);
        let holder = e.mem.alloc(8);
        let bits = e.mem.alloc(16);
        e.mem.set_u32(holder, bits);
        e.mem.set_u32(this + 0x14, holder);
        let out = e.mem.alloc(8);
        // Both leave `out` in EAX at every RET (the hkBool result pointer callers read through).
        assert_eq!(
            e.call(0x00e4_8510, &args![this, out, 6u32, 0u32]).u32(),
            out
        );
        assert_eq!(
            e.call(0x00e4_8550, &args![this, out, 1i32, 0u32]).u32(),
            out
        );
    }

    #[test]
    fn appending_shifts_the_value_and_grows_a_full_array() {
        let mut e = engine();
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_0010, 0x4444);
        let grown: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&grown);
        e.register_double(RESERVE_MORE, move |e, a| {
            log.borrow_mut().push((RESERVE_MORE, a.to_vec()));
            let data = e.mem.alloc(64);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(16);
        put_u32s(&mut e, this + 4, &[(0, data), (4, 1), (8, 4)]);
        e.mem.set_u32(data, 0x1234);
        e.call(0x00e4_8580, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(data + 4), 0x7700);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert!(grown.borrow().is_empty());
        // Full: size equals the capacity bits (the flag bit is ignored).
        put_u32s(&mut e, this + 4, &[(4, 4), (8, 0x8000_0004)]);
        e.call(0x00e4_8580, &args![this, 0x5u32]);
        assert_eq!(
            *grown.borrow(),
            vec![(RESERVE_MORE, vec![0x4444, this + 4, 4])]
        );
        let new_data = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(new_data + 16), 0x500);
        assert_eq!(e.mem.u32(this + 8), 5);
    }

    /// A resolver whose slot `0x14` returns a query object and whose query object's slot `0x1c`
    /// writes the AABB `(item - 1 ..= item + 1)` for the item it was resolved from.
    fn resolver(e: &mut Engine) -> (u32, Calls) {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        let current = Rc::new(RefCell::new(0u32));
        let remember = Rc::clone(&current);
        e.put_vtable(0x0126_0000, &[0, 0, 0, 0, 0, 0x0ff0_0014]);
        e.put_vtable(0x0126_0100, &[0, 0, 0, 0, 0, 0, 0, 0x0ff0_001c]);
        let query = e.mem.alloc(8);
        e.mem.set_u32(query, 0x0126_0100);
        e.register_double(0x0ff0_0014, move |_, a| {
            *remember.borrow_mut() = a[1];
            log.borrow_mut().push((0x14, a.to_vec()));
            Ret {
                eax: query,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_001c, move |e, a| {
            log.borrow_mut().push((0x1c, a.to_vec()));
            let item = *current.borrow() as f32;
            store_vec(e, a[3], [item - 1.0; 4]);
            store_vec(e, a[3] + 0x10, [item + 1.0; 4]);
            Ret::default()
        });
        let resolver = e.mem.alloc(8);
        e.mem.set_u32(resolver, 0x0126_0000);
        (resolver, calls)
    }

    #[test]
    fn constructor_merges_the_boxes_of_the_listed_objects() {
        let mut e = engine();
        let (resolver, calls) = resolver(&mut e);
        let built: BuildCalls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&built);
        e.register_double(BUILD_FROM_AABB, move |e, a| {
            let mut lanes = [0.0; 8];
            for (lane, value) in lanes.iter_mut().enumerate() {
                *value = e.mem.f32(a[1] + 4 * lane as u32);
            }
            log.borrow_mut()
                .push((a.to_vec(), lanes, e.mem.u32(a[2] + 0x10)));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(16);
        for (index, item) in [10u32, 30, 20].iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *item);
        }
        let list = e.mem.alloc(8);
        put_u32s(&mut e, list, &[(0, data), (4, 3)]);
        let result = e
            .call(0x00e4_85d0, &args![this, 0x55u32, resolver, list])
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010e_88b4);
        assert_eq!(e.mem.u32(this + 8), 0x010e_88a4);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        let built = built.borrow();
        assert_eq!(built.len(), 1);
        assert_eq!(built[0].0[0], 0x55);
        assert_eq!(built[0].0[2], this + 8);
        assert_eq!(built[0].1, [9.0, 9.0, 9.0, 9.0, 31.0, 31.0, 31.0, 31.0]);
        assert_eq!(built[0].2, list);
        assert_eq!(calls.borrow().len(), 6);
        assert_eq!(calls.borrow()[1].1[1], QUERY_TRANSFORM);
        assert_eq!(calls.borrow()[1].1[2], 0.0f32.to_bits());
    }

    #[test]
    fn constructor_with_an_empty_list_only_initialises() {
        let mut e = engine();
        let calls = record(&mut e, &[BUILD_FROM_AABB]);
        let this = e.mem.alloc(0x40);
        let list = e.mem.alloc(8);
        e.call(0x00e4_85d0, &args![this, 0u32, 0u32, list]);
        assert!(calls.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 8), 0x010e_88a4);
    }

    #[test]
    fn destructor_frees_an_owned_block_only() {
        let mut e = engine();
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_0010, 0x0126_0100);
        e.mem.set_u32(0x0126_0100, 0x0126_0200);
        e.put_vtable(0x0126_0200, &[0, 0, 0, 0, 0x0ff0_0010]);
        let calls = record(&mut e, &[0x0ff0_0010]);
        let this = e.mem.alloc(0x20);
        put_u32s(&mut e, this, &[(0xc, 0x9000), (0x10, 3), (0x14, 6)]);
        e.call(0x00e4_8740, &args![this]);
        assert_eq!(
            *calls.borrow(),
            vec![(0x0ff0_0010, vec![0x0126_0100, 0x9000, 24])]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0x010e_8894);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        // An unowned array (sign bit set) is not freed.
        put_u32s(&mut e, this, &[(0xc, 0x9000), (0x14, 0x8000_0006)]);
        e.call(0x00e4_8740, &args![this]);
        assert_eq!(calls.borrow().len(), 1);
    }

    #[test]
    fn second_constructor_passes_the_fixed_aabb() {
        let mut e = engine();
        e.mem.set_f32(AABB_MINIMUM, -5.0);
        e.mem.set_f32(AABB_MAXIMUM, 5.0);
        let seen: FixedCalls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(BUILD_FROM_AABB, move |e, a| {
            log.borrow_mut().push((
                a.to_vec(),
                load_vec(e, a[1]),
                load_vec(e, a[1] + 0x10),
                e.mem.u32(a[2] + 0x14),
            ));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let result = e.call(0x00e4_87a0, &args![this, 0x66u32, 0x1234u32]).u32();
        assert_eq!(result, this);
        let seen = seen.borrow();
        assert_eq!(seen[0].0[0], 0x66);
        assert_eq!(seen[0].0[2], this + 8);
        assert_eq!(seen[0].1, [-5.0; 4]);
        assert_eq!(seen[0].2, [5.0; 4]);
        assert_eq!(seen[0].3, 0x1234);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this), 0x010e_c1a4);
        assert_eq!(e.mem.u32(this + 8), 0x010e_c1b8);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn second_constructor_keeps_the_value_while_building() {
        let mut e = engine();
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(BUILD_FROM_AABB, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[2] + 0x14));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.call(0x00e4_87a0, &args![this, 0u32, 0x1234u32]);
        assert_eq!(*seen.borrow(), vec![0x1234]);
    }

    fn filter_answer(e: &mut Engine, this: u32, first: u32, second: u32) -> u8 {
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0xee);
        e.call(0x00e4_8830, &args![this, out, first, second]);
        assert_eq!(e.mem.u8(out + 1), 0);
        e.mem.u8(out)
    }

    #[test]
    fn group_filter_covers_its_three_cases() {
        let mut e = engine();
        let this = e.mem.alloc(0xb8);
        // Table word 3 has bit 2 set.
        e.mem.set_u32(this + 0x34 + 12, 1 << 2);
        // Different high halves: the table decides.
        assert_eq!(filter_answer(&mut e, this, 0x0001_0003, 0x0002_0002), 1);
        assert_eq!(filter_answer(&mut e, this, 0x0001_0003, 0x0002_0001), 0);
        // First has no high half: the table decides too.
        assert_eq!(filter_answer(&mut e, this, 3, 0x0001_0002), 1);
        // Same nonzero high half.
        assert_eq!(filter_answer(&mut e, this, 0x0001_0020, 0x0001_0040), 1);
        assert_eq!(filter_answer(&mut e, this, 0x0001_0000, 0x0001_0020), 0);
        assert_eq!(filter_answer(&mut e, this, 0x0001_0020, 0x0001_0000), 0);
    }

    /// A filter object whose table word `a` has bit `b` set, answering for `(a, b)` only.
    fn selective_filter(e: &mut Engine, a: u32, b: u32) -> u32 {
        let this = e.mem.alloc(0xd0) + 0x20;
        e.mem.set_u32(this + 0x34 + 4 * a, 1 << b);
        this
    }

    fn answering_object(e: &mut Engine, slot_address: u32, value: u32) -> u32 {
        e.register_double(slot_address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
        e.put_vtable(slot_address + 0x1000, &[0, 0, 0, 0, slot_address]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, slot_address + 0x1000);
        object
    }

    #[test]
    fn thunks_use_their_adjusted_this_and_words() {
        let mut e = engine();
        let out = e.mem.alloc(8);
        let holder = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        put_u32s(&mut e, holder, &[(0x1c, 3), (0x20, 3), (0x24, 3)]);
        put_u32s(&mut e, other, &[(0x1c, 2), (0x20, 2), (0x24, 2)]);
        let base = selective_filter(&mut e, 3, 2);
        // 00e48ab0: this - 8, words +0x1c of both.
        assert_eq!(
            e.call(0x00e4_8ab0, &args![base + 8, out, holder, other])
                .u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00e4_8ab0, &args![base + 8, out, other, holder]);
        assert_eq!(e.mem.u8(out), 0);
        // 00e48b60: this - 0x14, +0x24 of the first, +0x1c of the second.
        e.call(0x00e4_8b60, &args![base + 0x14, out, holder, other]);
        assert_eq!(e.mem.u8(out), 1);
        // 00e488a0: this - 0x10, +0x20 of the holder and the object's answer.
        let object = answering_object(&mut e, 0x0ff0_0010, 2);
        assert_eq!(
            e.call(
                0x00e4_88a0,
                &args![base + 0x10, out, holder, 0u32, object, 5u32]
            )
            .u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        let object = answering_object(&mut e, 0x0ff0_0020, 1);
        e.call(
            0x00e4_88a0,
            &args![base + 0x10, out, holder, 0u32, object, 5u32],
        );
        assert_eq!(e.mem.u8(out), 0);
    }

    /// A chain `leaf -> root` where each node has the word `value` at `+0x1c`.
    fn node(e: &mut Engine, parent: u32, value: u32) -> u32 {
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 0xc, parent);
        e.mem.set_u32(node + 0x1c, value);
        node
    }

    #[test]
    fn two_object_thunk_replaces_minus_one_by_the_chain_root() {
        let mut e = engine();
        let out = e.mem.alloc(8);
        let base = selective_filter(&mut e, 3, 2);
        let root = node(&mut e, 0, 3);
        let leaf = node(&mut e, root, 9);
        let other_root = node(&mut e, 0, 2);
        let other_leaf = node(&mut e, other_root, 8);
        let minus_one = answering_object(&mut e, 0x0ff0_0300, u32::MAX);
        let plain = answering_object(&mut e, 0x0ff0_0400, 2);
        // Both answer -1: (3, 2) from the roots.
        let words = args![
            base + 0xc,
            out,
            0u32,
            leaf,
            other_leaf,
            minus_one,
            minus_one,
            1u32,
            2u32
        ];
        assert_eq!(e.call(0x00e4_8ae0, &words).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        // The second answers 7: (3, 7) is not in the table.
        let seven = answering_object(&mut e, 0x0ff0_0500, 7);
        e.call(
            0x00e4_8ae0,
            &args![
                base + 0xc,
                out,
                0u32,
                leaf,
                other_leaf,
                minus_one,
                seven,
                1u32,
                2u32
            ],
        );
        assert_eq!(e.mem.u8(out), 0);
        // A plain answer of 2 for the second and -1 for the first gives (3, 2) again.
        e.call(
            0x00e4_8ae0,
            &args![
                base + 0xc,
                out,
                0u32,
                leaf,
                other_leaf,
                minus_one,
                plain,
                1u32,
                2u32
            ],
        );
        assert_eq!(e.mem.u8(out), 1);
    }

    /// The pieces of a `00e48b90` call: the flags table owner, a chain node whose object has the
    /// kind `kind`, and a filter answering for `(wanted, 2)`.
    struct ChainSetup {
        base: u32,
        table_owner: u32,
        start: u32,
        link: u32,
        out: u32,
    }

    fn chain_setup(e: &mut Engine, kind: u32, flags: u32, wanted: u32) -> ChainSetup {
        let table = e.mem.alloc(0x200);
        e.mem.set_u32(table + 0x110 + 4 * kind, flags);
        let table_owner = e.mem.alloc(8);
        e.mem.set_u32(table_owner, table);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0xc, kind);
        let link = node(e, 0, 5);
        e.mem.set_u32(link, object);
        let start = node(e, link, 7);
        e.mem.set_u32(start + 4, 0x321);
        ChainSetup {
            base: selective_filter(e, wanted, 2),
            table_owner,
            start,
            link,
            out: e.mem.alloc(8),
        }
    }

    fn run_chain(e: &mut Engine, setup: &ChainSetup, fallback: u32) -> (u32, u8) {
        let object = answering_object(e, 0x0ff0_0060, 2);
        let result = e
            .call(
                0x00e4_8b90,
                &args![
                    setup.base + 0xc,
                    setup.out,
                    setup.table_owner,
                    setup.start,
                    fallback,
                    object,
                    0u32
                ],
            )
            .u32();
        (result, e.mem.u8(setup.out))
    }

    #[test]
    fn chain_without_a_link_uses_its_own_word() {
        let mut e = engine();
        let setup = chain_setup(&mut e, 1, 0, 7);
        e.mem.set_u32(setup.start + 0xc, 0);
        let fallback = node(&mut e, 0, 0);
        assert_eq!(run_chain(&mut e, &setup, fallback), (setup.out, 1));
    }

    #[test]
    fn chain_end_gives_the_last_nodes_word() {
        let mut e = engine();
        // No flags: the link is passed over and its word (5) is the first value.
        let setup = chain_setup(&mut e, 1, 0, 5);
        let fallback = node(&mut e, 0, 0);
        assert_eq!(run_chain(&mut e, &setup, fallback), (setup.out, 1));
    }

    #[test]
    fn flag_0x80_asks_the_sub_object() {
        let mut e = engine();
        let setup = chain_setup(&mut e, 1, 0x80, 4);
        let sub_object = e.mem.u32(setup.link);
        // The object's sub-object at +0x10 has a vtable with the answering slot 4.
        e.register_double(0x0ff0_0070, |_, a| Ret {
            eax: if a[1] == 0x321 { 4 } else { 99 },
            ..Ret::default()
        });
        e.put_vtable(0x0126_0300, &[0, 0, 0, 0, 0x0ff0_0070]);
        e.mem.set_u32(sub_object + 0x10, 0x0126_0300);
        let fallback = node(&mut e, 0, 0);
        assert_eq!(run_chain(&mut e, &setup, fallback), (setup.out, 1));
    }

    #[test]
    fn flag_0x100_asks_the_resolved_object() {
        let mut e = engine();
        let setup = chain_setup(&mut e, 1, 0x100, 6);
        let resolved = answering_object(&mut e, 0x0ff0_0080, 6);
        e.register_double(RESOLVE_NODE, move |_, _| Ret {
            eax: resolved,
            ..Ret::default()
        });
        let fallback = node(&mut e, 0, 0);
        assert_eq!(run_chain(&mut e, &setup, fallback), (setup.out, 1));
    }

    #[test]
    fn flag_0x200000_takes_the_root_word() {
        let mut e = engine();
        let setup = chain_setup(&mut e, 1, 0x20_0000, 5);
        let fallback = node(&mut e, 0, 0);
        // The chain from the start ends at the link (word 5).
        assert_eq!(run_chain(&mut e, &setup, fallback), (setup.out, 1));
    }

    #[test]
    fn flag_0x400000_accepts_without_the_filter() {
        let mut e = engine();
        let setup = chain_setup(&mut e, 1, 0x40_0000, 99);
        let fallback = node(&mut e, 0, 0);
        assert_eq!(run_chain(&mut e, &setup, fallback), (setup.out, 1));
    }

    #[test]
    fn minus_one_answer_uses_the_fallback_root() {
        let mut e = engine();
        // The fallback root's word 2 is the second value; the first is 5 from the chain end.
        let setup = chain_setup(&mut e, 1, 0, 5);
        let fallback = node(&mut e, 0, 2);
        let object = answering_object(&mut e, 0x0ff0_0090, u32::MAX);
        let result = e
            .call(
                0x00e4_8b90,
                &args![
                    setup.base + 0xc,
                    setup.out,
                    setup.table_owner,
                    setup.start,
                    fallback,
                    object,
                    0u32
                ],
            )
            .u32();
        assert_eq!(result, setup.out);
        assert_eq!(e.mem.u8(setup.out), 1);
    }

    #[test]
    fn constructors_store_their_tables() {
        let mut e = engine();
        let calls = record(&mut e, &[BASE_CONSTRUCTOR]);
        let this = e.mem.alloc(0xc0);
        for word in 0..48u32 {
            e.mem.set_u32(this + 4 * word, 0x1111_1111);
        }
        assert_eq!(e.call(0x00e4_89b0, &args![this]).u32(), this);
        assert_eq!(*calls.borrow(), vec![(BASE_CONSTRUCTOR, vec![this])]);
        assert_eq!(e.mem.u32(this), 0x010e_8a6c);
        assert_eq!(e.mem.u32(this + 0x14), 0x010e_8a34);
        assert_eq!(e.mem.u32(this + 0x20), 2);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), u32::MAX);
        assert_eq!(e.mem.u32(this + 0xb0), u32::MAX);
        assert_eq!(e.mem.u32(this + 0xb4), 0x1111_1111);
        e.call(0x00e4_8a80, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0x010c_3b80);
        assert_eq!(e.mem.u32(this + 0xc), 0x010c_3b98);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_3bac);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_3b8c);
    }

    /// A mesh for `00e48cb0`: vertices `v` are `(v, v, v, v)`, two records of four `u16`.
    fn mesh_owner(e: &mut Engine, with_user_data: bool) -> u32 {
        let this = e.mem.alloc(0x50);
        let mesh = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x80);
        for vertex in 0..8u32 {
            store_vec(e, vertices + 16 * vertex, [vertex as f32; 4]);
        }
        let records = e.mem.alloc(0x20);
        for (index, value) in [0u16, 1, 2, 3, 4, 5, 6, 7].iter().enumerate() {
            e.mem.set_u16(records + 2 * index as u32, *value);
        }
        put_u32s(
            e,
            mesh,
            &[
                (0, vertices),
                (4, 16),
                (0xc, records),
                (0x14, 8),
                (0x18, 1),
                (0x34, 1),
            ],
        );
        let user_data = e.mem.alloc(16);
        e.mem.set_u16(user_data + 4, 0x4242);
        e.mem.set_u32(this + 0x24, mesh);
        if with_user_data {
            e.mem.set_u32(this + 0x30, user_data);
            e.mem.set_u32(this + 0x34, 1);
        }
        store_vec(e, this + 0x10, [2.0; 4]);
        e.mem.set_u8(this + 0x3c, 9);
        e.mem.set_f32(this + 0x40, 0.5);
        this
    }

    #[test]
    fn the_triangle_answers_the_buffer() {
        let mut e = engine();
        let this = mesh_owner(&mut e, true);
        let out = e.mem.alloc(0x80);
        // EAX is `out` at the RET (00e48d9f); 00d21657 reads the child through it.
        assert_eq!(e.call(0x00e4_8cb0, &args![this, 1u32, out]).u32(), out);
    }

    #[test]
    fn triangle_is_built_with_the_slot_order_and_scale() {
        let mut e = engine();
        let this = mesh_owner(&mut e, true);
        let out = e.mem.alloc(0x80);

        e.call(0x00e4_8cb0, &args![this, 1u32, out]);

        assert_eq!(e.mem.u32(out), SHAPE_VTABLE);
        assert_eq!(e.mem.u16(out + 6), 1);
        assert_eq!(e.mem.u32(out + 0xc), 3);
        assert_eq!(e.mem.f32(out + 0x10), 0.5);
        assert_eq!(e.mem.u8(out + 0x16), 9);
        assert_eq!(e.mem.u16(out + 0x14), 0x4242);
        // Record 1 is (4, 5, 6, 7): first vertex 4; slot 1 names word index 2 = 6; slot 0 names
        // word index 1 = 5. All doubled.
        assert_eq!(load_vec(&e, out + 0x20), [8.0; 4]);
        assert_eq!(load_vec(&e, out + 0x30), [12.0; 4]);
        assert_eq!(load_vec(&e, out + 0x40), [10.0; 4]);
    }

    #[test]
    fn triangle_without_user_data_has_a_zero_word() {
        let mut e = engine();
        let this = mesh_owner(&mut e, false);
        let out = e.mem.alloc(0x80);
        e.mem.set_u16(out + 0x14, 0x7777);
        e.call(0x00e4_8cb0, &args![this, 0u32, out]);
        assert_eq!(e.mem.u16(out + 0x14), 0);
        // Index 0: record (0, 1, 2, 3); slot 0 names word index 1 = 1 and slot 1 names 2.
        assert_eq!(load_vec(&e, out + 0x20), [0.0; 4]);
        assert_eq!(load_vec(&e, out + 0x30), [2.0; 4]);
        assert_eq!(load_vec(&e, out + 0x40), [4.0; 4]);
    }
}
