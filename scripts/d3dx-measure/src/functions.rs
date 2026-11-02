//! The other D3DX maths functions the game imports, measured the same way as
//! `D3DXMatrixMultiply`: each line `Name<TAB>alias<TAB>x87 control word<TAB>inputs (words, each
//! argument's separated by `|`)<TAB>result words (`|` before the determinant for
//! `D3DXMatrixInverse`)<TAB>returned (1 the out pointer, 0 null, 2 anything else)`.
//! `alias` is `none` or `out=in` (the out pointer is the first input's).
use crate::{matrix, value, Rng};
use std::ffi::c_void;
use std::io::Write;

type P = *mut u32;
type F1 = unsafe extern "system" fn(P, P) -> P;
type F2 = unsafe extern "system" fn(P, P, P) -> P;
type Inv = unsafe extern "system" fn(P, P, P) -> P;
type Ypr = unsafe extern "system" fn(P, f32, f32, f32) -> P;

/// (export name, argument sizes in words, out size): the out pointer first, then the inputs.
pub const FUNCTIONS: &[(&str, &[usize], usize)] = &[
    ("D3DXMatrixTranspose", &[16], 16),
    ("D3DXMatrixMultiplyTranspose", &[16, 16], 16),
    ("D3DXMatrixInverse", &[16], 16),
    ("D3DXMatrixRotationYawPitchRoll", &[1, 1, 1], 16),
    ("D3DXVec3Normalize", &[3], 3),
    ("D3DXVec3TransformCoord", &[3, 16], 3),
    ("D3DXVec3TransformNormal", &[3, 16], 3),
    ("D3DXVec4Transform", &[4, 16], 4),
    ("D3DXPlaneNormalize", &[4], 4),
    ("D3DXPlaneTransform", &[4, 16], 4),
];

fn words(m: &[u32]) -> String {
    m.iter().map(|w| format!("{w:08x}")).collect::<Vec<_>>().join(" ")
}

/// Inputs for one argument of `n` words: matrices of every class (and, for the inverse,
/// singular ones), vectors and angles of every value class.
fn input(rng: &mut Rng, n: usize, i: usize) -> Vec<u32> {
    if n == 16 {
        let mut m = matrix(rng, (i as u64) % 6).to_vec();
        match i % 9 {
            // A repeated row (singular).
            7 => {
                for k in 0..4 {
                    m[4 + k] = m[k];
                }
            }
            // A rotation-and-translation-like matrix: small values, last column 0 0 0 1.
            8 => {
                for k in 0..12 {
                    m[k] = value(rng, 0);
                }
                m[3] = 0;
                m[7] = 0;
                m[11] = 0;
                m[15] = 0x3f80_0000;
            }
            _ => {}
        }
        m
    } else {
        let class = if (i / 6) % 4 == 0 { 5 } else { (i as u64) % 5 };
        (0..n)
            .map(|_| {
                let c = if class == 5 { rng.below(5) } else { class };
                value(rng, c)
            })
            .collect()
    }
}

