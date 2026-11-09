//! Host-side chat: names, sanitizing, rate limit, relay.

use std::collections::{BTreeMap, VecDeque};

use super::{ChatChannel, ChatReject, SocialMsg, MAX_CHAT_BYTES};
use crate::ids::PlayerId;
use crate::interest::Audience;

/// Lines allowed per window.
pub const RATE_LINES: usize = 5;
/// Rate window.
pub const RATE_WINDOW_MS: u64 = 10_000;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatOut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: SocialMsg,
}

/// Makes text safe to display and to embed in format strings: control
/// characters are dropped, `% & < > \` become `?`, whitespace collapses at the
/// ends, and the result is cut to [`MAX_CHAT_BYTES`] on a character boundary.
pub fn sanitize(text: &str) -> String {
    let mut s: String = text
        .chars()
        .filter(|c| !c.is_control())
        .map(|c| {
            if matches!(c, '%' | '&' | '<' | '>' | '\\') {
                '?'
            } else {
                c
            }
        })
        .collect();
    s = s.trim().to_string();
    if s.len() > MAX_CHAT_BYTES {
        let mut cut = MAX_CHAT_BYTES;
        while !s.is_char_boundary(cut) {
            cut -= 1;
        }
        s.truncate(cut);
        s = s.trim_end().to_string();
    }
    s
}

/// Chat state on the host.
#[derive(Clone, Debug, Default)]
pub struct ChatHost {
    names: BTreeMap<PlayerId, String>,
    recent: BTreeMap<PlayerId, VecDeque<u64>>,
    last_id: BTreeMap<PlayerId, u32>,
}

impl ChatHost {
    /// Empty chat.
    pub fn new() -> Self {
        Self::default()
    }

    /// Assigns a unique display name (suffix " 2", " 3" ... on a clash).
    pub fn set_name(&mut self, p: PlayerId, wanted: &str) -> String {
        let base = {
            let b = sanitize(wanted);
            let b: String = b.chars().take(24).collect();
            if b.is_empty() {
                format!("Player {}", p.0)
            } else {
                b
            }
        };
        let mut name = base.clone();
        let mut n = 2;
        while self
            .names
            .iter()
            .any(|(q, v)| *q != p && v.eq_ignore_ascii_case(&name))
        {
            name = format!("{base} {n}");
            n += 1;
        }
        self.names.insert(p, name.clone());
        name
    }

    /// Display name of a player.
    pub fn name(&self, p: PlayerId) -> Option<&str> {
        self.names.get(&p).map(|s| s.as_str())
    }

    /// Forgets a player.
    pub fn remove(&mut self, p: PlayerId) {
        self.names.remove(&p);
        self.recent.remove(&p);
        self.last_id.remove(&p);
    }

    /// A system line for everyone.
    pub fn system(&self, text: &str) -> ChatOut {
        ChatOut {
            aud: Audience::All,
            msg: SocialMsg::Line {
                from: None,
                name: String::new(),
                channel: ChatChannel::System,
                text: sanitize(text),
            },
        }
    }

