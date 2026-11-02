//! The Pip-Boy's local map (DATA › Local Map, `MapMenu` (Xbox PDB);
//! FalloutNV.exe 1.4.0.525, the research notes `localmap-spec.md`): its
//! rules apart from the drawing.
//!
//! * The map is `uGridsToLoad` (5) × 5 tiles of 4096 units around the
//!   player's tile (`0079d410`; interiors offset by 2048 and turned by the
//!   cell's north marker, `00555b10`); a place on it is (x − left) ÷ width
//!   across and (y − top) ÷ height down (`0079c380`).
//! * Each tile's picture is taken from above (`0054ee80` outdoors,
//!   `0054f500` indoors), 4352 units square (the tile and 256 more), 128
//!   pixels (`0054e830`), one tile a frame while the Pip-Boy comes up
//!   (`0079ffb0`); [`TilePicture`] is where its camera is.
//! * Fog of war (`SeenData` (Xbox PDB)): each 4096-unit cell or interior
//!   section keeps 16 × 16 points 256 units apart; those within
//!   `fSeenDataUpdateRadius` (1024) of the player are seen (`00879ad0`),
//!   the cell fully seen at 248 (`iNumBitsForFullySeen`); a map vertex's
//!   alpha is how many of its 2 × 2 points are seen, ÷ 4 (`00556870`).
//! * Door markers (`0054dcf0`, `0079d410`): doors that lead somewhere,
//!   not hidden from the local map, inside the map, their alpha the fog's.
//! * The zoom (`0079c5a0`): the magnification within `fLocalMapMinZoom`
//!   0.1 .. `fLocalMapMaxZoom` 0.9, the markers' sizes in a straight line
//!   with it.

use std::collections::{HashMap, HashSet};

use esm::{FormId, LoadOrder};

/// A tile's side (a cell), units.
pub const TILE: f32 = 4096.0;
/// Half a picture's side (`0102f05c`): the tile plus 256 on the +x/+y side.
pub const PICTURE_HALF: f32 = 2176.0;
/// A picture's pixels (`0054e830`: 128 × 128).
pub const PICTURE_PIXELS: usize = 128;
/// Tiles a side (`uGridsToLoad`, exe and INI 5).
pub const GRIDS: i32 = 5;
/// The cameras' height above the place (`0102f068` outdoors, `0102f080`
/// indoors).
pub const EXTERIOR_LIFT: f32 = 20_000.0;
pub const INTERIOR_LIFT: f32 = 40_000.0;
/// Outdoors the camera is 32 units south of the tile's middle (`0102f070`).
pub const EXTERIOR_Y_OFFSET: f32 = -32.0;
/// Zoom (`fLocalMapMinZoom`, `fLocalMapMaxZoom`).
pub const MIN_ZOOM: f32 = 0.1;
pub const MAX_ZOOM: f32 = 0.9;
/// The seen points' spacing (`011a31e8`) and the radius (`011ca094`).
pub const SEEN_STEP: f32 = 256.0;
pub const SEEN_RADIUS: f32 = 1024.0;
/// Points seen for a cell to count as fully seen (`iNumBitsForFullySeen`).
pub const FULLY_SEEN: u32 = 248;
/// The player arrow's turn less the heading (`010290d0`).
pub const ARROW_BIAS: f32 = 0.3;
/// The door marker's picture.
pub const DOOR_ICON: &str = "Interface\\Icons\\Local Map\\icon_local_door.dds";
/// The `NorthMarker` static (`00000003`).
pub const NORTH_MARKER: FormId = FormId(3);

/// Turns a point by a cell's north (`00555b10` with `004a0c90`): into the
/// map's frame (`to_map`) or back.
// Translated from 00555b10 + 004a0c90 + 004b3ae0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn rotate_by_north(north: f32, p: [f32; 3], to_map: bool) -> [f32; 3] {
    let a = north * if to_map { -1.0 } else { 1.0 };
    if a == 0.0 {
        return p;
    }
    let (s, c) = a.sin_cos();
    [p[0] * c - p[1] * s, p[0] * s + p[1] * c, p[2]]
}

