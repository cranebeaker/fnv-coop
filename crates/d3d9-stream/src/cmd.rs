//! The recorded calls ([`Cmd`]), the bytes they carry ([`Data`]), what a sync call answers
//! ([`Reply`]) and what carries them out ([`Backend`]).

use crate::wire::{Reader, Wire, WireError};
use std::borrow::Cow;
use std::fmt;

/// A modelled object: its address in the game's memory, 0 for none (a null pointer). Unique among
/// the objects alive; an address can be used again after [`Cmd::Destroy`], and the stream's order
/// says which object is meant.
pub type Handle = u32;

/// A rectangle as Win32 lays one out: left, top, right, bottom.
pub type Rect = [i32; 4];

/// The bytes a command carries (a matrix, constants, an unlock's bytes, shader tokens).
///
/// Commands handed to a render thread always carry [`Data::Bytes`], copied into the stream. On the
/// calling thread ([`crate::Stream::direct`]) a large payload can be read where it lies
/// ([`Data::Read`], the game's memory during the call), so the default path copies no more than a
/// direct call does.
#[derive(Clone, Copy)]
pub enum Data<'a> {
    /// The bytes themselves (4-aligned when they hold words, as the stream keeps them).
    Bytes(&'a [u8]),
    /// `len` bytes, read on demand: `read(offset, into)` fills `into` with the bytes from
    /// `offset`.
    Read {
        len: u32,
        read: &'a dyn Fn(u32, &mut [u8]),
    },
}

impl<'a> Data<'a> {
    /// Words as bytes (little-endian, as Direct3D 9 lays them out).
    pub fn from_words(words: &'a [u32]) -> Data<'a> {
        // SAFETY: `words` is `4 * len` initialised bytes; a byte view of words has no alignment or
        // validity requirement of its own.
        Data::Bytes(unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), 4 * words.len()) })
    }

    /// How many bytes.
    pub fn len(&self) -> u32 {
        match self {
            Data::Bytes(b) => b.len() as u32,
            Data::Read { len, .. } => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Fills `into` with the bytes from `offset` (`offset + into.len()` must not pass the end).
    pub fn copy_to(&self, offset: u32, into: &mut [u8]) {
        let end = offset as usize + into.len();
        assert!(
            end <= self.len() as usize,
            "Data::copy_to: bytes {offset}..{end} of {}",
            self.len()
        );
        match self {
            Data::Bytes(b) => into.copy_from_slice(&b[offset as usize..end]),
            Data::Read { read, .. } => read(offset, into),
        }
    }

    /// All the bytes: borrowed when they are at hand, read otherwise.
    pub fn bytes(&self) -> Cow<'a, [u8]> {
        match *self {
            Data::Bytes(b) => Cow::Borrowed(b),
            Data::Read { len, .. } => {
                let mut v = vec![0; len as usize];
                self.copy_to(0, &mut v);
                Cow::Owned(v)
            }
        }
    }

    /// The bytes as little-endian words (a last partial word padded with zeros): borrowed when
    /// they lie 4-aligned, copied otherwise.
    pub fn to_words(&self) -> Cow<'a, [u32]> {
        #[cfg(target_endian = "little")]
        if let Data::Bytes(b) = *self {
            if b.as_ptr() as usize % 4 == 0 && b.len() % 4 == 0 {
                // SAFETY: aligned, a whole number of words, and every bit pattern is a word.
                return Cow::Borrowed(unsafe {
                    std::slice::from_raw_parts(b.as_ptr().cast(), b.len() / 4)
                });
            }
        }
        let bytes = self.bytes();
        Cow::Owned(
            bytes
                .chunks(4)
                .map(|c| {
                    let mut w = [0u8; 4];
                    w[..c.len()].copy_from_slice(c);
                    u32::from_le_bytes(w)
                })
                .collect(),
        )
    }
}

/// The bytes in hexadecimal (read when they are not at hand), so a command prints the same
/// whether it was recorded or replayed.
impl fmt::Debug for Data<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.bytes();
        write!(f, "<{}:", bytes.len())?;
        for (i, b) in bytes.iter().enumerate() {
            if i % 4 == 0 {
                f.write_str(" ")?;
            }
            write!(f, "{b:02x}")?;
        }
        f.write_str(">")
    }
}

