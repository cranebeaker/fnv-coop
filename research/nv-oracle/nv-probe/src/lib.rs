//! nv-probe: an in-process probe for 32-bit Windows programs.
//!
//! The DLL reads `nv-probe.txt` from its own folder (format in
//! `nv_oracle_core::manifest`), checks the host program and each function's
//! bytes, patches the entry of each listed function with a jump to a
//! generated stub, and writes JSON lines describing every call: arguments by
//! declared type, memory dumps, and optionally the return value.
//!
//! It can be loaded two ways:
//!
//! - injected with `nv-inject` (or any `LoadLibrary` caller). With
//!   `mode = inject` the hooks are installed inside `DllMain`, before
//!   `LoadLibrary` returns, so a process started suspended is fully hooked
//!   before it runs its first instruction.
//! - as an NVSE plugin, from the loader's plugin folder. NVSE calls
//!   `NVSEPlugin_Query` and `NVSEPlugin_Load`; the hooks are installed in
//!   `NVSEPlugin_Load` (`mode = nvse`).
//!
//! `mode = auto` (the default) cannot tell the two apart inside `DllMain`,
//! so it waits `nvse_grace_ms` for NVSE to call in; if no call arrives it
//! installs on its own thread. Nothing is installed twice.
//!
//! A probe with a return-capturing hook pins itself in memory, because a
//! thread inside a hooked function returns through a stub that calls back
//! into the DLL. Without such a hook, `FreeLibrary` unloads the DLL and puts
//! the original bytes back.
//!
//! The ABI facts used for NVSE are only these: `PluginInfo` is
//! `{u32 infoVersion = 1, const char *name, u32 version}`, `NVSEInterface`
//! starts with `u32 nvseVersion, runtimeVersion, editorVersion, isEditor`,
//! and the two exports take and return as shown below. The probe refuses to
//! load in the editor (`isEditor != 0`).

#![cfg(all(windows, target_arch = "x86"))]
#![allow(non_snake_case, clippy::missing_safety_doc)]

mod log;
mod memory;
mod runtime;
mod stubs;

use nv_oracle_core::json::{self, ObjectWriter};
use nv_oracle_core::manifest::{self, Manifest, Mode};
use nv_oracle_core::{decode, sha256};
use nv_win as win;
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering::*};

static MODULE: AtomicU32 = AtomicU32::new(0);
/// 0 = not started, 1 = starting or started.
static STARTED: AtomicBool = AtomicBool::new(false);
static NVSE_SEEN: AtomicBool = AtomicBool::new(false);
static INSTALLED: AtomicBool = AtomicBool::new(false);

const MANIFEST_NAME: &str = "nv-probe.txt";
const DEFAULT_OUTPUT: &str = "nv-probe.jsonl";
const STUB_REGION: u32 = 256 * 1024;

#[repr(C)]
pub struct PluginInfo {
    pub info_version: u32,
    pub name: *const u8,
    pub version: u32,
}

#[repr(C)]
pub struct NvseInterface {
    pub nvse_version: u32,
    pub runtime_version: u32,
    pub editor_version: u32,
    pub is_editor: u32,
}

static PLUGIN_NAME: &[u8] = b"nv-probe\0";

#[no_mangle]
pub unsafe extern "C" fn NVSEPlugin_Query(
    nvse: *const NvseInterface,
    info: *mut PluginInfo,
) -> bool {
    NVSE_SEEN.store(true, SeqCst);
    if nvse.is_null() || info.is_null() || (*nvse).is_editor != 0 {
        return false;
    }
    (*info).info_version = 1;
    (*info).name = PLUGIN_NAME.as_ptr();
    (*info).version = 1;
    true
}

