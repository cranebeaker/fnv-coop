//! The simulated link: coop-core's seeded chaos transport on a virtual clock,
//! plus what the real relay does that a profile does not: a profile switch at
//! a given time and scheduled outages.

use std::cell::RefCell;
use std::rc::Rc;

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{
    ChaosProfile, ChaosStats, ChaosTransport, LoopbackEndpoint, LoopbackNet, Packet, Transport,
    TransportError, VirtualClock,
};

/// Shared control of one direction of the link.
#[derive(Default)]
pub struct Ctl {
    /// Outgoing traffic of this endpoint is cut (an outage).
    pub cut: bool,
    /// Switch to this profile at the next send or receive.
    pub new_profile: Option<ChaosProfile>,
    /// Sends attempted.
    pub sent: u64,
    /// Sends swallowed by an outage.
    pub cut_drops: u64,
    /// Last chaos counters.
    pub stats: ChaosStats,
}

/// Handle to a [`Ctl`].
pub type CtlRef = Rc<RefCell<Ctl>>;

/// One endpoint of the simulated link.
pub struct SimLink {
    chaos: ChaosTransport<LoopbackEndpoint>,
    ctl: CtlRef,
    held: Vec<(PlayerId, Channel, Vec<u8>)>,
}

impl SimLink {
    /// After a heal, the reliable messages held back go out first, in order.
    fn flush_held(&mut self) {
        if !self.ctl.borrow().cut && !self.held.is_empty() {
            for (to, ch, data) in std::mem::take(&mut self.held) {
                let _ = self.chaos.send(to, ch, data);
            }
        }
    }

    fn sync(&mut self) {
        let mut c = self.ctl.borrow_mut();
        if let Some(p) = c.new_profile.take() {
            self.chaos.set_profile(p);
        }
        c.stats = self.chaos.stats();
    }
}

impl Transport for SimLink {
    fn local(&self) -> PlayerId {
        self.chaos.local()
    }

    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        self.sync();
        let cut = {
            let mut c = self.ctl.borrow_mut();
            c.sent += 1;
            c.cut
        };
        if cut {
            // like a dead link: datagrams are lost, the reliable stream waits for the heal
            match channel {
                Channel::Unreliable => self.ctl.borrow_mut().cut_drops += 1,
                Channel::ReliableOrdered => self.held.push((to, channel, data)),
            }
            return Ok(());
        }
        self.flush_held();
        let r = self.chaos.send(to, channel, data);
        self.sync();
        r
    }

    fn recv(&mut self) -> Option<Packet> {
        self.sync();
        self.flush_held();
        let p = self.chaos.recv();
        self.sync();
        p
    }
}

/// Both endpoints of a link and its control handles.
pub struct Link {
    /// The host's endpoint.
    pub host: SimLink,
    /// The guest's endpoint.
    pub guest: SimLink,
    /// Host to guest.
    pub h2g: CtlRef,
    /// Guest to host.
    pub g2h: CtlRef,
    /// The one clock both directions follow.
    pub clock: VirtualClock,
}

impl Link {
    /// A link with `profile`; each direction has its own seeded stream.
    pub fn new(profile: ChaosProfile, seed: u64) -> Link {
        let net = LoopbackNet::new();
        let clock = VirtualClock::new();
        let mk = |id: PlayerId, s: u64| {
            let ctl: CtlRef = Rc::default();
            let l = SimLink {
                chaos: ChaosTransport::new(net.endpoint(id), clock.clone(), profile, s),
                ctl: ctl.clone(),
                held: Vec::new(),
            };
            (l, ctl)
        };
        let (host, h2g) = mk(PlayerId::HOST, seed ^ 0x6A09_E667);
        let (guest, g2h) = mk(PlayerId(1), seed.wrapping_mul(0x9E37_79B9) ^ 0xBB67_AE85);
        Link {
            host,
            guest,
            h2g,
            g2h,
            clock,
        }
    }

    /// Moves the virtual clock to `ms`.
    pub fn advance_to(&self, ms: u64) {
        advance_clock(&self.clock, ms);
    }
}

/// Moves a virtual clock forward to `ms` (never back).
pub fn advance_clock(clock: &VirtualClock, ms: u64) {
    clock.advance(ms.saturating_sub(clock.now()));
}

/// Cuts or heals the directions an outage names.
pub fn set_cut(h2g: &CtlRef, g2h: &CtlRef, dir: &str, cut: bool) {
    if dir == "both" || dir == "h2g" {
        h2g.borrow_mut().cut = cut;
    }
    if dir == "both" || dir == "g2h" {
        g2h.borrow_mut().cut = cut;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_outage_drops_datagrams_and_holds_the_reliable_stream() {
        let mut l = Link::new(ChaosProfile::PERFECT, 1);
        set_cut(&l.h2g, &l.g2h, "both", true);
        l.host
            .send(PlayerId(1), Channel::Unreliable, vec![1])
            .unwrap();
        l.host
            .send(PlayerId(1), Channel::ReliableOrdered, vec![2])
            .unwrap();
        l.advance_to(50);
        assert!(l.guest.recv().is_none());
        set_cut(&l.h2g, &l.g2h, "both", false);
        l.host.recv();
        l.advance_to(100);
        let p = l
            .guest
            .recv()
            .expect("the held message arrives after the heal");
        assert_eq!(p.data, vec![2]);
        assert!(l.guest.recv().is_none(), "the datagram is gone");
        assert_eq!(l.h2g.borrow().cut_drops, 1);
    }

    #[test]
    fn a_profile_switch_applies_on_the_next_call() {
        let mut l = Link::new(ChaosProfile::FLAKY, 3);
        l.h2g.borrow_mut().new_profile = Some(ChaosProfile::PERFECT);
        for i in 0..200u8 {
            l.host
                .send(PlayerId(1), Channel::Unreliable, vec![i])
                .unwrap();
        }
        l.advance_to(10);
        l.host.recv();
        let n = std::iter::from_fn(|| l.guest.recv()).count();
        assert_eq!(n, 200, "perfect after the switch: nothing lost");
    }
}
