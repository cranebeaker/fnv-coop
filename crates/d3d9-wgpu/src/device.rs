//! The Direct3D 9 device on wgpu: resources by the game's handles, the device state, render
//! passes, pipelines made from the state and draws. Each method is one recorded call
//! (`d3d9-stream`'s `Cmd`), with the meaning the DirectX 9 SDK gives it; the forwarding layer
//! (`engine-platform` `d3d9real.rs`) is the reference for which calls reach a backend.
//!
//! Correctness first (ADR-0007's phase 1): every draw sets its whole state; uploads into a
//! resource the frame's unsent commands still use send those commands first. The caches keyed by
//! state (pipelines, shader modules, samplers, bind groups) are where the speed work starts.

use crate::format::{self, d3dfmt, Format};
use crate::shot::{self, Shots};
use crate::sm::{self, wgsl, Stage};
use crate::state::{self, rs, SamplerKey};
use d3d9_stream::{Handle, Rect};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Where the device's frames go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Output {
    /// To the game's window (`D3DPRESENT_PARAMETERS.hDeviceWindow`), as Direct3D 9 presents.
    Window,
    /// Nowhere but `NV_SHOT` captures: the shadow of another backend that owns the window.
    Offscreen,
}

/// A texture (2D, or a cube's six faces as layers) and the batch that last used it.
pub(crate) struct Tex {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    format: Format,
    width: u32,
    height: u32,
    levels: u32,
    cube: bool,
    /// Samples a pixel (`D3DMULTISAMPLE_TYPE`; 1 without multisampling).
    samples: u32,
    id: u64,
    batch: AtomicU64,
}

/// A surface: a level (and face) of a texture, or a standalone render target or depth surface
/// (a texture of one level), or system memory only.
pub(crate) struct Surf {
    tex: Option<Arc<Tex>>,
    level: u32,
    layer: u32,
    width: u32,
    height: u32,
    format: Format,
    samples: u32,
    /// The view a render pass draws into (one level, one layer).
    view: Option<wgpu::TextureView>,
    id: u64,
}

pub(crate) struct Buf {
    buffer: wgpu::Buffer,
    index32: bool,
    batch: AtomicU64,
}

pub(crate) struct Shader {
    program: sm::Program,
    sig: wgsl::Signature,
    id: u64,
}

/// A `D3DVERTEXELEMENT9`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Element {
    stream: u16,
    offset: u16,
    ty: u8,
    usage: u8,
    index: u8,
}

pub(crate) struct Decl {
    elements: Vec<Element>,
    id: u64,
}

enum Obj {
    Texture(Arc<Tex>, Vec<Handle>),
    Surface(Arc<Surf>),
    Buffer(Arc<Buf>, Vec<u8>),
    Shader(Arc<Shader>),
    Decl(Arc<Decl>),
    Query,
}

#[derive(Clone, Copy, Debug, Default)]
struct Viewport {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    min_z: f32,
    max_z: f32,
}

#[derive(Clone, Default)]
struct Stream {
    buffer: Option<Arc<Buf>>,
    offset: u32,
    stride: u32,
}

/// The D3D9 device state.
struct State {
    rt: [Option<Arc<Surf>>; 4],
    ds: Option<Arc<Surf>>,
    viewport: Viewport,
    scissor: Rect,
    rs: Box<[u32; 256]>,
    /// 16 pixel samplers, then the 4 vertex samplers (`D3DVERTEXTEXTURESAMPLER0` = 257).
    samplers: Box<[[u32; 14]; 20]>,
    textures: [Option<Arc<Tex>>; 20],
    vs: Option<Arc<Shader>>,
    ps: Option<Arc<Shader>>,
    decl: Option<Arc<Decl>>,
    streams: [Stream; 16],
    indices: Option<Arc<Buf>>,
    /// Shader constants as the model holds them: per stage float (4 words), int (4), bool (1).
    vsf: Box<[[u32; 4]; 256]>,
    vsi: [[u32; 4]; 16],
    vsb: [u32; 16],
    psf: Box<[[u32; 4]; 256]>,
    psi: [[u32; 4]; 16],
    psb: [u32; 16],
    /// The uniform blocks need writing again.
    dirty: bool,
}

