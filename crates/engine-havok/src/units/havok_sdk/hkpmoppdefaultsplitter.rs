//! `Havok SDK/hkpmoppdefaultsplitter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppDefaultSplitter` (Xbox PDB) splits the primitives of a mediator into a binary tree
//! for the MOPP code generator. The object (`this`) holds: the settings of the compiler at
//! `+8`, a sorted queue of nodes at `+0xC` (count `+0`, array `+8`, last removed node `+0xC`),
//! two free lists of nodes (head `+0x10` and count `+0x14` for the 0x7C-byte tree nodes, head
//! `+0x18` and count `+0x1C` for the 0x64-byte leaf nodes), the settings at `+0x20`, the
//! assembler at `+0x24`, the mediator at `+0x28`, the last created root at `+0x2C`, the number
//! of splitting directions at `+0x30`, their array (32 bytes each) at `+0x34` and the bit
//! length of the primitive count at `+0x38`. A primitive is 20 bytes. The member names are
//! guesses and not confirmed in the exe, except the ones of the Xbox PDB.
//!
//! Several functions of the exe use their own stack arguments as scratch memory (for
//! instance the property buffer of `00e098b0`, which is the address of its argument slots);
//! here such buffers are separate blocks.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `easyAlloc` (`00c85690`, cdecl `(allocator, size)`) and `easyFree` (`00c856c0`, cdecl
/// `(allocator, block)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// The quick sort of primitives (`00e0ab60`, cdecl `(primitives, low, high, flag)`, the flag
/// is only handed on to the recursion).
const SORT_PRIMITIVES: u32 = 0x00e0_ab60;
/// `hkMath::hkFloatToInt` (`00cbfef0`, cdecl, one float): the integer for a float.
const FLOAT_TO_INT: u32 = 0x00cb_fef0;
/// The vtable of the splitter (`010ea610`) and the base one (`0102e388`).
const VTABLE_SPLITTER: u32 = 0x010e_a610;
const VTABLE_BASE: u32 = 0x0102_e388;
/// The double constants of the exe: 0.66 (`01090d68`), -0.03 (`010ea5f8`), 5.0 (`01020998`),
/// 0.5 (`01011588`), 16.0 (`010240b8`), 0.05 (`0103c7c8`), 0.75 (`0101de30`), 6.0
/// (`01034208`), 3.0 (`01021928`), 2.9 (`010ea600`) and 1500.0 (`010737c8`), by bit pattern.
const WEIGHT: f64 = 0.6600000262260437;
const NEGATIVE_DEPTH_FACTOR: f64 = -0.029999999329447746;
const BALANCE_FACTOR: f64 = 5.0;
const HALF: f64 = 0.5;
const SIXTEEN: f64 = 16.0;
const FALLOFF: f64 = 0.05000000074505806;
const THREE_QUARTERS: f64 = 0.75;
const SIX: f64 = 6.0;
const THREE: f64 = 3.0;
const TWO_NINE: f64 = 2.9000000953674316;
const FIFTEEN_HUNDRED: f64 = 1500.0;
/// The float constants: 1e6 (`0108cb00`), 1e10 (`010ea628`), 1e7 (`010d51b0`) and 0.001
/// (`01017d00`) and the huge cost (`010ea608`), by bit pattern.
const NEW_NODE_COST: u32 = 0x4974_2400;
const NEW_NODE_BOUND: u32 = 0x5015_02f9;
const SEARCH_START_COST: u32 = 0x4b18_9680;
const MIN_WIDTH: u32 = 0x3a83_126f;
const HUGE_COST: u32 = 0x7dcc_ccbf;

/// The difference of two floats as the x87 computes it, rounded to `f32` at the store.
fn sub_f32(a: f32, b: f32) -> f32 {
    (f64::from(a) - f64::from(b)) as f32
}

/// The product of a float and a double constant, rounded to `f32` at the store.
fn mul_f32(a: f32, factor: f64) -> f32 {
    (f64::from(a) * factor) as f32
}

/// The signed division by 20 the exe does with a multiplication by `0x66666667`.
fn div20(bytes: u32) -> i32 {
    (bytes as i32) / 20
}

/// The five words of the 20-byte record at `at`.
fn read_record(e: &Engine, at: u32) -> [u32; 5] {
    let mut record = [0u32; 5];
    for (i, slot) in record.iter_mut().enumerate() {
        *slot = e.mem.u32(at + 4 * i as u32);
    }
    record
}

fn write_record(e: &mut Engine, at: u32, record: [u32; 5]) {
    for (i, value) in record.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *value);
    }
}

/// Copies the record at `from` to `to`, word by word.
fn copy_record(e: &mut Engine, to: u32, from: u32) {
    for i in 0..5 {
        let value = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, value);
    }
}

/// Adds `delta` to the word at `at`.
fn add_word(e: &mut Engine, at: u32, delta: u32) {
    let value = e.mem.u32(at).wrapping_add(delta);
    e.mem.set_u32(at, value);
}

// Translated from 00e09670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts `node` into the sorted queue (`this`: count at `+0`, array of node pointers at
/// `+8`, the key of a node is the float at `+0xC`), `RET 4`: walks down from the last entry
/// as long as there is a node whose key is below the key of the new one (the x87 compare
/// leaves a NaN key where it stops), moving every one of them up by one place.
pub fn fn_00e09670(e: &mut Engine, this: Ptr, node: Ptr) {
    let me = this.addr();
    let node = node.addr();
    let key = e.mem.f32(node + 0xc);
    let items = e.mem.u32(me + 8);
    let mut index = (e.mem.u32(me) as i32).wrapping_sub(1);
    while index >= 0 {
        let at = items.wrapping_add((index as u32).wrapping_mul(4));
        let entry = e.mem.u32(at);
        if entry == 0 || !(e.mem.f32(entry + 0xc) < key) {
            break;
        }
        e.mem.set_u32(at.wrapping_add(4), entry);
        index -= 1;
    }
    let at = items.wrapping_add((index.wrapping_add(1) as u32).wrapping_mul(4));
    e.mem.set_u32(at, node);
    add_word(e, me, 1);
}

// Translated from 00e097a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the nodes off the end of the sorted queue, `RET 4` with a float: while the last
/// node exists and its key is below `key`, it is stored as the last removed node (`+0xC`),
/// cleared in the array and the count is decremented.
pub fn fn_00e097a0(e: &mut Engine, this: Ptr, key: f32) {
    let me = this.addr();
    let items = e.mem.u32(me + 8);
    let mut index = (e.mem.u32(me) as i32).wrapping_sub(1);
    while index >= 0 {
        let at = items.wrapping_add((index as u32).wrapping_mul(4));
        let entry = e.mem.u32(at);
        if entry == 0 || !(e.mem.f32(entry + 0xc) < key) {
            break;
        }
        e.mem.set_u32(me + 0xc, entry);
        e.mem.set_u32(at, 0);
        add_word(e, me, 0xffff_ffff);
        index -= 1;
    }
}

// Translated from 00e098b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 8` with `(items, output)`; `items` holds the array of primitives at `+0`, their
/// count at `+4`. Stores in `output` the count at `+8`, the smallest and largest first word
/// of the primitives (unsigned) at `+0x24` and `+0x28`, and, from the properties the mediator
/// at `this + 0x28` returns for each primitive (slot `0x24`, which also fills a buffer of
/// words): the largest number of properties at `+0x2C` and the smallest and largest value
/// of every property index as two arrays at `+0x30` and `+0x34` (starting 0xFFFFFFFF and 0).
/// The exe hands the mediator the address of its own argument slots as the buffer; here it
/// is a block of 64 words.
pub fn fn_00e098b0(e: &mut Engine, this: Ptr, items: Ptr, output: Ptr) {
    let me = this.addr();
    let items = items.addr();
    let output = output.addr();
    let first = e.mem.u32(items);
    let count = e.mem.u32(items + 4);
    e.mem.set_u32(output + 0x2c, 0);
    e.mem.set_u32(output + 0x30, 0xffff_ffff);
    e.mem.set_u32(output + 0x34, 0);
    let start = e.mem.u32(first);
    let (mut low, mut high) = (start, start);
    e.with_stack(0x100, |e, buffer| {
        let buffer = buffer.addr();
        let mut primitive = first;
        for _ in 0..(count as i32).max(0) {
            let value = e.mem.u32(primitive);
            high = high.max(value);
            low = low.min(value);
            let mediator = e.mem.u32(me + 0x28);
            let properties = e.vcall(mediator, 0x24, &args![primitive, buffer]).u32() as i32;
            if properties > e.mem.u32(output + 0x2c) as i32 {
                e.mem.set_u32(output + 0x2c, properties as u32);
            }
            for index in 0..properties.max(0) as u32 {
                let value = e.mem.u32(buffer + 4 * index);
                let minimum = output + 0x30 + 4 * index;
                if value < e.mem.u32(minimum) {
                    e.mem.set_u32(minimum, value);
                }
                let maximum = output + 0x34 + 4 * index;
                if value > e.mem.u32(maximum) {
                    e.mem.set_u32(maximum, value);
                }
            }
            primitive = primitive.wrapping_add(0x14);
        }
    });
    e.mem.set_u32(output + 0x24, low);
    e.mem.set_u32(output + 0x28, high);
    e.mem.set_u32(output + 8, count);
}

/// The weight of one more axis for `00e09970`: `0.05 * f32(16 * h^3)` with
/// `h = f32(f32(top - extent) / top)`, every step as the x87 does it.
fn axis_weight(top: f32, extent: f32) -> f32 {
    let difference = sub_f32(top, extent);
    let ratio = (f64::from(difference) / f64::from(top)) as f32;
    let square = f64::from(ratio) * f64::from(ratio);
    let cube = f64::from(ratio) * square;
    let scaled = (SIXTEEN * cube) as f32;
    (FALLOFF * f64::from(scaled)) as f32
}

// Translated from 00e09970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 0xC` with `(node, axes, values)`: ranks the three axes of the box of `node` (the
/// float pairs at `+0xC`, `+0x14`, `+0x1C`) by their extent, an extent being reduced by 0.66
/// when the word at `node + 0x60` is the axis descriptor `this[0x34] + 0x20 * axis`. With no
/// node the axes are 0, 1, 2 and the values 0. Otherwise `axes` gets the widest axis, then
/// the other two by decreasing extent, and `values` 0 followed by the falloff weights of the
/// two others, `0.05 * 16 * ((top - extent) / top)^3`.
pub fn fn_00e09970(e: &mut Engine, this: Ptr, node: Ptr, axes: Ptr, values: Ptr) {
    let me = this.addr();
    let node = node.addr();
    let axes = axes.addr();
    let values = values.addr();
    if node == 0 {
        for i in 0..3 {
            e.mem.set_u32(axes + 4 * i, i);
            e.mem.set_f32(values + 4 * i, 0.0);
        }
        return;
    }
    let reference = e.mem.u32(me + 0x34);
    let tag = e.mem.u32(node + 0x60);
    let mut extents = [0.0f32; 3];
    for (axis, extent) in extents.iter_mut().enumerate() {
        let at = node + 0xc + 8 * axis as u32;
        let mut width = sub_f32(e.mem.f32(at + 4), e.mem.f32(at));
        if tag == reference.wrapping_add(0x20 * axis as u32) {
            width = mul_f32(width, WEIGHT);
        }
        *extent = width;
    }
    let mut top = 0.0f32;
    let mut widest = 0usize;
    if extents[0] > 0.0 {
        top = extents[0];
    }
    if top < extents[1] {
        widest = 1;
        top = extents[1];
    }
    if extents[2] > top {
        widest = 2;
        top = extents[2];
    }
    let first = (widest + 1) % 3;
    let second = (widest + 2) % 3;
    e.mem.set_u32(axes, widest as u32);
    e.mem.set_f32(values, 0.0);
    let (near, far) = if extents[second] > extents[first] {
        (second, first)
    } else {
        (first, second)
    };
    e.mem.set_u32(axes + 4, near as u32);
    e.mem.set_f32(values + 4, axis_weight(top, extents[near]));
    e.mem.set_u32(axes + 8, far as u32);
    e.mem.set_f32(values + 8, axis_weight(top, extents[far]));
}

// Translated from 00e09ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultSplitter::hkpMoppDefaultSplitter` (Xbox PDB), `__fastcall`: sets the 16-bit
/// word 1 at `+6`, the vtable `010ea610` and clears the words at `+8`, `+0xC` and the free
/// lists at `+0x10` to `+0x1C`. Returns `this`.
pub fn hkp_mopp_default_splitter_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE_SPLITTER);
    for offset in [0x10, 0x14, 0x18, 0x1c, 0x8, 0xc] {
        e.mem.set_u32(me + offset, 0);
    }
    this
}

// Translated from 00e09bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the splitter (the engine map has no name), `__fastcall`: only sets the
/// base vtable.
pub fn fn_00e09bd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BASE);
}

