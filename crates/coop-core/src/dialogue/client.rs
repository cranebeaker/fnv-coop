//! Guest side of the dialogue protocol: what the dialogue UI needs to know.

use super::{CloseReason, Deny, DialogueMsg, HEARTBEAT_MS};
use crate::ids::{FormId, PlayerId};

/// The session this machine currently sees.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct View {
    /// Session id.
    pub session: u32,
    /// The NPC.
    pub npc: FormId,
    /// Who picks.
    pub initiator: PlayerId,
    /// Last step picked.
    pub step: u32,
    /// Choice of the step being played (None between steps).
    pub playing: Option<u32>,
    /// True once this machine reported its lines done for the current step.
    pub reported: bool,
}

/// Dialogue state for one machine.
#[derive(Clone, Debug)]
pub struct DialogueClient {
    me: PlayerId,
    next_req: u32,
    view: Option<View>,
    last_beat: u64,
    /// Newest denial of one of our opens.
    pub denied: Option<(u32, Deny)>,
    /// How the last session ended.
    pub last_closed: Option<(u32, CloseReason)>,
    /// Picks of ours the host refused, and the newest refused choice.
    pub refused: u32,
    /// The newest refused pick: (session, choice).
    pub last_refused: Option<(u32, u32)>,
}

impl DialogueClient {
    /// A client for player `me`.
    pub fn new(me: PlayerId) -> Self {
        DialogueClient {
            me,
            next_req: 1,
            view: None,
            last_beat: 0,
            denied: None,
            last_closed: None,
            refused: 0,
            last_refused: None,
        }
    }

    /// Current session, if any.
    pub fn view(&self) -> Option<&View> {
        self.view.as_ref()
    }

    /// True when this machine picks.
    pub fn is_initiator(&self) -> bool {
        self.view.is_some_and(|v| v.initiator == self.me)
    }

    /// True when this machine only watches (banner, read-only list).
    pub fn is_spectator(&self) -> bool {
        self.view.is_some_and(|v| v.initiator != self.me)
    }

    /// True while the initiator's UI must wait for the host's end script.
    pub fn ui_blocked(&self) -> bool {
        self.view.is_some_and(|v| v.playing.is_some())
    }

    /// Asks to talk to `npc`.
    pub fn open(&mut self, npc: FormId) -> (u32, DialogueMsg) {
        let req = self.next_req;
        self.next_req += 1;
        (req, DialogueMsg::Open { req, npc })
    }

    /// Picks the next choice; only the initiator between steps may.
    pub fn pick(&mut self, choice: u32) -> Option<DialogueMsg> {
        let v = self
            .view
            .as_ref()
            .filter(|v| v.initiator == self.me && v.playing.is_none())?;
        Some(DialogueMsg::Pick {
            session: v.session,
            step: v.step + 1,
            choice,
        })
    }

    /// Reports the current step's lines finished (voice and subtitle done).
    pub fn line_done(&mut self) -> Option<DialogueMsg> {
        let v = self
            .view
            .as_mut()
            .filter(|v| v.playing.is_some() && !v.reported)?;
        v.reported = true;
        Some(DialogueMsg::LineDone {
            session: v.session,
            step: v.step,
        })
    }

    /// Ends the conversation (initiator).
    pub fn close(&mut self) -> Option<DialogueMsg> {
        let v = self.view.as_ref().filter(|v| v.initiator == self.me)?;
        Some(DialogueMsg::Close { session: v.session })
    }

    /// Keep-alive when due (initiator).
    pub fn heartbeat(&mut self, now_ms: u64) -> Option<DialogueMsg> {
        let v = self.view.as_ref().filter(|v| v.initiator == self.me)?;
        if now_ms.saturating_sub(self.last_beat) < HEARTBEAT_MS {
            return None;
        }
        self.last_beat = now_ms;
        Some(DialogueMsg::Heartbeat { session: v.session })
    }

