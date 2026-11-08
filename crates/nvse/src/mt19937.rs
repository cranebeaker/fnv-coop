//! MT19937, the Mersenne Twister of Matsumoto and Nishimura, as xNVSE carries it
//! (`Utilities.cpp`, namespace `MersenneTwister` (xNVSE)): the generator `Rand` and `PickOneOf`
//! draw from, seeded with `GetTickCount()` when xNVSE loads (`nvse.cpp`). Its BSD 3-clause
//! licence is in `LICENSE-mt19937` beside this crate's `Cargo.toml`.
//!
//! Translated from the reference C code "MT19937, with initialization improved 2002/1/26",
//! Copyright (C) 1997 - 2002, Makoto Matsumoto and Takuji Nishimura, All rights reserved.

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908_b0df;
const UPPER_MASK: u32 = 0x8000_0000;
const LOWER_MASK: u32 = 0x7fff_ffff;

/// The generator's state (`mt`, `mti`; `mti == N + 1` until it is seeded).
#[derive(Clone)]
pub struct Mt19937 {
    mt: [u32; N],
    mti: usize,
}

impl Default for Mt19937 {
    fn default() -> Self {
        Mt19937 {
            mt: [0; N],
            mti: N + 1,
        }
    }
}

impl Mt19937 {
    // Translated from MT19937 (in xNVSE's Utilities.cpp): init_genrand.
    pub fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..N {
            let prev = self.mt[i - 1];
            self.mt[i] = 1_812_433_253u32
                .wrapping_mul(prev ^ (prev >> 30))
                .wrapping_add(i as u32);
        }
        self.mti = N;
    }

    // Translated from MT19937 (in xNVSE's Utilities.cpp): genrand_int32.
    pub fn genrand_int32(&mut self) -> u32 {
        let mag01 = |y: u32| if y & 1 != 0 { MATRIX_A } else { 0 };
        if self.mti >= N {
            if self.mti == N + 1 {
                self.init_genrand(5489);
            }
            let mt = &mut self.mt;
            for kk in 0..N - M {
                let y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
                mt[kk] = mt[kk + M] ^ (y >> 1) ^ mag01(y);
            }
            for kk in N - M..N - 1 {
                let y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
                mt[kk] = mt[kk + M - N] ^ (y >> 1) ^ mag01(y);
            }
            let y = (mt[N - 1] & UPPER_MASK) | (mt[0] & LOWER_MASK);
            mt[N - 1] = mt[M - 1] ^ (y >> 1) ^ mag01(y);
            self.mti = 0;
        }
        let mut y = self.mt[self.mti];
        self.mti += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;
        y
    }

    // Translated from MT19937 (in xNVSE's Utilities.cpp): genrand_real2.
    /// A number in `[0, 1)`: the 32 bits divided by 2^32.
    pub fn genrand_real2(&mut self) -> f64 {
        f64::from(self.genrand_int32()) * (1.0 / 4_294_967_296.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reference implementation's first outputs for `init_genrand(5489)` (its default seed).
    #[test]
    fn the_default_seed_gives_the_reference_sequence() {
        let mut mt = Mt19937::default();
        let first: Vec<u32> = (0..3).map(|_| mt.genrand_int32()).collect();
        assert_eq!(first, [3_499_211_612, 581_869_302, 3_890_346_734]);
        // The 10000th output of the default seed (the C++11 `mt19937` check value).
        let mut mt = Mt19937::default();
        let last = (0..10_000).map(|_| mt.genrand_int32()).last();
        assert_eq!(last, Some(4_123_659_995));
    }
}
