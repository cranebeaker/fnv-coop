//! The vector and result formats of `nv-call` (JSON lines).
//!
//! # Vector line
//!
//! ```text
//! {"id":"lerp_1","fn":"0x00401230","cc":"cdecl",
//!  "args":["f32:1.0","f32:2.0","f32:0.25"],
//!  "buffers":{"obj":{"size":32,"init":"0100000002000000"}},
//!  "ret":"f32_x87","fpcw":"0x027F","mxcsr":"0x1F80",
//!  "regs":{"eax":"0x1"}}
//! ```
//!
//! - `id` (required): a name copied into the result.
//! - `fn` (required): the function address, a hex string or a number.
//! - `cc`: `cdecl` (default), `stdcall`, `thiscall` or `fastcall`.
//! - `args`: strings written `kind:value`, placed by the convention (see
//!   [`crate::abi`]; for `thiscall` the first one is `this`):
//!   `i8 u8 i16 u16 i32 u32 bool i64 u64` take a number (decimal or `0x`
//!   hex, negative allowed); `f32` and `f64` take a decimal float, or `0x`
//!   and the exact bit pattern (`f32:0x3fc00000`); `ptr:NAME[+off]` is the
//!   address of a named buffer, and `ptr:0x1234` or `ptr:null` a literal
//!   address; `raw:0x3f800000[,0x1...]` pushes raw dwords on the stack as
//!   they are, even one dword under `fastcall` (it never goes in ECX or EDX,
//!   and cannot be the `this` of `thiscall`; use `u32:` or `ptr:` for those).
//! - `buffers`: named memory blocks, each `{"size": n, "init": "hex"}`,
//!   zero-filled beyond `init`. Fresh for every vector and reported after
//!   the call.
//! - `ret`: what the function returns (default `i32`); decides whether ST0
//!   is captured.
//! - `fpcw`, `mxcsr`: control words set before the call (defaults come from
//!   the command line, normally `0x027F` and `0x1F80`).
//! - `regs`: extra register values set before the call, for functions with
//!   custom conventions: `eax ecx edx ebx esi edi` and `xmm0`..`xmm3` (hex
//!   bytes in memory order, at most 16).
//!
//! # Result line
//!
//! One JSON object per vector with `type` `"result"`, the vector `id`, the
//! inputs echoed, `fault` (null or an object), `regs` (null after a fault),
//! and `buffers` as hex after the call. See [`CallResult::to_json`].

use crate::abi::{layout, ArgShape, ArgType, CallConv, Layout, RetKind};
use crate::hex;
use crate::json::{self, ObjectWriter, Value};
use crate::x87::Ext80;

pub const DEFAULT_FPCW: u16 = 0x027f;
pub const DEFAULT_MXCSR: u32 = 0x1f80;
pub const MAX_BUFFER: usize = 16 * 1024 * 1024;
pub const MAX_ARGS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferSpec {
    pub name: String,
    pub size: usize,
    pub init: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgValue {
    /// An integer-like value. `bits` holds the value as a 64-bit pattern
    /// (sign-extended for signed types).
    Int {
        ty: ArgType,
        bits: u64,
    },
    F32(u32),
    F64(u64),
    /// Address of a buffer plus a byte offset.
    Buffer {
        name: String,
        offset: i32,
    },
    /// A literal address.
    Address(u32),
    /// Raw dwords, always on the stack, in the order given.
    Raw(Vec<u32>),
}

impl ArgValue {
    pub fn shape(&self) -> ArgShape {
        match self {
            ArgValue::Int { ty, .. } => ty.shape(),
            ArgValue::F32(_) => ArgType::F32.shape(),
            ArgValue::F64(_) => ArgType::F64.shape(),
            ArgValue::Buffer { .. } | ArgValue::Address(_) => ArgType::Ptr.shape(),
            // Raw dwords are pushed as they are. They never take ECX or EDX,
            // not even a single one (a float bit pattern or a small struct
            // passed by value goes on the stack under every convention);
            // `u32:` and `ptr:` are the arguments that follow the register rules.
            ArgValue::Raw(d) => ArgShape {
                dwords: d.len(),
                int_like: false,
            },
        }
    }

    /// The dwords this argument occupies, low dword first. `resolve` maps a
    /// buffer name to its address.
    pub fn dwords(&self, resolve: &dyn Fn(&str) -> Option<u32>) -> Result<Vec<u32>, String> {
        Ok(match self {
            ArgValue::Int { ty, bits } => match ty.dwords() {
                2 => vec![*bits as u32, (*bits >> 32) as u32],
                _ => vec![*bits as u32],
            },
            ArgValue::F32(b) => vec![*b],
            ArgValue::F64(b) => vec![*b as u32, (*b >> 32) as u32],
            ArgValue::Buffer { name, offset } => {
                let base = resolve(name).ok_or_else(|| format!("unknown buffer {name:?}"))?;
                vec![base.wrapping_add(*offset as u32)]
            }
            ArgValue::Address(a) => vec![*a],
            ArgValue::Raw(d) => d.clone(),
        })
    }

    fn to_text(&self) -> String {
        match self {
            ArgValue::Int { ty, bits } => match ty {
                ArgType::I8 | ArgType::I16 | ArgType::I32 | ArgType::I64 => {
                    format!("{}:{}", ty.name(), *bits as i64)
                }
                _ => format!("{}:{}", ty.name(), bits),
            },
            ArgValue::F32(b) => format!("f32:0x{b:08x}"),
            ArgValue::F64(b) => format!("f64:0x{b:016x}"),
            ArgValue::Buffer { name, offset } if *offset == 0 => format!("ptr:{name}"),
            ArgValue::Buffer { name, offset } => format!("ptr:{name}{offset:+}"),
            ArgValue::Address(a) => format!("ptr:0x{a:08x}"),
            ArgValue::Raw(d) => {
                let parts: Vec<String> = d.iter().map(|x| format!("0x{x:08x}")).collect();
                format!("raw:{}", parts.join(","))
            }
        }
    }
}

fn parse_float_bits(text: &str, digits: usize) -> Result<Option<u64>, String> {
    if let Some(h) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        if h.is_empty() || h.len() > digits {
            return Err(format!(
                "float bit pattern needs 1 to {digits} hex digits: {text:?}"
            ));
        }
        return u64::from_str_radix(h, 16)
            .map(Some)
            .map_err(|_| format!("bad hex in {text:?}"));
    }
    Ok(None)
}

