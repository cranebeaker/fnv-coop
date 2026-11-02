//! vorbis-record: what the player's `libvorbisfile.dll` (with its `libvorbis.dll`) decodes from
//! the game's `.ogg` files, the way the game asks for it (`00af1e00`: `ov_open_callbacks` with
//! read, seek and tell callbacks and no close, `ov_info(-1)`, `ov_seekable`, `ov_pcm_total(0)`;
//! `00af2100`: `ov_read(.., 0, 2, 1, ..)`, little-endian signed 16-bit). For the platform's own
//! Vorbis decoder. 32-bit, as the game.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/vorbis-record/Cargo.toml -- <libvorbisfile.dll> <out.tsv> <archive.bsa>... [--precision 24|53|64] [--limit N] [--only TEXT --pcm DIR] [--read BYTES]`
//!
//! Reads the archives and the DLLs; writes only `out.tsv` (one line per `.ogg`: archive, path,
//! the open result, `vorbis_info`'s version, channels, rate and three bitrates, seekable, the
//! PCM total of link 0, the bytes `ov_read` gave, the number of `ov_read` calls, the SHA-256 of
//! the bytes) and, with `--pcm`, the decoded bytes of the files whose path contains `--only`.
//! These are private recordings of the player's files: keep them out of the repository.
//!
//! With `VORBIS_PACKETS=<file>` (bare packets, see crates/vorbis `diag::packets`) it runs those
//! through `libvorbis.dll` (`--blocks`) and writes their blocks to `<out.tsv>` instead; the
//! archives are then not read (give any).
//!
//! `--precision` sets the x87 precision control while the library decodes (libvorbis' float
//! code may be x87; Direct3D 9 sets 24 bits on the thread that makes the device unless
//! `D3DCREATE_FPU_PRESERVE`), default the process's (53 bits). `--read` is the length asked of
//! each `ov_read` (default 4096). `--float`: `ov_read_float` instead (up to `--read` samples a
//! call), recording the library's `f32` samples interleaved.
#![allow(clippy::missing_safety_doc)]

#[path = "../../../crates/nvinspect/src/sha256.rs"]
mod sha256;

use std::ffi::c_void;
use std::io::Write;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryExA(name: *const u8, file: *mut c_void, flags: u32) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
}

/// The data source the callbacks read: one file's bytes.
struct Source {
    data: Vec<u8>,
    at: usize,
}

unsafe extern "C" fn read(ptr: *mut u8, size: usize, n: usize, ds: *mut c_void) -> usize {
    let s = &mut *(ds as *mut Source);
    if size == 0 {
        return 0;
    }
    let items = n.min((s.data.len() - s.at) / size);
    std::ptr::copy_nonoverlapping(s.data.as_ptr().add(s.at), ptr, items * size);
    s.at += items * size;
    items
}

unsafe extern "C" fn seek(ds: *mut c_void, offset: i64, whence: i32) -> i32 {
    let s = &mut *(ds as *mut Source);
    let base = match whence {
        0 => 0i64,
        1 => s.at as i64,
        2 => s.data.len() as i64,
        _ => return -1,
    };
    let to = base + offset;
    if to < 0 || to > s.data.len() as i64 {
        return -1;
    }
    s.at = to as usize;
    0
}

