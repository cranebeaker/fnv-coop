//! Guest side of the live layer: a bundle of the per-domain clients plus the
//! avatar receive buffers, fed by decoded messages.

use std::collections::{BTreeMap, VecDeque};

use crate::actors::ActorClient;
use crate::avatar::{
    ActionRx, AvatarAction, AvatarMsg, AvatarParts, AvatarSender, AvatarState, InterpBuffer,
};
use crate::combat::CombatClient;
use crate::dialogue::DialogueClient;
use crate::ids::{FormId, PlayerId};
use crate::interact::InteractClient;
use crate::loot::LootClient;
use crate::proto::Msg;
use crate::rules::RulesClient;
use crate::scene::SceneClient;
use crate::social::{ChatChannel, Marker, SocialMsg, VoiceRx};

/// Chat lines kept.
pub const CHAT_LOG_CAP: usize = 200;

/// A remote player's avatar.
#[derive(Clone, Debug, Default)]
pub struct RemoteAvatar {
    /// Interpolation buffer for the state stream.
    pub interp: InterpBuffer,
    /// Newest parts.
    pub parts: Option<AvatarParts>,
    /// Combat action events, in order, once each.
    pub actions: ActionRx,
}

/// One received chat line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatLine {
    /// Speaker (`None` = system).
    pub from: Option<PlayerId>,
    /// Display name.
    pub name: String,
    /// Text.
    pub text: String,
}

/// Everything the guest tracks for the live layer.
pub struct GuestLive {
    me: PlayerId,
    /// The local player's avatar state, set by the game each frame.
    pub local: AvatarState,
    /// Fixed-rate sender of the local avatar.
    pub sender: AvatarSender,
    /// Other players' avatars.
    pub remote: BTreeMap<PlayerId, RemoteAvatar>,
    /// Actor ownership and streams.
    pub actors: ActorClient,
    /// Dialogue view.
    pub dialogue: DialogueClient,
    /// Container views.
    pub loot: LootClient,
    /// Rules prompts and notices.
    pub rules: RulesClient,
    /// Scenes and the clock.
    pub scene: SceneClient,
    /// Player-world interactions.
    pub interact: InteractClient,
    /// Combat verdicts and request ids.
    pub combat: CombatClient,
    /// Voice ordering.
    pub voice_rx: VoiceRx,
    /// Received chat.
    pub chat_log: VecDeque<ChatLine>,
    /// Lines the host refused, by message id.
    pub chat_rejected: Vec<u32>,
    /// Map markers by (owner, id).
    pub markers: BTreeMap<(PlayerId, u16), Marker>,
    /// Voice frames accepted in order, for the audio side.
    pub voice_in: Vec<crate::social::VoicePacket>,
    /// The host's weather and clock.
    pub env: crate::env::GuestEnv,
    /// Scene model data: other players' places, start placement, ops.
    pub roam: crate::roam::GuestRoam,
    next_chat: u32,
}

impl GuestLive {
    /// The slot this state belongs to.
    pub fn me(&self) -> PlayerId {
        self.me
    }

    /// A live layer for player `me`.
    pub fn new(me: PlayerId) -> Self {
        GuestLive {
            me,
            local: AvatarState::default(),
            sender: AvatarSender::new(me),
            remote: BTreeMap::new(),
            actors: ActorClient::new(me),
            dialogue: DialogueClient::new(me),
            loot: LootClient::new(),
            rules: RulesClient::new(me),
            scene: SceneClient::new(me),
            interact: InteractClient::new(),
            combat: CombatClient::new(me),
            voice_rx: VoiceRx::new(),
            chat_log: VecDeque::new(),
            chat_rejected: Vec::new(),
            markers: BTreeMap::new(),
            voice_in: Vec::new(),
            env: Default::default(),
            roam: Default::default(),
            next_chat: 1,
        }
    }

    /// A chat message to send.
    pub fn say(&mut self, channel: ChatChannel, text: &str) -> Msg {
        let msg_id = self.next_chat;
        self.next_chat += 1;
        Msg::Social(SocialMsg::Say {
            msg_id,
            channel,
            text: text.to_string(),
        })
    }

    /// Applies one host message; returns replies (none today, kept for
    /// symmetry with the core guest).
    pub fn on_msg(&mut self, msg: Msg, now_ms: u64) -> Vec<Msg> {
        match msg {
            Msg::Avatar(AvatarMsg::State { player, seq, state }) if player != self.me => {
                self.remote
                    .entry(player)
                    .or_default()
                    .interp
                    .push(now_ms, seq, state);
            }
            Msg::Avatar(AvatarMsg::Parts { player, parts }) if player != self.me => {
                let r = self.remote.entry(player).or_default();
                if r.parts.as_ref().map_or(true, |p| parts.rev > p.rev) {
                    r.parts = Some(parts);
                }
            }
            Msg::Avatar(AvatarMsg::Action { player, action }) if player != self.me => {
                self.remote.entry(player).or_default().actions.push(action);
            }
            Msg::Actor(m) => self.actors.on_msg(m),
            Msg::Roam(m) => self.roam.on_msg(self.me, m),
            Msg::Dialogue(m) => self.dialogue.on_msg(m, now_ms),
            Msg::Loot(m) => self.loot.on_msg(m),
            Msg::Rules(m) => self.rules.on_msg(m),
            Msg::Scene(m) => self.scene.on_msg(m, now_ms),
            Msg::Interact(m) => self.interact.on_msg(m),
            Msg::Env(m) => self.env.on_msg(m, now_ms),
            Msg::Combat(m) => self.combat.on_msg(m),
            Msg::Social(SocialMsg::Line {
                from, name, text, ..
            }) => {
                self.chat_log.push_back(ChatLine { from, name, text });
                while self.chat_log.len() > CHAT_LOG_CAP {
                    self.chat_log.pop_front();
                }
            }
            Msg::Social(SocialMsg::Rejected { msg_id, .. }) => self.chat_rejected.push(msg_id),
            Msg::Social(SocialMsg::MarkerShown { owner, marker }) => {
                self.markers.insert((owner, marker.id), marker);
            }
            Msg::Social(SocialMsg::MarkerGone { owner, id }) => {
                self.markers.remove(&(owner, id));
            }
            Msg::Social(SocialMsg::Voice(v)) if self.voice_rx.push(&v).is_some() => {
                self.voice_in.push(v);
            }
            _ => {}
        }
        Vec::new()
    }

