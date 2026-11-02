//! Direct3D 9 shader bytecode (shader models 1 to 3) as readable
//! instructions, in the style of Microsoft's `fxc /dumpbin`, with each
//! constant register's name from the shader's constant table.

use std::fmt::{self, Write};

use crate::package::is_version;
use crate::Error;

const END: u32 = 0x0000_FFFF;
const COMMENT: u32 = 0xFFFE;
const CTAB: u32 = u32::from_le_bytes(*b"CTAB");

/// Vertex or pixel shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderKind {
    Vertex,
    Pixel,
}

/// A shader's model, e.g. `ps_3_0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub kind: ShaderKind,
    pub major: u8,
    pub minor: u8,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.kind {
            ShaderKind::Vertex => "vs",
            ShaderKind::Pixel => "ps",
        };
        // Models 2.a and 2.b are both stored as 2.1, and written 2_x.
        if self.major == 2 && self.minor == 1 {
            write!(f, "{kind}_2_x")
        } else {
            write!(f, "{kind}_{}_{}", self.major, self.minor)
        }
    }
}

/// Which kind of register a constant lives in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterSet {
    Bool,
    Int4,
    Float4,
    Sampler,
}

impl RegisterSet {
    fn prefix(self) -> &'static str {
        match self {
            RegisterSet::Bool => "b",
            RegisterSet::Int4 => "i",
            RegisterSet::Float4 => "c",
            RegisterSet::Sampler => "s",
        }
    }
}

/// A named constant from the shader's constant table: the game fills these
/// registers in before drawing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    pub name: String,
    pub set: RegisterSet,
    pub index: u16,
    /// Registers it takes (an array or matrix takes several).
    pub count: u16,
}

impl Constant {
    /// `c4`, or `c4-c7` for several registers.
    pub fn registers(&self) -> String {
        let p = self.set.prefix();
        if self.count > 1 {
            format!("{p}{}-{p}{}", self.index, self.index + self.count - 1)
        } else {
            format!("{p}{}", self.index)
        }
    }

    fn covers(&self, set: RegisterSet, index: u32) -> Option<String> {
        let start = u32::from(self.index);
        let end = start + u32::from(self.count.max(1));
        if self.set != set || !(start..end).contains(&index) {
            return None;
        }
        Some(if self.count > 1 {
            format!("{}[{}]", self.name, index - start)
        } else {
            self.name.clone()
        })
    }
}

/// A disassembled shader.
#[derive(Debug, Clone, PartialEq)]
pub struct Disassembly {
    pub version: Version,
    pub constants: Vec<Constant>,
    /// The compiler that made it, from the constant table.
    pub creator: Option<String>,
    /// Instructions, declarations included.
    pub instructions: usize,
    /// The full listing.
    pub text: String,
}

impl Disassembly {
    /// The constant using a register, e.g. `LightData[1]` for `c5`.
    pub fn constant_at(&self, set: RegisterSet, index: u32) -> Option<String> {
        self.constants.iter().find_map(|c| c.covers(set, index))
    }
}

