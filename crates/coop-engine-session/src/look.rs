//! The other players' avatars as those players (docs/coop/SPECS.md S35): they walk and run as
//! their players do, hold, draw and fire their weapons, wear what they wear, and have their
//! faces, bodies and names.
//!
//! Everything travels in coop-core's avatar stream, unchanged:
//!
//! - each state (20 a second) carries the player's movement flags (`AvatarState::anim`), its
//!   weapon (`weapon`) and whether it sneaks and has its weapon drawn (`FLAG_SNEAK`,
//!   `FLAG_DRAWN`);
//! - each attack the player starts and each reload is an action event (`ActionKind::Shoot` with
//!   the animation group, `ActionKind::Reload`), in order, once each;
//! - what the player wears is the parts' equipment, and its face, body and name the parts'
//!   appearance ([`encode`]), sent again only when they change, and to every player who joins
//!   later by the host's hub.
//!
//! In a game showing another player, its avatar ([`Look::frame`], called by the ladder for each
//! avatar each frame) is led with its AI on and the player's gait forced on it, so the game
//! plays its walk, run and sneak animations, and the idle when the flags are 0; its weapon is
//! drawn or put away as the player's is, the attacks and reloads are played on it, it wears the
//! player's items; and it is placed from a base form of its own carrying the player's face and
//! name ([`Look::base_for`]), placed anew when that look changes.
//!
//! [`Cap`] says how far it goes (`NV_COOP_LOOK`); with no [`Look`] the ladder is as before (the
//! avatar's AI off, a sliding figure of this game's own player).

use std::collections::{BTreeMap, BTreeSet};

use coop_core::avatar::{
    ActionKind, AvatarState, Sample, FLAG_DRAWN, FLAG_SNEAK, MAX_APPEARANCE, MAX_EQUIP,
};
use coop_core::ids::{FormId, PlayerId};
use coop_core::sync::GuestLive;
use coop_engine::look::{self, Face};
use coop_engine::{actors, live};
use engine_core::Engine;

use crate::track::MOVE_SNEAKING;

/// How much of the other players' look is shown (`NV_COOP_LOOK`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cap {
    /// The avatar walks, runs and sneaks with the game's animations.
    Walk,
    /// Also: its weapon in hand, drawn as the player's, its attacks and reloads.
    Pose,
    /// Also: it wears what the player wears.
    Gear,
    /// Also: the player's face, body and name.
    Face,
}

impl Cap {
    /// `walk`, `pose`, `gear`, or `face` (also `name`, `on`, `all`); `off` (also `0`, `no`,
    /// `none`) is `None`. Anything else, or nothing, is [`Cap::Face`]: every level is on unless
    /// asked otherwise (anim, docs/coop/SPECS.md S39).
    pub fn parse(v: &str) -> Option<Cap> {
        match v.trim() {
            "walk" => Some(Cap::Walk),
            "pose" => Some(Cap::Pose),
            "gear" => Some(Cap::Gear),
            "off" | "0" | "no" | "none" => None, // anim
            _ => Some(Cap::Face),
        }
    }
}

/// What this player wears and looks like is read again this often (ms), and sent when it
/// changed.
pub const PARTS_EVERY_MS: u64 = 1000;
/// An avatar's items are put right at most this often (ms), and at once when they change.
pub const WEAR_EVERY_MS: u64 = 1000;
/// An avatar whose weapon is not drawn or put away as asked is asked again this often (ms).
pub const DRAW_EVERY_MS: u64 = 1000;
/// The appearance's first byte and its version.
pub const APPEARANCE_MAGIC: u8 = b'L';
/// See [`APPEARANCE_MAGIC`].
pub const APPEARANCE_VERSION: u8 = 1;
/// A face-gen value travels as a whole number of thousandths (co-op's own format), within the
/// range of an `i16`.
pub const FACE_SCALE: f32 = 1000.0;

