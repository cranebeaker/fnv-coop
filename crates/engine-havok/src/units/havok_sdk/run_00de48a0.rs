//! `Havok SDK/run_00de48a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplaySemiCircle` (class confirmed by the RTTI of its vtable `010d8274`; the field names
//! follow `hkdisplaysemicircle.rs`, which describes the same object and whose names are
//! unconfirmed): the geometry block pointer at `+8`, three rotation rows at `+0x10`, `+0x20`,
//! `+0x30`, a vector at `+0x40`, the axis vector at `+0x60`, the start direction at `+0x70`,
//! the centre at `+0x80`, the start angle (`float`) at `+0x90`, the end angle at `+0x94`, the
//! radius at `+0x98` and the number of segments (`int`) at `+0x9c`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkDisplaySemiCircle` (RTTI).
const VTABLE: u32 = 0x010d_8274;

/// The vector getters: copies the four words at `from` to `out` the way the exe does (the
/// first two words are read and written in turn, the last two are both read before either is
/// written), and returns `out` (`EAX`).
fn copy_vector_out(e: &mut Engine, from: u32, out: u32) -> Ptr {
    let first = e.mem.u32(from);
    e.mem.set_u32(out, first);
    let second = e.mem.u32(from.wrapping_add(4));
    e.mem.set_u32(out.wrapping_add(4), second);
    let third = e.mem.u32(from.wrapping_add(8));
    let fourth = e.mem.u32(from.wrapping_add(0xc));
    e.mem.set_u32(out.wrapping_add(8), third);
    e.mem.set_u32(out.wrapping_add(0xc), fourth);
    Ptr::new(out)
}

/// A `MOVAPS` load of the four words at `from`.
fn load_vector(e: &Engine, from: u32) -> [u32; 4] {
    std::array::from_fn(|lane| e.mem.u32(from.wrapping_add(4 * lane as u32)))
}

fn store_vector(e: &mut Engine, at: u32, v: [u32; 4]) {
    for (lane, word) in v.iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * lane as u32), *word);
    }
}

/// The identity rotation block and zeroed vectors both constructors begin with: the word at
/// `+6` is 1, `+0x50` is 5, `+8` is 0, the three rows at `+0x10`, `+0x20`, `+0x30` are zero
/// and then get 1.0 on the diagonal (`FST` of `1.0` into `+0x10`, `+0x24`, `+0x38`), and the
/// vector at `+0x40` is zero.
fn start_block(e: &mut Engine, me: u32) {
    e.mem.set_u16(me.wrapping_add(6), 1);
    e.mem.set_u32(me.wrapping_add(0x50), 5);
    e.mem.set_u32(me.wrapping_add(8), 0);
    for row in [0x10, 0x20, 0x30] {
        store_vector(e, me.wrapping_add(row), [0; 4]);
    }
    e.mem.set_f32(me.wrapping_add(0x10), 1.0);
    e.mem.set_f32(me.wrapping_add(0x24), 1.0);
    e.mem.set_f32(me.wrapping_add(0x38), 1.0);
    store_vector(e, me.wrapping_add(0x40), [0; 4]);
}

// Translated from 00de48a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the axis: copies the four words at `this + 0x60` to `out` (thiscall, `RET 4`) and
/// returns `out`.
pub fn fn_00de48a0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_vector_out(e, this.addr().wrapping_add(0x60), out.addr())
}

// Translated from 00de48c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the start direction: copies the four words at `this + 0x70` to `out` (thiscall,
/// `RET 4`) and returns `out`.
pub fn fn_00de48c0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_vector_out(e, this.addr().wrapping_add(0x70), out.addr())
}

// Translated from 00de48e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the start angle: `FLD` of the float at `this + 0x90`, the result in `ST0` (a
/// signalling NaN comes out quiet).
pub fn fn_00de48e0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x90)))
}

// Translated from 00de48f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the end angle: `FLD` of the float at `this + 0x94`, the result in `ST0`.
pub fn fn_00de48f0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x94)))
}

// Translated from 00de4900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the radius: `FLD` of the float at `this + 0x98`, the result in `ST0`.
pub fn fn_00de4900(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x98)))
}

// Translated from 00de4910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the number of segments: the integer at `this + 0x9c` in `EAX`.
pub fn fn_00de4910(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr().wrapping_add(0x9c))
}

// Translated from 00de4920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the centre: copies the four words at `this + 0x80` to `out` (thiscall, `RET 4`)
/// and returns `out`.
pub fn fn_00de4920(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_vector_out(e, this.addr().wrapping_add(0x80), out.addr())
}