fn is_buffer_name(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
}

/// Parse one `kind:value` argument.
pub fn parse_arg(text: &str) -> Result<ArgValue, String> {
    let (kind, value) = text
        .split_once(':')
        .ok_or_else(|| format!("argument {text:?} must be kind:value"))?;
    let value = value.trim();
    match kind.trim().to_ascii_lowercase().as_str() {
        "raw" => {
            let mut d = Vec::new();
            for tok in value.split(',') {
                let t = tok.trim();
                let h = t
                    .strip_prefix("0x")
                    .or_else(|| t.strip_prefix("0X"))
                    .unwrap_or(t);
                if h.is_empty() || h.len() > 8 {
                    return Err(format!("raw dword needs 1 to 8 hex digits: {tok:?}"));
                }
                d.push(u32::from_str_radix(h, 16).map_err(|_| format!("bad raw dword {tok:?}"))?);
            }
            Ok(ArgValue::Raw(d))
        }
        "f32" | "float" => match parse_float_bits(value, 8)? {
            Some(b) => Ok(ArgValue::F32(b as u32)),
            None => value
                .parse::<f32>()
                .map(|f| ArgValue::F32(f.to_bits()))
                .map_err(|_| format!("bad f32 value {value:?}")),
        },
        "f64" | "double" => match parse_float_bits(value, 16)? {
            Some(b) => Ok(ArgValue::F64(b)),
            None => value
                .parse::<f64>()
                .map(|f| ArgValue::F64(f.to_bits()))
                .map_err(|_| format!("bad f64 value {value:?}")),
        },
        "ptr" => {
            if value.eq_ignore_ascii_case("null") {
                return Ok(ArgValue::Address(0));
            }
            if value.as_bytes().first().is_some_and(u8::is_ascii_digit) {
                return Ok(ArgValue::Address(hex::parse_u32(value)?));
            }
            let split = value.find(['+', '-']).unwrap_or(value.len());
            let name = &value[..split];
            if !is_buffer_name(name) {
                return Err(format!("bad buffer name in {text:?}"));
            }
            let offset = if split < value.len() {
                hex::parse_i64(&value[split..])?
            } else {
                0
            };
            let offset = i32::try_from(offset)
                .map_err(|_| format!("buffer offset out of range in {text:?}"))?;
            Ok(ArgValue::Buffer {
                name: name.to_string(),
                offset,
            })
        }
        other => {
            let ty = ArgType::parse(other)?;
            let int = |signed: bool, bits: u32| -> Result<u64, String> {
                // Parse through i128 so every u64 and i64 value fits.
                let (neg, digits) = match value.strip_prefix('-') {
                    Some(r) => (true, r),
                    None => (false, value.strip_prefix('+').unwrap_or(value)),
                };
                let mag = i128::from(hex::parse_u64(digits)?);
                let mut v = if neg { -mag } else { mag };
                if !signed && ty == ArgType::U32 && (-(1i128 << 31)..0).contains(&v) {
                    // `-1` is a common way to write 0xFFFFFFFF.
                    v += 1i128 << 32;
                }
                let (lo, hi) = if signed {
                    (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)
                } else {
                    (0, (1i128 << bits) - 1)
                };
                if v < lo || v > hi {
                    return Err(format!("{value:?} does not fit {}", ty.name()));
                }
                Ok(v as i64 as u64)
            };
            let bits = match ty {
                ArgType::I8 => int(true, 8)?,
                ArgType::U8 => int(false, 8)?,
                ArgType::I16 => int(true, 16)?,
                ArgType::U16 => int(false, 16)?,
                ArgType::I32 => int(true, 32)?,
                ArgType::U32 => int(false, 32)?,
                ArgType::I64 => int(true, 64)?,
                ArgType::U64 => int(false, 64)?,
                ArgType::Bool => match value.to_ascii_lowercase().as_str() {
                    "0" | "false" => 0,
                    "1" | "true" => 1,
                    _ => return Err(format!("bool must be 0, 1, true or false: {value:?}")),
                },
                ArgType::Ptr => unreachable!("handled above"),
                ArgType::F32 | ArgType::F64 => unreachable!("handled above"),
            };
            Ok(ArgValue::Int { ty, bits })
        }
    }
}

