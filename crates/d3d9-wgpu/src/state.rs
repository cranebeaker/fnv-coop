//! The Direct3D 9 device state the backend keeps between draws, and the wgpu states made from
//! it: render states into the pipeline's blend, depth, stencil and raster states; sampler states
//! into samplers. Values and defaults are the DirectX 9 SDK's (`D3DRENDERSTATETYPE`,
//! `D3DSAMPLERSTATETYPE` and the "Render States" defaults table).

/// `D3DRENDERSTATETYPE` values the backend reads.
pub mod rs {
    pub const ZENABLE: usize = 7;
    pub const FILLMODE: usize = 8;
    pub const ZWRITEENABLE: usize = 14;
    pub const ALPHATESTENABLE: usize = 15;
    pub const SRCBLEND: usize = 19;
    pub const DESTBLEND: usize = 20;
    pub const CULLMODE: usize = 22;
    pub const ZFUNC: usize = 23;
    pub const ALPHAREF: usize = 24;
    pub const ALPHAFUNC: usize = 25;
    pub const ALPHABLENDENABLE: usize = 27;
    pub const FOGENABLE: usize = 28;
    pub const STENCILENABLE: usize = 52;
    pub const STENCILFAIL: usize = 53;
    pub const STENCILZFAIL: usize = 54;
    pub const STENCILPASS: usize = 55;
    pub const STENCILFUNC: usize = 56;
    pub const STENCILREF: usize = 57;
    pub const STENCILMASK: usize = 58;
    pub const STENCILWRITEMASK: usize = 59;
    pub const CLIPPLANEENABLE: usize = 152;
    pub const COLORWRITEENABLE: usize = 168;
    pub const BLENDOP: usize = 171;
    pub const SCISSORTESTENABLE: usize = 174;
    pub const SLOPESCALEDEPTHBIAS: usize = 175;
    pub const TWOSIDEDSTENCILMODE: usize = 185;
    pub const CCW_STENCILFAIL: usize = 186;
    pub const CCW_STENCILZFAIL: usize = 187;
    pub const CCW_STENCILPASS: usize = 188;
    pub const CCW_STENCILFUNC: usize = 189;
    pub const COLORWRITEENABLE1: usize = 190;
    pub const COLORWRITEENABLE2: usize = 191;
    pub const COLORWRITEENABLE3: usize = 192;
    pub const BLENDFACTOR: usize = 193;
    pub const SRGBWRITEENABLE: usize = 194;
    pub const DEPTHBIAS: usize = 195;
    pub const SEPARATEALPHABLENDENABLE: usize = 206;
    pub const SRCBLENDALPHA: usize = 207;
    pub const DESTBLENDALPHA: usize = 208;
    pub const BLENDOPALPHA: usize = 209;
}

/// `D3DSAMPLERSTATETYPE` values.
pub mod ss {
    pub const ADDRESSU: usize = 1;
    pub const ADDRESSV: usize = 2;
    pub const ADDRESSW: usize = 3;
    pub const BORDERCOLOR: usize = 4;
    pub const MAGFILTER: usize = 5;
    pub const MINFILTER: usize = 6;
    pub const MIPFILTER: usize = 7;
    pub const MIPMAPLODBIAS: usize = 8;
    pub const MAXMIPLEVEL: usize = 9;
    pub const MAXANISOTROPY: usize = 10;
    pub const SRGBTEXTURE: usize = 11;
}

/// The render states' defaults (the SDK's table); `auto_depth` is whether the device was created
/// with an automatic depth-stencil surface (`D3DRS_ZENABLE`'s default follows it).
pub fn default_render_states(auto_depth: bool) -> Box<[u32; 256]> {
    let mut s = Box::new([0u32; 256]);
    let one = 1.0f32.to_bits();
    for (state, value) in [
        (rs::ZENABLE, auto_depth as u32),
        (rs::FILLMODE, 3),
        (9, 2), // SHADEMODE: GOURAUD
        (rs::ZWRITEENABLE, 1),
        (16, 1), // LASTPIXEL
        (rs::SRCBLEND, 2),
        (rs::DESTBLEND, 1),
        (rs::CULLMODE, 3),
        (rs::ZFUNC, 4),
        (rs::ALPHAFUNC, 8),
        (37, one), // FOGEND 1.0
        (38, one), // FOGDENSITY 1.0
        (rs::STENCILFAIL, 1),
        (rs::STENCILZFAIL, 1),
        (rs::STENCILPASS, 1),
        (rs::STENCILFUNC, 8),
        (rs::STENCILMASK, 0xFFFF_FFFF),
        (rs::STENCILWRITEMASK, 0xFFFF_FFFF),
        (60, 0xFFFF_FFFF),  // TEXTUREFACTOR
        (136, 1),           // CLIPPING
        (137, 1),           // LIGHTING
        (154, one),         // POINTSIZE
        (155, one),         // POINTSIZE_MIN
        (161, 1),           // MULTISAMPLEANTIALIAS
        (162, 0xFFFF_FFFF), // MULTISAMPLEMASK
        (166, 0x4280_0000), // POINTSIZE_MAX 64.0
        (rs::COLORWRITEENABLE, 0xF),
        (rs::BLENDOP, 1),
        (rs::CCW_STENCILFAIL, 1),
        (rs::CCW_STENCILZFAIL, 1),
        (rs::CCW_STENCILPASS, 1),
        (rs::CCW_STENCILFUNC, 8),
        (rs::COLORWRITEENABLE1, 0xF),
        (rs::COLORWRITEENABLE2, 0xF),
        (rs::COLORWRITEENABLE3, 0xF),
        (rs::BLENDFACTOR, 0xFFFF_FFFF),
        (rs::SRCBLENDALPHA, 2),
        (rs::DESTBLENDALPHA, 1),
        (rs::BLENDOPALPHA, 1),
    ] {
        s[state] = value;
    }
    s
}

