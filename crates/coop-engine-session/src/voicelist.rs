//! voicelist: who is speaking, each player's mute and volume by name, and the in-game players
//! list (docs/coop/SPECS.md, "voice list").
//!
//! - [`Speaking`]: a player speaks while their voice frames arrive, and for [`HOLD_MS`] after the
//!   last one; this player speaks while its microphone sends (push to talk held, or the open
//!   microphone's gate open). [`mark`] gives the speaking players' avatars to the compass
//!   (`coop_engine::voicemark`), [`Announce`] says on the HUD who began to speak.
//! - [`Prefs`]: mutes and volumes by name (any case), from the environment's `NV_COOP_VOICE_MUTE`
//!   and `NV_COOP_VOICE_VOLUMES` and from `Player=` lines of the launcher's `coop.ini`, written
//!   back there when changed in the game.
//! - [`PlayerList`]: the players list the players key opens (`NV_COOP_PLAYERS_KEY`, F11), as the
//!   game's own message box (`coop_engine::msgbox`): each player with ping, speaking, mute and
//!   volume; a player's page to mute, turn up or down, and (the host) kick.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::PlayerId;
use engine_core::Engine;

use crate::ladder::Ladder;
use crate::voice::{dik_of, key_name, Settings};

/// How long a player still counts as speaking after their last frame (ms).
pub const HOLD_MS: u64 = 300;
/// One press of "volume up" or "down".
pub const VOLUME_STEP: f32 = 0.1;
/// The loudest a player can be set (the receiver's own limit, `coop_voice::pipeline`).
pub const MAX_VOLUME: f32 = 2.0;
/// A player's start of speech is said on the HUD at most this often (ms).
pub const ANNOUNCE_EVERY_MS: u64 = 10_000;
/// The players the list shows as buttons at most (the rest are named in its text).
pub const MAX_BUTTONS: usize = 8;
/// The default players key: F11 (DirectInput code, `voice`'s key table). Not F9: the game binds F9
/// (quick load) and F5 (quick save) by default (the keyboard defaults `fn_00a24b70` writes,
/// `run_00a237f0.rs`); F11 is in none of them.
pub const DEFAULT_PLAYERS_KEY: u8 = 0x57;

/// Who is speaking now.
#[derive(Clone, Debug, Default)]
pub struct Speaking {
    last: BTreeMap<PlayerId, u64>,
    local_ms: Option<u64>,
    /// This player's microphone level of its last frame (0 to 1), for the list's bar.
    pub local_level: f32,
}

impl Speaking {
    /// A frame of player `p` arrived at `now_ms`.
    pub fn heard(&mut self, p: PlayerId, now_ms: u64) {
        self.last.insert(p, now_ms);
    }

    /// This player's microphone sent a frame (`talking`) or not, at `now_ms`.
    pub fn local(&mut self, talking: bool, now_ms: u64) {
        if talking {
            self.local_ms = Some(now_ms);
        }
    }

    /// The other players speaking at `now_ms`, in slot order.
    pub fn now(&self, now_ms: u64) -> Vec<PlayerId> {
        self.last
            .iter()
            .filter(|(_, t)| now_ms.saturating_sub(**t) <= HOLD_MS)
            .map(|(p, _)| *p)
            .collect()
    }

    /// Whether this player speaks at `now_ms`.
    pub fn local_now(&self, now_ms: u64) -> bool {
        self.local_ms
            .is_some_and(|t| now_ms.saturating_sub(t) <= HOLD_MS)
    }

    /// Player `p` left.
    pub fn forget(&mut self, p: PlayerId) {
        self.last.remove(&p);
    }
}

/// Gives the compass the avatars of the players in `speaking` (those `ladder` has in this game).
pub fn mark(e: &mut Engine, ladder: Option<&Ladder>, speaking: &[PlayerId]) {
    let ids: Vec<u32> = ladder
        .map(|l| {
            l.avatar_ids()
                .into_iter()
                .filter(|(p, _)| speaking.contains(p))
                .map(|(_, id)| id)
                .collect()
        })
        .unwrap_or_default();
    coop_engine::voicemark::set_speaking(e, ids);
}

