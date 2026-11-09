//! The C runtime's `printf` as xNVSE's format strings reach it: `ExtractFormattedString`
//! (`GameAPI.cpp` (xNVSE)) hands what is left of a format string after its own specifiers to
//! `snprintf` (the Universal C Runtime's, `ucrtbase.dll`, which xNVSE links as a DLL) with the 20
//! numbers as the argument list ([`ucrt_format`]).
//!
//! Written clean-room from the C standard's `fprintf`, Microsoft Learn's pages on the format
//! specification syntax and its size prefixes, and the real `vsnprintf`'s measured answers: a
//! corpus of 969,777 formats run through a 32-bit harness (issue #357), and `printf_cases.txt`.
//! No runtime source was read.
//!
//! - **A specification:** `%`, flags (`-`, `+`, space, `#`, `0`: any number, in any order), a
//!   width (digits, or `*` for an `int` argument, a negative one meaning `-`; digits right after
//!   the `*` replace it), a precision (`.` and the same; a negative `*` means none), a size, then
//!   the conversion. A character that fits none of these is the conversion: one the runtime
//!   does not know is written as it is, with no padding (`%5%` is `%`, `%5-d` is `-d`). A format
//!   that ends inside a specification ends the text there.
//! - **Sizes:** `hh`, `h`, `l`, `ll`, `j`, `z`, `t`, `L`, `w`, `T`, `I32`, `I64` and a bare `I`.
//!   A second size is refused, except `hh` and `ll` themselves and a size after a bare `I`
//!   (which sets nothing: the pointer's size is an `int`'s here). `N` is always refused; `L`,
//!   `w` and `T` are refused on an integer. Integers read 8 bytes with `ll`, `j` and `I64`, 4
//!   otherwise; a floating-point number always 8; a character, a string and a pointer 4.
//! - **Refusals:** the call fails (`snprintf` returns -1) and the buffer is left empty: `%n`, the
//!   sizes above, a width or precision past what an `int` holds, and a text longer than an
//!   `int` can count. Processing stops at the first failure. `%n` and the sizes go through the
//!   runtime's invalid-parameter handler first, and xNVSE's process has none of its own, so
//!   there the default one ends the game (measured in the real game): with the size caps row the
//!   text is empty, as a handler that returns leaves it; off (vanilla) the game stops.
//! - **Floating point:** the exact decimal digits, ties to even; `%a` has 13 hex digits unless a
//!   precision says otherwise, a subnormal number a leading 0 and the exponent -1022, and a
//!   rounding that carries into the leading digit shows it (`0x2.0p+0`). Infinities and NaNs are
//!   `inf`, `nan`, `nan(ind)` (the negative default NaN) and `nan(snan)`. With `#` and a
//!   precision of 0, the point goes after the first character and the digits that follow it,
//!   which for an infinity or a NaN is inside the word (`i.nf`, `-.inf`); after that a text
//!   that does not start with a letter counts as a number: it may be padded with zeros and, for
//!   `%a`, gets its `0x` (`-0x.inf`).
//! - **The runtime's working buffer:** a precision takes about a byte a digit of memory. Past
//!   [`BUFFER_LIMIT`] the 32-bit process cannot have it: a floating-point conversion falls back to
//!   [`FALLBACK_PRECISION`] digits and an integer one crashes. With the size caps row
//!   (`NV_NVSE_SIZE_CAPS`, docs/FIXES.md) the text is made safely (the fallback for a
//!   floating-point number, the zeros asked for an integer); off (vanilla) the game stops as out
//!   of memory, since where xNVSE's own process runs out depends on the game's memory.
//! - **Strings:** `%s` reads the game's memory; a pointer outside it is the missing array guard
//!   row's ([`super::array::NULL_GUARD`]): the text ends there, or (vanilla) the read stops the
//!   game as xNVSE's does.
//!
//! Not measured: a wide string (`%S`, `%ls`, `%wZ`) with a character above 0xFF (taken as the
//! character being left out, as `%lc` of one is), and a precision on a `%Z` that is not null.

use super::array::{fix_on, out_of_memory, NULL_GUARD, SIZE_CAPS};
use super::string::MAX_MESSAGE_LENGTH;
use engine_core::prelude::*;