// Translated from 00e09be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 0x14` with `(best, axis, items, range, depth)`: looks for the best place to cut the
/// primitives of `items` (array at `+0`, count `+4`, limit `+8`) along the axis descriptor
/// `axis`, whose primitives carry their interval as the floats at `+8` and `+0xC`, within the
/// bounds `range` (two floats). The primitives are fed one by one into the sorted queue
/// (`00e09670`), cut down to the limit (`00e097a0`), and for every candidate position a cost
/// is made of four terms (`00e0a850`, `00e0a6c0`, a balance term from the position and a term
/// of the distance of the interval keys); the cheapest overall is stored in `best`: the
/// descriptor at `+0x60`, the two keys at `+0x64` and `+0x68` and the cost at `+0x6C`. The
/// queue is emptied at the end.
pub fn fn_00e09be0(
    e: &mut Engine,
    this: Ptr,
    best: Ptr,
    axis: Ptr,
    items: Ptr,
    range: Ptr,
    depth: u32,
) {
    let me = this.addr();
    let best = best.addr();
    let axis = axis.addr();
    let items = items.addr();
    let range = range.addr();
    e.with_stack(0x80, |e, frame| {
        // The state record the cost functions read, in the layout of the exe's frame.
        let s = frame.addr();
        let count = e.mem.u32(items + 4) as i32;
        e.mem.set_u32(s, depth);
        let directions = e.mem.u32(me + 0x38);
        e.mem.set_u32(s + 4, directions);
        let range_low = x87_quiet(e.mem.f32(range));
        e.mem.set_f32(s + 8, range_low);
        let range_high = x87_quiet(e.mem.f32(range + 4));
        e.mem.set_f32(s + 0xc, range_high);
        e.mem.set_u32(s + 0x18, count as u32);
        e.mem.set_u32(s + 0x24, axis);
        let inverse_count = (1.0f64 / f64::from(count)) as f32;
        e.mem.set_f32(s + 0x1c, inverse_count);
        let first_key = e.mem.u32(e.mem.u32(items));
        let (mut low, mut high) = (first_key, first_key);
        e.mem.set_u32(s + 0x28, low);
        e.mem.set_u32(s + 0x2c, high);
        let width = sub_f32(e.mem.f32(range + 4), e.mem.f32(range));
        let inverse_width = if width < f32::from_bits(MIN_WIDTH) {
            1.0
        } else {
            (1.0f64 / f64::from(width)) as f32
        };
        e.mem.set_f32(s + 0x10, inverse_width);
        let manager = e.mem.u32(me + 8);
        let limit = (e.mem.u32(items + 8) as i32).min(e.mem.u32(manager + 8) as i32);
        let mut index = 0i32;
        let mut best_position = 0i32;
        while index < e.mem.u32(items + 4) as i32 {
            let element = e
                .mem
                .u32(items)
                .wrapping_add((index as u32).wrapping_mul(0x14));
            let key = x87_quiet(e.mem.f32(element + 8));
            e.mem.set_f32(s + 0x20, key);
            e.mem.set_u32(s + 0x14, index as u32);
            let value = e.mem.u32(element);
            low = low.min(value);
            high = high.max(value);
            e.mem.set_u32(s + 0x28, low);
            e.mem.set_u32(s + 0x2c, high);
            let queue = e.mem.u32(me + 0xc);
            fn_00e097a0(e, Ptr::new(queue), key);
            let mut slot = (e.mem.u32(queue) as i32).wrapping_sub(1);
            while slot > limit {
                let at = e
                    .mem
                    .u32(queue + 8)
                    .wrapping_add((slot as u32).wrapping_mul(4));
                let entry = e.mem.u32(at);
                e.mem.set_u32(queue + 0xc, entry);
                e.mem.set_u32(at, 0);
                add_word(e, queue, 0xffff_ffff);
                slot -= 1;
            }
            let manager = e.mem.u32(me + 8);
            let manager_c = e.mem.u32(manager + 0xc) as i32;
            let settings = e.mem.u32(me + 0x20);
            let manager_10 = e.mem.u32(manager + 0x10) as i32;
            let node_cost = fn_00e0a750(e, Ptr::new(settings), Ptr::new(s));
            let span = e.mem.u32(s + 0x2c).wrapping_sub(e.mem.u32(s + 0x28)) as i32;
            let axis_cost = x87_quiet(e.mem.f32(axis + 0x10));
            let depth_term = if span < 0x10 {
                mul_f32(e.mem.f32(settings + 0x18), NEGATIVE_DEPTH_FACTOR)
            } else {
                0.0
            };
            let base_cost =
                ((f64::from(axis_cost) + f64::from(node_cost)) + f64::from(depth_term)) as f32;
            let half = manager_c >> 1;
            let queue_count = e.mem.u32(queue) as i32;
            let mut position = 0i32;
            let mut lower = 0i32;
            let mut best_cost = f32::from_bits(SEARCH_START_COST);
            let mut first_node = None;
            let mut search = true;
            if queue_count == 0 {
                let node = e.mem.u32(queue + 0xc);
                if node == 0 {
                    search = false;
                } else {
                    first_node = Some(node);
                }
            } else if e.mem.f32(best + 0x6c) < base_cost {
                search = false;
            } else {
                let upper = if manager_10 > 0 {
                    lower = (best_position - half).max(0);
                    (half + best_position).min(queue_count)
                } else {
                    queue_count
                };
                position = upper - 1;
            }
            if search {
                loop {
                    if position < lower {
                        best_position -= 1;
                        break;
                    }
                    let node = match first_node.take() {
                        Some(node) => node,
                        None => {
                            if position > e.mem.u32(queue) as i32 {
                                0
                            } else {
                                let array = e.mem.u32(queue + 8);
                                e.mem
                                    .u32(array.wrapping_add((position as u32).wrapping_mul(4)))
                            }
                        }
                    };
                    let node_key = x87_quiet(e.mem.f32(node + 0xc));
                    e.mem.set_f32(s + 0x38, node_key);
                    let remaining = e
                        .mem
                        .u32(items + 4)
                        .wrapping_sub(index as u32)
                        .wrapping_add(position as u32);
                    e.mem.set_u32(s + 0x30, position as u32);
                    e.mem.set_u32(s + 0x34, remaining);
                    let split_cost = fn_00e0a850(e, Ptr::new(settings), Ptr::new(s));
                    let size_cost = fn_00e0a6c0(e, Ptr::new(settings), Ptr::new(s));
                    let ratio = (f64::from(position) * f64::from(inverse_count)) as f32;
                    let balance = ((f64::from(ratio) * BALANCE_FACTOR)
                        * f64::from(e.mem.f32(settings + 8)))
                        as f32;
                    let distance = sub_f32(node_key, key);
                    let distance = (f64::from(distance) * f64::from(inverse_width)) as f32;
                    let distance =
                        (f64::from(distance) * f64::from(e.mem.f32(settings + 0xc))) as f32;
                    let sum = (((f64::from(size_cost) + f64::from(split_cost))
                        + f64::from(balance))
                        + f64::from(distance)) as f32;
                    let cost = (f64::from(sum) + f64::from(base_cost)) as f32;
                    if best_cost > cost {
                        best_cost = cost;
                        best_position = position;
                        if e.mem.f32(best + 0x6c) > cost {
                            e.mem.set_f32(best + 0x6c, cost);
                            e.mem.set_f32(best + 0x64, key);
                            e.mem.set_u32(best + 0x60, axis);
                            e.mem.set_f32(best + 0x68, node_key);
                            let candidate = position - half;
                            if candidate < lower {
                                lower = candidate.max(0);
                            }
                        }
                    }
                    position -= 1;
                }
            }
            let queue = e.mem.u32(me + 0xc);
            let element = e
                .mem
                .u32(items)
                .wrapping_add((index as u32).wrapping_mul(0x14));
            fn_00e09670(e, Ptr::new(queue), Ptr::new(element));
            index += 1;
        }
        let queue = e.mem.u32(me + 0xc);
        let mut slot = (e.mem.u32(queue) as i32).wrapping_sub(1);
        while slot > 0 {
            let at = e
                .mem
                .u32(queue + 8)
                .wrapping_add((slot as u32).wrapping_mul(4));
            e.mem.set_u32(at, 0);
            slot -= 1;
        }
        e.mem.set_u32(queue, 0);
        e.mem.set_u32(queue + 0xc, 0);
    });
}

// Translated from 00e09f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a leaf node, `RET 8` with `(parent, items)`: takes a node from the second free list
/// (`+0x18`, count `+0x1C`), clears its flags (bytes `+0x38` to `+0x3D`, `+4` set to 1,
/// `+0x40` and `+0x5C` to -1), stores the parent at `+0` and the first primitive at `+0x60`,
/// runs `00e098b0` on the primitives and gets the extents of the three axes (descriptors
/// `this[0x34] + 0x20 * axis`) from the mediator (slot `0x1C`) into the float pairs at `+0xC`,
/// `+0x14` and `+0x1C`. Returns the node.
pub fn fn_00e09f80(e: &mut Engine, this: Ptr, parent: Ptr, items: Ptr) -> Ptr {
    let me = this.addr();
    let items_at = items.addr();
    let node = e.mem.u32(me + 0x18);
    let next = e.mem.u32(node);
    e.mem.set_u32(me + 0x18, next);
    add_word(e, me + 0x1c, 0xffff_ffff);
    e.mem.set_u8(node + 0x38, 0);
    e.mem.set_u8(node + 0x39, 0);
    e.mem.set_u32(node + 0x40, 0xffff_ffff);
    e.mem.set_u32(node + 0x5c, 0xffff_ffff);
    for offset in 0x3a..=0x3d {
        e.mem.set_u8(node + offset, 0);
    }
    e.mem.set_u8(node + 4, 1);
    e.mem.set_u32(node, parent.addr());
    let first = e.mem.u32(items_at);
    e.mem.set_u32(node + 0x60, first);
    fn_00e098b0(e, this, items, Ptr::new(node));
    for axis in 0..3u32 {
        let mediator = e.mem.u32(me + 0x28);
        let descriptor = e.mem.u32(me + 0x34).wrapping_add(0x20 * axis);
        let primitives = e.mem.u32(items_at);
        let count = e.mem.u32(items_at + 4);
        let pair = node + 0xc + 8 * axis;
        e.vcall(
            mediator,
            0x1c,
            &args![descriptor, axis, primitives, count, pair, pair + 4],
        );
    }
    Ptr::new(node)
}

// Translated from 00e0a020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a node off the first free list (head `+0x10`, count `+0x14`) and clears it, the
/// engine map has no name: bytes `+0x38` to `+0x3D` and `+4` to 0, `+0x40` and `+0x5C` to -1,
/// the float `+0x6C` to 1e6, `+0x64` to 1e10 and the words `+0x70` and `+0x74` to 0. Returns
/// the node.
pub fn fn_00e0a020(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    let node = e.mem.u32(me + 0x10);
    let next = e.mem.u32(node);
    e.mem.set_u32(me + 0x10, next);
    add_word(e, me + 0x14, 0xffff_ffff);
    for offset in 0x38..=0x3d {
        e.mem.set_u8(node + offset, 0);
    }
    e.mem.set_u32(node + 0x40, 0xffff_ffff);
    e.mem.set_u32(node + 0x5c, 0xffff_ffff);
    e.mem.set_u8(node + 4, 0);
    e.mem
        .set_f32(node + 0x6c, x87_quiet(f32::from_bits(NEW_NODE_COST)));
    e.mem.set_u32(node + 0x70, 0);
    e.mem
        .set_f32(node + 0x64, x87_quiet(f32::from_bits(NEW_NODE_BOUND)));
    e.mem.set_u32(node + 0x74, 0);
    Ptr::new(node)
}

// Translated from 00e0a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultSplitter::releaseNode` (Xbox PDB), `RET 4`: puts `node` back on a free list,
/// the second one (`+0x18`, count `+0x1C`) when its byte at `+4` is set and the first one
/// (`+0x10`, count `+0x14`) otherwise. A null node is ignored.
pub fn hkp_mopp_default_splitter_release_node(e: &mut Engine, this: Ptr, node: Ptr) {
    let me = this.addr();
    let node = node.addr();
    if node == 0 {
        return;
    }
    let (head, count) = if e.mem.u8(node + 4) != 0 {
        (me + 0x18, me + 0x1c)
    } else {
        (me + 0x10, me + 0x14)
    };
    let next = e.mem.u32(head);
    e.mem.set_u32(node, next);
    add_word(e, count, 1);
    e.mem.set_u32(head, node);
}

// Translated from 00e0a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultSplitter::getFreeNodes` (Xbox PDB), `__fastcall`: the counts of the two free
/// lists added.
pub fn hkp_mopp_default_splitter_get_free_nodes(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    e.mem.u32(me + 0x1c).wrapping_add(e.mem.u32(me + 0x14))
}