/// `face` as the parts' appearance: `L`, version, flags (bit 0 female), race, hair, eyes, hair
/// colour (u32 each), hair length, height, weight (f32 each), the head parts (a count and u32
/// each), the four face-gen matrices (rows and columns as a byte each, then each value as an
/// `i16` of [`FACE_SCALE`]), the name (a length byte, then its bytes). `None` when it does not
/// fit [`MAX_APPEARANCE`].
pub fn encode(face: &Face) -> Option<Vec<u8>> {
    let mut b = vec![APPEARANCE_MAGIC, APPEARANCE_VERSION, u8::from(face.female)];
    for v in [face.race, face.hair, face.eyes, face.hair_color] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    for v in [face.hair_length, face.height, face.weight] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    let parts = face.head_parts.len().min(look::HEAD_PARTS_MAX);
    b.push(parts as u8);
    for p in &face.head_parts[..parts] {
        b.extend_from_slice(&p.to_le_bytes());
    }
    for (rows, cols, values) in &face.coord {
        let (r, c) = (u8::try_from(*rows).ok()?, u8::try_from(*cols).ok()?);
        if values.len() != usize::from(r) * usize::from(c) {
            return None;
        }
        b.push(r);
        b.push(c);
        for v in values {
            let q = (v * FACE_SCALE)
                .round()
                .clamp(f32::from(i16::MIN), f32::from(i16::MAX)) as i16;
            b.extend_from_slice(&q.to_le_bytes());
        }
    }
    let name: Vec<u8> = face.name.bytes().take(look::NAME_MAX).collect();
    b.push(name.len() as u8);
    b.extend_from_slice(&name);
    (b.len() <= MAX_APPEARANCE).then_some(b)
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let s = self.b.get(self.at..self.at.checked_add(n)?)?;
        self.at += n;
        Some(s)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn f32(&mut self) -> Option<f32> {
        Some(f32::from_bits(self.u32()?))
    }
    fn i16(&mut self) -> Option<i16> {
        Some(i16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
}

/// The [`Face`] an appearance [`encode`] made says; `None` for anything else (empty, another
/// version, cut short, bytes left over).
pub fn decode(b: &[u8]) -> Option<Face> {
    let mut r = Reader { b, at: 0 };
    if r.u8()? != APPEARANCE_MAGIC || r.u8()? != APPEARANCE_VERSION {
        return None;
    }
    let mut f = Face {
        female: r.u8()? & 1 != 0,
        race: r.u32()?,
        hair: r.u32()?,
        eyes: r.u32()?,
        hair_color: r.u32()?,
        hair_length: r.f32()?,
        height: r.f32()?,
        weight: r.f32()?,
        ..Face::default()
    };
    let parts = r.u8()?;
    if usize::from(parts) > look::HEAD_PARTS_MAX {
        return None;
    }
    for _ in 0..parts {
        f.head_parts.push(r.u32()?);
    }
    for m in f.coord.iter_mut() {
        let (rows, cols) = (r.u8()?, r.u8()?);
        let n = usize::from(rows) * usize::from(cols);
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(f32::from(r.i16()?) / FACE_SCALE);
        }
        *m = (u32::from(rows), u32::from(cols), values);
    }
    let len = usize::from(r.u8()?);
    f.name = String::from_utf8_lossy(r.take(len)?).into_owned();
    (r.at == b.len()).then_some(f)
}

/// A player's parts: what it wears (slot, item) and its appearance ([`encode`]).
pub type Parts = (Vec<(u8, FormId)>, Vec<u8>);

/// What [`Look::own`] has this player send.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Own {
    /// New parts: the equipment and the appearance, to publish under the next revision.
    pub parts: Option<Parts>,
    /// Action events to send, in order.
    pub actions: Vec<(ActionKind, u32)>,
}

