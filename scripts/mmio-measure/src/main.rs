//! mmio-measure: how WinMM's buffered file I/O (`mmioOpenA`, `mmioDescend`, `mmioAscend`,
//! `mmioRead`, `mmioGetInfo`, `mmioAdvance`, `mmioClose`) drives a custom I/O procedure, for the
//! platform's mmio layer (`engine_core::units::platform::mmio`).
//!
//! FalloutNV.exe 1.4.0.525 reads every WAV through its own I/O procedure (`00af0400`, four-character
//! code `WAV\0`) with `MMIO_ALLOCBUF`, in the order of `00aeb270`. This tool makes the same calls
//! in the same order on made-up WAV images held in memory, behind an I/O procedure that answers as
//! the game's does (open: position 0, answer 0; read: bytes read, `lDiskOffset` moved on; seek:
//! `lDiskOffset` set to the new position, answer 0; close: 0; any other message: 0x10C), and logs
//! every message the procedure gets and every answer the API gives. Nothing touches a file.
//!
//! 32-bit, as the game: `cargo run --release --target i686-pc-windows-msvc --manifest-path
//! scripts/mmio-measure/Cargo.toml > mmio.tsv`.
//!
//! Lines (tab separated): `case <name> <size>`; `proc <msg> <p1> <p2> <answer> <lBufOffset>
//! <lDiskOffset>` (a pointer argument as `buf+N` into the MMIOINFO's buffer, or `abs`); `call
//! <function> <answer> <details>`; `end`.
use std::ffi::c_void;
use std::io::Write;

#[repr(C)]
#[derive(Clone, Copy)]
struct MmioInfo {
    flags: u32,
    fcc_io_proc: u32,
    io_proc: Option<IoProc>,
    error_ret: u32,
    task: u32,
    cch_buffer: i32,
    pch_buffer: *mut u8,
    pch_next: *mut u8,
    pch_end_read: *mut u8,
    pch_end_write: *mut u8,
    buf_offset: i32,
    disk_offset: i32,
    info: [u32; 3],
    reserved1: u32,
    reserved2: u32,
    hmmio: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CkInfo {
    ckid: u32,
    cksize: u32,
    fcc_type: u32,
    data_offset: u32,
    flags: u32,
}

type IoProc = unsafe extern "system" fn(*mut MmioInfo, u32, isize, isize) -> isize;

#[link(name = "winmm")]
extern "system" {
    fn mmioOpenA(name: *const u8, info: *mut MmioInfo, flags: u32) -> *mut c_void;
    fn mmioClose(h: *mut c_void, flags: u32) -> u32;
    fn mmioDescend(h: *mut c_void, ck: *mut CkInfo, parent: *const CkInfo, flags: u32) -> u32;
    fn mmioAscend(h: *mut c_void, ck: *mut CkInfo, flags: u32) -> u32;
    fn mmioRead(h: *mut c_void, buf: *mut u8, count: i32) -> i32;
    fn mmioGetInfo(h: *mut c_void, info: *mut MmioInfo, flags: u32) -> u32;
    fn mmioAdvance(h: *mut c_void, info: *mut MmioInfo, flags: u32) -> u32;
    fn mmioSeek(h: *mut c_void, offset: i32, origin: i32) -> i32;
}

/// The image the procedure serves, its position, and the log.
struct Served {
    image: Vec<u8>,
    position: usize,
    open: bool,
    fail_open: bool,
    log: Vec<String>,
}

static mut SERVED: Option<Served> = None;

#[allow(static_mut_refs)]
fn served() -> &'static mut Served {
    unsafe { SERVED.as_mut().expect("a case is running") }
}

fn pointer(info: &MmioInfo, p: isize) -> String {
    let b = info.pch_buffer as isize;
    if p != 0 && b != 0 && p >= b && p <= b + info.cch_buffer as isize {
        format!("buf+{}", p - b)
    } else if p == 0 {
        "0".into()
    } else {
        "abs".into()
    }
}

