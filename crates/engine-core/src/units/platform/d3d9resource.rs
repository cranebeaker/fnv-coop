//! The platform's Direct3D 9 buffers, shaders and vertex declarations (ADR-0007), made by the
//! device of `super::d3d9device`: `CreateVertexBuffer`, `CreateIndexBuffer`,
//! `CreateVertexShader`, `CreatePixelShader`, `CreateVertexDeclaration`, the objects they make,
//! and the device state that binds them (stream sources, indices, the vertex declaration and
//! FVF, the shaders and their constants), with the documented Direct3D 9 semantics, and the draws
//! (`DrawPrimitiveUP` as the retail runtime answered it, measured with `scripts/d3d9-capture
//! device`).
//!
//! Each object lives in engine memory with its vtable pointer at `+0`, each provided slot a
//! platform function at a code address of its kind (see the `*_CODE` constants); a slot not
//! provided stops the run as an open function. Buffers keep their contents in the game's
//! address space (what `Lock` hands out); shaders and declarations keep the bytes the program
//! gave. Nothing is drawn yet.

use super::d3d9::{CODE_BASE, D3DERR_INVALIDCALL, D3D_OK, E_NOINTERFACE};
use super::d3d9device::{
    device_add_ref, device_release, forwarded, guid, iid_is, in_scene, Word, IID_IUNKNOWN,
};
use crate::{Engine, Ptr};
use std::collections::HashMap;

/// Where each kind's slots sit.
pub const VERTEX_BUFFER_CODE: u32 = CODE_BASE + 0x0600;
pub const INDEX_BUFFER_CODE: u32 = CODE_BASE + 0x0680;
pub const VERTEX_SHADER_CODE: u32 = CODE_BASE + 0x0700;
pub const PIXEL_SHADER_CODE: u32 = CODE_BASE + 0x0740;
pub const DECLARATION_CODE: u32 = CODE_BASE + 0x0780;

/// `IDirect3DVertexBuffer9`'s and `IDirect3DIndexBuffer9`'s slots (d3d9.h order).
pub const BUFFER_SLOTS: [&str; 14] = [
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
    "Lock",
    "Unlock",
    "GetDesc",
];
/// `IDirect3DVertexShader9`'s and `IDirect3DPixelShader9`'s slots; a declaration's fifth is
/// `GetDeclaration`.
pub const SHADER_SLOTS: [&str; 5] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetDevice",
    "GetFunction",
];

const IID_RESOURCE9: [u8; 16] = guid(
    0x05ee_c05d,
    0x8f7d,
    0x4362,
    [0xb9, 0x99, 0xd1, 0xba, 0xf3, 0x57, 0xc7, 0x04],
);
const IID_VERTEX_BUFFER9: [u8; 16] = guid(
    0xb64b_b1b5,
    0xfd70,
    0x4df6,
    [0xbf, 0x91, 0x19, 0xd0, 0xa1, 0x24, 0x55, 0xe3],
);
const IID_INDEX_BUFFER9: [u8; 16] = guid(
    0x7c9d_d65e,
    0xd3f7,
    0x4529,
    [0xac, 0xee, 0x78, 0x58, 0x30, 0xac, 0xde, 0x35],
);
const IID_VERTEX_SHADER9: [u8; 16] = guid(
    0xefc5_557e,
    0x6265,
    0x4613,
    [0x8a, 0x94, 0x43, 0x85, 0x78, 0x89, 0xeb, 0x36],
);
const IID_PIXEL_SHADER9: [u8; 16] = guid(
    0x6d3b_dbdc,
    0x5b02,
    0x4415,
    [0xb8, 0x52, 0xce, 0x5e, 0x8b, 0xcc, 0xb2, 0x89],
);
const IID_DECLARATION9: [u8; 16] = guid(
    0xdd13_c59c,
    0x36fa,
    0x4098,
    [0xa8, 0xfb, 0xc7, 0xed, 0x39, 0xdc, 0x85, 0x46],
);

/// `D3DFMT_VERTEXDATA`, `D3DFMT_INDEX16`, `D3DFMT_INDEX32`, `D3DRTYPE_VERTEXBUFFER`,
/// `D3DRTYPE_INDEXBUFFER`, `D3DPOOL_MANAGED`, `D3DUSAGE_DYNAMIC`.
const D3DFMT_VERTEXDATA: u32 = 100;
const D3DFMT_INDEX16: u32 = 101;
const D3DFMT_INDEX32: u32 = 102;
const D3DRTYPE_VERTEXBUFFER: u32 = 6;
const D3DRTYPE_INDEXBUFFER: u32 = 7;
const D3DPOOL_MANAGED: u32 = 1;
const D3DUSAGE_DYNAMIC: u32 = 0x200;
/// The token that ends a shader's function (`D3DPS_END`/`D3DVS_END`) and the stream number of
/// `D3DDECL_END`.
const SHADER_END: u32 = 0x0000_ffff;
const DECL_END_STREAM: u16 = 0xff;
/// The stream count the device takes (`MaxStreams` is at most 16) and the constant register
/// counts kept: float registers (vertex 256, pixel 224 in shader model 3), integer and boolean
/// registers (16 each).
const STREAMS: u32 = 16;
const VERTEX_FLOAT_REGISTERS: u32 = 256;
const PIXEL_FLOAT_REGISTERS: u32 = 224;
const INT_REGISTERS: u32 = 16;
const BOOL_REGISTERS: u32 = 16;
/// Register slots kept per constant kind (the most any kind has).
const CONSTANT_SLOTS: usize = 256;

/// What an object of this module is.
#[derive(Debug, Clone)]
enum Kind {
    /// A vertex or index buffer: its description words (vertex: Format, Type, Usage, Pool,
    /// Size, FVF; index: the first five), its memory and the open locks.
    Buffer {
        desc: Vec<u32>,
        bits: u32,
        locks: u32,
    },
    /// A shader's tokens (vertex or pixel by its code base).
    Shader { tokens: Vec<u8> },
    /// A vertex declaration's elements, `D3DDECL_END` included.
    Declaration { elements: Vec<u8> },
}

