//! d3dx-texture-capture: fills the machine-local D3DX texture cache the platform's D3DX layer
//! (`engine_core::units::platform::d3dx`) reads, in one pass over the game's textures.
//!
//! `cargo run --release --manifest-path scripts/d3dx-texture-capture/Cargo.toml -- <Data folder>
//! <cache folder> <d3dx-measure.exe (i686 build)>`
//!
//! The game's archives (`*.bsa` in the Data folder) and loose files under `Data\Textures` are read
//! only. Every `.dds` file goes, as the game hands it to D3DX, to the player's `d3dx9_38.dll` in a
//! 32-bit process (`d3dx-measure serve`): `D3DXCreateTextureFromFileInMemory`, or
//! `D3DXCreateCubeTextureFromFileInMemory` for a cube map (the game's 00e68a80 picks by the
//! image info's resource type). Each answer is written as `<key>.tex` in the cache folder, the
//! key being [`key`] of the call and its bytes; levels the DLL left exactly as the file stores
//! them are marked "copy" (the platform copies those itself, by its own rule, and checks the
//! rule agrees), the others (levels D3DX made itself: sub-block DXT levels, generated mips,
//! converted formats) keep their bytes. `index.tsv` lists what was written.
//!
//! The cache comes from the player's own game files: it stays on the player's machine (like the
//! machine table) and never goes into the repository. Each player runs this once; the DLL comes
//! with the game's DirectX redistributables.
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// FNV-1a, 64 bits.
fn fnv(seed: u64, b: &[u8]) -> u64 {
    b.iter().fold(seed, |h, &x| (h ^ u64::from(x)).wrapping_mul(0x100_0000_01b3))
}

/// The cache key of one call: the call's name and the data's length and FNV-1a hash (the
/// platform computes the same; `engine_core::units::platform::d3dx::texture_key`).
pub fn key(call: &str, data: &[u8]) -> String {
    let h = fnv(fnv(0xcbf2_9ce4_8422_2325, call.as_bytes()), data);
    format!("{call}-{:x}-{h:016x}", data.len())
}

fn word(d: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(d[at..at + 4].try_into().unwrap())
}

