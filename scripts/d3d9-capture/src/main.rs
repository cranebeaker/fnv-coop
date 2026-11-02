//! d3d9-capture: what Direct3D 9 answers on this machine for adapter 0, as the table the platform's
//! Direct3D 9 layer reads (`engine_core::units::platform::d3d9`), written to stdout. Each line:
//! the query, its words (space separated, hex), the HRESULT or count, and the bytes it wrote as
//! hex. Built without a DPI-awareness manifest, as FalloutNV.exe runs, so the screen metrics are
//! the ones the game sees. The table describes the player's own hardware: it stays on the
//! player's machine (git-ignored), never in the repository. Windows only:
//! `cargo run --release --manifest-path scripts/d3d9-capture/Cargo.toml > %LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv`
//! (or any file `NV_D3D9_ADAPTER` names). The `nvapi.dll` answers are appended to the same table
//! by `scripts/nvapi-capture` (32-bit, as the DLL).
use std::ffi::c_void;
use std::io::Write;

mod device;

#[link(name = "d3d9")]
extern "system" {
    fn Direct3DCreate9(sdk: u32) -> *mut *const usize;
}
#[link(name = "user32")]
extern "system" {
    fn GetSystemMetrics(index: i32) -> i32;
    fn RegisterClassA(class: *const WndClassA) -> u16;
    fn UnregisterClassA(name: *const u8, instance: *mut c_void) -> i32;
    fn CreateWindowExA(
        ex_style: u32,
        class: *const u8,
        title: *const u8,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: *mut c_void,
        menu: *mut c_void,
        instance: *mut c_void,
        param: *mut c_void,
    ) -> *mut c_void;
    fn DestroyWindow(window: *mut c_void) -> i32;
    fn GetWindowLongA(window: *mut c_void, index: i32) -> i32;
    fn GetClassLongA(window: *mut c_void, index: i32) -> u32;
    fn AdjustWindowRectEx(rect: *mut [i32; 4], style: u32, menu: i32, ex_style: u32) -> i32;
    fn DefWindowProcA(window: *mut c_void, message: u32, w: usize, l: isize) -> isize;
    fn EnumDisplayDevicesA(device: *const u8, index: u32, info: *mut u8, flags: u32) -> i32;
}
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetWindowsDirectoryA(buffer: *mut u8, size: u32) -> u32;
    fn GetTimeZoneInformation(info: *mut u8) -> u32;
    fn GetTimeZoneInformationForYear(year: u16, dynamic: *mut c_void, info: *mut u8) -> i32;
    fn GetACP() -> u32;
    fn GetOEMCP() -> u32;
    fn IsValidCodePage(code_page: u32) -> i32;
    fn GetCPInfo(code_page: u32, info: *mut u8) -> i32;
}
#[link(name = "shell32")]
extern "system" {
    fn SHGetFolderPathA(owner: *mut c_void, folder: i32, token: *mut c_void, flags: u32, path: *mut u8) -> i32;
}

/// The folders the game asks `SHGetFolderPathA` for (`CSIDL_PERSONAL` 5, `CSIDL_APPDATA` 0x1a,
/// `CSIDL_LOCAL_APPDATA` 0x1c, `CSIDL_COMMON_APPDATA` 0x23, `CSIDL_COMMON_DOCUMENTS` 0x2e) and
/// the Windows directory (where profile files named without a path are), as the game's ANSI
/// strings, hex.
unsafe fn folders(o: &mut impl Write) {
    let mut buffer = vec![0u8; 260];
    let n = GetWindowsDirectoryA(buffer.as_mut_ptr(), 260);
    writeln!(o, "GetWindowsDirectoryA\t\t{n:x}\t{}", hex(&buffer[..n as usize])).unwrap();
    for folder in [5, 0x1a, 0x1c, 0x23, 0x2e] {
        buffer.iter_mut().for_each(|b| *b = 0);
        let hr = SHGetFolderPathA(std::ptr::null_mut(), folder, std::ptr::null_mut(), 0, buffer.as_mut_ptr());
        let len = buffer.iter().position(|&b| b == 0).unwrap_or(0);
        writeln!(o, "SHGetFolderPathA\t{folder:x} 0\t{:x}\t{}", hr as u32, hex(&buffer[..len])).unwrap();
    }
}

