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
//!
//! **Decoding ahead** (the running game): the game decodes a whole sound at
//! once on its IO thread (`AudioLoadTask::Run` -> `00af2100`), which here is stepped on the
//! main core, so a long voice file's decode landed inside one frame (the two longest frames of
//! the Goodsprings stutter route, 114 and 120 ms, were almost all `ov_read`). With the real clock
//! on, [`ov_open_callbacks`] hands the opened decoder to a host
//! worker thread ([`Ahead`]) that decodes the stream ahead, one `ov_read`'s worth of samples at a
//! time; [`ov_read`] answers from what it decoded, cut to the length asked, exactly as the
//! decoder would have answered that call (same bytes, same return values, errors and the end in
//! the same place: [`Ahead::read`]). When nothing is decoded yet, a game thread gives up its turn
//! (`Sleep(1)` through the thread host) until enough is, so the main thread goes on with its
//! frames meanwhile; on the main thread the call waits. The game sees only time pass in the IO
//! thread's task, as on the original, whose IO thread decodes on its own core while the main
//! thread runs. Without the real clock (the tests, replays on the platform clock) the decoder
//! runs inside the call, as before.

use crate::{Engine, Ptr};
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use vorbis::{Precision, VorbisFile};

/// libvorbisfile's `CHUNKSIZE`.
const CHUNK: u32 = 65536;
const SEEK_SET: u32 = 0;
const SEEK_CUR: u32 = 1;
const SEEK_END: u32 = 2;
const OV_EINVAL: i32 = -131;
/// `sizeof(vorbis_info)`.
const INFO_SIZE: u32 = 0x20;

/// What answers `ov_read`: libvorbisfile's call on 16-bit signed little-endian samples (bytes
/// written, 0 at the end, or an error), as [`VorbisFile::read`] gives it.
pub(crate) trait Pcm: Send {
    fn read(&mut self, buffer: &mut [u8]) -> i64;
}

impl Pcm for VorbisFile {
    fn read(&mut self, buffer: &mut [u8]) -> i64 {
        VorbisFile::read(self, buffer)
    }
}

/// How an open file's samples are decoded.
enum Decoder {
    /// Inside each `ov_read`.
    Inline(Box<dyn Pcm>),
    /// Ahead, on a host worker thread (module documentation).
    Ahead(Ahead),
}

/// One open file: its decoder, the engine-memory `vorbis_info` `ov_info` gives, what
/// `ov_seekable` answers, and what `ov_pcm_total` answers for link 0 (or -1) and for a later
/// link, kept at the open.
struct Open {
    decoder: Decoder,
    info: u32,
    seekable: bool,
    total: [i64; 2],
}

/// The open files, by the game's `OggVorbis_File` address.
#[derive(Default)]
pub struct Files {
    open: BTreeMap<u32, Open>,
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

/// Whether a file opened now decodes ahead: the real clock running (the
/// running game; the tests and platform-clock replays stay inline and deterministic).
fn decode_ahead(e: &Engine) -> bool {
    e.real_clock.is_some()
}

/// How far the worker decodes ahead of the reads (bytes not yet taken): a bound on the memory a
/// stream read slowly holds, far above a whole voice file.
const AHEAD_LIMIT: usize = 16 << 20;
/// A game thread that found nothing decoded goes on once this much is (or the stream ended), so
/// a whole-file decode takes a few turns, not one per block.
const AHEAD_BATCH: usize = 256 << 10;
/// The worker's read: more samples than one call of the decoder can give (a Vorbis block is at
/// most 8192 samples), so each of its reads takes everything the decoder has ready.
const AHEAD_SAMPLES: usize = 1 << 16;

/// What the worker gives, in order.
enum Event {
    /// The bytes one call gave with all the room it wanted: the samples the decoder had ready.
    Chunk(Vec<u8>),
    /// The call's 0 (the end) or error, and the decoder, back to the reads from here on.
    End(i32, Box<dyn Pcm>),
}

#[derive(Default)]
struct Queue {
    events: VecDeque<Event>,
    /// Bytes in `events` not taken yet.
    queued: usize,
    /// An `End` was queued.
    ended: bool,
    /// The reads are gone (`ov_clear`): the worker stops.
    cancelled: bool,
    /// The worker stopped without an `End` (it panicked).
    lost: bool,
}

type Shared = Arc<(Mutex<Queue>, Condvar)>;

fn lock(s: &Shared) -> MutexGuard<'_, Queue> {
    s.0.lock().unwrap_or_else(|p| p.into_inner())
}

