//! Play.exe, the preview package's launcher: finds the player's game (Steam, GOG and Epic, read
//! only, or it asks), checks it is 1.4.0.525, makes this PC's tables, keeps every file the game
//! writes in the package's own folder and starts the game, saying plainly in a window what went
//! wrong when something does. Nothing is ever written outside the package's folder.
//!
//!   Play.exe                  plays
//!   Play.exe --mods           plays with the mods in the `mods` folder
//!   Play.exe --import-saves   copies the saves from Documents\My Games\FalloutNV (read only)
//!   Play.exe --check          everything up to starting the game, then says what it found
//!   --game FOLDER             the game's folder, instead of looking for it
//!   --from FOLDER             with --import-saves: where to copy saves from
//!
//! Scripted tests set `NV_RS_NO_WINDOWS=1` (no window shown; every message is in
//! `userdata\logs\launcher.log` either way), `NV_RS_REAL_DOCUMENTS=DIR` (where the player's
//! Documents are read from), `NV_RS_NO_SHORT_PATHS=1`, `NV_RS_NO_STORES=1` (look in no store) and `NV_RS_SYSTEM_FOLDER=DIR`.
#![windows_subsystem = "windows"]

mod find;
mod mods;
mod settings;
mod tables;
mod ui;
mod win;

use settings::Settings;
use std::cell::{Cell, RefCell};
use std::io::{Read, Write};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// Why the launcher stops: what the player reads, and the launcher's exit code.
#[derive(Debug)]
struct Stop {
    code: i32,
    message: String,
}

fn stop<T>(code: i32, message: impl Into<String>) -> Result<T, Stop> {
    Err(Stop {
        code,
        message: message.into(),
    })
}

const STOP_LOCATION: i32 = 2;
const STOP_NO_GAME: i32 = 3;
const STOP_WRONG_GAME: i32 = 4;
const STOP_DIRECTX: i32 = 5;
const STOP_TABLES: i32 = 6;
const STOP_GAME_STOPPED: i32 = 7;
const STOP_MODS: i32 = 8;
const STOP_BLOCKED: i32 = 9;
const STOP_HUNG: i32 = 10;
const STOP_SETTINGS: i32 = 11;

/// How long the game program's `--check` may take (about a second here, a little longer with a
/// slow sound device query).
const CHECK_LIMIT: Duration = Duration::from_secs(60);

const ISSUES: &str = "https://github.com/slaterain/nv-rs/issues";
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// The package's folders.
struct Package {
    root: PathBuf,
    game_exe: PathBuf,
    tools: PathBuf,
    userdata: PathBuf,
    machine: PathBuf,
    logs: PathBuf,
    reports: PathBuf,
    settings: PathBuf,
    remembered: PathBuf,
    /// Log lines kept in memory until the package's place is checked ([`clear_for_writing`]): a
    /// refusal leaves the disk untouched.
    held: RefCell<Vec<String>>,
    writing: Cell<bool>,
}

impl Package {
    fn at(root: PathBuf) -> Package {
        let userdata = root.join("userdata");
        Package {
            game_exe: root.join("app\\FalloutNV.exe"),
            tools: root.join("app\\tools"),
            machine: userdata.join("machine"),
            logs: userdata.join("logs"),
            reports: userdata.join("reports"),
            settings: userdata.join("settings.ini"),
            remembered: userdata.join("game-folder.txt"),
            held: RefCell::new(Vec::new()),
            writing: Cell::new(false),
            userdata,
            root,
        }
    }

    fn log(&self, text: &str) {
        let (when, _) = win::now();
        let line = format!("{when} {text}");
        if self.writing.get() {
            self.append(&line);
        } else {
            self.held.borrow_mut().push(line);
        }
    }

    fn append(&self, line: &str) {
        if std::fs::create_dir_all(&self.logs).is_ok() {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.logs.join("launcher.log"))
            {
                let _ = writeln!(file, "{line}");
            }
        }
    }

    /// From here on the log goes to its file, with what was kept so far first.
    fn start_writing(&self) {
        self.writing.set(true);
        let held = std::mem::take(&mut *self.held.borrow_mut());
        for line in held {
            self.append(&line);
        }
    }
}

/// The game's own folders in the package, one set for each way of playing.
struct Profile {
    documents: PathBuf,
    roaming: PathBuf,
    local: PathBuf,

    saves: PathBuf,
    prefs: PathBuf,
    table: PathBuf,
}

impl Profile {
    fn at(userdata: &Path, name: &str) -> Profile {
        let base = userdata.join(name);
        let documents = base.join("Documents");
        let game_documents = documents.join("My Games\\FalloutNV");
        Profile {
            roaming: base.join("AppData\\Roaming"),
            local: base.join("AppData\\Local"),
            saves: game_documents.join("Saves"),
            prefs: game_documents.join("FalloutPrefs.ini"),
            table: base.join("adapter.tsv"),

            documents,
        }
    }