    /// Per-step housekeeping: avatar send at its fixed rate, dialogue
    /// heartbeat.
    pub fn tick(&mut self, now_ms: u64) -> Vec<Msg> {
        let mut out = Vec::new();
        if let Some(m) = self.sender.tick(now_ms, &self.local) {
            out.push(Msg::Avatar(m));
        }
        if let Some(m) = self.dialogue.heartbeat(now_ms) {
            out.push(Msg::Dialogue(m));
        }
        out.extend(self.roam.take_out());
        out
    }

    /// A combat action event to send for the local avatar.
    pub fn action(&mut self, kind: crate::avatar::ActionKind, arg: u32) -> Msg {
        Msg::Avatar(self.sender.action(kind, arg))
    }

    /// Takes the action events received from every remote player.
    pub fn take_actions(&mut self) -> Vec<(PlayerId, AvatarAction)> {
        let mut out = Vec::new();
        for (p, r) in self.remote.iter_mut() {
            out.extend(r.actions.take().into_iter().map(|a| (*p, a)));
        }
        out
    }

    /// Looks up an interpolated remote avatar.
    pub fn sample(&mut self, p: PlayerId, now_ms: u64) -> Option<crate::avatar::Sample> {
        self.remote.get_mut(&p)?.interp.sample(now_ms)
    }

    /// Convenience: is this actor owned here?
    pub fn owns_actor(&self, a: FormId) -> bool {
        self.actors.owns(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{CellId, Seq};
    use crate::social::VoicePacket;

    #[test]
    fn tick_sends_avatar_at_fixed_rate() {
        let mut g = GuestLive::new(PlayerId(1));
        let n: usize = (0..1000).step_by(25).map(|t| g.tick(t).len()).sum();
        assert_eq!(n, 20);
    }

    #[test]
    fn remote_avatar_state_and_parts() {
        let mut g = GuestLive::new(PlayerId(1));
        let st = AvatarState {
            cell: CellId(1),
            ..Default::default()
        };
        g.on_msg(
            Msg::Avatar(AvatarMsg::State {
                player: PlayerId(2),
                seq: Seq(1),
                state: st,
            }),
            0,
        );
        g.on_msg(
            Msg::Avatar(AvatarMsg::State {
                player: PlayerId(1),
                seq: Seq(1),
                state: st,
            }),
            0,
        );
        assert_eq!(g.remote.len(), 1, "own echo ignored");
        assert!(g.sample(PlayerId(2), 500).is_some());
        let p = |rev| {
            Msg::Avatar(AvatarMsg::Parts {
                player: PlayerId(2),
                parts: AvatarParts {
                    rev,
                    ..Default::default()
                },
            })
        };
        g.on_msg(p(3), 1);
        g.on_msg(p(2), 2);
        assert_eq!(g.remote[&PlayerId(2)].parts.as_ref().unwrap().rev, 3);
    }

    #[test]
    fn chat_log_is_bounded_and_markers_track() {
        let mut g = GuestLive::new(PlayerId(1));
        for i in 0..CHAT_LOG_CAP + 20 {
            g.on_msg(
                Msg::Social(SocialMsg::Line {
                    from: Some(PlayerId(2)),
                    name: "x".into(),
                    channel: ChatChannel::All,
                    text: i.to_string(),
                }),
                0,
            );
        }
        assert_eq!(g.chat_log.len(), CHAT_LOG_CAP);
        let m = Marker {
            id: 1,
            kind: 0,
            cell: CellId(1),
            pos: [0; 3],
            label: String::new(),
            ttl_ms: 0,
        };
        g.on_msg(
            Msg::Social(SocialMsg::MarkerShown {
                owner: PlayerId(2),
                marker: m,
            }),
            0,
        );
        assert_eq!(g.markers.len(), 1);
        g.on_msg(
            Msg::Social(SocialMsg::MarkerGone {
                owner: PlayerId(2),
                id: 1,
            }),
            0,
        );
        assert!(g.markers.is_empty());
    }

    #[test]
    fn voice_frames_deduplicated() {
        let mut g = GuestLive::new(PlayerId(1));
        let v = |seq| {
            Msg::Social(SocialMsg::Voice(VoicePacket {
                speaker: PlayerId(2),
                seq,
                payload: vec![1],
            }))
        };
        g.on_msg(v(1), 0);
        g.on_msg(v(1), 0);
        g.on_msg(v(2), 0);
        assert_eq!(g.voice_in.len(), 2);
    }

    #[test]
    fn say_ids_increase() {
        let mut g = GuestLive::new(PlayerId(1));
        let (
            Msg::Social(SocialMsg::Say { msg_id: a, .. }),
            Msg::Social(SocialMsg::Say { msg_id: b, .. }),
        ) = (g.say(ChatChannel::All, "a"), g.say(ChatChannel::All, "b"))
        else {
            panic!()
        };
        assert!(b > a);
    }
}
