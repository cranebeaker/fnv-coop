//! `Havok SDK/hkpmoppagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d2f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_f380`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d2f430(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_f380, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// Profile labels of the linear cast: `TtMopp` at the start, another label at the end.
const LABEL_MOPP: u32 = 0x010c_e8e8;
const LABEL_END: u32 = 0x010c_41f8;
/// `float` `0.5` (`01016248`) and the four-lane vector `1.0` (`010ec7f0`).
const HALF: u32 = 0x0101_6248;
const ONES: u32 = 0x010e_c7f0;
/// Sign mask of the first three lanes (`010c4c00`), flips a normal.
const NORMAL_SIGN_MASK: u32 = 0x010c_4c00;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall, `RET 8`): `this = inverse(a) * b`.
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `004b4ab0` (thiscall, `RET 8`, `(transform, vector)`): writes the vector in the space of the
/// transform into the 16 bytes at `this`.
const TRANSFORM_VECTOR: u32 = 0x004b_4ab0;
/// `hkpMoppAabbCastVirtualMachine::aabbCast` (`00da8180`, thiscall, `RET 0xC`).
const AABB_CAST: u32 = 0x00da_8180;
/// The base constructor `00d2f980` (thiscall, `RET 4`).
const BASE_CONSTRUCTOR: u32 = 0x00d2_f980;
/// `00cfbbe0` (thiscall, `RET 0xC`, `(entry, type a, type b)`): registers an agent entry, a
/// block of four function pointers and two flag bytes, with the collision dispatcher.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// `hkpBvTreeAgent::updateShapeCollectionFilter` (`00d2f760`, `RET 0x10`).
const BV_TREE_UPDATE_FILTER: u32 = 0x00d2_f760;
/// `00d315f0` (`RET 0x10`): `hkpBvTreeAgent::getPenetrations` with a collector.
const BV_TREE_GET_PENETRATIONS: u32 = 0x00d3_15f0;
/// `hkpBvTreeAgent::processCollision` (`00d302d0`, `RET 0x10`).
const BV_TREE_PROCESS_COLLISION: u32 = 0x00d3_02d0;
/// Vtables written by the constructor and the destructor.
const VTABLE_AGENT: u32 = 0x010c_e930;
const VTABLE_AGENT_BASE: u32 = 0x010c_e8f4;
const VTABLE_BASE: u32 = 0x0102_e388;
const VTABLE_COLLECTOR: u32 = 0x010c_e81c;

type Lanes = [f32; 4];

fn load_lanes(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, value);
    }
}

// Translated from 00d2f0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAgent::staticLinearCast` (Xbox PDB), cdecl with five words: `(body_a, body_b, input,
/// output, start_output)`. A body has its shape at `+0` and its transform at `+8`; the input
/// has a float at `+8` (the extra radius of the cast) and a vector at `+0x50` (the path).
/// Profiles `TtMopp`, computes `T = inverse(body_b transform) * body_a transform`, asks the
/// shape of `body_a` (slot `0x1C`: `(T, radius, box)`) for its box, moves the path into the
/// space of `body_b`, and builds the query record for the MOPP AABB cast: the box center
/// `(min / 2 + max / 2)`, the center plus the path, the half extent plus the radius, and
/// the input and the two bodies (at `+0x30`, `+0x34`, `+0x38`), which it hands to
/// `hkpMoppAabbCastVirtualMachine::aabbCast(record, output, start_output)`. Profiles the end.
pub fn hkp_mopp_agent_static_linear_cast(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
    start_output: u32,
) {
    let (a, b, i) = (body_a.addr(), body_b.addr(), input.addr());
    profile_record(e, LABEL_MOPP);
    e.with_stack(0x100, |e, frame| {
        let transform = frame.addr();
        let low = frame.addr() + 0x40;
        let high = frame.addr() + 0x50;
        let moved = frame.addr() + 0x60;
        let record = frame.addr() + 0x70;
        let machine = frame.addr() + 0xb0;
        e.call(
            SET_MUL_INVERSE_MUL,
            &args![transform, e.mem.u32(b + 8), e.mem.u32(a + 8)],
        );
        let shape = e.mem.u32(a);
        let radius = x87_quiet(e.mem.f32(i + 8));
        e.vcall(shape, 0x1c, &args![transform, radius, low]);
        e.call(TRANSFORM_VECTOR, &args![moved, e.mem.u32(b + 8), i + 0x50]);
        let radius = x87_quiet(e.mem.f32(i + 8));
        let half = e.global::<f32>(HALF);
        let ones = load_lanes(e, ONES);
        let minimum = load_lanes(e, low);
        let maximum = load_lanes(e, high);
        let path = load_lanes(e, moved);
        let center: Lanes =
            std::array::from_fn(|lane| (ones[lane] - half) * minimum[lane] + maximum[lane] * half);
        let far: Lanes = std::array::from_fn(|lane| path[lane] + center[lane]);
        let extent: Lanes =
            std::array::from_fn(|lane| half * (maximum[lane] - minimum[lane]) + radius);
        store_lanes(e, record, center);
        store_lanes(e, record + 0x10, far);
        store_lanes(e, record + 0x20, extent);
        e.mem.set_u32(record + 0x30, i);
        e.mem.set_u32(record + 0x34, a);
        e.mem.set_u32(record + 0x38, b);
        e.call(AABB_CAST, &args![machine, record, output, start_output]);
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d2f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the engine map has no name), `RET 4`: runs the base constructor `00d2f980` with
/// the word and sets the vtable `010ce930`. Returns `this`.
pub fn fn_00d2f220(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, value]);
    e.mem.set_u32(this.addr(), VTABLE_AGENT);
    this
}

