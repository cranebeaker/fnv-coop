//! Memory recorded with a call (list tokens): the bytes, or a hash of them, at an address
//! computed from the call's words, with pointers followed.
//!
//! ```text
//! dump=EXPR:LEN      LEN bytes at entry          rdump=EXPR:LEN   the same at return
//! hash=EXPR:LEN      their 64-bit FNV-1a hash    rhash=EXPR:LEN   the same at return
//! shader=EXPR        the Direct3D 9 shader bytecode at EXPR (to its end token): its hash,
//!                    then the bytecode itself when it fits in a dump
//! when=EXPR==NUMBER  record only the calls where the value of EXPR is NUMBER (also !=)
//!
//! EXPR := BASE OFFSETS | '*(' EXPR ')' OFFSETS
//! BASE := 'w'N (the call's word N) | 'this' (w0) | NUMBER (an absolute address)
//! OFFSETS := ('+' | '-') NUMBER ...           NUMBER: decimal or 0x hex
//! LEN := NUMBER | 'w'N | 'w'N'*'NUMBER         (a word of the call times a factor)
//! ```
//!
//! `*(...)` reads the 32-bit word at the address inside it. Examples: `this+0x20:8` (the
//! property's two flag words), `*(*(this+0x60)+8)+0xec:4`, `0x011f9448:4` (a global),
//! `dump=w2:w3*16` (the `w3` four-float constants at `w2`), `hash=w1:w2` (the `w2` bytes of a
//! file in memory), `rdump=w3:4` (the object an out pointer received). A dump is at most
//! 0xfff0 bytes, a hash covers at most 64 MiB. The same syntax is parsed by the port's engine
//! (`crates/engine-core/src/call_trace.rs`), which keeps its own copy.

/// A dump expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Word(usize, i64),
    Abs(u32, i64),
    Deref(Box<Expr>, i64),
}

/// A dump's length: fixed, or a word of the call times a factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Len {
    Fixed(u32),
    Word(usize, u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Bytes,
    Hash,
    Shader,
}

/// One dump token of a list line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DumpSpec {
    /// The token as written (`dump=...`).
    pub text: String,
    pub expr: Expr,
    pub len: Len,
    pub kind: Kind,
    /// Taken when the call returns (`rdump`, `rhash`), from the words it was entered with.
    pub at_return: bool,
}

/// The most bytes a dump keeps and a hash covers.
pub const MAX_DUMP: u32 = 0xfff0;
pub const MAX_HASH: u32 = 0x400_0000;

fn number(s: &str) -> Option<i64> {
    match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(h) => i64::from_str_radix(h, 16).ok(),
        None => s.parse().ok(),
    }
}

/// `+a-b+c...` (possibly empty) as one offset.
fn offsets(s: &str) -> Option<i64> {
    let mut total = 0i64;
    let mut rest = s;
    while !rest.is_empty() {
        let sign = match rest.as_bytes()[0] {
            b'+' => 1,
            b'-' => -1,
            _ => return None,
        };
        rest = &rest[1..];
        let end = rest.find(['+', '-']).unwrap_or(rest.len());
        total += sign * number(&rest[..end])?;
        rest = &rest[end..];
    }
    Some(total)
}

/// Parses an expression (without the `:LEN`).
pub fn parse_expr(s: &str) -> Option<Expr> {
    if let Some(inner_start) = s.strip_prefix("*(") {
        // The matching close parenthesis.
        let mut depth = 1;
        let mut close = None;
        for (i, c) in inner_start.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let close = close?;
        let inner = parse_expr(&inner_start[..close])?;
        let off = offsets(&inner_start[close + 1..])?;
        return Some(Expr::Deref(Box::new(inner), off));
    }
    // A hex base contains no '+'/'-', so the first of those ends it.
    let end = s.find(['+', '-']).unwrap_or(s.len());
    let base = &s[..end];
    let off = offsets(&s[end..])?;
    if base == "this" {
        return Some(Expr::Word(0, off));
    }
    if let Some(n) = base.strip_prefix('w') {
        return Some(Expr::Word(n.parse().ok()?, off));
    }
    Some(Expr::Abs(u32::try_from(number(base)?).ok()?, off))
}

