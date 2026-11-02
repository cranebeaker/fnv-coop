//! Dialogue: topics (`DIAL`) and the lines said under them (`INFO`, in each
//! topic's child group), with the conditions that pick who says which
//! line and when.
//!
//! A line holds one or more responses (`TRDT` + `NAM1`: the emotion, the
//! response number and the text), the player's prompt for it (`RNAM`), the
//! topics offered next (`TCLT`), and conditions (`CTDA`), each a game
//! function (see [`crate::functions`]) compared with a value.

use esm::{FormId, FourCC, LoadOrder, Record, RecordRef};

use crate::cell::{le_f32, le_u32};
use crate::functions::function_name;

const DIAL: FourCC = FourCC::new(b"DIAL");
const INFO: FourCC = FourCC::new(b"INFO");
const TRDT: FourCC = FourCC::new(b"TRDT");
const NAM1: FourCC = FourCC::new(b"NAM1");
const CTDA: FourCC = FourCC::new(b"CTDA");
const TCLT: FourCC = FourCC::new(b"TCLT");
const TCFU: FourCC = FourCC::new(b"TCFU");
const NAME: FourCC = FourCC::new(b"NAME");
const RNAM: FourCC = FourCC::new(b"RNAM");
const QSTI: FourCC = FourCC::new(b"QSTI");
const INFC: FourCC = FourCC::new(b"INFC");
const PNAM: FourCC = FourCC::new(b"PNAM");
const KNAM: FourCC = FourCC::new(b"KNAM");
const TDUM: FourCC = FourCC::new(b"TDUM");
const FULL: FourCC = FourCC::new(b"FULL");
const SCTX: FourCC = FourCC::new(b"SCTX");
const NEXT: FourCC = FourCC::new(b"NEXT");

/// Function numbers the conditions use most.
pub mod functions {
    pub const GET_IS_ID: u16 = 72;
    pub const GET_IS_VOICE_TYPE: u16 = 427;
    pub const GET_IS_SEX: u16 = 70;
    pub const GET_IS_RACE: u16 = 69;
    pub const GET_IN_FACTION: u16 = 71;
    pub const GET_STAGE: u16 = 58;
    pub const GET_STAGE_DONE: u16 = 59;
    pub const GET_QUEST_RUNNING: u16 = 56;
    pub const GET_GLOBAL_VALUE: u16 = 74;
    pub const GET_IS_REFERENCE: u16 = 136;
    pub const GET_ACTOR_VALUE: u16 = 14;
    pub const HAS_PERK: u16 = 449;
}

/// A dialogue topic.
#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// What the player picks (`FULL`).
    pub name: Option<String>,
    /// `DATA`: the kind (0 an ordinary topic, 1 conversation, 2 combat, 3
    /// persuasion, 4 detection, 5 service, 6 miscellaneous, 7 radio) and
    /// flags (0x01 rumors, 0x02 top-level). Checked on Sunny Smiles'
    /// topics: her questions ("What do you do around here?") are 0 / 0x02,
    /// the follow-ups only offered after a line ("How many are there?")
    /// 0 / 0, her barks 1 / 0.
    pub kind: u8,
    pub flags: u8,
    /// `PNAM`: the topic's priority (50 unless set; Sunny's questions 75
    /// to 97).
    pub priority: f32,
    /// `TDUM`: what a player of low Intelligence says instead.
    pub dumb_prompt: Option<String>,
    /// The quests with lines under it (`QSTI`).
    pub quests: Vec<FormId>,
}

/// Topic `DATA` kind: an ordinary topic, the kind the player picks.
pub const ORDINARY_TOPIC: u8 = 0;
/// Topic `DATA` flag: offered in the list of things to ask about.
pub const TOP_LEVEL: u8 = 0x02;

/// One response of a line.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    /// `TRDT`: the emotion (0 neutral, 1 anger, 2 disgust, 3 fear, 4 sad,
    /// 5 happy, 6 surprise, 7 pained), its strength, and the response
    /// number (the voice file's last part).
    pub emotion: u32,
    pub emotion_value: i32,
    pub number: u8,
    /// `NAM1` as the game says it: the actors' notes in braces left out
    /// ([`without_notes`]).
    pub text: String,
    /// `TRDT` byte 20: the emotion counts for the speaker's animations
    /// (`DialogueResponse::bUseEmotion`, Xbox PDB; `008a5580` stores it at
    /// actor +0x86, which `GetDialogueEmotion` checks, `005a4480`).
    pub use_emotion: bool,
    /// `SNAM` and `LNAM` after the `TRDT`: the idles the speaker and the
    /// listener play with this response (`pSpeakerIdle`, `pListenIdle`,
    /// Xbox PDB; read by `0061e780`, said by `008a20d0`).
    pub speaker_idle: Option<FormId>,
    pub listener_idle: Option<FormId>,
    /// `TRDT` bytes 16..20: a sound form said instead of the voice file
    /// (`DialogueResponse::pVoiceSound` (Xbox PDB), `+0x20`): the radio's
    /// songs.
    pub sound: Option<FormId>,
}

/// A response's text as the game shows it (`0083d8b0`, building a line's
/// response, `0083d280`): what's in braces left out, braces within braces
/// counted, and a `}` with none open dropped ("{Rush of health}MMmmahh."
/// is "MMmmahh.").
pub fn without_notes(text: &str) -> String {
    let mut depth = 0u32;
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        if depth == 0 && c != '}' {
            out.push(c);
        }
    }
    out
}

/// How a condition compares (the top three bits of its first byte).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

/// One condition: `function(params) <comparison> value`. Conditions are
/// joined with AND unless a condition's OR flag joins it to the next.
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub comparison: Comparison,
    /// Joined to the next condition with OR rather than AND.
    pub or: bool,
    /// The value is a global variable's (`GLOB`) instead.
    pub value: f32,
    pub global: Option<FormId>,
    pub function: u16,
    /// The parameters as stored, and read as form IDs (in load-order
    /// numbering) for functions that take records.
    pub params: [u32; 2],
    pub param_forms: [FormId; 2],
    /// Who the function is asked about: 0 the speaker, 1 the target (the
    /// player in dialogue), 2 a reference, ...
    pub run_on: u32,
    pub reference: Option<FormId>,
}

impl Condition {
    pub fn function_name(&self) -> String {
        function_name(self.function)
    }