/// A field of a command: `payload` (bytes, the `P` of [`Cmd`]), `optional` (bytes or none: a
/// pointer that may be null), or a plain value of the given type.
macro_rules! field_ty {
    (payload) => { P };
    (optional) => { Option<P> };
    ($t:ty) => { $t };
}

macro_rules! map_field {
    (payload, $f:ident, $x:expr) => {
        $f($x)
    };
    (optional, $f:ident, $x:expr) => {
        $x.map(&mut $f)
    };
    ($t:tt, $f:ident, $x:expr) => {
        $x
    };
}

macro_rules! put_field {
    (payload, $x:expr, $out:ident) => {
        crate::wire::put_data($x, $out)
    };
    (optional, $x:expr, $out:ident) => {
        crate::wire::put_optional($x, $out)
    };
    ($t:tt, $x:expr, $out:ident) => {
        Wire::put($x, $out)
    };
}

macro_rules! get_field {
    (payload, $r:ident) => {
        $r.data()?
    };
    (optional, $r:ident) => {
        $r.optional()?
    };
    ($t:tt, $r:ident) => {
        Wire::get($r)?
    };
}

/// The command list, written once: the enum, its payload mapping, its names and its encoding
/// (`wire.rs`) all come from it, so they cannot drift apart.
macro_rules! commands {
    ($(
        $(#[$doc:meta])*
        $name:ident { $( $field:ident : $kind:tt ),* $(,)? }
    ),* $(,)?) => {
        /// One recorded Direct3D 9 call, as the game made it on a modelled object. `P` is how the
        /// bytes a call points at are carried: [`Data`] when a command is made and when a
        /// [`Backend`] gets it. (`Debug`, `Clone` and `Copy` are written out below as `derive`
        /// writes them: `derive` does not take fields whose type comes from a macro.)
        pub enum Cmd<P> {
            $( $(#[$doc])* $name { $( $field: field_ty!($kind) ),* } ),*
        }

        impl<P: fmt::Debug> fmt::Debug for Cmd<P> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( Cmd::$name { $( $field ),* } => f
                        .debug_struct(stringify!($name))
                        $( .field(stringify!($field), $field) )*
                        .finish() ),*
                }
            }
        }

        impl<P: Copy> Clone for Cmd<P> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<P: Copy> Copy for Cmd<P> {}

        /// The commands' numbers in the encoding (`wire.rs`), in the list's order.
        #[derive(Clone, Copy)]
        #[repr(u8)]
        enum Tag { $( $name ),* }

        /// Every command's name, in the list's order.
        #[cfg(test)]
        pub(crate) const NAMES: &[&str] = &[ $( stringify!($name) ),* ];

        /// The command list as written (names, fields and their kinds, in order): what a
        /// command's number and encoding mean.
        pub(crate) const LIST: &str = stringify!($( $name { $( $field : $kind ),* } ),*);

        impl<P> Cmd<P> {
            /// The same command with each payload passed through `f`.
            pub fn map<Q>(self, mut f: impl FnMut(P) -> Q) -> Cmd<Q> {
                match self {
                    $( Cmd::$name { $( $field ),* } => Cmd::$name {
                        $( $field: map_field!($kind, f, $field) ),*
                    } ),*
                }
            }

            /// The command's name (the Direct3D 9 method, or the layer's name for it).
            pub fn name(&self) -> &'static str {
                match self {
                    $( Cmd::$name { .. } => stringify!($name) ),*
                }
            }
        }

        impl Cmd<Data<'_>> {
            /// Appends the command's encoding (without its length) to `out`.
            pub(crate) fn put(&self, out: &mut Vec<u8>) {
                match self {
                    $( Cmd::$name { $( $field ),* } => {
                        out.push(Tag::$name as u8);
                        $( put_field!($kind, $field, out); )*
                    } ),*
                }
            }
        }

        impl<'a> Cmd<Data<'a>> {
            /// The command of number `tag`, its fields read from `r`.
            pub(crate) fn get(tag: u8, r: &mut Reader<'a>) -> Result<Cmd<Data<'a>>, WireError> {
                $(
                    if tag == Tag::$name as u8 {
                        $( let $field = get_field!($kind, r); )*
                        return Ok(Cmd::$name { $( $field ),* });
                    }
                )*
                Err(WireError::UnknownCommand { at: r.at(), tag })
            }
        }
    };
}

