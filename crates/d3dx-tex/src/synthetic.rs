//! Synthetic DDS files (built here from the documented layout, no game files) for checking this
//! crate against the 32-bit `d3dx9_38.dll`: `scripts/d3dx-tex-verify --capture-synthetic` runs
//! them through the DLL once and writes `data/synthetic_32bit.tsv`, which the tests compare with.
//! They cover what the game's textures need: DXT1/3/5 with the whole chain in the file (levels
//! under a block re-encoded with dither), with one level (the chain made by the box filter),
//! non-square sizes, transparent DXT1 pixels, A8R8G8B8 and A4R4G4B4 chains (integer and float box
//! paths), R8G8B8 converted to X8R8G8B8 with ordered dither, a DXT1 cube map, and files D3DX
//! refuses.

/// A file: its name, whether it goes to the cube call, and its bytes.
pub struct File {
    pub name: String,
    pub cube: bool,
    pub data: Vec<u8>,
}

/// xorshift64: the same bytes on every run.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn byte(&mut self) -> u8 {
        (self.next() >> 24) as u8
    }
}

/// The 128-byte DDS header (with the magic) for a `w` x `h` file of `mips` levels; `pf` is the
/// pixel format's flags, FourCC, bit count and four masks; `caps2` the cube bits.
fn header(w: u32, h: u32, mips: u32, pf: (u32, u32, u32, [u32; 4]), caps2: u32) -> Vec<u8> {
    let mut d = vec![0u8; 128];
    let mut put = |at: usize, v: u32| d[at..at + 4].copy_from_slice(&v.to_le_bytes());
    put(0, u32::from_le_bytes(*b"DDS "));
    put(4, 124);
    put(8, 0x1007 | if mips > 1 { 0x2_0000 } else { 0 });
    put(12, h);
    put(16, w);
    put(28, mips);
    put(76, 32);
    put(80, pf.0);
    put(84, pf.1);
    put(88, pf.2);
    for (i, m) in pf.3.iter().enumerate() {
        put(92 + 4 * i, *m);
    }
    put(108, 0x1000 | if mips > 1 { 0x40_0008 } else { 0 });
    put(112, caps2);
    d
}

fn fourcc(f: &[u8; 4]) -> (u32, u32, u32, [u32; 4]) {
    (4, u32::from_le_bytes(*f), 0, [0; 4])
}

/// DXT blocks with varied content: smooth gradients, edges, noise and flat runs, endpoints of
/// either order (DXT1's three-colour mode included), alpha in every form.
fn dxt_blocks(rng: &mut Rng, n: usize, bytes: usize, transparent: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(n * bytes);
    for i in 0..n {
        let mut b = vec![0u8; bytes];
        for x in b.iter_mut() {
            *x = rng.byte();
        }
        let c = bytes - 8;
        match i % 5 {
            // A flat block.
            0 => {
                b[c + 2] = b[c];
                b[c + 3] = b[c + 1];
            }
            // Close endpoints (the 5:6:5 rounding cases).
            1 => {
                b[c + 2] = b[c].wrapping_add(1);
                b[c + 3] = b[c + 1];
            }
            // Endpoints in the order DXT1 reads as three colours and transparent black.
            2 if transparent && b[c + 1] > b[c + 3] => {
                b.swap(c, c + 2);
                b.swap(c + 1, c + 3);
            }
            _ => {}
        }
        if bytes == 16 && i % 7 == 3 {
            // DXT5: the six-value alpha mode, or opaque.
            b[0] = b[1] / 2;
        }
        out.extend_from_slice(&b);
    }
    out
}

/// A DXT file of `fmt` (`w` x `h`, `mips` levels stored).
fn dxt(rng: &mut Rng, fmt: &[u8; 4], w: u32, h: u32, mips: u32) -> Vec<u8> {
    let bytes = if fmt == b"DXT1" { 8 } else { 16 };
    let mut d = header(w, h, mips, fourcc(fmt), 0);
    for l in 0..mips {
        let (lw, lh) = ((w >> l).max(1), (h >> l).max(1));
        let n = (lw.div_ceil(4) * lh.div_ceil(4)) as usize;
        d.extend(dxt_blocks(rng, n, bytes, fmt == b"DXT1"));
    }
    d
}

