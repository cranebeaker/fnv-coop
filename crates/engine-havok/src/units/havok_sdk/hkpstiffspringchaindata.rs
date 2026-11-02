//! `Havok SDK/hkpstiffspringchaindata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e3b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_b0a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e3b4a0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_b0a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

use super::hkpboxmotion::{mul, sub};
use super::hkpsphereshape::{copy_vec, inverse_length, load_vec, store_vec, sum3};

// The Xbox PDB has no record of the Havok classes, so the field names below describe what the
// code does and are unconfirmed. `hkpStiffSpringChainData` (the engine map names the class): the
// vtable at `+0`, a bridge-atom block at `+0x0C`, the array of links at `+0x18` (data) / `+0x1C`
// (size) / `+0x20` (capacity | flags, bit 31 = storage not owned; each element is two pivot
// vectors and the spring's rest distance, 0x30 bytes) and three floats at `+0x24`, `+0x28`
// and `+0x2C`.

/// The class's vtable (`010eb98c`) and its base class's (`0102e388`).
const CHAIN_DATA_VTABLE: u32 = 0x010e_b98c;
const BASE_VTABLE: u32 = 0x0102_e388;
/// `hkpBridgeConstraintAtom::init` (engine map `00d21ce0`): `this` is the atom at `+0x0C`, one word.
const ATOM_INIT: u32 = 0x00d2_1ce0;
/// The array growth helper (`00c90940`, cdecl: allocator, array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The prepare pass `00d6f2b0` (cdecl: four words), the output cursor start `00dd13b0` (one
/// word), the accumulator load `00dcee70`, the per-link jacobian `00dcf720` (cdecl: points,
/// input copy, cursor, one float), the chain jacobian `00dd1180` (cdecl: link count, three
/// floats, index data, base, cursor, input, output) and `hkVector4::setTransformedPos`
/// (`00c7f640`, `this` = the result, `(transform, point)`).
const PREPARE_PASS: u32 = 0x00d6_f2b0;
const JACOBIAN_CURSOR: u32 = 0x00dd_13b0;
const LOAD_ACCUMULATORS: u32 = 0x00dc_ee70;
const LINK_JACOBIAN: u32 = 0x00dc_f720;
const CHAIN_JACOBIAN: u32 = 0x00dd_1180;
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The direction used when two pivots coincide (a vector at `01268370`).
const COINCIDENT_DIRECTION: u32 = 0x0126_8370;

// Translated from 00e3af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringChainData::getType` (engine map): the constraint type, `0x64`.
pub fn hkp_stiff_spring_chain_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x64
}

// Translated from 00e3af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringChainData::getConstraintInfo` (engine map): fills the info record at `info`:
/// `+0x10` the atom block `this + 0x0C`, `+0x14` = 0x0C, `+0x00` = 0 and, with `n` the link
/// count at `+0x1C`, `+0x04` = `((n * 0x50 + 0x33) & ~0xF) + 0x10` and `+0x08` and `+0x0C`
/// both `n` (the 32-bit wrapping arithmetic of the exe).
pub fn hkp_stiff_spring_chain_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    let (this, info) = (this.addr(), info.addr());
    e.mem.set_u32(info + 0x10, this + 0xc);
    e.mem.set_u32(info + 8, 0);
    e.mem.set_u32(info + 0xc, 0);
    e.mem.set_u32(info, 0);
    e.mem.set_u32(info + 4, 0x10);
    e.mem.set_u32(info + 0x14, 0xc);
    let count = e.mem.u32(this + 0x1c);
    let size = (count.wrapping_mul(0x50).wrapping_add(0x33) & 0xffff_fff0).wrapping_add(0x10);
    e.mem.set_u32(info + 4, size);
    e.mem.set_u32(info + 8, count);
    e.mem.set_u32(info + 0xc, count);
}

