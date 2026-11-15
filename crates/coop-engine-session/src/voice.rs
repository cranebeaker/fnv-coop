//! voice: voice chat in the session (docs/coop/SPECS.md S36).
//!
//! The audio itself (microphone, codec, speakers) is the program's (`nv-viewer`'s `coop_voice`,
//! over `coop-voice`); this module carries its frames and says where the speakers stand:
//!
//! - [`VoiceLink::queue`] takes this player's encoded frames (`coop_voice::envelope` bytes, as
//!   `coop_voice::pipeline::VoiceTx` makes them). The next step sends each as coop-core's
//!   `SocialMsg::Voice` on the unreliable channel: a guest to the host, the host through its live
//!   layer, which checks the speaker and the rate (`VoiceHub`) and relays every frame to everyone
//!   but its speaker, the host's own player included.
//! - [`VoiceLink::take_heard`] gives the other players' frames, by the slot that sent them, in
//!   order (coop-core's `VoiceRx` drops repeats and late frames).
//! - [`VoiceLink::listener`] and [`VoiceLink::speakers`] are this player's place and the other
//!   players' places as the live ladder has them (their interpolated avatars), for positional
//!   sound; a player the ladder does not have yet has no place and is heard flat.
//!
//! No new wire message: `SocialMsg::Voice` was in coop-core's protocol from the start. The
//! settings ([`Settings`]) come from `NV_COOP_VOICE` and its companions.

use std::collections::BTreeMap;

use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::social::{SocialMsg, VoicePacket, MAX_VOICE_PAYLOAD};
use coop_core::sync::GuestPhase;
use coop_core::transport::Transport;
use engine_core::Engine;

use crate::{send, GuestDriver, HostDriver};

/// Frames waiting to be sent at most (about a second): an audio side that runs ahead of the
/// session's steps (a long frame, a load) drops its oldest frames instead of sending a burst.
pub const MAX_QUEUED: usize = 50;
/// Frames heard and not yet taken at most (about two seconds of four speakers).
pub const MAX_HEARD: usize = 400;

/// How the microphone opens: `NV_COOP_VOICE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// No voice chat: no microphone, no speakers opened.
    Off,
    /// The microphone sends while the push-to-talk key is held (the default).
    PushToTalk,
    /// The microphone sends whenever the voice is loud enough.
    Open,
}

impl Mode {
    /// `off` (or `0`), `ptt` (or nothing), `open` (or `on`, `vad`).
    pub fn parse(v: &str) -> Result<Mode, String> {
        match v.trim().to_ascii_lowercase().as_str() {
            "" | "ptt" => Ok(Mode::PushToTalk),
            "off" | "0" => Ok(Mode::Off),
            "open" | "on" | "vad" => Ok(Mode::Open),
            other => Err(format!("NV_COOP_VOICE={other}: use off, ptt or open")),
        }
    }

    /// The word for the log.
    pub fn word(self) -> &'static str {
        match self {
            Mode::Off => "off",
            Mode::PushToTalk => "push to talk",
            Mode::Open => "open microphone",
        }
    }
}

/// A player's place for positional sound.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Place {
    /// World position.
    pub pos: [f32; 3],
    /// Heading, radians (the game's: 0 faces +Y, clockwise).
    pub heading: f32,
    /// The space: an interior cell, or an exterior's world space (`coop_engine::live::Local`).
    pub space: u32,
}

/// The session's side of voice chat on one machine.
#[derive(Debug, Default)]
pub struct VoiceLink {
    out: Vec<Vec<u8>>,
    heard: Vec<(PlayerId, Vec<u8>)>,
    seq: u16,
    /// This player's place, read each step; `None` before the game has a player.
    pub listener: Option<Place>,
    /// The other players' places, by slot, each step (those the live ladder has in the world).
    pub speakers: BTreeMap<PlayerId, Place>,
    /// Frames sent.
    pub sent: u64,
    /// Frames heard (given to [`VoiceLink::take_heard`]).
    pub heard_frames: u64,
    /// Frames dropped: larger than a voice message carries, or queued too long.
    pub dropped: u64,
    /// Frames not sent because this guest was not in the session yet.
    pub not_joined: u64,
    // voiceui
    /// Who is speaking: each frame heard, and this player's microphone (the audio side tells).
    pub speaking: crate::voicelist::Speaking,
    /// The session's clock at the last step (ms), for the audio side's notes.
    pub now_ms: u64,
}

