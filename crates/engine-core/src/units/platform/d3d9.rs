//! The platform's Direct3D 9 layer (ADR-0007): `Direct3DCreate9` (found by name in `D3D9.DLL`,
//! as `NiDX9Renderer`'s `00e69410` loads it), the `IDirect3D9` it makes and, as the runs reach
//! them, the device and its resources, with the documented Direct3D 9 semantics.
//!
//! Headless for now: resources will be real objects (sizes, formats, lock memory) but nothing is
//! drawn; the viewer still draws. As in the DirectInput and DirectSound layers (`super::dinput`,
//! `super::dsound`), each object lives in engine memory with a vtable pointer at `+0`, each
//! provided slot a platform function at a code address from [`CODE_BASE`]; a slot not provided is
//! left unregistered, so a call to it stops the run as an open function and is recorded with its
//! address (slot = (address - the interface's code base) / 4, named in [`D3D9_SLOTS`]).
//!
//! **The adapter.** The game asks about the player's own graphics card, so the layer answers for
//! the player's adapter 0 as Direct3D 9 described it on the player's machine: a table that
//! `scripts/d3d9-capture` writes there (the HAL and REF devices' identifier, display mode, modes,
//! caps, every `Check*` answer over a wide domain, and the screen metrics a program without a
//! DPI-awareness manifest sees, as FalloutNV.exe). The host loads it at start-up
//! ([`load_adapter_table`]) from a local file that is never committed; without it the first
//! Direct3D 9 query stops the run and says how to make it. The layer reports one adapter (the
//! table's). A query the capture did not ask (a word outside the captured domain) is not
//! provided and stops the run, never a made-up answer.

use super::d3d9device::create_device;
use crate::{Engine, Ptr};
use std::collections::HashMap;

/// Where the platform's Direct3D 9 functions sit (code addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e02_0000;
/// `IDirect3D9`'s slots.
pub const D3D9_CODE: u32 = CODE_BASE;
/// The procedures `GetProcAddress` gives out for `D3D9.DLL` ([`PROCS`]).
pub const PROC_CODE: u32 = CODE_BASE + 0x0f00;
/// The module handle `LoadLibraryA("D3D9.DLL")` answers (a value no image uses).
pub const D3D9_MODULE: u32 = 0x7e11_0000;
/// The `HMONITOR` `GetAdapterMonitor` answers for the adapter (an opaque value).
pub const ADAPTER_MONITOR: u32 = 0x7e12_0001;

/// `D3D9.DLL`'s procedures found by name, in the order of their code addresses from
/// [`PROC_CODE`]. `Direct3DCreate9Ex` exists in the DLL (Windows Vista and later), but is not
/// provided: its address stops the run when called.
pub const PROCS: [&[u8]; 2] = [b"Direct3DCreate9", b"Direct3DCreate9Ex"];

/// `IDirect3D9`'s slots (d3d9.h order).
pub const D3D9_SLOTS: [&str; 17] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "RegisterSoftwareDevice",
    "GetAdapterCount",
    "GetAdapterIdentifier",
    "GetAdapterModeCount",
    "EnumAdapterModes",
    "GetAdapterDisplayMode",
    "CheckDeviceType",
    "CheckDeviceFormat",
    "CheckDeviceMultiSampleType",
    "CheckDepthStencilMatch",
    "CheckDeviceFormatConversion",
    "GetDeviceCaps",
    "GetAdapterMonitor",
    "CreateDevice",
];

/// Result codes (d3d9.h, winerror.h).
pub const D3D_OK: u32 = 0;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const D3DERR_INVALIDCALL: u32 = 0x8876_086c;
pub const D3DERR_NOTAVAILABLE: u32 = 0x8876_086a;
/// `D3D_SDK_VERSION` of the DirectX 9.0c SDK the game was built with (32), and the same with
/// the debug flag.
pub const D3D_SDK_VERSION: u32 = 32;
const D3D_SDK_VERSION_DEBUG: u32 = 0x8000_0000 | 32;
/// The sizes the queries write: `D3DADAPTER_IDENTIFIER9`, `D3DDISPLAYMODE`, `D3DCAPS9`.
const IDENTIFIER_SIZE: usize = 1100;
const DISPLAY_MODE_SIZE: usize = 16;
const CAPS_SIZE: usize = 304;

/// How to make the adapter table, for the stop when it is missing.
pub const CAPTURE_HINT: &str = "run `cargo run --release --manifest-path \
     scripts/d3d9-capture/Cargo.toml > <file>` on this machine and point NV_D3D9_ADAPTER at the \
     file (default: %LOCALAPPDATA%/nv-rs/d3d9_adapter.tsv)";

/// One folded `Check*` line: the default result, the exceptions by the last word, the domain
/// of the last word.
#[derive(Debug, Clone)]
struct CheckRow {
    default: String,
    exceptions: HashMap<u32, String>,
    domain: Vec<u32>,
}

