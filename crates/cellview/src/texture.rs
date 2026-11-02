//! Textures ready for the GPU: every mip level, kept block-compressed when
//! the graphics card can take it (all desktop cards can), so a scene's
//! textures use a quarter or less of the memory decoded ones would. A few
//! are made here instead, such as a diffuse texture multiplied by its glow
//! map, and go up as RGBA8.

use dds::{Dds, Format};

/// The colour of the texture the game binds as `FaceGenMap1` (`Decal2Map`
/// in its skin texture pass, `SLS1005.pso`) for every face and body: a
/// 32 × 32 A8R8G8B8 texture it makes at startup, every pixel red 62, green
/// 65, blue 62 (alpha 64). Read from the recording (`FalloutNVActors.trace`,
/// its contents at call 59178); its shaders multiply by 4 × it, so skin
/// comes out ×(0.973, 1.020, 0.973).
pub const FACEGEN_MAP1: [u8; 3] = [62, 65, 62];

/// Pixel formats a renderer needs to support. The color formats (BC1-3,
/// RGBA8) hold the game's stored (sRGB-encoded) colors, except in textures
/// marked `linear` (normal maps), which hold raw data; BC4 and BC5 always
/// hold raw data. The game samples all of them as stored (its samplers
/// never decode sRGB), and so does the viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuFormat {
    Bc1,
    Bc2,
    Bc3,
    Bc4,
    Bc5,
    Rgba8,
}

/// A texture with all its mip levels. Level data is produced on request, in
/// the form the graphics card accepts.
pub struct TextureData {
    /// Path the texture was loaded from (or what it was made from).
    pub path: String,
    pub width: u32,
    pub height: u32,
    /// Mip levels present.
    pub mip_levels: u32,
    /// Holds data rather than colors (a normal map). (Both kinds are
    /// sampled as stored; the flag keeps a file used both ways apart.)
    pub linear: bool,
    /// 6 for a cube map (faces +X, −X, +Y, −Y, +Z, −Z, each with its whole
    /// mip chain in [`TextureData::level_data`]), 1 otherwise.
    pub layers: u32,
    /// A cube map made from a flat picture: every face is that picture.
    repeat_face: bool,
    source: Source,
}

enum Source {
    Dds(Dds),
    /// RGBA8 levels, largest first.
    Rgba(Vec<Vec<u8>>),
}

impl TextureData {
    pub fn from_dds(path: impl Into<String>, bytes: Vec<u8>) -> Result<Self, dds::Error> {
        let dds = Dds::parse(bytes)?;
        // A full chain ends at 1x1; GPUs reject files that claim more.
        let full_chain = 32 - dds.width().max(dds.height()).max(1).leading_zeros();
        Ok(Self {
            path: path.into(),
            width: dds.width(),
            height: dds.height(),
            mip_levels: (dds.mip_count().max(1) as u32).min(full_chain),
            linear: false,
            layers: 1,
            repeat_face: false,
            source: Source::Dds(dds),
        })
    }

    /// The same texture as a cube map: the file's six faces when it has
    /// them, otherwise its (first) picture on every face, as the game does
    /// with its default reflection (`preview::cell::DEFAULT_ENVIRONMENT_MAP`).
    /// Cube faces must be square; anything else gives `None`.
    pub fn into_cube(mut self) -> Option<Self> {
        if self.width != self.height {
            return None;
        }
        let full_cube = matches!(&self.source, Source::Dds(dds) if dds.face_count() == 6);
        self.layers = 6;
        self.repeat_face = !full_cube;
        Some(self)
    }

    /// A texture from RGBA8 pixels (`width` x `height`, row by row), with a
    /// full chain of mip levels made from it.
    pub fn from_rgba(path: impl Into<String>, width: u32, height: u32, pixels: Vec<u8>) -> Self {
        assert_eq!(pixels.len(), width as usize * height as usize * 4);
        let mut levels = vec![pixels];
        let (mut w, mut h) = (width as usize, height as usize);
        while w > 1 || h > 1 {
            let next = half_size(levels.last().unwrap(), w, h);
            w = (w / 2).max(1);
            h = (h / 2).max(1);
            levels.push(next);
        }
        Self {
            path: path.into(),
            width,
            height,
            mip_levels: levels.len() as u32,
            linear: false,
            layers: 1,
            repeat_face: false,
            source: Source::Rgba(levels),
        }
    }

