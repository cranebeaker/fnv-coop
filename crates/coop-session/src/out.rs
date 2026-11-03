//! The JSONL event log and the state dump file.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::json::{self, V};

/// Append-only JSON lines. Write failures are ignored on purpose: a full
/// disk must not stop a game.
pub struct LogFile {
    file: File,
}

impl LogFile {
    /// Opens (creating folders, truncating) the log.
    pub fn create(path: &Path) -> std::io::Result<LogFile> {
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            fs::create_dir_all(dir)?;
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(LogFile { file })
    }

    /// Writes one event: `{"t":seconds,"ev":"name",...}`.
    pub fn event(&mut self, t_ms: u64, ev: &str, fields: &[(&str, V)]) {
        let mut all: Vec<(&str, V)> = vec![("t", V::F(t_ms as f64 / 1000.0)), ("ev", V::S(ev))];
        all.extend_from_slice(fields);
        let _ = writeln!(self.file, "{}", json::object(&all));
        let _ = self.file.flush();
    }
}

/// Writes a file atomically (temp file, then rename).
pub fn write_atomic(path: &Path, text: &str) {
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        let _ = fs::create_dir_all(dir);
    }
    let tmp = path.with_extension("tmp");
    if fs::write(&tmp, text).is_ok() {
        let _ = fs::rename(&tmp, path);
    }
}

/// Writes the field dump (whole file replaced each time).
pub struct DumpFile {
    path: PathBuf,
}

impl DumpFile {
    /// A dump target.
    pub fn new(path: PathBuf) -> DumpFile {
        DumpFile { path }
    }

    /// Replaces the file: a header object, then one line per shared field
    /// (`FORM Field{..}`), so two dumps diff line by line.
    pub fn write(&self, header: &[(&str, V)], fields: &[String]) {
        if let Some(dir) = self.path.parent().filter(|d| !d.as_os_str().is_empty()) {
            let _ = fs::create_dir_all(dir);
        }
        let mut text = json::object(header);
        text.push('\n');
        for f in fields {
            text.push_str(f);
            text.push('\n');
        }
        let tmp = self.path.with_extension("tmp");
        if fs::write(&tmp, text).is_ok() {
            let _ = fs::rename(&tmp, &self.path);
        }
    }
}