#[derive(Debug, Clone)]
struct Object {
    /// The program's references (what `AddRef` and `Release` answer).
    references: u32,
    /// The device's bindings of it (stream sources, indices, declaration, shaders): kept apart
    /// from the program's count, as the retail runtime keeps them (`d3d9-capture device refs`,
    /// 2026-10-27: binding leaves `AddRef`/`Release` unchanged, a `Release` to 0 while bound
    /// answers 0 and the object lives on until unbound).
    bound: u32,
    device: u32,
    code: u32,
    kind: Kind,
}

/// The state a device binds (per device).
#[derive(Debug, Clone, Default)]
struct Bindings {
    /// Stream sources: (buffer, offset, stride), held.
    streams: crate::engine::AddrMap<u32, (u32, u32, u32)>,
    indices: u32,
    declaration: u32,
    fvf: u32,
    vertex_shader: u32,
    pixel_shader: u32,
    /// Constant registers as their 4 words each at `kind * CONSTANT_SLOTS + register`: kinds
    /// 0..2 vertex float, int, bool; 3..5 pixel float, int, bool (a bool register keeps one
    /// word); `constants_set` has a bit for each register set. Both empty until the first set.
    constants: Vec<[u32; 4]>,
    constants_set: Vec<u64>,
    /// The stride `GetStreamSource` reports for a stream that holds no buffer (0 unless a
    /// failed `DrawPrimitiveUP` left one, measured).
    empty_strides: crate::engine::AddrMap<u32, u32>,
    /// Whether a `DrawPrimitiveUP` has succeeded in a scene on this device (from then on the
    /// runtime takes them outside a scene too, measured).
    drawn_from_memory: bool,
}

/// The buffers, shaders and declarations (`Kernel::graphics`).
#[derive(Debug, Default)]
pub struct Resources {
    vtables: HashMap<u32, u32>,
    objects: crate::engine::AddrMap<u32, Object>,
    bindings: crate::engine::AddrMap<u32, Bindings>,
}

fn resources(e: &mut Engine) -> &mut Resources {
    &mut e.kernel.graphics.device_objects.resources
}

fn object(e: &mut Engine, this: Ptr) -> &mut Object {
    resources(e)
        .objects
        .get_mut(&this.addr())
        .expect("a Direct3D 9 buffer, shader or declaration of this layer")
}

fn bindings(e: &mut Engine, device: Ptr) -> &mut Bindings {
    resources(e).bindings.entry(device.addr()).or_default()
}

/// Registers this module's slots and makes its vtables (once).
pub(super) fn register_code(e: &mut Engine) {
    let buffer: [(u32, crate::AbiFn); 9] = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (3, crate::entry!(0, get_device(Ptr, Ptr) -> u32).1),
        (9, crate::entry!(0, pre_load(Ptr)).1),
        (10, crate::entry!(0, get_type(Ptr) -> u32).1),
        (11, crate::entry!(0, lock(Ptr, u32, u32, Ptr, u32) -> u32).1),
        (12, crate::entry!(0, unlock(Ptr) -> u32).1),
        (13, crate::entry!(0, buffer_get_desc(Ptr, Ptr) -> u32).1),
    ];
    for base in [VERTEX_BUFFER_CODE, INDEX_BUFFER_CODE] {
        for (slot, f) in buffer {
            e.register(base + 4 * slot, f);
        }
    }
    let shader: [(u32, crate::AbiFn); 5] = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (3, crate::entry!(0, get_device(Ptr, Ptr) -> u32).1),
        (4, crate::entry!(0, get_bytes(Ptr, Ptr, Ptr) -> u32).1),
    ];
    for base in [VERTEX_SHADER_CODE, PIXEL_SHADER_CODE, DECLARATION_CODE] {
        for (slot, f) in shader {
            e.register(base + 4 * slot, f);
        }
    }
    if resources(e).vtables.is_empty() {
        for (base, slots) in [
            (VERTEX_BUFFER_CODE, BUFFER_SLOTS.len()),
            (INDEX_BUFFER_CODE, BUFFER_SLOTS.len()),
            (VERTEX_SHADER_CODE, SHADER_SLOTS.len()),
            (PIXEL_SHADER_CODE, SHADER_SLOTS.len()),
            (DECLARATION_CODE, SHADER_SLOTS.len()),
        ] {
            let table = e.mem.alloc(4 * slots as u32);
            for i in 0..slots as u32 {
                e.mem.set_u32(table + 4 * i, base + 4 * i);
            }
            resources(e).vtables.insert(base, table);
        }
    }
}

/// Makes an object of `code`'s kind for `device` (counted once; the device counted too).
fn make(e: &mut Engine, device: Ptr, code: u32, kind: Kind) -> u32 {
    register_code(e);
    let object = e.mem.alloc(8);
    let table = resources(e).vtables[&code];
    e.mem.set_u32(object, table);
    resources(e).objects.insert(
        object,
        Object {
            references: 1,
            bound: 0,
            device: device.addr(),
            code,
            kind,
        },
    );
    device_add_ref(e, device);
    object
}

