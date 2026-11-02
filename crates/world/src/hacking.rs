//! The terminal hacking game (`HackingMenu`): guessing a password from a
//! screen of words hidden in garbage. All read from `FalloutNV.exe`
//! 1.4.0.525; the names in brackets are the Xbox 360 prototype's
//! (Xbox PDB), whose `HackingMenu` has the same fields at the same offsets.
//!
//! **Opening** (`00765b80`, `HackingMenu::Create`): the word length is the
//! terminal's difficulty × 2 + 4, plus 1 for a base terminal whose form ID
//! is odd, at most 12 ([`word_length`], `00501270`). The number of words
//! ([`word_count`]) runs from `iHackingMinWords` (5) for a player with all
//! the Science above the difficulty's minimum (`fHackingMinSkill…`, 0 25 50
//! 75 100 for very easy … very hard) to `iHackingMaxWords` (20, at most 20)
//! for one with just the minimum. Science is the player's current value
//! clamped to 0..100 (`0066ef50`) after their perks' "Hacking Science
//! Bonus" (entry point 30; no perk in the game's data has one).
//!
//! **The words** (`00768070`, `Prepare`): every word of that length in
//! `Data\Menus\FalloutDict.txt` (read up to each space), shuffled three
//! times; then tries, each on a fresh shuffle, taking words in order while
//! each new word, against every word already taken, is not the same word,
//! and, before the last word, shares at least one letter in place with it
//! (a word sharing none passes only on a roll of 10 under an allowance
//! that starts at 0); the last word may share each number of letters with
//! at most (count − 2) / length + 1 of the others. Every 100 ms (the clock)
//! without a full set the allowance rises by one; after 10 rises it goes
//! back to 0 and the last word's limit rises, until it passes half the
//! count, when the game carries on with the words it has. The words are
//! shuffled four more times and one is the password. "Shuffling" is the
//! game's list sort (a Shell sort, `007653f0`) with a comparator that
//! answers at random (`0076b2e0`); every random number is the game's
//! generator (`00aa5230`, [`Twister`]).
//!
//! **Attempts** (`00769520`, `Grid`/`GridRow::GetMaxTally`): from the
//! words' likeness table, a count of guesses that always narrows the words
//! down ([`attempts`]); at least 4.
//!
//! **The screen** (`00768aa0`, `MakePasswordFile`): 408 characters (two
//! columns of 17 lines of 12) of garbage from ``!@#$%^*()_+=-`[]{}|;':,./<>?\"``,
//! the words dropped in at random places with two non-letters between any
//! two (`0x197 − length` places tried, 100 times per word, the whole screen
//! drawn again up to 5 times, then the words that didn't fit are dropped).
//! Each line shows a 16-bit address, "0x" and four hex digits, the low
//! half of where the game's stack keeps the screen plus 12 a line (not
//! reproducible: [`PasswordFile::address`]).
//!
//! **What the pointer picks** (`00769b50`): a word under it; else a
//! bracket under it opening `(`, `[`, `{` or `<` whose closing one follows
//! on the same line with no letter between (`0076a670`) and that hasn't
//! been used; else the one character.
//!
//! **A choice** (`00766b80`, `DoClick`), every log line led by ">":
//! - brackets: "Entry denied" when only the password is left; else, the
//!   first time only and on a roll of 4 under 1, the attempts are refilled
//!   ("Allowance" "replenished."); otherwise a word that isn't the password
//!   is turned to dots ("Dud removed.", `0076a730`, `RemoveDud`);
//! - the password: "Exact match!" "Please wait" "while system"
//!   "is accessed.", and the terminal opens 3 s later;
//! - anything else (a word, or a single character): "Entry denied", one
//!   attempt gone, "n/length correct." (letters in place), or at the last
//!   "Lockout in" "progress." and the terminal locks.
//!
//! The menu's timing and drawing are the interface's (`ui::menus::hacking`).

use crate::grass::Twister;