commands! {
    // The device.

    /// `IDirect3D9::CreateDevice` (sync): the backend makes the device of the modelled `device`.
    /// `focus` and `window` are the real windows (`HWND`s) of the game's focus and device windows
    /// (0: none); `present` is the game's `D3DPRESENT_PARAMETERS` as the model filled it in (14
    /// words in the 32-bit layout; its `hDeviceWindow` word is the game's handle, `window` the real
    /// one); `back_buffers` the modelled back buffers (handles, in order) and `depth_stencil` the
    /// automatic depth-stencil surface (0: none). Replies [`Reply::Device`].
    CreateDevice {
        device: Handle, focus: isize, window: isize, behavior: u32, present: payload,
        back_buffers: payload, depth_stencil: Handle,
    },
    /// `Reset`, after the model's succeeded: as [`Cmd::CreateDevice`]'s parameters; the new back
    /// buffers and depth-stencil surface are the ones named.
    Reset { device: Handle, window: isize, present: payload, back_buffers: payload, depth_stencil: Handle },
    /// `Present(source, dest, window, dirty region)`: `source` and `dest` a rectangle each (4
    /// words) or none; `window` the real window to show it in (0: the device's); `dirty` whether
    /// the game passed a dirty region; `model` the model's answer; `frame` the stream's count of
    /// presents from 1, the same for every backend of a run (set by [`crate::Stream::submit`]).
    Present {
        device: Handle, source: optional, dest: optional, window: isize, dirty: bool, model: u32,
        frame: u32,
    },
    /// `BeginScene()`; `model` the model's answer (forwarded whatever it is).
    BeginScene { device: Handle, model: u32 },
    /// `EndScene()`; `model` the model's answer (forwarded whatever it is).
    EndScene { device: Handle, model: u32 },
    /// `Clear(count, rects, flags, colour, z, stencil)`: `rects` the `D3DRECT`s (4 words each) or
    /// none (a null pointer); `z` the float's bits; `model` the model's answer (forwarded whatever
    /// it is).
    Clear {
        device: Handle, count: u32, rects: optional, flags: u32, colour: u32, z: u32, stencil: u32,
        model: u32,
    },
    /// `SetRenderTarget(index, surface)`; 0 unbinds.
    SetRenderTarget { device: Handle, index: u32, surface: Handle },
    /// `SetDepthStencilSurface(surface)`; 0 unbinds.
    SetDepthStencilSurface { device: Handle, surface: Handle },
    /// `SetViewport`: `D3DVIEWPORT9` as X, Y, Width, Height, then MinZ and MaxZ as float bits.
    SetViewport { device: Handle, viewport: [u32; 6] },
    /// `SetTransform(state, matrix)`: `matrix` 16 float bits, row by row.
    SetTransform { device: Handle, state: u32, matrix: payload },
    /// `SetScissorRect(rect)`.
    SetScissorRect { device: Handle, rect: Rect },
    /// `SetClipPlane(index, plane)`: 4 float bits.
    SetClipPlane { device: Handle, index: u32, plane: [u32; 4] },
    /// `SetGammaRamp(swap chain, flags, ramp)`: `D3DGAMMARAMP`, red, green and blue, 256 16-bit
    /// entries each.
    SetGammaRamp { device: Handle, swap_chain: u32, flags: u32, ramp: payload },
    /// `SetRenderState(state, value)`.
    SetRenderState { device: Handle, state: u32, value: u32 },
    /// `SetTextureStageState(stage, type, value)`.
    SetTextureStageState { device: Handle, stage: u32, kind: u32, value: u32 },
    /// `SetSamplerState(sampler, type, value)`.
    SetSamplerState { device: Handle, sampler: u32, kind: u32, value: u32 },
    /// `SetTexture(sampler, texture)`: a texture or cube texture; 0 unbinds.
    SetTexture { device: Handle, sampler: u32, texture: Handle },
    /// `SetVertexDeclaration(declaration)`; 0 unbinds.
    SetVertexDeclaration { device: Handle, declaration: Handle },
    /// `SetFVF(fvf)`.
    SetFvf { device: Handle, fvf: u32 },
    /// `SetVertexShader` (`pixel` false) or `SetPixelShader`; 0 unbinds.
    SetShader { device: Handle, pixel: bool, shader: Handle },
    /// `Set{Vertex,Pixel}ShaderConstant{F,I,B}(start, data, count)`: `kind` 0, 1, 2 the vertex
    /// shader's F, I, B; 3, 4, 5 the pixel shader's. `data` the `count` registers as the model
    /// holds them after the call: 4 words each for F (float bits) and I, one word each for B.
    SetConstants { device: Handle, kind: u8, start: u32, count: u32, data: payload },
    /// `SetStreamSource(stream, buffer, offset, stride)`; 0 unbinds.
    SetStreamSource { device: Handle, stream: u32, buffer: Handle, offset: u32, stride: u32 },
    /// `SetIndices(buffer)`; 0 unbinds.
    SetIndices { device: Handle, buffer: Handle },
    /// `DrawPrimitive(type, start vertex, primitive count)`.
    DrawPrimitive { device: Handle, kind: u32, start: u32, count: u32 },
    /// `DrawIndexedPrimitive(type, base vertex, min index, vertices, start index, primitive
    /// count)`.
    DrawIndexedPrimitive {
        device: Handle, kind: u32, base_vertex: i32, min_index: u32, vertices: u32,
        start_index: u32, count: u32,
    },
    /// `DrawPrimitiveUP(type, primitive count, vertices, stride)`: `vertices` the vertices'
    /// bytes (as many vertices as the type and count make, `stride` bytes each).
    DrawPrimitiveUp { device: Handle, kind: u32, count: u32, stride: u32, vertices: payload },
    /// `SetMaterial`: `D3DMATERIAL9`, 17 words.
    SetMaterial { device: Handle, material: payload },
    /// `SetLight(index, light)`: `D3DLIGHT9`, 26 words.
    SetLight { device: Handle, index: u32, light: payload },
    /// `LightEnable(index, enable)`.
    LightEnable { device: Handle, index: u32, enable: u32 },
    /// `SetSoftwareVertexProcessing(software)`.
    SetSoftwareVertexProcessing { device: Handle, software: u32 },
    /// `UpdateTexture(source, destination)`.
    UpdateTexture { device: Handle, source: Handle, destination: Handle },
    /// `StretchRect(source, source rect, destination, destination rect, filter)`: each rectangle
    /// 4 words or none.
    StretchRect {
        device: Handle, source: Handle, source_rect: optional, destination: Handle,
        destination_rect: optional, filter: u32,
    },
    /// `ColorFill(surface, rect, colour)`: the rectangle 4 words or none.
    ColorFill { device: Handle, surface: Handle, rect: optional, colour: u32 },

    // Resources. The model already answered with its own object; the backend makes its
    // counterpart when it reaches the command.

    /// `CreateTexture`: `levels` the modelled level surfaces (handles), largest first.
    CreateTexture {
        device: Handle, texture: Handle, levels: payload, width: u32, height: u32, usage: u32,
        format: u32, pool: u32,
    },
    /// `CreateCubeTexture`: `surfaces` the modelled surfaces (handles) face by face (+X, -X, +Y,
    /// -Y, +Z, -Z), each face's levels largest first.
    CreateCubeTexture {
        device: Handle, texture: Handle, surfaces: payload, edge: u32, usage: u32, format: u32,
        pool: u32,
    },
    /// `CreateRenderTarget` (`depth` false) or `CreateDepthStencilSurface`; `flag` is `Lockable`
    /// or `Discard`.
    CreateSurface {
        device: Handle, surface: Handle, depth: bool, width: u32, height: u32, format: u32,
        multisample: u32, quality: u32, flag: u32,
    },
    /// `CreateOffscreenPlainSurface`.
    CreateOffscreenSurface {
        device: Handle, surface: Handle, width: u32, height: u32, format: u32, pool: u32,
    },
    /// `CreateVertexBuffer` (`index` false, `format` the FVF) or `CreateIndexBuffer` (`format`
    /// the index format).
    CreateBuffer {
        device: Handle, buffer: Handle, index: bool, length: u32, usage: u32, format: u32,
        pool: u32,
    },
    /// `CreateVertexShader` (`pixel` false) or `CreatePixelShader`: `tokens` the game's Direct3D 9
    /// shader bytecode.
    CreateShader { device: Handle, shader: Handle, pixel: bool, tokens: payload },
    /// `CreateVertexDeclaration`: `elements` the `D3DVERTEXELEMENT9`s, `D3DDECL_END` included.
    CreateDeclaration { device: Handle, declaration: Handle, elements: payload },
    /// `CreateQuery(type)`: an event (8) or occlusion (9) query.
    CreateQuery { device: Handle, query: Handle, kind: u32 },
    /// The modelled object is destroyed; its handle may name a new object after this. (The model
    /// lets go as soon as the game releases it, which can be before the call that unbinds it: a
    /// backend keeps what is still bound, as the real one keeps it until the next `Present`.)
    Destroy { object: Handle },
    /// The game unlocked a texture level or lockable surface: the whole level, `rows` rows of
    /// `row` bytes, packed.
    UploadSurface { surface: Handle, row: u32, rows: u32, data: payload },
    /// The game unlocked a vertex or index buffer: `data` the bytes from `offset`; `flags` the
    /// game's lock flags (`D3DLOCK_DISCARD`, `D3DLOCK_NOOVERWRITE`, ...).
    UploadBuffer { buffer: Handle, offset: u32, flags: u32, data: payload },
    /// `SetLOD(lod)` of a managed texture; `model` the previous level the model answered.
    SetLod { texture: Handle, lod: u32, model: u32 },
    /// `IDirect3DQuery9::Issue(flags)`: `D3DISSUE_END` 1, `D3DISSUE_BEGIN` 2.
    QueryIssue { query: Handle, flags: u32 },

    // Sync: the caller waits for the reply, which comes after every command recorded before.

    /// (sync) The bytes of a level or surface as the backend keeps them: `size` bytes, rows
    /// packed. Replies [`Reply::Bytes`]: none when the backend has no such level or its size
    /// differs.
    ReadSurface { surface: Handle, size: u32 },
    /// (sync) `GetRenderTargetData(source, destination)`; then, when `pitch` is not 0, the
    /// destination's rows read back. Replies [`Reply::Rows`]: none without backend surfaces.
    GetRenderTargetData { device: Handle, source: Handle, destination: Handle, pitch: u32 },
    /// (sync) `IDirect3DQuery9::GetData(flags)` after the game's `D3DISSUE_END`. Replies
    /// [`Reply::Query`].
    QueryGetData { query: Handle, flags: u32 },
}

