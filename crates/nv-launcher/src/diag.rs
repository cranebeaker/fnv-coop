//! Diagnostics: a plain folder of redacted logs a player can share, and the `--selftest`.

use crate::redact::redact;
use crate::settings::Settings;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_LOGS: usize = 12;
const MAX_BYTES: usize = 2 * 1024 * 1024;

/// Copies the newest launcher logs and the settings into `out`, redacting the Windows user name,
/// and writes `info.txt` with `info`. Returns the files written (names only).
pub fn collect(
    home: &Path,
    out: &Path,
    user: Option<&str>,
    info: &str,
) -> Result<Vec<String>, String> {
    fs::create_dir_all(out).map_err(|e| format!("could not make {}: {e}", out.display()))?;
    let mut written = Vec::new();
    let mut put = |name: &str, text: &str| -> Result<(), String> {
        fs::write(out.join(name), redact(text, user))
            .map_err(|e| format!("could not write {name}: {e}"))?;
        written.push(name.to_string());
        Ok(())
    };
    put("info.txt", info)?;
    if let Ok(t) = fs::read(home.join("settings.json")) {
        put("settings.json", &String::from_utf8_lossy(&t))?;
    }
    let mut logs: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();
    if let Ok(rd) = fs::read_dir(home.join("logs")) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() {
                logs.push((
                    e.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::UNIX_EPOCH),
                    p,
                ));
            }
        }
    }
    logs.sort_by(|a, b| b.0.cmp(&a.0));
    for (_, p) in logs.into_iter().take(MAX_LOGS) {
        let Ok(bytes) = fs::read(&p) else { continue };
        let tail = if bytes.len() > MAX_BYTES {
            &bytes[bytes.len() - MAX_BYTES..]
        } else {
            &bytes[..]
        };
        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        put(&format!("log-{name}"), &String::from_utf8_lossy(tail))?;
    }
    Ok(written)
}

/// One self-test result.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: &'static str,
    pub ok: bool,
    pub detail: String,
}

/// The launcher's own checks. Uses a scratch folder under `scratch` and removes it again.
pub fn selftest(scratch: &Path, exe_dir: &Path) -> Vec<Item> {
    let mut v = Vec::new();
    let mut add = |name: &'static str, ok: bool, detail: String| v.push(Item { name, ok, detail });
    let dir = scratch.join(format!("selftest-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);

    let s = Settings {
        player_name: "Tester".into(),
        port: 31337,
        ..Settings::default()
    };
    let path = dir.join("settings.json");
    let r = s.save(&path).map(|_| Settings::load(&path).0 == s);
    add("settings file", matches!(r, Ok(true)), format!("{r:?}"));

    let code = crate::joincode::host_code("192.168.1.2", 27961, [1; 16]).unwrap_or_default();
    let back = crate::joincode::parse_join(&code);
    add("join code", back.is_ok(), code);

    add(
        "redaction",
        redact(r"C:\Users\someone\x", None) == r"C:\Users\<user>\x",
        "user folder replaced".into(),
    );

    let c = crate::portcheck::loopback_test(0u16.max(40000 + (std::process::id() % 2000) as u16));
    add("local UDP round trip", c.ok, c.detail);

    let saves = dir.join("saves");
    let id = crate::party::new_campaign_id([5; 8]);
    let r = crate::party::ensure_player_dir(&saves, &id, crate::party::Role::Host, "Tester", 0);
    add(
        "save folders",
        r.is_ok(),
        format!("{r:?}").replace(&dir.to_string_lossy().to_string(), "<scratch>"),
    );

    let viewer = crate::cli::find_viewer(None, exe_dir);
    add(
        "viewer present",
        viewer.is_file(),
        format!("looked for {}", viewer.display()),
    );

    let _ = fs::remove_dir_all(&dir);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_redacted_files() {
        let home = std::env::temp_dir().join(format!("nvl-diag-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(home.join("logs")).unwrap();
        fs::write(
            home.join("settings.json"),
            r#"{"data_path": "C:\\Users\\bobsmith\\Games"}"#,
        )
        .unwrap();
        fs::write(
            home.join("logs").join("launcher-1.log"),
            "ran as bobsmith in C:\\Users\\bobsmith\\AppData",
        )
        .unwrap();
        let out = home.join("diag");
        let files = collect(&home, &out, Some("bobsmith"), "data C:\\Users\\bobsmith\\x").unwrap();
        assert!(files.contains(&"log-launcher-1.log".to_string()));
        for f in &files {
            let t = fs::read_to_string(out.join(f)).unwrap();
            assert!(!t.to_ascii_lowercase().contains("bobsmith"), "{f}: {t}");
        }
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn selftest_runs() {
        let items = selftest(&std::env::temp_dir(), Path::new("Z:\\none"));
        let bad: Vec<_> = items
            .iter()
            .filter(|i| !i.ok && i.name != "viewer present")
            .collect();
        assert!(bad.is_empty(), "{bad:?}");
        assert!(items.iter().any(|i| i.name == "viewer present" && !i.ok));
    }
}