/// The captured adapter (a `scripts/d3d9-capture` table parsed).
#[derive(Debug, Default)]
pub struct Adapter {
    metrics: HashMap<u32, u32>,
    identifiers: HashMap<u32, (u32, Vec<u8>)>,
    display_mode: (u32, Vec<u8>),
    mode_counts: HashMap<u32, u32>,
    modes: HashMap<(u32, u32), (u32, Vec<u8>)>,
    caps: HashMap<u32, (u32, Vec<u8>)>,
    checks: HashMap<(String, Vec<u32>), CheckRow>,
    /// The style and extended style a window made with a style reads back, by (style, child).
    window_styles: HashMap<(u32, u32), (u32, u32)>,
    /// `AdjustWindowRectEx`'s border by (style, menu, extended style): left, top, right, bottom.
    borders: HashMap<(u32, u32, u32), [i32; 4]>,
    /// The Windows directory and the folders by CSIDL (`SHGetFolderPathA`: result, path).
    windows_directory: Vec<u8>,
    folders: HashMap<u32, (u32, Vec<u8>)>,
    /// `GetTimeZoneInformation`'s answer when captured and the `TIME_ZONE_INFORMATION` bytes.
    time_zone: Option<(u32, Vec<u8>)>,
    /// `GetTimeZoneInformationForYear(year, NULL, ..)`: (first year, last year, the 172
    /// `TIME_ZONE_INFORMATION` bytes) for runs of years with the same answer.
    zone_years: Vec<(u32, u32, Vec<u8>)>,
    /// `EnumDisplayDevicesA(NULL, index, .., 0)`'s answer and `DISPLAY_DEVICEA` bytes by index.
    display_devices: HashMap<u32, (u32, Vec<u8>)>,
    /// The `NvApi*` lines (`scripts/nvapi-capture`).
    nvapi: super::nvapi::Table,
    /// The processor: the `ID` flag test's answer and `CPUID` leaves by number (`EAX EBX ECX
    /// EDX`), for the CRT's `__get_sse2_info`.
    cpu_id_flag_test: Option<u32>,
    cpuid: HashMap<u32, [u32; 4]>,
    /// The code pages (`d3d9-capture codepages`): `GetACP`, `GetOEMCP`, and by code page
    /// `IsValidCodePage`'s answer and `GetCPInfo`'s answer with the 20 `CPINFO` bytes.
    ansi_code_page: Option<u32>,
    oem_code_page: Option<u32>,
    valid_code_pages: HashMap<u32, u32>,
    code_page_info: HashMap<u32, (u32, Vec<u8>)>,
    /// `IDirect3DDevice9::CreateQuery(type, NULL)`: whether the device takes each query type
    /// (`d3d9-capture device queries`).
    queries: HashMap<u32, u32>,
}

fn words(text: &str) -> Vec<u32> {
    text.split([' ', ','])
        .filter(|w| !w.is_empty())
        .map(|w| u32::from_str_radix(w, 16).expect("a hex word in d3d9_adapter.tsv"))
        .collect()
}

fn bytes(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex bytes"))
        .collect()
}

impl Adapter {
    /// Parses the capture table.
    pub fn parse(table: &str) -> Adapter {
        let mut a = Adapter::default();
        let mut domains: HashMap<String, Vec<u32>> = HashMap::new();
        for line in table.lines().filter(|l| !l.is_empty()) {
            let f: Vec<&str> = line.split('\t').collect();
            let hex = |i: usize| u32::from_str_radix(f[i], 16).expect("a hex result");
            if a.nvapi.take_line(&f) {
                continue;
            }
            match f[0] {
                "GetSystemMetrics" => {
                    a.metrics.insert(words(f[1])[0], hex(2));
                }
                "GetAdapterCount" | "GetAdapterMonitor" | "CreateDevice" => {}
                "CreateQuery" => {
                    a.queries.insert(hex(1), hex(2));
                }
                "GetAdapterIdentifier" => {
                    a.identifiers.insert(words(f[1])[1], (hex(2), bytes(f[3])));
                }
                "GetAdapterDisplayMode" => a.display_mode = (hex(2), bytes(f[3])),
                "GetAdapterModeCount" => {
                    a.mode_counts.insert(words(f[1])[1], hex(2));
                }
                "EnumAdapterModes" => {
                    let w = words(f[1]);
                    a.modes.insert((w[1], w[2]), (hex(2), bytes(f[3])));
                }
                "GetDeviceCaps" => {
                    a.caps.insert(words(f[1])[1], (hex(2), bytes(f[3])));
                }
                "GetWindowsDirectoryA" => a.windows_directory = bytes(f[3]),
                "CpuIdFlagTest" => a.cpu_id_flag_test = Some(hex(2)),
                "Cpuid" => {
                    let r = words(f[2]);
                    a.cpuid.insert(words(f[1])[0], [r[0], r[1], r[2], r[3]]);
                }
                "GetTimeZoneInformation" => a.time_zone = Some((hex(2), bytes(f[3]))),
                "GetTimeZoneInformationForYear" => {
                    let w = words(f[1]);
                    a.zone_years.push((w[0], w[1], bytes(f[3])));
                }
                "GetACP" => a.ansi_code_page = Some(hex(2)),
                "GetOEMCP" => a.oem_code_page = Some(hex(2)),
                "IsValidCodePage" => {
                    a.valid_code_pages.insert(words(f[1])[0], hex(2));
                }
                "GetCPInfo" => {
                    a.code_page_info
                        .insert(words(f[1])[0], (hex(2), bytes(f[3])));
                }
                "EnumDisplayDevicesA" => {
                    a.display_devices
                        .insert(words(f[1])[0], (hex(2), bytes(f[3])));
                }
                "SHGetFolderPathA" => {
                    a.folders.insert(words(f[1])[0], (hex(2), bytes(f[3])));
                }
                "CreateWindowStyle" => {
                    let w = words(f[1]);
                    a.window_styles.insert((w[0], w[1]), (hex(2), hex(3)));
                }
                "AdjustWindowRectEx" => {
                    let w = words(f[1]);
                    let r = words(f[3]);
                    a.borders.insert(
                        (w[0], w[1], w[2]),
                        [r[0] as i32, r[1] as i32, r[2] as i32, r[3] as i32],
                    );
                }
                "Domain" => {
                    domains.insert(f[1].to_string(), words(f[2]));
                }
                kind => {
                    let exceptions = f[3]
                        .split(',')
                        .filter(|e| !e.is_empty())
                        .map(|e| {
                            let (w, r) = e.split_once('=').expect("word=result");
                            (
                                u32::from_str_radix(w, 16).expect("a hex word"),
                                r.to_string(),
                            )
                        })
                        .collect();
                    a.checks.insert(
                        (kind.to_string(), words(f[1])),
                        CheckRow {
                            default: f[2].to_string(),
                            exceptions,
                            domain: domains[f[4]].clone(),
                        },
                    );
                }
            }
        }
        a
    }