/// The most bytes `snprintf(buffer, kMaxMessageLength - 2, ...)` keeps (0x3FFD, then its NUL).
pub(crate) const BUDGET: usize = MAX_MESSAGE_LENGTH - 3;

/// The longest text `snprintf` can count (its result is an `int`).
const MAX_COUNT: u64 = i32::MAX as u64;

/// The largest precision the runtime gets its working buffer for, as the 32-bit harness measured
/// it: floating-point conversions got it at 965,624,998 in every run (one run failed at
/// 965,624,999, the next did not); integers got it at 970,000,000 and crashed at 990,000,000.
/// It is about where a 32-bit process's free memory runs out, so it moves a little between runs.
const BUFFER_LIMIT: u64 = 965_624_998;

/// The precision a floating-point conversion falls back to past [`BUFFER_LIMIT`] (measured: 163
/// digits after the point for `%f`, `%e` and `%a`, 163 significant digits for `%g`).
const FALLBACK_PRECISION: u64 = 163;

/// Digits after the point that a double's exact decimal expansion can have (1074 at most); past
/// them there are only zeros.
const EXACT_DIGITS: u64 = 1100;

/// Why the call failed: `snprintf` returns -1 with an empty buffer either way.
enum Refusal {
    /// The runtime's invalid-parameter handler is called (`%n`, a size the conversion does not
    /// take, a second size).
    Invalid,
    /// No handler: a width or precision past an `int`, or a text longer than an `int` counts.
    Overflow,
}

/// What the call has made: the bytes the buffer keeps and how many were made in all.
struct Out {
    kept: Vec<u8>,
    count: u64,
}

impl Out {
    fn bytes(&mut self, b: &[u8]) {
        let room = BUDGET.saturating_sub(self.kept.len());
        self.kept.extend_from_slice(&b[..b.len().min(room)]);
        self.count += b.len() as u64;
    }

    fn repeat(&mut self, b: u8, n: u64) {
        let room = BUDGET.saturating_sub(self.kept.len()) as u64;
        let len = self.kept.len() + n.min(room) as usize;
        self.kept.resize(len, b);
        self.count += n;
    }

    /// The count past what an `int` holds fails the call at once.
    fn check(&self) -> Result<(), Refusal> {
        if self.count > MAX_COUNT {
            Err(Refusal::Overflow)
        } else {
            Ok(())
        }
    }
}

/// The `va_list`: the 20 doubles' bytes, read in order; past them zeros.
struct VaList<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl VaList<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0u8; N];
        for (i, b) in out.iter_mut().enumerate() {
            *b = self.bytes.get(self.at + i).copied().unwrap_or(0);
        }
        self.at += N;
        out
    }

    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.take::<4>())
    }

    fn u64(&mut self) -> u64 {
        u64::from_le_bytes(self.take::<8>())
    }
}

/// A size prefix. `None` is also a bare `I`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Size {
    None,
    Char,
    Short,
    Long,
    LongLong,
    LongDouble,
    Wide,
    Narrow,
    Max,
    SizeT,
    PtrDiff,
    I32,
    I64,
}

/// A conversion specification: its flags, width, precision and size.
struct Spec {
    left: bool,
    plus: bool,
    space: bool,
    alt: bool,
    zero: bool,
    width: u64,
    precision: Option<u64>,
    size: Size,
}

/// A conversion's text: `lead` (a sign, a `0x`), then `body`, which holds a run of `zeros` zeros
/// at `zeros_at` (a precision's zeros, too many to hold), and then
/// `past` more bytes that are counted but never kept (a string longer than the buffer).
struct Field {
    lead: Vec<u8>,
    body: Vec<u8>,
    zeros_at: usize,
    zeros: u64,
    past: u64,
}

impl Field {
    fn plain(body: Vec<u8>) -> Field {
        Field {
            lead: Vec::new(),
            body,
            zeros_at: 0,
            zeros: 0,
            past: 0,
        }
    }

    fn len(&self) -> u64 {
        (self.lead.len() + self.body.len()) as u64 + self.zeros + self.past
    }
}

