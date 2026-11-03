//! Scenarios with more than one guest (a 3 or 4 peer session) and the
//! bandwidth and CPU budget of every peer.
//!
//! The runner is the two peer one of [`crate::sim`] generalised: one host
//! and 1 to 3 guests, each guest on its own seeded chaos link to the host on
//! one virtual clock. Every link counts bytes and messages, and every peer's
//! frame is timed, so the same run that judges correctness also produces the
//! [`Budget`] report (`coop-sim budget`).
//!
//! # The budget
//!
//! Typical play, steady state, per guest (see docs/COOP.md "Budget"):
//!
//! * host to each guest: under [`Limits::guest_down_bps`] (64 KB/s);
//! * each guest to the host: under [`Limits::guest_up_bps`] (24 KB/s);
//! * the host's total upstream: under [`Limits::host_up_bps`] (192 KB/s,
//!   three guests);
//! * one peer's frame (session, live layer, scripts and bot, no rendering):
//!   mean under [`Limits::frame_mean_us`], 99th percentile under
//!   [`Limits::frame_p99_us`]. These are for release builds; a debug build
//!   is several times slower, so debug runs report CPU without gating it.
//!
//! "Steady" is the time after every peer is ready: the join snapshot is a
//! one-off and is reported separately.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{
    ChaosProfile, ChaosTransport, LoopbackEndpoint, LoopbackNet, Packet, Transport, TransportError,
    VirtualClock,
};
use coop_session::{Role, Session, SessionConfig};
use world::coop::apply::Ctx;

use crate::data::Data;
use crate::expect::{evaluate, Check};
use crate::json::{n, obj, s, J};
use crate::peer::Peer;
use crate::scenario::Scenario;
use crate::sim::{add_named_people, needs_assets, RunOpts, RunResult};
use crate::verdict::Verdict;

/// Documented typical limits (bytes per second, microseconds per frame).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Limits {
    /// Host to one guest.
    pub guest_down_bps: f64,
    /// One guest to the host.
    pub guest_up_bps: f64,
    /// Host to all guests together.
    pub host_up_bps: f64,
    /// Mean frame time (release builds).
    pub frame_mean_us: f64,
    /// 99th percentile frame time (release builds).
    pub frame_p99_us: f64,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            guest_down_bps: 64.0 * 1024.0,
            guest_up_bps: 24.0 * 1024.0,
            host_up_bps: 192.0 * 1024.0,
            frame_mean_us: 4_000.0,
            frame_p99_us: 16_000.0,
        }
    }
}

impl Limits {
    /// A scenario's `budget` object over the documented defaults.
    pub fn from_cfg(c: &crate::scenario::BudgetCfg) -> Limits {
        let d = Limits::default();
        let kb = |v: Option<f64>, def: f64| v.map_or(def, |x| x * 1024.0);
        Limits {
            guest_down_bps: kb(c.guest_down_kbps, d.guest_down_bps),
            guest_up_bps: kb(c.guest_up_kbps, d.guest_up_bps),
            host_up_bps: kb(c.host_up_kbps, d.host_up_bps),
            frame_mean_us: c.frame_mean_us.unwrap_or(d.frame_mean_us),
            frame_p99_us: c.frame_p99_us.unwrap_or(d.frame_p99_us),
        }
    }
}

/// The gate of a scenario `budget` key: one `budget` check per violation, or
/// one passing check that names the measured peak use. CPU is gated only when
/// the scenario gives a CPU key and this is a release build.
pub fn budget_checks(cfg: &crate::scenario::BudgetCfg, b: &Budget) -> Vec<Check> {
    let lim = Limits::from_cfg(cfg);
    let gate_cpu = cfg.gates_cpu() && !cfg!(debug_assertions);
    let v = b.violations(&lim, gate_cpu);
    if v.is_empty() {
        let worst_down = b
            .peers
            .iter()
            .skip(1)
            .map(|p| p.down_bps)
            .fold(0.0, f64::max);
        let worst_up = b.peers.iter().skip(1).map(|p| p.up_bps).fold(0.0, f64::max);
        let host_up = b.peers.first().map_or(0.0, |h| h.up_bps);
        return vec![Check {
            id: "budget".into(),
            ok: true,
            kind: "budget",
            detail: format!(
                "within budget over {:.0} steady s: guest down {:.1} KB/s, guest up {:.1} KB/s, host up {:.1} KB/s",
                b.steady_s,
                worst_down / 1024.0,
                worst_up / 1024.0,
                host_up / 1024.0
            ),
        }];
    }
    v.into_iter()
        .enumerate()
        .map(|(i, d)| Check {
            id: format!("budget#{}", i + 1),
            ok: false,
            kind: "budget",
            detail: d,
        })
        .collect()
}

