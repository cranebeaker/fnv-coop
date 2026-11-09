//! The real `d3d9.dll` as a backend of the Direct3D 9 call stream (`d3d9_stream`, ADR-0008):
//! each recorded call carried out on the real device as the forwarding did it on the game's
//! thread (`super::d3d9real`), on whichever thread the stream runs it (the calling one, or the
//! render thread with `NV_RENDER_THREAD=1`).
//!
//! The backend never sees the game's memory or the model: what it needs is in the command. It
//! keeps the real objects by the modelled object's address, prints once each call where the real
//! runtime answers otherwise than the model, skips the draws of a device while a binding has no
//! real counterpart, and keeps destroyed objects until the next `Present` (the game's device is a
//! pure one, which keeps no reference of its own to what is bound: an object released at once
//! would be freed while still bound, and the call that replaces the binding would read it, p169).
//!
//! A texture level the backend keeps (a managed or system-memory level: `d3d9device`'s
//! `Pixels::Real`) answers `ReadSurface` with what it was given; when its real surface is missing
//! or cannot be locked, its bytes stay here instead, so the game reads back what it wrote, as when
//! the model kept them.

use super::d3d9real::{face_check, shot};
use d3d9_stream::{Backend, Cmd, Data, Native, Reply, Rows};
use std::collections::HashSet;
use std::mem::ManuallyDrop;
use windows::Win32::Foundation::{BOOL, HWND, RECT};
use windows::Win32::Graphics::Direct3D9::*;

/// A real Direct3D 9 object, the counterpart of a modelled one.
enum Object {
    Device(IDirect3DDevice9),
    Surface(IDirect3DSurface9),
    Texture(IDirect3DTexture9),
    Cube(IDirect3DCubeTexture9),
    VertexBuffer(IDirect3DVertexBuffer9),
    IndexBuffer(IDirect3DIndexBuffer9),
    VertexShader(IDirect3DVertexShader9),
    PixelShader(IDirect3DPixelShader9),
    Declaration(IDirect3DVertexDeclaration9),
    Query(IDirect3DQuery9),
}

/// The real device handed to the layer's side channel (D3DX's texture loading on the game's
/// threads, `super::d3dxreal::texture_levels`): the device is made with
/// `D3DCREATE_MULTITHREADED` (the game's flags 0x54), so d3d9.dll serialises calls from any
/// thread.
pub struct SharedDevice(pub IDirect3DDevice9);

// SAFETY: a `D3DCREATE_MULTITHREADED` device takes calls from any thread (d3d9.dll locks itself).
unsafe impl Send for SharedDevice {}

/// A real query the game's threads read directly ([`QueryBoard`]).
pub struct SharedQuery(pub IDirect3DQuery9);

// SAFETY: a query of a `D3DCREATE_MULTITHREADED` device takes calls from any thread.
unsafe impl Send for SharedQuery {}

/// One query on the [`QueryBoard`].
pub struct BoardQuery {
    /// Which of the queries made at this address it is: the backend's count of its `CreateQuery`s
    /// there (a query destroyed and made again at the same address is another).
    pub made: u64,
    /// The real query; `None`: its creation failed.
    pub query: Option<SharedQuery>,
    /// The `D3DISSUE_END`s the backend has issued on it.
    pub ends: u64,
}

/// With the render thread: the real queries, which the game's threads read without waiting for
/// the render thread to catch up (`d3d9real::query_get_data`), by the modelled query. A query the
/// backend has not made yet, or has destroyed, is not in it.
#[derive(Default)]
pub struct QueryBoard(pub(crate) std::sync::Mutex<engine_core::AddrMap<u32, BoardQuery>>);

impl QueryBoard {
    pub(crate) fn lock(&self) -> std::sync::MutexGuard<'_, engine_core::AddrMap<u32, BoardQuery>> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// The real side of the game's Direct3D 9 calls.
#[derive(Default)]
pub struct RealBackend {
    d3d: Option<IDirect3D9>,
    objects: engine_core::AddrMap<u32, Object>,
    /// The device last made, not counted (its entry in `objects` holds the reference): every
    /// command looks its device up, and the game has one.
    device: Option<(u32, ManuallyDrop<IDirect3DDevice9>)>,
    /// The buffers made with `D3DUSAGE_DYNAMIC` (their locks pass `DISCARD`/`NOOVERWRITE` on).
    dynamic: engine_core::AddrSet<u32>,
    /// Bindings without a real counterpart, by (device, binding: a stream number, or
    /// [`INDICES`], [`DECLARATION`], [`VERTEX_SHADER`], [`PIXEL_SHADER`]): the real binding is
    /// null instead, and the device's draws are skipped while any is left.
    broken: engine_core::AddrSet<(u32, u32)>,
    /// Destroyed objects, released at the next `Present`.
    graveyard: Vec<Object>,
    /// Levels whose bytes the real surface could not take (missing, or its lock failed): the
    /// bytes last uploaded, for `ReadSurface`.
    kept: engine_core::AddrMap<u32, Vec<u8>>,
    /// The differences already printed: (call, model's answer, real answer).
    pub(crate) logged: HashSet<(&'static str, u32, u32)>,
    /// With the render thread: the queries the game's threads read ([`QueryBoard`]), and how
    /// many queries it has made at each address.
    board: Option<std::sync::Arc<QueryBoard>>,
    queries_made: engine_core::AddrMap<u32, u64>,
}

// SAFETY: the real objects are free-threaded COM objects of a `D3DCREATE_MULTITHREADED` device;
// the backend itself is used by one thread at a time (the stream's).
unsafe impl Send for RealBackend {}

/// The bindings [`RealBackend::broken`] names besides the streams (0..16).
const INDICES: u32 = 0x100;
const DECLARATION: u32 = 0x101;
const VERTEX_SHADER: u32 = 0x102;
const PIXEL_SHADER: u32 = 0x103;

/// `D3DCREATE_FPU_PRESERVE`, added to the game's flags: the game's x87 precision switch is
/// modelled (`crate::direct3d_device_created`), and the thread that runs the backend keeps its
/// own floating-point state.
const FPU_PRESERVE: u32 = 2;
/// `D3DUSAGE_DYNAMIC`, `D3DLOCK_READONLY`, `D3DLOCK_DISCARD`, `D3DLOCK_NOOVERWRITE`.
const USAGE_DYNAMIC: u32 = 0x200;
const LOCK_READONLY: u32 = 0x10;
const LOCK_DISCARD: u32 = 0x2000;
const LOCK_NOOVERWRITE: u32 = 0x1000;
/// `D3DPRESENT_INTERVAL_IMMEDIATE`.
const INTERVAL_IMMEDIATE: u32 = 0x8000_0000;

/// The real device's presentation interval for the game's `interval` (its `iPresentInterval`):
/// the game's, unless `NV_D3D9_PRESENT=immediate` (a measurement setting of the host, not the
/// game's) asks for `D3DPRESENT_INTERVAL_IMMEDIATE`, so a frame rate measures the work and not
/// the display's refresh. The model keeps the game's parameters and answers.
fn presentation_interval(interval: u32) -> u32 {
    static IMMEDIATE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let immediate = *IMMEDIATE.get_or_init(|| {
        std::env::var("NV_D3D9_PRESENT").is_ok_and(|v| v.eq_ignore_ascii_case("immediate"))
    });
    choose_interval(interval, immediate)
}

fn choose_interval(interval: u32, immediate: bool) -> u32 {
    if immediate {
        INTERVAL_IMMEDIATE
    } else {
        interval
    }
}

#[inline]
fn hr(r: windows::core::Result<()>) -> u32 {
    match r {
        Ok(()) => 0,
        Err(err) => err.code().0 as u32,
    }
}

/// A rectangle payload (4 words) as a `RECT`.
fn rect(d: &Option<Data<'_>>) -> Option<RECT> {
    d.as_ref().map(|d| {
        let w = d.to_words();
        RECT {
            left: w[0] as i32,
            top: w[1] as i32,
            right: w[2] as i32,
            bottom: w[3] as i32,
        }
    })
}

fn rect_ptr(r: &Option<RECT>) -> *const RECT {
    r.as_ref().map_or(std::ptr::null(), |r| r as *const RECT)
}

/// The game's `D3DPRESENT_PARAMETERS` (14 words, the 32-bit layout) with the real device window.
fn present_parameters(words: &[u32], window: isize) -> D3DPRESENT_PARAMETERS {
    let w = |i: usize| words.get(i).copied().unwrap_or(0);
    D3DPRESENT_PARAMETERS {
        BackBufferWidth: w(0),
        BackBufferHeight: w(1),
        BackBufferFormat: D3DFORMAT(w(2)),
        BackBufferCount: w(3),
        MultiSampleType: D3DMULTISAMPLE_TYPE(w(4) as i32),
        MultiSampleQuality: w(5),
        SwapEffect: D3DSWAPEFFECT(w(6) as i32),
        hDeviceWindow: HWND(window as *mut _),
        Windowed: BOOL(w(8) as i32),
        EnableAutoDepthStencil: BOOL(w(9) as i32),
        AutoDepthStencilFormat: D3DFORMAT(w(10)),
        Flags: w(11),
        FullScreen_RefreshRateInHz: w(12),
        PresentationInterval: presentation_interval(w(13)),
    }
}

impl RealBackend {
    pub fn new() -> RealBackend {
        RealBackend::default()
    }