/// Writes a field padded to the width: spaces after it (`-`), zeros between its lead and its body
/// (`0`, where `zeros_allowed`), or spaces before it.
fn pad(out: &mut Out, spec: &Spec, field: &Field, zeros_allowed: bool) {
    let gap = spec.width.saturating_sub(field.len());
    let body = |out: &mut Out| {
        out.bytes(&field.body[..field.zeros_at]);
        out.repeat(b'0', field.zeros);
        out.bytes(&field.body[field.zeros_at..]);
        // Past the body's kept part the buffer is full: these are only counted.
        out.count += field.past;
    };
    if spec.left {
        out.bytes(&field.lead);
        body(out);
        out.repeat(b' ', gap);
    } else if spec.zero && zeros_allowed {
        out.bytes(&field.lead);
        out.repeat(b'0', gap);
        body(out);
    } else {
        out.repeat(b' ', gap);
        out.bytes(&field.lead);
        body(out);
    }
}

/// A run of decimal digits at `i`, if any: a width or a precision; past an `int` the call fails.
fn number(format: &[u8], i: &mut usize) -> Result<Option<u64>, Refusal> {
    let mut n: Option<u64> = None;
    while let Some(d) = format.get(*i).filter(|d| d.is_ascii_digit()) {
        let v = n.unwrap_or(0) * 10 + u64::from(d - b'0');
        if v > MAX_COUNT {
            return Err(Refusal::Overflow);
        }
        n = Some(v);
        *i += 1;
    }
    Ok(n)
}

/// The size prefix at `i` (none is [`Size::None`]).
fn size(format: &[u8], i: &mut usize) -> Result<Size, Refusal> {
    let mut size = Size::None;
    while let Some(&c) = format.get(*i) {
        let next = match c {
            // `hh` and `ll`.
            b'h' if size == Size::Short => Size::Char,
            b'l' if size == Size::Long => Size::LongLong,
            // Any other size after one is refused, and `N` always is.
            b'h' | b'l' | b'L' | b'w' | b'T' | b'j' | b'z' | b't' | b'I' | b'N'
                if size != Size::None =>
            {
                return Err(Refusal::Invalid);
            }
            b'N' => return Err(Refusal::Invalid),
            b'h' => Size::Short,
            b'l' => Size::Long,
            b'L' => Size::LongDouble,
            b'w' => Size::Wide,
            b'T' => Size::Narrow,
            b'j' => Size::Max,
            b'z' => Size::SizeT,
            b't' => Size::PtrDiff,
            b'I' => {
                let rest = &format[*i + 1..];
                if rest.starts_with(b"32") {
                    *i += 2;
                    Size::I32
                } else if rest.starts_with(b"64") {
                    *i += 2;
                    Size::I64
                } else {
                    Size::None
                }
            }
            _ => break,
        };
        size = next;
        *i += 1;
    }
    Ok(size)
}

/// Whether the runtime gets its working buffer for a precision. Past [`BUFFER_LIMIT`] it does
/// not: with the size caps row the caller makes its text safely; off it (vanilla) the game stops
/// here, since where xNVSE's own process runs out depends on the game's memory.
fn buffer_had(e: &Engine, precision: u64) -> bool {
    if precision <= BUFFER_LIMIT {
        return true;
    }
    if !fix_on(e, SIZE_CAPS) {
        out_of_memory(&format!(
            "a printf precision of {precision}, a working buffer the C runtime cannot get"
        ));
    }
    false
}

/// The UCRT's `vsnprintf` on `format` with `list` as its arguments: the bytes its buffer holds
/// (at most [`BUDGET`]; a conversion's NUL included), empty when the call fails.
pub(crate) fn ucrt_format(e: &Engine, format: &[u8], list: &[u8]) -> Vec<u8> {
    let mut out = Out {
        kept: Vec::new(),
        count: 0,
    };
    let mut va = VaList { bytes: list, at: 0 };
    match run(e, format, &mut va, &mut out) {
        Ok(()) => out.kept,
        Err(Refusal::Overflow) => Vec::new(),
        Err(Refusal::Invalid) => {
            // Nothing in xNVSE's process installs an invalid-parameter handler, so the runtime's
            // default one ends the game (measured: the real game exits with 0xC0000409 at
            // `sv_Construct "refused [%Ld]" 5`). The size caps row gives what a handler that
            // returns leaves: an empty text.
            if !fix_on(e, SIZE_CAPS) {
                panic!(
                    "the C runtime refused the format string {:?}: its invalid-parameter handler \
                     ends xNVSE's process",
                    String::from_utf8_lossy(format)
                );
            }
            Vec::new()
        }
    }
}

