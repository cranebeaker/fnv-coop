//! d3dx-tex-verify: every texture in the game's archives through `crates/d3dx-tex`, each level's
//! bytes compared with the 32-bit `d3dx9_38.dll`'s answer for the same file, read from the
//! texture cache `scripts/d3dx-texture-capture` wrote (the 32-bit DLL through `d3dx-measure
//! serve`; `index.tsv` names every texture it took).
//!
//! `cargo run --release --manifest-path scripts/d3dx-tex-verify/Cargo.toml -- <Data folder>
//! <cache folder> [--only <path part>] [--threads N] [--worst N] [--list <out.tsv>] [--double]`
//!
//! `--double`: the cache was captured on a thread at 53-bit precision (`d3dx-texture-capture ...
//! thread`), so the textures are made at double precision; also for `--fuzz` and `--probe`.
//!
//! The archives are only read. Prints how many textures match exactly, which do not (by
//! format and level size), and the worst remaining levels by decoded difference (0-255 per
//! channel, DXT blocks decoded).
mod dll;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// The precision the references were captured at (`--double`: on a thread at 53 bits, the
/// capture's `thread` option; otherwise the device thread's 24 bits).
static DOUBLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn precision() -> d3dx_tex::Precision {
    if DOUBLE.load(Ordering::Relaxed) {
        d3dx_tex::Precision::Double
    } else {
        d3dx_tex::Precision::Single
    }
}

/// The x87 control word of [`precision`] for the DLL's own functions.
fn control_word() -> u16 {
    match precision() {
        d3dx_tex::Precision::Single => 0x7f,
        d3dx_tex::Precision::Double => 0x27f,
    }
}

/// FNV-1a, 64 bits.
fn fnv(seed: u64, b: &[u8]) -> u64 {
    b.iter().fold(seed, |h, &x| {
        (h ^ u64::from(x)).wrapping_mul(0x100_0000_01b3)
    })
}

/// The cache key (as `d3dx-texture-capture`'s and `engine_platform::d3dx::texture_key`).
fn key(call: &str, data: &[u8]) -> String {
    let h = fnv(fnv(0xcbf2_9ce4_8422_2325, call.as_bytes()), data);
    format!("{call}-{:x}-{h:016x}", data.len())
}

fn word(d: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(d[at..at + 4].try_into().unwrap())
}

/// One surface of the reference: format, width, height, bytes (`None`: the file's level as
/// stored).
struct RefSurface {
    format: u32,
    width: u32,
    height: u32,
    bytes: Option<Vec<u8>>,
}

struct Reference {
    hr: u32,
    levels: u32,
    faces: u32,
    surfaces: Vec<RefSurface>,
}

fn parse(b: &[u8]) -> Option<Reference> {
    if b.get(..8)? != b"NVTX0001" {
        return None;
    }
    fn take(b: &[u8], at: &mut usize) -> Option<u32> {
        let w = u32::from_le_bytes(b.get(*at..*at + 4)?.try_into().ok()?);
        *at += 4;
        Some(w)
    }
    let mut at = 8usize;
    let (_kind, hr, levels, faces) = (
        take(b, &mut at)?,
        take(b, &mut at)?,
        take(b, &mut at)?,
        take(b, &mut at)?,
    );
    let mut surfaces = Vec::new();
    for _ in 0..levels * faces {
        let (format, width, height, kept, len) = (
            take(b, &mut at)?,
            take(b, &mut at)?,
            take(b, &mut at)?,
            take(b, &mut at)?,
            take(b, &mut at)?,
        );
        let bytes = if kept != 0 {
            let s = b.get(at..at + len as usize)?.to_vec();
            at += len as usize;
            Some(s)
        } else {
            None
        };
        surfaces.push(RefSurface {
            format,
            width,
            height,
            bytes,
        });
    }
    Some(Reference {
        hr,
        levels,
        faces,
        surfaces,
    })
}

/// The file's level byte ranges, face by face (the documented DDS layout).
fn file_levels(d: &[u8], faces: u32) -> Vec<(usize, usize)> {
    let (height, width, mips) = (word(d, 12), word(d, 16), word(d, 28).max(1));
    let (flags, fourcc, bits) = (word(d, 80), word(d, 84), word(d, 88));
    let (block, bytes) = if flags & 4 != 0 {
        match &fourcc.to_le_bytes() {
            b"DXT1" => (4, 8),
            _ => (4, 16),
        }
    } else {
        (1, bits / 8)
    };
    let mut at = 128usize;
    let mut out = Vec::new();
    for _ in 0..faces {
        for l in 0..mips {
            let (w, h) = ((width >> l).max(1), (height >> l).max(1));
            let size = (w.div_ceil(block) * h.div_ceil(block) * bytes) as usize;
            out.push((at, size));
            at += size;
        }
    }
    out
}

fn fmt_name(f: u32) -> String {
    match f {
        0x3154_5844 => "DXT1".into(),
        0x3254_5844 => "DXT2".into(),
        0x3354_5844 => "DXT3".into(),
        0x3454_5844 => "DXT4".into(),
        0x3554_5844 => "DXT5".into(),
        20 => "R8G8B8".into(),
        21 => "A8R8G8B8".into(),
        22 => "X8R8G8B8".into(),
        23 => "R5G6B5".into(),
        25 => "A1R5G5B5".into(),
        26 => "A4R4G4B4".into(),
        28 => "A8".into(),
        50 => "L8".into(),
        51 => "A8L8".into(),
        other => format!("{other:#x}"),
    }
}

fn c565(c: u16) -> [i32; 3] {
    let c = i32::from(c);
    [
        ((c >> 11) & 31) * 255 / 31,
        ((c >> 5) & 63) * 255 / 63,
        (c & 31) * 255 / 31,
    ]
}

