//! `D3DXCompileShader` and `D3DXGetShaderConstantTable` through the player's own
//! `d3dx9_38.dll` (ADR-0008 item 5), Windows only. The DLL is Microsoft's shader compiler: the
//! layer loads it from the system folder (never copied into the repository) and calls it with
//! the source, macros and bytecode from engine memory; what it makes comes back to the game as
//! objects of this layer in engine memory: an `ID3DXBuffer` (the bytes copied) and an
//! `ID3DXConstantTable` whose reading slots ask the real table and copy the answers (names,
//! default values, descriptions) into engine memory. The game's `ID3DXInclude`, when it passes
//! one, is called back through its vtable in engine memory for each `#include`.
//!
//! The host process is 64-bit, so it loads the 64-bit build of the same DLL; the compiler's
//! output (bytecode and constant table) does not depend on the process's word size. The table's
//! setters (`SetDefaults`, `SetFloat`, ...) take a device and are not provided (the run stops).
//!
//! The texture loaders (`D3DXCreateTextureFromFileInMemory`, the cube one, the image info) go
//! through the same DLL when the real Direct3D 9 is in use (the maintainer's decision of
//! 2026-11-03; [`texture_levels`], chosen by `super::d3dx::texture_source`).

use super::d3d9::{CODE_BASE, D3DERR_INVALIDCALL};
use crate::{Engine, Ptr};
use std::collections::HashMap;
use std::ffi::c_void;

/// Where `ID3DXBuffer`'s and `ID3DXConstantTable`'s slots sit.
pub const BUFFER_CODE: u32 = CODE_BASE + 0x0a00;
pub const TABLE_CODE: u32 = CODE_BASE + 0x0a40;
/// `ID3DXConstantTable`'s slots (d3dx9shader.h order).
pub const TABLE_SLOTS: [&str; 20] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetBufferPointer",
    "GetBufferSize",
    "GetDesc",
    "GetConstantDesc",
    "GetSamplerIndex",
    "GetConstant",
    "GetConstantByName",
    "GetConstantElement",
    "SetDefaults",
    "SetValue",
    "SetBool",
    "SetBoolArray",
    "SetInt",
    "SetIntArray",
    "SetFloat",
    "SetFloatArray",
    "SetVector",
];
const E_NOINTERFACE: u32 = 0x8000_4002;
const E_FAIL: u32 = 0x8000_4005;

type Hresult = i32;
type CompileShader = unsafe extern "system" fn(
    *const u8,
    u32,
    *const [*const u8; 2],
    *mut c_void,
    *const u8,
    *const u8,
    u32,
    *mut *mut c_void,
    *mut *mut c_void,
    *mut *mut c_void,
) -> Hresult;
type GetShaderConstantTable = unsafe extern "system" fn(*const u32, *mut *mut c_void) -> Hresult;

struct Dll {
    compile: CompileShader,
    table: GetShaderConstantTable,
}

/// An export of the player's `d3dx9_38.dll`, which is loaded once from the system folder (the
/// shader compiler and the texture loaders share the one module, never freed); `None` when the
/// DLL is not installed or lacks the export. `name` ends in a NUL.
fn dll_export(name: &[u8]) -> Option<unsafe extern "system" fn() -> isize> {
    use windows::core::PCSTR;
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::System::LibraryLoader::{
        GetProcAddress, LoadLibraryExA, LOAD_LIBRARY_SEARCH_SYSTEM32,
    };
    // The handle kept as an address: `HMODULE` holds a raw pointer, which a static cannot.
    static MODULE: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();
    let module = (*MODULE.get_or_init(|| unsafe {
        LoadLibraryExA(
            PCSTR(b"d3dx9_38.dll\0".as_ptr()),
            None,
            LOAD_LIBRARY_SEARCH_SYSTEM32,
        )
        .ok()
        .map(|m| m.0 as usize)
    }))?;
    assert_eq!(name.last(), Some(&0), "an export name ends in a NUL");
    unsafe { GetProcAddress(HMODULE(module as *mut c_void), PCSTR(name.as_ptr())) }
}

/// The shader compiler's functions of the player's `d3dx9_38.dll` ([`dll_export`]).
fn dll() -> &'static Dll {
    static DLL: std::sync::OnceLock<Option<Dll>> = std::sync::OnceLock::new();
    DLL.get_or_init(|| unsafe {
        let compile = dll_export(b"D3DXCompileShader\0")?;
        let table = dll_export(b"D3DXGetShaderConstantTable\0")?;
        Some(Dll {
            compile: std::mem::transmute::<unsafe extern "system" fn() -> isize, CompileShader>(
                compile,
            ),
            table: std::mem::transmute::<
                unsafe extern "system" fn() -> isize,
                GetShaderConstantTable,
            >(table),
        })
    })
    .as_ref()
    .unwrap_or_else(|| {
        panic!("d3dx9_38.dll (the DirectX June 2010 runtime) is not on this system: D3DXCompileShader is not provided")
    })
}

/// Slot `index` of a real COM object's vtable.
unsafe fn slot(object: *mut c_void, index: usize) -> usize {
    *(*(object as *const *const usize)).add(index)
}

unsafe fn release(object: *mut c_void) {
    let f: unsafe extern "system" fn(*mut c_void) -> u32 = std::mem::transmute(slot(object, 2));
    f(object);
}

/// A real `ID3DXBuffer`'s bytes.
unsafe fn buffer_bytes(object: *mut c_void) -> Vec<u8> {
    let pointer: unsafe extern "system" fn(*mut c_void) -> *const u8 =
        std::mem::transmute(slot(object, 3));
    let size: unsafe extern "system" fn(*mut c_void) -> usize =
        std::mem::transmute(slot(object, 4));
    let (p, n) = (pointer(object), size(object));
    if p.is_null() {
        return Vec::new();
    }
    std::slice::from_raw_parts(p, n).to_vec()
}

/// A buffer of this layer: its bytes in engine memory.
#[derive(Debug)]
struct Buffer {
    references: u32,
    data: u32,
    size: u32,
}

/// A constant table of this layer: the real table, and what was copied into engine memory for
/// it (strings and default values by their real address, handles both ways).
#[derive(Debug)]
struct Table {
    references: u32,
    real: usize,
    bytes: u32,
    size: u32,
    copies: HashMap<usize, u32>,
    handles: HashMap<u32, usize>,
    by_real: HashMap<usize, u32>,
}

/// The buffers and tables (`Graphics::d3dx`).
#[derive(Debug, Default)]
pub struct Objects {
    registered: bool,
    buffer_vtable: u32,
    table_vtable: u32,
    buffers: HashMap<u32, Buffer>,
    tables: HashMap<u32, Table>,
    next_handle: u32,
    /// Strings the DLL answered, copied into engine memory once each.
    strings: HashMap<Vec<u8>, u32>,
}

/// Engine handles a table gives out: opaque values in no mapped memory.
const HANDLE_BASE: u32 = 0x7e30_0000;

fn objects(e: &mut Engine) -> &mut Objects {
    &mut state!(e).graphics.d3dx
}