/// What a render pass draws into.
#[derive(Clone, PartialEq, Eq)]
struct Targets {
    colours: [Option<u64>; 4],
    depth: Option<u64>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct PipeKey {
    vs: u64,
    ps: u64,
    decl: u64,
    strides: [u32; 8],
    topology: u8,
    index32: bool,
    colours: [Option<(wgpu::TextureFormat, bool)>; 4],
    depth: Option<wgpu::TextureFormat>,
    states: [u32; 30],
    samples: u32,
    samplers: Vec<(u32, u8, bool)>,
}

/// The render states a pipeline is made from.
const PIPELINE_STATES: [usize; 30] = [
    rs::ZENABLE,
    rs::ZWRITEENABLE,
    rs::ZFUNC,
    rs::CULLMODE,
    rs::ALPHABLENDENABLE,
    rs::SRCBLEND,
    rs::DESTBLEND,
    rs::BLENDOP,
    rs::SEPARATEALPHABLENDENABLE,
    rs::SRCBLENDALPHA,
    rs::DESTBLENDALPHA,
    rs::BLENDOPALPHA,
    rs::COLORWRITEENABLE,
    rs::COLORWRITEENABLE1,
    rs::COLORWRITEENABLE2,
    rs::COLORWRITEENABLE3,
    rs::STENCILENABLE,
    rs::STENCILFAIL,
    rs::STENCILZFAIL,
    rs::STENCILPASS,
    rs::STENCILFUNC,
    rs::STENCILMASK,
    rs::STENCILWRITEMASK,
    rs::TWOSIDEDSTENCILMODE,
    rs::CCW_STENCILFAIL,
    rs::CCW_STENCILZFAIL,
    rs::CCW_STENCILFUNC,
    rs::CCW_STENCILPASS,
    rs::DEPTHBIAS,
    rs::SLOPESCALEDEPTHBIAS,
];

/// The uniform block of one stage in the ring, rounded up to the offset alignment.
const BLOCK: u64 = wgsl::REGS_SIZE.div_ceil(256) * 256;
/// The ring of uniform blocks and `DrawPrimitiveUP` vertices sent with each batch.
const RING: u64 = 32 << 20;
const UP_RING: u64 = 8 << 20;

pub struct Device {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    adapter: wgpu::Adapter,
    adapter_name: String,
    surface: Option<(wgpu::Surface<'static>, wgpu::SurfaceConfiguration)>,
    present_interval: u32,
    objects: HashMap<Handle, Obj>,
    back_buffers: Vec<Handle>,
    auto_depth: Handle,
    st: State,
    encoder: Option<wgpu::CommandEncoder>,
    pass: Option<wgpu::RenderPass<'static>>,
    pass_targets: Option<Targets>,
    batch: u64,
    ring: Vec<u8>,
    ring_buffer: wgpu::Buffer,
    ring_group: wgpu::BindGroup,
    uniform_layout: wgpu::BindGroupLayout,
    up: Vec<u8>,
    up_buffer: wgpu::Buffer,
    blocks: (u32, u32),
    #[allow(clippy::type_complexity)]
    sampler_layouts: HashMap<Vec<(u32, u8, bool)>, (wgpu::BindGroupLayout, wgpu::PipelineLayout)>,
    pipelines: HashMap<PipeKey, Option<wgpu::RenderPipeline>>,
    #[allow(clippy::type_complexity)]
    vs_modules: HashMap<
        (u64, Vec<wgsl::Input>, Vec<wgsl::Semantic>, Vec<bool>),
        Option<wgpu::ShaderModule>,
    >,
    ps_modules: HashMap<(u64, Vec<wgsl::Semantic>), Option<wgpu::ShaderModule>>,
    samplers: HashMap<SamplerKey, wgpu::Sampler>,
    groups: HashMap<Vec<(u32, u64, Option<SamplerKey>)>, wgpu::BindGroup>,
    blit: HashMap<(wgpu::TextureFormat, bool), wgpu::RenderPipeline>,
    blit_layout: Option<wgpu::BindGroupLayout>,
    /// Depth stand-ins for targets smaller than the bound depth surface ([`Device::usable_depth`]).
    depth_parts: HashMap<(u64, u32, u32), Arc<Surf>>,
    #[allow(clippy::type_complexity)]
    clears: HashMap<
        (
            [Option<(wgpu::TextureFormat, bool)>; 4],
            Option<wgpu::TextureFormat>,
            u32,
        ),
        (wgpu::RenderPipeline, wgpu::BindGroupLayout),
    >,
    dummy: [wgpu::TextureView; 3],
    border: bool,
    float_filterable: bool,
    next_id: u64,
    presents: u64,
    shots: Option<Shots>,
    warned: HashSet<String>,
    errors: Arc<AtomicU64>,
    pub(crate) draws: u64,
    pub(crate) skipped: u64,
}

fn block_on<F: std::future::Future>(f: F) -> F::Output {
    // wgpu's native futures are ready when polled after the work they wait for (a `poll`).
    struct Noop;
    impl std::task::Wake for Noop {
        fn wake(self: Arc<Self>) {}
    }
    let waker = std::task::Waker::from(Arc::new(Noop));
    let mut cx = std::task::Context::from_waker(&waker);
    let mut f = std::pin::pin!(f);
    loop {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
        std::thread::yield_now();
    }
}

/// The present parameters' words (`D3DPRESENT_PARAMETERS`).
pub(crate) struct PresentParameters {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub auto_depth: bool,
    pub depth_format: u32,
    pub interval: u32,
}

impl PresentParameters {
    pub fn from_words(w: &[u32]) -> Self {
        let at = |i: usize| w.get(i).copied().unwrap_or(0);
        PresentParameters {
            width: at(0),
            height: at(1),
            format: at(2),
            auto_depth: at(9) != 0,
            depth_format: at(10),
            interval: at(13),
        }
    }
}

/// `D3DPRESENT_INTERVAL_IMMEDIATE`.
const INTERVAL_IMMEDIATE: u32 = 0x8000_0000;

impl Device {
    /// The wgpu device for the game's (`CreateDevice`): on the game's real window `window` when
    /// `output` is [`Output::Window`].
    pub(crate) fn new(
        output: Output,
        window: isize,
        pp: &PresentParameters,
        back_buffers: &[Handle],
        depth: Handle,
    ) -> Result<Device, String> {
        // `NV_WGPU_BACKEND` names one; by default Direct3D 12 on Windows (measured closest to the
        // real Direct3D 9 on the same driver: Doc's house differs in 0.3% of pixels against 6% on
        // Vulkan, DESIGN.md), then whatever the platform has.
        let tries: Vec<wgpu::Backends> = match std::env::var("NV_WGPU_BACKEND").as_deref() {
            Ok("dx12") => vec![wgpu::Backends::DX12],
            Ok("vulkan") => vec![wgpu::Backends::VULKAN],
            Ok("gl") => vec![wgpu::Backends::GL],
            Ok("metal") => vec![wgpu::Backends::METAL],
            _ if cfg!(windows) => vec![wgpu::Backends::DX12, wgpu::Backends::PRIMARY],
            _ => vec![wgpu::Backends::PRIMARY],
        };
        let mut found = None;
        for backends in tries {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends,
                flags: if std::env::var_os("NV_WGPU_DEBUG").is_some() {
                    wgpu::InstanceFlags::debugging()
                } else {
                    wgpu::InstanceFlags::empty()
                },
                ..wgpu::InstanceDescriptor::new_without_display_handle()
            });
            let surface = match output {
                Output::Window if window != 0 => Some(surface_for(&instance, window)?),
                _ => None,
            };
            let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: surface.as_ref(),
                apply_limit_buckets: false,
            }))
            .ok();
            if let Some(adapter) = adapter {
                found = Some((surface, adapter));
                break;
            }
        }
        let (surface, adapter) = found.ok_or("no wgpu adapter")?;
        let wanted = wgpu::Features::TEXTURE_COMPRESSION_BC
            | wgpu::Features::FLOAT32_FILTERABLE
            | wgpu::Features::TEXTURE_FORMAT_16BIT_NORM
            | wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER
            | wgpu::Features::DEPTH32FLOAT_STENCIL8;
        let features = adapter.features() & wanted;
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("d3d9-wgpu"),
            required_features: features,
            required_limits: adapter.limits(),
            memory_hints: wgpu::MemoryHints::Performance,
            experimental_features: Default::default(),
            trace: Default::default(),
        }))
        .map_err(|e| format!("request_device: {e}"))?;
        let errors = Arc::new(AtomicU64::new(0));
        {
            let errors = errors.clone();
            device.on_uncaptured_error(Arc::new(move |e: wgpu::Error| {
                if errors.fetch_add(1, Ordering::Relaxed) < 20 {
                    eprintln!("NV_D3D9 wgpu: {e}");
                }
            }));
        }
        let info = adapter.get_info();
        let adapter_name = format!("{} ({:?})", info.name, info.backend);
        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("registers"),
            entries: &[0, 1].map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: std::num::NonZeroU64::new(wgsl::REGS_SIZE),
                },
                count: None,
            }),
        });
        let ring_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("register ring"),
            size: RING,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let ring_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("registers"),
            layout: &uniform_layout,
            entries: &[0, 1].map(|binding| wgpu::BindGroupEntry {
                binding,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &ring_buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(wgsl::REGS_SIZE),
                }),
            }),
        });
        let up_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("DrawPrimitiveUP vertices"),
            size: UP_RING,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let dummy = dummies(&device, &queue);
        let mut d = Device {
            device,
            queue,
            adapter: adapter.clone(),
            adapter_name,
            surface: None,
            present_interval: pp.interval,
            objects: HashMap::new(),
            back_buffers: Vec::new(),
            auto_depth: 0,
            st: State {
                rt: Default::default(),
                ds: None,
                viewport: Viewport::default(),
                scissor: [0; 4],
                rs: state::default_render_states(pp.auto_depth),
                samplers: Box::new([state::default_sampler_states(); 20]),
                textures: Default::default(),
                vs: None,
                ps: None,
                decl: None,
                streams: Default::default(),
                indices: None,
                vsf: Box::new([[0; 4]; 256]),
                vsi: [[0; 4]; 16],
                vsb: [0; 16],
                psf: Box::new([[0; 4]; 256]),
                psi: [[0; 4]; 16],
                psb: [0; 16],
                dirty: true,
            },
            encoder: None,
            pass: None,
            pass_targets: None,
            batch: 1,
            ring: Vec::with_capacity(RING as usize),
            ring_buffer,
            ring_group,
            uniform_layout,
            up: Vec::new(),
            up_buffer,
            blocks: (0, 0),
            sampler_layouts: HashMap::new(),
            pipelines: HashMap::new(),
            vs_modules: HashMap::new(),
            ps_modules: HashMap::new(),
            samplers: HashMap::new(),
            groups: HashMap::new(),
            blit: HashMap::new(),
            blit_layout: None,
            depth_parts: HashMap::new(),
            clears: HashMap::new(),
            dummy,
            border: features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER),
            float_filterable: features.contains(wgpu::Features::FLOAT32_FILTERABLE),
            next_id: 1,
            presents: 0,
            shots: Shots::from_env(),
            warned: HashSet::new(),
            errors,
            draws: 0,
            skipped: 0,
        };
        if let Some(surface) = surface {
            let caps = surface.get_capabilities(&adapter);
            let format = caps
                .formats
                .iter()
                .copied()
                .find(|f| *f == wgpu::TextureFormat::Bgra8Unorm)
                .or_else(|| caps.formats.iter().copied().find(|f| !f.is_srgb()))
                .unwrap_or(caps.formats[0]);
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: pp.width.max(1),
                height: pp.height.max(1),
                present_mode: d.present_mode(&caps),
                color_space: wgpu::SurfaceColorSpace::Auto,
                desired_maximum_frame_latency: 2,
                alpha_mode: caps.alpha_modes[0],
                view_formats: vec![],
            };
            surface.configure(&d.device, &config);
            d.surface = Some((surface, config));
        }
        d.make_back_buffers(pp, back_buffers, depth);
        eprintln!(
            "NV_D3D9 wgpu: the game's device on {} ({}x{}, format {}, {})",
            d.adapter_name,
            pp.width,
            pp.height,
            pp.format,
            match output {
                Output::Window => "on the game's window",
                Output::Offscreen => "offscreen",
            }
        );
        Ok(d)
    }

    fn present_mode(&self, caps: &wgpu::SurfaceCapabilities) -> wgpu::PresentMode {
        let immediate = self.present_interval == INTERVAL_IMMEDIATE
            || std::env::var("NV_D3D9_PRESENT").is_ok_and(|v| v.eq_ignore_ascii_case("immediate"));
        if immediate && caps.present_modes.contains(&wgpu::PresentMode::Immediate) {
            wgpu::PresentMode::Immediate
        } else if immediate {
            wgpu::PresentMode::AutoNoVsync
        } else {
            wgpu::PresentMode::Fifo
        }
    }

    fn id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    /// Prints a message once.
    fn warn(&mut self, what: impl Into<String>) {
        let what = what.into();
        if self.warned.insert(what.clone()) {
            eprintln!("NV_D3D9 wgpu: {what}");
        }
    }

    fn make_back_buffers(
        &mut self,
        pp: &PresentParameters,
        back_buffers: &[Handle],
        depth: Handle,
    ) {
        let format = if pp.format == 0 {
            d3dfmt::X8R8G8B8
        } else {
            pp.format
        };
        self.back_buffers = back_buffers.to_vec();
        for &h in back_buffers {
            let s = self.new_surface(pp.width, pp.height, format, false, 1);
            if let Some(s) = s {
                self.objects.insert(h, Obj::Surface(s));
            }
        }
        if let Some(&first) = back_buffers.first() {
            self.st.rt[0] = self.surface_of(first);
        }
        self.auto_depth = depth;
        if depth != 0 && pp.auto_depth {
            if let Some(s) = self.new_surface(pp.width, pp.height, pp.depth_format, true, 1) {
                self.objects.insert(depth, Obj::Surface(s.clone()));
                self.st.ds = Some(s);
            }
        }
        self.st.viewport = Viewport {
            x: 0,
            y: 0,
            w: pp.width,
            h: pp.height,
            min_z: 0.0,
            max_z: 1.0,
        };
        self.st.scissor = [0, 0, pp.width as i32, pp.height as i32];
    }

    fn texture_usage(format: &Format, target: bool) -> wgpu::TextureUsages {
        let mut u = wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC;
        if target || format.depth {
            u |= wgpu::TextureUsages::RENDER_ATTACHMENT;
        }
        u
    }

    #[allow(clippy::too_many_arguments)]
    fn new_texture(
        &mut self,
        width: u32,
        height: u32,
        levels: u32,
        d3d_format: u32,
        cube: bool,
        target: bool,
        samples: u32,
    ) -> Option<Arc<Tex>> {
        let Some(format) = format::format(d3d_format) else {
            self.warn(format!("texture format {d3d_format:#x} is not provided"));
            return None;
        };
        if format.block && (width % 4 != 0 || height % 4 != 0) {
            self.warn(format!(
                "a {width}x{height} block-compressed texture: wgpu wants whole blocks (skipped)"
            ));
            return None;
        }
        let layers = if cube { 6 } else { 1 };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: layers,
            },
            mip_level_count: levels.max(1),
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: format.wgpu,
            // A multisampled surface is only drawn into and resolved (`StretchRect`).
            usage: if samples > 1 {
                wgpu::TextureUsages::RENDER_ATTACHMENT
            } else {
                Self::texture_usage(&format, target)
            },
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(if cube {
                wgpu::TextureViewDimension::Cube
            } else {
                wgpu::TextureViewDimension::D2
            }),
            aspect: if format.depth {
                wgpu::TextureAspect::DepthOnly
            } else {
                wgpu::TextureAspect::All
            },
            ..Default::default()
        });
        let id = self.id();
        Some(Arc::new(Tex {
            texture,
            view,
            format,
            width,
            height,
            levels: levels.max(1),
            cube,
            samples,
            id,
            batch: AtomicU64::new(0),
        }))
    }

    fn level_surface(&mut self, tex: &Arc<Tex>, level: u32, layer: u32) -> Arc<Surf> {
        let view = tex.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_mip_level: level,
            mip_level_count: Some(1),
            base_array_layer: layer,
            array_layer_count: Some(1),
            ..Default::default()
        });
        let id = self.id();
        Arc::new(Surf {
            tex: Some(tex.clone()),
            level,
            layer,
            width: (tex.width >> level).max(1),
            height: (tex.height >> level).max(1),
            format: tex.format,
            samples: tex.samples,
            view: Some(view),
            id,
        })
    }

    fn new_surface(
        &mut self,
        width: u32,
        height: u32,
        d3d: u32,
        depth: bool,
        samples: u32,
    ) -> Option<Arc<Surf>> {
        if d3d == d3dfmt::NULL {
            return None;
        }
        let tex = self.new_texture(width, height, 1, d3d, false, !depth, samples)?;
        Some(self.level_surface(&tex, 0, 0))
    }

    fn surface_of(&self, h: Handle) -> Option<Arc<Surf>> {
        match self.objects.get(&h) {
            Some(Obj::Surface(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn texture_of(&self, h: Handle) -> Option<Arc<Tex>> {
        match self.objects.get(&h) {
            Some(Obj::Texture(t, _)) => Some(t.clone()),
            _ => None,
        }
    }

    fn buffer_of(&self, h: Handle) -> Option<Arc<Buf>> {
        match self.objects.get(&h) {
            Some(Obj::Buffer(b, _)) => Some(b.clone()),
            _ => None,
        }
    }

    // Resources.

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_texture(
        &mut self,
        texture: Handle,
        levels: &[Handle],
        width: u32,
        height: u32,
        usage: u32,
        format: u32,
        cube: bool,
    ) {
        let count = if cube {
            levels.len() as u32 / 6
        } else {
            levels.len() as u32
        };
        let target = usage & 1 != 0;
        let Some(tex) = self.new_texture(width, height, count.max(1), format, cube, target, 1)
        else {
            return;
        };
        let mut surfaces = Vec::with_capacity(levels.len());
        for (k, &h) in levels.iter().enumerate() {
            let (layer, level) = if cube {
                ((k as u32) / count, (k as u32) % count)
            } else {
                (0, k as u32)
            };
            let s = self.level_surface(&tex, level, layer);
            self.objects.insert(h, Obj::Surface(s));
            surfaces.push(h);
        }
        self.objects.insert(texture, Obj::Texture(tex, surfaces));
    }

    pub(crate) fn create_surface(
        &mut self,
        surface: Handle,
        depth: bool,
        width: u32,
        height: u32,
        format: u32,
        multisample: u32,
    ) {
        // `D3DMULTISAMPLE_2_SAMPLES` and up are the sample counts; NONMASKABLE (1) has none here.
        let mut samples = if multisample >= 2 { multisample } else { 1 };
        if samples > 1 {
            let supported = format::format(format).is_some_and(|f| {
                self.adapter
                    .get_texture_format_features(f.wgpu)
                    .flags
                    .sample_count_supported(samples)
            });
            if !supported {
                self.warn(format!(
                    "{samples}x multisampling of format {format:#x} is not supported here: one sample"
                ));
                samples = 1;
            }
        }
        if let Some(s) = self.new_surface(width, height, format, depth, samples) {
            self.objects.insert(surface, Obj::Surface(s));
        } else if format != d3dfmt::NULL {
            self.warn(format!("a surface of format {format:#x} is not provided"));
        }
    }

    pub(crate) fn create_buffer(&mut self, buffer: Handle, index: bool, length: u32, format: u32) {
        let size = u64::from(length.max(4).div_ceil(4) * 4);
        let wgpu_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: if index {
                wgpu::BufferUsages::INDEX
            } else {
                wgpu::BufferUsages::VERTEX
            } | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let b = Arc::new(Buf {
            buffer: wgpu_buffer,
            index32: index && format == 102,
            batch: AtomicU64::new(0),
        });
        self.objects
            .insert(buffer, Obj::Buffer(b, vec![0; size as usize]));
    }

    pub(crate) fn create_shader(&mut self, shader: Handle, tokens: &[u32]) {
        match sm::parse(tokens) {
            Ok(program) => {
                let sig = wgsl::signature(&program);
                let id = self.id();
                self.objects
                    .insert(shader, Obj::Shader(Arc::new(Shader { program, sig, id })));
            }
            Err(e) => self.warn(format!("a shader the converter cannot read: {e}")),
        }
    }

    pub(crate) fn create_declaration(&mut self, declaration: Handle, bytes: &[u8]) {
        let mut elements = Vec::new();
        for e in bytes.chunks_exact(8) {
            if e[0] == 0xFF || e[4] == 17 {
                break;
            }
            elements.push(Element {
                stream: u16::from_le_bytes([e[0], e[1]]),
                offset: u16::from_le_bytes([e[2], e[3]]),
                ty: e[4],
                usage: e[6],
                index: e[7],
            });
        }
        let id = self.id();
        self.objects
            .insert(declaration, Obj::Decl(Arc::new(Decl { elements, id })));
    }

    pub(crate) fn create_query(&mut self, query: Handle) {
        self.objects.insert(query, Obj::Query);
    }

    pub(crate) fn destroy(&mut self, object: Handle) {
        if let Some(Obj::Texture(_, surfaces)) = self.objects.remove(&object) {
            for s in surfaces {
                self.objects.remove(&s);
            }
        }
    }

    /// Sends the frame's commands so far, so a later upload cannot change what they draw.
    fn flush(&mut self) {
        self.end_pass();
        if !self.ring.is_empty() {
            self.queue.write_buffer(&self.ring_buffer, 0, &self.ring);
            self.ring.clear();
        }
        if !self.up.is_empty() {
            self.queue.write_buffer(&self.up_buffer, 0, &self.up);
            self.up.clear();
        }
        if let Some(e) = self.encoder.take() {
            self.queue.submit([e.finish()]);
        }
        self.batch += 1;
        self.st.dirty = true;
    }

    pub(crate) fn upload_surface(&mut self, surface: Handle, data: &[u8]) {
        let Some(s) = self.surface_of(surface) else {
            return;
        };
        let Some(tex) = s.tex.clone() else { return };
        if tex.batch.load(Ordering::Relaxed) == self.batch {
            self.flush();
        }
        let (row, rows) = format::level_layout(&s.format, s.width, s.height);
        if (data.len() as u64) < u64::from(row) * u64::from(rows) {
            self.warn("an upload shorter than its level (skipped)");
            return;
        }
        let bytes = format::convert(&s.format, &data[..(row * rows) as usize]);
        let (row, rows) = format::uploaded_layout(&s.format, s.width, s.height);
        let (w, h) = if s.format.block {
            (s.width.div_ceil(4) * 4, s.height.div_ceil(4) * 4)
        } else {
            (s.width, s.height)
        };
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &tex.texture,
                mip_level: s.level,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: s.layer,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(row),
                rows_per_image: Some(rows),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
    }

    pub(crate) fn upload_buffer(
        &mut self,
        buffer: Handle,
        offset: u32,
        data: &d3d9_stream::Data<'_>,
    ) {
        let Some(Obj::Buffer(b, shadow)) = self.objects.get_mut(&buffer) else {
            return;
        };
        let b = b.clone();
        let len = (data.len() as usize).min(shadow.len().saturating_sub(offset as usize));
        if len == 0 {
            return;
        }
        data.copy_to(0, &mut shadow[offset as usize..offset as usize + len]);
        let start = (offset as usize) & !3;
        let end = (offset as usize + len).div_ceil(4) * 4;
        let end = end.min(shadow.len());
        let bytes = shadow[start..end].to_vec();
        if b.batch.load(Ordering::Relaxed) == self.batch {
            self.flush();
        }
        self.queue.write_buffer(&b.buffer, start as u64, &bytes);
    }

    // State.

    pub(crate) fn set_render_target(&mut self, index: u32, surface: Handle) {
        let s = if surface == 0 {
            None
        } else {
            self.surface_of(surface)
        };
        if surface != 0 && s.is_none() {
            self.warn("a render target without a counterpart");
        }
        if index == 0 {
            // SetRenderTarget(0) sets the viewport and scissor to the whole target.
            if let Some(s) = &s {
                self.st.viewport = Viewport {
                    x: 0,
                    y: 0,
                    w: s.width,
                    h: s.height,
                    min_z: 0.0,
                    max_z: 1.0,
                };
                self.st.scissor = [0, 0, s.width as i32, s.height as i32];
            }
        }
        if let Some(slot) = self.st.rt.get_mut(index as usize) {
            *slot = s;
        }
        self.st.dirty = true;
    }

    pub(crate) fn set_depth_stencil(&mut self, surface: Handle) {
        self.st.ds = if surface == 0 {
            None
        } else {
            self.surface_of(surface)
        };
    }

    pub(crate) fn set_viewport(&mut self, v: [u32; 6]) {
        self.st.viewport = Viewport {
            x: v[0],
            y: v[1],
            w: v[2],
            h: v[3],
            min_z: f32::from_bits(v[4]),
            max_z: f32::from_bits(v[5]),
        };
        self.st.dirty = true;
    }

    pub(crate) fn set_scissor(&mut self, r: Rect) {
        self.st.scissor = r;
    }

    pub(crate) fn set_render_state(&mut self, state: u32, value: u32) {
        if let Some(s) = self.st.rs.get_mut(state as usize) {
            *s = value;
        }
        match state as usize {
            rs::ALPHATESTENABLE | rs::ALPHAREF | rs::ALPHAFUNC => self.st.dirty = true,
            rs::FOGENABLE if value != 0 => {
                self.warn("D3DRS_FOGENABLE (fixed-function fog) is not provided")
            }
            rs::SRGBWRITEENABLE if value != 0 => self.warn("D3DRS_SRGBWRITEENABLE is not provided"),
            rs::CLIPPLANEENABLE if value != 0 => self.warn("user clip planes are not provided yet"),
            rs::FILLMODE if value != 3 => {
                self.warn("wireframe and point fill modes are not provided")
            }
            _ => {}
        }
    }

    fn sampler_slot(sampler: u32) -> Option<usize> {
        match sampler {
            0..=15 => Some(sampler as usize),
            257..=260 => Some(16 + (sampler - 257) as usize),
            _ => None,
        }
    }

    pub(crate) fn set_sampler_state(&mut self, sampler: u32, kind: u32, value: u32) {
        if let Some(slot) = Self::sampler_slot(sampler) {
            if let Some(s) = self.st.samplers[slot].get_mut(kind as usize) {
                *s = value;
            }
            self.st.dirty = true;
            if kind as usize == state::ss::SRGBTEXTURE && value != 0 {
                self.warn("D3DSAMP_SRGBTEXTURE is not provided yet");
            }
        }
    }

    pub(crate) fn set_texture(&mut self, sampler: u32, texture: Handle) {
        if let Some(slot) = Self::sampler_slot(sampler) {
            self.st.textures[slot] = if texture == 0 {
                None
            } else {
                self.texture_of(texture)
            };
            self.st.dirty = true;
        }
    }

    pub(crate) fn set_shader(&mut self, pixel: bool, shader: Handle) {
        let s = match self.objects.get(&shader) {
            Some(Obj::Shader(s)) => Some(s.clone()),
            _ => None,
        };
        if pixel {
            self.st.ps = s;
        } else {
            self.st.vs = s;
        }
    }

    pub(crate) fn set_declaration(&mut self, declaration: Handle) {
        self.st.decl = match self.objects.get(&declaration) {
            Some(Obj::Decl(d)) => Some(d.clone()),
            _ => None,
        };
    }

    pub(crate) fn set_constants(&mut self, kind: u8, start: u32, count: u32, words: &[u32]) {
        let st = &mut self.st;
        let s = start as usize;
        match kind {
            0 | 3 => {
                let regs = if kind == 0 { &mut st.vsf } else { &mut st.psf };
                for (k, r) in words.chunks_exact(4).take(count as usize).enumerate() {
                    if let Some(slot) = regs.get_mut(s + k) {
                        slot.copy_from_slice(r);
                    }
                }
            }
            1 | 4 => {
                let regs = if kind == 1 { &mut st.vsi } else { &mut st.psi };
                for (k, r) in words.chunks_exact(4).take(count as usize).enumerate() {
                    if let Some(slot) = regs.get_mut(s + k) {
                        slot.copy_from_slice(r);
                    }
                }
            }
            _ => {
                let regs = if kind == 2 { &mut st.vsb } else { &mut st.psb };
                for (k, &v) in words.iter().take(count as usize).enumerate() {
                    if let Some(slot) = regs.get_mut(s + k) {
                        *slot = v;
                    }
                }
            }
        }
        st.dirty = true;
    }

    pub(crate) fn set_stream(&mut self, stream: u32, buffer: Handle, offset: u32, stride: u32) {
        let b = if buffer == 0 {
            None
        } else {
            self.buffer_of(buffer)
        };
        if let Some(s) = self.st.streams.get_mut(stream as usize) {
            *s = Stream {
                buffer: b,
                offset,
                stride,
            };
        }
    }

    pub(crate) fn set_indices(&mut self, buffer: Handle) {
        self.st.indices = if buffer == 0 {
            None
        } else {
            self.buffer_of(buffer)
        };
    }

    // Passes.

    fn encoder(&mut self) -> &mut wgpu::CommandEncoder {
        self.encoder.get_or_insert_with(|| {
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
        })
    }

    fn end_pass(&mut self) {
        self.pass = None;
        self.pass_targets = None;
    }

    fn current_targets(&mut self) -> Targets {
        Targets {
            colours: std::array::from_fn(|i| self.st.rt[i].as_ref().map(|s| s.id)),
            depth: self.usable_depth().map(|s| s.id),
        }
    }

    /// The depth surface when it can be used with the colour targets (wgpu wants one size).
    /// The depth surface to draw with: the bound one, or when it is larger than the colour
    /// target (Direct3D 9 then uses its top-left part; wgpu wants one size) a stand-in of the
    /// target's size kept for that pair, whose contents are its own (the game clears it). None
    /// when the sample counts differ (Direct3D 9 refuses to draw so).
    fn usable_depth(&mut self) -> Option<Arc<Surf>> {
        let ds = self.st.ds.clone()?;
        let Some(rt) = self.st.rt.iter().flatten().next().cloned() else {
            return Some(ds);
        };
        if rt.samples != ds.samples {
            return None;
        }
        if rt.width == ds.width && rt.height == ds.height {
            return Some(ds);
        }
        if rt.width > ds.width || rt.height > ds.height {
            return None;
        }
        let key = (ds.id, rt.width, rt.height);
        if let Some(s) = self.depth_parts.get(&key) {
            return Some(s.clone());
        }
        let tex = self.new_texture(
            rt.width,
            rt.height,
            1,
            ds.format.d3d,
            false,
            false,
            ds.samples,
        )?;
        let s = self.level_surface(&tex, 0, 0);
        self.depth_parts.insert(key, s.clone());
        Some(s)
    }

    /// Opens a render pass on the current targets (loading what they hold), or clears them as it
    /// opens (`clear`: colour, depth, stencil).
    fn begin_pass(&mut self, clear: Option<(wgpu::Color, Option<f32>, Option<u32>, bool)>) {
        let targets = self.current_targets();
        if clear.is_none() && self.pass.is_some() && self.pass_targets.as_ref() == Some(&targets) {
            return;
        }
        self.end_pass();
        if let (Some(ds), Some(rt)) = (&self.st.ds, self.st.rt.iter().flatten().next()) {
            if ds.width < rt.width || ds.height < rt.height || ds.samples != rt.samples {
                let what = format!(
                    "a {}x{} depth surface ({} samples) with a {}x{} render target ({} samples): drawn without depth",
                    ds.width, ds.height, ds.samples, rt.width, rt.height, rt.samples
                );
                self.warn(what);
            }
        }
        // The views and their load operations, owned here so the encoder can be borrowed.
        let colour_views: Vec<Option<(wgpu::TextureView, wgpu::LoadOp<wgpu::Color>)>> = self
            .st
            .rt
            .iter()
            .map(|rt| {
                rt.as_ref().and_then(|s| {
                    s.view.clone().map(|view| {
                        let load = match clear {
                            Some((c, _, _, true)) => wgpu::LoadOp::Clear(if s.format.opaque {
                                wgpu::Color { a: 1.0, ..c }
                            } else {
                                c
                            }),
                            _ => wgpu::LoadOp::Load,
                        };
                        (view, load)
                    })
                })
            })
            .collect();
        let colours: Vec<Option<wgpu::RenderPassColorAttachment>> = colour_views
            .iter()
            .map(|c| {
                c.as_ref()
                    .map(|(view, load)| wgpu::RenderPassColorAttachment {
                        view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: *load,
                            store: wgpu::StoreOp::Store,
                        },
                    })
            })
            .collect();
        // As many attachments as the pipelines have targets: up to the last bound one.
        let colours = &colours[..colours
            .iter()
            .rposition(|c| c.is_some())
            .map_or(0, |i| i + 1)];
        let depth = self.usable_depth();
        let depth_view = depth.as_ref().and_then(|s| s.view.clone());
        let depth_attachment = depth.as_ref().zip(depth_view.as_ref()).map(|(s, view)| {
            let stencil = s.format.wgpu.has_stencil_aspect();
            wgpu::RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(wgpu::Operations {
                    load: match clear {
                        Some((_, Some(z), _, _)) => wgpu::LoadOp::Clear(z),
                        _ => wgpu::LoadOp::Load,
                    },
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: stencil.then_some(wgpu::Operations {
                    load: match clear {
                        Some((_, _, Some(v), _)) => wgpu::LoadOp::Clear(v),
                        _ => wgpu::LoadOp::Load,
                    },
                    store: wgpu::StoreOp::Store,
                }),
            }
        });
        let pass = self
            .encoder()
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: colours,
                depth_stencil_attachment: depth_attachment,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            })
            .forget_lifetime();
        self.pass = Some(pass);
        self.pass_targets = Some(targets);
        // The surfaces' textures are in use by this batch.
        for s in self.st.rt.iter().flatten() {
            if let Some(t) = &s.tex {
                t.batch.store(self.batch, Ordering::Relaxed);
            }
        }
        if let Some(t) = depth.as_ref().and_then(|d| d.tex.as_ref()) {
            t.batch.store(self.batch, Ordering::Relaxed);
        }
    }

    pub(crate) fn clear(
        &mut self,
        rects: Option<&[i32]>,
        flags: u32,
        colour: u32,
        z: f32,
        stencil: u32,
    ) {
        let target = self.st.rt.iter().flatten().next().cloned();
        let (w, h) = match (&target, &self.st.ds) {
            (Some(t), _) => (t.width, t.height),
            (None, Some(d)) => (d.width, d.height),
            _ => return,
        };
        let v = self.st.viewport;
        let scissored =
            self.st.rs[rs::SCISSORTESTENABLE] != 0 && self.st.scissor != [0, 0, w as i32, h as i32];
        let whole = v.x == 0 && v.y == 0 && v.w >= w && v.h >= h && !scissored;
        let rects_whole = match rects {
            None => true,
            Some(r) => r
                .chunks_exact(4)
                .any(|r| r[0] <= 0 && r[1] <= 0 && r[2] >= w as i32 && r[3] >= h as i32),
        };
        let c = wgpu::Color {
            r: f64::from((colour >> 16) & 0xFF) / 255.0,
            g: f64::from((colour >> 8) & 0xFF) / 255.0,
            b: f64::from(colour & 0xFF) / 255.0,
            a: f64::from(colour >> 24) / 255.0,
        };
        if !(whole && rects_whole) {
            // The viewport (and the scissor rectangle when on), with each rectangle given.
            let mut area = [
                v.x as i32,
                v.y as i32,
                (v.x + v.w).min(w) as i32,
                (v.y + v.h).min(h) as i32,
            ];
            if self.st.rs[rs::SCISSORTESTENABLE] != 0 {
                let s = self.st.scissor;
                area = [
                    area[0].max(s[0]),
                    area[1].max(s[1]),
                    area[2].min(s[2]),
                    area[3].min(s[3]),
                ];
            }
            let parts: Vec<[i32; 4]> = match rects {
                None => vec![area],
                Some(r) => r
                    .chunks_exact(4)
                    .map(|r| {
                        [
                            area[0].max(r[0]),
                            area[1].max(r[1]),
                            area[2].min(r[2]),
                            area[3].min(r[3]),
                        ]
                    })
                    .collect(),
            };
            self.clear_draw(&parts, flags, c, z.clamp(0.0, 1.0), stencil & 0xFF);
            return;
        }
        self.begin_pass(Some((
            c,
            (flags & 2 != 0).then_some(z.clamp(0.0, 1.0)),
            (flags & 4 != 0).then_some(stencil & 0xFF),
            flags & 1 != 0,
        )));
    }

    /// A clear of rectangles of the targets: a draw writing the colour, depth and stencil the
    /// flags name, cut to each rectangle by the scissor.
    fn clear_draw(&mut self, parts: &[[i32; 4]], flags: u32, c: wgpu::Color, z: f32, stencil: u32) {
        let colours: [Option<(wgpu::TextureFormat, bool)>; 4] = std::array::from_fn(|i| {
            self.st.rt[i]
                .as_ref()
                .map(|s| (s.format.wgpu, s.format.opaque))
        });
        let depth_surface = self.usable_depth();
        let depth = depth_surface.as_ref().map(|d| d.format.wgpu);
        let samples = self
            .st
            .rt
            .iter()
            .flatten()
            .next()
            .map(|s| s.samples)
            .or(depth_surface.as_ref().map(|d| d.samples))
            .unwrap_or(1);
        let key = (colours, depth, (flags & 7) | (samples << 8));
        if !self.clears.contains_key(&key) {
            let n = colours
                .iter()
                .rposition(|c| c.is_some())
                .map_or(0, |i| i + 1);
            let mut outputs = String::new();
            let mut values = Vec::new();
            for i in 0..n {
                let _ = std::fmt::Write::write_fmt(
                    &mut outputs,
                    format_args!("@location({i}) c{i}: vec4<f32>, "),
                );
                values.push("k.colour");
            }
            let fs = if n == 0 {
                "@fragment fn fs() {}".to_string()
            } else {
                format!(
                    "struct O {{ {outputs} }}\n@fragment fn fs() -> O {{ return O({}); }}",
                    values.join(", ")
                )
            };
            let source = format!(
                "struct K {{ colour: vec4<f32>, z: vec4<f32> }}\n@group(0) @binding(0) var<uniform> k: K;\n@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {{\n    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));\n    return vec4<f32>(p * 2.0 - 1.0, k.z.x, 1.0);\n}}\n{fs}\n"
            );
            let module = self
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("clear"),
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                });
            let layout = self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("clear"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
            let pl = self
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[Some(&layout)],
                    immediate_size: 0,
                });
            let targets: Vec<Option<wgpu::ColorTargetState>> = colours[..n]
                .iter()
                .map(|c| {
                    c.map(|(format, opaque)| wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: if flags & 1 != 0 {
                            state::write_mask(0xF, opaque)
                        } else {
                            wgpu::ColorWrites::empty()
                        },
                    })
                })
                .collect();
            let stencil_face = wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                fail_op: wgpu::StencilOperation::Replace,
                depth_fail_op: wgpu::StencilOperation::Replace,
                pass_op: wgpu::StencilOperation::Replace,
            };
            let pipeline = self
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("clear"),
                    layout: Some(&pl),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: depth.map(|format| wgpu::DepthStencilState {
                        format,
                        depth_write_enabled: Some(flags & 2 != 0),
                        depth_compare: Some(wgpu::CompareFunction::Always),
                        stencil: if flags & 4 != 0 && format.has_stencil_aspect() {
                            wgpu::StencilState {
                                front: stencil_face,
                                back: stencil_face,
                                read_mask: 0xFF,
                                write_mask: 0xFF,
                            }
                        } else {
                            wgpu::StencilState::default()
                        },
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: samples,
                        ..Default::default()
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: Some("fs"),
                        compilation_options: Default::default(),
                        targets: &targets,
                    }),
                    multiview_mask: None,
                    cache: None,
                });
            self.clears.insert(key, (pipeline, layout));
        }
        let (pipeline, layout) = self.clears[&key].clone();
        let values: [f32; 8] = [
            c.r as f32, c.g as f32, c.b as f32, c.a as f32, z, 0.0, 0.0, 0.0,
        ];
        let ub = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: true,
        });
        if let Ok(mut mapped) = ub.slice(..).get_mapped_range_mut() {
            let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
            mapped.copy_from_slice(&bytes);
        }
        ub.unmap();
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: ub.as_entire_binding(),
            }],
        });
        self.begin_pass(None);
        let pass = self.pass.as_mut().expect("a pass is open");
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.set_stencil_reference(stencil);
        for p in parts {
            if p[2] <= p[0] || p[3] <= p[1] || p[0] < 0 || p[1] < 0 {
                continue;
            }
            pass.set_scissor_rect(
                p[0] as u32,
                p[1] as u32,
                (p[2] - p[0]) as u32,
                (p[3] - p[1]) as u32,
            );
            pass.draw(0..3, 0..1);
        }
    }

    // Draws.

    fn input_of(ty: u8) -> Option<(wgpu::VertexFormat, wgsl::Input)> {
        use wgpu::VertexFormat as V;
        use wgsl::Input as I;
        Some(match ty {
            0 => (V::Float32, I::Float(1)),
            1 => (V::Float32x2, I::Float(2)),
            2 => (V::Float32x3, I::Float(3)),
            3 => (V::Float32x4, I::Float(4)),
            4 => (V::Unorm8x4Bgra, I::Float(4)),
            5 => (V::Uint8x4, I::Uint4),
            6 => (V::Sint16x2, I::Sint(2)),
            7 => (V::Sint16x4, I::Sint(4)),
            8 => (V::Unorm8x4, I::Float(4)),
            9 => (V::Snorm16x2, I::Float(2)),
            10 => (V::Snorm16x4, I::Float(4)),
            11 => (V::Unorm16x2, I::Float(2)),
            12 => (V::Unorm16x4, I::Float(4)),
            15 => (V::Float16x2, I::Float(2)),
            16 => (V::Float16x4, I::Float(4)),
            _ => return None,
        })
    }

    fn module(
        &mut self,
        source: Result<String, sm::Error>,
        what: &str,
    ) -> Option<wgpu::ShaderModule> {
        let source = match source {
            Ok(s) => s,
            Err(e) => {
                self.warn(format!("{what}: {e}"));
                return None;
            }
        };
        if let Err(e) = sm::validate(&source) {
            self.warn(format!("{what}: the converted shader is not valid: {e}"));
            return None;
        }
        Some(
            self.device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                }),
        )
    }

    fn sampler_layout(&mut self, key: &[(u32, u8, bool)]) -> wgpu::PipelineLayout {
        if let Some((_, p)) = self.sampler_layouts.get(key) {
            return p.clone();
        }
        let mut entries = Vec::new();
        for &(binding, dim, filterable) in key {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable },
                    view_dimension: match dim {
                        1 => wgpu::TextureViewDimension::Cube,
                        2 => wgpu::TextureViewDimension::D3,
                        _ => wgpu::TextureViewDimension::D2,
                    },
                    multisampled: false,
                },
                count: None,
            });
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(if filterable {
                    wgpu::SamplerBindingType::Filtering
                } else {
                    wgpu::SamplerBindingType::NonFiltering
                }),
                count: None,
            });
        }
        let layout = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            });
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&self.uniform_layout), Some(&layout)],
                immediate_size: 0,
            });
        self.sampler_layouts
            .insert(key.to_vec(), (layout, pipeline_layout.clone()));
        pipeline_layout
    }

    fn filterable(&self, t: Option<&Arc<Tex>>) -> bool {
        match t {
            None => true,
            Some(t) => {
                !t.format.depth
                    && (self.float_filterable
                        || !matches!(
                            t.format.wgpu,
                            wgpu::TextureFormat::R32Float
                                | wgpu::TextureFormat::Rg32Float
                                | wgpu::TextureFormat::Rgba32Float
                        ))
            }
        }
    }

    /// The samplers both shaders use, as (binding, dimension, filterable) and the device sampler
    /// slot.
    fn sampler_bindings(&self, vs: &Shader, ps: &Shader) -> Vec<((u32, u8, bool), usize)> {
        let mut out = Vec::new();
        for (stage, shader) in [(Stage::Vertex, vs), (Stage::Pixel, ps)] {
            for &(n, dim) in &shader.sig.samplers {
                let slot = match stage {
                    Stage::Pixel => n as usize,
                    Stage::Vertex => 16 + n as usize,
                };
                if slot >= 20 {
                    continue;
                }
                let d = match dim {
                    wgsl::Dim::D2 => 0,
                    wgsl::Dim::Cube => 1,
                    wgsl::Dim::D3 => 2,
                };
                let filterable = self.filterable(self.st.textures[slot].as_ref());
                out.push(((wgsl::texture_binding(stage, n), d, filterable), slot));
            }
        }
        out
    }

    fn topology(kind: u32) -> Option<wgpu::PrimitiveTopology> {
        use wgpu::PrimitiveTopology as P;
        Some(match kind {
            1 => P::PointList,
            2 => P::LineList,
            3 => P::LineStrip,
            4 => P::TriangleList,
            5 => P::TriangleStrip,
            _ => return None,
        })
    }

    /// Vertices a draw of `count` primitives reads.
    fn vertex_count(kind: u32, count: u32) -> u32 {
        match kind {
            1 => count,
            2 => count * 2,
            3 => count + 1,
            4 => count * 3,
            5 | 6 => count + 2,
            _ => 0,
        }
    }

    fn write_blocks(&mut self) {
        if !self.st.dirty && !self.ring.is_empty() {
            return;
        }
        if self.ring.len() as u64 + 2 * BLOCK > RING {
            self.flush();
        }
        let mut ring = std::mem::take(&mut self.ring);
        let st = &self.st;
        // Each sampler's fill mode (its texture's format) and LOD bias.
        let per_sampler = |range: std::ops::Range<usize>| -> ([u32; 16], [f32; 16]) {
            let mut fix = [0u32; 16];
            let mut bias = [0f32; 16];
            for (k, slot) in range.enumerate() {
                fix[k] = st.textures[slot].as_ref().map_or(0, |t| t.format.fix);
                bias[k] = f32::from_bits(st.samplers[slot][state::ss::MIPMAPLODBIAS]);
            }
            (fix, bias)
        };
        let v = st.viewport;
        let vs_sys = [1.0 / v.w.max(1) as f32, -1.0 / v.h.max(1) as f32, 0.0, 0.0];
        let alpha_func = if st.rs[rs::ALPHATESTENABLE] != 0 {
            st.rs[rs::ALPHAFUNC] as f32
        } else {
            8.0
        };
        let ps_sys = [
            (st.rs[rs::ALPHAREF] & 0xFF) as f32 / 255.0,
            alpha_func,
            0.0,
            0.0,
        ];
        let vs = write_block(
            &mut ring,
            &st.vsf,
            &st.vsi,
            &st.vsb,
            vs_sys,
            per_sampler(16..20),
        );
        let ps = write_block(
            &mut ring,
            &st.psf,
            &st.psi,
            &st.psb,
            ps_sys,
            per_sampler(0..16),
        );
        self.ring = ring;
        self.blocks = (vs, ps);
        self.st.dirty = false;
    }

    fn sampler(&mut self, key: SamplerKey) -> wgpu::Sampler {
        if let Some(s) = self.samplers.get(&key) {
            return s.clone();
        }
        let s = self.device.create_sampler(&key.descriptor(self.border));
        self.samplers.insert(key, s.clone());
        s
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw(
        &mut self,
        kind: u32,
        indexed: bool,
        base_vertex: i32,
        start: u32,
        count: u32,
        up: Option<(&[u8], u32)>,
    ) {
        let (Some(vs), Some(ps)) = (self.st.vs.clone(), self.st.ps.clone()) else {
            self.warn("a draw without both shaders (fixed function is not provided): skipped");
            self.skipped += 1;
            return;
        };
        let Some(topology) = Self::topology(kind) else {
            self.warn(format!("primitive type {kind} is not provided: skipped"));
            self.skipped += 1;
            return;
        };
        let Some(decl) = self.st.decl.clone() else {
            self.warn("a draw without a vertex declaration (FVF is not provided): skipped");
            self.skipped += 1;
            return;
        };
        if self.st.rt.iter().all(|r| r.is_none()) && self.st.ds.is_none() {
            self.skipped += 1;
            return;
        }
        // The vertex inputs: each shader input from the declaration's element of its usage.
        let mut inputs = vec![wgsl::Input::Missing; 16];
        let mut attributes: [Vec<wgpu::VertexAttribute>; 8] = Default::default();
        for (r, sem) in &vs.sig.inputs {
            if r.n >= 16 {
                continue;
            }
            let Some(e) = decl
                .elements
                .iter()
                .find(|e| e.usage == sem.usage && e.index == sem.index)
            else {
                continue;
            };
            let Some((format, input)) = Self::input_of(e.ty) else {
                self.warn(format!("vertex element type {} is not provided", e.ty));
                continue;
            };
            if e.stream >= 8 {
                continue;
            }
            inputs[r.n as usize] = input;
            attributes[e.stream as usize].push(wgpu::VertexAttribute {
                format,
                offset: u64::from(e.offset),
                shader_location: r.n,
            });
        }
        let mut strides = [0u32; 8];
        for (k, a) in attributes.iter().enumerate() {
            if !a.is_empty() {
                strides[k] = if up.is_some() && k == 0 {
                    up.map_or(0, |(_, s)| s)
                } else {
                    self.st.streams[k].stride
                };
            }
        }
        let varyings: Vec<wgsl::Semantic> = ps.sig.inputs.iter().map(|(_, s)| *s).collect();
        let samplers = self.sampler_bindings(&vs, &ps);
        let index32 = indexed && self.st.indices.as_ref().is_some_and(|b| b.index32);
        let depth = self.usable_depth();
        let key = PipeKey {
            vs: vs.id,
            ps: ps.id,
            decl: decl.id,
            strides,
            topology: kind as u8,
            index32,
            colours: std::array::from_fn(|i| {
                self.st.rt[i]
                    .as_ref()
                    .map(|s| (s.format.wgpu, s.format.opaque))
            }),
            depth: depth.as_ref().map(|d| d.format.wgpu),
            states: PIPELINE_STATES.map(|s| self.st.rs[s]),
            samples: self
                .st
                .rt
                .iter()
                .flatten()
                .next()
                .map(|s| s.samples)
                .or(depth.as_ref().map(|d| d.samples))
                .unwrap_or(1),
            samplers: samplers.iter().map(|(k, _)| *k).collect(),
        };
        if !self.pipelines.contains_key(&key) {
            // Bounded: render states with continuous values (depth bias) could grow it without end.
            if self.pipelines.len() >= 4096 {
                self.pipelines.clear();
            }
            let pipeline =
                self.make_pipeline(&key, &vs, &ps, &inputs, &varyings, &attributes, topology);
            self.pipelines.insert(key.clone(), pipeline);
        }
        let Some(pipeline) = self.pipelines.get(&key).cloned().flatten() else {
            self.skipped += 1;
            return;
        };
        // The textures and samplers.
        let mut group_key = Vec::with_capacity(samplers.len());
        for &((binding, _, filterable), slot) in &samplers {
            let tex_id = self.st.textures[slot].as_ref().map_or(0, |t| t.id);
            let sk = SamplerKey::new(&self.st.samplers[slot], filterable);
            group_key.push((binding, tex_id, Some(sk)));
        }
        if !self.groups.contains_key(&group_key) {
            if self.groups.len() > 8192 {
                self.groups.clear();
            }
            let (layout, _) = self
                .sampler_layouts
                .get(&key.samplers)
                .expect("made with the pipeline");
            let layout = layout.clone();
            let mut views = Vec::new();
            let mut samps = Vec::new();
            for &((binding, dim, filterable), slot) in &samplers {
                let view = match &self.st.textures[slot] {
                    Some(t) if (dim == 1) == t.cube => t.view.clone(),
                    _ => self.dummy[dim as usize].clone(),
                };
                let sk = SamplerKey::new(&self.st.samplers[slot], filterable);
                views.push((binding, view));
                samps.push((binding + 1, self.sampler(sk)));
            }
            let mut entries = Vec::new();
            for (b, v) in &views {
                entries.push(wgpu::BindGroupEntry {
                    binding: *b,
                    resource: wgpu::BindingResource::TextureView(v),
                });
            }
            for (b, s) in &samps {
                entries.push(wgpu::BindGroupEntry {
                    binding: *b,
                    resource: wgpu::BindingResource::Sampler(s),
                });
            }
            let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &entries,
            });
            self.groups.insert(group_key.clone(), group);
        }
        let group = self.groups[&group_key].clone();
        // Vertices from DrawPrimitiveUP go into the batch's ring.
        let up_offset = if let Some((bytes, _)) = up {
            if self.up.len() + bytes.len() > UP_RING as usize {
                self.flush();
            }
            let at = self.up.len();
            self.up.extend_from_slice(bytes);
            self.up.resize(self.up.len().div_ceil(4) * 4, 0);
            Some(at as u64)
        } else {
            None
        };
        self.write_blocks();
        self.begin_pass(None);
        let v = self.st.viewport;
        let (tw, th) = self
            .st
            .rt
            .iter()
            .flatten()
            .next()
            .map(|s| (s.width, s.height))
            .or(depth.as_ref().map(|d| (d.width, d.height)))
            .unwrap_or((1, 1));
        let vx = v.x.min(tw.saturating_sub(1));
        let vy = v.y.min(th.saturating_sub(1));
        let vw = v.w.min(tw - vx).max(1);
        let vh = v.h.min(th - vy).max(1);
        let scissor = if self.st.rs[rs::SCISSORTESTENABLE] != 0 {
            let r = self.st.scissor;
            let x0 = r[0].clamp(0, tw as i32) as u32;
            let y0 = r[1].clamp(0, th as i32) as u32;
            let x1 = r[2].clamp(x0 as i32, tw as i32) as u32;
            let y1 = r[3].clamp(y0 as i32, th as i32) as u32;
            (x0, y0, x1 - x0, y1 - y0)
        } else {
            (0, 0, tw, th)
        };
        if scissor.2 == 0 || scissor.3 == 0 {
            return;
        }
        let (vs_block, ps_block) = self.blocks;
        let stencil_ref = self.st.rs[rs::STENCILREF] & 0xFF;
        let bf = self.st.rs[rs::BLENDFACTOR];
        let blend_constant = wgpu::Color {
            r: f64::from((bf >> 16) & 0xFF) / 255.0,
            g: f64::from((bf >> 8) & 0xFF) / 255.0,
            b: f64::from(bf & 0xFF) / 255.0,
            a: f64::from(bf >> 24) / 255.0,
        };
        let mut streams = Vec::new();
        for (k, a) in attributes.iter().enumerate() {
            if a.is_empty() {
                continue;
            }
            if up.is_some() && k == 0 {
                streams.push((k as u32, self.up_buffer.clone(), up_offset.unwrap_or(0)));
                continue;
            }
            match &self.st.streams[k].buffer {
                Some(b) => {
                    let offset = u64::from(self.st.streams[k].offset);
                    // An offset past the buffer's end (wgpu would refuse the whole pass): the draw
                    // is skipped, as a bad call is.
                    if offset >= b.buffer.size() {
                        self.warn("a stream offset past its buffer's end: the draw is skipped");
                        self.skipped += 1;
                        return;
                    }
                    b.batch.store(self.batch, Ordering::Relaxed);
                    streams.push((k as u32, b.buffer.clone(), offset));
                }
                None => {
                    self.warn("a draw with a stream the declaration uses unbound: skipped");
                    self.skipped += 1;
                    return;
                }
            }
        }
        let n = Self::vertex_count(kind, count);
        let index = if indexed {
            match &self.st.indices {
                Some(b) => {
                    // Indices past the buffer's end: skipped (wgpu would refuse the pass).
                    let size = if b.index32 { 4 } else { 2 };
                    if (u64::from(start) + u64::from(n)) * size > b.buffer.size() {
                        self.warn("a draw reading indices past its buffer's end: skipped");
                        self.skipped += 1;
                        return;
                    }
                    b.batch.store(self.batch, Ordering::Relaxed);
                    Some((b.buffer.clone(), b.index32))
                }
                None => {
                    self.skipped += 1;
                    return;
                }
            }
        } else {
            None
        };
        // The textures are this batch's from here (after any send above), so an upload into one
        // later in the batch sends the batch first.
        for &(_, slot) in &samplers {
            if let Some(t) = &self.st.textures[slot] {
                t.batch.store(self.batch, Ordering::Relaxed);
            }
        }
        let pass = self.pass.as_mut().expect("a pass is open");
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &self.ring_group, &[vs_block, ps_block]);
        pass.set_bind_group(1, &group, &[]);
        pass.set_viewport(
            vx as f32,
            vy as f32,
            vw as f32,
            vh as f32,
            v.min_z.clamp(0.0, 1.0),
            v.max_z.clamp(0.0, 1.0),
        );
        pass.set_scissor_rect(scissor.0, scissor.1, scissor.2, scissor.3);
        pass.set_stencil_reference(stencil_ref);
        pass.set_blend_constant(blend_constant);
        for (slot, buffer, offset) in &streams {
            pass.set_vertex_buffer(*slot, buffer.slice(*offset..));
        }
        match index {
            Some((buffer, index32)) => {
                pass.set_index_buffer(
                    buffer.slice(..),
                    if index32 {
                        wgpu::IndexFormat::Uint32
                    } else {
                        wgpu::IndexFormat::Uint16
                    },
                );
                pass.draw_indexed(start..start + n, base_vertex, 0..1);
            }
            None => pass.draw(start..start + n, 0..1),
        }
        self.draws += 1;
    }

    #[allow(clippy::too_many_arguments)]
    fn make_pipeline(
        &mut self,
        key: &PipeKey,
        vs: &Shader,
        ps: &Shader,
        inputs: &[wgsl::Input],
        varyings: &[wgsl::Semantic],
        attributes: &[Vec<wgpu::VertexAttribute>; 8],
        topology: wgpu::PrimitiveTopology,
    ) -> Option<wgpu::RenderPipeline> {
        let vkey = (
            vs.id,
            inputs.to_vec(),
            varyings.to_vec(),
            ps.sig.centroid.clone(),
        );
        if !self.vs_modules.contains_key(&vkey) {
            let link = wgsl::Link {
                inputs: inputs.to_vec(),
                varyings: varyings.to_vec(),
                centroid: ps.sig.centroid.clone(),
            };
            let m = self.module(wgsl::convert(&vs.program, &link), "a vertex shader");
            self.vs_modules.insert(vkey.clone(), m);
        }
        let pkey = (ps.id, varyings.to_vec());
        if !self.ps_modules.contains_key(&pkey) {
            let link = wgsl::Link {
                inputs: Vec::new(),
                varyings: varyings.to_vec(),
                centroid: ps.sig.centroid.clone(),
            };
            let m = self.module(wgsl::convert(&ps.program, &link), "a pixel shader");
            self.ps_modules.insert(pkey.clone(), m);
        }
        let vm = self.vs_modules[&vkey].clone()?;
        let pm = self.ps_modules[&pkey].clone()?;
        let layout = self.sampler_layout(&key.samplers);
        // One layout per stream slot up to the last used (a slot's index is its stream's number).
        let last = attributes
            .iter()
            .rposition(|a| !a.is_empty())
            .map_or(0, |i| i + 1);
        let buffers: Vec<Option<wgpu::VertexBufferLayout>> = attributes[..last]
            .iter()
            .enumerate()
            .map(|(k, a)| {
                (!a.is_empty()).then(|| wgpu::VertexBufferLayout {
                    array_stride: u64::from(key.strides[k]),
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: a,
                })
            })
            .collect();
        let s = &self.st.rs;
        let blend = state::blend(s);
        let colour_states = [
            rs::COLORWRITEENABLE,
            rs::COLORWRITEENABLE1,
            rs::COLORWRITEENABLE2,
            rs::COLORWRITEENABLE3,
        ];
        let written = ps.sig.colours;
        let mut targets: Vec<Option<wgpu::ColorTargetState>> = (0..4)
            .map(|i| {
                key.colours[i].map(|(format, opaque)| wgpu::ColorTargetState {
                    format,
                    blend,
                    write_mask: if written & (1 << i) != 0 {
                        state::write_mask(s[colour_states[i]], opaque)
                    } else {
                        wgpu::ColorWrites::empty()
                    },
                })
            })
            .collect();
        while targets.last().is_some_and(|t| t.is_none()) {
            targets.pop();
        }
        let strip = matches!(
            topology,
            wgpu::PrimitiveTopology::TriangleStrip | wgpu::PrimitiveTopology::LineStrip
        );
        let desc = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vm,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers: &buffers,
            },
            primitive: wgpu::PrimitiveState {
                topology,
                strip_index_format: strip.then_some(if key.index32 {
                    wgpu::IndexFormat::Uint32
                } else {
                    wgpu::IndexFormat::Uint16
                }),
                front_face: wgpu::FrontFace::Cw,
                cull_mode: state::cull(s[rs::CULLMODE]),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: key.depth.map(|f| state::depth_stencil(s, f)),
            multisample: wgpu::MultisampleState {
                count: key.samples,
                ..Default::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &pm,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            multiview_mask: None,
            cache: None,
        };
        let scope = self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let pipeline = self.device.create_render_pipeline(&desc);
        match block_on(scope.pop()) {
            None => Some(pipeline),
            Some(e) => {
                self.warn(format!("a pipeline wgpu refuses: {e}"));
                None
            }
        }
    }

    // Copies.

    /// The copy pipeline into `format` (`opaque`: a target without alpha, whose alpha stays 1).
    fn blit_pipeline(
        &mut self,
        (format, opaque): (wgpu::TextureFormat, bool),
    ) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
        let layout = self
            .blit_layout
            .get_or_insert_with(|| {
                self.device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("blit"),
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    sample_type: wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    multisampled: false,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 2,
                                visibility: wgpu::ShaderStages::VERTEX,
                                ty: wgpu::BindingType::Buffer {
                                    ty: wgpu::BufferBindingType::Uniform,
                                    has_dynamic_offset: false,
                                    min_binding_size: None,
                                },
                                count: None,
                            },
                        ],
                    })
            })
            .clone();
        if let Some(p) = self.blit.get(&(format, opaque)) {
            return (p.clone(), layout);
        }
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("blit"),
                source: wgpu::ShaderSource::Wgsl(BLIT.into()),
            });
        let pl = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
        let p = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("blit"),
                layout: Some(&pl),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: state::write_mask(0xF, opaque),
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });
        self.blit.insert((format, opaque), p.clone());
        (p, layout)
    }

    /// Draws `source` (its rectangle in texels) into `target`'s rectangle, filtered or not.
    #[allow(clippy::too_many_arguments)]
    fn blit_into(
        &mut self,
        source: &wgpu::TextureView,
        source_size: (u32, u32),
        source_rect: [f32; 4],
        target: &wgpu::TextureView,
        format: (wgpu::TextureFormat, bool),
        target_rect: [f32; 4],
        linear: bool,
    ) {
        self.end_pass();
        let (pipeline, layout) = self.blit_pipeline(format);
        let sampler = self.sampler(SamplerKey {
            address: [3, 3, 3],
            mag: if linear { 2 } else { 1 },
            min: if linear { 2 } else { 1 },
            mip: 0,
            max_level: 0,
            anisotropy: 1,
            border: 0,
            filterable: true,
        });
        let (w, h) = (source_size.0.max(1) as f32, source_size.1.max(1) as f32);
        let uv = [
            source_rect[0] / w,
            source_rect[1] / h,
            source_rect[2] / w,
            source_rect[3] / h,
        ];
        let bytes: Vec<u8> = uv.iter().flat_map(|v| v.to_le_bytes()).collect();
        let ub = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: true,
        });
        if let Ok(mut mapped) = ub.slice(..).get_mapped_range_mut() {
            mapped.copy_from_slice(&bytes);
        }
        ub.unmap();
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(source),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: ub.as_entire_binding(),
                },
            ],
        });
        let encoder = self.encoder();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("blit"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.set_viewport(
            target_rect[0],
            target_rect[1],
            (target_rect[2] - target_rect[0]).max(1.0),
            (target_rect[3] - target_rect[1]).max(1.0),
            0.0,
            1.0,
        );
        pass.draw(0..3, 0..1);
    }

    pub(crate) fn stretch_rect(
        &mut self,
        source: Handle,
        source_rect: Option<Rect>,
        destination: Handle,
        destination_rect: Option<Rect>,
        filter: u32,
    ) {
        let (Some(src), Some(dst)) = (self.surface_of(source), self.surface_of(destination)) else {
            self.warn("StretchRect of a surface without a counterpart");
            return;
        };
        let (Some(sv), Some(dv)) = (src.view.clone(), dst.view.clone()) else {
            return;
        };
        if src.format.depth || dst.format.depth {
            if src.width == dst.width
                && src.height == dst.height
                && src.format.wgpu == dst.format.wgpu
            {
                self.end_pass();
                let (st, dt) = (
                    src.tex.clone().expect("a texture"),
                    dst.tex.clone().expect("a texture"),
                );
                self.encoder().copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &st.texture,
                        mip_level: src.level,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: &dt.texture,
                        mip_level: dst.level,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width: src.width,
                        height: src.height,
                        depth_or_array_layers: 1,
                    },
                );
            } else {
                self.warn("StretchRect of depth surfaces of different sizes is not provided");
            }
            return;
        }
        let sr = source_rect.unwrap_or([0, 0, src.width as i32, src.height as i32]);
        let dr = destination_rect.unwrap_or([0, 0, dst.width as i32, dst.height as i32]);
        if dst.samples > 1 {
            self.warn("StretchRect into a multisampled surface is not provided");
            return;
        }
        // A multisampled source is resolved first (the samples averaged, as Direct3D 9's
        // StretchRect does): straight into the destination when it is the same whole surface,
        // else into a scratch texture that is then copied as any other source.
        let (sv, src_size) = if src.samples > 1 {
            let whole_src = sr == [0, 0, src.width as i32, src.height as i32];
            let whole_dst = dr == [0, 0, dst.width as i32, dst.height as i32];
            if whole_src
                && whole_dst
                && (src.width, src.height) == (dst.width, dst.height)
                && src.format.wgpu == dst.format.wgpu
            {
                self.resolve(&sv, &dv);
                for s in [&src, &dst] {
                    if let Some(t) = &s.tex {
                        t.batch.store(self.batch, Ordering::Relaxed);
                    }
                }
                return;
            }
            let Some(scratch) =
                self.new_texture(src.width, src.height, 1, src.format.d3d, false, true, 1)
            else {
                return;
            };
            let resolved = self.level_surface(&scratch, 0, 0);
            let rv = resolved.view.clone().expect("a view");
            self.resolve(&sv, &rv);
            (rv, (src.width, src.height))
        } else {
            (sv, (src.width, src.height))
        };
        let f = |r: Rect| r.map(|v| v as f32);
        self.blit_into(
            &sv,
            src_size,
            f(sr),
            &dv,
            (dst.format.wgpu, dst.format.opaque),
            f(dr),
            filter >= 2,
        );
        for s in [&src, &dst] {
            if let Some(t) = &s.tex {
                t.batch.store(self.batch, Ordering::Relaxed);
            }
        }
    }

    /// Resolves the multisampled `source` into `target` (same size and format): a render pass
    /// that only loads the samples and resolves them at its end.
    fn resolve(&mut self, source: &wgpu::TextureView, target: &wgpu::TextureView) {
        self.end_pass();
        let encoder = self.encoder();
        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("resolve"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: source,
                depth_slice: None,
                resolve_target: Some(target),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    pub(crate) fn update_texture(&mut self, source: Handle, destination: Handle) {
        let (Some(s), Some(d)) = (self.texture_of(source), self.texture_of(destination)) else {
            return;
        };
        // Direct3D 9 matches the levels by size: a source with more (larger) levels starts at the
        // one of the destination's top size.
        let Some(skip) = (0..s.levels)
            .find(|&l| ((s.width >> l).max(1), (s.height >> l).max(1)) == (d.width, d.height))
        else {
            self.warn("UpdateTexture between textures of no common level size: skipped");
            return;
        };
        if s.format.wgpu != d.format.wgpu || s.cube != d.cube {
            self.warn("UpdateTexture between different formats: skipped");
            return;
        }
        self.end_pass();
        let levels = (s.levels - skip).min(d.levels);
        let layers = if s.cube { 6 } else { 1 };
        for level in 0..levels {
            let w = (d.width >> level).max(1);
            let h = (d.height >> level).max(1);
            let (w, h) = if s.format.block {
                (w.div_ceil(4) * 4, h.div_ceil(4) * 4)
            } else {
                (w, h)
            };
            self.encoder().copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &s.texture,
                    mip_level: level + skip,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &d.texture,
                    mip_level: level,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: layers,
                },
            );
        }
    }

    // Present.

    /// `Present` of the stream's `frame` (numbered as the forwarding layer's captures are).
    pub(crate) fn present(&mut self, frame: u32) {
        self.end_pass();
        self.presents += 1;
        let frame = u64::from(frame);
        let back = self.back_buffers.first().and_then(|&h| self.surface_of(h));
        let shot = match (&mut self.shots, &back) {
            (Some(s), Some(_)) => s.take(frame),
            _ => false,
        };
        let mut readback = None;
        if let (true, Some(b)) = (shot, &back) {
            let tex = b.tex.clone().expect("the back buffer's texture");
            let row = (b.width * 4).div_ceil(256) * 256;
            let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("shot"),
                size: u64::from(row * b.height),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            self.encoder().copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(row),
                        rows_per_image: Some(b.height),
                    },
                },
                wgpu::Extent3d {
                    width: b.width,
                    height: b.height,
                    depth_or_array_layers: 1,
                },
            );
            readback = Some((buffer, row, b.width, b.height, b.format));
        }
        let mut frame_texture = None;
        if let (Some((surface, config)), Some(b)) = (&self.surface, &back) {
            // An outdated or lost surface (the window resized, the display changed) is configured
            // again and asked once more; a frame that cannot be had now (minimised, timed out) is
            // skipped, as Direct3D 9's Present then shows nothing.
            let mut acquired = None;
            for _ in 0..2 {
                match surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(t) => acquired = Some(t),
                    wgpu::CurrentSurfaceTexture::Suboptimal(t) => {
                        acquired = Some(t);
                    }
                    wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                        surface.configure(&self.device, config);
                        continue;
                    }
                    _ => {}
                }
                break;
            }
            match acquired {
                Some(t) => {
                    let view = t.texture.create_view(&Default::default());
                    let format = config.format;
                    let (w, h) = (config.width as f32, config.height as f32);
                    let sv = b.view.clone().expect("the back buffer's view");
                    let size = (b.width, b.height);
                    frame_texture = Some(t);
                    self.blit_into(
                        &sv,
                        size,
                        [0.0, 0.0, size.0 as f32, size.1 as f32],
                        &view,
                        (format, false),
                        [0.0, 0.0, w, h],
                        false,
                    );
                }
                None => self.warn("a frame the window could not take (skipped)"),
            }
        }
        self.flush();
        if let Some(t) = frame_texture {
            self.queue.present(t);
        }
        if let Some((buffer, row, width, height, format)) = readback {
            buffer.slice(..).map_async(wgpu::MapMode::Read, |_| {});
            let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
            let mut pixels = Vec::with_capacity((width * height * 4) as usize);
            match buffer.slice(..).get_mapped_range() {
                Ok(data) => {
                    for y in 0..height {
                        let r = &data[(y * row) as usize..(y * row + width * 4) as usize];
                        pixels.extend_from_slice(r);
                    }
                }
                Err(e) => eprintln!("NV_SHOT wgpu: frame {frame} not read back: {e}"),
            }
            buffer.unmap();
            if format.wgpu == wgpu::TextureFormat::Rgba8Unorm {
                for p in pixels.chunks_exact_mut(4) {
                    p.swap(0, 2);
                }
            }
            if let (Some(s), true) = (&self.shots, pixels.len() == (width * height * 4) as usize) {
                let path = s.path(frame);
                if let Err(e) = std::fs::write(&path, shot::bmp(width, height, &pixels)) {
                    eprintln!("NV_SHOT wgpu: {} not written: {e}", path.display());
                }
            }
        }
        if self.presents % 600 == 1 {
            eprintln!(
                "NV_D3D9 wgpu: present {frame}: {} draws, {} skipped, {} pipelines, {} wgpu errors",
                self.draws,
                self.skipped,
                self.pipelines.len(),
                self.errors.load(Ordering::Relaxed)
            );
        }
    }

    /// The bytes of a back buffer or render target, read back (tests and `GetRenderTargetData`).
    pub fn read_surface_pixels(&mut self, surface: Handle) -> Option<(u32, u32, Vec<u8>)> {
        let s = self.surface_of(surface)?;
        let tex = s.tex.clone()?;
        self.end_pass();
        // The texture's own bytes per pixel (16-bit formats are widened; blocks are not read).
        if s.format.block || s.samples > 1 {
            return None;
        }
        let bpp = s.format.wgpu.block_copy_size(None)?;
        let row = (s.width * bpp).div_ceil(256) * 256;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(row * s.height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        self.encoder().copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &tex.texture,
                mip_level: s.level,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: s.layer,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row),
                    rows_per_image: Some(s.height),
                },
            },
            wgpu::Extent3d {
                width: s.width,
                height: s.height,
                depth_or_array_layers: 1,
            },
        );
        self.flush();
        buffer.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
        let mut out = Vec::new();
        {
            let data = buffer.slice(..).get_mapped_range().ok()?;
            for y in 0..s.height {
                out.extend_from_slice(
                    &data[(y * row) as usize..(y * row + s.width * bpp) as usize],
                );
            }
        }
        buffer.unmap();
        Some((s.width, s.height, out))
    }

    pub(crate) fn reset(&mut self, pp: &PresentParameters, back_buffers: &[Handle], depth: Handle) {
        self.flush();
        if let Some((surface, config)) = &mut self.surface {
            config.width = pp.width.max(1);
            config.height = pp.height.max(1);
            surface.configure(&self.device, config);
        }
        // Reset returns the device to its defaults: every state, binding and register; the caches
        // that hold old resources go.
        let st = &mut self.st;
        st.rt = Default::default();
        st.ds = None;
        st.rs = state::default_render_states(pp.auto_depth);
        *st.samplers = [state::default_sampler_states(); 20];
        st.textures = Default::default();
        st.vs = None;
        st.ps = None;
        st.decl = None;
        st.streams = Default::default();
        st.indices = None;
        *st.vsf = [[0; 4]; 256];
        st.vsi = [[0; 4]; 16];
        st.vsb = [0; 16];
        *st.psf = [[0; 4]; 256];
        st.psi = [[0; 4]; 16];
        st.psb = [0; 16];
        st.dirty = true;
        self.groups.clear();
        self.depth_parts.clear();
        for h in std::mem::take(&mut self.back_buffers) {
            self.objects.remove(&h);
        }
        if self.auto_depth != 0 {
            self.objects.remove(&self.auto_depth);
        }
        self.make_back_buffers(pp, back_buffers, depth);
    }
}

