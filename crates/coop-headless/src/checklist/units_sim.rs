//! Checks over coop-core and coop-voice units: avatar interpolation, leveled picks
//! resolved once on the host, the voice pipeline.
//!
//! Every check takes `mutate`: with it set the check sabotages the thing under test and must
//! then fail (the mutation twin of the row).

use std::collections::{BTreeMap, BTreeSet};

use coop_core::actors::ActorClient;
use coop_core::avatar::interp::{DELAY_MS, EXTRAPOLATE_CAP_MS, TELEPORT_DIST};
use coop_core::avatar::{quant_pos, quant_vel, AvatarState, InterpBuffer, SEND_INTERVAL_MS};
use coop_core::ids::{CellId, FormId, PlayerId, Seq};
use coop_core::loot::LootHost;
use coop_core::proto::Msg;
use coop_core::sim_test::{Sim, STEPS_PER_TICK};
use coop_core::sync::GuestPhase;
use coop_core::transport::{ChaosProfile, SeededRng};
use coop_voice::codec::{Codec, CODEC_ADPCM, CODEC_OPUS};
use coop_voice::envelope::{
    DecodeError, VoicePacket, FLAG_END_OF_TALK, FLAG_PTT_ACTIVE, HEADER_LEN, MAX_PAYLOAD, VERSION,
};
use coop_voice::jitter::{JitterBuffer, Pop};
use coop_voice::pipeline::{
    default_codec, Activation, Positions, VoiceRx, VoiceTx, FAR, FLOOR, NEAR, OTHER_CELL_GAIN,
};

use super::builtin::{Check, Env};

/// The checks of this module by name.
pub fn registry() -> Vec<(&'static str, Check)> {
    vec![
        ("sim_avatar_interp", sim_avatar_interp),
        ("sim_leveled_once", sim_leveled_once),
        ("sim_leveled_loot_once", sim_leveled_loot_once),
        ("voice_envelope", voice_envelope),
        ("voice_vad", voice_vad),
        ("voice_ptt", voice_ptt),
        ("voice_jitter", voice_jitter),
        ("voice_mixer", voice_mixer),
    ]
}

fn profile(env: &Env) -> Result<ChaosProfile, String> {
    ChaosProfile::by_name(env.net).ok_or_else(|| format!("unknown profile {}", env.net))
}

/// A simulated session with every guest active: the party size of `env`, but at least
/// `min_guests` guests.
fn active_party(env: &Env, min_guests: u8) -> Result<Sim, String> {
    let n = (env.peers.clamp(2, 4) - 1).max(min_guests);
    let mut sim = Sim::new(n, profile(env)?, env.seed);
    for i in 0..n as usize {
        sim.start_guest(i);
    }
    let ok = sim.run_until(120_000, |s| {
        s.guests
            .iter()
            .all(|g| g.sync.phase() == GuestPhase::Active)
    });
    if !ok {
        return Err(format!("{n} guests did not become active on {}", env.net));
    }
    Ok(sim)
}

// ---------------------------------------------------------------------------
// world.interp

fn at(cell: u32, x: f32) -> AvatarState {
    AvatarState {
        cell: CellId(cell),
        pos: [quant_pos(x), 0, 0],
        ..Default::default()
    }
}

fn moving(cell: u32, x: f32, speed: f32) -> AvatarState {
    let mut s = at(cell, x);
    s.vel = [quant_vel(speed), 0, 0];
    s
}

