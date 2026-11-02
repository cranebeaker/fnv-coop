// Follows libvorbis 1.2.0 (lib/codebook.c, lib/sharedbook.c), Copyright (c) 2002-2004 Xiph.org
// Foundation, BSD licence: see ../LICENSE-libvorbis.
//! Codebooks: unpacking from the setup header, libvorbis' decode tables (codewords sorted by
//! their bit-reversed value, a first-stage table of up to 8 bits, then a bisection), and the
//! value vectors (`_book_unquantize`, in `f32` in libvorbis' order).

use crate::bits::{ilog, Bits};
use crate::fpu::{store, Fpu};

/// A codebook as the decoder uses it.
#[derive(Debug, Clone)]
pub struct Book {
    pub dim: usize,
    pub entries: usize,
    pub used: usize,
    /// The value vectors by sorted index (`valuelist`), `dim` each; empty without a mapping.
    pub values: Vec<f32>,
    /// What the values are made from at `vorbis_synthesis_init` ([`Book::prepare`]).
    quant: Option<(Static, Vec<usize>)>,
    codelist: Vec<u32>,
    dec_index: Vec<u32>,
    dec_lengths: Vec<u32>,
    firsttable: Vec<u32>,
    firsttablen: u32,
    maxlength: u32,
}

/// The codebook as read (`static_codebook`).
#[derive(Debug, Clone)]
struct Static {
    dim: i32,
    entries: i32,
    lengths: Vec<i32>,
    maptype: i32,
    q_min: i32,
    q_delta: i32,
    q_sequencep: bool,
    quantlist: Vec<i32>,
}

/// `_float32_unpack`.
fn float32_unpack(val: i32) -> f32 {
    let val = val as u32;
    let mut mant = f64::from(val & 0x1f_ffff);
    let exp = ((val & 0x7fe0_0000) >> 21) as i32;
    if val & 0x8000_0000 != 0 {
        mant = -mant;
    }
    (mant * 2f64.powi(exp - 20 - 768)) as f32
}

/// `_book_maptype1_quantvals`: the largest `v` with `v^dim <= entries` (the integer check that
/// libvorbis runs after its floating-point guess, which settles the answer).
fn maptype1_quantvals(entries: i32, dim: i32) -> i32 {
    let mut vals = (f64::from(entries)).powf(1.0 / f64::from(dim)).floor() as i64;
    loop {
        let (mut acc, mut acc1) = (1i64, 1i64);
        for _ in 0..dim {
            acc = acc.wrapping_mul(vals);
            acc1 = acc1.wrapping_mul(vals + 1);
        }
        let e = i64::from(entries);
        if acc <= e && acc1 > e {
            return vals as i32;
        }
        if acc > e {
            vals -= 1;
        } else {
            vals += 1;
        }
    }
}

/// `vorbis_staticbook_unpack`; `None` for a bad book.
fn unpack(b: &mut Bits<'_>) -> Option<Static> {
    if b.read(24) != 0x56_4342 {
        return None;
    }
    let dim = b.read(16);
    let entries = b.read(24);
    if entries == -1 {
        return None;
    }
    let mut lengths = vec![0i32; entries.max(0) as usize];
    match b.read(1) {
        0 => {
            if b.read(1) != 0 {
                for l in lengths.iter_mut() {
                    if b.read(1) != 0 {
                        let num = b.read(5);
                        if num == -1 {
                            return None;
                        }
                        *l = num + 1;
                    } else {
                        *l = 0;
                    }
                }
            } else {
                for l in lengths.iter_mut() {
                    let num = b.read(5);
                    if num == -1 {
                        return None;
                    }
                    *l = num + 1;
                }
            }
        }
        1 => {
            let mut length = b.read(5) + 1;
            let mut i = 0usize;
            while i < lengths.len() {
                let num = b.read(ilog((entries as usize - i) as u32));
                if num == -1 {
                    return None;
                }
                let mut j = 0;
                while j < num && i < lengths.len() {
                    lengths[i] = length;
                    j += 1;
                    i += 1;
                }
                length += 1;
            }
        }
        _ => return None,
    }
    let maptype = b.read(4);
    let mut s = Static {
        dim,
        entries,
        lengths,
        maptype,
        q_min: 0,
        q_delta: 0,
        q_sequencep: false,
        quantlist: Vec::new(),
    };
    match maptype {
        0 => {}
        1 | 2 => {
            s.q_min = b.read(32);
            s.q_delta = b.read(32);
            let q_quant = (b.read(4) + 1) as u32;
            s.q_sequencep = b.read(1) == 1;
            let quantvals = if maptype == 1 {
                maptype1_quantvals(entries, dim)
            } else {
                entries * dim
            };
            s.quantlist = (0..quantvals).map(|_| b.read(q_quant)).collect();
            if quantvals > 0 && s.quantlist[quantvals as usize - 1] == -1 {
                return None;
            }
        }
        _ => return None,
    }
    Some(s)
}

