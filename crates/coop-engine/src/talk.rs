//! Conversations (docs/coop/SPECS.md S14): what the player's own conversation says and does is
//! seen where the game does it, and its results run once.
//!
//! Every result of a line of dialogue goes through `TESTopicInfo::RunResult` (`0061f170`: the
//! info, index 0 when the line is said and 1 when it ends, the speaker). Co-op wraps it as
//! [`Owner::Coop`]. While a session shares conversations ([`set_sharing`]) and the call belongs
//! to the player's own conversation (the dialogue menu is open on that speaker):
//!
//! - every call is noted as an [`Event::Said`], so the session can tell the other players;
//! - on a guest, a line of the shared story (its quest is not one of the player's own) keeps its
//!   place in the guest's menu, but its result script does not run there: the call goes on inside
//!   the apply scope, and the host's game runs the result instead ([`enact`]). A line of one of
//!   the player's own quests (the opening), or one whose quest cannot be told, runs on the guest
//!   as it always did.
//!
//! `DialogMenu::Create` (`00761a20`) is wrapped too: an NPC another player is talking to
//! ([`set_busy`]) gets no menu (the answer 0, the game's own for a conversation that could not
//! start), and the refusal is noted as an [`Event::Blocked`].
//!
//! With sharing off, both wraps pass every call on untouched.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError, RwLock};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr, forms, gate};

/// Co-op's place in front of `TESTopicInfo::RunResult`.
pub(crate) const RUN_RESULT: WrapSlot = WrapSlot::new(Owner::Coop, addr::INFO_RUN_RESULT);
/// Co-op's place in front of `DialogMenu::Create`.
pub(crate) const CREATE: WrapSlot = WrapSlot::new(Owner::Coop, addr::DIALOG_MENU_CREATE);

/// The most events kept between two takes (a conversation makes a few per line).
const EVENTS_MAX: usize = 256;
/// How long a message stays on the HUD, in seconds (the game's own messages use 2.0).
const MESSAGE_SECONDS: f32 = 4.0;

/// What happened in this game's own conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// The game ran a line's result: the speaker's reference id, the info's form id, whether it
    /// was the line's end (index 1), and whether the result was held back here for the host to
    /// run (a guest, a line of the shared story).
    Said {
        /// The speaker's reference id.
        npc: u32,
        /// The info's form id.
        info: u32,
        /// False when the line is said, true when it ends.
        end: bool,
        /// The result script did not run here: the host is to run it.
        held: bool,
    },
    /// A conversation with this reference was refused: another player is talking to it.
    Blocked {
        /// The speaker's reference id.
        npc: u32,
    },
}

/// The game-wide state (a lock-free shared slot, as the journal's).
#[derive(Debug, Default)]
pub struct Talk {
    sharing: AtomicBool,
    /// How deep the host is in running a guest's result: those calls are not this game's own
    /// conversation.
    enacting: AtomicU32,
    busy: RwLock<BTreeSet<u32>>,
    events: Mutex<Vec<Event>>,
    held: AtomicU64,
    own: AtomicU64,
    unknown: AtomicU64,
}

impl Talk {
    fn push(&self, ev: Event) {
        let mut q = self.events.lock().unwrap_or_else(PoisonError::into_inner);
        if q.len() < EVENTS_MAX {
            q.push(ev);
        }
    }
}

/// Starts or stops sharing conversations: with it off the wraps change nothing.
pub fn set_sharing(e: &mut Engine, on: bool) {
    e.ext_free::<Talk>().sharing.store(on, Ordering::SeqCst);
}

/// Whether conversations are shared.
pub fn sharing(e: &mut Engine) -> bool {
    e.ext_free::<Talk>().sharing.load(Ordering::SeqCst)
}

