//! Packages and shaders built token by token, then read back.

use shaders::{disassemble, Package, RegisterSet, ShaderKind};

/// A register token: type split across bits 28-30 and 11-12.
fn register(kind: u32, number: u32) -> u32 {
    0x8000_0000 | ((kind & 7) << 28) | ((kind & 0x18) << 8) | number
}

fn dest(kind: u32, number: u32, mask: u32) -> u32 {
    register(kind, number) | (mask << 16)
}

/// A source with a swizzle (2 bits per component) and modifier.
fn src(kind: u32, number: u32, swizzle: u32, modifier: u32) -> u32 {
    register(kind, number) | (swizzle << 16) | (modifier << 24)
}

const XYZW: u32 = 0xE4;
const XXXX: u32 = 0x00;

fn instruction(op: u32, params: &[u32]) -> Vec<u32> {
    let mut v = vec![op | ((params.len() as u32) << 24)];
    v.extend(params);
    v
}

/// A constant table comment naming c1, c2-c5 and s0.
fn constant_table() -> Vec<u32> {
    let mut b = Vec::new();
    let names = ["AmbientColor", "LightData", "BaseMap"];
    let header = 28;
    let info = header;
    let strings = info + 20 * names.len();
    let creator_at = strings;
    let mut text = b"test compiler\0".to_vec();
    let mut name_at = Vec::new();
    for n in names {
        name_at.push(strings + text.len());
        text.extend(n.as_bytes());
        text.push(0);
    }
    for v in [28u32, creator_at as u32, 0xFFFF_0300, 3, info as u32, 0, 0] {
        b.extend(v.to_le_bytes());
    }
    for (k, (set, index, count)) in [(2u16, 1u16, 1u16), (2, 2, 4), (3, 0, 1)]
        .into_iter()
        .enumerate()
    {
        b.extend((name_at[k] as u32).to_le_bytes());
        for v in [set, index, count, 0] {
            b.extend(v.to_le_bytes());
        }
        b.extend([0u8; 8]);
    }
    b.extend(text);
    while b.len() % 4 != 0 {
        b.push(0);
    }
    let mut tokens = vec![u32::from_le_bytes(*b"CTAB")];
    tokens.extend(
        b.chunks(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])),
    );
    let mut out = vec![0xFFFE | ((tokens.len() as u32) << 16)];
    out.extend(tokens);
    out
}

