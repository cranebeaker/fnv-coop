//! References made while playing: `PlaceAtMe` (`005c4a70` → `005c4b30`),
//! `PlaceLeveledActorAtMe` (`005d9810`), and things dropped into the world
//! ([`drop_into_world`]).
//!
//! A made reference gets a form ID of its own (the game's run-time ones
//! start with 0xFF; nv-rs counts them up from 0xFF000001, the game's own
//! counter isn't traced), its base, its place and turn, and lives in
//! [`Placed`] (saved). [`crate::scripting::GameState::place`] and the
//! questions about a reference's base know them; the viewer is told
//! ([`super::Shown::Placed`]).

use std::collections::BTreeMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_f32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{base_of, Event, GameState, Runner};

const LVLC: FourCC = FourCC::new(b"LVLC");
const LVLN: FourCC = FourCC::new(b"LVLN");
const NPC_: FourCC = FourCC::new(b"NPC_");
const CREA: FourCC = FourCC::new(b"CREA");
const DATA: FourCC = FourCC::new(b"DATA");

/// The first form ID nv-rs gives a made reference.
pub const FIRST_ID: u32 = 0xFF00_0001;

/// How far around the caller the second to ninth references go
/// (`005c4b30`: 100 units, at eight headings 45 degrees apart).
pub const RING: f32 = 100.0;

/// A reference made while playing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Made {
    pub base: FormId,
    /// The interior cell or worldspace, and the cell (as
    /// [`GameState::place`] gives them).
    pub space: FormId,
    pub cell: FormId,
    pub position: [f32; 3],
    /// Radians, as a placed reference's `DATA` (the caller's turn,
    /// `005c4b30` hands the caller's rotation, ref+0x24, to the new one).
    pub rotation: [f32; 3],
    /// How many of an item it stands for (the extra count, `00419ad0`,
    /// for more than one dropped); 1 otherwise.
    pub count: i32,
}

/// The references made so far.
#[derive(Debug, Clone, Default)]
pub struct Placed {
    pub refs: BTreeMap<FormId, Made>,
    /// The next form ID's low bits (0 before any).
    pub next: u32,
}

impl Placed {
    fn new_id(&mut self) -> FormId {
        let id = FormId(FIRST_ID + self.next);
        self.next += 1;
        id
    }
}

/// A made reference's base.
pub fn base(state: &GameState, reference: FormId) -> Option<FormId> {
    state.more.placed.refs.get(&reference).map(|m| m.base)
}

/// A reference's base, made or placed.
pub fn base_now(order: &LoadOrder, state: &GameState, reference: FormId) -> Option<FormId> {
    base(state, reference).or_else(|| base_of(order, reference))
}

fn kind_of(order: &LoadOrder, id: FormId) -> Option<FourCC> {
    order.get(id).map(|r| r.entry.header.kind)
}

/// A reference's turn now (radians): a made one's, else its record's
/// `DATA` x and y with its heading now.
fn rotation_of(order: &LoadOrder, state: &GameState, r: FormId) -> [f32; 3] {
    if let Some(m) = state.more.placed.refs.get(&r) {
        let heading = state.positions.get(&r).map_or(m.rotation[2], |p| p.1);
        return [m.rotation[0], m.rotation[1], heading];
    }
    let heading = state.place(order, r).map_or(0.0, |p| p.3);
    let tilt = state.set_by_scripts.tilts.get(&r).copied().or_else(|| {
        let rr = order.get(r)?;
        let record = rr.record().ok()?;
        let d = record.get(DATA).filter(|s| s.data.len() >= 24)?;
        Some([le_f32(&d.data, 12), le_f32(&d.data, 16)])
    });
    let [x, y] = tilt.unwrap_or([0.0; 2]);
    [x, y, heading]
}

/// Whether a point is in the loaded area (`00451110`): always indoors;
/// outdoors within `uGridsToLoad` (5) squares of 4096 units around the
/// player's square, by the point's x and y.
fn loaded(state: &GameState, p: [f32; 3]) -> bool {
    const GRIDS: i32 = 5;
    if state.player_world.is_none() {
        return true;
    }
    let Some(player) = state.player_position else {
        return true;
    };
    let square = |v: f32| (v as i32) >> 12;
    let (cx, cy) = (square(player[0]), square(player[1]));
    let (x, y) = (square(p[0]), square(p[1]));
    let lo = |c: i32| c - GRIDS / 2;
    lo(cx) <= x && x < lo(cx) + GRIDS && lo(cy) <= y && y < lo(cy) + GRIDS
}