/// Counters of one link endpoint.
#[derive(Default)]
pub struct LinkStats {
    /// Bytes handed to the link.
    pub up_bytes: u64,
    /// Messages handed to the link.
    pub up_msgs: u64,
    /// Bytes received.
    pub down_bytes: u64,
    /// Messages received.
    pub down_msgs: u64,
    /// Bytes of unreliable (datagram) traffic sent.
    pub up_datagram_bytes: u64,
    /// Switch to this profile at the next call.
    pub new_profile: Option<ChaosProfile>,
}

type Stats = Rc<RefCell<LinkStats>>;

/// A chaos link that counts.
pub struct Counted {
    chaos: ChaosTransport<LoopbackEndpoint>,
    stats: Stats,
}

impl Counted {
    fn sync(&mut self) {
        if let Some(p) = self.stats.borrow_mut().new_profile.take() {
            self.chaos.set_profile(p);
        }
    }
}

impl Transport for Counted {
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
        {
            let mut s = self.stats.borrow_mut();
            s.up_bytes += data.len() as u64;
            s.up_msgs += 1;
            if channel == Channel::Unreliable {
                s.up_datagram_bytes += data.len() as u64;
            }
        }
        self.chaos.send(to, channel, data)
    }

    fn recv(&mut self) -> Option<Packet> {
        self.sync();
        let p = self.chaos.recv();
        if let Some(p) = &p {
            let mut s = self.stats.borrow_mut();
            s.down_bytes += p.data.len() as u64;
            s.down_msgs += 1;
        }
        p
    }
}

/// What one peer cost.
#[derive(Clone, Debug, Default)]
pub struct PeerBudget {
    /// `host`, `guest`, `guest2`, ...
    pub name: String,
    /// Bytes sent before everyone was ready (join).
    pub join_up: u64,
    /// Bytes received before everyone was ready (join).
    pub join_down: u64,
    /// Steady bytes sent per second, average.
    pub up_bps: f64,
    /// Steady bytes received per second, average.
    pub down_bps: f64,
    /// Busiest steady second sent.
    pub peak_up_bps: f64,
    /// Busiest steady second received.
    pub peak_down_bps: f64,
    /// Steady datagram (unreliable) bytes sent per second.
    pub datagram_up_bps: f64,
    /// Steady messages sent per second.
    pub up_msgs_ps: f64,
    /// Frames timed.
    pub frames: u64,
    /// Mean frame time, microseconds.
    pub frame_mean_us: f64,
    /// 99th percentile frame time.
    pub frame_p99_us: f64,
    /// Slowest frame.
    pub frame_max_us: f64,
}

/// The whole report of a run.
#[derive(Clone, Debug, Default)]
pub struct Budget {
    /// Per peer, host first.
    pub peers: Vec<PeerBudget>,
    /// Steady seconds measured.
    pub steady_s: f64,
}