    fn make(&self) -> std::io::Result<()> {
        for dir in [&self.saves, &self.roaming, &self.local.join("FalloutNV")] {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

/// `\\?\` taken off the front of what Windows answers for a real path.
fn without_verbatim(path: PathBuf) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(rest) = text.strip_prefix("\\\\?\\UNC\\") {
        PathBuf::from(format!("\\\\{rest}"))
    } else if let Some(rest) = text.strip_prefix("\\\\?\\") {
        PathBuf::from(rest)
    } else {
        path
    }
}

/// The path Windows really means: junctions, symbolic links, `subst` and mapped drives and 8.3
/// short names resolved. A part that doesn't exist (yet) is kept as it is, after the real path of
/// the longest part that does.
fn real_path(path: &Path) -> PathBuf {
    let mut rest = Vec::new();
    let mut at = path.to_path_buf();
    loop {
        if let Ok(real) = std::fs::canonicalize(&at) {
            let mut out = without_verbatim(real);
            for part in rest.iter().rev() {
                out.push(part);
            }
            return out;
        }
        match (at.file_name().map(|n| n.to_owned()), at.parent()) {
            (Some(name), Some(parent)) => {
                rest.push(name);
                at = parent.to_path_buf();
            }
            _ => return path.to_path_buf(),
        }
    }
}

/// Lower case, `\` only, no trailing `\`, links and short names resolved ([`real_path`]): for
/// comparing paths.
fn folder_key(path: &Path) -> String {
    real_path(path)
        .to_string_lossy()
        .replace('/', "\\")
        .trim_end_matches('\\')
        .to_lowercase()
}

/// Whether `inner` is `outer` or lies inside it (both [`folder_key`]s).
fn is_inside(inner: &str, outer: &str) -> bool {
    inner == outer || inner.starts_with(&format!("{outer}\\"))
}

/// The longest name the game makes under Documents (a save, up to 60 characters) must fit the
/// 259 characters its paths have. An error text when `documents`' path is too long.
fn path_too_long(documents_bytes: usize) -> bool {
    documents_bytes + "\\My Games\\FalloutNV\\Saves\\".len() + 60 >= 260
}

/// Where the player's real Documents are (only read).
fn real_documents() -> Option<PathBuf> {
    std::env::var_os("NV_RS_REAL_DOCUMENTS")
        .map(PathBuf::from)
        .or_else(|| win::known_folder(5))
}

/// The player's folders under `profile` (`%USERPROFILE%`) by their text, wherever Windows says
/// Documents is: Documents, OneDrive's Documents and AppData. The game refuses to keep saves there
/// (host `save_out::refused`), so the launcher refuses the same places. The rest of OneDrive is
/// allowed: a Desktop kept there is where many people unzip things.
fn profile_folders(profile: &Path) -> [(PathBuf, &'static str); 3] {
    [
        (profile.join("Documents"), "Documents"),
        (profile.join("OneDrive\\Documents"), "Documents"),
        (profile.join("AppData"), "AppData"),
    ]
}

/// Which of the player's `folders` the package's folder `root` is in or holds, by the real paths.
fn player_folder_of(root: &Path, folders: &[(PathBuf, &'static str)]) -> Option<&'static str> {
    let root = folder_key(root);
    folders.iter().find_map(|(folder, what)| {
        let key = folder_key(folder);
        (is_inside(&root, &key) || is_inside(&key, &root)).then_some(*what)
    })
}

/// Why the package can't run from where it is.
fn check_location(package: &Package) -> Result<(), Stop> {
    let root = folder_key(&package.root);
    let move_it = "Move the whole nv-rs folder somewhere else, for example C:\\Games\\nv-rs, and start Play.exe again.";
    let temporary = std::env::var_os("TEMP").map(|t| folder_key(Path::new(&t)));
    if temporary.is_some_and(|t| is_inside(&root, &t)) {
        return stop(
            STOP_LOCATION,
            format!(
                "nv-rs is running from a temporary folder, which usually means from inside the ZIP.\n\nExtract the whole ZIP to a folder first (for example C:\\Games\\nv-rs), then start Play.exe there.\n\nThis folder was:\n{}",
                package.root.display()
            ),
        );
    }
    if !package.game_exe.is_file() || !package.tools.join("d3d9-capture.exe").is_file() {
        return stop(
            STOP_BLOCKED,
            format!(
                "A part of nv-rs is missing here: {}\n\nExtract the whole ZIP again. If it is still missing, your antivirus may have removed it: the program is new and not signed (see READ ME.txt).",
                if package.game_exe.is_file() { "app\\tools" } else { "app\\FalloutNV.exe" }
            ),
        );
    }
    let mut folders: Vec<(PathBuf, &str)> = [
        (real_documents(), "Documents"),
        (win::known_folder(0x1a), "AppData"),
        (win::known_folder(0x1c), "AppData"),
    ]
    .into_iter()
    .filter_map(|(folder, what)| Some((folder?, what)))
    .collect();
    if let Some(profile) = std::env::var_os("USERPROFILE") {
        folders.extend(profile_folders(Path::new(&profile)));
    }
    if let Some(what) = player_folder_of(&package.root, &folders) {
        return stop(
            STOP_LOCATION,
            format!(
                "nv-rs is inside your {what} folder, and it keeps away from the game's own files there.\n\n{move_it}\n\nThis folder was:\n{}",
                package.root.display()
            ),
        );
    }
    // The game's own paths (and the tools' start folder) must fit, before anything runs here.
    let documents = package.userdata.join("game\\Documents");
    let length = guest_bytes(&package.root).map_or(package.root.as_os_str().len(), |bytes| {
        bytes.len() + documents.as_os_str().len() - package.root.as_os_str().len()
    });
    if path_too_long(length) {
        return stop(
            STOP_LOCATION,
            format!(
                "This folder's path is too long for the game:\n{}\n\nMove the whole nv-rs folder to a shorter path, for example C:\\Games\\nv-rs.",
                package.root.display()
            ),
        );
    }
    Ok(())
}

/// Whether the package's folder is the game's folder or is inside it or the other way round, by
/// the real paths.
fn inside_game(root: &Path, game: &Path) -> bool {
    let (root, game) = (folder_key(root), folder_key(game));
    is_inside(&root, &game) || is_inside(&game, &root)
}

/// Every check of where the package is that needs the game's folder, then the first write: the
/// folder test and the log. Nothing is written before this passes, and a refusal leaves the disk
/// as it was. `game` is the folder found; with `None` (nothing found, or only saves to import) the
/// folders the game could be in (the last one used, Steam, GOG and Epic) are compared instead.
fn clear_for_writing(package: &Package, game: Option<&Path>) -> Result<(), Stop> {
    let move_it = "Move the whole nv-rs folder somewhere else, for example C:\\Games\\nv-rs, and start Play.exe again.";
    let mut folders: Vec<PathBuf> = Vec::new();
    match game {
        Some(game) => folders.push(game.to_path_buf()),
        None => {
            if let Ok(text) = std::fs::read_to_string(&package.remembered) {
                folders.push(PathBuf::from(text.trim()));
            }
            folders.extend(find::candidates().into_iter().map(|(_, place)| place));
        }
    }
    for folder in &folders {
        if inside_game(&package.root, folder) {
            return stop(
                STOP_LOCATION,
                format!(
                    "nv-rs is inside your game's folder. It must never write there, so it won't run from there.\n\n{move_it}\n\nThis folder was:\n{}",
                    package.root.display()
                ),
            );
        }
    }
    // Writable: not under Program Files without rights, not on a read-only drive.
    let test = package.root.join(".nv-rs-write-test");
    let written = std::fs::create_dir_all(&package.userdata)
        .and_then(|()| std::fs::write(&test, b"x"))
        .and_then(|()| std::fs::remove_file(&test));
    if let Err(e) = written {
        return stop(
            STOP_LOCATION,
            format!(
                "nv-rs can't write in its own folder, where it keeps your saves and settings ({e}).\n\nIt doesn't need administrator rights and shouldn't be given them. {move_it}\n\nThis folder was:\n{}",
                package.root.display()
            ),
        );
    }
    package.start_writing();
    Ok(())
}

/// The game's folder and `Data` folder.
#[derive(Clone)]
struct Game {
    folder: PathBuf,
    data: PathBuf,
}

/// What a program printed, or why it didn't finish.
enum Limited {
    Done(Output),
    TimedOut,
    Failed(std::io::Error),
}

/// Runs `command` to its end, but no longer than `limit`: a program that hangs (a sound or graphics
/// driver can) is ended, so the player gets a message instead of a window that never goes.
fn run_limited(command: &mut Command, limit: Duration) -> Limited {
    let mut child = match command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return Limited::Failed(e),
    };
    fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> std::thread::JoinHandle<Vec<u8>> {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            bytes
        })
    }
    let (out, err) = (drain(child.stdout.take()), drain(child.stderr.take()));
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if start.elapsed() >= limit => {
                let _ = child.kill();
                let _ = child.wait();
                return Limited::TimedOut;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(25)),
            Err(e) => return Limited::Failed(e),
        }
    };
    Limited::Done(Output {
        status,
        stdout: out.join().unwrap_or_default(),
        stderr: err.join().unwrap_or_default(),
    })
}

/// What the player reads when the game's program can't be opened for reading (a lock or no
/// access), or `None` when it can.
fn exe_read_problem(exe: &Path) -> Option<String> {
    let e = std::fs::File::open(exe).err()?;
    let why = match e.raw_os_error() {
        Some(32 | 33) => "Another program has it open: the game or its launcher still running, or an antivirus scanning it. Close them, wait a moment and start Play.exe again.",
        Some(5) => "Windows does not let this account read it. The folder's permissions may be restrictive, or an antivirus or Windows' \"controlled folder access\" blocks it. Allow Play.exe, or give your account read access to the game's folder.",
        _ => "Windows can't read it.",
    };
    Some(format!(
        "nv-rs can't read the game's program, FalloutNV.exe ({e}):\n{}\n\n{why}",
        exe.display()
    ))
}

/// What `app\FalloutNV.exe --check` says about this copy of the game: its lines, or why it can't run.
fn check_game(package: &Package, game: &Game) -> Result<Vec<String>, Stop> {
    let output = run_limited(
        Command::new(&package.game_exe)
            .arg("--check")
            .arg(&game.data),
        CHECK_LIMIT,
    );
    let output = match output {
        Limited::Done(o) => o,
        Limited::TimedOut => {
            package.log("--check: did not finish");
            return stop(
                STOP_HUNG,
                format!(
                    "nv-rs's check of your game did not finish in {} seconds, so it was ended. This usually means the sound or graphics driver is stuck answering.\n\nRestart the PC and try again. If it happens again, set the sound output to another device (or stereo) in Windows' sound settings, and report it at {ISSUES}.",
                    CHECK_LIMIT.as_secs()
                ),
            );
        }
        Limited::Failed(e) => {
            return stop(
                STOP_BLOCKED,
                format!(
                    "Windows or your antivirus did not let nv-rs start its program ({e}).\n\nThe program is new and not signed, so this can happen: see READ ME.txt. Program:\n{}",
                    package.game_exe.display()
                ),
            )
        }
    };
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::to_owned)
            .collect());
    }
    let details = String::from_utf8_lossy(&output.stderr).trim().to_string();
    package.log(&format!("--check: {details}"));
    let exe = game.folder.join("FalloutNV.exe");
    if !exe.is_file() {
        return stop(
            STOP_WRONG_GAME,
            format!(
                "There is no FalloutNV.exe in this game folder:\n{}\n\nnv-rs needs the installed game, version 1.4.0.525.",
                game.folder.display()
            ),
        );
    }
    if let Some(message) = exe_read_problem(&exe) {
        return stop(STOP_BLOCKED, message);
    }
    if details.contains("error: ") && !details.contains("1.4.0.525") {
        return stop(
            STOP_BLOCKED,
            format!(
                "nv-rs's own program did not run ({details}).\n\nThe program is new and not signed, so Windows or an antivirus can block it: see READ ME.txt.\n\nPlease report this at {ISSUES}"
            ),
        );
    }
    let version = win::file_version(&exe);
    let found = version.as_deref().unwrap_or("unknown");
    if version.as_deref() == Some("1.4.0.525") {
        return stop(
            STOP_WRONG_GAME,
            format!(
                "Your game says it is version 1.4.0.525, but its program isn't the build nv-rs was made from (the Steam one). Some GOG, Epic and regional copies, and programs changed by patch tools, differ.\n\nnv-rs can't run on this copy yet, because every address it uses belongs to that one build. Please tell us which edition you have: {ISSUES}\n\nGame folder:\n{}",
                game.folder.display()
            ),
        );
    }
    stop(
        STOP_WRONG_GAME,
        format!(
            "Your game is version {found}, and nv-rs needs version 1.4.0.525 (the last update, with all patches installed).\n\nUpdate the game through the store you bought it from, then start Play.exe again.\n\nGame folder:\n{}",
            game.folder.display()
        ),
    )
}

