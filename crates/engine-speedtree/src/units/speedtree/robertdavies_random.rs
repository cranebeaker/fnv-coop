//! `SpeedTree/robertdavies_random.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The random-number classes SpeedTree links in (Robert Davies' Newran: the
//! base class `Random` with its 128-entry shuffle table, `Uniform`, and the
//! rejection samplers `PosGen`, `SymGen` and `Normal`). The generator is
//! the Park-Miller "minimal standard" sequence in `double` state, shuffled
//! through the table; every function here reproduces the exe's operations in
//! its order.
//!
//! The functions that can throw (`Random::Next`, `Random::load`,
//! `Random::Set`, `PosGen::Build`, `Normal::Normal`) carry the compiler's
//! exception frame (`FS:[0]`, `/GS` cookie). The unwinding tables are not
//! translated; the throw itself is, and it does not return.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The uniform form takes the value as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `operator new(size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)` (`cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
/// `std::runtime_error::runtime_error(const std::string&)` (`thiscall`; the
/// engine map has no name for the body, the call is read from the throw
/// sites).
const RUNTIME_ERROR_FROM_STRING: u32 = 0x0044_a310;
/// The exception constructor `PosGen::Build` uses (`thiscall`, one string
/// argument; unnamed in the engine map, `SpeedTree/speedtreert.cpp`).
const BUILD_ERROR_FROM_STRING: u32 = 0x00b0_96a0;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `fabs` (`cdecl`, `double` argument and result in ST0).
const FABS: u32 = 0x00ec_6cde;
/// `exp` (`cdecl`, `double` argument and result in ST0).
const EXP: u32 = 0x00ec_ae30;

/// The shared generator state: the Park-Miller seed as a `double`
/// (`Random::seed` in the Xbox PDB).
const RANDOM_STATE: u32 = 0x011f_9120;
/// The 128-entry shuffle table of `float` (`Random::Buffer`, which the Xbox
/// PDB sizes at 512 bytes).
const RANDOM_TABLE: u32 = 0x011f_8f20;
/// `4.656613e-10` (`double`, 2^-31): scales the integer seed into 0..1.
const RAW_SCALE: u32 = 0x0102_29b8;
/// `128.0` (`double`): the table size, as a multiplier for the table index.
const TABLE_SIZE: u32 = 0x0102_e430;
/// `2147483648.0` (`double`): scales a seed in 0..1 to the integer state.
const SEED_SCALE: u32 = 0x010a_5990;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;
/// `8.0` (`double`): the cut-off of `Normal::Density`.
const DENSITY_CUTOFF: u32 = 0x0102_40b0;
/// `1/sqrt(2 pi)` (`double`).
const NORMAL_FACTOR: u32 = 0x010a_5a78;
/// `-1.0` (`float`): `SymGen::Next`'s sign for the upper half.
const MINUS_ONE: u32 = 0x0101_2054;
/// `0.01` (`float`): `PosGen::Build`'s step for the symmetric case.
const BUILD_STEP_SYMMETRIC: u32 = 0x0101_3ea4;
/// `0.02` (`float`): `PosGen::Build`'s step for the one-sided case.
const BUILD_STEP_ONE_SIDED: u32 = 0x0102_d934;
/// The `Random` vtable (RTTI `.?AVRandom@@`).
const RANDOM_VTABLE: u32 = 0x010a_58c8;
/// The `PosGen` vtable (RTTI `.?AVPosGen@@`).
const POS_GEN_VTABLE: u32 = 0x010a_59d4;
/// The `Normal` vtable (RTTI `.?AVNormal@@`).
const NORMAL_VTABLE: u32 = 0x010a_5a2c;
/// The `SymGen` vtable (RTTI `.?AVSymGen@@`).
const SYM_GEN_VTABLE: u32 = 0x010a_5a54;
/// `Normal::count` (Xbox PDB static): the number of live `Normal`s that
/// share the built tables.
const NORMAL_COUNT: u32 = 0x011f_912c;
/// `Normal::Nxi` (Xbox PDB static).
const NORMAL_XI: u32 = 0x011f_9128;
/// `Normal::Nsx` (Xbox PDB static).
const NORMAL_SX: u32 = 0x011f_8f18;
/// `Normal::Nsfx` (Xbox PDB static).
const NORMAL_SFX: u32 = 0x011f_8f1c;
/// `"Random number generator not initialised"`.
const MESSAGE_NOT_INITIALISED: u32 = 0x010a_5968;
/// `"Newran: illegal combination"`.
const MESSAGE_ILLEGAL_COMBINATION: u32 = 0x010a_5998;
/// `"Newran: seed out of range"`.
const MESSAGE_SEED_OUT_OF_RANGE: u32 = 0x010a_59b4;
/// `"Newran: area too large"`.
const MESSAGE_AREA_TOO_LARGE: u32 = 0x010a_5a10;
/// `"Newran: area too small"`.
const MESSAGE_AREA_TOO_SMALL: u32 = 0x010a_59f8;
/// The throw information of the `runtime_error` throws.
const RUNTIME_ERROR_THROW_INFO: u32 = 0x0117_39ec;
/// The throw information of `PosGen::Build`'s throws.
const BUILD_ERROR_THROW_INFO: u32 = 0x0117_105c;
/// The names `Random::Name` and its overrides return.
const NAME_RANDOM: u32 = 0x0105_c3b0;
const NAME_UNIFORM: u32 = 0x010a_5a80;
const NAME_NORMAL: u32 = 0x0101_c29c;
const NAME_POS_GEN: u32 = 0x010a_5a88;
const NAME_SYM_GEN: u32 = 0x010a_5a90;

layout! {
    /// `PosGen` (Xbox PDB), 0x14 bytes; `SymGen` and `Normal` have the same
    /// layout.
    pub struct PosGen: 0x14 {
        /// The vtable.
        0x00 vtable: u32,
        /// `xi` (Xbox PDB): the number of table segments, as a `float`.
        0x04 xi: f32,
        /// `sx` (Xbox PDB): `float*`, the segment boundaries.
        0x08 sx: Ptr,
        /// `sfx` (Xbox PDB): `float*`, the density at the boundaries.
        0x0C sfx: Ptr,
        /// `NotReady` (Xbox PDB): the tables are not built yet.
        0x10 NotReady: bool,
    }
}

