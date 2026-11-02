// Follows libvorbis 1.2.0 (lib/synthesis.c, lib/block.c, lib/mapping0.c, lib/floor1.c,
// lib/res0.c), Copyright (c) 2002-2004 Xiph.org Foundation, BSD licence: see
// ../LICENSE-libvorbis.
//! Audio packets to PCM: `vorbis_synthesis` (mode and window flags, then mapping 0: floors,
//! residues, coupling, the floor curve, the inverse MDCT) and `vorbis_synthesis_blockin` (the
//! windowed overlap-add into the two-stage buffer, the granule position bookkeeping that trims
//! the stream's start and end), in `f32` in libvorbis' order (see `mdct` for why `f32`).

use crate::bits::{ilog, ilog_ceil, Bits};
use crate::fpu::{store, Fpu};
use crate::mdct::Mdct;
use crate::setup::{Floor1, Info, Residue, OV_EBADPACKET, OV_ENOTAUDIO};
use crate::tables::{FLOOR1_FROM_DB, VWIN};

/// A decoded block (`vorbis_block`).
pub struct Block {
    pub w: bool,
    pub granulepos: i64,
    pub sequence: i64,
    pub eofflag: bool,
    pub pcm: Vec<Vec<f32>>,
}

/// `vorbis_dsp_state` for decoding.
pub struct Dsp {
    pub info: Info,
    /// For measuring: leave the spectrum (the inverse MDCT's input) in the block.
    pub(crate) skip_transform: bool,
    fpu: Fpu,
    pub(crate) transforms: [Mdct; 2],
    modebits: u32,
    pcm: Vec<Vec<f32>>,
    lw: bool,
    w: bool,
    center_w: usize,
    pcm_current: i64,
    pcm_returned: i64,
    granulepos: i64,
    sequence: i64,
    sample_count: i64,
}

impl Dsp {
    /// `vorbis_synthesis_init`.
    pub fn new(mut info: Info, fpu: Fpu) -> Dsp {
        let transforms = [
            Mdct::new(info.blocksizes[0], fpu),
            Mdct::new(info.blocksizes[1], fpu),
        ];
        for book in info.books.iter_mut() {
            book.prepare(fpu);
        }
        let modebits = ilog_ceil(info.modes.len() as u32);
        let channels = info.channels as usize;
        let storage = info.blocksizes[1];
        let mut d = Dsp {
            transforms,
            modebits,
            pcm: vec![vec![0f32; storage]; channels],
            lw: false,
            w: false,
            center_w: 0,
            pcm_current: 0,
            pcm_returned: -1,
            granulepos: -1,
            sequence: -1,
            sample_count: -1,
            skip_transform: false,
            info,
            fpu,
        };
        d.restart();
        d
    }

    /// `vorbis_synthesis_restart`.
    pub fn restart(&mut self) {
        self.center_w = self.info.blocksizes[1] / 2;
        self.pcm_current = self.center_w as i64;
        self.pcm_returned = -1;
        self.granulepos = -1;
        self.sequence = -1;
        self.sample_count = -1;
    }

    /// `vorbis_packet_blocksize`.
    pub fn packet_blocksize(info: &Info, data: &[u8]) -> i64 {
        let mut b = Bits::new(data);
        if b.read(1) != 0 {
            return i64::from(OV_ENOTAUDIO);
        }
        let mut modebits = 0;
        let mut v = info.modes.len();
        while v > 1 {
            modebits += 1;
            v >>= 1;
        }
        let mode = b.read(modebits);
        if mode == -1 {
            return i64::from(OV_EBADPACKET);
        }
        info.blocksizes[usize::from(info.modes[mode as usize].blockflag)] as i64
    }

