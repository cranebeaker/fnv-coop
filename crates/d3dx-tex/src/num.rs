//! The arithmetic D3DX's texture code does, as it does it.
//!
//! The 32-bit `d3dx9_38.dll` computes on the x87 unit, in the precision the calling thread has
//! ([`Precision`]). The game makes its Direct3D 9 device without `D3DCREATE_FPU_PRESERVE`, which
//! sets single precision on that thread (measured with `d3dx-measure control-word`: 0x27f
//! before `CreateDevice`, 0x7f after). Its loading threads start at 0x27f, double precision,
//! and keep it while D3DX and the device work (`d3dx-measure thread-texture`), so the same
//! file gives other bytes there. In single precision every x87 operation rounds to a 24-bit
//! significand: each is the `f32` operation. In double precision each rounds to 53 bits, and
//! only a store to a float rounds to 24, so what the DLL keeps in registers between stores
//! matters ([`Reg`]). Every constant the DLL loads (as a double) is a value an `f32` holds
//! exactly. One operation on two floats followed by a store rounds the same at either
//! precision (53 bits are more than twice 24 plus 2, so rounding twice is the same as once, and
//! truncating twice is truncating once); where the DLL rounds to nearest the code keeps such
//! steps as plain `f32`.
//!
//! The DLL's encoders set the rounding control to "toward zero" (they OR 0xC00 into the control
//! word, keeping its precision) around their loops, for their integer conversions; every add,
//! subtract, multiply and divide in those loops then truncates too. That arithmetic is [`Z`]'s
//! and [`Z53`]'s. Everything else rounds to nearest (`f32`, `f64`).
//!
//! The order of operations is the DLL's: a sum `a + (b + c)` is not `(a + b) + c` in floating
//! point.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// `FISTP` of a 32-bit integer with the rounding control set to truncate: towards zero, and
/// the "integer indefinite" `0x80000000` for a NaN or a value out of range.
#[inline(always)]
pub fn fistp(x: f32) -> i32 {
    if x.is_nan() || x >= 2_147_483_648.0 || x < -2_147_483_648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// `s`, the nearest `f32` to an exact result, made the result rounded towards zero: the one
/// before it when the exact result is nearer zero (`below`; never for an exact zero, an
/// infinity or a NaN, whose errors are zero or NaN). Round-to-nearest is within half a unit, so
/// truncation is `s` or its neighbour towards zero, one step down in magnitude: its bits minus 1.
#[inline(always)]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
fn chop(s: f32, below: bool) -> f32 {
    f32::from_bits(s.to_bits().wrapping_sub(u32::from(below)))
}

/// `a + b` rounded towards zero: the nearest sum and its exact error (TwoSum, exact in `f32`);
/// an error pointing towards zero moves the sum one step that way.
#[inline]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
pub(crate) fn add_z(a: f32, b: f32) -> f32 {
    let s = a + b;
    let bb = s - a;
    let e = (a - (s - bb)) + (b - bb);
    chop(s, e != 0.0 && (e < 0.0) != (s < 0.0))
}

/// `a * b` rounded towards zero (the product of two `f32`s is exact in `f64`).
#[inline]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
pub(crate) fn mul_z(a: f32, b: f32) -> f32 {
    let p = f64::from(a) * f64::from(b);
    let s = p as f32;
    chop(s, f64::from(s).abs() > p.abs())
}

/// `a / b` rounded towards zero: the nearest quotient `q` is too far out when `|q b| > |a|`
/// (`q b` is exact in `f64`).
#[inline]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
pub(crate) fn div_z(a: f32, b: f32) -> f32 {
    let q = a / b;
    chop(q, (f64::from(q) * f64::from(b)).abs() > f64::from(a).abs())
}

/// [`chop`] for `f64`.
#[inline(always)]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
fn chop64(s: f64, below: bool) -> f64 {
    f64::from_bits(s.to_bits().wrapping_sub(u64::from(below)))
}

/// [`add_z`] in double precision (TwoSum is exact in `f64` too).
#[inline]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
pub(crate) fn add_z64(a: f64, b: f64) -> f64 {
    let s = a + b;
    let bb = s - a;
    let e = (a - (s - bb)) + (b - bb);
    chop64(s, e != 0.0 && (e < 0.0) != (s < 0.0))
}

/// `a * b` rounded towards zero in double precision: the nearest product's error is exact
/// (`a * b - p` by one fused multiply-add).
#[inline]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
pub(crate) fn mul_z64(a: f64, b: f64) -> f64 {
    let p = a * b;
    let e = a.mul_add(b, -p);
    chop64(p, e != 0.0 && (e < 0.0) != (p < 0.0))
}

/// `a / b` rounded towards zero in double precision: the remainder `a - q b` of the nearest
/// quotient `q` is exact (one fused multiply-add); `q` is too far out when the remainder over
/// `b` points the other way.
#[inline]
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
pub(crate) fn div_z64(a: f64, b: f64) -> f64 {
    let q = a / b;
    let r = (-q).mul_add(b, a);
    chop64(q, r != 0.0 && ((r < 0.0) != (b < 0.0)) != (q < 0.0))
}

/// The processor's own rounding towards zero (x86-64: SSE with MXCSR's rounding control set to
/// truncate by [`Rz`]). Each operation is one instruction in its own `asm!` block: the compiler
/// can neither fold it with its round-to-nearest assumptions nor move it out of the region the
/// guard sets, since it keeps such blocks in order with the guard's. The functions that use them
/// are not inlined into code that rounds to nearest, and use no other floating point arithmetic.
#[cfg(all(target_arch = "x86_64", not(feature = "software-truncation")))]
mod hw {
    use std::arch::asm;

    const RC_TRUNCATE: u32 = 0x6000;

    /// MXCSR's rounding control set to truncate until dropped.
    /// Not `Send`: the guard restores the rounding control of the thread that set it.
    pub struct Rz(u32, std::marker::PhantomData<*const ()>);

    impl Rz {
        #[inline(always)]
        pub fn new() -> Rz {
            let mut old = 0u32;
            // SAFETY: STMXCSR/LDMXCSR of a local; only the rounding control bits change.
            unsafe {
                asm!("stmxcsr [{p}]", p = in(reg) &mut old, options(nostack));
                let new = old | RC_TRUNCATE;
                asm!("ldmxcsr [{p}]", p = in(reg) &new, options(nostack, readonly));
            }
            Rz(old, std::marker::PhantomData)
        }
    }

    impl Drop for Rz {
        #[inline(always)]
        fn drop(&mut self) {
            // SAFETY: restores the MXCSR read in `new`.
            unsafe { asm!("ldmxcsr [{p}]", p = in(reg) &self.0, options(nostack, readonly)) }
        }
    }

    /// Whether the rounding control truncates now (debug builds check every operation).
    #[cfg(debug_assertions)]
    fn truncating() -> bool {
        let mut m = 0u32;
        // SAFETY: STMXCSR of a local.
        unsafe { asm!("stmxcsr [{p}]", p = in(reg) &mut m, options(nostack)) };
        m & RC_TRUNCATE == RC_TRUNCATE
    }

    macro_rules! op {
        ($name:ident, $ins:literal) => {
            #[inline(always)]
            pub fn $name(a: f32, b: f32) -> f32 {
                #[cfg(debug_assertions)]
                debug_assert!(truncating(), "a truncating operation outside an Rz region");
                let mut x = a;
                // SAFETY: one SSE arithmetic instruction on registers.
                unsafe { asm!(concat!($ins, " {x}, {b}"), x = inout(xmm_reg) x, b = in(xmm_reg) b, options(nomem, nostack)) };
                x
            }
        };
    }
    op!(add, "addss");
    op!(sub, "subss");
    op!(mul, "mulss");
    op!(div, "divss");

    macro_rules! op64 {
        ($name:ident, $ins:literal) => {
            #[inline(always)]
            pub fn $name(a: f64, b: f64) -> f64 {
                #[cfg(debug_assertions)]
                debug_assert!(truncating(), "a truncating operation outside an Rz region");
                let mut x = a;
                // SAFETY: one SSE2 arithmetic instruction on registers.
                unsafe { asm!(concat!($ins, " {x}, {b}"), x = inout(xmm_reg) x, b = in(xmm_reg) b, options(nomem, nostack)) };
                x
            }
        };
    }
    op64!(add64, "addsd");
    op64!(sub64, "subsd");
    op64!(mul64, "mulsd");
    op64!(div64, "divsd");

    /// `CVTSD2SS` under the guard: the double rounded to a float by the rounding control,
    /// truncating.
    #[inline(always)]
    pub fn st64(v: f64) -> f32 {
        #[cfg(debug_assertions)]
        debug_assert!(truncating(), "a truncating store outside an Rz region");
        let x: f32;
        // SAFETY: one SSE2 conversion instruction on registers.
        unsafe {
            asm!("cvtsd2ss {x}, {v}", x = out(xmm_reg) x, v = in(xmm_reg) v, options(nomem, nostack))
        };
        x
    }
}

/// Truncating arithmetic in force for the guard's life: on x86-64 the processor's rounding
/// control (restored when dropped); elsewhere nothing, [`Z`] truncating in software. Every
/// [`Z`] operation runs inside one.
#[cfg(all(target_arch = "x86_64", not(feature = "software-truncation")))]
pub(crate) use hw::Rz;

/// Truncating arithmetic in force for the guard's life (software here: nothing to set).
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
pub(crate) struct Rz(std::marker::PhantomData<*const ()>);

#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
impl Rz {
    #[inline(always)]
    pub fn new() -> Rz {
        Rz(std::marker::PhantomData)
    }
}

impl Default for Rz {
    fn default() -> Rz {
        Rz::new()
    }
}

/// A value computed while the DLL's rounding control truncates: its operators round towards
/// zero. Used only inside an [`Rz`] region.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Z(pub(crate) f32);

#[cfg(all(target_arch = "x86_64", not(feature = "software-truncation")))]
use hw::{add as zadd, div as zdiv, mul as zmul, sub as zsub};

#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zadd(a: f32, b: f32) -> f32 {
    add_z(a, b)
}
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zsub(a: f32, b: f32) -> f32 {
    add_z(a, -b)
}
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zmul(a: f32, b: f32) -> f32 {
    mul_z(a, b)
}
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zdiv(a: f32, b: f32) -> f32 {
    div_z(a, b)
}

