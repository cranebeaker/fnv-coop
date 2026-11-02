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

/// The player's `d3dx9_38.dll`, loaded once from the system folder.
fn dll() -> &'static Dll {
    static DLL: std::sync::OnceLock<Option<Dll>> = std::sync::OnceLock::new();
    DLL.get_or_init(|| unsafe {
        use windows::core::PCSTR;
        use windows::Win32::System::LibraryLoader::{
            GetProcAddress, LoadLibraryExA, LOAD_LIBRARY_SEARCH_SYSTEM32,
        };
        let module = LoadLibraryExA(
            PCSTR(b"d3dx9_38.dll\0".as_ptr()),
            None,
            LOAD_LIBRARY_SEARCH_SYSTEM32,
        )
        .ok()?;
        let compile = GetProcAddress(module, PCSTR(b"D3DXCompileShader\0".as_ptr()))?;
        let table = GetProcAddress(module, PCSTR(b"D3DXGetShaderConstantTable\0".as_ptr()))?;
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
    &mut e.kernel.graphics.d3dx
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
    use windows::core::PCSTR;
    use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
    dll();
    unsafe {
        let module =
            GetModuleHandleA(PCSTR(b"d3dx9_38.dll\0".as_ptr())).expect("d3dx9_38.dll is loaded");
        GetProcAddress(module, PCSTR(name.as_ptr()))
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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