/// Marks the queue lost when the worker ends without its `End` (a panic), so a read stops
/// with a message instead of waiting for ever.
struct WorkerGone(Shared);

impl Drop for WorkerGone {
    fn drop(&mut self) {
        let mut q = lock(&self.0);
        if !q.ended {
            q.lost = true;
        }
        drop(q);
        self.0 .1.notify_all();
    }
}

/// A decoder run ahead on a host worker thread (module documentation).
pub(crate) struct Ahead {
    shared: Shared,
    /// The bytes of the chunk being read, and how far the reads have taken them.
    chunk: Vec<u8>,
    at: usize,
    /// Bytes per sample frame (2 x channels).
    frame: usize,
}

/// What [`Ahead::read`] answers.
enum Read {
    /// `ov_read`'s answer: bytes put in the buffer (above 0), or `OV_EINVAL`.
    Done(i32),
    /// The decoder's end or error: the answer, and the decoder for the reads from here on.
    End(i32, Box<dyn Pcm>),
    /// Nothing decoded yet.
    Pending,
}

impl Ahead {
    /// Starts the worker on `pcm` (`frame` bytes per sample frame). With `hold`, the worker
    /// starts decoding only once it receives (the tests' way to catch a read waiting).
    fn start(
        pcm: Box<dyn Pcm>,
        frame: usize,
        hold: Option<std::sync::mpsc::Receiver<()>>,
    ) -> Ahead {
        let shared: Shared = Arc::default();
        let worker = Arc::clone(&shared);
        std::thread::Builder::new()
            .name("nv-vorbis-ahead".into())
            .spawn(move || {
                if let Some(hold) = hold {
                    let _ = hold.recv();
                }
                decode(pcm, frame, worker)
            })
            .expect("a host thread for the sound decoder");
        Ahead {
            shared,
            chunk: Vec::new(),
            at: 0,
            frame,
        }
    }

    /// One `ov_read` of `length` bytes (not negative) into `out`, as the decoder answers it: the
    /// samples it has ready, up to the length; when it has none, the next block it decodes (the
    /// next chunk); `OV_EINVAL` when the length holds no sample frame (after that fetch, as the
    /// decoder does); its end or error where it gave them. The decoder's own call takes the same
    /// steps: it fetches only when no samples are ready, and a fetch makes ready exactly what
    /// the worker's read of all of them took, so the answers do not depend on how the reads cut
    /// the stream.
    fn read(&mut self, out: &mut Vec<u8>, length: usize) -> Read {
        loop {
            let left = self.chunk.len() - self.at;
            if left > 0 {
                let n = left.min(length / self.frame * self.frame);
                if n == 0 {
                    return Read::Done(OV_EINVAL);
                }
                out.clear();
                out.extend_from_slice(&self.chunk[self.at..self.at + n]);
                self.at += n;
                return Read::Done(n as i32);
            }
            let mut q = lock(&self.shared);
            let Some(event) = q.events.pop_front() else {
                assert!(
                    !q.lost,
                    "the sound decoder's worker thread stopped (see the panic above)"
                );
                return Read::Pending;
            };
            match event {
                Event::Chunk(bytes) => {
                    q.queued -= bytes.len();
                    drop(q);
                    self.shared.1.notify_all();
                    self.chunk = bytes;
                    self.at = 0;
                }
                Event::End(code, pcm) => return Read::End(code, pcm),
            }
        }
    }

    /// On the main thread: waits until something is queued (or the worker is lost).
    fn wait(&self) {
        let mut q = lock(&self.shared);
        while q.events.is_empty() && !q.lost {
            q = self.shared.1.wait(q).unwrap_or_else(|p| p.into_inner());
        }
    }
}

/// Whether a game thread's read that found nothing decoded goes on: a batch is queued, or the
/// stream's end, or the worker is lost.
fn batch_ready(shared: &Shared) -> bool {
    let q = lock(shared);
    q.lost || q.ended || q.queued >= AHEAD_BATCH
}

impl Drop for Ahead {
    fn drop(&mut self) {
        lock(&self.shared).cancelled = true;
        self.shared.1.notify_all();
    }
}

