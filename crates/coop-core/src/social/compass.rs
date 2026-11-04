//! Teammates, pings and markers on the compass and the map, and the name tag
//! numbers. Pure math for the viewer's HUD: nothing here draws.
//!
//! Conventions (the game's): heading is a rotation about Z, clockwise seen
//! from above, 0 facing +Y (north), so the forward vector is
//! `(sin h, cos h)` and a target at bearing `b` relative to the heading is to
//! the right when `b > 0`. World units are the game's (about 70 to the metre).

use std::collections::BTreeMap;
use std::f32::consts::PI;

use super::{Marker, SocialMsg};
use crate::ids::{CellId, PlayerId};

/// Marker kind: a ping (short lived, "look here").
pub const KIND_PING: u8 = 1;
/// Marker kind: a waypoint (lasts until cleared).
pub const KIND_WAYPOINT: u8 = 2;
/// Marker kind: danger.
pub const KIND_DANGER: u8 = 3;
/// Marker kind a teammate's position takes on the compass (not on the wire).
pub const KIND_TEAMMATE: u8 = 0;

/// How long a ping lasts.
pub const PING_TTL_MS: u32 = 8_000;
/// Least time between two pings from this player.
pub const PING_COOLDOWN_MS: u64 = 600;
/// A ping is placed this far along the view ray when nothing was hit.
pub const PING_MAX_RANGE: f32 = 12_000.0;
/// Marker ids from this number up belong to pings (the rest to waypoints).
pub const PING_ID_BASE: u16 = 100;
/// Pings one player keeps at once (the oldest goes first).
pub const PINGS_KEPT: usize = 3;

/// Compass half field of view in radians (the strip shows 180 degrees).
pub const COMPASS_HALF_FOV: f32 = PI / 2.0;
/// Teammates farther than this are not named on the compass strip (units).
pub const TAG_RANGE: f32 = 9_000.0;
/// Tags fade out over the last part of this range (units).
pub const TAG_FADE_FROM: f32 = 6_000.0;

/// The local player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Listener {
    /// Cell the player stands in.
    pub cell: u32,
    /// Feet position.
    pub pos: [f32; 3],
    /// Heading in radians.
    pub heading: f32,
}

/// A thing to show: a teammate or a marker.
#[derive(Clone, Debug, PartialEq)]
pub struct Pin {
    /// Who it belongs to.
    pub owner: PlayerId,
    /// [`KIND_TEAMMATE`], [`KIND_PING`], ...
    pub kind: u8,
    /// Cell.
    pub cell: u32,
    /// World position.
    pub pos: [f32; 3],
    /// Name or label.
    pub label: String,
    /// Health 0..=1 for teammates.
    pub health: Option<f32>,
    /// Teammate is downed.
    pub downed: bool,
}

/// Wraps an angle to -PI..=PI.
pub fn wrap(a: f32) -> f32 {
    let mut a = a % (2.0 * PI);
    if a > PI {
        a -= 2.0 * PI;
    } else if a < -PI {
        a += 2.0 * PI;
    }
    a
}

/// Bearing from the listener to a point, relative to where it faces
/// (positive: to its right), and the horizontal distance.
pub fn bearing(l: &Listener, to: [f32; 3]) -> (f32, f32) {
    let dx = to[0] - l.pos[0];
    let dy = to[1] - l.pos[1];
    let d = (dx * dx + dy * dy).sqrt();
    if d < 1e-3 {
        return (0.0, 0.0);
    }
    (wrap(dx.atan2(dy) - l.heading), d)
}

/// Which compass edge an off-strip pin sticks to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    /// Inside the strip.
    None,
    /// Clamped to the left end.
    Left,
    /// Clamped to the right end.
    Right,
}

/// One glyph on the compass strip.
#[derive(Clone, Debug, PartialEq)]
pub struct CompassEntry {
    /// Owner.
    pub owner: PlayerId,
    /// Kind.
    pub kind: u8,
    /// Position across the strip, -1 (left end) to 1 (right end).
    pub x: f32,
    /// The pin is outside the strip's field of view and stuck to an end.
    pub edge: Edge,
    /// Horizontal distance in units.
    pub distance: f32,
    /// Height of the pin above the listener (arrows up or down).
    pub dz: f32,
    /// Opacity 0..=1 (fades with distance for teammates).
    pub alpha: f32,
    /// Name or label.
    pub label: String,
}