    /// A backend that keeps `board` up to date with its queries (for the render thread).
    pub fn with_query_board(board: std::sync::Arc<QueryBoard>) -> RealBackend {
        RealBackend {
            board: Some(board),
            ..RealBackend::default()
        }
    }

    /// Prints, once per kind, a call where the real runtime answers otherwise than the model.
    #[inline]
    fn answer(&mut self, call: &'static str, model: u32, real: u32) {
        if model != real {
            self.differs(call, model, real);
        }
    }

    #[cold]
    #[inline(never)]
    fn differs(&mut self, call: &'static str, model: u32, real: u32) {
        if self.logged.insert((call, model, real)) {
            eprintln!(
                "NV_D3D9=real: {call}: the model answers {model:#010x}, the real runtime {real:#010x}"
            );
        }
    }

    /// A modelled object's real counterpart is missing: printed once by call.
    fn missing(&mut self, call: &'static str) {
        self.answer(call, 0, 0xffff_ffff);
    }

    /// The real device of the modelled device `at`, borrowed (the table holds its reference for
    /// the whole command: only `Destroy` takes it out, into the graveyard).
    #[inline]
    fn device(&self, at: u32) -> Option<ManuallyDrop<IDirect3DDevice9>> {
        let found = match &self.device {
            Some((d, device)) if *d == at => Some(&**device),
            _ => match self.objects.get(&at) {
                Some(Object::Device(d)) => Some(d),
                _ => None,
            },
        };
        // SAFETY: an interface is one non-null COM pointer; the copy is never released.
        found.map(|d| ManuallyDrop::new(unsafe { std::mem::transmute_copy(d) }))
    }

    fn insert(&mut self, at: u32, object: Object) {
        if let Object::Device(d) = &object {
            // SAFETY: as in `device`; the copy goes when the entry does (here and in `destroy`).
            self.device = Some((
                at,
                ManuallyDrop::new(unsafe { std::mem::transmute_copy(d) }),
            ));
        } else if self.device.as_ref().is_some_and(|(d, _)| *d == at) {
            self.device = None;
        }
        // A counterpart replaced in place (the back buffers after a `Reset`) is released now, as
        // the forwarding always did; only destroyed objects wait for the `Present`.
        self.objects.insert(at, object);
    }

    fn destroy(&mut self, at: u32) {
        if self.device.as_ref().is_some_and(|(d, _)| *d == at) {
            self.device = None;
        }
        if let Some(o) = self.objects.remove(&at) {
            self.graveyard.push(o);
        }
        self.dynamic.remove(&at);
        self.kept.remove(&at);
        if let Some(board) = &self.board {
            board.lock().remove(&at);
        }
    }

    fn surface(&self, at: u32) -> Option<IDirect3DSurface9> {
        match self.objects.get(&at) {
            Some(Object::Surface(s)) => Some(s.clone()),
            _ => None,
        }
    }

    /// Tests: the real texture or cube texture behind the modelled `at`.
    #[cfg(test)]
    pub(crate) fn test_texture(&self, at: u32) -> Option<IDirect3DBaseTexture9> {
        self.base_texture(at)
    }

    fn base_texture(&self, at: u32) -> Option<IDirect3DBaseTexture9> {
        use windows::core::Interface;
        match self.objects.get(&at) {
            Some(Object::Texture(t)) => t.cast().ok(),
            Some(Object::Cube(t)) => t.cast().ok(),
            _ => None,
        }
    }

    fn broken(&mut self, device: u32, binding: u32, call: &'static str) {
        self.broken.insert((device, binding));
        self.answer(call, 0, 0xffff_ffff);
    }

    #[inline]
    fn mended(&mut self, device: u32, binding: u32) {
        // Nearly always none is broken: then there is nothing to look up.
        if !self.broken.is_empty() {
            self.broken.remove(&(device, binding));
        }
    }

    /// Whether the real draws of `device` are skipped (a binding without a real counterpart).
    #[inline]
    fn skipped(&mut self, device: u32) -> bool {
        let any = !self.broken.is_empty() && self.broken.iter().any(|&(d, _)| d == device);
        if any {
            self.missing("a draw skipped: a binding has no real counterpart");
        }
        any
    }

    /// The real device's back buffers and depth surface as the counterparts of the modelled ones.
    fn implicit_surfaces(
        &mut self,
        d: &IDirect3DDevice9,
        back_buffers: &[u32],
        depth_stencil: u32,
    ) {
        for (i, &surface) in back_buffers.iter().enumerate() {
            if let Ok(s) = unsafe { d.GetBackBuffer(0, i as u32, D3DBACKBUFFER_TYPE_MONO) } {
                self.insert(surface, Object::Surface(s));
            }
        }
        if depth_stencil != 0 {
            if let Ok(s) = unsafe { d.GetDepthStencilSurface() } {
                self.insert(depth_stencil, Object::Surface(s));
            }
        }
    }

    fn create_device(
        &mut self,
        device: u32,
        focus: isize,
        window: isize,
        behavior: u32,
        present: &[u32],
        back_buffers: &[u32],
        depth_stencil: u32,
    ) -> Reply {
        let none = Reply::Device {
            created: false,
            native: None,
        };
        if self.d3d.is_none() {
            match unsafe { Direct3DCreate9(32) } {
                Some(d3d) => self.d3d = Some(d3d),
                None => {
                    eprintln!("NV_D3D9=real: Direct3DCreate9 answered null");
                    return none;
                }
            }
        }
        let d3d = self.d3d.clone().expect("made above");
        let mut pp = present_parameters(present, window);
        let mut out = None;
        let r = unsafe {
            d3d.CreateDevice(
                0,
                D3DDEVTYPE_HAL,
                HWND(focus as *mut _),
                behavior | FPU_PRESERVE,
                &mut pp,
                &mut out,
            )
        };
        let Some(real) = out.filter(|_| r.is_ok()) else {
            eprintln!(
                "NV_D3D9=real: CreateDevice answered {:#010x}: nothing is forwarded",
                hr(r)
            );
            return none;
        };
        self.implicit_surfaces(&real, back_buffers, depth_stencil);
        self.insert(device, Object::Device(real.clone()));
        eprintln!(
            "NV_D3D9=real: the game's device is on the real window ({}x{}, format {}, behaviour {behavior:#x})",
            present.first().copied().unwrap_or(0),
            present.get(1).copied().unwrap_or(0),
            present.get(2).copied().unwrap_or(0),
        );
        Reply::Device {
            created: true,
            native: Some(Native(Box::new(SharedDevice(real)))),
        }
    }

    /// The level or surface's bytes (rows packed, `size` of them): the kept ones when its real
    /// surface could not take the last upload, else the real surface's.
    fn read_surface(&mut self, surface: u32, size: u32) -> Option<Vec<u8>> {
        if let Some(bytes) = self.kept.get(&surface) {
            return (bytes.len() == size as usize).then(|| bytes.clone());
        }
        let s = self.surface(surface)?;
        let mut desc = D3DSURFACE_DESC::default();
        unsafe { s.GetDesc(&mut desc) }.ok()?;
        let (block, bytes) = super::d3d9device::format_blocks(desc.Format.0)?;
        let row = desc.Width.div_ceil(block) * bytes;
        let rows = desc.Height.div_ceil(block);
        if row * rows != size {
            return None;
        }
        let mut locked = D3DLOCKED_RECT::default();
        let r = unsafe { s.LockRect(&mut locked, std::ptr::null(), LOCK_READONLY) };
        if r.is_err() {
            self.answer("LockRect of the real surface to read", 0, hr(r));
            return None;
        }
        let mut out = Vec::with_capacity(size as usize);
        for y in 0..rows {
            out.extend_from_slice(unsafe {
                std::slice::from_raw_parts(
                    (locked.pBits as *const u8).add(y as usize * locked.Pitch as usize),
                    row as usize,
                )
            });
        }
        let _ = unsafe { s.UnlockRect() };
        Some(out)
    }