// Translated from 00e3af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringChainData::getRuntimeInfo` (engine map): with `flag` set, `info + 4` is the
/// link count `n` (`+0x1C`) and `info + 0` is `n * 8`; otherwise both are 0.
pub fn hkp_stiff_spring_chain_data_get_runtime_info(
    e: &mut Engine,
    this: Ptr,
    flag: u8,
    info: Ptr,
) {
    let info = info.addr();
    if flag != 0 {
        let count = e.mem.u32(this.addr() + 0x1c);
        e.mem.set_u32(info + 4, count);
        e.mem.set_u32(info, count.wrapping_mul(8));
    } else {
        e.mem.set_u32(info + 4, 0);
        e.mem.set_u32(info, 0);
    }
}

// Translated from 00e3afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends one link to the array at `+0x18` (no engine-map name), `(this; pivot a, pivot b,
/// distance)`: when the size equals the capacity (`+0x20 & 0x3fffffff`) the array grows through
/// `00c90940` (the memory router's allocator, the array, element size 0x30); then the two
/// vectors are copied (16 bytes each) to `data + size * 0x30` and `+0x10` and the distance (an
/// x87 copy, so a signalling NaN comes out quiet) to `+0x20`.
pub fn fn_00e3afb0(e: &mut Engine, this: Ptr, pivot_a: Ptr, pivot_b: Ptr, distance: f32) {
    let array = this.addr() + 0x18;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x30u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = size.wrapping_mul(0x30).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    copy_vec(e, pivot_a.addr(), slot);
    copy_vec(e, pivot_b.addr(), slot + 0x10);
    e.mem.set_f32(slot + 0x20, x87_quiet(distance));
}

// Translated from 00e3b060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor for a loaded object (no engine-map name), `(this; flag)`: sets the vtable and,
/// when `flag` is not zero, runs the atom's init (`00d21ce0`, on `this + 0x0C`) with the word at
/// `+0x14` and then, `flag` being tested again, with `this`. Returns `this`.
pub fn fn_00e3b060(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let object = this.addr();
    e.mem.set_u32(object, CHAIN_DATA_VTABLE);
    let atom = object + 0xc;
    if flag != 0 {
        let word = e.mem.u32(atom + 8);
        e.call(ATOM_INIT, &args![atom, word]);
        if flag != 0 {
            e.call(ATOM_INIT, &args![atom, object]);
        }
    }
    this
}

// Translated from 00e3b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body (no engine-map name): sets the vtable, the link count to 0 and, when the
/// capacity word at `+0x20` has bit 31 clear, frees the array's storage through the allocator
/// (slot 0x10: data, `(capacity & 0x3fffffff) * 0x30` bytes); then the array pointer is 0, the
/// capacity word 0x80000000 and the vtable the base class's (`0102e388`).
pub fn fn_00e3b0a0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, CHAIN_DATA_VTABLE);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(object + 0x20);
    e.mem.set_u32(object + 0x1c, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(object + 0x18);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(0x30);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(object + 0x18, 0);
    e.mem.set_u32(object + 0x20, 0x8000_0000);
    e.mem.set_u32(object, BASE_VTABLE);
}