/// The dictionary's path in the game's files.
pub const DICTIONARY: &str = "Menus\\FalloutDict.txt";

/// The garbage characters (`01072ae0`).
pub const GARBAGE: &[u8; 30] = b"!@#$%^*()_+=-`[]{}|;':,./<>?\\\"";

/// Opening and closing brackets (`011a0104`, `011a0100`).
const OPENERS: &[u8; 4] = b"([{<";
const CLOSERS: &[u8; 4] = b")]}>";

/// Characters on a line, lines in a column, columns.
pub const LINE_CHARS: usize = 12;
pub const ROWS: usize = 17;
pub const COLUMNS: usize = 2;
/// The screen's characters (0x198).
pub const FILE_CHARS: usize = LINE_CHARS * ROWS * COLUMNS;

/// `iHackingMaxWords`, `iHackingMinWords`: exe defaults; the official data
/// sets neither.
pub const MAX_WORDS: (&str, i32) = ("iHackingMaxWords", 20);
pub const MIN_WORDS: (&str, i32) = ("iHackingMinWords", 5);

/// `fHackingMinSkill<difficulty>` (`[Hacking]` in the INI; the game's
/// INI files set none, so the exe's: `00f8f590` …).
pub const MIN_SKILL: [f32; 5] = [0.0, 25.0, 50.0, 75.0, 100.0];

/// "Hacking Science Bonus" (`00765b80` asks it on the player's Science).
pub const SCIENCE_BONUS_ENTRY: u8 = 30;

/// The log's texts: name and the exe's default (none is in the official
/// data). In the order the game prints them.
pub const TEXTS: &[(&str, &str)] = &[
    ("sHackingDenied", "Entry denied"),
    ("sHackingDudRemoved", "Dud removed."),
    ("sHackingToleranceReset1", "Allowance"),
    ("sHackingToleranceReset2", "replenished."),
    ("sHackingGranted", "Exact match!"),
    ("sHackingAccessing1", "Please wait"),
    ("sHackingAccessing2", "while system"),
    ("sHackingAccessing3", "is accessed."),
    ("sHackingCorrect", "correct"),
    ("sHackingLockout1", "Lockout in"),
    ("sHackingLockout2", "progress."),
];

/// The minimum Science for a difficulty (`005017a0`; anything above very
/// hard: no Science is enough).
pub fn min_skill(difficulty: u8) -> f32 {
    MIN_SKILL
        .get(usize::from(difficulty))
        .copied()
        .unwrap_or(f32::MAX)
}

/// The words' length for a base terminal and difficulty (`00501270`, then
/// at most 12 in `00765b80`).
pub fn word_length(terminal: esm::FormId, difficulty: u8) -> usize {
    (usize::from(difficulty) * 2 + (terminal.0 & 1) as usize + 4).min(12)
}

/// How many words (`00765b80`): `max` and `min` are `iHackingMaxWords`
/// (at most 20) and `iHackingMinWords` (at most `max`).
pub fn word_count(science: f32, min_skill: f32, max: i32, min: i32) -> u32 {
    let max = max.min(20);
    let min = min.min(max);
    let needed = min_skill.clamp(0.0, 100.0);
    let over = (science - needed).max(0.0);
    let span = (100.0 - f64::from(needed)) as f32;
    let share = if span == 0.0 { 0.5 } else { 1.0 - over / span };
    let extra = round_half_up((max - min) as u32 as f32 * share);
    // Unsigned in the game: below 0 wraps round to more than 20.
    let n = (i64::from(extra) + i64::from(min)) as u32;
    n.min(20)
}

/// `004bd510(x, 1)`: truncated, plus one when what was cut is at least ½.
fn round_half_up(x: f32) -> i32 {
    let t = x.trunc();
    t as i32 + i32::from(x - t >= 0.5)
}

/// Letters in the same places (`00768a10`, `CommonPositions`), over the
/// shorter word.
pub fn likeness(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).filter(|(x, y)| x == y).count()
}

