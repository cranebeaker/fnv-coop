//! nv-rs's own player options in the run's `Fallout.ini`: the four `[nv-rs ...]` sections
//! (the maintainer approved the design of measure/ini/DESIGN.md, 2026-11-09).
//!
//! The file is the one the game itself reads: `My Games\FalloutNV\Fallout.ini` in the Documents
//! folder of the run's adapter table ([`path`]). The original keys keep the original's meaning;
//! nothing here reads them. Our keys are looked up by name the way the game's
//! `GetPrivateProfileStringA` finds a key (`engine_platform::ini`): names compared without case,
//! spaces around names and values trimmed, one pair of quotes taken off, the first match wins. A
//! `;` line is a note (its key would start with `;`), and any key we don't ask for is never looked
//! at, as in the original. A value we can't use falls back to the default and is named in one log
//! line ([`Keys::bad`]); it never stops the start.
//!
//! Precedence, lowest to highest (`crate::fixes::Choice::read`): the built-in default, the
//! preset, these keys, then the `NV_*` environment variables. Only a launcher's play run that is
//! no test reads them (`crate::fixes::is_play_run`); every scripted, gated or measured run never
//! does.

use crate::fixes::{Preset, FIXES};
use std::path::{Path, PathBuf};

/// The table of our keys: section, key, environment variable, the generated file's value, and
/// the comment line above it (`scripts/game-ini-sections.sh` writes the file from it).
pub const TABLE: &str = include_str!("../data/game_ini.tsv");

/// The section of the fix rows: each key turns one row of `crate::fixes::FIXES` on or off.
pub const FIXES_SECTION: &str = "nv-rs Fixes";

/// One row of [`TABLE`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub section: &'static str,
    pub key: &'static str,
    /// The environment variable that wins over the key (tests and scripts).
    pub switch: &'static str,
    /// The value the generated file writes (for a fix row: the line it writes commented out).
    pub default: &'static str,
    /// The one `;` line written above the key.
    pub comment: &'static str,
}

/// The rows of [`TABLE`], in order. `#` lines and empty lines are not rows.
pub fn rows() -> Vec<Row> {
    TABLE
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|line| {
            let f: Vec<&'static str> = line.split('\t').collect();
            assert_eq!(f.len(), 5, "game_ini.tsv: {line}");
            Row {
                section: f[0],
                key: f[1],
                switch: f[2],
                default: f[3],
                comment: f[4],
            }
        })
        .collect()
}

/// The value of `key` in `section` of the INI `text`, as the game's `GetPrivateProfileStringA`
/// finds it (`engine_platform::ini`'s reader): every section of that name is looked in, names
/// compared without case, spaces around names and values trimmed, one pair of surrounding quotes
/// taken off; the first match wins. A `;` line never matches.
pub fn value<'a>(text: &'a str, section: &str, key: &str) -> Option<&'a str> {
    let mut inside = false;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('[') {
            let name = rest.split(']').next().unwrap_or(rest);
            inside = name.trim().eq_ignore_ascii_case(section);
            continue;
        }
        if !inside {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case(key) {
            let value = value.trim();
            let value = value
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .unwrap_or(value);
            return Some(value);
        }
    }
    None
}

/// What the `[nv-rs ...]` sections set: `None` where a key is missing, empty or not usable.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Keys {
    pub preset: Option<Preset>,
    pub borderless: Option<bool>,
    pub frame_limit: Option<u32>,
    pub menu_frame_limit: Option<u32>,
    pub stepped_threads: Option<bool>,
    pub render_thread: Option<bool>,
    /// The fix rows the file turns on or off, by `crate::fixes::Fix::env`.
    pub fixes: Vec<(&'static str, bool)>,
    /// The lines whose value can't be used (`key=value`): the default stands for each.
    pub bad: Vec<String>,
}