// Translated from 00e3b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStiffSpringChainData::buildJacobian` (engine map): `input` (0x80 bytes) and `output` are
/// the constraint query structures. After the prepare pass `00d6f2b0(input, output, input[0x64],
/// 8)` the input is copied to the frame, its words `+0x60` and `+0x64` cleared, `+0x20` set to
/// input word 2 and `+0x24` to 1.0, `+0x34` and `+0x3C` to the first body's offset and matrix.
/// The chain's body list (the instance at `input[0x60]`, bodies at `[0x38]`, count at `[0x3C]`)
/// gives each body's word at `+0xA4` (relative to the first body's), collected in an inline
/// array that grows through `00c90940`. For each consecutive pair the link `this.array[i]` has
/// its first pivot taken to world space by the earlier body's matrix (`+0xF0`) and its second
/// by the later one's (`00c7f640`); the direction is the first world point less the second,
/// scaled by the reciprocal square root of its squared length (`RSQRTSS`, one refinement step,
/// 0 for a zero length; the squares summed as `z + (y + x)`) and the length is the squared
/// length times that scale. A length that is not above 0 (a NaN too, by the x87 compare)
/// becomes 0 and the direction the vector at `01268370`. After the accumulators are loaded
/// (`00dcee70`) the link jacobian `00dcf720` gets the three vectors (first point, second point,
/// direction), the input copy, the output cursor (`00dd13b0`, advancing 0x30 bytes per link) and
/// the link's rest distance (`+0x20`) less the length. The chain jacobian `00dd1180` gets the
/// link count, the three floats at `+0x24`, `+0x28`, `+0x2C`, the index array, the base, the
/// cursor and both structures; finally the index array's storage is freed (allocator slot 0x10).
pub fn hkp_stiff_spring_chain_data_build_jacobian(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let this = this.addr();
    let (input, output) = (input.addr(), output.addr());
    e.with_stack(0x1c0, |e, frame| {
        let f = frame.addr();
        // Frame offsets: `+0x30` the three vectors handed to the link jacobian (the two world
        // points and the direction), `+0x60` the copy of the input (0x80 bytes), `+0x110` the
        // index array (data, size, capacity word, then 32 inline words from `+0x11C`).
        let points = f + 0x30;
        let copy = f + 0x60;
        let index_array = f + 0x110;

        let prepare_words = e.mem.u32(input + 0x64);
        e.call(PREPARE_PASS, &args![input, output, prepare_words, 8u32]);
        e.mem.set_u32(index_array + 4, 0);
        let input_bytes = e.mem.bytes(input, 0x80);
        e.mem.write(copy, &input_bytes);
        let chain_origin = e.mem.u32(input + 0x30);
        e.mem.set_u32(copy + 0x60, 0);
        e.mem.set_u32(copy + 0x64, 0);
        let instance = e.mem.u32(input + 0x60);
        e.mem.set_u32(index_array + 8, 0x8000_0020);
        e.mem.set_u32(index_array, f + 0x11c);
        let instance_body = e.mem.u32(instance + 0x14);
        let link_count = e.mem.u32(instance + 0x3c);
        let bodies = e.mem.u32(instance + 0x38);
        let first_body = e.mem.u32(bodies);
        let base = chain_origin.wrapping_sub(e.mem.u32(instance_body + 0xa4));
        let first_offset = e.mem.u32(first_body + 0xa4).wrapping_add(base);
        e.mem.set_f32(copy + 0x24, 1.0);
        e.mem.set_u32(copy + 0x34, first_offset);
        let first_relative = first_offset.wrapping_sub(base);
        e.mem.set_u32(f + 0x11c, first_relative);
        let output_start = e.mem.u32(output);
        e.mem.set_u32(copy + 0x3c, first_body.wrapping_add(0xf0));
        let links = link_count.wrapping_sub(1) as i32;
        e.mem.set_u32(index_array + 4, 1);
        let input_word_two = e.mem.u32(copy + 8);
        e.mem.set_u32(copy + 0x20, input_word_two);
        let mut cursor = e.call(JACOBIAN_CURSOR, &args![output_start]).u32();
        let mut link_offset = 0u32;
        for index in 0..links.max(0) as u32 {
            let previous_offset = e.mem.u32(copy + 0x34);
            let previous_matrix = e.mem.u32(copy + 0x3c);
            e.mem.set_u32(copy + 0x30, previous_offset);
            e.mem.set_u32(copy + 0x38, previous_matrix);
            let body_list = e.mem.u32(instance + 0x38);
            let body = e.mem.u32(body_list + index * 4 + 4);
            let relative = e.mem.u32(body + 0xa4);
            e.mem.set_u32(copy + 0x34, relative.wrapping_add(base));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(index_array + 8) & 0x3fff_ffff;
            if e.mem.u32(index_array + 4) == capacity {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_RESERVE_MORE, &args![allocator, index_array, 4u32]);
            }
            let size = e.mem.u32(index_array + 4);
            let slot = e.mem.u32(index_array).wrapping_add(size.wrapping_mul(4));
            if slot != 0 {
                e.mem.set_u32(slot, relative);
            }
            e.mem.set_u32(index_array + 4, size.wrapping_add(1));
            let matrix = body.wrapping_add(0xf0);
            e.mem.set_u32(copy + 0x3c, matrix);
            let links_data = e.mem.u32(this + 0x18);
            e.call(
                SET_TRANSFORMED_POS,
                &args![
                    points,
                    previous_matrix,
                    links_data.wrapping_add(link_offset)
                ],
            );
            e.call(
                SET_TRANSFORMED_POS,
                &args![
                    points + 0x10,
                    matrix,
                    links_data.wrapping_add(link_offset).wrapping_add(0x10)
                ],
            );
            let delta = sub(load_vec(e, points), load_vec(e, points + 0x10));
            let squared = sum3(mul(delta, delta));
            let scale = inverse_length(e, squared);
            let length = x87_quiet(squared * scale);
            // The x87 compare against 0 jumps to the fallback unless the length is above 0.
            let length = if length > 0.0 {
                store_vec(e, points + 0x20, mul([scale; 4], delta));
                length
            } else {
                let direction = load_vec(e, COINCIDENT_DIRECTION);
                store_vec(e, points + 0x20, direction);
                0.0
            };
            e.call(LOAD_ACCUMULATORS, &args![copy]);
            let links_data = e.mem.u32(this + 0x18);
            let distance = x87_quiet(e.mem.f32(links_data.wrapping_add(link_offset) + 0x20));
            let remaining = (f64::from(distance) - f64::from(length)) as f32;
            e.call(LINK_JACOBIAN, &args![points, copy, cursor, remaining]);
            cursor = cursor.wrapping_add(0x30);
            link_offset = link_offset.wrapping_add(0x30);
        }
        let setting_2c = x87_quiet_bits(e.mem.u32(this + 0x2c));
        let setting_28 = x87_quiet_bits(e.mem.u32(this + 0x28));
        let setting_24 = x87_quiet_bits(e.mem.u32(this + 0x24));
        let data = e.mem.u32(index_array);
        e.call(
            CHAIN_JACOBIAN,
            &args![
                links as u32,
                setting_24,
                setting_28,
                setting_2c,
                data,
                base,
                cursor,
                input,
                output
            ],
        );
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(index_array + 8);
        e.mem.set_u32(index_array + 4, 0);
        if capacity as i32 >= 0 {
            let data = e.mem.u32(index_array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
    });
}

// Translated from 00e3b490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte 1 at `out` (no engine-map name; a virtual method, `this` unused; the
/// constraint data's `isValid` slot).
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00d4c170).
pub fn fn_00e3b490(e: &mut Engine, _this: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_af30, hkp_stiff_spring_chain_data_get_type(Ptr) -> u32),
        entry!(
            0x00e3_af40,
            hkp_stiff_spring_chain_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00e3_af80,
            hkp_stiff_spring_chain_data_get_runtime_info(Ptr, u8, Ptr)
        ),
        entry!(0x00e3_afb0, fn_00e3afb0(Ptr, Ptr, Ptr, f32)),
        entry!(0x00e3_b060, fn_00e3b060(Ptr, u32) -> Ptr),
        entry!(0x00e3_b0a0, fn_00e3b0a0(Ptr)),
        entry!(
            0x00e3_b0f0,
            hkp_stiff_spring_chain_data_build_jacobian(Ptr, Ptr, Ptr)
        ),
        entry!(0x00e3_b490, fn_00e3b490(Ptr, Ptr) -> Ptr),
        entry!(0x00e3_b4a0, deleting_destructor_00e3b4a0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e3b4a0_frees_only_when_bit_zero_is_set() {
        check(0x00e3_b4a0, 0x00e3_b0a0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::super::hkpboxmotion::add;
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;
    /// The chain jacobian's arguments and the index array words it was handed.
    type ChainCalls = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>>;

    const FREE_DOUBLE: u32 = 0x0900_0001;

    fn bits(v: [f32; 4]) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    /// An engine with the exe's data range mapped, the constants of the length normalisation,
    /// a memory router whose allocator frees through a double at slot 0x10 and a log the
    /// doubles append `(address, arguments)` to.
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        store_vec(&mut e, COINCIDENT_DIRECTION, [0.0, 0.0, 1.0, 0.0]);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_DOUBLE, ATOM_INIT, ARRAY_RESERVE_MORE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= 1e-4 * expected.abs().max(1.0),
            "{actual} is not {expected}"
        );
    }

    #[test]
    fn the_constraint_type_is_0x64() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e3_af30, &args![0x1000u32]).u32(), 0x64);
    }

    #[test]
    fn constraint_info_sizes_the_chain_by_its_link_count() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 5);
        e.call(0x00e3_af40, &args![this, info]);
        assert_eq!(e.mem.u32(info + 0x10), this + 0xc);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(e.mem.u32(info + 4), 0x1d0);
        assert_eq!(e.mem.u32(info + 8), 5);
        assert_eq!(e.mem.u32(info + 0xc), 5);
        assert_eq!(e.mem.u32(info + 0x14), 0xc);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00e3_af40, &args![this, info]);
        assert_eq!(e.mem.u32(info + 4), 0x40);
        assert_eq!(e.mem.u32(info + 8), 0);
    }

    #[test]
    fn runtime_info_is_the_link_count_and_eight_times_it_or_zero() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x1c, 7);
        e.call(0x00e3_af80, &args![this, 1u8, info]);
        assert_eq!(e.mem.u32(info + 4), 7);
        assert_eq!(e.mem.u32(info), 56);
        e.call(0x00e3_af80, &args![this, 0u8, info]);
        assert_eq!(e.mem.u32(info + 4), 0);
        assert_eq!(e.mem.u32(info), 0);
    }

    #[test]
    fn adding_a_link_to_a_full_array_grows_it_then_stores_both_pivots_and_the_distance() {
        let (mut e, log) = setup();
        let this = e.mem.alloc(0x40);
        let old = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x18, old);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, 0x8000_0001);
        let storage = e.mem.alloc(0x90);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, 3);
            Ret::default()
        });
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        store_vec(&mut e, first, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, second, [5.0, 6.0, 7.0, 8.0]);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(0x00e3_afb0, &args![this, first, second, signalling]);
        let reserve = calls_to(&log, ARRAY_RESERVE_MORE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][1..], [this + 0x18, 0x30]);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        let slot = storage + 0x30;
        assert_eq!(bits(load_vec(&e, slot)), bits([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(bits(load_vec(&e, slot + 0x10)), bits([5.0, 6.0, 7.0, 8.0]));
        assert_eq!(e.mem.u32(slot + 0x20), 0x7fc0_0001);
        // With room left the array is not grown again.
        e.call(0x00e3_afb0, &args![this, second, first, 2.5f32]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        assert_eq!(e.mem.f32(storage + 0x60 + 0x20), 2.5);
        assert_eq!(
            bits(load_vec(&e, storage + 0x60)),
            bits([5.0, 6.0, 7.0, 8.0])
        );
    }

    #[test]
    fn the_loaded_object_constructor_runs_the_atom_init_twice_when_the_flag_is_set() {
        let (mut e, log) = setup();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x1234);
        assert_eq!(e.call(0x00e3_b060, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CHAIN_DATA_VTABLE);
        assert!(calls_to(&log, ATOM_INIT).is_empty());
        assert_eq!(e.call(0x00e3_b060, &args![this, 3u32]).u32(), this);
        assert_eq!(
            calls_to(&log, ATOM_INIT),
            vec![vec![this + 0xc, 0x1234], vec![this + 0xc, this]]
        );
    }

    #[test]
    fn the_destructor_body_frees_owned_storage_by_the_capacity() {
        let (mut e, log) = setup();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x7000);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, 4);
        e.call(0x00e3_b0a0, &args![this]);
        let freed = calls_to(&log, FREE_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [0x7000, 4 * 0x30]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn the_destructor_body_leaves_storage_it_does_not_own() {
        let (mut e, log) = setup();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x7000);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, 0x8000_0004);
        e.call(0x00e3_b0a0, &args![this]);
        assert!(calls_to(&log, FREE_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn the_byte_writer_stores_one() {
        let mut e = Engine::new();
        let out = e.mem.alloc(4);
        e.call(0x00e3_b490, &args![0x1000u32, out]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn fn_00e3b490_answers_its_result_pointer() {
        let mut e = Engine::new();
        let out = e.mem.alloc(4);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00e3_b490, &args![0x1000u32, out]).u32(),
            args![out][0]
        );
    }

    struct Scene {
        e: Engine,
        log: Log,
        this: u32,
        input: u32,
        output: u32,
        bodies: Vec<u32>,
        accumulators: Words,
        links: Words,
        chain: ChainCalls,
    }

    /// A chain of `translations.len()` bodies (body `i` moves points by `translations[i]`) and
    /// the links `(pivot a, pivot b, distance)`, with doubles over the callees that record
    /// what they were handed.
    fn scene(translations: &[[f32; 4]], links: &[([f32; 4], [f32; 4], f32)]) -> Scene {
        let (mut e, log) = setup();
        let accumulators: Words = Rc::new(RefCell::new(Vec::new()));
        let recorded_links: Words = Rc::new(RefCell::new(Vec::new()));
        let chain: ChainCalls = Rc::new(RefCell::new(Vec::new()));
        e.register_double(PREPARE_PASS, {
            let sink = Rc::clone(&log);
            move |_, a| {
                sink.borrow_mut().push((PREPARE_PASS, a.to_vec()));
                Ret::default()
            }
        });
        e.register_double(JACOBIAN_CURSOR, {
            let sink = Rc::clone(&log);
            move |_, a| {
                sink.borrow_mut().push((JACOBIAN_CURSOR, a.to_vec()));
                Ret {
                    eax: 0x9000,
                    ..Ret::default()
                }
            }
        });
        e.register_double(SET_TRANSFORMED_POS, |e, a| {
            let moved = add(load_vec(e, a[2]), load_vec(e, a[1] + 0x30));
            store_vec(e, a[0], moved);
            Ret::default()
        });
        e.register_double(LOAD_ACCUMULATORS, {
            let sink = Rc::clone(&accumulators);
            move |e, a| {
                let mut words: Vec<u32> = (0..0x20).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
                words.push(a[0]);
                sink.borrow_mut().push(words);
                Ret::default()
            }
        });
        e.register_double(LINK_JACOBIAN, {
            let sink = Rc::clone(&recorded_links);
            move |e, a| {
                let mut words = a.to_vec();
                for vector in 0..3 {
                    words.extend(bits(load_vec(e, a[0] + 0x10 * vector)));
                }
                sink.borrow_mut().push(words);
                Ret::default()
            }
        });
        e.register_double(CHAIN_JACOBIAN, {
            let sink = Rc::clone(&chain);
            move |e, a| {
                let entries: Vec<u32> = (0..=a[0]).map(|i| e.mem.u32(a[4] + 4 * i)).collect();
                sink.borrow_mut().push((a.to_vec(), entries));
                Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x30 * links.len().max(1) as u32);
        for (i, (first, second, distance)) in links.iter().enumerate() {
            let link = array + 0x30 * i as u32;
            store_vec(&mut e, link, *first);
            store_vec(&mut e, link + 0x10, *second);
            e.mem.set_f32(link + 0x20, *distance);
        }
        e.mem.set_u32(this + 0x18, array);
        e.mem.set_f32(this + 0x24, 0.5);
        e.mem.set_f32(this + 0x28, 0.25);
        e.mem.set_u32(this + 0x2c, 0x7f80_0001);
        let bodies: Vec<u32> = translations
            .iter()
            .enumerate()
            .map(|(i, translation)| {
                let body = e.mem.alloc(0x130);
                e.mem.set_u32(body + 0xa4, 100 + i as u32);
                store_vec(&mut e, body + 0x120, *translation);
                body
            })
            .collect();
        let list = e.mem.alloc(4 * bodies.len() as u32);
        for (i, body) in bodies.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *body);
        }
        let instance_body = e.mem.alloc(0xb0);
        e.mem.set_u32(instance_body + 0xa4, 40);
        let instance = e.mem.alloc(0x50);
        e.mem.set_u32(instance + 0x14, instance_body);
        e.mem.set_u32(instance + 0x38, list);
        e.mem.set_u32(instance + 0x3c, bodies.len() as u32);
        let input = e.mem.alloc(0x90);
        for i in 0..0x20 {
            e.mem.set_u32(input + 4 * i, 0x1000 + i);
        }
        e.mem.set_u32(input + 0x30, 1000);
        e.mem.set_u32(input + 0x60, instance);
        e.mem.set_u32(input + 0x64, 7);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output, 0x5000);
        Scene {
            e,
            log,
            this,
            input,
            output,
            bodies,
            accumulators,
            links: recorded_links,
            chain,
        }
    }

    #[test]
    fn build_jacobian_walks_the_links_of_a_three_body_chain() {
        let mut s = scene(
            &[[0.0; 4], [0.0, 3.0, 0.0, 0.5], [0.0, 3.0, 0.0, 0.5]],
            &[
                ([4.0, 0.0, 0.0, 0.0], [0.0; 4], 7.0),
                ([1.0, 1.0, 1.0, 0.0], [1.0, 1.0, 1.0, 0.0], 2.5),
            ],
        );
        s.e.call(0x00e3_b0f0, &args![s.this, s.input, s.output]);
        assert_eq!(
            calls_to(&s.log, PREPARE_PASS),
            vec![vec![s.input, s.output, 7, 8]]
        );
        assert_eq!(calls_to(&s.log, JACOBIAN_CURSOR), vec![vec![0x5000]]);
        let accumulators = s.accumulators.borrow();
        assert_eq!(accumulators.len(), 2);
        // The copy of the input with its words +0x60 and +0x64 cleared; +0x20 is input word 2,
        // +0x24 is 1.0; +0x30/+0x34 are the previous and current body offsets (relative offset
        // + 960); +0x38/+0x3C the previous and current matrices.
        let first = &accumulators[0];
        assert_eq!(first[0], 0x1000);
        assert_eq!(first[0x18], 0);
        assert_eq!(first[0x19], 0);
        assert_eq!(first[8], 0x1002);
        assert_eq!(first[9], 1.0f32.to_bits());
        assert_eq!(first[0xc], 1060);
        assert_eq!(first[0xd], 1061);
        assert_eq!(first[0xe], s.bodies[0] + 0xf0);
        assert_eq!(first[0xf], s.bodies[1] + 0xf0);
        let second = &accumulators[1];
        assert_eq!(second[0xc], 1061);
        assert_eq!(second[0xd], 1062);
        assert_eq!(second[0xe], s.bodies[1] + 0xf0);
        assert_eq!(second[0xf], s.bodies[2] + 0xf0);
        let links = s.links.borrow();
        assert_eq!(links.len(), 2);
        // Link 0: the points are (4, 0, 0, 0) and (0, 3, 0, 0.5), so the direction is the
        // difference over its length 5 and the remaining distance 7 - 5.
        let link = &links[0];
        assert_eq!(link[1], accumulators[0][0x20]);
        assert_eq!(link[2], 0x9000);
        close(f32::from_bits(link[3]), 2.0);
        assert_eq!(link[4..8], bits([4.0, 0.0, 0.0, 0.0]));
        assert_eq!(link[8..12], bits([0.0, 3.0, 0.0, 0.5]));
        let direction: Vec<f32> = link[12..16].iter().map(|b| f32::from_bits(*b)).collect();
        close(direction[0], 0.8);
        close(direction[1], -0.6);
        assert_eq!(direction[2], 0.0);
        close(direction[3], -0.1);
        // Link 1: the pivots coincide, so the length is 0, the direction the stored constant
        // and the whole rest distance is left.
        let link = &links[1];
        assert_eq!(link[2], 0x9030);
        assert_eq!(f32::from_bits(link[3]), 2.5);
        assert_eq!(link[12..16], bits([0.0, 0.0, 1.0, 0.0]));
        let chain = s.chain.borrow();
        assert_eq!(chain.len(), 1);
        let (words, entries) = &chain[0];
        assert_eq!(words[0], 2);
        assert_eq!(
            words[1..4],
            [0.5f32.to_bits(), 0.25f32.to_bits(), 0x7fc0_0001]
        );
        assert_eq!(words[5], 960);
        assert_eq!(words[6], 0x9000 + 2 * 0x30);
        assert_eq!(words[7..9], [s.input, s.output]);
        assert_eq!(entries[..3], [100, 101, 102]);
        // The inline index array is not freed.
        assert!(calls_to(&s.log, FREE_DOUBLE).is_empty());
    }

    #[test]
    fn build_jacobian_takes_the_fallback_direction_for_a_nan_length() {
        let mut s = scene(
            &[[0.0; 4], [0.0; 4]],
            &[([f32::NAN, 0.0, 0.0, 0.0], [0.0; 4], 3.0)],
        );
        s.e.call(0x00e3_b0f0, &args![s.this, s.input, s.output]);
        let links = s.links.borrow();
        assert_eq!(links.len(), 1);
        assert_eq!(f32::from_bits(links[0][3]), 3.0);
        assert_eq!(links[0][12..16], bits([0.0, 0.0, 1.0, 0.0]));
    }

    #[test]
    fn build_jacobian_of_a_single_body_skips_the_links() {
        let mut s = scene(&[[0.0; 4]], &[]);
        s.e.call(0x00e3_b0f0, &args![s.this, s.input, s.output]);
        assert!(s.accumulators.borrow().is_empty());
        assert!(s.links.borrow().is_empty());
        let chain = s.chain.borrow();
        assert_eq!(chain[0].0[0], 0);
        assert_eq!(chain[0].0[6], 0x9000);
        assert_eq!(chain[0].1[0], 100);
    }

    #[test]
    fn build_jacobian_grows_the_index_array_past_32_entries_and_frees_it() {
        let translations = vec![[0.0; 4]; 35];
        let links = vec![([0.0; 4], [0.0; 4], 1.0); 34];
        let mut s = scene(&translations, &links);
        let sink = Rc::clone(&s.log);
        s.e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let size = e.mem.u32(a[1] + 4);
            let bigger = e.mem.alloc(0x100);
            for i in 0..size {
                let word = e.mem.u32(e.mem.u32(a[1]) + 4 * i);
                e.mem.set_u32(bigger + 4 * i, word);
            }
            e.mem.set_u32(a[1], bigger);
            e.mem.set_u32(a[1] + 8, 0x40);
            Ret::default()
        });
        s.e.call(0x00e3_b0f0, &args![s.this, s.input, s.output]);
        let reserve = calls_to(&s.log, ARRAY_RESERVE_MORE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][2], 4);
        let chain = s.chain.borrow();
        let (words, entries) = &chain[0];
        assert_eq!(words[0], 34);
        let expected: Vec<u32> = (0..35).map(|i| 100 + i).collect();
        assert_eq!(&entries[..], &expected[..]);
        let freed = calls_to(&s.log, FREE_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1], words[4]);
        assert_eq!(freed[0][2], 0x100);
    }
}