    /// The game unlocked `surface`: its `rows` rows of `row` bytes go to the real surface, or are
    /// kept here when it has none or cannot be locked.
    fn upload_surface(&mut self, surface: u32, row: u32, rows: u32, data: &Data<'_>) {
        let Some(s) = self.surface(surface) else {
            self.kept.insert(surface, data.bytes().into_owned());
            return;
        };
        let mut locked = D3DLOCKED_RECT::default();
        let r = unsafe { s.LockRect(&mut locked, std::ptr::null(), 0) };
        if r.is_err() {
            self.answer("LockRect of the real surface", 0, hr(r));
            self.kept.insert(surface, data.bytes().into_owned());
            return;
        }
        for y in 0..rows {
            let to = unsafe {
                std::slice::from_raw_parts_mut(
                    (locked.pBits as *mut u8).add(y as usize * locked.Pitch as usize),
                    row as usize,
                )
            };
            data.copy_to(y * row, to);
        }
        let _ = unsafe { s.UnlockRect() };
        if !self.kept.is_empty() {
            self.kept.remove(&surface);
        }
        if face_check::enabled() {
            // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
            super::d3d9real::face_check_upload(surface, &s, &data.bytes(), row, rows);
        }
    }

    fn upload_buffer(&mut self, buffer: u32, offset: u32, flags: u32, data: &Data<'_>) {
        if flags & LOCK_READONLY != 0 {
            return;
        }
        let size = data.len();
        let pass = if self.dynamic.contains(&buffer) {
            flags & (LOCK_DISCARD | LOCK_NOOVERWRITE)
        } else {
            0
        };
        let mut at: *mut core::ffi::c_void = std::ptr::null_mut();
        let r = match self.objects.get(&buffer) {
            Some(Object::VertexBuffer(b)) => unsafe { b.Lock(offset, size, &mut at, pass) },
            Some(Object::IndexBuffer(b)) => unsafe { b.Lock(offset, size, &mut at, pass) },
            _ => return,
        };
        if r.is_err() || at.is_null() {
            return self.answer("Lock of the real buffer", 0, hr(r));
        }
        data.copy_to(0, unsafe {
            std::slice::from_raw_parts_mut(at as *mut u8, size as usize)
        });
        let _ = match self.objects.get(&buffer) {
            Some(Object::VertexBuffer(b)) => unsafe { b.Unlock() },
            Some(Object::IndexBuffer(b)) => unsafe { b.Unlock() },
            _ => Ok(()),
        };
    }

    fn render_target_data(
        &mut self,
        device: u32,
        source: u32,
        destination: u32,
        pitch: u32,
    ) -> Option<Rows> {
        let d = self.device(device)?;
        let (from, to) = (self.surface(source)?, self.surface(destination)?);
        let r = unsafe { d.GetRenderTargetData(&from, &to) };
        self.answer("GetRenderTargetData", 0, hr(r));
        let none = Rows {
            row: 0,
            bytes: Vec::new(),
        };
        if pitch == 0 {
            // A texture level the backend keeps: the caller reads it back when it needs it.
            return Some(none);
        }
        let mut desc = D3DSURFACE_DESC::default();
        let mut locked = D3DLOCKED_RECT::default();
        unsafe {
            if to.GetDesc(&mut desc).is_err()
                || to
                    .LockRect(&mut locked, std::ptr::null(), LOCK_READONLY)
                    .is_err()
            {
                return Some(none);
            }
            let row = pitch.min(locked.Pitch as u32);
            let mut bytes = Vec::with_capacity((row * desc.Height) as usize);
            for y in 0..desc.Height {
                bytes.extend_from_slice(std::slice::from_raw_parts(
                    (locked.pBits as *const u8).add(y as usize * locked.Pitch as usize),
                    row as usize,
                ));
            }
            let _ = to.UnlockRect();
            Some(Rows { row, bytes })
        }
    }

