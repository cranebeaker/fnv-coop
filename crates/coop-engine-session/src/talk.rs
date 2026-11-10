//! Conversations in a session (docs/coop/SPECS.md S14): coop-core's dialogue protocol driven by
//! the game's own dialogue menu.
//!
//! - [`Talker`] runs on every machine. What this game's own conversation says
//!   (`coop_engine::talk`'s events) becomes the protocol's moves: the first shared line asks the
//!   host for the NPC (`Open`), each line said is a `Pick` of its info, its end a `LineDone`, the
//!   menu closing a `Close`. A machine that watches another player's conversation shows each
//!   line picked, through the game's own HUD message, when its player is in the NPC's cell, and
//!   reports the line done at once, so the talker never waits for a watcher. The NPC another
//!   player talks to is named busy to the game, which then opens no menu on it; a conversation
//!   the host refuses is closed through the game.
//! - [`Enactor`] runs on the host: a guest's pick of a line of the shared story has its result
//!   run in the host's game, once, when the protocol says so (the begin result at the pick, the
//!   end result once every machine has reported the line done).

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use coop_core::dialogue::{DialogueClient, DialogueEvent, DialogueMsg};
use coop_core::ids::{FormId, PlayerId};
use coop_engine::talk::{self, Event};
use engine_core::Engine;

/// A line of this game's conversation waiting for its turn in the protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Said {
    npc: u32,
    info: u32,
    end: bool,
}

/// One machine's part in the dialogue protocol.
#[derive(Debug, Default)]
pub struct Talker {
    queue: VecDeque<Said>,
    /// An `Open` sent and not answered: (request id, the NPC).
    pending: Option<(u32, u32)>,
    /// A `Pick` sent for this step and not confirmed.
    awaiting: Option<u32>,
    closing: bool,
    /// The watched line last shown: (session, step).
    shown: Option<(u32, u32)>,
    /// Conversations of this player the host opened.
    pub opened: u32,
    /// Conversations the host refused (the NPC was taken, or dead).
    pub denied: u32,
    /// Conversations the game did not start here because another player has the NPC.
    pub blocked: u32,
    /// Lines of this player's conversations sent as picks.
    pub picks: u32,
    /// Other players' lines shown here.
    pub lines_shown: u32,
    /// Other players' lines not shown: this player is not with the NPC.
    pub lines_far: u32,
}

/// How a watched line is shown: what the player chose, when the info has a prompt, then each of
/// the NPC's responses as a message of its own.
fn messages(initiator: PlayerId, text: &str, prompt: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !prompt.is_empty() {
        out.push(format!("Player {}: {prompt}", u32::from(initiator.0) + 1));
    }
    out.extend(
        text.split(" | ")
            .filter(|t| !t.trim().is_empty())
            .map(str::to_string),
    );
    out
}

impl Talker {
    /// One step. `share` says which of this game's lines are the shared story's (on a guest the
    /// ones the game held back for the host; on the host the ones of a shared quest). Returns the
    /// protocol messages to send to the host's dialogue table.
    pub fn step(
        &mut self,
        e: &mut Engine,
        me: PlayerId,
        client: &mut DialogueClient,
        share: &dyn Fn(&mut Engine, u32, bool) -> bool,
    ) -> Vec<DialogueMsg> {
        let mut out = Vec::new();
        for ev in talk::take_events(e) {
            match ev {
                Event::Said {
                    npc,
                    info,
                    end,
                    held,
                } => {
                    if share(e, info, held) {
                        self.queue.push_back(Said { npc, info, end });
                    }
                }
                Event::Blocked { .. } => {
                    self.blocked += 1;
                    talk::show(e, "Another player is talking to them.");
                }
            }
        }
        // The NPC another player talks to gets no menu here.
        let busy = client.view().filter(|v| v.initiator != me).map(|v| v.npc.0);
        talk::set_busy(e, busy);
        self.watch(e, me, client, &mut out);
        self.drive(e, client, &mut out);
        out
    }

    /// A line of another player's conversation: shown once, and reported done.
    fn watch(
        &mut self,
        e: &mut Engine,
        me: PlayerId,
        client: &mut DialogueClient,
        out: &mut Vec<DialogueMsg>,
    ) {
        let Some(v) = client.view().copied().filter(|v| v.initiator != me) else {
            return;
        };
        let Some(choice) = v.playing.filter(|_| !v.reported) else {
            return;
        };
        if self.shown != Some((v.session, v.step)) {
            self.shown = Some((v.session, v.step));
            if talk::near(e, v.npc.0) {
                if let Some((text, prompt)) = talk::line(e, choice) {
                    for m in messages(v.initiator, &text, &prompt) {
                        talk::show(e, &m);
                    }
                    self.lines_shown += 1;
                }
            } else {
                self.lines_far += 1;
            }
        }
        out.extend(client.line_done());
    }

