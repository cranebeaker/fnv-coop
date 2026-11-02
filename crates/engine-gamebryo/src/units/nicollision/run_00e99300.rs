//! `NiCollision/run_00e99300` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// The use counter (`.data`): its first increment, from 0, registers the exit handler.
const USE_COUNT_0126FA60: u32 = 0x0126_fa60;
/// `00aa2080` (cdecl, two words): registers the function given in the first word.
const REGISTER_EXIT_HANDLER: u32 = 0x00aa_2080;
/// The two words handed to it.
const EXIT_HANDLER_00E98FF0: u32 = 0x00e9_8ff0;
const EXIT_HANDLER_ARGUMENT_00E99260: u32 = 0x00e9_9260;

// Translated from 00e99300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: increments the word at `0126fa60`; when it was 0, calls
/// `00aa2080(00e98ff0, 00e99260)`. Returns `this`, which it does not otherwise read.
pub fn fn_00e99300(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.global::<u32>(USE_COUNT_0126FA60);
    e.set_global(USE_COUNT_0126FA60, count.wrapping_add(1));
    if count == 0 {
        e.call(
            REGISTER_EXIT_HANDLER,
            &args![EXIT_HANDLER_00E98FF0, EXIT_HANDLER_ARGUMENT_00E99260],
        );
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// Byte `0126fa5c`: set to 1 when `fn_00e98ff0` has started.
const OBJECTS_BUILT: u32 = 0x0126_fa5c;
/// The six function addresses `fn_00e98ff0` stores first: (global, function).
const FUNCTION_GLOBALS: [(u32, u32); 6] = [
    (0x0126_fa80, 0x00ea_7490),
    (0x0126_fa84, 0x00ea_04b0),
    (0x0126_fa90, 0x00e9_e780),
    (0x0126_fa88, 0x00ea_cb00),
    (0x0126_fa7c, 0x00ea_c170),
    (0x0126_fa8c, 0x00e9_efe0),
];
/// Words of the first 0x90-byte object (offset, value).
const FIRST_OBJECT: [(u32, u32); 36] = [
    (0x0, 0x00ea_b9d0),
    (0x4, 0x0000_0000),
    (0x8, 0x0000_0000),
    (0xc, 0x0000_0000),
    (0x10, 0x0000_0000),
    (0x14, 0x0000_0000),
    (0x18, 0x00ea_7530),
    (0x1c, 0x00ea_0ad0),
    (0x20, 0x00ea_2030),
    (0x24, 0x0000_0000),
    (0x28, 0x0000_0000),
    (0x2c, 0x0000_0000),
    (0x30, 0x00e9_f620),
    (0x34, 0x0000_0000),
    (0x38, 0x00e9_f760),
    (0x3c, 0x0000_0000),
    (0x40, 0x0000_0000),
    (0x44, 0x0000_0000),
    (0x48, 0x0000_0000),
    (0x4c, 0x0000_0000),
    (0x50, 0x0000_0000),
    (0x54, 0x0000_0000),
    (0x58, 0x0000_0000),
    (0x5c, 0x0000_0000),
    (0x60, 0x00e9_e980),
    (0x64, 0x00e9_e980),
    (0x68, 0x00e9_e980),
    (0x6c, 0x0000_0000),
    (0x70, 0x00e9_e9e0),
    (0x74, 0x00e9_e980),
    (0x78, 0x00e9_cd10),
    (0x7c, 0x00e9_cde0),
    (0x80, 0x00e9_cf50),
    (0x84, 0x0000_0000),
    (0x88, 0x0000_0000),
    (0x8c, 0x00e9_d070),
];
/// Words of the second 0x90-byte object (offset, value).
const SECOND_OBJECT: [(u32, u32); 36] = [
    (0x0, 0x00ea_bb20),
    (0x4, 0x0000_0000),
    (0x8, 0x0000_0000),
    (0xc, 0x0000_0000),
    (0x10, 0x0000_0000),
    (0x14, 0x0000_0000),
    (0x18, 0x00ea_3950),
    (0x1c, 0x00ea_7d40),
    (0x20, 0x00ea_3c50),
    (0x24, 0x0000_0000),
    (0x28, 0x0000_0000),
    (0x2c, 0x0000_0000),
    (0x30, 0x00e9_f400),
    (0x34, 0x0000_0000),
    (0x38, 0x00e9_f510),
    (0x3c, 0x0000_0000),
    (0x40, 0x0000_0000),
    (0x44, 0x0000_0000),
    (0x48, 0x0000_0000),
    (0x4c, 0x0000_0000),
    (0x50, 0x0000_0000),
    (0x54, 0x0000_0000),
    (0x58, 0x0000_0000),
    (0x5c, 0x0000_0000),
    (0x60, 0x00e9_eae0),
    (0x64, 0x00e9_eae0),
    (0x68, 0x00e9_eae0),
    (0x6c, 0x00e9_eae0),
    (0x70, 0x00e9_eb60),
    (0x74, 0x00e9_eae0),
    (0x78, 0x00e9_d2c0),
    (0x7c, 0x00e9_d540),
    (0x80, 0x00e9_da30),
    (0x84, 0x0000_0000),
    (0x88, 0x0000_0000),
    (0x8c, 0x00e9_dfa0),
];
/// Words of the first 0x18-byte object (offset, value).
const THIRD_OBJECT: [(u32, u32); 6] = [
    (0x0, 0x00ea_c240),
    (0x4, 0x00ea_2190),
    (0x8, 0x00e9_f8c0),
    (0xc, 0x0000_0000),
    (0x10, 0x00e9_ea70),
    (0x14, 0x00e9_d150),
];
/// Words of the second 0x18-byte object (offset, value).
const FOURTH_OBJECT: [(u32, u32); 6] = [
    (0x0, 0x00ea_c3f0),
    (0x4, 0x00ea_3d60),
    (0x8, 0x00e9_fdc0),
    (0xc, 0x0000_0000),
    (0x10, 0x00e9_ec10),
    (0x14, 0x00e9_e2c0),
];
/// The three function addresses `fn_00e98ff0` stores last: (global, function).
const LAST_GLOBALS: [(u32, u32); 3] = [
    (0x0126_faa4, 0x00e9_c7d0),
    (0x0126_faa8, 0x00e9_c800),
    (0x0126_faac, 0x00e9_c850),
];

/// Allocates `size` bytes (`00aa1070`) and stores the (offset, value) words in them.
fn make_object(e: &mut Engine, size: u32, words: &[(u32, u32)]) -> u32 {
    let block = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    for (offset, value) in words {
        e.mem.set_u32(block.wrapping_add(*offset), *value);
    }
    block
}

// Translated from 00e98ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00e99300` registers; no
/// arguments, NiCollision): once only (byte `0126fa5c`), registers the class name at
/// `0109b0e8` with the creation function `00ead090` (`00a64900`), stores six function
/// addresses at `0126fa7c` to `0126fa90`, builds two 0x90-byte and two 0x18-byte objects of
/// function addresses (`00aa1070`; no check for a failed allocation, as in the exe), keeps
/// them in `0126fa94`, `0126fa98`, `0126fa9c` and `0126faa0` and stores three more
/// function addresses at `0126faa4` to `0126faac`.
pub fn fn_00e98ff0(e: &mut Engine) {
    if e.mem.u8(OBJECTS_BUILT) != 0 {
        return;
    }
    e.mem.set_u8(OBJECTS_BUILT, 1);
    e.call(REGISTER_LOADER, &args![0x0109_b0e8u32, 0x00ea_d090u32]);
    for (address, function) in FUNCTION_GLOBALS {
        e.mem.set_u32(address, function);
    }
    let first = make_object(e, 0x90, &FIRST_OBJECT);
    let second = make_object(e, 0x90, &SECOND_OBJECT);
    let third = make_object(e, 0x18, &THIRD_OBJECT);
    let fourth = make_object(e, 0x18, &FOURTH_OBJECT);
    e.mem.set_u32(0x0126_fa98, second);
    e.mem.set_u32(0x0126_fa94, first);
    e.mem.set_u32(0x0126_fa9c, third);
    e.mem.set_u32(0x0126_faa0, fourth);
    for (address, function) in LAST_GLOBALS {
        e.mem.set_u32(address, function);
    }
}

/// `00a63520` (`cdecl`, one word, unit `NiMain/nistream.cpp`, no Xbox name): called by
/// `fn_00e99260` with the class name `fn_00e98ff0` registered.
const RELEASE_LOADER: u32 = 0x00a6_3520;
/// `00aa10f0` (`cdecl`, one word, no Xbox name): called on each of the four blocks
/// `fn_00e98ff0` built (by its use here the free paired with `00aa1070`; unconfirmed).
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// The four object globals `fn_00e98ff0` fills, in the order `fn_00e99260` frees them.
const OBJECT_GLOBALS: [u32; 4] = [0x0126_fa94, 0x0126_fa98, 0x0126_fa9c, 0x0126_faa0];

// Translated from 00e99260 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the exit handler `fn_00e99300` registers, no arguments,
/// `cdecl`): undoes `fn_00e98ff0` once (byte `0126fa5c` set): clears the byte, calls
/// `00a63520` with the class name `0109b0e8`, clears the six function globals, hands the
/// four objects (`0126fa94`, `0126fa98`, `0126fa9c`, `0126faa0`) to `00aa10f0` one by one
/// and clears those four and the three last globals (`0126faa4` to `0126faac`).
pub fn fn_00e99260(e: &mut Engine) {
    if e.mem.u8(OBJECTS_BUILT) == 0 {
        return;
    }
    e.mem.set_u8(OBJECTS_BUILT, 0);
    e.call(RELEASE_LOADER, &args![0x0109_b0e8u32]);
    let first = e.mem.u32(OBJECT_GLOBALS[0]);
    for (address, _) in FUNCTION_GLOBALS {
        e.mem.set_u32(address, 0);
    }
    e.call(FREE_BLOCK, &args![first]);
    for address in &OBJECT_GLOBALS[1..] {
        let block = e.mem.u32(*address);
        e.call(FREE_BLOCK, &args![block]);
    }
    for address in OBJECT_GLOBALS {
        e.mem.set_u32(address, 0);
    }
    for (address, _) in LAST_GLOBALS {
        e.mem.set_u32(address, 0);
    }
}

/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00e98aa0` (`thiscall`, one word, no Xbox name): run on the element `fn_00e99330` hands out.
const ELEMENT_SET: u32 = 0x00e9_8aa0;
/// `00a90180` (`thiscall`, one word: the new capacity; unit `NiMain/nioldscreenspace.cpp` in
/// the engine map, no Xbox name): grows the pool's pointer array (by its use here).
const ARRAY_GROW: u32 = 0x00a9_0180;
/// `00ba3aa0` (`thiscall`, (index, address of the pointer), no Xbox name): stores a pointer
/// into the pool's array (by its use here).
const ARRAY_STORE: u32 = 0x00ba_3aa0;

// Translated from 00e99330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): hands out the next element of
/// a pool of 0x44-byte objects. The pool keeps its pointer array at +0x4, the array's
/// capacity (`u16`) at +0x8, its length (`u16`) at +0xA, the growth step (`u16`) at +0xE
/// and the count handed out at +0x10. While the count is below the length, the element at
/// that index gets `value` through `00e98aa0`. Otherwise a new 0x44-byte block comes from
/// the memory manager (when it is not null: words +0x0 and +0x4 zeroed, the floats at
/// +0x34 to +0x40 set to 0.0, then `00e98aa0` with `value`); when the length has reached
/// the capacity the array grows to step + length (`00a90180`), and the new pointer (0 when
/// the allocation failed) is stored at index length (`00ba3aa0`). Either way the count goes
/// up by one and the element is answered.
pub fn fn_00e99330(e: &mut Engine, this: u32, value: u32) -> u32 {
    let length = u32::from(e.mem.u16(this + 0xa));
    let handed_out = e.mem.u32(this + 0x10);
    if handed_out < length {
        let array = e.mem.u32(this + 0x4);
        let element = e.mem.u32(array.wrapping_add(handed_out.wrapping_mul(4)));
        e.call(ELEMENT_SET, &args![element, value]);
        let count = e.mem.u32(this + 0x10);
        e.mem.set_u32(this + 0x10, count.wrapping_add(1));
        return element;
    }
    let element = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x44u32])
        .u32();
    if element != 0 {
        e.mem.set_u32(element, 0);
        e.mem.set_u32(element + 0x4, 0);
        for offset in [0x34, 0x38, 0x3c, 0x40] {
            e.mem.set_f32(element + offset, 0.0);
        }
        e.call(ELEMENT_SET, &args![element, value]);
    }
    let capacity = u32::from(e.mem.u16(this + 0x8));
    let length = u32::from(e.mem.u16(this + 0xa));
    if length >= capacity {
        let step = u32::from(e.mem.u16(this + 0xe));
        e.call(ARRAY_GROW, &args![this, step + length]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(ARRAY_STORE, &args![this, length, slot]);
    });
    let count = e.mem.u32(this + 0x10);
    e.mem.set_u32(this + 0x10, count.wrapping_add(1));
    element
}

/// The bound test of a pick ray (`00eada70`, cdecl: the object, the ray origin and direction).
const RAY_HITS_BOUND: u32 = 0x00ea_da70;
/// The pick of a tri-based geometry (`00e9b4f0`, cdecl, the same four words).
const PICK_GEOMETRY: u32 = 0x00e9_b4f0;
/// `0043b300` (cdecl: an `NiRTTI` record, an object; no Xbox name): whether the object's class
/// is that one or derives from it (by its use here).
const IS_KIND_OF: u32 = 0x0043_b300;
/// The two flags (bytes) that let the pick take objects of the classes whose `NiRTTI` records
/// are at `011f4a48` and `011f5f64` by their world translation.
const PICK_CLASS_FLAG_0126FA65: u32 = 0x0126_fa65;
const PICK_CLASS_FLAG_0126FA64: u32 = 0x0126_fa64;
const PICK_CLASS_RTTI_011F4A48: u32 = 0x011f_4a48;
const PICK_CLASS_RTTI_011F5F64: u32 = 0x011f_5f64;
/// The `NiRTTI` record at `011f5eb4`, the class `fn_00e9c6f0` hands to `fn_00e9c680` (a node
/// with an active child index at +0xb0, `NiSwitchNode::m_iIndex` by the Xbox PDB's layout).
const SWITCH_NODE_RTTI_011F5EB4: u32 = 0x011f_5eb4;
/// `00439ef0` (thiscall on a point: the out point, the other point): the difference into the
/// out point; answers it.
const POINT_MINUS_POINT: u32 = 0x0043_9ef0;
/// `00439e90` (thiscall on a point: the out point, the other point): the sum; answers it.
const POINT_PLUS_POINT: u32 = 0x0043_9e90;
/// `NiPoint3::Dot` (`004b6190`, thiscall on one point, the other point), in ST0.
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// `00457990` (thiscall on a point): its length, in ST0.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `0045bb20` (thiscall on a point: the out point, a float): the scaled point; answers it.
const POINT_TIMES_SCALAR: u32 = 0x0045_bb20;
/// The `float` 0.0 at `01011d78` the ray's direction dot product is compared with.
const ZERO_01011D78: u32 = 0x0101_1d78;

/// Whether the pick is a "find first" that already has a result: pick type (+0) 1, sort type
/// (+4) 1 and a valid result count (+0x28) other than 0.
fn pick_is_done(e: &Engine, pick: u32) -> bool {
    e.mem.u32(pick) == 1 && e.mem.u32(pick + 4) == 1 && e.mem.u32(pick + 0x28) != 0
}

// Translated from 00e9c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the ray origin and direction, the `NiPick`, the object):
/// the recursive pick of one scene graph object. False for a null object, or an app-culled one
/// (flags +0x30 bit 0) when the pick observes the cull flag (+0x11). With `node` the object's
/// `IsNode` (virtual slot +0xc): false when the node's flags have bit 21; false when the ray
/// misses the object's bound (`00eada70`) unless the node's flags have bit 13. Then a node is
/// picked through its children (`fn_00e9c6f0`), a tri-based geometry (`IsTriBasedGeom`, slot
/// +0x1c) through `00e9b4f0`. Otherwise, for an object of one of two classes (each enabled by
/// its flag byte, `0126fa65` with the `NiRTTI` at `011f4a48` first, then `0126fa64` with
/// `011f5f64`; `0043b300`): when the vector from the origin to the object's world translation
/// (+0x8c, `00439ef0`) is not behind the ray (`NiPoint3::Dot`, compared with the float at
/// `01011d78`, unordered counts as in front), a record is added (`fn_00e99330` on the results at
/// pick +0x18) with that vector's length (`00457990`) as its distance; for the second class,
/// with an intersect type (+8) of 1, its point (+8) is the origin plus the direction times the
/// length (`0045bb20`, `00439e90`). Those cases answer false.
pub fn fn_00e9c490(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, object: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    let p = pick.addr();
    let o = object.addr();
    if e.mem.u8(p + 0x11) != 0 && e.mem.u8(o + 0x30) & 1 != 0 {
        return false;
    }
    let node = e.vcall(o, 0xc, &args![]).u32();
    if node != 0 && (e.mem.u32(node + 0x30) >> 21) & 1 != 0 {
        return false;
    }
    let hit = e
        .call(RAY_HITS_BOUND, &args![object, origin, direction])
        .bool();
    if !hit && !(node != 0 && (e.mem.u32(node + 0x30) >> 13) & 1 != 0) {
        return false;
    }
    if node != 0 {
        return fn_00e9c6f0(e, origin, direction, pick, object);
    }
    if e.vcall(o, 0x1c, &args![]).u32() != 0 {
        return e
            .call(PICK_GEOMETRY, &args![origin, direction, pick, object])
            .bool();
    }
    let first_class = e.mem.u8(PICK_CLASS_FLAG_0126FA65) != 0
        && e.call(IS_KIND_OF, &args![PICK_CLASS_RTTI_011F4A48, object])
            .bool();
    let second_class = !first_class
        && e.mem.u8(PICK_CLASS_FLAG_0126FA64) != 0
        && e.call(IS_KIND_OF, &args![PICK_CLASS_RTTI_011F5F64, object])
            .bool();
    if !first_class && !second_class {
        return false;
    }
    e.with_stack(0x18, |e, locals| {
        let offset = locals.addr();
        let sum = offset + 0xc;
        e.call(
            POINT_MINUS_POINT,
            &args![o.wrapping_add(0x8c), offset, origin],
        );
        let along = e.call(NI_POINT3_DOT, &args![direction, offset]).f64();
        if along < float_operand(e, ZERO_01011D78) {
            return false;
        }
        if first_class {
            let length = x87_store32(e.call(POINT_LENGTH, &args![offset]).f64());
            let record = fn_00e99330(e, p + 0x18, o);
            e.mem
                .set_u32(record.wrapping_add(0x14), x87_quiet_bits(length));
            return false;
        }
        let record = fn_00e99330(e, p + 0x18, o);
        let length = x87_quiet_bits(x87_store32(e.call(POINT_LENGTH, &args![offset]).f64()));
        e.mem.set_u32(record.wrapping_add(0x14), length);
        if e.mem.u32(p + 8) == 1 {
            let scaled = e
                .call(POINT_TIMES_SCALAR, &args![direction, offset, length])
                .u32();
            let point = e.call(POINT_PLUS_POINT, &args![origin, sum, scaled]).u32();
            for word in [0u32, 4, 8] {
                let value = e.mem.u32(point.wrapping_add(word));
                e.mem.set_u32(record.wrapping_add(8 + word), value);
            }
        }
        false
    })
}

// Translated from 00e9c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the pick traversal's four words): the pick of a switch
/// node (the active index at +0xb0, `NiSwitchNode::m_iIndex` by the Xbox PDB's layout). False for
/// a negative index; true when the pick is a finished "find first" (`pick_is_done`); otherwise
/// the active child (the children's `m_pBase` at +0xa0, `m_usSize` at +0xa6; an index past the
/// size or a null child answers false) is picked (`fn_00e9c490`) and its answer returned.
pub fn fn_00e9c680(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, node: Ptr) -> bool {
    let n = node.addr();
    let index = e.mem.u32(n + 0xb0) as i32;
    if index < 0 {
        return false;
    }
    if pick_is_done(e, pick.addr()) {
        return true;
    }
    if index >= i32::from(e.mem.u16(n + 0xa6)) {
        return false;
    }
    let children = e.mem.u32(n + 0xa0);
    let child = e
        .mem
        .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
    if child == 0 {
        return false;
    }
    fn_00e9c490(e, origin, direction, pick, Ptr::new(child))
}

// Translated from 00e9c6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the pick traversal's four words): the pick of a node. A
/// pick of intersect type (+8) 0 and pick type (+0) 1 just adds the node as a record
/// (`fn_00e99330`) and answers true. A node whose class chain (`GetRTTI`, slot +8, then each
/// record's parent at +4) reaches the record at `011f5eb4` is picked as a switch node
/// (`fn_00e9c680`). Otherwise every non-null child (`m_pBase` +0xa0, `m_usSize` +0xa6, read
/// again each step) is picked in order, answering true as soon as the pick is a finished "find
/// first" and otherwise whether any child answered true. A null node is read through (the exe
/// faults there).
pub fn fn_00e9c6f0(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, node: Ptr) -> bool {
    let p = pick.addr();
    if e.mem.u32(p + 8) == 0 && e.mem.u32(p) == 1 {
        fn_00e99330(e, p + 0x18, node.addr());
        return true;
    }
    let n = node.addr();
    if n != 0 {
        let mut rtti = e.vcall(n, 8, &args![]).u32();
        while rtti != 0 {
            if rtti == SWITCH_NODE_RTTI_011F5EB4 {
                return fn_00e9c680(e, origin, direction, pick, node);
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        }
    }
    let mut found = false;
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(n.wrapping_add(0xa6))) {
        if pick_is_done(e, p) {
            return true;
        }
        let children = e.mem.u32(n.wrapping_add(0xa0));
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        if child != 0 && fn_00e9c490(e, origin, direction, pick, Ptr::new(child)) {
            found = true;
        }
        index += 1;
    }
    found
}

/// `NiTriStrips::ms_RTTI` (`011f4a20`, the record `NiTriStrips::GetRTTI` `00a71d20` answers).
const TRI_STRIPS_RTTI_011F4A20: u32 = 0x011f_4a20;
/// `NiTriShape::ms_RTTI` (`011f4a40`, the record `NiTriShape::GetRTTI` `00a74520` answers).
const TRI_SHAPE_RTTI_011F4A40: u32 = 0x011f_4a40;
/// `NiGeometryData`'s virtual slots `GetActiveTriangleCount` (+0xa8, a `u16`) and
/// `GetTriangleIndices` (+0xac: the triangle, then the addresses of its three `u16` indices).
const GET_ACTIVE_TRIANGLE_COUNT: u32 = 0xa8;
const GET_TRIANGLE_INDICES: u32 = 0xac;
/// The slot +0x8c of the object at the geometry data's `m_spAdditionalGeomData` (+0x30): its
/// answer (a byte) decides whether the pick locks the vertices.
const ADDITIONAL_DATA_LOCKABLE: u32 = 0x8c;
/// `00a67360` (thiscall on the geometry data, one word; `NiMain/nigeometrydata.cpp`, no Xbox
/// name): called with 1 before the vertex lock; its byte answer says the data must be released
/// (`00a673d0`) when the pick is done.
const DATA_LOCK: u32 = 0x00a6_7360;
/// `00a673d0` (thiscall on the geometry data, no arguments): the release paired with `00a67360`.
const DATA_UNLOCK: u32 = 0x00a6_73d0;
/// The geometry data's lock calls (thiscall, `NiMain/nigeometrydata.cpp`, no Xbox names), each
/// filling a three-word block (array pointer, stride in bytes, a byte) the caller zeroes first:
/// the vertices (`00a67410`, the block), the texture coordinates (`00a67720`, the set 0 and the
/// block), the normals (`00a675b0`, the block) and the colors (`00a67660`, the block).
const DATA_LOCK_VERTICES: u32 = 0x00a6_7410;
const DATA_LOCK_TEXTURES: u32 = 0x00a6_7720;
const DATA_LOCK_NORMALS: u32 = 0x00a6_75b0;
const DATA_LOCK_COLORS: u32 = 0x00a6_7660;
/// `NiTriStripsData::GetStripLists` (`00a75f10`, Xbox PDB; the word at +0x4c).
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// `NiTriShapeData::GetTriList` (`00a7b850`, Xbox PDB).
const GET_TRI_LIST: u32 = 0x00a7_b850;
/// The pick's ray-triangle test (`00eadb90`, cdecl, ten words).
const RAY_HITS_TRIANGLE: u32 = 0x00ea_db90;
/// `004b4500` (thiscall on a matrix: the out point, the point): the product; answers the out point.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `004b3ae0` (cdecl: the out point, the point, the matrix): the point times the matrix (each
/// coordinate the point's dot product with a column); answers the out point.
const POINT_TIMES_MATRIX: u32 = 0x004b_3ae0;
/// `00532fd0` (cdecl: the out point, a float, an `NiPoint2`): the scaled point; answers the out.
const POINT2_TIMES_SCALAR: u32 = 0x0053_2fd0;
/// `004dfe30` (thiscall on an `NiPoint2`: the out point, the other point): the sum; answers it.
const POINT2_PLUS_POINT2: u32 = 0x004d_fe30;
/// `004a3760` (cdecl: the out point, a float, a point): the scaled point; answers the out.
const SCALAR_TIMES_POINT: u32 = 0x004a_3760;
/// `00532f40` (cdecl: the out color, a float, an `NiColorA`): the scaled color; answers the out.
const COLOR_TIMES_SCALAR: u32 = 0x0053_2f40;
/// `00532e40` (thiscall on an `NiColorA`: the out color, the other color): the sum; answers it.
const COLOR_PLUS_COLOR: u32 = 0x0053_2e40;
/// `00534780` (thiscall on a locked color block: the vertex index, the out color; unit
/// `fallout shared/tesland.cpp` in the engine map, no Xbox name): the color of that vertex (by
/// its use here).
const LOCKED_COLOR_AT: u32 = 0x0053_4780;
/// `004b3800` (thiscall on a point: the out point, the other point): the cross product.
const POINT_CROSS: u32 = 0x004b_3800;
/// `004a0c10` (thiscall on a point, no arguments): normalizes it in place.
const UNITIZE: u32 = 0x004a_0c10;
/// The CRT square root (`00ec6040`): the value in ST0 (a leading `f64`), the root in ST0.
const CRT_SQRT: u32 = 0x00ec_6040;
/// The defaults a record gets when the pick does not return the texture (two words at
/// `011f4980`), the normal (three words at `011f426c`) or the color (four words at `011a9be0`).
const DEFAULT_TEXTURE_011F4980: u32 = 0x011f_4980;
const DEFAULT_NORMAL_011F426C: u32 = 0x011f_426c;
const DEFAULT_COLOR_011A9BE0: u32 = 0x011a_9be0;
/// The four words `fn_00e9b4f0` reads when the object's pointer at +0x20 is null.
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;

/// `FLD dword [address]`.
fn ld(e: &Engine, address: u32) -> f64 {
    load_float(e, address)
}

/// A `float` memory operand of an x87 arithmetic instruction.
fn op(e: &Engine, address: u32) -> f64 {
    float_operand(e, address)
}

/// `FSTP dword [address]`.
fn st(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_u32(address, x87_store32(value));
}

/// A value stored as a float and loaded back.
fn stored(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// Copies `count` words (`MOV` pairs).
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// Writes words to `to`, `to + 4`, ...
fn write_words(e: &mut Engine, to: u32, words: &[u32]) {
    for (i, word) in (0u32..).zip(words) {
        e.mem.set_u32(to.wrapping_add(4 * i), *word);
    }
}

/// A float argument the exe pushes with `FLD dword [address]; FSTP dword [ESP]`: its bits.
fn pushed_float(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

/// Zeroes a lock block (array pointer, stride, a byte) as the exe does before a lock call.
fn clear_lock_block(e: &mut Engine, block: u32) {
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u8(block + 8, 0);
}

/// Whether the pick is a "find first" with sort type 1 (`m_ePickType` +0 and `m_eSortType` +4
/// both 1), which stops at its first hit.
fn stops_at_first_hit(e: &Engine, pick: u32) -> bool {
    e.mem.u32(pick) == 1 && e.mem.u32(pick + 4) == 1
}

/// The frame slots of a hit's barycentric weights: `1 - u - v`, `u` and `v`.
#[derive(Clone, Copy)]
struct Weights {
    w: u32,
    u: u32,
    v: u32,
}

impl Weights {
    /// `1 - u - v` (`FLD u; FADD v; FLD1; FSUBRP`), stored at its slot.
    fn store_first(&self, e: &mut Engine) {
        let weight = x87_sub(1.0, x87_add(ld(e, self.u), op(e, self.v)));
        st(e, self.w, weight);
    }

    /// The three weights as `FLD` loads them (corner 0, 1, 2).
    fn values(&self, e: &Engine) -> [f64; 3] {
        [ld(e, self.w), ld(e, self.u), ld(e, self.v)]
    }

    /// The three weights as pushed float arguments (corner 0, 1, 2).
    fn pushed(&self, e: &Engine) -> [u32; 3] {
        [
            pushed_float(e, self.w),
            pushed_float(e, self.u),
            pushed_float(e, self.v),
        ]
    }
}

/// `c0 * w + c1 * u + c2 * v` over `N` floats per corner, as the picks compute it inline: each
/// product stored as a float (`c * weight`, the last component `weight * c` from a memory
/// operand), then `(c0 w + c1 u) + c2 v`, each sum stored. `corners` holds the three corners'
/// addresses, `weights` their weights.
fn weighted<const N: usize>(e: &Engine, corners: [u32; 3], weights: [f64; 3]) -> [u32; N] {
    let product = |corner: u32, weight: f64| -> [f64; N] {
        core::array::from_fn(|i| {
            let at = corner.wrapping_add(4 * i as u32);
            if i + 1 < N {
                stored(x87_mul(ld(e, at), weight))
            } else {
                stored(x87_mul(weight, op(e, at)))
            }
        })
    };
    let far = product(corners[2], weights[2]);
    let middle = product(corners[1], weights[1]);
    let near = product(corners[0], weights[0]);
    core::array::from_fn(|i| {
        let sum = stored(x87_add(near[i], middle[i]));
        x87_store32(x87_add(sum, far[i]))
    })
}

/// The frame slots of a weighted sum through calls: the outs of the third, second and first
/// corners' scaled values, of their partial sum and of the result.
#[derive(Clone, Copy)]
struct CallSlots {
    third: u32,
    second: u32,
    first: u32,
    sum: u32,
    result: u32,
}

/// `c0 * w + c1 * u + c2 * v` through a scaling call (cdecl: out, float, corner) and an adding
/// call (thiscall on the first part: out, other part), in the exe's order: the third corner, the
/// second, the first, then `(first + second)` into `slots.sum` and that plus the third into
/// `slots.result`. Answers the last call's answer.
fn weighted_by_calls(
    e: &mut Engine,
    times: u32,
    plus: u32,
    corners: [u32; 3],
    weights: [u32; 3],
    slots: CallSlots,
) -> u32 {
    let third = e
        .call(times, &args![slots.third, weights[2], corners[2]])
        .u32();
    let second = e
        .call(times, &args![slots.second, weights[1], corners[1]])
        .u32();
    let first = e
        .call(times, &args![slots.first, weights[0], corners[0]])
        .u32();
    let sum = e.call(plus, &args![first, slots.sum, second]).u32();
    e.call(plus, &args![sum, slots.result, third]).u32()
}

/// The corners' addresses in an array of `stride`-byte elements at `base` (`index * stride +
/// base`).
fn corners_in(base: u32, stride: u32, indices: [u32; 3]) -> [u32; 3] {
    indices.map(|index| index.wrapping_mul(stride).wrapping_add(base))
}

/// The face normal through `004b3800`: the edges `v1 - v0` (to `first_edge`) and `v2 - v0` (to
/// `second_edge`), each coordinate stored, in the exe's operand order; the cross product goes to
/// `out`. Answers the call's answer.
fn face_normal_by_call(
    e: &mut Engine,
    corners: [u32; 3],
    first_edge: u32,
    second_edge: u32,
    out: u32,
) -> u32 {
    let [v0, v1, v2] = corners;
    for i in [0u32, 4, 8] {
        let value = x87_sub(ld(e, v1 + i), ld(e, v0 + i));
        st(e, first_edge + i, value);
    }
    st(e, second_edge, x87_sub(ld(e, v2), ld(e, v0)));
    st(e, second_edge + 4, x87_sub(op(e, v2 + 4), ld(e, v0 + 4)));
    st(e, second_edge + 8, x87_sub(op(e, v2 + 8), ld(e, v0 + 8)));
    e.call(POINT_CROSS, &args![first_edge, out, second_edge])
        .u32()
}

/// The colors of the three corners from the locked color block `lock` (each read into a zeroed
/// color at `zeroed[i]` through `00534780`), weighted through `00532f40`/`00532e40`. Answers the
/// result slot's address.
fn locked_colors(
    e: &mut Engine,
    lock: u32,
    indices: [u32; 3],
    zeroed: [u32; 3],
    weights: [u32; 3],
    slots: CallSlots,
) -> u32 {
    for color in zeroed {
        write_words(e, color, &[0; 4]);
    }
    for (index, color) in indices.into_iter().zip(zeroed) {
        e.call(LOCKED_COLOR_AT, &args![lock, index, color]);
    }
    weighted_by_calls(
        e,
        COLOR_TIMES_SCALAR,
        COLOR_PLUS_COLOR,
        zeroed,
        weights,
        slots,
    );
    slots.result
}

/// A hit's normal: copied to `scratch`, normalized (`004a0c10`) and, for coordinate type 1,
/// rotated to world space (`004b4500` into `rotated`); the record's +0x28 gets it.
fn finish_normal(
    e: &mut Engine,
    normal: [u32; 3],
    scratch: u32,
    rotated: u32,
    pick: u32,
    rotate: u32,
    record: u32,
) {
    write_words(e, scratch, &normal);
    e.call(UNITIZE, &args![scratch]);
    if e.mem.u32(pick + 0xc) == 1 {
        let r = e
            .call(MATRIX_TIMES_POINT, &args![rotate, rotated, scratch])
            .u32();
        copy_words(e, r, record.wrapping_add(0x28), 3);
    } else {
        copy_words(e, scratch, record.wrapping_add(0x28), 3);
    }
}

/// The three words at `address`.
fn words3(e: &Engine, address: u32) -> [u32; 3] {
    [0u32, 4, 8].map(|i| e.mem.u32(address.wrapping_add(i)))
}

/// A record's triangle (+0x18) and vertex indices (+0x1a, +0x1c, +0x1e).
fn record_triangle(e: &mut Engine, record: u32, triangle: u32, indices: [u32; 3]) {
    e.mem.set_u16(record.wrapping_add(0x18), triangle as u16);
    for (i, index) in (0u32..).zip(indices) {
        e.mem
            .set_u16(record.wrapping_add(0x1a + 2 * i), index as u16);
    }
}

/// The world rotation's rows as `fn_00e993d0` (`Strips`) and `fn_00e9a280` (`Shape`) apply them
/// to the origin (their second and third rows differ in operand order).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Rows {
    Strips,
    Shape,
}

/// The frame slots of the model-space ray.
#[derive(Clone, Copy)]
struct ModelSlots {
    difference: u32,
    inverse: u32,
    origin: u32,
    direction: u32,
}

/// The ray in model space: `(origin - translation)` (+0x8c) and `direction` through the world
/// rotation (`rotate`, transposed), times `1 / scale` (+0x30 after the rotation), each value
/// stored as a float in the exe's order.
fn model_ray(e: &mut Engine, o: u32, dir: u32, rotate: u32, slots: ModelSlots, rows: Rows) {
    let at = |offset: u32| rotate.wrapping_add(offset);
    let d = slots.difference;
    st(e, d, x87_sub(ld(e, o), op(e, at(0x24))));
    st(e, d + 4, x87_sub(ld(e, o.wrapping_add(4)), op(e, at(0x28))));
    st(e, d + 8, x87_sub(ld(e, o.wrapping_add(8)), op(e, at(0x2c))));
    st(e, slots.inverse, x87_div(1.0, ld(e, at(0x30))));
    let dy = ld(e, d + 4);
    let a = x87_mul(dy, op(e, at(0xc)));
    let dx = ld(e, d);
    let b = x87_mul(ld(e, at(0)), dx);
    let s = x87_add(b, a);
    let dz = ld(e, d + 8);
    let s = x87_add(s, x87_mul(dz, op(e, at(0x18))));
    st(e, d, s);
    let t = x87_add(x87_mul(dx, op(e, at(0x4))), x87_mul(dy, op(e, at(0x10))));
    let t = match rows {
        Rows::Strips => x87_add(t, x87_mul(ld(e, at(0x1c)), dz)),
        Rows::Shape => x87_add(t, x87_mul(dz, op(e, at(0x1c)))),
    };
    st(e, d + 4, t);
    let u = match rows {
        Rows::Strips => {
            let c = x87_mul(dx, ld(e, at(0x8)));
            let f = x87_mul(dy, ld(e, at(0x14)));
            let u = x87_add(f, c);
            x87_add(u, x87_mul(dz, op(e, at(0x20))))
        }
        Rows::Shape => {
            let f = x87_mul(dy, ld(e, at(0x14)));
            let c = x87_mul(dx, ld(e, at(0x8)));
            let u = x87_add(c, f);
            x87_add(x87_mul(dz, ld(e, at(0x20))), u)
        }
    };
    st(e, d + 8, u);
    let inverse = ld(e, slots.inverse);
    for i in [0u32, 4, 8] {
        st(e, slots.origin + i, x87_mul(ld(e, d + i), inverse));
    }
    // The direction: the same for both.
    let a = x87_mul(ld(e, dir.wrapping_add(4)), op(e, at(0xc)));
    let b = x87_mul(ld(e, at(0)), op(e, dir));
    let s = x87_add(a, b);
    let s = x87_add(s, x87_mul(ld(e, at(0x18)), op(e, dir.wrapping_add(8))));
    st(e, d, s);
    let a = x87_mul(ld(e, dir), op(e, at(0x4)));
    let b = x87_mul(ld(e, dir.wrapping_add(4)), op(e, at(0x10)));
    let t = x87_add(a, b);
    let t = x87_add(t, x87_mul(ld(e, at(0x1c)), op(e, dir.wrapping_add(8))));
    st(e, d + 4, t);
    let a = x87_mul(ld(e, at(0x14)), op(e, dir.wrapping_add(4)));
    let b = x87_mul(ld(e, at(0x8)), op(e, dir));
    let u = x87_add(a, b);
    let u = x87_add(u, x87_mul(ld(e, at(0x20)), op(e, dir.wrapping_add(8))));
    st(e, d + 8, u);
    st(e, slots.direction, x87_mul(ld(e, d), inverse));
    st(e, slots.direction + 4, x87_mul(ld(e, d + 4), inverse));
    st(e, slots.direction + 8, x87_mul(inverse, op(e, d + 8)));
}

/// The geometry data's vertex lock as the three picks start: the block at `lock` zeroed; without
/// a vertex array (+0x20) and with an additional data object (+0x30) whose slot +0x8c answers
/// true (a byte), `00a67360` (with 1; its byte answer returned) and `00a67410` on the block.
fn lock_vertices(e: &mut Engine, data: u32, vertices: u32, lock: u32) -> bool {
    let mut locked = false;
    if vertices == 0 {
        let additional = e.mem.u32(data.wrapping_add(0x30));
        if additional != 0
            && e.vcall(additional, ADDITIONAL_DATA_LOCKABLE, &args![])
                .u32() as u8
                != 0
        {
            locked = e.call(DATA_LOCK, &args![data, 1u32]).u32() as u8 != 0;
            e.call(DATA_LOCK_VERTICES, &args![data, lock]);
        }
    }
    locked
}

/// The pick of a geometry without triangles to test (`fn_00e993d0`, `fn_00e9a280`): a record of
/// the object (`fn_00e99330`) whose distance is `|translation - origin|` (the differences stored
/// at `differences`, the square at `square`, `00ec6040`). The squares of x and y are added as
/// `x^2 + y^2` (`Rows::Strips`) or `y^2 + x^2` (`Rows::Shape`). Answers true.
fn record_distance_to_translation(
    e: &mut Engine,
    o: u32,
    p: u32,
    obj: u32,
    differences: u32,
    square: u32,
    rows: Rows,
) -> bool {
    let record = fn_00e99330(e, p + 0x18, obj);
    for i in [0u32, 4, 8] {
        let value = x87_sub(ld(e, obj.wrapping_add(0x8c + i)), op(e, o.wrapping_add(i)));
        st(e, differences + i, value);
    }
    let x = ld(e, differences);
    let y = ld(e, differences + 4);
    let z = ld(e, differences + 8);
    let (xx, yy) = match rows {
        Rows::Strips => {
            let yy = x87_mul(y, y);
            (x87_mul(x, x), yy)
        }
        Rows::Shape => {
            let xx = x87_mul(x, x);
            (xx, x87_mul(y, y))
        }
    };
    let sum = match rows {
        Rows::Strips => x87_add(xx, yy),
        Rows::Shape => x87_add(yy, xx),
    };
    st(e, square, x87_add(sum, x87_mul(z, z)));
    let root = e.call(CRT_SQRT, &args![ld(e, square)]).f64();
    st(e, square, root);
    let distance = x87_quiet_bits(e.mem.u32(square));
    e.mem.set_u32(record.wrapping_add(0x14), distance);
    true
}

/// The ray-triangle test of the corner copies at `corners` (`00eadb90`): the cull byte (pick
/// +0x10) goes out in the low byte of the frame word at `cull_slot`, as the exe pushes it.
#[allow(clippy::too_many_arguments)]
fn test_triangle(
    e: &mut Engine,
    p: u32,
    model: ModelSlots,
    corners: [u32; 3],
    cull_slot: u32,
    point: u32,
    distance: u32,
    weights: Weights,
) -> bool {
    let cull = e.mem.u8(p + 0x10);
    e.mem.set_u8(cull_slot, cull);
    let cull_word = e.mem.u32(cull_slot);
    e.call(
        RAY_HITS_TRIANGLE,
        &args![
            model.origin,
            model.direction,
            corners[0],
            corners[1],
            corners[2],
            cull_word,
            point,
            distance,
            weights.u,
            weights.v
        ],
    )
    .u32() as u8
        != 0
}

// Translated from 00e993d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the ray origin and direction, the `NiPick`, the
/// `NiTriStrips`; `fn_00e9b4f0` calls it for single-strip data): the pick of a tri-strips
/// geometry. The ray goes into model space (`model_ray`). The geometry data (+0xb8) gives the
/// triangle count (`GetActiveTriangleCount`) and maybe locks its vertices (`lock_vertices`).
/// Without strip lists (`00a75f10`), or without vertices, a record of the object gets the
/// distance to the world translation and the answer is true. Otherwise each triangle `i` of the
/// strip (indices `i`, `i + 1`, `i + 2`, the first two swapped for odd `i`; a triangle whose
/// first two or last two indices are equal is skipped) is tested against the model ray
/// (`00eadb90`, culling with `m_bFrontOnly` +0x10). A hit adds a record (`fn_00e99330` on the
/// results at pick +0x18): the point (back to world space with `004b4500`, the scale and the
/// translation when `m_eCoordinateType` +0xc is 1), the distance, the triangle and its indices;
/// with `m_bReturnTexture` (+0x30) the texture coordinates weighted by `1 - u - v`, `u`, `v`
/// (the locked set 0 inline, else the data's texture array +0x2c through
/// `00532fd0`/`004dfe30`, else the default at `011f4980`); with `m_bReturnNormal` (+0x31) the
/// normal (locked normals inline, else with `m_bReturnSmoothNormal` +0x32 the data's normals
/// +0x24 through `004a3760`/`00439e90`, else the face normal through `004b3800`), normalized
/// and rotated to world space for coordinate type 1, otherwise the default at `011f426c`; with
/// `m_bReturnColor` (+0x33) the color (locked colors through `00534780`, else the data's colors
/// +0x28, both weighted through `00532f40`/`00532e40`, else the default at `011a9be0`). A pick
/// that stops at its first hit answers true there. The data lock is released (`00a673d0`) on
/// those returns (not on the no-triangle one); answers whether any triangle was hit.
pub fn fn_00e993d0(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, object: Ptr) -> bool {
    let (o, dir, p, obj) = (origin.addr(), direction.addr(), pick.addr(), object.addr());
    // The exe's frame: offsets from ESP after its four register pushes.
    e.with_stack(0x2fc, |e, frame| {
        let f = frame.addr();
        let rotate = obj.wrapping_add(0x68);
        let model = ModelSlots {
            difference: f + 0x20,
            inverse: f + 0x40,
            origin: f + 0x1d0,
            direction: f + 0x1c4,
        };
        model_ray(e, o, dir, rotate, model, Rows::Strips);
        e.mem.set_u8(f + 0x1b, 0);
        let data = e.mem.u32(obj.wrapping_add(0xb8));
        let count = e.vcall(data, GET_ACTIVE_TRIANGLE_COUNT, &args![]).u32() & 0xffff;
        let data = e.mem.u32(obj.wrapping_add(0xb8));
        e.mem.set_u32(f + 0x38, data);
        let vertex_lock = f + 0xc0;
        clear_lock_block(e, vertex_lock);
        let vertices = e.mem.u32(data.wrapping_add(0x20));
        let locked = lock_vertices(e, data, vertices, vertex_lock);
        let strips = e.call(GET_STRIP_LISTS, &args![data]).u32();
        if strips == 0 || (vertices == 0 && e.mem.u32(vertex_lock) == 0) {
            return record_distance_to_translation(e, o, p, obj, f + 0x7c, f + 0x3c, Rows::Strips);
        }
        let weights = Weights {
            w: f + 0x1c,
            u: f + 0x30,
            v: f + 0x2c,
        };
        let copies = [f + 0x90, f + 0x9c, f + 0xa8];
        let mut triangle = 0u32;
        while (triangle as u16) < (count as u16) {
            let at = strips.wrapping_add((triangle & 0xffff) * 2);
            let (first, second) = if triangle & 1 != 0 {
                (e.mem.u16(at.wrapping_add(2)), e.mem.u16(at))
            } else {
                (e.mem.u16(at), e.mem.u16(at.wrapping_add(2)))
            };
            let third = e.mem.u16(at.wrapping_add(4));
            if first != second && second != third {
                let indices = [u32::from(first), u32::from(second), u32::from(third)];
                let sources = if vertices == 0 {
                    let stride = e.mem.u32(vertex_lock + 4);
                    corners_in(e.mem.u32(vertex_lock), stride, indices)
                } else {
                    corners_in(vertices, 12, indices)
                };
                for (source, copy) in sources.into_iter().zip(copies) {
                    copy_words(e, source, copy, 3);
                }
                let point = f + 0xcc;
                if test_triangle(e, p, model, copies, f + 0x134, point, f + 0x114, weights) {
                    e.mem.set_u8(f + 0x1b, 1);
                    let record = fn_00e99330(e, p + 0x18, obj);
                    if e.mem.u32(p + 0xc) == 1 {
                        let r = e
                            .call(MATRIX_TIMES_POINT, &args![rotate, f + 0x21c, point])
                            .u32();
                        st(e, f + 0x118, ld(e, rotate.wrapping_add(0x30)));
                        let scale = ld(e, f + 0x118);
                        let x = stored(x87_mul(ld(e, r), scale));
                        let y = x87_store32(x87_mul(ld(e, r.wrapping_add(4)), scale));
                        let z = x87_store32(x87_mul(scale, op(e, r.wrapping_add(8))));
                        let x = x87_store32(x87_add(x, op(e, rotate.wrapping_add(0x24))));
                        let y = x87_store32(x87_add(ld(e, rotate.wrapping_add(0x28)), x87_m32(y)));
                        let z = x87_store32(x87_add(ld(e, rotate.wrapping_add(0x2c)), x87_m32(z)));
                        write_words(e, point, &[x, y, z]);
                    }
                    copy_words(e, point, record.wrapping_add(8), 3);
                    let distance = x87_quiet_bits(e.mem.u32(f + 0x114));
                    e.mem.set_u32(record.wrapping_add(0x14), distance);
                    record_triangle(e, record, triangle, indices);
                    weights.store_first(e);
                    strips_record_texture(e, f, p, obj, record, indices, weights);
                    strips_record_normal(e, f, p, obj, record, indices, weights);
                    strips_record_color(e, f, p, obj, record, indices, weights);
                    if stops_at_first_hit(e, p) {
                        if locked {
                            e.call(DATA_UNLOCK, &args![e.mem.u32(f + 0x38)]);
                        }
                        return true;
                    }
                }
            }
            triangle = triangle.wrapping_add(1);
        }
        if locked {
            e.call(DATA_UNLOCK, &args![e.mem.u32(f + 0x38)]);
        }
        e.mem.u8(f + 0x1b) != 0
    })
}

/// `fn_00e993d0`'s texture coordinates of a hit (its frame `f`; the data at +0x38).
fn strips_record_texture(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    let out = record.wrapping_add(0x20);
    if e.mem.u8(p + 0x30) == 0 {
        copy_words(e, DEFAULT_TEXTURE_011F4980, out, 2);
        return;
    }
    let lock = f + 0xb4;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_TEXTURES, &args![e.mem.u32(f + 0x38), 0u32, lock]);
    let base = e.mem.u32(lock);
    if base != 0 {
        let corners = corners_in(base, e.mem.u32(lock + 4), indices);
        let texture: [u32; 2] = weighted(e, corners, weights.values(e));
        write_words(e, out, &texture);
        return;
    }
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let textures = e.mem.u32(data.wrapping_add(0x2c));
    if textures == 0 {
        copy_words(e, DEFAULT_TEXTURE_011F4980, out, 2);
        return;
    }
    let slots = CallSlots {
        third: f + 0x20c,
        second: f + 0x204,
        first: f + 0x214,
        sum: f + 0x1fc,
        result: f + 0x194,
    };
    let pushed = weights.pushed(e);
    weighted_by_calls(
        e,
        POINT2_TIMES_SCALAR,
        POINT2_PLUS_POINT2,
        corners_in(textures, 8, indices),
        pushed,
        slots,
    );
    copy_words(e, slots.result, out, 2);
}

