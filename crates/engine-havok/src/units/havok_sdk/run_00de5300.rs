//! `Havok SDK/run_00de5300` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplayCone` (class confirmed by the RTTI of its vtable `010d82a4`; the field roles
//! follow `hkdisplaycone.rs`, which describes the same object): the geometry block pointer
//! at `+8`, three rotation rows at `+0x10`, `+0x20`, `+0x30`, a vector at `+0x40`, the point
//! the sides fan out from at `+0x60`, the direction at `+0x70`, the number of sides (`int`)
//! at `+0x80`, a float angle at `+0x84` and a float scale at `+0x88`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkDisplayCone` (RTTI).
const VTABLE: u32 = 0x010d_82a4;

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

/// What both constructors begin with: the word at `+6` is 1, `+0x50` is 4, `+8` is 0, the
/// three rows at `+0x10`, `+0x20`, `+0x30` are zero and then get 1.0 on the diagonal (`FST` of
/// `1.0` into `+0x10`, `+0x24`, `+0x38`), and the vector at `+0x40` is zero.
fn start_block(e: &mut Engine, me: u32) {
    e.mem.set_u16(me.wrapping_add(6), 1);
    e.mem.set_u32(me.wrapping_add(0x50), 4);
    e.mem.set_u32(me.wrapping_add(8), 0);
    for row in [0x10, 0x20, 0x30] {
        store_vector(e, me.wrapping_add(row), [0; 4]);
    }
    e.mem.set_f32(me.wrapping_add(0x10), 1.0);
    e.mem.set_f32(me.wrapping_add(0x24), 1.0);
    e.mem.set_f32(me.wrapping_add(0x38), 1.0);
    store_vector(e, me.wrapping_add(0x40), [0; 4]);
}

// Translated from 00de5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the point the sides fan out from: copies the four words at `this + 0x60` to `out`
/// (thiscall, `RET 4`) and returns `out`.
pub fn fn_00de5300(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_vector_out(e, this.addr().wrapping_add(0x60), out.addr())
}

// Translated from 00de5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the direction: copies the four words at `this + 0x70` to `out` (thiscall,
/// `RET 4`) and returns `out`.
pub fn fn_00de5320(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_vector_out(e, this.addr().wrapping_add(0x70), out.addr())
}

// Translated from 00de5340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the scale: `FLD` of the float at `this + 0x88`, the result in `ST0` (a
/// signalling NaN comes out quiet).
pub fn fn_00de5340(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x88)))
}

// Translated from 00de5350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the angle: `FLD` of the float at `this + 0x84`, the result in `ST0`.
pub fn fn_00de5350(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x84)))
}

// Translated from 00de5360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter of the number of sides: the integer at `this + 0x80` in `EAX`.
pub fn fn_00de5360(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr().wrapping_add(0x80))
}

// Translated from 00de5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter of all parameters (the engine map has no name), thiscall with five words `(angle,
/// scale, sides, direction, point)`: the floats pass through the x87 stack (`FLD`/`FSTP`: a
/// signalling NaN is stored quiet) to `+0x84` and `+0x88`, the number of sides goes to
/// `+0x80`, the vectors are copied with `MOVAPS` from the pointers to `+0x70` (direction)
/// and `+0x60` (point). The order of reads and writes of the exe is kept.
pub fn fn_00de5370(
    e: &mut Engine,
    this: Ptr,
    angle: f32,
    scale: f32,
    sides: u32,
    direction: Ptr,
    point: Ptr,
) {
    let me = this.addr();
    e.mem
        .set_u32(me.wrapping_add(0x84), x87_quiet_bits(angle.to_bits()));
    let point_vector = load_vector(e, point.addr());
    store_vector(e, me.wrapping_add(0x60), point_vector);
    let direction_vector = load_vector(e, direction.addr());
    e.mem
        .set_u32(me.wrapping_add(0x88), x87_quiet_bits(scale.to_bits()));
    store_vector(e, me.wrapping_add(0x70), direction_vector);
    e.mem.set_u32(me.wrapping_add(0x80), sides);
}

