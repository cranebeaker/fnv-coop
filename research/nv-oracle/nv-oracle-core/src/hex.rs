//! Hex and integer text helpers.

/// Lower-case hex of `bytes`, first byte first (memory order).
pub fn encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(DIGITS[(b >> 4) as usize] as char);
        out.push(DIGITS[(b & 15) as usize] as char);
    }
    out
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// Decode a strict hex string (even length, no separators).
pub fn decode(text: &str) -> Result<Vec<u8>, String> {
    let bytes = text.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err(format!("hex string has an odd number of digits: {text:?}"));
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks(2) {
        match (nibble(pair[0]), nibble(pair[1])) {
            (Some(hi), Some(lo)) => out.push((hi << 4) | lo),
            _ => return Err(format!("invalid hex digit in {text:?}")),
        }
    }
    Ok(out)
}

/// Like [`decode`], but ignores spaces, commas, colons and dashes so bytes
/// can be pasted from a disassembler (`55 8B EC` or `55:8b:ec`).
pub fn decode_lenient(text: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = text
        .chars()
        .filter(|c| !c.is_ascii_whitespace() && !matches!(c, ',' | ':' | '-'))
        .collect();
    decode(&cleaned)
}

/// Parse an unsigned integer written as decimal or `0x` hex.
pub fn parse_u64(text: &str) -> Result<u64, String> {
    let t = text.trim();
    let parsed = if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u64::from_str_radix(h, 16)
    } else {
        t.parse::<u64>()
    };
    parsed.map_err(|_| format!("not a number: {text:?}"))
}

/// Parse a 32-bit value: decimal, `0x` hex, or a negative decimal that
/// wraps (so `-1` is `0xFFFFFFFF`, as an `i32` argument would be).
pub fn parse_u32(text: &str) -> Result<u32, String> {
    let t = text.trim();
    if let Some(neg) = t.strip_prefix('-') {
        let v = parse_u64(neg)?;
        if v > 0x8000_0000 {
            return Err(format!("negative value out of 32-bit range: {text:?}"));
        }
        return Ok((v as u32).wrapping_neg());
    }
    let v = parse_u64(t)?;
    u32::try_from(v).map_err(|_| format!("value out of 32-bit range: {text:?}"))
}

/// Parse a signed offset such as `+0x10`, `-8` or `4`.
pub fn parse_i64(text: &str) -> Result<i64, String> {
    let t = text.trim();
    let (neg, digits) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let v = parse_u64(digits)?;
    let v = i64::try_from(v).map_err(|_| format!("offset out of range: {text:?}"))?;
    Ok(if neg { -v } else { v })
}

/// `0x` followed by 8 upper-case hex digits.
pub fn fmt_u32(v: u32) -> String {
    format!("0x{v:08X}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_round_trip() {
        let data = [0x00, 0x7f, 0x80, 0xff, 0x12];
        assert_eq!(encode(&data), "007f80ff12");
        assert_eq!(decode("007F80ff12").unwrap(), data);
    }

    #[test]
    fn decode_rejects_bad_input() {
        assert!(decode("abc").is_err());
        assert!(decode("zz").is_err());
        assert_eq!(decode("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn lenient_decode_accepts_separators() {
        assert_eq!(decode_lenient("55 8B EC").unwrap(), [0x55, 0x8b, 0xec]);
        assert_eq!(
            decode_lenient("55:8b-ec,83").unwrap(),
            [0x55, 0x8b, 0xec, 0x83]
        );
        assert!(decode_lenient("55 8").is_err());
    }

    #[test]
    fn integer_parsing() {
        assert_eq!(parse_u32("0x00401000").unwrap(), 0x0040_1000);
        assert_eq!(parse_u32("42").unwrap(), 42);
        assert_eq!(parse_u32("-1").unwrap(), 0xFFFF_FFFF);
        assert_eq!(parse_u32("-2147483648").unwrap(), 0x8000_0000);
        assert!(parse_u32("4294967296").is_err());
        assert!(parse_u32("-2147483649").is_err());
        assert!(parse_u32("").is_err());
        assert_eq!(parse_i64("-0x10").unwrap(), -16);
        assert_eq!(parse_i64("+8").unwrap(), 8);
        assert_eq!(fmt_u32(0x27f), "0x0000027F");
    }
}