impl VoiceLink {
    /// An empty link.
    pub fn new() -> VoiceLink {
        VoiceLink::default()
    }

    /// One encoded frame of this player's microphone, sent on the next step.
    pub fn queue(&mut self, frame: Vec<u8>) {
        if frame.is_empty() || frame.len() > MAX_VOICE_PAYLOAD {
            self.dropped += 1;
            return;
        }
        if self.out.len() >= MAX_QUEUED {
            self.out.remove(0);
            self.dropped += 1;
        }
        self.out.push(frame);
    }

    /// The frames the other players sent since the last call, by their slot, in order.
    pub fn take_heard(&mut self) -> Vec<(PlayerId, Vec<u8>)> {
        std::mem::take(&mut self.heard)
    }

    /// Frames waiting to be sent.
    pub fn queued(&self) -> usize {
        self.out.len()
    }

    /// The queued frames as messages from `me`; the queue is emptied.
    fn messages(&mut self, me: PlayerId) -> Vec<Msg> {
        self.out
            .drain(..)
            .map(|payload| {
                self.seq = self.seq.wrapping_add(1);
                Msg::Social(SocialMsg::Voice(VoicePacket {
                    speaker: me,
                    seq: self.seq,
                    payload,
                }))
            })
            .collect()
    }

    /// Frames that arrived; this player's own (never relayed back, but checked) are left out.
    fn hear(&mut self, me: PlayerId, packets: Vec<VoicePacket>) {
        for p in packets {
            if p.speaker == me {
                continue;
            }
            if self.heard.len() >= MAX_HEARD {
                self.heard.remove(0);
                self.dropped += 1;
            }
            self.heard_frames += 1;
            self.speaking.heard(p.speaker, self.now_ms); // voiceui
            self.heard.push((p.speaker, p.payload));
        }
    }

    /// One line for the status: frames sent and heard, and who has a place.
    pub fn status(&self) -> String {
        format!(
            "co-op voice: frames sent {}, heard {}, dropped {}, not sent before joining {}, players with a place {}",
            self.sent,
            self.heard_frames,
            self.dropped,
            self.not_joined,
            self.speakers.len()
        )
    }
}

fn place_of(e: &mut Engine) -> Option<Place> {
    coop_engine::live::local(e).map(|l| Place {
        pos: l.pos,
        heading: l.heading,
        space: l.cell,
    })
}

fn place_of_sample(s: &coop_core::avatar::Sample) -> Option<Place> {
    (s.cell.0 != 0).then_some(Place {
        pos: s.pos,
        heading: s.heading,
        space: s.cell.0,
    })
}

/// The host's voice step, after its live step: the guests' frames the live layer gave the host
/// player, the places, and this player's frames to every guest.
pub(crate) fn host_step(h: &mut HostDriver, e: &mut Engine, net: &mut dyn Transport, now_ms: u64) {
    let Some(v) = h.voice.as_mut() else {
        return;
    };
    // What the live layer addressed to the host player since the social step drained it.
    for msg in h.sync.live.take_local() {
        h.view.on_msg(msg, now_ms);
    }
    v.now_ms = now_ms; // voiceui
    v.hear(PlayerId::HOST, std::mem::take(&mut h.view.voice_in));
    // voiceui: the speaking players' avatars marked on the compass.
    crate::voicelist::mark(e, h.ladder.as_ref(), &v.speaking.now(now_ms));
    v.listener = place_of(e);
    v.speakers.clear();
    for p in h.sync.players() {
        if let Some(place) = h.view.sample(p, now_ms).as_ref().and_then(place_of_sample) {
            v.speakers.insert(p, place);
        }
    }
    for msg in v.messages(PlayerId::HOST) {
        v.sent += 1;
        for (to, m) in h.sync.on_local(msg) {
            send(net, to, &m, &mut h.send_failures);
        }
    }
}

