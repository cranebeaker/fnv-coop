//! The probe manifest: the line-based text file `nv-probe.txt` that sits
//! next to `nv-probe.dll` and says what to hook and where to write.
//!
//! # Format
//!
//! One directive per line. Blank lines and lines starting with `#` are
//! ignored. There are two kinds of line.
//!
//! Settings, written `key = value` (the value is the rest of the line):
//!
//! | key | meaning | default |
//! | --- | --- | --- |
//! | `output` | log file path; `%PID%` is replaced by the process id; a relative path is relative to the DLL's folder | `nv-probe.jsonl` |
//! | `host_sha256` | expected SHA-256 of the host exe file; on mismatch no hook is installed | not checked (the log header carries the hash either way) |
//! | `mode` | `inject`, `nvse` or `auto` (see the README) | `auto` |
//! | `nvse_grace_ms` | how long `auto` mode waits for NVSE to call the plugin before starting on its own | `1500` |
//! | `flush_ms` | how often buffered log lines are written | `250` |
//! | `suspend_threads` | `1` suspends other threads while patching, `0` does not | `1` |
//! | `max_records` | default per-hook limit on logged calls | `100000` |
//!
//! Hook lines, written `hook <name> key=value ...`. Values containing
//! spaces are put in double quotes.
//!
//! | key | meaning |
//! | --- | --- |
//! | `addr` | function entry address, `0x...` (required) |
//! | `cc` | `cdecl`, `stdcall`, `thiscall` or `fastcall` (default `cdecl`) |
//! | `bytes` | the bytes expected at `addr`, hex, at least as many as are stolen; the hook is refused if memory differs (required) |
//! | `steal` | number of bytes to replace, at least 5 and ending on an instruction boundary, or `auto` for the first boundary of 5 or more (default `auto`) |
//! | `args` | comma-separated argument types: `i8 u8 i16 u16 i32 u32 bool ptr f32 f64 i64 u64` (default none) |
//! | `ret` | capture the return: `void i32 u32 ptr i64 f32_x87 f64_x87 xmm0` (default: no return capture) |
//! | `dump` | memory to record, `expr:length`, several separated by commas; may repeat |
//! | `regs` | `1` to log all eight general registers too (for functions with custom conventions) |
//! | `max` | limit on logged calls for this hook (default `max_records`) |
//!
//! A dump expression is a base plus optional offsets: `ecx+0x170:16`,
//! `arg0+0x10:12`, `[0x011dea3c]+0x760:4`, `esp+4:8`. Bases are the eight
//! general registers at function entry, `argN` (the value of declared
//! argument N, counting `this` in ECX as `arg0` for `thiscall`), a number,
//! or `[expr]`, which reads a 32-bit value from memory.
//!
//! Example:
//!
//! ```text
//! output = nv-probe-%PID%.jsonl
//! host_sha256 = 0123...cdef
//! hook look_ik addr=0x00401000 cc=thiscall bytes="55 8B EC 83 E4 F0" steal=6 args=ptr,f32,f32 ret=void dump=ecx+0x170:16
//! ```

use crate::abi::{ArgType, CallConv, RetKind};
use crate::decode;
use crate::hex;
use std::collections::HashSet;
use std::fmt;

/// How the probe decides when to start (see the README).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Inject,
    Nvse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reg {
    Eax,
    Ecx,
    Edx,
    Ebx,
    Esp,
    Ebp,
    Esi,
    Edi,
}

