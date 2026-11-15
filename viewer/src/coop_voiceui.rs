//! voiceui: the players list, who is speaking, and each player's mute and volume by name
//! (docs/coop/SPECS.md, voice list).
//!
//! - The players key (`NV_COOP_PLAYERS_KEY`, F11) opens the list as the game's own message box
//!   (`coop_engine::msgbox`): every player with ping (where this game's network measures it),
//!   speaking, muted and volume; a click on a player opens their page: mute, volume up and down,
//!   kick (the host). The pages are `coop_engine_session::voicelist::PlayerList`'s.
//! - Mutes and volumes are by name ([`Prefs`]): from `NV_COOP_VOICE_MUTE`, `NV_COOP_VOICE_VOLUMES`
//!   and the `Player=` lines of the launcher's `coop.ini` (`NV_COOP_INI`), written back there on
//!   every change in the game.
//! - Who speaks is shown on the HUD every frame (`coop_engine::hudtext`): a live list (name and
//!   level bar) and a label over each speaking player's head; `NV_COOP_SPEAKING=messages` also says
//!   who begins in the game's own messages, `off` shows none of it. The compass marks of speaking
//!   players are the session's (`voicelist::mark`).

use std::collections::BTreeMap;
use std::path::PathBuf;

use coop_core::ids::PlayerId;
use coop_engine_session::voice::{Press, Settings, VoiceLink};
use coop_engine_session::voicelist::{
    self, announce_text, Action, Announce, ListSettings, PlayerList, Prefs, Row,
};
use engine_core::Engine;

/// The players list, the HUD's "is speaking" and the saved mutes and volumes.
pub struct VoiceUi {
    /// Mutes and volumes by name (the audio side reads them).
    pub prefs: Prefs,
    settings: ListSettings,
    ini: Option<PathBuf>,
    list: PlayerList,
    key: Press,
    announce: Announce,
    /// Buttons of the page shown, while one is.
    shown: Option<usize>,
    /// The game showed the box (its menu existed) since it was asked for.
    seen_open: bool,
    unsaved_said: bool,
}

impl VoiceUi {
    /// From the environment and the launcher's file; the lines say what was read.
    pub fn open() -> (VoiceUi, Vec<String>) {
        let var = |n: &str| std::env::var(n).ok();
        let (voice, _) = Settings::read(var);
        let (settings, mut lines) = ListSettings::read(var);
        lines.insert(0, settings.line());
        let mut prefs = Prefs::from_settings(&voice);
        let ini = voicelist::ini_path(var);
        match &ini {
            Some(p) => match std::fs::read(p) {
                Ok(bytes) => {
                    prefs.read_ini(&String::from_utf8_lossy(&bytes));
                    lines.push(format!(
                        "co-op players list: {} players' mutes and volumes read from coop.ini",
                        prefs.list().len()
                    ));
                }
                Err(_) => lines.push(
                    "co-op players list: no coop.ini yet; changes made in the game start it".into(),
                ),
            },
            None => lines.push(
                "co-op players list: no coop.ini (NV_COOP_INI); changes made in the game last until it ends".into(),
            ),
        }
        (
            VoiceUi {
                prefs,
                settings,
                ini,
                list: PlayerList::default(),
                key: Press::default(),
                announce: Announce::default(),
                shown: None,
                seen_open: false,
                unsaved_said: false,
            },
            lines,
        )
    }