impl Keys {
    /// Our keys in `text` (a whole `Fallout.ini`).
    pub fn read(text: &str) -> Keys {
        let mut keys = Keys::default();
        for row in rows() {
            let Some(raw) = value(text, row.section, row.key) else {
                continue;
            };
            // An empty value is the default, as the launcher reads an empty settings.ini line.
            if raw.is_empty() {
                continue;
            }
            let used = match row.key {
                "sPreset" => {
                    keys.preset = Preset::parse(raw);
                    keys.preset.is_some()
                }
                "bBorderless" => {
                    keys.borderless = flag(raw);
                    keys.borderless.is_some()
                }
                "iFrameLimit" => {
                    keys.frame_limit = raw.parse().ok();
                    keys.frame_limit.is_some()
                }
                "iMenuFrameLimit" => {
                    keys.menu_frame_limit = raw.parse().ok();
                    keys.menu_frame_limit.is_some()
                }
                "bSteppedThreads" => {
                    keys.stepped_threads = flag(raw);
                    keys.stepped_threads.is_some()
                }
                "bRenderThread" => {
                    keys.render_thread = flag(raw);
                    keys.render_thread.is_some()
                }
                _ => {
                    let fix = FIXES.iter().find(|f| f.env == row.switch);
                    match (fix, flag(raw)) {
                        (Some(fix), Some(on)) => {
                            keys.fixes.push((fix.env, on));
                            true
                        }
                        _ => false,
                    }
                }
            };
            if !used {
                keys.bad.push(format!("{}={raw}", row.key));
            }
        }
        keys
    }

    /// The one log line for values that can't be used, if any.
    pub fn bad_line(&self) -> Option<String> {
        (!self.bad.is_empty()).then(|| {
            format!(
                "Fallout.ini's nv-rs settings: not used, the default instead: {}",
                self.bad.join(", ")
            )
        })
    }
}

