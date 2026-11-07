//! "Play with mods": the game reads a whole install made in `userdata\modgame`, never the player's
//! own folder. Every file of the game is a hard link there (no copy, same drive, only ever read);
//! the files in the package's `mods` folder are copied over them, each link removed first, so
//! nothing can be written through to the game's own file. Removing that folder only removes names.
//! The load order goes to the package's own `plugins.txt`.
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The official masters a mod may name, in the game's load order.
const OFFICIAL: [&str; 9] = [
    "DeadMoney.esm",
    "HonestHearts.esm",
    "OldWorldBlues.esm",
    "LonesomeRoad.esm",
    "GunRunnersArsenal.esm",
    "ClassicPack.esm",
    "MercenaryPack.esm",
    "TribalPack.esm",
    "CaravanPack.esm",
];

/// The masters a plugin's header names (`MAST` fields of its `TES4` record).
pub fn masters(file: &[u8]) -> Vec<String> {
    if file.get(..4) != Some(b"TES4") || file.len() < 24 {
        return vec![];
    }
    let size = u32::from_le_bytes([file[4], file[5], file[6], file[7]]) as usize;
    let end = (24 + size).min(file.len());
    let mut at = 24;
    let mut out = Vec::new();
    while at + 6 <= end {
        let length = u16::from_le_bytes([file[at + 4], file[at + 5]]) as usize;
        if &file[at..at + 4] == b"MAST" {
            if let Some(name) = file.get(at + 6..at + 6 + length) {
                let name = name.split(|&b| b == 0).next().unwrap_or(name);
                out.push(String::from_utf8_lossy(name).into_owned());
            }
        }
        at += 6 + length;
    }
    out
}

/// The plugin list for these mod plugins: the official masters they name, their `.esm`, their
/// `.esp` (the game orders the load by file date; this only says which are on).
pub fn plugin_list(plugins: &[(String, Vec<u8>)]) -> String {
    let needed: BTreeSet<String> = plugins
        .iter()
        .flat_map(|(_, header)| masters(header))
        .map(|m| m.to_ascii_lowercase())
        .collect();
    let mut names: Vec<&str> = OFFICIAL
        .iter()
        .copied()
        .filter(|m| needed.contains(&m.to_ascii_lowercase()))
        .collect();
    let by_ext = |ext: &str| -> Vec<&str> {
        let mut v: Vec<&str> = plugins
            .iter()
            .map(|(n, _)| n.as_str())
            .filter(|n| n.to_ascii_lowercase().ends_with(ext))
            .collect();
        v.sort_by_key(|n| n.to_ascii_lowercase());
        v
    };
    names.extend(by_ext(".esm"));
    names.extend(by_ext(".esp"));
    let mut out = String::new();
    for name in names {
        out.push_str(name);
        out.push_str("\r\n");
    }
    out
}

fn link_tree(from: &Path, to: &Path) -> std::io::Result<u64> {
    std::fs::create_dir_all(to)?;
    let mut count = 0;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            count += link_tree(&entry.path(), &target)?;
        } else if kind.is_file() {
            std::fs::hard_link(entry.path(), &target)?;
            count += 1;
        }
    }
    Ok(count)
}

/// Files under `folder`, relative to it, without the folder's `READ ME` and `plugins.txt`.
fn mod_files(folder: &Path, relative: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(folder.join(relative))? {
        let entry = entry?;
        let name = entry.file_name();
        let path = relative.join(&name);
        if entry.file_type()?.is_dir() {
            mod_files(folder, &path, out)?;
        } else {
            let text = name.to_string_lossy().to_ascii_lowercase();
            let top = relative.as_os_str().is_empty();
            if !(top && (text == "plugins.txt" || text.starts_with("read me"))) {
                out.push(path);
            }
        }
    }
    Ok(())
}

/// What building the install found.
#[derive(Debug, Default, PartialEq)]
pub struct Built {
    pub linked: u64,
    pub mod_files: usize,
    /// A script extender's plugin folder or a DLL among the mods: not used.
    pub extender_plugins: bool,
}

