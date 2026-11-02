//! `SpeedTree/idvrandom.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! IDV's random-number front end for SpeedTree: `CIdvRandom` seeds the shared
//! `Uniform` generator (`011f8ebc`, `robertdavies_random.cpp`) and draws
//! uniform floats from it, and the base `Random` and `Uniform` classes' small
//! virtual functions.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_ftol2_sse` (`00ec62c0`): the float in ST0 truncated to an integer. The
/// uniform form takes it as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `__time64(0)` (LIBCMT, `cdecl`; the code keeps the low word of the
/// 64-bit result).
const TIME64: u32 = 0x00ec_adda;
/// `operator delete(pointer)` (`cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `Random::Next` (`thiscall`, `float` result in ST0).
const RANDOM_NEXT: u32 = 0x00b4_33a0;
/// `Random::Set(double seed)` (`cdecl`).
const RANDOM_SET: u32 = 0x00b4_3510;
/// `Random::SetLong(int seed)` (`cdecl`).
const RANDOM_SET_LONG: u32 = 0x00b4_35f0;

/// Byte flag: the shared generator has been seeded.
const SEEDED: u32 = 0x011f_8eb8;
/// The shared generator object `CIdvRandom::GetUniform` draws from.
const SHARED_RANDOM: u32 = 0x011f_8ebc;
/// `2147483648.0f`: the upper end of the range `Reseed` draws its time
/// seed from.
const SEED_RANGE_END: u32 = 0x0107_dd24;
/// `0.5f`: `Random::Mean`.
const MEAN: u32 = 0x0101_6248;
/// `1/12 = 0.0833333f` (`3daaaaab`): `Random::Variance`.
const VARIANCE: u32 = 0x010a_58ec;
/// `0.0` (`double`): `Uniform::Density`'s lower bound.
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`): `Uniform::Density`'s upper bound.
const ONE: u32 = 0x0101_2070;
/// The `Random` vtable (RTTI `.?AVRandom@@`).
const RANDOM_VTABLE: u32 = 0x010a_58c8;

// Translated from 00b3f100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvRandom::CIdvRandom` (Xbox PDB): seeds the shared generator from the
/// clock the first time any `CIdvRandom` is made. Returns `this`.
pub fn cidv_random_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    if e.global::<u8>(SEEDED) == 0 {
        cidv_random_reseed(e, this, -1);
    }
    this
}

// Translated from 00b3f130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvRandom::GetUniform` (Xbox PDB): a uniform `float` in `[from, to)`,
/// `Next() * (to - from) + from` from the shared generator. `this` is not
/// read.
pub fn cidv_random_get_uniform(e: &mut Engine, _this: Ptr, from: f32, to: f32) -> f32 {
    // `(to - from)` is stored as a `double` before `Next` is called.
    let span = to as f64 - from as f64;
    let next = e.call(RANDOM_NEXT, &args![SHARED_RANDOM]).f64();
    (next * span + from as f64) as f32
}

// Translated from 00b3f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvRandom::Reseed` (Xbox PDB): seed -1 seeds from the clock; any other
/// value (at least 1) is the integer seed. For -1, a random number in
/// `[0, 2^31)` from the generator already running (`12345` when it was never
/// seeded) and the clock's low word are divided, smaller by larger, into a
/// seed in (0, 1] for `Random::Set`. Marks the generator seeded.
pub fn cidv_random_reseed(e: &mut Engine, this: Ptr, seed: i32) {
    if seed == -1 {
        let mut previous = 0x3039;
        if e.global::<u8>(SEEDED) != 0 {
            let end = x87_quiet(e.global::<f32>(SEED_RANGE_END));
            let draw = cidv_random_get_uniform(e, this, 0.0, end);
            previous = e.call(FTOL, &args![draw as f64]).i32();
        }
        let now = e.call(TIME64, &args![0u32]).i32();
        if previous > now {
            let ratio = now as f64 / previous as f64;
            e.call(RANDOM_SET, &args![ratio]);
        } else {
            let ratio = previous as f64 / now as f64;
            e.call(RANDOM_SET, &args![ratio]);
        }
    } else {
        let seed = if seed <= 1 { 1 } else { seed };
        e.call(RANDOM_SET_LONG, &args![seed]);
    }
    e.set_global(SEEDED, 1u8);
}