    /// `vorbis_synthesis`: the packet decoded into a block, or a libvorbis error.
    pub fn synthesis(
        &self,
        data: &[u8],
        granulepos: i64,
        packetno: i64,
        eos: bool,
    ) -> Result<Block, i32> {
        let info = &self.info;
        let mut b = Bits::new(data);
        if b.read(1) != 0 {
            return Err(OV_ENOTAUDIO);
        }
        let mode = b.read(self.modebits);
        if mode == -1 {
            return Err(OV_EBADPACKET);
        }
        let mode = info.modes[mode as usize];
        let w = mode.blockflag;
        if w {
            b.read(1);
            if b.read(1) == -1 {
                return Err(OV_EBADPACKET);
            }
        }
        let n = info.blocksizes[usize::from(w)];
        let channels = info.channels as usize;
        let mut pcm = vec![vec![0f32; n]; channels];
        self.mapping0(&mut b, mode.mapping, w, &mut pcm);
        Ok(Block {
            w,
            granulepos,
            sequence: packetno,
            eofflag: eos,
            pcm,
        })
    }

    #[allow(clippy::needless_range_loop)] // libvorbis' loops, index for index.
    fn mapping0(&self, b: &mut Bits<'_>, mapping: usize, w: bool, pcm: &mut [Vec<f32>]) {
        let info = &self.info;
        let map = &info.mappings[mapping];
        let n = info.blocksizes[usize::from(w)];
        let channels = pcm.len();
        let mut memo: Vec<Option<Vec<i32>>> = Vec::with_capacity(channels);
        let mut nonzero = vec![false; channels];
        for i in 0..channels {
            let submap = map.chmuxlist[i];
            let floor = &info.floors[map.floorsubmap[submap]];
            let m = floor1_inverse1(info, floor, b);
            nonzero[i] = m.is_some();
            memo.push(m);
            pcm[i][..n / 2].fill(0.0);
        }
        for &(mag, ang) in &map.coupling {
            if nonzero[mag] || nonzero[ang] {
                nonzero[mag] = true;
                nonzero[ang] = true;
            }
        }
        for i in 0..map.submaps {
            let mut bundle: Vec<&mut [f32]> = Vec::new();
            let mut zero: Vec<bool> = Vec::new();
            for (j, ch) in pcm.iter_mut().enumerate() {
                if map.chmuxlist[j] == i {
                    zero.push(nonzero[j]);
                    bundle.push(&mut ch[..]);
                }
            }
            let r = &info.residues[map.residuesubmap[i]];
            residue_inverse(info, r, b, n, bundle, &zero);
        }
        for &(mag, ang) in map.coupling.iter().rev() {
            for j in 0..n / 2 {
                let m = pcm[mag][j];
                let a = pcm[ang][j];
                let (nm, na) = if m > 0.0 {
                    if a > 0.0 {
                        (m, m - a)
                    } else {
                        (m + a, m)
                    }
                } else if a > 0.0 {
                    (m, m + a)
                } else {
                    (m - a, m)
                };
                pcm[mag][j] = nm;
                pcm[ang][j] = na;
            }
        }
        for i in 0..channels {
            let submap = map.chmuxlist[i];
            let floor = &info.floors[map.floorsubmap[submap]];
            floor1_inverse2(floor, memo[i].as_deref(), &mut pcm[i], n / 2);
        }
        if self.skip_transform {
            return;
        }
        for ch in pcm.iter_mut() {
            self.transforms[usize::from(w)].backward(ch);
        }
    }

