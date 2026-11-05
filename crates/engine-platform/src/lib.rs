//! The platform layer of the engine (ADR-0006, docs/ENGINE_CRATE.md): the Windows imports the
//! game calls and the objects behind them, replaced by Rust (docs/ENGINE_PORT_PLAN.md,
//! "platform"). Hand-written, owned by the lead.
//!
//! The window, Direct3D 9 and D3DX (modelled, and forwarded to the real `d3d9.dll` with
//! `NV_D3D9=real`), DirectInput, DirectSound with its mixer and resampler, DirectShow, Bink,
//! Ogg Vorbis, `mmio`, files, `LoadLibraryA`, the process side and NVAPI. Its core, the kernel
//! objects, threads, `VirtualAlloc`, the memory manager, the clock and the loader's module
//! handles and fiber-local storage, stays in `engine-core`
//! (`units/platform.rs`), which the engine part crates' tests need; this crate is the rest, which
//! only the `engine` facade and the host depend on, so a change here rebuilds no part crate.
//!
//! It joins the core through `engine_core::units::platform::Layer` ([`LAYER`]): its imports
//! come after the core's in `imports`, its [`State`] is each engine's `Kernel::platform`, and
//! `CloseHandle` on one of its file handles runs [`files::closed`], and `GetProcAddress` on a module
//! it loaded asks [`library::proc_address`]. [`units::funcs`] (which
//! `Engine::new` runs in a program that links this crate) installs it.
//!
//! The files were `engine-core`'s `units/platform/` folder; the core's items are re-exported
//! here, so `super::<item>` and `crate::<module>` reach them as before.

// The same lints as `engine-core` allows, for the same reasons (see its `lib.rs`).
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::manual_saturating_arithmetic)]
#![allow(clippy::eq_op)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::assign_op_pattern)]
#![cfg_attr(test, allow(clippy::type_complexity))]
#![cfg_attr(test, allow(clippy::useless_vec))]

#[allow(unused_imports)]
use crate::prelude::*;

// The core's model and the core of the platform layer, as the files reached them in `engine-core`.
pub use engine_core::units::platform::*;
pub use engine_core::{abi, exe, mem, prelude, ptr, queue_check, sse, thread_host, types, x87};
pub use engine_core::{args, entry, layout};
pub use engine_core::{AbiFn, Engine, Field, Inline, Layout, Mem, Ptr, Ret, Scalar};

/// This layer's [`State`] in the engine `$e` (`&mut`): `state!(e).sound`, as `e.kernel.sound`
/// was. It borrows only `$e.kernel.platform`, so the engine's other fields stay free.
macro_rules! state {
    ($e:expr) => {
        $crate::state_mut(&mut $e.kernel.platform)
    };
}

/// [`state!`] to read, where `$e` is `&Engine` or something else in it is borrowed beside it.
macro_rules! state_ref {
    ($e:expr) => {
        $crate::state(&$e.kernel.platform)
    };
}

#[cfg(test)]
pub(crate) mod alloc_count;
pub mod bink;
#[cfg(test)]
mod crt_stdio_tests;
pub mod d3d9;
pub mod d3d9device;
pub mod d3d9query;
#[cfg(windows)]
pub mod d3d9real;
pub mod d3d9resource;
pub mod d3dx;
#[cfg(windows)]
pub mod d3dxreal;
pub mod dinput;
pub mod ds3d;
pub mod dsfx;
pub mod dshow;
pub mod dsound;
pub mod files;
pub mod ini;
pub mod library;
pub mod mixer;
pub mod mmio;
pub mod nls;
pub mod nvapi;
pub mod process;
pub mod resampler;
pub mod sound_out;
pub mod speakers;
pub mod time;
pub mod vorbisfile;
pub mod window;
pub mod zone;

