//! Milestone 3: the Direct3D 9 state of each draw, rebuilt from a trace of the device's calls
//! (`lists/d3d-*.txt`), and the draw-by-draw diff of two runs.
//!
//! Device methods are recorded under the port's addresses for them, `DEVICE_CODE + 4 * slot`
//! (the slot in `IDirect3DDevice9`'s vtable), and the D3DX texture loaders under the exe's import
//! slots, on both sides. Objects are compared by what made them, never by address: a texture
//! from a file by the hash of the file's bytes, a shader by its bytecode, a vertex declaration
//! by its elements, other resources by their creation parameters and their order among the
//! same, and an object the trace never saw created (a texture's surface, the back buffer) by
//! the order it was first used in. Creation calls a recorded D3DX call made itself (the real
//! D3DX creates its textures through the device) are left out: the port's D3DX does not go
//! through the device's slots.
//!
//! Replaying the `Set*` calls in order (sequence numbers are global in a trace) gives the state
//! at each draw. Only what the draw uses is kept: the samplers and constants its shaders read
//! (from their bytecode, when it was recorded), the texture stage states and transforms only
//! without shaders. Within each frame (split by `Present`), the two runs' draws are aligned by
//! their mesh (draw kind, primitive type, vertex and primitive counts, shaders) with a longest
//! common subsequence, then each aligned pair's states are compared.

use crate::format::{calls, Call, Trace};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::rc::Rc;

/// The port's code address of the device's slot 0.
pub const DEVICE_CODE: u32 = 0x7e02_0200;
/// The exe's import slots of the D3DX texture loaders.
pub const D3DX_TEXTURE: u32 = 0x00fd_f3fc;
pub const D3DX_CUBE: u32 = 0x00fd_f400;
pub const D3DX_VOLUME: u32 = 0x00fd_f404;
/// The exe's texture-from-file function (`__thiscall`, w1 the file name): `00e68a80`, unnamed in
/// the symbols (`crates/engine-fallout/src/units/unplaced/run_00e673f0.rs`).
pub const TEXTURE_FROM_FILE: u32 = 0x00e6_8a80;

/// `IDirect3DDevice9` slots (d3d9.h order).
pub mod slot {
    pub const PRESENT: u32 = 17;
    pub const CREATE_TEXTURE: u32 = 23;
    pub const CREATE_VOLUME_TEXTURE: u32 = 24;
    pub const CREATE_CUBE_TEXTURE: u32 = 25;
    pub const CREATE_VERTEX_BUFFER: u32 = 26;
    pub const CREATE_INDEX_BUFFER: u32 = 27;
    pub const CREATE_RENDER_TARGET: u32 = 28;
    pub const CREATE_DEPTH_STENCIL: u32 = 29;
    pub const STRETCH_RECT: u32 = 34;
    pub const SET_RENDER_TARGET: u32 = 37;
    pub const SET_DEPTH_STENCIL: u32 = 39;
    pub const CLEAR: u32 = 43;
    pub const SET_TRANSFORM: u32 = 44;
    pub const SET_VIEWPORT: u32 = 47;
    pub const SET_CLIP_PLANE: u32 = 55;
    pub const SET_RENDER_STATE: u32 = 57;
    pub const SET_TEXTURE: u32 = 65;
    pub const SET_STAGE_STATE: u32 = 67;
    pub const SET_SAMPLER_STATE: u32 = 69;
    pub const SET_SCISSOR: u32 = 75;
    pub const DRAW_PRIMITIVE: u32 = 81;
    pub const DRAW_INDEXED: u32 = 82;
    pub const DRAW_UP: u32 = 83;
    pub const DRAW_INDEXED_UP: u32 = 84;
    pub const CREATE_DECL: u32 = 86;
    pub const SET_DECL: u32 = 87;
    pub const SET_FVF: u32 = 89;
    pub const CREATE_VS: u32 = 91;
    pub const SET_VS: u32 = 92;
    pub const VS_CONST_F: u32 = 94;
    pub const VS_CONST_I: u32 = 96;
    pub const VS_CONST_B: u32 = 98;
    pub const SET_STREAM: u32 = 100;
    pub const SET_STREAM_FREQ: u32 = 102;
    pub const SET_INDICES: u32 = 104;
    pub const CREATE_PS: u32 = 106;
    pub const SET_PS: u32 = 107;
    pub const PS_CONST_F: u32 = 109;
    pub const PS_CONST_I: u32 = 111;
    pub const PS_CONST_B: u32 = 113;
}

/// The device slot a recorded address is, if it is one.
pub fn slot_of(addr: u32) -> Option<u32> {
    let off = addr.checked_sub(DEVICE_CODE)?;
    (off % 4 == 0 && off / 4 < 119).then_some(off / 4)
}

/// The name of a state-setting method, `None` for other slots.
pub fn set_name(s: u32) -> Option<&'static str> {
    Some(match s {
        slot::SET_RENDER_TARGET => "SetRenderTarget",
        slot::SET_DEPTH_STENCIL => "SetDepthStencilSurface",
        slot::SET_TRANSFORM => "SetTransform",
        slot::SET_VIEWPORT => "SetViewport",
        slot::SET_CLIP_PLANE => "SetClipPlane",
        slot::SET_RENDER_STATE => "SetRenderState",
        slot::SET_TEXTURE => "SetTexture",
        slot::SET_STAGE_STATE => "SetTextureStageState",
        slot::SET_SAMPLER_STATE => "SetSamplerState",
        slot::SET_SCISSOR => "SetScissorRect",
        slot::SET_DECL => "SetVertexDeclaration",
        slot::SET_FVF => "SetFVF",
        slot::SET_VS => "SetVertexShader",
        slot::VS_CONST_F => "SetVertexShaderConstantF",
        slot::VS_CONST_I => "SetVertexShaderConstantI",
        slot::VS_CONST_B => "SetVertexShaderConstantB",
        slot::SET_STREAM => "SetStreamSource",
        slot::SET_STREAM_FREQ => "SetStreamSourceFreq",
        slot::SET_INDICES => "SetIndices",
        slot::SET_PS => "SetPixelShader",
        slot::PS_CONST_F => "SetPixelShaderConstantF",
        slot::PS_CONST_I => "SetPixelShaderConstantI",
        slot::PS_CONST_B => "SetPixelShaderConstantB",
        _ => return None,
    })
}

/// One piece of device state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// The draw call's own words that are not pointers or counts (a Clear's flags, colour, Z and
    /// stencil; an indexed draw's base vertex, first vertex and first index).
    Args,
    RenderTarget(u32),
    DepthStencil,
    Viewport,
    Scissor,
    ClipPlane(u32),
    RenderState(u32),
    Texture(u32),
    Sampler(u32, u32),
    StageState(u32, u32),
    Transform(u32),
    Decl,
    Fvf,
    VertexShader,
    PixelShader,
    VsConstF(u32),
    VsConstI(u32),
    VsConstB(u32),
    PsConstF(u32),
    PsConstI(u32),
    PsConstB(u32),
    Stream(u32),
    StreamFreq(u32),
    Indices,
}

/// A state's value: a word, an object's name, or bytes (a viewport, a matrix, a constant).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Val {
    Word(u32),
    Name(Rc<str>),
    Bytes(Rc<[u8]>),
    /// A stream: the buffer's name, offset and stride.
    Stream(Rc<str>, u32, u32),
}

/// What a shader reads: constant registers and samplers (from its bytecode).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShaderUse {
    pub floats: BTreeSet<u32>,
    pub ints: BTreeSet<u32>,
    pub bools: BTreeSet<u32>,
    pub samplers: BTreeSet<u32>,
    /// A float constant is read with relative addressing: any set constant may be read.
    pub relative: bool,
}

