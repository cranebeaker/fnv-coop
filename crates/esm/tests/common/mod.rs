//! Builders for the plugin format, shared by the integration tests.
#![allow(dead_code)]

use esm::flags;

// ---------------------------------------------------------------------------
// Builders for the on-disk format
// ---------------------------------------------------------------------------

pub fn sub(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let size = u16::try_from(data.len()).expect("use big_sub for >64 KiB");
    let mut v = kind.to_vec();
    v.extend(size.to_le_bytes());
    v.extend(data);
    v
}

/// A subrecord larger than 64 KiB, written with an XXXX size prefix.
pub fn big_sub(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut v = b"XXXX".to_vec();
    v.extend(4u16.to_le_bytes());
    v.extend((data.len() as u32).to_le_bytes());
    v.extend(kind);
    v.extend(0u16.to_le_bytes());
    v.extend(data);
    v
}

pub fn zstr(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}

pub fn record(kind: &[u8; 4], form_id: u32, flags: u32, data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u32).to_le_bytes());
    v.extend(flags.to_le_bytes());
    v.extend(form_id.to_le_bytes());
    v.extend(0u32.to_le_bytes()); // revision
    v.extend(15u16.to_le_bytes()); // version
    v.extend(0u16.to_le_bytes());
    v.extend(data);
    v
}

pub fn group(label: [u8; 4], group_type: i32, contents: &[u8]) -> Vec<u8> {
    let mut v = b"GRUP".to_vec();
    v.extend(((24 + contents.len()) as u32).to_le_bytes());
    v.extend(label);
    v.extend(group_type.to_le_bytes());
    v.extend([0u8; 8]); // stamp, unknown, version, unknown
    v.extend(contents);
    v
}

pub fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in data {
        a = (a + u32::from(x)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// A valid zlib stream using uncompressed ("stored") DEFLATE blocks.
pub fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut v = vec![0x78, 0x01];
    let chunks: Vec<&[u8]> = if data.is_empty() {
        vec![&[]]
    } else {
        data.chunks(65_535).collect()
    };
    for (i, chunk) in chunks.iter().enumerate() {
        v.push(u8::from(i == chunks.len() - 1)); // BFINAL, BTYPE = 00
        let len = chunk.len() as u16;
        v.extend(len.to_le_bytes());
        v.extend((!len).to_le_bytes());
        v.extend(*chunk);
    }
    v.extend(adler32(data).to_be_bytes());
    v
}

pub fn compressed_record(kind: &[u8; 4], form_id: u32, flags: u32, subrecords: &[u8]) -> Vec<u8> {
    let mut data = (subrecords.len() as u32).to_le_bytes().to_vec();
    data.extend(zlib_stored(subrecords));
    record(kind, form_id, flags | flags::COMPRESSED, &data)
}

pub fn tes4(record_flags: u32, masters: &[&str], author: &str) -> Vec<u8> {
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend(7i32.to_le_bytes());
    hedr.extend(0x800u32.to_le_bytes());
    let mut data = sub(b"HEDR", &hedr);
    data.extend(sub(b"CNAM", &zstr(author)));
    data.extend(sub(b"SNAM", &zstr("Test plugin")));
    for m in masters {
        data.extend(sub(b"MAST", &zstr(m)));
        data.extend(sub(b"DATA", &0u64.to_le_bytes()));
    }
    record(b"TES4", 0, record_flags, &data)
}

pub fn weapon_data(value: i32, health: i32, weight: f32, damage: i16, clip: u8) -> Vec<u8> {
    let mut d = value.to_le_bytes().to_vec();
    d.extend(health.to_le_bytes());
    d.extend(weight.to_le_bytes());
    d.extend(damage.to_le_bytes());
    d.push(clip);
    d
}
