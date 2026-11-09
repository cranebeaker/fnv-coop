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

/// The value of `key` in `section` of an INI text, found as the game finds it (host
/// `game_ini::value`): names compared without case, spaces trimmed, one pair of quotes taken off,
/// the first match wins; a `;` line never matches.
pub fn ini_value<'a>(text: &'a str, section: &str, key: &str) -> Option<&'a str> {
    let mut inside = false;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('[') {
            let name = rest.split(']').next().unwrap_or(rest);
            inside = name.trim().eq_ignore_ascii_case(section);
            continue;
        }
        let Some((name, value)) = line.split_once('=').filter(|_| inside) else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case(key) {
            let value = value.trim();
            return Some(
                value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .unwrap_or(value),
            );
        }
    }
    None
}

/// The variables that make the game's run a test (host `fixes.rs`: `TEST_VARIABLES` and each
/// fix's own variable; a host test keeps the two lists alike). In such a run the game never reads
/// Fallout.ini's `[nv-rs ...]` keys, so the launcher doesn't pass them on either.
pub const TEST_SWITCHES: &[&str] = &[
    "NV_TRACE_LIST",
    "NV_MOVE_KEYS",
    "NV_MOVE_MOUSE",
    "NV_MOVE_REAL_INPUT",
    "NV_MOVE_TRACE",
    "NV_MOVE_PROBE",
    "NV_MOVE_TSV",
    "NV_MOVE_WATCH",
    "NV_MOVE_CALL_PROFILE",
    "NV_MOVE_CALL_BUDGET",
    "NV_MOVE_BUDGET_FROM",
    "NV_SOUND_RECORD",
    "NV_SHOT",
    "NV_FRAME_LOG",
    "NV_LOCK_STATS",
    "NV_REAL_DUMP",
    "NV_PHYS_BLEND_LOG",
    "NV_BENCH_CALLS",
    "NV_BENCH_CALL_COUNTS",
    "NV_PROFILE",
    "NV_WATCH",
    "NV_WORD_CHECK",
    "NV_SKIN_CHECK",
    "NV_QUEUE_CHECK",
    "NV_FACE_CHECK",
    "NV_SPEECH_CHECK",
    "NV_LIGHT_CHECK",
    "NV_HKMEM_CHECK",
    "NV_HEAD_CHECK",
    "NV_BLOCK_CHECK",
    "NV_CENSUS",
    "NV_TOUCH",
    "NV_PROFILE_THREAD_CALLS",
    "NV_AUDIO_CHECK",
    "NV_PASS_CHECK",
    "NV_TOPIC_INDEX_CHECK",
    "NV_NVSE_EVENT_LOG",
    "NV_UNCAP",
    "NV_HF_LANDING",
    "NV_HF_SLOWMO",
    "NV_MENU_CAP",
    "NV_GRAB_PUSH",
    "NV_PHYS_BLEND",
    "NV_MOD_WORN",
    "NV_TOPIC_INDEX",
    "NV_SHADER_WARM",
    "NV_IO_WAIT",
    "NV_COSAVE_KEEP",
    "NV_NVSE_ERROR_GUARD",
    "NV_NVSE_IMOD_CHECK",
    "NV_NVSE_INVREF_CHECK",
    "NV_NVSE_SIZE_CAPS",
    "NV_NVSE_NULL_GUARD",
    "NV_NVSE_CYCLE_GUARD",
    "NV_NVSE_LIST_GUARD",
];

