//! `BSMain/run_00c5d2b0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A one-dimensional gradient noise (its fade curve, its interpolation and
//! the noise with its lazily built permutation and gradient tables), the
//! start-up and shut-down of seven `BSMain` registries, and a texture-path
//! helper. All are `cdecl`.
//!
//! x87 note: the noise computes in extended precision and stores floats;
//! the translation computes in `f64` and rounds to `f32` at each `float`
//! store (ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `6.0`, `15.0`, `10.0` (`double`): the fade curve's coefficients.
const SIX: u32 = 0x0103_4208;
const FIFTEEN: u32 = 0x0101_5a38;
const TEN: u32 = 0x0102_0758;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `256.0` (`double`).
const TWO_HUNDRED_FIFTY_SIX: u32 = 0x0102_31d8;
/// `4096.0` (`double`): added to the input before it is split.
const OFFSET: u32 = 0x0101_7a10;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;

/// The noise's tables: 256 permutation entries and their copy (with two
/// more), 256 gradients and their copy (with two more), and the `float`
/// that is 1.0 once they are built.
const PERMUTATION: u32 = 0x0120_3b58;
const PERMUTATION_COPY: u32 = 0x0120_3f58;
const GRADIENTS: u32 = 0x0120_3350;
const GRADIENTS_COPY: u32 = 0x0120_3750;
const TABLES_BUILT: u32 = 0x0120_4360;

/// `_rand` (engine map, `00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// `_ftol2_sse` (`00ec62c0`): the float in ST0 truncated to an integer, the
/// value passed as a leading `f64`.
const FTOL: u32 = 0x00ec_62c0;

/// The byte that is set while the registries are started.
const REGISTRIES_STARTED: u32 = 0x0120_4364;

/// `00af4200(name, buffer, size)`, `00afd270(path, &a, &b)` and
/// `00af6540(1, &a, &b, path)` (`cdecl`, `BSSystem/archive.cpp` in the
/// engine map for the first and last).
const ARCHIVE_GET_PATH: u32 = 0x00af_4200;
const ARCHIVE_SPLIT_PATH: u32 = 0x00af_d270;
const ARCHIVE_LOOKUP: u32 = 0x00af_6540;

/// A float copied through the x87 stack: a signalling NaN comes out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

// Translated from 00c5d2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fade curve `6t^5 - 15t^4 + 10t^3`, rounded to a float.
pub fn fn_00c5d2b0(e: &mut Engine, t: f32) -> f32 {
    let t = t as f64;
    let six: f64 = e.global(SIX);
    let fifteen: f64 = e.global(FIFTEEN);
    let ten: f64 = e.global(TEN);
    let fifth = t * t * t * t * t * six;
    let fourth = t * t * t * t * fifteen;
    let third = t * t * t * ten;
    (fifth - fourth + third) as f32
}

// Translated from 00c5d300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Linear interpolation `(b - a) * t + a`, rounded to a float.
pub fn fn_00c5d300(_e: &mut Engine, t: f32, a: f32, b: f32) -> f32 {
    ((b as f64 - a as f64) * t as f64 + a as f64) as f32
}

/// The word at `table + 4 * index` (the exe's signed index arithmetic).
fn entry_address(table: u32, index: u32) -> u32 {
    table.wrapping_add(index.wrapping_mul(4))
}

/// `rand() % modulus` as the compiler emits it for a power of two
/// (`AND` with the sign bit kept, then the negative fix-up).
fn signed_remainder(value: u32, mask: u32) -> u32 {
    let masked = value & (0x8000_0000 | mask);
    if (masked as i32) < 0 {
        (masked.wrapping_sub(1) | !mask).wrapping_add(1)
    } else {
        masked
    }
}

