//! Tests of the C runtime's `printf` as xNVSE reaches it: hand-made cases, the measured cases of
//! `printf_cases.txt`, and the presets' answers where the runtime cannot go on.

use super::super::array::{set_fix, NULL_GUARD, SIZE_CAPS};
use super::*;

fn doubles(vs: &[f64]) -> Vec<u8> {
    vs.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn words(vs: &[u64]) -> Vec<u8> {
    vs.iter().flat_map(|v| v.to_le_bytes()).collect()
}

/// An engine with the format rows of the fixes preset on.
fn fixes() -> Engine {
    let mut e = Engine::new();
    set_fix(&mut e, NULL_GUARD, true);
    set_fix(&mut e, SIZE_CAPS, true);
    e
}

/// Whether the call stops the game.
fn stops(e: &Engine, format: &str, list: &[u8]) -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ucrt_format(e, format.as_bytes(), list)
    }))
    .is_err()
}

/// The UCRT's `printf` on numbers as `ExtractFormattedString` hands it: each case's format and
/// its doubles.
#[test]
fn printf_formats_as_the_ucrt() {
    let e = Engine::new();
    let cases: &[(&str, &[f64], &str)] = &[
        ("%g", &[3.0], "3"),
        ("%g items", &[0.1], "0.1 items"),
        ("%.2f", &[1.005], "1.00"),
        ("%.0f", &[0.5], "0"),
        ("%.0f", &[1.5], "2"),
        ("%5.1f|", &[-2.25], " -2.2|"),
        ("%f", &[1.0 / 3.0], "0.333333"),
        ("%e", &[12345.678], "1.234568e+04"),
        ("%E", &[0.0], "0.000000E+00"),
        ("%g", &[1e-5], "1e-05"),
        ("%g", &[123456789.0], "1.23457e+08"),
        ("%g", &[100000.0], "100000"),
        ("%g", &[1e6], "1e+06"),
        ("%#g", &[1.0], "1.00000"),
        ("%G", &[1e-10], "1E-10"),
        ("%08.3f", &[1.23456], "0001.235"),
        ("%-6g|", &[1.5], "1.5   |"),
        ("%+g % g", &[2.0, 2.0], "+2  2"),
        ("%g %g", &[f64::INFINITY, f64::NEG_INFINITY], "inf -inf"),
        ("%g", &[f64::from_bits(0xfff8_0000_0000_0000)], "-nan(ind)"),
        ("%f", &[f64::from_bits(0x7ff8_0000_0000_0000)], "nan"),
        ("%a", &[1.0], "0x1.0000000000000p+0"),
        ("%.1a", &[1.5], "0x1.8p+0"),
        ("%A", &[-0.5], "-0X1.0000000000000P-1"),
        ("%d|%d", &[1.0], "0|1072693248"),
        (
            "%5.3d|%x|%#X",
            &[f64::from_bits(0x0000_00ff_0000_0007)],
            "  007|ff|0",
        ),
        (
            "%llX",
            &[f64::from_bits(0xdead_beef_0000_0001)],
            "DEADBEEF00000001",
        ),
        ("%08llX", &[f64::from_bits(255)], "000000FF"),
        ("%c%c", &[f64::from_bits(0x42_0000_0041)], "AB"),
        ("%s|%5.2s|", &[0.0], "(null)|   (n|"),
        (
            "%.*f",
            &[f64::from_bits(2 | (0x4004_0000u64 << 32)), 0.0],
            "0.00",
        ),
        // The real game's own answer (xNVSE 6.4.9's `sv_Construct`, 2026-11-08).
        ("ok [%5.2f] [%#.0e]", &[1.5, 2.0], "ok [ 1.50] [2.e+00]"),
        // `#` with a precision of 0 puts the point inside an infinity's word, and then the text
        // counts as a number: zeros pad it and `%a` gives it its `0x`.
        (
            "%#.0f|%#.0a",
            &[f64::INFINITY, f64::NEG_INFINITY],
            "i.nf|-0x.inf",
        ),
        ("%#010.0f", &[f64::NEG_INFINITY], "-00000.inf"),
        (
            "%.0a|%.12a",
            &[1.5, 1.9999999999999998],
            "0x2p+0|0x2.000000000000p+0",
        ),
    ];
    for (format, args, want) in cases {
        let got = ucrt_format(&e, format.as_bytes(), &doubles(args));
        assert_eq!(String::from_utf8_lossy(&got), *want, "{format}");
    }
}