unsafe extern "system" fn io_proc(info: *mut MmioInfo, msg: u32, p1: isize, p2: isize) -> isize {
    let s = served();
    let i = &mut *info;
    let (shown1, answer) = match msg {
        // MMIOM_OPEN: p1 is the name.
        3 => {
            if s.fail_open {
                ("name".to_string(), 0x101)
            } else {
                s.open = true;
                s.position = 0;
                i.disk_offset = 0;
                i.info[0] = 1;
                ("name".to_string(), 0)
            }
        }
        // MMIOM_READ.
        0 => {
            let shown = pointer(i, p1);
            let want = p2.max(0) as usize;
            let n = want.min(s.image.len().saturating_sub(s.position));
            std::ptr::copy_nonoverlapping(s.image.as_ptr().add(s.position), p1 as *mut u8, n);
            s.position += n;
            i.disk_offset += n as i32;
            (shown, n as isize)
        }
        // MMIOM_SEEK: the game's procedure sets lDiskOffset to the new position and answers 0.
        2 => {
            let base = match p2 {
                0 => 0isize,
                1 => s.position as isize,
                _ => s.image.len() as isize,
            };
            let to = (base + p1).clamp(0, s.image.len() as isize) as usize;
            s.position = to;
            i.disk_offset = to as i32;
            (format!("{p1}"), 0)
        }
        // MMIOM_CLOSE.
        4 => {
            s.open = false;
            ("0".to_string(), 0)
        }
        _ => (format!("{p1:x}"), 0x10c),
    };
    let shown2 = if msg == 0 || msg == 2 {
        format!("{p2}")
    } else {
        format!("{p2:x}")
    };
    s.log.push(format!(
        "proc\t{msg}\t{shown1}\t{shown2}\t{answer}\t{}\t{}",
        i.buf_offset, i.disk_offset
    ));
    answer
}

fn fourcc(s: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*s)
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

