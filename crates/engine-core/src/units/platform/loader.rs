//! The loader's side of the platform: module handles (`GetModuleHandleW`), procedures found by
//! name (`GetProcAddress`), and the KERNEL32 functions the CRT finds that way (`FlsAlloc`,
//! `FlsGetValue`, `FlsSetValue`, `FlsFree`, `EncodePointer`, `DecodePointer`; the CRT's
//! `__mtinit` `00ed1327` and `__encode_pointer` look them up rather than importing them; and
//! `IsProcessorFeaturePresent`, which `_ms_p5_mp_test_fdiv` `00ed2518` looks up).
//!
//! A procedure found by name has a code address of the platform's ([`PROC_CODE`] up), registered
//! the first time it is asked for; a name this layer does not provide stops the run, never a
//! made-up null.

use crate::{Engine, Ptr};

/// The module handle of KERNEL32 this layer gives out (a value no image uses), and the
/// executable's own (its image base, `GetModuleHandleW(NULL)`).
pub const KERNEL32_MODULE: u32 = 0x7e10_0000;
pub const IMAGE_MODULE: u32 = 0x0040_0000;
/// Where the procedures [`get_proc_address`] gives out sit.
pub const PROC_CODE: u32 = 0x7e01_0000;
/// Fiber-local storage indexes Windows provides (`FLS_MAXIMUM_AVAILABLE`), `FlsAlloc`'s failure.
const FLS_INDEXES: usize = 128;
const FLS_OUT_OF_INDEXES: u32 = 0xffff_ffff;

/// The fiber-local storage (`Kernel::fls`): the indexes handed out (with their callbacks) and
/// the values by (thread id, index). Each game thread is one fiber.
#[derive(Debug, Default)]
pub struct Fls {
    allocated: Vec<Option<u32>>,
    values: std::collections::BTreeMap<(u32, u32), u32>,
    registered: bool,
}

/// The KERNEL32 procedures provided by name, in the order of their code addresses.
const PROCS: [&[u8]; 7] = [
    b"FlsAlloc",
    b"FlsGetValue",
    b"FlsSetValue",
    b"FlsFree",
    b"EncodePointer",
    b"DecodePointer",
    b"IsProcessorFeaturePresent",
];

fn register_procs(e: &mut Engine) {
    let procs: [crate::AbiFn; 7] = [
        crate::entry!(0, fls_alloc(u32) -> u32).1,
        crate::entry!(0, fls_get_value(u32) -> u32).1,
        crate::entry!(0, fls_set_value(u32, u32) -> i32).1,
        crate::entry!(0, fls_free(u32) -> i32).1,
        crate::entry!(0, encode_pointer(u32) -> u32).1,
        crate::entry!(0, decode_pointer(u32) -> u32).1,
        crate::entry!(0, is_processor_feature_present(u32) -> u32).1,
    ];
    for (i, f) in procs.into_iter().enumerate() {
        e.register(PROC_CODE + 4 * i as u32, f);
    }
    e.kernel.fls.registered = true;
}

/// A NUL-terminated UTF-16 string from engine memory (its code units).
fn wide(e: &Engine, at: u32) -> Vec<u16> {
    let mut out = Vec::new();
    let mut p = at;
    loop {
        let unit = e.mem.u16(p);
        if unit == 0 {
            return out;
        }
        out.push(unit);
        p = p.wrapping_add(2);
    }
}

// Platform: replaces KERNEL32.DLL GetModuleHandleW (import)
/// `GetModuleHandleW(name)`: no name is the executable (its image base); `KERNEL32.DLL` (any
/// case, with or without `.DLL`) is [`KERNEL32_MODULE`]. Other modules are not provided.
pub fn get_module_handle_w(e: &mut Engine, name: Ptr) -> u32 {
    if name.is_null() {
        return IMAGE_MODULE;
    }
    let units = wide(e, name.addr());
    let text: String = char::decode_utf16(units)
        .map(|c| c.unwrap_or('?'))
        .collect::<String>()
        .to_ascii_uppercase();
    if text == "KERNEL32.DLL" || text == "KERNEL32" {
        return KERNEL32_MODULE;
    }
    panic!("GetModuleHandleW({text:?}) is not provided");
}