/// The sampler states' defaults (wrap, point filtering, no mipmaps, anisotropy 1).
pub fn default_sampler_states() -> [u32; 14] {
    let mut s = [0u32; 14];
    s[ss::ADDRESSU] = 1;
    s[ss::ADDRESSV] = 1;
    s[ss::ADDRESSW] = 1;
    s[ss::MAGFILTER] = 1;
    s[ss::MINFILTER] = 1;
    s[ss::MIPFILTER] = 0;
    s[ss::MAXANISOTROPY] = 1;
    s
}

/// `D3DBLEND` as a wgpu blend factor.
fn factor(b: u32) -> wgpu::BlendFactor {
    use wgpu::BlendFactor as F;
    match b {
        1 => F::Zero,
        2 => F::One,
        3 => F::Src,
        4 => F::OneMinusSrc,
        5 => F::SrcAlpha,
        6 => F::OneMinusSrcAlpha,
        7 => F::DstAlpha,
        8 => F::OneMinusDstAlpha,
        9 => F::Dst,
        10 => F::OneMinusDst,
        11 => F::SrcAlphaSaturated,
        14 => F::Constant,
        15 => F::OneMinusConstant,
        _ => F::One,
    }
}

fn operation(op: u32) -> wgpu::BlendOperation {
    use wgpu::BlendOperation as O;
    match op {
        2 => O::Subtract,
        3 => O::ReverseSubtract,
        4 => O::Min,
        5 => O::Max,
        _ => O::Add,
    }
}

