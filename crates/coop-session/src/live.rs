//! The live layer of a session: avatars, shared actors, loot, dialogue and
//! rules on top of the replicated state, plus the campaign policy and the
//! per-player shadows.
//!
//! Off unless [`crate::SessionConfig::live`] is set. The viewer calls
//! [`Session::live_frame`] once per frame (after the local player's state is
//! written to the `GameState`, before [`Session::post_frame`]) and reads the
//! plain-data [`LiveView`] it returns. Nothing here depends on Bevy.
//!
//! What `live_frame` does:
//!
//! * host: shadow registration per joined player, `PolicyGate` presence and
//!   timers, [`LiveHostAdapter::frame`], re-running stages the gate released
//!   (or redirected to one player's shadow);
//! * guest: [`LiveGuestAdapter::frame`];
//! * both: travel requests become party votes, and the view is built from the
//!   interpolation buffers (remote avatars), the actor table (shared actors)
//!   and the `GameState` live facts (dialogue lines, flags).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use coop_core::avatar::{
    dequant_heading, dequant_pos, FLAG_DEAD, FLAG_DOWNED, FLAG_DRAWN, FLAG_SNEAK,
};
use coop_core::ids::{FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use coop_core::rules::VoteKind;
use coop_core::social::ChatChannel;
use coop_core::sync::GuestLive;
use coop_policy::PolicyTable;
use esm::FormId;
use world::coop::apply::Ctx;
use world::coop::facts::puppet_of;
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::ShadowPlan;
use world::scripting::{GameState, Runner};

use crate::session::send_msg;
use crate::{Role, Session};

/// How long an action asked for before the layer was ready stays valid (session ms).
pub const QUEUE_MS: u64 = 15_000;
/// Most actions kept while waiting.
pub const QUEUE_MAX: usize = 64;

/// The slot a guest's transport reports before the host assigns one.
const UNASSIGNED: u8 = 255;

fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

fn wf(f: CForm) -> FormId {
    FormId(f.0)
}

/// A remote player, interpolated, ready to drive a puppet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RemoteAvatarView {
    /// Their slot.
    pub player: PlayerId,
    /// Display name.
    pub name: String,
    /// The synthetic reference that stands for them.
    pub puppet: FormId,
    /// Cell form id they stand in.
    pub cell: u32,
    /// Position (world units).
    pub pos: [f32; 3],
    /// Heading in radians.
    pub heading: f32,
    /// Velocity (units per second).
    pub vel: [f32; 3],
    /// Animation id they publish.
    pub anim: u16,
    /// Drawn weapon form (0: none).
    pub weapon: u32,
    /// Health points.
    pub health: u16,
    /// Downed (waiting for a revive).
    pub downed: bool,
    /// Dead.
    pub dead: bool,
    /// Sneaking.
    pub sneaking: bool,
    /// Weapon drawn.
    pub drawn: bool,
    /// The sample jumped (cell change or warp): do not blend.
    pub teleported: bool,
    /// Worn equipment (slot, item form).
    pub equipment: Vec<(u8, FormId)>,
    /// Revision of the equipment/appearance parts.
    pub parts_rev: u32,
}

/// An actor owned by another peer, as last published.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActorView {
    /// The reference.
    pub id: FormId,
    /// Cell form id.
    pub cell: u32,
    /// Position.
    pub pos: [f32; 3],
    /// Heading in radians.
    pub heading: f32,
    /// Animation id.
    pub anim: u16,
    /// Health points.
    pub health: u16,
    /// Target (0: none).
    pub target: u32,
    /// Dead per the host.
    pub dead: bool,
}

/// One row of the actor table this machine knows (for the dump and tests).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActorRow {
    /// The reference.
    pub id: FormId,
    /// Current owner (`None`: dormant, nobody shares its place).
    pub owner: Option<PlayerId>,
    /// Alive per the host.
    pub alive: bool,
}

/// A dynamic ref the host announced (a creature or person made while
/// playing): every peer shows it once, under the host's id.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpawnedView {
    /// Host-allocated id.
    pub id: FormId,
    /// Base form.
    pub base: FormId,
    /// Place (cell or worldspace) it was made in.
    pub cell: u32,
    /// Where it was made.
    pub pos: [f32; 3],
}

