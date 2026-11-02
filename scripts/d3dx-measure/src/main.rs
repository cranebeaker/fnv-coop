//! d3dx-measure: calls the player's `d3dx9_38.dll` in a 32-bit process on many inputs and writes
//! each call's inputs and results bit for bit (hex words), for checking the platform's own D3DX
//! functions (`engine_core::units::platform::d3dx`) against. Only the DLL's documented entry
//! points are called; nothing of the DLL itself is read or written out. The DLL picks a
//! CPU-specific path, so the answers are this machine's: the first lines name the CPU.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/d3dx-measure/Cargo.toml -- matrix-multiply [count] > FILE`
//! `-- functions [count] [D3DXName]` measures the other maths functions (`functions.rs`).
//! (`-- check FILE` compares a measurement file with the possible orders of the arithmetic.)
//!
//! Each `MatrixMultiply` line: `alias<TAB>x87 control word<TAB>M1 (16 words)<TAB>M2 (16 words)
//! <TAB>result (16 words)<TAB>returned (1 when the answer is the out pointer)`. `alias` says
//! which pointers were the same: `none`, `out=m1`, `out=m2`, `m1=m2`, `out=m1=m2` (the inputs
//! written are the values before the call).
mod analyze;
mod functions;
mod image;
mod texture;

use std::ffi::c_void;
use std::io::Write;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn GetModuleFileNameA(module: *mut c_void, buffer: *mut u8, size: u32) -> u32;
}

type MatrixMultiply = unsafe extern "system" fn(*mut f32, *const f32, *const f32) -> *mut f32;

/// xorshift64*: the inputs are the same on every run.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, n: u64) -> u64 {
        (self.next() >> 11) % n
    }
}

/// Values chosen for their edges: zeros, ones, infinities, NaNs (quiet, signalling, with
/// payloads), denormals, the smallest normal and the largest finite.
const SPECIAL: &[u32] = &[
    0x0000_0000, 0x8000_0000, 0x3f80_0000, 0xbf80_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_0000,
    0xffc0_0000, 0x7f80_0001, 0x7fa0_0000, 0x7fc1_2345, 0x0000_0001, 0x8000_0001, 0x007f_ffff,
    0x0040_0000, 0x0080_0000, 0x8080_0000, 0x7f7f_ffff, 0xff7f_ffff, 0x3400_0000, 0x4b80_0000,
    0x1e3c_e508, 0x5f80_0000, 0x2000_0000, 0x3eaa_aaab, 0x4049_0fdb,
];

/// One input word by class: 0 uniform in [-1, 1] scaled by 10^-3..10^3, 1 any bit pattern,
/// 2 a special value, 3 a wide exponent spread (2^-60..2^60, either sign), 4 small integers.
fn value(rng: &mut Rng, class: u64) -> u32 {
    match class {
        0 => {
            let unit = (rng.next() >> 40) as f32 / (1u64 << 24) as f32 * 2.0 - 1.0;
            let scale = 10f32.powi(rng.below(7) as i32 - 3);
            (unit * scale).to_bits()
        }
        1 => rng.next() as u32,
        2 => SPECIAL[rng.below(SPECIAL.len() as u64) as usize],
        3 => {
            let mantissa = (rng.next() as u32) & 0x007f_ffff;
            let exponent = (127 - 60 + rng.below(121) as u32) << 23;
            let sign = ((rng.next() & 1) as u32) << 31;
            sign | exponent | mantissa
        }
        _ => (rng.below(9) as f32 - 4.0).to_bits(),
    }
}

/// A matrix of one class, or (class 5) each word of a random class.
fn matrix(rng: &mut Rng, class: u64) -> [u32; 16] {
    let mut m = [0u32; 16];
    for w in m.iter_mut() {
        let c = if class == 5 { rng.below(5) } else { class };
        *w = value(rng, c);
    }
    m
}

unsafe fn set_x87_control_word(cw: u16) {
    std::arch::asm!("fldcw [{0}]", in(reg) &cw);
}

unsafe fn x87_control_word() -> u16 {
    let mut cw = 0u16;
    std::arch::asm!("fnstcw [{0}]", in(reg) &mut cw);
    cw
}

unsafe fn mxcsr() -> u32 {
    let mut m = 0u32;
    std::arch::asm!("stmxcsr [{0}]", in(reg) &mut m);
    m
}

fn words(m: &[u32]) -> String {
    m.iter().map(|w| format!("{w:08x}")).collect::<Vec<_>>().join(" ")
}

/// The CPU's vendor and brand strings (CPUID 0 and 0x80000002..4).
#[allow(unused_unsafe)] // `__cpuid` is unsafe on older toolchains.
fn cpu() -> (String, String) {
    use std::arch::x86::__cpuid;
    unsafe {
        let r = __cpuid(0);
        let vendor: Vec<u8> = [r.ebx, r.edx, r.ecx].iter().flat_map(|w| w.to_le_bytes()).collect();
        let mut brand = Vec::new();
        for leaf in 0x8000_0002u32..=0x8000_0004 {
            let r = __cpuid(leaf);
            for w in [r.eax, r.ebx, r.ecx, r.edx] {
                brand.extend_from_slice(&w.to_le_bytes());
            }
        }
        let text = |b: &[u8]| String::from_utf8_lossy(b).trim_end_matches('\0').trim().to_string();
        (text(&vendor), text(&brand))
    }
}