/// The guest's voice step: the frames the host relayed, the places, and this player's frames to
/// the host (only once the host has let this guest in: it drops them before).
pub(crate) fn guest_step(
    g: &mut GuestDriver,
    e: &mut Engine,
    net: &mut dyn Transport,
    now_ms: u64,
) {
    let Some(v) = g.voice.as_mut() else {
        return;
    };
    let me = g.sync.player;
    v.now_ms = now_ms; // voiceui
    v.hear(me, std::mem::take(&mut g.sync.live.voice_in));
    // voiceui: the speaking players' avatars marked on the compass.
    crate::voicelist::mark(e, g.ladder.as_ref(), &v.speaking.now(now_ms));
    v.listener = place_of(e);
    v.speakers.clear();
    let ids: Vec<PlayerId> = g.sync.live.remote.keys().copied().collect();
    for p in ids {
        if let Some(place) = g
            .sync
            .live
            .sample(p, now_ms)
            .as_ref()
            .and_then(place_of_sample)
        {
            v.speakers.insert(p, place);
        }
    }
    let msgs = v.messages(me);
    if g.sync.phase() != GuestPhase::Active {
        v.not_joined += msgs.len() as u64;
        return;
    }
    for msg in msgs {
        v.sent += 1;
        send(net, PlayerId::HOST, &msg, &mut g.send_failures);
    }
}

// The settings.

/// DirectInput key codes by name, for the keys a player may choose (the physical key: the
/// scan code, whatever the keyboard's layout prints on it).
const KEYS: &[(&str, u8)] = &[
    ("ESCAPE", 0x01),
    ("1", 0x02),
    ("2", 0x03),
    ("3", 0x04),
    ("4", 0x05),
    ("5", 0x06),
    ("6", 0x07),
    ("7", 0x08),
    ("8", 0x09),
    ("9", 0x0a),
    ("0", 0x0b),
    ("MINUS", 0x0c),
    ("EQUALS", 0x0d),
    ("BACKSPACE", 0x0e),
    ("TAB", 0x0f),
    ("Q", 0x10),
    ("W", 0x11),
    ("E", 0x12),
    ("R", 0x13),
    ("T", 0x14),
    ("Y", 0x15),
    ("U", 0x16),
    ("I", 0x17),
    ("O", 0x18),
    ("P", 0x19),
    ("LBRACKET", 0x1a),
    ("RBRACKET", 0x1b),
    ("ENTER", 0x1c),
    ("LCTRL", 0x1d),
    ("A", 0x1e),
    ("S", 0x1f),
    ("D", 0x20),
    ("F", 0x21),
    ("G", 0x22),
    ("H", 0x23),
    ("J", 0x24),
    ("K", 0x25),
    ("L", 0x26),
    ("SEMICOLON", 0x27),
    ("APOSTROPHE", 0x28),
    ("GRAVE", 0x29),
    ("LSHIFT", 0x2a),
    ("BACKSLASH", 0x2b),
    ("Z", 0x2c),
    ("X", 0x2d),
    ("C", 0x2e),
    ("V", 0x2f),
    ("B", 0x30),
    ("N", 0x31),
    ("M", 0x32),
    ("COMMA", 0x33),
    ("PERIOD", 0x34),
    ("SLASH", 0x35),
    ("RSHIFT", 0x36),
    ("NUMPADSTAR", 0x37),
    ("LALT", 0x38),
    ("SPACE", 0x39),
    ("CAPSLOCK", 0x3a),
    ("F1", 0x3b),
    ("F2", 0x3c),
    ("F3", 0x3d),
    ("F4", 0x3e),
    ("F5", 0x3f),
    ("F6", 0x40),
    ("F7", 0x41),
    ("F8", 0x42),
    ("F9", 0x43),
    ("F10", 0x44),
    ("NUMLOCK", 0x45),
    ("SCROLLLOCK", 0x46),
    ("NUMPAD7", 0x47),
    ("NUMPAD8", 0x48),
    ("NUMPAD9", 0x49),
    ("NUMPADMINUS", 0x4a),
    ("NUMPAD4", 0x4b),
    ("NUMPAD5", 0x4c),
    ("NUMPAD6", 0x4d),
    ("NUMPADPLUS", 0x4e),
    ("NUMPAD1", 0x4f),
    ("NUMPAD2", 0x50),
    ("NUMPAD3", 0x51),
    ("NUMPAD0", 0x52),
    ("NUMPADPERIOD", 0x53),
    ("F11", 0x57),
    ("F12", 0x58),
    ("NUMPADENTER", 0x9c),
    ("RCTRL", 0x9d),
    ("NUMPADSLASH", 0xb5),
    ("RALT", 0xb8),
    ("HOME", 0xc7),
    ("UP", 0xc8),
    ("PAGEUP", 0xc9),
    ("LEFT", 0xcb),
    ("RIGHT", 0xcd),
    ("END", 0xcf),
    ("DOWN", 0xd0),
    ("PAGEDOWN", 0xd1),
    ("INSERT", 0xd2),
    ("DELETE", 0xd3),
];

