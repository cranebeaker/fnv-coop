//! The random numbers SpeedTreeRT grows trees with: Robert Davies' Newran
//! "Random" generator, read from the game's code.
//!
//! - `Raw` (`00b43310`): a Park–Miller minimal standard generator in
//!   Schrage's form (`seed = 16807 × lo − 2836 × hi` with `hi = seed /
//!   127773`, `lo = seed − 127773 × hi`, plus `0x7fffffff` when not
//!   positive). The seed is kept as a double; the result is the seed
//!   rounded to a float, × 2⁻³¹, rounded to a float.
//! - `Next` (`00b433a0`): Bays–Durham shuffle over a 128-entry table:
//!   `i = trunc(Raw() × 128)`, answer the table's entry `i`, refill it with
//!   `Raw()`.
//! - Seeding (`00b435f0`): the seed (an integer, at least 1: `00b3f160`
//!   turns anything below 2 into 1), then the table filled with 128 `Raw()`.
//! - Uniform between `lo` and `hi` (`00b3f130`): `Next() × (hi − lo) + lo`,
//!   the range a double, the result a float.
//!
//! The generator is one global in the game (`011f9120` the seed, `011f8f20`
//! the table): every tree computed reseeds it, so a tree's numbers depend
//! only on its own seed.

/// The generator's state.
#[derive(Debug, Clone, PartialEq)]
pub struct Random {
    seed: f64,
    table: [f32; 128],
}

impl Random {
    /// Seeded as `00b3f160` does with a tree's seed: below 2 counts as 1.
    pub fn new(seed: i32) -> Self {
        let mut r = Random {
            seed: 0.0,
            table: [0.0; 128],
        };
        r.reseed(seed);
        r
    }

    /// `00b3f160` with a seed other than −1 (the time-based one).
    pub fn reseed(&mut self, seed: i32) {
        let seed = if seed < 2 { 1 } else { seed };
        self.seed = seed as f64;
        for i in 0..128 {
            self.table[i] = self.raw();
        }
    }

    /// `00b43310`.
    fn raw(&mut self) -> f32 {
        // `_ftol` truncates; the seed is always a whole number here.
        let mut iseed = self.seed as i32;
        let hi = iseed / 127773;
        let lo = iseed - hi * 127773;
        iseed = lo.wrapping_mul(16807).wrapping_sub(hi.wrapping_mul(2836));
        if iseed <= 0 {
            iseed = iseed.wrapping_add(0x7fff_ffff);
        }
        self.seed = iseed as f64;
        let f = self.seed as f32;
        (f as f64 * (1.0 / 2_147_483_648.0)) as f32
    }

    /// `00b433a0`: the next number, 0 ≤ x < 1.
    pub fn next_float(&mut self) -> f32 {
        let i = (self.raw() as f64 * 128.0) as i32 as usize;
        let f = self.table[i];
        self.table[i] = self.raw();
        f
    }

    /// `00b3f130`: uniform between `lo` and `hi` (either order).
    pub fn uniform(&mut self, lo: f32, hi: f32) -> f32 {
        let range = hi as f64 - lo as f64;
        (self.next_float() as f64 * range + lo as f64) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn park_miller_steps() {
        // The minimal standard generator from seed 1 gives 16807, then
        // 282475249 (Park and Miller's published sequence).
        let mut r = Random {
            seed: 1.0,
            table: [0.0; 128],
        };
        r.raw();
        assert_eq!(r.seed, 16807.0);
        r.raw();
        assert_eq!(r.seed, 282_475_249.0);
        // The 10,000th value from seed 1 is 1043618065 (their check).
        let mut r = Random {
            seed: 1.0,
            table: [0.0; 128],
        };
        for _ in 0..10_000 {
            r.raw();
        }
        assert_eq!(r.seed, 1_043_618_065.0);
    }

    #[test]
    fn uniform_stays_in_range() {
        let mut r = Random::new(171_677);
        for _ in 0..1000 {
            let v = r.uniform(-180.0, 180.0);
            assert!((-180.0..180.0).contains(&v));
        }
        // Seeds below 2 all mean 1.
        let a: Vec<f32> = (0..5).map(|_| Random::new(0).next_float()).collect();
        let b: Vec<f32> = (0..5).map(|_| Random::new(1).next_float()).collect();
        assert_eq!(a, b);
    }
}