// Translated from 00de4950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of all parameters (the engine map has no name), thiscall with seven words `(radius,
/// start_angle, end_angle, segments, centre, axis, start_direction)`: the vectors are copied
/// with `MOVAPS` from the three pointers to `+0x80`, `+0x60` and `+0x70`, the three floats
/// pass through the x87 stack (`FLD`/`FSTP`: a signalling NaN is stored quiet) to `+0x98`,
/// `+0x90` and `+0x94`, the segment count goes to `+0x9c`. The exe's order of reads and
/// writes is kept (the third vector is read before the first angle is stored).
#[allow(clippy::too_many_arguments)] // the exe takes seven words and `this`
pub fn fn_00de4950(
    e: &mut Engine,
    this: Ptr,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: u32,
    centre: Ptr,
    axis: Ptr,
    start_direction: Ptr,
) {
    let me = this.addr();
    let centre_vector = load_vector(e, centre.addr());
    store_vector(e, me.wrapping_add(0x80), centre_vector);
    let axis_vector = load_vector(e, axis.addr());
    store_vector(e, me.wrapping_add(0x60), axis_vector);
    let direction_vector = load_vector(e, start_direction.addr());
    e.mem
        .set_u32(me.wrapping_add(0x90), x87_quiet_bits(start_angle.to_bits()));
    store_vector(e, me.wrapping_add(0x70), direction_vector);
    e.mem
        .set_u32(me.wrapping_add(0x94), x87_quiet_bits(end_angle.to_bits()));
    e.mem.set_u32(me.wrapping_add(0x9c), segments);
    e.mem
        .set_u32(me.wrapping_add(0x98), x87_quiet_bits(radius.to_bits()));
}

// Translated from 00de49b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySemiCircle::hkDisplaySemiCircle` (constructor with parameters; class confirmed by
/// the RTTI of the vtable it stores), thiscall with seven words `(centre, axis,
/// start_direction, start_angle, end_angle, radius, segments)`: sets the identity rotation
/// block, the vtable (`010d8274`) and copies the three vectors from their pointers to `+0x80`,
/// `+0x60` and `+0x70` as plain words (the centre's last word is stored after the angle
/// stores); the three floats pass through the x87 stack. Returns `this`.
#[allow(clippy::too_many_arguments)] // the exe takes seven words and `this`
pub fn fn_00de49b0(
    e: &mut Engine,
    this: Ptr,
    centre: Ptr,
    axis: Ptr,
    start_direction: Ptr,
    start_angle: f32,
    end_angle: f32,
    radius: f32,
    segments: u32,
) -> Ptr {
    let me = this.addr();
    start_block(e, me);
    e.mem.set_u32(me, VTABLE);
    let axis_vector = load_vector(e, axis.addr());
    store_vector(e, me.wrapping_add(0x60), axis_vector);
    let direction_vector = load_vector(e, start_direction.addr());
    store_vector(e, me.wrapping_add(0x70), direction_vector);
    for word in 0..3 {
        let value = e.mem.u32(centre.addr().wrapping_add(4 * word));
        e.mem.set_u32(me.wrapping_add(0x80 + 4 * word), value);
    }
    let centre_last = e.mem.u32(centre.addr().wrapping_add(0xc));
    e.mem
        .set_u32(me.wrapping_add(0x90), x87_quiet_bits(start_angle.to_bits()));
    e.mem
        .set_u32(me.wrapping_add(0x94), x87_quiet_bits(end_angle.to_bits()));
    e.mem.set_u32(me.wrapping_add(0x8c), centre_last);
    e.mem.set_u32(me.wrapping_add(0x9c), segments);
    e.mem
        .set_u32(me.wrapping_add(0x98), x87_quiet_bits(radius.to_bits()));
    this
}