/// The names a key may be chosen by, in the order a list shows them.
pub fn key_names() -> impl Iterator<Item = &'static str> {
    KEYS.iter().map(|(n, _)| *n)
}

/// A key's DirectInput code from its name (any case; `CAPS LOCK` and `CAPS_LOCK` too), or from
/// its code in hexadecimal (`0x2B`).
pub fn dik_of(name: &str) -> Option<u8> {
    let n: String = name
        .trim()
        .chars()
        .filter(|c| !matches!(c, ' ' | '_' | '-'))
        .collect::<String>()
        .to_ascii_uppercase();
    if let Some(hex) = n.strip_prefix("0X") {
        return u8::from_str_radix(hex, 16).ok().filter(|c| *c != 0);
    }
    KEYS.iter().find(|(k, _)| *k == n).map(|(_, c)| *c)
}

/// A key's name for the log (its code in hexadecimal when it has none here).
pub fn key_name(dik: u8) -> String {
    KEYS.iter()
        .find(|(_, c)| *c == dik)
        .map_or_else(|| format!("0x{dik:02X}"), |(n, _)| (*n).to_string())
}

/// Voice chat's settings, from the environment (the launcher writes them).
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// `NV_COOP_VOICE`: off, push to talk (the default) or an open microphone.
    pub mode: Mode,
    /// `NV_COOP_PTT_KEY`: the push-to-talk key (default BACKSLASH, which the game binds to
    /// nothing).
    pub ptt_key: u8,
    /// `NV_COOP_MUTE_KEY`: turns this player's microphone off and on (default F8).
    pub mute_key: u8,
    /// `NV_COOP_DEAFEN_KEY`: turns the other players' voices off and on (default F7).
    pub deafen_key: u8,
    /// `NV_COOP_VOICE_VOLUME`: the other players' voices, 0 to 100 percent (default 100).
    pub master: f32,
    /// `NV_COOP_VOICE_MUTE`: players never heard, by name (comma separated, any case).
    pub muted: Vec<String>,
    /// `NV_COOP_VOICE_VOLUMES`: one player's volume, `name:percent` (0 to 200), comma separated.
    pub volumes: Vec<(String, f32)>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            mode: Mode::PushToTalk,
            ptt_key: 0x2b,
            mute_key: 0x42,
            deafen_key: 0x41,
            master: 1.0,
            muted: Vec::new(),
            volumes: Vec::new(),
        }
    }
}

