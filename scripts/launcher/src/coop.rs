//! The co-op window's choices (docs/coop/SPECS.md S38): what the player picked, kept in
//! `userdata\coop.ini`, and the game's environment it makes (the same switches `Play Co-op.cmd`
//! sets, and the ones it never asked: Steam, voice, the off-by-default parts). No window here:
//! `coop_ui` shows it.

use std::path::{Path, PathBuf};

/// The file the choices are kept in, in the package's `userdata`.
pub const FILE: &str = "coop.ini";

/// Valve's Steam library, next to `Play.exe` when the player (or the package) put it there.
pub const STEAM_DLL: &str = "steam_api64.dll";

/// The switches the co-op window sets or clears; one left from settings.ini or an earlier
/// console never mixes into a run the window started.
pub const SWITCHES: [&str; 14] = [
    "NV_COOP",
    "NV_COOP_NAME",
    "NV_COOP_ADDRESS",
    "NV_COOP_STEAM",
    "NV_COOP_STEAM_DLL",
    "NV_COOP_VOICE",
    "NV_COOP_PTT_KEY",
    "NV_COOP_ITEMS",
    "NV_COOP_VATS",
    "NV_COOP_HARM",
    "NV_COOP_INVITE_KEY",
    // voiceui
    "NV_COOP_PLAYERS_KEY",
    "NV_COOP_MUTE_KEY",
    "NV_COOP_DEAFEN_KEY",
];

/// The keys the push-to-talk list offers (the game program's names, `coop_engine_session::voice`;
/// any of its names works in `NV_COOP_PTT_KEY`). The game leaves BACKSLASH, the default, unbound;
/// with another key the game sees the key too.
pub const PTT_KEYS: [&str; 16] = [
    "BACKSLASH",
    "CAPSLOCK",
    "B",
    "G",
    "H",
    "N",
    "X",
    "LALT",
    "RALT",
    "RCTRL",
    "F1",
    "F2",
    "F3",
    "F4",
    "NUMPAD0",
    "INSERT",
];

// voiceui
/// The keys the players list, mute and deafen lists offer (the game program's names; F11, F8 and
/// F7 are the game program's defaults for them). F5 and F9 are left out: the game binds them by
/// default (quick save and quick load, the keyboard defaults `fn_00a24b70` writes,
/// `run_00a237f0.rs`).
pub const LIST_KEYS: [&str; 16] = [
    "F1", "F2", "F3", "F4", "F6", "F7", "F8", "F10", "F11", "F12", "INSERT", "DELETE", "HOME",
    "END", "PAGEUP", "PAGEDOWN",
];

// voiceui
/// One other player's voice as this player set it (in the game's players list or this window),
/// by name: Player=name;muted;volume in coop.ini, the game program's own lines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerVoice {
    pub name: String,
    pub muted: bool,
    /// Percent, 0 to 200 (100 is as sent).
    pub volume: u32,
}

impl PlayerVoice {
    /// A Player= line's value; None when it is not one.
    fn parse(v: &str) -> Option<PlayerVoice> {
        let mut parts = v.split(';');
        let name = parts.next()?.trim();
        let muted = flag(parts.next()?);
        let volume = parts
            .next()?
            .trim()
            .trim_end_matches('%')
            .parse::<f32>()
            .ok()?;
        (!name.is_empty() && volume.is_finite()).then(|| PlayerVoice {
            name: name.to_string(),
            muted,
            volume: volume.clamp(0.0, 200.0).round() as u32,
        })
    }

    fn line(&self) -> String {
        format!(
            "Player={};{};{}\r\n",
            one_line(&self.name).replace(';', "_"),
            u8::from(self.muted),
            self.volume.min(200)
        )
    }
}

/// What the player pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// The game alone, no session.
    Alone,
    /// Host the story; Steam beside it when ticked and there.
    Host,
    /// Host with Steam and open the overlay's invite dialog once in the world.
    InviteSteamFriend,
    /// Join with the host's join code.
    Join,
    /// Join a Steam friend who hosts, or the first invite.
    JoinSteamFriend,
}

/// Voice chat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voice {
    Off,
    PushToTalk,
    Open,
}

impl Voice {
    /// The order the window's list shows them in.
    pub const ALL: [Voice; 3] = [Voice::PushToTalk, Voice::Open, Voice::Off];