/// Interpolation, the teleport threshold, the cell-change teleport and the
/// extrapolation cap on `InterpBuffer` (manual clock), then the same stream end to
/// end through the simulated session. Mutation: every sample of the walk is sent as
/// a teleport (the walker changes cell on each sample).
fn sim_avatar_interp(env: &Env, mutate: bool) -> Result<String, String> {
    // the thresholds are the documented ones (rustdoc of the constants)
    if TELEPORT_DIST != 4000.0
        || DELAY_MS != 100
        || EXTRAPOLATE_CAP_MS != 250
        || SEND_INTERVAL_MS != 50
    {
        return Err(format!(
            "thresholds changed: teleport {TELEPORT_DIST}, delay {DELAY_MS} ms, \
             extrapolation cap {EXTRAPOLATE_CAP_MS} ms, send interval {SEND_INTERVAL_MS} ms"
        ));
    }
    let walk_cell = |q: u32| if mutate { 1 + q % 2 } else { 1 };

    // 1. a walker at 100 units/s, 20 Hz, 30 ms latency, sampled every 7 ms
    let speed = 100.0f32;
    let lat = 30u64;
    let step = speed * SEND_INTERVAL_MS as f32 / 1000.0;
    let mut b = InterpBuffer::new();
    for q in 1..=40u32 {
        let t = q as u64 * SEND_INTERVAL_MS;
        b.push(
            t + lat,
            Seq(q),
            moving(walk_cell(q), speed * t as f32 / 1000.0, speed),
        );
    }
    let (mut n, mut between, mut last) = (0, 0, None::<f32>);
    for now in (1500..2030u64).step_by(7) {
        let s = b.sample(now).ok_or("the walker has no sample")?;
        if s.teleported || s.extrapolated {
            return Err(format!(
                "a small move snapped or extrapolated at {now} ms (teleported {}, extrapolated {})",
                s.teleported, s.extrapolated
            ));
        }
        let want = speed * (now as f32 - lat as f32 - DELAY_MS as f32) / 1000.0;
        if (s.pos[0] - want).abs() > 1.0 {
            return Err(format!(
                "at {now} ms the walker is drawn at {} instead of {want} ({DELAY_MS} ms behind)",
                s.pos[0]
            ));
        }
        if let Some(l) = last {
            let d = s.pos[0] - l;
            if !(-0.01..=speed * 7.0 / 1000.0 + 0.1).contains(&d) {
                return Err(format!("the walker jumped {d} units in 7 ms"));
            }
        }
        let r = s.pos[0] / step;
        if (r - r.round()).abs() > 0.05 {
            between += 1;
        }
        last = Some(s.pos[0]);
        n += 1;
    }
    if between * 4 < n * 3 {
        return Err(format!(
            "only {between} of {n} samples lie between two received positions"
        ));
    }

    // 2. a jump just under the threshold blends, just over it snaps at once
    let mut b = InterpBuffer::new();
    let under = TELEPORT_DIST - 1.0;
    b.push(50, Seq(1), at(1, 0.0));
    b.push(100, Seq(2), at(1, under));
    let s = b.sample(175).ok_or("no sample under the threshold")?;
    if b.teleports != 0 || s.teleported || !(s.pos[0] > 1.0 && s.pos[0] < under - 1.0) {
        return Err(format!(
            "a {under} unit jump must blend: teleports {}, drawn at {}",
            b.teleports, s.pos[0]
        ));
    }
    let mut b = InterpBuffer::new();
    let over = TELEPORT_DIST + 1.0;
    b.push(50, Seq(1), at(1, 0.0));
    b.push(100, Seq(2), at(1, over));
    let s = b.sample(175).ok_or("no sample over the threshold")?;
    let s2 = b.sample(200).ok_or("no second sample over the threshold")?;
    if b.teleports != 1 || !s.teleported || (s.pos[0] - over).abs() > 0.1 || s2.teleported {
        return Err(format!(
            "a {over} unit jump must snap once: teleports {}, drawn at {}, flagged {} then {}",
            b.teleports, s.pos[0], s.teleported, s2.teleported
        ));
    }

    // 3. a cell change is a teleport whatever the distance
    let mut b = InterpBuffer::new();
    b.push(50, Seq(1), at(1, 0.0));
    b.push(100, Seq(2), at(2, 5.0));
    let s = b.sample(175).ok_or("no sample after a cell change")?;
    if b.teleports != 1 || !s.teleported || s.cell != CellId(2) || (s.pos[0] - 5.0).abs() > 0.1 {
        return Err(format!(
            "a cell change must snap: teleports {}, cell {:?}, drawn at {}",
            b.teleports, s.cell, s.pos[0]
        ));
    }

    // 4. extrapolation past the newest sample is capped
    let mut b = InterpBuffer::new();
    for q in 1..=20u32 {
        let t = q as u64 * SEND_INTERVAL_MS;
        b.push(t, Seq(q), moving(1, 40.0 * t as f32 / 1000.0, 40.0));
    }
    let late = b
        .sample(1000 + DELAY_MS as u64 + 5000)
        .ok_or("no late sample")?;
    let capped = 40.0 + 40.0 * EXTRAPOLATE_CAP_MS as f32 / 1000.0;
    if !late.extrapolated || (late.pos[0] - capped).abs() > 1.0 {
        return Err(format!(
            "extrapolation must stop after {EXTRAPOLATE_CAP_MS} ms: drawn at {} (cap {capped})",
            late.pos[0]
        ));
    }

    // 5. end to end: guest 1 walks, guest 2 draws it, then guest 1 changes cell
    let mut sim = active_party(env, 2)?;
    sim.host.live.interest.link(CellId(1), CellId(2));
    sim.guests[1].sync.live.local = at(1, 0.0);
    let walker = PlayerId(1);
    // guest 1 spawns standing in cell 1; its first sample there is the spawn snap
    sim.guests[0].sync.live.local = at(1, 0.0);
    let placed = sim.run_until(60_000, |s| {
        let now = s.clock.now();
        s.guests[1]
            .sync
            .live
            .sample(walker, now)
            .is_some_and(|smp| smp.cell == CellId(1))
    });
    if !placed {
        return Err(format!(
            "guest 2 never drew guest 1 in cell 1 on {}",
            env.net
        ));
    }
    let t0 = sim.clock.now();
    let (mut flagged, mut back, mut top) = (0u32, 0.0f32, f32::MIN);
    let mut errs = Vec::new();
    for k in 0..400u64 {
        let t = (sim.clock.now() - t0) as f32 / 1000.0;
        let cell = walk_cell((k / STEPS_PER_TICK) as u32);
        sim.guests[0].sync.live.local = moving(cell, speed * t, speed);
        sim.step();
        let now = sim.clock.now();
        if let Some(s) = sim.guests[1].sync.live.sample(walker, now) {
            flagged += s.teleported as u32;
            // the lag settles once the stream has run for 3 s
            if t > 3.0 {
                back = back.max(top - s.pos[0]);
                top = top.max(s.pos[0]);
                errs.push((s.pos[0] - speed * t).abs());
            }
        }
    }
    if errs.is_empty() {
        return Err(format!("guest 2 never drew guest 1 on {}", env.net));
    }
    errs.sort_by(|a, b| a.total_cmp(b));
    let median = errs[errs.len() / 2];
    if flagged != 0 || back > 8.0 || median > 80.0 {
        return Err(format!(
            "the walk over {} was not smooth: {flagged} teleports, {back} units backwards, \
             median lag {median} units",
            env.net
        ));
    }
    sim.guests[0].sync.live.local = at(2, 5000.0);
    let mut snaps = Vec::new();
    for _ in 0..200 {
        sim.step();
        let now = sim.clock.now();
        if let Some(s) = sim.guests[1].sync.live.sample(walker, now) {
            if s.teleported {
                snaps.push((s.cell, s.pos[0]));
            }
        }
    }
    if snaps.len() != 1 || snaps[0].0 != CellId(2) || (snaps[0].1 - 5000.0).abs() > 0.1 {
        return Err(format!(
            "a teleport over {} must snap once to cell 2 at 5000: {snaps:?}",
            env.net
        ));
    }
    Ok(format!(
        "small moves drawn between samples {DELAY_MS} ms behind, a {under} unit jump blends, \
         a {over} unit jump or a cell change snaps once, extrapolation capped at \
         {EXTRAPOLATE_CAP_MS} ms; end to end over {}: smooth walk (median lag {median:.1}), \
         one snap on the teleport",
        env.net
    ))
}

