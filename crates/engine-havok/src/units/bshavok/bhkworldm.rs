//! `BSHavok/bhkworldm.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkWorldM` (Xbox PDB) is a `bhkWorld` that also keeps three 16-byte vectors: `+0x60` (the
//! centre the position is measured from, a field of the base class), `+0xA0` and `+0xB0` (the
//! two vectors the constructor copies in) and `+0xC0` (copied from the construction info's
//! `+0x40`). The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding
//! frame or a `/GS` cookie check (`00ec408c`), neither of which is modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    enter_scope, push_viewer_string, MEMORY_MANAGER, NI_GET_VIEWER_STRING,
    NI_POINT3_GET_VIEWER_STRING,
};

/// The class vtable the constructor and the destructor store.
const WORLD_M_VTABLE: u32 = 0x010c_69f4;
/// `bhkWorld` constructor (`00c69010`, `(this, info)`), destructor body (`00c6aca0`) and
/// `GetViewerStrings` (`00c69240`, `(this, strings)`).
const WORLD_CONSTRUCT: u32 = 0x00c6_9010;
const WORLD_DESTRUCT: u32 = 0x00c6_aca0;
const WORLD_GET_VIEWER_STRINGS: u32 = 0x00c6_9240;
/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the manager, `(pointer)`).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// Handles the position that `fn_00c9a1f0` has wrapped (`00c66c00`, `(this, position)`).
const WORLD_APPLY_POSITION: u32 = 0x00c6_6c00;
/// The 16-byte mask `ANDPS` clears the sign bits with and the float the lanes are compared with.
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
const POSITION_TOLERANCE: u32 = 0x0101_6248;
/// The word holding the class name `GetViewerStrings` lists first, and the two names of the
/// vectors it lists.
const CLASS_NAME_POINTER: u32 = 0x0126_81e8;
const NAME_FIRST_VECTOR: u32 = 0x010c_6adc;
const NAME_SECOND_VECTOR: u32 = 0x010c_6ad0;
/// Copies a Havok vector into a `NiPoint3` (`00458620`, cdecl `(point, vector)`).
const VECTOR_TO_POINT: u32 = 0x0045_8620;

/// Copies the 16 bytes at `from` to `to` (`MOVAPS` load and store).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00c99fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldM::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_81e8.
pub fn bhk_world_m_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_81e8
}

// Translated from 00c99ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldM` constructor (no PDB name): runs the `bhkWorld` constructor (`00c69010`) on
/// `(this, info)`, stores the class vtable, copies the 16 bytes at `first` to `+0xA0`, the 16
/// bytes at `second` to `+0xB0` and the 16 bytes at `info + 0x40` to `+0xC0`. Answers `this`.
pub fn fn_00c99ff0(e: &mut Engine, this: Ptr, info: u32, first: u32, second: u32) -> Ptr {
    e.call(WORLD_CONSTRUCT, &args![this, info]);
    e.mem.set_u32(this.addr(), WORLD_M_VTABLE);
    copy_vector(e, first, this.addr() + 0xa0);
    copy_vector(e, second, this.addr() + 0xb0);
    copy_vector(e, info + 0x40, this.addr() + 0xc0);
    this
}

// Translated from 00c9a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldM::_scalar_deleting_destructor_` (Xbox PDB): stores the class vtable, runs the
/// `bhkWorld` destructor body (`00c6aca0`) and, when bit 0 of `flags` is set, frees the block
/// the object was allocated in: the byte before the object is the distance back to the
/// block, handed to `MemoryManager::Deallocate` (`00aa4060`). Answers `this`.
pub fn bhk_world_m_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), WORLD_M_VTABLE);
    e.call(WORLD_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        let distance = u32::from(e.mem.u8(this.addr().wrapping_sub(1)));
        let block = this.addr().wrapping_sub(distance);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    this
}