/// One stage's register block (`Regs` of the converted shaders) at the end of the ring: float,
/// integer and boolean registers, the system values, each sampler's fill mode and LOD bias. Its
/// offset.
fn write_block(
    ring: &mut Vec<u8>,
    f: &[[u32; 4]; 256],
    i: &[[u32; 4]; 16],
    b: &[u32; 16],
    sys: [f32; 4],
    (fix, bias): ([u32; 16], [f32; 16]),
) -> u32 {
    let at = ring.len();
    for w in f.iter().chain(i.iter()).flatten().chain(b.iter()) {
        ring.extend_from_slice(&w.to_le_bytes());
    }
    for v in sys {
        ring.extend_from_slice(&v.to_le_bytes());
    }
    ring.extend_from_slice(&[0u8; 48]);
    for w in fix {
        ring.extend_from_slice(&w.to_le_bytes());
    }
    for v in bias {
        ring.extend_from_slice(&v.to_le_bytes());
    }
    debug_assert_eq!((ring.len() - at) as u64, wgsl::REGS_SIZE);
    ring.resize(at + BLOCK as usize, 0);
    at as u32
}

/// The copy of one texture's rectangle into another's (`Present`, `StretchRect`).
const BLIT: &str = "
struct Rect { uv: vec4<f32> }
@group(0) @binding(0) var t: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;
@group(0) @binding(2) var<uniform> r: Rect;
struct V { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> }
@vertex fn vs(@builtin(vertex_index) i: u32) -> V {
    let c = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var v: V;
    v.pos = vec4<f32>(c.x * 2.0 - 1.0, 1.0 - c.y * 2.0, 0.0, 1.0);
    v.uv = mix(r.uv.xy, r.uv.zw, c);
    return v;
}
@fragment fn fs(v: V) -> @location(0) vec4<f32> {
    return textureSample(t, s, v.uv);
}
";

