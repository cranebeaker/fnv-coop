//! A self-contained zlib (RFC 1950) / DEFLATE (RFC 1951) decoder.
//!
//! Compressed records in Fallout 3 / New Vegas plugins store their subrecords
//! as a standard zlib stream. This decoder covers everything the format
//! allows (stored, fixed-Huffman and dynamic-Huffman blocks) and verifies the
//! Adler-32 checksum, so the crate needs no external dependencies.

const MAX_BITS: usize = 15;
/// Codes up to this many bits long are decoded with a single table lookup;
/// longer codes fall back to a canonical bit-by-bit walk.
const FAST_BITS: u32 = 10;

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
/// Order in which code-length code lengths are stored in a dynamic block.
const CLEN_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// Decompresses a zlib stream. `size_hint` pre-sizes the output buffer.
/// Errors are returned as human-readable strings.
pub fn zlib_decompress(input: &[u8], size_hint: Option<usize>) -> Result<Vec<u8>, String> {
    let (out, checksum) = zlib_decompress_unverified(input, size_hint)?;
    checksum?;
    Ok(out)
}

/// Decompresses a zlib stream, reporting a wrong Adler-32 checksum
/// separately (as the inner `Err`) instead of failing: the game doesn't
/// check it, and `FalloutNV.esm` itself has a record whose checksum is
/// wrong (the `LAND` record 00150FC0; Windows' own decompressor gives the
/// same bytes and the same mismatch).
pub fn zlib_decompress_unverified(
    input: &[u8],
    size_hint: Option<usize>,
) -> Result<(Vec<u8>, Result<(), String>), String> {
    if input.len() < 2 {
        return Err("zlib stream is shorter than its 2-byte header".into());
    }
    let (cmf, flg) = (input[0], input[1]);
    if cmf & 0x0F != 8 {
        return Err(format!(
            "unsupported zlib compression method {}",
            cmf & 0x0F
        ));
    }
    if cmf >> 4 > 7 {
        return Err(format!("invalid zlib window size {}", cmf >> 4));
    }
    if (u16::from(cmf) << 8 | u16::from(flg)) % 31 != 0 {
        return Err("zlib header checksum mismatch".into());
    }
    if flg & 0x20 != 0 {
        return Err("zlib preset dictionaries are not supported".into());
    }

    let mut out = Vec::with_capacity(size_hint.unwrap_or(input.len() * 4));
    let mut bits = BitReader::new(&input[2..]);
    inflate(&mut bits, &mut out)?;

    let trailer_start = 2 + bits.byte_position();
    let trailer = input
        .get(trailer_start..trailer_start + 4)
        .ok_or("zlib stream is missing its Adler-32 checksum")?;
    let expected = u32::from_be_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
    let actual = adler32(&out);
    let checksum = if expected == actual {
        Ok(())
    } else {
        Err(format!(
            "Adler-32 mismatch (stored {expected:08x}, computed {actual:08x})"
        ))
    };
    Ok((out, checksum))
}

/// Adler-32 checksum as used by zlib.
pub fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65_521;
    // 5552 is the largest block size for which the sums cannot overflow u32.
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in data.chunks(5552) {
        for &byte in chunk {
            a += u32::from(byte);
            b += a;
        }
        a %= MOD;
        b %= MOD;
    }
    (b << 16) | a
}

/// Reads bits least-significant first, as DEFLATE requires.
struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    buf: u64,
    count: u32,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            buf: 0,
            count: 0,
        }
    }

    /// Loads as many whole bytes as fit into the buffer.
    fn refill(&mut self) {
        while self.count <= 56 && self.pos < self.data.len() {
            self.buf |= u64::from(self.data[self.pos]) << self.count;
            self.pos += 1;
            self.count += 8;
        }
    }

    fn bits(&mut self, n: u32) -> Result<u32, String> {
        debug_assert!(n <= 32);
        if n == 0 {
            return Ok(0);
        }
        if self.count < n {
            self.refill();
            if self.count < n {
                return Err("compressed data ended unexpectedly".into());
            }
        }
        let value = (self.buf & ((1u64 << n) - 1)) as u32;
        self.buf >>= n;
        self.count -= n;
        Ok(value)
    }

    /// Discards bits up to the next byte boundary.
    fn align_to_byte(&mut self) {
        let drop = self.count % 8;
        self.buf >>= drop;
        self.count -= drop;
    }

    /// Byte offset of the first byte not yet consumed, after aligning.
    fn byte_position(&mut self) -> usize {
        self.align_to_byte();
        self.pos - (self.count / 8) as usize
    }
}

/// Canonical Huffman decoding table.
struct Huffman {
    /// Number of codes of each length (index 0 unused).
    counts: [u16; MAX_BITS + 1],
    /// Symbols ordered by code length, then by symbol value.
    symbols: Vec<u16>,
    /// Lookup table indexed by the next FAST_BITS input bits (bit-reversed
    /// codes). Each entry is `symbol << 4 | length`, or 0 if no code of
    /// length <= FAST_BITS matches.
    fast: Vec<u16>,
}