/// The worker: reads with room for everything ready, queueing each answer, until the end, an
/// error, or the reads are gone; at most [`AHEAD_LIMIT`] bytes ahead of the reads.
fn decode(mut pcm: Box<dyn Pcm>, frame: usize, shared: Shared) {
    let _gone = WorkerGone(Arc::clone(&shared));
    let mut buffer = vec![0u8; AHEAD_SAMPLES * frame];
    loop {
        {
            let mut q = lock(&shared);
            while q.queued > AHEAD_LIMIT && !q.cancelled {
                q = shared.1.wait(q).unwrap_or_else(|p| p.into_inner());
            }
            if q.cancelled {
                return;
            }
        }
        let n = pcm.read(&mut buffer);
        assert!(
            n < buffer.len() as i64,
            "the sound decoder gave {n} bytes at once: more than a block can hold"
        );
        let mut q = lock(&shared);
        if n > 0 {
            q.queued += n as usize;
            q.events
                .push_back(Event::Chunk(buffer[..n as usize].to_vec()));
        } else {
            q.ended = true;
            q.events.push_back(Event::End(n as i32, pcm));
            drop(q);
            shared.1.notify_all();
            return;
        }
        drop(q);
        shared.1.notify_all();
    }
}

/// The decoding thread's precision; 64 bits is not provided (module documentation).
fn precision(e: &mut Engine, call: &str) -> Precision {
    let word = engine_core::units::crt::x87_control_word(e);
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
            let seekable = file.seekable();
            let total = [file.pcm_total(0), file.pcm_total(1)];
            let frame = 2 * words[1].max(1) as usize;
            let decoder = if decode_ahead(e) {
                Decoder::Ahead(Ahead::start(Box::new(file), frame, None))
            } else {
                Decoder::Inline(Box::new(file))
            };
            let sound = &mut state!(e).sound.vorbis;
            assert!(
                !sound.open.contains_key(&vf.addr()),
                "ov_open_callbacks on an open OggVorbis_File is not provided"
            );
            sound.open.insert(
                vf.addr(),
                Open {
                    decoder,
                    info,
                    seekable,
                    total,
                },
            );
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

fn file<'a>(e: &'a mut Engine, vf: Ptr, call: &str) -> &'a mut Open {
    state!(e)
        .sound
        .vorbis
        .open
        .get_mut(&vf.addr())
        .unwrap_or_else(|| panic!("{call} on an OggVorbis_File that is not open is not provided"))
}

/// `ov_info(vf, link)`: the stream's `vorbis_info` for link -1 or 0.
pub fn ov_info(e: &mut Engine, vf: Ptr, link: i32) -> u32 {
    assert!(link <= 0, "ov_info of link {link} is not provided");
    file(e, vf, "ov_info").info
}

/// `ov_seekable(vf)`: 1 (the game's files seek).
pub fn ov_seekable(e: &mut Engine, vf: Ptr) -> i32 {
    i32::from(file(e, vf, "ov_seekable").seekable)
}

/// `ov_pcm_total(vf, link)`: the samples of link 0 (or all, -1), `OV_EINVAL` for another link.
pub fn ov_pcm_total(e: &mut Engine, vf: Ptr, link: i32) -> u64 {
    let total = file(e, vf, "ov_pcm_total").total;
    total[usize::from(link >= 1)] as u64
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
    let mut out = std::mem::take(&mut state!(e).sound.vorbis.read);
    let on_main = crate::thread_host::on_main(e);
    let n = loop {
        let open = file(e, vf, "ov_read");
        let ahead = match &mut open.decoder {
            Decoder::Inline(pcm) => {
                out.resize(length as usize, 0);
                break pcm.read(&mut out) as i32;
            }
            Decoder::Ahead(ahead) => ahead,
        };
        match ahead.read(&mut out, length as usize) {
            Read::Done(n) => break n,
            Read::End(n, pcm) => {
                // The end or an error: the decoder answers the reads from here on.
                open.decoder = Decoder::Inline(pcm);
                break n;
            }
            Read::Pending => {
                // Nothing decoded yet: the main thread waits; a game thread gives up its turn
                // (`Sleep(1)`) until a batch is decoded, so the main thread goes on meanwhile.
                if on_main {
                    ahead.wait();
                    continue;
                }
                let shared = Arc::clone(&ahead.shared);
                while !batch_ready(&shared) {
                    crate::thread_host::host_for(e).sleep(e, 1, crate::signalled);
                }
            }
        }
    };
    if n > 0 {
        e.mem.write(buffer, &out[..n as usize]);
        if !bitstream.is_null() {
            e.mem.set_u32(bitstream.addr(), 0);
        }
    }
    state!(e).sound.vorbis.read = out;
    n
}