// Translated from 00c5d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// One-dimensional gradient noise at `x`. The first call (while the
/// "built" float equals 0.0) fills the permutation with 0..256 and the
/// gradients with `(rand() % 512 - 256) / 256`, shuffles the permutation
/// with `rand() % 256`, copies 258 entries of each table (the last two read
/// past the first table, into the copy's start), and sets the float to 1.0.
/// Then `x + 4096` is split into a cell (its low byte and the next) and a
/// fraction `f`; the result is the interpolation, by the fade of `f`, of
/// `f * gradient[perm[cell]]` and `(f - 1) * gradient[perm[cell + 1]]`.
pub fn fn_00c5d320(e: &mut Engine, x: f32) -> f32 {
    let built: f32 = e.global(TABLES_BUILT);
    let zero: f64 = e.global(ZERO);
    // `FCOMP`, `TEST AH,0x44`, `JP`: the tables are built only when the
    // comparison is "equal" (not for unordered).
    if built as f64 == zero {
        let scale: f64 = e.global(TWO_HUNDRED_FIFTY_SIX);
        let mut i = 0u32;
        while (i as i32) < 0x100 {
            e.mem.set_u32(entry_address(PERMUTATION, i), i);
            let random = e.call(RAND, &[]).u32();
            let value = signed_remainder(random, 0x1ff).wrapping_sub(0x100);
            let gradient = (value as i32 as f64 / scale) as f32;
            e.mem.set_f32(entry_address(GRADIENTS, i), gradient);
            i = i.wrapping_add(1);
        }
        loop {
            i = i.wrapping_sub(1);
            if i == 0 {
                break;
            }
            let held = e.mem.u32(entry_address(PERMUTATION, i));
            let random = e.call(RAND, &[]).u32();
            let other = signed_remainder(random, 0xff);
            let moved = e.mem.u32(entry_address(PERMUTATION, other));
            e.mem.set_u32(entry_address(PERMUTATION, i), moved);
            e.mem.set_u32(entry_address(PERMUTATION, other), held);
        }
        i = 0;
        while (i as i32) < 0x102 {
            let value = e.mem.u32(entry_address(PERMUTATION, i));
            e.mem.set_u32(entry_address(PERMUTATION_COPY, i), value);
            let gradient = quiet(e.mem.f32(entry_address(GRADIENTS, i)));
            e.mem.set_f32(entry_address(GRADIENTS_COPY, i), gradient);
            i = i.wrapping_add(1);
        }
        e.set_global(TABLES_BUILT, 1.0f32);
    }
    let offset: f64 = e.global(OFFSET);
    let shifted = (x as f64 + offset) as f32;
    let cell = e.call(FTOL, &args![quiet(shifted) as f64]).u32() & 0xff;
    let next = cell.wrapping_add(1) & 0xff;
    let whole = e.call(FTOL, &args![quiet(shifted) as f64]).i32();
    let fraction = (shifted as f64 - whole as f64) as f32;
    let one: f64 = e.global(ONE);
    let fraction_less_one = (fraction as f64 - one) as f32;
    let fade = fn_00c5d2b0(e, quiet(fraction));
    let index = e.mem.u32(entry_address(PERMUTATION, cell));
    let gradient = e.mem.f32(entry_address(GRADIENTS, index));
    let first = (fraction as f64 * gradient as f64) as f32;
    let index = e.mem.u32(entry_address(PERMUTATION, next));
    let gradient = e.mem.f32(entry_address(GRADIENTS, index));
    let second = (fraction_less_one as f64 * gradient as f64) as f32;
    fn_00c5d300(e, quiet(fade), quiet(first), quiet(second))
}

// Translated from 00c5d4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts seven `BSMain` registries once (`00c65530`, `00c55200`,
/// `00c546c0`, `00c618f0`, `00c5f240`, `00c616c0`, `00c43500`, in that
/// order) and sets the started byte.
pub fn fn_00c5d4e0(e: &mut Engine) {
    if e.global::<u8>(REGISTRIES_STARTED) == 0 {
        for start in [
            0x00c6_5530,
            0x00c5_5200,
            0x00c5_46c0,
            0x00c6_18f0,
            0x00c5_f240,
            0x00c6_16c0,
            0x00c4_3500,
        ] {
            e.call(start, &[]);
        }
        e.set_global(REGISTRIES_STARTED, 1u8);
    }
}

// Translated from 00c5d520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The counterpart of `fn_00c5d4e0`: when started, calls the seven
/// shut-down functions (`00c655d0`, `00c552a0`, `00c54760`, `00c61990`,
/// `00c5f2e0`, `00c61760`, `00c435a0`) and clears the started byte.
pub fn fn_00c5d520(e: &mut Engine) {
    if e.global::<u8>(REGISTRIES_STARTED) != 0 {
        for stop in [
            0x00c6_55d0,
            0x00c5_52a0,
            0x00c5_4760,
            0x00c6_1990,
            0x00c5_f2e0,
            0x00c6_1760,
            0x00c4_35a0,
        ] {
            e.call(stop, &[]);
        }
        e.set_global(REGISTRIES_STARTED, 0u8);
    }
}