/// Makes one reference and tells the viewer.
fn make(
    runner: &mut Runner,
    base: FormId,
    at: (FormId, FormId),
    p: [f32; 3],
    r: [f32; 3],
) -> FormId {
    let id = runner.state.more.placed.new_id();
    runner.state.more.placed.refs.insert(
        id,
        Made {
            base,
            space: at.0,
            cell: at.1,
            position: p,
            rotation: r,
            count: 1,
        },
    );
    runner
        .state
        .events
        .push(Event::More(super::Shown::Placed { reference: id }));
    id
}

/// Someone drops `count` of an item into the world (`RemoveItem` with its
/// drop flag, `004c37d0` → `004c6dd0`): it leaves their things and a new
/// reference of it appears where they stand plus (0, 50, 30) turned by
/// their rotation (`00a59540`, `00416870`, `004b4500`: 50 units in front,
/// 30 up), turned as they are, in their cell, standing for `count` of it
/// (`00419ad0` for more than one). (The game lets it fall from there; no
/// falling here.) Something worn comes off once none is left. Their
/// scripts aren't moved: see [`GameState::scripts_follow`]. Returns the
/// new reference.
pub fn drop_into_world(
    order: &LoadOrder,
    state: &mut GameState,
    holder: FormId,
    item: FormId,
    count: i32,
) -> Option<FormId> {
    let n = count.min(state.item_count(order, holder, item));
    if n <= 0 {
        return None;
    }
    let (space, cell, position, _) = state.place(order, holder)?;
    let rotation = rotation_of(order, state, holder);
    let m = crate::RotationConvention::DEFAULT.matrix(rotation);
    let local = [0.0, 50.0, 30.0];
    let mut at = position;
    for (i, v) in at.iter_mut().enumerate() {
        *v += (0..3).map(|j| m[i][j] * local[j]).sum::<f32>();
    }
    state.stock(order, holder);
    let key = (holder, item);
    if let Some(left) = state.items.get_mut(&key) {
        *left -= n;
        if *left <= 0 {
            state.items.remove(&key);
            if state.is_equipped(holder, item) {
                state.unequip_item(order, holder, item);
            }
        }
    }
    let id = state.more.placed.new_id();
    state.more.placed.refs.insert(
        id,
        Made {
            base: item,
            space,
            cell,
            position: at,
            rotation,
            count: n,
        },
    );
    state
        .events
        .push(Event::More(super::Shown::Placed { reference: id }));
    Some(id)
}

/// How many of an item a made reference stands for (0 when it isn't one of
/// it, or it's gone: picked up or disabled).
pub fn held_in_world(order: &LoadOrder, state: &GameState, reference: FormId, item: FormId) -> i32 {
    match state.more.placed.refs.get(&reference) {
        Some(m) if m.base == item && crate::enabled_now(order, reference, &state.disabled) => {
            m.count
        }
        _ => 0,
    }
}

