//! Someone's body while they talk to the player: the idle requests the
//! game makes for the speaker in the dialogue menu.
//!
//! Read from `FalloutNV.exe` 1.4.0.525 (names marked (Xbox PDB) come from
//! the prototype's symbols):
//!
//! - The dialogue menu updates the speaker every frame
//!   (`Actor::UpdateInDialogue`, Xbox PDB, `008a5580`). With a response to
//!   say it stores the response's `bUseEmotion` (actor +0x86, `008a5cf0`)
//!   and calls the say function (Actor vtable +0x284, `008a20d0`) with the
//!   response's emotion, its speaker and listener idles
//!   (`DialogueResponse` +0x18/+0x1c, Xbox PDB) and a last argument of
//!   one. The say sets the speaking emotion (vfuncs +0x2d4, +0x2dc) and,
//!   since that last argument is set, asks for an idle: the response's
//!   speaker idle (request mode 3), else the idle tree (mode 2), even
//!   while another special idle plays (the request's "force" argument).
//! - After that, every frame while the speaker runs their dialogue
//!   package (type 0x1c): once their special idle is done (`004985f0`)
//!   and no request waits (process +0x350, vfunc +0x718), the tree is
//!   asked again (mode 2, not forced).
//! - A request (`008dab40`, process vfunc +0x44) is refused unless the
//!   actor's sit state is 0, 4 or 9 (standing, seated, asleep), and
//!   unless their special idle is done, an idle is named or it is forced.
//!   With no idle named the tree is asked (`00600950`) when no special
//!   idle is starting (`00498f80`).
//! - Leaving the menu (`Actor::EndDialogue`, Xbox PDB, `008b1070`) sets
//!   process flag 0x800 for the speaker in sit state 0, 4 or 9; the flags
//!   update (`008ba600`) carries it out once no special idle is starting:
//!   `008daf20` frees the special idle (`00498910(1,0)`, its blend out).
//!
//! The tree's own answer (the `DialogueIdles` / `TalkToPlayer` branch,
//! `ChairDialogueIdles` for the seated) asks `MenuMode 1009`,
//! `IsTalking`, `GetDialogueEmotion` (`005a4480`: the speaking emotion
//! when `bUseEmotion`, else −1) and `GetRandomPercent`: see
//! [`crate::idles::IdleQuestion`].

use esm::FormId;

use crate::dialogue::Response;

/// What an idle request asks for (`008dab40`'s idle and mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Request {
    /// A named idle (mode 3): the response's speaker idle.
    Idle(FormId),
    /// Whatever the idle tree gives now (mode 2).
    Tree,
}

/// A request and whether it may replace a special idle still playing
/// (the say forces it; the menu's frame-by-frame ask doesn't).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ask {
    pub request: Request,
    pub forced: bool,
}

/// The response said, as the menu tells it apart from the one before: the
/// line, the response number, and when it began.
pub type SaidKey = (FormId, usize, f32);

/// One actor's talking state, as the say and the dialogue update keep it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Talking {
    /// The speaking emotion and its value (`Actor::SetSpeakingEmotion`,
    /// Xbox PDB; vfuncs +0x2d4/+0x2dc) and the response's `bUseEmotion`
    /// (actor +0x86).
    pub emotion: u32,
    pub emotion_value: i32,
    pub use_emotion: bool,
    /// The response last said, so each response is said once.
    said: Option<SaidKey>,
    /// Leaving the menu asked for the special idle to be freed (process
    /// flag 0x800), not yet carried out.
    pub free_pending: bool,
    /// An idle taken but not yet playing (process +0x350): the process
    /// plays it once no special idle is starting (`008dae00`).
    pub queued: Option<FormId>,
}

impl Talking {
    /// `GetDialogueEmotion` (`005a4480`): the speaking emotion when the
    /// response said wanted it used, else none (−1).
    // Translated from 005a4480 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn dialogue_emotion(&self) -> Option<u32> {
        self.use_emotion.then_some(self.emotion)
    }

    /// The dialogue menu's update with a response to say: once per
    /// response, the emotion is taken and the speaker's idle request made
    /// (always, as the menu's say asks for one). None when this response
    /// was already said.
    // Translated from 008a5580 and 008a20d0 (decompiled, FalloutNV.exe
    // 1.4.0.525)
    pub fn say(&mut self, key: SaidKey, response: &Response) -> Option<Ask> {
        if self.said == Some(key) {
            return None;
        }
        self.said = Some(key);
        self.use_emotion = response.use_emotion;
        self.emotion = response.emotion;
        self.emotion_value = response.emotion_value;
        Some(Ask {
            request: response.speaker_idle.map_or(Request::Tree, Request::Idle),
            forced: true,
        })
    }

    /// The dialogue menu's update between says: the tree is asked again
    /// once the special idle is done and no request waits.
    // Translated from 008a5580 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn between_says(&self, special_idle_done: bool) -> Option<Ask> {
        (special_idle_done && self.queued.is_none()).then_some(Ask {
            request: Request::Tree,
            forced: false,
        })
    }

    /// The menu closed: the special idle is to be freed if the actor's sit
    /// state is 0, 4 or 9 (`008b1070` → flag 0x800 → `008daf20`).
    // Translated from 008b1070 and 008daf20 (decompiled, FalloutNV.exe
    // 1.4.0.525)
    pub fn menu_closed(&mut self, sit_state: u8) {
        self.said = None;
        // `008b1070` clears the waiting request (process vfunc +0x71c).
        self.queued = None;
        if matches!(sit_state, 0 | 4 | 9) {
            self.free_pending = true;
        }
    }
}

