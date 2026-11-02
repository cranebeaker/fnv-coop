//! Text decoding. Strings in Fallout 3 / New Vegas plugins are
//! NUL-terminated and encoded as Windows-1252.

/// Unicode mappings for Windows-1252 bytes 0x80..=0x9F. The five bytes that
/// Windows-1252 leaves undefined map to the matching C1 control character,
/// as Windows itself does.
const CP1252_HIGH: [char; 32] = [
    '\u{20AC}', '\u{0081}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
    '\u{02C6}', '\u{2030}', '\u{0160}', '\u{2039}', '\u{0152}', '\u{008D}', '\u{017D}', '\u{008F}',
    '\u{0090}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{02DC}', '\u{2122}', '\u{0161}', '\u{203A}', '\u{0153}', '\u{009D}', '\u{017E}', '\u{0178}',
];

/// Decodes Windows-1252 bytes to a Rust string.
pub fn decode_cp1252(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| match b {
            0x80..=0x9F => CP1252_HIGH[usize::from(b - 0x80)],
            // 0x00..=0x7F is ASCII and 0xA0..=0xFF matches Latin-1.
            _ => char::from(b),
        })
        .collect()
}

/// Decodes a NUL-terminated string, ignoring anything after the first NUL.
pub fn zstring(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    decode_cp1252(&bytes[..end])
}

/// Heuristic used for display: true when the bytes look like a single
/// NUL-terminated (or unterminated) text string.
pub fn looks_like_text(bytes: &[u8]) -> bool {
    let body = match bytes.split_last() {
        Some((0, rest)) => rest,
        _ => bytes,
    };
    !body.is_empty()
        && body
            .iter()
            .all(|&b| (b >= 0x20 && b != 0x7F) || matches!(b, b'\t' | b'\n' | b'\r'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_windows_1252_punctuation() {
        assert_eq!(
            decode_cp1252(b"It\x92s \x93fine\x94 \x96 really\x85"),
            "It’s “fine” – really…"
        );
        assert_eq!(decode_cp1252(b"caf\xe9"), "café");
    }

    #[test]
    fn zstring_stops_at_nul() {
        assert_eq!(zstring(b"Rifle\0junk"), "Rifle");
        assert_eq!(zstring(b"NoTerminator"), "NoTerminator");
        assert_eq!(zstring(b"\0"), "");
    }

    #[test]
    fn text_heuristic() {
        assert!(looks_like_text(b"Hello\0"));
        assert!(looks_like_text(b"Line one\r\nLine two"));
        assert!(!looks_like_text(b"\0"));
        assert!(!looks_like_text(b"\x01\x02\x03\x04"));
        assert!(!looks_like_text(b"ab\0cd\0"));
    }
}