/// The dialogue session this machine sees.
#[derive(Clone, Debug, PartialEq)]
pub struct DialogueView {
    /// Session id.
    pub session: u32,
    /// The NPC.
    pub npc: FormId,
    /// Who picks.
    pub initiator: PlayerId,
    /// True when this machine picks.
    pub mine: bool,
    /// The choice being played (`None` between steps).
    pub playing: Option<FormId>,
}

/// A map marker or ping of another player (compass dot data).
#[derive(Clone, Debug, PartialEq)]
pub struct MarkerView {
    /// Owner.
    pub owner: PlayerId,
    /// Game-defined kind.
    pub kind: u8,
    /// Cell form id.
    pub cell: u32,
    /// Position.
    pub pos: [f32; 3],
    /// Label.
    pub label: String,
}

/// One chat line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatView {
    /// Speaker (`None`: system).
    pub from: Option<PlayerId>,
    /// Speaker's name.
    pub name: String,
    /// Text.
    pub text: String,
}

/// Rule facts the viewer reacts to (copied from `gs.coop.live`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveFlags {
    /// The local player is downed.
    pub downed: bool,
    /// The whole party was downed.
    pub wiped: bool,
    /// A tether holds the story stage.
    pub stage_held: bool,
    /// The party allowed fast travel to this marker.
    pub travel_allowed: Option<FormId>,
    /// The party allowed a sleep or wait of this many hours.
    pub time_skip_ok: Option<u32>,
    /// Result of a fork vote (param, approved).
    pub fork_result: Option<(u32, bool)>,
}

/// Everything the viewer needs from the live layer for one frame.
#[derive(Clone, Debug, Default)]
pub struct LiveView {
    /// Session clock (ms) of this view.
    pub now_ms: u64,
    /// This machine's slot.
    pub me: PlayerId,
    /// Everyone in the session, host first.
    pub members: Vec<PlayerId>,
    /// Remote players with a published avatar (the viewer shows those in
    /// view of the local player).
    pub avatars: Vec<RemoteAvatarView>,
    /// Players that left since the last frame (despawn their puppets).
    pub left: Vec<PlayerId>,
    /// Actors owned by other peers: apply their state to the local copy.
    pub remote_actors: Vec<ActorView>,
    /// Actors this machine owns: publish animation ids for them
    /// (`gs.coop.live.anim`).
    pub owned_actors: Vec<FormId>,
    /// Every actor this machine knows, with its owner and whether it lives.
    pub actor_table: Vec<ActorRow>,
    /// Dynamic refs the host announced and that still exist.
    pub spawned: Vec<SpawnedView>,
    /// The dialogue session, if any.
    pub dialogue: Option<DialogueView>,
    /// Picked lines to play this frame: (step, INFO form).
    pub lines: Vec<(u32, FormId)>,
    /// Chat lines that arrived this frame.
    pub chat: Vec<ChatView>,
    /// Other players' markers.
    pub markers: Vec<MarkerView>,
    /// Rule facts.
    pub flags: LiveFlags,
    /// Short messages for the HUD.
    pub notices: Vec<String>,
    /// Combat action events other players did since the last frame, in
    /// order per player (puppets play them).
    pub actions: Vec<(PlayerId, coop_core::avatar::AvatarAction)>,
    /// Cell presence, owners per peer, deferred operations, a placement to
    /// take (the roam model, docs/COOP-SCENES.md).
    pub roam: crate::live_roam::RoamView,
    /// The scene this player is in or held for (movie, locks, camera), if any.
    pub scene: Option<coop_core::scene::SceneView>,
    /// The party prompt waiting for this player's answer (see [`Intent::Answer`]).
    pub prompt: Option<coop_core::rules::Prompt>,
    /// Voice frames other players sent since the last frame.
    pub voice: Vec<crate::live_extra::VoiceFrame>,
    /// Guest: the host's game clock, smoothed (absolute hours).
    pub hour: Option<f64>,
}

