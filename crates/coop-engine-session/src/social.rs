//! The session's part in conversations, party saves and rests (docs/coop/SPECS.md S14 to S17):
//! one step for the host and one for a guest, called by the drivers between two of the game's
//! frames. The pieces are [`crate::talk`], [`crate::party`] and [`crate::rest`].
//!
//! On the host with no guest nothing is shared: its conversations and rests are the game's own,
//! as without co-op.

use coop_core::ids::{Epoch, PlayerId};
use coop_core::proto::{Msg, MAX_INTENT_CHANGES};
use coop_core::schema::Field;
use coop_core::sync::GuestPhase;
use coop_core::transport::Transport;
use coop_engine::{rest as game_rest, saves, talk as game_talk};
use engine_core::Engine;

use crate::party::{PartyGuest, PartyHost, PART_UP_TAG};
use crate::rest::Rester;
use crate::talk::{Enactor, Talker};
use crate::{send, GuestDriver, HostDriver};

/// The host's state.
#[derive(Debug, Default)]
pub struct HostSocial {
    /// The host player's own conversations and the ones it watches.
    pub talker: Talker,
    /// The guests' lines the host's game runs.
    pub enactor: Enactor,
    /// The guests' parts and the party's files.
    pub party: PartyHost,
    /// The rest vote.
    pub rester: Rester,
}

/// A guest's state.
#[derive(Debug, Default)]
pub struct GuestSocial {
    /// This player's own conversations and the ones it watches.
    pub talker: Talker,
    /// This player's part.
    pub party: PartyGuest,
    /// The rest vote.
    pub rester: Rester,
}

/// Feeds a message of the host player's own game to the host's live layer and sends what it
/// answers to the guests (the host's own copies wait in the local inbox).
fn local(h: &mut HostDriver, net: &mut dyn Transport, msg: Msg) {
    for (to, m) in h.sync.on_local(msg) {
        send(net, to, &m, &mut h.send_failures);
    }
}

/// The host's step: after the guests' messages were taken, before their claims are enacted and
/// the game's changes collected.
pub(crate) fn host_step(h: &mut HostDriver, e: &mut Engine, net: &mut dyn Transport, now_ms: u64) {
    h.sync.claim_tags.insert(PART_UP_TAG);
    // What the live layer addressed to the host player.
    for msg in h.sync.live.take_local() {
        h.view.on_msg(msg, now_ms);
    }
    let players = h.sync.players();
    let shared = !players.is_empty();
    game_talk::set_sharing(e, shared);
    game_rest::set_sharing(e, shared);

    // The guests' parts: taken out of the claims before the driver judges the rest.
    let claims = std::mem::take(&mut h.sync.claims);
    for (from, form, field) in claims {
        if matches!(&field, Field::Custom { tag, .. } if *tag == PART_UP_TAG) {
            h.social.party.on_claim(from, &field);
        } else {
            h.sync.claims.push((from, form, field));
        }
    }
    let mut writes = h.social.party.on_players(&players);
    for ev in saves::take_events(e) {
        match ev {
            saves::Event::Saved(name) => h.social.party.on_saved(&name),
            saves::Event::Loaded(name) => writes.extend(h.social.party.on_loaded(&name)),
        }
    }
    for (form, field) in writes {
        h.sync.write(form, field);
    }

    // Conversations: the host player's own, the ones it watches, and the guests' results.
    let quests = &h.quests;
    let share = |e: &mut Engine, info: u32, _held: bool| {
        game_talk::info_quest(e, info).is_some_and(|q| quests.contains(&q))
    };
    let msgs = h
        .social
        .talker
        .step(e, PlayerId::HOST, &mut h.view.dialogue, &share);
    for m in msgs {
        local(h, net, Msg::Dialogue(m));
    }
    for ev in h.sync.live.dialogue.take_events() {
        h.social.enactor.on_event(e, ev, &h.quests);
    }

    // The rest vote.
    let (msgs, passed) = h.social.rester.step(e, &mut h.view.rules);
    for m in msgs {
        local(h, net, Msg::Rules(m));
    }
    if passed {
        // The new time goes to the guests now, not at the clock's next turn.
        h.clock_sent_ms = None;
    }
}

/// A guest's step: after the host's messages were taken.
pub(crate) fn guest_step(
    g: &mut GuestDriver,
    e: &mut Engine,
    net: &mut dyn Transport,
    now_ms: u64,
) {
    let active = g.sync.phase() == GuestPhase::Active;
    game_talk::set_sharing(e, active);
    game_rest::set_sharing(e, active);
    if !active {
        return;
    }
    let me = g.sync.player;
    let share = |_: &mut Engine, _info: u32, held: bool| held;
    let msgs = g
        .social
        .talker
        .step(e, me, &mut g.sync.live.dialogue, &share);
    for m in msgs {
        send(net, PlayerId::HOST, &Msg::Dialogue(m), &mut g.send_failures);
    }
    let (msgs, _) = g.social.rester.step(e, &mut g.sync.live.rules);
    for m in msgs {
        send(net, PlayerId::HOST, &Msg::Rules(m), &mut g.send_failures);
    }

    // This player's part: a saved one is put back first; then the game's own goes to the host.
    g.social.party.restore(
        e,
        g.sync
            .store
            .iter()
            .map(|(k, entry)| (k.form.0, &entry.field)),
    );
    let claims = g.social.party.upload(e, now_ms);
    for batch in claims.chunks(MAX_INTENT_CHANGES) {
        for msg in g.sync.send_intent(Epoch(0), batch.to_vec()) {
            send(net, PlayerId::HOST, &msg, &mut g.send_failures);
        }
    }
}