    fn save(&mut self, lines: &mut Vec<String>) {
        if !self.prefs.dirty {
            return;
        }
        let Some(path) = self.ini.as_ref() else {
            if !self.unsaved_said {
                self.unsaved_said = true;
                lines.push("co-op players list: not saved (no coop.ini)".into());
            }
            return;
        };
        let old = std::fs::read(path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        match std::fs::write(path, self.prefs.merged_ini(&old)) {
            Ok(()) => self.prefs.dirty = false,
            Err(e) => {
                lines.push(format!("co-op players list: coop.ini not written: {e}"));
                self.prefs.dirty = false;
            }
        }
    }

    /// One step after the session's and the audio's. `names`: every player's name, `me` this
    /// player, `host` whether this game hosts, `link` the voice link (who speaks), `ping` the
    /// round trip to a player. Returns the player the host chose to kick, if any.
    #[allow(clippy::too_many_arguments)]
    pub fn frame(
        &mut self,
        e: &mut Engine,
        names: &BTreeMap<PlayerId, String>,
        me: PlayerId,
        host: bool,
        link: Option<&VoiceLink>,
        ping: &dyn Fn(PlayerId) -> Option<u32>,
        lines: &mut Vec<String>,
    ) -> Option<PlayerId> {
        let now = link.map_or(0, |l| l.now_ms);
        let speaking = link.map(|l| l.speaking.now(now)).unwrap_or_default();
        let local = link.is_some_and(|l| l.speaking.local_now(now));
        // Who speaks, on the HUD: the live list and the head labels, every frame.
        if self.settings.live {
            let levels = link.map(|l| l.levels.clone()).unwrap_or_default();
            let local_level = link.map_or(0.0, |l| l.speaking.local_level);
            let lines =
                voicelist::live_lines(names, &speaking, local.then_some(local_level), &levels);
            coop_engine::hudtext::show_list(e, &lines);
            let avatars = link.map(|l| l.avatars.clone()).unwrap_or_default();
            coop_engine::hudtext::show_heads(e, &voicelist::head_marks(names, &speaking, &avatars));
        }
        // Who began to speak, in the game's own messages (when asked for).
        if self.settings.announce {
            for who in self.announce.step(&speaking, local, now) {
                coop_engine::talk::show(e, &announce_text(who, names));
            }
        }
        // The players key (only while the game's window has the focus).
        #[cfg(windows)]
        let keys = engine_platform::d3d9real::held_keys();
        #[cfg(not(windows))]
        let keys = [false; 256];
        if self
            .key
            .pressed(keys[usize::from(self.settings.players_key)])
        {
            // A page asked for and not on the screen is asked for again; one on the screen is
            // closed with its own Close (or Escape); without one the key opens the list.
            if self.shown.is_some() {
                if !coop_engine::msgbox::open(e) {
                    self.shown = None;
                }
            } else {
                self.list.toggle();
            }
        }
        let rows: Vec<Row> = names
            .iter()
            .map(|(p, n)| Row {
                player: *p,
                name: n.clone(),
                ping_ms: if *p == me { None } else { ping(*p) },
                speaking: if *p == me {
                    local
                } else {
                    speaking.contains(p)
                },
                muted: *p != me && self.prefs.is_muted(n),
                volume: self.prefs.volume_of(n),
                me: *p == me,
                level: if *p == me {
                    local.then(|| link.map_or(0.0, |l| l.speaking.local_level))
                } else {
                    link.and_then(|l| l.levels.get(p).copied())
                        .filter(|_| speaking.contains(p))
                },
            })
            .collect();
        let mut kick = None;
        if let Some(count) = self.shown {
            if coop_engine::msgbox::open(e) {
                self.seen_open = true;
            }
            if let Some(index) = coop_engine::msgbox::answer(e, count) {
                self.shown = None;
                let action = self.list.click(index, &rows, host);
                if voicelist::apply(&mut self.prefs, &action) {
                    self.save(lines);
                }
                match action {
                    Action::Kick(p, n) => {
                        lines.push(format!("co-op players list: the host removes {n}"));
                        kick = Some(p);
                    }
                    Action::Mute(n, m) => lines.push(format!(
                        "co-op players list: {n} {}",
                        if m { "muted" } else { "heard again" }
                    )),
                    Action::Volume(n, v) => {
                        lines.push(format!("co-op players list: {n} at {:.0}%", v * 100.0))
                    }
                    Action::Show | Action::Close => {}
                }
            } else if self.seen_open && !coop_engine::msgbox::open(e) {
                // The game took the box away (the pause menu, a load) without a click.
                self.shown = None;
                self.list.close();
            }
        }
        if self.shown.is_none() {
            if let Some(page) = self.list.page(&rows, host) {
                self.seen_open = false;
                if coop_engine::msgbox::show(e, &page.text, &page.buttons) {
                    self.shown = Some(page.buttons.len());
                } else {
                    lines.push("co-op players list: the game shows no message box now".into());
                    self.list.close();
                }
            }
        }
        kick
    }
}