/// Compass glyphs for pins in the listener's cell, nearest first. Pings and
/// markers keep to the strip ends when behind the player; teammates beyond
/// [`TAG_RANGE`] are left out.
pub fn compass(l: &Listener, pins: &[Pin]) -> Vec<CompassEntry> {
    let mut out: Vec<CompassEntry> = Vec::new();
    for p in pins.iter().filter(|p| p.cell == l.cell) {
        let (b, d) = bearing(l, p.pos);
        if p.kind == KIND_TEAMMATE && d > TAG_RANGE {
            continue;
        }
        let inside = b.abs() <= COMPASS_HALF_FOV;
        let (x, edge) = if inside {
            (b / COMPASS_HALF_FOV, Edge::None)
        } else if b > 0.0 {
            (1.0, Edge::Right)
        } else {
            (-1.0, Edge::Left)
        };
        let alpha = if p.kind == KIND_TEAMMATE {
            fade(d)
        } else {
            1.0
        };
        out.push(CompassEntry {
            owner: p.owner,
            kind: p.kind,
            x,
            edge,
            distance: d,
            dz: p.pos[2] - l.pos[2],
            alpha,
            label: p.label.clone(),
        });
    }
    out.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

fn fade(d: f32) -> f32 {
    if d <= TAG_FADE_FROM {
        1.0
    } else {
        (1.0 - (d - TAG_FADE_FROM) / (TAG_RANGE - TAG_FADE_FROM)).clamp(0.0, 1.0)
    }
}

/// One glyph on the Pip-Boy map or the local map. The map code converts
/// the world position itself; cells that are not the map's are left out by
/// the caller (`on_map_cell`).
#[derive(Clone, Debug, PartialEq)]
pub struct MapGlyph {
    /// Owner.
    pub owner: PlayerId,
    /// Kind.
    pub kind: u8,
    /// Cell.
    pub cell: u32,
    /// World position.
    pub pos: [f32; 3],
    /// Label shown on hover.
    pub label: String,
    /// Palette slot of the owner (see [`color_slot`]).
    pub color: u8,
}

/// Glyphs for every pin whose cell the map shows.
pub fn map_glyphs(pins: &[Pin], on_map_cell: impl Fn(u32) -> bool) -> Vec<MapGlyph> {
    pins.iter()
        .filter(|p| on_map_cell(p.cell))
        .map(|p| MapGlyph {
            owner: p.owner,
            kind: p.kind,
            cell: p.cell,
            pos: p.pos,
            label: p.label.clone(),
            color: color_slot(p.owner),
        })
        .collect()
}

/// A stable colour slot (0..7) for a player: the host and each guest keep
/// theirs for the whole session.
pub fn color_slot(p: PlayerId) -> u8 {
    p.0 % 8
}

/// The numbers for one name tag over a teammate's head.
#[derive(Clone, Debug, PartialEq)]
pub struct NameTag {
    /// Owner.
    pub owner: PlayerId,
    /// Name shown.
    pub name: String,
    /// Health bar fill 0..=1.
    pub health: f32,
    /// Show the bar (hidden at full health and far away).
    pub show_bar: bool,
    /// Opacity.
    pub alpha: f32,
    /// Bar colour: 0 healthy, 1 hurt, 2 critical, 3 downed.
    pub tone: u8,
    /// Speaking indicator (voice).
    pub speaking: bool,
    /// Distance in units.
    pub distance: f32,
}

/// Builds a tag for a teammate.
pub fn name_tag(l: &Listener, p: &Pin, speaking: bool) -> Option<NameTag> {
    if p.kind != KIND_TEAMMATE || p.cell != l.cell {
        return None;
    }
    let (_, d) = bearing(l, p.pos);
    let alpha = fade(d);
    if alpha <= 0.0 {
        return None;
    }
    let health = p.health.unwrap_or(1.0).clamp(0.0, 1.0);
    let tone = if p.downed {
        3
    } else if health < 0.25 {
        2
    } else if health < 0.6 {
        1
    } else {
        0
    };
    Some(NameTag {
        owner: p.owner,
        name: p.label.clone(),
        health,
        show_bar: health < 0.999 || d < TAG_FADE_FROM / 2.0,
        alpha,
        tone,
        speaking,
        distance: d,
    })
}

/// Where a ping lands: the first thing the view ray hit, else far along it.
pub fn ping_point(eye: [f32; 3], dir: [f32; 3], hit_distance: Option<f32>) -> [f32; 3] {
    let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2])
        .sqrt()
        .max(1e-6);
    let t = hit_distance
        .unwrap_or(PING_MAX_RANGE)
        .clamp(0.0, PING_MAX_RANGE);
    [
        eye[0] + dir[0] / len * t,
        eye[1] + dir[1] / len * t,
        eye[2] + dir[2] / len * t,
    ]
}