/// What `snprintf` keeps (0x3FFD bytes) bounds every width and precision, so none allocates more
/// or panics; an unknown conversion is its character, `%0Nc`/`%0Ns` pad with zeros, `%lc` is
/// the byte.
#[test]
fn printf_keeps_to_the_buffer() {
    let e = Engine::new();
    let one = words(&[1.0f64.to_bits()]);
    for format in [
        "%.100000f",
        "%.70000e",
        "%.99999g",
        "%#.99999g",
        "%.70000a",
        "%2000000000d",
        "%-2000000000d|",
        "%.1000000d",
        "%.965624998d",
        "%*d",
        "%.*f",
    ] {
        let out = ucrt_format(&e, format.as_bytes(), &one);
        assert!(out.len() <= BUDGET, "{format}: {}", out.len());
    }
    let big = ucrt_format(&e, b"%.100000f", &one);
    assert!(big.starts_with(b"1.000") && big.len() == BUDGET);
    let cases: &[(&str, u64, &str)] = &[
        ("%1.0qd|%d", 0, "qd|0"),
        ("%qf|", 0, "qf|"),
        ("[%05c]", 0x41, "[0000A]"),
        ("[%06s]", 0, "[(null)]"),
        ("[%08s]", 0, "[00(null)]"),
        ("[%lc]", 0xe9, "[\u{e9}]"),
        ("[%lc]", 0x141, "[]"),
        ("[%5Tc]", 0x41, "[    A]"),
        ("%Lf", 0x3ff8_0000_0000_0000, "1.500000"),
    ];
    for &(format, arg, want) in cases {
        let out = ucrt_format(&e, format.as_bytes(), &words(&[arg]));
        let want: Vec<u8> = want.chars().map(|c| c as u32 as u8).collect();
        assert_eq!(out, want, "{format}");
    }
    // A string outside the game's memory: the game stops (vanilla), or the text ends there.
    assert!(stops(&e, "a%sb", &words(&[0x10])));
    let mut guarded = Engine::new();
    set_fix(&mut guarded, NULL_GUARD, true);
    assert_eq!(ucrt_format(&guarded, b"a%sb", &words(&[0x10])), b"a");
}

/// A count past an `int` fails the call (an empty text) in every preset; up to it the buffer
/// keeps its 0x3FFD bytes.
#[test]
fn printf_fails_past_an_ints_count() {
    for e in [Engine::new(), fixes()] {
        let five = words(&[5 | (5 << 32)]);
        let spaces = ucrt_format(&e, b"%2147483646d%d", &five);
        assert!(spaces.len() == BUDGET && spaces.iter().all(|&b| b == b' '));
        let past = words(&[5 | (55 << 32)]);
        assert!(ucrt_format(&e, b"%2147483646d%d", &past).is_empty());
        // A negative `*` of `INT_MIN + 1` is a width of `INT_MAX`, plus the brackets.
        assert!(ucrt_format(&e, b"[%*d]", &words(&[0x2a_8000_0001])).is_empty());
        assert!(ucrt_format(&e, b"%2147483648d", &five).is_empty());
    }
}

/// A format the runtime refuses ends xNVSE's process (its default invalid-parameter handler):
/// vanilla stops the game; the size caps row gives the empty text a returning handler leaves.
#[test]
fn printf_refusals_stop_vanilla() {
    let e = Engine::new();
    let fixed = fixes();
    for format in [
        "%Ld|%d",
        "%wd",
        "%%%n tail",
        "%5n",
        "%hhhd",
        "%Ihd%N",
        "%LLf",
    ] {
        assert!(stops(&e, format, &words(&[5])), "{format}");
        assert_eq!(
            ucrt_format(&fixed, format.as_bytes(), &words(&[5])),
            b"",
            "{format}"
        );
    }
    // A bare `I` sets no size, so another may follow it.
    assert_eq!(ucrt_format(&e, b"%Ihd", &words(&[0x1_0005])), b"5");
}