    pub fn label(self) -> &'static str {
        match self {
            Voice::Off => "Off",
            Voice::PushToTalk => "Push to talk",
            Voice::Open => "Open microphone",
        }
    }

    fn word(self) -> &'static str {
        match self {
            Voice::Off => "off",
            Voice::PushToTalk => "ptt",
            Voice::Open => "open",
        }
    }

    fn parse(v: &str) -> Voice {
        match v.trim().to_ascii_lowercase().as_str() {
            "off" | "0" => Voice::Off,
            "open" => Voice::Open,
            _ => Voice::PushToTalk,
        }
    }
}

/// Everything the window asks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    pub name: String,
    /// The address for the host's join code (empty: this home network's).
    pub address: String,
    /// The last join code pasted.
    pub code: String,
    /// A host also makes a Steam lobby.
    pub steam: bool,
    pub voice: Voice,
    pub ptt_key: String,
    pub items: bool,
    pub vats: bool,
    pub harm: bool,
    // voiceui
    /// The key that opens the players list in the game, and the mute and deafen keys.
    pub players_key: String,
    pub mute_key: String,
    pub deafen_key: String,
    /// The other players' voices by name.
    pub players: Vec<PlayerVoice>,
}

impl Default for Choice {
    fn default() -> Choice {
        Choice {
            name: "player".into(),
            address: String::new(),
            code: String::new(),
            steam: true,
            voice: Voice::PushToTalk,
            ptt_key: PTT_KEYS[0].into(),
            items: false,
            vats: false,
            harm: false,
            // voiceui
            players_key: "F11".into(),
            mute_key: "F8".into(),
            deafen_key: "F7".into(),
            players: Vec::new(),
        }
    }
}

fn flag(v: &str) -> bool {
    matches!(v.trim(), "1" | "on" | "yes" | "true")
}

impl Choice {
    /// The choices `coop.ini` holds (a line it does not know is skipped; a missing one keeps
    /// its default).
    pub fn parse(text: &str) -> Choice {
        let mut c = Choice::default();
        for line in text.lines() {
            let Some((k, v)) = line.split_once('=') else {
                continue;
            };
            let v = v.trim();
            match k.trim().to_ascii_lowercase().as_str() {
                "name" => c.name = v.into(),
                "address" => c.address = v.into(),
                "code" => c.code = v.into(),
                "steam" => c.steam = flag(v),
                "voice" => c.voice = Voice::parse(v),
                "pushtotalkkey" if !v.is_empty() => c.ptt_key = v.into(),
                "items" => c.items = flag(v),
                "vats" => c.vats = flag(v),
                "harm" => c.harm = flag(v),
                // voiceui
                "playerskey" if !v.is_empty() => c.players_key = v.into(),
                "mutekey" if !v.is_empty() => c.mute_key = v.into(),
                "deafenkey" if !v.is_empty() => c.deafen_key = v.into(),
                "player" => {
                    if let Some(p) = PlayerVoice::parse(v) {
                        c.players.retain(|q| !q.name.eq_ignore_ascii_case(&p.name));
                        c.players.push(p);
                    }
                }
                _ => {}
            }
        }
        c
    }

    /// The text of `coop.ini`.
    pub fn to_ini(&self) -> String {
        let b = |on: bool| if on { "1" } else { "0" };
        let mut text = format!(
            "; The co-op window's choices (Play Co-op.exe). Delete this file to start again.\r\nName={}\r\nAddress={}\r\nCode={}\r\nSteam={}\r\nVoice={}\r\nPushToTalkKey={}\r\nItems={}\r\nVats={}\r\nHarm={}\r\n",
            one_line(&self.name),
            one_line(&self.address),
            one_line(&self.code),
            b(self.steam),
            self.voice.word(),
            one_line(&self.ptt_key),
            b(self.items),
            b(self.vats),
            b(self.harm)
        );
        // voiceui
        text.push_str(&format!(
            "PlayersKey={}\r\nMuteKey={}\r\nDeafenKey={}\r\n",
            one_line(&self.players_key),
            one_line(&self.mute_key),
            one_line(&self.deafen_key)
        ));
        for p in &self.players {
            if p.muted || p.volume != 100 {
                text.push_str(&p.line());
            }
        }
        text
    }