fn fix(v: f32) -> i32 {
    (v * 16.0).round() as i32
}

/// The local player's own pings and waypoints: lifetimes, the cooldown and
/// the messages to send. The host expires everyone's markers; this keeps the
/// owner's copy in step so it can draw them without waiting for the host.
#[derive(Clone, Debug, Default)]
pub struct PingBook {
    mine: BTreeMap<u16, (Pin, Option<u64>)>,
    order: Vec<u16>,
    next: u16,
    last_ping_ms: Option<u64>,
    /// Pings sent.
    pub pings: u32,
    /// Pings refused by the cooldown.
    pub refused: u32,
}

impl PingBook {
    /// Empty book.
    pub fn new() -> Self {
        Self::default()
    }

    /// Places a ping. `None` while the cooldown runs; otherwise the message
    /// for the host. The oldest ping is cleared when too many are out.
    pub fn ping(
        &mut self,
        me: PlayerId,
        cell: u32,
        pos: [f32; 3],
        label: &str,
        now_ms: u64,
    ) -> Option<Vec<SocialMsg>> {
        if self
            .last_ping_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < PING_COOLDOWN_MS)
        {
            self.refused += 1;
            return None;
        }
        self.last_ping_ms = Some(now_ms);
        self.pings += 1;
        let mut msgs = Vec::new();
        let kept: Vec<u16> = self
            .order
            .iter()
            .copied()
            .filter(|i| *i >= PING_ID_BASE)
            .collect();
        if kept.len() >= PINGS_KEPT {
            let old = kept[0];
            self.mine.remove(&old);
            self.order.retain(|i| *i != old);
            msgs.push(SocialMsg::MarkerClear { id: old });
        }
        let id = PING_ID_BASE + self.next % 100;
        self.next = self.next.wrapping_add(1);
        self.mine.remove(&id);
        self.order.retain(|i| *i != id);
        self.order.push(id);
        self.mine.insert(
            id,
            (
                Pin {
                    owner: me,
                    kind: KIND_PING,
                    cell,
                    pos,
                    label: label.to_string(),
                    health: None,
                    downed: false,
                },
                Some(now_ms + u64::from(PING_TTL_MS)),
            ),
        );
        msgs.push(SocialMsg::MarkerSet {
            marker: Marker {
                id,
                kind: KIND_PING,
                cell: CellId(cell),
                pos: [fix(pos[0]), fix(pos[1]), fix(pos[2])],
                label: label.to_string(),
                ttl_ms: PING_TTL_MS,
            },
        });
        Some(msgs)
    }

    /// Sets (or moves) the single waypoint, which lasts until cleared.
    pub fn waypoint(&mut self, me: PlayerId, cell: u32, pos: [f32; 3], label: &str) -> SocialMsg {
        self.mine.insert(
            1,
            (
                Pin {
                    owner: me,
                    kind: KIND_WAYPOINT,
                    cell,
                    pos,
                    label: label.to_string(),
                    health: None,
                    downed: false,
                },
                None,
            ),
        );
        if !self.order.contains(&1) {
            self.order.push(1);
        }
        SocialMsg::MarkerSet {
            marker: Marker {
                id: 1,
                kind: KIND_WAYPOINT,
                cell: CellId(cell),
                pos: [fix(pos[0]), fix(pos[1]), fix(pos[2])],
                label: label.to_string(),
                ttl_ms: 0,
            },
        }
    }

    /// Clears the waypoint.
    pub fn clear_waypoint(&mut self) -> Option<SocialMsg> {
        self.order.retain(|i| *i != 1);
        self.mine
            .remove(&1)
            .map(|_| SocialMsg::MarkerClear { id: 1 })
    }

    /// Drops expired pings.
    pub fn tick(&mut self, now_ms: u64) {
        let gone: Vec<u16> = self
            .mine
            .iter()
            .filter(|(_, (_, e))| e.is_some_and(|e| now_ms >= e))
            .map(|(i, _)| *i)
            .collect();
        for i in gone {
            self.mine.remove(&i);
            self.order.retain(|x| *x != i);
        }
    }

    /// The local player's markers still alive.
    pub fn pins(&self) -> Vec<Pin> {
        self.mine.values().map(|(p, _)| p.clone()).collect()
    }
}

