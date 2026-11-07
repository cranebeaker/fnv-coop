//! `NV_SAVE_OUT=DIR`, for scripted runs and the preview package: the save files the game wrote in
//! this run copied into `DIR`, so a later run can start from them through the main menu (CONTINUE
//! or LOAD) instead of replaying the opening. A save the game deleted in this run is deleted there
//! too. `NV_PREFS_OUT=FILE`: the same for the `FalloutPrefs.ini` the game wrote (the settings
//! changed in its menus).
//!
//! Files the game writes stay in engine memory (`engine_platform::files::Disk`); nothing
//! reaches the disk unless this is asked for. Only files whose name ends in `.fos` are copied,
//! each under its own name (as its directory entry has it). A file is written to a temporary
//! name first and then renamed, so a reader never sees half of it. A copy is made again whenever
//! the game's bytes change. A save still being written may be copied part-way; it is copied
//! again, complete, at the next check.
//!
//! `DIR` must be a scratch folder or the package's own. One inside the player's Documents,
//! OneDrive's Documents or AppData is refused, and nothing is written.

use engine::Engine;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

/// `NV_SAVE_OUT`'s folder, or `None` when it is unset or refused (refused once, with a line).
fn out_dir() -> Option<&'static Path> {
    static DIR: OnceLock<Option<PathBuf>> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = PathBuf::from(std::env::var_os("NV_SAVE_OUT")?);
        let profile = std::env::var_os("USERPROFILE").map(PathBuf::from);
        if let Some(why) = refused(&dir, profile.as_deref(), &known_folders()) {
            warn_visibly(
                &format!(
                    "NV_SAVE_OUT={}: refused ({why}); saves are not copied.",
                    dir.display()
                ),
                "Your saves are NOT being kept: they will be lost when the game closes. The save folder the game was given is one it must not write to. Start the game through Play.exe, and report this if you did.",
            );
            return None;
        }
        Some(dir)
    })
    .as_deref()
}

/// A warning the player can't miss: a line in the log, and on Windows a message box (from its own
/// thread, so the game goes on), unless `NV_RS_NO_WINDOWS` is set (scripted runs).
fn warn_visibly(line: &str, plain: &str) {
    println!("{line}");
    #[cfg(windows)]
    if std::env::var_os("NV_RS_NO_WINDOWS").is_none() {
        let text: Vec<u16> = format!("{plain}\n\n{line}")
            .encode_utf16()
            .chain([0])
            .collect();
        let title: Vec<u16> = "nv-rs".encode_utf16().chain([0]).collect();
        std::thread::spawn(move || {
            #[link(name = "user32")]
            extern "system" {
                fn MessageBoxW(
                    window: isize,
                    text: *const u16,
                    title: *const u16,
                    kind: u32,
                ) -> i32;
            }
            // MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST
            // SAFETY: both strings end in a NUL and live through the call.
            unsafe { MessageBoxW(0, text.as_ptr(), title.as_ptr(), 0x30 | 0x1_0000 | 0x4_0000) };
        });
    }
    #[cfg(not(windows))]
    let _ = plain;
}

/// The player's Documents, AppData and Local AppData as Windows answers for them (wherever they
/// were moved to: OneDrive, another drive); empty off Windows.
pub fn known_folders() -> Vec<PathBuf> {
    #[cfg(windows)]
    {
        #[link(name = "shell32")]
        extern "system" {
            fn SHGetFolderPathW(
                window: isize,
                csidl: i32,
                token: isize,
                flags: u32,
                path: *mut u16,
            ) -> i32;
        }
        let mut out = Vec::new();
        for csidl in [0x05, 0x1a, 0x1c] {
            let mut buffer = vec![0u16; 1024];
            // SAFETY: the buffer is longer than MAX_PATH, as the call asks.
            let hr = unsafe { SHGetFolderPathW(0, csidl, 0, 0, buffer.as_mut_ptr()) };
            if hr == 0 {
                let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
                out.push(PathBuf::from(String::from_utf16_lossy(&buffer[..end])));
            }
        }
        out
    }
    #[cfg(not(windows))]
    Vec::new()
}