/// The game's clock in milliseconds (`GetTickCount`).
pub trait Clock {
    fn now(&mut self) -> u32;
}

impl<F: FnMut() -> u32> Clock for F {
    fn now(&mut self) -> u32 {
        self()
    }
}

/// The dictionary's words of one length (`00768070`): read up to each
/// space, in the order the game's list holds them (each added at the
/// front).
pub fn dictionary_words(text: &[u8], length: usize) -> Vec<Vec<u8>> {
    let mut words: Vec<Vec<u8>> = text
        .split(|&c| c == b' ')
        .filter(|w| !w.is_empty() && w.len() == length)
        .map(<[u8]>::to_vec)
        .collect();
    words.reverse();
    words
}

/// The game's list shuffle (`007687b0`): its Shell sort (`007653f0`,
/// gaps 1, 4, 13, … below a ninth of the span) with a comparator that
/// says "before" on a roll of 2 under 1.
pub fn shuffle<T>(items: &mut [T], rng: &mut Twister) {
    let n = items.len() as i64;
    let hi = n - 1;
    let mut h: i64 = 1;
    while h <= hi / 9 {
        h = 3 * h + 1;
    }
    while h > 0 {
        for i in h..=hi {
            let mut j = i;
            while j >= h && rng.below(2) == 0 {
                items.swap(j as usize, (j - h) as usize);
                j -= h;
            }
        }
        h /= 3;
    }
}

/// Words picked from the dictionary (`00768070`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Words {
    /// In the game's list order (the order they go onto the screen).
    pub words: Vec<Vec<u8>>,
    /// Which is the password.
    pub password: usize,
}

/// Picks `count` words of `length` letters from the dictionary's
/// (`00768070`; see the module notes). `None` for a length of 0.
pub fn choose_words(
    dictionary: &[Vec<u8>],
    length: usize,
    count: u32,
    rng: &mut Twister,
    clock: &mut impl Clock,
) -> Option<Words> {
    if length == 0 {
        return None;
    }
    let mut pool = dictionary.to_vec();
    for _ in 0..3 {
        shuffle(&mut pool, rng);
    }
    let mut max_same = (count.wrapping_sub(2) / length as u32).wrapping_add(1);
    let mut allowance = 0u32;
    let mut tries = 0u32;
    // The game's list adds at the front; kept here in its order.
    let mut words: Vec<Vec<u8>> = Vec::new();
    'relax: loop {
        let deadline = clock.now().wrapping_add(100);
        loop {
            if clock.now() >= deadline {
                break;
            }
            tries = tries.wrapping_add(1);
            if tries == 0 {
                break;
            }
            words.clear();
            shuffle(&mut pool, rng);
            for candidate in &pool {
                if words.len() as u32 >= count {
                    break;
                }
                let mut ok = true;
                let mut same = [0u8; 8];
                for existing in &words {
                    if !ok {
                        break;
                    }
                    let n = likeness(candidate, existing);
                    ok = if n == length {
                        false
                    } else if (words.len() as u32) < count.wrapping_sub(1) {
                        n != 0 || rng.below(10) < allowance
                    } else if n < 8 {
                        same[n] = same[n].wrapping_add(1);
                        u32::from(same[n]) <= max_same
                    } else {
                        true
                    };
                }
                if ok {
                    words.insert(0, candidate.clone());
                }
            }
            if words.len() as u32 == count {
                break 'relax;
            }
        }
        if words.len() as u32 == count {
            break;
        }
        allowance += 1;
        if allowance == 10 {
            allowance = 0;
            max_same = max_same.wrapping_add(1);
            if max_same > count >> 1 {
                break;
            }
        }
    }
    for _ in 0..4 {
        shuffle(&mut words, rng);
    }
    let password = rng.below(words.len() as u32) as usize;
    Some(Words { words, password })
}