// Platform: replaces KERNEL32.DLL GetModuleHandleA (import)
/// `GetModuleHandleA(name)`: as [`get_module_handle_w`] for an ANSI name (`_ms_p5_mp_test_fdiv`
/// asks for `KERNEL32`).
pub fn get_module_handle_a(e: &mut Engine, name: Ptr) -> u32 {
    if name.is_null() {
        return IMAGE_MODULE;
    }
    let text = String::from_utf8_lossy(&e.mem.cstr(name.addr())).to_ascii_uppercase();
    if text == "KERNEL32.DLL" || text == "KERNEL32" {
        return KERNEL32_MODULE;
    }
    panic!("GetModuleHandleA({text:?}) is not provided");
}

/// `IsProcessorFeaturePresent(feature)` (stdcall): the player's machine answers 0 for
/// `PF_FLOATING_POINT_PRECISION_ERRATA` (0, measured), the only feature the game asks
/// about; any other feature is not provided.
fn is_processor_feature_present(_e: &mut Engine, feature: u32) -> u32 {
    if feature == 0 {
        return 0;
    }
    panic!("IsProcessorFeaturePresent({feature}) is not provided");
}

// Platform: replaces KERNEL32.DLL LoadLibraryA (import)
/// `LoadLibraryA(name)`: `D3D9.DLL` (any case, with or without `.DLL`, no path) is
/// [`super::d3d9::D3D9_MODULE`], the platform's Direct3D 9 layer; `nvapi.dll` answers from the
/// machine table (`super::nvapi::load_library`). Other modules are not provided.
pub fn load_library_a(e: &mut Engine, name: Ptr) -> u32 {
    let text = String::from_utf8_lossy(&e.mem.cstr(name.addr())).to_ascii_uppercase();
    if text == "D3D9.DLL" || text == "D3D9" {
        return super::d3d9::D3D9_MODULE;
    }
    if text == "NVAPI.DLL" || text == "NVAPI" {
        return super::nvapi::load_library(e);
    }
    panic!("LoadLibraryA({text:?}) is not provided");
}

// Platform: replaces KERNEL32.DLL FreeLibrary (import)
/// `FreeLibrary(module)` for a module [`load_library_a`] handed out (`D3D9.DLL`, `nvapi.dll`):
/// nonzero, as Windows answers when the module's load count goes down. Nothing is unloaded: those
/// modules are the platform's own layers, whose objects the game releases itself. The renderer
/// frees its `D3D9.DLL` this way when the game exits (`00e6ec6d`). Another module is not
/// provided.
pub fn free_library(_e: &mut Engine, module: u32) -> u32 {
    if module == super::d3d9::D3D9_MODULE || module == super::nvapi::NVAPI_MODULE {
        return 1;
    }
    panic!("FreeLibrary({module:#x}) is not provided");
}

