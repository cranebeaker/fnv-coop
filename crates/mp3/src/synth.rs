//! The polyphase synthesis filterbank (ISO 11172-3 Annex A, Figure A.2):
//! 32 subband samples in, 32 output samples out, per time slot.

use std::sync::OnceLock;

use crate::tables::computed;

/// The synthesis window's prototype `h[0..=256]` in units of 2^-16 (the
/// standard's Table B.3 `D[i]` are all whole multiples of 2^-16; the other
/// half mirrors this one). Checked value for value against the window in
/// Windows' Fraunhofer MP3 codec (`l3codeca.acm`).
#[rustfmt::skip]
const WINDOW_BASE: [i32; 257] = [
         0,     -1,     -1,     -1,     -1,     -1,     -1,     -2,     -2,     -2,
        -2,     -3,     -3,     -4,     -4,     -5,     -5,     -6,     -7,     -7,
        -8,     -9,    -10,    -11,    -13,    -14,    -16,    -17,    -19,    -21,
       -24,    -26,    -29,    -31,    -35,    -38,    -41,    -45,    -49,    -53,
       -58,    -63,    -68,    -73,    -79,    -85,    -91,    -97,   -104,   -111,
      -117,   -125,   -132,   -139,   -147,   -154,   -161,   -169,   -176,   -183,
      -190,   -196,   -202,   -208,   -213,   -218,   -222,   -225,   -227,   -228,
      -228,   -227,   -224,   -221,   -215,   -208,   -200,   -189,   -177,   -163,
      -146,   -127,   -106,    -83,    -57,    -29,      2,     36,     72,    111,
       153,    197,    244,    294,    347,    401,    459,    519,    581,    645,
       711,    779,    848,    919,    991,   1064,   1137,   1210,   1283,   1356,
      1428,   1498,   1567,   1634,   1698,   1759,   1817,   1870,   1919,   1962,
      2001,   2032,   2057,   2075,   2085,   2087,   2080,   2063,   2037,   2000,
      1952,   1893,   1822,   1739,   1644,   1535,   1414,   1280,   1131,    970,
       794,    605,    402,    185,    -45,   -288,   -545,   -814,  -1095,  -1388,
     -1692,  -2006,  -2330,  -2663,  -3004,  -3351,  -3705,  -4063,  -4425,  -4788,
     -5153,  -5517,  -5879,  -6237,  -6589,  -6935,  -7271,  -7597,  -7910,  -8209,
     -8491,  -8755,  -8998,  -9219,  -9416,  -9585,  -9727,  -9838,  -9916,  -9959,
     -9966,  -9935,  -9863,  -9750,  -9592,  -9389,  -9139,  -8840,  -8492,  -8092,
     -7640,  -7134,  -6574,  -5959,  -5288,  -4561,  -3776,  -2935,  -2037,  -1082,
       -70,    998,   2122,   3300,   4533,   5818,   7154,   8540,   9975,  11455,
     12980,  14548,  16155,  17799,  19478,  21189,  22929,  24694,  26482,  28289,
     30112,  31947,  33791,  35640,  37489,  39336,  41176,  43006,  44821,  46617,
     48390,  50137,  51853,  53534,  55178,  56778,  58333,  59838,  61289,  62684,
     64019,  65290,  66494,  67629,  68692,  69679,  70590,  71420,  72169,  72835,
     73415,  73908,  74313,  74630,  74856,  74992,  75038,
];

/// The standard's synthesis window `D[0..512]`: the prototype, mirrored
/// about 256, with the sign flipped in every other run of 64.
pub fn synthesis_window() -> &'static [f64; 512] {
    static WINDOW: OnceLock<[f64; 512]> = OnceLock::new();
    WINDOW.get_or_init(|| {
        let mut d = [0.0; 512];
        for (i, v) in d.iter_mut().enumerate() {
            let base = WINDOW_BASE[if i <= 256 { i } else { 512 - i }];
            let sign = if (i / 64) % 2 == 1 { -1.0 } else { 1.0 };
            *v = sign * f64::from(base) / 65536.0;
        }
        d
    })
}

/// One channel's filterbank state: the last 1,024 matrixed values `V`, as
/// a ring (`pos` is where `V[0]` is).
#[derive(Clone)]
pub(crate) struct Synth {
    v: [f64; 1024],
    pos: usize,
}

impl Default for Synth {
    fn default() -> Self {
        Synth {
            v: [0.0; 1024],
            pos: 0,
        }
    }
}

impl Synth {
    /// Turns one time slot's 32 subband samples into 32 output samples.
    pub fn run(&mut self, subbands: &[f64; 32], out: &mut [f32]) {
        let matrix = &computed().matrix;
        // Shift V by 64 (V[i] = V[i - 64]) by moving where V[0] is.
        self.pos = (self.pos + 1024 - 64) & 1023;
        // Matrixing: V[i] = sum over k of cos((16 + i)(2k + 1) pi / 64) S[k].
        // Rows 17..=32 are the negatives of rows 15..=0 and rows 49..=63
        // repeat rows 47..=33, so only 33 rows are summed.
        let mut v = [0.0f64; 64];
        for (row, coefficients) in matrix.iter().enumerate() {
            let i = if row <= 16 { row } else { row + 16 };
            v[i] = coefficients.iter().zip(subbands).map(|(c, s)| c * s).sum();
        }
        for i in 17..=32 {
            v[i] = -v[32 - i];
        }
        for i in 49..64 {
            v[i] = v[96 - i];
        }
        for (i, value) in v.iter().enumerate() {
            self.v[(self.pos + i) & 1023] = *value;
        }
        // Windowing and summing: U takes V[128i + j] and V[128i + 96 + j]
        // for the 8 blocks i; sample j = sum of U[j + 32k] D[j + 32k].
        let d = synthesis_window();
        for (j, sample) in out.iter_mut().enumerate().take(32) {
            let mut sum = 0.0;
            for i in 0..8 {
                sum += self.v[(self.pos + 128 * i + j) & 1023] * d[64 * i + j];
                sum += self.v[(self.pos + 128 * i + 96 + j) & 1023] * d[64 * i + 32 + j];
            }
            *sample = sum as f32;
        }
    }
}