/// `_ftol2_sse` on a value held in ST0.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// Builds and throws a C++ exception: a `std::string` from `message`, the
/// exception object from it (`make_exception`), then `_CxxThrowException`.
/// The compiler knows the throw does not return; neither does this.
fn throw_with_message(e: &mut Engine, message: u32, make_exception: u32, throw_info: u32) -> ! {
    e.with_stack(0x1c, |e, text| {
        e.call(STRING_FROM_C_STRING, &args![text, message]);
        e.with_stack(0x28, |e, exception| {
            e.call(make_exception, &args![exception, text]);
            e.call(CXX_THROW_EXCEPTION, &args![exception, throw_info]);
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

// Translated from 00b43310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::Raw` (Xbox PDB): one step of the Park-Miller generator
/// (multiplier 16807, modulus 2^31 - 1 by Schrage's method: 127773 and
/// 2836). The state is the `double` at `011f9120`; the result is the new
/// state as a `float` scaled by 2^-31.
pub fn random_raw(e: &mut Engine) -> f32 {
    let state: f64 = e.global(RANDOM_STATE);
    let value = float_to_int(e, state);
    let quotient = value / 0x1f31d;
    let remainder = value % 0x1f31d;
    let mut next = remainder
        .wrapping_mul(0x41a7)
        .wrapping_sub(quotient.wrapping_mul(0xb14));
    if next <= 0 {
        next = next.wrapping_add(0x7fff_ffff);
    }
    e.set_global(RANDOM_STATE, next as f64);
    // `FLD double` then `FSTP float`: the new state rounded to a float.
    let state_as_float = next as f64 as f32;
    let scale: f64 = e.global(RAW_SCALE);
    (state_as_float as f64 * scale) as f32
}

// Translated from 00b43390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::Density` (Xbox PDB): the base class has no density, 0. The
/// argument (the point) is not read.
pub fn random_density(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> f32 {
    0.0
}

// Translated from 00b433a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b433a0 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throw is.
/// `Random::Next` (Xbox PDB): one shuffled sample. A state of exactly 0
/// means the generator was never seeded: it throws
/// `"Random number generator not initialised"`. Otherwise one `Raw` picks a
/// table slot, the slot's value is the result and `Raw` refills the slot.
pub fn random_next(e: &mut Engine, _this: Ptr) -> f32 {
    let state: f64 = e.global(RANDOM_STATE);
    if 0.0 == state {
        throw_with_message(
            e,
            MESSAGE_NOT_INITIALISED,
            RUNTIME_ERROR_FROM_STRING,
            RUNTIME_ERROR_THROW_INFO,
        );
    }
    let raw = random_raw(e);
    let table_size: f64 = e.global(TABLE_SIZE);
    let index = float_to_int(e, raw as f64 * table_size);
    // Two calls of the empty index check, as the exe makes them.
    fn_00b43480(e, index as u32);
    fn_00b43480(e, index as u32);
    let slot = RANDOM_TABLE.wrapping_add((index as u32).wrapping_mul(4));
    let result = x87_quiet(e.mem.f32(slot));
    let refill = random_raw(e);
    e.mem.set_f32(slot, refill);
    result
}

// Translated from 00b43480 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (`cdecl`, one word) that `Random::Next` calls twice with
/// the table index: what is left of a bounds check in this build. Its name
/// is unknown.
pub fn fn_00b43480(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00b43490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b43490 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throw is.
/// `Random::load` (Xbox PDB): always throws `"Newran: illegal combination"`
/// (the base class cannot be loaded).
pub fn random_load(e: &mut Engine, _this: Ptr) {
    throw_with_message(
        e,
        MESSAGE_ILLEGAL_COMBINATION,
        RUNTIME_ERROR_FROM_STRING,
        RUNTIME_ERROR_THROW_INFO,
    )
}

// Translated from 00b43510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b43510 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throw is.
/// `Random::Set` (Xbox PDB, `cdecl`): seeds the generator from a number in
/// the open interval (0, 1) (anything else, NaN included, throws
/// `"Newran: seed out of range"`): the state is the seed times 2^31,
/// truncated, then the 128-entry table is filled with `Raw` samples.
pub fn random_set(e: &mut Engine, seed: f64) {
    // `FLD1; FCOMP seed` jumps to the throw when 1.0 <= seed, then
    // `FLDZ; FCOMP seed` continues only when 0.0 < seed.
    if !(1.0 <= seed) && 0.0 < seed {
        let scale: f64 = e.global(SEED_SCALE);
        let state = float_to_int(e, seed * scale);
        e.set_global(RANDOM_STATE, state as f64);
        for i in 0..0x80u32 {
            let sample = random_raw(e);
            e.mem.set_f32(RANDOM_TABLE + 4 * i, sample);
        }
        return;
    }
    throw_with_message(
        e,
        MESSAGE_SEED_OUT_OF_RANGE,
        RUNTIME_ERROR_FROM_STRING,
        RUNTIME_ERROR_THROW_INFO,
    )
}

// Translated from 00b435f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::SetLong` (Xbox PDB, `cdecl`): seeds the generator with an
/// integer state and fills the 128-entry table with `Raw` samples.
pub fn random_set_long(e: &mut Engine, seed: i32) {
    e.set_global(RANDOM_STATE, seed as f64);
    for i in 0..0x80u32 {
        let sample = random_raw(e);
        e.mem.set_f32(RANDOM_TABLE + 4 * i, sample);
    }
}

// Translated from 00b43630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PosGen::PosGen` (identified from the `PosGen` vtable it stores and the
/// Xbox PDB layout; the engine map has no name): the `Random` vtable, then
/// `PosGen`'s; not ready, `xi` 0, no tables. Returns `this`.
pub fn fn_00b43630(e: &mut Engine, this: Ptr<PosGen>) -> Ptr<PosGen> {
    e.set(this, PosGen::vtable, RANDOM_VTABLE);
    e.set(this, PosGen::vtable, POS_GEN_VTABLE);
    e.set(this, PosGen::NotReady, true);
    e.set(this, PosGen::xi, 0.0);
    e.set(this, PosGen::sx, Ptr::NULL);
    e.set(this, PosGen::sfx, Ptr::NULL);
    this
}

// Translated from 00b43680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PosGen::Mean` (Xbox PDB): writes the pair (0.0, 4) to `result` (a
/// `float` and an integer) and returns it.
pub fn pos_gen_mean(e: &mut Engine, _this: Ptr<PosGen>, result: Ptr) -> Ptr {
    e.mem.set_u32(result.addr() + 4, 4);
    e.mem.set_f32(result.addr(), 0.0);
    result
}

// Translated from 00b436b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PosGen` destructor body (identified from the `PosGen` vtable it
/// stores; the engine map has no name): sets `PosGen`'s vtable, frees the
/// two tables when the object built them (`NotReady` clear), then sets the
/// `Random` vtable.
pub fn fn_00b436b0(e: &mut Engine, this: Ptr<PosGen>) {
    e.set(this, PosGen::vtable, POS_GEN_VTABLE);
    if !e.get(this, PosGen::NotReady) {
        let sx = e.get(this, PosGen::sx);
        e.call(OPERATOR_DELETE, &args![sx]);
        let sfx = e.get(this, PosGen::sfx);
        e.call(OPERATOR_DELETE, &args![sfx]);
    }
    e.set(this, PosGen::vtable, RANDOM_VTABLE);
}

// Translated from 00b43710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b43710 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throws are.
/// `PosGen::Build` (Xbox PDB): builds the 60-entry tables of segment
/// boundaries `sx` and densities `sfx` (`float`, 0xf0 bytes each) by walking
/// right from 0 in steps of `step / density` (step 0.02, or 0.01 when
/// `symmetric`) until the density (the virtual `Density`, vtable +0xc) drops
/// to 0 or below. Throws `"Newran: area too large"` when 60 entries do not
/// reach that point and `"Newran: area too small"` when it comes before 50.
/// `xi` becomes the entry count (twice that when symmetric).
pub fn pos_gen_build(e: &mut Engine, this: Ptr<PosGen>, symmetric: u8) {
    e.set(this, PosGen::NotReady, false);
    let xs = e.call(OPERATOR_NEW, &args![0xf0u32]).ptr::<()>();
    e.set(this, PosGen::sx, xs);
    let densities = e.call(OPERATOR_NEW, &args![0xf0u32]).ptr::<()>();
    e.set(this, PosGen::sfx, densities);
    let mut position = 0.0f32;
    let step = if symmetric != 0 {
        x87_quiet(e.global::<f32>(BUILD_STEP_SYMMETRIC))
    } else {
        x87_quiet(e.global::<f32>(BUILD_STEP_ONE_SIDED))
    };
    let mut count = 0i32;
    loop {
        if count >= 0x3c {
            throw_with_message(
                e,
                MESSAGE_AREA_TOO_LARGE,
                BUILD_ERROR_FROM_STRING,
                BUILD_ERROR_THROW_INFO,
            );
        }
        let sx = e.get(this, PosGen::sx);
        e.mem.set_f32(
            sx.addr().wrapping_add(4 * count as u32),
            x87_quiet(position),
        );
        let density = e.vcall(this.addr(), 0xc, &args![x87_quiet(position)]).f32();
        let sfx = e.get(this, PosGen::sfx);
        e.mem
            .set_f32(sfx.addr().wrapping_add(4 * count as u32), density);
        let zero: f64 = e.global(ZERO);
        // `FCOMP` then `TEST AH,0x41; JP`: leaves the loop when the density
        // is below or equal to 0; NaN stays in it.
        if density as f64 <= zero {
            break;
        }
        position = (step as f64 / density as f64 + position as f64) as f32;
        count += 1;
    }
    if count < 0x32 {
        throw_with_message(
            e,
            MESSAGE_AREA_TOO_SMALL,
            BUILD_ERROR_FROM_STRING,
            BUILD_ERROR_THROW_INFO,
        );
    }
    let entries = if symmetric != 0 {
        let two: f64 = e.global(TWO);
        count as f64 * two
    } else {
        count as f64
    };
    e.set(this, PosGen::xi, entries as f32);
}

// Translated from 00b43910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PosGen::Next` (Xbox PDB): builds the tables on first use, then
/// rejection-samples: a uniform sample picks a table segment, a second one a
/// candidate inside it, a third a height under the segment's left density;
/// the candidate is accepted when the next density is above that height, or
/// when the height lies under the virtual `Density` at the candidate.
pub fn pos_gen_next(e: &mut Engine, this: Ptr<PosGen>) -> f32 {
    if e.get(this, PosGen::NotReady) {
        pos_gen_build(e, this, 0);
    }
    loop {
        let selector = random_next(e, this.cast());
        let xi = e.get(this, PosGen::xi);
        let index = float_to_int(e, selector as f64 * xi as f64) as u32;
        let offset = index.wrapping_mul(4);
        let sx = e.get(this, PosGen::sx).addr();
        let left = x87_quiet(e.mem.f32(sx.wrapping_add(offset)));
        let sx = e.get(this, PosGen::sx).addr();
        let right = e.mem.f32(sx.wrapping_add(offset).wrapping_add(4));
        let width = right as f64 - left as f64;
        let across = random_next(e, this.cast());
        let candidate = (across as f64 * width + left as f64) as f32;
        let sfx = e.get(this, PosGen::sfx).addr();
        let up = random_next(e, this.cast());
        let height = (up as f64 * e.mem.f32(sfx.wrapping_add(offset)) as f64) as f32;
        let sfx = e.get(this, PosGen::sfx).addr();
        let next_density = x87_quiet(e.mem.f32(sfx.wrapping_add(offset).wrapping_add(4)));
        // `FCOMPP` then `TEST AH,0x41; JP`: the density test only runs when
        // the next density is below or equal to the height; greater or NaN
        // accepts the candidate.
        if !(next_density as f64 <= height as f64) {
            return x87_quiet(candidate);
        }
        let density = e
            .vcall(this.addr(), 0xc, &args![x87_quiet(candidate)])
            .f64();
        if (height as f64) < density {
            return x87_quiet(candidate);
        }
    }
}

// Translated from 00b439e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SymGen::Next` (Xbox PDB): `PosGen::Next` on the symmetric tables. A
/// first uniform sample above 0.5 is reflected (`1 - u`) and gives the
/// result a sign of -1.0; the accepted candidate is multiplied by that sign.
pub fn sym_gen_next(e: &mut Engine, this: Ptr<PosGen>) -> f32 {
    if e.get(this, PosGen::NotReady) {
        pos_gen_build(e, this, 1);
    }
    loop {
        let mut sign = 1.0f32;
        let mut selector = random_next(e, this.cast());
        // `FCOMP` then `TEST AH,0x41; JNZ` skips the reflection for
        // below, equal and NaN.
        let half: f64 = e.global(HALF);
        if selector as f64 > half {
            sign = x87_quiet(e.global::<f32>(MINUS_ONE));
            selector = (1.0 - selector as f64) as f32;
        }
        let xi = e.get(this, PosGen::xi);
        let index = float_to_int(e, selector as f64 * xi as f64) as u32;
        let offset = index.wrapping_mul(4);
        let sx = e.get(this, PosGen::sx).addr();
        let left = x87_quiet(e.mem.f32(sx.wrapping_add(offset)));
        let sx = e.get(this, PosGen::sx).addr();
        let right = e.mem.f32(sx.wrapping_add(offset).wrapping_add(4));
        let width = right as f64 - left as f64;
        let across = random_next(e, this.cast());
        let candidate = (across as f64 * width + left as f64) as f32;
        let sfx = e.get(this, PosGen::sfx).addr();
        let up = random_next(e, this.cast());
        let height = (up as f64 * e.mem.f32(sfx.wrapping_add(offset)) as f64) as f32;
        let sfx = e.get(this, PosGen::sfx).addr();
        let next_density = x87_quiet(e.mem.f32(sfx.wrapping_add(offset).wrapping_add(4)));
        if !(next_density as f64 <= height as f64) {
            return (sign as f64 * candidate as f64) as f32;
        }
        let density = e
            .vcall(this.addr(), 0xc, &args![x87_quiet(candidate)])
            .f64();
        if (height as f64) < density {
            return (sign as f64 * candidate as f64) as f32;
        }
    }
}

// Translated from 00b43ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b43ae0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `Normal::Normal` (identified from the `Normal` vtable it stores and the
/// Xbox PDB statics `Nxi`, `Nsx`, `Nsfx`, `count`; the engine map has no
/// name): the `PosGen` constructor, `SymGen`'s vtable, then `Normal`'s. The
/// first `Normal` builds the symmetric tables (`PosGen::Build(1)`) and
/// keeps them in the statics; every later one shares them and is marked
/// ready. `count` goes up by one. Returns `this`.
pub fn fn_00b43ae0(e: &mut Engine, this: Ptr<PosGen>) -> Ptr<PosGen> {
    fn_00b43630(e, this);
    e.set(this, PosGen::vtable, SYM_GEN_VTABLE);
    e.set(this, PosGen::vtable, NORMAL_VTABLE);
    let count: u32 = e.global(NORMAL_COUNT);
    if count != 0 {
        e.set(this, PosGen::NotReady, false);
        let xi = x87_quiet(e.global::<f32>(NORMAL_XI));
        e.set(this, PosGen::xi, xi);
        let sx: u32 = e.global(NORMAL_SX);
        e.set(this, PosGen::sx, Ptr::new(sx));
        let sfx: u32 = e.global(NORMAL_SFX);
        e.set(this, PosGen::sfx, Ptr::new(sfx));
    } else {
        pos_gen_build(e, this, 1);
        let xi = x87_quiet(e.get(this, PosGen::xi));
        e.set_global(NORMAL_XI, xi);
        let sx = e.get(this, PosGen::sx).addr();
        e.set_global(NORMAL_SX, sx);
        let sfx = e.get(this, PosGen::sfx).addr();
        e.set_global(NORMAL_SFX, sfx);
    }
    e.set_global(NORMAL_COUNT, count.wrapping_add(1));
    this
}

// Translated from 00b43bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Normal::Mean` (Xbox PDB): writes the pair (0.0, 0) to `result` and
/// returns it.
pub fn normal_mean(e: &mut Engine, _this: Ptr<PosGen>, result: Ptr) -> Ptr {
    e.mem.set_u32(result.addr() + 4, 0);
    e.mem.set_f32(result.addr(), 0.0);
    result
}

// Translated from 00b43bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Normal::Variance` (Xbox PDB): writes the pair (1.0, 0) to `result` and
/// returns it.
pub fn normal_variance(e: &mut Engine, _this: Ptr<PosGen>, result: Ptr) -> Ptr {
    e.mem.set_u32(result.addr() + 4, 0);
    e.mem.set_f32(result.addr(), 1.0);
    result
}

// Translated from 00b43c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Normal::scalar deleting destructor` (Xbox PDB): the `Normal` destructor
/// body, then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn normal_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PosGen>,
    flags: u32,
) -> Ptr<PosGen> {
    fn_00b43ca0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00b43c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor thunk (unnamed in the engine map): runs the `PosGen`
/// destructor body (`00b436b0`).
pub fn fn_00b43c50(e: &mut Engine, this: Ptr<PosGen>) {
    fn_00b436b0(e, this);
}

// Translated from 00b43c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PosGen::scalar deleting destructor` (Xbox PDB): the `PosGen` destructor
/// body, then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn pos_gen_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PosGen>,
    flags: u32,
) -> Ptr<PosGen> {
    fn_00b436b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00b43ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Normal` destructor body (identified from the `Normal` vtable it
/// stores; the engine map has no name): sets `Normal`'s vtable and takes one
/// off `count`; while other `Normal`s remain the shared tables must not be
/// freed, so this one is marked not ready. Then the `PosGen` destructor body.
pub fn fn_00b43ca0(e: &mut Engine, this: Ptr<PosGen>) {
    e.set(this, PosGen::vtable, NORMAL_VTABLE);
    let count = e.global::<u32>(NORMAL_COUNT).wrapping_sub(1);
    e.set_global(NORMAL_COUNT, count);
    if count != 0 {
        e.set(this, PosGen::NotReady, true);
    }
    fn_00b436b0(e, this);
}

// Translated from 00b43ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Normal::Density` (Xbox PDB): the standard normal density at `x`,
/// `exp(-x*x/2) / sqrt(2 pi)`, 0 when `|x|` (as a `float`) is above 8.
pub fn normal_density(e: &mut Engine, _this: Ptr<PosGen>, x: f32) -> f32 {
    let magnitude = e.call(FABS, &args![x as f64]).f64() as f32;
    // `FCOMP` then `TEST AH,0x41; JNZ`: below, equal and NaN go on to the
    // exponential.
    let cutoff: f64 = e.global(DENSITY_CUTOFF);
    let density = if magnitude as f64 > cutoff {
        0.0
    } else {
        let two: f64 = e.global(TWO);
        let exponent = ((x as f64 * -(x as f64)) / two) as f32;
        let exponential = e.call(EXP, &args![exponent as f64]).f64() as f32;
        let factor: f64 = e.global(NORMAL_FACTOR);
        exponential as f64 * factor
    };
    density as f32
}

// Translated from 00b43d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Random::Name` (Xbox PDB): the address of the class name string.
pub fn random_name(_e: &mut Engine, _this: Ptr) -> u32 {
    NAME_RANDOM
}

// Translated from 00b43d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Uniform::Name` (Xbox PDB): the address of the class name string.
pub fn uniform_name(_e: &mut Engine, _this: Ptr) -> u32 {
    NAME_UNIFORM
}

// Translated from 00b43d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Normal::Name` (Xbox PDB): the address of the class name string.
pub fn normal_name(_e: &mut Engine, _this: Ptr) -> u32 {
    NAME_NORMAL
}

// Translated from 00b43d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PosGen::Name` (Xbox PDB): the address of the class name string.
pub fn pos_gen_name(_e: &mut Engine, _this: Ptr) -> u32 {
    NAME_POS_GEN
}

// Translated from 00b43da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SymGen::Name` (Xbox PDB): the address of the class name string.
pub fn sym_gen_name(_e: &mut Engine, _this: Ptr) -> u32 {
    NAME_SYM_GEN
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b43310, random_raw() -> f32),
        entry!(0x00b43390, random_density(Ptr, u32) -> f32),
        entry!(0x00b433a0, random_next(Ptr) -> f32),
        entry!(0x00b43480, fn_00b43480(u32)),
        entry!(0x00b43490, random_load(Ptr)),
        entry!(0x00b43510, random_set(f64)),
        entry!(0x00b435f0, random_set_long(i32)),
        entry!(0x00b43630, fn_00b43630(Ptr<PosGen>) -> Ptr<PosGen>),
        entry!(0x00b43680, pos_gen_mean(Ptr<PosGen>, Ptr) -> Ptr),
        entry!(0x00b436b0, fn_00b436b0(Ptr<PosGen>)),
        entry!(0x00b43710, pos_gen_build(Ptr<PosGen>, u8)),
        entry!(0x00b43910, pos_gen_next(Ptr<PosGen>) -> f32),
        entry!(0x00b439e0, sym_gen_next(Ptr<PosGen>) -> f32),
        entry!(0x00b43ae0, fn_00b43ae0(Ptr<PosGen>) -> Ptr<PosGen>),
        entry!(0x00b43bc0, normal_mean(Ptr<PosGen>, Ptr) -> Ptr),
        entry!(0x00b43bf0, normal_variance(Ptr<PosGen>, Ptr) -> Ptr),
        entry!(
            0x00b43c20,
            normal_scalar_deleting_destructor(Ptr<PosGen>, u32) -> Ptr<PosGen>
        ),
        entry!(0x00b43c50, fn_00b43c50(Ptr<PosGen>)),
        entry!(
            0x00b43c70,
            pos_gen_scalar_deleting_destructor(Ptr<PosGen>, u32) -> Ptr<PosGen>
        ),
        entry!(0x00b43ca0, fn_00b43ca0(Ptr<PosGen>)),
        entry!(0x00b43ce0, normal_density(Ptr<PosGen>, f32) -> f32),
        entry!(0x00b43d60, random_name(Ptr) -> u32),
        entry!(0x00b43d70, uniform_name(Ptr) -> u32),
        entry!(0x00b43d80, normal_name(Ptr) -> u32),
        entry!(0x00b43d90, pos_gen_name(Ptr) -> u32),
        entry!(0x00b43da0, sym_gen_name(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Where the test `Density` double lives, and the vtable address of
    /// the test objects (slot +0xc is the density).
    const DENSITY_DOUBLE: u32 = 0x0900_0100;
    const TEST_VTABLE: u32 = 0x0900_0000;

    /// The words that every test of this unit needs in memory: the
    /// generator's state and table, the constants the code loads and a
    /// truncating `_ftol2_sse`.
    fn random_engine() -> Engine {
        let mut e = Engine::new();
        e.register(FTOL, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        for page in [
            0x011f_8000,
            0x011f_9000,
            0x0101_1000,
            0x0101_2000,
            0x0101_3000,
            0x0102_2000,
            0x0102_4000,
            0x0102_d000,
            0x0102_e000,
            0x010a_5000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(RAW_SCALE, 1.0f64 / 2147483648.0);
        e.set_global(TABLE_SIZE, 128.0f64);
        e.set_global(SEED_SCALE, 2147483648.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(DENSITY_CUTOFF, 8.0f64);
        e.set_global(NORMAL_FACTOR, 0.3989422804014327f64);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(BUILD_STEP_SYMMETRIC, 0.01f32);
        e.set_global(BUILD_STEP_ONE_SIDED, 0.02f32);
        e.put_vtable(TEST_VTABLE, &[0, 0, 0, DENSITY_DOUBLE]);
        e
    }

    /// `Raw`'s result for the new state `state`: the state as a `float`
    /// times 2^-31.
    fn raw_of(state: i32) -> f32 {
        (state as f32 as f64 * (1.0f64 / 2147483648.0)) as f32
    }

    type Log = Rc<RefCell<Vec<u32>>>;

    /// Doubles for the throw machinery: the string constructor records its
    /// message, the throw records its information and panics.
    fn throw_probe(e: &mut Engine) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let messages = log.clone();
        e.register_double(STRING_FROM_C_STRING, move |_, a| {
            messages.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.register(RUNTIME_ERROR_FROM_STRING, |_, _| Ret::default());
        e.register(BUILD_ERROR_FROM_STRING, |_, _| Ret::default());
        let thrown = log.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |_, a| {
            thrown.borrow_mut().push(a[1]);
            panic!("thrown")
        });
        log
    }

    /// Runs `f` and says whether it ended in the throw double's panic.
    fn throws(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> bool {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(e))).is_err()
    }

    fn table(e: &Engine, index: u32) -> f32 {
        e.mem.f32(RANDOM_TABLE + 4 * index)
    }

    /// A `PosGen`-shaped object with the test vtable.
    fn object(e: &mut Engine) -> Ptr<PosGen> {
        let this: Ptr<PosGen> = e.new_object();
        e.set(this, PosGen::vtable, TEST_VTABLE);
        this
    }

    /// A density double that answers from `values` in order (the last one
    /// repeated) and records the points it was asked about.
    fn density_double(e: &mut Engine, values: Vec<f32>) -> Rc<RefCell<Vec<f32>>> {
        let asked = Rc::new(RefCell::new(Vec::new()));
        let record = asked.clone();
        e.register_double(DENSITY_DOUBLE, move |_, a| {
            let mut record = record.borrow_mut();
            let value = values[record.len().min(values.len() - 1)];
            record.push(f32::from_bits(a[1]));
            Ret {
                st0: value as f64,
                ..Ret::default()
            }
        });
        asked
    }

    #[test]
    fn raw_steps_the_park_miller_generator() {
        let mut e = random_engine();
        e.set_global(RANDOM_STATE, 1.0f64);
        assert_eq!(e.call(0x00b4_3310, &[]).f32(), raw_of(16807));
        assert_eq!(e.global::<f64>(RANDOM_STATE), 16807.0);
        assert_eq!(e.call(0x00b4_3310, &[]).f32(), raw_of(282475249));
        assert_eq!(e.global::<f64>(RANDOM_STATE), 282475249.0);
    }

    #[test]
    fn raw_adds_the_modulus_when_the_step_is_not_positive() {
        let mut e = random_engine();
        // 127773 = 0x1f31d: quotient 1, remainder 0, so 0 * 16807 - 2836.
        e.set_global(RANDOM_STATE, 127773.0f64);
        assert_eq!(e.call(0x00b4_3310, &[]).f32(), raw_of(2147480811));
        assert_eq!(e.global::<f64>(RANDOM_STATE), 2147480811.0);
        e.set_global(RANDOM_STATE, 0.0f64);
        e.call(0x00b4_3310, &[]);
        assert_eq!(e.global::<f64>(RANDOM_STATE), 2147483647.0);
    }

    #[test]
    fn density_of_the_base_class_is_zero() {
        let mut e = random_engine();
        assert_eq!(e.call(0x00b4_3390, &args![0u32, 1.5f32]).f32(), 0.0);
    }

    #[test]
    fn next_returns_the_chosen_slot_and_refills_it() {
        let mut e = random_engine();
        e.set_global(RANDOM_STATE, 1.0f64);
        e.mem.set_f32(RANDOM_TABLE, 0.5);
        e.mem.set_f32(RANDOM_TABLE + 4 * 96, 0.75);
        // States 16807 (slot 0) and 282475249 (the refill), then 1622650073
        // (slot 96) and 984943658 (the refill).
        assert_eq!(e.call(0x00b4_33a0, &args![0u32]).f32(), 0.5);
        assert_eq!(table(&e, 0), raw_of(282475249));
        assert_eq!(e.call(0x00b4_33a0, &args![0u32]).f32(), 0.75);
        assert_eq!(table(&e, 96), raw_of(984943658));
        assert_eq!(e.global::<f64>(RANDOM_STATE), 984943658.0);
    }

    #[test]
    fn next_throws_when_the_generator_was_never_seeded() {
        let mut e = random_engine();
        let log = throw_probe(&mut e);
        e.set_global(RANDOM_STATE, 0.0f64);
        assert!(throws(&mut e, |e| {
            e.call(0x00b4_33a0, &args![0u32]);
        }));
        assert_eq!(
            *log.borrow(),
            vec![MESSAGE_NOT_INITIALISED, RUNTIME_ERROR_THROW_INFO]
        );
    }

    #[test]
    fn the_index_check_does_nothing() {
        let mut e = random_engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_3480, &args![5u32]), Ret::default());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn load_always_throws() {
        let mut e = random_engine();
        let log = throw_probe(&mut e);
        assert!(throws(&mut e, |e| {
            e.call(0x00b4_3490, &args![0u32]);
        }));
        assert_eq!(
            *log.borrow(),
            vec![MESSAGE_ILLEGAL_COMBINATION, RUNTIME_ERROR_THROW_INFO]
        );
    }

    #[test]
    fn set_seeds_the_state_and_fills_the_table() {
        let mut e = random_engine();
        e.call(0x00b4_3510, &args![0.5f64]);
        // 0.5 * 2^31 = 1073741824; the table holds the next 128 samples.
        assert_eq!(table(&e, 0), raw_of(1073750227));
        assert_eq!(table(&e, 1), raw_of(1214979448));
        assert_eq!(table(&e, 126), raw_of(2080801380));
        assert_eq!(table(&e, 127), raw_of(257602265));
        assert_eq!(e.global::<f64>(RANDOM_STATE), 257602265.0);
    }

    #[test]
    fn set_throws_unless_the_seed_is_inside_zero_and_one() {
        let mut e = random_engine();
        let log = throw_probe(&mut e);
        for seed in [0.0, 1.0, 1.5, -0.5, f64::NAN] {
            log.borrow_mut().clear();
            assert!(throws(&mut e, |e| {
                e.call(0x00b4_3510, &args![seed]);
            }));
            assert_eq!(
                *log.borrow(),
                vec![MESSAGE_SEED_OUT_OF_RANGE, RUNTIME_ERROR_THROW_INFO]
            );
        }
        assert_eq!(e.global::<f64>(RANDOM_STATE), 0.0);
    }

    #[test]
    fn set_long_seeds_with_the_integer() {
        let mut e = random_engine();
        e.call(0x00b4_35f0, &args![1u32]);
        assert_eq!(table(&e, 0), raw_of(16807));
        assert_eq!(table(&e, 1), raw_of(282475249));
        assert_eq!(table(&e, 126), raw_of(2014119113));
        assert_eq!(table(&e, 127), raw_of(515204530));
        assert_eq!(e.global::<f64>(RANDOM_STATE), 515204530.0);
    }

    #[test]
    fn pos_gen_constructor_sets_the_vtable_and_clears_the_tables() {
        let mut e = random_engine();
        let this: Ptr<PosGen> = e.new_object();
        e.mem.set_u32(this.addr() + 4, 0x4000_0000);
        e.mem.set_u32(this.addr() + 8, 0x1111);
        e.mem.set_u32(this.addr() + 12, 0x2222);
        assert_eq!(e.call(0x00b4_3630, &args![this]).ptr::<PosGen>(), this);
        assert_eq!(e.get(this, PosGen::vtable), POS_GEN_VTABLE);
        assert!(e.get(this, PosGen::NotReady));
        assert_eq!(e.get(this, PosGen::xi), 0.0);
        assert!(e.get(this, PosGen::sx).is_null());
        assert!(e.get(this, PosGen::sfx).is_null());
    }

    #[test]
    fn pos_gen_mean_is_zero_with_four_segments() {
        let mut e = random_engine();
        let result = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(result.addr(), 0xffff_ffff);
        assert_eq!(
            e.call(0x00b4_3680, &args![0u32, result]).ptr::<()>(),
            result
        );
        assert_eq!(e.mem.f32(result.addr()), 0.0);
        assert_eq!(e.mem.u32(result.addr() + 4), 4);
    }

    #[test]
    fn pos_gen_destructor_frees_the_tables_only_when_built() {
        let mut e = random_engine();
        let freed: Log = Rc::new(RefCell::new(Vec::new()));
        let record = freed.clone();
        e.register_double(OPERATOR_DELETE, move |_, a| {
            record.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = object(&mut e);
        e.set(this, PosGen::NotReady, true);
        e.set(this, PosGen::sx, Ptr::new(0x1000));
        e.set(this, PosGen::sfx, Ptr::new(0x2000));
        e.call(0x00b4_36b0, &args![this]);
        assert!(freed.borrow().is_empty());
        assert_eq!(e.get(this, PosGen::vtable), RANDOM_VTABLE);
        e.set(this, PosGen::NotReady, false);
        e.call(0x00b4_36b0, &args![this]);
        assert_eq!(*freed.borrow(), vec![0x1000, 0x2000]);
    }

    /// Doubles for `operator new` that hand out real blocks.
    fn allocator(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
    }

    #[test]
    fn build_walks_right_until_the_density_reaches_zero() {
        let mut e = random_engine();
        allocator(&mut e);
        let mut values = vec![1.0f32; 55];
        values.push(0.0);
        let asked = density_double(&mut e, values);
        let this = object(&mut e);
        e.call(0x00b4_3710, &args![this, 0u32]);
        assert!(!e.get(this, PosGen::NotReady));
        // 56 densities were asked, the last one 0: 55 entries before it.
        assert_eq!(asked.borrow().len(), 56);
        assert_eq!(e.get(this, PosGen::xi), 55.0);
        let sx = e.get(this, PosGen::sx).addr();
        let sfx = e.get(this, PosGen::sfx).addr();
        assert_eq!(e.mem.f32(sx), 0.0);
        assert_eq!(e.mem.f32(sx + 4), 0.02);
        assert!((e.mem.f32(sx + 8) - 0.04).abs() < 1e-6);
        assert_eq!(asked.borrow()[1], 0.02);
        assert_eq!(e.mem.f32(sfx + 4 * 54), 1.0);
        assert_eq!(e.mem.f32(sfx + 4 * 55), 0.0);
    }

    #[test]
    fn build_symmetric_uses_the_smaller_step_and_doubles_the_count() {
        let mut e = random_engine();
        allocator(&mut e);
        let mut values = vec![2.0f32; 50];
        values.push(-1.0);
        density_double(&mut e, values);
        let this = object(&mut e);
        e.call(0x00b4_3710, &args![this, 1u32]);
        assert_eq!(e.get(this, PosGen::xi), 100.0);
        let sx = e.get(this, PosGen::sx).addr();
        // 0.01 / 2.0 per step.
        assert_eq!(e.mem.f32(sx + 4), 0.005);
    }

    #[test]
    fn build_throws_when_the_area_is_too_large_or_too_small() {
        let mut e = random_engine();
        allocator(&mut e);
        let log = throw_probe(&mut e);
        density_double(&mut e, vec![1.0]);
        let this = object(&mut e);
        assert!(throws(&mut e, |e| {
            e.call(0x00b4_3710, &args![this, 0u32]);
        }));
        assert_eq!(
            *log.borrow(),
            vec![MESSAGE_AREA_TOO_LARGE, BUILD_ERROR_THROW_INFO]
        );
        log.borrow_mut().clear();
        density_double(&mut e, vec![1.0, 1.0, 0.0]);
        assert!(throws(&mut e, |e| {
            e.call(0x00b4_3710, &args![this, 0u32]);
        }));
        assert_eq!(
            *log.borrow(),
            vec![MESSAGE_AREA_TOO_SMALL, BUILD_ERROR_THROW_INFO]
        );
    }

    /// A ready `PosGen` with four segments [0,1,2,3,4] and the given
    /// densities at the five boundaries.
    fn ready_object(e: &mut Engine, densities: [f32; 5]) -> Ptr<PosGen> {
        let this = object(e);
        let sx = e.mem.alloc(20);
        let sfx = e.mem.alloc(20);
        for i in 0..5u32 {
            e.mem.set_f32(sx + 4 * i, i as f32);
            e.mem.set_f32(sfx + 4 * i, densities[i as usize]);
        }
        e.set(this, PosGen::xi, 4.0);
        e.set(this, PosGen::sx, Ptr::new(sx));
        e.set(this, PosGen::sfx, Ptr::new(sfx));
        e.set(this, PosGen::NotReady, false);
        this
    }

    /// The generator state and table so the next uniform samples are the
    /// given ones: slots 0, 96 and 68 answer the first three (the Park-Miller
    /// sequence from 1 picks those slots), slots 6, 86 and 49 the next three.
    fn scripted_samples(e: &mut Engine, samples: [f32; 6]) {
        e.set_global(RANDOM_STATE, 1.0f64);
        for (slot, value) in [0, 96, 68, 6, 86, 49].into_iter().zip(samples) {
            e.mem.set_f32(RANDOM_TABLE + 4 * slot, value);
        }
    }

    #[test]
    fn pos_gen_next_accepts_under_the_next_density() {
        let mut e = random_engine();
        scripted_samples(&mut e, [0.5, 0.25, 0.5, 0.0, 0.0, 0.0]);
        let asked = density_double(&mut e, vec![0.0]);
        let this = ready_object(&mut e, [1.0; 5]);
        // Segment 2 (0.5 * 4), candidate 2 + 0.25, height 0.5 * 1.0 < 1.0.
        assert_eq!(e.call(0x00b4_3910, &args![this]).f32(), 2.25);
        assert!(asked.borrow().is_empty());
    }

    #[test]
    fn pos_gen_next_tests_the_density_when_the_height_is_high() {
        let mut e = random_engine();
        scripted_samples(&mut e, [0.5, 0.25, 0.5, 0.0, 0.0, 0.0]);
        let asked = density_double(&mut e, vec![0.9]);
        // The next density 0.25 is not above the height 0.5: the density
        // at the candidate decides, and 0.9 is above the height.
        let this = ready_object(&mut e, [1.0, 1.0, 1.0, 0.25, 0.25]);
        assert_eq!(e.call(0x00b4_3910, &args![this]).f32(), 2.25);
        assert_eq!(*asked.borrow(), vec![2.25]);
    }

    #[test]
    fn pos_gen_next_rejects_and_tries_again() {
        let mut e = random_engine();
        // First round: segment 2, height 0.5, density 0.1 rejects. Second
        // round: selector 0.0 (segment 0), candidate 0 + 0.5, height 0.
        scripted_samples(&mut e, [0.5, 0.25, 0.5, 0.0, 0.5, 0.0]);
        let asked = density_double(&mut e, vec![0.1, 0.2]);
        let this = ready_object(&mut e, [1.0, 1.0, 1.0, 0.25, 0.25]);
        assert_eq!(e.call(0x00b4_3910, &args![this]).f32(), 0.5);
        // The first candidate 2.25 was rejected by the density 0.1; in the
        // second round the next density 1.0 is above the height 0.0.
        assert_eq!(*asked.borrow(), vec![2.25]);
    }

    #[test]
    fn pos_gen_next_builds_the_tables_first_when_not_ready() {
        let mut e = random_engine();
        allocator(&mut e);
        scripted_samples(&mut e, [0.0; 6]);
        let mut values = vec![1.0f32; 55];
        values.push(0.0);
        values.push(1.0);
        density_double(&mut e, values);
        let this = object(&mut e);
        e.set(this, PosGen::NotReady, true);
        // Segment 0, candidate sx[0] + 0 * width = 0.
        assert_eq!(e.call(0x00b4_3910, &args![this]).f32(), 0.0);
        assert!(!e.get(this, PosGen::NotReady));
        assert_eq!(e.get(this, PosGen::xi), 55.0);
    }

    #[test]
    fn sym_gen_next_reflects_samples_above_one_half() {
        let mut e = random_engine();
        // Selector 0.75 is reflected to 0.25 (segment 1) and the result is
        // negative: candidate 1 + 0.5, height 0.5.
        scripted_samples(&mut e, [0.75, 0.5, 0.5, 0.0, 0.0, 0.0]);
        density_double(&mut e, vec![0.0]);
        let this = ready_object(&mut e, [1.0; 5]);
        assert_eq!(e.call(0x00b4_39e0, &args![this]).f32(), -1.5);
    }

    #[test]
    fn sym_gen_next_keeps_the_sign_for_samples_up_to_one_half() {
        let mut e = random_engine();
        scripted_samples(&mut e, [0.5, 0.5, 0.5, 0.0, 0.0, 0.0]);
        density_double(&mut e, vec![0.0]);
        let this = ready_object(&mut e, [1.0; 5]);
        // 0.5 is not above 0.5: segment 2, candidate 2.5.
        assert_eq!(e.call(0x00b4_39e0, &args![this]).f32(), 2.5);
    }

    #[test]
    fn sym_gen_next_rejects_and_restores_the_sign() {
        let mut e = random_engine();
        // First round reflected (-1) and rejected by the density; the second
        // round has selector 0.0, sign +1.
        scripted_samples(&mut e, [0.75, 0.5, 0.5, 0.0, 0.5, 0.0]);
        density_double(&mut e, vec![0.1]);
        // Round one: segment 1 with the next density 0.25 below the height 0.5.
        let this = ready_object(&mut e, [1.0, 1.0, 0.25, 1.0, 1.0]);
        assert_eq!(e.call(0x00b4_39e0, &args![this]).f32(), 0.5);
    }

    #[test]
    fn sym_gen_next_builds_the_tables_first_when_not_ready() {
        let mut e = random_engine();
        allocator(&mut e);
        scripted_samples(&mut e, [0.0; 6]);
        let mut values = vec![1.0f32; 55];
        values.push(0.0);
        values.push(1.0);
        density_double(&mut e, values);
        let this = object(&mut e);
        e.set(this, PosGen::NotReady, true);
        assert_eq!(e.call(0x00b4_39e0, &args![this]).f32(), 0.0);
        // Symmetric: twice the 55 entries.
        assert_eq!(e.get(this, PosGen::xi), 110.0);
    }

    #[test]
    fn normal_constructor_builds_once_and_shares_the_tables() {
        let mut e = random_engine();
        allocator(&mut e);
        e.put_vtable(NORMAL_VTABLE, &[0, 0, 0, DENSITY_DOUBLE]);
        let mut values = vec![1.0f32; 55];
        values.push(0.0);
        density_double(&mut e, values);
        let first: Ptr<PosGen> = e.new_object();
        assert_eq!(e.call(0x00b4_3ae0, &args![first]).ptr::<PosGen>(), first);
        assert_eq!(e.get(first, PosGen::vtable), NORMAL_VTABLE);
        assert!(!e.get(first, PosGen::NotReady));
        assert_eq!(e.get(first, PosGen::xi), 110.0);
        assert_eq!(e.global::<u32>(NORMAL_COUNT), 1);
        assert_eq!(e.global::<f32>(NORMAL_XI), 110.0);
        let sx = e.get(first, PosGen::sx);
        assert_eq!(e.global::<u32>(NORMAL_SX), sx.addr());
        assert_eq!(
            e.global::<u32>(NORMAL_SFX),
            e.get(first, PosGen::sfx).addr()
        );

        let second: Ptr<PosGen> = e.new_object();
        e.call(0x00b4_3ae0, &args![second]);
        assert_eq!(e.global::<u32>(NORMAL_COUNT), 2);
        assert!(!e.get(second, PosGen::NotReady));
        assert_eq!(e.get(second, PosGen::xi), 110.0);
        assert_eq!(e.get(second, PosGen::sx), sx);
        assert_eq!(e.get(second, PosGen::sfx), e.get(first, PosGen::sfx));
    }

    #[test]
    fn normal_mean_and_variance_write_their_pairs() {
        let mut e = random_engine();
        let result = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(result.addr() + 4, 7);
        assert_eq!(
            e.call(0x00b4_3bc0, &args![0u32, result]).ptr::<()>(),
            result
        );
        assert_eq!(e.mem.f32(result.addr()), 0.0);
        assert_eq!(e.mem.u32(result.addr() + 4), 0);
        e.mem.set_u32(result.addr() + 4, 7);
        assert_eq!(
            e.call(0x00b4_3bf0, &args![0u32, result]).ptr::<()>(),
            result
        );
        assert_eq!(e.mem.f32(result.addr()), 1.0);
        assert_eq!(e.mem.u32(result.addr() + 4), 0);
    }

    #[test]
    fn normal_destructor_keeps_the_shared_tables_while_others_live() {
        let mut e = random_engine();
        let freed: Log = Rc::new(RefCell::new(Vec::new()));
        let record = freed.clone();
        e.register_double(OPERATOR_DELETE, move |_, a| {
            record.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = object(&mut e);
        e.set(this, PosGen::NotReady, false);
        e.set(this, PosGen::sx, Ptr::new(0x1000));
        e.set(this, PosGen::sfx, Ptr::new(0x2000));
        e.set_global(NORMAL_COUNT, 2u32);
        // Another Normal remains: this one is marked not ready, nothing freed.
        e.call(0x00b4_3ca0, &args![this]);
        assert_eq!(e.global::<u32>(NORMAL_COUNT), 1);
        assert!(e.get(this, PosGen::NotReady));
        assert!(freed.borrow().is_empty());
        assert_eq!(e.get(this, PosGen::vtable), RANDOM_VTABLE);
        // The last one frees the tables.
        e.set(this, PosGen::NotReady, false);
        e.call(0x00b4_3ca0, &args![this]);
        assert_eq!(e.global::<u32>(NORMAL_COUNT), 0);
        assert_eq!(*freed.borrow(), vec![0x1000, 0x2000]);
    }

    #[test]
    fn scalar_deleting_destructors_delete_only_on_bit_zero() {
        let mut e = random_engine();
        let freed: Log = Rc::new(RefCell::new(Vec::new()));
        let record = freed.clone();
        e.register_double(OPERATOR_DELETE, move |_, a| {
            record.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = object(&mut e);
        e.set(this, PosGen::NotReady, true);
        e.set_global(NORMAL_COUNT, 1u32);
        assert_eq!(
            e.call(0x00b4_3c70, &args![this, 0u32]).ptr::<PosGen>(),
            this
        );
        assert!(freed.borrow().is_empty());
        assert_eq!(
            e.call(0x00b4_3c70, &args![this, 1u32]).ptr::<PosGen>(),
            this
        );
        assert_eq!(*freed.borrow(), vec![this.addr()]);
        freed.borrow_mut().clear();
        assert_eq!(
            e.call(0x00b4_3c20, &args![this, 2u32]).ptr::<PosGen>(),
            this
        );
        assert!(freed.borrow().is_empty());
        assert_eq!(e.global::<u32>(NORMAL_COUNT), 0);
        e.set_global(NORMAL_COUNT, 1u32);
        e.call(0x00b4_3c20, &args![this, 3u32]);
        assert_eq!(*freed.borrow(), vec![this.addr()]);
    }

    #[test]
    fn the_destructor_thunk_runs_the_pos_gen_body() {
        let mut e = random_engine();
        let this = object(&mut e);
        e.set(this, PosGen::NotReady, true);
        e.call(0x00b4_3c50, &args![this]);
        assert_eq!(e.get(this, PosGen::vtable), RANDOM_VTABLE);
    }

    #[test]
    fn normal_density_is_the_gaussian_up_to_eight() {
        let mut e = random_engine();
        e.register(FABS, |_, a| Ret {
            st0: f64::take(a, &mut 0).abs(),
            ..Ret::default()
        });
        e.register(EXP, |_, a| Ret {
            st0: f64::take(a, &mut 0).exp(),
            ..Ret::default()
        });
        let expected = |x: f32| {
            let exponent = ((x as f64 * -(x as f64)) / 2.0) as f32;
            ((exponent as f64).exp() as f32 as f64 * 0.3989422804014327) as f32
        };
        for x in [0.0f32, 1.0, -1.5, 8.0, -8.0] {
            assert_eq!(e.call(0x00b4_3ce0, &args![0u32, x]).f32(), expected(x));
        }
        assert!((e.call(0x00b4_3ce0, &args![0u32, 0.0f32]).f32() - 0.398_942_3).abs() < 1e-6);
        assert_eq!(e.call(0x00b4_3ce0, &args![0u32, 8.5f32]).f32(), 0.0);
        assert_eq!(e.call(0x00b4_3ce0, &args![0u32, -9.0f32]).f32(), 0.0);
        // NaN is not above the cut-off: it goes through the exponential.
        assert!(e.call(0x00b4_3ce0, &args![0u32, f32::NAN]).f32().is_nan());
    }

    #[test]
    fn the_names_are_the_class_name_strings() {
        let mut e = random_engine();
        for (address, name) in [
            (0x00b4_3d60, NAME_RANDOM),
            (0x00b4_3d70, NAME_UNIFORM),
            (0x00b4_3d80, NAME_NORMAL),
            (0x00b4_3d90, NAME_POS_GEN),
            (0x00b4_3da0, NAME_SYM_GEN),
        ] {
            assert_eq!(e.call(address, &args![0u32]).u32(), name);
        }
    }
}