impl Add for Z {
    type Output = Z;
    #[inline(always)]
    fn add(self, o: Z) -> Z {
        Z(zadd(self.0, o.0))
    }
}

impl Sub for Z {
    type Output = Z;
    #[inline(always)]
    fn sub(self, o: Z) -> Z {
        Z(zsub(self.0, o.0))
    }
}

impl Mul for Z {
    type Output = Z;
    #[inline(always)]
    fn mul(self, o: Z) -> Z {
        Z(zmul(self.0, o.0))
    }
}

impl Div for Z {
    type Output = Z;
    #[inline(always)]
    fn div(self, o: Z) -> Z {
        Z(zdiv(self.0, o.0))
    }
}

impl Neg for Z {
    type Output = Z;
    #[inline(always)]
    fn neg(self) -> Z {
        Z(-self.0)
    }
}

/// A value computed in double precision while the DLL's rounding control truncates (a thread at
/// 53-bit precision): its operators round towards zero to 53 bits. Used only inside an [`Rz`]
/// region.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Z53(pub(crate) f64);

#[cfg(all(target_arch = "x86_64", not(feature = "software-truncation")))]
use hw::{add64 as zadd64, div64 as zdiv64, mul64 as zmul64, st64 as zst64, sub64 as zsub64};

#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zst64(v: f64) -> f32 {
    st_chop(v)
}

