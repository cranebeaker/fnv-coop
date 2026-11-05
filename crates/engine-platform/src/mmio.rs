//! The platform's WinMM buffered file I/O: `mmioOpenA`, `mmioDescend`, `mmioAscend`, `mmioRead`,
//! `mmioGetInfo`, `mmioAdvance` and `mmioClose` (WINMM imports `00fdf318`..`00fdf330`). The game
//! calls them only from its WAV reader `00aeb270`, always with its own I/O procedure (`00af0400`,
//! four-character code `WAV\0`, which reads through the game's archives) and `MMIO_ALLOCBUF`.
//! Nothing here touches a file: every byte comes through that procedure, as on Windows.
//!
//! The behaviour is Windows' as measured by `scripts/mmio-measure` (2026-10-23, this machine):
//! the game's call sequence on 25 made-up WAV images behind a procedure that answers as the
//! game's does, every message the procedure got and every answer logged
//! (`mmio_measured.tsv`, replayed by the tests here):
//! - `mmioOpenA` calls `MMIOM_OPEN` (name, 0) and nothing else; the caller's `MMIOINFO` is left
//!   as it was, except `wErrorRet` = the procedure's answer when that is not 0 (then the answer
//!   is NULL and nothing more is called).
//! - The buffer is 8192 bytes. It is filled only when a read needs a byte past what it holds:
//!   `lBufOffset` moves on by what it held, then `MMIOM_READ` (the buffer, 8192).
//! - Moving to a position inside what the buffer holds (its end included) only moves the
//!   buffer pointer; anywhere else it is `MMIOM_SEEK` (position, `SEEK_SET`), whose answer is
//!   not used, and the buffer then starts there, empty.
//! - `mmioDescend` clears `dwFlags`, reads the 8-byte header into `ckid`/`cksize`, sets
//!   `fccType` 0 and `dwDataOffset` past the header, and for `RIFF` or `LIST` reads `fccType`.
//!   With `MMIO_FINDCHUNK` it moves past each chunk that is not the one asked for (its size
//!   rounded up to even) and reads the next header, with no check against the parent's end.
//!   No header (the procedure has no more bytes): `MMIOERR_CHUNKNOTFOUND`, the last header read
//!   left in the chunk.
//! - `mmioAscend` moves to the end of the chunk (size rounded up to even).
//! - `mmioGetInfo` copies the handle's `MMIOINFO`: `dwFlags` `MMIO_ALLOCBUF`, the procedure's
//!   code and address, `cchBuffer`, the buffer pointers (`pchEndWrite` at the buffer's end),
//!   `lBufOffset`, `lDiskOffset`, `adwInfo` as the procedure keeps them, `hmmio` the handle and
//!   0 elsewhere. `mmioAdvance` fills the buffer again and copies it the same way.
//!
//! Not provided (the run stops): other open flags, a buffer the caller gives, opening without
//! an I/O procedure, writing, `MMIO_FINDRIFF`/`MMIO_FINDLIST`, moving backwards, a read larger
//! than the buffer, a header cut short by the end of the file, and `mmioAdvance` finding no
//! more bytes (the game then copies bytes the buffer held from before, which on Windows are
//! whatever its heap held).

use crate::{Engine, Ptr};

/// `MMIOINFO` fields (mmsystem.h, 0x48 bytes).
const DW_FLAGS: u32 = 0;
const FCC_IO_PROC: u32 = 4;
const P_IO_PROC: u32 = 8;
const W_ERROR_RET: u32 = 0xc;
const CCH_BUFFER: u32 = 0x14;
const PCH_BUFFER: u32 = 0x18;
const PCH_NEXT: u32 = 0x1c;
const PCH_END_READ: u32 = 0x20;
const PCH_END_WRITE: u32 = 0x24;
const L_BUF_OFFSET: u32 = 0x28;
const ADW_INFO: u32 = 0x30;
const HMMIO: u32 = 0x44;
pub const MMIOINFO_SIZE: u32 = 0x48;
/// `MMCKINFO` fields.
const CKID: u32 = 0;
const CKSIZE: u32 = 4;
const FCC_TYPE: u32 = 8;
const DATA_OFFSET: u32 = 0xc;
const CK_FLAGS: u32 = 0x10;