/// What a say asks of the speaker and the listener (`008a20d0`, the say
/// for any line, said in the menu or not).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SayRequests {
    pub speaker: Option<Ask>,
    pub listener: Option<Ask>,
}

/// The idle requests a say makes (`008a20d0`): the speaker asks for the
/// response's speaker idle, or the tree when its caller forces one
/// (`force_tree`, the say's last argument), forced unless the speaker is in
/// combat; a listener that is an actor with a process (not the player)
/// asks for the response's listener idle, or the tree unless its running
/// package has idles of its own (package +0x34, `009344a0` →
/// `005f36f0`), always forced.
// Translated from 008a20d0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn say_requests(
    response: &Response,
    force_tree: bool,
    speaker_in_combat: bool,
    listener_package_has_idles: Option<bool>,
) -> SayRequests {
    let speaker = (response.speaker_idle.is_some() || force_tree).then(|| Ask {
        request: response.speaker_idle.map_or(Request::Tree, Request::Idle),
        forced: !speaker_in_combat,
    });
    let listener = listener_package_has_idles.and_then(|has_idles| {
        (response.listener_idle.is_some() || !has_idles).then(|| Ask {
            request: response.listener_idle.map_or(Request::Tree, Request::Idle),
            forced: true,
        })
    });
    SayRequests { speaker, listener }
}

/// Whether the GREET procedure's says force a tree request (`008dbe30`'s
/// last argument): unless the current package has idles of its own or the
/// run-once package is of type 0x1a.
// Translated from 008dbe30 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn greet_forces_tree(package_has_idles: bool, run_once_kind: Option<u8>) -> bool {
    !package_has_idles && run_once_kind != Some(0x1a)
}

/// Whether a conversation's says force a tree request (its +0xa0, given
/// when it's made, `009edd80`): a script's `StartConversation` does
/// (`008b2170`); one a package starts does unless that package has idles
/// of its own (`008b19c0`, `00935480`).
pub fn conversation_forces_tree(package_has_idles: Option<bool>) -> bool {
    !package_has_idles.unwrap_or(false)
}

/// A conversation's update with no line said (the next one not due yet):
/// both sides ask the tree, forced (`009ee0a0`).
// Translated from 009ee0a0 (decompiled, FalloutNV.exe 1.4.0.525)
pub const BETWEEN_LINES: Ask = Ask {
    request: Request::Tree,
    forced: true,
};

/// Whether the running package refuses idle requests (`008dab40` →
/// `008dade0`: its flag 0x1000000, "no idle anims").
// Translated from 008dade0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn package_refuses_idles(package_flags: Option<u32>) -> bool {
    package_flags.is_some_and(|f| f & 0x100_0000 != 0)
}

/// Whether a package has idles of its own (`IDLA` with an `IDLC` count
/// above 0): what package +0x34 holds.
pub fn package_has_idles(order: &esm::LoadOrder, package: FormId) -> bool {
    let Some(rr) = order
        .get(package)
        .filter(|r| r.entry.header.kind == esm::FourCC::new(b"PACK"))
    else {
        return false;
    };
    rr.record().ok().is_some_and(|r| {
        let count = r
            .get(esm::FourCC::new(b"IDLC"))
            .and_then(|s| s.data.first().copied())
            .unwrap_or(0);
        count > 0 && r.get(esm::FourCC::new(b"IDLA")).is_some()
    })
}

