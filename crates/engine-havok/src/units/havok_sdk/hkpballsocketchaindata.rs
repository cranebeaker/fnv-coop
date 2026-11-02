//! `Havok SDK/hkpballsocketchaindata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// The Xbox PDB has no record of the Havok classes, so the field names below describe what the
// code does and are unconfirmed. `hkpBallSocketChainData` (the engine map names the class):
// the vtable at `+0`, a bridge-atom block at `+0x0C`, the array of pivot pairs at `+0x18`
// (data) / `+0x1C` (size) / `+0x20` (capacity | flags, bit 31 = storage not owned; each element
// is two vectors, 0x20 bytes) and four floats at `+0x24`, `+0x28`, `+0x2C`, `+0x30`.
use super::hkpsphereshape::{load_vec, store_vec, Vec4};

/// The class's vtable (`010d0c8c`) and its base class's (`0102e388`).
const CHAIN_DATA_VTABLE: u32 = 0x010d_0c8c;
const BASE_VTABLE: u32 = 0x0102_e388;
/// `hkpBridgeConstraintAtom::init` (engine map `00d21ce0`): `this` is the atom at `+0x0C`, one word.
const ATOM_INIT: u32 = 0x00d2_1ce0;
/// The array growth helper (`00c90940`, cdecl: allocator, array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Constants the constructor copies into the settings at `+0x24`, `+0x2C` and `+0x30`.
const SETTING_A: u32 = 0x0101_8180;
const SETTING_C: u32 = 0x010d_0c80;
const SETTING_D: u32 = 0x0101_e2bc;

/// The pivot-point transform pass `00d6f2b0` (cdecl: four words), the cursor start
/// `00dd13c0` (one word), the accumulator load `00dcee70`, the per-link jacobian
/// `00dcfc00` (cdecl, five words) and the chain jacobian `00dd0ad0` (cdecl, nine words).
const PREPARE_PASS: u32 = 0x00d6_f2b0;
const JACOBIAN_CURSOR: u32 = 0x00dd_13c0;
const LOAD_ACCUMULATORS: u32 = 0x00dc_ee70;
const LINK_JACOBIAN: u32 = 0x00dc_fc00;
const CHAIN_JACOBIAN: u32 = 0x00dd_0ad0;

// Translated from 00d53c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallSocketChainData::getType` (engine map): the constraint type, `0x65`.
pub fn hkp_ball_socket_chain_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x65
}

// Translated from 00d53ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallSocketChainData::getConstraintInfo` (engine map): fills the info record at `info`:
/// `+0x10` the atom block `this + 0x0C`, `+0x14` = 0x0C, `+0x00` = 0 and, with `n` the pivot
/// count at `+0x1C`, `+0x04` = `((n * 0x134 + 0x33) & ~0xF) + 0x10` and `+0x08` and `+0x0C`
/// both `3 * n` (the 32-bit wrapping arithmetic of the exe).
pub fn hkp_ball_socket_chain_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    let (this, info) = (this.addr(), info.addr());
    e.mem.set_u32(info + 0x10, this + 0xc);
    e.mem.set_u32(info + 8, 0);
    e.mem.set_u32(info + 0xc, 0);
    e.mem.set_u32(info, 0);
    e.mem.set_u32(info + 4, 0x10);
    e.mem.set_u32(info + 0x14, 0xc);
    let count = e.mem.u32(this + 0x1c);
    let triple = count.wrapping_add(count.wrapping_mul(2));
    let size = (count.wrapping_mul(0x134).wrapping_add(0x33) & 0xffff_fff0).wrapping_add(0x10);
    e.mem.set_u32(info + 4, size);
    e.mem.set_u32(info + 8, triple);
    e.mem.set_u32(info + 0xc, triple);
}