/// Something the local player does that the live layer carries.
#[derive(Clone, Debug, PartialEq)]
pub enum Intent {
    /// Starts talking to an NPC.
    DialogueOpen(FormId),
    /// Picks a line (INFO form).
    DialoguePick(FormId),
    /// The step's lines finished playing here.
    DialogueLineDone,
    /// Leaves the conversation.
    DialogueClose,
    /// Opens a container (takes the lease).
    LootOpen(FormId),
    /// Takes items from a leased container.
    LootTake {
        /// Container.
        container: FormId,
        /// Item.
        item: FormId,
        /// Count.
        n: u32,
    },
    /// Puts items into a leased container.
    LootPut {
        /// Container.
        container: FormId,
        /// Item.
        item: FormId,
        /// Count.
        n: u32,
    },
    /// Closes a container.
    LootClose(FormId),
    /// Claims a dropped item.
    ClaimDrop(FormId),
    /// Chat to everyone.
    Say(String),
    /// Asks the party for a vote.
    Vote(VoteKind, u32),
    /// Starts reviving a downed player.
    ReviveBegin(PlayerId),
    /// Lets go of a revive.
    ReviveCancel,
    /// Answers the open party prompt (a vote to sleep or wait, a barrier).
    Answer(bool),
    /// One encoded voice frame (envelope bytes) from the local microphone.
    Voice(Vec<u8>),
    /// A marker or ping to set or clear (`SocialMsg::MarkerSet` / `MarkerClear`).
    Social(coop_core::social::SocialMsg),
}

/// The live layer's own state, inside the session.
pub struct Layer {
    policy: Option<Arc<PolicyTable>>,
    no_policy: bool,
    setup: bool,
    registered: BTreeSet<PlayerId>,
    pub(crate) host: Option<LiveHostAdapter>,
    pub(crate) guest: Option<(PlayerId, LiveGuestAdapter)>,
    asked_travel: BTreeMap<FormId, u64>,
    present: BTreeSet<PlayerId>,
    was_wiped: bool,
    was_held: bool,
    /// Intents asked for before the layer could carry them (a guest that has
    /// not joined yet), with the session time they were asked at.
    queued: Vec<(u64, Intent)>,
    pub(crate) x: crate::live_extra::Extra,
    /// The view of the last frame.
    pub view: LiveView,
}

impl Layer {
    pub(crate) fn new(
        role: Role,
        policy: Option<Arc<PolicyTable>>,
        no_policy: bool,
        native: bool,
    ) -> Layer {
        let mut host = (role == Role::Host).then(LiveHostAdapter::new);
        if let Some(h) = host.as_mut() {
            h.dialogue.native_host = native;
        }
        Layer {
            policy,
            no_policy,
            setup: false,
            registered: BTreeSet::new(),
            host,
            guest: None,
            asked_travel: BTreeMap::new(),
            present: BTreeSet::new(),
            was_wiped: false,
            was_held: false,
            queued: Vec::new(),
            x: Default::default(),
            view: LiveView::default(),
        }
    }
}

fn chat_name(names: &BTreeMap<PlayerId, String>, p: PlayerId) -> String {
    names
        .get(&p)
        .cloned()
        .unwrap_or_else(|| format!("player {}", p.0))
}

impl Session {
    /// The view built by the last [`Session::live_frame`].
    pub fn live_view(&self) -> Option<&LiveView> {
        self.live.as_ref().map(|l| &l.view)
    }

    /// True when the live layer is running in this session.
    pub fn live_enabled(&self) -> bool {
        self.live.is_some()
    }

    /// Hit requests sent so far (diagnostics and tests).
    pub fn live_hits_sent(&self) -> u32 {
        let Some(l) = &self.live else { return 0 };
        l.host
            .as_ref()
            .map(|h| h.stats.hits_sent)
            .or_else(|| l.guest.as_ref().map(|g| g.1.stats.hits_sent))
            .unwrap_or(0)
    }