#[derive(Debug, Default)]
struct Peer {
    /// The parts' revision taken, and what they said.
    rev: Option<u32>,
    equipment: Option<Vec<u32>>,
    face: Option<Face>,
    /// The base made for the face, and the face it was made for.
    base: Option<(Face, u32)>,
    /// A face no base could be made for (not tried again).
    failed: Option<Face>,
    /// Actions taken and not yet shown.
    actions: Vec<(ActionKind, u32)>,
}

#[derive(Debug, Default)]
struct Shown {
    forced: Option<u16>,
    drawn_asked: Option<(bool, u64)>,
    worn: Option<(Vec<u32>, u64)>,
    // anim: its head built from its base form's face.
    head: bool,
}

/// Counters, for the status line and the tests.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Gaits forced on avatars (each change).
    pub gaits: u64,
    /// Fights an avatar's own AI started, ended.
    pub calmed: u64,
    /// Draws and put-aways asked.
    pub draws: u64,
    /// Attacks and reloads played on avatars, and attacks the game did not take.
    pub attacks: u64,
    /// See [`Counts::attacks`].
    pub attacks_refused: u64,
    /// See [`Counts::attacks`].
    pub reloads: u64,
    /// Items put on and taken off avatars.
    pub put_on: u64,
    /// See [`Counts::put_on`].
    pub taken_off: u64,
    /// Base forms made with another player's look.
    pub bases: u64,
    /// This player's parts sent, and its actions.
    pub parts_sent: u64,
    /// See [`Counts::parts_sent`].
    pub actions_sent: u64,
    // anim
    /// Avatars' heads built from their base form's face.
    pub heads: u64,
}

/// The look of this player for the others, and of the others' avatars here.
#[derive(Debug)]
pub struct Look {
    cap: Cap,
    /// The parts last sent, and when they were last read.
    sent: Option<Parts>,
    parts_at: Option<u64>,
    /// This player's last attack count seen (`None` before the first frame; `Some(None)`
    /// before its first attack), and whether it was reloading.
    attack_seen: Option<Option<u8>>,
    reloading: bool,
    peers: BTreeMap<PlayerId, Peer>,
    shown: BTreeMap<u32, Shown>,
    seen: BTreeSet<u32>,
    /// Lines for the log (taken by the caller).
    pub notes: Vec<String>,
    /// See [`Counts`].
    pub counts: Counts,
    // anim
    /// Every other animation of this player and of the others' avatars (docs/coop/SPECS.md S39).
    pub anim: crate::anim::Anim,
}

impl Look {
    /// A look up to `cap`.
    pub fn new(cap: Cap) -> Look {
        Look {
            cap,
            sent: None,
            parts_at: None,
            attack_seen: None,
            reloading: false,
            peers: BTreeMap::new(),
            shown: BTreeMap::new(),
            seen: BTreeSet::new(),
            notes: Vec::new(),
            counts: Counts::default(),
            anim: crate::anim::Anim::default(), // anim
        }
    }

    /// How far it goes.
    pub fn cap(&self) -> Cap {
        self.cap
    }

    /// Puts this player's movement flags, and from [`Cap::Pose`] its weapon, into its state.
    pub fn stamp(&self, e: &mut Engine, state: &mut AvatarState) {
        let Some(p) = look::pose(e) else {
            return;
        };
        state.anim = p.moves;
        if p.moves & MOVE_SNEAKING != 0 {
            state.flags |= FLAG_SNEAK;
        }
        self.anim.stamp(e, state); // anim
        if self.cap >= Cap::Pose {
            state.weapon = p.weapon;
            if p.drawn {
                state.flags |= FLAG_DRAWN;
            }
        }
    }

