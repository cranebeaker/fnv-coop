//! `--game` and `--wait`: the library driven the way FalloutNV.exe 1.4.0.525
//! drives it, so the platform's Bink layer (`engine-core`'s
//! `units::platform::bink`) can be checked against the same calls.
//!
//! `--game` follows the game's frame copy (`00ec14d0`) and its texture tiles
//! (`00ebf8e0`, `00ebfda0`): `BinkDoFrame`, then for each 256x256 tile, row
//! by row, `BinkCopyToBufferRect(bink, tile, pitch, bink->Height, 0, 0, x, y,
//! w, h, flags)`, where `w`/`h` are 256, or for the last column/row the
//! remainder rounded up to a power of two (`00ebfc80`); then
//! `BinkNextFrame`. Each tile is a zeroed buffer of its texture's size
//! (pitch = width * 4) kept from frame to frame, as the game's textures are.
//! `game.tsv` gets, per frame: frame number, `BinkDoFrame`'s answer, each
//! copy's answer (comma list), then the SHA-256 of each tile's buffer.
//! `words.tsv` gets the first 64 words of the HBINK after `BinkOpen` and
//! after each call on the first three frames and the last two (and the
//! `BinkNextFrame` after the last one). `--dump` writes the tiles of the
//! listed frames to `tiles_NNNNN.raw` (one after the other).
//!
//! `--wait N` writes `wait.tsv`: frame, microseconds since `BinkOpen`
//! returned (QueryPerformanceCounter) when `BinkWait` first answered 0 for
//! it, `timeGetTime` then, how many polls answered non-zero before it, and
//! HBINK +0xC (`FrameNum`) then, and the answer of a
//! `BinkCopyToBufferRect` of the top left 256x256 (surface type 3) made
//! between `BinkDoFrame` and `BinkNextFrame`.

use nv_oracle_core::sha256;
use std::ffi::c_void;
use std::io::Write;

pub type BinkDoFrame = unsafe extern "system" fn(*mut u32) -> i32;
pub type BinkNextFrame = unsafe extern "system" fn(*mut u32);
pub type BinkWait = unsafe extern "system" fn(*mut u32) -> i32;
pub type BinkPause = unsafe extern "system" fn(*mut u32, i32) -> i32;
pub type BinkCopyToBufferRect = unsafe extern "system" fn(
    *mut u32,
    *mut c_void,
    i32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
) -> i32;

#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
}
#[link(name = "winmm")]
extern "system" {
    fn timeGetTime() -> u32;
}

pub struct Api {
    pub do_frame: BinkDoFrame,
    pub next: BinkNextFrame,
    pub wait: BinkWait,
    pub pause: BinkPause,
    pub copy_rect: BinkCopyToBufferRect,
}

/// `00ebfc80`: a tile remainder rounded up to a power of two.
fn round_up(n: u32) -> u32 {
    match n {
        0..=2 => n,
        3..=4 => 4,
        5..=8 => 8,
        9..=16 => 16,
        17..=32 => 32,
        33..=64 => 64,
        65..=128 => 128,
        129..=256 => 256,
        257..=512 => 512,
        _ => 1024,
    }
}

/// One texture tile: its source rectangle and texture size.
struct Tile {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    buf: Vec<u8>,
}

/// The game's tiles for a `width` x `height` movie (`00ebf8e0`, `00ebfda0`,
/// tile size 256, 4 bytes per texel): full rows, then the edge row.
fn tiles(width: u32, height: u32) -> Vec<Tile> {
    const T: u32 = 256;
    let (full_x, full_y) = (width / T, height / T);
    let (edge_w, edge_h) = (round_up(width % T), round_up(height % T));
    let cols = full_x + u32::from(edge_w != 0);
    let rows = full_y + u32::from(edge_h != 0);
    let mut out = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let w = if c < full_x { T } else { edge_w };
            let h = if r < full_y { T } else { edge_h };
            out.push(Tile {
                x: c * T,
                y: r * T,
                w,
                h,
                buf: vec![0; (w * 4 * h) as usize],
            });
        }
    }
    out
}

unsafe fn words(bink: *mut u32) -> String {
    (0..64)
        .map(|i| format!("{:08x}", *bink.add(i)))
        .collect::<Vec<_>>()
        .join(" ")
}

