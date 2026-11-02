//! nv-trace-rec: the recorder DLL for the real FalloutNV.exe.
//!
//! `nv-trace-launch` starts the game suspended and loads this DLL into it before the game
//! runs an instruction. In `DllMain` the DLL reads `nv-trace-rec.txt` from its own folder and,
//! in this order:
//!
//! 1. installs the player-folder guard ([`guard`]): nothing in the process can change a file
//!    in a protected folder from here on. Failure ends the process at once;
//! 2. redirects the exe's folder lookups to the scratch copies and answers its Steam imports
//!    as with no Steam client ([`iat`]). Failure ends the process too;
//! 3. starts the trace file and hooks the listed functions ([`hooks`], [`ring`]);
//! 4. arms the key script ([`input`]).
//!
//! Config (`key=value` lines, written by the launcher):
//!
//! ```text
//! out=C:\...\real.nvtrace      the trace file
//! list=C:\...\speech.txt       the function list (research/nv-trace/lists)
//! log=C:\...\rec.log           what the recorder did, guard refusals, keys
//! protect=C:\...               a protected folder (one line each, at least one)
//! personal=C:\...\Documents    CSIDL_PERSONAL for the exe
//! local_appdata=C:\...         CSIDL_LOCAL_APPDATA for the exe
//! appdata=C:\...               CSIDL_APPDATA for the exe
//! keys=1500:d0:1,...           the key script (optional)
//! keys_fps=58                  the script's frames are times at this rate (optional)
//! keys_delay_ms=30000           with keys_fps: every key this much later (optional)
//! keys_after=007cb7d0:1470       with keys_fps: the clock starts at the first call of 007cb7d0
//!                               (it must be in the list), which the port made at frame 1470
//! shot_every_ms=30000           a screenshot of the game window this often (optional)
//! shot_full=1                  screenshots at the window's full size (default half)
//! mouse_mute=always            the game's mouse reads nothing for the whole run (default: only
//!                               while a key script runs)
//! keys2=0:1e:2,10:1c:2,...      a second key script on its own clock (optional)
//! keys2_after=0064aec0:0        its clock starts at the first call of this function
//! keys2_delay_ms=10000          and every key this much later
//! exe_sha256=<hex>             hooks are refused unless the exe has this hash (optional)
//! overlay_root=C:\...          a protected folder whose new files go to the overlay
//! overlay=C:\...\overlay       where they go (same relative names)
//! ```

#![cfg(all(windows, target_arch = "x86"))]
#![allow(non_snake_case, clippy::missing_safety_doc)]

mod d3d;
mod dinput;
mod guard;
mod hooks;
mod iat;
mod input;
mod ring;
mod shot;
mod stubs;
mod sys;

use nv_oracle_core::sha256;
use nv_trace_core::guard::Protected;
use nv_trace_core::{format, keys, list};
use nv_win as win;
use std::ffi::c_void;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering::*};
use std::sync::Mutex;

static MODULE: AtomicU32 = AtomicU32::new(0);
static LOG: Mutex<Option<std::fs::File>> = Mutex::new(None);

const CONFIG_NAME: &str = "nv-trace-rec.txt";
const STUB_REGION: u32 = 1024 * 1024;

/// Appends a line to the recorder's log.
pub(crate) fn log(line: &str) {
    // SAFETY: plain query.
    let tick = unsafe { win::GetTickCount() };
    if let Ok(mut f) = LOG.lock() {
        if let Some(f) = f.as_mut() {
            let _ = writeln!(f, "{tick:>10} {line}");
        }
    }
}

/// The end of the run (the exe's `ExitProcess`): the trace is drained and closed.
pub(crate) fn finish_run(code: u32) {
    log(&format!(
        "exit {code}: {} frames, {} guard refusals, {} records dropped",
        hooks::FRAMES.load(Relaxed),
        guard::REFUSED.load(Relaxed),
        ring::dropped()
    ));
    ring::finish(true);
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
    PathBuf::from(String::from_utf16_lossy(&buf[..n as usize]))
        .parent()
        .map(Path::to_path_buf)
}