// ---------------------------------------------------------------------------
// world.leveled_once

const LIST: FormId = FormId(0x0001_5A00);
const KEY: u32 = 0x0010_7E11;
const KEY2: u32 = 0x0010_7E12;

/// Each peer's own roll from its own seed, pairwise different.
fn local_rolls(seed: u64, peers: usize, salt: u64) -> Vec<FormId> {
    let entries: Vec<FormId> = (0..32u32).map(|i| FormId(0x000A_0000 + i * 3)).collect();
    let mut out = Vec::new();
    for p in 0..peers {
        let mut rng = SeededRng::new(seed ^ salt ^ (0x5EED_0000 + p as u64 * 0x9E37));
        let mut pick = entries[rng.below(entries.len() as u64) as usize];
        while out.contains(&pick) {
            pick = entries[rng.below(entries.len() as u64) as usize];
        }
        out.push(pick);
    }
    out
}

fn drain_host(sim: &mut Sim, host: &mut ActorClient) {
    for m in sim.host.live.take_local() {
        if let Msg::Actor(a) = m {
            host.on_msg(a);
        }
    }
}

/// Guest `i` proposes its own roll to the host. Under mutation the last guest skips
/// the host and keeps its own roll.
fn guest_propose(sim: &mut Sim, i: usize, key: u32, roll: FormId, mutate: bool) {
    let last = sim.guests.len() - 1;
    let actors = &mut sim.guests[i].sync.live.actors;
    if mutate && i == last {
        actors.leveled.insert((LIST, key), roll);
    } else {
        let m = actors.request_leveled(LIST, key, roll);
        sim.guest_emit(i, vec![Msg::Actor(m)]);
    }
}

fn wait_picks(sim: &mut Sim, key: u32, who: &[usize]) -> bool {
    sim.run_until(60_000, |s| {
        who.iter().all(|i| {
            s.guests[*i]
                .sync
                .live
                .actors
                .leveled
                .contains_key(&(LIST, key))
        })
    })
}

/// Host player and guests each roll a leveled pick with their own seed; the host's
/// `LeveledReq` table keeps the first proposal and every peer ends with it. A repeated
/// request does not re-roll. Mutation: the last guest rolls its own pick and never asks
/// the host.
fn sim_leveled_once(env: &Env, mutate: bool) -> Result<String, String> {
    let mut sim = active_party(env, 2)?;
    let n = sim.guests.len();
    let all: Vec<usize> = (0..n).collect();
    let mut host = ActorClient::new(PlayerId::HOST);
    if (0..n).any(|i| !sim.guests[i].sync.live.actors.leveled.is_empty()) {
        return Err("a guest held a leveled pick before the host answered".into());
    }

    // key 1: the host player rolls first, the guests propose their own rolls after
    let rolls = local_rolls(env.seed, n + 1, 0);
    let m = host.request_leveled(LIST, KEY, rolls[0]);
    sim.host_local(Msg::Actor(m));
    drain_host(&mut sim, &mut host);
    let host_pick = *host
        .leveled
        .get(&(LIST, KEY))
        .ok_or("the host did not resolve its own leveled pick")?;
    for i in 0..n {
        guest_propose(&mut sim, i, KEY, rolls[i + 1], mutate);
    }
    if !wait_picks(&mut sim, KEY, &all) {
        return Err(format!("not every guest received the pick on {}", env.net));
    }
    let picks: Vec<FormId> = (0..n)
        .map(|i| sim.guests[i].sync.live.actors.leveled[&(LIST, KEY)])
        .collect();
    if host_pick != rolls[0] || picks.iter().any(|p| *p != host_pick) {
        return Err(format!(
            "picks differ: host rolled {:?} and holds {host_pick:?}, guests rolled {:?} and \
             hold {picks:?}",
            rolls[0],
            &rolls[1..]
        ));
    }

    // a repeated request with a fresh roll gets the same pick: no re-roll
    let again = local_rolls(env.seed, n + 1, 0xA6A1);
    let fresh = again
        .iter()
        .copied()
        .find(|r| *r != host_pick)
        .ok_or("no fresh roll")?;
    sim.guests[0].sync.live.actors.leveled.remove(&(LIST, KEY));
    guest_propose(&mut sim, 0, KEY, fresh, false);
    if !wait_picks(&mut sim, KEY, &[0]) {
        return Err(format!("no reply to a repeated request on {}", env.net));
    }
    let repeat = sim.guests[0].sync.live.actors.leveled[&(LIST, KEY)];
    if repeat != host_pick {
        return Err(format!(
            "a repeated request re-rolled: {repeat:?} instead of {host_pick:?}"
        ));
    }

    // key 2: guest 1 asks first; the host player and the other guests end with its roll
    let rolls2 = local_rolls(env.seed, n + 1, 0x2222);
    guest_propose(&mut sim, 0, KEY2, rolls2[1], mutate);
    if !wait_picks(&mut sim, KEY2, &[0]) {
        return Err(format!(
            "guest 1 got no pick for the second key on {}",
            env.net
        ));
    }
    let m = host.request_leveled(LIST, KEY2, rolls2[0]);
    sim.host_local(Msg::Actor(m));
    drain_host(&mut sim, &mut host);
    for i in 1..n {
        guest_propose(&mut sim, i, KEY2, rolls2[i + 1], mutate);
    }
    if !wait_picks(&mut sim, KEY2, &all) {
        return Err(format!(
            "not every guest received the second pick on {}",
            env.net
        ));
    }
    let first = sim.guests[0].sync.live.actors.leveled[&(LIST, KEY2)];
    let host2 = host.leveled.get(&(LIST, KEY2)).copied();
    let picks2: Vec<FormId> = (0..n)
        .map(|i| sim.guests[i].sync.live.actors.leveled[&(LIST, KEY2)])
        .collect();
    if first != rolls2[1] || host2 != Some(first) || picks2.iter().any(|p| *p != first) {
        return Err(format!(
            "second key: guest 1 rolled {:?}, host holds {host2:?}, guests hold {picks2:?}",
            rolls2[1]
        ));
    }
    Ok(format!(
        "host and {n} guests rolled different leveled picks with their own seeds; the host \
         resolved each key once (first proposal wins, a repeat does not re-roll) and every \
         peer holds the same pick, over {}",
        env.net
    ))
}