/// Registers that can be preset for functions with custom conventions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegPresets {
    pub eax: Option<u32>,
    pub ecx: Option<u32>,
    pub edx: Option<u32>,
    pub ebx: Option<u32>,
    pub esi: Option<u32>,
    pub edi: Option<u32>,
    pub xmm: [Option<[u8; 16]>; 4],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vector {
    pub id: String,
    pub addr: u32,
    pub cc: CallConv,
    pub args: Vec<ArgValue>,
    pub buffers: Vec<BufferSpec>,
    pub ret: RetKind,
    pub fpcw: Option<u16>,
    pub mxcsr: Option<u32>,
    pub regs: RegPresets,
}

fn hex_or_int(v: &Value, what: &str) -> Result<u32, String> {
    match v {
        Value::Str(s) => hex::parse_u32(s).map_err(|e| format!("{what}: {e}")),
        Value::Int(i) => u32::try_from(*i).map_err(|_| format!("{what}: {i} does not fit 32 bits")),
        _ => Err(format!("{what} must be a hex string or a number")),
    }
}

impl Vector {
    /// Parse one vector line.
    pub fn parse(line: &str) -> Result<Vector, String> {
        let v = Value::parse(line).map_err(|e| e.to_string())?;
        let members = v.as_object().ok_or("a vector must be a JSON object")?;
        for (k, _) in members {
            if !matches!(
                k.as_str(),
                "id" | "fn"
                    | "cc"
                    | "args"
                    | "buffers"
                    | "ret"
                    | "fpcw"
                    | "mxcsr"
                    | "regs"
                    | "note"
            ) {
                return Err(format!("unknown vector field {k:?}"));
            }
        }
        let id = v
            .get("id")
            .and_then(Value::as_str)
            .ok_or("vector needs a string \"id\"")?
            .to_string();
        if id.is_empty() {
            return Err("vector id is empty".into());
        }
        let addr = hex_or_int(v.get("fn").ok_or("vector needs \"fn\"")?, "fn")?;
        let cc = match v.get("cc") {
            Some(c) => CallConv::parse(c.as_str().ok_or("cc must be a string")?)?,
            None => CallConv::Cdecl,
        };
        let ret = match v.get("ret") {
            Some(r) => RetKind::parse(r.as_str().ok_or("ret must be a string")?)?,
            None => RetKind::I32,
        };
        let mut args = Vec::new();
        if let Some(a) = v.get("args") {
            for item in a.as_array().ok_or("args must be an array of strings")? {
                args.push(parse_arg(
                    item.as_str()
                        .ok_or("each arg must be a string like \"i32:5\"")?,
                )?);
            }
        }
        if args.len() > MAX_ARGS {
            return Err(format!("too many arguments (limit {MAX_ARGS})"));
        }
        let mut buffers = Vec::new();
        if let Some(b) = v.get("buffers") {
            for (name, spec) in b.as_object().ok_or("buffers must be an object")? {
                if !is_buffer_name(name) {
                    return Err(format!("bad buffer name {name:?}"));
                }
                let size = spec
                    .get("size")
                    .and_then(Value::as_i128)
                    .ok_or_else(|| format!("buffer {name:?} needs a numeric size"))?;
                let size =
                    usize::try_from(size).map_err(|_| format!("buffer {name:?} has a bad size"))?;
                if size > MAX_BUFFER {
                    return Err(format!("buffer {name:?} is larger than {MAX_BUFFER} bytes"));
                }
                let init = match spec.get("init") {
                    Some(h) => hex::decode_lenient(h.as_str().ok_or("init must be a hex string")?)?,
                    None => Vec::new(),
                };
                if init.len() > size {
                    return Err(format!("buffer {name:?}: init is longer than size"));
                }
                buffers.push(BufferSpec {
                    name: name.clone(),
                    size,
                    init,
                });
            }
        }
        for a in &args {
            if let ArgValue::Buffer { name, .. } = a {
                if !buffers.iter().any(|b| &b.name == name) {
                    return Err(format!("argument refers to unknown buffer {name:?}"));
                }
            }
        }
        let fpcw = match v.get("fpcw") {
            Some(x) => Some(
                u16::try_from(hex_or_int(x, "fpcw")?)
                    .map_err(|_| "fpcw must fit 16 bits".to_string())?,
            ),
            None => None,
        };
        let mxcsr = match v.get("mxcsr") {
            Some(x) => Some(hex_or_int(x, "mxcsr")?),
            None => None,
        };
        let mut regs = RegPresets::default();
        if let Some(r) = v.get("regs") {
            for (name, val) in r.as_object().ok_or("regs must be an object")? {
                match name.as_str() {
                    "eax" => regs.eax = Some(hex_or_int(val, name)?),
                    "ecx" => regs.ecx = Some(hex_or_int(val, name)?),
                    "edx" => regs.edx = Some(hex_or_int(val, name)?),
                    "ebx" => regs.ebx = Some(hex_or_int(val, name)?),
                    "esi" => regs.esi = Some(hex_or_int(val, name)?),
                    "edi" => regs.edi = Some(hex_or_int(val, name)?),
                    x if x.len() == 4
                        && x.starts_with("xmm")
                        && matches!(x.as_bytes()[3], b'0'..=b'3') =>
                    {
                        let bytes =
                            hex::decode_lenient(val.as_str().ok_or("xmm values are hex strings")?)?;
                        if bytes.len() > 16 {
                            return Err(format!("{name} has more than 16 bytes"));
                        }
                        let mut full = [0u8; 16];
                        full[..bytes.len()].copy_from_slice(&bytes);
                        regs.xmm[usize::from(x.as_bytes()[3] - b'0')] = Some(full);
                    }
                    other => return Err(format!("unknown register {other:?} in regs")),
                }
            }
        }
        let vector = Vector {
            id,
            addr,
            cc,
            args,
            buffers,
            ret,
            fpcw,
            mxcsr,
            regs,
        };
        vector.layout()?;
        Ok(vector)
    }