/// A game whose program passes the check, from `--game`, the remembered folder, the stores or the
/// player. Nothing there is written.
fn find_game(package: &Package, explicit: Option<&Path>) -> Result<(Game, Vec<String>), Stop> {
    let game_at = |folder: &Path| -> Option<Game> {
        let data = find::data_folder(folder)?;
        let folder = data.parent()?.to_path_buf();
        Some(Game { folder, data })
    };
    if let Some(path) = explicit {
        let Some(game) = game_at(path) else {
            return stop(
                STOP_NO_GAME,
                format!(
                    "This folder has no FalloutNV.esm, in itself or in a Data folder inside it:\n{}",
                    path.display()
                ),
            );
        };
        let lines = check_game(package, &game)?;
        return Ok((game, lines));
    }
    let mut places: Vec<(&str, PathBuf)> = Vec::new();
    if let Ok(text) = std::fs::read_to_string(&package.remembered) {
        places.push(("the last time", PathBuf::from(text.trim())));
    }
    places.extend(find::candidates());
    let mut problem: Option<Stop> = None;
    for (from, place) in &places {
        let Some(game) = game_at(place) else {
            continue;
        };
        package.log(&format!(
            "game found through {from}: {}",
            game.folder.display()
        ));
        match check_game(package, &game) {
            Ok(lines) => return Ok((game, lines)),
            Err(why) => {
                package.log(&why.message);
                problem.get_or_insert(why);
            }
        }
    }
    let first = match problem {
        Some(why) if why.code == STOP_BLOCKED || why.code == STOP_HUNG => return Err(why),
        Some(why) => {
            ui::error(&why.message);
            Some(why)
        }
        None => {
            ui::info("nv-rs could not find Fallout: New Vegas on this PC. It looks in Steam, GOG and Epic.\n\nIn the next window, choose the game's folder: the one with FalloutNV.exe and a Data folder in it.");
            None
        }
    };
    loop {
        let Some(picked) = ui::pick_folder(
            "Choose your Fallout: New Vegas folder (with FalloutNV.exe and Data in it)",
        ) else {
            return Err(first.unwrap_or(Stop {
                code: STOP_NO_GAME,
                message: "nv-rs can't start without the game. It needs your own installed copy of Fallout: New Vegas, version 1.4.0.525: Steam, GOG or Epic.".into(),
            }));
        };
        let Some(game) = game_at(&picked) else {
            ui::error(&format!(
                "That folder has no FalloutNV.esm, in itself or in a Data folder inside it:\n{}\n\nChoose the folder Fallout: New Vegas is installed in.",
                picked.display()
            ));
            continue;
        };
        match check_game(package, &game) {
            Ok(lines) => return Ok((game, lines)),
            Err(why) if why.code == STOP_BLOCKED || why.code == STOP_HUNG => return Err(why),
            Err(why) => {
                package.log(&why.message);
                ui::error(&why.message);
            }
        }
    }
}

