//! Party saves: campaigns and per-player save sidecars, adapted to nv-rs's own text saves.
//!
//! Layout under the launcher's saves folder:
//!
//! ```text
//! saves/<campaign id>/campaign.txt            id, host name, creation time
//! saves/<campaign id>/<role>-<player>/        the viewer's working folder for that player
//!     nv-rs-quicksave.txt                     the viewer's save (F5 / F9)
//!     nv-rs-quicksave.txt.party               sidecar the launcher writes after a session
//! ```
//!
//! A campaign id is 16 lower-case hex digits, never zero. The host makes it for a new game and
//! shows it with the join code; a guest may type it in to keep its own character's saves with the
//! same campaign. The launcher only ever creates files here and deletes single files by exact name.

use std::fs;
use std::path::{Path, PathBuf};

pub const SIDECAR_EXT: &str = "party";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Host,
    Guest,
}

impl Role {
    pub fn name(self) -> &'static str {
        match self {
            Role::Host => "host",
            Role::Guest => "guest",
        }
    }
    fn parse(s: &str) -> Option<Role> {
        match s {
            "host" => Some(Role::Host),
            "guest" => Some(Role::Guest),
            _ => None,
        }
    }
}

/// A fresh campaign id from 8 random bytes (never zero).
pub fn new_campaign_id(random: [u8; 8]) -> String {
    let n = u64::from_le_bytes(random).max(1);
    format!("{n:016x}")
}

pub fn valid_campaign_id(s: &str) -> bool {
    s.len() == 16 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) && s != "0000000000000000"
}

/// Lower-cases and validates what a player typed.
pub fn parse_campaign_id(s: &str) -> Option<String> {
    let t = s.trim().to_ascii_lowercase();
    valid_campaign_id(&t).then_some(t)
}

/// A folder-safe form of a player name.
pub fn slug(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect();
    let s = s.trim_matches('-').to_string();
    let s: String = s.chars().take(24).collect();
    if s.is_empty() { "player".into() } else { s }
}

pub fn player_folder(role: Role, player: &str) -> String {
    format!("{}-{}", role.name(), slug(player))
}

pub fn campaign_dir(root: &Path, id: &str) -> PathBuf {
    root.join(id)
}

/// Makes (if needed) the campaign and player folders and the campaign file; returns the player
/// folder, which the viewer is started in.
pub fn ensure_player_dir(root: &Path, id: &str, role: Role, player: &str, now: u64) -> std::io::Result<PathBuf> {
    if !valid_campaign_id(id) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "bad campaign id"));
    }
    let cdir = campaign_dir(root, id);
    let dir = cdir.join(player_folder(role, player));
    fs::create_dir_all(&dir)?;
    let cfile = cdir.join("campaign.txt");
    if !cfile.exists() {
        let host = if role == Role::Host { player } else { "" };
        fs::write(&cfile, format!("NVRS-CAMPAIGN 1\nid={id}\nhost={host}\ncreated={now}\n"))?;
    }
    Ok(dir)
}

fn sidecar_path(save: &Path) -> PathBuf {
    let mut n = save.file_name().map(|n| n.to_os_string()).unwrap_or_default();
    n.push(format!(".{SIDECAR_EXT}"));
    save.with_file_name(n)
}

fn is_save_file(name: &str) -> bool {
    name.to_ascii_lowercase().ends_with(".txt") && !name.eq_ignore_ascii_case("campaign.txt")
}

fn mtime(p: &Path) -> u64 {
    fs::metadata(p)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// After a session: writes a sidecar for each save in `dir` that has none or whose save changed
/// since. Returns how many sidecars were written.
pub fn stamp_saves(dir: &Path, id: &str, role: Role, player: &str, now: u64) -> usize {
    let mut n = 0;
    let Ok(rd) = fs::read_dir(dir) else { return 0 };
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let p = e.path();
        if !p.is_file() || !is_save_file(&name) {
            continue;
        }
        let side = sidecar_path(&p);
        if side.exists() && mtime(&side) >= mtime(&p) {
            continue;
        }
        let body = format!("NVRS-PARTY 1\ncampaign={id}\nrole={}\nplayer={player}\nfile={name}\nsaved={}\nstamped={now}\n", role.name(), mtime(&p));
        if fs::write(&side, body).is_ok() {
            n += 1;
        }
    }
    n
}