    /// Whether `result` passes the comparison.
    pub fn compare(&self, result: f32, value: f32) -> bool {
        match self.comparison {
            Comparison::Equal => result == value,
            Comparison::NotEqual => result != value,
            Comparison::Greater => result > value,
            Comparison::GreaterOrEqual => result >= value,
            Comparison::Less => result < value,
            Comparison::LessOrEqual => result <= value,
        }
    }
}

/// A dialogue line.
#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub form_id: FormId,
    pub topic: Option<FormId>,
    /// The quest the line belongs to (`QSTI`).
    pub quest: Option<FormId>,
    /// The line before it in its topic's order (`PNAM`).
    pub previous: Option<FormId>,
    /// `DATA`: type, next speaker, flags (0x01 goodbye, 0x02 random, 0x04
    /// say once, ...).
    pub flags: u8,
    /// `DATA` byte 3, the second flags (the game keeps the four bytes at
    /// `INFO+0x23`, `0061dbd0`): 0x02 "always darken" (`00763fd0` reads
    /// `+0x26 & 2`).
    pub flags2: u8,
    pub responses: Vec<Response>,
    pub conditions: Vec<Condition>,
    /// The player's line that leads to this one (`RNAM`), when it differs
    /// from the topic's name.
    pub prompt: Option<String>,
    /// The skill, S.P.E.C.I.A.L. (`AVIF`) or perk the choice is a check of
    /// (`KNAM`; Trudy's "< Speech 25 >" line names `AVSpeech`), for its tag.
    pub check: Option<FormId>,
    /// Topics offered after this line (`TCLT`), and added to what the
    /// player can ask about (`NAME`).
    pub choices: Vec<FormId>,
    pub add_topics: Vec<FormId>,
    /// Lines the speaker goes straight on to after this one, without the
    /// player choosing (`TCFU`, read into the line's conversation data by
    /// `0061dbd0`; the first that can be said is chosen by `0061af30`, see
    /// [`follow_up`]). Doc Mitchell's farewell at his door is a chain of
    /// them ending in the Goodbye line that lets the player leave.
    pub follow_ups: Vec<FormId>,
    /// Result scripts (`SCTX`): run as the line starts, and after it's
    /// said (the second, after the `NEXT` marker).
    pub begin_script: Option<String>,
    pub end_script: Option<String>,
}

/// `DATA` flag: the conversation ends after this line.
pub const GOODBYE: u8 = 0x01;
/// `DATA` flag: said only once a game (Doc Mitchell's "You're awake. How
/// about that." has it, and the intro asks for the topic again later).
pub const SAY_ONCE: u8 = 0x04;
/// `DATA` flag 0x02: one of a run of lines picked among at random
/// (`00619410`; `0061a7d0` and `0061af30` gather consecutive available
/// lines with it and choose one, see [`choose`]).
pub const RANDOM: u8 = 0x02;
/// `DATA` flag 0x08 (the GECK calls it "Run Immediately"; the name isn't
/// in the executable): the dialogue menu neither runs this line's second
/// result script nor looks for its follow-ups (`00579200`, tested by
/// `00762ff0` and the menu's close `00762160`). Where such a line's second
/// script runs instead is not traced.
pub const RUN_IMMEDIATELY: u8 = 0x08;
/// `DATA` flag 0x20: the last line of a random run (`0061aed0`).
pub const RANDOM_END: u8 = 0x20;
/// `DATA` flag 0x40 (the GECK's "Run for Rumors", name unverified): the
/// menu doesn't run this line's first result script (`0083ebb0` tests
/// `INFO+0x25 & 0x40` before running script 0).
pub const NO_BEGIN_SCRIPT_IN_MENU: u8 = 0x40;
/// `DATA` byte 3 flags (`INFO+0x26`): lines for a listener of low or high
/// Intelligence only (`0061e720`: 0x10 → class 1, else 0x20 → class 3,
/// else 2; `0061e600` drops class 3 when the listener's Intelligence is at
/// most `iDialogueDummySpeakThisIntOrBelow` (`011d0dc8`), class 1 when
/// above).
pub const LOW_INTELLIGENCE: u8 = 0x10;
pub const HIGH_INTELLIGENCE: u8 = 0x20;

/// The hard-coded `GOODBYE` topic (default topic kind 1, index 2:
/// `0061a2d0(1, 2)`).
pub const GOODBYE_TOPIC: FormId = crate::social::topics::GOODBYE;

/// Whether the player activating someone has them just say their greeting,
/// with no dialogue menu (an NPC's `Activate`, `005fa330` at `005faa3f`):
/// the `GREETING` line found for them (`0061a2d0(0, 0)`, `0061b320`) is
/// flagged Goodbye (`00619df0`: `INFO+0x25 & 1`) and has a single response
/// (`0083c7b0` finds a first, `0083c7e0` no next). Then the actor says the
/// topic to the player through `0057b7c0` (the speech in progress stopped,
/// the listener set to the player, the GREET procedure, `008dbe30`), which
/// picks the line again; otherwise the dialogue menu opens (`00709470(4,
/// …)`). No line at all: nothing happens.
// Translated from 005fa330 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn activation_says_a_line(info: &Info) -> bool {
    info.flags & GOODBYE != 0 && info.responses.len() == 1
}

fn global(rr: &RecordRef<'_>, data: &[u8]) -> FormId {
    rr.plugin.to_global(FormId(le_u32(data, 0)))
}

impl Topic {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Topic> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == DIAL)?;
        let record = rr.record().ok()?;
        Some(Topic::parse(&rr, &record))
    }

    fn parse(rr: &RecordRef<'_>, record: &Record) -> Topic {
        let data = record.get(esm::sig::DATA).map(|s| s.data.as_slice());
        let byte = |i: usize| data.and_then(|d| d.get(i).copied()).unwrap_or(0);
        Topic {
            form_id: rr.form_id,
            editor_id: record.editor_id(),
            name: record.get(FULL).map(|s| s.zstring()),
            kind: byte(0),
            flags: byte(1),
            priority: record
                .get(PNAM)
                .filter(|s| s.data.len() >= 4)
                .map_or(50.0, |s| le_f32(&s.data, 0)),
            dumb_prompt: record
                .get(TDUM)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty()),
            quests: record
                .get_all(QSTI)
                .filter(|s| s.data.len() >= 4)
                .map(|s| global(rr, &s.data))
                .collect(),
        }
    }

    /// Flagged top-level, whatever its kind: the game adds every such topic
    /// to the player's topics as it's loaded (`TESTopic::InitItem`, Xbox
    /// PDB, `00619030`, tests `00619410`: `DATA` flags 0x02 only). The
    /// hard-coded `GOODBYE` topic is one (kind 1, flags 0x02, priority 5):
    /// Sunny Smiles' "Until next time." answers it from the main list.
    pub fn is_top_level(&self) -> bool {
        self.flags & TOP_LEVEL != 0
    }

    /// The topic's Intelligence class (`0083f0d0`: `DATA` flags 0x10 → 1,
    /// else 0x20 → 3, else 2), as for lines ([`LOW_INTELLIGENCE`],
    /// [`HIGH_INTELLIGENCE`]).
    pub fn intelligence_class(&self) -> u8 {
        if self.flags & LOW_INTELLIGENCE != 0 {
            1
        } else if self.flags & HIGH_INTELLIGENCE != 0 {
            3
        } else {
            2
        }
    }

    /// What the menu shows for it when its line has no prompt of its own.
    pub fn label(&self) -> Option<String> {
        self.name
            .clone()
            .filter(|n| !n.is_empty())
            .or_else(|| self.editor_id.clone())
    }
}