/// Disassembles Direct3D 9 shader bytecode.
pub fn disassemble(bytecode: &[u8]) -> Result<Disassembly, Error> {
    if bytecode.len() % 4 != 0 {
        return Err(Error::BadBytecode(format!(
            "{} bytes isn't a whole number of 4-byte tokens",
            bytecode.len()
        )));
    }
    let tokens: Vec<u32> = bytecode
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    let first = *tokens
        .first()
        .ok_or_else(|| Error::BadBytecode("empty".into()))?;
    if !is_version(first) {
        return Err(Error::BadBytecode(format!(
            "starts with {first:#010x}, not a shader version"
        )));
    }
    let version = Version {
        kind: if first >> 16 == 0xFFFE {
            ShaderKind::Vertex
        } else {
            ShaderKind::Pixel
        },
        major: ((first >> 8) & 0xFF) as u8,
        minor: (first & 0xFF) as u8,
    };

    // The constant table comes first, in a comment; read it before the
    // instructions so they can be annotated with its names.
    let mut constants = Vec::new();
    let mut creator = None;
    let mut i = 1;
    while i < tokens.len() && tokens[i] & 0xFFFF == COMMENT {
        let len = ((tokens[i] >> 16) & 0x7FFF) as usize;
        let data = tokens.get(i + 1..i + 1 + len).unwrap_or(&[]);
        if data.first() == Some(&CTAB) {
            let bytes: Vec<u8> = data[1..].iter().flat_map(|t| t.to_le_bytes()).collect();
            (constants, creator) = parse_constant_table(&bytes);
        }
        i += 1 + len;
    }
    let mut listing = Disassembly {
        version,
        constants,
        creator,
        instructions: 0,
        text: String::new(),
    };

    let mut body = String::new();
    let mut depth = 0usize;
    let mut i = 1;
    loop {
        let Some(&token) = tokens.get(i) else {
            return Err(Error::BadBytecode("no end token".into()));
        };
        if token == END {
            break;
        }
        let opcode = token & 0xFFFF;
        if opcode == COMMENT {
            i += 1 + ((token >> 16) & 0x7FFF) as usize;
            continue;
        }
        let len = if version.major >= 2 {
            ((token >> 24) & 0xF) as usize
        } else {
            sm1_length(opcode, version).ok_or_else(|| {
                Error::BadBytecode(format!("unknown instruction {opcode} at token {i}"))
            })?
        };
        let params = tokens.get(i + 1..i + 1 + len).ok_or_else(|| {
            Error::BadBytecode(format!("instruction at token {i} runs past the end"))
        })?;
        if matches!(opcode, ops::ELSE | ops::ENDIF | ops::ENDLOOP | ops::ENDREP) {
            depth = depth.saturating_sub(1);
        }
        let line = listing.instruction(token, params);
        let _ = writeln!(body, "{}{line}", "    ".repeat(depth));
        if matches!(
            opcode,
            ops::IF | ops::IFC | ops::ELSE | ops::LOOP | ops::REP
        ) {
            depth += 1;
        }
        listing.instructions += 1;
        i += 1 + len;
    }

    let mut text = String::new();
    let _ = write!(
        text,
        "// {}, {} instructions",
        listing.version, listing.instructions
    );
    if let Some(c) = &listing.creator {
        let _ = write!(text, ", compiled by {c}");
    }
    text.push('\n');
    if !listing.constants.is_empty() {
        text.push_str("//\n// Constants:\n");
        let width = listing
            .constants
            .iter()
            .map(|c| c.name.len())
            .max()
            .unwrap_or(0);
        for c in &listing.constants {
            let _ = writeln!(text, "//   {:<width$}  {}", c.name, c.registers());
        }
    }
    let _ = writeln!(text, "\n{}", listing.version);
    text.push_str(&body);
    listing.text = text;
    Ok(listing)
}

/// Reads the constant table: each constant's name, register set, first
/// register and register count, and the compiler's name.
fn parse_constant_table(b: &[u8]) -> (Vec<Constant>, Option<String>) {
    let u32_at = |at: usize| {
        b.get(at..at + 4)
            .map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as usize)
    };
    let u16_at = |at: usize| b.get(at..at + 2).map(|s| u16::from_le_bytes([s[0], s[1]]));
    let string_at = |at: usize| {
        let s = b.get(at..)?;
        let end = s.iter().position(|&c| c == 0)?;
        Some(String::from_utf8_lossy(&s[..end]).into_owned())
    };
    let creator = u32_at(4).and_then(string_at);
    let (Some(count), Some(info)) = (u32_at(12), u32_at(16)) else {
        return (Vec::new(), creator);
    };
    let mut constants = Vec::new();
    for k in 0..count.min(4096) {
        let at = info + k * 20;
        let (Some(name), Some(set), Some(index), Some(n)) = (
            u32_at(at).and_then(string_at),
            u16_at(at + 4),
            u16_at(at + 6),
            u16_at(at + 8),
        ) else {
            break;
        };
        let set = match set {
            0 => RegisterSet::Bool,
            1 => RegisterSet::Int4,
            2 => RegisterSet::Float4,
            3 => RegisterSet::Sampler,
            _ => continue,
        };
        constants.push(Constant {
            name,
            set,
            index,
            count: n,
        });
    }
    (constants, creator)
}

mod ops {
    pub const NOP: u32 = 0;
    pub const CALL: u32 = 25;
    pub const CALLNZ: u32 = 26;
    pub const LOOP: u32 = 27;
    pub const RET: u32 = 28;
    pub const ENDLOOP: u32 = 29;
    pub const LABEL: u32 = 30;
    pub const DCL: u32 = 31;
    pub const REP: u32 = 38;
    pub const ENDREP: u32 = 39;
    pub const IF: u32 = 40;
    pub const IFC: u32 = 41;
    pub const ELSE: u32 = 42;
    pub const ENDIF: u32 = 43;
    pub const BREAK: u32 = 44;
    pub const BREAKC: u32 = 45;
    pub const DEFB: u32 = 47;
    pub const DEFI: u32 = 48;
    pub const TEXKILL: u32 = 65;
    pub const TEX: u32 = 66;
    pub const DEF: u32 = 81;
    pub const SETP: u32 = 94;
    pub const BREAKP: u32 = 96;
    pub const PHASE: u32 = 0xFFFD;
}