/// Says on the HUD (the game's own messages) when a player begins to speak, each at most every
/// [`ANNOUNCE_EVERY_MS`]. The game's messages wait their turn and stay their time, so this is no
/// live list: the live marks are the compass ticks and the players list.
#[derive(Clone, Debug, Default)]
pub struct Announce {
    was: BTreeSet<Option<PlayerId>>,
    said_ms: BTreeMap<Option<PlayerId>, u64>,
}

impl Announce {
    /// The speakers that began now (`None` is this player), to be said.
    pub fn step(
        &mut self,
        speaking: &[PlayerId],
        local: bool,
        now_ms: u64,
    ) -> Vec<Option<PlayerId>> {
        let mut now: BTreeSet<Option<PlayerId>> = speaking.iter().map(|p| Some(*p)).collect();
        if local {
            now.insert(None);
        }
        let mut out = Vec::new();
        for who in &now {
            if self.was.contains(who) {
                continue;
            }
            let due = self
                .said_ms
                .get(who)
                .map_or(true, |t| now_ms.saturating_sub(*t) >= ANNOUNCE_EVERY_MS);
            if due {
                self.said_ms.insert(*who, now_ms);
                out.push(*who);
            }
        }
        self.was = now;
        out
    }
}

/// The text the HUD shows for `who` beginning to speak.
pub fn announce_text(who: Option<PlayerId>, names: &BTreeMap<PlayerId, String>) -> String {
    match who {
        None => "Voice chat: you are speaking".into(),
        Some(p) => format!("Voice chat: {} is speaking", name_or_slot(names, p)),
    }
}

fn name_or_slot(names: &BTreeMap<PlayerId, String>, p: PlayerId) -> String {
    names
        .get(&p)
        .cloned()
        .unwrap_or_else(|| format!("player {}", p.0))
}

/// A voice's loudness (RMS of a frame, 0 to 1) as a level for a bar (0 to 1): speech's RMS is
/// mostly under a quarter of full scale, so it is shown four times larger (a display choice).
pub fn display_level(rms: f32) -> f32 {
    if rms.is_finite() {
        (rms * 4.0).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// The live list's lines: each player speaking now (this player first, as "You"), with a level
/// bar when the level is known.
pub fn live_lines(
    names: &BTreeMap<PlayerId, String>,
    speaking: &[PlayerId],
    local: Option<f32>,
    levels: &BTreeMap<PlayerId, f32>,
) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(l) = local {
        out.push(format!("You {}", level_bar(l)));
    }
    for p in speaking {
        let name = name_or_slot(names, *p);
        out.push(match levels.get(p) {
            Some(l) => format!("{name} {}", level_bar(*l)),
            None => name,
        });
    }
    out
}

/// The head labels: each speaking player's avatar in this game and the text over it.
pub fn head_marks(
    names: &BTreeMap<PlayerId, String>,
    speaking: &[PlayerId],
    avatars: &[(PlayerId, u32)],
) -> Vec<(u32, String)> {
    speaking
        .iter()
        .filter_map(|p| {
            let (_, id) = avatars.iter().find(|(q, _)| q == p)?;
            Some((
                *id,
                format!("{} (speaking)", box_text(&name_or_slot(names, *p))),
            ))
        })
        .collect()
}

/// The list's own settings, from the environment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListSettings {
    /// `NV_COOP_PLAYERS_KEY`: opens and closes the players list (default F11).
    pub players_key: u8,
    /// `NV_COOP_SPEAKING`: the live list and the head labels on the HUD (`live`, the default; off
    /// with `off`).
    pub live: bool,
    /// `NV_COOP_SPEAKING=messages`: also the game's own "is speaking" messages (off by default).
    pub announce: bool,
}

impl Default for ListSettings {
    fn default() -> ListSettings {
        ListSettings {
            players_key: DEFAULT_PLAYERS_KEY,
            live: true,
            announce: false,
        }
    }
}

