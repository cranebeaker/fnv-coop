//! Starting the viewer and keeping its output in a launcher log.

use crate::command::{self, Spec};
use crate::redact::redact;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Unix seconds now.
pub fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// `20251029-010600`-style stamp for file names (UTC).
pub fn stamp(secs: u64) -> String {
    crate::party::format_time(secs).replace(['-', ':'], "").replace(' ', "-")
}

/// Runs the viewer to completion with stdout and stderr going to `log`. Returns the exit code
/// (None when it was ended by a signal).
pub fn run_viewer(spec: &Spec, log: &Path, user: Option<&str>) -> Result<Option<i32>, String> {
    if !spec.program.is_file() {
        return Err(format!("The viewer was not found at {}. Extract the whole package and keep nv-viewer.exe where it was.", spec.program.display()));
    }
    fs::create_dir_all(&spec.cwd).map_err(|e| format!("could not make the save folder: {e}"))?;
    if let Some(d) = log.parent() {
        fs::create_dir_all(d).map_err(|e| format!("could not make the log folder: {e}"))?;
    }
    let mut f = File::create(log).map_err(|e| format!("could not create the log: {e}"))?;
    let _ = writeln!(f, "nv-launcher {} starting the viewer at {}\n{}", crate::VERSION, crate::party::format_time(now_secs()), redact(&command::render(spec), user));
    let out = f.try_clone().map_err(|e| e.to_string())?;
    let err = f.try_clone().map_err(|e| e.to_string())?;
    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args).current_dir(&spec.cwd).stdout(Stdio::from(out)).stderr(Stdio::from(err)).stdin(Stdio::null());
    for (k, v) in &spec.env {
        cmd.env(k, v);
    }
    let status = cmd.status().map_err(|e| format!("could not start the viewer: {e}"))?;
    let _ = writeln!(f, "\nviewer exited: {status}");
    Ok(status.code())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamp_shape() {
        assert_eq!(stamp(0), "19700101-0000");
    }

    #[test]
    fn missing_viewer_is_a_clear_error() {
        let spec = Spec { program: "Z:\\nope\\nv-viewer.exe".into(), args: vec![], env: vec![], cwd: std::env::temp_dir() };
        let e = run_viewer(&spec, &std::env::temp_dir().join("nvl-never.log"), None).unwrap_err();
        assert!(e.contains("not found"));
    }
}
