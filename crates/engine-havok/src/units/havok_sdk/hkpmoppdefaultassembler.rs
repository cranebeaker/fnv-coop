//! `Havok SDK/hkpmoppdefaultassembler.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The assembler (`hkpMoppDefaultAssembler`, vtable `010ea5dc`), its node infos
//! (`hkpMoppAssemblerNodeInfo`, 0x48 bytes on the stack) and the code generator
//! (`hkpMoppCodeGenerator`, vtable `010ea5b4`) are not in the Xbox PDB, so their fields are
//! raw offsets here and the doc comments say what each function does to them. Generator:
//! capacity +8, used bytes +0xc, buffer +0x10 (bytes are written from the top of the buffer
//! downwards), array of jump records +0x14. Tree node: leaf flag +4, extents +0xc, command
//! count +0x2c and terminal words +0x30, written flag +0x38, chunk number +0x40, split vector
//! at the pointer in +0x60, children +0x70 and +0x74.

use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// Four SSE lanes as bits.
type Lanes = [u32; 4];

/// The vtable of `hkpMoppDefaultAssembler`.
const ASSEMBLER_VTABLE: u32 = 0x010e_a5dc;
/// The base class vtable the destructor ends with.
const BASE_ASSEMBLER_VTABLE: u32 = 0x0102_e388;
/// The generator's `resize` (`00e06490`, `this` only), run when its buffer is full.
const GENERATOR_RESIZE: u32 = 0x00e0_6490;
/// The generator constructor (`00e065f0`, `this` and the capacity).
const GENERATOR_CTOR: u32 = 0x00e0_65f0;
/// The generator destructor (`00e06650`, `this`).
const GENERATOR_DTOR: u32 = 0x00e0_6650;
/// `hkpMoppCodeGenerator::validateJumpCommands` (`00e06480`).
const VALIDATE_JUMP_COMMANDS: u32 = 0x00e0_6480;
/// The memory router getter (`00c85750`); its word +0x10 is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The array grow helper (`00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Insert into an `hkArray` of words (`00cd1e20`, `(array, allocator, index, source, count)`).
const ARRAY_INSERT_AT: u32 = 0x00cd_1e20;
/// `hkMath::hkFloatToInt` (`00cbfef0`, cdecl, one float).
const FLOAT_TO_INT: u32 = 0x00cb_fef0;
/// `hkMath::hkFloorToInt` (`00cbfe20`, cdecl, one float).
const FLOOR_TO_INT: u32 = 0x00cb_fe20;
/// `hkMath::hkFloor` (`00cbfd80`, cdecl, one float, result in `ST0`).
const HK_FLOOR: u32 = 0x00cb_fd80;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00cb_1260;
/// The reference counter increment (`00c90510`, `this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// The double 0.9 widened from a float.
const SCALE_FACTOR: u32 = 0x0106_b9e8;
/// The double 0.1 widened from a float (the split code threshold).
const SPLIT_THRESHOLD: u32 = 0x0101_ffa0;
/// The negated threshold.
const SPLIT_THRESHOLD_NEGATIVE: u32 = 0x0106_b170;
/// The float 0.0 (the fourth lane of vectors the code builds).
const SPLIT_LANE_FILL: u32 = 0x0101_1d78;
/// The double 0.0 `getScaleInfo` compares the first width with.
const SCALE_INFO_FLOOR: u32 = 0x0101_2060;
/// The double 255.0.
const SCALE_INFO_NUMERATOR: u32 = 0x010e_a5c8;
/// The double 65536.0.
const SCALE_INFO_FACTOR: u32 = 0x0108_e058;
/// The double 0.5.
const FACTOR_HALF: u32 = 0x0101_1588;
/// The double 1.4142 (as `0x3ff6a09e60000000`).
const FACTOR_PAIR_TWO: u32 = 0x0101_ff98;
/// The double `0x3fd5555560000000` (a third).
const FACTOR_THREE_OUTER: u32 = 0x0109_9ee0;
/// The double `0x3ffbb67ae0000000`.
const FACTOR_THREE_INNER: u32 = 0x010e_a5c0;
/// The float 0.2.
const PLANE_FIFTH: u32 = 0x0101_df44;
/// The float 0.5.
const PLANE_HALF: u32 = 0x0101_6248;
/// The float 0.05.
const PLANE_TWENTIETH: u32 = 0x0101_81bc;
/// The float 1.0.
const PLANE_ONE: u32 = 0x0101_2068;
/// The float -1.0.
const PLANE_NEGATIVE_ONE: u32 = 0x0101_2054;
/// The float 0.25.
const PLANE_QUARTER: u32 = 0x0101_622c;
/// The float 0.3.
const PLANE_THREE_TENTHS: u32 = 0x0101_7870;
/// The float 0.32.
const PLANE_VALUE_32: u32 = 0x010e_a5d4;
/// The float 0.34.
const PLANE_VALUE_34: u32 = 0x010e_a5d0;
/// The float 3.0.
const PLANE_THREE: u32 = 0x0101_7718;

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

fn lanes_op(dest: Lanes, source: Lanes, op: fn(f32, f32) -> f32) -> Lanes {
    [0, 1, 2, 3].map(|lane| sse(dest[lane], source[lane], op))
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

/// The lane sum `x2 + (x1 + x0)` the code builds with two `ADDSS` over shuffled copies.
fn lane_sum(lanes: Lanes) -> u32 {
    let low = sse(lanes[1], lanes[0], add);
    sse(lanes[2], low, add)
}

/// A `MOVAPS` load of four lanes.
fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    [0, 1, 2, 3].map(|lane| e.mem.u32(addr + 4 * lane))
}

/// A `MOVAPS` store of four lanes.
fn store_lanes(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, value);
    }
}

// Translated from 00e066f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAssemblerNodeInfo::updateOffsetAndSpread` (Xbox PDB): `this` +0x38 gets the
/// difference of `other`'s words +0x28 and +0x24, +0x3c its count at +0x2c, and for every
/// index below that count the word `other[+0x30 + 4 * i]` goes to `this[+0x40 + 4 * i]` and
/// the difference to the next word to `this[+0x44 + 4 * i]`.
pub fn hkp_mopp_assembler_node_info_update_offset_and_spread(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) {
    let (this, other) = (this.addr(), other.addr());
    let width = e
        .mem
        .u32(other + 0x28)
        .wrapping_sub(e.mem.u32(other + 0x24));
    e.mem.set_u32(this + 0x38, width);
    let count = e.mem.u32(other + 0x2c);
    e.mem.set_u32(this + 0x3c, count);
    if 0 < e.mem.i32(other + 0x2c) {
        let mut index = 0i32;
        let mut written = this + 0x44;
        let mut read = other + 0x30;
        loop {
            let first = e.mem.u32(read);
            e.mem.set_u32(written - 4, first);
            let spread = e.mem.u32(read + 4).wrapping_sub(e.mem.u32(read));
            e.mem.set_u32(written, spread);
            index += 1;
            read += 4;
            written += 4;
            if index >= e.mem.i32(other + 0x2c) {
                break;
            }
        }
    }
}

/// `(value >> shift) << shift` with the shift count taken modulo 32, as `SAR`/`SHL` by `CL`.
fn snap_down(value: i32, shift: u32) -> i32 {
    value.wrapping_shr(shift).wrapping_shl(shift)
}

/// `((value >> shift) + 1) << shift` with the shift count taken modulo 32.
fn snap_up(value: i32, shift: u32) -> i32 {
    value
        .wrapping_shr(shift)
        .wrapping_add(1)
        .wrapping_shl(shift)
}

// Translated from 00e06730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppAssemblerNodeInfo`, no map name; the register is `this`): with the shift
/// count at +0x24 (low five bits, read again before each pair of words), rounds the six
/// extent words +0xc..+0x20 to multiples of `1 << shift`: the lower ones down (to +0x48,
/// +0x4c, +0x50) and the upper ones up to the next multiple above (+0x54, +0x58, +0x5c).
pub fn fn_00e06730(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let shift = e.mem.u32(this + 0x24);
    let value = snap_up(e.mem.i32(this + 0x10), shift);
    e.mem.set_i32(this + 0x54, value);
    let value = snap_down(e.mem.i32(this + 0xc), shift);
    e.mem.set_i32(this + 0x48, value);
    let shift = e.mem.u32(this + 0x24);
    let value = snap_down(e.mem.i32(this + 0x14), shift);
    e.mem.set_i32(this + 0x4c, value);
    let value = snap_up(e.mem.i32(this + 0x18), shift);
    e.mem.set_i32(this + 0x58, value);
    let shift = e.mem.u32(this + 0x24);
    let value = snap_down(e.mem.i32(this + 0x1c), shift);
    e.mem.set_i32(this + 0x50, value);
    let value = snap_up(e.mem.i32(this + 0x20), shift);
    e.mem.set_i32(this + 0x5c, value);
}

/// One byte pushed into a `hkpMoppCodeGenerator` buffer, as every command writer of the exe
/// has it inline: the byte goes to `buffer[capacity - used - 1]` (`+0x10`, `+8`, `+0xc`), the
/// count of used bytes grows by one and, when it has reached the capacity (signed), the
/// generator's `resize` (`00e06490`) runs.
fn generator_push(e: &mut Engine, generator: u32, byte: u8) {
    let at = e
        .mem
        .u32(generator + 0x10)
        .wrapping_sub(e.mem.u32(generator + 0xc))
        .wrapping_add(e.mem.u32(generator + 8))
        .wrapping_sub(1);
    e.mem.set_u8(at, byte);
    let used = e.mem.u32(generator + 0xc).wrapping_add(1);
    e.mem.set_u32(generator + 0xc, used);
    if used as i32 >= e.mem.i32(generator + 8) {
        e.call(GENERATOR_RESIZE, &args![generator]);
    }
}

// Translated from 00e06780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppCodeGenerator`, no map name): appends one byte.
pub fn fn_00e06780(e: &mut Engine, this: Ptr, byte: u8) {
    generator_push(e, this.addr(), byte);
}

// Translated from 00e067b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::addCommand8` (Xbox PDB): appends `value`, then `command`.
pub fn hkp_mopp_code_generator_add_command8(e: &mut Engine, this: Ptr, command: u8, value: u8) {
    generator_push(e, this.addr(), value);
    generator_push(e, this.addr(), command);
}

// Translated from 00e06800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::addCommand16` (Xbox PDB): appends the low two bytes of `value`
/// (low byte first), then `command`.
pub fn hkp_mopp_code_generator_add_command16(e: &mut Engine, this: Ptr, command: u8, value: i32) {
    let generator = this.addr();
    generator_push(e, generator, value as u8);
    generator_push(e, generator, (value >> 8) as u8);
    generator_push(e, generator, command);
}

// Translated from 00e06880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::addCommand24` (Xbox PDB): appends the low three bytes of `value`
/// (low byte first), then `command`.
pub fn hkp_mopp_code_generator_add_command24(e: &mut Engine, this: Ptr, command: u8, value: i32) {
    let generator = this.addr();
    generator_push(e, generator, value as u8);
    generator_push(e, generator, (value >> 8) as u8);
    generator_push(e, generator, (value >> 16) as u8);
    generator_push(e, generator, command);
}

// Translated from 00e06920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::addParam24` (Xbox PDB): appends the low three bytes of `value`,
/// low byte first.
pub fn hkp_mopp_code_generator_add_param24(e: &mut Engine, this: Ptr, value: i32) {
    let generator = this.addr();
    generator_push(e, generator, value as u8);
    generator_push(e, generator, (value >> 8) as u8);
    generator_push(e, generator, (value >> 16) as u8);
}

// Translated from 00e069a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppCodeGenerator`, no map name): appends the four bytes of `value`, low byte
/// first, then `command`.
pub fn fn_00e069a0(e: &mut Engine, this: Ptr, command: u8, value: u32) {
    let generator = this.addr();
    generator_push(e, generator, value as u8);
    generator_push(e, generator, (value >> 8) as u8);
    generator_push(e, generator, (value >> 16) as u8);
    generator_push(e, generator, (value >> 24) as u8);
    generator_push(e, generator, command);
}

// Translated from 00e06a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::pad` (Xbox PDB): appends `count` bytes of `0xCD`; nothing when
/// `count` is not positive.
pub fn hkp_mopp_code_generator_pad(e: &mut Engine, this: Ptr, count: i32) {
    for _ in 0..count.max(0) {
        generator_push(e, this.addr(), 0xcd);
    }
}

// Translated from 00e06aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::releaseNode` (Xbox PDB): when the byte at `node` +4 is zero and
/// the byte at `this` +0x74 is not, calls virtual slot +0xc of the object at `this` +0xc
/// (an allocator-like object; the slot is unconfirmed) with each of the node's words +0x70
/// and +0x74 and clears them.
pub fn hkp_mopp_default_assembler_release_node(e: &mut Engine, this: Ptr, node: Ptr) {
    let (this, node) = (this.addr(), node.addr());
    if e.mem.u8(node + 4) == 0 && e.mem.u8(this + 0x74) != 0 {
        let owner = e.mem.u32(this + 0xc);
        let first = e.mem.u32(node + 0x70);
        e.vcall(owner, 0xc, &args![first]);
        let owner = e.mem.u32(this + 0xc);
        let second = e.mem.u32(node + 0x74);
        e.vcall(owner, 0xc, &args![second]);
        e.mem.set_u32(node + 0x70, 0);
        e.mem.set_u32(node + 0x74, 0);
    }
}

// Translated from 00e06af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name): writes the byte `1` to `out` when the
/// word at `node` +8 is below 0x16 (signed) or the word `*limit` is above the word at `this`
/// +0x18 (signed), else `0`. Returns `out` (the exe leaves it in `EAX`).
pub fn fn_00e06af0(e: &mut Engine, this: Ptr, out: Ptr, node: Ptr, limit: Ptr) -> Ptr {
    if e.mem.i32(node.addr() + 8) < 0x16 {
        e.mem.set_u8(out.addr(), 1);
        return out;
    }
    let value = e.mem.i32(limit.addr());
    let flag = u8::from(value > e.mem.i32(this.addr() + 0x18));
    e.mem.set_u8(out.addr(), flag);
    out
}

// Translated from 00e06b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no map name): walks the chain of nodes from `node` through the first word of each
/// and sets the byte at +0x39 of each to 1, stopping at the end of the chain or at the first
/// node whose byte was already set.
pub fn fn_00e06b20(e: &mut Engine, node: Ptr) {
    let mut node = node.addr();
    while node != 0 {
        if e.mem.u8(node + 0x39) != 0 {
            break;
        }
        e.mem.set_u8(node + 0x39, 1);
        node = e.mem.u32(node);
    }
}

// Translated from 00e06b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name; the `this` register is not read): copies
/// the word at `second` +0x34 to `third` +0x34; then, with `n` the word `third` +0x38 and
/// `size = n - second[+0x34] + first[+0x24]`, puts `first[+0x24]` there instead when `size`
/// is at least 0x20 and some size class of the two (`n`, `second[+0x38]`) crosses a boundary
/// of 0x20, 0x100 or 0x10000 (unsigned compares).
pub fn fn_00e06b50(e: &mut Engine, _this: Ptr, first: Ptr, second: Ptr, third: Ptr) {
    let (first, second, third) = (first.addr(), second.addr(), third.addr());
    let carried = e.mem.u32(second + 0x34);
    e.mem.set_u32(third + 0x34, carried);
    let base = e.mem.u32(first + 0x24);
    let own = e.mem.u32(third + 0x38);
    let size = own
        .wrapping_sub(e.mem.u32(second + 0x34))
        .wrapping_add(base);
    if size < 0x20 {
        return;
    }
    if own <= 2 && size < 0x100 {
        return;
    }
    let other = e.mem.u32(second + 0x38);
    let reset = (own < 0x20 && other >= 0x20)
        || (own < 0x100 && other >= 0x100)
        || (own < 0x10000 && other >= 0x10000);
    if reset {
        e.mem.set_u32(third + 0x34, base);
    }
}

// Translated from 00e06bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name): with the generator at `this` +0x10,
/// appends `command_base` plus 0x60 with the one-byte value when `value` is in 0..0x100
/// (`addCommand8`), plus 0x64 with two bytes when below 0x10000 (`addCommand16`), plus 0x68
/// with four bytes otherwise and for a negative `value`.
pub fn fn_00e06bc0(e: &mut Engine, this: Ptr, command_base: u32, value: i32) {
    let generator = Ptr::new(e.mem.u32(this.addr() + 0x10));
    if value < 0 {
        fn_00e069a0(
            e,
            generator,
            command_base.wrapping_add(0x68) as u8,
            value as u32,
        );
    } else if value < 0x100 {
        hkp_mopp_code_generator_add_command8(
            e,
            generator,
            command_base.wrapping_add(0x60) as u8,
            value as u8,
        );
    } else if value < 0x10000 {
        hkp_mopp_code_generator_add_command16(
            e,
            generator,
            command_base.wrapping_add(0x64) as u8,
            value,
        );
    } else {
        fn_00e069a0(
            e,
            generator,
            command_base.wrapping_add(0x68) as u8,
            value as u32,
        );
    }
}

// Translated from 00e06c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name): appends the four bytes of `value` and
/// the command `0x70` to the generator at `this` +0x10. Returns 5.
pub fn fn_00e06c20(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let generator = Ptr::new(e.mem.u32(this.addr() + 0x10));
    fn_00e069a0(e, generator, 0x70, value);
    5
}

// Translated from 00e06c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::addDataOffset` (Xbox PDB): appends the low two bytes of `second`
/// (low byte first) to the generator at `this` +0x10, then the command `0x0d` with the
/// two-byte value `first` (`addCommand16`). Returns 5.
pub fn hkp_mopp_default_assembler_add_data_offset(
    e: &mut Engine,
    this: Ptr,
    first: i32,
    second: i32,
) -> u32 {
    let generator = e.mem.u32(this.addr() + 0x10);
    generator_push(e, generator, second as u8);
    let generator = e.mem.u32(this.addr() + 0x10);
    generator_push(e, generator, (second >> 8) as u8);
    let generator = Ptr::new(e.mem.u32(this.addr() + 0x10));
    hkp_mopp_code_generator_add_command16(e, generator, 0x0d, first);
    5
}

// Translated from 00e06cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::addRescale` (Xbox PDB): appends the bytes at `info` +0x10, +0xc,
/// +8 and +4 (in that order) to the generator at `this` +0x10.
pub fn hkp_mopp_default_assembler_add_rescale(e: &mut Engine, this: Ptr, info: Ptr) {
    for offset in [0x10, 0xc, 8, 4] {
        let generator = e.mem.u32(this.addr() + 0x10);
        let byte = e.mem.u8(info.addr() + offset);
        generator_push(e, generator, byte);
    }
}

// Translated from 00e06d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::checkAndAddProperties` (Xbox PDB): appends `value` to the
/// generator at `this` +0x10 with the command 9 and one byte below 0x100 (unsigned),
/// command 0xa and two bytes below 0x10000, else command 0xb and four bytes.
pub fn hkp_mopp_default_assembler_check_and_add_properties(e: &mut Engine, this: Ptr, value: u32) {
    let generator = Ptr::new(e.mem.u32(this.addr() + 0x10));
    if value < 0x100 {
        hkp_mopp_code_generator_add_command8(e, generator, 9, value as u8);
    } else if value < 0x10000 {
        hkp_mopp_code_generator_add_command16(e, generator, 0xa, value as i32);
    } else {
        fn_00e069a0(e, generator, 0xb, value);
    }
}

// Translated from 00e06db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name): with the word at `first` zero, appends
/// `fn_00e06bc0(0, third[+0x40])` when `third[+0x44]` is zero and `third[+0x40]` is not;
/// with it non-zero, the same when `third[+0x44]` is zero and `second[+0x44]` is not (the
/// value is not tested there).
pub fn fn_00e06db0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr) {
    let (first, second, third) = (first.addr(), second.addr(), third.addr());
    if e.mem.u32(first) == 0 {
        if e.mem.u32(third + 0x44) != 0 {
            return;
        }
        let value = e.mem.i32(third + 0x40);
        if value == 0 {
            return;
        }
        fn_00e06bc0(e, this, 0, value);
    } else {
        if e.mem.u32(third + 0x44) != 0 {
            return;
        }
        if e.mem.u32(second + 0x44) == 0 {
            return;
        }
        let value = e.mem.i32(third + 0x40);
        fn_00e06bc0(e, this, 0, value);
    }
}

// Translated from 00e06e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::addTerminals` (Xbox PDB; the body is a command writer for the
/// terminal ids, named by the map): with `distance = **(node + 0x60) - third[+0x34]`, appends
/// to the generator at `this` +0x10 one byte `0x30 + distance` below 0x20 or the value in
/// one (command 0x50), two (0x51), three (0x52) or four (0x53) bytes by size, then for every
/// index below `node[+0x2c]` whose word `node[+0x30 + 4 * i]` and `second[+0x44 + 4 * i]` are
/// both non-zero calls `fn_00e06bc0(i, node[+0x30 + 4 * i])`. Returns the number of bytes
/// appended.
pub fn hkp_mopp_default_assembler_add_terminals(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: Ptr,
    third: Ptr,
) -> i32 {
    let (node, second, third) = (node.addr(), second.addr(), third.addr());
    let generator = e.mem.u32(this.addr() + 0x10);
    let start = e.mem.u32(generator + 0xc);
    let terminals = e.mem.u32(node + 0x60);
    let distance = e.mem.u32(terminals).wrapping_sub(e.mem.u32(third + 0x34));
    if distance < 0x20 {
        generator_push(e, generator, (distance as u8).wrapping_add(0x30));
    } else if distance < 0x100 {
        hkp_mopp_code_generator_add_command8(e, Ptr::new(generator), 0x50, distance as u8);
    } else if distance < 0x10000 {
        hkp_mopp_code_generator_add_command16(e, Ptr::new(generator), 0x51, distance as i32);
    } else if distance < 0x100_0000 {
        hkp_mopp_code_generator_add_command24(e, Ptr::new(generator), 0x52, distance as i32);
    } else {
        fn_00e069a0(e, Ptr::new(generator), 0x53, distance);
    }
    let mut index = 0i32;
    if 0 < e.mem.i32(node + 0x2c) {
        loop {
            let value = e.mem.u32(node + 0x30 + 4 * index as u32);
            if value != 0 && e.mem.u32(second + 0x44 + 4 * index as u32) != 0 {
                fn_00e06bc0(e, this, index as u32, value as i32);
            }
            index += 1;
            if index >= e.mem.i32(node + 0x2c) {
                break;
            }
        }
    }
    let generator = e.mem.u32(this.addr() + 0x10);
    e.mem.u32(generator + 0xc).wrapping_sub(start) as i32
}

// Translated from 00e06ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::addJump` (Xbox PDB): with `offset = used - target` for the
/// generator at `this` +0x10 (`used` its count at +0xc), appends nothing when `offset` is not
/// positive, else `offset` as command 5 with one byte below 0xff, 6 with two bytes below
/// 0xffff, 7 with three bytes below 0xffffff, else 8 with four (all compares signed).
pub fn hkp_mopp_default_assembler_add_jump(e: &mut Engine, this: Ptr, target: i32) {
    let generator = Ptr::new(e.mem.u32(this.addr() + 0x10));
    let offset = e.mem.i32(generator.addr() + 0xc).wrapping_sub(target);
    if offset <= 0 {
        return;
    }
    if offset < 0xff {
        hkp_mopp_code_generator_add_command8(e, generator, 5, offset as u8);
    } else if offset < 0xffff {
        hkp_mopp_code_generator_add_command16(e, generator, 6, offset);
    } else if offset < 0xff_ffff {
        hkp_mopp_code_generator_add_command24(e, generator, 7, offset);
    } else {
        fn_00e069a0(e, generator, 8, offset as u32);
    }
}