// Translated from 00b3f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::Mean` (Xbox PDB): writes the pair (0.5, 0) to `result` and
/// returns it.
pub fn random_mean(e: &mut Engine, _this: Ptr, result: Ptr) -> Ptr {
    e.mem.set_u32(result.addr() + 4, 0);
    let mean = x87_quiet(e.global::<f32>(MEAN));
    e.mem.set_f32(result.addr(), mean);
    result
}

// Translated from 00b3f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::Variance` (Xbox PDB): writes the pair (1/12, 0) to `result` and
/// returns it.
pub fn random_variance(e: &mut Engine, _this: Ptr, result: Ptr) -> Ptr {
    e.mem.set_u32(result.addr() + 4, 0);
    let variance = x87_quiet(e.global::<f32>(VARIANCE));
    e.mem.set_f32(result.addr(), variance);
    result
}

// Translated from 00b3f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual function that answers 1 (unnamed in the engine map; it sits in
/// the `Random` vtable region of this unit).
pub fn fn_00b3f270(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00b3f280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Uniform::Next` (Xbox PDB): `Random::Next`, whose result stays in ST0.
pub fn uniform_next(e: &mut Engine, this: Ptr) -> f64 {
    e.call(RANDOM_NEXT, &args![this]).f64()
}

// Translated from 00b3f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Uniform::Density` (Xbox PDB): 1.0 on `[0, 1]`, 0.0 outside it. NaN is
/// not outside (both compares fail), so it answers 1.0.
pub fn uniform_density(e: &mut Engine, _this: Ptr, x: f32) -> f32 {
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    if (x as f64) < zero || x as f64 > one {
        0.0
    } else {
        1.0
    }
}

// Translated from 00b3f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::scalar deleting destructor` (Xbox PDB): sets the `Random`
/// vtable, then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn random_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), RANDOM_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b3f100, cidv_random_constructor(Ptr) -> Ptr),
        entry!(0x00b3f130, cidv_random_get_uniform(Ptr, f32, f32) -> f32),
        entry!(0x00b3f160, cidv_random_reseed(Ptr, i32)),
        entry!(0x00b3f210, random_mean(Ptr, Ptr) -> Ptr),
        entry!(0x00b3f240, random_variance(Ptr, Ptr) -> Ptr),
        entry!(0x00b3f270, fn_00b3f270(Ptr) -> u32),
        entry!(0x00b3f280, uniform_next(Ptr) -> f64),
        entry!(0x00b3f2a0, uniform_density(Ptr, f32) -> f32),
        entry!(0x00b3f2e0, random_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the pages the unit reads mapped and doubles for the
    /// generator's callees; `calls` records (address, words) of them.
    fn seeding_engine(
        seeded: bool,
        draw: f64,
        time: i32,
    ) -> (Engine, Rc<RefCell<Vec<(u32, Vec<u32>)>>>) {
        let mut e = Engine::new();
        for page in [
            0x011f_8000,
            0x0107_d000,
            0x0101_6000,
            0x010a_5000,
            0x0101_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(SEEDED, seeded as u8);
        e.set_global(SEED_RANGE_END, 2147483648.0f32);
        e.register(FTOL, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        let calls = Rc::new(RefCell::new(Vec::new()));
        for address in [RANDOM_SET, RANDOM_SET_LONG, RANDOM_NEXT, OPERATOR_DELETE] {
            let record = calls.clone();
            e.register_double(address, move |_, a| {
                record.borrow_mut().push((address, a.to_vec()));
                Ret {
                    st0: draw,
                    ..Ret::default()
                }
            });
        }
        e.register_double(TIME64, move |_, _| Ret {
            eax: time as u32,
            ..Ret::default()
        });
        (e, calls)
    }

    #[test]
    fn constructor_seeds_from_the_clock_only_the_first_time() {
        let (mut e, calls) = seeding_engine(false, 0.0, 1000);
        let this = Ptr::<()>::new(0x1234);
        assert_eq!(e.call(0x00b3_f100, &args![this]).ptr::<()>(), this);
        // Never seeded: previous = 12345 > clock 1000, so 1000 / 12345.
        let log = calls.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, RANDOM_SET);
        let ratio = f64::take(&log[0].1, &mut 0);
        assert_eq!(ratio, 1000.0 / 12345.0);
        drop(log);
        assert_eq!(e.global::<u8>(SEEDED), 1);
        // Seeded now: the constructor leaves the generator alone.
        calls.borrow_mut().clear();
        e.call(0x00b3_f100, &args![this]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn get_uniform_scales_the_next_sample() {
        let (mut e, calls) = seeding_engine(true, 0.25, 0);
        let drawn = e.call(0x00b3_f130, &args![0u32, 2.0f32, 10.0f32]).f32();
        assert_eq!(drawn, 4.0);
        // The shared generator object is `this` of the draw.
        assert_eq!(calls.borrow()[0], (RANDOM_NEXT, vec![SHARED_RANDOM]));
    }

    #[test]
    fn reseed_from_the_clock_divides_smaller_by_larger() {
        // Seeded: previous = Next() * 2^31 = 0.5 * 2^31 = 1073741824, below
        // the clock's 2000000000: previous / clock.
        let (mut e, calls) = seeding_engine(true, 0.5, 2_000_000_000);
        e.call(0x00b3_f160, &args![0u32, -1i32]);
        let log = calls.borrow();
        let set = log.iter().find(|c| c.0 == RANDOM_SET).unwrap();
        assert_eq!(f64::take(&set.1, &mut 0), 1073741824.0 / 2000000000.0);
        drop(log);
        // Clock below the draw: clock / previous.
        let (mut e, calls) = seeding_engine(true, 0.5, 1_000_000_000);
        e.call(0x00b3_f160, &args![0u32, -1i32]);
        let log = calls.borrow();
        let set = log.iter().find(|c| c.0 == RANDOM_SET).unwrap();
        assert_eq!(f64::take(&set.1, &mut 0), 1000000000.0 / 1073741824.0);
    }

    #[test]
    fn reseed_with_a_number_uses_it_as_the_integer_seed() {
        for (given, used) in [(5i32, 5u32), (1, 1), (0, 1), (-2, 1), (i32::MIN, 1)] {
            let (mut e, calls) = seeding_engine(false, 0.0, 0);
            e.call(0x00b3_f160, &args![0u32, given]);
            assert_eq!(*calls.borrow(), vec![(RANDOM_SET_LONG, vec![used])]);
            assert_eq!(e.global::<u8>(SEEDED), 1);
        }
    }

    #[test]
    fn mean_and_variance_write_their_pairs() {
        let (mut e, _) = seeding_engine(false, 0.0, 0);
        e.set_global(MEAN, 0.5f32);
        e.set_global(VARIANCE, f32::from_bits(0x3daa_aaab));
        let result = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(result.addr() + 4, 9);
        assert_eq!(
            e.call(0x00b3_f210, &args![0u32, result]).ptr::<()>(),
            result
        );
        assert_eq!(e.mem.f32(result.addr()), 0.5);
        assert_eq!(e.mem.u32(result.addr() + 4), 0);
        e.mem.set_u32(result.addr() + 4, 9);
        assert_eq!(
            e.call(0x00b3_f240, &args![0u32, result]).ptr::<()>(),
            result
        );
        assert_eq!(e.mem.u32(result.addr()), 0x3daa_aaab);
        assert_eq!(e.mem.u32(result.addr() + 4), 0);
    }

    #[test]
    fn the_constant_one_function_answers_one() {
        let (mut e, _) = seeding_engine(false, 0.0, 0);
        assert_eq!(e.call(0x00b3_f270, &args![0u32]).u32(), 1);
    }

    #[test]
    fn uniform_next_is_random_next() {
        let (mut e, calls) = seeding_engine(false, 0.375, 0);
        assert_eq!(e.call(0x00b3_f280, &args![0x4321u32]).f64(), 0.375);
        assert_eq!(*calls.borrow(), vec![(RANDOM_NEXT, vec![0x4321])]);
    }

    #[test]
    fn uniform_density_is_one_on_the_unit_interval() {
        let (mut e, _) = seeding_engine(false, 0.0, 0);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        for (x, density) in [
            (0.0f32, 1.0f32),
            (0.5, 1.0),
            (1.0, 1.0),
            (-0.001, 0.0),
            (1.001, 0.0),
            (f32::NAN, 1.0),
        ] {
            assert_eq!(e.call(0x00b3_f2a0, &args![0u32, x]).f32(), density);
        }
    }

    #[test]
    fn scalar_deleting_destructor_sets_the_vtable_and_deletes_on_bit_zero() {
        let (mut e, calls) = seeding_engine(false, 0.0, 0);
        let this = Ptr::<()>::new(e.mem.alloc(4));
        assert_eq!(e.call(0x00b3_f2e0, &args![this, 0u32]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), RANDOM_VTABLE);
        assert!(calls.borrow().is_empty());
        e.call(0x00b3_f2e0, &args![this, 1u32]);
        assert_eq!(*calls.borrow(), vec![(OPERATOR_DELETE, vec![this.addr()])]);
    }
}
