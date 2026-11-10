//! Host side of the live layer: avatars, actors, dialogue, loot, rules and
//! social, glued to the session. Pure: messages in, addressed messages out.
//! Messages addressed to the host player itself land in a local inbox.

use std::collections::BTreeMap;

use crate::actors::{ActorHost, PlayerLoc};
use crate::authority::AuthorityTable;
use crate::avatar::{dequant_pos, AvatarHub, AvatarMsg};
use crate::combat::{Attribution, CombatHost};
use crate::dialogue::DialogueHost;
use crate::ids::{PlayerId, Seq};
use crate::interact::InteractHost;
use crate::interest::{Audience, InterestMap};
use crate::loot::LootHost;
use crate::proto::Msg;
use crate::rules::{Mode, RulesHost};
use crate::scene::SceneHost;
use crate::social::{ChatHost, MarkerTable, SocialMsg, VoiceHub};

type Outs = Vec<(PlayerId, Msg)>;

/// All host-side live state.
pub struct HostLive {
    /// Avatar table.
    pub avatars: AvatarHub,
    /// Which peer sees which cells.
    pub interest: InterestMap,
    /// Actor ownership, damage, spawns.
    pub actors: ActorHost,
    /// Dialogue sessions.
    pub dialogue: DialogueHost,
    /// Containers and drops.
    pub loot: LootHost,
    /// Tether, barrier, votes, revive, difficulty.
    pub rules: RulesHost,
    /// Scenes, endings and world time.
    pub scene: SceneHost,
    /// Chat.
    pub chat: ChatHost,
    /// Markers and pings.
    pub markers: MarkerTable,
    /// Voice validation.
    pub voice: VoiceHub,
    /// Player-world interactions.
    pub interact: InteractHost,
    /// Scene model, deferred operations, presence.
    pub roam: crate::roam::HostRoam,
    /// Combat depth: player vitals, blasts, VATS, threat, extras.
    pub combat: CombatHost,
    members: Vec<PlayerId>,
    local_inbox: Vec<Msg>,
}

impl HostLive {
    /// A fresh live layer; the host player is the first member.
    pub fn new(mode: Mode) -> Self {
        let mut l = HostLive {
            avatars: AvatarHub::new(),
            interest: InterestMap::new(),
            actors: ActorHost::new(),
            dialogue: DialogueHost::new(),
            loot: LootHost::new(),
            rules: RulesHost::new(mode),
            scene: SceneHost::new(),
            chat: ChatHost::new(),
            markers: MarkerTable::new(),
            voice: VoiceHub::new(),
            interact: InteractHost::new(),
            roam: crate::roam::HostRoam::default(),
            combat: CombatHost::new(),
            members: vec![PlayerId::HOST],
            local_inbox: Vec::new(),
        };
        l.rules.on_join(PlayerId::HOST, 0);
        l.chat.set_name(PlayerId::HOST, "");
        l
    }

    /// Everyone in the session, host first.
    pub fn members(&self) -> &[PlayerId] {
        &self.members
    }