/// Every top-level topic in the game, highest priority first (the order
/// the menu lists them in, an assumption: equal priorities keep the files'
/// order).
pub fn top_level_topics(order: &LoadOrder) -> Vec<Topic> {
    let mut topics: Vec<Topic> = order
        .records_of_type(DIAL)
        .filter(|rr| !rr.entry.header.is_deleted())
        .filter_map(|rr| {
            let record = rr.record().ok()?;
            Some(Topic::parse(&rr, &record))
        })
        .filter(Topic::is_top_level)
        .collect();
    topics.sort_by(|a, b| b.priority.total_cmp(&a.priority));
    topics
}

/// One thing the player can say: the topic, the line the speaker would
/// answer with, and the text shown (the line's prompt, else the topic's
/// name).
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    pub topic: Topic,
    pub info: Info,
    pub label: String,
    /// Shown dimmed (the dialogue menu's `_line_alpha` 128, `007638b0`):
    /// its line has been said before, or is flagged "always darken"
    /// (`DATA` byte 3, 0x02).
    pub dim: bool,
    /// A skill check the player doesn't pass: its tag reads "[Speech
    /// 12/25]", and the menu's highlight turns dark red on it (`007638b0`
    /// sets the tile's flag, `00764ef0` copies it).
    pub failed: bool,
}

/// `DATA` byte 3 flag: the choice is always shown dimmed.
pub const ALWAYS_DARKEN: u8 = 0x02;

/// The hard-coded `SpeechChallengeFailure` and `InfoRefusal` topics
/// (`0083de60` tests their form IDs, 0xFD and 0x118): never in a topic
/// list (`0083ed50`).
pub const SPEECH_CHALLENGE_FAILURE: FormId = FormId(0xFD);
pub const INFO_REFUSAL: FormId = FormId(0x118);

/// Each topic the speaker has a line for now, as a choice, in the order
/// given (`MenuTopicManager::FillTopicList`, Xbox PDB, `0083ed50`): the
/// two hard-coded refusal topics are left out, and topics of the wrong
/// Intelligence class for the player ([`Topic::intelligence_class`],
/// compared as for lines). The topic's rumor flag (0x01) and what it does
/// there (`0042df90`, `0087f510`) are not carried out.
fn answered(
    order: &LoadOrder,
    topics: &[&Topic],
    speaker: &Speaker,
    state: &GameState,
) -> Vec<Choice> {
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    topics
        .iter()
        .filter(|t| t.form_id != SPEECH_CHALLENGE_FAILURE && t.form_id != INFO_REFUSAL)
        .filter(|t| suits_intelligence(order, &facts, t.intelligence_class(), PLAYER_REF))
        // Only a running quest's lines can be said; the topic's record
        // lists the quests it has lines for.
        .filter(|t| t.quests.is_empty() || t.quests.iter().any(|q| state.running.contains(q)))
        .filter_map(|t| {
            let info = pick(order, t.form_id, speaker, state)?;
            let mut label = prompt(order, t, &info, speaker, state)?;
            let mut failed = false;
            if let Some((tag, passed)) = check_tag_passed(order, &info, speaker, state) {
                label = format!("{tag}  {label}");
                failed = !passed;
            }
            let dim = state.said.contains(&info.form_id) || info.flags2 & ALWAYS_DARKEN != 0;
            Some(Choice {
                topic: (*t).clone(),
                info,
                label,
                dim,
                failed,
            })
        })
        .collect()
}

/// What a choice says (`0083db00`): the topic's "dumb" prompt (`TDUM`)
/// when the player's Intelligence is at most
/// `iDialogueDummySpeakThisIntOrBelow` (3) and the topic has one; else the
/// line's own prompt (`RNAM`); else the topic's name.
pub fn prompt(
    order: &LoadOrder,
    topic: &Topic,
    info: &Info,
    speaker: &Speaker,
    state: &GameState,
) -> Option<String> {
    if let Some(dumb) = &topic.dumb_prompt {
        let most = crate::scripting::game_setting(order, "iDialogueDummySpeakThisIntOrBelow")
            .unwrap_or(4.0);
        let facts = crate::scripting::Facts {
            order,
            state,
            speaker: Some(speaker),
        };
        let intelligence = facts.current_actor_value(PLAYER_REF, 9).unwrap_or(5.0);
        if intelligence <= f64::from(most) {
            return Some(dumb.clone());
        }
    }
    info.prompt.clone().or_else(|| topic.label())
}

/// The tag a skill check's choice is shown with (`007638b0`, read from the
/// game's code): only for a line naming a skill, S.P.E.C.I.A.L. or perk in
/// `KNAM`, that record's name as stored; then from the line's first
/// `GetActorValue` condition (whatever its comparison, run-on or value
/// asked about) the value needed and the player's own (rounded down) of the
/// value it names: "[Speech 25]" when the player has enough, "[Speech
/// 12/25]" when not; a line with no such condition (most perks) just
/// "[Black Widow]". The menu puts two spaces before the prompt. Trudy's
/// "< Speech 25 >" line: `KNAM` `AVSpeech`, `GetActorValue Speech >= 25`.
/// (The words "[SUCCEEDED]" and "[FAILED]" are in the lines' own text.)
pub fn check_tag(
    order: &LoadOrder,
    info: &Info,
    speaker: &Speaker,
    state: &GameState,
) -> Option<String> {
    check_tag_passed(order, info, speaker, state).map(|(tag, _)| tag)
}