    /// The captured answer to `kind(words..)` (the adapter first), or `None` when the capture
    /// did not ask it.
    fn check(&self, kind: &str, words: &[u32]) -> Option<&str> {
        let (last, lead) = words.split_last()?;
        let row = self.checks.get(&(kind.to_string(), lead.to_vec()))?;
        if !row.domain.contains(last) {
            return None;
        }
        Some(row.exceptions.get(last).unwrap_or(&row.default))
    }
}

/// An `IDirect3D9` of this layer.
#[derive(Debug, Clone)]
struct Direct3D {
    references: u32,
}

/// The Direct3D 9 objects (`Kernel::graphics`).
#[derive(Debug, Default)]
pub struct Graphics {
    adapter: Option<Adapter>,
    d3d9_vtable: u32,
    objects: HashMap<u32, Direct3D>,
    procs_registered: bool,
    /// The devices and their resources (`super::d3d9device`).
    pub(crate) device_objects: super::d3d9device::Objects,
    /// The NvAPI answers and the functions handed out (`super::nvapi`).
    pub(crate) nvapi: super::nvapi::State,
    /// The machine-local D3DX texture cache folder (`super::d3dx`).
    pub(crate) d3dx_texture_cache: Option<std::path::PathBuf>,
    /// The real `d3d9.dll` objects behind the modelled ones (`NV_D3D9=real`, ADR-0008).
    #[cfg(windows)]
    pub(crate) real: super::d3d9real::Real,
    /// The shader compiler's buffers and constant tables (`super::d3dxreal`).
    #[cfg(windows)]
    pub(crate) d3dx: super::d3dxreal::Objects,
}

/// Gives the layer the player's adapter table (`scripts/d3d9-capture`'s output).
pub fn load_adapter_table(e: &mut Engine, table: &str) {
    let adapter = Adapter::parse(table);
    e.kernel.graphics.nvapi.table = adapter.nvapi.clone();
    e.kernel.graphics.adapter = Some(adapter);
}

/// The loaded adapter; without one the run stops and says how to capture it.
fn adapter(e: &mut Engine) -> &Adapter {
    match e.kernel.graphics.adapter.as_ref() {
        Some(a) => a,
        None => panic!("the Direct3D 9 adapter table is not loaded: {CAPTURE_HINT}"),
    }
}

fn not_captured(kind: &str, words: &[u32]) -> ! {
    let words: Vec<String> = words.iter().map(|w| format!("{w:x}")).collect();
    panic!(
        "IDirect3D9::{kind}({}) was not captured in the adapter table: not provided",
        words.join(" ")
    )
}

/// The style and extended style a window made with `style` reads back (`child`: made as a
/// child window), if the capture has them.
/// What `CreateQuery(type, NULL)` answers on the player's device, when the table has it.
pub fn query_support(e: &mut Engine, kind: u32) -> Option<u32> {
    adapter(e).queries.get(&kind).copied()
}

pub fn window_style(e: &mut Engine, style: u32, child: u32) -> Option<(u32, u32)> {
    adapter(e).window_styles.get(&(style, child)).copied()
}

/// The player's Windows directory, from the table (where profile files named without a path
/// are).
pub fn windows_directory(e: &mut Engine) -> Vec<u8> {
    let dir = adapter(e).windows_directory.clone();
    assert!(
        !dir.is_empty(),
        "the adapter table has no Windows directory: {CAPTURE_HINT}"
    );
    dir
}

// Platform: replaces USER32.DLL EnumDisplayDevicesA (import)
/// `EnumDisplayDevicesA(device, index, info, flags)` for the adapters (`device` NULL, flags 0):
/// the player's devices as the machine table recorded them (`scripts/d3d9-capture`): a listed
/// index answers TRUE and its 0x1A8-byte `DISPLAY_DEVICEA`; the first index past them answers
/// FALSE and writes nothing (measured: the capture's buffer after the failing call was as it
/// went in). A device name, flags, another `cb` or an index the capture didn't reach are not
/// provided.
pub fn enum_display_devices_a(
    e: &mut Engine,
    device: Ptr,
    index: u32,
    info: Ptr,
    flags: u32,
) -> u32 {
    assert!(
        device.addr() == 0 && flags == 0 && e.mem.u32(info.addr()) == 0x1a8,
        "EnumDisplayDevicesA(device {:#x}, {index}, cb {:#x}, flags {flags:#x}) is not provided",
        device.addr(),
        e.mem.u32(info.addr())
    );
    let Some((answer, bytes)) = adapter(e).display_devices.get(&index).cloned() else {
        panic!(
            "EnumDisplayDevicesA({index}) was not captured in the adapter table: {CAPTURE_HINT}"
        );
    };
    if answer != 0 {
        e.mem.write(info.addr(), &bytes);
    }
    answer
}