// Translated from 00d2f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the three MOPP agent entries with the dispatcher at `dispatcher` (the engine map
/// has no name), cdecl with that one word. Each entry is four function pointers and two flag
/// bytes, handed to `00cfbbe0(entry, type a, type b)` on a stack record: `(10, -1)` with
/// `00d2fa00, 00d2e970, 00d2e9b0, 00d2daf0` and flags `(1, 0)`, then `(-1, 10)` with
/// `00d2f9d0, 00d312f0, 00d31110, 00d2f0d0` and flags `(0, 0)`, then `(10, 10)` with
/// `00d2f240, 00d312f0, 00d31110, 00d2f0d0` and flags `(0, 1)`.
pub fn fn_00d2f2c0(e: &mut Engine, dispatcher: Ptr) {
    let entries: [([u32; 4], [u8; 2], u32, u32); 3] = [
        (
            [0x00d2_fa00, 0x00d2_e970, 0x00d2_e9b0, 0x00d2_daf0],
            [1, 0],
            10,
            0xffff_ffff,
        ),
        (
            [0x00d2_f9d0, 0x00d3_12f0, 0x00d3_1110, 0x00d2_f0d0],
            [0, 0],
            0xffff_ffff,
            10,
        ),
        (
            [0x00d2_f240, 0x00d3_12f0, 0x00d3_1110, 0x00d2_f0d0],
            [0, 1],
            10,
            10,
        ),
    ];
    e.with_stack(0x14, |e, entry| {
        for (functions, flags, first, second) in entries {
            for (index, function) in functions.into_iter().enumerate() {
                e.mem.set_u32(entry.addr() + 4 * index as u32, function);
            }
            e.mem.set_u8(entry.addr() + 0x10, flags[0]);
            e.mem.set_u8(entry.addr() + 0x11, flags[1]);
            e.call(REGISTER_AGENT, &args![dispatcher, entry, first, second]);
        }
    });
}

// Translated from 00d2f380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the agent's base part (the engine map has no name), `__fastcall`: sets the
/// vtable `010ce8f4`, empties the array at `+0xC` (12-byte entries, size at `+0x10`, capacity
/// word at `+0x14`; freed through the memory router's allocator unless not owned) and sets the
/// base vtable `0102e388`.
pub fn fn_00d2f380(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_AGENT_BASE);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x14);
    e.mem.set_u32(me + 0x10, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xc);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
        );
    }
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x14, 0x8000_0000);
    e.mem.set_u32(me, VTABLE_BASE);
}

// Translated from 00d2f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMoppAgent>::updateShapeCollectionFilter` (Xbox PDB), `RET
/// 0x10`: `hkpBvTreeAgent::updateShapeCollectionFilter` with the first two words swapped.
/// Virtual method: the first word is `this` (`ECX` in the exe). The body leaves `ECX`
/// unchanged, so the `thiscall` callee runs on this same object.
pub fn hkp_symmetric_agent_linear_cast_mopp_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    e.call(
        BV_TREE_UPDATE_FILTER,
        &args![this, second, first, third, fourth],
    );
}