impl Budget {
    /// Problems against `lim`; empty when within budget. CPU is gated only
    /// when `gate_cpu` (release builds).
    pub fn violations(&self, lim: &Limits, gate_cpu: bool) -> Vec<String> {
        let mut v = Vec::new();
        let guests: Vec<&PeerBudget> = self.peers.iter().skip(1).collect();
        for g in &guests {
            if g.down_bps > lim.guest_down_bps {
                v.push(format!(
                    "{}: {:.1} KB/s in from the host, limit {:.0}",
                    g.name,
                    g.down_bps / 1024.0,
                    lim.guest_down_bps / 1024.0
                ));
            }
            if g.up_bps > lim.guest_up_bps {
                v.push(format!(
                    "{}: {:.1} KB/s out to the host, limit {:.0}",
                    g.name,
                    g.up_bps / 1024.0,
                    lim.guest_up_bps / 1024.0
                ));
            }
        }
        if let Some(h) = self.peers.first() {
            if h.up_bps > lim.host_up_bps {
                v.push(format!(
                    "host: {:.1} KB/s out in total, limit {:.0}",
                    h.up_bps / 1024.0,
                    lim.host_up_bps / 1024.0
                ));
            }
        }
        if gate_cpu {
            for p in &self.peers {
                if p.frame_mean_us > lim.frame_mean_us {
                    v.push(format!(
                        "{}: mean frame {:.0} us, limit {:.0}",
                        p.name, p.frame_mean_us, lim.frame_mean_us
                    ));
                }
                if p.frame_p99_us > lim.frame_p99_us {
                    v.push(format!(
                        "{}: p99 frame {:.0} us, limit {:.0}",
                        p.name, p.frame_p99_us, lim.frame_p99_us
                    ));
                }
            }
        }
        v
    }

    /// The text table.
    pub fn report(&self) -> String {
        let mut o = format!("budget over {:.0} steady seconds (bytes per second; join = one-off bytes before ready)\n", self.steady_s);
        o.push_str("peer      up KB/s  peak   dgram  msgs/s | down KB/s  peak | join up/down KB | frame us mean  p99   max\n");
        for p in &self.peers {
            o.push_str(&format!(
                "{:<8} {:>8.1} {:>6.1} {:>7.1} {:>7.0} | {:>9.1} {:>5.1} | {:>6.0}/{:<6.0} | {:>13.0} {:>5.0} {:>5.0}\n",
                p.name,
                p.up_bps / 1024.0,
                p.peak_up_bps / 1024.0,
                p.datagram_up_bps / 1024.0,
                p.up_msgs_ps,
                p.down_bps / 1024.0,
                p.peak_down_bps / 1024.0,
                p.join_up as f64 / 1024.0,
                p.join_down as f64 / 1024.0,
                p.frame_mean_us,
                p.frame_p99_us,
                p.frame_max_us
            ));
        }
        o
    }
}

/// Options of a multi-peer run.
#[derive(Clone, Debug, Default)]
pub struct MultiOpts {
    /// The ordinary run options (seed, net, fps, ...).
    pub run: RunOpts,
}

/// A finished multi-peer run.
pub struct MultiResult {
    /// The judgement.
    pub verdict: Verdict,
    /// Hash of all final dumps.
    pub hash: u64,
    /// Final dumps, host first.
    pub dumps: Vec<String>,
    /// Cost report.
    pub budget: Budget,
    /// Every check.
    pub checks: Vec<Check>,
    /// The flight recording, when `RunOpts::record` was set.
    pub recording: Option<crate::recorder::Recording>,
}

fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ *b as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn name_of(i: usize) -> String {
    match i {
        0 => "host".into(),
        1 => "guest".into(),
        n => format!("guest{n}"),
    }
}