// Translated from 00de4a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySemiCircle::hkDisplaySemiCircle` (default constructor; class confirmed by the
/// RTTI of the vtable it stores), thiscall without words: the identity rotation block, the
/// vtable (`010d8274`), `0.0` at `+0x90`, `+0x94` and `+0x98`, zero at `+0x9c` and zeroed
/// vectors at `+0x60`, `+0x70` and `+0x80`. Returns `this`.
pub fn fn_00de4a80(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    start_block(e, me);
    e.mem.set_u32(me, VTABLE);
    e.mem.set_f32(me.wrapping_add(0x90), 0.0);
    e.mem.set_u32(me.wrapping_add(0x9c), 0);
    e.mem.set_f32(me.wrapping_add(0x94), 0.0);
    store_vector(e, me.wrapping_add(0x60), [0; 4]);
    e.mem.set_f32(me.wrapping_add(0x98), 0.0);
    store_vector(e, me.wrapping_add(0x80), [0; 4]);
    store_vector(e, me.wrapping_add(0x70), [0; 4]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_48a0, fn_00de48a0(Ptr, Ptr) -> Ptr),
        entry!(0x00de_48c0, fn_00de48c0(Ptr, Ptr) -> Ptr),
        entry!(0x00de_48e0, fn_00de48e0(Ptr) -> f32),
        entry!(0x00de_48f0, fn_00de48f0(Ptr) -> f32),
        entry!(0x00de_4900, fn_00de4900(Ptr) -> f32),
        entry!(0x00de_4910, fn_00de4910(Ptr) -> i32),
        entry!(0x00de_4920, fn_00de4920(Ptr, Ptr) -> Ptr),
        entry!(
            0x00de_4950,
            fn_00de4950(Ptr, f32, f32, f32, u32, Ptr, Ptr, Ptr)
        ),
        entry!(0x00de_49b0, fn_00de49b0(Ptr, Ptr, Ptr, Ptr, f32, f32, f32, u32) -> Ptr),
        entry!(0x00de_4a80, fn_00de4a80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A semi-circle block with distinct words at `+0x60 ..= +0x9c`.
    fn object(e: &mut Engine) -> u32 {
        let me = e.mem.alloc(0xa0);
        for word in 0x60 / 4..=0x9c / 4 {
            e.mem.set_u32(me + 4 * word, 0x1000 + word);
        }
        me
    }

    fn check_vector_getter(address: u32, offset: u32) {
        let mut e = Engine::new();
        let me = object(&mut e);
        let out = e.mem.alloc(0x10);
        let result = e.call(address, &args![me, out]);
        assert_eq!(result.u32(), out);
        for word in 0..4 {
            assert_eq!(e.mem.u32(out + 4 * word), 0x1000 + (offset / 4) + word);
        }
    }

    #[test]
    fn the_axis_getter_copies_the_words_at_0x60() {
        check_vector_getter(0x00de_48a0, 0x60);
    }

    #[test]
    fn the_direction_getter_copies_the_words_at_0x70() {
        check_vector_getter(0x00de_48c0, 0x70);
    }

    #[test]
    fn the_centre_getter_copies_the_words_at_0x80() {
        check_vector_getter(0x00de_4920, 0x80);
    }

    fn check_angle_getter(address: u32, offset: u32) {
        let mut e = Engine::new();
        let me = object(&mut e);
        e.mem.set_f32(me + offset, 2.5);
        assert_eq!(e.call(address, &args![me]).f32(), 2.5);
        e.mem.set_u32(me + offset, 0x7f80_0001);
        let quiet = e.call(address, &args![me]).f64();
        assert!(quiet.is_nan());
        assert_eq!((quiet as f32).to_bits() & 0x0040_0000, 0x0040_0000);
    }

    #[test]
    fn the_start_angle_getter_loads_the_float_at_0x90() {
        check_angle_getter(0x00de_48e0, 0x90);
    }

    #[test]
    fn the_end_angle_getter_loads_the_float_at_0x94() {
        check_angle_getter(0x00de_48f0, 0x94);
    }

    #[test]
    fn the_radius_getter_loads_the_float_at_0x98() {
        check_angle_getter(0x00de_4900, 0x98);
    }

    #[test]
    fn the_segment_getter_answers_the_integer_at_0x9c() {
        let mut e = Engine::new();
        let me = object(&mut e);
        e.mem.set_i32(me + 0x9c, -7);
        assert_eq!(e.call(0x00de_4910, &args![me]).i32(), -7);
    }

    /// Three vectors with distinct words.
    fn vectors(e: &mut Engine) -> (u32, u32, u32) {
        let make = |e: &mut Engine, base: u32| {
            let at = e.mem.alloc(0x10);
            for word in 0..4 {
                e.mem.set_u32(at + 4 * word, base + word);
            }
            at
        };
        (make(e, 0x10), make(e, 0x20), make(e, 0x30))
    }

    #[test]
    fn the_setter_copies_the_vectors_and_the_parameters() {
        let mut e = Engine::new();
        let me = object(&mut e);
        let (centre, axis, direction) = vectors(&mut e);
        e.call(
            0x00de_4950,
            &args![me, 1.5f32, 0.25f32, 3.0f32, 12u32, centre, axis, direction],
        );
        for word in 0..4 {
            assert_eq!(e.mem.u32(me + 0x80 + 4 * word), 0x10 + word);
            assert_eq!(e.mem.u32(me + 0x60 + 4 * word), 0x20 + word);
            assert_eq!(e.mem.u32(me + 0x70 + 4 * word), 0x30 + word);
        }
        assert_eq!(e.mem.f32(me + 0x98), 1.5);
        assert_eq!(e.mem.f32(me + 0x90), 0.25);
        assert_eq!(e.mem.f32(me + 0x94), 3.0);
        assert_eq!(e.mem.u32(me + 0x9c), 12);
    }

    #[test]
    fn the_setter_quiets_signalling_angles() {
        let mut e = Engine::new();
        let me = object(&mut e);
        let (centre, axis, direction) = vectors(&mut e);
        e.call(
            0x00de_4950,
            &args![
                me,
                f32::from_bits(0x7f80_0003),
                f32::from_bits(0xff80_0001),
                1.0f32,
                0u32,
                centre,
                axis,
                direction
            ],
        );
        assert_eq!(e.mem.u32(me + 0x98), 0x7fc0_0003);
        assert_eq!(e.mem.u32(me + 0x90), 0xffc0_0001);
    }

    #[test]
    fn the_parameter_constructor_builds_the_block_and_copies_the_vectors() {
        let mut e = Engine::new();
        let me = e.mem.alloc(0xa0);
        for word in 0..0xa0 / 4 {
            e.mem.set_u32(me + 4 * word, 0xdead_0000 + word);
        }
        let (centre, axis, direction) = vectors(&mut e);
        let result = e.call(
            0x00de_49b0,
            &args![me, centre, axis, direction, 0.5f32, 1.5f32, 2.5f32, 9u32],
        );
        assert_eq!(result.u32(), me);
        assert_eq!(e.mem.u32(me), 0x010d_8274);
        assert_eq!(e.mem.u16(me + 6), 1);
        assert_eq!(e.mem.u32(me + 8), 0);
        assert_eq!(e.mem.u32(me + 0x50), 5);
        let rotation: Vec<u32> = (0x10 / 4..0x40 / 4)
            .map(|w| e.mem.u32(me + 4 * w))
            .collect();
        let want: Vec<u32> = (0..12)
            .map(|w| if w % 5 == 0 { 0x3f80_0000 } else { 0 })
            .collect();
        assert_eq!(rotation, want);
        assert_eq!((0..4).map(|w| e.mem.u32(me + 0x40 + 4 * w)).sum::<u32>(), 0);
        for word in 0..4 {
            assert_eq!(e.mem.u32(me + 0x60 + 4 * word), 0x20 + word);
            assert_eq!(e.mem.u32(me + 0x70 + 4 * word), 0x30 + word);
            assert_eq!(e.mem.u32(me + 0x80 + 4 * word), 0x10 + word);
        }
        assert_eq!(e.mem.f32(me + 0x90), 0.5);
        assert_eq!(e.mem.f32(me + 0x94), 1.5);
        assert_eq!(e.mem.f32(me + 0x98), 2.5);
        assert_eq!(e.mem.u32(me + 0x9c), 9);
        // the words the constructor does not touch keep their value
        assert_eq!(e.mem.u32(me + 4), 0x0001_0001);
        assert_eq!(e.mem.u32(me + 0xc), 0xdead_0003);
    }

    #[test]
    fn the_default_constructor_zeroes_the_parameters() {
        let mut e = Engine::new();
        let me = e.mem.alloc(0xa0);
        for word in 0..0xa0 / 4 {
            e.mem.set_u32(me + 4 * word, 0xdead_0000 + word);
        }
        let result = e.call(0x00de_4a80, &args![me]);
        assert_eq!(result.u32(), me);
        assert_eq!(e.mem.u32(me), 0x010d_8274);
        assert_eq!(e.mem.u16(me + 6), 1);
        assert_eq!(e.mem.u32(me + 0x50), 5);
        assert_eq!(e.mem.f32(me + 0x10), 1.0);
        assert_eq!(e.mem.f32(me + 0x24), 1.0);
        assert_eq!(e.mem.f32(me + 0x38), 1.0);
        for offset in [
            0x14, 0x40, 0x60, 0x6c, 0x70, 0x7c, 0x80, 0x8c, 0x90, 0x94, 0x98, 0x9c,
        ] {
            assert_eq!(e.mem.u32(me + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(me + 4), 0x0001_0001);
    }
}
