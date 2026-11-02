//! Ready-made test characters: a text file that sets up the player and the
//! game as if they had been played up to a point (character creation done,
//! a level, gear, quests along the way), so a test can start somewhere
//! later in the game without the opening or the face menu.
//!
//! The file is the game's script language, one line at a time, as its
//! console runs it, with editor IDs (so it doesn't depend on the load
//! order's numbering). Blank lines and lines starting with `;` are skipped.
//! One line isn't script: `level N` puts the player at level N with the
//! experience it takes, and no level-up waiting (the game has no console
//! command for that without its level-up menus).
//!
//! It runs on a new game before its first frame, so quest scripts already
//! see the character.

use esm::LoadOrder;
use script::interp::Flow;

use crate::dialogue::PLAYER_REF;
use crate::scripting::{GameState, Runner, ScriptCache};

/// Something in a character file that didn't take.
#[derive(Debug, Clone, PartialEq)]
pub struct Problem {
    /// The line's number (from 1) and text.
    pub line: usize,
    pub text: String,
    pub why: String,
}

/// Sets up `state` as the character file says; what didn't take.
pub fn apply(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    file: &str,
) -> Vec<Problem> {
    let mut problems = Vec::new();
    for (i, raw) in file.lines().enumerate() {
        let text = raw.trim();
        if text.is_empty() || text.starts_with(';') {
            continue;
        }
        let problem = |why: String| Problem {
            line: i + 1,
            text: text.to_string(),
            why,
        };
        let mut words = text.split_whitespace();
        if words
            .next()
            .is_some_and(|w| w.eq_ignore_ascii_case("level"))
        {
            match words.next().and_then(|n| n.parse::<u16>().ok()) {
                Some(n) if n >= 1 && words.next().is_none() => set_level(order, state, n),
                _ => problems.push(problem("expected `level N`".into())),
            }
            continue;
        }
        // Names that aren't a form or a variable read as 0 in the game's
        // language; here they're worth knowing about.
        let unknown: Vec<String> = names_in(text)
            .into_iter()
            .filter(|w| unknown_name(order, w))
            .collect();
        let missing_before: usize = state.unhandled.values().sum();
        let flow = Runner::new(order, scripts, state).run_source(text, None, None);
        // One problem a line: a name that isn't a form explains the rest.
        if !unknown.is_empty() {
            problems.push(problem(format!("no form is called {}", unknown.join(", "))));
        } else if flow == Flow::Stopped {
            problems.push(problem(
                "stopped: a function it needs isn't carried out".into(),
            ));
        } else if state.unhandled.values().sum::<usize>() > missing_before {
            problems.push(problem(
                "skipped: a function in it isn't carried out".into(),
            ));
        }
    }
    problems
}

/// The player at `level`, with the experience it takes and no level-up
/// waiting.
pub fn set_level(order: &LoadOrder, state: &mut GameState, level: u16) {
    let level = level.min(crate::experience::max_level(order)).max(1);
    state.player_level = level;
    state.level_up_pending = false;
    let xp = crate::experience::xp_for_level(order, level);
    state
        .actor_values
        .insert((PLAYER_REF, crate::experience::XP), xp);
}

/// The words in a line that would name a form: `Name` or `Name.…`, not
/// numbers, keywords or the script's own words.
fn names_in(line: &str) -> Vec<String> {
    let code = line.split(';').next().unwrap_or("");
    let mut names = Vec::new();
    let mut first = true;
    for token in code.split(|c: char| c.is_whitespace() || "(),=<>!+-*/&|\"".contains(c)) {
        if token.is_empty() {
            continue;
        }
        // The function itself (first word, or after the reference's dot).
        let (reference, rest) = match token.split_once('.') {
            Some((r, rest)) => (Some(r), rest),
            None => (None, token),
        };
        if let Some(r) = reference {
            // `Ref.Function` or `Quest.variable`: only the part before the
            // dot is a form.
            names.push(r.to_string());
            first = false;
            continue;
        }
        if first {
            first = false;
            continue;
        }
        if rest.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
            && !["set", "to", "if", "else", "endif", "player", "playerref"]
                .iter()
                .any(|k| rest.eq_ignore_ascii_case(k))
        {
            names.push(rest.to_string());
        }
    }
    names
}

/// Whether a word names nothing the load order knows (`player` is always
/// known).
fn unknown_name(order: &LoadOrder, word: &str) -> bool {
    if word.eq_ignore_ascii_case("player") || word.eq_ignore_ascii_case("playerref") {
        return false;
    }
    // Actor value names and other keywords the functions take aren't forms.
    if script::actor_value(word).is_some() || u32::from_str_radix(word, 16).is_ok() {
        return false;
    }
    order.form_by_editor_id(word).is_none()
}