    /// Start of the frame (called by `pre_frame_with`): turns on the shadows
    /// and the policy gate, once, and keeps the guest adapter matched to the
    /// slot the host assigned.
    pub(crate) fn live_setup(&mut self, gs: &mut GameState, order: Option<&esm::LoadOrder>) {
        let Some(l) = self.live.as_mut() else { return };
        if let Some(g) = &mut self.guest {
            let me = g.sync.player;
            // 255 is the transport's placeholder until the Welcome assigns
            // the real slot: no adapter, no shadows, no rows under it.
            if me.0 == UNASSIGNED {
                return;
            }
            if l.guest.as_ref().map(|x| x.0) != Some(me) && !me.is_host() {
                g.sync.live = GuestLive::new(me);
                l.guest = Some((me, LiveGuestAdapter::new(me)));
            }
        }
        if l.setup {
            return;
        }
        let me = match (&self.host, &self.guest) {
            (Some(_), _) => 0u8,
            (_, Some(g)) if !g.sync.player.is_host() => g.sync.player.0,
            _ => return,
        };
        l.setup = true;
        if l.no_policy {
            return;
        }
        let mut problem = None;
        let mut note: Option<String> = None;
        let table = match l.policy.clone() {
            Some(t) => Some(t),
            None => match PolicyTable::bundled() {
                // the file keys DLC quests by their own load index: key them by the real order
                Ok(t) => match order {
                    Some(o) => match crate::policy_ids::resolve_policy(&t, o) {
                        Ok((t, rep)) => {
                            note = Some(rep.describe());
                            Some(Arc::new(t))
                        }
                        Err(e) => {
                            problem = Some(format!("policy ids: {e}"));
                            None
                        }
                    },
                    None => Some(Arc::new(t)),
                },
                Err(e) => {
                    problem = Some(format!("policy table: {e:?}"));
                    None
                }
            },
        };
        if let Some(table) = table {
            if self.host.is_some() {
                gs.enable_coop_policy(table, me);
            } else {
                gs.enable_shadows(ShadowPlan::from_policy(&table), me);
            }
        }
        if let Some(msg) = note {
            self.log_event("policy_ids", &[("msg", crate::json::V::S(&msg))]);
        }
        if let Some(msg) = problem {
            self.log_event("error", &[("msg", crate::json::V::S(&msg))]);
        }
    }

    /// Host: starts replicating the actors of the loaded cells; a guest
    /// reports its own to the host. False when the layer cannot take them
    /// yet (ask again).
    pub fn live_track(&mut self, cx: Ctx, gs: &GameState, actors: &[FormId]) -> bool {
        let Some(l) = self.live.as_mut() else {
            return false;
        };
        if let (Some(h), Some(ad)) = (&mut self.host, l.host.as_mut()) {
            ad.track(cx, gs, &mut h.sync.live, actors);
            true
        } else if let Some((_, ad)) = l.guest.as_mut() {
            // a guest reports what it has loaded; the host tracks it too
            ad.track(cx, gs, actors);
            true
        } else {
            // a guest that has not joined yet: ask again later
            false
        }
    }

    /// Actors the host replicates (host) or this guest has reported.
    pub fn live_tracked(&self) -> usize {
        let Some(l) = &self.live else { return 0 };
        l.host
            .as_ref()
            .map(|h| h.tracked_count())
            .or_else(|| l.guest.as_ref().map(|g| g.1.tracked_count()))
            .unwrap_or(0)
    }

    /// Host: registers a container for shared looting. False when it was
    /// registered before (or the layer is off).
    pub fn live_register_container(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        container: FormId,
    ) -> bool {
        let (Some(h), Some(l)) = (&mut self.host, self.live.as_mut()) else {
            return false;
        };
        let Some(ad) = l.host.as_mut() else {
            return false;
        };
        ad.loot.register(cx, gs, &mut h.sync.live, container)
    }

    /// Host: registers a dropped item for first-come claims.
    pub fn live_register_drop(&mut self, cx: Ctx, gs: &GameState, drop: FormId) {
        if let (Some(h), Some(l)) = (&mut self.host, self.live.as_mut()) {
            if let Some(ad) = l.host.as_mut() {
                ad.loot.register_drop(cx, gs, &mut h.sync.live, drop);
            }
        }
    }

    /// Host: starts (`Some`) or ends (`None`) the story tether at an anchor.
    pub fn live_tether(&mut self, cx: Ctx, gs: &GameState, anchor: Option<FormId>) {
        let now = self.now_ms;
        let (Some(h), Some(l)) = (&mut self.host, self.live.as_mut()) else {
            return;
        };
        let Some(ad) = l.host.as_mut() else { return };
        match anchor {
            Some(a) => ad.rules.start_tether(cx, gs, &mut h.sync.live, a, now),
            None => ad.rules.end_tether(&mut h.sync.live),
        }
    }

