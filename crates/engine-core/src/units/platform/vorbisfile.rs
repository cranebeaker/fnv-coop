//! The platform's `libvorbisfile.dll`: the six functions the game imports (`ov_open_callbacks`,
//! `ov_info`, `ov_seekable`, `ov_pcm_total`, `ov_read`, `ov_clear`), answered by our own Vorbis
//! decoder (`crates/vorbis`), which gives the same samples as the player's library (libVorbis
//! 1.2.0): on all 63,575 `.ogg` files in the game's archives, every byte `ov_read` returns, and
//! `ov_pcm_total`, agree with the library's run at 24-bit and at 53-bit x87 precision
//! (`scripts/vorbis-record`, 2026-10-24). The library computes on the x87 unit, so its output
//! depends on the decoding thread's precision control (62,819 of the files differ between the
//! two); the decoder is run at the calling thread's. Which game thread decodes
//! (`AudioLoadTask::Run`, `00ae7050`, queued to the IO manager `01202d98`) and its control
//! word are not measured yet.
//!
//! The game opens a sound (`00af1e00`) with `ov_open_callbacks(file, this, NULL, 0, {read
//! 00af2180, seek 00af2200, close NULL, tell 00af2290})` on its own `OggVorbis_File` (0x2d0
//! bytes inside the reader, which this layer leaves alone: the state is its own, by address),
//! keeps `ov_info(-1)` and `ov_pcm_total(0)`, and decodes the whole file (`00af2100`) with
//! `ov_read(.., 0, 2, 1, ..)`.
//!
//! Where this layer stops with "not provided":
//! - Decoding at 64-bit x87 precision (not measured).
//! - Initial bytes, a stream that cannot seek, other `ov_read` formats.
//!
//! The file is read whole through the game's callbacks when it is opened (the seek test, then
//! to the end and back to measure it, then reads of 64 KiB); the library's own pattern of
//! reads and seeks, which only moves the game file's position, is not reproduced.

use crate::{Engine, Ptr};
use std::collections::BTreeMap;
use vorbis::{Precision, VorbisFile};

/// libvorbisfile's `CHUNKSIZE`.
const CHUNK: u32 = 65536;
const SEEK_SET: u32 = 0;
const SEEK_CUR: u32 = 1;
const SEEK_END: u32 = 2;
const OV_EINVAL: i32 = -131;
/// `sizeof(vorbis_info)`.
const INFO_SIZE: u32 = 0x20;

/// The open files, by the game's `OggVorbis_File` address: the decoder and the engine-memory
/// `vorbis_info` `ov_info` gives.
#[derive(Default)]
pub struct Files {
    open: BTreeMap<u32, (VorbisFile, u32)>,
    /// What [`ov_read`] decodes into before it goes to engine memory, kept from one read to the
    /// next (the game reads a stream a few KB at a time, every frame).
    read: Vec<u8>,
}

impl std::fmt::Debug for Files {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Files {{ open: {:?} }}",
            self.open.keys().collect::<Vec<_>>()
        )
    }
}

/// The decoding thread's precision; 64 bits is not provided (module documentation).
fn precision(e: &mut Engine, call: &str) -> Precision {
    let word = crate::units::crt::x87_control_word(e);
    match (word >> 8) & 3 {
        0 => Precision::Bits24,
        2 => Precision::Bits53,
        pc => panic!(
            "{call} at x87 precision control {pc} (control word {word:#06x}) is not provided: the \
             player's libvorbis computes on the x87 unit and only its 24- and 53-bit results are matched"
        ),
    }
}

