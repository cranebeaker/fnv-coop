//! A small deflate compressor: greedy LZ77 matching with hash chains, coded
//! with the fixed Huffman tables from RFC 1951. Not as tight as zlib, but
//! renders and textures shrink several times over.

const WINDOW: usize = 32 * 1024;
const MIN_MATCH: usize = 3;
const MAX_MATCH: usize = 258;
const HASH_BITS: u32 = 15;
/// How many earlier positions with the same hash to try.
const MAX_CHAIN: usize = 48;

const LENGTH_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

struct BitWriter {
    out: Vec<u8>,
    bits: u64,
    count: u32,
}

impl BitWriter {
    /// Appends `n` bits of `value`, least significant first.
    fn put(&mut self, value: u32, n: u32) {
        self.bits |= u64::from(value) << self.count;
        self.count += n;
        while self.count >= 8 {
            self.out.push(self.bits as u8);
            self.bits >>= 8;
            self.count -= 8;
        }
    }

    /// Appends a Huffman code, which deflate stores most significant first.
    fn code(&mut self, code: u32, n: u32) {
        let reversed = code.reverse_bits() >> (32 - n);
        self.put(reversed, n);
    }

    fn finish(mut self) -> Vec<u8> {
        if self.count > 0 {
            self.out.push(self.bits as u8);
        }
        self.out
    }

    fn literal(&mut self, symbol: u16) {
        match symbol {
            0..=143 => self.code(0x30 + u32::from(symbol), 8),
            144..=255 => self.code(0x190 + u32::from(symbol - 144), 9),
            256..=279 => self.code(u32::from(symbol - 256), 7),
            _ => self.code(0xC0 + u32::from(symbol - 280), 8),
        }
    }

    fn matched(&mut self, length: usize, distance: usize) {
        let li = LENGTH_BASE
            .iter()
            .rposition(|&b| usize::from(b) <= length)
            .unwrap();
        self.literal(257 + li as u16);
        self.put(
            (length - usize::from(LENGTH_BASE[li])) as u32,
            u32::from(LENGTH_EXTRA[li]),
        );
        let di = DIST_BASE
            .iter()
            .rposition(|&b| usize::from(b) <= distance)
            .unwrap();
        self.code(di as u32, 5);
        self.put(
            (distance - usize::from(DIST_BASE[di])) as u32,
            u32::from(DIST_EXTRA[di]),
        );
    }
}

fn hash(data: &[u8], i: usize) -> usize {
    let v = u32::from(data[i]) | (u32::from(data[i + 1]) << 8) | (u32::from(data[i + 2]) << 16);
    (v.wrapping_mul(0x9E37_79B1) >> (32 - HASH_BITS)) as usize
}

/// Compresses `data` into a raw deflate stream (one fixed-Huffman block).
pub fn deflate(data: &[u8]) -> Vec<u8> {
    let mut w = BitWriter {
        out: Vec::with_capacity(data.len() / 3 + 16),
        bits: 0,
        count: 0,
    };
    w.put(1, 1); // final block
    w.put(1, 2); // fixed Huffman codes
    let mut head = vec![usize::MAX; 1 << HASH_BITS];
    let mut prev = vec![usize::MAX; WINDOW];
    let insert = |head: &mut Vec<usize>, prev: &mut Vec<usize>, i: usize| {
        if i + MIN_MATCH <= data.len() {
            let h = hash(data, i);
            prev[i % WINDOW] = head[h];
            head[h] = i;
        }
    };

    let mut i = 0;
    while i < data.len() {
        let mut best_len = 0;
        let mut best_dist = 0;
        if i + MIN_MATCH <= data.len() {
            let mut candidate = head[hash(data, i)];
            let limit = (data.len() - i).min(MAX_MATCH);
            let mut tries = 0;
            while candidate != usize::MAX && i - candidate <= WINDOW && tries < MAX_CHAIN {
                let len = data[candidate..]
                    .iter()
                    .zip(&data[i..i + limit])
                    .take_while(|(a, b)| a == b)
                    .count();
                if len > best_len {
                    best_len = len;
                    best_dist = i - candidate;
                    if len == limit {
                        break;
                    }
                }
                let next = prev[candidate % WINDOW];
                if next == usize::MAX || next >= candidate {
                    break;
                }
                candidate = next;
                tries += 1;
            }
        }
        if best_len >= MIN_MATCH {
            w.matched(best_len, best_dist);
            for k in i..i + best_len {
                insert(&mut head, &mut prev, k);
            }
            i += best_len;
        } else {
            w.literal(u16::from(data[i]));
            insert(&mut head, &mut prev, i);
            i += 1;
        }
    }
    w.literal(256); // end of block
    w.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny decoder for the fixed-Huffman subset this encoder writes. (The
    /// PNG tests also check the output with the full inflate in esm.)
    fn inflate_fixed(stream: &[u8]) -> Vec<u8> {
        struct Bits<'a> {
            data: &'a [u8],
            pos: usize,
        }
        impl Bits<'_> {
            fn bit(&mut self) -> u32 {
                let b = (self.data[self.pos / 8] >> (self.pos % 8)) & 1;
                self.pos += 1;
                u32::from(b)
            }
            fn bits(&mut self, n: u32) -> u32 {
                (0..n).fold(0, |acc, i| acc | (self.bit() << i))
            }
            fn code(&mut self, n: u32) -> u32 {
                (0..n).fold(0, |acc, _| (acc << 1) | self.bit())
            }
        }
        let mut r = Bits {
            data: stream,
            pos: 0,
        };
        assert_eq!(r.bits(1), 1);
        assert_eq!(r.bits(2), 1);
        let mut out: Vec<u8> = Vec::new();
        loop {
            // Read 7 bits, then extend to 8 or 9 as the fixed code demands.
            let mut c = r.code(7);
            let symbol = if c <= 0x17 {
                c + 256
            } else {
                c = (c << 1) | r.bit();
                if (0x30..=0xBF).contains(&c) {
                    c - 0x30
                } else if (0xC0..=0xC7).contains(&c) {
                    c - 0xC0 + 280
                } else {
                    c = (c << 1) | r.bit();
                    c - 0x190 + 144
                }
            };
            match symbol {
                0..=255 => out.push(symbol as u8),
                256 => break,
                _ => {
                    let li = (symbol - 257) as usize;
                    let len =
                        usize::from(LENGTH_BASE[li]) + r.bits(u32::from(LENGTH_EXTRA[li])) as usize;
                    let di = r.code(5) as usize;
                    let dist =
                        usize::from(DIST_BASE[di]) + r.bits(u32::from(DIST_EXTRA[di])) as usize;
                    for _ in 0..len {
                        out.push(out[out.len() - dist]);
                    }
                }
            }
        }
        out
    }

    #[test]
    fn round_trips_assorted_data() {
        let mut seed = 7u32;
        let mut random = Vec::new();
        for _ in 0..5000 {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            random.push((seed >> 24) as u8);
        }
        let long_run = vec![9u8; 100_000];
        let mut far_repeat = random.clone();
        far_repeat.extend(vec![0u8; 30_000]);
        far_repeat.extend(&random[..2000]);
        let text = b"the quick brown fox jumps over the lazy dog. ".repeat(50);
        for data in [
            Vec::new(),
            vec![1],
            vec![1, 2],
            random,
            long_run.clone(),
            far_repeat,
            text,
        ] {
            assert_eq!(inflate_fixed(&deflate(&data)), data);
        }
        assert!(deflate(&long_run).len() < 1000);
    }
}
