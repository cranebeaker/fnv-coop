// Follows libvorbisfile 1.2.0 (lib/vorbisfile.c), Copyright (c) 2002-2004 Xiph.org Foundation,
// BSD licence: see ../LICENSE-libvorbis.
//! What the game calls: `ov_open_callbacks` on a seekable file, `ov_info`, `ov_seekable`,
//! `ov_pcm_total` and `ov_read` as 16-bit signed little-endian, following libvorbisfile's steps,
//! quirks included: the open reads the headers, measures the stream (its first granule position
//! less the samples before it, the last page's granule position) and then goes back to the
//! start the way `ov_raw_seek(0)` does, which drops every packet of the first page that holds a
//! granule position if that page also ends the stream (so a stream whose audio fits on one page
//! decodes to nothing, as the player's library does).
//!
//! The file is one logical stream (what every game file is); a chained or multiplexed file, a
//! hole in the pages and the other output formats are not provided. The bytes are read whole;
//! the library's own pattern of reads and seeks through the callbacks is not reproduced here.

use crate::decode::Dsp;
use crate::fpu::{Fpu, Precision};
use crate::ogg::{next_page, Out, Page, Stream};
use crate::setup::{
    is_id_header, Info, OV_EBADHEADER, OV_EFAULT, OV_EINVAL, OV_ENOTVORBIS, OV_HOLE,
};

const OV_EOF: i32 = -2;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Ready {
    Opened = 2,
    StreamSet = 3,
    InitSet = 4,
}

/// An open file (`OggVorbis_File`).
pub struct VorbisFile {
    data: Vec<u8>,
    offset: usize,
    os: Stream,
    ready: Ready,
    info: Info,
    serialno: i32,
    pcmlengths: [i64; 2],
    dsp: Option<Dsp>,
    fpu: Fpu,
}

impl VorbisFile {
    /// `ov_open_callbacks` with seekable callbacks over `data`, decoding at the thread's x87
    /// `precision`: the file, or libvorbisfile's error.
    pub fn open(data: Vec<u8>, precision: Precision) -> Result<VorbisFile, i32> {
        let mut vf = VorbisFile {
            data,
            offset: 0,
            os: Stream::new(-1),
            ready: Ready::Opened,
            info: Info::default(),
            serialno: 0,
            pcmlengths: [0, 0],
            dsp: None,
            fpu: Fpu::new(precision),
        };
        vf.fetch_headers()?;
        let dataoffset = vf.offset;
        vf.ready = Ready::Opened;
        vf.open_seekable(dataoffset)?;
        Ok(vf)
    }

    fn next_page(&mut self) -> Option<(usize, Page)> {
        let (at, page) = next_page(&self.data, self.offset)?;
        self.offset = at + page.len();
        Some((at, page))
    }

    /// `_fetch_headers` from the start of the file.
    fn fetch_headers(&mut self) -> Result<(), i32> {
        let Some((_, mut og)) = self.next_page() else {
            return Err(OV_ENOTVORBIS);
        };
        self.info = Info::default();
        loop {
            if !og.bos() {
                return Err(OV_ENOTVORBIS);
            }
            self.os.reset(og.serialno());
            self.os.pagein(&og);
            if let Out::Packet(p) = self.os.packetout() {
                if is_id_header(&p.data, p.bos) {
                    self.serialno = self.os.serialno();
                    self.ready = Ready::StreamSet;
                    let r = self.info.headerin(&p.data, p.bos);
                    if r != 0 {
                        return Err(r);
                    }
                    let mut i = 0;
                    let mut allbos = false;
                    while i < 2 {
                        while i < 2 {
                            match self.os.packetout() {
                                Out::None => break,
                                Out::Hole => return Err(OV_EBADHEADER),
                                Out::Packet(p) => {
                                    let r = self.info.headerin(&p.data, p.bos);
                                    if r != 0 {
                                        return Err(r);
                                    }
                                    i += 1;
                                }
                            }
                        }
                        if i < 2 {
                            loop {
                                let Some((_, page)) = self.next_page() else {
                                    return Err(OV_EBADHEADER);
                                };
                                if page.serialno() == self.os.serialno() {
                                    self.os.pagein(&page);
                                    break;
                                }
                                if page.bos() {
                                    if allbos {
                                        return Err(OV_EBADHEADER);
                                    }
                                    allbos = true;
                                }
                            }
                        }
                    }
                    return Ok(());
                }
            }
            let Some((_, page)) = self.next_page() else {
                return Err(OV_ENOTVORBIS);
            };
            og = page;
        }
    }

