//! Inputs for one function (edge cases plus seeded random values), the
//! `nv-call` vector line for them, and reading the result line back.

use crate::json::Value;

use crate::source::Ty;

/// One argument value as raw bits (`f32`/`u32`-like: low 32 bits; `f64`/`u64`: 64).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Val {
    pub ty: Ty,
    pub bits: u64,
}

impl Val {
    /// The argument words of the engine's uniform form (`abi.rs`).
    pub fn put(self, out: &mut Vec<u32>) {
        match self.ty {
            Ty::F64 | Ty::U64 => {
                out.push(self.bits as u32);
                out.push((self.bits >> 32) as u32);
            }
            Ty::I16 => out.push(self.bits as u16 as i16 as i32 as u32),
            Ty::I8 => out.push(self.bits as u8 as i8 as i32 as u32),
            Ty::U16 => out.push(self.bits as u16 as u32),
            Ty::U8 | Ty::Bool => out.push(self.bits as u8 as u32),
            _ => out.push(self.bits as u32),
        }
    }

    /// The `kind:value` string of an `nv-call` argument.
    pub fn nv_call(self) -> String {
        match self.ty {
            Ty::F32 => format!("f32:0x{:08x}", self.bits as u32),
            Ty::F64 => format!("f64:0x{:016x}", self.bits),
            Ty::U64 => format!("u64:0x{:016x}", self.bits),
            Ty::I32 => format!("i32:{}", self.bits as u32 as i32),
            Ty::I16 => format!("i16:{}", self.bits as u16 as i16),
            Ty::I8 => format!("i8:{}", self.bits as u8 as i8),
            Ty::U16 => format!("u16:{}", self.bits as u16),
            Ty::U8 => format!("u8:{}", self.bits as u8),
            Ty::Bool => format!("u32:{}", self.bits as u8 & 1),
            _ => format!("u32:0x{:08x}", self.bits as u32),
        }
    }

    /// Human-readable form for reports.
    pub fn show(self) -> String {
        match self.ty {
            Ty::F32 => format!(
                "{:?}(0x{:08x})",
                f32::from_bits(self.bits as u32),
                self.bits as u32
            ),
            Ty::F64 => format!("{:?}(0x{:016x})", f64::from_bits(self.bits), self.bits),
            _ => self.nv_call(),
        }
    }
}

/// xorshift64*: deterministic per function (seeded by its address).
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng((seed ^ 0x9e37_79b9_7f4a_7c15) | 1)
    }
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
}

const F32_EDGES: &[u32] = &[
    0x0000_0000, // 0
    0x8000_0000, // -0
    0x3f80_0000, // 1
    0xbf80_0000, // -1
    0x3f00_0000, // 0.5
    0x4000_0000, // 2
    0x3a83_126f, // 0.001
    0x447a_0000, // 1000
    0x4049_0fdb, // pi
    0xbfc9_0fdb, // -pi/2
    0x43b4_0000, // 360
    0x3dcc_cccd, // 0.1
    0x7149_f2ca, // 1e30
    0xf149_f2ca, // -1e30
    0x7f7f_ffff, // FLT_MAX
    0x0080_0000, // smallest normal
    0x0000_0001, // smallest denormal
    0x007f_ffff, // largest denormal
    0x7f80_0000, // +inf
    0xff80_0000, // -inf
    0x7fc0_0000, // qNaN
    0xffc0_0000, // -qNaN
    0x7fc1_2345, // qNaN with payload
    0x7f80_0001, // sNaN
];

const F64_EDGES: &[u64] = &[
    0x0000_0000_0000_0000,
    0x8000_0000_0000_0000,
    0x3ff0_0000_0000_0000,
    0xbff0_0000_0000_0000,
    0x3fe0_0000_0000_0000,
    0x4009_21fb_5444_2d18, // pi
    0x7fef_ffff_ffff_ffff, // DBL_MAX
    0x0000_0000_0000_0001, // denormal
    0x7ff0_0000_0000_0000, // +inf
    0xfff0_0000_0000_0000, // -inf
    0x7ff8_0000_0000_0000, // qNaN
    0x7ff0_0000_0000_0001, // sNaN
    0x4197_d784_0000_0000, // 1e8
    0xc197_d784_0000_0000, // -1e8
];