/// What stopped the install being made.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Failure {
    /// Hard links across two drives (`ERROR_NOT_SAME_DEVICE`).
    OtherDrive,
    /// A drive that has no hard links (FAT, exFAT, a network share).
    NoLinks,
    /// Another program has the file open (sharing or lock violation).
    Locked,
    /// Windows refused access: a read-only file or folder, rights, an antivirus.
    Denied,
    /// No room on the drive.
    Full,
    Other,
}

/// A failed install: which step and path, and what that means.
#[derive(Debug)]
pub struct ModError {
    pub kind: Failure,
    pub step: String,
}

impl ModError {
    /// What the player reads: the step that failed, then what to do about that kind of failure.
    pub fn player_text(&self) -> String {
        let advice = match self.kind {
            Failure::OtherDrive => "For mods nv-rs makes links to the game's files, never copies, and a link can't reach another drive. Move the nv-rs folder to the drive the game is on, or play without mods.",
            Failure::NoLinks => "For mods nv-rs makes links to the game's files, never copies, and this drive can't hold links: it needs a normal Windows (NTFS) drive. Move the nv-rs folder to an NTFS drive that the game is on, or play without mods.",
            Failure::Locked => "Another program has this file open: the game or its launcher still running, a mod manager, or an antivirus scanning it. Close it, wait a moment and try again.",
            Failure::Denied => "Windows did not allow it: the file or folder may be read-only, or your account lacks the rights, or an antivirus or controlled folder access blocks it. Clear \"Read-only\" in its Properties, or allow Play.exe, and try again.",
            Failure::Full => "There is no room left on the drive. Free some space, or play without mods: the links take little room, but the mods' files are copied.",
            Failure::Other => "Please report this if it keeps happening.",
        };
        format!(
            "nv-rs could not make its own copy of the game's folder for mods.\n\nWhat failed: {}\n\n{advice}",
            self.step
        )
    }
}

/// The kind of failure for a Windows error code, for `linking` a file or any other step.
pub fn classify(code: Option<i32>, linking: bool) -> Failure {
    match code {
        Some(17) => Failure::OtherDrive,
        Some(1 | 50) if linking => Failure::NoLinks,
        Some(32 | 33) => Failure::Locked,
        Some(5) => Failure::Denied,
        Some(39 | 112) => Failure::Full,
        _ => Failure::Other,
    }
}