// Platform: replaces KERNEL32.DLL GetProcAddress (import)
/// `GetProcAddress(module, name)`: a KERNEL32 procedure of [`PROCS`] by name, its platform code
/// address, a `D3D9.DLL` one of [`super::d3d9::PROCS`], or `nvapi.dll`'s `nvapi_QueryInterface`
/// ([`super::nvapi::proc_address`]). A procedure by ordinal, another module or another name is
/// not provided.
pub fn get_proc_address(e: &mut Engine, module: u32, name: u32) -> u32 {
    if name < 0x1_0000 {
        panic!("GetProcAddress by ordinal ({name}) is not provided");
    }
    let text = e.mem.cstr(name);
    if module == super::d3d9::D3D9_MODULE {
        return match super::d3d9::proc_address(e, &text) {
            Some(address) => address,
            None => panic!(
                "GetProcAddress(D3D9, {:?}) is not provided",
                String::from_utf8_lossy(&text)
            ),
        };
    }
    if module == super::nvapi::NVAPI_MODULE {
        return super::nvapi::proc_address(e, &text);
    }
    if module != KERNEL32_MODULE {
        panic!("GetProcAddress on module {module:#x} is not provided");
    }
    let Some(i) = PROCS.iter().position(|p| *p == text.as_slice()) else {
        panic!(
            "GetProcAddress(KERNEL32, {:?}) is not provided",
            String::from_utf8_lossy(&text)
        );
    };
    if !e.kernel.fls.registered {
        register_procs(e);
    }
    PROC_CODE + 4 * i as u32
}

/// `FlsAlloc(callback)`: the lowest free index (its value 0 on every thread), or
/// `FLS_OUT_OF_INDEXES`. The callback is kept; no fiber or thread here ever ends in a way that
/// would call it.
fn fls_alloc(e: &mut Engine, callback: u32) -> u32 {
    let fls = &mut e.kernel.fls;
    if fls.allocated.is_empty() {
        fls.allocated.resize(FLS_INDEXES, None);
    }
    match fls.allocated.iter().position(|a| a.is_none()) {
        Some(i) => {
            fls.allocated[i] = Some(callback);
            i as u32
        }
        None => FLS_OUT_OF_INDEXES,
    }
}

fn fls_in_use(e: &Engine, index: u32) -> bool {
    matches!(e.kernel.fls.allocated.get(index as usize), Some(Some(_)))
}

/// `FlsGetValue(index)`: the calling thread's value (0 when never set).
fn fls_get_value(e: &mut Engine, index: u32) -> u32 {
    let thread = super::get_current_thread_id(e);
    e.kernel
        .fls
        .values
        .get(&(thread, index))
        .copied()
        .unwrap_or(0)
}

/// `FlsSetValue(index, value)`: 0 for an index not allocated.
fn fls_set_value(e: &mut Engine, index: u32, value: u32) -> i32 {
    if !fls_in_use(e, index) {
        return 0;
    }
    let thread = super::get_current_thread_id(e);
    e.kernel.fls.values.insert((thread, index), value);
    1
}

/// `FlsFree(index)`: frees it and forgets its values; 0 for an index not allocated. (Windows
/// calls the callback for each thread's non-null value; the game frees its index only at exit.)
fn fls_free(e: &mut Engine, index: u32) -> i32 {
    if !fls_in_use(e, index) {
        return 0;
    }
    e.kernel.fls.allocated[index as usize] = None;
    e.kernel.fls.values.retain(|&(_, i), _| i != index);
    1
}

/// `EncodePointer(p)` / `DecodePointer(p)`: Windows combines the pointer with a secret it picks
/// for each process; the platform's secret makes the encoding the identity, so
/// `DecodePointer(EncodePointer(p)) == p` as on Windows and runs are reproducible.
fn encode_pointer(_e: &mut Engine, pointer: u32) -> u32 {
    pointer
}