// Translated from 00d53cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallSocketChainData::getRuntimeInfo` (engine map): with `flag` set, `info + 4` is `3 * n`
/// and `info + 0` is `3 * n * 8` (`n` the pivot count at `+0x1C`); otherwise both are 0.
pub fn hkp_ball_socket_chain_data_get_runtime_info(e: &mut Engine, this: Ptr, flag: u8, info: Ptr) {
    let info = info.addr();
    if flag != 0 {
        let count = e.mem.u32(this.addr() + 0x1c);
        let triple = count.wrapping_add(count.wrapping_mul(2));
        e.mem.set_u32(info + 4, triple);
        e.mem.set_u32(info, triple.wrapping_mul(8));
    } else {
        e.mem.set_u32(info + 4, 0);
        e.mem.set_u32(info, 0);
    }
}

// Translated from 00d53d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends one pivot pair to the array at `+0x18` (no engine-map name): when the size equals
/// the capacity (`+0x20 & 0x3fffffff`) the array grows through `00c90940` (the memory
/// router's allocator, the array, element size 0x20); then the two vectors `first` and `second`
/// are copied (16 bytes each) to `data + size * 0x20` and `data + size * 0x20 + 0x10`.
pub fn fn_00d53d30(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let array = this.addr() + 0x18;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x20u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = (size << 5).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let first_vector = load_bits_16(e, first.addr());
    store_bits_16(e, slot, first_vector);
    let second_vector = load_bits_16(e, second.addr());
    store_bits_16(e, slot + 0x10, second_vector);
}

/// A 16-byte `MOVAPS` load, bit for bit.
fn load_bits_16(e: &Engine, addr: u32) -> [u32; 4] {
    std::array::from_fn(|lane| e.mem.u32(addr + 4 * lane as u32))
}

/// A 16-byte `MOVAPS` store, bit for bit.
fn store_bits_16(e: &mut Engine, addr: u32, v: [u32; 4]) {
    for (lane, word) in v.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, word);
    }
}

// Translated from 00d53d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallSocketChainData::hkpBallSocketChainData` (engine map): reference count 1, user data
/// 0, vtable, the atom word at `+0x0C` = 1, an empty pivot array (`+0x18`, `+0x1C` = 0,
/// `+0x20` = 0x80000000), the settings copied through the x87 stack from `01018180` (`+0x24`),
/// 1.0 (`+0x28`), `010d0c80` (`+0x2C`) and `0101e2bc` (`+0x30`), then the atom's init
/// (`00d21ce0`, `this + 0x0C`, argument `this`). Returns `this`.
pub fn hkp_ball_socket_chain_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    let setting_a = x87_quiet_bits(e.mem.u32(SETTING_A));
    e.mem.set_u16(object + 6, 1);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object, CHAIN_DATA_VTABLE);
    e.mem.set_u16(object + 0xc, 1);
    e.mem.set_u32(object + 0x18, 0);
    e.mem.set_u32(object + 0x1c, 0);
    e.mem.set_u32(object + 0x20, 0x8000_0000);
    e.mem.set_u32(object + 0x24, setting_a);
    e.mem.set_f32(object + 0x28, 1.0);
    let setting_c = x87_quiet_bits(e.mem.u32(SETTING_C));
    e.mem.set_u32(object + 0x2c, setting_c);
    let setting_d = x87_quiet_bits(e.mem.u32(SETTING_D));
    e.mem.set_u32(object + 0x30, setting_d);
    e.call(ATOM_INIT, &args![object + 0xc, object]);
    this
}

// Translated from 00d53df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor for a loaded object (no engine-map name): sets the vtable and, when `flag` is
/// not zero, runs the atom's init (`00d21ce0`, on `this + 0x0C`) with the word at `+0x14`
/// and then, `flag` being tested again, with `this`. Returns `this`.
pub fn fn_00d53df0(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
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

// Translated from 00d53e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body (no engine-map name): sets the vtable, the pivot count to 0 and, when
/// the capacity word at `+0x20` has bit 31 clear, frees the array's storage through the
/// allocator (slot 0x10: data, `(capacity & 0x3fffffff) << 5` bytes); then the array pointer
/// is 0, the capacity word 0x80000000 and the vtable the base class's (`0102e388`).
pub fn fn_00d53e30(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, CHAIN_DATA_VTABLE);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(object + 0x20);
    e.mem.set_u32(object + 0x1c, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(object + 0x18);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 5]);
    }
    e.mem.set_u32(object + 0x18, 0);
    e.mem.set_u32(object + 0x20, 0x8000_0000);
    e.mem.set_u32(object, BASE_VTABLE);
}

