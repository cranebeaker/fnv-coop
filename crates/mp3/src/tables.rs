//! Fixed tables of Layer III: scale factor bands, scale factor sizes, and
//! values worked out once (powers, cosines, windows).
//!
//! The band tables are ISO 11172-3 Table B.8 and ISO 13818-3 Table B.2
//! (MPEG-2.5's as Fraunhofer published them); they, `PRETAB`, `SLEN` and
//! `NR_OF_SFB` were checked value for value against the tables inside
//! Windows' own MP3 decoders.

use std::f64::consts::PI;
use std::sync::OnceLock;

use crate::header::{FrameHeader, Version};

/// Scale factor band edges, in spectral lines: 22 long bands (576 lines)
/// and 13 short bands (192 lines per window).
pub(crate) struct Bands {
    pub long: [usize; 23],
    pub short: [usize; 14],
}

const fn bands(long: [usize; 23], short: [usize; 14]) -> Bands {
    Bands { long, short }
}

const LONG_44: [usize; 23] = [
    0, 4, 8, 12, 16, 20, 24, 30, 36, 44, 52, 62, 74, 90, 110, 134, 162, 196, 238, 288, 342, 418,
    576,
];
const LONG_48: [usize; 23] = [
    0, 4, 8, 12, 16, 20, 24, 30, 36, 42, 50, 60, 72, 88, 106, 128, 156, 190, 230, 276, 330, 384,
    576,
];
const LONG_32: [usize; 23] = [
    0, 4, 8, 12, 16, 20, 24, 30, 36, 44, 54, 66, 82, 102, 126, 156, 194, 240, 296, 364, 448, 550,
    576,
];
const LONG_22: [usize; 23] = [
    0, 6, 12, 18, 24, 30, 36, 44, 54, 66, 80, 96, 116, 140, 168, 200, 238, 284, 336, 396, 464, 522,
    576,
];
const LONG_24: [usize; 23] = [
    0, 6, 12, 18, 24, 30, 36, 44, 54, 66, 80, 96, 114, 136, 162, 194, 232, 278, 332, 394, 464, 540,
    576,
];
const LONG_8: [usize; 23] = [
    0, 12, 24, 36, 48, 60, 72, 88, 108, 132, 160, 192, 232, 280, 336, 400, 476, 566, 568, 570, 572,
    574, 576,
];
const SHORT_44: [usize; 14] = [0, 4, 8, 12, 16, 22, 30, 40, 52, 66, 84, 106, 136, 192];
const SHORT_48: [usize; 14] = [0, 4, 8, 12, 16, 22, 28, 38, 50, 64, 80, 100, 126, 192];
const SHORT_32: [usize; 14] = [0, 4, 8, 12, 16, 22, 30, 42, 58, 78, 104, 138, 180, 192];
const SHORT_22: [usize; 14] = [0, 4, 8, 12, 18, 24, 32, 42, 56, 74, 100, 132, 174, 192];
const SHORT_24: [usize; 14] = [0, 4, 8, 12, 18, 26, 36, 48, 62, 80, 104, 136, 180, 192];
const SHORT_16: [usize; 14] = [0, 4, 8, 12, 18, 26, 36, 48, 62, 80, 104, 134, 174, 192];
const SHORT_8: [usize; 14] = [0, 8, 16, 24, 36, 52, 72, 96, 124, 160, 162, 164, 166, 192];

/// By version (MPEG-1, 2, 2.5) and sample rate index.
static BANDS: [[Bands; 3]; 3] = [
    [
        bands(LONG_44, SHORT_44),
        bands(LONG_48, SHORT_48),
        bands(LONG_32, SHORT_32),
    ],
    [
        bands(LONG_22, SHORT_22),
        bands(LONG_24, SHORT_24),
        bands(LONG_22, SHORT_16),
    ],
    [
        bands(LONG_22, SHORT_16),
        bands(LONG_22, SHORT_16),
        bands(LONG_8, SHORT_8),
    ],
];