// Translated from 00e06f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::getSplitCode` (Xbox PDB; the `this` register is not read): the
/// code 0..0xc for the sign class of the three floats at `vector` against the double
/// +/-0.1f (`0101ffa0`, `0106b170`): x above the threshold or not, then y, then z, each as
/// above, within or below. The compares are the x87's (a NaN takes the "not above" and "not
/// below" side).
pub fn hkp_mopp_default_assembler_get_split_code(e: &mut Engine, _this: Ptr, vector: Ptr) -> u32 {
    let threshold = e.mem.f64(SPLIT_THRESHOLD);
    let negative = e.mem.f64(SPLIT_THRESHOLD_NEGATIVE);
    let load = |e: &Engine, offset: u32| f64::from(e.mem.f32(vector.addr() + offset));
    let (x, y, z) = (load(e, 0), load(e, 4), load(e, 8));
    if x > threshold {
        if z > threshold {
            if y > threshold {
                9
            } else if y < negative {
                0xb
            } else {
                5
            }
        } else if z < negative {
            if y > threshold {
                0xa
            } else if y < negative {
                0xc
            } else {
                6
            }
        } else if y > threshold {
            7
        } else if y < negative {
            8
        } else {
            0
        }
    } else if y > threshold {
        if z > threshold {
            3
        } else if z < negative {
            4
        } else {
            1
        }
    } else {
        2
    }
}

// Translated from 00e07040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name; static): with `limit = max(second[+8] -
/// 8, 0)`, returns `first[+0x24] - limit`, or `first[+0x24]` when the subtraction passed it
/// (signed).
pub fn fn_00e07040(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let limit = e.mem.i32(second.addr() + 8).wrapping_sub(8).max(0);
    let shift = e.mem.i32(first.addr() + 0x24);
    let result = shift.wrapping_sub(limit);
    if result > shift {
        shift
    } else {
        result
    }
}

// Translated from 00e07070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name; static): takes `n = fn_00e07040(first,
/// second)` (at most 4) and writes the byte `0` to `out`; with `n` not positive copies the
/// words +0x24, +0x28, +0x2c and +0x30 of `first` into `second`. Otherwise sets `second[+0x24]
/// = first[+0x24] - n` and for the three lanes `i` stores `q = (second[+0xc + 8 * i] -
/// first[+0x28 + 4 * i]) >> shift` (the shift of `first`) as `second[+0x28 + 4 * i] = (q <<
/// shift) + first[+0x28 + 4 * i]` and `out[+8 + 4 * i] = q`; if the span `second[+0x10 + 8 *
/// i] - second[+0x28 + 4 * i]`, shifted by the new `second[+0x24]`, reaches 0xff (signed),
/// `n` drops by one and the lanes are redone. At the end the byte `1` and `n` go to `out`
/// (+0 and +4).
pub fn fn_00e07070(e: &mut Engine, first: Ptr, second: Ptr, out: Ptr) {
    let (first, second, out) = (first.addr(), second.addr(), out.addr());
    let mut count = fn_00e07040(e, Ptr::new(first), Ptr::new(second));
    e.mem.set_u8(out, 0);
    if count <= 0 {
        for offset in [0x24, 0x28, 0x2c, 0x30] {
            let word = e.mem.u32(first + offset);
            e.mem.set_u32(second + offset, word);
        }
        return;
    }
    if count >= 4 {
        count = 4;
    }
    loop {
        let shift = e.mem.i32(first + 0x24).wrapping_sub(count);
        e.mem.set_i32(second + 0x24, shift);
        let mut lane = 0u32;
        while lane < 3 {
            let shift = e.mem.u32(first + 0x24);
            let base = e.mem.i32(first + 0x28 + 4 * lane);
            let quotient = e
                .mem
                .i32(second + 0xc + 8 * lane)
                .wrapping_sub(base)
                .wrapping_shr(shift);
            let snapped = quotient.wrapping_shl(shift).wrapping_add(base);
            e.mem.set_i32(second + 0x28 + 4 * lane, snapped);
            e.mem.set_i32(out + 8 + 4 * lane, quotient);
            let span = e
                .mem
                .i32(second + 0x10 + 8 * lane)
                .wrapping_sub(e.mem.i32(second + 0x28 + 4 * lane));
            let shift = e.mem.u32(second + 0x24);
            if span.wrapping_shr(shift) >= 0xff {
                break;
            }
            lane += 1;
        }
        if lane >= 3 {
            break;
        }
        count = count.wrapping_sub(1);
    }
    e.mem.set_i32(out + 4, count);
    e.mem.set_u8(out, 1);
}

// Translated from 00e07160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::preCalcScale` (Xbox PDB): the smallest shift count (the scale
/// exponent) the subtree at `node` needs, `info` being the node info (`hkpMoppAssemblerNodeInfo`
/// copy; its word +8 is the shift count) and `parent` that of the level above.
///
/// A leaf (byte `node` +4 non-zero) looks at the three extents: for each lane `i` with
/// `d = high - low` (floats at `node` +0xc + 8 * i) and `a` the float at `this` +0x60 + 4 * i, an
/// `a` above `d` and below the running best (starting at `this` +0x58) is scaled by `this` +0x3c,
/// converted (`00cbfef0`) and shifted left by 7: if that is below `1 << shift` the byte
/// `node[+0x3b + i]` becomes 2 (when the byte at +0x39 is zero), else `a` becomes the new best.
/// The result is `max(shift, bits(best * this[+0x3c] * 0106b9e8) + 6)`. Bytes `node[+0x3b + i]`
/// are set to 1 first when the byte at +0x39 is zero.
///
/// An inner node sets the byte at +0x3a when its shift is small (`fn_00e07040`) and
/// re-quantises (`fn_00e07070`), then recurses into the children at +0x74 and +0x70 with a
/// fresh copy of `info` built from the child (the `hkpMoppAssemblerNodeInfo` constructor,
/// `hkpMoppAssemblerNodeInfo` constructor), and combines their results with the flags at +0x39 and +0x3a.
pub fn hkp_mopp_default_assembler_pre_calc_scale(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    parent: Ptr,
    info: Ptr,
) -> i32 {
    let (this, node, parent, info) = (this.addr(), node.addr(), parent.addr(), info.addr());
    let shift = e.mem.i32(info + 8);
    if e.mem.u8(node + 4) != 0 {
        let mut best = x87_quiet(e.mem.f32(this + 0x58));
        let limit = (1u32.rotate_left(shift as u32 & 31)) as i32;
        let mut extent = this + 0x60;
        let mut low = node + 0xc;
        for lane in 0..3u32 {
            if e.mem.u8(node + 0x39) == 0 {
                e.mem.set_u8(node + 0x3b + lane, 1);
            }
            let value = x87_quiet(e.mem.f32(extent));
            let span = (f64::from(e.mem.f32(low + 4)) - f64::from(e.mem.f32(low))) as f32;
            if f64::from(value) > f64::from(span) && f64::from(best) > f64::from(value) {
                let scale = x87_quiet(e.mem.f32(this + 0x3c));
                let product = x87_mul(f64::from(value), f64::from(scale)) as f32;
                let converted = e.call(FLOAT_TO_INT, &args![product.to_bits()]).i32();
                if converted.wrapping_shl(7) >= limit {
                    best = value;
                } else if e.mem.u8(node + 0x39) == 0 {
                    e.mem.set_u8(node + 0x3b + lane, 2);
                }
            }
            extent += 4;
            low += 8;
        }
        let scale = x87_quiet(e.mem.f32(this + 0x3c));
        let factor = e.mem.f64(SCALE_FACTOR);
        let scaled = x87_mul(x87_mul(f64::from(scale), factor), f64::from(best)) as f32;
        let mut remaining = e.call(FLOAT_TO_INT, &args![scaled.to_bits()]).u32();
        let mut bits = 0i32;
        while remaining != 0 {
            remaining >>= 1;
            bits += 1;
        }
        return if bits + 6 > shift { bits + 6 } else { shift };
    }
    if e.mem.u8(node + 0x39) == 0
        && e.mem.i32(info) < e.mem.i32(this + 0x1c)
        && fn_00e07040(e, Ptr::new(parent), Ptr::new(info)) > 2
    {
        e.mem.set_u8(node + 0x3a, 1);
    }
    if e.mem.u8(node + 0x3a) == 1 {
        e.with_stack(0x14, |e, out| {
            fn_00e07070(e, Ptr::new(parent), Ptr::new(info), out);
        });
    }
    let children = [e.mem.u32(node + 0x70), e.mem.u32(node + 0x74)];
    let mut results = [-1i32, -1];
    let mut smallest = 0x7fff_ffffi32;
    for side in [1usize, 0] {
        let child = children[side];
        if child == 0 {
            continue;
        }
        e.with_stack(0x48, |e, copy| {
            hkp_mopp_assembler_node_info_construct(
                e,
                copy,
                Ptr::new(info),
                Ptr::new(child),
                Ptr::new(this + 0x30),
            );
            results[side] = hkp_mopp_default_assembler_pre_calc_scale(
                e,
                Ptr::new(this),
                Ptr::new(child),
                Ptr::new(info),
                copy,
            );
            let copied = e.mem.i32(copy.addr() + 8);
            if smallest >= copied {
                smallest = copied;
            }
        });
    }
    let mut current = smallest.wrapping_add(2);
    let ceiling = shift.wrapping_add(2);
    if ceiling < current {
        current = ceiling;
    }
    for side in 0..2 {
        let child = children[side];
        if child == 0 {
            continue;
        }
        let result = results[side];
        if e.mem.u8(node + 0x39) != 0 {
            if e.mem.u8(child + 0x39) == 0 && result < current {
                e.mem.set_u8(child + 0x3a, 1);
            }
        } else if result < shift {
            e.mem.set_u8(child + 0x3a, 1);
        }
        if e.mem.u8(child + 0x3a) == 0 && current >= result {
            current = result;
        }
    }
    current
}

// Translated from 00e07400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::calcTermCut` (Xbox PDB): for the three lanes `i` (`width` the
/// float difference of `first`'s words +0x10 + 8 * i and +0xc + 8 * i, `shift` the word at
/// `second` +0x24): `q = (1 << shift) / this[+0x3c]`, raised to `this[+0x58]` unless that is
/// below it, and to the float at `this` +0x60 + 4 * i when that is above `width` and not
/// below `q`; then `n = convert(this[+0x3c] * q)` (`00cbfef0`) gives `first[+0x44 + 4 * i] =
/// second[+0xc + 8 * i] - n` and `first[+0x50 + 4 * i] = second[+0x10 + 8 * i] + n + 1`.
pub fn hkp_mopp_default_assembler_calc_term_cut(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
) {
    let (this, first, second) = (this.addr(), first.addr(), second.addr());
    for lane in 0..3u32 {
        let shift = e.mem.u32(second + 0x24);
        let width = (f64::from(e.mem.f32(first + 0x10 + 8 * lane))
            - f64::from(e.mem.f32(first + 0xc + 8 * lane))) as f32;
        let power = f64::from(1i32.wrapping_shl(shift));
        let mut quotient = (power / f64::from(e.mem.f32(this + 0x3c))) as f32;
        let limit = x87_quiet(e.mem.f32(this + 0x58));
        if !(f64::from(limit) < f64::from(quotient)) {
            quotient = limit;
        }
        let extent = x87_quiet(e.mem.f32(this + 0x60 + 4 * lane));
        if f64::from(extent) > f64::from(width) && !(f64::from(quotient) < f64::from(extent)) {
            quotient = extent;
        }
        let scale = x87_quiet(e.mem.f32(this + 0x3c));
        let product = x87_mul(f64::from(scale), f64::from(quotient)) as f32;
        let n = e.call(FLOAT_TO_INT, &args![product.to_bits()]).u32();
        let low = e.mem.u32(second + 0xc + 8 * lane).wrapping_sub(n);
        e.mem.set_u32(first + 0x44 + 4 * lane, low);
        let high = e
            .mem
            .u32(second + 0x10 + 8 * lane)
            .wrapping_add(n)
            .wrapping_add(1);
        e.mem.set_u32(first + 0x50 + 4 * lane, high);
    }
}

// Translated from 00e07510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::initTopDown` (Xbox PDB): when the byte at `first` +0x3a is 1 runs
/// `fn_00e07070(second, third, fourth)`, then `fn_00e06b50(first, second, third)`.
pub fn hkp_mopp_default_assembler_init_top_down(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: Ptr,
) {
    if e.mem.u8(first.addr() + 0x3a) == 1 {
        fn_00e07070(e, second, third, fourth);
    }
    fn_00e06b50(e, this, first, second, third);
}

// Translated from 00e094a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAssemblerNodeInfo::updateExtents` (Xbox PDB): for the three lanes `i`, with `scale`
/// the float `origin[3]`, stores `convert((node[+0xc + 8 * i] - origin[i]) * scale)` to `this`
/// +0xc + 8 * i and `convert((node[+0x10 + 8 * i] - origin[i]) * scale) + 1` to `this` +0x10 +
/// 8 * i, with `convert` the float-to-integer helper `00cbfe20`. (The exe also tracks the
/// largest span in a local it never reads; that is left out.)
pub fn hkp_mopp_assembler_node_info_update_extents(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    origin: Ptr,
) {
    let (this, node, origin) = (this.addr(), node.addr(), origin.addr());
    for lane in 0..3u32 {
        let scale = x87_quiet(e.mem.f32(origin + 0xc));
        let difference = x87_sub(
            f64::from(e.mem.f32(node + 0xc + 8 * lane)),
            f64::from(e.mem.f32(origin + 4 * lane)),
        );
        let product = x87_mul(difference, f64::from(scale)) as f32;
        let low = e.call(FLOOR_TO_INT, &args![product.to_bits()]).u32();
        e.mem.set_u32(this + 0xc + 8 * lane, low);
        let scale = x87_quiet(e.mem.f32(origin + 0xc));
        let difference = x87_sub(
            f64::from(e.mem.f32(node + 0x10 + 8 * lane)),
            f64::from(e.mem.f32(origin + 4 * lane)),
        );
        let product = x87_mul(difference, f64::from(scale)) as f32;
        let high = e
            .call(FLOOR_TO_INT, &args![product.to_bits()])
            .u32()
            .wrapping_add(1);
        e.mem.set_u32(this + 0x10 + 8 * lane, high);
    }
}

/// The number of significant bits of `value` (the exe's shift-until-zero loop).
fn significant_bits(mut value: u32) -> i32 {
    let mut bits = 0;
    while value != 0 {
        value >>= 1;
        bits += 1;
    }
    bits
}

// Translated from 00e09540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAssemblerNodeInfo::calcMaxExtentsSignificantBits` (Xbox PDB): the largest of the
/// three extent widths (`this` +0x10 - +0xc, +0x18 - +0x14, +0x20 - +0x1c, signed), plus
/// `1 << (bits - 4)` with `bits` its significant bits, has its own significant-bit count
/// stored at `this` +8, limited to 0x18.
pub fn hkp_mopp_assembler_node_info_calc_max_extents_significant_bits(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut widest = e.mem.i32(this + 0x10).wrapping_sub(e.mem.i32(this + 0xc));
    let second = e.mem.i32(this + 0x18).wrapping_sub(e.mem.i32(this + 0x14));
    if widest <= second {
        widest = second;
    }
    let third = e.mem.i32(this + 0x20).wrapping_sub(e.mem.i32(this + 0x1c));
    if widest <= third {
        widest = third;
    }
    let bits = significant_bits(widest as u32);
    let rounded = 1u32
        .wrapping_shl(bits.wrapping_sub(4) as u32)
        .wrapping_add(widest as u32);
    e.mem.set_i32(this + 8, significant_bits(rounded).min(0x18));
}

// Translated from 00e095d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAssemblerNodeInfo::hkpMoppAssemblerNodeInfo` (Xbox PDB): copies the 0x48 bytes of
/// `source`, runs `updateOffsetAndSpread(second)` and `updateExtents(second, third)`, sets the
/// word at +0 to `source[0] + 1`, clears the byte at +4 and runs
/// `calcMaxExtentsSignificantBits`. Returns `this`.
pub fn hkp_mopp_assembler_node_info_construct(
    e: &mut Engine,
    this: Ptr,
    source: Ptr,
    second: Ptr,
    third: Ptr,
) -> Ptr {
    for word in 0..0x12u32 {
        let value = e.mem.u32(source.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 4 * word, value);
    }
    hkp_mopp_assembler_node_info_update_offset_and_spread(e, this, second);
    hkp_mopp_assembler_node_info_update_extents(e, this, second, third);
    let depth = e.mem.u32(source.addr()).wrapping_add(1);
    e.mem.set_u32(this.addr(), depth);
    e.mem.set_u8(this.addr() + 4, 0);
    hkp_mopp_assembler_node_info_calc_max_extents_significant_bits(e, this);
    this
}

// Translated from 00e09450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::~hkpMoppDefaultAssembler` (Xbox PDB), without the deleting
/// flag: sets the vtable word to the one at `010ea5dc`, clears the word at +0x44, frees the
/// array data at +0x40 through the allocator's slot +0x10 (`(data, capacity << 5)`) when the
/// flags word at +0x48 is not negative, then clears +0x40, stores `0x80000000` to +0x48 and
/// the base vtable `0102e388` to +0.
pub fn hkp_mopp_default_assembler_destruct(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, ASSEMBLER_VTABLE);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(this + 0x48);
    e.mem.set_u32(this + 0x44, 0);
    if flags as i32 >= 0 {
        let data = e.mem.u32(this + 0x40);
        e.vcall(allocator, 0x10, &args![data, (flags & 0x3fff_ffff) << 5]);
    }
    e.mem.set_u32(this + 0x40, 0);
    e.mem.set_u32(this + 0x48, 0x8000_0000);
    e.mem.set_u32(this, BASE_ASSEMBLER_VTABLE);
}

// Translated from 00e09620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler` vtable slot +0xc at `010ea5dc`): returns the constant
/// 0x0d.
pub fn fn_00e09620(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0d
}

// Translated from 00e09630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler` vtable slot +0x10 at `010ea5dc`): returns the word at
/// `this` +0x40.
pub fn fn_00e09630(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x40)
}

// Translated from 00e07550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::addCut` (Xbox PDB): for each lane `i` of the cut flags at `node`
/// +0x3b + `i`: a flag of 1 appends to the generator at `this` +0x10 the byte
/// `((info[+0x10 + 8 * i] - base) >> shift) + 1`, the byte `(info[+0xc + 8 * i] - base) >>
/// shift` (`base = info[+0x28 + 4 * i]`, `shift = info[+0x24]`) and the command byte `0x26 + i`;
/// a flag of 2 appends `addParam24(info[+0x10 + 8 * i] + 1)`, `addParam24(info[+0xc + 8 * i])`
/// and the command byte `0x29 + i`.
pub fn hkp_mopp_default_assembler_add_cut(e: &mut Engine, this: Ptr, node: Ptr, info: Ptr) {
    let (this, node, info) = (this.addr(), node.addr(), info.addr());
    for lane in 0..3u32 {
        if e.mem.u8(node + 0x3b + lane) == 1 {
            let base = e.mem.i32(info + 0x28 + 4 * lane);
            let shift = e.mem.u32(info + 0x24);
            let generator = e.mem.u32(this + 0x10);
            let high = e
                .mem
                .i32(info + 0x10 + 8 * lane)
                .wrapping_sub(base)
                .wrapping_shr(shift);
            let low = e
                .mem
                .i32(info + 0xc + 8 * lane)
                .wrapping_sub(base)
                .wrapping_shr(shift);
            generator_push(e, generator, (high as u8).wrapping_add(1));
            let generator = e.mem.u32(this + 0x10);
            generator_push(e, generator, low as u8);
            let generator = e.mem.u32(this + 0x10);
            generator_push(e, generator, (lane as u8).wrapping_add(0x26));
        }
        if e.mem.u8(node + 0x3b + lane) == 2 {
            let high = e.mem.i32(info + 0x10 + 8 * lane).wrapping_add(1);
            let generator = Ptr::new(e.mem.u32(this + 0x10));
            hkp_mopp_code_generator_add_param24(e, generator, high);
            let low = e.mem.i32(info + 0xc + 8 * lane);
            let generator = Ptr::new(e.mem.u32(this + 0x10));
            hkp_mopp_code_generator_add_param24(e, generator, low);
            let generator = e.mem.u32(this + 0x10);
            generator_push(e, generator, (lane as u8).wrapping_add(0x29));
        }
    }
}

// Translated from 00e07680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name; five stack words, all signed): writes a
/// split command to the generator at `this` +0x10. With `used` the generator's byte count,
/// `d = used - fifth` and, when `third < 3`, `e2 = used - fourth`: if (`e2 > 0` or `d > 0xf8`)
/// and `e2` and `d` are both below 0x10000, appends the bytes `d`, `d >> 8`, `e2`, `e2 >> 8`,
/// `first`, `second` and `third + 0x23` and returns. Otherwise it appends a jump to `fifth`
/// first when `d > 0xfb`, a jump to `fourth`, the low byte of the bytes used since, `first`,
/// and either (`third < 3` and `second - first == 1`) the byte `third + 0x20`, or `second` and
/// `third + 0x10`.
pub fn fn_00e07680(
    e: &mut Engine,
    this: Ptr,
    first: i32,
    second: i32,
    third: i32,
    fourth: i32,
    fifth: i32,
) {
    let this_addr = this.addr();
    let generator = e.mem.u32(this_addr + 0x10);
    let used = e.mem.i32(generator + 0xc);
    let distance = used.wrapping_sub(fifth);
    let mut mark = fifth;
    let mut long_form = true;
    if third < 3 {
        let near = used.wrapping_sub(fourth);
        if (near > 0 || distance > 0xf8) && near < 0x10000 && distance < 0x10000 {
            for byte in [
                distance as u8,
                (distance >> 8) as u8,
                near as u8,
                (near >> 8) as u8,
                first as u8,
                second as u8,
                (third as u8).wrapping_add(0x23),
            ] {
                let generator = e.mem.u32(this_addr + 0x10);
                generator_push(e, generator, byte);
            }
            return;
        }
        long_form = second.wrapping_sub(first) != 1;
    }
    if distance > 0xfb {
        hkp_mopp_default_assembler_add_jump(e, this, fifth);
        let generator = e.mem.u32(this_addr + 0x10);
        mark = e.mem.i32(generator + 0xc);
    }
    hkp_mopp_default_assembler_add_jump(e, this, fourth);
    let generator = e.mem.u32(this_addr + 0x10);
    let spent = e.mem.u8(generator + 0xc).wrapping_sub(mark as u8);
    generator_push(e, generator, spent);
    let generator = e.mem.u32(this_addr + 0x10);
    generator_push(e, generator, first as u8);
    if long_form {
        let generator = e.mem.u32(this_addr + 0x10);
        generator_push(e, generator, second as u8);
        let generator = e.mem.u32(this_addr + 0x10);
        generator_push(e, generator, (third as u8).wrapping_add(0x10));
    } else {
        let generator = e.mem.u32(this_addr + 0x10);
        generator_push(e, generator, (third as u8).wrapping_add(0x20));
    }
}

/// `hkMath::hkFloor` (`00cbfd80`, cdecl, one float; the result is in `ST0`).
fn floor(e: &mut Engine, value: f32) -> f32 {
    e.call(HK_FLOOR, &args![value.to_bits()]).f32()
}

