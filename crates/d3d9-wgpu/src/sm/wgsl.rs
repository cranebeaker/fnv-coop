//! Shader bytecode to WGSL: each Direct3D 9 instruction becomes WGSL with the Direct3D 9 meaning
//! (the DirectX 9 SDK's instruction reference), which differs from WGSL's in a few places the
//! game relies on:
//!
//! - **Multiplication**: `0 * x = 0` for any `x`, infinity and NaN included, in every instruction
//!   that multiplies (`mul`, `mad`, `dp2add`, `dp3`, `dp4`, the matrix instructions, `lrp`, `nrm`,
//!   `crs`, `dst`, `pow`'s exponent). This is the rule Direct3D 9 hardware follows and drivers
//!   still apply to Direct3D 9 shaders; an `nrm` of a zero vector is 0, not NaN.
//! - **`rsq`, `log`, `pow`** take the absolute value of their source; `log(0)` is -infinity.
//! - **`rcp`** of 1 is exactly 1.
//! - **`cmp`** selects on `src0 >= 0`, **`cnd`** on `src0 > 0.5`, **`slt`/`sge`** give 1.0 or
//!   0.0, **`frc`** is `x - floor(x)`, **`sincos`** writes cosine to x and sine to y.
//! - **Saturation** clamps to [0, 1] (`saturate`, NaN to 0 as the hardware does).
//! - **Address registers**: `mova` rounds to nearest (halves up: `floor(x + 0.5)`); shader model 1's `mov a0.x` takes the floor.
//!   A constant read outside the register file through an address register reads 0.
//! - **Pixel centres**: Direct3D 9 puts them on whole coordinates, wgpu half a pixel further. Each
//!   vertex shader moves its position by half a pixel right and down (`sys[0]`, set by the device
//!   from the viewport), so every pixel samples where Direct3D 9 would; `vPos` is the pixel's
//!   integer coordinate.
//!
//! The minimum and maximum (`min`, `max`) are WGSL's, which the hardware computes the way it
//! does for Direct3D 9 shaders; the SDK's pseudo-code (`src0 < src1 ? src0 : src1`) differs only
//! when an operand is NaN (left to measure against the real driver, DESIGN.md).
//!
//! Registers are module-scope `var<private>`s, so subroutines (`call`/`label`) see them;
//! `diagnostic(off, derivative_uniformity)` lets `texld` sit inside the shader's own flow control,
//! where Direct3D 9 allows it.

use super::{op, reg, usage, Dst, Error, Ins, Program, Reg, Src, Stage};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// A varying's meaning: `D3DDECLUSAGE` and its index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Semantic {
    pub usage: u8,
    pub index: u8,
}

/// How a vertex shader input arrives from the vertex declaration (`D3DDECLTYPE`, as the device
/// maps it to a wgpu vertex format).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Input {
    /// Not in the declaration: the shader reads (0, 0, 0, 1) (a guess, DESIGN.md).
    Missing,
    /// `n` floats (float, normalized and half types).
    Float(u8),
    /// Four unsigned bytes (`UBYTE4`).
    Uint4,
    /// `n` signed shorts (`SHORT2`, `SHORT4`).
    Sint(u8),
}

/// A sampler's texture type (`dcl_2d`, `dcl_cube`, `dcl_volume`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dim {
    D2,
    Cube,
    D3,
}

/// The register file size of the uniform block: 256 float constants (vs_3_0's count; pixel
/// shaders use 224 of them), 16 integer, 16 boolean.
pub const FLOAT_CONSTANTS: u32 = 256;
pub const INT_CONSTANTS: u32 = 16;
pub const BOOL_CONSTANTS: u32 = 16;

/// The uniform block both stages read (`Regs`), in bytes: `c`, `i`, `b` (16 words in 4
/// vectors), `sys` (4 vectors of the device's values) and `fix` (each sampler's fill mode, 16
/// words).
pub const REGS_SIZE: u64 = 16 * 256 + 16 * 16 + 16 * 4 + 16 * 4 + 16 * 4 + 16 * 4;

/// The binding of sampler register `n`'s texture in group 1 (its sampler is the next one):
/// pixel samplers from 0, vertex samplers from 32.
pub fn texture_binding(stage: Stage, n: u32) -> u32 {
    match stage {
        Stage::Pixel => 2 * n,
        Stage::Vertex => 32 + 2 * n,
    }
}

/// What a shader takes in and gives out.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Signature {
    /// Vertex shaders: each input register with its usage. Pixel shaders: each input register
    /// (`v#`, or `t#` before ps_3_0 as type `ADDR_TEXTURE`) with its varying.
    pub inputs: Vec<(Reg, Semantic)>,
    /// Whether each input (as `inputs`) is declared `_centroid`.
    pub centroid: Vec<bool>,
    /// Vertex shaders: each output register with its varying (position included).
    pub outputs: Vec<(Reg, Semantic)>,
    /// Sampler registers with their texture type.
    pub samplers: Vec<(u32, Dim)>,
    /// Whether the pixel shader reads `vPos` / `vFace`.
    pub position: bool,
    pub face: bool,
    /// The pixel shader writes `oDepth`.
    pub depth: bool,
    /// Colour outputs written (`oC#`), as a bit mask.
    pub colours: u8,
}

/// The varying a shader model 2 (or 1) output register stands for.
fn fixed_output(r: Reg) -> Option<Semantic> {
    let s = |usage, index| Some(Semantic { usage, index });
    match r.ty {
        reg::RASTOUT => match r.n {
            0 => s(usage::POSITION, 0),
            1 => s(usage::FOG, 0),
            _ => s(usage::PSIZE, 0),
        },
        reg::ATTROUT => s(usage::COLOR, r.n as u8),
        reg::OUTPUT => s(usage::TEXCOORD, r.n as u8),
        _ => None,
    }
}

/// Reads a shader's declarations (and, before vs_3_0, the output registers it writes).
pub fn signature(p: &Program) -> Signature {
    let mut sig = Signature::default();
    let mut outputs: BTreeMap<(u8, u32), Semantic> = BTreeMap::new();
    for ins in &p.ins {
        if ins.op == op::DCL {
            let (Some(d), Some(decl)) = (ins.dst, ins.decl) else {
                continue;
            };
            let semantic = Semantic {
                usage: decl.usage,
                index: decl.index,
            };
            match (p.stage, d.reg.ty) {
                (_, reg::SAMPLER) => sig.samplers.push((
                    d.reg.n,
                    match decl.texture {
                        3 => Dim::Cube,
                        4 => Dim::D3,
                        _ => Dim::D2,
                    },
                )),
                (Stage::Vertex, reg::INPUT) => sig.inputs.push((d.reg, semantic)),
                (Stage::Vertex, reg::OUTPUT) => {
                    outputs.insert((d.reg.ty, d.reg.n), semantic);
                }
                (Stage::Pixel, reg::INPUT) if p.major >= 3 => sig.inputs.push((d.reg, semantic)),
                (Stage::Pixel, reg::INPUT) => sig.inputs.push((
                    d.reg,
                    Semantic {
                        usage: usage::COLOR,
                        index: d.reg.n as u8,
                    },
                )),
                (Stage::Pixel, reg::ADDR_TEXTURE) => sig.inputs.push((
                    d.reg,
                    Semantic {
                        usage: usage::TEXCOORD,
                        index: d.reg.n as u8,
                    },
                )),
                (Stage::Pixel, reg::MISC) => {
                    if d.reg.n == 0 {
                        sig.position = true;
                    } else {
                        sig.face = true;
                    }
                }
                _ => {}
            }
            sig.centroid.resize(sig.inputs.len(), d.centroid);
        }
        if let Some(d) = ins.dst {
            if p.stage == Stage::Vertex && p.major < 3 && ins.op != op::DCL {
                if let Some(s) = fixed_output(d.reg) {
                    outputs.insert((d.reg.ty, d.reg.n), s);
                }
            }
            if p.stage == Stage::Pixel {
                // oC0 to oC3 (any other number is refused when it is written).
                if d.reg.ty == reg::COLOROUT && d.reg.n < 4 {
                    sig.colours |= 1 << d.reg.n;
                }
                if d.reg.ty == reg::DEPTHOUT {
                    sig.depth = true;
                }
            }
        }
    }
    // Shader model 1 vertex shaders declare inputs without a usage token's meaning for every
    // register; ps_2_0 colour inputs are read without a `dcl` in some compilers' output.
    sig.outputs = outputs
        .into_iter()
        .map(|((ty, n), s)| (Reg { ty, n }, s))
        .collect();
    sig
}