// Translated from 00de53b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCone::hkDisplayCone` (constructor with parameters; class confirmed by the RTTI of
/// the vtable it stores), thiscall with five words `(angle, scale, sides, direction, point)`:
/// sets the identity rotation block, the vtable (`010d82a4`), copies the point to `+0x60` and
/// the direction to `+0x70` as plain words (the last word of the direction is stored after the
/// angle), the floats pass through the x87 stack (`+0x84`, `+0x88`) and the number of sides
/// goes to `+0x80`. Returns `this`.
pub fn fn_00de53b0(
    e: &mut Engine,
    this: Ptr,
    angle: f32,
    scale: f32,
    sides: u32,
    direction: Ptr,
    point: Ptr,
) -> Ptr {
    let me = this.addr();
    start_block(e, me);
    e.mem.set_u32(me, VTABLE);
    let point_vector = load_vector(e, point.addr());
    store_vector(e, me.wrapping_add(0x60), point_vector);
    for word in 0..3 {
        let value = e.mem.u32(direction.addr().wrapping_add(4 * word));
        e.mem.set_u32(me.wrapping_add(0x70 + 4 * word), value);
    }
    let direction_last = e.mem.u32(direction.addr().wrapping_add(0xc));
    e.mem
        .set_u32(me.wrapping_add(0x84), x87_quiet_bits(angle.to_bits()));
    e.mem
        .set_u32(me.wrapping_add(0x88), x87_quiet_bits(scale.to_bits()));
    e.mem.set_u32(me.wrapping_add(0x7c), direction_last);
    e.mem.set_u32(me.wrapping_add(0x80), sides);
    this
}

