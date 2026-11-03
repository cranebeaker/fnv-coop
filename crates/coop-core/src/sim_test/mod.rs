//! Deterministic multi-peer simulation: a fake world, one host, N guests,
//! every link wrapped in a [`ChaosTransport`] on one virtual clock.

#[cfg(test)]
mod live;
#[cfg(test)]
mod live_combat;
#[cfg(test)]
mod live_more;
#[cfg(test)]
mod roam;
#[cfg(test)]
mod tests;

use crate::hash::ShardHashes;
use crate::ids::{FormId, PlayerId};
use crate::proto::{self, Hello, Msg, PROTOCOL_VERSION};
use crate::schema::Field;
use crate::sync::{GuestPhase, GuestSync, HostConfig, HostSync};
use crate::transport::{
    ChaosProfile, ChaosTransport, LoopbackEndpoint, LoopbackNet, SeededRng, Transport, VirtualClock,
};

/// Milliseconds of virtual time per step.
pub const STEP_MS: u64 = 25;
/// Steps per host tick (so 50 ms ticks).
pub const STEPS_PER_TICK: u64 = 2;
/// Ticks between hash broadcasts.
pub const HASH_EVERY_TICKS: u64 = 10;

type Net = ChaosTransport<LoopbackEndpoint>;

/// Identity every well-behaved peer uses.
pub fn test_hello() -> Hello {
    Hello {
        proto: PROTOCOL_VERSION,
        build_hash: 0xB001,
        rules_hash: 0x201E5,
        mods_hash: 0x30D5,
        exe: String::new(),
        volatile_hash: 0,
        plugins: Vec::new(),
    }
}

/// One simulated guest.
pub struct SimGuest {
    /// Protocol state machine.
    pub sync: GuestSync,
    /// Its outgoing link.
    pub net: Net,
    /// Offline guests ignore and discard everything.
    pub online: bool,
}

/// The whole simulated session.
pub struct Sim {
    /// Shared virtual clock.
    pub clock: VirtualClock,
    /// The host.
    pub host: HostSync,
    /// Host's outgoing link.
    pub host_net: Net,
    /// Guests, player ids 1..=n.
    pub guests: Vec<SimGuest>,
    /// Mutation randomness.
    pub rng: SeededRng,
    /// Steps taken.
    pub steps: u64,
    /// Messages that failed to decode.
    pub decode_errors: u64,
    /// Forms the fake world mutates.
    pub universe: Vec<FormId>,
}

impl Sim {
    /// Builds a session with `n_guests` (not started) and `profile` on every
    /// link, all derived from `seed`.
    pub fn new(n_guests: u8, profile: ChaosProfile, seed: u64) -> Sim {
        let clock = VirtualClock::new();
        let net = LoopbackNet::new();
        let host_ep = net.endpoint(PlayerId::HOST);
        let mut cfg = HostConfig::new(test_hello(), 0x5E55);
        cfg.max_guests = n_guests as usize;
        let host_net = ChaosTransport::new(host_ep, clock.clone(), profile, seed ^ 0xA1);
        let guests = (1..=n_guests)
            .map(|i| {
                let ep = net.endpoint(PlayerId(i));
                SimGuest {
                    sync: GuestSync::new(PlayerId(i), test_hello()),
                    net: ChaosTransport::new(ep, clock.clone(), profile, seed ^ (0xB0 + i as u64)),
                    online: false,
                }
            })
            .collect();
        let universe = (0..120u32).map(|i| FormId(0x0100_0000 + i * 7)).collect();
        Sim {
            clock,
            host: HostSync::new(cfg),
            host_net,
            guests,
            rng: SeededRng::new(seed),
            steps: 0,
            decode_errors: 0,
            universe,
        }
    }

    /// Starts guest `i` (0-based) with a fresh join.
    pub fn start_guest(&mut self, i: usize) {
        let msgs = self.guests[i].sync.start();
        self.guests[i].online = true;
        send_all(&mut self.guests[i].net, PlayerId::HOST, msgs);
    }

    /// Cuts guest `i` off: it ignores traffic until [`Sim::reconnect_guest`].
    pub fn drop_guest(&mut self, i: usize) {
        self.guests[i].online = false;
    }

    /// Brings guest `i` back, keeping its state, and resumes.
    pub fn reconnect_guest(&mut self, i: usize) {
        let msgs = self.guests[i].sync.reconnect();
        self.guests[i].online = true;
        send_all(&mut self.guests[i].net, PlayerId::HOST, msgs);
    }

    /// Sends messages from guest `i` to the host.
    pub fn guest_emit(&mut self, i: usize, msgs: Vec<Msg>) {
        send_all(&mut self.guests[i].net, PlayerId::HOST, msgs);
    }

    /// Sends host outputs to their recipients.
    pub fn host_emit(&mut self, outs: Vec<(PlayerId, Msg)>) {
        for (to, m) in outs {
            send(&mut self.host_net, to, &m);
        }
    }