/// What a converted shader is linked to.
#[derive(Debug, Clone, Default)]
pub struct Link {
    /// Vertex shaders: how each input register `v#` arrives (by register number, 16 at most).
    pub inputs: Vec<Input>,
    /// The varyings between the two stages, in location order: a vertex shader writes these
    /// (0 for any it does not write), a pixel shader reads them.
    pub varyings: Vec<Semantic>,
    /// Whether each varying (as `varyings`) is interpolated at the centroid: the pixel shader's
    /// `dcl_centroid`, which wgpu wants on both stages.
    pub centroid: Vec<bool>,
}

const COMPONENTS: [char; 4] = ['x', 'y', 'z', 'w'];

/// The common functions every converted shader starts with.
const PRELUDE: &str = "diagnostic(off, derivative_uniformity);

struct Regs {
    c: array<vec4<f32>, 256>,
    i: array<vec4<i32>, 16>,
    b: array<vec4<u32>, 4>,
    sys: array<vec4<f32>, 4>,
    fix: array<vec4<u32>, 4>,
    bias: array<vec4<f32>, 4>,
}

// Direct3D 9 multiplication: 0 times anything is 0.
fn m4(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return select(a * b, vec4<f32>(0.0), (a == vec4<f32>(0.0)) | (b == vec4<f32>(0.0)));
}
fn m1(a: f32, b: f32) -> f32 {
    return select(a * b, 0.0, (a == 0.0) | (b == 0.0));
}
fn dp3(a: vec4<f32>, b: vec4<f32>) -> f32 {
    let p = m4(a, b);
    return p.x + p.y + p.z;
}
fn dp4(a: vec4<f32>, b: vec4<f32>) -> f32 {
    let p = m4(a, b);
    return p.x + p.y + p.z + p.w;
}
fn rcp(x: f32) -> f32 {
    return select(1.0 / x, 1.0, x == 1.0);
}
fn rsq(x: f32) -> f32 {
    return select(inverseSqrt(abs(x)), 1.0, x == 1.0);
}
fn lg(x: f32) -> f32 {
    return log2(abs(x));
}
fn pw(a: f32, b: f32) -> f32 {
    return exp2(m1(b, log2(abs(a))));
}
fn nrm(a: vec4<f32>) -> vec4<f32> {
    return m4(a, vec4<f32>(inverseSqrt(dp3(a, a))));
}
fn lrp(s: vec4<f32>, a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return m4(s, a - b) + b;
}
fn crs(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(m1(a.y, b.z) - m1(a.z, b.y), m1(a.z, b.x) - m1(a.x, b.z), m1(a.x, b.y) - m1(a.y, b.x), 0.0);
}
fn lit(s: vec4<f32>) -> vec4<f32> {
    var r = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    if (s.x > 0.0) {
        r.y = s.x;
        if (s.y > 0.0) {
            r.z = pw(s.y, clamp(s.w, -128.0, 128.0));
        }
    }
    return r;
}
fn dst4(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(1.0, m1(a.y, b.y), a.z, b.w);
}
fn slt(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return select(vec4<f32>(0.0), vec4<f32>(1.0), a < b);
}
fn sge(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return select(vec4<f32>(0.0), vec4<f32>(1.0), a >= b);
}
fn cmp(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>) -> vec4<f32> {
    return select(c, b, a >= vec4<f32>(0.0));
}
fn cnd(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>) -> vec4<f32> {
    return select(c, b, a > vec4<f32>(0.5));
}
fn frc(a: vec4<f32>) -> vec4<f32> {
    return a - floor(a);
}
";

/// A texture's sampled value with the components its format does not have filled in as
/// Direct3D 9 fills them (`fix` mode: 0 as sampled, 1 (r, 1, 1, 1), 2 (r, g, 1, 1), 3 alpha 1,
/// 4 (0, 0, 0, r) for A8, 5 (r, r, r, 1) for L8 and L16, 6 (r, r, r, g) for A8L8).
const FIX: &str = "fn fixup(t: vec4<f32>, mode: u32) -> vec4<f32> {
    switch mode {
        case 1u: { return vec4<f32>(t.x, 1.0, 1.0, 1.0); }
        case 2u: { return vec4<f32>(t.x, t.y, 1.0, 1.0); }
        case 3u: { return vec4<f32>(t.xyz, 1.0); }
        case 4u: { return vec4<f32>(0.0, 0.0, 0.0, t.x); }
        case 5u: { return vec4<f32>(t.x, t.x, t.x, 1.0); }
        case 6u: { return vec4<f32>(t.x, t.x, t.x, t.y); }
        default: { return t; }
    }
}
";

struct Gen<'a> {
    p: &'a Program,
    sig: Signature,
    link: &'a Link,
    out: String,
    depth: usize,
    /// Constants the shader defines (`def`), by register: they replace the device's values.
    defs: BTreeMap<u32, [u32; 4]>,
    defis: BTreeMap<u32, [u32; 4]>,
    defbs: BTreeMap<u32, u32>,
    /// Whether any float constant is read through an address register.
    relative_constants: bool,
    /// Loops open, for `aL`.
    loops: usize,
    temps: BTreeSet<u32>,
    labels: BTreeSet<u32>,
}

