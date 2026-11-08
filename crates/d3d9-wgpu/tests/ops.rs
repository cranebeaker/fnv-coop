//! Each shader instruction the game's packages use, run on the GPU through the whole backend
//! (bytecode, converter, constants, draw, read-back) and compared with the meaning the DirectX 9
//! SDK's instruction reference gives it, Direct3D 9's `0 * anything = 0` included.
//!
//! The shaders are built here from tokens (no game file). A machine without a GPU adapter skips
//! with a message: these need one.

use d3d9_stream::{Backend, Cmd, Data};
use d3d9_wgpu::{Output, WgpuBackend};

const END: u32 = 0x0000_FFFF;
const INF: f32 = f32::INFINITY;

fn ins(op: u32, params: &[u32]) -> Vec<u32> {
    let mut v = vec![op | ((params.len() as u32) << 24)];
    v.extend_from_slice(params);
    v
}

/// `r0` as a destination with `mask`.
fn r0(mask: u32) -> u32 {
    0x8000_0000 | (mask << 16)
}
/// `cN.xyzw`.
fn c(n: u32) -> u32 {
    0xA0E4_0000 | n
}
/// `cN` with a replicate swizzle of `component`.
fn cs(n: u32, component: u32) -> u32 {
    let sw = component * 0x55;
    0xA000_0000 | (sw << 16) | n
}
fn r0_src() -> u32 {
    0x80E4_0000
}
/// `mov oC0, r0`.
fn out() -> Vec<u32> {
    ins(1, &[0x800F_0800, r0_src()])
}

/// The vertex shader: `dcl_position v0; mov oPos, v0`.
fn vertex_shader() -> Vec<u32> {
    let mut v = vec![0xFFFE_0200];
    v.extend(ins(31, &[0x8000_0000, 0x900F_0000]));
    v.extend(ins(1, &[0xC00F_0000, 0x90E4_0000]));
    v.push(END);
    v
}

fn words(w: &[u32]) -> Vec<u8> {
    w.iter().flat_map(|w| w.to_le_bytes()).collect()
}

struct Rig {
    b: WgpuBackend,
}

const DEVICE: u32 = 0x1000;
const BACK: u32 = 0x1100;
const RT: u32 = 0x1200;
const VS: u32 = 0x1300;
const DECL: u32 = 0x1400;
const VB: u32 = 0x1500;

impl Rig {
    fn new() -> Option<Rig> {
        let mut b = WgpuBackend::new(Output::Offscreen);
        // D3DPRESENT_PARAMETERS: 1x1, A32B32G32R32F, one back buffer, no depth.
        let present = words(&[1, 1, 116, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0]);
        let back = words(&[BACK]);
        b.execute(Cmd::CreateDevice {
            device: DEVICE,
            focus: 0,
            window: 0,
            behavior: 0x54,
            present: Data::Bytes(&present),
            back_buffers: Data::Bytes(&back),
            depth_stencil: 0,
        });
        if b.device().is_none() {
            eprintln!("no GPU adapter: the instruction tests are skipped");
            return None;
        }
        let mut r = Rig { b };
        let run = |r: &mut Rig, cmd| {
            r.b.execute(cmd);
        };
        run(
            &mut r,
            Cmd::CreateSurface {
                device: DEVICE,
                surface: RT,
                depth: false,
                width: 1,
                height: 1,
                format: 116,
                multisample: 0,
                quality: 0,
                flag: 0,
            },
        );
        let vs = words(&vertex_shader());
        run(
            &mut r,
            Cmd::CreateShader {
                device: DEVICE,
                shader: VS,
                pixel: false,
                tokens: Data::Bytes(&vs),
            },
        );
        // FLOAT4 position in stream 0, then D3DDECL_END.
        let decl = [0u8, 0, 0, 0, 3, 0, 0, 0, 0xFF, 0, 0, 0, 17, 0, 0, 0];
        run(
            &mut r,
            Cmd::CreateDeclaration {
                device: DEVICE,
                declaration: DECL,
                elements: Data::Bytes(&decl),
            },
        );
        let tri: Vec<u8> = [
            -1.0f32, -1.0, 0.0, 1.0, -1.0, 3.0, 0.0, 1.0, 3.0, -1.0, 0.0, 1.0,
        ]
        .iter()
        .flat_map(|f| f.to_le_bytes())
        .collect();
        run(
            &mut r,
            Cmd::CreateBuffer {
                device: DEVICE,
                buffer: VB,
                index: false,
                length: 48,
                usage: 0,
                format: 0,
                pool: 1,
            },
        );
        run(
            &mut r,
            Cmd::UploadBuffer {
                buffer: VB,
                offset: 0,
                flags: 0,
                data: Data::Bytes(&tri),
            },
        );
        run(
            &mut r,
            Cmd::SetRenderTarget {
                device: DEVICE,
                index: 0,
                surface: RT,
            },
        );
        run(
            &mut r,
            Cmd::SetShader {
                device: DEVICE,
                pixel: false,
                shader: VS,
            },
        );
        run(
            &mut r,
            Cmd::SetVertexDeclaration {
                device: DEVICE,
                declaration: DECL,
            },
        );
        run(
            &mut r,
            Cmd::SetStreamSource {
                device: DEVICE,
                stream: 0,
                buffer: VB,
                offset: 0,
                stride: 16,
            },
        );
        // D3DRS_CULLMODE = D3DCULL_NONE.
        run(
            &mut r,
            Cmd::SetRenderState {
                device: DEVICE,
                state: 22,
                value: 1,
            },
        );
        Some(r)
    }

