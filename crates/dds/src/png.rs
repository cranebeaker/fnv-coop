//! A minimal PNG writer (8-bit RGBA) with no dependencies. Rows are
//! filtered and compressed with the small deflate encoder in this crate.

use std::io::{self, Write};

fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for (n, entry) in table.iter_mut().enumerate() {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 {
                0xEDB8_8320 ^ (c >> 1)
            } else {
                c >> 1
            };
        }
        *entry = c;
    }
    table
}

fn crc32(table: &[u32; 256], parts: &[&[u8]]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for part in parts {
        for &b in *part {
            c = table[((c ^ u32::from(b)) & 0xFF) as usize] ^ (c >> 8);
        }
    }
    c ^ 0xFFFF_FFFF
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in data.chunks(5552) {
        for &x in chunk {
            a += u32::from(x);
            b += a;
        }
        a %= 65_521;
        b %= 65_521;
    }
    (b << 16) | a
}

fn chunk(out: &mut impl Write, table: &[u32; 256], kind: &[u8; 4], data: &[u8]) -> io::Result<()> {
    out.write_all(&(data.len() as u32).to_be_bytes())?;
    out.write_all(kind)?;
    out.write_all(data)?;
    out.write_all(&crc32(table, &[kind, data]).to_be_bytes())
}

/// Writes `rgba` (4 bytes per pixel, rows top to bottom) as a PNG.
pub fn write_rgba(out: &mut impl Write, width: u32, height: u32, rgba: &[u8]) -> io::Result<()> {
    let row = width as usize * 4;
    if rgba.len() != row * height as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "pixel data doesn't match the image size",
        ));
    }
    let table = crc32_table();
    out.write_all(b"\x89PNG\r\n\x1a\n")?;

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend(width.to_be_bytes());
    ihdr.extend(height.to_be_bytes());
    ihdr.extend([8, 6, 0, 0, 0]); // 8-bit, RGBA, deflate, no filter, no interlace
    chunk(out, &table, b"IHDR", &ihdr)?;

    let raw = filter_rows(rgba, row);
    let mut zlib = vec![0x78, 0x01];
    zlib.extend(crate::deflate::deflate(&raw));
    zlib.extend(adler32(&raw).to_be_bytes());
    chunk(out, &table, b"IDAT", &zlib)?;
    chunk(out, &table, b"IEND", &[])
}

/// Prefixes each row with the PNG filter that makes it smallest by the
/// usual heuristic (least sum of absolute differences).
fn filter_rows(rgba: &[u8], row: usize) -> Vec<u8> {
    const BPP: usize = 4;
    let mut raw = Vec::with_capacity(rgba.len() + rgba.len() / row.max(1) + 1);
    if row == 0 {
        return raw;
    }
    let zeros = vec![0u8; row];
    let mut candidate = vec![0u8; row];
    let mut best = vec![0u8; row];
    for (y, line) in rgba.chunks_exact(row).enumerate() {
        let above: &[u8] = if y == 0 {
            &zeros
        } else {
            &rgba[(y - 1) * row..y * row]
        };
        let mut best_score = u64::MAX;
        let mut best_type = 0u8;
        for kind in 0..5u8 {
            for i in 0..row {
                let a = if i >= BPP { line[i - BPP] } else { 0 };
                let b = above[i];
                let c = if i >= BPP { above[i - BPP] } else { 0 };
                let predicted = match kind {
                    0 => 0,
                    1 => a,
                    2 => b,
                    3 => ((u16::from(a) + u16::from(b)) / 2) as u8,
                    _ => paeth(a, b, c),
                };
                candidate[i] = line[i].wrapping_sub(predicted);
            }
            let score: u64 = candidate
                .iter()
                .map(|&v| u64::from((v as i8).unsigned_abs()))
                .sum();
            if score < best_score {
                best_score = score;
                best_type = kind;
                best.copy_from_slice(&candidate);
            }
        }
        raw.push(best_type);
        raw.extend(&best);
    }
    raw
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = i16::from(a) + i16::from(b) - i16::from(c);
    let (pa, pb, pc) = (
        (p - i16::from(a)).abs(),
        (p - i16::from(b)).abs(),
        (p - i16::from(c)).abs(),
    );
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_matches_the_png_specification_example() {
        // CRC of an empty IEND chunk, as found at the end of every PNG.
        assert_eq!(crc32(&crc32_table(), &[b"IEND"]), 0xAE42_6082);
    }
}