/// [`check_tag`], and whether the player passes the check (a line with no
/// `GetActorValue` condition counts as passed: the menu marks only a value
/// short of the one needed, `007638b0`).
pub fn check_tag_passed(
    order: &LoadOrder,
    info: &Info,
    speaker: &Speaker,
    state: &GameState,
) -> Option<(String, bool)> {
    let checked = info.check?;
    let name = order
        .get(checked)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| r.full_name())?;
    let Some(c) = info
        .conditions
        .iter()
        .find(|c| c.function == functions::GET_ACTOR_VALUE)
    else {
        return Some((format!("[{name}]"), true));
    };
    let needed = match c.global {
        Some(g) => state.globals.get(&g).copied().unwrap_or(0.0),
        None => c.value,
    }
    .trunc();
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    let have = facts
        .current_actor_value(PLAYER_REF, c.params[0] as u16)
        .unwrap_or(0.0)
        .floor();
    if have >= f64::from(needed) {
        Some((format!("[{name} {needed}]"), true))
    } else {
        Some((format!("[{name} {have}/{needed}]"), false))
    }
}

/// The main list of things to ask about, shown after a line with no
/// choices of its own (`TCLT`): the player's topics (`PlayerCharacter`
/// `+0x6a8`, `0083ec30`), which are every top-level topic (added as the
/// game loads, `00619030`) and every topic learned since (`AddTopic`
/// `00952830`, a said line's `NAME` via `0061f150`), each once, for which
/// the speaker has a line now ([`answered`]), highest priority first
/// (`0083ed50` sorts this list, not a line's choices, with `0083f110`;
/// whether that sort keeps equal priorities in order isn't traced, they
/// keep the files' order here).
///
/// Sunny Smiles: "That's all I wanted to know. Let's talk about something
/// else." has no choices, so it leads back here, where her top-level
/// questions and the hard-coded `GOODBYE` topic (top-level, priority 5,
/// her "Until next time.") are offered.
pub fn menu_topics(
    order: &LoadOrder,
    top_level: &[Topic],
    speaker: &Speaker,
    state: &GameState,
) -> Vec<Choice> {
    let mut learned: Vec<FormId> = state.topics.iter().copied().collect();
    learned.sort_by_key(|id| id.0);
    let mut seen: std::collections::HashSet<FormId> = top_level.iter().map(|t| t.form_id).collect();
    let learned: Vec<Topic> = learned
        .into_iter()
        .filter(|id| seen.insert(*id))
        .filter_map(|id| Topic::load(order, id))
        .collect();
    let mut topics: Vec<&Topic> = top_level.iter().chain(&learned).collect();
    topics.sort_by(|a, b| b.priority.total_cmp(&a.priority));
    answered(order, &topics, speaker, state)
}

/// What the player can say after a line (`0083ec30`): its choices
/// (`TCLT`) in their stored order, when it has any (the menu topic's
/// `bTopicIsChoice`, Xbox PDB, set by `0083db00` when the line's choice
/// list isn't empty); otherwise the main list ([`menu_topics`]).
pub fn next_choices(
    order: &LoadOrder,
    info: &Info,
    top_level: &[Topic],
    speaker: &Speaker,
    state: &GameState,
) -> Vec<Choice> {
    if info.choices.is_empty() {
        return menu_topics(order, top_level, speaker, state);
    }
    let topics: Vec<Topic> = info
        .choices
        .iter()
        .filter_map(|&id| Topic::load(order, id))
        .collect();
    let topics: Vec<&Topic> = topics.iter().collect();
    answered(order, &topics, speaker, state)
}

/// A line starts: it's been said (for "say once"), the speaker has talked
/// to the player, and the topics it names (`NAME`) are learned.
pub fn line_begins(state: &mut GameState, info: &Info, speaker: FormId) {
    use crate::coop::Domain;
    if state.is_guest() {
        return; // the host runs the line's effects and replicates them
    }
    state.emit(Domain::Said, info.form_id);
    state.emit(Domain::TalkedTo, speaker);
    for t in &info.add_topics {
        state.emit(Domain::Topic, *t);
    }
    state.said.insert(info.form_id);
    state.talked_to.insert(speaker);
    state.topics.extend(info.add_topics.iter().copied());
}

pub(crate) fn read_condition(rr: &RecordRef<'_>, data: &[u8]) -> Option<Condition> {
    if data.len() < 24 {
        return None;
    }
    let kind = data[0];
    let comparison = match kind >> 5 {
        0 => Comparison::Equal,
        1 => Comparison::NotEqual,
        2 => Comparison::Greater,
        3 => Comparison::GreaterOrEqual,
        4 => Comparison::Less,
        _ => Comparison::LessOrEqual,
    };
    let uses_global = kind & 0x04 != 0;
    Some(Condition {
        comparison,
        or: kind & 0x01 != 0,
        value: if uses_global { 0.0 } else { le_f32(data, 4) },
        global: uses_global.then(|| global(rr, &data[4..])),
        function: u16::from_le_bytes([data[8], data[9]]),
        params: [le_u32(data, 12), le_u32(data, 16)],
        param_forms: [global(rr, &data[12..]), global(rr, &data[16..])],
        run_on: le_u32(data, 20),
        reference: (data.len() >= 28)
            .then(|| global(rr, &data[24..]))
            .filter(|id| id.0 != 0),
    })
}