/// One blend component from (source, destination, operation), with `D3DBLEND_BOTHSRCALPHA`
/// (12) and `BOTHINVSRCALPHA` (13), which set both factors.
fn component(src: u32, dst: u32, op: u32) -> wgpu::BlendComponent {
    let (src, dst) = match src {
        12 => (5, 6),
        13 => (6, 5),
        _ => (src, dst),
    };
    let operation = operation(op);
    // Minimum and maximum ignore the factors in Direct3D 9; wgpu wants them One.
    let (src_factor, dst_factor) = if matches!(
        operation,
        wgpu::BlendOperation::Min | wgpu::BlendOperation::Max
    ) {
        (wgpu::BlendFactor::One, wgpu::BlendFactor::One)
    } else {
        (factor(src), factor(dst))
    };
    wgpu::BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

/// The blend state the render states give (`None`: blending off).
pub fn blend(s: &[u32; 256]) -> Option<wgpu::BlendState> {
    if s[rs::ALPHABLENDENABLE] == 0 {
        return None;
    }
    let color = component(s[rs::SRCBLEND], s[rs::DESTBLEND], s[rs::BLENDOP]);
    let alpha = if s[rs::SEPARATEALPHABLENDENABLE] != 0 {
        component(
            s[rs::SRCBLENDALPHA],
            s[rs::DESTBLENDALPHA],
            s[rs::BLENDOPALPHA],
        )
    } else {
        color
    };
    Some(wgpu::BlendState { color, alpha })
}

/// `D3DCMPFUNC` as a wgpu comparison.
pub fn compare(f: u32) -> wgpu::CompareFunction {
    use wgpu::CompareFunction as C;
    match f {
        1 => C::Never,
        2 => C::Less,
        3 => C::Equal,
        4 => C::LessEqual,
        5 => C::Greater,
        6 => C::NotEqual,
        7 => C::GreaterEqual,
        _ => C::Always,
    }
}

fn stencil_op(op: u32) -> wgpu::StencilOperation {
    use wgpu::StencilOperation as S;
    match op {
        2 => S::Zero,
        3 => S::Replace,
        4 => S::IncrementClamp,
        5 => S::DecrementClamp,
        6 => S::Invert,
        7 => S::IncrementWrap,
        8 => S::DecrementWrap,
        _ => S::Keep,
    }
}

/// The depth-stencil state for a depth surface of `format` (`stencil`: whether it has stencil
/// bits). `D3DRS_DEPTHBIAS` is in depth units; wgpu's constant is in the format's smallest
/// step (2^-24 for 24-bit depth, 2^-16 for 16-bit).
pub fn depth_stencil(s: &[u32; 256], format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    let z = s[rs::ZENABLE] != 0;
    let stencil_on = s[rs::STENCILENABLE] != 0 && format.has_stencil_aspect();
    let face = |func, fail, zfail, pass| wgpu::StencilFaceState {
        compare: compare(s[func]),
        fail_op: stencil_op(s[fail]),
        depth_fail_op: stencil_op(s[zfail]),
        pass_op: stencil_op(s[pass]),
    };
    let front = face(
        rs::STENCILFUNC,
        rs::STENCILFAIL,
        rs::STENCILZFAIL,
        rs::STENCILPASS,
    );
    let back = if s[rs::TWOSIDEDSTENCILMODE] != 0 {
        face(
            rs::CCW_STENCILFUNC,
            rs::CCW_STENCILFAIL,
            rs::CCW_STENCILZFAIL,
            rs::CCW_STENCILPASS,
        )
    } else {
        front
    };
    let steps = match format {
        wgpu::TextureFormat::Depth16Unorm => 65536.0,
        wgpu::TextureFormat::Depth32Float | wgpu::TextureFormat::Depth32FloatStencil8 => {
            8_388_608.0
        }
        _ => 16_777_216.0,
    };
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: Some(z && s[rs::ZWRITEENABLE] != 0),
        depth_compare: Some(if z {
            compare(s[rs::ZFUNC])
        } else {
            wgpu::CompareFunction::Always
        }),
        stencil: if stencil_on {
            wgpu::StencilState {
                front,
                back,
                read_mask: s[rs::STENCILMASK] & 0xFF,
                write_mask: s[rs::STENCILWRITEMASK] & 0xFF,
            }
        } else {
            wgpu::StencilState::default()
        },
        bias: wgpu::DepthBiasState {
            constant: (f32::from_bits(s[rs::DEPTHBIAS]) * steps) as i32,
            slope_scale: f32::from_bits(s[rs::SLOPESCALEDEPTHBIAS]),
            clamp: 0.0,
        },
    }
}

/// `D3DCULL` with Direct3D 9's front faces (clockwise), as wgpu's culling of a pipeline whose
/// front face is clockwise.
pub fn cull(mode: u32) -> Option<wgpu::Face> {
    match mode {
        2 => Some(wgpu::Face::Front),
        3 => Some(wgpu::Face::Back),
        _ => None,
    }
}

/// A colour write mask from `D3DRS_COLORWRITEENABLE` (bit 0 red .. bit 3 alpha).
pub fn write_mask(bits: u32, opaque: bool) -> wgpu::ColorWrites {
    let mut m = wgpu::ColorWrites::empty();
    if bits & 1 != 0 {
        m |= wgpu::ColorWrites::RED;
    }
    if bits & 2 != 0 {
        m |= wgpu::ColorWrites::GREEN;
    }
    if bits & 4 != 0 {
        m |= wgpu::ColorWrites::BLUE;
    }
    if bits & 8 != 0 && !opaque {
        m |= wgpu::ColorWrites::ALPHA;
    }
    m
}

/// What makes a wgpu sampler: the sampler states that matter, and whether the texture can be
/// filtered at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerKey {
    pub address: [u8; 3],
    pub mag: u8,
    pub min: u8,
    pub mip: u8,
    pub max_level: u8,
    pub anisotropy: u8,
    pub border: u32,
    pub filterable: bool,
}

impl SamplerKey {
    pub fn new(s: &[u32; 14], filterable: bool) -> Self {
        SamplerKey {
            address: [
                s[ss::ADDRESSU] as u8,
                s[ss::ADDRESSV] as u8,
                s[ss::ADDRESSW] as u8,
            ],
            mag: s[ss::MAGFILTER] as u8,
            min: s[ss::MINFILTER] as u8,
            mip: s[ss::MIPFILTER] as u8,
            max_level: s[ss::MAXMIPLEVEL].min(15) as u8,
            anisotropy: s[ss::MAXANISOTROPY].clamp(1, 16) as u8,
            border: s[ss::BORDERCOLOR],
            filterable,
        }
    }