/// Messages to the I/O procedure.
pub const MMIOM_READ: u32 = 0;
pub const MMIOM_SEEK: u32 = 2;
pub const MMIOM_OPEN: u32 = 3;
pub const MMIOM_CLOSE: u32 = 4;
/// `MMIO_ALLOCBUF`, `MMIO_FINDCHUNK`, `MMIO_DIRTY`; `MMIOERR_CHUNKNOTFOUND`.
pub const MMIO_ALLOCBUF: u32 = 0x1_0000;
pub const MMIO_FINDCHUNK: u32 = 0x10;
const MMIO_DIRTY: u32 = 0x1_0000;
pub const MMIOERR_CHUNKNOTFOUND: u32 = 0x109;
/// The buffer `MMIO_ALLOCBUF` makes (measured: `cchBuffer` 8192).
pub const BUFFER_SIZE: u32 = 8192;
const RIFF: u32 = u32::from_le_bytes(*b"RIFF");
const LIST: u32 = u32::from_le_bytes(*b"LIST");

fn io(e: &mut Engine, h: u32, msg: u32, p1: u32, p2: u32) -> u32 {
    let procedure = e.mem.u32(h + P_IO_PROC);
    e.call(procedure, &[h, msg, p1, p2]).u32()
}

/// The read position (`lBufOffset` plus where `pchNext` is).
fn position(e: &Engine, h: u32) -> u32 {
    let buffer = e.mem.u32(h + PCH_BUFFER);
    e.mem
        .u32(h + L_BUF_OFFSET)
        .wrapping_add(e.mem.u32(h + PCH_NEXT).wrapping_sub(buffer))
}

/// Fills the buffer again: `lBufOffset` moves on by what it held, then `MMIOM_READ`. Answers the
/// bytes read.
fn refill(e: &mut Engine, h: u32) -> u32 {
    let buffer = e.mem.u32(h + PCH_BUFFER);
    let held = e.mem.u32(h + PCH_END_READ).wrapping_sub(buffer);
    let offset = e.mem.u32(h + L_BUF_OFFSET).wrapping_add(held);
    e.mem.set_u32(h + L_BUF_OFFSET, offset);
    e.mem.set_u32(h + PCH_NEXT, buffer);
    e.mem.set_u32(h + PCH_END_READ, buffer);
    let size = e.mem.u32(h + CCH_BUFFER);
    let got = io(e, h, MMIOM_READ, buffer, size);
    assert!(
        (got as i32) >= 0 && got <= size,
        "the mmio I/O procedure answered {got:#x} to MMIOM_READ: not provided"
    );
    e.mem.set_u32(h + PCH_END_READ, buffer + got);
    got
}

/// Reads up to `count` bytes through the buffer into `to`; answers how many there were.
fn read_bytes(e: &mut Engine, h: u32, to: u32, count: u32) -> u32 {
    let mut done = 0;
    while done < count {
        let next = e.mem.u32(h + PCH_NEXT);
        let end = e.mem.u32(h + PCH_END_READ);
        if next == end {
            if refill(e, h) == 0 {
                break;
            }
            continue;
        }
        let n = (end - next).min(count - done);
        let bytes = e.mem.bytes(next, n);
        e.mem.write(to + done, &bytes);
        e.mem.set_u32(h + PCH_NEXT, next + n);
        done += n;
    }
    done
}

/// Moves the read position to `to`.
fn seek(e: &mut Engine, h: u32, to: u32) {
    let buffer = e.mem.u32(h + PCH_BUFFER);
    let start = e.mem.u32(h + L_BUF_OFFSET);
    let held = e.mem.u32(h + PCH_END_READ) - buffer;
    assert!(
        to >= position(e, h),
        "mmio: moving back from {} to {to} is not provided",
        position(e, h)
    );
    if to.wrapping_sub(start) <= held {
        e.mem.set_u32(h + PCH_NEXT, buffer + (to - start));
        return;
    }
    e.mem.set_u32(h + L_BUF_OFFSET, to);
    io(e, h, MMIOM_SEEK, to, 0);
    e.mem.set_u32(h + PCH_NEXT, buffer);
    e.mem.set_u32(h + PCH_END_READ, buffer);
}