const BOX: FormId = FormId(0x0010_B0C5);

fn loot_roll(seed: u64, peer: u64) -> Vec<(FormId, u32)> {
    let mut rng = SeededRng::new(seed ^ 0x100F ^ (peer << 40));
    (0..3)
        .map(|_| {
            (
                FormId(0x000F_0000 + rng.below(64) as u32),
                1 + rng.below(20) as u32,
            )
        })
        .collect()
}

/// The host registers a container's leveled loot once from its own roll; later rolls
/// from other machines change nothing, and every guest that opens the container sees
/// the host's roll. Mutation: the host loses its once table before the second guest's
/// roll arrives, so that roll is registered again.
fn sim_leveled_loot_once(env: &Env, mutate: bool) -> Result<String, String> {
    let mut sim = active_party(env, 2)?;
    let n = sim.guests.len();
    let host_roll = loot_roll(env.seed, 0);
    if !sim
        .host
        .live
        .loot
        .register(BOX, CellId(1), || host_roll.clone())
    {
        return Err("the host could not register the container".into());
    }
    let host_items: BTreeMap<FormId, u32> = sim
        .host
        .live
        .loot
        .items(BOX)
        .ok_or("no host contents")?
        .clone();
    let mut views = Vec::new();
    let mut rerolled = Vec::new();
    for i in 0..n {
        let mut roll = loot_roll(env.seed, i as u64 + 1);
        if roll == host_roll {
            roll = loot_roll(env.seed ^ 0xFFFF, i as u64 + 1);
        }
        if mutate && i == 1 {
            sim.host.live.loot = LootHost::new();
        }
        if sim.host.live.loot.register(BOX, CellId(1), || roll.clone()) {
            rerolled.push(i + 1);
        }
        let m = sim.guests[i].sync.live.loot.open(BOX);
        sim.guest_emit(i, vec![Msg::Loot(m)]);
        let ok = sim.run_until(60_000, |s| {
            s.guests[i]
                .sync
                .live
                .loot
                .view(BOX)
                .is_some_and(|v| v.leased)
        });
        if !ok {
            return Err(format!("guest {} could not open the container", i + 1));
        }
        let seen = sim.guests[i]
            .sync
            .live
            .loot
            .view(BOX)
            .map(|v| v.visible())
            .unwrap_or_default();
        views.push(seen);
        if let Some(m) = sim.guests[i].sync.live.loot.close(BOX) {
            sim.guest_emit(i, vec![Msg::Loot(m)]);
        }
        if !sim.run_until(60_000, |s| s.host.live.loot.holder(BOX).is_none()) {
            return Err(format!("guest {} kept the lease", i + 1));
        }
    }
    let final_items = sim.host.live.loot.items(BOX).cloned().unwrap_or_default();
    if views.iter().any(|v| *v != host_items) || final_items != host_items {
        return Err(format!(
            "contents differ: host rolled {host_items:?}, guests saw {views:?}, host now holds \
             {final_items:?}"
        ));
    }
    if !rerolled.is_empty() {
        return Err(format!(
            "the rolls of guests {rerolled:?} replaced the first roll"
        ));
    }
    Ok(format!(
        "the host rolled the container once; {n} later rolls from other machines changed \
         nothing; {n} guests opened it and saw the host's {} item kinds, over {}",
        host_items.len(),
        env.net
    ))
}

// ---------------------------------------------------------------------------
// social.voice

fn codec_name(c: &dyn Codec) -> &'static str {
    match c.id() {
        CODEC_OPUS => "Opus",
        CODEC_ADPCM => "ADPCM (the Opus feature is off)",
        _ => "an unknown codec",
    }
}

fn tone(n: usize, rate: u32, amp: f32, from: usize) -> Vec<f32> {
    (from..from + n)
        .map(|i| (i as f32 / rate as f32 * 440.0 * std::f32::consts::TAU).sin() * amp)
        .collect()
}