    /// This player's own conversation.
    fn drive(&mut self, e: &mut Engine, client: &mut DialogueClient, out: &mut Vec<DialogueMsg>) {
        // An open asked for: granted, refused, or still on its way.
        if let Some((req, _)) = self.pending {
            if client.is_initiator() {
                self.pending = None;
                self.opened += 1;
            } else if client.denied.is_some_and(|(r, _)| r == req) {
                self.pending = None;
                self.denied += 1;
                self.queue.clear();
                talk::close_menu(e);
                talk::show(e, "They are talking to another player.");
            } else {
                return;
            }
        }
        if !client.is_initiator() {
            self.awaiting = None;
            self.closing = false;
            // The first shared line of a conversation asks the host for the NPC.
            if let Some(first) = self.queue.front() {
                let (req, msg) = client.open(FormId(first.npc));
                self.pending = Some((req, first.npc));
                out.push(msg);
            }
            return;
        }
        let Some(v) = client.view().copied() else {
            return;
        };
        if self.closing {
            // The close is on its way: what is said now belongs to the next conversation.
            return;
        }
        if let Some(step) = self.awaiting {
            if v.step < step {
                return;
            }
            self.awaiting = None;
        }
        while let Some(next) = self.queue.front().copied() {
            if next.npc != v.npc.0 {
                // A line of a conversation that is over.
                self.queue.pop_front();
                continue;
            }
            match (next.end, v.playing) {
                // The line ends: every machine may finish it.
                (true, Some(choice)) if choice == next.info => {
                    self.queue.pop_front();
                    out.extend(client.line_done());
                    return;
                }
                // An end with no line playing, or of another line: nothing to finish.
                (true, _) => {
                    self.queue.pop_front();
                }
                // The next line while the last still plays: the last is done first.
                (false, Some(_)) => {
                    out.extend(client.line_done());
                    return;
                }
                (false, None) => {
                    let Some(msg) = client.pick(next.info) else {
                        return;
                    };
                    self.queue.pop_front();
                    self.awaiting = Some(v.step + 1);
                    self.picks += 1;
                    out.push(msg);
                    return;
                }
            }
        }
        // Nothing left to say and the menu is closed: the conversation is over.
        if talk::menu_npc(e).is_none() {
            if v.playing.is_some() {
                out.extend(client.line_done());
            } else if !self.closing {
                self.closing = true;
                out.extend(client.close());
            }
        }
    }
}

/// The host's game runs the results of the guests' lines.
#[derive(Debug, Default)]
pub struct Enactor {
    initiators: BTreeMap<u32, PlayerId>,
    /// Results the host's game ran for a guest.
    pub enacted: u32,
    /// Results left alone: not a line of the shared story, or the NPC is not in the host's game.
    pub left: u32,
    /// Why, for the log (taken by the caller).
    pub notes: Vec<String>,
}

impl Enactor {
    /// Takes one of the dialogue table's events. `shared` are the shared story's quests.
    pub fn on_event(&mut self, e: &mut Engine, ev: DialogueEvent, shared: &BTreeSet<u32>) {
        let (session, npc, choice, end) = match ev {
            DialogueEvent::RunBegin {
                session,
                npc,
                choice,
                initiator,
                ..
            } => {
                self.initiators.insert(session, initiator);
                (session, npc, choice, false)
            }
            DialogueEvent::RunEnd {
                session,
                npc,
                choice,
                ..
            } => (session, npc, choice, true),
            DialogueEvent::Closed { session, .. } => {
                self.initiators.remove(&session);
                return;
            }
        };
        // The host's own lines ran in its game when it said them.
        let from = match self.initiators.get(&session) {
            Some(p) if !p.is_host() => *p,
            _ => return,
        };
        let quest = talk::info_quest(e, choice);
        if !quest.is_some_and(|q| q != 0 && shared.contains(&q)) {
            self.left += 1;
            self.notes.push(format!(
                "player {}'s line {choice:08x} is not a line of the shared story: not run here",
                from.0
            ));
            return;
        }
        match talk::enact(e, npc.0, choice, end) {
            Ok(()) => self.enacted += 1,
            Err(why) => {
                self.left += 1;
                self.notes.push(format!(
                    "player {}'s line {choice:08x} to {:08x} could not be run here: {why:?}",
                    from.0, npc.0
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_watched_line_is_the_prompt_then_each_response() {
        assert_eq!(
            messages(PlayerId(1), "Howdy. | Welcome to Goodsprings.", "Hello."),
            ["Player 2: Hello.", "Howdy.", "Welcome to Goodsprings."]
        );
        assert_eq!(messages(PlayerId(0), "Howdy.", ""), ["Howdy."]);
        assert!(messages(PlayerId(0), "", "").is_empty());
    }
}