    /// What this player has to send now: its attacks and reloads since the last frame (from
    /// [`Cap::Pose`]), and its parts when they changed (from [`Cap::Gear`], read every
    /// [`PARTS_EVERY_MS`]).
    pub fn own(&mut self, e: &mut Engine, now_ms: u64) -> Own {
        let mut out = Own::default();
        if self.cap >= Cap::Pose {
            let attack = look::player_attack(e);
            let seq = attack.map(|a| a.seq);
            match self.attack_seen {
                // The first frame only takes the count (an attack before the session is over).
                None => {}
                Some(before) if before != seq => {
                    if let Some(a) = attack {
                        out.actions.push((ActionKind::Shoot, u32::from(a.group)));
                    }
                }
                Some(_) => {}
            }
            self.attack_seen = Some(seq);
            let reloading = look::pose(e).is_some_and(|p| p.reloading);
            if reloading && !self.reloading {
                out.actions.push((ActionKind::Reload, 0));
            }
            self.reloading = reloading;
        }
        if self.cap >= Cap::Gear
            && self
                .parts_at
                .map_or(true, |t| now_ms.saturating_sub(t) >= PARTS_EVERY_MS)
        {
            self.parts_at = Some(now_ms);
            if let Some(parts) = self.read_parts(e) {
                if self.sent.as_ref() != Some(&parts) {
                    self.sent = Some(parts.clone());
                    self.counts.parts_sent += 1;
                    out.parts = Some(parts);
                }
            }
        }
        // anim: every level sends the rest of what the body does.
        let body = self.anim.own(e);
        out.actions.extend(body);
        self.counts.actions_sent += out.actions.len() as u64;
        out
    }

