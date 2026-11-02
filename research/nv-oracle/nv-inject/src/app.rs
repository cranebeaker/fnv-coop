use crate::cmdline;
use nv_win as win;
use std::ffi::{c_void, OsStr};
use std::path::{Path, PathBuf};

const USAGE: &str = "\
usage: nv-inject [--dll <path>] [--wait] --launch <exe> [args...]
       nv-inject [--dll <path>] --pid <n>

  --dll <path>    the DLL to load (default: nv_probe.dll next to nv-inject.exe)
  --wait          with --launch: wait for the program to exit and print its
                  exit code (nv-inject then exits with 3 if the code is not 0)
  --launch <exe>  start the program suspended, load the DLL, then resume it;
                  everything after the exe is passed to the program, so give
                  --dll and --wait before --launch
  --pid <n>       load the DLL into a running process
  -h, --help      this text

The injected process must be 32-bit, like this tool. Use a private copy of
the game, never the files of the real installation.
";

const ONE_TARGET: &str = "give only one of --launch <exe> and --pid <n>";

enum Target {
    Launch {
        exe: String,
        args: Vec<String>,
        wait: bool,
    },
    Pid(u32),
}

struct Options {
    dll: PathBuf,
    target: Target,
}

fn parse_args() -> Result<Options, String> {
    parse(std::env::args().skip(1))
}