    /// Runs `body` (ps_3_0 instructions writing r0) with c0..c2 set, and reads oC0.
    fn run(&mut self, body: &[u32], constants: [[f32; 4]; 3]) -> [f32; 4] {
        static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0x2000);
        let ps = NEXT.fetch_add(4, std::sync::atomic::Ordering::Relaxed);
        let mut tokens = vec![0xFFFF_0300];
        // r0 starts at 0 (the converter's private registers are zeroed).
        tokens.extend_from_slice(body);
        tokens.extend(out());
        tokens.push(END);
        let bytes = words(&tokens);
        self.b.execute(Cmd::CreateShader {
            device: DEVICE,
            shader: ps,
            pixel: true,
            tokens: Data::Bytes(&bytes),
        });
        self.b.execute(Cmd::SetShader {
            device: DEVICE,
            pixel: true,
            shader: ps,
        });
        let regs: Vec<u8> = constants
            .iter()
            .flatten()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        self.b.execute(Cmd::SetConstants {
            device: DEVICE,
            kind: 3,
            start: 0,
            count: 3,
            data: Data::Bytes(&regs),
        });
        self.b.execute(Cmd::DrawPrimitive {
            device: DEVICE,
            kind: 4,
            start: 0,
            count: 1,
        });
        let d = self.b.device().expect("the device");
        let (_, _, px) = d.read_surface_pixels(RT).expect("the target's pixels");
        std::array::from_fn(|i| f32::from_le_bytes(px[4 * i..4 * i + 4].try_into().unwrap()))
    }
}

fn same(a: [f32; 4], b: [f32; 4]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| {
        (x.is_nan() && y.is_nan()) || x == y || (x - y).abs() <= 1e-5 * y.abs().max(1.0)
    })
}

macro_rules! check {
    ($rig:expr, $name:expr, $body:expr, $c:expr, $want:expr) => {{
        let got = $rig.run(&$body, $c);
        assert!(
            same(got, $want),
            "{}: got {:?}, want {:?}",
            $name,
            got,
            $want
        );
    }};
}

const Z: [f32; 4] = [0.0; 4];