// Translated from 00c9a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, one argument): adjusts the three floats at `vector` one after the
/// other by the span between the world's `+0xC0` and `+0xA0` vectors. For each lane, with
/// `x` the float (through the x87 stack), `sign` = -1 when `x < 0` and 1 otherwise (a NaN
/// gives 1), `span` = `[+0xC0 + 4 * lane] - [+0xA0 + 4 * lane]` and `u` = `x + sign * span`
/// (stored as a float): when `sign * u > 0` the float becomes `x - u`. All the arithmetic is
/// in `f64`, rounded to `f32` at each store. What the world means by it is not confirmed.
pub fn fn_00c9a070(e: &mut Engine, this: Ptr, vector: u32) {
    for lane in 0..3 {
        let address = vector + 4 * lane;
        let value = x87_quiet(e.mem.f32(address));
        let high = f64::from(x87_quiet(e.mem.f32(this.addr() + 0xc0 + 4 * lane)));
        let low = f64::from(x87_quiet(e.mem.f32(this.addr() + 0xa0 + 4 * lane)));
        let sign: f32 = if value < 0.0 { -1.0 } else { 1.0 };
        let span = (high - low) * f64::from(sign);
        let moved = (f64::from(value) + span) as f32;
        if 0.0 < f64::from(sign) * f64::from(moved) {
            let current = f64::from(e.mem.f32(address));
            e.mem.set_f32(address, (current - f64::from(moved)) as f32);
        }
    }
}

// Translated from 00c9a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9a1f0 - the /GS cookie check (00ec408c) is not modeled.
/// Unnamed (`thiscall`, one argument, answers a `bool` in `AL`): takes the 16-byte vector at
/// `position`, subtracts the world's vector at `+0x60` and compares the absolute value of
/// each of the first three lanes with the lane of the vector at `+0xB0` (x87 compare after a
/// float round trip, so a signalling NaN is quieted; a NaN never counts as outside). When
/// no lane has `bound <= |difference|` the answer is false. Otherwise a local copy of the
/// vector is wrapped by `fn_00c9a070`, the local copy minus the `+0x60` vector gets the sign
/// bits cleared (mask at `01017d10`) and when any of the three lanes is not `<=` the
/// tolerance float at `01016248` (a NaN is not) the world's `00c66c00` is run on `(this,
/// local copy)` and the answer is true; otherwise false.
pub fn fn_00c9a1f0(e: &mut Engine, this: Ptr, position: u32) -> bool {
    let base: [f32; 4] =
        std::array::from_fn(|lane| e.mem.f32(this.addr() + 0x60 + 4 * lane as u32));
    let given: [f32; 4] = std::array::from_fn(|lane| e.mem.f32(position + 4 * lane as u32));
    let difference: [f32; 4] = std::array::from_fn(|lane| given[lane] - base[lane]);
    let outside = (0..3).any(|lane| {
        let magnitude = f32::from_bits(x87_quiet_bits(difference[lane].to_bits()) & 0x7fff_ffff);
        let bound = x87_quiet(e.mem.f32(this.addr() + 0xb0 + 4 * lane as u32));
        bound <= magnitude
    });
    if !outside {
        return false;
    }
    e.with_stack(16, |e, local| {
        copy_vector(e, position, local.addr());
        fn_00c9a070(e, this, local.addr());
        let tolerance = e.mem.f32(POSITION_TOLERANCE);
        let farther = (0..3).any(|lane| {
            let moved = e.mem.f32(local.addr() + 4 * lane) - base[lane as usize];
            let magnitude = f32::from_bits(moved.to_bits() & e.mem.u32(ABSOLUTE_MASK + 4 * lane));
            !(magnitude <= tolerance)
        });
        if farther {
            e.call(WORLD_APPLY_POSITION, &args![this, local]);
        }
        farther
    })
}