/// A cell's north (`00542d40`): minus the z turn of its `NorthMarker`
/// reference; 0 without one (and outdoors).
pub fn north_of(order: &LoadOrder, cell: FormId) -> f32 {
    for rr in order.references_in_cell(cell) {
        let Ok(record) = rr.record() else {
            continue;
        };
        let base = record
            .get(esm::FourCC::new(b"NAME"))
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(crate::cell::le_u32(&s.data, 0))));
        if base != Some(NORTH_MARKER) {
            continue;
        }
        if let Some(d) = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 24) {
            return -crate::cell::le_f32(&d.data, 20);
        }
    }
    0.0
}

/// The map's rectangle in world units (`+0xe8 .. +0xf4`): left, top, right,
/// bottom (top north of bottom).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

/// The tile a coordinate is in (`>> 12` of the rounded value, less the
/// interior's 2048).
fn tile_of(v: f32, interior: bool) -> i32 {
    let off = if interior { 0x800 } else { 0 };
    ((v.round() as i32) - off) >> 12
}

/// `0079d410`: the rectangle around the player (already turned into the
/// map's frame indoors), `n` tiles a side.
// Translated from 0079d410 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn map_rect(p: [f32; 3], interior: bool, n: i32) -> MapRect {
    let off = if interior { 0x800 } else { 0 };
    let cx0 = tile_of(p[0], interior) - (n >> 1);
    let cy0 = tile_of(p[1], interior) - (n >> 1);
    MapRect {
        left: (cx0 * 4096 + off) as f32,
        right: ((cx0 + n) * 4096 + off) as f32,
        bottom: (cy0 * 4096 + off) as f32,
        top: ((cy0 + n) * 4096 + off) as f32,
    }
}

/// `0079c380`: a point (in the map's frame) on the map, 0..1 across and
/// down.
// Translated from 0079c380 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn world_to_map(r: &MapRect, p: [f32; 3]) -> [f32; 2] {
    [
        (p[0] - r.left) / (r.right - r.left),
        (p[1] - r.top) / (r.bottom - r.top),
    ]
}

/// One tile's picture: its grid place (0..n, `gy` northward), the world
/// point of its south-west corner (in the map's frame), and where the
/// camera looks from (`0054ee80`, `0054f500`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TilePicture {
    pub gx: i32,
    pub gy: i32,
    pub origin: [f32; 2],
    /// The picture's middle (the camera's x, y in the map's frame).
    pub centre: [f32; 2],
}

/// The tile `counter` of the grid (`0079ffb0`: `gx = counter % n`, `gy =
/// counter / n`), around the player's tile.
// Translated from 0079ffb0 / 0054ee80 / 0054f500 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn tile_picture(p: [f32; 3], interior: bool, n: i32, counter: i32) -> TilePicture {
    let (gx, gy) = (counter % n, counter / n);
    let off = if interior { 2048.0 } else { 0.0 };
    let tx = tile_of(p[0], interior) + gx - (n >> 1);
    let ty = tile_of(p[1], interior) + gy - (n >> 1);
    let origin = [tx as f32 * TILE + off, ty as f32 * TILE + off];
    let y_off = if interior { 0.0 } else { EXTERIOR_Y_OFFSET };
    TilePicture {
        gx,
        gy,
        origin,
        centre: [origin[0] + PICTURE_HALF, origin[1] + PICTURE_HALF + y_off],
    }
}

/// The markers' widths at a magnification (`0079c5a0`): door markers
/// (`fLocalMapMarkerMinSize` 20 .. `MaxSize` 75), quest markers
/// (`fLocalQuestMarkerMinSize` 40 .. 75) and the player's arrow
/// (`fLocalfPlayerCursorMinSize` 40 .. `fLocalPlayerCursorMaxSize` 70).
// Translated from 0079c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn marker_sizes(magnification: f32) -> [f32; 3] {
    let t = ((magnification - MIN_ZOOM) / (MAX_ZOOM - MIN_ZOOM)).clamp(0.0, 1.0);
    [20.0 + 55.0 * t, 40.0 + 35.0 * t, 40.0 + 30.0 * t]
}

/// The arrow's `rotateangle` (`0079dbb0`): −heading − 0.3 − the cell's
/// north (radians).
pub fn arrow_angle(heading: f32, north: f32) -> f32 {
    -heading - ARROW_BIAS - north
}