// Translated from 00d2f3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A penetration query through a small collector (the engine map has no name), `RET 0x10`
/// with four words: builds a collector on the stack (vtable `010ce81c`, flag byte 0, the
/// fourth word at `+8`) and calls `00d315f0(second, first, third, collector)`.
/// Virtual method: the first word is `this` (`ECX` in the exe); the body leaves `ECX` as it
/// came, so it is the `this` of `00d315f0`.
pub fn fn_00d2f3f0(e: &mut Engine, this: u32, first: u32, second: u32, third: u32, fourth: u32) {
    e.with_stack(0xc, |e, collector| {
        let at = collector.addr();
        e.mem.set_u32(at + 8, fourth);
        e.mem.set_u8(at + 4, 0);
        e.mem.set_u32(at, VTABLE_COLLECTOR);
        // 00d2f41b: `00d315f0` (RET 0x10) is a method; ECX is this function's `this`.
        e.call(
            BV_TREE_GET_PENETRATIONS,
            &args![this, second, first, third, collector],
        );
    });
}

// Translated from 00d31610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMoppAgent>::processCollision` (Xbox PDB), `RET 0x10`
/// `(body_a, body_b, input, output)`: runs `hkpBvTreeAgent::processCollision` with the bodies
/// swapped, and turns the contact points it added (48 bytes each from the output's first
/// word; point at `+0`, normal and distance at `+0x10`) around: the point moves by
/// `distance * normal` and the normal is negated with the sign mask. When the float at `+0x3030`
/// of the output changed (NaN counts as changed), the vector at `+0x3020` is negated with the
/// mask, the nibbles of the byte at `+0x304A` are swapped, the bytes at `+0x3048` and
/// `+0x3049` are exchanged, the 16-bit entry at `+0x3040` is exchanged with the entry
/// `a + b - 1` of that array (`a`, `b` the bytes before the exchange), and when the two bytes
/// `AND` to 2 the entries at `+0x3042` and `+0x3044` are exchanged.
/// Virtual method: the first word is `this` (`ECX` in the exe); the body leaves `ECX` as it
/// came, so it is the `this` of `hkpBvTreeAgent::processCollision`.
pub fn hkp_symmetric_agent_linear_cast_mopp_agent_process_collision(
    e: &mut Engine,
    this: u32,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: Ptr,
) {
    let out = output.addr();
    let saved = x87_quiet(e.mem.f32(out + 0x3030));
    let mut point = e.mem.u32(out);
    // 00d31637: `00d302d0` (RET 0x10) reads ECX, this function's `this`.
    e.call(
        BV_TREE_PROCESS_COLLISION,
        &args![this, body_b, body_a, input, output],
    );
    while point < e.mem.u32(out) {
        let position = load_lanes(e, point);
        let normal = load_lanes(e, point + 0x10);
        let distance = normal[3];
        let moved: Lanes = std::array::from_fn(|lane| distance * normal[lane] + position[lane]);
        store_lanes(e, point, moved);
        let flipped: Lanes = std::array::from_fn(|lane| {
            f32::from_bits(normal[lane].to_bits() ^ e.mem.u32(NORMAL_SIGN_MASK + 4 * lane as u32))
        });
        store_lanes(e, point + 0x10, flipped);
        point = point.wrapping_add(0x30);
    }
    let current = x87_quiet(e.mem.f32(out + 0x3030));
    // FUCOMPP: equal skips the rest; unordered and different continue.
    if current == saved {
        return;
    }
    for lane in 0..4 {
        let word = e.mem.u32(out + 0x3020 + 4 * lane) ^ e.mem.u32(NORMAL_SIGN_MASK + 4 * lane);
        e.mem.set_u32(out + 0x3020 + 4 * lane, word);
    }
    let nibbles = e.mem.u8(out + 0x304a);
    e.mem.set_u8(out + 0x304a, nibbles.rotate_left(4));
    let second_byte = e.mem.u8(out + 0x3049);
    let first_byte = e.mem.u8(out + 0x3048);
    e.mem.set_u8(out + 0x3049, first_byte);
    e.mem.set_u8(out + 0x3048, second_byte);
    let head = e.mem.u16(out + 0x3040);
    let index = u32::from(second_byte) + u32::from(first_byte) - 1;
    let other = out.wrapping_add(0x3040).wrapping_add(index.wrapping_mul(2));
    let swapped = e.mem.u16(other);
    e.mem.set_u16(out + 0x3040, swapped);
    e.mem.set_u16(other, head);
    if e.mem.u8(out + 0x3048) & e.mem.u8(out + 0x3049) == 2 {
        let first = e.mem.u16(out + 0x3042);
        let second = e.mem.u16(out + 0x3044);
        e.mem.set_u16(out + 0x3042, second);
        e.mem.set_u16(out + 0x3044, first);
    }
}