unsafe extern "C" fn tell(ds: *mut c_void) -> i32 {
    (*(ds as *mut Source)).at as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Callbacks {
    read: unsafe extern "C" fn(*mut u8, usize, usize, *mut c_void) -> usize,
    seek: unsafe extern "C" fn(*mut c_void, i64, i32) -> i32,
    close: usize,
    tell: unsafe extern "C" fn(*mut c_void) -> i32,
}

type OpenCallbacks = unsafe extern "C" fn(*mut c_void, *mut u8, *const u8, i32, Callbacks) -> i32;
type Info = unsafe extern "C" fn(*mut u8, i32) -> *const i32;
type Seekable = unsafe extern "C" fn(*mut u8) -> i32;
type PcmTotal = unsafe extern "C" fn(*mut u8, i32) -> i64;
type Read = unsafe extern "C" fn(*mut u8, *mut u8, i32, i32, i32, i32, *mut i32) -> i32;
type Clear = unsafe extern "C" fn(*mut u8) -> i32;
type ReadFloat = unsafe extern "C" fn(*mut u8, *mut *const *const f32, i32, *mut i32) -> i32;

fn control_word() -> u16 {
    let mut cw: u16 = 0;
    unsafe { core::arch::asm!("fnstcw word ptr [{0}]", in(reg) &mut cw, options(nostack)) };
    cw
}

fn set_control_word(cw: u16) {
    unsafe { core::arch::asm!("fldcw word ptr [{0}]", in(reg) &cw, options(nostack)) };
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut plain = Vec::new();
    let (mut precision, mut limit, mut only, mut pcm, mut chunk) = (None, usize::MAX, None, None, 4096i32);
    let mut float = false;
    let mut blocks = false;
    let mut i = 0;
    while i < args.len() {
        let v = || args.get(i + 1).cloned().expect("a value");
        match args[i].as_str() {
            "--precision" => precision = Some(v().parse::<u32>().unwrap()),
            "--limit" => limit = v().parse().unwrap(),
            "--only" => only = Some(v().to_lowercase()),
            "--pcm" => pcm = Some(std::path::PathBuf::from(v())),
            "--read" => chunk = v().parse().unwrap(),
            "--blocks" => {
                blocks = true;
                i += 1;
                continue;
            }
            "--float" => {
                float = true;
                i += 1;
                continue;
            }
            _ => {
                plain.push(args[i].clone());
                i += 1;
                continue;
            }
        }
        i += 2;
    }
    assert!(plain.len() >= 3, "usage: vorbis-record <libvorbisfile.dll> <out.tsv> <archive.bsa>...");
    unsafe {
        let name = format!("{}\0", plain[0].replace('/', "\\"));
        // LOAD_WITH_ALTERED_SEARCH_PATH: its libvorbis.dll from the same folder.
        let dll = LoadLibraryExA(name.as_ptr(), std::ptr::null_mut(), 8);
        assert!(!dll.is_null(), "LoadLibrary {}: {}", plain[0], std::io::Error::last_os_error());
        let f = |n: &str| {
            let c = format!("{n}\0");
            let p = GetProcAddress(dll, c.as_ptr());
            assert!(!p.is_null(), "no export {n}");
            p
        };
        let open: OpenCallbacks = std::mem::transmute(f("ov_open_callbacks"));
        let info: Info = std::mem::transmute(f("ov_info"));
        let seekable: Seekable = std::mem::transmute(f("ov_seekable"));
        let pcm_total: PcmTotal = std::mem::transmute(f("ov_pcm_total"));
        let ov_read: Read = std::mem::transmute(f("ov_read"));
        let clear: Clear = std::mem::transmute(f("ov_clear"));
        let read_float: ReadFloat = std::mem::transmute(f("ov_read_float"));
        if let Some(p) = precision {
            let pc = match p {
                24 => 0u16,
                53 => 2,
                64 => 3,
                other => panic!("--precision {other}: use 24, 53 or 64"),
            };
            set_control_word((control_word() & !0x0300) | (pc << 8));
        }
        if let Some(crafted) = std::env::var_os("VORBIS_PACKETS") {
            // A measuring file of bare packets (crates/vorbis `diag::packets`): its blocks, to
            // `<out.tsv>` (the second argument) as `f32`.
            let lib = plain[0].replace('/', "\\").replace("libvorbisfile.dll", "libvorbis.dll");
            let data = std::fs::read(crafted).unwrap();
            std::fs::write(&plain[1], record_blocks(&lib, &data)).unwrap();
            return;
        }
        let mut out = std::io::BufWriter::new(std::fs::File::create(&plain[1]).unwrap());
        writeln!(out, "# control word {:04x}, read {chunk}", control_word()).unwrap();
        let mut done = 0;
        for archive in &plain[2..] {
            let a = bsa::Archive::open(archive).unwrap_or_else(|e| panic!("{archive}: {e}"));
            let short = std::path::Path::new(archive).file_name().unwrap().to_string_lossy().into_owned();
            for entry in a.files() {
                if done >= limit {
                    break;
                }
                if !entry.path.ends_with(".ogg") {
                    continue;
                }
                if let Some(o) = &only {
                    if pcm.is_none() && !entry.path.contains(o.as_str()) {
                        continue;
                    }
                }
                done += 1;
                if blocks {
                    if let (Some(dir), Some(o)) = (&pcm, &only) {
                        if entry.path.contains(o.as_str()) {
                            let lib = plain[0].replace('/', "\\").replace("libvorbisfile.dll", "libvorbis.dll");
                            let out = record_blocks(&lib, &a.read(entry).unwrap());
                            std::fs::create_dir_all(dir).unwrap();
                            let file = entry.path.replace(['\\', '/'], "_");
                            std::fs::write(dir.join(format!("{file}.blocks")), out).unwrap();
                        }
                    }
                    continue;
                }
                let mut src = Source { data: a.read(entry).unwrap(), at: 0 };
                let mut vf = vec![0u8; 0x1000];
                let cb = Callbacks { read, seek, close: 0, tell };
                let r = open(&mut src as *mut _ as *mut c_void, vf.as_mut_ptr(), std::ptr::null(), 0, cb);
                if r != 0 {
                    writeln!(out, "{short}\t{}\topen {r}", entry.path).unwrap();
                    continue;
                }
                let vi = info(vf.as_mut_ptr(), -1);
                let fields: Vec<i32> = (0..6).map(|k| *vi.add(k)).collect();
                let s = seekable(vf.as_mut_ptr());
                let total = if s != 0 { pcm_total(vf.as_mut_ptr(), 0) } else { -1 };
                let mut pcm_bytes = Vec::new();
                let mut buf = vec![0u8; chunk as usize];
                let mut calls = 0u32;
                let mut bitstream = 0i32;
                let channels = fields[1] as usize;
                let last = loop {
                    if float {
                        let mut p: *const *const f32 = std::ptr::null();
                        let n = read_float(vf.as_mut_ptr(), &mut p, chunk, &mut bitstream);
                        calls += 1;
                        if n <= 0 {
                            break n;
                        }
                        for j in 0..n as usize {
                            for c in 0..channels {
                                pcm_bytes.extend_from_slice(&(*(*p.add(c)).add(j)).to_le_bytes());
                            }
                        }
                        continue;
                    }
                    let n = ov_read(vf.as_mut_ptr(), buf.as_mut_ptr(), chunk, 0, 2, 1, &mut bitstream);
                    calls += 1;
                    if n <= 0 {
                        break n;
                    }
                    pcm_bytes.extend_from_slice(&buf[..n as usize]);
                };
                clear(vf.as_mut_ptr());
                writeln!(
                    out,
                    "{short}\t{}\topen 0\tinfo {}\tseekable {s}\ttotal {total}\tbytes {}\tcalls {calls}\tlast {last}\tsha256 {}",
                    entry.path,
                    fields.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "),
                    pcm_bytes.len(),
                    sha256::hex_digest(&pcm_bytes)
                )
                .unwrap();
                if let (Some(dir), Some(o)) = (&pcm, &only) {
                    if entry.path.contains(o.as_str()) {
                        std::fs::create_dir_all(dir).unwrap();
                        let file = entry.path.replace(['\\', '/'], "_");
                        std::fs::write(dir.join(format!("{file}.pcm")), &pcm_bytes).unwrap();
                    }
                }
            }
        }
        eprintln!("{done} files");
    }
}

/// `ogg_packet`.
#[repr(C)]
struct OggPacket {
    packet: *const u8,
    bytes: i32,
    b_o_s: i32,
    e_o_s: i32,
    granulepos: i64,
    packetno: i64,
}

/// `--blocks`: drives `libvorbis.dll` itself (`vorbis_synthesis_headerin`, `vorbis_synthesis_init`,
/// `vorbis_block_init`, then `vorbis_synthesis` for each audio packet) and records each block's
/// output (`vb->pcm`, every channel's `pcmend` samples after the inverse MDCT) as `f32`.
unsafe fn record_blocks(lib: &str, data: &[u8]) -> Vec<u8> {
    let name = format!("{lib}\0");
    let dll = LoadLibraryExA(name.as_ptr(), std::ptr::null_mut(), 8);
    assert!(!dll.is_null(), "LoadLibrary {lib}");
    let f = |n: &str| {
        let c = format!("{n}\0");
        let p = GetProcAddress(dll, c.as_ptr());
        assert!(!p.is_null(), "no export {n}");
        p
    };
    type F1 = unsafe extern "C" fn(*mut u8) -> i32;
    type F2 = unsafe extern "C" fn(*mut u8, *mut u8) -> i32;
    type F3 = unsafe extern "C" fn(*mut u8, *mut u8, *const OggPacket) -> i32;
    let info_init: F1 = std::mem::transmute(f("vorbis_info_init"));
    let comment_init: F1 = std::mem::transmute(f("vorbis_comment_init"));
    let headerin: F3 = std::mem::transmute(f("vorbis_synthesis_headerin"));
    let synthesis_init: F2 = std::mem::transmute(f("vorbis_synthesis_init"));
    let block_init: F2 = std::mem::transmute(f("vorbis_block_init"));
    let synthesis: unsafe extern "C" fn(*mut u8, *const OggPacket) -> i32 = std::mem::transmute(f("vorbis_synthesis"));
    let (mut vi, mut vc, mut vd, mut vb) = (vec![0u8; 4096], vec![0u8; 4096], vec![0u8; 4096], vec![0u8; 4096]);
    info_init(vi.as_mut_ptr());
    comment_init(vc.as_mut_ptr());
    let packets = vorbis::diag::packets(data);
    let op = |p: &vorbis::diag::RawPacket| OggPacket {
        packet: p.0.as_ptr(),
        bytes: p.0.len() as i32,
        b_o_s: i32::from(p.4) * 256,
        e_o_s: i32::from(p.3) * 512,
        granulepos: p.1,
        packetno: p.2,
    };
    for p in &packets[..3] {
        let r = headerin(vi.as_mut_ptr(), vc.as_mut_ptr(), &op(p));
        assert_eq!(r, 0, "vorbis_synthesis_headerin");
    }
    synthesis_init(vd.as_mut_ptr(), vi.as_mut_ptr());
    block_init(vd.as_mut_ptr(), vb.as_mut_ptr());
    let channels = i32::from_le_bytes(vi[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::new();
    for p in &packets[3..] {
        if synthesis(vb.as_mut_ptr(), &op(p)) != 0 {
            continue;
        }
        let pcm = *(vb.as_ptr() as *const *const *const f32);
        let pcmend = i32::from_le_bytes(vb[36..40].try_into().unwrap()) as usize;
        for c in 0..channels {
            let ch = *pcm.add(c);
            for j in 0..pcmend {
                out.extend_from_slice(&(*ch.add(j)).to_le_bytes());
            }
        }
    }
    out
}