pub unsafe fn game(
    api: &Api,
    bink: *mut u32,
    out: &std::path::Path,
    flags: u32,
    dump: &[u32],
    count: Option<u32>,
) -> Result<(), String> {
    let (w, h, frames) = (*bink, *bink.add(1), *bink.add(2));
    let mut tiles = tiles(w, h);
    let mut tsv = std::fs::File::create(out.join("game.tsv")).map_err(|e| e.to_string())?;
    let mut wtsv = std::fs::File::create(out.join("words.tsv")).map_err(|e| e.to_string())?;
    let watch = |f: u32| f < 3 || f + 2 >= frames;
    writeln!(wtsv, "open\t{}", words(bink)).map_err(|e| e.to_string())?;
    for f in 0..count.map_or(frames, |c| c.min(frames)) {
        let d = (api.do_frame)(bink);
        if watch(f) {
            writeln!(wtsv, "doframe {f}\t{}", words(bink)).map_err(|e| e.to_string())?;
        }
        let mut rets = Vec::with_capacity(tiles.len());
        for t in tiles.iter_mut() {
            let r = (api.copy_rect)(
                bink,
                t.buf.as_mut_ptr() as *mut c_void,
                (t.w * 4) as i32,
                h,
                0,
                0,
                t.x,
                t.y,
                t.w,
                t.h,
                flags,
            );
            rets.push(r.to_string());
        }
        if watch(f) {
            writeln!(wtsv, "copy {f}\t{}", words(bink)).map_err(|e| e.to_string())?;
        }
        let mut line = format!("{f}\t{d}\t{}", rets.join(","));
        for t in &tiles {
            line.push('\t');
            line.push_str(&sha256::hex_digest(&t.buf));
        }
        writeln!(tsv, "{line}").map_err(|e| e.to_string())?;
        if dump.contains(&f) {
            let mut all = Vec::new();
            for t in &tiles {
                all.extend_from_slice(&t.buf);
            }
            std::fs::write(out.join(format!("tiles_{f:05}.raw")), all)
                .map_err(|e| e.to_string())?;
        }
        (api.next)(bink);
        if watch(f) {
            writeln!(wtsv, "next {f}\t{}", words(bink)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn now_us(freq: i64, origin: i64) -> i64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    (c - origin) * 1_000_000 / freq
}

/// `--wait`: `opened` is the counter right after `BinkOpen` returned.
pub unsafe fn wait(
    api: &Api,
    bink: *mut u32,
    out: &std::path::Path,
    count: u32,
    opened: i64,
    late: &[(u32, char, u32)],
) -> Result<(), String> {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let frames = *bink.add(2);
    let mut tsv = std::fs::File::create(out.join("wait.tsv")).map_err(|e| e.to_string())?;
    writeln!(tsv, "# qpc frequency {freq}").map_err(|e| e.to_string())?;
    let pause = |f: u32, stage: char| {
        for &(lf, ls, ms) in late {
            if lf == f && ls == stage {
                nv_win::Sleep(ms);
            }
        }
    };
    let mut tile = vec![0u8; 256 * 1024];
    for f in 0..count.min(frames) {
        pause(f, 'b');
        let mut polls = 0u64;
        while (api.wait)(bink) != 0 {
            polls += 1;
        }
        let t = now_us(freq, opened);
        let tg = timeGetTime();
        crate::dshook::log(format!("go frame {f}"));
        let num = *bink.add(3);
        pause(f, 'w');
        (api.do_frame)(bink);
        pause(f, 'd');
        let copied = (api.copy_rect)(
            bink,
            tile.as_mut_ptr() as *mut c_void,
            1024,
            *bink.add(1),
            0,
            0,
            0,
            0,
            256,
            256,
            3,
        );
        writeln!(tsv, "{f}\t{t}\t{tg}\t{polls}\t{num}\t{copied}").map_err(|e| e.to_string())?;
        (api.next)(bink);
        pause(f, 'n');
        for &(lf, ls, ms) in late {
            if lf == f && ls == 'p' {
                let r1 = (api.pause)(bink, 1);
                let start = now_us(freq, opened);
                let mut answers = std::collections::BTreeMap::new();
                while now_us(freq, opened) - start < i64::from(ms) * 1000 {
                    *answers.entry((api.wait)(bink)).or_insert(0u64) += 1;
                }
                let r0 = (api.pause)(bink, 0);
                writeln!(tsv, "# pause after {f}: BinkPause(1) {r1}, BinkPause(0) {r0}, waits while paused {answers:?}").map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

/// The counter now (the origin for [`wait`]).
pub fn counter() -> i64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    c
}
