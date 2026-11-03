//! Subtitles: what someone is saying, drawn at the bottom centre of the
//! screen by the HUD (`ui::hud`'s "Subtitles" tile, `HudInput::subtitle`),
//! with the speaker's name. Fed by the lines people say without the dialogue
//! menu (`chatter`) and, in co-op, by the lines another player picked in a
//! shared conversation (`coop_live::intents::play_lines`), so a guest reads
//! what the host reads. Not the corner messages (`hud::HudMessages`), which
//! stay for real status text.

use bevy::prelude::*;
use esm::FormId;

/// The line on screen and when (seconds) it goes.
#[derive(Resource, Default)]
pub struct Subtitles {
    current: Option<(String, f32)>,
}

/// How long a line stays: its reading time, at least `min` seconds.
pub fn seconds(text: &str, min: f32) -> f32 {
    crate::chatter::reading_time(text).max(min)
}

/// The text shown for a line: the speaker's name, then the words.
pub fn label(who: &str, text: &str) -> String {
    if who.is_empty() {
        text.to_string()
    } else {
        format!("{who}: {text}")
    }
}

impl Subtitles {
    /// Shows a line from `now` for `secs`, replacing the one on screen.
    pub fn show(&mut self, who: &str, text: &str, now: f32, secs: f32) {
        if text.trim().is_empty() {
            return;
        }
        self.current = Some((label(who, text), now + secs));
    }

    /// The line to draw at `now`, if it is still up.
    pub fn current(&self, now: f32) -> Option<&str> {
        self.current
            .as_ref()
            .filter(|(_, until)| *until > now)
            .map(|(s, _)| s.as_str())
    }
}

/// A speaker's name: the person's `Speaker` name, else their base's `FULL`.
pub fn speaker_name(order: &esm::LoadOrder, reference: FormId) -> String {
    let Some(base) = world::scripting::base_of(order, reference) else {
        return String::new();
    };
    world::dialogue::Speaker::load(order, reference, base)
        .and_then(|s| s.name)
        .filter(|n| !n.is_empty())
        .or_else(|| {
            order
                .get(base)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.full_name())
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_has_speaker_and_expires() {
        let mut s = Subtitles::default();
        assert_eq!(s.current(0.0), None);
        s.show("Doc Mitchell", "Easy now.", 10.0, 3.0);
        assert_eq!(s.current(11.0), Some("Doc Mitchell: Easy now."));
        assert_eq!(s.current(13.5), None);
    }

    #[test]
    fn newer_line_replaces_and_blank_is_ignored() {
        let mut s = Subtitles::default();
        s.show("A", "one", 0.0, 5.0);
        s.show("", "two", 1.0, 5.0);
        assert_eq!(s.current(2.0), Some("two"));
        s.show("B", "  ", 2.0, 5.0);
        assert_eq!(s.current(3.0), Some("two"));
    }

    #[test]
    fn seconds_has_a_floor() {
        assert!(seconds("Hi.", 2.5) >= 2.5);
        assert!(seconds(&"word ".repeat(40), 2.5) > 10.0);
    }
}