/// This layer's state in each engine (`Kernel::platform`; made by [`LAYER`]'s `new_state`).
#[derive(Debug, Default)]
pub struct State {
    /// The DirectInput 8 objects and the keyboard and mouse state behind them ([`dinput`]).
    pub input: dinput::Input,
    /// The DirectSound 8 objects ([`dsound`]).
    pub sound: dsound::Sound,
    /// The Direct3D 9 objects ([`d3d9`]).
    pub graphics: d3d9::Graphics,
    /// Window classes, windows, the message queue and the command line ([`window`]).
    pub windows: window::Windows,
    /// The files the game made in this run ([`files`]).
    pub disk: files::Disk,
    /// The process side: the ANSI command line and the environment blocks ([`process`]).
    pub process: process::Process,
}

/// The [`State`] in an engine's `Kernel::platform`.
#[inline]
pub fn state(slot: &Option<Box<dyn std::any::Any>>) -> &State {
    let any = slot
        .as_deref()
        .expect("the engine has the platform layer's state");
    debug_assert!(any.is::<State>());
    // SAFETY: only `Engine::new` fills `Kernel::platform`, from the one `LAYER`'s `new_state`,
    // which makes a `State`. This runs on every platform call, so no `TypeId` check in release.
    unsafe { &*(any as *const dyn std::any::Any as *const State) }
}

/// The [`State`] in an engine's `Kernel::platform`, to change.
#[inline]
pub fn state_mut(slot: &mut Option<Box<dyn std::any::Any>>) -> &mut State {
    let any = slot
        .as_deref_mut()
        .expect("the engine has the platform layer's state");
    debug_assert!(any.is::<State>());
    // SAFETY: as in [`state`].
    unsafe { &mut *(any as *mut dyn std::any::Any as *mut State) }
}

fn new_state() -> Box<dyn std::any::Any> {
    Box::<State>::default()
}

/// `CloseHandle` on a file handle: [`files::closed`].
fn file_closed(e: &mut Engine, file: &Opaque) {
    if let Some(file) = file.get::<files::OpenFile>() {
        files::closed(e, file);
    }
}

/// This layer, as the core takes it.
pub static LAYER: Layer = Layer {
    imports: layer_imports,
    new_state,
    file_closed,
    proc_address: library::proc_address,
};

/// This crate's registered functions, as the `engine` facade takes each engine crate's.
pub mod units {
    /// The functions this layer registers at exe addresses; installs the layer for the
    /// engines made from now on (`Engine::new` runs this before it makes the kernel).
    pub fn funcs(out: &mut Vec<(u32, crate::AbiFn)>) {
        crate::install_layer(&crate::LAYER);
        out.extend(crate::layer_funcs());
    }
}

fn layer_funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_193c, d3d9::cpu_id_flag_test() -> u32),
        entry!(0x00ed_1951, d3d9::cpuid(u32, Ptr)),
        entry!(0x00ed_1964, d3d9::cpuid(u32, Ptr)),
    ]
}