/// `fn_00e993d0`'s normal of a hit (its frame `f`; the corner copies at +0x90, +0x9c, +0xa8).
fn strips_record_normal(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    if e.mem.u8(p + 0x31) == 0 {
        copy_words(e, DEFAULT_NORMAL_011F426C, record.wrapping_add(0x28), 3);
        return;
    }
    let lock = f + 0xd8;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_NORMALS, &args![e.mem.u32(f + 0x38), lock]);
    let base = e.mem.u32(lock);
    let normal = if base != 0 {
        let corners = corners_in(base, e.mem.u32(lock + 4), indices);
        weighted::<3>(e, corners, weights.values(e))
    } else {
        let smooth = if e.mem.u8(p + 0x32) != 0 {
            let data = e.mem.u32(obj.wrapping_add(0xb8));
            e.mem.u32(data.wrapping_add(0x24))
        } else {
            0
        };
        let answer = if smooth != 0 {
            let slots = CallSlots {
                third: f + 0x270,
                second: f + 0x24c,
                first: f + 0x240,
                sum: f + 0x228,
                result: f + 0x258,
            };
            let pushed = weights.pushed(e);
            weighted_by_calls(
                e,
                SCALAR_TIMES_POINT,
                POINT_PLUS_POINT,
                corners_in(smooth, 12, indices),
                pushed,
                slots,
            )
        } else {
            let copies = [f + 0x90, f + 0x9c, f + 0xa8];
            face_normal_by_call(e, copies, f + 0x1ac, f + 0x1b8, f + 0x234)
        };
        words3(e, answer)
    };
    let rotate = obj.wrapping_add(0x68);
    finish_normal(e, normal, f + 0x44, f + 0x264, p, rotate, record);
}

