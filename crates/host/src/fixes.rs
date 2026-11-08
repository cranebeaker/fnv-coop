//! The fixes preset: what runs differently from FalloutNV.exe 1.4.0.525 (docs/FIXES.md).
//!
//! A player who changes nothing gets the `fixes` set; `vanilla` is the original's behaviour, and every
//! comparison, gate and recording pins it (the maintainer, 2026-11-02). [`Choice`] is read once at
//! start-up and [`install`] binds it before the first frame: a fix that is on is registered over the
//! translation at its exe address, a fix that is off registers nothing, so the translation runs as
//! it is and no call tests a flag. The translations themselves are never edited for a fix.
//!
//! [`crate::Host::start`] installs nothing: tests and tools run the original. The viewer reads the
//! choice ([`Choice::read`]) and calls [`crate::Host::start_with`].

use engine::prelude::*;
use std::path::{Path, PathBuf};

/// One fix: a behaviour that differs from the exe, registered at exe addresses.
pub struct Fix {
    /// Its name in the start-up line and in docs/FIXES.md.
    pub name: &'static str,
    /// The environment variable that turns only this fix on (`1`) or off (`0`) whatever the preset
    /// says: for tests and gates. A player has the preset and [`Choice::frame_limit`].
    pub env: &'static str,
    /// The exe addresses it registers at. Every one is in `crates/engine-core/data/overridable.tsv`,
    /// so typed code keeps calling it by address (a test here checks).
    pub addresses: &'static [u32],
    /// One line: what it changes.
    pub summary: &'static str,
    /// Registers it, at [`Fix::addresses`] only.
    pub install: fn(&mut Engine),
}

/// The fixes, in the order they are installed.
pub static FIXES: &[Fix] = &[
    crate::uncap::FIX,
    crate::uncap::PHYSICS_FIX,
    crate::uncap::SLOW_MOTION_FIX,
    crate::uncap::MENU_FIX,
    crate::uncap::GRAB_FIX,
    crate::phys_blend::FIX,
    crate::mod_fit_worn::FIX,
    crate::topic_index::FIX,
    KEEP_COSAVE_PLUGINS,
    SCRIPT_ERROR_GUARD,
];

/// The built-in script extender's rows (docs/FIXES.md, "The script extender"): each sets a flag
/// `crates/nvse` reads, at no exe address of its own.
pub const KEEP_COSAVE_PLUGINS: Fix = Fix {
    name: "co-save keeps other plugins",
    env: "NV_COSAVE_KEEP",
    addresses: &[],
    summary:
        "a co-save's sections of plugins not built in (JIP LN NVSE's) are written back while the same files are loaded",
    install: install_keep_cosave_plugins,
};

fn install_keep_cosave_plugins(e: &mut Engine) {
    nvse::saveload::set_keep_other_plugins(e, true);
}

pub const SCRIPT_ERROR_GUARD: Fix = Fix {
    name: "script error loop guard",
    env: "NV_NVSE_ERROR_GUARD",
    addresses: &[],
    summary: "an error inside a RuntimeScriptError handler is not sent to the handlers again",
    install: install_script_error_guard,
};

fn install_script_error_guard(e: &mut Engine) {
    nvse::events::set_error_guard(e, true);
}

/// The rows that register nothing at an exe address: the menu limit paces the uncapped step;
/// the extender's rows set flags `crates/nvse` reads.
#[cfg(test)]
const NO_ADDRESS: &[&str] = &[
    crate::uncap::MENU_FIX.name,
    KEEP_COSAVE_PLUGINS.name,
    SCRIPT_ERROR_GUARD.name,
];

/// Menus' frames a second at most when nothing says otherwise (the maintainer, 2026-11-07).
pub const DEFAULT_MENU_FRAME_LIMIT: u32 = 120;

/// How the game's threads run in play when `NV_THREADS` names nothing ([`Choice::threads`]):
/// real, side by side on their own cores (`engine::real_threads`), since the evidence of
/// measure/real-threads (the maintainer's gate, 2026-11-08); `NV_THREADS=stepped` runs them one
/// at a time. A test never takes this: it runs stepped unless it names threads.
pub const PLAY_THREADS: &str = "real";

