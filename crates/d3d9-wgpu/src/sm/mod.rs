//! Direct3D 9 shader bytecode (shader models 1 to 3): the game's packages (`package`), the tokens
//! read into instructions (this file), a listing in the style of `fxc /dumpbin` (`disasm`) and the
//! conversion to WGSL (`wgsl`).
//!
//! The token layout is the one the DirectX 9 SDK documents ("Shader Code Format": version token,
//! instruction tokens, destination and source parameter tokens, comment tokens, the end token).
//! The reader came from the removed `crates/shaders` (#282, ADR-0011), rewritten to give
//! structured instructions instead of text.

pub mod disasm;
pub mod package;
pub mod wgsl;

use std::fmt;

/// Why a shader could not be read or converted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Not a shader package, or cut short.
    NotAPackage(String),
    /// The bytecode does not follow the Direct3D 9 format.
    BadBytecode(String),
    /// Read, but uses something the converter does not provide yet.
    Unsupported(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotAPackage(why) => write!(f, "not a shader package: {why}"),
            Error::BadBytecode(why) => write!(f, "unreadable shader: {why}"),
            Error::Unsupported(why) => write!(f, "not converted: {why}"),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) const END: u32 = 0x0000_FFFF;
pub(crate) const COMMENT: u32 = 0xFFFE;
const CTAB: u32 = u32::from_le_bytes(*b"CTAB");

/// Vertex or pixel shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    Vertex,
    Pixel,
}

/// Opcodes (`D3DSHADER_INSTRUCTION_OPCODE_TYPE`).
pub mod op {
    pub const NOP: u16 = 0;
    pub const MOV: u16 = 1;
    pub const ADD: u16 = 2;
    pub const SUB: u16 = 3;
    pub const MAD: u16 = 4;
    pub const MUL: u16 = 5;
    pub const RCP: u16 = 6;
    pub const RSQ: u16 = 7;
    pub const DP3: u16 = 8;
    pub const DP4: u16 = 9;
    pub const MIN: u16 = 10;
    pub const MAX: u16 = 11;
    pub const SLT: u16 = 12;
    pub const SGE: u16 = 13;
    pub const EXP: u16 = 14;
    pub const LOG: u16 = 15;
    pub const LIT: u16 = 16;
    pub const DST: u16 = 17;
    pub const LRP: u16 = 18;
    pub const FRC: u16 = 19;
    pub const M4X4: u16 = 20;
    pub const M4X3: u16 = 21;
    pub const M3X4: u16 = 22;
    pub const M3X3: u16 = 23;
    pub const M3X2: u16 = 24;
    pub const CALL: u16 = 25;
    pub const CALLNZ: u16 = 26;
    pub const LOOP: u16 = 27;
    pub const RET: u16 = 28;
    pub const ENDLOOP: u16 = 29;
    pub const LABEL: u16 = 30;
    pub const DCL: u16 = 31;
    pub const POW: u16 = 32;
    pub const CRS: u16 = 33;
    pub const SGN: u16 = 34;
    pub const ABS: u16 = 35;
    pub const NRM: u16 = 36;
    pub const SINCOS: u16 = 37;
    pub const REP: u16 = 38;
    pub const ENDREP: u16 = 39;
    pub const IF: u16 = 40;
    pub const IFC: u16 = 41;
    pub const ELSE: u16 = 42;
    pub const ENDIF: u16 = 43;
    pub const BREAK: u16 = 44;
    pub const BREAKC: u16 = 45;
    pub const MOVA: u16 = 46;
    pub const DEFB: u16 = 47;
    pub const DEFI: u16 = 48;
    pub const TEXCOORD: u16 = 64;
    pub const TEXKILL: u16 = 65;
    pub const TEX: u16 = 66;
    pub const EXPP: u16 = 78;
    pub const LOGP: u16 = 79;
    pub const CND: u16 = 80;
    pub const DEF: u16 = 81;
    pub const CMP: u16 = 88;
    pub const BEM: u16 = 89;
    pub const DP2ADD: u16 = 90;
    pub const DSX: u16 = 91;
    pub const DSY: u16 = 92;
    pub const TEXLDD: u16 = 93;
    pub const SETP: u16 = 94;
    pub const TEXLDL: u16 = 95;
    pub const BREAKP: u16 = 96;
    pub const PHASE: u16 = 0xFFFD;
}