/// `fn_00e993d0`'s color of a hit (its frame `f`).
fn strips_record_color(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    let out = record.wrapping_add(0x34);
    if e.mem.u8(p + 0x33) == 0 {
        copy_words(e, DEFAULT_COLOR_011A9BE0, out, 4);
        return;
    }
    let lock = f + 0x20;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_COLORS, &args![e.mem.u32(f + 0x38), lock]);
    if e.mem.u32(lock) != 0 {
        let slots = CallSlots {
            third: f + 0x27c,
            second: f + 0x29c,
            first: f + 0x2bc,
            sum: f + 0x2dc,
            result: f + 0x1ec,
        };
        let pushed = weights.pushed(e);
        let zeroed = [f + 0x174, f + 0x19c, f + 0x184];
        let result = locked_colors(e, lock, indices, zeroed, pushed, slots);
        copy_words(e, result, out, 4);
        return;
    }
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let colors = e.mem.u32(data.wrapping_add(0x28));
    if colors == 0 {
        copy_words(e, DEFAULT_COLOR_011A9BE0, out, 4);
        return;
    }
    let slots = CallSlots {
        third: f + 0x28c,
        second: f + 0x2ac,
        first: f + 0x2ec,
        sum: f + 0x2cc,
        result: f + 0x1dc,
    };
    let pushed = weights.pushed(e);
    weighted_by_calls(
        e,
        COLOR_TIMES_SCALAR,
        COLOR_PLUS_COLOR,
        corners_in(colors, 16, indices),
        pushed,
        slots,
    );
    copy_words(e, slots.result, out, 4);
}