/// Whether `pool` is a `D3DPOOL` value (default, managed, system memory, scratch).
fn is_pool(pool: u32) -> bool {
    pool <= 3
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateVertexBuffer (vtable +0x68)
/// `CreateVertexBuffer(length, usage, FVF, pool, out, shared handle)`: a buffer of `length`
/// bytes (zero-filled) in the game's address space. A zero length, a null out, a shared handle
/// (Direct3D 9Ex only), a pool that is no `D3DPOOL` or `D3DUSAGE_DYNAMIC` in `D3DPOOL_MANAGED`
/// answer `D3DERR_INVALIDCALL`.
pub(super) fn create_vertex_buffer(
    e: &mut Engine,
    this: Ptr,
    length: u32,
    usage: u32,
    fvf: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if length == 0 || out.is_null() || shared != 0 || !is_pool(pool) {
        return D3DERR_INVALIDCALL;
    }
    if usage & D3DUSAGE_DYNAMIC != 0 && pool == D3DPOOL_MANAGED {
        return D3DERR_INVALIDCALL;
    }
    let bits = e.mem.alloc(length);
    let desc = vec![
        D3DFMT_VERTEXDATA,
        D3DRTYPE_VERTEXBUFFER,
        usage,
        pool,
        length,
        fvf,
    ];
    let buffer = make(
        e,
        this,
        VERTEX_BUFFER_CODE,
        Kind::Buffer {
            desc,
            bits,
            locks: 0,
        },
    );
    e.mem.set_u32(out.addr(), buffer);
    #[cfg(windows)]
    super::d3d9real::buffer_created(e, this.addr(), buffer, false, length, usage, fvf, pool);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateIndexBuffer (vtable +0x6C)
/// `CreateIndexBuffer(length, usage, format, pool, out, shared handle)`: as
/// [`create_vertex_buffer`]; the format must be `D3DFMT_INDEX16` or `D3DFMT_INDEX32`.
pub(super) fn create_index_buffer(
    e: &mut Engine,
    this: Ptr,
    length: u32,
    usage: u32,
    format: u32,
    pool: u32,
    out: Ptr,
    shared: u32,
) -> u32 {
    if length == 0 || out.is_null() || shared != 0 || !is_pool(pool) {
        return D3DERR_INVALIDCALL;
    }
    if format != D3DFMT_INDEX16 && format != D3DFMT_INDEX32 {
        return D3DERR_INVALIDCALL;
    }
    if usage & D3DUSAGE_DYNAMIC != 0 && pool == D3DPOOL_MANAGED {
        return D3DERR_INVALIDCALL;
    }
    let bits = e.mem.alloc(length);
    let desc = vec![format, D3DRTYPE_INDEXBUFFER, usage, pool, length];
    let buffer = make(
        e,
        this,
        INDEX_BUFFER_CODE,
        Kind::Buffer {
            desc,
            bits,
            locks: 0,
        },
    );
    e.mem.set_u32(out.addr(), buffer);
    #[cfg(windows)]
    super::d3d9real::buffer_created(e, this.addr(), buffer, true, length, usage, format, pool);
    D3D_OK
}

/// A shader function's bytes: its tokens up to and including the end token.
fn shader_tokens(e: &Engine, function: u32) -> Vec<u8> {
    let mut at = function;
    loop {
        let token = e.mem.u32(at);
        at += 4;
        if token == SHADER_END {
            break;
        }
    }
    e.mem.bytes(function, at - function)
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateVertexShader (vtable +0x16C)
/// `CreateVertexShader(function, out)`: a shader holding the function's tokens (up to the end
/// token). A null function or out answers `D3DERR_INVALIDCALL`. The runtime's validation of
/// the tokens is not reproduced: the game's shaders are the ones its own packages hold.
pub(super) fn create_vertex_shader(e: &mut Engine, this: Ptr, function: Ptr, out: Ptr) -> u32 {
    if function.is_null() || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let tokens = shader_tokens(e, function.addr());
    let shader = make(e, this, VERTEX_SHADER_CODE, Kind::Shader { tokens });
    e.mem.set_u32(out.addr(), shader);
    #[cfg(windows)]
    if let Kind::Shader { tokens } = &object(e, Ptr::new(shader)).kind.clone() {
        super::d3d9real::shader_created(e, this.addr(), shader, false, tokens);
    }
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreatePixelShader (vtable +0x1A8)
/// `CreatePixelShader(function, out)`: as [`create_vertex_shader`].
pub(super) fn create_pixel_shader(e: &mut Engine, this: Ptr, function: Ptr, out: Ptr) -> u32 {
    if function.is_null() || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let tokens = shader_tokens(e, function.addr());
    let shader = make(e, this, PIXEL_SHADER_CODE, Kind::Shader { tokens });
    e.mem.set_u32(out.addr(), shader);
    #[cfg(windows)]
    if let Kind::Shader { tokens } = &object(e, Ptr::new(shader)).kind.clone() {
        super::d3d9real::shader_created(e, this.addr(), shader, true, tokens);
    }
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateVertexDeclaration (vtable +0x158)
/// `CreateVertexDeclaration(elements, out)`: a declaration holding the 8-byte
/// `D3DVERTEXELEMENT9`s up to and including `D3DDECL_END` (stream 0xFF). A null pointer
/// answers `D3DERR_INVALIDCALL`.
pub(super) fn create_vertex_declaration(e: &mut Engine, this: Ptr, elements: Ptr, out: Ptr) -> u32 {
    if elements.is_null() || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let mut at = elements.addr();
    loop {
        let stream = e.mem.u16(at);
        at += 8;
        if stream == DECL_END_STREAM {
            break;
        }
    }
    let bytes = e.mem.bytes(elements.addr(), at - elements.addr());
    let declaration = make(
        e,
        this,
        DECLARATION_CODE,
        Kind::Declaration { elements: bytes },
    );
    e.mem.set_u32(out.addr(), declaration);
    #[cfg(windows)]
    if let Kind::Declaration { elements } = &object(e, Ptr::new(declaration)).kind.clone() {
        super::d3d9real::declaration_created(e, this.addr(), declaration, elements);
    }
    D3D_OK
}

/// `QueryInterface(iid, out)`: `IUnknown`, and for buffers `IDirect3DResource9` and their own
/// interface, for shaders and declarations their own: the object, counted; others
/// `E_NOINTERFACE` with a null out.
fn query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let ids: &[[u8; 16]] = match object(e, this).code {
        VERTEX_BUFFER_CODE => &[IID_IUNKNOWN, IID_RESOURCE9, IID_VERTEX_BUFFER9],
        INDEX_BUFFER_CODE => &[IID_IUNKNOWN, IID_RESOURCE9, IID_INDEX_BUFFER9],
        VERTEX_SHADER_CODE => &[IID_IUNKNOWN, IID_VERTEX_SHADER9],
        PIXEL_SHADER_CODE => &[IID_IUNKNOWN, IID_PIXEL_SHADER9],
        _ => &[IID_IUNKNOWN, IID_DECLARATION9],
    };
    if iid_is(e, iid, ids) {
        add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let o = object(e, this);
    o.references += 1;
    o.references
}

/// `Release()`: the last reference frees the object (a buffer's memory too) and releases the
/// device's reference it held.
fn release(e: &mut Engine, this: Ptr) -> u32 {
    let o = object(e, this);
    o.references -= 1;
    let (left, bound) = (o.references, o.bound);
    if left == 0 && bound == 0 {
        destroy(e, this);
    }
    left
}

/// The device lets go of a binding of `this`; the object goes once neither the program nor the
/// device holds it.
fn unbind(e: &mut Engine, this: Ptr) {
    let o = object(e, this);
    o.bound -= 1;
    if o.bound == 0 && o.references == 0 {
        destroy(e, this);
    }
}

fn destroy(e: &mut Engine, this: Ptr) {
    {
        let o = resources(e)
            .objects
            .remove(&this.addr())
            .expect("the object");
        if let Kind::Buffer { bits, .. } = o.kind {
            e.mem.free(bits);
        }
        #[cfg(windows)]
        super::d3d9real::forget(e, this.addr());
        e.mem.free(this.addr());
        device_release(e, Ptr::new(o.device));
    }
}

/// `GetDevice(out)`: the object's device, counted.
fn get_device(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let owner = object(e, this).device;
    device_add_ref(e, Ptr::new(owner));
    e.mem.set_u32(out.addr(), owner);
    D3D_OK
}

/// `PreLoad()`: a hint about managed resources; nothing to do here.
fn pre_load(_e: &mut Engine, _this: Ptr) {}

/// `GetType()`: `D3DRTYPE_VERTEXBUFFER` or `D3DRTYPE_INDEXBUFFER`.
fn get_type(e: &mut Engine, this: Ptr) -> u32 {
    match object(e, this).code {
        VERTEX_BUFFER_CODE => D3DRTYPE_VERTEXBUFFER,
        _ => D3DRTYPE_INDEXBUFFER,
    }
}

/// `Lock(offset, size, data out, flags)`: the address of byte `offset` of the buffer's memory;
/// size 0 (with offset 0) locks the whole buffer. A range past the end or a null out answers
/// `D3DERR_INVALIDCALL`. Buffers may be locked more than once; each lock needs its `Unlock`.
/// The flags (`D3DLOCK_DISCARD`, `D3DLOCK_NOOVERWRITE`, ...) change nothing here: the memory is
/// the buffer's and stays.
fn lock(e: &mut Engine, this: Ptr, offset: u32, size: u32, out: Ptr, flags: u32) -> u32 {
    let model = lock_model(e, this, offset, size, out);
    #[cfg(windows)]
    super::d3d9real::buffer_locked(e, this.addr(), offset, size, flags, model);
    let _ = flags;
    model
}

fn lock_model(e: &mut Engine, this: Ptr, offset: u32, size: u32, out: Ptr) -> u32 {
    let o = object(e, this);
    let Kind::Buffer { desc, bits, locks } = &mut o.kind else {
        unreachable!("Lock is a buffer's slot")
    };
    let length = desc[4];
    if out.is_null() || u64::from(offset) + u64::from(size) > u64::from(length) {
        return D3DERR_INVALIDCALL;
    }
    *locks += 1;
    let at = *bits + offset;
    e.mem.set_u32(out.addr(), at);
    D3D_OK
}

/// `Unlock()`: closes a lock; a buffer not locked answers `D3DERR_INVALIDCALL`.
fn unlock(e: &mut Engine, this: Ptr) -> u32 {
    let o = object(e, this);
    let Kind::Buffer { locks, bits, desc } = &mut o.kind else {
        unreachable!("Unlock is a buffer's slot")
    };
    if *locks == 0 {
        return D3DERR_INVALIDCALL;
    }
    *locks -= 1;
    let (bits, length) = (*bits, desc[4]);
    #[cfg(windows)]
    super::d3d9real::buffer_unlocked(e, this.addr(), bits, length, D3D_OK);
    let _ = (bits, length);
    D3D_OK
}

/// `GetDesc(out)`: `D3DVERTEXBUFFER_DESC` (6 words) or `D3DINDEXBUFFER_DESC` (5 words).
fn buffer_get_desc(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let Kind::Buffer { desc, .. } = object(e, this).kind.clone() else {
        unreachable!("GetDesc is a buffer's slot")
    };
    for (i, word) in desc.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
    D3D_OK
}

/// `GetFunction(data, size)` / `GetDeclaration(elements, count)`: with a null data pointer the
/// size (in bytes; for a declaration the element count) is written; otherwise the bytes are
/// copied when the size given is enough, else `D3DERR_INVALIDCALL`.
fn get_bytes(e: &mut Engine, this: Ptr, data: Ptr, size: Ptr) -> u32 {
    if size.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let (bytes, unit) = match object(e, this).kind.clone() {
        Kind::Shader { tokens } => (tokens, 1),
        Kind::Declaration { elements } => (elements, 8),
        Kind::Buffer { .. } => unreachable!("a shader's or declaration's slot"),
    };
    let count = bytes.len() as u32 / unit;
    if data.is_null() {
        e.mem.set_u32(size.addr(), count);
        return D3D_OK;
    }
    if e.mem.u32(size.addr()) < count {
        return D3DERR_INVALIDCALL;
    }
    e.mem.write(data.addr(), &bytes);
    D3D_OK
}

/// Replaces a held object word: the new one counted, the old one released. The same object
/// again changes nothing (counted once more and released once, it stays).
fn hold(e: &mut Engine, old: u32, new: u32) {
    if old == new {
        return;
    }
    if new != 0 {
        object(e, Ptr::new(new)).bound += 1;
    }
    if old != 0 {
        unbind(e, Ptr::new(old));
    }
}

/// The object `new` (0 for none) when it is one of this module's objects of kind `code`
/// (`Err` when it is not), looked up once for both the check and the count [`hold`] adds.
fn checked(
    objects: &mut crate::engine::AddrMap<u32, Object>,
    new: u32,
    code: u32,
) -> Result<Option<&mut Object>, ()> {
    if new == 0 {
        return Ok(None);
    }
    match objects.get_mut(&new) {
        Some(o) if o.code == code => Ok(Some(o)),
        _ => Err(()),
    }
}

/// The object words of the bindings [`bind`] replaces.
#[derive(Clone, Copy)]
enum Held {
    Indices,
    Declaration,
    VertexShader,
    PixelShader,
}

/// Binds `new` (0 for none) in the bindings' word `which`: an object that is not of kind `code`
/// answers `D3DERR_INVALIDCALL`; else the word is replaced and [`hold`] counts the new object
/// and releases the old one, with one lookup of the new object and of the bindings.
fn bind(e: &mut Engine, this: Ptr, new: u32, code: u32, which: Held) -> u32 {
    let Resources {
        objects, bindings, ..
    } = resources(e);
    let Ok(held) = checked(objects, new, code) else {
        return D3DERR_INVALIDCALL;
    };
    let b = bindings.entry(this.addr()).or_default();
    let word = match which {
        Held::Indices => &mut b.indices,
        Held::Declaration => &mut b.declaration,
        Held::VertexShader => &mut b.vertex_shader,
        Held::PixelShader => &mut b.pixel_shader,
    };
    let old = std::mem::replace(word, new);
    if old != new {
        if let Some(o) = held {
            o.bound += 1;
        }
        if old != 0 {
            unbind(e, Ptr::new(old));
        }
    }
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetStreamSource (vtable +0x190)
/// `SetStreamSource(stream, buffer, offset, stride)`: the device holds the vertex buffer
/// (counted) with its offset and stride for the stream, releasing the one it held; null clears
/// the stream. A stream past 15 or an object that is no vertex buffer answers
/// `D3DERR_INVALIDCALL`.
pub(super) fn set_stream_source(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    buffer: u32,
    offset: u32,
    stride: u32,
) -> u32 {
    if stream >= STREAMS {
        return D3DERR_INVALIDCALL;
    }
    let Resources {
        objects, bindings, ..
    } = resources(e);
    let Ok(held) = checked(objects, buffer, VERTEX_BUFFER_CODE) else {
        return D3DERR_INVALIDCALL;
    };
    let b = bindings.entry(this.addr()).or_default();
    if !b.empty_strides.is_empty() {
        b.empty_strides.remove(&stream);
    }
    let old = if buffer != 0 {
        b.streams.insert(stream, (buffer, offset, stride))
    } else {
        b.streams.remove(&stream)
    }
    .map_or(0, |o| o.0);
    // As [`hold`], with the buffer looked up once.
    if old != buffer {
        if let Some(o) = held {
            o.bound += 1;
        }
        if old != 0 {
            unbind(e, Ptr::new(old));
        }
    }
    D3D_OK
}

/// `GetStreamSource(stream, buffer out, offset out, stride out)`: what the stream holds (the
/// buffer counted), or null and zeros.
pub(super) fn get_stream_source(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    buffer: Ptr,
    offset: Ptr,
    stride: Ptr,
) -> u32 {
    if stream >= STREAMS || buffer.is_null() || offset.is_null() || stride.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let (held, at, step) = bindings(e, this)
        .streams
        .get(&stream)
        .copied()
        .unwrap_or((0, 0, 0));
    if held != 0 {
        add_ref(e, Ptr::new(held));
    }
    let step = if held == 0 {
        bindings(e, this)
            .empty_strides
            .get(&stream)
            .copied()
            .unwrap_or(step)
    } else {
        step
    };
    e.mem.set_u32(buffer.addr(), held);
    e.mem.set_u32(offset.addr(), at);
    e.mem.set_u32(stride.addr(), step);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetIndices (vtable +0x1A0)
/// `SetIndices(buffer)`: the device holds the index buffer (counted), releasing the one it
/// held; an object that is no index buffer answers `D3DERR_INVALIDCALL`.
pub(super) fn set_indices(e: &mut Engine, this: Ptr, buffer: u32) -> u32 {
    bind(e, this, buffer, INDEX_BUFFER_CODE, Held::Indices)
}

/// `GetIndices(out)`: the index buffer held (counted), or null.
pub(super) fn get_indices(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let held = bindings(e, this).indices;
    if held != 0 {
        add_ref(e, Ptr::new(held));
    }
    e.mem.set_u32(out.addr(), held);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetVertexDeclaration (vtable +0x15C)
/// `SetVertexDeclaration(declaration)`: held (counted), the old one released; another kind of
/// object answers `D3DERR_INVALIDCALL`.
pub(super) fn set_vertex_declaration(e: &mut Engine, this: Ptr, declaration: u32) -> u32 {
    bind(e, this, declaration, DECLARATION_CODE, Held::Declaration)
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::GetVertexDeclaration (vtable +0x160)
/// `GetVertexDeclaration(out)`: the declaration held (counted), or null.
pub(super) fn get_vertex_declaration(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let held = bindings(e, this).declaration;
    if held != 0 {
        add_ref(e, Ptr::new(held));
    }
    e.mem.set_u32(out.addr(), held);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetFVF (vtable +0x164)
/// `SetFVF(fvf)`: kept. (Direct3D replaces the declaration by one made from the FVF; that
/// implicit declaration is not modelled yet, so a `GetVertexDeclaration` after it is not
/// provided.)
pub(super) fn set_fvf(e: &mut Engine, this: Ptr, fvf: u32) -> u32 {
    bindings(e, this).fvf = fvf;
    D3D_OK
}

/// `GetFVF(out)`: the FVF set (0 until one is).
pub(super) fn get_fvf(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let fvf = bindings(e, this).fvf;
    e.mem.set_u32(out.addr(), fvf);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetVertexShader (vtable +0x170)
/// `SetVertexShader(shader)`: held (counted), the old one released; null is the fixed-function
/// pipeline; another kind of object answers `D3DERR_INVALIDCALL`.
pub(super) fn set_vertex_shader(e: &mut Engine, this: Ptr, shader: u32) -> u32 {
    bind(e, this, shader, VERTEX_SHADER_CODE, Held::VertexShader)
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::SetPixelShader (vtable +0x1AC)
/// `SetPixelShader(shader)`: as [`set_vertex_shader`].
pub(super) fn set_pixel_shader(e: &mut Engine, this: Ptr, shader: u32) -> u32 {
    bind(e, this, shader, PIXEL_SHADER_CODE, Held::PixelShader)
}

/// `GetVertexShader(out)` / `GetPixelShader(out)` (by `pixel`): the shader held (counted), or
/// null.
fn get_shader(e: &mut Engine, this: Ptr, out: Ptr, pixel: bool) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let b = bindings(e, this);
    let held = if pixel {
        b.pixel_shader
    } else {
        b.vertex_shader
    };
    if held != 0 {
        add_ref(e, Ptr::new(held));
    }
    e.mem.set_u32(out.addr(), held);
    D3D_OK
}

pub(super) fn get_vertex_shader(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    get_shader(e, this, out, false)
}

pub(super) fn get_pixel_shader(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    get_shader(e, this, out, true)
}

/// The register count kept for a constant kind (0..2 vertex float, int, bool; 3..5 pixel).
fn registers(kind: u8) -> u32 {
    match kind {
        0 => VERTEX_FLOAT_REGISTERS,
        3 => PIXEL_FLOAT_REGISTERS,
        1 | 4 => INT_REGISTERS,
        _ => BOOL_REGISTERS,
    }
}

/// `Set*ShaderConstant{F,I,B}(start, data, count)`: the registers `start..start + count` take
/// 4 words each (one for booleans) from `data`; a range past the registers or a null `data`
/// answers `D3DERR_INVALIDCALL`.
fn set_constants(e: &mut Engine, this: Ptr, kind: u8, start: u32, data: Ptr, count: u32) -> u32 {
    if data.is_null() || u64::from(start) + u64::from(count) > u64::from(registers(kind)) {
        return D3DERR_INVALIDCALL;
    }
    let words = if kind % 3 == 2 { 1 } else { 4 };
    let first = usize::from(kind) * CONSTANT_SLOTS + start as usize;
    // The range is at most 4 KB (256 registers of 4 words), so it touches at most two pages:
    // when both are mapped no read can fault, and the registers are read straight into the
    // device's bindings (no copy of the range, no 4 KB buffer to clear per call).
    let last = data
        .addr()
        .checked_add((4 * words * count).saturating_sub(1));
    if last.is_some_and(|last| e.mem.is_mapped(data.addr()) && e.mem.is_mapped(last)) {
        let mem = &e.mem;
        let b = e
            .kernel
            .graphics
            .device_objects
            .resources
            .bindings
            .entry(this.addr())
            .or_default();
        if b.constants.is_empty() {
            b.constants = vec![[0; 4]; 6 * CONSTANT_SLOTS];
            b.constants_set = vec![0; 6 * CONSTANT_SLOTS / 64];
        }
        let at = data.addr();
        let registers = &mut b.constants[first..first + count as usize];
        if words == 4 {
            for (i, value) in registers.iter_mut().enumerate() {
                let at = at + 16 * i as u32;
                *value = [
                    mem.u32(at),
                    mem.u32(at + 4),
                    mem.u32(at + 8),
                    mem.u32(at + 12),
                ];
            }
        } else {
            for (i, value) in registers.iter_mut().enumerate() {
                *value = [mem.u32(at + 4 * i as u32), 0, 0, 0];
            }
        }
        for slot in first..first + count as usize {
            b.constants_set[slot / 64] |= 1 << (slot % 64);
        }
        return D3D_OK;
    }
    // Otherwise the registers are read first (a fault stops the run with the bindings as they
    // were), then stored with one lookup of the device's bindings.
    let mut values = [[0u32; 4]; CONSTANT_SLOTS];
    for (i, value) in values.iter_mut().enumerate().take(count as usize) {
        let at = data.addr() + 4 * words * i as u32;
        for (w, slot) in value.iter_mut().enumerate().take(words as usize) {
            *slot = e.mem.u32(at + 4 * w as u32);
        }
    }
    let b = bindings(e, this);
    if b.constants.is_empty() {
        b.constants = vec![[0; 4]; 6 * CONSTANT_SLOTS];
        b.constants_set = vec![0; 6 * CONSTANT_SLOTS / 64];
    }
    for (i, value) in values.iter().enumerate().take(count as usize) {
        let slot = first + i;
        b.constants[slot] = *value;
        b.constants_set[slot / 64] |= 1 << (slot % 64);
    }
    D3D_OK
}

/// The registers `start..start + count` of a constant kind as the device's bindings hold them
/// (4 words each; a bool register's word first): what the last successful
/// `Set*ShaderConstant*` calls stored, read from the program's memory. `None` before the
/// device's first set.
#[cfg(windows)]
pub(super) fn constant_registers(
    e: &Engine,
    device: u32,
    kind: u8,
    start: u32,
    count: u32,
) -> Option<&[[u32; 4]]> {
    let b = e
        .kernel
        .graphics
        .device_objects
        .resources
        .bindings
        .get(&device)?;
    let first = usize::from(kind) * CONSTANT_SLOTS + start as usize;
    b.constants.get(first..first + count as usize)
}

/// `Get*ShaderConstant{F,I,B}(start, data, count)`: the registers' words; a register never set
/// is not provided yet (its initial value).
fn get_constants(e: &mut Engine, this: Ptr, kind: u8, start: u32, data: Ptr, count: u32) -> u32 {
    if data.is_null() || u64::from(start) + u64::from(count) > u64::from(registers(kind)) {
        return D3DERR_INVALIDCALL;
    }
    let words = if kind % 3 == 2 { 1 } else { 4 };
    for i in 0..count {
        let slot = usize::from(kind) * CONSTANT_SLOTS + (start + i) as usize;
        let b = bindings(e, this);
        let set = b
            .constants_set
            .get(slot / 64)
            .is_some_and(|bits| bits & (1 << (slot % 64)) != 0);
        let Some(value) = set.then(|| b.constants[slot]) else {
            panic!(
                "a shader constant read before it was set (kind {kind}, register {}): not provided",
                start + i
            );
        };
        for (w, word) in value.iter().enumerate().take(words as usize) {
            e.mem
                .set_u32(data.addr() + 4 * (words * i + w as u32), *word);
        }
    }
    D3D_OK
}

macro_rules! constants {
    ($set:ident, $get:ident, $kind:expr) => {
        pub(super) fn $set(e: &mut Engine, this: Ptr, start: u32, data: Ptr, count: u32) -> u32 {
            set_constants(e, this, $kind, start, data, count)
        }
        pub(super) fn $get(e: &mut Engine, this: Ptr, start: u32, data: Ptr, count: u32) -> u32 {
            get_constants(e, this, $kind, start, data, count)
        }
    };
}
constants!(set_vertex_constant_f, get_vertex_constant_f, 0);
constants!(set_vertex_constant_i, get_vertex_constant_i, 1);
constants!(set_vertex_constant_b, get_vertex_constant_b, 2);
constants!(set_pixel_constant_f, get_pixel_constant_f, 3);
constants!(set_pixel_constant_i, get_pixel_constant_i, 4);
constants!(set_pixel_constant_b, get_pixel_constant_b, 5);

/// Whether the device has a buffer of its own in `D3DPOOL_DEFAULT` alive (`Reset` refuses then).
pub(super) fn default_pool_alive(e: &Engine, device: u32) -> bool {
    e.kernel
        .graphics
        .device_objects
        .resources
        .objects
        .values()
        .any(|o| o.device == device && matches!(&o.kind, Kind::Buffer { desc, .. } if desc[3] == 0))
}

/// Releases what a device being destroyed binds.
pub(super) fn release_bindings(e: &mut Engine, device: u32) {
    let Some(b) = resources(e).bindings.remove(&device) else {
        return;
    };
    let held = b.streams.values().map(|s| s.0).chain([
        b.indices,
        b.declaration,
        b.vertex_shader,
        b.pixel_shader,
    ]);
    for object in held.collect::<Vec<_>>() {
        if object != 0 {
            unbind(e, Ptr::new(object));
        }
    }
}

impl super::d3d9device::Objects {
    /// Whether an object of this module still exists (tests).
    #[cfg(test)]
    pub(super) fn resources_hold(&self, object: u32) -> bool {
        self.resources.objects.contains_key(&object)
    }
}

/// `D3DPT_POINTLIST` .. `D3DPT_TRIANGLEFAN` (`D3DPRIMITIVETYPE`).
const D3DPT_POINTLIST: u32 = 1;
const D3DPT_TRIANGLEFAN: u32 = 6;

/// What every draw needs before it can be answered: a scene, a primitive type, a vertex
/// declaration or FVF and stream 0. Nothing is drawn yet (the viewer draws), so a draw only
/// answers; a draw the documented semantics leave open here (outside a scene, without a
/// declaration or FVF or stream 0, an unknown primitive type) stops the run as not provided.
/// Answers the index buffer the device holds (0 for none), which the indexed draw checks.
fn draw_checks(e: &mut Engine, this: Ptr, what: &str, kind: u32) -> u32 {
    // Read in place: a copy of the bindings (their stream and constant maps) per draw cost more
    // than the rest of the draw (a29).
    let scene = in_scene(e, this);
    let b = bindings(e, this);
    let open = if !scene {
        Some("outside a scene")
    } else if !(D3DPT_POINTLIST..=D3DPT_TRIANGLEFAN).contains(&kind) {
        Some("an unknown primitive type")
    } else if b.declaration == 0 && b.fvf == 0 {
        Some("without a vertex declaration or FVF")
    } else if !b.streams.contains_key(&0) {
        Some("without stream 0")
    } else {
        None
    };
    if let Some(open) = open {
        panic!("IDirect3DDevice9::{what} {open} is not provided");
    }
    b.indices
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::DrawPrimitive (vtable +0x144)
/// `DrawPrimitive(type, start vertex, primitive count)`: answers `D3D_OK` after
/// [`draw_checks`]; nothing is drawn yet.
pub(super) fn draw_primitive(
    e: &mut Engine,
    this: Ptr,
    kind: u32,
    _start: u32,
    _count: u32,
) -> u32 {
    draw_checks(e, this, "DrawPrimitive", kind);
    D3D_OK
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::DrawIndexedPrimitive (vtable +0x148)
/// `DrawIndexedPrimitive(type, base vertex, min index, vertex count, start index, primitive
/// count)`: answers `D3D_OK` after [`draw_checks`] and with an index buffer set (without one,
/// or for a point list, which indexed draws do not take, the run stops as not provided);
/// nothing is drawn yet.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_indexed_primitive(
    e: &mut Engine,
    this: Ptr,
    kind: u32,
    _base_vertex: u32,
    _min_index: u32,
    _vertices: u32,
    _start_index: u32,
    _count: u32,
) -> u32 {
    if draw_checks(e, this, "DrawIndexedPrimitive", kind) == 0 {
        panic!("IDirect3DDevice9::DrawIndexedPrimitive without an index buffer is not provided");
    }
    if kind == D3DPT_POINTLIST {
        panic!("IDirect3DDevice9::DrawIndexedPrimitive of a point list is not provided");
    }
    D3D_OK
}

/// Leaves stream 0 as a draw from memory leaves it: `buffer` (0 for none, released if it was
/// another), `offset` and `stride` (kept for `GetStreamSource` with no buffer too).
fn leave_stream_zero(e: &mut Engine, this: Ptr, buffer: u32, offset: u32, stride: u32) {
    let old = if buffer != 0 {
        bindings(e, this)
            .streams
            .insert(0, (buffer, offset, stride))
    } else {
        bindings(e, this).streams.remove(&0)
    };
    if buffer == 0 {
        bindings(e, this).empty_strides.insert(0, stride);
    }
    hold(e, old.map_or(0, |o| o.0), buffer);
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::DrawPrimitiveUP (vtable +0x14C)
/// `DrawPrimitiveUP(type, primitive count, vertices, stride)`: a draw from the caller's memory.
/// Nothing is drawn yet (the viewer draws); what the call answers and leaves on stream 0 is the
/// retail runtime's, measured on this machine (2026-10-27, `scripts/d3d9-capture device`, a
/// windowed HAL device with an FVF set):
/// - in a scene with a nonzero stride it answers `D3D_OK` whatever the type (0, 7 and 9
///   included), the count (0 included) and the vertex pointer (null included): the runtime
///   checks none of them; stream 0 is then empty with offset and stride 0 (a vertex buffer it
///   held is released);
/// - in a scene with stride 0 it answers `D3DERR_INVALIDCALL`, stream 0 keeping its buffer with
///   offset and stride 0;
/// - outside a scene it answers `D3DERR_INVALIDCALL` until a draw from memory has succeeded in a
///   scene on the device (on a fresh device, after an empty scene, after a `Present`), stream 0
///   then empty with offset 0 and the given stride; after one it answers `D3D_OK` outside a scene
///   too, as in one.
///
/// Not provided (stop the run): without an FVF or a vertex declaration (the runtime faulted
/// there, measured with the declaration set to null), and stride 0 outside a scene (not
/// measured).
pub(super) fn draw_primitive_up(
    e: &mut Engine,
    this: Ptr,
    _kind: u32,
    _count: u32,
    _vertices: u32,
    stride: u32,
) -> u32 {
    let b = bindings(e, this);
    if b.declaration == 0 && b.fvf == 0 {
        panic!(
            "IDirect3DDevice9::DrawPrimitiveUP without a vertex declaration or FVF is not provided"
        );
    }
    let held = b.streams.get(&0).map_or(0, |s| s.0);
    let drawn_from_memory = b.drawn_from_memory;
    if in_scene(e, this) {
        if stride == 0 {
            leave_stream_zero(e, this, held, 0, 0);
            return D3DERR_INVALIDCALL;
        }
        bindings(e, this).drawn_from_memory = true;
        leave_stream_zero(e, this, 0, 0, 0);
        return D3D_OK;
    }
    if stride == 0 {
        panic!("IDirect3DDevice9::DrawPrimitiveUP with stride 0 outside a scene is not provided");
    }
    if drawn_from_memory {
        leave_stream_zero(e, this, 0, 0, 0);
        D3D_OK
    } else {
        leave_stream_zero(e, this, 0, 0, stride);
        D3DERR_INVALIDCALL
    }
}

forwarded! {
    pub(super) fn fwd_set_stream_source = set_stream_source(stream: u32, buffer: u32, offset: u32, stride: u32)
        => set_stream_source;
    pub(super) fn fwd_set_indices = set_indices(buffer: u32) => set_indices;
    pub(super) fn fwd_set_vertex_declaration = set_vertex_declaration(declaration: u32)
        => set_vertex_declaration;
    pub(super) fn fwd_set_fvf = set_fvf(fvf: u32) => set_fvf;
    pub(super) fn fwd_draw_primitive = draw_primitive(kind: u32, start: u32, count: u32) => draw_primitive;
    pub(super) fn fwd_draw_indexed_primitive = draw_indexed_primitive(
        kind: u32, base_vertex: u32, min_index: u32, vertices: u32, start_index: u32, count: u32
    ) => draw_indexed_primitive;
}

/// `SetVertexShader` / `SetPixelShader`, then (`NV_D3D9=real`) the real shader.
pub(super) fn fwd_set_vertex_shader(e: &mut Engine, this: Ptr, shader: u32) -> u32 {
    let model = set_vertex_shader(e, this, shader);
    #[cfg(windows)]
    if model == D3D_OK {
        super::d3d9real::set_shader(e, this.addr(), shader, false, model);
    }
    model
}

pub(super) fn fwd_set_pixel_shader(e: &mut Engine, this: Ptr, shader: u32) -> u32 {
    let model = set_pixel_shader(e, this, shader);
    #[cfg(windows)]
    if model == D3D_OK {
        super::d3d9real::set_shader(e, this.addr(), shader, true, model);
    }
    model
}

macro_rules! forwarded_constants {
    ($($name:ident = $set:ident, $kind:expr;)*) => {$(
        pub(super) fn $name(e: &mut Engine, this: Ptr, start: u32, data: Ptr, count: u32) -> u32 {
            let model = $set(e, this, start, data, count);
            #[cfg(windows)]
            if model == D3D_OK {
                super::d3d9real::set_constants(e, this.addr(), $kind, start, data.addr(), count, model);
            }
            model
        }
    )*};
}
forwarded_constants! {
    fwd_set_vertex_constant_f = set_vertex_constant_f, 0;
    fwd_set_vertex_constant_i = set_vertex_constant_i, 1;
    fwd_set_vertex_constant_b = set_vertex_constant_b, 2;
    fwd_set_pixel_constant_f = set_pixel_constant_f, 3;
    fwd_set_pixel_constant_i = set_pixel_constant_i, 4;
    fwd_set_pixel_constant_b = set_pixel_constant_b, 5;
}

/// The vertices a draw of `count` primitives of type `kind` takes (`D3DPT_POINTLIST` ..
/// `D3DPT_TRIANGLEFAN`).
#[cfg(windows)]
fn vertices_of(kind: u32, count: u32) -> u32 {
    match kind {
        1 => count,
        2 => 2 * count,
        3 => count + 1,
        4 => 3 * count,
        _ => count + 2,
    }
}

/// `DrawPrimitiveUP` ([`draw_primitive_up`]), then (`NV_D3D9=real`) the same draw on the real
/// device with the vertices from engine memory, when the model answers `D3D_OK` for a known
/// primitive type and a vertex pointer.
pub(super) fn fwd_draw_primitive_up(
    e: &mut Engine,
    this: Ptr,
    kind: u32,
    count: u32,
    vertices: u32,
    stride: u32,
) -> u32 {
    let model = draw_primitive_up(e, this, kind, count, vertices, stride);
    #[cfg(windows)]
    if model == D3D_OK && vertices != 0 && (D3DPT_POINTLIST..=D3DPT_TRIANGLEFAN).contains(&kind) {
        super::d3d9real::draw_primitive_up(
            e,
            this.addr(),
            kind,
            count,
            vertices,
            stride,
            vertices_of(kind, count),
            model,
        );
    }
    model
}
