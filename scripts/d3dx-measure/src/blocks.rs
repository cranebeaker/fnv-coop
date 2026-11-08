//! `blocks`: the 32-bit side of `scripts/d3dx-tex-verify`'s block checks. Calls the DXT block
//! encoders and decoders inside the player's 32-bit `d3dx9_38.dll` (June 2008, 3,850,760 bytes:
//! the internal functions at the addresses below are only there in that build, which `blocks`
//! checks first) with the x87 control word each request names, so `crates/d3dx-tex` can be
//! compared with the DLL block by block. The maintainer allows calling and reading the DLL to
//! match it exactly (2026-11-08); nothing of it is written out.
//!
//! Requests on stdin, answers on stdout, until stdin ends:
//! - `1`, `3`, `5`: encode a DXT1, DXT3 or DXT5 block. Then the control word (u32), dither
//!   (u32), 64 floats (16 pixels: red, green, blue, alpha). Answer: 16 bytes (DXT1: 8, then
//!   zeros).
//! - `11`, `13`, `15`: decode a DXT1 colour, DXT3 or DXT5 block. Then the control word, 0, the
//!   16 block bytes. Answer: 64 floats.
//! - `20`: the colour endpoint search. Then the control word, the step count (3 or 4), 64
//!   floats. Answer: 6 floats (the two endpoints' red, green, blue).
use std::ffi::c_void;
use std::io::{Read, Write};

/// The DLL's preferred base; the internal functions are found at their offset from it.
const BASE: usize = 0x40_0000;
const ENCODE_DXT1: usize = 0x629c16;
const ENCODE_DXT3: usize = 0x629d52;
const ENCODE_DXT5: usize = 0x629e94;
const DECODE_DXT1: usize = 0x62990d;
const DECODE_DXT3: usize = 0x629a3d;
const DECODE_DXT5: usize = 0x629ac7;

const OPTIMIZE_RGB: usize = 0x6288d0;

type OptimizeRgb = unsafe extern "stdcall" fn(*mut f32, *mut f32, *const f32, u32);
type Encode = unsafe extern "stdcall" fn(*mut u8, *const f32, u32) -> i32;
type Decode = unsafe extern "stdcall" fn(*mut f32, *const u8) -> i32;

fn read_u32(input: &mut impl Read) -> Option<u32> {
    let mut b = [0u8; 4];
    input.read_exact(&mut b).ok()?;
    Some(u32::from_le_bytes(b))
}

pub unsafe fn serve(module: *mut c_void) {
    // The build check: the DLL's file size and the first bytes of each function used.
    let mut path = vec![0u8; 260];
    let n = crate::GetModuleFileNameA(module, path.as_mut_ptr(), 260) as usize;
    let file = String::from_utf8_lossy(&path[..n]).to_string();
    let len = std::fs::metadata(&file).map(|m| m.len()).unwrap_or(0);
    assert_eq!(
        len, 3_850_760,
        "{file} is not the d3dx9_38.dll build these addresses are from"
    );
    let at = |va: usize| (module as usize + va - BASE) as *const u8;
    for va in [
        ENCODE_DXT1,
        ENCODE_DXT3,
        ENCODE_DXT5,
        DECODE_DXT1,
        DECODE_DXT5,
        OPTIMIZE_RGB,
    ] {
        // `MOV EDI,EDI; PUSH EBP; MOV EBP,ESP`.
        let head = std::slice::from_raw_parts(at(va), 5);
        assert_eq!(head, [0x8b, 0xff, 0x55, 0x8b, 0xec], "function at {va:#x}");
    }
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    while let Some(kind) = read_u32(&mut input) {
        let cw = read_u32(&mut input).expect("a control word") as u16;
        let flag = read_u32(&mut input).expect("a flag");
        match kind {
            1 | 3 | 5 => {
                let mut px = [0f32; 64];
                for p in px.iter_mut() {
                    *p = f32::from_bits(read_u32(&mut input).expect("a pixel"));
                }
                let mut block = [0u8; 16];
                let f: Encode = std::mem::transmute(at(match kind {
                    1 => ENCODE_DXT1,
                    3 => ENCODE_DXT3,
                    _ => ENCODE_DXT5,
                }));
                let saved = crate::x87_control_word();
                crate::set_x87_control_word(cw);
                f(block.as_mut_ptr(), px.as_ptr(), flag);
                crate::set_x87_control_word(saved);
                out.write_all(&block).unwrap();
            }
            11 | 13 | 15 => {
                let mut block = [0u8; 16];
                input.read_exact(&mut block).expect("a block");
                let mut px = [0f32; 64];
                let f: Decode = std::mem::transmute(at(match kind {
                    11 => DECODE_DXT1,
                    13 => DECODE_DXT3,
                    _ => DECODE_DXT5,
                }));
                let saved = crate::x87_control_word();
                crate::set_x87_control_word(cw);
                f(px.as_mut_ptr(), block.as_ptr());
                crate::set_x87_control_word(saved);
                for p in px {
                    out.write_all(&p.to_bits().to_le_bytes()).unwrap();
                }
            }
            20 => {
                // The colour endpoint search: `flag` steps (3 or 4), 64 floats in, X and Y out.
                let mut px = [0f32; 64];
                for p in px.iter_mut() {
                    *p = f32::from_bits(read_u32(&mut input).expect("a pixel"));
                }
                let (mut x, mut y) = ([0f32; 4], [0f32; 4]);
                let f: OptimizeRgb = std::mem::transmute(at(OPTIMIZE_RGB));
                let saved = crate::x87_control_word();
                crate::set_x87_control_word(cw);
                f(x.as_mut_ptr(), y.as_mut_ptr(), px.as_ptr(), flag);
                crate::set_x87_control_word(saved);
                for v in x[..3].iter().chain(y[..3].iter()) {
                    out.write_all(&v.to_bits().to_le_bytes()).unwrap();
                }
            }
            other => panic!("unknown request {other}"),
        }
        out.flush().unwrap();
    }
}
