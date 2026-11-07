//! The tables the game's code answers Windows' questions from (docs/adr/0008): made on this PC by
//! the tools in `app\tools`, kept in `userdata\machine`.
//!
//! - The adapter table (graphics card and driver, displays, metrics, time zone, code pages, folders):
//!   its tools take about a second, so it is made again at every start and can't go stale. When a
//!   tool fails, the last good table is used and the log says so.
//! - The sound table takes about 20 s, so it is made again only when Windows' build or the default
//!   sound device changed.
use crate::win;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// `CREATE_NO_WINDOW`: a console tool without a console window.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// How long a graphics tool may take (about a second here) and the sound tool (about 19 s here).
const GRAPHICS_TOOL_LIMIT: Duration = Duration::from_secs(60);
const SOUND_TOOL_LIMIT: Duration = Duration::from_secs(240);

/// A tool's standard output, or why not. A tool that hangs (a driver can) is ended after `limit`,
/// so the player gets a message instead of a window that never goes.
fn run_tool(
    tool: &Path,
    args: &[&std::ffi::OsStr],
    log: &Path,
    limit: Duration,
) -> Result<Vec<u8>, String> {
    let name = tool.file_name().unwrap_or_default().to_string_lossy();
    let stderr = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log)
        .map(Stdio::from)
        .unwrap_or_else(|_| Stdio::null());
    let mut child = Command::new(tool)
        .args(args)
        .current_dir(tool.parent().unwrap_or(Path::new(".")))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(stderr)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("{} could not start: {e}", tool.display()))?;
    let mut pipe = child.stdout.take().ok_or("no output pipe")?;
    let reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = pipe.read_to_end(&mut bytes);
        bytes
    });
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if start.elapsed() >= limit => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "{name} did not finish in {} seconds",
                    limit.as_secs()
                ));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(e) => return Err(format!("{name}: {e}")),
        }
    };
    let bytes = reader.join().unwrap_or_default();
    if status.success() {
        Ok(bytes)
    } else {
        Err(format!("{name} ended with status {:?}", status.code()))
    }
}

/// Writes through a temporary name, so a cut-off write never leaves half a table.
pub fn write_whole(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(".part");
    std::fs::write(&temporary, bytes)?;
    std::fs::rename(&temporary, path)
}

/// Why an adapter table can't be used: every line kind the game's start-up stops without.
pub fn adapter_table_problem(table: &str) -> Option<String> {
    for kind in [
        "GetACP",
        "GetOEMCP",
        "CpuIdFlagTest",
        "GetAdapterIdentifier",
        "GetAdapterDisplayMode",
        "GetDeviceCaps",
        "GetWindowsDirectoryA",
        "CreateQuery",
    ] {
        if !table.lines().any(|l| l.split('\t').next() == Some(kind)) {
            return Some(format!("it has no {kind} answer"));
        }
    }
    let hal = table
        .lines()
        .find(|l| l.starts_with("GetDeviceCaps\t0 1\t"))
        .and_then(|l| l.split('\t').nth(2));
    if hal != Some("0") {
        return Some("the graphics card offers no hardware Direct3D 9".into());
    }
    None
}

/// The first adapter's name in the table (`D3DADAPTER_IDENTIFIER9`'s `Description`, after the 512
/// bytes of `Driver`), one character per byte, as the game compares it with `sD3DDevice`.
pub fn adapter_name(table: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(table);
    let hex = text
        .lines()
        .find_map(|l| l.strip_prefix("GetAdapterIdentifier\t0 0\t0\t"))?;
    let digit = |i: usize| u8::from_str_radix(hex.get(i..i + 2)?, 16).ok();
    let name: String = (512..1024)
        .map_while(|at| digit(2 * at).filter(|&b| b != 0))
        .map(|b| b as char)
        .collect();
    (!name.is_empty()).then_some(name)
}

/// The whole adapter table: `d3d9-capture`, then the `nvapi.dll` answers, then which query types
/// the device takes (only its `CreateQuery` lines).
pub fn make_adapter_table(tools: &Path, game_exe: &Path, log: &Path) -> Result<Vec<u8>, String> {
    let d3d9 = tools.join("d3d9-capture.exe");
    let mut table = run_tool(&d3d9, &[], log, GRAPHICS_TOOL_LIMIT)?;
    table.extend(run_tool(
        &tools.join("nvapi-capture.exe"),
        &[game_exe.as_os_str()],
        log,
        GRAPHICS_TOOL_LIMIT,
    )?);
    let queries = run_tool(
        &d3d9,
        &[
            std::ffi::OsStr::new("device"),
            std::ffi::OsStr::new("queries"),
        ],
        log,
        GRAPHICS_TOOL_LIMIT,
    )?;
    for line in String::from_utf8_lossy(&queries).lines() {
        if line.starts_with("CreateQuery\t") {
            table.extend_from_slice(line.as_bytes());
            table.push(b'\n');
        }
    }
    let text = String::from_utf8_lossy(&table);
    match adapter_table_problem(&text) {
        Some(why) => Err(why),
        None => Ok(table),
    }
}

