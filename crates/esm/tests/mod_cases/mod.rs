//! The mod setups of the plugin tests (`tests/mods.rs`, [`cases`]): a temporary Data
//! folder and the plugin record pieces. Nothing from the game is needed. (From the removed
//! `testdata` crate, 2026-11-03.)

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

pub mod cases;

pub fn sub(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u16).to_le_bytes());
    v.extend(data);
    v
}

pub fn zstr(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}

pub fn group(label: [u8; 4], kind: i32, contents: &[u8]) -> Vec<u8> {
    let mut v = b"GRUP".to_vec();
    v.extend((24 + contents.len() as u32).to_le_bytes());
    v.extend(label);
    v.extend(kind.to_le_bytes());
    v.extend([0; 8]);
    v.extend(contents);
    v
}

/// A temporary folder, removed when dropped.
pub struct TempData(PathBuf);

impl Drop for TempData {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

impl TempData {
    fn new(tag: &str) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let dir = std::env::temp_dir().join(format!(
                "nv-rs-{tag}-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&dir) {
                Ok(()) => return Self(dir),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot create test fixture {dir:?}: {e}"),
            }
        }
    }

    /// An empty fixture folder, for a test that writes its own plugin.
    pub fn empty(tag: &str) -> Self {
        Self::new(tag)
    }

    pub fn write(&self, relative: &str, bytes: &[u8]) {
        // Game records use Windows separators; make fixture paths portable.
        let path = self.0.join(relative.replace('\\', "/"));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}