fn host_exe() -> Option<PathBuf> {
    let mut buf = vec![0u16; 4096];
    // SAFETY: the buffer and its length are passed together.
    let n = unsafe {
        win::GetModuleFileNameW(std::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32)
    };
    (n != 0 && (n as usize) < buf.len())
        .then(|| PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
}

#[derive(Default)]
struct Config {
    out: Option<PathBuf>,
    list: Option<PathBuf>,
    log: Option<PathBuf>,
    protect: Vec<String>,
    personal: Option<String>,
    local_appdata: Option<String>,
    appdata: Option<String>,
    keys: String,
    keys_fps: u32,
    keys_delay_ms: u64,
    keys_after: Option<(u32, u64)>,
    keys2: String,
    keys2_after: Option<(u32, u64)>,
    keys2_delay_ms: u64,
    shot_full: bool,
    shot_every_ms: u32,
    exe_sha256: Option<String>,
    overlay_root: Option<String>,
    overlay: Option<String>,
}

/// An `ADDR:PORT_FRAME` event (a function whose first call starts a key script's clock, and
/// the port's frame of the same event).
fn event(path: &Path, v: &str) -> Result<(u32, u64), String> {
    let bad = || format!("{}: bad event {v:?} (ADDR:PORT_FRAME)", path.display());
    let (a, f) = v.split_once(':').ok_or_else(bad)?;
    Ok((
        u32::from_str_radix(a.trim_start_matches("0x"), 16).map_err(|_| bad())?,
        f.parse().map_err(|_| bad())?,
    ))
}

fn read_config(dir: &Path) -> Result<Config, String> {
    let path = dir.join(CONFIG_NAME);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut c = Config::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line
            .split_once('=')
            .ok_or_else(|| format!("{}: bad line {line:?}", path.display()))?;
        let v = v.trim().to_string();
        match k.trim() {
            "out" => c.out = Some(v.into()),
            "list" => c.list = Some(v.into()),
            "log" => c.log = Some(v.into()),
            "protect" => c.protect.push(v),
            "personal" => c.personal = Some(v),
            "local_appdata" => c.local_appdata = Some(v),
            "appdata" => c.appdata = Some(v),
            "keys" => c.keys = v,
            "keys_after" => c.keys_after = Some(event(&path, &v)?),
            "keys2" => c.keys2 = v,
            "keys2_after" => c.keys2_after = Some(event(&path, &v)?),
            "keys2_delay_ms" => {
                c.keys2_delay_ms = v
                    .parse()
                    .map_err(|_| format!("{}: bad keys2_delay_ms {v:?}", path.display()))?
            }
            "shot_full" => c.shot_full = v == "1",
            "keyboard_mute" => {
                input::KEYBOARD_MUTE.store(v == "always", std::sync::atomic::Ordering::Relaxed)
            }
            "mouse_mute" => {
                input::MUTE_ALWAYS.store(v == "always", std::sync::atomic::Ordering::Relaxed)
            }
            "shot_every_ms" => {
                c.shot_every_ms = v
                    .parse()
                    .map_err(|_| format!("{}: bad shot_every_ms {v:?}", path.display()))?
            }
            "keys_delay_ms" => {
                c.keys_delay_ms = v
                    .parse()
                    .map_err(|_| format!("{}: bad keys_delay_ms {v:?}", path.display()))?
            }
            "keys_fps" => {
                c.keys_fps = v
                    .parse()
                    .map_err(|_| format!("{}: bad keys_fps {v:?}", path.display()))?
            }
            "exe_sha256" => c.exe_sha256 = Some(v.to_ascii_lowercase()),
            "overlay_root" => c.overlay_root = Some(v),
            "overlay" => c.overlay = Some(v),
            other => return Err(format!("{}: unknown key {other:?}", path.display())),
        }
    }
    Ok(c)
}

/// Ends the process before the game runs: the run can't be made safe or can't record.
fn refuse(dir: &Path, why: &str) -> ! {
    log(&format!("REFUSED TO RUN: {why}"));
    let _ = std::fs::write(
        dir.join("nv-trace-rec-error.txt"),
        format!("nv-trace-rec ended the game before it ran: {why}\n"),
    );
    // SAFETY: ending this process.
    unsafe { win::TerminateProcess(win::GetCurrentProcess(), 0x4e57) };
    std::process::abort();
}