fn sidecar_field(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|l| l.strip_prefix(key).and_then(|r| r.strip_prefix('=')).map(|v| v.trim().to_string()))
}

#[derive(Debug, Clone, PartialEq)]
pub struct SaveEntry {
    pub campaign: String,
    pub role: Role,
    pub player: String,
    pub folder: String,
    pub file: String,
    pub path: PathBuf,
    pub modified: u64,
    pub bytes: u64,
}

/// Every save under `root`, newest first. Unknown folders and files are ignored.
pub fn list_saves(root: &Path) -> Vec<SaveEntry> {
    let mut out = Vec::new();
    let Ok(cs) = fs::read_dir(root) else { return out };
    for c in cs.flatten() {
        let id = c.file_name().to_string_lossy().into_owned();
        if !valid_campaign_id(&id) || !c.path().is_dir() {
            continue;
        }
        let Ok(ps) = fs::read_dir(c.path()) else { continue };
        for p in ps.flatten() {
            let folder = p.file_name().to_string_lossy().into_owned();
            let Some((r, who)) = folder.split_once('-') else { continue };
            let Some(role) = Role::parse(r) else { continue };
            if !p.path().is_dir() {
                continue;
            }
            let Ok(fs_) = fs::read_dir(p.path()) else { continue };
            for f in fs_.flatten() {
                let file = f.file_name().to_string_lossy().into_owned();
                if !f.path().is_file() || !is_save_file(&file) {
                    continue;
                }
                let side = fs::read_to_string(sidecar_path(&f.path())).unwrap_or_default();
                let player = sidecar_field(&side, "player").unwrap_or_else(|| who.to_string());
                let meta = f.metadata().ok();
                out.push(SaveEntry {
                    campaign: id.clone(),
                    role,
                    player,
                    folder: folder.clone(),
                    file,
                    path: f.path(),
                    modified: meta.as_ref().map(|_| mtime(&f.path())).unwrap_or(0),
                    bytes: meta.map(|m| m.len()).unwrap_or(0),
                });
            }
        }
    }
    out.sort_by(|a, b| b.modified.cmp(&a.modified).then_with(|| a.path.cmp(&b.path)));
    out
}

/// Host saves, newest first: what Continue offers.
pub fn host_saves(root: &Path) -> Vec<SaveEntry> {
    list_saves(root).into_iter().filter(|s| s.role == Role::Host).collect()
}

/// Checks that `typed` is exactly the file name of `entry` and that the file is a plain file
/// inside the saves folder, then deletes that one file and its sidecar. Never recursive.
pub fn delete_save(root: &Path, entry: &SaveEntry, typed: &str) -> Result<(), String> {
    if typed != entry.file {
        return Err("The name typed does not match the file name exactly. Nothing was deleted.".into());
    }
    if entry.file.contains(['/', '\\', ':']) || entry.file == ".." || entry.file == "." || !is_save_file(&entry.file) {
        return Err("Refusing: not a plain save file name.".into());
    }
    let root_c = fs::canonicalize(root).map_err(|e| format!("saves folder: {e}"))?;
    let file_c = fs::canonicalize(&entry.path).map_err(|e| format!("save file: {e}"))?;
    if !file_c.starts_with(&root_c) || file_c == root_c {
        return Err("Refusing: the file is outside the launcher's saves folder.".into());
    }
    let meta = fs::symlink_metadata(&entry.path).map_err(|e| e.to_string())?;
    if !meta.is_file() {
        return Err("Refusing: not a plain file.".into());
    }
    fs::remove_file(&entry.path).map_err(|e| format!("could not delete: {e}"))?;
    let side = sidecar_path(&entry.path);
    if side.is_file() {
        let _ = fs::remove_file(side);
    }
    Ok(())
}

