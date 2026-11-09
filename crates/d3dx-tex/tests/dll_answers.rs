//! The crate against the 32-bit `d3dx9_38.dll`'s answers, captured once on Windows with
//! `scripts/d3dx-tex-verify` (`--capture-blocks`, `--capture-synthetic`; the DLL called through the
//! i686 `scripts/d3dx-measure`). No game files and no DLL are needed here.

/// `data/blocks_32bit.tsv`: blocks of varied pixels through the DLL's own DXT1, DXT3 and DXT5
/// encoders (called inside the DLL with the x87 control word of the thread that made the device,
/// 0x7f), without and with dither: ours are byte for byte the same.
#[test]
fn dxt_blocks_are_the_32_bit_dlls() {
    blocks(
        include_str!("../data/blocks_32bit.tsv"),
        d3dx_tex::Precision::Single,
        24,
    );
}

/// `data/blocks_32bit_double.tsv`: the same at 0x27f (a loading thread), blocks the DLL encodes
/// differently at the two precisions: for each encoder, without and with dither, at least 8, so
/// what the DLL keeps in registers and what it stores is pinned here, not only by the game's
/// textures.
#[test]
fn dxt_blocks_are_the_32_bit_dlls_at_double_precision() {
    blocks(
        include_str!("../data/blocks_32bit_double.tsv"),
        d3dx_tex::Precision::Double,
        38,
    );
}

fn blocks(data: &str, precision: d3dx_tex::Precision, count: usize) {
    let mut n = 0;
    for line in data.lines().filter(|l| !l.starts_with('#')) {
        let mut f = line.split('\t');
        let words: Vec<u32> = f
            .next()
            .unwrap()
            .split(' ')
            .map(|w| u32::from_str_radix(w, 16).unwrap())
            .collect();
        let px: [[f32; 4]; 16] =
            std::array::from_fn(|i| std::array::from_fn(|c| f32::from_bits(words[i * 4 + c])));
        for format in [
            d3dx_tex::format::DXT1,
            d3dx_tex::format::DXT3,
            d3dx_tex::format::DXT5,
        ] {
            for dither in [false, true] {
                let theirs = f.next().unwrap();
                let ours: String = d3dx_tex::filter::encode_block(format, &px, dither, precision)
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect();
                assert_eq!(ours, theirs, "{format:#x} dither {dither}: {line}");
                n += 1;
            }
        }
    }
    assert_eq!(n, count * 6);
}

/// `data/synthetic_32bit.tsv`: the synthetic DDS files (`d3dx_tex::synthetic`) through the DLL's
/// `D3DXCreateTextureFromFileInMemory` / `D3DXCreateCubeTextureFromFileInMemory` on a real
/// device, each level read back: our answer, level and face counts, every surface's format, size
/// and bytes (their FNV-1a) are the DLL's, on the thread that made the device.
#[test]
fn synthetic_textures_are_the_32_bit_dlls() {
    synthetic(
        include_str!("../data/synthetic_32bit.tsv"),
        d3dx_tex::Precision::Single,
    );
}

/// `data/synthetic_32bit_double.tsv`: the same with the DLL on a thread at 53-bit precision
/// (`d3dx-measure serve thread`), as the game's loading threads call it.
#[test]
fn synthetic_textures_are_the_32_bit_dlls_at_double_precision() {
    synthetic(
        include_str!("../data/synthetic_32bit_double.tsv"),
        d3dx_tex::Precision::Double,
    );
}

fn synthetic(data: &str, precision: d3dx_tex::Precision) {
    let caps = d3dx_tex::Caps::default();
    let files = d3dx_tex::synthetic::files();
    let lines: Vec<&str> = data.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(lines.len(), files.len());
    for (file, line) in files.iter().zip(lines) {
        let f: Vec<&str> = line.split('\t').collect();
        assert_eq!(f[0], file.name);
        let hr = u32::from_str_radix(f[2], 16).unwrap();
        let made = if file.cube {
            d3dx_tex::create_cube_texture_from_file_in_memory(&file.data, &caps, precision)
        } else {
            d3dx_tex::create_texture_from_file_in_memory(&file.data, &caps, precision)
        };
        match made {
            Err(d3dx_tex::Error::D3dx(ours)) => assert_eq!(ours, hr, "{}", file.name),
            Err(d3dx_tex::Error::NotProvided(why)) => panic!("{}: not provided: {why}", file.name),
            Ok(t) => {
                assert_eq!(hr, 0, "{}", file.name);
                assert_eq!(
                    (t.levels.to_string(), t.faces.to_string()),
                    (f[3].to_string(), f[4].to_string()),
                    "{}",
                    file.name
                );
                let ours: Vec<String> = t
                    .surfaces
                    .iter()
                    .map(|s| {
                        format!(
                            "{:x},{},{},{:016x}",
                            s.format,
                            s.width,
                            s.height,
                            d3dx_tex::synthetic::fnv(&s.bytes)
                        )
                    })
                    .collect();
                assert_eq!(ours.join(";"), f[5], "{}", file.name);
            }
        }
    }
}

/// The image info of each synthetic file D3DX reads: the header's size, level count and format,
/// the resource type, DDS.
#[test]
fn image_info_follows_the_header() {
    for file in d3dx_tex::synthetic::files() {
        match d3dx_tex::get_image_info_from_file_in_memory(&file.data) {
            Ok(info) => {
                let w = |at: usize| u32::from_le_bytes(file.data[at..at + 4].try_into().unwrap());
                assert_eq!(
                    (info.width, info.height, info.depth),
                    (w(16), w(12), 1),
                    "{}",
                    file.name
                );
                assert_eq!(info.mip_levels, w(28).max(1), "{}", file.name);
                assert_eq!(info.image_file_format, 4);
                assert_eq!(
                    info.resource_type,
                    if w(112) & 0xfc00 == 0xfc00 { 5 } else { 3 },
                    "{}",
                    file.name
                );
            }
            Err(e) => assert_eq!(
                e,
                d3dx_tex::Error::D3dx(d3dx_tex::D3DXERR_INVALIDDATA),
                "{}",
                file.name
            ),
        }
    }
}