// Translated from 00de5450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCone::hkDisplayCone` (default constructor; class confirmed by the RTTI of the
/// vtable it stores), thiscall without words: the identity rotation block, the vtable
/// (`010d82a4`), `0.0` at `+0x84` and `+0x88`, zero at `+0x80` and zeroed vectors at `+0x60`
/// and `+0x70`. Returns `this`.
pub fn fn_00de5450(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    start_block(e, me);
    e.mem.set_u32(me, VTABLE);
    e.mem.set_f32(me.wrapping_add(0x84), 0.0);
    e.mem.set_u32(me.wrapping_add(0x80), 0);
    e.mem.set_f32(me.wrapping_add(0x88), 0.0);
    store_vector(e, me.wrapping_add(0x60), [0; 4]);
    store_vector(e, me.wrapping_add(0x70), [0; 4]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_5300, fn_00de5300(Ptr, Ptr) -> Ptr),
        entry!(0x00de_5320, fn_00de5320(Ptr, Ptr) -> Ptr),
        entry!(0x00de_5340, fn_00de5340(Ptr) -> f32),
        entry!(0x00de_5350, fn_00de5350(Ptr) -> f32),
        entry!(0x00de_5360, fn_00de5360(Ptr) -> i32),
        entry!(0x00de_5370, fn_00de5370(Ptr, f32, f32, u32, Ptr, Ptr)),
        entry!(0x00de_53b0, fn_00de53b0(Ptr, f32, f32, u32, Ptr, Ptr) -> Ptr),
        entry!(0x00de_5450, fn_00de5450(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A cone block with distinct words at `+0x60 ..= +0x88`.
    fn object(e: &mut Engine) -> u32 {
        let me = e.mem.alloc(0x90);
        for word in 0x60 / 4..=0x88 / 4 {
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
    fn the_point_getter_copies_the_words_at_0x60() {
        check_vector_getter(0x00de_5300, 0x60);
    }

    #[test]
    fn the_direction_getter_copies_the_words_at_0x70() {
        check_vector_getter(0x00de_5320, 0x70);
    }

    fn check_float_getter(address: u32, offset: u32) {
        let mut e = Engine::new();
        let me = object(&mut e);
        e.mem.set_f32(me + offset, -4.5);
        assert_eq!(e.call(address, &args![me]).f32(), -4.5);
        e.mem.set_u32(me + offset, 0xff80_0001);
        let quiet = e.call(address, &args![me]).f64();
        assert!(quiet.is_nan());
        assert_eq!((quiet as f32).to_bits() & 0x0040_0000, 0x0040_0000);
    }

    #[test]
    fn the_scale_getter_loads_the_float_at_0x88() {
        check_float_getter(0x00de_5340, 0x88);
    }

    #[test]
    fn the_angle_getter_loads_the_float_at_0x84() {
        check_float_getter(0x00de_5350, 0x84);
    }

    #[test]
    fn the_side_getter_answers_the_integer_at_0x80() {
        let mut e = Engine::new();
        let me = object(&mut e);
        e.mem.set_i32(me + 0x80, -3);
        assert_eq!(e.call(0x00de_5360, &args![me]).i32(), -3);
    }

    /// Two vectors with distinct words.
    fn vectors(e: &mut Engine) -> (u32, u32) {
        let make = |e: &mut Engine, base: u32| {
            let at = e.mem.alloc(0x10);
            for word in 0..4 {
                e.mem.set_u32(at + 4 * word, base + word);
            }
            at
        };
        (make(e, 0x10), make(e, 0x20))
    }

    #[test]
    fn the_setter_copies_the_vectors_and_the_parameters() {
        let mut e = Engine::new();
        let me = object(&mut e);
        let (direction, point) = vectors(&mut e);
        e.call(
            0x00de_5370,
            &args![me, 0.75f32, 1.25f32, 16u32, direction, point],
        );
        for word in 0..4 {
            assert_eq!(e.mem.u32(me + 0x70 + 4 * word), 0x10 + word);
            assert_eq!(e.mem.u32(me + 0x60 + 4 * word), 0x20 + word);
        }
        assert_eq!(e.mem.f32(me + 0x84), 0.75);
        assert_eq!(e.mem.f32(me + 0x88), 1.25);
        assert_eq!(e.mem.u32(me + 0x80), 16);
    }

    #[test]
    fn the_setter_quiets_signalling_floats() {
        let mut e = Engine::new();
        let me = object(&mut e);
        let (direction, point) = vectors(&mut e);
        e.call(
            0x00de_5370,
            &args![
                me,
                f32::from_bits(0x7f80_0003),
                f32::from_bits(0xff80_0001),
                0u32,
                direction,
                point
            ],
        );
        assert_eq!(e.mem.u32(me + 0x84), 0x7fc0_0003);
        assert_eq!(e.mem.u32(me + 0x88), 0xffc0_0001);
    }

    #[test]
    fn the_parameter_constructor_builds_the_block_and_copies_the_vectors() {
        let mut e = Engine::new();
        let me = e.mem.alloc(0x90);
        for word in 0..0x90 / 4 {
            e.mem.set_u32(me + 4 * word, 0xdead_0000 + word);
        }
        let (direction, point) = vectors(&mut e);
        let result = e.call(
            0x00de_53b0,
            &args![me, 0.5f32, 1.5f32, 9u32, direction, point],
        );
        assert_eq!(result.u32(), me);
        assert_eq!(e.mem.u32(me), 0x010d_82a4);
        assert_eq!(e.mem.u16(me + 6), 1);
        assert_eq!(e.mem.u32(me + 8), 0);
        assert_eq!(e.mem.u32(me + 0x50), 4);
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
            assert_eq!(e.mem.u32(me + 0x70 + 4 * word), 0x10 + word);
        }
        assert_eq!(e.mem.f32(me + 0x84), 0.5);
        assert_eq!(e.mem.f32(me + 0x88), 1.5);
        assert_eq!(e.mem.u32(me + 0x80), 9);
        // the words the constructor does not touch keep their value
        assert_eq!(e.mem.u32(me + 4), 0x0001_0001);
        assert_eq!(e.mem.u32(me + 0xc), 0xdead_0003);
    }

    #[test]
    fn the_default_constructor_zeroes_the_parameters() {
        let mut e = Engine::new();
        let me = e.mem.alloc(0x90);
        for word in 0..0x90 / 4 {
            e.mem.set_u32(me + 4 * word, 0xdead_0000 + word);
        }
        let result = e.call(0x00de_5450, &args![me]);
        assert_eq!(result.u32(), me);
        assert_eq!(e.mem.u32(me), 0x010d_82a4);
        assert_eq!(e.mem.u16(me + 6), 1);
        assert_eq!(e.mem.u32(me + 0x50), 4);
        assert_eq!(e.mem.f32(me + 0x10), 1.0);
        assert_eq!(e.mem.f32(me + 0x24), 1.0);
        assert_eq!(e.mem.f32(me + 0x38), 1.0);
        for offset in [0x14, 0x40, 0x60, 0x6c, 0x70, 0x7c, 0x80, 0x84, 0x88] {
            assert_eq!(e.mem.u32(me + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(me + 4), 0x0001_0001);
    }
}