// Translated from 00d2f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (an agent creation function of this unit, cdecl, four stack words,
/// nothing popped): `body_a`, `body_b`, `_unused_2` and `manager`. Compares the signed word at
/// +0x24 of the object at +0x14 of the shape of each collidable (the word at +0 of the body).
/// Allocates 0x40 bytes through the memory router's allocator (`00c85750`; virtual function at +4
/// of the object at router + 0x10), stores the size as a 16-bit word at +4 and runs the constructor
/// `00d2f220` on it with `manager`. When the first value is below the second it returns what the
/// constructor answers; otherwise it also stores the vtable word `010ce96c` at +0 and returns the
/// block. Ghidra made no function here; the address is an immediate operand at 00d2dcf7.
pub fn fn_00d2f240(e: &mut Engine, body_a: u32, body_b: u32, _unused_2: u32, manager: u32) -> u32 {
    let shape_a = e.mem.u32(body_a);
    let first = e.mem.u32(shape_a + 0x14);
    let shape_b = e.mem.u32(body_b);
    let second = e.mem.u32(shape_b + 0x14);
    let first_below = e.mem.i32(first + 0x24) < e.mem.i32(second + 0x24);
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let built = e.call(0x00d2_f220, &args![block, manager]).u32();
    if first_below {
        return built;
    }
    e.mem.set_u32(block, 0x010c_e96c);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_f430, deleting_destructor_00d2f430(Ptr, u8) -> Ptr),
        entry!(
            0x00d2_f0d0,
            hkp_mopp_agent_static_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d2_f220, fn_00d2f220(Ptr, u32) -> Ptr),
        entry!(0x00d2_f2c0, fn_00d2f2c0(Ptr)),
        entry!(0x00d2_f380, fn_00d2f380(Ptr)),
        entry!(
            0x00d2_f3d0,
            hkp_symmetric_agent_linear_cast_mopp_agent_update_shape_collection_filter(
                u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d2_f3f0, fn_00d2f3f0(u32, u32, u32, u32, u32)),
        entry!(
            0x00d3_1610,
            hkp_symmetric_agent_linear_cast_mopp_agent_process_collision(u32, u32, u32, u32, Ptr)
        ),
        entry!(0x00d2_f240, fn_00d2f240(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d2f430_frees_only_when_bit_zero_is_set() {
        check(0x00d2_f430, 0x00d2_f380);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const SHAPE_SLOT: u32 = 0x0900_0201;
    const FREE_SLOT: u32 = 0x0900_0202;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn log_double(e: &mut Engine, addr: u32, log: &Log, tag: u32) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            let mut row = vec![tag];
            row.extend_from_slice(a);
            sink.borrow_mut().push(row);
            Ret::default()
        });
    }

    fn install_mask(e: &mut Engine) {
        for (lane, word) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(NORMAL_SIGN_MASK + 4 * lane as u32, word);
        }
    }

    #[test]
    fn the_linear_cast_builds_the_query_record_for_the_aabb_cast() {
        let mut e = engine();
        let log: Log = Rc::default();
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x80);
        e.mem.set_u32(PROFILE_SLOT, 2);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.set_global(HALF, 0.5f32);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
        }
        log_double(&mut e, SET_MUL_INVERSE_MUL, &log, 1);
        e.register_double(SHAPE_SLOT, {
            let sink = Rc::clone(&log);
            move |e, a| {
                sink.borrow_mut().push(vec![2, a[0], a[1], a[2], a[3]]);
                for (lane, v) in [0.0f32, 2.0, 4.0, 6.0].into_iter().enumerate() {
                    e.mem.set_f32(a[3] + 4 * lane as u32, v);
                }
                for (lane, v) in [2.0f32, 4.0, 8.0, 10.0].into_iter().enumerate() {
                    e.mem.set_f32(a[3] + 0x10 + 4 * lane as u32, v);
                }
                Ret::default()
            }
        });
        e.register_double(TRANSFORM_VECTOR, {
            let sink = Rc::clone(&log);
            move |e, a| {
                sink.borrow_mut().push(vec![3, a[0], a[1], a[2]]);
                for (lane, v) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
                    e.mem.set_f32(a[0] + 4 * lane as u32, v);
                }
                Ret::default()
            }
        });
        e.register_double(AABB_CAST, {
            let sink = Rc::clone(&log);
            move |e, a| {
                let mut row = vec![4, a[1], a[2], a[3]];
                row.extend((0..14).map(|i| e.mem.u32(a[1] + 4 * i)));
                sink.borrow_mut().push(row);
                Ret::default()
            }
        });
        let shape_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape_vtable + 0x1c, SHAPE_SLOT);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, shape_vtable);
        let body_a = e.mem.alloc(0x20);
        e.mem.set_u32(body_a, shape);
        e.mem.set_u32(body_a + 8, 0xa000);
        let body_b = e.mem.alloc(0x20);
        e.mem.set_u32(body_b + 8, 0xb000);
        let input = e.mem.alloc(0x80);
        e.mem.set_f32(input + 8, 0.25);
        e.call(
            0x00d2f0d0,
            &args![body_a, body_b, input, 0x7001u32, 0x7002u32],
        );
        let log = log.borrow();
        // inverse(b transform) * a transform, then the shape's box with the radius.
        assert_eq!(log[0][2..], [0xb000, 0xa000]);
        let transform = log[0][1];
        assert_eq!(
            log[1],
            vec![2, shape, transform, 0.25f32.to_bits(), log[1][4]]
        );
        let low = log[1][4];
        assert_eq!(low, transform + 0x40);
        // The path is moved into the space of body b.
        assert_eq!(log[2][2..], [0xb000, input + 0x50]);
        // The query record.
        let record = log[3][1];
        assert_eq!(log[3][2..4], [0x7001, 0x7002]);
        let words: Vec<f32> = log[3][4..16].iter().map(|w| f32::from_bits(*w)).collect();
        // Center, center + path, half extent + radius.
        assert_eq!(words[0..4], [1.0, 3.0, 6.0, 8.0]);
        assert_eq!(words[4..8], [11.0, 23.0, 36.0, 48.0]);
        assert_eq!(words[8..12], [1.25, 1.25, 2.25, 2.25]);
        assert_eq!(log[3][16..18], [input, body_a]);
        assert_ne!(record, 0);
        assert_eq!(log.len(), 4);
        // Profile records at the start and the end.
        assert_eq!(e.mem.u32(stream + 0x20), LABEL_MOPP);
        assert_eq!(e.mem.u32(stream + 0x2c), LABEL_END);
    }

    #[test]
    fn the_constructor_runs_the_base_constructor_and_sets_the_vtable() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, BASE_CONSTRUCTOR, &log, 1);
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00d2f220, &args![this, 0x55u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![vec![1, this, 0x55]]);
        assert_eq!(e.mem.u32(this), VTABLE_AGENT);
    }

    #[test]
    fn three_entries_are_registered_with_their_types() {
        let mut e = engine();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut row = a.to_vec();
            row.extend((0..4).map(|i| e.mem.u32(a[1] + 4 * i)));
            row.push(u32::from(e.mem.u8(a[1] + 0x10)));
            row.push(u32::from(e.mem.u8(a[1] + 0x11)));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        e.call(0x00d2f2c0, &args![0x1234u32]);
        let rows = log.borrow();
        let tail = |i: usize| rows[i][4..].to_vec();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0], 0x1234);
        assert_eq!(rows[0][2..4], [10, 0xffff_ffff]);
        assert_eq!(tail(0), vec![0xd2fa00, 0xd2e970, 0xd2e9b0, 0xd2daf0, 1, 0]);
        assert_eq!(rows[1][2..4], [0xffff_ffff, 10]);
        assert_eq!(tail(1), vec![0xd2f9d0, 0xd312f0, 0xd31110, 0xd2f0d0, 0, 0]);
        assert_eq!(rows[2][2..4], [10, 10]);
        assert_eq!(tail(2), vec![0xd2f240, 0xd312f0, 0xd31110, 0xd2f0d0, 0, 1]);
    }

    #[test]
    fn the_base_destructor_frees_an_owned_array_and_resets_the_vtable() {
        let mut e = engine();
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, FREE_SLOT, &log, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x4000);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x14, 5);
        e.call(0x00d2f380, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![1, allocator, 0x4000, 60]]);
        assert_eq!(e.mem.u32(this), VTABLE_BASE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        // Not owned: nothing is freed.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0xc, 0x4000);
        e.mem.set_u32(this + 0x14, 0x8000_0005);
        e.call(0x00d2f380, &args![this]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn update_shape_collection_filter_swaps_the_first_two_words() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, BV_TREE_UPDATE_FILTER, &log, 1);
        e.call(0x00d2f3d0, &args![VCALL_THIS, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(*log.borrow(), vec![vec![1, VCALL_THIS, 2, 1, 3, 4]]);
    }

    #[test]
    fn get_penetrations_wraps_the_fourth_word_in_a_collector() {
        let mut e = engine();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(BV_TREE_GET_PENETRATIONS, move |e, a| {
            sink.borrow_mut().push(vec![
                a[0],
                a[1],
                a[2],
                a[3],
                e.mem.u32(a[4]),
                u32::from(e.mem.u8(a[4] + 4)),
                e.mem.u32(a[4] + 8),
            ]);
            Ret::default()
        });
        e.call(0x00d2f3f0, &args![VCALL_THIS, 1u32, 2u32, 3u32, 0x44u32]);
        assert_eq!(
            *log.borrow(),
            vec![vec![VCALL_THIS, 2, 1, 3, VTABLE_COLLECTOR, 0, 0x44]]
        );
    }

    /// An output record with two contact points, the base processing double adding them.
    fn process_rig(before: f32, after: f32, counts: (u8, u8)) -> (Engine, u32, u32, Log) {
        let mut e = engine();
        install_mask(&mut e);
        let log: Log = Rc::default();
        let output = e.mem.alloc(0x3100);
        let points = e.mem.alloc(0x100);
        e.mem.set_u32(output, points);
        e.mem.set_f32(output + 0x3030, before);
        for (lane, v) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(output + 0x3020 + 4 * lane as u32, v);
        }
        e.mem.set_u8(output + 0x304a, 0x12);
        e.mem.set_u8(output + 0x3048, counts.0);
        e.mem.set_u8(output + 0x3049, counts.1);
        for (i, word) in [0x1111u16, 0x2222, 0x3333, 0x4444].into_iter().enumerate() {
            e.mem.set_u16(output + 0x3040 + 2 * i as u32, word);
        }
        let sink = Rc::clone(&log);
        e.register_double(BV_TREE_PROCESS_COLLISION, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let cursor = e.mem.u32(a[4]);
            for (lane, v) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(cursor + 4 * lane as u32, v);
            }
            for (lane, v) in [0.5f32, 0.0, 0.0, 2.0].into_iter().enumerate() {
                e.mem.set_f32(cursor + 0x10 + 4 * lane as u32, v);
            }
            for (lane, v) in [5.0f32, 5.0, 5.0, 5.0].into_iter().enumerate() {
                e.mem.set_f32(cursor + 0x30 + 4 * lane as u32, v);
            }
            for (lane, v) in [0.0f32, 1.0, 0.0, -1.0].into_iter().enumerate() {
                e.mem.set_f32(cursor + 0x40 + 4 * lane as u32, v);
            }
            e.mem.set_u32(a[4], cursor + 0x60);
            e.mem.set_f32(a[4] + 0x3030, after);
            Ret::default()
        });
        (e, output, points, log)
    }

    fn lanes(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    #[test]
    fn process_collision_turns_the_new_contact_points_around() {
        let (mut e, output, points, log) = process_rig(0.25, 0.25, (2, 2));
        e.call(
            0x00d31610,
            &args![VCALL_THIS, 0xaau32, 0xbbu32, 0xccu32, output],
        );
        // The base runs with the bodies swapped.
        assert_eq!(
            *log.borrow(),
            vec![vec![VCALL_THIS, 0xbb, 0xaa, 0xcc, output]]
        );
        assert_eq!(e.mem.u32(output), points + 0x60);
        // Point + distance * normal, and the negated normal.
        assert_eq!(lanes(&e, points), [2.0, 2.0, 3.0, 8.0]);
        assert_eq!(e.mem.u32(points + 0x10), (-0.5f32).to_bits());
        assert_eq!(e.mem.u32(points + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(points + 0x1c), 2.0f32.to_bits());
        assert_eq!(lanes(&e, points + 0x30), [5.0, 4.0, 5.0, 6.0]);
        // The float did not change: nothing else moves.
        assert_eq!(lanes(&e, output + 0x3020), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u8(output + 0x304a), 0x12);
    }

    #[test]
    fn a_changed_time_negates_the_vector_and_swaps_the_slots() {
        let (mut e, output, _points, _log) = process_rig(0.25, 0.5, (2, 2));
        e.call(0x00d31610, &args![VCALL_THIS, 0u32, 0u32, 0u32, output]);
        assert_eq!(lanes(&e, output + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(e.mem.u8(output + 0x304a), 0x21);
        assert_eq!(e.mem.u8(output + 0x3048), 2);
        assert_eq!(e.mem.u8(output + 0x3049), 2);
        // Entry 0 and entry 2 + 2 - 1 = 3 are exchanged, then (the bytes AND to 2) 1 and 2.
        assert_eq!(e.mem.u16(output + 0x3040), 0x4444);
        assert_eq!(e.mem.u16(output + 0x3046), 0x1111);
        assert_eq!(e.mem.u16(output + 0x3042), 0x3333);
        assert_eq!(e.mem.u16(output + 0x3044), 0x2222);
    }

    #[test]
    fn a_nan_time_counts_as_changed_and_other_byte_pairs_skip_the_last_exchange() {
        let (mut e, output, _points, _log) = process_rig(0.25, f32::NAN, (1, 2));
        e.call(0x00d31610, &args![VCALL_THIS, 0u32, 0u32, 0u32, output]);
        // The bytes are exchanged: 2, 1; entry 0 swaps with entry 1 + 2 - 1 = 2.
        assert_eq!(e.mem.u8(output + 0x3048), 2);
        assert_eq!(e.mem.u8(output + 0x3049), 1);
        assert_eq!(e.mem.u16(output + 0x3040), 0x3333);
        assert_eq!(e.mem.u16(output + 0x3044), 0x1111);
        // 2 AND 1 is not 2.
        assert_eq!(e.mem.u16(output + 0x3042), 0x2222);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// A collidable whose shape's object at +0x14 holds `key` at +0x24.
    fn body_with_key(e: &mut Engine, key: i32) -> u32 {
        let body = e.mem.alloc(0x10);
        let shape = e.mem.alloc(0x40);
        let code = e.mem.alloc(0x40);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(shape + 0x14, code);
        e.mem.set_i32(code + 0x24, key);
        body
    }

    /// The constructor sees the block and the manager; with the first key below the second its answer
    /// comes back and the vtable word is not touched.
    #[test]
    fn fn_00d2f240_keeps_the_constructors_answer_when_the_first_key_is_lower() {
        let (mut e, sizes) = allocator_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d2_f220, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            e.mem.set_u32(a[0], 0x1111);
            Ret {
                eax: 0xabcd,
                ..Ret::default()
            }
        });
        let first = body_with_key(&mut e, -5);
        let second = body_with_key(&mut e, 3);
        let result = e
            .call(0x00d2_f240, &args![first, second, 0u32, 0x7777u32])
            .u32();
        assert_eq!(result, 0xabcd);
        assert_eq!(*sizes.borrow(), vec![0x40]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1], 0x7777);
        assert_eq!(seen[0].1, 0x40);
        assert_eq!(e.mem.u32(seen[0].0[0]), 0x1111);
    }

    /// An equal or higher first key stores the second vtable word and returns the block.
    #[test]
    fn fn_00d2f240_swaps_the_vtable_when_the_first_key_is_not_lower() {
        for (a, b) in [(3, 3), (4, 3), (0, -1)] {
            let (mut e, _) = allocator_engine();
            e.register_double(0x00d2_f220, |e, a| {
                e.mem.set_u32(a[0], 0x1111);
                Ret {
                    eax: 0xabcd,
                    ..Ret::default()
                }
            });
            let first = body_with_key(&mut e, a);
            let second = body_with_key(&mut e, b);
            let result = e
                .call(0x00d2_f240, &args![first, second, 0u32, 0x7777u32])
                .u32();
            assert_ne!(result, 0xabcd);
            assert_eq!(e.mem.u32(result), 0x010c_e96c);
        }
    }
}