/// The file's level byte ranges, face by face (a cube stores six faces, each with its chain),
/// by the documented DDS layout; `None` when the header is not one this reads.
fn file_levels(d: &[u8], faces: u32) -> Option<Vec<(usize, usize)>> {
    if d.len() < 128 || &d[..4] != b"DDS " {
        return None;
    }
    let (height, width, mips) = (word(d, 12), word(d, 16), word(d, 28).max(1));
    let (flags, fourcc, bits) = (word(d, 80), word(d, 84), word(d, 88));
    let (block, bytes) = if flags & 4 != 0 {
        match &fourcc.to_le_bytes() {
            b"DXT1" => (4, 8),
            b"DXT2" | b"DXT3" | b"DXT4" | b"DXT5" => (4, 16),
            _ => return None,
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
    Some(out)
}

/// One answer of `d3dx-measure serve`.
struct Answer {
    hr: u32,
    levels: u32,
    faces: u32,
    /// (format, width, height, bytes) face by face, level by level.
    surfaces: Vec<(u32, u32, u32, Vec<u8>)>,
}

fn read_u32(r: &mut impl Read) -> u32 {
    let mut b = [0u8; 4];
    r.read_exact(&mut b).expect("the server's answer");
    u32::from_le_bytes(b)
}

fn ask(server: &mut std::process::Child, kind: u32, data: &[u8]) -> Answer {
    let stdin = server.stdin.as_mut().unwrap();
    stdin.write_all(&kind.to_le_bytes()).unwrap();
    stdin.write_all(&(data.len() as u32).to_le_bytes()).unwrap();
    stdin.write_all(data).unwrap();
    stdin.flush().unwrap();
    let out = server.stdout.as_mut().unwrap();
    let hr = read_u32(out);
    let levels = read_u32(out);
    let faces = read_u32(out);
    let mut surfaces = Vec::new();
    for _ in 0..levels * faces {
        let (format, w, h, len) = (read_u32(out), read_u32(out), read_u32(out), read_u32(out));
        let mut bytes = vec![0u8; len as usize];
        out.read_exact(&mut bytes).unwrap();
        surfaces.push((format, w, h, bytes));
    }
    Answer { hr, levels, faces, surfaces }
}

/// The cache entry: `NVTX0001`, the call kind, the HRESULT, level and face counts, then per
/// face and level its format, width, height, 0 (copy: the file's level as stored) or 1 (bytes
/// follow), the byte count and the bytes.
fn entry(kind: u32, a: &Answer, data: &[u8]) -> (Vec<u8>, u32, usize) {
    let mut e = b"NVTX0001".to_vec();
    for w in [kind, a.hr, a.levels, a.faces] {
        e.extend_from_slice(&w.to_le_bytes());
    }
    let file = file_levels(data, a.faces.max(1)).unwrap_or_default();
    let stored = file.len() as u32 / a.faces.max(1);
    let (mut copies, mut kept) = (0u32, 0usize);
    for (i, (format, w, h, bytes)) in a.surfaces.iter().enumerate() {
        let (face, level) = (i as u32 / a.levels, i as u32 % a.levels);
        let same = level < stored && {
            let (at, len) = file[(face * stored + level) as usize];
            len == bytes.len() && data.len() >= at + len && &data[at..at + len] == bytes.as_slice()
        };
        for w in [*format, *w, *h, u32::from(!same), bytes.len() as u32] {
            e.extend_from_slice(&w.to_le_bytes());
        }
        if same {
            copies += 1;
        } else {
            e.extend_from_slice(bytes);
            kept += bytes.len();
        }
    }
    (e, copies, kept)
}

fn loose_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                loose_files(&p, out);
            } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("dds")) {
                out.push(p);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 4, "usage: d3dx-texture-capture <Data folder> <cache folder> <d3dx-measure.exe>");
    let (data_dir, cache) = (PathBuf::from(&args[1]), PathBuf::from(&args[2]));
    std::fs::create_dir_all(&cache).unwrap();
    let mut server = Command::new(&args[3])
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("d3dx-measure serve");
    let mut index = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(cache.join("index.tsv"))
        .unwrap();
    let mut done = 0usize;
    let mut handle = |source: &str, path: &str, data: &[u8], index: &mut std::fs::File| {
        if data.len() < 128 || &data[..4] != b"DDS " {
            writeln!(index, "skipped\t{source}\t{path}\tnot a DDS file").unwrap();
            return;
        }
        let caps2 = word(data, 112);
        if caps2 & 0x20_0000 != 0 {
            writeln!(index, "skipped\t{source}\t{path}\ta volume texture (not captured)").unwrap();
            return;
        }
        let (kind, call) = if caps2 & 0x200 != 0 {
            (2, "CreateCubeTextureFromFileInMemory")
        } else {
            (1, "CreateTextureFromFileInMemory")
        };
        let k = key(call, data);
        let file = cache.join(format!("{k}.tex"));
        if file.exists() {
            return;
        }
        let answer = ask(&mut server, kind, data);
        let (bytes, copies, kept) = entry(kind, &answer, data);
        std::fs::write(&file, bytes).unwrap();
        writeln!(
            index,
            "{k}\t{source}\t{path}\t{:08x}\t{}x{}\t{copies} copied\t{kept} bytes kept",
            answer.hr, answer.faces, answer.levels
        )
        .unwrap();
        done += 1;
        if done % 500 == 0 {
            eprintln!("{done} textures");
        }
    };
    let mut archives: Vec<PathBuf> = std::fs::read_dir(&data_dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bsa")))
        .collect();
    archives.sort();
    for path in archives {
        let archive = bsa::Archive::open(&path).expect("an archive");
        let source = path.file_name().unwrap().to_string_lossy().to_string();
        for entry in archive.files() {
            if !entry.path.to_ascii_lowercase().ends_with(".dds") {
                continue;
            }
            let data = archive.read(entry).expect("an archive entry");
            handle(&source, &entry.path, &data, &mut index);
        }
        eprintln!("{source} done");
    }
    let mut loose = Vec::new();
    loose_files(&data_dir.join("Textures"), &mut loose);
    for p in loose {
        let data = std::fs::read(&p).unwrap();
        handle("loose", &p.to_string_lossy(), &data, &mut index);
    }
    drop(server.stdin.take());
    server.wait().unwrap();
    eprintln!("{done} textures written");
}