impl Info {
    pub fn parse(order: &LoadOrder, rr: &RecordRef<'_>, record: &Record) -> Info {
        let form = |kind: FourCC| {
            record
                .get(kind)
                .filter(|s| s.data.len() >= 4)
                .map(|s| global(rr, &s.data))
                .filter(|id| id.0 != 0)
        };
        let mut responses = Vec::new();
        let mut pending: Option<Response> = None;
        let mut conditions = Vec::new();
        let mut choices = Vec::new();
        let mut add_topics = Vec::new();
        let mut follow_ups = Vec::new();
        let mut scripts: [Option<String>; 2] = [None, None];
        let mut after_next = false;
        for sub in &record.subrecords {
            match sub.kind {
                k if k == NEXT => after_next = true,
                k if k == SCTX => {
                    let text = esm::text::decode_cp1252(&sub.data);
                    scripts[usize::from(after_next)] = Some(text);
                }
                k if k == TRDT && sub.data.len() >= 13 => {
                    if let Some(r) = pending.take() {
                        responses.push(r);
                    }
                    pending = Some(Response {
                        emotion: le_u32(&sub.data, 0),
                        emotion_value: le_u32(&sub.data, 4) as i32,
                        number: sub.data[12],
                        text: String::new(),
                        use_emotion: sub.data.get(20).is_some_and(|b| *b != 0),
                        speaker_idle: None,
                        listener_idle: None,
                        sound: (sub.data.len() >= 20)
                            .then(|| global(rr, &sub.data[16..]))
                            .filter(|id| id.0 != 0),
                    });
                }
                k if k == NAM1 => {
                    if let Some(r) = pending.as_mut() {
                        r.text = without_notes(&sub.zstring());
                    }
                }
                // A response's idles (`0061e780`: only after a `TRDT`).
                k if (k == SNAM || k == LNAM) && sub.data.len() >= 4 => {
                    if let Some(r) = pending.as_mut() {
                        let idle = Some(global(rr, &sub.data)).filter(|id| id.0 != 0);
                        if k == SNAM {
                            r.speaker_idle = idle;
                        } else {
                            r.listener_idle = idle;
                        }
                    }
                }
                k if k == CTDA => conditions.extend(read_condition(rr, &sub.data)),
                k if k == TCLT && sub.data.len() >= 4 => choices.push(global(rr, &sub.data)),
                k if k == TCFU && sub.data.len() >= 4 => follow_ups.push(global(rr, &sub.data)),
                k if k == NAME && sub.data.len() >= 4 => add_topics.push(global(rr, &sub.data)),
                _ => {}
            }
        }
        if let Some(r) = pending.take() {
            responses.push(r);
        }
        Info {
            form_id: rr.form_id,
            topic: order.topic_of(rr),
            quest: form(QSTI),
            previous: form(PNAM),
            flags: record
                .get(esm::sig::DATA)
                .and_then(|s| s.data.get(2).copied())
                .unwrap_or(0),
            flags2: record
                .get(esm::sig::DATA)
                .and_then(|s| s.data.get(3).copied())
                .unwrap_or(0),
            responses,
            conditions,
            prompt: record
                .get(RNAM)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty()),
            check: form(KNAM),
            choices,
            add_topics,
            follow_ups,
            begin_script: scripts[0].take(),
            end_script: scripts[1].take(),
        }
    }

    pub fn load(order: &LoadOrder, id: FormId) -> Option<Info> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == INFO)?;
        let record = rr.record().ok()?;
        Some(Info::parse(order, &rr, &record))
    }

    /// Whether a condition names this subject: `GetIsID` of the base, or
    /// `GetIsVoiceType` of its voice, asked about the speaker.
    pub fn names_speaker(&self, base: FormId, voice: Option<FormId>) -> bool {
        self.conditions.iter().any(|c| {
            c.run_on == 0
                && c.comparison == Comparison::Equal
                && c.value == 1.0
                && ((c.function == functions::GET_IS_ID && c.param_forms[0] == base)
                    || (c.function == functions::GET_IS_VOICE_TYPE
                        && voice == Some(c.param_forms[0])))
        })
    }
}

/// Every line whose conditions name the speaker (its base record, or its
/// voice type), in record order. Lines anyone can say aren't included.
pub fn lines_for(order: &LoadOrder, base: FormId, voice: Option<FormId>) -> Vec<Info> {
    let mut out = Vec::new();
    for rr in order.records_of_type(INFO) {
        let Ok(record) = rr.record() else {
            continue;
        };
        // Quick check before parsing: the base's form ID somewhere in a
        // condition.
        let mentions = record.get_all(CTDA).any(|s| {
            s.data.len() >= 16 && {
                let p = global(&rr, &s.data[12..]);
                p == base || voice == Some(p)
            }
        });
        if !mentions {
            continue;
        }
        let info = Info::parse(order, &rr, &record);
        if info.names_speaker(base, voice) {
            out.push(info);
        }
    }
    out
}

const VTCK: FourCC = FourCC::new(b"VTCK");
const VNAM: FourCC = FourCC::new(b"VNAM");
const RNAM_RACE: FourCC = FourCC::new(b"RNAM");
const ACBS: FourCC = FourCC::new(b"ACBS");
const SNAM: FourCC = FourCC::new(b"SNAM");
const LNAM: FourCC = FourCC::new(b"LNAM");

/// The player's reference and base record (fixed forms in every game).
pub const PLAYER_REF: FormId = FormId(0x14);
pub const PLAYER_BASE: FormId = FormId(0x7);

pub use crate::scripting::GameState;

/// Who's speaking: what conditions ask about them.
#[derive(Debug, Clone, PartialEq)]
pub struct Speaker {
    pub reference: FormId,
    pub base: FormId,
    pub name: Option<String>,
    pub voice: Option<FormId>,
    pub race: Option<FormId>,
    pub female: bool,
    pub factions: Vec<FormId>,
}

impl Speaker {
    /// A placed NPC (or creature) by its reference and base.
    pub fn load(order: &LoadOrder, reference: FormId, base: FormId) -> Option<Speaker> {
        let rr = order.get(base)?;
        let record = rr.record().ok()?;
        let form = |kind: FourCC| {
            record
                .get(kind)
                .filter(|s| s.data.len() >= 4)
                .map(|s| global(&rr, &s.data))
                .filter(|id| id.0 != 0)
        };
        Some(Speaker {
            reference,
            base,
            name: record.full_name(),
            // A talking activator's voice type is its VNAM (form type
            // 0x16 in 00616fa0 → 009185e0).
            voice: form(VTCK).or_else(|| {
                (rr.entry.header.kind.as_bytes() == b"TACT")
                    .then(|| form(VNAM))
                    .flatten()
            }),
            race: form(RNAM_RACE),
            female: record
                .get(ACBS)
                .filter(|s| s.data.len() >= 4)
                .is_some_and(|s| le_u32(&s.data, 0) & 1 != 0),
            factions: record
                .get_all(SNAM)
                .filter(|s| s.data.len() >= 4)
                .map(|s| global(&rr, &s.data))
                .collect(),
        })
    }
}

/// Whether a line's conditions pass, said by `speaker` to the player (see
/// [`crate::scripting::Facts::conditions_pass`]).
pub fn passes(order: &LoadOrder, info: &Info, speaker: &Speaker, state: &GameState) -> bool {
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    facts.conditions_pass(&info.conditions, speaker.reference, PLAYER_REF)
}

