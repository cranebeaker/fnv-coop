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
    IMOD_EVENT_FORM_CHECK,
    INVENTORY_REFERENCE_CHECKS,
    SCRIPT_SIZE_CAPS,
    SCRIPT_NULL_GUARD,
    SCRIPT_CYCLE_GUARD,
    SCRIPT_LIST_GUARD,
    SHADER_WARM,
    IO_WAIT_ENDS_ON_COMPLETION,
];

/// The IO manager's wait for queued loads (`00c3dfa0`: main at a cell border, a load, a save) sleeps
/// 50 ms a turn while the completed work is short of what is in flight; here the sleep ends as
/// soon as it is not (looked at after each 1 ms sleep; 50 ms by the clock or 50 looks at most), so
/// the wait for queued loads ends when they are done, not at its next 50 ms step. Vanilla keeps the
/// 50 ms steps.
pub const IO_WAIT_ENDS_ON_COMPLETION: Fix = Fix {
    name: "IO wait ends on completion",
    env: "NV_IO_WAIT",
    addresses: &[0x00c3_dfa0],
    summary: "the wait for queued loads (a cell border, a load, a save) ends when they are done, not at its next 50 ms step",
    install: install_io_wait,
};

fn install_io_wait(e: &mut Engine) {
    use engine::units::bsmain::iomanager::fn_00c3dfa0_waking_early;
    let (at, f) = engine::entry!(0x00c3_dfa0, fn_00c3dfa0_waking_early(Ptr, i32));
    e.register(at, f);
}

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

/// The driver compiles a Direct3D 9 shader at its first draw: the first shot's frame took 25-46 ms
/// (7 shaders the game made at start-up, first drawn with at the shot). With the row the real
/// backend draws once with each shader the game makes, into a 1x1 target of its own, a few
/// milliseconds after each `Present` (`engine_platform::d3d9backend`): the compiles happen in the
/// menus and loading screens instead. No exe address; nothing the game draws changes.
pub const SHADER_WARM: Fix = Fix {
    name: "shaders compiled ahead of first use",
    env: "NV_SHADER_WARM",
    addresses: &[],
    summary:
        "the driver compiles each of the game's shaders in the menus, not at its first draw in play",
    install: install_shader_warm,
};