    fn query_get_data(&mut self, query: u32, flags: u32) -> Option<(u32, u32)> {
        let Some(Object::Query(q)) = self.objects.get(&query) else {
            return None;
        };
        Some(query_data(q, flags))
    }
}

/// `GetData` of a real query into 4 bytes: the answer (`S_OK` 0, `S_FALSE` 1 while the GPU is not
/// done, ...) and the result.
pub(crate) fn query_data(q: &IDirect3DQuery9, flags: u32) -> (u32, u32) {
    let mut value = 0u32;
    // The raw slot: `S_FALSE` (not done yet) is a success code the wrapper would fold into Ok.
    let hr = unsafe {
        use windows::core::Interface;
        (Interface::vtable(q).GetData)(
            Interface::as_raw(q),
            &mut value as *mut u32 as *mut core::ffi::c_void,
            4,
            flags,
        )
    };
    (hr.0 as u32, value)
}

/// `D3DISSUE_END`.
const ISSUE_END: u32 = 1;

/// `Set*ShaderConstant*` of `kind` (0..2 vertex F, I, B; 3..5 pixel) with the `count` registers
/// at `p` (4 words each, or one for booleans).
#[inline]
fn forward_constants(
    d: &IDirect3DDevice9,
    kind: u8,
    start: u32,
    p: *const u32,
    count: u32,
) -> windows::core::Result<()> {
    // SAFETY: `p` holds `count` registers of the kind's size.
    unsafe {
        match kind {
            0 => d.SetVertexShaderConstantF(start, p as *const f32, count),
            1 => d.SetVertexShaderConstantI(start, p as *const i32, count),
            2 => d.SetVertexShaderConstantB(start, p as *const BOOL, count),
            3 => d.SetPixelShaderConstantF(start, p as *const f32, count),
            4 => d.SetPixelShaderConstantI(start, p as *const i32, count),
            _ => d.SetPixelShaderConstantB(start, p as *const BOOL, count),
        }
    }
}

impl Backend for RealBackend {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        match cmd {
            Cmd::CreateDevice {
                device,
                focus,
                window,
                behavior,
                present,
                back_buffers,
                depth_stencil,
            } => {
                return self.create_device(
                    device,
                    focus,
                    window,
                    behavior,
                    &present.to_words(),
                    &back_buffers.to_words(),
                    depth_stencil,
                )
            }
            Cmd::ReadSurface { surface, size } => {
                return Reply::Bytes(self.read_surface(surface, size))
            }
            Cmd::GetRenderTargetData {
                device,
                source,
                destination,
                pitch,
            } => return Reply::Rows(self.render_target_data(device, source, destination, pitch)),
            Cmd::QueryGetData { query, flags } => {
                return Reply::Query(self.query_get_data(query, flags))
            }
            Cmd::Destroy { object } => self.destroy(object),
            Cmd::UploadSurface {
                surface,
                row,
                rows,
                data,
            } => self.upload_surface(surface, row, rows, &data),
            Cmd::UploadBuffer {
                buffer,
                offset,
                flags,
                data,
            } => self.upload_buffer(buffer, offset, flags, &data),
            Cmd::SetLod {
                texture,
                lod,
                model,
            } => match self.base_texture(texture) {
                Some(t) => {
                    let previous = unsafe { t.SetLOD(lod) };
                    self.answer("SetLOD (the previous level)", model, previous);
                }
                None => self.missing("SetLOD of a texture without a real one"),
            },
            Cmd::QueryIssue { query, flags } => {
                if let Some(Object::Query(q)) = self.objects.get(&query) {
                    let r = unsafe { q.Issue(flags) };
                    self.answer("Query::Issue", 0, hr(r));
                }
                if flags == ISSUE_END {
                    if let Some(board) = &self.board {
                        if let Some(q) = board.lock().get_mut(&query) {
                            q.ends += 1;
                        }
                    }
                }
            }
            _ => self.device_command(cmd),
        }
        Reply::None
    }
}

impl RealBackend {
    /// A command of a device (the device's methods): nothing without its real device.
    fn device_command(&mut self, cmd: Cmd<Data<'_>>) {
        let device = match cmd {
            Cmd::Reset { device, .. }
            | Cmd::Present { device, .. }
            | Cmd::BeginScene { device, .. }
            | Cmd::EndScene { device, .. }
            | Cmd::Clear { device, .. }
            | Cmd::SetRenderTarget { device, .. }
            | Cmd::SetDepthStencilSurface { device, .. }
            | Cmd::SetViewport { device, .. }
            | Cmd::SetTransform { device, .. }
            | Cmd::SetScissorRect { device, .. }
            | Cmd::SetClipPlane { device, .. }
            | Cmd::SetGammaRamp { device, .. }
            | Cmd::SetRenderState { device, .. }
            | Cmd::SetTextureStageState { device, .. }
            | Cmd::SetSamplerState { device, .. }
            | Cmd::SetTexture { device, .. }
            | Cmd::SetVertexDeclaration { device, .. }
            | Cmd::SetFvf { device, .. }
            | Cmd::SetShader { device, .. }
            | Cmd::SetConstants { device, .. }
            | Cmd::SetStreamSource { device, .. }
            | Cmd::SetIndices { device, .. }
            | Cmd::DrawPrimitive { device, .. }
            | Cmd::DrawIndexedPrimitive { device, .. }
            | Cmd::DrawPrimitiveUp { device, .. }
            | Cmd::SetMaterial { device, .. }
            | Cmd::SetLight { device, .. }
            | Cmd::LightEnable { device, .. }
            | Cmd::SetSoftwareVertexProcessing { device, .. }
            | Cmd::UpdateTexture { device, .. }
            | Cmd::StretchRect { device, .. }
            | Cmd::ColorFill { device, .. }
            | Cmd::CreateTexture { device, .. }
            | Cmd::CreateCubeTexture { device, .. }
            | Cmd::CreateSurface { device, .. }
            | Cmd::CreateOffscreenSurface { device, .. }
            | Cmd::CreateBuffer { device, .. }
            | Cmd::CreateShader { device, .. }
            | Cmd::CreateDeclaration { device, .. }
            | Cmd::CreateQuery { device, .. } => device,
            other => unreachable!("{} is not a device's command", other.name()),
        };
        let Some(d) = self.device(device) else { return };
        let ok = 0;
        match cmd {
            Cmd::Reset {
                window,
                present,
                back_buffers,
                depth_stencil,
                ..
            } => {
                let mut pp = present_parameters(&present.to_words(), window);
                let r = unsafe { d.Reset(&mut pp) };
                self.answer("Reset", ok, hr(r));
                self.implicit_surfaces(&d, &back_buffers.to_words(), depth_stencil);
            }
            Cmd::Present {
                source,
                dest,
                window,
                dirty,
                model,
                frame,
                ..
            } => {
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_present();
                }
                if dirty {
                    return self.answer("Present with a dirty region (not forwarded)", model, 1);
                }
                if let Some(shots) = shot::wanted() {
                    // Diagnostic only (`NV_SHOT`, not exe behaviour): the frame the game just
                    // drew.
                    shot::at_present(&d, shots, frame);
                }
                // Diagnostic only (`NV_REPORT_DIR`, not exe behaviour): the player pressed F12.
                shot::report_at_present(&d);
                let (source, dest) = (rect(&source), rect(&dest));
                let r = unsafe {
                    d.Present(
                        rect_ptr(&source),
                        rect_ptr(&dest),
                        HWND(window as *mut _),
                        std::ptr::null(),
                    )
                };
                self.answer("Present", model, hr(r));
                // The frame is over: what the model let go of is bound nowhere on the real
                // device now.
                self.graveyard.clear();
            }
            Cmd::BeginScene { model, .. } => {
                let r = unsafe { d.BeginScene() };
                self.answer("BeginScene", model, hr(r));
            }
            Cmd::EndScene { model, .. } => {
                let r = unsafe { d.EndScene() };
                self.answer("EndScene", model, hr(r));
            }
            Cmd::Clear {
                count,
                rects,
                flags,
                colour,
                z,
                stencil,
                model,
                ..
            } => {
                let words = rects.as_ref().map(|r| r.to_words());
                let pointer = words
                    .as_ref()
                    .map_or(std::ptr::null(), |w| w.as_ptr() as *const D3DRECT);
                let r =
                    unsafe { d.Clear(count, pointer, flags, colour, f32::from_bits(z), stencil) };
                self.answer("Clear", model, hr(r));
            }
            Cmd::SetRenderTarget { index, surface, .. } => {
                let real = match surface {
                    0 => None,
                    s => match self.surface(s) {
                        Some(s) => Some(s),
                        None => {
                            return self.missing("SetRenderTarget of a surface without a real one")
                        }
                    },
                };
                let r = unsafe { d.SetRenderTarget(index, real.as_ref()) };
                self.answer("SetRenderTarget", ok, hr(r));
            }
            Cmd::SetDepthStencilSurface { surface, .. } => {
                let real = match surface {
                    0 => None,
                    s => match self.surface(s) {
                        Some(s) => Some(s),
                        None => {
                            return self
                                .missing("SetDepthStencilSurface of a surface without a real one")
                        }
                    },
                };
                let r = unsafe { d.SetDepthStencilSurface(real.as_ref()) };
                self.answer("SetDepthStencilSurface", ok, hr(r));
            }
            Cmd::SetViewport { viewport: v, .. } => {
                let vp = D3DVIEWPORT9 {
                    X: v[0],
                    Y: v[1],
                    Width: v[2],
                    Height: v[3],
                    MinZ: f32::from_bits(v[4]),
                    MaxZ: f32::from_bits(v[5]),
                };
                let r = unsafe { d.SetViewport(&vp) };
                self.answer("SetViewport", ok, hr(r));
            }
            Cmd::SetTransform { state, matrix, .. } => {
                let m = matrix.to_words();
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    face_check::state().transforms.insert(state, m.to_vec());
                }
                let r = unsafe {
                    d.SetTransform(D3DTRANSFORMSTATETYPE(state as i32), m.as_ptr() as *const _)
                };
                self.answer("SetTransform", ok, hr(r));
            }
            Cmd::SetScissorRect { rect: r, .. } => {
                let area = RECT {
                    left: r[0],
                    top: r[1],
                    right: r[2],
                    bottom: r[3],
                };
                let result = unsafe { d.SetScissorRect(&area) };
                self.answer("SetScissorRect", ok, hr(result));
            }
            Cmd::SetClipPlane { index, plane, .. } => {
                let r = unsafe { d.SetClipPlane(index, plane.as_ptr() as *const f32) };
                self.answer("SetClipPlane", ok, hr(r));
            }
            Cmd::SetGammaRamp {
                swap_chain,
                flags,
                ramp,
                ..
            } => {
                // `void` in the SDK: nothing to compare with the model's answer.
                let bytes = ramp.bytes();
                let entry = |at: usize| {
                    bytes
                        .get(at..at + 2)
                        .map_or(0, |b| u16::from_le_bytes([b[0], b[1]]))
                };
                let mut table = D3DGAMMARAMP::default();
                for i in 0..0x100 {
                    table.red[i] = entry(2 * i);
                    table.green[i] = entry(0x200 + 2 * i);
                    table.blue[i] = entry(0x400 + 2 * i);
                }
                unsafe { d.SetGammaRamp(swap_chain, flags, &table) };
            }
            Cmd::SetRenderState { state, value, .. } => {
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_render_state(state, value);
                }
                let r = unsafe { d.SetRenderState(D3DRENDERSTATETYPE(state as i32), value) };
                self.answer("SetRenderState", ok, hr(r));
            }
            Cmd::SetTextureStageState {
                stage, kind, value, ..
            } => {
                let r = unsafe {
                    d.SetTextureStageState(stage, D3DTEXTURESTAGESTATETYPE(kind as i32), value)
                };
                self.answer("SetTextureStageState", ok, hr(r));
            }
            Cmd::SetSamplerState {
                sampler,
                kind,
                value,
                ..
            } => {
                let r =
                    unsafe { d.SetSamplerState(sampler, D3DSAMPLERSTATETYPE(kind as i32), value) };
                self.answer("SetSamplerState", ok, hr(r));
            }
            Cmd::SetTexture {
                sampler, texture, ..
            } => {
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_bind(sampler, texture);
                }
                let r = match self.objects.get(&texture) {
                    _ if texture == 0 => unsafe {
                        d.SetTexture(sampler, None::<&IDirect3DBaseTexture9>)
                    },
                    Some(Object::Texture(t)) => unsafe { d.SetTexture(sampler, t) },
                    Some(Object::Cube(t)) => unsafe { d.SetTexture(sampler, t) },
                    _ => return self.missing("SetTexture of a texture without a real one"),
                };
                self.answer("SetTexture", ok, hr(r));
            }
            Cmd::SetVertexDeclaration { declaration, .. } => {
                let r = match self.objects.get(&declaration) {
                    _ if declaration == 0 => unsafe {
                        d.SetVertexDeclaration(None::<&IDirect3DVertexDeclaration9>)
                    },
                    Some(Object::Declaration(v)) => unsafe { d.SetVertexDeclaration(v) },
                    _ => {
                        let _ =
                            unsafe { d.SetVertexDeclaration(None::<&IDirect3DVertexDeclaration9>) };
                        return self.broken(
                            device,
                            DECLARATION,
                            "SetVertexDeclaration without a real one",
                        );
                    }
                };
                self.mended(device, DECLARATION);
                self.answer("SetVertexDeclaration", ok, hr(r));
            }
            Cmd::SetFvf { fvf, .. } => {
                let r = unsafe { d.SetFVF(fvf) };
                self.answer("SetFVF", ok, hr(r));
            }
            Cmd::SetShader { pixel, shader, .. } => {
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_shader(shader, pixel);
                }
                let r = match (pixel, self.objects.get(&shader)) {
                    (false, _) if shader == 0 => unsafe {
                        d.SetVertexShader(None::<&IDirect3DVertexShader9>)
                    },
                    (true, _) if shader == 0 => unsafe {
                        d.SetPixelShader(None::<&IDirect3DPixelShader9>)
                    },
                    (false, Some(Object::VertexShader(s))) => unsafe { d.SetVertexShader(s) },
                    (true, Some(Object::PixelShader(s))) => unsafe { d.SetPixelShader(s) },
                    _ => {
                        let _ = unsafe {
                            if pixel {
                                d.SetPixelShader(None::<&IDirect3DPixelShader9>)
                            } else {
                                d.SetVertexShader(None::<&IDirect3DVertexShader9>)
                            }
                        };
                        let which = if pixel { PIXEL_SHADER } else { VERTEX_SHADER };
                        return self.broken(device, which, "Set*Shader without a real one");
                    }
                };
                self.mended(device, if pixel { PIXEL_SHADER } else { VERTEX_SHADER });
                self.answer("Set*Shader", ok, hr(r));
            }
            Cmd::SetConstants {
                kind,
                start,
                count,
                data,
                ..
            } => {
                let words = data.to_words();
                let per = if kind % 3 == 2 { 1 } else { 4 };
                assert!(
                    words.len() >= (per * count) as usize,
                    "SetConstants: {count} registers in {} words",
                    words.len()
                );
                if face_check::enabled() && (kind == 0 || kind == 3) {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_constants(kind == 3, start, &words);
                }
                let r = forward_constants(&d, kind, start, words.as_ptr(), count);
                self.answer("Set*ShaderConstant*", ok, hr(r));
            }
            Cmd::SetStreamSource {
                stream,
                buffer,
                offset,
                stride,
                ..
            } => {
                let r = match self.objects.get(&buffer) {
                    _ if buffer == 0 => unsafe {
                        d.SetStreamSource(stream, None::<&IDirect3DVertexBuffer9>, offset, stride)
                    },
                    Some(Object::VertexBuffer(b)) => unsafe {
                        d.SetStreamSource(stream, b, offset, stride)
                    },
                    _ => {
                        let _ = unsafe {
                            d.SetStreamSource(stream, None::<&IDirect3DVertexBuffer9>, 0, 0)
                        };
                        return self.broken(
                            device,
                            stream,
                            "SetStreamSource without a real buffer",
                        );
                    }
                };
                self.mended(device, stream);
                self.answer("SetStreamSource", ok, hr(r));
            }
            Cmd::SetIndices { buffer, .. } => {
                let r = match self.objects.get(&buffer) {
                    _ if buffer == 0 => unsafe { d.SetIndices(None::<&IDirect3DIndexBuffer9>) },
                    Some(Object::IndexBuffer(b)) => unsafe { d.SetIndices(b) },
                    _ => {
                        let _ = unsafe { d.SetIndices(None::<&IDirect3DIndexBuffer9>) };
                        return self.broken(device, INDICES, "SetIndices without a real buffer");
                    }
                };
                self.mended(device, INDICES);
                self.answer("SetIndices", ok, hr(r));
            }
            Cmd::DrawPrimitive {
                kind, start, count, ..
            } => {
                if self.skipped(device) {
                    return;
                }
                let r = unsafe { d.DrawPrimitive(D3DPRIMITIVETYPE(kind as i32), start, count) };
                self.answer("DrawPrimitive", ok, hr(r));
            }
            Cmd::DrawIndexedPrimitive {
                kind,
                base_vertex,
                min_index,
                vertices,
                start_index,
                count,
                ..
            } => {
                let skip = self.skipped(device);
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_draw(
                        kind,
                        base_vertex as u32,
                        min_index,
                        vertices,
                        start_index,
                        count,
                        skip,
                    );
                }
                if skip {
                    return;
                }
                let r = unsafe {
                    d.DrawIndexedPrimitive(
                        D3DPRIMITIVETYPE(kind as i32),
                        base_vertex,
                        min_index,
                        vertices,
                        start_index,
                        count,
                    )
                };
                self.answer("DrawIndexedPrimitive", ok, hr(r));
            }
            Cmd::DrawPrimitiveUp {
                kind,
                count,
                stride,
                vertices,
                ..
            } => {
                if self.skipped(device) {
                    return;
                }
                let words = vertices.to_words();
                let r = unsafe {
                    d.DrawPrimitiveUP(
                        D3DPRIMITIVETYPE(kind as i32),
                        count,
                        words.as_ptr() as *const _,
                        stride,
                    )
                };
                self.answer("DrawPrimitiveUP", ok, hr(r));
            }
            Cmd::SetMaterial { material, .. } => {
                let m = material.to_words();
                assert!(m.len() >= 17, "SetMaterial: {} words", m.len());
                let r = unsafe { d.SetMaterial(m.as_ptr() as *const D3DMATERIAL9) };
                self.answer("SetMaterial", ok, hr(r));
            }
            Cmd::SetLight { index, light, .. } => {
                let l = light.to_words();
                assert!(l.len() >= 26, "SetLight: {} words", l.len());
                let r = unsafe { d.SetLight(index, l.as_ptr() as *const D3DLIGHT9) };
                self.answer("SetLight", ok, hr(r));
            }
            Cmd::LightEnable { index, enable, .. } => {
                let r = unsafe { d.LightEnable(index, BOOL(enable as i32)) };
                self.answer("LightEnable", ok, hr(r));
            }
            Cmd::SetSoftwareVertexProcessing { software, .. } => {
                let r = unsafe { d.SetSoftwareVertexProcessing(BOOL(software as i32)) };
                self.answer("SetSoftwareVertexProcessing", ok, hr(r));
            }
            Cmd::UpdateTexture {
                source,
                destination,
                ..
            } => {
                let (Some(from), Some(to)) =
                    (self.base_texture(source), self.base_texture(destination))
                else {
                    return self.missing("UpdateTexture without real textures");
                };
                let r = unsafe { d.UpdateTexture(&from, &to) };
                self.answer("UpdateTexture", ok, hr(r));
            }
            Cmd::StretchRect {
                source,
                source_rect,
                destination,
                destination_rect,
                filter,
                ..
            } => {
                let (Some(from), Some(to)) = (self.surface(source), self.surface(destination))
                else {
                    return self.missing("StretchRect without real surfaces");
                };
                let (a, b) = (rect(&source_rect), rect(&destination_rect));
                let r = unsafe {
                    d.StretchRect(
                        &from,
                        rect_ptr(&a),
                        &to,
                        rect_ptr(&b),
                        D3DTEXTUREFILTERTYPE(filter as i32),
                    )
                };
                self.answer("StretchRect", ok, hr(r));
            }
            Cmd::ColorFill {
                surface,
                rect: area,
                colour,
                ..
            } => {
                let Some(s) = self.surface(surface) else {
                    return self.missing("ColorFill of a surface without a real one");
                };
                let area = rect(&area);
                let r = unsafe { d.ColorFill(&s, rect_ptr(&area), colour) };
                self.answer("ColorFill", ok, hr(r));
            }
            Cmd::CreateTexture {
                texture,
                levels,
                width,
                height,
                usage,
                format,
                pool,
                ..
            } => {
                let levels = levels.to_words();
                let mut out = None;
                let r = unsafe {
                    d.CreateTexture(
                        width,
                        height,
                        levels.len() as u32,
                        usage,
                        D3DFORMAT(format),
                        D3DPOOL(pool as i32),
                        &mut out,
                        std::ptr::null_mut(),
                    )
                };
                let Some(t) = out.filter(|_| r.is_ok()) else {
                    return self.answer("CreateTexture", 0, hr(r));
                };
                for (i, &level) in levels.iter().enumerate() {
                    if let Ok(s) = unsafe { t.GetSurfaceLevel(i as u32) } {
                        self.insert(level, Object::Surface(s));
                    }
                }
                self.insert(texture, Object::Texture(t));
                if face_check::enabled() {
                    // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                    super::d3d9real::face_check_texture(
                        texture,
                        &levels,
                        (width, height),
                        format,
                        usage,
                        pool,
                    );
                }
            }
            Cmd::CreateCubeTexture {
                texture,
                surfaces,
                edge,
                usage,
                format,
                pool,
                ..
            } => {
                let surfaces = surfaces.to_words();
                let levels = surfaces.len() as u32 / 6;
                let mut out = None;
                let r = unsafe {
                    d.CreateCubeTexture(
                        edge,
                        levels,
                        usage,
                        D3DFORMAT(format),
                        D3DPOOL(pool as i32),
                        &mut out,
                        std::ptr::null_mut(),
                    )
                };
                let Some(t) = out.filter(|_| r.is_ok()) else {
                    return self.answer("CreateCubeTexture", 0, hr(r));
                };
                for (i, &surface) in surfaces.iter().enumerate() {
                    let (face, level) = (i as u32 / levels, i as u32 % levels);
                    if let Ok(s) =
                        unsafe { t.GetCubeMapSurface(D3DCUBEMAP_FACES(face as i32), level) }
                    {
                        self.insert(surface, Object::Surface(s));
                    }
                }
                self.insert(texture, Object::Cube(t));
            }
            Cmd::CreateSurface {
                surface,
                depth,
                width,
                height,
                format,
                multisample,
                quality,
                flag,
                ..
            } => {
                let mut out = None;
                let (format, multisample, flag) = (
                    D3DFORMAT(format),
                    D3DMULTISAMPLE_TYPE(multisample as i32),
                    BOOL(flag as i32),
                );
                let null = std::ptr::null_mut();
                let r = unsafe {
                    if depth {
                        d.CreateDepthStencilSurface(
                            width,
                            height,
                            format,
                            multisample,
                            quality,
                            flag,
                            &mut out,
                            null,
                        )
                    } else {
                        d.CreateRenderTarget(
                            width,
                            height,
                            format,
                            multisample,
                            quality,
                            flag,
                            &mut out,
                            null,
                        )
                    }
                };
                match out.filter(|_| r.is_ok()) {
                    Some(s) => self.insert(surface, Object::Surface(s)),
                    None => self.answer("CreateRenderTarget/DepthStencilSurface", 0, hr(r)),
                }
            }
            Cmd::CreateOffscreenSurface {
                surface,
                width,
                height,
                format,
                pool,
                ..
            } => {
                let mut out = None;
                let r = unsafe {
                    d.CreateOffscreenPlainSurface(
                        width,
                        height,
                        D3DFORMAT(format),
                        D3DPOOL(pool as i32),
                        &mut out,
                        std::ptr::null_mut(),
                    )
                };
                match out.filter(|_| r.is_ok()) {
                    Some(s) => self.insert(surface, Object::Surface(s)),
                    None => self.answer("CreateOffscreenPlainSurface", 0, hr(r)),
                }
            }
            Cmd::CreateBuffer {
                buffer,
                index,
                length,
                usage,
                format,
                pool,
                ..
            } => {
                let null = std::ptr::null_mut();
                let object = if index {
                    let mut out = None;
                    let r = unsafe {
                        d.CreateIndexBuffer(
                            length,
                            usage,
                            D3DFORMAT(format),
                            D3DPOOL(pool as i32),
                            &mut out,
                            null,
                        )
                    };
                    match out.filter(|_| r.is_ok()) {
                        Some(b) => Object::IndexBuffer(b),
                        None => return self.answer("CreateIndexBuffer", 0, hr(r)),
                    }
                } else {
                    let mut out = None;
                    let r = unsafe {
                        d.CreateVertexBuffer(
                            length,
                            usage,
                            format,
                            D3DPOOL(pool as i32),
                            &mut out,
                            null,
                        )
                    };
                    match out.filter(|_| r.is_ok()) {
                        Some(b) => Object::VertexBuffer(b),
                        None => return self.answer("CreateVertexBuffer", 0, hr(r)),
                    }
                };
                self.insert(buffer, object);
                if usage & USAGE_DYNAMIC != 0 {
                    self.dynamic.insert(buffer);
                } else {
                    self.dynamic.remove(&buffer);
                }
            }
            Cmd::CreateShader {
                shader,
                pixel,
                tokens,
                ..
            } => {
                let tokens = tokens.to_words();
                let object = if pixel {
                    match unsafe { d.CreatePixelShader(tokens.as_ptr()) } {
                        Ok(s) => Object::PixelShader(s),
                        Err(err) => {
                            return self.answer("CreatePixelShader", 0, err.code().0 as u32)
                        }
                    }
                } else {
                    match unsafe { d.CreateVertexShader(tokens.as_ptr()) } {
                        Ok(s) => Object::VertexShader(s),
                        Err(err) => {
                            return self.answer("CreateVertexShader", 0, err.code().0 as u32)
                        }
                    }
                };
                self.insert(shader, object);
            }
            Cmd::CreateDeclaration {
                declaration,
                elements,
                ..
            } => {
                let elements = elements.to_words();
                match unsafe {
                    d.CreateVertexDeclaration(elements.as_ptr() as *const D3DVERTEXELEMENT9)
                } {
                    Ok(v) => self.insert(declaration, Object::Declaration(v)),
                    Err(err) => self.answer("CreateVertexDeclaration", 0, err.code().0 as u32),
                }
            }
            Cmd::CreateQuery { query, kind, .. } => {
                let made = match unsafe { d.CreateQuery(D3DQUERYTYPE(kind as i32)) } {
                    Ok(q) => {
                        self.insert(query, Object::Query(q.clone()));
                        Some(SharedQuery(q))
                    }
                    Err(err) => {
                        self.answer("CreateQuery", 0, err.code().0 as u32);
                        None
                    }
                };
                if let Some(board) = &self.board {
                    let count = self.queries_made.entry(query).or_default();
                    *count += 1;
                    board.lock().insert(
                        query,
                        BoardQuery {
                            made: *count,
                            query: made,
                            ends: 0,
                        },
                    );
                }
            }
            other => unreachable!("{} is not a device's command", other.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_presentation_interval_is_the_games_unless_immediate_is_asked() {
        assert_eq!(choose_interval(1, false), 1);
        assert_eq!(choose_interval(0, false), 0);
        assert_eq!(choose_interval(1, true), INTERVAL_IMMEDIATE);
    }

    /// Commands of a device the backend never made, and of objects it never made, are skipped
    /// (no device: nothing to call); reads of what it never had answer none.
    #[test]
    fn commands_without_real_objects_do_nothing() {
        let mut b = RealBackend::new();
        let words = [0u32; 16];
        for cmd in [
            Cmd::SetRenderState {
                device: 1,
                state: 7,
                value: 1,
            },
            Cmd::SetTransform {
                device: 1,
                state: 256,
                matrix: Data::from_words(&words),
            },
            Cmd::DrawIndexedPrimitive {
                device: 1,
                kind: 4,
                base_vertex: 0,
                min_index: 0,
                vertices: 3,
                start_index: 0,
                count: 1,
            },
            Cmd::Destroy { object: 2 },
            Cmd::QueryIssue { query: 3, flags: 1 },
        ] {
            assert!(matches!(b.execute(cmd), Reply::None));
        }
        assert!(matches!(
            b.execute(Cmd::QueryGetData { query: 3, flags: 1 }),
            Reply::Query(None)
        ));
        assert!(matches!(
            b.execute(Cmd::GetRenderTargetData {
                device: 1,
                source: 2,
                destination: 3,
                pitch: 16,
            }),
            Reply::Rows(None)
        ));
    }

    /// A backend with a real device for the modelled device [`DEVICE`] on a hidden window of the
    /// window thread (`behavior`: the creation flags, the game's 0x54); `None` where Direct3D 9
    /// has no device (no display adapter, as on CI).
    fn with_device(behavior: u32) -> Option<RealBackend> {
        let window = super::super::d3d9real::test_window()?;
        let mut b = RealBackend::new();
        let present = super::super::d3d9real::test_present();
        match b.execute(Cmd::CreateDevice {
            device: DEVICE,
            focus: window,
            window,
            behavior,
            present: Data::from_words(&present),
            back_buffers: Data::from_words(&[]),
            depth_stencil: 0,
        }) {
            Reply::Device { created: true, .. } => Some(b),
            _ => None,
        }
    }

    const DEVICE: u32 = 0x1000;

    /// p169's crash: on a pure device (the game's behaviour flags 0x54) the runtime keeps no
    /// reference of its own to a bound vertex buffer. The model lets go of a buffer when the
    /// program rebinds the stream, before the same call reaches the real device; the real buffer
    /// must outlive that call (it is released at the next `Present`), or the real
    /// `SetStreamSource` reads freed memory. Skipped where there is no Direct3D 9 device.
    #[test]
    fn a_buffer_the_model_lets_go_of_stays_alive_until_present() {
        let Some(mut b) = with_device(0x54) else {
            return;
        };
        let (first, second) = (0x2000, 0x2008);
        let buffer = |buffer| Cmd::CreateBuffer {
            device: DEVICE,
            buffer,
            index: false,
            length: 96,
            // D3DUSAGE_WRITEONLY, default pool.
            usage: 8,
            format: 0,
            pool: 0,
        };
        let bind = |buffer| Cmd::SetStreamSource {
            device: DEVICE,
            stream: 0,
            buffer,
            offset: 0,
            stride: if buffer == 0 { 0 } else { 24 },
        };
        b.execute(buffer(first));
        b.execute(bind(first));
        // The model rebinds stream 0: the first buffer goes before the real call.
        b.execute(buffer(second));
        b.execute(Cmd::Destroy { object: first });
        assert_eq!(b.graveyard.len(), 1);
        b.execute(bind(second));
        b.execute(bind(0));
        b.execute(Cmd::Present {
            device: DEVICE,
            source: None,
            dest: None,
            window: 0,
            dirty: false,
            model: 0,
            frame: 1,
        });
        assert!(b.graveyard.is_empty());
    }

    /// The device kept apart from the object table answers as the table does: the same real
    /// device, none once the modelled one is destroyed, and a device at another address found in
    /// the table. Skipped where there is no Direct3D 9 device.
    #[test]
    fn the_kept_device_answers_as_the_object_table() {
        use windows::core::Interface;
        let Some(mut b) = with_device(0x54) else {
            return;
        };
        let raw = |d: Option<ManuallyDrop<IDirect3DDevice9>>| d.map(|d| d.as_raw());
        let table = |b: &RealBackend, at: u32| match b.objects.get(&at) {
            Some(Object::Device(d)) => Some(d.as_raw()),
            _ => None,
        };
        assert!(raw(b.device(DEVICE)).is_some());
        assert_eq!(raw(b.device(DEVICE)), table(&b, DEVICE));
        // Another address: from the table (none there).
        assert_eq!(raw(b.device(DEVICE + 8)), None);
        // The same device at a second address too: the newer is kept, the older found in the
        // table.
        let real = match b.objects.get(&DEVICE) {
            Some(Object::Device(d)) => d.clone(),
            _ => unreachable!("made above"),
        };
        b.insert(DEVICE + 8, Object::Device(real));
        assert_eq!(raw(b.device(DEVICE + 8)), table(&b, DEVICE + 8));
        assert_eq!(raw(b.device(DEVICE)), table(&b, DEVICE));
        // Destroyed: none, and none kept.
        b.execute(Cmd::Destroy { object: DEVICE + 8 });
        assert_eq!(raw(b.device(DEVICE + 8)), None);
        assert!(b.device.is_none());
        assert_eq!(raw(b.device(DEVICE)), table(&b, DEVICE));
        b.execute(Cmd::Destroy { object: DEVICE });
        assert_eq!(raw(b.device(DEVICE)), None);
    }

    /// `SetLOD` on a managed texture reaches the real texture, and the real one answers the
    /// previous level as the model does; a managed level uploaded reads back as given.
    #[test]
    fn set_lod_and_level_bytes_reach_the_real_texture() {
        let Some(mut b) = with_device(0x54) else {
            return;
        };
        let (texture, levels) = (0x3000, [0x3008u32, 0x3010, 0x3018]);
        // 64x64, three levels, A8R8G8B8, managed.
        b.execute(Cmd::CreateTexture {
            device: DEVICE,
            texture,
            levels: Data::from_words(&levels),
            width: 64,
            height: 64,
            usage: 0,
            format: 21,
            pool: 1,
        });
        b.execute(Cmd::SetLod {
            texture,
            lod: 2,
            model: 0,
        });
        b.execute(Cmd::SetLod {
            texture,
            lod: 1,
            model: 2,
        });
        let t = b.base_texture(texture).expect("the real texture");
        assert_eq!(unsafe { t.GetLOD() }, 1);
        // The third level: 16x16, 64-byte rows.
        let bytes: Vec<u8> = (0..1024u32).map(|i| (i * 7) as u8).collect();
        b.execute(Cmd::UploadSurface {
            surface: levels[2],
            row: 64,
            rows: 16,
            data: Data::Bytes(&bytes),
        });
        assert!(b.kept.is_empty(), "the real level took the bytes");
        match b.execute(Cmd::ReadSurface {
            surface: levels[2],
            size: 1024,
        }) {
            Reply::Bytes(Some(read)) => assert_eq!(read, bytes),
            other => panic!("{other:?}"),
        }
        assert!(b.logged.is_empty(), "no answer differed: {:?}", b.logged);
    }

    /// A level without a real surface keeps the bytes it was given, and answers them back; a
    /// destroyed one forgets them.
    #[test]
    fn a_level_without_a_real_surface_keeps_its_bytes() {
        let mut b = RealBackend::new();
        let bytes: Vec<u8> = (0..64u8).collect();
        b.execute(Cmd::UploadSurface {
            surface: 0x40,
            row: 16,
            rows: 4,
            data: Data::Bytes(&bytes),
        });
        match b.execute(Cmd::ReadSurface {
            surface: 0x40,
            size: 64,
        }) {
            Reply::Bytes(Some(read)) => assert_eq!(read, bytes),
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            b.execute(Cmd::ReadSurface {
                surface: 0x40,
                size: 32
            }),
            Reply::Bytes(None)
        ));
        b.execute(Cmd::Destroy { object: 0x40 });
        assert!(matches!(
            b.execute(Cmd::ReadSurface {
                surface: 0x40,
                size: 64
            }),
            Reply::Bytes(None)
        ));
    }
}