/// Register types (`D3DSHADER_PARAM_REGISTER_TYPE`).
pub mod reg {
    pub const TEMP: u8 = 0;
    pub const INPUT: u8 = 1;
    pub const CONST: u8 = 2;
    /// `a0` in a vertex shader, `t#` in a pixel shader.
    pub const ADDR_TEXTURE: u8 = 3;
    /// `oPos` (0), `oFog` (1), `oPts` (2).
    pub const RASTOUT: u8 = 4;
    pub const ATTROUT: u8 = 5;
    /// `oT#` before vs_3_0, `o#` in vs_3_0.
    pub const OUTPUT: u8 = 6;
    pub const CONSTINT: u8 = 7;
    pub const COLOROUT: u8 = 8;
    pub const DEPTHOUT: u8 = 9;
    pub const SAMPLER: u8 = 10;
    pub const CONST2: u8 = 11;
    pub const CONST3: u8 = 12;
    pub const CONST4: u8 = 13;
    pub const CONSTBOOL: u8 = 14;
    pub const LOOP: u8 = 15;
    pub const TEMPFLOAT16: u8 = 16;
    /// `vPos` (0), `vFace` (1).
    pub const MISC: u8 = 17;
    pub const LABEL: u8 = 18;
    pub const PREDICATE: u8 = 19;
}

/// Declaration usages (`D3DDECLUSAGE`), shared by shaders' `dcl` and vertex declarations.
pub mod usage {
    pub const POSITION: u8 = 0;
    pub const BLENDWEIGHT: u8 = 1;
    pub const BLENDINDICES: u8 = 2;
    pub const NORMAL: u8 = 3;
    pub const PSIZE: u8 = 4;
    pub const TEXCOORD: u8 = 5;
    pub const TANGENT: u8 = 6;
    pub const BINORMAL: u8 = 7;
    pub const TESSFACTOR: u8 = 8;
    pub const POSITIONT: u8 = 9;
    pub const COLOR: u8 = 10;
    pub const FOG: u8 = 11;
    pub const DEPTH: u8 = 12;
    pub const SAMPLE: u8 = 13;
}

/// A register: type and number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reg {
    pub ty: u8,
    pub n: u32,
}

/// The register a relative address reads (`a0` or `aL`), with the component it uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rel {
    pub reg: Reg,
    pub comp: u8,
}

/// A destination parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dst {
    pub reg: Reg,
    /// Write mask, bit 0 = x.
    pub mask: u8,
    pub sat: bool,
    pub pp: bool,
    pub centroid: bool,
    /// Shader model 1's result shift (4 bits, two's complement).
    pub shift: u8,
    pub rel: Option<Rel>,
}

/// A source parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Src {
    pub reg: Reg,
    /// Two bits a component, x first (0xE4 = .xyzw).
    pub swizzle: u8,
    /// `D3DSHADER_PARAM_SRCMOD_TYPE`: 0 none, 1 neg, 2 bias, 3 bias+neg, 4 bx2, 5 bx2+neg, 6
    /// comp (1 - x), 7 x2, 8 x2+neg, 9 dz, 10 dw, 11 abs, 12 abs+neg, 13 not.
    pub modifier: u8,
    pub rel: Option<Rel>,
}

impl Src {
    /// The component the swizzle puts in position `k`.
    pub fn component(&self, k: usize) -> u8 {
        (self.swizzle >> (2 * k)) & 3
    }
}

/// A `dcl`'s usage token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decl {
    pub usage: u8,
    pub index: u8,
    /// `D3DSAMPLER_TEXTURE_TYPE` >> 27: 2 = 2D, 3 = cube, 4 = volume (samplers only).
    pub texture: u8,
}

/// One instruction.
#[derive(Debug, Clone, PartialEq)]
pub struct Ins {
    pub op: u16,
    /// The instruction's specific controls: the comparison of `ifc`, `breakc` and `setp`
    /// (1 gt, 2 eq, 3 ge, 4 lt, 5 ne, 6 le), `texld`'s variant (1 project, 2 bias).
    pub controls: u8,
    pub dst: Option<Dst>,
    pub predicate: Option<Src>,
    pub src: Vec<Src>,
    pub decl: Option<Decl>,
    /// `def`'s four values (float bits), `defi`'s four integers, `defb`'s one.
    pub values: [u32; 4],
}

/// A named constant of the shader's constant table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    pub name: String,
    /// 0 bool, 1 int4, 2 float4, 3 sampler (`D3DXREGISTER_SET`).
    pub set: u16,
    pub index: u16,
    pub count: u16,
}

/// A whole shader.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stage: Stage,
    pub major: u8,
    pub minor: u8,
    pub ins: Vec<Ins>,
    pub constants: Vec<Constant>,
    pub creator: Option<String>,
}