impl Cmd<Data<'_>> {
    /// Whether each payload has the size the call gives it (a structure's, `count` registers' or
    /// rectangles', a level's rows, whole handles or elements): checked on every command read
    /// from a file, and on every command submitted in a debug build.
    pub fn check(&self) -> Result<(), String> {
        let name = self.name();
        let exact = |what: &str, d: &Data<'_>, want: u64| {
            if u64::from(d.len()) == want {
                Ok(())
            } else {
                Err(format!("{name}: {what} is {} bytes, not {want}", d.len()))
            }
        };
        let whole = |what: &str, d: &Data<'_>, unit: u32| {
            if d.len() % unit == 0 {
                Ok(())
            } else {
                Err(format!(
                    "{name}: {what} is {} bytes, not a multiple of {unit}",
                    d.len()
                ))
            }
        };
        let rect =
            |what: &str, d: &Option<Data<'_>>| d.as_ref().map_or(Ok(()), |d| exact(what, d, 16));
        match self {
            Cmd::CreateDevice {
                present,
                back_buffers,
                ..
            }
            | Cmd::Reset {
                present,
                back_buffers,
                ..
            } => {
                exact("present", present, 56)?;
                whole("back_buffers", back_buffers, 4)
            }
            Cmd::Present { source, dest, .. } => {
                rect("source", source)?;
                rect("dest", dest)
            }
            Cmd::Clear { count, rects, .. } => rects
                .as_ref()
                .map_or(Ok(()), |r| exact("rects", r, 16 * u64::from(*count))),
            Cmd::SetTransform { matrix, .. } => exact("matrix", matrix, 64),
            Cmd::SetGammaRamp { ramp, .. } => exact("ramp", ramp, 0x600),
            Cmd::SetConstants {
                kind, count, data, ..
            } => {
                if *kind > 5 {
                    return Err(format!("{name}: kind {kind}"));
                }
                let per = if kind % 3 == 2 { 4 } else { 16 };
                exact("data", data, per * u64::from(*count))
            }
            Cmd::DrawPrimitiveUp {
                kind,
                count,
                stride,
                vertices,
                ..
            } => {
                let c = u64::from(*count);
                let n = match kind {
                    1 => c,
                    2 => 2 * c,
                    3 => c + 1,
                    4 => 3 * c,
                    5 | 6 => c + 2,
                    _ => return Err(format!("{name}: primitive type {kind}")),
                };
                exact("vertices", vertices, n * u64::from(*stride))
            }
            Cmd::SetMaterial { material, .. } => exact("material", material, 68),
            Cmd::SetLight { light, .. } => exact("light", light, 104),
            Cmd::StretchRect {
                source_rect,
                destination_rect,
                ..
            } => {
                rect("source_rect", source_rect)?;
                rect("destination_rect", destination_rect)
            }
            Cmd::ColorFill { rect: r, .. } => rect("rect", r),
            Cmd::CreateTexture { levels, .. } => whole("levels", levels, 4),
            Cmd::CreateCubeTexture { surfaces, .. } => whole("surfaces", surfaces, 24),
            Cmd::CreateShader { tokens, .. } => whole("tokens", tokens, 4),
            Cmd::CreateDeclaration { elements, .. } => whole("elements", elements, 8),
            Cmd::UploadSurface {
                row, rows, data, ..
            } => exact("data", data, u64::from(*row) * u64::from(*rows)),
            _ => Ok(()),
        }
    }
}