const I32_EDGES: &[u32] = &[
    0,
    1,
    0xffff_ffff,
    2,
    3,
    7,
    0x7fff_ffff,
    0x8000_0000,
    0xff,
    0x100,
    0xffff,
    0x1_0000,
    0xffff_ff00,
    0x0040_0000,
];

/// Rounding ties and float-to-integer conversion limits: x87 rounding
/// (FISTP, round half to even), truncation (`_ftol2`) and range checks
/// differ only here, and random values almost never land on them.
const ROUNDING: &[f64] = &[
    0.5,
    -0.5,
    1.5,
    -1.5,
    2.5,
    -2.5,
    255.5,
    65535.5,
    0.499_999_970_197_677_6, // largest float below 0.5
    0.500_000_059_604_644_8, // smallest float above 0.5
    -0.499_999_970_197_677_6,
    4_194_304.5,
    8_388_607.5,
    8_388_608.0,      // 2^23
    16_777_216.0,     // 2^24
    2_147_483_520.0,  // largest float below 2^31
    2_147_483_648.0,  // 2^31
    -2_147_483_648.0, // -2^31
    -2_147_483_904.0, // next float below -2^31
    2_147_483_647.0,
    4_294_967_296.0, // 2^32
    1e10,
    -1e10,
];

fn edges(ty: Ty) -> Vec<u64> {
    let mut v: Vec<u64> = match ty {
        Ty::F32 => F32_EDGES.iter().map(|&b| b as u64).collect(),
        Ty::F64 => F64_EDGES.to_vec(),
        _ => Vec::new(),
    };
    let extra: Vec<u64> = match ty {
        Ty::F32 => ROUNDING
            .iter()
            .map(|&x| u64::from((x as f32).to_bits()))
            .collect(),
        Ty::F64 => ROUNDING.iter().map(|&x| x.to_bits()).collect(),
        _ => Vec::new(),
    };
    for b in extra {
        if !v.contains(&b) {
            v.push(b);
        }
    }
    if !v.is_empty() {
        return v;
    }
    match ty {
        Ty::Bool => vec![0, 1],
        Ty::U8 | Ty::I8 => vec![0, 1, 2, 0x7f, 0x80, 0xff, 0x10],
        Ty::U16 | Ty::I16 => vec![0, 1, 0x7fff, 0x8000, 0xffff, 0xff, 0x100],
        Ty::U64 => vec![
            0,
            1,
            u64::MAX,
            0x8000_0000_0000_0000,
            0xffff_ffff,
            0x1_0000_0000,
        ],
        _ => I32_EDGES.iter().map(|&b| b as u64).collect(),
    }
}