impl Program {
    /// `vs_3_0`, `ps_2_x`, ...
    pub fn profile(&self) -> String {
        let kind = match self.stage {
            Stage::Vertex => "vs",
            Stage::Pixel => "ps",
        };
        if self.major == 2 && self.minor == 1 {
            format!("{kind}_2_x")
        } else {
            format!("{kind}_{}_{}", self.major, self.minor)
        }
    }
}

/// A vertex (`0xFFFE`) or pixel (`0xFFFF`) shader version token, model 1-3.
pub(crate) fn is_version(token: u32) -> bool {
    let kind = token >> 16;
    let major = (token >> 8) & 0xFF;
    (kind == 0xFFFF || kind == 0xFFFE) && (1..=3).contains(&major)
}

fn reg_type(t: u32) -> u8 {
    (((t >> 28) & 0x7) | ((t >> 8) & 0x18)) as u8
}

fn reg_of(t: u32) -> Reg {
    Reg {
        ty: reg_type(t),
        n: t & 0x7FF,
    }
}

/// Parameter tokens of shader model 1 instructions, which do not store their length.
fn sm1_length(op: u16, stage: Stage, minor: u8) -> Option<usize> {
    Some(match op {
        0 | op::PHASE => 0,
        1 | 6 | 7 | 14 | 15 | 16 | 19 | 78 | 79 => 2,
        2 | 3 | 5 | 8 | 9 | 10 | 11 | 12 | 13 | 17 | 20..=24 | 89 => 3,
        4 | 18 | 80 | 88 => 4,
        op::DCL => 2,
        op::DEF => 5,
        64 | 65 | 87 => 1,
        op::TEX if stage == Stage::Pixel && minor >= 4 => 2,
        op::TEX => 1,
        67..=77 | 82..=86 => 2,
        _ => return None,
    })
}

/// Whether an instruction's first parameter is a destination.
pub(crate) fn has_destination(op: u16) -> bool {
    !matches!(
        op,
        op::NOP
            | op::CALL
            | op::CALLNZ
            | op::LOOP
            | op::RET
            | op::ENDLOOP
            | op::LABEL
            | op::REP
            | op::ENDREP
            | op::IF
            | op::IFC
            | op::ELSE
            | op::ENDIF
            | op::BREAK
            | op::BREAKC
            | op::BREAKP
            | op::PHASE
    )
}

/// The bytecode's 32-bit tokens.
pub fn tokens(bytecode: &[u8]) -> Result<Vec<u32>, Error> {
    if bytecode.len() % 4 != 0 {
        return Err(Error::BadBytecode(format!(
            "{} bytes is not a whole number of 4-byte tokens",
            bytecode.len()
        )));
    }
    Ok(bytecode
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect())
}

/// Reads a shader's tokens into instructions.
pub fn parse(tokens: &[u32]) -> Result<Program, Error> {
    let first = *tokens
        .first()
        .ok_or_else(|| Error::BadBytecode("empty".into()))?;
    if !is_version(first) {
        return Err(Error::BadBytecode(format!(
            "starts with {first:#010x}, not a shader version"
        )));
    }
    let stage = if first >> 16 == 0xFFFE {
        Stage::Vertex
    } else {
        Stage::Pixel
    };
    let major = ((first >> 8) & 0xFF) as u8;
    let minor = (first & 0xFF) as u8;
    let mut program = Program {
        stage,
        major,
        minor,
        ins: Vec::new(),
        constants: Vec::new(),
        creator: None,
    };
    let mut i = 1;
    loop {
        let Some(&token) = tokens.get(i) else {
            return Err(Error::BadBytecode("no end token".into()));
        };
        if token == END {
            break;
        }
        let opcode = (token & 0xFFFF) as u16;
        if u32::from(opcode) == COMMENT {
            let len = ((token >> 16) & 0x7FFF) as usize;
            let data = tokens.get(i + 1..i + 1 + len).unwrap_or(&[]);
            if data.first() == Some(&CTAB) {
                let bytes: Vec<u8> = data[1..].iter().flat_map(|t| t.to_le_bytes()).collect();
                (program.constants, program.creator) = constant_table(&bytes);
            }
            i += 1 + len;
            continue;
        }
        let len = if major >= 2 {
            ((token >> 24) & 0xF) as usize
        } else {
            sm1_length(opcode, stage, minor).ok_or_else(|| {
                Error::BadBytecode(format!("unknown instruction {opcode} at token {i}"))
            })?
        };
        let params = tokens.get(i + 1..i + 1 + len).ok_or_else(|| {
            Error::BadBytecode(format!("instruction at token {i} runs past the end"))
        })?;
        program.ins.push(instruction(token, params, major)?);
        i += 1 + len;
    }
    Ok(program)
}