/// Whether an idle request is taken (`008dab40`): sit state 0, 4 or 9,
/// and the special idle done, an idle named or the request forced; the
/// tree is only asked when no special idle is starting (`00498f80`).
/// (Its refusal for a running package with the "no idle anims" flag is
/// [`package_refuses_idles`]; a knocked-down or ragdolled actor, vfuncs
/// +0x230/+0x234, and the untraced `00437bf0` aren't modelled.)
// Translated from 008dab40 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn takes_request(
    sit_state: u8,
    special_idle_done: bool,
    special_starting: bool,
    ask: Ask,
) -> bool {
    if !matches!(sit_state, 0 | 4 | 9) {
        return false;
    }
    let named = matches!(ask.request, Request::Idle(_));
    if !(special_idle_done || named || ask.forced) {
        return false;
    }
    named || !special_starting
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(emotion: u32, use_emotion: bool, speaker_idle: Option<u32>) -> Response {
        Response {
            emotion,
            emotion_value: 50,
            number: 1,
            text: "Line.".into(),
            use_emotion,
            speaker_idle: speaker_idle.map(FormId),
            listener_idle: None,
            sound: None,
        }
    }

    #[test]
    fn each_response_is_said_once_with_its_idle_or_the_tree() {
        let mut t = Talking::default();
        assert_eq!(t.dialogue_emotion(), None);
        let happy = response(5, true, None);
        let ask = t.say((FormId(1), 0, 2.0), &happy).unwrap();
        assert_eq!(
            ask,
            Ask {
                request: Request::Tree,
                forced: true
            }
        );
        assert_eq!(t.dialogue_emotion(), Some(5));
        // The same response in later frames asks nothing more.
        assert_eq!(t.say((FormId(1), 0, 2.0), &happy), None);
        // The next one names its idle; its emotion isn't to be used.
        let named = response(1, false, Some(0x47CDC));
        let ask = t.say((FormId(1), 1, 5.0), &named).unwrap();
        assert_eq!(ask.request, Request::Idle(FormId(0x47CDC)));
        assert_eq!(t.dialogue_emotion(), None);
        assert_eq!(t.emotion, 1);
        // Said again later (a new start time): said again.
        assert!(t.say((FormId(1), 1, 9.0), &named).is_some());
    }

    #[test]
    fn between_says_the_tree_is_asked_once_the_idle_is_done() {
        let mut t = Talking::default();
        assert_eq!(t.between_says(false), None);
        assert_eq!(
            t.between_says(true),
            Some(Ask {
                request: Request::Tree,
                forced: false
            })
        );
        // Not while a taken idle waits to play.
        t.queued = Some(FormId(3));
        assert_eq!(t.between_says(true), None);
    }

    #[test]
    fn requests_follow_the_sit_state_and_the_special_idle() {
        let tree = Ask {
            request: Request::Tree,
            forced: false,
        };
        let forced = Ask {
            forced: true,
            ..tree
        };
        let named = Ask {
            request: Request::Idle(FormId(7)),
            forced: false,
        };
        // Standing, seated, asleep: yes; sitting down (3) or getting up
        // (5): no.
        for state in [0, 4, 9] {
            assert!(takes_request(state, true, false, tree));
        }
        for state in [1, 2, 3, 5, 10] {
            assert!(!takes_request(state, true, false, forced));
        }
        // An idle still playing: only a forced or named request.
        assert!(!takes_request(0, false, false, tree));
        assert!(takes_request(0, false, false, forced));
        assert!(takes_request(0, false, false, named));
        // One just starting: the tree isn't asked, a named idle still is.
        assert!(!takes_request(0, false, true, forced));
        assert!(takes_request(0, false, true, named));
    }

    #[test]
    fn a_say_asks_for_the_speaker_and_the_listener() {
        let mut r = response(0, false, None);
        // A greeting forcing the tree, the listener an NPC whose package
        // has no idles: both ask the tree, forced.
        let asks = say_requests(&r, true, false, Some(false));
        let tree = Ask {
            request: Request::Tree,
            forced: true,
        };
        assert_eq!(asks.speaker, Some(tree));
        assert_eq!(asks.listener, Some(tree));
        // In combat the speaker's isn't forced; to the player, no listener.
        let asks = say_requests(&r, true, true, None);
        assert_eq!(asks.speaker.map(|a| a.forced), Some(false));
        assert_eq!(asks.listener, None);
        // Not forcing, no idles named: nothing for the speaker; a listener
        // whose package has idles asks nothing either.
        assert_eq!(
            say_requests(&r, false, false, Some(true)),
            SayRequests::default()
        );
        // Named idles are asked for whatever else.
        r.speaker_idle = Some(FormId(5));
        r.listener_idle = Some(FormId(6));
        let asks = say_requests(&r, false, false, Some(true));
        assert_eq!(asks.speaker.unwrap().request, Request::Idle(FormId(5)));
        assert_eq!(asks.listener.unwrap().request, Request::Idle(FormId(6)));
        assert!(greet_forces_tree(false, None));
        assert!(!greet_forces_tree(true, None));
        assert!(!greet_forces_tree(false, Some(0x1a)));
        assert!(conversation_forces_tree(None));
        assert!(!conversation_forces_tree(Some(true)));
    }

    #[test]
    fn closing_the_menu_frees_the_idle_unless_in_transition() {
        let mut t = Talking::default();
        t.say((FormId(1), 0, 0.0), &response(0, false, None));
        t.menu_closed(4);
        assert!(t.free_pending);
        // Said again after the menu: a new conversation.
        assert!(t
            .say((FormId(1), 0, 0.0), &response(0, false, None))
            .is_some());
        let mut getting_up = Talking::default();
        getting_up.menu_closed(5);
        assert!(!getting_up.free_pending);
    }
}