impl Huffman {
    fn new(lengths: &[u8]) -> Result<Self, String> {
        let mut counts = [0u16; MAX_BITS + 1];
        for &len in lengths {
            counts[usize::from(len)] += 1;
        }
        counts[0] = 0;

        // Reject over-subscribed code sets. Incomplete sets are allowed
        // (DEFLATE permits them, e.g. a distance tree with a single code);
        // reading an unassigned code is reported as an error when decoding.
        let mut left: i32 = 1;
        for &count in &counts[1..] {
            left = (left << 1) - i32::from(count);
            if left < 0 {
                return Err("invalid Huffman code lengths (over-subscribed)".into());
            }
        }

        let mut offsets = [0u16; MAX_BITS + 2];
        for len in 1..=MAX_BITS {
            offsets[len + 1] = offsets[len] + counts[len];
        }
        let mut symbols = vec![0u16; usize::from(offsets[MAX_BITS + 1])];
        for (symbol, &len) in lengths.iter().enumerate() {
            if len != 0 {
                let slot = &mut offsets[usize::from(len)];
                symbols[usize::from(*slot)] = symbol as u16;
                *slot += 1;
            }
        }

        let mut fast = vec![0u16; 1 << FAST_BITS];
        let mut next_code = [0u32; MAX_BITS + 1];
        let mut code = 0u32;
        for len in 1..=MAX_BITS {
            code = (code + u32::from(counts[len - 1])) << 1;
            next_code[len] = code;
        }
        for (symbol, &len) in lengths.iter().enumerate() {
            let len = u32::from(len);
            if len == 0 {
                continue;
            }
            let code = next_code[len as usize];
            next_code[len as usize] += 1;
            if len <= FAST_BITS {
                let reversed = code.reverse_bits() >> (32 - len);
                let entry = (symbol as u16) << 4 | len as u16;
                let mut index = reversed as usize;
                while index < fast.len() {
                    fast[index] = entry;
                    index += 1 << len;
                }
            }
        }

        Ok(Self {
            counts,
            symbols,
            fast,
        })
    }