/// The bytes the game's code gets for one of the package's folders. Its paths go back to Windows
/// as UTF-8 (`engine-platform` files.rs), and it reads them one byte at a time under a one-byte
/// code page. So: plain letters as they are; else the 8.3 name, when Windows has a plain one; else
/// UTF-8 on a one-byte code page. On a Chinese, Japanese or Korean code page the game's C runtime
/// would read UTF-8 as double-byte text, which can swallow a `\`: `None`.
fn guest_bytes(path: &Path) -> Option<Vec<u8>> {
    let text = path.to_str()?;
    if text.is_ascii() {
        return Some(text.as_bytes().to_vec());
    }
    if std::env::var_os("NV_RS_NO_SHORT_PATHS").is_none() {
        if let Some(short) = win::short_path(path) {
            if let Some(short) = short.to_str().filter(|s| s.is_ascii()) {
                return Some(short.as_bytes().to_vec());
            }
        }
    }
    if matches!(win::ansi_code_page(), 932 | 936 | 949 | 950 | 1361 | 54936) {
        return None;
    }
    Some(text.as_bytes().to_vec())
}

/// `FalloutPrefs.ini` in the profile: the player's own copied once, read only (the settings the
/// game's launcher made); then the choices from Settings.cmd over it.
///
/// On a PC where the game's own launcher has never run, this does what it leaves behind: `Fallout.ini`
/// from the game's `Fallout_default.ini` and `FalloutPrefs.ini` from its High preset (`high.ini`), both
/// only read from the game's folder. `sD3DDevice` always names this PC's card: the game hands over to
/// its launcher when it doesn't (`004da570`), and here there is no launcher to hand over to.
fn prepare_prefs(
    package: &Package,
    profile: &Profile,
    settings: &Settings,
    game_folder: &Path,
    device: Option<&str>,
) -> std::io::Result<()> {
    let real = real_documents().map(|d| d.join("My Games\\FalloutNV"));
    let read_real = |name: &str| real.as_ref().and_then(|d| std::fs::read(d.join(name)).ok());
    let ini = profile.prefs.with_file_name("Fallout.ini");
    if !ini.is_file() {
        let bytes = read_real("Fallout.ini")
            .or_else(|| std::fs::read(game_folder.join("Fallout_default.ini")).ok());
        if let Some(bytes) = bytes {
            tables::write_whole(&ini, &bytes)?;
        }
    }
    let mut first_time_size = None;
    if !profile.prefs.is_file() {
        if let Some(bytes) = read_real("FalloutPrefs.ini") {
            tables::write_whole(&profile.prefs, &bytes)?;
            package.log("copied the player's FalloutPrefs.ini (read only) for the first start");
        } else {
            // The game's window starts at the desktop's size.
            first_time_size = Some(ui::desktop_size());
            let preset = std::fs::read(game_folder.join("high.ini")).unwrap_or_default();
            tables::write_whole(&profile.prefs, &preset)?;
            package.log("no FalloutPrefs.ini to copy: starting from the game's High preset");
        }
    }
    let mut lines = settings.prefs_lines();
    if let Some(device) = device {
        lines.push(("sD3DDevice".into(), format!("\"{device}\"")));
    }
    if let Some((w, h)) = first_time_size {
        if !lines.iter().any(|(n, _)| n == "iSize W") {
            lines.push(("iSize W".into(), w.to_string()));
            lines.push(("iSize H".into(), h.to_string()));
        }
    }
    if lines.is_empty() {
        return Ok(());
    }
    // One character per byte, so the rest of the file comes back unchanged.
    let old: Vec<u8> = std::fs::read(&profile.prefs).unwrap_or_default();
    let text: String = old.iter().map(|&b| b as char).collect();
    let new = settings::apply_to_prefs(&text, &lines);
    let bytes: Vec<u8> = new.chars().map(|c| c as u32 as u8).collect();
    if bytes != old {
        tables::write_whole(&profile.prefs, &bytes)?;
    }
    Ok(())
}

/// The newest logs stay; older game logs go (only `game-*.log` in the package's own folder).
fn tidy_logs(logs: &Path) {
    let Ok(entries) = std::fs::read_dir(logs) else {
        return;
    };
    let mut names: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("game-") && n.ends_with(".log"))
        })
        .collect();
    names.sort();
    for old in names.iter().rev().skip(10) {
        let _ = std::fs::remove_file(old);
    }
}

/// What the player reads when the game's run ended badly, from the log's end.
fn plain_end(tail: &str, status: Option<i32>) -> String {
    let lines: Vec<&str> = tail.lines().filter(|l| !l.trim().is_empty()).collect();
    let last = lines
        .iter()
        .rev()
        .find(|l| {
            l.contains("Game's main stopped") || l.contains("panicked") || l.starts_with("error:")
        })
        .or(lines.last())
        .copied()
        .unwrap_or("(the log is empty)");
    let has = |what: &str| tail.contains(what);
    let what = if has("the sound table disagrees") || has("the sound table has") {
        "Sound on this PC answers differently from what nv-rs knows, and it stops rather than guess. Changing the sound output device, or setting it to stereo, may get past this."
    } else if has("d3dx9_38.dll") {
        "DirectX 9's D3DX library (d3dx9_38.dll) was missing or did something unexpected. Installing Microsoft's \"DirectX End-User Runtimes (June 2010)\" may help."
    } else if has("was not captured in the adapter table")
        || has("is not in the machine table")
        || has("adapter table")
    {
        "The game asked your graphics card something this PC's table has no answer for, so it stopped. That is a gap in nv-rs, and your report is what fills it."
    } else if has("CreateDevice answered") {
        "Your graphics card refused the game's display settings. Try Settings.cmd: Window, a lower resolution, anti-aliasing off."
    } else if has("the game's window") && has("not provided") {
        "The game asked Windows for something nv-rs doesn't provide yet."
    } else {
        "This preview has bugs left, and the game's code stopped at one of them."
    };
    format!(
        "The game stopped (status {}).\n\n{what}\n\nPlease report it at {ISSUES} and attach the log named below. It may show your Windows user name in folder paths.\n\nLast line of the log:\n{last}",
        status.map_or("unknown".to_string(), |s| s.to_string())
    )
}

/// The last 64 KB of the log, as text.
fn log_tail(path: &Path) -> String {
    let Ok(bytes) = std::fs::read(path) else {
        return String::new();
    };
    let from = bytes.len().saturating_sub(64 * 1024);
    String::from_utf8_lossy(&bytes[from..]).into_owned()
}

struct Options {
    mods: bool,
    import: bool,
    check_only: bool,
    game: Option<PathBuf>,
    from: Option<PathBuf>,
}

fn parse_options(raw: &[String]) -> Options {
    let mut options = Options {
        mods: false,
        import: false,
        check_only: false,
        game: std::env::var_os("NV_RS_GAME").map(PathBuf::from),
        from: None,
    };
    let mut args = raw.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--mods" => options.mods = true,
            "--import-saves" => options.import = true,
            "--check" => options.check_only = true,
            "--game" => options.game = args.next().map(PathBuf::from),
            "--from" => options.from = args.next().map(PathBuf::from),
            _ => {}
        }
    }
    options
}