/// What a shader's bytecode reads, or `None` when it can't be told (shader model 1, or no
/// bytecode). Constants the shader defines itself (`def`, `defi`, `defb`) are not read from
/// the device.
pub fn shader_use(code: &[u8]) -> Option<ShaderUse> {
    let words: Vec<u32> = code
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    let version = *words.first()?;
    if (version >> 8) & 0xff < 2 {
        return None;
    }
    let mut u = ShaderUse::default();
    let (mut defined_f, mut defined_i, mut defined_b) =
        (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
    let mut i = 1;
    while i < words.len() {
        let t = words[i];
        let op = t & 0xffff;
        if t == 0x0000_ffff {
            break;
        }
        if op == 0xfffe {
            i += 1 + ((t >> 16) & 0x7fff) as usize;
            continue;
        }
        let len = ((t >> 24) & 0xf) as usize;
        let params = words.get(i + 1..(i + 1 + len).min(words.len()))?;
        let reg = |p: u32| (((p >> 28) & 7) | ((p >> 8) & 0x18), p & 0x7ff);
        match op {
            // def, defi, defb: the register they define, then literals.
            81..=83 => {
                if let Some(&p) = params.first() {
                    let n = reg(p).1;
                    match op {
                        81 => defined_f.insert(n),
                        82 => defined_i.insert(n),
                        _ => defined_b.insert(n),
                    };
                }
            }
            // dcl: a usage token, then the register.
            31 => {
                if let Some(&p) = params.get(1) {
                    let (ty, n) = reg(p);
                    if ty == 10 {
                        u.samplers.insert(n);
                    }
                }
            }
            _ => {
                for &p in params {
                    let (ty, n) = reg(p);
                    match ty {
                        2 => {
                            u.floats.insert(n);
                            if p & (1 << 13) != 0 {
                                u.relative = true;
                            }
                        }
                        7 => {
                            u.ints.insert(n);
                        }
                        14 => {
                            u.bools.insert(n);
                        }
                        10 => {
                            u.samplers.insert(n);
                        }
                        _ => {}
                    }
                }
            }
        }
        i += 1 + len;
    }
    u.floats.retain(|n| !defined_f.contains(n));
    u.ints.retain(|n| !defined_i.contains(n));
    u.bools.retain(|n| !defined_b.contains(n));
    Some(u)
}

/// One entry of a shader's constant table (the `CTAB` comment the HLSL compiler writes):
/// its name, register set (0 bool, 1 int, 2 float, 3 sampler), first register and count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CtabEntry {
    pub name: String,
    pub set: u16,
    pub index: u16,
    pub count: u16,
}

/// The constant table of shader bytecode, when it has one (D3DXSHADER_CONSTANTTABLE after
/// the `CTAB` four-character code; offsets from that code's end).
pub fn ctab(code: &[u8]) -> Vec<CtabEntry> {
    let word = |b: &[u8], at: usize| -> Option<u32> {
        Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
    };
    let half = |b: &[u8], at: usize| -> Option<u16> {
        Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
    };
    let mut at = 4;
    while let Some(t) = word(code, at) {
        if t == 0x0000_ffff {
            break;
        }
        if t & 0xffff != 0xfffe {
            // The table comes before the instructions.
            break;
        }
        let len = 4 * ((t >> 16) & 0x7fff) as usize;
        let body = code.get(at + 4..at + 4 + len).unwrap_or(&[]);
        if body.get(..4) == Some(b"CTAB") {
            let table = &body[4..];
            let string = |off: u32| -> String {
                let s = table.get(off as usize..).unwrap_or(&[]);
                let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
                String::from_utf8_lossy(&s[..end]).into_owned()
            };
            let (Some(n), Some(info)) = (word(table, 12), word(table, 16)) else {
                return Vec::new();
            };
            return (0..n.min(256) as usize)
                .filter_map(|i| {
                    let e = info as usize + 20 * i;
                    Some(CtabEntry {
                        name: string(word(table, e)?),
                        set: half(table, e + 4)?,
                        index: half(table, e + 6)?,
                        count: half(table, e + 8)?,
                    })
                })
                .collect();
        }
        at += 4 + len;
    }
    Vec::new()
}

/// A draw and the state it used.
#[derive(Clone, Debug)]
pub struct Draw {
    /// The frame in the window (Presents before it) and its index in that frame.
    pub frame: u32,
    pub index: u32,
    pub seq: u32,
    pub slot: u32,
    /// The call's words after `this`.
    pub words: Vec<u32>,
    pub state: BTreeMap<Key, Val>,
}

impl Draw {
    /// What a draw is made of: kind, primitive type and counts (a Clear: its flags).
    pub fn geometry(&self) -> String {
        let w = |i: usize| self.words.get(i).copied().unwrap_or(0);
        match self.slot {
            slot::DRAW_INDEXED => format!("DIP {} v={} p={}", prim(w(0)), w(3), w(5)),
            slot::DRAW_PRIMITIVE => format!("DP {} p={}", prim(w(0)), w(2)),
            slot::DRAW_UP => format!("DPUP {} p={} stride={}", prim(w(0)), w(1), w(3)),
            slot::DRAW_INDEXED_UP => {
                format!("DIPUP {} v={} p={} stride={}", prim(w(0)), w(2), w(3), w(7))
            }
            slot::CLEAR => format!("Clear flags={:#x}", w(2)),
            slot::STRETCH_RECT => "StretchRect".to_string(),
            s => format!("slot {s}"),
        }
    }

    /// What the draw is aligned by first: its geometry and shaders (a Clear: its colour).
    pub fn mesh(&self) -> String {
        let shader = |k: Key| match self.state.get(&k) {
            Some(Val::Name(n)) => n.to_string(),
            _ => "-".into(),
        };
        match self.slot {
            slot::CLEAR => format!(
                "{} color={:08x}",
                self.geometry(),
                self.words.get(3).copied().unwrap_or(0)
            ),
            slot::STRETCH_RECT => self.geometry(),
            _ => format!(
                "{} {} {}",
                self.geometry(),
                shader(Key::VertexShader),
                shader(Key::PixelShader)
            ),
        }
    }

    /// The mesh and the textures the draw samples (file names without their folders), to
    /// recognise it by.
    pub fn label(&self) -> String {
        let textures: Vec<String> = self
            .state
            .iter()
            .filter_map(|(k, v)| match (k, v) {
                (Key::Texture(i), Val::Name(n)) if &**n != "null" => {
                    let short = n.rsplit(['\\', '/']).next().unwrap_or_default();
                    Some(format!("t{i}={short}"))
                }
                _ => None,
            })
            .collect();
        let target = match self.state.get(&Key::RenderTarget(0)) {
            Some(Val::Name(n)) => format!(" @({n})"),
            _ => String::new(),
        };
        if textures.is_empty() {
            format!("{}{target}", self.mesh())
        } else {
            format!("{} [{}]{target}", self.mesh(), textures.join(" "))
        }
    }
}

fn prim(t: u32) -> &'static str {
    match t {
        1 => "points",
        2 => "lines",
        3 => "strip-lines",
        4 => "triangles",
        5 => "strip",
        6 => "fan",
        _ => "?",
    }
}

/// Names objects by their creation; pointers never seen created get the order of first use.
#[derive(Default)]
struct Names {
    by_ptr: HashMap<u32, Rc<str>>,
    counts: HashMap<String, u32>,
    unseen: u32,
    shaders: HashMap<Rc<str>, Option<ShaderUse>>,
}

impl Names {
    fn counted(&mut self, base: String) -> Rc<str> {
        // No order number: the two runs may make them in another order (a different load path),
        // so objects made with the same parameters share a name.
        *self.counts.entry(base.clone()).or_default() += 1;
        base.into()
    }

    fn name(&mut self, ptr: u32) -> Rc<str> {
        if ptr == 0 {
            return "null".into();
        }
        if let Some(n) = self.by_ptr.get(&ptr) {
            return n.clone();
        }
        self.unseen += 1;
        let n: Rc<str> = format!("unseen{}", self.unseen).into();
        self.by_ptr.insert(ptr, n.clone());
        n
    }
}

fn dumped(c: &Call, k: usize) -> Option<&[u8]> {
    c.dumps.get(k)?.bytes.as_deref()
}