// Platform: answers the processor at 00ed193c (the ID flag test in __get_sse2_info 00ed1927)
/// The `ID` flag test `__get_sse2_info` runs (`00ed193c`: EFLAGS read back after flipping bit
/// 21, minus the entry flags): the player's processor's answer from the machine table
/// (`scripts/d3d9-capture` writes it); without it the run stops.
pub fn cpu_id_flag_test(e: &mut Engine) -> u32 {
    match adapter(e).cpu_id_flag_test {
        Some(change) => change,
        None => panic!("the machine table has no processor lines (CpuIdFlagTest): run `d3d9-capture cpu` and add them: {CAPTURE_HINT}"),
    }
}

// Platform: answers the processor at 00ed1951 and 00ed1964 (CPUID in __get_sse2_info 00ed1927)
/// `CPUID` with `EAX = leaf` (`00ed1951` leaf 0, `00ed1964` leaf 1; cdecl: the leaf, the four
/// words `EAX`, `EBX`, `ECX`, `EDX` are written to): the player's processor's answer from the
/// machine table; a leaf it lacks stops the run.
/// The hint for code-page lines missing from the machine table.
const CODE_PAGE_HINT: &str = "run `d3d9-capture codepages` and add the lines";

// Platform: replaces KERNEL32.DLL GetACP (import)
/// `GetACP()`: the player's ANSI code page as the machine table recorded it.
pub fn get_acp(e: &mut Engine) -> u32 {
    adapter(e).ansi_code_page.unwrap_or_else(|| {
        panic!("GetACP is not in the machine table: {CODE_PAGE_HINT}: {CAPTURE_HINT}")
    })
}

// Platform: replaces KERNEL32.DLL GetOEMCP (import)
/// `GetOEMCP()`: the player's OEM code page as the machine table recorded it.
pub fn get_oemcp(e: &mut Engine) -> u32 {
    adapter(e).oem_code_page.unwrap_or_else(|| {
        panic!("GetOEMCP is not in the machine table: {CODE_PAGE_HINT}: {CAPTURE_HINT}")
    })
}

// Platform: replaces KERNEL32.DLL IsValidCodePage (import)
/// `IsValidCodePage(code page)`: the answer the machine table recorded for that page; a page
/// the capture did not ask about is not provided.
pub fn is_valid_code_page(e: &mut Engine, page: u32) -> u32 {
    match adapter(e).valid_code_pages.get(&page) {
        Some(&answer) => answer,
        None => panic!(
            "IsValidCodePage({page}) is not in the machine table: {CODE_PAGE_HINT}: {CAPTURE_HINT}"
        ),
    }
}

// Platform: replaces KERNEL32.DLL GetCPInfo (import)
/// `GetCPInfo(code page, info)`: the answer and the 20 `CPINFO` bytes the machine table recorded
/// for that page (written only when it answered true); a page the capture did not ask about is
/// not provided.
pub fn get_cp_info(e: &mut Engine, page: u32, info: Ptr) -> u32 {
    let Some((answer, bytes)) = adapter(e).code_page_info.get(&page).cloned() else {
        panic!("GetCPInfo({page}) is not in the machine table: {CODE_PAGE_HINT}: {CAPTURE_HINT}");
    };
    if answer != 0 {
        e.mem.write(info.addr(), &bytes);
    }
    answer
}

pub fn cpuid(e: &mut Engine, leaf: u32, out: Ptr) {
    let Some(r) = adapter(e).cpuid.get(&leaf).copied() else {
        panic!("CPUID leaf {leaf:#x} is not in the machine table: run `d3d9-capture cpu` and add the lines: {CAPTURE_HINT}");
    };
    for (i, w) in r.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *w);
    }
}

/// The player's time zone from the table (`GetTimeZoneInformation`'s 172 bytes); the run stops
/// when the table has none.
pub fn time_zone(e: &mut Engine) -> Vec<u8> {
    match adapter(e).time_zone.as_ref() {
        Some((_, bytes)) if bytes.len() == 172 => bytes.clone(),
        _ => panic!("the adapter table has no time zone (GetTimeZoneInformation): {CAPTURE_HINT}"),
    }
}

/// The player's time zone rules for `year` (`GetTimeZoneInformationForYear(year, NULL, ..)`,
/// the 172 `TIME_ZONE_INFORMATION` bytes: the rules Windows keeps for each year, which can
/// differ from the current ones `GetTimeZoneInformation` answers).
pub fn time_zone_for_year(e: &mut Engine, year: u32) -> Vec<u8> {
    let found = adapter(e)
        .zone_years
        .iter()
        .find(|(first, last, bytes)| (*first..=*last).contains(&year) && bytes.len() == 172)
        .map(|(_, _, bytes)| bytes.clone());
    found.unwrap_or_else(|| {
        panic!("the adapter table has no time zone rules for {year} (GetTimeZoneInformationForYear): {CAPTURE_HINT}")
    })
}

/// The player's folder for a CSIDL (`SHGetFolderPathA`'s result and path), if the table has it.
pub fn folder_path(e: &mut Engine, csidl: u32) -> Option<(u32, Vec<u8>)> {
    adapter(e).folders.get(&csidl).cloned()
}

/// The border `AdjustWindowRectEx` adds for `style`, `menu` (0 or 1) and `ex_style`, if the
/// capture has it.
pub fn window_border(e: &mut Engine, style: u32, menu: u32, ex_style: u32) -> Option<[i32; 4]> {
    adapter(e).borders.get(&(style, menu, ex_style)).copied()
}