// Translated from 00e0a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 0x14` with `(items, node, depth, left, right)`: splits the primitives of `items`
/// (array `+0`, count `+4`, second count `+8`) around the cut stored in `node` (interval keys
/// at `+0x64` and `+0x68`). Every primitive of the first group is classified by how its
/// interval (floats at `+8` and `+0xC`) lies against the cut and moved by `00e0a950`; then
/// `00e0acb0` balances the groups. `left` and `right` get `(first primitive, count, second
/// count)` of the two halves, the second count of the left one being the second count scaled
/// by the share of the first group (through `hkFloatToInt`), the right one gets the rest;
/// the primitives of the right half are moved to follow the left ones.
pub fn fn_00e0a0b0(
    e: &mut Engine,
    this: Ptr,
    items: Ptr,
    node: Ptr,
    depth: u32,
    left: Ptr,
    right: Ptr,
) {
    let items = items.addr();
    let node_at = node.addr();
    let left = left.addr();
    let right = right.addr();
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        // The cursors and counters the exe keeps in its frame and hands on by address.
        let (cursor_a, cursor_b, cursor_c, cursor_d) = (f, f + 4, f + 8, f + 0xc);
        let (second_count, moved, base, end, d_copy) =
            (f + 0x10, f + 0x14, f + 0x18, f + 0x1c, f + 0x20);
        let start = e.mem.u32(items);
        let first_count = e.mem.u32(items + 4);
        let second = e.mem.u32(items + 8);
        let end_value = start.wrapping_add(first_count.wrapping_add(second).wrapping_mul(0x14));
        let middle = start.wrapping_add(first_count.wrapping_mul(0x14));
        e.mem.set_u32(cursor_d, end_value);
        e.mem.set_u32(second_count, second);
        e.mem.set_u32(cursor_c, end_value);
        e.mem.set_u32(base, start);
        e.mem.set_u32(cursor_a, start);
        e.mem.set_u32(moved, 0);
        e.mem.set_u32(end, end_value);
        e.mem.set_u32(cursor_b, middle);
        if start < middle {
            loop {
                let element = e.mem.u32(cursor_a);
                let low = e.mem.f32(element + 8);
                let high = e.mem.f32(element + 0xc);
                let cut_low = e.mem.f32(node_at + 0x64);
                let cut_high = e.mem.f32(node_at + 0x68);
                let below = cut_high < high;
                let case = if cut_low > low {
                    if below || cut_high.is_nan() || high.is_nan() {
                        3
                    } else {
                        1
                    }
                } else if below {
                    2
                } else {
                    0
                };
                fn_00e0a950(
                    e,
                    this,
                    case,
                    node,
                    depth,
                    Ptr::new(second_count),
                    Ptr::new(moved),
                    Ptr::new(cursor_a),
                    Ptr::new(cursor_b),
                    Ptr::new(cursor_c),
                    Ptr::new(cursor_d),
                );
                if e.mem.u32(cursor_a) >= e.mem.u32(cursor_b) {
                    break;
                }
            }
        }
        let d_value = e.mem.u32(cursor_d);
        e.mem.set_u32(d_copy, d_value);
        fn_00e0acb0(
            e,
            this,
            Ptr::new(base),
            Ptr::new(cursor_b),
            Ptr::new(cursor_c),
            Ptr::new(d_copy),
            Ptr::new(cursor_d),
            Ptr::new(end),
            node,
        );
        let middle_now = e.mem.u32(cursor_b);
        let base_now = e.mem.u32(base);
        let left_count = div20(middle_now.wrapping_sub(base_now));
        e.mem.set_u32(left + 4, left_count as u32);
        let second_now = e.mem.u32(second_count) as i32;
        let share = ((f64::from(left_count) * f64::from(second_now))
            / f64::from(first_count as i32)) as f32;
        let scaled = e.call(FLOAT_TO_INT, &args![share]).u32();
        e.mem.set_u32(left + 8, scaled);
        let c_now = e.mem.u32(cursor_c);
        let d_now = e.mem.u32(d_copy);
        let right_count = div20(d_now.wrapping_sub(c_now));
        e.mem.set_u32(right + 4, right_count as u32);
        let right_second = (second_now as u32).wrapping_sub(scaled);
        e.mem.set_u32(right + 8, right_second);
        e.mem.set_u32(left, base_now);
        let right_start = middle_now.wrapping_add(scaled.wrapping_mul(0x14));
        e.mem.set_u32(right, right_start);
        if right_second != 0 {
            let mut source = c_now;
            let mut target = right_start;
            while source < d_now {
                copy_record(e, target, source);
                source = source.wrapping_add(0x14);
                target = target.wrapping_add(0x14);
            }
        }
    });
}

// Translated from 00e0a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 0x10` with `(parent, items, side, depth)`: builds the subtree for the primitives of
/// `items` (array `+0`, count `+4`). A single primitive becomes a leaf (`00e09f80`); before
/// that, when the compiler flag at `settings + 0x14` is set and the matching free list is
/// empty (`+0x1C` for a leaf, `+0x14` otherwise), the assembler (slot `0x14`) is told the
/// nodes ran out (`(root, this, 0x800)`). Otherwise a node comes off the first free list
/// (`00e0a020`), takes the parent (`+0`), the count (`+0x78`) and is linked as the child
/// `side` of the parent (`+0x70` or `+0x74`), copying the parent's six box words from `+0xC`.
/// The three axes are ranked (`00e09970`); for each, when the cost of the node does not
/// already beat the cost of the axis, the mediator projects the primitives (slot `0x18`) and
/// `00e09be0` looks for the best cut; the other axes only get their extents (slot `0x1C`),
/// and so do the extra directions at `this[0x34] + 0x60 ...` (while their cost is within
/// reach). The primitives are then split (`00e0a0b0`) and both halves become the children
/// (`+0x74` first, then `+0x70`). Returns the node.
pub fn fn_00e0a270(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
    items: Ptr,
    side: u32,
    depth: u32,
) -> Ptr {
    let me = this.addr();
    let items_at = items.addr();
    let count = e.mem.u32(items_at + 4) as i32;
    let compiler = e.mem.u32(me + 8);
    if count == 1 {
        if e.mem.u8(compiler + 0x14) != 0 && e.mem.u32(me + 0x1c) == 0 {
            run_out_of_nodes(e, me);
        }
        return fn_00e09f80(e, this, Ptr::new(parent), items);
    }
    if e.mem.u8(compiler + 0x14) != 0 && e.mem.u32(me + 0x14) == 0 {
        run_out_of_nodes(e, me);
    }
    let node = fn_00e0a020(e, this).addr();
    e.mem.set_u32(node + 0x78, count as u32);
    e.mem.set_u32(node, parent);
    if parent == 0 {
        e.mem.set_u32(me + 0x2c, node);
    } else {
        if side == 0 {
            e.mem.set_u32(parent + 0x70, node);
        } else {
            e.mem.set_u32(parent + 0x74, node);
        }
        for offset in [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
            let value = e.mem.u32(parent + offset);
            e.mem.set_u32(node + offset, value);
        }
    }
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let (axes, values) = (f, f + 0x10);
        let (left, right) = (f + 0x20, f + 0x30);
        let (low_high, scratch) = (f + 0x40, f + 0x50);
        fn_00e09970(e, this, Ptr::new(parent), Ptr::new(axes), Ptr::new(values));
        let mut last_descriptor = 0u32;
        let mut index = 0u32;
        while index < 3 {
            let axis = e.mem.u32(axes + 4 * index);
            let descriptor = e.mem.u32(me + 0x34).wrapping_add(axis << 5);
            let bound = (f64::from(e.mem.f32(values + 4 * index))
                + f64::from(e.mem.f32(descriptor + 0x10))) as f32;
            if e.mem.f32(node + 0x6c) < bound {
                // The node already costs less: only the extents of the remaining axes.
                for rest in index..3 {
                    let axis = e.mem.u32(axes + 4 * rest);
                    let descriptor = e.mem.u32(me + 0x34).wrapping_add(axis << 5);
                    let mediator = e.mem.u32(me + 0x28);
                    let primitives = e.mem.u32(items_at);
                    let pair = node + 0xc + axis * 8;
                    e.vcall(
                        mediator,
                        0x1c,
                        &args![descriptor, axis, primitives, count as u32, pair, pair + 4],
                    );
                }
                break;
            }
            let pair = node + 0xc + axis * 8;
            let mediator = e.mem.u32(me + 0x28);
            let primitives = e.mem.u32(items_at);
            let total = e.mem.u32(items_at + 4);
            e.vcall(
                mediator,
                0x18,
                &args![descriptor, axis, primitives, total, pair, pair + 4],
            );
            sort_if_several(e, items_at);
            fn_00e09be0(
                e,
                this,
                Ptr::new(node),
                Ptr::new(descriptor),
                items,
                Ptr::new(pair),
                depth,
            );
            last_descriptor = descriptor;
            index += 1;
        }
        let directions = e.mem.u32(me + 0x30) as i32;
        let mut direction = 3i32;
        let mut offset = 0x60u32;
        while direction < directions {
            let descriptor = e.mem.u32(me + 0x34).wrapping_add(offset);
            if e.mem.f32(node + 0x6c) < e.mem.f32(descriptor + 0x10) {
                break;
            }
            let mediator = e.mem.u32(me + 0x28);
            let primitives = e.mem.u32(items_at);
            let total = e.mem.u32(items_at + 4);
            e.vcall(
                mediator,
                0x18,
                &args![
                    descriptor,
                    direction,
                    primitives,
                    total,
                    low_high,
                    low_high + 4
                ],
            );
            sort_if_several(e, items_at);
            fn_00e09be0(
                e,
                this,
                Ptr::new(node),
                Ptr::new(descriptor),
                items,
                Ptr::new(low_high),
                depth,
            );
            last_descriptor = descriptor;
            direction += 1;
            offset += 0x20;
        }
        fn_00e098b0(e, this, items, Ptr::new(node));
        let chosen = e.mem.u32(node + 0x60);
        if last_descriptor != chosen {
            let mediator = e.mem.u32(me + 0x28);
            let axis = e.mem.u32(chosen + 0x14);
            let primitives = e.mem.u32(items_at);
            let total = e.mem.u32(items_at + 4);
            e.vcall(
                mediator,
                0x18,
                &args![chosen, axis, primitives, total, scratch + 4, scratch],
            );
        }
        fn_00e0a0b0(
            e,
            this,
            items,
            Ptr::new(node),
            depth,
            Ptr::new(left),
            Ptr::new(right),
        );
        let right_child = fn_00e0a270(e, this, node, Ptr::new(right), 1, depth.wrapping_add(1));
        e.mem.set_u32(node + 0x74, right_child.addr());
        let left_child = fn_00e0a270(e, this, node, Ptr::new(left), 0, depth.wrapping_add(1));
        e.mem.set_u32(node + 0x70, left_child.addr());
    });
    Ptr::new(node)
}

/// Tells the assembler (slot `0x14` of the object at `this + 0x24`) that the free nodes are
/// used up: `(root, this, 0x800)`.
fn run_out_of_nodes(e: &mut Engine, me: u32) {
    let assembler = e.mem.u32(me + 0x24);
    let root = e.mem.u32(me + 0x2c);
    e.vcall(assembler, 0x14, &args![root, me, 0x800u32]);
}

/// Sorts the primitives of `items` (`00e0ab60`) when there is more than one.
fn sort_if_several(e: &mut Engine, items: u32) {
    let count = e.mem.u32(items + 4) as i32;
    if count > 1 {
        let primitives = e.mem.u32(items);
        e.call(
            SORT_PRIMITIVES,
            &args![primitives, 0u32, (count - 1) as u32, 0u32],
        );
    }
}

