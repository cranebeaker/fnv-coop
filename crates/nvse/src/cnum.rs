//! The C runtime's number readers xNVSE calls on script strings (`strtod`, `strtol`, `sscanf`
//! `%x`), on raw bytes: a script string is any bytes (a name with an umlaut is one byte in the
//! game's code page, not UTF-8), so nothing here decodes text. Each reader scans once, as the C
//! functions do, and reports where it stopped.
//!
//! - **Spaces:** `isspace` of the "C" locale (space, `\t`, `\n`, `\v`, `\f`, `\r`); any other
//!   byte, non-ASCII included, ends the leading spaces.
//! - **`strtod`:** an optional sign, then `inf`/`infinity`, `nan` (with an optional
//!   `(chars)`), a hexadecimal float (`0x` digits, `.`, `p` exponent) or a decimal one (digits,
//!   `.`, `e` exponent; an exponent without digits is not taken). The decimal value is rounded
//!   once, to nearest, as the UCRT's `strtod` rounds. Out of range: `ERANGE`, infinity or 0.
//! - **`strtol` base 2:** an optional sign and binary digits; out of range saturates to
//!   `LONG_MAX`/`LONG_MIN` with `ERANGE`.
//! - **`sscanf("%x")`:** an optional sign, an optional `0x`, hex digits, into an `unsigned int`
//!   (read in 64 bits as `strtoull` reads, held at the maximum, negated for `-`, the low 32
//!   bits kept); no digits is no conversion.

/// `isspace` in the "C" locale.
pub fn is_c_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn skip_spaces(s: &[u8], mut i: usize) -> usize {
    while i < s.len() && is_c_space(s[i]) {
        i += 1;
    }
    i
}

fn hex_value(b: u8) -> Option<u32> {
    (b as char).to_digit(16)
}

fn starts_with_ignore_case(s: &[u8], at: usize, word: &[u8]) -> bool {
    s.len() >= at + word.len() && s[at..at + word.len()].eq_ignore_ascii_case(word)
}