impl ListSettings {
    /// The settings `var` gives, with the lines that say what was not understood.
    pub fn read(var: impl Fn(&str) -> Option<String>) -> (ListSettings, Vec<String>) {
        let mut s = ListSettings::default();
        let mut notes = Vec::new();
        if let Some(v) = var("NV_COOP_PLAYERS_KEY").filter(|v| !v.trim().is_empty()) {
            match dik_of(&v) {
                Some(c) => s.players_key = c,
                None => notes.push(format!(
                    "co-op players list: NV_COOP_PLAYERS_KEY={v} is not a key name; {} it is",
                    key_name(s.players_key)
                )),
            }
        }
        if let Some(v) = var("NV_COOP_SPEAKING").filter(|v| !v.trim().is_empty()) {
            match v.trim().to_ascii_lowercase().as_str() {
                "off" | "0" => (s.live, s.announce) = (false, false),
                "live" | "on" => (s.live, s.announce) = (true, false),
                "messages" | "all" => (s.live, s.announce) = (true, true),
                _ => notes.push(format!(
                    "co-op players list: NV_COOP_SPEAKING={v}: use live, messages or off; live it is"
                )),
            }
        }
        (s, notes)
    }

    /// The line for the log.
    pub fn line(&self) -> String {
        format!(
            "co-op players list: {} opens it (NV_COOP_PLAYERS_KEY); who speaks is {} on the HUD (NV_COOP_SPEAKING)",
            key_name(self.players_key),
            match (self.live, self.announce) {
                (false, _) => "not shown",
                (true, false) => "listed and marked over their heads",
                (true, true) => "listed, marked over their heads and said in messages",
            }
        )
    }
}

/// One player's mute and volume, by name.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerPref {
    /// The name as shown (its key is the same in lower case).
    pub name: String,
    /// Never heard.
    pub muted: bool,
    /// 1 is as sent, 0 to [`MAX_VOLUME`].
    pub volume: f32,
}

/// Mutes and volumes by name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Prefs {
    list: Vec<PlayerPref>,
    /// Changed since the file was last written.
    pub dirty: bool,
}