    /// Where the arguments go for this vector's convention.
    pub fn layout(&self) -> Result<Layout, String> {
        let shapes: Vec<ArgShape> = self.args.iter().map(ArgValue::shape).collect();
        layout(self.cc, &shapes)
    }

    /// Serialise back to one line (used by tools that generate vectors).
    pub fn to_json(&self) -> String {
        let mut o = ObjectWriter::new();
        o.str("id", &self.id)
            .hex32("fn", self.addr)
            .str("cc", self.cc.name());
        let args: Vec<String> = self
            .args
            .iter()
            .map(|a| {
                let mut s = String::new();
                json::push_str(&mut s, &a.to_text());
                s
            })
            .collect();
        o.raw("args", &json::array_of(&args));
        let mut b = ObjectWriter::new();
        for buf in &self.buffers {
            let mut spec = ObjectWriter::new();
            spec.u64("size", buf.size as u64)
                .hex_bytes("init", &buf.init);
            b.raw(&buf.name, &spec.finish());
        }
        o.raw("buffers", &b.finish());
        o.str("ret", self.ret.name());
        if let Some(f) = self.fpcw {
            o.hex32("fpcw", u32::from(f));
        }
        if let Some(m) = self.mxcsr {
            o.hex32("mxcsr", m);
        }
        let mut r = ObjectWriter::new();
        for (name, val) in [
            ("eax", self.regs.eax),
            ("ecx", self.regs.ecx),
            ("edx", self.regs.edx),
            ("ebx", self.regs.ebx),
            ("esi", self.regs.esi),
            ("edi", self.regs.edi),
        ] {
            if let Some(x) = val {
                r.hex32(name, x);
            }
        }
        for (i, x) in self.regs.xmm.iter().enumerate() {
            if let Some(bytes) = x {
                r.hex_bytes(&format!("xmm{i}"), bytes);
            }
        }
        let regs = r.finish();
        if regs != "{}" {
            o.raw("regs", &regs);
        }
        o.finish()
    }
}

/// Parse a whole vectors file: one vector per line, blank lines and lines
/// starting with `#` skipped. Each entry carries its 1-based line number, so
/// one bad line is reported without losing the rest.
pub fn parse_file(text: &str) -> Vec<(usize, Result<Vector, String>)> {
    text.lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .map(|(i, l)| (i + 1, Vector::parse(l.trim())))
        .collect()
}

/// Serialise vectors as a JSON-lines file.
pub fn to_file(vectors: &[Vector]) -> String {
    let mut out = String::new();
    for v in vectors {
        out.push_str(&v.to_json());
        out.push('\n');
    }
    out
}

/// Exception codes the harness reports by name. Any other code is reported
/// as a fault too, just under the name `exception`.
pub fn exception_name(code: u32) -> &'static str {
    match code {
        0xC000_0005 => "access_violation",
        0xC000_0006 => "in_page_error",
        0xC000_001D => "illegal_instruction",
        0xC000_0025 => "noncontinuable_exception",
        0xC000_0026 => "invalid_disposition",
        0xC000_0094 => "integer_divide_by_zero",
        0xC000_0095 => "integer_overflow",
        0xC000_0096 => "privileged_instruction",
        0xC000_008C => "array_bounds_exceeded",
        0xC000_008D => "float_denormal_operand",
        0xC000_008E => "float_divide_by_zero",
        0xC000_008F => "float_inexact_result",
        0xC000_0090 => "float_invalid_operation",
        0xC000_0091 => "float_overflow",
        0xC000_0092 => "float_stack_check",
        0xC000_0093 => "float_underflow",
        0xC000_00FD => "stack_overflow",
        0xC000_02B4 => "float_multiple_faults",
        0xC000_02B5 => "float_multiple_traps",
        0x8000_0001 => "guard_page_violation",
        0x8000_0002 => "datatype_misalignment",
        0x8000_0003 => "breakpoint",
        0x8000_0004 => "single_step",
        // What MSVC's `throw` raises (code is the ASCII of "msc" plus 0xE0).
        0xE06D_7363 => "cpp_exception",
        _ => "exception",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    /// A CPU exception raised by the called code.
    Exception {
        code: u32,
        eip: u32,
        /// For access violations: the address touched and how.
        address: Option<u32>,
        access: Option<&'static str>,
    },
    /// The code called an unresolved import; the call was abandoned.
    ImportTrap {
        index: usize,
        name: String,
        caller: u32,
    },
}