fn register(e: &mut Engine) {
    if objects(e).registered {
        return;
    }
    let buffer: [(u32, crate::AbiFn); 5] = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, buffer_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, buffer_release(Ptr) -> u32).1),
        (3, crate::entry!(0, buffer_pointer(Ptr) -> u32).1),
        (4, crate::entry!(0, buffer_size(Ptr) -> u32).1),
    ];
    for (i, f) in buffer {
        e.register(BUFFER_CODE + 4 * i, f);
    }
    let table: [(u32, crate::AbiFn); 11] = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, table_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, table_release(Ptr) -> u32).1),
        (3, crate::entry!(0, table_pointer(Ptr) -> u32).1),
        (4, crate::entry!(0, table_size(Ptr) -> u32).1),
        (5, crate::entry!(0, table_get_desc(Ptr, Ptr) -> u32).1),
        (
            6,
            crate::entry!(0, table_get_constant_desc(Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
        (
            7,
            crate::entry!(0, table_get_sampler_index(Ptr, u32) -> u32).1,
        ),
        (
            8,
            crate::entry!(0, table_get_constant(Ptr, u32, u32) -> u32).1,
        ),
        (
            9,
            crate::entry!(0, table_get_constant_by_name(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            10,
            crate::entry!(0, table_get_constant_element(Ptr, u32, u32) -> u32).1,
        ),
    ];
    for (i, f) in table {
        e.register(TABLE_CODE + 4 * i, f);
    }
    let make = |e: &mut Engine, code: u32, slots: u32| {
        let t = e.mem.alloc(4 * slots);
        for i in 0..slots {
            e.mem.set_u32(t + 4 * i, code + 4 * i);
        }
        t
    };
    let b = make(e, BUFFER_CODE, 5);
    let t = make(e, TABLE_CODE, TABLE_SLOTS.len() as u32);
    let o = objects(e);
    o.buffer_vtable = b;
    o.table_vtable = t;
    o.registered = true;
    o.next_handle = HANDLE_BASE;
}

fn no_interface(e: &mut Engine, _this: Ptr, _iid: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

/// A buffer of this layer holding `bytes`.
fn new_buffer(e: &mut Engine, bytes: &[u8]) -> u32 {
    register(e);
    let object = e.mem.alloc(8);
    let table = objects(e).buffer_vtable;
    e.mem.set_u32(object, table);
    let data = e.mem.alloc(bytes.len().max(1) as u32);
    e.mem.write(data, bytes);
    objects(e).buffers.insert(
        object,
        Buffer {
            references: 1,
            data,
            size: bytes.len() as u32,
        },
    );
    object
}

fn buffer(e: &mut Engine, this: Ptr) -> &mut Buffer {
    objects(e)
        .buffers
        .get_mut(&this.addr())
        .expect("an ID3DXBuffer of this layer")
}

fn buffer_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let b = buffer(e, this);
    b.references += 1;
    b.references
}

fn buffer_release(e: &mut Engine, this: Ptr) -> u32 {
    let b = buffer(e, this);
    b.references -= 1;
    let left = b.references;
    if left == 0 {
        let b = objects(e).buffers.remove(&this.addr()).expect("the buffer");
        e.mem.free(b.data);
        e.mem.free(this.addr());
    }
    left
}

fn buffer_pointer(e: &mut Engine, this: Ptr) -> u32 {
    buffer(e, this).data
}

fn buffer_size(e: &mut Engine, this: Ptr) -> u32 {
    buffer(e, this).size
}

/// A table of this layer for the real table `real` (the reference is kept).
fn new_table(e: &mut Engine, real: *mut c_void) -> u32 {
    register(e);
    let bytes = unsafe { buffer_bytes(real) };
    let data = e.mem.alloc(bytes.len().max(1) as u32);
    e.mem.write(data, &bytes);
    let object = e.mem.alloc(8);
    let table = objects(e).table_vtable;
    e.mem.set_u32(object, table);
    objects(e).tables.insert(
        object,
        Table {
            references: 1,
            real: real as usize,
            bytes: data,
            size: bytes.len() as u32,
            copies: HashMap::new(),
            handles: HashMap::new(),
            by_real: HashMap::new(),
        },
    );
    object
}

fn table(e: &mut Engine, this: Ptr) -> &mut Table {
    objects(e)
        .tables
        .get_mut(&this.addr())
        .expect("an ID3DXConstantTable of this layer")
}

fn table_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let t = table(e, this);
    t.references += 1;
    t.references
}

fn table_release(e: &mut Engine, this: Ptr) -> u32 {
    let t = table(e, this);
    t.references -= 1;
    let left = t.references;
    if left == 0 {
        let t = objects(e).tables.remove(&this.addr()).expect("the table");
        unsafe { release(t.real as *mut c_void) };
        e.mem.free(t.bytes);
        for copy in t.copies.values() {
            e.mem.free(*copy);
        }
        e.mem.free(this.addr());
    }
    left
}

fn table_pointer(e: &mut Engine, this: Ptr) -> u32 {
    table(e, this).bytes
}

fn table_size(e: &mut Engine, this: Ptr) -> u32 {
    table(e, this).size
}

/// The engine copy of `len` bytes at the real address `real` (made once per table).
fn copy_out(e: &mut Engine, this: Ptr, real: usize, len: usize) -> u32 {
    if real == 0 {
        return 0;
    }
    if let Some(&at) = table(e, this).copies.get(&real) {
        return at;
    }
    let bytes = unsafe { std::slice::from_raw_parts(real as *const u8, len) }.to_vec();
    let at = e.mem.alloc(len.max(1) as u32);
    e.mem.write(at, &bytes);
    table(e, this).copies.insert(real, at);
    at
}

/// The engine copy of the C string at the real address `real`.
fn string_out(e: &mut Engine, this: Ptr, real: usize) -> u32 {
    if real == 0 {
        return 0;
    }
    let len = unsafe { std::ffi::CStr::from_ptr(real as *const std::ffi::c_char) }
        .to_bytes()
        .len();
    copy_out(e, this, real, len + 1)
}

/// The engine handle of a real handle (0 for 0).
fn handle_out(e: &mut Engine, this: Ptr, real: usize) -> u32 {
    if real == 0 {
        return 0;
    }
    if let Some(&h) = table(e, this).by_real.get(&real) {
        return h;
    }
    let h = {
        let o = objects(e);
        o.next_handle += 4;
        o.next_handle
    };
    let t = table(e, this);
    t.by_real.insert(real, h);
    t.handles.insert(h, real);
    h
}

/// The real handle of an engine handle: one this table gave out, or (as D3DX allows) the
/// engine address of a constant's name; the name's copy is kept alive with the table.
fn handle_in(e: &mut Engine, this: Ptr, handle: u32) -> usize {
    if handle == 0 {
        return 0;
    }
    if let Some(&real) = table(e, this).handles.get(&handle) {
        return real;
    }
    let mut name = e.mem.cstr(handle);
    name.push(0);
    let leaked = Box::leak(name.into_boxed_slice());
    leaked.as_ptr() as usize
}

/// `D3DXCONSTANTTABLE_DESC` (Creator, Version, Constants).
fn table_get_desc(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let real = table(e, this).real as *mut c_void;
    let mut desc = [0usize; 2];
    let hr = unsafe {
        let f: unsafe extern "system" fn(*mut c_void, *mut usize) -> Hresult =
            std::mem::transmute(slot(real, 5));
        f(real, desc.as_mut_ptr())
    };
    if hr >= 0 {
        let creator = string_out(e, this, desc[0]);
        e.mem.set_u32(out.addr(), creator);
        e.mem.set_u32(out.addr() + 4, desc[1] as u32);
        e.mem.set_u32(out.addr() + 8, (desc[1] >> 32) as u32);
    }
    hr as u32
}

/// `D3DXCONSTANT_DESC` as the 64-bit DLL writes it: Name, ten words, DefaultValue.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct ConstantDesc {
    name: usize,
    words: [u32; 10],
    default: usize,
}

fn table_get_constant_desc(e: &mut Engine, this: Ptr, handle: u32, out: Ptr, count: Ptr) -> u32 {
    if count.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let real = table(e, this).real as *mut c_void;
    let h = handle_in(e, this, handle);
    let mut n = e.mem.u32(count.addr());
    let mut descs = vec![ConstantDesc::default(); n.max(1) as usize];
    let hr = unsafe {
        let f: unsafe extern "system" fn(
            *mut c_void,
            usize,
            *mut ConstantDesc,
            *mut u32,
        ) -> Hresult = std::mem::transmute(slot(real, 6));
        f(
            real,
            h,
            if out.is_null() {
                std::ptr::null_mut()
            } else {
                descs.as_mut_ptr()
            },
            &mut n,
        )
    };
    e.mem.set_u32(count.addr(), n);
    if hr >= 0 && !out.is_null() {
        for (i, d) in descs.iter().take(n as usize).enumerate() {
            let at = out.addr() + 0x30 * i as u32;
            let name = string_out(e, this, d.name);
            let bytes = d.words[9] as usize;
            let default = copy_out(e, this, d.default, bytes);
            e.mem.set_u32(at, name);
            for (w, word) in d.words.iter().enumerate() {
                e.mem.set_u32(at + 4 + 4 * w as u32, *word);
            }
            e.mem.set_u32(at + 0x2c, default);
        }
    }
    hr as u32
}

fn table_get_sampler_index(e: &mut Engine, this: Ptr, handle: u32) -> u32 {
    let real = table(e, this).real as *mut c_void;
    let h = handle_in(e, this, handle);
    unsafe {
        let f: unsafe extern "system" fn(*mut c_void, usize) -> u32 =
            std::mem::transmute(slot(real, 7));
        f(real, h)
    }
}

fn get_handle(e: &mut Engine, this: Ptr, index: usize, parent: u32, word: usize) -> u32 {
    let real = table(e, this).real as *mut c_void;
    let p = handle_in(e, this, parent);
    let h = unsafe {
        let f: unsafe extern "system" fn(*mut c_void, usize, usize) -> usize =
            std::mem::transmute(slot(real, index));
        f(real, p, word)
    };
    handle_out(e, this, h)
}

fn table_get_constant(e: &mut Engine, this: Ptr, parent: u32, index: u32) -> u32 {
    get_handle(e, this, 8, parent, index as usize)
}

fn table_get_constant_element(e: &mut Engine, this: Ptr, parent: u32, index: u32) -> u32 {
    get_handle(e, this, 10, parent, index as usize)
}

fn table_get_constant_by_name(e: &mut Engine, this: Ptr, parent: u32, name: Ptr) -> u32 {
    let mut text = e.mem.cstr(name.addr());
    text.push(0);
    get_handle(e, this, 9, parent, text.as_ptr() as usize)
}

// The game's `ID3DXInclude` (engine memory), called back while the DLL compiles.

thread_local! {
    static ENGINE: std::cell::Cell<*mut Engine> = const { std::cell::Cell::new(std::ptr::null_mut()) };
}

/// The host-side `ID3DXInclude` the DLL calls: the game's include object, and the data of the
/// open includes (host copy, engine address).
#[repr(C)]
struct Include {
    vtable: *const IncludeVtable,
    game: u32,
    open: Vec<(Box<[u8]>, u32)>,
}

#[repr(C)]
struct IncludeVtable {
    open: unsafe extern "system" fn(
        *mut Include,
        u32,
        *const u8,
        *const c_void,
        *mut *const c_void,
        *mut u32,
    ) -> Hresult,
    close: unsafe extern "system" fn(*mut Include, *const c_void) -> Hresult,
}

static INCLUDE_VTABLE: IncludeVtable = IncludeVtable {
    open: include_open,
    close: include_close,
};

unsafe extern "system" fn include_open(
    this: *mut Include,
    kind: u32,
    name: *const u8,
    parent: *const c_void,
    data: *mut *const c_void,
    bytes: *mut u32,
) -> Hresult {
    let e = &mut *ENGINE.with(|c| c.get());
    let include = &mut *this;
    let name = std::ffi::CStr::from_ptr(name as *const std::ffi::c_char).to_bytes();
    let parent = include
        .open
        .iter()
        .find(|(d, _)| d.as_ptr() as *const c_void == parent)
        .map_or(0, |(_, at)| *at);
    let frame = e.mem.alloc(name.len() as u32 + 1 + 8);
    e.mem.write(frame, name);
    e.mem.set_u8(frame + name.len() as u32, 0);
    let out = frame + name.len() as u32 + 1;
    e.mem.set_u32(out, 0);
    e.mem.set_u32(out + 4, 0);
    let hr = e
        .vcall(include.game, 0, &[kind, frame, parent, out, out + 4])
        .u32() as Hresult;
    let (at, len) = (e.mem.u32(out), e.mem.u32(out + 4));
    e.mem.free(frame);
    if hr < 0 {
        return hr;
    }
    let copy = e.mem.bytes(at, len).into_boxed_slice();
    *data = copy.as_ptr() as *const c_void;
    *bytes = len;
    include.open.push((copy, at));
    hr
}

unsafe extern "system" fn include_close(this: *mut Include, data: *const c_void) -> Hresult {
    let e = &mut *ENGINE.with(|c| c.get());
    let include = &mut *this;
    let Some(i) = include
        .open
        .iter()
        .position(|(d, _)| d.as_ptr() as *const c_void == data)
    else {
        return E_FAIL as Hresult;
    };
    let (_, at) = include.open.remove(i);
    e.vcall(include.game, 4, &[at]).u32() as Hresult
}

// Platform: replaces d3dx9_38.dll D3DXCompileShader (import, thunk 00ee6e0a)
/// `D3DXCompileShader(source, length, defines, include, function, profile, flags, shader out,
/// errors out, constant table out)`: the player's DLL compiles the source from engine memory
/// (the macros copied, the game's include object called back); the shader, the error text and
/// the constant table come back as objects of this layer. The answer is the DLL's.
#[allow(clippy::too_many_arguments)]
pub fn compile_shader(
    e: &mut Engine,
    source: Ptr,
    length: u32,
    defines: Ptr,
    include: Ptr,
    function: Ptr,
    profile: Ptr,
    flags: u32,
    shader: Ptr,
    errors: Ptr,
    constants: Ptr,
) -> u32 {
    let dll = dll();
    let text = e.mem.bytes(source.addr(), length);
    let cstring = |e: &Engine, at: u32| -> Option<Vec<u8>> {
        (at != 0).then(|| {
            let mut s = e.mem.cstr(at);
            s.push(0);
            s
        })
    };
    // D3DXMACRO { Name, Definition } pairs up to a null name.
    let mut strings: Vec<Vec<u8>> = Vec::new();
    let mut macros: Vec<[*const u8; 2]> = Vec::new();
    if !defines.is_null() {
        let mut at = defines.addr();
        loop {
            let (name, definition) = (e.mem.u32(at), e.mem.u32(at + 4));
            if name == 0 {
                break;
            }
            strings.push(cstring(e, name).unwrap_or_default());
            strings.push(cstring(e, definition).unwrap_or_else(|| vec![0]));
            at += 8;
        }
        for pair in strings.chunks(2) {
            macros.push([pair[0].as_ptr(), pair[1].as_ptr()]);
        }
        macros.push([std::ptr::null(), std::ptr::null()]);
    }
    let function_name = cstring(e, function.addr());
    let profile_name = cstring(e, profile.addr());
    let mut host_include = Include {
        vtable: &INCLUDE_VTABLE,
        game: include.addr(),
        open: Vec::new(),
    };
    let (mut out_shader, mut out_errors, mut out_table) = (
        std::ptr::null_mut::<c_void>(),
        std::ptr::null_mut::<c_void>(),
        std::ptr::null_mut::<c_void>(),
    );
    ENGINE.with(|c| c.set(e as *mut Engine));
    let hr = unsafe {
        (dll.compile)(
            text.as_ptr(),
            length,
            if macros.is_empty() {
                std::ptr::null()
            } else {
                macros.as_ptr()
            },
            if include.is_null() {
                std::ptr::null_mut()
            } else {
                &mut host_include as *mut Include as *mut c_void
            },
            function_name
                .as_ref()
                .map_or(std::ptr::null(), |s| s.as_ptr()),
            profile_name
                .as_ref()
                .map_or(std::ptr::null(), |s| s.as_ptr()),
            flags,
            if shader.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_shader
            },
            if errors.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_errors
            },
            if constants.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_table
            },
        )
    };
    ENGINE.with(|c| c.set(std::ptr::null_mut()));
    for (out, real) in [(shader, out_shader), (errors, out_errors)] {
        if out.is_null() {
            continue;
        }
        let object = if real.is_null() {
            0
        } else {
            let bytes = unsafe { buffer_bytes(real) };
            unsafe { release(real) };
            new_buffer(e, &bytes)
        };
        e.mem.set_u32(out.addr(), object);
    }
    if !constants.is_null() {
        let object = if out_table.is_null() {
            0
        } else {
            new_table(e, out_table)
        };
        e.mem.set_u32(constants.addr(), object);
    }
    hr as u32
}

