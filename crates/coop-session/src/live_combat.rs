//! Combat helpers on the session: an encounter's spawns go through the
//! host's spawn arbiter (where the party scaling adds its extras).

use coop_core::actors::ActorMsg;
use coop_core::avatar::quant_pos;
use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use esm::FormId;

use crate::session::send_msg;
use crate::{Role, Session};

/// The size of the square (in game units) that one encounter site covers.
const SITE_GRID: f32 = 512.0;

/// The spawn site of creature `i` of an encounter of `base` at `pos` in
/// `cell`. Every machine computes the same id for the same encounter, so two
/// players who ask for one encounter name one site, and the host's arbiter
/// grants it once (the other request is denied as `SiteTaken`).
pub fn encounter_site(base: FormId, cell: FormId, pos: [f32; 3], i: u32) -> CForm {
    let square = |v: f32| (v / SITE_GRID).floor() as i32 as u32;
    let mut h: u32 = 0x811c_9dc5;
    for w in [
        cell.0,
        base.0,
        square(pos[0]),
        square(pos[1]),
        square(pos[2]),
        i,
    ] {
        for b in w.to_le_bytes() {
            h ^= u32::from(b);
            h = h.wrapping_mul(0x0100_0193);
        }
    }
    CForm(0xFC00_0000 | (h & 0x00FF_FFFF))
}

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
            // the scaling announcement tells the guests
            h.sync.live.rules.friendly_fire = mode != 0;
        }
    }

    /// Host: the campaign rules in force (the difficulty mode and friendly
    /// fire). `None` on a guest.
    pub fn rules(&self) -> Option<world::coop::party_save::PartyRules> {
        let h = self.host.as_ref()?;
        Some(world::coop::party_save::PartyRules {
            mode: h.sync.live.rules.mode(),
            friendly_fire: h.sync.live.rules.friendly_fire,
        })
    }

    /// Host: puts campaign rules in force (the launcher or viewer flags, a
    /// loaded party save). The rules and the combat vitals change at once.
    /// The next scaling announcement tells the guests.
    pub fn set_rules(&mut self, r: world::coop::party_save::PartyRules) {
        if let Some(h) = self.host.as_mut() {
            h.sync.live.rules.set_mode(r.mode);
        }
        self.live_friendly_fire(u8::from(r.friendly_fire));
    }

    /// Asks the host to create `n` creatures of `base` around `pos` in
    /// `cell`, each at its own spawn site, the way a rolled encounter does.
    /// The sites come from [`encounter_site`]: a second player asking for the
    /// same encounter gets no second set.
    /// The host decides each ref's id and, with a party, which of them bring
    /// an extra. Returns the number of requests sent.
    pub fn live_spawn_encounter(
        &mut self,
        base: FormId,
        cell: FormId,
        pos: [f32; 3],
        n: u32,
        health: u16,
    ) -> u32 {
        let now = self.now_ms;
        let fixed = [quant_pos(pos[0]), quant_pos(pos[1]), quant_pos(pos[2])];
        let mut sent = 0;
        for i in 0..n {
            let site = encounter_site(base, cell, pos, i);
            let Some(l) = self.live.as_mut() else {
                return sent;
            };
            if let (Role::Host, Some(h), Some(ad)) =
                (self.role, self.host.as_mut(), l.host.as_mut())
            {
                let (_, m) =
                    ad.own
                        .actors
                        .request_spawn(site, CForm(base.0), CellId(cell.0), fixed, health);
                for (to, m) in ad.on_local(&mut h.sync.live, &mut h.sync.auth, now, Msg::Actor(m)) {
                    send_msg(&mut self.net, to, &m);
                }
                sent += 1;
            } else if let (Some(g), true) = (self.guest.as_mut(), self.role == Role::Guest) {
                let (_, m) = g.sync.live.actors.request_spawn(
                    site,
                    CForm(base.0),
                    CellId(cell.0),
                    fixed,
                    health,
                );
                let _: &ActorMsg = &m;
                send_msg(&mut self.net, PlayerId::HOST, &Msg::Actor(m));
                sent += 1;
            }
        }
        sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_encounter_names_one_site_on_every_machine() {
        let (base, cell) = (FormId(0x10A1FE), FormId(0x103D79));
        let host = encounter_site(base, cell, [2352.0, 1603.6, 7360.0], 0);
        let guest = encounter_site(base, cell, [2351.98, 1603.62, 7360.26], 0);
        assert_eq!(host, guest);
        assert_ne!(
            host,
            encounter_site(base, cell, [2352.0, 1603.6, 7360.0], 1)
        );
        assert_ne!(
            host,
            encounter_site(FormId(7), cell, [2352.0, 1603.6, 7360.0], 0)
        );
        assert_ne!(
            host,
            encounter_site(base, cell, [4000.0, 1603.6, 7360.0], 0)
        );
        assert_eq!(host.0 & 0xFF00_0000, 0xFC00_0000);
    }
}
