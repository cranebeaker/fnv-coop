//! The settings file, `settings.json` in the launcher's home (`%APPDATA%\nv-rs-coop`).

use crate::json::{self, Value};
use std::path::{Path, PathBuf};

pub const DEFAULT_PORT: u16 = 27961;

/// Preferred transport. Steam is used only when it is available, else UDP direct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Udp,
    Steam,
}

impl Transport {
    pub fn name(self) -> &'static str {
        match self {
            Transport::Udp => "udp",
            Transport::Steam => "steam",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// The game's Data folder (empty: not set up yet).
    pub data_path: String,
    pub player_name: String,
    pub port: u16,
    pub transport: Transport,
    pub windowed: bool,
    pub width: u32,
    pub height: u32,
    /// Sets `NV_UNCAP=1` for the viewer.
    pub uncap: bool,
    /// The address shown in join codes (your LAN or public IP), empty until asked.
    pub host_address: String,
    /// `nv-viewer.exe`, when it is not next to the launcher (empty: look next to the launcher).
    pub viewer_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            data_path: String::new(),
            player_name: "Player".into(),
            port: DEFAULT_PORT,
            transport: Transport::Udp,
            windowed: false,
            width: 1280,
            height: 720,
            uncap: false,
            host_address: String::new(),
            viewer_path: String::new(),
        }
    }
}

/// A player name the viewer and the save folders can take: printable ASCII, no quotes or path
/// characters, at most 24 characters. Empty becomes "Player".
pub fn clean_name(raw: &str) -> String {
    let s: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '_' | '-' | '.'))
        .collect();
    let s: String = s.trim().chars().take(24).collect();
    let s = s.trim().trim_matches('.').to_string();
    if s.is_empty() {
        "Player".into()
    } else {
        s
    }
}

impl Settings {
    pub fn from_json(v: &Value) -> Settings {
        let d = Settings::default();
        let port = v
            .get("port")
            .and_then(Value::as_f64)
            .filter(|p| p.fract() == 0.0 && *p >= 1.0 && *p <= 65535.0)
            .map(|p| p as u16)
            .unwrap_or(d.port);
        let dim = |k: &str, def: u32| {
            v.get(k)
                .and_then(Value::as_f64)
                .filter(|n| n.fract() == 0.0 && (320.0..=16384.0).contains(n))
                .map(|n| n as u32)
                .unwrap_or(def)
        };
        Settings {
            data_path: v
                .get("data_path")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_string(),
            player_name: v
                .get("player_name")
                .and_then(Value::as_str)
                .map(clean_name)
                .unwrap_or(d.player_name),
            port,
            transport: match v.get("transport").and_then(Value::as_str) {
                Some(t) if t.eq_ignore_ascii_case("steam") => Transport::Steam,
                _ => Transport::Udp,
            },
            windowed: v
                .get("windowed")
                .and_then(Value::as_bool)
                .unwrap_or(d.windowed),
            width: dim("width", d.width),
            height: dim("height", d.height),
            uncap: v.get("uncap").and_then(Value::as_bool).unwrap_or(d.uncap),
            host_address: v
                .get("host_address")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_string(),
            viewer_path: v
                .get("viewer_path")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_string(),
        }
    }

    pub fn to_json(&self) -> Value {
        Value::Obj(vec![
            ("data_path".into(), Value::Str(self.data_path.clone())),
            ("player_name".into(), Value::Str(self.player_name.clone())),
            ("port".into(), Value::Num(self.port as f64)),
            ("transport".into(), Value::Str(self.transport.name().into())),
            ("windowed".into(), Value::Bool(self.windowed)),
            ("width".into(), Value::Num(self.width as f64)),
            ("height".into(), Value::Num(self.height as f64)),
            ("uncap".into(), Value::Bool(self.uncap)),
            ("host_address".into(), Value::Str(self.host_address.clone())),
            ("viewer_path".into(), Value::Str(self.viewer_path.clone())),
        ])
    }