#[test]
fn each_instruction_means_what_direct3d_9_says() {
    let Some(mut r) = Rig::new() else { return };
    let full = 0xF;
    // Arithmetic, with 0 * inf = 0 wherever the instruction multiplies.
    check!(
        r,
        "mul",
        ins(5, &[r0(full), c(0), c(1)]),
        [[0.0, INF, 2.0, -0.0], [INF, 0.0, 3.0, f32::NAN], Z],
        [0.0, 0.0, 6.0, 0.0]
    );
    check!(
        r,
        "mad",
        ins(4, &[r0(full), c(0), c(1), c(2)]),
        [[0.0, 2.0, 0.0, 1.0], [INF, 3.0, f32::NAN, 1.0], [1.0; 4]],
        [1.0, 7.0, 1.0, 2.0]
    );
    check!(
        r,
        "add",
        ins(2, &[r0(full), c(0), c(1)]),
        [[1.0, 2.0, 3.0, 4.0], [0.5, -2.0, 1.0, -8.0], Z],
        [1.5, 0.0, 4.0, -4.0]
    );
    check!(
        r,
        "dp3",
        ins(8, &[r0(full), c(0), c(1)]),
        [[0.0, 1.0, 2.0, 99.0], [INF, 1.0, 1.0, 5.0], Z],
        [3.0; 4]
    );
    check!(
        r,
        "dp4",
        ins(9, &[r0(full), c(0), c(1)]),
        [[0.0, 1.0, 2.0, 3.0], [INF, 1.0, 1.0, 2.0], Z],
        [9.0; 4]
    );
    check!(
        r,
        "dp2add",
        ins(90, &[r0(full), c(0), c(1), cs(2, 0)]),
        [
            [1.0, 2.0, 7.0, 7.0],
            [3.0, 4.0, 7.0, 7.0],
            [5.0, 0.0, 0.0, 0.0]
        ],
        [16.0; 4]
    );
    check!(
        r,
        "min",
        ins(10, &[r0(full), c(0), c(1)]),
        [[1.0, -2.0, 3.0, -0.5], [2.0, -1.0, -3.0, 0.5], Z],
        [1.0, -2.0, -3.0, -0.5]
    );
    check!(
        r,
        "max",
        ins(11, &[r0(full), c(0), c(1)]),
        [[1.0, -2.0, 3.0, -0.5], [2.0, -1.0, -3.0, 0.5], Z],
        [2.0, -1.0, 3.0, 0.5]
    );
    check!(
        r,
        "abs",
        ins(35, &[r0(full), c(0)]),
        [[-1.0, 2.0, -0.0, -INF], Z, Z],
        [1.0, 2.0, 0.0, INF]
    );
    check!(
        r,
        "frc",
        ins(19, &[r0(full), c(0)]),
        [[1.25, -1.25, 0.0, 2.0], Z, Z],
        [0.25, 0.75, 0.0, 0.0]
    );
    // Scalar instructions read the replicated component and write it to every component.
    check!(
        r,
        "rcp",
        ins(6, &[r0(full), cs(0, 0)]),
        [[4.0, 0.0, 0.0, 0.0], Z, Z],
        [0.25; 4]
    );
    check!(
        r,
        "rcp of 1",
        ins(6, &[r0(full), cs(0, 1)]),
        [[4.0, 1.0, 0.0, 0.0], Z, Z],
        [1.0; 4]
    );
    check!(
        r,
        "rcp of 0",
        ins(6, &[r0(full), cs(0, 2)]),
        [[4.0, 1.0, 0.0, 0.0], Z, Z],
        [INF; 4]
    );
    check!(
        r,
        "rsq of a negative",
        ins(7, &[r0(full), cs(0, 0)]),
        [[-4.0, 0.0, 0.0, 0.0], Z, Z],
        [0.5; 4]
    );
    check!(
        r,
        "rsq of 0",
        ins(7, &[r0(full), cs(0, 1)]),
        [[-4.0, 0.0, 0.0, 0.0], Z, Z],
        [INF; 4]
    );
    check!(
        r,
        "exp",
        ins(14, &[r0(full), cs(0, 0)]),
        [[3.0, 0.0, 0.0, 0.0], Z, Z],
        [8.0; 4]
    );
    check!(
        r,
        "log of a negative",
        ins(15, &[r0(full), cs(0, 0)]),
        [[-8.0, 0.0, 0.0, 0.0], Z, Z],
        [3.0; 4]
    );
    check!(
        r,
        "log of 0",
        ins(15, &[r0(full), cs(0, 1)]),
        [[-8.0, 0.0, 0.0, 0.0], Z, Z],
        [-INF; 4]
    );
    check!(
        r,
        "pow of a negative base",
        ins(32, &[r0(full), cs(0, 0), cs(1, 0)]),
        [[-2.0, 0.0, 0.0, 0.0], [2.0, 0.0, 0.0, 0.0], Z],
        [4.0; 4]
    );
    check!(
        r,
        "pow(0, 0)",
        ins(32, &[r0(full), cs(0, 0), cs(1, 0)]),
        [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], Z],
        [1.0; 4]
    );
    // Selection.
    check!(
        r,
        "cmp",
        ins(88, &[r0(full), c(0), c(1), c(2)]),
        [[0.0, -0.0, -1.0, 1.0], [10.0; 4], [20.0; 4]],
        [10.0, 10.0, 20.0, 10.0]
    );
    check!(
        r,
        "lrp",
        ins(18, &[r0(full), c(0), c(1), c(2)]),
        [
            [0.5, 0.0, 1.0, 0.25],
            [4.0, INF, 3.0, 8.0],
            [2.0, 1.0, 1.0, 0.0]
        ],
        [3.0, 1.0, 3.0, 2.0]
    );
    // Vectors.
    check!(
        r,
        "nrm",
        ins(36, &[r0(full), c(0)]),
        [[3.0, 4.0, 0.0, 1.0], Z, Z],
        [0.6, 0.8, 0.0, 0.2]
    );
    check!(
        r,
        "nrm of zero",
        ins(36, &[r0(full), c(0)]),
        [Z, Z, Z],
        [0.0; 4]
    );
    check!(
        r,
        "crs",
        ins(33, &[r0(0x7), c(0), c(1)]),
        [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], Z],
        [0.0, 0.0, 1.0, 0.0]
    );
    check!(
        r,
        "sincos",
        ins(37, &[r0(0x3), cs(0, 0)]),
        [[0.0, 0.0, 0.0, 0.0], Z, Z],
        [1.0, 0.0, 0.0, 0.0]
    );
    // Write masks, saturation and source modifiers.
    check!(
        r,
        "a masked mov",
        ins(1, &[r0(0x5), c(0)]),
        [[1.0, 2.0, 3.0, 4.0], Z, Z],
        [1.0, 0.0, 3.0, 0.0]
    );
    check!(
        r,
        "mov_sat",
        ins(1, &[r0(full) | (1 << 20), c(0)]),
        [[-1.0, 0.5, 2.0, f32::NAN], Z, Z],
        [0.0, 0.5, 1.0, 0.0]
    );
    check!(
        r,
        "a negated, swizzled source",
        ins(
            1,
            &[r0(full), (c(0) & !0x00FF_0000) | (0x1B << 16) | (1 << 24)]
        ),
        [[1.0, 2.0, 3.0, 4.0], Z, Z],
        [-4.0, -3.0, -2.0, -1.0]
    );
    check!(
        r,
        "an absolute source",
        ins(1, &[r0(full), c(0) | (11 << 24)]),
        [[-1.0, 2.0, -3.0, 0.0], Z, Z],
        [1.0, 2.0, 3.0, 0.0]
    );
}