    /// Handles a line from `from`.
    pub fn on_say(
        &mut self,
        from: PlayerId,
        msg_id: u32,
        channel: ChatChannel,
        text: &str,
        now_ms: u64,
    ) -> Vec<ChatOut> {
        let reject = |reason| {
            vec![ChatOut {
                aud: Audience::To(from),
                msg: SocialMsg::Rejected { msg_id, reason },
            }]
        };
        if self.last_id.get(&from).map_or(false, |l| msg_id <= *l) {
            return vec![]; // duplicate delivery
        }
        let q = self.recent.entry(from).or_default();
        while q
            .front()
            .map_or(false, |t| now_ms.saturating_sub(*t) >= RATE_WINDOW_MS)
        {
            q.pop_front();
        }
        if q.len() >= RATE_LINES {
            return reject(ChatReject::RateLimited);
        }
        let text = sanitize(text);
        if text.is_empty() || channel == ChatChannel::System {
            return reject(ChatReject::Invalid);
        }
        q.push_back(now_ms);
        self.last_id.insert(from, msg_id);
        let name = self.names.get(&from).cloned().unwrap_or_default();
        let line = SocialMsg::Line {
            from: Some(from),
            name,
            channel,
            text,
        };
        match channel {
            ChatChannel::Whisper(to) if to != from => vec![ChatOut {
                aud: Audience::To(to),
                msg: line,
            }],
            ChatChannel::Whisper(_) => reject(ChatReject::Invalid),
            _ => vec![ChatOut {
                aud: Audience::AllExcept(from),
                msg: line,
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const A: PlayerId = PlayerId(1);
    const B: PlayerId = PlayerId(2);

    proptest! {
        #[test]
        fn sanitize_is_safe_and_idempotent(s in "\\PC{0,300}") {
            let t = sanitize(&s);
            prop_assert!(t.len() <= MAX_CHAT_BYTES);
            prop_assert!(!t.chars().any(|c| c.is_control() || "%&<>\\".contains(c)));
            prop_assert_eq!(sanitize(&t), t);
        }
    }

    #[test]
    fn sanitize_examples() {
        assert_eq!(sanitize("  hi %s <b>\u{7}there\\ "), "hi ?s ?b?there?");
        let long = "é".repeat(200);
        let t = sanitize(&long);
        assert!(t.len() <= MAX_CHAT_BYTES && t.chars().all(|c| c == 'é'));
        assert_eq!(sanitize("\u{1}\u{2}"), "");
    }

    #[test]
    fn names_are_unique() {
        let mut c = ChatHost::new();
        assert_eq!(c.set_name(A, "Courier"), "Courier");
        assert_eq!(c.set_name(B, "courier"), "courier 2");
        assert_eq!(
            c.set_name(B, "courier"),
            "courier 2",
            "same player keeps its name"
        );
        assert_eq!(c.set_name(PlayerId(3), "   "), "Player 3");
        c.remove(A);
        assert_eq!(c.set_name(PlayerId(4), "Courier"), "Courier");
    }

    #[test]
    fn say_relays_to_everyone_else_with_name() {
        let mut c = ChatHost::new();
        c.set_name(A, "Ann");
        let o = c.on_say(A, 1, ChatChannel::All, "hello <world>", 0);
        assert_eq!(o[0].aud, Audience::AllExcept(A));
        assert!(
            matches!(&o[0].msg, SocialMsg::Line { name, text, .. } if name == "Ann" && text == "hello ?world?")
        );
    }

    #[test]
    fn five_lines_per_ten_seconds() {
        let mut c = ChatHost::new();
        for i in 1..=5 {
            assert!(matches!(
                c.on_say(A, i, ChatChannel::All, "x", i as u64)[0].msg,
                SocialMsg::Line { .. }
            ));
        }
        let o = c.on_say(A, 6, ChatChannel::All, "x", 100);
        assert!(matches!(
            o[0].msg,
            SocialMsg::Rejected {
                reason: ChatReject::RateLimited,
                msg_id: 6
            }
        ));
        assert!(matches!(
            c.on_say(A, 7, ChatChannel::All, "x", RATE_WINDOW_MS + 1)[0].msg,
            SocialMsg::Line { .. }
        ));
        assert!(
            matches!(
                c.on_say(B, 1, ChatChannel::All, "x", 100)[0].msg,
                SocialMsg::Line { .. }
            ),
            "per sender"
        );
    }

    #[test]
    fn duplicate_ids_are_dropped_silently() {
        let mut c = ChatHost::new();
        assert_eq!(c.on_say(A, 4, ChatChannel::All, "x", 0).len(), 1);
        assert!(c.on_say(A, 4, ChatChannel::All, "x", 1).is_empty());
        assert!(c.on_say(A, 3, ChatChannel::All, "x", 2).is_empty());
    }

    #[test]
    fn whisper_system_and_empty() {
        let mut c = ChatHost::new();
        let o = c.on_say(A, 1, ChatChannel::Whisper(B), "psst", 0);
        assert_eq!(o[0].aud, Audience::To(B));
        assert!(matches!(
            c.on_say(A, 2, ChatChannel::Whisper(A), "me", 0)[0].msg,
            SocialMsg::Rejected { .. }
        ));
        assert!(matches!(
            c.on_say(A, 3, ChatChannel::System, "fake", 0)[0].msg,
            SocialMsg::Rejected {
                reason: ChatReject::Invalid,
                ..
            }
        ));
        assert!(matches!(
            c.on_say(A, 4, ChatChannel::All, "  \u{1} ", 0)[0].msg,
            SocialMsg::Rejected { .. }
        ));
        assert_eq!(c.system("* host saved").aud, Audience::All);
    }
}