/// An uncompressed file: `bpp` bytes per pixel, a smooth picture with noise.
fn plain(rng: &mut Rng, pf: (u32, u32, u32, [u32; 4]), w: u32, h: u32, mips: u32) -> Vec<u8> {
    let bpp = (pf.2 / 8) as usize;
    let mut d = header(w, h, mips, pf, 0);
    for l in 0..mips {
        let (lw, lh) = ((w >> l).max(1), (h >> l).max(1));
        for y in 0..lh {
            for x in 0..lw {
                for k in 0..bpp {
                    let v = (x * 255 / lw.max(1)) as u8 ^ (y * 97 / lh.max(1)) as u8;
                    d.push(v.wrapping_add((k as u8) * 61).wrapping_add(rng.byte() & 7));
                }
            }
        }
    }
    d
}

/// The synthetic set.
pub fn files() -> Vec<File> {
    let mut rng = Rng(0x5eed_d3d8_0909_2008);
    let mut v = Vec::new();
    let mut add = |name: &str, cube: bool, data: Vec<u8>| {
        v.push(File {
            name: name.into(),
            cube,
            data,
        })
    };
    // The whole chain stored: levels under a block re-encoded with dither.
    add("dxt1-64-full", false, dxt(&mut rng, b"DXT1", 64, 64, 7));
    add("dxt3-32-full", false, dxt(&mut rng, b"DXT3", 32, 32, 6));
    add("dxt5-32-full", false, dxt(&mut rng, b"DXT5", 32, 32, 6));
    add("dxt1-64x16-full", false, dxt(&mut rng, b"DXT1", 64, 16, 7));
    add("dxt5-16x64-full", false, dxt(&mut rng, b"DXT5", 16, 64, 7));
    // One level: the chain from the box filter.
    add("dxt1-128-one", false, dxt(&mut rng, b"DXT1", 128, 128, 1));
    add("dxt3-64-one", false, dxt(&mut rng, b"DXT3", 64, 64, 1));
    add("dxt5-128-one", false, dxt(&mut rng, b"DXT5", 128, 128, 1));
    add("dxt5-256x32-one", false, dxt(&mut rng, b"DXT5", 256, 32, 1));
    // Some levels stored, the rest made.
    add("dxt1-64-three", false, dxt(&mut rng, b"DXT1", 64, 64, 3));
    add("dxt5-64-four", false, dxt(&mut rng, b"DXT5", 64, 64, 4));
    // Uncompressed.
    let argb = (0x41, 0, 32, [0xff_0000, 0xff00, 0xff, 0xff00_0000]);
    let xrgb = (0x40, 0, 32, [0xff_0000, 0xff00, 0xff, 0]);
    let rgb = (0x40, 0, 24, [0xff_0000, 0xff00, 0xff, 0]);
    let argb4 = (0x41, 0, 16, [0xf00, 0xf0, 0xf, 0xf000]);
    add("a8r8g8b8-32-one", false, plain(&mut rng, argb, 32, 32, 1));
    add("a8r8g8b8-64x4-one", false, plain(&mut rng, argb, 64, 4, 1));
    add("a8r8g8b8-16-full", false, plain(&mut rng, argb, 16, 16, 5));
    add("x8r8g8b8-32x8-one", false, plain(&mut rng, xrgb, 32, 8, 1));
    add("a4r4g4b4-32-one", false, plain(&mut rng, argb4, 32, 32, 1));
    add("a4r4g4b4-8x32-one", false, plain(&mut rng, argb4, 8, 32, 1));
    add("r8g8b8-16-one", false, plain(&mut rng, rgb, 16, 16, 1));
    add("r8g8b8-8x2-full", false, plain(&mut rng, rgb, 8, 2, 4));
    // A DXT1 cube map with its chain down to 4x4, as the game's.
    {
        let mut d = header(16, 16, 3, fourcc(b"DXT1"), 0xfe00);
        for _ in 0..6 {
            for l in 0..3u32 {
                let s = (16u32 >> l).max(1);
                let n = (s.div_ceil(4) * s.div_ceil(4)) as usize;
                d.extend(dxt_blocks(&mut rng, n, 8, true));
            }
        }
        add("dxt1-cube-16-three", true, d);
    }
    // Refused: cut one byte short; the cube call on a file with one face.
    let mut short = dxt(&mut rng, b"DXT5", 16, 16, 5);
    short.pop();
    add("dxt5-16-short", false, short);
    add(
        "dxt1-16-not-a-cube",
        true,
        dxt(&mut rng, b"DXT1", 16, 16, 5),
    );
    v
}

/// FNV-1a, 64 bits (the test data's level hashes).
pub fn fnv(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &x| {
        (h ^ u64::from(x)).wrapping_mul(0x100_0000_01b3)
    })
}