/// The length in bytes of shader bytecode at `at` (up to and including the end token).
fn bytecode_length(e: &Engine, at: u32) -> u32 {
    let mut p = at + 4;
    loop {
        let token = e.mem.u32(p);
        p += 4;
        if token == 0x0000_ffff {
            return p - at;
        }
        // A comment token carries its length in DWORDs in bits 16..30.
        if token & 0xffff == 0xfffe {
            p += 4 * ((token >> 16) & 0x7fff);
        }
    }
}

// Platform: replaces d3dx9_38.dll D3DXGetShaderConstantTable (import, thunk 00ee6e40)
/// `D3DXGetShaderConstantTable(function, table out)`: the player's DLL reads the constant table
/// of the bytecode in engine memory; it comes back as a table of this layer.
pub fn get_shader_constant_table(e: &mut Engine, function: Ptr, out: Ptr) -> u32 {
    if function.is_null() || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let dll = dll();
    let len = bytecode_length(e, function.addr());
    let bytes = e.mem.bytes(function.addr(), len);
    let words: Vec<u32> = bytes
        .chunks(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    let mut real = std::ptr::null_mut::<c_void>();
    let hr = unsafe { (dll.table)(words.as_ptr(), &mut real) };
    let object = if hr >= 0 && !real.is_null() {
        new_table(e, real)
    } else {
        0
    };
    e.mem.set_u32(out.addr(), object);
    hr as u32
}

// The device the DLL sees: a host-side object whose `GetDeviceCaps` asks the game's (modelled)
// device in engine memory, for the profile functions. Other slots are not given to the DLL.

#[repr(C)]
struct ShimDevice {
    vtable: *const usize,
    device: u32,
}

unsafe extern "system" fn shim_query_interface(
    _this: *mut ShimDevice,
    _iid: *const c_void,
    out: *mut *mut c_void,
) -> Hresult {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    E_NOINTERFACE as Hresult
}

unsafe extern "system" fn shim_reference(_this: *mut ShimDevice) -> u32 {
    1
}

unsafe extern "system" fn shim_get_device_caps(this: *mut ShimDevice, caps: *mut u8) -> Hresult {
    let e = &mut *ENGINE.with(|c| c.get());
    let device = (*this).device;
    let at = e.mem.alloc(304);
    let hr = e.vcall(device, 4 * 7, &[at]).u32() as Hresult;
    let bytes = e.mem.bytes(at, 304);
    e.mem.free(at);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), caps, 304);
    hr
}