/// Formats `format` into `out` up to its end or the first failure.
fn run(e: &Engine, format: &[u8], va: &mut VaList, out: &mut Out) -> Result<(), Refusal> {
    let mut i = 0;
    while i < format.len() {
        let c = format[i];
        i += 1;
        if c != b'%' {
            out.bytes(&[c]);
            out.check()?;
            continue;
        }
        let mut spec = Spec {
            left: false,
            plus: false,
            space: false,
            alt: false,
            zero: false,
            width: 0,
            precision: None,
            size: Size::None,
        };
        // Flags, then the width: an `int` argument for `*` (negative: left-justified), and digits
        // after it replace it.
        while let Some(&f) = format.get(i) {
            match f {
                b'-' => spec.left = true,
                b'+' => spec.plus = true,
                b' ' => spec.space = true,
                b'#' => spec.alt = true,
                b'0' => spec.zero = true,
                _ => break,
            }
            i += 1;
        }
        // The precision: the same, a negative `*` meaning none.
        if format.get(i) == Some(&b'*') {
            i += 1;
            let w = va.u32() as i32;
            if w < 0 {
                spec.left = true;
            }
            spec.width = u64::from(w.unsigned_abs());
        }
        if let Some(w) = number(format, &mut i)? {
            spec.width = w;
        }
        if format.get(i) == Some(&b'.') {
            i += 1;
            spec.precision = Some(0);
            if format.get(i) == Some(&b'*') {
                i += 1;
                let p = va.u32() as i32;
                spec.precision = u64::try_from(p).ok();
            }
            if let Some(p) = number(format, &mut i)? {
                spec.precision = Some(p);
            }
        }
        spec.size = size(format, &mut i)?;
        let Some(&conv) = format.get(i) else {
            // The format ends inside the specification: so does the text.
            return Ok(());
        };
        i += 1;
        match conv {
            b'd' | b'i' | b'u' | b'o' | b'x' | b'X' => {
                if matches!(spec.size, Size::LongDouble | Size::Wide | Size::Narrow) {
                    return Err(Refusal::Invalid);
                }
                let raw = match spec.size {
                    Size::LongLong | Size::Max | Size::I64 => va.u64(),
                    _ => u64::from(va.u32()),
                };
                integer(e, out, &spec, conv, raw);
            }
            b'e' | b'E' | b'f' | b'F' | b'g' | b'G' | b'a' | b'A' => {
                let v = f64::from_bits(va.u64());
                float(e, out, &spec, conv, v);
            }
            b'c' | b'C' => {
                let v = va.u32();
                let narrow = if is_wide(&spec, conv) {
                    // `wctomb` in the C locale: a byte up to 0xFF; above it nothing at all.
                    let w = v & 0xffff;
                    if w > 0xff {
                        continue;
                    }
                    w as u8
                } else {
                    v as u8
                };
                pad(out, &spec, &Field::plain(vec![narrow]), true);
            }
            b's' | b'S' => {
                let ptr = va.u32();
                let text = if ptr == 0 {
                    let null: &[u8] = b"(null)";
                    let n = spec
                        .precision
                        .map_or(null.len(), |p| null.len().min(p as usize));
                    Some(Field::plain(null[..n].to_vec()))
                } else {
                    guest_string(e, ptr, is_wide(&spec, conv), spec.precision, spec.width)
                };
                let Some(field) = text else {
                    // Outside the game's memory under the missing array guard: the text ends.
                    return Ok(());
                };
                pad(out, &spec, &field, true);
            }
            b'Z' => {
                // An `ANSI_STRING` (or with `l`/`w` a `UNICODE_STRING`): its length in bytes
                // (`u16`) and its buffer (`+4`). A null one is `(null)`, whatever the precision.
                let ptr = va.u32();
                let wide = matches!(spec.size, Size::Long | Size::Wide);
                let text = if ptr == 0 {
                    Some(Field::plain(b"(null)".to_vec()))
                } else if fix_on(e, NULL_GUARD)
                    && !(e.mem.is_mapped(ptr) && e.mem.is_mapped(ptr.wrapping_add(7)))
                {
                    None
                } else {
                    let len = usize::from(e.mem.u16(ptr));
                    let buffer = e.mem.u32(ptr.wrapping_add(4));
                    let n = if wide { len / 2 } else { len };
                    guest_string(e, buffer, wide, Some(n as u64), spec.width)
                };
                let Some(field) = text else {
                    return Ok(());
                };
                pad(out, &spec, &field, true);
            }
            b'p' => {
                // Eight upper-case hex digits; the precision, `#`, `+` and space change nothing,
                // and `0` pads with spaces.
                let v = va.u32();
                pad(
                    out,
                    &spec,
                    &Field::plain(format!("{v:08X}").into_bytes()),
                    false,
                );
            }
            b'n' => return Err(Refusal::Invalid),
            // `%` itself and any character the runtime does not know: as it is, no padding.
            other => out.bytes(&[other]),
        }
        out.check()?;
    }
    Ok(())
}