    /// `_open_seekable2`, then `ov_raw_seek(0)`.
    fn open_seekable(&mut self, dataoffset: usize) -> Result<(), i32> {
        // The whole file's pages (what `_get_prev_page` and the link bisection walk).
        let mut pages = Vec::new();
        self.offset = 0;
        while let Some((at, page)) = self.next_page() {
            pages.push((at, page));
        }
        if pages.is_empty() {
            return Err(OV_EINVAL);
        }
        if pages.iter().any(|(_, p)| p.serialno() != self.serialno) {
            panic!("an Ogg file with more than one logical stream is not provided");
        }
        // `_prefetch_all_headers`: the PCM offset of the first page with a granule position.
        let mut accumulated = 0i64;
        let mut lastblock = -1i64;
        let mut os = Stream::new(self.serialno);
        for (_, og) in pages.iter().filter(|(at, _)| *at >= dataoffset) {
            if og.bos() {
                break;
            }
            os.pagein(og);
            loop {
                match os.packetout() {
                    Out::None => break,
                    Out::Hole => {}
                    Out::Packet(p) => {
                        let thisblock = Dsp::packet_blocksize(&self.info, &p.data);
                        if lastblock != -1 {
                            accumulated += (lastblock + thisblock) >> 2;
                        }
                        lastblock = thisblock;
                    }
                }
            }
            if og.granulepos() != -1 {
                accumulated = og.granulepos() - accumulated;
                break;
            }
        }
        if accumulated < 0 {
            accumulated = 0;
        }
        self.pcmlengths[0] = accumulated;
        if let Some((_, og)) = pages.iter().rev().find(|(_, p)| p.granulepos() != -1) {
            self.pcmlengths[1] = og.granulepos() - accumulated;
        }
        self.raw_seek_start();
        Ok(())
    }

    /// `ov_raw_seek(vf, 0)`: back to the start; the header packets (and, on an end-of-stream
    /// page, the audio packets) up to the first granule position are dropped from the stream.
    fn raw_seek_start(&mut self) {
        self.os.reset(self.serialno);
        if let Some(d) = self.dsp.as_mut() {
            d.restart();
        }
        self.offset = 0;
        let mut work = Stream::new(self.serialno);
        let mut lastblock = 0i64;
        let mut eosflag = false;
        loop {
            if self.ready >= Ready::StreamSet {
                if let Out::Packet(p) = work.packetout() {
                    let thisblock = Dsp::packet_blocksize(&self.info, &p.data);
                    if thisblock < 0 || eosflag {
                        self.os.packetout();
                    }
                    let thisblock = thisblock.max(0);
                    if p.granulepos != -1 {
                        break;
                    }
                    lastblock = thisblock;
                    continue;
                }
            }
            if lastblock != 0 {
                break;
            }
            let Some((_, og)) = self.next_page() else {
                break;
            };
            if self.ready < Ready::StreamSet {
                if og.serialno() != self.serialno {
                    continue;
                }
                self.os.reset(self.serialno);
                work.reset(self.serialno);
                self.ready = Ready::StreamSet;
            }
            self.os.pagein(&og);
            work.pagein(&og);
            eosflag = og.eos();
        }
    }

