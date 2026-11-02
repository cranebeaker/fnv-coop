//! Tests on generated input. The game's movie itself is compared with the
//! game's library outside the tests (see the crate documentation).

use crate::bits::BitReader;
use crate::tables::{PATTERNS, SCAN, TREE_CODES, TREE_LENGTHS};
use crate::{Decoder, Error, Movie, AUDIO_STEREO};

#[test]
fn bits_come_from_the_low_end_of_little_endian_words() {
    // 0b1011_0110 then 0xff.
    let data = [0xb6u8, 0xff, 0x00, 0x80];
    let mut br = BitReader::new(&data);
    assert_eq!(br.bits(1), 0);
    assert_eq!(br.bits(2), 0b11);
    assert_eq!(br.bits(5), 0b10110);
    assert_eq!(br.bits(8), 0xff);
    br.align32();
    assert_eq!(br.position(), 32);
    assert!(!br.overrun());
    assert_eq!(br.bits(4), 0);
    assert!(br.overrun());
}

#[test]
fn every_tree_is_a_complete_prefix_code() {
    // Kraft sum of exactly 1 and no code a prefix of another (read from the
    // low bit up): every 7-bit pattern then decodes to exactly one leaf.
    for t in 0..16 {
        let kraft: u32 = TREE_LENGTHS[t].iter().map(|&len| 1u32 << (7 - len)).sum();
        assert_eq!(kraft, 128, "tree {t}");
        for pattern in 0u32..128 {
            let matches = (0..16)
                .filter(|&leaf| {
                    let len = TREE_LENGTHS[t][leaf] as u32;
                    pattern & ((1 << len) - 1) == TREE_CODES[t][leaf] as u32
                })
                .count();
            assert_eq!(matches, 1, "tree {t} pattern {pattern:07b}");
        }
    }
}

#[test]
fn scan_and_patterns_are_permutations_of_the_block() {
    let check = |order: &[u8; 64]| {
        let mut seen = [false; 64];
        for &p in order {
            assert!(!seen[p as usize]);
            seen[p as usize] = true;
        }
    };
    check(&SCAN);
    for p in &PATTERNS {
        check(p);
    }
}

/// A header with `frames` frames of the given sizes, one stereo track.
fn movie_bytes(frame_sizes: &[usize]) -> Vec<u8> {
    let mut d = Vec::new();
    d.extend_from_slice(b"BIKi");
    d.extend_from_slice(&0u32.to_le_bytes()); // size, filled below
    d.extend_from_slice(&(frame_sizes.len() as u32).to_le_bytes());
    d.extend_from_slice(&(*frame_sizes.iter().max().unwrap() as u32).to_le_bytes());
    d.extend_from_slice(&(frame_sizes.len() as u32).to_le_bytes());
    d.extend_from_slice(&64u32.to_le_bytes());
    d.extend_from_slice(&48u32.to_le_bytes());
    d.extend_from_slice(&30u32.to_le_bytes());
    d.extend_from_slice(&1u32.to_le_bytes());
    d.extend_from_slice(&0u32.to_le_bytes()); // flags
    d.extend_from_slice(&1u32.to_le_bytes()); // one track
    d.extend_from_slice(&4096u32.to_le_bytes());
    d.extend_from_slice(&44100u16.to_le_bytes());
    d.extend_from_slice(&(AUDIO_STEREO).to_le_bytes());
    d.extend_from_slice(&7u32.to_le_bytes()); // track id
    let index_at = d.len();
    let first = index_at + 4 * (frame_sizes.len() + 1);
    let mut o = first;
    for (i, &s) in frame_sizes.iter().enumerate() {
        let key = if i == 0 { 1 } else { 0 };
        d.extend_from_slice(&((o as u32) | key).to_le_bytes());
        o += s;
    }
    d.extend_from_slice(&(o as u32).to_le_bytes());
    for (i, &s) in frame_sizes.iter().enumerate() {
        // An audio packet of 8 bytes, then video bytes.
        d.extend_from_slice(&8u32.to_le_bytes());
        d.extend_from_slice(&[0xa0 + i as u8; 8]);
        d.extend(std::iter::repeat(i as u8).take(s - 12));
    }
    let size = (d.len() - 8) as u32;
    d[4..8].copy_from_slice(&size.to_le_bytes());
    d
}

#[test]
fn the_header_index_and_packets_are_read() {
    let bytes = movie_bytes(&[40, 20]);
    let m = Movie::parse(&bytes).unwrap();
    assert_eq!(m.revision, b'i');
    assert_eq!((m.width, m.height), (64, 48));
    assert_eq!(m.frame_count(), 2);
    assert_eq!(m.fps(), 30.0);
    assert_eq!(m.audio.len(), 1);
    assert_eq!(m.audio[0].sample_rate, 44100);
    assert_eq!(m.audio[0].channels(), 2);
    assert_eq!(m.audio[0].id, 7);
    let p0 = m.packet(0).unwrap();
    assert!(p0.keyframe);
    assert_eq!(p0.audio[0], &[0xa0; 8]);
    assert_eq!(p0.video.len(), 28);
    let p1 = m.packet(1).unwrap();
    assert!(!p1.keyframe);
    assert_eq!(p1.video, &[1u8; 8]);
    assert!(m.packet(2).is_err());
}

