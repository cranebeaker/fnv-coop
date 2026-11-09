//! Malformed DDS headers (a mod's broken texture): every call answers, quickly, with D3DX's
//! error or a texture, never a panic, a hang or an allocation the header asks for. TEST DATA
//! built here from the documented DDS layout (adapted from the review of PR #343).

use std::sync::mpsc;
use std::time::Duration;

fn header(
    w: u32,
    h: u32,
    mips: u32,
    flags: u32,
    pf: (u32, u32, u32, [u32; 4]),
    caps2: u32,
) -> Vec<u8> {
    let mut d = vec![0u8; 128];
    let mut put = |at: usize, v: u32| d[at..at + 4].copy_from_slice(&v.to_le_bytes());
    put(0, u32::from_le_bytes(*b"DDS "));
    put(4, 124);
    put(8, flags);
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
    put(112, caps2);
    d
}

fn dxt1() -> (u32, u32, u32, [u32; 4]) {
    (4, u32::from_le_bytes(*b"DXT1"), 0, [0; 4])
}

/// The info call's words, then the create call's level and face counts.
type Answers = (
    Result<[u32; 7], d3dx_tex::Error>,
    Result<(u32, u32), d3dx_tex::Error>,
);

/// What the two calls answer for `data`: the info's error or words, the create call's error
/// or level and face counts. Fails the test on a panic or after 10 s.
fn answers(data: Vec<u8>, cube: bool) -> Answers {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let info = d3dx_tex::get_image_info_from_file_in_memory(&data).map(|i| i.words());
        let caps = d3dx_tex::Caps::default();
        let made = if cube {
            d3dx_tex::create_cube_texture_from_file_in_memory(&data, &caps, PRECISION)
        } else {
            d3dx_tex::create_texture_from_file_in_memory(&data, &caps, PRECISION)
        };
        let _ = tx.send((info, made.map(|t| (t.levels, t.faces))));
    });
    rx.recv_timeout(Duration::from_secs(10))
        .expect("an answer within 10 s, without a panic")
}

const INVALID: d3dx_tex::Error = d3dx_tex::Error::D3dx(d3dx_tex::D3DXERR_INVALIDDATA);

/// The precision the files go through here (the other tests use single precision).
const PRECISION: d3dx_tex::Precision = d3dx_tex::Precision::Double;

/// Sides whose sizes pass 32 bits, and a side of 0 (levels of no bytes): the image info is the
/// header's, or a refusal when even the wrapped size is not in the file, and the create call answers a D3DX error (E_FAIL for the
/// side of 0; a refusal where the DLL would read past the file); the exact answers are the DLL's
/// in `tests/dll_answers.rs`.
#[test]
fn huge_sides_answer() {
    let argb = (0x41, 0, 32, [0xff_0000, 0xff00, 0xff, 0xff00_0000]);
    let f128 = (4, 116, 0, [0; 4]);
    for (w, h, pf) in [
        (0x8000_0001, 0, dxt1()),
        (0x8000_0000, 0x8000_0000, argb),
        (u32::MAX, u32::MAX, f128),
    ] {
        let (info, made) = answers(header(w, h, 1, 0x1007, pf, 0), false);
        assert!(
            matches!(info, Ok(i) if (i[0], i[1]) == (w, h))
                || info == Err(d3dx_tex::Error::D3dx(d3dx_tex::D3DXERR_INVALIDDATA)),
            "{w:#x}x{h:#x}: {info:?}"
        );
        assert!(
            matches!(made, Err(d3dx_tex::Error::D3dx(0x8000_4005 | 0x8876_0b59))),
            "{w:#x}x{h:#x}: {made:?}"
        );
    }
}