    /// `vorbis_synthesis_blockin`.
    pub fn blockin(&mut self, vb: &Block) {
        let f = self.fpu;
        if self.pcm_current > self.pcm_returned && self.pcm_returned != -1 {
            // OV_EINVAL: vorbisfile never gets here (it reads every sample first).
            return;
        }
        let bs = self.info.blocksizes;
        self.lw = self.w;
        self.w = vb.w;
        if self.sequence == -1 || self.sequence + 1 != vb.sequence {
            self.granulepos = -1;
            self.sample_count = -1;
        }
        self.sequence = vb.sequence;
        let n = bs[usize::from(self.w)] >> 1;
        let n0 = bs[0] >> 1;
        let n1 = bs[1] >> 1;
        let (this_center, prev_center) = if self.center_w != 0 { (n1, 0) } else { (0, n1) };
        let wl = VWIN[(ilog_ceil(bs[1] as u32) - 6) as usize];
        let ws = VWIN[(ilog_ceil(bs[0] as u32) - 6) as usize];
        for (j, out) in self.pcm.iter_mut().enumerate() {
            let p = &vb.pcm[j];
            if self.lw {
                if self.w {
                    let pc = &mut out[prev_center..];
                    for i in 0..n1 {
                        pc[i] = lap(f, pc[i], wl[n1 - i - 1], p[i], wl[i]);
                    }
                } else {
                    let pc = &mut out[prev_center + n1 / 2 - n0 / 2..];
                    for i in 0..n0 {
                        pc[i] = lap(f, pc[i], ws[n0 - i - 1], p[i], ws[i]);
                    }
                }
            } else if self.w {
                let pc = &mut out[prev_center..];
                let pp = &p[n1 / 2 - n0 / 2..];
                let mut i = 0;
                while i < n0 {
                    pc[i] = lap(f, pc[i], ws[n0 - i - 1], pp[i], ws[i]);
                    i += 1;
                }
                while i < n1 / 2 + n0 / 2 {
                    pc[i] = pp[i];
                    i += 1;
                }
            } else {
                let pc = &mut out[prev_center..];
                for i in 0..n0 {
                    pc[i] = lap(f, pc[i], ws[n0 - i - 1], p[i], ws[i]);
                }
            }
            out[this_center..this_center + n].copy_from_slice(&p[n..2 * n]);
        }
        self.center_w = if self.center_w != 0 { 0 } else { n1 };
        if self.pcm_returned == -1 {
            self.pcm_returned = this_center as i64;
            self.pcm_current = this_center as i64;
        } else {
            self.pcm_returned = prev_center as i64;
            self.pcm_current = prev_center as i64
                + (bs[usize::from(self.lw)] / 4 + bs[usize::from(self.w)] / 4) as i64;
        }
        let step = (bs[usize::from(self.lw)] / 4 + bs[usize::from(self.w)] / 4) as i64;
        if self.sample_count == -1 {
            self.sample_count = 0;
        } else {
            self.sample_count += step;
        }
        if self.granulepos == -1 {
            if vb.granulepos != -1 {
                self.granulepos = vb.granulepos;
                if self.sample_count > self.granulepos {
                    if vb.eofflag {
                        self.pcm_current -= self.sample_count - self.granulepos;
                    } else {
                        self.pcm_returned += self.sample_count - self.granulepos;
                        if self.pcm_returned > self.pcm_current {
                            self.pcm_returned = self.pcm_current;
                        }
                    }
                }
            }
        } else {
            self.granulepos += step;
            if vb.granulepos != -1 && self.granulepos != vb.granulepos {
                if self.granulepos > vb.granulepos {
                    let extra = self.granulepos - vb.granulepos;
                    if extra != 0 && vb.eofflag {
                        self.pcm_current -= extra;
                    }
                }
                self.granulepos = vb.granulepos;
            }
        }
    }

    /// `vorbis_synthesis_pcmout`: samples ready (and where they start).
    pub fn pcmout(&self) -> usize {
        if self.pcm_returned > -1 && self.pcm_returned < self.pcm_current {
            (self.pcm_current - self.pcm_returned) as usize
        } else {
            0
        }
    }

    /// The ready samples of channel `ch`.
    pub fn pcm(&self, ch: usize) -> &[f32] {
        &self.pcm[ch][self.pcm_returned as usize..self.pcm_current as usize]
    }

    /// `vorbis_synthesis_read`.
    pub fn read(&mut self, n: usize) {
        self.pcm_returned += n as i64;
    }
}

/// `pcm[i] * w[a] + p[i] * w[b]`, one C expression.
fn lap(f: Fpu, a: f32, wa: f32, b: f32, wb: f32) -> f32 {
    store(f.add(
        f.mul(f64::from(a), f64::from(wa)),
        f.mul(f64::from(b), f64::from(wb)),
    ))
}