impl Reg {
    fn parse(text: &str) -> Option<Reg> {
        Some(match text {
            "eax" => Reg::Eax,
            "ecx" => Reg::Ecx,
            "edx" => Reg::Edx,
            "ebx" => Reg::Ebx,
            "esp" => Reg::Esp,
            "ebp" => Reg::Ebp,
            "esi" => Reg::Esi,
            "edi" => Reg::Edi,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom {
    Reg(Reg),
    Arg(usize),
    Const(u32),
    /// Read a 32-bit value from the address the inner expression gives.
    Deref(Box<Expr>),
}

/// An address expression: an atom plus a signed offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub atom: Atom,
    pub offset: i64,
}

/// What an expression needs from the running program.
pub trait EvalContext {
    fn reg(&self, r: Reg) -> u32;
    fn arg(&self, index: usize) -> Result<u32, String>;
    fn read_u32(&self, addr: u32) -> Result<u32, String>;
}

impl Expr {
    /// Parse `base [+|- number]...`.
    pub fn parse(text: &str) -> Result<Expr, String> {
        let t = text.trim();
        if t.is_empty() {
            return Err("empty expression".into());
        }
        let (atom, rest) = if let Some(inner) = t.strip_prefix('[') {
            let mut depth = 1;
            let mut end = None;
            for (i, c) in inner.char_indices() {
                match c {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let end = end.ok_or_else(|| format!("missing ']' in {text:?}"))?;
            (
                Atom::Deref(Box::new(Expr::parse(&inner[..end])?)),
                &inner[end + 1..],
            )
        } else {
            let end = t.find(['+', '-']).unwrap_or(t.len());
            let head = t[..end].trim();
            let lower = head.to_ascii_lowercase();
            let atom = if let Some(r) = Reg::parse(&lower) {
                Atom::Reg(r)
            } else if let Some(n) = lower.strip_prefix("arg") {
                Atom::Arg(
                    n.parse::<usize>()
                        .map_err(|_| format!("bad argument reference {head:?}"))?,
                )
            } else {
                Atom::Const(
                    hex::parse_u32(head)
                        .map_err(|_| format!("unknown base {head:?} in {text:?}"))?,
                )
            };
            (atom, &t[end..])
        };
        let mut offset = 0i64;
        let mut rest = rest.trim();
        while !rest.is_empty() {
            let sign = match rest.as_bytes()[0] {
                b'+' => 1i64,
                b'-' => -1i64,
                _ => return Err(format!("expected '+' or '-' in {text:?}")),
            };
            let tail = &rest[1..];
            let end = tail.find(['+', '-']).unwrap_or(tail.len());
            let n = hex::parse_u64(tail[..end].trim())?;
            let n = i64::try_from(n).map_err(|_| format!("offset too large in {text:?}"))?;
            offset = offset
                .checked_add(sign * n)
                .ok_or_else(|| format!("offset overflow in {text:?}"))?;
            rest = tail[end..].trim_start();
        }
        Ok(Expr { atom, offset })
    }

    /// Evaluate to a 32-bit address (arithmetic wraps like the CPU's).
    pub fn eval(&self, ctx: &dyn EvalContext) -> Result<u32, String> {
        let base = match &self.atom {
            Atom::Reg(r) => ctx.reg(*r),
            Atom::Arg(i) => ctx.arg(*i)?,
            Atom::Const(c) => *c,
            Atom::Deref(inner) => {
                let addr = inner.eval(ctx)?;
                ctx.read_u32(addr)?
            }
        };
        Ok(base.wrapping_add(self.offset as u32))
    }
}

/// A memory region to record: where (an expression) and how many bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DumpSpec {
    pub text: String,
    pub expr: Expr,
    pub len: usize,
}

pub const MAX_DUMP_LEN: usize = 65536;

impl DumpSpec {
    pub fn parse(text: &str) -> Result<DumpSpec, String> {
        let t = text.trim();
        let (expr, len) = t
            .rsplit_once(':')
            .ok_or_else(|| format!("dump {t:?} needs ':length'"))?;
        let len = hex::parse_u64(len).map_err(|e| format!("dump length: {e}"))? as usize;
        if len == 0 || len > MAX_DUMP_LEN {
            return Err(format!("dump length must be 1..={MAX_DUMP_LEN}: {t:?}"));
        }
        Ok(DumpSpec {
            text: t.to_string(),
            expr: Expr::parse(expr)?,
            len,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HookSpec {
    pub name: String,
    pub addr: u32,
    pub cc: CallConv,
    /// Bytes to replace (always at least 5 and on an instruction boundary).
    pub steal: usize,
    /// True if `steal` was chosen automatically.
    pub steal_auto: bool,
    pub expected: Vec<u8>,
    pub args: Vec<ArgType>,
    pub ret: Option<RetKind>,
    pub dumps: Vec<DumpSpec>,
    pub max: Option<u64>,
    /// Log the general registers at entry (and at return).
    pub regs: bool,
    /// Manifest line number, for messages.
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    pub output: Option<String>,
    pub host_sha256: Option<[u8; 32]>,
    pub mode: Mode,
    pub nvse_grace_ms: u32,
    pub flush_ms: u32,
    pub suspend_threads: bool,
    pub max_records: u64,
    pub hooks: Vec<HookSpec>,
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest {
            output: None,
            host_sha256: None,
            mode: Mode::Auto,
            nvse_grace_ms: 1500,
            flush_ms: 250,
            suspend_threads: true,
            max_records: 100_000,
            hooks: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestError {
    pub line: usize,
    pub msg: String,
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.msg)
    }
}

impl std::error::Error for ManifestError {}

/// Split a hook line into tokens, honouring double quotes: `key="a b"`
/// yields one token `key=a b`.
fn tokenize(line: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut have = false;
    for c in line.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                have = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if have {
                    tokens.push(std::mem::take(&mut cur));
                    have = false;
                }
            }
            c => {
                cur.push(c);
                have = true;
            }
        }
    }
    if in_quotes {
        return Err("unterminated quote".into());
    }
    if have {
        tokens.push(cur);
    }
    Ok(tokens)
}

fn parse_bool(text: &str) -> Result<bool, String> {
    match text.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        other => Err(format!("expected 0 or 1, got {other:?}")),
    }
}

fn parse_hook(line_no: usize, text: &str) -> Result<HookSpec, String> {
    let tokens = tokenize(text)?;
    let name = tokens.first().ok_or("hook needs a name")?.clone();
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | ':' | '-'))
    {
        return Err(format!(
            "hook name {name:?} may only use letters, digits and _ . : -"
        ));
    }
    let mut addr = None;
    let mut cc = CallConv::Cdecl;
    let mut steal_text: Option<String> = None;
    let mut bytes = None;
    let mut args = Vec::new();
    let mut ret = None;
    let mut dumps = Vec::new();
    let mut max = None;
    let mut regs = false;
    let mut seen: HashSet<String> = HashSet::new();
    for tok in &tokens[1..] {
        let (key, value) = tok
            .split_once('=')
            .ok_or_else(|| format!("expected key=value, got {tok:?}"))?;
        let key = key.to_ascii_lowercase();
        if key != "dump" && !seen.insert(key.clone()) {
            return Err(format!("{key} given twice"));
        }
        match key.as_str() {
            "addr" => addr = Some(hex::parse_u32(value).map_err(|e| format!("addr: {e}"))?),
            "cc" => cc = CallConv::parse(value)?,
            "steal" => steal_text = Some(value.to_string()),
            "bytes" => bytes = Some(hex::decode_lenient(value).map_err(|e| format!("bytes: {e}"))?),
            "args" => {
                let v = value.trim();
                if !v.is_empty() && !v.eq_ignore_ascii_case("none") {
                    for a in v.split(',') {
                        args.push(ArgType::parse(a)?);
                    }
                }
            }
            "ret" => ret = Some(RetKind::parse(value)?),
            "dump" => {
                for d in value.split(',') {
                    dumps.push(DumpSpec::parse(d)?);
                }
            }
            "max" => max = Some(hex::parse_u64(value).map_err(|e| format!("max: {e}"))?),
            "regs" => regs = parse_bool(value).map_err(|e| format!("regs: {e}"))?,
            other => return Err(format!("unknown hook key {other:?}")),
        }
    }
    let addr = addr.ok_or("hook needs addr=")?;
    let expected = bytes.ok_or("hook needs bytes= (the bytes expected at addr)")?;
    if expected.len() < decode::JMP_LEN {
        return Err(format!("bytes= needs at least {} bytes", decode::JMP_LEN));
    }
    // Validate the stolen range with the decoder now, so a bad manifest is
    // reported with its line number instead of at injection time.
    let (steal, steal_auto) = match steal_text.as_deref() {
        None | Some("auto") => (
            decode::auto_steal(&expected).map_err(|e| e.to_string())?,
            true,
        ),
        Some(n) => {
            let n = hex::parse_u64(n).map_err(|e| format!("steal: {e}"))? as usize;
            if n > expected.len() {
                return Err(format!(
                    "steal={n} but bytes= has only {} bytes",
                    expected.len()
                ));
            }
            // Validate boundary and relocatability without needing real
            // addresses (branches into the stolen range are checked again
            // when the hook is installed).
            decode::build_trampoline(&expected, n, addr, 0x1000_0000).map_err(|e| e.to_string())?;
            (n, false)
        }
    };
    if steal > expected.len() {
        return Err(format!(
            "automatic steal count {steal} is longer than bytes= ({} bytes)",
            expected.len()
        ));
    }
    let _ = line_no;
    Ok(HookSpec {
        name,
        addr,
        cc,
        steal,
        steal_auto,
        expected,
        args,
        ret,
        dumps,
        max,
        regs,
        line: line_no,
    })
}

/// Read just `mode` and `nvse_grace_ms` from manifest text, ignoring every
/// other line and any errors in it. The probe uses this to decide *when* to
/// start even if the rest of the manifest cannot be parsed, so the problem
/// can still be reported at the right moment.
pub fn peek_settings(text: &str) -> (Mode, u32) {
    let defaults = Manifest::default();
    let (mut mode, mut grace) = (defaults.mode, defaults.nvse_grace_ms);
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match (
            key.trim().to_ascii_lowercase().as_str(),
            value.trim().to_ascii_lowercase().as_str(),
        ) {
            ("mode", "auto") => mode = Mode::Auto,
            ("mode", "inject") => mode = Mode::Inject,
            ("mode", "nvse") => mode = Mode::Nvse,
            ("nvse_grace_ms", v) => {
                if let Ok(g) = hex::parse_u32(v) {
                    grace = g;
                }
            }
            _ => {}
        }
    }
    (mode, grace)
}

impl Manifest {
    /// Parse manifest text. Errors carry the line number.
    pub fn parse(text: &str) -> Result<Manifest, ManifestError> {
        let mut m = Manifest::default();
        let mut names = HashSet::new();
        let mut hook_addrs: Vec<(u32, usize, String)> = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let line_no = i + 1;
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let err = |msg: String| ManifestError { line: line_no, msg };
            if let Some(rest) = line
                .strip_prefix("hook")
                .filter(|r| r.starts_with(char::is_whitespace))
            {
                let hook = parse_hook(line_no, rest.trim()).map_err(err)?;
                if !names.insert(hook.name.clone()) {
                    return Err(err(format!("hook name {:?} used twice", hook.name)));
                }
                for (addr, len, other) in &hook_addrs {
                    let (a0, a1) = (hook.addr as u64, hook.addr as u64 + hook.steal as u64);
                    let (b0, b1) = (*addr as u64, *addr as u64 + *len as u64);
                    if a0 < b1 && b0 < a1 {
                        return Err(err(format!(
                            "hook {:?} overlaps the stolen bytes of hook {other:?}",
                            hook.name
                        )));
                    }
                }
                hook_addrs.push((hook.addr, hook.steal, hook.name.clone()));
                m.hooks.push(hook);
                continue;
            }
            let (key, value) = line.split_once('=').ok_or_else(|| {
                err(format!(
                    "expected 'key = value' or a hook line, got {line:?}"
                ))
            })?;
            let (key, value) = (key.trim().to_ascii_lowercase(), value.trim());
            match key.as_str() {
                "output" => {
                    if value.is_empty() {
                        return Err(err("output needs a path".into()));
                    }
                    m.output = Some(value.to_string());
                }
                "host_sha256" => {
                    let bytes = hex::decode(value).map_err(|e| err(format!("host_sha256: {e}")))?;
                    let arr: [u8; 32] = bytes
                        .try_into()
                        .map_err(|_| err("host_sha256 must be 64 hex digits".to_string()))?;
                    m.host_sha256 = Some(arr);
                }
                "mode" => {
                    m.mode = match value.to_ascii_lowercase().as_str() {
                        "auto" => Mode::Auto,
                        "inject" => Mode::Inject,
                        "nvse" => Mode::Nvse,
                        other => {
                            return Err(err(format!(
                                "mode must be auto, inject or nvse, got {other:?}"
                            )))
                        }
                    }
                }
                "nvse_grace_ms" => {
                    m.nvse_grace_ms =
                        hex::parse_u32(value).map_err(|e| err(format!("{key}: {e}")))?
                }
                "flush_ms" => {
                    m.flush_ms = hex::parse_u32(value).map_err(|e| err(format!("{key}: {e}")))?
                }
                "suspend_threads" => {
                    m.suspend_threads = parse_bool(value).map_err(|e| err(format!("{key}: {e}")))?
                }
                "max_records" => {
                    m.max_records = hex::parse_u64(value).map_err(|e| err(format!("{key}: {e}")))?
                }
                other => return Err(err(format!("unknown setting {other:?}"))),
            }
        }
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct Ctx {
        regs: HashMap<&'static str, u32>,
        args: Vec<u32>,
        mem: HashMap<u32, u32>,
    }