    /// The two textures multiplied together, channel by channel, at the size
    /// of the larger one; alpha comes from `self`. Colors multiply the same
    /// way before and after gamma encoding, so the product stays sRGB.
    pub fn times(&self, other: &TextureData) -> Result<Self, dds::Error> {
        let (width, height) = if self.width * self.height >= other.width * other.height {
            (self.width, self.height)
        } else {
            (other.width, other.height)
        };
        let a = self.top_level()?;
        let b = other.top_level()?;
        let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
        for y in 0..height {
            for x in 0..width {
                let u = (x as f32 + 0.5) / width as f32;
                let v = (y as f32 + 0.5) / height as f32;
                let pa = sample(&a, self.width, self.height, u, v);
                let pb = sample(&b, other.width, other.height, u, v);
                for k in 0..3 {
                    pixels.push((pa[k] * pb[k] / 255.0).round() as u8);
                }
                pixels.push(pa[3].round() as u8);
            }
        }
        Ok(Self::from_rgba(
            format!("{} x {}", self.path, other.path),
            width,
            height,
            pixels,
        ))
    }

    /// A skin texture with an NPC's FaceGen tint laid on, as the game's
    /// texture pass for skin does (`SLS1005.pso`, and `SKIN2000.pso` alike:
    /// `4 × FaceGenMap1 × (BaseMap + 2 × (FaceGenMap0 − 0.5))`), at the
    /// base's size; alpha from the base. `FaceGenMap0` is `tint` (a face's
    /// `facemods` file, or the body's `bodymods` one); `FaceGenMap1` is
    /// always [`FACEGEN_MAP1`].
    pub fn plus_face_tint(&self, tint: &TextureData) -> Result<Self, dds::Error> {
        let base = self.top_level()?;
        let t = tint.top_level()?;
        let mut pixels = Vec::with_capacity(base.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let u = (x as f32 + 0.5) / self.width as f32;
                let v = (y as f32 + 0.5) / self.height as f32;
                let i = ((y * self.width + x) * 4) as usize;
                let pt = sample(&t, tint.width, tint.height, u, v);
                for k in 0..3 {
                    let tinted = f32::from(base[i + k]) + 2.0 * (pt[k] - 127.5);
                    let value = tinted * 4.0 * f32::from(FACEGEN_MAP1[k]) / 255.0;
                    pixels.push(value.round().clamp(0.0, 255.0) as u8);
                }
                pixels.push(base[i + 3]);
            }
        }
        Ok(Self::from_rgba(
            format!("{} + FaceGen {}", self.path, tint.path),
            self.width,
            self.height,
            pixels,
        ))
    }

    /// A hair texture with its layer map laid over it, as the game's hair
    /// shader does before tinting (`lerp(BaseMap, LayerMap, LayerMap.a)`
    /// on stored values), at the base's size; alpha from the base.
    pub fn with_layer(&self, layer: &TextureData) -> Result<Self, dds::Error> {
        let base = self.top_level()?;
        let l = layer.top_level()?;
        let mut pixels = Vec::with_capacity(base.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let u = (x as f32 + 0.5) / self.width as f32;
                let v = (y as f32 + 0.5) / self.height as f32;
                let i = ((y * self.width + x) * 4) as usize;
                let pl = sample(&l, layer.width, layer.height, u, v);
                let a = pl[3] / 255.0;
                for k in 0..3 {
                    let value = f32::from(base[i + k]) * (1.0 - a) + pl[k] * a;
                    pixels.push(value.round().clamp(0.0, 255.0) as u8);
                }
                pixels.push(base[i + 3]);
            }
        }
        Ok(Self::from_rgba(
            format!("{} + layer {}", self.path, layer.path),
            self.width,
            self.height,
            pixels,
        ))
    }

    /// Whether the file has an alpha channel: DXT3/DXT5, or uncompressed
    /// with alpha bits. DXT1 counts as having none (its 1-bit alpha aside),
    /// as the game treats it: a normal map without alpha gets no specular
    /// (see `cellview::specular_allowed`). Textures made here have alpha.
    pub fn has_alpha_channel(&self) -> bool {
        match &self.source {
            Source::Dds(dds) => match dds.format() {
                Format::Bc2 { .. } | Format::Bc3 { .. } => true,
                Format::Uncompressed(m) => m.a != 0,
                Format::Bc1 | Format::Bc4 | Format::Bc5 => false,
            },
            Source::Rgba(_) => true,
        }
    }

    /// The largest level, as RGBA8.
    fn top_level(&self) -> Result<Vec<u8>, dds::Error> {
        match &self.source {
            Source::Dds(dds) => dds.decode_rgba(0, 0),
            Source::Rgba(levels) => Ok(levels[0].clone()),
        }
    }

    /// The format this texture can be uploaded in: its own block
    /// compression when `compressed_supported` and the size allows it
    /// (block-compressed textures must be a multiple of 4 wide and high),
    /// otherwise RGBA8.
    pub fn gpu_format(&self, compressed_supported: bool) -> GpuFormat {
        let blocks_fit = self.width % 4 == 0 && self.height % 4 == 0;
        let dds = match &self.source {
            Source::Dds(dds) if compressed_supported && blocks_fit => dds,
            _ => return GpuFormat::Rgba8,
        };
        match dds.format() {
            Format::Bc1 => GpuFormat::Bc1,
            // Premultiplied variants (DXT2/DXT4) are rare; decoding handles them.
            Format::Bc2 {
                premultiplied: false,
            } => GpuFormat::Bc2,
            Format::Bc3 {
                premultiplied: false,
            } => GpuFormat::Bc3,
            Format::Bc4 => GpuFormat::Bc4,
            Format::Bc5 => GpuFormat::Bc5,
            _ => GpuFormat::Rgba8,
        }
    }

    /// Every mip level of each layer (one, or a cube map's six faces),
    /// largest first, concatenated layer by layer, in `format` (from
    /// [`TextureData::gpu_format`]).
    pub fn level_data(&self, format: GpuFormat) -> Result<Vec<u8>, dds::Error> {
        let mut out = Vec::new();
        for layer in 0..self.layers as usize {
            let face = if self.repeat_face { 0 } else { layer };
            match &self.source {
                Source::Rgba(levels) => out.extend(levels.concat()),
                Source::Dds(dds) => {
                    for level in 0..self.mip_levels as usize {
                        if format == GpuFormat::Rgba8 {
                            out.extend(dds.decode_rgba(face, level)?);
                        } else {
                            out.extend_from_slice(dds.level_data(face, level)?);
                        }
                    }
                }
            }
        }
        Ok(out)
    }
}

