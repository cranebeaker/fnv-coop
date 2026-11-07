//! `LoadLibraryA` and `FreeLibrary` for the modules the platform layer stands in for
//! (`D3D9.DLL`, `nvapi.dll`) and `GetProcAddress` on them ([`proc_address`], the core's
//! `loader::get_proc_address` asks through its `Layer`). Moved from the loader (`engine-core`'s
//! `units/platform/loader.rs`, which keeps the module handles, the KERNEL32 procedures found by
//! name and fiber-local storage).

use crate::{Engine, Ptr};

/// `ERROR_MOD_NOT_FOUND`.
const ERROR_MOD_NOT_FOUND: u32 = 126;

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
    // An AMD card's libraries: the multi-GPU probe's `ATIMGPUD.dll` (`Renderer::Init`, vendor word
    // 2) and, with bEnableEyefinity, ADL's two (`004dd2c0`). They are the player's 32-bit DLLs,
    // which this 64-bit process can't load, and the exe goes on without them. So on AMD the game
    // starts as on a card with one GPU and no Eyefinity: Windows' answer for a missing DLL.
    if matches!(
        text.as_str(),
        "ATIMGPUD.DLL" | "ATIADLXX.DLL" | "ATIADLXY.DLL"
    ) {
        e.kernel.last_error = ERROR_MOD_NOT_FOUND;
        return 0;
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

/// `GetProcAddress(module, name)` on a module [`load_library_a`] handed out: a `D3D9.DLL` one of
/// [`super::d3d9::PROCS`], or `nvapi.dll`'s `nvapi_QueryInterface`
/// ([`super::nvapi::proc_address`]); `None` for another module.
pub fn proc_address(e: &mut Engine, module: u32, text: &[u8]) -> Option<u32> {
    if module == super::d3d9::D3D9_MODULE {
        return Some(match super::d3d9::proc_address(e, text) {
            Some(address) => address,
            None => panic!(
                "GetProcAddress(D3D9, {:?}) is not provided",
                String::from_utf8_lossy(text)
            ),
        });
    }
    if module == super::nvapi::NVAPI_MODULE {
        return Some(super::nvapi::proc_address(e, text));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        e
    }

    /// An AMD card's libraries aren't there for the game, which goes on without them.
    #[test]
    fn an_amd_cards_libraries_answer_as_missing() {
        for name in ["ATIMGPUD.dll", "atiadlxx.dll", "atiadlxy.dll"] {
            let mut e = engine();
            e.mem.write(0x1000, name.as_bytes());
            e.mem.write(0x1000 + name.len() as u32, b"\0");
            assert_eq!(load_library_a(&mut e, Ptr::new(0x1000)), 0);
            assert_eq!(e.kernel.last_error, ERROR_MOD_NOT_FOUND);
        }
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
}