#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zadd64(a: f64, b: f64) -> f64 {
    add_z64(a, b)
}
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zsub64(a: f64, b: f64) -> f64 {
    add_z64(a, -b)
}
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zmul64(a: f64, b: f64) -> f64 {
    mul_z64(a, b)
}
#[cfg(any(not(target_arch = "x86_64"), feature = "software-truncation"))]
#[inline(always)]
fn zdiv64(a: f64, b: f64) -> f64 {
    div_z64(a, b)
}

impl Add for Z53 {
    type Output = Z53;
    #[inline(always)]
    fn add(self, o: Z53) -> Z53 {
        Z53(zadd64(self.0, o.0))
    }
}

impl Sub for Z53 {
    type Output = Z53;
    #[inline(always)]
    fn sub(self, o: Z53) -> Z53 {
        Z53(zsub64(self.0, o.0))
    }
}

impl Mul for Z53 {
    type Output = Z53;
    #[inline(always)]
    fn mul(self, o: Z53) -> Z53 {
        Z53(zmul64(self.0, o.0))
    }
}

impl Div for Z53 {
    type Output = Z53;
    #[inline(always)]
    fn div(self, o: Z53) -> Z53 {
        Z53(zdiv64(self.0, o.0))
    }
}

impl Neg for Z53 {
    type Output = Z53;
    #[inline(always)]
    fn neg(self) -> Z53 {
        Z53(-self.0)
    }
}