// Platform: replaces WINMM.DLL mmioOpenA (import)
/// `mmioOpenA(name, info, flags)` with the caller's I/O procedure (`info->fccIOProc` and
/// `info->pIOProc`) and `MMIO_ALLOCBUF`: a handle with an 8192-byte buffer, after `MMIOM_OPEN`
/// (name, 0) answered 0; otherwise NULL with the answer in `info->wErrorRet`.
pub fn mmio_open_a(e: &mut Engine, name: Ptr, info: Ptr, flags: u32) -> u32 {
    let i = info.addr();
    assert!(
        flags == MMIO_ALLOCBUF
            && i != 0
            && e.mem.u32(i + P_IO_PROC) != 0
            && e.mem.u32(i + CCH_BUFFER) == 0
            && e.mem.u32(i + PCH_BUFFER) == 0,
        "mmioOpenA with flags {flags:#x} and this MMIOINFO is not provided (only an I/O \
         procedure with MMIO_ALLOCBUF)"
    );
    if !name.is_null() {
        let what = format!(
            "mmioOpen {}",
            String::from_utf8_lossy(&e.mem.cstr(name.addr()))
        );
        super::dsound::note(e, &what);
    }
    let h = e.mem.alloc(MMIOINFO_SIZE);
    let buffer = e.mem.alloc(BUFFER_SIZE);
    e.mem.set_u32(h + DW_FLAGS, MMIO_ALLOCBUF);
    e.mem.set_u32(h + FCC_IO_PROC, e.mem.u32(i + FCC_IO_PROC));
    e.mem.set_u32(h + P_IO_PROC, e.mem.u32(i + P_IO_PROC));
    e.mem.set_u32(h + CCH_BUFFER, BUFFER_SIZE);
    e.mem.set_u32(h + PCH_BUFFER, buffer);
    e.mem.set_u32(h + PCH_NEXT, buffer);
    e.mem.set_u32(h + PCH_END_READ, buffer);
    e.mem.set_u32(h + PCH_END_WRITE, buffer + BUFFER_SIZE);
    for k in 0..3 {
        e.mem
            .set_u32(h + ADW_INFO + 4 * k, e.mem.u32(i + ADW_INFO + 4 * k));
    }
    e.mem.set_u32(h + HMMIO, h);
    let answer = io(e, h, MMIOM_OPEN, name.addr(), 0);
    if answer != 0 {
        e.mem.set_u32(i + W_ERROR_RET, answer);
        e.mem.free(buffer);
        e.mem.free(h);
        return 0;
    }
    h
}

// Platform: replaces WINMM.DLL mmioClose (import)
/// `mmioClose(h, 0)`: `MMIOM_CLOSE` (0, 0), the buffer and handle freed; answers what the
/// procedure answered.
pub fn mmio_close(e: &mut Engine, h: u32, flags: u32) -> u32 {
    assert!(flags == 0, "mmioClose flags {flags:#x} are not provided");
    let answer = io(e, h, MMIOM_CLOSE, 0, 0);
    let buffer = e.mem.u32(h + PCH_BUFFER);
    e.mem.free(buffer);
    e.mem.free(h);
    answer
}

// Platform: replaces WINMM.DLL mmioRead (import)
/// `mmioRead(h, to, count)`: up to `count` bytes through the buffer; answers how many.
pub fn mmio_read(e: &mut Engine, h: u32, to: Ptr, count: i32) -> i32 {
    assert!(
        count >= 0 && (count as u32) < BUFFER_SIZE,
        "mmioRead of {count} bytes is not provided"
    );
    read_bytes(e, h, to.addr(), count as u32) as i32
}