fn destination(t: u32) -> Dst {
    let m = (t >> 20) & 0xF;
    Dst {
        reg: reg_of(t),
        mask: ((t >> 16) & 0xF) as u8,
        sat: m & 1 != 0,
        pp: m & 2 != 0,
        centroid: m & 4 != 0,
        shift: ((t >> 24) & 0xF) as u8,
        rel: None,
    }
}

fn relative(t: u32) -> Rel {
    Rel {
        reg: reg_of(t),
        comp: ((t >> 16) & 3) as u8,
    }
}

fn source(t: u32) -> Src {
    Src {
        reg: reg_of(t),
        swizzle: ((t >> 16) & 0xFF) as u8,
        modifier: ((t >> 24) & 0xF) as u8,
        rel: None,
    }
}

fn instruction(token: u32, params: &[u32], major: u8) -> Result<Ins, Error> {
    let op = (token & 0xFFFF) as u16;
    let mut ins = Ins {
        op,
        controls: ((token >> 16) & 0xFF) as u8,
        dst: None,
        predicate: None,
        src: Vec::new(),
        decl: None,
        values: [0; 4],
    };
    let bad = || Error::BadBytecode(format!("instruction {op} has too few parameters"));
    match op {
        op::DCL => {
            let (&u, &d) = (
                params.first().ok_or_else(bad)?,
                params.get(1).ok_or_else(bad)?,
            );
            ins.decl = Some(Decl {
                usage: (u & 0x1F) as u8,
                index: ((u >> 16) & 0xF) as u8,
                texture: ((u >> 27) & 0xF) as u8,
            });
            ins.dst = Some(destination(d));
            return Ok(ins);
        }
        op::DEF | op::DEFI => {
            if params.len() < 5 {
                return Err(bad());
            }
            ins.dst = Some(destination(params[0]));
            ins.values.copy_from_slice(&params[1..5]);
            return Ok(ins);
        }
        op::DEFB => {
            if params.len() < 2 {
                return Err(bad());
            }
            ins.dst = Some(destination(params[0]));
            ins.values[0] = params[1];
            return Ok(ins);
        }
        _ => {}
    }
    let mut rest = params;
    if has_destination(op) {
        let (&d, tail) = rest.split_first().ok_or_else(bad)?;
        let mut dst = destination(d);
        rest = tail;
        if d & (1 << 13) != 0 && major >= 3 {
            let (&r, tail) = rest.split_first().ok_or_else(bad)?;
            dst.rel = Some(relative(r));
            rest = tail;
        }
        ins.dst = Some(dst);
        if token & (1 << 28) != 0 {
            let (&p, tail) = rest.split_first().ok_or_else(bad)?;
            ins.predicate = Some(source(p));
            rest = tail;
        }
    } else if token & (1 << 28) != 0 {
        // A predicated flow instruction (`callnz`, `if`, `breakp` take the predicate as a source).
    }
    let mut k = 0;
    while k < rest.len() {
        let t = rest[k];
        let mut s = source(t);
        if t & (1 << 13) != 0 {
            if major >= 2 {
                let r = *rest.get(k + 1).ok_or_else(bad)?;
                s.rel = Some(relative(r));
                k += 1;
            } else {
                // Shader model 1: always a0.x.
                s.rel = Some(Rel {
                    reg: Reg {
                        ty: reg::ADDR_TEXTURE,
                        n: 0,
                    },
                    comp: 0,
                });
            }
        }
        ins.src.push(s);
        k += 1;
    }
    Ok(ins)
}

/// Reads the constant table: each constant's name, register set, first register and register
/// count, and the compiler's name.
fn constant_table(b: &[u8]) -> (Vec<Constant>, Option<String>) {
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
        constants.push(Constant {
            name,
            set,
            index,
            count: n,
        });
    }
    (constants, creator)
}

/// The name of an opcode as the SDK's assembler writes it.
pub fn opcode_name(op: u16) -> Option<&'static str> {
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
        96 => "break",
        0xFFFD => "phase",
        _ => return None,
    })
}