unsafe extern "system" fn shim_unprovided(_this: *mut ShimDevice) -> Hresult {
    panic!("d3dx9_38.dll called a device slot the D3DX layer does not give it (only GetDeviceCaps): not provided")
}

fn shim_vtable() -> &'static [usize; 119] {
    static TABLE: std::sync::OnceLock<[usize; 119]> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = [shim_unprovided as *const () as usize; 119];
        t[0] = shim_query_interface as *const () as usize;
        t[1] = shim_reference as *const () as usize;
        t[2] = shim_reference as *const () as usize;
        t[7] = shim_get_device_caps as *const () as usize;
        t
    })
}

/// A string the DLL answers (static in the DLL), copied into engine memory once.
fn string_copy(e: &mut Engine, text: *const u8) -> u32 {
    if text.is_null() {
        return 0;
    }
    let bytes = unsafe { std::ffi::CStr::from_ptr(text as *const std::ffi::c_char) }
        .to_bytes()
        .to_vec();
    if let Some(&at) = objects(e).strings.get(&bytes) {
        return at;
    }
    let at = e.mem.alloc(bytes.len() as u32 + 1);
    e.mem.write(at, &bytes);
    e.mem.set_u8(at + bytes.len() as u32, 0);
    objects(e).strings.insert(bytes, at);
    at
}

/// An export of the loaded DLL by name (with its terminating zero).
fn export(name: &[u8]) -> unsafe extern "system" fn() -> isize {
    dll();
    dll_export(name)
        .unwrap_or_else(|| panic!("{} is not in d3dx9_38.dll", String::from_utf8_lossy(name)))
}