/// `ov_clear(vf)`: closes the stream (the game passes no close callback); 0. A worker still
/// decoding ahead stops.
pub fn ov_clear(e: &mut Engine, vf: Ptr) -> i32 {
    if let Some(open) = state!(e).sound.vorbis.open.remove(&vf.addr()) {
        e.mem.free(open.info);
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
        engine_core::units::crt::set_x87_control_word(&mut e, 0x007f);
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
        assert!(state!(e).sound.vorbis.open.is_empty());
    }

    #[test]
    #[should_panic(expected = "is not provided: the player's libvorbis computes on the x87 unit")]
    fn decoding_at_64_bits_stops() {
        let mut e = Engine::new();
        engine_core::units::crt::set_x87_control_word(&mut e, 0x037f);
        open(&mut e, b"OggS");
    }

    /// A decoder answering `ov_read` as libvorbisfile does over a script of fetches: a fetch is
    /// made only when no samples are ready; it makes `n` sample frames ready (0: none, the
    /// call fetches again) or answers an error; past the script, the end. Each sample is the
    /// next number, so the bytes show any sample lost, repeated or moved.
    struct Script {
        steps: Vec<Step>,
        next: usize,
        ready: usize,
        made: u16,
        channels: usize,
        /// Counts the calls (the cancel tests).
        calls: Arc<std::sync::atomic::AtomicUsize>,
    }

    #[derive(Clone, Copy)]
    enum Step {
        Samples(usize),
        Error(i32),
        Panic,
    }

    impl Script {
        fn new(channels: usize, steps: &[Step]) -> Script {
            Script {
                steps: steps.to_vec(),
                next: 0,
                ready: 0,
                made: 0,
                channels,
                calls: Arc::default(),
            }
        }
    }

    impl Pcm for Script {
        fn read(&mut self, buffer: &mut [u8]) -> i64 {
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let frame = 2 * self.channels;
            while self.ready == 0 {
                let Some(&step) = self.steps.get(self.next) else {
                    return 0;
                };
                self.next += 1;
                match step {
                    Step::Samples(n) => self.ready = n,
                    Step::Error(code) => return i64::from(code),
                    Step::Panic => panic!("the script's decoder fails"),
                }
            }
            let samples = self.ready.min(buffer.len() / frame);
            if samples == 0 {
                return i64::from(OV_EINVAL);
            }
            for b in buffer[..samples * frame].chunks_mut(2) {
                b.copy_from_slice(&self.made.to_le_bytes());
                self.made = self.made.wrapping_add(1);
            }
            self.ready -= samples;
            (samples * frame) as i64
        }
    }

    const VF: u32 = 0x5000;

    fn insert(e: &mut Engine, decoder: Decoder) {
        engine_core::units::crt::set_x87_control_word(e, 0x027f);
        state!(e).sound.vorbis.open.insert(
            VF,
            Open {
                decoder,
                info: 0,
                seekable: true,
                total: [0, 0],
            },
        );
    }

    /// `ov_read` answers (and the bytes of each) for reads of these lengths.
    fn reads(e: &mut Engine, lengths: &[i32]) -> Vec<(i32, Vec<u8>)> {
        let buffer = e.mem.alloc(1 << 16);
        let out = lengths
            .iter()
            .map(|&length| {
                let n = ov_read(e, Ptr::new(VF), buffer, length, 0, 2, 1, Ptr::new(0));
                let bytes = if n > 0 {
                    e.mem.bytes(buffer, n as u32).to_vec()
                } else {
                    Vec::new()
                };
                (n, bytes)
            })
            .collect();
        e.mem.free(buffer);
        out
    }

    fn inline_and_ahead(channels: usize, steps: &[Step], lengths: &[i32]) {
        let mut e = Engine::new();
        insert(
            &mut e,
            Decoder::Inline(Box::new(Script::new(channels, steps))),
        );
        let inline = reads(&mut e, lengths);
        let mut e = Engine::new();
        let ahead = Ahead::start(Box::new(Script::new(channels, steps)), 2 * channels, None);
        insert(&mut e, Decoder::Ahead(ahead));
        let ahead = reads(&mut e, lengths);
        assert_eq!(inline, ahead, "{channels} channels, lengths {lengths:?}");
        assert!(inline.iter().any(|(n, _)| *n > 0));
    }

    #[test]
    fn decoded_ahead_the_reads_answer_as_the_decoder_does() {
        use Step::*;
        let steps = [
            Samples(0),
            Samples(1024),
            Samples(300),
            Samples(0),
            Samples(0),
            Samples(2048),
            Error(-3),
            Samples(512),
            Samples(1),
            Error(-3),
            Error(-131),
            Samples(4096),
        ];
        let game: Vec<i32> = vec![4096; 40];
        let odd: Vec<i32> = [1, 0, 2, 3, 4, 5, 7, 4097, 100, 6000, 2, 1, 9999]
            .iter()
            .copied()
            .cycle()
            .take(200)
            .collect();
        for channels in [1, 2] {
            inline_and_ahead(channels, &steps, &game);
            inline_and_ahead(channels, &steps, &odd);
            // Past the end the decoder answers again (the reads went back to it).
            inline_and_ahead(channels, &[Samples(10)], &[4096, 4096, 4096, 3, 4096]);
        }
        // Nothing at all: the end at once.
        let mut e = Engine::new();
        let ahead = Ahead::start(Box::new(Script::new(1, &[])), 2, None);
        insert(&mut e, Decoder::Ahead(ahead));
        assert_eq!(reads(&mut e, &[4096, 4096]), [(0, vec![]), (0, vec![])]);
    }

    /// The game thread of the next test: reads 4096 bytes at a time to the end, into `result`:
    /// the last answer, the bytes read, then the bytes themselves.
    fn reader(e: &mut Engine, result: u32) -> u32 {
        let buffer = e.mem.alloc(4096);
        let mut at = 0;
        loop {
            let n = ov_read(e, Ptr::new(VF), buffer, 4096, 0, 2, 1, Ptr::new(0));
            if n <= 0 {
                e.mem.set_i32(result, n);
                break;
            }
            let bytes = e.mem.bytes(buffer, n as u32).to_vec();
            e.mem.write(result + 8 + at, &bytes);
            at += n as u32;
        }
        e.mem.set_u32(result + 4, at);
        0
    }

    #[test]
    fn a_game_thread_gives_up_its_turn_until_the_decode_is_done_then_goes_on() {
        use engine_core::thread_host::{self, RunState};
        let steps: Vec<Step> = (0..20).map(|_| Step::Samples(2048)).collect();
        let mut e = Engine::new();
        insert(&mut e, Decoder::Inline(Box::new(Script::new(2, &steps))));
        let want = reads(&mut e, &[4096; 45]);
        let total: usize = want.iter().map(|(_, b)| b.len()).sum();

        let mut e = Engine::new();
        let (go, hold) = std::sync::mpsc::channel();
        let ahead = Ahead::start(Box::new(Script::new(2, &steps)), 4, Some(hold));
        let shared = Arc::clone(&ahead.shared);
        insert(&mut e, Decoder::Ahead(ahead));
        e.register(0x7f00_0100, crate::entry!(0, reader(u32) -> u32).1);
        let result = e.mem.alloc(8 + total as u32);
        e.mem.set_i32(result, 7);
        thread_host::create(&mut e, 0x77, 0x7f00_0100, result, false);

        // Nothing decoded: the thread gives up its turn (Sleep(1)), and the main thread's pass
        // ends without it.
        thread_host::run_threads(&mut e, crate::signalled);
        let sleeping =
            |e: &Engine| matches!(e.kernel.threads.list[1].state, RunState::Sleeping { .. });
        assert!(sleeping(&e));
        assert_eq!(e.mem.u32(result + 4), 0);
        // Still nothing: its next turn gives it up again.
        let now = e.clock_now_us();
        e.wait_clock_until_us(now + 1000);
        thread_host::run_threads(&mut e, crate::signalled);
        assert!(sleeping(&e));
        assert_eq!(e.mem.i32(result), 7);
        // The worker decodes the whole stream; at its next turn the thread reads it to the end.
        go.send(()).unwrap();
        while !lock(&shared).ended {
            std::thread::yield_now();
        }
        let now = e.clock_now_us();
        e.wait_clock_until_us(now + 1000);
        thread_host::run_threads(&mut e, crate::signalled);
        assert!(matches!(
            e.kernel.threads.list[1].state,
            RunState::Finished { code: 0 }
        ));
        assert_eq!(e.mem.i32(result), 0);
        assert_eq!(e.mem.u32(result + 4) as usize, total);
        let got = e.mem.bytes(result + 8, total as u32).to_vec();
        let want: Vec<u8> = want.into_iter().flat_map(|(_, b)| b).collect();
        assert_eq!(got, want);
    }

    /// Waits (ten seconds at most) until the worker behind `shared` has ended.
    fn worker_gone(shared: &Shared) -> bool {
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_secs(10) {
            if lock(shared).lost {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        false
    }

    #[test]
    fn a_cleared_file_stops_its_worker() {
        // Cleared before the worker started: it decodes nothing.
        let mut e = Engine::new();
        let script = Script::new(1, &[Step::Samples(100)]);
        let calls = Arc::clone(&script.calls);
        let (go, hold) = std::sync::mpsc::channel();
        let ahead = Ahead::start(Box::new(script), 2, Some(hold));
        let shared = Arc::clone(&ahead.shared);
        insert(&mut e, Decoder::Ahead(ahead));
        assert_eq!(ov_clear(&mut e, Ptr::new(VF)), 0);
        go.send(()).unwrap();
        assert!(worker_gone(&shared));
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(state!(e).sound.vorbis.open.is_empty());

        // Cleared partway through a long stream: the worker waits at its limit ahead, then
        // stops.
        let mut e = Engine::new();
        let steps: Vec<Step> = (0..400).map(|_| Step::Samples(60000)).collect();
        let script = Script::new(2, &steps);
        let calls = Arc::clone(&script.calls);
        let ahead = Ahead::start(Box::new(script), 4, None);
        let shared = Arc::clone(&ahead.shared);
        insert(&mut e, Decoder::Ahead(ahead));
        assert_eq!(reads(&mut e, &[4096])[0].0, 4096);
        while lock(&shared).queued <= AHEAD_LIMIT {
            std::thread::yield_now();
        }
        ov_clear(&mut e, Ptr::new(VF));
        assert!(worker_gone(&shared));
        assert!(calls.load(std::sync::atomic::Ordering::SeqCst) < 400);
    }

    #[test]
    #[should_panic(expected = "the sound decoder's worker thread stopped")]
    fn a_read_stops_when_the_worker_is_lost() {
        let mut e = Engine::new();
        let script = Script::new(1, &[Step::Samples(10), Step::Panic]);
        insert(
            &mut e,
            Decoder::Ahead(Ahead::start(Box::new(script), 2, None)),
        );
        reads(&mut e, &[4096, 4096, 4096]);
    }

    #[test]
    fn without_the_real_clock_files_decode_inline() {
        assert!(!decode_ahead(&Engine::new()));
    }

    /// The game's own files (`NV_VORBIS_TEST_DIR`: a folder of `.ogg` files taken from its
    /// archives): decoded ahead, every read answers as the decoder inside the call, at both
    /// precisions, for the game's 4096-byte reads and for odd lengths.
    #[test]
    #[ignore = "needs NV_VORBIS_TEST_DIR, a folder of the game's .ogg files"]
    fn the_games_files_decode_ahead_to_the_same_bytes() {
        let dir = std::env::var("NV_VORBIS_TEST_DIR").expect("NV_VORBIS_TEST_DIR");
        let mut files = 0;
        for entry in std::fs::read_dir(dir).expect("the folder") {
            let path = entry.expect("an entry").path();
            if path.extension().is_none_or(|x| x != "ogg") {
                continue;
            }
            let data = std::fs::read(&path).expect("the file");
            let open = |p| VorbisFile::open(data.clone(), p).expect("an Ogg Vorbis file");
            for p in [Precision::Bits24, Precision::Bits53] {
                let frame = 2 * open(p).info().channels as usize;
                for lengths in [vec![4096], vec![4097, 1, 3, 0, 9000, 2, 4]] {
                    let lengths: Vec<i32> = lengths.into_iter().cycle().take(6000).collect();
                    let mut e = Engine::new();
                    insert(&mut e, Decoder::Inline(Box::new(open(p))));
                    let inline = reads(&mut e, &lengths);
                    let mut e = Engine::new();
                    let ahead = Ahead::start(Box::new(open(p)), frame, None);
                    insert(&mut e, Decoder::Ahead(ahead));
                    let ahead = reads(&mut e, &lengths);
                    assert_eq!(inline, ahead, "{}", path.display());
                }
            }
            files += 1;
        }
        assert!(files > 0, "no .ogg file in NV_VORBIS_TEST_DIR");
    }
}