    /// Sets what the local avatar wears (slot, item) and its look; sent
    /// reliably, relayed to the others.
    pub fn set_local_parts(&mut self, equipment: Vec<(u8, FormId)>, appearance: Vec<u8>) {
        let eq: Vec<(u8, CForm)> = equipment.into_iter().map(|(s, f)| (s, cf(f))).collect();
        let now = self.now_ms;
        let Some(l) = self.live.as_mut() else { return };
        if let (Some(h), Some(ad)) = (&mut self.host, l.host.as_mut()) {
            let m = ad.own.sender.set_parts(eq, appearance);
            let outs = ad.on_local(&mut h.sync.live, &mut h.sync.auth, now, Msg::Avatar(m));
            for (to, m) in outs {
                send_msg(&mut self.net, to, &m);
            }
        } else if let Some(g) = &mut self.guest {
            let m = g.sync.live.sender.set_parts(eq, appearance);
            send_msg(&mut self.net, PlayerId::HOST, &Msg::Avatar(m));
        }
    }

    /// Whether the live layer can carry an intent now: always on a host, on a
    /// guest once the host has assigned it a slot and its state is flowing.
    fn live_ready(&self) -> bool {
        let Some(l) = &self.live else { return false };
        match (&self.host, &self.guest) {
            (Some(_), _) => l.host.is_some(),
            (_, Some(g)) => {
                l.guest.is_some() && g.sync.phase() == coop_core::sync::GuestPhase::Active
            }
            _ => false,
        }
    }

    /// Carries a local action to the host (or runs it on the host). An action
    /// asked for before a guest has joined is kept (for [`QUEUE_MS`] of session
    /// time, [`QUEUE_MAX`] at most) and carried as soon as the layer can.
    pub fn live_intent(&mut self, intent: Intent) {
        if self.live.is_some() && !self.live_ready() {
            if matches!(intent, Intent::Voice(_)) {
                return;
            }
            let now = self.now_ms;
            if let Some(l) = self.live.as_mut() {
                if l.queued.len() < QUEUE_MAX {
                    l.queued.push((now, intent));
                }
            }
            return;
        }
        self.live_intent_now(intent);
    }

    /// Carries the actions that waited for the layer.
    fn flush_queued(&mut self) {
        if !self.live_ready() {
            return;
        }
        let now = self.now_ms;
        let q = match self.live.as_mut() {
            Some(l) if !l.queued.is_empty() => std::mem::take(&mut l.queued),
            _ => return,
        };
        for (at, i) in q {
            if now.saturating_sub(at) <= QUEUE_MS {
                self.live_intent_now(i);
            }
        }
    }