/// Test only: a stand-in for the real device and its objects (COM objects of this module whose
/// methods answer `S_OK`), so the forwarding can be driven and measured without a GPU. While
/// recording (`Log::record`) each call that reaches a stand-in device is kept with its
/// arguments (data behind pointers as words, objects by the number they were installed with).
#[cfg(test)]
pub(crate) mod stub {
    use super::*;

    use std::ffi::c_void;
    use windows::core::{Interface, HRESULT};
    use windows::Foundation::Numerics::Matrix4x4;

    /// What each stand-in is installed as.
    #[derive(Debug, Clone, Copy)]
    pub(crate) enum Kind {
        Device,
        Texture,
        VertexBuffer,
        IndexBuffer,
        VertexShader,
        PixelShader,
        Declaration,
    }

    /// A stand-in COM object: its vtable and the number it is recorded by.
    #[repr(C)]
    struct StubObject {
        vtable: *const usize,
        id: u32,
        log: std::sync::Arc<Log>,
    }

    /// The calls that reached a test's stand-ins, while recording: shared by every stand-in
    /// installed with it, and filled on whichever thread the backend runs (the render thread
    /// too).
    #[derive(Default)]
    pub(crate) struct Log(std::sync::Mutex<Option<Vec<String>>>);

    impl Log {
        /// Starts recording (an empty log).
        pub(crate) fn record(&self) {
            *self.0.lock().unwrap() = Some(Vec::new());
        }