// Translated from 00e07880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::findPlanes` (Xbox PDB): from the three floats at `node` +0x60's
/// target (`extents`) and the two bounds at `node` +0x64 and +0x68 computes two cut indices
/// and writes them to `second_out` and `first_out`, both limited to 0..=0xff (a never-taken
/// "store 0 when negative" check of the exe after the clamp is left out).
///
/// Counting the non-zero extents (a NaN counts, `-0.0` does not): with exactly one, on the
/// lane `k` of the last non-zero one, `first_out = clamp((hkFloatToInt(hkFloor((bound1 -
/// this[+0x30 + 4 * k]) * scale)) - info[+0x28 + 4 * k]) >> info[+0x24])` and `second_out =
/// clamp(((same with bound2) - info[+0x28 + 4 * k]) >> info[+0x24]) + 1)`. Otherwise `T` is the
/// SSE lane sum `z + (y + x)` of `(base * (1 / scale) + origin) * extents` (base the info's
/// words +0x28, +0x2c, +0x30 as floats; origin the words `this` +0x30..+0x3c) and the bounds are
/// scaled `(bound - T) * scale / 2^info[+0x24]`, multiplied by a factor pair that depends on
/// the count (2: `0101ff98`, `01011588`; 3: `010ea5c0`, `01099ee0`; else 1), offset by
/// 255 per negative extent and floored (`hkFloorToInt`).
pub fn hkp_mopp_default_assembler_find_planes(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    info: Ptr,
    second_out: Ptr,
    first_out: Ptr,
) {
    let (this, node, info) = (this.addr(), node.addr(), info.addr());
    let extents = e.mem.u32(node + 0x60);
    let mut nonzero = 0u32;
    let mut negative = 0i32;
    let mut last = 0u32;
    for lane in 0..3u32 {
        let value = f64::from(x87_quiet(e.mem.f32(extents + 4 * lane)));
        if value == 0.0 {
            continue;
        }
        nonzero += 1;
        last = lane;
        if value < 0.0 {
            negative += 1;
        }
    }
    let first_bound = f64::from(x87_quiet(e.mem.f32(node + 0x64)));
    let second_bound = f64::from(x87_quiet(e.mem.f32(node + 0x68)));
    if nonzero == 1 {
        let scale = f64::from(x87_quiet(e.mem.f32(this + 0x3c)));
        let origin = f64::from(e.mem.f32(this + 0x30 + 4 * last));
        let product = x87_mul(x87_sub(first_bound, origin), scale) as f32;
        let floored = floor(e, product);
        let first = e.call(FLOAT_TO_INT, &args![floored.to_bits()]).i32();
        let scale = f64::from(x87_quiet(e.mem.f32(this + 0x3c)));
        let product = x87_mul(x87_sub(second_bound, origin), scale) as f32;
        let floored = floor(e, product);
        let second = e.call(FLOAT_TO_INT, &args![floored.to_bits()]).i32();
        let base = e.mem.i32(info + 0x28 + 4 * last);
        let shift = e.mem.u32(info + 0x24);
        let high = second
            .wrapping_sub(base)
            .wrapping_shr(shift)
            .wrapping_add(1);
        let low = first.wrapping_sub(base).wrapping_shr(shift);
        e.mem.set_i32(first_out.addr(), low.clamp(0, 0xff));
        e.mem.set_i32(second_out.addr(), high.clamp(0, 0xff));
        return;
    }
    let scale = f64::from(x87_quiet(e.mem.f32(this + 0x3c)));
    let inverse = x87_div(1.0, scale) as f32;
    let fill = e.mem.u32(SPLIT_LANE_FILL);
    let origin = load_lanes(e, this + 0x30);
    let base = [
        (e.mem.i32(info + 0x28) as f32).to_bits(),
        (e.mem.i32(info + 0x2c) as f32).to_bits(),
        (e.mem.i32(info + 0x30) as f32).to_bits(),
        fill,
    ];
    let shift = e.mem.u32(info + 0x24);
    let extent_lanes = load_lanes(e, extents);
    let scaled = lanes_op(base, [inverse.to_bits(); 4], mul);
    let moved = lanes_op(scaled, origin, add);
    let products = lanes_op(moved, extent_lanes, mul);
    let sum = f64::from(x87_quiet(f32::from_bits(lane_sum(products))));
    let power = f64::from(1i32.wrapping_shl(shift) as f32);
    let first_scaled = x87_div(x87_mul(x87_sub(first_bound, sum), scale), power);
    let second_scaled = x87_div(x87_mul(x87_sub(second_bound, sum), scale), power);
    let (outer, inner) = match nonzero {
        2 => (e.mem.f64(FACTOR_HALF), e.mem.f64(FACTOR_PAIR_TWO)),
        3 => (e.mem.f64(FACTOR_THREE_OUTER), e.mem.f64(FACTOR_THREE_INNER)),
        _ => (1.0, 1.0),
    };
    let mut first_value = x87_mul(first_scaled, inner);
    let mut second_value = x87_mul(second_scaled, inner);
    if negative != 0 {
        let offset = f64::from(negative.wrapping_mul(0xff));
        first_value = x87_add(first_value, offset);
        second_value = x87_add(second_value, offset);
    }
    let low_argument = floor_to_int_argument(x87_mul(first_value, outer));
    let low = e.call(FLOOR_TO_INT, &args![low_argument]).i32();
    let high_argument = floor_to_int_argument(x87_mul(outer, second_value));
    let high = e
        .call(FLOOR_TO_INT, &args![high_argument])
        .i32()
        .wrapping_add(1);
    e.mem.set_i32(first_out.addr(), low.clamp(0, 0xff));
    e.mem.set_i32(second_out.addr(), high.clamp(0, 0xff));
}

/// The bits of the float an x87 value is stored as before `hkFloorToInt` takes it.
fn floor_to_int_argument(value: f64) -> u32 {
    (value as f32).to_bits()
}

// Translated from 00e07b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::getScaleInfo` (Xbox PDB; virtual slot +0x18 of the
/// `hkpMoppDefaultAssembler` vtable `010ea5dc`; the `this` register is not read): writes the
/// vector `(node[+0xc], node[+0x14], node[+0x1c], fill)` (`fill` the float at `01011d78`) to
/// `out`, then the fourth lane `(0x010ea5c8 double / m) * 0x0108e058 double` as a float, with
/// `m` the largest of the three widths `node[+0x10] - node[+0xc]`, `[+0x18] - [+0x14]`,
/// `[+0x20] - [+0x1c]` (each taken when it is not below the running value; the first against
/// the double at `01012060`, the running value starting at 0).
pub fn hkp_mopp_default_assembler_get_scale_info(e: &mut Engine, _this: Ptr, node: Ptr, out: Ptr) {
    let (node, out) = (node.addr(), out.addr());
    let fill = e.mem.u32(SPLIT_LANE_FILL);
    let lanes = [
        e.mem.u32(node + 0xc),
        e.mem.u32(node + 0x14),
        e.mem.u32(node + 0x1c),
        fill,
    ];
    store_lanes(e, out, lanes);
    let width = |e: &Engine, high: u32, low: u32| {
        (f64::from(e.mem.f32(node + high)) - f64::from(e.mem.f32(node + low))) as f32
    };
    let mut largest = 0.0f32;
    let first = width(e, 0x10, 0xc);
    if !(f64::from(first) < e.mem.f64(SCALE_INFO_FLOOR)) {
        largest = first;
    }
    let second = width(e, 0x18, 0x14);
    if !(f64::from(second) < f64::from(largest)) {
        largest = second;
    }
    let third = width(e, 0x20, 0x1c);
    if !(f64::from(third) < f64::from(largest)) {
        largest = third;
    }
    let quotient = x87_div(
        e.mem.f64(SCALE_INFO_NUMERATOR),
        f64::from(x87_quiet(largest)),
    );
    let result = x87_mul(quotient, e.mem.f64(SCALE_INFO_FACTOR)) as f32;
    e.mem.set_u32(out + 0xc, result.to_bits());
}

// Translated from 00e07c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::assembleCutAndTerminalCommand` (Xbox PDB): marks the node chain
/// (`fn_00e06b20`), counts one more command in the word at `this` +0x28, appends the terminals
/// (`addTerminals`) and the cut (`addCut`), records the generator's count in `node` +0x5c, sets
/// the byte at +0x38 and returns the bytes appended.
pub fn hkp_mopp_default_assembler_assemble_cut_and_terminal_command(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: Ptr,
    third: Ptr,
) -> i32 {
    fn_00e06b20(e, node);
    let generator = e.mem.u32(this.addr() + 0x10);
    let commands = e.mem.u32(this.addr() + 0x28).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0x28, commands);
    let start = e.mem.u32(generator + 0xc);
    hkp_mopp_default_assembler_add_terminals(e, this, node, second, third);
    hkp_mopp_default_assembler_add_cut(e, this, node, third);
    let generator = e.mem.u32(this.addr() + 0x10);
    let used = e.mem.u32(generator + 0xc);
    e.mem.set_u32(node.addr() + 0x5c, used);
    e.mem.set_u8(node.addr() + 0x38, 1);
    let generator = e.mem.u32(this.addr() + 0x10);
    e.mem.u32(generator + 0xc).wrapping_sub(start) as i32
}

// Translated from 00e07ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::assembleNonTerminalCommand` (Xbox PDB): marks the node chain,
/// takes the split code of `node`'s vector (`getSplitCode`) and the two cut indices
/// (`findPlanes`), appends the split command (`fn_00e07680` with the cut indices, the code
/// and the words +0x5c of the two children), the property change `info[+0x34] -
/// second[+0x34]` when not zero (`checkAndAddProperties`), `fn_00e06db0` and the cut, and the
/// rescale (`addRescale`) when the byte at `rescale` is set; records the generator's count in
/// `node` +0x5c, sets the byte at +0x38 and returns the bytes appended.
pub fn hkp_mopp_default_assembler_assemble_non_terminal_command(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: Ptr,
    info: Ptr,
    rescale: Ptr,
) -> i32 {
    fn_00e06b20(e, node);
    let generator = e.mem.u32(this.addr() + 0x10);
    let start = e.mem.u32(generator + 0xc);
    let vector = Ptr::new(e.mem.u32(node.addr() + 0x60));
    let code = hkp_mopp_default_assembler_get_split_code(e, this, vector);
    let (high, low) = e.with_stack(8, |e, indices| {
        hkp_mopp_default_assembler_find_planes(
            e,
            this,
            node,
            info,
            indices,
            Ptr::new(indices.addr() + 4),
        );
        (e.mem.i32(indices.addr()), e.mem.i32(indices.addr() + 4))
    });
    let later = e.mem.u32(node.addr() + 0x74);
    let later_end = e.mem.i32(later + 0x5c);
    let earlier = e.mem.u32(node.addr() + 0x70);
    let earlier_end = e.mem.i32(earlier + 0x5c);
    fn_00e07680(e, this, low, high, code as i32, earlier_end, later_end);
    let difference = e
        .mem
        .u32(info.addr() + 0x34)
        .wrapping_sub(e.mem.u32(second.addr() + 0x34));
    if difference != 0 {
        hkp_mopp_default_assembler_check_and_add_properties(e, this, difference);
    }
    fn_00e06db0(e, this, node, second, info);
    hkp_mopp_default_assembler_add_cut(e, this, node, info);
    if e.mem.u8(rescale.addr()) != 0 {
        hkp_mopp_default_assembler_add_rescale(e, this, rescale);
    }
    let generator = e.mem.u32(this.addr() + 0x10);
    let used = e.mem.u32(generator + 0xc);
    e.mem.set_u32(node.addr() + 0x5c, used);
    e.mem.set_u8(node.addr() + 0x38, 1);
    let generator = e.mem.u32(this.addr() + 0x10);
    e.mem.u32(generator + 0xc).wrapping_sub(start) as i32
}

// Translated from 00e07d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpMoppDefaultAssembler`, no map name): copies a settings block into the
/// assembler: the floats at `source` +0, +4, +8 (through the x87, so a signalling NaN comes
/// out quiet) to `this` +0x50, +0x54, +0x58, the sixteen bytes at `source` +0x10 to +0x60, the
/// word at +0x20 to +0x70 and the byte at +0x24 to +0x74.
pub fn fn_00e07d60(e: &mut Engine, this: Ptr, source: Ptr) {
    let (this, source) = (this.addr(), source.addr());
    for lane in 0..3u32 {
        let bits = x87_quiet_bits(e.mem.u32(source + 4 * lane));
        e.mem.set_u32(this + 0x50 + 4 * lane, bits);
    }
    let lanes = load_lanes(e, source + 0x10);
    store_lanes(e, this + 0x60, lanes);
    let word = e.mem.u32(source + 0x20);
    e.mem.set_u32(this + 0x70, word);
    let byte = e.mem.u8(source + 0x24);
    e.mem.set_u8(this + 0x74, byte);
}

// Translated from 00e07da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::preCalcCutPositions` (Xbox PDB): nothing when the byte at `node`
/// +0x38 is set. A leaf (byte +4) snaps the extents of `info` (`fn_00e06730`) and runs
/// `calcTermCut(node, info)`. An inner node first requantises through `parent` when its byte
/// +0x3a is 1 (`fn_00e07070`), snaps `info`, takes the split code and the cut indices (the
/// exe discards the indices), converts `this[+0x54] * this[+0x3c]` with `hkFloatToInt` and
/// discards the result, resets the bounds `node[+0x44 + 4 * i] = 0` and `node[+0x50 + 4 * i] =
/// 0x7fffffff`, then for each child (the one at +0x74, then the one at +0x70) builds a copy of
/// `info` for it (`hkpMoppAssemblerNodeInfo` constructor), recurses, resets the child's bound
/// number `code` (the one at +0x44 for the +0x74 child, +0x50 for the other, when `code <= 2`)
/// and merges the child's cut flags and bounds into `node`'s lane by lane.
pub fn hkp_mopp_default_assembler_pre_calc_cut_positions(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    parent: Ptr,
    info: Ptr,
) {
    let (this_addr, node_addr, info_addr) = (this.addr(), node.addr(), info.addr());
    if e.mem.u8(node_addr + 0x38) != 0 {
        return;
    }
    if e.mem.u8(node_addr + 4) != 0 {
        fn_00e06730(e, info);
        hkp_mopp_default_assembler_calc_term_cut(e, this, node, info);
        return;
    }
    if e.mem.u8(node_addr + 0x3a) == 1 {
        e.with_stack(0x14, |e, out| fn_00e07070(e, parent, info, out));
    }
    fn_00e06730(e, info);
    let vector = Ptr::new(e.mem.u32(node_addr + 0x60));
    let code = hkp_mopp_default_assembler_get_split_code(e, this, vector) as i32;
    e.with_stack(8, |e, indices| {
        hkp_mopp_default_assembler_find_planes(
            e,
            this,
            node,
            info,
            indices,
            Ptr::new(indices.addr() + 4),
        );
    });
    let later = e.mem.u32(node_addr + 0x74);
    let earlier = e.mem.u32(node_addr + 0x70);
    for lane in 0..3u32 {
        let scale = x87_quiet(e.mem.f32(this_addr + 0x3c));
        let product = x87_mul(
            f64::from(x87_quiet(e.mem.f32(this_addr + 0x54))),
            f64::from(scale),
        ) as f32;
        e.call(FLOAT_TO_INT, &args![product.to_bits()]);
        e.mem.set_u32(node_addr + 0x44 + 4 * lane, 0);
        e.mem.set_u32(node_addr + 0x50 + 4 * lane, 0x7fff_ffff);
    }
    for child in [later, earlier] {
        if child == 0 {
            continue;
        }
        e.with_stack(0x48, |e, copy| {
            hkp_mopp_assembler_node_info_construct(
                e,
                copy,
                info,
                Ptr::new(child),
                Ptr::new(this_addr + 0x30),
            );
            hkp_mopp_default_assembler_pre_calc_cut_positions(e, this, Ptr::new(child), info, copy);
        });
        if child == later {
            if code <= 2 {
                e.mem
                    .set_u32(child.wrapping_add((code as u32).wrapping_mul(4)) + 0x44, 0);
            }
        } else if code <= 2 {
            e.mem.set_u32(
                child.wrapping_add((code as u32).wrapping_mul(4)) + 0x50,
                0x7fff_ffff,
            );
        }
        for lane in 0..3u32 {
            let flag = e.mem.i8(child + 0x3b + lane);
            let child_low = e.mem.i32(child + 0x44 + 4 * lane);
            let child_high = e.mem.i32(child + 0x50 + 4 * lane);
            if flag == 1 {
                if !(child_low > e.mem.i32(info_addr + 0x48 + 4 * lane))
                    && !(child_high < e.mem.i32(info_addr + 0x54 + 4 * lane))
                {
                    let current = e.mem.i8(node_addr + 0x3b + lane);
                    e.mem.set_i8(node_addr + 0x3b + lane, current.max(1));
                    e.mem.set_u8(child + 0x3b + lane, 0);
                }
            } else if flag == 2
                && !(child_low > e.mem.i32(info_addr + 0xc + 8 * lane))
                && !(child_high < e.mem.i32(info_addr + 0x10 + 8 * lane))
            {
                e.mem.set_u8(node_addr + 0x3b + lane, 2);
                e.mem.set_u8(child + 0x3b + lane, 0);
            }
            if e.mem.u8(child + 0x3b + lane) == 0 {
                let node_low = e.mem.i32(node_addr + 0x44 + 4 * lane);
                e.mem
                    .set_i32(node_addr + 0x44 + 4 * lane, node_low.max(child_low));
                let node_high = e.mem.i32(node_addr + 0x50 + 4 * lane);
                e.mem
                    .set_i32(node_addr + 0x50 + 4 * lane, node_high.min(child_high));
            }
        }
    }
}

/// Pushes an element of an `hkArray` (data, size, capacity-and-flags) of eight-byte elements:
/// asks the memory router first, grows the array through `hkArrayUtil` when its size has
/// reached its capacity, bumps the size and returns the address of the new element.
fn array_push_slot(e: &mut Engine, array: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    slot
}

// Translated from 00e08040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::assemblesubNode` (Xbox PDB): returns 3 when the byte at `node`
/// +0x38 is set. When `info[0] <= this[+0x18]`, `info[0]` divides evenly by `this[+0x70]`
/// (a zero divisor faults in the exe; here it panics) and the byte at `info` +4 is zero, it
/// sets that byte and assembles the node again and again from fresh copies of `info`, lowering
/// `this[+0x18]` by `this[+0x70]` after each pass that emitted nothing (stopping when it would
/// go negative, when `this[+0x28]` exceeds `this[+0x24]`, or once the node's byte is set),
/// restores `this[+0x18]` and the byte, and returns the last pass's result.
///
/// Otherwise a leaf is assembled as a cut and terminal command when `fn_00e06af0` says so (else
/// -1); an inner node runs `initTopDown`, assembles the child at +0x74 and the child at +0x70
/// from `info` copies built for them, releases them, and when both succeeded and `fn_00e06af0`
/// allows it appends the non-terminal command and returns its size plus the two child results;
/// any failure gives -1.
pub fn hkp_mopp_default_assembler_assemblesub_node(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: Ptr,
    info: Ptr,
) -> i32 {
    let (this_addr, node_addr, info_addr) = (this.addr(), node.addr(), info.addr());
    if e.mem.u8(node_addr + 0x38) != 0 {
        return 3;
    }
    let level = e.mem.i32(info_addr);
    if !(level > e.mem.i32(this_addr + 0x18))
        && level % e.mem.i32(this_addr + 0x70) == 0
        && e.mem.u8(info_addr + 4) == 0
    {
        e.mem.set_u8(info_addr + 4, 1);
        let saved = e.mem.u32(this_addr + 0x18);
        let mut result = -1;
        if e.mem.u8(node_addr + 0x38) == 0 {
            loop {
                let generator = e.mem.u32(this_addr + 0x10);
                let used = e.mem.u32(generator + 0xc);
                result = e.with_stack(0x48, |e, copy| {
                    for word in 0..0x12u32 {
                        let value = e.mem.u32(info_addr + 4 * word);
                        e.mem.set_u32(copy.addr() + 4 * word, value);
                    }
                    hkp_mopp_default_assembler_assemblesub_node(e, this, node, second, copy)
                });
                if e.mem.i32(this_addr + 0x28) > e.mem.i32(this_addr + 0x24) {
                    break;
                }
                let generator = e.mem.u32(this_addr + 0x10);
                if used == e.mem.u32(generator + 0xc) {
                    let limit = e.mem.i32(this_addr + 0x18);
                    if limit < 0 {
                        break;
                    }
                    let lowered = limit.wrapping_sub(e.mem.i32(this_addr + 0x70));
                    e.mem.set_i32(this_addr + 0x18, lowered);
                }
                if e.mem.u8(node_addr + 0x38) != 0 {
                    break;
                }
            }
        }
        e.mem.set_u32(this_addr + 0x18, saved);
        e.mem.set_u8(info_addr + 4, 0);
        return result;
    }
    if e.mem.u8(node_addr + 4) != 0 {
        fn_00e06b20(e, node);
        let allowed = e.with_stack(4, |e, flag| {
            let flag = fn_00e06af0(e, this, flag, node, info);
            e.mem.u8(flag.addr()) != 0
        });
        if !allowed {
            return -1;
        }
        return hkp_mopp_default_assembler_assemble_cut_and_terminal_command(
            e, this, node, second, info,
        );
    }
    e.with_stack(0x14, |e, commands| {
        e.mem.set_u8(commands.addr(), 0);
        e.mem.set_i32(commands.addr() + 4, -1);
        hkp_mopp_default_assembler_init_top_down(e, this, node, second, info, commands);
        let later = e.mem.u32(node_addr + 0x74);
        let mut later_result = -1;
        if later != 0 {
            later_result = e.with_stack(0x48, |e, copy| {
                hkp_mopp_assembler_node_info_construct(
                    e,
                    copy,
                    info,
                    Ptr::new(later),
                    Ptr::new(this_addr + 0x30),
                );
                hkp_mopp_default_assembler_assemblesub_node(e, this, Ptr::new(later), info, copy)
            });
            if later_result >= 0 {
                hkp_mopp_default_assembler_release_node(e, this, Ptr::new(later));
            }
        }
        let earlier = e.mem.u32(node_addr + 0x70);
        if earlier == 0 {
            return -1;
        }
        let earlier_result = e.with_stack(0x48, |e, copy| {
            hkp_mopp_assembler_node_info_construct(
                e,
                copy,
                info,
                Ptr::new(earlier),
                Ptr::new(this_addr + 0x30),
            );
            hkp_mopp_default_assembler_assemblesub_node(e, this, Ptr::new(earlier), info, copy)
        });
        if earlier_result < 0 {
            return -1;
        }
        hkp_mopp_default_assembler_release_node(e, this, Ptr::new(earlier));
        if later_result < 0 {
            return -1;
        }
        let allowed = e.with_stack(4, |e, flag| {
            let flag = fn_00e06af0(e, this, flag, node, info);
            e.mem.u8(flag.addr()) != 0
        });
        if !allowed {
            return -1;
        }
        let commands_count = e.mem.u32(this_addr + 0x28).wrapping_add(1);
        e.mem.set_u32(this_addr + 0x28, commands_count);
        hkp_mopp_default_assembler_assemble_non_terminal_command(
            e, this, node, second, info, commands,
        )
        .wrapping_add(earlier_result)
        .wrapping_add(later_result)
    })
}