/// Converts a shader to WGSL, linked as `link` says.
pub fn convert(p: &Program, link: &Link) -> Result<String, Error> {
    if p.stage == Stage::Pixel && p.major < 2 {
        return Err(Error::Unsupported(format!("{} pixel shaders", p.profile())));
    }
    let mut g = Gen {
        p,
        sig: signature(p),
        link,
        out: String::new(),
        depth: 0,
        defs: BTreeMap::new(),
        defis: BTreeMap::new(),
        defbs: BTreeMap::new(),
        relative_constants: false,
        loops: 0,
        temps: BTreeSet::new(),
        labels: BTreeSet::new(),
    };
    for ins in &p.ins {
        match ins.op {
            op::DEF | op::DEFI | op::DEFB => {
                let d = ins
                    .dst
                    .ok_or_else(|| Error::BadBytecode("a def without its register".into()))?;
                match ins.op {
                    op::DEF => g.defs.insert(d.reg.n, ins.values),
                    op::DEFI => g.defis.insert(d.reg.n, ins.values),
                    _ => g.defbs.insert(d.reg.n, ins.values[0]).map(|v| [v; 4]),
                };
            }
            op::LABEL => {
                let l = ins
                    .src
                    .first()
                    .ok_or_else(|| Error::BadBytecode("a label without its number".into()))?;
                g.labels.insert(l.reg.n);
            }
            _ => {}
        }
        for s in &ins.src {
            if s.reg.ty == reg::CONST && s.rel.is_some() {
                g.relative_constants = true;
            }
            if s.reg.ty == reg::TEMP {
                g.temps.insert(s.reg.n);
            }
        }
        if let Some(d) = ins.dst {
            if d.reg.ty == reg::TEMP {
                g.temps.insert(d.reg.n);
            }
            if d.rel.is_some() {
                return Err(Error::Unsupported(
                    "a destination indexed by a register".into(),
                ));
            }
        }
        if matches!(ins.op, op::M4X4 | op::M4X3 | op::M3X4 | op::M3X3 | op::M3X2) {
            if let Some(s) = ins.src.get(1) {
                if s.reg.ty == reg::TEMP {
                    for k in 0..4 {
                        g.temps.insert(s.reg.n + k);
                    }
                }
            }
        }
    }
    g.module()?;
    Ok(g.out)
}

fn float(bits: u32) -> String {
    let f = f32::from_bits(bits);
    if f.is_finite() && f == f.trunc() && f.abs() < 1e7 {
        format!("{f:.1}")
    } else {
        format!("bitcast<f32>({bits:#010x}u)")
    }
}