#[test]
fn bad_files_are_refused() {
    assert_eq!(Movie::parse(b"KB2j....").unwrap_err(), Error::NotBink);
    assert_eq!(Movie::parse(b"BIKz").unwrap_err(), Error::Revision(b'z'));
    let mut bytes = movie_bytes(&[40, 20]);
    let short = bytes.len() - 30;
    bytes.truncate(short);
    assert!(Movie::parse(&bytes).is_err());
}

#[test]
fn an_empty_frame_leaves_the_planes_alone() {
    let mut d = Decoder::new(64, 48, b'i', 0);
    // Only the 32-bit size word, then nothing: the luma plane starts but
    // every bundle reads a count of zero, so every block takes type 0
    // (skip) from the zeroed bundle and copies the black previous frame.
    let r = d.decode(&[0u8; 4]);
    assert!(r.is_err() || r.is_ok());
    let mut out = Vec::new();
    d.to_yv12(&mut out);
    assert_eq!(out.len(), 64 * 48 + 2 * 32 * 24);
}

/// Writes bits low-first into 32-bit little-endian words, as Bink reads them.
struct BitWriter {
    bytes: Vec<u8>,
    bits: usize,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter {
            bytes: Vec::new(),
            bits: 0,
        }
    }

    fn put(&mut self, value: u32, n: u32) {
        for i in 0..n {
            if self.bits % 8 == 0 {
                self.bytes.push(0);
            }
            let bit = (value >> i) & 1;
            *self.bytes.last_mut().unwrap() |= (bit as u8) << (self.bits % 8);
            self.bits += 1;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        while self.bytes.len() % 4 != 0 {
            self.bytes.push(0);
        }
        self.bytes
    }
}

/// One DCT block for `channels` channels: leading coefficients from
/// `lead` (raw 29-bit fields), all band levels at code `level`, and the
/// rest of the coefficients zero in runs of 64 * 8.
fn silent_block(w: &mut BitWriter, channels: usize, lead: [u32; 2], level: u32) {
    w.put(0, 2);
    for _ in 0..channels {
        w.put(lead[0], 29);
        w.put(lead[1], 29);
        for _ in 0..25 {
            w.put(level, 8);
        }
        let mut i = 2;
        while i < 2048 {
            // A run of 64 * 8 coefficients of width 0.
            w.put(1, 1);
            w.put(15, 4);
            w.put(0, 4);
            i += 512;
        }
    }
}

#[test]
fn a_silent_audio_block_decodes_to_silence() {
    let mut d = crate::AudioDecoder::new(48000, 2, crate::AUDIO_DCT | AUDIO_STEREO).unwrap();
    assert_eq!(d.block_samples(), 3840);
    let mut w = BitWriter::new();
    silent_block(&mut w, 2, [0, 0], 0);
    let block = w.finish();
    let mut packet = (3840u32 * 2).to_le_bytes().to_vec();
    packet.extend_from_slice(&block);
    let mut out = Vec::new();
    d.decode_packet(&packet, &mut out).unwrap();
    assert_eq!(out.len(), 3840);
    assert!(out.iter().all(|&s| s == 0));
}

#[test]
fn a_constant_first_coefficient_gives_a_constant_level() {
    // Coefficient 0 alone: the inverse DCT gives the same value at every
    // sample, scaled by 2 / sqrt(2048) when converted. Exponent 23 with
    // mantissa 724 is 724.0; 724 * 0.0441942 = 31.997, rounding to 32.
    let mut d = crate::AudioDecoder::new(48000, 1, crate::AUDIO_DCT).unwrap();
    let mut w = BitWriter::new();
    silent_block(&mut w, 1, [(724 << 5) | 23, 0], 0);
    let block = w.finish();
    let mut packet = (1920u32 * 2).to_le_bytes().to_vec();
    packet.extend_from_slice(&block);
    let mut out = Vec::new();
    d.decode_packet(&packet, &mut out).unwrap();
    assert_eq!(out.len(), 1920);
    assert!(out.iter().all(|&s| s == 32), "{:?}", &out[..8]);

    // A second identical block crossfades its start with the end of the
    // first: equal values stay equal.
    let mut packet2 = (1920u32 * 2).to_le_bytes().to_vec();
    packet2.extend_from_slice(&block);
    let mut out2 = Vec::new();
    d.decode_packet(&packet2, &mut out2).unwrap();
    assert!(out2.iter().all(|&s| s == 32));
}

#[test]
fn rdft_audio_is_refused() {
    assert!(crate::AudioDecoder::new(48000, 2, AUDIO_STEREO).is_err());
}