/// (DLL, name, platform version) of this layer's imports with a Rust stand-in; the core's
/// `imports` answers them after its own.
fn layer_imports() -> Vec<(&'static str, &'static str, AbiFn)> {
    let k = "KERNEL32.dll";
    #[allow(unused_mut)]
    let mut imports = vec![
        (
            k,
            "GetPrivateProfileStringA",
            entry!(0, ini::get_private_profile_string_a(Ptr, Ptr, Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            k,
            "WritePrivateProfileStringA",
            entry!(0, ini::write_private_profile_string_a(Ptr, Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetAsyncKeyState",
            entry!(0, dinput::get_async_key_state(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetSystemMetrics",
            entry!(0, window::get_system_metrics(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "EnumDisplayDevicesA",
            entry!(0, d3d9::enum_display_devices_a(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "LoadLibraryA",
            entry!(0, library::load_library_a(Ptr) -> u32).1,
        ),
        (k, "FreeLibrary", entry!(0, library::free_library(u32) -> u32).1),
        (
            "d3dx9_38.dll",
            "D3DXMatrixMultiply",
            entry!(0, d3dx::matrix_multiply(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixMultiplyTranspose",
            entry!(0, d3dx::matrix_multiply_transpose(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixRotationYawPitchRoll",
            entry!(0, d3dx::matrix_rotation_yaw_pitch_roll(Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixTranspose",
            entry!(0, d3dx::matrix_transpose(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec3TransformNormal",
            entry!(0, d3dx::vec3_transform_normal(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec3Normalize",
            entry!(0, d3dx::vec3_normalize(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec3TransformCoord",
            entry!(0, d3dx::vec3_transform_coord(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixInverse",
            entry!(0, d3dx::matrix_inverse(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXPlaneNormalize",
            entry!(0, d3dx::plane_normalize(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXPlaneTransform",
            entry!(0, d3dx::plane_transform(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec4Transform",
            entry!(0, d3dx::vec4_transform_import(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetImageInfoFromFileInMemory",
            entry!(0, d3dx::get_image_info_from_file_in_memory(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXCreateTextureFromFileInMemory",
            entry!(0, d3dx::create_texture_from_file_in_memory(Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXCreateCubeTextureFromFileInMemory",
            entry!(0, d3dx::create_cube_texture_from_file_in_memory(Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            k,
            "CreateMutexA",
            entry!(0, window::create_mutex_a(u32, u32, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "MessageBoxA",
            entry!(0, window::message_box_a(u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (k, "ExitProcess", entry!(0, window::exit_process(u32)).1),
        (
            "SHELL32.dll",
            "ShellExecuteA",
            entry!(0, window::shell_execute_a(u32, u32, Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "IsDebuggerPresent",
            entry!(0, window::is_debugger_present() -> u32).1,
        ),
        (
            "USER32.dll",
            "FindWindowA",
            entry!(0, window::find_window_a(u32, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "SetForegroundWindow",
            entry!(0, window::set_foreground_window(u32) -> u32).1,
        ),
        (
            k,
            "GetCommandLineW",
            entry!(0, window::get_command_line_w() -> u32).1,
        ),
        (
            "SHELL32.dll",
            "CommandLineToArgvW",
            entry!(0, window::command_line_to_argv_w(Ptr, Ptr) -> u32).1,
        ),
        (k, "LocalFree", entry!(0, window::local_free(u32) -> u32).1),
        (
            "USER32.dll",
            "LoadIconA",
            entry!(0, window::load_icon_a(u32, u32) -> u32).1,
        ),
        (
            "GDI32.dll",
            "GetStockObject",
            entry!(0, window::get_stock_object(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "RegisterClassA",
            entry!(0, window::register_class_a(Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "AdjustWindowRect",
            entry!(0, window::adjust_window_rect(Ptr, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "AdjustWindowRectEx",
            entry!(0, window::adjust_window_rect_ex(Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "CreateWindowExA",
            entry!(0, window::create_window_ex_a(u32, u32, Ptr, u32, i32, i32, i32, i32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetWindowLongA",
            entry!(0, window::get_window_long_a(u32, i32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetClassLongA",
            entry!(0, window::get_class_long_a(u32, i32) -> u32).1,
        ),
        (
            "USER32.dll",
            "SetWindowTextA",
            entry!(0, window::set_window_text_a(u32, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "SetWindowPos",
            entry!(0, window::set_window_pos(u32, u32, i32, i32, i32, i32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "PeekMessageA",
            entry!(0, window::peek_message_a(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "TranslateMessage",
            entry!(0, window::translate_message(Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "DispatchMessageA",
            entry!(0, window::dispatch_message_a(Ptr) -> u32).1,
        ),
        (
            k,
            "GetPrivateProfileIntA",
            entry!(0, ini::get_private_profile_int_a(Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            k,
            "GetFileAttributesA",
            entry!(0, files::get_file_attributes_a(Ptr) -> u32).1,
        ),
        (
            k,
            "CreateDirectoryA",
            entry!(0, files::create_directory_a(Ptr, u32) -> u32).1,
        ),
        (k, "DeleteFileA", entry!(0, files::delete_file_a(Ptr) -> u32).1),
        (
            k,
            "CopyFileA",
            entry!(0, files::copy_file_a(Ptr, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "MoveFileA",
            entry!(0, files::move_file_a(Ptr, Ptr) -> u32).1,
        ),
        (
            "SHELL32.dll",
            "SHGetFolderPathA",
            entry!(0, files::sh_get_folder_path_a(u32, u32, u32, u32, Ptr) -> u32).1,
        ),
        (
            "ADVAPI32.dll",
            "GetUserNameA",
            entry!(0, window::get_user_name_a(Ptr, Ptr) -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_Init",
            entry!(0, window::steam_api_init() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_RegisterCallback",
            entry!(0, window::steam_api_register_callback(Ptr, u32)).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_UnregisterCallback",
            entry!(0, window::steam_api_unregister_callback(Ptr)).1,
        ),
        // The platform5 imports (slot a04): begin.
        (
            "steam_api.dll",
            "SteamAPI_IsSteamRunning",
            entry!(0, window::steam_api_is_steam_running() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamUserStats",
            entry!(0, window::steam_user_stats() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamFriends",
            entry!(0, window::steam_friends() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamUtils",
            entry!(0, window::steam_utils() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamUser",
            entry!(0, window::steam_user() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_RunCallbacks",
            entry!(0, window::steam_api_run_callbacks()).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_Shutdown",
            entry!(0, window::steam_api_shutdown()).1,
        ),
        (
            "steam_api.dll",
            "SteamApps",
            entry!(0, window::steam_apps() -> u32).1,
        ),
        (
            k,
            "GetModuleFileNameA",
            entry!(0, process::get_module_file_name_a(u32, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "GetCommandLineA",
            entry!(0, process::get_command_line_a() -> u32).1,
        ),
        (
            k,
            "GetEnvironmentStringsW",
            entry!(0, process::get_environment_strings_w() -> u32).1,
        ),
        (
            k,
            "FreeEnvironmentStringsW",
            entry!(0, process::free_environment_strings_w(u32) -> u32).1,
        ),
        (
            k,
            "GetCurrentProcessId",
            entry!(0, process::get_current_process_id() -> u32).1,
        ),
        (
            k,
            "GlobalMemoryStatus",
            entry!(0, process::global_memory_status(Ptr)).1,
        ),
        (
            k,
            "GlobalMemoryStatusEx",
            entry!(0, process::global_memory_status_ex(Ptr) -> u32).1,
        ),
        (
            k,
            "HeapFree",
            entry!(0, process::heap_free(u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "HeapReAlloc",
            entry!(0, process::heap_re_alloc(u32, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "CreateFileW",
            entry!(0, files::create_file_w(Ptr, u32, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "GetFileSizeEx",
            entry!(0, files::get_file_size_ex(u32, Ptr) -> u32).1,
        ),
        (
            k,
            "SystemTimeToTzSpecificLocalTime",
            entry!(0, time::system_time_to_tz_specific_local_time(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "SetUnhandledExceptionFilter",
            entry!(0, process::set_unhandled_exception_filter(u32) -> u32).1,
        ),
        (
            k,
            "GetStringTypeW",
            entry!(0, nls::get_string_type_w(u32, Ptr, i32, Ptr) -> u32).1,
        ),
        // The platform5 imports: end.
        (
            "DSOUND.dll",
            "#11",
            entry!(0, dsound::direct_sound_create8(u32, Ptr, u32) -> u32).1,
        ),
        (
            "ole32.dll",
            "CoInitialize",
            entry!(0, dsound::co_initialize(u32) -> u32).1,
        ),
        (
            "DINPUT8.dll",
            "DirectInput8Create",
            entry!(0, dinput::direct_input8_create(u32, u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "CreateFileA",
            entry!(0, files::create_file_a(Ptr, u32, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (k, "GetFileType", entry!(0, files::get_file_type(u32) -> u32).1),
        (
            k,
            "SetFilePointer",
            entry!(0, files::set_file_pointer(u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "WriteFile",
            entry!(0, files::write_file(u32, Ptr, u32, Ptr, u32) -> i32).1,
        ),
        (
            k,
            "ReadFile",
            entry!(0, files::read_file(u32, Ptr, u32, Ptr, u32) -> i32).1,
        ),
        (
            k,
            "FindFirstFileA",
            entry!(0, files::find_first_file_a(Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "FindNextFileA",
            entry!(0, files::find_next_file_a(u32, Ptr) -> u32).1,
        ),
        (
            k,
            "CompareFileTime",
            entry!(0, files::compare_file_time(Ptr, Ptr) -> i32).1,
        ),
        (k, "FindClose", entry!(0, files::find_close(u32) -> u32).1),
        (
            k,
            "GetFullPathNameA",
            entry!(0, files::get_full_path_name_a(Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "GetSystemTimeAsFileTime",
            entry!(0, time::get_system_time_as_file_time(Ptr)).1,
        ),
        (
            k,
            "GetTimeZoneInformation",
            entry!(0, time::get_time_zone_information(Ptr) -> u32).1,
        ),
        (k, "GetACP", entry!(0, d3d9::get_acp() -> u32).1),
        (k, "GetOEMCP", entry!(0, d3d9::get_oemcp() -> u32).1),
        (
            k,
            "IsValidCodePage",
            entry!(0, d3d9::is_valid_code_page(u32) -> u32).1,
        ),
        (k, "GetCPInfo", entry!(0, d3d9::get_cp_info(u32, Ptr) -> u32).1),
        (
            k,
            "FileTimeToLocalFileTime",
            entry!(0, time::file_time_to_local_file_time(Ptr, Ptr) -> u32).1,
        ),
        (k, "GetLocalTime", entry!(0, time::get_local_time(Ptr)).1),
        (
            k,
            "FileTimeToSystemTime",
            entry!(0, time::file_time_to_system_time(Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "WideCharToMultiByte",
            entry!(0, time::wide_char_to_multi_byte(u32, u32, Ptr, i32, Ptr, i32, Ptr, Ptr) -> u32).1,
        ),
        // The Bink imports (`bink`, slot a11): begin.
        (
            "binkw32.dll",
            "_BinkSetSoundSystem@8",
            entry!(0, bink::set_sound_system(u32, u32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkOpenDirectSound@4",
            entry!(0, bink::open_direct_sound(u32) -> u32).1,
        ),
        (
            "binkw32.dll",
            "_BinkOpen@8",
            entry!(0, bink::open(Ptr, u32) -> u32).1,
        ),
        (
            "binkw32.dll",
            "_BinkClose@4",
            entry!(0, bink::close(u32)).1,
        ),
        (
            "binkw32.dll",
            "_BinkDoFrame@4",
            entry!(0, bink::do_frame(u32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkNextFrame@4",
            entry!(0, bink::next_frame(u32)).1,
        ),
        (
            "binkw32.dll",
            "_BinkWait@4",
            entry!(0, bink::wait(u32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkPause@8",
            entry!(0, bink::pause(u32, i32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkCopyToBufferRect@44",
            entry!(0, bink::copy_to_buffer_rect(u32, Ptr, i32, u32, u32, u32, u32, u32, u32, u32, u32) -> i32).1,
        ),
        // The Bink imports: end.
        // The Ogg Vorbis imports (`vorbisfile`, slot a11): begin.
        (
            "libvorbisfile.dll",
            "ov_open_callbacks",
            entry!(0, vorbisfile::ov_open_callbacks(u32, Ptr, u32, i32, u32, u32, u32, u32) -> i32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_info",
            entry!(0, vorbisfile::ov_info(Ptr, i32) -> u32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_seekable",
            entry!(0, vorbisfile::ov_seekable(Ptr) -> i32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_pcm_total",
            entry!(0, vorbisfile::ov_pcm_total(Ptr, i32) -> u64).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_read",
            entry!(0, vorbisfile::ov_read(Ptr, u32, i32, i32, i32, i32, Ptr) -> i32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_clear",
            entry!(0, vorbisfile::ov_clear(Ptr) -> i32).1,
        ),
        // The Ogg Vorbis imports: end.
        // The sound imports (`mmio`, `dsound`, slot a11): begin.
        (
            "WINMM.dll",
            "mmioOpenA",
            entry!(0, mmio::mmio_open_a(Ptr, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioClose",
            entry!(0, mmio::mmio_close(u32, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioRead",
            entry!(0, mmio::mmio_read(u32, Ptr, i32) -> i32).1,
        ),
        (
            "WINMM.dll",
            "mmioDescend",
            entry!(0, mmio::mmio_descend(u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioAscend",
            entry!(0, mmio::mmio_ascend(u32, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioGetInfo",
            entry!(0, mmio::mmio_get_info(u32, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioAdvance",
            entry!(0, mmio::mmio_advance(u32, Ptr, u32) -> u32).1,
        ),
        (
            "ole32.dll",
            "CoCreateInstance",
            entry!(0, dsound::co_create_instance(Ptr, u32, u32, Ptr, Ptr) -> u32).1,
        ),
        // The sound imports: end.
    ];
    // The shader compiler through the player's own d3dx9_38.dll (ADR-0008, Windows only).
    #[cfg(windows)]
    imports.extend([
        (
            "d3dx9_38.dll",
            "D3DXCompileShader",
            entry!(0, d3dxreal::compile_shader(Ptr, u32, Ptr, Ptr, Ptr, Ptr, u32, Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetShaderConstantTable",
            entry!(0, d3dxreal::get_shader_constant_table(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetVertexShaderProfile",
            entry!(0, d3dxreal::get_vertex_shader_profile(Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetPixelShaderProfile",
            entry!(0, d3dxreal::get_pixel_shader_profile(Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXCreateBuffer",
            entry!(0, d3dxreal::create_buffer(u32, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXAssembleShader",
            entry!(0, d3dxreal::assemble_shader(Ptr, u32, Ptr, Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
    ]);
    imports
}

// This crate's unit tests: `Engine::new()` registers the core's functions and this layer's.
#[cfg(test)]
engine_core::assemble!(units::funcs);

/// In this crate's tests, the core's `imports` with this layer installed first: a test may ask
/// for the table before any `Engine::new` has run on any thread. (Outside the tests the table
/// is only read for an engine that exists, `Engine::register_imports`.)
#[cfg(test)]
pub fn imports() -> Vec<(&'static str, &'static str, AbiFn)> {
    install_layer(&LAYER);
    engine_core::units::platform::imports()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `register_imports` takes the first entry for a (DLL, name), so the order of the core's
    /// imports and this layer's cannot matter while no name is in the table twice.
    #[test]
    fn every_import_is_answered_by_one_function() {
        let table = imports();
        let mut seen = std::collections::HashSet::new();
        for (dll, name, _) in &table {
            assert!(
                seen.insert((dll.to_ascii_lowercase(), *name)),
                "{dll} {name} is in the import table twice"
            );
        }
        // The 195 of the table before the layer moved out of `engine-core` (6 are Windows only),
        // and `GlobalMemoryStatus(Ex)`.
        assert_eq!(table.len(), if cfg!(windows) { 197 } else { 191 });
    }

    /// Not a check: the rows of `engine-core`'s `allocations_per_operation_of_the_kernel` whose
    /// imports are this layer's (`cargo test --release -p engine-platform --lib allocations_per
    /// -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn allocations_per_operation_of_the_message_imports() {
        use crate::alloc_count::{per_run, row};
        let mut e = Engine::new();
        let table = imports();
        let slot = |name: &str| {
            0x7f00_0000 + 4 * table.iter().position(|(_, n, _)| *n == name).unwrap() as u32
        };
        for (i, (_, _, f)) in table.iter().enumerate() {
            e.register(0x7f00_0000 + 4 * i as u32, *f);
        }
        let message = e.mem.alloc(0x1c);
        for (what, name, args) in [
            ("GetAsyncKeyState", "GetAsyncKeyState", vec![0x09]),
            (
                "PeekMessageA, none",
                "PeekMessageA",
                vec![message, 0, 0, 0, 1],
            ),
        ] {
            let f = slot(name);
            row(
                &format!("kernel {what}"),
                per_run(1000, || {
                    e.call(f, &args);
                }),
            );
        }
    }
}