// Translated from 00e08240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::calcTerminalIdsForChunks` (Xbox PDB): clears the byte at `node`
/// +0x38; takes the chunk number `chunk = node[+0x40]` and a counter of 0 when `node[+0x40]` is
/// not negative, else `chunk = second` and the counter `third`. When the word at `this[+0x14]`
/// +0x1c is set, the node's word +0x24 gets the counter. A leaf then (with that word set)
/// copies its terminal list's first word to +0x10, appends the pair `(old first word, chunk
/// << 8 | counter)` to the array at `this[+0x14]` +4, stores the counter as the first word and
/// counts it; an inner node recurses into its +0x74 and +0x70 children, threading the
/// counter. With the word set, +0x24 becomes 0 and +0x28 the counter. Returns the counter when
/// `node[+0x40]` is negative, else `third`.
pub fn hkp_mopp_default_assembler_calc_terminal_ids_for_chunks(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: u32,
    third: u32,
) -> u32 {
    let (this_addr, node_addr) = (this.addr(), node.addr());
    e.mem.set_u8(node_addr + 0x38, 0);
    let (chunk, mut counter) = if e.mem.i32(node_addr + 0x40) >= 0 {
        (e.mem.u32(node_addr + 0x40), 0)
    } else {
        (second, third)
    };
    let table = e.mem.u32(this_addr + 0x14);
    if e.mem.u32(table + 0x1c) != 0 {
        e.mem.set_u32(node_addr + 0x24, counter);
    }
    if e.mem.u8(node_addr + 4) != 0 {
        let table = e.mem.u32(this_addr + 0x14);
        if e.mem.u32(table + 0x1c) != 0 {
            let terminals = e.mem.u32(node_addr + 0x60);
            let first = e.mem.u32(terminals);
            e.mem.set_u32(terminals + 0x10, first);
            let table = e.mem.u32(this_addr + 0x14);
            let slot = array_push_slot(e, table + 4);
            e.mem.set_u32(slot + 4, (chunk << 8) | counter);
            let terminals = e.mem.u32(node_addr + 0x60);
            let first = e.mem.u32(terminals);
            e.mem.set_u32(slot, first);
            let terminals = e.mem.u32(node_addr + 0x60);
            e.mem.set_u32(terminals, counter);
            counter = counter.wrapping_add(1);
        }
    } else {
        let later = Ptr::new(e.mem.u32(node_addr + 0x74));
        let after_later =
            hkp_mopp_default_assembler_calc_terminal_ids_for_chunks(e, this, later, chunk, counter);
        let earlier = Ptr::new(e.mem.u32(node_addr + 0x70));
        counter = hkp_mopp_default_assembler_calc_terminal_ids_for_chunks(
            e,
            this,
            earlier,
            chunk,
            after_later,
        );
    }
    let table = e.mem.u32(this_addr + 0x14);
    if e.mem.u32(table + 0x1c) != 0 {
        e.mem.set_u32(node_addr + 0x24, 0);
        e.mem.set_u32(node_addr + 0x28, counter);
    }
    if e.mem.i32(node_addr + 0x40) < 0 {
        counter
    } else {
        third
    }
}

// Translated from 00e08320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyJumpCommandInfo` (Xbox PDB), cdecl with the destination array in `ESI`: given first
/// the register `destination` (an `hkArray` of eight-byte elements), then `source` (the same
/// kind of array) and `offset`, appends for every element of `source` the pair `(first word +
/// offset, second word)` to `destination`.
pub fn copy_jump_command_info(e: &mut Engine, destination: Ptr, source: Ptr, offset: i32) {
    let (destination, source) = (destination.addr(), source.addr());
    let mut index = 0u32;
    if 0 < e.mem.i32(source + 4) {
        loop {
            let slot = array_push_slot(e, destination);
            let data = e.mem.u32(source);
            let second = e.mem.u32(data.wrapping_add(index.wrapping_mul(8)) + 4);
            e.mem.set_u32(slot + 4, second);
            let data = e.mem.u32(source);
            let first = e
                .mem
                .u32(data.wrapping_add(index.wrapping_mul(8)))
                .wrapping_add(offset as u32);
            index += 1;
            e.mem.set_u32(slot, first);
            if index as i32 >= e.mem.i32(source + 4) {
                break;
            }
        }
    }
}

// Translated from 00e08380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyCodeAndEmbedTerminals` (Xbox PDB), cdecl with the destination generator in `EDI`:
/// given first the register `generator`, then `assembler`, `source` (a generator whose code
/// is copied) and `terminals` (a data pointer and a count). With the object at `assembler`
/// +0x1c missing the count is zeroed. A non-zero count asks that object (slot +4) for the size
/// of the embedded terminal data, pads the generator by it, carves a 128-byte-rounded area off
/// the memory router's cursor, has the object (slot +8) write the terminals there, copies them
/// into the generator and puts the cursor back. Then it pads the generator by `source`'s
/// count plus the padding to a multiple of 4, copies `source`'s code, runs `validateJumpCommands`
/// (`00e06480`) on both generators around `copyJumpCommandInfo` of the source's jump array
/// (+0x14), offset by the difference of the counts. Returns the bytes appended after the
/// terminal data.
pub fn copy_code_and_embed_terminals(
    e: &mut Engine,
    generator: Ptr,
    assembler: Ptr,
    source: Ptr,
    terminals: Ptr,
) -> i32 {
    let (generator, assembler, source, terminals) = (
        generator.addr(),
        assembler.addr(),
        source.addr(),
        terminals.addr(),
    );
    let padding = e.mem.u32(source + 0xc).wrapping_neg() & 3;
    if e.mem.u32(assembler + 0x1c) == 0 {
        e.mem.set_u32(terminals + 4, 0);
    }
    let count = e.mem.u32(terminals + 4);
    if count != 0 {
        let data = e.mem.u32(terminals);
        let object = e.mem.u32(assembler + 0x1c);
        let size = e.vcall(object, 4, &args![data, count]).u32();
        hkp_mopp_code_generator_pad(e, Ptr::new(generator), size as i32);
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        let cursor = e.mem.u32(router);
        let rounded = size.wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, rounded.wrapping_add(cursor));
        let object = e.mem.u32(assembler + 0x1c);
        let count = e.mem.u32(terminals + 4);
        let data = e.mem.u32(terminals);
        e.vcall(object, 8, &args![data, count, cursor]);
        let target = e
            .mem
            .u32(generator + 0x10)
            .wrapping_sub(e.mem.u32(generator + 0xc))
            .wrapping_add(e.mem.u32(generator + 8));
        e.call(MEM_COPY, &args![target, cursor, size]);
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        e.mem.set_u32(router, cursor);
    }
    let start = e.mem.u32(generator + 0xc);
    let length = e.mem.u32(source + 0xc);
    hkp_mopp_code_generator_pad(e, Ptr::new(generator), length.wrapping_add(padding) as i32);
    let from = e
        .mem
        .u32(source + 0x10)
        .wrapping_add(e.mem.u32(source + 8))
        .wrapping_sub(length);
    let to = e
        .mem
        .u32(generator + 0x10)
        .wrapping_add(e.mem.u32(generator + 8))
        .wrapping_sub(e.mem.u32(generator + 0xc));
    e.call(MEM_COPY, &args![to, from, length]);
    e.call(VALIDATE_JUMP_COMMANDS, &args![source]);
    let offset = e
        .mem
        .u32(generator + 0xc)
        .wrapping_sub(e.mem.u32(source + 0xc)) as i32;
    copy_jump_command_info(
        e,
        Ptr::new(generator + 0x14),
        Ptr::new(source + 0x14),
        offset,
    );
    e.call(VALIDATE_JUMP_COMMANDS, &args![generator]);
    e.mem.u32(generator + 0xc).wrapping_sub(start) as i32
}

// Translated from 00e08470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::assembleJumpChunkCommand` (Xbox PDB): appends the command of
/// `fn_00e06c20(node[+0x40])` to the generator at `this` +0x10, records the generator's count
/// in `node` +0x5c, sets the byte at +0x38, appends the pair `(count, node[+0x40])` to the
/// generator's jump array (+0x14) and runs `validateJumpCommands` (`00e06480`) on it. Returns
/// the bytes the command took.
pub fn hkp_mopp_default_assembler_assemble_jump_chunk_command(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
) -> i32 {
    let (this_addr, node_addr) = (this.addr(), node.addr());
    let generator = e.mem.u32(this_addr + 0x10);
    let start = e.mem.u32(generator + 0xc);
    let target = e.mem.u32(node_addr + 0x40);
    fn_00e06c20(e, this, target);
    let generator = e.mem.u32(this_addr + 0x10);
    let used = e.mem.u32(generator + 0xc);
    e.mem.set_u32(node_addr + 0x5c, used);
    e.mem.set_u8(node_addr + 0x38, 1);
    let generator = e.mem.u32(this_addr + 0x10);
    let slot = array_push_slot(e, generator + 0x14);
    let target = e.mem.u32(node_addr + 0x40);
    e.mem.set_u32(slot + 4, target);
    e.mem.set_u32(slot, used);
    let generator = e.mem.u32(this_addr + 0x10);
    e.call(VALIDATE_JUMP_COMMANDS, &args![generator]);
    used.wrapping_sub(start) as i32
}

/// The object at `this` +0x14 +0x1c (the terminal-data embedder; zero when absent).
fn terminal_embedder(e: &Engine, this: u32) -> u32 {
    let table = e.mem.u32(this + 0x14);
    e.mem.u32(table + 0x1c)
}

/// The generator's count of used bytes (`this` +0x10, +0xc).
fn generator_used(e: &Engine, this: u32) -> u32 {
    let generator = e.mem.u32(this + 0x10);
    e.mem.u32(generator + 0xc)
}

/// The allocator the memory router hands out (`00c85750`, +0x10).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.u32(router + 0x10)
}

/// Releases the storage of an `hkArray` of four-byte elements the way the exe's inline
/// destructor does: the data goes back through allocator slot +0x10 when the flags word does
/// not have bit 31 set, then the array is reset.
fn array_release(e: &mut Engine, array: u32) {
    let allocator = router_allocator(e);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if flags & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, (flags & 0x3fff_ffff) << 2]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Inserts the elements of `source` (an `hkArray` of four-byte elements) at `index` of
/// `destination` through `00cd1e20`.
fn array_insert(e: &mut Engine, destination: u32, index: u32, source: u32) {
    let data = e.mem.u32(source);
    let count = e.mem.u32(source + 4);
    let allocator = router_allocator(e);
    e.call(
        ARRAY_INSERT_AT,
        &args![destination, allocator, index, data, count],
    );
}

/// Pushes the word at `source` onto an `hkArray` of four-byte elements, growing the array
/// first when it is full.
fn array_push_word_from(e: &mut Engine, array: u32, source: u32) {
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let count = e.mem.u32(array + 4);
    let word = e.mem.u32(source);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(count.wrapping_mul(4)), word);
    e.mem.set_u32(array + 4, count.wrapping_add(1));
}

// Translated from 00e084f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::calcChunkPoints` (Xbox PDB): like `assemblesubNode`, but cuts the
/// tree into chunks. `node` is the subtree, `second` the info of the level above, `info` the
/// node's own info; `terminals_out` (an `hkArray` of words) receives the terminal ids that
/// stay in this chunk. Returns the bytes the subtree takes.
///
/// A leaf is assembled as a cut and terminal command (`assembleCutAndTerminalCommand`) and its first terminal
/// word pushed to `terminals_out`; with an embedder (`this` +0x14 +0x1c) the size of that word
/// is added (embedder slot +4 on `(&word, 1)`). An inner node assembles both children into
/// lists of their own (the child at +0x74 first), then the non-terminal command, and merges
/// the lists. While the total is not below `table[0] - table[0x20]` (`table` = `this`
/// +0x14) it turns the larger child into a five-byte jump to a new chunk (appends a jump
/// record to `table` +0x10, sets the child's word +0x40, and moves the generator's count and
/// the nodes' words +0x5c by `5 - used`), then merges the lists of the children that stayed.
pub fn hkp_mopp_default_assembler_calc_chunk_points(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: Ptr,
    info: Ptr,
    terminals_out: Ptr,
) -> i32 {
    let (this_addr, node_addr, second_addr, info_addr, out) = (
        this.addr(),
        node.addr(),
        second.addr(),
        info.addr(),
        terminals_out.addr(),
    );
    if terminal_embedder(e, this_addr) != 0 {
        let carried = e.mem.u32(second_addr + 0x34);
        e.mem.set_u32(info_addr + 0x34, carried);
    }
    if e.mem.u8(node_addr + 4) != 0 {
        if terminal_embedder(e, this_addr) != 0 {
            let terminals = e.mem.u32(node_addr + 0x60);
            let first = e.mem.u32(terminals);
            e.mem.set_u32(info_addr + 0x34, first);
        }
        let start = generator_used(e, this_addr);
        hkp_mopp_default_assembler_assemble_cut_and_terminal_command(e, this, node, second, info);
        let size = generator_used(e, this_addr).wrapping_sub(start) as i32;
        let terminals = e.mem.u32(node_addr + 0x60);
        array_push_word_from(e, out, terminals);
        let terminals = e.mem.u32(node_addr + 0x60);
        let first = e.mem.u32(terminals);
        let embedder = terminal_embedder(e, this_addr);
        if embedder == 0 {
            return size;
        }
        let added = e.with_stack(4, |e, word| {
            e.mem.set_u32(word.addr(), first);
            e.vcall(embedder, 4, &args![word, 1u32]).i32()
        });
        return added.wrapping_add(size);
    }
    e.with_stack(0x80, |e, frame| {
        let copy = Ptr::new(frame.addr());
        let later_list = frame.addr() + 0x48;
        let earlier_list = frame.addr() + 0x54;
        let merged = frame.addr() + 0x60;
        let commands = Ptr::new(frame.addr() + 0x6c);
        e.mem.set_u8(commands.addr(), 0);
        e.mem.set_i32(commands.addr() + 4, -1);
        hkp_mopp_default_assembler_init_top_down(e, this, node, second, info, commands);
        let carried = e.mem.u32(second_addr + 0x34);
        e.mem.set_u32(info_addr + 0x34, carried);
        for list in [later_list, earlier_list, merged] {
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 8, 0x8000_0000);
        }
        let mut totals = [0i32; 2];
        let mut used = [0i32; 3];
        for (side, (child_offset, list)) in [(0x74u32, later_list), (0x70, earlier_list)]
            .into_iter()
            .enumerate()
        {
            let child = Ptr::new(e.mem.u32(node_addr + child_offset));
            hkp_mopp_assembler_node_info_construct(
                e,
                copy,
                info,
                child,
                Ptr::new(this_addr + 0x30),
            );
            let start = generator_used(e, this_addr);
            hkp_mopp_default_assembler_calc_chunk_points(
                e,
                this,
                child,
                info,
                copy,
                Ptr::new(list),
            );
            let size = generator_used(e, this_addr).wrapping_sub(start) as i32;
            used[side] = size;
            totals[side] = size;
            let embedder = terminal_embedder(e, this_addr);
            if embedder != 0 && e.mem.u32(list + 4) != 0 {
                let data = e.mem.u32(list);
                let count = e.mem.u32(list + 4);
                let added = e.vcall(embedder, 4, &args![data, count]).i32();
                totals[side] = added.wrapping_add(size);
            }
        }
        let start = generator_used(e, this_addr);
        hkp_mopp_default_assembler_assemble_non_terminal_command(
            e, this, node, second, info, commands,
        );
        used[2] = generator_used(e, this_addr).wrapping_sub(start) as i32;
        array_insert(e, merged, 0, later_list);
        let at = e.mem.u32(merged + 4);
        array_insert(e, merged, at, earlier_list);
        let mut sum = used[1].wrapping_add(used[2]).wrapping_add(used[0]);
        let embedder = terminal_embedder(e, this_addr);
        if embedder != 0 && e.mem.u32(merged + 4) != 0 {
            let data = e.mem.u32(merged);
            let count = e.mem.u32(merged + 4);
            sum = sum.wrapping_add(e.vcall(embedder, 4, &args![data, count]).i32());
        }
        let mut later_jumped = false;
        let mut earlier_jumped = false;
        let table_limit = |e: &Engine| {
            let table = e.mem.u32(this_addr + 0x14);
            e.mem.i32(table).wrapping_sub(e.mem.i32(table + 0x20))
        };
        if sum >= table_limit(e) {
            loop {
                let table = e.mem.u32(this_addr + 0x14);
                let chunk = e.mem.u32(table + 0x14);
                let slot = array_push_slot(e, table + 0x10);
                let (chosen_total, chosen_used, child) = if totals[1] > totals[0] {
                    let chosen = (totals[1], used[1], e.mem.u32(node_addr + 0x70));
                    totals[1] = 5;
                    earlier_jumped = true;
                    sum = if later_jumped {
                        used[2].wrapping_add(10)
                    } else {
                        used[2].wrapping_add(totals[0]).wrapping_add(5)
                    };
                    chosen
                } else {
                    let chosen = (totals[0], used[0], e.mem.u32(node_addr + 0x74));
                    totals[0] = 5;
                    later_jumped = true;
                    sum = if earlier_jumped {
                        used[2].wrapping_add(10)
                    } else {
                        used[2].wrapping_add(totals[1]).wrapping_add(5)
                    };
                    chosen
                };
                let reserve = e.mem.i32(table + 0x20);
                let target = reserve.wrapping_add(chosen_total).wrapping_add(5);
                e.mem.set_i32(slot + 4, target);
                e.mem.set_u32(child + 0x40, chunk);
                let shrink = 5i32.wrapping_sub(chosen_used);
                let generator = e.mem.u32(this_addr + 0x10);
                let count = e.mem.i32(generator + 0xc).wrapping_add(shrink);
                e.mem.set_i32(generator + 0xc, count);
                if later_jumped {
                    let later = e.mem.u32(node_addr + 0x74);
                    let position = e.mem.i32(later + 0x5c).wrapping_add(shrink);
                    e.mem.set_i32(later + 0x5c, position);
                }
                let earlier = e.mem.u32(node_addr + 0x70);
                let position = e.mem.i32(earlier + 0x5c).wrapping_add(shrink);
                e.mem.set_i32(earlier + 0x5c, position);
                let position = e.mem.i32(node_addr + 0x5c).wrapping_add(shrink);
                e.mem.set_i32(node_addr + 0x5c, position);
                if sum < table_limit(e) {
                    break;
                }
            }
        }
        e.mem.set_u32(out + 4, 0);
        if !later_jumped {
            array_insert(e, out, 0, later_list);
        }
        if !earlier_jumped {
            let at = e.mem.u32(out + 4);
            array_insert(e, out, at, earlier_list);
        }
        array_release(e, merged);
        array_release(e, earlier_list);
        array_release(e, later_list);
        sum
    })
}

// Translated from 00e08a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::assembleSubNodeIntoChunks` (Xbox PDB; `this` in `ECX`): assembles
/// the subtree at `node` into the chunks `calcChunkPoints` planned. `second` and `info` are
/// the level above's and the node's infos, `chunk` is forwarded unchanged to the children (a
/// word the exe never reads here) and `list` is an `hkArray` of words that collects the
/// terminal ids.
///
/// A node whose chunk number (word +0x40) is positive and whose info byte +4 is clear is a
/// chunk of its own: a `hkpMoppCodeGenerator` on the stack (capacity from the jump record of
/// that chunk in the array at `this` +0x14 +0x10) takes the place of the one at `this` +0x10,
/// the byte at `info` +4 is set, the subtree is assembled into it with a fresh list, the data
/// offset of the embedded terminals is added when the embedder (`this` +0x14 +0x1c) exists, a
/// new heap generator (0x20 bytes from the allocator, flags word `0x20`) is built with the
/// same capacity, stored in the jump record's first word and filled by
/// `copyCodeAndEmbedTerminals`; the list is released, the stack generator destroyed, the old
/// one restored and `assembleJumpChunkCommand(node)` appended.
///
/// Otherwise a leaf is assembled as a cut and terminal command and its terminal word `+0x10`
/// pushed to `list`; an inner node runs `initTopDown`, assembles the child at +0x74 and the
/// child at +0x70 from `info` copies (`hkpMoppAssemblerNodeInfo` constructor) and appends the
/// non-terminal command.
pub fn hkp_mopp_default_assembler_assemble_sub_node_into_chunks(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    second: Ptr,
    info: Ptr,
    chunk: i32,
    list: Ptr,
) {
    let (this_addr, node_addr, info_addr) = (this.addr(), node.addr(), info.addr());
    let target = e.mem.i32(node_addr + 0x40);
    if target > 0 && e.mem.u8(info_addr + 4) == 0 {
        let saved = e.mem.u32(this_addr + 0x10);
        let table = e.mem.u32(this_addr + 0x14);
        let records = e.mem.u32(table + 0x10);
        let record = records.wrapping_add((target as u32).wrapping_mul(8));
        let capacity = e.mem.u32(record + 4);
        e.with_stack(0x20, |e, temporary| {
            e.call(GENERATOR_CTOR, &args![temporary, capacity]);
            e.mem.set_u32(this_addr + 0x10, temporary.addr());
            e.with_stack(0xc, |e, terminals| {
                e.mem.set_u32(terminals.addr(), 0);
                e.mem.set_u32(terminals.addr() + 4, 0);
                e.mem.set_u32(terminals.addr() + 8, 0x8000_0000);
                e.mem.set_u8(info_addr + 4, 1);
                hkp_mopp_default_assembler_assemble_sub_node_into_chunks(
                    e, this, node, second, info, target, terminals,
                );
                if terminal_embedder(e, this_addr) != 0 {
                    let used = generator_used(e, this_addr).wrapping_add(5);
                    let aligned = (used.wrapping_neg() & 3).wrapping_add(used);
                    let count = e.mem.i32(terminals.addr() + 4);
                    hkp_mopp_default_assembler_add_data_offset(e, this, aligned as i32, count);
                }
                let allocator = router_allocator(e);
                let object = e.vcall(allocator, 4, &args![0x20u32]).u32();
                e.mem.set_u16(object + 4, 0x20);
                let capacity = e.mem.u32(record + 4);
                let generator = e.call(GENERATOR_CTOR, &args![object, capacity]).u32();
                e.mem.set_u32(record, generator);
                let source = e.mem.u32(this_addr + 0x10);
                let assembler = e.mem.u32(this_addr + 0x14);
                copy_code_and_embed_terminals(
                    e,
                    Ptr::new(generator),
                    Ptr::new(assembler),
                    Ptr::new(source),
                    terminals,
                );
                let source = e.mem.u32(this_addr + 0x10);
                e.mem.set_u32(source + 0x18, 0);
                array_release(e, terminals.addr());
            });
            e.call(GENERATOR_DTOR, &args![temporary]);
        });
        e.mem.set_u32(this_addr + 0x10, saved);
        hkp_mopp_default_assembler_assemble_jump_chunk_command(e, this, node);
        return;
    }
    if e.mem.u8(node_addr + 4) != 0 {
        hkp_mopp_default_assembler_assemble_cut_and_terminal_command(e, this, node, second, info);
        let terminals = e.mem.u32(node_addr + 0x60);
        array_push_word_from(e, list.addr(), terminals + 0x10);
        return;
    }
    e.with_stack(0x14, |e, commands| {
        e.mem.set_u8(commands.addr(), 0);
        e.mem.set_i32(commands.addr() + 4, -1);
        hkp_mopp_default_assembler_init_top_down(e, this, node, second, info, commands);
        for child_offset in [0x74u32, 0x70] {
            let child = Ptr::new(e.mem.u32(node_addr + child_offset));
            e.with_stack(0x48, |e, copy| {
                hkp_mopp_assembler_node_info_construct(
                    e,
                    copy,
                    info,
                    child,
                    Ptr::new(this_addr + 0x30),
                );
                hkp_mopp_default_assembler_assemble_sub_node_into_chunks(
                    e, this, child, info, copy, chunk, list,
                );
            });
        }
        hkp_mopp_default_assembler_assemble_non_terminal_command(
            e, this, node, second, info, commands,
        );
    });
}