fn riff(form: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = form.to_vec();
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

fn cases() -> Vec<(&'static str, Vec<u8>)> {
    let f = fmt(1, 1, 22050, 16, &[]);
    vec![
        ("plain_100", riff(b"WAVE", &[f.clone(), data(100)])),
        ("plain_odd_101", riff(b"WAVE", &[f.clone(), data(101)])),
        ("plain_20000", riff(b"WAVE", &[f.clone(), data(20000)])),
        ("plain_8192", riff(b"WAVE", &[f.clone(), data(8192)])),
        ("plain_8184", riff(b"WAVE", &[f.clone(), data(8184)])),
        (
            "fmt18",
            riff(b"WAVE", &[fmt(1, 2, 44100, 16, &[0, 0]), data(64)]),
        ),
        (
            "list_before_fmt",
            riff(b"WAVE", &[chunk(b"LIST", &[1u8; 33]), f.clone(), data(64)]),
        ),
        (
            "big_chunk_before_data",
            riff(
                b"WAVE",
                &[f.clone(), chunk(b"junk", &vec![9u8; 12001]), data(300)],
            ),
        ),
        (
            "smpl_after_data",
            riff(b"WAVE", &[f.clone(), data(5000), smpl()]),
        ),
        (
            "smpl_after_odd_data",
            riff(b"WAVE", &[f.clone(), data(4097), smpl()]),
        ),
        ("no_smpl", riff(b"WAVE", &[f.clone(), data(64)])),
        ("data_before_fmt", riff(b"WAVE", &[data(64), f.clone()])),
        ("no_data", riff(b"WAVE", &[f.clone()])),
        ("no_fmt", riff(b"WAVE", &[data(64)])),
        (
            "fmt_short",
            riff(
                b"WAVE",
                &[chunk(b"fmt ", &[1, 0, 1, 0, 0x22, 0x56, 0, 0]), data(64)],
            ),
        ),
        (
            "adpcm",
            riff(b"WAVE", &[fmt(2, 1, 22050, 4, &[0, 0]), data(64)]),
        ),
        ("not_riff", chunk(b"JUNK", &[0u8; 40])),
        ("empty", Vec::new()),
        ("truncated_data", {
            let mut v = riff(b"WAVE", &[f.clone(), data(1000)]);
            v.truncate(v.len() - 600);
            v
        }),
        ("riff_size_short", {
            let mut v = riff(b"WAVE", &[f.clone(), data(1000)]);
            v[4..8].copy_from_slice(&40u32.to_le_bytes());
            v
        }),
        ("open_fail", Vec::new()),
        (
            "header_straddle",
            riff(
                b"WAVE",
                &[chunk(b"LIST", &vec![1u8; 8188 - 20]), f.clone(), data(64)],
            ),
        ),
        (
            "fmt_straddle",
            riff(
                b"WAVE",
                &[chunk(b"LIST", &vec![1u8; 8188 - 28]), f.clone(), data(64)],
            ),
        ),
        (
            "data_at_8192",
            riff(
                b"WAVE",
                &[f.clone(), chunk(b"LIST", &vec![1u8; 8192 - 44]), data(5000)],
            ),
        ),
        ("rifx", {
            let mut v = riff(b"WAVE", &[f, data(64)]);
            v[0..4].copy_from_slice(b"RIFX");
            v
        }),
    ]
}

fn ck(c: &CkInfo) -> String {
    format!(
        "ckid {:08x} cksize {} type {:08x} offset {} flags {:x}",
        c.ckid, c.cksize, c.fcc_type, c.data_offset, c.flags
    )
}

fn info_fields(i: &MmioInfo) -> String {
    let b = i.pch_buffer as isize;
    let rel = |p: *mut u8| {
        if p.is_null() {
            "0".to_string()
        } else {
            format!("{}", p as isize - b)
        }
    };
    format!(
        "flags {:x} cch {} next {} endread {} endwrite {} bufoffset {} diskoffset {} info0 {} err {}",
        i.flags,
        i.cch_buffer,
        rel(i.pch_next),
        rel(i.pch_end_read),
        rel(i.pch_end_write),
        i.buf_offset,
        i.disk_offset,
        i.info[0],
        i.error_ret
    )
}

/// The game's sequence (`00aeb270`), with `smpl` read as for a looping sound.
fn note(s: String) {
    served().log.push(s);
}

unsafe fn game_sequence() {
    let mut info: MmioInfo = std::mem::zeroed();
    info.fcc_io_proc = 0x0056_4157;
    info.io_proc = Some(io_proc);
    let h = mmioOpenA(b"Data\\Sound\\test.wav\0".as_ptr(), &mut info, 0x10000);
    note(format!(
        "call\tmmioOpenA\t{}\terr {} {}",
        u32::from(!h.is_null()),
        info.error_ret,
        info_fields(&info)
    ));
    if h.is_null() {
        return;
    }
    let mut parent = CkInfo {
        ckid: 0xaaaa_aaaa,
        cksize: 0xaaaa_aaaa,
        fcc_type: 0xaaaa_aaaa,
        data_offset: 0xaaaa_aaaa,
        flags: 0xaaaa_aaaa,
    };
    let r = mmioDescend(h, &mut parent, std::ptr::null(), 0);
    note(format!("call\tmmioDescend riff\t{r:x}\t{}", ck(&parent)));
    if r == 0x109 {
        note(format!("call\tmmioClose\t{:x}", mmioClose(h, 0)));
        return;
    }
    let mut c = CkInfo {
        ckid: fourcc(b"fmt "),
        cksize: 0xaaaa_aaaa,
        fcc_type: 0xaaaa_aaaa,
        data_offset: 0xaaaa_aaaa,
        flags: 0xaaaa_aaaa,
    };
    let r = mmioDescend(h, &mut c, &parent, 0x10);
    note(format!("call\tmmioDescend fmt\t{r:x}\t{}", ck(&c)));
    if r == 0x109 || c.cksize < 0x10 {
        note(format!("call\tmmioClose\t{:x}", mmioClose(h, 0)));
        return;
    }
    let mut format = [0u8; 16];
    let r = mmioRead(h, format.as_mut_ptr(), 0x10);
    note(format!("call\tmmioRead 16\t{r}\t{}", hex(&format)));
    if u16::from_le_bytes([format[0], format[1]]) != 1 {
        note(format!("call\tmmioClose\t{:x}", mmioClose(h, 0)));
        return;
    }
    let r = mmioAscend(h, &mut c, 0);
    note(format!("call\tmmioAscend fmt\t{r:x}\t{}", ck(&c)));
    c.ckid = fourcc(b"data");
    let r = mmioDescend(h, &mut c, &parent, 0x10);
    note(format!("call\tmmioDescend data\t{r:x}\t{}", ck(&c)));
    let mut i: MmioInfo = std::mem::zeroed();
    let r = mmioGetInfo(h, &mut i, 0);
    note(format!("call\tmmioGetInfo\t{r:x}\t{}", info_fields(&i)));
    note(format!(
        "call\tmmioGetInfo rest\t{r:x}\tfcc {:x} proc_is_ours {} task {:x} r1 {:x} r2 {:x} hmmio_is_handle {} info1 {:x} info2 {:x}",
        i.fcc_io_proc,
        i.io_proc.map(|p| p as usize) == Some(io_proc as IoProc as usize),
        i.task,
        i.reserved1,
        i.reserved2,
        i.hmmio == h,
        i.info[1],
        i.info[2]
    ));
    let first = std::slice::from_raw_parts(i.pch_buffer, i.cch_buffer as usize);
    let tail_zero = first[(i.pch_end_read as usize - i.pch_buffer as usize)..]
        .iter()
        .all(|&b| b == 0);
    note(format!(
        "call\tbuffer past endread zero\t{}",
        u32::from(tail_zero)
    ));
    let mut got = Vec::new();
    let mut failed = None;
    for _ in 0..c.cksize {
        if i.pch_next == i.pch_end_read {
            let r = mmioAdvance(h, &mut i, 0);
            note(format!("call\tmmioAdvance\t{r:x}\t{}", info_fields(&i)));
            if r != 0 {
                failed = Some(r);
                break;
            }
        }
        got.push(*i.pch_next);
        i.pch_next = i.pch_next.add(1);
    }
    let digest = got
        .iter()
        .fold(0u32, |a, &b| a.wrapping_mul(31).wrapping_add(b as u32));
    note(format!(
        "call\tdata read\t{}\tbytes {} sum {digest:08x}",
        failed.unwrap_or(0),
        got.len()
    ));
    if failed.is_some() {
        note(format!("call\tmmioClose\t{:x}", mmioClose(h, 0)));
        return;
    }
    let r = mmioAscend(h, &mut c, 0);
    note(format!("call\tmmioAscend data\t{r:x}\t{}", ck(&c)));
    c.ckid = fourcc(b"smpl");
    let r = mmioDescend(h, &mut c, &parent, 0x10);
    note(format!("call\tmmioDescend smpl\t{r:x}\t{}", ck(&c)));
    if r != 0x109 {
        let mut s = [0u8; 0x3c];
        let r = mmioRead(h, s.as_mut_ptr(), 0x3c);
        note(format!("call\tmmioRead 60\t{r}\t{}", hex(&s[..8])));
    }
    let r = mmioSeek(h, 0, 1);
    note(format!("call\tmmioSeek cur (probe)\t{r}"));
    note(format!("call\tmmioClose\t{:x}", mmioClose(h, 0)));
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() {
    let mut out = std::io::stdout().lock();
    for (name, image) in cases() {
        let size = image.len();
        unsafe {
            SERVED = Some(Served {
                image,
                position: 0,
                open: false,
                fail_open: name == "open_fail",
                log: Vec::new(),
            });
        }

        unsafe { game_sequence() };
        writeln!(out, "case\t{name}\t{size}").unwrap();
        let s = served();
        for l in &s.log {
            writeln!(out, "{l}").unwrap();
        }
        writeln!(out, "end").unwrap();
    }
}