    /// Routes a message the host game produced itself (for example a host-side
    /// container edit or an NPC death) to its audience.
    pub fn route_extern(&mut self, aud: Audience, msg: Msg) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        self.route(aud, msg, &mut out);
        out
    }

    /// Messages the host player's own game must handle.
    pub fn take_local(&mut self) -> Vec<Msg> {
        std::mem::take(&mut self.local_inbox)
    }

    fn route(&mut self, aud: Audience, msg: Msg, out: &mut Outs) {
        for to in self.interest.resolve(aud, &self.members) {
            if to.is_host() {
                self.local_inbox.push(msg.clone());
            } else {
                out.push((to, msg.clone()));
            }
        }
    }

    /// A player joined (or rejoined).
    pub fn join(&mut self, p: PlayerId, now_ms: u64) -> Outs {
        let mut out = Vec::new();
        if self.members.contains(&p) {
            // a rejoin: the scenes in progress are replayed to them
            let clock = self.scene.last_clock();
            for m in self.scene.resync(p, now_ms, clock) {
                self.route(m.aud, Msg::Scene(m.msg), &mut out);
            }
            return out;
        }
        self.members.push(p);
        self.chat.set_name(p, "");
        for m in self.avatars.parts_snapshot() {
            self.route(Audience::To(p), Msg::Avatar(m), &mut out);
        }
        for m in self.markers.snapshot() {
            self.route(Audience::To(p), Msg::Social(m), &mut out);
        }
        for m in self.rules.on_join(p, now_ms) {
            self.route(m.aud, Msg::Rules(m.msg), &mut out);
        }
        for (a, m) in self.roam.on_join(p) {
            self.route(a, m, &mut out);
        }
        let clock = self.scene.last_clock();
        for m in self.scene.on_join(p, now_ms, clock) {
            self.route(m.aud, Msg::Scene(m.msg), &mut out);
        }
        for o in self.interact.sync_for(p) {
            self.route(o.aud, Msg::Interact(o.msg), &mut out);
        }
        out
    }

    /// A player left; every open rule stops waiting for them.
    pub fn leave(&mut self, p: PlayerId, now_ms: u64) -> Outs {
        let mut out = Vec::new();
        self.members.retain(|m| *m != p);
        self.avatars.remove(p);
        self.roam.on_leave(&mut self.interest, p);
        for m in self.dialogue.on_disconnect(p) {
            self.route(m.aud, Msg::Dialogue(m.msg), &mut out);
        }
        self.loot.on_disconnect(p);
        for m in self.rules.on_leave(p, now_ms) {
            self.route(m.aud, Msg::Rules(m.msg), &mut out);
        }
        for m in self.markers.remove_player(p) {
            self.route(m.aud, Msg::Social(m.msg), &mut out);
        }
        for m in self.scene.on_leave(p, now_ms) {
            self.route(m.aud, Msg::Scene(m.msg), &mut out);
        }
        self.chat.remove(p);
        self.voice.remove(p);
        self.interact.forget(p);
        out
    }

    /// Handles one live-layer message from `from`. Other messages are ignored.
    pub fn on_msg(
        &mut self,
        from: PlayerId,
        msg: Msg,
        now_ms: u64,
        auth: &mut AuthorityTable,
    ) -> Outs {
        let mut out = Vec::new();
        if !self.members.contains(&from) {
            return out;
        }
        match msg {
            Msg::Avatar(m) => self.on_avatar(from, m, &mut out),
            Msg::Actor(m) => {
                let cell = self.interest.peer_cell(from);
                for o in self.actors.on_msg(from, m, now_ms, cell, auth) {
                    note_attribution(&mut self.combat.attributions, &o.msg);
                    self.route(o.aud, Msg::Actor(o.msg), &mut out);
                }
            }
            Msg::Combat(m) => self.on_combat(from, m, now_ms, auth, &mut out),
            Msg::Dialogue(m) => {
                let members = self.members.clone();
                for o in self.dialogue.on_msg(from, m, now_ms, &members) {
                    self.route(o.aud, Msg::Dialogue(o.msg), &mut out);
                }
            }
            Msg::Loot(m) => {
                for o in self.loot.on_msg(from, m, now_ms) {
                    self.route(o.aud, Msg::Loot(o.msg), &mut out);
                }
            }
            Msg::Rules(m) => {
                let hub = &self.avatars;
                let os = self
                    .rules
                    .on_msg(from, m, now_ms, &|a, b| hub.distance(a, b));
                for o in os {
                    self.route(o.aud, Msg::Rules(o.msg), &mut out);
                }
            }
            Msg::Social(m) => self.on_social(from, m, now_ms, &mut out),
            Msg::Roam(m) => self.roam.on_msg(from, m),
            Msg::Scene(m) => {
                for o in self.scene.on_msg(from, m, now_ms) {
                    self.route(o.aud, Msg::Scene(o.msg), &mut out);
                }
            }
            Msg::Interact(m) => {
                for o in self.interact.on_msg(from, m) {
                    self.route(o.aud, Msg::Interact(o.msg), &mut out);
                }
            }
            _ => {}
        }
        out
    }

    fn on_combat(
        &mut self,
        from: PlayerId,
        m: crate::combat::CombatMsg,
        now_ms: u64,
        auth: &mut AuthorityTable,
        out: &mut Outs,
    ) {
        let at = self.avatars.state(from).map(|s| {
            [
                dequant_pos(s.pos[0]),
                dequant_pos(s.pos[1]),
                dequant_pos(s.pos[2]),
            ]
        });
        let cell = self.interest.peer_cell(from);
        let scaling = self.rules.scaling(now_ms);
        for o in self.combat.on_msg(from, m, now_ms, at, cell, &scaling) {
            self.route(o.aud, Msg::Combat(o.msg), out);
        }
        // VATS shots that hit go through the one actor ledger
        for h in std::mem::take(&mut self.combat.vats_hits) {
            let cell = self.interest.peer_cell(h.from);
            let msg = crate::actors::ActorMsg::Hit {
                actor: h.target,
                hit: h.hit,
                amount: h.amount,
                npc: None,
            };
            for o in self.actors.on_msg(h.from, msg, now_ms, cell, auth) {
                note_attribution(&mut self.combat.attributions, &o.msg);
                self.route(o.aud, Msg::Actor(o.msg), out);
            }
        }
    }

    /// Routes combat messages the host game produced itself (NPC and blast
    /// damage to players, effects).
    pub fn route_combat(&mut self, msgs: Vec<crate::combat::CombatMsg>) -> Outs {
        let mut out = Vec::new();
        for o in CombatHost::deliver(msgs) {
            self.route(o.aud, Msg::Combat(o.msg), &mut out);
        }
        out
    }

    /// Routes one combat message to every member (stealth numbers).
    pub fn route_combat_all(&mut self, m: crate::combat::CombatMsg) -> Outs {
        let mut out = Vec::new();
        self.route(Audience::All, Msg::Combat(m), &mut out);
        out
    }

    fn on_avatar(&mut self, from: PlayerId, m: AvatarMsg, out: &mut Outs) {
        let Some(relay) = self.avatars.on_msg(from, m) else {
            return;
        };
        let aud = match relay.cell {
            Some(c) => {
                self.interest.set_peer_cell(from, c);
                let pos = self
                    .avatars
                    .state(from)
                    .map_or([0.0; 3], |s| s.pos.map(dequant_pos));
                for (a, m) in self
                    .roam
                    .on_avatar(&mut self.interest, &self.actors, from, c, pos)
                {
                    self.route(a, m, out);
                }
                Audience::CellExcept(c, from)
            }
            None => Audience::AllExcept(from),
        };
        self.route(aud, Msg::Avatar(relay.msg), out);
    }

    fn on_social(&mut self, from: PlayerId, m: SocialMsg, now_ms: u64, out: &mut Outs) {
        match m {
            SocialMsg::Say {
                msg_id,
                channel,
                text,
            } => {
                for o in self.chat.on_say(from, msg_id, channel, &text, now_ms) {
                    self.route(o.aud, Msg::Social(o.msg), out);
                }
            }
            SocialMsg::MarkerSet { marker } => {
                for o in self.markers.set(from, marker, now_ms) {
                    self.route(o.aud, Msg::Social(o.msg), out);
                }
            }
            SocialMsg::MarkerClear { id } => {
                for o in self.markers.clear(from, id, now_ms) {
                    self.route(o.aud, Msg::Social(o.msg), out);
                }
            }
            SocialMsg::Voice(v) => {
                if let Some(aud) = self.voice.on_packet(from, &v, now_ms) {
                    self.route(aud, Msg::Social(SocialMsg::Voice(v)), out);
                }
            }
            _ => {}
        }
    }

    fn locations(&self) -> BTreeMap<PlayerId, PlayerLoc> {
        self.members
            .iter()
            .filter_map(|p| {
                let s = self.avatars.state(*p)?;
                let pos = [
                    dequant_pos(s.pos[0]),
                    dequant_pos(s.pos[1]),
                    dequant_pos(s.pos[2]),
                ];
                Some((*p, PlayerLoc { cell: s.cell, pos }))
            })
            .collect()
    }

    /// Advances every timer and ownership; returns the sends.
    pub fn tick(&mut self, now_ms: u64, auth: &mut AuthorityTable) -> Outs {
        let mut out = Vec::new();
        self.roam.model.apply_ownership(&mut self.actors.own);
        for (a, m) in self.roam.tick(now_ms) {
            self.route(a, m, &mut out);
        }
        let locs = self.locations();
        for o in self.actors.tick(now_ms, &locs, auth) {
            self.route(o.aud, Msg::Actor(o.msg), &mut out);
        }
        for o in self.dialogue.tick(now_ms) {
            self.route(o.aud, Msg::Dialogue(o.msg), &mut out);
        }
        for o in self.loot.tick(now_ms) {
            self.route(o.aud, Msg::Loot(o.msg), &mut out);
        }
        let hub = &self.avatars;
        let os = self.rules.tick(now_ms, &|a, b| hub.distance(a, b));
        for o in os {
            self.route(o.aud, Msg::Rules(o.msg), &mut out);
        }
        for o in self.markers.tick(now_ms) {
            self.route(o.aud, Msg::Social(o.msg), &mut out);
        }
        for o in self.scene.tick(now_ms) {
            self.route(o.aud, Msg::Scene(o.msg), &mut out);
        }
        out
    }

    /// Routes scene messages the host game produced (opening a scene, a time
    /// skip, a slide); the host player's own copy lands in the local inbox.
    pub fn route_scene(&mut self, outs: Vec<crate::scene::SOut>) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        for o in outs {
            self.route(o.aud, Msg::Scene(o.msg), &mut out);
        }
        out
    }

    /// Convenience: the host player's own avatar sample.
    pub fn local_avatar(
        &mut self,
        seq: Seq,
        state: crate::avatar::AvatarState,
        now_ms: u64,
        auth: &mut AuthorityTable,
    ) -> Outs {
        let m = Msg::Avatar(AvatarMsg::State {
            player: PlayerId::HOST,
            seq,
            state,
        });
        self.on_msg(PlayerId::HOST, m, now_ms, auth)
    }
}