fn key(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

/// A volume as whole percent, rounded to the step.
fn percent(v: f32) -> u32 {
    ((v.clamp(0.0, MAX_VOLUME) * 100.0) / (VOLUME_STEP * 100.0)).round() as u32
        * (VOLUME_STEP * 100.0).round() as u32
}

impl Prefs {
    /// The mutes and volumes the environment set (`voice::Settings`).
    pub fn from_settings(s: &Settings) -> Prefs {
        let mut p = Prefs::default();
        for n in &s.muted {
            p.entry(n).muted = true;
        }
        for (n, v) in &s.volumes {
            p.entry(n).volume = *v;
        }
        p.dirty = false;
        p
    }

    fn entry(&mut self, name: &str) -> &mut PlayerPref {
        let k = key(name);
        let at = match self.list.iter().position(|e| key(&e.name) == k) {
            Some(i) => i,
            None => {
                self.list.push(PlayerPref {
                    name: name.trim().to_string(),
                    muted: false,
                    volume: 1.0,
                });
                self.list.len() - 1
            }
        };
        // The latest spelling is the one shown (the environment's names are in lower case).
        self.list[at].name = name.trim().to_string();
        &mut self.list[at]
    }

    /// Every name with a setting, in the order first seen.
    pub fn list(&self) -> &[PlayerPref] {
        &self.list
    }

    /// Whether `name` is muted.
    pub fn is_muted(&self, name: &str) -> bool {
        let k = key(name);
        self.list.iter().any(|e| key(&e.name) == k && e.muted)
    }

    /// The volume of `name` (1 when it has none).
    pub fn volume_of(&self, name: &str) -> f32 {
        let k = key(name);
        self.list
            .iter()
            .find(|e| key(&e.name) == k)
            .map_or(1.0, |e| e.volume)
    }

    /// Mutes or unmutes `name`.
    pub fn set_muted(&mut self, name: &str, muted: bool) {
        let e = self.entry(name);
        if e.muted != muted {
            e.muted = muted;
            self.dirty = true;
        }
    }

    /// Sets the volume of `name` (kept within 0 and [`MAX_VOLUME`], to the step).
    pub fn set_volume(&mut self, name: &str, volume: f32) {
        let v = percent(volume) as f32 / 100.0;
        let e = self.entry(name);
        if (e.volume - v).abs() > f32::EPSILON {
            e.volume = v;
            self.dirty = true;
        }
    }

    /// The `Player=` lines of `coop.ini`: `Player=name;muted (0 or 1);volume in percent`. Each
    /// overrides what the environment said of that name; a line not understood is skipped.
    pub fn read_ini(&mut self, text: &str) {
        for line in text.lines() {
            let Some((k, v)) = line.split_once('=') else {
                continue;
            };
            if !k.trim().eq_ignore_ascii_case("player") {
                continue;
            }
            let mut parts = v.split(';');
            let (Some(name), Some(m), Some(vol)) = (parts.next(), parts.next(), parts.next())
            else {
                continue;
            };
            let name = name.trim();
            let Ok(vol) = vol.trim().trim_end_matches('%').parse::<f32>() else {
                continue;
            };
            if name.is_empty() || !vol.is_finite() {
                continue;
            }
            let e = self.entry(name);
            e.muted = matches!(m.trim(), "1" | "on" | "yes" | "true");
            e.volume = (vol / 100.0).clamp(0.0, MAX_VOLUME);
        }
        self.dirty = false;
    }

    /// `text` (the file as it is) with its `Player=` lines replaced by these settings; every
    /// other line is kept as it was. A name at its default (heard, 100%) gets no line.
    pub fn merged_ini(&self, text: &str) -> String {
        let mut out = String::new();
        for line in text.lines() {
            let is_player = line
                .split_once('=')
                .is_some_and(|(k, _)| k.trim().eq_ignore_ascii_case("player"));
            if !is_player {
                out.push_str(line);
                out.push_str("\r\n");
            }
        }
        for e in &self.list {
            if !e.muted && percent(e.volume) == 100 {
                continue;
            }
            let name: String = e
                .name
                .chars()
                .map(|c| {
                    if matches!(c, ';' | '\r' | '\n') {
                        '_'
                    } else {
                        c
                    }
                })
                .collect();
            out.push_str(&format!(
                "Player={};{};{}\r\n",
                name,
                u8::from(e.muted),
                percent(e.volume)
            ));
        }
        out
    }
}

/// Where the launcher's `coop.ini` is: `NV_COOP_INI` (the launcher sets it), else `coop.ini` in
/// the folder the game was started from when it is there (the launcher starts the game in the
/// package's `userdata`).
pub fn ini_path(var: impl Fn(&str) -> Option<String>) -> Option<std::path::PathBuf> {
    if let Some(v) = var("NV_COOP_INI").filter(|v| !v.trim().is_empty()) {
        return Some(v.trim().into());
    }
    let here = std::path::PathBuf::from("coop.ini");
    here.is_file().then_some(here)
}

/// One row of the players list.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// The player's slot.
    pub player: PlayerId,
    /// Their name.
    pub name: String,
    /// The round trip to them, when this game's network measures it.
    pub ping_ms: Option<u32>,
    /// Speaking now.
    pub speaking: bool,
    /// Muted here.
    pub muted: bool,
    /// Their volume here.
    pub volume: f32,
    /// This game's own player.
    pub me: bool,
    /// The voice's level while speaking, 0 to 1 (this player's microphone, or the other player's
    /// voice as heard here).
    pub level: Option<f32>,
}

/// What the list shows: one page of the game's message box.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    /// The box's text.
    pub text: String,
    /// Its buttons, in order.
    pub buttons: Vec<String>,
}

/// What a click asks for.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// Nothing to do but show the page again.
    Show,
    /// The list was closed.
    Close,
    /// Mute (`true`) or unmute the player of this name.
    Mute(String, bool),
    /// Set the volume of the player of this name.
    Volume(String, f32),
    /// The host removes this player from the session.
    Kick(PlayerId, String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Closed,
    All,
    One(PlayerId),
    Kick(PlayerId),
}

/// The players list's state.
#[derive(Clone, Debug)]
pub struct PlayerList {
    screen: Screen,
}

impl Default for PlayerList {
    fn default() -> PlayerList {
        PlayerList {
            screen: Screen::Closed,
        }
    }
}

/// A text the message box shows as it is: no `;` (it ends a button), `&` (it marks a hotkey) or
/// `%`; characters past Latin-1 become `?`.
pub fn box_text(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ';' | '&' | '%' => '_',
            c if (c as u32) < 0x100 => c,
            _ => '?',
        })
        .collect()
}

