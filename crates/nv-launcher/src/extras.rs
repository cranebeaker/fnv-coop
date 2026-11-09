//! Viewer options that are not part of the basic host/join plan: the
//! compatibility override, voice settings, and the couch (split-screen on
//! one PC) flags another launcher uses. See docs/COOP-COUCH.md.
//!
//! Everything here is pure: a value in, viewer arguments out.

/// A window rectangle in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    /// `X,Y,W,H`.
    pub fn parse(s: &str) -> Result<Rect, String> {
        let p: Vec<&str> = s.split(',').map(str::trim).collect();
        if p.len() != 4 {
            return Err(format!("'{s}' is not X,Y,W,H"));
        }
        let n = |i: usize| {
            p[i].parse::<i32>()
                .map_err(|_| format!("'{}' is not a number in '{s}'", p[i]))
        };
        let (x, y, w, h) = (n(0)?, n(1)?, n(2)?, n(3)?);
        if w < 160 || h < 120 {
            return Err(format!("a window of {w}x{h} is too small"));
        }
        Ok(Rect {
            x,
            y,
            w: w as u32,
            h: h as u32,
        })
    }

    /// The viewer's argument text.
    pub fn arg(&self) -> String {
        format!("{},{},{},{}", self.x, self.y, self.w, self.h)
    }
}

/// This process's place among the local players of one PC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Couch {
    /// 0-based seat among the local players (Couchlink's `$SEAT`).
    pub slot: u8,
    /// How many local players share this PC (`$PLAYERS`).
    pub players: u8,
}

impl Couch {
    /// `SEAT/PLAYERS`, e.g. `1/4` (seat 0 is the first player).
    pub fn parse(s: &str) -> Result<Couch, String> {
        let (a, b) = s
            .split_once('/')
            .ok_or_else(|| format!("'{s}' is not SEAT/PLAYERS (for example 1/4)"))?;
        let slot: u8 = a
            .trim()
            .parse()
            .map_err(|_| format!("'{a}' is not a slot number"))?;
        let players: u8 = b
            .trim()
            .parse()
            .map_err(|_| format!("'{b}' is not a player count"))?;
        if !(1..=4).contains(&players) || slot >= players {
            return Err(format!(
                "seat {slot} of {players}: players must be 1 to 4 and the seat 0 to players-1"
            ));
        }
        Ok(Couch { slot, players })
    }

    /// The first seat owns the microphone and the speakers' voice chat.
    pub fn owns_voice(&self) -> bool {
        self.slot == 0
    }
}

/// How the microphone opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceMode {
    Off,
    PushToTalk,
    Activity,
}

impl VoiceMode {
    pub fn parse(s: &str) -> Result<VoiceMode, String> {
        match s.to_ascii_lowercase().as_str() {
            "off" => Ok(VoiceMode::Off),
            "ptt" => Ok(VoiceMode::PushToTalk),
            "vad" => Ok(VoiceMode::Activity),
            other => Err(format!("voice mode '{other}': use off, ptt or vad")),
        }
    }

    pub fn arg(&self) -> &'static str {
        match self {
            VoiceMode::Off => "off",
            VoiceMode::PushToTalk => "ptt",
            VoiceMode::Activity => "vad",
        }
    }
}

/// The extra viewer options.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Extras {
    /// `--coop-allow-mismatch`: developer override of the compatibility check.
    pub allow_mismatch: bool,
    /// `--coop-couch SEAT/PLAYERS`.
    pub couch: Option<Couch>,
    /// `--coop-window X,Y,W,H`.
    pub window: Option<Rect>,
    /// `--coop-gamepad N`: the controller this process listens to.
    pub gamepad: Option<u8>,
    /// `--coop-voice MODE`.
    pub voice: Option<VoiceMode>,
    /// `--coop-ptt-key KEY`.
    pub ptt_key: Option<String>,
    /// `--coop-ping-key KEY`.
    pub ping_key: Option<String>,
    /// Host: `--coop-difficulty normal|balanced`.
    pub difficulty: Option<Difficulty>,
    /// Host: `--coop-friendly-fire on|off`.
    pub friendly_fire: Option<bool>,
}

/// The host's campaign difficulty mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Single-player numbers whatever the party size.
    Normal,
    /// Scaled for the party (the viewer's default).
    Balanced,
}

impl Difficulty {
    pub fn parse(s: &str) -> Result<Difficulty, String> {
        match s.to_ascii_lowercase().as_str() {
            "normal" => Ok(Difficulty::Normal),
            "balanced" => Ok(Difficulty::Balanced),
            other => Err(format!("difficulty '{other}': use normal or balanced")),
        }
    }