/// The table with the game's Documents and its two AppData folders moved (`SHGetFolderPathA` for
/// `CSIDL_PERSONAL` 5, `CSIDL_APPDATA` 1a and `CSIDL_LOCAL_APPDATA` 1c): those lines get these
/// bytes as hex, every other byte stays. An error when one of them isn't there exactly once.
pub fn with_folders(
    table: &[u8],
    documents: &[u8],
    roaming: &[u8],
    local: &[u8],
) -> Result<Vec<u8>, String> {
    let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
    let wanted = [
        ("5", hex(documents)),
        ("1a", hex(roaming)),
        ("1c", hex(local)),
    ];
    let mut seen = [0; 3];
    let mut out = Vec::with_capacity(table.len());
    for line in table.split_inclusive(|&b| b == b'\n') {
        let text = String::from_utf8_lossy(line);
        let mut parts = text.trim_end_matches(['\r', '\n']).split('\t');
        let mut replaced = false;
        if parts.next() == Some("SHGetFolderPathA") {
            if let Some(key) = parts.next().and_then(|w| w.strip_suffix(" 0")) {
                if let Some(i) = wanted.iter().position(|(k, _)| *k == key) {
                    out.extend_from_slice(
                        format!("SHGetFolderPathA\t{key} 0\t0\t{}\n", wanted[i].1).as_bytes(),
                    );
                    seen[i] += 1;
                    replaced = true;
                }
            }
        }
        if !replaced {
            out.extend_from_slice(line);
        }
    }
    if seen != [1, 1, 1] {
        return Err("the table's folder answers are not all there".into());
    }
    Ok(out)
}

/// What decides whether the sound table is still right.
pub fn sound_fingerprint() -> String {
    format!(
        "windows {}\ndevice {}\n",
        win::windows_build(),
        win::default_sound_device().unwrap_or_else(|| "none".into())
    )
}

/// The sound table's state after [`make_sound_table`].
#[derive(Debug, PartialEq)]
pub enum Sound {
    Ready,
    /// DirectSound has no device here: the game runs without sound.
    NoDevice,
}

/// What a sound table says about the device.
pub fn sound_state(table: &str) -> Result<Sound, String> {
    if !table.lines().any(|l| l == "DsEnd") {
        return Err("it was cut short".into());
    }
    let create = table
        .lines()
        .find(|l| l.starts_with("DsCreate\t"))
        .and_then(|l| l.split('\t').nth(2))
        .ok_or("it has no DsCreate answer")?;
    Ok(if create == "0" {
        Sound::Ready
    } else {
        Sound::NoDevice
    })
}

/// The sound table at `table`, made again when `fingerprint` differs from the one kept beside it.
/// `Ok(true)` when it was made now.
pub fn make_sound_table(
    tools: &Path,
    table: &Path,
    stamp: &Path,
    fingerprint: &str,
    log: &Path,
) -> Result<bool, String> {
    let same = std::fs::read_to_string(stamp).is_ok_and(|kept| kept == fingerprint);
    if same && table.is_file() {
        return Ok(false);
    }
    let bytes = run_tool(
        &tools.join("dsound-capture.exe"),
        &[],
        log,
        SOUND_TOOL_LIMIT,
    )?;
    sound_state(&String::from_utf8_lossy(&bytes))?;
    write_whole(table, &bytes).map_err(|e| format!("{}: {e}", table.display()))?;
    write_whole(stamp, fingerprint.as_bytes()).map_err(|e| format!("{}: {e}", stamp.display()))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folders_are_replaced_and_the_rest_is_kept() {
        let table = b"GetACP\t\t4e4\nSHGetFolderPathA\t5 0\t0\t4344\nSHGetFolderPathA\t1a 0\t0\t41\nSHGetFolderPathA\t1c 0\t80070002\t\nSHGetFolderPathA\t23 0\t0\t5a\nlast";
        let out = with_folders(table, b"D:\\d", b"R", b"\xc3\xa9").unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "GetACP\t\t4e4\nSHGetFolderPathA\t5 0\t0\t443a5c64\nSHGetFolderPathA\t1a 0\t0\t52\nSHGetFolderPathA\t1c 0\t0\tc3a9\nSHGetFolderPathA\t23 0\t0\t5a\nlast"
        );
        assert!(with_folders(b"GetACP\t\t4e4\n", b"a", b"b", b"c").is_err());
    }

    #[test]
    fn the_adapters_name_is_read() {
        let mut bytes = vec![0u8; 1100];
        bytes[0..4].copy_from_slice(b"nvld");
        bytes[512..516].copy_from_slice(b"Card");
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let table = format!(
            "x\t\t0\nGetAdapterIdentifier\t0 0\t0\t{hex}\nGetAdapterIdentifier\t0 2\t0\t00\n"
        );
        assert_eq!(adapter_name(table.as_bytes()).unwrap(), "Card");
        assert_eq!(adapter_name(b"GetACP\t\t4e4\n"), None);
    }

    #[test]
    fn an_adapter_table_without_hardware_direct3d_is_refused() {
        let mut table = String::from(
            "GetACP\t\t4e4\nGetOEMCP\t\t1b5\nCpuIdFlagTest\t\t200000\nGetAdapterIdentifier\t0 0\t0\t00\nGetAdapterDisplayMode\t0\t0\t00\nGetWindowsDirectoryA\t\t3\t00\nCreateQuery\t0\t0\n",
        );
        assert!(adapter_table_problem(&table)
            .unwrap()
            .contains("GetDeviceCaps"));
        table.push_str("GetDeviceCaps\t0 1\t8876086a\t00\n");
        assert!(adapter_table_problem(&table).unwrap().contains("hardware"));
        table = table.replace("GetDeviceCaps\t0 1\t8876086a", "GetDeviceCaps\t0 1\t0");
        assert_eq!(adapter_table_problem(&table), None);
    }

    #[test]
    fn a_sound_table_says_whether_there_is_a_device() {
        assert_eq!(sound_state("DsCreate\t\t0\t\nDsEnd\n"), Ok(Sound::Ready));
        assert_eq!(
            sound_state("DsCreate\t\t88780078\t\nDsEnd\n"),
            Ok(Sound::NoDevice)
        );
        assert!(sound_state("DsCreate\t\t0\t\n").is_err());
        assert!(sound_state("DsEnd\n").is_err());
    }
}