/// Feeds `ms` of microphone signal in 10 ms chunks; returns the wire packets.
fn feed(tx: &mut VoiceTx, ms: u32, rate: u32, amp: f32) -> Vec<Vec<u8>> {
    let chunk = (rate / 100) as usize;
    let mut out = Vec::new();
    for k in 0..(ms / 10) as usize {
        out.extend(tx.push_mic(&tone(chunk, rate, amp, k * chunk)));
    }
    out
}

fn decode_all(pk: &[Vec<u8>]) -> Result<Vec<VoicePacket>, String> {
    pk.iter()
        .map(|b| VoicePacket::decode(b).map_err(|e| format!("a sent packet did not decode: {e:?}")))
        .collect()
}

/// The voice envelope: real packets and edge packets survive encode, the wire and
/// decode unchanged, the header layout is the documented one, malformed datagrams are
/// refused. Mutation: the wire swaps the two sequence bytes.
fn voice_envelope(_env: &Env, mutate: bool) -> Result<String, String> {
    let wire = |mut b: Vec<u8>| {
        if mutate && b.len() > 5 {
            b.swap(4, 5);
        }
        b
    };
    let codec = default_codec();
    let (rate, frame) = (codec.sample_rate(), codec.frame_samples() as u32);
    let name = codec_name(codec.as_ref());
    let mut tx = VoiceTx::new(3, rate, Activation::PushToTalk, codec);
    tx.set_ptt(true);
    let mut pk = feed(&mut tx, 200, rate, 0.4);
    tx.set_ptt(false);
    pk.extend(feed(&mut tx, 40, rate, 0.4));
    if pk.len() < 10 {
        return Err(format!("only {} packets were built", pk.len()));
    }
    for (i, b) in pk.iter().enumerate() {
        let p = VoicePacket::decode(&wire(b.clone()))
            .map_err(|e| format!("packet {i} did not decode: {e:?}"))?;
        if p.encode() != *b {
            return Err(format!("packet {i} changed on the way: {p:?}"));
        }
        let end = i == pk.len() - 1;
        let layout = b[0] == VERSION
            && b.len() == HEADER_LEN + p.payload.len()
            && u16::from_le_bytes([b[4], b[5]]) == p.seq
            && u32::from_le_bytes([b[6], b[7], b[8], b[9]]) == p.timestamp
            && u16::from_le_bytes([b[10], b[11]]) as usize == p.payload.len();
        let fields = p.speaker == 3
            && p.seq == i as u16
            && p.timestamp == i as u32 * frame
            && p.is_end_of_talk() == end
            && (end || (p.flags & FLAG_PTT_ACTIVE != 0 && !p.payload.is_empty()));
        if !layout || !fields {
            return Err(format!("packet {i} has a wrong header: {p:?}"));
        }
    }
    let edges = [
        VoicePacket {
            flags: FLAG_END_OF_TALK | FLAG_PTT_ACTIVE,
            codec: CODEC_OPUS,
            speaker: 255,
            seq: 0x1234,
            timestamp: u32::MAX,
            payload: vec![0xAB; MAX_PAYLOAD],
        },
        VoicePacket {
            flags: 0,
            codec: 0,
            speaker: 0,
            seq: u16::MAX,
            timestamp: 0,
            payload: Vec::new(),
        },
    ];
    for p in &edges {
        let got = VoicePacket::decode(&wire(p.encode()));
        if got.as_ref() != Ok(p) {
            return Err(format!("an edge packet did not survive: {got:?}"));
        }
    }
    let b = edges[1].encode();
    let mut bad_version = b.clone();
    bad_version[0] = VERSION + 1;
    let mut long = edges[0].encode();
    long.push(0);
    long[10..12].copy_from_slice(&((MAX_PAYLOAD + 1) as u16).to_le_bytes());
    let refusals = [
        (
            VoicePacket::decode(&b[..HEADER_LEN - 1]),
            DecodeError::TooShort,
        ),
        (
            VoicePacket::decode(&bad_version),
            DecodeError::BadVersion(VERSION + 1),
        ),
        (
            VoicePacket::decode(&edges[0].encode()[..HEADER_LEN + 1]),
            DecodeError::BadLength,
        ),
        (VoicePacket::decode(&long), DecodeError::BadLength),
    ];
    for (got, want) in refusals {
        if got != Err(want.clone()) {
            return Err(format!("expected {want:?}, got {got:?}"));
        }
    }
    Ok(format!(
        "{} real packets ({name}) and 2 edge packets survive encode and decode byte for byte; \
         12 byte header in the documented order; short, wrong version and wrong length \
         datagrams are refused",
        pk.len()
    ))
}

