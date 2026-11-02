//! nv-trace-launch: records the real FalloutNV.exe without touching the player's folders.
//!
//! ```text
//! nv-trace-launch --list FILE [--keys SCRIPT] [--seconds N] [--exe FILE] [--game DIR]
//!                 [--scratch DIR] [--name NAME] [--no-saves] [--prepare-only]
//! ```
//!
//! 1. Finds the player's folders: the game folder, `Documents\My Games\FalloutNV` (the
//!    Documents folder as the shell reports it, and `%USERPROFILE%\Documents` too),
//!    `%LOCALAPPDATA%\FalloutNV` and `%APPDATA%\FalloutNV`, and lists every file in them with
//!    its size and modification time.
//! 2. Makes a run folder under the scratch folder: the exe copy, the recorder DLL and its
//!    config, and copies of the player's INI files, `plugins.txt`, `NVDLCList.txt` and (unless
//!    `--no-saves`) the saves, all read from the originals, never written there.
//! 3. Starts the exe from the run folder suspended, with the game folder as its working
//!    directory (the game reads `Data\` relative to it), loads the recorder (which installs
//!    the guard before anything else), resumes it, and waits `--seconds` (default 300) or
//!    until it exits. Then it asks the recorder to drain the trace and ends the game.
//! 4. Lists the player's folders again and reports any file that appeared, disappeared or
//!    changed size or time. Exit status 0 only when nothing changed.

#![cfg_attr(not(all(windows, target_arch = "x86")), allow(dead_code))]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[cfg(not(all(windows, target_arch = "x86")))]
fn main() {
    eprintln!("nv-trace-launch is a 32-bit Windows program (i686-pc-windows-msvc)");
    std::process::exit(2);
}

/// Every file under `dir` with (size, modified), by path. A missing folder is empty.
fn listing(dir: &Path, out: &mut BTreeMap<PathBuf, (u64, Option<SystemTime>)>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    out.insert(
        dir.to_path_buf(),
        (
            0,
            std::fs::metadata(dir).ok().and_then(|m| m.modified().ok()),
        ),
    );
    for e in rd.flatten() {
        let p = e.path();
        match std::fs::symlink_metadata(&p) {
            Ok(m) if m.is_dir() => listing(&p, out),
            Ok(m) => {
                out.insert(p, (m.len(), m.modified().ok()));
            }
            Err(_) => {}
        }
    }
}

fn snapshot(dirs: &[PathBuf]) -> BTreeMap<PathBuf, (u64, Option<SystemTime>)> {
    let mut out = BTreeMap::new();
    for d in dirs {
        listing(d, &mut out);
    }
    out
}

fn compare(
    before: &BTreeMap<PathBuf, (u64, Option<SystemTime>)>,
    after: &BTreeMap<PathBuf, (u64, Option<SystemTime>)>,
) -> Vec<String> {
    let mut changes = Vec::new();
    for (p, b) in before {
        match after.get(p) {
            None => changes.push(format!("removed: {}", p.display())),
            Some(a) if a != b => changes.push(format!(
                "changed: {} (size {} -> {}, modified {:?} -> {:?})",
                p.display(),
                b.0,
                a.0,
                b.1,
                a.1
            )),
            _ => {}
        }
    }
    for p in after.keys() {
        if !before.contains_key(p) {
            changes.push(format!("added: {}", p.display()));
        }
    }
    changes
}

fn copy_tree(from: &Path, to: &Path) -> std::io::Result<u64> {
    let mut n = 0;
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let p = e.path();
        if e.file_type()?.is_dir() {
            n += copy_tree(&p, &to.join(e.file_name()))?;
        } else {
            std::fs::copy(&p, to.join(e.file_name()))?;
            n += 1;
        }
    }
    Ok(n)
}

#[cfg(all(windows, target_arch = "x86"))]
mod win32 {
    use nv_win as win;
    use std::ffi::c_void;

    #[link(name = "shell32")]
    extern "system" {
        pub fn SHGetFolderPathW(
            hwnd: *mut c_void,
            csidl: i32,
            token: *mut c_void,
            flags: u32,
            path: *mut u16,
        ) -> i32;
    }
    #[link(name = "kernel32")]
    extern "system" {
        pub fn CreateEventW(
            attrs: *mut c_void,
            manual: i32,
            initial: i32,
            name: *const u16,
        ) -> win::Handle;
        pub fn SetEvent(h: win::Handle) -> i32;
    }