pub unsafe fn measure(o: &mut impl Write, module: *mut c_void, wanted: &str, count: usize) {
    for &(name, args, out_size) in FUNCTIONS {
        if wanted != "all" && wanted != name {
            continue;
        }
        let mut cname = name.as_bytes().to_vec();
        cname.push(0);
        let f = crate::GetProcAddress(module, cname.as_ptr());
        assert!(!f.is_null(), "{name} not in the DLL");
        let mut rng = Rng(0x6e76_2d72_7364_3379 ^ name.len() as u64);
        for &cw in &[0x027fu16, 0x007f] {
            for i in 0..count {
                let inputs: Vec<Vec<u32>> = args.iter().map(|&n| input(&mut rng, n, i)).collect();
                let aliases: &[&str] = if args[0] == out_size { &["none", "out=in"] } else { &["none"] };
                for &alias in aliases {
                    for with_det in if name == "D3DXMatrixInverse" { &[true, false][..] } else { &[true][..] } {
                        let mut bufs: Vec<Vec<u32>> = inputs.clone();
                        let mut out = vec![0x5555_5555u32; out_size];
                        let mut det = [0x5555_5555u32];
                        let po: P = if alias == "out=in" { bufs[0].as_mut_ptr() } else { out.as_mut_ptr() };
                        crate::set_x87_control_word(cw);
                        let ret = match name {
                            "D3DXMatrixRotationYawPitchRoll" => {
                                let g: Ypr = std::mem::transmute(f);
                                g(po, f32::from_bits(bufs[0][0]), f32::from_bits(bufs[1][0]), f32::from_bits(bufs[2][0]))
                            }
                            "D3DXMatrixInverse" => {
                                let g: Inv = std::mem::transmute(f);
                                let d = if *with_det { det.as_mut_ptr() } else { std::ptr::null_mut() };
                                g(po, d, bufs[0].as_mut_ptr())
                            }
                            _ if args.len() == 1 => {
                                let g: F1 = std::mem::transmute(f);
                                g(po, bufs[0].as_mut_ptr())
                            }
                            _ => {
                                let g: F2 = std::mem::transmute(f);
                                let p1 = bufs[1].as_mut_ptr();
                                g(po, bufs[0].as_mut_ptr(), p1)
                            }
                        };
                        crate::set_x87_control_word(0x027f);
                        let result: Vec<u32> = std::slice::from_raw_parts(po, out_size).to_vec();
                        let returned = if ret == po { 1 } else if ret.is_null() { 0 } else { 2 };
                        let ins: Vec<String> = inputs.iter().map(|v| words(v)).collect();
                        let mut res = words(&result);
                        if name == "D3DXMatrixInverse" {
                            res += &if *with_det { format!("|{:08x}", det[0]) } else { "|-".to_string() };
                        }
                        writeln!(o, "{name}\t{alias}\t{cw:04x}\t{}\t{res}\t{returned}", ins.join("|")).unwrap();
                    }
                }
            }
        }
    }
}

/// The edge values of `functions-edges`: zeros, ones, the smallest denormal, the smallest
/// normal, a tiny and a big normal, the largest finite, infinities, a quiet and a signalling NaN.
const EDGES: &[u32] = &[
    0x0000_0000, 0x8000_0000, 0x3f80_0000, 0xbf80_0000, 0x0000_0001, 0x0080_0000, 0x1e3c_e508,
    0x5f80_0000, 0x7f7f_ffff, 0x7f80_0000, 0xff80_0000, 0x7fc1_2345, 0x7f80_0001,
];

/// The inputs of `functions-edges` for an argument of `n` words: every combination of three
/// edge values (a fourth word from a few of them), or matrices (identity with one word made an
/// edge value, each position; zero; the identity scaled by each edge value).
fn edge_inputs(n: usize) -> Vec<Vec<u32>> {
    let mut out = Vec::new();
    match n {
        1 | 3 => {
            for &a in EDGES {
                if n == 1 {
                    out.push(vec![a]);
                    continue;
                }
                for &b in EDGES {
                    for &c in EDGES {
                        out.push(vec![a, b, c]);
                    }
                }
            }
        }
        4 => {
            for &a in EDGES {
                for &b in EDGES {
                    for &c in EDGES {
                        for &d in &[0x0000_0000u32, 0x3f80_0000, 0x7f80_0000, 0x7fc1_2345, 0x0000_0001] {
                            out.push(vec![a, b, c, d]);
                        }
                    }
                }
            }
        }
        _ => {
            let identity: Vec<u32> = (0..16).map(|i| if i % 5 == 0 { 0x3f80_0000 } else { 0 }).collect();
            for &v in EDGES {
                for p in 0..16 {
                    let mut m = identity.clone();
                    m[p] = v;
                    out.push(m);
                }
                out.push(identity.iter().map(|&w| if w != 0 { v } else { 0 }).collect());
            }
            out.push(vec![0; 16]);
        }
    }
    out
}