/// `--import-saves`: the player's saves copied into the package, never the other way, never over a
/// save already there.
fn import_saves(package: &Package, profile: &Profile, from: Option<&Path>) -> Result<i32, Stop> {
    let source = match from {
        Some(path) => path.to_path_buf(),
        None => match real_documents() {
            Some(docs) => docs.join("My Games\\FalloutNV\\Saves"),
            None => {
                return stop(
                    STOP_LOCATION,
                    "Windows did not say where your Documents folder is.",
                )
            }
        },
    };
    let saves: Vec<PathBuf> = std::fs::read_dir(&source)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.is_file() && p.extension().is_some_and(|x| x.eq_ignore_ascii_case("fos"))
                })
                .collect()
        })
        .unwrap_or_default();
    if saves.is_empty() {
        let text = format!("No saves were found in:\n{}", source.display());
        package.log(&text);
        ui::info(&text);
        return Ok(0);
    }
    if !ui::ask(&format!(
        "Copy {} saves into nv-rs's own folder?\n\nFrom (only read, nothing there changes):\n{}\n\nTo:\n{}\n\nA save already there stays as it is. A save made with mods needs those mods: use Play with mods for it.",
        saves.len(),
        source.display(),
        profile.saves.display()
    )) {
        return Ok(0);
    }
    profile.make().map_err(|e| Stop {
        code: STOP_LOCATION,
        message: format!("{}: {e}", profile.saves.display()),
    })?;
    let (mut copied, mut kept) = (0, 0);
    for save in &saves {
        let Some(name) = save.file_name() else {
            continue;
        };
        let target = profile.saves.join(name);
        if target.exists() {
            kept += 1;
            continue;
        }
        let mut temporary = target.as_os_str().to_owned();
        temporary.push(".part");
        // `copy` only reads the source, and keeps its date (the game sorts saves by date).
        let done =
            std::fs::copy(save, &temporary).and_then(|_| std::fs::rename(&temporary, &target));
        match done {
            Ok(()) => copied += 1,
            Err(e) => package.log(&format!("{}: {e}", save.display())),
        }
    }
    let text = format!("Saves copied: {copied}. Already there, left as they were: {kept}.");
    package.log(&format!("import from {}: {text}", source.display()));
    ui::info(&text);
    Ok(0)
}

fn run(package: &Package, options: &Options) -> Result<i32, Stop> {
    check_location(package)?;
    let profile = Profile::at(
        &package.userdata,
        if options.mods { "mods" } else { "game" },
    );
    if options.import {
        clear_for_writing(package, None)?;
        return import_saves(package, &profile, options.from.as_deref());
    }
    if win::another_is_running() {
        return stop(
            STOP_LOCATION,
            "nv-rs is already running. Only one game at a time.",
        );
    }
    let status = ui::Status::show("Looking for Fallout: New Vegas...");
    let result = play(package, options, &profile, &status);
    status.close();
    result
}