#[test]
fn huge_mip_counts_are_refused() {
    let mut d = header(4, 4, u32::MAX, 0x1007, dxt1(), 0);
    d.extend([0u8; 64]);
    let (info, made) = answers(d, false);
    assert_eq!((info, made), (Err(INVALID), Err(INVALID)));
    // A format of 2-byte pixels (FourCC 117) with 50 million levels in 64 bytes.
    let mut d = header(4, 4, 50_000_000, 0x1007, (4, 117, 0, [0; 4]), 0);
    d.extend([0u8; 64]);
    let (info, made) = answers(d, false);
    assert_eq!((info, made), (Err(INVALID), Err(INVALID)));
}

/// A mip count past the chain with its 1x1 levels present: D3DX reads it; the texture is the
/// chain.
#[test]
fn extra_one_by_one_levels_are_read_and_dropped() {
    let mut d = header(8, 8, 6, 0x1007, dxt1(), 0);
    d.extend(vec![0x33u8; 32 + 8 + 8 + 3 * 8]);
    let (info, made) = answers(d, false);
    assert_eq!(info.map(|w| w[3]), Ok(6));
    assert_eq!(made, Ok((4, 1)));
}

#[test]
fn random_small_headers_answer() {
    let mut seed = 0x1234_5678_9abc_def1u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let pfs = [
        dxt1(),
        (4, u32::from_le_bytes(*b"DXT3"), 0, [0; 4]),
        (4, u32::from_le_bytes(*b"DXT5"), 0, [0; 4]),
        (0x41, 0, 32, [0xff_0000, 0xff00, 0xff, 0xff00_0000]),
        (0x40, 0, 32, [0xff_0000, 0xff00, 0xff, 0]),
        (0x40, 0, 24, [0xff_0000, 0xff00, 0xff, 0]),
        (0x41, 0, 16, [0xf00, 0xf0, 0xf, 0xf000]),
    ];
    let sides = [0u32, 1, 2, 3, 4, 5, 8, 12, 16, 32, 64];
    for _ in 0..3000 {
        let r = next();
        let pf = pfs[(r % 7) as usize];
        let (w, h) = (
            sides[(r >> 8) as usize % 11],
            sides[(r >> 16) as usize % 11],
        );
        let mips = (r >> 24) as u32 % 12;
        let caps2 = [0u32, 0xfc00, 0xfe00, 0x0400][(r >> 32) as usize % 4];
        let flags = if (r >> 40) & 7 == 0 {
            0x80_1007
        } else {
            0x1007
        };
        let mut d = header(w, h, mips, flags, pf, caps2);
        for _ in 0..next() % 40000 {
            d.push(next() as u8);
        }
        let cube = (r >> 44) & 1 == 1;
        let data = d.clone();
        let ok = std::panic::catch_unwind(move || {
            let _ = d3dx_tex::get_image_info_from_file_in_memory(&data);
            let caps = d3dx_tex::Caps::default();
            if cube {
                let _ = d3dx_tex::create_cube_texture_from_file_in_memory(&data, &caps, PRECISION);
            } else {
                let _ = d3dx_tex::create_texture_from_file_in_memory(&data, &caps, PRECISION);
            }
        });
        assert!(
            ok.is_ok(),
            "w {w} h {h} mips {mips} pf {:x} caps2 {caps2:x} flags {flags:x} len {} cube {cube}",
            pf.1,
            d.len()
        );
    }
}

#[test]
fn every_truncation_answers() {
    let mut full = header(16, 16, 5, 0x1007, dxt1(), 0);
    full.extend(vec![0x5au8; 128 + 32 + 8 + 8 + 8]);
    for n in 0..=full.len() {
        let data = full[..n].to_vec();
        let ok = std::panic::catch_unwind(move || {
            let info = d3dx_tex::get_image_info_from_file_in_memory(&data);
            let made = d3dx_tex::create_texture_from_file_in_memory(
                &data,
                &d3dx_tex::Caps::default(),
                PRECISION,
            );
            (info.is_ok(), made.is_ok())
        });
        let (info, made) = ok.unwrap_or_else(|_| panic!("{n} bytes: a panic"));
        // Only the whole file is a texture.
        assert_eq!(
            (info, made),
            (n == full.len(), n == full.len()),
            "{n} bytes"
        );
    }
}