impl<P> Cmd<P> {
    /// Whether the caller waits for the command's [`Reply`].
    pub fn is_sync(&self) -> bool {
        matches!(
            self,
            Cmd::CreateDevice { .. }
                | Cmd::ReadSurface { .. }
                | Cmd::GetRenderTargetData { .. }
                | Cmd::QueryGetData { .. }
        )
    }
}

/// What a command answers: [`Reply::None`] for every command that is not sync.
#[derive(Debug)]
pub enum Reply {
    None,
    /// [`Cmd::CreateDevice`]: whether the backend has a device now, and what it hands to the
    /// layer's side channels (the real backend: its device, for D3DX's texture loading).
    Device {
        created: bool,
        native: Option<Native>,
    },
    /// [`Cmd::ReadSurface`]: the bytes, rows packed; none without them.
    Bytes(Option<Vec<u8>>),
    /// [`Cmd::GetRenderTargetData`]: none without backend surfaces.
    Rows(Option<Rows>),
    /// [`Cmd::QueryGetData`]: the backend's answer (`S_OK` 0, `S_FALSE` 1, ...) and the 4-byte
    /// result; none without a backend query.
    Query(Option<(u32, u32)>),
}

/// Rows read back: each `row` bytes, packed (none when the command asked for none).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rows {
    pub row: u32,
    pub bytes: Vec<u8>,
}

/// Something a backend hands to the layer that made the stream, opaque here.
pub struct Native(pub Box<dyn std::any::Any + Send>);

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Native(..)")
    }
}

/// What carries the commands out: the real `d3d9.dll` (engine-platform), a wgpu layer, a log.
///
/// A backend gets the commands in the order the game made the calls, across all the game's
/// threads, on one thread at a time. It never reads the game's memory or the engine: everything is
/// in the command (a [`Data::Read`] reads the game's memory for it, on the calling thread, during
/// the call). A command naming a handle the backend has no object for (its creation failed) is
/// skipped. A level the backend keeps (managed or system-memory pool) must answer
/// [`Cmd::ReadSurface`] with the bytes it was last given, even when its own object failed.
pub trait Backend: Send {
    /// Carries out `cmd`; a sync command's answer is its reply, any other's [`Reply::None`].
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply;
}

impl<B: Backend + ?Sized> Backend for Box<B> {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        (**self).execute(cmd)
    }
}