/// The path the system really means: links, junctions, `subst` and short names resolved, a part
/// that does not exist yet kept after the real path of the longest part that does.
fn real_path(path: &Path) -> PathBuf {
    let mut rest = Vec::new();
    let mut at = path.to_path_buf();
    loop {
        if let Ok(real) = std::fs::canonicalize(&at) {
            let mut out = real;
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

/// Why `dir` may not receive saves: it lies inside one of the player's own folders under
/// `profile` (Documents, OneDrive's Documents, AppData) or inside one of the `known` folders
/// (Windows' own answer for Documents and AppData, wherever they were moved to), by its text or by
/// the real path it leads to (a junction or a drive made with `subst` doesn't get round this).
/// Compared without case, `/` read as `\`.
/// The rest of OneDrive is not the game's: a Desktop kept there is where many people unzip things.
pub fn refused(dir: &Path, profile: Option<&Path>, known: &[PathBuf]) -> Option<&'static str> {
    let norm = |p: &Path| {
        let text = p.to_string_lossy().replace('/', "\\").to_ascii_lowercase();
        let text = match text.strip_prefix("\\\\?\\unc\\") {
            Some(rest) => format!("\\\\{rest}"),
            None => text
                .strip_prefix("\\\\?\\")
                .map_or(text.clone(), str::to_owned),
        };
        text.trim_end_matches('\\').to_owned()
    };
    let real_dir = norm(&real_path(dir));
    let dir = norm(dir);
    if !dir.contains(':') && !dir.starts_with("\\\\") {
        return Some("not a full path");
    }
    if dir.split('\\').any(|part| part == "..") {
        return Some("a `..` in the path");
    }
    let Some(profile) = profile else {
        return Some("USERPROFILE is not set, so the player's folders are unknown");
    };
    let profile = norm(profile);
    let mut roots: Vec<String> = ["documents", "onedrive\\documents", "appdata"]
        .iter()
        .map(|player| format!("{profile}\\{player}"))
        .collect();
    for folder in known {
        roots.push(norm(folder));
        roots.push(norm(&real_path(folder)));
    }
    for root in roots {
        for place in [&dir, &real_dir] {
            if *place == root || place.starts_with(&format!("{root}\\")) {
                return Some("inside the player's own folders");
            }
        }
    }
    None
}

/// Copies the run's `.fos` files into `NV_SAVE_OUT` where they are new or changed since the last
/// call; the names copied, for the log.
pub fn copy_saves(e: &Engine) -> Vec<String> {
    copy_prefs(e);
    let Some(dir) = out_dir() else {
        return vec![];
    };
    static DONE: Mutex<Option<HashMap<Vec<u8>, u64>>> = Mutex::new(None);
    let mut done = DONE.lock().unwrap_or_else(|p| p.into_inner());
    let mut names = copy_changed(e, dir, done.get_or_insert_with(HashMap::new));
    names.extend(remove_deleted(e, dir));
    names
}

/// `NV_PREFS_OUT`'s file gets the game's `FalloutPrefs.ini` when its bytes change.
fn copy_prefs(e: &Engine) {
    static FILE: OnceLock<Option<PathBuf>> = OnceLock::new();
    let Some(file) = FILE
        .get_or_init(|| {
            let file = PathBuf::from(std::env::var_os("NV_PREFS_OUT")?);
            let profile = std::env::var_os("USERPROFILE").map(PathBuf::from);
            if let Some(why) = refused(&file, profile.as_deref(), &known_folders()) {
                warn_visibly(
                    &format!(
                        "NV_PREFS_OUT={}: refused ({why}); settings are not copied.",
                        file.display()
                    ),
                    "Your changed game settings are NOT being kept: they will be lost when the game closes. The settings file the game was given is in a folder it must not write to. Start the game through Play.exe, and report this if you did.",
                );
                return None;
            }
            Some(file)
        })
        .as_deref()
    else {
        return;
    };
    static DONE: Mutex<Option<u64>> = Mutex::new(None);
    let mut done = DONE.lock().unwrap_or_else(|p| p.into_inner());
    prefs_changed(e, file, &mut done);
}

/// [`copy_prefs`] into `file`, with `done` the fingerprint of the bytes copied last.
pub fn prefs_changed(e: &Engine, file: &Path, done: &mut Option<u64>) {
    use std::hash::{Hash, Hasher};
    let disk = &engine_platform::state(&e.kernel.platform).disk;
    let Some(data) = disk
        .files
        .iter()
        .find(|(key, _)| key.ends_with(b"\\falloutprefs.ini"))
        .map(|(_, data)| data)
    else {
        return;
    };
    let mut h = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut h);
    let print = h.finish();
    if *done == Some(print) {
        return;
    }
    let temporary = file.with_extension("ini.part");
    let written = std::fs::write(&temporary, data).and_then(|()| std::fs::rename(&temporary, file));
    match written {
        Ok(()) => *done = Some(print),
        Err(err) => println!("NV_PREFS_OUT: {} not copied: {err}", file.display()),
    }
}

/// The saves the game deleted in this run go from `dir` too, where a file of that name is there
/// and the game has not made it again. The names, for the log.
pub fn remove_deleted(e: &Engine, dir: &Path) -> Vec<String> {
    let disk = &engine_platform::state(&e.kernel.platform).disk;
    let mut removed = Vec::new();
    for key in &disk.deleted {
        if !key.ends_with(b".fos") || disk.files.contains_key(key) {
            continue;
        }
        let name = key.rsplit(|&b| b == b'\\').next().unwrap_or(key);
        let name = String::from_utf8_lossy(name).into_owned();
        let target = dir.join(&name);
        if target.is_file() && std::fs::remove_file(&target).is_ok() {
            removed.push(format!("{name} (deleted)"));
        }
    }
    removed
}

/// [`copy_saves`] into `dir`, with `done` the fingerprints of the bytes copied so far by key.
pub fn copy_changed(e: &Engine, dir: &Path, done: &mut HashMap<Vec<u8>, u64>) -> Vec<String> {
    use std::hash::{Hash, Hasher};
    let mut copied = Vec::new();
    let disk = &engine_platform::state(&e.kernel.platform).disk;
    for (key, data) in &disk.files {
        if !key.ends_with(b".fos") {
            continue;
        }
        let mut h = std::collections::hash_map::DefaultHasher::new();
        data.hash(&mut h);
        let print = h.finish();
        if done.get(key) == Some(&print) {
            continue;
        }
        let name = key.rsplit(|&b| b == b'\\').next().unwrap_or(key);
        let name = String::from_utf8_lossy(name).into_owned();
        let name = disk
            .entries
            .get(key)
            .map_or(name, |entry| entry.name.clone());
        let target = dir.join(&name);
        let temporary = dir.join(format!("{name}.part"));
        let written = std::fs::create_dir_all(dir)
            .and_then(|()| std::fs::write(&temporary, data))
            .and_then(|()| std::fs::rename(&temporary, &target));
        match written {
            Ok(()) => {
                done.insert(key.clone(), print);
                copied.push(name);
            }
            Err(err) => println!("NV_SAVE_OUT: {} not copied: {err}", target.display()),
        }
    }
    copied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_players_folders_are_refused_and_scratch_folders_taken() {
        let profile = Path::new(r"C:\Users\<name>");
        for dir in [
            r"C:\Users\<name>\Documents\My Games\FalloutNV\Saves",
            r"c:/users/<name>/onedrive/documents/x",
            r"C:\Users\<name>\AppData\Local\FalloutNV",
            r"C:\Users\<name>\Documents",
            "relative\\saves",
            r"C:\Users\<name>\nv-re\..\Documents\My Games",
        ] {
            assert!(
                refused(Path::new(dir), Some(profile), &[]).is_some(),
                "{dir}"
            );
        }
        assert!(refused(Path::new(r"D:\scratch"), None, &[]).is_some());
        for dir in [
            r"C:\Users\<name>\nv-re\work\build43-scratch\docs-port\My Games\FalloutNV\Saves",
            r"C:\Users\<name>\DocumentsScratch",
            r"C:\Users\<name>\OneDrive\Desktop\nv-rs\userdata\Documents\My Games\FalloutNV\Saves",
            r"D:\scratch",
        ] {
            assert_eq!(refused(Path::new(dir), Some(profile), &[]), None, "{dir}");
        }
    }

    /// Documents moved away from the profile (OneDrive, another drive) is still the player's
    /// Documents, and so is a junction that leads into it.
    #[test]
    fn a_redirected_documents_and_a_junction_into_it_are_refused() {
        let profile = Path::new(r"C:\Users\<name>");
        let moved = vec![PathBuf::from(r"D:\Moved\Documents")];
        let saves = Path::new(r"D:\Moved\Documents\My Games\FalloutNV\Saves");
        assert!(refused(saves, Some(profile), &moved).is_some());
        assert!(refused(Path::new("d:/moved/documents"), Some(profile), &moved).is_some());
        for fine in [r"D:\Moved\DocumentsScratch", r"D:\Moved\Other"] {
            assert_eq!(
                refused(Path::new(fine), Some(profile), &moved),
                None,
                "{fine}"
            );
        }
        #[cfg(windows)]
        {
            let base =
                std::env::temp_dir().join(format!("nv-save-junction-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            let documents = base.join("Moved Documents");
            std::fs::create_dir_all(&documents).unwrap();
            let link = base.join("innocent");
            let made = std::process::Command::new("cmd")
                .args(["/c", "mklink", "/J"])
                .arg(&link)
                .arg(&documents)
                .output()
                .unwrap();
            assert!(made.status.success(), "{made:?}");
            let known = vec![documents.clone()];
            assert!(refused(&link.join("Saves"), Some(profile), &known).is_some());
            assert!(refused(&documents.join("Saves"), Some(profile), &known).is_some());
            assert_eq!(
                refused(&base.join("elsewhere"), Some(profile), &known),
                None
            );
            let _ = std::fs::remove_dir_all(&base);
        }
    }

    #[test]
    fn new_and_changed_saves_are_copied_once_each() {
        let dir = std::env::temp_dir().join(format!("nv-save-out-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut e = Engine::new();
        let saves = br"c:\scratch\my games\falloutnv\saves\";
        let key = |name: &str| [saves.as_slice(), name.as_bytes()].concat();
        engine_platform::state_mut(&mut e.kernel.platform)
            .disk
            .files
            .insert(key("quicksave.fos"), b"FO3SAVEGAME one".to_vec());
        engine_platform::state_mut(&mut e.kernel.platform)
            .disk
            .files
            .insert(key("falloutprefs.ini"), b"x".to_vec());
        let mut done = HashMap::new();
        assert_eq!(
            copy_changed(&e, &dir, &mut done),
            vec!["quicksave.fos".to_string()]
        );
        assert_eq!(
            std::fs::read(dir.join("quicksave.fos")).unwrap(),
            b"FO3SAVEGAME one"
        );
        assert!(!dir.join("falloutprefs.ini").exists());
        assert!(!dir.join("quicksave.fos.part").exists());
        assert!(copy_changed(&e, &dir, &mut done).is_empty());
        engine_platform::state_mut(&mut e.kernel.platform)
            .disk
            .files
            .insert(key("quicksave.fos"), b"FO3SAVEGAME two".to_vec());
        assert_eq!(
            copy_changed(&e, &dir, &mut done),
            vec!["quicksave.fos".to_string()]
        );
        assert_eq!(
            std::fs::read(dir.join("quicksave.fos")).unwrap(),
            b"FO3SAVEGAME two"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_save_the_game_deleted_goes_and_its_settings_are_copied() {
        let dir = std::env::temp_dir().join(format!("nv-save-del-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for name in ["old.fos", "made again.fos", "kept.fos"] {
            std::fs::write(dir.join(name), b"x").unwrap();
        }
        let mut e = Engine::new();
        let saves = br"c:\scratch\my games\falloutnv\";
        let key = |name: &str| [saves.as_slice(), name.as_bytes()].concat();
        let disk = &mut engine_platform::state_mut(&mut e.kernel.platform).disk;
        disk.deleted.insert(key(r"saves\old.fos"));
        disk.deleted.insert(key(r"saves\made again.fos"));
        disk.files.insert(key(r"saves\made again.fos"), vec![1]);
        disk.deleted.insert(key(r"saves\note.txt"));
        disk.files.insert(key("falloutprefs.ini"), b"one".to_vec());
        assert_eq!(remove_deleted(&e, &dir), vec!["old.fos (deleted)"]);
        assert!(!dir.join("old.fos").exists());
        assert!(dir.join("made again.fos").exists());
        assert!(dir.join("kept.fos").exists());
        let prefs = dir.join("FalloutPrefs.ini");
        let mut done = None;
        prefs_changed(&e, &prefs, &mut done);
        assert_eq!(std::fs::read(&prefs).unwrap(), b"one");
        std::fs::remove_file(&prefs).unwrap();
        prefs_changed(&e, &prefs, &mut done);
        assert!(!prefs.exists(), "the same bytes are not copied again");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