/// `ov_open_callbacks(datasource, vf, initial, ibytes, callbacks)`: 0, or libvorbisfile's error.
#[allow(clippy::too_many_arguments)] // The two words and the callbacks passed by value.
pub fn ov_open_callbacks(
    e: &mut Engine,
    source: u32,
    vf: Ptr,
    initial: u32,
    ibytes: i32,
    read: u32,
    seek: u32,
    _close: u32,
    tell: u32,
) -> i32 {
    assert!(
        initial == 0 && ibytes == 0,
        "ov_open_callbacks with initial bytes is not provided"
    );
    let p = precision(e, "ov_open_callbacks");
    let seekable = seek != 0 && e.call(seek, &[source, 0, 0, SEEK_CUR]).u32() as i32 != -1;
    assert!(
        seekable && tell != 0,
        "ov_open_callbacks on a stream that cannot seek is not provided"
    );
    e.call(seek, &[source, 0, 0, SEEK_END]);
    let size = e.call(tell, &[source]).u32();
    e.call(seek, &[source, 0, 0, SEEK_SET]);
    let buffer = e.mem.alloc(CHUNK);
    let mut data = Vec::with_capacity(size as usize);
    loop {
        let n = e.call(read, &[buffer, 1, CHUNK, source]).u32();
        if n == 0 || n > CHUNK {
            break;
        }
        data.extend_from_slice(&e.mem.bytes(buffer, n));
    }
    e.mem.free(buffer);
    match VorbisFile::open(data, p) {
        Ok(file) => {
            let i = file.info();
            let info = e.mem.alloc(INFO_SIZE);
            let words = [
                i.version,
                i.channels,
                i.rate,
                i.bitrate_upper,
                i.bitrate_nominal,
                i.bitrate_lower,
                0,
                0,
            ];
            for (k, w) in words.into_iter().enumerate() {
                e.mem.set_i32(info + 4 * k as u32, w);
            }
            let sound = &mut e.kernel.sound.vorbis;
            assert!(
                !sound.open.contains_key(&vf.addr()),
                "ov_open_callbacks on an open OggVorbis_File is not provided"
            );
            sound.open.insert(vf.addr(), (file, info));
            let what = format!(
                "ogg {:08x}: {size} bytes, {} Hz, {} ch",
                vf.addr(),
                words[2],
                words[1]
            );
            super::dsound::note(e, &what);
            0
        }
        Err(code) => code,
    }
}

fn file<'a>(e: &'a mut Engine, vf: Ptr, call: &str) -> &'a mut (VorbisFile, u32) {
    e.kernel
        .sound
        .vorbis
        .open
        .get_mut(&vf.addr())
        .unwrap_or_else(|| panic!("{call} on an OggVorbis_File that is not open is not provided"))
}

/// `ov_info(vf, link)`: the stream's `vorbis_info` for link -1 or 0.
pub fn ov_info(e: &mut Engine, vf: Ptr, link: i32) -> u32 {
    assert!(link <= 0, "ov_info of link {link} is not provided");
    file(e, vf, "ov_info").1
}

/// `ov_seekable(vf)`: 1 (the game's files seek).
pub fn ov_seekable(e: &mut Engine, vf: Ptr) -> i32 {
    i32::from(file(e, vf, "ov_seekable").0.seekable())
}

/// `ov_pcm_total(vf, link)`: the samples of link 0 (or all, -1), `OV_EINVAL` for another link.
pub fn ov_pcm_total(e: &mut Engine, vf: Ptr, link: i32) -> u64 {
    file(e, vf, "ov_pcm_total").0.pcm_total(link) as u64
}

/// `ov_read(vf, buffer, length, bigendianp, word, sgned, bitstream)`: little-endian signed
/// 16-bit only (what the game asks).
#[allow(clippy::too_many_arguments)] // The call's seven words.
pub fn ov_read(
    e: &mut Engine,
    vf: Ptr,
    buffer: u32,
    length: i32,
    bigendianp: i32,
    word: i32,
    sgned: i32,
    bitstream: Ptr,
) -> i32 {
    assert!(
        bigendianp == 0 && word == 2 && sgned == 1,
        "ov_read({bigendianp}, {word}, {sgned}) is not provided"
    );
    precision(e, "ov_read");
    if length < 0 {
        return OV_EINVAL;
    }
    // The decoder writes the samples it gives and reads nothing of the buffer: the last read's
    // room serves, its length the one asked.
    let mut out = std::mem::take(&mut e.kernel.sound.vorbis.read);
    out.resize(length as usize, 0);
    let n = file(e, vf, "ov_read").0.read(&mut out);
    if n > 0 {
        e.mem.write(buffer, &out[..n as usize]);
        if !bitstream.is_null() {
            e.mem.set_u32(bitstream.addr(), 0);
        }
    }
    e.kernel.sound.vorbis.read = out;
    n as i32
}