/// Evaluates the scenario's checks over any number of dumps (host first).
/// A fifth peer that must be refused (`expect_refused`) is judged only by
/// checks `on` it (`guest4`); `equal`, `all`, `both` and the hash check cover
/// the party it was refused from.
pub fn evaluate_all(scn: &Scenario, all_dumps: &[J]) -> Vec<Check> {
    let mut out = Vec::new();
    let play = if scn.expect_refused {
        all_dumps.len().min(crate::scenario::MAX_GUESTS + 1)
    } else {
        all_dumps.len()
    };
    let dumps = &all_dumps[..play];
    let host = &dumps[0];
    for e in &scn.expect {
        let mut one = |on: &str, a: &J, b: &J, suffix: &str| {
            let mut e2 = e.clone();
            e2.on = on.to_string();
            for mut c in evaluate(&[e2], a, b) {
                if !suffix.is_empty() {
                    c.id = format!("{}{suffix}", c.id);
                }
                out.push(c);
            }
        };
        match e.on.as_str() {
            "equal" => {
                for (i, g) in dumps.iter().enumerate().skip(1) {
                    let suffix = if dumps.len() > 2 {
                        format!("#{i}")
                    } else {
                        String::new()
                    };
                    one("equal", host, g, &suffix);
                }
            }
            "guest2" | "guest3" | "guest4" => {
                let i = match e.on.as_str() {
                    "guest2" => 2,
                    "guest3" => 3,
                    _ => 4,
                };
                match all_dumps.get(i) {
                    Some(g) => one("guest", host, g, ""),
                    None => out.push(Check {
                        id: e.id.clone(),
                        ok: false,
                        kind: "assert",
                        detail: format!("{} needs a {}th peer", e.on, i + 1),
                    }),
                }
            }
            "all" => {
                one("host", host, &dumps[1], "");
                for (i, g) in dumps.iter().enumerate().skip(1) {
                    one("guest", host, g, &format!("@{}", name_of(i)));
                }
            }
            "both" => {
                one("host", host, &dumps[1], "@host");
                for (i, g) in dumps.iter().enumerate().skip(1) {
                    one("guest", host, g, &format!("@{}", name_of(i)));
                }
            }
            on => one(on, host, &dumps[1], ""),
        }
    }
    if scn.hash_check {
        let h = host.path("hash.shared");
        for (i, g) in dumps.iter().enumerate().skip(1) {
            let ok = h.is_some() && h == g.path("hash.shared");
            out.push(Check {
                id: format!("hash.shared#{i}"),
                ok,
                kind: "desync",
                detail: format!("host vs {} shared hash", name_of(i)),
            });
        }
    }
    out
}

fn percentile(sorted: &[u32], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let i = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    f64::from(sorted[i.min(sorted.len() - 1)])
}

struct Meter {
    stats: Stats,
    frame_us: Vec<u32>,
    at_ready: Option<(u64, u64, u64, u64)>,
    up_bins: BTreeMap<u64, u64>,
    down_bins: BTreeMap<u64, u64>,
    last: (u64, u64),
}

fn cfg(name: &str, t: Counted, side: &crate::scenario::Side) -> SessionConfig {
    SessionConfig {
        compat: coop_session::CompatConfig {
            exe: "1.4.0.525".into(),
            plugins: side.plugins.clone(),
            allow_mismatch: side.allow_mismatch,
            ..Default::default()
        },
        name: name.into(),
        session_id: 0xC00B,
        transport: Some(Box::new(t)),
        live: true,
        native_dialogue: true,
        // the viewer's `--coop-difficulty` and `--coop-friendly-fire` (host)
        rules: crate::sim::rules_of(&side.extra_args).ok().flatten(),
        ..SessionConfig::default()
    }
}