    fn live_intent_now(&mut self, intent: Intent) {
        let now = self.now_ms;
        let Some(l) = self.live.as_mut() else { return };
        if let (Some(h), Some(ad)) = (&mut self.host, l.host.as_mut()) {
            let mut msgs: Vec<Msg> = Vec::new();
            match intent {
                Intent::DialogueOpen(n) => msgs.push(ad.dialogue.open(&mut ad.own.dialogue, n)),
                Intent::DialoguePick(i) => msgs.extend(ad.dialogue.pick(&mut ad.own.dialogue, i)),
                Intent::DialogueLineDone => {
                    msgs.extend(ad.dialogue.line_done(&mut ad.own.dialogue))
                }
                Intent::DialogueClose => msgs.extend(ad.dialogue.close(&mut ad.own.dialogue)),
                Intent::LootOpen(c) => msgs.push(ad.loot.host_open(c)),
                Intent::LootClose(c) => msgs.extend(ad.own.loot.close(cf(c)).map(Msg::Loot)),
                // the host edits the world itself; `reconcile` follows it
                Intent::LootTake { .. } | Intent::LootPut { .. } => {}
                Intent::ClaimDrop(d) => msgs.push(coop_core::proto::Msg::Loot(
                    coop_core::loot::LootMsg::ClaimDrop { drop: cf(d) },
                )),
                Intent::Say(t) => msgs.push(ad.own.say(ChatChannel::All, &t)),
                Intent::Vote(k, p) => msgs.push(ad.rules.request_vote(k, p)),
                Intent::ReviveBegin(p) => msgs.push(ad.rules.revive_begin(p)),
                Intent::ReviveCancel => msgs.push(ad.rules.revive_cancel()),
                Intent::Answer(y) => msgs.extend(ad.own.rules.answer(y).map(Msg::Rules)),
                other => msgs.extend(crate::live_extra::social_msg(other, PlayerId::HOST)),
            }
            for m in msgs {
                for (to, m) in ad.on_local(&mut h.sync.live, &mut h.sync.auth, now, m) {
                    send_msg(&mut self.net, to, &m);
                }
            }
        } else if let (Some(g), Some((_, ad))) = (&mut self.guest, l.guest.as_mut()) {
            let live = &mut g.sync.live;
            let mut msgs: Vec<Msg> = Vec::new();
            match intent {
                Intent::DialogueOpen(n) => msgs.push(ad.dialogue.open(&mut live.dialogue, n)),
                Intent::DialoguePick(i) => msgs.extend(ad.dialogue.pick(&mut live.dialogue, i)),
                Intent::DialogueLineDone => msgs.extend(ad.dialogue.line_done(&mut live.dialogue)),
                Intent::DialogueClose => msgs.extend(ad.dialogue.close(&mut live.dialogue)),
                Intent::LootOpen(c) => msgs.push(ad.loot.open(&live.loot, c)),
                Intent::LootTake { container, item, n } => {
                    msgs.extend(ad.loot.take(&mut live.loot, container, item, n))
                }
                Intent::LootPut { container, item, n } => {
                    msgs.extend(ad.loot.put(&mut live.loot, container, item, n))
                }
                Intent::LootClose(c) => msgs.extend(ad.loot.close(&mut live.loot, c)),
                Intent::ClaimDrop(d) => msgs.push(ad.loot.claim(d)),
                Intent::Say(t) => msgs.push(live.say(ChatChannel::All, &t)),
                Intent::Vote(k, p) => msgs.push(ad.rules.request_vote(k, p)),
                Intent::ReviveBegin(p) => msgs.push(ad.rules.revive_begin(p)),
                Intent::ReviveCancel => msgs.push(ad.rules.revive_cancel()),
                Intent::Answer(y) => msgs.extend(live.rules.answer(y).map(Msg::Rules)),
                other => msgs.extend(crate::live_extra::social_msg(other, g.sync.player)),
            }
            for m in msgs {
                send_msg(&mut self.net, PlayerId::HOST, &m);
            }
        }
    }

