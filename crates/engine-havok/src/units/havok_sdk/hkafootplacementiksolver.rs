//! `Havok SDK/hkafootplacementiksolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the object `fn_00cd8820` builds (`010cb644`), the 16-byte vector at
/// `01267e30` (zero in the exe) several constructors copy, and the float globals the second
/// constructor reads.
const BUILT_OBJECT_VTABLE: u32 = 0x010c_b644;
const VECTOR_GLOBAL: u32 = 0x0126_7e30;
const INPUT_DEFAULT_A: u32 = 0x0101_8238;
const INPUT_DEFAULT_B: u32 = 0x0105_0de4;
const INPUT_DEFAULT_C: u32 = 0x0101_2054;
const INPUT_DEFAULT_D: u32 = 0x0101_6248;
const INPUT_DEFAULT_E: u32 = 0x0103_0ff0;
/// The float `Input::Input` stores at +0x68 (`0101df44`).
const INPUT_FLOAT: u32 = 0x0101_df44;

/// Copies the four words of the vector global to `to`.
fn copy_vector_global(e: &mut Engine, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(VECTOR_GLOBAL + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00cd8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRaycastInterface::castRay` (Xbox PDB; thiscall, five stack words, `this` not read): the
/// base implementation. Clears the byte at `hit` and returns `hit`; the other four words are
/// not read.
pub fn hka_raycast_interface_cast_ray(
    e: &mut Engine,
    _this: Ptr,
    hit: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    e.mem.set_u8(hit.addr(), 0);
    hit
}

// Translated from 00cd8810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRaycastInterface::castRay_ov2` (Xbox PDB; thiscall, six stack words, `this` not read):
/// the base implementation. Clears the byte at `hit` and returns `hit`; the other five words
/// are not read.
pub fn hka_raycast_interface_cast_ray_ov2(
    e: &mut Engine,
    _this: Ptr,
    hit: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> Ptr {
    e.mem.set_u8(hit.addr(), 0);
    hit
}

// Translated from 00cd8820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an unnamed object (thiscall, one stack word `source`; no PDB name): sets the
/// vtable `010cb644` and the 16-bit word at +6 to 1, copies the 0x80 bytes at `source` to +0x10,
/// clears the floats at +0x90, +0x94 and +0xb0 and the byte at +0xb4, copies the four words at
/// `source + 0x30` to +0xa0 and fills +0xc0, +0xd0 and +0xe0 with the vector at `01267e30`.
/// Returns `this`.
pub fn fn_00cd8820(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    let from = source.addr();
    e.mem.set_u32(me, BUILT_OBJECT_VTABLE);
    e.mem.set_u16(me + 6, 1);
    for word in 0..0x20 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(me + 0x10 + 4 * word, value);
    }
    e.mem.set_f32(me + 0x90, 0.0);
    e.mem.set_f32(me + 0x94, 0.0);
    for word in 0..4 {
        let value = e.mem.u32(from + 0x30 + 4 * word);
        e.mem.set_u32(me + 0xa0 + 4 * word, value);
    }
    e.mem.set_f32(me + 0xb0, 0.0);
    e.mem.set_u8(me + 0xb4, 0);
    copy_vector_global(e, me + 0xc0);
    copy_vector_global(e, me + 0xd0);
    copy_vector_global(e, me + 0xe0);
    this
}

// Translated from 00cd8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an unnamed object (thiscall, no stack words; no PDB name): sets the dword at
/// +0 to 0, the 16-bit words at +4, +6 and +8 to `0xffff`, fills +0x10, +0x20, +0x30 and +0x40
/// with the vector at `01267e30`, clears the floats at +0x50, +0x54 and +0x58 and the byte at
/// +0x74 and stores the floats of five globals (`01018238`, `01050de4`, `01012054`, 1.0,
/// `01016248`, `01030ff0`) at +0x5c, +0x60, +0x64, +0x68, +0x6c and +0x70 (each a load/store
/// pair, so a signalling NaN is quieted). Returns `this`.
pub fn fn_00cd8920(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0);
    e.mem.set_u16(me + 4, 0xffff);
    e.mem.set_u16(me + 6, 0xffff);
    e.mem.set_u16(me + 8, 0xffff);
    for offset in [0x10, 0x20, 0x30, 0x40] {
        copy_vector_global(e, me + offset);
    }
    e.mem.set_f32(me + 0x50, 0.0);
    e.mem.set_f32(me + 0x54, 0.0);
    e.mem.set_f32(me + 0x58, 0.0);
    e.mem.set_u8(me + 0x74, 0);
    let copies = [
        (0x5c, INPUT_DEFAULT_A),
        (0x60, INPUT_DEFAULT_B),
        (0x64, INPUT_DEFAULT_C),
    ];
    for (offset, global) in copies {
        let bits = x87_quiet_bits(e.mem.u32(global));
        e.mem.set_u32(me + offset, bits);
    }
    e.mem.set_f32(me + 0x68, 1.0);
    for (offset, global) in [(0x6c, INPUT_DEFAULT_D), (0x70, INPUT_DEFAULT_E)] {
        let bits = x87_quiet_bits(e.mem.u32(global));
        e.mem.set_u32(me + offset, bits);
    }
    this
}

// Translated from 00cd8a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaFootPlacementIkSolver::Input::Input` (Xbox PDB; thiscall, no stack words): stores the
/// float at `0101df44` (a load/store pair, a signalling NaN quieted) at +0x68, 1.0 at +0x6c,
/// +0x70, +0x74, +0x78 and +0x7c, 0 at +0x64 and +0x80, and the bytes 1 at +0x60 and 0 at
/// +0x84. Returns `this`.
pub fn hka_foot_placement_ik_solver_input_input(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    let bits = x87_quiet_bits(e.mem.u32(INPUT_FLOAT));
    e.mem.set_u32(me + 0x68, bits);
    e.mem.set_u8(me + 0x60, 1);
    e.mem.set_f32(me + 0x6c, 1.0);
    e.mem.set_u32(me + 0x64, 0);
    e.mem.set_f32(me + 0x70, 1.0);
    e.mem.set_u32(me + 0x80, 0);
    e.mem.set_f32(me + 0x74, 1.0);
    e.mem.set_u8(me + 0x84, 0);
    e.mem.set_f32(me + 0x78, 1.0);
    e.mem.set_f32(me + 0x7c, 1.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_8800, hka_raycast_interface_cast_ray(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00cd_8810, hka_raycast_interface_cast_ray_ov2(Ptr, Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00cd_8820, fn_00cd8820(Ptr, Ptr) -> Ptr),
        entry!(0x00cd_8920, fn_00cd8920(Ptr) -> Ptr),
        entry!(0x00cd_8a10, hka_foot_placement_ik_solver_input_input(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Fills `len` bytes at `at` with a pattern that never repeats a byte value for long.
    fn fill(e: &mut Engine, at: u32, len: u32) {
        for index in 0..len {
            e.mem.set_u8(at + index, (index * 7 + 3) as u8);
        }
    }

    fn set_vector_global(e: &mut Engine) -> [u32; 4] {
        let words = [0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444];
        for (lane, word) in words.iter().enumerate() {
            e.mem.set_u32(VECTOR_GLOBAL + 4 * lane as u32, *word);
        }
        words
    }

    #[test]
    fn cast_ray_clears_the_hit_byte_and_returns_it() {
        let mut e = engine();
        let hit = e.mem.alloc(8);
        e.mem.set_u8(hit, 1);
        e.mem.set_u8(hit + 1, 9);
        let result =
            hka_raycast_interface_cast_ray(&mut e, Ptr::new(0x1234), Ptr::new(hit), 1, 2, 3, 4);
        assert_eq!(result, Ptr::new(hit));
        assert_eq!(e.mem.u8(hit), 0);
        assert_eq!(e.mem.u8(hit + 1), 9);
    }

    #[test]
    fn cast_ray_ov2_clears_the_hit_byte_and_returns_it() {
        let mut e = engine();
        let hit = e.mem.alloc(8);
        e.mem.set_u8(hit, 0xff);
        let result = hka_raycast_interface_cast_ray_ov2(
            &mut e,
            Ptr::new(0x1234),
            Ptr::new(hit),
            1,
            2,
            3,
            4,
            5,
        );
        assert_eq!(result, Ptr::new(hit));
        assert_eq!(e.mem.u8(hit), 0);
    }

    #[test]
    fn cast_ray_functions_take_their_stack_words_through_the_uniform_form() {
        let mut e = engine();
        let hit = e.mem.alloc(8);
        e.mem.set_u8(hit, 1);
        let result = e.call(0x00cd_8800, &args![0x1234u32, hit, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(result.u32(), hit);
        assert_eq!(e.mem.u8(hit), 0);
        e.mem.set_u8(hit, 1);
        let result = e.call(
            0x00cd_8810,
            &args![0x1234u32, hit, 1u32, 2u32, 3u32, 4u32, 5u32],
        );
        assert_eq!(result.u32(), hit);
        assert_eq!(e.mem.u8(hit), 0);
    }

    #[test]
    fn the_first_constructor_copies_the_source_and_fills_the_rest() {
        let mut e = engine();
        let vector = set_vector_global(&mut e);
        let source = e.mem.alloc(0x80);
        fill(&mut e, source, 0x80);
        let this = e.mem.alloc(0x100);
        for offset in 0..0x100 {
            e.mem.set_u8(this + offset, 0xaa);
        }
        let result = fn_00cd8820(&mut e, Ptr::new(this), Ptr::new(source));
        assert_eq!(result, Ptr::new(this));
        assert_eq!(e.mem.u32(this), BUILT_OBJECT_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        for offset in 0..0x80 {
            assert_eq!(
                e.mem.u8(this + 0x10 + offset),
                e.mem.u8(source + offset),
                "{offset}"
            );
        }
        assert_eq!(e.mem.u32(this + 0x90), 0);
        assert_eq!(e.mem.u32(this + 0x94), 0);
        for word in 0..4 {
            assert_eq!(
                e.mem.u32(this + 0xa0 + 4 * word),
                e.mem.u32(source + 0x30 + 4 * word)
            );
        }
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u8(this + 0xb4), 0);
        for base in [0xc0, 0xd0, 0xe0] {
            for (lane, word) in vector.iter().enumerate() {
                assert_eq!(e.mem.u32(this + base + 4 * lane as u32), *word);
            }
        }
        // Bytes the constructor does not set stay as they were.
        assert_eq!(e.mem.u8(this + 4), 0xaa);
        assert_eq!(e.mem.u8(this + 0x98), 0xaa);
        assert_eq!(e.mem.u8(this + 0xb5), 0xaa);
        assert_eq!(e.mem.u8(this + 0xf0), 0xaa);
    }

    #[test]
    fn the_first_constructor_is_reachable_through_the_uniform_form() {
        let mut e = engine();
        set_vector_global(&mut e);
        let source = e.mem.alloc(0x80);
        let this = e.mem.alloc(0x100);
        let result = e.call(0x00cd_8820, &args![this, source]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), BUILT_OBJECT_VTABLE);
    }

    #[test]
    fn the_second_constructor_sets_words_vectors_and_floats() {
        let mut e = engine();
        let vector = set_vector_global(&mut e);
        e.mem.set_u32(INPUT_DEFAULT_A, 0x3e80_0000);
        e.mem.set_u32(INPUT_DEFAULT_B, 0x7f80_0001);
        e.mem.set_u32(INPUT_DEFAULT_C, 0x4000_0000);
        e.mem.set_u32(INPUT_DEFAULT_D, 0xbf00_0000);
        e.mem.set_u32(INPUT_DEFAULT_E, 0x4120_0000);
        let this = e.mem.alloc(0x80);
        for offset in 0..0x80 {
            e.mem.set_u8(this + offset, 0xaa);
        }
        let result = fn_00cd8920(&mut e, Ptr::new(this));
        assert_eq!(result, Ptr::new(this));
        assert_eq!(e.mem.u32(this), 0);
        for offset in [4, 6, 8] {
            assert_eq!(e.mem.u16(this + offset), 0xffff);
        }
        for base in [0x10, 0x20, 0x30, 0x40] {
            for (lane, word) in vector.iter().enumerate() {
                assert_eq!(e.mem.u32(this + base + 4 * lane as u32), *word);
            }
        }
        for offset in [0x50, 0x54, 0x58] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x74), 0);
        assert_eq!(e.mem.u32(this + 0x5c), 0x3e80_0000);
        // A signalling NaN comes out quiet.
        assert_eq!(e.mem.u32(this + 0x60), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x64), 0x4000_0000);
        assert_eq!(e.mem.u32(this + 0x68), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x6c), 0xbf00_0000);
        assert_eq!(e.mem.u32(this + 0x70), 0x4120_0000);
        // Bytes it does not set stay as they were.
        assert_eq!(e.mem.u8(this + 0x0a), 0xaa);
        assert_eq!(e.mem.u8(this + 0x75), 0xaa);
    }

    #[test]
    fn input_constructor_sets_its_defaults() {
        let mut e = engine();
        e.mem.set_u32(INPUT_FLOAT, 0x3e4c_cccd);
        let this = e.mem.alloc(0x90);
        for offset in 0..0x90 {
            e.mem.set_u8(this + offset, 0xaa);
        }
        let result = hka_foot_placement_ik_solver_input_input(&mut e, Ptr::new(this));
        assert_eq!(result, Ptr::new(this));
        assert_eq!(e.mem.u32(this + 0x68), 0x3e4c_cccd);
        assert_eq!(e.mem.u8(this + 0x60), 1);
        for offset in [0x6c, 0x70, 0x74, 0x78, 0x7c] {
            assert_eq!(e.mem.u32(this + offset), 0x3f80_0000, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u8(this + 0x84), 0);
        // Neighbouring bytes are untouched.
        assert_eq!(e.mem.u8(this + 0x61), 0xaa);
        assert_eq!(e.mem.u8(this + 0x85), 0xaa);
    }

    #[test]
    fn input_constructor_quiets_a_signalling_nan_float() {
        let mut e = engine();
        e.mem.set_u32(INPUT_FLOAT, 0xff80_0003);
        let this = e.mem.alloc(0x90);
        hka_foot_placement_ik_solver_input_input(&mut e, Ptr::new(this));
        assert_eq!(e.mem.u32(this + 0x68), 0xffc0_0003);
    }
}