// Translated from 00e9a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the ray origin and direction, the `NiPick`, the
/// `NiTriShape`; `fn_00e9b4f0` calls it for objects of that class): the pick of a tri-shape
/// geometry, as `fn_00e993d0` with these differences: the triangles are the triangle list's
/// (`00a7b850`, three indices each, none skipped); the corners come from the vertex lock when
/// the data was locked (`00a67360`'s answer), else from the vertex array; the hit point goes to
/// world space through the rotation inline (not `004b4500`); the unlocked texture coordinates
/// and smooth normals are weighted inline, the face normal is computed inline; only the
/// unlocked colors go through `00532f40`/`00532e40`, the locked ones (read with `00534780`) are
/// weighted inline.
pub fn fn_00e9a280(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, object: Ptr) -> bool {
    let (o, dir, p, obj) = (origin.addr(), direction.addr(), pick.addr(), object.addr());
    e.with_stack(0x2e0, |e, frame| {
        let f = frame.addr();
        let rotate = obj.wrapping_add(0x68);
        let model = ModelSlots {
            difference: f + 0x18,
            inverse: f + 0x38,
            origin: f + 0x26c,
            direction: f + 0x278,
        };
        model_ray(e, o, dir, rotate, model, Rows::Shape);
        e.mem.set_u8(f + 0x13, 0);
        let data = e.mem.u32(obj.wrapping_add(0xb8));
        let count = e.vcall(data, GET_ACTIVE_TRIANGLE_COUNT, &args![]).u32() & 0xffff;
        let data = e.mem.u32(obj.wrapping_add(0xb8));
        e.mem.set_u32(f + 0x34, data);
        let vertex_lock = f + 0x130;
        clear_lock_block(e, vertex_lock);
        let vertices = e.mem.u32(data.wrapping_add(0x20));
        let locked = lock_vertices(e, data, vertices, vertex_lock);
        let list = e.call(GET_TRI_LIST, &args![data]).u32();
        if list == 0 || (vertices == 0 && e.mem.u32(vertex_lock) == 0) {
            return record_distance_to_translation(e, o, p, obj, f + 0x9c, f + 0x2c, Rows::Shape);
        }
        let weights = Weights {
            w: f + 0x14,
            u: f + 0x30,
            v: f + 0x28,
        };
        let copies = [f + 0x100, f + 0x118, f + 0x10c];
        let mut triangle = 0u32;
        while (triangle as u16) < (count as u16) {
            let at = list.wrapping_add((triangle & 0xffff) * 6);
            let indices = [0u32, 2, 4].map(|i| u32::from(e.mem.u16(at.wrapping_add(i))));
            let sources = if locked {
                let stride = e.mem.u32(vertex_lock + 4);
                corners_in(e.mem.u32(vertex_lock), stride, indices)
            } else {
                corners_in(vertices, 12, indices)
            };
            for (source, copy) in sources.into_iter().zip(copies) {
                copy_words(e, source, copy, 3);
            }
            let point = f + 0x124;
            if test_triangle(e, p, model, copies, f + 0x1c0, point, f + 0x1e8, weights) {
                e.mem.set_u8(f + 0x13, 1);
                let record = fn_00e99330(e, p + 0x18, obj);
                if e.mem.u32(p + 0xc) == 1 {
                    shape_point_to_world(e, f, rotate, point);
                }
                copy_words(e, point, record.wrapping_add(8), 3);
                let distance = x87_quiet_bits(e.mem.u32(f + 0x1e8));
                e.mem.set_u32(record.wrapping_add(0x14), distance);
                record_triangle(e, record, triangle, indices);
                weights.store_first(e);
                shape_record_texture(e, f, p, obj, record, indices, weights);
                shape_record_normal(e, f, p, obj, record, indices, weights);
                shape_record_color(e, f, p, obj, record, indices, weights);
                if stops_at_first_hit(e, p) {
                    if locked {
                        e.call(DATA_UNLOCK, &args![e.mem.u32(f + 0x34)]);
                    }
                    return true;
                }
            }
            triangle = triangle.wrapping_add(1);
        }
        if locked {
            e.call(DATA_UNLOCK, &args![e.mem.u32(f + 0x34)]);
        }
        e.mem.u8(f + 0x13) != 0
    })
}