/// Everything the HUD draws from one live frame: teammates (with health) and
/// other players' markers, plus the local player's own.
pub fn collect_pins(
    teammates: impl IntoIterator<Item = (PlayerId, String, u32, [f32; 3], f32, bool)>,
    markers: impl IntoIterator<Item = (PlayerId, u8, u32, [f32; 3], String)>,
    own: &[Pin],
) -> Vec<Pin> {
    let mut v: Vec<Pin> = teammates
        .into_iter()
        .map(|(owner, name, cell, pos, health, downed)| Pin {
            owner,
            kind: KIND_TEAMMATE,
            cell,
            pos,
            label: name,
            health: Some(health),
            downed,
        })
        .collect();
    v.extend(
        markers
            .into_iter()
            .map(|(owner, kind, cell, pos, label)| Pin {
                owner,
                kind,
                cell,
                pos,
                label,
                health: None,
                downed: false,
            }),
    );
    v.extend(own.iter().cloned());
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const ME: PlayerId = PlayerId(0);
    const ANN: PlayerId = PlayerId(1);

    fn at(h: f32) -> Listener {
        Listener {
            cell: 7,
            pos: [0.0, 0.0, 0.0],
            heading: h,
        }
    }

    fn pin(kind: u8, pos: [f32; 3]) -> Pin {
        Pin {
            owner: ANN,
            kind,
            cell: 7,
            pos,
            label: "Ann".into(),
            health: Some(1.0),
            downed: false,
        }
    }

    #[test]
    fn bearing_uses_the_game_convention() {
        let (b, d) = bearing(&at(0.0), [0.0, 100.0, 0.0]);
        assert!(b.abs() < 1e-5 && (d - 100.0).abs() < 1e-3, "north");
        let (b, _) = bearing(&at(0.0), [100.0, 0.0, 0.0]);
        assert!((b - PI / 2.0).abs() < 1e-5, "east is right of north: {b}");
        let (b, _) = bearing(&at(PI / 2.0), [0.0, 100.0, 0.0]);
        assert!(
            (b + PI / 2.0).abs() < 1e-5,
            "facing east, north is left: {b}"
        );
        let (b, _) = bearing(&at(0.0), [0.0, -100.0, 0.0]);
        assert!((b.abs() - PI).abs() < 1e-5, "behind");
    }

    #[test]
    fn compass_places_strip_entries_and_edges() {
        let pins = vec![
            pin(KIND_TEAMMATE, [0.0, 500.0, 0.0]),
            pin(KIND_PING, [500.0, 0.0, 30.0]),
            pin(KIND_PING, [-500.0, -10.0, 0.0]),
        ];
        let c = compass(&at(0.0), &pins);
        assert_eq!(c.len(), 3);
        let ahead = c.iter().find(|e| e.kind == KIND_TEAMMATE).unwrap();
        assert!(ahead.x.abs() < 1e-5 && ahead.edge == Edge::None);
        let east = c.iter().find(|e| e.kind == KIND_PING && e.x > 0.9).unwrap();
        assert_eq!(
            east.edge,
            Edge::None,
            "exactly 90 degrees is on the strip's end"
        );
        assert!((east.dz - 30.0).abs() < 1e-5);
        let back = c.iter().find(|e| e.x < -0.9).unwrap();
        assert!(back.x <= -0.99);
        let behind = compass(&at(0.0), &[pin(KIND_PING, [10.0, -500.0, 0.0])]);
        assert_eq!(
            behind[0].edge,
            Edge::Right,
            "a bit east of straight behind sticks right"
        );
        let behind = compass(&at(0.0), &[pin(KIND_PING, [-10.0, -500.0, 0.0])]);
        assert_eq!(behind[0].edge, Edge::Left);
    }

    #[test]
    fn other_cells_and_far_teammates_are_left_out() {
        let mut other = pin(KIND_PING, [0.0, 10.0, 0.0]);
        other.cell = 8;
        let far = pin(KIND_TEAMMATE, [0.0, TAG_RANGE + 1.0, 0.0]);
        let far_ping = pin(KIND_PING, [0.0, TAG_RANGE + 1.0, 0.0]);
        let c = compass(&at(0.0), &[other, far, far_ping]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].kind, KIND_PING, "a far ping still shows");
    }

    #[test]
    fn teammates_fade_with_distance_and_sort_nearest_first() {
        let c = compass(
            &at(0.0),
            &[
                pin(KIND_TEAMMATE, [0.0, 8000.0, 0.0]),
                pin(KIND_TEAMMATE, [0.0, 100.0, 0.0]),
            ],
        );
        assert!(c[0].distance < c[1].distance);
        assert_eq!(c[0].alpha, 1.0);
        assert!(c[1].alpha > 0.0 && c[1].alpha < 1.0);
    }

    #[test]
    fn name_tag_tones_and_bar() {
        let l = at(0.0);
        let mut p = pin(KIND_TEAMMATE, [0.0, 4000.0, 0.0]);
        let t = name_tag(&l, &p, false).unwrap();
        assert!(!t.show_bar, "full health far away: no bar");
        p.health = Some(0.5);
        let t = name_tag(&l, &p, true).unwrap();
        assert!(t.show_bar && t.tone == 1 && t.speaking);
        p.health = Some(0.1);
        assert_eq!(name_tag(&l, &p, false).unwrap().tone, 2);
        p.downed = true;
        assert_eq!(name_tag(&l, &p, false).unwrap().tone, 3);
        assert!(name_tag(&l, &pin(KIND_PING, [0.0, 1.0, 0.0]), false).is_none());
        p.pos[1] = TAG_RANGE + 10.0;
        assert!(name_tag(&l, &p, false).is_none());
    }

    #[test]
    fn map_glyphs_follow_the_map_cell() {
        let pins = vec![
            pin(KIND_TEAMMATE, [1.0, 2.0, 3.0]),
            Pin {
                cell: 9,
                ..pin(KIND_PING, [0.0; 3])
            },
        ];
        let g = map_glyphs(&pins, |c| c == 7);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].color, color_slot(ANN));
        assert_eq!(map_glyphs(&pins, |_| true).len(), 2);
    }

    #[test]
    fn ping_point_hits_or_goes_far() {
        let p = ping_point([0.0, 0.0, 100.0], [0.0, 2.0, 0.0], Some(500.0));
        assert!((p[1] - 500.0).abs() < 1e-3);
        let far = ping_point([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], None);
        assert_eq!(far[1], PING_MAX_RANGE);
        let capped = ping_point([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], Some(1e9));
        assert_eq!(capped[0], PING_MAX_RANGE);
    }

    #[test]
    fn pings_have_a_cooldown_a_lifetime_and_a_cap() {
        let mut b = PingBook::new();
        let m = b.ping(ME, 7, [1.0, 2.0, 3.0], "ping", 1000).unwrap();
        assert!(
            matches!(&m[0], SocialMsg::MarkerSet { marker } if marker.ttl_ms == PING_TTL_MS && marker.kind == KIND_PING)
        );
        assert!(b.ping(ME, 7, [0.0; 3], "ping", 1300).is_none(), "cooldown");
        assert_eq!(b.refused, 1);
        for i in 0..3u64 {
            b.ping(ME, 7, [i as f32, 0.0, 0.0], "ping", 2000 + i * 1000)
                .unwrap();
        }
        assert_eq!(b.pins().len(), PINGS_KEPT, "oldest cleared");
        let last = b.ping(ME, 7, [9.0, 0.0, 0.0], "ping", 6000).unwrap();
        assert!(
            matches!(last[0], SocialMsg::MarkerClear { .. }),
            "the cap clears the oldest first"
        );
        b.tick(6000 + u64::from(PING_TTL_MS) + 1);
        assert!(b.pins().is_empty(), "expired");
    }

    #[test]
    fn waypoint_set_move_clear() {
        let mut b = PingBook::new();
        b.waypoint(ME, 7, [1.0, 1.0, 0.0], "camp");
        b.waypoint(ME, 7, [5.0, 5.0, 0.0], "camp");
        assert_eq!(b.pins().len(), 1);
        b.tick(u64::MAX / 2);
        assert_eq!(b.pins().len(), 1, "waypoints do not expire");
        assert_eq!(b.clear_waypoint(), Some(SocialMsg::MarkerClear { id: 1 }));
        assert_eq!(b.clear_waypoint(), None);
    }

    #[test]
    fn wrap_angles() {
        assert!((wrap(3.0 * PI) - PI).abs() < 1e-4 || (wrap(3.0 * PI) + PI).abs() < 1e-4);
        assert!((wrap(-0.5) + 0.5).abs() < 1e-6);
    }

    #[test]
    fn collect_pins_merges_sources() {
        let own = vec![pin(KIND_PING, [0.0; 3])];
        let v = collect_pins(
            vec![(ANN, "Ann".to_string(), 7u32, [0.0f32; 3], 0.5f32, false)],
            vec![(PlayerId(2), KIND_DANGER, 7u32, [1.0f32; 3], "x".to_string())],
            &own,
        );
        assert_eq!(v.len(), 3);
    }
}