/// `functions-edges NAME`: `NAME` (one of [`FUNCTIONS`]) on the edge inputs, both control words,
/// lines as in [`measure`]. A function of a vector and a matrix takes the edge vectors with a
/// few matrices and the edge matrices with a few vectors.
pub unsafe fn edges(o: &mut impl Write, module: *mut c_void, wanted: &str) {
    for &(name, args, out_size) in FUNCTIONS {
        if wanted != "all" && wanted != name {
            continue;
        }
        let mut cname = name.as_bytes().to_vec();
        cname.push(0);
        let f = crate::GetProcAddress(module, cname.as_ptr());
        assert!(!f.is_null(), "{name} not in the DLL");
        let cases: Vec<Vec<Vec<u32>>> = if args.len() == 2 {
            let vectors = edge_inputs(args[0]);
            let matrices = edge_inputs(16);
            let some_m: Vec<Vec<u32>> = matrices.iter().step_by(37).cloned().collect();
            let some_v: Vec<Vec<u32>> = vectors.iter().step_by(97).cloned().collect();
            let mut c: Vec<Vec<Vec<u32>>> = Vec::new();
            for m in &some_m {
                for v in &vectors {
                    c.push(vec![v.clone(), m.clone()]);
                }
            }
            for v in &some_v {
                for m in &matrices {
                    c.push(vec![v.clone(), m.clone()]);
                }
            }
            c
        } else if args.len() == 3 {
            let e = edge_inputs(1);
            let mut c = Vec::new();
            for a in &e {
                for b in &e {
                    for d in &e {
                        c.push(vec![a.clone(), b.clone(), d.clone()]);
                    }
                }
            }
            c
        } else {
            edge_inputs(args[0]).into_iter().map(|v| vec![v]).collect()
        };
        for &cw in &[0x027fu16, 0x007f] {
            for inputs in &cases {
                let aliases: &[&str] = if args[0] == out_size { &["none", "out=in"] } else { &["none"] };
                // A function of a matrix runs with the matrix on a 16-byte boundary and off it
                // (the DLL takes another path), recorded as `/m16` and `/m4` after the alias.
                let aligns: &[usize] = if args.len() == 2 { &[16, 4] } else { &[0] };
                for &alias in aliases {
                    for &align in aligns {
                        for with_det in if name == "D3DXMatrixInverse" { &[true, false][..] } else { &[true][..] } {
                            let mut bufs: Vec<Vec<u32>> = inputs.clone();
                            let mut room = [0u32; 24];
                            let base = (room.as_ptr() as usize).wrapping_neg() % 16 / 4;
                            let at = base + usize::from(align == 4);
                            if args.len() == 2 {
                                room[at..at + 16].copy_from_slice(&bufs[1]);
                            }
                            let mut out = vec![0x5555_5555u32; out_size];
                            let mut det = [0x5555_5555u32];
                            let po: P = if alias == "out=in" { bufs[0].as_mut_ptr() } else { out.as_mut_ptr() };
                            crate::set_x87_control_word(cw);
                            let ret = match name {
                                "D3DXMatrixRotationYawPitchRoll" => {
                                    let g: Ypr = std::mem::transmute(f);
                                    g(po, f32::from_bits(bufs[0][0]), f32::from_bits(bufs[1][0]), f32::from_bits(bufs[2][0]))
                                }
                                "D3DXMatrixInverse" => {
                                    let g: Inv = std::mem::transmute(f);
                                    let d = if *with_det { det.as_mut_ptr() } else { std::ptr::null_mut() };
                                    g(po, d, bufs[0].as_mut_ptr())
                                }
                                _ if args.len() == 1 => {
                                    let g: F1 = std::mem::transmute(f);
                                    g(po, bufs[0].as_mut_ptr())
                                }
                                _ => {
                                    let g: F2 = std::mem::transmute(f);
                                    g(po, bufs[0].as_mut_ptr(), room[at..].as_mut_ptr())
                                }
                            };
                            crate::set_x87_control_word(0x027f);
                            let result: Vec<u32> = std::slice::from_raw_parts(po, out_size).to_vec();
                            let returned = if ret == po { 1 } else if ret.is_null() { 0 } else { 2 };
                            let ins: Vec<String> = inputs.iter().map(|v| words(v)).collect();
                            let mut res = words(&result);
                            if name == "D3DXMatrixInverse" {
                                res += &if *with_det { format!("|{:08x}", det[0]) } else { "|-".to_string() };
                            }
                            let tag = if align == 0 { alias.to_string() } else { format!("{alias}/m{align}") };
                            writeln!(o, "{name}\t{tag}\t{cw:04x}\t{}\t{res}\t{returned}", ins.join("|")).unwrap();
                        }
                    }
                }
            }
        }
    }
}