// Translated from 00e0a550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultSplitter::buildTree` (Xbox PDB), `RET 0x14` with `(mediator, settings,
/// assembler, compiler, pool)`: stores the arguments at `+0x28`, `+0x20`, `+0x24` and `+8`,
/// gets the primitive count from the mediator (slot `0x10`) and its bit length for `+0x38`.
/// The number of nodes is 0x1000 when the byte at `compiler + 0x14` is set, otherwise the
/// count plus the word at `compiler + 4`; the pool (words `+4` and `+8` are the starts of two
/// node areas) is cut into that many tree nodes of 0x7C bytes (first free list) and leaf nodes
/// of 0x64 bytes (second). The assembler gives the number of directions (slot `0xC`, to
/// `+0x30`) and their array (slot `0x10`, to `+0x34`). A queue with room for the compiler's
/// word `+8` plus two entries is allocated, `00e0a270` builds the tree from the primitives
/// (pointer in the pool's first word) and the assembler (slot `0x14`) is told the root, the
/// splitter and twice the node count. The queue and the directions are released again.
/// Returns the root.
pub fn hkp_mopp_default_splitter_build_tree(
    e: &mut Engine,
    this: Ptr,
    mediator: Ptr,
    settings: Ptr,
    assembler: Ptr,
    compiler: Ptr,
    pool: Ptr,
) -> Ptr {
    let me = this.addr();
    let mediator = mediator.addr();
    let assembler = assembler.addr();
    let compiler = compiler.addr();
    let pool = pool.addr();
    e.mem.set_u32(me + 0x20, settings.addr());
    e.mem.set_u32(me + 8, compiler);
    e.mem.set_u32(me + 0x28, mediator);
    e.mem.set_u32(me + 0x24, assembler);
    let primitives = e.vcall(mediator, 0x10, &args![]).u32();
    let extra = e.mem.u32(compiler + 4);
    let first = e.mem.u32(pool);
    e.mem.set_u32(me + 0x38, 0);
    if primitives != 0 {
        let mut rest = primitives as i32;
        let mut bits = 0;
        loop {
            rest >>= 1;
            bits += 1;
            if rest == 0 {
                break;
            }
        }
        e.mem.set_u32(me + 0x38, bits);
    }
    let units = if e.mem.u8(compiler + 0x14) != 0 {
        0x1000
    } else {
        extra.wrapping_add(primitives)
    };
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x14, 0);
    let mut node = e.mem.u32(pool + 4);
    for _ in 0..(units as i32).max(0) {
        let head = e.mem.u32(me + 0x10);
        e.mem.set_u32(node, head);
        add_word(e, me + 0x14, 1);
        e.mem.set_u32(me + 0x10, node);
        node = node.wrapping_add(0x7c);
    }
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x1c, 0);
    let mut node = e.mem.u32(pool + 8);
    for _ in 0..(units as i32).max(0) {
        let head = e.mem.u32(me + 0x18);
        e.mem.set_u32(node, head);
        add_word(e, me + 0x1c, 1);
        e.mem.set_u32(me + 0x18, node);
        node = node.wrapping_add(0x64);
    }
    let directions = e.vcall(assembler, 0xc, &args![]).u32();
    e.mem.set_u32(me + 0x30, directions);
    let array = e.vcall(assembler, 0x10, &args![]).u32();
    e.mem.set_u32(me + 0x34, array);
    let slots = e.mem.u32(e.mem.u32(me + 8) + 8).wrapping_add(2);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let queue_array = e
        .call(EASY_ALLOC, &args![allocator, slots.wrapping_mul(4)])
        .u32();
    let root = e.with_stack(0x20, |e, frame| {
        // The queue (count, size, array, last removed) and the description of the primitives
        // (array, count, the compiler's word `+4`), as the exe keeps them in its frame.
        let queue = frame.addr();
        let list = queue + 0x10;
        e.mem.set_u32(queue, 0);
        e.mem.set_u32(queue + 4, slots);
        e.mem.set_u32(queue + 8, queue_array);
        e.mem.set_u32(queue + 0xc, 0);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, primitives);
        e.mem.set_u32(list + 8, extra);
        e.mem.set_u32(me + 0xc, queue);
        let root = fn_00e0a270(e, this, 0, Ptr::new(list), 0, 0).addr();
        e.vcall(assembler, 0x14, &args![root, me, units.wrapping_add(units)]);
        root
    });
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x34, 0);
    e.mem.set_u32(me + 0x30, 0);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(EASY_FREE, &args![allocator, queue_array]);
    Ptr::new(root)
}

// Translated from 00e0a6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cost term, `RET 4` with the state record of `00e09be0` (see there) and `this` the
/// settings: `|i - j|` (the index of the primitive minus the count behind the position,
/// words `+0x14` and `+0x34`) times the inverse count (`+0x1C`); for a depth (`+0`) below the
/// directions (`+4`) it becomes `(x - 0.75) * 6`, a negative result giving 0, otherwise it is
/// squared. The value is raised to the fourth power and scaled by the float at `this + 0x10`.
pub fn fn_00e0a6c0(e: &mut Engine, this: Ptr, state: Ptr) -> f32 {
    let settings = this.addr();
    let s = state.addr();
    let distance = e.mem.u32(s + 0x14).wrapping_sub(e.mem.u32(s + 0x34)) as i32;
    let magnitude = f64::from(distance).abs() as f32;
    let scaled = mul_f32(magnitude, f64::from(e.mem.f32(s + 0x1c)));
    let value = if (e.mem.u32(s) as i32) >= (e.mem.u32(s + 4) as i32) {
        (f64::from(scaled) * f64::from(scaled)) as f32
    } else {
        let shifted = (f64::from(scaled) - THREE_QUARTERS) as f32;
        let steep = mul_f32(shifted, SIX);
        if steep < 0.0 {
            return 0.0;
        }
        steep
    };
    let square = f64::from(value) * f64::from(value);
    let cube = square * f64::from(value);
    let fourth = (f64::from(value) * cube) as f32;
    mul_f32(fourth, f64::from(e.mem.f32(settings + 0x10)))
}

/// The common part of `00e0a750` and `00e0a850`: the position of the key at `key_offset`
/// of the state record against the middle of the bounds (words `+8` and `+0xC`), made into a
/// polynomial cost scaled by the float at `this + 0x14`. `zero_when_positive` tells which side
/// of the scaled distance gives 0.
fn side_cost(e: &Engine, settings: u32, s: u32, key_offset: u32, zero_when_positive: bool) -> f32 {
    // Arithmetic in the exe's operand order with the x87 NaN choice (#199): `FADD`/`FSUB`/
    // `FMUL dword ptr` read their operand from memory (`x87_m32`); `FLD` values are quiet.
    let wide = |value: f32| f64::from(x87_quiet(value));
    let key = wide(e.mem.f32(s + key_offset));
    let middle = x87_mul(
        x87_add(wide(e.mem.f32(s + 0xc)), x87_m32(e.mem.u32(s + 8))),
        HALF,
    ) as f32;
    let offset = x87_sub(key, x87_m32(middle.to_bits())) as f32;
    let inverse = if (e.mem.u32(s) as i32) >= (e.mem.u32(s + 4) as i32) {
        e.mem.f32(s + 0x10)
    } else {
        let width = x87_sub(wide(e.mem.f32(s + 0xc)), x87_m32(e.mem.u32(s + 8))) as f32;
        let limit = x87_quiet(e.mem.f32(settings + 0x1c));
        let smaller = if limit < width { limit } else { width };
        let sum = x87_add(wide(width), x87_m32(smaller.to_bits()));
        if 0.0 < sum {
            x87_div(1.0, sum) as f32
        } else {
            f32::from_bits(HUGE_COST)
        }
    };
    let scaled = x87_mul(wide(inverse), x87_m32(offset.to_bits())) as f32;
    let outside = if zero_when_positive {
        scaled > 0.0
    } else {
        scaled < 0.0
    };
    if outside {
        return 0.0;
    }
    let square = x87_mul(f64::from(scaled), f64::from(scaled)) as f32;
    let fourth = x87_mul(f64::from(square), f64::from(square)) as f32;
    // FADDP ST2,ST0: fourth * 3 + square * 2.9.
    let polynomial = x87_add(
        x87_mul(f64::from(fourth), THREE),
        x87_mul(f64::from(square), TWO_NINE),
    );
    let eighth = x87_mul(f64::from(fourth), f64::from(fourth)) as f32;
    let sum = x87_add(polynomial, x87_mul(f64::from(eighth), FIFTEEN_HUNDRED)) as f32;
    x87_mul(f64::from(sum), x87_m32(e.mem.u32(settings + 0x14))) as f32
}

// Translated from 00e0a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cost term, `RET 4` with the state record of `00e09be0`; `this` is the settings. See
/// `side_cost`: the key at `+0x20` of the record, and 0 when the scaled distance is below 0.
pub fn fn_00e0a750(e: &mut Engine, this: Ptr, state: Ptr) -> f32 {
    side_cost(e, this.addr(), state.addr(), 0x20, false)
}

// Translated from 00e0a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cost term, `RET 4` with the state record of `00e09be0`; `this` is the settings. See
/// `side_cost`: the key at `+0x38` of the record, and 0 when the scaled distance is above 0.
pub fn fn_00e0a850(e: &mut Engine, this: Ptr, state: Ptr) -> f32 {
    side_cost(e, this.addr(), state.addr(), 0x38, true)
}

// Translated from 00e0a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 0x24` with `(case, node, depth, count_a, count_b, cursor_a, cursor_b, cursor_c,
/// cursor_d)`, the last four being pointers to cursors over 20-byte primitives and the two
/// counts pointers to words. Moves one primitive according to the case of `00e0a0b0`:
/// 0 swaps the primitives at cursor A and B and the ones at C and D after moving B, C and D
/// down by one; 1 moves cursor A up; 2 moves C and B down and swaps A with B through C; 3
/// splits the primitive at cursor A at the middle of the cut of `node` (keys at `+0x64` and
/// `+0x68` and the descriptor at `+0x60`), the mediator (slot `0x20` of `this[0x28]`) writing
/// one half in place and the other behind C, and counts one primitive from A to B.
pub fn fn_00e0a950(
    e: &mut Engine,
    this: Ptr,
    case: u32,
    node: Ptr,
    depth: u32,
    count_a: Ptr,
    count_b: Ptr,
    cursor_a: Ptr,
    cursor_b: Ptr,
    cursor_c: Ptr,
    cursor_d: Ptr,
) {
    let me = this.addr();
    let (a, b, c, d) = (
        cursor_a.addr(),
        cursor_b.addr(),
        cursor_c.addr(),
        cursor_d.addr(),
    );
    match case {
        0 => {
            add_word(e, c, 0xffff_ffec);
            add_word(e, d, 0xffff_ffec);
            add_word(e, b, 0xffff_ffec);
            let at_b = e.mem.u32(b);
            let at_a = e.mem.u32(a);
            let record_a = read_record(e, at_a);
            let record_b = read_record(e, at_b);
            let at_c = e.mem.u32(c);
            let at_d = e.mem.u32(d);
            copy_record(e, at_c, at_d);
            let at_d = e.mem.u32(d);
            write_record(e, at_d, record_a);
            let at_a = e.mem.u32(a);
            if at_a != e.mem.u32(b) {
                write_record(e, at_a, record_b);
            }
        }
        1 => add_word(e, a, 0x14),
        2 => {
            add_word(e, c, 0xffff_ffec);
            add_word(e, b, 0xffff_ffec);
            let at_b = e.mem.u32(b);
            let at_a = e.mem.u32(a);
            let at_c = e.mem.u32(c);
            if at_a != at_c {
                let record_b = read_record(e, at_b);
                copy_record(e, at_c, at_a);
                let at_a = e.mem.u32(a);
                write_record(e, at_a, record_b);
            }
        }
        3 => {
            add_word(e, count_a.addr(), 0xffff_ffff);
            add_word(e, count_b.addr(), 1);
            let node = node.addr();
            add_word(e, c, 0xffff_ffec);
            let low = e.mem.f32(node + 0x64);
            let high = e.mem.f32(node + 0x68);
            let middle = ((f64::from(high) + f64::from(low)) * HALF) as f32;
            let plane = e.mem.u32(node + 0x60);
            let mediator = e.mem.u32(me + 0x28);
            let at_c = e.mem.u32(c);
            let at_a = e.mem.u32(a);
            e.vcall(mediator, 0x20, &args![at_a, plane, middle, depth, at_c]);
            let lanes: Vec<u32> = (0..4)
                .map(|i| e.mem.u32(plane + 4 * i) ^ 0x8000_0000)
                .collect();
            let at_a = e.mem.u32(a);
            e.with_stack(0x10, |e, negated| {
                let negated = negated.addr();
                for (i, lane) in lanes.iter().enumerate() {
                    e.mem.set_u32(negated + 4 * i as u32, *lane);
                }
                e.vcall(mediator, 0x20, &args![at_a, negated, -middle, depth, at_a]);
            });
            add_word(e, a, 0x14);
        }
        _ => {}
    }
}