fn opcode_name(op: u32) -> Option<&'static str> {
    Some(match op {
        0 => "nop",
        1 => "mov",
        2 => "add",
        3 => "sub",
        4 => "mad",
        5 => "mul",
        6 => "rcp",
        7 => "rsq",
        8 => "dp3",
        9 => "dp4",
        10 => "min",
        11 => "max",
        12 => "slt",
        13 => "sge",
        14 => "exp",
        15 => "log",
        16 => "lit",
        17 => "dst",
        18 => "lrp",
        19 => "frc",
        20 => "m4x4",
        21 => "m4x3",
        22 => "m3x4",
        23 => "m3x3",
        24 => "m3x2",
        25 => "call",
        26 => "callnz",
        27 => "loop",
        28 => "ret",
        29 => "endloop",
        30 => "label",
        31 => "dcl",
        32 => "pow",
        33 => "crs",
        34 => "sgn",
        35 => "abs",
        36 => "nrm",
        37 => "sincos",
        38 => "rep",
        39 => "endrep",
        40 => "if",
        41 => "if",
        42 => "else",
        43 => "endif",
        44 => "break",
        45 => "break",
        46 => "mova",
        47 => "defb",
        48 => "defi",
        64 => "texcoord",
        65 => "texkill",
        66 => "texld",
        67 => "texbem",
        68 => "texbeml",
        69 => "texreg2ar",
        70 => "texreg2gb",
        71 => "texm3x2pad",
        72 => "texm3x2tex",
        73 => "texm3x3pad",
        74 => "texm3x3tex",
        76 => "texm3x3spec",
        77 => "texm3x3vspec",
        78 => "expp",
        79 => "logp",
        80 => "cnd",
        81 => "def",
        82 => "texreg2rgb",
        83 => "texdp3tex",
        84 => "texm3x2depth",
        85 => "texdp3",
        86 => "texm3x3",
        87 => "texdepth",
        88 => "cmp",
        89 => "bem",
        90 => "dp2add",
        91 => "dsx",
        92 => "dsy",
        93 => "texldd",
        94 => "setp",
        95 => "texldl",
        96 => "break_pred",
        0xFFFD => "phase",
        _ => return None,
    })
}

/// Parameter tokens of shader model 1 instructions, which don't store their
/// length.
fn sm1_length(op: u32, version: Version) -> Option<usize> {
    Some(match op {
        0 | ops::PHASE => 0,
        1 | 6 | 7 | 14 | 15 | 16 | 19 | 78 | 79 => 2,
        2 | 3 | 5 | 8 | 9 | 10 | 11 | 12 | 13 | 17 | 20..=24 | 89 => 3,
        4 | 18 | 80 | 88 => 4,
        ops::DCL => 2,
        ops::DEF => 5,
        64 | 65 | 87 => 1,
        ops::TEX if version.kind == ShaderKind::Pixel && version.minor >= 4 => 2,
        ops::TEX => 1,
        67..=77 | 82..=86 => 2,
        _ => return None,
    })
}

fn has_destination(op: u32) -> bool {
    !matches!(
        op,
        ops::NOP
            | ops::CALL
            | ops::CALLNZ
            | ops::LOOP
            | ops::RET
            | ops::ENDLOOP
            | ops::LABEL
            | ops::REP
            | ops::ENDREP
            | ops::IF
            | ops::IFC
            | ops::ELSE
            | ops::ENDIF
            | ops::BREAK
            | ops::BREAKC
            | ops::BREAKP
            | ops::TEXKILL
            | ops::PHASE
    )
}

fn register_type(t: u32) -> u32 {
    ((t >> 28) & 0x7) | ((t >> 8) & 0x18)
}

fn register_number(t: u32) -> u32 {
    t & 0x7FF
}

mod reg {
    pub const CONST: u32 = 2;
    pub const CONSTINT: u32 = 7;
    pub const SAMPLER: u32 = 10;
    pub const CONST2: u32 = 11;
    pub const CONST3: u32 = 12;
    pub const CONST4: u32 = 13;
    pub const CONSTBOOL: u32 = 14;
    pub const MISC: u32 = 17;
}

const USAGES: [&str; 14] = [
    "position",
    "blendweight",
    "blendindices",
    "normal",
    "psize",
    "texcoord",
    "tangent",
    "binormal",
    "tessfactor",
    "positiont",
    "color",
    "fog",
    "depth",
    "sample",
];