impl Settings {
    /// The settings `var` gives (`std::env::var` in the program), with the lines that say what
    /// was not understood (each such value keeps its default).
    pub fn read(var: impl Fn(&str) -> Option<String>) -> (Settings, Vec<String>) {
        let mut s = Settings::default();
        let mut notes = Vec::new();
        if let Some(v) = var("NV_COOP_VOICE") {
            match Mode::parse(&v) {
                Ok(m) => s.mode = m,
                Err(e) => notes.push(format!("co-op voice: {e}; push to talk it is")),
            }
        }
        for (name, slot) in [
            ("NV_COOP_PTT_KEY", &mut s.ptt_key),
            ("NV_COOP_MUTE_KEY", &mut s.mute_key),
            ("NV_COOP_DEAFEN_KEY", &mut s.deafen_key),
        ] {
            if let Some(v) = var(name).filter(|v| !v.trim().is_empty()) {
                match dik_of(&v) {
                    Some(code) => *slot = code,
                    None => notes.push(format!(
                        "co-op voice: {name}={v} is not a key name; {} it is",
                        key_name(*slot)
                    )),
                }
            }
        }
        if let Some(v) = var("NV_COOP_VOICE_VOLUME").filter(|v| !v.trim().is_empty()) {
            match v.trim().trim_end_matches('%').parse::<f32>() {
                Ok(p) if p.is_finite() => s.master = (p / 100.0).clamp(0.0, 1.0),
                _ => notes.push(format!(
                    "co-op voice: NV_COOP_VOICE_VOLUME={v} is not a number from 0 to 100"
                )),
            }
        }
        if let Some(v) = var("NV_COOP_VOICE_MUTE") {
            s.muted = v
                .split(',')
                .map(|n| n.trim().to_ascii_lowercase())
                .filter(|n| !n.is_empty())
                .collect();
        }
        if let Some(v) = var("NV_COOP_VOICE_VOLUMES") {
            for item in v.split(',').map(str::trim).filter(|i| !i.is_empty()) {
                let parsed = item.rsplit_once(':').and_then(|(n, p)| {
                    let p = p.trim().trim_end_matches('%').parse::<f32>().ok()?;
                    let n = n.trim().to_ascii_lowercase();
                    (!n.is_empty() && p.is_finite()).then_some((n, (p / 100.0).clamp(0.0, 2.0)))
                });
                match parsed {
                    Some(nv) => s.volumes.push(nv),
                    None => notes.push(format!(
                        "co-op voice: NV_COOP_VOICE_VOLUMES item {item:?} is not name:percent"
                    )),
                }
            }
        }
        (s, notes)
    }

    /// Whether the player named `name` is muted.
    pub fn is_muted(&self, name: &str) -> bool {
        let n = name.trim().to_ascii_lowercase();
        self.muted.contains(&n)
    }

    /// The volume of the player named `name` (1 is as sent).
    pub fn volume_of(&self, name: &str) -> f32 {
        let n = name.trim().to_ascii_lowercase();
        self.volumes
            .iter()
            .rev()
            .find(|(k, _)| *k == n)
            .map_or(1.0, |(_, v)| *v)
    }

    /// The line that says what voice chat does.
    pub fn line(&self) -> String {
        match self.mode {
            Mode::Off => "co-op voice: off (NV_COOP_VOICE=ptt or open turns it on)".into(),
            Mode::PushToTalk => format!(
                "co-op voice: push to talk, hold {} (NV_COOP_PTT_KEY); {} mutes the microphone, {} the others; volume {:.0}%",
                key_name(self.ptt_key),
                key_name(self.mute_key),
                key_name(self.deafen_key),
                self.master * 100.0
            ),
            Mode::Open => format!(
                "co-op voice: open microphone (sends when you speak); {} mutes the microphone, {} the others; volume {:.0}%",
                key_name(self.mute_key),
                key_name(self.deafen_key),
                self.master * 100.0
            ),
        }
    }
}

/// A key's presses: turns a held state into presses (the first frame it is down).
#[derive(Clone, Copy, Debug, Default)]
pub struct Press {
    was: bool,
}

impl Press {
    /// Whether the key went down this frame.
    pub fn pressed(&mut self, down: bool) -> bool {
        let p = down && !self.was;
        self.was = down;
        p
    }
}

/// How many 20 ms frames of the other players' voices are due: the audio side mixes by the time
/// that passed between two of the game's frames, at most [`FrameClock::MAX_DUE`] at once (after
/// a load or a long frame the late frames are skipped, not played in a rush).
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameClock {
    acc_ms: f64,
}

