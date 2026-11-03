//! Whether the Steam friends transport can be offered.
//!
//! It needs both the user-supplied (or shipped, unmodified) `steam_api64.dll` next to the
//! launcher or viewer, and a viewer that lists `--coop-steam` in its `--help`. The viewer lists
//! the flag in every build, but only a build with the `coop-steam` feature acts on it; any other
//! build hosts on UDP and logs a `transport_fallback` event, so passing the flag is always safe.
//! Without the DLL the launcher says why and uses UDP direct.

use std::path::{Path, PathBuf};

pub const DLL: &str = "steam_api64.dll";
pub const VIEWER_FLAG: &str = "--coop-steam";

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Available,
    Unavailable(String),
}

impl Status {
    pub fn available(&self) -> bool {
        *self == Status::Available
    }
}

/// The DLL in any of `dirs`.
pub fn find_dll(dirs: &[PathBuf]) -> Option<PathBuf> {
    dirs.iter().map(|d| d.join(DLL)).find(|p| p.is_file())
}

/// Pure decision from what was found.
pub fn status(dll: Option<&Path>, viewer_help: Option<&str>) -> Status {
    if dll.is_none() {
        return Status::Unavailable(format!(
            "{DLL} was not found next to the launcher (it is optional; see THIRD-PARTY.txt)."
        ));
    }
    match viewer_help {
        Some(h) if h.contains(VIEWER_FLAG) => Status::Available,
        Some(_) => Status::Unavailable(
            "this viewer build has no Steam transport yet; UDP direct is used.".into(),
        ),
        None => Status::Unavailable(
            "the viewer was not found, so Steam support could not be checked.".into(),
        ),
    }
}

/// Runs `viewer --help` (which exits before opening a window) and returns its text.
pub fn viewer_help(viewer: &Path) -> Option<String> {
    let out = std::process::Command::new(viewer)
        .arg("--help")
        .output()
        .ok()?;
    let mut t = String::from_utf8_lossy(&out.stdout).into_owned();
    t.push_str(&String::from_utf8_lossy(&out.stderr));
    Some(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decisions() {
        let dll = PathBuf::from("steam_api64.dll");
        assert!(status(None, Some("--coop-steam")).available() == false);
        assert!(status(Some(&dll), Some("usage --coop-steam")).available());
        let s = status(Some(&dll), Some("usage --coop-host"));
        assert!(matches!(s, Status::Unavailable(ref m) if m.contains("no Steam transport")));
        assert!(!status(Some(&dll), None).available());
    }

    #[test]
    fn dll_search() {
        let d = std::env::temp_dir().join(format!("nvl-steam-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        assert!(find_dll(&[d.clone()]).is_none());
        std::fs::write(d.join(DLL), b"x").unwrap();
        assert_eq!(find_dll(&[d.clone()]), Some(d.join(DLL)));
        let _ = std::fs::remove_dir_all(&d);
    }
}