    /// The per-frame driver of the live layer. Call after the local player's
    /// state (cell, position, heading, health, `gs.coop.live.local_*`) is in
    /// `gs`, before [`Session::post_frame`]. `dt` is the frame time in seconds.
    pub fn live_frame(&mut self, gs: &mut GameState, cx: Ctx, dt: f32) -> Option<&LiveView> {
        let now = self.now_ms;
        let mut names: BTreeMap<PlayerId, String> = BTreeMap::new();
        let me_name = self.name.clone();
        let mut me = PlayerId::HOST;
        if let Some(h) = &self.host {
            names.insert(PlayerId::HOST, me_name.clone());
            for (id, p) in &h.peers {
                names.insert(
                    *id,
                    p.name.clone().unwrap_or_else(|| format!("player {}", id.0)),
                );
            }
        }
        if let Some(g) = &self.guest {
            me = g.sync.player;
            for p in &g.roster {
                names.insert(p.player, p.name.clone());
            }
            names.insert(me, me_name);
        }
        self.live_setup(gs, Some(cx.order));
        self.flush_queued();
        let asked_time = gs.coop.live.scene.time_asks.len();
        let mut outs: Vec<(PlayerId, Msg)> = Vec::new();
        let mut to_host: Vec<Msg> = Vec::new();
        let Some(l) = self.live.as_mut() else {
            return None;
        };
        let mut view = LiveView {
            now_ms: now,
            me,
            ..LiveView::default()
        };
        if let (Some(h), Some(ad)) = (&mut self.host, l.host.as_mut()) {
            let live = &mut h.sync.live;
            for (id, n) in &names {
                ad.registry.join(*id, n);
                live.chat.set_name(*id, n);
            }
            let members = live.members().to_vec();
            for p in &members {
                if l.registered.insert(*p) {
                    gs.register_player(p.0);
                }
            }
            let gone: Vec<PlayerId> = l
                .registered
                .iter()
                .copied()
                .filter(|p| !members.contains(p))
                .collect();
            for p in gone {
                l.registered.remove(&p);
                gs.forget_player(p.0);
            }
            if let Some(gate) = gs.coop.gate.as_mut() {
                gate.set_present(members.iter().copied());
                gate.tick(dt);
            }
            outs.extend(ad.frame(cx, gs, live, &mut h.sync.auth, now));
            run_released(cx, gs);
            outs.extend(l.x.host_env(cx, gs, &members, now));
            view.members = members;
            collect_remote(&mut view, &mut ad.own, &names, now);
            crate::live_roam::host_frame(cx, gs, live, &mut view, now);
        } else if let (Some(g), Some((_, ad))) = (&mut self.guest, l.guest.as_mut()) {
            g.sync.live.scene.set_rtt(g.rtt_ms.unwrap_or(0));
            if g.sync.phase() != coop_core::sync::GuestPhase::Active
                && g.sync.live.scene.count() > 0
            {
                // the link to the host is down: no scene outlives it (a rejoin is replayed the scenes)
                g.sync.live.scene.abort_all();
            }
            to_host.extend(ad.frame(cx, gs, &mut g.sync.live, now));
            let mut members = vec![PlayerId::HOST, me];
            members.extend(g.roster.iter().map(|p| p.player));
            members.sort();
            members.dedup();
            view.members = members;
            view.hour = l.x.guest_env(cx, gs, &mut g.sync.live, now);
            collect_remote(&mut view, &mut g.sync.live, &names, now);
            crate::live_roam::guest_frame(gs, &mut g.sync.live, &mut view);
        }
        if asked_time > 0 {
            view.notices
                .push("Asking the party to pass the time".to_string());
        }
        view.scene = gs.coop.live.scene.view.clone();
        // fast travel the party must agree to
        let requests = std::mem::take(&mut gs.coop.live.travel_requests);
        for m in requests {
            let due = l
                .asked_travel
                .get(&m)
                .map_or(true, |t| now.saturating_sub(*t) > 10_000);
            if due {
                l.asked_travel.insert(m, now);
                let msg = world::coop::rules::RulesBridge::new().request_travel(m);
                if self.host.is_some() {
                    if let (Some(h), Some(ad)) = (&mut self.host, l.host.as_mut()) {
                        outs.extend(ad.on_local(&mut h.sync.live, &mut h.sync.auth, now, msg));
                    }
                } else {
                    to_host.push(msg);
                }
                view.notices.push("Asking the party to travel".to_string());
            }
        }
        if gs.coop.live.travel_allowed.is_some() {
            l.asked_travel.clear();
        }
        // presence changes
        let here: BTreeSet<PlayerId> = view.avatars.iter().map(|a| a.player).collect();
        view.left = l.present.difference(&here).copied().collect();
        l.present = here;
        // facts
        view.lines = std::mem::take(&mut gs.coop.live.lines);
        let f = &gs.coop.live;
        view.flags = LiveFlags {
            downed: f.downed,
            wiped: f.wiped,
            stage_held: f.stage_held,
            travel_allowed: f.travel_allowed,
            time_skip_ok: f.time_skip_ok,
            fork_result: f.fork_result,
        };
        if f.wiped && !l.was_wiped {
            view.notices.push("The whole party went down".to_string());
        }
        if f.stage_held && !l.was_held {
            view.notices
                .push("The story waits for the whole party".to_string());
        }
        l.was_wiped = f.wiped;
        l.was_held = f.stage_held;
        l.view = view;
        for (to, m) in outs {
            send_msg(&mut self.net, to, &m);
        }
        for m in to_host {
            send_msg(&mut self.net, PlayerId::HOST, &m);
        }
        self.live.as_ref().map(|l| &l.view)
    }
}