unsafe fn matrix_multiply(o: &mut impl Write, f: MatrixMultiply, count: usize) {
    let mut rng = Rng(0x6e76_2d72_7364_3378);
    let aliases = ["none", "out=m1", "out=m2", "m1=m2", "out=m1=m2"];
    for &cw in &[0x027fu16, 0x007f] {
        let mut cases: Vec<([u32; 16], [u32; 16])> = Vec::new();
        // Identity and a few fixed matrices first, then random ones of each class.
        let mut identity = [0u32; 16];
        for i in 0..4 {
            identity[i * 5] = 0x3f80_0000;
        }
        cases.push((identity, identity));
        for i in 0..count {
            let class_a = (i as u64) % 6;
            let class_b = (i as u64 / 6) % 6;
            cases.push((matrix(&mut rng, class_a), matrix(&mut rng, class_b)));
        }
        for (a, b) in &cases {
            for alias in aliases {
                let mut out = [0x5555_5555u32; 16];
                let mut ma = *a;
                let mut mb = if alias == "m1=m2" || alias == "out=m1=m2" { *a } else { *b };
                let (po, p1, p2): (*mut f32, *const f32, *const f32) = match alias {
                    "none" => (out.as_mut_ptr().cast(), ma.as_ptr().cast(), mb.as_ptr().cast()),
                    "out=m1" => (ma.as_mut_ptr().cast(), ma.as_ptr().cast(), mb.as_ptr().cast()),
                    "out=m2" => (mb.as_mut_ptr().cast(), ma.as_ptr().cast(), mb.as_ptr().cast()),
                    "m1=m2" => (out.as_mut_ptr().cast(), ma.as_ptr().cast(), ma.as_ptr().cast()),
                    _ => (ma.as_mut_ptr().cast(), ma.as_ptr().cast(), ma.as_ptr().cast()),
                };
                let in_a = ma;
                let in_b = mb;
                set_x87_control_word(cw);
                let ret = f(po, p1, p2);
                set_x87_control_word(0x027f);
                let result: [u32; 16] = std::ptr::read(po.cast());
                writeln!(
                    o,
                    "MatrixMultiply\t{alias}\t{cw:04x}\t{}\t{}\t{}\t{}",
                    words(&in_a),
                    words(if alias == "m1=m2" || alias == "out=m1=m2" { &in_a } else { &in_b }),
                    words(&result),
                    u32::from(ret == po)
                )
                .unwrap();
                let _ = &mut mb;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let what = args.get(1).map(String::as_str).unwrap_or("matrix-multiply");
    if what == "check" {
        analyze::check(&args[2]);
        return;
    }
    if what == "nan-positions" {
        analyze::nan_positions(&args[2]);
        return;
    }
    if what == "rows" {
        analyze::rows(&args[2]);
        return;
    }
    if what == "positions" {
        analyze::positions(&args[2]);
        return;
    }
    let count: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(4000);
    if what == "serve" {
        unsafe {
            let module = LoadLibraryA(b"d3dx9_38.dll\0".as_ptr());
            assert!(!module.is_null(), "d3dx9_38.dll did not load");
            texture::serve(module);
        }
        return;
    }
    let out = std::io::stdout();
    let mut o = out.lock();
    unsafe {
        let module = LoadLibraryA(b"d3dx9_38.dll\0".as_ptr());
        assert!(!module.is_null(), "d3dx9_38.dll did not load");
        let mut path = vec![0u8; 260];
        let n = GetModuleFileNameA(module, path.as_mut_ptr(), 260) as usize;
        let (vendor, brand) = cpu();
        writeln!(o, "Dll\t{}", String::from_utf8_lossy(&path[..n])).unwrap();
        writeln!(o, "Cpu\t{vendor}\t{brand}").unwrap();
        writeln!(o, "Start\tx87 {:04x}\tmxcsr {:08x}", x87_control_word(), mxcsr()).unwrap();
        match what {
            "matrix-multiply" => {
                let p = GetProcAddress(module, b"D3DXMatrixMultiply\0".as_ptr());
                assert!(!p.is_null());
                matrix_multiply(&mut o, std::mem::transmute(p), count);
            }
            "image-info" => image::image_info(&mut o, module, &args[2]),
            "create-texture" => texture::create_texture(&mut o, module, &args[2]),
            "functions-edges" => {
                let name = args.get(2).map(String::as_str).unwrap_or("all");
                functions::edges(&mut o, module, name);
            }
            "functions" => {
                let name = args.get(3).map(String::as_str).unwrap_or("all");
                functions::measure(&mut o, module, name, count);
            }
            other => panic!("unknown measurement {other}"),
        }
        writeln!(o, "End\tx87 {:04x}\tmxcsr {:08x}", x87_control_word(), mxcsr()).unwrap();
    }
}