/// Converts every shader of a package with a default link (each vertex input four floats, the
/// varyings the shader itself declares) and checks the WGSL with naga: the failures, by name.
pub fn check_package(package: &package::Package) -> Vec<(String, String)> {
    let mut failures = Vec::new();
    for s in &package.shaders {
        let result = tokens(&s.bytecode).and_then(|t| parse(&t)).and_then(|p| {
            let sig = wgsl::signature(&p);
            let link = wgsl::Link {
                inputs: vec![wgsl::Input::Float(4); 16],
                centroid: if p.stage == Stage::Pixel {
                    sig.centroid.clone()
                } else {
                    Vec::new()
                },
                varyings: match p.stage {
                    Stage::Vertex => sig
                        .outputs
                        .iter()
                        .map(|(_, s)| *s)
                        .filter(|s| !(s.usage == usage::POSITION && s.index == 0))
                        .collect(),
                    Stage::Pixel => sig.inputs.iter().map(|(_, s)| *s).collect(),
                },
            };
            let source = wgsl::convert(&p, &link)?;
            validate(&source).map_err(Error::Unsupported)
        });
        if let Err(e) = result {
            failures.push((s.name.clone(), e.to_string()));
        }
    }
    failures
}

/// Parses and validates WGSL with naga, as wgpu will.
pub fn validate(source: &str) -> Result<naga::Module, String> {
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .map_err(|e| format!("{e:?}"))?;
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `vs_2_0`: dcl_position v0; m4x4 oPos, v0, c0; mov oT0.xy, v0.yx; end.
    pub(crate) fn small_vertex_shader() -> Vec<u32> {
        vec![
            0xFFFE_0200,
            0x0200_001F,
            0x8000_0000,
            0x900F_0000,
            0x0300_0014,
            0xC00F_0000,
            0x90E4_0000,
            0xA0E4_0000,
            0x0200_0001,
            0xE003_0000,
            0x90E1_0000,
            END,
        ]
    }

    #[test]
    fn instructions_are_read_with_their_parameters() {
        let p = parse(&small_vertex_shader()).unwrap();
        assert_eq!(p.profile(), "vs_2_0");
        assert_eq!(p.ins.len(), 3);
        assert_eq!(p.ins[0].op, op::DCL);
        assert_eq!(p.ins[0].decl.unwrap().usage, usage::POSITION);
        assert_eq!(p.ins[1].op, op::M4X4);
        let d = p.ins[1].dst.unwrap();
        assert_eq!((d.reg.ty, d.reg.n, d.mask), (reg::RASTOUT, 0, 0xF));
        assert_eq!(
            p.ins[1].src[1].reg,
            Reg {
                ty: reg::CONST,
                n: 0
            }
        );
        let mov = &p.ins[2];
        assert_eq!(mov.dst.unwrap().reg.ty, reg::OUTPUT);
        assert_eq!(mov.dst.unwrap().mask, 0x3);
        assert_eq!(mov.src[0].swizzle, 0xE1);
        assert_eq!((mov.src[0].component(0), mov.src[0].component(1)), (1, 0));
    }

    #[test]
    fn a_relative_source_takes_its_address_token() {
        // vs_3_0: mov r0, c4[a0.y]; end.
        let t = [
            0xFFFE_0300,
            0x0300_0001,
            0x800F_0000,
            0xA0E4_2004,
            0xB055_0000,
            END,
        ];
        let p = parse(&t).unwrap();
        let s = p.ins[0].src[0];
        assert_eq!(
            s.reg,
            Reg {
                ty: reg::CONST,
                n: 4
            }
        );
        let rel = s.rel.unwrap();
        assert_eq!((rel.reg.ty, rel.comp), (reg::ADDR_TEXTURE, 1));
    }

    #[test]
    fn a_predicated_instruction_reads_the_predicate_after_its_destination() {
        // ps_3_0: (p0.x) mov r0, r1; end.
        let t = [
            0xFFFF_0300,
            0x1300_0001,
            0x800F_0000,
            0xB000_1000,
            0x80E4_0001,
            END,
        ];
        let p = parse(&t).unwrap();
        let ins = &p.ins[0];
        assert_eq!(ins.predicate.unwrap().reg.ty, reg::PREDICATE);
        assert_eq!(ins.src.len(), 1);
        assert_eq!(
            ins.src[0].reg,
            Reg {
                ty: reg::TEMP,
                n: 1
            }
        );
    }

    #[test]
    fn bytecode_without_an_end_is_refused() {
        assert!(parse(&[0xFFFE_0200, 0x0200_0001]).is_err());
        assert!(parse(&[0x1234_5678, END]).is_err());
        assert!(tokens(&[1, 2, 3]).is_err());
    }
}