impl Gen<'_> {
    fn line(&mut self, s: &str) {
        for _ in 0..self.depth {
            self.out.push_str("    ");
        }
        self.out.push_str(s);
        self.out.push('\n');
    }

    fn vertex(&self) -> bool {
        self.p.stage == Stage::Vertex
    }

    fn module(&mut self) -> Result<(), Error> {
        self.out.push_str(PRELUDE);
        let group0 = if self.vertex() { 0 } else { 1 };
        let _ = writeln!(
            self.out,
            "@group(0) @binding({group0}) var<uniform> R: Regs;\n"
        );
        if !self.sig.samplers.is_empty() {
            self.out.push_str(FIX);
        }
        for &(n, dim) in &self.sig.samplers.clone() {
            let ty = match dim {
                Dim::D2 => "texture_2d<f32>",
                Dim::Cube => "texture_cube<f32>",
                Dim::D3 => "texture_3d<f32>",
            };
            let b = texture_binding(self.p.stage, n);
            let _ = writeln!(
                self.out,
                "@group(1) @binding({b}) var t{n}: {ty};\n@group(1) @binding({}) var s{n}: sampler;",
                b + 1
            );
        }
        // Registers.
        for &n in &self.temps.clone() {
            let _ = writeln!(self.out, "var<private> r{n}: vec4<f32>;");
        }
        let _ = writeln!(self.out, "var<private> a0: vec4<i32>;");
        let _ = writeln!(self.out, "var<private> aL: i32;");
        let _ = writeln!(self.out, "var<private> p0: vec4<bool>;");
        if self.vertex() {
            for (r, _) in &self.sig.inputs.clone() {
                let _ = writeln!(self.out, "var<private> v{}: vec4<f32>;", r.n);
            }
            let _ = writeln!(self.out, "var<private> o_pos: vec4<f32>;");
            // `oPos` is `o_pos` (above); every other output register its own variable.
            for (r, _) in &self.sig.outputs.clone() {
                if !(r.ty == reg::RASTOUT && r.n == 0) {
                    let _ = writeln!(self.out, "var<private> {}: vec4<f32>;", out_name(*r));
                }
            }
        } else {
            for (r, _) in &self.sig.inputs.clone() {
                let _ = writeln!(self.out, "var<private> {}: vec4<f32>;", in_name(*r));
            }
            if self.sig.position {
                let _ = writeln!(self.out, "var<private> vpos: vec4<f32>;");
            }
            if self.sig.face {
                let _ = writeln!(self.out, "var<private> vface: vec4<f32>;");
            }
            for k in 0..4 {
                if self.sig.colours & (1 << k) != 0 {
                    let _ = writeln!(self.out, "var<private> oC{k}: vec4<f32>;");
                }
            }
            if self.sig.depth {
                let _ = writeln!(self.out, "var<private> odepth: vec4<f32>;");
            }
        }
        if self.relative_constants && !self.defs.is_empty() {
            self.out
                .push_str("fn cr(i: i32) -> vec4<f32> {\n    switch i {\n");
            for (n, v) in &self.defs.clone() {
                let _ = writeln!(
                    self.out,
                    "        case {n}: {{ return vec4<f32>({}, {}, {}, {}); }}",
                    float(v[0]),
                    float(v[1]),
                    float(v[2]),
                    float(v[3])
                );
            }
            self.out.push_str("        default: { if (i < 0 || i > 255) { return vec4<f32>(0.0); } return R.c[i]; }\n    }\n}\n");
        } else if self.relative_constants {
            self.out.push_str("fn cr(i: i32) -> vec4<f32> {\n    if (i < 0 || i > 255) { return vec4<f32>(0.0); }\n    return R.c[i];\n}\n");
        }
        // The body: the main program, then each subroutine.
        let ins = &self.p.ins;
        let main_end = ins
            .iter()
            .position(|i| i.op == op::LABEL)
            .unwrap_or(ins.len());
        let mut k = main_end;
        while k < ins.len() {
            let n = ins[k]
                .src
                .first()
                .ok_or_else(|| Error::BadBytecode("a label without its number".into()))?
                .reg
                .n;
            let end = ins[k + 1..]
                .iter()
                .position(|i| i.op == op::LABEL)
                .map_or(ins.len(), |e| k + 1 + e);
            let _ = writeln!(self.out, "fn l{n}() {{");
            self.depth = 1;
            for (i, x) in ins.iter().enumerate().take(end).skip(k + 1) {
                if x.op == op::RET && i + 1 == end {
                    continue;
                }
                self.instruction(x)?;
            }
            self.closed()?;
            self.depth = 0;
            self.out.push_str("}\n");
            k = end;
        }
        self.out.push_str("fn body() {\n");
        self.depth = 1;
        for (i, x) in ins[..main_end].iter().enumerate() {
            if x.op == op::RET && i + 1 == main_end {
                continue;
            }
            self.instruction(x)?;
        }
        self.closed()?;
        self.depth = 0;
        self.out.push_str("}\n");
        if self.vertex() {
            self.vertex_entry();
        } else {
            self.pixel_entry();
        }
        Ok(())
    }

    fn vertex_entry(&mut self) {
        let inputs = self.link.inputs.clone();
        let mut fields = Vec::new();
        let mut loads = Vec::new();
        for (r, _) in &self.sig.inputs {
            let n = r.n as usize;
            let input = inputs.get(n).copied().unwrap_or(Input::Missing);
            let (ty, expand) = match input {
                Input::Missing => {
                    loads.push(format!("    v{n} = vec4<f32>(0.0, 0.0, 0.0, 1.0);"));
                    continue;
                }
                Input::Float(1) => ("f32", "vec4<f32>(x, 0.0, 0.0, 1.0)"),
                Input::Float(2) => ("vec2<f32>", "vec4<f32>(x, 0.0, 1.0)"),
                Input::Float(3) => ("vec3<f32>", "vec4<f32>(x, 1.0)"),
                Input::Float(_) => ("vec4<f32>", "x"),
                Input::Uint4 => ("vec4<u32>", "vec4<f32>(x)"),
                Input::Sint(2) => ("vec2<i32>", "vec4<f32>(vec2<f32>(x), 0.0, 1.0)"),
                Input::Sint(_) => ("vec4<i32>", "vec4<f32>(x)"),
            };
            fields.push(format!("    @location({n}) v{n}: {ty},"));
            loads.push(format!(
                "    v{n} = {};",
                expand.replace('x', &format!("input.v{n}"))
            ));
        }
        let mut s = String::from("struct In {\n");
        if fields.is_empty() {
            // WGSL wants a struct member: an unused vertex index.
            s.push_str("    @builtin(vertex_index) index: u32,\n");
        }
        for f in &fields {
            s.push_str(f);
            s.push('\n');
        }
        s.push_str("}\nstruct Out {\n    @builtin(position) position: vec4<f32>,\n");
        for (l, v) in self.link.varyings.iter().enumerate() {
            let interp = if self.link.centroid.get(l).copied().unwrap_or(false) {
                " @interpolate(perspective, centroid)"
            } else {
                ""
            };
            let _ = writeln!(s, "    @location({l}){interp} l{l}: vec4<f32>,  // {v:?}");
        }
        s.push_str("}\n@vertex\nfn main(input: In) -> Out {\n");
        for l in &loads {
            s.push_str(l);
            s.push('\n');
        }
        s.push_str("    body();\n    var out: Out;\n");
        let position = self
            .sig
            .outputs
            .iter()
            .find(|(_, s)| s.usage == usage::POSITION && s.index == 0)
            .map(|(r, _)| *r);
        let pos = match position {
            Some(r) if r.ty == reg::RASTOUT => "o_pos".to_string(),
            Some(r) => out_name(r),
            None => "o_pos".to_string(),
        };
        // Half a pixel right and down: Direct3D 9's pixel centres (module documentation).
        let _ = writeln!(
            s,
            "    out.position = {pos} + vec4<f32>(R.sys[0].xy * {pos}.w, 0.0, 0.0);"
        );
        for (l, v) in self.link.varyings.iter().enumerate() {
            let written = self
                .sig
                .outputs
                .iter()
                .find(|(_, s)| s == v)
                .map(|(r, _)| *r);
            let value = match written {
                Some(r) if r.ty == reg::RASTOUT && r.n == 0 => "o_pos".into(),
                // Before vs_3_0 the colour outputs are clamped to [0, 1].
                Some(r) if r.ty == reg::ATTROUT && self.p.major < 3 => {
                    format!("saturate({})", out_name(r))
                }
                Some(r) => out_name(r),
                None => "vec4<f32>(0.0)".into(),
            };
            let _ = writeln!(s, "    out.l{l} = {value};");
        }
        s.push_str("    return out;\n}\n");
        self.out.push_str(&s);
    }

    fn pixel_entry(&mut self) {
        let mut s = String::from("struct In {\n    @builtin(position) frag: vec4<f32>,\n");
        if self.sig.face {
            s.push_str("    @builtin(front_facing) front: bool,\n");
        }
        for (l, v) in self.link.varyings.iter().enumerate() {
            let interp = if self.link.centroid.get(l).copied().unwrap_or(false) {
                " @interpolate(perspective, centroid)"
            } else {
                ""
            };
            let _ = writeln!(s, "    @location({l}){interp} l{l}: vec4<f32>,  // {v:?}");
        }
        s.push_str("}\nstruct Out {\n");
        for k in 0..4 {
            if self.sig.colours & (1 << k) != 0 {
                let _ = writeln!(s, "    @location({k}) c{k}: vec4<f32>,");
            }
        }
        if self.sig.depth {
            s.push_str("    @builtin(frag_depth) depth: f32,\n");
        }
        if self.sig.colours == 0 && !self.sig.depth {
            s.push_str("    @location(0) c0: vec4<f32>,\n");
        }
        s.push_str("}\n@fragment\nfn main(input: In) -> Out {\n");
        for (r, sem) in &self.sig.inputs {
            let l = self.link.varyings.iter().position(|v| v == sem);
            let value = match l {
                Some(l) => format!("input.l{l}"),
                None => "vec4<f32>(0.0)".into(),
            };
            let _ = writeln!(s, "    {} = {value};", in_name(*r));
        }
        if self.sig.position {
            s.push_str("    vpos = vec4<f32>(input.frag.xy - vec2<f32>(0.5), 0.0, 1.0);\n");
        }
        if self.sig.face {
            s.push_str("    vface = vec4<f32>(select(-1.0, 1.0, input.front));\n");
        }
        s.push_str("    body();\n");
        // The fixed-function alpha test (`D3DRS_ALPHATESTENABLE`), as the device sets it:
        // sys[0].x the reference (0..1), sys[0].y the comparison (`D3DCMPFUNC`, 8 = always).
        if self.sig.colours & 1 != 0 {
            s.push_str(
                "    let a = saturate(oC0.w);\n    let f = u32(R.sys[0].y);\n    let aref = R.sys[0].x;\n    if (f != 8u) {\n        var ok = true;\n        switch f {\n            case 1u: { ok = false; }\n            case 2u: { ok = a < aref; }\n            case 3u: { ok = a == aref; }\n            case 4u: { ok = a <= aref; }\n            case 5u: { ok = a > aref; }\n            case 6u: { ok = a != aref; }\n            case 7u: { ok = a >= aref; }\n            default: { ok = true; }\n        }\n        if (!ok) { discard; }\n    }\n",
            );
        }
        s.push_str("    var out: Out;\n");
        for k in 0..4 {
            if self.sig.colours & (1 << k) != 0 {
                let _ = writeln!(s, "    out.c{k} = oC{k};");
            }
        }
        if self.sig.depth {
            s.push_str("    out.depth = saturate(odepth.x);\n");
        }
        s.push_str("    return out;\n}\n");
        self.out.push_str(&s);
    }

    /// A source register's value as a vec4 expression, before swizzle and modifier.
    fn register(&mut self, s: &Src) -> Result<String, Error> {
        let n = s.reg.n;
        Ok(match s.reg.ty {
            reg::TEMP => format!("r{n}"),
            reg::INPUT if self.vertex() => format!("v{n}"),
            reg::INPUT => in_name(s.reg),
            reg::ADDR_TEXTURE if !self.vertex() => in_name(s.reg),
            reg::ADDR_TEXTURE => "vec4<f32>(a0)".into(),
            reg::CONST => match s.rel {
                Some(rel) => {
                    let index = match rel.reg.ty {
                        reg::LOOP => "aL".to_string(),
                        _ => format!("a0.{}", COMPONENTS[rel.comp as usize]),
                    };
                    format!("cr(i32({n}) + {index})")
                }
                None => match self.defs.get(&n) {
                    Some(v) => format!(
                        "vec4<f32>({}, {}, {}, {})",
                        float(v[0]),
                        float(v[1]),
                        float(v[2]),
                        float(v[3])
                    ),
                    None => format!("R.c[{n}]"),
                },
            },
            reg::CONSTINT => format!("vec4<f32>({})", self.int_constant(n)),
            reg::CONSTBOOL => format!("vec4<f32>(select(0.0, 1.0, {}))", self.bool_constant(n)),
            reg::LOOP => "vec4<f32>(f32(aL))".into(),
            reg::MISC if n == 0 => "vpos".into(),
            reg::MISC => "vface".into(),
            reg::PREDICATE => "vec4<f32>(p0)".into(),
            other => {
                return Err(Error::Unsupported(format!(
                    "reading register type {other} in {}",
                    self.p.profile()
                )))
            }
        })
    }

    fn int_constant(&self, n: u32) -> String {
        match self.defis.get(&n) {
            Some(v) => format!(
                "vec4<i32>({}, {}, {}, {})",
                v[0] as i32, v[1] as i32, v[2] as i32, v[3] as i32
            ),
            None => format!("R.i[{n}]"),
        }
    }

    fn bool_constant(&self, n: u32) -> String {
        match self.defbs.get(&n) {
            Some(v) => format!("{}", *v != 0),
            None => format!("(R.b[{}][{}] != 0u)", n / 4, n % 4),
        }
    }

    /// A source operand as a vec4 expression: register, swizzle, modifier.
    fn src(&mut self, s: &Src) -> Result<String, Error> {
        let base = self.register(s)?;
        let v = if s.swizzle == 0xE4 {
            base
        } else {
            let sw: String = (0..4)
                .map(|k| COMPONENTS[s.component(k) as usize])
                .collect();
            format!("({base}).{sw}")
        };
        Ok(match s.modifier {
            0 => v,
            1 => format!("(-{v})"),
            2 => format!("({v} - vec4<f32>(0.5))"),
            3 => format!("(vec4<f32>(0.5) - {v})"),
            4 => format!("({v} * 2.0 - vec4<f32>(1.0))"),
            5 => format!("(vec4<f32>(1.0) - {v} * 2.0)"),
            6 => format!("(vec4<f32>(1.0) - {v})"),
            7 => format!("({v} * 2.0)"),
            8 => format!("({v} * -2.0)"),
            11 => format!("abs({v})"),
            12 => format!("(-abs({v}))"),
            m => {
                return Err(Error::Unsupported(format!("source modifier {m}")));
            }
        })
    }

    /// A source's first swizzled component as a scalar (what scalar instructions read).
    fn scalar(&mut self, s: &Src) -> Result<String, Error> {
        Ok(format!("{}.x", self.src(s)?))
    }

    /// Writes `value` (a vec4 expression) to the destination, masked, saturated, predicated.
    fn write(&mut self, ins: &Ins, value: String) -> Result<(), Error> {
        let d = ins
            .dst
            .ok_or_else(|| Error::BadBytecode("an instruction without its destination".into()))?;
        let value = if d.sat {
            format!("saturate({value})")
        } else {
            value
        };
        let target = match d.reg.ty {
            reg::TEMP => format!("r{}", d.reg.n),
            reg::ADDR_TEXTURE if self.vertex() => {
                // `mova` rounds to nearest, halves up (floor(x + 0.5)); shader model 1's `mov a0.x`
                // takes the floor.
                let rounded = if ins.op == op::MOVA {
                    format!("vec4<i32>(floor({value} + vec4<f32>(0.5)))")
                } else {
                    format!("vec4<i32>(floor({value}))")
                };
                self.masked("a0", &d, rounded, ins)?;
                return Ok(());
            }
            reg::RASTOUT if d.reg.n == 0 => "o_pos".into(),
            reg::RASTOUT | reg::ATTROUT | reg::OUTPUT => out_name(d.reg),
            reg::COLOROUT => format!("oC{}", d.reg.n),
            reg::DEPTHOUT => "odepth".into(),
            reg::PREDICATE => {
                return Err(Error::Unsupported("predicate writes outside setp".into()))
            }
            other => return Err(Error::Unsupported(format!("writing register type {other}"))),
        };
        self.masked(&target, &d, value, ins)
    }

    fn masked(&mut self, target: &str, d: &Dst, value: String, ins: &Ins) -> Result<(), Error> {
        let mask = if d.mask == 0 { 0xF } else { d.mask };
        let mut condition = if mask == 0xF {
            None
        } else {
            let b: Vec<&str> = (0..4)
                .map(|k| {
                    if mask & (1 << k) != 0 {
                        "true"
                    } else {
                        "false"
                    }
                })
                .collect();
            Some(format!("vec4<bool>({})", b.join(", ")))
        };
        if let Some(p) = &ins.predicate {
            let sw: String = (0..4)
                .map(|k| COMPONENTS[p.component(k) as usize])
                .collect();
            let pred = if p.modifier == 13 {
                format!("!p0.{sw}")
            } else {
                format!("p0.{sw}")
            };
            condition = Some(match condition {
                Some(c) => format!("({c} & {pred})"),
                None => pred,
            });
        }
        let line = match condition {
            None => format!("{target} = {value};"),
            Some(c) => format!("{target} = select({target}, {value}, {c});"),
        };
        self.line(&line);
        Ok(())
    }

    fn comparison(controls: u8) -> Result<&'static str, Error> {
        Ok(match controls {
            1 => ">",
            2 => "==",
            3 => ">=",
            4 => "<",
            5 => "!=",
            6 => "<=",
            c => return Err(Error::Unsupported(format!("comparison {c}"))),
        })
    }

    /// A boolean condition from `if`'s, `callnz`'s or `breakp`'s source (a boolean constant or
    /// the predicate).
    fn condition(&mut self, s: &Src) -> Result<String, Error> {
        Ok(match s.reg.ty {
            reg::CONSTBOOL => self.bool_constant(s.reg.n),
            reg::PREDICATE => {
                let c = COMPONENTS[s.component(0) as usize];
                if s.modifier == 13 {
                    format!("!p0.{c}")
                } else {
                    format!("p0.{c}")
                }
            }
            other => {
                return Err(Error::Unsupported(format!(
                    "condition on register type {other}"
                )))
            }
        })
    }

    fn sample(&mut self, ins: &Ins) -> Result<String, Error> {
        let coords = self.src(&ins.src[0])?;
        let sampler = ins.src[1];
        let n = sampler.reg.n;
        let dim = self
            .sig
            .samplers
            .iter()
            .find(|(s, _)| *s == n)
            .map_or(Dim::D2, |(_, d)| *d);
        let c = match dim {
            Dim::D2 => "xy",
            Dim::Cube | Dim::D3 => "xyz",
        };
        // `D3DSAMP_MIPMAPLODBIAS` (wgpu samplers have no bias): from the uniform block.
        let bias = format!("R.bias[{}][{}]", n / 4, n % 4);
        let call = match ins.op {
            op::TEXLDL => format!("textureSampleLevel(t{n}, s{n}, ({coords}).{c}, ({coords}).w)"),
            op::TEXLDD => {
                let (dx, dy) = (self.src(&ins.src[2])?, self.src(&ins.src[3])?);
                format!("textureSampleGrad(t{n}, s{n}, ({coords}).{c}, ({dx}).{c}, ({dy}).{c})")
            }
            _ if self.vertex() => {
                format!("textureSampleLevel(t{n}, s{n}, ({coords}).{c}, 0.0)")
            }
            _ => match ins.controls {
                1 => format!("textureSampleBias(t{n}, s{n}, ({coords}).{c} / ({coords}).w, {bias})"),
                2 => format!(
                    "textureSampleBias(t{n}, s{n}, ({coords}).{c}, clamp(({coords}).w + {bias}, -16.0, 15.99))"
                ),
                _ => format!("textureSampleBias(t{n}, s{n}, ({coords}).{c}, {bias})"),
            },
        };
        let fixed = format!("fixup({call}, R.fix[{}][{}])", n / 4, n % 4);
        // ps_2_x and ps_3_0 allow a swizzle on the sampler: it applies to the result.
        Ok(if sampler.swizzle == 0xE4 {
            fixed
        } else {
            let sw: String = (0..4)
                .map(|k| COMPONENTS[sampler.component(k) as usize])
                .collect();
            format!("({fixed}).{sw}")
        })
    }

    /// The block level one out, or an error for a closing instruction without its opening one.
    fn outer(&self) -> Result<usize, Error> {
        self.depth.checked_sub(1).filter(|&d| d > 0).ok_or_else(|| {
            Error::BadBytecode("an else, endif, endloop or endrep without its start".into())
        })
    }

    fn loop_out(&self) -> Result<usize, Error> {
        self.loops
            .checked_sub(1)
            .ok_or_else(|| Error::BadBytecode("an endloop or endrep without its loop".into()))
    }

    /// At a body's end: every block it opened is closed.
    fn closed(&self) -> Result<(), Error> {
        if self.depth != 1 || self.loops != 0 {
            return Err(Error::BadBytecode(
                "an if, loop or rep without its end".into(),
            ));
        }
        Ok(())
    }

    fn instruction(&mut self, ins: &Ins) -> Result<(), Error> {
        let s = ins.src.clone();
        // Every instruction has the parameters it reads (malformed bytecode is an error, never a
        // panic).
        let (destination, sources) = operands(ins.op);
        if (destination && ins.dst.is_none()) || ins.src.len() < sources {
            return Err(Error::BadBytecode(format!(
                "{} with too few parameters",
                super::opcode_name(ins.op).unwrap_or("an instruction")
            )));
        }
        match ins.op {
            op::NOP | op::DCL | op::DEF | op::DEFI | op::DEFB | op::PHASE => {}
            op::MOV => {
                let v = self.src(&s[0])?;
                self.write(ins, v)?;
            }
            op::MOVA => {
                let v = self.src(&s[0])?;
                self.write(ins, v)?;
            }
            op::ADD | op::SUB => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                let o = if ins.op == op::ADD { "+" } else { "-" };
                self.write(ins, format!("({a} {o} {b})"))?;
            }
            op::MUL => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                self.write(ins, format!("m4({a}, {b})"))?;
            }
            op::MAD => {
                let (a, b, c) = (self.src(&s[0])?, self.src(&s[1])?, self.src(&s[2])?);
                self.write(ins, format!("(m4({a}, {b}) + {c})"))?;
            }
            op::DP3 | op::DP4 => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                let f = if ins.op == op::DP3 { "dp3" } else { "dp4" };
                self.write(ins, format!("vec4<f32>({f}({a}, {b}))"))?;
            }
            op::DP2ADD => {
                let (a, b, c) = (self.src(&s[0])?, self.src(&s[1])?, self.scalar(&s[2])?);
                self.write(
                    ins,
                    format!("vec4<f32>(m1(({a}).x, ({b}).x) + m1(({a}).y, ({b}).y) + {c})"),
                )?;
            }
            op::MIN | op::MAX => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                let f = if ins.op == op::MIN { "min" } else { "max" };
                self.write(ins, format!("{f}({a}, {b})"))?;
            }
            op::SLT | op::SGE => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                let f = if ins.op == op::SLT { "slt" } else { "sge" };
                self.write(ins, format!("{f}({a}, {b})"))?;
            }
            op::RCP | op::RSQ | op::EXP | op::EXPP | op::LOG | op::LOGP => {
                let x = self.scalar(&s[0])?;
                let f = match ins.op {
                    op::RCP => "rcp",
                    op::RSQ => "rsq",
                    op::EXP | op::EXPP => "exp2",
                    _ => "lg",
                };
                self.write(ins, format!("vec4<f32>({f}({x}))"))?;
            }
            op::POW => {
                let (a, b) = (self.scalar(&s[0])?, self.scalar(&s[1])?);
                self.write(ins, format!("vec4<f32>(pw({a}, {b}))"))?;
            }
            op::FRC => {
                let a = self.src(&s[0])?;
                self.write(ins, format!("frc({a})"))?;
            }
            op::ABS => {
                let a = self.src(&s[0])?;
                self.write(ins, format!("abs({a})"))?;
            }
            op::SGN => {
                let a = self.src(&s[0])?;
                self.write(ins, format!("sign({a})"))?;
            }
            op::NRM => {
                let a = self.src(&s[0])?;
                self.write(ins, format!("nrm({a})"))?;
            }
            op::CRS => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                self.write(ins, format!("crs({a}, {b})"))?;
            }
            op::LRP => {
                let (a, b, c) = (self.src(&s[0])?, self.src(&s[1])?, self.src(&s[2])?);
                self.write(ins, format!("lrp({a}, {b}, {c})"))?;
            }
            op::CMP | op::CND => {
                let (a, b, c) = (self.src(&s[0])?, self.src(&s[1])?, self.src(&s[2])?);
                let f = if ins.op == op::CMP { "cmp" } else { "cnd" };
                self.write(ins, format!("{f}({a}, {b}, {c})"))?;
            }
            op::SINCOS => {
                let a = self.scalar(&s[0])?;
                self.write(ins, format!("vec4<f32>(cos({a}), sin({a}), 0.0, 0.0)"))?;
            }
            op::LIT => {
                let a = self.src(&s[0])?;
                self.write(ins, format!("lit({a})"))?;
            }
            op::DST => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                self.write(ins, format!("dst4({a}, {b})"))?;
            }
            op::M4X4 | op::M4X3 | op::M3X4 | op::M3X3 | op::M3X2 => {
                let (rows, dot) = match ins.op {
                    op::M4X4 => (4, "dp4"),
                    op::M4X3 => (3, "dp4"),
                    op::M3X4 => (4, "dp3"),
                    op::M3X3 => (3, "dp3"),
                    _ => (2, "dp3"),
                };
                let a = self.src(&s[0])?;
                let mut parts = Vec::new();
                for k in 0..rows {
                    let mut row = s[1];
                    row.reg.n += k;
                    let r = self.src(&row)?;
                    parts.push(format!("{dot}({a}, {r})"));
                }
                while parts.len() < 4 {
                    parts.push("0.0".into());
                }
                self.write(ins, format!("vec4<f32>({})", parts.join(", ")))?;
            }
            op::TEX | op::TEXLDL | op::TEXLDD => {
                let v = self.sample(ins)?;
                self.write(ins, v)?;
            }
            op::TEXKILL => {
                let d = ins.dst.expect("texkill's register");
                let src = Src {
                    reg: d.reg,
                    swizzle: 0xE4,
                    modifier: 0,
                    rel: None,
                };
                let v = self.src(&src)?;
                let mask = if d.mask == 0 { 0xF } else { d.mask };
                let tests: Vec<String> = (0..4)
                    .filter(|k| mask & (1 << k) != 0)
                    .map(|k| format!("({v}).{} < 0.0", COMPONENTS[k]))
                    .collect();
                self.line(&format!("if ({}) {{ discard; }}", tests.join(" || ")));
            }
            op::DSX | op::DSY => {
                let a = self.src(&s[0])?;
                let f = if ins.op == op::DSX { "dpdx" } else { "dpdy" };
                self.write(ins, format!("{f}({a})"))?;
            }
            op::SETP => {
                let (a, b) = (self.src(&s[0])?, self.src(&s[1])?);
                let c = Self::comparison(ins.controls)?;
                let d = ins.dst.expect("setp's predicate");
                let value = format!("({a} {c} {b})");
                let mask = if d.mask == 0 { 0xF } else { d.mask };
                let b: Vec<&str> = (0..4)
                    .map(|k| {
                        if mask & (1 << k) != 0 {
                            "true"
                        } else {
                            "false"
                        }
                    })
                    .collect();
                self.line(&format!(
                    "p0 = select(p0, {value}, vec4<bool>({}));",
                    b.join(", ")
                ));
            }
            op::IF => {
                let c = self.condition(&s[0])?;
                self.line(&format!("if ({c}) {{"));
                self.depth += 1;
            }
            op::IFC => {
                let (a, b) = (self.scalar(&s[0])?, self.scalar(&s[1])?);
                let c = Self::comparison(ins.controls)?;
                self.line(&format!("if ({a} {c} {b}) {{"));
                self.depth += 1;
            }
            op::ELSE => {
                self.depth = self.outer()?;
                self.line("} else {");
                self.depth += 1;
            }
            op::ENDIF => {
                self.depth = self.outer()?;
                self.line("}");
            }
            op::REP => {
                let i = self.int_constant(s[0].reg.n);
                let k = self.loops;
                self.loops += 1;
                self.line(&format!(
                    "for (var it{k} = 0; it{k} < clamp(({i}).x, 0, 255); it{k} = it{k} + 1) {{"
                ));
                self.depth += 1;
            }
            op::ENDREP => {
                self.loops = self.loop_out()?;
                self.depth = self.outer()?;
                self.line("}");
            }
            op::LOOP => {
                // loop aL, i#: i#.x iterations, aL from i#.y in steps of i#.z.
                let i = self.int_constant(s[1].reg.n);
                let k = self.loops;
                self.loops += 1;
                self.line("{");
                self.depth += 1;
                self.line(&format!("let li{k} = {i};"));
                self.line(&format!("let al{k} = aL;"));
                self.line(&format!("aL = li{k}.y;"));
                self.line(&format!(
                    "for (var it{k} = 0; it{k} < clamp(li{k}.x, 0, 255); it{k} = it{k} + 1) {{"
                ));
                self.depth += 1;
            }
            op::ENDLOOP => {
                self.loops = self.loop_out()?;
                let k = self.loops;
                self.line(&format!("aL = aL + li{k}.z;"));
                self.depth = self.outer()?;
                self.line("}");
                self.line(&format!("aL = al{k};"));
                self.depth = self.outer()?;
                self.line("}");
            }
            op::BREAK => self.line("break;"),
            op::BREAKC => {
                let (a, b) = (self.scalar(&s[0])?, self.scalar(&s[1])?);
                let c = Self::comparison(ins.controls)?;
                self.line(&format!("if ({a} {c} {b}) {{ break; }}"));
            }
            op::BREAKP => {
                let c = self.condition(&s[0])?;
                self.line(&format!("if ({c}) {{ break; }}"));
            }
            op::CALL => {
                let n = s[0].reg.n;
                self.line(&format!("l{n}();"));
            }
            op::CALLNZ => {
                let n = s[0].reg.n;
                let c = self.condition(&s[1])?;
                self.line(&format!("if ({c}) {{ l{n}(); }}"));
            }
            op::RET => self.line("return;"),
            other => {
                return Err(Error::Unsupported(format!(
                    "instruction {} in {}",
                    super::opcode_name(other).unwrap_or("?"),
                    self.p.profile()
                )))
            }
        }
        Ok(())
    }
}