/// The player's time zone: `GetTimeZoneInformation`'s answer (`TIME_ZONE_ID_*`) and the 172
/// bytes of the `TIME_ZONE_INFORMATION` it wrote, hex. The platform's `GetTimeZoneInformation`,
/// `FileTimeToLocalFileTime` and the CRT's `_tzset` read it; which bias is in effect is worked
/// out from its rules and the real clock when the game asks.
unsafe fn time_zone(o: &mut impl Write) {
    let mut info = vec![0u8; 172];
    let id = GetTimeZoneInformation(info.as_mut_ptr());
    writeln!(o, "GetTimeZoneInformation\t\t{id:x}\t{}", hex(&info)).unwrap();
    // The rules Windows keeps for each year (`SystemTimeToTzSpecificLocalTime` follows them; they
    // can differ from the current ones): runs of years with the same answer merged, each line the
    // first and last year, the answer and the bytes.
    let mut run: Option<(u16, u16, i32, Vec<u8>)> = None;
    for year in 1601..=30827u16 {
        let mut bytes = vec![0u8; 172];
        let ok = GetTimeZoneInformationForYear(year, std::ptr::null_mut(), bytes.as_mut_ptr());
        match &mut run {
            Some(r) if r.2 == ok && r.3 == bytes => r.1 = year,
            _ => {
                if let Some(r) = run.take() {
                    zone_year_line(o, &r);
                }
                run = Some((year, year, ok, bytes));
            }
        }
    }
    if let Some(r) = run {
        zone_year_line(o, &r);
    }
}

fn zone_year_line(o: &mut impl Write, r: &(u16, u16, i32, Vec<u8>)) {
    writeln!(o, "GetTimeZoneInformationForYear\t{:x} {:x}\t{:x}\t{}", r.0, r.1, r.2 as u32, hex(&r.3)).unwrap();
}

/// The display devices as `EnumDisplayDevicesA(NULL, index, info, 0)` lists them (the game's
/// renderer start-up looks for the primary one): each index's answer and the 0x1A8-byte
/// `DISPLAY_DEVICEA` after the call (`cb` set, the rest zero before it), hex, through the first
/// index that answers 0.
unsafe fn display_devices(o: &mut impl Write) {
    for index in 0..64u32 {
        let mut info = vec![0u8; 0x1a8];
        info[0] = 0xa8;
        info[1] = 1;
        let answer = EnumDisplayDevicesA(std::ptr::null(), index, info.as_mut_ptr(), 0);
        writeln!(o, "EnumDisplayDevicesA\t{index:x} 0\t{:x}\t{}", answer as u32, hex(&info)).unwrap();
        if answer == 0 {
            break;
        }
    }
}

/// `WNDCLASSA`.
#[repr(C)]
struct WndClassA {
    style: u32,
    procedure: unsafe extern "system" fn(*mut c_void, u32, usize, isize) -> isize,
    class_extra: i32,
    window_extra: i32,
    instance: *mut c_void,
    icon: *mut c_void,
    cursor: *mut c_void,
    background: *mut c_void,
    menu_name: *const u8,
    class_name: *const u8,
}

/// The window styles the game creates or reads back (its main window `0x10000000`, the
/// renderer's child `0x50000000`) and the common frame styles, for `AdjustWindowRectEx`.
const ADJUST_STYLES: &[u32] = &[
    0, 0x1000_0000, 0x5000_0000, 0x8000_0000, 0x9000_0000, 0x00cf_0000, 0x10cf_0000, 0x00c0_0000,
    0x10c0_0000, 0x14c0_0000, 0x16c0_0000, 0x04c0_0000, 0x06c0_0000, 0x00ca_0000, 0x10ca_0000,
];