/// `1` or `0`.
fn flag(raw: &str) -> Option<bool> {
    match raw {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

/// The run's `Fallout.ini`: `My Games\FalloutNV\Fallout.ini` in the Documents folder the run's
/// adapter table gives the game (`crate::adapter_table_path`), where the game reads it.
/// A folder whose bytes can't be read is named once in the log ([`path_from_table`]).
pub fn path() -> Option<PathBuf> {
    let table = std::fs::read_to_string(crate::adapter_table_path()?).ok()?;
    path_from_table(&table).unwrap_or_else(|why| {
        eprintln!("{why}");
        None
    })
}

/// `CSIDL_PERSONAL` (5), the Documents folder, the game's `SHGetFolderPathA` asks for.
const DOCUMENTS: u32 = 5;

/// [`path`] from the adapter table's text: its last `SHGetFolderPathA` line for the Documents
/// folder (`engine_platform::d3d9`'s reader keeps the last). The folder's bytes are read as the
/// game's file layer turns its paths back to Windows, as UTF-8 (`engine_platform` files.rs; the
/// launcher writes plain, 8.3 or UTF-8 names for that reason). Bytes that aren't UTF-8 are a
/// folder the game can't open either: an error naming it, for the log.
pub fn path_from_table(table: &str) -> Result<Option<PathBuf>, String> {
    let Some(path) = documents_bytes(table) else {
        return Ok(None);
    };
    let documents = String::from_utf8(path).map_err(|e| {
        format!(
            "Fallout.ini's nv-rs settings not read: the adapter table's Documents folder is not \
             UTF-8 text, as the game's file layer reads its paths ({})",
            String::from_utf8_lossy(e.as_bytes())
        )
    })?;
    Ok(Some(
        Path::new(&documents)
            .join("My Games")
            .join("FalloutNV")
            .join("Fallout.ini"),
    ))
}

/// The Documents folder's bytes from the adapter table, up to the first zero.
fn documents_bytes(table: &str) -> Option<Vec<u8>> {
    let folder = table
        .lines()
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 4 || f[0] != "SHGetFolderPathA" {
                return None;
            }
            let csidl = f[1].split([' ', ',']).find(|w| !w.is_empty())?;
            (u32::from_str_radix(csidl, 16).ok()? == DOCUMENTS).then_some(f[3].trim())
        })
        .next_back()?;
    let bytes: Vec<u8> = (0..folder.len() / 2)
        .map(|i| u8::from_str_radix(folder.get(2 * i..2 * i + 2)?, 16).ok())
        .collect::<Option<_>>()?;
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    (end != 0).then(|| bytes[..end].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One row per fix (in docs/FIXES.md's order), each with its own variable; the player
    /// settings before them. Every key has a comment, and every key carries the original's type
    /// prefix.
    #[test]
    fn the_table_has_every_fix_and_a_comment_per_key() {
        let rows = rows();
        let mut fixes: Vec<&str> = rows
            .iter()
            .filter(|r| r.section == FIXES_SECTION)
            .map(|r| r.switch)
            .collect();
        let mut envs: Vec<&str> = FIXES.iter().map(|f| f.env).collect();
        fixes.sort_unstable();
        envs.sort_unstable();
        assert_eq!(fixes, envs);
        let mut seen = std::collections::BTreeSet::new();
        for row in &rows {
            assert!(row.section.starts_with("nv-rs "), "{row:?}");
            assert!(!row.section.contains(':'), "{row:?}");
            assert!(!row.comment.trim().is_empty(), "{row:?}");
            assert!(row.key.starts_with(['b', 'i', 's']), "{row:?}");
            assert!(seen.insert(row.key.to_ascii_lowercase()), "{row:?}");
            if row.key.starts_with('b') {
                assert!(flag(row.default).is_some(), "{row:?}");
            }
        }
        assert_eq!(
            rows.iter()
                .filter(|r| r.section != FIXES_SECTION)
                .map(|r| r.key)
                .collect::<Vec<_>>(),
            [
                "sPreset",
                "bBorderless",
                "iFrameLimit",
                "iMenuFrameLimit",
                "bSteppedThreads",
                "bRenderThread"
            ]
        );
    }

    /// The table's values are the built-in defaults, so the generated file changes nothing.
    #[test]
    fn the_tables_values_are_the_built_in_defaults() {
        let keys = Keys::read(&generated());
        assert_eq!(keys.preset, Some(Preset::Fixes));
        assert_eq!(keys.borderless, Some(false));
        assert_eq!(keys.frame_limit, Some(0));
        assert_eq!(
            keys.menu_frame_limit,
            Some(crate::fixes::DEFAULT_MENU_FRAME_LIMIT)
        );
        assert_eq!(
            keys.stepped_threads,
            Some(crate::fixes::PLAY_THREADS == "stepped")
        );
        assert_eq!(keys.render_thread, Some(crate::fixes::PLAY_RENDER_THREAD));
        // The fix lines are commented out: the preset decides.
        assert!(keys.fixes.is_empty());
        assert!(keys.bad.is_empty(), "{:?}", keys.bad);
        for row in rows().iter().filter(|r| r.section == FIXES_SECTION) {
            assert_eq!(row.default, "1", "{row:?}");
        }
    }

    /// The generated sections (`distribution/nv-rs-sections.ini`, written by
    /// `scripts/game-ini-sections.sh`) hold exactly the table: each key once, with its comment
    /// on the line above it, the fix rows commented out, nothing else.
    #[test]
    fn the_generated_sections_are_the_table() {
        let text = generated();
        let lines: Vec<&str> = text.lines().collect();
        let mut keys = Vec::new();
        let mut section = "";
        for (i, line) in lines.iter().enumerate() {
            // Notes start with "; "; a fix row's line with ";" and its key.
            if line.is_empty() || line.starts_with("; ") {
                continue;
            }
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                section = name;
                continue;
            }
            let Some((key, value)) = line.trim_start_matches(';').split_once('=') else {
                continue;
            };
            let row = rows()
                .into_iter()
                .find(|r| r.key == key)
                .unwrap_or_else(|| panic!("line {}: {line} is not in the table", i + 1));
            assert_eq!(section, row.section, "{line}");
            assert_eq!(value, row.default, "{line}");
            assert_eq!(
                line.starts_with(';'),
                row.section == FIXES_SECTION,
                "{line}"
            );
            assert_eq!(
                lines[i - 1],
                format!("; {}", row.comment),
                "the comment above {key}"
            );
            keys.push(key);
        }
        assert_eq!(keys, rows().iter().map(|r| r.key).collect::<Vec<_>>());
    }

    fn generated() -> String {
        let bytes = include_bytes!("../../../distribution/nv-rs-sections.ini");
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[test]
    fn values_are_found_as_the_games_reader_finds_them() {
        let text = "[nv-rs Display]\n;iFrameLimit=30\n  IFRAMELIMIT = \"60\" \niFrameLimit=90\n\
                    [General]\niFrameLimit=10\n[ NV-RS display ]\nbBorderless=1\n";
        assert_eq!(value(text, "nv-rs Display", "iFrameLimit"), Some("60"));
        assert_eq!(value(text, "nv-rs Display", "bBorderless"), Some("1"));
        assert_eq!(value(text, "General", "iFrameLimit"), Some("10"));
        assert_eq!(value(text, "nv-rs Display", ";iFrameLimit"), Some("30"));
        assert_eq!(value(text, "nv-rs Threads", "iFrameLimit"), None);
    }

    /// Unknown keys, original keys and notes are passed over; a value we can't use leaves the
    /// default and is named once.
    #[test]
    fn unknown_keys_pass_and_bad_values_fall_back() {
        let text = "[nv-rs General]\nsPreset=Vanilla\nsSomethingNew=1\n\
                    [nv-rs Display]\niFrameLimit=fast\nbBorderless=yes\niMenuFrameLimit=\n\
                    [nv-rs Fixes]\n;bUncap=0\nbGrabPush=0\nbTopicIndex=2\n\
                    [General]\nbUseThreadedAI=1\n";
        let keys = Keys::read(text);
        assert_eq!(keys.preset, Some(Preset::Vanilla));
        assert_eq!(keys.frame_limit, None);
        assert_eq!(keys.borderless, None);
        assert_eq!(keys.menu_frame_limit, None);
        assert_eq!(keys.fixes, [("NV_GRAB_PUSH", false)]);
        assert_eq!(
            keys.bad_line().unwrap(),
            "Fallout.ini's nv-rs settings: not used, the default instead: \
             bBorderless=yes, iFrameLimit=fast, bTopicIndex=2"
        );
        assert_eq!(
            Keys::read("[General]\nsLanguage=ENGLISH\n"),
            Keys::default()
        );
        assert_eq!(Keys::default().bad_line(), None);
    }

    #[test]
    fn the_documents_folder_comes_from_the_adapter_table() {
        let hex = |s: &str| s.bytes().map(|b| format!("{b:02x}")).collect::<String>();
        let table = format!(
            "GetACP\t\t4e4\t\nSHGetFolderPathA\t1c 0\t0\t{}\nSHGetFolderPathA\t5 0\t0\t{}\n\
             SHGetFolderPathA\t5 0\t0\t{}0000\n",
            hex(r"C:\Local"),
            hex(r"C:\Old"),
            hex(r"D:\Play\docs"),
        );
        assert_eq!(
            path_from_table(&table).unwrap().unwrap(),
            Path::new(r"D:\Play\docs")
                .join("My Games")
                .join("FalloutNV")
                .join("Fallout.ini")
        );
        assert_eq!(path_from_table("GetACP\t\t4e4\t\n"), Ok(None));
        // A UTF-8 name (what the launcher writes) is read; a one-byte code page's isn't, and the
        // miss is said rather than passed over.
        let utf8 = format!("SHGetFolderPathA\t5 0\t0\t{}\n", hex("D:\\Play\\Jos\u{e9}"));
        let path = path_from_table(&utf8).unwrap().unwrap();
        assert!(path.starts_with("D:\\Play\\Jos\u{e9}"), "{path:?}");
        // "D:\Play\Jos" and the one byte e9 a Western code page has for the accented e.
        let ansi = "SHGetFolderPathA\t5 0\t0\t443a5c506c61795c4a6f73e9\n";
        let why = path_from_table(ansi).unwrap_err();
        assert!(why.contains("not read"), "{why}");
    }
}