#[no_mangle]
pub unsafe extern "C" fn NVSEPlugin_Load(nvse: *const NvseInterface) -> bool {
    NVSE_SEEN.store(true, SeqCst);
    if nvse.is_null() || (*nvse).is_editor != 0 {
        return false;
    }
    start(Origin::Nvse);
    true
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    module: *mut c_void,
    reason: u32,
    reserved: *mut c_void,
) -> i32 {
    match reason {
        win::DLL_PROCESS_ATTACH => {
            MODULE.store(module as usize as u32, SeqCst);
            on_attach();
        }
        win::DLL_THREAD_DETACH => runtime::free_thread_state(),
        win::DLL_PROCESS_DETACH => on_detach(!reserved.is_null()),
        _ => {}
    }
    1
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Origin {
    DllMain,
    Nvse,
    AutoThread,
}

impl Origin {
    fn name(self) -> &'static str {
        match self {
            Origin::DllMain => "dllmain",
            Origin::Nvse => "nvse",
            Origin::AutoThread => "auto-thread",
        }
    }
}

fn dll_dir() -> Option<PathBuf> {
    let mut buf = vec![0u16; 4096];
    // SAFETY: the buffer and its length are passed together.
    let n = unsafe {
        win::GetModuleFileNameW(
            MODULE.load(SeqCst) as usize as win::Handle,
            buf.as_mut_ptr(),
            buf.len() as u32,
        )
    };
    if n == 0 || n as usize >= buf.len() {
        return None;
    }
    let path = PathBuf::from(String::from_utf16_lossy(&buf[..n as usize]));
    path.parent().map(Path::to_path_buf)
}

fn host_exe() -> Option<PathBuf> {
    let mut buf = vec![0u16; 4096];
    // SAFETY: the buffer and its length are passed together.
    let n = unsafe {
        win::GetModuleFileNameW(std::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32)
    };
    if n == 0 || n as usize >= buf.len() {
        return None;
    }
    Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
}

fn manifest_text() -> Result<(PathBuf, String), String> {
    let dir = dll_dir().ok_or("cannot find the DLL's folder")?;
    let path = dir.join(MANIFEST_NAME);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    Ok((path, text))
}