    impl EvalContext for Ctx {
        fn reg(&self, r: Reg) -> u32 {
            let name = match r {
                Reg::Eax => "eax",
                Reg::Ecx => "ecx",
                Reg::Edx => "edx",
                Reg::Ebx => "ebx",
                Reg::Esp => "esp",
                Reg::Ebp => "ebp",
                Reg::Esi => "esi",
                Reg::Edi => "edi",
            };
            *self.regs.get(name).unwrap_or(&0)
        }
        fn arg(&self, index: usize) -> Result<u32, String> {
            self.args
                .get(index)
                .copied()
                .ok_or_else(|| format!("no argument {index}"))
        }
        fn read_u32(&self, addr: u32) -> Result<u32, String> {
            self.mem
                .get(&addr)
                .copied()
                .ok_or_else(|| format!("unreadable {addr:#x}"))
        }
    }

    fn ctx() -> Ctx {
        Ctx {
            regs: HashMap::from([("ecx", 0x1000), ("esp", 0x2000), ("eax", 5)]),
            args: vec![0x1000, 0x3000],
            mem: HashMap::from([(0x0011_dea3, 0), (0x011d_ea3c, 0x5000)]),
        }
    }

    #[test]
    fn dump_expressions_from_the_spec() {
        let c = ctx();
        let d = DumpSpec::parse("ecx+0x170:16").unwrap();
        assert_eq!((d.expr.eval(&c).unwrap(), d.len), (0x1170, 16));
        let d = DumpSpec::parse("arg0+0x10:12").unwrap();
        assert_eq!(d.expr.eval(&c).unwrap(), 0x1010);
        let d = DumpSpec::parse("arg1:4").unwrap();
        assert_eq!(d.expr.eval(&c).unwrap(), 0x3000);
        let d = DumpSpec::parse("[0x011dea3c]+0x760:4").unwrap();
        assert_eq!(d.expr.eval(&c).unwrap(), 0x5760);
        let d = DumpSpec::parse("esp+4:8").unwrap();
        assert_eq!((d.expr.eval(&c).unwrap(), d.len), (0x2004, 8));
        let d = DumpSpec::parse("0x401000:0x20").unwrap();
        assert_eq!((d.expr.eval(&c).unwrap(), d.len), (0x40_1000, 32));
    }