/// `fn_00e9a280`'s hit point taken to world space in place: the rotation's rows (not
/// transposed) applied inline, times the scale (+0x30, stored at `f + 0x3c`), plus the
/// translation (+0x24 after the rotation).
fn shape_point_to_world(e: &mut Engine, f: u32, rotate: u32, point: u32) {
    let at = |offset: u32| rotate.wrapping_add(offset);
    let py = ld(e, point + 4);
    let a = x87_mul(py, op(e, at(0x4)));
    let px = ld(e, point);
    let b = x87_mul(ld(e, at(0)), px);
    let s = x87_add(b, a);
    let pz = ld(e, point + 8);
    let s = stored(x87_add(s, x87_mul(pz, op(e, at(0x8)))));
    let t = x87_add(x87_mul(px, op(e, at(0xc))), x87_mul(py, op(e, at(0x10))));
    let t = x87_store32(x87_add(t, x87_mul(pz, op(e, at(0x14)))));
    let c = x87_mul(px, ld(e, at(0x18)));
    let d = x87_mul(py, ld(e, at(0x1c)));
    let u = x87_add(d, c);
    let u = x87_store32(x87_add(u, x87_mul(pz, op(e, at(0x20)))));
    st(e, f + 0x3c, ld(e, at(0x30)));
    let scale = ld(e, f + 0x3c);
    let x = x87_store32(x87_mul(s, scale));
    let y = x87_store32(x87_mul(x87_load32(t), scale));
    let z = x87_store32(x87_mul(scale, x87_m32(u)));
    let x = x87_store32(x87_add(ld(e, at(0x24)), x87_m32(x)));
    let y = x87_store32(x87_add(ld(e, at(0x28)), x87_m32(y)));
    let z = x87_store32(x87_add(ld(e, at(0x2c)), x87_m32(z)));
    write_words(e, point, &[x, y, z]);
}

/// `fn_00e9a280`'s texture coordinates of a hit (its frame `f`; the data at +0x34).
fn shape_record_texture(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    let out = record.wrapping_add(0x20);
    if e.mem.u8(p + 0x30) == 0 {
        copy_words(e, DEFAULT_TEXTURE_011F4980, out, 2);
        return;
    }
    let lock = f + 0x18;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_TEXTURES, &args![e.mem.u32(f + 0x34), 0u32, lock]);
    let base = e.mem.u32(lock);
    let corners = if base != 0 {
        corners_in(base, e.mem.u32(lock + 4), indices)
    } else {
        let data = e.mem.u32(obj.wrapping_add(0xb8));
        let textures = e.mem.u32(data.wrapping_add(0x2c));
        if textures == 0 {
            copy_words(e, DEFAULT_TEXTURE_011F4980, out, 2);
            return;
        }
        corners_in(textures, 8, indices)
    };
    let texture: [u32; 2] = weighted(e, corners, weights.values(e));
    write_words(e, out, &texture);
}

/// `fn_00e9a280`'s normal of a hit (its frame `f`; the corner copies at +0x100, +0x118, +0x10c).
fn shape_record_normal(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    if e.mem.u8(p + 0x31) == 0 {
        copy_words(e, DEFAULT_NORMAL_011F426C, record.wrapping_add(0x28), 3);
        return;
    }
    let lock = f + 0x148;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_NORMALS, &args![e.mem.u32(f + 0x34), lock]);
    let base = e.mem.u32(lock);
    let corners = if base != 0 {
        Some(corners_in(base, e.mem.u32(lock + 4), indices))
    } else {
        let normals = if e.mem.u8(p + 0x32) != 0 {
            let data = e.mem.u32(obj.wrapping_add(0xb8));
            e.mem.u32(data.wrapping_add(0x24))
        } else {
            0
        };
        (normals != 0).then(|| corners_in(normals, 12, indices))
    };
    let normal = match corners {
        Some(corners) => weighted::<3>(e, corners, weights.values(e)),
        None => shape_face_normal(e, [f + 0x100, f + 0x118, f + 0x10c]),
    };
    let rotate = obj.wrapping_add(0x68);
    finish_normal(e, normal, f + 0x44, f + 0x294, p, rotate, record);
}

/// `fn_00e9a280`'s face normal `(v1 - v0) x (v2 - v0)`, inline: each edge coordinate stored,
/// then each cross product coordinate as the difference of two products in the exe's order.
fn shape_face_normal(e: &Engine, corners: [u32; 3]) -> [u32; 3] {
    let [v0, v1, v2] = corners;
    let [e1x, e1y, e1z] = [0u32, 4, 8].map(|i| stored(x87_sub(ld(e, v1 + i), ld(e, v0 + i))));
    let e2x = stored(x87_sub(ld(e, v2), ld(e, v0)));
    let e2y = stored(x87_sub(op(e, v2 + 4), ld(e, v0 + 4)));
    let e2z = stored(x87_sub(op(e, v2 + 8), ld(e, v0 + 8)));
    let x = x87_sub(x87_mul(e2z, e1y), x87_mul(e2y, e1z));
    let y = x87_sub(x87_mul(e1z, e2x), x87_mul(e2z, e1x));
    let z = x87_sub(x87_mul(e1x, e2y), x87_mul(e2x, e1y));
    [x87_store32(x), x87_store32(y), x87_store32(z)]
}

/// `fn_00e9a280`'s color of a hit (its frame `f`).
fn shape_record_color(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    let out = record.wrapping_add(0x34);
    if e.mem.u8(p + 0x33) == 0 {
        copy_words(e, DEFAULT_COLOR_011A9BE0, out, 4);
        return;
    }
    let lock = f + 0x13c;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_COLORS, &args![e.mem.u32(f + 0x34), lock]);
    if e.mem.u32(lock) != 0 {
        let zeroed = [f + 0x154, f + 0x174, f + 0x164];
        for color in zeroed {
            write_words(e, color, &[0; 4]);
        }
        for (index, color) in indices.into_iter().zip(zeroed) {
            e.call(LOCKED_COLOR_AT, &args![lock, index, color]);
        }
        let color: [u32; 4] = weighted(e, zeroed, weights.values(e));
        write_words(e, out, &color);
        return;
    }
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let colors = e.mem.u32(data.wrapping_add(0x28));
    if colors == 0 {
        copy_words(e, DEFAULT_COLOR_011A9BE0, out, 4);
        return;
    }
    let slots = CallSlots {
        third: f + 0x2a0,
        second: f + 0x2c0,
        first: f + 0x2d0,
        sum: f + 0x2b0,
        result: f + 0x284,
    };
    let pushed = weights.pushed(e);
    weighted_by_calls(
        e,
        COLOR_TIMES_SCALAR,
        COLOR_PLUS_COLOR,
        corners_in(colors, 16, indices),
        pushed,
        slots,
    );
    copy_words(e, slots.result, out, 4);
}