/// Builds the top-level `hkpMoppAssemblerNodeInfo` of the exe's `assemble`: `updateExtents`
/// from `node` and `origin`, then depth -1, the byte at +4 clear, shift 0x10 at +0x24, zero at
/// +0x28..+0x34, `updateOffsetAndSpread(node)` and `calcMaxExtentsSignificantBits`.
fn build_root_info(e: &mut Engine, info: Ptr, node: Ptr, origin: Ptr) {
    hkp_mopp_assembler_node_info_update_extents(e, info, node, origin);
    e.mem.set_i32(info.addr(), -1);
    e.mem.set_u8(info.addr() + 4, 0);
    e.mem.set_u32(info.addr() + 0x24, 0x10);
    for offset in [0x28, 0x2c, 0x30, 0x34] {
        e.mem.set_u32(info.addr() + offset, 0);
    }
    hkp_mopp_assembler_node_info_update_offset_and_spread(e, info, node);
    hkp_mopp_assembler_node_info_calc_max_extents_significant_bits(e, info);
}

// Translated from 00e08cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppDefaultAssembler::assemble` (Xbox PDB; virtual slot +0x14 of the vtable `010ea5dc`;
/// `this` in `ECX`, `RET 0xc`): assembles the tree at `node` into the generator at `this` +0x10.
/// `owner` is stored to `this` +0xc (the object `releaseNode` calls) and `command_limit` to
/// `this` +0x24 (the unconfirmed upper bound on the number of non-terminal commands);
/// `this` +0x28 starts at 0. Virtual slot +0x18 (`getScaleInfo`) fills the vector at `this`
/// +0x30 from `node`; a null `node` stops there.
///
/// Otherwise the scale exponent range comes from the significant bits of `node` +0x28 (stored
/// at +0x18 as `(bits / this[+0x70] + 2) * this[+0x70] - 1`, half the bits at +0x1c; a zero
/// `this[+0x70]` faults in the exe and panics here), then `preCalcScale` and
/// `preCalcCutPositions` run on top-level node infos built for it, and the tree is assembled
/// with `assemblesubNode`, or with chunks when `this` +0x14 is set: `node` +0x40 is cleared, a
/// record slot is added to the array at `this` +0x14 +0x10, a stack generator of 0x100000
/// bytes takes the place of the one at `this` +0x10 for `calcChunkPoints` and
/// `calcTerminalIdsForChunks`, is rebuilt for `assembleSubNodeIntoChunks` (plus the data offset
/// of the embedded terminals), the old generator is restored and
/// `copyCodeAndEmbedTerminals` copies the code into it; the record at `node` +0x40 gets the
/// generator (with a reference added, `00c90510`) and its used bytes.
pub fn hkp_mopp_default_assembler_assemble(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    owner: u32,
    command_limit: i32,
) {
    let this_addr = this.addr();
    let node_addr = node.addr();
    e.mem.set_u32(this_addr + 0xc, owner);
    e.mem.set_i32(this_addr + 0x24, command_limit);
    e.mem.set_u32(this_addr + 0x28, 0);
    e.vcall(this_addr, 0x18, &args![node, this_addr + 0x30]);
    if node_addr == 0 {
        return;
    }
    let bits = significant_bits(e.mem.u32(node_addr + 0x28));
    e.mem.set_i32(this_addr + 0x18, bits);
    e.mem.set_i32(this_addr + 0x1c, bits >> 1);
    let step = e.mem.i32(this_addr + 0x70);
    let range = (bits / step)
        .wrapping_add(2)
        .wrapping_mul(step)
        .wrapping_sub(1);
    e.mem.set_i32(this_addr + 0x18, range);
    let origin = Ptr::new(this_addr + 0x30);
    e.with_stack(0x184, |e, frame| {
        let at = |offset: u32| Ptr::new(frame.addr() + offset);
        let (info_a, info_b, info_c, info_d) = (at(0x20), at(0x68), at(0x130), at(0xe8));
        build_root_info(e, info_a, node, origin);
        hkp_mopp_assembler_node_info_construct(e, info_b, info_a, node, origin);
        hkp_mopp_default_assembler_pre_calc_scale(e, this, node, info_a, info_b);
        build_root_info(e, info_b, node, origin);
        hkp_mopp_assembler_node_info_construct(e, info_c, info_b, node, origin);
        hkp_mopp_default_assembler_pre_calc_cut_positions(e, this, node, info_b, info_c);
        hkp_mopp_assembler_node_info_construct(e, info_d, info_a, node, origin);
        if e.mem.u32(this_addr + 0x14) == 0 {
            hkp_mopp_default_assembler_assemblesub_node(e, this, node, info_a, info_d);
            return;
        }
        e.mem.set_u32(node_addr + 0x40, 0);
        let table = e.mem.u32(this_addr + 0x14);
        array_push_slot(e, table + 0x10);
        let saved = e.mem.u32(this_addr + 0x10);
        hkp_mopp_assembler_node_info_construct(e, info_b, info_a, node, origin);
        let list = at(0x10);
        let temporary = at(0xc8);
        e.mem.set_u32(list.addr(), 0);
        e.mem.set_u32(list.addr() + 4, 0);
        e.mem.set_u32(list.addr() + 8, 0x8000_0000);
        e.call(GENERATOR_CTOR, &args![temporary, 0x10_0000u32]);
        e.mem.set_u32(this_addr + 0x10, temporary.addr());
        hkp_mopp_default_assembler_calc_chunk_points(e, this, node, info_a, info_b, list);
        let chunk = e.mem.u32(node_addr + 0x40);
        hkp_mopp_default_assembler_calc_terminal_ids_for_chunks(e, this, node, chunk, 0);
        e.call(GENERATOR_DTOR, &args![temporary]);
        array_release(e, list.addr());
        e.call(GENERATOR_CTOR, &args![temporary, 0x10_0000u32]);
        e.mem.set_u32(this_addr + 0x10, temporary.addr());
        let chunk = e.mem.i32(node_addr + 0x40);
        hkp_mopp_default_assembler_assemble_sub_node_into_chunks(
            e, this, node, info_a, info_d, chunk, list,
        );
        if terminal_embedder(e, this_addr) != 0 {
            let used = generator_used(e, this_addr).wrapping_add(5);
            let aligned = (used.wrapping_neg() & 3).wrapping_add(used);
            let count = e.mem.i32(list.addr() + 4);
            hkp_mopp_default_assembler_add_data_offset(e, this, aligned as i32, count);
        }
        let assembler = e.mem.u32(this_addr + 0x14);
        e.mem.set_u32(this_addr + 0x10, saved);
        copy_code_and_embed_terminals(e, Ptr::new(saved), Ptr::new(assembler), temporary, list);
        let table = e.mem.u32(this_addr + 0x14);
        let records = e.mem.u32(table + 0x10);
        let record = records.wrapping_add(e.mem.u32(node_addr + 0x40).wrapping_mul(8));
        let generator = e.mem.u32(this_addr + 0x10);
        e.mem.set_u32(record, generator);
        e.call(ADD_REFERENCE, &args![generator]);
        let generator = e.mem.u32(this_addr + 0x10);
        let used = e.mem.u32(generator + 0xc);
        e.mem.set_u32(record + 4, used);
        e.call(GENERATOR_DTOR, &args![temporary]);
        array_release(e, list.addr());
    });
}

/// The direction vectors the constructor lays out: their offset in the plane array and the
/// `x`, `y`, `z` lanes as 0, 1 or -1 (the fourth lane is always zero).
const PLANE_DIRECTIONS: [(u32, [i8; 3]); 13] = [
    (0x000, [1, 0, 0]),
    (0x020, [0, 1, 0]),
    (0x040, [0, 0, 1]),
    (0x060, [0, 1, 1]),
    (0x080, [1, 0, 1]),
    (0x0a0, [1, 1, 0]),
    (0x0c0, [0, 1, -1]),
    (0x0e0, [1, 0, -1]),
    (0x100, [1, -1, 0]),
    (0x120, [1, 1, 1]),
    (0x140, [1, 1, -1]),
    (0x160, [1, -1, 1]),
    (0x180, [1, -1, -1]),
];