/// The next mip level down: each pixel averages (up to) four.
fn half_size(pixels: &[u8], w: usize, h: usize) -> Vec<u8> {
    let (nw, nh) = ((w / 2).max(1), (h / 2).max(1));
    let mut out = Vec::with_capacity(nw * nh * 4);
    for y in 0..nh {
        for x in 0..nw {
            for k in 0..4 {
                let mut sum = 0u32;
                for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let sx = (x * 2 + dx).min(w - 1);
                    let sy = (y * 2 + dy).min(h - 1);
                    sum += u32::from(pixels[(sy * w + sx) * 4 + k]);
                }
                out.push(((sum + 2) / 4) as u8);
            }
        }
    }
    out
}

/// Bilinear sample of RGBA8 pixels at texture coordinates (wrapping), as
/// floats 0..255.
fn sample(pixels: &[u8], w: u32, h: u32, u: f32, v: f32) -> [f32; 4] {
    let (w, h) = (w as i64, h as i64);
    let x = u * w as f32 - 0.5;
    let y = v * h as f32 - 0.5;
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let texel = |xi: i64, yi: i64, k: usize| {
        let (xi, yi) = (xi.rem_euclid(w), yi.rem_euclid(h));
        f32::from(pixels[((yi * w + xi) * 4) as usize + k])
    };
    let (x0, y0) = (x0 as i64, y0 as i64);
    [0, 1, 2, 3].map(|k| {
        let top = texel(x0, y0, k) * (1.0 - fx) + texel(x0 + 1, y0, k) * fx;
        let bottom = texel(x0, y0 + 1, k) * (1.0 - fx) + texel(x0 + 1, y0 + 1, k) * fx;
        top * (1.0 - fy) + bottom * fy
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A DDS header for `width` x `height` with `mips` levels, in the given
    /// four-character format, followed by `data`.
    fn dds_file(width: u32, height: u32, mips: u32, four_cc: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut h = vec![0u8; 124];
        let mut put = |at: usize, v: u32| h[at..at + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 124);
        put(4, 0x1007 | if mips > 1 { 0x2_0000 } else { 0 });
        put(8, height);
        put(12, width);
        put(24, mips);
        put(72, 32);
        put(76, 0x4); // four-cc
        put(104, 0x1000);
        h[80..84].copy_from_slice(four_cc);
        let mut out = b"DDS ".to_vec();
        out.extend(h);
        out.extend(data);
        out
    }

    /// A solid red BC1 block.
    const RED_BLOCK: [u8; 8] = [0x00, 0xF8, 0x00, 0xF8, 0, 0, 0, 0];

    #[test]
    fn keeps_block_compressed_levels_as_they_are() {
        // 8x8 is 2x2 blocks, then the 4x4 level is one block.
        let data: Vec<u8> = RED_BLOCK.iter().copied().cycle().take(8 * 5).collect();
        let t = TextureData::from_dds("red.dds", dds_file(8, 8, 2, b"DXT1", &data)).unwrap();
        assert_eq!((t.width, t.height, t.mip_levels), (8, 8, 2));
        assert_eq!(t.gpu_format(true), GpuFormat::Bc1);
        assert_eq!(t.level_data(GpuFormat::Bc1).unwrap(), data);

        // Without compressed-texture support, every level is decoded.
        assert_eq!(t.gpu_format(false), GpuFormat::Rgba8);
        let rgba = t.level_data(GpuFormat::Rgba8).unwrap();
        assert_eq!(rgba.len(), (8 * 8 + 4 * 4) * 4);
        assert_eq!(&rgba[..4], &[255, 0, 0, 255]);
    }

    #[test]
    fn never_claims_more_levels_than_a_full_chain() {
        // 4x4 has levels 4x4, 2x2, 1x1: three at most, whatever the header says.
        let data: Vec<u8> = RED_BLOCK.iter().copied().cycle().take(8 * 5).collect();
        let t = TextureData::from_dds("deep.dds", dds_file(4, 4, 5, b"DXT1", &data)).unwrap();
        assert_eq!(t.mip_levels, 3);
    }

    #[test]
    fn multiplies_textures_at_the_larger_size_with_every_level() {
        // 8x8 red (BC1) times a 4x4 uncompressed half-grey.
        let red: Vec<u8> = RED_BLOCK.iter().copied().cycle().take(8 * 4).collect();
        let red = TextureData::from_dds("red.dds", dds_file(8, 8, 1, b"DXT1", &red)).unwrap();
        let grey = TextureData::from_rgba("grey", 4, 4, [128, 128, 128, 7].repeat(16));
        let product = grey.times(&red).unwrap();
        assert_eq!((product.width, product.height), (8, 8));
        // 8x8, 4x4, 2x2, 1x1.
        assert_eq!(product.mip_levels, 4);
        assert_eq!(product.gpu_format(true), GpuFormat::Rgba8);
        let data = product.level_data(GpuFormat::Rgba8).unwrap();
        assert_eq!(data.len(), (64 + 16 + 4 + 1) * 4);
        // Red times half-grey, with the first texture's alpha.
        assert_eq!(&data[..4], &[128, 0, 0, 7]);
        assert_eq!(&data[data.len() - 4..], &[128, 0, 0, 7]);
        assert_eq!(product.path, "grey x red.dds");
    }

    #[test]
    fn hair_blends_toward_its_layer_by_the_layers_alpha() {
        let base = TextureData::from_rgba("hair.dds", 2, 2, [200, 100, 50, 90].repeat(4));
        // Three quarters of the way to dark brown.
        let layer = TextureData::from_rgba("hair_hl.dds", 1, 1, vec![40, 20, 10, 191]);
        let hair = base.with_layer(&layer).unwrap();
        let data = hair.level_data(GpuFormat::Rgba8).unwrap();
        // 200 × 0.251 + 40 × 0.749 = 80.1 …; the base's alpha stays.
        assert_eq!(&data[..4], &[80, 40, 20, 90]);
    }

    #[test]
    fn only_dxt3_dxt5_and_alpha_masks_count_as_alpha() {
        let dxt1 = TextureData::from_dds("n.dds", dds_file(4, 4, 1, b"DXT1", &RED_BLOCK)).unwrap();
        assert!(!dxt1.has_alpha_channel());
        let dxt5 = TextureData::from_dds("n.dds", dds_file(4, 4, 1, b"DXT5", &[0u8; 16])).unwrap();
        assert!(dxt5.has_alpha_channel());
    }

    #[test]
    fn a_face_tint_adds_twice_its_difference_from_grey() {
        let base = TextureData::from_rgba("face.dds", 2, 2, vec![100; 16]);
        // Brighter in red, darker in green, neutral in blue.
        let tint = TextureData::from_rgba("tint.dds", 1, 1, vec![148, 108, 128, 255]);
        let face = base.plus_face_tint(&tint).unwrap();
        let data = face.level_data(GpuFormat::Rgba8).unwrap();
        // 100 + 2 × (148 − 127.5) = 141; 100 + 2 × (108 − 127.5) = 61;
        // 100 + 2 × 0.5 = 101; then × 4 × FaceGenMap1 (62, 65, 62) / 255:
        // 141 × 0.9725 = 137.1, 61 × 1.0196 = 62.2, 101 × 0.9725 = 98.2;
        // alpha kept.
        assert_eq!(&data[..4], &[137, 62, 98, 100]);
    }

    #[test]
    fn makes_cube_maps_from_six_faces_or_from_one_picture() {
        // A real cube map: six 4x4 BC1 faces, each a different block.
        let faces: Vec<u8> = (0..6u8).flat_map(|f| [f, 0, f, 0, 0, 0, 0, 0]).collect();
        let mut file = dds_file(4, 4, 1, b"DXT1", &faces);
        // Header flags: a cube map with all six faces.
        file[4 + 108..4 + 112].copy_from_slice(&(0x200u32 | 0xFC00).to_le_bytes());
        let cube = TextureData::from_dds("cube.dds", file).unwrap();
        // Used as an ordinary texture, only the first face is uploaded.
        assert_eq!(cube.layers, 1);
        assert_eq!(cube.level_data(GpuFormat::Bc1).unwrap(), faces[..8]);
        let cube = cube.into_cube().unwrap();
        assert_eq!(cube.layers, 6);
        assert_eq!(cube.level_data(GpuFormat::Bc1).unwrap(), faces);

        // A flat picture becomes the same picture on every face, as the
        // game does with its default reflection.
        let flat =
            TextureData::from_dds("flat.dds", dds_file(4, 4, 1, b"DXT1", &RED_BLOCK)).unwrap();
        let cube = flat.into_cube().unwrap();
        assert_eq!(
            cube.level_data(GpuFormat::Bc1).unwrap(),
            RED_BLOCK.repeat(6)
        );

        // Faces must be square.
        let wide =
            TextureData::from_dds("wide.dds", dds_file(8, 4, 1, b"DXT1", &RED_BLOCK.repeat(2)));
        assert!(wide.unwrap().into_cube().is_none());
    }

    #[test]
    fn makes_mip_levels_for_odd_sizes() {
        let t = TextureData::from_rgba(
            "odd",
            3,
            1,
            vec![0, 0, 0, 255, 200, 200, 200, 255, 100, 100, 100, 255],
        );
        // 3x1, then 1x1.
        assert_eq!(t.mip_levels, 2);
        let data = t.level_data(GpuFormat::Rgba8).unwrap();
        assert_eq!(&data[12..], &[100, 100, 100, 255]);
    }

    #[test]
    fn decodes_sizes_that_blocks_dont_fit() {
        // 6 wide: not a multiple of 4, so it can't be uploaded compressed.
        let data: Vec<u8> = RED_BLOCK.iter().copied().cycle().take(8 * 2).collect();
        let t = TextureData::from_dds("odd.dds", dds_file(6, 4, 1, b"DXT1", &data)).unwrap();
        assert_eq!(t.gpu_format(true), GpuFormat::Rgba8);
        assert_eq!(t.level_data(GpuFormat::Rgba8).unwrap().len(), 6 * 4 * 4);
    }
}