/// Makes `install` (removed first) from the game's folder and `mods`, and writes the plugin list
/// to `plugins_txt`: `mods\plugins.txt` as it is when there is one, else [`plugin_list`].
pub fn build_install(
    game: &Path,
    mods: &Path,
    install: &Path,
    plugins_txt: &Path,
) -> Result<Built, ModError> {
    let describe = |what: &str, path: &Path, e: std::io::Error| ModError {
        kind: classify(e.raw_os_error(), what == "linking"),
        step: format!("{what} {}: {e}", path.display()),
    };
    if install.exists() {
        std::fs::remove_dir_all(install).map_err(|e| describe("removing", install, e))?;
    }
    std::fs::create_dir_all(install).map_err(|e| describe("making", install, e))?;
    let mut built = Built::default();
    for entry in std::fs::read_dir(game).map_err(|e| describe("reading", game, e))? {
        let entry = entry.map_err(|e| describe("reading", game, e))?;
        let kind = entry
            .file_type()
            .map_err(|e| describe("reading", &entry.path(), e))?;
        if kind.is_file() {
            std::fs::hard_link(entry.path(), install.join(entry.file_name()))
                .map_err(|e| describe("linking", &entry.path(), e))?;
            built.linked += 1;
        }
    }
    built.linked += link_tree(&game.join("Data"), &install.join("Data"))
        .map_err(|e| describe("linking", &game.join("Data"), e))?;
    let mut files = Vec::new();
    std::fs::create_dir_all(mods).map_err(|e| describe("making", mods, e))?;
    mod_files(mods, Path::new(""), &mut files).map_err(|e| describe("reading", mods, e))?;
    let data = install.join("Data");
    let mut plugins = Vec::new();
    for relative in &files {
        let source = mods.join(relative);
        let target = data.join(relative);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| describe("making", parent, e))?;
        }
        // The link goes first: a copy over it would write into the game's own file.
        if target.exists() {
            std::fs::remove_file(&target).map_err(|e| describe("replacing", &target, e))?;
        }
        std::fs::copy(&source, &target).map_err(|e| describe("copying", &source, e))?;
        let text = relative.to_string_lossy().to_ascii_lowercase();
        if text.contains("nvse\\") || text.ends_with(".dll") {
            built.extender_plugins = true;
        }
        let top = relative.parent().is_some_and(|p| p.as_os_str().is_empty());
        if top && (text.ends_with(".esm") || text.ends_with(".esp")) {
            let header = std::fs::read(&source).map_err(|e| describe("reading", &source, e))?;
            plugins.push((relative.to_string_lossy().into_owned(), header));
        }
    }
    built.mod_files = files.len();
    let list = match std::fs::read(mods.join("plugins.txt")) {
        Ok(own) => own,
        Err(_) => plugin_list(&plugins).into_bytes(),
    };
    if let Some(parent) = plugins_txt.parent() {
        std::fs::create_dir_all(parent).map_err(|e| describe("making", parent, e))?;
    }
    std::fs::write(plugins_txt, list).map_err(|e| describe("writing", plugins_txt, e))?;
    Ok(built)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugin(names: &[&str]) -> Vec<u8> {
        let mut fields = Vec::new();
        fields.extend_from_slice(b"HEDR\x0c\0");
        fields.extend_from_slice(&[0; 12]);
        for name in names {
            fields.extend_from_slice(b"MAST");
            fields.extend_from_slice(&(name.len() as u16 + 1).to_le_bytes());
            fields.extend_from_slice(name.as_bytes());
            fields.push(0);
            fields.extend_from_slice(b"DATA\x08\0");
            fields.extend_from_slice(&[0; 8]);
        }
        let mut file = b"TES4".to_vec();
        file.extend_from_slice(&(fields.len() as u32).to_le_bytes());
        file.extend_from_slice(&[0; 16]);
        file.extend_from_slice(&fields);
        file
    }

    #[test]
    fn a_plugins_masters_are_read() {
        assert_eq!(
            masters(&plugin(&["FalloutNV.esm", "DeadMoney.esm"])),
            ["FalloutNV.esm", "DeadMoney.esm"]
        );
        assert!(masters(b"not a plugin").is_empty());
    }

    #[test]
    fn the_list_has_named_masters_then_esm_then_esp() {
        let list = plugin_list(&[
            (
                "b.esp".into(),
                plugin(&["FalloutNV.esm", "lonesomeroad.esm"]),
            ),
            ("A.ESP".into(), plugin(&["FalloutNV.esm"])),
            ("Own.esm".into(), plugin(&["DeadMoney.esm"])),
        ]);
        assert_eq!(
            list,
            "DeadMoney.esm\r\nLonesomeRoad.esm\r\nOwn.esm\r\nA.ESP\r\nb.esp\r\n"
        );
    }

    /// The game's own files are links, a mod's file replaces its link, and neither the build nor
    /// removing the install changes the game's files.
    #[test]
    fn the_game_folder_is_never_written() {
        let dir = std::env::temp_dir().join(format!("nv-rs-mods-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (game, mods) = (dir.join("game"), dir.join("mods"));
        std::fs::create_dir_all(game.join("Data\\Textures")).unwrap();
        std::fs::create_dir_all(mods.join("textures")).unwrap();
        std::fs::write(game.join("FalloutNV.exe"), b"exe").unwrap();
        std::fs::write(game.join("Data\\FalloutNV.esm"), b"esm").unwrap();
        std::fs::write(game.join("Data\\Textures\\a.dds"), b"game's").unwrap();
        std::fs::write(mods.join("textures\\a.dds"), b"mod's").unwrap();
        std::fs::write(
            mods.join("Mine.esp"),
            plugin(&["FalloutNV.esm", "HonestHearts.esm"]),
        )
        .unwrap();
        std::fs::write(mods.join("READ ME.txt"), b"not a mod").unwrap();
        let install = dir.join("install");
        let plugins = dir.join("p\\plugins.txt");
        let built = build_install(&game, &mods, &install, &plugins).unwrap();
        assert_eq!(
            (built.linked, built.mod_files, built.extender_plugins),
            (3, 2, false)
        );
        assert_eq!(
            std::fs::read(install.join("Data\\Textures\\a.dds")).unwrap(),
            b"mod's"
        );
        assert_eq!(
            std::fs::read(install.join("Data\\FalloutNV.esm")).unwrap(),
            b"esm"
        );
        assert!(!install.join("Data\\READ ME.txt").exists());
        assert_eq!(
            std::fs::read_to_string(&plugins).unwrap(),
            "HonestHearts.esm\r\nMine.esp\r\n"
        );
        assert_eq!(
            std::fs::read(game.join("Data\\Textures\\a.dds")).unwrap(),
            b"game's"
        );
        // Again over the first one, then gone.
        build_install(&game, &mods, &install, &plugins).unwrap();
        std::fs::remove_dir_all(&install).unwrap();
        assert_eq!(
            std::fs::read(game.join("Data\\FalloutNV.esm")).unwrap(),
            b"esm"
        );
        assert_eq!(
            std::fs::read(game.join("Data\\Textures\\a.dds")).unwrap(),
            b"game's"
        );
        std::fs::write(mods.join("plugins.txt"), b"Own order.esp\r\n").unwrap();
        std::fs::write(mods.join("nvse.dll"), b"x").unwrap();
        let built = build_install(&game, &mods, &install, &plugins).unwrap();
        assert!(built.extender_plugins);
        assert_eq!(std::fs::read(&plugins).unwrap(), b"Own order.esp\r\n");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn each_failure_gets_its_own_words() {
        assert_eq!(classify(Some(17), true), Failure::OtherDrive);
        assert_eq!(classify(Some(1), true), Failure::NoLinks);
        assert_eq!(classify(Some(1), false), Failure::Other);
        assert_eq!(classify(Some(32), false), Failure::Locked);
        assert_eq!(classify(Some(33), false), Failure::Locked);
        assert_eq!(classify(Some(5), false), Failure::Denied);
        assert_eq!(classify(Some(112), false), Failure::Full);
        let text = |kind| {
            ModError {
                kind,
                step: "linking X: boom".into(),
            }
            .player_text()
        };
        assert!(text(Failure::OtherDrive).contains("another drive"));
        assert!(text(Failure::NoLinks).contains("NTFS"));
        assert!(text(Failure::Locked).contains("open"));
        assert!(text(Failure::Denied).contains("read-only"));
        assert!(!text(Failure::Locked).contains("same drive"));
        assert!(text(Failure::Locked).contains("linking X: boom"));
    }

    /// A file of the old install held open by another program: the error says locked, not drive.
    #[test]
    fn a_locked_file_is_reported_as_locked() {
        use std::os::windows::fs::OpenOptionsExt;
        let dir = std::env::temp_dir().join(format!("nv-rs-mods-lock-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (game, mods, install) = (dir.join("game"), dir.join("mods"), dir.join("install"));
        std::fs::create_dir_all(game.join("Data")).unwrap();
        std::fs::write(game.join("FalloutNV.exe"), b"exe").unwrap();
        std::fs::write(game.join("Data\\FalloutNV.esm"), b"esm").unwrap();
        let plugins = dir.join("plugins.txt");
        build_install(&game, &mods, &install, &plugins).unwrap();
        let held = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(install.join("FalloutNV.exe"))
            .unwrap();
        let error = build_install(&game, &mods, &install, &plugins).unwrap_err();
        drop(held);
        assert_eq!(error.kind, Failure::Locked, "{}", error.step);
        assert!(error.step.starts_with("removing "));
        assert_eq!(std::fs::read(game.join("FalloutNV.exe")).unwrap(), b"exe");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