    /// Feeds a live-layer message from the host player's own game.
    pub fn host_local(&mut self, msg: Msg) {
        let outs = self.host.on_local(msg);
        self.host_emit(outs);
    }

    /// The host drops guest `i` for good (connection lost) and the guest goes
    /// silent.
    pub fn kick_guest(&mut self, i: usize) {
        self.host.disconnect(PlayerId(i as u8 + 1));
        self.guests[i].online = false;
    }

    /// Steps until `cond` holds or `max_ms` of virtual time passes.
    pub fn run_until(&mut self, max_ms: u64, mut cond: impl FnMut(&mut Sim) -> bool) -> bool {
        let deadline = self.clock.now() + max_ms;
        while self.clock.now() < deadline {
            if cond(self) {
                return true;
            }
            self.step();
        }
        cond(self)
    }

    /// One random host-side world mutation.
    pub fn mutate(&mut self) {
        let form = self.universe[self.rng.below(self.universe.len() as u64) as usize];
        let v = self.rng.next_u64();
        let field = match v % 7 {
            0 => Field::QuestStage {
                stage: (v >> 8) as u16 % 100,
            },
            1 => Field::QuestFlags {
                flags: (v >> 8) as u32 & 0xFF,
            },
            2 => Field::Objective {
                index: ((v >> 8) % 6) as u16,
                state: ((v >> 16) % 4) as u8,
            },
            3 => Field::global(((v >> 8) % 1000) as f32 / 4.0),
            4 => Field::ItemCount {
                item: FormId(0x0200_0000 + ((v >> 8) % 20) as u32),
                count: ((v >> 20) % 50) as i32,
            },
            5 => Field::RefDisabled {
                disabled: v & 0x100 != 0,
            },
            _ => Field::script_var(((v >> 8) % 8) as u16, ((v >> 16) % 100) as f64),
        };
        self.host.write(form, field);
    }

    /// Advances virtual time by one step and runs every peer once.
    pub fn step(&mut self) {
        self.clock.advance(STEP_MS);
        self.steps += 1;
        let now_ms = self.clock.now();
        self.host.set_time_ms(now_ms);
        for g in &mut self.guests {
            g.sync.set_time_ms(now_ms);
        }
        self.host_net.pump();
        for g in &mut self.guests {
            g.net.pump();
        }
        // host receives
        while let Some(p) = self.host_net.recv() {
            match proto::decode(p.channel, &p.data) {
                Ok(m) => {
                    let replies = self.host.on_msg(p.from, m);
                    for (to, m) in replies {
                        send(&mut self.host_net, to, &m);
                    }
                }
                Err(_) => self.decode_errors += 1,
            }
        }
        if self.steps % STEPS_PER_TICK == 0 {
            let tick = self.host.tick_no().0 as u64 + 1;
            for (to, m) in self.host.tick() {
                send(&mut self.host_net, to, &m);
            }
            if tick % HASH_EVERY_TICKS == 0 {
                for (to, m) in self.host.hash_broadcast() {
                    send(&mut self.host_net, to, &m);
                }
            }
        }
        // guests
        let now = self.host.tick_no();
        for g in &mut self.guests {
            while let Some(p) = g.net.recv() {
                if !g.online {
                    continue;
                }
                match proto::decode(p.channel, &p.data) {
                    Ok(m) => {
                        let out = g.sync.on_msg(m);
                        send_all(&mut g.net, PlayerId::HOST, out);
                    }
                    Err(_) => self.decode_errors += 1,
                }
            }
            if g.online && self.steps % STEPS_PER_TICK == 0 {
                let out = g.sync.tick(now);
                send_all(&mut g.net, PlayerId::HOST, out);
            }
        }
    }

    /// Runs `steps` steps, mutating `per_step` times each step.
    pub fn run(&mut self, steps: u64, per_step: u32) {
        for _ in 0..steps {
            for _ in 0..per_step {
                self.mutate();
            }
            self.step();
        }
    }

    /// True when every online guest is active and byte-identical to the host.
    pub fn converged(&self) -> bool {
        let host_entries = self.host.store.entries();
        self.guests.iter().filter(|g| g.online).all(|g| {
            g.sync.phase() == GuestPhase::Active
                && g.sync.store.rev() == self.host.store.rev()
                && g.sync.store.entries() == host_entries
        })
    }

    /// Steps (without mutating) until converged or `max_ms` of virtual time.
    pub fn run_until_converged(&mut self, max_ms: u64) -> bool {
        let deadline = self.clock.now() + max_ms;
        while self.clock.now() < deadline {
            if self.converged() {
                return true;
            }
            self.step();
        }
        self.converged()
    }

    /// Root hash of the host state.
    pub fn host_root(&self) -> u64 {
        ShardHashes::compute(&self.host.store).root()
    }
}

fn send(net: &mut Net, to: PlayerId, m: &Msg) {
    if let Ok(bytes) = proto::encode(m) {
        let _ = net.send(to, m.channel(), bytes);
    }
}

fn send_all(net: &mut Net, to: PlayerId, msgs: Vec<Msg>) {
    for m in msgs {
        send(net, to, &m);
    }
}