/// `ov_clear(vf)`: closes the stream (the game passes no close callback); 0.
pub fn ov_clear(e: &mut Engine, vf: Ptr) -> i32 {
    if let Some((_, info)) = e.kernel.sound.vorbis.open.remove(&vf.addr()) {
        e.mem.free(info);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        /// The file the callbacks read, its position, and the calls made.
        static FILE: RefCell<(Vec<u8>, usize, Vec<String>)> = const { RefCell::new((Vec::new(), 0, Vec::new())) };
    }

    fn read(e: &mut Engine, buffer: u32, size: u32, count: u32, _source: u32) -> u32 {
        let chunk = FILE.with(|f| {
            let mut f = f.borrow_mut();
            let n = ((size * count) as usize).min(f.0.len() - f.1);
            let chunk = f.0[f.1..f.1 + n].to_vec();
            f.1 += n;
            f.2.push(format!("read {}", size * count));
            chunk
        });
        e.mem.write(buffer, &chunk);
        chunk.len() as u32
    }

    fn seek(_e: &mut Engine, _source: u32, offset: u32, _high: u32, whence: u32) -> u32 {
        FILE.with(|f| {
            let mut f = f.borrow_mut();
            f.1 = match whence {
                0 => offset as usize,
                1 => f.1 + offset as usize,
                _ => f.0.len(),
            };
            f.2.push(format!("seek {whence}"));
        });
        0
    }

    fn tell(_e: &mut Engine, _source: u32) -> u32 {
        FILE.with(|f| {
            let mut f = f.borrow_mut();
            f.2.push("tell".into());
            f.1 as u32
        })
    }

    fn open(e: &mut Engine, data: &[u8]) -> i32 {
        FILE.with(|f| *f.borrow_mut() = (data.to_vec(), 0, Vec::new()));
        e.register(
            0x7f00_0000,
            crate::entry!(0, read(u32, u32, u32, u32) -> u32).1,
        );
        e.register(
            0x7f00_0010,
            crate::entry!(0, seek(u32, u32, u32, u32) -> u32).1,
        );
        e.register(0x7f00_0020, crate::entry!(0, tell(u32) -> u32).1);
        ov_open_callbacks(
            e,
            0x1234,
            Ptr::new(0x5000),
            0,
            0,
            0x7f00_0000,
            0x7f00_0010,
            0,
            0x7f00_0020,
        )
    }

    #[test]
    fn a_file_that_is_not_ogg_vorbis_is_refused_after_reading_it_through_the_callbacks() {
        let mut e = Engine::new();
        crate::units::crt::set_x87_control_word(&mut e, 0x007f);
        let r = open(&mut e, &[0x55; 70000]);
        assert_eq!(r, -132);
        let calls = FILE.with(|f| f.borrow().2.clone());
        assert_eq!(
            calls,
            [
                "seek 1",
                "seek 2",
                "tell",
                "seek 0",
                "read 65536",
                "read 65536",
                "read 65536"
            ]
        );
        assert!(e.kernel.sound.vorbis.open.is_empty());
    }

    #[test]
    #[should_panic(expected = "is not provided: the player's libvorbis computes on the x87 unit")]
    fn decoding_at_64_bits_stops() {
        let mut e = Engine::new();
        crate::units::crt::set_x87_control_word(&mut e, 0x037f);
        open(&mut e, b"OggS");
    }
}