// Translated from 00c5d560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5d560 - the /GS cookie check (00ec408c) is not translated
/// Takes the text the first word of `object`'s virtual slot +0x9C result
/// points to, has `00af4200` write a path for it into a 0x104-byte buffer,
/// splits that with `00afd270` into two 8-byte locals and returns
/// `00af6540(1, &first, &second, buffer)`.
pub fn fn_00c5d560(e: &mut Engine, object: Ptr) -> u32 {
    let holder = e.vcall(object.addr(), 0x9c, &[]).u32();
    let name = e.mem.u32(holder);
    e.with_stack(0x104, |e, buffer| {
        e.with_stack(0x10, |e, locals| {
            let first = locals.addr();
            let second = first + 8;
            e.call(ARCHIVE_GET_PATH, &args![name, buffer, 0x104u32]);
            e.call(ARCHIVE_SPLIT_PATH, &args![buffer, first, second]);
            e.call(ARCHIVE_LOOKUP, &args![1u32, first, second, buffer])
                .u32()
        })
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5d2b0, fn_00c5d2b0(f32) -> f32),
        entry!(0x00c5d300, fn_00c5d300(f32, f32, f32) -> f32),
        entry!(0x00c5d320, fn_00c5d320(f32) -> f32),
        entry!(0x00c5d4e0, fn_00c5d4e0()),
        entry!(0x00c5d520, fn_00c5d520()),
        entry!(0x00c5d560, fn_00c5d560(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The constants the noise reads, at their addresses.
    fn constants_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_2000, 0x0101_5000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        for page in [
            0x0102_0000,
            0x0102_3000,
            0x0103_4000,
            0x0120_3000,
            0x0120_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(SIX, 6.0f64);
        e.set_global(FIFTEEN, 15.0f64);
        e.set_global(TEN, 10.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(TWO_HUNDRED_FIFTY_SIX, 256.0f64);
        e.set_global(OFFSET, 4096.0f64);
        e.set_global(ONE, 1.0f64);
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e
    }

    #[test]
    fn fade_curve_values() {
        let mut e = constants_engine();
        assert_eq!(e.call(0x00c5_d2b0, &args![0.0f32]).f32(), 0.0);
        assert_eq!(e.call(0x00c5_d2b0, &args![1.0f32]).f32(), 1.0);
        assert_eq!(e.call(0x00c5_d2b0, &args![0.5f32]).f32(), 0.5);
        let t = 0.25f64;
        let want = (6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3)) as f32;
        assert_eq!(e.call(0x00c5_d2b0, &args![0.25f32]).f32(), want);
    }

    #[test]
    fn interpolation_values() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c5_d300, &args![0.25f32, 2.0f32, 6.0f32]).f32(),
            3.0
        );
        assert_eq!(
            e.call(0x00c5_d300, &args![0.0f32, -1.0f32, 5.0f32]).f32(),
            -1.0
        );
    }

    #[test]
    fn signed_remainder_matches_the_compiler_sequence() {
        assert_eq!(signed_remainder(1000, 0x1ff), 1000 % 512);
        assert_eq!(
            signed_remainder(-1000i32 as u32, 0x1ff),
            (-1000i32 % 512) as u32
        );
        assert_eq!(signed_remainder(-512i32 as u32, 0x1ff), 0);
        assert_eq!(signed_remainder(300, 0xff), 300 % 256);
    }

    #[test]
    fn noise_builds_its_tables_once() {
        let mut e = constants_engine();
        // `rand` returns 0, 1, 2, ...: gradient i is (i % 512 - 256) / 256,
        // the shuffle swaps entry i (255 down to 1) with entry (n % 256).
        let counter = Rc::new(RefCell::new(0u32));
        let c = counter.clone();
        e.register_double(RAND, move |_, _| {
            let n = *c.borrow();
            *c.borrow_mut() += 1;
            ret(n)
        });
        e.call(0x00c5_d320, &args![0.0f32]);
        assert_eq!(*counter.borrow(), 256 + 255);
        assert_eq!(e.global::<f32>(TABLES_BUILT), 1.0);
        assert_eq!(e.mem.f32(GRADIENTS), -1.0);
        assert_eq!(e.mem.f32(GRADIENTS + 4 * 255), -1.0 / 256.0);
        // Reference shuffle.
        let mut perm: Vec<u32> = (0..256).collect();
        for (n, i) in (256u32..).zip((1..256).rev()) {
            perm.swap(i, (n % 256) as usize);
        }
        for (i, value) in perm.iter().enumerate() {
            assert_eq!(e.mem.u32(PERMUTATION + 4 * i as u32), *value);
            assert_eq!(e.mem.u32(PERMUTATION_COPY + 4 * i as u32), *value);
        }
        // The two extra entries come from past the first table: the copy's
        // own first entries.
        assert_eq!(e.mem.u32(PERMUTATION_COPY + 4 * 256), perm[0]);
        assert_eq!(e.mem.u32(PERMUTATION_COPY + 4 * 257), perm[1]);
        // A second call does not rebuild.
        e.call(0x00c5_d320, &args![0.5f32]);
        assert_eq!(*counter.borrow(), 256 + 255);
    }

    #[test]
    fn noise_interpolates_the_two_gradients() {
        let mut e = constants_engine();
        e.set_global(TABLES_BUILT, 1.0f32);
        // perm[i] = i; gradient[n] = n / 8.
        for i in 0..256u32 {
            e.mem.set_u32(PERMUTATION + 4 * i, i);
            e.mem.set_f32(GRADIENTS + 4 * i, i as f32 / 8.0);
        }
        e.mem.set_u32(PERMUTATION + 4 * 256, 0);
        // x = 3.25: 4099.25, cell 4099 & 255 = 3, next 4, f = 0.25.
        let value = e.call(0x00c5_d320, &args![3.25f32]).f32();
        let f = 0.25f64;
        let fade = (6.0 * f.powi(5) - 15.0 * f.powi(4) + 10.0 * f.powi(3)) as f32;
        let a = (0.25f64 * (3.0 / 8.0)) as f32;
        let b = (-0.75f64 * (4.0 / 8.0)) as f32;
        let want = ((b as f64 - a as f64) * fade as f64 + a as f64) as f32;
        assert_eq!(value, want);
        // Cell 255 wraps to 0 for the next gradient.
        e.mem.set_f32(GRADIENTS, 2.0);
        let value = e.call(0x00c5_d320, &args![-3840.5f32]).f32();
        let a = (0.5f64 * (255.0 / 8.0)) as f32;
        let b = (-0.5f64 * 2.0) as f32;
        let want = ((b as f64 - a as f64) * 0.5 + a as f64) as f32;
        assert_eq!(value, want);
        // A NaN "built" flag is unordered: no rebuild either.
        e.set_global(TABLES_BUILT, f32::NAN);
        e.register(RAND, |_, _| panic!("rebuilt"));
        e.call(0x00c5_d320, &args![1.0f32]);
    }

    #[test]
    fn registries_start_and_stop_once() {
        let mut e = Engine::new();
        e.map(0x0120_4000, 0x1000);
        let starts = [
            0x00c6_5530u32,
            0x00c5_5200,
            0x00c5_46c0,
            0x00c6_18f0,
            0x00c5_f240,
            0x00c6_16c0,
            0x00c4_3500,
        ];
        let stops = [
            0x00c6_55d0u32,
            0x00c5_52a0,
            0x00c5_4760,
            0x00c6_1990,
            0x00c5_f2e0,
            0x00c6_1760,
            0x00c4_35a0,
        ];
        for f in starts.iter().chain(stops.iter()) {
            e.register(*f, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00c5_d4e0, &[]);
        e.call(0x00c5_d4e0, &[]);
        let log = e.call_log.take().unwrap();
        let called: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        let mut want = vec![0x00c5_d4e0];
        want.extend(starts);
        want.push(0x00c5_d4e0);
        assert_eq!(called, want);
        assert_eq!(e.global::<u8>(REGISTRIES_STARTED), 1);
        e.call_log = Some(vec![]);
        e.call(0x00c5_d520, &[]);
        e.call(0x00c5_d520, &[]);
        let log = e.call_log.take().unwrap();
        let called: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        let mut want = vec![0x00c5_d520];
        want.extend(stops);
        want.push(0x00c5_d520);
        assert_eq!(called, want);
        assert_eq!(e.global::<u8>(REGISTRIES_STARTED), 0);
    }

    #[test]
    fn texture_path_helper_chains_the_archive_calls() {
        let mut e = Engine::new();
        let mut table = vec![0u32; 0x28];
        table[0x9c / 4] = 0x0050_009c;
        e.put_vtable(0x0090_0000, &table);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x3333);
        e.register_double(0x0050_009c, move |_, _| ret(holder));
        e.register(ARCHIVE_GET_PATH, |_, _| Ret::default());
        e.register(ARCHIVE_SPLIT_PATH, |_, _| Ret::default());
        e.register(ARCHIVE_LOOKUP, |_, _| ret(0xbeef));
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0090_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_d560, &args![object]).u32(), 0xbeef);
        let log = e.call_log.take().unwrap();
        let path = &log[2];
        assert_eq!(path.0, ARCHIVE_GET_PATH);
        assert_eq!((path.1[0], path.1[2]), (0x3333, 0x104));
        let buffer = path.1[1];
        assert_eq!(log[3].0, ARCHIVE_SPLIT_PATH);
        assert_eq!(log[3].1[0], buffer);
        assert_eq!(log[3].1[2], log[3].1[1] + 8);
        assert_eq!(
            log[4],
            (ARCHIVE_LOOKUP, vec![1, log[3].1[1], log[3].1[2], buffer])
        );
    }
}
