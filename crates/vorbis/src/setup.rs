// Follows libvorbis 1.2.0 (lib/info.c, lib/floor1.c, lib/res0.c, lib/mapping0.c), Copyright (c)
// 2002-2004 Xiph.org Foundation, BSD licence: see ../LICENSE-libvorbis.
//! The three header packets: identification, comments (checked, not kept) and setup (codebooks,
//! floors, residues, mappings, modes), with libvorbis' checks and the lookups it derives
//! (`floor1_look`, `res0_look`). Floor type 0 is not provided (no game file uses it).

use crate::bits::{ilog, ilog_ceil, Bits};
use crate::codebook::Book;

/// libvorbisfile's error codes.
pub const OV_EFAULT: i32 = -129;
pub const OV_EINVAL: i32 = -131;
pub const OV_ENOTVORBIS: i32 = -132;
pub const OV_EBADHEADER: i32 = -133;
pub const OV_EVERSION: i32 = -134;
pub const OV_ENOTAUDIO: i32 = -135;
pub const OV_EBADPACKET: i32 = -136;
pub const OV_HOLE: i32 = -3;

/// `vorbis_info` (what `ov_info` gives) and the codec setup.
#[derive(Debug, Clone, Default)]
pub struct Info {
    pub version: i32,
    pub channels: i32,
    pub rate: i32,
    pub bitrate_upper: i32,
    pub bitrate_nominal: i32,
    pub bitrate_lower: i32,
    pub blocksizes: [usize; 2],
    pub books: Vec<Book>,
    pub floors: Vec<Floor1>,
    pub residues: Vec<Residue>,
    pub mappings: Vec<Mapping>,
    pub modes: Vec<Mode>,
    /// Whether the comment header came (libvorbis' `vc->vendor`).
    comments_read: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Floor1 {
    pub partitions: usize,
    pub partitionclass: Vec<usize>,
    pub class_dim: [usize; 16],
    pub class_subs: [u32; 16],
    pub class_book: [usize; 16],
    pub class_subbook: [[i32; 8]; 16],
    pub mult: i32,
    pub postlist: Vec<i32>,
    // floor1_look
    pub posts: usize,
    pub forward_index: Vec<usize>,
    pub loneighbor: Vec<usize>,
    pub hineighbor: Vec<usize>,
    pub quant_q: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Residue {
    pub kind: i32,
    pub begin: i32,
    pub end: i32,
    pub grouping: i32,
    pub partitions: usize,
    pub groupbook: usize,
    pub secondstages: Vec<i32>,
    // res0_look
    pub partbooks: Vec<Vec<Option<usize>>>,
    pub stages: usize,
    pub decodemap: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Default)]
pub struct Mapping {
    pub submaps: usize,
    pub coupling: Vec<(usize, usize)>,
    pub chmuxlist: Vec<usize>,
    pub floorsubmap: Vec<usize>,
    pub residuesubmap: Vec<usize>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Mode {
    pub blockflag: bool,
    pub mapping: usize,
}

fn readstring(b: &mut Bits<'_>, n: usize) -> Vec<u8> {
    (0..n).map(|_| b.read(8) as u8).collect()
}

/// `vorbis_synthesis_idheader`.
pub fn is_id_header(data: &[u8], bos: bool) -> bool {
    let mut b = Bits::new(data);
    bos && b.read(8) == 1 && readstring(&mut b, 6) == b"vorbis"
}

impl Info {
    /// `vorbis_synthesis_headerin`: 0 or a libvorbis error.
    pub fn headerin(&mut self, data: &[u8], bos: bool) -> i32 {
        let mut b = Bits::new(data);
        let packtype = b.read(8);
        if readstring(&mut b, 6) != b"vorbis" {
            return OV_ENOTVORBIS;
        }
        match packtype {
            1 => {
                if !bos || self.rate != 0 {
                    return OV_EBADHEADER;
                }
                self.unpack_info(&mut b)
            }
            3 => {
                if self.rate == 0 {
                    return OV_EBADHEADER;
                }
                self.unpack_comment(&mut b)
            }
            5 => {
                if self.rate == 0 || !self.comments_read {
                    return OV_EBADHEADER;
                }
                self.unpack_books(&mut b)
            }
            _ => OV_EBADHEADER,
        }
    }