/// Whether a character or string conversion takes `wchar_t`: `l` and `w` make it wide, `h` and `T`
/// narrow; otherwise `C` and `S` are wide and `c` and `s` narrow.
fn is_wide(spec: &Spec, conv: u8) -> bool {
    match spec.size {
        Size::Long | Size::Wide => true,
        Size::Short | Size::Narrow => false,
        _ => conv.is_ascii_uppercase(),
    }
}

/// An integer conversion of `raw` (the argument's bytes as read).
fn integer(e: &Engine, out: &mut Out, spec: &Spec, conv: u8, raw: u64) {
    let bits = match spec.size {
        Size::Char => 8,
        Size::Short => 16,
        Size::LongLong | Size::Max | Size::I64 => 64,
        _ => 32,
    };
    let signed = conv == b'd' || conv == b'i';
    let (negative, magnitude) = if signed {
        // Sign-extended from the argument's size.
        let shift = 64 - bits;
        let v = ((raw << shift) as i64) >> shift;
        (v < 0, v.unsigned_abs())
    } else if bits == 64 {
        (false, raw)
    } else {
        (false, raw & ((1u64 << bits) - 1))
    };
    let mut digits = match conv {
        b'o' => format!("{magnitude:o}"),
        b'x' => format!("{magnitude:x}"),
        b'X' => format!("{magnitude:X}"),
        _ => format!("{magnitude}"),
    }
    .into_bytes();
    // A precision is the fewest digits: zeros before them; a precision of 0 shows no 0. Past the
    // working buffer the runtime crashes; the size caps row makes the zeros asked.
    let mut zeros = 0;
    if let Some(p) = spec.precision {
        buffer_had(e, p);
        if p == 0 && magnitude == 0 {
            digits.clear();
        }
        zeros = p.saturating_sub(digits.len() as u64);
    }
    let mut lead = Vec::new();
    if signed {
        if negative {
            lead.push(b'-');
        } else if spec.plus {
            lead.push(b'+');
        } else if spec.space {
            lead.push(b' ');
        }
    }
    if spec.alt {
        match conv {
            // `#o` makes sure the digits start with a 0.
            b'o' if zeros == 0 && digits.first() != Some(&b'0') => digits.insert(0, b'0'),
            b'x' if magnitude != 0 => lead.extend_from_slice(b"0x"),
            b'X' if magnitude != 0 => lead.extend_from_slice(b"0X"),
            _ => {}
        }
    }
    let field = Field {
        lead,
        body: digits,
        zeros_at: 0,
        zeros,
        past: 0,
    };
    pad(out, spec, &field, spec.precision.is_none());
}