        /// The calls recorded since [`Log::record`]; recording stops.
        pub(crate) fn take(&self) -> Vec<String> {
            self.0.lock().unwrap().take().unwrap_or_default()
        }
    }

    /// Notes a call that reached the stand-in `this`, in its log.
    fn note(this: *mut c_void, f: impl FnOnce() -> String) {
        // SAFETY: every stand-in is a `StubObject`, never freed.
        let log = unsafe { &(*(this as *const StubObject)).log };
        if let Some(calls) = log.0.lock().unwrap().as_mut() {
            calls.push(f());
        }
    }

    fn id(p: *mut c_void) -> String {
        if p.is_null() {
            "null".to_string()
        } else {
            // SAFETY: every object a stand-in device is given is a `StubObject`.
            format!("#{}", unsafe { (*(p as *const StubObject)).id })
        }
    }

    fn data<T: Copy>(p: *const T, n: usize, word: impl Fn(T) -> u32) -> String {
        if p.is_null() {
            return "null".to_string();
        }
        // SAFETY: the forwarding passes `n` readable values.
        let v = unsafe { std::slice::from_raw_parts(p, n) };
        v.iter()
            .map(|&x| format!("{:08x}", word(x)))
            .collect::<Vec<_>>()
            .join(" ")
    }

    const OK: HRESULT = HRESULT(0);