// Translated from 00e9b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the ray origin and direction, the `NiPick`, the
/// tri-based geometry; `fn_00e9c490` calls it): the pick of a tri-based geometry.
///
/// With `m_eIntersectType` (+8) 0 or a skin instance (+0xbc): the four words at object +0x20
/// (`011f4288` when null) are a center; when `(center - origin) . direction` (each difference
/// stored) is not below the float 0.0 at `01011d78` (unordered counts as not below), a record
/// gets `|center - origin|` (`00ec6040`) as its distance and, for intersect type 1, the point
/// `origin + direction * distance`; answers true, else false.
///
/// Otherwise an object whose class chain (`GetRTTI`, slot +8; each record's parent at +4)
/// reaches `NiTriStrips` with one strip (`u16` at data +0x44) is picked by `fn_00e993d0`, else
/// one of class `NiTriShape` (`0043b300`) by `fn_00e9a280`. The rest: the ray goes into model
/// space through `004b3ae0` with the world rotation (+0x68), the differences and `1 / scale`
/// stored; the data maybe locks its vertices (`lock_vertices`); without vertices a record gets
/// `|translation - origin|` (`00439ef0`, `00457990`) and the answer is true (the data lock is
/// not released there). Otherwise each triangle of `GetActiveTriangleCount` comes from
/// `GetTriangleIndices` and is tested and recorded as in `fn_00e993d0` (the point back to world
/// space through `004b4500`, the scale and the translation; texture coordinates and normals of
/// the locks weighted inline, the unlocked ones and the colors through calls). Without
/// `m_bReturnColor`'s arrays (no lock, no data colors) the record's color is left as it is.
pub fn fn_00e9b4f0(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, object: Ptr) -> bool {
    let (o, dir, p, obj) = (origin.addr(), direction.addr(), pick.addr(), object.addr());
    e.with_stack(0x2f4, |e, frame| {
        let f = frame.addr();
        if e.mem.u32(p + 8) == 0 || e.mem.u32(obj.wrapping_add(0xbc)) != 0 {
            return pick_center(e, f, o, dir, p, obj);
        }
        let mut rtti = e.vcall(obj, 8, &args![]).u32();
        let mut strips = false;
        while rtti != 0 {
            if rtti == TRI_STRIPS_RTTI_011F4A20 {
                strips = true;
                break;
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        }
        if strips {
            let data = e.mem.u32(obj.wrapping_add(0xb8));
            if e.mem.u16(data.wrapping_add(0x44)) == 1 {
                return fn_00e993d0(e, origin, direction, pick, object);
            }
        } else if e
            .call(IS_KIND_OF, &args![TRI_SHAPE_RTTI_011F4A40, obj])
            .u32() as u8
            != 0
        {
            return fn_00e9a280(e, origin, direction, pick, object);
        }
        pick_triangles(e, f, o, dir, p, obj)
    })
}

/// `fn_00e9b4f0`'s pick by the center at object +0x20 (frame `f`).
fn pick_center(e: &mut Engine, f: u32, o: u32, dir: u32, p: u32, obj: u32) -> bool {
    let mut center = e.mem.u32(obj.wrapping_add(0x20));
    if center == 0 {
        center = DEFAULT_BOUND_011F4288;
    }
    copy_words(e, center, f + 0x168, 4);
    for i in [0u32, 4, 8] {
        let value = x87_sub(ld(e, f + 0x168 + i), op(e, o.wrapping_add(i)));
        st(e, f + 0x38 + i, value);
    }
    let a = x87_mul(ld(e, dir.wrapping_add(4)), op(e, f + 0x3c));
    let b = x87_mul(ld(e, f + 0x38), op(e, dir));
    let s = x87_add(a, b);
    let s = x87_add(s, x87_mul(ld(e, dir.wrapping_add(8)), op(e, f + 0x40)));
    st(e, f + 0x24, s);
    if ld(e, f + 0x24) < op(e, ZERO_01011D78) {
        return false;
    }
    let record = fn_00e99330(e, p + 0x18, obj);
    let x = ld(e, f + 0x38);
    let y = ld(e, f + 0x3c);
    let z = ld(e, f + 0x40);
    let xx = x87_mul(x, x);
    let yy = x87_mul(y, y);
    let sum = x87_add(yy, xx);
    st(e, f + 0x24, x87_add(x87_mul(z, z), sum));
    let root = e.call(CRT_SQRT, &args![ld(e, f + 0x24)]).f64();
    st(e, f + 0x24, root);
    let distance = x87_quiet_bits(e.mem.u32(f + 0x24));
    e.mem.set_u32(record.wrapping_add(0x14), distance);
    if e.mem.u32(p + 8) == 1 {
        let t = x87_load32(distance);
        let px = x87_store32(x87_mul(t, op(e, dir)));
        let py = x87_store32(x87_mul(ld(e, dir.wrapping_add(4)), t));
        let pz = x87_store32(x87_mul(t, op(e, dir.wrapping_add(8))));
        let x = x87_store32(x87_add(ld(e, o), x87_m32(px)));
        let y = x87_store32(x87_add(ld(e, o.wrapping_add(4)), x87_m32(py)));
        let z = x87_store32(x87_add(x87_load32(pz), op(e, o.wrapping_add(8))));
        write_words(e, record.wrapping_add(8), &[x, y, z]);
    }
    true
}

/// `fn_00e9b4f0`'s pick of the triangles from `GetTriangleIndices` (frame `f`).
fn pick_triangles(e: &mut Engine, f: u32, o: u32, dir: u32, p: u32, obj: u32) -> bool {
    let rotate = obj.wrapping_add(0x68);
    let at = |offset: u32| rotate.wrapping_add(offset);
    // The model ray through `004b3ae0`: the origin at +0x1d4, the direction at +0x1c8.
    let difference = f + 0x48;
    st(e, difference, x87_sub(ld(e, o), op(e, at(0x24))));
    st(
        e,
        difference + 4,
        x87_sub(ld(e, o.wrapping_add(4)), op(e, at(0x28))),
    );
    st(
        e,
        difference + 8,
        x87_sub(ld(e, o.wrapping_add(8)), op(e, at(0x2c))),
    );
    st(e, f + 0x28, x87_div(1.0, ld(e, at(0x30))));
    let model_origin = f + 0x1d4;
    let model_direction = f + 0x1c8;
    for (vector, out, temporary) in [
        (difference, model_origin, f + 0x20),
        (dir, model_direction, f + 0x28),
    ] {
        let inverse = ld(e, f + 0x28);
        let r = e
            .call(POINT_TIMES_MATRIX, &args![f + 0xc4, vector, rotate])
            .u32();
        st(e, f + 0x44, x87_mul(ld(e, r.wrapping_add(4)), inverse));
        st(e, temporary, x87_mul(ld(e, r.wrapping_add(8)), inverse));
        st(e, out, x87_mul(inverse, op(e, r)));
        st(e, out + 4, ld(e, f + 0x44));
        st(e, out + 8, ld(e, temporary));
    }
    let model = ModelSlots {
        difference,
        inverse: f + 0x28,
        origin: model_origin,
        direction: model_direction,
    };
    e.mem.set_u8(f + 0x1f, 0);
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let count = e.vcall(data, GET_ACTIVE_TRIANGLE_COUNT, &args![]).u32() & 0xffff;
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let vertex_lock = f + 0xd0;
    clear_lock_block(e, vertex_lock);
    let vertices = e.mem.u32(data.wrapping_add(0x20));
    e.mem.set_u8(f + 0x1e, 0);
    e.mem.set_u32(f + 0x2c, data);
    let locked = lock_vertices(e, data, vertices, vertex_lock);
    if vertices == 0 && e.mem.u32(vertex_lock) == 0 {
        let record = fn_00e99330(e, p + 0x18, obj);
        e.call(
            POINT_MINUS_POINT,
            &args![obj.wrapping_add(0x8c), f + 0xc4, o],
        );
        let length = e.call(POINT_LENGTH, &args![f + 0xc4]).f64();
        st(e, record.wrapping_add(0x14), length);
        return true;
    }
    let weights = Weights {
        w: f + 0x20,
        u: f + 0x30,
        v: f + 0x34,
    };
    let copies = [f + 0xa0, f + 0x94, f + 0x88];
    let mut triangle = 0u32;
    while (triangle as u16) < (count as u16) {
        let data = e.mem.u32(obj.wrapping_add(0xb8));
        e.vcall(
            data,
            GET_TRIANGLE_INDICES,
            &args![triangle, f + 0x10, f + 0x14, f + 0x18],
        );
        let indices = [0x10u32, 0x14, 0x18].map(|slot| u32::from(e.mem.u16(f + slot)));
        let sources = if vertices == 0 {
            let stride = e.mem.u32(vertex_lock + 4);
            corners_in(e.mem.u32(vertex_lock), stride, indices)
        } else {
            corners_in(vertices, 12, indices)
        };
        for (source, copy) in sources.into_iter().zip(copies) {
            copy_words(e, source, copy, 3);
        }
        let point = f + 0xac;
        if test_triangle(e, p, model, copies, f + 0x128, point, f + 0x24, weights) {
            e.mem.set_u8(f + 0x1f, 1);
            let record = fn_00e99330(e, p + 0x18, obj);
            if e.mem.u32(p + 0xc) == 1 {
                let r = e
                    .call(MATRIX_TIMES_POINT, &args![rotate, f + 0x238, point])
                    .u32();
                st(e, f + 0x20, ld(e, at(0x30)));
                let scale = ld(e, f + 0x20);
                let x = x87_store32(x87_mul(scale, op(e, r)));
                let y = x87_store32(x87_mul(ld(e, r.wrapping_add(4)), scale));
                let z = x87_store32(x87_mul(scale, op(e, r.wrapping_add(8))));
                let x = x87_store32(x87_add(x87_load32(x), op(e, at(0x24))));
                let y = x87_store32(x87_add(ld(e, at(0x28)), x87_m32(y)));
                let z = x87_store32(x87_add(ld(e, at(0x2c)), x87_m32(z)));
                write_words(e, point, &[x, y, z]);
            }
            copy_words(e, point, record.wrapping_add(8), 3);
            let distance = x87_quiet_bits(e.mem.u32(f + 0x24));
            e.mem.set_u32(record.wrapping_add(0x14), distance);
            record_triangle(e, record, triangle, indices);
            weights.store_first(e);
            triangles_record_texture(e, f, p, obj, record, indices, weights);
            triangles_record_normal(e, f, p, obj, record, indices, weights);
            triangles_record_color(e, f, p, obj, record, indices, weights);
            if stops_at_first_hit(e, p) {
                if locked {
                    e.call(DATA_UNLOCK, &args![e.mem.u32(f + 0x2c)]);
                }
                return true;
            }
        }
        triangle = triangle.wrapping_add(1);
    }
    if locked {
        e.call(DATA_UNLOCK, &args![e.mem.u32(f + 0x2c)]);
    }
    e.mem.u8(f + 0x1f) != 0
}

/// `fn_00e9b4f0`'s texture coordinates of a hit (its frame `f`; the data at +0x2c).
fn triangles_record_texture(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    let out = record.wrapping_add(0x20);
    if e.mem.u8(p + 0x30) == 0 {
        copy_words(e, DEFAULT_TEXTURE_011F4980, out, 2);
        return;
    }
    let lock = f + 0x138;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_TEXTURES, &args![e.mem.u32(f + 0x2c), 0u32, lock]);
    let base = e.mem.u32(lock);
    if base != 0 {
        let corners = corners_in(base, e.mem.u32(lock + 4), indices);
        let texture: [u32; 2] = weighted(e, corners, weights.values(e));
        write_words(e, out, &texture);
        return;
    }
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let textures = e.mem.u32(data.wrapping_add(0x2c));
    if textures == 0 {
        copy_words(e, DEFAULT_TEXTURE_011F4980, out, 2);
        return;
    }
    let slots = CallSlots {
        third: f + 0x200,
        second: f + 0x210,
        first: f + 0x208,
        sum: f + 0x218,
        result: f + 0x184,
    };
    let pushed = weights.pushed(e);
    weighted_by_calls(
        e,
        POINT2_TIMES_SCALAR,
        POINT2_PLUS_POINT2,
        corners_in(textures, 8, indices),
        pushed,
        slots,
    );
    copy_words(e, slots.result, out, 2);
}

/// `fn_00e9b4f0`'s normal of a hit (its frame `f`; the corner copies at +0xa0, +0x94, +0x88).
fn triangles_record_normal(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    if e.mem.u8(p + 0x31) == 0 {
        copy_words(e, DEFAULT_NORMAL_011F426C, record.wrapping_add(0x28), 3);
        return;
    }
    let lock = f + 0xdc;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_NORMALS, &args![e.mem.u32(f + 0x2c), lock]);
    let base = e.mem.u32(lock);
    let normal = if base != 0 {
        let corners = corners_in(base, e.mem.u32(lock + 4), indices);
        weighted::<3>(e, corners, weights.values(e))
    } else {
        let smooth = if e.mem.u8(p + 0x32) != 0 {
            let data = e.mem.u32(obj.wrapping_add(0xb8));
            e.mem.u32(data.wrapping_add(0x24))
        } else {
            0
        };
        let answer = if smooth != 0 {
            let slots = CallSlots {
                third: f + 0x220,
                second: f + 0x250,
                first: f + 0x244,
                sum: f + 0x25c,
                result: f + 0x268,
            };
            let pushed = weights.pushed(e);
            weighted_by_calls(
                e,
                SCALAR_TIMES_POINT,
                POINT_PLUS_POINT,
                corners_in(smooth, 12, indices),
                pushed,
                slots,
            )
        } else {
            let copies = [f + 0xa0, f + 0x94, f + 0x88];
            face_normal_by_call(e, copies, f + 0xc4, f + 0x1bc, f + 0x22c)
        };
        words3(e, answer)
    };
    let rotate = obj.wrapping_add(0x68);
    finish_normal(e, normal, f + 0x38, f + 0x168, p, rotate, record);
}

/// `fn_00e9b4f0`'s color of a hit (its frame `f`). Without a color lock and without data
/// colors the record's color is not written.
fn triangles_record_color(
    e: &mut Engine,
    f: u32,
    p: u32,
    obj: u32,
    record: u32,
    indices: [u32; 3],
    weights: Weights,
) {
    let out = record.wrapping_add(0x34);
    if e.mem.u8(p + 0x33) == 0 {
        copy_words(e, DEFAULT_COLOR_011A9BE0, out, 4);
        return;
    }
    let lock = f + 0xb8;
    clear_lock_block(e, lock);
    e.call(DATA_LOCK_COLORS, &args![e.mem.u32(f + 0x2c), lock]);
    if e.mem.u32(lock) != 0 {
        let slots = CallSlots {
            third: f + 0x274,
            second: f + 0x294,
            first: f + 0x2b4,
            sum: f + 0x2d4,
            result: f + 0x1f0,
        };
        let pushed = weights.pushed(e);
        let zeroed = [f + 0x18c, f + 0x19c, f + 0x1ac];
        let result = locked_colors(e, lock, indices, zeroed, pushed, slots);
        copy_words(e, result, out, 4);
        return;
    }
    let data = e.mem.u32(obj.wrapping_add(0xb8));
    let colors = e.mem.u32(data.wrapping_add(0x28));
    if colors == 0 {
        return;
    }
    let slots = CallSlots {
        third: f + 0x284,
        second: f + 0x2a4,
        first: f + 0x2e4,
        sum: f + 0x2c4,
        result: f + 0x1e0,
    };
    let pushed = weights.pushed(e);
    weighted_by_calls(
        e,
        COLOR_TIMES_SCALAR,
        COLOR_PLUS_COLOR,
        corners_in(colors, 16, indices),
        pushed,
        slots,
    );
    copy_words(e, slots.result, out, 4);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_93d0, fn_00e993d0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_a280, fn_00e9a280(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_b4f0, fn_00e9b4f0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_c490, fn_00e9c490(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_c680, fn_00e9c680(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_c6f0, fn_00e9c6f0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_9300, fn_00e99300(Ptr) -> Ptr),
        entry!(0x00e9_8ff0, fn_00e98ff0()),
        entry!(0x00e9_9260, fn_00e99260()),
        entry!(0x00e9_9330, fn_00e99330(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e99260_releases_once_and_clears_the_globals() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(RELEASE_LOADER, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // Not built: nothing.
        e.call(0x00e9_9260, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        e.mem.set_u8(OBJECTS_BUILT, 1);
        for (i, address) in (0x5000u32..).zip(OBJECT_GLOBALS) {
            e.mem.set_u32(address, i);
        }
        for (address, f) in FUNCTION_GLOBALS.iter().chain(LAST_GLOBALS.iter()) {
            e.mem.set_u32(*address, *f);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e9_9260, &[]);
        assert_eq!(e.mem.u8(OBJECTS_BUILT), 0);
        assert_eq!(calls_to(&e, RELEASE_LOADER), vec![vec![0x0109_b0e8]]);
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            vec![vec![0x5000], vec![0x5001], vec![0x5002], vec![0x5003]]
        );
        for address in (0x0126_fa7c..=0x0126_faac).step_by(4) {
            assert_eq!(e.mem.u32(address), 0, "{address:08x}");
        }
    }

    #[test]
    fn fn_00e99330_reuses_then_allocates_and_grows() {
        let mut e = Engine::new();
        let pool = e.mem.alloc(0x14);
        let array = e.mem.alloc(0x10);
        let reused = e.mem.alloc(0x44);
        e.mem.set_u32(pool + 0x4, array);
        e.mem.set_u32(array, reused);
        e.mem.set_u16(pool + 0x8, 2);
        e.mem.set_u16(pool + 0xa, 1);
        e.mem.set_u16(pool + 0xe, 3);
        e.register(ELEMENT_SET, |_, _| Ret::default());
        e.register(ARRAY_GROW, |_, _| Ret::default());
        let fresh = e.mem.alloc(0x44);
        e.mem.set_u32(fresh, 0xffff_ffff);
        e.mem.set_u32(fresh + 0x40, 0xffff_ffff);
        e.register_double(MEMORY_ALLOCATE, move |_, _| fresh.into_ret());
        e.register_double(ARRAY_STORE, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), fresh);
            Ret::default()
        });

        // Count 0 < length 1: the element at index 0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 9]).u32(), reused);
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![reused, 9]]);
        assert_eq!(e.mem.u32(pool + 0x10), 1);

        // Count 1 == length 1 < capacity 2: a new block, stored at index 1, no growth.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 7]).u32(), fresh);
        assert_eq!(e.mem.u32(fresh), 0);
        assert_eq!(e.mem.u32(fresh + 0x40), 0);
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![fresh, 7]]);
        assert!(calls_to(&e, ARRAY_GROW).is_empty());
        let stores = calls_to(&e, ARRAY_STORE);
        assert_eq!(stores.len(), 1);
        assert_eq!(&stores[0][..2], &[pool, 1]);
        assert_eq!(e.mem.u32(pool + 0x10), 2);

        // Length 2 == capacity 2: grows to 3 + 2 first; a failed allocation stores 0.
        e.mem.set_u16(pool + 0xa, 2);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_STORE, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 7]).u32(), 0);
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![pool, 5]]);
        assert!(calls_to(&e, ELEMENT_SET).is_empty());
        assert_eq!(e.mem.u32(pool + 0x10), 3);
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
    fn registers_the_handler_on_the_first_use_only() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(REGISTER_EXIT_HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9300, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(USE_COUNT_0126FA60), 1);
        assert_eq!(
            calls_to(&mut e, REGISTER_EXIT_HANDLER),
            vec![vec![0x00e9_8ff0, 0x00e9_9260]]
        );
        e.call(0x00e9_9300, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(USE_COUNT_0126FA60), 2);
        assert_eq!(calls_to(&mut e, REGISTER_EXIT_HANDLER).len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_four_objects_once() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_8ff0, &args![]);
        let loaders: Vec<_> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == REGISTER_LOADER)
            .collect();
        assert_eq!(
            loaders,
            vec![(REGISTER_LOADER, vec![0x0109_b0e8, 0x00ea_d090])]
        );
        let first = e.mem.u32(0x0126_fa94);
        let second = e.mem.u32(0x0126_fa98);
        let third = e.mem.u32(0x0126_fa9c);
        let fourth = e.mem.u32(0x0126_faa0);
        assert_eq!(e.mem.u32(first), 0x00ea_b9d0);
        assert_eq!(e.mem.u32(first + 0x60), 0x00e9_e980);
        assert_eq!(e.mem.u32(first + 0x8c), 0x00e9_d070);
        assert_eq!(e.mem.u32(second), 0x00ea_bb20);
        assert_eq!(e.mem.u32(second + 0x60), 0x00e9_eae0);
        assert_eq!(e.mem.u32(second + 0x70), 0x00e9_eb60);
        assert_eq!(e.mem.u32(third), 0x00ea_c240);
        assert_eq!(e.mem.u32(third + 0x14), 0x00e9_d150);
        assert_eq!(e.mem.u32(fourth), 0x00ea_c3f0);
        assert_eq!(e.mem.u32(fourth + 0x14), 0x00e9_e2c0);
        assert_eq!(e.mem.u32(0x0126_fa80), 0x00ea_7490);
        assert_eq!(e.mem.u32(0x0126_faac), 0x00e9_c850);
        assert_eq!(e.mem.u8(OBJECTS_BUILT), 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8ff0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}