/// The guesses a set of words gets (`00768070`: `GetMaxTally` of their
/// likeness table, at least 4).
pub fn attempts(words: &[Vec<u8>]) -> u32 {
    let grid: Vec<Vec<u8>> = words
        .iter()
        .enumerate()
        .map(|(i, a)| {
            words
                .iter()
                .enumerate()
                .map(|(j, b)| if i == j { 0xFF } else { likeness(a, b) as u8 })
                .collect()
        })
        .collect();
    max_tally(&grid).max(4)
}

/// The most words sharing one likeness with a row's word
/// (`GridRow::GetMaxTally`, `0076ae90`).
fn row_tally(row: &[u8]) -> u8 {
    let mut counts = [0u8; 32];
    for &v in row {
        if v < 32 {
            counts[usize::from(v)] = counts[usize::from(v)].wrapping_add(1);
        }
    }
    counts.into_iter().max().unwrap_or(0)
}

/// `Grid::GetMaxTally` (`00769520`).
fn max_tally(grid: &[Vec<u8>]) -> u32 {
    let least = grid.iter().map(|r| row_tally(r)).min().unwrap_or(0xFF);
    if least == 0xFF {
        return 0;
    }
    if least <= 2 {
        return u32::from(least) + 1;
    }
    let mut result = grid.len() as u32;
    for row in grid {
        if row_tally(row) != least {
            continue;
        }
        let mut best = 0;
        for value in 0..grid.len() {
            // `0076ae30`: the columns holding this likeness.
            let picked: Vec<usize> = row
                .iter()
                .enumerate()
                .filter(|(_, &v)| usize::from(v) == value)
                .map(|(k, _)| k)
                .collect();
            if picked.len() as u8 != least {
                continue;
            }
            // `0076af90`, `Grid::MakeSubGrid`.
            let sub: Vec<Vec<u8>> = picked
                .iter()
                .map(|&a| picked.iter().map(|&b| grid[a][b]).collect())
                .collect();
            best = best.max(max_tally(&sub));
        }
        if best != 0 {
            result = result.min(best);
        }
        if result <= 2 {
            return result + 1;
        }
    }
    result + 1
}

/// The screen of garbage and words (`00768aa0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordFile {
    pub chars: Vec<u8>,
    /// Where each word starts, in the words' order; [`FILE_CHARS`] for a
    /// word that didn't fit.
    pub positions: Vec<usize>,
    /// Words dropped because they didn't fit (the game logs "MENUS: Unable
    /// to construct complete password file …"), from the end of the list.
    pub dropped: usize,
}

impl PasswordFile {
    /// Garbage with the words dropped in (`00768aa0`).
    pub fn build(words: &[Vec<u8>], rng: &mut Twister) -> PasswordFile {
        let mut chars = vec![0u8; FILE_CHARS];
        let mut positions = vec![FILE_CHARS; words.len()];
        let mut kept = words.len();
        let mut redrawn = 0;
        loop {
            for c in chars.iter_mut() {
                *c = GARBAGE[rng.below(GARBAGE.len() as u32) as usize];
            }
            let mut placed = 0;
            let mut misses = 0;
            while placed < kept && misses < 100 {
                let word = &words[placed];
                let len = word.len();
                let at = rng.below((0x197 - len) as u32) as usize;
                let clear = (-2..len as i64 + 2).all(|off| {
                    let i = at as i64 + off;
                    i < 0 || !chars[i as usize].is_ascii_uppercase()
                });
                if clear {
                    chars[at..at + len].copy_from_slice(word);
                    positions[placed] = at;
                    placed += 1;
                    misses = 0;
                } else {
                    misses += 1;
                }
            }
            if misses == 0 {
                break;
            }
            redrawn += 1;
            if redrawn > 4 {
                // The words from the one that missed on are dropped.
                for p in positions.iter_mut().skip(placed) {
                    *p = FILE_CHARS;
                }
                kept = placed;
            }
        }
        PasswordFile {
            chars,
            positions,
            dropped: words.len() - kept,
        }
    }