    unsafe extern "system" fn unexpected(this: *mut c_void) -> HRESULT {
        note(this, || "an unexpected method".to_string());
        HRESULT(0x8000_4005_u32 as i32)
    }
    unsafe extern "system" fn add_ref(_this: *mut c_void) -> u32 {
        1
    }
    unsafe extern "system" fn release(_this: *mut c_void) -> u32 {
        0
    }
    unsafe extern "system" fn begin_scene(this: *mut c_void) -> HRESULT {
        note(this, || "BeginScene".to_string());
        OK
    }
    unsafe extern "system" fn end_scene(this: *mut c_void) -> HRESULT {
        note(this, || "EndScene".to_string());
        OK
    }
    unsafe extern "system" fn set_transform(
        this: *mut c_void,
        state: D3DTRANSFORMSTATETYPE,
        m: *const Matrix4x4,
    ) -> HRESULT {
        note(this, || {
            format!(
                "SetTransform({}, {})",
                state.0,
                data(m as *const f32, 16, f32::to_bits)
            )
        });
        OK
    }
    unsafe extern "system" fn set_viewport(this: *mut c_void, v: *const D3DVIEWPORT9) -> HRESULT {
        note(this, || {
            // SAFETY: the forwarding passes a viewport.
            let v = unsafe { *v };
            format!(
                "SetViewport({} {} {} {} {:08x} {:08x})",
                v.X,
                v.Y,
                v.Width,
                v.Height,
                v.MinZ.to_bits(),
                v.MaxZ.to_bits()
            )
        });
        OK
    }
    unsafe extern "system" fn set_render_state(
        this: *mut c_void,
        state: D3DRENDERSTATETYPE,
        value: u32,
    ) -> HRESULT {
        note(this, || format!("SetRenderState({}, {value:#x})", state.0));
        OK
    }
    unsafe extern "system" fn clear(
        this: *mut c_void,
        count: u32,
        rects: *const D3DRECT,
        flags: u32,
        colour: u32,
        z: f32,
        stencil: u32,
    ) -> HRESULT {
        note(this, || {
            format!(
                "Clear({count}, [{}], {flags:#x}, {colour:#x}, {:08x}, {stencil})",
                data(rects as *const u32, 4 * count as usize, |w| w),
                z.to_bits()
            )
        });
        OK
    }
    unsafe extern "system" fn set_material(this: *mut c_void, m: *const D3DMATERIAL9) -> HRESULT {
        note(this, || {
            format!("SetMaterial([{}])", data(m as *const u32, 17, |w| w))
        });
        OK
    }
    unsafe extern "system" fn set_light(
        this: *mut c_void,
        index: u32,
        l: *const D3DLIGHT9,
    ) -> HRESULT {
        note(this, || {
            format!("SetLight({index}, [{}])", data(l as *const u32, 26, |w| w))
        });
        OK
    }
    unsafe extern "system" fn draw_primitive_up(
        this: *mut c_void,
        kind: D3DPRIMITIVETYPE,
        count: u32,
        v: *const c_void,
        stride: u32,
    ) -> HRESULT {
        note(this, || {
            let vertices = match kind.0 {
                1 => count,
                2 => 2 * count,
                3 => count + 1,
                4 => 3 * count,
                _ => count + 2,
            };
            format!(
                "DrawPrimitiveUP({}, {count}, [{}], {stride})",
                kind.0,
                data(v as *const u8, (vertices * stride) as usize, u32::from)
            )
        });
        OK
    }
    unsafe extern "system" fn set_texture(
        this: *mut c_void,
        stage: u32,
        t: *mut c_void,
    ) -> HRESULT {
        note(this, || format!("SetTexture({stage}, {})", id(t)));
        OK
    }
    unsafe extern "system" fn set_texture_stage_state(
        this: *mut c_void,
        stage: u32,
        kind: D3DTEXTURESTAGESTATETYPE,
        value: u32,
    ) -> HRESULT {
        note(this, || {
            format!("SetTextureStageState({stage}, {}, {value:#x})", kind.0)
        });
        OK
    }
    unsafe extern "system" fn set_sampler_state(
        this: *mut c_void,
        sampler: u32,
        kind: D3DSAMPLERSTATETYPE,
        value: u32,
    ) -> HRESULT {
        note(this, || {
            format!("SetSamplerState({sampler}, {}, {value:#x})", kind.0)
        });
        OK
    }
    unsafe extern "system" fn draw_primitive(
        this: *mut c_void,
        kind: D3DPRIMITIVETYPE,
        start: u32,
        count: u32,
    ) -> HRESULT {
        note(this, || {
            format!("DrawPrimitive({}, {start}, {count})", kind.0)
        });
        OK
    }
    unsafe extern "system" fn draw_indexed_primitive(
        this: *mut c_void,
        kind: D3DPRIMITIVETYPE,
        base: i32,
        min: u32,
        vertices: u32,
        start: u32,
        count: u32,
    ) -> HRESULT {
        note(this, || {
            format!(
                "DrawIndexedPrimitive({}, {base}, {min}, {vertices}, {start}, {count})",
                kind.0
            )
        });
        OK
    }
    unsafe extern "system" fn set_vertex_declaration(this: *mut c_void, d: *mut c_void) -> HRESULT {
        note(this, || format!("SetVertexDeclaration({})", id(d)));
        OK
    }
    unsafe extern "system" fn set_fvf(this: *mut c_void, fvf: u32) -> HRESULT {
        note(this, || format!("SetFVF({fvf:#x})"));
        OK
    }
    unsafe extern "system" fn set_vertex_shader(this: *mut c_void, s: *mut c_void) -> HRESULT {
        note(this, || format!("SetVertexShader({})", id(s)));
        OK
    }
    unsafe extern "system" fn set_pixel_shader(this: *mut c_void, s: *mut c_void) -> HRESULT {
        note(this, || format!("SetPixelShader({})", id(s)));
        OK
    }
    macro_rules! constants {
        ($($name:ident, $t:ty, $per:expr, $word:expr;)*) => {$(
            unsafe extern "system" fn $name(
                this: *mut c_void,
                start: u32,
                v: *const $t,
                count: u32,
            ) -> HRESULT {
                note(this, || {
                    format!(
                        "{}({start}, [{}], {count})",
                        stringify!($name),
                        data(v, ($per * count) as usize, $word)
                    )
                });
                OK
            }
        )*};
    }
    constants! {
        vertex_constant_f, f32, 4, f32::to_bits;
        vertex_constant_i, i32, 4, |x: i32| x as u32;
        vertex_constant_b, BOOL, 1, |x: BOOL| x.0 as u32;
        pixel_constant_f, f32, 4, f32::to_bits;
        pixel_constant_i, i32, 4, |x: i32| x as u32;
        pixel_constant_b, BOOL, 1, |x: BOOL| x.0 as u32;
    }
    unsafe extern "system" fn set_stream_source(
        this: *mut c_void,
        stream: u32,
        b: *mut c_void,
        offset: u32,
        stride: u32,
    ) -> HRESULT {
        note(this, || {
            format!("SetStreamSource({stream}, {}, {offset}, {stride})", id(b))
        });
        OK
    }
    unsafe extern "system" fn set_indices(this: *mut c_void, b: *mut c_void) -> HRESULT {
        note(this, || format!("SetIndices({})", id(b)));
        OK
    }

