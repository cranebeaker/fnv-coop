//! Finding the player's Fallout: New Vegas installation (read only).
//!
//! Where it looks: the Steam registry entries and `libraryfolders.vdf` (every library), GOG and
//! retail registry entries, common folders on each drive, and a path the player types. Nothing is
//! ever written, copied or modified in the game folder.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const GAME_EXE: &str = "FalloutNV.exe";
/// The only game build the engine is translated from.
pub const SUPPORTED: [u16; 4] = [1, 4, 0, 525];
const STEAM_NAMES: [&str; 2] = ["Fallout New Vegas", "Fallout New Vegas PCR"];
const STEAM_APPIDS: [&str; 2] = ["22380", "22490"];

/// Read-only registry access; tests pass a fake.
pub trait Registry {
    fn get(&self, hive: &str, key: &str, value: &str) -> Option<String>;
}

/// The real registry, through `reg query` (no extra crates).
pub struct RegExe;

impl Registry for RegExe {
    fn get(&self, hive: &str, key: &str, value: &str) -> Option<String> {
        if !cfg!(windows) {
            return None;
        }
        for view in ["/reg:64", "/reg:32"] {
            let out = std::process::Command::new("reg")
                .args(["query", &format!("{hive}\\{key}"), "/v", value, view])
                .output()
                .ok()?;
            if out.status.success() {
                if let Some(v) = parse_reg_query(&String::from_utf8_lossy(&out.stdout), value) {
                    return Some(v);
                }
            }
        }
        None
    }
}