/// Decodes a level to RGBA bytes for measuring differences (a plain decoder: both sides go
/// through the same one).
fn decode(format: u32, w: u32, h: u32, d: &[u8]) -> Vec<[i32; 4]> {
    let (w, h) = (w as usize, h as usize);
    let mut px = vec![[0i32; 4]; w * h];
    let dxt1 = format == 0x3154_5844;
    let dxt = (0x3154_5844..=0x3554_5844).contains(&format);
    if !dxt {
        let bpp = d.len() / (w * h).max(1);
        for (i, p) in px.iter_mut().enumerate() {
            for c in 0..bpp.min(4) {
                p[c] = i32::from(*d.get(i * bpp + c).unwrap_or(&0));
            }
        }
        return px;
    }
    let bs = if dxt1 { 8 } else { 16 };
    let (bw, bh) = (w.div_ceil(4), h.div_ceil(4));
    for by in 0..bh {
        for bx in 0..bw {
            let i = by * bw + bx;
            let Some(blk) = d.get(i * bs..i * bs + bs) else {
                continue;
            };
            let mut alpha = [255i32; 16];
            let cb = if dxt1 {
                blk
            } else if format <= 0x3354_5844 {
                for (p, a) in alpha.iter_mut().enumerate() {
                    *a = i32::from((blk[p >> 1] >> ((p & 1) * 4)) & 15) * 17;
                }
                &blk[8..]
            } else {
                let (a0, a1) = (i32::from(blk[0]), i32::from(blk[1]));
                let mut bits = 0u64;
                for k in 0..6 {
                    bits |= u64::from(blk[2 + k]) << (8 * k);
                }
                let mut pal = [a0, a1, 0, 0, 0, 0, 0, 255];
                if a0 > a1 {
                    for k in 1..7 {
                        pal[k + 1] = ((7 - k as i32) * a0 + k as i32 * a1) / 7;
                    }
                } else {
                    for k in 1..5 {
                        pal[k + 1] = ((5 - k as i32) * a0 + k as i32 * a1) / 5;
                    }
                }
                for (p, a) in alpha.iter_mut().enumerate() {
                    *a = pal[((bits >> (3 * p)) & 7) as usize];
                }
                &blk[8..]
            };
            let c0 = u16::from_le_bytes([cb[0], cb[1]]);
            let c1 = u16::from_le_bytes([cb[2], cb[3]]);
            let idx = u32::from_le_bytes([cb[4], cb[5], cb[6], cb[7]]);
            let (a, b) = (c565(c0), c565(c1));
            let four = c0 > c1 || !dxt1;
            let pal: [[i32; 3]; 4] = if four {
                [
                    a,
                    b,
                    [0, 1, 2].map(|k| (2 * a[k] + b[k]) / 3),
                    [0, 1, 2].map(|k| (a[k] + 2 * b[k]) / 3),
                ]
            } else {
                [a, b, [0, 1, 2].map(|k| (a[k] + b[k]) / 2), [0, 0, 0]]
            };
            for p in 0..16 {
                let (x, y) = (bx * 4 + (p & 3), by * 4 + (p >> 2));
                if x >= w || y >= h {
                    continue;
                }
                let s = ((idx >> (2 * p)) & 3) as usize;
                let c = pal[s];
                let a = if !four && s == 3 { 0 } else { alpha[p] };
                px[y * w + x] = [c[0], c[1], c[2], a];
            }
        }
    }
    px
}

/// One level that differs.
#[derive(Clone)]
struct Diff {
    path: String,
    face: u32,
    level: u32,
    format: u32,
    width: u32,
    height: u32,
    /// Largest decoded channel difference (0-255).
    max: i32,
    /// Blocks (or pixels for non-block formats) whose bytes differ, of all.
    blocks: usize,
    of: usize,
    /// The first differing block: (x, y, ours, theirs).
    first: Option<(u32, u32, Vec<u8>, Vec<u8>)>,
}

enum Outcome {
    Same,
    NoReference,
    /// A different answer, level count, face count, format or size.
    Shape(String),
    Levels(Vec<Diff>),
    Panic(String),
}