/// `YYYY-MM-DD HH:MM` (UTC) from unix seconds.
pub fn format_time(secs: u64) -> String {
    let days = (secs / 86400) as i64;
    let rem = secs % 86400;
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02} {:02}:{:02}", rem / 3600, (rem % 3600) / 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("nvl-party-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn ids() {
        let id = new_campaign_id([1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(valid_campaign_id(&id));
        assert!(valid_campaign_id(&new_campaign_id([0; 8])));
        assert_eq!(parse_campaign_id(" 00AB12CD34EF5678 ").as_deref(), Some("00ab12cd34ef5678"));
        assert_eq!(parse_campaign_id("short"), None);
        assert_eq!(parse_campaign_id("../../../etc/pass"), None);
        assert_eq!(parse_campaign_id("0000000000000000"), None);
    }

    #[test]
    fn slugs() {
        assert_eq!(slug("Ann Primm"), "ann-primm");
        assert_eq!(slug("..\\x"), "x");
        assert_eq!(slug("!!!"), "player");
    }

    #[test]
    fn stamp_and_list() {
        let root = tmp("list");
        let id = new_campaign_id([9; 8]);
        let host = ensure_player_dir(&root, &id, Role::Host, "Ann", 100).unwrap();
        let guest = ensure_player_dir(&root, &id, Role::Guest, "Bo", 100).unwrap();
        fs::write(host.join("nv-rs-quicksave.txt"), "host state").unwrap();
        fs::write(guest.join("nv-rs-quicksave.txt"), "guest").unwrap();
        fs::write(guest.join("notes.dat"), "ignored").unwrap();
        assert_eq!(stamp_saves(&host, &id, Role::Host, "Ann", 200), 1);
        assert_eq!(stamp_saves(&host, &id, Role::Host, "Ann", 300), 0);
        assert_eq!(stamp_saves(&guest, &id, Role::Guest, "Bo", 200), 1);
        let all = list_saves(&root);
        assert_eq!(all.len(), 2);
        assert!(all.iter().any(|s| s.role == Role::Guest && s.player == "Bo" && s.campaign == id));
        let hosts = host_saves(&root);
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].player, "Ann");
        let campaign = fs::read_to_string(root.join(&id).join("campaign.txt")).unwrap();
        assert!(campaign.contains("host=Ann"));
        // stray folders are ignored
        fs::create_dir_all(root.join("junk").join("host-x")).unwrap();
        fs::write(root.join("junk").join("host-x").join("a.txt"), "x").unwrap();
        assert_eq!(list_saves(&root).len(), 2);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn delete_needs_exact_name_and_removes_one_file() {
        let root = tmp("del");
        let id = new_campaign_id([7; 8]);
        let dir = ensure_player_dir(&root, &id, Role::Host, "Ann", 1).unwrap();
        fs::write(dir.join("nv-rs-quicksave.txt"), "a").unwrap();
        fs::write(dir.join("other.txt"), "b").unwrap();
        stamp_saves(&dir, &id, Role::Host, "Ann", 2);
        let saves = list_saves(&root);
        let target = saves.iter().find(|s| s.file == "nv-rs-quicksave.txt").unwrap().clone();
        assert!(delete_save(&root, &target, "nv-rs-quicksave").is_err());
        assert!(delete_save(&root, &target, "NV-RS-QUICKSAVE.TXT").is_err());
        assert!(dir.join("nv-rs-quicksave.txt").exists());
        delete_save(&root, &target, "nv-rs-quicksave.txt").unwrap();
        assert!(!dir.join("nv-rs-quicksave.txt").exists());
        assert!(!dir.join("nv-rs-quicksave.txt.party").exists());
        assert!(dir.join("other.txt").exists());
        assert!(dir.exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn delete_refuses_outside_paths() {
        let root = tmp("esc");
        let outside = tmp("esc-outside");
        fs::write(outside.join("victim.txt"), "keep").unwrap();
        let fake = SaveEntry {
            campaign: new_campaign_id([3; 8]),
            role: Role::Host,
            player: "x".into(),
            folder: "host-x".into(),
            file: "victim.txt".into(),
            path: outside.join("victim.txt"),
            modified: 0,
            bytes: 0,
        };
        assert!(delete_save(&root, &fake, "victim.txt").is_err());
        assert!(outside.join("victim.txt").exists());
        let mut dots = fake.clone();
        dots.file = "..\\victim.txt".into();
        assert!(delete_save(&root, &dots, "..\\victim.txt").is_err());
        let _ = fs::remove_dir_all(&root);
        let _ = fs::remove_dir_all(&outside);
    }

    #[test]
    fn time_format() {
        assert_eq!(format_time(0), "1970-01-01 00:00");
        assert_eq!(format_time(1_761_700_000), "2025-10-29 01:06");
    }
}