    #[test]
    fn expression_arithmetic() {
        let c = ctx();
        assert_eq!(Expr::parse("esp-4").unwrap().eval(&c).unwrap(), 0x1ffc);
        assert_eq!(
            Expr::parse("esp+0x10-0x4+2").unwrap().eval(&c).unwrap(),
            0x200e
        );
        assert!(Expr::parse("[arg0-0x1000+0x011dea3c]").is_ok());
        // Wrapping at 32 bits, like the CPU.
        let mut c2 = ctx();
        c2.regs.insert("ecx", 0xffff_fff0);
        assert_eq!(Expr::parse("ecx+0x20").unwrap().eval(&c2).unwrap(), 0x10);
        // Nested dereference with an offset inside.
        let mut c3 = ctx();
        c3.mem.insert(0x1004, 0x7000);
        c3.mem.insert(0x7008, 0x9000);
        assert_eq!(
            Expr::parse("[[ecx+4]+8]+1").unwrap().eval(&c3).unwrap(),
            0x9001
        );
        // Failure paths are reported, not panics.
        assert!(Expr::parse("[0x1]").unwrap().eval(&c).is_err());
        assert!(Expr::parse("arg9").unwrap().eval(&c).is_err());
    }

    #[test]
    fn bad_expressions() {
        for bad in [
            "", "foo", "ecx+", "ecx*4", "[ecx", "arg", "argx", "ecx+zz", "+4",
        ] {
            assert!(Expr::parse(bad).is_err(), "{bad:?}");
        }
        for bad in ["ecx", "ecx:0", "ecx:70000", "ecx:x"] {
            assert!(DumpSpec::parse(bad).is_err(), "{bad:?}");
        }
    }