/// `strtod`: the value, the bytes read (0: no conversion) and whether `errno` is `ERANGE`.
pub fn strtod(s: &[u8]) -> (f64, usize, bool) {
    let mut i = skip_spaces(s, 0);
    let negative = match s.get(i) {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let sign = |v: f64| if negative { -v } else { v };
    if starts_with_ignore_case(s, i, b"inf") {
        let end = if starts_with_ignore_case(s, i, b"infinity") {
            i + 8
        } else {
            i + 3
        };
        return (sign(f64::INFINITY), end, false);
    }
    if starts_with_ignore_case(s, i, b"nan") {
        let mut end = i + 3;
        if s.get(end) == Some(&b'(') {
            let mut j = end + 1;
            while j < s.len() && (s[j].is_ascii_alphanumeric() || s[j] == b'_') {
                j += 1;
            }
            if s.get(j) == Some(&b')') {
                end = j + 1;
            }
        }
        return (sign(f64::NAN), end, false);
    }
    if s.get(i) == Some(&b'0') && matches!(s.get(i + 1), Some(b'x' | b'X')) {
        if let Some((v, end, range)) = hex_float(s, i + 2) {
            return (sign(v), end, range);
        }
        // "0x" with no hex digit: the "0" alone is the number.
        return (sign(0.0), i + 1, false);
    }
    // A decimal float: the longest prefix the grammar takes, read once.
    let start = i;
    let mut digits = 0;
    let mut nonzero = false;
    while i < s.len() && s[i].is_ascii_digit() {
        nonzero |= s[i] != b'0';
        i += 1;
        digits += 1;
    }
    if s.get(i) == Some(&b'.') {
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            nonzero |= s[i] != b'0';
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return (0.0, 0, false);
    }
    if matches!(s.get(i), Some(b'e' | b'E')) {
        let mut j = i + 1;
        if matches!(s.get(j), Some(b'+' | b'-')) {
            j += 1;
        }
        if s.get(j).is_some_and(u8::is_ascii_digit) {
            while j < s.len() && s[j].is_ascii_digit() {
                j += 1;
            }
            i = j;
        }
    }
    // The bytes are ASCII digits, '.', 'e' and a sign: valid UTF-8 and Rust's grammar.
    let text = std::str::from_utf8(&s[start..i]).unwrap_or("0");
    let v = text.parse::<f64>().unwrap_or(0.0);
    let range = v.is_infinite() || (v == 0.0 && nonzero);
    (sign(v), i, range)
}

/// A hexadecimal float's digits from `at` (past `0x`): `None` without a digit.
fn hex_float(s: &[u8], mut i: usize) -> Option<(f64, usize, bool)> {
    // Up to 15 significant digits (60 bits) are kept exactly; later ones only move the
    // exponent and are dropped (a script never writes that many).
    let mut mantissa: u64 = 0;
    let mut exp: i64 = 0;
    let mut kept = 0;
    let mut any = false;
    while let Some(d) = s.get(i).copied().and_then(hex_value) {
        any = true;
        if kept < 15 {
            if mantissa != 0 || d != 0 {
                kept += 1;
            }
            mantissa = mantissa * 16 + u64::from(d);
        } else {
            exp += 4;
        }
        i += 1;
    }
    if s.get(i) == Some(&b'.') {
        i += 1;
        while let Some(d) = s.get(i).copied().and_then(hex_value) {
            any = true;
            if kept < 15 {
                if mantissa != 0 || d != 0 {
                    kept += 1;
                }
                mantissa = mantissa * 16 + u64::from(d);
                exp -= 4;
            }
            i += 1;
        }
    }
    if !any {
        return None;
    }
    if matches!(s.get(i), Some(b'p' | b'P')) {
        let mut j = i + 1;
        let neg = match s.get(j) {
            Some(b'-') => {
                j += 1;
                true
            }
            Some(b'+') => {
                j += 1;
                false
            }
            _ => false,
        };
        if s.get(j).is_some_and(u8::is_ascii_digit) {
            let mut e: i64 = 0;
            while let Some(d) = s.get(j).filter(|b| b.is_ascii_digit()) {
                e = (e * 10 + i64::from(d - b'0')).min(100_000);
                j += 1;
            }
            exp += if neg { -e } else { e };
            i = j;
        }
    }
    let v = mantissa as f64 * 2f64.powi(exp.clamp(-2000, 2000) as i32);
    let range = v.is_infinite() || (v == 0.0 && mantissa != 0);
    Some((v, i, range))
}

/// `strtol(s, &end, 2)`: the value, the bytes read and `ERANGE`.
pub fn strtol_binary(s: &[u8]) -> (i32, usize, bool) {
    let mut i = skip_spaces(s, 0);
    let negative = match s.get(i) {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let start = i;
    // Accumulated past `i32` and held there (more digits only keep it out of range).
    let mut v: i64 = 0;
    while let Some(b @ (b'0' | b'1')) = s.get(i).copied() {
        v = (v * 2 + i64::from(b - b'0')).min(1 << 33);
        i += 1;
    }
    if i == start {
        return (0, 0, false);
    }
    let v = if negative { -v } else { v };
    if v > i64::from(i32::MAX) {
        (i32::MAX, i, true)
    } else if v < i64::from(i32::MIN) {
        (i32::MIN, i, true)
    } else {
        (v as i32, i, false)
    }
}

/// `sscanf(s, "%x", &u)`: the value (unchanged 0 without a conversion) and whether one was
/// made.
pub fn scan_hex(s: &[u8]) -> (u32, bool) {
    let mut i = skip_spaces(s, 0);
    let negative = match s.get(i) {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    if s.get(i) == Some(&b'0')
        && matches!(s.get(i + 1), Some(b'x' | b'X'))
        && s.get(i + 2).copied().and_then(hex_value).is_some()
    {
        i += 2;
    }
    // Read as `strtoull` reads (64 bits, held at the maximum when out of range), the low 32 bits
    // stored.
    let mut v: u64 = 0;
    let mut any = false;
    while let Some(d) = s.get(i).copied().and_then(hex_value) {
        v = v
            .checked_mul(16)
            .and_then(|x| x.checked_add(u64::from(d)))
            .unwrap_or(u64::MAX);
        any = true;
        i += 1;
    }
    if !any {
        return (0, false);
    }
    let v = if negative { v.wrapping_neg() } else { v };
    (v as u32, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strtod_reads_the_longest_number_once() {
        assert_eq!(strtod(b"  2.5abc"), (2.5, 5, false));
        assert_eq!(strtod(b"-1e3x"), (-1000.0, 4, false));
        assert_eq!(strtod(b"1e+"), (1.0, 1, false));
        assert_eq!(strtod(b".5"), (0.5, 2, false));
        assert_eq!(strtod(b"x").1, 0);
        assert_eq!(strtod(b"Inf"), (f64::INFINITY, 3, false));
        assert!(strtod(b"nan(1)").0.is_nan());
        assert_eq!(strtod(b"-0x10"), (-16.0, 5, false));
        assert_eq!(strtod(b"0x1.8p1"), (3.0, 7, false));
        assert_eq!(strtod(b"1e999"), (f64::INFINITY, 5, true));
        // Bytes of the game's code page (an umlaut is 0xFC) end the number; no decoding.
        assert_eq!(strtod(b"12\xfc"), (12.0, 2, false));
        assert_eq!(strtod(b"\xfc12").1, 0);
    }

    #[test]
    fn binary_and_hex() {
        assert_eq!(strtol_binary(b"101z"), (5, 3, false));
        assert_eq!(strtol_binary(b"-11"), (-3, 3, false));
        assert_eq!(strtol_binary(&[b'1'; 40]), (i32::MAX, 40, true));
        assert_eq!(scan_hex(b" 0x1F"), (31, true));
        assert_eq!(scan_hex(b"ff"), (255, true));
        assert_eq!(scan_hex(b"-1"), (u32::MAX, true));
        assert_eq!(scan_hex(b"123456789"), (0x2345_6789, true));
        assert_eq!(scan_hex(b"zz"), (0, false));
    }
}