fn out_ptr(c: &Call) -> Option<u32> {
    let b = c.rdumps.first()?.bytes.as_deref()?;
    (b.len() >= 4).then(|| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// A NUL-terminated string from a dump, lowercased (Windows paths).
fn c_string(b: &[u8]) -> Rc<str> {
    let end = b.iter().position(|&x| x == 0).unwrap_or(b.len());
    String::from_utf8_lossy(&b[..end]).to_lowercase().into()
}

fn hash_text(payload: &[u8]) -> Option<String> {
    let h = u64::from_le_bytes(payload.get(..8)?.try_into().ok()?);
    let len = u32::from_le_bytes(payload.get(8..12)?.try_into().ok()?);
    Some(format!("{h:016x}-{len}"))
}

/// A vertex declaration's elements up to the end marker, readable: `stream+offset type usage`.
fn decl_text(b: &[u8]) -> String {
    let mut parts = Vec::new();
    for e in b.chunks_exact(8) {
        let stream = u16::from_le_bytes([e[0], e[1]]);
        if stream == 0xff {
            break;
        }
        let offset = u16::from_le_bytes([e[2], e[3]]);
        parts.push(format!("{stream}+{offset} t{} u{}.{}", e[4], e[6], e[7]));
    }
    format!("decl[{}]", parts.join(", "))
}

/// The draws of a trace's window with their state, and what was created.
pub struct Run {
    pub draws: Vec<Draw>,
    pub frames: u32,
    /// Creation calls recorded, by kind.
    pub created: BTreeMap<&'static str, u32>,
    /// Creation calls left out because a recorded D3DX call made them.
    pub nested: u32,
    /// Texture names by content (`tex:HASH-LEN`) and the file each was made from, when
    /// recorded: the output shows the file.
    pub files: BTreeMap<String, String>,
    /// Each shader's constant table, by name (`vs:HASH`), when its bytecode had one.
    pub ctab: BTreeMap<String, Vec<CtabEntry>>,
    /// State calls the device refused (not an OK answer), by method.
    pub refused: BTreeMap<&'static str, u32>,
}

/// Rebuilds the draws of a trace.
pub fn replay(trace: &Trace) -> Run {
    // Every thread's calls in sequence order, with whether a D3DX loader was open around them
    // and the file name of the innermost open texture-from-file call (00e68a80, when listed).
    let mut all: Vec<(Call, bool, Option<Rc<str>>)> = Vec::new();
    for (_, cs) in calls(trace) {
        let mut open: Vec<(u32, Option<Rc<str>>)> = Vec::new();
        for c in cs {
            open.truncate(c.depth as usize);
            let nested = open
                .iter()
                .any(|(a, _)| matches!(*a, D3DX_TEXTURE | D3DX_CUBE | D3DX_VOLUME));
            let file = open.iter().rev().find_map(|(_, f)| f.clone());
            let own = (c.addr == TEXTURE_FROM_FILE)
                .then(|| dumped(&c, 0).map(c_string))
                .flatten();
            open.push((c.addr, own));
            all.push((c, nested, file));
        }
    }
    all.sort_by_key(|(c, _, _)| c.seq);
    let mut names = Names::default();
    let mut state: BTreeMap<Key, Val> = BTreeMap::new();
    let mut run = Run {
        draws: Vec::new(),
        frames: 0,
        created: BTreeMap::new(),
        nested: 0,
        files: BTreeMap::new(),
        ctab: BTreeMap::new(),
        refused: BTreeMap::new(),
    };
    // Frames are counted by the list's per-frame function when it has one (the game presents a
    // frame's draws in the next one), else by Present.
    let frame_fn = crate::list::from_header(&trace.header)
        .ok()
        .and_then(|l| l.into_iter().find(|s| s.conv == crate::list::Conv::Frame))
        .map(|s| s.addr);
    let mut frame = 0u32;
    let mut index = 0u32;
    for (c, nested, file) in &all {
        if Some(c.addr) == frame_fn {
            frame += 1;
            index = 0;
            continue;
        }
        let ok = c.ret.as_ref().is_some_and(|r| r.eax == 0);
        let w = |i: usize| c.words.get(i).copied().unwrap_or(0);
        // D3DX loaders: named by the file (from the enclosing texture-from-file call), else by
        // the hash of its bytes.
        if matches!(c.addr, D3DX_TEXTURE | D3DX_CUBE | D3DX_VOLUME) {
            // Compared by content (the file may be unreadable on one side); shown by file.
            let h = dumped(c, 0)
                .and_then(hash_text)
                .or_else(|| file.as_ref().map(|f| f.to_string()));
            if let (true, Some(p), Some(h)) = (ok, out_ptr(c), h) {
                let kind = match c.addr {
                    D3DX_TEXTURE => "tex",
                    D3DX_CUBE => "cube",
                    _ => "volume",
                };
                let n = format!("{kind}:{h}");
                if let Some(f) = file {
                    run.files.insert(n.clone(), f.to_string());
                }
                names.by_ptr.insert(p, n.into());
                *run.created.entry("D3DX texture").or_default() += 1;
            }
            continue;
        }
        let Some(s) = slot_of(c.addr) else {
            continue;
        };
        // A state call the device refused changes nothing (counted: the port's show up here).
        if let Some(set) = set_name(s) {
            if !ok {
                *run.refused.entry(set).or_default() += 1;
                continue;
            }
        }
        let create = |kind: &'static str, base: String, names: &mut Names, run: &mut Run| {
            if *nested {
                run.nested += 1;
                return;
            }
            if let (true, Some(p)) = (ok, out_ptr(c)) {
                let n = names.counted(base);
                names.by_ptr.insert(p, n);
                *run.created.entry(kind).or_default() += 1;
            }
        };
        let name = |names: &mut Names, p: u32| Val::Name(names.name(p));
        let bytes = |k: usize| Val::Bytes(dumped(c, k).unwrap_or(&[]).into());
        match s {
            slot::CREATE_TEXTURE => create(
                "texture",
                format!(
                    "texture {}x{} levels {} usage {:#x} format {} pool {}",
                    w(1),
                    w(2),
                    w(3),
                    w(4),
                    w(5),
                    w(6)
                ),
                &mut names,
                &mut run,
            ),
            slot::CREATE_VOLUME_TEXTURE => create(
                "volume texture",
                format!(
                    "volume {}x{}x{} levels {} usage {:#x} format {} pool {}",
                    w(1),
                    w(2),
                    w(3),
                    w(4),
                    w(5),
                    w(6),
                    w(7)
                ),
                &mut names,
                &mut run,
            ),
            slot::CREATE_CUBE_TEXTURE => create(
                "cube texture",
                format!(
                    "cube {} levels {} usage {:#x} format {} pool {}",
                    w(1),
                    w(2),
                    w(3),
                    w(4),
                    w(5)
                ),
                &mut names,
                &mut run,
            ),
            slot::CREATE_VERTEX_BUFFER => create(
                "vertex buffer",
                format!(
                    "vb {} usage {:#x} fvf {:#x} pool {}",
                    w(1),
                    w(2),
                    w(3),
                    w(4)
                ),
                &mut names,
                &mut run,
            ),
            slot::CREATE_INDEX_BUFFER => create(
                "index buffer",
                format!(
                    "ib {} usage {:#x} format {} pool {}",
                    w(1),
                    w(2),
                    w(3),
                    w(4)
                ),
                &mut names,
                &mut run,
            ),
            slot::CREATE_RENDER_TARGET | slot::CREATE_DEPTH_STENCIL => create(
                "surface",
                format!(
                    "{} {}x{} format {} ms {}",
                    if s == slot::CREATE_RENDER_TARGET {
                        "rt"
                    } else {
                        "ds"
                    },
                    w(1),
                    w(2),
                    w(3),
                    w(4)
                ),
                &mut names,
                &mut run,
            ),
            slot::CREATE_DECL => {
                if let (true, Some(p), Some(b)) = (ok, out_ptr(c), dumped(c, 0)) {
                    names.by_ptr.insert(p, decl_text(b).into());
                    *run.created.entry("declaration").or_default() += 1;
                }
            }
            slot::CREATE_VS | slot::CREATE_PS => {
                if let (true, Some(p), Some(b)) = (ok, out_ptr(c), dumped(c, 0)) {
                    let kind = if s == slot::CREATE_VS { "vs" } else { "ps" };
                    let h = hash_text(b).unwrap_or_default();
                    let n: Rc<str> = format!("{kind}:{}", &h[..h.len().min(12)]).into();
                    let code = b.get(12..).filter(|c| !c.is_empty());
                    if let Some(code) = code {
                        let table = ctab(code);
                        if !table.is_empty() {
                            run.ctab.insert(n.to_string(), table);
                        }
                    }
                    names
                        .shaders
                        .entry(n.clone())
                        .or_insert_with(|| code.and_then(shader_use));
                    names.by_ptr.insert(p, n);
                    *run.created.entry("shader").or_default() += 1;
                }
            }
            // Surfaces got from the device: named by how they were first got, unless named.
            18 | 38 | 40 => {
                if let (true, Some(p)) = (ok, out_ptr(c)) {
                    let base = match s {
                        18 => format!("back buffer {}", w(2)),
                        38 => format!("render target {} (got)", w(1)),
                        _ => "depth-stencil (got)".to_string(),
                    };
                    if !names.by_ptr.contains_key(&p) {
                        let n = names.counted(base);
                        names.by_ptr.insert(p, n);
                    }
                }
            }
            slot::SET_RENDER_TARGET => {
                let v = name(&mut names, w(2));
                state.insert(Key::RenderTarget(w(1)), v);
            }
            slot::SET_DEPTH_STENCIL => {
                let v = name(&mut names, w(1));
                state.insert(Key::DepthStencil, v);
            }
            slot::SET_TRANSFORM => {
                state.insert(Key::Transform(w(1)), bytes(0));
            }
            slot::SET_VIEWPORT => {
                state.insert(Key::Viewport, bytes(0));
            }
            slot::SET_SCISSOR => {
                state.insert(Key::Scissor, bytes(0));
            }
            slot::SET_CLIP_PLANE => {
                state.insert(Key::ClipPlane(w(1)), bytes(0));
            }
            slot::SET_RENDER_STATE => {
                state.insert(Key::RenderState(w(1)), Val::Word(w(2)));
            }
            slot::SET_TEXTURE => {
                let v = name(&mut names, w(2));
                state.insert(Key::Texture(w(1)), v);
            }
            slot::SET_STAGE_STATE => {
                state.insert(Key::StageState(w(1), w(2)), Val::Word(w(3)));
            }
            slot::SET_SAMPLER_STATE => {
                state.insert(Key::Sampler(w(1), w(2)), Val::Word(w(3)));
            }
            slot::SET_DECL => {
                let v = name(&mut names, w(1));
                state.insert(Key::Decl, v);
            }
            slot::SET_FVF => {
                state.insert(Key::Fvf, Val::Word(w(1)));
            }
            slot::SET_VS => {
                let v = name(&mut names, w(1));
                state.insert(Key::VertexShader, v);
            }
            slot::SET_PS => {
                let v = name(&mut names, w(1));
                state.insert(Key::PixelShader, v);
            }
            slot::VS_CONST_F
            | slot::VS_CONST_I
            | slot::VS_CONST_B
            | slot::PS_CONST_F
            | slot::PS_CONST_I
            | slot::PS_CONST_B => {
                let (start, count) = (w(1), w(3));
                let per = if matches!(s, slot::VS_CONST_B | slot::PS_CONST_B) {
                    4
                } else {
                    16
                };
                let data = dumped(c, 0).unwrap_or(&[]);
                for r in 0..count {
                    let reg = start + r;
                    let key = match s {
                        slot::VS_CONST_F => Key::VsConstF(reg),
                        slot::VS_CONST_I => Key::VsConstI(reg),
                        slot::VS_CONST_B => Key::VsConstB(reg),
                        slot::PS_CONST_F => Key::PsConstF(reg),
                        slot::PS_CONST_I => Key::PsConstI(reg),
                        _ => Key::PsConstB(reg),
                    };
                    let at = (r * per) as usize;
                    let v = data.get(at..at + per as usize).unwrap_or(&[]);
                    state.insert(key, Val::Bytes(v.into()));
                }
            }
            slot::SET_STREAM => {
                let n = names.name(w(2));
                state.insert(Key::Stream(w(1)), Val::Stream(n, w(3), w(4)));
            }
            slot::SET_STREAM_FREQ => {
                state.insert(Key::StreamFreq(w(1)), Val::Word(w(2)));
            }
            slot::SET_INDICES => {
                let v = name(&mut names, w(1));
                state.insert(Key::Indices, v);
            }
            slot::PRESENT if frame_fn.is_none() => {
                frame += 1;
                index = 0;
            }
            slot::DRAW_PRIMITIVE
            | slot::DRAW_INDEXED
            | slot::DRAW_UP
            | slot::DRAW_INDEXED_UP
            | slot::CLEAR
            | slot::STRETCH_RECT => {
                let mut used = used_state(&state, &names, s);
                let args: Vec<u32> = match s {
                    slot::CLEAR => vec![w(3), w(4), w(5), w(6)],
                    slot::DRAW_INDEXED => vec![w(2), w(3), w(5)],
                    slot::DRAW_PRIMITIVE => vec![w(2)],
                    _ => Vec::new(),
                };
                if !args.is_empty() {
                    let b: Vec<u8> = args.iter().flat_map(|x| x.to_le_bytes()).collect();
                    used.insert(Key::Args, Val::Bytes(b.into()));
                }
                run.draws.push(Draw {
                    frame,
                    index,
                    seq: c.seq,
                    slot: s,
                    words: c.words.get(1..).unwrap_or(&[]).to_vec(),
                    state: used,
                });
                index += 1;
            }
            _ => {}
        }
    }
    // Frames from the window's first.
    let first = run.draws.iter().map(|d| d.frame).min().unwrap_or(0);
    for d in &mut run.draws {
        d.frame -= first;
    }
    run.frames = run.draws.iter().map(|d| d.frame + 1).max().unwrap_or(0);
    run
}

/// The part of the state a draw uses.
fn used_state(state: &BTreeMap<Key, Val>, names: &Names, s: u32) -> BTreeMap<Key, Val> {
    let mut out = BTreeMap::new();
    let targets = |k: &Key| {
        matches!(
            k,
            Key::RenderTarget(_) | Key::DepthStencil | Key::Viewport | Key::Scissor
        )
    };
    if matches!(s, slot::CLEAR | slot::STRETCH_RECT) {
        for (k, v) in state {
            if targets(k) {
                out.insert(*k, v.clone());
            }
        }
        return out;
    }
    let shader_use = |k: Key| match state.get(&k) {
        Some(Val::Name(n)) if &**n != "null" => Some(names.shaders.get(n).cloned().flatten()),
        _ => None,
    };
    // None: no shader; Some(None): a shader whose use is unknown.
    let vs = shader_use(Key::VertexShader);
    let ps = shader_use(Key::PixelShader);
    let indexed = matches!(s, slot::DRAW_INDEXED);
    for (k, v) in state {
        let keep = match k {
            // A clip plane counts only while CLIPPLANEENABLE has its bit (or is unknown).
            Key::ClipPlane(i) => match state.get(&Key::RenderState(152)) {
                Some(Val::Word(m)) => m & (1 << (i & 31)) != 0,
                _ => true,
            },
            Key::Args
            | Key::RenderTarget(_)
            | Key::DepthStencil
            | Key::Viewport
            | Key::Scissor
            | Key::RenderState(_)
            | Key::Decl
            | Key::Fvf
            | Key::VertexShader
            | Key::PixelShader
            | Key::Stream(_)
            | Key::StreamFreq(_) => true,
            Key::Indices => indexed,
            Key::Texture(n) | Key::Sampler(n, _) => match &ps {
                Some(Some(u)) => u.samplers.contains(n),
                _ => true,
            },
            Key::StageState(..) => ps.is_none(),
            Key::Transform(_) => vs.is_none(),
            Key::VsConstF(n) => reads(&vs, |u| u.relative || u.floats.contains(n)),
            Key::VsConstI(n) => reads(&vs, |u| u.ints.contains(n)),
            Key::VsConstB(n) => reads(&vs, |u| u.bools.contains(n)),
            Key::PsConstF(n) => reads(&ps, |u| u.relative || u.floats.contains(n)),
            Key::PsConstI(n) => reads(&ps, |u| u.ints.contains(n)),
            Key::PsConstB(n) => reads(&ps, |u| u.bools.contains(n)),
        };
        if keep {
            out.insert(*k, v.clone());
        }
    }
    out
}

fn reads(shader: &Option<Option<ShaderUse>>, f: impl Fn(&ShaderUse) -> bool) -> bool {
    match shader {
        None => false,
        Some(None) => true,
        Some(Some(u)) => f(u),
    }
}

/// `D3DRENDERSTATETYPE` names (d3d9types.h).
pub fn render_state_name(s: u32) -> String {
    let n = match s {
        7 => "ZENABLE",
        8 => "FILLMODE",
        9 => "SHADEMODE",
        14 => "ZWRITEENABLE",
        15 => "ALPHATESTENABLE",
        16 => "LASTPIXEL",
        19 => "SRCBLEND",
        20 => "DESTBLEND",
        22 => "CULLMODE",
        23 => "ZFUNC",
        24 => "ALPHAREF",
        25 => "ALPHAFUNC",
        26 => "DITHERENABLE",
        27 => "ALPHABLENDENABLE",
        28 => "FOGENABLE",
        29 => "SPECULARENABLE",
        34 => "FOGCOLOR",
        35 => "FOGTABLEMODE",
        36 => "FOGSTART",
        37 => "FOGEND",
        38 => "FOGDENSITY",
        48 => "RANGEFOGENABLE",
        52 => "STENCILENABLE",
        53 => "STENCILFAIL",
        54 => "STENCILZFAIL",
        55 => "STENCILPASS",
        56 => "STENCILFUNC",
        57 => "STENCILREF",
        58 => "STENCILMASK",
        59 => "STENCILWRITEMASK",
        60 => "TEXTUREFACTOR",
        128..=135 => return format!("WRAP{}", s - 128),
        136 => "CLIPPING",
        137 => "LIGHTING",
        139 => "AMBIENT",
        140 => "FOGVERTEXMODE",
        141 => "COLORVERTEX",
        142 => "LOCALVIEWER",
        143 => "NORMALIZENORMALS",
        145 => "DIFFUSEMATERIALSOURCE",
        146 => "SPECULARMATERIALSOURCE",
        147 => "AMBIENTMATERIALSOURCE",
        148 => "EMISSIVEMATERIALSOURCE",
        151 => "VERTEXBLEND",
        152 => "CLIPPLANEENABLE",
        154 => "POINTSIZE",
        155 => "POINTSIZE_MIN",
        156 => "POINTSPRITEENABLE",
        157 => "POINTSCALEENABLE",
        158 => "POINTSCALE_A",
        159 => "POINTSCALE_B",
        160 => "POINTSCALE_C",
        161 => "MULTISAMPLEANTIALIAS",
        162 => "MULTISAMPLEMASK",
        163 => "PATCHEDGESTYLE",
        165 => "DEBUGMONITORTOKEN",
        166 => "POINTSIZE_MAX",
        167 => "INDEXEDVERTEXBLENDENABLE",
        168 => "COLORWRITEENABLE",
        170 => "TWEENFACTOR",
        171 => "BLENDOP",
        172 => "POSITIONDEGREE",
        173 => "NORMALDEGREE",
        174 => "SCISSORTESTENABLE",
        175 => "SLOPESCALEDEPTHBIAS",
        176 => "ANTIALIASEDLINEENABLE",
        178 => "MINTESSELLATIONLEVEL",
        179 => "MAXTESSELLATIONLEVEL",
        180 => "ADAPTIVETESS_X",
        181 => "ADAPTIVETESS_Y",
        182 => "ADAPTIVETESS_Z",
        183 => "ADAPTIVETESS_W",
        184 => "ENABLEADAPTIVETESSELLATION",
        185 => "TWOSIDEDSTENCILMODE",
        186 => "CCW_STENCILFAIL",
        187 => "CCW_STENCILZFAIL",
        188 => "CCW_STENCILPASS",
        189 => "CCW_STENCILFUNC",
        190 => "COLORWRITEENABLE1",
        191 => "COLORWRITEENABLE2",
        192 => "COLORWRITEENABLE3",
        193 => "BLENDFACTOR",
        194 => "SRGBWRITEENABLE",
        195 => "DEPTHBIAS",
        198..=205 => return format!("WRAP{}", s - 190),
        206 => "SEPARATEALPHABLENDENABLE",
        207 => "SRCBLENDALPHA",
        208 => "DESTBLENDALPHA",
        209 => "BLENDOPALPHA",
        _ => return format!("RS{s}"),
    };
    n.to_string()
}

/// `D3DSAMPLERSTATETYPE` names.
fn sampler_state_name(s: u32) -> String {
    match s {
        1 => "ADDRESSU",
        2 => "ADDRESSV",
        3 => "ADDRESSW",
        4 => "BORDERCOLOR",
        5 => "MAGFILTER",
        6 => "MINFILTER",
        7 => "MIPFILTER",
        8 => "MIPMAPLODBIAS",
        9 => "MAXMIPLEVEL",
        10 => "MAXANISOTROPY",
        11 => "SRGBTEXTURE",
        12 => "ELEMENTINDEX",
        13 => "DMAPOFFSET",
        _ => return format!("SS{s}"),
    }
    .to_string()
}

/// `D3DTEXTURESTAGESTATETYPE` names.
fn stage_state_name(s: u32) -> String {
    match s {
        1 => "COLOROP",
        2 => "COLORARG1",
        3 => "COLORARG2",
        4 => "ALPHAOP",
        5 => "ALPHAARG1",
        6 => "ALPHAARG2",
        7 => "BUMPENVMAT00",
        8 => "BUMPENVMAT01",
        9 => "BUMPENVMAT10",
        10 => "BUMPENVMAT11",
        11 => "TEXCOORDINDEX",
        22 => "BUMPENVLSCALE",
        23 => "BUMPENVLOFFSET",
        24 => "TEXTURETRANSFORMFLAGS",
        26 => "COLORARG0",
        27 => "ALPHAARG0",
        28 => "RESULTARG",
        32 => "CONSTANT",
        _ => return format!("TSS{s}"),
    }
    .to_string()
}

pub fn key_text(k: &Key) -> String {
    match *k {
        Key::Args => "the call's words".into(),
        Key::RenderTarget(i) => format!("render target {i}"),
        Key::DepthStencil => "depth-stencil".into(),
        Key::Viewport => "viewport".into(),
        Key::Scissor => "scissor rect".into(),
        Key::ClipPlane(i) => format!("clip plane {i}"),
        Key::RenderState(s) => format!("render state {}", render_state_name(s)),
        Key::Texture(i) => format!("texture {i}"),
        Key::Sampler(i, s) => format!("sampler {i} {}", sampler_state_name(s)),
        Key::StageState(i, s) => format!("stage {i} {}", stage_state_name(s)),
        Key::Transform(t) => format!("transform {t}"),
        Key::Decl => "vertex declaration".into(),
        Key::Fvf => "FVF".into(),
        Key::VertexShader => "vertex shader".into(),
        Key::PixelShader => "pixel shader".into(),
        Key::VsConstF(r) => format!("vs c{r}"),
        Key::VsConstI(r) => format!("vs i{r}"),
        Key::VsConstB(r) => format!("vs b{r}"),
        Key::PsConstF(r) => format!("ps c{r}"),
        Key::PsConstI(r) => format!("ps i{r}"),
        Key::PsConstB(r) => format!("ps b{r}"),
        Key::Stream(i) => format!("stream {i}"),
        Key::StreamFreq(i) => format!("stream {i} frequency"),
        Key::Indices => "indices".into(),
    }
}

/// A value as text: floats for constants, matrices, planes and the viewport's depth range.
pub fn val_text(k: &Key, v: Option<&Val>) -> String {
    let Some(v) = v else {
        return "unknown (not set since the recording started)".into();
    };
    let floats = |b: &[u8]| {
        b.chunks_exact(4)
            .map(|c| format!("{}", f32::from_le_bytes([c[0], c[1], c[2], c[3]])))
            .collect::<Vec<_>>()
            .join(" ")
    };
    match v {
        Val::Word(w) => match k {
            Key::RenderState(36..=38) | Key::RenderState(154..=160) | Key::RenderState(166) => {
                format!("{w:#x} ({})", f32::from_bits(*w))
            }
            _ => format!("{w:#x}"),
        },
        Val::Name(n) => n.to_string(),
        Val::Stream(n, off, stride) => format!("{n} offset {off} stride {stride}"),
        Val::Bytes(b) if b.is_empty() => "(unreadable)".into(),
        Val::Bytes(b) => match k {
            Key::VsConstF(_) | Key::PsConstF(_) | Key::Transform(_) | Key::ClipPlane(_) => {
                floats(b)
            }
            Key::Args => b
                .chunks_exact(4)
                .map(|c| format!("{:08x}", u32::from_le_bytes([c[0], c[1], c[2], c[3]])))
                .collect::<Vec<_>>()
                .join(" "),
            // D3DVIEWPORT9: X, Y, Width, Height (words), MinZ, MaxZ (floats); a scissor RECT.
            Key::Viewport | Key::Scissor if b.len() >= 16 => {
                let words: Vec<String> = b[..16]
                    .chunks_exact(4)
                    .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]).to_string())
                    .collect();
                format!("{} {}", words.join(" "), floats(&b[16..]))
                    .trim_end()
                    .to_string()
            }
            _ => b.iter().map(|x| format!("{x:02x}")).collect(),
        },
    }
}

