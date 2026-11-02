//! The game's music player, "FalloutAudioMedia" (`0082f830`): two decks,
//! each streamed by its own thread in the game (`00830b30`). Every request
//! goes into the deck that didn't take the last one; the other fades out
//! over the same time the new one fades in. Here the decks are a model of
//! what the game's threads do, ticked with the game's clock; a player
//! (the viewer) follows them.
//!
//! Times are milliseconds of the audio clock, as the game's are.

/// The kinds of music a deck plays. They are also priorities: a request
/// lower than the most recent deck's kind is refused unless forced.
pub mod kind {
    pub const EMPTY: u8 = 0;
    pub const LOCATION: u8 = 2;
    pub const DUNGEON: u8 = 3;
    pub const BATTLE: u8 = 4;
    /// Stopped by the manager but requested by nothing.
    pub const UNUSED: u8 = 5;
    /// `PlayMusic`.
    pub const SCRIPT: u8 = 6;
    pub const RADIO: u8 = 7;
    /// Title, loading and credits music.
    pub const MENU: u8 = 8;
    /// A request of this kind clears both decks.
    pub const CLEAR: u8 = 9;

    pub fn name(kind: u8) -> &'static str {
        match kind {
            EMPTY => "empty",
            LOCATION => "location",
            DUNGEON => "dungeon",
            BATTLE => "battle",
            SCRIPT => "script",
            RADIO => "radio",
            MENU => "menu",
            _ => "other",
        }
    }
}

/// A deck's flags (the game's byte at `011dd310` / `011dd311`).
pub mod flags {
    /// The deck's thread should end.
    pub const QUIT: u8 = 0x01;
    /// Stopped (`IMediaControl::Stop`); cleared by every set's request.
    pub const STOPPED: u8 = 0x02;
    /// Paused (battle sets hold the decks for `DNAM` ms after the intro).
    pub const HELD: u8 = 0x04;
    /// Fading out, to end.
    pub const STOPPING: u8 = 0x08;
    /// A fade runs (in or out) until its end time.
    pub const FADING: u8 = 0x10;
    /// At the end of the file, start again.
    pub const LOOP: u8 = 0x20;
}

/// The volume settings the decks play at (`[Audio]` in the game's INI:
/// `fDefaultMasterVolume`, `fDefaultMusicVolume`, `fDefaultRadioVolume`,
/// `fMainMenuMusicVolume`; 1, 0.6, 0.5 and 0.6 in this install's
/// `Fallout_default.ini`, the defaults here).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Volumes {
    pub master: f32,
    pub music: f32,
    pub radio: f32,
    pub menu: f32,
}

impl Default for Volumes {
    fn default() -> Self {
        Volumes {
            master: 1.0,
            music: 0.6,
            radio: 0.5,
            menu: 0.6,
        }
    }
}

/// A deck's loudness for a kind and decibels (`0082fec0`): millibels
/// `dB × 100 + 2000 × log10(V)` clamped to −10000..0, then `10^(mB /
/// 2000) × master` clamped to 1e-4..100; V is the radio volume for radio,
/// the main menu's for menus, the music volume otherwise. So master × V ×
/// 10^(dB / 20), linear amplitude.
pub fn gain(kind: u8, decibels: f32, volumes: &Volumes) -> f32 {
    let v = match kind {
        kind::MENU => volumes.menu,
        kind::RADIO => volumes.radio,
        _ => volumes.music,
    };
    let mut mb = f64::from(decibels) * 100.0 + 2000.0 * f64::from(v).log10();
    if mb.is_nan() || mb < -10000.0 {
        mb = -10000.0;
    } else if mb > 0.0 {
        mb = 0.0;
    }
    let g = 10f64.powf(mb / 2000.0) * f64::from(volumes.master);
    g.clamp(1e-4, 100.0) as f32
}

/// How far a synced deck may stray before it's moved (`00647b70`:
/// `max(500, iMusicSynchOverride + 500)` ms; the setting is 0).
pub const SYNC_TOLERANCE_MS: f64 = 500.0;

/// Volume steps per tick outside a fade (`0103c7c8`).
const STEP: f32 = 0.05;

/// A fading-out deck stops below this (`01016408`).
const SILENT: f32 = 0.01;