    /// One of the 34 lines' 12 characters (lines 0–16 the left column).
    pub fn line(&self, line: usize) -> &[u8] {
        &self.chars[line * LINE_CHARS..(line + 1) * LINE_CHARS]
    }

    /// A line's address (`00768970`): "0x" and the low 16 bits, in four
    /// upper-case hex digits, of `base` + 12 a line. The game's base is
    /// where its stack holds the screen (`00768aa0`'s buffer), which
    /// nothing here can know.
    pub fn address(base: u16, line: usize) -> String {
        format!("0x{:04X}", base.wrapping_add((line * LINE_CHARS) as u16))
    }
}

/// What the pointer picks on the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    /// A word, by its place in [`Game::words`].
    Word(usize),
    /// Brackets: where they start and their length.
    Brackets { start: usize, len: usize },
    /// One character.
    Char(usize),
}

/// How a session ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The password: the terminal opens.
    Granted,
    /// Out of attempts: the terminal locks.
    LockedOut,
}

/// What a choice did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    /// Log lines, oldest first.
    pub lines: Vec<String>,
    /// The attempts left are down to one ("!!! WARNING: LOCKOUT IMMINENT
    /// !!!" and the header flashing), or back up from one.
    pub warning: Option<bool>,
    pub outcome: Option<Outcome>,
}

/// One hacking session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub length: usize,
    /// The words still on the screen, in the game's list order.
    pub words: Vec<Vec<u8>>,
    /// Where each word starts on the screen.
    pub positions: Vec<usize>,
    pub password: Vec<u8>,
    /// The screen as shown (removed duds as dots).
    pub chars: Vec<u8>,
    pub attempts: u32,
    pub max_attempts: u32,
    /// The attempts were refilled once (`bUsedReset`).
    pub used_reset: bool,
    /// Where used brackets start (`xUsedBonuses`).
    pub used_brackets: Vec<usize>,
    pub outcome: Option<Outcome>,
}

impl Game {
    /// A new session (`Prepare` and `MakePasswordFile`): `None` when there
    /// are no words.
    pub fn new(
        dictionary: &[Vec<u8>],
        length: usize,
        count: u32,
        rng: &mut Twister,
        clock: &mut impl Clock,
    ) -> Option<Game> {
        let picked = choose_words(dictionary, length, count, rng, clock)?;
        if picked.words.is_empty() {
            return None;
        }
        let password = picked.words[picked.password].clone();
        let attempts = attempts(&picked.words);
        let file = PasswordFile::build(&picked.words, rng);
        let kept = picked.words.len() - file.dropped;
        let mut words = picked.words;
        words.truncate(kept);
        let mut positions = file.positions;
        positions.truncate(kept);
        Some(Game {
            length,
            words,
            positions,
            password,
            chars: file.chars,
            attempts,
            max_attempts: attempts,
            used_reset: false,
            used_brackets: Vec::new(),
            outcome: None,
        })
    }

    /// One line's 12 characters as shown.
    pub fn line(&self, line: usize) -> &[u8] {
        &self.chars[line * LINE_CHARS..(line + 1) * LINE_CHARS]
    }

    /// What the pointer at a screen position picks (`00769b50`).
    pub fn selection(&self, at: usize) -> Selection {
        if let Some(k) = self
            .positions
            .iter()
            .position(|&p| p <= at && at < p + self.length)
        {
            return Selection::Word(k);
        }
        if self.used_brackets.contains(&at) {
            return Selection::Char(at);
        }
        // `0076a670`: to a closing bracket of the same kind, on this line,
        // before any letter.
        let line_end = (at / LINE_CHARS + 1) * LINE_CHARS;
        if let Some(kind) = OPENERS.iter().position(|&o| o == self.chars[at]) {
            for i in at + 1..line_end {
                let c = self.chars[i];
                if c.is_ascii_alphabetic() {
                    break;
                }
                if c == CLOSERS[kind] {
                    return Selection::Brackets {
                        start: at,
                        len: i - at + 1,
                    };
                }
            }
        }
        Selection::Char(at)
    }