fn profile(e: &mut Engine, device: Ptr, name: &[u8]) -> u32 {
    let f: unsafe extern "system" fn(*mut ShimDevice) -> *const u8 =
        unsafe { std::mem::transmute(export(name)) };
    let mut shim = ShimDevice {
        vtable: shim_vtable().as_ptr(),
        device: device.addr(),
    };
    ENGINE.with(|c| c.set(e as *mut Engine));
    let text = unsafe { f(&mut shim) };
    ENGINE.with(|c| c.set(std::ptr::null_mut()));
    string_copy(e, text)
}

// Platform: replaces d3dx9_38.dll D3DXGetVertexShaderProfile (import)
/// `D3DXGetVertexShaderProfile(device)`: the player's DLL, which reads the device's caps (the
/// game's device answers them, as on the player's machine); its string copied into engine memory.
pub fn get_vertex_shader_profile(e: &mut Engine, device: Ptr) -> u32 {
    profile(e, device, b"D3DXGetVertexShaderProfile\0")
}

// Platform: replaces d3dx9_38.dll D3DXGetPixelShaderProfile (import, thunk 00ee6e28)
/// `D3DXGetPixelShaderProfile(device)`: as [`get_vertex_shader_profile`].
pub fn get_pixel_shader_profile(e: &mut Engine, device: Ptr) -> u32 {
    profile(e, device, b"D3DXGetPixelShaderProfile\0")
}

// Platform: replaces d3dx9_38.dll D3DXCreateBuffer (import, thunk 00ee6e3a)
/// `D3DXCreateBuffer(size, out)`: a buffer of this layer of `size` bytes (zeroed: the DLL leaves
/// the memory as its allocator gives it, the program writes it). A null out answers
/// `D3DERR_INVALIDCALL`.
pub fn create_buffer(e: &mut Engine, size: u32, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let object = new_buffer(e, &vec![0u8; size as usize]);
    e.mem.set_u32(out.addr(), object);
    0
}

type AssembleShader = unsafe extern "system" fn(
    *const u8,
    u32,
    *const [*const u8; 2],
    *mut c_void,
    u32,
    *mut *mut c_void,
    *mut *mut c_void,
) -> Hresult;

// Platform: replaces d3dx9_38.dll D3DXAssembleShader (import)
/// `D3DXAssembleShader(source, length, defines, include, flags, shader out, errors out)`: the
/// player's DLL assembles the source from engine memory (as [`compile_shader`]: the macros
/// copied, the include called back, the buffers made in engine memory); the answer is the DLL's.
#[allow(clippy::too_many_arguments)]
pub fn assemble_shader(
    e: &mut Engine,
    source: Ptr,
    length: u32,
    defines: Ptr,
    include: Ptr,
    flags: u32,
    shader: Ptr,
    errors: Ptr,
) -> u32 {
    let f: AssembleShader = unsafe { std::mem::transmute(export(b"D3DXAssembleShader\0")) };
    let text = e.mem.bytes(source.addr(), length);
    let (_strings, macros) = read_macros(e, defines);
    let mut host_include = Include {
        vtable: &INCLUDE_VTABLE,
        game: include.addr(),
        open: Vec::new(),
    };
    let (mut out_shader, mut out_errors) = (std::ptr::null_mut(), std::ptr::null_mut());
    ENGINE.with(|c| c.set(e as *mut Engine));
    let hr = unsafe {
        f(
            text.as_ptr(),
            length,
            if macros.is_empty() {
                std::ptr::null()
            } else {
                macros.as_ptr()
            },
            if include.is_null() {
                std::ptr::null_mut()
            } else {
                &mut host_include as *mut Include as *mut c_void
            },
            flags,
            if shader.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_shader
            },
            if errors.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_errors
            },
        )
    };
    ENGINE.with(|c| c.set(std::ptr::null_mut()));
    for (out, real) in [(shader, out_shader), (errors, out_errors)] {
        if out.is_null() {
            continue;
        }
        let object = if real.is_null() {
            0
        } else {
            let bytes = unsafe { buffer_bytes(real) };
            unsafe { release(real) };
            new_buffer(e, &bytes)
        };
        e.mem.set_u32(out.addr(), object);
    }
    hr as u32
}

/// The `D3DXMACRO` pairs at `defines` (up to a null name) as host strings, and the pointer
/// pairs the DLL takes (with the null pair at the end); empty for a null `defines`.
#[allow(clippy::type_complexity)]
fn read_macros(e: &Engine, defines: Ptr) -> (Vec<Vec<u8>>, Vec<[*const u8; 2]>) {
    let mut strings: Vec<Vec<u8>> = Vec::new();
    let mut macros: Vec<[*const u8; 2]> = Vec::new();
    if defines.is_null() {
        return (strings, macros);
    }
    let cstring = |at: u32| -> Vec<u8> {
        let mut s = if at == 0 { Vec::new() } else { e.mem.cstr(at) };
        s.push(0);
        s
    };
    let mut at = defines.addr();
    loop {
        let (name, definition) = (e.mem.u32(at), e.mem.u32(at + 4));
        if name == 0 {
            break;
        }
        strings.push(cstring(name));
        strings.push(cstring(definition));
        at += 8;
    }
    for pair in strings.chunks(2) {
        macros.push([pair[0].as_ptr(), pair[1].as_ptr()]);
    }
    macros.push([std::ptr::null(), std::ptr::null()]);
    (strings, macros)
}

// Textures (maintainer, 2026-11-03): `D3DXCreateTextureFromFileInMemory`,
// `D3DXCreateCubeTextureFromFileInMemory` and `D3DXGetImageInfoFromFileInMemory` through the same
// 64-bit DLL, on the real device behind the game's (`NV_D3D9=real`). Measured over all 21,776 of
// the game's textures: the 64-bit DLL's textures are the 32-bit DLL's bit for bit on 18,356; on
// the other 3,420 the levels D3DX generates in DXT formats differ (its 64-bit DXT encoder), which
// the maintainer looked at and accepted. `super::d3dx` chooses between this and the texture cache.

type CreateFromFile =
    unsafe extern "system" fn(*mut c_void, *const u8, u32, *mut *mut c_void) -> Hresult;
type ImageInfoFromFile = unsafe extern "system" fn(*const u8, u32, *mut u32) -> Hresult;

struct TextureDll {
    texture: CreateFromFile,
    cube: CreateFromFile,
    info: ImageInfoFromFile,
}

/// The texture functions of the player's `d3dx9_38.dll` ([`dll_export`]: the system folder's, the
/// 64-bit build in this 64-bit process); `None` when it is not installed, so the caller can fall
/// back.
fn texture_dll() -> Option<&'static TextureDll> {
    static DLL: std::sync::OnceLock<Option<TextureDll>> = std::sync::OnceLock::new();
    DLL.get_or_init(|| unsafe {
        let get = dll_export;
        type Export = unsafe extern "system" fn() -> isize;
        Some(TextureDll {
            texture: std::mem::transmute::<Export, CreateFromFile>(get(
                b"D3DXCreateTextureFromFileInMemory\0",
            )?),
            cube: std::mem::transmute::<Export, CreateFromFile>(get(
                b"D3DXCreateCubeTextureFromFileInMemory\0",
            )?),
            info: std::mem::transmute::<Export, ImageInfoFromFile>(get(
                b"D3DXGetImageInfoFromFileInMemory\0",
            )?),
        })
    })
    .as_ref()
}