/// Records the window side: the style and class word a window made as the game makes its main
/// window (and a child in it) reads back, and `AdjustWindowRectEx`'s border for each style.
unsafe fn windows(o: &mut impl Write) {
    let instance = GetModuleHandleA(std::ptr::null());
    let name = b"nv-rs d3d9-capture\0";
    let class = WndClassA {
        style: 3,
        procedure: DefWindowProcA,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    RegisterClassA(&class);
    for (requested, parent_style) in [(0x1000_0000u32, None), (0x5000_0000, Some(0x1000_0000u32))] {
        let parent = match parent_style {
            Some(style) => CreateWindowExA(0, name.as_ptr(), name.as_ptr(), style, 0, 0, 320, 240,
                std::ptr::null_mut(), std::ptr::null_mut(), instance, std::ptr::null_mut()),
            None => std::ptr::null_mut(),
        };
        let window = CreateWindowExA(0, name.as_ptr(), name.as_ptr(), requested, 0, 0, 320, 240,
            parent, std::ptr::null_mut(), instance, std::ptr::null_mut());
        let style = GetWindowLongA(window, -16) as u32;
        let ex_style = GetWindowLongA(window, -20) as u32;
        let menu_name = GetClassLongA(window, -8);
        writeln!(o, "CreateWindowStyle\t{requested:x} {}\t{style:x}\t{ex_style:x}\t{menu_name:x}",
            u32::from(parent_style.is_some())).unwrap();
        DestroyWindow(window);
        if !parent.is_null() {
            DestroyWindow(parent);
        }
    }
    UnregisterClassA(name.as_ptr(), instance);
    for &style in ADJUST_STYLES {
        for menu in [0i32, 1] {
            for ex_style in [0u32, 0x100, 0x40000] {
                let mut rect = [0i32, 0, 0, 0];
                let ok = AdjustWindowRectEx(&mut rect, style, menu, ex_style);
                let rect: Vec<String> = rect.iter().map(|v| format!("{:x}", *v as u32)).collect();
                writeln!(o, "AdjustWindowRectEx\t{style:x} {menu:x} {ex_style:x}\t{ok:x}\t{}", rect.join(" ")).unwrap();
            }
        }
    }
}

type Fn1 = unsafe extern "system" fn(*mut c_void) -> u32;
type Fn2 = unsafe extern "system" fn(*mut c_void, u32) -> usize;
type Fn3 = unsafe extern "system" fn(*mut c_void, u32, u32) -> u32;
type Fn3P = unsafe extern "system" fn(*mut c_void, u32, u32, *mut u8) -> u32;
type Fn2P = unsafe extern "system" fn(*mut c_void, u32, *mut u8) -> u32;
type Fn4P = unsafe extern "system" fn(*mut c_void, u32, u32, u32, *mut u8) -> u32;
type Fn5 = unsafe extern "system" fn(*mut c_void, u32, u32, u32, u32, u32) -> u32;
type Fn6 = unsafe extern "system" fn(*mut c_void, u32, u32, u32, u32, u32, u32) -> u32;
type Fn6P = unsafe extern "system" fn(*mut c_void, u32, u32, u32, u32, u32, *mut u32) -> u32;
type Fn4 = unsafe extern "system" fn(*mut c_void, u32, u32, u32, u32) -> u32;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// D3DFORMAT values (d3d9types.h), FourCCs included (the DXTn ones and the vendor formats
/// games probe for).
const FORMATS: &[u32] = &[
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 40, 41, 50, 51, 52, 60,
    61, 62, 63, 64, 67, 70, 71, 73, 75, 77, 79, 80, 81, 82, 83, 84, 85, 100, 101, 102, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 199, 0x3154_5844, 0x3254_5844, 0x3354_5844,
    0x3454_5844, 0x3554_5844, 0x5956_5955, 0x3259_5559, 0x4742_5247, 0x4247_5247, 0x3154_454d,
    0x5a54_4e49, 0x5a57_4152, 0x3631_4644, 0x3432_4644, 0x4c4c_554e, 0x3149_5441, 0x3249_5441,
    0x5453_4e49, 0x5a53_4552, 0x434f_5441, 0x4141_5353, 0x4244_564e, 0x4256_3252, 0x3631_4c41,
];
const DISPLAY_FORMATS: &[u32] = &[21, 22, 23, 24, 25, 35];
const DEPTH_FORMATS: &[u32] = &[
    70, 71, 73, 75, 77, 79, 80, 82, 83, 84, 0x5a54_4e49, 0x5a57_4152, 0x3631_4644, 0x3432_4644,
];
const USAGES: &[u32] = &[
    0, 1, 2, 0x200, 0x400, 0x2000, 0x8000, 0x10000, 0x20000, 0x40000, 0x80000, 0x100000,
    0x200000,
];

/// The processor's answers the CRT's `__get_sse2_info` (`00ed1927`) asks for: the `ID` flag test
/// (EFLAGS read back after flipping bit 21, minus the entry flags, low 32 bits: `0x00200000`
/// when the bit starts clear) and `CPUID` leaves 0 and 1 (`EAX EBX ECX EDX`). Leaf 1's `EBX`
/// high byte is the APIC id of whichever core ran this; only `EDX` bit 26 decides anything.
fn processor(o: &mut impl Write) {
    let change: u64;
    // SAFETY: flips EFLAGS.ID and puts the entry flags back.
    unsafe {
        std::arch::asm!(
            "pushfq", "pop rax", "mov rcx, rax", "xor rax, 0x200000", "push rax", "popfq",
            "pushfq", "pop rdx", "push rcx", "popfq", "sub rdx, rcx",
            out("rax") _, out("rcx") _, out("rdx") change,
        );
    }
    writeln!(o, "CpuIdFlagTest\t\t{:x}", change as u32).unwrap();
    for leaf in [0u32, 1] {
        // SAFETY: CPUID is available on every x86-64 processor.
        #[allow(unused_unsafe)] // `__cpuid` is unsafe on older toolchains.
        let r = unsafe { std::arch::x86_64::__cpuid(leaf) };
        writeln!(o, "Cpuid\t{leaf:x}\t{:x} {:x} {:x} {:x}", r.eax, r.ebx, r.ecx, r.edx).unwrap();
    }
}

/// The code pages the C runtime asks about (`GetACP`, `GetOEMCP`; `IsValidCodePage` and
/// `GetCPInfo` for those two and the common pages 1252, 437 and 65001): each answer and the
/// `CPINFO` bytes (20: `MaxCharSize`, `DefaultChar`, `LeadByte`).
fn code_pages(o: &mut impl Write) {
    // SAFETY: plain queries; `info` is a 20-byte buffer as `CPINFO` is.
    unsafe {
        let (ansi, oem) = (GetACP(), GetOEMCP());
        writeln!(o, "GetACP		{ansi:x}").unwrap();
        writeln!(o, "GetOEMCP		{oem:x}").unwrap();
        let mut pages = vec![ansi, oem, 1252, 437, 65001];
        pages.dedup();
        for page in pages {
            writeln!(o, "IsValidCodePage	{page:x}	{:x}", IsValidCodePage(page) as u32).unwrap();
            let mut info = [0u8; 20];
            let r = GetCPInfo(page, info.as_mut_ptr());
            let hex: String = info.iter().map(|b| format!("{b:02x}")).collect();
            writeln!(o, "GetCPInfo	{page:x}	{:x}	{hex}", r as u32).unwrap();
        }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut o = out.lock();
    // `d3d9-capture device`: the device calls' answers (general behaviour, `device.rs`).
    if std::env::args().nth(1).as_deref() == Some("device") {
        unsafe { device::measure(&mut o) };
        return;
    }
    // `d3d9-capture codepages`: the code-page lines alone.
    if std::env::args().nth(1).as_deref() == Some("codepages") {
        code_pages(&mut o);
        return;
    }
    // `d3d9-capture cpu`: the processor lines alone (to add them to a table made before them).
    processor(&mut o);
    if std::env::args().nth(1).as_deref() == Some("cpu") {
        return;
    }
    let mut checks: Vec<(&str, String, String)> = Vec::new();
    unsafe {
        for index in (0..=0x5f).chain(0x1000..=0x1003).chain(0x2000..=0x2003) {
            writeln!(o, "GetSystemMetrics\t{index:x}\t{:x}", GetSystemMetrics(index) as u32).unwrap();
        }
        windows(&mut o);
        folders(&mut o);
        time_zone(&mut o);
        code_pages(&mut o);
        display_devices(&mut o);
        let d3d = Direct3DCreate9(32);
        assert!(!d3d.is_null());
        let vt = *d3d;
        let slot = |i: usize| *vt.add(i);
        let this = d3d as *mut c_void;
        let count: Fn1 = std::mem::transmute(slot(4));
        writeln!(o, "GetAdapterCount\t\t{:x}", count(this)).unwrap();
        let mut id = vec![0u8; 1100];
        let ident: Fn3P = std::mem::transmute(slot(5));
        for flags in [0u32, 2] {
            let hr = ident(this, 0, flags, id.as_mut_ptr());
            writeln!(o, "GetAdapterIdentifier\t0 {flags:x}\t{hr:x}\t{}", hex(&id)).unwrap();
        }
        let mut mode = vec![0u8; 16];
        let dm: Fn2P = std::mem::transmute(slot(8));
        let hr = dm(this, 0, mode.as_mut_ptr());
        writeln!(o, "GetAdapterDisplayMode\t0\t{hr:x}\t{}", hex(&mode)).unwrap();
        let mc: Fn3 = std::mem::transmute(slot(6));
        let em: Fn4P = std::mem::transmute(slot(7));
        for &format in FORMATS {
            let n = mc(this, 0, format);
            writeln!(o, "GetAdapterModeCount\t0 {format:x}\t{n:x}").unwrap();
            for i in 0..n {
                let hr = em(this, 0, format, i, mode.as_mut_ptr());
                writeln!(o, "EnumAdapterModes\t0 {format:x} {i:x}\t{hr:x}\t{}", hex(&mode)).unwrap();
            }
        }
        let mut caps = vec![0u8; 304];
        let gc: Fn3P = std::mem::transmute(slot(14));
        for devtype in [1u32, 2, 3] {
            caps.iter_mut().for_each(|b| *b = 0);
            let hr = gc(this, 0, devtype, caps.as_mut_ptr());
            writeln!(o, "GetDeviceCaps\t0 {devtype:x}\t{hr:x}\t{}", hex(&caps)).unwrap();
        }
        let cdt: Fn5 = std::mem::transmute(slot(9));
        for devtype in [1u32, 2] {
            for &display in DISPLAY_FORMATS {
                for &back in DISPLAY_FORMATS.iter().chain([0u32].iter()) {
                    for windowed in [0u32, 1] {
                        let hr = cdt(this, 0, devtype, display, back, windowed);
                        let words = format!("0 {devtype:x} {display:x} {back:x} {windowed:x}");
                        checks.push(("CheckDeviceType", words, format!("{hr:x}")));
                    }
                }
            }
        }
        let cf: Fn6 = std::mem::transmute(slot(10));
        for devtype in [1u32, 2] {
            for &display in DISPLAY_FORMATS {
                for &usage in USAGES {
                    for rtype in 1u32..=7 {
                        for &format in FORMATS {
                            let hr = cf(this, 0, devtype, display, usage, rtype, format);
                            let words =
                                format!("0 {devtype:x} {display:x} {usage:x} {rtype:x} {format:x}");
                            checks.push(("CheckDeviceFormat", words, format!("{hr:x}")));
                        }
                    }
                }
            }
        }
        let ms: Fn6P = std::mem::transmute(slot(11));
        for devtype in [1u32, 2] {
            for &format in FORMATS {
                for windowed in [0u32, 1] {
                    for samples in 0u32..=16 {
                        let mut quality = 0u32;
                        let hr = ms(this, 0, devtype, format, windowed, samples, &mut quality);
                        let words = format!("0 {devtype:x} {format:x} {windowed:x} {samples:x}");
                        checks.push((
                            "CheckDeviceMultiSampleType",
                            words,
                            format!("{hr:x}/{quality:x}"),
                        ));
                    }
                }
            }
        }
        let dsm: Fn5 = std::mem::transmute(slot(12));
        for devtype in [1u32, 2] {
            for &display in DISPLAY_FORMATS {
                for &target in FORMATS {
                    for &depth in DEPTH_FORMATS {
                        let hr = dsm(this, 0, devtype, display, target, depth);
                        let words = format!("0 {devtype:x} {display:x} {target:x} {depth:x}");
                        checks.push(("CheckDepthStencilMatch", words, format!("{hr:x}")));
                    }
                }
            }
        }
        let conv: Fn4 = std::mem::transmute(slot(13));
        for devtype in [1u32, 2] {
            for &source in FORMATS {
                for &target in DISPLAY_FORMATS {
                    let hr = conv(this, 0, devtype, source, target);
                    let words = format!("0 {devtype:x} {source:x} {target:x}");
                    checks.push(("CheckDeviceFormatConversion", words, format!("{hr:x}")));
                }
            }
        }
        let monitor: Fn2 = std::mem::transmute(slot(15));
        let has_monitor = u32::from(monitor(this, 0) != 0);
        writeln!(o, "GetAdapterMonitor\t0\t{has_monitor:x}").unwrap();
        let release: Fn1 = std::mem::transmute(slot(2));
        release(this);
    }
    fold(&mut o, &checks);
}

/// Writes the `Check*` answers folded: one line per query and leading words,
/// `Kind<TAB>words<TAB>default result<TAB>last=result,...<TAB>domain`, listing only the last
/// words whose result differs from the line's most common one; the domain (the last words
/// asked, comma separated) is written once as `Domain<TAB>dN<TAB>list` and named after that.
fn fold(o: &mut impl Write, checks: &[(&str, String, String)]) {
    let mut order: Vec<(String, String)> = Vec::new();
    let mut rows: std::collections::HashMap<(String, String), Vec<(String, String)>> =
        std::collections::HashMap::new();
    for (kind, words, result) in checks {
        let (lead, last) = words.rsplit_once(' ').unwrap();
        let key = (kind.to_string(), lead.to_string());
        if !rows.contains_key(&key) {
            order.push(key.clone());
        }
        rows.entry(key)
            .or_default()
            .push((last.to_string(), result.clone()));
    }
    let mut domains: Vec<String> = Vec::new();
    for key in order {
        let row = &rows[&key];
        let mut counts: Vec<(String, usize)> = Vec::new();
        for (_, result) in row {
            match counts.iter_mut().find(|(r, _)| r == result) {
                Some(c) => c.1 += 1,
                None => counts.push((result.clone(), 1)),
            }
        }
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let default = &counts[0].0;
        let exceptions: Vec<String> = row
            .iter()
            .filter(|(_, r)| r != default)
            .map(|(w, r)| format!("{w}={r}"))
            .collect();
        let domain: Vec<&str> = row.iter().map(|(w, _)| w.as_str()).collect();
        let domain = domain.join(",");
        let index = match domains.iter().position(|d| *d == domain) {
            Some(i) => i,
            None => {
                writeln!(o, "Domain\td{}\t{domain}", domains.len()).unwrap();
                domains.push(domain);
                domains.len() - 1
            }
        };
        writeln!(
            o,
            "{}\t{}\t{default}\t{}\td{index}",
            key.0,
            key.1,
            exceptions.join(",")
        )
        .unwrap();
    }
}