/// A 4x4 texture drawn onto a 4x4 target the way Direct3D 9 programs do it (the quad from -0.5
/// to 3.5 in pixels, texture coordinates 0 to 1), point-sampled: each pixel gets exactly its
/// texel only if pixel centres are where Direct3D 9 puts them (the half-pixel move of the
/// converted vertex shaders).
#[test]
fn a_texel_aligned_quad_copies_its_texture_exactly() {
    let Some(mut r) = Rig::new() else { return };
    const VS2: u32 = 0x3000;
    const PS2: u32 = 0x3004;
    const DECL2: u32 = 0x3008;
    const VB2: u32 = 0x300c;
    const TEX: u32 = 0x3010;
    const LEVEL: u32 = 0x3014;
    const RT2: u32 = 0x3018;
    let vs = words(&[
        0xFFFE_0200,
        0x0200_001F,
        0x8000_0000,
        0x900F_0000,
        0x0200_001F,
        0x8000_0005,
        0x900F_0001,
        0x0200_0001,
        0xC00F_0000,
        0x90E4_0000,
        0x0200_0001,
        0xE00F_0000,
        0x90E4_0001,
        END,
    ]);
    let ps = words(&[
        0xFFFF_0200,
        0x0200_001F,
        0x8000_0000,
        0xB003_0000,
        0x0200_001F,
        0x9000_0000,
        0xA00F_0800,
        0x0300_0042,
        0x800F_0000,
        0xB0E4_0000,
        0xA0E4_0800,
        0x0200_0001,
        0x800F_0800,
        0x80E4_0000,
        END,
    ]);
    let decl = [
        0u8, 0, 0, 0, 3, 0, 0, 0, // position, FLOAT4
        0, 0, 16, 0, 1, 0, 5, 0, // texcoord0, FLOAT2
        0xFF, 0, 0, 0, 17, 0, 0, 0,
    ];
    let quad: Vec<u8> = [
        [-1.25f32, 1.25, 0.0, 1.0, 0.0, 0.0],
        [0.75, 1.25, 0.0, 1.0, 1.0, 0.0],
        [-1.25, -0.75, 0.0, 1.0, 0.0, 1.0],
        [0.75, -0.75, 0.0, 1.0, 1.0, 1.0],
    ]
    .iter()
    .flatten()
    .flat_map(|f| f.to_le_bytes())
    .collect();
    let texels: Vec<u8> = (0..16u32)
        .flat_map(|i| [(i * 16) as u8, (255 - i * 16) as u8, (i * 7) as u8, 255])
        .collect();
    let levels = words(&[LEVEL]);
    let b = &mut r.b;
    b.execute(Cmd::CreateShader {
        device: DEVICE,
        shader: VS2,
        pixel: false,
        tokens: Data::Bytes(&vs),
    });
    b.execute(Cmd::CreateShader {
        device: DEVICE,
        shader: PS2,
        pixel: true,
        tokens: Data::Bytes(&ps),
    });
    b.execute(Cmd::CreateDeclaration {
        device: DEVICE,
        declaration: DECL2,
        elements: Data::Bytes(&decl),
    });
    b.execute(Cmd::CreateBuffer {
        device: DEVICE,
        buffer: VB2,
        index: false,
        length: 96,
        usage: 0,
        format: 0,
        pool: 1,
    });
    b.execute(Cmd::UploadBuffer {
        buffer: VB2,
        offset: 0,
        flags: 0,
        data: Data::Bytes(&quad),
    });
    b.execute(Cmd::CreateTexture {
        device: DEVICE,
        texture: TEX,
        levels: Data::Bytes(&levels),
        width: 4,
        height: 4,
        usage: 0,
        format: 21,
        pool: 1,
    });
    b.execute(Cmd::UploadSurface {
        surface: LEVEL,
        row: 16,
        rows: 4,
        data: Data::Bytes(&texels),
    });
    b.execute(Cmd::CreateSurface {
        device: DEVICE,
        surface: RT2,
        depth: false,
        width: 4,
        height: 4,
        format: 21,
        multisample: 0,
        quality: 0,
        flag: 0,
    });
    b.execute(Cmd::SetRenderTarget {
        device: DEVICE,
        index: 0,
        surface: RT2,
    });
    b.execute(Cmd::SetShader {
        device: DEVICE,
        pixel: false,
        shader: VS2,
    });
    b.execute(Cmd::SetShader {
        device: DEVICE,
        pixel: true,
        shader: PS2,
    });
    b.execute(Cmd::SetVertexDeclaration {
        device: DEVICE,
        declaration: DECL2,
    });
    b.execute(Cmd::SetStreamSource {
        device: DEVICE,
        stream: 0,
        buffer: VB2,
        offset: 0,
        stride: 24,
    });
    b.execute(Cmd::SetTexture {
        device: DEVICE,
        sampler: 0,
        texture: TEX,
    });
    b.execute(Cmd::DrawPrimitive {
        device: DEVICE,
        kind: 5,
        start: 0,
        count: 2,
    });
    let (_, _, px) = b.device().unwrap().read_surface_pixels(RT2).unwrap();
    assert_eq!(px, texels);
}