    fn unpack_info(&mut self, b: &mut Bits<'_>) -> i32 {
        self.version = b.read(32);
        if self.version != 0 {
            return OV_EVERSION;
        }
        self.channels = b.read(8);
        self.rate = b.read(32);
        self.bitrate_upper = b.read(32);
        self.bitrate_nominal = b.read(32);
        self.bitrate_lower = b.read(32);
        let b0 = b.read(4);
        let b1 = b.read(4);
        self.blocksizes = [1usize << (b0 & 31), 1usize << (b1 & 31)];
        let ok = self.rate >= 1
            && self.channels >= 1
            && self.blocksizes[0] >= 64
            && self.blocksizes[1] >= self.blocksizes[0]
            && self.blocksizes[1] <= 8192
            && b.read(1) == 1;
        if !ok {
            *self = Info::default();
            return OV_EBADHEADER;
        }
        0
    }

    fn unpack_comment(&mut self, b: &mut Bits<'_>) -> i32 {
        let vendorlen = b.read(32);
        if vendorlen < 0 {
            return OV_EBADHEADER;
        }
        readstring(b, vendorlen as usize);
        let comments = b.read(32);
        if comments < 0 {
            return OV_EBADHEADER;
        }
        for _ in 0..comments {
            let len = b.read(32);
            if len < 0 {
                return OV_EBADHEADER;
            }
            readstring(b, len as usize);
        }
        if b.read(1) != 1 {
            return OV_EBADHEADER;
        }
        self.comments_read = true;
        0
    }

    fn unpack_books(&mut self, b: &mut Bits<'_>) -> i32 {
        match self.setup(b) {
            Some(()) => 0,
            None => {
                *self = Info::default();
                OV_EBADHEADER
            }
        }
    }

    fn setup(&mut self, b: &mut Bits<'_>) -> Option<()> {
        let books = (b.read(8) + 1) as usize;
        for _ in 0..books {
            let book = Book::read(b)?;
            self.books.push(book);
        }
        let times = b.read(6) + 1;
        for _ in 0..times {
            let t = b.read(16);
            if t != 0 {
                return None;
            }
        }
        let floors = b.read(6) + 1;
        for _ in 0..floors {
            match b.read(16) {
                1 => {
                    let f = self.floor1(b)?;
                    self.floors.push(f);
                }
                0 => panic!("a Vorbis stream with floor type 0 is not provided"),
                _ => return None,
            }
        }
        let residues = b.read(6) + 1;
        for _ in 0..residues {
            let kind = b.read(16);
            if !(0..3).contains(&kind) {
                return None;
            }
            let r = self.residue(b, kind)?;
            self.residues.push(r);
        }
        let maps = b.read(6) + 1;
        for _ in 0..maps {
            if b.read(16) != 0 {
                return None;
            }
            let m = self.mapping(b)?;
            self.mappings.push(m);
        }
        let modes = b.read(6) + 1;
        for _ in 0..modes {
            let blockflag = b.read(1) == 1;
            let windowtype = b.read(16);
            let transformtype = b.read(16);
            let mapping = b.read(8);
            if windowtype >= 1 || transformtype >= 1 || mapping >= self.mappings.len() as i32 {
                return None;
            }
            self.modes.push(Mode {
                blockflag,
                mapping: mapping as usize,
            });
        }
        if b.read(1) != 1 {
            return None;
        }
        Some(())
    }

    fn floor1(&self, b: &mut Bits<'_>) -> Option<Floor1> {
        let mut f = Floor1 {
            partitions: b.read(5) as usize,
            ..Floor1::default()
        };
        let mut maxclass = -1i32;
        for _ in 0..f.partitions {
            let c = b.read(4);
            maxclass = maxclass.max(c);
            f.partitionclass.push(c as usize);
        }
        for j in 0..(maxclass + 1) as usize {
            f.class_dim[j] = (b.read(3) + 1) as usize;
            let subs = b.read(2);
            if subs < 0 {
                return None;
            }
            f.class_subs[j] = subs as u32;
            let mut book = 0i32;
            if subs != 0 {
                book = b.read(8);
            }
            if book < 0 || book as usize >= self.books.len() {
                return None;
            }
            f.class_book[j] = book as usize;
            for k in 0..(1usize << subs) {
                let sb = b.read(8) - 1;
                if sb < -1 || sb >= self.books.len() as i32 {
                    return None;
                }
                f.class_subbook[j][k] = sb;
            }
        }
        f.mult = b.read(2) + 1;
        let rangebits = b.read(4);
        f.postlist = vec![0, 1 << rangebits];
        for j in 0..f.partitions {
            for _ in 0..f.class_dim[f.partitionclass[j]] {
                let t = b.read(rangebits as u32);
                if t < 0 || t >= (1 << rangebits) {
                    return None;
                }
                f.postlist.push(t);
            }
        }
        // floor1_look.
        let n = f.postlist.len();
        f.posts = n;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| f.postlist[i]);
        for w in order.windows(2) {
            assert!(
                f.postlist[w[0]] != f.postlist[w[1]],
                "a floor with two posts at {} is not provided (libvorbis' qsort order)",
                f.postlist[w[0]]
            );
        }
        f.forward_index = order;
        f.quant_q = match f.mult {
            1 => 256,
            2 => 128,
            3 => 86,
            _ => 64,
        };
        let range = f.postlist[1];
        for i in 0..n - 2 {
            let (mut lo, mut hi, mut lx, mut hx) = (0usize, 1usize, 0i32, range);
            let currentx = f.postlist[i + 2];
            for j in 0..i + 2 {
                let x = f.postlist[j];
                if x > lx && x < currentx {
                    lo = j;
                    lx = x;
                }
                if x < hx && x > currentx {
                    hi = j;
                    hx = x;
                }
            }
            f.loneighbor.push(lo);
            f.hineighbor.push(hi);
        }
        Some(f)
    }