fn level_bar(level: f32) -> String {
    let n = (level.clamp(0.0, 1.0) * 10.0).round() as usize;
    format!("[{}{}]", "|".repeat(n), ".".repeat(10 - n))
}

fn state_words(r: &Row) -> String {
    let mut w = Vec::new();
    if r.speaking {
        w.push(match r.level {
            Some(l) => format!("speaking {}", level_bar(l)),
            None => "speaking".to_string(),
        });
    }
    if !r.me {
        if r.muted {
            w.push("muted".into());
        }
        w.push(format!("volume {}", percent(r.volume)));
    }
    if let Some(p) = r.ping_ms {
        w.push(format!("ping {p} ms"));
    }
    w.join(", ")
}

impl PlayerList {
    /// Whether the list is shown.
    pub fn is_open(&self) -> bool {
        self.screen != Screen::Closed
    }

    /// The players key: opens the list, or closes it.
    pub fn toggle(&mut self) {
        self.screen = match self.screen {
            Screen::Closed => Screen::All,
            _ => Screen::Closed,
        };
    }

    /// Closes the list (the box was taken away by the game).
    pub fn close(&mut self) {
        self.screen = Screen::Closed;
    }

    fn others(rows: &[Row]) -> Vec<&Row> {
        rows.iter().filter(|r| !r.me).take(MAX_BUTTONS).collect()
    }

    fn row(rows: &[Row], p: PlayerId) -> Option<&Row> {
        rows.iter().find(|r| r.player == p && !r.me)
    }

    /// The page to show for `rows` (`host`: this game hosts), `None` while closed. A player's
    /// page whose player left is the whole list again.
    pub fn page(&mut self, rows: &[Row], host: bool) -> Option<Page> {
        if let Screen::One(p) | Screen::Kick(p) = self.screen {
            if Self::row(rows, p).is_none() {
                self.screen = Screen::All;
            }
        }
        let page = match self.screen {
            Screen::Closed => return None,
            Screen::All => {
                let mut text = String::from("Co-op players");
                for r in rows {
                    let who = if r.me {
                        format!("{} (you)", r.name)
                    } else {
                        r.name.clone()
                    };
                    let words = state_words(r);
                    text.push('\n');
                    text.push_str(&who);
                    if !words.is_empty() {
                        text.push_str(": ");
                        text.push_str(&words);
                    }
                }
                let mut buttons: Vec<String> = Self::others(rows)
                    .into_iter()
                    .map(|r| {
                        let mut b = r.name.clone();
                        if r.speaking {
                            b.push_str(" (speaking)");
                        }
                        if r.muted {
                            b.push_str(" (muted)");
                        }
                        b
                    })
                    .collect();
                buttons.push("Close".into());
                Page { text, buttons }
            }
            Screen::One(p) => {
                let r = Self::row(rows, p)?;
                let mut buttons = vec![
                    if r.muted { "Unmute" } else { "Mute" }.to_string(),
                    "Volume up".to_string(),
                    "Volume down".to_string(),
                ];
                if host {
                    buttons.push("Kick".into());
                }
                buttons.push("Back".into());
                Page {
                    text: format!("{}\n{}", r.name, state_words(r)),
                    buttons,
                }
            }
            Screen::Kick(p) => {
                let r = Self::row(rows, p)?;
                Page {
                    text: format!("Remove {} from the session?", r.name),
                    buttons: vec!["Kick".into(), "Back".into()],
                }
            }
        };
        Some(Page {
            text: box_text(&page.text),
            buttons: page.buttons.iter().map(|b| box_text(b)).collect(),
        })
    }