/// The fog of war: 256 seen points per cell (outdoors, by grid square) or
/// interior section (by cell and section), and the cells fully seen.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Seen {
    pub bits: HashMap<SeenKey, [u32; 8]>,
    pub fully: HashSet<SeenKey>,
    /// The last snapped place (`011ca208`): nothing new until it moves.
    last: Option<[i32; 3]>,
}

/// Whose seen points: an outdoor square, or an interior's section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SeenKey {
    Exterior(FormId, i32, i32),
    Interior(FormId, i32, i32),
}

impl SeenKey {
    /// The section's south-west corner (in the map's frame indoors).
    pub fn origin(&self) -> [f32; 2] {
        match *self {
            SeenKey::Exterior(_, x, y) => [x as f32 * TILE, y as f32 * TILE],
            SeenKey::Interior(_, x, y) => [x as f32 * TILE + 2048.0, y as f32 * TILE + 2048.0],
        }
    }
}

/// Where a point is: its cell or section and its grid point (0..15).
fn locate(space: FormId, interior: bool, p: [f32; 2]) -> (SeenKey, i32, i32) {
    let tx = tile_of(p[0], interior);
    let ty = tile_of(p[1], interior);
    let key = if interior {
        SeenKey::Interior(space, tx, ty)
    } else {
        SeenKey::Exterior(space, tx, ty)
    };
    let o = key.origin();
    let sx = ((p[0].round() - o[0]) / SEEN_STEP) as i32;
    let sy = ((p[1].round() - o[1]) / SEEN_STEP) as i32;
    (key, sx.clamp(0, 15), sy.clamp(0, 15))
}

impl Seen {
    fn bit(&self, key: SeenKey, sx: i32, sy: i32) -> bool {
        if self.fully.contains(&key) {
            return true;
        }
        let b = (sy * 16 + sx) as usize;
        self.bits
            .get(&key)
            .is_some_and(|bits| bits[b >> 5] >> (b & 31) & 1 != 0)
    }

    /// `00556870`: how many of the 2 × 2 grid points at and after a
    /// point (in the map's frame) are seen, each in its own cell or
    /// section.
    // Translated from 00556870 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn count(&self, space: FormId, interior: bool, p: [f32; 2]) -> u32 {
        let mut n = 0;
        for (dx, dy) in [
            (0.0, 0.0),
            (0.0, SEEN_STEP),
            (SEEN_STEP, 0.0),
            (SEEN_STEP, SEEN_STEP),
        ] {
            let (key, sx, sy) = locate(space, interior, [p[0] + dx, p[1] + dy]);
            if self.bit(key, sx, sy) {
                n += 1;
            }
        }
        n
    }

    /// `00555c20` / `00555f60` / `00879ad0`: the player's place (in the map's
    /// frame) snapped to the 256 grid; when it moved, the points within the
    /// radius are seen in its cell or section and the neighbours the
    /// radius reaches (outdoors: up to three; indoors: the 3 × 3 around).
    /// [guess: indoors the game's quirk that never updates the +x / +y
    /// neighbours of an unturned interior isn't copied; all 3 × 3 are.]
    // Translated from 00555c20 / 00555f60 / 00879ad0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update(&mut self, space: FormId, interior: bool, p: [f32; 3]) {
        let (key, sx, sy) = locate(space, interior, [p[0], p[1]]);
        let snapped = [
            sx,
            sy,
            match key {
                SeenKey::Exterior(_, x, y) | SeenKey::Interior(_, x, y) => x * 1000 + y,
            },
        ];
        if self.last == Some(snapped) {
            return;
        }
        self.last = Some(snapped);
        let o = key.origin();
        let point = [o[0] + sx as f32 * SEEN_STEP, o[1] + sy as f32 * SEEN_STEP];
        let (kx, ky) = match key {
            SeenKey::Exterior(_, x, y) | SeenKey::Interior(_, x, y) => (x, y),
        };
        let mut keys = vec![key];
        let near = |v: f32, lo: f32| -> i32 {
            if v - SEEN_RADIUS < lo {
                -1
            } else if v + SEEN_RADIUS > lo + TILE {
                1
            } else {
                0
            }
        };
        let make = |x: i32, y: i32| {
            if interior {
                SeenKey::Interior(space, x, y)
            } else {
                SeenKey::Exterior(space, x, y)
            }
        };
        if interior {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx != 0 || dy != 0 {
                        keys.push(make(kx + dx, ky + dy));
                    }
                }
            }
        } else {
            let dx = near(point[0], o[0]);
            let dy = near(point[1], o[1]);
            if dx != 0 {
                keys.push(make(kx + dx, ky));
            }
            if dy != 0 {
                keys.push(make(kx, ky + dy));
            }
            if dx != 0 && dy != 0 {
                keys.push(make(kx + dx, ky + dy));
            }
        }
        for k in keys {
            if self.fully.contains(&k) {
                continue;
            }
            let origin = k.origin();
            let bits = self.bits.entry(k).or_insert([0; 8]);
            let mut count = 0;
            for b in 0..256usize {
                let set = bits[b >> 5] >> (b & 31) & 1 != 0;
                if set {
                    count += 1;
                    continue;
                }
                let pt = [
                    origin[0] + (b % 16) as f32 * SEEN_STEP,
                    origin[1] + (b / 16) as f32 * SEEN_STEP,
                ];
                let d = ((point[0] - pt[0]).powi(2) + (point[1] - pt[1]).powi(2)).sqrt();
                if d < SEEN_RADIUS {
                    bits[b >> 5] |= 1 << (b & 31);
                    count += 1;
                }
            }
            if count >= FULLY_SEEN {
                self.fully.insert(k);
            }
        }
    }
}