/// `render_point`.
fn render_point(x0: i32, x1: i32, y0: i32, y1: i32, x: i32) -> i32 {
    let y0 = y0 & 0x7fff;
    let y1 = y1 & 0x7fff;
    let dy = y1 - y0;
    let adx = x1 - x0;
    let ady = dy.abs();
    let err = ady * (x - x0);
    let off = err / adx;
    if dy < 0 {
        y0 - off
    } else {
        y0 + off
    }
}

/// `render_line`: multiplies `d` by the floor's line from `(x0, y0)` to `(x1, y1)`.
fn render_line(n: usize, x0: i32, x1: i32, y0: i32, y1: i32, d: &mut [f32]) {
    let dy = y1 - y0;
    let adx = x1 - x0;
    let mut ady = dy.abs();
    let base = dy / adx;
    let sy = if dy < 0 { base - 1 } else { base + 1 };
    let mut x = x0;
    let mut y = y0;
    let mut err = 0;
    ady -= (base * adx).abs();
    let n = (n as i32).min(x1);
    if x < n {
        d[x as usize] *= FLOOR1_FROM_DB[y as usize];
    }
    loop {
        x += 1;
        if x >= n {
            break;
        }
        err += ady;
        if err >= adx {
            err -= adx;
            y += sy;
        } else {
            y += base;
        }
        d[x as usize] *= FLOOR1_FROM_DB[y as usize];
    }
}

/// `floor1_inverse1`: the floor's posts, or `None` (an unused floor, or the packet ended).
fn floor1_inverse1(info: &Info, f: &Floor1, b: &mut Bits<'_>) -> Option<Vec<i32>> {
    if b.read(1) != 1 {
        return None;
    }
    let mut fit = vec![0i32; f.posts];
    let bits = ilog((f.quant_q - 1) as u32);
    fit[0] = b.read(bits);
    fit[1] = b.read(bits);
    let mut j = 2;
    for i in 0..f.partitions {
        let class = f.partitionclass[i];
        let cdim = f.class_dim[class];
        let csubbits = f.class_subs[class];
        let csub = 1i64 << csubbits;
        let mut cval = 0i64;
        if csubbits != 0 {
            cval = info.books[f.class_book[class]].decode(b);
            if cval == -1 {
                return None;
            }
        }
        for k in 0..cdim {
            let book = f.class_subbook[class][(cval & (csub - 1)) as usize];
            cval >>= csubbits;
            if book >= 0 {
                let v = info.books[book as usize].decode(b);
                fit[j + k] = v as i32;
                if v == -1 {
                    return None;
                }
            } else {
                fit[j + k] = 0;
            }
        }
        j += cdim;
    }
    for i in 2..f.posts {
        let lo = f.loneighbor[i - 2];
        let hi = f.hineighbor[i - 2];
        let predicted = render_point(
            f.postlist[lo],
            f.postlist[hi],
            fit[lo],
            fit[hi],
            f.postlist[i],
        );
        let hiroom = f.quant_q - predicted;
        let loroom = predicted;
        let room = hiroom.min(loroom) << 1;
        let mut val = fit[i];
        if val != 0 {
            if val >= room {
                if hiroom > loroom {
                    val -= loroom;
                } else {
                    val = -1 - (val - hiroom);
                }
            } else if val & 1 != 0 {
                val = -((val + 1) >> 1);
            } else {
                val >>= 1;
            }
            fit[i] = val + predicted;
            fit[lo] &= 0x7fff;
            fit[hi] &= 0x7fff;
        } else {
            fit[i] = predicted | 0x8000;
        }
    }
    Some(fit)
}