impl FrameClock {
    /// Most frames mixed in one call.
    pub const MAX_DUE: u32 = 10;

    /// The frames due after `elapsed_ms`, for frames of `frame_ms`.
    pub fn due(&mut self, elapsed_ms: f64, frame_ms: f64) -> u32 {
        if frame_ms.is_nan() || frame_ms <= 0.0 || !elapsed_ms.is_finite() || elapsed_ms < 0.0 {
            return 0;
        }
        self.acc_ms += elapsed_ms;
        let n = (self.acc_ms / frame_ms).floor();
        if n > f64::from(Self::MAX_DUE) {
            self.acc_ms = 0.0;
            return Self::MAX_DUE;
        }
        self.acc_ms -= n * frame_ms;
        n as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_are_due_by_the_time_that_passed_and_a_long_wait_is_not_played_in_a_rush() {
        let mut c = FrameClock::default();
        assert_eq!(c.due(10.0, 20.0), 0);
        assert_eq!(c.due(10.0, 20.0), 1);
        assert_eq!(c.due(45.0, 20.0), 2);
        assert_eq!(c.due(15.0, 20.0), 1, "5 ms were left over");
        assert_eq!(c.due(5000.0, 20.0), FrameClock::MAX_DUE);
        assert_eq!(
            c.due(10.0, 20.0),
            0,
            "nothing carried over from the long wait"
        );
        assert_eq!(c.due(-1.0, 20.0), 0);
        assert_eq!(c.due(f64::NAN, 20.0), 0);
        assert_eq!(c.due(50.0, 0.0), 0);
    }

    fn env<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn the_mode_is_push_to_talk_unless_asked() {
        assert_eq!(Mode::parse(""), Ok(Mode::PushToTalk));
        assert_eq!(Mode::parse(" PTT "), Ok(Mode::PushToTalk));
        assert_eq!(Mode::parse("off"), Ok(Mode::Off));
        assert_eq!(Mode::parse("0"), Ok(Mode::Off));
        assert_eq!(Mode::parse("open"), Ok(Mode::Open));
        assert_eq!(Mode::parse("vad"), Ok(Mode::Open));
        assert!(Mode::parse("loud").is_err());
    }

    #[test]
    fn keys_are_named_or_given_as_codes() {
        assert_eq!(dik_of("backslash"), Some(0x2b));
        assert_eq!(dik_of("Caps Lock"), Some(0x3a));
        assert_eq!(dik_of("caps_lock"), Some(0x3a));
        assert_eq!(dik_of("f8"), Some(0x42));
        assert_eq!(dik_of("0x2F"), Some(0x2f));
        assert_eq!(dik_of("0x00"), None);
        assert_eq!(dik_of("nokey"), None);
        assert_eq!(key_name(0x2b), "BACKSLASH");
        assert_eq!(key_name(0xee), "0xEE");
        // Every name reads back as itself, and no code has two names.
        for name in key_names() {
            assert_eq!(key_name(dik_of(name).unwrap()), name);
        }
    }

    #[test]
    fn settings_default_and_read() {
        let (s, notes) = Settings::read(env(&[]));
        assert_eq!(s, Settings::default());
        assert!(notes.is_empty());
        let (s, notes) = Settings::read(env(&[
            ("NV_COOP_VOICE", "open"),
            ("NV_COOP_PTT_KEY", "CAPSLOCK"),
            ("NV_COOP_MUTE_KEY", "F9"),
            ("NV_COOP_VOICE_VOLUME", "40%"),
            ("NV_COOP_VOICE_MUTE", " Ann , bob,"),
            ("NV_COOP_VOICE_VOLUMES", "Cid:150, dee:300"),
        ]));
        assert!(notes.is_empty(), "{notes:?}");
        assert_eq!(s.mode, Mode::Open);
        assert_eq!(s.ptt_key, 0x3a);
        assert_eq!(s.mute_key, 0x43);
        assert_eq!(s.deafen_key, 0x41);
        assert!((s.master - 0.4).abs() < 1e-6);
        assert!(s.is_muted("ANN") && s.is_muted("bob") && !s.is_muted("cid"));
        assert!((s.volume_of("cid") - 1.5).abs() < 1e-6);
        assert!((s.volume_of("Dee") - 2.0).abs() < 1e-6, "clamped to 200%");
        assert!((s.volume_of("nobody") - 1.0).abs() < 1e-6);
    }

    #[test]
    fn settings_keep_their_default_and_say_so_for_a_bad_value() {
        let (s, notes) = Settings::read(env(&[
            ("NV_COOP_VOICE", "loud"),
            ("NV_COOP_PTT_KEY", "the big one"),
            ("NV_COOP_VOICE_VOLUME", "much"),
            ("NV_COOP_VOICE_VOLUMES", "nocolon"),
        ]));
        assert_eq!(s.mode, Mode::PushToTalk);
        assert_eq!(s.ptt_key, 0x2b);
        assert!((s.master - 1.0).abs() < 1e-6);
        assert_eq!(notes.len(), 4, "{notes:?}");
        assert!(notes[1].contains("BACKSLASH"));
    }

    #[test]
    fn the_line_names_the_keys() {
        let line = Settings::default().line();
        assert!(line.contains("BACKSLASH") && line.contains("F8") && line.contains("F7"));
        let off = Settings {
            mode: Mode::Off,
            ..Settings::default()
        };
        assert!(off.line().contains("off"));
    }

    #[test]
    fn a_press_is_the_first_frame_down() {
        let mut p = Press::default();
        assert!(!p.pressed(false));
        assert!(p.pressed(true));
        assert!(!p.pressed(true));
        assert!(!p.pressed(false));
        assert!(p.pressed(true));
    }

    #[test]
    fn frames_become_numbered_voice_messages_and_the_queue_is_bounded() {
        let mut v = VoiceLink::new();
        v.queue(vec![]);
        v.queue(vec![0; MAX_VOICE_PAYLOAD + 1]);
        assert_eq!((v.queued(), v.dropped), (0, 2));
        for i in 0..(MAX_QUEUED + 5) {
            v.queue(vec![i as u8; 10]);
        }
        assert_eq!(v.queued(), MAX_QUEUED);
        assert_eq!(v.dropped, 7);
        let msgs = v.messages(PlayerId(2));
        assert_eq!(msgs.len(), MAX_QUEUED);
        assert_eq!(v.queued(), 0);
        let seqs: Vec<u16> = msgs
            .iter()
            .map(|m| match m {
                Msg::Social(SocialMsg::Voice(p)) => {
                    assert_eq!(p.speaker, PlayerId(2));
                    p.seq
                }
                other => panic!("{other:?}"),
            })
            .collect();
        assert_eq!(seqs, (1..=MAX_QUEUED as u16).collect::<Vec<_>>());
        // The oldest were dropped: the first sent is the sixth queued.
        match &msgs[0] {
            Msg::Social(SocialMsg::Voice(p)) => assert_eq!(p.payload, vec![5; 10]),
            _ => unreachable!(),
        }
        // Every message fits the unreliable channel.
        for m in &msgs {
            assert!(coop_core::proto::encode(m).is_ok());
        }
    }

    #[test]
    fn own_frames_are_not_heard_and_the_heard_list_is_bounded() {
        let mut v = VoiceLink::new();
        let pk = |s: u8, n: u8| VoicePacket {
            speaker: PlayerId(s),
            seq: n as u16,
            payload: vec![n],
        };
        v.hear(PlayerId(1), vec![pk(1, 1), pk(0, 2), pk(2, 3)]);
        assert_eq!(
            v.take_heard(),
            vec![(PlayerId(0), vec![2]), (PlayerId(2), vec![3])]
        );
        assert!(v.take_heard().is_empty());
        let many: Vec<VoicePacket> = (0..(MAX_HEARD + 3)).map(|i| pk(0, i as u8)).collect();
        v.hear(PlayerId(1), many);
        assert_eq!(v.take_heard().len(), MAX_HEARD);
        assert_eq!(v.dropped, 3);
        assert_eq!(v.heard_frames, 2 + MAX_HEARD as u64 + 3);
    }
}