#[cfg(test)]
pub(crate) mod reference {
    use super::synthesis_window;
    use std::f64::consts::PI;

    /// The filterbank written out as the standard's flowchart: shift a
    /// plain 1,024-value V, matrix with the full 64 x 32 cosines, build U,
    /// window and sum.
    pub struct Flowchart {
        v: Vec<f64>,
    }

    impl Default for Flowchart {
        fn default() -> Self {
            Flowchart { v: vec![0.0; 1024] }
        }
    }

    impl Flowchart {
        pub fn run(&mut self, s: &[f64; 32]) -> [f64; 32] {
            for i in (64..1024).rev() {
                self.v[i] = self.v[i - 64];
            }
            for i in 0..64 {
                self.v[i] = (0..32)
                    .map(|k| ((16 + i) as f64 * (2 * k + 1) as f64 * PI / 64.0).cos() * s[k])
                    .sum();
            }
            let mut u = [0.0; 512];
            for i in 0..8 {
                for j in 0..32 {
                    u[64 * i + j] = self.v[128 * i + j];
                    u[64 * i + 32 + j] = self.v[128 * i + 96 + j];
                }
            }
            let d = synthesis_window();
            let mut out = [0.0; 32];
            for (j, o) in out.iter_mut().enumerate() {
                *o = (0..16).map(|i| u[j + 32 * i] * d[j + 32 * i]).sum();
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reference::Flowchart;
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn matches_the_standards_flowchart() {
        let mut fast = Synth::default();
        let mut slow = Flowchart::default();
        let mut seed = 12345u32;
        for _ in 0..40 {
            let mut s = [0.0; 32];
            for x in s.iter_mut() {
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
                *x = f64::from(seed >> 8) / f64::from(1u32 << 24) - 0.5;
            }
            let expected = slow.run(&s);
            let mut got = [0.0f32; 32];
            fast.run(&s, &mut got);
            for (g, e) in got.iter().zip(expected) {
                assert!((f64::from(*g) - e).abs() < 1e-6, "{g} vs {e}");
            }
        }
    }

    #[test]
    fn window_matches_the_standards_table() {
        let d = synthesis_window();
        // A few entries as printed in ISO 11172-3 Table B.3.
        assert_eq!(d[0], 0.0);
        assert!((d[1] - -0.000015259).abs() < 1e-9);
        assert!((d[64] - 0.003250122).abs() < 1e-9);
        assert!((d[128] - 0.031082153).abs() < 1e-9);
        assert!((d[192] - 0.100311279).abs() < 1e-9);
        assert!((d[256] - 1.144989014).abs() < 1e-9);
        assert!((d[257] - 1.144287109).abs() < 1e-9);
        assert!((d[511] - 0.000015259).abs() < 1e-9);
        // The prototype is smooth: no entry sticks out from its
        // neighbours (a mistyped value would).
        let h: Vec<f64> = WINDOW_BASE.iter().map(|&v| f64::from(v)).collect();
        for i in 2..255 {
            let second = h[i - 1] - 2.0 * h[i] + h[i + 1];
            let nearby = (h[i - 2] - 2.0 * h[i - 1] + h[i]).abs().max(8.0);
            assert!(second.abs() < 3.0 * nearby, "window entry {i}");
        }
    }

    /// Each subband's synthesis filter passes its own 1/32 of the
    /// spectrum: feeding one subband a burst puts nearly all the output's
    /// energy between k/64 and (k + 1)/64 of the sample rate. Wrong signs
    /// or values in the window would leak it elsewhere.
    #[test]
    fn subbands_land_in_their_own_frequency_range() {
        for k in [0usize, 1, 7, 16, 31] {
            let mut synth = Synth::default();
            let mut out = Vec::new();
            for t in 0..64 {
                let mut s = [0.0; 32];
                // A tone at the middle of the subband, faded in and out.
                let fade = 0.5 - 0.5 * (2.0 * PI * t as f64 / 64.0).cos();
                s[k] = (PI / 2.0 * t as f64).cos() * fade;
                let mut block = [0.0f32; 32];
                synth.run(&s, &mut block);
                out.extend(block.iter().map(|&v| f64::from(v)));
            }
            let n = out.len();
            let (mut inside, mut total) = (0.0, 0.0);
            for bin in 0..n / 2 {
                let (mut re, mut im) = (0.0, 0.0);
                for (t, x) in out.iter().enumerate() {
                    let a = 2.0 * PI * (bin * t) as f64 / n as f64;
                    re += x * a.cos();
                    im -= x * a.sin();
                }
                let power = re * re + im * im;
                total += power;
                let f = bin as f64 / n as f64;
                let (lo, hi) = (k as f64 / 64.0, (k + 1) as f64 / 64.0);
                if f >= lo - 0.004 && f <= hi + 0.004 {
                    inside += power;
                }
            }
            assert!(inside / total > 0.99, "subband {k}: {}", inside / total);
        }
    }
}