    /// The vtable every stand-in shares (`IDirect3DDevice9`'s slots; the other kinds are only
    /// ever counted and released).
    fn vtable() -> *const usize {
        static TABLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        *TABLE.get_or_init(|| Box::leak(Box::new(table())).as_ptr() as usize) as *const usize
    }

    fn table() -> [usize; 128] {
        let mut t = [unexpected as *const () as usize; 128];
        t[1] = add_ref as *const () as usize;
        t[2] = release as *const () as usize;
        t[41] = begin_scene as *const () as usize;
        t[42] = end_scene as *const () as usize;
        t[43] = clear as *const () as usize;
        t[49] = set_material as *const () as usize;
        t[51] = set_light as *const () as usize;
        t[83] = draw_primitive_up as *const () as usize;
        t[44] = set_transform as *const () as usize;
        t[47] = set_viewport as *const () as usize;
        t[57] = set_render_state as *const () as usize;
        t[65] = set_texture as *const () as usize;
        t[67] = set_texture_stage_state as *const () as usize;
        t[69] = set_sampler_state as *const () as usize;
        t[81] = draw_primitive as *const () as usize;
        t[82] = draw_indexed_primitive as *const () as usize;
        t[87] = set_vertex_declaration as *const () as usize;
        t[89] = set_fvf as *const () as usize;
        t[92] = set_vertex_shader as *const () as usize;
        t[94] = vertex_constant_f as *const () as usize;
        t[96] = vertex_constant_i as *const () as usize;
        t[98] = vertex_constant_b as *const () as usize;
        t[100] = set_stream_source as *const () as usize;
        t[104] = set_indices as *const () as usize;
        t[107] = set_pixel_shader as *const () as usize;
        t[109] = pixel_constant_f as *const () as usize;
        t[111] = pixel_constant_i as *const () as usize;
        t[113] = pixel_constant_b as *const () as usize;
        t
    }

    /// Makes `model` (a modelled object's address) have a stand-in real counterpart of `kind` in
    /// `backend`, recorded as `#id` in `log`.
    pub(crate) fn install(
        backend: &mut RealBackend,
        model: u32,
        kind: Kind,
        id: u32,
        log: &std::sync::Arc<Log>,
    ) {
        let raw = Box::into_raw(Box::new(StubObject {
            vtable: vtable(),
            id,
            log: std::sync::Arc::clone(log),
        })) as *mut c_void;
        // SAFETY: `raw` is a COM object whose vtable answers every slot these interfaces have
        // (the stand-ins are never freed: `Release` does nothing).
        let object = unsafe {
            match kind {
                Kind::Device => Object::Device(IDirect3DDevice9::from_raw(raw)),
                Kind::Texture => Object::Texture(IDirect3DTexture9::from_raw(raw)),
                Kind::VertexBuffer => Object::VertexBuffer(IDirect3DVertexBuffer9::from_raw(raw)),
                Kind::IndexBuffer => Object::IndexBuffer(IDirect3DIndexBuffer9::from_raw(raw)),
                Kind::VertexShader => Object::VertexShader(IDirect3DVertexShader9::from_raw(raw)),
                Kind::PixelShader => Object::PixelShader(IDirect3DPixelShader9::from_raw(raw)),
                Kind::Declaration => {
                    Object::Declaration(IDirect3DVertexDeclaration9::from_raw(raw))
                }
            }
        };
        backend.insert(model, object);
    }

    /// How many differences between the model's and the real answers the backend printed.
    pub(crate) fn logged(backend: &RealBackend) -> usize {
        backend.logged.len()
    }
}