fn parse_len(s: &str, max: u32) -> Option<Len> {
    if let Some(w) = s.strip_prefix('w') {
        let (n, k) = w.split_once('*').unwrap_or((w, "1"));
        let k = u32::try_from(number(k)?).ok().filter(|&k| k > 0)?;
        return Some(Len::Word(n.parse().ok()?, k));
    }
    let l = u32::try_from(number(s)?)
        .ok()
        .filter(|&l| l > 0 && l <= max)?;
    Some(Len::Fixed(l))
}

/// Parses one token (`dump=...`, `rdump=...`, `hash=...`, `rhash=...`, `shader=...`); `None`
/// when the token is not a dump token or is malformed (see [`is_dump_token`]).
pub fn parse_token(token: &str) -> Option<DumpSpec> {
    let (key, v) = token.split_once('=')?;
    let (kind, at_return) = match key {
        "dump" => (Kind::Bytes, false),
        "rdump" => (Kind::Bytes, true),
        "hash" => (Kind::Hash, false),
        "rhash" => (Kind::Hash, true),
        "shader" => (Kind::Shader, false),
        _ => return None,
    };
    let (expr, len) = if kind == Kind::Shader {
        (parse_expr(v)?, Len::Fixed(4))
    } else {
        let (e, len) = v.rsplit_once(':')?;
        let max = if kind == Kind::Hash {
            MAX_HASH
        } else {
            MAX_DUMP
        };
        (parse_expr(e)?, parse_len(len, max)?)
    };
    Some(DumpSpec {
        text: token.to_string(),
        expr,
        len,
        kind,
        at_return,
    })
}

/// Whether a list token is one of the dump tokens (then it must parse).
pub fn is_dump_token(token: &str) -> bool {
    ["dump=", "rdump=", "hash=", "rhash=", "shader="]
        .iter()
        .any(|p| token.starts_with(p))
}

/// `dump=EXPR:LEN` (kept for callers that build a plain dump).
pub fn parse(s: &str) -> Option<DumpSpec> {
    parse_token(&format!("dump={s}"))
}

impl Expr {
    /// The address, given the call's words and a reader of 32-bit words (None: unreadable).
    pub fn eval(&self, words: &[u32], read: &mut dyn FnMut(u32) -> Option<u32>) -> Option<u32> {
        match self {
            Expr::Word(i, off) => Some(words.get(*i)?.wrapping_add(*off as u32)),
            Expr::Abs(a, off) => Some(a.wrapping_add(*off as u32)),
            Expr::Deref(inner, off) => {
                let at = inner.eval(words, read)?;
                Some(read(at)?.wrapping_add(*off as u32))
            }
        }
    }
}

impl Len {
    /// The length for these words (`None` when out of range).
    pub fn eval(&self, words: &[u32], max: u32) -> Option<u32> {
        let l = match *self {
            Len::Fixed(l) => l,
            Len::Word(i, k) => words.get(i)?.checked_mul(k)?,
        };
        (l > 0 && l <= max).then_some(l)
    }
}

/// 64-bit FNV-1a.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// The length in bytes of Direct3D 9 shader bytecode starting at its version token: up to and
/// including the end token 0x0000FFFF (comments skipped by their length), given a reader of
/// 32-bit words; `None` when unreadable or longer than 256 KiB.
pub fn shader_len(at: u32, read: &mut dyn FnMut(u32) -> Option<u32>) -> Option<u32> {
    let mut off = 4u32;
    while off < 0x4_0000 {
        let token = read(at.wrapping_add(off))?;
        if token == 0x0000_ffff {
            return Some(off + 4);
        }
        // A comment token: 0xFFFE, its length in dwords in bits 16..30.
        if token & 0xffff == 0xfffe {
            off += 4 * ((token >> 16) & 0x7fff);
        }
        off += 4;
    }
    None
}

/// What a recorder writes for a dump: for `Bytes`, the bytes; for `Hash`, the 8-byte hash
/// followed by the 4-byte length hashed; for `Shader`, [`shader_payload`].
pub fn hash_payload(bytes: &[u8]) -> Vec<u8> {
    let mut out = fnv1a(bytes).to_le_bytes().to_vec();
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out
}

/// A shader's dump: the hash payload, then the bytecode when the whole fits in a dump (so the
/// diff can see which constants and samplers the shader reads).
pub fn shader_payload(bytes: &[u8]) -> Vec<u8> {
    let mut out = hash_payload(bytes);
    if out.len() + bytes.len() <= MAX_DUMP as usize {
        out.extend_from_slice(bytes);
    }
    out
}