/// A small ps_3_0 shader: texture times a light, plus ambient.
fn pixel_shader() -> Vec<u8> {
    let mut t = vec![0xFFFF_0300];
    t.extend(constant_table());
    t.extend(instruction(
        81,
        &[
            dest(2, 0, 0xF),
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            0.5f32.to_bits(),
            0.0f32.to_bits(),
        ],
    ));
    t.extend(instruction(31, &[0x8000_0005, dest(1, 0, 0x3)]));
    t.extend(instruction(
        31,
        &[0x8000_0000 | (2 << 27), dest(10, 0, 0xF)],
    ));
    t.extend(instruction(
        66,
        &[dest(0, 0, 0xF), src(1, 0, XYZW, 0), src(10, 0, XYZW, 0)],
    ));
    t.extend(instruction(
        5,
        &[dest(0, 1, 0x7), src(0, 0, XYZW, 0), src(2, 3, XYZW, 0)],
    ));
    // mad_sat r1.xyz, r1, c0.x, -c1_abs  (saturate is bit 20)
    t.extend(instruction(
        4,
        &[
            dest(0, 1, 0x7) | (1 << 20),
            src(0, 1, XYZW, 0),
            src(2, 0, XXXX, 0),
            src(2, 1, XYZW, 12),
        ],
    ));
    t.extend(instruction(1, &[dest(8, 0, 0xF), src(0, 1, XYZW, 0)]));
    t.push(0x0000_FFFF);
    t.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn vertex_shader() -> Vec<u8> {
    let mut t = vec![0xFFFE_0200];
    // dp4 oPos.x, v0, c[a0.x + 4]: shader model 2 indexing adds a token.
    t.extend(instruction(
        9,
        &[
            dest(4, 0, 0x1),
            src(1, 0, XYZW, 0),
            src(2, 4, XYZW, 0) | (1 << 13),
            src(3, 0, XXXX, 0),
        ],
    ));
    t.push(0x0000_FFFF);
    t.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn package(shaders: &[(&str, Vec<u8>)]) -> Vec<u8> {
    let mut body = Vec::new();
    for (name, code) in shaders {
        let mut field = name.as_bytes().to_vec();
        field.resize(256, 0);
        body.extend(field);
        body.extend((code.len() as u32).to_le_bytes());
        body.extend(code);
    }
    let mut out = Vec::new();
    for v in [100u32, shaders.len() as u32, body.len() as u32] {
        out.extend(v.to_le_bytes());
    }
    out.extend(body);
    out
}

#[test]
fn reads_named_shaders_from_a_package() {
    let bytes = package(&[
        ("SLS2001.pso", pixel_shader()),
        ("SLS2001.vso", vertex_shader()),
    ]);
    let p = Package::parse(&bytes).unwrap();
    assert_eq!(p.shaders.len(), 2);
    assert!(p.layout.starts_with("header"));
    assert_eq!(p.shaders[0].name, "SLS2001.pso");
    assert_eq!(p.get("sls2001.VSO").unwrap().bytecode, vertex_shader());
    assert_eq!(p.shaders[0].offset, 12 + 256 + 4);
}

#[test]
fn finds_shaders_in_layouts_it_doesnt_know() {
    // A different header size: the index doesn't line up, so the shaders
    // are found by searching.
    let mut bytes = vec![7u8; 20];
    bytes.extend(&package(&[("A.pso", pixel_shader()), ("B.vso", vertex_shader())])[12..]);
    let p = Package::parse(&bytes).unwrap();
    assert!(p.layout.contains("searching"));
    let names: Vec<&str> = p.shaders.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["A.pso", "B.vso"]);
    assert!(Package::parse(b"not a package at all").is_err());
}

#[test]
fn disassembles_with_the_constant_names() {
    let d = disassemble(&pixel_shader()).unwrap();
    assert_eq!(d.version.to_string(), "ps_3_0");
    assert_eq!(d.version.kind, ShaderKind::Pixel);
    assert_eq!(d.creator.as_deref(), Some("test compiler"));
    assert_eq!(d.constants.len(), 3);
    assert_eq!(d.constants[1].registers(), "c2-c5");
    assert_eq!(
        d.constant_at(RegisterSet::Float4, 3).as_deref(),
        Some("LightData[1]")
    );
    assert_eq!(d.instructions, 7);
    let lines: Vec<&str> = d
        .text
        .lines()
        .skip_while(|l| *l != "ps_3_0")
        .skip(1)
        .map(str::trim_end)
        .collect();
    let expected = [
        "def c0, 1, 0, 0.5, 0",
        "dcl_texcoord v0.xy",
        "dcl_2d s0                                    // s0 = BaseMap",
        "texld r0, v0, s0                             // s0 = BaseMap",
        "mul r1.xyz, r0, c3                           // c3 = LightData[1]",
        "mad_sat r1.xyz, r1, c0.x, -c1_abs            // c1 = AmbientColor",
        "mov oC0, r1",
    ];
    assert_eq!(lines, expected, "{}", d.text);
    assert!(d.text.contains("//   LightData     c2-c5"), "{}", d.text);
}

#[test]
fn disassembles_indexed_constants() {
    let d = disassemble(&vertex_shader()).unwrap();
    assert_eq!(d.version.to_string(), "vs_2_0");
    assert!(d.text.contains("dp4 oPos.x, v0, c4[a0.x]"), "{}", d.text);
    assert!(disassemble(&[1, 2, 3, 4]).is_err());
}