/// A precision past the runtime's working buffer: vanilla stops the game (where xNVSE's process
/// runs out is not known); with the size caps row a number with a point falls back to 163 digits,
/// as the runtime does, and an integer (where the runtime crashes) gets the zeros asked.
#[test]
fn printf_precisions_past_the_buffer() {
    let e = Engine::new();
    let fixed = fixes();
    let one = doubles(&[1.0]);
    for format in [
        "%.1000000000f",
        "%.965624999e",
        "%.2147483647g",
        "%.1000000000d",
    ] {
        assert!(stops(&e, format, &one), "{format}");
        assert!(!stops(&fixed, format, &one), "{format}");
    }
    let fallback = ucrt_format(&fixed, b"%.1000000000f", &one);
    assert_eq!(fallback.len(), 165);
    assert!(fallback.starts_with(b"1.000") && fallback.ends_with(b"000"));
    let a = ucrt_format(&fixed, b"%.1500000000a", &doubles(&[-2.5]));
    assert_eq!(a.len(), 171);
    assert!(a.starts_with(b"-0x1.4000") && a.ends_with(b"0p+1"));
    let zeros = ucrt_format(&fixed, b"%.1000000000d", &words(&[5]));
    assert!(zeros.len() == BUDGET && zeros.iter().all(|&b| b == b'0'));
    // At the limit the buffer is had, in both presets.
    let full = ucrt_format(&e, b"%.965624998f", &one);
    assert!(full.len() == BUDGET && full.starts_with(b"1.000"));
}

/// A string conversion reads a bounded number of characters: a run of wide characters that give
/// no byte, longer than the buffer and with no end in mapped memory, gives nothing and reads
/// nothing past the buffer's size (read on, it would reach unmapped memory and stop the game).
#[test]
fn printf_wide_strings_read_a_bounded_run() {
    const AT: u32 = 0x0300_0000;
    let mut e = Engine::new();
    e.map(AT, 0x10000);
    // 32,768 characters of U+0141 fill the mapping, with no NUL after them.
    let run: Vec<u8> = (0..0x8000).flat_map(|_| 0x0141u16.to_le_bytes()).collect();
    e.mem.write(AT, &run);
    assert_eq!(
        ucrt_format(&e, b"[%S]|%d", &words(&[AT as u64 | 7 << 32])),
        b"[]|7"
    );
}

/// A width pads from a guest string's whole length, not from what the buffer keeps of it.
#[test]
fn printf_pads_a_long_string_from_its_length() {
    const AT: u32 = 0x0300_0000;
    let mut e = Engine::new();
    e.map(AT, 0x10000);
    e.mem.write(AT, &[b'a'; 17000]);
    e.mem.set_u8(AT + 17000, 0);
    let out = ucrt_format(&e, b"%20000s", &words(&[AT as u64]));
    assert_eq!(out.len(), BUDGET);
    assert!(out[..3000].iter().all(|&b| b == b' '));
    assert!(out[3000..].iter().all(|&b| b == b'a'));
    // Longer than a width: no padding at all.
    let out = ucrt_format(&e, b"%-16999s|", &words(&[AT as u64]));
    assert!(out.len() == BUDGET && out.iter().all(|&b| b == b'a'));
}

/// The real UCRT's text for each measured case of `printf_cases.txt` (format, arguments and text
/// in hex, a case a line), with the fixes preset's rows on.
#[test]
fn printf_follows_the_ucrts_state_machine() {
    let unhex = |s: &str| {
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
            .collect::<Vec<u8>>()
    };
    let e = fixes();
    let mut checked = 0;
    for line in include_str!("printf_cases.txt").lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('|');
        let format = unhex(parts.next().unwrap());
        let args = unhex(parts.next().unwrap());
        let want = unhex(parts.next().unwrap());
        let got = ucrt_format(&e, &format, &args);
        assert_eq!(got, want, "{}", String::from_utf8_lossy(&format));
        checked += 1;
    }
    assert!(checked > 1200);
}