#[cfg(test)]
mod n034_pick_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// Test functions for the virtual slots: answer 0, answer the object itself, answer 1.
    const ANSWER_ZERO: u32 = 0x00ab_1000;
    const ANSWER_SELF: u32 = 0x00ab_1001;
    const ANSWER_ONE: u32 = 0x00ab_1002;
    const ANSWER_SWITCH_RTTI: u32 = 0x00ab_1003;

    fn doubles(e: &mut Engine) {
        e.register(ANSWER_ZERO, |_, _| Ret::default());
        e.register(ANSWER_SELF, |_, a| a[0].into_ret());
        e.register(ANSWER_ONE, |_, _| 1u32.into_ret());
        e.register(ANSWER_SWITCH_RTTI, |_, _| {
            SWITCH_NODE_RTTI_011F5EB4.into_ret()
        });
        e.register(ELEMENT_SET, |_, _| Ret::default());
        e.register(RAY_HITS_BOUND, |_, _| 1u32.into_ret());
        e.map(0x011f_5000, 0x1000);
        e.map(0x0126_f000, 0x1000);
    }

    /// An object whose `GetRTTI`, `IsNode` and `IsTriBasedGeom` are the given functions.
    fn object(e: &mut Engine, rtti: u32, is_node: u32, is_geometry: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 8, rtti);
        e.mem.set_u32(vtable + 0xc, is_node);
        e.mem.set_u32(vtable + 0x1c, is_geometry);
        let o = e.mem.alloc(0xc0);
        e.mem.set_u32(o, vtable);
        o
    }

    /// A pick (type, sort, intersect) with a results pool of four ready elements at +0x18.
    fn pick(e: &mut Engine, kind: u32, sort: u32, intersect: u32) -> (u32, Vec<u32>) {
        let p = e.mem.alloc(0x40);
        e.mem.set_u32(p, kind);
        e.mem.set_u32(p + 4, sort);
        e.mem.set_u32(p + 8, intersect);
        let array = e.mem.alloc(0x10);
        let mut records = vec![];
        for i in 0..4 {
            let r = e.mem.alloc(0x44);
            e.mem.set_u32(array + 4 * i, r);
            records.push(r);
        }
        e.mem.set_u32(p + 0x18 + 4, array);
        e.mem.set_u16(p + 0x18 + 0xa, 4);
        (p, records)
    }

    fn with_children(e: &mut Engine, node: u32, children: &[u32]) {
        let array = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
    }

    #[test]
    fn traversal_rejects_null_culled_and_missed_objects() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (p, _) = pick(&mut e, 0, 0, 1);
        assert!(!fn_00e9c490(
            &mut e,
            Ptr::new(0x10),
            Ptr::new(0x20),
            Ptr::new(p),
            Ptr::NULL
        ));
        let geometry = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        e.mem.set_u32(geometry + 0x30, 1);
        e.mem.set_u8(p + 0x11, 1);
        e.register(PICK_GEOMETRY, |_, _| 1u32.into_ret());
        let r = e.call(0x00e9_c490, &args![0x10u32, 0x20u32, p, geometry]);
        assert!(!r.bool());
        e.mem.set_u8(p + 0x11, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, geometry])
            .bool());
        assert_eq!(
            calls_to(&e, PICK_GEOMETRY),
            vec![vec![0x10, 0x20, p, geometry]]
        );
        assert_eq!(
            calls_to(&e, RAY_HITS_BOUND),
            vec![vec![geometry, 0x10, 0x20]]
        );
        // A missed bound stops the pick.
        e.register(RAY_HITS_BOUND, |_, _| Ret::default());
        assert!(!e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, geometry])
            .bool());
        // A node with flag bit 21 is skipped before the bound test; bit 13 picks it anyway.
        let node = object(&mut e, ANSWER_ZERO, ANSWER_SELF, ANSWER_ZERO);
        with_children(&mut e, node, &[geometry]);
        e.mem.set_u32(node + 0x30, 1 << 21);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, node])
            .bool());
        assert!(calls_to(&e, RAY_HITS_BOUND).is_empty());
        e.mem.set_u32(node + 0x30, 1 << 13);
        e.mem.set_u32(geometry + 0x30, 0);
        e.register_double(RAY_HITS_BOUND, move |_, a| (a[0] == geometry).into_ret());
        assert!(e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, node])
            .bool());
    }

    #[test]
    fn nodes_pick_each_child_and_stop_when_done() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(PICK_GEOMETRY, |e, a| {
            // The pick finds a result on its first geometry.
            e.mem.set_u32(a[2] + 0x28, 1);
            1u32.into_ret()
        });
        let first = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let second = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let node = object(&mut e, ANSWER_ZERO, ANSWER_SELF, ANSWER_ZERO);
        with_children(&mut e, node, &[first, 0, second]);
        // Find all: every child.
        let (p, _) = pick(&mut e, 0, 0, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]).bool());
        assert_eq!(calls_to(&e, PICK_GEOMETRY).len(), 2);
        // Find first, no sort: stops after the first result.
        let (p, _) = pick(&mut e, 1, 1, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]).bool());
        assert_eq!(calls_to(&e, PICK_GEOMETRY).len(), 1);
        // Bound-only find first: the node itself becomes a record.
        let (p, r) = pick(&mut e, 1, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]).bool());
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![r[0], node]]);
        assert!(calls_to(&e, PICK_GEOMETRY).is_empty());
    }

    #[test]
    fn switch_nodes_pick_only_the_active_child() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(PICK_GEOMETRY, |_, a| (a[3] & 0xff).into_ret());
        let first = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let second = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let node = object(&mut e, ANSWER_SWITCH_RTTI, ANSWER_SELF, ANSWER_ZERO);
        with_children(&mut e, node, &[first, second]);
        let (p, _) = pick(&mut e, 0, 0, 1);
        e.mem.set_u32(node + 0xb0, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]);
        assert_eq!(calls_to(&e, PICK_GEOMETRY), vec![vec![1, 2, p, second]]);
        for index in [-1i32, 2] {
            e.mem.set_u32(node + 0xb0, index as u32);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x00e9_c680, &args![1u32, 2u32, p, node]).bool());
            assert!(calls_to(&e, PICK_GEOMETRY).is_empty());
        }
        // A finished find first answers true before the index is checked against the size.
        let (p, _) = pick(&mut e, 1, 1, 1);
        e.mem.set_u32(p + 0x28, 1);
        assert!(e.call(0x00e9_c680, &args![1u32, 2u32, p, node]).bool());
    }

    #[test]
    fn class_picks_add_a_record_in_front_of_the_ray() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(IS_KIND_OF, |_, a| {
            (a[0] == PICK_CLASS_RTTI_011F5F64).into_ret()
        });
        e.register(POINT_MINUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(NI_POINT3_DOT, |e, a| Ret {
            st0: (0..3)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i)))
                .sum(),
            ..Ret::default()
        });
        e.register(POINT_LENGTH, |e, a| Ret {
            st0: (0..3)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i).powi(2)))
                .sum::<f64>()
                .sqrt(),
            ..Ret::default()
        });
        e.register(POINT_TIMES_SCALAR, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * s;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_PLUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.map(0x0101_1000, 0x1000);
        let leaf = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ZERO);
        e.mem.set_f32(leaf + 0x8c + 8, 4.0);
        let origin = e.mem.alloc(12);
        let up = e.mem.alloc(12);
        e.mem.set_f32(up + 8, 1.0);
        let down = e.mem.alloc(12);
        e.mem.set_f32(down + 8, -1.0);
        // No class flag: nothing.
        let (p, r) = pick(&mut e, 0, 0, 1);
        assert!(!e.call(0x00e9_c490, &args![origin, up, p, leaf]).bool());
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 0);
        // The second class: a record 4 along the ray, with its point.
        e.mem.set_u8(PICK_CLASS_FLAG_0126FA64, 1);
        e.mem.set_u8(PICK_CLASS_FLAG_0126FA65, 1);
        assert!(!e.call(0x00e9_c490, &args![origin, up, p, leaf]).bool());
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 1);
        assert_eq!(e.mem.f32(r[0] + 0x14), 4.0);
        assert_eq!(e.mem.f32(r[0] + 0x10), 4.0);
        // Behind the ray: no record.
        assert!(!e.call(0x00e9_c490, &args![origin, down, p, leaf]).bool());
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 1);
        // The first class: the distance only.
        e.register(IS_KIND_OF, |_, a| {
            (a[0] == PICK_CLASS_RTTI_011F4A48).into_ret()
        });
        e.mem.set_u32(r[1] + 0x10, 0x1234);
        assert!(!e.call(0x00e9_c490, &args![origin, up, p, leaf]).bool());
        assert_eq!(e.mem.f32(r[1] + 0x14), 4.0);
        assert_eq!(e.mem.u32(r[1] + 0x10), 0x1234);
    }
}