impl Fault {
    pub fn to_json(&self) -> String {
        let mut o = ObjectWriter::new();
        match self {
            Fault::Exception {
                code,
                eip,
                address,
                access,
            } => {
                o.str("kind", "exception")
                    .hex32("code", *code)
                    .str("name", exception_name(*code))
                    .hex32("eip", *eip);
                if let Some(a) = address {
                    o.hex32("address", *a);
                }
                if let Some(a) = access {
                    o.str("access", a);
                }
            }
            Fault::ImportTrap {
                index,
                name,
                caller,
            } => {
                o.str("kind", "import")
                    .str("name", name)
                    .u64("index", *index as u64)
                    .hex32("caller", *caller);
            }
        }
        o.finish()
    }
}

/// CPU state captured after a call that returned normally.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegsOut {
    pub eax: u32,
    pub edx: u32,
    /// Raw ST0, captured only for x87 return kinds with a non-empty stack.
    pub st0: Option<[u8; 10]>,
    /// How many x87 registers the callee left in use (0 is clean).
    pub fpu_depth: u32,
    pub xmm0: [u8; 16],
    pub fsw: u16,
    pub fcw: u16,
    pub mxcsr: u32,
    /// Bytes of arguments the callee removed with `ret N` (0 for cdecl).
    pub callee_popped: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallResult {
    pub id: String,
    pub addr: u32,
    pub cc: CallConv,
    pub ret: RetKind,
    pub fpcw: u16,
    pub mxcsr: u32,
    pub fault: Option<Fault>,
    pub regs: Option<RegsOut>,
    /// Bytes of stack arguments the call pushed.
    pub stack_bytes: u32,
    pub buffers: Vec<(String, Vec<u8>)>,
}

fn float_fields(o: &mut ObjectWriter, f32_bits: Option<u32>, f64_bits: Option<u64>) {
    if let Some(b) = f32_bits {
        o.hex32("f32_bits", b)
            .f64("f32", f64::from(f32::from_bits(b)));
    }
    if let Some(b) = f64_bits {
        o.raw("f64_bits", &format!("\"0x{b:016X}\""))
            .f64("f64", f64::from_bits(b));
    }
}

impl CallResult {
    pub fn to_json(&self) -> String {
        let mut o = ObjectWriter::new();
        o.str("type", "result")
            .str("id", &self.id)
            .hex32("fn", self.addr)
            .str("cc", self.cc.name());
        o.str("ret", self.ret.name())
            .hex32("fpcw_in", u32::from(self.fpcw))
            .hex32("mxcsr_in", self.mxcsr);
        match &self.fault {
            Some(f) => o.raw("fault", &f.to_json()),
            None => o.null("fault"),
        };
        match &self.regs {
            None => {
                o.null("regs");
            }
            Some(r) => {
                let mut g = ObjectWriter::new();
                g.hex32("eax", r.eax).hex32("edx", r.edx);
                match &r.st0 {
                    Some(raw) => {
                        let e = Ext80::from_bytes(*raw);
                        let mut s = ObjectWriter::new();
                        s.str("raw", &e.to_hex());
                        float_fields(&mut s, Some(e.to_f32_bits()), Some(e.to_f64_bits()));
                        g.raw("st0", &s.finish());
                    }
                    None => {
                        g.null("st0");
                    }
                }
                g.u64("fpu_depth", u64::from(r.fpu_depth));
                g.hex_bytes("xmm0", &r.xmm0);
                g.hex32("fsw", u32::from(r.fsw))
                    .hex32("fcw", u32::from(r.fcw))
                    .hex32("mxcsr", r.mxcsr);
                g.u64("callee_popped", u64::from(r.callee_popped));
                let cleanup = if self.stack_bytes == 0 {
                    "none"
                } else if r.callee_popped == 0 {
                    "caller"
                } else if r.callee_popped == self.stack_bytes {
                    "callee"
                } else {
                    "other"
                };
                g.str("cleanup", cleanup);
                o.raw("regs", &g.finish());
                o.raw("value", &self.value_json(r));
            }
        }
        o.u64("stack_bytes", u64::from(self.stack_bytes));
        let mut b = ObjectWriter::new();
        for (name, bytes) in &self.buffers {
            b.hex_bytes(name, bytes);
        }
        o.raw("buffers", &b.finish());
        o.finish()
    }

