//! Combat helpers on the session: an encounter's spawns go through the
//! host's spawn arbiter (where the party scaling adds its extras).

use std::sync::atomic::{AtomicU32, Ordering};

use coop_core::actors::ActorMsg;
use coop_core::avatar::quant_pos;
use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use esm::FormId;

use crate::session::send_msg;
use crate::{Role, Session};

static NEXT_SITE: AtomicU32 = AtomicU32::new(1);

impl Session {
    /// Host: sets the player-versus-player rule (0 off, 1 on, 2 half).
    pub fn live_friendly_fire(&mut self, mode: u8) {
        use coop_core::combat::FriendlyFire;
        if let Some(h) = self.host.as_mut() {
            h.sync.live.combat.vitals.friendly = match mode {
                0 => FriendlyFire::Off,
                1 => FriendlyFire::On,
                _ => FriendlyFire::Scaled(0.5),
            };
        }
    }

    /// Asks the host to create `n` creatures of `base` around `pos` in
    /// `cell`, each at its own spawn site, the way a rolled encounter does.
    /// The host decides each ref's id and, with a party, which of them bring
    /// an extra. Returns the number of requests sent.
    pub fn live_spawn_encounter(&mut self, base: FormId, cell: FormId, pos: [f32; 3], n: u32, health: u16) -> u32 {
        let now = self.now_ms;
        let fixed = [quant_pos(pos[0]), quant_pos(pos[1]), quant_pos(pos[2])];
        let mut sent = 0;
        for _ in 0..n {
            let site = CForm(0xFC00_0000 | (NEXT_SITE.fetch_add(1, Ordering::Relaxed) & 0x00FF_FFFF));
            let Some(l) = self.live.as_mut() else { return sent };
            if let (Role::Host, Some(h), Some(ad)) = (self.role, self.host.as_mut(), l.host.as_mut()) {
                let (_, m) = ad.own.actors.request_spawn(site, CForm(base.0), CellId(cell.0), fixed, health);
                for (to, m) in ad.on_local(&mut h.sync.live, &mut h.sync.auth, now, Msg::Actor(m)) {
                    send_msg(&mut self.net, to, &m);
                }
                sent += 1;
            } else if let (Some(g), true) = (self.guest.as_mut(), self.role == Role::Guest) {
                let (_, m) = g.sync.live.actors.request_spawn(site, CForm(base.0), CellId(cell.0), fixed, health);
                let _: &ActorMsg = &m;
                send_msg(&mut self.net, PlayerId::HOST, &Msg::Actor(m));
                sent += 1;
            }
        }
        sent
    }
}