/// One deck.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Deck {
    pub kind: u8,
    /// Relative to `Data` (`music\...mp3`); `None` when empty.
    pub path: Option<String>,
    /// A new number for every request (0: empty, no thread).
    pub generation: u64,
    pub decibels: f32,
    /// The volume it plays at now (linear amplitude, settings included).
    pub volume: f32,
    /// Where a fade started from.
    pub fade_from: f32,
    pub flags: u8,
    pub fade_ms: u32,
    pub fade_end: u64,
    /// The time the track it follows started (`syncStart`), 0 for none:
    /// the deck keeps itself at `(now − sync) mod length`.
    pub sync: u64,
    /// Where it plays, ms (the game reads DirectShow's position).
    pub position_ms: f64,
    /// The file's length, ms (0 until known).
    pub duration_ms: u32,
    /// Counts the times the deck jumped (moved to keep in step), so a
    /// player knows to follow. Going round a looping track isn't a jump.
    pub seeks: u32,
    last_tick: u64,
}

impl Deck {
    pub fn is_live(&self) -> bool {
        self.generation != 0
    }

    /// Whether it plays on (neither stopped nor held).
    pub fn running(&self) -> bool {
        self.flags & (flags::STOPPED | flags::HELD) == 0
    }

    pub fn looping(&self) -> bool {
        self.flags & flags::LOOP != 0
    }

    pub fn stopping(&self) -> bool {
        self.flags & flags::STOPPING != 0
    }
}

/// What became of a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Requested {
    /// Into this deck (0 A, 1 B).
    Opened(usize),
    /// Nothing to play (an empty path), or kind 9 (both cleared).
    Nothing,
    /// "Trying to open media out of priority".
    OutOfPriority,
    /// "File does not exist".
    Missing,
}

/// The two decks.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Decks {
    pub decks: [Deck; 2],
    /// Which deck the next request goes into (`011dd36c`).
    pub next: usize,
    generations: u64,
    pub volumes: Volumes,
}

impl Decks {
    pub fn new(volumes: Volumes) -> Decks {
        Decks {
            volumes,
            ..Decks::default()
        }
    }

    /// The deck that took the most recent request.
    pub fn most_recent(&self) -> &Deck {
        &self.decks[1 - self.next]
    }

    /// A request (`008300c0`): `kind`, a file (`None` does nothing), the
    /// fade in ms, whether it loops, whether it overrides priority, its
    /// decibels, and the time to keep in step with (0 none). `duration`
    /// is the file's length, `None` when it can't be opened. The new deck
    /// fades in from 0; the other fades out from where it is over the same
    /// time, whatever fade it had.
    #[allow(clippy::too_many_arguments)]
    pub fn request(
        &mut self,
        kind: u8,
        path: Option<&str>,
        duration: Option<u32>,
        fade_ms: u32,
        looping: bool,
        force: bool,
        decibels: f32,
        sync: u64,
        now: u64,
    ) -> Requested {
        if kind == kind::CLEAR {
            self.decks = [Deck::default(), Deck::default()];
            self.next = 0;
            return Requested::Nothing;
        }
        let Some(path) = path.filter(|p| !p.is_empty()) else {
            return Requested::Nothing;
        };
        if !force && kind < self.most_recent().kind {
            return Requested::OutOfPriority;
        }
        let Some(duration) = duration else {
            return Requested::Missing;
        };
        self.generations += 1;
        let new = self.next;
        let old = 1 - new;
        self.decks[new] = Deck {
            kind,
            path: Some(path.to_string()),
            generation: self.generations,
            decibels,
            volume: 0.0,
            fade_from: 0.0,
            flags: flags::FADING | if looping { flags::LOOP } else { 0 },
            fade_ms,
            fade_end: now + u64::from(fade_ms),
            sync,
            position_ms: 0.0,
            duration_ms: duration,
            seeks: 0,
            last_tick: now,
        };
        let other = &mut self.decks[old];
        other.flags |= flags::STOPPING | flags::FADING;
        other.fade_from = other.volume;
        other.fade_ms = fade_ms;
        other.fade_end = now + u64::from(fade_ms);
        self.next = old;
        Requested::Opened(new)
    }