fn read_manifest() -> Result<Manifest, String> {
    let (path, text) = manifest_text()?;
    Manifest::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn on_attach() {
    // The mode decides what happens now. Only `mode` and `nvse_grace_ms`
    // are read here, so a manifest with a mistake elsewhere still starts at
    // the right time and the mistake is reported then.
    let mode = manifest_text()
        .map(|(_, t)| manifest::peek_settings(&t))
        .unwrap_or((Mode::Auto, 1500));
    match mode.0 {
        Mode::Inject => start(Origin::DllMain),
        Mode::Nvse => {}
        Mode::Auto => {
            // SAFETY: creating a thread that only runs `auto_thread`.
            unsafe {
                let h = win::CreateThread(
                    std::ptr::null_mut(),
                    0,
                    auto_thread,
                    mode.1 as usize as *mut c_void,
                    0,
                    std::ptr::null_mut(),
                );
                if !h.is_null() {
                    win::CloseHandle(h);
                }
            }
        }
    }
}

unsafe extern "system" fn auto_thread(grace_ms: *mut c_void) -> u32 {
    win::Sleep(grace_ms as usize as u32);
    if !NVSE_SEEN.load(SeqCst) {
        start(Origin::AutoThread);
    }
    0
}

fn on_detach(process_exit: bool) {
    if !process_exit && INSTALLED.load(SeqCst) {
        uninstall();
    }
    log::flush_now(false);
}

/// Start the probe once, whichever path gets here first.
fn start(origin: Origin) {
    if STARTED.swap(true, SeqCst) {
        return;
    }
    install(origin);
}

fn describe_refusal(spec: &nv_oracle_core::manifest::HookSpec, reason: &str) -> String {
    let mut o = ObjectWriter::new();
    o.str("name", &spec.name)
        .hex32("addr", spec.addr)
        .str("status", "refused")
        .str("reason", reason);
    o.finish()
}

fn output_path(manifest: Option<&Manifest>, dir: &Path) -> PathBuf {
    let text = manifest
        .and_then(|m| m.output.clone())
        .unwrap_or_else(|| DEFAULT_OUTPUT.to_string());
    // SAFETY: plain query.
    let pid = unsafe { win::GetCurrentProcessId() };
    let text = text.replace("%PID%", &pid.to_string());
    let p = PathBuf::from(text);
    if p.is_absolute() {
        p
    } else {
        dir.join(p)
    }
}

fn install(origin: Origin) {
    let dir = dll_dir().unwrap_or_else(|| PathBuf::from("."));
    let parsed = read_manifest();
    let manifest = parsed.as_ref().ok();
    let out_path = output_path(manifest, &dir);
    let flush_ms = manifest.map_or(250, |m| m.flush_ms);
    let logger = match log::Logger::open(&out_path, flush_ms) {
        Ok(l) => l,
        Err(e) => {
            // Install nothing, and leave a note where the user will look.
            let note = format!(
                "nv-probe could not open its output file {}: {e}\nNo hooks were installed.\n",
                out_path.display()
            );
            let _ = std::fs::write(dir.join("nv-probe-error.txt"), note);
            return;
        }
    };
    // Held until the header is written, so no call record can precede it.
    let mut guard = log::lock();
    *guard = Some(logger);

    // SAFETY: plain queries.
    let (pid, base) = unsafe {
        (
            win::GetCurrentProcessId(),
            win::GetModuleHandleW(std::ptr::null()),
        )
    };
    let mut header = ObjectWriter::new();
    header
        .str("type", "header")
        .str("tool", "nv-probe")
        .str("version", env!("CARGO_PKG_VERSION"));
    header
        .str("origin", origin.name())
        .u64("pid", u64::from(pid))
        .hex32("host_base", base as usize as u32);
    header.str("output", &out_path.display().to_string());

    let manifest = match parsed {
        Ok(m) => m,
        Err(e) => {
            header
                .str("manifest_error", &e)
                .raw("hooks", "[]")
                .bool("installed", false);
            push(&mut guard, &header.finish());
            return;
        }
    };
    let mode_name = match manifest.mode {
        Mode::Auto => "auto",
        Mode::Inject => "inject",
        Mode::Nvse => "nvse",
    };
    header
        .str("mode", mode_name)
        .bool("suspend_threads", manifest.suspend_threads);
    let exe = host_exe();
    if let Some(e) = &exe {
        header.str("host_exe", &e.display().to_string());
    }

    // The log always says which binary it measured: the host exe is hashed
    // whether or not the manifest asks for the hash to be checked. When it
    // does, the host must be the program the manifest was written for.
    let hash = exe
        .as_ref()
        .ok_or_else(|| "cannot find the host exe".to_string())
        .and_then(|p| std::fs::File::open(p).map_err(|e| e.to_string()))
        .and_then(|f| sha256::digest_reader(f).map_err(|e| e.to_string()));
    match &hash {
        Ok(have) => {
            header.str("host_sha256", &nv_oracle_core::hex::encode(have));
        }
        Err(e) => {
            header.str("host_sha256_error", e);
        }
    }
    let mut refuse_all: Option<String> = None;
    match (manifest.host_sha256, &hash) {
        (None, _) => {
            header.str("host_sha256_check", "not_requested");
        }
        (Some(want), Ok(have)) if *have == want => {
            header.str("host_sha256_check", "match");
        }
        (Some(_), Ok(_)) => {
            header.str("host_sha256_check", "mismatch");
            refuse_all = Some("host exe SHA-256 does not match host_sha256".into());
        }
        (Some(_), Err(e)) => {
            header.str("host_sha256_check", "error");
            refuse_all = Some(format!("cannot hash the host exe: {e}"));
        }
    }

    let mut statuses: Vec<String> = Vec::new();
    let mut hooks: Vec<runtime::HookRt> = Vec::new();
    let mut region = None;
    if let Some(reason) = &refuse_all {
        for spec in &manifest.hooks {
            statuses.push(describe_refusal(spec, reason));
        }
    } else if !manifest.hooks.is_empty() {
        match stubs::Region::new(STUB_REGION) {
            Ok(r) => region = Some(r),
            Err(e) => {
                for spec in &manifest.hooks {
                    statuses.push(describe_refusal(spec, &e));
                }
            }
        }
    }
    if let Some(r) = region.as_mut() {
        if !runtime::init_tls() {
            header.str("error", "cannot allocate thread-local storage");
        } else {
            for spec in &manifest.hooks {
                match prepare(spec, r, manifest.max_records) {
                    Ok(h) => hooks.push(h),
                    Err(e) => statuses.push(describe_refusal(spec, &e)),
                }
            }
        }
    }

    // A function hooked with return capture comes back through a stub that
    // calls into this DLL, possibly long after it was entered. If the DLL could
    // be unloaded meanwhile, that thread would return into unmapped memory, so
    // such a probe keeps itself in the process. When that fails, those hooks
    // are refused: `FreeLibrary` would otherwise unload the probe under them.
    let mut pinned = false;
    if hooks.iter().any(|h| h.ret_stub != 0) {
        pinned = pin_module();
    }
    let (kept, refused) = split_for_pin(hooks, pinned, |h| h.ret_stub != 0);
    hooks = kept;
    for h in &refused {
        statuses.push(describe_refusal(
            &h.spec,
            "return capture needs the probe to stay loaded, and keeping it loaded failed (no hook with ret= is installed)",
        ));
    }

    // Publish the table, then patch (with other threads stopped).
    let mut installed_json: Vec<String> = Vec::new();
    if !hooks.is_empty() {
        let table = runtime::publish(runtime::Table { hooks });
        let frozen = if manifest.suspend_threads {
            memory::Frozen::freeze_others()
        } else {
            memory::Frozen::none()
        };
        // Nothing between here and `thaw` may allocate: a suspended thread
        // could be holding the heap lock.
        let mut ranges: Vec<(u32, u32, u32)> = Vec::with_capacity(table.hooks.len());
        let mut results: Vec<Result<(), &'static str>> = Vec::with_capacity(table.hooks.len());
        for h in &table.hooks {
            match memory::write_jump(h.spec.addr, h.entry_stub, &h.spec.expected) {
                Ok(p) => {
                    *h.patch.lock().unwrap_or_else(|e| e.into_inner()) = Some(p);
                    h.installed.store(true, SeqCst);
                    ranges.push((h.spec.addr, h.spec.steal as u32, h.trampoline));
                    results.push(Ok(()));
                }
                Err(e) => results.push(Err(e)),
            }
        }
        let moved = frozen.fix_instruction_pointers(&ranges);
        let frozen_count = frozen.count();
        frozen.thaw();
        header
            .u64("threads_suspended", frozen_count as u64)
            .u64("threads_moved", moved as u64);
        for (h, r) in table.hooks.iter().zip(results) {
            match r {
                Ok(()) => installed_json.push(runtime::hook_summary(h)),
                Err(e) => statuses.push(describe_refusal(&h.spec, e)),
            }
        }
        INSTALLED.store(table.hooks.iter().any(|h| h.installed.load(SeqCst)), SeqCst);
    }
    installed_json.extend(statuses);
    header
        .raw("hooks", &json::array_of(&installed_json))
        .bool("installed", INSTALLED.load(SeqCst))
        .bool("pinned", pinned);
    push(&mut guard, &header.finish());
}

/// Hooks that may be installed, and hooks that must be refused. Without a pin
/// (the module could not be kept loaded) a hook that captures the return must
/// not be installed; the others do not depend on the DLL staying loaded.
fn split_for_pin<T>(
    hooks: Vec<T>,
    pinned: bool,
    captures_return: impl Fn(&T) -> bool,
) -> (Vec<T>, Vec<T>) {
    if pinned {
        return (hooks, Vec::new());
    }
    hooks.into_iter().partition(|h| !captures_return(h))
}

/// Keep this DLL mapped until the process ends: `FreeLibrary` then does not
/// unload it, and its `DllMain` is not told to detach before the process
/// exits.
fn pin_module() -> bool {
    let mut module: win::Handle = std::ptr::null_mut();
    // SAFETY: the address is a function of this module, as the flag says.
    unsafe {
        win::GetModuleHandleExW(
            win::GET_MODULE_HANDLE_EX_FLAG_PIN | win::GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            pin_module as *const () as *const u16,
            &mut module,
        ) != 0
    }
}

fn push(guard: &mut std::sync::MutexGuard<'_, Option<log::Logger>>, line: &str) {
    if let Some(l) = guard.as_mut() {
        l.push_line(line);
        l.flush();
    }
}

/// Validate a hook against live memory and build its stubs.
fn prepare(
    spec: &nv_oracle_core::manifest::HookSpec,
    region: &mut stubs::Region,
    default_max: u64,
) -> Result<runtime::HookRt, String> {
    let live = memory::read_bytes(spec.addr, spec.expected.len())?;
    if let Some(i) = live.iter().zip(&spec.expected).position(|(a, b)| a != b) {
        return Err(format!(
            "expected bytes do not match memory: first difference at offset {i} (expected {:02x}, found {:02x})",
            spec.expected[i], live[i]
        ));
    }
    let shapes: Vec<_> = spec.args.iter().map(|a| a.shape()).collect();
    let layout = nv_oracle_core::abi::layout(spec.cc, &shapes)?;
    let trampoline = region.alloc(spec.steal + decode::JMP_LEN)?;
    let code = decode::build_trampoline(&live, spec.steal, spec.addr, trampoline)
        .map_err(|e| e.to_string())?;
    region.write(trampoline, &code);

    let id = NEXT_ID.fetch_add(1, SeqCst);
    let enter = runtime::enter as *const () as usize as u32;
    let leave = runtime::leave as *const () as usize as u32;
    let entry_stub = region.alloc(96)?;
    region.write(
        entry_stub,
        &stubs::entry_stub(entry_stub, id, enter, region.mxcsr_const(), trampoline),
    );
    let ret_stub = if spec.ret.is_some() {
        let a = region.alloc(96)?;
        region.write(a, &stubs::return_stub(id, leave, region.mxcsr_const()));
        a
    } else {
        0
    };
    Ok(runtime::HookRt {
        spec: spec.clone(),
        layout,
        trampoline,
        entry_stub,
        ret_stub,
        max: spec.max.unwrap_or(default_max),
        calls: std::sync::atomic::AtomicU64::new(0),
        patch: std::sync::Mutex::new(None),
        installed: AtomicBool::new(false),
    })
}

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// Put the original bytes back (the DLL is being unloaded). Only a probe
/// without return capture gets here: one with it has pinned itself. The stubs
/// stay allocated, but they call into this DLL, so a thread that is inside one
/// of them at this moment is lost.
fn uninstall() {
    let Some(table) = runtime::table() else {
        return;
    };
    let frozen = memory::Frozen::freeze_others();
    for h in &table.hooks {
        if h.installed.swap(false, SeqCst) {
            if let Some(p) = *h.patch.lock().unwrap_or_else(|e| e.into_inner()) {
                let _ = memory::restore(&p);
            }
        }
    }
    frozen.thaw();
    INSTALLED.store(false, SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unpinned_probe_refuses_return_capture() {
        // Hooks stand in as their return-stub address (0 for none).
        let (kept, refused) = split_for_pin(vec![0u32, 5, 0, 7], false, |&h| h != 0);
        assert_eq!(kept, [0, 0]);
        assert_eq!(refused, [5, 7]);
        // With the pin in place nothing is refused.
        let (kept, refused) = split_for_pin(vec![0u32, 5, 0, 7], true, |&h| h != 0);
        assert_eq!(kept, [0, 5, 0, 7]);
        assert!(refused.is_empty());
    }
}