    /// Button `index` of the page last shown was clicked.
    pub fn click(&mut self, index: usize, rows: &[Row], host: bool) -> Action {
        match self.screen {
            Screen::Closed => Action::Close,
            Screen::All => {
                let others = Self::others(rows);
                match others.get(index) {
                    Some(r) => {
                        self.screen = Screen::One(r.player);
                        Action::Show
                    }
                    None => {
                        self.screen = Screen::Closed;
                        Action::Close
                    }
                }
            }
            Screen::One(p) => {
                let Some(r) = Self::row(rows, p) else {
                    self.screen = Screen::All;
                    return Action::Show;
                };
                match (index, host) {
                    (0, _) => Action::Mute(r.name.clone(), !r.muted),
                    (1, _) => {
                        Action::Volume(r.name.clone(), (r.volume + VOLUME_STEP).min(MAX_VOLUME))
                    }
                    (2, _) => Action::Volume(r.name.clone(), (r.volume - VOLUME_STEP).max(0.0)),
                    (3, true) => {
                        self.screen = Screen::Kick(p);
                        Action::Show
                    }
                    _ => {
                        self.screen = Screen::All;
                        Action::Show
                    }
                }
            }
            Screen::Kick(p) => {
                let name = Self::row(rows, p).map(|r| r.name.clone());
                self.screen = Screen::All;
                match (index, name) {
                    (0, Some(n)) if host => Action::Kick(p, n),
                    _ => Action::Show,
                }
            }
        }
    }
}