    pub fn folder(csidl: i32) -> Option<std::path::PathBuf> {
        let mut buf = [0u16; 260];
        // SAFETY: a MAX_PATH buffer as the API requires.
        let hr = unsafe {
            SHGetFolderPathW(
                std::ptr::null_mut(),
                csidl,
                std::ptr::null_mut(),
                0,
                buf.as_mut_ptr(),
            )
        };
        if hr != 0 {
            return None;
        }
        let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        Some(String::from_utf16_lossy(&buf[..n]).into())
    }
}

#[cfg(all(windows, target_arch = "x86"))]
fn main() {
    std::process::exit(match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("nv-trace-launch: {e}");
            2
        }
    });
}

/// The installed game (this machine's Steam library).
const STEAM_GAME: &str = r"C:\Games\Steam\steamapps\common\Fallout New Vegas";

#[cfg(all(windows, target_arch = "x86"))]
fn run() -> Result<i32, String> {
    use nv_win as win;
    let mut list: Option<PathBuf> = None;
    let mut keys = String::new();
    let mut keys_fps: u32 = 0;
    let mut keys_delay_ms: u64 = 0;
    let mut keys_after = String::new();
    let mut keys2 = String::new();
    let mut keys2_after = String::new();
    let mut keys2_delay_ms: u64 = 0;
    let mut shot_full = false;
    let mut mute_mouse = false;
    let mut mute_keyboard = false;
    let mut shot_every_s: u32 = 30;
    let mut always_active = true;
    let mut seconds: u32 = 300;
    let profile = PathBuf::from(std::env::var_os("USERPROFILE").unwrap_or_default());
    let mut exe = profile.join(r"nv-re\FalloutNV_unpacked.exe");
    let mut game = PathBuf::from(STEAM_GAME);
    let mut scratch = profile.join(r"nv-re\work\build43-scratch\measure\rec");
    let mut name: Option<String> = None;
    let mut saves = true;
    let mut prepare_only = false;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().ok_or(format!("{a} needs a value"));
        match a.as_str() {
            "--list" => list = Some(val()?.into()),
            "--keys" => keys = val()?,
            "--keys-fps" => keys_fps = val()?.parse().map_err(|_| "bad --keys-fps")?,
            "--keys-after" => keys_after = val()?,
            "--keys2" => keys2 = val()?,
            "--keys2-after" => keys2_after = val()?,
            "--keys2-delay-ms" => {
                keys2_delay_ms = val()?.parse().map_err(|_| "bad --keys2-delay-ms")?
            }
            "--shot-full" => shot_full = true,
            "--mute-mouse" => mute_mouse = true,
            "--mute-keyboard" => mute_keyboard = true,
            "--shot-every-s" => shot_every_s = val()?.parse().map_err(|_| "bad --shot-every-s")?,
            "--no-always-active" => always_active = false,
            "--keys-delay-ms" => {
                keys_delay_ms = val()?.parse().map_err(|_| "bad --keys-delay-ms")?
            }
            "--seconds" => seconds = val()?.parse().map_err(|_| "bad --seconds")?,
            "--exe" => exe = val()?.into(),
            "--game" => game = val()?.into(),
            "--scratch" => scratch = val()?.into(),
            "--name" => name = Some(val()?),
            "--no-saves" => saves = false,
            "--prepare-only" => prepare_only = true,
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let list = list.ok_or("--list FILE is required")?;
    let list = std::fs::canonicalize(&list).map_err(|e| format!("{}: {e}", list.display()))?;
    nv_trace_core::list::parse(&std::fs::read_to_string(&list).map_err(|e| e.to_string())?)?;
    nv_trace_core::keys::parse(&keys)?;
    nv_trace_core::keys::parse(&keys2)?;

    // 1. The player's folders.
    let documents = win32::folder(0x05).ok_or("cannot find the Documents folder")?;
    let local = win32::folder(0x1c).ok_or("cannot find the local application data folder")?;
    let roaming = win32::folder(0x1a).ok_or("cannot find the application data folder")?;
    let profile = std::env::var_os("USERPROFILE").map(PathBuf::from);
    let my_games = documents.join("My Games").join("FalloutNV");
    let mut protect: Vec<PathBuf> = vec![
        game.clone(),
        // The installed game stays protected whatever --game says.
        PathBuf::from(STEAM_GAME),
        my_games.clone(),
        local.join("FalloutNV"),
        roaming.join("FalloutNV"),
    ];
    protect.dedup();
    if let Some(p) = &profile {
        protect.push(p.join("Documents").join("My Games").join("FalloutNV"));
    }
    // Their final paths as well (a junction or a OneDrive move gives a second name).
    for p in protect.clone() {
        if let Ok(c) = std::fs::canonicalize(&p) {
            let s = c.display().to_string();
            let s = s.strip_prefix(r"\\?\").unwrap_or(&s).to_string();
            if !protect
                .iter()
                .any(|q| q.as_os_str().eq_ignore_ascii_case(s.as_str()))
            {
                protect.push(s.into());
            }
        }
    }
    // The scratch folder must not lie inside a protected one, nor contain one.
    let scratch_n = nv_trace_core::guard::normalize(&scratch.display().to_string());
    let guard =
        nv_trace_core::guard::Protected::new(protect.iter().map(|p| p.to_str().unwrap_or("")));
    if guard.refuses(&scratch_n) {
        return Err(format!(
            "the scratch folder {} overlaps a protected folder",
            scratch.display()
        ));
    }
    println!("protected folders:");
    for p in &protect {
        println!("  {}", p.display());
    }
    let before = snapshot(&protect);
    println!("listed {} entries in them", before.len());

    // 2. The run folder.
    let stamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let run = scratch.join(name.unwrap_or_else(|| format!("run-{stamp}")));
    let bin = run.join("game");
    let docs = run.join("Documents");
    let run_local = run.join("LocalAppData");
    let run_roaming = run.join("AppData");
    for d in [
        &bin,
        &docs.join("My Games").join("FalloutNV"),
        &run_local.join("FalloutNV"),
        &run_roaming,
    ] {
        std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    let me = std::env::current_exe().map_err(|e| e.to_string())?;
    let dll_src = me.with_file_name("nv_trace_rec.dll");
    std::fs::copy(&exe, bin.join("FalloutNV.exe"))
        .map_err(|e| format!("{}: {e}", exe.display()))?;
    std::fs::copy(&dll_src, bin.join("nv_trace_rec.dll"))
        .map_err(|e| format!("{}: {e}", dll_src.display()))?;
    let run_my_games = docs.join("My Games").join("FalloutNV");
    for f in [
        "Fallout.ini",
        "FalloutPrefs.ini",
        "FalloutCustom.ini",
        "RendererInfo.txt",
    ] {
        let src = my_games.join(f);
        if src.is_file() {
            std::fs::copy(&src, run_my_games.join(f))
                .map_err(|e| format!("{}: {e}", src.display()))?;
        }
    }
    // The copies only (never the originals): bAlwaysActive=1 so the game keeps running
    // while its window is not in front (it pauses otherwise).
    if always_active {
        for f in ["Fallout.ini", "FalloutPrefs.ini"] {
            let p = run_my_games.join(f);
            if p.is_file() {
                set_ini(&p, "General", "bAlwaysActive", "1")?;
            }
        }
    }
    if saves && my_games.join("Saves").is_dir() {
        let n = copy_tree(&my_games.join("Saves"), &run_my_games.join("Saves"))
            .map_err(|e| e.to_string())?;
        println!("copied {n} save files");
    }
    for f in ["plugins.txt", "NVDLCList.txt"] {
        let src = local.join("FalloutNV").join(f);
        if src.is_file() {
            std::fs::copy(&src, run_local.join("FalloutNV").join(f))
                .map_err(|e| format!("{}: {e}", src.display()))?;
        }
    }
    let out = run.join("real.nvtrace");
    let log = run.join("rec.log");
    let mut cfg = String::new();
    cfg.push_str(&format!(
        "out={}\nlist={}\nlog={}\n",
        out.display(),
        list.display(),
        log.display()
    ));
    for p in &protect {
        cfg.push_str(&format!("protect={}\n", p.display()));
    }
    // New files the game makes in its working folder go to the run folder's overlay.
    cfg.push_str(&format!(
        "overlay_root={}\noverlay={}\n",
        game.display(),
        run.join("game-overlay").display()
    ));
    cfg.push_str(&format!(
        "personal={}\nlocal_appdata={}\nappdata={}\nkeys={keys}\nkeys_fps={keys_fps}\nkeys_delay_ms={keys_delay_ms}\nshot_every_ms={}\n{}",
        docs.display(),
        run_local.display(),
        run_roaming.display(),
        shot_every_s * 1000,
        if keys_after.is_empty() {
            String::new()
        } else {
            format!("keys_after={keys_after}\n")
        }
    ));
    cfg.push_str(&format!(
        "keys2={keys2}\nkeys2_delay_ms={keys2_delay_ms}\nshot_full={}\n",
        u8::from(shot_full)
    ));
    if mute_mouse {
        cfg.push_str("mouse_mute=always\n");
    }
    if mute_keyboard {
        cfg.push_str("keyboard_mute=always\n");
    }
    if !keys2_after.is_empty() {
        cfg.push_str(&format!("keys2_after={keys2_after}\n"));
    }
    std::fs::write(bin.join("nv-trace-rec.txt"), &cfg).map_err(|e| e.to_string())?;
    println!("run folder: {}", run.display());
    if prepare_only {
        let after = snapshot(&protect);
        let changes = compare(&before, &after);
        println!(
            "prepared only; player folders unchanged: {}",
            changes.is_empty()
        );
        return Ok(if changes.is_empty() { 0 } else { 1 });
    }

    // 3. Start suspended, load the recorder, resume.
    let exe_run = bin.join("FalloutNV.exe");
    let app = win::wide(exe_run.as_os_str());
    let mut cmd = win::wide(std::ffi::OsStr::new(&format!("\"{}\"", exe_run.display())));
    let cwd = win::wide(game.as_os_str());
    let si = win::StartupInfoW::new();
    let mut pi = win::ProcessInformation {
        process: std::ptr::null_mut(),
        thread: std::ptr::null_mut(),
        process_id: 0,
        thread_id: 0,
    };
    // SAFETY: valid NUL-terminated strings and output structures.
    let ok = unsafe {
        win::CreateProcessW(
            app.as_ptr(),
            cmd.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            win::CREATE_SUSPENDED,
            std::ptr::null_mut(),
            cwd.as_ptr(),
            &si,
            &mut pi,
        )
    };
    if ok == 0 {
        return Err(format!(
            "CreateProcessW: {}",
            win::error_text(unsafe { win::GetLastError() })
        ));
    }
    let pid = pi.process_id;
    // The recorder's stop event (it drains the trace when signalled).
    let stop_name = win::wide(std::ffi::OsStr::new(&format!(
        "Local\\nv-trace-rec-{pid}-stop"
    )));
    // SAFETY: a named manual-reset event.
    let stop = unsafe { win32::CreateEventW(std::ptr::null_mut(), 1, 0, stop_name.as_ptr()) };
    let dll = win::wide(bin.join("nv_trace_rec.dll").as_os_str());
    // SAFETY: remote allocation, copy, LoadLibraryW thread (kernel32 is at the same address
    // in both 32-bit processes of this boot session), as nv-inject does.
    let loaded = unsafe {
        let bytes = dll.len() * 2;
        let remote = win::VirtualAllocEx(
            pi.process,
            std::ptr::null_mut(),
            bytes,
            win::MEM_RESERVE | win::MEM_COMMIT,
            win::PAGE_READWRITE,
        );
        if remote.is_null() {
            0
        } else {
            let mut written = 0;
            win::WriteProcessMemory(
                pi.process,
                remote,
                dll.as_ptr() as *const c_void,
                bytes,
                &mut written,
            );
            let k32 =
                win::GetModuleHandleW(win::wide(std::ffi::OsStr::new("kernel32.dll")).as_ptr());
            let load = win::GetProcAddress(k32, c"LoadLibraryW".as_ptr() as *const u8);
            let t = win::CreateRemoteThread(
                pi.process,
                std::ptr::null_mut(),
                0,
                load,
                remote,
                0,
                std::ptr::null_mut(),
            );
            if t.is_null() {
                0
            } else {
                win::WaitForSingleObject(t, 60_000);
                let mut code = 0u32;
                win::GetExitCodeThread(t, &mut code);
                win::CloseHandle(t);
                code
            }
        }
    };
    let mut exit_code = 0u32;
    // SAFETY: the process handle we own.
    let alive =
        unsafe { win::GetExitCodeProcess(pi.process, &mut exit_code) != 0 && exit_code == 259 };
    if loaded == 0 || !alive {
        // SAFETY: ending the suspended process we started.
        unsafe { win::TerminateProcess(pi.process, 1) };
        let note = std::fs::read_to_string(bin.join("nv-trace-rec-error.txt")).unwrap_or_default();
        println!("the recorder did not start (module {loaded:#x}, process alive {alive}); the game never ran.\n{note}");
    } else {
        println!("recorder loaded in process {pid}; resuming the game for up to {seconds} s");
        // SAFETY: the main thread we created suspended.
        unsafe { win::ResumeThread(pi.thread) };
        // SAFETY: waiting on our own process handle.
        let waited = unsafe { win::WaitForSingleObject(pi.process, seconds.saturating_mul(1000)) };
        if waited != win::WAIT_OBJECT_0 {
            println!("time is up: draining the trace and ending the game");
            // SAFETY: the stop event we created; then ending the game.
            unsafe {
                win32::SetEvent(stop);
                win::WaitForSingleObject(pi.process, 1500);
                win::TerminateProcess(pi.process, 0);
                win::WaitForSingleObject(pi.process, 10_000);
            }
        }
        // SAFETY: our own handle.
        unsafe { win::GetExitCodeProcess(pi.process, &mut exit_code) };
        println!("game exit code {exit_code:#x}");
    }
    // SAFETY: closing our handles.
    unsafe {
        win::CloseHandle(pi.thread);
        win::CloseHandle(pi.process);
        if !stop.is_null() {
            win::CloseHandle(stop);
        }
    }

    // 4. The player's folders again.
    let after = snapshot(&protect);
    let changes = compare(&before, &after);
    let refusals = std::fs::read_to_string(&log)
        .unwrap_or_default()
        .lines()
        .filter(|l| l.contains("guard: refused"))
        .count();
    println!("guard refusals (in {}): {refusals}", log.display());
    if changes.is_empty() {
        println!(
            "player folders: unchanged ({} entries compared)",
            after.len()
        );
        println!("trace: {}", out.display());
        Ok(0)
    } else {
        println!("PLAYER FOLDERS CHANGED:");
        for c in &changes {
            println!("  {c}");
        }
        Ok(1)
    }
}

#[cfg(all(windows, target_arch = "x86"))]
use std::ffi::c_void;

/// Sets `key=value` in `[section]` of the INI file at `path` (a scratch copy; never an
/// original): replaces the line if the key is there, else adds it after the section header,
/// else adds the section. The file's read-only attribute (copied from the original) is
/// cleared first.
fn set_ini(path: &Path, section: &str, key: &str, value: &str) -> Result<(), String> {
    let mut perms = std::fs::metadata(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    perms.set_readonly(false);
    std::fs::set_permissions(path, perms).map_err(|e| format!("{}: {e}", path.display()))?;
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let out = ini_with(&text, section, key, value);
    std::fs::write(path, out).map_err(|e| format!("{}: {e}", path.display()))
}

fn ini_with(text: &str, section: &str, key: &str, value: &str) -> String {
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let header = format!("[{section}]");
    let mut in_section = false;
    let mut header_at = None;
    for (i, l) in lines.iter_mut().enumerate() {
        let t = l.trim();
        if t.starts_with('[') {
            in_section = t.eq_ignore_ascii_case(&header);
            if in_section {
                header_at = Some(i);
            }
            continue;
        }
        if in_section
            && t.split('=')
                .next()
                .is_some_and(|k| k.trim().eq_ignore_ascii_case(key))
        {
            *l = format!("{key}={value}");
            return lines.join("\r\n") + "\r\n";
        }
    }
    match header_at {
        Some(i) => lines.insert(i + 1, format!("{key}={value}")),
        None => {
            lines.push(header);
            lines.push(format!("{key}={value}"));
        }
    }
    lines.join("\r\n") + "\r\n"
}

#[cfg(test)]
mod tests {
    use super::ini_with;

    #[test]
    fn an_ini_key_is_replaced_or_added_in_its_section() {
        let t = "[General]\nsA=1\nbAlwaysActive=0\n[Display]\nbAlwaysActive=5\n";
        let o = ini_with(t, "General", "bAlwaysActive", "1");
        assert!(o.contains("[General]\r\nsA=1\r\nbAlwaysActive=1\r\n[Display]\r\nbAlwaysActive=5"));
        let o = ini_with("[General]\nsA=1\n", "General", "bAlwaysActive", "1");
        assert!(o.starts_with("[General]\r\nbAlwaysActive=1\r\nsA=1"));
        let o = ini_with("[Display]\nx=1\n", "General", "bAlwaysActive", "1");
        assert!(o.ends_with("[General]\r\nbAlwaysActive=1\r\n"));
    }
}