fn random(ty: Ty, r: &mut Rng) -> u64 {
    let x = r.next();
    match ty {
        // half raw bit patterns (any class, NaNs included), half ordinary magnitudes
        Ty::F32 if x & 1 == 0 => (x >> 32) & 0xffff_ffff,
        Ty::F32 => {
            let v = ((x >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 2000.0;
            (v as f32).to_bits() as u64
        }
        Ty::F64 if x & 1 == 0 => x,
        Ty::F64 => (((x >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 2000.0).to_bits(),
        Ty::Bool => x & 1,
        Ty::U8 | Ty::I8 => x & 0xff,
        Ty::U16 | Ty::I16 => x & 0xffff,
        Ty::U64 => x,
        // integers: mostly small, sometimes any 32-bit pattern
        _ if x & 3 == 0 => (x >> 32) & 0xffff_ffff,
        _ => (x >> 32) % 512,
    }
}

/// Boundary values from the function's own constants (`i<decimal>`,
/// `f<float bits>`, `d<double bits>`, as `NvPurity.java` mines them): for an
/// integer parameter c-1, c and c+1 of every integer constant; for a float
/// parameter every float or double constant and its two neighbouring values
/// in the parameter's type.
fn boundaries(ty: Ty, consts: &[String]) -> Vec<u64> {
    let mut out = Vec::new();
    for t in consts {
        let (kind, body) = t.split_at(1);
        match (ty, kind) {
            (Ty::Bool, _) => {}
            (Ty::F32 | Ty::F64, "f" | "d") => {
                let Ok(bits) = u64::from_str_radix(body, 16) else {
                    continue;
                };
                let v = if kind == "f" {
                    f32::from_bits(bits as u32) as f64
                } else {
                    f64::from_bits(bits)
                };
                if ty == Ty::F32 {
                    let c = v as f32;
                    out.extend([c, c.next_down(), c.next_up()].map(|x| u64::from(x.to_bits())));
                } else {
                    out.extend([v, v.next_down(), v.next_up()].map(f64::to_bits));
                }
            }
            (Ty::F32 | Ty::F64, _) => {}
            (_, "i") => {
                let Ok(c) = body.parse::<i64>() else { continue };
                let mask = match ty {
                    Ty::U8 | Ty::I8 => 0xff,
                    Ty::U16 | Ty::I16 => 0xffff,
                    Ty::U64 => u64::MAX,
                    _ => 0xffff_ffff,
                };
                out.extend([c - 1, c, c + 1].map(|v| v as u64 & mask));
            }
            _ => {}
        }
    }
    out
}

/// A small set per type whose combinations across parameters matter: signed
/// zeros against each other (min/max and `>` vs `>=` differ only there),
/// equal operands, NaN and infinities on either side.
fn specials(ty: Ty) -> Vec<u64> {
    match ty {
        Ty::F32 => [
            0x0000_0000u32,
            0x8000_0000,
            0x3f80_0000,
            0xbf80_0000,
            0x3fc0_0000,
            0x7fc0_0000,
            0x7f80_0000,
            0xff80_0000,
        ]
        .iter()
        .map(|&b| u64::from(b))
        .collect(),
        Ty::F64 => vec![
            0,
            0x8000_0000_0000_0000,
            0x3ff0_0000_0000_0000,
            0xbff0_0000_0000_0000,
            0x3ff8_0000_0000_0000,
            0x7ff8_0000_0000_0000,
            0x7ff0_0000_0000_0000,
            0xfff0_0000_0000_0000,
        ],
        Ty::Bool => vec![0, 1],
        Ty::U8 | Ty::I8 => vec![0, 1, 0xff],
        Ty::U16 | Ty::I16 => vec![0, 1, 0xffff],
        Ty::U64 => vec![0, 1, u64::MAX],
        _ => vec![0, 1, 3, 0xffff_ffff, 0x8000_0000],
    }
}

/// Argument lists: first each parameter through its edge values and the
/// boundaries of the function's own constants, with the others at a plain
/// value (all of them, however many); for two or three parameters, every
/// combination of the per-type special values; then random lists up to
/// `count` in total and at least 16.
pub fn inputs(params: &[Ty], count: usize, seed: u64, consts: &[String]) -> Vec<Vec<Val>> {
    let mut r = Rng::new(seed);
    let plain = |t: Ty| -> u64 {
        match t {
            Ty::F32 => 0x3fc0_0000, // 1.5
            Ty::F64 => 0x3ff8_0000_0000_0000,
            _ => 3,
        }
    };
    let mut out = Vec::new();
    if params.is_empty() {
        out.push(Vec::new());
        return out;
    }
    for (i, &t) in params.iter().enumerate() {
        let mut values = edges(t);
        for b in boundaries(t, consts) {
            if !values.contains(&b) {
                values.push(b);
            }
        }
        for e in values {
            let v = params
                .iter()
                .enumerate()
                .map(|(j, &u)| Val {
                    ty: u,
                    bits: if j == i { e } else { plain(u) },
                })
                .collect();
            out.push(v);
        }
    }
    if (2..=3).contains(&params.len()) {
        let sets: Vec<Vec<u64>> = params.iter().map(|&t| specials(t)).collect();
        let mut idx = vec![0usize; params.len()];
        loop {
            let v: Vec<Val> = params
                .iter()
                .enumerate()
                .map(|(j, &u)| Val {
                    ty: u,
                    bits: sets[j][idx[j]],
                })
                .collect();
            if !out.contains(&v) {
                out.push(v);
            }
            // odometer over the cartesian product
            let mut k = 0;
            while k < idx.len() {
                idx[k] += 1;
                if idx[k] < sets[k].len() {
                    break;
                }
                idx[k] = 0;
                k += 1;
            }
            if k == idx.len() {
                break;
            }
        }
    }
    let total = count.max(out.len() + 16);
    while out.len() < total {
        out.push(
            params
                .iter()
                .map(|&t| Val {
                    ty: t,
                    bits: random(t, &mut r),
                })
                .collect(),
        );
    }
    out
}

/// The `ret` field of an `nv-call` vector.
pub fn nv_call_ret(ty: Ty) -> &'static str {
    match ty {
        Ty::F32 => "f32_x87",
        Ty::F64 => "f64_x87",
        Ty::U64 => "i64",
        _ => "u32",
    }
}

/// One vector line.
pub fn vector_line(id: &str, addr: u32, cc: &str, args: &[Val], ret: Ty) -> String {
    let a: Vec<String> = args
        .iter()
        .map(|v| format!("\"{}\"", v.nv_call()))
        .collect();
    format!(
        "{{\"id\":\"{id}\",\"fn\":\"0x{addr:08x}\",\"cc\":\"{cc}\",\"args\":[{}],\"ret\":\"{}\"}}",
        a.join(","),
        nv_call_ret(ret)
    )
}

/// nv-call's buffer arena (its default `--arena`), where both sides place the
/// buffers pointer arguments point into. Inside the engine's heap range, far
/// above where a fresh engine's bump allocator is (checked per vector).
pub const ARENA: u32 = 0x3000_0000;

/// Where nv-call puts each buffer of a vector: from the arena base, each
/// 16-byte aligned after the previous one, in order (nv-call-engine `Arena`).
pub fn buffer_addrs(bufs: &[crate::list::Buf]) -> Vec<u32> {
    let mut used = 0u32;
    bufs.iter()
        .map(|b| {
            let start = (used + 15) & !15;
            used = start + b.size;
            ARENA + start
        })
        .collect()
}

/// One vector of a function with pointer parameters: the argument values (a
/// pointer parameter's `bits` are unused; it points into its buffer) and
/// every buffer's contents, in the order of `Pure::bufs`.
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub args: Vec<Val>,
    pub bufs: Vec<Vec<u8>>,
}

/// Values for one field the exe reads, by its size: typed edges (integers
/// and float/double bit patterns) and the function's own constants.
fn field_pool(size: u32, consts: &[String]) -> Vec<u64> {
    let mut v: Vec<u64> = match size {
        1 => vec![0, 1, 2, 0x7f, 0x80, 0xff],
        2 => vec![0, 1, 2, 0x7fff, 0x8000, 0xffff, 0x100],
        4 => vec![
            0,
            1,
            2,
            3,
            0xffff_ffff,
            0x8000_0000,
            0x7fff_ffff,
            0x3f80_0000, // 1.0f
            0xbf80_0000, // -1.0f
            0x3fc0_0000, // 1.5f
            0x3f00_0000, // 0.5f
            0x7fc0_0000, // qNaN
            0x7f80_0000, // inf
            0x0000_0001, // denormal / 1
        ],
        8 => vec![
            0,
            1,
            0x3ff0_0000_0000_0000,
            0xbff0_0000_0000_0000,
            0x3ff8_0000_0000_0000,
            0x7ff8_0000_0000_0000,
            0x7ff0_0000_0000_0000,
            u64::MAX,
        ],
        _ => vec![0, 1],
    };
    let mask = if size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    };
    for t in consts {
        let (kind, body) = t.split_at(1);
        let extra: Vec<u64> = match (kind, size) {
            ("i", _) => match body.parse::<i64>() {
                Ok(c) => [c - 1, c, c + 1].iter().map(|&x| x as u64 & mask).collect(),
                Err(_) => vec![],
            },
            ("f", 4) | ("d", 8) => u64::from_str_radix(body, 16).into_iter().collect(),
            ("d", 4) => u64::from_str_radix(body, 16)
                .map(|b| u64::from((f64::from_bits(b) as f32).to_bits()))
                .into_iter()
                .collect(),
            ("f", 8) => u64::from_str_radix(body, 16)
                .map(|b| (f32::from_bits(b as u32) as f64).to_bits())
                .into_iter()
                .collect(),
            _ => vec![],
        };
        for x in extra {
            if !v.contains(&x) {
                v.push(x);
            }
        }
    }
    v
}

fn random_bytes(len: usize, r: &mut Rng) -> Vec<u8> {
    (0..len).map(|_| (r.next() >> 56) as u8).collect()
}

/// Vectors for a function with pointer parameters (phase 1, #193). Buffer
/// contents per vector: all zero, all 0xff, two seeded random fills, and
/// "structured" fills where every field the exe reads takes, in turn, each
/// value of its pool (typed edges and the function's constants), shifted per
/// field so fields don't move in lockstep. Scalar parameters go through
/// [`inputs`] as before, each list paired with one of the buffer fills.
pub fn pointer_cases(
    params: &[Ty],
    bufs: &[crate::list::Buf],
    count: usize,
    seed: u64,
    consts: &[String],
) -> Vec<Case> {
    let mut r = Rng::new(seed ^ 0x5bd1_e995);
    let scalar_tys: Vec<Ty> = params.iter().copied().filter(|t| *t != Ty::Ptr).collect();
    let lists = if scalar_tys.is_empty() {
        vec![Vec::new()]
    } else {
        inputs(&scalar_tys, count, seed, consts)
    };
    let plain: Vec<Val> = scalar_tys
        .iter()
        .map(|&t| Val {
            ty: t,
            bits: match t {
                Ty::F32 => 0x3fc0_0000,
                Ty::F64 => 0x3ff8_0000_0000_0000,
                _ => 3,
            },
        })
        .collect();

    let mut fills: Vec<Vec<Vec<u8>>> = Vec::new();
    fills.push(bufs.iter().map(|b| vec![0u8; b.size as usize]).collect());
    fills.push(bufs.iter().map(|b| vec![0xffu8; b.size as usize]).collect());
    for _ in 0..2 {
        fills.push(
            bufs.iter()
                .map(|b| random_bytes(b.size as usize, &mut r))
                .collect(),
        );
    }
    // The pointer arguments' own values (buffer address + bias): a 4-byte field
    // may be compared with one of them (`if (this->procedure == procedure)`).
    let pointers: Vec<u64> = buffer_addrs(bufs)
        .iter()
        .zip(bufs)
        .map(|(a, b)| u64::from(a + b.bias))
        .collect();
    let pools: Vec<Vec<Vec<u64>>> = bufs
        .iter()
        .map(|b| {
            b.fields
                .iter()
                .map(|&(_, z)| {
                    let mut pool = field_pool(z, consts);
                    if z == 4 {
                        for &p in &pointers {
                            if !pool.contains(&p) {
                                pool.push(p);
                            }
                        }
                    }
                    pool
                })
                .collect()
        })
        .collect();
    let rounds = pools
        .iter()
        .flatten()
        .map(Vec::len)
        .max()
        .unwrap_or(0)
        .min(32);
    // Two passes: fields shifted against each other (each field sees every value
    // while the others differ), then in lockstep (every field the same value, so
    // comparisons between fields, or of several fields with one pointer, can be equal).
    for (k, lockstep) in (0..rounds)
        .map(|k| (k, false))
        .chain((0..rounds).map(|k| (k, true)))
    {
        // The shift counts fields across all buffers, so the fields of two
        // one-field buffers (compare(a, b)) differ in the shifted pass.
        let mut field_no = 0usize;
        let fill: Vec<Vec<u8>> = bufs
            .iter()
            .zip(&pools)
            .map(|(b, ps)| {
                let mut bytes = vec![0u8; b.size as usize];
                for (&(off, z), pool) in b.fields.iter().zip(ps) {
                    let value = pool[if lockstep { k } else { k + field_no } % pool.len()];
                    field_no += 1;
                    let at = i64::from(b.bias) + off;
                    for i in 0..z as usize {
                        if let Some(slot) =
                            usize::try_from(at).ok().and_then(|a| bytes.get_mut(a + i))
                        {
                            *slot = (value >> (8 * i)) as u8;
                        }
                    }
                }
                bytes
            })
            .collect();
        if !fills.contains(&fill) {
            fills.push(fill);
        }
    }

    let merge = |scalars: &[Val]| -> Vec<Val> {
        let mut it = scalars.iter();
        params
            .iter()
            .map(|&t| {
                if t == Ty::Ptr {
                    Val {
                        ty: Ty::Ptr,
                        bits: 0,
                    }
                } else {
                    *it.next().expect("one scalar per non-pointer parameter")
                }
            })
            .collect()
    };
    let mut out: Vec<Case> = fills
        .iter()
        .map(|f| Case {
            args: merge(&plain),
            bufs: f.clone(),
        })
        .collect();
    for (i, l) in lists.iter().enumerate() {
        let case = Case {
            args: merge(l),
            bufs: fills[i % fills.len()].clone(),
        };
        if !out.contains(&case) {
            out.push(case);
        }
    }
    out
}

/// One vector line with buffers: a pointer parameter becomes
/// `ptr:b<j>+0x<bias>` (buffer `j` of `bufs`).
pub fn vector_line_buffers(
    id: &str,
    addr: u32,
    cc: &str,
    case: &Case,
    bufs: &[crate::list::Buf],
    ret: Ty,
) -> String {
    let args: Vec<String> = case
        .args
        .iter()
        .enumerate()
        .map(|(k, v)| match bufs.iter().position(|b| b.param == k) {
            Some(j) if v.ty == Ty::Ptr => format!("\"ptr:b{j}+0x{:x}\"", bufs[j].bias),
            _ => format!("\"{}\"", v.nv_call()),
        })
        .collect();
    let buffers: Vec<String> = bufs
        .iter()
        .zip(&case.bufs)
        .enumerate()
        .map(|(j, (b, init))| {
            let hex: String = init.iter().map(|x| format!("{x:02x}")).collect();
            format!("\"b{j}\":{{\"size\":{},\"init\":\"{hex}\"}}", b.size)
        })
        .collect();
    format!(
        "{{\"id\":\"{id}\",\"fn\":\"0x{addr:08x}\",\"cc\":\"{cc}\",\"args\":[{}],\"buffers\":{{{}}},\"ret\":\"{}\"}}",
        args.join(","),
        buffers.join(","),
        nv_call_ret(ret)
    )
}

/// A result line's buffers by name, after the call.
pub type Buffers = Vec<(String, Vec<u8>)>;

/// The buffers of a result line after the call: (id, [(name, bytes)]).
pub fn parse_buffers(line: &str) -> Option<(String, Buffers)> {
    let v = Value::parse(line)?;
    if v.get("type")?.as_str()? != "result" {
        return None;
    }
    let id = v.get("id")?.as_str()?.to_string();
    let mut out = Vec::new();
    if let Some(Value::Obj(kv)) = v.get("buffers") {
        for (name, hex) in kv {
            let h = hex.as_str()?;
            let bytes = (0..h.len() / 2)
                .map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).ok())
                .collect::<Option<Vec<u8>>>()?;
            out.push((name.clone(), bytes));
        }
    }
    Some((id, out))
}