/// `_make_words` with a sparse count: the codewords of the used entries, LSb first; `None` for
/// an overpopulated tree.
fn make_words(lengths: &[i32], sparsecount: usize) -> Option<Vec<u32>> {
    let mut marker = [0u32; 33];
    let mut r = Vec::with_capacity(sparsecount);
    for &length in lengths {
        if length > 0 {
            let length = length as usize;
            let mut entry = marker[length];
            if length < 32 && (entry >> length) != 0 {
                return None;
            }
            r.push(entry);
            let mut j = length;
            while j > 0 {
                if marker[j] & 1 != 0 {
                    if j == 1 {
                        marker[1] += 1;
                    } else {
                        marker[j] = marker[j - 1] << 1;
                    }
                    break;
                }
                marker[j] += 1;
                j -= 1;
            }
            for j in length + 1..33 {
                if (marker[j] >> 1) == entry {
                    entry = marker[j];
                    marker[j] = marker[j - 1] << 1;
                } else {
                    break;
                }
            }
        }
    }
    // Bit-reverse each word to its length.
    let mut count = 0;
    for &l in lengths {
        if l > 0 {
            let mut temp = 0u32;
            for j in 0..l as u32 {
                temp <<= 1;
                temp |= (r[count] >> j) & 1;
            }
            r[count] = temp;
            count += 1;
        }
    }
    Some(r)
}

/// `_book_unquantize` for the used entries, in sorted order.
fn unquantize(s: &Static, n: usize, sortindex: &[usize], f: Fpu) -> Vec<f32> {
    if s.maptype != 1 && s.maptype != 2 {
        return Vec::new();
    }
    let dim = s.dim as usize;
    let mindel = f64::from(float32_unpack(s.q_min));
    let delta = f64::from(float32_unpack(s.q_delta));
    let mut r = vec![0f32; n * dim];
    let mut count = 0;
    let quantvals = if s.maptype == 1 {
        maptype1_quantvals(s.entries, s.dim) as usize
    } else {
        0
    };
    for j in 0..s.entries as usize {
        if s.lengths[j] == 0 {
            continue;
        }
        let mut last = 0f64;
        let mut indexdiv = 1usize;
        for k in 0..dim {
            let q = if s.maptype == 1 {
                let index = (j / indexdiv) % quantvals;
                indexdiv *= quantvals;
                s.quantlist[index]
            } else {
                s.quantlist[j * dim + k]
            };
            // `float val = quantlist[index]; val = fabs(val) * delta + mindel + last;`
            let val = f.local(f.add(f.add(f.mul(f64::from(q as f32).abs(), delta), mindel), last));
            if s.q_sequencep {
                last = val;
            }
            r[sortindex[count] * dim + k] = store(val);
        }
        count += 1;
    }
    r
}

