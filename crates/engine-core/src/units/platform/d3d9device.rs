//! The platform's Direct3D 9 device (ADR-0007): `IDirect3D9::CreateDevice` and the
//! `IDirect3DDevice9` it makes, with its surfaces, following the documented Direct3D 9
//! semantics (the DirectX 9.0c SDK documentation of each method). Answers that depend on the
//! player's hardware come from the machine table (`super::d3d9`, `scripts/d3d9-capture`).
//!
//! As in `super::d3d9`, each object lives in engine memory with its vtable pointer at `+0`, each
//! provided slot a platform function at a code address from [`DEVICE_CODE`] (the device) or
//! [`SURFACE_CODE`] (surfaces); a slot not provided is left unregistered, so a call to it stops
//! the run as an open function with its address (slot = (address - the code base) / 4, named
//! in [`DEVICE_SLOTS`] and [`SURFACE_SLOTS`]). Nothing is drawn yet: the viewer still draws.
//!
//! Provided so far, as the game's start-up reaches them: windowed devices of the HAL type with
//! hardware or mixed vertex processing, without multisampling (anything else stops the run as
//! not provided); the device's reference counting, `GetDirect3D`, `GetDeviceCaps`,
//! `GetDisplayMode`, `GetCreationParameters`, `GetBackBuffer`, the render target and
//! depth-stencil slots, the viewport and `CreateTexture`; a surface's reference counting,
//! `GetDevice`, `GetType`, `GetContainer`, `GetDesc` and `LockRect`/`UnlockRect`; a texture's
//! reference counting, priority, LOD, levels, their descriptions, surfaces and locks.
//! Measured with the retail runtime (`scripts/d3d9-capture device fixed`): `SetMaterial`, `SetLight`,
//! `LightEnable`, `SetSoftwareVertexProcessing`, `UpdateTexture` and `StretchRect`.

use super::d3d9::{CODE_BASE, D3DERR_INVALIDCALL, D3D_OK, E_NOINTERFACE};
use crate::{Engine, Ptr};

/// Where the device's slots sit (after `IDirect3D9`'s).
pub const DEVICE_CODE: u32 = CODE_BASE + 0x0200;
/// Where a surface's slots sit.
pub const SURFACE_CODE: u32 = CODE_BASE + 0x0400;
/// Where a texture's slots sit.
pub const TEXTURE_CODE: u32 = CODE_BASE + 0x0500;
/// `IDirect3DCubeTexture9`'s slots (the texture's names, with `GetCubeMapSurface` at 18).
pub const CUBE_CODE: u32 = CODE_BASE + 0x0900;

/// `D3DERR_NOTFOUND` (d3d9.h).
pub const D3DERR_NOTFOUND: u32 = 0x8876_0866;

/// `IDirect3DDevice9`'s slots (d3d9.h order).
pub const DEVICE_SLOTS: [&str; 119] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "TestCooperativeLevel",
    "GetAvailableTextureMem",
    "EvictManagedResources",
    "GetDirect3D",
    "GetDeviceCaps",
    "GetDisplayMode",
    "GetCreationParameters",
    "SetCursorProperties",
    "SetCursorPosition",
    "ShowCursor",
    "CreateAdditionalSwapChain",
    "GetSwapChain",
    "GetNumberOfSwapChains",
    "Reset",
    "Present",
    "GetBackBuffer",
    "GetRasterStatus",
    "SetDialogBoxMode",
    "SetGammaRamp",
    "GetGammaRamp",
    "CreateTexture",
    "CreateVolumeTexture",
    "CreateCubeTexture",
    "CreateVertexBuffer",
    "CreateIndexBuffer",
    "CreateRenderTarget",
    "CreateDepthStencilSurface",
    "UpdateSurface",
    "UpdateTexture",
    "GetRenderTargetData",
    "GetFrontBufferData",
    "StretchRect",
    "ColorFill",
    "CreateOffscreenPlainSurface",
    "SetRenderTarget",
    "GetRenderTarget",
    "SetDepthStencilSurface",
    "GetDepthStencilSurface",
    "BeginScene",
    "EndScene",
    "Clear",
    "SetTransform",
    "GetTransform",
    "MultiplyTransform",
    "SetViewport",
    "GetViewport",
    "SetMaterial",
    "GetMaterial",
    "SetLight",
    "GetLight",
    "LightEnable",
    "GetLightEnable",
    "SetClipPlane",
    "GetClipPlane",
    "SetRenderState",
    "GetRenderState",
    "CreateStateBlock",
    "BeginStateBlock",
    "EndStateBlock",
    "SetClipStatus",
    "GetClipStatus",
    "GetTexture",
    "SetTexture",
    "GetTextureStageState",
    "SetTextureStageState",
    "GetSamplerState",
    "SetSamplerState",
    "ValidateDevice",
    "SetPaletteEntries",
    "GetPaletteEntries",
    "SetCurrentTexturePalette",
    "GetCurrentTexturePalette",
    "SetScissorRect",
    "GetScissorRect",
    "SetSoftwareVertexProcessing",
    "GetSoftwareVertexProcessing",
    "SetNPatchMode",
    "GetNPatchMode",
    "DrawPrimitive",
    "DrawIndexedPrimitive",
    "DrawPrimitiveUP",
    "DrawIndexedPrimitiveUP",
    "ProcessVertices",
    "CreateVertexDeclaration",
    "SetVertexDeclaration",
    "GetVertexDeclaration",
    "SetFVF",
    "GetFVF",
    "CreateVertexShader",
    "SetVertexShader",
    "GetVertexShader",
    "SetVertexShaderConstantF",
    "GetVertexShaderConstantF",
    "SetVertexShaderConstantI",
    "GetVertexShaderConstantI",
    "SetVertexShaderConstantB",
    "GetVertexShaderConstantB",
    "SetStreamSource",
    "GetStreamSource",
    "SetStreamSourceFreq",
    "GetStreamSourceFreq",
    "SetIndices",
    "GetIndices",
    "CreatePixelShader",
    "SetPixelShader",
    "GetPixelShader",
    "SetPixelShaderConstantF",
    "GetPixelShaderConstantF",
    "SetPixelShaderConstantI",
    "GetPixelShaderConstantI",
    "SetPixelShaderConstantB",
    "GetPixelShaderConstantB",
    "DrawRectPatch",
    "DrawTriPatch",
    "DeletePatch",
    "CreateQuery",
];

/// `IDirect3DSurface9`'s slots (d3d9.h order: `IUnknown`, `IDirect3DResource9`, its own).
pub const SURFACE_SLOTS: [&str; 17] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetDevice",
    "SetPrivateData",
    "GetPrivateData",
    "FreePrivateData",
    "SetPriority",
    "GetPriority",
    "PreLoad",
    "GetType",
    "GetContainer",
    "GetDesc",
    "LockRect",
    "UnlockRect",
    "GetDC",
    "ReleaseDC",
];

/// `IDirect3DTexture9`'s slots (d3d9.h order: `IUnknown`, `IDirect3DResource9`,
/// `IDirect3DBaseTexture9`, its own).
pub const TEXTURE_SLOTS: [&str; 22] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetDevice",
    "SetPrivateData",
    "GetPrivateData",
    "FreePrivateData",
    "SetPriority",
    "GetPriority",
    "PreLoad",
    "GetType",
    "SetLOD",
    "GetLOD",
    "GetLevelCount",
    "SetAutoGenFilterType",
    "GetAutoGenFilterType",
    "GenerateMipSubLevels",
    "GetLevelDesc",
    "GetSurfaceLevel",
    "LockRect",
    "UnlockRect",
    "AddDirtyRect",
];

/// `D3DDEVTYPE_HAL`, the vertex-processing behaviour flags (`D3DCREATE_SOFTWARE_`,
/// `HARDWARE_`, `MIXED_VERTEXPROCESSING`), `D3DFMT_UNKNOWN`, `D3DRTYPE_SURFACE`, the surface
/// usages `D3DUSAGE_RENDERTARGET` and `D3DUSAGE_DEPTHSTENCIL`, `D3DPRESENT_BACK_BUFFERS_MAX`.
const D3DDEVTYPE_HAL: u32 = 1;
const SOFTWARE_VERTEXPROCESSING: u32 = 0x20;
const HARDWARE_VERTEXPROCESSING: u32 = 0x40;
const MIXED_VERTEXPROCESSING: u32 = 0x80;
const D3DFMT_UNKNOWN: u32 = 0;
const D3DRTYPE_SURFACE: u32 = 1;
const D3DUSAGE_RENDERTARGET: u32 = 1;
const D3DUSAGE_DEPTHSTENCIL: u32 = 2;
const BACK_BUFFERS_MAX: u32 = 3;
/// `D3DPRESENT_PARAMETERS` (0x38 bytes) fields.
const PP_WIDTH: u32 = 0x0;
const PP_HEIGHT: u32 = 0x4;
const PP_FORMAT: u32 = 0x8;
const PP_COUNT: u32 = 0xc;
const PP_MULTISAMPLE: u32 = 0x10;
const PP_QUALITY: u32 = 0x14;
const PP_WINDOW: u32 = 0x1c;
const PP_WINDOWED: u32 = 0x20;
const PP_AUTO_DEPTH: u32 = 0x24;
const PP_DEPTH_FORMAT: u32 = 0x28;
const PP_REFRESH: u32 = 0x30;
const PRESENT_SIZE: u32 = 0x38;
/// `D3DCAPS9::NumSimultaneousRTs`.
const CAPS_SIMULTANEOUS_RTS: usize = 0xf0;
/// The render target slots a device keeps (`D3D_MAX_SIMULTANEOUS_RENDERTARGETS`).
const RENDER_TARGETS: usize = 4;

/// A GUID's 16 bytes as they lie in memory (`Data1`..`Data3` little-endian, then `Data4`).
pub(super) const fn guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> [u8; 16] {
    let a = d1.to_le_bytes();
    let b = d2.to_le_bytes();
    let c = d3.to_le_bytes();
    [
        a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d4[0], d4[1], d4[2], d4[3], d4[4], d4[5],
        d4[6], d4[7],
    ]
}

/// The interface identifiers (unknwn.h, d3d9.h).
pub(super) const IID_IUNKNOWN: [u8; 16] = guid(0, 0, 0, [0xc0, 0, 0, 0, 0, 0, 0, 0x46]);
const IID_DEVICE9: [u8; 16] = guid(
    0xd022_3b96,
    0xbf7a,
    0x43fd,
    [0x92, 0xbd, 0xa4, 0x3b, 0x0d, 0x82, 0xb9, 0xeb],
);
const IID_RESOURCE9: [u8; 16] = guid(
    0x05ee_c05d,
    0x8f7d,
    0x4362,
    [0xb9, 0x99, 0xd1, 0xba, 0xf3, 0x57, 0xc7, 0x04],
);
const IID_BASETEXTURE9: [u8; 16] = guid(
    0x580c_a87e,
    0x1d3c,
    0x4d54,
    [0x99, 0x1d, 0xb7, 0xd3, 0xe3, 0xc2, 0x98, 0xce],
);
const IID_TEXTURE9: [u8; 16] = guid(
    0x85c3_1227,
    0x3de5,
    0x4f00,
    [0x9b, 0x3a, 0xf1, 0x1a, 0xc3, 0x8c, 0x18, 0xb5],
);
const IID_SURFACE9: [u8; 16] = guid(
    0x0cfb_af3a,
    0x9ff6,
    0x429a,
    [0x99, 0xb3, 0xa2, 0x79, 0x6a, 0xf8, 0xb8, 0x9b],
);

/// A device of this layer.
#[derive(Debug, Clone)]
struct Device {
    references: u32,
    direct3d: u32,
    /// `D3DDEVICE_CREATION_PARAMETERS`: adapter, device type, focus window, behaviour flags.
    creation: [u32; 4],
    /// The present parameters as the device took them (after filling in the defaults).
    present: Vec<u8>,
    back_buffers: Vec<u32>,
    render_targets: [u32; RENDER_TARGETS],
    depth_stencil: u32,
    /// `D3DVIEWPORT9`: X, Y, Width, Height, then MinZ and MaxZ as `float` bits.
    viewport: [u32; 6],
    /// The states the program set: render states by type, texture stage states and sampler
    /// states by (stage or sampler, type), the textures by sampler (held, counted).
    /// The last gamma ramp `SetGammaRamp` was given (three 256-entry WORD tables, 0x600 bytes).
    gamma_ramp: Option<Vec<u16>>,
    render_states: States<RENDER_STATES>,
    stage_states: States<STAGE_STATES>,
    sampler_states: States<SAMPLER_STATES>,
    textures: crate::engine::AddrMap<u32, u32>,
    /// Transforms by `D3DTRANSFORMSTATETYPE`, the scissor rectangle, user clip planes, and
    /// whether a scene is open.
    transforms: crate::engine::AddrMap<u32, [u32; 16]>,
    scissor: [u32; 4],
    clip_planes: crate::engine::AddrMap<u32, [u32; 4]>,
    in_scene: bool,
    /// The fixed-function material (`D3DMATERIAL9`, 17 words), the lights by index
    /// (`D3DLIGHT9`, 26 words), the enabled ones, and the software vertex processing switch.
    material: Option<[u32; 17]>,
    lights: crate::engine::AddrMap<u32, [u32; 26]>,
    enabled_lights: std::collections::BTreeSet<u32>,
    software_vertex_processing: bool,
    /// After a `Reset` that failed: the device needs a successful `Reset` (`TestCooperativeLevel`
    /// answers `D3DERR_DEVICENOTRESET`; scenes and `Present` are refused).
    lost: bool,
}

/// The render states kept (numbers 0 to 255), the texture stage states (8 stages of types 1 to
/// 32) and the sampler states (21 samplers, [`sampler_index`], of types 1 to 13).
const RENDER_STATES: usize = 256;
const STAGE_STATES: usize = 8 * 32;
const SAMPLER_STATES: usize = 21 * 13;

/// States by number: each value the program set, `None` for one never set. A table rather than
/// a map: these are set thousands of times a frame.
#[derive(Debug, Clone)]
struct States<const N: usize>(Box<[Option<u32>; N]>);

impl<const N: usize> Default for States<N> {
    fn default() -> Self {
        States(Box::new([None; N]))
    }
}

impl<const N: usize> States<N> {
    fn get(&self, index: Option<usize>) -> Option<u32> {
        index.and_then(|i| self.0.get(i).copied().flatten())
    }
    fn set(&mut self, index: usize, value: u32) {
        self.0[index] = Some(value);
    }
    fn clear(&mut self) {
        self.0.fill(None);
    }
    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.0.iter().all(Option::is_none)
    }
}

/// The place of texture stage state `kind` of `stage` (0 to 7) in [`Device::stage_states`];
/// `None` for a type past 1 to 32.
fn stage_index(stage: u32, kind: u32) -> Option<usize> {
    (1..=32)
        .contains(&kind)
        .then(|| stage as usize * 32 + kind as usize - 1)
}

/// The place of sampler state `kind` of `sampler` (one [`is_sampler`] names) in
/// [`Device::sampler_states`]: samplers 0 to 15 then 256 to 260; `None` for a type past 1 to 13.
fn sampler_index(sampler: u32, kind: u32) -> Option<usize> {
    let s = if sampler < 16 { sampler } else { sampler - 240 };
    (1..=13)
        .contains(&kind)
        .then(|| s as usize * 13 + kind as usize - 1)
}

/// A surface of this layer.
#[derive(Debug, Clone)]
struct Surface {
    /// References the program holds (`AddRef`/`Release`).
    references: u32,
    device: u32,
    /// Whether the device owns it (the back buffers and the automatic depth-stencil surface):
    /// it stays while the device lives, whatever the program's count.
    implicit: bool,
    /// `D3DSURFACE_DESC`: Format, Type, Usage, Pool, MultiSampleType, MultiSampleQuality,
    /// Width, Height.
    desc: [u32; 8],
    /// The texture whose level this is (0 for none): its references are the texture's.
    container: u32,
    /// The surface's memory for `LockRect` (0 for none) and its pitch.
    bits: u32,
    pitch: u32,
    locked: bool,
}

/// The devices and their resources (`Kernel::graphics`).
#[derive(Debug, Default)]
pub struct Objects {
    device_vtable: u32,
    surface_vtable: u32,
    texture_vtable: u32,
    cube_vtable: u32,
    devices: crate::engine::AddrMap<u32, Device>,
    surfaces: crate::engine::AddrMap<u32, Surface>,
    textures: crate::engine::AddrMap<u32, Texture>,
    /// The buffers, shaders and declarations (`super::d3d9resource`).
    pub(super) resources: super::d3d9resource::Resources,
    /// The queries (`super::d3d9query`).
    pub(super) queries: super::d3d9query::Queries,
}

fn vtable(e: &mut Engine, base: u32, slots: usize) -> u32 {
    let table = e.mem.alloc(4 * slots as u32);
    for i in 0..slots as u32 {
        e.mem.set_u32(table + 4 * i, base + 4 * i);
    }
    table
}

/// A word of a call's arguments, as the real side (`super::d3d9real`) takes it.
pub(super) trait Word: Copy {
    fn word(self) -> u32;
}
impl Word for u32 {
    fn word(self) -> u32 {
        self
    }
}
impl Word for Ptr {
    fn word(self) -> u32 {
        self.addr()
    }
}

/// The slots that change what is drawn: the model's answer, and with `NV_D3D9=real` the same
/// call made on the real device (ADR-0008, `super::d3d9real`), which gets the model's answer
/// to compare.
macro_rules! forwarded {
    ($($vis:vis fn $name:ident = $model:ident($($a:ident: $t:ty),*) => $real:ident;)*) => {$(
        $vis fn $name(e: &mut Engine, this: Ptr, $($a: $t),*) -> u32 {
            let model = $model(e, this, $($a),*);
            // A call the model refuses changes nothing, so the real device does not get it.
            #[cfg(windows)]
            if model == D3D_OK {
                super::d3d9real::$real(e, this.addr(), $(Word::word($a),)* model);
            }
            model
        }
    )*};
}
pub(super) use forwarded;

forwarded! {
    fn fwd_set_render_target = set_render_target(index: u32, surface: u32) => set_render_target;
    fn fwd_set_depth_stencil_surface = set_depth_stencil_surface(surface: u32)
        => set_depth_stencil_surface;
    fn fwd_set_viewport = set_viewport(viewport: Ptr) => set_viewport;
    fn fwd_set_transform = set_transform(state: u32, matrix: Ptr) => set_transform;
    fn fwd_set_scissor_rect = set_scissor_rect(rect: Ptr) => set_scissor_rect;
    fn fwd_set_clip_plane = set_clip_plane(index: u32, plane: Ptr) => set_clip_plane;
    fn fwd_set_render_state = set_render_state(state: u32, value: u32) => set_render_state;
    fn fwd_set_gamma_ramp = set_gamma_ramp(swap_chain: u32, flags: u32, ramp: Ptr)
        => set_gamma_ramp;
    fn fwd_set_texture_stage_state = set_texture_stage_state(stage: u32, kind: u32, value: u32)
        => set_texture_stage_state;
    fn fwd_set_sampler_state = set_sampler_state(sampler: u32, kind: u32, value: u32)
        => set_sampler_state;
    fn fwd_set_texture = set_texture(sampler: u32, texture: u32) => set_texture;
    fn fwd_set_material = set_material(material: Ptr) => set_material;
    fn fwd_set_light = set_light(index: u32, light: Ptr) => set_light;
    fn fwd_light_enable = light_enable(index: u32, enable: u32) => light_enable;
    fn fwd_set_software_vertex_processing = set_software_vertex_processing(software: u32)
        => set_software_vertex_processing;
    fn fwd_update_texture = update_texture(source: u32, destination: u32) => update_texture;
    fn fwd_stretch_rect = stretch_rect(
        source: u32, source_rect: u32, destination: u32, destination_rect: u32, filter: u32
    ) => stretch_rect;
}

/// `CreateTexture`, then (`NV_D3D9=real`) the real texture of the same arguments.
#[allow(clippy::too_many_arguments)]
fn fwd_create_texture(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    levels: u32,
    usage: u32,
    format: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    let model = create_texture(
        e, this, width, height, levels, usage, format, pool, out, shared,
    );
    #[cfg(windows)]
    if model == D3D_OK {
        let texture = e.mem.u32(out.addr());
        let surfaces = e.kernel.graphics.device_objects.textures[&texture]
            .levels
            .clone();
        super::d3d9real::texture_created(
            e,
            this.addr(),
            texture,
            &surfaces,
            width,
            height,
            usage,
            format,
            pool,
        );
    }
    model
}

/// `CreateCubeTexture`, then (`NV_D3D9=real`) the real cube texture of the same arguments.
#[allow(clippy::too_many_arguments)]
fn fwd_create_cube_texture(
    e: &mut Engine,
    this: Ptr,
    edge: u32,
    levels: u32,
    usage: u32,
    format: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    let model = create_cube_texture(e, this, edge, levels, usage, format, pool, out, shared);
    #[cfg(windows)]
    if model == D3D_OK {
        let texture = e.mem.u32(out.addr());
        let surfaces = e.kernel.graphics.device_objects.textures[&texture]
            .levels
            .clone();
        super::d3d9real::cube_created(
            e,
            this.addr(),
            texture,
            &surfaces,
            edge,
            usage,
            format,
            pool,
        );
    }
    model
}

/// `CreateRenderTarget` / `CreateDepthStencilSurface`, then (`NV_D3D9=real`) the real surface.
#[allow(clippy::too_many_arguments)]
fn fwd_create_surface(
    e: &mut Engine,
    this: Ptr,
    depth: bool,
    width: u32,
    height: u32,
    format: u32,
    multisample: u32,
    quality: u32,
    flag: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    let model = if depth {
        create_depth_stencil_surface(
            e,
            this,
            width,
            height,
            format,
            multisample,
            quality,
            flag,
            out,
            shared,
        )
    } else {
        create_render_target(
            e,
            this,
            width,
            height,
            format,
            multisample,
            quality,
            flag,
            out,
            shared,
        )
    };
    #[cfg(windows)]
    if model == D3D_OK {
        let surface = e.mem.u32(out.addr());
        super::d3d9real::surface_created(
            e,
            this.addr(),
            surface,
            depth,
            width,
            height,
            format,
            multisample,
            quality,
            flag,
        );
    }
    model
}

#[allow(clippy::too_many_arguments)]
fn fwd_create_render_target(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    format: u32,
    multisample: u32,
    quality: u32,
    lockable: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    fwd_create_surface(
        e,
        this,
        false,
        width,
        height,
        format,
        multisample,
        quality,
        lockable,
        out,
        shared,
    )
}

#[allow(clippy::too_many_arguments)]
fn fwd_create_depth_stencil_surface(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    format: u32,
    multisample: u32,
    quality: u32,
    discard: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    fwd_create_surface(
        e,
        this,
        true,
        width,
        height,
        format,
        multisample,
        quality,
        discard,
        out,
        shared,
    )
}