/// A topic's lines in the order the game tries them: by their quests'
/// priority (highest first), then as stored.
pub fn topic_lines(order: &LoadOrder, topic: FormId) -> Vec<Info> {
    let priority = |quest: Option<FormId>| -> u8 {
        quest
            .and_then(|q| order.get(q))
            .and_then(|r| r.record().ok())
            .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.get(1).copied()))
            .unwrap_or(0)
    };
    let mut lines: Vec<(u8, Info)> = order
        .in_topic(topic)
        .into_iter()
        .filter(|rr| rr.entry.header.kind == INFO && !rr.entry.header.is_deleted())
        .filter_map(|rr| {
            let record = rr.record().ok()?;
            let info = Info::parse(order, &rr, &record);
            Some((priority(info.quest), info))
        })
        .collect();
    // The lines the topic is connected to (`INFC`, in the topic record):
    // the game's topic loader puts them in the topic's list of lines
    // (`TESTopic::vfunc_8`, `00618aa0`), though they stand under another
    // topic. Dead Money's Dog says his Gala greeting through one.
    // [G] that picking a line uses them as it does the topic's own.
    if let Some(record) = order
        .get(topic)
        .and_then(|rr| rr.record().ok().map(|r| (rr, r)))
    {
        let (rr, record) = record;
        for sub in record.get_all(INFC).filter(|s| s.data.len() >= 4) {
            let id = global(&rr, &sub.data);
            if lines.iter().any(|(_, i)| i.form_id == id) {
                continue;
            }
            let Some(line) = order
                .get(id)
                .filter(|l| l.entry.header.kind == INFO && !l.entry.header.is_deleted())
            else {
                continue;
            };
            if let Ok(record) = line.record() {
                let info = Info::parse(order, &line, &record);
                lines.push((priority(info.quest), info));
            }
        }
    }
    lines.sort_by_key(|(p, _)| std::cmp::Reverse(*p));
    lines.into_iter().map(|(_, i)| i).collect()
}

/// The first line of a topic the speaker can say now: its quest (`QSTI`)
/// is running and the quest's own conditions pass for the speaker (they
/// apply to all its dialogue: without them, Sunny Smiles greets the
/// player with one of ED-E's beeps, a line with no conditions of its own
/// in `vDialogueEDE`, whose conditions name ED-E), its own conditions
/// pass, and it isn't a "say once" line already said.
///
/// A run of lines flagged random is chosen among ([`choose`], with the
/// state's current dice value; callers that want a fresh draw roll the
/// dice first). Lines for the other end of the Intelligence scale than the
/// player's are left out ([`LOW_INTELLIGENCE`]).
pub fn pick(
    order: &LoadOrder,
    topic: FormId,
    speaker: &Speaker,
    state: &GameState,
) -> Option<Info> {
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    let mut quest_ok: std::collections::HashMap<FormId, bool> = Default::default();
    let available = topic_lines(order, topic).into_iter().filter(|info| {
        if let Some(q) = info.quest {
            let ok = *quest_ok.entry(q).or_insert_with(|| {
                state.running.contains(&q)
                    && facts.conditions_pass(
                        &crate::quest::quest_conditions(order, q),
                        speaker.reference,
                        PLAYER_REF,
                    )
            });
            if !ok {
                return false;
            }
        }
        line_available(order, info, speaker, PLAYER_REF, state)
    });
    choose(available, state.dice)
}

/// Whether a line itself can be said by `speaker` to `listener` now
/// (`0061e600`, read): it has something to say (an nv-rs rule: a line
/// without responses isn't picked), it isn't a "say once" line already
/// said, it suits the listener's Intelligence, and its conditions pass.
/// "Say once a day" (`DATA` byte 3 flag 0x01, a per-speaker list read by
/// `00935a40`) is not carried out: when that list is cleared isn't
/// traced.
pub fn line_available(
    order: &LoadOrder,
    info: &Info,
    speaker: &Speaker,
    listener: FormId,
    state: &GameState,
) -> bool {
    if info.responses.is_empty()
        || (info.flags & SAY_ONCE != 0 && state.said.contains(&info.form_id))
    {
        return false;
    }
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    let class = if info.flags2 & LOW_INTELLIGENCE != 0 {
        1
    } else if info.flags2 & HIGH_INTELLIGENCE != 0 {
        3
    } else {
        2
    };
    if !suits_intelligence(order, &facts, class, listener) {
        return false;
    }
    facts.conditions_pass(&info.conditions, speaker.reference, listener)
}

/// Translated from 0061e600 (decompiled, FalloutNV.exe 1.4.0.525): the
/// Intelligence class of a line (`0061e720`), or of a topic (`0083ed50`
/// with `0083f0d0`, the same comparison), against the listener's
/// Intelligence (actor value 9) and `iDialogueDummySpeakThisIntOrBelow`
/// (exe 4, `011d0dc8`; the data sets 3). A listener whose Intelligence
/// isn't known here passes (unresolved: the game always has one).
fn suits_intelligence(
    order: &LoadOrder,
    facts: &crate::scripting::Facts<'_>,
    class: u8,
    listener: FormId,
) -> bool {
    if class == 2 {
        return true;
    }
    let Some(intelligence) = facts.current_actor_value(listener, 9) else {
        return true;
    };
    let most =
        crate::scripting::game_setting(order, "iDialogueDummySpeakThisIntOrBelow").unwrap_or(4.0);
    // The game compares the whole value (vfunc +8 gives an int).
    if intelligence.trunc() <= f64::from(most).trunc() {
        class != 3
    } else {
        class != 1
    }
}

/// Translated from 0061a7d0 / 0061af30 (decompiled, FalloutNV.exe
/// 1.4.0.525): the line said out of those that can be, in order. The
/// first one is said unless it's flagged [`RANDOM`]; consecutive random
/// lines are gathered (up to and including one flagged [`RANDOM_END`], or
/// until a line that isn't random) and one of them is chosen with `roll`
/// (`rand() % count`).
pub fn choose(available: impl IntoIterator<Item = Info>, roll: u64) -> Option<Info> {
    let mut randoms: Vec<Info> = Vec::new();
    for info in available {
        let last = info.flags & RANDOM_END != 0;
        if info.flags & RANDOM == 0 {
            if randoms.is_empty() {
                return Some(info);
            }
            break;
        }
        randoms.push(info);
        if last {
            break;
        }
    }
    if randoms.is_empty() {
        return None;
    }
    let i = (roll % randoms.len() as u64) as usize;
    Some(randoms.swap_remove(i))
}