/// The player's one line: the fixes set, or the original's behaviour.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Preset {
    /// The default (the maintainer, 2026-11-02).
    Fixes,
    /// FalloutNV.exe 1.4.0.525's behaviour: nothing registered.
    Vanilla,
}

impl Preset {
    /// The preset a word names, compared without case.
    fn parse(word: &str) -> Option<Preset> {
        match word.trim().to_ascii_lowercase().as_str() {
            "fixes" => Some(Preset::Fixes),
            "vanilla" => Some(Preset::Vanilla),
            _ => None,
        }
    }

    /// Its name in settings and logs.
    pub fn name(self) -> &'static str {
        match self {
            Preset::Fixes => "fixes",
            Preset::Vanilla => "vanilla",
        }
    }
}

/// What this run does, bound before the first frame ([`install`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    pub preset: Preset,
    /// Frames a second at most, 0 for no limit. Only the uncapped step reads it: the original's
    /// own limit is its sleep, which vanilla keeps.
    pub frame_limit: u32,
    /// Frames a second at most in a menu, 0 for no limit of its own: read by the menu frame limit
    /// ([`crate::uncap::MENU_FIX`]), which rides on the uncapped step.
    pub menu_frame_limit: u32,
    /// Fixes one environment variable turned on or off, by [`Fix::env`].
    overrides: Vec<(&'static str, bool)>,
    /// How the game's threads run, in `NV_THREADS`'s words
    /// (`engine::thread_host::Scheme::from_name`): `NV_THREADS` when it names any; else
    /// `stepped` for a test (whatever pins vanilla pins this too, so a comparison, recording or
    /// measurement never changes when the play default does); else [`PLAY_THREADS`].
    pub threads: String,
}

impl Choice {
    /// The original's behaviour, whatever the environment and the settings file say: what
    /// [`crate::Host::start`] runs (stepped threads too).
    pub fn vanilla() -> Choice {
        Choice {
            preset: Preset::Vanilla,
            frame_limit: 0,
            menu_frame_limit: DEFAULT_MENU_FRAME_LIMIT,
            overrides: Vec::new(),
            threads: "stepped".to_string(),
        }
    }

    /// The choice from the settings file ([`settings_file`]) and the environment, which wins:
    /// `NV_PRESET=vanilla|fixes`, `NV_FRAME_LIMIT=n`, `NV_MENU_FRAME_LIMIT=n`, and each
    /// [`Fix::env`]. Anything it can't
    /// read stops the start with a message, so a misspelt pin never runs the wrong game.
    pub fn read() -> Result<Choice, String> {
        Choice::from_file(settings_file().as_deref(), |name| std::env::var(name).ok())
    }