/// `v * m` for a point: `((v.y * row1 + v.x * row0) + (v.z * row2 + row3))` lane by lane, the
/// `SHUFPS`/`MULPS`/`ADDPS` sequence the exe uses (matrix rows at `matrix + 0x10 * row`).
fn transform_point(e: &Engine, v: Vec4, matrix: u32) -> Vec4 {
    let row = |index: u32| load_vec(e, matrix + 0x10 * index);
    let (r0, r1, r2, r3) = (row(0), row(1), row(2), row(3));
    std::array::from_fn(|lane| (v[1] * r1[lane] + v[0] * r0[lane]) + (v[2] * r2[lane] + r3[lane]))
}

// Translated from 00d53e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallSocketChainData::buildJacobian` (engine map): `input` (0x80 bytes) and `output`
/// are the constraint query structures. After the prepare pass `00d6f2b0(input, output,
/// input[0x64], 8)` the input is copied to the frame, its words `+0x60` and `+0x64` cleared,
/// `+0x20` set to input word 2 and `+0x24` to 1.0, `+0x30` and `+0x34` and `+0x3C` to the
/// first body's values. The chain's body list (the instance at `input[0x60]`, bodies at
/// `[0x38]`, count at `[0x3C]`) gives each body's word at `+0xA4` (relative to the
/// first body's), collected in an inline array that grows through `00c90940`; for each
/// consecutive pair the pivot pair `this.array[i]` is transformed by the earlier body's
/// matrix (`+0xF0`) and the later one's, the accumulators are loaded (`00dcee70`) and the
/// link jacobian `00dcfc00` is built with `[+0x30]` and the output cursor (`00dd13c0`,
/// advancing 0x90 bytes per link); the chain jacobian `00dd0ad0` gets the link count, the
/// three settings at `+0x24`, `+0x28`, `+0x2C`, the index array, the offset, the cursor and
/// both structures; finally the index array's storage is freed (allocator slot 0x10).
pub fn hkp_ball_socket_chain_data_build_jacobian(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let this = this.addr();
    let (input, output) = (input.addr(), output.addr());
    e.with_stack(0x160, |e, frame| {
        let f = frame.addr();
        // Frame offsets (from the aligned frame base): `+0x0C` the output cursor, `+0x10` the
        // link index, `+0x14` the byte offset into the pivot array, `+0x18` and `+0x1C` the
        // current body's offset and pointer, `+0x20` the chain instance, `+0x24` the copy of
        // the input (0x80 bytes), `+0xA4` and `+0xB4` the two transformed pivots, `+0xC4` the
        // index array (data, size, capacity word, then 32 inline words from `+0xD0`).
        let copy = f + 0x24;
        let pivots = [f + 0xb4, f + 0xa4];
        let index_array = f + 0xc4;

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
        e.mem.set_u32(f + 0x20, instance);
        e.mem.set_u32(index_array, f + 0xd0);
        let instance_body = e.mem.u32(instance + 0x14);
        let link_count = e.mem.u32(instance + 0x3c);
        let bodies = e.mem.u32(instance + 0x38);
        let first_body = e.mem.u32(bodies);
        let base = chain_origin.wrapping_sub(e.mem.u32(instance_body + 0xa4));
        let first_offset = e.mem.u32(first_body + 0xa4).wrapping_add(base);
        e.mem.set_f32(copy + 0x24, 1.0);
        e.mem.set_u32(copy + 0x34, first_offset);
        let first_relative = first_offset.wrapping_sub(base);
        e.mem.set_u32(f + 0xd0, first_relative);
        let output_start = e.mem.u32(output);
        e.mem.set_u32(copy + 0x3c, first_body.wrapping_add(0xf0));
        let input_word_two = e.mem.u32(copy + 8);
        let links = link_count.wrapping_sub(1) as i32;
        e.mem.set_u32(index_array + 4, 1);
        e.mem.set_u32(copy + 0x20, input_word_two);
        let cursor = e.call(JACOBIAN_CURSOR, &args![output_start]).u32();
        e.mem.set_u32(f + 0x0c, cursor);
        e.mem.set_u32(f + 0x10, 0);
        if links > 0 {
            e.mem.set_u32(f + 0x14, 0);
            loop {
                let index = e.mem.u32(f + 0x10);
                let previous_offset = e.mem.u32(copy + 0x34);
                let previous_matrix = e.mem.u32(copy + 0x3c);
                e.mem.set_u32(copy + 0x30, previous_offset);
                e.mem.set_u32(copy + 0x38, previous_matrix);
                let body_list = e.mem.u32(e.mem.u32(f + 0x20) + 0x38);
                let body = e.mem.u32(body_list + index * 4 + 4);
                e.mem.set_u32(f + 0x1c, body);
                let relative = e.mem.u32(body + 0xa4);
                e.mem.set_u32(copy + 0x34, relative.wrapping_add(base));
                e.mem.set_u32(f + 0x18, relative);
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
                let pairs = e.mem.u32(this + 0x18);
                let pair = e.mem.u32(f + 0x14).wrapping_add(pairs);
                let matrix = body.wrapping_add(0xf0);
                e.mem.set_u32(copy + 0x3c, matrix);
                let local = load_vec(e, pair);
                let moved = transform_point(e, local, previous_matrix);
                store_vec(e, f + 0xb4, moved);
                let local = load_vec(e, pair + 0x10);
                let moved = transform_point(e, local, matrix);
                store_vec(e, f + 0xa4, moved);
                e.call(LOAD_ACCUMULATORS, &args![copy]);
                let setting = x87_quiet_bits(e.mem.u32(this + 0x30));
                let cursor = e.mem.u32(f + 0x0c);
                e.call(
                    LINK_JACOBIAN,
                    &args![pivots[0], pivots[1], setting, copy, cursor],
                );
                let index = e.mem.u32(f + 0x10);
                e.mem.set_u32(f + 0x0c, cursor.wrapping_add(0x90));
                let offset = e.mem.u32(f + 0x14);
                e.mem.set_u32(f + 0x14, offset.wrapping_add(0x20));
                e.mem.set_u32(f + 0x10, index.wrapping_add(1));
                if index.wrapping_add(1) as i32 >= links {
                    break;
                }
            }
        }
        let setting_2c = x87_quiet_bits(e.mem.u32(this + 0x2c));
        let setting_28 = x87_quiet_bits(e.mem.u32(this + 0x28));
        let setting_24 = x87_quiet_bits(e.mem.u32(this + 0x24));
        let data = e.mem.u32(index_array);
        let cursor = e.mem.u32(f + 0x0c);
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

// Translated from 00d54150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d5_3e30`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d54150(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d5_3e30, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_3c90, hkp_ball_socket_chain_data_get_type(Ptr) -> u32),
        entry!(
            0x00d5_3ca0,
            hkp_ball_socket_chain_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00d5_3cf0,
            hkp_ball_socket_chain_data_get_runtime_info(Ptr, u8, Ptr)
        ),
        entry!(0x00d5_3d30, fn_00d53d30(Ptr, Ptr, Ptr)),
        entry!(0x00d5_3d90, hkp_ball_socket_chain_data_construct(Ptr) -> Ptr),
        entry!(0x00d5_3df0, fn_00d53df0(Ptr, u32) -> Ptr),
        entry!(0x00d5_3e30, fn_00d53e30(Ptr)),
        entry!(
            0x00d5_3e80,
            hkp_ball_socket_chain_data_build_jacobian(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d5_4150, deleting_destructor_00d54150(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d54150_frees_only_when_bit_zero_is_set() {
        check(0x00d5_4150, 0x00d5_3e30);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const FREE_DOUBLE: u32 = 0x0900_0001;

    fn bits(v: Vec4) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    /// An engine with the exe's data range mapped, a memory router whose allocator frees
    /// through a double at slot 0x10, and a log the doubles append `(address, arguments)` to.
    fn setup() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
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
        log_callees(&mut e, &log, &[FREE_DOUBLE, ATOM_INIT, ARRAY_RESERVE_MORE]);
        (e, log, allocator)
    }

    /// Doubles over callees that log their arguments and return 0.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn the_constraint_type_is_0x65() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d5_3c90, &args![0x1000u32]).u32(), 0x65);
    }

    #[test]
    fn constraint_info_sizes_the_chain_by_its_pivot_count() {
        let (mut e, _, _) = setup();
        let this = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 5);
        e.call(0x00d5_3ca0, &args![this, info]);
        assert_eq!(e.mem.u32(info + 0x10), this + 0xc);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(e.mem.u32(info + 4), 0x640);
        assert_eq!(e.mem.u32(info + 8), 15);
        assert_eq!(e.mem.u32(info + 0xc), 15);
        assert_eq!(e.mem.u32(info + 0x14), 0xc);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00d5_3ca0, &args![this, info]);
        assert_eq!(e.mem.u32(info + 4), 0x40);
        assert_eq!(e.mem.u32(info + 8), 0);
    }

    #[test]
    fn runtime_info_is_empty_without_the_flag() {
        let (mut e, _, _) = setup();
        let this = e.mem.alloc(0x40);
        let info = e.mem.alloc(8);
        e.mem.set_u32(this + 0x1c, 4);
        e.mem.set_u32(info, 0x1111);
        e.mem.set_u32(info + 4, 0x2222);
        e.call(0x00d5_3cf0, &args![this, 0u8, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
        e.call(0x00d5_3cf0, &args![this, 1u8, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (96, 12));
    }

    #[test]
    fn push_pair_copies_both_vectors_after_the_last_element() {
        let (mut e, log, _) = setup();
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, 4);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        store_bits_16(&mut e, first, [1, 2, 3, 0x7f80_0001]);
        store_bits_16(&mut e, second, [5, 6, 7, 8]);
        e.call(0x00d5_3d30, &args![this, first, second]);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        assert_eq!(load_bits_16(&e, data + 0x20), [1, 2, 3, 0x7f80_0001]);
        assert_eq!(load_bits_16(&e, data + 0x30), [5, 6, 7, 8]);
        assert!(calls_to(&log, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn push_pair_grows_a_full_array_first() {
        let (mut e, log, allocator) = setup();
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x1c, 2);
        // The compare uses the capacity masked with 0x3fffffff: the top flag bit is ignored.
        e.mem.set_u32(this + 0x20, 0x8000_0002);
        let bigger = e.mem.alloc(0x100);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            e.mem.set_u32(a[1], bigger);
            Ret::default()
        });
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        store_bits_16(&mut e, first, [1, 2, 3, 4]);
        store_bits_16(&mut e, second, [5, 6, 7, 8]);
        e.call(0x00d5_3d30, &args![this, first, second]);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE_MORE),
            vec![vec![allocator, this + 0x18, 0x20]]
        );
        assert_eq!(load_bits_16(&e, bigger + 0x40), [1, 2, 3, 4]);
        assert_eq!(load_bits_16(&e, bigger + 0x50), [5, 6, 7, 8]);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
    }

    #[test]
    fn construct_sets_the_fields_and_runs_the_atom_init() {
        let (mut e, log, _) = setup();
        e.mem.set_u32(SETTING_A, 0x7f80_0001);
        e.mem.set_u32(SETTING_C, 0x3e80_0000);
        e.mem.set_u32(SETTING_D, 0x3f00_0000);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 8, 0xdead);
        let got = e.call(0x00d5_3d90, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), CHAIN_DATA_VTABLE);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x24), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x28), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x2c), 0x3e80_0000);
        assert_eq!(e.mem.u32(this + 0x30), 0x3f00_0000);
        assert_eq!(calls_to(&log, ATOM_INIT), vec![vec![this + 0xc, this]]);
    }

    #[test]
    fn loaded_constructor_runs_the_atom_init_twice_only_with_the_flag() {
        let (mut e, log, _) = setup();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x14, 0x77);
        assert_eq!(e.call(0x00d5_3df0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CHAIN_DATA_VTABLE);
        assert!(calls_to(&log, ATOM_INIT).is_empty());
        assert_eq!(e.call(0x00d5_3df0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&log, ATOM_INIT),
            vec![vec![this + 0xc, 0x77], vec![this + 0xc, this]]
        );
    }

    #[test]
    fn destructor_frees_owned_storage_and_restores_the_base_vtable() {
        let (mut e, log, allocator) = setup();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x18, 0x1234);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, 0x4000_0008);
        e.call(0x00d5_3e30, &args![this]);
        assert_eq!(
            calls_to(&log, FREE_DOUBLE),
            vec![vec![allocator, 0x1234, 0x100]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn destructor_leaves_storage_it_does_not_own() {
        let (mut e, log, _) = setup();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x18, 0x1234);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, 0x8000_0008);
        e.call(0x00d5_3e30, &args![this]);
        assert!(calls_to(&log, FREE_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    /// A chain of `bodies` bodies, each at `[0xa4] = 100 + index` with the identity rotation
    /// and the translation `(index, 2 * index, 3 * index, 1)`; the pivot pair `i` is
    /// `((i + 1, 0, 1, 0), (0, i + 1, 2, 0))`.
    struct Scene {
        e: Engine,
        log: Log,
        this: u32,
        input: u32,
        output: u32,
        bodies: Vec<u32>,
        accumulators: Rc<RefCell<Vec<Vec<u32>>>>,
        links: Rc<RefCell<Vec<Vec<u32>>>>,
        chain: Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>>,
    }

    fn scene(count: u32) -> Scene {
        let (mut e, log, _) = setup();
        let accumulators = Rc::new(RefCell::new(Vec::new()));
        let links = Rc::new(RefCell::new(Vec::new()));
        let chain = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(PREPARE_PASS, move |_, a| {
            sink.borrow_mut().push((PREPARE_PASS, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(JACOBIAN_CURSOR, move |_, a| {
            sink.borrow_mut().push((JACOBIAN_CURSOR, a.to_vec()));
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&accumulators);
        e.register_double(LOAD_ACCUMULATORS, move |e, a| {
            let mut words: Vec<u32> = (0..0x20).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            words.push(a[0]);
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let sink = Rc::clone(&links);
        e.register_double(LINK_JACOBIAN, move |e, a| {
            let mut words = a.to_vec();
            words.extend(bits(load_vec(e, a[0])));
            words.extend(bits(load_vec(e, a[1])));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let sink = Rc::clone(&chain);
        e.register_double(CHAIN_JACOBIAN, move |e, a| {
            let entries: Vec<u32> = (0..=a[0]).map(|i| e.mem.u32(a[4] + 4 * i)).collect();
            sink.borrow_mut().push((a.to_vec(), entries));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let pairs = e.mem.alloc(0x20 * count);
        for i in 0..count {
            let pair = pairs + 0x20 * i;
            store_vec(&mut e, pair, [i as f32 + 1.0, 0.0, 1.0, 0.0]);
            store_vec(&mut e, pair + 0x10, [0.0, i as f32 + 1.0, 2.0, 0.0]);
        }
        e.mem.set_u32(this + 0x18, pairs);
        e.mem.set_f32(this + 0x24, 0.5);
        e.mem.set_f32(this + 0x28, 0.25);
        e.mem.set_f32(this + 0x2c, 0.125);
        e.mem.set_u32(this + 0x30, 0x7f80_0001);
        let bodies: Vec<u32> = (0..count)
            .map(|i| {
                let body = e.mem.alloc(0x130);
                e.mem.set_u32(body + 0xa4, 100 + i);
                for row in 0..3 {
                    let mut unit = [0.0f32; 4];
                    unit[row] = 1.0;
                    store_vec(&mut e, body + 0xf0 + 0x10 * row as u32, unit);
                }
                store_vec(
                    &mut e,
                    body + 0x120,
                    [i as f32, 2.0 * i as f32, 3.0 * i as f32, 1.0],
                );
                body
            })
            .collect();
        let list = e.mem.alloc(4 * count);
        for (i, body) in bodies.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *body);
        }
        let instance_body = e.mem.alloc(0xb0);
        e.mem.set_u32(instance_body + 0xa4, 40);
        let instance = e.mem.alloc(0x50);
        e.mem.set_u32(instance + 0x14, instance_body);
        e.mem.set_u32(instance + 0x38, list);
        e.mem.set_u32(instance + 0x3c, count);
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
            links,
            chain,
        }
    }

    #[test]
    fn build_jacobian_walks_the_links_of_a_three_body_chain() {
        let mut s = scene(3);
        s.e.call(0x00d5_3e80, &args![s.this, s.input, s.output]);
        // Prepare pass, then the start of the output cursor is the output's first word.
        assert_eq!(
            calls_to(&s.log, PREPARE_PASS),
            vec![vec![s.input, s.output, 7, 8]]
        );
        let accumulators = s.accumulators.borrow();
        assert_eq!(accumulators.len(), 2);
        let first = &accumulators[0];
        // The copy of the input with its words +0x60 and +0x64 cleared; +0x20 is input word 2,
        // +0x24 is 1.0; +0x30/+0x34 are the previous and current body offsets
        // (relative offset + 960); +0x38/+0x3C the previous and current matrices.
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
        // Pivot 0 of link 0 is pair 0's first vector moved by body 0 (translation 0), pivot 1
        // its second vector moved by body 1 (translation (1, 2, 3)).
        assert_eq!(links[0][2], 0x7fc0_0001);
        assert_eq!(links[0][3], accumulators[0][0x20]);
        assert_eq!(links[0][4], 0x9000);
        assert_eq!(links[0][5..9], bits([1.0, 0.0, 1.0, 1.0]));
        assert_eq!(links[0][9..13], bits([1.0, 3.0, 5.0, 1.0]));
        // Link 1: pair 1 moved by body 1 and body 2 (translation (2, 4, 6)).
        assert_eq!(links[1][4], 0x9090);
        assert_eq!(links[1][5..9], bits([3.0, 2.0, 4.0, 1.0]));
        assert_eq!(links[1][9..13], bits([2.0, 6.0, 8.0, 1.0]));
        let chain = s.chain.borrow();
        assert_eq!(chain.len(), 1);
        let (words, entries) = &chain[0];
        assert_eq!(words[0], 2);
        assert_eq!(words[1..4], bits([0.5, 0.25, 0.125, 0.0])[..3]);
        assert_eq!(words[5], 960);
        assert_eq!(words[6], 0x9000 + 2 * 0x90);
        assert_eq!(words[7..9], [s.input, s.output]);
        assert_eq!(entries[..3], [100, 101, 102]);
        // The inline index array is not freed.
        assert!(calls_to(&s.log, FREE_DOUBLE).is_empty());
    }

    #[test]
    fn build_jacobian_of_a_single_body_skips_the_links() {
        let mut s = scene(1);
        s.e.call(0x00d5_3e80, &args![s.this, s.input, s.output]);
        assert!(s.accumulators.borrow().is_empty());
        assert!(s.links.borrow().is_empty());
        let chain = s.chain.borrow();
        assert_eq!(chain[0].0[0], 0);
        assert_eq!(chain[0].0[6], 0x9000);
        assert_eq!(chain[0].1[0], 100);
    }

    #[test]
    fn build_jacobian_grows_the_index_array_past_32_entries_and_frees_it() {
        let mut s = scene(35);
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
        s.e.call(0x00d5_3e80, &args![s.this, s.input, s.output]);
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