/// `PlaceAtMe base [count] [distance] [direction]` on `caller`: `count`
/// references of the base (1 when left out) where the caller is, in the
/// caller's cell, turned as the caller is. The first goes on the caller's
/// spot; the next eight on a ring of 100 units around it at the caller's
/// height, the first of them at a random angle between 0 and 45 degrees
/// clockwise from north and each next one 45 degrees on; then round again
/// (the tenth on the caller's spot). A ring spot outside the loaded area,
/// or every one when the caller stands outside it, is the caller's own.
/// (The game also casts a line to each ring spot and stops short of
/// whatever it meets: not here, the world has no collision.)
///
/// A leveled list of people or creatures (`LVLN`, `LVLC`) gives `count`
/// picks at the player's level (the caller's encounter zone's level when
/// it has one: zones' levels aren't kept here), all on one spot: the
/// caller's, or, `distance` units from it in front (direction 0), behind
/// (1), to the left (2) or right (3) of the caller's turned model when it
/// has one (markers don't). `distance` and `direction` do nothing for
/// other bases. The last reference made is the function's value.
pub fn place_at_me(
    runner: &mut Runner,
    caller: FormId,
    base: FormId,
    count: i32,
    distance: f32,
    direction: i32,
) -> Option<FormId> {
    let order = runner.order;
    let (space, cell, position, _) = runner.state.place(order, caller)?;
    let rotation = rotation_of(order, runner.state, caller);
    // The ring's first heading: a random number in [0, 1) × 45 degrees
    // (`005c5420` × 0.7853982), drawn whatever the base.
    let start =
        (runner.state.roll() % 1_000_000) as f32 / 1_000_000.0 * std::f32::consts::FRAC_PI_4;
    let kind = kind_of(order, base)?;
    let mut last = None;
    if kind == LVLC || kind == LVLN {
        let mut spot = position;
        let modelled = caller == PLAYER_REF
            || base_of(order, caller)
                .and_then(|b| order.get(b).map(|r| (b, r)))
                .is_some_and(|(b, r)| {
                    let model = r
                        .record()
                        .ok()
                        .and_then(|rec| rec.get(FourCC::new(b"MODL")).map(|s| s.zstring()));
                    !crate::is_marker(b, r.entry.header.kind, model.as_deref())
                });
        if distance != 0.0 && modelled {
            let m = crate::RotationConvention::DEFAULT.matrix(rotation);
            // Left or right along the model's x axis, else its y axis (the
            // world matrix's column 0 or 1); in front and right add.
            let column = if (2..=3).contains(&direction) { 0 } else { 1 };
            let sign = if (1..=2).contains(&direction) {
                -1.0
            } else {
                1.0
            };
            for (i, v) in spot.iter_mut().enumerate() {
                *v += sign * distance * m[i][column];
            }
        }
        let level = runner.state.player_level;
        let picked = {
            let state = &mut *runner.state;
            crate::leveled::resolve(order, base, count.max(1), level, &mut || state.roll())
        };
        for (form, n) in picked {
            if !matches!(kind_of(order, form), Some(k) if k == NPC_ || k == CREA) {
                continue;
            }
            for _ in 0..n {
                last = Some(make(runner, form, (space, cell), spot, rotation));
            }
        }
        return last;
    }
    let caller_loaded = loaded(runner.state, position);
    let mut slot = 0usize;
    for _ in 0..count.max(1) {
        let mut spot = position;
        if slot != 0 && caller_loaded {
            let a = start + (slot - 1) as f32 * std::f32::consts::FRAC_PI_4;
            let ring = [
                position[0] + RING * a.sin(),
                position[1] + RING * a.cos(),
                position[2],
            ];
            if loaded(runner.state, ring) {
                spot = ring;
            }
        }
        last = Some(make(runner, base, (space, cell), spot, rotation));
        slot = (slot + 1) % 9;
    }
    last
}

/// `PlaceLeveledActorAtMe actor [level modifier] [level]` (`005d9810`): a
/// new `Character` (for an `NPC_`) or `Creature` (`CREA`) of the base on
/// the caller's spot, in the caller's cell, turned as the caller, and the
/// value. The base's own template (a leveled list) decides who it is, as
/// for any templated actor. The level modifier (default 4) and a forced
/// level go into the new reference's extra data (`00421540`, `00567dd0`);
/// what reads them isn't traced, so they aren't kept.
pub fn place_leveled_actor(runner: &mut Runner, caller: FormId, actor: FormId) -> Option<FormId> {
    let order = runner.order;
    let kind = kind_of(order, actor)?;
    if kind != NPC_ && kind != CREA {
        return None;
    }
    let (space, cell, position, _) = runner.state.place(order, caller)?;
    let rotation = rotation_of(order, runner.state, caller);
    Some(make(runner, actor, (space, cell), position, rotation))
}