/// Voice activity detection on the real send path: silence and low noise send nothing,
/// speech sends, the end of speech sends a short hangover and one end marker, then
/// silence again sends nothing. Mutation: the gate is held open (the VAD is bypassed).
fn voice_vad(env: &Env, mutate: bool) -> Result<String, String> {
    let codec = default_codec();
    let rate = codec.sample_rate();
    let mut tx = VoiceTx::new(1, rate, Activation::VoiceActivity, codec);
    if mutate {
        tx.set_activation(Activation::PushToTalk);
        tx.set_ptt(true);
    }
    let silent = feed(&mut tx, 400, rate, 0.0);
    if !silent.is_empty() {
        return Err(format!("silence sent {} packets", silent.len()));
    }
    let chunk = (rate / 100) as usize;
    let mut rng = SeededRng::new(env.seed ^ 0x0A1D);
    let mut noisy = Vec::new();
    for _ in 0..40 {
        let hiss: Vec<f32> = (0..chunk)
            .map(|_| (rng.unit() as f32 * 2.0 - 1.0) * 0.005)
            .collect();
        noisy.extend(tx.push_mic(&hiss));
    }
    if !noisy.is_empty() {
        return Err(format!("low noise sent {} packets", noisy.len()));
    }
    let speech = decode_all(&feed(&mut tx, 200, rate, 0.4))?;
    if speech.len() < 9
        || speech
            .iter()
            .any(|p| p.is_end_of_talk() || p.payload.is_empty())
    {
        return Err(format!("speech sent {} voice packets", speech.len()));
    }
    let tail = decode_all(&feed(&mut tx, 400, rate, 0.0))?;
    let ends = tail.iter().filter(|p| p.is_end_of_talk()).count();
    if ends != 1 || !tail.last().is_some_and(|p| p.is_end_of_talk()) || tail.len() > 11 {
        return Err(format!(
            "the end of speech must send a short hangover and one end marker: {} packets, \
             {ends} end markers",
            tail.len()
        ));
    }
    let after = feed(&mut tx, 400, rate, 0.0);
    if !after.is_empty() {
        return Err(format!(
            "silence after the utterance sent {} packets",
            after.len()
        ));
    }
    let again = feed(&mut tx, 100, rate, 0.4);
    if again.is_empty() {
        return Err("speech did not open the gate again".into());
    }
    Ok(format!(
        "VAD on VoiceTx: 400 ms silence and 400 ms noise sent nothing, 200 ms speech sent {} \
         packets, the end sent {} hangover packets and one end marker, then silence sent \
         nothing",
        speech.len(),
        tail.len() - 1
    ))
}

/// Push to talk on the real send path: the key decides, not the voice. Mutation: the
/// push-to-talk gate is replaced by voice activation.
fn voice_ptt(_env: &Env, mutate: bool) -> Result<String, String> {
    let codec = default_codec();
    let rate = codec.sample_rate();
    // the game feeds the microphone at 48 kHz and resamples
    let mut tx = VoiceTx::new(1, 48_000, Activation::PushToTalk, codec);
    if mutate {
        tx.set_activation(Activation::VoiceActivity);
    }
    let up = feed(&mut tx, 300, 48_000, 0.4);
    if !up.is_empty() {
        return Err(format!("speech with the key up sent {} packets", up.len()));
    }
    tx.set_ptt(true);
    let down = decode_all(&feed(&mut tx, 200, 48_000, 0.4))?;
    if down.len() < 9
        || down
            .iter()
            .any(|p| p.is_end_of_talk() || p.flags & FLAG_PTT_ACTIVE == 0)
    {
        return Err(format!(
            "speech with the key down sent {} push-to-talk packets",
            down.len()
        ));
    }
    let held = decode_all(&feed(&mut tx, 100, 48_000, 0.0))?;
    if held.len() < 4 || held.iter().any(|p| p.is_end_of_talk()) {
        return Err(format!(
            "the held key sent {} packets of silence",
            held.len()
        ));
    }
    tx.set_ptt(false);
    let release = decode_all(&feed(&mut tx, 200, 48_000, 0.4))?;
    if release.len() != 1 || !release[0].is_end_of_talk() {
        return Err(format!(
            "releasing the key must send one end marker, sent {} packets",
            release.len()
        ));
    }
    let after = feed(&mut tx, 200, 48_000, 0.4);
    if !after.is_empty() {
        return Err(format!(
            "speech after the release sent {} packets",
            after.len()
        ));
    }
    tx.set_ptt(true);
    if feed(&mut tx, 100, 48_000, 0.4).is_empty() {
        return Err("the key did not open the microphone again".into());
    }
    tx.set_muted(true);
    let muted = decode_all(&feed(&mut tx, 200, 48_000, 0.4))?;
    if muted.len() != 1 || !muted[0].is_end_of_talk() || !feed(&mut tx, 200, 48_000, 0.4).is_empty()
    {
        return Err(format!(
            "self-mute must end the utterance once, sent {} packets",
            muted.len()
        ));
    }
    Ok(format!(
        "push to talk on VoiceTx (48 kHz microphone, {rate} Hz codec): key up sent nothing, \
         key down sent {} flagged packets, release and self-mute each sent one end marker",
        down.len()
    ))
}