/// Whether the player's `d3dx9_38.dll` can answer the texture calls in this process.
pub fn textures_available() -> bool {
    texture_dll().is_some()
}

/// `D3DXGetImageInfoFromFileInMemory(data, size, info)` by the DLL: its answer, and the seven
/// words of `D3DXIMAGE_INFO` (the same layout in a 32-bit and a 64-bit process) after the call,
/// starting from `info` (what the game's buffer held, so a word the DLL leaves alone is left
/// alone). `None` when the DLL is not installed.
pub fn image_info(data: &[u8], info: [u32; 7]) -> Option<(u32, [u32; 7])> {
    let dll = texture_dll()?;
    let mut words = info;
    let hr = unsafe { (dll.info)(data.as_ptr(), data.len() as u32, words.as_mut_ptr()) };
    Some((hr as u32, words))
}

/// The bytes of one locked level (rows packed, as the cache keeps them) and its description.
///
/// # Safety
/// `locked` is a lock of that level, still held.
unsafe fn packed_level(
    desc: &windows::Win32::Graphics::Direct3D9::D3DSURFACE_DESC,
    locked: &windows::Win32::Graphics::Direct3D9::D3DLOCKED_RECT,
) -> super::d3dx::CachedSurface {
    let format = desc.Format.0;
    let Some((block, bytes)) = super::d3d9device::format_blocks(format) else {
        panic!(
            "d3dx9_38.dll made a texture of format {format:#x}, whose memory layout the Direct3D 9 layer does not provide"
        );
    };
    let row = (desc.Width.div_ceil(block) * bytes) as usize;
    let rows = desc.Height.div_ceil(block) as usize;
    let mut out = Vec::with_capacity(row * rows);
    for r in 0..rows {
        out.extend_from_slice(std::slice::from_raw_parts(
            (locked.pBits as *const u8).add(r * locked.Pitch as usize),
            row,
        ));
    }
    super::d3dx::CachedSurface {
        format,
        width: desc.Width,
        height: desc.Height,
        bytes: Some(out),
    }
}

/// `D3DXCreateTextureFromFileInMemory` (or, `cube`, `D3DXCreateCubeTextureFromFileInMemory`) of
/// `data` by the DLL on the real `device`, the call the game makes, with D3DX's defaults; the
/// texture it made read back level by level (face by face for a cube, the DDS order) as a
/// texture cache entry is, every level with D3DX's bytes, and released. `None` when the DLL is not
/// installed. A failed call gives an entry with the DLL's answer and no levels.
pub fn texture_levels(
    device: &windows::Win32::Graphics::Direct3D9::IDirect3DDevice9,
    data: &[u8],
    cube: bool,
) -> Option<super::d3dx::CachedTexture> {
    use windows::core::Interface;
    use windows::Win32::Graphics::Direct3D9::IDirect3DBaseTexture9;
    let dll = texture_dll()?;
    let create = if cube { dll.cube } else { dll.texture };
    let mut made = std::ptr::null_mut::<c_void>();
    let hr = unsafe { create(device.as_raw(), data.as_ptr(), data.len() as u32, &mut made) } as u32;
    let faces = if cube { 6 } else { 1 };
    if hr != 0 || made.is_null() {
        if !made.is_null() {
            unsafe { release(made) };
        }
        return Some(super::d3dx::CachedTexture {
            hr,
            levels: 0,
            faces,
            surfaces: Vec::new(),
        });
    }
    let made = unsafe { IDirect3DBaseTexture9::from_raw(made) };
    let (levels, surfaces) = read_levels(&made, cube);
    drop(made);
    Some(super::d3dx::CachedTexture {
        hr,
        levels,
        faces,
        surfaces,
    })
}

