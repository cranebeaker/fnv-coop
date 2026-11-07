//! Finding the player's game: Steam, GOG and Epic installs, read only. The parsers are plain
//! functions over text, so they are tested without those stores on this PC.
use crate::win;
use std::path::{Path, PathBuf};

/// Steam's app ids of Fallout: New Vegas (the main one and the one sold in some regions).
const STEAM_APPS: [u32; 2] = [22380, 22490];

/// The quoted strings of a Steam `.vdf` or `.acf` file, in order, `\\` read as `\`.
pub fn vdf_strings(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '"' {
            continue;
        }
        let mut s = String::new();
        while let Some(c) = chars.next() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(next) = chars.next() {
                        s.push(next);
                    }
                }
                other => s.push(other),
            }
        }
        out.push(s);
    }
    out
}

/// The value after each `key` in a `.vdf`: its `"key" "value"` pairs, any case.
pub fn vdf_values(text: &str, key: &str) -> Vec<String> {
    let strings = vdf_strings(text);
    strings
        .windows(2)
        .filter(|pair| pair[0].eq_ignore_ascii_case(key))
        .map(|pair| pair[1].clone())
        .collect()
}

/// One string field of an Epic `.item` manifest (JSON), `\\` read as `\`.
pub fn json_string(text: &str, field: &str) -> Option<String> {
    let at = text.find(&format!("\"{field}\""))?;
    let rest = &text[at + field.len() + 2..];
    let rest = &rest[rest.find(':')? + 1..];
    let rest = &rest[rest.find('"')? + 1..];
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => out.push(chars.next()?),
            other => out.push(other),
        }
    }
    None
}

/// Whether `folder` (the game's folder or its `Data`) holds a game; its `Data` folder.
pub fn data_folder(folder: &Path) -> Option<PathBuf> {
    [folder.to_path_buf(), folder.join("Data")]
        .into_iter()
        .find(|f| f.join("FalloutNV.esm").is_file())
}

fn steam_candidates() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    for (root, key, name) in [
        (win::HKLM, "SOFTWARE\\Valve\\Steam", "InstallPath"),
        (win::HKCU, "Software\\Valve\\Steam", "SteamPath"),
    ] {
        if let Some(path) = win::registry_string(root, key, name) {
            roots.push(PathBuf::from(path));
        }
    }
    let mut libraries: Vec<PathBuf> = Vec::new();
    for root in &roots {
        libraries.push(root.clone());
        if let Ok(text) = std::fs::read_to_string(root.join("steamapps\\libraryfolders.vdf")) {
            libraries.extend(vdf_values(&text, "path").into_iter().map(PathBuf::from));
        }
    }
    let mut out = Vec::new();
    for library in &libraries {
        let apps = library.join("steamapps");
        for app in STEAM_APPS {
            if let Ok(text) = std::fs::read_to_string(apps.join(format!("appmanifest_{app}.acf"))) {
                if let Some(dir) = vdf_values(&text, "installdir").into_iter().next() {
                    out.push(apps.join("common").join(dir));
                }
            }
        }
        out.push(apps.join("common").join("Fallout New Vegas"));
    }
    out
}

fn gog_candidates() -> Vec<PathBuf> {
    win::registry_scan(
        win::HKLM,
        "SOFTWARE\\GOG.com\\Games",
        "gameName",
        "new vegas",
        "path",
    )
    .into_iter()
    .map(PathBuf::from)
    .collect()
}

fn epic_candidates() -> Vec<PathBuf> {
    let Some(data) = std::env::var_os("ProgramData") else {
        return vec![];
    };
    let folder = PathBuf::from(data).join("Epic\\EpicGamesLauncher\\Data\\Manifests");
    let Ok(entries) = std::fs::read_dir(folder) else {
        return vec![];
    };
    entries
        .flatten()
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|x| x.eq_ignore_ascii_case("item"))
        })
        .filter_map(|e| {
            let text = std::fs::read_to_string(e.path()).ok()?;
            let title = json_string(&text, "DisplayName")?;
            title
                .to_ascii_lowercase()
                .contains("new vegas")
                .then(|| json_string(&text, "InstallLocation"))
                .flatten()
                .map(PathBuf::from)
        })
        .collect()
}

/// Every place the stores say the game is, in order, each with the store's name: the folders may
/// not exist. The registry and the stores' files are only read.
pub fn candidates() -> Vec<(&'static str, PathBuf)> {
    let mut out: Vec<(&'static str, PathBuf)> = Vec::new();
    if std::env::var_os("NV_RS_NO_STORES").is_some() {
        return out;
    }
    out.extend(steam_candidates().into_iter().map(|p| ("Steam", p)));
    out.extend(gog_candidates().into_iter().map(|p| ("GOG", p)));
    out.extend(epic_candidates().into_iter().map(|p| ("Epic", p)));
    if let Some(p) = win::registry_string(
        win::HKLM,
        "SOFTWARE\\Bethesda Softworks\\FalloutNV",
        "Installed Path",
    ) {
        out.push(("the game's installer", PathBuf::from(p)));
    }
    let mut seen = Vec::new();
    out.retain(|(_, p)| {
        let key = p.to_string_lossy().to_ascii_lowercase();
        let key = key.trim_end_matches('\\').to_string();
        if seen.contains(&key) {
            false
        } else {
            seen.push(key);
            true
        }
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam_library_folders_are_read() {
        let text = r#"
"libraryfolders"
{
	"0"
	{
		"path"		"C:\\Program Files (x86)\\Steam"
		"apps" { "22380" "123" }
	}
	"1"
	{
		"path"		"D:\\Games \"Steam\""
	}
}"#;
        assert_eq!(
            vdf_values(text, "path"),
            ["C:\\Program Files (x86)\\Steam", "D:\\Games \"Steam\""]
        );
    }

    #[test]
    fn a_steam_manifest_names_the_install_folder() {
        let text = "\"AppState\"\n{\n\t\"appid\"\t\t\"22380\"\n\t\"installdir\"\t\t\"Fallout New Vegas\"\n}\n";
        assert_eq!(vdf_values(text, "InstallDir"), ["Fallout New Vegas"]);
    }

    #[test]
    fn an_epic_manifest_is_read() {
        let text = r#"{ "FormatVersion": 0, "DisplayName": "Fallout: New Vegas - Ultimate Edition",
            "InstallLocation": "C:\\Program Files\\Epic Games\\FalloutNV", "x": 1 }"#;
        assert_eq!(
            json_string(text, "DisplayName").unwrap(),
            "Fallout: New Vegas - Ultimate Edition"
        );
        assert_eq!(
            json_string(text, "InstallLocation").unwrap(),
            "C:\\Program Files\\Epic Games\\FalloutNV"
        );
        assert_eq!(json_string(text, "Nothing"), None);
    }

    #[test]
    fn the_data_folder_is_found_from_either_folder() {
        let dir = std::env::temp_dir().join(format!("nv-rs-find-{}", std::process::id()));
        let data = dir.join("Data");
        std::fs::create_dir_all(&data).unwrap();
        std::fs::write(data.join("FalloutNV.esm"), b"x").unwrap();
        assert_eq!(data_folder(&dir).unwrap(), data);
        assert_eq!(data_folder(&data).unwrap(), data);
        assert_eq!(data_folder(&dir.join("missing")), None);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