    /// The game's environment for `mode`: each switch set (`Some`) or cleared (`None`). `dll` is
    /// the Steam library next to `Play.exe`, if there is one. `Err` says, in the player's words,
    /// what to fix.
    pub fn environment(
        &self,
        mode: Mode,
        dll: Option<&Path>,
    ) -> Result<Vec<(&'static str, Option<String>)>, String> {
        let mut env: Vec<(&'static str, Option<String>)> =
            SWITCHES.iter().map(|s| (*s, None)).collect();
        let mut set = |name: &'static str, value: String| {
            if let Some(slot) = env.iter_mut().find(|(n, _)| *n == name) {
                slot.1 = Some(value);
            }
        };
        if mode == Mode::Alone {
            return Ok(env);
        }
        let name = valid_name(&self.name)?;
        set("NV_COOP_NAME", name);
        let steam_mode = matches!(mode, Mode::InviteSteamFriend | Mode::JoinSteamFriend)
            || (mode == Mode::Host && self.steam);
        if steam_mode {
            match dll {
                Some(p) => set("NV_COOP_STEAM_DLL", p.display().to_string()),
                None if mode != Mode::Host => {
                    return Err(format!("Steam needs {STEAM_DLL} next to Play.exe, and it is not there. Put Valve's 64-bit {STEAM_DLL} (from the Steamworks SDK) next to Play.exe, or use a join code."));
                }
                None => {}
            }
        }
        match mode {
            Mode::Alone => unreachable!(),
            Mode::Host | Mode::InviteSteamFriend => {
                set("NV_COOP", "host".into());
                let address = self.address.trim();
                if !address.is_empty() {
                    if address.parse::<std::net::IpAddr>().is_err() {
                        return Err(format!("\"{address}\" is not an address. Type one like 203.0.113.7, or leave the box empty for friends in your home network."));
                    }
                    set("NV_COOP_ADDRESS", address.into());
                }
                let steam = match mode {
                    Mode::InviteSteamFriend => Some("invite"),
                    _ if self.steam && dll.is_some() => Some("on"),
                    _ => None,
                };
                if let Some(s) = steam {
                    set("NV_COOP_STEAM", s.into());
                }
            }
            Mode::Join => {
                let code = valid_code(&self.code)?;
                set("NV_COOP", format!("join:{code}"));
            }
            Mode::JoinSteamFriend => set("NV_COOP", "steam".into()),
        }
        set("NV_COOP_VOICE", self.voice.word().into());
        // voiceui
        set("NV_COOP_PLAYERS_KEY", one_line(&self.players_key));
        if self.voice != Voice::Off {
            set("NV_COOP_MUTE_KEY", one_line(&self.mute_key));
            set("NV_COOP_DEAFEN_KEY", one_line(&self.deafen_key));
        }
        if self.voice == Voice::PushToTalk {
            set("NV_COOP_PTT_KEY", one_line(&self.ptt_key));
        }
        for (on, name) in [
            (self.items, "NV_COOP_ITEMS"),
            (self.vats, "NV_COOP_VATS"),
            (self.harm, "NV_COOP_HARM"),
        ] {
            if on {
                set(name, "on".into());
            }
        }
        Ok(env)
    }
}

fn one_line(v: &str) -> String {
    v.replace(['\r', '\n'], " ").trim().to_string()
}

/// A player name: letters, digits, spaces, `-` and `_`, 1 to 24 of them.
pub fn valid_name(v: &str) -> Result<String, String> {
    let n = v.trim();
    if n.is_empty() {
        return Err("Type your name for the other players.".into());
    }
    if n.chars().count() > 24
        || !n
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_'))
    {
        return Err("Your name may have letters, digits, spaces, - and _, at most 24.".into());
    }
    Ok(n.to_string())
}

/// A join code as pasted: spaces and line ends taken out, `NVRS-` first.
pub fn valid_code(v: &str) -> Result<String, String> {
    let c: String = v.chars().filter(|c| !c.is_whitespace()).collect();
    if c.is_empty() {
        return Err("Paste the host's join code (NVRS-...) in the Join code box.".into());
    }
    if !c.to_ascii_uppercase().starts_with("NVRS-")
        || !c.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
    {
        return Err(format!(
            "\"{c}\" is not a join code. A join code starts with NVRS-."
        ));
    }
    Ok(c)
}

/// The Steam library next to `Play.exe` (`root` is the package's folder), if it is there.
pub fn steam_dll(root: &Path) -> Option<PathBuf> {
    let p = root.join(STEAM_DLL);
    p.is_file().then_some(p)
}