/// What the exe produced for one vector.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// eax, edx, ST0 as f32 bits, ST0 as f64 bits (None when ST0 was empty), callee-popped bytes
    Value {
        eax: u32,
        edx: u32,
        st0_f32: Option<u32>,
        st0_f64: Option<u64>,
        popped: u32,
    },
    /// The exe raised an exception it did not handle. `read_at`: the address
    /// of an access violation on a read; `name`: nv-call's exception name.
    Fault {
        text: String,
        name: String,
        read_at: Option<u32>,
    },
}

fn hex_field(v: Option<&Value>) -> Option<u64> {
    let s = v?.as_str()?;
    u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
}

/// Parses a `{"type":"result",...}` line into (id, outcome).
pub fn parse_result(line: &str) -> Option<(String, Outcome)> {
    let v = Value::parse(line)?;
    if v.get("type")?.as_str()? != "result" {
        return None;
    }
    let id = v.get("id")?.as_str()?.to_string();
    if let Some(f) = v.get("fault") {
        if f.is_object() {
            let name = f
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let read = f.get("access").and_then(Value::as_str) == Some("read");
            let read_at = if name == "access_violation" && read {
                hex_field(f.get("address")).map(|a| a as u32)
            } else {
                None
            };
            return Some((
                id,
                Outcome::Fault {
                    text: f.show(),
                    name,
                    read_at,
                },
            ));
        }
    }
    let regs = v.get("regs")?;
    let st0 = regs.get("st0").filter(|s| s.is_object());
    Some((
        id,
        Outcome::Value {
            eax: hex_field(regs.get("eax"))? as u32,
            edx: hex_field(regs.get("edx")).unwrap_or(0) as u32,
            st0_f32: st0
                .and_then(|s| hex_field(s.get("f32_bits")))
                .map(|b| b as u32),
            st0_f64: st0.and_then(|s| hex_field(s.get("f64_bits"))),
            popped: regs
                .get("callee_popped")
                .and_then(Value::as_num)
                .unwrap_or(0.0) as u32,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_covering() {
        let a = inputs(&[Ty::F32, Ty::I32], 64, 0x0040_1000, &[]);
        let b = inputs(&[Ty::F32, Ty::I32], 64, 0x0040_1000, &[]);
        assert_eq!(a, b);
        assert!(a.len() >= 64);
        assert!(
            a.iter().any(|v| v[0].bits == 0x7f80_0001),
            "sNaN edge present"
        );
        assert!(
            a.iter().any(|v| v[1].bits == 0x8000_0000),
            "INT_MIN edge present"
        );
    }

    #[test]
    fn words_match_engine_abi() {
        let mut w = Vec::new();
        Val {
            ty: Ty::F64,
            bits: 0x1122_3344_5566_7788,
        }
        .put(&mut w);
        Val {
            ty: Ty::I8,
            bits: 0xff,
        }
        .put(&mut w);
        assert_eq!(w, vec![0x5566_7788, 0x1122_3344, 0xffff_ffff]);
    }

    #[test]
    fn pointer_vectors() {
        use crate::list::Buf;
        // a getter reading a byte at +4 and a float at +8 of `this`, plus one u32
        let bufs = vec![Buf::from_reads(0, &[(4, 1), (8, 4)])];
        let k = vec!["i6".to_string()];
        let cases = pointer_cases(&[Ty::Ptr, Ty::U32], &bufs, 64, 9, &k);
        assert!(
            cases.iter().any(|c| c.bufs[0].iter().all(|&b| b == 0)),
            "zero fill"
        );
        assert!(
            cases.iter().any(|c| c.bufs[0][4] == 6),
            "mined constant lands in the byte field"
        );
        assert!(
            cases
                .iter()
                .any(|c| c.bufs[0][8..12] == 1.0f32.to_bits().to_le_bytes()),
            "float edge in the float field"
        );
        assert!(cases
            .iter()
            .all(|c| c.args[0].ty == Ty::Ptr && c.args.len() == 2));
        let line = vector_line_buffers("x_0", 0x0040_1170, "thiscall", &cases[0], &bufs, Ty::U8);
        assert!(
            line.contains("\"ptr:b0+0x0\"") && line.contains("\"b0\":{\"size\":"),
            "{line}"
        );
        let res = r#"{"type":"result","id":"x_0","buffers":{"b0":"00ff10"}}"#;
        assert_eq!(
            parse_buffers(res),
            Some(("x_0".into(), vec![("b0".into(), vec![0, 0xff, 0x10])]))
        );
    }

    #[test]
    fn two_buffers_differ() {
        use crate::list::Buf;
        // compare(a, b) over one u16 field each: the fields must be unequal in
        // both orders in some vectors, and equal in others
        let bufs = vec![Buf::from_reads(0, &[(0, 2)]), Buf::from_reads(1, &[(0, 2)])];
        let cases = pointer_cases(&[Ty::Ptr, Ty::Ptr], &bufs, 16, 3, &[]);
        let v = |c: &Case, j: usize| u16::from_le_bytes([c.bufs[j][0], c.bufs[j][1]]);
        assert!(cases.iter().any(|c| v(c, 0) < v(c, 1)));
        assert!(cases.iter().any(|c| v(c, 0) > v(c, 1)));
        assert!(
            cases.iter().filter(|c| v(c, 0) != v(c, 1)).count() >= 8,
            "structured fills, not just the random ones"
        );
        assert!(cases.iter().any(|c| v(c, 0) == v(c, 1)));
    }

    #[test]
    fn combinations_and_ties() {
        let all = inputs(&[Ty::F32, Ty::F32], 64, 7, &[]);
        // +0 against -0 in both orders (min/max with > vs >= differ only there)
        assert!(all
            .iter()
            .any(|v| v[0].bits == 0 && v[1].bits == 0x8000_0000));
        assert!(all
            .iter()
            .any(|v| v[0].bits == 0x8000_0000 && v[1].bits == 0));
        // equal operands
        assert!(all
            .iter()
            .any(|v| v[0].bits == 0x3f80_0000 && v[1].bits == 0x3f80_0000));
        // rounding ties reach a one-parameter function
        let one = inputs(&[Ty::F32], 16, 7, &[]);
        assert!(one.iter().any(|v| v[0].bits == u64::from(1.5f32.to_bits())));
        assert!(one
            .iter()
            .any(|v| v[0].bits == u64::from((-0.5f32).to_bits())));
    }

    #[test]
    fn constant_boundaries() {
        // TESFurniture::IsSitMarker compares with 10, 20 and 26
        let k: Vec<String> = ["i10", "i20", "i26"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = boundaries(Ty::U32, &k);
        for v in [9, 10, 11, 19, 20, 21, 25, 26, 27] {
            assert!(b.contains(&v), "{v}");
        }
        let all = inputs(&[Ty::U32], 64, 1, &k);
        assert!(
            all.iter().any(|v| v[0].bits == 26),
            "boundary reaches the input list"
        );
        // a double 1.0 constant gives 1.0f and its float neighbours for an f32 parameter
        let f = boundaries(Ty::F32, &["d3ff0000000000000".to_string()]);
        assert_eq!(f, vec![0x3f80_0000, 0x3f7f_ffff, 0x3f80_0001]);
        // -1 as a byte
        assert!(boundaries(Ty::U8, &["i0".to_string()]).contains(&0xff));
    }

    #[test]
    fn result_lines() {
        let ok = r#"{"type":"result","id":"a_1","fault":null,"regs":{"eax":"0x00000006","edx":"0x00000000","st0":null,"callee_popped":0},"value":{"u32":6}}"#;
        assert_eq!(
            parse_result(ok),
            Some((
                "a_1".into(),
                Outcome::Value {
                    eax: 6,
                    edx: 0,
                    st0_f32: None,
                    st0_f64: None,
                    popped: 0
                }
            ))
        );
        let f = r#"{"type":"result","id":"a_2","fault":{"kind":"exception","name":"access_violation","address":"0x021F91C8","access":"read"},"regs":null}"#;
        assert!(matches!(
            parse_result(f),
            Some((
                _,
                Outcome::Fault {
                    read_at: Some(0x021f_91c8),
                    ..
                }
            ))
        ));
    }
}