/// The level count of a real `texture` (a cube texture when `cube`) and its levels as a texture
/// cache entry holds them (face by face for a cube, the DDS order; rows packed), each read with
/// `LockRect(D3DLOCK_READONLY)`.
pub(crate) fn read_levels(
    texture: &windows::Win32::Graphics::Direct3D9::IDirect3DBaseTexture9,
    cube: bool,
) -> (u32, Vec<super::d3dx::CachedSurface>) {
    use windows::core::Interface;
    use windows::Win32::Graphics::Direct3D9::{
        IDirect3DCubeTexture9, IDirect3DTexture9, D3DCUBEMAP_FACES, D3DLOCKED_RECT,
        D3DLOCK_READONLY, D3DSURFACE_DESC,
    };
    let mut surfaces = Vec::new();
    let levels;
    unsafe {
        if cube {
            let t: IDirect3DCubeTexture9 = texture.cast().expect("a cube texture");
            levels = t.GetLevelCount();
            for face in 0..6 {
                for level in 0..levels {
                    let (mut desc, mut locked) =
                        (D3DSURFACE_DESC::default(), D3DLOCKED_RECT::default());
                    t.GetLevelDesc(level, &mut desc)
                        .expect("GetLevelDesc of a real cube texture");
                    let face = D3DCUBEMAP_FACES(face);
                    t.LockRect(
                        face,
                        level,
                        &mut locked,
                        std::ptr::null(),
                        D3DLOCK_READONLY as u32,
                    )
                    .expect("LockRect of a real cube texture");
                    surfaces.push(packed_level(&desc, &locked));
                    let _ = t.UnlockRect(face, level);
                }
            }
        } else {
            let t: IDirect3DTexture9 = texture.cast().expect("a texture");
            levels = t.GetLevelCount();
            for level in 0..levels {
                let (mut desc, mut locked) =
                    (D3DSURFACE_DESC::default(), D3DLOCKED_RECT::default());
                t.GetLevelDesc(level, &mut desc)
                    .expect("GetLevelDesc of a real texture");
                t.LockRect(
                    level,
                    &mut locked,
                    std::ptr::null(),
                    D3DLOCK_READONLY as u32,
                )
                .expect("LockRect of a real texture");
                surfaces.push(packed_level(&desc, &locked));
                let _ = t.UnlockRect(level);
            }
        }
    }
    (levels, surfaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The game's device behaviour flags: hardware vertex processing, pure, multithreaded.
    const GAME_BEHAVIOR: u32 = 0x54;

    /// A real HAL device created as the game creates its own (behaviour [`GAME_BEHAVIOR`], the
    /// layer's `D3DCREATE_FPU_PRESERVE`; `d3d9real::test_real_device`) on a hidden window;
    /// `None` where there is none (no display adapter, as on CI).
    fn test_device() -> Option<windows::Win32::Graphics::Direct3D9::IDirect3DDevice9> {
        super::super::d3d9real::test_real_device(&mut Engine::new(), 0x1000, GAME_BEHAVIOR)
    }

    /// The synthetic DDS of the test data (TEST DATA, made from the documented layout): one level
    /// of `width` x `height`, its bytes `(k * 2654435761 mod 2^32) >> 24` for byte `k`.
    fn synthetic_dds(width: u32, height: u32, format: &str) -> Vec<u8> {
        let mut v = vec![0u8; 128];
        let mut put = |at: usize, w: u32| v[at..at + 4].copy_from_slice(&w.to_le_bytes());
        put(4, 124);
        put(8, 0x000a_1007);
        put(12, height);
        put(16, width);
        put(28, 1);
        put(76, 32);
        put(108, 0x1000);
        let size = if format == "DXT5" {
            put(80, 4);
            put(84, u32::from_le_bytes(*b"DXT5"));
            width.div_ceil(4) * height.div_ceil(4) * 16
        } else {
            put(80, 0x41);
            put(88, 32);
            put(92, 0x00ff_0000);
            put(96, 0xff00);
            put(100, 0xff);
            put(104, 0xff00_0000);
            width * height * 4
        };
        v[..4].copy_from_slice(b"DDS ");
        v.extend((0..size).map(|k| (k.wrapping_mul(2_654_435_761) >> 24) as u8));
        v
    }

    /// A texture made through the 64-bit path from a small DDS in memory has the levels the
    /// 64-bit DLL gave for the same file in `d3dx-measure serve` (`data/d3dx/texture_levels_64.tsv`,
    /// 2026-11-03): a DXT5 file whose generated levels are the DLL's DXT encoder's (the 32-bit
    /// DLL's differ on them), and an A8R8G8B8 one. Skipped where the DLL or a display adapter is
    /// missing (as on CI).
    #[test]
    fn textures_through_the_64_bit_dll_match_its_capture() {
        if !textures_available() {
            return;
        }
        let Some(device) = test_device() else {
            return;
        };
        let text = include_str!("../data/d3dx/texture_levels_64.tsv");
        for (name, width, format) in [("dxt5-32x32", 32, "DXT5"), ("argb-16x16", 16, "ARGB")] {
            let want: Vec<Vec<&str>> = text
                .lines()
                .filter(|l| l.split('\t').next() == Some(name))
                .map(|l| l.split('\t').collect())
                .collect();
            assert!(!want.is_empty(), "{name} in the test data");
            let file = synthetic_dds(width, width, format);
            let got = texture_levels(&device, &file, false).expect("the DLL is loaded");
            assert_eq!(got.hr, u32::from_str_radix(want[0][1], 16).unwrap());
            assert_eq!(got.levels as usize, want.len(), "{name}: level count");
            assert_eq!(got.faces, 1);
            for (s, w) in got.surfaces.iter().zip(&want) {
                let bytes: Vec<u8> = (0..w[7].len() / 2)
                    .map(|i| u8::from_str_radix(&w[7][2 * i..2 * i + 2], 16).unwrap())
                    .collect();
                assert_eq!(
                    (s.format, s.width, s.height),
                    (
                        u32::from_str_radix(w[4], 16).unwrap(),
                        w[5].parse().unwrap(),
                        w[6].parse().unwrap()
                    ),
                    "{name} level {}",
                    w[3]
                );
                assert_eq!(
                    s.bytes.as_deref(),
                    Some(&bytes[..]),
                    "{name} level {}",
                    w[3]
                );
            }
            // The stored level is the file's.
            assert_eq!(
                got.surfaces[0].bytes.as_deref(),
                Some(&file[128..]),
                "{name}: level 0 as stored"
            );
            // The image info the DLL answers for the same file.
            let (hr, info) = image_info(&file, [0x5555_5555; 7]).unwrap();
            assert_eq!(hr, 0);
            assert_eq!(info[..4], [width, width, 1, 1]);
        }
        // A file the DLL refuses (cut short): its error, no levels.
        let mut short = synthetic_dds(32, 32, "DXT5");
        short.truncate(200);
        let refused = texture_levels(&device, &short, false).unwrap();
        assert_ne!(refused.hr, 0);
        assert!(refused.surfaces.is_empty());
    }

    /// A synthetic A8R8G8B8 cube DDS (TEST DATA, made from the documented layout): six faces of
    /// one `edge` x `edge` level each, the bytes as [`synthetic_dds`]'s over the six faces.
    fn synthetic_cube(edge: u32) -> Vec<u8> {
        let mut v = synthetic_dds(edge, edge, "ARGB");
        v.truncate(128);
        // DDSCAPS_COMPLEX | DDSCAPS_TEXTURE; DDSCAPS2_CUBEMAP with all six faces.
        v[108..112].copy_from_slice(&0x1008u32.to_le_bytes());
        v[112..116].copy_from_slice(&0xfe00u32.to_le_bytes());
        v.extend((0..6 * edge * edge * 4).map(|k| (k.wrapping_mul(2_654_435_761) >> 24) as u8));
        v
    }

    /// The game's calls, end to end through the layer with the 64-bit DLL: on a modelled device
    /// with a real one behind it, both made with the game's behaviour flags (0x54),
    /// `D3DXCreateTextureFromFileInMemory` (a DXT5 file whose other levels D3DX generates) and
    /// `D3DXCreateCubeTextureFromFileInMemory` (an A8R8G8B8 cube) answer `D3D_OK` and a texture of
    /// this layer whose level count, level descriptions and every level's bytes (read through the
    /// game's own `GetLevelCount`, `GetLevelDesc` and `LockRect`) are the DLL's for the same file;
    /// the layer's real counterpart holds the same levels; `D3DXGetImageInfoFromFileInMemory`
    /// answers the DLL's info; the log names the DLL. Skipped where the DLL or a display adapter
    /// is missing (as on CI).
    #[test]
    fn the_games_texture_calls_reach_the_layer_and_its_real_texture_through_the_dll() {
        use super::super::d3dx::{self, CachedSurface, TextureSource};
        if !textures_available() {
            return;
        }
        // CheckDeviceFormat of A8R8G8B8 and DXT5 textures and cube textures, usage 0, on the
        // display format X8R8G8B8: what the layer's CreateTexture and CreateCubeTexture ask.
        let extra = "\nDomain\td2\t15,35545844\nCheckDeviceFormat\t0 1 16 0 3\t0\t\td2\
                     \nCheckDeviceFormat\t0 1 16 0 5\t0\t\td2";
        let (mut e, device) = super::super::d3d9device::tests::device_with(extra, GAME_BEHAVIOR);
        let Some(real) = super::super::d3d9real::test_real_device(&mut e, device, GAME_BEHAVIOR)
        else {
            return;
        };
        // Bytes of one level with its rows packed, as the game reads them through a lock.
        let packed = |e: &Engine, s: &CachedSurface, pitch: u32, bits: u32| -> Vec<u8> {
            let block = super::super::d3d9device::format_blocks(s.format).map_or(1, |(b, _)| b);
            let rows = s.height.div_ceil(block).max(1);
            let row = s.bytes.as_ref().unwrap().len() as u32 / rows;
            (0..rows)
                .flat_map(|r| e.mem.bytes(bits + r * pitch, row))
                .collect()
        };
        for (file, cube) in [
            (synthetic_dds(32, 32, "DXT5"), false),
            (synthetic_cube(16), true),
        ] {
            let want = texture_levels(&real, &file, cube).expect("the DLL is loaded");
            assert_eq!(want.hr, 0);
            assert!(want.levels > 1, "D3DX generates the other levels");
            let data = e.mem.alloc(file.len() as u32);
            e.mem.write(data, &file);
            let out = e.mem.alloc(4);
            let call = if cube {
                d3dx::create_cube_texture_from_file_in_memory
            } else {
                d3dx::create_texture_from_file_in_memory
            };
            let hr = call(
                &mut e,
                Ptr::new(device),
                Ptr::new(data),
                file.len() as u32,
                Ptr::new(out),
            );
            assert_eq!(hr, 0, "cube {cube}");
            let texture = e.mem.u32(out);
            // The real counterpart, as the layer's uploads left it.
            let t =
                super::super::d3d9real::real_texture(&mut e, texture).expect("its real texture");
            let (levels, surfaces) = read_levels(&t, cube);
            assert_eq!(levels, want.levels, "cube {cube}: the real level count");
            for (i, (got, w)) in surfaces.iter().zip(&want.surfaces).enumerate() {
                assert_eq!(
                    (got.format, got.width, got.height, &got.bytes),
                    (w.format, w.width, w.height, &w.bytes),
                    "cube {cube}: the real texture's surface {i}"
                );
            }
            assert_eq!(surfaces.len(), want.surfaces.len());
            // The layer's texture, through the game's own calls (IDirect3DTexture9 and
            // IDirect3DCubeTexture9: GetLevelCount 13, GetLevelDesc 17, LockRect 19, UnlockRect 20).
            assert_eq!(e.vcall(texture, 4 * 13, &[]).u32(), want.levels);
            let (desc, locked) = (e.mem.alloc(32), e.mem.alloc(8));
            for (i, w) in want.surfaces.iter().enumerate() {
                let (face, level) = (i as u32 / want.levels, i as u32 % want.levels);
                assert_eq!(e.vcall(texture, 4 * 17, &[level, desc]).u32(), 0);
                assert_eq!(
                    (e.mem.u32(desc), e.mem.u32(desc + 24), e.mem.u32(desc + 28)),
                    (w.format, w.width, w.height),
                    "cube {cube}: level {level}'s description"
                );
                let (lock, unlock) = if cube {
                    (vec![face, level, locked, 0, 0x10], vec![face, level])
                } else {
                    (vec![level, locked, 0, 0x10], vec![level])
                };
                assert_eq!(e.vcall(texture, 4 * 19, &lock).u32(), 0);
                let (pitch, bits) = (e.mem.u32(locked), e.mem.u32(locked + 4));
                assert_eq!(
                    Some(packed(&e, w, pitch, bits)),
                    w.bytes,
                    "cube {cube}: face {face} level {level}'s bytes"
                );
                assert_eq!(e.vcall(texture, 4 * 20, &unlock).u32(), 0);
            }
            // The image info: the DLL's.
            let info = e.mem.alloc(28);
            for i in 0..7 {
                e.mem.set_u32(info + 4 * i, 0x5555_5555);
            }
            let hr = d3dx::get_image_info_from_file_in_memory(
                &mut e,
                Ptr::new(data),
                file.len() as u32,
                Ptr::new(info),
            );
            let (dll_hr, words) = image_info(&file, [0x5555_5555; 7]).unwrap();
            assert_eq!(hr, dll_hr);
            assert_eq!(
                (0..7).map(|i| e.mem.u32(info + 4 * i)).collect::<Vec<_>>(),
                words.to_vec(),
                "cube {cube}: the image info"
            );
        }
        assert_eq!(
            state!(e).graphics.d3dx_texture_sources,
            vec![TextureSource::Dll],
            "every call through the DLL, said once"
        );
    }

    /// The player's DLL compiles a small shader from engine memory, and its constant table
    /// answers through this layer's objects (skipped where the DLL is not installed).
    #[test]
    fn compiles_through_the_players_dll() {
        use windows::core::PCSTR;
        use windows::Win32::System::LibraryLoader::{LoadLibraryExA, LOAD_LIBRARY_SEARCH_SYSTEM32};
        if unsafe {
            LoadLibraryExA(
                PCSTR(b"d3dx9_38.dll\0".as_ptr()),
                None,
                LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        }
        .is_err()
        {
            return;
        }
        let mut e = Engine::new();
        let source =
            b"float4 tint; float4 main(float4 p : POSITION) : POSITION { return p * tint; }";
        let at = e.mem.alloc(source.len() as u32);
        e.mem.write(at, source);
        let main = e.mem.alloc(8);
        e.mem.write(main, b"main\0");
        let profile = e.mem.alloc(8);
        e.mem.write(profile, b"vs_2_0\0");
        let outs = e.mem.alloc(12);
        let hr = compile_shader(
            &mut e,
            Ptr::new(at),
            source.len() as u32,
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(main),
            Ptr::new(profile),
            0,
            Ptr::new(outs),
            Ptr::new(outs + 4),
            Ptr::new(outs + 8),
        );
        assert_eq!(hr, 0);
        let (shader, table) = (e.mem.u32(outs), e.mem.u32(outs + 8));
        assert_ne!(shader, 0);
        assert_eq!(e.mem.u32(outs + 4), 0, "no error text");
        // The bytecode starts with the vs_2_0 version token.
        let code = buffer_pointer(&mut e, Ptr::new(shader));
        assert_eq!(e.mem.u32(code), 0xfffe_0200);
        // One constant, `tint`, a float4 in register c0.
        let desc = e.mem.alloc(12);
        assert_eq!(table_get_desc(&mut e, Ptr::new(table), Ptr::new(desc)), 0);
        assert_eq!(e.mem.u32(desc + 8), 1);
        let handle = table_get_constant(&mut e, Ptr::new(table), 0, 0);
        assert_ne!(handle, 0);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"tint\0");
        assert_eq!(
            table_get_constant_by_name(&mut e, Ptr::new(table), 0, Ptr::new(name)),
            handle
        );
        let (d, count) = (e.mem.alloc(0x30), e.mem.alloc(4));
        e.mem.set_u32(count, 1);
        assert_eq!(
            table_get_constant_desc(
                &mut e,
                Ptr::new(table),
                handle,
                Ptr::new(d),
                Ptr::new(count)
            ),
            0
        );
        assert_eq!(e.mem.cstr(e.mem.u32(d)), b"tint");
        // RegisterSet D3DXRS_FLOAT4 (2), index 0, count 1.
        assert_eq!(
            [e.mem.u32(d + 4), e.mem.u32(d + 8), e.mem.u32(d + 12)],
            [2, 0, 1]
        );
        // The table read back from the bytecode answers the same.
        let out = e.mem.alloc(4);
        assert_eq!(
            get_shader_constant_table(&mut e, Ptr::new(code), Ptr::new(out)),
            0
        );
        assert_ne!(e.mem.u32(out), 0);
        let read_back = e.mem.u32(out);
        assert_eq!(table_release(&mut e, Ptr::new(read_back)), 0);
        assert_eq!(table_release(&mut e, Ptr::new(table)), 0);
        assert_eq!(buffer_release(&mut e, Ptr::new(shader)), 0);
    }
}