/// A floating-point conversion: the sign, the digits, the `#` point, then the padding.
fn float(e: &Engine, out: &mut Out, spec: &Spec, conv: u8, v: f64) {
    let upper = conv.is_ascii_uppercase();
    let kind = conv.to_ascii_lowercase();
    let precision = spec.precision.map(|p| {
        if buffer_had(e, p) {
            p
        } else {
            FALLBACK_PRECISION
        }
    });
    // The text with its sign, as the runtime makes it before the flags are applied.
    let mut text: Vec<u8> = Vec::new();
    if v.is_sign_negative() {
        text.push(b'-');
    }
    let sign_len = text.len();
    let mut zeros_at = 0;
    let mut zeros = 0;
    if v.is_finite() {
        let a = v.abs();
        let (digits, extra, suffix) = match kind {
            b'f' => {
                let (d, extra) = fixed(a, precision.unwrap_or(6));
                (d, extra, Vec::new())
            }
            b'e' => {
                let (d, extra, x) = scientific(a, precision.unwrap_or(6));
                (d, extra, exponent(b'e', x, 2))
            }
            b'g' => general(a, precision, spec.alt),
            _ => {
                let (d, extra, x) = hex(a, precision);
                (d, extra, exponent(b'p', x, 1))
            }
        };
        text.extend_from_slice(&digits);
        zeros_at = text.len();
        zeros = extra;
        text.extend_from_slice(&suffix);
    } else if v.is_infinite() {
        text.extend_from_slice(b"inf");
    } else {
        let mantissa = v.to_bits() & ((1 << 52) - 1);
        text.extend_from_slice(if mantissa & (1 << 51) == 0 {
            b"nan(snan)"
        } else if v.is_sign_negative() && mantissa == 1 << 51 {
            b"nan(ind)"
        } else {
            b"nan"
        });
    }
    // `#` with a precision of 0: a point after the first character and the digits after it.
    if spec.alt && precision == Some(0) && kind != b'g' {
        let mut at = 1;
        while text.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        let at = at.min(text.len());
        text.insert(at, b'.');
    }
    if upper {
        text.make_ascii_uppercase();
    }
    let mut lead = Vec::new();
    if sign_len == 1 {
        lead.push(b'-');
    } else if spec.plus {
        lead.push(b'+');
    } else if spec.space {
        lead.push(b' ');
    }
    let body = text.split_off(sign_len);
    // A text that starts with a letter (an infinity or a NaN) is no number: no zeros pad it,
    // and `%a` gives it no `0x`.
    let number = !body.first().is_some_and(u8::is_ascii_alphabetic);
    if number && kind == b'a' {
        lead.extend_from_slice(if upper { b"0X" } else { b"0x" });
    }
    let field = Field {
        lead,
        zeros_at: zeros_at.saturating_sub(sign_len).min(body.len()),
        body,
        zeros,
        past: 0,
    };
    pad(out, spec, &field, number);
}

/// `%f`'s digits of `a` (finite, not negative) with `precision` digits after the point (none
/// and no point for 0), and the zeros past the exact ones.
fn fixed(a: f64, precision: u64) -> (Vec<u8>, u64) {
    let shown = precision.min(EXACT_DIGITS) as usize;
    (
        format!("{a:.shown$}").into_bytes(),
        precision - shown as u64,
    )
}

/// `%e`'s digits `d.ddd` of `a` with `precision` digits after the point, the zeros past the exact
/// ones, and the decimal exponent.
fn scientific(a: f64, precision: u64) -> (Vec<u8>, u64, i32) {
    let shown = precision.min(EXACT_DIGITS) as usize;
    let s = format!("{a:.shown$e}");
    let (digits, x) = s.split_once('e').unwrap_or((&s, "0"));
    (
        digits.as_bytes().to_vec(),
        precision - shown as u64,
        x.parse().unwrap_or(0),
    )
}

/// An exponent: `e+05`, `p-1022`, with at least `min_digits` digits.
fn exponent(letter: u8, x: i32, min_digits: usize) -> Vec<u8> {
    let sign = if x < 0 { '-' } else { '+' };
    format!("{}{sign}{:0min_digits$}", letter as char, x.unsigned_abs()).into_bytes()
}