#[cfg(test)]
mod n034b_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// Test functions for virtual slots.
    const COUNT_AT_48: u32 = 0x00ab_2000;
    const ANSWER_ONE: u32 = 0x00ab_2001;
    const INDICES_FROM_4C: u32 = 0x00ab_2002;
    const ANSWER_ZERO: u32 = 0x00ab_2003;
    const ANSWER_STRIPS_RTTI: u32 = 0x00ab_2004;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + 4 * i, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn scale_into(e: &mut Engine, a: &[u32], n: u32) -> Ret {
        let s = f32::from_bits(a[1]);
        for i in 0..n {
            let v = e.mem.f32(a[2] + 4 * i) * s;
            e.mem.set_f32(a[0] + 4 * i, v);
        }
        a[0].into_ret()
    }

    fn add_into(e: &mut Engine, a: &[u32], n: u32) -> Ret {
        for i in 0..n {
            let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
            e.mem.set_f32(a[1] + 4 * i, v);
        }
        a[1].into_ret()
    }

    fn setup(e: &mut Engine) {
        e.map(0x011f_4000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.register(COUNT_AT_48, |e, a| {
            u32::from(e.mem.u16(a[0] + 0x48)).into_ret()
        });
        e.register(ANSWER_ONE, |_, _| 1u32.into_ret());
        e.register(ANSWER_ZERO, |_, _| Ret::default());
        e.register(ANSWER_STRIPS_RTTI, |_, _| {
            TRI_STRIPS_RTTI_011F4A20.into_ret()
        });
        e.register(INDICES_FROM_4C, |e, a| {
            let list = e.mem.u32(a[0] + 0x4c);
            for k in 0..3u32 {
                let index = e.mem.u16(list + (a[1] * 3 + k) * 2);
                e.mem.set_u16(a[2 + k as usize], index);
            }
            Ret::default()
        });
        e.register(ELEMENT_SET, |_, _| Ret::default());
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(UNITIZE, |_, _| Ret::default());
        e.register(DATA_UNLOCK, |_, _| Ret::default());
        e.register(DATA_LOCK_TEXTURES, |_, _| Ret::default());
        e.register(DATA_LOCK_NORMALS, |_, _| Ret::default());
        e.register(DATA_LOCK_COLORS, |_, _| Ret::default());
        e.register(MATRIX_TIMES_POINT, |e, a| {
            copy_words(e, a[2], a[1], 3);
            a[1].into_ret()
        });
        e.register(POINT_CROSS, |e, a| {
            let (u, v) = (read(e, a[0], 3), read(e, a[2], 3));
            let c = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            for (i, x) in (0u32..).zip(c) {
                e.mem.set_f32(a[1] + 4 * i, x);
            }
            a[1].into_ret()
        });
        e.register(POINT2_TIMES_SCALAR, |e, a| scale_into(e, a, 2));
        e.register(SCALAR_TIMES_POINT, |e, a| scale_into(e, a, 3));
        e.register(COLOR_TIMES_SCALAR, |e, a| scale_into(e, a, 4));
        e.register(POINT2_PLUS_POINT2, |e, a| add_into(e, a, 2));
        e.register(POINT_PLUS_POINT, |e, a| add_into(e, a, 3));
        e.register(COLOR_PLUS_COLOR, |e, a| add_into(e, a, 4));
        // The defaults.
        for (i, v) in (0u32..).zip([1.0f32, 2.0, 3.0, 4.0]) {
            e.mem.set_f32(DEFAULT_COLOR_011A9BE0 + 4 * i, v);
        }
        e.mem.set_f32(DEFAULT_TEXTURE_011F4980, -1.0);
        e.mem.set_f32(DEFAULT_TEXTURE_011F4980 + 4, -2.0);
        e.mem.set_f32(DEFAULT_NORMAL_011F426C + 8, 9.0);
    }

    /// A pick (type, sort, intersect, coordinate) with a results pool of four ready records.
    fn pick(e: &mut Engine, settings: [u32; 4]) -> (u32, Vec<u32>) {
        let p = e.mem.alloc(0x40);
        for (i, v) in (0u32..).zip(settings) {
            e.mem.set_u32(p + 4 * i, v);
        }
        let array = e.mem.alloc(0x10);
        let mut records = vec![];
        for i in 0..4 {
            let r = e.mem.alloc(0x44);
            e.mem.set_u32(array + 4 * i, r);
            records.push(r);
        }
        e.mem.set_u32(p + 0x18 + 4, array);
        e.mem.set_u16(p + 0x18 + 0xa, 4);
        (p, records)
    }

    /// A geometry with an identity rotation, the translation and scale given, and its data
    /// (count at +0x48); answers (object, data).
    fn geometry(e: &mut Engine, translation: [f32; 3], scale: f32, count: u16) -> (u32, u32) {
        let object = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 8, ANSWER_ZERO);
        e.mem.set_u32(object, vtable);
        for i in [0u32, 4, 8] {
            e.mem.set_f32(object + 0x68 + i * 4, 1.0);
        }
        for (i, v) in (0u32..).zip(translation) {
            e.mem.set_f32(object + 0x8c + 4 * i, v);
        }
        e.mem.set_f32(object + 0x98, scale);
        let data = e.mem.alloc(0x50);
        let data_vtable = e.mem.alloc(0xb0);
        e.mem
            .set_u32(data_vtable + GET_ACTIVE_TRIANGLE_COUNT, COUNT_AT_48);
        e.mem
            .set_u32(data_vtable + GET_TRIANGLE_INDICES, INDICES_FROM_4C);
        e.mem.set_u32(data, data_vtable);
        e.mem.set_u16(data + 0x48, count);
        e.mem.set_u32(object + 0xb8, data);
        (object, data)
    }

    /// Vertices 0..4: (5,5,5), (0,0,0), (1,0,0), (0,1,0).
    fn vertices(e: &mut Engine) -> u32 {
        floats(
            e,
            &[5.0, 5.0, 5.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        )
    }

    /// The ray test: a hit when the first two corners are vertices 1 and 2 (x 0 and 1), point
    /// (1,1,1), distance 2, u 0.25, v 0.5.
    fn ray_hits_vertices_1_2(e: &mut Engine) {
        e.register(RAY_HITS_TRIANGLE, |e, a| {
            if e.mem.f32(a[2]) != 0.0 || e.mem.f32(a[3]) != 1.0 {
                return Ret::default();
            }
            for i in 0..3 {
                e.mem.set_f32(a[6] + 4 * i, 1.0);
            }
            e.mem.set_f32(a[7], 2.0);
            e.mem.set_f32(a[8], 0.25);
            e.mem.set_f32(a[9], 0.5);
            1u32.into_ret()
        });
    }

    #[test]
    fn strips_skip_degenerate_triangles_and_record_the_hit() {
        let mut e = Engine::new();
        setup(&mut e);
        ray_hits_vertices_1_2(&mut e);
        let (object, data) = geometry(&mut e, [0.0; 3], 2.0, 3);
        let verts = vertices(&mut e);
        e.mem.set_u32(data + 0x20, verts);
        // Strip 0 1 1 2 3: triangles 0 (0,1,1) and 1 (1,1,2) are skipped, 2 is (1,2,3).
        let strip = e.mem.alloc(10);
        for (i, v) in (0u32..).zip([0u16, 1, 1, 2, 3]) {
            e.mem.set_u16(strip + 2 * i, v);
        }
        e.register_double(GET_STRIP_LISTS, move |_, _| strip.into_ret());
        let textures = floats(&mut e, &[0.0, 0.0, 1.0, 10.0, 2.0, 20.0, 3.0, 30.0]);
        e.mem.set_u32(data + 0x2c, textures);
        let (p, records) = pick(&mut e, [0, 0, 1, 0]);
        e.mem.set_u8(p + 0x30, 1);
        e.mem.set_u8(p + 0x31, 1);
        let origin = floats(&mut e, &[2.0, 4.0, 6.0]);
        let direction = floats(&mut e, &[0.0, 0.0, 2.0]);
        e.call_log = Some(vec![]);
        let hit = e
            .call(0x00e9_93d0, &args![origin, direction, p, object])
            .bool();
        assert!(hit);
        let rays = calls_to(&e, RAY_HITS_TRIANGLE);
        assert_eq!(rays.len(), 1);
        assert_eq!(read(&e, rays[0][0], 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(read(&e, rays[0][1], 3), vec![0.0, 0.0, 1.0]);
        let r = records[0];
        assert_eq!(read(&e, r + 8, 4), vec![1.0, 1.0, 1.0, 2.0]);
        assert_eq!(e.mem.u16(r + 0x18), 2);
        assert_eq!([0x1a, 0x1c, 0x1e].map(|o| e.mem.u16(r + o)), [1, 2, 3]);
        // 0.25 * t1 + 0.25 * t2 + 0.5 * t3, through the calls.
        assert_eq!(read(&e, r + 0x20, 2), vec![2.25, 22.5]);
        assert_eq!(calls_to(&e, POINT2_TIMES_SCALAR).len(), 3);
        // The face normal through 004b3800.
        assert_eq!(read(&e, r + 0x28, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(read(&e, r + 0x34, 4), vec![1.0, 2.0, 3.0, 4.0]);
        assert!(calls_to(&e, DATA_UNLOCK).is_empty());

        // No strip lists: the distance to the translation.
        e.register(GET_STRIP_LISTS, |_, _| Ret::default());
        e.mem.set_f32(object + 0x8c, 3.0);
        e.mem.set_f32(object + 0x90, 4.0);
        let zero = floats(&mut e, &[0.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_93d0, &args![zero, direction, p, object])
            .bool());
        assert!(calls_to(&e, RAY_HITS_TRIANGLE).is_empty());
        assert_eq!(e.mem.f32(records[1] + 0x14), 5.0);
    }

    #[test]
    fn shapes_use_the_locks_and_stop_at_the_first_hit() {
        let mut e = Engine::new();
        setup(&mut e);
        ray_hits_vertices_1_2(&mut e);
        let (object, data) = geometry(&mut e, [1.0, 2.0, 3.0], 2.0, 2);
        // No vertex array: an additional object that allows the lock.
        let additional = e.mem.alloc(8);
        let additional_vtable = e.mem.alloc(0x90);
        e.mem
            .set_u32(additional_vtable + ADDITIONAL_DATA_LOCKABLE, ANSWER_ONE);
        e.mem.set_u32(additional, additional_vtable);
        e.mem.set_u32(data + 0x30, additional);
        let verts = vertices(&mut e);
        e.mem.set_u32(data + 0x4c, verts);
        e.register(DATA_LOCK, |_, _| 1u32.into_ret());
        e.register(DATA_LOCK_VERTICES, |e, a| {
            let verts = e.mem.u32(a[0] + 0x4c);
            e.mem.set_u32(a[1], verts);
            e.mem.set_u32(a[1] + 4, 12);
            Ret::default()
        });
        let list = e.mem.alloc(12);
        for (i, v) in (0u32..).zip([1u16, 2, 3, 1, 2, 3]) {
            e.mem.set_u16(list + 2 * i, v);
        }
        e.register_double(GET_TRI_LIST, move |_, _| list.into_ret());
        let textures = floats(&mut e, &[0.0, 0.0, 1.0, 10.0, 2.0, 20.0, 3.0, 30.0]);
        e.mem.set_u32(data + 0x2c, textures);
        e.register(DATA_LOCK_TEXTURES, |e, a| {
            let textures = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[2], textures);
            e.mem.set_u32(a[2] + 4, 8);
            Ret::default()
        });
        e.register(DATA_LOCK_COLORS, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            Ret::default()
        });
        e.register(LOCKED_COLOR_AT, |e, a| {
            let i = a[1] as f32;
            for (k, v) in (0u32..).zip([i, i, i, 1.0]) {
                e.mem.set_f32(a[2] + 4 * k, v);
            }
            Ret::default()
        });
        let (p, records) = pick(&mut e, [1, 1, 1, 1]);
        for flag in [0x30, 0x31, 0x33] {
            e.mem.set_u8(p + flag, 1);
        }
        let origin = floats(&mut e, &[0.0, 0.0, 0.0]);
        let direction = floats(&mut e, &[0.0, 0.0, 1.0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_a280, &args![origin, direction, p, object])
            .bool());
        // Stopped at the first of the two triangles; the lock released.
        assert_eq!(calls_to(&e, RAY_HITS_TRIANGLE).len(), 1);
        assert_eq!(calls_to(&e, DATA_UNLOCK), vec![vec![data]]);
        assert_eq!(calls_to(&e, DATA_LOCK), vec![vec![data, 1]]);
        let r = records[0];
        // (1,1,1) * 2 + (1,2,3).
        assert_eq!(read(&e, r + 8, 4), vec![3.0, 4.0, 5.0, 2.0]);
        // Weights 0.25 (1), 0.25 (2), 0.5 (3), inline.
        assert_eq!(read(&e, r + 0x20, 2), vec![2.25, 22.5]);
        assert!(calls_to(&e, POINT2_TIMES_SCALAR).is_empty());
        // The inline face normal, rotated by the identity.
        assert_eq!(read(&e, r + 0x28, 3), vec![0.0, 0.0, 1.0]);
        assert!(calls_to(&e, POINT_CROSS).is_empty());
        assert_eq!(read(&e, r + 0x34, 4), vec![2.25, 2.25, 2.25, 1.0]);
        let colors: Vec<u32> = calls_to(&e, LOCKED_COLOR_AT).iter().map(|a| a[1]).collect();
        assert_eq!(colors, vec![1, 2, 3]);
    }

    #[test]
    fn center_pick_records_what_is_in_front() {
        let mut e = Engine::new();
        setup(&mut e);
        let (object, _) = geometry(&mut e, [0.0; 3], 1.0, 0);
        // No center pointer: the words at 011f4288.
        for (i, v) in (0u32..).zip([0.0f32, 0.0, 5.0, 1.0]) {
            e.mem.set_f32(DEFAULT_BOUND_011F4288 + 4 * i, v);
        }
        let origin = floats(&mut e, &[0.0, 0.0, 1.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        let (p, records) = pick(&mut e, [0, 0, 0, 0]);
        assert!(e.call(0x00e9_b4f0, &args![origin, up, p, object]).bool());
        assert_eq!(e.mem.f32(records[0] + 0x14), 4.0);
        assert_eq!(e.mem.f32(records[0] + 8), 0.0);
        assert!(!e.call(0x00e9_b4f0, &args![origin, down, p, object]).bool());
        // A skinned geometry with intersect type 1 also gets the point.
        e.mem.set_u32(p + 8, 1);
        e.mem.set_u32(object + 0xbc, 1);
        let center = floats(&mut e, &[0.0, 0.0, 3.0, 1.0]);
        e.mem.set_u32(object + 0x20, center);
        assert!(e.call(0x00e9_b4f0, &args![origin, up, p, object]).bool());
        assert_eq!(read(&e, records[1] + 8, 4), vec![0.0, 0.0, 3.0, 2.0]);
    }

    #[test]
    fn tri_based_geometry_dispatches_and_picks_its_triangles() {
        let mut e = Engine::new();
        setup(&mut e);
        // Single-strip tri-strips go to fn_00e993d0 (here without strip lists).
        let (object, data) = geometry(&mut e, [3.0, 4.0, 0.0], 1.0, 1);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(vtable + 8, ANSWER_STRIPS_RTTI);
        e.mem.set_u16(data + 0x44, 1);
        let verts = vertices(&mut e);
        e.mem.set_u32(data + 0x20, verts);
        e.register(GET_STRIP_LISTS, |_, _| Ret::default());
        e.register(IS_KIND_OF, |_, _| 1u32.into_ret());
        e.register(GET_TRI_LIST, |_, _| Ret::default());
        let origin = floats(&mut e, &[0.0, 0.0, 0.0]);
        let direction = floats(&mut e, &[0.0, 0.0, 1.0]);
        let (p, records) = pick(&mut e, [0, 0, 1, 0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_b4f0, &args![origin, direction, p, object])
            .bool());
        assert_eq!(calls_to(&e, GET_STRIP_LISTS).len(), 1);
        assert!(calls_to(&e, IS_KIND_OF).is_empty());
        assert_eq!(e.mem.f32(records[0] + 0x14), 5.0);
        // A tri-shape goes to fn_00e9a280.
        e.mem.set_u32(vtable + 8, ANSWER_ZERO);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_b4f0, &args![origin, direction, p, object])
            .bool());
        assert_eq!(calls_to(&e, GET_TRI_LIST).len(), 1);
        // Any other: the triangles from GetTriangleIndices (1,2,3 twice).
        e.register(IS_KIND_OF, |_, _| Ret::default());
        e.register(POINT_TIMES_MATRIX, |e, a| {
            copy_words(e, a[1], a[0], 3);
            a[0].into_ret()
        });
        ray_hits_vertices_1_2(&mut e);
        let list = e.mem.alloc(12);
        for (i, v) in (0u32..).zip([1u16, 2, 3, 1, 2, 3]) {
            e.mem.set_u16(list + 2 * i, v);
        }
        e.mem.set_u32(data + 0x4c, list);
        e.mem.set_u16(data + 0x48, 2);
        let (p, records) = pick(&mut e, [0, 0, 1, 0]);
        e.mem.set_u8(p + 0x33, 1);
        e.mem.set_u32(records[1] + 0x34, 0x5555);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_b4f0, &args![origin, direction, p, object])
            .bool());
        let rays = calls_to(&e, RAY_HITS_TRIANGLE);
        assert_eq!(rays.len(), 2);
        // The model origin: origin - translation, times 1 / scale.
        assert_eq!(read(&e, rays[0][0], 3), vec![-3.0, -4.0, 0.0]);
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 2);
        let r = records[1];
        assert_eq!(e.mem.f32(r + 0x14), 2.0);
        assert_eq!(e.mem.u16(r + 0x18), 1);
        assert_eq!(read(&e, r + 0x20, 2), vec![-1.0, -2.0]);
        assert_eq!(read(&e, r + 0x28, 3), vec![0.0, 0.0, 9.0]);
        // Colors asked for, but neither locked nor in the data: left as they were.
        assert_eq!(e.mem.u32(r + 0x34), 0x5555);
    }
}