/// `FSTP` of a double-precision register to a float with the rounding control truncating: the
/// nearest `f32`, or its neighbour towards zero when that is further out than `v` (a value past
/// the largest float gives the largest float).
#[cfg(any(test, feature = "software-truncation", not(target_arch = "x86_64")))]
#[inline(always)]
pub(crate) fn st_chop(v: f64) -> f32 {
    let s = v as f32;
    if s.is_infinite() && v.is_finite() {
        return f32::MAX.copysign(s);
    }
    if f64::from(s).abs() > v.abs() {
        f32::from_bits(s.to_bits() - 1)
    } else {
        s
    }
}

/// A value in an x87 register, as D3DX's code holds it between loading floats and storing one:
/// operations round to the thread's precision in the rounding the code runs under, a load of a
/// float is exact, and only a store to a float rounds to 24 bits. At 24-bit precision every
/// operation already rounds to a float's 24 bits, so a store changes nothing; at 53 bits what
/// the code keeps in registers and what it stores is the difference.
pub(crate) trait Reg:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    /// `FLD` of a float.
    fn ld(x: f32) -> Self;
    /// `FILD` of an integer (exact: the integers here are small).
    fn int(i: i64) -> Self;
    /// `FSTP` to a float, rounded as this value's operations round.
    fn st(self) -> f32;
    /// A store to a float and its load again in one step: the value rounded to a float, kept in
    /// the register type (for loops that keep stored values in registers).
    #[inline(always)]
    fn round(self) -> Self {
        Self::ld(self.st())
    }
    /// `FISTP` with the rounding control truncating, of a value a float holds (a [`Reg::round`]
    /// one): [`fistp`] of it.
    fn trunc(self) -> i32;
}

impl Reg for f32 {
    #[inline(always)]
    fn ld(x: f32) -> f32 {
        x
    }
    #[inline(always)]
    fn int(i: i64) -> f32 {
        i as f32
    }
    #[inline(always)]
    fn st(self) -> f32 {
        self
    }
    #[inline(always)]
    fn trunc(self) -> i32 {
        fistp(self)
    }
}

impl Reg for Z {
    #[inline(always)]
    fn ld(x: f32) -> Z {
        Z(x)
    }
    #[inline(always)]
    fn int(i: i64) -> Z {
        Z(i as f32)
    }
    #[inline(always)]
    fn st(self) -> f32 {
        self.0
    }
    #[inline(always)]
    fn trunc(self) -> i32 {
        fistp(self.0)
    }
}