/// Parse the arguments after the program name. Everything after the program
/// that `--launch` names belongs to that program.
fn parse(args: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut args = args;
    let mut dll: Option<PathBuf> = None;
    let mut target: Option<Target> = None;
    let mut wait = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            "--dll" => dll = Some(PathBuf::from(args.next().ok_or("--dll needs a path")?)),
            "--wait" => wait = true,
            "--pid" => {
                let n = args.next().ok_or("--pid needs a number")?;
                let pid = n.parse().map_err(|_| format!("bad process id {n:?}"))?;
                if target.is_some() {
                    return Err(ONE_TARGET.into());
                }
                target = Some(Target::Pid(pid));
            }
            "--launch" => {
                let exe = args.next().ok_or("--launch needs a program")?;
                let rest: Vec<String> = args.by_ref().collect();
                if target.is_some() {
                    return Err(ONE_TARGET.into());
                }
                target = Some(Target::Launch {
                    exe,
                    args: rest,
                    wait: false,
                });
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    match &mut target {
        Some(Target::Launch { wait: w, .. }) => *w = wait,
        // There is no process of ours to wait for when the DLL goes into a
        // running one, so a script that asks for it must be told.
        Some(Target::Pid(_)) if wait => return Err("--wait only works with --launch".into()),
        _ => {}
    }
    let target = target.ok_or("give --launch <exe> or --pid <n>")?;
    let dll = match dll {
        Some(d) => d,
        None => std::env::current_exe()
            .map_err(|e| e.to_string())?
            .with_file_name("nv_probe.dll"),
    };
    Ok(Options { dll, target })
}

fn full_path(p: &Path) -> Result<Vec<u16>, String> {
    let wide = win::wide(p.as_os_str());
    let mut buf = vec![0u16; 4096];
    // SAFETY: the buffer and its length are passed together.
    let n = unsafe {
        win::GetFullPathNameW(
            wide.as_ptr(),
            buf.len() as u32,
            buf.as_mut_ptr(),
            std::ptr::null_mut(),
        )
    };
    if n == 0 || n as usize >= buf.len() {
        return Err(format!("cannot resolve the path {}", p.display()));
    }
    buf.truncate(n as usize);
    buf.push(0);
    Ok(buf)
}

/// Load `dll` into `process`. Returns the low 32 bits of the module handle
/// the remote `LoadLibraryW` returned.
fn inject(process: win::Handle, dll: &[u16]) -> Result<u32, String> {
    let bytes = std::mem::size_of_val(dll);
    // SAFETY: remote allocation and write of the DLL path; every result is checked.
    unsafe {
        let remote = win::VirtualAllocEx(
            process,
            std::ptr::null_mut(),
            bytes,
            win::MEM_COMMIT | win::MEM_RESERVE,
            win::PAGE_READWRITE,
        );
        if remote.is_null() {
            return Err(format!(
                "VirtualAllocEx failed: {}",
                win::error_text(win::GetLastError())
            ));
        }
        let mut written = 0usize;
        if win::WriteProcessMemory(
            process,
            remote,
            dll.as_ptr() as *const c_void,
            bytes,
            &mut written,
        ) == 0
            || written != bytes
        {
            return Err(format!(
                "WriteProcessMemory failed: {}",
                win::error_text(win::GetLastError())
            ));
        }
        // kernel32 is mapped at the same address in every process of a
        // session, so the local address of LoadLibraryW is valid remotely.
        let k32 = win::GetModuleHandleW(win::wide(OsStr::new("kernel32.dll")).as_ptr());
        let load = win::GetProcAddress(k32, c"LoadLibraryW".as_ptr().cast());
        if load.is_null() {
            return Err("cannot find LoadLibraryW".into());
        }
        let mut tid = 0u32;
        let thread =
            win::CreateRemoteThread(process, std::ptr::null_mut(), 0, load, remote, 0, &mut tid);
        if thread.is_null() {
            return Err(format!(
                "CreateRemoteThread failed: {}",
                win::error_text(win::GetLastError())
            ));
        }
        win::WaitForSingleObject(thread, win::INFINITE);
        let mut code = 0u32;
        win::GetExitCodeThread(thread, &mut code);
        win::CloseHandle(thread);
        win::VirtualFreeEx(process, remote, 0, win::MEM_RELEASE);
        Ok(code)
    }
}

fn report(dll: &Path, pid: u32, handle: u32) -> bool {
    if handle == 0 {
        eprintln!(
            "nv-inject: LoadLibraryW failed in process {pid} for {}. Common causes: the file is missing, it is a \
             64-bit DLL, a dependency is missing, or its DllMain returned FALSE.",
            dll.display()
        );
        false
    } else {
        println!(
            "nv-inject: loaded {} into process {pid}, module handle 0x{handle:08X}",
            dll.display()
        );
        true
    }
}

pub fn main() -> i32 {
    let opts = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("nv-inject: {e}\n\n{USAGE}");
            return 2;
        }
    };
    if !opts.dll.is_file() {
        eprintln!("nv-inject: DLL not found: {}", opts.dll.display());
        return 1;
    }
    let dll = match full_path(&opts.dll) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("nv-inject: {e}");
            return 1;
        }
    };
    match opts.target {
        Target::Pid(pid) => {
            // SAFETY: opening a process by id; the result is checked.
            let process = unsafe { win::OpenProcess(win::PROCESS_ALL_ACCESS, 0, pid) };
            if process.is_null() {
                // SAFETY: plain query.
                let code = unsafe { win::GetLastError() };
                eprintln!(
                    "nv-inject: cannot open process {pid}: {}",
                    win::error_text(code)
                );
                return 1;
            }
            let result = inject(process, &dll);
            // SAFETY: handle opened above.
            unsafe { win::CloseHandle(process) };
            match result {
                Ok(h) => i32::from(!report(&opts.dll, pid, h)),
                Err(e) => {
                    eprintln!("nv-inject: {e}");
                    1
                }
            }
        }
        Target::Launch { exe, args, wait } => {
            for a in args
                .iter()
                .filter(|a| matches!(a.as_str(), "--wait" | "--dll"))
            {
                eprintln!(
                    "nv-inject: note: {a} after the program name is passed to the program, not to nv-inject; \
                     give it before --launch"
                );
            }
            let exe_path = PathBuf::from(&exe);
            let wide_exe = win::wide(exe_path.as_os_str());
            let mut cmd = win::wide(OsStr::new(&cmdline::command_line(&exe, &args)));
            let cwd = exe_path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .map(|p| win::wide(p.as_os_str()));
            let si = win::StartupInfoW::new();
            let mut pi = win::ProcessInformation {
                process: std::ptr::null_mut(),
                thread: std::ptr::null_mut(),
                process_id: 0,
                thread_id: 0,
            };
            // SAFETY: all pointers are valid for the call; `cmd` is mutable
            // as CreateProcessW requires.
            let ok = unsafe {
                win::CreateProcessW(
                    wide_exe.as_ptr(),
                    cmd.as_mut_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                    win::CREATE_SUSPENDED,
                    std::ptr::null_mut(),
                    cwd.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
                    &si,
                    &mut pi,
                )
            };
            if ok == 0 {
                // SAFETY: plain query.
                let code = unsafe { win::GetLastError() };
                eprintln!("nv-inject: cannot start {exe}: {}", win::error_text(code));
                return 1;
            }
            let injected = inject(pi.process, &dll);
            let loaded = match &injected {
                Ok(h) => report(&opts.dll, pi.process_id, *h),
                Err(e) => {
                    eprintln!("nv-inject: {e}");
                    false
                }
            };
            if !loaded {
                // SAFETY: handles from CreateProcessW.
                unsafe {
                    win::TerminateProcess(pi.process, 1);
                    win::CloseHandle(pi.thread);
                    win::CloseHandle(pi.process);
                }
                return 1;
            }
            // SAFETY: handles from CreateProcessW.
            unsafe {
                win::ResumeThread(pi.thread);
                win::CloseHandle(pi.thread);
            }
            let mut exit = 0;
            if wait {
                let mut code = 0u32;
                // SAFETY: handle from CreateProcessW.
                unsafe {
                    win::WaitForSingleObject(pi.process, win::INFINITE);
                    win::GetExitCodeProcess(pi.process, &mut code);
                }
                println!(
                    "nv-inject: process {} exited with code {}",
                    pi.process_id, code as i32
                );
                exit = if code == 0 { 0 } else { 3 };
            }
            // SAFETY: handle from CreateProcessW.
            unsafe { win::CloseHandle(pi.process) };
            exit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_text(text: &str) -> Result<Options, String> {
        parse(text.split_whitespace().map(String::from))
    }

    #[test]
    fn options_before_launch_are_nv_injects() {
        let o = parse_text("--dll a.dll --wait --launch game.exe -windowed 1").unwrap();
        assert_eq!(o.dll, PathBuf::from("a.dll"));
        match o.target {
            Target::Launch { exe, args, wait } => {
                assert_eq!(exe, "game.exe");
                assert_eq!(args, ["-windowed", "1"]);
                assert!(wait);
            }
            Target::Pid(_) => panic!("expected a launch"),
        }
    }

    #[test]
    fn everything_after_the_program_belongs_to_the_program() {
        let o = parse_text("--dll a.dll --launch game.exe loop 2 --wait").unwrap();
        match o.target {
            Target::Launch { args, wait, .. } => {
                assert_eq!(args, ["loop", "2", "--wait"]);
                assert!(!wait, "a --wait after the program must not turn waiting on");
            }
            Target::Pid(_) => panic!("expected a launch"),
        }
    }

    #[test]
    fn pid_and_errors() {
        let o = parse_text("--dll a.dll --pid 42").unwrap();
        assert!(matches!(o.target, Target::Pid(42)));
        assert!(parse_text("--dll a.dll").is_err());
        assert!(parse_text("--dll a.dll --pid x").is_err());
        assert!(parse_text("--dll a.dll --launch").is_err());
        assert!(parse_text("--bogus --pid 1").is_err());
    }

    #[test]
    fn contradictory_options_are_errors() {
        // --wait has nothing to wait for with --pid, in either order.
        for text in ["--wait --pid 42", "--pid 42 --wait"] {
            let e = parse_text(text).err().expect(text);
            assert!(e.contains("--wait"), "{text}: {e}");
        }
        // Only one target.
        for text in ["--pid 42 --launch game.exe", "--pid 1 --pid 2"] {
            let e = parse_text(text).err().expect(text);
            assert!(e.contains("only one"), "{text}: {e}");
        }
        // After --launch everything belongs to the program: this is a launch
        // of game.exe with two arguments, not a second target.
        match parse_text("--launch game.exe --pid 42").unwrap().target {
            Target::Launch { args, .. } => assert_eq!(args, ["--pid", "42"]),
            Target::Pid(_) => panic!("expected a launch"),
        }
        // A wait on a launch is fine.
        assert!(parse_text("--wait --launch game.exe").is_ok());
    }
}
