//! Starting the viewer and keeping its output in a launcher log.

use crate::command::{self, Spec};
use crate::redact::redact;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;

/// Unix seconds now.
pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// `20251029-010600`-style stamp for file names (UTC).
pub fn stamp(secs: u64) -> String {
    crate::party::format_time(secs)
        .replace(['-', ':'], "")
        .replace(' ', "-")
}

/// Whether a viewer output line is worth showing in the launcher's window: co-op notices, join
/// code, the opening's menus, errors. The log has every line.
pub fn interesting(line: &str) -> bool {
    let l = line.to_ascii_lowercase();
    line.starts_with("Co-op:")
        || l.contains("coop_join_code")
        || l.contains("join code")
        || l.contains("error")
        || l.contains("panicked")
        || l.contains("--answer-boxes")
        || l.starts_with("tag skills")
        || l.starts_with("trait")
        || l.starts_with("vit-o-matic")
}

fn pump(pipe: impl Read + Send + 'static, tx: mpsc::Sender<String>) {
    std::thread::spawn(move || {
        let mut r = BufReader::new(pipe);
        let mut buf = Vec::new();
        while r.read_until(b'\n', &mut buf).unwrap_or(0) > 0 {
            if tx
                .send(String::from_utf8_lossy(&buf).trim_end().to_string())
                .is_err()
            {
                break;
            }
            buf.clear();
        }
    });
}

/// Runs the viewer to completion. Its stdout and stderr go to `log` (redacted) as they come and
/// each line is also handed to `on_line` (the launcher shows the interesting ones). Returns the
/// exit code (None when it was ended by a signal).
pub fn run_viewer(
    spec: &Spec,
    log: &Path,
    user: Option<&str>,
    on_line: &mut dyn FnMut(&str),
) -> Result<Option<i32>, String> {
    if !spec.program.is_file() {
        return Err(format!("The viewer was not found at {}. Extract the whole package and keep nv-viewer.exe where it was.", spec.program.display()));
    }
    fs::create_dir_all(&spec.cwd).map_err(|e| format!("could not make the save folder: {e}"))?;
    if let Some(d) = log.parent() {
        fs::create_dir_all(d).map_err(|e| format!("could not make the log folder: {e}"))?;
    }
    let mut f = File::create(log).map_err(|e| format!("could not create the log: {e}"))?;
    let _ = writeln!(
        f,
        "nv-launcher {} starting the viewer at {}\n{}",
        crate::VERSION,
        crate::party::format_time(now_secs()),
        redact(&command::render(spec), user)
    );
    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args)
        .current_dir(&spec.cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());
    for (k, v) in &spec.env {
        cmd.env(k, v);
    }
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("could not start the viewer: {e}"))?;
    let (tx, rx) = mpsc::channel::<String>();
    if let Some(o) = child.stdout.take() {
        pump(o, tx.clone());
    }
    if let Some(e) = child.stderr.take() {
        pump(e, tx.clone());
    }
    drop(tx);
    for line in rx {
        let line = redact(&line, user);
        let _ = writeln!(f, "{line}");
        on_line(&line);
    }
    let status = child
        .wait()
        .map_err(|e| format!("could not wait for the viewer: {e}"))?;
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
        let spec = Spec {
            program: "Z:\\nope\\nv-viewer.exe".into(),
            args: vec![],
            env: vec![],
            cwd: std::env::temp_dir(),
        };
        let e = run_viewer(
            &spec,
            &std::env::temp_dir().join("nvl-never.log"),
            None,
            &mut |_| {},
        )
        .unwrap_err();
        assert!(e.contains("not found"));
    }

    #[test]
    fn interesting_lines() {
        assert!(interesting("Co-op: joined; loading the host's place"));
        assert!(interesting("--answer-boxes: the name entry accepted."));
        assert!(interesting("thread 'main' panicked at x"));
        assert!(!interesting("Loaded GSDocMitchellHouse in 3.1 s"));
    }

    #[cfg(windows)]
    #[test]
    fn output_is_tailed_logged_and_the_exit_code_returned() {
        let sh = std::env::var("COMSPEC").unwrap_or_else(|_| r"C:\Windows\System32\cmd.exe".into());
        let dir = std::env::temp_dir().join(format!("nvl-run-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let spec = Spec {
            program: sh.into(),
            args: vec![
                "/C".into(),
                "echo Co-op: hello & echo to-stderr 1>&2 & exit 3".into(),
            ],
            env: vec![],
            cwd: dir.join("cwd"),
        };
        let mut seen = Vec::new();
        let code = run_viewer(&spec, &dir.join("l").join("v.log"), None, &mut |l| {
            seen.push(l.to_string())
        })
        .unwrap();
        assert_eq!(code, Some(3));
        assert!(
            seen.iter().any(|l| l.starts_with("Co-op: hello")),
            "{seen:?}"
        );
        assert!(seen.iter().any(|l| l.contains("to-stderr")), "{seen:?}");
        let log = fs::read_to_string(dir.join("l").join("v.log")).unwrap();
        assert!(
            log.contains("Co-op: hello") && log.contains("viewer exited"),
            "{log}"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