// Translated from 00e090c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `hkpMoppDefaultAssembler` constructor: it sets the vtable `010ea5dc`; no map
/// name; `this` in `ECX`, `RET 0xc`): sets the word at +6 to 1, the vtable, and an empty
/// array at +0x40 (data, size, flags `0x80000000`); asks the memory router's allocator (slot
/// +0xc, `(&size)` with 0x1a0 in and the granted size out) for the storage of the plane array
/// and takes its size / 32 as the element count (flags `0x80000000` when that is zero).
/// Defaults: floats 0.5, 0.2 and 1.0 at +0x50, +0x54, +0x58, the vector `(0.2, 0.2, 0.05, 0)`
/// at +0x60, 5 at +0x70, the byte 1 at +0x74. The 13 planes (a direction, a float at +0x10 and
/// the index at +0x14 per 0x20 bytes) are written and every direction is normalized with the
/// `RSQRTSS` estimate and one Newton step (zero length gives a zero vector). Then
/// `fn_00e07d60(settings)` overwrites the defaults from `settings`, `third` goes to +8,
/// `generator` to +0x10 and 0 to +0xc. Returns `this`.
pub fn fn_00e090c0(e: &mut Engine, this: Ptr, settings: Ptr, generator: u32, third: u32) -> Ptr {
    let this_addr = this.addr();
    e.mem.set_u16(this_addr + 6, 1);
    e.mem.set_u32(this_addr, ASSEMBLER_VTABLE);
    e.mem.set_u32(this_addr + 0x40, 0);
    e.mem.set_u32(this_addr + 0x44, 0);
    e.mem.set_u32(this_addr + 0x48, 0x8000_0000);
    let allocator = router_allocator(e);
    let (block, granted) = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 0x1a0);
        let block = e.vcall(allocator, 0xc, &args![size]).u32();
        (block, e.mem.i32(size.addr()))
    });
    let count = granted.wrapping_add((granted >> 31) & 0x1f) >> 5;
    let flags = if count == 0 {
        0x8000_0000
    } else {
        count as u32
    };
    e.mem.set_u32(this_addr + 0x40, block);
    e.mem.set_i32(this_addr + 0x44, count);
    e.mem.set_u32(this_addr + 0x48, flags);
    let float_a = e.mem.u32(PLANE_FIFTH);
    let half = e.mem.u32(PLANE_HALF);
    let float_b = e.mem.u32(PLANE_TWENTIETH);
    let zero = e.mem.u32(SPLIT_LANE_FILL);
    let one = e.mem.u32(PLANE_ONE);
    let negative_one = e.mem.u32(PLANE_NEGATIVE_ONE);
    let quarter = e.mem.u32(PLANE_QUARTER);
    let three_tenths = e.mem.u32(PLANE_THREE_TENTHS);
    let small = e.mem.u32(PLANE_VALUE_32);
    let large = e.mem.u32(PLANE_VALUE_34);
    let three = e.mem.u32(PLANE_THREE);
    e.mem.set_u32(this_addr + 0x50, x87_quiet_bits(half));
    e.mem.set_u32(this_addr + 0x54, x87_quiet_bits(float_a));
    e.mem.set_u32(this_addr + 0x58, 1.0f32.to_bits());
    e.mem.set_u32(this_addr + 0x70, 5);
    store_lanes(e, this_addr + 0x60, [float_a, float_a, float_b, zero]);
    e.mem.set_u8(this_addr + 0x74, 1);
    let lane = |direction: i8| match direction {
        0 => zero,
        1 => one,
        _ => negative_one,
    };
    for (offset, [x, y, z]) in PLANE_DIRECTIONS {
        store_lanes(e, block + offset, [lane(x), lane(y), lane(z), zero]);
    }
    let extras = [
        (0x010u32, 0u32),
        (0x030, 0),
        (0x050, 0),
        (0x070, x87_quiet_bits(float_a)),
        (0x090, x87_quiet_bits(float_a)),
        (0x0b0, x87_quiet_bits(float_a)),
        (0x0d0, x87_quiet_bits(quarter)),
        (0x0f0, x87_quiet_bits(quarter)),
        (0x110, x87_quiet_bits(quarter)),
        (0x130, x87_quiet_bits(three_tenths)),
        (0x150, x87_quiet_bits(small)),
        (0x170, x87_quiet_bits(small)),
        (0x190, x87_quiet_bits(large)),
    ];
    for (offset, bits) in extras {
        e.mem.set_u32(block + offset, bits);
    }
    for index in 0..13u32 {
        let at = block + 0x20 * index;
        let direction = load_lanes(e, at);
        let squares = lanes_op(direction, direction, mul);
        let length = lane_sum(squares);
        let estimate = rsqrt_estimate(f32::from_bits(length)).to_bits();
        let scaled = sse(sse(length, estimate, mul), estimate, mul);
        let correction = sse(three, scaled, sub);
        let factor = sse(sse(half, estimate, mul), correction, mul);
        let factor = if f32::from_bits(length) == 0.0 {
            0
        } else {
            factor
        };
        store_lanes(e, at, lanes_op([factor; 4], direction, mul));
        e.mem.set_u32(at + 0x14, index);
    }
    fn_00e07d60(e, this, settings);
    e.mem.set_u32(this_addr + 8, third);
    e.mem.set_u32(this_addr + 0x10, generator);
    e.mem.set_u32(this_addr + 0xc, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e0_66f0,
            hkp_mopp_assembler_node_info_update_offset_and_spread(Ptr, Ptr)
        ),
        entry!(0x00e0_6730, fn_00e06730(Ptr)),
        entry!(0x00e0_6780, fn_00e06780(Ptr, u8)),
        entry!(
            0x00e0_67b0,
            hkp_mopp_code_generator_add_command8(Ptr, u8, u8)
        ),
        entry!(
            0x00e0_6800,
            hkp_mopp_code_generator_add_command16(Ptr, u8, i32)
        ),
        entry!(
            0x00e0_6880,
            hkp_mopp_code_generator_add_command24(Ptr, u8, i32)
        ),
        entry!(0x00e0_6920, hkp_mopp_code_generator_add_param24(Ptr, i32)),
        entry!(0x00e0_69a0, fn_00e069a0(Ptr, u8, u32)),
        entry!(0x00e0_6a60, hkp_mopp_code_generator_pad(Ptr, i32)),
        entry!(
            0x00e0_6aa0,
            hkp_mopp_default_assembler_release_node(Ptr, Ptr)
        ),
        entry!(0x00e0_6af0, fn_00e06af0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00e0_6b20, fn_00e06b20(Ptr)),
        entry!(0x00e0_6b50, fn_00e06b50(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e0_6bc0, fn_00e06bc0(Ptr, u32, i32)),
        entry!(0x00e0_6c20, fn_00e06c20(Ptr, u32) -> u32),
        entry!(
            0x00e0_6c40,
            hkp_mopp_default_assembler_add_data_offset(Ptr, i32, i32) -> u32
        ),
        entry!(
            0x00e0_6cb0,
            hkp_mopp_default_assembler_add_rescale(Ptr, Ptr)
        ),
        entry!(
            0x00e0_6d70,
            hkp_mopp_default_assembler_check_and_add_properties(Ptr, u32)
        ),
        entry!(0x00e0_6db0, fn_00e06db0(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00e0_6e00,
            hkp_mopp_default_assembler_add_terminals(Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(0x00e0_6ed0, hkp_mopp_default_assembler_add_jump(Ptr, i32)),
        entry!(
            0x00e0_6f20,
            hkp_mopp_default_assembler_get_split_code(Ptr, Ptr) -> u32
        ),
        entry!(0x00e0_7040, fn_00e07040(Ptr, Ptr) -> i32),
        entry!(0x00e0_7070, fn_00e07070(Ptr, Ptr, Ptr)),
        entry!(
            0x00e0_7160,
            hkp_mopp_default_assembler_pre_calc_scale(Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00e0_7400,
            hkp_mopp_default_assembler_calc_term_cut(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_7510,
            hkp_mopp_default_assembler_init_top_down(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_7550,
            hkp_mopp_default_assembler_add_cut(Ptr, Ptr, Ptr)
        ),
        entry!(0x00e0_7680, fn_00e07680(Ptr, i32, i32, i32, i32, i32)),
        entry!(
            0x00e0_7880,
            hkp_mopp_default_assembler_find_planes(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_7b90,
            hkp_mopp_default_assembler_get_scale_info(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_7c50,
            hkp_mopp_default_assembler_assemble_cut_and_terminal_command(Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00e0_7ca0,
            hkp_mopp_default_assembler_assemble_non_terminal_command(Ptr, Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(0x00e0_7d60, fn_00e07d60(Ptr, Ptr)),
        entry!(
            0x00e0_7da0,
            hkp_mopp_default_assembler_pre_calc_cut_positions(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_8040,
            hkp_mopp_default_assembler_assemblesub_node(Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00e0_8240,
            hkp_mopp_default_assembler_calc_terminal_ids_for_chunks(Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(0x00e0_8320, copy_jump_command_info(Ptr, Ptr, i32)),
        entry!(
            0x00e0_8380,
            copy_code_and_embed_terminals(Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00e0_8470,
            hkp_mopp_default_assembler_assemble_jump_chunk_command(Ptr, Ptr) -> i32
        ),
        entry!(
            0x00e0_84f0,
            hkp_mopp_default_assembler_calc_chunk_points(Ptr, Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00e0_8a60,
            hkp_mopp_default_assembler_assemble_sub_node_into_chunks(Ptr, Ptr, Ptr, Ptr, i32, Ptr)
        ),
        entry!(
            0x00e0_8cc0,
            hkp_mopp_default_assembler_assemble(Ptr, Ptr, u32, i32)
        ),
        entry!(0x00e0_90c0, fn_00e090c0(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00e0_9450, hkp_mopp_default_assembler_destruct(Ptr)),
        entry!(
            0x00e0_94a0,
            hkp_mopp_assembler_node_info_update_extents(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e0_9540,
            hkp_mopp_assembler_node_info_calc_max_extents_significant_bits(Ptr)
        ),
        entry!(
            0x00e0_95d0,
            hkp_mopp_assembler_node_info_construct(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00e0_9620, fn_00e09620(Ptr) -> u32),
        entry!(0x00e0_9630, fn_00e09630(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A result with only `eax` set.
    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn new_engine() -> Engine {
        let mut e = Engine::new();
        e.register(GENERATOR_RESIZE, |_, _| Ret::default());
        e.register(VALIDATE_JUMP_COMMANDS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn map_f64(e: &mut Engine, addr: u32, value: f64) {
        e.map(addr, 8);
        e.mem.set_f64(addr, value);
    }

    fn map_f32(e: &mut Engine, addr: u32, value: f32) {
        e.map(addr, 4);
        e.mem.set_f32(addr, value);
    }

    /// The constants the split code, the cut search and the scale code read.
    fn map_split_constants(e: &mut Engine) {
        map_f64(e, SPLIT_THRESHOLD, f64::from(0.1f32));
        map_f64(e, SPLIT_THRESHOLD_NEGATIVE, f64::from(-0.1f32));
        map_f32(e, SPLIT_LANE_FILL, 0.0);
        map_f64(e, FACTOR_HALF, 0.5);
        map_f64(e, FACTOR_PAIR_TWO, f64::from_bits(0x3ff6_a09e_6000_0000));
        map_f64(e, FACTOR_THREE_OUTER, f64::from_bits(0x3fd5_5555_6000_0000));
        map_f64(e, FACTOR_THREE_INNER, f64::from_bits(0x3ffb_b67a_e000_0000));
        map_f64(e, SCALE_FACTOR, f64::from(0.9f32));
        map_f64(e, SCALE_INFO_FLOOR, 0.0);
        map_f64(e, SCALE_INFO_NUMERATOR, 255.0);
        map_f64(e, SCALE_INFO_FACTOR, 65536.0);
    }

    /// Doubles for the float conversions: truncation and floor, and `hkFloor` in `ST0`.
    fn install_float_doubles(e: &mut Engine) {
        e.register(
            FLOAT_TO_INT,
            |_, a| word(f32::from_bits(a[0]) as i32 as u32),
        );
        e.register(FLOOR_TO_INT, |_, a| {
            word(f32::from_bits(a[0]).floor() as i32 as u32)
        });
        e.register(HK_FLOOR, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).floor()),
            ..Ret::default()
        });
    }

    /// A generator with a buffer of `capacity` bytes.
    fn make_generator(e: &mut Engine, capacity: u32) -> u32 {
        let generator = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(capacity);
        e.mem.set_u32(generator + 8, capacity);
        e.mem.set_u32(generator + 0x10, buffer);
        generator
    }

    /// The bytes pushed to a generator, in the order they were pushed.
    fn pushed(e: &Engine, generator: u32) -> Vec<u8> {
        let used = e.mem.u32(generator + 0xc);
        let top = e.mem.u32(generator + 0x10) + e.mem.u32(generator + 8);
        (0..used).map(|index| e.mem.u8(top - 1 - index)).collect()
    }

    /// An assembler-like object with a generator of `capacity` bytes at +0x10.
    fn make_assembler(e: &mut Engine, capacity: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x80);
        let generator = make_generator(e, capacity);
        e.mem.set_u32(this + 0x10, generator);
        (this, generator)
    }

    /// An object whose vtable has `target` in the slot at `slot`.
    fn make_object(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Makes the memory router (`00c85750`) return a block whose allocator is `allocator`.
    fn install_router(e: &mut Engine, allocator: u32) {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| word(router));
    }

    #[test]
    fn update_offset_and_spread_copies_the_widths_and_spreads() {
        let mut e = new_engine();
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(other + 0x24, 3);
        e.mem.set_u32(other + 0x28, 10);
        e.mem.set_u32(other + 0x2c, 2);
        e.mem.set_u32(other + 0x30, 5);
        e.mem.set_u32(other + 0x34, 9);
        e.mem.set_u32(other + 0x38, 20);
        e.call(0x00e0_66f0, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0x38), 7);
        assert_eq!(e.mem.u32(this + 0x3c), 2);
        assert_eq!(e.mem.u32(this + 0x40), 5);
        assert_eq!(e.mem.u32(this + 0x44), 9);
        assert_eq!(e.mem.u32(this + 0x48), 11);
        e.mem.set_u32(other + 0x2c, 0);
        e.mem.set_u32(this + 0x40, 0x77);
        e.call(0x00e0_66f0, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0x77);
    }

    #[test]
    fn extents_are_snapped_down_and_up_to_the_shift() {
        let mut e = new_engine();
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info + 0x24, 3);
        let extents = [
            (0xc, 13i32),
            (0x10, 13),
            (0x14, -5),
            (0x18, 16),
            (0x1c, 7),
            (0x20, 0),
        ];
        for (offset, value) in extents {
            e.mem.set_i32(info + offset, value);
        }
        e.call(0x00e0_6730, &args![info]);
        let snapped: Vec<i32> = [0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c]
            .iter()
            .map(|offset| e.mem.i32(info + offset))
            .collect();
        assert_eq!(snapped, [8, -8, 0, 16, 24, 8]);
        e.mem.set_u32(info + 0x24, 35);
        e.mem.set_i32(info + 0x48, 0);
        e.call(0x00e0_6730, &args![info]);
        assert_eq!(e.mem.i32(info + 0x48), 8);
    }

    #[test]
    fn pushing_a_byte_resizes_when_the_buffer_is_full() {
        let mut e = new_engine();
        let generator = make_generator(&mut e, 4);
        e.call(0x00e0_6780, &args![generator, 0xabu8]);
        assert_eq!(pushed(&e, generator), [0xab]);
        assert!(calls(&e, GENERATOR_RESIZE).is_empty());
        let tight = make_generator(&mut e, 1);
        e.call(0x00e0_6780, &args![tight, 0xcdu8]);
        assert_eq!(calls(&e, GENERATOR_RESIZE), [vec![tight]]);
        assert_eq!(e.mem.u32(tight + 0xc), 1);
    }

    #[test]
    fn commands_write_the_value_bytes_then_the_command() {
        let mut e = new_engine();
        let generator = make_generator(&mut e, 0x40);
        e.call(0x00e0_67b0, &args![generator, 0x11u8, 0x22u8]);
        e.call(0x00e0_6800, &args![generator, 0x33u8, 0x12345i32]);
        e.call(0x00e0_6880, &args![generator, 0x44u8, 0x123456i32]);
        e.call(0x00e0_6920, &args![generator, -2i32]);
        e.call(0x00e0_69a0, &args![generator, 0x55u8, 0x1122_3344u32]);
        assert_eq!(
            pushed(&e, generator),
            [
                0x22, 0x11, 0x45, 0x23, 0x33, 0x56, 0x34, 0x12, 0x44, 0xfe, 0xff, 0xff, 0x44, 0x33,
                0x22, 0x11, 0x55
            ]
        );
    }

    #[test]
    fn pad_appends_cd_bytes_only_for_a_positive_count() {
        let mut e = new_engine();
        let generator = make_generator(&mut e, 0x10);
        e.call(0x00e0_6a60, &args![generator, 3i32]);
        e.call(0x00e0_6a60, &args![generator, 0i32]);
        e.call(0x00e0_6a60, &args![generator, -4i32]);
        assert_eq!(pushed(&e, generator), [0xcd, 0xcd, 0xcd]);
    }

    #[test]
    fn release_node_frees_both_children_of_an_inner_node() {
        let mut e = new_engine();
        let owner = make_object(&mut e, 0xc, 0x0900_0001);
        e.register(0x0900_0001, |_, _| Ret::default());
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0xc, owner);
        e.mem.set_u8(this + 0x74, 1);
        let node = e.mem.alloc(0x80);
        e.mem.set_u32(node + 0x70, 0x111);
        e.mem.set_u32(node + 0x74, 0x222);
        e.call(0x00e0_6aa0, &args![this, node]);
        assert_eq!(
            calls(&e, 0x0900_0001),
            [vec![owner, 0x111], vec![owner, 0x222]]
        );
        assert_eq!(e.mem.u32(node + 0x70), 0);
        assert_eq!(e.mem.u32(node + 0x74), 0);
        e.mem.set_u32(node + 0x70, 0x333);
        e.mem.set_u8(node + 4, 1);
        e.call(0x00e0_6aa0, &args![this, node]);
        e.mem.set_u8(node + 4, 0);
        e.mem.set_u8(this + 0x74, 0);
        e.call(0x00e0_6aa0, &args![this, node]);
        assert_eq!(calls(&e, 0x0900_0001).len(), 2);
        assert_eq!(e.mem.u32(node + 0x70), 0x333);
    }

    #[test]
    fn the_limit_check_allows_small_nodes_and_levels_above_the_limit() {
        let mut e = new_engine();
        let this = e.mem.alloc(0x80);
        let out = e.mem.alloc(8);
        let node = e.mem.alloc(0x20);
        let limit = e.mem.alloc(8);
        e.mem.set_i32(this + 0x18, 5);
        e.mem.set_i32(node + 8, 0x15);
        let result = e.call(0x00e0_6af0, &args![this, out, node, limit]);
        assert_eq!(result.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_i32(node + 8, 0x16);
        e.mem.set_i32(limit, 6);
        e.call(0x00e0_6af0, &args![this, out, node, limit]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_i32(limit, 5);
        e.call(0x00e0_6af0, &args![this, out, node, limit]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn marking_stops_at_the_first_marked_node() {
        let mut e = new_engine();
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x40)).collect();
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(nodes[1], nodes[2]);
        e.mem.set_u8(nodes[1] + 0x39, 1);
        e.call(0x00e0_6b20, &args![nodes[0]]);
        assert_eq!(e.mem.u8(nodes[0] + 0x39), 1);
        assert_eq!(e.mem.u8(nodes[2] + 0x39), 0);
    }

    /// Runs `fn_00e06b50` and returns the word it leaves at `third` +0x34.
    fn carried_offset(base: u32, carried: u32, own: u32, other: u32) -> u32 {
        let mut e = new_engine();
        let this = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let third = e.mem.alloc(0x40);
        e.mem.set_u32(first + 0x24, base);
        e.mem.set_u32(second + 0x34, carried);
        e.mem.set_u32(second + 0x38, other);
        e.mem.set_u32(third + 0x38, own);
        e.call(0x00e0_6b50, &args![this, first, second, third]);
        e.mem.u32(third + 0x34)
    }

    #[test]
    fn the_carried_offset_resets_when_a_size_class_boundary_is_crossed() {
        assert_eq!(carried_offset(0x10, 0, 4, 0), 0);
        assert_eq!(carried_offset(0x30, 0, 2, 0x20), 0);
        assert_eq!(carried_offset(0x40, 5, 0x10, 0x20), 0x40);
        assert_eq!(carried_offset(0x40, 5, 0x10, 0x10), 5);
        assert_eq!(carried_offset(0x200, 5, 0x80, 0x100), 0x200);
        assert_eq!(carried_offset(0x77, 5, 0x8000, 0x1_0000), 0x77);
    }

    #[test]
    fn a_command_is_sized_to_its_value() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        e.call(0x00e0_6bc0, &args![this, 2u32, 0x50i32]);
        e.call(0x00e0_6bc0, &args![this, 2u32, 0x1234i32]);
        e.call(0x00e0_6bc0, &args![this, 2u32, 0x12345i32]);
        e.call(0x00e0_6bc0, &args![this, 2u32, -1i32]);
        assert_eq!(
            pushed(&e, generator),
            [
                0x50, 0x62, 0x34, 0x12, 0x66, 0x45, 0x23, 0x01, 0x00, 0x6a, 0xff, 0xff, 0xff, 0xff,
                0x6a
            ]
        );
    }

    #[test]
    fn the_raw_word_command_returns_five() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        assert_eq!(e.call(0x00e0_6c20, &args![this, 0x0102_0304u32]).u32(), 5);
        assert_eq!(pushed(&e, generator), [4, 3, 2, 1, 0x70]);
    }

    #[test]
    fn the_data_offset_command_writes_two_values_and_returns_five() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let result = e.call(0x00e0_6c40, &args![this, 0x0102i32, 0x0304i32]);
        assert_eq!(result.u32(), 5);
        assert_eq!(pushed(&e, generator), [0x04, 0x03, 0x02, 0x01, 0x0d]);
    }

    #[test]
    fn rescale_writes_the_four_bytes_in_reverse() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let info = e.mem.alloc(0x20);
        for (offset, value) in [(4, 0xa4u8), (8, 0xa8), (0xc, 0xac), (0x10, 0xb0)] {
            e.mem.set_u8(info + offset, value);
        }
        e.call(0x00e0_6cb0, &args![this, info]);
        assert_eq!(pushed(&e, generator), [0xb0, 0xac, 0xa8, 0xa4]);
    }

    #[test]
    fn properties_are_written_in_the_smallest_command() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        e.call(0x00e0_6d70, &args![this, 0x50u32]);
        e.call(0x00e0_6d70, &args![this, 0x1234u32]);
        e.call(0x00e0_6d70, &args![this, 0x1234_5678u32]);
        assert_eq!(
            pushed(&e, generator),
            [0x50, 9, 0x34, 0x12, 0xa, 0x78, 0x56, 0x34, 0x12, 0xb]
        );
    }

    #[test]
    fn fn_00e06db0_writes_a_command_only_for_the_right_flags() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x60);
        let third = e.mem.alloc(0x60);
        e.mem.set_u32(third + 0x40, 7);
        e.call(0x00e0_6db0, &args![this, first, second, third]);
        assert_eq!(pushed(&e, generator), [7, 0x60]);
        e.mem.set_u32(third + 0x44, 1);
        e.call(0x00e0_6db0, &args![this, first, second, third]);
        e.mem.set_u32(third + 0x44, 0);
        e.mem.set_u32(third + 0x40, 0);
        e.call(0x00e0_6db0, &args![this, first, second, third]);
        assert_eq!(pushed(&e, generator).len(), 2);
        e.mem.set_u32(first, 1);
        e.call(0x00e0_6db0, &args![this, first, second, third]);
        assert_eq!(pushed(&e, generator).len(), 2);
        e.mem.set_u32(second + 0x44, 1);
        e.mem.set_u32(third + 0x40, 9);
        e.call(0x00e0_6db0, &args![this, first, second, third]);
        assert_eq!(pushed(&e, generator)[2..], [9, 0x60]);
    }

    /// Runs `addTerminals` with the distance `distance` and no commands.
    fn terminal_bytes(distance: u32) -> Vec<u8> {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let node = e.mem.alloc(0x80);
        let terminals = e.mem.alloc(0x20);
        e.mem.set_u32(terminals, distance.wrapping_add(7));
        e.mem.set_u32(node + 0x60, terminals);
        let second = e.mem.alloc(0x60);
        let third = e.mem.alloc(0x60);
        e.mem.set_u32(third + 0x34, 7);
        let size = e.call(0x00e0_6e00, &args![this, node, second, third]).i32();
        assert_eq!(size as usize, pushed(&e, generator).len());
        pushed(&e, generator)
    }

    #[test]
    fn terminal_distances_are_sized_by_range() {
        assert_eq!(terminal_bytes(5), [0x35]);
        assert_eq!(terminal_bytes(0x40), [0x40, 0x50]);
        assert_eq!(terminal_bytes(0x1234), [0x34, 0x12, 0x51]);
        assert_eq!(terminal_bytes(0x12_3456), [0x56, 0x34, 0x12, 0x52]);
        assert_eq!(terminal_bytes(0x123_4567), [0x67, 0x45, 0x23, 0x01, 0x53]);
    }

    #[test]
    fn terminals_with_both_words_set_get_their_own_command() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let node = e.mem.alloc(0x80);
        let terminals = e.mem.alloc(0x20);
        e.mem.set_u32(terminals, 5);
        e.mem.set_u32(node + 0x60, terminals);
        e.mem.set_u32(node + 0x2c, 2);
        e.mem.set_u32(node + 0x30, 3);
        e.mem.set_u32(node + 0x34, 4);
        let second = e.mem.alloc(0x60);
        e.mem.set_u32(second + 0x44, 1);
        let third = e.mem.alloc(0x60);
        let size = e.call(0x00e0_6e00, &args![this, node, second, third]).i32();
        assert_eq!(size, 3);
        assert_eq!(pushed(&e, generator), [0x35, 3, 0x60]);
    }

    /// The bytes `addJump` writes for `target` into an empty generator.
    fn jump_bytes(target: i32) -> Vec<u8> {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        e.call(0x00e0_6ed0, &args![this, target]);
        pushed(&e, generator)
    }

    #[test]
    fn a_jump_is_sized_to_its_distance() {
        assert_eq!(jump_bytes(-6), [6, 5]);
        assert!(jump_bytes(0).is_empty());
        assert!(jump_bytes(5).is_empty());
        assert_eq!(jump_bytes(-0xff), [0xff, 0, 6]);
        assert_eq!(jump_bytes(-0x200), [0, 2, 6]);
        assert_eq!(jump_bytes(-0x1_0000), [0, 0, 1, 7]);
        assert_eq!(jump_bytes(-0x100_0000), [0, 0, 0, 1, 8]);
    }

    /// The split code of the vector `(x, y, z)`.
    fn split_code(x: f32, y: f32, z: f32) -> u32 {
        let mut e = new_engine();
        map_split_constants(&mut e);
        let vector = e.mem.alloc(0x10);
        e.mem.set_f32(vector, x);
        e.mem.set_f32(vector + 4, y);
        e.mem.set_f32(vector + 8, z);
        let this = e.mem.alloc(0x10);
        e.call(0x00e0_6f20, &args![this, vector]).u32()
    }

    #[test]
    fn the_split_code_follows_the_sign_classes_of_the_vector() {
        assert_eq!(split_code(1.0, 1.0, 1.0), 9);
        assert_eq!(split_code(1.0, -1.0, 1.0), 0xb);
        assert_eq!(split_code(1.0, 0.0, 1.0), 5);
        assert_eq!(split_code(1.0, 1.0, -1.0), 0xa);
        assert_eq!(split_code(1.0, -1.0, -1.0), 0xc);
        assert_eq!(split_code(1.0, 0.0, -1.0), 6);
        assert_eq!(split_code(1.0, 1.0, 0.0), 7);
        assert_eq!(split_code(1.0, -1.0, 0.0), 8);
        assert_eq!(split_code(1.0, 0.0, 0.0), 0);
        assert_eq!(split_code(0.0, 1.0, 1.0), 3);
        assert_eq!(split_code(0.0, 1.0, -1.0), 4);
        assert_eq!(split_code(0.0, 1.0, 0.0), 1);
        assert_eq!(split_code(0.0, 0.0, 0.0), 2);
        assert_eq!(split_code(0.05, 0.05, 0.05), 2);
        assert_eq!(split_code(f32::NAN, 1.0, 1.0), 3);
        assert_eq!(split_code(-1.0, 1.0, 0.0), 1);
    }

    #[test]
    fn the_shift_available_is_the_shift_minus_the_excess_of_the_size() {
        let mut e = new_engine();
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        e.mem.set_i32(first + 0x24, 10);
        e.mem.set_i32(second + 8, 12);
        assert_eq!(e.call(0x00e0_7040, &args![first, second]).i32(), 6);
        e.mem.set_i32(second + 8, 5);
        assert_eq!(e.call(0x00e0_7040, &args![first, second]).i32(), 10);
        e.mem.set_i32(first + 0x24, i32::MIN);
        e.mem.set_i32(second + 8, 9);
        assert_eq!(e.call(0x00e0_7040, &args![first, second]).i32(), i32::MIN);
    }

    /// Runs `fn_00e07070` for three lanes whose second extent is `span` above the first one
    /// in lane 0 and 500 above in the others; returns `(first, second, out)` blocks.
    fn requantise(span: i32) -> (Engine, u32, u32, u32) {
        let mut e = new_engine();
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        e.mem.set_i32(first + 0x24, 10);
        for lane in 0..3u32 {
            let base = 100 * (lane as i32 + 1);
            e.mem.set_i32(first + 0x28 + 4 * lane, base);
            e.mem.set_i32(second + 0xc + 8 * lane, base + 2048);
            let width = if lane == 0 { span } else { 500 };
            e.mem.set_i32(second + 0x10 + 8 * lane, base + 2048 + width);
        }
        e.mem.set_i32(second + 8, 12);
        e.call(0x00e0_7070, &args![first, second, out]);
        (e, first, second, out)
    }

    #[test]
    fn requantising_lowers_the_shift_by_four_when_every_span_fits() {
        let (e, _, second, out) = requantise(500);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.i32(out + 4), 4);
        assert_eq!(e.mem.i32(second + 0x24), 6);
        for lane in 0..3u32 {
            assert_eq!(e.mem.i32(out + 8 + 4 * lane), 2);
            assert_eq!(
                e.mem.i32(second + 0x28 + 4 * lane),
                100 * (lane as i32 + 1) + 2048
            );
        }
    }

    #[test]
    fn requantising_backs_off_when_a_span_is_too_wide() {
        let (e, _, second, out) = requantise(16320);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.i32(out + 4), 3);
        assert_eq!(e.mem.i32(second + 0x24), 7);
    }

    #[test]
    fn requantising_copies_the_words_when_there_is_no_room() {
        let mut e = new_engine();
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        e.mem.set_u8(out, 7);
        e.mem.set_i32(first + 0x24, 3);
        for (offset, value) in [(0x28, 11), (0x2c, 12), (0x30, 13)] {
            e.mem.set_i32(first + offset, value);
        }
        e.mem.set_i32(second + 8, 100);
        e.call(0x00e0_7070, &args![first, second, out]);
        assert_eq!(e.mem.u8(out), 0);
        let copied: Vec<i32> = [0x24, 0x28, 0x2c, 0x30]
            .iter()
            .map(|offset| e.mem.i32(second + offset))
            .collect();
        assert_eq!(copied, [3, 11, 12, 13]);
    }

    /// A pre-calc scale leaf: a node whose lane extents are `high` above zero, an assembler
    /// with best 10.0, scale 2.0 and the extents `4.0, 1.0, 0.5`, and the info with shift 10.
    fn scale_leaf() -> (Engine, u32, u32, u32, u32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        let this = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let parent = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_f32(this + 0x58, 10.0);
        e.mem.set_f32(this + 0x3c, 2.0);
        for (lane, (extent, high)) in [(4.0f32, 1.0f32), (1.0, 2.0), (0.5, 0.25)]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(this + 0x60 + 4 * lane as u32, extent);
            e.mem.set_f32(node + 0x10 + 8 * lane as u32, high);
        }
        e.mem.set_u8(node + 4, 1);
        e.mem.set_i32(info + 8, 10);
        (e, this, node, parent, info)
    }

    #[test]
    fn a_leaf_scale_marks_the_lanes_and_returns_at_least_the_shift() {
        let (mut e, this, node, parent, info) = scale_leaf();
        let result = e.call(0x00e0_7160, &args![this, node, parent, info]).i32();
        assert_eq!(result, 10);
        assert_eq!(e.mem.bytes(node + 0x3b, 3), [1, 1, 2]);
        let scaled = ((2.0f64 * f64::from(0.9f32)) * 4.0) as f32;
        assert_eq!(
            calls(&e, FLOAT_TO_INT),
            [
                vec![8.0f32.to_bits()],
                vec![1.0f32.to_bits()],
                vec![scaled.to_bits()]
            ]
        );
    }

    #[test]
    fn a_leaf_scale_follows_the_bits_of_the_best_extent() {
        let (mut e, this, node, parent, info) = scale_leaf();
        e.mem.set_i32(info + 8, 2);
        let result = e.call(0x00e0_7160, &args![this, node, parent, info]).i32();
        assert_eq!(result, 6);
        e.mem.set_u8(node + 0x39, 1);
        e.mem.set_u8(node + 0x3b, 0);
        e.call(0x00e0_7160, &args![this, node, parent, info]);
        assert_eq!(e.mem.u8(node + 0x3b), 0);
    }

    /// An inner node with one leaf child at +0x70 and shift `shift` in the info.
    fn scale_inner(shift: i32) -> (Engine, u32, u32, u32, i32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        let this = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let child = e.mem.alloc(0x80);
        let parent = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_i32(this + 0x1c, 5);
        e.mem.set_f32(this + 0x3c, 1.0);
        e.mem.set_u8(child + 4, 1);
        e.mem.set_u32(node + 0x70, child);
        e.mem.set_i32(parent + 0x24, 10);
        e.mem.set_i32(info + 8, shift);
        let result = e.call(0x00e0_7160, &args![this, node, parent, info]).i32();
        (e, node, child, info, result)
    }

    #[test]
    fn an_inner_scale_limits_itself_to_the_shift_plus_two() {
        let (e, node, child, _, result) = scale_inner(12);
        assert_eq!(result, 14);
        assert_eq!(e.mem.u8(node + 0x3a), 1);
        assert_eq!(e.mem.u8(child + 0x3a), 0);
    }

    #[test]
    fn an_inner_scale_marks_a_child_that_needs_more_bits_than_the_shift() {
        let (e, node, child, _, result) = scale_inner(30);
        assert_eq!(result, 26);
        assert_eq!(e.mem.u8(node + 0x3a), 0);
        assert_eq!(e.mem.u8(child + 0x3a), 1);
    }

    /// The assembler, node and info of a term cut: widths 0.5, 5.0, 0.5; extents 3.0, 1.0,
    /// 20.0; shift 4, scale 2.0, floor 1.0; lows 100, 200, 300 and highs 150, 250, 350.
    fn term_cut_setup(e: &mut Engine) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x3c, 2.0);
        e.mem.set_f32(this + 0x58, 1.0);
        for (lane, (extent, width)) in [(3.0f32, 0.5f32), (1.0, 5.0), (20.0, 0.5)]
            .into_iter()
            .enumerate()
        {
            let lane = lane as u32;
            e.mem.set_f32(this + 0x60 + 4 * lane, extent);
            e.mem.set_f32(node + 0x10 + 8 * lane, width);
            e.mem
                .set_i32(info + 0xc + 8 * lane, 100 * (lane as i32 + 1));
            e.mem
                .set_i32(info + 0x10 + 8 * lane, 100 * (lane as i32 + 1) + 50);
        }
        e.mem.set_u32(info + 0x24, 4);
        (this, node, info)
    }

    #[test]
    fn a_term_cut_widens_the_extents_by_the_scaled_width() {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        let (this, node, info) = term_cut_setup(&mut e);
        e.call(0x00e0_7400, &args![this, node, info]);
        for (lane, (low, high)) in [(94, 157), (184, 267), (284, 367)].into_iter().enumerate() {
            let lane = lane as u32;
            assert_eq!(e.mem.i32(node + 0x44 + 4 * lane), low);
            assert_eq!(e.mem.i32(node + 0x50 + 4 * lane), high);
        }
    }

    #[test]
    fn init_top_down_requantises_only_a_marked_node() {
        let mut e = new_engine();
        let this = e.mem.alloc(0x80);
        let first = e.mem.alloc(0x80);
        let second = e.mem.alloc(0x80);
        let third = e.mem.alloc(0x80);
        let fourth = e.mem.alloc(0x20);
        e.mem.set_u8(fourth, 0x55);
        e.mem.set_u32(second + 0x34, 0x99);
        e.call(0x00e0_7510, &args![this, first, second, third, fourth]);
        assert_eq!(e.mem.u8(fourth), 0x55);
        assert_eq!(e.mem.u32(third + 0x34), 0x99);
        e.mem.set_u8(first + 0x3a, 1);
        e.mem.set_i32(second + 0x24, 10);
        e.mem.set_i32(third + 8, 12);
        e.call(0x00e0_7510, &args![this, first, second, third, fourth]);
        assert_eq!(e.mem.u8(fourth), 1);
        assert_eq!(e.mem.i32(fourth + 4), 4);
    }

    #[test]
    fn extents_are_scaled_to_integers_around_the_origin() {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        let this = e.mem.alloc(0x40);
        let node = e.mem.alloc(0x40);
        let origin = e.mem.alloc(0x10);
        for (lane, (base, low, high)) in
            [(1.0f32, 2.0f32, 4.0f32), (2.0, 3.5, 6.0), (3.0, 3.0, 3.25)]
                .into_iter()
                .enumerate()
        {
            let lane = lane as u32;
            e.mem.set_f32(origin + 4 * lane, base);
            e.mem.set_f32(node + 0xc + 8 * lane, low);
            e.mem.set_f32(node + 0x10 + 8 * lane, high);
        }
        e.mem.set_f32(origin + 0xc, 2.0);
        e.call(0x00e0_94a0, &args![this, node, origin]);
        let stored: Vec<i32> = [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20]
            .iter()
            .map(|offset| e.mem.i32(this + offset))
            .collect();
        assert_eq!(stored, [2, 7, 3, 9, 0, 1]);
        assert_eq!(calls(&e, FLOOR_TO_INT)[0], [2.0f32.to_bits()]);
    }

    #[test]
    fn the_significant_bits_come_from_the_widest_extent() {
        let mut e = new_engine();
        let this = e.mem.alloc(0x40);
        for (offset, value) in [(0x10, 1), (0x18, 2), (0x20, 100)] {
            e.mem.set_i32(this + offset, value);
        }
        e.call(0x00e0_9540, &args![this]);
        assert_eq!(e.mem.i32(this + 8), 7);
        e.mem.set_i32(this + 0x20, 16);
        e.call(0x00e0_9540, &args![this]);
        assert_eq!(e.mem.i32(this + 8), 5);
        e.mem.set_i32(this + 0x20, 0x100_0000);
        e.call(0x00e0_9540, &args![this]);
        assert_eq!(e.mem.i32(this + 8), 24);
    }

    #[test]
    fn the_node_info_copies_the_source_and_derives_the_rest() {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        let this = e.mem.alloc(0x60);
        let source = e.mem.alloc(0x60);
        let node = e.mem.alloc(0x80);
        let origin = e.mem.alloc(0x10);
        for word in 0..0x12u32 {
            e.mem.set_u32(source + 4 * word, 0x1000 + word);
        }
        e.mem.set_u32(source, 4);
        e.mem.set_u32(node + 0x24, 2);
        e.mem.set_u32(node + 0x28, 5);
        for lane in 0..3u32 {
            e.mem.set_f32(node + 0x10 + 8 * lane, 100.0);
        }
        e.mem.set_f32(origin + 0xc, 1.0);
        let result = e.call(0x00e0_95d0, &args![this, source, node, origin]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 5);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0x1000 + 9);
        assert_eq!(e.mem.u32(this + 0x34), 0x1000 + 13);
        assert_eq!(e.mem.u32(this + 0x38), 3);
        assert_eq!(e.mem.i32(this + 0x10), 101);
        assert_eq!(e.mem.i32(this + 0xc), 0);
        assert_eq!(e.mem.i32(this + 8), 7);
    }

    #[test]
    fn the_destructor_frees_the_plane_array_unless_it_is_not_owned() {
        let mut e = new_engine();
        let allocator = make_object(&mut e, 0x10, 0x0900_0002);
        e.register(0x0900_0002, |_, _| Ret::default());
        install_router(&mut e, allocator);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x40, 0x1234);
        e.mem.set_u32(this + 0x44, 7);
        e.mem.set_u32(this + 0x48, 5);
        e.call(0x00e0_9450, &args![this]);
        assert_eq!(calls(&e, 0x0900_0002), [vec![allocator, 0x1234, 5 << 5]]);
        assert_eq!(e.mem.u32(this), BASE_ASSEMBLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_0000);
        e.mem.set_u32(this + 0x40, 0x1234);
        e.mem.set_u32(this + 0x48, 0x8000_0005);
        e.call(0x00e0_9450, &args![this]);
        assert_eq!(calls(&e, 0x0900_0002).len(), 1);
    }

    #[test]
    fn the_vtable_constants_are_returned() {
        let mut e = new_engine();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x40, 0x4321);
        assert_eq!(e.call(0x00e0_9620, &args![this]).u32(), 0x0d);
        assert_eq!(e.call(0x00e0_9630, &args![this]).u32(), 0x4321);
    }

    /// An allocator whose slot +0x10 frees the block it is given and whose slot +4 hands out
    /// zeroed blocks, behind a memory router; the array grow helper gives a sixteen-element
    /// array and the insert helper splices words in. Returns `(allocator, router)`.
    fn install_arrays(e: &mut Engine) -> (u32, u32) {
        let allocator = make_object(e, 0x10, 0x0900_0010);
        let vtable = e.mem.u32(allocator);
        e.mem.set_u32(vtable + 4, 0x0900_0011);
        e.register(0x0900_0010, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(0x0900_0011, |e, a| word(e.mem.alloc(a[1])));
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| word(router));
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let (array, element) = (a[1], a[2]);
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let data = e.mem.alloc(16 * element);
            for index in 0..size * element / 4 {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(data + 4 * index, value);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 16);
            Ret::default()
        });
        e.register(ARRAY_INSERT_AT, |e, a| {
            let (array, index, source, count) = (a[0], a[2], a[3], a[4]);
            if e.mem.u32(array) == 0 {
                let data = e.mem.alloc(0x100);
                e.mem.set_u32(array, data);
            }
            let data = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            for position in (index..size).rev() {
                let value = e.mem.u32(data + 4 * position);
                e.mem.set_u32(data + 4 * (position + count), value);
            }
            for position in 0..count {
                let value = e.mem.u32(source + 4 * position);
                e.mem.set_u32(data + 4 * (index + position), value);
            }
            e.mem.set_u32(array + 4, size + count);
            Ret::default()
        });
        e.register(MEM_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            word(a[0])
        });
        (allocator, router)
    }

    /// An `hkArray` header (data, size, capacity) with room for `capacity` words of
    /// `element` bytes.
    fn make_array(e: &mut Engine, capacity: u32, element: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity * element);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    /// The words of an array of eight-byte elements (two words each).
    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4) * 2)
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn a_cut_writes_the_flagged_lanes() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let node = e.mem.alloc(0x80);
        let info = e.mem.alloc(0x80);
        e.mem.set_u8(node + 0x3b, 1);
        e.mem.set_u8(node + 0x3c, 2);
        e.mem.set_u32(info + 0x24, 2);
        e.mem.set_i32(info + 0x28, 10);
        e.mem.set_i32(info + 0x10, 50);
        e.mem.set_i32(info + 0xc, 26);
        e.mem.set_i32(info + 0x18, 0x12_3456);
        e.mem.set_i32(info + 0x14, 0x01_0203);
        e.call(0x00e0_7550, &args![this, node, info]);
        assert_eq!(
            pushed(&e, generator),
            [11, 4, 0x26, 0x57, 0x34, 0x12, 3, 2, 1, 0x2a]
        );
    }

    /// Runs `fn_00e07680` with `used` bytes in the generator already and returns the new bytes.
    fn split_command(used: u32, values: [i32; 5]) -> Vec<u8> {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x100);
        e.mem.set_u32(generator + 0xc, used);
        let [first, second, third, fourth, fifth] = values;
        e.call(
            0x00e0_7680,
            &args![this, first, second, third, fourth, fifth],
        );
        pushed(&e, generator)[used as usize..].to_vec()
    }

    #[test]
    fn a_split_command_uses_the_short_form_when_both_distances_fit() {
        assert_eq!(
            split_command(0x20, [4, 7, 2, 0x08, 0x10]),
            [0x10, 0, 0x18, 0, 4, 7, 0x25]
        );
    }

    #[test]
    fn a_split_command_adds_jumps_when_the_distances_do_not_fit() {
        assert_eq!(
            split_command(0x20, [4, 9, 3, 0x10, -0x100]),
            [0x20, 1, 6, 0x13, 5, 2, 4, 9, 0x13]
        );
    }

    #[test]
    fn a_split_command_with_adjacent_planes_uses_one_byte_for_the_code() {
        assert_eq!(split_command(0x20, [4, 5, 1, 0x30, 0x18]), [8, 4, 0x21]);
    }

    /// An assembler for the cut search: scale 2.0, a zero split vector and the planes `4.0`
    /// and `6.0`. Returns `(engine, this, node, info, outputs)`; the outputs are the two
    /// words the call writes (the high index first).
    fn plane_setup(extents: [f32; 3]) -> (Engine, u32, u32, u32, u32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        let this = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let info = e.mem.alloc(0x60);
        let vector = e.mem.alloc(0x10);
        for (lane, value) in extents.into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * lane as u32, value);
        }
        e.mem.set_u32(node + 0x60, vector);
        e.mem.set_f32(this + 0x3c, 2.0);
        let outputs = e.mem.alloc(8);
        (e, this, node, info, outputs)
    }

    fn find_planes(e: &mut Engine, this: u32, node: u32, info: u32, outputs: u32) -> (i32, i32) {
        e.call(0x00e0_7880, &args![this, node, info, outputs, outputs + 4]);
        (e.mem.i32(outputs + 4), e.mem.i32(outputs))
    }

    #[test]
    fn one_nonzero_extent_scales_the_planes_along_its_lane() {
        let (mut e, this, node, info, outputs) = plane_setup([0.0, 0.0, 2.0]);
        e.mem.set_f32(this + 0x38, 1.0);
        e.mem.set_f32(node + 0x64, 5.0);
        e.mem.set_f32(node + 0x68, 9.0);
        e.mem.set_u32(info + 0x24, 2);
        assert_eq!(find_planes(&mut e, this, node, info, outputs), (2, 5));
        e.mem.set_f32(node + 0x64, -100.0);
        e.mem.set_f32(node + 0x68, 1000.0);
        assert_eq!(find_planes(&mut e, this, node, info, outputs), (0, 255));
    }

    #[test]
    fn without_extents_the_planes_are_the_scaled_bounds() {
        let (mut e, this, node, info, outputs) = plane_setup([0.0, 0.0, 0.0]);
        e.mem.set_f32(node + 0x64, 4.0);
        e.mem.set_f32(node + 0x68, 6.0);
        e.mem.set_u32(info + 0x24, 1);
        assert_eq!(find_planes(&mut e, this, node, info, outputs), (4, 7));
    }

    #[test]
    fn two_extents_scale_the_planes_and_offset_negative_ones() {
        let (mut e, this, node, info, outputs) = plane_setup([2.0, -2.0, 0.0]);
        e.mem.set_f32(node + 0x64, 4.0);
        e.mem.set_f32(node + 0x68, 6.0);
        e.mem.set_u32(info + 0x24, 1);
        assert_eq!(find_planes(&mut e, this, node, info, outputs), (130, 132));
    }

    #[test]
    fn three_extents_use_the_third_factor_pair() {
        let (mut e, this, node, info, outputs) = plane_setup([1.0, 1.0, -1.0]);
        e.mem.set_f32(node + 0x64, 4.0);
        e.mem.set_f32(node + 0x68, 6.0);
        e.mem.set_u32(info + 0x24, 1);
        assert_eq!(find_planes(&mut e, this, node, info, outputs), (87, 89));
    }

    #[test]
    fn the_scale_info_is_the_node_origin_and_the_scale_of_the_widest_extent() {
        let mut e = new_engine();
        map_split_constants(&mut e);
        let this = e.mem.alloc(0x10);
        let node = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x10);
        let extents = [
            (0xc, 1.0f32),
            (0x10, 5.0),
            (0x14, 2.0),
            (0x18, 4.0),
            (0x1c, 3.0),
            (0x20, 3.5),
        ];
        for (offset, value) in extents {
            e.mem.set_f32(node + offset, value);
        }
        e.call(0x00e0_7b90, &args![this, node, out]);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        assert_eq!(e.mem.f32(out + 0xc), 4_177_920.0);
        for offset in [0x10, 0x18, 0x20] {
            e.mem.set_f32(node + offset, -1.0);
        }
        e.call(0x00e0_7b90, &args![this, node, out]);
        assert_eq!(e.mem.u32(out + 0xc), 0x7f80_0000);
    }

    #[test]
    fn a_cut_and_terminal_command_records_where_it_ended() {
        let mut e = new_engine();
        let (this, generator) = make_assembler(&mut e, 0x40);
        let node = e.mem.alloc(0x80);
        let terminals = e.mem.alloc(0x20);
        e.mem.set_u32(terminals, 5);
        e.mem.set_u32(node + 0x60, terminals);
        let second = e.mem.alloc(0x60);
        let third = e.mem.alloc(0x60);
        let size = e.call(0x00e0_7c50, &args![this, node, second, third]).i32();
        assert_eq!(size, 1);
        assert_eq!(pushed(&e, generator), [0x35]);
        assert_eq!(e.mem.u32(node + 0x5c), 1);
        assert_eq!(e.mem.u8(node + 0x38), 1);
        assert_eq!(e.mem.u8(node + 0x39), 1);
        assert_eq!(e.mem.u32(this + 0x28), 1);
    }

    /// An assembler with 0x40 bytes in its generator, a split node with the children's end
    /// positions 0x20 (+0x70) and 0x30 (+0x74); returns `(engine, this, generator, node,
    /// second, info)`.
    fn non_terminal_setup() -> (Engine, u32, u32, u32, u32, u32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        let (this, generator) = make_assembler(&mut e, 0x100);
        e.mem.set_u32(generator + 0xc, 0x40);
        e.mem.set_f32(this + 0x3c, 2.0);
        let node = e.mem.alloc(0x80);
        let vector = e.mem.alloc(0x10);
        e.mem.set_u32(node + 0x60, vector);
        e.mem.set_f32(node + 0x64, 4.0);
        e.mem.set_f32(node + 0x68, 6.0);
        let earlier = e.mem.alloc(0x80);
        let later = e.mem.alloc(0x80);
        e.mem.set_u32(earlier + 0x5c, 0x20);
        e.mem.set_u32(later + 0x5c, 0x30);
        e.mem.set_u32(node + 0x70, earlier);
        e.mem.set_u32(node + 0x74, later);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info + 0x24, 1);
        (e, this, generator, node, second, info)
    }

    #[test]
    fn a_non_terminal_command_writes_the_split_and_records_the_end() {
        let (mut e, this, generator, node, second, info) = non_terminal_setup();
        let rescale = e.mem.alloc(0x20);
        let size = e
            .call(0x00e0_7ca0, &args![this, node, second, info, rescale])
            .i32();
        assert_eq!(size, 7);
        assert_eq!(
            pushed(&e, generator)[0x40..],
            [0x10, 0, 0x20, 0, 4, 7, 0x25]
        );
        assert_eq!(e.mem.u32(node + 0x5c), 0x47);
        assert_eq!(e.mem.u8(node + 0x38), 1);
        assert_eq!(e.mem.u8(node + 0x39), 1);
    }

    #[test]
    fn a_non_terminal_command_adds_the_property_change_and_the_rescale() {
        let (mut e, this, generator, node, second, info) = non_terminal_setup();
        let rescale = e.mem.alloc(0x20);
        e.mem.set_u8(rescale, 1);
        for (offset, value) in [(4, 0xa4u8), (8, 0xa8), (0xc, 0xac), (0x10, 0xb0)] {
            e.mem.set_u8(rescale + offset, value);
        }
        e.mem.set_u32(info + 0x34, 0x1234);
        let size = e
            .call(0x00e0_7ca0, &args![this, node, second, info, rescale])
            .i32();
        assert_eq!(size, 14);
        assert_eq!(
            pushed(&e, generator)[0x47..],
            [0x34, 0x12, 0x0a, 0xb0, 0xac, 0xa8, 0xa4]
        );
    }

    #[test]
    fn the_settings_are_copied_into_the_assembler() {
        let mut e = new_engine();
        let this = e.mem.alloc(0x80);
        let settings = e.mem.alloc(0x30);
        e.mem.set_u32(settings, 0x7f80_0001);
        e.mem.set_u32(settings + 4, 0x3f80_0000);
        e.mem.set_u32(settings + 8, 0xff80_1234);
        for lane in 0..4u32 {
            e.mem.set_u32(settings + 0x10 + 4 * lane, 0xa1 + lane);
        }
        e.mem.set_u32(settings + 0x20, 0x55);
        e.mem.set_u8(settings + 0x24, 3);
        e.call(0x00e0_7d60, &args![this, settings]);
        assert_eq!(e.mem.u32(this + 0x50), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x54), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x58), 0xffc0_1234);
        for lane in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x60 + 4 * lane), 0xa1 + lane);
        }
        assert_eq!(e.mem.u32(this + 0x70), 0x55);
        assert_eq!(e.mem.u8(this + 0x74), 3);
    }

    #[test]
    fn cut_positions_leave_a_written_node_alone() {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        let this = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let parent = e.mem.alloc(0x80);
        let info = e.mem.alloc(0x80);
        e.mem.set_u8(node + 0x38, 1);
        e.mem.set_u32(info + 0x48, 0x77);
        e.call(0x00e0_7da0, &args![this, node, parent, info]);
        assert_eq!(e.mem.u32(info + 0x48), 0x77);
        assert!(calls(&e, FLOAT_TO_INT).is_empty());
    }

    #[test]
    fn cut_positions_of_a_leaf_snap_the_extents_and_widen_the_term_cut() {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        let (this, node, info) = term_cut_setup(&mut e);
        let parent = e.mem.alloc(0x80);
        e.mem.set_u8(node + 4, 1);
        e.call(0x00e0_7da0, &args![this, node, parent, info]);
        assert_eq!(e.mem.i32(info + 0x48), 96);
        assert_eq!(e.mem.i32(info + 0x54), 160);
        assert_eq!(e.mem.i32(node + 0x44), 94);
        assert_eq!(e.mem.i32(node + 0x50), 157);
    }

    #[test]
    fn cut_positions_of_an_inner_node_merge_the_cuts_of_its_child() {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        let this = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let child = e.mem.alloc(0x80);
        let parent = e.mem.alloc(0x80);
        let info = e.mem.alloc(0x80);
        let vector = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0x3c, 2.0);
        e.mem.set_u32(node + 0x60, vector);
        e.mem.set_u32(node + 0x74, child);
        e.mem.set_u8(child + 4, 1);
        e.mem.set_u8(child + 0x3b, 1);
        e.mem.set_u8(child + 0x3c, 2);
        e.mem.set_u32(info + 0x24, 4);
        for lane in 0..3u32 {
            e.mem.set_i32(info + 0x10 + 8 * lane, 10);
        }
        e.call(0x00e0_7da0, &args![this, node, parent, info]);
        assert_eq!(e.mem.bytes(node + 0x3b, 3), [1, 2, 0]);
        assert_eq!(e.mem.bytes(child + 0x3b, 3), [0, 0, 0]);
        for lane in 0..3u32 {
            assert_eq!(e.mem.i32(node + 0x44 + 4 * lane), 0);
            assert_eq!(e.mem.i32(node + 0x50 + 4 * lane), 18);
        }
        assert_eq!(e.mem.i32(child + 0x4c), 0);
        assert_eq!(calls(&e, FLOAT_TO_INT).len(), 6);
    }

    /// An assembler for `assemblesubNode`: level limit 5, divisor 7, command limit 100, scale 2.0.
    fn sub_node_setup() -> (Engine, u32, u32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        let (this, generator) = make_assembler(&mut e, 0x100);
        e.mem.set_i32(this + 0x18, 5);
        e.mem.set_i32(this + 0x24, 100);
        e.mem.set_i32(this + 0x70, 7);
        e.mem.set_f32(this + 0x3c, 2.0);
        (e, this, generator)
    }

    /// A leaf whose terminal list starts with `distance` and has `distance + 0x1000` at +0x10.
    fn make_leaf(e: &mut Engine, distance: u32) -> u32 {
        let node = e.mem.alloc(0x80);
        let terminals = e.mem.alloc(0x20);
        e.mem.set_u32(terminals, distance);
        e.mem.set_u32(terminals + 0x10, distance + 0x1000);
        e.mem.set_u32(node + 0x60, terminals);
        e.mem.set_u8(node + 4, 1);
        node
    }

    #[test]
    fn a_written_node_costs_three_bytes_without_assembling() {
        let (mut e, this, generator) = sub_node_setup();
        let node = make_leaf(&mut e, 5);
        e.mem.set_u8(node + 0x38, 1);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let size = e.call(0x00e0_8040, &args![this, node, second, info]).i32();
        assert_eq!(size, 3);
        assert!(pushed(&e, generator).is_empty());
    }

    #[test]
    fn a_leaf_above_the_level_limit_is_assembled_when_allowed() {
        let (mut e, this, generator) = sub_node_setup();
        let node = make_leaf(&mut e, 5);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_i32(info, 10);
        let size = e.call(0x00e0_8040, &args![this, node, second, info]).i32();
        assert_eq!(size, 1);
        assert_eq!(pushed(&e, generator), [0x35]);
        assert_eq!(e.mem.u8(node + 0x38), 1);
    }

    #[test]
    fn a_big_leaf_below_the_level_limit_is_refused() {
        let (mut e, this, generator) = sub_node_setup();
        let node = make_leaf(&mut e, 5);
        e.mem.set_i32(node + 8, 0x20);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_i32(info, 3);
        e.mem.set_u8(info + 4, 1);
        let size = e.call(0x00e0_8040, &args![this, node, second, info]).i32();
        assert_eq!(size, -1);
        assert!(pushed(&e, generator).is_empty());
    }

    #[test]
    fn a_node_on_a_chunk_level_is_assembled_from_a_fresh_copy() {
        let (mut e, this, generator) = sub_node_setup();
        e.mem.set_i32(this + 0x18, 10);
        let node = make_leaf(&mut e, 5);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let size = e.call(0x00e0_8040, &args![this, node, second, info]).i32();
        assert_eq!(size, 1);
        assert_eq!(pushed(&e, generator), [0x35]);
        assert_eq!(e.mem.i32(this + 0x18), 10);
        assert_eq!(e.mem.u8(info + 4), 0);
        assert_eq!(e.mem.u8(node + 0x38), 1);
    }

    #[test]
    fn an_inner_node_assembles_both_children_and_then_its_own_split() {
        let (mut e, this, generator) = sub_node_setup();
        let node = e.mem.alloc(0x80);
        let vector = e.mem.alloc(0x10);
        e.mem.set_u32(node + 0x60, vector);
        e.mem.set_f32(node + 0x64, 4.0);
        e.mem.set_f32(node + 0x68, 6.0);
        let later = make_leaf(&mut e, 5);
        let earlier = make_leaf(&mut e, 5);
        e.mem.set_u32(node + 0x74, later);
        e.mem.set_u32(node + 0x70, earlier);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_u8(info + 4, 1);
        e.mem.set_u32(info + 0x24, 1);
        let size = e.call(0x00e0_8040, &args![this, node, second, info]).i32();
        assert_eq!(size, 6);
        assert_eq!(pushed(&e, generator), [0x35, 0x35, 1, 4, 7, 0x12]);
        assert_eq!(e.mem.u32(later + 0x5c), 1);
        assert_eq!(e.mem.u32(earlier + 0x5c), 2);
        assert_eq!(e.mem.u32(node + 0x5c), 6);
        assert_eq!(e.mem.u8(node + 0x38), 1);
    }

    #[test]
    fn an_inner_node_without_an_earlier_child_fails() {
        let (mut e, this, generator) = sub_node_setup();
        let node = e.mem.alloc(0x80);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_u8(info + 4, 1);
        let size = e.call(0x00e0_8040, &args![this, node, second, info]).i32();
        assert_eq!(size, -1);
        assert!(pushed(&e, generator).is_empty());
    }

    /// An assembler whose table (+0x14) has an embedder when `embedder` is set and an empty
    /// eight-byte-element array at +4.
    fn chunk_table(e: &mut Engine, embedder: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x14, table);
        e.mem.set_u32(table + 0x1c, embedder);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, data);
        e.mem.set_u32(table + 0xc, 8);
        (this, table)
    }

    #[test]
    fn terminal_ids_of_a_leaf_are_recorded_with_the_chunk() {
        let mut e = new_engine();
        install_arrays(&mut e);
        let (this, table) = chunk_table(&mut e, 1);
        let node = make_leaf(&mut e, 0x55);
        e.mem.set_i32(node + 0x40, -1);
        e.mem.set_u8(node + 0x38, 1);
        let end = e.call(0x00e0_8240, &args![this, node, 3u32, 7u32]).u32();
        assert_eq!(end, 8);
        assert_eq!(e.mem.u8(node + 0x38), 0);
        assert_eq!(e.mem.u32(node + 0x24), 0);
        assert_eq!(e.mem.u32(node + 0x28), 8);
        let terminals = e.mem.u32(node + 0x60);
        assert_eq!(e.mem.u32(terminals + 0x10), 0x55);
        assert_eq!(e.mem.u32(terminals), 7);
        assert_eq!(array_words(&e, table + 4), [0x55, (3 << 8) | 7]);
    }

    #[test]
    fn terminal_ids_of_an_inner_node_thread_the_counter_through_its_children() {
        let mut e = new_engine();
        install_arrays(&mut e);
        let (this, table) = chunk_table(&mut e, 1);
        let node = e.mem.alloc(0x80);
        e.mem.set_i32(node + 0x40, -1);
        let later = make_leaf(&mut e, 0x11);
        let earlier = make_leaf(&mut e, 0x22);
        e.mem.set_i32(later + 0x40, -1);
        e.mem.set_i32(earlier + 0x40, -1);
        e.mem.set_u32(node + 0x74, later);
        e.mem.set_u32(node + 0x70, earlier);
        let end = e.call(0x00e0_8240, &args![this, node, 3u32, 7u32]).u32();
        assert_eq!(end, 9);
        assert_eq!(e.mem.u32(node + 0x28), 9);
        assert_eq!(
            array_words(&e, table + 4),
            [0x11, (3 << 8) | 7, 0x22, (3 << 8) | 8]
        );
    }

    #[test]
    fn terminal_ids_of_a_chunk_node_use_its_own_chunk_and_return_the_input() {
        let mut e = new_engine();
        install_arrays(&mut e);
        let (this, table) = chunk_table(&mut e, 0);
        let node = make_leaf(&mut e, 0x55);
        e.mem.set_i32(node + 0x40, 2);
        let end = e.call(0x00e0_8240, &args![this, node, 3u32, 7u32]).u32();
        assert_eq!(end, 7);
        assert_eq!(e.mem.u32(table + 8), 0);
        assert_eq!(e.mem.u8(node + 0x38), 0);
    }

    #[test]
    fn jump_records_are_copied_with_the_offset_added() {
        let mut e = new_engine();
        install_arrays(&mut e);
        let destination = make_array(&mut e, 4, 8);
        let source = make_array(&mut e, 4, 8);
        let data = e.mem.u32(source);
        for (index, value) in [0x10u32, 0x77, 0x20, 0x88].into_iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, value);
        }
        e.mem.set_u32(source + 4, 2);
        e.call(0x00e0_8320, &args![destination, source, 3i32]);
        assert_eq!(array_words(&e, destination), [0x13, 0x77, 0x23, 0x88]);
        assert_eq!(e.mem.u32(destination + 4), 2);
    }

    /// A source generator holding the five bytes 10..15 and one jump record `(2, 0x77)`.
    fn make_source(e: &mut Engine) -> u32 {
        let source = make_generator(e, 0x20);
        e.mem.set_u32(source + 0xc, 5);
        let top = e.mem.u32(source + 0x10) + 0x20;
        for index in 0..5u32 {
            e.mem.set_u8(top - 5 + index, 10 + index as u8);
        }
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(records, 2);
        e.mem.set_u32(records + 4, 0x77);
        e.mem.set_u32(source + 0x14, records);
        e.mem.set_u32(source + 0x18, 1);
        e.mem.set_u32(source + 0x1c, 4);
        source
    }

    #[test]
    fn code_is_copied_with_padding_and_the_jump_records_are_moved() {
        let mut e = new_engine();
        install_arrays(&mut e);
        let generator = make_generator(&mut e, 0x40);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(generator + 0x14, records);
        e.mem.set_u32(generator + 0x1c, 4);
        let source = make_source(&mut e);
        let assembler = e.mem.alloc(0x40);
        let terminals = e.mem.alloc(8);
        e.mem.set_u32(terminals + 4, 9);
        let size = e
            .call(0x00e0_8380, &args![generator, assembler, source, terminals])
            .i32();
        assert_eq!(size, 8);
        assert_eq!(e.mem.u32(terminals + 4), 0);
        assert_eq!(e.mem.u32(generator + 0xc), 8);
        let top = e.mem.u32(generator + 0x10) + 0x40;
        let bytes = e.mem.bytes(top - 8, 8);
        assert_eq!(bytes, [10, 11, 12, 13, 14, 0xcd, 0xcd, 0xcd]);
        assert_eq!(array_words(&e, generator + 0x14), [5, 0x77]);
        assert_eq!(
            calls(&e, VALIDATE_JUMP_COMMANDS),
            [vec![source], vec![generator]]
        );
    }

    #[test]
    fn terminal_data_is_written_through_the_memory_router_cursor() {
        let mut e = new_engine();
        let (_, router) = install_arrays(&mut e);
        let cursor = e.mem.alloc(0x200);
        e.mem.set_u32(router, cursor);
        let embedder = make_object(&mut e, 4, 0x0900_0020);
        let vtable = e.mem.u32(embedder);
        e.mem.set_u32(vtable + 8, 0x0900_0021);
        e.register(0x0900_0020, |_, _| word(6));
        e.register(0x0900_0021, |e, a| {
            for index in 0..6u32 {
                e.mem.set_u8(a[3] + index, 0x20 + index as u8);
            }
            Ret::default()
        });
        let generator = make_generator(&mut e, 0x40);
        let source = make_source(&mut e);
        let assembler = e.mem.alloc(0x40);
        e.mem.set_u32(assembler + 0x1c, embedder);
        let terminals = e.mem.alloc(8);
        e.mem.set_u32(terminals, 0x1234);
        e.mem.set_u32(terminals + 4, 1);
        let size = e
            .call(0x00e0_8380, &args![generator, assembler, source, terminals])
            .i32();
        assert_eq!(size, 8);
        let top = e.mem.u32(generator + 0x10) + 0x40;
        assert_eq!(
            e.mem.bytes(top - 6, 6),
            [0x20, 0x21, 0x22, 0x23, 0x24, 0x25]
        );
        assert_eq!(e.mem.bytes(top - 14, 5), [10, 11, 12, 13, 14]);
        assert_eq!(e.mem.u32(router), cursor);
        assert_eq!(calls(&e, 0x0900_0020), [vec![embedder, 0x1234, 1]]);
        assert_eq!(calls(&e, 0x0900_0021), [vec![embedder, 0x1234, 1, cursor]]);
        let source_top = e.mem.u32(source + 0x10) + 0x20;
        assert_eq!(
            calls(&e, MEM_COPY),
            [vec![top - 6, cursor, 6], vec![top - 14, source_top - 5, 5]]
        );
    }

    #[test]
    fn a_jump_chunk_command_records_the_chunk_it_jumps_to() {
        let mut e = new_engine();
        install_arrays(&mut e);
        let (this, generator) = make_assembler(&mut e, 0x40);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(generator + 0x14, records);
        e.mem.set_u32(generator + 0x1c, 4);
        let node = e.mem.alloc(0x80);
        e.mem.set_u32(node + 0x40, 9);
        let size = e.call(0x00e0_8470, &args![this, node]).i32();
        assert_eq!(size, 5);
        assert_eq!(pushed(&e, generator), [9, 0, 0, 0, 0x70]);
        assert_eq!(e.mem.u32(node + 0x5c), 5);
        assert_eq!(e.mem.u8(node + 0x38), 1);
        assert_eq!(array_words(&e, generator + 0x14), [5, 9]);
        assert_eq!(calls(&e, VALIDATE_JUMP_COMMANDS), [vec![generator]]);
    }

    /// Doubles for the generator constructor (fields and a buffer) and destructor.
    fn install_generator_doubles(e: &mut Engine) {
        e.register(GENERATOR_CTOR, |e, a| {
            let (this, capacity) = (a[0], a[1]);
            let buffer = e.mem.alloc(capacity);
            e.mem.set_u32(this + 8, capacity);
            e.mem.set_u32(this + 0xc, 0);
            e.mem.set_u32(this + 0x10, buffer);
            word(this)
        });
        e.register(GENERATOR_DTOR, |_, _| Ret::default());
    }

    /// The words of an array of four-byte elements.
    fn list_words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    /// An assembler for the chunk code: the generator, a table (+0x14) with the chunk limit
    /// `limit`, an empty record array at table +0x10, scale 2.0 and the array doubles.
    /// Returns `(engine, this, generator, table)`.
    fn chunk_points_rig(limit: i32) -> (Engine, u32, u32, u32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        install_arrays(&mut e);
        install_generator_doubles(&mut e);
        let (this, generator) = make_assembler(&mut e, 0x100);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x14, table);
        e.mem.set_i32(table, limit);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, records);
        e.mem.set_u32(table + 0x18, 4);
        e.mem.set_f32(this + 0x3c, 2.0);
        (e, this, generator, table)
    }

    /// A leaf that assembles to seven bytes: a five-byte distance and one terminal command.
    fn make_big_leaf(e: &mut Engine, distance: u32) -> u32 {
        let node = make_leaf(e, distance);
        e.mem.set_u32(node + 0x2c, 1);
        e.mem.set_u32(node + 0x30, 9);
        node
    }

    #[test]
    fn a_leaf_chunk_point_pushes_its_first_terminal() {
        let (mut e, this, generator, _) = chunk_points_rig(1000);
        let node = make_leaf(&mut e, 0x42);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let out = make_array(&mut e, 4, 4);
        let size = e
            .call(0x00e0_84f0, &args![this, node, second, info, out])
            .i32();
        assert_eq!(size, 2);
        assert_eq!(pushed(&e, generator), [0x42, 0x50]);
        assert_eq!(list_words(&e, out), [0x42]);
        assert_eq!(e.mem.u32(info + 0x34), 0);
    }

    #[test]
    fn a_leaf_chunk_point_adds_the_embedded_size_of_its_terminal() {
        use std::cell::Cell;
        use std::rc::Rc;
        let (mut e, this, generator, table) = chunk_points_rig(1000);
        let embedder = make_object(&mut e, 4, 0x0900_0030);
        e.mem.set_u32(table + 0x1c, embedder);
        let seen = Rc::new(Cell::new(0));
        let seen_in_double = Rc::clone(&seen);
        e.register_double(0x0900_0030, move |e, a| {
            seen_in_double.set(e.mem.u32(a[1]));
            word(3)
        });
        let node = make_leaf(&mut e, 0x42);
        let second = e.mem.alloc(0x60);
        e.mem.set_u32(second + 0x34, 0x99);
        let info = e.mem.alloc(0x60);
        let out = make_array(&mut e, 4, 4);
        let size = e
            .call(0x00e0_84f0, &args![this, node, second, info, out])
            .i32();
        assert_eq!(size, 4);
        assert_eq!(pushed(&e, generator), [0x30]);
        assert_eq!(e.mem.u32(info + 0x34), 0x42);
        assert_eq!(seen.get(), 0x42);
        assert_eq!(calls(&e, 0x0900_0030)[0][0], embedder);
        assert_eq!(calls(&e, 0x0900_0030)[0][2], 1);
    }

    /// An inner node with two big leaves (distances `0x1000042` at +0x74 and `0x1000043` at +0x70).
    fn chunk_inner_node(e: &mut Engine) -> (u32, u32, u32, u32, u32, u32) {
        let node = e.mem.alloc(0x80);
        let vector = e.mem.alloc(0x10);
        e.mem.set_u32(node + 0x60, vector);
        e.mem.set_f32(node + 0x64, 4.0);
        e.mem.set_f32(node + 0x68, 6.0);
        let later = make_big_leaf(e, 0x100_0042);
        let earlier = make_big_leaf(e, 0x100_0043);
        e.mem.set_u32(node + 0x74, later);
        e.mem.set_u32(node + 0x70, earlier);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info + 0x24, 1);
        e.mem.set_u32(info + 0x44, 1);
        (node, later, earlier, second, info, 0)
    }

    #[test]
    fn an_inner_chunk_point_merges_the_lists_of_its_children() {
        let (mut e, this, generator, table) = chunk_points_rig(1000);
        let (node, later, earlier, second, info, _) = chunk_inner_node(&mut e);
        let out = make_array(&mut e, 4, 4);
        let size = e
            .call(0x00e0_84f0, &args![this, node, second, info, out])
            .i32();
        assert_eq!(size, 18);
        assert_eq!(
            pushed(&e, generator),
            [0x42, 0, 0, 1, 0x53, 9, 0x60, 0x43, 0, 0, 1, 0x53, 9, 0x60, 7, 4, 7, 0x12]
        );
        assert_eq!(list_words(&e, out), [0x100_0042, 0x100_0043]);
        assert_eq!(e.mem.u32(table + 0x14), 0);
        assert_eq!(e.mem.u32(later + 0x5c), 7);
        assert_eq!(e.mem.u32(earlier + 0x5c), 14);
        assert_eq!(e.mem.u32(node + 0x5c), 18);
    }

    #[test]
    fn an_inner_chunk_point_turns_children_into_jumps_until_it_fits() {
        let (mut e, this, generator, table) = chunk_points_rig(15);
        let (node, later, earlier, second, info, _) = chunk_inner_node(&mut e);
        let out = make_array(&mut e, 4, 4);
        e.mem.set_u32(out + 4, 3);
        let size = e
            .call(0x00e0_84f0, &args![this, node, second, info, out])
            .i32();
        assert_eq!(size, 14);
        assert_eq!(e.mem.u32(generator + 0xc), 14);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(table + 0x14), 2);
        assert_eq!(array_words(&e, table + 0x10)[1], 12);
        assert_eq!(array_words(&e, table + 0x10)[3], 12);
        assert_eq!(e.mem.u32(later + 0x40), 0);
        assert_eq!(e.mem.u32(earlier + 0x40), 1);
        assert_eq!(e.mem.u32(later + 0x5c), 3);
        assert_eq!(e.mem.u32(earlier + 0x5c), 10);
        assert_eq!(e.mem.u32(node + 0x5c), 14);
    }

    #[test]
    fn a_leaf_in_a_chunk_pushes_the_word_at_0x10_of_its_terminals() {
        let (mut e, this, generator, _) = chunk_points_rig(1000);
        let node = make_leaf(&mut e, 5);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let list = make_array(&mut e, 4, 4);
        e.call(0x00e0_8a60, &args![this, node, second, info, 0i32, list]);
        assert_eq!(pushed(&e, generator), [0x35]);
        assert_eq!(list_words(&e, list), [0x1005]);
    }

    #[test]
    fn an_inner_node_in_a_chunk_assembles_its_children_in_order() {
        let (mut e, this, generator, _) = chunk_points_rig(1000);
        let (node, _, _, second, info, _) = chunk_inner_node(&mut e);
        let later = make_leaf(&mut e, 5);
        let earlier = make_leaf(&mut e, 6);
        e.mem.set_u32(node + 0x74, later);
        e.mem.set_u32(node + 0x70, earlier);
        let list = make_array(&mut e, 4, 4);
        e.call(0x00e0_8a60, &args![this, node, second, info, 0i32, list]);
        assert_eq!(pushed(&e, generator), [0x35, 0x36, 1, 4, 7, 0x12]);
        // The later child comes first; each pushes the word at +0x10 of its terminals.
        assert_eq!(list_words(&e, list), [0x1005, 0x1006]);
    }

    #[test]
    fn a_chunk_node_is_assembled_into_a_generator_of_its_own() {
        let (mut e, this, generator, table) = chunk_points_rig(1000);
        let records = e.mem.u32(table + 0x10);
        e.mem.set_u32(records + 12, 0x40);
        let node = make_leaf(&mut e, 5);
        e.mem.set_i32(node + 0x40, 1);
        let second = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let list = make_array(&mut e, 4, 4);
        e.call(0x00e0_8a60, &args![this, node, second, info, 0i32, list]);
        assert_eq!(e.mem.u32(this + 0x10), generator);
        assert_eq!(pushed(&e, generator), [1, 0, 0, 0, 0x70]);
        assert_eq!(array_words(&e, generator + 0x14), [5, 1]);
        let chunk_generator = e.mem.u32(records + 8);
        assert_ne!(chunk_generator, 0);
        assert_eq!(e.mem.u16(chunk_generator + 4), 0x20);
        assert_eq!(e.mem.u32(chunk_generator + 0xc), 4);
        let top = e.mem.u32(chunk_generator + 0x10) + 0x40;
        assert_eq!(e.mem.bytes(top - 4, 4), [0x35, 0xcd, 0xcd, 0xcd]);
        let constructed = calls(&e, GENERATOR_CTOR);
        assert_eq!(constructed.len(), 2);
        assert_eq!(constructed[0][1], 0x40);
        assert_eq!(constructed[1], [chunk_generator, 0x40]);
        assert_eq!(calls(&e, GENERATOR_DTOR), [vec![constructed[0][0]]]);
        assert_eq!(e.mem.u8(info + 4), 1);
        assert_eq!(e.mem.u8(node + 0x38), 1);
        assert_eq!(e.mem.u32(node + 0x5c), 5);
    }

    /// An assembler for `assemble`: the vtable slot +0x18 is `getScaleInfo`, the divisor is 5,
    /// and the tree is one leaf whose extents run 0..10 and whose +0x28 word is `0x100`.
    fn assemble_setup() -> (Engine, u32, u32, u32) {
        let mut e = new_engine();
        install_float_doubles(&mut e);
        map_split_constants(&mut e);
        install_arrays(&mut e);
        install_generator_doubles(&mut e);
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        let (this, generator) = make_assembler(&mut e, 0x100);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, 0x00e0_7b90);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x70, 5);
        let node = make_leaf(&mut e, 5);
        e.mem.set_u32(node + 0x28, 0x100);
        for lane in 0..3u32 {
            e.mem.set_f32(node + 0x10 + 8 * lane, 10.0);
        }
        (e, this, generator, node)
    }

    #[test]
    fn assemble_scales_the_tree_and_assembles_it_into_the_generator() {
        let (mut e, this, generator, node) = assemble_setup();
        e.call(0x00e0_8cc0, &args![this, node, 0x77u32, 100i32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x77);
        assert_eq!(e.mem.i32(this + 0x24), 100);
        assert_eq!(e.mem.i32(this + 0x18), 14);
        assert_eq!(e.mem.i32(this + 0x1c), 4);
        assert_eq!(e.mem.f32(this + 0x3c), 1_671_168.0);
        assert_eq!(e.mem.u32(this + 0x28), 1);
        assert_eq!(e.mem.u8(node + 0x38), 1);
        assert_eq!(
            pushed(&e, generator),
            [0x35, 0, 0, 0x26, 0, 0, 0x27, 0, 0, 0x28]
        );
    }

    #[test]
    fn assemble_without_a_tree_only_asks_for_the_scale_info() {
        let (mut e, this, generator, _) = assemble_setup();
        // The real `getScaleInfo` reads the node, so a null one needs another implementation.
        let vtable = e.mem.u32(this);
        e.mem.set_u32(vtable + 0x18, 0x0900_0018);
        e.register(0x0900_0018, |_, _| Ret::default());
        e.mem.set_u32(this + 0x28, 9);
        e.call(0x00e0_8cc0, &args![this, 0u32, 0x77u32, 100i32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x77);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert!(pushed(&e, generator).is_empty());
        assert_eq!(calls(&e, 0x0900_0018), [vec![this, 0, this + 0x30]]);
    }

    #[test]
    fn assemble_with_chunks_stores_the_generator_in_the_chunk_record() {
        let (mut e, this, generator, node) = assemble_setup();
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x14, table);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, records);
        e.mem.set_u32(table + 0x18, 4);
        e.call(0x00e0_8cc0, &args![this, node, 0x77u32, 100i32]);
        assert_eq!(e.mem.u32(this + 0x10), generator);
        assert_eq!(e.mem.u32(table + 0x14), 1);
        assert_eq!(e.mem.u32(node + 0x40), 0);
        assert_eq!(e.mem.u32(records), generator);
        assert_eq!(e.mem.u32(records + 4), 12);
        assert_eq!(calls(&e, ADD_REFERENCE), [vec![generator]]);
        let constructed = calls(&e, GENERATOR_CTOR);
        assert_eq!(constructed.len(), 2);
        assert_eq!(constructed[0][1], 0x10_0000);
        assert_eq!(calls(&e, GENERATOR_DTOR).len(), 2);
    }

    /// Checks `actual` against `expected` to within a few ULPs (the `RSQRTSS` estimate and one
    /// Newton step leave that much, and the estimate differs between processors).
    fn assert_close(actual: u32, expected: f32) {
        if expected == 0.0 {
            assert_eq!(actual & 0x7fff_ffff, 0);
        } else {
            assert_eq!(f32::from_bits(actual).signum(), expected.signum());
            let distance = (i64::from(actual & 0x7fff_ffff)
                - i64::from(expected.to_bits() & 0x7fff_ffff))
            .abs();
            assert!(
                distance <= 4,
                "{} against {expected}",
                f32::from_bits(actual)
            );
        }
    }

    #[test]
    fn the_constructor_lays_out_thirteen_normalized_planes() {
        let mut e = new_engine();
        let constants = [
            (PLANE_FIFTH, 0.2f32),
            (PLANE_HALF, 0.5),
            (PLANE_TWENTIETH, 0.05),
            (SPLIT_LANE_FILL, 0.0),
            (PLANE_ONE, 1.0),
            (PLANE_NEGATIVE_ONE, -1.0),
            (PLANE_QUARTER, 0.25),
            (PLANE_THREE_TENTHS, 0.3),
            (PLANE_VALUE_32, 0.32),
            (PLANE_VALUE_34, 0.34),
            (PLANE_THREE, 3.0),
        ];
        for (address, value) in constants {
            map_f32(&mut e, address, value);
        }
        let allocator = make_object(&mut e, 0xc, 0x0900_0040);
        install_router(&mut e, allocator);
        e.register(0x0900_0040, |e, a| {
            e.mem.set_u32(a[1], 0x1a0);
            word(e.mem.alloc(0x1a0))
        });
        let this = e.mem.alloc(0x80);
        let settings = e.mem.alloc(0x30);
        let result = e.call(0x00e0_90c0, &args![this, settings, 0x1234u32, 0x55u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), ASSEMBLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x44), 13);
        assert_eq!(e.mem.u32(this + 0x48), 13);
        assert_eq!(e.mem.u32(this + 8), 0x55);
        assert_eq!(e.mem.u32(this + 0x10), 0x1234);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        let planes = e.mem.u32(this + 0x40);
        let root = 0.57735027f32;
        let half_root = std::f32::consts::FRAC_1_SQRT_2;
        let expected = [
            (0x000, [1.0, 0.0, 0.0], 0u32),
            (0x060, [0.0, half_root, half_root], 0x3e4c_cccd),
            (0x0c0, [0.0, half_root, -half_root], 0x3e80_0000),
            (0x120, [root, root, root], 0x3e99_999a),
            (0x150 - 0x10, [root, root, -root], 0x3ea3_d70a),
            (0x180, [root, -root, -root], 0x3eae_147b),
        ];
        for (offset, direction, value) in expected {
            for (lane, component) in direction.into_iter().enumerate() {
                assert_close(e.mem.u32(planes + offset + 4 * lane as u32), component);
            }
            assert_eq!(e.mem.u32(planes + offset + 0xc), 0);
            assert_eq!(e.mem.u32(planes + offset + 0x10), value, "{offset:#x}");
            assert_eq!(e.mem.u32(planes + offset + 0x14), offset / 0x20);
        }
    }
}
