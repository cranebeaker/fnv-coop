//! The x87 80-bit extended format and its conversion to `f32` and `f64`.
//!
//! The raw form is 10 bytes in memory order: a 64-bit significand with an
//! explicit integer bit, then a 16-bit sign and 15-bit biased exponent.
//! Conversions round to nearest even, as the FPU does with the default
//! rounding mode, and are done with integer arithmetic so they behave the
//! same on every host.

/// An 80-bit extended-precision value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ext80 {
    pub mantissa: u64,
    /// Sign in bit 15, biased exponent (bias 16383) in the low 15 bits.
    pub sign_exp: u16,
}

const EXT_BIAS: i32 = 16383;

impl Ext80 {
    pub fn from_bytes(raw: [u8; 10]) -> Ext80 {
        let mut m = [0u8; 8];
        m.copy_from_slice(&raw[..8]);
        Ext80 {
            mantissa: u64::from_le_bytes(m),
            sign_exp: u16::from_le_bytes([raw[8], raw[9]]),
        }
    }

    pub fn to_bytes(self) -> [u8; 10] {
        let mut out = [0u8; 10];
        out[..8].copy_from_slice(&self.mantissa.to_le_bytes());
        out[8..].copy_from_slice(&self.sign_exp.to_le_bytes());
        out
    }

    /// Parse 20 hex digits (memory order, as `to_hex` prints them).
    pub fn from_hex(text: &str) -> Result<Ext80, String> {
        let bytes = crate::hex::decode(text)?;
        let raw: [u8; 10] = bytes
            .try_into()
            .map_err(|_| format!("an 80-bit value needs 20 hex digits: {text:?}"))?;
        Ok(Ext80::from_bytes(raw))
    }

    pub fn to_hex(self) -> String {
        crate::hex::encode(&self.to_bytes())
    }

    fn sign(self) -> bool {
        self.sign_exp & 0x8000 != 0
    }

    fn exponent(self) -> i32 {
        i32::from(self.sign_exp & 0x7fff)
    }

    /// Exact conversion from `f64`.
    pub fn from_f64(v: f64) -> Ext80 {
        let bits = v.to_bits();
        let sign = ((bits >> 63) as u16) << 15;
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & ((1u64 << 52) - 1);
        if exp == 0x7ff {
            return Ext80 {
                mantissa: (1 << 63) | (frac << 11),
                sign_exp: sign | 0x7fff,
            };
        }
        if exp == 0 {
            if frac == 0 {
                return Ext80 {
                    mantissa: 0,
                    sign_exp: sign,
                };
            }
            let lz = frac.leading_zeros() as i32;
            let e = -1011 - lz;
            return Ext80 {
                mantissa: frac << lz,
                sign_exp: sign | (e + EXT_BIAS) as u16,
            };
        }
        let e = exp - 1023;
        Ext80 {
            mantissa: (1 << 63) | (frac << 11),
            sign_exp: sign | (e + EXT_BIAS) as u16,
        }
    }

    /// Exact conversion from `f32`.
    pub fn from_f32(v: f32) -> Ext80 {
        Ext80::from_f64(f64::from(v))
    }

    /// Round to the target format. `frac_bits`/`exp_bits` are 52/11 for
    /// `f64` and 23/8 for `f32`; the result is the IEEE bit pattern.
    fn round_to(self, frac_bits: u32, exp_bits: u32) -> u64 {
        let sign_bit = u64::from(self.sign()) << (frac_bits + exp_bits);
        let exp_all_ones = (1u64 << exp_bits) - 1;
        let bias = (1i32 << (exp_bits - 1)) - 1;
        let (emax, emin) = (bias, 1 - bias);
        let exp = self.exponent();
        let mant = self.mantissa;

        if exp == 0x7fff {
            let frac64 = mant & ((1 << 63) - 1);
            if frac64 == 0 && mant >> 63 == 1 {
                return sign_bit | (exp_all_ones << frac_bits);
            }
            // NaN (or an invalid pseudo-infinity): keep the top payload
            // bits and force the quiet bit so it cannot turn into infinity.
            let payload = (mant << 1) >> (64 - frac_bits);
            return sign_bit | (exp_all_ones << frac_bits) | payload | (1 << (frac_bits - 1));
        }
        if exp == 0 {
            // Zero, denormal and pseudo-denormal: all far below the target's
            // smallest subnormal, so they round to a signed zero.
            return sign_bit;
        }
        if mant >> 63 == 0 {
            // Unnormal: invalid operand; the FPU's answer is the default NaN.
            return (1u64 << (frac_bits + exp_bits))
                | (exp_all_ones << frac_bits)
                | (1 << (frac_bits - 1));
        }

        let e = exp - EXT_BIAS;
        if e >= emin {
            let shift = 63 - frac_bits;
            let mut q = mant >> shift;
            let rem = mant & ((1u64 << shift) - 1);
            let half = 1u64 << (shift - 1);
            if rem > half || (rem == half && q & 1 == 1) {
                q += 1;
            }
            let mut e = e;
            if q == 1 << (frac_bits + 1) {
                q >>= 1;
                e += 1;
            }
            if e > emax {
                return sign_bit | (exp_all_ones << frac_bits);
            }
            let biased = (e + bias) as u64;
            return sign_bit | (biased << frac_bits) | (q & ((1 << frac_bits) - 1));
        }

        // Subnormal result: value / 2^(emin - frac_bits) = mant >> shift.
        let shift = 63 + emin - e - frac_bits as i32;
        if shift >= 128 {
            return sign_bit;
        }
        let m = u128::from(mant);
        let mut q = (m >> shift) as u64;
        let rem = m & ((1u128 << shift) - 1);
        let half = 1u128 << (shift - 1);
        if rem > half || (rem == half && q & 1 == 1) {
            q += 1;
        }
        // If rounding carried into the smallest normal, q already encodes
        // exponent 1 with a zero fraction.
        sign_bit | q
    }