fn compare(path: &str, data: &[u8], cache: &std::path::Path) -> (Outcome, Duration) {
    let caps2 = if data.len() >= 128 {
        word(data, 112)
    } else {
        0
    };
    let cube = caps2 & 0x200 != 0;
    let call = if cube {
        "CreateCubeTextureFromFileInMemory"
    } else {
        "CreateTextureFromFileInMemory"
    };
    let k = key(call, data);
    let Some(reference) = std::fs::read(cache.join(format!("{k}.tex")))
        .ok()
        .as_deref()
        .and_then(parse)
    else {
        return (Outcome::NoReference, Duration::ZERO);
    };
    let start = Instant::now();
    let made = std::panic::catch_unwind(|| {
        if cube {
            d3dx_tex::create_cube_texture_from_file_in_memory(
                data,
                &d3dx_tex::Caps::default(),
                precision(),
            )
        } else {
            d3dx_tex::create_texture_from_file_in_memory(
                data,
                &d3dx_tex::Caps::default(),
                precision(),
            )
        }
    });
    let took = start.elapsed();
    let made = match made {
        Ok(m) => m,
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            return (Outcome::Panic(msg), took);
        }
    };
    let ours = match made {
        Err(d3dx_tex::Error::D3dx(hr)) => {
            return if hr == reference.hr {
                (Outcome::Same, took)
            } else {
                (
                    Outcome::Shape(format!("answer {hr:08x}, the DLL's {:08x}", reference.hr)),
                    took,
                )
            }
        }
        Err(d3dx_tex::Error::NotProvided(why)) => {
            return (Outcome::Panic(format!("not provided: {why}")), took)
        }
        Ok(t) => t,
    };
    if reference.hr != 0 {
        return (
            Outcome::Shape(format!("answer 0, the DLL's {:08x}", reference.hr)),
            took,
        );
    }
    if ours.levels != reference.levels || ours.faces != reference.faces {
        return (
            Outcome::Shape(format!(
                "{} levels x {} faces, the DLL's {} x {}",
                ours.levels, ours.faces, reference.levels, reference.faces
            )),
            took,
        );
    }
    let file = file_levels(data, reference.faces);
    let stored = file.len() as u32 / reference.faces.max(1);
    let mut diffs = Vec::new();
    for (i, (r, o)) in reference.surfaces.iter().zip(&ours.surfaces).enumerate() {
        let (face, level) = (i as u32 / reference.levels, i as u32 % reference.levels);
        if (o.format, o.width, o.height) != (r.format, r.width, r.height) {
            return (
                Outcome::Shape(format!(
                    "face {face} level {level}: {} {}x{}, the DLL's {} {}x{}",
                    fmt_name(o.format),
                    o.width,
                    o.height,
                    fmt_name(r.format),
                    r.width,
                    r.height
                )),
                took,
            );
        }
        let theirs: &[u8] = match &r.bytes {
            Some(b) => b,
            None => {
                assert!(
                    level < stored,
                    "{path}: the cache says copy for a level the file lacks"
                );
                let (at, len) = file[(face * stored + level) as usize];
                &data[at..at + len]
            }
        };
        let mine: &[u8] = &o.bytes;
        if mine == theirs {
            continue;
        }
        let dxt = (0x3154_5844..=0x3554_5844).contains(&r.format);
        let unit = if r.format == 0x3154_5844 {
            8
        } else if dxt {
            16
        } else {
            (theirs.len() / (r.width * r.height).max(1) as usize).max(1)
        };
        let per_row = if dxt { r.width.div_ceil(4) } else { r.width } as usize;
        let mut blocks = 0;
        let mut first = None;
        let n = theirs.len().min(mine.len()) / unit;
        for b in 0..n {
            if mine[b * unit..(b + 1) * unit] != theirs[b * unit..(b + 1) * unit] {
                blocks += 1;
                if first.is_none() {
                    first = Some((
                        (b % per_row) as u32,
                        (b / per_row) as u32,
                        mine[b * unit..(b + 1) * unit].to_vec(),
                        theirs[b * unit..(b + 1) * unit].to_vec(),
                    ));
                }
            }
        }
        let pa = decode(r.format, r.width, r.height, theirs);
        let pb = decode(r.format, r.width, r.height, mine);
        let max = pa
            .iter()
            .zip(&pb)
            .flat_map(|(a, b)| (0..4).map(move |c| (a[c] - b[c]).abs()))
            .max()
            .unwrap_or(0);
        diffs.push(Diff {
            path: path.to_string(),
            face,
            level,
            format: r.format,
            width: r.width,
            height: r.height,
            max,
            blocks,
            of: n,
            first,
        });
    }
    if diffs.is_empty() {
        (Outcome::Same, took)
    } else {
        (Outcome::Levels(diffs), took)
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// `--time`: load time of every texture, one thread, the archives' bytes already read: the
/// player's 64-bit DLL the way the port loads today (create on the real device, read every
/// level back) against `d3dx-tex`. Totals, and the slowest textures of each.
fn time_both(
    archives: &[PathBuf],
    wanted: &std::collections::HashSet<(String, String)>,
    only: &[String],
) {
    let dll = unsafe { dll::Dll::open() };
    let (mut t_dll, mut t_ours) = (Duration::ZERO, Duration::ZERO);
    let mut slow: Vec<(Duration, Duration, String)> = Vec::new();
    let mut n = 0usize;
    for path in archives {
        let archive = bsa::Archive::open(path).expect("an archive");
        let source = path.file_name().unwrap().to_string_lossy().to_string();
        let files: Vec<(String, Vec<u8>)> = archive
            .files()
            .iter()
            .filter(|e| wanted.contains(&(source.clone(), e.path.clone())))
            .filter(|e| only.is_empty() || only.iter().any(|o| e.path.contains(o.as_str())))
            .map(|e| (e.path.clone(), archive.read(e).expect("an archive entry")))
            .collect();
        for (name, data) in &files {
            let cube = word(data, 112) & 0x200 != 0;
            let s = Instant::now();
            let got = unsafe { dll.load(data, cube) };
            let a = s.elapsed();
            let s = Instant::now();
            let ours = if cube {
                d3dx_tex::create_cube_texture_from_file_in_memory(
                    data,
                    &d3dx_tex::Caps::default(),
                    precision(),
                )
            } else {
                d3dx_tex::create_texture_from_file_in_memory(
                    data,
                    &d3dx_tex::Caps::default(),
                    precision(),
                )
            };
            let b = s.elapsed();
            std::hint::black_box((got, &ours));
            t_dll += a;
            t_ours += b;
            n += 1;
            slow.push((a, b, name.clone()));
        }
        eprintln!(
            "{source}: {n} textures, DLL {:.2} s, ours {:.2} s",
            t_dll.as_secs_f64(),
            t_ours.as_secs_f64()
        );
    }
    println!(
        "{n} textures: the 64-bit DLL (create + read back) {:.2} s, d3dx-tex {:.2} s",
        t_dll.as_secs_f64(),
        t_ours.as_secs_f64()
    );
    slow.sort_by(|x, y| y.1.cmp(&x.1));
    println!("slowest for d3dx-tex (ours ms / DLL ms):");
    for (a, b, name) in slow.iter().take(12) {
        println!(
            "  {:8.2} {:8.2}  {name}",
            b.as_secs_f64() * 1e3,
            a.as_secs_f64() * 1e3
        );
    }
    slow.sort_by(|x, y| y.0.cmp(&x.0));
    println!("slowest for the DLL (ours ms / DLL ms):");
    for (a, b, name) in slow.iter().take(6) {
        println!(
            "  {:8.2} {:8.2}  {name}",
            b.as_secs_f64() * 1e3,
            a.as_secs_f64() * 1e3
        );
    }
}

/// `--capture-synthetic <out.tsv> --serve <d3dx-measure.exe>`: `d3dx_tex::synthetic`'s files
/// through the 32-bit DLL (`d3dx-measure serve`: the create call on a HAL device, each level read
/// back), one line each: name, call, answer, levels, faces, then per surface `format,width,height,
/// fnv1a64 of the bytes`. `crates/d3dx-tex/data/synthetic_32bit.tsv` is this output;
/// `synthetic_32bit_double.tsv` with `--double` (the DLL on a thread at 53 bits).
fn capture_synthetic(out: &std::path::Path, serve: &str) {
    use std::io::Read;
    let mut child = std::process::Command::new(serve)
        .arg("serve")
        .args(DOUBLE.load(Ordering::Relaxed).then_some("thread"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("d3dx-measure serve");
    let mut text = format!("# d3dx_tex::synthetic::files() through the 32-bit d3dx9_38.dll (d3dx-measure serve, x87 control word {:#x}): name, call, answer, levels, faces, then per surface format,width,height,fnv1a64\n", control_word());
    for f in d3dx_tex::synthetic::files() {
        let stdin = child.stdin.as_mut().unwrap();
        stdin
            .write_all(&(if f.cube { 2u32 } else { 1 }).to_le_bytes())
            .unwrap();
        stdin
            .write_all(&(f.data.len() as u32).to_le_bytes())
            .unwrap();
        stdin.write_all(&f.data).unwrap();
        stdin.flush().unwrap();
        let o = child.stdout.as_mut().unwrap();
        fn rd(o: &mut impl Read) -> u32 {
            let mut b = [0u8; 4];
            o.read_exact(&mut b).unwrap();
            u32::from_le_bytes(b)
        }
        let (hr, levels, faces) = (rd(o), rd(o), rd(o));
        let mut parts = Vec::new();
        for _ in 0..levels * faces {
            let (format, w, h, len) = (rd(o), rd(o), rd(o), rd(o));
            let mut bytes = vec![0u8; len as usize];
            o.read_exact(&mut bytes).unwrap();
            parts.push(format!(
                "{format:x},{w},{h},{:016x}",
                d3dx_tex::synthetic::fnv(&bytes)
            ));
        }
        text += &format!(
            "{}\t{}\t{hr:08x}\t{levels}\t{faces}\t{}\n",
            f.name,
            if f.cube { "cube" } else { "texture" },
            parts.join(";")
        );
    }
    drop(child.stdin.take());
    child.wait().unwrap();
    std::fs::write(out, text).unwrap();
}

/// `--capture-blocks <out.tsv> --oracle <d3dx-measure.exe>`: blocks of the fuzz kinds through the
/// 32-bit DLL's DXT encoders at single precision (`--double`: double), with and without dither,
/// one line each: format, dither, the 64 floats' bits, the DLL's block.
/// `crates/d3dx-tex/data/blocks_32bit.tsv` (`blocks_32bit_double.tsv`) is this output.
fn capture_blocks(out: &std::path::Path, oracle: &mut Oracle) {
    let mut seed = 0x0bad_5eed_1234_5678u64;
    let mut next = move || {
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        seed.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };
    let mut text = format!("# DXT blocks through the 32-bit d3dx9_38.dll's encoders (d3dx-measure blocks, x87 control word {:#x}): 16 pixels (red, green, blue, alpha float bits), then the DLL's DXT1, DXT3 and DXT5 blocks without and with dither\n", control_word());
    // At double precision only blocks the DLL encodes differently at single precision are kept,
    // so each one tests what the precision changes.
    let double = DOUBLE.load(Ordering::Relaxed);
    let (mut kept, mut next_k) = (0, 0usize);
    while kept < 24 {
        let k = next_k;
        next_k += 1;
        let unit = |r: u64| (r >> 40) as f32 / (1u64 << 24) as f32;
        let base: [f32; 4] = std::array::from_fn(|_| unit(next()));
        let other: [f32; 4] = std::array::from_fn(|_| unit(next()));
        let px: [[f32; 4]; 16] = std::array::from_fn(|i| match k % 4 {
            0 => std::array::from_fn(|_| unit(next())),
            1 => {
                let t = (next() % 3) as f32 / 2.0;
                std::array::from_fn(|c| base[c] + (other[c] - base[c]) * t)
            }
            2 => std::array::from_fn(|c| base[c] + (other[c] - base[c]) * (i as f32 / 15.0)),
            _ => {
                let a = [
                    0.0,
                    1.0,
                    (next() % 16) as f32 / 15.0,
                    (next() % 256) as f32 / 255.0,
                ][(next() % 4) as usize];
                [
                    ((base[0] * 31.0) as i32 as f32 + (next() % 3) as f32) / 31.0,
                    base[1],
                    base[2],
                    a,
                ]
            }
        });
        let mut blocks = Vec::new();
        let mut differs = false;
        for f in [0x3154_5844u32, 0x3354_5844, 0x3554_5844] {
            for dither in [false, true] {
                let b = oracle.encode(f, control_word(), dither, &px);
                differs |= double && b != oracle.encode(f, 0x7f, dither, &px);
                blocks.push(b);
            }
        }
        if double && !differs {
            continue;
        }
        kept += 1;
        let bits: Vec<String> = px
            .iter()
            .flatten()
            .map(|v| format!("{:08x}", v.to_bits()))
            .collect();
        text += &bits.join(" ");
        for b in &blocks {
            text += &format!("\t{}", hex(b));
        }
        text += "\n";
    }
    std::fs::write(out, text).unwrap();
}

/// The 32-bit DLL's block encoders (`d3dx-measure blocks`, an i686 build).
struct Oracle {
    child: std::process::Child,
}

impl Oracle {
    fn start(exe: &str) -> Oracle {
        let child = std::process::Command::new(exe)
            .arg("blocks")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("d3dx-measure blocks");
        Oracle { child }
    }

    fn encode(&mut self, f: u32, cw: u16, dither: bool, px: &[[f32; 4]; 16]) -> Vec<u8> {
        use std::io::Read;
        let kind: u32 = match f {
            0x3154_5844 => 1,
            0x3354_5844 => 3,
            _ => 5,
        };
        let mut req = Vec::new();
        for w in [kind, u32::from(cw), u32::from(dither)] {
            req.extend_from_slice(&w.to_le_bytes());
        }
        for p in px {
            for c in p {
                req.extend_from_slice(&c.to_bits().to_le_bytes());
            }
        }
        let stdin = self.child.stdin.as_mut().unwrap();
        stdin.write_all(&req).unwrap();
        stdin.flush().unwrap();
        let mut out = [0u8; 16];
        self.child
            .stdout
            .as_mut()
            .unwrap()
            .read_exact(&mut out)
            .unwrap();
        out[..if kind == 1 { 8 } else { 16 }].to_vec()
    }
}

impl Oracle {
    /// The DLL's colour endpoint search on 16 points.
    fn optimize_rgb(&mut self, cw: u16, steps: u32, px: &[[f32; 4]; 16]) -> [f32; 6] {
        use std::io::Read;
        let mut req = Vec::new();
        for w in [20u32, u32::from(cw), steps] {
            req.extend_from_slice(&w.to_le_bytes());
        }
        for p in px {
            for c in p {
                req.extend_from_slice(&c.to_bits().to_le_bytes());
            }
        }
        let stdin = self.child.stdin.as_mut().unwrap();
        stdin.write_all(&req).unwrap();
        stdin.flush().unwrap();
        let mut out = [0u8; 24];
        self.child
            .stdout
            .as_mut()
            .unwrap()
            .read_exact(&mut out)
            .unwrap();
        std::array::from_fn(|i| {
            f32::from_bits(u32::from_le_bytes(
                out[i * 4..i * 4 + 4].try_into().unwrap(),
            ))
        })
    }
}

/// `--fuzz N --oracle <d3dx-measure.exe>`: N blocks of several kinds (random colours, few
/// colours, gradients, alpha patterns) through our DXT encoders and the DLL's at single
/// precision (`--double`: at 53 bits, the DLL at control word 0x27f), with and without
/// dither; the endpoint search too. Prints the first differences.
fn fuzz(n: usize, oracle: &mut Oracle) {
    let cw = control_word();
    let mut seed = 0x1234_5678_9abc_def0u64;
    let mut next = move || {
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        seed.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };
    let unit = |next: &mut dyn FnMut() -> u64| (next() >> 40) as f32 / (1u64 << 24) as f32;
    let (mut same, mut total, mut shown) = ([0usize; 4], [0usize; 4], 0);
    for k in 0..n {
        let kind = k % 4;
        let mut px = [[0.0f32; 4]; 16];
        let base: [f32; 4] = std::array::from_fn(|_| unit(&mut next));
        let other: [f32; 4] = std::array::from_fn(|_| unit(&mut next));
        for (i, p) in px.iter_mut().enumerate() {
            *p = match kind {
                0 => std::array::from_fn(|_| unit(&mut next)),
                1 => {
                    let t = (next() % 3) as f32 / 2.0;
                    std::array::from_fn(|c| base[c] + (other[c] - base[c]) * t)
                }
                2 => {
                    let t = i as f32 / 15.0;
                    std::array::from_fn(|c| base[c] + (other[c] - base[c]) * t)
                }
                _ => {
                    let q: [f32; 4] = std::array::from_fn(|c| {
                        ((base[c] * 31.0) as i32 as f32 + (next() % 3) as f32) / 31.0
                    });
                    let a = [
                        0.0,
                        1.0,
                        (next() % 16) as f32 / 15.0,
                        (next() % 256) as f32 / 255.0,
                    ][(next() % 4) as usize];
                    [q[0].min(1.0), q[1].min(1.0), q[2].min(1.0), a]
                }
            };
        }
        for (fi, f) in [0x3154_5844u32, 0x3354_5844, 0x3554_5844]
            .into_iter()
            .enumerate()
        {
            for dither in [false, true] {
                let mine = d3dx_tex::filter::encode_block(f, &px, dither, precision());
                let theirs = oracle.encode(f, cw, dither, &px);
                total[fi] += 1;
                if mine == theirs {
                    same[fi] += 1;
                } else if shown < 12 {
                    shown += 1;
                    println!(
                        "kind {kind} fmt {fi} dither {dither}: ours {} DLL {}",
                        hex(&mine),
                        hex(&theirs)
                    );
                    for p in &px {
                        print!(
                            " [{:08x} {:08x} {:08x} {:08x}]",
                            p[0].to_bits(),
                            p[1].to_bits(),
                            p[2].to_bits(),
                            p[3].to_bits()
                        );
                    }
                    println!();
                }
            }
        }
        // The endpoint search on its own, on the colours as the encoder weighs them.
        let lum = [
            f32::from_bits(0x3e98_1530),
            1.0,
            f32::from_bits(0x3dce_6734),
        ];
        let q: [[f32; 4]; 16] = std::array::from_fn(|i| {
            let p = px[i];
            let r = d3dx_tex::num::fistp(p[0] * 31.0 + 0.5) as f32 * d3dx_tex::num::INV31;
            let g = d3dx_tex::num::fistp(p[1] * 63.0 + 0.5) as f32 * d3dx_tex::num::INV63;
            let b = d3dx_tex::num::fistp(p[2] * 31.0 + 0.5) as f32 * d3dx_tex::num::INV31;
            [r * lum[0], g * lum[1], b * lum[2], 1.0]
        });
        for steps in [3u32, 4] {
            let (x, y) = d3dx_tex::dxt::optimize_rgb(&q, steps as usize, precision());
            let theirs = oracle.optimize_rgb(cw, steps, &q);
            let mine = [x[0], x[1], x[2], y[0], y[1], y[2]];
            total[3] += 1;
            if mine
                .iter()
                .zip(&theirs)
                .all(|(a, b)| a.to_bits() == b.to_bits())
            {
                same[3] += 1;
            } else if shown < 24 {
                shown += 1;
                println!("optimize_rgb steps {steps}: ours {mine:?} DLL {theirs:?}");
            }
        }
    }
    println!(
        "same: DXT1 {}/{}, DXT3 {}/{}, DXT5 {}/{}, endpoint search {}/{}",
        same[0], total[0], same[1], total[1], same[2], total[2], same[3], total[3]
    );
}

/// `--probe <path part> --oracle <d3dx-measure.exe>`: for each DXT level the DLL made of the
/// first texture whose path contains the part, the blocks made from the DLL's own previous
/// level (or the file's level), compared with the DLL's: ours, and the DLL's own encoder run on
/// the same pixels with the x87 control word at single (0x7f), double (0x27f) and extended
/// (0x37f) precision.
fn probe(path: &str, data: &[u8], cache: &std::path::Path, oracle: &mut Oracle) {
    let caps2 = word(data, 112);
    let cube = caps2 & 0x200 != 0;
    let call = if cube {
        "CreateCubeTextureFromFileInMemory"
    } else {
        "CreateTextureFromFileInMemory"
    };
    let r = parse(&std::fs::read(cache.join(format!("{}.tex", key(call, data)))).unwrap()).unwrap();
    let file = file_levels(data, r.faces);
    let stored = file.len() as u32 / r.faces.max(1);
    let level_bytes = |i: usize| -> Vec<u8> {
        match &r.surfaces[i].bytes {
            Some(b) => b.clone(),
            None => {
                let (face, level) = (i as u32 / r.levels, i as u32 % r.levels);
                let (at, len) = file[(face * stored + level) as usize];
                data[at..at + len].to_vec()
            }
        }
    };
    println!("{path}: {} levels x {} faces", r.levels, r.faces);
    let mut tally = [0usize; 5];
    for (i, s) in r.surfaces.iter().enumerate() {
        let level = i as u32 % r.levels;
        if s.bytes.is_none() || !(0x3154_5844..=0x3554_5844).contains(&s.format) {
            continue;
        }
        let (w, h) = (s.width as usize, s.height as usize);
        let (colors, dither) = if level < stored {
            let (face, l) = (i as u32 / r.levels, level);
            let (at, len) = file[(face * stored + l) as usize];
            (
                d3dx_tex::filter::decode(s.format, w, h, &data[at..at + len], precision()),
                true,
            )
        } else {
            let prev = &r.surfaces[i - 1];
            let lv = d3dx_tex::filter::Level {
                format: prev.format,
                width: prev.width,
                height: prev.height,
                bytes: level_bytes(i - 1),
            };
            (d3dx_tex::filter::box_colors(&lv, precision()), false)
        };
        let theirs = level_bytes(i);
        let bs = if s.format == 0x3154_5844 { 8 } else { 16 };
        let bw = w.div_ceil(4);
        let mut shown = 0;
        for by in 0..h.div_ceil(4) {
            for bx in 0..bw {
                let px = d3dx_tex::filter::block_pixels(&colors, w, h, bx, by);
                let o = (by * bw + bx) * bs;
                let want = &theirs[o..o + bs];
                let mine = d3dx_tex::filter::encode_block(s.format, &px, dither, precision());
                let o7 = oracle.encode(s.format, 0x7f, dither, &px);
                let o27 = oracle.encode(s.format, 0x27f, dither, &px);
                let o37 = oracle.encode(s.format, 0x37f, dither, &px);
                tally[0] += 1;
                tally[1] += usize::from(mine == want);
                tally[2] += usize::from(o7 == want);
                tally[3] += usize::from(o27 == want);
                tally[4] += usize::from(o37 == want);
                if (mine != want || o7 != want) && shown < 3 {
                    shown += 1;
                    println!(
                        "  level {level} {w}x{h} block {bx},{by}: DLL {}  ours {}  pc24 {}  pc53 {}  pc64 {}",
                        hex(want),
                        hex(&mine),
                        hex(&o7),
                        hex(&o27),
                        hex(&o37)
                    );
                    if shown == 1 {
                        for p in &px {
                            print!(
                                " [{:08x} {:08x} {:08x} {:08x}]",
                                p[0].to_bits(),
                                p[1].to_bits(),
                                p[2].to_bits(),
                                p[3].to_bits()
                            );
                        }
                        println!();
                    }
                }
            }
        }
    }
    println!(
        "blocks {}: ours same {}, the DLL's encoder at pc24 same {}, pc53 same {}, pc64 same {}",
        tally[0], tally[1], tally[2], tally[3], tally[4]
    );
}

/// `--stats`: the files' pixel formats and sizes, and what the DLL made of them (which levels
/// it left as stored, `c`, and which it made, `K`), grouped.
fn stats(path: &str, data: &[u8], cache: &std::path::Path) -> Option<String> {
    let caps2 = word(data, 112);
    let cube = caps2 & 0x200 != 0;
    let call = if cube {
        "CreateCubeTextureFromFileInMemory"
    } else {
        "CreateTextureFromFileInMemory"
    };
    let r = parse(&std::fs::read(cache.join(format!("{}.tex", key(call, data)))).ok()?)?;
    let (h, w, mips) = (word(data, 12), word(data, 16), word(data, 28));
    let (flags, fourcc, bits) = (word(data, 80), word(data, 84), word(data, 88));
    let masks = [92, 96, 100, 104].map(|a| word(data, a));
    let pf = if flags & 4 != 0 {
        String::from_utf8_lossy(&fourcc.to_le_bytes()).to_string()
    } else {
        format!(
            "flags {flags:x} bits {bits} masks {:x}/{:x}/{:x}/{:x}",
            masks[0], masks[1], masks[2], masks[3]
        )
    };
    let pat: String = r.surfaces[..r.levels as usize]
        .iter()
        .map(|s| if s.bytes.is_some() { 'K' } else { 'c' })
        .collect();
    let pow2 = w.is_power_of_two() && h.is_power_of_two();
    let _ = path;
    Some(format!(
        "{pf} -> {} | {} | mips {} of {} | {}{}",
        fmt_name(r.surfaces[0].format),
        if cube { "cube" } else { "2d" },
        mips,
        r.levels,
        if pow2 { "" } else { "nonpow2 " },
        pat
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() >= 3,
        "usage: d3dx-tex-verify <Data folder> <cache folder> [--only <path part>] [--threads N] [--worst N] [--list out.tsv]"
    );
    let (data_dir, cache) = (PathBuf::from(&args[1]), PathBuf::from(&args[2]));
    let mut only: Vec<String> = Vec::new();
    let mut threads = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(8);
    let mut worst_n = 25usize;
    let mut list: Option<PathBuf> = None;
    let mut stats_mode = false;
    let mut probe_part: Option<String> = None;
    let mut oracle_exe: Option<String> = None;
    let mut fuzz_n: Option<usize> = None;
    let mut time_mode = false;
    let mut capture_synth: Option<PathBuf> = None;
    let mut capture_blk: Option<PathBuf> = None;
    let mut serve_exe: Option<String> = None;
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--stats" => stats_mode = true,
            "--probe" => {
                probe_part = Some(args[i + 1].to_ascii_lowercase());
                i += 1;
            }
            "--time" => time_mode = true,
            "--capture-synthetic" => {
                capture_synth = Some(PathBuf::from(&args[i + 1]));
                i += 1;
            }
            "--capture-blocks" => {
                capture_blk = Some(PathBuf::from(&args[i + 1]));
                i += 1;
            }
            "--serve" => {
                serve_exe = Some(args[i + 1].clone());
                i += 1;
            }
            "--level-threads" => {
                d3dx_tex::filter::set_max_threads(args[i + 1].parse().unwrap());
                i += 1;
            }
            "--fuzz" => {
                fuzz_n = Some(args[i + 1].parse().unwrap());
                i += 1;
            }
            "--double" => DOUBLE.store(true, Ordering::Relaxed),
            "--oracle" => {
                oracle_exe = Some(args[i + 1].clone());
                i += 1;
            }
            "--only" => {
                only.push(args[i + 1].to_ascii_lowercase());
                i += 1;
            }
            "--threads" => {
                threads = args[i + 1].parse().unwrap();
                i += 1;
            }
            "--worst" => {
                worst_n = args[i + 1].parse().unwrap();
                i += 1;
            }
            "--list" => {
                list = Some(PathBuf::from(&args[i + 1]));
                i += 1;
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    // The textures the capture took, by archive path (index.tsv: key, archive, path, ...).
    let index = std::fs::read_to_string(cache.join("index.tsv")).expect("the cache's index.tsv");
    let wanted: std::collections::HashSet<(String, String)> = index
        .lines()
        .filter_map(|l| {
            let mut f = l.split('\t');
            let k = f.next()?;
            if k == "skipped" {
                return None;
            }
            Some((f.next()?.to_string(), f.next()?.to_string()))
        })
        .collect();
    let mut archives: Vec<PathBuf> = std::fs::read_dir(&data_dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bsa")))
        .collect();
    archives.sort();
    if time_mode {
        time_both(&archives, &wanted, &only);
        return;
    }
    if let Some(out) = capture_synth {
        capture_synthetic(
            &out,
            serve_exe.as_deref().expect("--serve <d3dx-measure.exe>"),
        );
        return;
    }
    if let Some(out) = capture_blk {
        let mut oracle = Oracle::start(oracle_exe.as_deref().expect("--oracle <d3dx-measure.exe>"));
        capture_blocks(&out, &mut oracle);
        return;
    }
    if let Some(n) = fuzz_n {
        let mut oracle = Oracle::start(oracle_exe.as_deref().expect("--oracle <d3dx-measure.exe>"));
        fuzz(n, &mut oracle);
        return;
    }
    if let Some(part) = probe_part {
        let mut oracle = Oracle::start(oracle_exe.as_deref().expect("--oracle <d3dx-measure.exe>"));
        for path in &archives {
            let archive = bsa::Archive::open(path).expect("an archive");
            let source = path.file_name().unwrap().to_string_lossy().to_string();
            for e in archive.files() {
                if e.path.contains(part.as_str())
                    && wanted.contains(&(source.clone(), e.path.clone()))
                {
                    let data = archive.read(e).expect("an archive entry");
                    probe(&e.path, &data, &cache, &mut oracle);
                    return;
                }
            }
        }
        println!("no texture matches");
        return;
    }
    if stats_mode {
        let mut groups: BTreeMap<String, (usize, String)> = BTreeMap::new();
        for path in &archives {
            let archive = bsa::Archive::open(path).expect("an archive");
            let source = path.file_name().unwrap().to_string_lossy().to_string();
            for e in archive.files() {
                if !wanted.contains(&(source.clone(), e.path.clone())) {
                    continue;
                }
                let data = archive.read(e).expect("an archive entry");
                if let Some(s) = stats(&e.path, &data, &cache) {
                    let g = groups.entry(s).or_insert((0, e.path.clone()));
                    g.0 += 1;
                }
            }
        }
        let mut v: Vec<_> = groups.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
        for (s, (n, example)) in v {
            println!("{n}\t{s}\t{example}");
        }
        return;
    }
    let same = AtomicUsize::new(0);
    let total = AtomicUsize::new(0);
    let no_ref = AtomicUsize::new(0);
    let results: Mutex<Vec<(String, Outcome)>> = Mutex::new(Vec::new());
    let busy = Mutex::new(Duration::ZERO);
    let wall = Instant::now();
    for path in archives {
        let archive = bsa::Archive::open(&path).expect("an archive");
        let source = path.file_name().unwrap().to_string_lossy().to_string();
        let entries: Vec<&bsa::FileEntry> = archive
            .files()
            .iter()
            .filter(|e| e.path.to_ascii_lowercase().ends_with(".dds"))
            .filter(|e| wanted.contains(&(source.clone(), e.path.clone())))
            .filter(|e| only.is_empty() || only.iter().any(|o| e.path.contains(o.as_str())))
            .collect();
        if entries.is_empty() {
            continue;
        }
        let next = AtomicUsize::new(0);
        std::thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(|| loop {
                    let n = next.fetch_add(1, Ordering::Relaxed);
                    let Some(entry) = entries.get(n) else { break };
                    let data = archive.read(entry).expect("an archive entry");
                    let (outcome, took) = compare(&entry.path, &data, &cache);
                    *busy.lock().unwrap() += took;
                    match outcome {
                        Outcome::NoReference => {
                            no_ref.fetch_add(1, Ordering::Relaxed);
                        }
                        Outcome::Same => {
                            same.fetch_add(1, Ordering::Relaxed);
                            total.fetch_add(1, Ordering::Relaxed);
                        }
                        other => {
                            total.fetch_add(1, Ordering::Relaxed);
                            results
                                .lock()
                                .unwrap()
                                .push((format!("{source}\t{}", entry.path), other));
                        }
                    }
                });
            }
        });
        eprintln!(
            "{source}: {} of {} so far identical",
            same.load(Ordering::Relaxed),
            total.load(Ordering::Relaxed)
        );
    }
    let results = results.into_inner().unwrap();
    let (same, total) = (same.into_inner(), total.into_inner());
    println!(
        "textures compared: {total}; identical: {same}; differ: {}",
        total - same
    );
    let negative = d3dx_tex::dxt::NEGATIVE_STEPS.load(Ordering::Relaxed);
    if negative > 0 {
        println!("endpoint search steps below 0 (the DLL reads memory before its tables there): {negative}");
    }
    if no_ref.load(Ordering::Relaxed) > 0 {
        println!(
            "no reference in the cache: {}",
            no_ref.load(Ordering::Relaxed)
        );
    }
    println!(
        "translation time: {:.2} s over all threads ({:.2} s wall, {threads} threads)",
        busy.into_inner().unwrap().as_secs_f64(),
        wall.elapsed().as_secs_f64()
    );
    let mut shapes: BTreeMap<String, usize> = BTreeMap::new();
    let mut panics: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
    let mut levels: Vec<Diff> = Vec::new();
    let mut listing = String::new();
    for (name, o) in &results {
        match o {
            Outcome::Shape(s) => {
                *shapes.entry(s.clone()).or_default() += 1;
                listing += &format!("{name}\tshape\t{s}\n");
            }
            Outcome::Panic(s) => {
                *panics.entry(s.chars().take(160).collect()).or_default() += 1;
                listing += &format!("{name}\tpanic\t{s}\n");
            }
            Outcome::Levels(d) => {
                for x in d {
                    *by_kind
                        .entry(format!("{} {}x{}", fmt_name(x.format), x.width, x.height))
                        .or_default() += 1;
                    listing += &format!(
                        "{name}\tlevel\t{} {} {} {}x{} max {} blocks {}/{}\n",
                        x.face,
                        x.level,
                        fmt_name(x.format),
                        x.width,
                        x.height,
                        x.max,
                        x.blocks,
                        x.of
                    );
                }
                levels.extend(d.iter().cloned());
            }
            _ => {}
        }
    }
    if !panics.is_empty() {
        println!("stopped (not provided):");
        let mut v: Vec<_> = panics.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        for (s, n) in v.iter().take(20) {
            println!("  {n}  {s}");
        }
    }
    if !shapes.is_empty() {
        println!("different shape:");
        let mut v: Vec<_> = shapes.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        for (s, n) in v.iter().take(20) {
            println!("  {n}  {s}");
        }
    }
    if !by_kind.is_empty() {
        let mut v: Vec<_> = by_kind.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        println!(
            "levels that differ, by format and size: {}",
            v.iter()
                .take(40)
                .map(|(k, n)| format!("{k} {n}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        levels.sort_by(|a, b| b.max.cmp(&a.max).then(b.blocks.cmp(&a.blocks)));
        println!("worst levels:");
        for d in levels.iter().take(worst_n) {
            print!(
                "  {:3}/255  {} {}x{} face {} level {}  {}/{} blocks  {}",
                d.max,
                fmt_name(d.format),
                d.width,
                d.height,
                d.face,
                d.level,
                d.blocks,
                d.of,
                d.path
            );
            if let Some((x, y, a, b)) = &d.first {
                print!("  first at block {x},{y}: ours {} DLL {}", hex(a), hex(b));
            }
            println!();
        }
    }
    if let Some(l) = list {
        std::fs::File::create(l)
            .unwrap()
            .write_all(listing.as_bytes())
            .unwrap();
    }
}