// Platform: replaces WINMM.DLL mmioDescend (import)
/// `mmioDescend(h, chunk, parent, flags)` with flags 0 or `MMIO_FINDCHUNK` (see the module
/// documentation). `parent` is not read (measured: no check against its end).
pub fn mmio_descend(e: &mut Engine, h: u32, chunk: Ptr, _parent: Ptr, flags: u32) -> u32 {
    assert!(
        flags == 0 || flags == MMIO_FINDCHUNK,
        "mmioDescend flags {flags:#x} are not provided"
    );
    let c = chunk.addr();
    let wanted = e.mem.u32(c + CKID);
    e.mem.set_u32(c + CK_FLAGS, 0);
    let header = e.mem.alloc(8);
    let answer = loop {
        let got = read_bytes(e, h, header, 8);
        if got == 0 {
            break MMIOERR_CHUNKNOTFOUND;
        }
        assert!(
            got == 8,
            "mmioDescend: a chunk header cut short ({got} of 8 bytes) is not provided"
        );
        let id = e.mem.u32(header);
        let size = e.mem.u32(header + 4);
        e.mem.set_u32(c + CKID, id);
        e.mem.set_u32(c + CKSIZE, size);
        e.mem.set_u32(c + FCC_TYPE, 0);
        let data = position(e, h);
        e.mem.set_u32(c + DATA_OFFSET, data);
        if id == RIFF || id == LIST {
            let got = read_bytes(e, h, header, 4);
            assert!(
                got == 4,
                "mmioDescend: a form type cut short is not provided"
            );
            e.mem.set_u32(c + FCC_TYPE, e.mem.u32(header));
        }
        if flags == 0 || id == wanted {
            break 0;
        }
        seek(e, h, data.wrapping_add(size.wrapping_add(1) & !1));
    };
    e.mem.free(header);
    answer
}

// Platform: replaces WINMM.DLL mmioAscend (import)
/// `mmioAscend(h, chunk, 0)`: moves to the end of the chunk (its size rounded up to even);
/// answers 0. A chunk written to (`MMIO_DIRTY`) is not provided.
pub fn mmio_ascend(e: &mut Engine, h: u32, chunk: Ptr, flags: u32) -> u32 {
    let c = chunk.addr();
    assert!(
        flags == 0 && e.mem.u32(c + CK_FLAGS) & MMIO_DIRTY == 0,
        "mmioAscend after writing is not provided"
    );
    let end = e
        .mem
        .u32(c + DATA_OFFSET)
        .wrapping_add(e.mem.u32(c + CKSIZE).wrapping_add(1) & !1);
    seek(e, h, end);
    0
}

// Platform: replaces WINMM.DLL mmioGetInfo (import)
/// `mmioGetInfo(h, info, 0)`: the handle's `MMIOINFO` copied to `info`; answers 0.
pub fn mmio_get_info(e: &mut Engine, h: u32, info: Ptr, flags: u32) -> u32 {
    assert!(flags == 0, "mmioGetInfo flags {flags:#x} are not provided");
    let bytes = e.mem.bytes(h, MMIOINFO_SIZE);
    e.mem.write(info.addr(), &bytes);
    0
}