fn play(
    package: &Package,
    options: &Options,
    profile: &Profile,
    status: &ui::Status,
) -> Result<i32, Stop> {
    let (mut game, check_lines) = find_game(package, options.game.as_deref())?;
    // The first write of the run: the game's folder is known and the package is not in it.
    clear_for_writing(package, Some(&game.folder))?;
    for line in &check_lines {
        package.log(line);
    }
    let _ = std::fs::write(
        &package.remembered,
        game.folder.to_string_lossy().as_bytes(),
    );
    // DirectX 9's D3DX is not part of Windows; the game's installer adds it.
    let system = std::env::var_os("NV_RS_SYSTEM_FOLDER")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("SystemRoot").map(|r| PathBuf::from(r).join("System32")))
        .unwrap_or_else(|| PathBuf::from("C:\\Windows\\System32"));
    if !system.join("d3dx9_38.dll").is_file() {
        return stop(
            STOP_DIRECTX,
            "DirectX 9's D3DX library (64-bit d3dx9_38.dll) is not on this PC. Windows doesn't include it, and nv-rs uses it for the game's shaders and textures.\n\nInstall \"DirectX End-User Runtimes (June 2010)\" from Microsoft's download site, then start Play.exe again. It is safe alongside newer DirectX.",
        );
    }

    profile.make().map_err(|e| Stop {
        code: STOP_LOCATION,
        message: format!(
            "nv-rs could not make its folders ({e}):\n{}",
            package.userdata.display()
        ),
    })?;
    std::fs::create_dir_all(&package.machine).map_err(|e| Stop {
        code: STOP_LOCATION,
        message: format!("{}: {e}", package.machine.display()),
    })?;

    // Mods: a whole install of links and copies in the package.
    if options.mods {
        status.set("Getting the mods ready...");
        let install = package.userdata.join("modgame");
        let plugins = profile.local.join("FalloutNV\\plugins.txt");
        let built =
            mods::build_install(&game.folder, &package.root.join("mods"), &install, &plugins)
                .map_err(|e| Stop {
                    code: STOP_MODS,
                    message: e.player_text(),
                })?;
        package.log(&format!(
            "mods: {} files linked, {} mod files",
            built.linked, built.mod_files
        ));
        if built.extender_plugins {
            ui::warning("Some of your mods include plugins for xNVSE or other script extenders. Those plugins are not used yet, and mods that need them may not work or may stop the game.");
        }
        game = Game {
            data: install.join("Data"),
            folder: install,
        };
    }

    // The tables.
    status.set("Reading your graphics card...");
    let tables_log = package.machine.join("tables.log");
    let adapter = package.machine.join("d3d9_adapter.tsv");
    let game_exe = game.folder.join("FalloutNV.exe");
    let table = match tables::make_adapter_table(&package.tools, &game_exe, &tables_log) {
        Ok(bytes) => {
            tables::write_whole(&adapter, &bytes).map_err(|e| Stop {
                code: STOP_TABLES,
                message: format!("{}: {e}", adapter.display()),
            })?;
            bytes
        }
        Err(why) => {
            package.log(&format!("the graphics table was not made: {why}"));
            match std::fs::read(&adapter) {
                Ok(bytes) if !why.contains("hardware") => {
                    package.log("using the last good graphics table");
                    bytes
                }
                _ => {
                    return stop(
                        STOP_TABLES,
                        format!(
                            "nv-rs could not read your graphics card: {why}.\n\nThe game needs hardware Direct3D 9 from a graphics driver on this PC. Over Remote Desktop, or with no driver installed, there is none. Update the graphics driver and try again.\n\nIf it still happens, please report it at {ISSUES} with this file:\n{}",
                            tables_log.display()
                        ),
                    )
                }
            }
        }
    };

    status.set("Reading your sound device. The first time this takes about 20 seconds...");
    let sound = package.machine.join("dsound.tsv");
    let fingerprint = tables::sound_fingerprint();
    let stamp = package.machine.join("dsound.machine.txt");
    match tables::make_sound_table(&package.tools, &sound, &stamp, &fingerprint, &tables_log) {
        Ok(made) => {
            package.log(if made {
                "the sound table was made"
            } else {
                "the sound table is still right"
            });
            if made {
                sound_notes(package, &sound, &check_lines);
            }
        }
        Err(why) if sound.is_file() => {
            package.log(&format!(
                "the sound table was not made again ({why}): using the last one"
            ));
        }
        Err(why) => {
            package.log(&format!("the sound table was not made: {why}"));
            // What DirectSound answers without a device (DSERR_NODRIVER): the game's own way of
            // going on without sound.
            let _ = tables::write_whole(&sound, b"DsCreate\t\t88780078\t\nDsEnd\n");
            let _ = std::fs::remove_file(&stamp);
            ui::warning(&format!(
                "nv-rs could not read your sound device ({why}), so the game will start without sound.\n\nIf you have sound normally, please report this at {ISSUES} and attach:\n{}",
                tables_log.display()
            ));
        }
    }

    // The game's folders in the package.
    let refuse_path = |path: &Path| {
        Stop {
        code: STOP_LOCATION,
        message: format!(
            "The game can't use this folder's path, because of the letters in it on this Windows' language setting:\n{}\n\nMove the whole nv-rs folder to a path with plain letters only, for example C:\\Games\\nv-rs.",
            path.display()
        ),
    }
    };
    let documents =
        guest_bytes(&profile.documents).ok_or_else(|| refuse_path(&profile.documents))?;
    let roaming = guest_bytes(&profile.roaming).ok_or_else(|| refuse_path(&profile.roaming))?;
    let local = guest_bytes(&profile.local).ok_or_else(|| refuse_path(&profile.local))?;
    if path_too_long(documents.len()) {
        return stop(
            STOP_LOCATION,
            format!(
                "This folder's path is too long for the game:\n{}\n\nMove the whole nv-rs folder to a shorter path, for example C:\\Games\\nv-rs.",
                package.root.display()
            ),
        );
    }
    let data_argument = match game.data.to_str() {
        Some(text) if text.is_ascii() => game.data.clone(),
        _ => match guest_bytes(&game.data) {
            Some(bytes) => match String::from_utf8(bytes) {
                Ok(text) => PathBuf::from(text),
                Err(_) => game.data.clone(),
            },
            None => return Err(refuse_path(&game.data)),
        },
    };
    let device = tables::adapter_name(&table);
    let table = tables::with_folders(&table, &documents, &roaming, &local).map_err(|why| Stop {
        code: STOP_TABLES,
        message: format!(
            "nv-rs could not prepare the game's folders: {why}. Please report this at {ISSUES}."
        ),
    })?;
    tables::write_whole(&profile.table, &table).map_err(|e| Stop {
        code: STOP_TABLES,
        message: format!("{}: {e}", profile.table.display()),
    })?;

    let settings = Settings::parse(&settings::decode(
        &std::fs::read(&package.settings).unwrap_or_default(),
    ));
    let game_switches = settings.game_switches().map_err(|why| Stop {
        code: STOP_SETTINGS,
        message: format!(
            "nv-rs's settings have a line it can't use: {why}.\n\nOpen Settings.cmd, choose again and click Save. The file is:\n{}",
            package.settings.display()
        ),
    })?;
    if settings
        .0
        .keys()
        .any(|k| k.eq_ignore_ascii_case("NV_FIXES"))
    {
        package.log("settings.ini's NV_FIXES line is not used: Preset= chooses the fixes");
    }
    for raw in ["NV_PRESET", "NV_FRAME_LIMIT", "NV_MENU_FRAME_LIMIT"] {
        if settings.0.keys().any(|k| k.eq_ignore_ascii_case(raw)) {
            package.log(&format!(
                "settings.ini's {raw} line is not used: Preset=, FrameLimit= and MenuFrameLimit= choose them (Settings.cmd)"
            ));
        }
    }
    prepare_prefs(package, profile, &settings, &game.folder, device.as_deref()).map_err(|e| {
        Stop {
            code: STOP_LOCATION,
            message: format!("{}: {e}", profile.prefs.display()),
        }
    })?;

    if options.check_only {
        let mut text = format!("Game: {}\n", game.folder.display());
        for line in &check_lines {
            text.push_str(line);
            text.push('\n');
        }
        text.push_str("\nTables made. Everything is ready to play.");
        package.log(&text);
        ui::info(&text);
        return Ok(0);
    }

    // The run.
    status.set("Starting the game...");
    let (_, stamp_text) = win::now();
    tidy_logs(&package.logs);
    let log_path = package.logs.join(format!("game-{stamp_text}.log"));
    let log_file = std::fs::File::create(&log_path).map_err(|e| Stop {
        code: STOP_LOCATION,
        message: format!("{}: {e}", log_path.display()),
    })?;
    let log_clone = log_file.try_clone().map_err(|e| Stop {
        code: STOP_LOCATION,
        message: format!("{}: {e}", log_path.display()),
    })?;
    let mut command = Command::new(&package.game_exe);
    // What settings.ini passes on first, so the launcher's own switches below win over it.
    for (name, value) in user_switches(&settings) {
        command.env(name, value);
    }
    command
        .arg(&data_argument)
        .current_dir(&package.userdata)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_clone))
        .creation_flags(CREATE_NO_WINDOW)
        .env("NV_D3D9", "real")
        .env("NV_SOUND", "out")
        .env("NV_D3D9_ADAPTER", &profile.table)
        .env("NV_DSOUND_TABLE", &sound)
        .env("NV_D3DX_CACHE", package.machine.join("d3dx-cache"))
        .env("NV_SAVE_OUT", &profile.saves)
        .env("NV_PREFS_OUT", &profile.prefs)
        .env("NV_REPORT_DIR", &package.reports)
        .env("NV_REPORT_LOG", &log_path)
        .env("NV_SETTINGS", &package.settings);
    for (name, value) in &game_switches {
        command.env(name, value);
    }
    if settings.borderless() {
        command.env("NV_BORDERLESS", "1");
    } else {
        command.env_remove("NV_BORDERLESS");
    }
    // The run's Fallout.ini wins over settings.ini (host `game_ini.rs`): each player switch its
    // [nv-rs ...] sections set goes to the game with the file's value instead. Not when a
    // settings.ini line (or the launcher's own environment) makes the run a test, which never
    // reads the file: then settings.ini's switches stand, as the game sees it.
    let fallout_ini = profile.documents.join("My Games\\FalloutNV\\Fallout.ini");
    let test = settings::makes_a_test(
        user_switches(&settings).into_iter().map(|(name, _)| name),
        |name| std::env::var_os(name).is_some(),
    );
    if let Some(bytes) = (!test).then(|| std::fs::read(&fallout_ini).ok()).flatten() {
        let from_file = Settings::file_switches(&settings::decode(&bytes));
        for (name, value) in &from_file {
            match value {
                Some(value) => command.env(name, value),
                None => command.env_remove(name),
            };
        }
        if !from_file.is_empty() {
            let names: Vec<&str> = from_file.iter().map(|(name, _)| *name).collect();
            package.log(&format!(
                "Fallout.ini's nv-rs settings over settings.ini: {}",
                names.join(", ")
            ));
        }
    }

    package.log(&format!(
        "starting the game on {}; preset {}, frame limit {}, menu frame limit {}; log {}",
        data_argument.display(),
        game_switches[0].1,
        game_switches[1].1,
        game_switches[2].1,
        log_path.display()
    ));
    let mut child = command.spawn().map_err(|e| Stop {
        code: STOP_BLOCKED,
        message: format!(
            "Windows or your antivirus did not let nv-rs start its program ({e}).\n\nThe program is new and not signed, so this can happen: see READ ME.txt. Program:\n{}",
            package.game_exe.display()
        ),
    })?;
    let mut waiting_for_window = true;
    let exit = loop {
        match child.try_wait() {
            Ok(Some(code)) => break code,
            Ok(None) => {}
            Err(e) => {
                package.log(&format!("waiting for the game: {e}"));
                break child.wait().map_err(|e| Stop {
                    code: STOP_GAME_STOPPED,
                    message: e.to_string(),
                })?;
            }
        }
        if waiting_for_window && ui::game_window_is_there() {
            waiting_for_window = false;
            status.close();
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    };
    status.close();
    package.log(&format!("the game ended with status {:?}", exit.code()));
    if exit.success() {
        return Ok(0);
    }
    let message = plain_end(&log_tail(&log_path), exit.code());
    package.log(&message);
    let text = format!(
        "{message}\n\nLog:\n{}\n\nOpen the logs folder?",
        log_path.display()
    );
    if ui::windows_allowed() && ui::ask(&text) {
        ui::open_folder(&package.logs);
    }
    Ok(STOP_GAME_STOPPED)
}