/// The line a speaker goes straight on to after `info` (translated from
/// 0061af30, decompiled, FalloutNV.exe 1.4.0.525; the Xbox prototype calls
/// it `TESTopic::GetMatchingFollowUpInfo` (Xbox PDB)): of the line's
/// follow-ups (`TCFU`) in order, those whose quest is running and whose
/// quest conditions and own conditions pass (as [`line_available`]),
/// chosen as [`choose`] says with the state's dice value.
pub fn follow_up(
    order: &LoadOrder,
    info: &Info,
    speaker: &Speaker,
    state: &GameState,
) -> Option<Info> {
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    let available = info.follow_ups.iter().filter_map(|&id| {
        let rr = order.get(id)?;
        if rr.entry.header.is_deleted() {
            return None;
        }
        let next = Info::load(order, id)?;
        // `0061af30`: the line's quest (`INFO+0x48`) must be set and
        // running (`00455620`), and its conditions pass.
        let quest = next.quest?;
        if !state.running.contains(&quest)
            || !facts.conditions_pass(
                &crate::quest::quest_conditions(order, quest),
                speaker.reference,
                PLAYER_REF,
            )
        {
            return None;
        }
        line_available(order, &next, speaker, PLAYER_REF, state).then_some(next)
    });
    choose(available, state.dice)
}

/// How the dialogue menu ends a conversation after a line (the menu's
/// `+0x2c`, set as each line starts by `00762860`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// Topics follow.
    Continue,
    /// The line answers the `GOODBYE` topic (`+0x2c` 1): follow-ups are
    /// still looked for, then the menu closes.
    GoodbyeTopic,
    /// The line is flagged [`GOODBYE`] (`+0x2c` 2): no follow-ups; the
    /// menu closes and its second result script runs as it does
    /// (`00762160`).
    GoodbyeLine,
}

/// Translated from 00762860 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn ending(info: &Info) -> Ending {
    if info.flags & GOODBYE != 0 {
        Ending::GoodbyeLine
    } else if info.topic == Some(GOODBYE_TOPIC) {
        Ending::GoodbyeTopic
    } else {
        Ending::Continue
    }
}

/// Whether the dialogue menu runs a line's second result script once it's
/// said (`00762ff0`, `00762160`: not for [`RUN_IMMEDIATELY`] lines).
pub fn menu_runs_end_script(info: &Info) -> bool {
    info.flags & RUN_IMMEDIATELY == 0
}

/// Whether the dialogue menu runs a line's first result script as it
/// starts (`0083ebb0`: not for [`NO_BEGIN_SCRIPT_IN_MENU`] lines).
pub fn menu_runs_begin_script(info: &Info) -> bool {
    info.flags & NO_BEGIN_SCRIPT_IN_MENU == 0
}

/// What the dialogue menu does once a line has been said and its second
/// result script has run.
#[derive(Debug, Clone, PartialEq)]
pub enum AfterLine {
    /// The speaker goes straight on with this line.
    FollowUp(Box<Info>),
    /// The player chooses among these.
    Topics(Vec<Choice>),
    /// The menu closes.
    Close,
}

/// Translated from 00762ff0 (decompiled, FalloutNV.exe 1.4.0.525), the
/// part after a line's last response: unless the line is a Goodbye line
/// or [`RUN_IMMEDIATELY`], its follow-up ([`follow_up`]) is said next;
/// otherwise a line answering `GOODBYE` closes the menu, else the topics
/// (`0083ec30`, here [`next_choices`]) are offered, and with none the menu
/// closes (`+0x2c` 3, "Invalid choice list encountered").
pub fn after_line(
    order: &LoadOrder,
    info: &Info,
    top_level: &[Topic],
    speaker: &Speaker,
    state: &GameState,
) -> AfterLine {
    let ending = ending(info);
    if ending != Ending::GoodbyeLine && menu_runs_end_script(info) {
        if let Some(next) = follow_up(order, info, speaker, state) {
            return AfterLine::FollowUp(Box::new(next));
        }
    }
    if ending != Ending::Continue {
        return AfterLine::Close;
    }
    let list = next_choices(order, info, top_level, speaker, state);
    if list.is_empty() {
        AfterLine::Close
    } else {
        AfterLine::Topics(list)
    }
}

/// The quest and topic parts of a voice file's name: whole when together
/// they're at most 25 letters; else, with a quest of at most 10 letters,
/// the quest whole and the topic cut to the rest of the 25; with a longer
/// quest, the quest's first 10 and the topic's first 15
/// (`vfreeformgoodsprings_hit_00126f2b_1.ogg`, Sunny Smiles' hurt line,
/// but `vfreeformg_greeting_00107220_1.ogg`; Doc Mitchell's psych test
/// answers, quest `VCG01`, are `vcg01_vcg01docmitchelltopi_…`).
// Translated from 006172c0 (decompiled, FalloutNV.exe 1.4.0.525; called by
// the response's file name builder 00617400, "%s_%08X_%u"): strlen of
// both; over 0x19, the topic is cut at 0x19 - quest length when the quest
// is shorter than 0xb, else the quest at 10 and the topic at 0xf.
pub fn voice_name_parts(quest: &str, topic: &str) -> (String, String) {
    // Byte lengths, as strlen counts; cut on a character boundary.
    fn cut(s: &mut String, mut at: usize) {
        while !s.is_char_boundary(at) {
            at -= 1;
        }
        s.truncate(at);
    }
    let (mut q, mut t) = (quest.to_ascii_lowercase(), topic.to_ascii_lowercase());
    if q.len() + t.len() > 25 {
        let topic_len = if q.len() < 11 {
            25 - q.len()
        } else {
            cut(&mut q, 10);
            15
        };
        cut(&mut t, topic_len);
    }
    (q, t)
}

/// The voice file for a response: `sound\voice\<plugin>\<voice type>\
/// <quest>_<topic>_<line's form ID>_<response>.ogg` (the quest and topic
/// cut as [`voice_name_parts`] says: `vfreeformg_greeting_00107220_1.ogg`
/// for Doc Mitchell's line 00107220 of `VFreeformGoodsprings`, topic
/// `GREETING`). The form ID's load-order byte is the line's own plugin's
/// index among its masters: 00 for `FalloutNV.esm`.
pub fn voice_path(
    order: &LoadOrder,
    info: &Info,
    response: &Response,
    voice: FormId,
) -> Option<String> {
    let edid = |id: Option<FormId>| -> Option<String> {
        order
            .get(id?)?
            .editor_id()
            .ok()?
            .map(|e| e.to_ascii_lowercase())
    };
    let voice = line_speaker_voice(order, info).unwrap_or(voice);
    let voice_name = edid(Some(voice))?;
    let (quest, topic) = voice_name_parts(&edid(info.quest)?, &edid(info.topic)?);
    let rr = order.get(info.form_id)?;
    let local = rr.entry.header.form_id.0 & 0x00FF_FFFF;
    Some(format!(
        "sound\\voice\\{}\\{voice_name}\\{quest}_{topic}_{local:08x}_{}.ogg",
        rr.plugin.name.to_ascii_lowercase(),
        response.number
    ))
}

