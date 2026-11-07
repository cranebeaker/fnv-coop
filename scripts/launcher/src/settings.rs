//! `userdata\settings.ini`, which Settings.cmd writes and the launcher reads: `Name=Value` lines.
//! Names the launcher doesn't know are kept by Settings.cmd and ignored here, so a new setting can
//! join without a new launcher. A name starting with `NV_` goes to the game's environment as it is
//! (but the launcher's own switches: `main.rs`'s `OWN_SWITCHES`), which is how a switch of the
//! game's own can join Settings later. `Preset`, `FrameLimit` and `MenuFrameLimit` become the game's
//! `NV_PRESET`, `NV_FRAME_LIMIT` and `NV_MENU_FRAME_LIMIT` ([`Settings::game_switches`]).
use std::collections::BTreeMap;

/// Menus' frames a second at most with no `MenuFrameLimit=` line: the game's default.
const MENU_FRAME_LIMIT: u32 = 120;

/// The file's text as the game decodes it (host `fixes.rs`): a UTF-8 byte order mark dropped,
/// UTF-16 (what PowerShell 5.1's `>` writes) decoded, anything else read as UTF-8 with bad bytes
/// replaced. The launcher passes the preset on, so it must read the same lines the game would.
pub fn decode(bytes: &[u8]) -> String {
    let utf16 = |rest: &[u8], big: bool| {
        let units: Vec<u16> = rest
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&c| {
                if big {
                    u16::from_be_bytes(c)
                } else {
                    u16::from_le_bytes(c)
                }
            })
            .collect();
        String::from_utf16_lossy(&units)
    };
    match bytes {
        [0xff, 0xfe, rest @ ..] => utf16(rest, false),
        [0xfe, 0xff, rest @ ..] => utf16(rest, true),
        [0xef, 0xbb, 0xbf, rest @ ..] => String::from_utf8_lossy(rest).into_owned(),
        _ => String::from_utf8_lossy(bytes).into_owned(),
    }
}

/// The file's settings.
#[derive(Debug, Default, PartialEq)]
pub struct Settings(pub BTreeMap<String, String>);