/// The join codes a host's game log holds: the one for this network (or the address typed), and
/// the Steam lobby's.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Codes {
    pub direct: Option<String>,
    pub steam: Option<String>,
}

fn code_after(line: &str, label: &str) -> Option<String> {
    let at = line.find(label)? + label.len();
    let rest = line[at..].trim_start();
    let code: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    code.starts_with("NVRS-").then_some(code)
}

/// The codes in a game log's text (the game program's `co-op:` lines).
pub fn codes_in(log: &str) -> Codes {
    let mut c = Codes::default();
    for line in log.lines() {
        if c.direct.is_none() {
            c.direct = code_after(line, "join code for this network:");
        }
        if c.steam.is_none() {
            c.steam = code_after(line, "Steam join code:");
        }
    }
    c
}

/// What the window tells the host once the game has made its codes.
pub fn codes_message(c: &Codes) -> Option<String> {
    let direct = c.direct.as_ref()?;
    let mut text = format!(
        "Your join code:\n\n{direct}\n\nIt is on the clipboard. Send it to your friends; each one chooses Join in their co-op window and pastes it.\nFriends over the internet need UDP port 27961 forwarded to this PC, or the address of a private network app typed in the window."
    );
    if let Some(s) = &c.steam {
        text.push_str(&format!(
            "\n\nSteam: friends choose \"Join a Steam friend\" in their co-op window, then you invite them with F6 in the game (or from your Steam friends list). No port to forward. The Steam code works too:\n{s}"
        ));
    }
    text.push_str("\n\nThen start a new game (or load the same save as your friends).");
    Some(text)
}

// voiceui
/// The volume list's entries: 0% to 200% in steps of 10.
pub fn volume_labels() -> Vec<String> {
    (0..=20).map(|i| format!("{}%", i * 10)).collect()
}

/// The volume list's entry for `volume` percent.
pub fn volume_index(volume: u32) -> usize {
    ((volume.min(200) + 5) / 10) as usize
}