/// Runs a scenario with one host and `scn.guest_count()` guests.
pub fn run_multi(scn: &Scenario, d: &Arc<Data>, o: &MultiOpts) -> MultiResult {
    let ro = &o.run;
    let seed = ro.seed.unwrap_or(scn.seed);
    let mut scn = scn.clone();
    if let Some(nn) = &ro.net {
        scn.network = nn.clone();
    }
    let rerun = format!(
        "coop-sim run {} --seed {}{}",
        ro.path,
        seed,
        ro.net
            .as_ref()
            .map(|n| format!(" --net {n}"))
            .unwrap_or_default()
    );
    let fail = |scn: &Scenario, class: &str, why: String| {
        let json = obj(vec![
            ("scenario", s(scn.name.clone())),
            ("result", s("fail")),
            ("class", s(class)),
            ("reason", s(why.clone())),
            ("network", s(scn.network.clone())),
            ("seed", n(seed as f64)),
            ("duration_s", n(0.0)),
            ("rerun", s(rerun.clone())),
        ]);
        MultiResult {
            verdict: Verdict {
                json,
                result: "fail".into(),
                class: class.into(),
                reason: why,
            },
            hash: 0,
            dumps: vec![],
            budget: Budget::default(),
            checks: vec![],
            recording: None,
        }
    };
    // data the scenario needs: judged per side by the two peer rule
    let sides = scn.guest_sides();
    for (i, side) in sides.iter().enumerate() {
        let mut probe = scn.clone();
        probe.guest = (*side).clone();
        probe.guests.clear();
        if i > 0 {
            probe.host = crate::scenario::Side::default();
        }
        if let Some(why) = needs_assets(&probe, d) {
            let json = obj(vec![
                ("scenario", s(scn.name.clone())),
                ("result", s("skip")),
                ("class", s("needs_assets")),
                ("reason", s(why.clone())),
                ("network", s(scn.network.clone())),
                ("seed", n(seed as f64)),
                ("duration_s", n(0.0)),
                ("rerun", s(rerun.clone())),
            ]);
            return MultiResult {
                verdict: Verdict {
                    json,
                    result: "skip".into(),
                    class: "needs_assets".into(),
                    reason: why,
                },
                hash: 0,
                dumps: vec![],
                budget: Budget::default(),
                checks: vec![],
                recording: None,
            };
        }
    }
    let mut place = match d.place(scn.cell.as_deref().unwrap_or("")) {
        Ok(p) => p,
        Err(e) => return fail(&scn, "harness", e),
    };
    add_named_people(&mut place, &scn, d);
    let profile = ChaosProfile::by_name(&scn.network).unwrap_or(ChaosProfile::PERFECT);
    let net = LoopbackNet::new();
    let clock = VirtualClock::new();
    let mk = |id: PlayerId, salt: u64| {
        let stats: Stats = Rc::default();
        let t = Counted {
            chaos: ChaosTransport::new(
                net.endpoint(id),
                clock.clone(),
                profile,
                seed.wrapping_mul(0x9E37_79B9).wrapping_add(salt),
            ),
            stats: stats.clone(),
        };
        (t, stats)
    };
    let count = sides.len();
    let mut metas: Vec<Meter> = Vec::new();
    let mut peers: Vec<Peer> = Vec::new();
    let (ht, hs) = mk(PlayerId::HOST, 0x6A09);
    let host_sess = match Session::host(cfg("Host", ht, &scn.host)) {
        Ok(s) => s,
        Err(e) => return fail(&scn, "harness", format!("session: {e}")),
    };
    peers.push(Peer::new(
        Role::Host,
        host_sess,
        d,
        &place,
        &scn.host,
        !ro.talk_at_ready,
        ro.fixed_glue,
    ));
    metas.push(Meter {
        stats: hs,
        frame_us: Vec::new(),
        at_ready: None,
        up_bins: BTreeMap::new(),
        down_bins: BTreeMap::new(),
        last: (0, 0),
    });
    for (i, side) in sides.iter().enumerate() {
        let (gt, gs) = mk(PlayerId(i as u8 + 1), 0xBB67 + i as u64);
        let sess = match Session::join(cfg(&format!("Guest {}", i + 1), gt, side)) {
            Ok(s) => s,
            Err(e) => return fail(&scn, "harness", format!("session: {e}")),
        };
        peers.push(Peer::new(
            Role::Guest,
            sess,
            d,
            &place,
            side,
            !ro.talk_at_ready,
            ro.fixed_glue,
        ));
        metas.push(Meter {
            stats: gs,
            frame_us: Vec::new(),
            at_ready: None,
            up_bins: BTreeMap::new(),
            down_bins: BTreeMap::new(),
            last: (0, 0),
        });
    }
    // a fifth peer the full party must refuse (`expect_refused` with a fourth guest
    // entry): it starts once the four others are ready, and nobody waits for it
    let extra = (scn.expect_refused && count > crate::scenario::MAX_GUESTS).then_some(count);
    if ro.record {
        for p in peers.iter_mut() {
            p.flight = Some(Vec::new());
        }
    }
    let cx = Ctx {
        order: &d.order,
        scripts: &d.scripts,
    };
    let step_ms = (1000 / ro.fps.max(1)) as u64;
    let dt = step_ms as f32 / 1000.0;
    let launch_limit_s = 120.0;
    let mut now_ms = 0u64;
    let mut end_s: Option<f64> = None;
    let mut ready_s: Option<f64> = None;
    let mut armed_at: Option<f64> = None;
    let mut switched = scn.network == "perfect";
    loop {
        let t = now_ms as f64 / 1000.0;
        clock.advance(now_ms.saturating_sub(clock.now()));
        if armed_at.is_none() && metas.iter().skip(1).any(|m| m.stats.borrow().up_msgs > 0) {
            armed_at = Some(t);
        }
        if let Some(a) = armed_at {
            if !switched && t >= a + scn.chaos_seconds() {
                switched = true;
                for m in &metas {
                    m.stats.borrow_mut().new_profile = Some(ChaosProfile::PERFECT);
                }
            }
        }
        for (i, (p, m)) in peers.iter_mut().zip(metas.iter_mut()).enumerate() {
            if Some(i) == extra && ready_s.is_none() {
                // the fifth peer is not started until the party is full
                continue;
            }
            let t0 = Instant::now();
            p.frame(t, dt, d, cx);
            m.frame_us
                .push(t0.elapsed().as_micros().min(u128::from(u32::MAX)) as u32);
            let st = m.stats.borrow();
            let bin = t.floor() as u64;
            *m.up_bins.entry(bin).or_default() += st.up_bytes - m.last.0;
            *m.down_bins.entry(bin).or_default() += st.down_bytes - m.last.1;
            m.last = (st.up_bytes, st.down_bytes);
        }
        let playing_ready = peers
            .iter()
            .enumerate()
            .all(|(i, p)| Some(i) == extra || p.ready_at.is_some());
        if ready_s.is_none() && playing_ready {
            ready_s = Some(t);
            for m in metas.iter_mut() {
                let st = m.stats.borrow();
                m.at_ready = Some((st.up_bytes, st.down_bytes, st.up_datagram_bytes, st.up_msgs));
            }
        }
        if end_s.is_none() {
            if let Some(r) = peers
                .iter()
                .enumerate()
                .filter(|(i, _)| Some(*i) != extra)
                .map(|(_, p)| p.ready_at)
                .collect::<Option<Vec<f64>>>()
            {
                let latest = r.into_iter().fold(0.0f64, f64::max);
                end_s = Some(latest + scn.duration_s + scn.settle_s);
            }
        }
        now_ms += step_ms;
        if now_ms as f64 / 1000.0 >= end_s.unwrap_or(launch_limit_s) {
            break;
        }
    }
    let t = now_ms as f64 / 1000.0;
    for p in peers.iter_mut() {
        p.finish(t, d);
    }
    // budget
    let ready = ready_s.unwrap_or(0.0);
    let steady = (t - ready).max(0.001);
    let mut budget = Budget {
        peers: Vec::new(),
        steady_s: steady,
    };
    for (i, (p, m)) in peers.iter().zip(&metas).enumerate() {
        let st = m.stats.borrow();
        let (ru, rd, rdg, rm) =
            m.at_ready
                .unwrap_or((st.up_bytes, st.down_bytes, st.up_datagram_bytes, st.up_msgs));
        let rb = ready.floor() as u64 + 1;
        let peak = |bins: &BTreeMap<u64, u64>| {
            bins.iter()
                .filter(|(k, _)| **k >= rb)
                .map(|(_, v)| *v)
                .max()
                .unwrap_or(0) as f64
        };
        let mut f = m.frame_us.clone();
        f.sort_unstable();
        let mean = if f.is_empty() {
            0.0
        } else {
            f.iter().map(|x| f64::from(*x)).sum::<f64>() / f.len() as f64
        };
        budget.peers.push(PeerBudget {
            name: name_of(i),
            join_up: ru,
            join_down: rd,
            up_bps: (st.up_bytes - ru) as f64 / steady,
            down_bps: (st.down_bytes - rd) as f64 / steady,
            peak_up_bps: peak(&m.up_bins),
            peak_down_bps: peak(&m.down_bins),
            datagram_up_bps: (st.up_datagram_bytes - rdg) as f64 / steady,
            up_msgs_ps: (st.up_msgs - rm) as f64 / steady,
            frames: f.len() as u64,
            frame_mean_us: mean,
            frame_p99_us: percentile(&f, 0.99),
            frame_max_us: f.last().copied().map_or(0.0, f64::from),
        });
        let _ = p;
    }
    // judge
    let dumps: Vec<String> = peers.iter().map(|p| p.dump_text().to_string()).collect();
    let parsed: Vec<J> = dumps
        .iter()
        .map(|d| J::parse(d).unwrap_or(J::Null))
        .collect();
    let mut checks = if parsed.iter().all(|j| *j != J::Null) {
        evaluate_all(&scn, &parsed)
    } else {
        vec![]
    };
    if let Some(cfg) = &scn.budget {
        checks.extend(budget_checks(cfg, &budget));
    }
    let mut class: Option<(&str, String)> = None;
    if let Some(x) = extra {
        // the fifth peer's refusal is its end state, not an error (the checks judge it)
        peers[x]
            .errors
            .retain(|e| !e.contains("refused the connection"));
    }
    if let Some(i) = peers
        .iter()
        .enumerate()
        .position(|(i, p)| Some(i) != extra && p.ready_at.is_none())
    {
        class = Some(("timeout", format!("{} never became ready", name_of(i))));
    }
    if class.is_none() {
        if let Some((i, e)) = peers
            .iter()
            .enumerate()
            .find_map(|(i, p)| p.errors.first().map(|e| (i, e.clone())))
        {
            class = Some((
                "log",
                format!(
                    "{} logged error: {}",
                    name_of(i),
                    e.chars().take(120).collect::<String>()
                ),
            ));
        }
    }
    let checks_failed = checks.iter().find(|c| !c.ok).map(|c| {
        (
            match c.kind {
                "desync" => "desync",
                "budget" => "budget",
                _ => "assert",
            },
            format!("{}: {}", c.id, c.detail),
        )
    });
    if class.is_none() {
        class = checks_failed;
    }
    let twin = scn.expect_fail;
    let (result, class_s, reason) = match (&class, twin) {
        (None, false) => ("pass", "pass".to_string(), "all checks passed".to_string()),
        (None, true) => (
            "fail",
            "vacuous".to_string(),
            "the twin passed: its checks prove nothing".to_string(),
        ),
        (Some((c, why)), false) => ("fail", (*c).to_string(), why.clone()),
        (Some((c, why)), true) if matches!(*c, "assert" | "desync" | "budget") => {
            ("pass", "twin".to_string(), why.clone())
        }
        (Some((c, why)), true) => ("fail", (*c).to_string(), why.clone()),
    };
    let json = obj(vec![
        ("schema", n(1.0)),
        ("scenario", s(scn.name.clone())),
        ("result", s(result)),
        ("class", s(class_s.clone())),
        ("reason", s(reason.clone())),
        ("network", s(scn.network.clone())),
        ("seed", n(seed as f64)),
        ("duration_s", n((t * 10.0).round() / 10.0)),
        ("rerun", s(rerun)),
        ("engine", s("headless")),
        ("peers", n((peers.len()) as f64)),
        (
            "checks",
            J::Arr(
                checks
                    .iter()
                    .map(|c| {
                        obj(vec![
                            ("id", s(c.id.clone())),
                            ("ok", J::Bool(c.ok)),
                            ("detail", s(c.detail.chars().take(160).collect::<String>())),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]);
    let hash = dumps.iter().enumerate().fold(0u64, |h, (i, d)| {
        h ^ fnv(d.as_bytes()).rotate_left(i as u32)
    });
    let recording = ro.record.then(|| {
        let logs: Vec<crate::recorder::FlightLog> = peers
            .iter()
            .enumerate()
            .map(|(i, p)| (name_of(i), p.ready_at, p.flight.as_deref().unwrap_or(&[])))
            .collect();
        crate::recorder::Recording::from_peers(&scn.name, seed, &scn.network, &logs)
    });
    MultiResult {
        verdict: Verdict {
            json,
            result: result.into(),
            class: class_s,
            reason,
        },
        hash,
        dumps,
        budget,
        checks,
        recording,
    }
}

/// [`run_multi`] as the two peer runner's result type.
pub fn run_as_result(scn: &Scenario, d: &Arc<Data>, o: &RunOpts) -> RunResult {
    let r = run_multi(scn, d, &MultiOpts { run: o.clone() });
    let first = r.dumps.first().cloned().unwrap_or_default();
    let second = r.dumps.get(1).cloned().unwrap_or_default();
    RunResult {
        verdict: r.verdict,
        hash: r.hash,
        dumps: [first, second],
        recording: r.recording,
    }
}