// Translated from 00c9a2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9a2c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWorldM::GetViewerStrings` (Xbox PDB), with the allocation scope word set to 0x10: runs
/// `bhkWorld::GetViewerStrings` (`00c69240`) on `(this, strings)`, then appends to the array
/// the string `NiGetViewerString` makes of the word at `012681e8` and the two `NiPoint3`
/// strings of the vectors at `+0xA0` (name `010c6adc`) and `+0xB0` (name `010c6ad0`), each
/// vector copied into a temporary point by `00458620`.
pub fn bhk_world_m_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(WORLD_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.with_stack(12, |e, point| {
        for (offset, name) in [(0xa0, NAME_FIRST_VECTOR), (0xb0, NAME_SECOND_VECTOR)] {
            e.call(VECTOR_TO_POINT, &args![point, this.addr() + offset]);
            let text = e
                .call(NI_POINT3_GET_VIEWER_STRING, &args![point, name])
                .u32();
            push_viewer_string(e, strings, text);
        }
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c99fe0, bhk_world_m_get_r_t_t_i() -> u32),
        entry!(0x00c99ff0, fn_00c99ff0(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00c9a040,
            bhk_world_m_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c9a070, fn_00c9a070(Ptr, u32)),
        entry!(0x00c9a1f0, fn_00c9a1f0(Ptr, u32) -> bool),
        entry!(0x00c9a2c0, bhk_world_m_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::SCOPE_WORD;
    use super::*;

    #[test]
    fn bhk_world_m_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_9fe0, &args![]).u32(), 0x0126_81e8);
    }

    #[test]
    fn constructor_copies_the_three_vectors_after_the_world_constructor() {
        let mut e = logging_engine();
        e.register(WORLD_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(0x60);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_u32(info + 0x40 + 4 * lane, 0x300 + lane);
            e.mem.set_u32(first + 4 * lane, 0x100 + lane);
            e.mem.set_u32(second + 4 * lane, 0x200 + lane);
        }
        let result = e.call(0x00c9_9ff0, &args![this, info, first, second]).u32();
        assert_eq!(result, this);
        assert_eq!(calls_to(&e, WORLD_CONSTRUCT), vec![vec![this, info]]);
        assert_eq!(e.mem.u32(this), WORLD_M_VTABLE);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(this + 0xa0 + 4 * lane), 0x100 + lane);
            assert_eq!(e.mem.u32(this + 0xb0 + 4 * lane), 0x200 + lane);
            assert_eq!(e.mem.u32(this + 0xc0 + 4 * lane), 0x300 + lane);
        }
    }

    #[test]
    fn deleting_destructor_frees_the_padded_block_only_on_bit_0() {
        let mut e = logging_engine();
        e.register(WORLD_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let block = e.mem.alloc(0x100);
        let this = block + 5;
        e.mem.set_u8(this - 1, 5);
        assert_eq!(e.call(0x00c9_a040, &args![this, 0xfeu32]).u32(), this);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this), WORLD_M_VTABLE);
        assert_eq!(e.call(0x00c9_a040, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls_to(&e, WORLD_DESTRUCT), vec![vec![this]; 2]);
    }

    /// A world with `+0xA0` = (0, 0, 0), `+0xC0` = the given span per lane and a vector
    /// of three floats.
    fn wrap_setup(span: [f32; 3], vector: [f32; 3]) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        for lane in 0..3 {
            e.mem.set_f32(this + 0xc0 + 4 * lane as u32, span[lane]);
        }
        let position = e.mem.alloc(0x10);
        for lane in 0..3 {
            e.mem.set_f32(position + 4 * lane as u32, vector[lane]);
        }
        (e, this, position)
    }

    fn read3(e: &Engine, address: u32) -> [f32; 3] {
        std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
    }

    #[test]
    fn wrap_moves_a_lane_by_minus_sign_times_the_span() {
        let (mut e, this, position) = wrap_setup([8.0, 8.0, 8.0], [3.0, -3.0, 0.0]);
        e.call(0x00c9_a070, &args![this, position]);
        // x = 3: sign 1, u = 11, 1 * 11 > 0 so 3 - 11 = -8; x = -3: sign -1, u = -11, so
        // -3 - (-11) = 8; 0: sign 1, u = 8, 0 - 8 = -8.
        assert_eq!(read3(&e, position), [-8.0, 8.0, -8.0]);
    }

    #[test]
    fn wrap_leaves_a_lane_alone_when_the_span_is_not_positive_or_a_nan() {
        let (mut e, this, position) = wrap_setup([-8.0, 0.0, 8.0], [3.0, 0.0, f32::NAN]);
        e.call(0x00c9_a070, &args![this, position]);
        // Span -8: u = -5, sign * u < 0. Span 0 and x = 0: u = 0, not > 0. NaN stays NaN.
        let after = read3(&e, position);
        assert_eq!(after[0], 3.0);
        assert_eq!(after[1], 0.0);
        assert!(after[2].is_nan());
    }

    /// Maps the mask and the tolerance (0.5) the way the exe's data holds them.
    fn map_constants(e: &mut Engine) {
        e.map(ABSOLUTE_MASK, 16);
        for lane in 0..4 {
            e.mem.set_u32(ABSOLUTE_MASK + 4 * lane, 0x7fff_ffff);
        }
        e.map(POSITION_TOLERANCE, 4);
        e.mem.set_f32(POSITION_TOLERANCE, 0.5);
    }

    /// What the double for `00c66c00` saw: the three floats of the vector it was given.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<[f32; 3]>>>;

    /// A world centred on `centre` (`+0x60`) with half extents 5 (`+0xB0`) and the wrap span
    /// `span` (`+0xC0`); `00c66c00` is a double that records the vector it gets.
    fn bounds_setup(centre: f32, span: f32, vector: [f32; 3]) -> (Engine, u32, u32, Seen) {
        let mut e = logging_engine();
        map_constants(&mut e);
        let seen: Seen = Default::default();
        let log = seen.clone();
        e.register_double(WORLD_APPLY_POSITION, move |e, a| {
            log.borrow_mut().push(read3(e, a[1]));
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        for lane in 0..3 {
            e.mem.set_f32(this + 0x60 + 4 * lane, centre);
            e.mem.set_f32(this + 0xb0 + 4 * lane, 5.0);
            e.mem.set_f32(this + 0xc0 + 4 * lane, span);
        }
        let position = e.mem.alloc(0x10);
        for lane in 0..3 {
            e.mem.set_f32(position + 4 * lane, vector[lane as usize]);
        }
        (e, this, position, seen)
    }

    #[test]
    fn bounds_inside_the_extents_answer_false_without_calls() {
        let (mut e, this, position, seen) = bounds_setup(1.0, 10.0, [5.9, -3.9, 1.0]);
        assert!(!e.call(0x00c9_a1f0, &args![this, position]).bool());
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn bounds_outside_run_the_world_routine_on_the_wrapped_copy() {
        let (mut e, this, position, seen) = bounds_setup(1.0, 10.0, [7.0, 1.0, 1.0]);
        assert!(e.call(0x00c9_a1f0, &args![this, position]).bool());
        // Each lane is wrapped by minus the span (7 - 17, 1 - 11, 1 - 11); the caller's
        // vector is untouched.
        assert_eq!(*seen.borrow(), vec![[-10.0, -10.0, -10.0]]);
        assert_eq!(calls_to(&e, WORLD_APPLY_POSITION)[0][0], this);
        assert_eq!(read3(&e, position), [7.0, 1.0, 1.0]);
    }

    #[test]
    fn bounds_outside_that_wrap_back_to_the_centre_answer_false() {
        // Span 0 moves a lane that is not 0 to 0; with the centre at the origin the wrapped
        // copy equals the centre in every lane.
        let (mut e, this, position, seen) = bounds_setup(0.0, 0.0, [7.0, 0.0, 0.0]);
        assert!(!e.call(0x00c9_a1f0, &args![this, position]).bool());
        assert!(seen.borrow().is_empty());
        // With the centre at (1, 1, 1) the wrapped copy is farther than the tolerance.
        let (mut e, this, position, seen) = bounds_setup(1.0, 0.0, [7.0, 1.0, 1.0]);
        assert!(e.call(0x00c9_a1f0, &args![this, position]).bool());
        assert_eq!(*seen.borrow(), vec![[0.0, 0.0, 0.0]]);
    }

    #[test]
    fn bounds_with_a_nan_lane_are_not_outside() {
        let (mut e, this, position, seen) = bounds_setup(1.0, 10.0, [f32::NAN, 1.0, 1.0]);
        assert!(!e.call(0x00c9_a1f0, &args![this, position]).bool());
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn viewer_strings_list_the_class_and_the_two_vectors() {
        let mut e = logging_engine();
        viewer_doubles(&mut e);
        e.register(WORLD_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VECTOR_TO_POINT, |_, _| Ret::default());
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, a| Ret {
            eax: 0x8000 + a[1],
            ..Ret::default()
        });
        e.map(CLASS_NAME_POINTER, 4);
        e.mem.set_u32(CLASS_NAME_POINTER, 0x1234);
        let this = e.mem.alloc(0x100);
        let strings = strings_header(&mut e, 0, 8, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c9_a2c0, &args![this, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, WORLD_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x1234]]);
        let vectors: Vec<u32> = calls_to(&e, VECTOR_TO_POINT).iter().map(|c| c[1]).collect();
        assert_eq!(vectors, vec![this + 0xa0, this + 0xb0]);
        assert_eq!(
            pushed_texts(&e, strings, 0, 3),
            vec![
                0x7000 + 0x1234,
                0x8000 + NAME_FIRST_VECTOR,
                0x8000 + NAME_SECOND_VECTOR
            ]
        );
    }
}