    /// Applies a host message.
    pub fn on_msg(&mut self, msg: DialogueMsg, now_ms: u64) {
        match msg {
            DialogueMsg::Started {
                session,
                npc,
                initiator,
                ..
            } => {
                self.view = Some(View {
                    session,
                    npc,
                    initiator,
                    step: 0,
                    playing: None,
                    reported: false,
                });
                self.last_beat = now_ms;
            }
            DialogueMsg::Denied { req, reason } => self.denied = Some((req, reason)),
            DialogueMsg::Refused {
                session, choice, ..
            } => {
                if self.view.is_some_and(|v| v.session == session) {
                    self.refused += 1;
                    self.last_refused = Some((session, choice));
                }
            }
            DialogueMsg::Picked {
                session,
                step,
                choice,
            } => {
                if let Some(v) = self
                    .view
                    .as_mut()
                    .filter(|v| v.session == session && step > v.step)
                {
                    v.step = step;
                    v.playing = Some(choice);
                    v.reported = false;
                }
            }
            DialogueMsg::StepEnd { session, step } => {
                if let Some(v) = self
                    .view
                    .as_mut()
                    .filter(|v| v.session == session && v.step == step)
                {
                    v.playing = None;
                }
            }
            DialogueMsg::Closed { session, reason } => {
                if self.view.is_some_and(|v| v.session == session) {
                    self.view = None;
                }
                self.last_closed = Some((session, reason));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NPC: FormId = FormId(1);

    fn start(c: &mut DialogueClient, initiator: u8) {
        c.on_msg(
            DialogueMsg::Started {
                session: 4,
                npc: NPC,
                initiator: PlayerId(initiator),
                req: 1,
            },
            0,
        );
    }

    #[test]
    fn initiator_flow_blocks_between_pick_and_end() {
        let mut c = DialogueClient::new(PlayerId(1));
        start(&mut c, 1);
        assert!(c.is_initiator());
        let Some(DialogueMsg::Pick { step: 1, .. }) = c.pick(7) else {
            panic!()
        };
        c.on_msg(
            DialogueMsg::Picked {
                session: 4,
                step: 1,
                choice: 7,
            },
            1,
        );
        assert!(c.ui_blocked());
        assert!(c.pick(8).is_none(), "blocked until the end script");
        assert!(c.line_done().is_some());
        assert!(c.line_done().is_none(), "reported once");
        c.on_msg(
            DialogueMsg::StepEnd {
                session: 4,
                step: 1,
            },
            2,
        );
        assert!(!c.ui_blocked());
        let Some(DialogueMsg::Pick { step: 2, .. }) = c.pick(8) else {
            panic!()
        };
    }

    #[test]
    fn spectator_cannot_pick_or_close_but_reports_lines() {
        let mut c = DialogueClient::new(PlayerId(2));
        start(&mut c, 1);
        assert!(c.is_spectator());
        assert!(c.pick(1).is_none() && c.close().is_none() && c.heartbeat(99_999).is_none());
        c.on_msg(
            DialogueMsg::Picked {
                session: 4,
                step: 1,
                choice: 5,
            },
            1,
        );
        assert!(c.line_done().is_some());
    }

    #[test]
    fn stale_messages_are_ignored() {
        let mut c = DialogueClient::new(PlayerId(2));
        start(&mut c, 1);
        c.on_msg(
            DialogueMsg::Picked {
                session: 9,
                step: 1,
                choice: 5,
            },
            1,
        );
        assert!(c.view().unwrap().playing.is_none());
        c.on_msg(
            DialogueMsg::Picked {
                session: 4,
                step: 1,
                choice: 5,
            },
            1,
        );
        c.on_msg(
            DialogueMsg::Picked {
                session: 4,
                step: 1,
                choice: 6,
            },
            1,
        );
        assert_eq!(c.view().unwrap().playing, Some(5));
        c.on_msg(
            DialogueMsg::StepEnd {
                session: 4,
                step: 7,
            },
            2,
        );
        assert!(c.ui_blocked());
    }

    #[test]
    fn heartbeat_cadence_and_close() {
        let mut c = DialogueClient::new(PlayerId(1));
        start(&mut c, 1);
        assert!(c.heartbeat(HEARTBEAT_MS - 1).is_none());
        assert!(c.heartbeat(HEARTBEAT_MS).is_some());
        assert!(c.heartbeat(HEARTBEAT_MS + 1).is_none());
        c.on_msg(
            DialogueMsg::Closed {
                session: 4,
                reason: CloseReason::InitiatorLeft,
            },
            5,
        );
        assert!(c.view().is_none());
        assert_eq!(c.last_closed, Some((4, CloseReason::InitiatorLeft)));
    }

    #[test]
    fn denial_is_recorded() {
        let mut c = DialogueClient::new(PlayerId(1));
        let (req, _) = c.open(NPC);
        c.on_msg(
            DialogueMsg::Denied {
                req,
                reason: Deny::Busy,
            },
            0,
        );
        assert_eq!(c.denied, Some((req, Deny::Busy)));
    }
}