    const HOOK: &str = r#"
# comment
output = C:\private\probe-%PID%.jsonl
host_sha256 = 0000000000000000000000000000000000000000000000000000000000000001
mode = inject
nvse_grace_ms = 500
flush_ms = 100
suspend_threads = 0
max_records = 50

hook look_ik addr=0x00401000 cc=thiscall bytes="55 8B EC 83 E4 F0" steal=6 args=ptr,f32,f32 ret=void dump=ecx+0x170:16,arg0+0x10:12 regs=1
hook add3 addr=0x00402000 bytes=558BEC83EC10 args=i32,i32,i32 ret=i32 max=3
hook lerp addr=0x00403000 cc=cdecl bytes=6AFF68785634126A00 ret=f32_x87 dump=esp+4:8 dump=[0x011dea3c]+0x760:4
"#;

    #[test]
    fn parses_a_full_manifest() {
        let m = Manifest::parse(HOOK).unwrap();
        assert_eq!(m.output.as_deref(), Some(r"C:\private\probe-%PID%.jsonl"));
        assert_eq!(m.host_sha256.unwrap()[31], 1);
        assert_eq!(m.mode, Mode::Inject);
        assert_eq!(
            (
                m.nvse_grace_ms,
                m.flush_ms,
                m.suspend_threads,
                m.max_records
            ),
            (500, 100, false, 50)
        );
        assert_eq!(m.hooks.len(), 3);
        let h = &m.hooks[0];
        assert_eq!(
            (h.name.as_str(), h.addr, h.cc, h.steal, h.steal_auto),
            ("look_ik", 0x40_1000, CallConv::Thiscall, 6, false)
        );
        assert_eq!(h.args, [ArgType::Ptr, ArgType::F32, ArgType::F32]);
        assert_eq!(h.ret, Some(RetKind::Void));
        assert_eq!(h.dumps.len(), 2);
        assert!(h.regs);
        assert_eq!(h.expected, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf0]);
        let h = &m.hooks[1];
        assert_eq!(
            (h.steal, h.steal_auto, h.max, h.ret, h.cc),
            (6, true, Some(3), Some(RetKind::I32), CallConv::Cdecl)
        );
        assert!(!h.regs);
        let h = &m.hooks[2];
        assert_eq!(h.steal, 7); // 6A FF | 68 78 56 34 12 -> boundary at 2 then 7
        assert_eq!(h.dumps.len(), 2);
    }