/// `floor1_inverse2`: multiplies the spectrum by the floor curve, or clears it.
fn floor1_inverse2(f: &Floor1, memo: Option<&[i32]>, out: &mut [f32], n: usize) {
    let Some(fit) = memo else {
        out[..n].fill(0.0);
        return;
    };
    let mut hx = 0;
    let mut lx = 0;
    let mut ly = fit[0] * f.mult;
    for j in 1..f.posts {
        let current = f.forward_index[j];
        let hy = fit[current] & 0x7fff;
        if hy == fit[current] {
            let hy = hy * f.mult;
            hx = f.postlist[current];
            render_line(n, lx, hx, ly, hy, out);
            lx = hx;
            ly = hy;
        }
    }
    for v in out.iter_mut().take(n).skip(hx as usize) {
        *v *= FLOOR1_FROM_DB[ly as usize];
    }
}

/// The residue types' `inverse`.
fn residue_inverse(
    info: &Info,
    r: &Residue,
    b: &mut Bits<'_>,
    pcmend: usize,
    mut bundle: Vec<&mut [f32]>,
    nonzero: &[bool],
) {
    if r.kind == 2 {
        let ch = bundle.len();
        let max = ((pcmend * ch) >> 1) as i32;
        let end = r.end.min(max);
        let n = end - r.begin;
        if n <= 0 || !nonzero.iter().any(|&z| z) {
            return;
        }
        let spp = r.grouping as usize;
        let ppw = info.books[r.groupbook].dim;
        let partvals = n as usize / spp;
        let mut partword: Vec<usize> = Vec::new();
        for s in 0..r.stages {
            let (mut i, mut l) = (0usize, 0usize);
            while i < partvals {
                if s == 0 {
                    let temp = info.books[r.groupbook].decode(b);
                    if temp == -1 {
                        return;
                    }
                    partword.push(temp as usize);
                }
                let mut k = 0;
                while k < ppw && i < partvals {
                    let part = r.decodemap[partword[l]][k];
                    if r.secondstages[part] & (1 << s) != 0 {
                        if let Some(book) = r.partbooks[part][s] {
                            if !info.books[book].decodevv_add(
                                &mut bundle,
                                i * spp + r.begin as usize,
                                b,
                                spp,
                            ) {
                                return;
                            }
                        }
                    }
                    k += 1;
                    i += 1;
                }
                l += 1;
            }
        }
        return;
    }
    // Types 0 and 1: only the channels with a floor.
    let mut used: Vec<&mut [f32]> = Vec::new();
    for (ch, &z) in bundle.drain(..).zip(nonzero) {
        if z {
            used.push(ch);
        }
    }
    let ch = used.len();
    if ch == 0 {
        return;
    }
    let max = (pcmend >> 1) as i32;
    let end = r.end.min(max);
    let n = end - r.begin;
    if n <= 0 {
        return;
    }
    let spp = r.grouping as usize;
    let ppw = info.books[r.groupbook].dim;
    let partvals = n as usize / spp;
    let mut partword: Vec<Vec<usize>> = vec![Vec::new(); ch];
    for s in 0..r.stages {
        let (mut i, mut l) = (0usize, 0usize);
        while i < partvals {
            if s == 0 {
                for pw in partword.iter_mut() {
                    let temp = info.books[r.groupbook].decode(b);
                    if temp == -1 {
                        return;
                    }
                    pw.push(temp as usize);
                }
            }
            let mut k = 0;
            while k < ppw && i < partvals {
                for j in 0..ch {
                    let offset = r.begin as usize + i * spp;
                    let part = r.decodemap[partword[j][l]][k];
                    if r.secondstages[part] & (1 << s) != 0 {
                        if let Some(book) = r.partbooks[part][s] {
                            let book = &info.books[book];
                            let ok = if r.kind == 0 {
                                book.decodevs_add(&mut used[j][offset..], b, spp)
                            } else {
                                book.decodev_add(&mut used[j][offset..], b, spp)
                            };
                            if !ok {
                                return;
                            }
                        }
                    }
                }
                k += 1;
                i += 1;
            }
            l += 1;
        }
    }
}