    /// Loads `path`. A missing file gives the defaults; a damaged one gives the defaults and a warning.
    pub fn load(path: &Path) -> (Settings, Option<String>) {
        match std::fs::read(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Settings::default(), None),
            Err(e) => (
                Settings::default(),
                Some(format!("could not read settings: {e}")),
            ),
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let text = text.trim_start_matches('\u{feff}');
                match json::parse(text) {
                    Ok(v @ Value::Obj(_)) => (Settings::from_json(&v), None),
                    Ok(_) => (
                        Settings::default(),
                        Some("settings file is not a JSON object; using defaults".into()),
                    ),
                    Err(e) => (
                        Settings::default(),
                        Some(format!("settings file is damaged ({e}); using defaults")),
                    ),
                }
            }
        }
    }

    /// Saves to `path` (written to a temporary file and renamed, so a crash cannot truncate it).
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, json::to_string_pretty(&self.to_json()))?;
        std::fs::rename(&tmp, path)
    }
}

/// The launcher's home folder: `NV_LAUNCHER_HOME`, else `%APPDATA%\nv-rs-coop`, else `./nv-rs-coop`.
pub fn home_dir(env: &dyn Fn(&str) -> Option<String>) -> PathBuf {
    if let Some(h) = env("NV_LAUNCHER_HOME").filter(|h| !h.is_empty()) {
        return PathBuf::from(h);
    }
    if let Some(a) = env("APPDATA").filter(|h| !h.is_empty()) {
        return PathBuf::from(a).join("nv-rs-coop");
    }
    PathBuf::from("nv-rs-coop")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("nvl-settings-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn save_and_load_round_trip() {
        let d = tmp("rt");
        let s = Settings {
            data_path: r"D:\Games\FNV\Data".into(),
            player_name: "Ann".into(),
            port: 4000,
            transport: Transport::Steam,
            windowed: true,
            width: 1920,
            height: 1080,
            uncap: true,
            host_address: "203.0.113.9".into(),
            viewer_path: r"D:\nv\nv-viewer.exe".into(),
        };
        let p = d.join("settings.json");
        s.save(&p).unwrap();
        let (back, warn) = Settings::load(&p);
        assert_eq!(back, s);
        assert!(warn.is_none());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn missing_is_default_and_damaged_warns() {
        let d = tmp("bad");
        std::fs::create_dir_all(&d).unwrap();
        let (s, w) = Settings::load(&d.join("nope.json"));
        assert_eq!(s, Settings::default());
        assert!(w.is_none());
        std::fs::write(d.join("x.json"), "{ not json").unwrap();
        let (s, w) = Settings::load(&d.join("x.json"));
        assert_eq!(s, Settings::default());
        assert!(w.unwrap().contains("damaged"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn bad_values_fall_back() {
        let v = json::parse(r#"{"port": 70000, "width": 5, "player_name": "..\\x\"y", "transport": "pigeon", "uncap": "yes"}"#).unwrap();
        let s = Settings::from_json(&v);
        assert_eq!(s.port, DEFAULT_PORT);
        assert_eq!(s.width, 1280);
        assert_eq!(s.player_name, "xy");
        assert_eq!(s.transport, Transport::Udp);
        assert!(!s.uncap);
    }

    #[test]
    fn names_are_cleaned() {
        assert_eq!(clean_name("  "), "Player");
        assert_eq!(clean_name("A\"; rm -rf"), "A rm -rf");
        assert_eq!(clean_name(&"x".repeat(60)).len(), 24);
        assert_eq!(clean_name("..."), "Player");
    }

    #[test]
    fn home_prefers_override_then_appdata() {
        let e = |k: &str| match k {
            "NV_LAUNCHER_HOME" => Some("X:\\h".to_string()),
            "APPDATA" => Some("C:\\A".to_string()),
            _ => None,
        };
        assert_eq!(home_dir(&e), PathBuf::from("X:\\h"));
        let e2 = |k: &str| {
            if k == "APPDATA" {
                Some("C:\\A".to_string())
            } else {
                None
            }
        };
        assert_eq!(home_dir(&e2), PathBuf::from("C:\\A").join("nv-rs-coop"));
        assert_eq!(home_dir(&|_| None), PathBuf::from("nv-rs-coop"));
    }
}