/// Applies an action to the prefs; true when it changed them.
pub fn apply(prefs: &mut Prefs, action: &Action) -> bool {
    match action {
        Action::Mute(n, m) => {
            prefs.set_muted(n, *m);
            true
        }
        Action::Volume(n, v) => {
            prefs.set_volume(n, *v);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows() -> Vec<Row> {
        vec![
            Row {
                player: PlayerId::HOST,
                name: "Ann".into(),
                ping_ms: None,
                speaking: true,
                muted: false,
                volume: 1.0,
                me: true,
                level: Some(0.5),
            },
            Row {
                player: PlayerId(1),
                name: "Bo".into(),
                ping_ms: Some(42),
                speaking: true,
                muted: false,
                volume: 1.0,
                me: false,
                level: None,
            },
            Row {
                player: PlayerId(2),
                name: "Cy;&%".into(),
                ping_ms: None,
                speaking: false,
                muted: true,
                volume: 0.5,
                me: false,
                level: None,
            },
        ]
    }

    #[test]
    fn a_player_speaks_until_300_ms_after_the_last_frame() {
        let mut s = Speaking::default();
        s.heard(PlayerId(2), 1000);
        s.heard(PlayerId(1), 1100);
        assert_eq!(s.now(1200), vec![PlayerId(1), PlayerId(2)]);
        assert_eq!(s.now(1300), vec![PlayerId(1), PlayerId(2)]);
        assert_eq!(s.now(1301), vec![PlayerId(1)]);
        assert!(s.now(1401).is_empty());
        s.forget(PlayerId(1));
        s.heard(PlayerId(2), 2000);
        assert_eq!(s.now(2000), vec![PlayerId(2)]);
        // This player: only while its microphone sends.
        assert!(!s.local_now(0));
        s.local(false, 3000);
        assert!(!s.local_now(3000));
        s.local(true, 3000);
        assert!(s.local_now(3300) && !s.local_now(3301));
    }

    #[test]
    fn a_start_of_speech_is_said_once_and_not_again_too_soon() {
        let mut a = Announce::default();
        let p = PlayerId(1);
        assert_eq!(a.step(&[p], false, 0), vec![Some(p)]);
        assert!(a.step(&[p], false, 100).is_empty(), "still speaking");
        assert!(a.step(&[], false, 500).is_empty());
        assert!(a.step(&[p], false, 2000).is_empty(), "again within 10 s");
        assert!(a.step(&[], false, 3000).is_empty());
        assert_eq!(a.step(&[p], true, 12_000), vec![None, Some(p)]);
        let mut names = BTreeMap::new();
        names.insert(p, "Bo".to_string());
        assert_eq!(announce_text(Some(p), &names), "Voice chat: Bo is speaking");
        assert_eq!(
            announce_text(Some(PlayerId(3)), &names),
            "Voice chat: player 3 is speaking"
        );
        assert!(announce_text(None, &names).contains("you"));
    }

    #[test]
    fn list_settings_read_the_key_and_what_the_hud_shows() {
        let (s, n) = ListSettings::read(|_| None);
        assert_eq!(s, ListSettings::default());
        assert_eq!(s.players_key, 0x57, "F11: the game binds F9");
        assert!(s.live && !s.announce);
        assert!(n.is_empty());
        let (s, n) = ListSettings::read(|k| match k {
            "NV_COOP_PLAYERS_KEY" => Some("F10".into()),
            "NV_COOP_SPEAKING" => Some("off".into()),
            _ => None,
        });
        assert_eq!((s.players_key, s.live, s.announce), (0x44, false, false));
        assert!(n.is_empty());
        let (s, _) = ListSettings::read(|k| (k == "NV_COOP_SPEAKING").then(|| "messages".into()));
        assert!(s.live && s.announce);
        let (s, n) = ListSettings::read(|k| match k {
            "NV_COOP_PLAYERS_KEY" => Some("huge".into()),
            "NV_COOP_SPEAKING" => Some("loud".into()),
            _ => None,
        });
        assert_eq!(s.players_key, 0x57);
        assert!(s.live);
        assert_eq!(n.len(), 2);
        assert!(s.line().contains("F11"));
    }

    #[test]
    fn the_live_list_and_the_head_labels_name_who_speaks() {
        let mut names = BTreeMap::new();
        names.insert(PlayerId(1), "Bo".to_string());
        names.insert(PlayerId(2), "Cy;".to_string());
        let mut levels = BTreeMap::new();
        levels.insert(PlayerId(1), display_level(0.125));
        let lines = live_lines(&names, &[PlayerId(1), PlayerId(2)], Some(1.0), &levels);
        assert_eq!(lines, vec!["You [||||||||||]", "Bo [|||||.....]", "Cy;"]);
        assert!(live_lines(&names, &[], None, &levels).is_empty());
        let marks = head_marks(
            &names,
            &[PlayerId(1), PlayerId(2), PlayerId(3)],
            &[(PlayerId(1), 0xff00_0001), (PlayerId(3), 0xff00_0003)],
        );
        assert_eq!(
            marks,
            vec![
                (0xff00_0001, "Bo (speaking)".to_string()),
                (0xff00_0003, "player 3 (speaking)".to_string())
            ],
            "a player with no avatar here has no label"
        );
        assert_eq!(display_level(0.5), 1.0);
        assert_eq!(display_level(f32::NAN), 0.0);
    }
    #[test]
    fn prefs_come_from_the_environment_and_the_file_and_go_back_to_the_file() {
        let (settings, _) = Settings::read(|k| match k {
            "NV_COOP_VOICE_MUTE" => Some("Bo".into()),
            "NV_COOP_VOICE_VOLUMES" => Some("cy:150".into()),
            _ => None,
        });
        let mut p = Prefs::from_settings(&settings);
        assert!(p.is_muted("BO") && !p.is_muted("cy"));
        assert!((p.volume_of("Cy") - 1.5).abs() < 1e-6);
        assert!((p.volume_of("nobody") - 1.0).abs() < 1e-6);
        assert!(!p.dirty);
        let file = "; comment\r\nName=Ann\r\nPlayer=Bo;0;70\r\nPlayer=Dee Two;1;100\r\nPlayer=bad\r\nVoice=ptt\r\n";
        p.read_ini(file);
        assert!(
            !p.is_muted("bo"),
            "the file's line overrides the environment"
        );
        assert!((p.volume_of("bo") - 0.7).abs() < 1e-6);
        assert!(p.is_muted("dee two"));
        p.set_muted("Eve", true);
        p.set_volume("Bo", 0.86);
        assert!(p.dirty);
        assert!((p.volume_of("bo") - 0.9).abs() < 1e-6, "to the step");
        p.set_volume("cy", 1.0);
        let out = p.merged_ini(file);
        assert!(out.contains("Name=Ann\r\n") && out.contains("Voice=ptt\r\n"));
        assert!(out.contains("Player=Bo;0;90\r\n"), "{out}");
        assert!(out.contains("Player=Dee Two;1;100\r\n"));
        assert!(out.contains("Player=Eve;1;100\r\n"));
        assert!(!out.contains("cy;"), "a default gets no line: {out}");
        assert!(!out.contains("Player=bad"));
        // Read back, the same settings.
        let mut q = Prefs::default();
        q.read_ini(&out);
        assert!(q.is_muted("eve") && q.is_muted("dee two") && !q.is_muted("bo"));
        assert!((q.volume_of("bo") - 0.9).abs() < 1e-6);
        // The ends of the scale.
        p.set_volume("bo", 9.0);
        assert!((p.volume_of("bo") - MAX_VOLUME).abs() < 1e-6);
        p.set_volume("bo", -1.0);
        assert_eq!(p.volume_of("bo"), 0.0);
    }

    #[test]
    fn the_ini_is_found_where_the_launcher_says() {
        assert_eq!(
            ini_path(|k| (k == "NV_COOP_INI").then(|| " x\\coop.ini ".into())),
            Some(std::path::PathBuf::from("x\\coop.ini"))
        );
    }

    #[test]
    fn the_list_shows_each_player_and_its_page_mutes_turns_and_kicks() {
        let rows = rows();
        let mut l = PlayerList::default();
        assert!(l.page(&rows, true).is_none());
        l.toggle();
        let page = l.page(&rows, true).unwrap();
        assert!(page
            .text
            .starts_with("Co-op players\nAnn (you): speaking [|||||.....]"));
        assert!(page.text.contains("Bo: speaking, volume 100, ping 42 ms"));
        assert!(
            page.text.contains("Cy___: muted, volume 50"),
            "{}",
            page.text
        );
        assert_eq!(
            page.buttons,
            vec!["Bo (speaking)", "Cy___ (muted)", "Close"]
        );
        // Bo's page.
        assert_eq!(l.click(0, &rows, true), Action::Show);
        let page = l.page(&rows, true).unwrap();
        assert_eq!(
            page.buttons,
            vec!["Mute", "Volume up", "Volume down", "Kick", "Back"]
        );
        assert_eq!(l.click(0, &rows, true), Action::Mute("Bo".into(), true));
        match l.click(1, &rows, true) {
            Action::Volume(n, v) => assert!(n == "Bo" && (v - 1.1).abs() < 1e-6),
            other => panic!("{other:?}"),
        }
        match l.click(2, &rows, true) {
            Action::Volume(n, v) => assert!(n == "Bo" && (v - 0.9).abs() < 1e-6),
            other => panic!("{other:?}"),
        }
        // A guest has no kick: its fourth button is Back.
        let guest_page = l.clone().page(&rows, false).unwrap();
        assert_eq!(guest_page.buttons.len(), 4);
        assert_eq!(l.clone().click(3, &rows, false), Action::Show);
        // Kick asks first.
        assert_eq!(l.click(3, &rows, true), Action::Show);
        assert!(l.page(&rows, true).unwrap().text.contains("Remove Bo"));
        assert_eq!(
            l.click(0, &rows, true),
            Action::Kick(PlayerId(1), "Bo".into())
        );
        // Back on the whole list; Close closes.
        assert_eq!(l.page(&rows, true).unwrap().buttons.len(), 3);
        assert_eq!(l.click(2, &rows, true), Action::Close);
        assert!(!l.is_open());
    }

    #[test]
    fn a_page_of_a_player_who_left_is_the_whole_list() {
        let mut rows = rows();
        let mut l = PlayerList::default();
        l.toggle();
        l.page(&rows, true);
        l.click(1, &rows, true);
        rows.retain(|r| r.player != PlayerId(2));
        let page = l.page(&rows, true).unwrap();
        assert!(page.text.starts_with("Co-op players"));
        // A guest cannot kick even from the kick page.
        let mut g = PlayerList::default();
        g.toggle();
        g.click(0, &rows, false);
        g.screen = Screen::Kick(PlayerId(1));
        assert_eq!(g.click(0, &rows, false), Action::Show);
    }

    #[test]
    fn mute_and_volume_actions_change_the_prefs() {
        let mut p = Prefs::default();
        assert!(apply(&mut p, &Action::Mute("Bo".into(), true)));
        assert!(apply(&mut p, &Action::Volume("Bo".into(), 1.2)));
        assert!(!apply(&mut p, &Action::Close));
        assert!(p.is_muted("bo"));
        assert!((p.volume_of("bo") - 1.2).abs() < 1e-6);
    }

    #[test]
    fn box_texts_lose_what_the_box_reads_as_markup() {
        assert_eq!(box_text("a;b&c%d"), "a_b_c_d");
        assert_eq!(box_text("Zoë"), "Zoë");
        assert_eq!(box_text("名"), "?");
    }
}