    /// IEEE bit pattern of the value rounded to `f64`.
    pub fn to_f64_bits(self) -> u64 {
        self.round_to(52, 11)
    }

    /// IEEE bit pattern of the value rounded directly to `f32` (one
    /// rounding step, as `fstp dword` does; not via `f64`).
    pub fn to_f32_bits(self) -> u32 {
        self.round_to(23, 8) as u32
    }

    pub fn to_f64(self) -> f64 {
        f64::from_bits(self.to_f64_bits())
    }

    pub fn to_f32(self) -> f32 {
        f32::from_bits(self.to_f32_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ext(mantissa: u64, sign: bool, exp: u16) -> Ext80 {
        Ext80 {
            mantissa,
            sign_exp: (u16::from(sign) << 15) | exp,
        }
    }

    #[test]
    fn known_values() {
        // 1.0 = integer bit set, exponent 16383.
        let one = ext(1 << 63, false, 0x3fff);
        assert_eq!(one.to_f64(), 1.0);
        assert_eq!(one.to_f32(), 1.0);
        assert_eq!(one.to_hex(), "0000000000000080ff3f");
        // -2.5 = -1.25 * 2^1
        let v = ext(0xA000_0000_0000_0000, true, 0x4000);
        assert_eq!(v.to_f64(), -2.5);
        assert_eq!(Ext80::from_f64(-2.5), v);
        // pi as written by fldpi (significand 0xC90FDAA22168C235, exponent 0x4000).
        let pi = Ext80::from_bytes([0x35, 0xc2, 0x68, 0x21, 0xa2, 0xda, 0x0f, 0xc9, 0x00, 0x40]);
        assert_eq!(pi.to_f64(), std::f64::consts::PI);
        assert_eq!(pi.to_f32(), std::f32::consts::PI);
    }

    #[test]
    fn hex_round_trip() {
        let v = Ext80::from_f64(123.456);
        assert_eq!(Ext80::from_hex(&v.to_hex()).unwrap(), v);
        assert!(Ext80::from_hex("00").is_err());
        assert!(Ext80::from_hex("zz00000000000000000").is_err());
    }

    #[test]
    fn f64_round_trip_is_exact() {
        let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut checked = 0;
        for _ in 0..20000 {
            // xorshift64 for a spread of exponents and fractions.
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let v = f64::from_bits(x);
            if v.is_nan() {
                continue;
            }
            let back = Ext80::from_f64(v).to_f64();
            assert_eq!(back.to_bits(), v.to_bits(), "bits {x:#x}");
            checked += 1;
        }
        assert!(checked > 19000);
        for v in [
            0.0,
            -0.0,
            f64::MIN_POSITIVE,
            5e-324,
            -5e-324,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            assert_eq!(Ext80::from_f64(v).to_f64().to_bits(), v.to_bits());
        }
    }

    #[test]
    fn f32_round_trip_is_exact() {
        for bits in [
            0u32,
            1,
            0x007f_ffff,
            0x0080_0000,
            0x3f80_0000,
            0x7f7f_ffff,
            0x7f80_0000,
            0x8000_0001,
            0xff80_0000,
        ] {
            let v = f32::from_bits(bits);
            assert_eq!(Ext80::from_f32(v).to_f32().to_bits(), bits);
        }
    }

    #[test]
    fn rounds_to_nearest_even_at_f64_precision() {
        // 1.0 + k * 2^-63 for low significand bits around the halfway point
        // (the low 11 bits are dropped when narrowing to 53 bits).
        let base = 1u64 << 63;
        let one_ulp = 1.0f64 + f64::EPSILON;
        // Exactly halfway between 1.0 and 1+eps, ties to even (1.0).
        assert_eq!(ext(base | 0x400, false, 0x3fff).to_f64(), 1.0);
        // Just above halfway rounds up.
        assert_eq!(ext(base | 0x401, false, 0x3fff).to_f64(), one_ulp);
        // Just below halfway rounds down.
        assert_eq!(ext(base | 0x3ff, false, 0x3fff).to_f64(), 1.0);
        // Halfway above an odd value rounds up to the even one.
        assert_eq!(
            ext(base | 0x800 | 0x400, false, 0x3fff).to_f64(),
            1.0 + 2.0 * f64::EPSILON
        );
        // Carry out of the significand bumps the exponent: 2 - 2^-64 -> 2.
        assert_eq!(ext(u64::MAX, false, 0x3fff).to_f64(), 2.0);
    }

    #[test]
    fn rounds_to_nearest_even_at_f32_precision() {
        let base = 1u64 << 63;
        // Dropped bits: 40. Halfway is 1 << 39.
        let half = 1u64 << 39;
        assert_eq!(ext(base | half, false, 0x3fff).to_f32(), 1.0);
        assert_eq!(
            ext(base | half | 1, false, 0x3fff).to_f32(),
            1.0 + f32::EPSILON
        );
        assert_eq!(
            ext(base | (1 << 40) | half, false, 0x3fff).to_f32(),
            1.0 + 2.0 * f32::EPSILON
        );
        // Narrowing directly differs from narrowing through f64 when the
        // f64 step lands exactly on a tie: 1 + 2^-24 + 2^-60.
        let v = ext(base | half | (1 << 3), false, 0x3fff);
        assert_eq!(v.to_f32(), 1.0 + f32::EPSILON);
        assert_eq!(v.to_f64() as f32, 1.0);
    }

    #[test]
    fn overflow_and_underflow() {
        assert_eq!(ext(1 << 63, false, 0x3fff + 1024).to_f64(), f64::INFINITY);
        assert_eq!(
            ext(1 << 63, true, 0x3fff + 1024).to_f64(),
            f64::NEG_INFINITY
        );
        assert_eq!(ext(1 << 63, false, 0x3fff + 128).to_f32(), f32::INFINITY);
        // Largest finite f64 rounds up to infinity only when the carry hits.
        assert_eq!(Ext80::from_f64(f64::MAX).to_f64(), f64::MAX);
        assert_eq!(ext(u64::MAX, false, 0x3fff + 1023).to_f64(), f64::INFINITY);
        // Smallest f64 subnormal and the halfway point below it.
        assert_eq!(
            ext(1 << 63, false, (0x3fff - 1074) as u16)
                .to_f64()
                .to_bits(),
            1
        );
        assert_eq!(
            ext(1 << 63, false, (0x3fff - 1075) as u16)
                .to_f64()
                .to_bits(),
            0
        ); // tie to even (0)
        assert_eq!(
            ext((1 << 63) | 1, false, (0x3fff - 1075) as u16)
                .to_f64()
                .to_bits(),
            1
        );
        assert_eq!(
            ext(1 << 63, false, (0x3fff - 1200) as u16)
                .to_f64()
                .to_bits(),
            0
        );
        // Subnormal rounding up into the smallest normal.
        let almost = ext(u64::MAX, false, (0x3fff - 1023) as u16);
        assert_eq!(almost.to_f64().to_bits(), f64::MIN_POSITIVE.to_bits());
        // Smallest f32 subnormal.
        assert_eq!(
            ext(1 << 63, false, (0x3fff - 149) as u16)
                .to_f32()
                .to_bits(),
            1
        );
        assert_eq!(
            ext(1 << 63, true, (0x3fff - 200) as u16).to_f32().to_bits(),
            0x8000_0000
        );
    }

    #[test]
    fn special_encodings() {
        assert_eq!(ext(1 << 63, false, 0x7fff).to_f64(), f64::INFINITY);
        assert_eq!(ext(1 << 63, true, 0x7fff).to_f32(), f32::NEG_INFINITY);
        let nan = ext((1 << 63) | (1 << 62) | 0x1234, false, 0x7fff);
        assert!(nan.to_f64().is_nan() && nan.to_f32().is_nan());
        // A signalling NaN (quiet bit clear) is quieted, never infinity.
        let snan = ext((1 << 63) | 1, false, 0x7fff);
        assert!(snan.to_f64().is_nan());
        // The default "real indefinite" NaN: negative, quiet, no payload.
        let indef = ext(0xC000_0000_0000_0000, true, 0x7fff);
        assert_eq!(indef.to_f64_bits(), 0xFFF8_0000_0000_0000);
        assert_eq!(indef.to_f32_bits(), 0xFFC0_0000);
        // Signed zero and denormals.
        assert_eq!(ext(0, true, 0).to_f64().to_bits(), 0x8000_0000_0000_0000);
        assert_eq!(ext(5, false, 0).to_f64().to_bits(), 0);
        // Unnormal (explicit integer bit clear with nonzero exponent).
        assert!(ext(1, false, 0x3fff).to_f64().is_nan());
    }

    #[test]
    fn agrees_with_host_arithmetic_for_exact_products() {
        // 1/3 rounded to 64 bits then to f64 and f32: 0xAAAAAAAAAAAAAAAB * 2^-65.
        let third = ext(0xAAAA_AAAA_AAAA_AAAB, false, (0x3fff - 2) as u16);
        assert_eq!(third.to_f64(), 1.0 / 3.0);
        assert_eq!(third.to_f32(), 1.0f32 / 3.0);
    }
}