fn note_attribution(v: &mut Vec<Attribution>, m: &crate::actors::ActorMsg) {
    use crate::actors::ActorMsg;
    match *m {
        ActorMsg::Damaged {
            actor,
            by: Some(by),
            amount,
            ..
        } => v.push(Attribution {
            actor,
            by,
            amount,
            killed: false,
        }),
        ActorMsg::Died {
            actor,
            killer: Some(by),
        } => v.push(Attribution {
            actor,
            by,
            amount: 0,
            killed: true,
        }),
        _ => {}
    }
    if v.len() > 512 {
        v.drain(..256);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actors::ActorMsg;
    use crate::avatar::{quant_pos, AvatarState};
    use crate::dialogue::DialogueMsg;
    use crate::ids::CellId;
    use crate::rules::RulesMsg;

    fn state(cell: u32, x: f32) -> AvatarState {
        AvatarState {
            cell: CellId(cell),
            pos: [quant_pos(x), 0, 0],
            ..Default::default()
        }
    }
    fn av(p: u8, q: u32, cell: u32, x: f32) -> Msg {
        Msg::Avatar(AvatarMsg::State {
            player: PlayerId(p),
            seq: Seq(q),
            state: state(cell, x),
        })
    }
    fn live3() -> (HostLive, AuthorityTable) {
        let mut l = HostLive::new(Mode::CoopBalanced);
        l.join(PlayerId(1), 0);
        l.join(PlayerId(2), 0);
        (l, AuthorityTable::new())
    }

    #[test]
    fn avatar_goes_only_to_peers_in_view_and_host_gets_local_copy() {
        let (mut l, mut auth) = live3();
        l.on_msg(PlayerId(0), av(0, 1, 1, 0.0), 0, &mut auth);
        l.on_msg(PlayerId(2), av(2, 1, 2, 0.0), 0, &mut auth);
        l.take_local();
        let out = l.on_msg(PlayerId(1), av(1, 1, 1, 5.0), 10, &mut auth);
        assert!(out.is_empty(), "player 2 is in another cell");
        assert_eq!(l.take_local().len(), 1, "host sees it");
        l.interest.link(CellId(1), CellId(2));
        let out = l.on_msg(PlayerId(1), av(1, 2, 1, 6.0), 20, &mut auth);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0, PlayerId(2));
    }

    #[test]
    fn strangers_are_ignored() {
        let (mut l, mut auth) = live3();
        assert!(l
            .on_msg(PlayerId(9), av(9, 1, 1, 0.0), 0, &mut auth)
            .is_empty());
        assert!(l.avatars.state(PlayerId(9)).is_none());
    }

    #[test]
    fn ownership_follows_the_nearest_player_through_tick() {
        let (mut l, mut auth) = live3();
        let a = crate::ids::FormId(0x100);
        l.actors.register(
            a,
            crate::actors::ActorState {
                cell: CellId(1),
                pos: [quant_pos(10.0), 0, 0],
                health: 50,
                ..Default::default()
            },
        );
        l.on_msg(PlayerId(1), av(1, 1, 1, 0.0), 0, &mut auth);
        l.on_msg(PlayerId(2), av(2, 1, 1, 900.0), 0, &mut auth);
        let out = l.tick(100, &mut auth);
        assert!(out.iter().any(|(to, m)| *to == PlayerId(2)
            && matches!(
                m,
                Msg::Actor(ActorMsg::Assign {
                    owner: Some(PlayerId(1)),
                    ..
                })
            )));
        let leave = l.leave(PlayerId(1), 200);
        let _ = leave;
        auth.release_all_of(PlayerId(1));
        let out = l.tick(300, &mut auth);
        assert!(out.iter().any(|(_, m)| matches!(
            m,
            Msg::Actor(ActorMsg::Assign {
                owner: Some(PlayerId(2)),
                ..
            })
        )));
    }

    #[test]
    fn join_sends_known_parts_and_markers() {
        let (mut l, mut auth) = live3();
        let mut s = crate::avatar::AvatarSender::new(PlayerId(1));
        let parts = s.set_parts(vec![(0, crate::ids::FormId(5))], vec![1]);
        l.on_msg(PlayerId(1), Msg::Avatar(parts), 0, &mut auth);
        let out = l.join(PlayerId(3), 10);
        assert!(
            out.iter()
                .any(|(to, m)| *to == PlayerId(3)
                    && matches!(m, Msg::Avatar(AvatarMsg::Parts { .. })))
        );
        assert!(out
            .iter()
            .any(|(_, m)| matches!(m, Msg::Rules(RulesMsg::ScalingSet { players: 4, .. }))));
        assert!(
            l.join(PlayerId(3), 11)
                .iter()
                .all(|(_, m)| matches!(m, Msg::Scene(_))),
            "idempotent: a rejoin only replays the scenes"
        );
    }

    #[test]
    fn leave_cleans_everything() {
        let (mut l, mut auth) = live3();
        l.on_msg(
            PlayerId(1),
            Msg::Dialogue(DialogueMsg::Open {
                req: 1,
                npc: crate::ids::FormId(7),
            }),
            0,
            &mut auth,
        );
        l.on_msg(PlayerId(1), av(1, 1, 1, 0.0), 0, &mut auth);
        let out = l.leave(PlayerId(1), 5);
        assert!(out
            .iter()
            .any(|(_, m)| matches!(m, Msg::Dialogue(DialogueMsg::Closed { .. }))));
        assert_eq!(l.members().len(), 2);
        assert!(l.interest.peer_cell(PlayerId(1)).is_none());
    }
}