fn decode_pointer(_e: &mut Engine, pointer: u32) -> u32 {
    pointer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        e
    }

    /// The renderer frees the `D3D9.DLL` it loaded when the game exits (00e6ec6d): the module
    /// handle the loader gave it is freed with a nonzero answer.
    #[test]
    fn free_library_answers_for_the_modules_the_loader_handed_out() {
        let mut e = engine();
        e.mem.write(0x1000, b"d3d9.dll\0");
        let module = load_library_a(&mut e, Ptr::new(0x1000));
        assert_eq!(free_library(&mut e, module), 1);
        assert_eq!(free_library(&mut e, super::super::nvapi::NVAPI_MODULE), 1);
    }

    fn put_wide(e: &mut Engine, at: u32, text: &str) {
        for (i, unit) in text.encode_utf16().chain([0]).enumerate() {
            e.mem.set_u16(at + 2 * i as u32, unit);
        }
    }

    #[test]
    fn module_handles() {
        let mut e = engine();
        assert_eq!(get_module_handle_w(&mut e, Ptr::NULL), IMAGE_MODULE);
        put_wide(&mut e, 0x1000, "kernel32.dll");
        assert_eq!(
            get_module_handle_w(&mut e, Ptr::new(0x1000)),
            KERNEL32_MODULE
        );
    }

    /// The CRT's lookups: each name gives a code address that runs the procedure; the fiber
    /// storage keeps a value per thread; pointers come back decoded.
    #[test]
    fn procedures_by_name_run_their_stand_ins() {
        let mut e = engine();
        e.mem.map(0x0001_1000, 0x1000);
        let address = |e: &mut Engine, name: &[u8]| {
            e.mem.set_cstr(0x0001_1100, name);
            get_proc_address(e, KERNEL32_MODULE, 0x0001_1100)
        };
        let alloc = address(&mut e, b"FlsAlloc");
        let get = address(&mut e, b"FlsGetValue");
        let set = address(&mut e, b"FlsSetValue");
        let free = address(&mut e, b"FlsFree");
        let encode = address(&mut e, b"EncodePointer");
        let decode = address(&mut e, b"DecodePointer");
        assert_eq!(alloc, PROC_CODE);
        let index = e.call(alloc, &[0x00ed_1000]).u32();
        assert_eq!(index, 0);
        assert_eq!(e.call(get, &[index]).u32(), 0);
        assert_eq!(e.call(set, &[index, 0x1234]).u32(), 1);
        assert_eq!(e.call(get, &[index]).u32(), 0x1234);
        assert_eq!(e.call(set, &[5, 1]).u32(), 0, "not allocated");
        let encoded = e.call(encode, &[0x00ed_1004]).u32();
        assert_eq!(e.call(decode, &[encoded]).u32(), 0x00ed_1004);
        assert_eq!(e.call(free, &[index]).u32(), 1);
        assert_eq!(e.call(free, &[index]).u32(), 0);
    }

    /// `_ms_p5_mp_test_fdiv`'s lookup: `GetModuleHandleA("KERNEL32")`, then
    /// `IsProcessorFeaturePresent(0)`, which answers 0 as on the player's machine.
    #[test]
    fn the_fdiv_errata_query_answers_no() {
        let mut e = engine();
        e.mem.map(0x0001_1000, 0x1000);
        e.mem.set_cstr(0x0001_1000, b"KERNEL32");
        let module = get_module_handle_a(&mut e, Ptr::new(0x0001_1000));
        assert_eq!(module, KERNEL32_MODULE);
        assert_eq!(get_module_handle_a(&mut e, Ptr::NULL), IMAGE_MODULE);
        e.mem.set_cstr(0x0001_1100, b"IsProcessorFeaturePresent");
        let present = get_proc_address(&mut e, module, 0x0001_1100);
        assert_eq!(present, PROC_CODE + 4 * 6);
        assert_eq!(e.call(present, &[0]).u32(), 0);
    }

    #[test]
    #[should_panic(expected = "IsProcessorFeaturePresent(10) is not provided")]
    fn other_processor_features_stop() {
        let mut e = engine();
        e.mem.map(0x0001_1000, 0x1000);
        e.mem.set_cstr(0x0001_1100, b"IsProcessorFeaturePresent");
        let present = get_proc_address(&mut e, KERNEL32_MODULE, 0x0001_1100);
        e.call(present, &[10]);
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn an_unknown_procedure_stops() {
        let mut e = engine();
        e.mem.map(0x0001_1000, 0x1000);
        e.mem.set_cstr(0x0001_1100, b"CreateFiber");
        get_proc_address(&mut e, KERNEL32_MODULE, 0x0001_1100);
    }
}