/// The textures an unbound sampler reads: 2D, cube and volume, each (0, 0, 0, 1) (a guess,
/// DESIGN.md).
fn dummies(device: &wgpu::Device, queue: &wgpu::Queue) -> [wgpu::TextureView; 3] {
    let make = |dimension, layers, view_dimension| {
        let t = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("unbound sampler"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        for layer in 0..layers {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &t,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &[0, 0, 0, 255],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );
        }
        t.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(view_dimension),
            ..Default::default()
        })
    };
    [
        make(
            wgpu::TextureDimension::D2,
            1,
            wgpu::TextureViewDimension::D2,
        ),
        make(
            wgpu::TextureDimension::D2,
            6,
            wgpu::TextureViewDimension::Cube,
        ),
        make(
            wgpu::TextureDimension::D3,
            1,
            wgpu::TextureViewDimension::D3,
        ),
    ]
}

/// The surface of a real window (Windows: an `HWND`).
#[cfg(windows)]
fn surface_for(instance: &wgpu::Instance, window: isize) -> Result<wgpu::Surface<'static>, String> {
    use raw_window_handle::{
        RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle,
    };
    let hwnd = std::num::NonZeroIsize::new(window).ok_or("no window")?;
    let mut handle = Win32WindowHandle::new(hwnd);
    // SAFETY: GetModuleHandleW(null) is this process's own module.
    let module = unsafe { GetModuleHandleW(std::ptr::null()) };
    handle.hinstance = std::num::NonZeroIsize::new(module);
    // SAFETY: the window lives as long as the game's device (the forwarding layer's window
    // thread owns it; it is never destroyed while the game runs).
    unsafe {
        instance
            .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: Some(RawDisplayHandle::Windows(WindowsDisplayHandle::new())),
                raw_window_handle: RawWindowHandle::Win32(handle),
            })
            .map_err(|e| format!("create_surface: {e}"))
    }
}

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> isize;
}

#[cfg(not(windows))]
fn surface_for(
    _instance: &wgpu::Instance,
    _window: isize,
) -> Result<wgpu::Surface<'static>, String> {
    Err("windows of other platforms come with the window layer's port".into())
}