/// A captured screen metric (`GetSystemMetrics`), if the capture has it.
pub fn system_metric(e: &mut Engine, index: u32) -> Option<u32> {
    adapter(e).metrics.get(&index).copied()
}

/// Whether `object` is an `IDirect3D9` of this layer.
pub fn is_direct3d(e: &Engine, object: u32) -> bool {
    e.kernel.graphics.objects.contains_key(&object)
}

fn register_code(e: &mut Engine) {
    let d3d9: [(u32, crate::AbiFn); 17] = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (4, crate::entry!(0, get_adapter_count(Ptr) -> u32).1),
        (
            5,
            crate::entry!(0, get_adapter_identifier(Ptr, u32, u32, Ptr) -> u32).1,
        ),
        (6, crate::entry!(0, get_adapter_mode_count(Ptr, u32, u32) -> u32).1),
        (
            7,
            crate::entry!(0, enum_adapter_modes(Ptr, u32, u32, u32, Ptr) -> u32).1,
        ),
        (
            8,
            crate::entry!(0, get_adapter_display_mode(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            9,
            crate::entry!(0, check_device_type(Ptr, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            10,
            crate::entry!(0, check_device_format(Ptr, u32, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            11,
            crate::entry!(0, check_device_multi_sample_type(Ptr, u32, u32, u32, u32, u32, Ptr) -> u32).1,
        ),
        (
            12,
            crate::entry!(0, check_depth_stencil_match(Ptr, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            13,
            crate::entry!(0, check_device_format_conversion(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (14, crate::entry!(0, get_device_caps(Ptr, u32, u32, Ptr) -> u32).1),
        (15, crate::entry!(0, get_adapter_monitor(Ptr, u32) -> u32).1),
        (3, crate::entry!(0, register_software_device(Ptr, u32) -> u32).1),
        (
            16,
            crate::entry!(0, create_device(Ptr, u32, u32, u32, u32, Ptr, Ptr) -> u32).1,
        ),
    ];
    for (slot, f) in d3d9 {
        e.register(D3D9_CODE + 4 * slot, f);
    }
}

/// The procedure `name` of `D3D9.DLL` ([`PROCS`]): its code address (the provided ones
/// registered on first use), or `None` for a name the DLL doesn't export.
pub fn proc_address(e: &mut Engine, name: &[u8]) -> Option<u32> {
    let index = PROCS.iter().position(|p| p.eq_ignore_ascii_case(name))?;
    if !e.kernel.graphics.procs_registered {
        e.register(PROC_CODE, crate::entry!(0, direct3d_create9(u32) -> u32).1);
        e.kernel.graphics.procs_registered = true;
    }
    Some(PROC_CODE + 4 * index as u32)
}

// Platform: replaces D3D9.DLL Direct3DCreate9 (found by name)
/// `Direct3DCreate9(sdk version)`: a new `IDirect3D9` (reference count 1) for
/// `D3D_SDK_VERSION` (32, with or without the debug bit); null for any other version, as the
/// runtime refuses a mismatched SDK.
pub fn direct3d_create9(e: &mut Engine, sdk: u32) -> u32 {
    if sdk != D3D_SDK_VERSION && sdk != D3D_SDK_VERSION_DEBUG {
        return 0;
    }
    if e.kernel.graphics.d3d9_vtable == 0 {
        register_code(e);
        let table = e.mem.alloc(4 * D3D9_SLOTS.len() as u32);
        for i in 0..D3D9_SLOTS.len() as u32 {
            e.mem.set_u32(table + 4 * i, D3D9_CODE + 4 * i);
        }
        e.kernel.graphics.d3d9_vtable = table;
    }
    let object = e.mem.alloc(8);
    e.mem.set_u32(object, e.kernel.graphics.d3d9_vtable);
    e.kernel
        .graphics
        .objects
        .insert(object, Direct3D { references: 1 });
    #[cfg(windows)]
    super::d3d9real::direct3d_created(e);
    object
}

fn this_object(e: &mut Engine, this: Ptr) -> &mut Direct3D {
    e.kernel
        .graphics
        .objects
        .get_mut(&this.addr())
        .expect("an IDirect3D9 of this layer")
}

fn no_interface(e: &mut Engine, _this: Ptr, _interface: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

fn add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = this_object(e, this);
    d.references += 1;
    d.references
}

fn release(e: &mut Engine, this: Ptr) -> u32 {
    let d = this_object(e, this);
    d.references -= 1;
    let left = d.references;
    if left == 0 {
        e.kernel.graphics.objects.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `RegisterSoftwareDevice(init function)`: pluggable software devices are not provided.
fn register_software_device(_e: &mut Engine, _this: Ptr, _init: u32) -> u32 {
    panic!("IDirect3D9::RegisterSoftwareDevice is not provided")
}

/// `GetAdapterCount()`: the one adapter of the table.
fn get_adapter_count(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

/// `GetAdapterIdentifier(adapter, flags, out)`: the captured `D3DADAPTER_IDENTIFIER9` for
/// these flags (0, or `D3DENUM_WHQL_LEVEL` 2); `D3DERR_INVALIDCALL` for another adapter or a
/// null `out`.
fn get_adapter_identifier(e: &mut Engine, _this: Ptr, index: u32, flags: u32, out: Ptr) -> u32 {
    if index != 0 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let Some((hr, data)) = adapter(e).identifiers.get(&flags).cloned() else {
        not_captured("GetAdapterIdentifier", &[index, flags]);
    };
    debug_assert_eq!(data.len(), IDENTIFIER_SIZE);
    e.mem.write(out.addr(), &data);
    hr
}

/// `GetAdapterModeCount(adapter, format)`: the captured count (0 for another adapter).
fn get_adapter_mode_count(e: &mut Engine, _this: Ptr, index: u32, format: u32) -> u32 {
    if index != 0 {
        return 0;
    }
    match adapter(e).mode_counts.get(&format) {
        Some(&n) => n,
        None => not_captured("GetAdapterModeCount", &[index, format]),
    }
}

/// `EnumAdapterModes(adapter, format, mode, out)`: the captured `D3DDISPLAYMODE`;
/// `D3DERR_INVALIDCALL` for another adapter, a mode past the count or a null `out`.
fn enum_adapter_modes(
    e: &mut Engine,
    _this: Ptr,
    index: u32,
    format: u32,
    mode: u32,
    out: Ptr,
) -> u32 {
    if index != 0 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let count = get_adapter_mode_count(e, Ptr::new(0), index, format);
    if mode >= count {
        return D3DERR_INVALIDCALL;
    }
    let (hr, data) = adapter(e).modes[&(format, mode)].clone();
    debug_assert_eq!(data.len(), DISPLAY_MODE_SIZE);
    e.mem.write(out.addr(), &data);
    hr
}

/// `GetAdapterDisplayMode(adapter, out)`: the captured desktop mode.
fn get_adapter_display_mode(e: &mut Engine, _this: Ptr, index: u32, out: Ptr) -> u32 {
    if index != 0 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let (hr, data) = adapter(e).display_mode.clone();
    e.mem.write(out.addr(), &data);
    hr
}

fn checked(e: &mut Engine, kind: &str, words: &[u32]) -> u32 {
    if words[0] != 0 {
        return D3DERR_INVALIDCALL;
    }
    match adapter(e).check(kind, words) {
        Some(r) => u32::from_str_radix(r, 16).expect("a hex result"),
        None => not_captured(kind, words),
    }
}

/// `CheckDeviceType(adapter, device type, display format, back buffer format, windowed)`.
fn check_device_type(
    e: &mut Engine,
    _this: Ptr,
    index: u32,
    device: u32,
    display: u32,
    back: u32,
    windowed: u32,
) -> u32 {
    checked(
        e,
        "CheckDeviceType",
        &[index, device, display, back, u32::from(windowed != 0)],
    )
}

/// `CheckDeviceFormat(adapter, device type, adapter format, usage, resource type, format)`.
#[allow(clippy::too_many_arguments)]
fn check_device_format(
    e: &mut Engine,
    _this: Ptr,
    index: u32,
    device: u32,
    display: u32,
    usage: u32,
    resource: u32,
    format: u32,
) -> u32 {
    checked(
        e,
        "CheckDeviceFormat",
        &[index, device, display, usage, resource, format],
    )
}

/// `CheckDeviceMultiSampleType(adapter, device type, format, windowed, type, quality out)`:
/// the captured result, the quality level count stored through `quality` when not null.
#[allow(clippy::too_many_arguments)]
fn check_device_multi_sample_type(
    e: &mut Engine,
    _this: Ptr,
    index: u32,
    device: u32,
    format: u32,
    windowed: u32,
    samples: u32,
    quality: Ptr,
) -> u32 {
    let words = [index, device, format, u32::from(windowed != 0), samples];
    if index != 0 {
        return D3DERR_INVALIDCALL;
    }
    let Some(answer) = adapter(e).check("CheckDeviceMultiSampleType", &words) else {
        not_captured("CheckDeviceMultiSampleType", &words);
    };
    let (hr, levels) = answer.split_once('/').expect("hr/quality");
    let hr = u32::from_str_radix(hr, 16).expect("a hex result");
    let levels = u32::from_str_radix(levels, 16).expect("a hex quality count");
    if !quality.is_null() {
        e.mem.set_u32(quality.addr(), levels);
    }
    hr
}

/// `CheckDepthStencilMatch(adapter, device type, adapter format, target format, depth format)`.
fn check_depth_stencil_match(
    e: &mut Engine,
    _this: Ptr,
    index: u32,
    device: u32,
    display: u32,
    target: u32,
    depth: u32,
) -> u32 {
    checked(
        e,
        "CheckDepthStencilMatch",
        &[index, device, display, target, depth],
    )
}

/// `CheckDeviceFormatConversion(adapter, device type, source format, target format)`.
fn check_device_format_conversion(
    e: &mut Engine,
    _this: Ptr,
    index: u32,
    device: u32,
    source: u32,
    target: u32,
) -> u32 {
    checked(
        e,
        "CheckDeviceFormatConversion",
        &[index, device, source, target],
    )
}

/// `GetDeviceCaps(adapter, device type, out)`: the captured `D3DCAPS9` (HAL, REF; the software
/// device type answers what the capture got, `D3DERR_NOTAVAILABLE`).
fn get_device_caps(e: &mut Engine, _this: Ptr, index: u32, device: u32, out: Ptr) -> u32 {
    if index != 0 || out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let Some((hr, data)) = adapter(e).caps.get(&device).cloned() else {
        not_captured("GetDeviceCaps", &[index, device]);
    };
    debug_assert_eq!(data.len(), CAPS_SIZE);
    if hr == D3D_OK {
        e.mem.write(out.addr(), &data);
    }
    hr
}

/// `GetAdapterMonitor(adapter)`: the adapter's monitor handle; null for another adapter.
fn get_adapter_monitor(_e: &mut Engine, _this: Ptr, index: u32) -> u32 {
    if index == 0 {
        ADAPTER_MONITOR
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST DATA, not a capture: a small adapter table in `scripts/d3d9-capture`'s format,
    /// made up for these tests (an identifier whose driver name is "test.dll", one 640x480
    /// X8R8G8B8 mode, caps whose first word is the HAL device type, a few folded answers).
    fn test_table() -> String {
        let hex = |b: &[u8]| -> String { b.iter().map(|x| format!("{x:02x}")).collect() };
        let mut identifier = vec![0u8; IDENTIFIER_SIZE];
        identifier[..8].copy_from_slice(b"test.dll");
        let mut mode = vec![0u8; DISPLAY_MODE_SIZE];
        mode[..4].copy_from_slice(&640u32.to_le_bytes());
        mode[4..8].copy_from_slice(&480u32.to_le_bytes());
        mode[8..12].copy_from_slice(&60u32.to_le_bytes());
        mode[12..].copy_from_slice(&0x16u32.to_le_bytes());
        let mut caps = vec![0u8; CAPS_SIZE];
        caps[..4].copy_from_slice(&1u32.to_le_bytes());
        [
            "GetSystemMetrics\t0\t280".to_string(),
            "GetSystemMetrics\t1\t1e0".to_string(),
            "GetAdapterCount\t\t1".to_string(),
            format!("GetAdapterIdentifier\t0 0\t0\t{}", hex(&identifier)),
            format!("GetAdapterDisplayMode\t0\t0\t{}", hex(&mode)),
            "GetAdapterModeCount\t0 16\t1".to_string(),
            "GetAdapterModeCount\t0 17\t0".to_string(),
            format!("EnumAdapterModes\t0 16 0\t0\t{}", hex(&mode)),
            format!("GetDeviceCaps\t0 1\t0\t{}", hex(&caps)),
            "GetDeviceCaps\t0 3\t8876086a\t".to_string() + &hex(&[0u8; CAPS_SIZE]),
            "Domain\td0\t15,16,17".to_string(),
            "CheckDeviceFormat\t0 1 16 0 3\t8876086a\t16=0\td0".to_string(),
            "Domain\td1\t0,2,4".to_string(),
            "CheckDeviceMultiSampleType\t0 1 16 0\t0/1\t4=8876086a/0\td1".to_string(),
        ]
        .join("\n")
    }

    #[test]
    fn code_pages_answer_from_the_table() {
        let mut e = Engine::new();
        load_adapter_table(
            &mut e,
            "GetACP		4e4
GetOEMCP		1b5
IsValidCodePage	4e4	1
GetCPInfo	4e4	1	010000003f000000000000000000000000000000
",
        );
        assert_eq!(get_acp(&mut e), 1252);
        assert_eq!(get_oemcp(&mut e), 437);
        assert_eq!(is_valid_code_page(&mut e, 1252), 1);
        let info = e.mem.alloc(20);
        assert_eq!(get_cp_info(&mut e, 1252, Ptr::new(info)), 1);
        assert_eq!(e.mem.u32(info), 1, "MaxCharSize");
        assert_eq!(e.mem.u8(info + 4), b'?', "DefaultChar");
        let missing = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            get_cp_info(&mut e, 932, Ptr::new(info));
        }));
        assert!(
            missing.is_err(),
            "a page the capture did not ask about stops"
        );
    }

    fn d3d() -> (Engine, u32) {
        let mut e = Engine::new();
        load_adapter_table(&mut e, &test_table());
        let create = proc_address(&mut e, b"Direct3DCreate9").unwrap();
        let d3d = e.call(create, &[D3D_SDK_VERSION]).u32();
        assert_ne!(d3d, 0);
        (e, d3d)
    }

    #[test]
    fn the_table_parses_and_answers_from_it() {
        let a = Adapter::parse(&test_table());
        assert_eq!(a.identifiers[&0].1.len(), IDENTIFIER_SIZE);
        assert_eq!(a.caps[&1].1.len(), CAPS_SIZE);
        assert_eq!(
            a.check("CheckDeviceFormat", &[0, 1, 0x16, 0, 3, 0x16]),
            Some("0")
        );
        assert_eq!(
            a.check("CheckDeviceFormat", &[0, 1, 0x16, 0, 3, 0x15]),
            Some("8876086a")
        );
        assert_eq!(
            a.check("CheckDeviceFormat", &[0, 1, 0x16, 0, 3, 0x1234]),
            None
        );
        assert_eq!(
            a.check("CheckDeviceFormat", &[0, 1, 0x16, 1, 3, 0x16]),
            None
        );
    }

    #[test]
    fn the_object_answers_through_its_vtable() {
        let (mut e, d3d) = d3d();
        assert_eq!(e.vcall(d3d, 4 * 4, &[]).u32(), 1);
        let out = e.mem.alloc(IDENTIFIER_SIZE as u32);
        assert_eq!(e.vcall(d3d, 4 * 5, &[0, 0, out]).u32(), D3D_OK);
        assert_eq!(e.mem.bytes(out, 8), b"test.dll".to_vec());
        assert_eq!(e.vcall(d3d, 4 * 5, &[1, 0, out]).u32(), D3DERR_INVALIDCALL);
        let mode = e.mem.alloc(16);
        assert_eq!(e.vcall(d3d, 4 * 8, &[0, mode]).u32(), D3D_OK);
        assert_eq!((e.mem.u32(mode), e.mem.u32(mode + 4)), (640, 480));
        assert_eq!(e.mem.u32(mode + 12), 0x16);
        let caps = e.mem.alloc(CAPS_SIZE as u32);
        assert_eq!(e.vcall(d3d, 4 * 14, &[0, 1, caps]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(caps), 1, "DeviceType: HAL");
        assert_eq!(
            e.vcall(d3d, 4 * 14, &[0, 3, caps]).u32(),
            D3DERR_NOTAVAILABLE
        );
        assert_eq!(
            e.vcall(d3d, 4 * 10, &[0, 1, 0x16, 0, 3, 0x16]).u32(),
            D3D_OK
        );
        assert_eq!(e.vcall(d3d, 4 * 15, &[0]).u32(), ADAPTER_MONITOR);
        assert_eq!(system_metric(&mut e, 0), Some(640));
        assert_eq!(system_metric(&mut e, 0x50), None);
        assert_eq!(e.vcall(d3d, 4, &[]).u32(), 2);
        assert_eq!(e.vcall(d3d, 4 * 2, &[]).u32(), 1);
        assert_eq!(e.vcall(d3d, 4 * 2, &[]).u32(), 0);
    }

    #[test]
    fn modes_follow_the_counts() {
        let (mut e, d3d) = d3d();
        assert_eq!(e.vcall(d3d, 4 * 6, &[0, 0x16]).u32(), 1);
        assert_eq!(e.vcall(d3d, 4 * 6, &[0, 0x17]).u32(), 0);
        let mode = e.mem.alloc(16);
        assert_eq!(e.vcall(d3d, 4 * 7, &[0, 0x16, 0, mode]).u32(), D3D_OK);
        assert_eq!(e.mem.u32(mode), 640);
        assert_eq!(
            e.vcall(d3d, 4 * 7, &[0, 0x16, 1, mode]).u32(),
            D3DERR_INVALIDCALL
        );
    }

    #[test]
    fn multisample_answers_store_the_quality_levels() {
        let (mut e, d3d) = d3d();
        let quality = e.mem.alloc(4);
        assert_eq!(
            e.vcall(d3d, 4 * 11, &[0, 1, 0x16, 0, 2, quality]).u32(),
            D3D_OK
        );
        assert_eq!(e.mem.u32(quality), 1);
        let hr = e.vcall(d3d, 4 * 11, &[0, 1, 0x16, 0, 4, quality]).u32();
        assert_eq!(hr, D3DERR_NOTAVAILABLE);
        assert_eq!(e.mem.u32(quality), 0);
    }

    #[test]
    #[should_panic(expected = "was not captured")]
    fn a_query_outside_the_capture_stops() {
        let (mut e, d3d) = d3d();
        e.vcall(d3d, 4 * 10, &[0, 1, 0x16, 0, 3, 0x1234]);
    }

    #[test]
    #[should_panic(expected = "adapter table is not loaded")]
    fn without_a_table_the_first_query_stops() {
        let mut e = Engine::new();
        let d3d = direct3d_create9(&mut e, D3D_SDK_VERSION);
        let mode = e.mem.alloc(16);
        e.vcall(d3d, 4 * 8, &[0, mode]);
    }

    #[test]
    fn other_sdk_versions_get_null_and_ex_is_not_provided() {
        let mut e = Engine::new();
        assert_eq!(direct3d_create9(&mut e, 31), 0);
        let ex = proc_address(&mut e, b"Direct3DCreate9Ex").unwrap();
        assert!(e.is_translated(PROC_CODE));
        assert!(!e.is_translated(ex));
        assert_eq!(proc_address(&mut e, b"D3DPERF_BeginEvent"), None);
    }
}

#[cfg(test)]
mod tests_display_devices {
    use super::*;

    #[test]
    fn display_devices_come_from_the_table_and_end_with_false() {
        let mut e = Engine::new();
        let mut device = vec![0u8; 0x1a8];
        device[0] = 0xa8;
        device[1] = 1;
        device[0xa4] = 5;
        let hex: String = device.iter().map(|x| format!("{x:02x}")).collect();
        let empty: String = [0xa8u8, 1, 0, 0]
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect();
        let table =
            format!("EnumDisplayDevicesA\t0 0\t1\t{hex}\nEnumDisplayDevicesA\t1 0\t0\t{empty}\n");
        load_adapter_table(&mut e, &table);
        let info = e.mem.alloc(0x1a8);
        e.mem.set_u32(info, 0x1a8);
        assert_eq!(
            enum_display_devices_a(&mut e, Ptr::new(0), 0, Ptr::new(info), 0),
            1
        );
        assert_eq!(e.mem.u8(info + 0xa4), 5);
        e.mem.set_u8(info + 0xa4, 9);
        assert_eq!(
            enum_display_devices_a(&mut e, Ptr::new(0), 1, Ptr::new(info), 0),
            0
        );
        assert_eq!(e.mem.u8(info + 0xa4), 9);
    }
}

#[cfg(test)]
mod tests_processor {
    use super::*;

    #[test]
    fn the_processor_answers_come_from_the_table() {
        let mut e = Engine::new();
        // TEST DATA: made-up processor lines.
        load_adapter_table(&mut e, "CpuIdFlagTest\t\t200000\nCpuid\t0\t10 68747541 444d4163 69746e65\nCpuid\t1\ta20f10 800 7ed8320b 178bfbff\n");
        assert_eq!(e.call(0x00ed_193c, &[]).u32(), 0x20_0000);
        let out = e.mem.alloc(16);
        e.call(0x00ed_1964, &[1, out]);
        assert_eq!(e.mem.u32(out + 0xc), 0x178b_fbff);
        e.call(0x00ed_1951, &[0, out]);
        assert_eq!(e.mem.u32(out + 4), 0x6874_7541);
    }
}