    /// Fades out the decks of a kind (0: all) over `fade_ms`, unless
    /// they're fading out already (`00830680`).
    pub fn fade_out(&mut self, fade_ms: u32, kind: u8, now: u64) {
        for d in &mut self.decks {
            if (d.kind == kind || kind == kind::EMPTY) && !d.stopping() {
                d.flags |= flags::STOPPING | flags::FADING;
                d.fade_from = d.volume;
                d.fade_ms = fade_ms;
                d.fade_end = now + u64::from(fade_ms);
            }
        }
    }

    /// Whether a deck of a kind (0: any) plays (`00830750`): live, and not
    /// stopped (`stopped` true) or not fading out (`stopped` false).
    pub fn is_playing(&self, kind: u8, stopped: bool) -> bool {
        self.decks.iter().any(|d| {
            d.is_live()
                && (kind == kind::EMPTY || d.kind == kind)
                && if stopped {
                    d.flags & flags::STOPPED == 0
                } else {
                    !d.stopping()
                }
        })
    }

    /// The most recent deck, if it's live and of the kind (0: any): the
    /// game's position and length queries (`00830a20`, `008308c0`,
    /// `00830970`) look at that deck only.
    fn recent_of(&self, kind: u8) -> Option<&Deck> {
        let d = self.most_recent();
        (d.is_live() && (kind == kind::EMPTY || d.kind == kind)).then_some(d)
    }

    /// Where the most recent deck plays, ms, if it's of the kind; else 0.
    pub fn position(&self, kind: u8) -> u64 {
        self.recent_of(kind)
            .map_or(0, |d| d.position_ms.round().max(0.0) as u64)
    }

    /// The most recent deck's length, ms, if it's of the kind; else 0.
    pub fn duration(&self, kind: u8) -> u32 {
        self.recent_of(kind).map_or(0, |d| d.duration_ms)
    }

    /// Both decks play on (`008304c0`: clears the stopped flags).
    pub fn clear_stops(&mut self) {
        for d in &mut self.decks {
            d.flags &= !flags::STOPPED;
        }
    }

    /// Both decks wait (`008305e0`), or carry on (`00830610`).
    pub fn hold(&mut self, on: bool) {
        for d in &mut self.decks {
            if on {
                d.flags |= flags::HELD;
            } else {
                d.flags &= !flags::HELD;
            }
        }
    }