fn start() {
    let dir = dll_dir().unwrap_or_else(|| PathBuf::from("."));
    let config = match read_config(&dir) {
        Ok(c) => c,
        Err(e) => refuse(&dir, &e),
    };
    if let Some(p) = &config.log {
        if let Ok(f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
        {
            *LOG.lock().unwrap_or_else(|e| e.into_inner()) = Some(f);
        }
    }
    // SAFETY: plain query.
    log(&format!(
        "nv-trace-rec {} in process {}",
        env!("CARGO_PKG_VERSION"),
        unsafe { win::GetCurrentProcessId() }
    ));
    let mut region = match stubs::Region::new(STUB_REGION) {
        Ok(r) => r,
        Err(e) => refuse(&dir, &e),
    };
    // 1. The guard, before anything else.
    let protected = Protected::new(config.protect.iter().map(String::as_str));
    for r in protected.roots() {
        log(&format!("guard: protecting {r}"));
    }
    if let (Some(root), Some(dir)) = (&config.overlay_root, &config.overlay) {
        guard::set_overlay(root, dir);
        log(&format!("guard: new files under {root} go to {dir}"));
    }
    if let Err(e) = guard::install(protected, &mut region) {
        refuse(&dir, &format!("guard: {e}"));
    }
    log("guard: installed");
    // 2. Folders, Steam, ExitProcess.
    let (Some(personal), Some(local_appdata), Some(appdata)) = (
        config.personal.clone(),
        config.local_appdata.clone(),
        config.appdata.clone(),
    ) else {
        refuse(
            &dir,
            "personal, local_appdata and appdata must all be configured",
        );
    };
    match iat::install(iat::Folders {
        personal,
        local_appdata,
        appdata,
    }) {
        Ok(lines) => lines.iter().for_each(|l| log(l)),
        Err(e) => refuse(&dir, &format!("imports: {e}")),
    }
    // 3. The trace.
    let exe = host_exe();
    let hash = exe
        .as_ref()
        .and_then(|p| std::fs::File::open(p).ok())
        .and_then(|f| sha256::digest_reader(f).ok())
        .map(|h| nv_oracle_core::hex::encode(&h).to_ascii_lowercase());
    let list_text = config
        .list
        .as_ref()
        .map(|p| std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display())))
        .transpose();
    let specs = match list_text.and_then(|t| list::parse(&t.unwrap_or_default())) {
        Ok(s) => s,
        Err(e) => refuse(&dir, &format!("list: {e}")),
    };
    let hash_ok = match (&config.exe_sha256, &hash) {
        (None, _) => true,
        (Some(want), Some(have)) => want == have,
        (Some(_), None) => false,
    };
    let status = if hash_ok {
        hooks::install(&specs, &mut region)
    } else {
        vec![format!(
            "hooks refused: exe SHA-256 {} is not the configured {}",
            hash.as_deref().unwrap_or("?"),
            config.exe_sha256.as_deref().unwrap_or("?")
        )]
    };
    status.iter().for_each(|l| log(l));
    let mut header = String::from("source=real\nformat=1\n");
    header.push_str(&format!(
        "exe={}\nexe_sha256={}\n",
        exe.as_ref().map_or("?".into(), |p| p.display().to_string()),
        hash.as_deref().unwrap_or("?")
    ));
    // SAFETY: plain query.
    header.push_str(&format!("pid={}\n", unsafe { win::GetCurrentProcessId() }));
    if let Some(p) = &config.list {
        header.push_str(&format!("list={}\n", p.display()));
    }
    for s in &specs {
        header.push_str(&format!("fn={}\n", s.line()));
    }
    for s in &status {
        header.push_str(&format!("{s}\n"));
    }
    header.push_str(&format!(
        "keys={}\nkeys_fps={}\n",
        config.keys, config.keys_fps
    ));
    let out = config
        .out
        .clone()
        .unwrap_or_else(|| dir.join("real.nvtrace"));
    if let Some(run) = out.parent() {
        shot::set_dir(run.to_path_buf());
    }
    if let Err(e) = ring::start(&out, &format::encode_header(&header)) {
        refuse(&dir, &format!("trace: {e}"));
    }
    // 4. Keys.
    shot::set_full(config.shot_full);
    match keys::parse(&config.keys2) {
        Ok(script) => input::set_script(
            1,
            script,
            input::KeyTiming {
                fps: config.keys_fps,
                delay_ms: config.keys2_delay_ms,
                after: config.keys2_after,
                shot_every_ms: 0,
            },
        ),
        Err(e) => refuse(&dir, &format!("keys2: {e}")),
    }
    match keys::parse(&config.keys) {
        Ok(script) => input::set_script(
            0,
            script,
            input::KeyTiming {
                fps: config.keys_fps,
                delay_ms: config.keys_delay_ms,
                after: config.keys_after,
                shot_every_ms: config.shot_every_ms,
            },
        ),
        Err(e) => refuse(&dir, &format!("keys: {e}")),
    }
    // The hooks' return stubs call into this DLL: it must never be unloaded.
    let mut module: win::Handle = std::ptr::null_mut();
    // SAFETY: the address is a function of this module, as the flag says.
    unsafe {
        win::GetModuleHandleExW(
            win::GET_MODULE_HANDLE_EX_FLAG_PIN | win::GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            start as *const () as *const u16,
            &mut module,
        )
    };
    log("ready");
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    module: *mut c_void,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    match reason {
        win::DLL_PROCESS_ATTACH => {
            MODULE.store(module as usize as u32, SeqCst);
            start();
        }
        win::DLL_PROCESS_DETACH => {
            log("process detach");
            ring::finish(false);
        }
        _ => {}
    }
    1
}

/// `len` bytes at `addr` if all are readable.
pub(crate) fn memory_bytes(addr: u32, len: usize) -> Option<Vec<u8>> {
    if !win::is_readable(addr, len) {
        return None;
    }
    let mut out = vec![0u8; len];
    // SAFETY: the range was just checked to be committed and readable.
    unsafe { std::ptr::copy_nonoverlapping(addr as usize as *const u8, out.as_mut_ptr(), len) };
    Some(out)
}