impl Reg for f64 {
    #[inline(always)]
    fn ld(x: f32) -> f64 {
        f64::from(x)
    }
    #[inline(always)]
    fn int(i: i64) -> f64 {
        i as f64
    }
    #[inline(always)]
    fn st(self) -> f32 {
        self as f32
    }
    #[inline(always)]
    fn trunc(self) -> i32 {
        fistp(self as f32)
    }
}

impl Reg for Z53 {
    #[inline(always)]
    fn ld(x: f32) -> Z53 {
        Z53(f64::from(x))
    }
    #[inline(always)]
    fn int(i: i64) -> Z53 {
        Z53(i as f64)
    }
    #[inline(always)]
    fn st(self) -> f32 {
        zst64(self.0)
    }
    #[inline(always)]
    fn trunc(self) -> i32 {
        if self.0.is_nan() || self.0 >= 2_147_483_648.0 || self.0 < -2_147_483_648.0 {
            i32::MIN
        } else {
            self.0 as i32
        }
    }
}

/// The x87 precision D3DX computes in: the precision control of the calling thread's x87
/// control word, which D3DX keeps (it changes only the rounding control).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Precision {
    /// 24 bits (control word 0x7f): the thread that made a Direct3D 9 device without
    /// `D3DCREATE_FPU_PRESERVE`, as the game makes its device; the game's main thread from then on.
    #[default]
    Single,
    /// 53 bits (0x27f): a thread as Windows starts it; the game's loading threads
    /// (`BSTaskThread`) when they create textures.
    Double,
}

/// One x87 precision as types: [`Reg`] values rounding to nearest (`N`) and truncating (`T`).
pub(crate) trait Fpu: 'static {
    type N: Reg;
    type T: Reg;
}

/// 24-bit precision: `f32` and [`Z`].
pub(crate) enum P24 {}

/// 53-bit precision: `f64` and [`Z53`].
pub(crate) enum P53 {}

impl Fpu for P24 {
    type N = f32;
    type T = Z;
}

impl Fpu for P53 {
    type N = f64;
    type T = Z53;
}

/// `1/31`, `1/63`, `1/255`, `1/15`, `1/7`, `1/5`, `1/3`, `2/3` as the DLL holds them (each the
/// `f32` nearest the fraction).
pub const INV31: f32 = 1.0 / 31.0;
pub const INV63: f32 = 1.0 / 63.0;
pub const INV255: f32 = 1.0 / 255.0;
pub const INV15: f32 = 1.0 / 15.0;
pub const INV7: f32 = 1.0 / 7.0;
pub const INV5: f32 = 1.0 / 5.0;
pub const THIRD: f32 = 1.0 / 3.0;
pub const TWO_THIRDS: f32 = 2.0 / 3.0;