    /// What each deck's thread does on a tick (the game's wake at least
    /// every 100 ms and once a frame): play on, start again or end at the
    /// end of the file, set the volume (a linear fade while one runs, else
    /// steps of 0.05 toward the target; a fading-out deck ends below
    /// 0.01), and keep in step with the time it follows.
    ///
    /// Looping restarts with no gap here; the game's thread seeks back to
    /// the start when DirectShow says the file ended, a tick later.
    pub fn tick(&mut self, now: u64) {
        let volumes = self.volumes;
        for d in &mut self.decks {
            if !d.is_live() {
                continue;
            }
            let dt = now.saturating_sub(d.last_tick) as f64;
            d.last_tick = now;
            if d.running() {
                d.position_ms += dt;
            }
            let length = f64::from(d.duration_ms);
            if length > 0.0 && d.position_ms >= length {
                if d.looping() {
                    d.position_ms %= length;
                } else {
                    *d = Deck::default();
                    continue;
                }
            }
            let target = if d.stopping() {
                0.0
            } else {
                gain(d.kind, d.decibels, &volumes)
            };
            if d.flags & flags::FADING == 0 || d.fade_end <= now {
                if d.volume < target {
                    d.volume = (d.volume + STEP).min(target);
                } else if d.volume > target {
                    d.volume = (d.volume - STEP).max(target);
                }
            } else {
                let t = if d.fade_ms == 0 {
                    0.0
                } else {
                    ((d.fade_end - now) as f64 / f64::from(d.fade_ms)).clamp(0.0, 1.0) as f32
                };
                d.volume = if d.stopping() {
                    t * d.fade_from
                } else {
                    (target - d.fade_from) * (1.0 - t) + d.fade_from
                };
            }
            if d.stopping() && d.volume < SILENT {
                *d = Deck::default();
                continue;
            }
            if d.sync != 0 {
                let mut correct = now.saturating_sub(d.sync) as f64;
                if length > 0.0 {
                    while correct > length {
                        correct -= length;
                    }
                }
                let off = (correct - d.position_ms).abs();
                if off > SYNC_TOLERANCE_MS && (off < 80_000.0 || d.position_ms == 0.0) {
                    d.position_ms = correct;
                    d.seeks += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loudness_is_master_times_music_times_decibels() {
        let v = Volumes::default();
        // Doc Mitchell's house: −4 dB at music volume 0.6.
        let g = gain(kind::LOCATION, -4.0, &v);
        assert!((g - 0.6 * 10f32.powf(-4.0 / 20.0)).abs() < 1e-5, "{g}");
        assert!((g - 0.3786).abs() < 1e-3);
        // Louder than full is held at full; silence at the floor.
        assert_eq!(
            gain(kind::LOCATION, 20.0, &Volumes { music: 1.0, ..v }),
            1.0
        );
        assert_eq!(
            gain(kind::LOCATION, -4.0, &Volumes { music: 0.0, ..v }),
            1e-4
        );
        // Menus use their own volume, the radio its own.
        assert!((gain(kind::RADIO, 0.0, &v) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn a_new_track_fades_in_as_the_old_fades_out() {
        let mut d = Decks::new(Volumes {
            master: 1.0,
            music: 1.0,
            ..Volumes::default()
        });
        let r = d.request(
            kind::LOCATION,
            Some("music\\a.mp3"),
            Some(60_000),
            1000,
            true,
            true,
            0.0,
            0,
            1000,
        );
        assert_eq!(r, Requested::Opened(0));
        d.tick(1500);
        assert!((d.decks[0].volume - 0.5).abs() < 1e-6);
        d.tick(1999);
        assert!((d.decks[0].volume - 0.999).abs() < 1e-6);
        // Past the fade's end it steps the rest of the way (0.05 a tick).
        d.tick(2000);
        assert_eq!(d.decks[0].volume, 1.0);
        // Into the other deck; the first fades out over the same 2 s.
        let r = d.request(
            kind::LOCATION,
            Some("music\\b.mp3"),
            Some(60_000),
            2000,
            true,
            true,
            0.0,
            0,
            3000,
        );
        assert_eq!(r, Requested::Opened(1));
        d.tick(4000);
        assert!((d.decks[0].volume - 0.5).abs() < 1e-6);
        assert!((d.decks[1].volume - 0.5).abs() < 1e-6);
        assert_eq!(d.position(kind::LOCATION), 1000);
        // Below 0.01 it's gone.
        d.tick(4999);
        assert!(!d.decks[0].is_live());
        d.tick(5000);
        assert_eq!(d.decks[1].volume, 1.0);
        // A lower kind than the most recent one is refused unless forced.
        let r = d.request(
            kind::SCRIPT,
            Some("music\\s.mp3"),
            Some(5000),
            1000,
            false,
            false,
            0.0,
            0,
            5000,
        );
        assert_eq!(r, Requested::Opened(0));
        let r = d.request(
            kind::LOCATION,
            Some("music\\a.mp3"),
            Some(60_000),
            1000,
            true,
            false,
            0.0,
            0,
            5100,
        );
        assert_eq!(r, Requested::OutOfPriority);
        // A file that can't be opened changes nothing.
        let before = d.clone();
        let r = d.request(
            kind::BATTLE,
            Some("music\\gone.mp3"),
            None,
            1000,
            true,
            true,
            0.0,
            0,
            5100,
        );
        assert_eq!((r, &d), (Requested::Missing, &before));
    }

    #[test]
    fn a_synced_deck_starts_where_the_old_one_was_and_tracks_end() {
        let mut d = Decks::new(Volumes::default());
        // The new deck follows a track that began at 10 s; now 100 s.
        d.request(
            kind::LOCATION,
            Some("music\\b.mp3"),
            Some(60_000),
            9000,
            false,
            true,
            -4.0,
            10_000,
            100_000,
        );
        d.tick(100_000);
        // 90 s into a 60 s track: 30 s.
        assert_eq!(d.decks[0].position_ms, 30_000.0);
        assert_eq!(d.decks[0].seeks, 1);
        // Plays on, and ends with the file (it doesn't loop).
        d.tick(120_000);
        assert_eq!(d.decks[0].position_ms, 50_000.0);
        d.tick(130_000);
        assert!(!d.decks[0].is_live());
    }
}