/// `%g`: `p` significant digits (6 unless given, 0 meaning 1) as `%e` when the exponent `x` is
/// below -4 or at least `p`, else as `%f`; trailing zeros and a bare point go unless `#`, and with
/// `#` there is always a point. Returns the digits, the zeros after them and the exponent part.
fn general(a: f64, precision: Option<u64>, alt: bool) -> (Vec<u8>, u64, Vec<u8>) {
    let p = match precision {
        None => 6,
        Some(0) => 1,
        Some(p) => p,
    };
    let x = if a == 0.0 { 0 } else { scientific(a, p - 1).2 };
    let (mut digits, mut extra, suffix) = if x < -4 || i64::from(x) >= p as i64 {
        let (d, extra, x) = scientific(a, p - 1);
        (d, extra, exponent(b'e', x, 2))
    } else {
        let after = (p as i64 - 1 - i64::from(x)) as u64;
        let (d, extra) = fixed(a, after);
        (d, extra, Vec::new())
    };
    if alt {
        if !digits.contains(&b'.') {
            digits.push(b'.');
        }
    } else if digits.contains(&b'.') {
        extra = 0;
        while digits.last() == Some(&b'0') {
            digits.pop();
        }
        if digits.last() == Some(&b'.') {
            digits.pop();
        }
    }
    (digits, extra, suffix)
}

/// `%a`'s digits `h.hhh` of `a` (finite, not negative), the zeros past the 13 a double has, and
/// the binary exponent. A normal number leads with 1, a subnormal one with 0 (exponent -1022), and
/// zero is `0` with exponent 0; rounding to fewer digits is to nearest, ties to even, and may make
/// the leading digit 2.
fn hex(a: f64, precision: Option<u64>) -> (Vec<u8>, u64, i32) {
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1 << 52) - 1);
    let (lead, x) = match (biased, fraction) {
        (0, 0) => (0u64, 0),
        (0, _) => (0, -1022),
        _ => (1, biased - 1023),
    };
    let wanted = precision.unwrap_or(13);
    let shown = wanted.min(13) as u32;
    let all = (lead << 52) | fraction;
    let dropped = 4 * (13 - shown);
    let mut kept = all >> dropped;
    if dropped > 0 {
        let half = 1u64 << (dropped - 1);
        let rest = all & ((1u64 << dropped) - 1);
        if rest > half || (rest == half && kept & 1 == 1) {
            kept += 1;
        }
    }
    let lead = kept >> (4 * shown);
    let mut digits = format!("{lead}").into_bytes();
    if shown > 0 {
        let fraction = kept & ((1u64 << (4 * shown)) - 1);
        let width = shown as usize;
        digits.extend_from_slice(format!(".{fraction:0width$x}").as_bytes());
    }
    (digits, wanted - u64::from(shown), x)
}

/// The most characters a string conversion reads past the buffer's size: enough to pad to any
/// width up to it exactly. A longer string (only a huge width can tell) counts as this long.
const READ_LIMIT: u64 = 1 << 24;

/// A string argument as a field: the game's `char` (or, `wide`, `wchar_t`) string at `ptr`, up to
/// `max` characters; a wide character above 0xFF gives no byte (`wctomb` in the C locale). The
/// buffer keeps [`BUDGET`] bytes of it; the rest is only counted, and at most the width or the
/// buffer's size, whichever is larger (up to [`READ_LIMIT`]), is read, so a long run of wide
/// characters that give no byte cannot hold the game up. A pointer outside the game's memory
/// stops xNVSE (an access violation): under the missing array guard row the text ends there
/// (`None`), off it the read stops the game as it does.
fn guest_string(e: &Engine, ptr: u32, wide: bool, max: Option<u64>, width: u64) -> Option<Field> {
    let guarded = fix_on(e, NULL_GUARD);
    let step = if wide { 2 } else { 1 };
    let read = width
        .clamp(BUDGET as u64, READ_LIMIT)
        .min(max.unwrap_or(u64::MAX));
    let mut text = Vec::new();
    let mut made = 0u64;
    let mut p = ptr;
    for _ in 0..read {
        if guarded && !(e.mem.is_mapped(p) && e.mem.is_mapped(p.wrapping_add(step - 1))) {
            return None;
        }
        let c = if wide {
            u32::from(e.mem.u16(p))
        } else {
            u32::from(e.mem.u8(p))
        };
        if c == 0 {
            break;
        }
        if c < 0x100 {
            if text.len() < BUDGET {
                text.push(c as u8);
            }
            made += 1;
        }
        p = p.wrapping_add(step);
    }
    let mut field = Field::plain(text);
    field.past = made - field.body.len() as u64;
    Some(field)
}

#[cfg(test)]
#[path = "printf_tests.rs"]
mod tests;