    /// [`Choice::read`] on a given settings file (none: no file) and environment. A file that
    /// exists and can't be read stops the start, since its `Preset=vanilla` would be lost; an
    /// explicit `NV_PRESET` makes the file moot, so then it doesn't matter.
    pub fn from_file(
        file: Option<&Path>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Choice, String> {
        let text = match file.map(|p| (p, std::fs::read(p))) {
            None => None,
            Some((_, Ok(bytes))) => Some(decode_settings(&bytes)),
            Some((_, Err(e))) if e.kind() == std::io::ErrorKind::NotFound => None,
            Some((_, Err(_))) if env("NV_PRESET").is_some() => None,
            Some((path, Err(e))) => {
                return Err(format!(
                    "Can't read the settings file {}: {e}. Fix it or remove it (its Preset line \
                     would be lost), or set NV_PRESET.",
                    path.display()
                ))
            }
        };
        Choice::from_parts(text.as_deref(), env)
    }

    /// [`Choice::read`] on given text and environment.
    pub fn from_parts(
        settings: Option<&str>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Choice, String> {
        let mut choice = Choice {
            preset: Preset::Fixes,
            frame_limit: 0,
            menu_frame_limit: DEFAULT_MENU_FRAME_LIMIT,
            overrides: Vec::new(),
            threads: PLAY_THREADS.to_string(),
        };
        for line in settings.unwrap_or("").lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            if key.starts_with(['#', ';']) {
                continue;
            }
            // The environment wins, so a file's line it overrides isn't even read (a test's pin
            // beats a settings file above the program, whatever that holds).
            if key.eq_ignore_ascii_case("Preset") {
                if env("NV_PRESET").is_none() {
                    choice.preset = parse_preset(value, "settings.ini's Preset")?;
                }
            } else if key.eq_ignore_ascii_case("FrameLimit") {
                if env("NV_FRAME_LIMIT").is_some() {
                    continue;
                }
                choice.frame_limit = parse_limit(value, "settings.ini's FrameLimit")?;
            } else if key.eq_ignore_ascii_case("MenuFrameLimit") {
                // An empty line is the default, as the launcher reads it.
                if env("NV_MENU_FRAME_LIMIT").is_some() || value.trim().is_empty() {
                    continue;
                }
                choice.menu_frame_limit = parse_limit(value, "settings.ini's MenuFrameLimit")?;
            }
        }
        if let Some(value) = env("NV_FRAME_LIMIT") {
            choice.frame_limit = parse_limit(&value, "NV_FRAME_LIMIT")?;
        }
        if let Some(value) = env("NV_MENU_FRAME_LIMIT") {
            choice.menu_frame_limit = parse_limit(&value, "NV_MENU_FRAME_LIMIT")?;
        }
        for fix in FIXES {
            match env(fix.env).as_deref().map(str::trim) {
                None => {}
                Some("1") => choice.overrides.push((fix.env, true)),
                Some("0") => choice.overrides.push((fix.env, false)),
                Some(other) => return Err(format!("{}={other}: use 0 or 1", fix.env)),
            }
        }
        // A recording, a scripted run, a measurement or a single fix's switch is a test: with no
        // NV_PRESET of its own it runs on vanilla, so a script that was never told about the preset
        // still measures the original. Only a plain start (or the launcher, whose NV_SAVE_OUT is a
        // real player's) gets the fixes. A test runs the game's threads stepped the same way,
        // unless its NV_THREADS names them.
        let test =
            TEST_VARIABLES.iter().any(|name| env(name).is_some()) || !choice.overrides.is_empty();
        if test {
            choice.preset = Preset::Vanilla;
            choice.threads = "stepped".to_string();
        }
        if let Some(value) = env("NV_PRESET") {
            choice.preset = parse_preset(&value, "NV_PRESET")?;
        }
        if let Some(value) = env("NV_THREADS").filter(|v| !v.trim().is_empty()) {
            choice.threads = value;
        }
        Ok(choice)
    }

    /// A run its command line marks as a test (the viewer's `--movement-missing` and
    /// `--movement-probe`, which record or go on past untranslated functions on the main
    /// thread's engine only): its threads run stepped, as a test's do, unless `NV_THREADS` (in
    /// `env`) names them.
    pub fn pin_test_threads(&mut self, env: impl Fn(&str) -> Option<String>) {
        if env("NV_THREADS").map_or(true, |v| v.trim().is_empty()) {
            self.threads = "stepped".to_string();
        }
    }

    /// The menu frame limit the uncapped step gets: `None` when [`crate::uncap::MENU_FIX`] is off.
    pub fn bound_menu_frame_limit(&self) -> Option<u32> {
        self.is_on(&crate::uncap::MENU_FIX)
            .then_some(self.menu_frame_limit)
    }

    /// Whether this fix runs: its environment variable if set, else the preset.
    pub fn is_on(&self, fix: &Fix) -> bool {
        self.overrides
            .iter()
            .find(|(env, _)| *env == fix.env)
            .map_or(self.preset == Preset::Fixes, |(_, on)| *on)
    }

    /// The start-up line: what this run is, so a log (and an F12 report) says which game it
    /// measured: the preset, the fixes on, the limits and the threads asked for ([`Choice::threads`];
    /// the host's own `threads:` line says how they ran).
    pub fn describe(&self) -> String {
        let on: Vec<&str> = FIXES
            .iter()
            .filter(|f| self.is_on(f))
            .map(|f| f.name)
            .collect();
        let uncapped = on.contains(&crate::uncap::FIX.name);
        let menu = match self.bound_menu_frame_limit() {
            Some(0) if uncapped => ", menus no limit of their own".to_string(),
            Some(n) if uncapped => format!(", menu limit {n}"),
            _ => String::new(),
        };
        let limit = if self.frame_limit == 0 || !uncapped {
            menu
        } else {
            format!("{menu}, frame limit {}", self.frame_limit)
        };
        format!(
            "Preset {}: {}{limit}; threads {}",
            self.preset.name(),
            if on.is_empty() {
                "no fix on".to_string()
            } else {
                on.join(", ")
            },
            self.threads.trim()
        )
    }
}

/// Environment variables that mean a test, a measurement or a scripted run: the host's input
/// scripts, recordings, probes and logs. Without an `NV_PRESET` they run vanilla. (`NV_SAVE_OUT`,
/// which the preview launcher sets for real play, and `NV_SOUND`, `NV_THREADS` and the like, which
/// are run settings, are not here.)
const TEST_VARIABLES: &[&str] = &[
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
    "NV_TICK_COUNT",
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
];

/// The settings file's text: a UTF-8 byte order mark is dropped and UTF-16 (what PowerShell 5.1's
/// `>` writes) is decoded; anything else is read as UTF-8, bad bytes replaced.
fn decode_settings(bytes: &[u8]) -> String {
    let utf16 = |rest: &[u8], big: bool| {
        let units: Vec<u16> = rest
            .chunks_exact(2)
            .map(|c| {
                if big {
                    u16::from_be_bytes([c[0], c[1]])
                } else {
                    u16::from_le_bytes([c[0], c[1]])
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

fn parse_preset(value: &str, source: &str) -> Result<Preset, String> {
    Preset::parse(value).ok_or_else(|| format!("{source}={}: use fixes or vanilla", value.trim()))
}

fn parse_limit(value: &str, source: &str) -> Result<u32, String> {
    value.trim().parse().map_err(|_| {
        format!(
            "{source}={}: use a number of frames a second, 0 for no limit",
            value.trim()
        )
    })
}

/// Registers each fix of the choice over its translation: run after every registration and
/// before the game's first frame (EXTENDER-HOOKS R1). Each exe address has one implementation
/// for the whole run.
pub fn install(e: &mut Engine, choice: &Choice) {
    crate::uncap::set_frame_limit(choice.frame_limit);
    crate::uncap::set_menu_frame_limit(choice.bound_menu_frame_limit());
    for fix in FIXES.iter().filter(|f| choice.is_on(f)) {
        (fix.install)(e);
    }
}

/// The settings file: `NV_SETTINGS` when set; else `settings.ini` beside the program or in the
/// folder above it (a package has `app\nv-viewer.exe` under the folder with `Play.cmd`). The same
/// file `Settings.cmd` writes.
pub fn settings_file() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("NV_SETTINGS") {
        return Some(PathBuf::from(path));
    }
    let exe = std::env::current_exe().ok()?;
    let beside = exe.parent()?;
    candidates(beside).into_iter().find(|p| p.is_file())
}

fn candidates(beside: &Path) -> Vec<PathBuf> {
    let mut paths = vec![beside.join("settings.ini")];
    if let Some(above) = beside.parent() {
        paths.push(above.join("settings.ini"));
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_env(_: &str) -> Option<String> {
        None
    }

    fn env_of(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |name| {
            pairs
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, v)| (*v).to_string())
        }
    }

    #[test]
    fn nothing_set_is_the_fixes_preset() {
        let choice = Choice::from_parts(None, no_env).unwrap();
        assert_eq!(choice.preset, Preset::Fixes);
        assert_eq!(choice.frame_limit, 0);
        assert!(FIXES.iter().all(|f| choice.is_on(f)));
    }

    #[test]
    fn the_settings_file_picks_vanilla_and_the_environment_wins() {
        let text = "# written by Settings.cmd\nBorderless=1\nPreset=Vanilla\nFrameLimit=120\n";
        let choice = Choice::from_parts(Some(text), no_env).unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        assert_eq!(choice.frame_limit, 120);
        assert!(FIXES.iter().all(|f| !choice.is_on(f)));

        let choice = Choice::from_parts(Some(text), env_of(&[("NV_PRESET", "fixes")])).unwrap();
        assert_eq!(choice.preset, Preset::Fixes);
        let choice = Choice::from_parts(
            Some("Preset=fixes"),
            env_of(&[("NV_PRESET", "vanilla"), ("NV_FRAME_LIMIT", "60")]),
        )
        .unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        assert_eq!(choice.frame_limit, 60);
    }

    /// `#` and `;` lines don't count, and neither does anything else the settings file holds.
    #[test]
    fn comments_and_other_keys_are_left_alone() {
        let text = "#Preset=vanilla\n; Preset = vanilla\nVSync=Off\nResolution=2560x1440\n";
        let choice = Choice::from_parts(Some(text), no_env).unwrap();
        assert_eq!(choice.preset, Preset::Fixes);
    }

    /// A pin that can't be read must not run the wrong game.
    #[test]
    fn a_misspelt_setting_stops_the_start() {
        assert!(Choice::from_parts(None, env_of(&[("NV_PRESET", "vanila")])).is_err());
        assert!(Choice::from_parts(Some("Preset=none"), no_env).is_err());
        assert!(Choice::from_parts(Some("FrameLimit=fast"), no_env).is_err());
        assert!(Choice::from_parts(None, env_of(&[("NV_FRAME_LIMIT", "-1")])).is_err());
        assert!(Choice::from_parts(None, env_of(&[("NV_UNCAP", "yes")])).is_err());
    }

    /// The speed gates run the old `NV_UNCAP=1` on vanilla: only that fix, whatever else.
    #[test]
    fn one_fix_can_be_switched_alone() {
        let choice =
            Choice::from_parts(None, env_of(&[("NV_PRESET", "vanilla"), ("NV_UNCAP", "1")]))
                .unwrap();
        assert!(choice.is_on(&crate::uncap::FIX));
        assert!(!choice.is_on(&crate::uncap::PHYSICS_FIX));
        let choice = Choice::from_parts(None, env_of(&[("NV_UNCAP", "1")])).unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        assert!(choice.is_on(&crate::uncap::FIX));
        assert!(!choice.is_on(&crate::uncap::PHYSICS_FIX));
        let choice =
            Choice::from_parts(None, env_of(&[("NV_PRESET", "fixes"), ("NV_UNCAP", "0")])).unwrap();
        assert!(!choice.is_on(&crate::uncap::FIX));
        assert!(choice.is_on(&crate::uncap::PHYSICS_FIX));
    }

    /// A script that records and was never told about the preset still measures the original; to
    /// record the fixes it says so.
    #[test]
    fn a_recording_pins_vanilla_unless_it_names_a_preset() {
        let choice =
            Choice::from_parts(Some("Preset=fixes"), env_of(&[("NV_TRACE_LIST", "x")])).unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        let choice = Choice::from_parts(
            None,
            env_of(&[("NV_TRACE_LIST", "x"), ("NV_PRESET", "fixes")]),
        )
        .unwrap();
        assert_eq!(choice.preset, Preset::Fixes);
    }

    /// Scripted input and measurement variables make a test; the preview launcher's save folder
    /// doesn't.
    #[test]
    fn scripted_and_measured_runs_pin_vanilla() {
        for name in TEST_VARIABLES {
            let pairs: &'static [(&str, &str)] = Box::leak(Box::new([(*name, "x")]));
            let choice = Choice::from_parts(Some("Preset=fixes"), env_of(pairs)).unwrap();
            assert_eq!(choice.preset, Preset::Vanilla, "{name}");
            let both: &'static [(&str, &str)] =
                Box::leak(Box::new([(*name, "x"), ("NV_PRESET", "fixes")]));
            let choice = Choice::from_parts(None, env_of(both)).unwrap();
            assert_eq!(choice.preset, Preset::Fixes, "{name}");
        }
        for name in [
            "NV_MOVE_KEYS",
            "NV_MOVE_MOUSE",
            "NV_SOUND_RECORD",
            "NV_SHOT",
            "NV_TICK_COUNT",
        ] {
            assert!(TEST_VARIABLES.contains(&name), "{name}");
        }
        let choice = Choice::from_parts(None, env_of(&[("NV_SAVE_OUT", "x")])).unwrap();
        assert_eq!(choice.preset, Preset::Fixes);
    }

    /// A test runs the game's threads stepped unless its `NV_THREADS` names them, so no comparison,
    /// recording or measurement changes when the play default does; naming a preset doesn't
    /// unpin them. A plain start (and the launcher's) gets the play default.
    /// A run the viewer's command line marks as a test (`--movement-missing`, `--movement-probe`)
    /// runs stepped threads, unless `NV_THREADS` names them.
    #[test]
    fn a_test_by_its_command_line_pins_stepped_threads() {
        let mut choice = Choice::from_parts(None, no_env).unwrap();
        assert_eq!(choice.threads, PLAY_THREADS);
        choice.pin_test_threads(no_env);
        assert_eq!(choice.threads, "stepped");
        let named = env_of(&[("NV_THREADS", "real")]);
        let mut choice = Choice::from_parts(None, &named).unwrap();
        choice.pin_test_threads(&named);
        assert_eq!(choice.threads, "real");
    }

    #[test]
    fn scripted_and_measured_runs_pin_stepped_threads() {
        for name in TEST_VARIABLES {
            let pairs: &'static [(&str, &str)] = Box::leak(Box::new([(*name, "x")]));
            let choice = Choice::from_parts(None, env_of(pairs)).unwrap();
            assert_eq!(choice.threads, "stepped", "{name}");
            let fixes: &'static [(&str, &str)] =
                Box::leak(Box::new([(*name, "x"), ("NV_PRESET", "fixes")]));
            let choice = Choice::from_parts(None, env_of(fixes)).unwrap();
            assert_eq!(choice.threads, "stepped", "{name}");
            let real: &'static [(&str, &str)] =
                Box::leak(Box::new([(*name, "x"), ("NV_THREADS", "real")]));
            let choice = Choice::from_parts(None, env_of(real)).unwrap();
            assert_eq!(choice.threads, "real", "{name}");
        }
        let one_fix = Choice::from_parts(None, env_of(&[("NV_UNCAP", "1")])).unwrap();
        assert_eq!(one_fix.threads, "stepped");
        let plain = Choice::from_parts(Some("Preset=fixes"), no_env).unwrap();
        assert_eq!(plain.threads, PLAY_THREADS);
        let launcher = Choice::from_parts(
            None,
            env_of(&[("NV_SAVE_OUT", "x"), ("NV_PRESET", "fixes")]),
        )
        .unwrap();
        assert_eq!(launcher.threads, PLAY_THREADS);
        let empty = Choice::from_parts(None, env_of(&[("NV_THREADS", " ")])).unwrap();
        assert_eq!(empty.threads, PLAY_THREADS);
        assert_eq!(Choice::vanilla().threads, "stepped");
        for words in [PLAY_THREADS, "stepped"] {
            assert!(engine::thread_host::Scheme::from_name(Some(words)).is_ok());
        }
    }

    #[test]
    fn a_byte_order_mark_or_utf16_doesnt_hide_the_preset() {
        let text = "Borderless=1\r\nPreset=vanilla\r\n";
        let mut bom = vec![0xef, 0xbb, 0xbf];
        bom.extend_from_slice(text.as_bytes());
        let mut utf16le = vec![0xff, 0xfe];
        utf16le.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
        let mut utf16be = vec![0xfe, 0xff];
        utf16be.extend(text.encode_utf16().flat_map(u16::to_be_bytes));
        for bytes in [text.as_bytes().to_vec(), bom, utf16le, utf16be] {
            let decoded = decode_settings(&bytes);
            let choice = Choice::from_parts(Some(&decoded), no_env).unwrap();
            assert_eq!(choice.preset, Preset::Vanilla, "{bytes:02x?}");
        }
        // The very first line, where a mark would hide its key.
        let mut bom = vec![0xef, 0xbb, 0xbf];
        bom.extend_from_slice(b"Preset=vanilla\n");
        let choice = Choice::from_parts(Some(&decode_settings(&bom)), no_env).unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
    }

    #[test]
    fn a_settings_file_that_cant_be_read_stops_the_start() {
        let dir = std::env::temp_dir().join(format!("nv-fixes-settings-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // A folder where the file should be: it exists, and reading it fails.
        let err = Choice::from_file(Some(&dir), no_env).unwrap_err();
        assert!(err.contains("settings file"), "{err}");
        // An explicit preset makes the file moot.
        let choice = Choice::from_file(Some(&dir), env_of(&[("NV_PRESET", "vanilla")])).unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        // No such file: the defaults. A real one is read, UTF-16 included.
        let missing = dir.join("missing.ini");
        assert_eq!(
            Choice::from_file(Some(&missing), no_env).unwrap().preset,
            Preset::Fixes
        );
        let file = dir.join("settings.ini");
        let mut bytes = vec![0xff, 0xfe];
        bytes.extend(
            "Preset=vanilla\r\n"
                .encode_utf16()
                .flat_map(u16::to_le_bytes),
        );
        std::fs::write(&file, bytes).unwrap();
        assert_eq!(
            Choice::from_file(Some(&file), no_env).unwrap().preset,
            Preset::Vanilla
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// A test sets NV_PRESET and gets that, whatever a settings file in a folder above the program
    /// says, even one that doesn't parse.
    #[test]
    fn an_explicit_preset_beats_a_settings_file() {
        let choice = Choice::from_parts(
            Some("Preset=banana\nFrameLimit=fast\n"),
            env_of(&[("NV_PRESET", "vanilla"), ("NV_FRAME_LIMIT", "90")]),
        )
        .unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        assert_eq!(choice.frame_limit, 90);
        let choice =
            Choice::from_parts(Some("Preset=vanilla"), env_of(&[("NV_PRESET", "fixes")])).unwrap();
        assert_eq!(choice.preset, Preset::Fixes);
    }

    /// Menus at 120 unless the file or the environment says otherwise; its own switch turns it off,
    /// and vanilla binds nothing.
    #[test]
    fn the_menu_frame_limit_and_its_switch() {
        let limit = |text: Option<&str>, pairs: &'static [(&'static str, &'static str)]| {
            Choice::from_parts(text, env_of(pairs))
                .unwrap()
                .bound_menu_frame_limit()
        };
        assert_eq!(limit(None, &[]), Some(DEFAULT_MENU_FRAME_LIMIT));
        assert_eq!(limit(Some("MenuFrameLimit=60\n"), &[]), Some(60));
        assert_eq!(limit(Some("menuframelimit = 0\n"), &[]), Some(0));
        assert_eq!(limit(Some("MenuFrameLimit=\n"), &[]), Some(120));
        assert_eq!(
            limit(Some("MenuFrameLimit=60\n"), &[("NV_MENU_FRAME_LIMIT", "0")]),
            Some(0)
        );
        assert_eq!(
            limit(
                Some("MenuFrameLimit=fast\n"),
                &[("NV_MENU_FRAME_LIMIT", "90")]
            ),
            Some(90)
        );
        assert!(Choice::from_parts(Some("MenuFrameLimit=fast\n"), no_env).is_err());
        assert!(Choice::from_parts(None, env_of(&[("NV_MENU_FRAME_LIMIT", "-1")])).is_err());
        // Vanilla and the row's own switch: nothing bound, and the uncapped step stays.
        assert_eq!(Choice::vanilla().bound_menu_frame_limit(), None);
        assert_eq!(
            limit(Some("Preset=vanilla\nMenuFrameLimit=60\n"), &[]),
            None
        );
        let choice = Choice::from_parts(
            None,
            env_of(&[("NV_PRESET", "fixes"), ("NV_MENU_CAP", "0")]),
        )
        .unwrap();
        assert_eq!(choice.bound_menu_frame_limit(), None);
        assert!(choice.is_on(&crate::uncap::FIX));
        assert!(!choice.describe().contains("menu"), "{}", choice.describe());
        // The start-up line says it, and says nothing when the uncapped step is off.
        let line = Choice::from_parts(None, no_env).unwrap().describe();
        assert!(
            line.ends_with(&format!(", menu limit 120; threads {PLAY_THREADS}")),
            "{line}"
        );
        let line = Choice::from_parts(None, env_of(&[("NV_PRESET", "fixes"), ("NV_UNCAP", "0")]))
            .unwrap()
            .describe();
        assert!(!line.contains("menu limit"), "{line}");
    }

    /// The fixes preset turns the dragged-body row on; vanilla and its own switch leave it off.
    #[test]
    fn the_grab_row_follows_the_preset_and_its_switch() {
        let fixes = Choice::from_parts(None, no_env).unwrap();
        assert!(fixes.is_on(&crate::uncap::GRAB_FIX));
        assert!(!Choice::vanilla().is_on(&crate::uncap::GRAB_FIX));
        let off = Choice::from_parts(
            None,
            env_of(&[("NV_PRESET", "fixes"), ("NV_GRAB_PUSH", "0")]),
        )
        .unwrap();
        assert!(!off.is_on(&crate::uncap::GRAB_FIX));
        assert!(off.is_on(&crate::uncap::FIX));
    }

    #[test]
    fn vanilla_and_a_log_line() {
        assert_eq!(
            Choice::vanilla().describe(),
            "Preset vanilla: no fix on; threads stepped"
        );
        assert!(FIXES.iter().all(|f| !Choice::vanilla().is_on(f)));
        let choice = Choice::from_parts(Some("FrameLimit=90"), no_env).unwrap();
        let line = choice.describe();
        assert!(line.starts_with("Preset fixes: "), "{line}");
        assert!(
            line.ends_with(&format!(", frame limit 90; threads {PLAY_THREADS}")),
            "{line}"
        );
    }

    /// EXTENDER-HOOKS R2 and docs/ENGINE_CRATE.md, "Typed code": what a fix overrides is listed, so
    /// typed code keeps calling it by address.
    #[test]
    fn every_fix_address_is_listed_as_overridable() {
        let table = include_str!("../../engine-core/data/overridable.tsv");
        for fix in FIXES {
            assert!(
                !fix.addresses.is_empty() || NO_ADDRESS.contains(&fix.name),
                "{}",
                fix.name
            );
            for address in fix.addresses {
                let hex = format!("{address:08x}\t");
                assert!(
                    table.lines().any(|line| line.starts_with(&hex)),
                    "{} registers at {address:08x}, which overridable.tsv doesn't list",
                    fix.name
                );
            }
        }
    }

    /// Vanilla binds nothing and the original's sleep stays; the fixes preset's step doesn't sleep.
    #[test]
    fn the_timer_step_in_both_modes() {
        use engine::units::bscore::bstimer::BSTimer;
        const SLEEP: u32 = 0x00fd_f104;
        let run = |choice: &Choice| {
            let mut e = Engine::new();
            e.map(0x0101_2060, 8);
            e.map(0x0101_6978, 8);
            e.mem.set_u64(0x0101_6978, 0.001f64.to_bits());
            e.map(0x011a_c3a0, 4);
            e.mem.set_f32(0x011a_c3a0, 1.0);
            let slept = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let log = slept.clone();
            e.register_double(SLEEP, move |e, a| {
                log.borrow_mut().push(a[0]);
                e.advance_clock_us(u64::from(a[0]) * 1000);
                Ret::default()
            });
            install(&mut e, choice);
            let timer = e.new_object::<BSTimer>();
            e.clock_ms = 1000;
            e.set(timer, BSTimer::iLastTime, 1000);
            e.advance_clock_us(4000);
            e.call(0x00aa_4ee0, &[timer.addr(), e.clock_ms]);
            let delta = e.get(timer, BSTimer::fDelta);
            let slept = slept.borrow().clone();
            (slept, delta)
        };
        assert_eq!(run(&Choice::vanilla()), (vec![6], 0.01));
        let fixes = Choice::from_parts(None, no_env).unwrap();
        // The first step answers as the original (10 ms); no real sleep, a pass for the other threads.
        assert_eq!(run(&fixes), (vec![0], 0.01));
    }

    /// Vanilla's translation of `Move` is what's registered; the fixes preset's differs only at
    /// the addresses its rows name, which the rows' tests cover.
    #[test]
    fn rows_are_unique_by_address() {
        let mut seen = std::collections::BTreeSet::new();
        for address in FIXES.iter().flat_map(|f| f.addresses) {
            assert!(seen.insert(*address), "{address:08x} has two owners");
        }
    }

    #[test]
    fn candidates_are_beside_the_program_and_above_it() {
        let paths = candidates(Path::new("game/app"));
        assert_eq!(paths[0], Path::new("game/app/settings.ini"));
        assert_eq!(paths[1], Path::new("game/settings.ini"));
    }
}