    fn read_parts(&self, e: &mut Engine) -> Option<Parts> {
        live::player(e)?;
        let equipment = look::worn(e, actors::PLAYER_REF_ID)
            .into_iter()
            .filter(|id| actors::shareable(*id))
            .take(MAX_EQUIP)
            .enumerate()
            .map(|(i, id)| (i as u8, FormId(id)))
            .collect();
        let appearance = if self.cap >= Cap::Face {
            look::player_face(e)
                .and_then(|f| encode(&f))
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        Some((equipment, appearance))
    }

    /// Takes the other players' parts and action events from `live` (the guest's own, or the
    /// host's view of the others). Actions not shown by the next frame are dropped (they are
    /// cues of that moment).
    pub fn take(&mut self, live: &mut GuestLive) {
        for peer in self.peers.values_mut() {
            peer.actions.clear();
        }
        self.anim.clear_pending(); // anim
        for (p, r) in &live.remote {
            let Some(parts) = &r.parts else {
                continue;
            };
            let peer = self.peers.entry(*p).or_default();
            if peer.rev.is_some_and(|rev| parts.rev <= rev) {
                continue;
            }
            peer.rev = Some(parts.rev);
            peer.equipment = Some(parts.equipment.iter().map(|(_, f)| f.0).collect());
            peer.face = decode(&parts.appearance);
        }
        let actions = live.take_actions();
        for (p, a) in actions {
            // anim
            if self.anim.take(p, a.kind, a.arg) {
                continue;
            }
            if self.cap >= Cap::Pose
                && matches!(
                    a.kind,
                    ActionKind::Shoot | ActionKind::Swing | ActionKind::Reload
                )
            {
                self.peers
                    .entry(p)
                    .or_default()
                    .actions
                    .push((a.kind, a.arg));
            }
        }
    }

    /// The base form player `p`'s avatar is to be placed from: from [`Cap::Face`], once that
    /// player's face is known, a copy of `default` (made once for each face) with it; otherwise
    /// `None` (the caller's default).
    pub fn base_for(&mut self, e: &mut Engine, p: PlayerId, default: u32) -> Option<u32> {
        if self.cap < Cap::Face {
            return None;
        }
        let peer = self.peers.get_mut(&p)?;
        let face = peer.face.clone()?;
        if let Some((made_for, id)) = &peer.base {
            if *made_for == face {
                return Some(*id);
            }
        }
        if peer.failed.as_ref() == Some(&face) {
            return None;
        }
        match look::base_copy(e, default) {
            Some(id) if look::set_face(e, id, &face) => {
                peer.base = Some((face.clone(), id));
                self.counts.bases += 1;
                self.notes.push(format!(
                    "co-op look: player {} has base form {id:08x} with their look ({})",
                    p.0, face.name
                ));
                Some(id)
            }
            _ => {
                peer.failed = Some(face);
                self.notes.push(format!(
                    "co-op look: no base form could be made with player {}'s look: their avatar keeps base {default:08x}",
                    p.0
                ));
                None
            }
        }
    }

    /// One frame of player `p`'s avatar `avatar`, shown at `s`: its gait, its weapon and its
    /// actions, and its items, as far as the cap goes.
    pub fn frame(&mut self, e: &mut Engine, p: PlayerId, avatar: u32, s: &Sample, now_ms: u64) {
        self.seen.insert(avatar);
        // anim: the gait with its turns on the spot; none forced while it uses furniture.
        let moves = self.anim.moves(avatar, s, now_ms);
        let shown = self.shown.entry(avatar).or_default();
        match moves {
            Some(m) if shown.forced != Some(m) && look::gait(e, avatar, m) => {
                shown.forced = Some(m);
                self.counts.gaits += 1;
            }
            None => shown.forced = None,
            _ => {}
        }
        if look::calm(e, avatar) {
            self.counts.calmed += 1;
        }
        // anim: its other animations, and its head from its own base form once it has its 3D.
        self.anim.frame(e, p, avatar, s, now_ms);
        let made = self
            .peers
            .get(&p)
            .and_then(|peer| peer.base.as_ref().map(|b| b.1));
        let shown = self.shown.entry(avatar).or_default();
        if self.cap >= Cap::Face
            && !shown.head
            && made.is_some()
            && look::base_of(e, avatar) == made
            && look::rebuild_head(e, avatar)
        {
            shown.head = true;
            self.counts.heads += 1;
        }
        if self.cap < Cap::Pose {
            return;
        }
        // Its items first: the weapon is in hand before it is drawn.
        let peer = self.peers.entry(p).or_default();
        let known = self.cap >= Cap::Gear && peer.equipment.is_some();
        let mut want = if known {
            peer.equipment.clone().unwrap_or_default()
        } else {
            Vec::new()
        };
        if s.weapon != 0 && !want.contains(&s.weapon) {
            want.push(s.weapon);
        }
        let shown = self.shown.entry(avatar).or_default();
        let due = match &shown.worn {
            Some((last, at)) => *last != want || now_ms.saturating_sub(*at) >= WEAR_EVERY_MS,
            None => true,
        };
        if due && (known || !want.is_empty()) {
            shown.worn = Some((want.clone(), now_ms));
            let w = look::wear(e, avatar, &want, known);
            self.counts.put_on += u64::from(w.put_on);
            self.counts.taken_off += u64::from(w.taken_off);
        }
        let want_drawn = s.flags & FLAG_DRAWN != 0;
        let drawn = look::pose_of_id(e, avatar).is_some_and(|q| q.drawn);
        let shown = self.shown.entry(avatar).or_default();
        if drawn != want_drawn {
            let ask = match shown.drawn_asked {
                Some((asked, at)) => {
                    asked != want_drawn || now_ms.saturating_sub(at) >= DRAW_EVERY_MS
                }
                None => true,
            };
            if ask && look::draw(e, avatar, want_drawn) {
                shown.drawn_asked = Some((want_drawn, now_ms));
                self.counts.draws += 1;
            }
        } else {
            shown.drawn_asked = None;
        }
        let actions = std::mem::take(&mut self.peers.entry(p).or_default().actions);
        for (kind, arg) in actions {
            match kind {
                ActionKind::Reload => {
                    if look::reload(e, avatar) {
                        self.counts.reloads += 1;
                    }
                }
                _ => {
                    if look::attack(e, avatar, arg as u8) {
                        self.counts.attacks += 1;
                    } else {
                        self.counts.attacks_refused += 1;
                    }
                }
            }
        }
    }

    /// Ends a frame of the avatars: what was kept for one no longer shown is let go.
    pub fn end_frame(&mut self) {
        let seen = std::mem::take(&mut self.seen);
        self.anim.end_frame(&seen); // anim
        self.shown.retain(|id, _| seen.contains(id));
    }

    /// A player who left: what was known of them is forgotten.
    pub fn forget(&mut self, p: PlayerId) {
        self.peers.remove(&p);
        self.anim.forget(p); // anim
    }

    /// One status phrase for the log.
    pub fn status(&self) -> String {
        let c = &self.counts;
        format!(
            "look {:?}: gaits {}, calmed {}, draws {}, attacks {} (refused {}), reloads {}, put on {}, taken off {}, bases {}, heads {}, parts sent {}, actions sent {}; {}",
            self.cap,
            c.gaits,
            c.calmed,
            c.draws,
            c.attacks,
            c.attacks_refused,
            c.reloads,
            c.put_on,
            c.taken_off,
            c.bases,
            c.heads,
            c.parts_sent,
            c.actions_sent,
            self.anim.status()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn face() -> Face {
        Face {
            female: true,
            race: 0x0010_7100,
            hair: 0x0010_7101,
            hair_length: 0.25,
            eyes: 0x0010_7102,
            hair_color: 0x0040_2010,
            height: 1.02,
            weight: 0.5,
            head_parts: vec![0x0010_7103, 0x0010_7104],
            coord: [
                (50, 1, (0..50).map(|i| i as f32 * 0.125 - 3.0).collect()),
                (30, 1, (0..30).map(|i| -(i as f32) * 0.25).collect()),
                (50, 1, (0..50).map(|i| i as f32 * 0.5).collect()),
                (0, 0, Vec::new()),
            ],
            name: "Courier Two".into(),
        }
    }

    #[test]
    fn a_face_the_size_of_the_games_goes_and_comes_back() {
        let f = face();
        let b = encode(&f).expect("it fits");
        assert!(b.len() <= MAX_APPEARANCE);
        assert_eq!(decode(&b), Some(f));
    }

    #[test]
    fn values_travel_in_thousandths_within_range() {
        let mut f = face();
        f.coord[0] = (2, 1, vec![0.12345, 99.0]);
        let back = decode(&encode(&f).unwrap()).unwrap();
        assert!((back.coord[0].2[0] - 0.123).abs() < 1e-6);
        assert!((back.coord[0].2[1] - 32.767).abs() < 1e-4);
    }

    #[test]
    fn anything_else_is_no_face() {
        let b = encode(&face()).unwrap();
        assert_eq!(decode(&[]), None);
        assert_eq!(decode(&b[..b.len() - 1]), None);
        let mut longer = b.clone();
        longer.push(0);
        assert_eq!(decode(&longer), None);
        let mut other = b;
        other[1] = APPEARANCE_VERSION + 1;
        assert_eq!(decode(&other), None);
        // A face too large for the parts is not sent.
        let mut big = face();
        big.coord[3] = (200, 1, vec![0.0; 200]);
        assert_eq!(encode(&big), None);
        // A matrix whose values do not match its size is not either.
        let mut bad = face();
        bad.coord[0].2.pop();
        assert_eq!(encode(&bad), None);
    }

    #[test]
    fn the_cap_reads_its_words() {
        assert_eq!(Cap::parse("walk"), Some(Cap::Walk));
        assert_eq!(Cap::parse(" pose "), Some(Cap::Pose));
        assert_eq!(Cap::parse("gear"), Some(Cap::Gear));
        assert_eq!(Cap::parse("on"), Some(Cap::Face));
        // anim: on unless asked otherwise.
        assert_eq!(Cap::parse(""), Some(Cap::Face));
        assert_eq!(Cap::parse("face"), Some(Cap::Face));
        assert_eq!(Cap::parse("off"), None);
        assert_eq!(Cap::parse(" 0 "), None);
    }
}