    fn residue(&self, b: &mut Bits<'_>, kind: i32) -> Option<Residue> {
        let mut r = Residue {
            kind,
            begin: b.read(24),
            end: b.read(24),
            grouping: b.read(24) + 1,
            partitions: (b.read(6) + 1) as usize,
            groupbook: b.read(8) as usize,
            ..Residue::default()
        };
        let mut acc = 0;
        for _ in 0..r.partitions {
            let mut cascade = b.read(3);
            if b.read(1) != 0 {
                cascade |= b.read(5) << 3;
            }
            r.secondstages.push(cascade);
            acc += (cascade as u32).count_ones();
        }
        let booklist: Vec<i32> = (0..acc).map(|_| b.read(8)).collect();
        if r.groupbook >= self.books.len()
            || booklist.iter().any(|&x| x as usize >= self.books.len())
        {
            return None;
        }
        // res0_look.
        let dim = self.books[r.groupbook].dim;
        let mut acc = 0;
        let mut maxstage = 0;
        for j in 0..r.partitions {
            let stages = ilog(r.secondstages[j] as u32) as usize;
            let mut books = vec![None; stages];
            if stages > 0 {
                maxstage = maxstage.max(stages);
                for (k, slot) in books.iter_mut().enumerate() {
                    if r.secondstages[j] & (1 << k) != 0 {
                        *slot = Some(booklist[acc] as usize);
                        acc += 1;
                    }
                }
            }
            r.partbooks.push(books);
        }
        r.stages = maxstage;
        let partvals = r.partitions.pow(dim as u32);
        for j in 0..partvals {
            let mut val = j;
            let mut mult = partvals / r.partitions;
            let mut word = Vec::with_capacity(dim);
            for _ in 0..dim {
                let deco = val / mult;
                val -= deco * mult;
                mult /= r.partitions;
                word.push(deco);
            }
            r.decodemap.push(word);
        }
        Some(r)
    }

    fn mapping(&self, b: &mut Bits<'_>) -> Option<Mapping> {
        let channels = self.channels as usize;
        let mut m = Mapping {
            submaps: if b.read(1) != 0 {
                (b.read(4) + 1) as usize
            } else {
                1
            },
            chmuxlist: vec![0; channels],
            ..Mapping::default()
        };
        if b.read(1) != 0 {
            let steps = b.read(8) + 1;
            let bits = ilog_ceil(channels as u32);
            for _ in 0..steps {
                let mag = b.read(bits);
                let ang = b.read(bits);
                if mag < 0
                    || ang < 0
                    || mag == ang
                    || mag as usize >= channels
                    || ang as usize >= channels
                {
                    return None;
                }
                m.coupling.push((mag as usize, ang as usize));
            }
        }
        if b.read(2) > 0 {
            return None;
        }
        if m.submaps > 1 {
            for c in m.chmuxlist.iter_mut() {
                let v = b.read(4);
                if v as usize >= m.submaps {
                    return None;
                }
                *c = v as usize;
            }
        }
        for _ in 0..m.submaps {
            b.read(8);
            let f = b.read(8);
            if f as usize >= self.floors.len() {
                return None;
            }
            m.floorsubmap.push(f as usize);
            let r = b.read(8);
            if r as usize >= self.residues.len() {
                return None;
            }
            m.residuesubmap.push(r as usize);
        }
        Some(m)
    }
}