/// The ash pile and the goo pile (`ACTI` 0x1B `DefaultAshPile1`, "Ash
/// Pile", and 0x22 `DefaultAshPile2`, "Goo Pile", both in `FalloutNV.esm`;
/// the engine looks them up by these IDs at start, `0046a370`, and keeps
/// them at `011ca27c` and `011ca280`).
pub const ASH_PILE: FormId = FormId(0x1B);
pub const GOO_PILE: FormId = FormId(0x22);

/// `AttachAshPile [kind]` on an actor (`005db870`): a reference of the goo
/// pile when `kind` is 2, else the ash pile, in the actor's cell, turned
/// to the actor's heading, linked to the actor both ways (extra data
/// `ExtraAshPileRef`, `0041e340` on each; kept as
/// [`super::State::ash_piles`]), so that activating the pile activates the
/// corpse (`00573170`: its container, [`crate::activation::stands_for`]).
/// The game places it where a ray from 32 units above the actor's
/// position straight down to 256 below meets the ground, tilted to the
/// ground's slope, else at the actor's position; the world here has no
/// collision, so it goes on the actor's position (their feet). The pile
/// made.
pub fn attach_ash_pile(runner: &mut Runner, actor: FormId, kind: i32) -> Option<FormId> {
    let order = runner.order;
    let (space, cell, position, heading) = runner.state.place(order, actor)?;
    let base = if kind == 2 { GOO_PILE } else { ASH_PILE };
    let pile = make(runner, base, (space, cell), position, [0.0, 0.0, heading]);
    runner.state.more.ash_piles.insert(pile, actor);
    Some(pile)
}

/// `fPlayerDropDistance` (`011d0628`): how far in front of the player a
/// dropped item goes, beyond its own size.
pub const DROP_DISTANCE: f32 = 100.0;

/// An item's size for dropping: half its bounds' (`OBND`) diagonal, a
/// stand-in for the bound radius `009614b0` takes (`0050ebf0`, not
/// traced); 0 without bounds.
fn drop_radius(order: &LoadOrder, item: FormId) -> f32 {
    let Some(record) = order.get(item).and_then(|r| r.record().ok()) else {
        return 0.0;
    };
    let Some(s) = record
        .get(FourCC::new(b"OBND"))
        .filter(|s| s.data.len() >= 12)
    else {
        return 0.0;
    };
    let v = |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
    let d = [v(3) - v(0), v(4) - v(1), v(5) - v(2)];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / 2.0
}

/// The player drops `count` of an item they carry (`00780c50` →
/// `PlayerCharacter` slot 0x3cc): they're taken out of the inventory
/// (taken off when none is left on) and lie in the world as one
/// reference keeping the count, turned as the player. Where: the
/// player's place plus their facing × (the item's size +
/// `fPlayerDropDistance`), the branch of `009614b0` that finds room at
/// once; the game also casts the item's shape there and tries ten
/// headings round the player, turning each drop on from the last, and
/// lets physics drop it: none of that here (the world has no collision).
/// `heading` is the player's facing (radians clockwise from north; the
/// game state doesn't keep the player's, the view does). Returns the new
/// reference, or none when nothing was carried.
pub fn drop_item(
    order: &LoadOrder,
    state: &mut GameState,
    item: FormId,
    count: i32,
    heading: f32,
) -> Option<FormId> {
    let (space, cell, position, _) = state.place(order, PLAYER_REF)?;
    let moved = {
        state.stock(order, PLAYER_REF);
        let have = state.item_count(order, PLAYER_REF, item);
        let n = count.min(have).max(0);
        if n == 0 {
            return None;
        }
        if have == n {
            state.items.remove(&(PLAYER_REF, item));
            if state.is_equipped(PLAYER_REF, item) {
                state.unequip_item(order, PLAYER_REF, item);
            }
        } else {
            state.items.insert((PLAYER_REF, item), have - n);
        }
        n
    };
    let distance = drop_radius(order, item) + DROP_DISTANCE;
    let spot = [
        position[0] + distance * heading.sin(),
        position[1] + distance * heading.cos(),
        position[2],
    ];
    let id = state.more.placed.new_id();
    state.more.placed.refs.insert(
        id,
        Made {
            base: item,
            space,
            cell,
            position: spot,
            rotation: [0.0, 0.0, heading],
            count: moved,
        },
    );
    // Their scripts go with them (`OnDrop`, [`GameState::scripts_follow`]).
    state.scripts_follow(order, PLAYER_REF, id, item, moved);
    state
        .events
        .push(Event::More(super::Shown::Placed { reference: id }));
    Some(id)
}