impl Disassembly {
    fn register_name(&self, t: u32) -> String {
        let n = register_number(t);
        let vertex = self.version.kind == ShaderKind::Vertex;
        match register_type(t) {
            0 => format!("r{n}"),
            1 => format!("v{n}"),
            2 => format!("c{n}"),
            3 if vertex => format!("a{n}"),
            3 => format!("t{n}"),
            4 => match n {
                0 => "oPos".into(),
                1 => "oFog".into(),
                _ => "oPts".into(),
            },
            5 => format!("oD{n}"),
            6 if vertex && self.version.major >= 3 => format!("o{n}"),
            6 => format!("oT{n}"),
            7 => format!("i{n}"),
            8 => format!("oC{n}"),
            9 => "oDepth".into(),
            10 => format!("s{n}"),
            11 => format!("c{}", n + 2048),
            12 => format!("c{}", n + 4096),
            13 => format!("c{}", n + 6144),
            14 => format!("b{n}"),
            15 => "aL".into(),
            16 => format!("half{n}"),
            17 if n == 0 => "vPos".into(),
            17 => "vFace".into(),
            18 => format!("l{n}"),
            19 => format!("p{n}"),
            other => format!("?{other}_{n}"),
        }
    }

    /// The constant a register token refers to, for the line's comment.
    fn constant_note(&self, t: u32) -> Option<String> {
        let n = register_number(t);
        let (set, index) = match register_type(t) {
            reg::CONST => (RegisterSet::Float4, n),
            reg::CONST2 => (RegisterSet::Float4, n + 2048),
            reg::CONST3 => (RegisterSet::Float4, n + 4096),
            reg::CONST4 => (RegisterSet::Float4, n + 6144),
            reg::CONSTINT => (RegisterSet::Int4, n),
            reg::CONSTBOOL => (RegisterSet::Bool, n),
            reg::SAMPLER => (RegisterSet::Sampler, n),
            _ => return None,
        };
        let name = self.constant_at(set, index)?;
        Some(format!("{}{index} = {name}", set.prefix()))
    }

    fn destination(&self, t: u32) -> String {
        let mut s = self.register_name(t);
        let mask = (t >> 16) & 0xF;
        if mask != 0xF && mask != 0 {
            s.push('.');
            for (bit, c) in ['x', 'y', 'z', 'w'].into_iter().enumerate() {
                if mask & (1 << bit) != 0 {
                    s.push(c);
                }
            }
        }
        s
    }

    /// A source operand; `relative` is the address register token that
    /// follows it when it's indexed.
    fn source(&self, t: u32, relative: Option<u32>) -> String {
        let mut name = self.register_name(t);
        if t & (1 << 13) != 0 {
            let address = match relative {
                Some(r) => {
                    let component = ['x', 'y', 'z', 'w'][((r >> 16) & 3) as usize];
                    format!("{}.{component}", self.register_name(r))
                }
                None => "a0.x".into(),
            };
            name = format!("{name}[{address}]");
        }
        let swizzle = (t >> 16) & 0xFF;
        let mut comps: Vec<char> = (0..4)
            .map(|k| ['x', 'y', 'z', 'w'][((swizzle >> (2 * k)) & 3) as usize])
            .collect();
        let swizzle = if swizzle == 0xE4 {
            String::new()
        } else {
            while comps.len() > 1 && comps[comps.len() - 1] == comps[comps.len() - 2] {
                comps.pop();
            }
            format!(".{}", comps.into_iter().collect::<String>())
        };
        match (t >> 24) & 0xF {
            1 => format!("-{name}{swizzle}"),
            2 => format!("{name}_bias{swizzle}"),
            3 => format!("-{name}_bias{swizzle}"),
            4 => format!("{name}_bx2{swizzle}"),
            5 => format!("-{name}_bx2{swizzle}"),
            6 => format!("1 - {name}{swizzle}"),
            7 => format!("{name}_x2{swizzle}"),
            8 => format!("-{name}_x2{swizzle}"),
            9 => format!("{name}_dz{swizzle}"),
            10 => format!("{name}_dw{swizzle}"),
            11 => format!("{name}_abs{swizzle}"),
            12 => format!("-{name}_abs{swizzle}"),
            13 => format!("!{name}{swizzle}"),
            _ => format!("{name}{swizzle}"),
        }
    }