/// Longest common subsequence of two key lists: the aligned index pairs.
fn align(a: &[String], b: &[String]) -> Vec<(usize, usize)> {
    let (n, m) = (a.len(), b.len());
    if n == 0 || m == 0 {
        return Vec::new();
    }
    if n.saturating_mul(m) > 60_000_000 {
        // Too big for the table: pair in order while the keys agree.
        return a
            .iter()
            .zip(b)
            .enumerate()
            .take_while(|(_, (x, y))| x == y)
            .map(|(i, _)| (i, i))
            .collect();
    }
    let w = m + 1;
    let mut t = vec![0u16; (n + 1) * w];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            t[i * w + j] = if a[i] == b[j] {
                t[(i + 1) * w + j + 1].saturating_add(1)
            } else {
                t[(i + 1) * w + j].max(t[i * w + j + 1])
            };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n && j < m {
        if a[i] == b[j] {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if t[(i + 1) * w + j] >= t[i * w + j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// How two values of one state differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    /// Both set, different.
    Value,
    /// Set in the window on one side only: the other set it before the recording started.
    OneSide,
    /// Floats that differ by rounding only (relative 1e-5).
    Rounding,
}

fn floaty(k: &Key) -> bool {
    matches!(
        k,
        Key::VsConstF(_) | Key::PsConstF(_) | Key::Transform(_) | Key::ClipPlane(_)
    )
}

fn close(x: &[u8], y: &[u8]) -> bool {
    x.len() == y.len()
        && x.chunks_exact(4).zip(y.chunks_exact(4)).all(|(a, b)| {
            let a = f32::from_le_bytes([a[0], a[1], a[2], a[3]]);
            let b = f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
            (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 * 1f32.max(a.abs()).max(b.abs())
        })
}

fn kind_of(k: &Key, x: Option<&Val>, y: Option<&Val>) -> Kind {
    match (x, y) {
        (None, _) | (_, None) => Kind::OneSide,
        (Some(Val::Bytes(a)), Some(Val::Bytes(b))) if floaty(k) && close(a, b) => Kind::Rounding,
        _ => Kind::Value,
    }
}

/// A draw a difference affects: frame, its index on each side, its label.
type Affected = (u32, u32, u32, String);

/// One state's differences: by value pair, the draws.
#[derive(Default)]
struct StateDiff {
    draws: usize,
    /// By (what the shader calls the register, real value, port value).
    values: BTreeMap<(String, String, String), Vec<Affected>>,
}

/// Aligns a frame's draws: first by mesh with shaders, then, between those anchors, by
/// geometry alone (kind, primitive type, counts), so a draw made with other shaders on one
/// side still pairs with its counterpart and the shaders show as a difference.
fn align_draws(da: &[&Draw], db: &[&Draw]) -> Vec<(usize, usize, bool)> {
    let ka: Vec<String> = da.iter().map(|d| d.mesh()).collect();
    let kb: Vec<String> = db.iter().map(|d| d.mesh()).collect();
    let exact = align(&ka, &kb);
    let mut out = Vec::new();
    let (mut ia, mut ib) = (0, 0);
    let end = (da.len(), db.len());
    for &(i, j) in exact.iter().chain(std::iter::once(&end)) {
        let ga: Vec<String> = da[ia..i].iter().map(|d| d.geometry()).collect();
        let gb: Vec<String> = db[ib..j].iter().map(|d| d.geometry()).collect();
        for (x, y) in align(&ga, &gb) {
            out.push((ia + x, ib + y, false));
        }
        if i < da.len() && j < db.len() {
            out.push((i, j, true));
        }
        (ia, ib) = (i + 1, j + 1);
    }
    out
}

/// What the draws' shaders call a constant register or sampler (their constant tables):
/// `[LightData+1] `, or both sides' names when they differ; empty when unknown.
fn annotate(a: &Run, b: &Run, x: &Draw, y: &Draw, k: &Key) -> String {
    let (shader, set, reg) = match *k {
        Key::VsConstF(r) => (Key::VertexShader, 2, r),
        Key::VsConstI(r) => (Key::VertexShader, 1, r),
        Key::VsConstB(r) => (Key::VertexShader, 0, r),
        Key::PsConstF(r) => (Key::PixelShader, 2, r),
        Key::PsConstI(r) => (Key::PixelShader, 1, r),
        Key::PsConstB(r) => (Key::PixelShader, 0, r),
        Key::Texture(i) | Key::Sampler(i, _) => (Key::PixelShader, 3, i),
        _ => return String::new(),
    };
    let names: Vec<String> = [(a, x), (b, y)]
        .iter()
        .filter_map(|(run, d)| {
            let Some(Val::Name(n)) = d.state.get(&shader) else {
                return None;
            };
            let e = run.ctab.get(&**n)?.iter().find(|e| {
                let first = u32::from(e.index);
                e.set == set && reg >= first && reg < first + u32::from(e.count.max(1))
            })?;
            let off = reg - u32::from(e.index);
            Some(if e.count > 1 {
                format!("{}+{off}", e.name)
            } else {
                e.name.clone()
            })
        })
        .collect();
    match names.as_slice() {
        [one] => format!("[{one}] "),
        [p, q] if p == q => format!("[{p}] "),
        [p, q] => format!("[{p} / {q}] "),
        _ => String::new(),
    }
}

/// The shaders the draws use with the names in their constant tables.
fn legend(a: &Run, b: &Run) -> String {
    let mut used: BTreeSet<String> = BTreeSet::new();
    for d in a.draws.iter().chain(&b.draws) {
        for k in [Key::VertexShader, Key::PixelShader] {
            if let Some(Val::Name(n)) = d.state.get(&k) {
                used.insert(n.to_string());
            }
        }
    }
    let mut out = String::new();
    for n in used {
        let table = a.ctab.get(&n).or_else(|| b.ctab.get(&n));
        let names: Vec<&str> = table
            .map(|t| t.iter().map(|e| e.name.as_str()).collect())
            .unwrap_or_default();
        let _ = writeln!(out, "  {n}: {}", names.join(", "));
    }
    out
}

/// The draw-by-draw diff of two runs (`a` the real game, `b` the port).
pub fn diff(a: &Run, b: &Run, names: (&str, &str), show: usize) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{}: {} frames, {} draws; {}: {} frames, {} draws",
        names.0,
        a.frames,
        a.draws.len(),
        names.1,
        b.frames,
        b.draws.len()
    );
    for (label, r) in [(names.0, a), (names.1, b)] {
        let created: Vec<String> = r.created.iter().map(|(k, n)| format!("{k} {n}")).collect();
        let _ = writeln!(
            out,
            "{label} created: {}; left out (made inside D3DX): {}",
            created.join(", "),
            r.nested
        );
        if !r.refused.is_empty() {
            let refused: Vec<String> = r.refused.iter().map(|(k, n)| format!("{k} {n}")).collect();
            let _ = writeln!(out, "{label} state calls refused: {}", refused.join(", "));
        }
    }
    let frames = a.frames.max(b.frames);
    let mut diffs: BTreeMap<(Kind, Key), StateDiff> = BTreeMap::new();
    let mut only: Vec<String> = Vec::new();
    let (mut paired, mut by_geometry, mut equal) = (0usize, 0usize, 0usize);
    for f in 0..frames {
        let da: Vec<&Draw> = a.draws.iter().filter(|d| d.frame == f).collect();
        let db: Vec<&Draw> = b.draws.iter().filter(|d| d.frame == f).collect();
        if da.is_empty() && db.is_empty() {
            continue;
        }
        let pairs = align_draws(&da, &db);
        let _ = writeln!(
            out,
            "frame {f}: {} {} draws, {} {} draws, {} aligned ({} by geometry only)",
            names.0,
            da.len(),
            names.1,
            db.len(),
            pairs.len(),
            pairs.iter().filter(|p| !p.2).count()
        );
        let (mut ia, mut ib) = (0, 0);
        let end = (da.len(), db.len(), true);
        for &(i, j, _) in pairs.iter().chain(std::iter::once(&end)) {
            for d in &da[ia..i] {
                only.push(format!(
                    "{} only: frame {f} draw {} {}",
                    names.0,
                    d.index,
                    d.label()
                ));
            }
            for d in &db[ib..j] {
                only.push(format!(
                    "{} only: frame {f} draw {} {}",
                    names.1,
                    d.index,
                    d.label()
                ));
            }
            (ia, ib) = (i + 1, j + 1);
        }
        for &(i, j, exact) in &pairs {
            let (x, y) = (da[i], db[j]);
            paired += 1;
            if !exact {
                by_geometry += 1;
            }
            // Constants mean something else under another shader: compared only when the shader
            // is the same (the shaders themselves are compared).
            let same_vs = x.state.get(&Key::VertexShader) == y.state.get(&Key::VertexShader);
            let same_ps = x.state.get(&Key::PixelShader) == y.state.get(&Key::PixelShader);
            let keys: BTreeSet<&Key> = x
                .state
                .keys()
                .chain(y.state.keys())
                .filter(|k| match k {
                    Key::VsConstF(_) | Key::VsConstI(_) | Key::VsConstB(_) => same_vs,
                    Key::PsConstF(_) | Key::PsConstI(_) | Key::PsConstB(_) => same_ps,
                    _ => true,
                })
                .collect();
            let mut same = true;
            for k in keys {
                let (vx, vy) = (x.state.get(k), y.state.get(k));
                if vx == vy {
                    continue;
                }
                let kind = kind_of(k, vx, vy);
                if kind == Kind::Value {
                    same = false;
                }
                let d = diffs.entry((kind, *k)).or_default();
                d.draws += 1;
                d.values
                    .entry((annotate(a, b, x, y, k), val_text(k, vx), val_text(k, vy)))
                    .or_default()
                    .push((f, x.index, y.index, x.label()));
            }
            if same {
                equal += 1;
            }
        }
    }
    let _ = writeln!(
        out,
        "aligned draws: {paired} ({by_geometry} by geometry only), with no value difference: \
         {equal}; draws only one side made: {}",
        only.len()
    );
    let sections = [
        (
            Kind::Value,
            "states that differ, by draws affected (each value pair with its first draws)",
        ),
        (
            Kind::OneSide,
            "states set in the window on one side only (the other set them before the recording \
             started, so their values there are unknown)",
        ),
        (
            Kind::Rounding,
            "float states that differ by rounding only (relative 1e-5)",
        ),
    ];
    for (kind, title) in sections {
        let mut list: Vec<(&Key, &StateDiff)> = diffs
            .iter()
            .filter(|((k, _), _)| *k == kind)
            .map(|((_, key), d)| (key, d))
            .collect();
        list.sort_by(|x, y| y.1.draws.cmp(&x.1.draws).then(x.0.cmp(y.0)));
        let _ = writeln!(out, "\n{title}: {}", list.len());
        for (k, d) in &list {
            let _ = writeln!(
                out,
                "- {}: {} draws, {} value pairs",
                key_text(k),
                d.draws,
                d.values.len()
            );
            let full = kind == Kind::Value;
            let mut pairs: Vec<_> = d.values.iter().collect();
            pairs.sort_by_key(|p| std::cmp::Reverse(p.1.len()));
            for ((note, va, vb), draws) in pairs.iter().take(if full { show } else { 1 }) {
                let _ = writeln!(
                    out,
                    "    {note}{} {va} | {} {vb} ({} draws)",
                    names.0,
                    names.1,
                    draws.len()
                );
                if full {
                    for (f, i, j, label) in draws.iter().take(2) {
                        let _ = writeln!(out, "        frame {f} draw {i}/{j}: {label}");
                    }
                }
            }
        }
    }
    let _ = writeln!(out, "\ndraws only one side made: {}", only.len());
    for l in only.iter().take(show * 40) {
        let _ = writeln!(out, "  {l}");
    }
    let _ = writeln!(out, "\nshaders (names from their constant tables):");
    out.push_str(&legend(a, b));
    let mut files = a.files.clone();
    files.extend(b.files.iter().map(|(k, v)| (k.clone(), v.clone())));
    show_files(out, &files)
}

/// One run's draws with their state, for reading alone.
pub fn describe(run: &Run, limit: usize, files: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let created: Vec<String> = run
        .created
        .iter()
        .map(|(k, n)| format!("{k} {n}"))
        .collect();
    let _ = writeln!(
        out,
        "{} frames, {} draws; created: {}; left out (made inside D3DX): {}",
        run.frames,
        run.draws.len(),
        created.join(", "),
        run.nested
    );
    for d in run.draws.iter().take(limit) {
        let _ = writeln!(
            out,
            "frame {} draw {} (seq {}): {}",
            d.frame,
            d.index,
            d.seq,
            d.label()
        );
        for (k, v) in &d.state {
            let _ = writeln!(out, "    {} = {}", key_text(k), val_text(k, Some(v)));
        }
    }
    let mut all = run.files.clone();
    all.extend(files.iter().map(|(k, v)| (k.clone(), v.clone())));
    show_files(out, &all)
}

/// Texture names by content (`tex:HASH-LEN`) shown as the file they were made from, when either
/// run recorded it.
fn show_files(mut text: String, files: &BTreeMap<String, String>) -> String {
    for (name, file) in files {
        if text.contains(name.as_str()) {
            text = text.replace(name.as_str(), file);
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    /// TEST DATA made from the documented token layout: ps_2_0, `def c1`, `dcl_2d s0`,
    /// `texld r0, t0, s0`, `mul r0, r0, c0`, `mad r0, r0, c1, c2`, end.
    #[test]
    fn a_shader_reads_its_constants_and_samplers() {
        let p = |ty: u32, n: u32| 0x8000_0000 | ((ty & 7) << 28) | ((ty & 0x18) << 8) | n;
        let ps = code(&[
            0xffff_0200,
            81 | (5 << 24),
            p(2, 1),
            0,
            0,
            0,
            0,
            31 | (2 << 24),
            0x9000_0000,
            p(10, 0),
            66 | (3 << 24),
            p(0, 0),
            p(3, 0),
            p(10, 0),
            5 | (3 << 24),
            p(0, 0),
            p(0, 0),
            p(2, 0),
            4 | (4 << 24),
            p(0, 0),
            p(0, 0),
            p(2, 1),
            p(2, 2),
            0x0000_ffff,
        ]);
        let u = shader_use(&ps).unwrap();
        assert_eq!(u.floats.iter().copied().collect::<Vec<_>>(), [0, 2]);
        assert_eq!(u.samplers.iter().copied().collect::<Vec<_>>(), [0]);
        assert!(!u.relative);
        // Shader model 1 is not read.
        assert!(shader_use(&code(&[0xfffe_0101, 0x0000_ffff])).is_none());
    }

    #[test]
    fn draws_align_by_their_mesh() {
        let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let pairs = align(&s(&["a", "b", "c", "d"]), &s(&["a", "c", "x", "d"]));
        assert_eq!(pairs, [(0, 0), (2, 1), (3, 3)]);
    }

    /// TEST DATA from the documented D3DXSHADER_CONSTANTTABLE layout: one float constant
    /// `Bones` at c44, 51 registers, in a comment before the end token.
    #[test]
    fn constant_tables_name_registers() {
        let mut table = Vec::new();
        // Size, Creator, Version, Constants, ConstantInfo (at 28), Flags, Target.
        for w in [28u32, 0, 0xfffe_0200, 1, 28, 0, 0] {
            table.extend_from_slice(&w.to_le_bytes());
        }
        // The entry: Name (at 48), RegisterSet 2, RegisterIndex 44, RegisterCount 51.
        table.extend_from_slice(&48u32.to_le_bytes());
        for h in [2u16, 44, 51, 0] {
            table.extend_from_slice(&h.to_le_bytes());
        }
        table.extend_from_slice(&[0; 8]);
        table.extend_from_slice(b"Bones\0\0\0");
        let mut body = b"CTAB".to_vec();
        body.extend_from_slice(&table);
        let dwords = (body.len() / 4) as u32;
        let mut code = code(&[0xfffe_0200, 0xfffe | (dwords << 16)]);
        code.extend_from_slice(&body);
        code.extend_from_slice(&0x0000_ffffu32.to_le_bytes());
        let t = ctab(&code);
        assert_eq!(
            t,
            [CtabEntry {
                name: "Bones".into(),
                set: 2,
                index: 44,
                count: 51
            }]
        );
    }

    #[test]
    fn declarations_read_to_their_end() {
        let mut b = Vec::new();
        b.extend_from_slice(&[0, 0, 0, 0, 2, 0, 0, 0]);
        b.extend_from_slice(&[0, 0, 12, 0, 2, 0, 3, 0]);
        b.extend_from_slice(&[0xff, 0, 0, 0, 17, 0, 0, 0]);
        b.extend_from_slice(&[9; 8]);
        assert_eq!(decl_text(&b), "decl[0+0 t2 u0.0, 0+12 t2 u3.0]");
    }

    /// A small run: a texture from a file (made through the device inside D3DX on one side
    /// only), a pixel shader, states, one draw per frame. The texture is named by its file on
    /// both sides although its address differs; a different render state is reported with the
    /// draw it affects.
    #[test]
    fn runs_replay_and_differ_by_state() {
        use crate::dump::Dumped;
        use crate::format::Record;
        let dev = DEVICE_CODE;
        let ps = code(&[
            0xffff_0200,
            31 | (2 << 24),
            0x9000_0000,
            0xa000_0800,
            0x0000_ffff,
        ]);
        let trace = |tex: u32, alpha: u32, nested: bool| {
            let mut recs = Vec::new();
            let mut seq = 0;
            let mut call =
                |recs: &mut Vec<Record>, addr: u32, words: Vec<u32>, dumps: Vec<Dumped>| {
                    seq += 1;
                    recs.push(Record::Enter { addr, seq, words });
                    if !dumps.is_empty() {
                        recs.push(Record::Dump { seq, dumps });
                    }
                    seq
                };
            let leave = |recs: &mut Vec<Record>, addr: u32, s: u32, out: Option<u32>| {
                recs.push(Record::Leave {
                    addr,
                    seq: s,
                    eax: 0,
                    edx: 0,
                    st0: None,
                });
                if let Some(p) = out {
                    recs.push(Record::RDump {
                        seq: s,
                        dumps: vec![Dumped {
                            addr: 0x100,
                            bytes: Some(p.to_le_bytes().to_vec()),
                        }],
                    });
                }
            };
            let file = Dumped {
                addr: 0x200,
                bytes: Some(crate::dump::hash_payload(b"a dds file")),
            };
            let x = call(
                &mut recs,
                D3DX_TEXTURE,
                vec![1, 0x200, 10, 0x100],
                vec![file],
            );
            if nested {
                let ct = dev + 4 * slot::CREATE_TEXTURE;
                let y = call(&mut recs, ct, vec![1, 4, 4, 1, 0, 21, 1, 0x100, 0], vec![]);
                leave(&mut recs, ct, y, Some(tex));
            }
            leave(&mut recs, D3DX_TEXTURE, x, Some(tex));
            let cps = dev + 4 * slot::CREATE_PS;
            let code_dump = Dumped {
                addr: 0x300,
                bytes: Some(crate::dump::shader_payload(&ps)),
            };
            let y = call(&mut recs, cps, vec![1, 0x300, 0x100], vec![code_dump]);
            leave(&mut recs, cps, y, Some(0x7000 + tex));
            let simple: Vec<(u32, Vec<u32>)> = vec![
                (slot::SET_PS, vec![1, 0x7000 + tex]),
                (slot::SET_TEXTURE, vec![1, 0, tex]),
                (slot::SET_TEXTURE, vec![1, 3, 0]),
                (slot::SET_RENDER_STATE, vec![1, 27, alpha]),
                (slot::DRAW_INDEXED, vec![1, 4, 0, 0, 30, 0, 10]),
                (slot::PRESENT, vec![1, 0, 0, 0, 0]),
                (slot::DRAW_INDEXED, vec![1, 4, 0, 0, 30, 0, 10]),
            ];
            for (s, words) in simple {
                let addr = dev + 4 * s;
                let q = call(&mut recs, addr, words, vec![]);
                leave(&mut recs, addr, q, None);
            }
            let mut threads = BTreeMap::new();
            threads.insert(1, recs);
            Trace {
                header: String::new(),
                threads,
                torn: BTreeMap::new(),
                starts: BTreeMap::new(),
            }
        };
        let real = replay(&trace(0x5000, 1, true));
        let port = replay(&trace(0x9000, 0, false));
        assert_eq!((real.nested, port.nested), (1, 0));
        assert_eq!((real.draws.len(), real.frames), (2, 2));
        let d = &real.draws[0];
        let tex = d.state.get(&Key::Texture(0)).unwrap();
        assert_eq!(tex, port.draws[0].state.get(&Key::Texture(0)).unwrap());
        assert!(matches!(tex, Val::Name(n) if n.starts_with("tex:")));
        // Sampler 3 is not read by the shader (it declares s0 only).
        assert!(!d.state.contains_key(&Key::Texture(3)));
        let text = diff(&real, &port, ("real", "port"), 4);
        assert!(
            text.contains("aligned draws: 2 (0 by geometry only), with no value difference: 0"),
            "{text}"
        );
        assert!(
            text.contains(
                "- render state ALPHABLENDENABLE: 2 draws, 1 value pairs\n    real 0x1 | port 0x0"
            ),
            "{text}"
        );
    }
}
