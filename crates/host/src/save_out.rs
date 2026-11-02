//! `NV_SAVE_OUT=DIR`, a test aid for scripted runs: the save files the game wrote in this run
//! copied into `DIR`, so a later run can start from them through the main menu (CONTINUE or
//! LOAD) instead of replaying the opening.
//!
//! Files the game writes stay in engine memory (`engine::units::platform::files::Disk`); nothing
//! reaches the disk unless this is asked for. Only files whose name ends in `.fos` are copied,
//! each under its own name (the run's names are lower case). A file is written to a temporary
//! name first and then renamed, so a reader never sees half of it. A copy is made again whenever
//! the game's bytes change. A save still being written may be copied part-way; it is copied
//! again, complete, at the next check.
//!
//! `DIR` must be a scratch folder. One inside the player's Documents, OneDrive or AppData is
//! refused, and nothing is written.

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
        if let Some(why) = refused(&dir, profile.as_deref()) {
            println!(
                "NV_SAVE_OUT={}: refused ({why}); saves are not copied.",
                dir.display()
            );
            return None;
        }
        Some(dir)
    })
    .as_deref()
}

/// Why `dir` may not receive saves: it lies inside one of the player's own folders under
/// `profile` (Documents, OneDrive, AppData). Compared without case, `/` read as `\`.
pub fn refused(dir: &Path, profile: Option<&Path>) -> Option<&'static str> {
    let norm = |p: &Path| p.to_string_lossy().replace('/', "\\").to_ascii_lowercase();
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
    let profile = profile.trim_end_matches('\\');
    for player in ["documents", "onedrive", "appdata"] {
        let root = format!("{profile}\\{player}");
        if dir == root || dir.starts_with(&format!("{root}\\")) {
            return Some("inside the player's own folders");
        }
    }
    None
}

/// Copies the run's `.fos` files into `NV_SAVE_OUT` where they are new or changed since the last
/// call; the names copied, for the log.
pub fn copy_saves(e: &Engine) -> Vec<String> {
    let Some(dir) = out_dir() else {
        return vec![];
    };
    static DONE: Mutex<Option<HashMap<Vec<u8>, u64>>> = Mutex::new(None);
    let mut done = DONE.lock().unwrap_or_else(|p| p.into_inner());
    copy_changed(e, dir, done.get_or_insert_with(HashMap::new))
}

/// [`copy_saves`] into `dir`, with `done` the fingerprints of the bytes copied so far by key.
pub fn copy_changed(e: &Engine, dir: &Path, done: &mut HashMap<Vec<u8>, u64>) -> Vec<String> {
    use std::hash::{Hash, Hasher};
    let mut copied = Vec::new();
    for (key, data) in &e.kernel.disk.files {
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
            assert!(refused(Path::new(dir), Some(profile)).is_some(), "{dir}");
        }
        assert!(refused(Path::new(r"D:\scratch"), None).is_some());
        for dir in [
            r"C:\Users\<name>\nv-re\work\build43-scratch\docs-port\My Games\FalloutNV\Saves",
            r"C:\Users\<name>\DocumentsScratch",
            r"D:\scratch",
        ] {
            assert_eq!(refused(Path::new(dir), Some(profile)), None, "{dir}");
        }
    }

    #[test]
    fn new_and_changed_saves_are_copied_once_each() {
        let dir = std::env::temp_dir().join(format!("nv-save-out-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut e = Engine::new();
        let saves = br"c:\scratch\my games\falloutnv\saves\";
        let key = |name: &str| [saves.as_slice(), name.as_bytes()].concat();
        e.kernel
            .disk
            .files
            .insert(key("quicksave.fos"), b"FO3SAVEGAME one".to_vec());
        e.kernel
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
        e.kernel
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
}