    pub fn arg(&self) -> &'static str {
        match self {
            Difficulty::Normal => "normal",
            Difficulty::Balanced => "balanced",
        }
    }
}

/// `on` or `off`.
pub fn parse_switch(s: &str) -> Result<bool, String> {
    match s.to_ascii_lowercase().as_str() {
        "on" => Ok(true),
        "off" => Ok(false),
        other => Err(format!("'{other}': use on or off")),
    }
}

impl Extras {
    /// The host-only viewer arguments: the campaign rules. The viewer keeps
    /// them in the party save, so a Continue without them plays on with the
    /// saved rules.
    pub fn host_args(&self) -> Vec<String> {
        let mut a = Vec::new();
        if let Some(d) = self.difficulty {
            a.push("--coop-difficulty".into());
            a.push(d.arg().into());
        }
        if let Some(f) = self.friendly_fire {
            a.push("--coop-friendly-fire".into());
            a.push(if f { "on" } else { "off" }.into());
        }
        a
    }

    /// The viewer arguments. A couch player that does not own the voice gets
    /// voice off unless the caller said otherwise, so one PC never opens
    /// several microphones into the same session.
    pub fn args(&self) -> Vec<String> {
        let mut a = Vec::new();
        if self.allow_mismatch {
            a.push("--coop-allow-mismatch".into());
        }
        if let Some(c) = &self.couch {
            a.push("--coop-couch".into());
            a.push(format!("{}/{}", c.slot, c.players));
        }
        if let Some(w) = &self.window {
            a.push("--coop-window".into());
            a.push(w.arg());
        }
        if let Some(g) = self.gamepad {
            a.push("--coop-gamepad".into());
            a.push(g.to_string());
        }
        let voice = self.voice.or_else(|| {
            self.couch
                .filter(|c| !c.owns_voice())
                .map(|_| VoiceMode::Off)
        });
        if let Some(v) = voice {
            a.push("--coop-voice".into());
            a.push(v.arg().into());
        }
        if let Some(k) = &self.ptt_key {
            a.push("--coop-ptt-key".into());
            a.push(k.clone());
        }
        if let Some(k) = &self.ping_key {
            a.push("--coop-ping-key".into());
            a.push(k.clone());
        }
        a
    }
}