/// The voice type of a line's own speaker (`ANAM`, `TESTopicInfo::pSpeaker`
/// (Xbox PDB), +0x3c here: `00639b40`), when the line names one.
// Translated from 00616fa0 (decompiled, FalloutNV.exe 1.4.0.525): the voice
// file's voice type is the line's own speaker's when it has one; only
// without one does the speaking reference's base give it (an NPC's or
// creature's voice type, a talking activator's `VNAM` via `009185e0`).
// Dead Money's narrator (`NVDLC01Narrator`, voice type
// `MaleAdult01Default`) says intro lines whose speaker is Elijah, so they
// play from Elijah's voice type folder.
fn line_speaker_voice(order: &LoadOrder, info: &Info) -> Option<FormId> {
    let rr = order.get(info.form_id)?;
    let record = rr.record().ok()?;
    let speaker = record
        .get(FourCC::new(b"ANAM"))
        .filter(|s| s.data.len() >= 4)
        .map(|s| global(&rr, &s.data))
        .filter(|id| id.0 != 0)?;
    let srr = order.get(speaker)?;
    let srec = srr.record().ok()?;
    srec.get(VTCK)
        .filter(|s| s.data.len() >= 4)
        .map(|s| global(&srr, &s.data))
        .filter(|id| id.0 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `0083d8b0`: notes in braces aren't shown (the game's lines as
    /// written: Cass's `Regenerating`, `FollowersTacticsCombatAggressive`).
    #[test]
    fn notes_left_out() {
        assert_eq!(
            without_notes("{Rush of health}MMmmahh. <Smacks lips>"),
            "MMmmahh. <Smacks lips>"
        );
        assert_eq!(
            without_notes("{Eager}I'll do like I'm doing then - {slight evil}except I'll try."),
            "I'll do like I'm doing then - except I'll try."
        );
        assert_eq!(without_notes("a{b{c}d}e}f"), "aef");
        assert_eq!(without_notes("No notes."), "No notes.");
    }

    fn speaker() -> Speaker {
        Speaker {
            reference: FormId(0x100),
            base: FormId(0x200),
            name: None,
            voice: Some(FormId(0x300)),
            race: None,
            female: false,
            factions: vec![FormId(0x400)],
        }
    }

    fn cond(function: u16, form: u32, comparison: Comparison, value: f32, or: bool) -> Condition {
        Condition {
            comparison,
            or,
            value,
            global: None,
            function,
            params: [form, 0],
            param_forms: [FormId(form), FormId(0)],
            run_on: 0,
            reference: None,
        }
    }

    fn info(conditions: Vec<Condition>) -> Info {
        Info {
            form_id: FormId(1),
            topic: None,
            quest: None,
            previous: None,
            flags: 0,
            flags2: 0,
            responses: Vec::new(),
            conditions,
            prompt: None,
            check: None,
            choices: Vec::new(),
            add_topics: Vec::new(),
            follow_ups: Vec::new(),
            begin_script: None,
            end_script: None,
        }
    }

    /// A load order to ask in (the conditions here don't read it).
    fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
        let data = testdata::quests(tag);
        let order =
            LoadOrder::from_data_dir(data.path(), &esm::ActivePlugins::OfficialOnly).unwrap();
        (data, order)
    }

    #[test]
    fn lines_are_picked_by_who_speaks_and_quest_stages() {
        let (_data, order) = order("dialogue-unit-pick");
        let state = GameState {
            stages: [(FormId(0x500), 20)].into(),
            ..GameState::default()
        };
        let s = speaker();
        let passes = |i: &Info| passes(&order, i, &s, &state);
        let mine = info(vec![cond(
            functions::GET_IS_ID,
            0x200,
            Comparison::Equal,
            1.0,
            false,
        )]);
        assert!(passes(&mine));
        let other = info(vec![cond(
            functions::GET_IS_ID,
            0x999,
            Comparison::Equal,
            1.0,
            false,
        )]);
        assert!(!passes(&other));
        let early = info(vec![cond(
            functions::GET_STAGE,
            0x500,
            Comparison::Less,
            10.0,
            false,
        )]);
        assert!(!passes(&early));
        // Functions not carried out (GetDistance) give 0.
        let unknown = info(vec![cond(1, 0x500, Comparison::Equal, 0.0, false)]);
        assert!(passes(&unknown));
    }

    #[test]
    fn or_binds_before_and() {
        let (_data, order) = order("dialogue-unit-or");
        let s = speaker();
        let state = GameState::default();
        let passes = |i: &Info| passes(&order, i, &s, &state);
        // (wrong id OR my faction) AND my voice: passes.
        let ok = info(vec![
            cond(functions::GET_IS_ID, 0x999, Comparison::Equal, 1.0, true),
            cond(
                functions::GET_IN_FACTION,
                0x400,
                Comparison::Equal,
                1.0,
                false,
            ),
            cond(
                functions::GET_IS_VOICE_TYPE,
                0x300,
                Comparison::Equal,
                1.0,
                false,
            ),
        ]);
        assert!(passes(&ok));
        // (wrong id OR wrong faction) AND my voice: fails.
        let no = info(vec![
            cond(functions::GET_IS_ID, 0x999, Comparison::Equal, 1.0, true),
            cond(
                functions::GET_IN_FACTION,
                0x998,
                Comparison::Equal,
                1.0,
                false,
            ),
            cond(
                functions::GET_IS_VOICE_TYPE,
                0x300,
                Comparison::Equal,
                1.0,
                false,
            ),
        ]);
        assert!(!passes(&no));
    }

    #[test]
    fn comparisons_follow_the_top_bits() {
        let c = |kind: u8| Condition {
            comparison: match kind >> 5 {
                0 => Comparison::Equal,
                2 => Comparison::Greater,
                _ => Comparison::LessOrEqual,
            },
            or: kind & 1 != 0,
            value: 3.0,
            global: None,
            function: 58,
            params: [0, 0],
            param_forms: [FormId(0); 2],
            run_on: 0,
            reference: None,
        };
        assert!(c(0x00).compare(3.0, 3.0));
        assert!(c(0x40).compare(4.0, 3.0) && !c(0x40).compare(3.0, 3.0));
        assert_eq!(c(0x01).function_name(), "GetStage");
    }
}