// Platform: replaces WINMM.DLL mmioAdvance (import)
/// `mmioAdvance(h, info, MMIO_READ)` (the game passes 0, which is `MMIO_READ`): the buffer
/// filled again and the handle's `MMIOINFO` copied to `info`; answers 0. Finding no more bytes
/// is not provided (see the module documentation).
pub fn mmio_advance(e: &mut Engine, h: u32, info: Ptr, flags: u32) -> u32 {
    assert!(flags == 0, "mmioAdvance flags {flags:#x} are not provided");
    let got = refill(e, h);
    assert!(
        got != 0,
        "mmioAdvance at the end of the file is not provided (the game would copy what the \
         buffer held before)"
    );
    mmio_get_info(e, h, info, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The I/O procedure's address in the tests.
    const PROCEDURE: u32 = 0x00af_0400;

    /// What the test procedure serves and logs, answering as the game's `00af0400` does.
    #[derive(Default)]
    struct Served {
        image: Vec<u8>,
        position: usize,
        fail_open: bool,
        log: Vec<String>,
    }

    fn procedure(e: &mut Engine, served: &Rc<RefCell<Served>>) {
        let s = served.clone();
        e.register_double(PROCEDURE, move |e, a| {
            let (info, msg, p1, p2) = (a[0], a[1], a[2], a[3]);
            let mut s = s.borrow_mut();
            let buffer = e.mem.u32(info + PCH_BUFFER);
            let shown = |p: u32| {
                if p >= buffer && p <= buffer + BUFFER_SIZE {
                    format!("buf+{}", p - buffer)
                } else {
                    "abs".to_string()
                }
            };
            let (shown1, answer) = match msg {
                MMIOM_OPEN => {
                    if s.fail_open {
                        ("name".to_string(), 0x101)
                    } else {
                        s.position = 0;
                        e.mem.set_u32(info + 0x2c, 0);
                        e.mem.set_u32(info + ADW_INFO, 1);
                        ("name".to_string(), 0)
                    }
                }
                MMIOM_READ => {
                    let n = (p2 as usize).min(s.image.len().saturating_sub(s.position));
                    let bytes = s.image[s.position..s.position + n].to_vec();
                    e.mem.write(p1, &bytes);
                    s.position += n;
                    let disk = e.mem.u32(info + 0x2c);
                    e.mem.set_u32(info + 0x2c, disk + n as u32);
                    (shown(p1), n as u32)
                }
                MMIOM_SEEK => {
                    let to = (p1 as usize).min(s.image.len());
                    s.position = to;
                    e.mem.set_u32(info + 0x2c, to as u32);
                    (format!("{}", p1 as i32), 0)
                }
                MMIOM_CLOSE => ("0".to_string(), 0),
                _ => (format!("{p1:x}"), 0x10c),
            };
            let shown2 = if msg == MMIOM_READ || msg == MMIOM_SEEK {
                format!("{}", p2 as i32)
            } else {
                format!("{p2:x}")
            };
            let line = format!(
                "proc\t{msg}\t{shown1}\t{shown2}\t{answer}\t{}\t{}",
                e.mem.u32(info + L_BUF_OFFSET) as i32,
                e.mem.u32(info + 0x2c) as i32
            );
            s.log.push(line);
            crate::abi::RetVal::into_ret(answer)
        });
    }

    fn chunk(id: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut v = id.to_vec();
        v.extend_from_slice(&(body.len() as u32).to_le_bytes());
        v.extend_from_slice(body);
        if body.len() % 2 == 1 {
            v.push(0);
        }
        v
    }

    fn riff(chunks: &[Vec<u8>]) -> Vec<u8> {
        let mut body = b"WAVE".to_vec();
        for c in chunks {
            body.extend_from_slice(c);
        }
        chunk(b"RIFF", &body)
    }

    fn fmt(tag: u16, channels: u16, rate: u32, bits: u16, extra: &[u8]) -> Vec<u8> {
        let align = channels * bits / 8;
        let mut b = Vec::new();
        b.extend_from_slice(&tag.to_le_bytes());
        b.extend_from_slice(&channels.to_le_bytes());
        b.extend_from_slice(&rate.to_le_bytes());
        b.extend_from_slice(&(rate * align as u32).to_le_bytes());
        b.extend_from_slice(&align.to_le_bytes());
        b.extend_from_slice(&bits.to_le_bytes());
        b.extend_from_slice(extra);
        chunk(b"fmt ", &b)
    }

    fn data(n: usize) -> Vec<u8> {
        chunk(
            b"data",
            &(0..n).map(|i| (i * 7 + 3) as u8).collect::<Vec<_>>(),
        )
    }

    fn smpl() -> Vec<u8> {
        chunk(b"smpl", &(0..0x3cu8).collect::<Vec<_>>())
    }

    /// The images `scripts/mmio-measure` used, by case name.
    fn image(name: &str) -> Vec<u8> {
        let f = fmt(1, 1, 22050, 16, &[]);
        match name {
            "plain_100" => riff(&[f, data(100)]),
            "plain_odd_101" => riff(&[f, data(101)]),
            "plain_20000" => riff(&[f, data(20000)]),
            "plain_8192" => riff(&[f, data(8192)]),
            "plain_8184" => riff(&[f, data(8184)]),
            "fmt18" => riff(&[fmt(1, 2, 44100, 16, &[0, 0]), data(64)]),
            "list_before_fmt" => riff(&[chunk(b"LIST", &[1u8; 33]), f, data(64)]),
            "big_chunk_before_data" => riff(&[f, chunk(b"junk", &vec![9u8; 12001]), data(300)]),
            "smpl_after_data" => riff(&[f, data(5000), smpl()]),
            "smpl_after_odd_data" => riff(&[f, data(4097), smpl()]),
            "no_smpl" => riff(&[f, data(64)]),
            "data_before_fmt" => riff(&[data(64), f]),
            "no_data" => riff(&[f]),
            "no_fmt" => riff(&[data(64)]),
            "fmt_short" => riff(&[chunk(b"fmt ", &[1, 0, 1, 0, 0x22, 0x56, 0, 0]), data(64)]),
            "adpcm" => riff(&[fmt(2, 1, 22050, 4, &[0, 0]), data(64)]),
            "not_riff" => chunk(b"JUNK", &[0u8; 40]),
            "empty" | "open_fail" => Vec::new(),
            "truncated_data" => {
                let mut v = riff(&[f, data(1000)]);
                v.truncate(v.len() - 600);
                v
            }
            "riff_size_short" => {
                let mut v = riff(&[f, data(1000)]);
                v[4..8].copy_from_slice(&40u32.to_le_bytes());
                v
            }
            "header_straddle" => riff(&[chunk(b"LIST", &vec![1u8; 8188 - 20]), f, data(64)]),
            "fmt_straddle" => riff(&[chunk(b"LIST", &vec![1u8; 8188 - 28]), f, data(64)]),
            "data_at_8192" => riff(&[f, chunk(b"LIST", &vec![1u8; 8192 - 44]), data(5000)]),
            "rifx" => {
                let mut v = riff(&[f, data(64)]);
                v[0..4].copy_from_slice(b"RIFX");
                v
            }
            other => panic!("no image {other}"),
        }
    }

    fn ck(e: &Engine, c: u32) -> String {
        format!(
            "ckid {:08x} cksize {} type {:08x} offset {} flags {:x}",
            e.mem.u32(c),
            e.mem.u32(c + 4),
            e.mem.u32(c + 8),
            e.mem.u32(c + 0xc),
            e.mem.u32(c + 0x10)
        )
    }

    fn info_fields(e: &Engine, i: u32) -> String {
        let b = e.mem.u32(i + PCH_BUFFER) as i32;
        let rel = |p: u32| {
            if p == 0 {
                "0".to_string()
            } else {
                format!("{}", p as i32 - b)
            }
        };
        format!(
            "flags {:x} cch {} next {} endread {} endwrite {} bufoffset {} diskoffset {} info0 {} err {}",
            e.mem.u32(i),
            e.mem.u32(i + CCH_BUFFER),
            rel(e.mem.u32(i + PCH_NEXT)),
            rel(e.mem.u32(i + PCH_END_READ)),
            rel(e.mem.u32(i + PCH_END_WRITE)),
            e.mem.u32(i + L_BUF_OFFSET) as i32,
            e.mem.u32(i + 0x2c) as i32,
            e.mem.u32(i + ADW_INFO),
            e.mem.u32(i + W_ERROR_RET)
        )
    }

    /// The game's sequence (`00aeb270`), logged as `scripts/mmio-measure` logs it.
    fn game_sequence(e: &mut Engine, served: &Rc<RefCell<Served>>) {
        let note = |s: String| served.borrow_mut().log.push(s);
        let name = e.mem.alloc(32);
        e.mem.set_cstr(name, b"Data\\Sound\\test.wav");
        let info = e.mem.alloc(MMIOINFO_SIZE);
        e.mem.set_u32(info + FCC_IO_PROC, 0x0056_4157);
        e.mem.set_u32(info + P_IO_PROC, PROCEDURE);
        let h = mmio_open_a(e, Ptr::new(name), Ptr::new(info), MMIO_ALLOCBUF);
        note(format!(
            "call\tmmioOpenA\t{}\terr {} {}",
            u32::from(h != 0),
            e.mem.u32(info + W_ERROR_RET),
            info_fields(e, info)
        ));
        if h == 0 {
            return;
        }
        let parent = e.mem.alloc(20);
        let c = e.mem.alloc(20);
        for k in 0..5 {
            e.mem.set_u32(parent + 4 * k, 0xaaaa_aaaa);
            e.mem.set_u32(c + 4 * k, 0xaaaa_aaaa);
        }
        e.mem.set_u32(c, u32::from_le_bytes(*b"fmt "));
        let close = |e: &mut Engine| {
            let r = mmio_close(e, h, 0);
            served
                .borrow_mut()
                .log
                .push(format!("call\tmmioClose\t{r:x}"));
        };
        let r = mmio_descend(e, h, Ptr::new(parent), Ptr::new(0), 0);
        note(format!("call\tmmioDescend riff\t{r:x}\t{}", ck(e, parent)));
        if r == MMIOERR_CHUNKNOTFOUND {
            return close(e);
        }
        let r = mmio_descend(e, h, Ptr::new(c), Ptr::new(parent), MMIO_FINDCHUNK);
        note(format!("call\tmmioDescend fmt\t{r:x}\t{}", ck(e, c)));
        if r == MMIOERR_CHUNKNOTFOUND || e.mem.u32(c + 4) < 0x10 {
            return close(e);
        }
        let format = e.mem.alloc(16);
        let r = mmio_read(e, h, Ptr::new(format), 0x10);
        let hex: String = e
            .mem
            .bytes(format, 16)
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect();
        note(format!("call\tmmioRead 16\t{r}\t{hex}"));
        if e.mem.u16(format) != 1 {
            return close(e);
        }
        let r = mmio_ascend(e, h, Ptr::new(c), 0);
        note(format!("call\tmmioAscend fmt\t{r:x}\t{}", ck(e, c)));
        e.mem.set_u32(c, u32::from_le_bytes(*b"data"));
        let r = mmio_descend(e, h, Ptr::new(c), Ptr::new(parent), MMIO_FINDCHUNK);
        note(format!("call\tmmioDescend data\t{r:x}\t{}", ck(e, c)));
        let i = e.mem.alloc(MMIOINFO_SIZE);
        let r = mmio_get_info(e, h, Ptr::new(i), 0);
        note(format!("call\tmmioGetInfo\t{r:x}\t{}", info_fields(e, i)));
        note(format!(
            "call\tmmioGetInfo rest\t{r:x}\tfcc {:x} proc_is_ours {} task {:x} r1 {:x} r2 {:x} hmmio_is_handle {} info1 {:x} info2 {:x}",
            e.mem.u32(i + FCC_IO_PROC),
            e.mem.u32(i + P_IO_PROC) == PROCEDURE,
            e.mem.u32(i + 0x10),
            e.mem.u32(i + 0x3c),
            e.mem.u32(i + 0x40),
            e.mem.u32(i + HMMIO) == h,
            e.mem.u32(i + 0x34),
            e.mem.u32(i + 0x38)
        ));
        // The buffer's bytes past what it holds are the heap's on Windows: that line is left out.
        note("call\tbuffer past endread zero\t-".to_string());
        let mut got = Vec::new();
        for _ in 0..e.mem.u32(c + 4) {
            if e.mem.u32(i + PCH_NEXT) == e.mem.u32(i + PCH_END_READ) {
                let r = mmio_advance(e, h, Ptr::new(i), 0);
                note(format!("call\tmmioAdvance\t{r:x}\t{}", info_fields(e, i)));
            }
            let next = e.mem.u32(i + PCH_NEXT);
            got.push(e.mem.u8(next));
            e.mem.set_u32(i + PCH_NEXT, next + 1);
        }
        let digest = got
            .iter()
            .fold(0u32, |a, &b| a.wrapping_mul(31).wrapping_add(b as u32));
        note(format!(
            "call\tdata read\t0\tbytes {} sum {digest:08x}",
            got.len()
        ));
        let r = mmio_ascend(e, h, Ptr::new(c), 0);
        note(format!("call\tmmioAscend data\t{r:x}\t{}", ck(e, c)));
        e.mem.set_u32(c, u32::from_le_bytes(*b"smpl"));
        let r = mmio_descend(e, h, Ptr::new(c), Ptr::new(parent), MMIO_FINDCHUNK);
        note(format!("call\tmmioDescend smpl\t{r:x}\t{}", ck(e, c)));
        if r != MMIOERR_CHUNKNOTFOUND {
            let s = e.mem.alloc(0x3c);
            let r = mmio_read(e, h, Ptr::new(s), 0x3c);
            let hex: String = e
                .mem
                .bytes(s, 8)
                .iter()
                .map(|x| format!("{x:02x}"))
                .collect();
            note(format!("call\tmmioRead 60\t{r}\t{hex}"));
        }
        // The tool's mmioSeek(h, 0, SEEK_CUR) probe (the game does not import mmioSeek).
        note(format!("call\tmmioSeek cur (probe)\t{}", position(e, h)));
        close(e);
    }

    /// The measured log, by case: the lines up to `end`.
    fn measured() -> Vec<(String, Vec<String>)> {
        let mut cases = Vec::new();
        for line in include_str!("mmio_measured.tsv").lines() {
            if let Some(rest) = line.strip_prefix("case\t") {
                let name = rest.split('\t').next().unwrap_or_default().to_string();
                cases.push((name, Vec::new()));
            } else if line != "end" {
                let l = if line.starts_with("call\tbuffer past endread zero") {
                    "call\tbuffer past endread zero\t-".to_string()
                } else {
                    line.to_string()
                };
                cases.last_mut().expect("a case").1.push(l);
            }
        }
        cases
    }

    /// The cases in which the game reads past the end of the file through `mmioAdvance`.
    const READS_PAST_THE_END: [&str; 3] = ["data_before_fmt", "no_data", "truncated_data"];

    #[test]
    fn the_game_s_reads_go_as_measured_on_windows() {
        let cases = measured();
        assert_eq!(cases.len(), 25);
        for (name, want) in cases {
            let mut e = Engine::new();
            let served = Rc::new(RefCell::new(Served {
                image: image(&name),
                fail_open: name == "open_fail",
                ..Default::default()
            }));
            procedure(&mut e, &served);
            let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                game_sequence(&mut e, &served)
            }))
            .is_err();
            let got = served.borrow().log.clone();
            if READS_PAST_THE_END.contains(&name.as_str()) {
                assert!(stopped, "{name}: should stop at mmioAdvance");
                // Everything up to that mmioAdvance is as measured.
                let advance = want
                    .iter()
                    .position(|l| l.starts_with("call\tmmioAdvance"))
                    .expect("an mmioAdvance");
                // The procedure's read of that mmioAdvance happened too.
                assert_eq!(got[..advance], want[..advance], "{name}");
            } else {
                assert!(!stopped, "{name} stopped");
                assert_eq!(got, want, "{name}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn opening_without_a_procedure_stops() {
        let mut e = Engine::new();
        let info = e.mem.alloc(MMIOINFO_SIZE);
        mmio_open_a(&mut e, Ptr::new(0), Ptr::new(info), MMIO_ALLOCBUF);
    }

    #[test]
    #[should_panic(expected = "flags 0x20 are not provided")]
    fn other_descend_flags_stop() {
        let mut e = Engine::new();
        let c = e.mem.alloc(20);
        mmio_descend(&mut e, 0, Ptr::new(c), Ptr::new(0), 0x20);
    }
}