    /// `_fetch_and_process_packet(vf, NULL, 1, 1)`.
    fn fetch_and_process(&mut self) -> i32 {
        loop {
            if self.ready == Ready::InitSet {
                loop {
                    match self.os.packetout() {
                        Out::Hole => return OV_HOLE,
                        Out::None => break,
                        Out::Packet(p) => {
                            let dsp = self.dsp.as_mut().expect("decoding");
                            if let Ok(block) =
                                dsp.synthesis(&p.data, p.granulepos, p.packetno, p.eos)
                            {
                                if dsp.pcmout() != 0 {
                                    return OV_EFAULT;
                                }
                                dsp.blockin(&block);
                                return 1;
                            }
                        }
                    }
                }
            }
            let og = loop {
                let Some((_, og)) = self.next_page() else {
                    return OV_EOF;
                };
                if self.ready == Ready::InitSet && og.serialno() != self.serialno {
                    assert!(!og.bos(), "a chained Ogg file is not provided");
                    continue;
                }
                break og;
            };
            if self.ready != Ready::InitSet {
                if self.ready < Ready::StreamSet {
                    if og.serialno() != self.serialno {
                        continue;
                    }
                    self.os.reset(self.serialno);
                    self.ready = Ready::StreamSet;
                }
                self.dsp = Some(Dsp::new(self.info.clone(), self.fpu));
                self.ready = Ready::InitSet;
            }
            self.os.pagein(&og);
        }
    }

    /// `ov_read_float(vf, &pcm, length, &bitstream)`: up to `length` samples of each channel;
    /// none at the end; or an error.
    pub fn read_float(&mut self, length: usize) -> Result<Vec<Vec<f32>>, i32> {
        loop {
            if self.ready == Ready::InitSet {
                let dsp = self.dsp.as_mut().expect("decoding");
                let s = dsp.pcmout();
                if s > 0 {
                    let s = s.min(length);
                    let out = (0..self.info.channels as usize)
                        .map(|c| dsp.pcm(c)[..s].to_vec())
                        .collect();
                    dsp.read(s);
                    return Ok(out);
                }
            }
            let r = self.fetch_and_process();
            if r == OV_EOF {
                return Ok(Vec::new());
            }
            if r <= 0 {
                return Err(r);
            }
        }
    }

    /// `ov_info(vf, -1)`.
    pub fn info(&self) -> &Info {
        &self.info
    }

    /// `ov_seekable`.
    pub fn seekable(&self) -> bool {
        true
    }

    /// `ov_pcm_total(vf, i)`.
    pub fn pcm_total(&self, i: i32) -> i64 {
        if i >= 1 {
            return i64::from(OV_EINVAL);
        }
        self.pcmlengths[1]
    }

    /// `ov_read(vf, buffer, length, 0, 2, 1, &bitstream)`: bytes written, 0 at the end, or an
    /// error.
    #[allow(clippy::needless_range_loop)] // libvorbisfile's loops.
    pub fn read(&mut self, buffer: &mut [u8]) -> i64 {
        let length = buffer.len() as i64;
        let samples = loop {
            if self.ready == Ready::InitSet {
                let s = self.dsp.as_ref().expect("decoding").pcmout();
                if s > 0 {
                    break s as i64;
                }
            }
            let r = self.fetch_and_process();
            if r == OV_EOF {
                return 0;
            }
            if r <= 0 {
                return i64::from(r);
            }
        };
        let channels = self.info.channels as i64;
        let bytespersample = 2 * channels;
        let samples = samples.min(length / bytespersample);
        if samples <= 0 {
            return i64::from(OV_EINVAL);
        }
        let dsp = self.dsp.as_mut().expect("decoding");
        let ch = channels as usize;
        for i in 0..ch {
            let src = dsp.pcm(i);
            for j in 0..samples as usize {
                let v = ftoi(src[j] * 32768.0).clamp(-32768, 32767) as i16;
                let at = 2 * (j * ch + i);
                buffer[at..at + 2].copy_from_slice(&v.to_le_bytes());
            }
        }
        dsp.read(samples as usize);
        samples * bytespersample
    }
}

/// `vorbis_ftoi` on MSVC: `fistp`, rounding to nearest even; out of range (or NaN) the integer
/// indefinite value.
fn ftoi(f: f32) -> i32 {
    let x = f64::from(f);
    let mut r = x.round();
    if (x - x.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r -= x.signum();
    }
    if (-2_147_483_648.0..=2_147_483_647.0).contains(&r) {
        r as i32
    } else {
        i32::MIN
    }
}