fn register_code(e: &mut Engine) {
    let device: [(u32, crate::AbiFn); 48] = [
        (16, crate::entry!(0, reset(Ptr, Ptr) -> u32).1),
        (
            32,
            crate::entry!(0, get_render_target_data(Ptr, u32, u32) -> u32).1,
        ),
        (35, crate::entry!(0, color_fill(Ptr, u32, Ptr, u32) -> u32).1),
        (
            36,
            crate::entry!(0, create_offscreen_plain_surface(Ptr, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            118,
            crate::entry!(0, super::d3d9query::create_query(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            0,
            crate::entry!(0, device_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, device_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, device_release(Ptr) -> u32).1),
        (6, crate::entry!(0, get_direct3d(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, get_device_caps(Ptr, Ptr) -> u32).1),
        (
            8,
            crate::entry!(0, get_display_mode(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            9,
            crate::entry!(0, get_creation_parameters(Ptr, Ptr) -> u32).1,
        ),
        (
            18,
            crate::entry!(0, get_back_buffer(Ptr, u32, u32, u32, Ptr) -> u32).1,
        ),
        (
            23,
            crate::entry!(0, fwd_create_texture(Ptr, u32, u32, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            25,
            crate::entry!(0, fwd_create_cube_texture(Ptr, u32, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            28,
            crate::entry!(0, fwd_create_render_target(Ptr, u32, u32, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            29,
            crate::entry!(0, fwd_create_depth_stencil_surface(Ptr, u32, u32, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            37,
            crate::entry!(0, fwd_set_render_target(Ptr, u32, u32) -> u32).1,
        ),
        (
            38,
            crate::entry!(0, get_render_target(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            39,
            crate::entry!(0, fwd_set_depth_stencil_surface(Ptr, u32) -> u32).1,
        ),
        (
            40,
            crate::entry!(0, get_depth_stencil_surface(Ptr, Ptr) -> u32).1,
        ),
        (47, crate::entry!(0, fwd_set_viewport(Ptr, Ptr) -> u32).1),
        (
            21,
            crate::entry!(0, fwd_set_gamma_ramp(Ptr, u32, u32, Ptr) -> u32).1,
        ),
        (3, crate::entry!(0, test_cooperative_level(Ptr) -> u32).1),
        (
            17,
            crate::entry!(0, present(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (41, crate::entry!(0, begin_scene(Ptr) -> u32).1),
        (42, crate::entry!(0, end_scene(Ptr) -> u32).1),
        (
            43,
            crate::entry!(0, clear(Ptr, u32, Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (44, crate::entry!(0, fwd_set_transform(Ptr, u32, Ptr) -> u32).1),
        (45, crate::entry!(0, get_transform(Ptr, u32, Ptr) -> u32).1),
        (55, crate::entry!(0, fwd_set_clip_plane(Ptr, u32, Ptr) -> u32).1),
        (75, crate::entry!(0, fwd_set_scissor_rect(Ptr, Ptr) -> u32).1),
        (76, crate::entry!(0, get_scissor_rect(Ptr, Ptr) -> u32).1),
        (
            57,
            crate::entry!(0, fwd_set_render_state(Ptr, u32, u32) -> u32).1,
        ),
        (
            58,
            crate::entry!(0, get_render_state(Ptr, u32, Ptr) -> u32).1,
        ),
        (64, crate::entry!(0, get_texture(Ptr, u32, Ptr) -> u32).1),
        (65, crate::entry!(0, fwd_set_texture(Ptr, u32, u32) -> u32).1),
        (
            66,
            crate::entry!(0, get_texture_stage_state(Ptr, u32, u32, Ptr) -> u32).1,
        ),
        (
            67,
            crate::entry!(0, fwd_set_texture_stage_state(Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            68,
            crate::entry!(0, get_sampler_state(Ptr, u32, u32, Ptr) -> u32).1,
        ),
        (
            69,
            crate::entry!(0, fwd_set_sampler_state(Ptr, u32, u32, u32) -> u32).1,
        ),
        (48, crate::entry!(0, get_viewport(Ptr, Ptr) -> u32).1),
        (49, crate::entry!(0, fwd_set_material(Ptr, Ptr) -> u32).1),
        (51, crate::entry!(0, fwd_set_light(Ptr, u32, Ptr) -> u32).1),
        (53, crate::entry!(0, fwd_light_enable(Ptr, u32, u32) -> u32).1),
        (
            77,
            crate::entry!(0, fwd_set_software_vertex_processing(Ptr, u32) -> u32).1,
        ),
        (
            31,
            crate::entry!(0, fwd_update_texture(Ptr, u32, u32) -> u32).1,
        ),
        (
            34,
            crate::entry!(0, fwd_stretch_rect(Ptr, u32, u32, u32, u32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in device {
        e.register(DEVICE_CODE + 4 * slot, f);
    }
    use super::d3d9resource as r;
    let resource: [(u32, crate::AbiFn); 32] = [
        (
            26,
            crate::entry!(0, r::create_vertex_buffer(Ptr, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            27,
            crate::entry!(0, r::create_index_buffer(Ptr, u32, u32, u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            86,
            crate::entry!(0, r::create_vertex_declaration(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            87,
            crate::entry!(0, r::fwd_set_vertex_declaration(Ptr, u32) -> u32).1,
        ),
        (89, crate::entry!(0, r::fwd_set_fvf(Ptr, u32) -> u32).1),
        (90, crate::entry!(0, r::get_fvf(Ptr, Ptr) -> u32).1),
        (
            91,
            crate::entry!(0, r::create_vertex_shader(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            92,
            crate::entry!(0, r::fwd_set_vertex_shader(Ptr, u32) -> u32).1,
        ),
        (
            93,
            crate::entry!(0, r::get_vertex_shader(Ptr, Ptr) -> u32).1,
        ),
        (
            94,
            crate::entry!(0, r::fwd_set_vertex_constant_f(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            95,
            crate::entry!(0, r::get_vertex_constant_f(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            96,
            crate::entry!(0, r::fwd_set_vertex_constant_i(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            97,
            crate::entry!(0, r::get_vertex_constant_i(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            98,
            crate::entry!(0, r::fwd_set_vertex_constant_b(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            99,
            crate::entry!(0, r::get_vertex_constant_b(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            100,
            crate::entry!(0, r::fwd_set_stream_source(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (
            101,
            crate::entry!(0, r::get_stream_source(Ptr, u32, Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            81,
            crate::entry!(0, r::fwd_draw_primitive(Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            82,
            crate::entry!(0, r::fwd_draw_indexed_primitive(Ptr, u32, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            83,
            crate::entry!(0, r::fwd_draw_primitive_up(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (
            88,
            crate::entry!(0, r::get_vertex_declaration(Ptr, Ptr) -> u32).1,
        ),
        (104, crate::entry!(0, r::fwd_set_indices(Ptr, u32) -> u32).1),
        (105, crate::entry!(0, r::get_indices(Ptr, Ptr) -> u32).1),
        (
            106,
            crate::entry!(0, r::create_pixel_shader(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            107,
            crate::entry!(0, r::fwd_set_pixel_shader(Ptr, u32) -> u32).1,
        ),
        (
            108,
            crate::entry!(0, r::get_pixel_shader(Ptr, Ptr) -> u32).1,
        ),
        (
            109,
            crate::entry!(0, r::fwd_set_pixel_constant_f(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            110,
            crate::entry!(0, r::get_pixel_constant_f(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            111,
            crate::entry!(0, r::fwd_set_pixel_constant_i(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            112,
            crate::entry!(0, r::get_pixel_constant_i(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            113,
            crate::entry!(0, r::fwd_set_pixel_constant_b(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            114,
            crate::entry!(0, r::get_pixel_constant_b(Ptr, u32, Ptr, u32) -> u32).1,
        ),
    ];
    for (slot, f) in resource {
        e.register(DEVICE_CODE + 4 * slot, f);
    }
    let surface: [(u32, crate::AbiFn); 9] = [
        (
            0,
            crate::entry!(0, surface_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, surface_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, surface_release(Ptr) -> u32).1),
        (3, crate::entry!(0, surface_get_device(Ptr, Ptr) -> u32).1),
        (10, crate::entry!(0, surface_get_type(Ptr) -> u32).1),
        (
            11,
            crate::entry!(0, surface_get_container(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (12, crate::entry!(0, surface_get_desc(Ptr, Ptr) -> u32).1),
        (
            13,
            crate::entry!(0, surface_lock_rect(Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (14, crate::entry!(0, surface_unlock_rect(Ptr) -> u32).1),
    ];
    for (slot, f) in surface {
        e.register(SURFACE_CODE + 4 * slot, f);
    }
    let texture: [(u32, crate::AbiFn); 15] = [
        (
            0,
            crate::entry!(0, texture_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, texture_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, texture_release(Ptr) -> u32).1),
        (3, crate::entry!(0, texture_get_device(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, texture_set_priority(Ptr, u32) -> u32).1),
        (8, crate::entry!(0, texture_get_priority(Ptr) -> u32).1),
        (9, crate::entry!(0, texture_pre_load(Ptr)).1),
        (10, crate::entry!(0, texture_get_type(Ptr) -> u32).1),
        (11, crate::entry!(0, fwd_texture_set_lod(Ptr, u32) -> u32).1),
        (12, crate::entry!(0, texture_get_lod(Ptr) -> u32).1),
        (13, crate::entry!(0, texture_get_level_count(Ptr) -> u32).1),
        (
            17,
            crate::entry!(0, texture_get_level_desc(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            18,
            crate::entry!(0, texture_get_surface_level(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            19,
            crate::entry!(0, texture_lock_rect(Ptr, u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (20, crate::entry!(0, texture_unlock_rect(Ptr, u32) -> u32).1),
    ];
    for (slot, f) in texture {
        e.register(TEXTURE_CODE + 4 * slot, f);
    }
    // `IDirect3DCubeTexture9`: the base-texture slots are the texture's (they read the same
    // object kind); its own slots take a face.
    let cube: [(u32, crate::AbiFn); 15] = [
        (
            0,
            crate::entry!(0, cube_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, texture_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, texture_release(Ptr) -> u32).1),
        (3, crate::entry!(0, texture_get_device(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, texture_set_priority(Ptr, u32) -> u32).1),
        (8, crate::entry!(0, texture_get_priority(Ptr) -> u32).1),
        (9, crate::entry!(0, texture_pre_load(Ptr)).1),
        (10, crate::entry!(0, cube_get_type(Ptr) -> u32).1),
        (11, crate::entry!(0, fwd_texture_set_lod(Ptr, u32) -> u32).1),
        (12, crate::entry!(0, texture_get_lod(Ptr) -> u32).1),
        (13, crate::entry!(0, texture_get_level_count(Ptr) -> u32).1),
        (
            17,
            crate::entry!(0, texture_get_level_desc(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            18,
            crate::entry!(0, cube_get_surface(Ptr, u32, u32, Ptr) -> u32).1,
        ),
        (
            19,
            crate::entry!(0, cube_lock_rect(Ptr, u32, u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (
            20,
            crate::entry!(0, cube_unlock_rect(Ptr, u32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in cube {
        e.register(CUBE_CODE + 4 * slot, f);
    }
    if e.kernel.graphics.device_objects.device_vtable == 0 {
        let device = vtable(e, DEVICE_CODE, DEVICE_SLOTS.len());
        let surface = vtable(e, SURFACE_CODE, SURFACE_SLOTS.len());
        let texture = vtable(e, TEXTURE_CODE, TEXTURE_SLOTS.len());
        let cube = vtable(e, CUBE_CODE, TEXTURE_SLOTS.len());
        let objects = &mut e.kernel.graphics.device_objects;
        objects.device_vtable = device;
        objects.surface_vtable = surface;
        objects.texture_vtable = texture;
        objects.cube_vtable = cube;
    }
}

fn not_provided(what: &str) -> ! {
    panic!("IDirect3D9::CreateDevice: {what} is not provided")
}

fn new_surface(e: &mut Engine, device: u32, implicit: bool, desc: [u32; 8]) -> u32 {
    let object = e.mem.alloc(8);
    let table = e.kernel.graphics.device_objects.surface_vtable;
    e.mem.set_u32(object, table);
    let references = u32::from(!implicit);
    e.kernel.graphics.device_objects.surfaces.insert(
        object,
        Surface {
            references,
            device,
            implicit,
            desc,
            container: 0,
            bits: 0,
            pitch: 0,
            locked: false,
        },
    );
    object
}

// Platform: replaces D3D9.DLL IDirect3D9::CreateDevice (vtable +0x40)
/// `CreateDevice(adapter, device type, focus window, behaviour flags, present parameters,
/// device out)`. A null present parameters or out, another adapter, or behaviour flags without
/// exactly one of the vertex-processing flags answer `D3DERR_INVALIDCALL`; so does a windowed
/// device with neither a device window nor a focus window, or with a full-screen refresh rate,
/// and a back buffer count above `D3DPRESENT_BACK_BUFFERS_MAX` (3; the count is set to 3, so a
/// second call can succeed). A count of 0 becomes 1 and the format `D3DFMT_UNKNOWN` the
/// display mode's format, written back into the caller's parameters as Direct3D does. The
/// device gets its back buffers (render-target surfaces of the parameters' size and format;
/// the first is render target 0), the automatic depth-stencil surface when asked for, the
/// viewport of the whole back buffer (depth 0 to 1), a reference to this `IDirect3D9`, and its
/// creation runs [`crate::units::platform::direct3d_device_created`] (the calling thread's
/// x87 precision). Not provided (the run stops): other device types, software vertex
/// processing, full-screen devices, a zero width or height (the window's client area),
/// multisampling, and a back buffer format the table's `CheckDeviceType` refuses.
#[allow(clippy::too_many_arguments)]
pub fn create_device(
    e: &mut Engine,
    this: Ptr,
    adapter: u32,
    device_type: u32,
    focus: u32,
    behavior: u32,
    present: Ptr,
    out: Ptr,
) -> u32 {
    if present.is_null() || out.is_null() || adapter != 0 {
        return D3DERR_INVALIDCALL;
    }
    let processing =
        behavior & (SOFTWARE_VERTEXPROCESSING | HARDWARE_VERTEXPROCESSING | MIXED_VERTEXPROCESSING);
    if processing.count_ones() != 1 {
        return D3DERR_INVALIDCALL;
    }
    if device_type != D3DDEVTYPE_HAL {
        not_provided(&format!("device type {device_type}"));
    }
    if processing == SOFTWARE_VERTEXPROCESSING {
        not_provided("software vertex processing");
    }
    let p = present.addr();
    let field = |e: &Engine, offset: u32| e.mem.u32(p + offset);
    if field(e, PP_WINDOWED) == 0 {
        not_provided("a full-screen device");
    }
    if field(e, PP_WINDOW) == 0 && focus == 0 {
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_REFRESH) != 0 {
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_COUNT) > BACK_BUFFERS_MAX {
        e.mem.set_u32(p + PP_COUNT, BACK_BUFFERS_MAX);
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_WIDTH) == 0 || field(e, PP_HEIGHT) == 0 {
        not_provided("a back buffer size taken from the window");
    }
    if field(e, PP_MULTISAMPLE) != 0 {
        not_provided("multisampling");
    }
    // The display mode's format (`D3DDISPLAYMODE` +0xC).
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(this.addr(), 4 * 8, &[0, mode.addr()]).u32();
        assert_eq!(hr, D3D_OK, "GetAdapterDisplayMode for CreateDevice");
        e.mem.u32(mode.addr() + 0xc)
    });
    if field(e, PP_COUNT) == 0 {
        e.mem.set_u32(p + PP_COUNT, 1);
    }
    if field(e, PP_FORMAT) == D3DFMT_UNKNOWN {
        e.mem.set_u32(p + PP_FORMAT, display_format);
    }
    let format = field(e, PP_FORMAT);
    let check = e
        .vcall(
            this.addr(),
            4 * 9,
            &[adapter, device_type, display_format, format, 1],
        )
        .u32();
    if check != D3D_OK {
        not_provided(&format!(
            "a windowed back buffer of format {format} (CheckDeviceType answers {check:#x})"
        ));
    }
    register_code(e);
    let object = e.mem.alloc(8);
    let table = e.kernel.graphics.device_objects.device_vtable;
    e.mem.set_u32(object, table);
    let (width, height) = (field(e, PP_WIDTH), field(e, PP_HEIGHT));
    let quality = field(e, PP_QUALITY);
    let back_buffers: Vec<u32> = (0..field(e, PP_COUNT))
        .map(|_| {
            let desc = [
                format,
                D3DRTYPE_SURFACE,
                D3DUSAGE_RENDERTARGET,
                0,
                0,
                quality,
                width,
                height,
            ];
            new_surface(e, object, true, desc)
        })
        .collect();
    let depth_stencil = if field(e, PP_AUTO_DEPTH) != 0 {
        let depth_format = field(e, PP_DEPTH_FORMAT);
        let desc = [
            depth_format,
            D3DRTYPE_SURFACE,
            D3DUSAGE_DEPTHSTENCIL,
            0,
            0,
            quality,
            width,
            height,
        ];
        new_surface(e, object, true, desc)
    } else {
        0
    };
    let mut render_targets = [0; RENDER_TARGETS];
    render_targets[0] = back_buffers[0];
    // The device holds its render target and depth surface (counted, as `SetRenderTarget` and
    // `SetDepthStencilSurface` do).
    surface_add_ref(e, Ptr::new(back_buffers[0]));
    if depth_stencil != 0 {
        surface_add_ref(e, Ptr::new(depth_stencil));
    }
    e.vcall(this.addr(), 4, &[]);
    let device = Device {
        references: 1,
        direct3d: this.addr(),
        creation: [adapter, device_type, focus, behavior],
        present: e.mem.bytes(p, PRESENT_SIZE),
        back_buffers,
        render_targets,
        depth_stencil,
        viewport: [0, 0, width, height, 0f32.to_bits(), 1f32.to_bits()],
        gamma_ramp: None,
        render_states: Default::default(),
        stage_states: Default::default(),
        sampler_states: Default::default(),
        textures: Default::default(),
        transforms: Default::default(),
        scissor: [0, 0, width, height],
        clip_planes: Default::default(),
        in_scene: false,
        material: None,
        lights: Default::default(),
        enabled_lights: std::collections::BTreeSet::new(),
        software_vertex_processing: false,
        lost: false,
    };
    e.kernel
        .graphics
        .device_objects
        .devices
        .insert(object, device);
    crate::units::platform::direct3d_device_created(e, behavior);
    #[cfg(windows)]
    {
        let d = &e.kernel.graphics.device_objects.devices[&object];
        let (present, back_buffers, depth) =
            (d.present.clone(), d.back_buffers.clone(), d.depth_stencil);
        super::d3d9real::device_created(e, object, focus, behavior, &present, &back_buffers, depth);
    }
    e.mem.set_u32(out.addr(), object);
    D3D_OK
}

fn device(e: &mut Engine, this: Ptr) -> &mut Device {
    e.kernel
        .graphics
        .device_objects
        .devices
        .get_mut(&this.addr())
        .expect("an IDirect3DDevice9 of this layer")
}

/// Whether the device is between `BeginScene` and `EndScene`.
pub(super) fn in_scene(e: &mut Engine, this: Ptr) -> bool {
    device(e, this).in_scene
}

pub(super) fn iid_is(e: &Engine, iid: Ptr, ids: &[[u8; 16]]) -> bool {
    if iid.is_null() || ids.is_empty() {
        return false;
    }
    let mut asked = [0u8; 16];
    e.mem.read(iid.addr(), &mut asked);
    ids.contains(&asked)
}

/// `QueryInterface(iid, out)`: `IUnknown` and `IDirect3DDevice9` answer the device (counted);
/// others `E_NOINTERFACE` with a null out.
fn device_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(e, iid, &[IID_IUNKNOWN, IID_DEVICE9]) {
        device_add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

pub(super) fn device_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references += 1;
    d.references
}

/// `Release()`: the last reference destroys the device: its implicit surfaces go, the surfaces
/// it holds in its render target and depth slots are released, and its `IDirect3D9` reference.
pub(super) fn device_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references -= 1;
    let left = d.references;
    if left == 0 {
        let d = e
            .kernel
            .graphics
            .device_objects
            .devices
            .remove(&this.addr())
            .expect("the device");
        for held in d.render_targets.iter().chain([&d.depth_stencil]) {
            if *held != 0 {
                surface_release(e, Ptr::new(*held));
            }
        }
        for texture in d.textures.values() {
            texture_unbind(e, Ptr::new(*texture));
        }
        super::d3d9resource::release_bindings(e, this.addr());
        let implicit: Vec<u32> = e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .iter()
            .filter(|(_, s)| s.device == this.addr() && s.implicit)
            .map(|(a, _)| *a)
            .collect();
        for surface in implicit {
            e.kernel.graphics.device_objects.surfaces.remove(&surface);
            #[cfg(windows)]
            super::d3d9real::forget(e, surface);
            e.mem.free(surface);
        }
        #[cfg(windows)]
        super::d3d9real::forget(e, this.addr());
        e.vcall(d.direct3d, 4 * 2, &[]);
        e.mem.free(this.addr());
    }
    left
}

/// `GetDirect3D(out)`: the `IDirect3D9` that made the device, counted.
fn get_direct3d(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let direct3d = device(e, this).direct3d;
    e.vcall(direct3d, 4, &[]);
    e.mem.set_u32(out.addr(), direct3d);
    D3D_OK
}

/// `GetDeviceCaps(out)`: the adapter's caps for the device's type
/// (`IDirect3D9::GetDeviceCaps`, from the table); with hardware or mixed vertex processing the
/// device reports the same caps.
fn get_device_caps(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    e.vcall(direct3d, 4 * 14, &[adapter, device_type, out.addr()])
        .u32()
}

/// `GetDisplayMode(swap chain, out)`: for the implicit swap chain (0) of a windowed device the
/// adapter's display mode; another swap chain `D3DERR_INVALIDCALL`.
fn get_display_mode(e: &mut Engine, this: Ptr, swap_chain: u32, out: Ptr) -> u32 {
    if swap_chain != 0 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, adapter) = (d.direct3d, d.creation[0]);
    e.vcall(direct3d, 4 * 8, &[adapter, out.addr()]).u32()
}

/// `GetCreationParameters(out)`: the adapter, device type, focus window and behaviour flags
/// `CreateDevice` was given.
fn get_creation_parameters(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let creation = device(e, this).creation;
    for (i, word) in creation.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// `GetBackBuffer(swap chain, index, type, out)`: back buffer `index` of the implicit swap
/// chain (0), counted; another swap chain, an index past the count, a type other than
/// `D3DBACKBUFFER_TYPE_MONO` (0) or a null out answer `D3DERR_INVALIDCALL`.
fn get_back_buffer(
    e: &mut Engine,
    this: Ptr,
    swap_chain: u32,
    index: u32,
    kind: u32,
    out: Ptr,
) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let buffers = device(e, this).back_buffers.clone();
    if swap_chain != 0 || kind != 0 || index as usize >= buffers.len() {
        return D3DERR_INVALIDCALL;
    }
    let surface = buffers[index as usize];
    surface_add_ref(e, Ptr::new(surface));
    e.mem.set_u32(out.addr(), surface);
    D3D_OK
}

fn simultaneous_render_targets(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(0x130, |e, caps| {
        let hr = get_device_caps(e, this, caps);
        assert_eq!(hr, D3D_OK, "the device's caps");
        e.mem.u32(caps.addr() + CAPS_SIMULTANEOUS_RTS as u32)
    })
}

/// `SetRenderTarget(index, surface)`: an index at or past the caps' `NumSimultaneousRTs`, a
/// null surface for index 0, or a surface without `D3DUSAGE_RENDERTARGET` answer
/// `D3DERR_INVALIDCALL`. The device holds the new surface (counted) and releases the old one;
/// setting index 0 resets the viewport to the whole surface (depth 0 to 1).
fn set_render_target(e: &mut Engine, this: Ptr, index: u32, surface: u32) -> u32 {
    if index >= simultaneous_render_targets(e, this) || index as usize >= RENDER_TARGETS {
        return D3DERR_INVALIDCALL;
    }
    if surface == 0 && index == 0 {
        return D3DERR_INVALIDCALL;
    }
    let desc = if surface != 0 {
        match e.kernel.graphics.device_objects.surfaces.get(&surface) {
            Some(s) if s.desc[2] & D3DUSAGE_RENDERTARGET != 0 => Some(s.desc),
            _ => return D3DERR_INVALIDCALL,
        }
    } else {
        None
    };
    if surface != 0 {
        surface_add_ref(e, Ptr::new(surface));
    }
    let old = std::mem::replace(&mut device(e, this).render_targets[index as usize], surface);
    if old != 0 {
        surface_release(e, Ptr::new(old));
    }
    if let (0, Some(desc)) = (index, desc) {
        device(e, this).viewport = [0, 0, desc[6], desc[7], 0f32.to_bits(), 1f32.to_bits()];
        device(e, this).scissor = [0, 0, desc[6], desc[7]];
    }
    D3D_OK
}

/// `GetRenderTarget(index, out)`: the surface in that slot, counted; an empty slot answers
/// `D3DERR_NOTFOUND` (out null); an index at or past `NumSimultaneousRTs` or a null out
/// `D3DERR_INVALIDCALL`.
fn get_render_target(e: &mut Engine, this: Ptr, index: u32, out: Ptr) -> u32 {
    if out.is_null()
        || index >= simultaneous_render_targets(e, this)
        || index as usize >= RENDER_TARGETS
    {
        return D3DERR_INVALIDCALL;
    }
    let surface = device(e, this).render_targets[index as usize];
    e.mem.set_u32(out.addr(), surface);
    if surface == 0 {
        return D3DERR_NOTFOUND;
    }
    surface_add_ref(e, Ptr::new(surface));
    D3D_OK
}

/// `SetDepthStencilSurface(surface)`: null removes the depth buffer; a surface without
/// `D3DUSAGE_DEPTHSTENCIL` answers `D3DERR_INVALIDCALL`. The device holds the new surface
/// (counted) and releases the old one.
fn set_depth_stencil_surface(e: &mut Engine, this: Ptr, surface: u32) -> u32 {
    if surface != 0 {
        match e.kernel.graphics.device_objects.surfaces.get(&surface) {
            Some(s) if s.desc[2] & D3DUSAGE_DEPTHSTENCIL != 0 => {}
            _ => return D3DERR_INVALIDCALL,
        }
        surface_add_ref(e, Ptr::new(surface));
    }
    let old = std::mem::replace(&mut device(e, this).depth_stencil, surface);
    if old != 0 {
        surface_release(e, Ptr::new(old));
    }
    D3D_OK
}

/// `GetDepthStencilSurface(out)`: the depth surface, counted; none answers `D3DERR_NOTFOUND`
/// (out null).
fn get_depth_stencil_surface(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let surface = device(e, this).depth_stencil;
    e.mem.set_u32(out.addr(), surface);
    if surface == 0 {
        return D3DERR_NOTFOUND;
    }
    surface_add_ref(e, Ptr::new(surface));
    D3D_OK
}

/// `SetViewport(viewport)`: keeps the six words of the `D3DVIEWPORT9`; a null pointer, or a
/// rectangle outside render target 0, answers `D3DERR_INVALIDCALL`.
fn set_viewport(e: &mut Engine, this: Ptr, viewport: Ptr) -> u32 {
    if viewport.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let words: [u32; 6] = std::array::from_fn(|i| e.mem.u32(viewport.addr() + 4 * i as u32));
    let target = device(e, this).render_targets[0];
    let desc = e.kernel.graphics.device_objects.surfaces[&target].desc;
    let right = u64::from(words[0]) + u64::from(words[2]);
    let bottom = u64::from(words[1]) + u64::from(words[3]);
    if right > u64::from(desc[6]) || bottom > u64::from(desc[7]) {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    d.viewport.copy_from_slice(&words);
    D3D_OK
}

/// `GetViewport(out)`: the viewport's six words.
fn get_viewport(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let viewport = device(e, this).viewport;
    for (i, word) in viewport.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

fn surface(e: &mut Engine, this: Ptr) -> &mut Surface {
    e.kernel
        .graphics
        .device_objects
        .surfaces
        .get_mut(&this.addr())
        .expect("an IDirect3DSurface9 of this layer")
}

/// `QueryInterface(iid, out)`: `IUnknown`, `IDirect3DResource9` and `IDirect3DSurface9` answer
/// the surface (counted); others `E_NOINTERFACE` with a null out.
fn surface_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(e, iid, &[IID_IUNKNOWN, IID_RESOURCE9, IID_SURFACE9]) {
        surface_add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

/// `AddRef()`; a texture's level counts its texture (Direct3D 9 sub-objects share their
/// container's count).
fn surface_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let s = surface(e, this);
    if s.container != 0 {
        let container = s.container;
        return texture_add_ref(e, Ptr::new(container));
    }
    s.references += 1;
    s.references
}

/// `Release()`: the count falls; a surface the program made goes at 0, one the device owns
/// (back buffers, the automatic depth surface) stays until the device goes, and a texture's
/// level releases its texture.
fn surface_release(e: &mut Engine, this: Ptr) -> u32 {
    let s = surface(e, this);
    if s.container != 0 {
        let container = s.container;
        return texture_release(e, Ptr::new(container));
    }
    let s = surface(e, this);
    s.references = s.references.saturating_sub(1);
    let (left, implicit) = (s.references, s.implicit);
    let surface_owner = s.device;
    if left == 0 && !implicit {
        let removed = e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .remove(&this.addr());
        if let Some(s) = removed {
            if s.bits != 0 {
                e.mem.free(s.bits);
            }
        }
        #[cfg(windows)]
        super::d3d9real::forget(e, this.addr());
        e.mem.free(this.addr());
        // A surface the program made (CreateRenderTarget, CreateDepthStencilSurface) held its
        // device.
        let owner = surface_owner;
        device_release(e, Ptr::new(owner));
    }
    left
}

/// `GetDevice(out)`: the surface's device, counted.
fn surface_get_device(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let owner = surface(e, this).device;
    device_add_ref(e, Ptr::new(owner));
    e.mem.set_u32(out.addr(), owner);
    D3D_OK
}

/// `GetType()`: `D3DRTYPE_SURFACE`.
fn surface_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    D3DRTYPE_SURFACE
}

/// `GetDesc(out)`: the 0x20-byte `D3DSURFACE_DESC`.
fn surface_get_desc(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let desc = surface(e, this).desc;
    for (i, word) in desc.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// The 16 words of the identity matrix (`float` 1.0 on the diagonal).
fn identity() -> [u32; 16] {
    let mut m = [0u32; 16];
    for i in 0..4 {
        m[5 * i] = 1f32.to_bits();
    }
    m
}

/// Whether `state` names a transform: `D3DTS_VIEW` (2), `D3DTS_PROJECTION` (3),
/// `D3DTS_TEXTURE0..7` (16 to 23) or `D3DTS_WORLDMATRIX(0..255)` (256 to 511).
fn is_transform(state: u32) -> bool {
    matches!(state, 2 | 3 | 16..=23 | 256..=511)
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetTransform (vtable +0xB0)
/// `SetTransform(state, matrix)`: keeps the 16 words of the `D3DMATRIX`; a null matrix or a
/// state that names no transform answers `D3DERR_INVALIDCALL`.
fn set_transform(e: &mut Engine, this: Ptr, state: u32, matrix: Ptr) -> u32 {
    if matrix.is_null() || !is_transform(state) {
        return D3DERR_INVALIDCALL;
    }
    let mut m = [0u32; 16];
    for (i, word) in m.iter_mut().enumerate() {
        *word = e.mem.u32(matrix.addr() + 4 * i as u32);
    }
    device(e, this).transforms.insert(state, m);
    D3D_OK
}

/// `GetTransform(state, out)`: the matrix set, or the identity (every transform starts as
/// the identity).
fn get_transform(e: &mut Engine, this: Ptr, state: u32, out: Ptr) -> u32 {
    if out.is_null() || !is_transform(state) {
        return D3DERR_INVALIDCALL;
    }
    let m = device(e, this)
        .transforms
        .get(&state)
        .copied()
        .unwrap_or_else(identity);
    for (i, word) in m.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// `SetScissorRect(rect)`: keeps the `RECT`; null answers `D3DERR_INVALIDCALL`.
fn set_scissor_rect(e: &mut Engine, this: Ptr, rect: Ptr) -> u32 {
    if rect.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let mut r = [0u32; 4];
    for (i, word) in r.iter_mut().enumerate() {
        *word = e.mem.u32(rect.addr() + 4 * i as u32);
    }
    device(e, this).scissor = r;
    D3D_OK
}

/// `GetScissorRect(out)`: the rectangle set (at first, and after `SetRenderTarget` on index 0,
/// the whole render target).
fn get_scissor_rect(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let r = device(e, this).scissor;
    for (i, word) in r.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// `SetClipPlane(index, plane)`: keeps the plane's four words for indices 0 to 5
/// (`D3DMAXUSERCLIPPLANES` 32 is the API's limit; the caps' `MaxUserClipPlanes` the device's);
/// a null plane or an index at or past the caps' count answers `D3DERR_INVALIDCALL`.
fn set_clip_plane(e: &mut Engine, this: Ptr, index: u32, plane: Ptr) -> u32 {
    if plane.is_null() || index >= max_user_clip_planes(e, this) {
        return D3DERR_INVALIDCALL;
    }
    let mut p = [0u32; 4];
    for (i, word) in p.iter_mut().enumerate() {
        *word = e.mem.u32(plane.addr() + 4 * i as u32);
    }
    device(e, this).clip_planes.insert(index, p);
    D3D_OK
}

/// `D3DCAPS9::MaxUserClipPlanes` (+0xA4).
fn max_user_clip_planes(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(0x130, |e, caps| {
        assert_eq!(get_device_caps(e, this, caps), D3D_OK, "the device's caps");
        e.mem.u32(caps.addr() + 0xa4)
    })
}

/// `BeginScene()`: starts a scene; inside one already, `D3DERR_INVALIDCALL`.
fn begin_scene(e: &mut Engine, this: Ptr) -> u32 {
    let model = {
        let d = device(e, this);
        if d.in_scene || d.lost {
            D3DERR_INVALIDCALL
        } else {
            d.in_scene = true;
            D3D_OK
        }
    };
    #[cfg(windows)]
    super::d3d9real::begin_scene(e, this.addr(), model);
    model
}

/// `EndScene()`: ends the scene; outside one, `D3DERR_INVALIDCALL`.
fn end_scene(e: &mut Engine, this: Ptr) -> u32 {
    let model = {
        let d = device(e, this);
        if !d.in_scene || d.lost {
            D3DERR_INVALIDCALL
        } else {
            d.in_scene = false;
            D3D_OK
        }
    };
    #[cfg(windows)]
    super::d3d9real::end_scene(e, this.addr(), model);
    model
}

/// `D3DCLEAR_TARGET`, `D3DCLEAR_ZBUFFER`, `D3DCLEAR_STENCIL`.
const D3DCLEAR_TARGET: u32 = 1;
const D3DCLEAR_ZBUFFER: u32 = 2;
const D3DCLEAR_STENCIL: u32 = 4;

// Platform: replaces D3D9.DLL IDirect3DDevice9::Clear (vtable +0xAC)
/// `Clear(count, rects, flags, colour, z, stencil)`: nothing is drawn yet (the viewer draws), so
/// only the call's validity is answered: rectangles without a count or a count without
/// rectangles, flags other than the three clear bits or none of them, and a depth or stencil
/// clear without a depth surface answer `D3DERR_INVALIDCALL`.
#[allow(clippy::too_many_arguments)]
fn clear(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    rects: Ptr,
    flags: u32,
    colour: u32,
    z: u32,
    stencil: u32,
) -> u32 {
    let model = clear_answer(e, this, count, rects, flags);
    #[cfg(windows)]
    super::d3d9real::clear(
        e,
        this.addr(),
        count,
        rects.addr(),
        flags,
        colour,
        z,
        stencil,
        model,
    );
    let _ = (colour, z, stencil);
    model
}

fn clear_answer(e: &mut Engine, this: Ptr, count: u32, rects: Ptr, flags: u32) -> u32 {
    if (count == 0) != rects.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let all = D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER | D3DCLEAR_STENCIL;
    if flags == 0 || flags & !all != 0 {
        return D3DERR_INVALIDCALL;
    }
    if flags & (D3DCLEAR_ZBUFFER | D3DCLEAR_STENCIL) != 0 && device(e, this).depth_stencil == 0 {
        return D3DERR_INVALIDCALL;
    }
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::Present (vtable +0x44)
/// `Present(source, dest, window, dirty region)`: shows the back buffer; nothing is shown yet
/// (the viewer draws), so it answers `D3D_OK` outside a scene and `D3DERR_INVALIDCALL` inside
/// one.
fn present(e: &mut Engine, this: Ptr, source: u32, dest: u32, window: u32, dirty: u32) -> u32 {
    if crate::queue_check::enabled() {
        // Diagnostic only (`NV_QUEUE_CHECK`, not exe behaviour): the frame ends here.
        crate::queue_check::frame(e);
    }
    let model = if device(e, this).in_scene || device(e, this).lost {
        D3DERR_INVALIDCALL
    } else {
        D3D_OK
    };
    #[cfg(windows)]
    super::d3d9real::present(e, this.addr(), source, dest, window, dirty, model);
    let _ = (source, dest, window, dirty);
    model
}

/// `TestCooperativeLevel()`: a windowed device is not lost here: `D3D_OK`, or
/// `D3DERR_DEVICENOTRESET` after a `Reset` that failed until one succeeds (measured, 2026-10-27).
const D3DERR_DEVICENOTRESET: u32 = 0x8876_0869;
fn test_cooperative_level(e: &mut Engine, this: Ptr) -> u32 {
    if device(e, this).lost {
        return D3DERR_DEVICENOTRESET;
    }
    D3D_OK
}

/// Whether `sampler` names a sampler: 0 to 15, `D3DDMAPSAMPLER` (256) or the vertex samplers
/// `D3DVERTEXTEXTURESAMPLER0..3` (257 to 260).
fn is_sampler(sampler: u32) -> bool {
    sampler < 16 || (256..=260).contains(&sampler)
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetGammaRamp (vtable +0x54)
/// `SetGammaRamp(swap_chain, flags, ramp)`: the ramp (`D3DGAMMARAMP`, three tables of 256
/// WORDs) is kept for the swap chain 0, the only one the game makes. The method is `void` in
/// the SDK, so the word in `EAX` is not part of its contract; the game ignores it (004dcef0, the
/// Display menu's brightness). Swap chains other than 0 and a null ramp are not provided.
fn set_gamma_ramp(e: &mut Engine, this: Ptr, swap_chain: u32, flags: u32, ramp: Ptr) -> u32 {
    let _ = flags;
    assert!(
        swap_chain == 0 && !ramp.is_null(),
        "IDirect3DDevice9::SetGammaRamp({swap_chain}, .., {:08x}) is not provided",
        ramp.addr()
    );
    let table = (0..0x300).map(|i| e.mem.u16(ramp.addr() + 2 * i)).collect();
    device(e, this).gamma_ramp = Some(table);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetRenderState (vtable +0xE4)
/// `SetRenderState(state, value)`: kept; answers `D3D_OK`. A state number above 255
/// (`D3DRENDERSTATETYPE` ends at 209) is not provided.
fn set_render_state(e: &mut Engine, this: Ptr, state: u32, value: u32) -> u32 {
    assert!(
        state < 256,
        "IDirect3DDevice9::SetRenderState({state}) is not provided"
    );
    device(e, this).render_states.set(state as usize, value);
    D3D_OK
}

/// `GetRenderState(state, out)`: the value set. A state the program never set (its default)
/// is not provided yet.
fn get_render_state(e: &mut Engine, this: Ptr, state: u32, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let Some(value) = device(e, this).render_states.get(Some(state as usize)) else {
        panic!("IDirect3DDevice9::GetRenderState({state}) before a SetRenderState: its default is not provided");
    };
    e.mem.set_u32(out.addr(), value);
    D3D_OK
}

/// `SetTextureStageState(stage, type, value)`: kept for stages 0 to 7
/// (`D3DTEXTURESTAGESTATETYPE` 1 to 32); another stage answers `D3DERR_INVALIDCALL`.
fn set_texture_stage_state(e: &mut Engine, this: Ptr, stage: u32, kind: u32, value: u32) -> u32 {
    if stage >= 8 {
        return D3DERR_INVALIDCALL;
    }
    assert!(
        (1..=32).contains(&kind),
        "IDirect3DDevice9::SetTextureStageState(type {kind}) is not provided"
    );
    let index = stage_index(stage, kind).expect("a type 1 to 32");
    device(e, this).stage_states.set(index, value);
    D3D_OK
}

/// `GetTextureStageState(stage, type, out)`: the value set; one never set is not provided yet.
fn get_texture_stage_state(e: &mut Engine, this: Ptr, stage: u32, kind: u32, out: Ptr) -> u32 {
    if stage >= 8 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let Some(value) = device(e, this).stage_states.get(stage_index(stage, kind)) else {
        panic!("IDirect3DDevice9::GetTextureStageState({stage}, {kind}) before a set: its default is not provided");
    };
    e.mem.set_u32(out.addr(), value);
    D3D_OK
}

/// `SetSamplerState(sampler, type, value)`: kept for the samplers [`is_sampler`] names
/// (`D3DSAMPLERSTATETYPE` 1 to 13); another sampler answers `D3DERR_INVALIDCALL`.
fn set_sampler_state(e: &mut Engine, this: Ptr, sampler: u32, kind: u32, value: u32) -> u32 {
    if !is_sampler(sampler) {
        return D3DERR_INVALIDCALL;
    }
    assert!(
        (1..=13).contains(&kind),
        "IDirect3DDevice9::SetSamplerState(type {kind}) is not provided"
    );
    let index = sampler_index(sampler, kind).expect("a type 1 to 13");
    device(e, this).sampler_states.set(index, value);
    D3D_OK
}

/// `GetSamplerState(sampler, type, out)`: the value set; one never set is not provided yet.
fn get_sampler_state(e: &mut Engine, this: Ptr, sampler: u32, kind: u32, out: Ptr) -> u32 {
    if !is_sampler(sampler) || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let Some(value) = device(e, this)
        .sampler_states
        .get(sampler_index(sampler, kind))
    else {
        panic!("IDirect3DDevice9::GetSamplerState({sampler}, {kind}) before a set: its default is not provided");
    };
    e.mem.set_u32(out.addr(), value);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetTexture (vtable +0x104)
/// `SetTexture(sampler, texture)`: the device holds the texture (counted) for the sampler and
/// releases the one it held; null clears it. Another sampler answers `D3DERR_INVALIDCALL`;
/// a texture not of this layer's kinds (cube and volume textures are not provided yet) stops.
fn set_texture(e: &mut Engine, this: Ptr, sampler: u32, texture: u32) -> u32 {
    if !is_sampler(sampler) {
        return D3DERR_INVALIDCALL;
    }
    // The texture and the device are looked up once each (they are apart in `Objects`).
    let objects = &mut e.kernel.graphics.device_objects;
    let held = if texture != 0 {
        let Some(t) = objects.textures.get_mut(&texture) else {
            panic!(
                "IDirect3DDevice9::SetTexture({sampler}, {texture:#x}): not a texture of this layer"
            );
        };
        Some(t)
    } else {
        None
    };
    let d = objects
        .devices
        .get_mut(&this.addr())
        .expect("an IDirect3DDevice9 of this layer");
    let old = if let Some(t) = held {
        // The texture the sampler holds already: counting it once more and letting go of it
        // once changes nothing.
        if d.textures.get(&sampler) == Some(&texture) {
            return D3D_OK;
        }
        t.bound += 1;
        d.textures.insert(sampler, texture)
    } else {
        d.textures.remove(&sampler)
    };
    if let Some(old) = old {
        texture_unbind(e, Ptr::new(old));
    }
    D3D_OK
}

/// `GetTexture(sampler, out)`: the texture held for the sampler, counted, or null.
fn get_texture(e: &mut Engine, this: Ptr, sampler: u32, out: Ptr) -> u32 {
    if !is_sampler(sampler) || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let texture = device(e, this).textures.get(&sampler).copied().unwrap_or(0);
    if texture != 0 {
        texture_add_ref(e, Ptr::new(texture));
    }
    e.mem.set_u32(out.addr(), texture);
    D3D_OK
}

/// `GetContainer(iid, out)`: a texture's level answers its texture (counted) for `IUnknown`,
/// `IDirect3DResource9`, `IDirect3DBaseTexture9` or `IDirect3DTexture9`, `E_NOINTERFACE` (out
/// null) for other interfaces; the container of other surfaces is not provided.
fn surface_get_container(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let container = surface(e, this).container;
    if container != 0
        && iid_is(
            e,
            iid,
            &[IID_IUNKNOWN, IID_RESOURCE9, IID_BASETEXTURE9, IID_TEXTURE9],
        )
    {
        texture_add_ref(e, Ptr::new(container));
        e.mem.set_u32(out.addr(), container);
        return D3D_OK;
    }
    if container != 0 {
        e.mem.set_u32(out.addr(), 0);
        return E_NOINTERFACE;
    }
    panic!("IDirect3DSurface9::GetContainer of a surface without a texture is not provided")
}

/// The block width (1, or 4 for the DXT formats) and the bytes of one block (one pixel for the
/// others) of a format; `None` for formats this layer doesn't size yet.
fn format_blocks(format: u32) -> Option<(u32, u32)> {
    const DXT1: u32 = u32::from_le_bytes(*b"DXT1");
    const DXT2: u32 = u32::from_le_bytes(*b"DXT2");
    const DXT3: u32 = u32::from_le_bytes(*b"DXT3");
    const DXT4: u32 = u32::from_le_bytes(*b"DXT4");
    const DXT5: u32 = u32::from_le_bytes(*b"DXT5");
    Some(match format {
        DXT1 => (4, 8),
        DXT2 | DXT3 | DXT4 | DXT5 => (4, 16),
        // R8G8B8.
        20 => (1, 3),
        // A8R8G8B8, X8R8G8B8, A2B10G10R10, A8B8G8R8, X8B8G8R8, G16R16, A2R10G10B10,
        // X8L8V8U8, Q8W8V8U8, V16U16, A2W10V10U10, G16R16F, R32F, D32, D24S8, D24X8, D24X4S4.
        21 | 22 | 31 | 32 | 33 | 34 | 35 | 62 | 63 | 64 | 67 | 112 | 114 | 71 | 75 | 77 | 79 => {
            (1, 4)
        }
        // R5G6B5, X1R5G5B5, A1R5G5B5, A4R4G4B4, A8R3G3B2, X4R4G4B4, A8P8, A8L8, V8U8, L6V5U5,
        // L16, R16F, D16, D15S1.
        23 | 24 | 25 | 26 | 29 | 30 | 40 | 51 | 60 | 61 | 81 | 111 | 80 | 73 => (1, 2),
        // R3G3B2, A8, P8, L8, A4L4.
        27 | 28 | 41 | 50 | 52 => (1, 1),
        // A16B16G16R16, A16B16G16R16F, G32R32F.
        36 | 113 | 115 => (1, 8),
        // A32B32G32R32F.
        116 => (1, 16),
        _ => return None,
    })
}

/// A level's pitch (bytes per row of blocks) and size, for a format [`format_blocks`] sizes.
fn level_layout(format: u32, width: u32, height: u32) -> (u32, u32) {
    let Some((block, bytes)) = format_blocks(format) else {
        panic!("IDirect3DDevice9: the memory layout of format {format:#x} is not provided");
    };
    let pitch = width.div_ceil(block) * bytes;
    (pitch, pitch * height.div_ceil(block))
}

/// A texture of this layer.
#[derive(Debug, Clone)]
struct Texture {
    /// The program's references (what `AddRef` and `Release` answer).
    references: u32,
    /// The device's samplers holding it: apart from the program's count, as the retail
    /// runtime keeps them (`d3d9-capture device refs`, 2026-10-27).
    bound: u32,
    device: u32,
    pool: u32,
    /// 1, or 6 for a cube texture.
    faces: u32,
    /// The surfaces, face by face (each face's chain largest first).
    levels: Vec<u32>,
    lod: u32,
    priority: u32,
}

fn texture(e: &mut Engine, this: Ptr) -> &mut Texture {
    e.kernel
        .graphics
        .device_objects
        .textures
        .get_mut(&this.addr())
        .expect("an IDirect3DTexture9 of this layer")
}

/// `D3DPOOL_DEFAULT`, `D3DPOOL_MANAGED`; the usages `D3DUSAGE_DYNAMIC`, `D3DUSAGE_AUTOGENMIPMAP`;
/// `D3DRTYPE_TEXTURE`; the texture caps `D3DPTEXTURECAPS_POW2` (`D3DCAPS9::TextureCaps` +0x3C).
const D3DPOOL_DEFAULT: u32 = 0;
const D3DPOOL_MANAGED: u32 = 1;
const D3DUSAGE_DYNAMIC: u32 = 0x200;
const D3DUSAGE_AUTOGENMIPMAP: u32 = 0x400;
const D3DRTYPE_TEXTURE: u32 = 3;
const CAPS_TEXTURE_CAPS: u32 = 0x3c;
const D3DPTEXTURECAPS_POW2: u32 = 0x2;

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateTexture (vtable +0x5C)
/// `CreateTexture(width, height, levels, usage, format, pool, out, shared handle)`. A zero
/// size, a null out, a shared handle (Direct3D 9Ex only), more levels than the full chain, a
/// render-target or depth-stencil usage outside `D3DPOOL_DEFAULT`, `D3DUSAGE_DYNAMIC` in
/// `D3DPOOL_MANAGED`, or a format the table's `CheckDeviceFormat` (the device's adapter and
/// type, the display format, the usage, `D3DRTYPE_TEXTURE`) refuses answer
/// `D3DERR_INVALIDCALL`. Levels 0 is the full chain down to 1 by 1 (sides halved, rounding
/// down, at least 1). Each level is a surface of the texture (`GetSurfaceLevel`, counted with
/// the texture); levels the program can lock (any pool but `D3DPOOL_DEFAULT`, or dynamic) get
/// memory in the game's address space, rows packed tightly (the pitch is the row's bytes: a
/// value Direct3D leaves to the runtime; the game copies by it). Not provided (the run stops):
/// `D3DUSAGE_AUTOGENMIPMAP`, a side that is not a power of two on a device whose caps require
/// them, and formats [`format_blocks`] doesn't size.
#[allow(clippy::too_many_arguments)]
fn create_texture(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    levels: u32,
    usage: u32,
    format: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if width == 0 || height == 0 || out.is_null() || shared != 0 {
        return D3DERR_INVALIDCALL;
    }
    if usage & D3DUSAGE_AUTOGENMIPMAP != 0 {
        panic!("IDirect3DDevice9::CreateTexture: D3DUSAGE_AUTOGENMIPMAP is not provided");
    }
    let full = 32 - width.max(height).leading_zeros();
    if levels > full {
        return D3DERR_INVALIDCALL;
    }
    if usage & (D3DUSAGE_RENDERTARGET | D3DUSAGE_DEPTHSTENCIL) != 0 && pool != D3DPOOL_DEFAULT {
        return D3DERR_INVALIDCALL;
    }
    if usage & D3DUSAGE_DYNAMIC != 0 && pool == D3DPOOL_MANAGED {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(direct3d, 4 * 8, &[adapter, mode.addr()]).u32();
        assert_eq!(hr, D3D_OK, "GetAdapterDisplayMode for CreateTexture");
        e.mem.u32(mode.addr() + 0xc)
    });
    let check = e
        .vcall(
            direct3d,
            4 * 10,
            &[
                adapter,
                device_type,
                display_format,
                usage,
                D3DRTYPE_TEXTURE,
                format,
            ],
        )
        .u32();
    if check != D3D_OK {
        return D3DERR_INVALIDCALL;
    }
    if !(width.is_power_of_two() && height.is_power_of_two()) {
        let caps = e.with_stack(0x130, |e, caps| {
            assert_eq!(get_device_caps(e, this, caps), D3D_OK, "the device's caps");
            e.mem.u32(caps.addr() + CAPS_TEXTURE_CAPS)
        });
        if caps & D3DPTEXTURECAPS_POW2 != 0 {
            panic!(
                "IDirect3DDevice9::CreateTexture: a {width}x{height} texture on a device that \
                 wants powers of two is not provided"
            );
        }
    }
    let count = if levels == 0 { full } else { levels };
    let lockable = pool != D3DPOOL_DEFAULT || usage & D3DUSAGE_DYNAMIC != 0;
    let object = e.mem.alloc(8);
    let table = e.kernel.graphics.device_objects.texture_vtable;
    e.mem.set_u32(object, table);
    let device_addr = this.addr();
    let surfaces: Vec<u32> = (0..count)
        .map(|level| {
            let (w, h) = ((width >> level).max(1), (height >> level).max(1));
            let desc = [format, D3DRTYPE_SURFACE, usage, pool, 0, 0, w, h];
            let surface = new_surface(e, device_addr, false, desc);
            let (pitch, size) = if lockable {
                level_layout(format, w, h)
            } else {
                (0, 0)
            };
            let bits = if lockable { e.mem.alloc(size) } else { 0 };
            let s = e
                .kernel
                .graphics
                .device_objects
                .surfaces
                .get_mut(&surface)
                .expect("the level");
            s.container = object;
            s.bits = bits;
            s.pitch = pitch;
            surface
        })
        .collect();
    e.kernel.graphics.device_objects.textures.insert(
        object,
        Texture {
            references: 1,
            bound: 0,
            device: device_addr,
            pool,
            faces: 1,
            levels: surfaces,
            lod: 0,
            priority: 0,
        },
    );
    device_add_ref(e, this);
    note_reference(e, Ptr::new(object), "Create", 1);
    e.mem.set_u32(out.addr(), object);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateCubeTexture (vtable +0x64)
/// `CreateCubeTexture(edge, levels, usage, format, pool, out, shared handle)`: as
/// [`create_texture`] for six square faces (`D3DRTYPE_CUBETEXTURE` in the table's
/// `CheckDeviceFormat`), each face with its own chain of surfaces (counted with the cube
/// texture). Not provided (the run stops): `D3DUSAGE_AUTOGENMIPMAP`, an edge that is not a
/// power of two on a device whose caps want powers of two for cube maps, and formats
/// [`format_blocks`] doesn't size.
#[allow(clippy::too_many_arguments)]
fn create_cube_texture(
    e: &mut Engine,
    this: Ptr,
    edge: u32,
    levels: u32,
    usage: u32,
    format: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if edge == 0 || out.is_null() || shared != 0 {
        return D3DERR_INVALIDCALL;
    }
    if usage & D3DUSAGE_AUTOGENMIPMAP != 0 {
        panic!("IDirect3DDevice9::CreateCubeTexture: D3DUSAGE_AUTOGENMIPMAP is not provided");
    }
    let full = 32 - edge.leading_zeros();
    if levels > full {
        return D3DERR_INVALIDCALL;
    }
    if usage & (D3DUSAGE_RENDERTARGET | D3DUSAGE_DEPTHSTENCIL) != 0 && pool != D3DPOOL_DEFAULT {
        return D3DERR_INVALIDCALL;
    }
    if usage & D3DUSAGE_DYNAMIC != 0 && pool == D3DPOOL_MANAGED {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(direct3d, 4 * 8, &[adapter, mode.addr()]).u32();
        assert_eq!(hr, D3D_OK, "GetAdapterDisplayMode for CreateCubeTexture");
        e.mem.u32(mode.addr() + 0xc)
    });
    let check = e
        .vcall(
            direct3d,
            4 * 10,
            &[
                adapter,
                device_type,
                display_format,
                usage,
                D3DRTYPE_CUBETEXTURE,
                format,
            ],
        )
        .u32();
    if check != D3D_OK {
        return D3DERR_INVALIDCALL;
    }
    if !edge.is_power_of_two() {
        let caps = e.with_stack(0x130, |e, caps| {
            assert_eq!(get_device_caps(e, this, caps), D3D_OK, "the device's caps");
            e.mem.u32(caps.addr() + CAPS_TEXTURE_CAPS)
        });
        if caps & D3DPTEXTURECAPS_CUBEMAP_POW2 != 0 {
            panic!("IDirect3DDevice9::CreateCubeTexture: an edge of {edge} on a device that wants powers of two is not provided");
        }
    }
    let count = if levels == 0 { full } else { levels };
    let lockable = pool != D3DPOOL_DEFAULT || usage & D3DUSAGE_DYNAMIC != 0;
    let object = e.mem.alloc(8);
    let table = e.kernel.graphics.device_objects.cube_vtable;
    e.mem.set_u32(object, table);
    let device_addr = this.addr();
    let mut surfaces = Vec::new();
    for _face in 0..6 {
        for level in 0..count {
            let side = (edge >> level).max(1);
            let desc = [format, D3DRTYPE_SURFACE, usage, pool, 0, 0, side, side];
            let surface = new_surface(e, device_addr, false, desc);
            let (pitch, size) = if lockable {
                level_layout(format, side, side)
            } else {
                (0, 0)
            };
            let bits = if lockable { e.mem.alloc(size) } else { 0 };
            let s = e
                .kernel
                .graphics
                .device_objects
                .surfaces
                .get_mut(&surface)
                .expect("the level");
            s.container = object;
            s.bits = bits;
            s.pitch = pitch;
            surfaces.push(surface);
        }
    }
    e.kernel.graphics.device_objects.textures.insert(
        object,
        Texture {
            references: 1,
            bound: 0,
            device: device_addr,
            pool,
            faces: 6,
            levels: surfaces,
            lod: 0,
            priority: 0,
        },
    );
    device_add_ref(e, this);
    note_reference(e, Ptr::new(object), "CreateCube", 1);
    e.mem.set_u32(out.addr(), object);
    D3D_OK
}

/// Whether a surface of `format` may have `multisample` samples at `quality`: `D3DMULTISAMPLE_NONE`
/// (0) always; otherwise the device's `IDirect3D9::CheckDeviceMultiSampleType` (its adapter and
/// type, windowed as the device is) answers `D3D_OK` with more quality levels than `quality`.
fn multisample_supported(
    e: &mut Engine,
    this: Ptr,
    format: u32,
    multisample: u32,
    quality: u32,
) -> bool {
    if multisample == 0 {
        return true;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    let windowed = u32::from_le_bytes(
        d.present[PP_WINDOWED as usize..PP_WINDOWED as usize + 4]
            .try_into()
            .unwrap(),
    );
    e.with_stack(4, |e, levels| {
        let hr = e
            .vcall(
                direct3d,
                4 * 11,
                &[
                    adapter,
                    device_type,
                    format,
                    windowed,
                    multisample,
                    levels.addr(),
                ],
            )
            .u32();
        hr == D3D_OK && quality < e.mem.u32(levels.addr())
    })
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateRenderTarget (vtable +0x70)
/// `CreateRenderTarget(width, height, format, multisample type, quality, lockable, out, shared
/// handle)`. A zero size, a null out, a shared handle (Direct3D 9Ex only), a format the table's
/// `CheckDeviceFormat` (the device's adapter and type, the display format,
/// `D3DUSAGE_RENDERTARGET`, `D3DRTYPE_SURFACE`) refuses, or a multisample type and quality its
/// `CheckDeviceMultiSampleType` refuses answer `D3DERR_INVALIDCALL`. The surface
/// (`D3DPOOL_DEFAULT`, usage `D3DUSAGE_RENDERTARGET`, reference count 1) holds its device until
/// released; a lockable one gets memory in the game's address space (rows packed). Not
/// provided (the run stops): a lockable multisampled target (Direct3D refuses it) and formats
/// [`format_blocks`] doesn't size.
#[allow(clippy::too_many_arguments)]
fn create_render_target(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    format: u32,
    multisample: u32,
    quality: u32,
    lockable: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if width == 0 || height == 0 || out.is_null() || shared != 0 {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(direct3d, 4 * 8, &[adapter, mode.addr()]).u32();
        assert_eq!(hr, D3D_OK, "GetAdapterDisplayMode for CreateRenderTarget");
        e.mem.u32(mode.addr() + 0xc)
    });
    let check = e
        .vcall(
            direct3d,
            4 * 10,
            &[
                adapter,
                device_type,
                display_format,
                D3DUSAGE_RENDERTARGET,
                D3DRTYPE_SURFACE,
                format,
            ],
        )
        .u32();
    if check != D3D_OK || !multisample_supported(e, this, format, multisample, quality) {
        return D3DERR_INVALIDCALL;
    }
    if lockable != 0 && multisample != 0 {
        panic!(
            "IDirect3DDevice9::CreateRenderTarget: a lockable multisampled target is not provided"
        );
    }
    let desc = [
        format,
        D3DRTYPE_SURFACE,
        D3DUSAGE_RENDERTARGET,
        D3DPOOL_DEFAULT,
        multisample,
        quality,
        width,
        height,
    ];
    let surface = new_surface(e, this.addr(), false, desc);
    if lockable != 0 {
        let (pitch, size) = level_layout(format, width, height);
        let bits = e.mem.alloc(size);
        let s = e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .get_mut(&surface)
            .expect("the target");
        s.bits = bits;
        s.pitch = pitch;
    }
    device_add_ref(e, this);
    e.mem.set_u32(out.addr(), surface);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateDepthStencilSurface (vtable +0x74)
/// `CreateDepthStencilSurface(width, height, format, multisample type, quality, discard, out,
/// shared handle)`. A zero size, a null out, a shared handle (Direct3D 9Ex only) or a format
/// the table's `CheckDeviceFormat` (the device's adapter and type, the display format,
/// `D3DUSAGE_DEPTHSTENCIL`, `D3DRTYPE_SURFACE`) refuses answer `D3DERR_INVALIDCALL`. The
/// surface (`D3DPOOL_DEFAULT`, usage `D3DUSAGE_DEPTHSTENCIL`, reference count 1) holds its
/// device until released; it has no memory the program can lock (the depth formats the game
/// uses are not lockable), so `LockRect` answers `D3DERR_INVALIDCALL`. A multisample type and
/// quality the table's `CheckDeviceMultiSampleType` refuses answer `D3DERR_INVALIDCALL`.
#[allow(clippy::too_many_arguments)]
fn create_depth_stencil_surface(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    format: u32,
    multisample: u32,
    quality: u32,
    _discard: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if width == 0 || height == 0 || out.is_null() || shared != 0 {
        return D3DERR_INVALIDCALL;
    }
    if !multisample_supported(e, this, format, multisample, quality) {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(direct3d, 4 * 8, &[adapter, mode.addr()]).u32();
        assert_eq!(
            hr, D3D_OK,
            "GetAdapterDisplayMode for CreateDepthStencilSurface"
        );
        e.mem.u32(mode.addr() + 0xc)
    });
    let check = e
        .vcall(
            direct3d,
            4 * 10,
            &[
                adapter,
                device_type,
                display_format,
                D3DUSAGE_DEPTHSTENCIL,
                D3DRTYPE_SURFACE,
                format,
            ],
        )
        .u32();
    if check != D3D_OK {
        return D3DERR_INVALIDCALL;
    }
    let desc = [
        format,
        D3DRTYPE_SURFACE,
        D3DUSAGE_DEPTHSTENCIL,
        D3DPOOL_DEFAULT,
        multisample,
        quality,
        width,
        height,
    ];
    let surface = new_surface(e, this.addr(), false, desc);
    device_add_ref(e, this);
    e.mem.set_u32(out.addr(), surface);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateOffscreenPlainSurface (vtable +0x90)
/// `CreateOffscreenPlainSurface(width, height, format, pool, out, shared handle)`, as the retail
/// runtime answered (`d3d9-capture device extra`, 2026-10-27): a zero side, a null out, a shared
/// handle, `D3DPOOL_MANAGED` or a format the table's `CheckDeviceFormat` (usage 0,
/// `D3DRTYPE_SURFACE`) refuses answer `D3DERR_INVALIDCALL`; in the other pools (default, system
/// memory, scratch) the surface has memory the program can lock (rows packed tightly), and holds
/// the device.
#[allow(clippy::too_many_arguments)]
fn create_offscreen_plain_surface(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    format: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if width == 0 || height == 0 || out.is_null() || shared != 0 || pool == D3DPOOL_MANAGED {
        return D3DERR_INVALIDCALL;
    }
    let d = device(e, this);
    let (direct3d, [adapter, device_type, ..]) = (d.direct3d, d.creation);
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(direct3d, 4 * 8, &[adapter, mode.addr()]).u32();
        assert_eq!(
            hr, D3D_OK,
            "GetAdapterDisplayMode for CreateOffscreenPlainSurface"
        );
        e.mem.u32(mode.addr() + 0xc)
    });
    let check = e
        .vcall(
            direct3d,
            4 * 10,
            &[
                adapter,
                device_type,
                display_format,
                0,
                D3DRTYPE_SURFACE,
                format,
            ],
        )
        .u32();
    if check != D3D_OK {
        return D3DERR_INVALIDCALL;
    }
    let desc = [format, D3DRTYPE_SURFACE, 0, pool, 0, 0, width, height];
    let surface = new_surface(e, this.addr(), false, desc);
    let (pitch, size) = level_layout(format, width, height);
    let bits = e.mem.alloc(size);
    let s = e
        .kernel
        .graphics
        .device_objects
        .surfaces
        .get_mut(&surface)
        .expect("the surface");
    s.bits = bits;
    s.pitch = pitch;
    device_add_ref(e, this);
    #[cfg(windows)]
    super::d3d9real::offscreen_created(e, this.addr(), surface, width, height, format, pool);
    e.mem.set_u32(out.addr(), surface);
    D3D_OK
}

/// A rectangle (`RECT`) inside a `width` by `height` surface, or the whole surface for null.
fn rect_inside(e: &Engine, rect: Ptr, width: u32, height: u32) -> Option<[u32; 4]> {
    if rect.is_null() {
        return Some([0, 0, width, height]);
    }
    let r: [i32; 4] = std::array::from_fn(|i| e.mem.i32(rect.addr() + 4 * i as u32));
    let inside = r[0] >= 0
        && r[1] >= 0
        && r[0] <= r[2]
        && r[1] <= r[3]
        && r[2] as u32 <= width
        && r[3] as u32 <= height;
    inside.then(|| r.map(|v| v as u32))
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::ColorFill (vtable +0x8C)
/// `ColorFill(surface, rect, colour)`, as the retail runtime answered (2026-10-27): a surface
/// outside `D3DPOOL_DEFAULT` or a rectangle past its edges answers `D3DERR_INVALIDCALL`; in a
/// scene too it answers `D3D_OK`. The surface's memory, when it has some here, takes the colour
/// as the runtime wrote it: `X8R8G8B8` with the unused byte 0, `A8R8G8B8` the colour; other
/// formats with memory are not provided. A render target's pixels are the real device's
/// (`NV_D3D9=real`).
fn color_fill(e: &mut Engine, this: Ptr, surface: u32, rect: Ptr, colour: u32) -> u32 {
    let Some(s) = e
        .kernel
        .graphics
        .device_objects
        .surfaces
        .get(&surface)
        .cloned()
    else {
        return D3DERR_INVALIDCALL;
    };
    if s.desc[3] != D3DPOOL_DEFAULT {
        return D3DERR_INVALIDCALL;
    }
    let Some([left, top, right, bottom]) = rect_inside(e, rect, s.desc[6], s.desc[7]) else {
        return D3DERR_INVALIDCALL;
    };
    if s.bits != 0 {
        let word = match s.desc[0] {
            22 => colour & 0x00ff_ffff,
            21 => colour,
            f => panic!("IDirect3DDevice9::ColorFill of a surface of format {f} is not provided"),
        };
        for y in top..bottom {
            for x in left..right {
                e.mem.set_u32(s.bits + y * s.pitch + 4 * x, word);
            }
        }
    }
    #[cfg(windows)]
    super::d3d9real::color_fill(e, this.addr(), surface, rect.addr(), colour, D3D_OK);
    let _ = this;
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::GetRenderTargetData (vtable +0x80)
/// `GetRenderTargetData(render target, destination)`, as the retail runtime answered
/// (2026-10-27): the source must be a render target in `D3DPOOL_DEFAULT`, the destination a
/// `D3DPOOL_SYSTEMMEM` surface of the same size and format, else `D3DERR_INVALIDCALL`; in a
/// scene too it answers `D3D_OK`. The pixels are the real device's: with `NV_D3D9=real` they are
/// read back into the destination's memory; without it nothing is drawn and the copy is not
/// provided (the run stops).
fn get_render_target_data(e: &mut Engine, this: Ptr, source: u32, destination: u32) -> u32 {
    let surfaces = &e.kernel.graphics.device_objects.surfaces;
    let (Some(from), Some(to)) = (
        surfaces.get(&source).cloned(),
        surfaces.get(&destination).cloned(),
    ) else {
        return D3DERR_INVALIDCALL;
    };
    let ok = from.desc[2] & D3DUSAGE_RENDERTARGET != 0
        && from.desc[3] == D3DPOOL_DEFAULT
        && to.desc[3] == D3DPOOL_SYSTEMMEM
        && (from.desc[0], from.desc[6], from.desc[7]) == (to.desc[0], to.desc[6], to.desc[7]);
    if !ok {
        return D3DERR_INVALIDCALL;
    }
    #[cfg(windows)]
    if super::d3d9real::get_render_target_data(
        e,
        this.addr(),
        source,
        destination,
        to.bits,
        to.pitch,
    ) {
        return D3D_OK;
    }
    let _ = this;
    panic!(
        "IDirect3DDevice9::GetRenderTargetData without NV_D3D9=real (nothing is drawn, so no \
         pixels to read back) is not provided"
    )
}

/// Whether the device has a resource of its own in `D3DPOOL_DEFAULT` alive (a texture, a
/// surface it did not make itself, a buffer).
fn default_pool_alive(e: &Engine, device: u32) -> bool {
    let o = &e.kernel.graphics.device_objects;
    o.textures
        .values()
        .any(|t| t.device == device && t.pool == D3DPOOL_DEFAULT)
        || o.surfaces.values().any(|s| {
            s.device == device && !s.implicit && s.container == 0 && s.desc[3] == D3DPOOL_DEFAULT
        })
        || super::d3d9resource::default_pool_alive(e, device)
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::Reset (vtable +0x40)
/// `Reset(present parameters)`, as the retail runtime answered on a windowed device
/// (`d3d9-capture device reset`, 2026-10-27):
/// - with a resource of the device's own in `D3DPOOL_DEFAULT` alive, or more than three back
///   buffers (the count left as given), it answers `D3DERR_INVALIDCALL` and the device needs a
///   successful `Reset`: `TestCooperativeLevel` answers `D3DERR_DEVICENOTRESET`, `BeginScene`,
///   `EndScene` and `Present` are refused, also after the resource is released;
/// - else, in a scene too (which it ends), it answers `D3D_OK` with a count of 0 made 1 and
///   `D3DFMT_UNKNOWN` the display format (written back); the old back buffers and automatic
///   depth surface go (a reference the program still holds no longer names a live surface) and
///   new ones of the parameters are made, render target 0 the first back buffer, the depth
///   surface the automatic one; every state the program set returns to its default (render,
///   texture stage and sampler states, transforms, clip planes, material and lights, textures,
///   streams, indices, declaration, FVF, shaders, constants), the viewport and scissor the whole
///   back buffer.
///
/// Not provided (the run stops): a full-screen device, a size taken from the window (0),
/// multisampling.
fn reset(e: &mut Engine, this: Ptr, present: Ptr) -> u32 {
    if present.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let p = present.addr();
    let field = |e: &Engine, offset: u32| e.mem.u32(p + offset);
    if field(e, PP_WINDOWED) == 0 {
        panic!("IDirect3DDevice9::Reset to a full-screen device is not provided");
    }
    if default_pool_alive(e, this.addr()) || field(e, PP_COUNT) > BACK_BUFFERS_MAX {
        device(e, this).lost = true;
        return D3DERR_INVALIDCALL;
    }
    if field(e, PP_WIDTH) == 0 || field(e, PP_HEIGHT) == 0 {
        panic!(
            "IDirect3DDevice9::Reset with a back buffer size taken from the window is not provided"
        );
    }
    if field(e, PP_MULTISAMPLE) != 0 {
        panic!("IDirect3DDevice9::Reset with multisampling is not provided");
    }
    let (direct3d, adapter) = {
        let d = device(e, this);
        (d.direct3d, d.creation[0])
    };
    let display_format = e.with_stack(0x10, |e, mode| {
        let hr = e.vcall(direct3d, 4 * 8, &[adapter, mode.addr()]).u32();
        assert_eq!(hr, D3D_OK, "GetAdapterDisplayMode for Reset");
        e.mem.u32(mode.addr() + 0xc)
    });
    if field(e, PP_COUNT) == 0 {
        e.mem.set_u32(p + PP_COUNT, 1);
    }
    if field(e, PP_FORMAT) == D3DFMT_UNKNOWN {
        e.mem.set_u32(p + PP_FORMAT, display_format);
    }
    // The device's bindings and states go back to their defaults.
    let old = {
        let d = device(e, this);
        let held: Vec<u32> = d
            .render_targets
            .iter()
            .chain([&d.depth_stencil])
            .copied()
            .filter(|&s| s != 0)
            .collect();
        let textures: Vec<u32> = d.textures.values().copied().collect();
        d.render_targets = [0; RENDER_TARGETS];
        d.depth_stencil = 0;
        d.textures.clear();
        d.render_states.clear();
        d.stage_states.clear();
        d.sampler_states.clear();
        d.transforms.clear();
        d.clip_planes.clear();
        d.material = None;
        d.lights.clear();
        d.enabled_lights.clear();
        d.in_scene = false;
        (held, textures)
    };
    for surface in old.0 {
        surface_release(e, Ptr::new(surface));
    }
    for texture in old.1 {
        texture_unbind(e, Ptr::new(texture));
    }
    super::d3d9resource::release_bindings(e, this.addr());
    let implicit: Vec<u32> = e
        .kernel
        .graphics
        .device_objects
        .surfaces
        .iter()
        .filter(|(_, s)| s.device == this.addr() && s.implicit)
        .map(|(a, _)| *a)
        .collect();
    for surface in implicit {
        e.kernel.graphics.device_objects.surfaces.remove(&surface);
        #[cfg(windows)]
        super::d3d9real::forget(e, surface);
        e.mem.free(surface);
    }
    // The new swap chain.
    let format = field(e, PP_FORMAT);
    let (width, height, quality) = (
        field(e, PP_WIDTH),
        field(e, PP_HEIGHT),
        field(e, PP_QUALITY),
    );
    let back_buffers: Vec<u32> = (0..field(e, PP_COUNT))
        .map(|_| {
            let desc = [
                format,
                D3DRTYPE_SURFACE,
                D3DUSAGE_RENDERTARGET,
                0,
                0,
                quality,
                width,
                height,
            ];
            new_surface(e, this.addr(), true, desc)
        })
        .collect();
    let depth_stencil = if field(e, PP_AUTO_DEPTH) != 0 {
        let desc = [
            field(e, PP_DEPTH_FORMAT),
            D3DRTYPE_SURFACE,
            D3DUSAGE_DEPTHSTENCIL,
            0,
            0,
            quality,
            width,
            height,
        ];
        new_surface(e, this.addr(), true, desc)
    } else {
        0
    };
    surface_add_ref(e, Ptr::new(back_buffers[0]));
    if depth_stencil != 0 {
        surface_add_ref(e, Ptr::new(depth_stencil));
    }
    let bytes = e.mem.bytes(p, PRESENT_SIZE);
    {
        let d = device(e, this);
        d.present = bytes.clone();
        d.back_buffers = back_buffers.clone();
        d.render_targets[0] = back_buffers[0];
        d.depth_stencil = depth_stencil;
        d.viewport = [0, 0, width, height, 0f32.to_bits(), 1f32.to_bits()];
        d.scissor = [0, 0, width, height];
        d.lost = false;
    }
    #[cfg(windows)]
    super::d3d9real::device_reset(e, this.addr(), &bytes, &back_buffers, depth_stencil);
    D3D_OK
}

/// `QueryInterface(iid, out)`: `IUnknown`, `IDirect3DResource9`, `IDirect3DBaseTexture9` and
/// `IDirect3DTexture9` answer the texture (counted); others `E_NOINTERFACE` with a null out.
fn texture_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(
        e,
        iid,
        &[IID_IUNKNOWN, IID_RESOURCE9, IID_BASETEXTURE9, IID_TEXTURE9],
    ) {
        texture_add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn texture_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let t = texture(e, this);
    t.references += 1;
    let left = t.references;
    note_reference(e, this, "AddRef", left);
    left
}

/// `NV_D3D9_REFS=FILE` (a diagnostic, not Direct3D behaviour): every count change of a texture is
/// appended to `FILE` with the count after it and the exe functions it came from (innermost
/// first), to find a reference given back once too often. Off by default.
fn note_reference(e: &Engine, this: Ptr, what: &str, left: u32) {
    static FILE: std::sync::OnceLock<Option<std::sync::Mutex<std::fs::File>>> =
        std::sync::OnceLock::new();
    let Some(file) = FILE.get_or_init(|| {
        let path = std::env::var_os("NV_D3D9_REFS")?;
        std::fs::File::create(path).ok().map(std::sync::Mutex::new)
    }) else {
        return;
    };
    let calls: Vec<String> = e
        .call_stack
        .iter()
        .rev()
        .take(14)
        .map(|a| format!("{a:08x}"))
        .collect();
    if let Ok(mut f) = file.lock() {
        use std::io::Write;
        let _ = writeln!(f, "{:08x}	{what}	{left}	{}", this.addr(), calls.join(" < "));
    }
}

/// `Release()`: the last reference frees the texture, its levels and their memory, and
/// releases the device's reference the texture held.
fn texture_release(e: &mut Engine, this: Ptr) -> u32 {
    let t = texture(e, this);
    t.references -= 1;
    let (left, bound) = (t.references, t.bound);
    note_reference(e, this, "Release", left);
    if left == 0 && bound == 0 {
        texture_destroy(e, this);
    }
    left
}

/// A sampler lets go of `this`; the texture goes once neither the program nor a sampler holds it.
fn texture_unbind(e: &mut Engine, this: Ptr) {
    let t = texture(e, this);
    t.bound -= 1;
    if t.bound == 0 && t.references == 0 {
        texture_destroy(e, this);
    }
}

fn texture_destroy(e: &mut Engine, this: Ptr) {
    note_reference(e, this, "destroyed", 0);
    {
        let t = e
            .kernel
            .graphics
            .device_objects
            .textures
            .remove(&this.addr())
            .expect("the texture");
        for level in t.levels {
            let s = e
                .kernel
                .graphics
                .device_objects
                .surfaces
                .remove(&level)
                .expect("the level");
            e.mem.free(s.bits);
            #[cfg(windows)]
            super::d3d9real::forget(e, level);
            e.mem.free(level);
        }
        #[cfg(windows)]
        super::d3d9real::forget(e, this.addr());
        e.mem.free(this.addr());
        device_release(e, Ptr::new(t.device));
    }
}

/// `GetDevice(out)`: the texture's device, counted.
fn texture_get_device(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let owner = texture(e, this).device;
    device_add_ref(e, Ptr::new(owner));
    e.mem.set_u32(out.addr(), owner);
    D3D_OK
}

/// `SetPriority(priority)`: kept for a managed texture (others keep 0); answers the old one.
fn texture_set_priority(e: &mut Engine, this: Ptr, priority: u32) -> u32 {
    let t = texture(e, this);
    if t.pool != D3DPOOL_MANAGED {
        return 0;
    }
    std::mem::replace(&mut t.priority, priority)
}

/// `GetPriority()`.
fn texture_get_priority(e: &mut Engine, this: Ptr) -> u32 {
    texture(e, this).priority
}

/// `PreLoad()`: a hint about managed resources; nothing to do here.
fn texture_pre_load(_e: &mut Engine, _this: Ptr) {}

/// `GetType()`: `D3DRTYPE_TEXTURE`.
fn texture_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    D3DRTYPE_TEXTURE
}

/// `SetLOD(level)`: for a managed texture the most detailed level kept (at most the last);
/// answers the old one. Other pools keep 0 and answer 0.
fn texture_set_lod(e: &mut Engine, this: Ptr, lod: u32) -> u32 {
    let t = texture(e, this);
    if t.pool != D3DPOOL_MANAGED {
        return 0;
    }
    let last = t.levels.len() as u32 / t.faces - 1;
    std::mem::replace(&mut t.lod, lod.min(last))
}

/// `SetLOD`, then (`NV_D3D9=real`) the same on the real texture, whose sampling starts at that
/// level too (a managed texture; the model's answer stands).
fn fwd_texture_set_lod(e: &mut Engine, this: Ptr, lod: u32) -> u32 {
    let previous = texture_set_lod(e, this, lod);
    #[cfg(windows)]
    if texture(e, this).pool == D3DPOOL_MANAGED {
        super::d3d9real::texture_set_lod(e, this.addr(), lod, previous);
    }
    previous
}

/// `GetLOD()`.
fn texture_get_lod(e: &mut Engine, this: Ptr) -> u32 {
    texture(e, this).lod
}

/// `GetLevelCount()`.
fn texture_get_level_count(e: &mut Engine, this: Ptr) -> u32 {
    let t = texture(e, this);
    t.levels.len() as u32 / t.faces
}

/// A level's surface (face 0's for a cube texture: its levels all have the same description).
fn texture_level(e: &mut Engine, this: Ptr, level: u32) -> Option<u32> {
    let t = texture(e, this);
    if level >= t.levels.len() as u32 / t.faces {
        return None;
    }
    t.levels.get(level as usize).copied()
}

/// A cube texture's surface for (face, level).
fn cube_level(e: &mut Engine, this: Ptr, face: u32, level: u32) -> Option<u32> {
    let t = texture(e, this);
    let count = t.levels.len() as u32 / t.faces;
    if face >= t.faces || level >= count {
        return None;
    }
    t.levels.get((face * count + level) as usize).copied()
}

/// `IID_IDirect3DCubeTexture9` (d3d9.h).
const IID_CUBETEXTURE9: [u8; 16] = guid(
    0xfff3_2f81,
    0xd953,
    0x473a,
    [0x92, 0x23, 0x93, 0xd6, 0x52, 0xab, 0xa9, 0x3f],
);
/// `D3DRTYPE_CUBETEXTURE`; the cube-map caps `D3DPTEXTURECAPS_CUBEMAP_POW2`.
const D3DRTYPE_CUBETEXTURE: u32 = 5;
const D3DPTEXTURECAPS_CUBEMAP_POW2: u32 = 0x2_0000;

/// `QueryInterface(iid, out)`: `IUnknown`, `IDirect3DResource9`, `IDirect3DBaseTexture9` and
/// `IDirect3DCubeTexture9` answer the cube texture (counted); others `E_NOINTERFACE`.
fn cube_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(
        e,
        iid,
        &[
            IID_IUNKNOWN,
            IID_RESOURCE9,
            IID_BASETEXTURE9,
            IID_CUBETEXTURE9,
        ],
    ) {
        texture_add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

/// `GetType()`: `D3DRTYPE_CUBETEXTURE`.
fn cube_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    D3DRTYPE_CUBETEXTURE
}

/// `GetCubeMapSurface(face, level, out)`: that surface, counted (with the texture).
fn cube_get_surface(e: &mut Engine, this: Ptr, face: u32, level: u32, out: Ptr) -> u32 {
    match cube_level(e, this, face, level) {
        Some(surface) if !out.is_null() => {
            texture_add_ref(e, this);
            e.mem.set_u32(out.addr(), surface);
            D3D_OK
        }
        _ => D3DERR_INVALIDCALL,
    }
}

/// `LockRect(face, level, locked out, rectangle, flags)`: [`surface_lock_rect`] on it.
fn cube_lock_rect(
    e: &mut Engine,
    this: Ptr,
    face: u32,
    level: u32,
    locked: Ptr,
    rect: Ptr,
    flags: u32,
) -> u32 {
    match cube_level(e, this, face, level) {
        Some(surface) => surface_lock_rect(e, Ptr::new(surface), locked, rect, flags),
        None => D3DERR_INVALIDCALL,
    }
}

/// `UnlockRect(face, level)`: [`surface_unlock_rect`] on it.
fn cube_unlock_rect(e: &mut Engine, this: Ptr, face: u32, level: u32) -> u32 {
    match cube_level(e, this, face, level) {
        Some(surface) => surface_unlock_rect(e, Ptr::new(surface)),
        None => D3DERR_INVALIDCALL,
    }
}

/// `GetLevelDesc(level, out)`: that level's `D3DSURFACE_DESC`; a level past the count or a
/// null out answers `D3DERR_INVALIDCALL`.
fn texture_get_level_desc(e: &mut Engine, this: Ptr, level: u32, out: Ptr) -> u32 {
    match texture_level(e, this, level) {
        Some(surface) if !out.is_null() => surface_get_desc(e, Ptr::new(surface), out),
        _ => D3DERR_INVALIDCALL,
    }
}

/// `GetSurfaceLevel(level, out)`: that level's surface, counted (with the texture).
fn texture_get_surface_level(e: &mut Engine, this: Ptr, level: u32, out: Ptr) -> u32 {
    match texture_level(e, this, level) {
        Some(surface) if !out.is_null() => {
            texture_add_ref(e, this);
            e.mem.set_u32(out.addr(), surface);
            D3D_OK
        }
        _ => D3DERR_INVALIDCALL,
    }
}

/// `LockRect(level, locked out, rectangle, flags)`: [`surface_lock_rect`] on that level.
fn texture_lock_rect(
    e: &mut Engine,
    this: Ptr,
    level: u32,
    locked: Ptr,
    rect: Ptr,
    flags: u32,
) -> u32 {
    match texture_level(e, this, level) {
        Some(surface) => surface_lock_rect(e, Ptr::new(surface), locked, rect, flags),
        None => D3DERR_INVALIDCALL,
    }
}

/// `UnlockRect(level)`: [`surface_unlock_rect`] on that level.
fn texture_unlock_rect(e: &mut Engine, this: Ptr, level: u32) -> u32 {
    match texture_level(e, this, level) {
        Some(surface) => surface_unlock_rect(e, Ptr::new(surface)),
        None => D3DERR_INVALIDCALL,
    }
}

/// `LockRect(locked out, rectangle, flags)`: a surface with memory, not locked, answers its
/// `D3DLOCKED_RECT` (pitch, then the address of the rectangle's first block: the whole surface
/// for a null rectangle); one already locked, one without memory (`D3DPOOL_DEFAULT` and not
/// dynamic, the back buffers) or a null out answers `D3DERR_INVALIDCALL`. The flags
/// (`D3DLOCK_READONLY`, `D3DLOCK_DISCARD`, ...) change nothing here.
fn surface_lock_rect(e: &mut Engine, this: Ptr, locked: Ptr, rect: Ptr, _flags: u32) -> u32 {
    let s = surface(e, this);
    if locked.is_null() || s.bits == 0 || s.locked {
        return D3DERR_INVALIDCALL;
    }
    let (bits, pitch, format) = (s.bits, s.pitch, s.desc[0]);
    s.locked = true;
    let at = if rect.is_null() {
        bits
    } else {
        let (block, bytes) = format_blocks(format).expect("a sized format");
        let left = e.mem.u32(rect.addr());
        let top = e.mem.u32(rect.addr() + 4);
        bits + (top / block) * pitch + (left / block) * bytes
    };
    e.mem.set_u32(locked.addr(), pitch);
    e.mem.set_u32(locked.addr() + 4, at);
    D3D_OK
}

/// `UnlockRect()`: a locked surface is unlocked; one not locked answers `D3DERR_INVALIDCALL`.
fn surface_unlock_rect(e: &mut Engine, this: Ptr) -> u32 {
    let s = surface(e, this);
    if !s.locked {
        return D3DERR_INVALIDCALL;
    }
    s.locked = false;
    #[cfg(windows)]
    {
        let (bits, pitch, desc) = (s.bits, s.pitch, s.desc);
        let (_, size) = level_layout(desc[0], desc[6], desc[7]);
        super::d3d9real::surface_unlocked(e, this.addr(), bits, pitch, pitch, size / pitch.max(1));
    }
    D3D_OK
}

/// The present parameters a device took (for the viewer and tests).
pub fn device_present_parameters(e: &Engine, device: u32) -> Option<Vec<u8>> {
    e.kernel
        .graphics
        .device_objects
        .devices
        .get(&device)
        .map(|d| d.present.clone())
}

// The fixed-function and copy calls below answer as the retail runtime did on this machine
// (2026-10-27, `scripts/d3d9-capture device fixed`, windowed HAL devices with hardware and with
// mixed vertex processing; cases that faulted the runtime were run one per process). Nothing is
// drawn yet: the states are kept, the copies only answered.

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetMaterial (vtable +0xC4)
/// `SetMaterial(material)`: keeps the 17 words of the `D3DMATERIAL9` and answers `D3D_OK` (the
/// runtime checks nothing in it). A null material faulted the runtime: not provided.
fn set_material(e: &mut Engine, this: Ptr, material: Ptr) -> u32 {
    if material.is_null() {
        panic!("IDirect3DDevice9::SetMaterial(NULL) is not provided (the runtime faulted)");
    }
    let words: [u32; 17] = std::array::from_fn(|i| e.mem.u32(material.addr() + 4 * i as u32));
    device(e, this).material = Some(words);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetLight (vtable +0xCC)
/// `SetLight(index, light)`: keeps the 26 words of the `D3DLIGHT9` for the index and answers
/// `D3D_OK` whatever they hold: the runtime took types 0 and 4, zero attenuation, negative or
/// zero ranges, a zero direction, theta above phi, phi above pi and NaN colours alike. A null
/// light faulted the runtime: not provided.
fn set_light(e: &mut Engine, this: Ptr, index: u32, light: Ptr) -> u32 {
    if light.is_null() {
        panic!("IDirect3DDevice9::SetLight({index}, NULL) is not provided (the runtime faulted)");
    }
    let words: [u32; 26] = std::array::from_fn(|i| e.mem.u32(light.addr() + 4 * i as u32));
    device(e, this).lights.insert(index, words);
    D3D_OK
}

/// The light `LightEnable` makes for an index never set (read back with `GetLight`): a
/// directional light (type 3) with diffuse (1, 1, 1, 0) and direction (0, 0, 1), everything
/// else 0.
fn default_light() -> [u32; 26] {
    let mut l = [0u32; 26];
    l[0] = 3;
    l[1] = 1f32.to_bits();
    l[2] = 1f32.to_bits();
    l[3] = 1f32.to_bits();
    l[18] = 1f32.to_bits();
    l
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::LightEnable (vtable +0xD4)
/// `LightEnable(index, enable)`: enables (any nonzero value) or disables the light and answers
/// `D3D_OK`; an index never set first gets [`default_light`]. The runtime set no limit (ten more
/// lights than any caps count were taken).
fn light_enable(e: &mut Engine, this: Ptr, index: u32, enable: u32) -> u32 {
    let d = device(e, this);
    d.lights.entry(index).or_insert_with(default_light);
    if enable != 0 {
        d.enabled_lights.insert(index);
    } else {
        d.enabled_lights.remove(&index);
    }
    D3D_OK
}

/// `D3DCREATE_MIXED_VERTEXPROCESSING`.
const D3DCREATE_MIXED_VERTEXPROCESSING: u32 = 0x80;

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetSoftwareVertexProcessing (vtable +0x134)
/// `SetSoftwareVertexProcessing(software)`: on a device with mixed vertex processing any value
/// is taken (nonzero switches to software) and `D3D_OK` answered; on one with hardware vertex
/// processing FALSE answers `D3D_OK` and anything else `D3DERR_INVALIDCALL` (measured with 1
/// and 2).
fn set_software_vertex_processing(e: &mut Engine, this: Ptr, software: u32) -> u32 {
    let d = device(e, this);
    if d.creation[3] & D3DCREATE_MIXED_VERTEXPROCESSING != 0 {
        d.software_vertex_processing = software != 0;
        return D3D_OK;
    }
    if software == 0 {
        D3D_OK
    } else {
        D3DERR_INVALIDCALL
    }
}

/// `D3DPOOL_SYSTEMMEM`.
const D3DPOOL_SYSTEMMEM: u32 = 2;

// Platform: replaces D3D9.DLL IDirect3DDevice9::UpdateTexture (vtable +0x7C)
/// `UpdateTexture(source, destination)` for textures of this layer: a system-memory source and a
/// default-pool destination answer `D3D_OK` (the runtime took mismatched sizes, formats and
/// level counts there, in a scene and outside one); any other pair of pools answers
/// `D3DERR_INVALIDCALL`. Nothing is drawn, so no level is copied (a default-pool texture is not
/// read back here). A null source faulted the runtime: not provided; so is a destination that
/// is not a texture of this layer.
fn update_texture(e: &mut Engine, _this: Ptr, source: u32, destination: u32) -> u32 {
    let pools = |e: &Engine, t: u32| {
        e.kernel
            .graphics
            .device_objects
            .textures
            .get(&t)
            .map(|t| t.pool)
    };
    let (Some(from), Some(to)) = (pools(e, source), pools(e, destination)) else {
        panic!("IDirect3DDevice9::UpdateTexture({source:#x}, {destination:#x}) on objects that are not textures of this layer is not provided");
    };
    if from == D3DPOOL_SYSTEMMEM && to == D3DPOOL_DEFAULT {
        D3D_OK
    } else {
        D3DERR_INVALIDCALL
    }
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::StretchRect (vtable +0x88)
/// `StretchRect(source, source rect, destination, destination rect, filter)` for surfaces of
/// this layer: the destination must be a render target (a render-target surface or a level of a
/// render-target texture) and the source a default-pool surface (a render target, a level of a
/// render-target or default-pool texture) other than the destination; then `D3D_OK` (stretching,
/// rectangles, a format conversion and each filter were taken, in a scene and outside one),
/// else `D3DERR_INVALIDCALL` (a system-memory source, a destination that is no render target,
/// the same surface, a null destination). Nothing is drawn, so nothing is copied. A source that
/// is not a surface of this layer is not provided.
fn stretch_rect(
    e: &mut Engine,
    _this: Ptr,
    source: u32,
    _source_rect: u32,
    destination: u32,
    _destination_rect: u32,
    _filter: u32,
) -> u32 {
    let desc = |e: &Engine, s: u32| {
        e.kernel
            .graphics
            .device_objects
            .surfaces
            .get(&s)
            .map(|s| (s.desc, s.container))
    };
    let Some((from, from_texture)) = desc(e, source) else {
        panic!("IDirect3DDevice9::StretchRect from {source:#x}, which is not a surface of this layer, is not provided");
    };
    let Some((to, _)) = desc(e, destination) else {
        return D3DERR_INVALIDCALL;
    };
    // A depth-stencil surface on either side was not measured.
    if (from[2] | to[2]) & D3DUSAGE_DEPTHSTENCIL != 0 {
        panic!("IDirect3DDevice9::StretchRect with a depth-stencil surface is not provided");
    }
    let source_kind_measured =
        from[2] & D3DUSAGE_RENDERTARGET != 0 || from_texture != 0 || from[3] == D3DPOOL_SYSTEMMEM;
    if !source_kind_measured {
        panic!("IDirect3DDevice9::StretchRect from a kind of surface not measured is not provided");
    }
    // `D3DSURFACE_DESC`: Usage +2, Pool +3.
    let ok = source != destination
        && to[2] & D3DUSAGE_RENDERTARGET != 0
        && to[3] == D3DPOOL_DEFAULT
        && from[3] == D3DPOOL_DEFAULT;
    if ok {
        D3D_OK
    } else {
        D3DERR_INVALIDCALL
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::platform::d3d9;

    /// TEST DATA, not a capture: a small adapter table in `scripts/d3d9-capture`'s format (a
    /// 640x480 X8R8G8B8 desktop, HAL caps with four simultaneous render targets, a windowed
    /// X8R8G8B8 back buffer accepted).
    fn test_table() -> String {
        let hex = |b: &[u8]| -> String { b.iter().map(|x| format!("{x:02x}")).collect() };
        let mut mode = vec![0u8; 16];
        mode[..4].copy_from_slice(&640u32.to_le_bytes());
        mode[4..8].copy_from_slice(&480u32.to_le_bytes());
        mode[8..12].copy_from_slice(&60u32.to_le_bytes());
        mode[12..].copy_from_slice(&0x16u32.to_le_bytes());
        let mut caps = vec![0u8; 304];
        caps[..4].copy_from_slice(&1u32.to_le_bytes());
        caps[CAPS_SIMULTANEOUS_RTS..CAPS_SIMULTANEOUS_RTS + 4].copy_from_slice(&4u32.to_le_bytes());
        [
            format!("GetAdapterDisplayMode\t0\t0\t{}", hex(&mode)),
            format!("GetDeviceCaps\t0 1\t0\t{}", hex(&caps)),
            "Domain\td0\t0,1".to_string(),
            "CheckDeviceType\t0 1 16 16\t0\t\td0".to_string(),
        ]
        .join("\n")
    }

    /// An engine with the table and an `IDirect3D9`, and windowed present parameters (640x480,
    /// format and count left for the defaults, device window 0x11c).
    fn d3d() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        d3d9::load_adapter_table(&mut e, &test_table());
        let create = d3d9::proc_address(&mut e, b"Direct3DCreate9").unwrap();
        let d3d = e.call(create, &[d3d9::D3D_SDK_VERSION]).u32();
        let present = e.mem.alloc(PRESENT_SIZE);
        e.mem.set_u32(present + PP_WIDTH, 640);
        e.mem.set_u32(present + PP_HEIGHT, 480);
        e.mem.set_u32(present + PP_WINDOW, 0x11c);
        e.mem.set_u32(present + PP_WINDOWED, 1);
        (e, d3d, present)
    }

    const CREATE_DEVICE: u32 = 0x40;

    #[test]
    fn create_device_fills_the_defaults_and_makes_the_back_buffer_render_target_0() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        let hr = e
            .vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out])
            .u32();
        assert_eq!(hr, D3D_OK);
        let device = e.mem.u32(out);
        assert_eq!(
            e.mem.u32(device),
            e.kernel.graphics.device_objects.device_vtable
        );
        assert_eq!(e.mem.u32(present + PP_COUNT), 1);
        assert_eq!(e.mem.u32(present + PP_FORMAT), 0x16);
        // The device holds a reference to the IDirect3D9.
        assert_eq!(e.vcall(d3d, 4, &[]).u32(), 3);
        e.vcall(d3d, 8, &[]);
        let slot = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 4 * 38, &[0, slot]).u32(), D3D_OK);
        let target = e.mem.u32(slot);
        assert_eq!(e.vcall(device, 4 * 18, &[0, 0, 0, slot]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(slot), target);
        assert_eq!(
            e.vcall(device, 4 * 18, &[0, 1, 0, slot]).u32(),
            D3DERR_INVALIDCALL
        );
        let desc = e.mem.alloc(0x20);
        assert_eq!(e.vcall(target, 4 * 12, &[desc]).u32(), D3D_OK);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(desc + 4 * i)).collect();
        assert_eq!(words, [0x16, 1, 1, 0, 0, 0, 640, 480]);
        assert_eq!(e.vcall(target, 4 * 10, &[]).u32(), D3DRTYPE_SURFACE);
        // Two references taken, two released; the back buffer stays with the device.
        assert_eq!(e.vcall(target, 8, &[]).u32(), 2);
        assert_eq!(e.vcall(target, 8, &[]).u32(), 1);
        assert!(e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .contains_key(&target));
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3DERR_NOTFOUND);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.vcall(device, 4 * 38, &[1, slot]).u32(), D3DERR_NOTFOUND);
        assert_eq!(
            e.vcall(device, 4 * 38, &[4, slot]).u32(),
            D3DERR_INVALIDCALL
        );
        let viewport = e.mem.alloc(0x18);
        assert_eq!(e.vcall(device, 4 * 48, &[viewport]).u32(), D3D_OK);
        assert_eq!(
            (e.mem.u32(viewport + 8), e.mem.u32(viewport + 0xc)),
            (640, 480)
        );
        assert_eq!(e.mem.u32(viewport + 0x14), 1f32.to_bits());
        e.mem.set_u32(viewport + 8, 641);
        assert_eq!(
            e.vcall(device, 4 * 47, &[viewport]).u32(),
            D3DERR_INVALIDCALL
        );
        let params = e.mem.alloc(0x10);
        assert_eq!(e.vcall(device, 4 * 9, &[params]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(params + 0xc), 0x40);
        let caps = e.mem.alloc(304);
        assert_eq!(e.vcall(device, 4 * 7, &[caps]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(caps), 1);
        assert_eq!(e.vcall(device, 4 * 2, &[]).u32(), 0);
        assert!(e.kernel.graphics.device_objects.devices.is_empty());
        assert!(e.kernel.graphics.device_objects.surfaces.is_empty());
        assert_eq!(e.vcall(d3d, 4, &[]).u32(), 2);
    }

    #[test]
    fn create_device_refuses_what_direct3d_refuses() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        let create = |e: &mut Engine, behavior: u32| {
            e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, behavior, present, out])
                .u32()
        };
        assert_eq!(create(&mut e, 0), D3DERR_INVALIDCALL);
        assert_eq!(create(&mut e, 0x40 | 0x80), D3DERR_INVALIDCALL);
        e.mem.set_u32(present + PP_COUNT, 4);
        assert_eq!(create(&mut e, 0x40), D3DERR_INVALIDCALL);
        assert_eq!(e.mem.u32(present + PP_COUNT), 3);
        e.mem.set_u32(present + PP_REFRESH, 60);
        assert_eq!(create(&mut e, 0x40), D3DERR_INVALIDCALL);
        e.mem.set_u32(present + PP_REFRESH, 0);
        assert_eq!(create(&mut e, 0x40), D3D_OK);
        let device = e.mem.u32(out);
        let slot = e.mem.alloc(4);
        for i in 0..3 {
            assert_eq!(e.vcall(device, 4 * 18, &[0, i, 0, slot]).u32(), D3D_OK);
        }
        let hr = e
            .vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, 0, out])
            .u32();
        assert_eq!(hr, D3DERR_INVALIDCALL);
    }

    #[test]
    fn render_target_and_depth_slots_hold_their_surfaces() {
        let (mut e, d3d, present) = d3d();
        e.mem.set_u32(present + PP_AUTO_DEPTH, 1);
        e.mem.set_u32(present + PP_DEPTH_FORMAT, 0x4b);
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let slot = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3D_OK);
        let depth = e.mem.u32(slot);
        assert_eq!(e.vcall(device, 4 * 38, &[0, slot]).u32(), D3D_OK);
        let target = e.mem.u32(slot);
        // The depth surface is no render target, the back buffer no depth surface.
        assert_eq!(
            e.vcall(device, 4 * 37, &[0, depth]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 39, &[target]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 37, &[0, 0]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 37, &[1, target]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 37, &[1, 0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 39, &[0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3DERR_NOTFOUND);
        assert_eq!(e.vcall(device, 4 * 39, &[depth]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 40, &[slot]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(slot), depth);
        let got = e.mem.alloc(4);
        assert_eq!(e.vcall(depth, 4 * 3, &[got]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(got), device);
        assert_eq!(e.vcall(device, 4 * 2, &[]).u32(), 1);
    }

    #[test]
    fn cube_textures_have_six_faces_of_lockable_levels() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        // TEST DATA: the table accepts A8R8G8B8 (0x15) cube textures with usage 0.
        let table =
            test_table() + "\nDomain\td1\t15,16\nCheckDeviceFormat\t0 1 16 0 5\t0\t16=8876086a\td1";
        d3d9::load_adapter_table(&mut e, &table);
        assert_eq!(
            e.vcall(device, 4 * 25, &[4, 0, 0, 0x16, 1, out, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 4 * 25, &[4, 0, 0, 0x15, 1, out, 0]).u32(),
            D3D_OK
        );
        let cube = e.mem.u32(out);
        assert_eq!(e.vcall(cube, 4 * 10, &[]).u32(), D3DRTYPE_CUBETEXTURE);
        assert_eq!(e.vcall(cube, 4 * 13, &[]).u32(), 3, "4, 2, 1");
        let desc = e.mem.alloc(0x20);
        assert_eq!(e.vcall(cube, 4 * 17, &[1, desc]).u32(), D3D_OK);
        assert_eq!((e.mem.u32(desc + 0x18), e.mem.u32(desc + 0x1c)), (2, 2));
        assert_eq!(e.vcall(cube, 4 * 17, &[3, desc]).u32(), D3DERR_INVALIDCALL);
        let slot = e.mem.alloc(4);
        assert_eq!(e.vcall(cube, 4 * 18, &[5, 2, slot]).u32(), D3D_OK);
        let last = e.mem.u32(slot);
        assert_eq!(e.vcall(cube, 4 * 18, &[0, 2, slot]).u32(), D3D_OK);
        assert_ne!(e.mem.u32(slot), last, "each face its own surface");
        assert_eq!(
            e.vcall(cube, 4 * 18, &[6, 0, slot]).u32(),
            D3DERR_INVALIDCALL
        );
        let locked = e.mem.alloc(8);
        assert_eq!(e.vcall(cube, 4 * 19, &[5, 0, locked, 0, 0]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(locked), 16, "4 pixels of 4 bytes");
        assert_eq!(e.vcall(cube, 4 * 20, &[5, 0]).u32(), D3D_OK);
        // Two surfaces taken, the texture's own reference: three releases free it.
        e.vcall(cube, 8, &[]);
        e.vcall(cube, 8, &[]);
        assert_eq!(e.vcall(cube, 8, &[]).u32(), 0);
        assert!(!e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .contains_key(&last));
    }

    #[test]
    fn render_targets_check_their_format_and_multisampling() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        // TEST DATA: A8R8G8B8 render targets accepted; 4 samples with 2 quality levels.
        let table = test_table()
            + "\nDomain\td1\t15,16\nCheckDeviceFormat\t0 1 16 1 1\t8876086a\t15=0\td1"
            + "\nDomain\td2\t0,1,2,3,4\nCheckDeviceMultiSampleType\t0 1 15 1\t8876086a/0\t0=0/1,4=0/2\td2";
        d3d9::load_adapter_table(&mut e, &table);
        let create = |e: &mut Engine, format: u32, ms: u32, q: u32, lockable: u32| {
            e.vcall(device, 4 * 28, &[640, 480, format, ms, q, lockable, out, 0])
                .u32()
        };
        assert_eq!(create(&mut e, 0x16, 0, 0, 0), D3DERR_INVALIDCALL);
        assert_eq!(create(&mut e, 0x15, 2, 0, 0), D3DERR_INVALIDCALL);
        assert_eq!(create(&mut e, 0x15, 4, 2, 0), D3DERR_INVALIDCALL);
        assert_eq!(create(&mut e, 0x15, 4, 1, 0), D3D_OK);
        let target = e.mem.u32(out);
        let desc = e.mem.alloc(0x20);
        assert_eq!(e.vcall(target, 4 * 12, &[desc]).u32(), D3D_OK);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(desc + 4 * i)).collect();
        assert_eq!(words, [0x15, 1, 1, 0, 4, 1, 640, 480]);
        assert_eq!(create(&mut e, 0x15, 0, 0, 1), D3D_OK);
        let lockable = e.mem.u32(out);
        let locked = e.mem.alloc(8);
        assert_eq!(e.vcall(lockable, 4 * 13, &[locked, 0, 0]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(locked), 640 * 4);
        assert_eq!(e.vcall(lockable, 4 * 14, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(lockable, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(target, 8, &[]).u32(), 0);
    }

    #[test]
    fn depth_stencil_surfaces_are_made_unlockable_and_hold_their_device() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        // TEST DATA: the table accepts D24S8 (0x4b) as a depth-stencil surface, refuses D16 (0x50).
        let table =
            test_table() + "\nDomain\td1\t4b,50\nCheckDeviceFormat\t0 1 16 2 1\t0\t50=8876086a\td1";
        d3d9::load_adapter_table(&mut e, &table);
        let refs = |e: &mut Engine| {
            e.vcall(device, 4, &[]);
            e.vcall(device, 8, &[]).u32()
        };
        let before = refs(&mut e);
        let create = |e: &mut Engine, format: u32| {
            e.vcall(device, 4 * 29, &[1920, 1080, format, 0, 0, 0, out, 0])
                .u32()
        };
        assert_eq!(create(&mut e, 0x50), D3DERR_INVALIDCALL);
        assert_eq!(
            e.vcall(device, 4 * 29, &[0, 1080, 0x4b, 0, 0, 0, out, 0])
                .u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(create(&mut e, 0x4b), D3D_OK);
        let surface = e.mem.u32(out);
        assert_eq!(refs(&mut e), before + 1);
        let desc = e.mem.alloc(0x20);
        assert_eq!(e.vcall(surface, 4 * 12, &[desc]).u32(), D3D_OK);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(desc + 4 * i)).collect();
        assert_eq!(words, [0x4b, 1, 2, 0, 0, 0, 1920, 1080]);
        let locked = e.mem.alloc(8);
        assert_eq!(
            e.vcall(surface, 4 * 13, &[locked, 0, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(surface, 8, &[]).u32(), 0);
        assert!(!e
            .kernel
            .graphics
            .device_objects
            .surfaces
            .contains_key(&surface));
        assert_eq!(refs(&mut e), before);
    }

    #[test]
    fn textures_have_lockable_levels_that_count_with_the_texture() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        // TEST DATA: the table accepts R5G6B5 (0x17) and DXT1 textures with usage 0.
        let dxt1 = u32::from_le_bytes(*b"DXT1");
        let table = test_table()
            + "\nDomain\td1\t16,17,31545844\nCheckDeviceFormat\t0 1 16 0 3\t0\t16=8876086a\td1";
        d3d9::load_adapter_table(&mut e, &table);
        let hr = e
            .vcall(device, 4 * 23, &[4, 2, 0, 0, 0x17, 1, out, 0])
            .u32();
        assert_eq!(hr, D3D_OK);
        let texture = e.mem.u32(out);
        assert_eq!(e.vcall(texture, 4 * 13, &[]).u32(), 3, "4x2, 2x1, 1x1");
        assert_eq!(e.vcall(texture, 4 * 10, &[]).u32(), D3DRTYPE_TEXTURE);
        let desc = e.mem.alloc(0x20);
        assert_eq!(e.vcall(texture, 4 * 17, &[1, desc]).u32(), D3D_OK);
        assert_eq!((e.mem.u32(desc + 0x18), e.mem.u32(desc + 0x1c)), (2, 1));
        assert_eq!(e.mem.u32(desc + 0xc), 1, "managed");
        assert_eq!(
            e.vcall(texture, 4 * 17, &[3, desc]).u32(),
            D3DERR_INVALIDCALL
        );
        let slot = e.mem.alloc(4);
        assert_eq!(e.vcall(texture, 4 * 18, &[0, slot]).u32(), D3D_OK);
        let level = e.mem.u32(slot);
        // The level's reference is the texture's.
        assert_eq!(e.vcall(texture, 4, &[]).u32(), 3);
        assert_eq!(e.vcall(level, 8, &[]).u32(), 2);
        let locked = e.mem.alloc(8);
        assert_eq!(e.vcall(level, 4 * 13, &[locked, 0, 0]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(locked), 8, "4 pixels of 2 bytes");
        let bits = e.mem.u32(locked + 4);
        e.mem.set_u32(bits + 12, 0x1234_5678);
        assert_eq!(
            e.vcall(level, 4 * 13, &[locked, 0, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(level, 4 * 14, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(level, 4 * 14, &[]).u32(), D3DERR_INVALIDCALL);
        let rect = e.mem.alloc(16);
        e.mem.set_u32(rect, 2);
        e.mem.set_u32(rect + 4, 1);
        assert_eq!(
            e.vcall(texture, 4 * 19, &[0, locked, rect, 0]).u32(),
            D3D_OK
        );
        assert_eq!(e.mem.u32(locked + 4), bits + 12);
        assert_eq!(e.mem.u32(bits + 12), 0x1234_5678);
        assert_eq!(e.vcall(texture, 4 * 20, &[0]).u32(), D3D_OK);
        assert_eq!(e.vcall(level, 4 * 11, &[0, slot]).u32(), E_NOINTERFACE);
        // DXT1: blocks of 4 by 4 pixels, 8 bytes.
        let hr = e
            .vcall(device, 4 * 23, &[8, 8, 1, 0, dxt1, 1, out, 0])
            .u32();
        assert_eq!(hr, D3D_OK);
        let compressed = e.mem.u32(out);
        assert_eq!(
            e.vcall(compressed, 4 * 19, &[0, locked, 0, 0]).u32(),
            D3D_OK
        );
        assert_eq!(e.mem.u32(locked), 16);
        // Refusals: an unsupported format, too many levels, dynamic managed, a shared handle.
        let refused = |e: &mut Engine, words: [u32; 8]| e.vcall(device, 4 * 23, &words).u32();
        assert_eq!(
            refused(&mut e, [4, 4, 0, 0, 0x16, 1, out, 0]),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            refused(&mut e, [4, 4, 4, 0, 0x17, 1, out, 0]),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            refused(&mut e, [4, 4, 1, 0x200, 0x17, 1, out, 0]),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            refused(&mut e, [4, 4, 1, 0, 0x17, 1, out, 4]),
            D3DERR_INVALIDCALL
        );
        // The textures hold the device; releasing them lets it go.
        assert_eq!(e.vcall(texture, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(texture, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(compressed, 8, &[]).u32(), 0);
        assert!(e.kernel.graphics.device_objects.textures.is_empty());
        assert_eq!(e.vcall(device, 8, &[]).u32(), 0);
    }

    /// The Display menu's brightness (004dcef0) sets the gamma ramp through slot 21 (+0x54);
    /// without it the game's main loop stopped on the open slot (the maintainer's freeze).
    #[test]
    fn the_gamma_ramp_is_kept() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let ramp = e.mem.alloc(0x600);
        for i in 0..0x300 {
            e.mem.set_u16(ramp + 2 * i, (i * 85) as u16);
        }
        assert_eq!(e.vcall(device, 4 * 21, &[0, 1, ramp]).u32(), D3D_OK);
        let kept = super::device(&mut e, Ptr::new(device))
            .gamma_ramp
            .clone()
            .unwrap();
        assert_eq!(kept.len(), 0x300);
        assert_eq!(kept[0x201], (0x201u32 * 85) as u16);
    }

    #[test]
    fn states_are_kept_and_textures_held_by_their_samplers() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let value = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 4 * 57, &[7, 1]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 58, &[7, value]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(value), 1);
        assert_eq!(e.vcall(device, 4 * 67, &[1, 1, 4]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 66, &[1, 1, value]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(value), 4);
        assert_eq!(
            e.vcall(device, 4 * 67, &[8, 1, 4]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 69, &[257, 5, 2]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 68, &[257, 5, value]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(value), 2);
        assert_eq!(
            e.vcall(device, 4 * 69, &[16, 5, 2]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 107, &[0]).u32(), D3D_OK);
        // TEST DATA: the table accepts R5G6B5 (0x17) textures with usage 0.
        let table = test_table() + "\nDomain\td1\t17\nCheckDeviceFormat\t0 1 16 0 3\t0\t\td1";
        d3d9::load_adapter_table(&mut e, &table);
        e.vcall(device, 4 * 23, &[4, 4, 1, 0, 0x17, 1, out, 0]);
        let texture = e.mem.u32(out);
        assert_eq!(e.vcall(device, 4 * 65, &[0, texture]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 64, &[0, value]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(value), texture);
        // The program's reference and GetTexture's: the sampler's binding is not counted (the
        // retail runtime, `d3d9-capture device refs`, 2026-10-27); released to 0 while bound,
        // the texture lives on until the sampler lets go.
        assert_eq!(e.vcall(texture, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(texture, 8, &[]).u32(), 0);
        assert!(!e.kernel.graphics.device_objects.textures.is_empty());
        assert_eq!(e.vcall(device, 4 * 65, &[0, 0]).u32(), D3D_OK);
        assert!(e.kernel.graphics.device_objects.textures.is_empty());
        assert_eq!(e.vcall(device, 4 * 64, &[0, value]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(value), 0);
    }

    #[test]
    fn buffers_shaders_and_declarations_are_made_bound_and_released() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        // A vertex buffer of 64 bytes, locked twice, its description.
        assert_eq!(
            e.vcall(device, 4 * 26, &[64, 0, 0x142, 1, out, 0]).u32(),
            D3D_OK
        );
        let vb = e.mem.u32(out);
        let data = e.mem.alloc(4);
        assert_eq!(e.vcall(vb, 4 * 11, &[16, 0, data, 0]).u32(), D3D_OK);
        let bits = e.mem.u32(data) - 16;
        assert_eq!(e.vcall(vb, 4 * 11, &[0, 0, data, 0]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(data), bits);
        assert_eq!(
            e.vcall(vb, 4 * 11, &[60, 8, data, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(vb, 4 * 12, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(vb, 4 * 12, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(vb, 4 * 12, &[]).u32(), D3DERR_INVALIDCALL);
        let desc = e.mem.alloc(24);
        assert_eq!(e.vcall(vb, 4 * 13, &[desc]).u32(), D3D_OK);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(desc + 4 * i)).collect();
        assert_eq!(words, [100, 6, 0, 1, 64, 0x142]);
        assert_eq!(
            e.vcall(device, 4 * 26, &[64, 0x200, 0, 1, out, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        // An index buffer; a wrong format is refused.
        assert_eq!(
            e.vcall(device, 4 * 27, &[12, 0, 101, 1, out, 0]).u32(),
            D3D_OK
        );
        let ib = e.mem.u32(out);
        assert_eq!(
            e.vcall(device, 4 * 27, &[12, 0, 0x16, 1, out, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        // A shader: its tokens up to the end token, given back by GetFunction.
        let function = e.mem.alloc(16);
        for (i, token) in [0xfffe_0300u32, 0x1234, 0x0000_ffff].iter().enumerate() {
            e.mem.set_u32(function + 4 * i as u32, *token);
        }
        assert_eq!(e.vcall(device, 4 * 91, &[function, out]).u32(), D3D_OK);
        let vs = e.mem.u32(out);
        let size = e.mem.alloc(4);
        assert_eq!(e.vcall(vs, 4 * 4, &[0, size]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(size), 12);
        // A declaration of one element and the end.
        let elements = e.mem.alloc(16);
        e.mem.set_u16(elements + 8, 0xff);
        e.mem.set_u8(elements + 12, 17);
        assert_eq!(e.vcall(device, 4 * 86, &[elements, out]).u32(), D3D_OK);
        let declaration = e.mem.u32(out);
        assert_eq!(e.vcall(declaration, 4 * 4, &[0, size]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(size), 2);
        // Binding: kinds are checked, objects are held.
        assert_eq!(e.vcall(device, 4 * 100, &[0, vb, 0, 32]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 100, &[0, ib, 0, 32]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 4 * 100, &[16, vb, 0, 32]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 104, &[ib]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 104, &[vb]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 92, &[vs]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 107, &[vs]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 87, &[declaration]).u32(), D3D_OK);
        let got = e.mem.alloc(12);
        assert_eq!(
            e.vcall(device, 4 * 101, &[0, got, got + 4, got + 8]).u32(),
            D3D_OK
        );
        assert_eq!((e.mem.u32(got), e.mem.u32(got + 8)), (vb, 32));
        // GetStreamSource's reference and the program's; the stream's binding is not counted.
        assert_eq!(e.vcall(vb, 8, &[]).u32(), 1, "the program's");
        assert_eq!(e.vcall(vb, 8, &[]).u32(), 0, "none: still bound");
        assert!(e.kernel.graphics.device_objects.resources_hold(vb));
        // Constants.
        let four = e.mem.alloc(32);
        e.mem.set_u32(four + 16, 7);
        assert_eq!(
            e.vcall(device, 4 * 94, &[255, four, 2]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 109, &[3, four, 2]).u32(), D3D_OK);
        let back = e.mem.alloc(32);
        assert_eq!(e.vcall(device, 4 * 110, &[4, back, 1]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(back), 7);
        // Unbinding releases: the stream's buffer goes.
        assert_eq!(e.vcall(device, 4 * 100, &[0, 0, 0, 0]).u32(), D3D_OK);
        assert!(!e.kernel.graphics.device_objects.resources_hold(vb));
    }

    #[test]
    fn draws_answer_inside_a_scene_with_their_bindings() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        e.vcall(device, 4 * 26, &[64, 0, 0x142, 1, out, 0]);
        let vb = e.mem.u32(out);
        e.vcall(device, 4 * 27, &[12, 0, 101, 1, out, 0]);
        let ib = e.mem.u32(out);
        let elements = e.mem.alloc(16);
        e.mem.set_u16(elements + 8, 0xff);
        e.mem.set_u8(elements + 12, 17);
        e.vcall(device, 4 * 86, &[elements, out]);
        let declaration = e.mem.u32(out);
        e.vcall(device, 4 * 100, &[0, vb, 0, 32]);
        e.vcall(device, 4 * 104, &[ib]);
        e.vcall(device, 4 * 87, &[declaration]);
        assert_eq!(e.vcall(device, 4 * 41, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 81, &[4, 0, 2]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 82, &[4, 0, 0, 4, 0, 2]).u32(), D3D_OK);
        let held = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 4 * 88, &[held]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(held), declaration);
        assert_eq!(e.vcall(device, 4 * 88, &[0]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 42, &[]).u32(), D3D_OK);
        let outside = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(device, 4 * 81, &[4, 0, 2]);
        }));
        assert!(outside.is_err(), "a draw outside a scene stops the run");
    }

    #[test]
    fn transforms_scenes_and_clears_follow_direct3d() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let m = e.mem.alloc(64);
        assert_eq!(e.vcall(device, 4 * 45, &[256, m]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(m), 1f32.to_bits());
        assert_eq!(e.mem.u32(m + 4), 0);
        e.mem.set_u32(m + 4, 2f32.to_bits());
        assert_eq!(e.vcall(device, 4 * 44, &[2, m]).u32(), D3D_OK);
        let back = e.mem.alloc(64);
        assert_eq!(e.vcall(device, 4 * 45, &[2, back]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(back + 4), 2f32.to_bits());
        assert_eq!(e.vcall(device, 4 * 44, &[4, m]).u32(), D3DERR_INVALIDCALL);
        let rect = e.mem.alloc(16);
        assert_eq!(e.vcall(device, 4 * 76, &[rect]).u32(), D3D_OK);
        assert_eq!((e.mem.u32(rect + 8), e.mem.u32(rect + 12)), (640, 480));
        assert_eq!(e.vcall(device, 4 * 41, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 41, &[]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(
            e.vcall(device, 4 * 17, &[0, 0, 0, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 42, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 42, &[]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 17, &[0, 0, 0, 0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 43, &[0, 0, 1, 0, 0, 0]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 43, &[0, 0, 2, 0, 0, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 4 * 43, &[1, 0, 1, 0, 0, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 3, &[]).u32(), D3D_OK);
    }

    /// `DrawPrimitiveUP` as the retail runtime answered it (`d3d9_resource::draw_primitive_up`):
    /// refused outside a scene until one has succeeded in a scene, then taken anywhere; stride
    /// 0 refused; stream 0 left as measured.
    #[test]
    fn draws_from_memory_answer_and_leave_stream_zero_as_measured() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let vertices = e.mem.alloc(0x90);
        let stream = |e: &mut Engine| {
            let words = e.mem.alloc(12);
            assert_eq!(
                e.vcall(device, 4 * 101, &[0, words, words + 4, words + 8])
                    .u32(),
                D3D_OK
            );
            let held = e.mem.u32(words);
            if held != 0 {
                e.vcall(held, 8, &[]);
            }
            (held, e.mem.u32(words + 4), e.mem.u32(words + 8))
        };
        // Without an FVF or a declaration the run stops.
        let bare = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(device, 4 * 83, &[5, 2, vertices, 0x18]);
        }));
        assert!(bare.is_err());
        e.vcall(device, 4 * 89, &[0x104]);
        e.vcall(device, 4 * 26, &[0x90, 8, 0x104, 0, out, 0]);
        let vb = e.mem.u32(out);
        // Outside a scene on a fresh device: refused, stream 0 emptied with the stride kept.
        e.vcall(device, 4 * 100, &[0, vb, 0x18, 0x18]);
        assert_eq!(
            e.vcall(device, 4 * 83, &[5, 2, vertices, 0x18]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(stream(&mut e), (0, 0, 0x18));
        // An empty scene does not change that.
        e.vcall(device, 4 * 41, &[]);
        e.vcall(device, 4 * 42, &[]);
        assert_eq!(
            e.vcall(device, 4 * 83, &[5, 2, vertices, 0x18]).u32(),
            D3DERR_INVALIDCALL
        );
        // In a scene: taken whatever the type, count and pointer; stream 0 emptied.
        e.vcall(device, 4 * 41, &[]);
        e.vcall(device, 4 * 100, &[0, vb, 0x18, 0x18]);
        for (kind, count, data) in [
            (5u32, 2u32, vertices),
            (2, 6, vertices),
            (0, 2, vertices),
            (9, 2, vertices),
            (5, 0, vertices),
            (5, 2, 0),
        ] {
            assert_eq!(
                e.vcall(device, 4 * 83, &[kind, count, data, 0x18]).u32(),
                D3D_OK
            );
        }
        assert_eq!(stream(&mut e), (0, 0, 0));
        // Stride 0: refused, the buffer kept with offset and stride 0.
        e.vcall(device, 4 * 100, &[0, vb, 0x18, 0x18]);
        assert_eq!(
            e.vcall(device, 4 * 83, &[5, 2, vertices, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(stream(&mut e), (vb, 0, 0));
        e.vcall(device, 4 * 42, &[]);
        // After a draw from memory in a scene, taken outside one too.
        assert_eq!(
            e.vcall(device, 4 * 83, &[5, 2, vertices, 0x18]).u32(),
            D3D_OK
        );
        assert_eq!(stream(&mut e), (0, 0, 0));
        // The stream's buffer was released by the draws: only the creator's reference is left.
        assert_eq!(e.vcall(vb, 8, &[]).u32(), 0);
    }

    /// The fixed-function states and the copies as the retail runtime answered them
    /// (`set_material` .. `stretch_rect`).
    #[test]
    fn lights_materials_and_copies_answer_as_measured() {
        let (mut e, d3d, present) = d3d();
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let material = e.mem.alloc(68);
        assert_eq!(e.vcall(device, 4 * 49, &[material]).u32(), D3D_OK);
        let light = e.mem.alloc(104);
        e.mem.set_u32(light, 4); // a type the runtime took anyway
        assert_eq!(e.vcall(device, 4 * 51, &[3, light]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 53, &[3, 2]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 53, &[7, 1]).u32(), D3D_OK);
        let d = device_state(&e, device);
        assert_eq!(d.lights[&7], default_light());
        assert_eq!(d.lights[&3][0], 4);
        assert!(d.enabled_lights.contains(&3) && d.enabled_lights.contains(&7));
        assert_eq!(e.vcall(device, 4 * 53, &[7, 0]).u32(), D3D_OK);
        assert!(!device_state(&e, device).enabled_lights.contains(&7));
        for null in [4 * 49, 4 * 51] {
            let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let args: &[u32] = if null == 4 * 49 { &[0] } else { &[0, 0] };
                e.vcall(device, null, args);
            }));
            assert!(stopped.is_err(), "a null pointer faulted the runtime");
        }
        // Software vertex processing: refused on a hardware device unless FALSE.
        assert_eq!(e.vcall(device, 4 * 77, &[1]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 4 * 77, &[0]).u32(), D3D_OK);
        let mixed = {
            e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x80, present, out]);
            e.mem.u32(out)
        };
        assert_eq!(e.vcall(mixed, 4 * 77, &[2]).u32(), D3D_OK);
        assert!(device_state(&e, mixed).software_vertex_processing);
        // TEST DATA: A8R8G8B8 (0x15) textures and render targets accepted.
        let table = test_table()
            + "\nDomain\td1\t15\nCheckDeviceFormat\t0 1 16 0 3\t0\t\td1"
            + "\nCheckDeviceFormat\t0 1 16 1 3\t0\t\td1"
            + "\nCheckDeviceFormat\t0 1 16 1 1\t0\t\td1"
            + "\nDomain\td2\t0\nCheckDeviceMultiSampleType\t0 1 15 1\t0/1\t\td2";
        d3d9::load_adapter_table(&mut e, &table);
        let texture = |e: &mut Engine, levels: u32, usage: u32, pool: u32| {
            let hr = e
                .vcall(device, 4 * 23, &[64, 64, levels, usage, 0x15, pool, out, 0])
                .u32();
            assert_eq!(hr, D3D_OK);
            e.mem.u32(out)
        };
        let system = texture(&mut e, 0, 0, 2);
        let default = texture(&mut e, 1, 0, 0);
        let managed = texture(&mut e, 0, 0, 1);
        assert_eq!(e.vcall(device, 4 * 31, &[system, default]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 31, &[default, default]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 4 * 31, &[system, managed]).u32(),
            D3DERR_INVALIDCALL
        );
        let level = |e: &mut Engine, t: u32| {
            assert_eq!(e.vcall(t, 4 * 18, &[0, out]).u32(), D3D_OK);
            let s = e.mem.u32(out);
            e.vcall(s, 8, &[]);
            s
        };
        let target = |e: &mut Engine| {
            assert_eq!(
                e.vcall(device, 4 * 28, &[64, 64, 0x15, 0, 0, 0, out, 0])
                    .u32(),
                D3D_OK
            );
            e.mem.u32(out)
        };
        let (a, b) = (target(&mut e), target(&mut e));
        let render_texture = texture(&mut e, 1, 1, 0);
        let rt_level = level(&mut e, render_texture);
        let default_level = level(&mut e, default);
        let system_level = level(&mut e, system);
        let stretch = |e: &mut Engine, from: u32, to: u32| {
            e.vcall(device, 4 * 34, &[from, 0, to, 0, 2]).u32()
        };
        assert_eq!(stretch(&mut e, a, b), D3D_OK);
        assert_eq!(stretch(&mut e, a, rt_level), D3D_OK);
        assert_eq!(stretch(&mut e, default_level, a), D3D_OK);
        assert_eq!(stretch(&mut e, a, default_level), D3DERR_INVALIDCALL);
        assert_eq!(stretch(&mut e, system_level, a), D3DERR_INVALIDCALL);
        assert_eq!(stretch(&mut e, a, a), D3DERR_INVALIDCALL);
        assert_eq!(stretch(&mut e, a, 0), D3DERR_INVALIDCALL);
    }

    fn device_state(e: &Engine, device: u32) -> &Device {
        &e.kernel.graphics.device_objects.devices[&device]
    }

    /// The test table with what the offscreen, fill and query calls ask: `CheckDeviceFormat` of
    /// an X8R8G8B8 plain surface (usage 0, `D3DRTYPE_SURFACE`) and the query types this machine's
    /// device took (`d3d9-capture device queries`, 2026-10-27: event, occlusion and a few more).
    fn extra_device() -> (Engine, u32) {
        let mut e = Engine::new();
        let table = test_table()
            + "\nDomain\td1\t15,16\nCheckDeviceFormat\t0 1 16 0 1\t0\t\td1"
            + "\nCreateQuery\t1\t8876086a\nCreateQuery\t8\t0\nCreateQuery\t9\t0\nCreateQuery\ta\t0";
        d3d9::load_adapter_table(&mut e, &table);
        let create = d3d9::proc_address(&mut e, b"Direct3DCreate9").unwrap();
        let d3d = e.call(create, &[d3d9::D3D_SDK_VERSION]).u32();
        let present = e.mem.alloc(PRESENT_SIZE);
        e.mem.set_u32(present + PP_WIDTH, 640);
        e.mem.set_u32(present + PP_HEIGHT, 480);
        e.mem.set_u32(present + PP_WINDOW, 0x11c);
        e.mem.set_u32(present + PP_WINDOWED, 1);
        let out = e.mem.alloc(4);
        let hr = e
            .vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out])
            .u32();
        assert_eq!(hr, D3D_OK);
        let device = e.mem.u32(out);
        (e, device)
    }

    /// Not a check: the host heap allocations of the device's per-frame calls as the game makes
    /// them (through its vtable), for the frame-rate work (`cargo test --release -p engine-core
    /// --lib allocations_per -- --ignored --nocapture`). The real device's side is measured in
    /// `d3d9real`.
    #[test]
    #[ignore]
    fn allocations_per_operation_of_the_device() {
        use crate::alloc_count::{per_run, row};
        let (mut e, device) = extra_device();
        let table = test_table()
            + "\nDomain\td1\t15,16\nCheckDeviceFormat\t0 1 16 0 1\t0\t\td1"
            + "\nCreateQuery\t1\t8876086a\nCreateQuery\t8\t0\nCreateQuery\t9\t0\nCreateQuery\ta\t0"
            + "\nDomain\td2\t17\nCheckDeviceFormat\t0 1 16 0 3\t0\t\td2";
        d3d9::load_adapter_table(&mut e, &table);
        let out = e.mem.alloc(4);
        let scratch = e.mem.alloc(0x400);
        e.vcall(device, 4 * 23, &[4, 4, 1, 0, 0x17, 1, out, 0]);
        let texture = e.mem.u32(out);
        e.vcall(device, 4 * 26, &[0x900, 0x208, 0, 0, out, 0]);
        let vb = e.mem.u32(out);
        e.vcall(device, 4 * 27, &[0x300, 0x208, 101, 0, out, 0]);
        let ib = e.mem.u32(out);
        let elements = e.mem.alloc(16);
        e.mem.set_u16(elements + 8, 0xff);
        e.mem.set_u8(elements + 12, 17);
        e.vcall(device, 4 * 86, &[elements, out]);
        let declaration = e.mem.u32(out);
        let function = e.mem.alloc(16);
        for (i, token) in [0xfffe_0300u32, 0x1234, 0x0000_ffff].iter().enumerate() {
            e.mem.set_u32(function + 4 * i as u32, *token);
        }
        e.vcall(device, 4 * 91, &[function, out]);
        let vs = e.mem.u32(out);
        e.vcall(device, 0x1d8, &[8, out]);
        let query = e.mem.u32(out);
        let viewport = e.mem.alloc(24);
        for (i, w) in [0u32, 0, 640, 480, 0, 1f32.to_bits()].iter().enumerate() {
            e.mem.set_u32(viewport + 4 * i as u32, *w);
        }
        let matrix = e.mem.alloc(64);
        let constants = e.mem.alloc(16 * 64);
        let rect = e.mem.alloc(16);
        e.mem.set_u32(rect + 8, 64);
        e.mem.set_u32(rect + 12, 64);
        let iid = e.mem.alloc(16);
        e.mem.write(iid, &IID_DEVICE9);
        e.vcall(device, 4 * 41, &[]);
        let mut calls: Vec<(&str, u32, Vec<u32>)> = vec![
            ("SetRenderState", device, vec![4 * 57, 7, 1]),
            ("SetTextureStageState", device, vec![4 * 67, 1, 1, 4]),
            ("SetSamplerState", device, vec![4 * 69, 0, 5, 2]),
            ("SetTransform", device, vec![4 * 44, 256, matrix]),
            ("SetViewport", device, vec![4 * 47, viewport]),
            ("SetScissorRect", device, vec![4 * 75, rect]),
            (
                "SetTexture (bind, unbind)",
                device,
                vec![4 * 65, 0, texture],
            ),
            ("SetStreamSource", device, vec![4 * 100, 0, vb, 0, 32]),
            ("SetIndices", device, vec![4 * 104, ib]),
            ("SetVertexDeclaration", device, vec![4 * 87, declaration]),
            ("SetVertexShader", device, vec![4 * 92, vs]),
            (
                "SetVertexShaderConstantF, 16",
                device,
                vec![4 * 94, 0, constants, 16],
            ),
            (
                "SetPixelShaderConstantF, 4",
                device,
                vec![4 * 109, 0, constants, 4],
            ),
            (
                "DrawIndexedPrimitive",
                device,
                vec![4 * 82, 4, 0, 0, 4, 0, 2],
            ),
            ("DrawPrimitive", device, vec![4 * 81, 4, 0, 2]),
            ("Clear", device, vec![4 * 43, 0, 0, 1, 0, 0, 0]),
            ("QueryInterface + Release", device, vec![0, iid, out]),
            (
                "vertex buffer Lock + Unlock",
                vb,
                vec![4 * 11, 0, 0x40, out, 0x2000],
            ),
            ("query Issue(END) + GetData", query, vec![0x18, 1]),
        ];
        for (what, object, words) in calls.iter_mut() {
            let (slot, args) = (words[0], words[1..].to_vec());
            let object = *object;
            let what = *what;
            row(
                &format!("IDirect3DDevice9 {what}"),
                per_run(1000, || {
                    e.vcall(object, slot, &args);
                    match what {
                        "SetTexture (bind, unbind)" => {
                            e.vcall(device, 4 * 65, &[0, 0]);
                        }
                        "QueryInterface + Release" => {
                            e.vcall(device, 8, &[]);
                        }
                        "vertex buffer Lock + Unlock" => {
                            e.vcall(vb, 4 * 12, &[]);
                        }
                        "query Issue(END) + GetData" => {
                            e.vcall(query, 0x1c, &[scratch, 4, 1]);
                        }
                        _ => {}
                    }
                }),
            );
        }
        row(
            "IDirect3DDevice9 EndScene + Present + BeginScene",
            per_run(1000, || {
                e.vcall(device, 4 * 42, &[]);
                e.vcall(device, 4 * 17, &[0, 0, 0, 0]);
                e.vcall(device, 4 * 41, &[]);
            }),
        );
    }

    /// `CreateOffscreenPlainSurface` (+0x90) and `ColorFill` (+0x8C) as the retail runtime
    /// answered (`d3d9-capture device extra`, 2026-10-27).
    #[test]
    fn offscreen_surfaces_and_fills_answer_as_measured() {
        let (mut e, device) = extra_device();
        let out = e.mem.alloc(4);
        let make = |e: &mut Engine, w: u32, pool: u32| {
            let hr = e.vcall(device, 0x90, &[w, 64, 0x16, pool, out, 0]).u32();
            (hr, e.mem.u32(out))
        };
        assert_eq!(make(&mut e, 64, 1).0, D3DERR_INVALIDCALL, "managed");
        assert_eq!(make(&mut e, 0, 2).0, D3DERR_INVALIDCALL, "a zero side");
        let (hr, system) = make(&mut e, 64, 2);
        assert_eq!(hr, D3D_OK);
        let (hr, default) = make(&mut e, 64, 0);
        assert_eq!(hr, D3D_OK);
        // Outside the default pool a fill is refused; X8R8G8B8 takes the colour with byte 3 zero.
        assert_eq!(
            e.vcall(device, 0x8c, &[system, 0, 0xff11_2233]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 0x8c, &[default, 0, 0xff11_2233]).u32(),
            D3D_OK
        );
        let locked = e.mem.alloc(8);
        assert_eq!(e.vcall(default, 4 * 13, &[locked, 0, 0]).u32(), D3D_OK);
        let bits = e.mem.u32(locked + 4);
        assert_eq!(e.mem.u32(bits), 0x0011_2233);
        assert_eq!(e.vcall(default, 4 * 14, &[]).u32(), D3D_OK);
        // A rectangle inside fills; past the edge is refused.
        let rect = e.mem.alloc(16);
        for (i, v) in [8u32, 8, 16, 16].iter().enumerate() {
            e.mem.set_u32(rect + 4 * i as u32, *v);
        }
        assert_eq!(
            e.vcall(device, 0x8c, &[default, rect, 0x0044_5566]).u32(),
            D3D_OK
        );
        e.mem.set_u32(rect + 8, 100);
        assert_eq!(
            e.vcall(device, 0x8c, &[default, rect, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        // GetRenderTargetData (+0x80): a system-memory destination of the back buffer's size and
        // format is taken (the pixels are the real device's); another size or pool is refused.
        let back = device_state(&e, device).render_targets[0];
        let (_, small) = make(&mut e, 32, 2);
        assert_eq!(
            e.vcall(device, 0x80, &[back, small]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 0x80, &[back, default]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 0x80, &[system, system]).u32(),
            D3DERR_INVALIDCALL
        );
    }

    /// `CreateQuery` (+0x1D8) and the event query's life as the retail runtime answered
    /// (2026-10-27); without `NV_D3D9=real` an ended event query is done at once.
    #[test]
    fn queries_answer_as_measured() {
        let (mut e, device) = extra_device();
        assert_eq!(e.vcall(device, 0x1d8, &[1, 0]).u32(), 0x8876_086a);
        assert_eq!(e.vcall(device, 0x1d8, &[8, 0]).u32(), D3D_OK);
        let out = e.mem.alloc(4);
        assert_eq!(e.vcall(device, 0x1d8, &[8, out]).u32(), D3D_OK);
        let q = e.mem.u32(out);
        assert_eq!(e.vcall(q, 0x10, &[]).u32(), 8, "GetType");
        assert_eq!(e.vcall(q, 0x14, &[]).u32(), 4, "GetDataSize");
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0xaaaa_aaaa);
        // Never issued: S_OK and dddddddd.
        assert_eq!(e.vcall(q, 0x1c, &[data, 4, 1]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(data), 0xdddd_dddd);
        assert_eq!(e.vcall(q, 0x18, &[2]).u32(), D3D_OK, "BEGIN");
        e.mem.set_u32(data, 0xaaaa_aaaa);
        assert_eq!(
            e.vcall(q, 0x1c, &[data, 4, 1]).u32(),
            1,
            "S_FALSE after BEGIN"
        );
        assert_eq!(e.mem.u32(data), 0xaaaa_aaaa);
        assert_eq!(e.vcall(q, 0x18, &[1]).u32(), D3D_OK, "END");
        assert_eq!(e.vcall(q, 0x1c, &[data, 4, 1]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(data), 1);
        // Size 2 writes two bytes; size 8 zero-extends.
        e.mem.set_u32(data, 0xaaaa_aaaa);
        assert_eq!(e.vcall(q, 0x1c, &[data, 2, 0]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(data), 0xaaaa_0001);
        e.mem.set_u32(data + 4, 0xaaaa_aaaa);
        assert_eq!(e.vcall(q, 0x1c, &[data, 8, 0]).u32(), D3D_OK);
        assert_eq!([e.mem.u32(data), e.mem.u32(data + 4)], [1, 0]);
        assert_eq!(e.vcall(q, 0x1c, &[0, 0, 0]).u32(), D3D_OK);
        assert_eq!(e.vcall(q, 8, &[]).u32(), 0, "Release");
    }

    /// `Reset` (+0x40) as the retail runtime answered (`d3d9-capture device reset`, 2026-10-27).
    #[test]
    fn reset_answers_as_measured() {
        let (mut e, device) = extra_device();
        let pp = e.mem.alloc(PRESENT_SIZE);
        let fill = |e: &mut Engine, count: u32, width: u32| {
            for i in 0..PRESENT_SIZE / 4 {
                e.mem.set_u32(pp + 4 * i, 0);
            }
            e.mem.set_u32(pp + PP_WIDTH, width);
            e.mem.set_u32(pp + PP_HEIGHT, 480);
            e.mem.set_u32(pp + PP_COUNT, count);
            e.mem.set_u32(pp + PP_WINDOW, 0x11c);
            e.mem.set_u32(pp + PP_WINDOWED, 1);
        };
        // States set before Reset.
        assert_eq!(e.vcall(device, 0xe4, &[7, 0]).u32(), D3D_OK);
        // A default-pool surface alive: refused, and the device needs a Reset.
        let out = e.mem.alloc(4);
        assert_eq!(
            e.vcall(device, 0x90, &[64, 64, 0x16, 0, out, 0]).u32(),
            D3D_OK
        );
        let surface = e.mem.u32(out);
        fill(&mut e, 0, 640);
        assert_eq!(e.vcall(device, 0x40, &[pp]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.vcall(device, 0xc, &[]).u32(), D3DERR_DEVICENOTRESET);
        assert_eq!(
            e.vcall(device, 0xa4, &[]).u32(),
            D3DERR_INVALIDCALL,
            "BeginScene"
        );
        assert_eq!(
            e.vcall(device, 0x44, &[0, 0, 0, 0]).u32(),
            D3DERR_INVALIDCALL,
            "Present"
        );
        assert_eq!(e.vcall(surface, 8, &[]).u32(), 0);
        assert_eq!(
            e.vcall(device, 0xc, &[]).u32(),
            D3DERR_DEVICENOTRESET,
            "still"
        );
        // Four back buffers: refused, the count left as given.
        fill(&mut e, 4, 640);
        assert_eq!(e.vcall(device, 0x40, &[pp]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(e.mem.u32(pp + PP_COUNT), 4);
        // A good Reset: defaults filled in, the device usable again, its states cleared, the
        // render target the new back buffer of the new size.
        fill(&mut e, 0, 800);
        assert_eq!(e.vcall(device, 0x40, &[pp]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(pp + PP_COUNT), 1);
        assert_eq!(e.mem.u32(pp + PP_FORMAT), 0x16);
        assert_eq!(e.vcall(device, 0xc, &[]).u32(), D3D_OK);
        let d = device_state(&e, device);
        assert!(d.render_states.is_empty());
        assert_eq!(d.viewport[2..4], [800, 480]);
        let back = d.back_buffers[0];
        assert_eq!(d.render_targets[0], back);
        assert_eq!(
            e.kernel.graphics.device_objects.surfaces[&back].desc[6..8],
            [800, 480]
        );
        // In a scene too; the scene ends.
        assert_eq!(e.vcall(device, 0xa4, &[]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 0x40, &[pp]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 0xa8, &[]).u32(),
            D3DERR_INVALIDCALL,
            "EndScene"
        );
    }

    /// A device with what a frame of the game binds (TEST DATA, not a capture): `n` textures,
    /// vertex and index buffers, vertex and pixel shaders and declarations (each a power of
    /// two), and 4 KB of shader constants (distinct floats).
    #[cfg(windows)]
    struct Scene {
        e: Engine,
        device: u32,
        textures: Vec<u32>,
        vertex_buffers: Vec<u32>,
        index_buffers: Vec<u32>,
        declarations: Vec<u32>,
        vertex_shaders: Vec<u32>,
        pixel_shaders: Vec<u32>,
        constants: u32,
    }

    #[cfg(windows)]
    fn scene(n: [u32; 4]) -> Scene {
        let (mut e, d3d, present) = d3d();
        let table = test_table() + "\nDomain\td1\t17\nCheckDeviceFormat\t0 1 16 0 3\t0\t\td1";
        d3d9::load_adapter_table(&mut e, &table);
        let out = e.mem.alloc(4);
        e.vcall(d3d, CREATE_DEVICE, &[0, 1, 0x11c, 0x40, present, out]);
        let device = e.mem.u32(out);
        let make = |e: &mut Engine, n: u32, slot: u32, args: &[u32]| -> Vec<u32> {
            (0..n)
                .map(|_| {
                    let mut words = args.to_vec();
                    words.push(out);
                    words.push(0);
                    assert_eq!(e.vcall(device, 4 * slot, &words).u32(), D3D_OK);
                    e.mem.u32(out)
                })
                .collect()
        };
        let textures = make(&mut e, n[0], 23, &[4, 4, 1, 0, 0x17, 1]);
        let vertex_buffers = make(&mut e, n[1], 26, &[64, 0, 0x142, 1]);
        let index_buffers = make(&mut e, n[1], 27, &[12, 0, 101, 1]);
        let function = |e: &mut Engine, version: u32| {
            let at = e.mem.alloc(8);
            e.mem.set_u32(at, version);
            e.mem.set_u32(at + 4, 0x0000_ffff);
            at
        };
        let shaders = |e: &mut Engine, slot: u32, version: u32| -> Vec<u32> {
            (0..n[2])
                .map(|_| {
                    let f = function(e, version);
                    assert_eq!(e.vcall(device, 4 * slot, &[f, out]).u32(), D3D_OK);
                    e.mem.u32(out)
                })
                .collect()
        };
        let vertex_shaders = shaders(&mut e, 91, 0xfffe_0300);
        let pixel_shaders = shaders(&mut e, 106, 0xffff_0300);
        let elements = e.mem.alloc(16);
        e.mem.set_u16(elements + 8, 0xff);
        e.mem.set_u8(elements + 12, 17);
        let declarations = (0..n[3])
            .map(|_| {
                assert_eq!(e.vcall(device, 4 * 86, &[elements, out]).u32(), D3D_OK);
                e.mem.u32(out)
            })
            .collect();
        let constants = e.mem.alloc(0x1100);
        for i in 0..0x440 {
            e.mem
                .set_u32(constants + 4 * i, (i as f32 * 1.5 - 7.25).to_bits());
        }
        Scene {
            e,
            device,
            textures,
            vertex_buffers,
            index_buffers,
            declarations,
            vertex_shaders,
            pixel_shaders,
            constants,
        }
    }

    /// Gives every object of the scene a stand-in real counterpart (`d3d9real::stub`): the
    /// device `#0`, textures `#1000..`, vertex buffers `#2000..`, index buffers `#3000..`,
    /// declarations `#4000..`, vertex shaders `#5000..`, pixel shaders `#6000..`.
    #[cfg(windows)]
    fn stand_in(s: &mut Scene) {
        use super::super::d3d9real::stub::{install, Kind};
        install(&mut s.e, s.device, Kind::Device, 0);
        for (kind, objects, base) in [
            (Kind::Texture, &s.textures, 1000),
            (Kind::VertexBuffer, &s.vertex_buffers, 2000),
            (Kind::IndexBuffer, &s.index_buffers, 3000),
            (Kind::Declaration, &s.declarations, 4000),
            (Kind::VertexShader, &s.vertex_shaders, 5000),
            (Kind::PixelShader, &s.pixel_shaders, 6000),
        ] {
            for (i, &o) in objects.iter().enumerate() {
                install(&mut s.e, o, kind, base + i as u32);
            }
        }
    }

    /// The groups of calls a [`draw`] makes, with how many calls each makes.
    #[cfg(windows)]
    const GROUPS: [(&str, u32); 9] = [
        ("SetVertexDeclaration", 1),
        ("SetVertexShader/SetPixelShader", 2),
        ("SetVertexShaderConstantF", 8),
        ("SetPixelShaderConstantF", 4),
        ("SetTexture", 2),
        ("SetSamplerState", 4),
        ("SetRenderState", 4),
        ("SetStreamSource/SetIndices", 2),
        ("DrawIndexedPrimitive", 1),
    ];

    /// One draw as the game's renderer makes them (TEST DATA, the shape of a lit, textured
    /// mesh's pass): declaration, shaders, eight vertex and four pixel float constant ranges,
    /// two textures, four sampler and four render states, stream 0, indices and the indexed
    /// draw: 28 calls, the [`GROUPS`] in order. `device` is the object called (the scene's
    /// device, or another object with the same slots). The objects are picked spread over the
    /// scene's (as a frame's meshes are).
    #[cfg(windows)]
    fn draw(s: &mut Scene, device: u32, i: usize) {
        for group in 0..GROUPS.len() {
            draw_group(s, device, i, group);
        }
    }

    #[cfg(windows)]
    fn draw_group(s: &mut Scene, device: u32, i: usize, group: usize) {
        let e = &mut s.e;
        let data = |k: usize| s.constants + 16 * (k % 64) as u32;
        let pick = |objects: &[u32], k: usize| objects[k.wrapping_mul(769) % objects.len()];
        let i32 = i as u32;
        match group {
            0 => {
                e.vcall(device, 4 * 87, &[pick(&s.declarations, i)]);
            }
            1 => {
                e.vcall(device, 4 * 92, &[pick(&s.vertex_shaders, i)]);
                e.vcall(device, 4 * 107, &[pick(&s.pixel_shaders, i / 2)]);
            }
            2 => {
                for (k, (start, count)) in [
                    (0, 4),
                    (4, 4),
                    (8, 3),
                    (12, 1),
                    (13, 1),
                    (20, 2),
                    (30, 1),
                    (40, 1),
                ]
                .into_iter()
                .enumerate()
                {
                    e.vcall(device, 4 * 94, &[start, data(3 * i + k), count]);
                }
            }
            3 => {
                for (k, (start, count)) in [(0, 1), (1, 1), (2, 2), (5, 1)].into_iter().enumerate()
                {
                    e.vcall(device, 4 * 109, &[start, data(5 * i + k), count]);
                }
            }
            4 => {
                e.vcall(device, 4 * 65, &[0, pick(&s.textures, i)]);
                e.vcall(device, 4 * 65, &[1, pick(&s.textures, 7 * i + 3)]);
            }
            5 => {
                e.vcall(device, 4 * 69, &[0, 1, 1 + (i32 & 1)]);
                e.vcall(device, 4 * 69, &[0, 2, 1]);
                e.vcall(device, 4 * 69, &[1, 5, 2]);
                e.vcall(device, 4 * 69, &[1, 6, 2]);
            }
            6 => {
                e.vcall(device, 4 * 57, &[7, 1]);
                e.vcall(device, 4 * 57, &[27, i32 & 1]);
                e.vcall(device, 4 * 57, &[22, 2 + (i32 & 1)]);
                e.vcall(device, 4 * 57, &[14, (i32 >> 1) & 1]);
            }
            7 => {
                e.vcall(device, 4 * 100, &[0, pick(&s.vertex_buffers, i), 0, 32]);
                e.vcall(device, 4 * 104, &[pick(&s.index_buffers, i)]);
            }
            _ => {
                e.vcall(device, 4 * 82, &[4, 0, 0, 100, 0, 50]);
            }
        }
    }

    /// The calls of one draw that reach the real device, as forwarded to a stand-in (the
    /// first draw of [`draw`]: index 0).
    #[cfg(windows)]
    const FIRST_DRAW: [&str; 28] = [
        "SetVertexDeclaration(#4000)",
        "SetVertexShader(#5000)",
        "SetPixelShader(#6000)",
        "vertex_constant_f(0, [c0e80000 c0b80000 c0880000 c0300000 bfa00000 3e800000 \
         3fe00000 40500000 40980000 40c80000 40f80000 41140000 412c0000 41440000 415c0000 \
         41740000], 4)",
        "vertex_constant_f(4, [bfa00000 3e800000 3fe00000 40500000 40980000 40c80000 \
         40f80000 41140000 412c0000 41440000 415c0000 41740000 41860000 41920000 419e0000 \
         41aa0000], 4)",
        "vertex_constant_f(8, [40980000 40c80000 40f80000 41140000 412c0000 41440000 \
         415c0000 41740000 41860000 41920000 419e0000 41aa0000], 3)",
        "vertex_constant_f(12, [412c0000 41440000 415c0000 41740000], 1)",
        "vertex_constant_f(13, [41860000 41920000 419e0000 41aa0000], 1)",
        "vertex_constant_f(20, [41b60000 41c20000 41ce0000 41da0000 41e60000 41f20000 \
         41fe0000 42050000], 2)",
        "vertex_constant_f(30, [41e60000 41f20000 41fe0000 42050000], 1)",
        "vertex_constant_f(40, [420b0000 42110000 42170000 421d0000], 1)",
        "pixel_constant_f(0, [c0e80000 c0b80000 c0880000 c0300000], 1)",
        "pixel_constant_f(1, [bfa00000 3e800000 3fe00000 40500000], 1)",
        "pixel_constant_f(2, [40980000 40c80000 40f80000 41140000 412c0000 41440000 \
         415c0000 41740000], 2)",
        "pixel_constant_f(5, [412c0000 41440000 415c0000 41740000], 1)",
        "SetTexture(0, #1000)",
        "SetTexture(1, #1003)",
        "SetSamplerState(0, 1, 0x1)",
        "SetSamplerState(0, 2, 0x1)",
        "SetSamplerState(1, 5, 0x2)",
        "SetSamplerState(1, 6, 0x2)",
        "SetRenderState(7, 0x1)",
        "SetRenderState(27, 0x0)",
        "SetRenderState(22, 0x2)",
        "SetRenderState(14, 0x0)",
        "SetStreamSource(0, #2000, 0, 32)",
        "SetIndices(#3000)",
        "DrawIndexedPrimitive(4, 0, 0, 100, 0, 50)",
    ];

    /// What reaches the real device is the game's calls as given, in the game's order, with
    /// the data behind their pointers as engine memory holds it; a call the model refuses does
    /// not reach it. Pins the forwarding against a stand-in device (`d3d9real::stub`).
    #[cfg(windows)]
    #[test]
    fn forwarded_calls_reach_the_real_device_as_given() {
        use super::super::d3d9real::stub;
        let mut s = scene([64, 16, 8, 8]);
        stand_in(&mut s);
        let device = s.device;
        stub::record();
        assert_eq!(s.e.vcall(device, 4 * 41, &[]).u32(), D3D_OK);
        draw(&mut s, device, 0);
        let log = stub::take();
        assert_eq!(log[0], "BeginScene");
        let first: Vec<String> = log[1..]
            .iter()
            .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        let expected: Vec<String> = FIRST_DRAW
            .iter()
            .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        assert_eq!(first, expected);
        // Calls the model refuses or takes at their edges, then many draws.
        stub::record();
        let e = &mut s.e;
        let data = s.constants;
        let (t0, t1) = (s.textures[0], s.textures[1]);
        assert_eq!(
            e.vcall(device, 4 * 69, &[16, 1, 1]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 65, &[16, t0]).u32(), D3DERR_INVALIDCALL);
        assert_eq!(
            e.vcall(device, 4 * 94, &[255, data, 2]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 4 * 109, &[223, data, 2]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(
            e.vcall(device, 4 * 94, &[3, 0, 1]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 94, &[3, data, 0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 94, &[0, data, 256]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 109, &[0, data + 4, 224]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 96, &[2, data + 8, 3]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 98, &[1, data + 12, 5]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 111, &[15, data, 1]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 113, &[0, data, 16]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 113, &[1, data, 16]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 65, &[2, t1]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 65, &[2, t1]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 65, &[2, 0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 65, &[257, t0]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 67, &[1, 1, 4]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 67, &[8, 1, 4]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 100, &[1, 0, 0, 0]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 100, &[16, 0, 0, 0]).u32(),
            D3DERR_INVALIDCALL
        );
        assert_eq!(e.vcall(device, 4 * 92, &[0]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 92, &[s.vertex_shaders[3]]).u32(),
            D3D_OK
        );
        // Transforms, the viewport, a material and a light, a clear and a draw from memory.
        assert_eq!(e.vcall(device, 4 * 44, &[256, data]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 44, &[3, data + 68]).u32(), D3D_OK);
        assert_eq!(
            e.vcall(device, 4 * 44, &[4, data]).u32(),
            D3DERR_INVALIDCALL
        );
        let viewport = e.mem.alloc(24);
        for (i, w) in [8, 4, 600, 400, 0.25f32.to_bits(), 1f32.to_bits()]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(viewport + 4 * i as u32, w);
        }
        assert_eq!(e.vcall(device, 4 * 47, &[viewport]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 49, &[data + 20]).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 51, &[1, data + 36]).u32(), D3D_OK);
        let rect = e.mem.alloc(16);
        e.mem.set_u32(rect + 8, 10);
        e.mem.set_u32(rect + 12, 12);
        let clear = [1, rect, 1, 0xff10_2030, 1f32.to_bits(), 0];
        assert_eq!(e.vcall(device, 4 * 43, &clear).u32(), D3D_OK);
        assert_eq!(e.vcall(device, 4 * 83, &[4, 2, data + 2, 16]).u32(), D3D_OK);
        for i in 1..200 {
            draw(&mut s, device, i);
        }
        assert_eq!(s.e.vcall(device, 4 * 42, &[]).u32(), D3D_OK);
        let log = stub::take();
        assert_eq!(log.len(), 22 + 199 * 28 + 1);
        assert!(!log.iter().any(|l| l.contains("unexpected")));
        let digest = log.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, line| {
            line.bytes()
                .chain(*b"\n")
                .fold(h, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3))
        });
        if std::env::var_os("NV_D3D9_STUB_LOG").is_some() {
            for line in &log {
                eprintln!("{line}");
            }
        }
        assert_eq!(
            digest, 5_257_225_520_366_858_290,
            "the forwarded calls' digest"
        );
        assert_eq!(super::super::d3d9real::stub::logged(&s.e), 0);
        // What the model answers the game afterwards: its getters' answers and words, and the
        // objects' counts.
        let answers = model_answers(&mut s);
        if std::env::var_os("NV_D3D9_STUB_LOG").is_some() {
            eprintln!("{answers:08x?}");
        }
        let digest = answers.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &w| {
            (h ^ u64::from(w)).wrapping_mul(0x0100_0000_01b3)
        });
        assert_eq!(answers.len(), 2303);
        assert_eq!(
            digest, 9_009_626_195_373_671_558,
            "the model's answers' digest"
        );
    }

    /// The scene's device state as the game can read it back: each getter's answer and the
    /// words it writes, and each object's reference count (`AddRef` then `Release`).
    #[cfg(windows)]
    fn model_answers(s: &mut Scene) -> Vec<u32> {
        let device = s.device;
        let e = &mut s.e;
        let out = e.mem.alloc(0x1000);
        let mut answers = Vec::new();
        let mut get = |e: &mut Engine, slot: u32, args: &[u32], words: u32| {
            e.mem.zero(out, 4 * words);
            answers.push(e.vcall(device, 4 * slot, args).u32());
            answers.extend((0..words).map(|i| e.mem.u32(out + 4 * i)));
        };
        for state in [7, 14, 22, 27] {
            get(e, 58, &[state, out], 1);
        }
        for (sampler, kind) in [(0, 1), (0, 2), (1, 5), (1, 6)] {
            get(e, 68, &[sampler, kind, out], 1);
        }
        get(e, 66, &[1, 1, out], 1);
        for sampler in [0, 1, 2, 257] {
            get(e, 64, &[sampler, out], 1);
        }
        for stream in [0, 1] {
            get(e, 101, &[stream, out, out + 4, out + 8], 3);
        }
        for slot in [105, 88, 93, 108] {
            get(e, slot, &[out], 1);
        }
        get(e, 95, &[0, out, 256], 1024);
        get(e, 110, &[0, out, 224], 896);
        get(e, 97, &[2, out, 3], 12);
        get(e, 99, &[1, out, 5], 5);
        get(e, 112, &[15, out, 1], 4);
        get(e, 114, &[0, out, 16], 16);
        for state in [256, 3, 2] {
            get(e, 45, &[state, out], 16);
        }
        get(e, 48, &[out], 6);
        let objects = [
            &s.textures,
            &s.vertex_buffers,
            &s.index_buffers,
            &s.declarations,
            &s.vertex_shaders,
            &s.pixel_shaders,
        ];
        for &o in objects.iter().flat_map(|o| o.iter()) {
            answers.push(e.vcall(o, 4, &[]).u32());
            answers.push(e.vcall(o, 8, &[]).u32());
        }
        answers
    }

    /// The cost of the platform's Direct3D 9 calls as the game makes them (through
    /// `Engine::vcall`), per call: the call machinery alone (the same calls to an object whose
    /// slots do nothing), the model alone, and the model forwarding to a stand-in real device
    /// (`d3d9real::stub`); then the forwarded cost by group of calls. The scene has 2048
    /// textures, 1024 vertex and index buffers, 64 shaders of each kind and 32 declarations.
    /// `cargo test --release -p engine-core --lib cost_of_d3d9 -- --ignored --nocapture`.
    #[cfg(windows)]
    #[test]
    #[ignore]
    fn cost_of_d3d9_calls() {
        const SIZE: [u32; 4] = [2048, 1024, 64, 32];
        let draws = 2000;
        let frames = 10;
        // Nanoseconds per call of `group` (all of a draw's for `None`), the best of five rounds
        // after a warm-up frame.
        let run = |s: &mut Scene, device: u32, group: Option<usize>| -> f64 {
            let calls = group.map_or(28, |g| GROUPS[g].1);
            let mut best = f64::MAX;
            for round in 0..6 {
                let t = std::time::Instant::now();
                for _ in 0..if round == 0 { 1 } else { frames } {
                    s.e.vcall(s.device, 4 * 41, &[]);
                    for i in 0..draws {
                        match group {
                            None => draw(s, device, i),
                            Some(_) if round == 0 => draw(s, device, i),
                            Some(g) => draw_group(s, device, i, g),
                        }
                    }
                    s.e.vcall(s.device, 4 * 42, &[]);
                }
                let ns = t.elapsed().as_nanos() as f64 / (frames * draws) as f64 / f64::from(calls);
                if round > 0 {
                    best = best.min(ns);
                }
            }
            best
        };
        // The floor: an object whose every slot is a function that does nothing.
        let mut s = scene(SIZE);
        fn nothing(_: &mut Engine, _: &[u32]) -> crate::Ret {
            crate::Ret::default()
        }
        let empty = 0x7e0f_0000;
        s.e.register(empty, nothing);
        let table = s.e.mem.alloc(4 * 128);
        for i in 0..128 {
            s.e.mem.set_u32(table + 4 * i, empty);
        }
        let object = s.e.mem.alloc(8);
        s.e.mem.set_u32(object, table);
        let floor = run(&mut s, object, None);
        let device = s.device;
        let model = run(&mut s, device, None);
        let model_groups: Vec<f64> = (0..GROUPS.len())
            .map(|g| run(&mut s, device, Some(g)))
            .collect();
        let mut s = scene(SIZE);
        stand_in(&mut s);
        let device = s.device;
        let forwarded = run(&mut s, device, None);
        eprintln!(
            "cost of a Direct3D 9 call: machinery {floor:.1} ns, model {model:.1} ns, \
             model + forwarding to a stand-in device {forwarded:.1} ns \
             ({draws} draws x 28 calls a frame: {:.2} ms)",
            forwarded * f64::from(draws as u32 * 28) / 1e6
        );
        for (g, (name, _)) in GROUPS.iter().enumerate() {
            let ns = run(&mut s, device, Some(g));
            eprintln!(
                "  {name}: model {:.1} ns, forwarded {ns:.1} ns a call",
                model_groups[g]
            );
        }
        // A skinned mesh's bone palette: 90 registers in one call.
        let calls = 200_000u32;
        let mut best = f64::MAX;
        for _ in 0..5 {
            let t = std::time::Instant::now();
            for i in 0..calls {
                let data = s.constants + 16 * (i % 64);
                s.e.vcall(device, 4 * 94, &[30, data, 90]);
            }
            best = best.min(t.elapsed().as_nanos() as f64 / f64::from(calls));
        }
        eprintln!("  SetVertexShaderConstantF of 90 registers: forwarded {best:.1} ns a call");
    }
}