/// What an instruction reads: whether it has a destination, and how many sources at least.
fn operands(code: u16) -> (bool, usize) {
    match code {
        op::MOV
        | op::MOVA
        | op::RCP
        | op::RSQ
        | op::EXP
        | op::EXPP
        | op::LOG
        | op::LOGP
        | op::FRC
        | op::ABS
        | op::SGN
        | op::NRM
        | op::SINCOS
        | op::LIT
        | op::DSX
        | op::DSY => (true, 1),
        op::ADD
        | op::SUB
        | op::MUL
        | op::DP3
        | op::DP4
        | op::MIN
        | op::MAX
        | op::SLT
        | op::SGE
        | op::POW
        | op::CRS
        | op::DST
        | op::M4X4
        | op::M4X3
        | op::M3X4
        | op::M3X3
        | op::M3X2
        | op::SETP
        | op::TEX
        | op::TEXLDL => (true, 2),
        op::MAD | op::LRP | op::CMP | op::CND | op::DP2ADD => (true, 3),
        op::TEXLDD => (true, 4),
        op::TEXKILL | op::DCL | op::DEF | op::DEFI | op::DEFB => (true, 0),
        op::IF | op::BREAKP | op::REP | op::CALL | op::LABEL => (false, 1),
        op::IFC | op::BREAKC | op::LOOP | op::CALLNZ => (false, 2),
        _ => (false, 0),
    }
}