    /// The text a selection puts on the entry line.
    pub fn text(&self, selection: Selection) -> Vec<u8> {
        match selection {
            Selection::Word(k) => self.words[k].clone(),
            Selection::Brackets { start, len } => self.chars[start..start + len].to_vec(),
            Selection::Char(at) => vec![self.chars[at]],
        }
    }

    /// The player picks what's at a screen position (`00766b80`).
    /// `text(name, default)` gives a setting's text.
    pub fn choose(
        &mut self,
        at: usize,
        rng: &mut Twister,
        text: &impl Fn(&str) -> String,
    ) -> Choice {
        if self.outcome.is_some() || at >= FILE_CHARS {
            return Choice {
                lines: Vec::new(),
                warning: None,
                outcome: self.outcome,
            };
        }
        let picked = self.text(self.selection(at));
        let shown = String::from_utf8_lossy(&picked).into_owned();
        let line = |s: &str| format!(">{s}");
        let mut lines = vec![line(&shown)];
        let mut warning = None;
        let likeness = likeness(&picked, &self.password);
        if !picked[0].is_ascii_alphabetic() && picked.len() > 1 {
            if self.words.len() == 1 {
                lines.push(line(&text("sHackingDenied")));
            } else if !self.used_reset && rng.below(4) == 0 {
                if self.attempts == 1 {
                    warning = Some(false);
                }
                lines.push(line(&text("sHackingToleranceReset1")));
                lines.push(line(&text("sHackingToleranceReset2")));
                self.attempts = self.max_attempts;
                self.used_reset = true;
                self.used_brackets.push(at);
            } else {
                self.remove_dud(rng);
                self.used_brackets.push(at);
                lines.push(line(&text("sHackingDudRemoved")));
            }
        } else if likeness == self.length {
            for s in [
                "sHackingGranted",
                "sHackingAccessing1",
                "sHackingAccessing2",
                "sHackingAccessing3",
            ] {
                lines.push(line(&text(s)));
            }
            self.outcome = Some(Outcome::Granted);
        } else {
            lines.push(line(&text("sHackingDenied")));
            self.attempts = self.attempts.saturating_sub(1);
            if self.attempts == 0 {
                lines.push(line(&text("sHackingLockout1")));
                lines.push(text("sHackingLockout2"));
                self.outcome = Some(Outcome::LockedOut);
            } else {
                lines.push(format!(
                    ">{likeness}/{} {}.",
                    self.length,
                    text("sHackingCorrect")
                ));
                if self.attempts == 1 {
                    warning = Some(true);
                }
            }
        }
        Choice {
            lines,
            warning,
            outcome: self.outcome,
        }
    }

    /// A word that isn't the password turned to dots (`0076a730`).
    fn remove_dud(&mut self, rng: &mut Twister) {
        if !self.words.iter().any(|w| *w != self.password) {
            return;
        }
        let k = loop {
            let k = rng.below(self.words.len() as u32) as usize;
            if self.words[k] != self.password {
                break k;
            }
        };
        let at = self.positions[k];
        for c in &mut self.chars[at..at + self.length] {
            *c = b'.';
        }
        self.words.remove(k);
        self.positions.remove(k);
    }
}

/// A setting's text, else the exe's default from [`TEXTS`].
pub fn setting_text(order: &esm::LoadOrder, name: &str) -> String {
    crate::scripting::game_setting_text(order, name).unwrap_or_else(|| {
        TEXTS
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, d)| d.to_string())
            .unwrap_or_default()
    })
}

/// A whole-number setting, else the exe's default.
pub fn setting_int(order: &esm::LoadOrder, (name, default): (&str, i32)) -> i32 {
    crate::scripting::game_setting(order, name).map_or(default, |v| v as i32)
}