pub(crate) fn bands_for(header: &FrameHeader) -> &'static Bands {
    let v = match header.version {
        Version::Mpeg1 => 0,
        Version::Mpeg2 => 1,
        Version::Mpeg25 => 2,
    };
    &BANDS[v][usize::from(header.sample_rate_index)]
}

/// How many long bands a mixed block starts with: 8 in MPEG-1, 6 in
/// MPEG-2 and 2.5. Either way they end where short band 3 starts (line 36,
/// or 72 at 8 kHz).
pub(crate) fn mixed_long_bands(lsf: bool) -> usize {
    if lsf {
        6
    } else {
        8
    }
}

/// Extra amplification of the high long bands when `preflag` is set.
pub(crate) const PRETAB: [i32; 22] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3, 2, 0,
];

/// MPEG-1 scale factor sizes in bits by `scalefac_compress`: the lower
/// bands' (`slen1`) and the upper bands' (`slen2`).
pub(crate) const SLEN: [[u32; 16]; 2] = [
    [0, 0, 0, 0, 3, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4],
    [0, 1, 2, 3, 0, 1, 2, 3, 1, 2, 3, 1, 2, 3, 2, 3],
];

/// MPEG-2 scale factors: how many there are in each of the four groups,
/// by table (0-2 ordinary, 3-5 the right channel under intensity stereo)
/// and block kind (long, short, mixed). ISO 13818-3 Table B.1
/// ("nr_of_sfb_block").
pub(crate) const NR_OF_SFB: [[[usize; 4]; 3]; 6] = [
    [[6, 5, 5, 5], [9, 9, 9, 9], [6, 9, 9, 9]],
    [[6, 5, 7, 3], [9, 9, 12, 6], [6, 9, 12, 6]],
    [[11, 10, 0, 0], [18, 18, 0, 0], [15, 18, 0, 0]],
    [[7, 7, 7, 0], [12, 12, 12, 0], [6, 15, 12, 0]],
    [[6, 6, 6, 3], [12, 9, 9, 6], [6, 12, 9, 6]],
    [[8, 8, 5, 0], [15, 12, 9, 0], [6, 18, 9, 0]],
];

/// The alias reduction butterflies' coefficients (ISO 11172-3 Table B.9):
/// `cs = 1 / sqrt(1 + c^2)`, `ca = c / sqrt(1 + c^2)`.
const ALIAS_C: [f64; 8] = [
    -0.6, -0.535, -0.33, -0.185, -0.095, -0.041, -0.0142, -0.0037,
];

/// Values the decoder works out once.
pub(crate) struct Computed {
    /// `n^(4/3)` for every value Huffman decoding can give (0..=8206).
    pub pow43: Vec<f64>,
    pub alias_cs: [f64; 8],
    pub alias_ca: [f64; 8],
    /// The 36-point IMDCT's distinct rows: outputs 0..9 then 18..27, each
    /// `cos(pi/72 * (2i + 1 + 18) * (2k + 1))` for the 18 inputs.
    pub imdct36: [[f64; 18]; 18],
    /// The 12-point IMDCT's distinct rows: outputs 0..3 then 6..9.
    pub imdct12: [[f64; 6]; 6],
    /// Long windows by block type (0 normal, 1 start, 3 stop; 2 unused).
    pub long_windows: [[f64; 36]; 4],
    pub short_window: [f64; 12],
    /// The synthesis matrixing rows `cos((16 + i)(2k + 1) pi / 64)` for
    /// i in 0..=16 (rows 0..17) and 33..=48 (rows 17..33); the other rows
    /// follow by symmetry.
    pub matrix: [[f64; 32]; 33],
}