/// Whether the game will run as a test: one of [`TEST_SWITCHES`] among the variables it gets
/// (`passed`, names compared without case, as Windows does) or in the launcher's own environment
/// (`inherited`), which it gets too.
pub fn makes_a_test<'a>(
    mut passed: impl Iterator<Item = &'a str>,
    inherited: impl Fn(&str) -> bool,
) -> bool {
    passed.any(|name| {
        TEST_SWITCHES
            .iter()
            .any(|test| test.eq_ignore_ascii_case(name))
    }) || TEST_SWITCHES.iter().any(|name| inherited(name))
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

    /// The player switches the run's `Fallout.ini` sets in its `[nv-rs ...]` sections (host
    /// `game_ini.rs`), which win over this file's: `Some(value)` to pass, `None` to leave the
    /// variable out (the game's own default). A key that is missing, empty or not a value the
    /// game takes leaves this file's switch as it is.
    pub fn file_switches(fallout_ini: &str) -> Vec<(&'static str, Option<String>)> {
        let get = |section: &str, key: &str| {
            ini_value(fallout_ini, section, key).filter(|v| !v.is_empty())
        };
        let flag = |section: &str, key: &str| match get(section, key) {
            Some("1") => Some(true),
            Some("0") => Some(false),
            _ => None,
        };
        let number = |key: &str| {
            get("nv-rs Display", key).and_then(|v| v.parse::<u32>().ok().map(|n| n.to_string()))
        };
        let mut out = Vec::new();
        match get("nv-rs General", "sPreset") {
            Some(v) if v.eq_ignore_ascii_case("fixes") || v.eq_ignore_ascii_case("vanilla") => {
                out.push(("NV_PRESET", Some(v.to_ascii_lowercase())));
            }
            _ => {}
        }
        if let Some(n) = number("iFrameLimit") {
            out.push(("NV_FRAME_LIMIT", Some(n)));
        }
        if let Some(n) = number("iMenuFrameLimit") {
            out.push(("NV_MENU_FRAME_LIMIT", Some(n)));
        }
        if let Some(on) = flag("nv-rs Display", "bBorderless") {
            out.push(("NV_BORDERLESS", on.then(|| "1".to_string())));
        }
        if let Some(stepped) = flag("nv-rs Threads", "bSteppedThreads") {
            out.push(("NV_THREADS", stepped.then(|| "stepped".to_string())));
        }
        if let Some(on) = flag("nv-rs Threads", "bRenderThread") {
            out.push((
                "NV_RENDER_THREAD",
                Some(if on { "1" } else { "0" }.to_string()),
            ));
        }
        out
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

    /// Fallout.ini's [nv-rs ...] keys become the switches that win over settings.ini's; a commented,
    /// empty or unusable key leaves settings.ini's.
    #[test]
    fn fallout_inis_keys_win_over_settings_ini() {
        let ini = "[General]\nsPreset=vanilla\n[nv-rs General]\nsPreset=Vanilla\n\
                   [nv-rs Display]\nbBorderless=0\niFrameLimit=fast\n;iMenuFrameLimit=60\n\
                   [nv-rs Threads]\nbSteppedThreads=1\nbRenderThread=0\n";
        let some = |v: &str| Some(v.to_string());
        assert_eq!(
            Settings::file_switches(ini),
            [
                ("NV_PRESET", some("vanilla")),
                ("NV_BORDERLESS", None),
                ("NV_THREADS", some("stepped")),
                ("NV_RENDER_THREAD", some("0")),
            ]
        );
        let ini = "[nv-rs Display]\nbBorderless=1\niFrameLimit=90\niMenuFrameLimit=0\n\
                   [nv-rs Threads]\nbSteppedThreads=0\n";
        assert_eq!(
            Settings::file_switches(ini),
            [
                ("NV_FRAME_LIMIT", some("90")),
                ("NV_MENU_FRAME_LIMIT", some("0")),
                ("NV_BORDERLESS", some("1")),
                ("NV_THREADS", None),
            ]
        );
        assert!(Settings::file_switches("[General]\nsLanguage=ENGLISH\n").is_empty());
        // A settings.ini line or an inherited variable that makes the run a test: the game
        // won't read the file, so the launcher passes none of it.
        let none = |_: &str| false;
        assert!(makes_a_test(["NV_THREADS", "nv_uncap"].into_iter(), none));
        assert!(makes_a_test(std::iter::empty(), |n| n == "NV_TRACE_LIST"));
        assert!(!makes_a_test(
            ["NV_THREADS", "NV_RENDER_THREAD", "NV_SAVE_OUT"].into_iter(),
            none
        ));
        assert_eq!(
            ini_value(
                " [ NV-RS display ]\n IFRAMELIMIT = \"5\" \n",
                "nv-rs Display",
                "iFrameLimit"
            ),
            Some("5")
        );
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