    #[test]
    fn defaults() {
        let m = Manifest::parse("").unwrap();
        assert_eq!(m, Manifest::default());
        assert_eq!(m.mode, Mode::Auto);
        assert!(m.suspend_threads);
    }

    fn err_of(text: &str) -> ManifestError {
        Manifest::parse(text).unwrap_err()
    }

    #[test]
    fn reports_errors_with_line_numbers() {
        assert_eq!(err_of("\n\nbogus line").line, 3);
        assert!(err_of("mode = fast").msg.contains("mode"));
        assert!(err_of("flush_ms = soon").msg.contains("flush_ms"));
        assert!(err_of("host_sha256 = abcd").msg.contains("64 hex"));
        assert!(err_of("unknown_key = 1").msg.contains("unknown setting"));
        assert!(err_of("hook").msg.contains("expected"));
        assert!(err_of("hook a bytes=558BEC83EC10").msg.contains("addr"));
        assert!(err_of("hook a addr=0x401000").msg.contains("bytes"));
        assert!(err_of("hook a addr=0x401000 bytes=55")
            .msg
            .contains("at least 5"));
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 cc=pascal")
            .msg
            .contains("calling convention"));
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 args=quad")
            .msg
            .contains("argument type"));
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 ret=st0")
            .msg
            .contains("return kind"));
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 color=red")
            .msg
            .contains("unknown hook key"));
        assert!(
            err_of("hook a addr=0x401000 addr=0x401000 bytes=558BEC83EC10")
                .msg
                .contains("twice")
        );
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 dump=ecx")
            .msg
            .contains("length"));
        assert!(err_of("hook a! addr=0x401000 bytes=558BEC83EC10")
            .msg
            .contains("name"));
        assert!(err_of("hook a addr=0x401000 bytes=\"558BEC83EC10")
            .msg
            .contains("quote"));
    }

    #[test]
    fn steal_must_be_valid() {
        // 5 is not a boundary of 55 8B EC 83 EC 10.
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 steal=5")
            .msg
            .contains("boundary"));
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 steal=4")
            .msg
            .contains("jump"));
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC10 steal=9")
            .msg
            .contains("only 6"));
        // A rel8 branch inside the stolen bytes is refused at parse time.
        assert!(
            err_of("hook a addr=0x401000 bytes=85C97406B801000000 steal=5")
                .msg
                .contains("short branch")
        );
        assert!(err_of("hook a addr=0x401000 bytes=85C97406B801000000")
            .msg
            .contains("short branch"));
        // Auto steal needs the bytes to cover a boundary of five.
        assert!(err_of("hook a addr=0x401000 bytes=558BEC83EC")
            .msg
            .contains("instruction"));
    }

    #[test]
    fn names_and_ranges_must_not_collide() {
        let a = "hook a addr=0x401000 bytes=558BEC83EC10\n";
        assert!(
            Manifest::parse(&format!("{a}hook a addr=0x402000 bytes=558BEC83EC10"))
                .unwrap_err()
                .msg
                .contains("twice")
        );
        let overlap =
            Manifest::parse(&format!("{a}hook b addr=0x401004 bytes=558BEC83EC10")).unwrap_err();
        assert!(overlap.msg.contains("overlaps"));
        assert_eq!(overlap.line, 2);
        // Adjacent ranges are fine.
        assert!(Manifest::parse(&format!("{a}hook b addr=0x401006 bytes=558BEC83EC10")).is_ok());
    }

    #[test]
    fn peek_reads_mode_despite_other_errors() {
        let text = "mode = inject\nnvse_grace_ms = 40\nhook broken addr=zzz\nunknown = 1\n";
        assert!(Manifest::parse(text).is_err());
        assert_eq!(peek_settings(text), (Mode::Inject, 40));
        assert_eq!(peek_settings(""), (Mode::Auto, 1500));
        assert_eq!(
            peek_settings("# mode = inject\nMODE = NVSE"),
            (Mode::Nvse, 1500)
        );
        assert_eq!(
            peek_settings("mode = banana\nnvse_grace_ms = x"),
            (Mode::Auto, 1500)
        );
    }

    #[test]
    fn hook_prefix_is_not_confused_with_a_setting() {
        // A setting that merely starts with "hook" is still an error, not a hook.
        assert!(err_of("hooks = 1").msg.contains("unknown setting"));
    }
}