    pub fn descriptor(&self, border_ok: bool) -> wgpu::SamplerDescriptor<'static> {
        let address = |a: u8| match a {
            2 => wgpu::AddressMode::MirrorRepeat,
            3 => wgpu::AddressMode::ClampToEdge,
            4 if border_ok => wgpu::AddressMode::ClampToBorder,
            4 => wgpu::AddressMode::ClampToEdge,
            5 => wgpu::AddressMode::MirrorRepeat,
            _ => wgpu::AddressMode::Repeat,
        };
        let filter = |f: u8| {
            if f >= 2 && self.filterable {
                wgpu::FilterMode::Linear
            } else {
                wgpu::FilterMode::Nearest
            }
        };
        let anisotropic =
            self.filterable && (self.min == 3 || self.mag == 3) && self.anisotropy > 1;
        let mip = if anisotropic || (self.mip >= 2 && self.filterable) {
            wgpu::MipmapFilterMode::Linear
        } else {
            wgpu::MipmapFilterMode::Nearest
        };
        let (min_filter, mag_filter, mipmap_filter) = if anisotropic {
            (wgpu::FilterMode::Linear, wgpu::FilterMode::Linear, mip)
        } else {
            (filter(self.min), filter(self.mag), mip)
        };
        // `D3DTEXF_NONE` as the mip filter: the most detailed level allowed only.
        let base = f32::from(self.max_level);
        let lod_max_clamp = if self.mip == 0 { base } else { 32.0 };
        let border_color = match self.border {
            0 => wgpu::SamplerBorderColor::TransparentBlack,
            0xFF00_0000 => wgpu::SamplerBorderColor::OpaqueBlack,
            _ => wgpu::SamplerBorderColor::OpaqueWhite,
        };
        wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: address(self.address[0]),
            address_mode_v: address(self.address[1]),
            address_mode_w: address(self.address[2]),
            mag_filter,
            min_filter,
            mipmap_filter,
            lod_min_clamp: base,
            lod_max_clamp,
            compare: None,
            anisotropy_clamp: if anisotropic {
                u16::from(self.anisotropy)
            } else {
                1
            },
            border_color: (self.address.contains(&4) && border_ok).then_some(border_color),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blending_is_off_by_default_and_follows_the_render_states() {
        let mut s = default_render_states(true);
        assert!(blend(&s).is_none());
        s[rs::ALPHABLENDENABLE] = 1;
        s[rs::SRCBLEND] = 5;
        s[rs::DESTBLEND] = 6;
        let b = blend(&s).unwrap();
        assert_eq!(b.color.src_factor, wgpu::BlendFactor::SrcAlpha);
        assert_eq!(b.color.dst_factor, wgpu::BlendFactor::OneMinusSrcAlpha);
        assert_eq!(b.alpha, b.color);
        s[rs::SRCBLEND] = 13;
        let both = blend(&s).unwrap();
        assert_eq!(both.color.src_factor, wgpu::BlendFactor::OneMinusSrcAlpha);
        assert_eq!(both.color.dst_factor, wgpu::BlendFactor::SrcAlpha);
    }

    #[test]
    fn depth_defaults_to_less_equal_with_writes() {
        let s = default_render_states(true);
        let d = depth_stencil(&s, wgpu::TextureFormat::Depth24PlusStencil8);
        assert_eq!(d.depth_write_enabled, Some(true));
        assert_eq!(d.depth_compare, Some(wgpu::CompareFunction::LessEqual));
        assert_eq!(d.stencil, wgpu::StencilState::default());
        let off = default_render_states(false);
        assert_eq!(
            depth_stencil(&off, wgpu::TextureFormat::Depth24Plus).depth_compare,
            Some(wgpu::CompareFunction::Always)
        );
    }

    #[test]
    fn counter_clockwise_culling_culls_wgpu_back_faces_of_a_clockwise_front() {
        assert_eq!(cull(3), Some(wgpu::Face::Back));
        assert_eq!(cull(2), Some(wgpu::Face::Front));
        assert_eq!(cull(1), None);
    }

    #[test]
    fn point_sampling_without_mipmaps_stays_on_the_top_level() {
        let k = SamplerKey::new(&default_sampler_states(), true);
        let d = k.descriptor(true);
        assert_eq!(d.min_filter, wgpu::FilterMode::Nearest);
        assert_eq!((d.lod_min_clamp, d.lod_max_clamp), (0.0, 0.0));
        assert_eq!(d.address_mode_u, wgpu::AddressMode::Repeat);
    }
}