/// The made item references (dropped items) lying in a place (an interior
/// cell or a worldspace) and not taken, as things E can take: what the
/// placed items of a cell give ([`crate::scripting::Interactive`]), with the
/// base's script, bounds and name. With them, the ash and goo piles
/// standing for a corpse (`AttachAshPile`; E on one searches the corpse,
/// [`crate::activation::stands_for`]).
pub fn dropped_items(
    order: &LoadOrder,
    state: &GameState,
    space: FormId,
) -> Vec<crate::scripting::Interactive> {
    let mut out = Vec::new();
    for (&reference, m) in &state.more.placed.refs {
        if m.space != space || state.disabled.get(&reference) == Some(&true) {
            continue;
        }
        let Some(brr) = order.get(m.base) else {
            continue;
        };
        let kind = brr.entry.header.kind;
        if !crate::scripting::is_item(kind) && !state.more.ash_piles.contains_key(&reference) {
            continue;
        }
        let Ok(record) = brr.record() else { continue };
        let script = record
            .get(FourCC::new(b"SCRI"))
            .filter(|s| s.data.len() >= 4)
            .map(|s| {
                brr.plugin.to_global(FormId(u32::from_le_bytes([
                    s.data[0], s.data[1], s.data[2], s.data[3],
                ])))
            });
        let bounds = record
            .get(FourCC::new(b"OBND"))
            .filter(|s| s.data.len() >= 12)
            .map(|s| {
                let v =
                    |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
                ([v(0), v(1), v(2)], [v(3), v(4), v(5)])
            });
        out.push(crate::scripting::Interactive {
            reference,
            base: m.base,
            script,
            count: m.count.max(1),
            position: m.position,
            rotation: m.rotation,
            scale: 1.0,
            trigger: None,
            bounds,
            name: record.full_name(),
            kind,
        });
    }
    out
}

/// A made reference taken (picked up): it's gone for good (the game deletes
/// a dropped item's reference when it's picked up).
pub fn taken(state: &mut GameState, reference: FormId) {
    state.more.placed.refs.remove(&reference);
}

/// Saved lines.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let p = &state.more.placed;
    if p.next != 0 {
        line(format!("madenext {}", p.next));
    }
    for (id, m) in &p.refs {
        // The count only when it isn't 1 (older saves have none).
        let count = if m.count == 1 {
            String::new()
        } else {
            format!(" {}", m.count)
        };
        line(format!(
            "made {:08X} {:08X} {:08X} {:08X} {} {} {} {} {} {}{count}",
            id.0,
            m.base.0,
            m.space.0,
            m.cell.0,
            m.position[0],
            m.position[1],
            m.position[2],
            m.rotation[0],
            m.rotation[1],
            m.rotation[2]
        ));
    }
}

/// A saved line back.
pub(crate) fn load_line(state: &mut GameState, parts: &[&str]) -> Option<Result<(), String>> {
    let bad = || format!("can't read '{}'", parts.join(" "));
    let form = |i: usize| {
        parts
            .get(i)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .map(FormId)
    };
    let num = |i: usize| parts.get(i).and_then(|s| s.parse::<f32>().ok());
    match *parts.first()? {
        "madenext" => Some(match parts.get(1).and_then(|s| s.parse().ok()) {
            Some(n) => {
                state.more.placed.next = n;
                Ok(())
            }
            None => Err(bad()),
        }),
        "made" => Some(
            (|| {
                let m = Made {
                    base: form(2)?,
                    space: form(3)?,
                    cell: form(4)?,
                    position: [num(5)?, num(6)?, num(7)?],
                    rotation: [num(8)?, num(9)?, num(10)?],
                    count: match parts.get(11) {
                        Some(s) => s.parse().ok()?,
                        None => 1,
                    },
                };
                state.more.placed.refs.insert(form(1)?, m);
                Some(())
            })()
            .ok_or_else(bad),
        ),
        _ => None,
    }
}