// Translated from 00e0acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 0x1C` with `(base, middle, cursor_c, d_copy, cursor_d, end, node)`, all but the last
/// pointers to cursors over 20-byte primitives: balances the two groups `00e0a0b0` made. If
/// the primitives from D to the end are any, the widest axis other than the one of the cut
/// (extents from the floats of `node` at `+0xC` to `+0x20`) is chosen, the primitives are
/// projected on it by the mediator (slot `0x18`, descriptor `this[0x34] + 0x20 * axis`) and
/// sorted when there are several. Then, while D has not reached the end, either both D and
/// the copy advance, or (when the smaller group is less than a quarter or four times of the
/// other, depending on which was last moved) the last primitive of the array goes to B's
/// place, B's old one to D's and D's old one into the end slot, and B, C, D and the copy
/// advance.
pub fn fn_00e0acb0(
    e: &mut Engine,
    this: Ptr,
    base: Ptr,
    middle: Ptr,
    cursor_c: Ptr,
    d_copy: Ptr,
    cursor_d: Ptr,
    end: Ptr,
    node: Ptr,
) {
    let me = this.addr();
    let (base, middle, c, d_copy, d, end, node) = (
        base.addr(),
        middle.addr(),
        cursor_c.addr(),
        d_copy.addr(),
        cursor_d.addr(),
        end.addr(),
        node.addr(),
    );
    let tail = div20(e.mem.u32(end).wrapping_sub(e.mem.u32(d)));
    if tail == 0 {
        return;
    }
    let cut_axis = e.mem.u32(e.mem.u32(node + 0x60) + 0x14);
    let mut top = 0.0f32;
    let mut axis = 0u32;
    if cut_axis != 0 {
        let width = sub_f32(e.mem.f32(node + 0x10), e.mem.f32(node + 0xc));
        if width > 0.0 {
            top = width;
        }
    }
    if cut_axis != 1 {
        let width = sub_f32(e.mem.f32(node + 0x18), e.mem.f32(node + 0x14));
        if top < width {
            top = width;
            axis = 1;
        }
    }
    if cut_axis != 2 {
        let width = sub_f32(e.mem.f32(node + 0x20), e.mem.f32(node + 0x1c));
        if top < width {
            axis = 2;
        }
    }
    let mediator = e.mem.u32(me + 0x28);
    let descriptor = e.mem.u32(me + 0x34).wrapping_add(axis << 5);
    let tail_start = e.mem.u32(d);
    e.with_stack(0x10, |e, range| {
        let range = range.addr();
        e.vcall(
            mediator,
            0x18,
            &args![descriptor, axis, tail_start, tail as u32, range, range + 4],
        );
    });
    if tail > 1 {
        let start = e.mem.u32(d);
        e.call(
            SORT_PRIMITIVES,
            &args![start, 0u32, (tail - 1) as u32, 0u32],
        );
    }
    let first_group = div20(e.mem.u32(middle).wrapping_sub(e.mem.u32(base)));
    let second_group = div20(e.mem.u32(d_copy).wrapping_sub(e.mem.u32(c)));
    let mut first_smaller = first_group < second_group;
    while e.mem.u32(d) < e.mem.u32(end) {
        let first_group = div20(e.mem.u32(middle).wrapping_sub(e.mem.u32(base)));
        let second_group = div20(e.mem.u32(d_copy).wrapping_sub(e.mem.u32(c)));
        let (ours, theirs) = if first_smaller {
            (first_group, second_group.wrapping_mul(4))
        } else {
            (first_group.wrapping_mul(4), second_group)
        };
        if ours < theirs {
            let last = e.mem.u32(end).wrapping_sub(0x14);
            let last_record = read_record(e, last);
            let at_d = e.mem.u32(d);
            copy_record(e, last, at_d);
            let at_c = e.mem.u32(c);
            let record_c = read_record(e, at_c);
            let at_middle = e.mem.u32(middle);
            write_record(e, at_middle, last_record);
            let at_d = e.mem.u32(d);
            write_record(e, at_d, record_c);
            add_word(e, middle, 0x14);
            add_word(e, c, 0x14);
            add_word(e, d, 0x14);
            add_word(e, d_copy, 0x14);
            first_smaller = true;
        } else {
            add_word(e, d_copy, 0x14);
            add_word(e, d, 0x14);
            first_smaller = false;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_9670, fn_00e09670(Ptr, Ptr)),
        entry!(0x00e0_97a0, fn_00e097a0(Ptr, f32)),
        entry!(0x00e0_98b0, fn_00e098b0(Ptr, Ptr, Ptr)),
        entry!(0x00e0_9970, fn_00e09970(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00e0_9ba0,
            hkp_mopp_default_splitter_constructor(Ptr) -> Ptr
        ),
        entry!(0x00e0_9bd0, fn_00e09bd0(Ptr)),
        entry!(0x00e0_9be0, fn_00e09be0(Ptr, Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00e0_9f80, fn_00e09f80(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00e0_a020, fn_00e0a020(Ptr) -> Ptr),
        entry!(
            0x00e0_a070,
            hkp_mopp_default_splitter_release_node(Ptr, Ptr)
        ),
        entry!(
            0x00e0_a0a0,
            hkp_mopp_default_splitter_get_free_nodes(Ptr) -> u32
        ),
        entry!(0x00e0_a0b0, fn_00e0a0b0(Ptr, Ptr, Ptr, u32, Ptr, Ptr)),
        entry!(0x00e0_a270, fn_00e0a270(Ptr, u32, Ptr, u32, u32) -> Ptr),
        entry!(
            0x00e0_a550,
            hkp_mopp_default_splitter_build_tree(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00e0_a6c0, fn_00e0a6c0(Ptr, Ptr) -> f32),
        entry!(0x00e0_a750, fn_00e0a750(Ptr, Ptr) -> f32),
        entry!(0x00e0_a850, fn_00e0a850(Ptr, Ptr) -> f32),
        entry!(
            0x00e0_a950,
            fn_00e0a950(Ptr, u32, Ptr, u32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_acb0,
            fn_00e0acb0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;
    /// The interval of every primitive on every axis: `[axis][index]`.
    type Table = Rc<Vec<Vec<(f32, f32)>>>;

    const M_COUNT: u32 = 0x0900_0701;
    const M_PROJECT: u32 = 0x0900_0702;
    const M_EXTENTS: u32 = 0x0900_0703;
    const M_SPLIT: u32 = 0x0900_0704;
    const M_PROPERTIES: u32 = 0x0900_0705;
    const A_KINDS: u32 = 0x0900_0710;
    const A_ARRAY: u32 = 0x0900_0711;
    const A_FINISH: u32 = 0x0900_0712;

    fn push(log: &Log, row: Vec<u32>) {
        log.borrow_mut().push(row);
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1e-5 * b.abs().max(1e-3)
    }

    /// An object whose vtable has the given slots (offset, target).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A mediator whose slots count the primitives, project them, find extents, split and
    /// give properties; `log` gets a row for every call to project, extents and split.
    fn mediator(e: &mut Engine, log: &Log, count: u32, table: Table) -> u32 {
        e.register_double(M_COUNT, move |_, _| Ret {
            eax: count,
            ..Ret::default()
        });
        for (target, row_kind, write) in [(M_PROJECT, 0x18, true), (M_EXTENTS, 0x1c, false)] {
            let sink = Rc::clone(log);
            let table = Rc::clone(&table);
            e.register_double(target, move |e, a| {
                let (axis, primitives, count) = (a[2], a[3], a[4] as i32);
                push(&sink, vec![row_kind, a[1], axis, primitives, count as u32]);
                let (mut low, mut high) = (f32::INFINITY, f32::NEG_INFINITY);
                for i in 0..count.max(0) as u32 {
                    let primitive = primitives + 0x14 * i;
                    let index = e.mem.u32(primitive + 4) as usize;
                    let (l, h) = table[axis as usize][index];
                    if write {
                        e.mem.set_f32(primitive + 8, l);
                        e.mem.set_f32(primitive + 0xc, h);
                    }
                    low = low.min(l);
                    high = high.max(h);
                }
                e.mem.set_f32(a[5], low);
                e.mem.set_f32(a[6], high);
                Ret::default()
            });
        }
        let sink = Rc::clone(log);
        e.register_double(M_SPLIT, move |e, a| {
            // (primitive, plane, middle, depth, output)
            let lanes: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            let mut row = vec![0x20, a[1], a[3], a[4], a[5]];
            row.extend(lanes);
            push(&sink, row);
            for i in 0..5 {
                let value = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[5] + 4 * i, value);
            }
            Ret::default()
        });
        e.register_double(M_PROPERTIES, |e, a| {
            e.mem.set_u32(a[2], 0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        object(
            e,
            &[
                (0x10, M_COUNT),
                (0x18, M_PROJECT),
                (0x1c, M_EXTENTS),
                (0x20, M_SPLIT),
                (0x24, M_PROPERTIES),
            ],
        )
    }

    /// The sorted queue of nodes: count, size, array, last removed.
    fn queue(e: &mut Engine, size: u32) -> u32 {
        let queue = e.mem.alloc(0x10);
        let array = e.mem.alloc(size * 4);
        e.mem.set_u32(queue + 4, size);
        e.mem.set_u32(queue + 8, array);
        queue
    }

    fn queue_entries(e: &Engine, queue: u32) -> Vec<u32> {
        let count = e.mem.u32(queue);
        let array = e.mem.u32(queue + 8);
        (0..count).map(|i| e.mem.u32(array + 4 * i)).collect()
    }

    fn node_with_key(e: &mut Engine, key: f32) -> u32 {
        let node = e.mem.alloc(0x80);
        e.mem.set_f32(node + 0xc, key);
        node
    }

    /// `count` primitives of 20 bytes with the id `10 * (i + 1)` and the index `i`.
    fn primitives(e: &mut Engine, count: u32) -> u32 {
        let at = e.mem.alloc(0x14 * count + 0x14);
        for i in 0..count {
            e.mem.set_u32(at + 0x14 * i, 10 * (i + 1));
            e.mem.set_u32(at + 0x14 * i + 4, i);
        }
        at
    }

    /// A description of primitives: array, count, a third word.
    fn items(e: &mut Engine, array: u32, count: u32, third: u32) -> u32 {
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at, array);
        e.mem.set_u32(at + 4, count);
        e.mem.set_u32(at + 8, third);
        at
    }

    /// Settings with every weight 1, no depth term and the limit 1 (words as the cost
    /// functions read them).
    fn settings(e: &mut Engine) -> u32 {
        let at = e.mem.alloc(0x40);
        for (offset, value) in [
            (8, 1.0f32),
            (0xc, 1.0),
            (0x10, 1.0),
            (0x14, 1.0),
            (0x18, 0.0),
            (0x1c, 1.0),
        ] {
            e.mem.set_f32(at + offset, value);
        }
        at
    }

    #[test]
    fn inserting_keeps_the_queue_in_descending_key_order() {
        let mut e = Engine::new();
        let queue = queue(&mut e, 8);
        let nodes: Vec<u32> = [5.0f32, 1.0, 3.0]
            .iter()
            .map(|key| node_with_key(&mut e, *key))
            .collect();
        for node in &nodes {
            e.call(0x00e09670, &args![queue, *node]);
        }
        assert_eq!(queue_entries(&e, queue), vec![nodes[0], nodes[2], nodes[1]]);
        // A NaN key is not above anything: it goes behind the entries it is compared with.
        let nan = node_with_key(&mut e, f32::NAN);
        e.call(0x00e09670, &args![queue, nan]);
        assert_eq!(
            queue_entries(&e, queue),
            vec![nodes[0], nodes[2], nodes[1], nan]
        );
        // The walk stops at an empty slot.
        let array = e.mem.u32(queue + 8);
        e.mem.set_u32(array + 12, 0);
        let last = node_with_key(&mut e, 9.0);
        e.call(0x00e09670, &args![queue, last]);
        assert_eq!(e.mem.u32(array + 16), last);
        assert_eq!(e.mem.u32(queue), 5);
    }

    #[test]
    fn taking_off_removes_the_nodes_below_the_key_from_the_end() {
        let mut e = Engine::new();
        let queue = queue(&mut e, 8);
        let nodes: Vec<u32> = [5.0f32, 3.0, 1.0]
            .iter()
            .map(|key| node_with_key(&mut e, *key))
            .collect();
        for node in &nodes {
            e.call(0x00e09670, &args![queue, *node]);
        }
        e.call(0x00e097a0, &args![queue, 4.0f32]);
        assert_eq!(queue_entries(&e, queue), vec![nodes[0]]);
        assert_eq!(e.mem.u32(queue + 0xc), nodes[1]);
        let array = e.mem.u32(queue + 8);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0);
        // Nothing is below 1.0 any more.
        e.call(0x00e097a0, &args![queue, 1.0f32]);
        assert_eq!(e.mem.u32(queue), 1);
    }

    #[test]
    fn properties_give_the_ranges_of_the_keys_and_of_each_property() {
        let mut e = Engine::new();
        // Two properties per primitive: twice the first word and the first word plus one.
        e.register_double(M_PROPERTIES, |e, a| {
            let key = e.mem.u32(a[1]);
            e.mem.set_u32(a[2], key * 2);
            e.mem.set_u32(a[2] + 4, key + 1);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        let mediator = object(&mut e, &[(0x24, M_PROPERTIES)]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, mediator);
        let array = e.mem.alloc(0x40);
        for (i, key) in [7u32, 3, 9].iter().enumerate() {
            e.mem.set_u32(array + 0x14 * i as u32, *key);
        }
        let description = items(&mut e, array, 3, 0);
        let output = e.mem.alloc(0x40);
        e.call(0x00e098b0, &args![this, description, output]);
        assert_eq!(e.mem.u32(output + 0x24), 3);
        assert_eq!(e.mem.u32(output + 0x28), 9);
        assert_eq!(e.mem.u32(output + 8), 3);
        assert_eq!(e.mem.u32(output + 0x2c), 2);
        // The minimum of property 1 and the maximum of property 0 share the word at +0x34.
        assert_eq!(e.mem.u32(output + 0x30), 6);
        assert_eq!(e.mem.u32(output + 0x34), 10);
        assert_eq!(e.mem.u32(output + 0x38), 10);
    }

    #[test]
    fn one_property_per_primitive_gives_a_plain_range() {
        let mut e = Engine::new();
        e.register_double(M_PROPERTIES, |e, a| {
            let key = e.mem.u32(a[1]);
            e.mem.set_u32(a[2], key + 100);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let mediator = object(&mut e, &[(0x24, M_PROPERTIES)]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, mediator);
        let array = e.mem.alloc(0x40);
        e.mem.set_u32(array, 5);
        e.mem.set_u32(array + 0x14, 8);
        let description = items(&mut e, array, 2, 0);
        let output = e.mem.alloc(0x40);
        e.call(0x00e098b0, &args![this, description, output]);
        assert_eq!(e.mem.u32(output + 0x2c), 1);
        assert_eq!(e.mem.u32(output + 0x30), 105);
        assert_eq!(e.mem.u32(output + 0x34), 108);
        assert_eq!(e.mem.u32(output + 0x24), 5);
        assert_eq!(e.mem.u32(output + 0x28), 8);
    }

    #[test]
    fn without_a_node_the_axes_are_in_order_and_the_values_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let axes = e.mem.alloc(0x10);
        let values = e.mem.alloc(0x10);
        e.mem.set_f32(values, 5.0);
        e.call(0x00e09970, &args![this, 0u32, axes, values]);
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(axes + 4 * i)).collect();
        assert_eq!(got, vec![0, 1, 2]);
        let weights: Vec<f32> = (0..3).map(|i| e.mem.f32(values + 4 * i)).collect();
        assert_eq!(weights, vec![0.0, 0.0, 0.0]);
    }

    /// A node whose box has the widths given for the three axes, starting at 0.
    fn boxed_node(e: &mut Engine, widths: [f32; 3], tag: u32) -> u32 {
        let node = e.mem.alloc(0x80);
        for (axis, width) in widths.iter().enumerate() {
            e.mem.set_f32(node + 0xc + 8 * axis as u32 + 4, *width);
        }
        e.mem.set_u32(node + 0x60, tag);
        node
    }

    #[test]
    fn the_widest_axis_comes_first_then_the_other_two_by_extent_with_falloff_weights() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let descriptors = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x34, descriptors);
        let axes = e.mem.alloc(0x10);
        let values = e.mem.alloc(0x10);
        let node = boxed_node(&mut e, [4.0, 2.0, 1.0], 0x1234);
        e.call(0x00e09970, &args![this, node, axes, values]);
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(axes + 4 * i)).collect();
        assert_eq!(got, vec![0, 1, 2]);
        assert_eq!(e.mem.f32(values), 0.0);
        // 0.05 * 16 * ((4 - 2) / 4)^3 and 0.05 * 16 * ((4 - 1) / 4)^3.
        assert!(close(e.mem.f32(values + 4), 0.1));
        assert!(close(e.mem.f32(values + 8), 0.3375));
        // The two others swapped: the second axis is the one with the larger extent.
        let node = boxed_node(&mut e, [4.0, 1.0, 2.0], 0x1234);
        e.call(0x00e09970, &args![this, node, axes, values]);
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(axes + 4 * i)).collect();
        assert_eq!(got, vec![0, 2, 1]);
        assert!(close(e.mem.f32(values + 4), 0.1));
        assert!(close(e.mem.f32(values + 8), 0.3375));
    }

    #[test]
    fn the_axis_of_the_tag_is_shrunk_to_0_66_of_its_width() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let descriptors = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x34, descriptors);
        let axes = e.mem.alloc(0x10);
        let values = e.mem.alloc(0x10);
        // Axis 0 is 4 wide but tagged, so it counts 2.64 and axis 1 (3 wide) is the widest.
        let node = boxed_node(&mut e, [4.0, 3.0, 1.0], descriptors);
        e.call(0x00e09970, &args![this, node, axes, values]);
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(axes + 4 * i)).collect();
        assert_eq!(got, vec![1, 0, 2]);
        // The weights use the widest extent, 3, against 2.64 and 1.
        assert!(close(
            e.mem.f32(values + 4),
            0.05 * 16.0 * (0.36f32 / 3.0).powi(3)
        ));
        assert!(close(
            e.mem.f32(values + 8),
            0.05 * 16.0 * (2.0f32 / 3.0).powi(3)
        ));
        // The tag of the third axis: 4 * 0.66 = 2.64 is still the widest.
        let node = boxed_node(&mut e, [1.0, 1.0, 4.0], descriptors + 0x40);
        e.call(0x00e09970, &args![this, node, axes, values]);
        assert_eq!(e.mem.u32(axes), 2);
        assert!(e.mem.f32(values + 4) > 0.0);
        // A wider untagged axis wins over the tagged one.
        let node = boxed_node(&mut e, [3.0, 4.0, 1.0], descriptors + 0x20);
        e.call(0x00e09970, &args![this, node, axes, values]);
        assert_eq!(e.mem.u32(axes), 0);
    }

    #[test]
    fn the_constructor_and_the_destructor_set_the_vtables() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        for offset in [8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
            e.mem.set_u32(this + offset, 0x55);
        }
        let got = e.call(0x00e09ba0, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), VTABLE_SPLITTER);
        assert_eq!(e.mem.u16(this + 6), 1);
        for offset in [8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        e.call(0x00e09bd0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_BASE);
    }

    #[test]
    fn a_leaf_comes_off_the_second_free_list_with_its_extents() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let intervals: Table = Rc::new(vec![vec![(1.0, 2.0)], vec![(3.0, 4.0)], vec![(5.0, 6.0)]]);
        let mediator = mediator(&mut e, &log, 1, intervals);
        let this = e.mem.alloc(0x40);
        let descriptors = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let spare = e.mem.alloc(0x80);
        e.mem.set_u32(node, spare);
        e.mem.set_u32(this + 0x18, node);
        e.mem.set_u32(this + 0x1c, 2);
        e.mem.set_u32(this + 0x28, mediator);
        e.mem.set_u32(this + 0x34, descriptors);
        let array = primitives(&mut e, 1);
        let description = items(&mut e, array, 1, 0);
        let parent = 0x7777u32;
        let got = e.call(0x00e09f80, &args![this, parent, description]).u32();
        assert_eq!(got, node);
        assert_eq!(e.mem.u32(this + 0x18), spare);
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        assert_eq!(e.mem.u32(node), parent);
        assert_eq!(e.mem.u8(node + 4), 1);
        assert_eq!(e.mem.u32(node + 0x60), array);
        assert_eq!(e.mem.u32(node + 0x40), 0xffff_ffff);
        assert_eq!(e.mem.u32(node + 0x5c), 0xffff_ffff);
        assert_eq!(e.mem.u32(node + 8), 1);
        let pairs: Vec<f32> = (0..6).map(|i| e.mem.f32(node + 0xc + 4 * i)).collect();
        assert_eq!(pairs, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // The properties of the one primitive: count 1, range of its zero property.
        assert_eq!(e.mem.u32(node + 0x2c), 1);
        assert_eq!(e.mem.u32(node + 0x30), 0);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x1c, descriptors, 0, array, 1],
                vec![0x1c, descriptors + 0x20, 1, array, 1],
                vec![0x1c, descriptors + 0x40, 2, array, 1],
            ]
        );
    }

    #[test]
    fn a_new_node_comes_off_the_first_free_list_and_is_cleared() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let node = e.mem.alloc(0x80);
        let spare = e.mem.alloc(0x80);
        e.mem.set_u32(node, spare);
        for offset in 0x38..=0x3d {
            e.mem.set_u8(node + offset, 0x77);
        }
        e.mem.set_u8(node + 4, 1);
        e.mem.set_u32(this + 0x10, node);
        e.mem.set_u32(this + 0x14, 5);
        let got = e.call(0x00e0a020, &args![this]).u32();
        assert_eq!(got, node);
        assert_eq!(e.mem.u32(this + 0x10), spare);
        assert_eq!(e.mem.u32(this + 0x14), 4);
        for offset in 0x38..=0x3d {
            assert_eq!(e.mem.u8(node + offset), 0);
        }
        assert_eq!(e.mem.u8(node + 4), 0);
        assert_eq!(e.mem.u32(node + 0x40), 0xffff_ffff);
        assert_eq!(e.mem.u32(node + 0x5c), 0xffff_ffff);
        assert_eq!(e.mem.f32(node + 0x6c), 1.0e6);
        assert_eq!(e.mem.f32(node + 0x64), 1.0e10);
        assert_eq!(e.mem.u32(node + 0x70), 0);
        assert_eq!(e.mem.u32(node + 0x74), 0);
    }

    #[test]
    fn releasing_a_node_puts_it_on_the_list_of_its_kind() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let plain = e.mem.alloc(0x80);
        let leaf = e.mem.alloc(0x80);
        e.mem.set_u8(leaf + 4, 1);
        e.call(0x00e0a070, &args![this, 0u32]);
        assert_eq!(e.call(0x00e0a0a0, &args![this]).u32(), 0);
        e.call(0x00e0a070, &args![this, plain]);
        e.call(0x00e0a070, &args![this, leaf]);
        assert_eq!(e.mem.u32(this + 0x10), plain);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(e.mem.u32(this + 0x18), leaf);
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        assert_eq!(e.call(0x00e0a0a0, &args![this]).u32(), 2);
        let second_leaf = e.mem.alloc(0x80);
        e.mem.set_u8(second_leaf + 4, 1);
        e.call(0x00e0a070, &args![this, second_leaf]);
        assert_eq!(e.mem.u32(second_leaf), leaf);
        assert_eq!(e.mem.u32(this + 0x18), second_leaf);
        assert_eq!(e.call(0x00e0a0a0, &args![this]).u32(), 3);
    }

    /// The state record of the cost functions: depth, directions, bounds, ...
    fn state(e: &mut Engine, depth: u32, directions: u32) -> u32 {
        let state = e.mem.alloc(0x60);
        e.mem.set_u32(state, depth);
        e.mem.set_u32(state + 4, directions);
        e.mem.set_f32(state + 8, 0.0);
        e.mem.set_f32(state + 0xc, 4.0);
        e.mem.set_f32(state + 0x10, 0.5);
        e.mem.set_u32(state + 0x14, 7);
        e.mem.set_f32(state + 0x1c, 0.25);
        e.mem.set_u32(state + 0x34, 3);
        state
    }

    #[test]
    fn the_size_cost_is_the_fourth_power_of_the_scaled_distance() {
        let mut e = Engine::new();
        let settings = e.mem.alloc(0x40);
        e.mem.set_f32(settings + 0x10, 2.0);
        // |7 - 3| * 0.25 = 1, squared 1, to the fourth power 1, times 2.
        let at_depth = state(&mut e, 3, 2);
        let got = e.call(0x00e0a6c0, &args![settings, at_depth]).f32();
        assert!(close(got, 2.0));
        // Below the directions: (1 - 0.75) * 6 = 1.5, to the fourth power 5.0625, times 2.
        let below = state(&mut e, 1, 2);
        let got = e.call(0x00e0a6c0, &args![settings, below]).f32();
        assert!(close(got, 10.125));
        // A scaled distance of 0.5 gives (0.5 - 0.75) * 6 < 0: no cost.
        e.mem.set_u32(below + 0x14, 5);
        let got = e.call(0x00e0a6c0, &args![settings, below]).f32();
        assert_eq!(got, 0.0);
    }

    /// The expected polynomial of the side costs for the scaled distance `q`.
    fn polynomial(q: f32, scale: f32) -> f32 {
        let s = (q * q) as f64;
        let s2 = s * s;
        let a = 3.0 * s2 + f64::from(2.9f32) * s;
        ((a + 1500.0 * s2 * s2) as f32) * scale
    }

    #[test]
    fn the_side_costs_are_zero_on_their_own_side_of_the_middle() {
        let mut e = Engine::new();
        let settings = settings(&mut e);
        e.mem.set_f32(settings + 0x14, 2.0);
        // Middle 2; the key 3 is one above it, and the inverse width 0.5 makes q = 0.5.
        let record = state(&mut e, 3, 2);
        e.mem.set_f32(record + 0x20, 3.0);
        e.mem.set_f32(record + 0x38, 3.0);
        let above = e.call(0x00e0a750, &args![settings, record]).f32();
        assert!(close(above, polynomial(0.5, 2.0)));
        assert_eq!(e.call(0x00e0a850, &args![settings, record]).f32(), 0.0);
        // The key 1 is below the middle: the other way round.
        e.mem.set_f32(record + 0x20, 1.0);
        e.mem.set_f32(record + 0x38, 1.0);
        assert_eq!(e.call(0x00e0a750, &args![settings, record]).f32(), 0.0);
        let below = e.call(0x00e0a850, &args![settings, record]).f32();
        assert!(close(below, polynomial(-0.5, 2.0)));
    }

    #[test]
    fn the_side_costs_below_the_directions_scale_by_the_width_and_the_limit() {
        let mut e = Engine::new();
        let settings = settings(&mut e);
        // Depth 1 is below the directions 2: the width is 4, the limit 1, so the inverse is
        // 1 / (4 + 1) and the key 3 (one above the middle) makes q = 0.2.
        let record = state(&mut e, 1, 2);
        e.mem.set_f32(record + 0x20, 3.0);
        let got = e.call(0x00e0a750, &args![settings, record]).f32();
        assert!(close(got, polynomial(0.2, 1.0)));
        // A limit above the width counts the width twice: 1 / 8.
        e.mem.set_f32(settings + 0x1c, 10.0);
        let got = e.call(0x00e0a750, &args![settings, record]).f32();
        assert!(close(got, polynomial(0.125, 1.0)));
        // A width of zero or below gives the huge inverse, which makes the cost infinite.
        e.mem.set_f32(record + 0xc, 0.0);
        let got = e.call(0x00e0a750, &args![settings, record]).f32();
        assert!(got.is_infinite() || got > 1.0e30);
    }

    #[test]
    fn the_side_costs_keep_the_x87_nan_when_two_nans_meet() {
        // #199: the key 0x7fc00000 makes the polynomial a NaN that meets the scale
        // 0xffffffff in the last FMUL; the larger significand wins.
        let mut e = Engine::new();
        let settings = settings(&mut e);
        e.mem.set_u32(settings + 0x14, 0xffff_ffff);
        let record = state(&mut e, 3, 2);
        e.mem.set_u32(record + 0x20, 0x7fc0_0000);
        e.mem.set_u32(record + 0x38, 0x7fc0_0000);
        let above = e.call(0x00e0a750, &args![settings, record]).f32();
        assert_eq!(above.to_bits(), 0xffff_ffff);
        let below = e.call(0x00e0a850, &args![settings, record]).f32();
        assert_eq!(below.to_bits(), 0xffff_ffff);
    }

    /// Records of 20 bytes whose first word is the id `i`, in an array of `count`.
    fn records(e: &mut Engine, count: u32) -> u32 {
        let at = e.mem.alloc(0x14 * count + 0x14);
        for i in 0..count {
            e.mem.set_u32(at + 0x14 * i, i);
            for word in 1..5 {
                e.mem.set_u32(at + 0x14 * i + 4 * word, 100 * i + word);
            }
        }
        at
    }

    /// A word holding `value`.
    fn cell(e: &mut Engine, value: u32) -> u32 {
        let at = e.mem.alloc(4);
        e.mem.set_u32(at, value);
        at
    }

    fn record(e: &Engine, at: u32) -> Vec<u32> {
        (0..5).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    #[test]
    fn case_0_rotates_four_records() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let r = records(&mut e, 8);
        let at = |i: u32| r + 0x14 * i;
        let before: Vec<Vec<u32>> = (0..8).map(|i| record(&e, at(i))).collect();
        let (a, b, c, d) = (
            cell(&mut e, at(0)),
            cell(&mut e, at(3)),
            cell(&mut e, at(5)),
            cell(&mut e, at(6)),
        );
        let (count_a, count_b) = (cell(&mut e, 5), cell(&mut e, 5));
        e.call(
            0x00e0a950,
            &args![this, 0u32, 0u32, 0u32, count_a, count_b, a, b, c, d],
        );
        // B, C and D moved down by one record.
        assert_eq!(e.mem.u32(b), at(2));
        assert_eq!(e.mem.u32(c), at(4));
        assert_eq!(e.mem.u32(d), at(5));
        assert_eq!(e.mem.u32(a), at(0));
        // R4 gets R5, R5 gets R0, R0 gets R2.
        assert_eq!(record(&e, at(4)), before[5]);
        assert_eq!(record(&e, at(5)), before[0]);
        assert_eq!(record(&e, at(0)), before[2]);
        // The counters are not touched.
        assert_eq!(e.mem.u32(count_a), 5);
        assert_eq!(e.mem.u32(count_b), 5);
        // With A at B nothing is written to A's record.
        let (a, b) = (cell(&mut e, at(2)), cell(&mut e, at(3)));
        let (c, d) = (cell(&mut e, at(6)), cell(&mut e, at(7)));
        let before: Vec<Vec<u32>> = (0..8).map(|i| record(&e, at(i))).collect();
        e.call(
            0x00e0a950,
            &args![this, 0u32, 0u32, 0u32, count_a, count_b, a, b, c, d],
        );
        assert_eq!(record(&e, at(2)), before[2]);
        assert_eq!(record(&e, at(5)), before[6]);
        assert_eq!(record(&e, at(6)), before[2]);
    }

    #[test]
    fn case_1_advances_the_first_cursor_and_unknown_cases_do_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let r = records(&mut e, 4);
        let a = cell(&mut e, r);
        let (b, c, d) = (cell(&mut e, r), cell(&mut e, r), cell(&mut e, r));
        let (count_a, count_b) = (cell(&mut e, 5), cell(&mut e, 5));
        e.call(
            0x00e0a950,
            &args![this, 1u32, 0u32, 0u32, count_a, count_b, a, b, c, d],
        );
        assert_eq!(e.mem.u32(a), r + 0x14);
        e.call(
            0x00e0a950,
            &args![this, 4u32, 0u32, 0u32, count_a, count_b, a, b, c, d],
        );
        assert_eq!(e.mem.u32(a), r + 0x14);
        assert_eq!(e.mem.u32(b), r);
    }

    #[test]
    fn case_2_swaps_the_first_cursor_through_the_third() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let r = records(&mut e, 8);
        let at = |i: u32| r + 0x14 * i;
        let before: Vec<Vec<u32>> = (0..8).map(|i| record(&e, at(i))).collect();
        let (a, b, c, d) = (
            cell(&mut e, at(0)),
            cell(&mut e, at(3)),
            cell(&mut e, at(6)),
            cell(&mut e, at(7)),
        );
        let (count_a, count_b) = (cell(&mut e, 5), cell(&mut e, 5));
        e.call(
            0x00e0a950,
            &args![this, 2u32, 0u32, 0u32, count_a, count_b, a, b, c, d],
        );
        assert_eq!(e.mem.u32(b), at(2));
        assert_eq!(e.mem.u32(c), at(5));
        assert_eq!(e.mem.u32(d), at(7));
        assert_eq!(record(&e, at(5)), before[0]);
        assert_eq!(record(&e, at(0)), before[2]);
        // With A at the new C nothing moves.
        let (a, b, c) = (
            cell(&mut e, at(4)),
            cell(&mut e, at(3)),
            cell(&mut e, at(5)),
        );
        let before: Vec<Vec<u32>> = (0..8).map(|i| record(&e, at(i))).collect();
        e.call(
            0x00e0a950,
            &args![this, 2u32, 0u32, 0u32, count_a, count_b, a, b, c, d],
        );
        assert_eq!(e.mem.u32(c), at(4));
        let after: Vec<Vec<u32>> = (0..8).map(|i| record(&e, at(i))).collect();
        assert_eq!(after, before);
    }

    #[test]
    fn case_3_splits_the_primitive_at_the_middle_of_the_cut_and_counts_it_twice() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let mediator = mediator(&mut e, &log, 0, Rc::new(vec![vec![]; 3]));
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, mediator);
        let r = records(&mut e, 8);
        let at = |i: u32| r + 0x14 * i;
        let plane = e.mem.alloc(0x10);
        for (i, lane) in [1.0f32, -2.0, 0.5, 0.0].iter().enumerate() {
            e.mem.set_f32(plane + 4 * i as u32, *lane);
        }
        let node = e.mem.alloc(0x80);
        e.mem.set_u32(node + 0x60, plane);
        e.mem.set_f32(node + 0x64, 1.0);
        e.mem.set_f32(node + 0x68, 4.0);
        let (a, b, c, d) = (
            cell(&mut e, at(1)),
            cell(&mut e, at(4)),
            cell(&mut e, at(7)),
            cell(&mut e, at(7)),
        );
        let (count_a, count_b) = (cell(&mut e, 5), cell(&mut e, 5));
        let before_a = record(&e, at(1));
        e.call(
            0x00e0a950,
            &args![this, 3u32, node, 9u32, count_a, count_b, a, b, c, d],
        );
        assert_eq!(e.mem.u32(count_a), 4);
        assert_eq!(e.mem.u32(count_b), 6);
        assert_eq!(e.mem.u32(c), at(6));
        assert_eq!(e.mem.u32(a), at(2));
        let middle = 2.5f32.to_bits();
        let negated: Vec<u32> = [-1.0f32, 2.0, -0.5, -0.0]
            .iter()
            .map(|lane| lane.to_bits())
            .collect();
        let plain: Vec<u32> = [1.0f32, -2.0, 0.5, 0.0]
            .iter()
            .map(|lane| lane.to_bits())
            .collect();
        // First the primitive is cut into the new record behind C, with the plane as it is;
        // then in place, against the negated plane and the negated middle.
        let mut first = vec![0x20, at(1), middle, 9, at(6)];
        first.extend(plain);
        let mut second = vec![0x20, at(1), (-2.5f32).to_bits(), 9, at(1)];
        second.extend(negated);
        assert_eq!(*log.borrow(), vec![first, second]);
        // The double copies the primitive to the output, so the record behind C is the same.
        assert_eq!(record(&e, at(6)), before_a);
    }

    /// A node for the balancing: a cut descriptor with the axis index `cut_axis` and the box
    /// with the widths 1 on axis 0, 3 on axis 1 and 2 on axis 2.
    fn balancing_node(e: &mut Engine, cut_axis: u32) -> u32 {
        let descriptor = e.mem.alloc(0x40);
        e.mem.set_u32(descriptor + 0x14, cut_axis);
        let node = e.mem.alloc(0x80);
        e.mem.set_u32(node + 0x60, descriptor);
        e.mem.set_f32(node + 0x10, 1.0);
        e.mem.set_f32(node + 0x18, 3.0);
        e.mem.set_f32(node + 0x20, 2.0);
        node
    }

    /// The first word of the primitive at `at`.
    fn id(e: &Engine, at: u32) -> u32 {
        e.mem.u32(at)
    }

    /// A splitter wired to the mediator and with a descriptor array, for the balancing.
    fn balancing_rig(e: &mut Engine, log: &Log) -> (u32, u32) {
        let rows = vec![(0.0f32, 0.0f32); 12];
        let mediator = mediator(e, log, 0, Rc::new(vec![rows; 3]));
        let this = e.mem.alloc(0x40);
        let descriptors = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x28, mediator);
        e.mem.set_u32(this + 0x34, descriptors);
        (this, descriptors)
    }

    #[test]
    fn balancing_does_nothing_without_a_tail_and_otherwise_projects_the_widest_other_axis() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let (this, descriptors) = balancing_rig(&mut e, &log);
        let sorts: Log = Rc::default();
        let sink = Rc::clone(&sorts);
        e.register_double(SORT_PRIMITIVES, move |_, a| {
            push(&sink, a[..4].to_vec());
            Ret::default()
        });
        let node = balancing_node(&mut e, 0);
        let r = primitives(&mut e, 9);
        let at = |i: u32| r + 0x14 * i;
        // A tail of zero primitives: the cursors stay.
        let (base, middle, c, d_copy, d, end) = (
            cell(&mut e, at(0)),
            cell(&mut e, at(1)),
            cell(&mut e, at(2)),
            cell(&mut e, at(8)),
            cell(&mut e, at(8)),
            cell(&mut e, at(8)),
        );
        e.call(
            0x00e0acb0,
            &args![this, base, middle, c, d_copy, d, end, node],
        );
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(d), at(8));
        // A tail of two: axis 1 is the widest axis other than the cut's 0.
        e.mem.set_u32(d, at(6));
        e.mem.set_u32(d_copy, at(6));
        e.call(
            0x00e0acb0,
            &args![this, base, middle, c, d_copy, d, end, node],
        );
        assert_eq!(
            *log.borrow(),
            vec![vec![0x18, descriptors + 0x20, 1, at(6), 2]]
        );
        assert_eq!(*sorts.borrow(), vec![vec![at(6), 0, 1, 0]]);
        // The groups are 1 and 4 big: the primitives of the tail are moved in, one by one.
        assert_eq!(e.mem.u32(d), at(8));
        assert_eq!(e.mem.u32(middle), at(3));
    }

    #[test]
    fn balancing_moves_a_record_to_the_smaller_group_when_the_groups_are_lopsided() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let (this, descriptors) = balancing_rig(&mut e, &log);
        // The cut's axis 2 leaves axis 1 (width 3) the widest.
        let node = balancing_node(&mut e, 2);
        let r = primitives(&mut e, 9);
        let at = |i: u32| r + 0x14 * i;
        let before: Vec<u32> = (0..9).map(|i| id(&e, at(i))).collect();
        // One record in the first group, five in the second, one in the tail.
        let (base, middle, c, d_copy, d, end) = (
            cell(&mut e, at(0)),
            cell(&mut e, at(1)),
            cell(&mut e, at(2)),
            cell(&mut e, at(7)),
            cell(&mut e, at(7)),
            cell(&mut e, at(8)),
        );
        e.call(
            0x00e0acb0,
            &args![this, base, middle, c, d_copy, d, end, node],
        );
        assert_eq!(
            *log.borrow(),
            vec![vec![0x18, descriptors + 0x20, 1, at(7), 1]]
        );
        // The first group is smaller than a quarter of the second: the last record R7 goes to
        // the middle's place R1, the record at C (R2) goes to D's place R7, and the cursors
        // advance.
        assert_eq!(id(&e, at(1)), before[7]);
        assert_eq!(id(&e, at(7)), before[2]);
        assert_eq!(e.mem.u32(middle), at(2));
        assert_eq!(e.mem.u32(c), at(3));
        assert_eq!(e.mem.u32(d), at(8));
        assert_eq!(e.mem.u32(d_copy), at(8));
    }

    #[test]
    fn splitting_keeps_the_records_in_place_when_they_are_in_their_halves() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let (this, _descriptors) = balancing_rig(&mut e, &log);
        e.register_double(FLOAT_TO_INT, |_, a| Ret {
            eax: f32::from_bits(a[0]) as i32 as u32,
            ..Ret::default()
        });
        let node = balancing_node(&mut e, 0);
        // The cut [5, 6].
        e.mem.set_f32(node + 0x64, 5.0);
        e.mem.set_f32(node + 0x68, 6.0);
        // R0 [0, 1] stays low (case 1), R1 [7, 9] is above the cut (case 2) and R2 [5.5, 5.8]
        // inside it (case 0).
        let r = primitives(&mut e, 3);
        let at = |i: u32| r + 0x14 * i;
        for (i, (low, high)) in [(0.0f32, 1.0f32), (7.0, 9.0), (5.5, 5.8)]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(at(i as u32) + 8, *low);
            e.mem.set_f32(at(i as u32) + 0xc, *high);
        }
        let before: Vec<u32> = (0..3).map(|i| id(&e, at(i))).collect();
        let description = items(&mut e, r, 3, 0);
        let left = e.mem.alloc(0x10);
        let right = e.mem.alloc(0x10);
        e.call(
            0x00e0a0b0,
            &args![this, description, node, 0u32, left, right],
        );
        let after: Vec<u32> = (0..3).map(|i| id(&e, at(i))).collect();
        assert_eq!(after, before);
        assert_eq!(e.mem.u32(left), r);
        assert_eq!(e.mem.u32(left + 4), 1);
        assert_eq!(e.mem.u32(left + 8), 0);
        assert_eq!(e.mem.u32(right), at(1));
        assert_eq!(e.mem.u32(right + 4), 2);
        assert_eq!(e.mem.u32(right + 8), 0);
    }

    #[test]
    fn the_second_count_of_the_halves_follows_the_scaled_share_of_the_first_group() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let (this, _descriptors) = balancing_rig(&mut e, &log);
        let converted: Rc<RefCell<Vec<f32>>> = Rc::default();
        let sink = Rc::clone(&converted);
        e.register_double(FLOAT_TO_INT, move |_, a| {
            sink.borrow_mut().push(f32::from_bits(a[0]));
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let node = balancing_node(&mut e, 0);
        // The cut is [0, 0] and every record lies below it: all of them stay.
        e.mem.set_f32(node + 0x64, 0.0);
        e.mem.set_f32(node + 0x68, 0.0);
        let r = primitives(&mut e, 4);
        let at = |i: u32| r + 0x14 * i;
        for i in 0..4 {
            e.mem.set_f32(at(i) + 8, -1.0);
            e.mem.set_f32(at(i) + 0xc, -1.0);
        }
        // Two records in the first group and two in the second.
        let description = items(&mut e, r, 2, 2);
        let left = e.mem.alloc(0x10);
        let right = e.mem.alloc(0x10);
        e.call(
            0x00e0a0b0,
            &args![this, description, node, 0u32, left, right],
        );
        assert_eq!(e.mem.u32(left + 4), 2);
        // 2 * 2 / 2 = 2 through hkFloatToInt, which answered 1.
        assert_eq!(*converted.borrow(), vec![2.0]);
        assert_eq!(e.mem.u32(left + 8), 1);
        assert_eq!(e.mem.u32(right + 8), 1);
        assert_eq!(e.mem.u32(right), at(3));
        assert_eq!(e.mem.u32(right + 4), 0);
    }

    #[test]
    fn a_cut_is_found_between_two_separate_primitives() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let compiler = e.mem.alloc(0x40);
        e.mem.set_u32(compiler + 8, 4);
        e.mem.set_u32(compiler + 0xc, 4);
        let queue = queue(&mut e, 8);
        let settings = settings(&mut e);
        e.mem.set_u32(this + 8, compiler);
        e.mem.set_u32(this + 0xc, queue);
        e.mem.set_u32(this + 0x20, settings);
        e.mem.set_u32(this + 0x38, 2);
        let descriptor = e.mem.alloc(0x40);
        e.mem.set_f32(descriptor + 0x10, 0.5);
        let array = primitives(&mut e, 2);
        // R0 covers [0, 1] and R1 [2, 3].
        for (i, (low, high)) in [(0.0f32, 1.0f32), (2.0, 3.0)].iter().enumerate() {
            e.mem.set_f32(array + 0x14 * i as u32 + 8, *low);
            e.mem.set_f32(array + 0x14 * i as u32 + 0xc, *high);
        }
        let description = items(&mut e, array, 2, 0);
        let range = e.mem.alloc(8);
        e.mem.set_f32(range + 4, 4.0);
        let best = e.mem.alloc(0x80);
        e.mem.set_f32(best + 0x6c, 1.0e6);
        e.call(
            0x00e09be0,
            &args![this, best, descriptor, description, range, 5u32],
        );
        // The cut lies between the end of R0 and the start of R1; the cost is the sum of the
        // axis cost 0.5, the distance term (1 - 2) * 0.25 and the polynomial of the side
        // cost for the scaled offset -0.25 from the middle of the range.
        assert_eq!(e.mem.u32(best + 0x60), descriptor);
        assert_eq!(e.mem.f32(best + 0x64), 2.0);
        assert_eq!(e.mem.f32(best + 0x68), 1.0);
        let expected = 0.5 + polynomial(-0.25, 1.0) - 0.25;
        assert!(close(e.mem.f32(best + 0x6c), expected));
        // The queue is emptied at the end.
        assert_eq!(e.mem.u32(queue), 0);
        assert_eq!(e.mem.u32(queue + 0xc), 0);
    }

    /// The pieces to build a tree over some primitives.
    struct Tree {
        this: u32,
        mediator: u32,
        settings: u32,
        assembler: u32,
        compiler: u32,
        pool: u32,
        array: u32,
        descriptors: u32,
        log: Log,
        allocations: Log,
    }

    fn tree(e: &mut Engine, count: u32, flag: u8) -> Tree {
        let log: Log = Rc::default();
        let allocations: Log = Rc::default();
        let rows: Vec<(f32, f32)> = (0..count)
            .map(|i| (3.0 * i as f32, 3.0 * i as f32 + 2.0))
            .collect();
        let mediator = mediator(e, &log, count, Rc::new(vec![rows; 3]));
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&allocations);
        e.register_double(EASY_ALLOC, move |e, a| {
            let block = e.mem.alloc(a[1]);
            push(&sink, vec![1, a[0], a[1], block]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&allocations);
        e.register_double(EASY_FREE, move |_, a| {
            push(&sink, vec![2, a[0], a[1]]);
            Ret::default()
        });
        let descriptors = e.mem.alloc(0x80);
        for axis in 0..3 {
            e.mem.set_u32(descriptors + 0x20 * axis + 0x14, axis);
        }
        e.register_double(A_KINDS, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(A_ARRAY, move |_, _| Ret {
            eax: descriptors,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(A_FINISH, move |_, a| {
            push(&sink, vec![0x30, a[1], a[2], a[3]]);
            Ret::default()
        });
        let assembler = object(e, &[(0xc, A_KINDS), (0x10, A_ARRAY), (0x14, A_FINISH)]);
        let settings = settings(e);
        let compiler = e.mem.alloc(0x40);
        e.mem.set_u32(compiler + 8, 4);
        e.mem.set_u32(compiler + 0xc, 4);
        e.mem.set_u32(compiler + 0x10, 1);
        e.mem.set_u8(compiler + 0x14, flag);
        let array = primitives(e, count);
        let nodes = e.mem.alloc(0x7c * 0x1000 + 0x100);
        let leaves = e.mem.alloc(0x64 * 0x1000 + 0x100);
        let pool = e.mem.alloc(0x10);
        e.mem.set_u32(pool, array);
        e.mem.set_u32(pool + 4, nodes);
        e.mem.set_u32(pool + 8, leaves);
        let this = e.mem.alloc(0x40);
        e.call(0x00e09ba0, &args![this]);
        Tree {
            this,
            mediator,
            settings,
            assembler,
            compiler,
            pool,
            array,
            descriptors,
            log,
            allocations,
        }
    }

    /// Connects the splitter of `t` to its parts and gives it one leaf node.
    fn wire(e: &mut Engine, t: &Tree, free_leaves: u32) -> u32 {
        let leaf = e.mem.alloc(0x80);
        e.mem.set_u32(t.this + 0x18, leaf);
        e.mem.set_u32(t.this + 0x1c, free_leaves);
        e.mem.set_u32(t.this + 0x28, t.mediator);
        e.mem.set_u32(t.this + 0x24, t.assembler);
        e.mem.set_u32(t.this + 0x8, t.compiler);
        e.mem.set_u32(t.this + 0x34, t.descriptors);
        leaf
    }

    #[test]
    fn a_single_primitive_becomes_a_leaf_of_its_parent() {
        let mut e = Engine::new();
        let t = tree(&mut e, 1, 0);
        let leaf = wire(&mut e, &t, 2);
        let description = items(&mut e, t.array, 1, 0);
        let got = e
            .call(0x00e0a270, &args![t.this, 0x40u32, description, 0u32, 0u32])
            .u32();
        assert_eq!(got, leaf);
        assert_eq!(e.mem.u32(leaf), 0x40);
        assert_eq!(e.mem.u8(leaf + 4), 1);
        assert_eq!(e.mem.u32(leaf + 0x60), t.array);
        assert!(t.log.borrow().iter().all(|row| row[0] != 0x30));
    }

    #[test]
    fn running_out_of_leaf_nodes_is_reported_to_the_assembler() {
        let mut e = Engine::new();
        let t = tree(&mut e, 1, 1);
        wire(&mut e, &t, 0);
        e.mem.set_u32(t.this + 0x2c, 0x5150);
        let description = items(&mut e, t.array, 1, 0);
        e.call(0x00e0a270, &args![t.this, 0u32, description, 0u32, 0u32]);
        let reports: Vec<Vec<u32>> = t
            .log
            .borrow()
            .iter()
            .filter(|row| row[0] == 0x30)
            .cloned()
            .collect();
        assert_eq!(reports, vec![vec![0x30, 0x5150, t.this, 0x800]]);
    }

    #[test]
    fn building_over_one_primitive_makes_a_leaf_root_and_frees_the_queue() {
        let mut e = Engine::new();
        let t = tree(&mut e, 1, 0);
        let root = e
            .call(
                0x00e0a550,
                &args![
                    t.this,
                    t.mediator,
                    t.settings,
                    t.assembler,
                    t.compiler,
                    t.pool
                ],
            )
            .u32();
        assert_eq!(e.mem.u8(root + 4), 1);
        assert_eq!(e.mem.u32(root), 0);
        assert_eq!(e.mem.u32(root + 0x60), t.array);
        // One tree node and one leaf node were made, and the leaf was used.
        assert_eq!(e.mem.u32(t.this + 0x14), 1);
        assert_eq!(e.mem.u32(t.this + 0x1c), 0);
        assert_eq!(e.call(0x00e0a0a0, &args![t.this]).u32(), 1);
        assert_eq!(e.mem.u32(t.this + 0x38), 1);
        assert_eq!(e.mem.u32(t.this + 0x20), t.settings);
        assert_eq!(e.mem.u32(t.this + 0x28), t.mediator);
        assert_eq!(e.mem.u32(t.this + 0x24), t.assembler);
        assert_eq!(e.mem.u32(t.this + 0x8), t.compiler);
        assert_eq!(e.mem.u32(t.this + 0x30), 0);
        assert_eq!(e.mem.u32(t.this + 0x34), 0);
        assert_eq!(e.mem.u32(t.this + 0xc), 0);
        // The assembler was told the root, the splitter and twice the node count.
        assert!(t.log.borrow().contains(&vec![0x30, root, t.this, 2]));
        // The queue of 6 pointers was allocated and freed again.
        let allocations = t.allocations.borrow();
        assert_eq!(allocations.len(), 2);
        assert_eq!(allocations[0][0], 1);
        assert_eq!(allocations[0][2], 24);
        assert_eq!(
            allocations[1],
            vec![2, allocations[0][1], allocations[0][3]]
        );
    }

    #[test]
    fn the_node_pools_hold_0x1000_nodes_when_the_flag_is_set() {
        let mut e = Engine::new();
        let t = tree(&mut e, 1, 1);
        let root = e
            .call(
                0x00e0a550,
                &args![
                    t.this,
                    t.mediator,
                    t.settings,
                    t.assembler,
                    t.compiler,
                    t.pool
                ],
            )
            .u32();
        assert_eq!(e.mem.u8(root + 4), 1);
        assert_eq!(e.mem.u32(t.this + 0x14), 0x1000);
        assert_eq!(e.mem.u32(t.this + 0x1c), 0xfff);
        assert_eq!(e.mem.u32(t.this + 0x38), 1);
        assert!(t.log.borrow().contains(&vec![0x30, root, t.this, 0x2000]));
    }
}