    fn decode(&self, bits: &mut BitReader) -> Result<u16, String> {
        if bits.count < FAST_BITS {
            bits.refill();
        }
        let entry = self.fast[(bits.buf & ((1 << FAST_BITS) - 1)) as usize];
        if entry != 0 {
            let len = u32::from(entry & 0xF);
            if len <= bits.count {
                bits.buf >>= len;
                bits.count -= len;
                return Ok(entry >> 4);
            }
        }

        // Slow path: walk the canonical code one bit at a time.
        let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
        for len in 1..=MAX_BITS {
            code |= bits.bits(1)? as i32;
            let count = i32::from(self.counts[len]);
            if code - first < count {
                return Ok(self.symbols[(index + code - first) as usize]);
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err("invalid Huffman code in compressed data".into())
    }
}

fn inflate(bits: &mut BitReader, out: &mut Vec<u8>) -> Result<(), String> {
    loop {
        let is_final = bits.bits(1)? == 1;
        match bits.bits(2)? {
            0 => stored_block(bits, out)?,
            1 => {
                let (litlen, dist) = fixed_tables()?;
                compressed_block(bits, out, &litlen, &dist)?;
            }
            2 => {
                let (litlen, dist) = dynamic_tables(bits)?;
                compressed_block(bits, out, &litlen, &dist)?;
            }
            _ => return Err("invalid DEFLATE block type 3".into()),
        }
        if is_final {
            return Ok(());
        }
    }
}

fn stored_block(bits: &mut BitReader, out: &mut Vec<u8>) -> Result<(), String> {
    bits.align_to_byte();
    let len = bits.bits(16)?;
    let nlen = bits.bits(16)?;
    if len != !nlen & 0xFFFF {
        return Err("stored block length check failed".into());
    }
    out.reserve(len as usize);
    for _ in 0..len {
        out.push(bits.bits(8)? as u8);
    }
    Ok(())
}

fn fixed_tables() -> Result<(Huffman, Huffman), String> {
    let mut lengths = [0u8; 288];
    lengths[..144].fill(8);
    lengths[144..256].fill(9);
    lengths[256..280].fill(7);
    lengths[280..].fill(8);
    Ok((Huffman::new(&lengths)?, Huffman::new(&[5u8; 30])?))
}

fn dynamic_tables(bits: &mut BitReader) -> Result<(Huffman, Huffman), String> {
    let hlit = bits.bits(5)? as usize + 257;
    let hdist = bits.bits(5)? as usize + 1;
    let hclen = bits.bits(4)? as usize + 4;
    if hlit > 286 || hdist > 30 {
        return Err("dynamic block declares too many codes".into());
    }

    let mut clen_lengths = [0u8; 19];
    for &slot in &CLEN_ORDER[..hclen] {
        clen_lengths[slot] = bits.bits(3)? as u8;
    }
    let clen = Huffman::new(&clen_lengths)?;

    let mut lengths = vec![0u8; hlit + hdist];
    let mut i = 0;
    while i < lengths.len() {
        let symbol = clen.decode(bits)?;
        let (value, repeat) = match symbol {
            0..=15 => (symbol as u8, 1),
            16 => {
                let previous = *i
                    .checked_sub(1)
                    .and_then(|p| lengths.get(p))
                    .ok_or("repeat code with no previous length")?;
                (previous, 3 + bits.bits(2)? as usize)
            }
            17 => (0, 3 + bits.bits(3)? as usize),
            18 => (0, 11 + bits.bits(7)? as usize),
            _ => return Err("invalid code-length symbol".into()),
        };
        if i + repeat > lengths.len() {
            return Err("code lengths overflow the declared table size".into());
        }
        lengths[i..i + repeat].fill(value);
        i += repeat;
    }
    if lengths[256] == 0 {
        return Err("dynamic block has no end-of-block code".into());
    }
    Ok((
        Huffman::new(&lengths[..hlit])?,
        Huffman::new(&lengths[hlit..])?,
    ))
}

fn compressed_block(
    bits: &mut BitReader,
    out: &mut Vec<u8>,
    litlen: &Huffman,
    dist: &Huffman,
) -> Result<(), String> {
    loop {
        let symbol = litlen.decode(bits)?;
        match symbol {
            0..=255 => out.push(symbol as u8),
            256 => return Ok(()),
            257..=285 => {
                let i = usize::from(symbol - 257);
                let length =
                    usize::from(LENGTH_BASE[i]) + bits.bits(u32::from(LENGTH_EXTRA[i]))? as usize;
                let d = usize::from(dist.decode(bits)?);
                if d >= DIST_BASE.len() {
                    return Err("invalid distance symbol".into());
                }
                let distance =
                    usize::from(DIST_BASE[d]) + bits.bits(u32::from(DIST_EXTRA[d]))? as usize;
                if distance > out.len() {
                    return Err("back-reference points before the start of the output".into());
                }
                let start = out.len() - distance;
                if distance >= length {
                    out.extend_from_within(start..start + length);
                } else {
                    // Overlapping copy: each byte may depend on one just written.
                    for k in 0..length {
                        let byte = out[start + k];
                        out.push(byte);
                    }
                }
            }
            _ => return Err("invalid literal/length symbol".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unhex(s: &str) -> Vec<u8> {
        let s: String = s.split_whitespace().collect();
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn adler32_matches_reference_values() {
        assert_eq!(adler32(b""), 1);
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
    }

    // Vectors below were produced with Python's zlib module (see the
    // generator comment on each).

    /// zlib.compress(b"hello, wasteland! " * 3, 9) -- fixed Huffman block.
    const FIXED: &str = include_str!("../testdata/fixed.hex");
    /// zlib.compress(b"", 9)
    const EMPTY: &str = include_str!("../testdata/empty.hex");
    /// zlib.compress(b"stored block test", 0) -- stored block.
    const STORED: &str = include_str!("../testdata/stored.hex");
    /// zlib.compress(dynamic_text(), 9) -- dynamic Huffman block(s).
    const DYNAMIC: &str = include_str!("../testdata/dynamic.hex");

    /// The same text the generator used for the dynamic vector.
    fn dynamic_text() -> Vec<u8> {
        (0..400)
            .map(|i| format!("record {i} weapon {} damage {}\n", i * 7 % 13, i * i % 97))
            .collect::<String>()
            .into_bytes()
    }

    #[test]
    fn decodes_fixed_huffman() {
        let out = zlib_decompress(&unhex(FIXED), None).unwrap();
        assert_eq!(out, b"hello, wasteland! ".repeat(3));
    }

    #[test]
    fn decodes_empty_stream() {
        assert_eq!(zlib_decompress(&unhex(EMPTY), None).unwrap(), b"");
    }

    #[test]
    fn decodes_stored_block() {
        assert_eq!(
            zlib_decompress(&unhex(STORED), None).unwrap(),
            b"stored block test"
        );
    }

    #[test]
    fn decodes_dynamic_huffman() {
        let out = zlib_decompress(&unhex(DYNAMIC), Some(1)).unwrap();
        assert_eq!(out, dynamic_text());
    }

    #[test]
    fn rejects_corrupt_checksum() {
        let mut data = unhex(FIXED);
        let last = data.len() - 1;
        data[last] ^= 0xFF;
        let err = zlib_decompress(&data, None).unwrap_err();
        assert!(err.contains("Adler-32"), "{err}");
    }

    #[test]
    fn rejects_bad_header_and_truncation() {
        assert!(zlib_decompress(&[0x78], None).is_err());
        assert!(zlib_decompress(&[0x79, 0x9C], None).is_err());
        let data = unhex(DYNAMIC);
        assert!(zlib_decompress(&data[..data.len() / 2], None).is_err());
    }
}