/// The switches the launcher sets itself: a `NV_` line in settings.ini can't replace them (it
/// could point the saves or settings anywhere). `NV_PRESET`, `NV_FRAME_LIMIT` and
/// `NV_MENU_FRAME_LIMIT` come from the `Preset=`, `FrameLimit=` and `MenuFrameLimit=` lines
/// Settings.cmd writes, so a second, raw line can't disagree with
/// what Settings.cmd shows. `NV_FIXES` was never a switch of the game's (an early name for the
/// preset, in a test): it is dropped, so a stale line can't look as if it did something.
const OWN_SWITCHES: [&str; 15] = [
    "NV_PRESET",
    "NV_FRAME_LIMIT",
    "NV_MENU_FRAME_LIMIT",
    "NV_FIXES",
    "NV_D3D9",
    "NV_SOUND",
    "NV_D3D9_ADAPTER",
    "NV_DSOUND_TABLE",
    "NV_D3DX_CACHE",
    "NV_SAVE_OUT",
    "NV_PREFS_OUT",
    "NV_REPORT_DIR",
    "NV_REPORT_LOG",
    "NV_BORDERLESS",
    "NV_SETTINGS",
];

/// settings.ini's `NV_` lines that go to the game: all but the launcher's own switches.
fn user_switches(settings: &Settings) -> Vec<(&str, &str)> {
    settings
        .environment()
        .into_iter()
        .filter(|(name, _)| {
            !OWN_SWITCHES
                .iter()
                .any(|own| own.eq_ignore_ascii_case(name))
        })
        .collect()
}

/// What the sound table and the game's own check say about sound, once when the table is made.
fn sound_notes(package: &Package, sound: &Path, check_lines: &[String]) {
    let table = std::fs::read_to_string(sound).unwrap_or_default();
    if tables::sound_state(&table) == Ok(tables::Sound::NoDevice) {
        package.log("no sound device");
        ui::warning("Windows reports no sound device, so the game will start without sound.\n\nPlug in or turn on a sound device and start Play.exe again: nv-rs notices the change and reads the new device.");
        return;
    }
    for line in check_lines {
        if let Some((_, why)) = line.split_once("not provided: ") {
            package.log(&format!("sound layout: {why}"));
            ui::warning(&format!(
                "The game will have no sound on your current sound device:\n{why}.\n\nIn Windows' sound settings, set the device to stereo, 5.1 or 7.1, or choose another device, then start Play.exe again."
            ));
        }
    }
}