/// Fills avatars, actors, dialogue, chat and markers from a client view.
fn collect_remote(
    view: &mut LiveView,
    live: &mut GuestLive,
    names: &BTreeMap<PlayerId, String>,
    now: u64,
) {
    view.actions.extend(
        live.take_actions()
            .into_iter()
            .filter(|(p, _)| view.members.is_empty() || view.members.contains(p)),
    );
    let ids: Vec<PlayerId> = live.remote.keys().copied().collect();
    for p in ids {
        if !view.members.contains(&p) && !view.members.is_empty() {
            continue;
        }
        let parts = live.remote.get(&p).and_then(|r| r.parts.clone());
        let Some(s) = live.sample(p, now) else {
            continue;
        };
        view.avatars.push(RemoteAvatarView {
            player: p,
            name: chat_name(names, p),
            puppet: puppet_of(p),
            cell: s.cell.0,
            pos: s.pos,
            heading: s.heading,
            vel: s.vel,
            anim: s.anim,
            weapon: s.weapon,
            health: s.health,
            downed: s.flags & FLAG_DOWNED != 0,
            dead: s.flags & FLAG_DEAD != 0,
            sneaking: s.flags & FLAG_SNEAK != 0,
            drawn: s.flags & FLAG_DRAWN != 0,
            teleported: s.teleported,
            equipment: parts
                .as_ref()
                .map(|x| x.equipment.iter().map(|(s, f)| (*s, wf(*f))).collect())
                .unwrap_or_default(),
            parts_rev: parts.map_or(0, |x| x.rev),
        });
    }
    view.prompt = live.rules.prompt.clone();
    let me = view.me;
    for (id, rec) in live.actors.iter() {
        if rec.owner == Some(me) {
            continue;
        }
        let Some(st) = rec.state else { continue };
        // nobody shares its place: every peer runs it for itself
        if rec.owner.is_none() {
            continue;
        }
        view.remote_actors.push(ActorView {
            id: wf(*id),
            cell: st.cell.0,
            pos: [
                dequant_pos(st.pos[0]),
                dequant_pos(st.pos[1]),
                dequant_pos(st.pos[2]),
            ],
            heading: dequant_heading(st.heading),
            anim: st.anim,
            health: st.health,
            target: st.target,
            dead: !rec.alive || st.flags & 1 != 0,
        });
    }
    view.owned_actors = live.actors.owned().into_iter().map(wf).collect();
    view.actor_table = live
        .actors
        .iter()
        .map(|(id, rec)| ActorRow {
            id: wf(*id),
            owner: rec.owner,
            alive: rec.alive,
        })
        .collect();
    view.spawned = live
        .actors
        .spawn_at
        .iter()
        .filter_map(|(id, (cell, pos))| {
            let base = *live.actors.spawned.get(id)?;
            Some(SpawnedView {
                id: wf(*id),
                base: wf(base),
                cell: cell.0,
                pos: [
                    dequant_pos(pos[0]),
                    dequant_pos(pos[1]),
                    dequant_pos(pos[2]),
                ],
            })
        })
        .collect();
    if let Some(v) = live.dialogue.view() {
        view.dialogue = Some(DialogueView {
            session: v.session,
            npc: wf(v.npc),
            initiator: v.initiator,
            mine: live.dialogue.is_initiator(),
            playing: v.playing.map(FormId),
        });
    }
    for c in live.chat_log.drain(..) {
        view.chat.push(ChatView {
            from: c.from,
            name: c.name,
            text: c.text,
        });
    }
    for id in std::mem::take(&mut live.chat_rejected) {
        view.notices.push(format!("Message {id} was refused"));
    }
    crate::live_extra::drain_voice(view, live);
    for ((owner, _), m) in &live.markers {
        view.markers.push(MarkerView {
            owner: *owner,
            kind: m.kind,
            cell: m.cell.0,
            pos: [
                dequant_pos(m.pos[0]),
                dequant_pos(m.pos[1]),
                dequant_pos(m.pos[2]),
            ],
            label: m.label.clone(),
        });
    }
}

/// Re-runs the stages the gate released, and those it sent to one player's
/// shadow.
fn run_released(cx: Ctx, gs: &mut GameState) {
    let Some(mut gate) = gs.coop.gate.take() else {
        return;
    };
    let released = gate.poll(gs);
    let redirects = gate.take_redirects();
    gs.coop.gate = Some(gate);
    for r in released {
        let slot = r.stage.initiator.unwrap_or(0);
        gs.with_player(slot, |g| {
            Runner::new(cx.order, cx.scripts, g).set_stage(r.stage.quest, r.stage.stage);
        });
    }
    for r in redirects {
        gs.with_player(r.slot, |g| {
            Runner::new(cx.order, cx.scripts, g).set_stage(r.quest, r.stage);
        });
    }
}