pub(crate) fn computed() -> &'static Computed {
    static COMPUTED: OnceLock<Computed> = OnceLock::new();
    COMPUTED.get_or_init(|| {
        let pow43 = (0..8207).map(|n| f64::from(n).powf(4.0 / 3.0)).collect();
        let mut alias_cs = [0.0; 8];
        let mut alias_ca = [0.0; 8];
        for (i, &c) in ALIAS_C.iter().enumerate() {
            let s = (1.0 + c * c).sqrt();
            alias_cs[i] = 1.0 / s;
            alias_ca[i] = c / s;
        }
        let mut imdct36 = [[0.0; 18]; 18];
        for (row, out) in imdct36.iter_mut().enumerate() {
            let i = if row < 9 { row } else { row + 9 } as f64;
            for (k, v) in out.iter_mut().enumerate() {
                *v = (PI / 72.0 * (2.0 * i + 19.0) * (2.0 * k as f64 + 1.0)).cos();
            }
        }
        let mut imdct12 = [[0.0; 6]; 6];
        for (row, out) in imdct12.iter_mut().enumerate() {
            let i = if row < 3 { row } else { row + 3 } as f64;
            for (k, v) in out.iter_mut().enumerate() {
                *v = (PI / 24.0 * (2.0 * i + 7.0) * (2.0 * k as f64 + 1.0)).cos();
            }
        }
        let long = |i: usize| (PI / 36.0 * (i as f64 + 0.5)).sin();
        let short = |i: usize| (PI / 12.0 * (i as f64 + 0.5)).sin();
        // Normal, start (long to short), unused, stop (short to long).
        let long_windows = [
            std::array::from_fn(long),
            std::array::from_fn(|i| match i {
                0..=17 => long(i),
                18..=23 => 1.0,
                24..=29 => short(i - 18),
                _ => 0.0,
            }),
            [0.0; 36],
            std::array::from_fn(|i| match i {
                0..=5 => 0.0,
                6..=11 => short(i - 6),
                12..=17 => 1.0,
                _ => long(i),
            }),
        ];
        let short_window = std::array::from_fn(short);
        let mut matrix = [[0.0; 32]; 33];
        for (row, out) in matrix.iter_mut().enumerate() {
            let i = if row <= 16 { row } else { row + 16 } as f64;
            for (k, v) in out.iter_mut().enumerate() {
                *v = ((16.0 + i) * (2.0 * k as f64 + 1.0) * PI / 64.0).cos();
            }
        }
        Computed {
            pow43,
            alias_cs,
            alias_ca,
            imdct36,
            imdct12,
            long_windows,
            short_window,
            matrix,
        }
    })
}

/// `2^(quarters / 4)`.
pub(crate) fn pow2_quarter(quarters: i32) -> f64 {
    const QUARTER: [f64; 4] = [
        1.0,
        1.189_207_115_002_721,
        std::f64::consts::SQRT_2,
        1.681_792_830_507_429,
    ];
    QUARTER[quarters.rem_euclid(4) as usize] * 2f64.powi(quarters.div_euclid(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_tables_cover_the_spectrum() {
        for (v, version) in BANDS.iter().enumerate() {
            for b in version {
                assert_eq!((b.long[0], b.long[22]), (0, 576));
                assert_eq!((b.short[0], b.short[13]), (0, 192));
                assert!(b.long.windows(2).all(|w| w[0] < w[1]));
                assert!(b.short.windows(2).all(|w| w[0] < w[1]));
                // Mixed blocks switch from long to short bands at one line.
                let end = b.long[mixed_long_bands(v > 0)];
                assert_eq!(end, 3 * b.short[3]);
                assert_eq!(end % 18, 0);
            }
        }
    }

    #[test]
    fn mpeg2_scale_factor_counts_add_up() {
        for table in &NR_OF_SFB {
            assert_eq!(table[0].iter().sum::<usize>(), 21);
            assert_eq!(table[1].iter().sum::<usize>(), 36);
            assert_eq!(table[2].iter().sum::<usize>(), 33);
        }
    }

    #[test]
    fn quarter_powers() {
        for q in -40..40 {
            let expected = 2f64.powf(f64::from(q) / 4.0);
            assert!((pow2_quarter(q) / expected - 1.0).abs() < 1e-14, "{q}");
        }
    }
}