/// The jitter buffer: 100 real packets arrive shuffled (up to 40 ms jitter), with loss
/// and duplicates; the buffer at the depth `VoiceRx` uses (3 frames, at most 12) plays
/// them in order, conceals each lost frame in its slot and drops the duplicates; the
/// `VoiceRx` mix stays voiced in every slot. Mutation: the buffer depth is cut to one
/// frame, so frames arrive after their slot.
fn voice_jitter(env: &Env, mutate: bool) -> Result<String, String> {
    const N: usize = 100;
    let codec = default_codec();
    let (rate, frame) = (codec.sample_rate(), codec.frame_samples());
    let mut tx = VoiceTx::new(1, rate, Activation::PushToTalk, codec);
    tx.set_ptt(true);
    let mut pk = Vec::new();
    let chunk = (rate / 100) as usize;
    let mut k = 0;
    while pk.len() < N {
        pk.extend(tx.push_mic(&tone(chunk, rate, 0.4, k * chunk)));
        k += 1;
    }
    tx.set_ptt(false);
    let mut end = Vec::new();
    while end.is_empty() {
        end = tx.push_mic(&tone(chunk, rate, 0.4, k * chunk));
        k += 1;
    }
    if pk.len() != N || !VoicePacket::decode(&end[0]).is_ok_and(|p| p.is_end_of_talk()) {
        return Err("the sender did not build 100 frames and an end marker".into());
    }

    // arrival schedule: 20 ms frames, 40 ms latency, 0..=40 ms jitter, loss, duplicates
    let mut rng = SeededRng::new(env.seed ^ 0x0717_7E12);
    let (mut arrivals, mut lost, mut dups) = (Vec::new(), BTreeSet::new(), 0u64);
    for i in 0..N {
        let keep = matches!(i, 0 | 20 | 21 | 50) || i == N - 1;
        if matches!(i, 10 | 37 | 38) || (!keep && rng.chance(0.08)) {
            lost.insert(i);
            continue;
        }
        let jitter = match i {
            0 | 21 => 0,
            20 => 40,
            _ => rng.below(41),
        };
        let t = i as u64 * 20 + 40 + jitter;
        arrivals.push((t, arrivals.len(), i));
        if i == 50 || rng.chance(0.05) {
            arrivals.push((t, arrivals.len(), i));
            dups += 1;
        }
    }
    let end_at = (N as u64 - 1) * 20 + 40 + 41;
    arrivals.sort();
    let inversions = arrivals.windows(2).filter(|w| w[1].2 < w[0].2).count();
    if inversions == 0 || lost.is_empty() || dups == 0 {
        return Err("the arrival schedule is not shuffled".into());
    }

    // 1. the jitter buffer on its own: payload is the whole datagram, to read the order
    let (target, max) = if mutate { (1, 2) } else { (3, 12) };
    let mut jb = JitterBuffer::new(target, max);
    let mut out = Vec::new();
    let mut idx = 0;
    let mut ended = false;
    for tick in 0..(N as u64 + 40) {
        let now = tick * 20;
        while idx < arrivals.len() && arrivals[idx].0 <= now {
            let i = arrivals[idx].2;
            let p = VoicePacket::decode(&pk[i]).map_err(|e| format!("{e:?}"))?;
            jb.push(p.seq, pk[i].clone());
            idx += 1;
        }
        if !ended && now >= end_at {
            jb.finish();
            ended = true;
        }
        match jb.pop() {
            Pop::Idle => out.push(None),
            Pop::Conceal => out.push(Some(None)),
            Pop::Frame(b) => {
                let p = VoicePacket::decode(&b).map_err(|e| format!("{e:?}"))?;
                out.push(Some(Some(p.seq as usize)));
            }
        }
    }
    let first = out
        .iter()
        .position(|o| o.is_some())
        .ok_or("nothing played")?;
    let played: Vec<Option<usize>> = out[first..].iter().map_while(|o| *o).collect();
    let want: Vec<Option<usize>> = (0..N).map(|i| (!lost.contains(&i)).then_some(i)).collect();
    let trailing = out[first + played.len()..].iter().all(|o| o.is_none());
    if played != want || !trailing || jb.stats.late != 0 || jb.stats.duplicates != dups {
        let at = played.iter().zip(&want).position(|(a, b)| a != b);
        return Err(format!(
            "the buffer played {} slots (first difference at {at:?}), late {}, duplicates {} \
             of {dups}, then {}",
            played.len(),
            jb.stats.late,
            jb.stats.duplicates,
            if trailing { "idle" } else { "more" }
        ));
    }

    // 2. the real receiver: audio in every slot, lost ones concealed
    let mut rx = VoiceRx::new();
    let mut mix = Vec::new();
    let (mut voiced, mut idx) = (0usize, 0usize);
    let mut ended = false;
    for tick in 0..(N as u64 + 40) {
        let now = tick * 20;
        while idx < arrivals.len() && arrivals[idx].0 <= now {
            rx.on_bytes(1, &pk[arrivals[idx].2], now);
            idx += 1;
        }
        if !ended && now >= end_at {
            rx.on_bytes(1, &end[0], now);
            ended = true;
        }
        if rx.mix(frame, &mut mix) > 0 {
            voiced += 1;
        }
    }
    if voiced != N {
        return Err(format!("VoiceRx mixed {voiced} voiced frames, want {N}"));
    }
    Ok(format!(
        "{N} packets in shuffled order ({inversions} inversions) with {} lost and {dups} \
         duplicated came out in order, each loss concealed in its slot, no late frame; \
         VoiceRx stayed voiced for all {N} slots",
        lost.len()
    ))
}