/// The players as the window holds them: the first `shown` of `known` with the mute box and the
/// volume entry `read(i)` gives, the rest as the file had them.
pub fn players_from_window(
    known: &[PlayerVoice],
    shown: usize,
    read: impl Fn(usize) -> (bool, usize),
) -> Vec<PlayerVoice> {
    known
        .iter()
        .enumerate()
        .map(|(i, p)| {
            if i < shown {
                let (muted, at) = read(i);
                PlayerVoice {
                    name: p.name.clone(),
                    muted,
                    volume: (at.min(20) * 10) as u32,
                }
            } else {
                p.clone()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get<'a>(env: &'a [(&'static str, Option<String>)], name: &str) -> Option<&'a str> {
        env.iter()
            .find(|(n, _)| *n == name)
            .and_then(|(_, v)| v.as_deref())
    }

    #[test]
    fn the_choices_read_back_from_their_file() {
        let c = Choice {
            name: "Ann B".into(),
            address: "100.64.0.7".into(),
            code: "NVRS-ABC".into(),
            steam: false,
            voice: Voice::Open,
            ptt_key: "CAPSLOCK".into(),
            items: true,
            vats: false,
            harm: true,
            // voiceui
            players_key: "F10".into(),
            mute_key: "F4".into(),
            deafen_key: "F3".into(),
            players: vec![
                PlayerVoice {
                    name: "Bo Ek".into(),
                    muted: true,
                    volume: 100,
                },
                PlayerVoice {
                    name: "Cy".into(),
                    muted: false,
                    volume: 150,
                },
            ],
        };
        assert_eq!(Choice::parse(&c.to_ini()), c);
        assert_eq!(Choice::parse(""), Choice::default());
        // A line end in a box does not make a second line.
        let odd = Choice {
            name: "a\r\nVoice=off".into(),
            ..Choice::default()
        };
        assert_eq!(Choice::parse(&odd.to_ini()).voice, Voice::PushToTalk);
    }

    #[test]
    fn alone_clears_every_co_op_switch() {
        let env = Choice::default().environment(Mode::Alone, None).unwrap();
        assert_eq!(env.len(), SWITCHES.len());
        assert!(env.iter().all(|(_, v)| v.is_none()));
    }

    #[test]
    fn a_host_gets_its_name_address_steam_and_voice() {
        let c = Choice {
            name: " Ann ".into(),
            address: "203.0.113.7".into(),
            items: true,
            ..Choice::default()
        };
        let dll = PathBuf::from("pkg").join(STEAM_DLL);
        let env = c.environment(Mode::Host, Some(&dll)).unwrap();
        assert_eq!(get(&env, "NV_COOP"), Some("host"));
        assert_eq!(get(&env, "NV_COOP_NAME"), Some("Ann"));
        assert_eq!(get(&env, "NV_COOP_ADDRESS"), Some("203.0.113.7"));
        assert_eq!(get(&env, "NV_COOP_STEAM"), Some("on"));
        assert_eq!(
            get(&env, "NV_COOP_STEAM_DLL"),
            Some(dll.display().to_string().as_str())
        );
        assert_eq!(get(&env, "NV_COOP_VOICE"), Some("ptt"));
        assert_eq!(get(&env, "NV_COOP_PTT_KEY"), Some("BACKSLASH"));
        assert_eq!(get(&env, "NV_COOP_ITEMS"), Some("on"));
        assert_eq!(get(&env, "NV_COOP_VATS"), None);
        assert_eq!(get(&env, "NV_COOP_HARM"), None);
        // No Steam library: the host hosts over UDP and nothing asks for Steam.
        let env = c.environment(Mode::Host, None).unwrap();
        assert_eq!(get(&env, "NV_COOP_STEAM"), None);
        assert_eq!(get(&env, "NV_COOP_STEAM_DLL"), None);
        // An empty address is the home network's.
        let home = Choice::default().environment(Mode::Host, None).unwrap();
        assert_eq!(get(&home, "NV_COOP_ADDRESS"), None);
    }

    #[test]
    fn inviting_a_steam_friend_needs_the_library() {
        let c = Choice::default();
        let dll = PathBuf::from(STEAM_DLL);
        let env = c.environment(Mode::InviteSteamFriend, Some(&dll)).unwrap();
        assert_eq!(get(&env, "NV_COOP"), Some("host"));
        assert_eq!(get(&env, "NV_COOP_STEAM"), Some("invite"));
        let e = c.environment(Mode::InviteSteamFriend, None).unwrap_err();
        assert!(e.contains(STEAM_DLL) && e.contains("Play.exe"), "{e}");
        let e = c.environment(Mode::JoinSteamFriend, None).unwrap_err();
        assert!(e.contains(STEAM_DLL), "{e}");
        let env = c.environment(Mode::JoinSteamFriend, Some(&dll)).unwrap();
        assert_eq!(get(&env, "NV_COOP"), Some("steam"));
        assert_eq!(get(&env, "NV_COOP_STEAM"), None);
    }

    #[test]
    fn a_guest_joins_with_a_cleaned_code_and_a_bad_one_is_refused() {
        let c = Choice {
            code: " NVRS-12AB-\r\n CD34 ".into(),
            voice: Voice::Off,
            ..Choice::default()
        };
        let env = c.environment(Mode::Join, None).unwrap();
        assert_eq!(get(&env, "NV_COOP"), Some("join:NVRS-12AB-CD34"));
        assert_eq!(get(&env, "NV_COOP_VOICE"), Some("off"));
        assert_eq!(get(&env, "NV_COOP_PTT_KEY"), None);
        for bad in ["", "hello", "NVRS-AB;CD"] {
            let c = Choice {
                code: bad.into(),
                ..Choice::default()
            };
            assert!(c.environment(Mode::Join, None).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn names_and_addresses_are_checked() {
        assert_eq!(valid_name(" Ann-2_b "), Ok("Ann-2_b".into()));
        assert!(valid_name("").is_err());
        assert!(valid_name("a;b").is_err());
        assert!(valid_name(&"x".repeat(25)).is_err());
        let c = Choice {
            address: "my-pc".into(),
            ..Choice::default()
        };
        let e = c.environment(Mode::Host, None).unwrap_err();
        assert!(e.contains("not an address"), "{e}");
    }

    #[test]
    fn the_codes_are_read_from_the_game_log() {
        let log = "co-op voice: push to talk\n\
            co-op: hosting on UDP port 27961; join code for this network: NVRS-1A2B-3C4D (over the internet: forward UDP port 27961)\n\
            co-op steam: hosting a friends-only Steam lobby 109775241; Steam join code: NVRS-9Z8Y (friends join)\n";
        let c = codes_in(log);
        assert_eq!(c.direct.as_deref(), Some("NVRS-1A2B-3C4D"));
        assert_eq!(c.steam.as_deref(), Some("NVRS-9Z8Y"));
        let text = codes_message(&c).unwrap();
        assert!(text.contains("NVRS-1A2B-3C4D") && text.contains("NVRS-9Z8Y"));
        assert_eq!(codes_in("co-op: waiting\n"), Codes::default());
        assert!(codes_message(&Codes::default()).is_none());
        // A code without Steam says nothing of Steam.
        let only = codes_in("join code for this network: NVRS-AA\n");
        assert!(!codes_message(&only).unwrap().contains("Steam"));
    }

    #[test]
    fn the_key_list_and_the_switches_are_distinct() {
        let mut k = PTT_KEYS.to_vec();
        k.sort();
        k.dedup();
        assert_eq!(k.len(), PTT_KEYS.len());
        let mut s = SWITCHES.to_vec();
        s.sort();
        s.dedup();
        assert_eq!(s.len(), SWITCHES.len());
    }

    // voiceui
    #[test]
    fn the_players_voices_and_list_keys_go_to_the_file_and_the_game() {
        // The game program's own lines read back; a default player gets no line.
        let text = "Name=Ann\r\nPlayer=Bo;1;100\r\nPlayer=cy;0;250\r\nPlayer=Dee;0;100\r\nPlayer=bad\r\nPlayer=bo;0;40\r\n";
        let c = Choice::parse(text);
        assert_eq!(
            c.players,
            vec![
                PlayerVoice {
                    name: "cy".into(),
                    muted: false,
                    volume: 200
                },
                PlayerVoice {
                    name: "Dee".into(),
                    muted: false,
                    volume: 100
                },
                PlayerVoice {
                    name: "bo".into(),
                    muted: false,
                    volume: 40
                },
            ],
            "the last line of a name wins, a volume is held within 0 and 200"
        );
        let ini = c.to_ini();
        assert!(ini.contains("Player=cy;0;200\r\n") && ini.contains("Player=bo;0;40\r\n"));
        assert!(!ini.contains("Player=Dee"));
        assert!(ini.contains("PlayersKey=F11\r\nMuteKey=F8\r\nDeafenKey=F7\r\n"));
        // The keys reach the game; without voice only the players list's.
        let env = c.environment(Mode::Host, None).unwrap();
        assert_eq!(get(&env, "NV_COOP_PLAYERS_KEY"), Some("F11"));
        assert_eq!(get(&env, "NV_COOP_MUTE_KEY"), Some("F8"));
        assert_eq!(get(&env, "NV_COOP_DEAFEN_KEY"), Some("F7"));
        let off = Choice {
            voice: Voice::Off,
            ..c.clone()
        };
        let env = off.environment(Mode::Host, None).unwrap();
        assert_eq!(get(&env, "NV_COOP_PLAYERS_KEY"), Some("F11"));
        assert_eq!(get(&env, "NV_COOP_MUTE_KEY"), None);
        let alone = c.environment(Mode::Alone, None).unwrap();
        assert_eq!(get(&alone, "NV_COOP_PLAYERS_KEY"), None);
    }

    #[test]
    fn the_window_s_player_rows_set_mute_and_volume() {
        let labels = volume_labels();
        assert_eq!((labels[0].as_str(), labels[20].as_str()), ("0%", "200%"));
        assert_eq!(volume_index(150), 15);
        assert_eq!(volume_index(999), 20);
        assert_eq!(labels[volume_index(70)], "70%");
        let known = vec![
            PlayerVoice {
                name: "Bo".into(),
                muted: false,
                volume: 100,
            },
            PlayerVoice {
                name: "Cy".into(),
                muted: true,
                volume: 30,
            },
        ];
        let out = players_from_window(&known, 1, |_| (true, 12));
        assert_eq!(
            out[0],
            PlayerVoice {
                name: "Bo".into(),
                muted: true,
                volume: 120
            }
        );
        assert_eq!(out[1], known[1], "a player past the rows keeps the file's");
        let mut keys = LIST_KEYS.to_vec();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), LIST_KEYS.len());
        assert!(
            LIST_KEYS.contains(&"F11") && LIST_KEYS.contains(&"F8") && LIST_KEYS.contains(&"F7")
        );
    }
}