/// The data of `value` in `reg query` output (`    Name    REG_SZ    data`).
pub fn parse_reg_query(text: &str, value: &str) -> Option<String> {
    for line in text.lines() {
        let t = line.trim();
        if t.len() > value.len() && t[..value.len()].eq_ignore_ascii_case(value) {
            let rest = t[value.len()..].trim_start();
            for ty in ["REG_SZ", "REG_EXPAND_SZ"] {
                if let Some(d) = rest.strip_prefix(ty) {
                    let d = d.trim();
                    if !d.is_empty() {
                        return Some(d.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Library paths from `libraryfolders.vdf` (new format `"path"`, old format `"1"`).
pub fn parse_libraryfolders(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let q: Vec<&str> = line.split('"').collect();
        // `  "path"  "D:\\Lib"` splits as ["  ", "path", "  ", "D:\\Lib", ""].
        if q.len() >= 4 {
            let key = q[1];
            if key == "path" || (!key.is_empty() && key.chars().all(|c| c.is_ascii_digit())) {
                let p = q[3].replace("\\\\", "\\");
                let b = p.as_bytes();
                let drive = b.len() >= 3
                    && b[0].is_ascii_alphabetic()
                    && b[1] == b':'
                    && (b[2] == b'\\' || b[2] == b'/');
                if drive || p.starts_with("\\\\") {
                    out.push(p);
                }
            }
        }
    }
    out
}

/// `installdir` from an `appmanifest_*.acf`.
pub fn parse_installdir(text: &str) -> Option<String> {
    for line in text.lines() {
        let q: Vec<&str> = line.split('"').collect();
        if q.len() >= 4 && q[1] == "installdir" && !q[3].is_empty() {
            return Some(q[3].to_string());
        }
    }
    None
}

/// Reads a Steam text file: UTF-8 (BOM allowed), else lossy.
pub fn read_text(p: &Path) -> String {
    match std::fs::read(p) {
        Ok(b) => {
            let b = b.strip_prefix(&[0xEF, 0xBB, 0xBF][..]).unwrap_or(&b);
            String::from_utf8_lossy(b).into_owned()
        }
        Err(_) => String::new(),
    }
}

/// A game folder that holds `FalloutNV.exe`.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub game_dir: PathBuf,
    /// "steam", "gog", "retail", "common" or "typed".
    pub source: &'static str,
}

fn norm(p: &Path) -> String {
    p.to_string_lossy()
        .replace('/', "\\")
        .trim_end_matches('\\')
        .to_ascii_lowercase()
}

/// Accepts the game folder, its `Data` folder or the exe, and gives the game folder.
pub fn normalize_game_dir(p: &Path) -> PathBuf {
    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if name == GAME_EXE.to_ascii_lowercase() || name == "data" {
        if let Some(parent) = p.parent() {
            return parent.to_path_buf();
        }
    }
    p.to_path_buf()
}

/// Every install found, best guess first, no duplicates. `drives` are roots like `C:\`.
pub fn candidates(
    reg: &dyn Registry,
    env: &HashMap<String, String>,
    drives: &[PathBuf],
) -> Vec<Candidate> {
    let mut found: Vec<Candidate> = Vec::new();
    let mut add = |p: PathBuf, source: &'static str| {
        if p.join(GAME_EXE).is_file() && !found.iter().any(|c| norm(&c.game_dir) == norm(&p)) {
            found.push(Candidate {
                game_dir: p,
                source,
            });
        }
    };
    let mut roots: Vec<PathBuf> = Vec::new();
    for (hive, key, val) in [
        ("HKCU", r"Software\Valve\Steam", "SteamPath"),
        ("HKLM", r"SOFTWARE\WOW6432Node\Valve\Steam", "InstallPath"),
        ("HKLM", r"SOFTWARE\Valve\Steam", "InstallPath"),
    ] {
        if let Some(v) = reg.get(hive, key, val) {
            roots.push(PathBuf::from(v.replace('/', "\\")));
        }
    }
    for k in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Some(pf) = env.get(k) {
            roots.push(PathBuf::from(pf).join("Steam"));
        }
    }
    let mut libs: Vec<PathBuf> = Vec::new();
    for r in &roots {
        libs.push(r.clone());
        for l in parse_libraryfolders(&read_text(&r.join("steamapps").join("libraryfolders.vdf"))) {
            libs.push(PathBuf::from(l));
        }
    }
    for lib in &libs {
        let common = lib.join("steamapps").join("common");
        for id in STEAM_APPIDS {
            if let Some(d) = parse_installdir(&read_text(
                &lib.join("steamapps").join(format!("appmanifest_{id}.acf")),
            )) {
                add(common.join(d), "steam");
            }
        }
        for n in STEAM_NAMES {
            add(common.join(n), "steam");
        }
    }
    for id in ["1454587428", "1454315831"] {
        if let Some(v) = reg.get(
            "HKLM",
            &format!(r"SOFTWARE\WOW6432Node\GOG.com\Games\{id}"),
            "path",
        ) {
            add(PathBuf::from(v), "gog");
        }
    }
    for key in [
        r"SOFTWARE\WOW6432Node\Bethesda Softworks\FalloutNV",
        r"SOFTWARE\Bethesda Softworks\FalloutNV",
    ] {
        if let Some(v) = reg.get("HKLM", key, "Installed Path") {
            add(PathBuf::from(v), "retail");
        }
    }
    for d in drives {
        for rel in [
            r"Program Files (x86)\Steam\steamapps\common",
            r"Program Files\Steam\steamapps\common",
            r"Steam\steamapps\common",
            r"SteamLibrary\steamapps\common",
            r"Games\Steam\steamapps\common",
            r"Games\SteamLibrary\steamapps\common",
        ] {
            for n in STEAM_NAMES {
                add(d.join(rel).join(n), "common");
            }
        }
        for rel in [
            r"GOG Games\Fallout New Vegas",
            r"Games\Fallout New Vegas",
            r"Games\GOG Games\Fallout New Vegas",
            r"Fallout New Vegas",
        ] {
            add(d.join(rel), "common");
        }
    }
    found
}

/// The file version in a PE image's version resource (`VS_FIXEDFILEINFO`), if it has one.
pub fn exe_version(bytes: &[u8]) -> Option<[u16; 4]> {
    const SIG: [u8; 4] = [0xBD, 0x04, 0xEF, 0xFE];
    let mut at = 0;
    while let Some(p) = bytes[at..].windows(4).position(|w| w == SIG) {
        let i = at + p;
        if let Some(f) = bytes.get(i + 8..i + 16) {
            let ms = u32::from_le_bytes([f[0], f[1], f[2], f[3]]);
            let ls = u32::from_le_bytes([f[4], f[5], f[6], f[7]]);
            // A real struct has a plausible major version; skip chance matches.
            if ms >> 16 < 100 {
                return Some([(ms >> 16) as u16, ms as u16, (ls >> 16) as u16, ls as u16]);
            }
        }
        at = i + 1;
    }
    None
}

/// The result of looking at a game folder.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub game_dir: PathBuf,
    pub data_dir: PathBuf,
    pub has_data: bool,
    pub has_esm: bool,
    pub has_exe: bool,
    pub version: Option<[u16; 4]>,
    /// Plain-language findings, one per line; empty when everything is fine.
    pub problems: Vec<String>,
}

impl Report {
    pub fn ok(&self) -> bool {
        self.problems.is_empty()
    }
}

pub fn version_text(v: [u16; 4]) -> String {
    format!("{}.{}.{}.{}", v[0], v[1], v[2], v[3])
}

/// Checks a game folder: `Data` with `FalloutNV.esm`, and `FalloutNV.exe` at version 1.4.0.525.
pub fn check(game_dir: &Path) -> Report {
    let data_dir = game_dir.join("Data");
    let exe = game_dir.join(GAME_EXE);
    let has_data = data_dir.is_dir();
    let has_esm = data_dir.join("FalloutNV.esm").is_file();
    let has_exe = exe.is_file();
    let mut problems = Vec::new();
    if !has_data {
        problems.push("There is no Data folder here.".to_string());
    } else if !has_esm {
        problems.push(
            "The Data folder has no FalloutNV.esm, so this is not a New Vegas install.".to_string(),
        );
    }
    let mut version = None;
    if !has_exe {
        problems.push(format!(
            "{GAME_EXE} was not found, so the game version cannot be checked."
        ));
    } else {
        match std::fs::read(&exe) {
            Err(e) => problems.push(format!("Could not read {GAME_EXE}: {e}")),
            Ok(b) => {
                version = exe_version(&b);
                match version {
                    None => problems.push(format!("{GAME_EXE} has no readable version. The co-op build expects 1.4.0.525.")),
                    Some(v) if v != SUPPORTED => problems.push(format!(
                        "{GAME_EXE} is version {}, but the co-op build needs 1.4.0.525. Update or repair the game and try again.",
                        version_text(v)
                    )),
                    Some(_) => {}
                }
            }
        }
    }
    Report {
        game_dir: game_dir.to_path_buf(),
        data_dir,
        has_data,
        has_esm,
        has_exe,
        version,
        problems,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake(HashMap<(String, String), String>);
    impl Registry for Fake {
        fn get(&self, h: &str, k: &str, v: &str) -> Option<String> {
            let _ = h;
            self.0.get(&(k.to_string(), v.to_string())).cloned()
        }
    }

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("nvl-install-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn fake_exe(version: [u16; 4]) -> Vec<u8> {
        let mut b = vec![0x4D, 0x5A, 0, 0, 1, 2, 3];
        b.extend_from_slice(&[0xBD, 0x04, 0xEF, 0xFE, 0, 0, 1, 0]);
        let ms = (version[0] as u32) << 16 | version[1] as u32;
        let ls = (version[2] as u32) << 16 | version[3] as u32;
        b.extend_from_slice(&ms.to_le_bytes());
        b.extend_from_slice(&ls.to_le_bytes());
        b.extend_from_slice(&[9; 32]);
        b
    }

    fn make_game(root: &Path, version: [u16; 4]) {
        std::fs::create_dir_all(root.join("Data")).unwrap();
        std::fs::write(root.join("Data").join("FalloutNV.esm"), b"TES4").unwrap();
        std::fs::write(root.join(GAME_EXE), fake_exe(version)).unwrap();
    }

    #[test]
    fn vdf_new_and_old_formats() {
        let new = "\"libraryfolders\"\n{\n\t\"0\"\n\t{\n\t\t\"path\"\t\t\"C:\\\\Program Files (x86)\\\\Steam\"\n\t\t\"label\"\t\t\"\"\n\t}\n\t\"1\"\n\t{\n\t\t\"path\"\t\t\"E:\\\\Lib\"\n\t}\n}";
        assert_eq!(
            parse_libraryfolders(new),
            vec![r"C:\Program Files (x86)\Steam", r"E:\Lib"]
        );
        let old = "\"LibraryFolders\"\n{\n\t\"TimeNextStatsReport\"\t\"123\"\n\t\"1\"\t\t\"D:\\\\Old\"\n}";
        assert_eq!(parse_libraryfolders(old), vec![r"D:\Old"]);
    }

    #[test]
    fn manifest_installdir() {
        assert_eq!(
            parse_installdir("\"AppState\"\n{\n\t\"installdir\"\t\t\"Fallout New Vegas\"\n}")
                .as_deref(),
            Some("Fallout New Vegas")
        );
        assert_eq!(parse_installdir("nothing"), None);
    }

    #[test]
    fn reg_query_output() {
        let t = "\nHKEY_CURRENT_USER\\Software\\Valve\\Steam\n    SteamPath    REG_SZ    c:/program files (x86)/steam\n\n";
        assert_eq!(
            parse_reg_query(t, "SteamPath").as_deref(),
            Some("c:/program files (x86)/steam")
        );
        assert_eq!(parse_reg_query(t, "Other"), None);
    }

    #[test]
    fn finds_game_in_second_steam_library() {
        let root = tmp("lib");
        let steam = root.join("Steam");
        let lib2 = root.join("Lib2");
        std::fs::create_dir_all(steam.join("steamapps")).unwrap();
        let vdf = format!(
            "\"libraryfolders\"\n{{\n\t\"1\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t}}\n}}",
            lib2.to_string_lossy().replace('\\', "\\\\")
        );
        std::fs::write(steam.join("steamapps").join("libraryfolders.vdf"), vdf).unwrap();
        std::fs::create_dir_all(lib2.join("steamapps")).unwrap();
        std::fs::write(
            lib2.join("steamapps").join("appmanifest_22380.acf"),
            "\"installdir\"\t\"FNV Custom\"",
        )
        .unwrap();
        make_game(
            &lib2.join("steamapps").join("common").join("FNV Custom"),
            SUPPORTED,
        );
        let mut m = HashMap::new();
        m.insert(
            (r"Software\Valve\Steam".to_string(), "SteamPath".to_string()),
            steam.to_string_lossy().replace('\\', "/"),
        );
        let found = candidates(&Fake(m), &HashMap::new(), &[]);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].source, "steam");
        assert!(found[0].game_dir.ends_with("FNV Custom"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn finds_common_folder_and_dedups() {
        let root = tmp("common");
        make_game(&root.join("Games").join("Fallout New Vegas"), SUPPORTED);
        let found = candidates(
            &Fake(HashMap::new()),
            &HashMap::new(),
            &[root.clone(), root.clone()],
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].source, "common");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn check_good_wrong_and_broken() {
        let root = tmp("check");
        let good = root.join("good");
        make_game(&good, SUPPORTED);
        assert!(check(&good).ok());
        assert_eq!(check(&good).version, Some(SUPPORTED));
        let old = root.join("old");
        make_game(&old, [1, 3, 0, 452]);
        let r = check(&old);
        assert!(!r.ok());
        assert!(r.problems[0].contains("1.3.0.452"));
        let empty = root.join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        let r = check(&empty);
        assert!(!r.has_data && !r.has_exe);
        assert_eq!(r.problems.len(), 2);
        let noesm = root.join("noesm");
        std::fs::create_dir_all(noesm.join("Data")).unwrap();
        assert!(check(&noesm)
            .problems
            .iter()
            .any(|p| p.contains("FalloutNV.esm")));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn version_parsing_skips_noise() {
        assert_eq!(exe_version(b"no version here"), None);
        let mut b = vec![
            0xBD, 0x04, 0xEF, 0xFE, 0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0,
        ];
        b.extend(fake_exe([1, 4, 0, 525]));
        assert_eq!(exe_version(&b), Some([1, 4, 0, 525]));
        assert_eq!(exe_version(&[0xBD, 0x04, 0xEF, 0xFE]), None);
    }

    #[test]
    fn normalizes_typed_paths() {
        assert_eq!(
            normalize_game_dir(Path::new(r"D:\G\FNV\Data")),
            PathBuf::from(r"D:\G\FNV")
        );
        assert_eq!(
            normalize_game_dir(Path::new(r"D:\G\FNV\FalloutNV.exe")),
            PathBuf::from(r"D:\G\FNV")
        );
        assert_eq!(
            normalize_game_dir(Path::new(r"D:\G\FNV")),
            PathBuf::from(r"D:\G\FNV")
        );
    }
}