/// The private variable of a vertex shader's output register.
fn out_name(r: Reg) -> String {
    match r.ty {
        reg::RASTOUT => match r.n {
            0 => "o_pos".into(),
            1 => "o_fog".into(),
            _ => "o_pts".into(),
        },
        reg::ATTROUT => format!("oD{}", r.n),
        _ => format!("o{}", r.n),
    }
}

/// The private variable of a pixel shader's input register.
fn in_name(r: Reg) -> String {
    match r.ty {
        reg::ADDR_TEXTURE => format!("t{}_in", r.n),
        _ => format!("v{}", r.n),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::small_vertex_shader};
    use super::*;

    pub(crate) fn validate(source: &str) {
        let module = naga::front::wgsl::parse_str(source)
            .unwrap_or_else(|e| panic!("{}\n{source}", e.emit_to_string(source)));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap_or_else(|e| panic!("{e:?}\n{source}"));
    }

    #[test]
    fn a_vertex_shader_converts_to_valid_wgsl() {
        let p = parse(&small_vertex_shader()).unwrap();
        let link = Link {
            inputs: vec![Input::Float(3)],
            varyings: vec![Semantic {
                usage: usage::TEXCOORD,
                index: 0,
            }],
            centroid: Vec::new(),
        };
        let source = convert(&p, &link).unwrap();
        validate(&source);
        assert!(source.contains("o_pos = vec4<f32>(dp4(v0, R.c[0]), dp4(v0, R.c[1]), dp4(v0, R.c[2]), dp4(v0, R.c[3]));"), "{source}");
        assert!(
            source.contains("o0 = select(o0, (v0).yxzw, vec4<bool>(true, true, false, false));"),
            "{source}"
        );
        assert!(source.contains("out.l0 = o0;"), "{source}");
    }

    fn converts(tokens: &[u32], link: &Link) -> String {
        let p = parse(tokens).unwrap();
        let source = convert(&p, link).unwrap();
        validate(&source);
        source
    }

    #[test]
    fn flow_control_converts_to_valid_wgsl() {
        // ps_3_0: defi i0; def c0; rep i0 { add; break_gt } if_lt { mov } else { mov -c0 }.
        let t = [
            0xFFFF_0300,
            0x0500_0030,
            0xF00F_0000,
            4,
            0,
            0,
            0,
            0x0500_0051,
            0xA00F_0000,
            0x3F80_0000,
            0,
            0,
            0,
            0x0100_0026,
            0xF0E4_0000,
            0x0300_0002,
            0x800F_0000,
            0x80E4_0000,
            0xA0E4_0000,
            0x0201_002D,
            0x8000_0000,
            0xA000_0000,
            0x0000_0027,
            0x0204_0029,
            0x8000_0000,
            0xA055_0000,
            0x0200_0001,
            0x800F_0000,
            0xA0E4_0000,
            0x0000_002A,
            0x0200_0001,
            0x800F_0000,
            0xA1E4_0000,
            0x0000_002B,
            0x0200_0001,
            0x800F_0800,
            0x80E4_0000,
            super::super::END,
        ];
        let s = converts(&t, &Link::default());
        assert!(
            s.contains("it0 < clamp((vec4<i32>(4, 0, 0, 0)).x, 0, 255)"),
            "{s}"
        );
        assert!(
            s.contains("if ((r0).x.x > (vec4<f32>(1.0, 0.0, 0.0, 0.0)).x.x) { break; }")
                || s.contains("break; }"),
            "{s}"
        );
        assert!(s.contains("} else {"), "{s}");
    }

    #[test]
    fn relative_constants_read_definitions_and_the_register_file() {
        // vs_3_0: def c10; dcl_position v0, o0; mova a0.x, v0.x; mov r0, c8[a0.x]; add o0, r0, v0.
        let t = [
            0xFFFE_0300,
            0x0500_0051,
            0xA00F_000A,
            0x3F80_0000,
            0x4000_0000,
            0x4040_0000,
            0x4080_0000,
            0x0200_001F,
            0x8000_0000,
            0x900F_0000,
            0x0200_001F,
            0x8000_0000,
            0xE00F_0000,
            0x0200_002E,
            0xB001_0000,
            0x9000_0000,
            0x0300_0001,
            0x800F_0000,
            0xA0E4_2008,
            0xB000_0000,
            0x0300_0002,
            0xE00F_0000,
            0x80E4_0000,
            0x90E4_0000,
            super::super::END,
        ];
        let link = Link {
            inputs: vec![Input::Float(4)],
            ..Link::default()
        };
        let s = converts(&t, &link);
        assert!(
            s.contains("case 10: { return vec4<f32>(1.0, 2.0, 3.0, 4.0); }"),
            "{s}"
        );
        assert!(s.contains("cr(i32(8) + a0.x)"), "{s}");
        assert!(
            s.contains("vec4<i32>(floor((v0).xxxx + vec4<f32>(0.5)))"),
            "{s}"
        );
        assert!(s.contains("out.position = o0 + "), "{s}");
    }

    #[test]
    fn texture_instructions_convert_to_valid_wgsl() {
        // ps_2_x: dcl t0; dcl_2d s0; dcl_cube s1; texld; texldp; texld (cube); dsx; texkill t0.
        let t = [
            0xFFFF_0201,
            0x0200_001F,
            0x8000_0000,
            0xB00F_0000,
            0x0200_001F,
            0x9000_0000,
            0xA00F_0800,
            0x0200_001F,
            0x9800_0000,
            0xA00F_0801,
            0x0300_0042,
            0x800F_0000,
            0xB0E4_0000,
            0xA0E4_0800,
            0x0301_0042,
            0x800F_0001,
            0xB0E4_0000,
            0xA0E4_0800,
            0x0300_0042,
            0x800F_0002,
            0xB0E4_0000,
            0xA0E4_0801,
            0x0200_005B,
            0x800F_0003,
            0xB0E4_0000,
            0x0100_0041,
            0xB00F_0000,
            0x0300_0002,
            0x800F_0000,
            0x80E4_0000,
            0x80E4_0001,
            0x0200_0001,
            0x800F_0800,
            0x80E4_0000,
            super::super::END,
        ];
        let link = Link {
            varyings: vec![Semantic {
                usage: usage::TEXCOORD,
                index: 0,
            }],
            ..Link::default()
        };
        let s = converts(&t, &link);
        assert!(s.contains("texture_cube<f32>"), "{s}");
        assert!(s.contains("/ (t0_in).w"), "{s}");
        assert!(s.contains("dpdx(t0_in)"), "{s}");
        assert!(s.contains("discard"), "{s}");
    }

    #[test]
    fn malformed_bytecode_is_an_error_not_a_panic() {
        let end = super::super::END;
        // endif without if; mov without its source; if without endif; a label without a number.
        for t in [
            vec![0xFFFF_0300, 0x0000_002B, end],
            vec![0xFFFF_0300, 0x0100_0001, 0x800F_0000, end],
            vec![0xFFFF_0300, 0x0100_0028, 0xE0E4_0800, end],
            vec![0xFFFF_0300, 0x0000_001E, end],
            vec![0xFFFF_0300, 0x0000_001B, 0x0000_001D, end],
        ] {
            let p = parse(&t).unwrap();
            assert!(convert(&p, &Link::default()).is_err(), "{t:x?}");
        }
        // Every token of these shaders changed in turn: read or not, converted or not, never a panic.
        let shaders = [
            small_vertex_shader(),
            vec![
                0xFFFF_0300,
                0x0500_0030,
                0xF00F_0000,
                4,
                0,
                0,
                0,
                0x0100_0026,
                0xF0E4_0000,
                0x0300_0002,
                0x800F_0000,
                0x80E4_0000,
                0xA0E4_0000,
                0x0000_0027,
                0x0200_0001,
                0x800F_0800,
                0x80E4_0000,
                end,
            ],
        ];
        let mut seed = 0x1234_5678u32;
        for shader in &shaders {
            for at in 1..shader.len() {
                for _ in 0..16 {
                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    let mut t = shader.clone();
                    t[at] = seed;
                    if let Ok(p) = parse(&t) {
                        let _ = convert(&p, &Link::default());
                    }
                }
            }
        }
    }

    #[test]
    fn the_signature_names_shader_model_2_outputs() {
        let p = parse(&small_vertex_shader()).unwrap();
        let sig = signature(&p);
        assert_eq!(sig.inputs.len(), 1);
        assert_eq!(
            sig.outputs
                .iter()
                .map(|(_, s)| (s.usage, s.index))
                .collect::<Vec<_>>(),
            vec![(usage::POSITION, 0), (usage::TEXCOORD, 0)]
        );
    }
}