/// Window rectangles for `players` local players on a `w` x `h` screen
/// (`$LAYOUT`): one fills the screen; two stack top and bottom; three are
/// two on top and one centred below; four are quarters.
pub fn layout(players: u8, w: u32, h: u32) -> Vec<Rect> {
    let (hw, hh) = (w / 2, h / 2);
    let r = |x: u32, y: u32, w: u32, h: u32| Rect {
        x: x as i32,
        y: y as i32,
        w,
        h,
    };
    match players {
        0 | 1 => vec![r(0, 0, w, h)],
        2 => vec![r(0, 0, w, hh), r(0, hh, w, h - hh)],
        3 => vec![
            r(0, 0, hw, hh),
            r(hw, 0, w - hw, hh),
            r(w / 4, hh, hw, h - hh),
        ],
        _ => vec![
            r(0, 0, hw, hh),
            r(hw, 0, w - hw, hh),
            r(0, hh, hw, h - hh),
            r(hw, hh, w - hw, h - hh),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_and_couch_parse() {
        assert_eq!(
            Rect::parse("0, 540,1920,540").unwrap(),
            Rect {
                x: 0,
                y: 540,
                w: 1920,
                h: 540
            }
        );
        assert!(Rect::parse("1,2,3").is_err());
        assert!(Rect::parse("0,0,10,10").unwrap_err().contains("too small"));
        assert_eq!(
            Couch::parse("3/4").unwrap(),
            Couch {
                slot: 3,
                players: 4
            }
        );
        assert_eq!(
            Couch::parse("0/1").unwrap(),
            Couch {
                slot: 0,
                players: 1
            }
        );
        for bad in ["2/2", "4/4", "0/5", "x", "0/0", "1-2"] {
            assert!(Couch::parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn nothing_set_means_no_arguments() {
        assert!(Extras::default().args().is_empty());
        assert!(Extras::default().host_args().is_empty());
    }

    #[test]
    fn the_host_chooses_the_campaign_rules() {
        let e = Extras {
            difficulty: Some(Difficulty::parse("Normal").unwrap()),
            friendly_fire: Some(parse_switch("on").unwrap()),
            ..Extras::default()
        };
        assert_eq!(
            e.host_args(),
            ["--coop-difficulty", "normal", "--coop-friendly-fire", "on"]
        );
        assert!(e.args().is_empty(), "a guest never gets the rules flags");
        assert!(Difficulty::parse("hard").is_err());
        assert!(parse_switch("maybe").is_err());
    }

    #[test]
    fn override_and_voice_flags() {
        let e = Extras {
            allow_mismatch: true,
            voice: Some(VoiceMode::Activity),
            ptt_key: Some("Backslash".into()),
            ping_key: Some("MouseMiddle".into()),
            ..Extras::default()
        };
        assert_eq!(
            e.args(),
            [
                "--coop-allow-mismatch",
                "--coop-voice",
                "vad",
                "--coop-ptt-key",
                "Backslash",
                "--coop-ping-key",
                "MouseMiddle"
            ]
        );
    }

    #[test]
    fn only_the_first_couch_player_owns_the_microphone() {
        let first = Extras {
            couch: Some(Couch {
                slot: 0,
                players: 2,
            }),
            ..Extras::default()
        };
        assert!(!first.args().contains(&"--coop-voice".to_string()));
        let second = Extras {
            couch: Some(Couch {
                slot: 1,
                players: 2,
            }),
            gamepad: Some(1),
            window: Some(Rect {
                x: 0,
                y: 540,
                w: 1920,
                h: 540,
            }),
            ..Extras::default()
        };
        assert_eq!(
            second.args(),
            [
                "--coop-couch",
                "1/2",
                "--coop-window",
                "0,540,1920,540",
                "--coop-gamepad",
                "1",
                "--coop-voice",
                "off"
            ]
        );
        let forced = Extras {
            voice: Some(VoiceMode::PushToTalk),
            ..second
        };
        assert!(forced
            .args()
            .windows(2)
            .any(|w| w == ["--coop-voice", "ptt"]));
    }

    #[test]
    fn layouts_tile_the_screen_without_overlap() {
        for n in 1..=4u8 {
            let l = layout(n, 1920, 1080);
            assert_eq!(l.len(), n as usize);
            for (i, a) in l.iter().enumerate() {
                assert!(
                    a.x >= 0 && a.y >= 0 && a.x as u32 + a.w <= 1920 && a.y as u32 + a.h <= 1080,
                    "{a:?}"
                );
                for b in l.iter().skip(i + 1) {
                    let apart = a.x as u32 + a.w <= b.x as u32
                        || b.x as u32 + b.w <= a.x as u32
                        || a.y as u32 + a.h <= b.y as u32
                        || b.y as u32 + b.h <= a.y as u32;
                    assert!(apart, "{a:?} overlaps {b:?}");
                }
            }
        }
        assert_eq!(
            layout(2, 1920, 1080)[1],
            Rect {
                x: 0,
                y: 540,
                w: 1920,
                h: 540
            }
        );
        assert_eq!(
            layout(4, 1920, 1080)[3],
            Rect {
                x: 960,
                y: 540,
                w: 960,
                h: 540
            }
        );
    }
}

/// The player-facing refusal in a viewer co-op log (JSON lines), if the host turned this guest away:
/// the text after the reason code, e.g. "You cannot join this game. You are missing 1 plugin ...".
/// The newest refusal wins.
pub fn refusal_from_log(log: &str) -> Option<String> {
    const KEY: &str = "host refused the connection: ";
    let mut found = None;
    for line in log.lines() {
        let Ok(v) = crate::json::parse(line) else {
            continue;
        };
        if v.get("ev").and_then(|e| e.as_str()) != Some("error") {
            continue;
        }
        let Some(msg) = v.get("msg").and_then(|m| m.as_str()) else {
            continue;
        };
        if let Some(rest) = msg.strip_prefix(KEY) {
            // "<ReasonCode>: <text>"
            let text = rest.split_once(": ").map_or(rest, |(_, t)| t);
            found = Some(text.trim().to_string());
        }
    }
    found
}

#[cfg(test)]
mod refusal_tests {
    use super::*;

    #[test]
    fn reads_the_newest_refusal_text() {
        let log = concat!(
            "{\"t\":1.0,\"ev\":\"start\"}\n",
            "{\"t\":2.0,\"ev\":\"error\",\"msg\":\"host refused the connection: BuildMismatch: The nv-rs builds differ.\"}\n",
            "not json\n",
            "{\"t\":3.0,\"ev\":\"error\",\"msg\":\"host refused the connection: PluginsMismatch: You cannot join this game. You are missing 1 plugin that the host has: DeadMoney.esm.\"}\n",
            "{\"t\":4.0,\"ev\":\"error\",\"msg\":\"something else\"}\n",
        );
        let t = refusal_from_log(log).unwrap();
        assert!(
            t.starts_with("You cannot join this game.") && t.contains("DeadMoney.esm"),
            "{t}"
        );
        assert_eq!(refusal_from_log("{\"ev\":\"start\"}\n"), None);
    }
}