fn main() {
    let raw: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    let options = parse_options(&raw);
    let root = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));
    let package = Package::at(root);
    package.log(&format!(
        "Play.exe {:?} in {}; Windows {}; code page {}",
        raw,
        package.root.display(),
        win::windows_build(),
        win::ansi_code_page()
    ));
    match run(&package, &options) {
        Ok(code) => std::process::exit(code),
        Err(why) => {
            // A stop before the first write keeps the disk untouched, unless the package's place is
            // fine (nothing found, wrong version...): then the log is for the player's report.
            if !package.writing.get() && why.code != STOP_LOCATION {
                let _ = clear_for_writing(&package, None);
            }
            package.log(&format!("stopped ({}): {}", why.code, why.message));
            ui::error(&why.message);
            std::process::exit(why.code);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_compare_without_case_and_whole_folders_only() {
        let key = |p: &str| folder_key(Path::new(p));
        assert!(is_inside(
            &key("C:/Games/FNV/nv-rs"),
            &key("c:\\games\\fnv\\")
        ));
        assert!(is_inside(&key("C:\\Games\\FNV"), &key("C:\\Games\\fnv")));
        assert!(!is_inside(&key("C:\\Games\\FNV2"), &key("C:\\Games\\FNV")));
    }

    #[test]
    fn a_long_path_is_refused_before_the_game_runs_out_of_room() {
        assert!(!path_too_long(170));
        assert!(path_too_long(180));
    }

    #[test]
    fn the_players_message_names_what_happened() {
        let tail = "Game code: ok\nsomething\nGame's main stopped in frame 12: open function 00123456\nmore\n";
        let text = plain_end(tail, Some(3));
        assert!(text.contains("status 3"));
        assert!(text.contains("Game's main stopped in frame 12"));
        assert!(plain_end(
            "panicked: the sound table disagrees with the layer",
            Some(3)
        )
        .contains("Sound on this PC"));
        assert!(plain_end("IDirect3D9::CheckDeviceFormat(0 1) was not captured in the adapter table: not provided", Some(3))
            .contains("graphics card"));
        assert!(plain_end(
            "NV_D3D9=real: CreateDevice answered 0x8876086a: nothing is forwarded",
            None
        )
        .contains("Settings.cmd"));
        assert!(plain_end("", None).contains("the log is empty"));
    }

    #[test]
    fn arguments_are_read() {
        let raw: Vec<String> = [
            "--mods",
            "--game",
            "D:\\x",
            "--import-saves",
            "--from",
            "E:\\y",
            "--other",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let o = parse_options(&raw);
        assert!(o.mods && o.import && !o.check_only);
        assert_eq!(o.game.unwrap(), PathBuf::from("D:\\x"));
        assert_eq!(o.from.unwrap(), PathBuf::from("E:\\y"));
    }

    #[test]
    fn only_old_game_logs_go() {
        let dir = std::env::temp_dir().join(format!("nv-rs-logs-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for i in 0..14 {
            std::fs::write(dir.join(format!("game-2026{i:02}.log")), b"x").unwrap();
        }
        std::fs::write(dir.join("launcher.log"), b"x").unwrap();
        tidy_logs(&dir);
        assert!(!dir.join("game-202600.log").exists());
        assert!(!dir.join("game-202603.log").exists());
        assert!(dir.join("game-202604.log").exists());
        assert!(dir.join("launcher.log").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    fn scratch(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("nv-rs-launcher-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// `link` made a junction to `target` (`mklink /J`, no rights needed).
    fn junction(link: &Path, target: &Path) {
        let made = Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(link)
            .arg(target)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .unwrap();
        assert!(made.status.success(), "{made:?}");
    }

    #[test]
    fn a_junction_and_a_short_name_are_the_same_place() {
        let dir = scratch("junction");
        let real = dir.join("Real Game Folder");
        std::fs::create_dir_all(real.join("nv-rs")).unwrap();
        let link = dir.join("link");
        junction(&link, &real);
        assert_eq!(folder_key(&link), folder_key(&real));
        assert!(inside_game(&real.join("nv-rs"), &link));
        assert!(inside_game(&link.join("nv-rs"), &real));
        assert!(!inside_game(&dir.join("elsewhere"), &real));
        // The 8.3 name, when this drive has one.
        if let Some(short) = win::short_path(&real) {
            assert_eq!(folder_key(&short), folder_key(&real));
        }
        // A part that doesn't exist yet follows the real path of the part that does.
        assert_eq!(
            folder_key(&link.join("not yet")),
            format!("{}\\not yet", folder_key(&real))
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_refusal_before_the_first_write_leaves_the_disk_untouched() {
        let dir = scratch("untouched");
        let game = dir.join("game");
        let root = game.join("nv-rs");
        std::fs::create_dir_all(&root).unwrap();
        let link = dir.join("link");
        junction(&link, &game);
        let package = Package::at(root.clone());
        package.log("held in memory");
        for found in [game.clone(), link.clone()] {
            let why = clear_for_writing(&package, Some(&found)).err().unwrap();
            assert_eq!(why.code, STOP_LOCATION);
            assert!(why.message.contains("inside your game's folder"));
        }
        package.log("still in memory");
        assert!(!package.writing.get());
        assert_eq!(
            std::fs::read_dir(&root).unwrap().count(),
            0,
            "nothing written"
        );
        // Where it is fine, the first write comes after the checks, with the kept lines first.
        let other = dir.join("other game");
        std::fs::create_dir_all(&other).unwrap();
        clear_for_writing(&package, Some(&other)).unwrap();
        package.log("written");
        let log = std::fs::read_to_string(package.logs.join("launcher.log")).unwrap();
        let lines: Vec<&str> = log
            .lines()
            .map(|l| l.split_once(' ').unwrap().1.split_once(' ').unwrap().1)
            .collect();
        assert_eq!(lines, ["held in memory", "still in memory", "written"]);
        assert!(!root.join(".nv-rs-write-test").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_program_that_hangs_is_ended_and_one_that_ends_is_read() {
        let mut slow = Command::new("cmd");
        slow.args(["/c", "ping -n 30 127.0.0.1 >nul"]);
        let start = Instant::now();
        assert!(matches!(
            run_limited(&mut slow, Duration::from_millis(700)),
            Limited::TimedOut
        ));
        assert!(start.elapsed() < Duration::from_secs(10));
        let mut quick = Command::new("cmd");
        quick.args(["/c", "echo hello"]);
        match run_limited(&mut quick, Duration::from_secs(30)) {
            Limited::Done(o) => {
                assert!(o.status.success());
                assert!(String::from_utf8_lossy(&o.stdout).contains("hello"));
            }
            _ => panic!("did not finish"),
        }
        assert!(matches!(
            run_limited(
                &mut Command::new("C:\\no\\such\\program.exe"),
                Duration::from_secs(5)
            ),
            Limited::Failed(_)
        ));
    }

    #[test]
    fn a_game_program_that_is_locked_says_so() {
        use std::os::windows::fs::OpenOptionsExt;
        let dir = scratch("locked");
        let exe = dir.join("FalloutNV.exe");
        std::fs::write(&exe, b"x").unwrap();
        assert!(exe_read_problem(&exe).is_none());
        let held = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(0)
            .open(&exe)
            .unwrap();
        let text = exe_read_problem(&exe).unwrap();
        assert!(text.contains("Another program has it open"), "{text}");
        assert!(!text.contains("antivirus blocks nv-rs"));
        drop(held);
        let text = exe_read_problem(&dir.join("missing.exe")).unwrap();
        assert!(text.contains("can't read"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn settings_ini_cannot_redirect_the_launchers_own_switches() {
        let settings = Settings::parse(
            "NV_SAVE_OUT=C:\\Elsewhere\nNV_PREFS_OUT=C:\\Elsewhere\\p.ini\nNV_SETTINGS=x\nNV_FIXES=vnv\nNV_BORDERLESS=0\nNV_SOMETHING_NEW=1\n",
        );
        assert_eq!(user_switches(&settings), [("NV_SOMETHING_NEW", "1")]);
    }

    #[test]
    fn the_preset_reaches_the_game_only_from_the_preset_line() {
        // Raw NV_PRESET / NV_FRAME_LIMIT / NV_MENU_FRAME_LIMIT lines and a stale NV_FIXES don't go
        // through; Preset=, FrameLimit= and MenuFrameLimit= do, as the game's switches.
        let settings = Settings::parse(
            "Preset=vanilla\nFrameLimit=90\nMenuFrameLimit=60\nNV_PRESET=fixes\nnv_frame_limit=30\nNV_MENU_FRAME_LIMIT=0\nNV_FIXES=vnv\n",
        );
        assert!(user_switches(&settings).is_empty());
        assert_eq!(
            settings.game_switches().unwrap(),
            [
                ("NV_PRESET", "vanilla".to_string()),
                ("NV_FRAME_LIMIT", "90".to_string()),
                ("NV_MENU_FRAME_LIMIT", "60".to_string())
            ]
        );
    }

    #[test]
    fn onedrive_documents_is_refused_as_the_game_refuses_it() {
        let profile = scratch("profile");
        let folders = profile_folders(&profile);
        let inside = |path: PathBuf| player_folder_of(&path, &folders);
        assert_eq!(
            inside(profile.join("OneDrive\\Documents\\nv-rs")),
            Some("Documents")
        );
        assert_eq!(
            inside(profile.join("onedrive/documents/Games/nv-rs")),
            Some("Documents")
        );
        assert_eq!(inside(profile.join("Documents\\nv-rs")), Some("Documents"));
        assert_eq!(
            inside(profile.join("AppData\\Local\\nv-rs")),
            Some("AppData")
        );
        // The rest of OneDrive is not the game's, nor a name that only starts the same.
        assert_eq!(inside(profile.join("OneDrive\\Desktop\\nv-rs")), None);
        assert_eq!(inside(profile.join("OneDrive\\Documents2\\nv-rs")), None);
        assert_eq!(inside(profile.join("Games\\nv-rs")), None);
        // A junction into OneDrive's Documents is the same place.
        std::fs::create_dir_all(profile.join("OneDrive\\Documents")).unwrap();
        junction(&profile.join("link"), &profile.join("OneDrive\\Documents"));
        assert_eq!(inside(profile.join("link\\nv-rs")), Some("Documents"));
        let _ = std::fs::remove_dir_all(&profile);
    }
}