impl Settings {
    pub fn parse(text: &str) -> Settings {
        let mut map = BTreeMap::new();
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if let Some((name, value)) = line.split_once('=') {
                // The last line of a name wins, its case aside, as the game and Settings.ps1 read it.
                let name = name.trim();
                map.retain(|k: &String, _| !k.eq_ignore_ascii_case(name));
                map.insert(name.to_string(), value.trim().to_string());
            }
        }
        Settings(map)
    }

    fn get(&self, name: &str) -> &str {
        self.0.get(name).map_or("", String::as_str)
    }

    /// Borderless window: on unless the file says otherwise (the way the game has been played most).
    pub fn borderless(&self) -> bool {
        self.get("Borderless") != "0"
    }

    /// A setting by its name compared without case, as the game reads `Preset` and `FrameLimit`.
    fn get_any_case(&self, name: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// The game's fixes preset and frame limits as its switches, `NV_PRESET` (`fixes` or `vanilla`),
    /// `NV_FRAME_LIMIT` and `NV_MENU_FRAME_LIMIT` (frames a second, 0 for no limit), from
    /// `Preset=`, `FrameLimit=` and `MenuFrameLimit=`: the fixes, no limit and menus at 120 (the
    /// game's own default, host `fixes.rs`) when a line is missing or empty. Always both, so a test switch in the
    /// file or the launcher's own environment can't turn the player's game to vanilla unasked. A
    /// value the game wouldn't take is an error the player reads, never a silent default.
    pub fn game_switches(&self) -> Result<[(&'static str, String); 3], String> {
        let preset = match self.get_any_case("Preset").map(str::trim) {
            None | Some("") => "fixes".to_string(),
            Some(v) if v.eq_ignore_ascii_case("fixes") => "fixes".to_string(),
            Some(v) if v.eq_ignore_ascii_case("vanilla") => "vanilla".to_string(),
            Some(v) => return Err(format!("Preset={v} (it can be fixes or vanilla)")),
        };
        let limit = match self.get_any_case("FrameLimit").map(str::trim) {
            None | Some("") => 0,
            Some(v) => v
                .parse::<u32>()
                .map_err(|_| format!("FrameLimit={v} (it can be a number, 0 for no limit)"))?,
        };
        let menu = match self.get_any_case("MenuFrameLimit").map(str::trim) {
            None | Some("") => MENU_FRAME_LIMIT,
            Some(v) => v
                .parse::<u32>()
                .map_err(|_| format!("MenuFrameLimit={v} (it can be a number, 0 for no limit)"))?,
        };
        Ok([
            ("NV_PRESET", preset),
            ("NV_FRAME_LIMIT", limit.to_string()),
            ("NV_MENU_FRAME_LIMIT", menu.to_string()),
        ])
    }

    /// The game's environment switches from the file (`NV_...=...`).
    pub fn environment(&self) -> Vec<(&str, &str)> {
        self.0
            .iter()
            .filter(|(k, _)| k.starts_with("NV_"))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    /// The `FalloutPrefs.ini` lines these settings set, by the game's own names; `desktop` is the
    /// primary monitor's size, used when no resolution was chosen and the prefs have none.
    pub fn prefs_lines(&self) -> Vec<(String, String)> {
        let mut set = Vec::new();
        let times = |v: &str| match v {
            "Off" => Some(0u32),
            v => v.strip_suffix('x').and_then(|n| n.parse().ok()),
        };
        if self.borderless() {
            set.push(("bFull Screen".into(), "0".into()));
        }
        if let Some((w, h)) = self.get("Resolution").split_once('x') {
            if w.parse::<u32>().is_ok() && h.parse::<u32>().is_ok() {
                set.push(("iSize W".into(), w.into()));
                set.push(("iSize H".into(), h.into()));
            }
        }
        if let Some(n) = times(self.get("AntiAliasing")) {
            set.push(("iMultiSample".into(), n.to_string()));
        }
        if let Some(n) = times(self.get("Anisotropy")) {
            set.push(("iMaxAnisotropy".into(), n.to_string()));
        }
        match self.get("VSync") {
            "On" => set.push(("iPresentInterval".into(), "1".into())),
            "Off" => set.push(("iPresentInterval".into(), "0".into())),
            _ => {}
        }
        set
    }
}

/// `prefs` (a `FalloutPrefs.ini`) with these lines set: each replaced where its name stands, and
/// those it doesn't have added to `[Display]` (the section made when missing). Line ends stay.
pub fn apply_to_prefs(prefs: &str, lines: &[(String, String)]) -> String {
    let mut missing: Vec<&(String, String)> = lines.iter().collect();
    let mut out = String::with_capacity(prefs.len() + 64);
    for line in prefs.split_inclusive('\n') {
        let body = line.trim_end_matches(['\r', '\n']);
        let end = &line[body.len()..];
        let set = body
            .split_once('=')
            .and_then(|(name, _)| lines.iter().find(|(n, _)| n == name));
        match set {
            Some((name, value)) => {
                out.push_str(name);
                out.push('=');
                out.push_str(value);
                out.push_str(end);
                missing.retain(|(n, _)| n != name);
            }
            None => out.push_str(line),
        }
    }
    if missing.is_empty() {
        return out;
    }
    let mut added = String::new();
    for (name, value) in &missing {
        added.push_str(name);
        added.push('=');
        added.push_str(value);
        added.push_str("\r\n");
    }
    if let Some(at) = out.find("[Display]") {
        let after = out[at..].find('\n').map_or(out.len(), |n| at + n + 1);
        if after == out.len() && !out.ends_with('\n') {
            out.push_str("\r\n");
            out.push_str(&added);
        } else {
            out.insert_str(after, &added);
        }
    } else {
        if !out.is_empty() && !out.ends_with('\n') {
            out.push_str("\r\n");
        }
        out.push_str("[Display]\r\n");
        out.push_str(&added);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &[(&str, &str)]) -> Vec<(String, String)> {
        s.iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    }

    #[test]
    fn the_file_is_read_and_unknown_names_do_no_harm() {
        let s = Settings::parse(
            "# a comment\nBorderless=1\nResolution=2560x1440\nAntiAliasing=8x\nAnisotropy=Off\nVSync=Game launcher's\nSomethingNew=7\nNV_FIXES=vnv\n",
        );
        assert!(s.borderless());
        assert_eq!(
            s.prefs_lines(),
            lines(&[
                ("bFull Screen", "0"),
                ("iSize W", "2560"),
                ("iSize H", "1440"),
                ("iMultiSample", "8"),
                ("iMaxAnisotropy", "0"),
            ])
        );
        assert_eq!(s.environment(), [("NV_FIXES", "vnv")]);
        assert!(Settings::parse("").borderless());
        assert!(!Settings::parse("Borderless=0").borderless());
        assert!(Settings::parse("Borderless=0\nResolution=Game launcher's")
            .prefs_lines()
            .is_empty());
    }

    fn switches(text: &str) -> Result<Vec<(&'static str, String)>, String> {
        Settings::parse(text).game_switches().map(Vec::from)
    }

    fn pair(preset: &str, limit: &str) -> Vec<(&'static str, String)> {
        with_menu(preset, limit, "120")
    }

    fn with_menu(preset: &str, limit: &str, menu: &str) -> Vec<(&'static str, String)> {
        vec![
            ("NV_PRESET", preset.to_string()),
            ("NV_FRAME_LIMIT", limit.to_string()),
            ("NV_MENU_FRAME_LIMIT", menu.to_string()),
        ]
    }

    #[test]
    fn the_menu_frame_limit_becomes_the_games_switch() {
        assert_eq!(
            switches("MenuFrameLimit=\n"),
            Ok(with_menu("fixes", "0", "120"))
        );
        assert_eq!(
            switches("FrameLimit=144\nMenuFrameLimit=60\n"),
            Ok(with_menu("fixes", "144", "60"))
        );
        assert_eq!(
            switches("menuframelimit = 0\n"),
            Ok(with_menu("fixes", "0", "0"))
        );
        let e = switches("MenuFrameLimit=fast\n").unwrap_err();
        assert!(e.contains("MenuFrameLimit=fast"), "{e}");
    }

    #[test]
    fn preset_and_frame_limit_become_the_games_switches() {
        // Nothing chosen, or an empty line: the fixes and no limit.
        assert_eq!(switches(""), Ok(pair("fixes", "0")));
        assert_eq!(switches("Preset=\nFrameLimit=\n"), Ok(pair("fixes", "0")));
        // What Settings.cmd writes.
        assert_eq!(
            switches("Preset=vanilla\nFrameLimit=144\n"),
            Ok(pair("vanilla", "144"))
        );
        assert_eq!(
            switches("Preset=fixes\nFrameLimit=0\n"),
            Ok(pair("fixes", "0"))
        );
        // Written by hand: case and spaces as the game takes them.
        assert_eq!(
            switches("preset = Vanilla \nframelimit= 60\n"),
            Ok(pair("vanilla", "60"))
        );
        // A value the game would refuse stops here, with the line named.
        let e = switches("Preset=vanila\n").unwrap_err();
        assert!(e.contains("Preset=vanila"), "{e}");
        let e = switches("FrameLimit=60fps\n").unwrap_err();
        assert!(e.contains("FrameLimit=60fps"), "{e}");
        assert!(switches("FrameLimit=-1\n").is_err());
        // A stale NV_FIXES line chooses nothing: the preset stays the fixes.
        assert_eq!(switches("NV_FIXES=vanilla\n"), Ok(pair("fixes", "0")));
    }

    #[test]
    fn the_file_is_decoded_as_the_game_decodes_it() {
        let text = "Preset=vanilla\r\n";
        let mut utf16 = vec![0xff, 0xfe];
        utf16.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
        let mut big = vec![0xfe, 0xff];
        big.extend(text.encode_utf16().flat_map(u16::to_be_bytes));
        let mut marked = vec![0xef, 0xbb, 0xbf];
        marked.extend(text.bytes());
        for bytes in [utf16, big, marked, text.as_bytes().to_vec()] {
            assert_eq!(
                Settings::parse(&decode(&bytes)).game_switches().unwrap()[0].1,
                "vanilla"
            );
        }
    }

    #[test]
    fn prefs_keep_everything_else() {
        let prefs = "[Display]\r\niSize W=800\r\nbFull Screen=1\r\n[Audio]\r\nfVolume=1\r\n";
        let out = apply_to_prefs(prefs, &lines(&[("iSize W", "1920"), ("bFull Screen", "0")]));
        assert_eq!(
            out,
            "[Display]\r\niSize W=1920\r\nbFull Screen=0\r\n[Audio]\r\nfVolume=1\r\n"
        );
    }

    #[test]
    fn what_prefs_lack_goes_into_display() {
        let out = apply_to_prefs(
            "[Audio]\r\nfVolume=1\r\n[Display]\r\niSize W=800\r\n",
            &lines(&[("iSize H", "600")]),
        );
        assert_eq!(
            out,
            "[Audio]\r\nfVolume=1\r\n[Display]\r\niSize H=600\r\niSize W=800\r\n"
        );
        let out = apply_to_prefs("", &lines(&[("iSize H", "600")]));
        assert_eq!(out, "[Display]\r\niSize H=600\r\n");
        let out = apply_to_prefs("[Audio]\nfVolume=1", &lines(&[("iSize H", "600")]));
        assert_eq!(out, "[Audio]\nfVolume=1\r\n[Display]\r\niSize H=600\r\n");
    }

    #[test]
    fn the_last_line_of_a_name_wins_whatever_its_case() {
        let s = Settings::parse("Preset=fixes\npreset=vanilla\n");
        assert_eq!(s.game_switches().unwrap()[0].1, "vanilla");
        let s = Settings::parse("preset=vanila\nPreset=fixes\nFrameLimit=60\nframelimit=0\n");
        let sw = s.game_switches().unwrap();
        assert_eq!((sw[0].1.as_str(), sw[1].1.as_str()), ("fixes", "0"));
    }
}