impl Book {
    /// Reads one codebook from the setup header (`vorbis_staticbook_unpack` then
    /// `vorbis_book_init_decode`).
    pub fn read(b: &mut Bits<'_>) -> Option<Book> {
        let s = unpack(b)?;
        let n = s.lengths.iter().filter(|&&l| l > 0).count();
        let mut book = Book {
            dim: s.dim as usize,
            entries: s.entries as usize,
            used: n,
            values: Vec::new(),
            quant: None,
            codelist: Vec::new(),
            dec_index: Vec::new(),
            dec_lengths: Vec::new(),
            firsttable: Vec::new(),
            firsttablen: 0,
            maxlength: 0,
        };
        if n == 0 {
            return Some(book);
        }
        let Some(mut codes) = make_words(&s.lengths, n) else {
            // An overpopulated tree: `vorbis_book_init_decode` fails and clears the book, and
            // `_vds_shared_init` goes on with the empty one.
            book.dim = 0;
            book.entries = 0;
            book.used = 0;
            return Some(book);
        };
        for c in codes.iter_mut() {
            *c = c.reverse_bits();
        }
        // qsort by value (codewords of a valid book are distinct).
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| codes[i]);
        let mut sortindex = vec![0usize; n];
        for (i, &position) in order.iter().enumerate() {
            sortindex[position] = i;
        }
        book.codelist = vec![0; n];
        for i in 0..n {
            book.codelist[sortindex[i]] = codes[i];
        }
        book.dec_index = vec![0; n];
        book.dec_lengths = vec![0; n];
        let mut k = 0;
        for (i, &l) in s.lengths.iter().enumerate() {
            if l > 0 {
                book.dec_index[sortindex[k]] = i as u32;
                book.dec_lengths[sortindex[k]] = l as u32;
                k += 1;
            }
        }
        book.quant = Some((s, sortindex));
        book.firsttablen = (ilog(n as u32) as i32 - 4).clamp(5, 8) as u32;
        let tabn = 1usize << book.firsttablen;
        book.firsttable = vec![0; tabn];
        for i in 0..n {
            let len = book.dec_lengths[i];
            book.maxlength = book.maxlength.max(len);
            if len <= book.firsttablen {
                let orig = book.codelist[i].reverse_bits();
                for j in 0..(1u32 << (book.firsttablen - len)) {
                    book.firsttable[(orig | (j << len)) as usize] = i as u32 + 1;
                }
            }
        }
        let mask = 0xffff_fffeu32 << (31 - book.firsttablen);
        let (mut lo, mut hi) = (0usize, 0usize);
        for i in 0..tabn {
            let word = (i as u32) << (32 - book.firsttablen);
            let at = word.reverse_bits() as usize;
            if book.firsttable[at] == 0 {
                while lo + 1 < n && book.codelist[lo + 1] <= word {
                    lo += 1;
                }
                while hi < n && word >= (book.codelist[hi] & mask) {
                    hi += 1;
                }
                let loval = lo.min(0x7fff) as u32;
                let hival = (n - hi).min(0x7fff) as u32;
                book.firsttable[at] = 0x8000_0000 | (loval << 15) | hival;
            }
        }
        Some(book)
    }

    /// Makes the value vectors (`_book_unquantize`, run by `vorbis_book_init_decode` at
    /// `vorbis_synthesis_init`) at the decoding thread's precision.
    pub fn prepare(&mut self, f: Fpu) {
        if let Some((s, sortindex)) = &self.quant {
            self.values = unquantize(s, self.used, sortindex, f);
        }
    }

    /// `decode_packed_entry_number`: the sorted index, or -1.
    fn decode_packed(&self, b: &mut Bits<'_>) -> i64 {
        let mut read = self.maxlength;
        let (mut lo, mut hi);
        let lok = b.look(self.firsttablen);
        if lok >= 0 {
            let entry = self.firsttable[lok as usize];
            if entry & 0x8000_0000 != 0 {
                lo = ((entry >> 15) & 0x7fff) as i64;
                hi = self.used as i64 - i64::from(entry & 0x7fff);
            } else {
                b.adv(self.dec_lengths[entry as usize - 1]);
                return i64::from(entry) - 1;
            }
        } else {
            lo = 0;
            hi = self.used as i64;
        }
        let mut lok = b.look(read);
        while lok < 0 && read > 1 {
            read -= 1;
            lok = b.look(read);
        }
        if lok < 0 {
            return -1;
        }
        let testword = (lok as u32).reverse_bits();
        while hi - lo > 1 {
            let p = (hi - lo) >> 1;
            let test = i64::from(self.codelist[(lo + p) as usize] > testword);
            lo += p & (test - 1);
            hi -= p & (-test);
        }
        if self.dec_lengths[lo as usize] <= read {
            b.adv(self.dec_lengths[lo as usize]);
            return lo;
        }
        b.adv(read);
        -1
    }

    /// `vorbis_book_decode`: the original entry number, or -1.
    pub fn decode(&self, b: &mut Bits<'_>) -> i64 {
        if self.used > 0 {
            let e = self.decode_packed(b);
            if e >= 0 {
                return i64::from(self.dec_index[e as usize]);
            }
        }
        -1
    }

    fn vector(&self, e: i64) -> &[f32] {
        let e = e as usize;
        &self.values[e * self.dim..(e + 1) * self.dim]
    }

    /// `vorbis_book_decodevs_add`: false at the end of the packet.
    pub fn decodevs_add(&self, a: &mut [f32], b: &mut Bits<'_>, n: usize) -> bool {
        if self.used > 0 {
            let step = n / self.dim;
            let mut entries = Vec::with_capacity(step);
            for _ in 0..step {
                let e = self.decode_packed(b);
                if e == -1 {
                    return false;
                }
                entries.push(e);
            }
            let mut o = 0;
            for i in 0..self.dim {
                for (j, &e) in entries.iter().enumerate() {
                    a[o + j] += self.vector(e)[i];
                }
                o += step;
            }
        }
        true
    }

    /// `vorbis_book_decodev_add`.
    pub fn decodev_add(&self, a: &mut [f32], b: &mut Bits<'_>, n: usize) -> bool {
        if self.used > 0 {
            let mut i = 0;
            while i < n {
                let e = self.decode_packed(b);
                if e == -1 {
                    return false;
                }
                for &t in self.vector(e) {
                    a[i] += t;
                    i += 1;
                }
            }
        }
        true
    }

    /// `vorbis_book_decodevv_add`: interleaved over the channels `a`.
    pub fn decodevv_add(
        &self,
        a: &mut [&mut [f32]],
        offset: usize,
        b: &mut Bits<'_>,
        n: usize,
    ) -> bool {
        let ch = a.len();
        if self.used > 0 {
            let mut chptr = 0;
            let mut i = offset / ch;
            while i < (offset + n) / ch {
                let e = self.decode_packed(b);
                if e == -1 {
                    return false;
                }
                for &t in self.vector(e) {
                    a[chptr][i] += t;
                    chptr += 1;
                    if chptr == ch {
                        chptr = 0;
                        i += 1;
                    }
                }
            }
        }
        true
    }
}