/// The error diffusion weights (Floyd-Steinberg) the DLL's encoders use.
pub const W7: f32 = 7.0 / 16.0;
pub const W3: f32 = 3.0 / 16.0;
pub const W5: f32 = 5.0 / 16.0;
pub const W1: f32 = 1.0 / 16.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// The constants are the DLL's (its doubles at 0x407c10, 0x407c00, 0x4014f8, 0x407e58,
    /// 0x407ee0, 0x408128, 0x407e70, 0x405bd0), read from its 32-bit `d3dx9_38.dll`.
    #[test]
    fn constants_are_the_dlls() {
        let dll = [
            (INV31, 0x3fa0_8421_0000_0000u64),
            (INV63, 0x3f90_4104_2000_0000),
            (INV255, 0x3f70_1010_2000_0000),
            (INV15, 0x3fb1_1111_2000_0000),
            (INV7, 0x3fc2_4924_a000_0000),
            (INV5, 0x3fc9_9999_a000_0000),
            (THIRD, 0x3fd5_5555_6000_0000),
            (TWO_THIRDS, 0x3fe5_5555_6000_0000),
        ];
        for (ours, theirs) in dll {
            assert_eq!(f64::from(ours).to_bits(), theirs);
        }
    }

    #[test]
    fn fistp_truncates_and_gives_the_indefinite_value() {
        assert_eq!(fistp(2.9), 2);
        assert_eq!(fistp(-2.9), -2);
        assert_eq!(fistp(f32::NAN), i32::MIN);
        assert_eq!(fistp(3e9), i32::MIN);
    }

    /// The processor's truncation (x86-64) and the software one agree, on values around the
    /// awkward places (halfway cases, cancellation, tiny against large, powers of two).
    #[test]
    fn processor_and_software_truncation_agree() {
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let pick = |r: u64| -> f32 {
            match r % 6 {
                0 => f32::from_bits(r as u32 >> 2 | 0x3c00_0000),
                1 => (r % 1000) as f32 / 255.0,
                2 => 2f32.powi((r % 40) as i32 - 20),
                3 => -f32::from_bits((r >> 8) as u32 & 0x3fff_ffff),
                4 => f32::from_bits(0x3f80_0000 + (r % 5) as u32 - 2),
                _ => (r % 64) as f32 / 63.0 - 0.5,
            }
        };
        let pairs: Vec<(f32, f32)> = (0..200_000).map(|_| (pick(next()), pick(next()))).collect();
        // The software answers, rounding to nearest underneath.
        let soft: Vec<[u32; 4]> = pairs
            .iter()
            .map(|&(a, b)| {
                let q = if b != 0.0 { div_z(a, b).to_bits() } else { 0 };
                [
                    add_z(a, b).to_bits(),
                    add_z(a, -b).to_bits(),
                    mul_z(a, b).to_bits(),
                    q,
                ]
            })
            .collect();
        let hard: Vec<[u32; 4]> = {
            let _rz = Rz::new();
            pairs
                .iter()
                .map(|&(a, b)| {
                    let q = if b != 0.0 {
                        (Z(a) / Z(b)).0.to_bits()
                    } else {
                        0
                    };
                    [
                        (Z(a) + Z(b)).0.to_bits(),
                        (Z(a) - Z(b)).0.to_bits(),
                        (Z(a) * Z(b)).0.to_bits(),
                        q,
                    ]
                })
                .collect()
        };
        for (i, (s, h)) in soft.iter().zip(&hard).enumerate() {
            assert_eq!(s, h, "{:?}", pairs[i]);
        }
    }

    /// The same in double precision ([`Z53`]), on products and quotients of floats (what the
    /// DLL's registers hold) and on values with long significands, plus the store to a float.
    #[test]
    fn processor_and_software_truncation_agree_in_double() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let pick = |r: u64| -> f64 {
            match r % 6 {
                0 => f64::from_bits(r >> 2 | 0x3f00_0000_0000_0000),
                1 => (r % 1000) as f64 / 255.0,
                2 => 2f64.powi((r % 80) as i32 - 40),
                // Long significands from 2^-60 to 2^4 (far from the subnormal range, where the
                // x87 registers' wider exponent would differ from both anyway).
                3 => -f64::from_bits(
                    ((r >> 8) & 0x000f_ffff_ffff_ffff) | (0x3c3 + (r >> 2) % 64) << 52,
                ),
                4 => f64::from(f32::from_bits(0x3f80_0000 + (r % 5) as u32 - 2)),
                _ => f64::from((r % 64) as f32 / 63.0) * f64::from(0.2125f32),
            }
        };
        let pairs: Vec<(f64, f64)> = (0..200_000).map(|_| (pick(next()), pick(next()))).collect();
        let soft: Vec<[u64; 4]> = pairs
            .iter()
            .map(|&(a, b)| {
                let q = if b != 0.0 { div_z64(a, b).to_bits() } else { 0 };
                [
                    add_z64(a, b).to_bits(),
                    add_z64(a, -b).to_bits(),
                    mul_z64(a, b).to_bits(),
                    q,
                ]
            })
            .collect();
        let hard: Vec<[u64; 4]> = {
            let _rz = Rz::new();
            pairs
                .iter()
                .map(|&(a, b)| {
                    let q = if b != 0.0 {
                        (Z53(a) / Z53(b)).0.to_bits()
                    } else {
                        0
                    };
                    [
                        (Z53(a) + Z53(b)).0.to_bits(),
                        (Z53(a) - Z53(b)).0.to_bits(),
                        (Z53(a) * Z53(b)).0.to_bits(),
                        q,
                    ]
                })
                .collect()
        };
        // The stores to a float: the processor's conversion under the guard and the software one.
        let values: Vec<f64> = pairs.iter().map(|&(a, b)| a * b).collect();
        let stored: Vec<u32> = {
            let _rz = Rz::new();
            values.iter().map(|&v| Z53(v).st().to_bits()).collect()
        };
        for (v, s) in values.iter().zip(&stored) {
            assert_eq!(st_chop(*v).to_bits(), *s, "{v:?}");
        }
        for (i, (s, h)) in soft.iter().zip(&hard).enumerate() {
            assert_eq!(s, h, "{:?}", pairs[i]);
        }
        // The store: 1 + 2^-24 + 2^-40 truncates to 1, rounds to 1 + 2^-23.
        let v = 1.0 + 2f64.powi(-24) + 2f64.powi(-40);
        assert_eq!(st_chop(v), 1.0);
        assert_eq!(v as f32, f32::from_bits(0x3f80_0001));
        assert_eq!(st_chop(-v), -1.0);
        assert_eq!(st_chop(1e300), f32::MAX);
        assert_eq!(st_chop(0.5), 0.5);
    }

    /// Rounding towards zero, against exact rational arithmetic on the f64 of each pair.
    #[test]
    fn chopped_operations_truncate() {
        let one_plus = f32::from_bits(0x3f80_0001);
        // 1 + 2^-24 is halfway: nearest rounds to even (1), towards zero gives 1.
        assert_eq!(add_z(1.0, 2f32.powi(-24)), 1.0);
        // 1 + 1.5 * 2^-23 is above 1 + 2^-23: nearest gives 1 + 2^-22, towards zero 1 + 2^-23.
        assert_eq!(add_z(1.0, 1.5 * 2f32.powi(-23)), one_plus);
        // 1 - tiny is below 1: towards zero gives the f32 below 1.
        assert_eq!(add_z(1.0, -1e-30), f32::from_bits(0x3f7f_ffff));
        assert_eq!(add_z(-1.0, 1e-30), -f32::from_bits(0x3f7f_ffff));
        // 1/3 truncated is below the nearest 1/3 (which rounds up).
        assert_eq!(div_z(1.0, 3.0), f32::from_bits(0x3eaa_aaaa));
        assert_eq!(div_z(2.0, 3.0), f32::from_bits(0x3f2a_aaaa));
        assert_eq!(div_z(1.0, 4.0), 0.25);
        assert_eq!(mul_z(one_plus, one_plus), f32::from_bits(0x3f80_0002));
        // (1 - 2^-24)^2 = 1 - 2^-23 + 2^-48: towards zero 1 - 2^-23.
        assert_eq!(
            mul_z(f32::from_bits(0x3f7f_ffff), f32::from_bits(0x3f7f_ffff)),
            f32::from_bits(0x3f7f_fffe)
        );
        // 3 * (1/3 rounded up) is above 1: towards zero gives 1, nearest too; 3 * 0x3eaaaaab
        // = 1 + 2^-24 exactly, below the next f32.
        assert_eq!(mul_z(3.0, f32::from_bits(0x3eaa_aaab)), 1.0);
    }
}