/// The mixer: one speaker's packets heard by listeners at several distances, in another
/// cell, and to the side. The gain is full inside `NEAR`, falls with distance to `FLOOR`
/// at `FAR`, is `OTHER_CELL_GAIN` in another cell, and the pan follows the side.
/// Mutation: the receivers get no speaker positions (positional audio off).
fn voice_mixer(_env: &Env, mutate: bool) -> Result<String, String> {
    if NEAR != 1_200.0 || FAR != 7_000.0 || FLOOR != 0.15 || OTHER_CELL_GAIN != 0.5 {
        return Err(format!(
            "documented gains changed: near {NEAR}, far {FAR}, floor {FLOOR}, other cell \
             {OTHER_CELL_GAIN}"
        ));
    }
    let codec = default_codec();
    let (rate, frame) = (codec.sample_rate(), codec.frame_samples());
    let mut tx = VoiceTx::new(2, rate, Activation::PushToTalk, codec);
    tx.set_ptt(true);
    let pk = feed(&mut tx, 200, rate, 0.4);
    let heard = |place: ([f32; 3], u32)| -> Result<(f32, f32), String> {
        let mut rx = VoiceRx::new();
        rx.positions.listener = ([0.0; 3], 0.0, 1);
        if !mutate {
            rx.positions.speakers.insert(2, place);
        }
        for b in &pk {
            if !rx.on_bytes(2, b, 0) {
                return Err("the receiver refused a packet".into());
            }
        }
        let (mut l, mut r) = (0.0f32, 0.0f32);
        let mut out = Vec::new();
        for _ in 0..8 {
            rx.mix(frame, &mut out);
            l += out.iter().step_by(2).map(|x| x.abs()).sum::<f32>();
            r += out.iter().skip(1).step_by(2).map(|x| x.abs()).sum::<f32>();
        }
        Ok((l, r))
    };
    let ahead = |d: f32| heard(([0.0, d, 0.0], 1)).map(|(l, r)| l + r);
    let full = ahead(0.0)?;
    if full <= 0.0 {
        return Err("nothing was heard".into());
    }
    let dists = [1_000.0f32, 3_000.0, 5_000.0, 6_500.0, 9_000.0];
    let mut ratios = Vec::new();
    for d in dists {
        ratios.push(ahead(d)? / full);
    }
    let falls = ratios.windows(2).all(|w| w[1] < w[0] - 0.01);
    let matches = dists
        .iter()
        .zip(&ratios)
        .all(|(d, r)| (r - Positions::distance_gain(*d)).abs() < 0.02);
    if (ratios[0] - 1.0).abs() > 0.01 || !falls || !matches || (ratios[4] - FLOOR).abs() > 0.01 {
        return Err(format!(
            "gain over distance {dists:?} was {ratios:?} of the near gain, want full inside \
             {NEAR} falling to {FLOOR} at {FAR}"
        ));
    }
    let other = heard(([0.0, 500.0, 0.0], 2)).map(|(l, r)| (l + r) / full)?;
    if (other - OTHER_CELL_GAIN).abs() > 0.01 {
        return Err(format!(
            "another cell was heard at {other}, want {OTHER_CELL_GAIN}"
        ));
    }
    let (l, r) = heard(([2_000.0, 0.0, 0.0], 1))?;
    let (cl, cr) = heard(([0.0, 3_000.0, 0.0], 1))?;
    if r < l * 3.0 || (cl - cr).abs() / (cl + cr) > 0.05 {
        return Err(format!(
            "pan: a speaker to the right gave {l}/{r}, one ahead gave {cl}/{cr}"
        ));
    }
    let shown: Vec<String> = ratios.iter().map(|r| format!("{r:.2}")).collect();
    Ok(format!(
        "VoiceRx gain for a speaker at 0, 1000, 3000, 5000, 6500, 9000 units: 1.00, {} (full \
         inside {NEAR}, floor {FLOOR} at {FAR}); another cell {other:.2}; a speaker to the \
         right is in the right ear",
        shown.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> Env<'static> {
        Env {
            data: None,
            peers: 2,
            net: "perfect",
            seed: 1,
        }
    }

    fn twin(name: &str, check: Check) {
        let e = env();
        let ok = check(&e, false);
        assert!(ok.is_ok(), "{name} normal run failed: {ok:?}");
        let bad = check(&e, true);
        assert!(bad.is_err(), "{name} mutated run passed: {bad:?}");
    }

    #[test]
    fn sim_avatar_interp_twin() {
        twin("sim_avatar_interp", sim_avatar_interp);
    }

    #[test]
    fn sim_leveled_once_twin() {
        twin("sim_leveled_once", sim_leveled_once);
    }

    #[test]
    fn sim_leveled_loot_once_twin() {
        twin("sim_leveled_loot_once", sim_leveled_loot_once);
    }

    #[test]
    fn voice_envelope_twin() {
        twin("voice_envelope", voice_envelope);
    }

    #[test]
    fn voice_vad_twin() {
        twin("voice_vad", voice_vad);
    }

    #[test]
    fn voice_ptt_twin() {
        twin("voice_ptt", voice_ptt);
    }

    #[test]
    fn voice_jitter_twin() {
        twin("voice_jitter", voice_jitter);
    }

    #[test]
    fn voice_mixer_twin() {
        twin("voice_mixer", voice_mixer);
    }

    /// The runner uses other profiles, seeds and party sizes than the unit env: every
    /// check must hold, and every mutant must fail, on each of them.
    #[test]
    fn every_check_holds_on_every_profile() {
        let nets = [
            "perfect",
            "good_wifi",
            "bad_wifi",
            "mobile",
            "congested",
            "flaky",
            "partition",
        ];
        for (name, check) in registry() {
            for net in nets {
                for seed in [1u64, 7, 23] {
                    for peers in [2u8, 4] {
                        let e = Env {
                            data: None,
                            peers,
                            net,
                            seed,
                        };
                        let ok = check(&e, false);
                        assert!(ok.is_ok(), "{name} on {net}/{seed}/{peers}: {ok:?}");
                        assert!(
                            check(&e, true).is_err(),
                            "{name} mutant passed on {net}/{seed}/{peers}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_check_is_registered() {
        let names: Vec<&str> = registry().into_iter().map(|(n, _)| n).collect();
        assert_eq!(names.len(), 8);
        assert!(names.contains(&"sim_avatar_interp") && names.contains(&"voice_jitter"));
    }
}