/// A condition on a call (`when=VALUE==NUMBER`, `when=VALUE!=NUMBER`): VALUE is an expression
/// as above taken as a value (`*(this)` is the object's vtable, `w2` the word itself), compared
/// with a number. A call that fails a condition, or whose VALUE is unreadable, is not recorded
/// and not counted by `max=`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cond {
    /// The token as written.
    pub text: String,
    pub expr: Expr,
    pub value: u32,
    pub equal: bool,
}

/// Parses a `when=` token; `None` when the token is not one or is malformed.
pub fn parse_when(token: &str) -> Option<Cond> {
    let v = token.strip_prefix("when=")?;
    let (e, n, equal) = match v.split_once("!=") {
        Some((e, n)) => (e, n, false),
        None => {
            let (e, n) = v.split_once("==")?;
            (e, n, true)
        }
    };
    Some(Cond {
        text: token.to_string(),
        expr: parse_expr(e)?,
        value: u32::try_from(number(n)?).ok()?,
        equal,
    })
}

impl Cond {
    pub fn holds(&self, words: &[u32], read: &mut dyn FnMut(u32) -> Option<u32>) -> bool {
        self.expr
            .eval(words, read)
            .is_some_and(|v| (v == self.value) == self.equal)
    }
}

/// One dump as recorded: the address (0 when it couldn't be computed) and the bytes (`None`
/// when unreadable).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dumped {
    pub addr: u32,
    pub bytes: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expressions_parse_and_follow_pointers() {
        let d = parse("*(*(this+0x60)+8)+0xec:4").unwrap();
        assert_eq!(d.len, Len::Fixed(4));
        let mem = |a: u32| match a {
            0x1060 => Some(0x2000),
            0x2008 => Some(0x3000),
            _ => None,
        };
        let mut read = mem;
        assert_eq!(d.expr.eval(&[0x1000], &mut read), Some(0x30ec));
        let d = parse("w1+0x20-4:8").unwrap();
        assert_eq!(d.expr.eval(&[0, 0x100], &mut read), Some(0x11c));
        // An unreadable pointer gives no address.
        assert_eq!(
            parse("*(w0)+4:4").unwrap().expr.eval(&[0x5000], &mut read),
            None
        );
        // An absolute address.
        assert_eq!(
            parse("0x011f9448:4").unwrap().expr.eval(&[], &mut read),
            Some(0x011f_9448)
        );
        for bad in [
            "this+0x20",
            "x+1:4",
            "*(this+0x60:4",
            "this:0",
            "this:70000",
            "w0+zz:4",
        ] {
            assert!(parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn tokens_lengths_and_hashes() {
        let d = parse_token("dump=w2:w3*16").unwrap();
        assert_eq!(d.len, Len::Word(3, 16));
        assert_eq!(d.len.eval(&[0, 0, 0, 4], MAX_DUMP), Some(64));
        assert_eq!(d.len.eval(&[0, 0, 0, 0], MAX_DUMP), None);
        let h = parse_token("hash=w1:w2").unwrap();
        assert_eq!((h.kind, h.at_return), (Kind::Hash, false));
        let r = parse_token("rdump=w3:4").unwrap();
        assert!(r.at_return);
        assert_eq!(parse_token("shader=w1").unwrap().kind, Kind::Shader);
        assert!(parse_token("max=3").is_none());
        assert!(is_dump_token("rhash=w1:4"));
        // FNV-1a test vector.
        assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
        // A shader: version, one comment of 1 dword, an instruction, the end token.
        let words = [
            0xffff_0300u32,
            0x0001_fffe,
            0x1234,
            0x0000_0001,
            0x0000_ffff,
        ];
        let mut read = |a: u32| words.get(((a - 0x100) / 4) as usize).copied();
        assert_eq!(shader_len(0x100, &mut read), Some(20));
    }

    #[test]
    fn conditions_compare_values() {
        let c = parse_when("when=*(this)==0x010b7eb8").unwrap();
        let mut read = |a: u32| (a == 0x1000).then_some(0x010b_7eb8);
        assert!(c.holds(&[0x1000], &mut read));
        assert!(!c.holds(&[0x2000], &mut read));
        let c = parse_when("when=w1!=0").unwrap();
        assert!(c.holds(&[0, 5], &mut read));
        assert!(!c.holds(&[0, 0], &mut read));
        assert!(parse_when("when=w1").is_none());
        assert!(parse_when("when=w1==x").is_none());
    }
}