/// Names the references other players are talking to: a conversation with one of them does not
/// start here.
pub fn set_busy(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    let t = e.ext_free::<Talk>();
    *t.busy.write().unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

/// The events since the last call, in order.
pub fn take_events(e: &mut Engine) -> Vec<Event> {
    let t = e.ext_free::<Talk>();
    std::mem::take(&mut *t.events.lock().unwrap_or_else(PoisonError::into_inner))
}

/// `(held for the host, run here as the player's own, run here with no quest known)`: how the
/// results of this game's own conversations were classed since the game started.
pub fn counts(e: &mut Engine) -> (u64, u64, u64) {
    let t = e.ext_free::<Talk>();
    (
        t.held.load(Ordering::SeqCst),
        t.own.load(Ordering::SeqCst),
        t.unknown.load(Ordering::SeqCst),
    )
}

/// The reference id of the speaker of the open dialogue menu; `None` with no menu open.
pub fn menu_npc(e: &mut Engine) -> Option<u32> {
    let speaker = menu_speaker(e)?;
    Some(e.mem.u32(speaker.wrapping_add(addr::FORM_ID_OFFSET)))
}

fn menu_speaker(e: &mut Engine) -> Option<u32> {
    let menu = e.mem.u32(addr::DIALOG_MENU_OPEN);
    if menu == 0 {
        return None;
    }
    Some(
        e.mem
            .u32(menu.wrapping_add(addr::DIALOG_MENU_SPEAKER_OFFSET)),
    )
    .filter(|s| *s != 0)
}

/// Ends the open conversation through the game's own `DialogMenu::Close`; false with none open.
pub fn close_menu(e: &mut Engine) -> bool {
    if menu_speaker(e).is_none() {
        return false;
    }
    e.call(addr::DIALOG_MENU_CLOSE, &[]);
    true
}

/// The info `id`, or `None` when the data lack it or the form is not an info.
fn info(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    (e.mem.u8(f + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_INFO).then_some(f)
}

/// The form id of the quest the info at `info` belongs to: the quest its topic holds it under,
/// else the info's own owner quest; 0 when neither is known.
fn quest_of(e: &mut Engine, info: u32) -> u32 {
    let topic = e.call(addr::TOPIC_GET_OWNER_TOPIC, &args![info]).u32();
    let mut quest = match topic {
        0 => 0,
        t => e.call(addr::TOPIC_INFO_QUEST, &args![t, info]).u32(),
    };
    if quest == 0 {
        quest = e.mem.u32(info.wrapping_add(addr::INFO_OWNER_QUEST_OFFSET));
    }
    match quest {
        0 => 0,
        q => e.mem.u32(q.wrapping_add(addr::FORM_ID_OFFSET)),
    }
}

/// The form id of the quest info `id` belongs to (`Some(0)`: an info with no quest known; `None`:
/// not an info of this game's data).
pub fn info_quest(e: &mut Engine, id: u32) -> Option<u32> {
    let f = info(e, id)?;
    Some(quest_of(e, f))
}

/// Why the host's game did not run a guest's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The data have no such info.
    NoInfo(u32),
    /// The speaker is not in this game now (its cell is not loaded), or is not an actor.
    NoSpeaker(u32),
}

/// Runs info `info`'s result on the reference `npc` in this game (the host's), through the game's
/// own `TESTopicInfo::RunResult`: `end` false for the result of the line being said, true for its
/// end. The call is not taken for this game's own conversation.
pub fn enact(e: &mut Engine, npc: u32, info_id: u32, end: bool) -> Result<(), Refusal> {
    let f = info(e, info_id).ok_or(Refusal::NoInfo(info_id))?;
    let speaker = actors::actor(e, npc).ok_or(Refusal::NoSpeaker(npc))?;
    e.ext_free::<Talk>().enacting.fetch_add(1, Ordering::SeqCst);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.call(addr::INFO_RUN_RESULT, &args![f, u32::from(end), speaker]);
    }));
    e.ext_free::<Talk>().enacting.fetch_sub(1, Ordering::SeqCst);
    match result {
        Ok(()) => Ok(()),
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// The text of info `id`'s responses as the game builds it (`" | "` between two responses), and
/// its prompt (what the player chose) when the info has one. `None`: not an info.
pub fn line(e: &mut Engine, id: u32) -> Option<(String, String)> {
    let f = info(e, id)?;
    let string = e.call(addr::OPERATOR_NEW, &args![addr::STRING_SIZE]).u32();
    if string == 0 {
        return None;
    }
    e.call(addr::STRING_CONSTRUCT, &args![string]);
    e.call(addr::INFO_GET_DISPLAY_TEXT, &args![f, string, 0u32]);
    let text = text_at(e, e.mem.u32(string));
    e.call(addr::STRING_DESTROY, &args![string]);
    e.call(addr::OPERATOR_DELETE, &args![string]);
    let prompt = text_at(e, e.mem.u32(f + addr::INFO_PROMPT_OFFSET));
    Some((text, prompt))
}

fn text_at(e: &Engine, chars: u32) -> String {
    if chars == 0 {
        return String::new();
    }
    // The game's text is one byte per character (Windows-1252): each byte is that code point,
    // near enough for a message.
    e.mem.cstr(chars).into_iter().map(char::from).collect()
}

/// Shows `text` as one of the game's own HUD messages (cut to what a message holds). False when
/// the game gave no memory for it.
pub fn show(e: &mut Engine, text: &str) -> bool {
    let mut bytes: Vec<u8> = text
        .chars()
        .map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' })
        .filter(|b| *b != 0)
        .collect();
    bytes.truncate(addr::HUD_MESSAGE_MAX);
    let buffer = e
        .call(addr::OPERATOR_NEW, &args![bytes.len() as u32 + 1])
        .u32();
    if buffer == 0 {
        return false;
    }
    e.mem.set_cstr(buffer, &bytes);
    e.call(
        addr::SHOW_HUD_MESSAGE,
        &args![buffer, 0u32, 0u32, 0u32, MESSAGE_SECONDS, 0u32],
    );
    e.call(addr::OPERATOR_DELETE, &args![buffer]);
    true
}

/// Whether reference `npc` is in this game now and in the cell the player is in.
pub fn near(e: &mut Engine, npc: u32) -> bool {
    let (Some(player), Some(r)) = (crate::live::player(e), forms::form(e, npc)) else {
        return false;
    };
    let cell = e.mem.u32(player + addr::REF_CELL_OFFSET);
    cell != 0 && e.mem.u32(r + addr::REF_CELL_OFFSET) == cell
}

/// The wrap at `0061f170`. Its words: the info, the index, the reference.
pub(crate) fn run_result(e: &mut Engine, a: &[u32]) -> Ret {
    let t = e.ext_free::<Talk>();
    if !t.sharing.load(Ordering::SeqCst) || t.enacting.load(Ordering::SeqCst) > 0 {
        return e.call_next(addr::INFO_RUN_RESULT, a);
    }
    let word = |i: usize| a.get(i).copied().unwrap_or(0);
    let (info, index, speaker) = (word(0), word(1), word(2));
    // Only the player's own conversation: the open menu's speaker. Other lines (NPCs talking to
    // each other, combat shouts) are each game's own.
    if info == 0 || speaker == 0 || menu_speaker(e) != Some(speaker) {
        return e.call_next(addr::INFO_RUN_RESULT, a);
    }
    let npc = e.mem.u32(speaker.wrapping_add(addr::FORM_ID_OFFSET));
    let id = e.mem.u32(info.wrapping_add(addr::FORM_ID_OFFSET));
    let quest = quest_of(e, info);
    let held = gate::is_guest(e) && quest != 0 && !gate::is_own_quest(e, quest);
    let t = e.ext_free::<Talk>();
    match (held, quest) {
        (true, _) => t.held.fetch_add(1, Ordering::SeqCst),
        (false, 0) => t.unknown.fetch_add(1, Ordering::SeqCst),
        (false, _) => t.own.fetch_add(1, Ordering::SeqCst),
    };
    t.push(Event::Said {
        npc,
        info: id,
        end: index != 0,
        held,
    });
    if held {
        gate::applying(e, |e| e.call_next(addr::INFO_RUN_RESULT, a))
    } else {
        e.call_next(addr::INFO_RUN_RESULT, a)
    }
}

/// The wrap at `00761a20`. Its words: the speaker, a word for the topic manager, a target.
pub(crate) fn create(e: &mut Engine, a: &[u32]) -> Ret {
    let speaker = a.first().copied().unwrap_or(0);
    let t = e.ext_free::<Talk>();
    if speaker != 0 && t.sharing.load(Ordering::SeqCst) && menu_speaker(e).is_none() {
        let npc = e.mem.u32(speaker.wrapping_add(addr::FORM_ID_OFFSET));
        let t = e.ext_free::<Talk>();
        if t.busy
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .contains(&npc)
        {
            t.push(Event::Blocked { npc });
            return Ret::default();
        }
    }
    e.call_next(addr::DIALOG_MENU_CREATE, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const MQ: u32 = 0x0010_4c0c;
    const OWN: u32 = 0x0010_4c1c;
    const NPC: u32 = 0x0010_4f25;
    const OTHER: u32 = 0x0010_4e85;
    const CELL: u32 = 0x0010_5a00;
    /// A line of the shared story that sets MQ's stage 10, one of the player's own quest that
    /// sets OWN's stage 5, and one with no quest.
    const SHARED_LINE: u32 = 0x0010_9001;
    const OWN_LINE: u32 = 0x0010_9002;
    const LOOSE_LINE: u32 = 0x0010_9003;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            quests: vec![(MQ, vec![5, 10, 20]), (OWN, vec![5])],
            cells: vec![CELL],
            actors: vec![NPC, OTHER],
            ..StorySpec::default()
        });
        s.add_info(
            SHARED_LINE,
            MQ,
            "Head on up to the saloon.",
            "Where is Sunny?",
        );
        s.set_info_result(SHARED_LINE, false, MQ, 10);
        s.set_info_result(SHARED_LINE, true, MQ, 20);
        s.add_info(OWN_LINE, OWN, "You have been out cold.", "");
        s.set_info_result(OWN_LINE, false, OWN, 5);
        s.add_info(LOOSE_LINE, 0, "Howdy.", "");
        s.move_player(CELL, [0.0; 3], 0.0);
        s.place(NPC, CELL);
        s
    }

    #[test]
    fn with_sharing_off_a_conversation_is_the_games_own() {
        let mut s = story();
        gate::set_guest(&mut s.e, true);
        assert!(s.talk_to(NPC));
        s.say(SHARED_LINE, false);
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(10));
        assert!(take_events(&mut s.e).is_empty());
        assert_eq!(counts(&mut s.e), (0, 0, 0));
    }

    #[test]
    fn a_host_notes_its_lines_and_runs_their_results() {
        let mut s = story();
        set_sharing(&mut s.e, true);
        assert_eq!(menu_npc(&mut s.e), None);
        assert!(s.talk_to(NPC));
        assert_eq!(menu_npc(&mut s.e), Some(NPC));
        s.say(SHARED_LINE, false);
        s.say(SHARED_LINE, true);
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(20));
        assert_eq!(
            take_events(&mut s.e),
            [false, true].map(|end| Event::Said {
                npc: NPC,
                info: SHARED_LINE,
                end,
                held: false
            })
        );
        assert!(take_events(&mut s.e).is_empty());
    }

    #[test]
    fn a_guest_holds_the_shared_story_s_results_and_runs_its_own() {
        let mut s = story();
        gate::set_guest(&mut s.e, true);
        gate::set_own_quests(&mut s.e, [OWN]);
        set_sharing(&mut s.e, true);
        assert!(s.talk_to(NPC));
        s.say(SHARED_LINE, false);
        s.say(OWN_LINE, false);
        s.say(LOOSE_LINE, false);
        // The shared line's result did not run here; the menu's bookkeeping did (the line is
        // said), the player's own quest moved, the loose line ran as before.
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(0));
        assert_eq!(forms::quest_stage(&mut s.e, OWN), Some(5));
        assert_eq!(s.said(SHARED_LINE), 1);
        assert_eq!(counts(&mut s.e), (1, 1, 1));
        let held: Vec<bool> = take_events(&mut s.e)
            .into_iter()
            .map(|ev| matches!(ev, Event::Said { held: true, .. }))
            .collect();
        assert_eq!(held, [true, false, false]);
        // Twin: the same line with the wrap passing it on runs the result on the guest.
        set_sharing(&mut s.e, false);
        s.say(SHARED_LINE, false);
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(10));
    }

    #[test]
    fn a_line_outside_the_players_conversation_is_left_alone() {
        let mut s = story();
        gate::set_guest(&mut s.e, true);
        set_sharing(&mut s.e, true);
        // No menu open: an NPC's own line.
        s.say_as(NPC, SHARED_LINE, false);
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(10));
        // A menu open on one NPC, a line of another.
        assert!(s.talk_to(NPC));
        s.say_as(OTHER, SHARED_LINE, true);
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(20));
        assert!(take_events(&mut s.e).is_empty());
    }

    #[test]
    fn the_host_runs_a_guests_result_without_taking_it_for_its_own_conversation() {
        let mut s = story();
        set_sharing(&mut s.e, true);
        // The host is itself talking to that NPC's neighbour.
        s.place(OTHER, CELL);
        assert!(s.talk_to(OTHER));
        assert_eq!(enact(&mut s.e, NPC, SHARED_LINE, false), Ok(()));
        assert_eq!(forms::quest_stage(&mut s.e, MQ), Some(10));
        assert!(take_events(&mut s.e).is_empty());
        assert_eq!(
            enact(&mut s.e, NPC, MQ, false),
            Err(Refusal::NoInfo(MQ)),
            "a quest is not an info"
        );
        assert_eq!(
            enact(&mut s.e, 0x0010_ffff, SHARED_LINE, false),
            Err(Refusal::NoSpeaker(0x0010_ffff))
        );
        assert_eq!(info_quest(&mut s.e, SHARED_LINE), Some(MQ));
        assert_eq!(info_quest(&mut s.e, LOOSE_LINE), Some(0));
        assert_eq!(info_quest(&mut s.e, NPC), None);
    }

    #[test]
    fn an_npc_another_player_talks_to_gets_no_menu() {
        let mut s = story();
        set_sharing(&mut s.e, true);
        set_busy(&mut s.e, [NPC]);
        assert!(!s.talk_to(NPC));
        assert_eq!(menu_npc(&mut s.e), None);
        assert_eq!(take_events(&mut s.e), [Event::Blocked { npc: NPC }]);
        set_busy(&mut s.e, []);
        assert!(s.talk_to(NPC));
        assert!(close_menu(&mut s.e));
        assert_eq!(menu_npc(&mut s.e), None);
        assert!(!close_menu(&mut s.e));
        // Twin: with sharing off the busy list is not consulted.
        set_sharing(&mut s.e, false);
        set_busy(&mut s.e, [NPC]);
        assert!(s.talk_to(NPC));
    }

    #[test]
    fn a_line_and_its_prompt_are_read_and_shown_through_the_game() {
        let mut s = story();
        assert_eq!(
            line(&mut s.e, SHARED_LINE),
            Some((
                "Head on up to the saloon.".to_string(),
                "Where is Sunny?".to_string()
            ))
        );
        assert_eq!(
            line(&mut s.e, LOOSE_LINE),
            Some(("Howdy.".to_string(), String::new()))
        );
        assert_eq!(line(&mut s.e, MQ), None);
        assert!(show(&mut s.e, "Howdy."));
        let long = "x".repeat(600);
        assert!(show(&mut s.e, &long));
        let shown = s.messages();
        assert_eq!(shown[0], "Howdy.");
        assert_eq!(shown[1].len(), addr::HUD_MESSAGE_MAX);
        assert!(near(&mut s.e, NPC));
        assert!(!near(&mut s.e, OTHER));
    }
}