    /// The return value interpreted according to the declared kind.
    fn value_json(&self, r: &RegsOut) -> String {
        let mut v = ObjectWriter::new();
        match self.ret {
            RetKind::Void => return "null".to_string(),
            RetKind::I32 => {
                v.i64("i32", i64::from(r.eax as i32))
                    .u64("u32", u64::from(r.eax));
            }
            RetKind::U32 => {
                v.u64("u32", u64::from(r.eax));
            }
            RetKind::Ptr => {
                v.hex32("ptr", r.eax);
            }
            RetKind::I64 => {
                let x = (u64::from(r.edx) << 32) | u64::from(r.eax);
                v.i64("i64", x as i64)
                    .raw("u64", &format!("\"0x{x:016X}\""));
            }
            RetKind::F32X87 | RetKind::F64X87 => match &r.st0 {
                Some(raw) => {
                    let e = Ext80::from_bytes(*raw);
                    if self.ret == RetKind::F32X87 {
                        float_fields(&mut v, Some(e.to_f32_bits()), None);
                    } else {
                        float_fields(&mut v, None, Some(e.to_f64_bits()));
                    }
                }
                None => {
                    v.str("error", "x87 return kind but the FPU stack was empty");
                }
            },
            RetKind::Xmm0 => {
                let lo32 = u32::from_le_bytes([r.xmm0[0], r.xmm0[1], r.xmm0[2], r.xmm0[3]]);
                let mut lo = [0u8; 8];
                lo.copy_from_slice(&r.xmm0[..8]);
                float_fields(&mut v, Some(lo32), Some(u64::from_le_bytes(lo)));
            }
        }
        v.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(ty: ArgType, bits: u64) -> ArgValue {
        ArgValue::Int { ty, bits }
    }

    #[test]
    fn parses_argument_kinds() {
        assert_eq!(
            parse_arg("i32:-5").unwrap(),
            int(ArgType::I32, (-5i64) as u64)
        );
        assert_eq!(
            parse_arg("u32:0xFFFFFFFF").unwrap(),
            int(ArgType::U32, 0xffff_ffff)
        );
        assert_eq!(parse_arg("u32:-1").unwrap(), int(ArgType::U32, 0xffff_ffff));
        assert_eq!(
            parse_arg("i8:-128").unwrap(),
            int(ArgType::I8, (-128i64) as u64)
        );
        assert_eq!(parse_arg("u16:65535").unwrap(), int(ArgType::U16, 65535));
        assert_eq!(parse_arg("i64:-1").unwrap(), int(ArgType::I64, u64::MAX));
        assert_eq!(
            parse_arg("u64:0xFFFFFFFFFFFFFFFF").unwrap(),
            int(ArgType::U64, u64::MAX)
        );
        assert_eq!(parse_arg("bool:true").unwrap(), int(ArgType::Bool, 1));
        assert_eq!(
            parse_arg("f32:1.5").unwrap(),
            ArgValue::F32(1.5f32.to_bits())
        );
        assert_eq!(
            parse_arg("f32:0x3fc00000").unwrap(),
            ArgValue::F32(0x3fc0_0000)
        );
        assert_eq!(
            parse_arg("f64:-2.5").unwrap(),
            ArgValue::F64((-2.5f64).to_bits())
        );
        assert_eq!(
            parse_arg("f64:0x7ff8000000000001").unwrap(),
            ArgValue::F64(0x7ff8_0000_0000_0001)
        );
        assert_eq!(parse_arg("ptr:null").unwrap(), ArgValue::Address(0));
        assert_eq!(
            parse_arg("ptr:0x011dea3c").unwrap(),
            ArgValue::Address(0x011d_ea3c)
        );
        assert_eq!(
            parse_arg("ptr:obj").unwrap(),
            ArgValue::Buffer {
                name: "obj".into(),
                offset: 0
            }
        );
        assert_eq!(
            parse_arg("ptr:obj+0x10").unwrap(),
            ArgValue::Buffer {
                name: "obj".into(),
                offset: 16
            }
        );
        assert_eq!(
            parse_arg("ptr:obj-4").unwrap(),
            ArgValue::Buffer {
                name: "obj".into(),
                offset: -4
            }
        );
        assert_eq!(
            parse_arg("raw:0x3f800000,1").unwrap(),
            ArgValue::Raw(vec![0x3f80_0000, 1])
        );
    }

    #[test]
    fn rejects_bad_arguments() {
        for bad in [
            "5",
            "i32:",
            "i32:abc",
            "i32:2147483648",
            "i32:-2147483649",
            "u32:4294967296",
            "i8:128",
            "u8:-1",
            "u8:256",
            "bool:2",
            "f32:x",
            "f32:0x123456789",
            "f64:0x",
            "ptr:",
            "ptr:a b",
            "raw:",
            "raw:123456789",
            "raw:zz",
            "quad:1",
            "i64:9223372036854775808",
            "u64:18446744073709551616",
        ] {
            assert!(parse_arg(bad).is_err(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn dwords_of_arguments() {
        let r = |_: &str| Some(0x3000_0000u32);
        assert_eq!(
            int(ArgType::I8, (-1i64) as u64).dwords(&r).unwrap(),
            [0xffff_ffff]
        );
        assert_eq!(
            int(ArgType::I64, 0x1122_3344_5566_7788).dwords(&r).unwrap(),
            [0x5566_7788, 0x1122_3344]
        );
        assert_eq!(
            ArgValue::F64(0x4000_0000_0000_0000).dwords(&r).unwrap(),
            [0, 0x4000_0000]
        );
        assert_eq!(
            ArgValue::Buffer {
                name: "b".into(),
                offset: -4
            }
            .dwords(&r)
            .unwrap(),
            [0x2fff_fffc]
        );
        assert!(ArgValue::Buffer {
            name: "b".into(),
            offset: 0
        }
        .dwords(&|_| None)
        .is_err());
    }

    const SAMPLE: &str = r#"{"id":"lerp_1","fn":"0x00401230","cc":"thiscall","args":["ptr:obj","f32:1.0","i32:-3"],"buffers":{"obj":{"size":8,"init":"0100"}},"ret":"f32_x87","fpcw":"0x037F","mxcsr":8064,"regs":{"esi":"0x5","xmm1":"0000803f"}}"#;

    #[test]
    fn parses_a_vector() {
        let v = Vector::parse(SAMPLE).unwrap();
        assert_eq!(
            (v.id.as_str(), v.addr, v.cc, v.ret),
            ("lerp_1", 0x40_1230, CallConv::Thiscall, RetKind::F32X87)
        );
        assert_eq!(v.fpcw, Some(0x37f));
        assert_eq!(v.mxcsr, Some(8064));
        assert_eq!(v.buffers[0].init, [1, 0]);
        assert_eq!(v.buffers[0].size, 8);
        assert_eq!(v.regs.esi, Some(5));
        assert_eq!(v.regs.xmm[1].unwrap()[..4], [0, 0, 0x80, 0x3f]);
        let l = v.layout().unwrap();
        assert_eq!(l.stack_dwords, 2);
    }

    #[test]
    fn defaults_for_optional_fields() {
        let v = Vector::parse(r#"{"id":"a","fn":4198400}"#).unwrap();
        assert_eq!(
            (v.addr, v.cc, v.ret, v.fpcw, v.mxcsr),
            (0x40_1000, CallConv::Cdecl, RetKind::I32, None, None)
        );
        assert!(v.args.is_empty() && v.buffers.is_empty());
    }

    #[test]
    fn raw_dwords_always_go_on_the_stack() {
        use crate::abi::Place;
        let v = Vector::parse(
            r#"{"id":"r","fn":"0x1000","cc":"fastcall","args":["raw:0x3f800000","u32:5","u32:6"]}"#,
        )
        .unwrap();
        // The single raw dword is on the stack; the integers take ECX and EDX.
        assert_eq!(
            v.layout().unwrap().places,
            [Place::Stack(0), Place::Ecx, Place::Edx]
        );
        // A raw value cannot stand in for `this`.
        assert!(
            Vector::parse(r#"{"id":"r","fn":"0x1000","cc":"thiscall","args":["raw:0x1000"]}"#)
                .is_err()
        );
        // Under cdecl it is one stack slot, as before.
        let v = Vector::parse(r#"{"id":"r","fn":"0x1000","args":["raw:1,2","i32:3"]}"#).unwrap();
        assert_eq!(v.layout().unwrap().stack_dwords, 3);
    }

    #[test]
    fn writer_and_parser_agree() {
        let v = Vector::parse(SAMPLE).unwrap();
        let again = Vector::parse(&v.to_json()).unwrap();
        assert_eq!(again, v);
        let plain = Vector::parse(r#"{"id":"p","fn":"0x1000","args":["u64:7","f64:0.1","ptr:0x1234","raw:1,2"],"ret":"i64"}"#).unwrap();
        assert_eq!(Vector::parse(&plain.to_json()).unwrap(), plain);
    }

    #[test]
    fn rejects_bad_vectors() {
        for bad in [
            "[]",
            r#"{"fn":"0x1000"}"#,
            r#"{"id":"a"}"#,
            r#"{"id":"","fn":"0x1000"}"#,
            r#"{"id":"a","fn":"0x1000","cc":"pascal"}"#,
            r#"{"id":"a","fn":"0x1000","ret":"st0"}"#,
            r#"{"id":"a","fn":"0x1000","args":"i32:1"}"#,
            r#"{"id":"a","fn":"0x1000","args":["ptr:nope"]}"#,
            r#"{"id":"a","fn":"0x1000","buffers":{"b":{"size":2,"init":"010203"}}}"#,
            r#"{"id":"a","fn":"0x1000","buffers":{"b":{"size":999999999}}}"#,
            r#"{"id":"a","fn":"0x1000","typo":1}"#,
            r#"{"id":"a","fn":"0x1000","regs":{"eip":"0x1"}}"#,
            r#"{"id":"a","fn":"0x1000","regs":{"xmm0":"000000000000000000000000000000000000"}}"#,
            r#"{"id":"a","fn":"0x1000","fpcw":"0x10000"}"#,
            r#"{"id":"a","fn":"0x1000","cc":"thiscall","args":["f32:1"]}"#,
            "not json",
        ] {
            assert!(Vector::parse(bad).is_err(), "{bad}");
        }
    }

    fn sample_result() -> CallResult {
        CallResult {
            id: "r".into(),
            addr: 0x40_1000,
            cc: CallConv::Stdcall,
            ret: RetKind::F64X87,
            fpcw: 0x27f,
            mxcsr: 0x1f80,
            fault: None,
            regs: Some(RegsOut {
                eax: 0xdead_beef,
                edx: 1,
                st0: Some(Ext80::from_f64(1.5).to_bytes()),
                fpu_depth: 1,
                xmm0: [0; 16],
                fsw: 0x3800,
                fcw: 0x27f,
                mxcsr: 0x1f80,
                callee_popped: 8,
            }),
            stack_bytes: 8,
            buffers: vec![("obj".into(), vec![1, 2, 3])],
        }
    }

    #[test]
    fn result_json_fields() {
        let line = sample_result().to_json();
        let v = Value::parse(&line).unwrap();
        assert_eq!(v.get("type").unwrap().as_str(), Some("result"));
        assert!(matches!(v.get("fault"), Some(Value::Null)));
        let regs = v.get("regs").unwrap();
        assert_eq!(regs.get("eax").unwrap().as_str(), Some("0xDEADBEEF"));
        assert_eq!(regs.get("cleanup").unwrap().as_str(), Some("callee"));
        let st0 = regs.get("st0").unwrap();
        assert_eq!(st0.get("f64").unwrap().as_f64(), Some(1.5));
        assert_eq!(
            st0.get("f64_bits").unwrap().as_str(),
            Some("0x3FF8000000000000")
        );
        assert_eq!(st0.get("raw").unwrap().as_str().unwrap().len(), 20);
        let value = v.get("value").unwrap();
        assert_eq!(value.get("f64").unwrap().as_f64(), Some(1.5));
        assert_eq!(
            v.get("buffers").unwrap().get("obj").unwrap().as_str(),
            Some("010203")
        );
        assert!(!line.contains('\n'));
    }

    #[test]
    fn result_value_kinds() {
        let mut r = sample_result();
        r.ret = RetKind::I32;
        r.regs.as_mut().unwrap().eax = 0xffff_fffe;
        let v = Value::parse(&r.to_json()).unwrap();
        assert_eq!(
            v.get("value").unwrap().get("i32").unwrap().as_i128(),
            Some(-2)
        );
        r.ret = RetKind::I64;
        r.regs.as_mut().unwrap().edx = 0xffff_ffff;
        r.regs.as_mut().unwrap().eax = 0xffff_fff0;
        let v = Value::parse(&r.to_json()).unwrap();
        assert_eq!(
            v.get("value").unwrap().get("i64").unwrap().as_i128(),
            Some(-16)
        );
        r.ret = RetKind::Void;
        assert!(matches!(
            Value::parse(&r.to_json()).unwrap().get("value"),
            Some(Value::Null)
        ));
        r.ret = RetKind::Xmm0;
        r.regs.as_mut().unwrap().xmm0[..4].copy_from_slice(&1.0f32.to_le_bytes());
        let v = Value::parse(&r.to_json()).unwrap();
        assert_eq!(
            v.get("value").unwrap().get("f32_bits").unwrap().as_str(),
            Some("0x3F800000")
        );
        r.ret = RetKind::F32X87;
        r.regs.as_mut().unwrap().st0 = None;
        let v = Value::parse(&r.to_json()).unwrap();
        assert!(v.get("value").unwrap().get("error").is_some());
    }

    #[test]
    fn result_cleanup_classification() {
        let mut r = sample_result();
        r.regs.as_mut().unwrap().callee_popped = 0;
        assert_eq!(
            Value::parse(&r.to_json())
                .unwrap()
                .get("regs")
                .unwrap()
                .get("cleanup")
                .unwrap()
                .as_str(),
            Some("caller")
        );
        r.regs.as_mut().unwrap().callee_popped = 4;
        assert_eq!(
            Value::parse(&r.to_json())
                .unwrap()
                .get("regs")
                .unwrap()
                .get("cleanup")
                .unwrap()
                .as_str(),
            Some("other")
        );
        r.stack_bytes = 0;
        assert_eq!(
            Value::parse(&r.to_json())
                .unwrap()
                .get("regs")
                .unwrap()
                .get("cleanup")
                .unwrap()
                .as_str(),
            Some("none")
        );
    }

    #[test]
    fn file_round_trip_and_error_isolation() {
        let a = Vector::parse(SAMPLE).unwrap();
        let text = format!(
            "# comment\n\n{}\nnot json\n  {}  \n",
            a.to_json(),
            a.to_json()
        );
        let parsed = parse_file(&text);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].0, 3);
        assert_eq!(parsed[0].1.as_ref().unwrap(), &a);
        assert_eq!(parsed[1].0, 4);
        assert!(parsed[1].1.is_err());
        assert_eq!(parsed[2].1.as_ref().unwrap(), &a);
        let written = to_file(&[a.clone(), a.clone()]);
        assert_eq!(written.lines().count(), 2);
        assert!(parse_file(&written)
            .iter()
            .all(|(_, r)| r.as_ref().is_ok_and(|v| v == &a)));
    }

    #[test]
    fn fault_results() {
        let mut r = sample_result();
        r.regs = None;
        r.fault = Some(Fault::Exception {
            code: 0xC000_0005,
            eip: 0x40_1010,
            address: Some(0),
            access: Some("read"),
        });
        let v = Value::parse(&r.to_json()).unwrap();
        assert!(matches!(v.get("regs"), Some(Value::Null)));
        let f = v.get("fault").unwrap();
        assert_eq!(f.get("name").unwrap().as_str(), Some("access_violation"));
        assert_eq!(f.get("address").unwrap().as_str(), Some("0x00000000"));
        r.fault = Some(Fault::ImportTrap {
            index: 2,
            name: "KERNEL32.dll!Sleep".into(),
            caller: 0x40_1234,
        });
        let v = Value::parse(&r.to_json()).unwrap();
        assert_eq!(
            v.get("fault").unwrap().get("kind").unwrap().as_str(),
            Some("import")
        );
        assert_eq!(
            v.get("fault").unwrap().get("name").unwrap().as_str(),
            Some("KERNEL32.dll!Sleep")
        );
        assert_eq!(exception_name(0xC000_001D), "illegal_instruction");
        // What SSE raises when a vector's MXCSR unmasks an exception.
        assert_eq!(exception_name(0xC000_02B5), "float_multiple_traps");
        assert_eq!(exception_name(0xC000_02B4), "float_multiple_faults");
        // Exceptions the called code raises itself and nothing handled.
        assert_eq!(exception_name(0xE06D_7363), "cpp_exception");
        assert_eq!(exception_name(0x8000_0004), "single_step");
        assert_eq!(exception_name(0xC000_0025), "noncontinuable_exception");
        assert_eq!(exception_name(0xC000_0026), "invalid_disposition");
        assert_eq!(exception_name(0xC000_0006), "in_page_error");
        assert_eq!(exception_name(1), "exception");
    }
}