/// A door's marker on the local map.
#[derive(Debug, Clone, PartialEq)]
pub struct DoorMarker {
    pub reference: FormId,
    /// Its place in the world (the map's frame indoors is the caller's).
    pub position: [f32; 3],
    /// Where it leads (`_LocationName`).
    pub name: String,
}

/// `0054dcf0` (`TESObjectCELL::BuildLocalMapDoorList` (Xbox PDB)) and the
/// filter of `0079d410`: a cell's doors that lead somewhere (`XTEL`), not
/// disabled or deleted, not the `PrisonMarker` door (`00000004`), not
/// flagged hidden from the local map (reference flag 0x00800000), whose
/// base isn't flagged 0x40 and whose `FNAM` hasn't the hidden flag 0x04;
/// each named for where it leads [guess: the destination cell's name, or
/// outdoors its worldspace's, as the viewer's doors are labelled: the
/// game's text (`0043a2e0` → `00578870`) isn't traced]. (The other kind of
/// door `005194d0` admits isn't traced.)
// Translated from 0054dcf0 / 0079d410 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn door_markers(
    order: &LoadOrder,
    state: &crate::scripting::GameState,
    cell: FormId,
) -> Vec<DoorMarker> {
    doors_among(order, state, order.references_in_cell(cell))
}