fn install_shader_warm(e: &mut Engine) {
    #[cfg(windows)]
    engine_platform::d3d9real::want_shader_warm(e, true);
    #[cfg(not(windows))]
    let _ = e;
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

pub const IMOD_EVENT_FORM_CHECK: Fix = Fix {
    name: "image-space event form check",
    env: "NV_NVSE_IMOD_CHECK",
    addresses: &[],
    summary:
        "OnRemoveIMOD only for a modifier form's instance that ran out (not depth of field or blur)",
    install: install_imod_event_form_check,
};

fn install_imod_event_form_check(e: &mut Engine) {
    nvse::call_sites::set_imod_form_check(e, true);
}

pub const INVENTORY_REFERENCE_CHECKS: Fix = Fix {
    name: "inventory reference checks",
    env: "NV_NVSE_INVREF_CHECK",
    addresses: &[],
    summary: "a removed stack leaves the container's own entry before its data is freed; a sale \
              ends with its event; a container entry without a form is passed over",
    install: install_inventory_reference_checks,
};

fn install_inventory_reference_checks(e: &mut Engine) {
    nvse::invref::set_checks(e, true);
}

pub const SCRIPT_SIZE_CAPS: Fix = Fix {
    name: "script array and string size caps",
    env: "NV_NVSE_SIZE_CAPS",
    addresses: &[],
    summary:
        "arrays the script commands make or grow stop at 8,388,608 elements; an empty text replaced a given number of times stops at 64 MiB; a format string the C runtime refuses or has no memory for is made safely instead of ending the game",
    install: install_script_size_caps,
};

fn install_script_size_caps(e: &mut Engine) {
    nvse::commands::array::set_fix(e, nvse::commands::array::SIZE_CAPS, true);
}

pub const SCRIPT_NULL_GUARD: Fix = Fix {
    name: "script missing array guard",
    env: "NV_NVSE_NULL_GUARD",
    addresses: &[],
    summary:
        "ar_Count and ar_GetNth on a missing array answer 0, and a format string's string outside the game's memory ends the text, instead of stopping the game",
    install: install_script_null_guard,
};

fn install_script_null_guard(e: &mut Engine) {
    nvse::commands::array::set_fix(e, nvse::commands::array::NULL_GUARD, true);
}

pub const SCRIPT_CYCLE_GUARD: Fix = Fix {
    name: "script array cycle guard",
    env: "NV_NVSE_CYCLE_GUARD",
    addresses: &[],
    summary:
        "a deep copy, deep compare or dump of an array that holds itself ends where it meets itself again, instead of running out of stack",
    install: install_script_cycle_guard,
};

fn install_script_cycle_guard(e: &mut Engine) {
    nvse::commands::array::set_fix(e, nvse::commands::array::CYCLE_GUARD, true);
}

pub const SCRIPT_LIST_GUARD: Fix = Fix {
    name: "script inventory list guard",
    env: "NV_NVSE_LIST_GUARD",
    addresses: &[],
    summary:
        "a container list that runs back into itself ends the walk of the inventory commands instead of stopping the game",
    install: install_script_list_guard,
};

fn install_script_list_guard(e: &mut Engine) {
    nvse::commands::array::set_fix(e, nvse::commands::array::LIST_GUARD, true);
}

/// The rows that register nothing at an exe address: the menu limit paces the uncapped step;
/// the extender's rows set flags `crates/nvse` reads.
#[cfg(test)]
const NO_ADDRESS: &[&str] = &[
    crate::uncap::MENU_FIX.name,
    KEEP_COSAVE_PLUGINS.name,
    SCRIPT_ERROR_GUARD.name,
    IMOD_EVENT_FORM_CHECK.name,
    INVENTORY_REFERENCE_CHECKS.name,
    SCRIPT_SIZE_CAPS.name,
    SCRIPT_NULL_GUARD.name,
    SCRIPT_CYCLE_GUARD.name,
    SCRIPT_LIST_GUARD.name,
    SHADER_WARM.name,
];

/// Menus' frames a second at most when nothing says otherwise (the maintainer, 2026-11-07).
pub const DEFAULT_MENU_FRAME_LIMIT: u32 = 120;

/// How the game's threads run in play when `NV_THREADS` names nothing ([`Choice::threads`]):
/// real, side by side on their own cores (`engine::real_threads`), since the evidence of
/// measure/real-threads (the maintainer's gate, 2026-11-08); `NV_THREADS=stepped` runs them one
/// at a time. A test never takes this: it runs stepped unless it names threads.
pub const PLAY_THREADS: &str = "real";

/// Whether the real Direct3D 9 calls run on a render thread of their own in play when
/// `NV_RENDER_THREAD` names nothing ([`Choice::render_thread`]): on, since the measurements of
/// measure/render-thread (town +14%, the crowded street +16%, the same calls in the same order;
/// #347); `NV_RENDER_THREAD=0` makes them on the game's threads, as the original does. A test never
/// takes this: it runs without the render thread unless it names it.
pub const PLAY_RENDER_THREAD: bool = true;

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
    pub(crate) fn parse(word: &str) -> Option<Preset> {
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
    /// Whether the real Direct3D 9 calls run on a render thread of their own
    /// (`engine_platform::d3d9real::want_render_thread`): `NV_RENDER_THREAD=1` or `0` when set;
    /// else off for a test (as its threads are stepped); else [`PLAY_RENDER_THREAD`].
    pub render_thread: bool,
    /// Whether the game's window has no frame and fills the screen
    /// (`engine_platform::d3d9real::want_borderless`): `NV_BORDERLESS` when set (`1` on, else
    /// off); else `Fallout.ini`'s `[nv-rs Display] bBorderless` (never for a test); `None` when
    /// neither says, and the window is framed, as the game makes it.
    pub borderless: Option<bool>,
    /// The one log line naming `Fallout.ini` values that could not be used
    /// ([`crate::game_ini::Keys::bad_line`]): the default stood for each.
    pub ini_note: Option<String>,
}

impl Choice {
    /// The original's behaviour, whatever the environment and the settings file say: what
    /// [`crate::Host::start`] runs (stepped threads, no render thread too).
    pub fn vanilla() -> Choice {
        Choice {
            preset: Preset::Vanilla,
            frame_limit: 0,
            menu_frame_limit: DEFAULT_MENU_FRAME_LIMIT,
            overrides: Vec::new(),
            threads: "stepped".to_string(),
            render_thread: false,
            borderless: None,
            ini_note: None,
        }
    }

    /// The choice from the run's `Fallout.ini` ([`crate::game_ini::path`], its `[nv-rs ...]`
    /// sections), the settings file ([`settings_file`], the fallback until it is migrated) and the
    /// environment, which wins: `NV_PRESET=vanilla|fixes`, `NV_FRAME_LIMIT=n`,
    /// `NV_MENU_FRAME_LIMIT=n`, `NV_THREADS`, `NV_RENDER_THREAD`, `NV_BORDERLESS` and each
    /// [`Fix::env`]. A settings file or variable it can't read stops the start with a message, so
    /// a misspelt pin never runs the wrong game; a `Fallout.ini` value it can't use leaves the
    /// default and is named in one log line ([`Choice::ini_note`]). Only a launcher's play run
    /// reads `Fallout.ini` ([`is_play_run`]).
    pub fn read() -> Result<Choice, String> {
        Choice::read_with(false)
    }

    /// [`Choice::read`] for a run its command line marks as a test (`command_line_test`: the
    /// viewer's `--movement-missing` and `--movement-probe`, which record or go on past
    /// untranslated functions on the main thread's engine only) or not. A marked run never reads
    /// `Fallout.ini`'s keys, and its threads run stepped unless `NV_THREADS` names them
    /// ([`Choice::pin_test_threads`]).
    pub fn read_with(command_line_test: bool) -> Result<Choice, String> {
        let env = |name: &str| std::env::var(name).ok();
        let game_ini = if command_line_test {
            None
        } else {
            crate::game_ini::path()
        };
        let mut choice = Choice::load(
            settings_file().as_deref(),
            game_ini.as_deref(),
            env,
            command_line_test,
        )?;
        if command_line_test {
            choice.pin_test_threads(env);
        }
        Ok(choice)
    }

    /// [`Choice::read`] on a given settings file (none: no file) and environment, with no
    /// `Fallout.ini`.
    pub fn from_file(
        file: Option<&Path>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Choice, String> {
        Choice::load(file, None, env, false)
    }

    /// [`Choice::read`] on a given settings file, `Fallout.ini` (none: no file) and environment.
    pub fn from_files(
        file: Option<&Path>,
        game_ini: Option<&Path>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Choice, String> {
        Choice::load(file, game_ini, env, false)
    }

    /// The files' texts, then [`Choice::build`]. A settings file that exists and can't be read
    /// stops the start, since its `Preset=vanilla` would be lost; an explicit `NV_PRESET` makes
    /// the file moot, so then it doesn't matter. A `Fallout.ini` that can't be read is passed
    /// over, as the game passes over a missing key.
    fn load(
        file: Option<&Path>,
        game_ini: Option<&Path>,
        env: impl Fn(&str) -> Option<String>,
        command_line_test: bool,
    ) -> Result<Choice, String> {
        let ini = game_ini
            .and_then(|p| std::fs::read(p).ok())
            .map(|bytes| decode_settings(&bytes));
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
        Choice::build(text.as_deref(), ini.as_deref(), env, command_line_test)
    }

    /// [`Choice::read`] on given settings text and environment, with no `Fallout.ini`.
    pub fn from_parts(
        settings: Option<&str>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Choice, String> {
        Choice::build(settings, None, env, false)
    }

    /// [`Choice::read`] on given settings text, `Fallout.ini` text and environment.
    pub fn from_texts(
        settings: Option<&str>,
        game_ini: Option<&str>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Choice, String> {
        Choice::build(settings, game_ini, env, false)
    }

    /// The choice. Lowest to highest: the built-in default, the preset, `Fallout.ini`'s
    /// `[nv-rs ...]` keys, then the environment. settings.ini, the fallback until it is migrated,
    /// sits under `Fallout.ini`: a line of it that `Fallout.ini` or the environment overrides
    /// isn't even read. `Fallout.ini` counts only in a launcher's play run ([`is_play_run`]) that
    /// is no test, by its variables or by its command line (`command_line_test`).
    fn build(
        settings: Option<&str>,
        game_ini: Option<&str>,
        env: impl Fn(&str) -> Option<String>,
        command_line_test: bool,
    ) -> Result<Choice, String> {
        let mut overrides = Vec::new();
        for fix in FIXES {
            match env(fix.env).as_deref().map(str::trim) {
                None => {}
                Some("1") => overrides.push((fix.env, true)),
                Some("0") => overrides.push((fix.env, false)),
                Some(other) => return Err(format!("{}={other}: use 0 or 1", fix.env)),
            }
        }
        // A recording, a scripted run, a measurement or a single fix's switch is a test: with no
        // NV_PRESET of its own it runs on vanilla, so a script that was never told about the preset
        // still measures the original. Only a plain start (or the launcher, whose NV_SAVE_OUT is a
        // real player's) gets the fixes. A test runs the game's threads stepped the same way,
        // unless its NV_THREADS names them, and without the render thread unless its
        // NV_RENDER_THREAD names it.
        let test = TEST_VARIABLES.iter().any(|name| env(name).is_some()) || !overrides.is_empty();
        // Only a launcher's play run reads the player's `[nv-rs ...]` keys: every scripted, gated
        // or measured run has pinned defaults and the environment only, so a player's file can
        // never change its result (measure/ini/DESIGN.md).
        let ini = match game_ini {
            Some(text) if is_play_run(&env) && !test && !command_line_test => {
                crate::game_ini::Keys::read(text)
            }
            _ => crate::game_ini::Keys::default(),
        };
        let mut choice = Choice {
            preset: Preset::Fixes,
            frame_limit: 0,
            menu_frame_limit: DEFAULT_MENU_FRAME_LIMIT,
            overrides: Vec::new(),
            threads: PLAY_THREADS.to_string(),
            render_thread: PLAY_RENDER_THREAD,
            borderless: ini.borderless,
            ini_note: ini.bad_line(),
        };
        for line in settings.unwrap_or("").lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            if key.starts_with(['#', ';']) {
                continue;
            }
            // The environment and Fallout.ini win, so a line they override isn't even read (a
            // test's pin beats a settings file above the program, whatever that holds).
            if key.eq_ignore_ascii_case("Preset") {
                if env("NV_PRESET").is_none() && ini.preset.is_none() {
                    choice.preset = parse_preset(value, "settings.ini's Preset")?;
                }
            } else if key.eq_ignore_ascii_case("FrameLimit") {
                if env("NV_FRAME_LIMIT").is_some() || ini.frame_limit.is_some() {
                    continue;
                }
                choice.frame_limit = parse_limit(value, "settings.ini's FrameLimit")?;
            } else if key.eq_ignore_ascii_case("MenuFrameLimit") {
                // An empty line is the default, as the launcher reads it.
                if env("NV_MENU_FRAME_LIMIT").is_some()
                    || ini.menu_frame_limit.is_some()
                    || value.trim().is_empty()
                {
                    continue;
                }
                choice.menu_frame_limit = parse_limit(value, "settings.ini's MenuFrameLimit")?;
            }
        }
        if let Some(preset) = ini.preset {
            choice.preset = preset;
        }
        if let Some(limit) = ini.frame_limit {
            choice.frame_limit = limit;
        }
        if let Some(limit) = ini.menu_frame_limit {
            choice.menu_frame_limit = limit;
        }
        if let Some(stepped) = ini.stepped_threads {
            choice.threads = if stepped { "stepped" } else { PLAY_THREADS }.to_string();
        }
        if let Some(on) = ini.render_thread {
            choice.render_thread = on;
        }
        if let Some(value) = env("NV_FRAME_LIMIT") {
            choice.frame_limit = parse_limit(&value, "NV_FRAME_LIMIT")?;
        }
        if let Some(value) = env("NV_MENU_FRAME_LIMIT") {
            choice.menu_frame_limit = parse_limit(&value, "NV_MENU_FRAME_LIMIT")?;
        }
        // A fix's variable first, so it wins over the file's line for the same fix (`is_on`
        // takes the first); a fix neither names follows the preset.
        choice.overrides = overrides;
        choice.overrides.extend(ini.fixes);
        if let Some(value) = env("NV_BORDERLESS").filter(|v| !v.trim().is_empty()) {
            choice.borderless = Some(value.trim() == "1");
        }
        if test {
            choice.preset = Preset::Vanilla;
            choice.threads = "stepped".to_string();
            choice.render_thread = false;
        }
        if let Some(value) = env("NV_PRESET") {
            choice.preset = parse_preset(&value, "NV_PRESET")?;
        }
        if let Some(value) = env("NV_THREADS").filter(|v| !v.trim().is_empty()) {
            choice.threads = value;
        }
        match env("NV_RENDER_THREAD").as_deref().map(str::trim) {
            None | Some("") => {}
            Some("1") => choice.render_thread = true,
            Some("0") => choice.render_thread = false,
            Some(other) => return Err(format!("NV_RENDER_THREAD={other}: use 0 or 1")),
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

    /// Whether this fix runs: its environment variable if set, else its `Fallout.ini` line
    /// (`[nv-rs Fixes]`, not commented out), else the preset.
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

/// Whether a launcher started this run for play: `NV_SETTINGS` set (Play.exe, and the play
/// folder's Play.cmd through apply-settings.sh). No scripted, gated or measured run sets it
/// (`NV_SAVE_OUT` is not a sign: scripts that make saves set it too). Only such a run reads
/// `Fallout.ini`'s `[nv-rs ...]` keys, and then only when it is no test ([`TEST_VARIABLES`], a
/// fix's own variable, a test's command line).
pub fn is_play_run(env: impl Fn(&str) -> Option<String>) -> bool {
    env("NV_SETTINGS").is_some()
}

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

    /// The render thread is play's default; settings.ini's `NV_RENDER_THREAD=0` line (which the
    /// launcher hands over as the variable) turns it off. A test runs without it unless its
    /// `NV_RENDER_THREAD` names it, so no comparison, recording or measurement changes when the
    /// play default does; naming a preset doesn't turn it on.
    #[test]
    fn the_render_thread_is_plays_and_a_test_names_it() {
        let plain = Choice::from_parts(Some("Preset=fixes"), no_env).unwrap();
        assert_eq!(plain.render_thread, PLAY_RENDER_THREAD);
        let vanilla = Choice::from_parts(Some("Preset=vanilla"), no_env).unwrap();
        assert_eq!(vanilla.render_thread, PLAY_RENDER_THREAD);
        let launcher = Choice::from_parts(
            None,
            env_of(&[("NV_SAVE_OUT", "x"), ("NV_PRESET", "fixes")]),
        )
        .unwrap();
        assert_eq!(launcher.render_thread, PLAY_RENDER_THREAD);
        let off = Choice::from_parts(None, env_of(&[("NV_RENDER_THREAD", "0")])).unwrap();
        assert!(!off.render_thread);
        assert_eq!(off.preset, Preset::Fixes, "a run setting, not a test");
        let on = Choice::from_parts(None, env_of(&[("NV_RENDER_THREAD", " 1 ")])).unwrap();
        assert!(on.render_thread);
        let empty = Choice::from_parts(None, env_of(&[("NV_RENDER_THREAD", "")])).unwrap();
        assert_eq!(empty.render_thread, PLAY_RENDER_THREAD);
        let wrong = Choice::from_parts(None, env_of(&[("NV_RENDER_THREAD", "yes")]));
        assert!(wrong.unwrap_err().contains("NV_RENDER_THREAD=yes"));
        for name in TEST_VARIABLES {
            let pairs: &'static [(&str, &str)] = Box::leak(Box::new([(*name, "x")]));
            let choice = Choice::from_parts(None, env_of(pairs)).unwrap();
            assert!(!choice.render_thread, "{name}");
            let fixes: &'static [(&str, &str)] =
                Box::leak(Box::new([(*name, "x"), ("NV_PRESET", "fixes")]));
            let choice = Choice::from_parts(None, env_of(fixes)).unwrap();
            assert!(!choice.render_thread, "{name}");
            let named: &'static [(&str, &str)] =
                Box::leak(Box::new([(*name, "x"), ("NV_RENDER_THREAD", "1")]));
            let choice = Choice::from_parts(None, env_of(named)).unwrap();
            assert!(choice.render_thread, "{name}");
        }
        let one_fix = Choice::from_parts(None, env_of(&[("NV_UNCAP", "1")])).unwrap();
        assert!(!one_fix.render_thread);
        assert!(!Choice::vanilla().render_thread);
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

    const PLAYER_INI: &str = "[General]\nsLanguage=ENGLISH\n\
        [nv-rs General]\nsPreset=vanilla\n\
        [nv-rs Display]\nbBorderless=1\niFrameLimit=90\niMenuFrameLimit=60\n\
        [nv-rs Threads]\nbSteppedThreads=1\nbRenderThread=0\n";

    /// A launcher's play run (its `NV_SETTINGS`) with these variables too.
    fn with_ini(
        settings: Option<&str>,
        ini: &str,
        pairs: &'static [(&'static str, &'static str)],
    ) -> Choice {
        let env = env_of(pairs);
        let play = move |name: &str| {
            (name == "NV_SETTINGS")
                .then(|| "x".to_string())
                .or(env(name))
        };
        Choice::from_texts(settings, Some(ini), play).unwrap()
    }

    /// Only a launcher's play run reads the file: a run with only a run setting, a preset, the
    /// renderer or a save folder (a script making saves) named, or one its command line marks as
    /// a test, never does.
    #[test]
    fn only_a_launchers_play_run_reads_fallout_ini() {
        let reads = |pairs: &'static [(&'static str, &'static str)], command_line_test: bool| {
            Choice::build(None, Some(PLAYER_INI), env_of(pairs), command_line_test)
                .unwrap()
                .borderless
                == Some(true)
        };
        assert!(!reads(&[("NV_SAVE_OUT", "x")], false));
        assert!(reads(&[("NV_SETTINGS", "x")], false));
        assert!(reads(&[("NV_SETTINGS", "x"), ("NV_SAVE_OUT", "x")], false));
        assert!(reads(
            &[("NV_SETTINGS", "x"), ("NV_PRESET", "fixes")],
            false
        ));
        assert!(!reads(&[], false));
        assert!(!reads(&[("NV_PRESET", "fixes")], false));
        assert!(!reads(&[("NV_FRAME_LIMIT", "60")], false));
        assert!(!reads(&[("NV_THREADS", "real")], false));
        assert!(!reads(&[("NV_RENDER_THREAD", "1")], false));
        assert!(!reads(&[("NV_D3D9", "real"), ("NV_SOUND", "out")], false));
        // The viewer's --movement-missing and --movement-probe, even in a play run.
        assert!(!reads(&[("NV_SETTINGS", "x")], true));
        let marked = Choice::build(
            None,
            Some(PLAYER_INI),
            env_of(&[("NV_SETTINGS", "x")]),
            true,
        )
        .unwrap();
        assert_eq!(marked, Choice::from_parts(None, no_env).unwrap());
        assert!(is_play_run(env_of(&[("NV_SETTINGS", "")])));
        assert!(!is_play_run(env_of(&[("NV_SAVE_OUT", "x")])));
        assert!(!is_play_run(env_of(&[("NV_D3D9", "real")])));
    }

    /// Play.exe passes Fallout.ini's player switches only when the game would read the file, so
    /// its list of what makes a test must be the host's (scripts/launcher/src/settings.rs).
    #[test]
    fn the_launcher_knows_what_makes_a_test() {
        let launcher = include_str!("../../../scripts/launcher/src/settings.rs");
        let list = launcher
            .split("pub const TEST_SWITCHES")
            .nth(1)
            .and_then(|rest| rest.split("];").next())
            .expect("TEST_SWITCHES in the launcher's settings.rs");
        let names: Vec<&str> = list
            .split('"')
            .skip(1)
            .step_by(2)
            .filter(|w| w.starts_with("NV_"))
            .collect();
        let mut host: Vec<&str> = TEST_VARIABLES.to_vec();
        host.extend(FIXES.iter().map(|f| f.env));
        host.sort_unstable();
        let mut names = names;
        names.sort_unstable();
        assert_eq!(names, host);
    }

    /// Fallout.ini's keys over the built-in defaults; nothing in the file is the defaults.
    #[test]
    fn fallout_ini_sets_the_player_options() {
        let choice = with_ini(None, PLAYER_INI, &[]);
        assert_eq!(choice.preset, Preset::Vanilla);
        assert_eq!(choice.borderless, Some(true));
        assert_eq!(choice.frame_limit, 90);
        assert_eq!(choice.menu_frame_limit, 60);
        assert_eq!(choice.threads, "stepped");
        assert!(!choice.render_thread);
        assert_eq!(choice.ini_note, None);
        let plain = with_ini(None, "[General]\nbUseThreadedAI=1\n", &[]);
        assert_eq!(plain, Choice::from_parts(None, no_env).unwrap());
        assert_eq!(plain.borderless, None);
        let real = with_ini(None, "[nv-rs Threads]\nbSteppedThreads=0\n", &[]);
        assert_eq!(real.threads, PLAY_THREADS);
    }

    /// Lowest to highest: default, preset, settings.ini (the fallback), Fallout.ini, environment.
    #[test]
    fn the_environment_beats_fallout_ini_which_beats_settings_ini() {
        let settings = "Preset=fixes\nFrameLimit=30\nMenuFrameLimit=45\n";
        let choice = with_ini(Some(settings), PLAYER_INI, &[]);
        assert_eq!(choice.preset, Preset::Vanilla);
        assert_eq!((choice.frame_limit, choice.menu_frame_limit), (90, 60));
        // settings.ini still decides what Fallout.ini doesn't say.
        let choice = with_ini(Some(settings), "[nv-rs Display]\niFrameLimit=90\n", &[]);
        assert_eq!(choice.preset, Preset::Fixes);
        assert_eq!((choice.frame_limit, choice.menu_frame_limit), (90, 45));
        // A settings.ini line Fallout.ini overrides isn't read, so it can't stop the start.
        let choice = with_ini(Some("Preset=banana\nFrameLimit=fast\n"), PLAYER_INI, &[]);
        assert_eq!(choice.preset, Preset::Vanilla);
        // The environment over the file, key by key.
        let choice = with_ini(
            None,
            PLAYER_INI,
            &[
                ("NV_PRESET", "fixes"),
                ("NV_FRAME_LIMIT", "144"),
                ("NV_MENU_FRAME_LIMIT", "0"),
                ("NV_THREADS", "real"),
                ("NV_RENDER_THREAD", "1"),
                ("NV_BORDERLESS", "0"),
            ],
        );
        assert_eq!(choice.preset, Preset::Fixes);
        assert_eq!((choice.frame_limit, choice.menu_frame_limit), (144, 0));
        assert_eq!(choice.threads, "real");
        assert!(choice.render_thread);
        assert_eq!(choice.borderless, Some(false));
        // An empty variable says nothing; the file decides.
        let choice = with_ini(None, PLAYER_INI, &[("NV_BORDERLESS", "")]);
        assert_eq!(choice.borderless, Some(true));
        let choice = Choice::from_parts(None, env_of(&[("NV_BORDERLESS", "1")])).unwrap();
        assert_eq!(choice.borderless, Some(true));
    }

    /// A commented fix line leaves the preset deciding; an uncommented one overrides that fix
    /// alone, under either preset; the fix's variable beats the line.
    #[test]
    fn a_fix_line_overrides_its_row_only_when_uncommented() {
        let commented = "[nv-rs Fixes]\n;bGrabPush=0\n;bUncap=1\n";
        let choice = with_ini(None, commented, &[]);
        assert!(FIXES.iter().all(|f| choice.is_on(f)));
        let vanilla = format!("[nv-rs General]\nsPreset=vanilla\n{commented}");
        let choice = with_ini(None, &vanilla, &[]);
        assert!(FIXES.iter().all(|f| !choice.is_on(f)));

        let off = with_ini(None, "[nv-rs Fixes]\nbGrabPush=0\n", &[]);
        assert!(!off.is_on(&crate::uncap::GRAB_FIX));
        assert!(off.is_on(&crate::uncap::FIX));
        assert_eq!(off.preset, Preset::Fixes, "a player's line is not a test");
        assert_eq!(off.threads, PLAY_THREADS);
        let on = with_ini(
            None,
            "[nv-rs General]\nsPreset=vanilla\n[nv-rs Fixes]\nbUncap=1\n",
            &[],
        );
        assert!(on.is_on(&crate::uncap::FIX));
        assert!(!on.is_on(&crate::uncap::PHYSICS_FIX));
        // The variable wins over the line (it also makes the run a test, which then names its
        // preset).
        let pinned = with_ini(
            None,
            "[nv-rs Fixes]\nbGrabPush=0\n",
            &[("NV_PRESET", "fixes"), ("NV_GRAB_PUSH", "1")],
        );
        assert!(pinned.is_on(&crate::uncap::GRAB_FIX));
        // A value it can't use: the preset decides and the one log line names it.
        let bad = with_ini(None, "[nv-rs Fixes]\nbGrabPush=off\n", &[]);
        assert!(bad.is_on(&crate::uncap::GRAB_FIX));
        assert_eq!(
            bad.ini_note.as_deref(),
            Some("Fallout.ini's nv-rs settings: not used, the default instead: bGrabPush=off")
        );
    }

    /// A scripted or measured run never reads the player's [nv-rs ...] keys: the same choice as
    /// with no file at all, whatever the file says (a fix line included).
    #[test]
    fn a_test_run_ignores_the_players_fallout_ini() {
        let ini = format!("{PLAYER_INI}[nv-rs Fixes]\nbUncap=1\nbGrabPush=1\n");
        for name in TEST_VARIABLES {
            for pairs in [
                vec![(*name, "x")],
                vec![(*name, "x"), ("NV_PRESET", "fixes")],
            ] {
                let pairs: &'static [(&str, &str)] = Box::leak(pairs.into_boxed_slice());
                let with = with_ini(None, &ini, pairs);
                let without = Choice::from_parts(None, env_of(pairs)).unwrap();
                assert_eq!(with, without, "{name}");
                assert_eq!(with.borderless, None, "{name}");
            }
        }
        // A single fix's variable makes a test too.
        let with = with_ini(None, &ini, &[("NV_UNCAP", "1")]);
        assert_eq!(with.preset, Preset::Vanilla);
        assert!(!with.is_on(&crate::uncap::GRAB_FIX));
        assert_eq!(with.frame_limit, 0);
        assert_eq!(with.threads, "stepped");
        // The launcher's own variables don't.
        let play = with_ini(None, &ini, &[("NV_SAVE_OUT", "x"), ("NV_D3D9", "real")]);
        assert_eq!(play.preset, Preset::Vanilla);
        assert_eq!(play.frame_limit, 90);
        assert!(play.is_on(&crate::uncap::FIX));
    }

    /// The file is read where the run's Documents are; a missing or unreadable one is passed
    /// over, never a stop.
    #[test]
    fn fallout_ini_from_disk() {
        let dir = std::env::temp_dir().join(format!("nv-fixes-game-ini-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("Fallout.ini");
        let mut bytes = vec![0xef, 0xbb, 0xbf];
        bytes.extend_from_slice(b"[nv-rs General]\r\nsPreset=vanilla\r\n");
        std::fs::write(&file, bytes).unwrap();
        let play = env_of(&[("NV_SETTINGS", "x")]);
        let choice = Choice::from_files(None, Some(&file), &play).unwrap();
        assert_eq!(choice.preset, Preset::Vanilla);
        let missing = Choice::from_files(None, Some(&dir.join("missing.ini")), &play).unwrap();
        assert_eq!(missing.preset, Preset::Fixes);
        // A folder where the file should be: it can't be read, and the start goes on.
        let folder = Choice::from_files(None, Some(&dir), &play).unwrap();
        assert_eq!(folder.preset, Preset::Fixes);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn candidates_are_beside_the_program_and_above_it() {
        let paths = candidates(Path::new("game/app"));
        assert_eq!(paths[0], Path::new("game/app/settings.ini"));
        assert_eq!(paths[1], Path::new("game/settings.ini"));
    }
}