    /// One instruction as text, with a comment naming the constants it
    /// reads.
    fn instruction(&self, token: u32, params: &[u32]) -> String {
        let op = token & 0xFFFF;
        let controls = (token >> 16) & 0xFF;
        let predicated = token & (1 << 28) != 0;
        let Some(base) = opcode_name(op) else {
            let raw: Vec<String> = params.iter().map(|p| format!("{p:#010x}")).collect();
            return format!("unknown_{op} {}", raw.join(", "));
        };
        let mut name = base.to_string();
        let mut operands: Vec<String> = Vec::new();
        let mut notes: Vec<String> = Vec::new();

        match op {
            ops::DCL if params.len() >= 2 => {
                let (usage, dest) = (params[0], params[1]);
                match register_type(dest) {
                    reg::SAMPLER => match (usage >> 27) & 0xF {
                        2 => name.push_str("_2d"),
                        3 => name.push_str("_cube"),
                        4 => name.push_str("_volume"),
                        _ => {}
                    },
                    reg::MISC => {}
                    _ if self.version.kind == ShaderKind::Pixel && self.version.major < 3 => {}
                    _ => {
                        let index = (usage >> 16) & 0xF;
                        let label = USAGES.get((usage & 0x1F) as usize).unwrap_or(&"usage");
                        name.push('_');
                        name.push_str(label);
                        if index > 0 {
                            name.push_str(&index.to_string());
                        }
                    }
                }
                name.push_str(&modifiers(dest));
                operands.push(self.destination(dest));
                notes.extend(self.constant_note(dest));
            }
            ops::DEF if params.len() >= 5 => {
                operands.push(self.destination(params[0]));
                operands.extend(
                    params[1..5]
                        .iter()
                        .map(|&v| format!("{}", f32::from_bits(v))),
                );
            }
            ops::DEFI if params.len() >= 5 => {
                operands.push(self.destination(params[0]));
                operands.extend(params[1..5].iter().map(|&v| format!("{}", v as i32)));
            }
            ops::DEFB if params.len() >= 2 => {
                operands.push(self.destination(params[0]));
                operands.push(if params[1] != 0 { "true" } else { "false" }.into());
            }
            _ => {
                if op == ops::TEX {
                    name = match (self.version.major, controls) {
                        (0 | 1, _) if self.version.minor < 4 => "tex".into(),
                        (_, 1) => "texldp".into(),
                        (_, 2) => "texldb".into(),
                        _ => "texld".into(),
                    };
                }
                if matches!(op, ops::IFC | ops::BREAKC | ops::SETP) {
                    name.push_str(match controls {
                        1 => "_gt",
                        2 => "_eq",
                        3 => "_ge",
                        4 => "_lt",
                        5 => "_ne",
                        6 => "_le",
                        _ => "",
                    });
                }
                let mut rest = params;
                let mut predicate = None;
                if has_destination(op) {
                    if let Some((&dest, tail)) = rest.split_first() {
                        name.push_str(&modifiers(dest));
                        operands.push(self.destination(dest));
                        rest = tail;
                    }
                    if predicated {
                        if let Some((&p, tail)) = rest.split_first() {
                            predicate = Some(self.source(p, None));
                            rest = tail;
                        }
                    }
                } else if op == ops::TEXKILL {
                    if let Some((&dest, tail)) = rest.split_first() {
                        operands.push(self.destination(dest));
                        rest = tail;
                    }
                }
                // Indexed sources are followed by their address register
                // (shader model 2 vertex shaders and up).
                let mut k = 0;
                while k < rest.len() {
                    let t = rest[k];
                    let indexed = t & (1 << 13) != 0 && self.version.major >= 2;
                    let relative = if indexed {
                        rest.get(k + 1).copied()
                    } else {
                        None
                    };
                    operands.push(self.source(t, relative));
                    notes.extend(self.constant_note(t));
                    k += if indexed { 2 } else { 1 };
                }
                if let Some(p) = predicate {
                    name = format!("({p}) {name}");
                }
            }
        }

        let mut line = name;
        if !operands.is_empty() {
            line.push(' ');
            line.push_str(&operands.join(", "));
        }
        notes.dedup();
        if !notes.is_empty() {
            line = format!("{line:<44} // {}", notes.join(", "));
        }
        line
    }
}

/// `_sat`, `_pp` and `_centroid` from a destination token.
fn modifiers(dest: u32) -> String {
    let m = (dest >> 20) & 0xF;
    let mut s = String::new();
    if m & 1 != 0 {
        s.push_str("_sat");
    }
    if m & 2 != 0 {
        s.push_str("_pp");
    }
    if m & 4 != 0 {
        s.push_str("_centroid");
    }
    s
}