/// [`door_markers`] among some references (an outdoor square's
/// persistent ones too).
pub fn doors_among<'a>(
    order: &'a LoadOrder,
    state: &crate::scripting::GameState,
    refs: impl IntoIterator<Item = esm::RecordRef<'a>>,
) -> Vec<DoorMarker> {
    use crate::cell::{le_f32, le_u32};
    let door_kind = esm::FourCC::new(b"DOOR");
    let mut out = Vec::new();
    for rr in refs {
        let flags = rr.entry.header.flags;
        if flags & 0x20 != 0 || flags & 0x0080_0000 != 0 {
            continue;
        }
        if !crate::placement::enabled_now(order, rr.form_id, &state.disabled) {
            continue;
        }
        let Ok(record) = rr.record() else {
            continue;
        };
        let Some(base) = record
            .get(esm::FourCC::new(b"NAME"))
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        else {
            continue;
        };
        let Some(base_rr) = order.get(base) else {
            continue;
        };
        if base_rr.entry.header.kind != door_kind || base == FormId(4) {
            continue;
        }
        if base_rr.entry.header.flags & 0x40 != 0 || crate::doors::flags(order, base) & 0x04 != 0 {
            continue;
        }
        let Some(xtel) = record
            .get(esm::FourCC::new(b"XTEL"))
            .filter(|s| s.data.len() >= 4)
        else {
            continue;
        };
        let to = rr.plugin.to_global(FormId(le_u32(&xtel.data, 0)));
        let name = order
            .get(to)
            .and_then(|d| order.cell_of(&d).map(|c| (c, order.world_of(&d))))
            .and_then(|(c, w)| {
                let info = crate::cell_info(order, c).ok()?;
                let interior = info.flags & crate::CELL_INTERIOR != 0;
                // The cell's own name, else outdoors its worldspace's.
                Some(match (info.name.clone(), interior, w) {
                    (Some(n), _, _) if !n.is_empty() => n,
                    (_, false, Some(w)) => crate::Worldspace::load(order, w)
                        .ok()
                        .and_then(|w| w.name)
                        .unwrap_or_else(|| info.label()),
                    _ => info.label(),
                })
            })
            .unwrap_or_default();
        let Some(d) = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 12) else {
            continue;
        };
        out.push(DoorMarker {
            reference: rr.form_id,
            position: [le_f32(&d.data, 0), le_f32(&d.data, 4), le_f32(&d.data, 8)],
            name,
        });
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;

    /// `0079d410` / `0079c380`: the 5 × 5 rectangle around the player's tile
    /// and places on it; indoors the tiles are offset by 2048.
    #[test]
    fn the_map_around_the_player() {
        let r = map_rect([1000.0, 5000.0, 0.0], false, 5);
        assert_eq!(
            r,
            MapRect {
                left: -8192.0,
                right: 12288.0,
                bottom: -4096.0,
                top: 16384.0
            }
        );
        let uv = world_to_map(&r, [2048.0, 6144.0, 0.0]);
        assert!((uv[0] - 0.5).abs() < 1e-6 && (uv[1] - 0.5).abs() < 1e-6);
        let inside = map_rect([2400.0, 1600.0, 0.0], true, 5);
        assert_eq!(inside.left, -8192.0 + 2048.0);
        assert_eq!(inside.right - inside.left, 5.0 * TILE);
        let t = tile_picture([1000.0, 5000.0, 0.0], false, 5, 12);
        assert_eq!((t.gx, t.gy), (2, 2));
        assert_eq!(t.origin, [0.0, 4096.0]);
        assert_eq!(t.centre, [2176.0, 4096.0 + 2176.0 - 32.0]);
        let inner = tile_picture([2400.0, 1600.0, 0.0], true, 5, 0);
        assert_eq!(inner.origin, [-2.0 * TILE + 2048.0, -3.0 * TILE + 2048.0]);
    }

    /// `00555b10`: into the map's frame and back.
    #[test]
    fn turning_by_north() {
        let p = [100.0, 0.0, 5.0];
        let m = rotate_by_north(std::f32::consts::FRAC_PI_2, p, true);
        assert!((m[0]).abs() < 1e-4 && (m[1] + 100.0).abs() < 1e-4);
        let back = rotate_by_north(std::f32::consts::FRAC_PI_2, m, false);
        assert!((back[0] - 100.0).abs() < 1e-3 && back[1].abs() < 1e-3);
        assert_eq!(rotate_by_north(0.0, p, true), p);
    }

    /// `00879ad0` / `00556870`: points within 1024 seen; a vertex counts
    /// its 2 × 2 points; nothing new until the player's grid point moves.
    #[test]
    fn the_fog_of_war() {
        let mut seen = Seen::default();
        let w = FormId(0x3C);
        assert_eq!(seen.count(w, false, [512.0, 512.0]), 0);
        seen.update(w, false, [512.0, 512.0, 0.0]);
        assert_eq!(seen.count(w, false, [512.0, 512.0]), 4);
        assert_eq!(seen.count(w, false, [3000.0, 3000.0]), 0);
        // Near the cell's west edge the neighbour is marked too.
        assert!(seen.bits.contains_key(&SeenKey::Exterior(w, -1, 0)));
        let marked = seen.bits.clone();
        seen.update(w, false, [520.0, 520.0, 0.0]);
        assert_eq!(seen.bits, marked);
        assert_eq!(marker_sizes(0.9), [75.0, 75.0, 70.0]);
        assert_eq!(marker_sizes(0.1), [20.0, 40.0, 40.0]);
    }
}
