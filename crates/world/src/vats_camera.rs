//! V.A.T.S.'s camera shots, read from the game's code
//! (`%USERPROFILE%\nv-re\findings\vats_camera_menu.md`) and records: the
//! camera paths (`CPTH`) that pick which shots play for an attack, and the
//! shots (`CAMS`) themselves.
//!
//! - **A shot** (`CAMS`, class `BGSCameraShot`, loader `0058c090`): `MODL`
//!   an animated camera (`meshes\vatscameras\*.nif`: a node whose
//!   transform is keyed, an `NiCamera` under it whose field of view is
//!   keyed), `DATA` (40 bytes, at object `+0x38`): action u32 (0 shoot, 1
//!   fly, 2 hit, 3 zoom), location u32 and target u32 (0 attacker, 1
//!   projectile, 2 target), flags u32 (0x01 the position follows the
//!   location, 0x02 the rotation follows the target, 0x04 don't follow a
//!   bone, 0x08 first-person camera, 0x10 no tracer, 0x20 start at time
//!   zero), the player's, the target's and the world's time multipliers
//!   f32, the longest and shortest time f32, the share of the way between
//!   the actors f32; `MNAM` an image space modifier. A record without
//!   `DATA` keeps the constructor's (`0058bf50`): hit, target, attacker,
//!   flags 3, world × 1, everything else 0.
//! - **A path** (`CPTH`, class `BGSCameraPath`, loader `0058a9e0`):
//!   conditions (`CTDA`), `ANAM` its parent and the path before it, `DATA`
//!   one byte (the zoom: 1 disable, 2 shot list, otherwise default; 0x80
//!   see [`CameraPaths::pick`]), `SNAM` its shots in order.
//! - **The tree** (`0058aef0`, run over the paths in the order they were
//!   made): each path goes into its parent's children (or the top-level
//!   list at `[011ca700]` when it has none) right after the path `ANAM`
//!   names before it; with none, at the front; when that one isn't in the
//!   list (yet), or the path is deleted (form flag 0x20), at the end.

use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::{read_condition, Condition};

const CPTH: FourCC = FourCC::new(b"CPTH");
const CAMS: FourCC = FourCC::new(b"CAMS");
const CTDA: FourCC = FourCC::new(b"CTDA");
const ANAM: FourCC = FourCC::new(b"ANAM");
const SNAM: FourCC = FourCC::new(b"SNAM");
const MNAM: FourCC = FourCC::new(b"MNAM");
const MODL: FourCC = FourCC::new(b"MODL");

/// A shot's action: when in the attack it plays.
pub mod action {
    /// As the attack starts.
    pub const SHOOT: u32 = 0;
    /// Once a projectile flies.
    pub const FLY: u32 = 1;
    /// Once the attack has hit.
    pub const HIT: u32 = 2;
    /// A zoom: the camera moves to it and stays.
    pub const ZOOM: u32 = 3;
}

/// What a shot is placed at and looks at (its location and target).
pub mod place {
    pub const ATTACKER: u32 = 0;
    pub const PROJECTILE: u32 = 1;
    pub const TARGET: u32 = 2;
}

/// A shot's flags (`DATA` u32 at 12, object `+0x44`).
pub mod shot_flags {
    /// The camera moves with what it's placed at (`0058ccd0`); otherwise
    /// it stays where that was when the shot began.
    pub const POSITION_FOLLOWS_LOCATION: u32 = 0x01;
    /// The camera turns with what it looks at (`0058ce30`).
    pub const ROTATION_FOLLOWS_TARGET: u32 = 0x02;
    /// Placed at the actor, not at a bone of it (`0058d4f0`).
    pub const DONT_FOLLOW_BONE: u32 = 0x04;
    /// The player's own view (`009c8600`).
    pub const FIRST_PERSON: u32 = 0x08;
    /// `009c8620`.
    pub const NO_TRACER: u32 = 0x10;
    /// `0058cac0`.
    pub const START_AT_TIME_ZERO: u32 = 0x20;
}

/// A camera path's `DATA` byte.
pub mod zoom {
    /// The camera moves to the shot (`0058b4a0`: neither of the two below).
    pub const DEFAULT: u8 = 0;
    pub const DISABLE: u8 = 0x01;
    pub const SHOT_LIST: u8 = 0x02;
    /// See [`super::CameraPaths::pick`] (`0058b470`).
    pub const NO_SHOTS: u8 = 0x80;
}

/// One camera shot (`CAMS`).
#[derive(Debug, Clone, PartialEq)]
pub struct CameraShot {
    pub form_id: FormId,
    pub editor_id: String,
    /// The animated camera, under `meshes\`.
    pub model: String,
    pub action: u32,
    pub location: u32,
    pub target: u32,
    pub flags: u32,
    /// The player's, the target's and the world's time multipliers.
    pub player_mult: f32,
    pub target_mult: f32,
    pub global_mult: f32,
    pub max_time: f32,
    pub min_time: f32,
    pub target_pct: f32,
    /// `MNAM`: an image space modifier.
    pub image_space: Option<FormId>,
}

impl CameraShot {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<CameraShot> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == CAMS)?;
        let record = rr.record().ok()?;
        // The constructor's values (`0058bf50`).
        let mut shot = CameraShot {
            form_id: rr.form_id,
            editor_id: record.editor_id().unwrap_or_default(),
            model: record.get(MODL).map(|s| s.zstring()).unwrap_or_default(),
            action: action::HIT,
            location: place::TARGET,
            target: place::ATTACKER,
            flags: 3,
            player_mult: 0.0,
            target_mult: 0.0,
            global_mult: 1.0,
            max_time: 0.0,
            min_time: 0.0,
            target_pct: 0.0,
            image_space: record
                .get(MNAM)
                .filter(|s| s.data.len() >= 4)
                .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
                .filter(|f| f.0 != 0),
        };
        if let Some(d) = record.get(esm::sig::DATA).map(|s| s.data.as_slice()) {
            if d.len() >= 40 {
                shot.action = le_u32(d, 0);
                shot.location = le_u32(d, 4);
                shot.target = le_u32(d, 8);
                shot.flags = le_u32(d, 12);
                shot.player_mult = le_f32(d, 16);
                shot.target_mult = le_f32(d, 20);
                shot.global_mult = le_f32(d, 24);
                shot.max_time = le_f32(d, 28);
                shot.min_time = le_f32(d, 32);
                shot.target_pct = le_f32(d, 36);
            } else if d.len() >= 32 {
                // The older 32-byte layout (`0058c090`): no shortest time
                // or share; the rest in order from the world's multiplier
                // on one slot later.
                shot.action = le_u32(d, 0);
                shot.location = le_u32(d, 4);
                shot.target = le_u32(d, 8);
                shot.flags = le_u32(d, 12);
                shot.player_mult = le_f32(d, 16);
                shot.global_mult = le_f32(d, 20);
                shot.max_time = le_f32(d, 24);
                shot.min_time = le_f32(d, 28);
            }
        }
        Some(shot)
    }

    pub fn has(&self, flag: u32) -> bool {
        self.flags & flag != 0
    }
}

/// One camera path (`CPTH`).
#[derive(Debug, Clone, PartialEq)]
pub struct CameraPath {
    pub form_id: FormId,
    pub editor_id: String,
    pub conditions: Vec<Condition>,
    /// `ANAM`: its parent, and the path before it among its parent's.
    pub parent: Option<FormId>,
    pub previous: Option<FormId>,
    /// `DATA`: [`zoom`].
    pub zoom: u8,
    /// `SNAM`, in order.
    pub shots: Vec<FormId>,
    /// Form flag 0x20 (`00440d80`).
    pub deleted: bool,
    /// Its children in the game's order (see the module notes).
    pub children: Vec<FormId>,
}

impl CameraPath {
    fn parse(order: &LoadOrder, id: FormId) -> Option<CameraPath> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == CPTH)?;
        let record = rr.record().ok()?;
        let global = |d: &[u8], at: usize| {
            Some(rr.plugin.to_global(FormId(le_u32(d, at)))).filter(|f| f.object_id() != 0)
        };
        let anam = record.get(ANAM).map(|s| s.data.as_slice()).unwrap_or(&[]);
        Some(CameraPath {
            form_id: rr.form_id,
            editor_id: record.editor_id().unwrap_or_default(),
            conditions: record
                .get_all(CTDA)
                .filter_map(|s| read_condition(&rr, &s.data))
                .collect(),
            parent: (anam.len() >= 4).then(|| global(anam, 0)).flatten(),
            previous: (anam.len() >= 8).then(|| global(anam, 4)).flatten(),
            zoom: record
                .get(esm::sig::DATA)
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
            shots: record
                .get_all(SNAM)
                .filter(|s| s.data.len() >= 4)
                .filter_map(|s| global(&s.data, 0))
                .collect(),
            deleted: rr.entry.header.is_deleted(),
            children: Vec::new(),
        })
    }

    /// The zoom the path asks for (`009c7240` → `[011f21c8]`): 0 default,
    /// 1 disable, 2 shot list (`0058b4a0`, `0058b4d0`, `0058b4f0`).
    pub fn zoom_mode(&self) -> u8 {
        if self.zoom & zoom::DISABLE != 0 {
            1
        } else if self.zoom & zoom::SHOT_LIST != 0 {
            2
        } else {
            0
        }
    }
}

/// Every camera path, in the game's tree.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraPaths {
    /// The top-level list (`[011ca700]`), in order; a path put "at the
    /// end" past the list's end leaves a gap (`0061bc10` grows the list to
    /// the index given), kept here as `None`.
    pub top: Vec<Option<FormId>>,
    pub paths: HashMap<FormId, CameraPath>,
}

/// `0061bc10`: put `item` at `index`; past the end, the list grows to it
/// (the slots between left empty).
fn insert_at(list: &mut Vec<Option<FormId>>, index: usize, item: FormId) {
    if index > list.len() {
        list.resize(index, None);
        list.push(Some(item));
    } else {
        list.insert(index, Some(item));
    }
}

/// `0058aef0`'s place in a list: right after `previous` when it's there,
/// at the front with none, else (or for a deleted path) at the end + 1.
fn place_in(list: &mut Vec<Option<FormId>>, item: FormId, previous: Option<FormId>, deleted: bool) {
    let mut index: Option<usize> = Some(0);
    if let Some(p) = previous {
        index = list.iter().position(|&x| x == Some(p)).map(|i| i + 1);
    }
    let index = match index {
        Some(i) if !deleted => i,
        _ => list.len() + 1,
    };
    insert_at(list, index, item);
}

impl CameraPaths {
    /// Every `CPTH` in the load order, linked as `0058aef0` links them, in
    /// the order the records were first made (each plugin's in its file
    /// order; an override keeps the original's place).
    pub fn load(order: &LoadOrder) -> CameraPaths {
        let mut ids = Vec::new();
        let mut seen = HashSet::new();
        for p in order.plugins() {
            for &ri in p.plugin.record_indices_of_type(CPTH) {
                let id = p.record_id(p.plugin.records()[ri].header.form_id);
                if seen.insert(id) {
                    ids.push(id);
                }
            }
        }
        let paths: Vec<CameraPath> = ids
            .iter()
            .filter_map(|&id| CameraPath::parse(order, id))
            .collect();
        Self::link(paths)
    }

    /// Links paths given in the order they were made.
    pub fn link(paths: Vec<CameraPath>) -> CameraPaths {
        let mut out = CameraPaths::default();
        let order: Vec<FormId> = paths.iter().map(|p| p.form_id).collect();
        for p in paths {
            out.paths.insert(p.form_id, p);
        }
        // Children lists, with gaps, per parent.
        let mut children: HashMap<FormId, Vec<Option<FormId>>> = HashMap::new();
        for id in order {
            let (parent, previous, deleted) = {
                let p = &out.paths[&id];
                (p.parent, p.previous, p.deleted)
            };
            // A parent that can't be found: "Could not find parent path"; the
            // path goes nowhere.
            let list = match parent {
                Some(par) if out.paths.contains_key(&par) => children.entry(par).or_default(),
                Some(_) => continue,
                None => &mut out.top,
            };
            place_in(list, id, previous, deleted);
        }
        for (parent, list) in children {
            if let Some(p) = out.paths.get_mut(&parent) {
                p.children = list.into_iter().flatten().collect();
            }
        }
        out
    }

    /// The path an attack plays (`0058bb50` over the top-level list,
    /// `0058b510` for each path): a path that isn't deleted and whose
    /// conditions pass (`passes`, asked about the attacker and the target:
    /// `00680c60`) counts itself when it has shots or the flag 0x80
    /// ([`zoom::NO_SHOTS`]), then its children are tried in order and the
    /// first that gives a path wins over it; the first top-level path that
    /// gives one is the answer. Last (`0058bb50`), a path with 0x80 that
    /// has shots gives nothing.
    pub fn pick(&self, passes: &mut dyn FnMut(&CameraPath) -> bool) -> Option<FormId> {
        let mut found = None;
        for id in self.top.iter().flatten() {
            found = self.try_path(*id, passes, 0);
            if found.is_some() {
                break;
            }
        }
        let id = found?;
        let p = &self.paths[&id];
        if p.zoom & zoom::NO_SHOTS != 0 && !p.shots.is_empty() {
            return None;
        }
        Some(id)
    }

    fn try_path(
        &self,
        id: FormId,
        passes: &mut dyn FnMut(&CameraPath) -> bool,
        depth: u32,
    ) -> Option<FormId> {
        let p = self.paths.get(&id)?;
        if p.deleted || depth > 64 || !passes(p) {
            return None;
        }
        let mut found = (p.zoom & zoom::NO_SHOTS != 0 || !p.shots.is_empty()).then_some(id);
        for &c in &p.children {
            if let Some(f) = self.try_path(c, passes, depth + 1) {
                found = Some(f);
                break;
            }
        }
        found
    }
}

// ---------------------------------------------------------------------------
// Settings.

/// The settings the V.A.T.S. camera reads: `FalloutNV.esm`'s values, else
/// the exe's defaults (named by their setting objects; addresses in the
/// findings).
#[derive(Debug, Clone, PartialEq)]
pub struct CameraSettings {
    /// `fVATSCameraMaxTime` (20): the longest a shot lasts when its own
    /// longest time is under 0.001, and the longest the playback runs.
    pub max_time: f32,
    /// `fVATSCameraMinTime` (0): the shortest any shot lasts.
    pub min_time: f32,
    /// `fVATSPlaybackDelay` (exe 3, `FalloutNV.esm` 0.17).
    pub playback_delay: f32,
    /// `fVATSPlayerTimeUpdateMult` (exe 3, `FalloutNV.esm` 6).
    pub player_time_mult: f32,
    /// `fVATSTargetTimeUpdateMult` (1).
    pub target_time_mult: f32,
    /// `iVATSCameraHitDist` (100).
    pub hit_distance: f32,
    /// `fVATSCameraCutAwayDistance` (exe 25, `FalloutNV.esm` 30).
    pub cut_away_distance: f32,
    /// `fVATSCameraDollyTime` (exe 1, `FalloutNV.esm` 0.38) and
    /// `fVATSCameraDollyMin` (exe 256, `FalloutNV.esm` 1536).
    pub dolly_time: f32,
    pub dolly_min: f32,
    /// `fVATSImageSpaceTransitionTime` (0.15).
    pub image_space_transition: f32,
    /// `fVATSCamTransRBStrengthCap` (0.5).
    pub transition_blur_cap: f32,
    /// The menu's zoom: `fVATSTargetFOVMultNear` (exe 46, data 50),
    /// `…Far` (exe 30, data 34), `…FarDist` (exe 1750, data 2750),
    /// `fVATSTargetFOVMinFOV` (1.5).
    pub fov_mult_near: f32,
    pub fov_mult_far: f32,
    pub fov_mult_far_distance: f32,
    pub fov_min: f32,
    /// The menu's turns: `fVATSTargetSelectCamPanTime` (0.75),
    /// `fVATSLimbSelectCamPanTime` (0.3), `fVATSCamZoomInTime` (0.75).
    pub target_pan_time: f32,
    pub limb_pan_time: f32,
    pub zoom_in_time: f32,
    /// `fVATSMoveCameraLimbPercent` (0.1).
    pub move_camera_limb_percent: f32,
    /// `fIronSightsFOVTimeChange` (0.25): out of a zoom the field of view
    /// goes back to the default at 30 ÷ this degrees a second.
    pub fov_return_time: f32,
}

impl CameraSettings {
    pub fn load(order: &LoadOrder) -> CameraSettings {
        let g = |name: &str, exe: f32| crate::scripting::game_setting(order, name).unwrap_or(exe);
        CameraSettings {
            max_time: g("fVATSCameraMaxTime", 20.0),
            min_time: g("fVATSCameraMinTime", 0.0),
            playback_delay: g("fVATSPlaybackDelay", 3.0),
            player_time_mult: g("fVATSPlayerTimeUpdateMult", 3.0),
            target_time_mult: g("fVATSTargetTimeUpdateMult", 1.0),
            hit_distance: g("iVATSCameraHitDist", 100.0),
            cut_away_distance: g("fVATSCameraCutAwayDistance", 25.0),
            dolly_time: g("fVATSCameraDollyTime", 1.0),
            dolly_min: g("fVATSCameraDollyMin", 256.0),
            image_space_transition: g("fVATSImageSpaceTransitionTime", 0.15),
            transition_blur_cap: g("fVATSCamTransRBStrengthCap", 0.5),
            fov_mult_near: g("fVATSTargetFOVMultNear", 46.0),
            fov_mult_far: g("fVATSTargetFOVMultFar", 30.0),
            fov_mult_far_distance: g("fVATSTargetFOVMultFarDist", 1750.0),
            fov_min: g("fVATSTargetFOVMinFOV", 1.5),
            target_pan_time: g("fVATSTargetSelectCamPanTime", 0.75),
            limb_pan_time: g("fVATSLimbSelectCamPanTime", 0.3),
            zoom_in_time: g("fVATSCamZoomInTime", 0.75),
            move_camera_limb_percent: g("fVATSMoveCameraLimbPercent", 0.1),
            fov_return_time: g("fIronSightsFOVTimeChange", 0.25),
        }
    }
}

// ---------------------------------------------------------------------------
// The menu's camera: the eased turn and zoom (`009445b0` modes 1–3,
// `0095de30`, `00946020`…`00946280`).

/// A value eased from one number to another over a time, as the camera's
/// smoothers do (`00946020` builds it, `00946140` steps it, `00946190`
/// reads it, `00946280` says it's done): a table of 100 points, the first
/// half `0.02 i² / 99` of the way and the second half mirrored, so it
/// starts and ends slowly; read at entry trunc(elapsed / length × 99),
/// moved toward the next entry by elapsed / length (the game's own,
/// slightly odd, blend); the end value once the time is up.
#[derive(Debug, Clone, PartialEq)]
pub struct Ease {
    table: Vec<f32>,
    length: f32,
    left: f32,
}

impl Ease {
    /// The table's shape, 0 to 1 (`00946020`: steps start at 0.02 and grow
    /// by 0.04, over 99).
    fn shape() -> [f32; 100] {
        let mut out = [0.0f32; 100];
        let (mut s, mut d) = (0.0f32, 0.02f32);
        for i in 0..50 {
            out[i] = s / 99.0;
            out[99 - i] = (99.0 - s) / 99.0;
            s += d;
            d += 0.04;
        }
        out
    }

    pub fn new(from: f32, to: f32, seconds: f32) -> Ease {
        Ease {
            table: Ease::shape()
                .iter()
                .map(|k| from + (to - from) * k)
                .collect(),
            length: seconds,
            left: seconds,
        }
    }

    /// One already at `value`.
    pub fn at(value: f32) -> Ease {
        Ease::new(value, value, 0.0)
    }

    pub fn step(&mut self, dt: f32) {
        self.left -= dt;
    }

    pub fn done(&self) -> bool {
        self.left <= 0.0
    }

    pub fn target(&self) -> f32 {
        self.table[99]
    }

    pub fn value(&self) -> f32 {
        if self.left <= 0.0 || self.length <= 0.0 {
            return self.table[99];
        }
        let elapsed = self.length - self.left.max(0.0);
        let i = ((elapsed / self.length) * 99.0) as usize;
        if i >= 99 {
            return self.table[99];
        }
        let (a, b) = (self.table[i], self.table[i + 1]);
        a + (b - a) * (elapsed / self.length)
    }
}

/// The menu's field of view on a target (`0095de30`), a 4:3 width in
/// degrees: atan(bound radius × `fDlgFocus` / d) × (the near multiplier
/// moved toward the far one by min(d / far distance, 1)), at least
/// `fVATSTargetFOVMinFOV`. d: the camera to the middle of the target's
/// bound; `dlg_focus` the INI's `[Interface] fDlgFocus` (3.2).
pub fn target_fov(s: &CameraSettings, bound_radius: f32, distance: f32, dlg_focus: f32) -> f32 {
    let d = distance.max(1e-3);
    let t = (d / s.fov_mult_far_distance).min(1.0);
    let mult = (s.fov_mult_far - s.fov_mult_near) * t + s.fov_mult_near;
    (bound_radius * dlg_focus / d).atan() * mult
}

/// The clamps on [`target_fov`]: the world's at least the minimum, the
/// first-person view's at most `[Display] fDefaultFOV` too.
pub fn clamp_fovs(s: &CameraSettings, fov: f32, default_fov: f32) -> (f32, f32) {
    let world = fov.max(s.fov_min);
    (world, world.min(default_fov))
}

/// How long the menu takes to turn to (and zoom on) what it shows
/// (`009445b0`, `0095de30`): the target select pan time in mode 1; in
/// mode 2 the zoom-in time for a new target (the field of view too) and
/// the limb pan time for a new aim on the same one; else 0.75.
pub fn pan_time(s: &CameraSettings, mode: u8, new_target: bool) -> f32 {
    match mode {
        1 => s.target_pan_time,
        2 if new_target => s.zoom_in_time,
        2 => s.limb_pan_time,
        _ => 0.75,
    }
}

/// The field of view's easing time on a new target (`0095de30`).
pub fn fov_time(s: &CameraSettings, mode: u8) -> f32 {
    match mode {
        1 => s.target_pan_time,
        2 => s.zoom_in_time,
        _ => 0.75,
    }
}

/// How far to turn (radians) to look from `eye` at `point`: heading
/// clockwise from north and pitch up, the difference from the current
/// ones brought into (−π, π] (`009445b0`).
pub fn turn_toward(eye: [f32; 3], point: [f32; 3], heading: f32, pitch: f32) -> (f32, f32) {
    let d = [point[0] - eye[0], point[1] - eye[1], point[2] - eye[2]];
    let flat = (d[0] * d[0] + d[1] * d[1]).sqrt();
    let want_heading = d[0].atan2(d[1]);
    let want_pitch = d[2].atan2(flat);
    let wrap = |mut a: f32| {
        while a > PI {
            a -= 2.0 * PI;
        }
        while a <= -PI {
            a += 2.0 * PI;
        }
        a
    };
    (wrap(want_heading - heading), wrap(want_pitch - pitch))
}

/// Where the menu aims at a target (`009445b0`): on a new target, its head
/// node when it has a head and it's within 300 units of the player
/// (`[011a3b60]`), else the middle of all its parts' nodes; things
/// without parts their root.
pub fn aim_point(
    head: Option<[f32; 3]>,
    parts: &[[f32; 3]],
    player: [f32; 3],
    root: [f32; 3],
) -> [f32; 3] {
    if let Some(h) = head {
        let d =
            ((h[0] - player[0]).powi(2) + (h[1] - player[1]).powi(2) + (h[2] - player[2]).powi(2))
                .sqrt();
        if d < 300.0 {
            return h;
        }
    }
    if parts.is_empty() {
        return root;
    }
    let n = parts.len() as f32;
    let mut sum = [0.0f32; 3];
    for p in parts {
        for i in 0..3 {
            sum[i] += p[i];
        }
    }
    sum.map(|v| v / n)
}

/// An actor's bound as the menu's zoom and the shots' shares measure it
/// (`0095de30`, `009c9fe0` read the model's bound sphere, `NiBound`, which
/// isn't built here): taken as the middle of its base's `OBND` box, turned
/// by its heading and placed at its position, and half the box's diagonal
/// (the same radius `world::vats::bound_of` ÷ 2 gives misses). 64 units
/// up and a radius of 64 without a box.
pub fn bound_sphere(
    order: &LoadOrder,
    reference: FormId,
    position: [f32; 3],
    heading: f32,
) -> ([f32; 3], f32) {
    let base = crate::scripting::base_of(order, reference).unwrap_or(reference);
    let corners = order.get(base).and_then(|r| r.record().ok()).and_then(|r| {
        let s = r.get(FourCC::new(b"OBND")).filter(|s| s.data.len() >= 12)?;
        let v = |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
        Some(([v(0), v(1), v(2)], [v(3), v(4), v(5)]))
    });
    let Some((lo, hi)) = corners else {
        return ([position[0], position[1], position[2] + 64.0], 64.0);
    };
    let mid = [0, 1, 2].map(|i| (lo[i] + hi[i]) / 2.0);
    let m = crate::rotation::RotationConvention::DEFAULT.matrix([0.0, 0.0, heading]);
    let r = nif::math::mat_vec(&m, mid);
    let radius =
        ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() / 2.0;
    (
        [position[0] + r[0], position[1] + r[1], position[2] + r[2]],
        radius,
    )
}

/// The menu's mode from the V.A.T.S. key (`007ec810`): while the key is
/// held after opening, 1 (choosing the target); let go, 2; pressed again
/// in 2 or 3, back to 1. Playback (4) isn't changed.
pub fn menu_mode(mode: u8, pressed: bool, held: bool) -> u8 {
    if mode == 4 {
        return mode;
    }
    if !pressed || mode == 1 {
        if !held && mode == 1 {
            return 2;
        }
        mode
    } else {
        1
    }
}

/// The compass bearing from one place to another (`00772840`): degrees
/// clockwise from north, 0 to 360. The menu's targets are listed by it
/// (`007f0500`), and A and D step through that list.
pub fn bearing(from: [f32; 3], to: [f32; 3]) -> f32 {
    let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
    dx.atan2(dy).to_degrees().rem_euclid(360.0)
}

/// The next (W, the mouse wheel up) or previous (S) part whose label
/// shows, wrapping (`007f3070`).
pub fn step_part(shown: &[bool], current: usize, forward: bool) -> usize {
    let n = shown.len();
    if n == 0 || !shown.iter().any(|s| *s) {
        return current;
    }
    let mut i = current.min(n - 1);
    loop {
        i = if forward {
            (i + 1) % n
        } else {
            (i + n - 1) % n
        };
        if shown[i] {
            return i;
        }
    }
}

/// The menu's view (`009445b0` modes 1–3, `0095de30`): the player turned
/// toward the target and the view zoomed on it, each eased (see [`Ease`]).
/// Angles are radians (heading clockwise from north, pitch up); fields of
/// view degrees, 4:3 widths as the game's settings are.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuView {
    /// The target the turn was set for (`[011e0be0]`) and the one the zoom
    /// was (`[011e0d4c]`).
    turn_target: Option<u64>,
    zoom_target: Option<u64>,
    /// The aim's height last turned to (`[011e0bcc]`) and the correction
    /// for the selected part (`[011e075c]`).
    aim_z: Option<f32>,
    pub height_fix: f32,
    pub heading: Ease,
    pub pitch: Ease,
    /// The world's and the first-person view's fields of view (player
    /// `+0x670`, `+0x674`) and their eases.
    pub world_fov: f32,
    pub first_fov: f32,
    world_ease: Option<Ease>,
    first_ease: Option<Ease>,
    /// The zoom has arrived (`[011a3b31]`).
    pub zoom_done: bool,
}

/// What [`MenuView::frame`] needs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuViewInput {
    pub mode: u8,
    /// Real seconds since the last frame.
    pub dt: f32,
    /// The target (the caller's key) and where the view aims at it before
    /// the part correction ([`aim_point`]).
    pub target: Option<(u64, [f32; 3])>,
    /// The selected part's node: where it shows on the screen (0–1 down
    /// or up: the test is symmetric) and its height.
    pub part: Option<(f32, f32)>,
    pub eye: [f32; 3],
    /// The zoom on the target ([`target_fov`] then [`clamp_fovs`]): world,
    /// first person.
    pub zoom: Option<(f32, f32)>,
    /// `[Display] fDefaultWorldFOV` and `fDefault1stPersonFOV`.
    pub default_fovs: (f32, f32),
}

impl MenuView {
    /// Starting from the player's view as it is.
    pub fn new(heading: f32, pitch: f32, default_fovs: (f32, f32)) -> MenuView {
        MenuView {
            turn_target: None,
            zoom_target: None,
            aim_z: None,
            height_fix: 0.0,
            heading: Ease::at(heading),
            pitch: Ease::at(pitch),
            world_fov: default_fovs.0,
            first_fov: default_fovs.1,
            world_ease: None,
            first_ease: None,
            zoom_done: true,
        }
    }

    /// The turn and zoom have arrived (`+0xf8`): the labels may show.
    pub fn settled(&self) -> bool {
        self.heading.done() && self.pitch.done() && self.zoom_done
    }

    /// The field of view (`0095de30`): in modes 2 and 3 with a target,
    /// eased to the zoom on it (over `fVATSCamZoomInTime` in mode 2, 0.75 s
    /// in 3) whenever the target differs from the last zoomed on; in mode
    /// 4 the defaults at once; otherwise back toward the defaults at 30 ÷
    /// `fIronSightsFOVTimeChange` degrees a second.
    fn zoom(&mut self, s: &CameraSettings, i: &MenuViewInput) {
        let (dw, df) = i.default_fovs;
        match (i.mode, i.target, i.zoom) {
            (2 | 3, Some((key, _)), Some((zw, zf))) => {
                if self.zoom_target != Some(key) {
                    let time = fov_time(s, i.mode);
                    self.world_ease = Some(Ease::new(self.world_fov, zw, time));
                    self.first_ease = Some(Ease::new(self.first_fov, zf, time));
                    self.zoom_target = Some(key);
                }
                for (ease, fov) in [
                    (&mut self.world_ease, &mut self.world_fov),
                    (&mut self.first_ease, &mut self.first_fov),
                ] {
                    if let Some(e) = ease {
                        e.step(i.dt);
                        *fov = e.value();
                    }
                }
                self.zoom_done = self.world_ease.as_ref().map_or(true, Ease::done);
            }
            (4, _, _) => {
                self.zoom_target = None;
                self.world_fov = dw;
                self.first_fov = df;
                self.zoom_done = true;
            }
            _ => {
                self.zoom_target = None;
                let step = i.dt * 30.0 / s.fov_return_time;
                let toward = |v: f32, goal: f32| {
                    if v < goal {
                        (v + step).min(goal)
                    } else {
                        (v - step).max(goal)
                    }
                };
                self.world_fov = toward(self.world_fov, dw);
                // The first-person view decides whether it has arrived
                // (equal at the start of the frame, or reached in it).
                let before = self.first_fov;
                self.first_fov = toward(self.first_fov, df);
                self.zoom_done = before == df || (df - before).abs() < step;
            }
        }
    }

    /// One frame of modes 1–3 (`009445b0`): the zoom; on a new target the
    /// part correction is cleared; in mode 2, once the turn and the zoom
    /// have arrived, a selected part showing within 2 ×
    /// `fVATSMoveCameraLimbPercent` of the screen's top or bottom moves the
    /// aim to its height; the turn is eased afresh on a new target, or when
    /// the aim's height has changed and the last turn has arrived (over
    /// [`pan_time`]); the player takes the eases' values. Returns the
    /// heading and pitch to take.
    pub fn frame(&mut self, s: &CameraSettings, i: &MenuViewInput) -> (f32, f32) {
        self.zoom(s, i);
        let Some((key, base)) = i.target else {
            return (self.heading.value(), self.pitch.value());
        };
        let new = self.turn_target != Some(key);
        if new {
            self.turn_target = Some(key);
            self.height_fix = 0.0;
        }
        if i.mode == 2 && self.zoom_done && self.heading.done() && self.pitch.done() {
            if let Some((y, z)) = i.part {
                let p = s.move_camera_limb_percent * 2.0;
                if y < p || 1.0 - p < y {
                    self.height_fix = z - base[2];
                }
            }
        }
        let aim = [base[0], base[1], base[2] + self.height_fix];
        let mut turn = new;
        match self.aim_z {
            None => self.aim_z = Some(aim[2]),
            Some(z) if z != aim[2] && self.heading.done() && self.pitch.done() => {
                self.aim_z = Some(aim[2]);
                turn = true;
            }
            _ => {}
        }
        if turn {
            let (h, p) = (self.heading.value(), self.pitch.value());
            let (dh, dp) = turn_toward(i.eye, aim, h, p);
            let time = pan_time(s, i.mode, new);
            self.heading = Ease::new(h, h + dh, time);
            self.pitch = Ease::new(p, p + dp, time);
        }
        self.heading.step(i.dt);
        self.pitch.step(i.dt);
        (self.heading.value(), self.pitch.value())
    }
}

// ---------------------------------------------------------------------------
// Playback (`009c7240`, `009c95e0`, `009c9920`, `009c8c40`, `009c8950`).

/// The time multipliers while a shot plays (`00aa4db0` from `009c7240`,
/// `009c8cc0`, `009c8d60`): the world runs at the shot's world multiplier
/// (0.3 for attack kind 0x12; 0.0001 while the camera dollies); the
/// player and the target at that × the shot's own multipliers when above
/// 0, else × `fVATSPlayerTimeUpdateMult` / `fVATSTargetTimeUpdateMult`.
/// Without a shot playing the world runs at 1, the player at
/// `fVATSPlayerTimeUpdateMult` and the target at
/// `fVATSTargetTimeUpdateMult` (`009c8cc0` takes the setting when there's
/// no shot at all; with a shot that hasn't started, 1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeScale {
    pub world: f32,
    /// Relative to the world's.
    pub player: f32,
    pub target: f32,
}

/// A queued attack as playback sees it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttackNow {
    /// Which attack (to notice the next one).
    pub id: u64,
    /// [`crate::vats::kind`].
    pub kind: u8,
    /// Melee kinds (`009c9f60`: 0, 1, 2, 0x10, 0x11, 0x14, 0x15).
    pub melee: bool,
    pub has_target: bool,
    /// Shots still to fire (the attack's `+0x08`).
    pub shots_left: bool,
    /// Its action points, taken when it's done.
    pub ap: f32,
}

/// Whether an attack kind is an attack at all (`0066dde0`).
pub fn is_attack_kind(kind: u8) -> bool {
    matches!(kind, 0..=8 | 0x10 | 0x11 | 0x13 | 0x14 | 0x15)
}

/// What playback needs from the world each frame.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PlaybackInput {
    /// Real seconds since the last frame (the game's timer: the frame's
    /// milliseconds, 10–166, ÷ 1000; `00aa4ee0`).
    pub real_dt: f32,
    pub attack: Option<AttackNow>,
    /// The attack's projectile (`+0x1c`, set when the weapon fires
    /// `00523150`) and the one still flying (`+0x20`), until they hit.
    pub projectile_a: bool,
    pub projectile_b: bool,
    /// Either one within the target's bound radius + `iVATSCameraHitDist`.
    pub projectile_a_near: bool,
    pub projectile_b_near: bool,
    /// Melee: the swing has struck (see [`Playback::update_phase`]).
    pub melee_struck: bool,
    /// Melee: the attack animation is over.
    pub melee_over: bool,
}

/// What playback did this frame.
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackEvent {
    /// A new attack begins (`009c8e00`, then `009c9280`: aim at the
    /// target, a melee attack's warp).
    BeginAttack(u64),
    /// A camera shot starts (`0058c5d0`): load its model, freeze or follow
    /// its nodes; `clock` the camera clock now (the time-zero shift).
    ShotStarted {
        shot: FormId,
        index: usize,
        clock: f32,
    },
    /// The shot ends (`0058ce50`).
    ShotEnded(FormId),
    /// The view goes to first person (`00950110` with 1) or third
    /// (with 0).
    FirstPerson,
    ThirdPerson,
    /// The shot's image space modifier: at once for the first shot, faded
    /// over `fVATSImageSpaceTransitionTime` for later ones (`009c7240`
    /// with `005299a0` / `00529df0`).
    ImageSpace {
        modifier: Option<FormId>,
        fade: Option<f32>,
    },
    /// The attack is done; its action points are taken (`009c86b0`).
    AttackDone {
        id: u64,
        ap: f32,
    },
    /// The playback ran past `fVATSCameraMaxTime`: the queue is dropped
    /// (`009c8950` → `009ca410`).
    QueueCleared,
    /// V.A.T.S. ends (`009c6c30` with 0).
    Over,
}

/// The camera's dolly between positions (`0058cf60`, set up by
/// `0058c5d0`): while it runs, the world nearly stops.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dolly {
    pub active: bool,
    /// Seconds it takes, and the camera clock it started at.
    pub time: f32,
    pub start_clock: f32,
    /// How far it has got (0–1).
    pub t: f32,
}

/// Playback's state (the V.A.T.S. manager `[011f2250]` and its globals).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Playback {
    /// The path playing for the attack (`[011f21dc]`) and its zoom
    /// (`[011f21c8]`), its shots (`+0x0c`: none until a path is picked).
    pub path: Option<FormId>,
    pub zoom: u8,
    pub has_path: bool,
    pub shots: Vec<CameraShot>,
    /// The shot (`+0x10`) and whether its camera has loaded.
    pub current: Option<usize>,
    pub loaded: bool,
    /// 0 shooting, 1 a projectile flies, 2 it has hit (`[011f21b8]`).
    pub phase: u8,
    /// The shot's longest and shortest time left (`+0x14`, `+0x18`).
    pub max_left: f32,
    pub min_left: f32,
    /// The camera clock (`[011f21d4]`) and the time since the last shot
    /// without a longest time began (`[011f21d8]`).
    pub clock: f32,
    pub total: f32,
    /// The playback delay left once the queue is empty (`[011f21e0]`).
    pub delay: f32,
    /// The view should go third person when the shot plays (`[011f21d3]`).
    pub third_person_pending: bool,
    pub dolly: Dolly,
    /// The camera came within `fVATSCameraCutAwayDistance` of what it
    /// looks at: first person from then on (the shot's `+0x75`).
    pub cut_away: bool,
    /// The attack last played (`[011f2320]`).
    pub last_attack: Option<u64>,
    /// The world multiplier set last frame (`[011ac3a4]`).
    pub world_mult: f32,
    /// An image space modifier is on (`+0x28`).
    pub modifier_on: bool,
}

impl Playback {
    pub fn new() -> Playback {
        Playback {
            world_mult: 1.0,
            ..Playback::default()
        }
    }

    pub fn shot(&self) -> Option<&CameraShot> {
        self.current.and_then(|i| self.shots.get(i))
    }

    /// Whether the shot is playing (`0058d630`): not a zoom, and its camera
    /// loaded (a shot naming no model counts at once).
    pub fn active(&self) -> bool {
        self.shot()
            .is_some_and(|s| s.action != action::ZOOM && (s.model.is_empty() || self.loaded))
    }

    fn end_shot(&mut self, events: &mut Vec<PlaybackEvent>) {
        if let Some(s) = self.shot() {
            events.push(PlaybackEvent::ShotEnded(s.form_id));
        }
        self.loaded = false;
        self.cut_away = false;
    }

    /// Makes `i` the shot (`009c9920`, the path's first in `009c7240`): its
    /// shortest time at least, then first person or third person pending.
    fn take_shot(&mut self, i: usize, events: &mut Vec<PlaybackEvent>) {
        self.current = Some(i);
        self.loaded = false;
        self.cut_away = false;
        let s = &self.shots[i];
        if self.min_left < s.min_time {
            self.min_left = s.min_time;
        }
        if s.has(shot_flags::FIRST_PERSON) {
            events.push(PlaybackEvent::FirstPerson);
        } else {
            self.third_person_pending = true;
        }
    }

    /// The next shot of the path (`009c9920`): false when there's none,
    /// and a shot that never started is dropped.
    fn advance(&mut self, events: &mut Vec<PlaybackEvent>) -> bool {
        let next = self
            .current
            .map(|i| i + 1)
            .filter(|&n| n < self.shots.len());
        match next {
            Some(n) => {
                if self.current.is_some() {
                    self.end_shot(events);
                }
                self.take_shot(n, events);
                true
            }
            None => {
                if self.current.is_some() && !self.active() {
                    self.end_shot(events);
                    self.current = None;
                }
                false
            }
        }
    }

    /// Steps on while the shot's action comes before the phase.
    fn skip_passed(&mut self, events: &mut Vec<PlaybackEvent>) {
        while let Some(s) = self.shot() {
            if u32::from(self.phase) <= s.action {
                break;
            }
            if !self.advance(events) {
                break;
            }
        }
    }

    /// The attack's phase (`009c95e0`). Guns: attack kind 0x12 at once 2;
    /// a projectile → 1; both gone after one flew → 2; the flying one
    /// gone → 2; both there and the shot a hit shot (or none) → 2; either
    /// within the target's bound radius + `iVATSCameraHitDist` → 2. Melee
    /// (from 0 only): 2 once the swing has struck — the game asks the
    /// attack animation's state there (`00491040`, `005f2630`, `005f2540`,
    /// `0070f490`), which isn't followed: the caller says when the blow
    /// lands — then, with the shortest time over, past shots are skipped.
    pub fn update_phase(
        &mut self,
        attack: &AttackNow,
        input: &PlaybackInput,
        events: &mut Vec<PlaybackEvent>,
    ) {
        if attack.melee {
            if self.phase != 0 {
                return;
            }
            if input.melee_struck {
                self.phase = 2;
            }
            if self.phase == 2 && self.min_left == 0.0 {
                self.skip_passed(events);
            }
            return;
        }
        if attack.kind == 0x12 {
            self.phase = 2;
            return;
        }
        let (a, b) = (input.projectile_a, input.projectile_b);
        if a && self.phase == 0 {
            self.phase = 1;
        }
        if !a && !b {
            if self.phase == 1 {
                self.phase = 2;
            }
            return;
        }
        if !a {
            self.phase = 2;
            return;
        }
        if b && self.shot().map_or(true, |s| s.action == action::HIT) {
            self.phase = 2;
            return;
        }
        if input.projectile_a_near || (b && input.projectile_b_near) {
            self.phase = 2;
        }
    }

    /// The time multipliers now (see [`TimeScale`]).
    pub fn time_scale(&self, s: &CameraSettings, attack_kind: Option<u8>) -> TimeScale {
        let shot = self.shot();
        let playing = shot.is_some() && self.active();
        let world = if !playing {
            1.0
        } else if self.dolly.active {
            0.0001
        } else if attack_kind == Some(0x12) {
            0.3
        } else {
            shot.map_or(1.0, |s| s.global_mult)
        };
        let (player, target) = match shot {
            Some(_) if !playing => (1.0, 1.0),
            Some(sh) => (
                if sh.player_mult > 0.0 {
                    sh.player_mult
                } else {
                    s.player_time_mult
                },
                if sh.target_mult > 0.0 {
                    sh.target_mult
                } else {
                    s.target_time_mult
                },
            ),
            None => (s.player_time_mult, s.target_time_mult),
        };
        TimeScale {
            world,
            player,
            target,
        }
    }

    /// One frame of playback (`009c7240`), with the queue's first attack.
    /// `pick` gives a new attack's camera path (`0058bb50`: see
    /// [`CameraPaths::pick`]) with its zoom and shots; `load` loads a shot's
    /// camera when it starts (`0058c5d0`: false when its model can't be
    /// read or its first child isn't a camera).
    pub fn frame(
        &mut self,
        s: &CameraSettings,
        input: &PlaybackInput,
        pick: &mut dyn FnMut() -> Option<(FormId, u8, Vec<CameraShot>)>,
        load: &mut dyn FnMut(&CameraShot) -> bool,
    ) -> Vec<PlaybackEvent> {
        let mut events = Vec::new();
        let dt = input.real_dt;
        let Some(attack) = input.attack else {
            self.after_queue(s, dt, &mut events);
            return events;
        };
        // The shortest time.
        if self.current.is_some() && self.active() && self.min_left > 0.0 {
            self.min_left -= dt;
            if self.min_left < 0.0 {
                self.min_left = 0.0;
                self.skip_passed(&mut events);
            }
        } else if self.current.is_some() && self.last_attack != Some(attack.id) {
            self.end_shot(&mut events);
            self.phase = 0;
            self.current = None;
            self.has_path = false;
            self.max_left = 0.0;
            self.min_left = 0.0;
        }
        // The longest time.
        let mut timed_out = false;
        if self.current.is_some() && self.max_left > 0.0 {
            self.max_left -= dt;
            if self.max_left <= 0.0 {
                timed_out = true;
                self.max_left = 0.0;
                self.min_left = 0.0;
            }
        }
        if !self.has_path || timed_out {
            let new = !self.has_path;
            if new {
                match pick() {
                    Some((path, zoom, shots)) => {
                        self.path = Some(path);
                        self.zoom = zoom;
                        self.shots = shots;
                        self.has_path = true;
                    }
                    None => {
                        self.path = None;
                        self.zoom = 0;
                        self.shots.clear();
                        self.has_path = false;
                    }
                }
            }
            if self.has_path {
                if !timed_out {
                    self.current = None;
                    if !self.shots.is_empty() {
                        self.take_shot(0, &mut events);
                    } else {
                        self.third_person_pending = true;
                    }
                } else if !self.advance(&mut events) {
                    // `009c8c40` with no projectile.
                    self.set_flying(false, &attack, input, &mut events);
                }
            }
            if new {
                events.push(PlaybackEvent::BeginAttack(attack.id));
            }
            self.last_attack = Some(attack.id);
        }
        // Starting the shot.
        if self.current.is_some() {
            if !self.active() {
                self.update_phase(&attack, input, &mut events);
                if let Some(i) = self.current {
                    let sh = self.shots[i].clone();
                    if u32::from(self.phase) >= sh.action || sh.action == action::ZOOM {
                        self.max_left = sh.max_time;
                        if self.max_left <= 0.001 {
                            self.max_left = s.max_time;
                            self.total = 0.0;
                        }
                        if self.min_left < s.min_time {
                            self.min_left = s.min_time;
                        }
                        self.loaded = load(&sh);
                        self.start_dolly(s, &sh);
                        events.push(PlaybackEvent::ShotStarted {
                            shot: sh.form_id,
                            index: i,
                            clock: self.clock,
                        });
                        events.push(PlaybackEvent::ImageSpace {
                            modifier: sh.image_space,
                            fade: self.modifier_on.then_some(s.image_space_transition),
                        });
                        self.modifier_on = true;
                        if !self.active() {
                            self.advance(&mut events);
                        }
                    }
                }
            }
            if self.dolly.active && self.dolly.t == 0.0 {
                self.dolly.start_clock = self.clock;
            }
            self.clock += dt;
            self.total += dt;
            if s.max_time < self.total {
                events.push(PlaybackEvent::QueueCleared);
                self.delay = s.playback_delay;
            }
            if self.active() && self.third_person_pending {
                events.push(PlaybackEvent::ThirdPerson);
                self.third_person_pending = false;
            }
        }
        self.world_mult = self.time_scale(s, Some(attack.kind)).world;
        // The attack itself.
        if is_attack_kind(attack.kind) && !attack.shots_left {
            let mut done = false;
            if self.current.is_none() || self.min_left == 0.0 {
                if !attack.melee {
                    done = !input.projectile_b;
                } else {
                    self.update_phase(&attack, input, &mut events);
                    done = input.melee_over;
                }
            }
            if done {
                events.push(PlaybackEvent::AttackDone {
                    id: attack.id,
                    ap: attack.ap,
                });
            }
        }
        events
    }

    /// `009c8c40`: the flying projectile changes; the phase is asked again
    /// and, with the shortest time over, past shots skipped.
    fn set_flying(
        &mut self,
        b: bool,
        attack: &AttackNow,
        input: &PlaybackInput,
        events: &mut Vec<PlaybackEvent>,
    ) {
        if self.current.is_some() {
            let mut i = *input;
            i.projectile_b = b;
            self.update_phase(attack, &i, events);
            if self.min_left == 0.0 {
                self.skip_passed(events);
            }
        }
    }

    /// `0058c5d0`: the dolly for the path's zoom: default (0) dollies over
    /// `fVATSCameraDollyTime`, "disable" (1) doesn't, "shot list" (2)
    /// dollies only to zoom shots, over their shortest time when they have
    /// one.
    fn start_dolly(&mut self, s: &CameraSettings, sh: &CameraShot) {
        self.dolly = match self.zoom {
            0 => Dolly {
                active: true,
                time: s.dolly_time,
                ..Dolly::default()
            },
            2 if sh.action == action::ZOOM => Dolly {
                active: true,
                time: if sh.min_time > 0.0 {
                    sh.min_time
                } else {
                    s.dolly_time
                },
                ..Dolly::default()
            },
            1 => Dolly::default(),
            _ => self.dolly,
        };
        self.dolly.start_clock = 0.0;
        self.dolly.t = 0.0;
    }

    /// The queue is empty (`009c7240`'s end): the playback delay starts;
    /// while it runs (counted in the world's time) the camera keeps playing;
    /// then V.A.T.S. ends.
    fn after_queue(&mut self, s: &CameraSettings, dt: f32, events: &mut Vec<PlaybackEvent>) {
        if self.delay == 0.0 {
            self.delay = s.playback_delay;
            return;
        }
        self.delay -= dt * self.world_mult;
        if self.delay > 0.0 {
            self.clock += dt;
            self.world_mult = if self.dolly.active {
                0.0001
            } else if self.active() {
                self.shot().map_or(1.0, |s| s.global_mult)
            } else {
                1.0
            };
        } else {
            self.world_mult = 1.0;
            if self.current.is_some() {
                self.end_shot(events);
            }
            events.push(PlaybackEvent::Over);
            self.delay = 0.0;
        }
    }
}

// ---------------------------------------------------------------------------
// Where the shot's camera is (`0058cf60`, `0094ae40`).

/// Which node a shot is placed at or looks at (`009c7240`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotNode {
    AttackerRoot,
    /// The attacker's node a shot following a bone uses (its 3D's
    /// `0x190` lookup of `00acbb70`'s name; not traced).
    AttackerBone,
    ProjectileA,
    ProjectileB,
    TargetRoot,
    /// The node of the targeted body part (`BPND` node).
    TargetPart,
}

/// What can be pointed at this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NodesThere {
    pub projectile_a: bool,
    pub projectile_b: bool,
    pub target_root: bool,
    pub target_part: bool,
}

/// The node a shot is placed at (`009c7240`: its location) and looks at
/// (its target) when it starts, with the game's fallbacks: the attacker's
/// bone when the shot follows a bone, else its root; a projectile's (A,
/// else B); the target's part, else its root, else the attacker's. A shot
/// placed at a projectile when none flies gets no location (the game then
/// sets the attacker as its *target* by mistake, `0058ccf0`, so a shot
/// also looking at a projectile looks at the attacker), and its camera
/// isn't moved (`0058cf60` places nothing without a location node).
pub fn shot_nodes(shot: &CameraShot, there: NodesThere) -> (Option<ShotNode>, ShotNode) {
    let attacker = if shot.has(shot_flags::DONT_FOLLOW_BONE) {
        ShotNode::AttackerRoot
    } else {
        ShotNode::AttackerBone
    };
    let projectile = if there.projectile_a {
        Some(ShotNode::ProjectileA)
    } else if there.projectile_b {
        Some(ShotNode::ProjectileB)
    } else {
        None
    };
    let target_side = if there.target_part {
        ShotNode::TargetPart
    } else if there.target_root {
        ShotNode::TargetRoot
    } else {
        attacker
    };
    let location = match shot.location {
        place::ATTACKER => Some(attacker),
        place::PROJECTILE => projectile,
        _ => Some(target_side),
    };
    let target = match shot.target {
        place::ATTACKER => attacker,
        place::PROJECTILE if shot.location == place::PROJECTILE && projectile.is_none() => attacker,
        place::PROJECTILE => projectile.unwrap_or(attacker),
        _ => target_side,
    };
    (location, target)
}

/// Whose heading turns a shot's camera path (`0058cf60`): a shot following
/// its location turns by the located-at actor's heading when it follows a
/// bone and is located at the target, else by the attacker's; one not
/// following turns by the located-at actor's heading as it was when the
/// shot began (`0058cbc0`). Only the heading: actors' rotations are taken
/// about z alone (`0056fa00`).
pub fn path_heading(
    shot: &CameraShot,
    attacker: f32,
    located_at: f32,
    located_at_start: f32,
) -> f32 {
    if shot.has(shot_flags::POSITION_FOLLOWS_LOCATION) {
        if !shot.has(shot_flags::DONT_FOLLOW_BONE) && shot.location == place::TARGET {
            located_at
        } else {
            attacker
        }
    } else {
        located_at_start
    }
}

/// The camera's position (`0058cf60`): its model's keyed translation at
/// the camera clock, turned by `heading` (clockwise from north, as
/// references turn) and set at the location's position.
pub fn shot_position(offset: [f32; 3], heading: f32, location: [f32; 3]) -> [f32; 3] {
    let m = crate::rotation::RotationConvention::DEFAULT.matrix([0.0, 0.0, heading]);
    let r = nif::math::mat_vec(&m, offset);
    [r[0] + location[0], r[1] + location[1], r[2] + location[2]]
}

/// What the camera looks at (`0058d510`, `009c9fe0`): with a share above
/// 0, that many hundredths of the way from the attacker's bound's middle
/// to the target's; else the target node.
pub fn look_point(
    shot: &CameraShot,
    target_node: [f32; 3],
    attacker_middle: [f32; 3],
    target_middle: [f32; 3],
) -> [f32; 3] {
    if shot.target_pct > 0.0 {
        let k = shot.target_pct / 100.0;
        [0, 1, 2].map(|i| attacker_middle[i] + (target_middle[i] - attacker_middle[i]) * k)
    } else {
        target_node
    }
}

/// The camera's turn (`0094ae40`, `00a701b0`): looking at `point` from
/// `eye` with the world's up, as an object-style frame (columns right,
/// forward, up), then × the camera model's own X·Y·Z angles when it has
/// rotation keys. Returns (forward, up).
pub fn shot_orientation(
    eye: [f32; 3],
    point: [f32; 3],
    angles: Option<[f32; 3]>,
) -> ([f32; 3], [f32; 3]) {
    let unit = |v: [f32; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if l < 1e-6 {
            [0.0; 3]
        } else {
            v.map(|x| x / l)
        }
    };
    let cross = |a: [f32; 3], b: [f32; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let back = unit([eye[0] - point[0], eye[1] - point[1], eye[2] - point[2]]);
    let right = unit(cross([0.0, 0.0, 1.0], back));
    let up = unit(cross(back, right));
    let forward = back.map(|x| -x);
    // Rows of the object-style frame (columns right, forward, up).
    let frame = [
        [right[0], forward[0], up[0]],
        [right[1], forward[1], up[1]],
        [right[2], forward[2], up[2]],
    ];
    let m = match angles {
        Some(a) => nif::math::mat_mul(
            &frame,
            &crate::rotation::RotationConvention::DEFAULT.matrix(a),
        ),
        None => frame,
    };
    ([m[0][1], m[1][1], m[2][1]], [m[0][2], m[1][2], m[2][2]])
}

/// The shot's frustum on a screen of this shape (`0058cf60`): left and
/// right × (width / height) ÷ (4/3), top and bottom as made.
pub fn shot_frustum(f: nif::camera::Frustum, aspect: f32) -> nif::camera::Frustum {
    let k = aspect / (4.0 / 3.0);
    nif::camera::Frustum {
        left: f.left * k,
        right: f.right * k,
        ..f
    }
}

/// The dolly's move (`0058cf60`): with the dolly on, when the camera's new
/// place is farther than `fVATSCameraDollyMin` from where it was and the
/// shot isn't a shoot shot, it slides there over the dolly time (`t` =
/// (clock − start) / time; at 1 it's done). Returns the position and
/// whether the dolly goes on.
pub fn dolly_position(
    s: &CameraSettings,
    dolly: &mut Dolly,
    shot_action: u32,
    from: [f32; 3],
    to: [f32; 3],
    clock: f32,
) -> [f32; 3] {
    if !dolly.active {
        return to;
    }
    let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if len <= s.dolly_min || shot_action == action::SHOOT {
        *dolly = Dolly::default();
        return to;
    }
    dolly.t = (clock - dolly.start_clock) * (1.0 / dolly.time);
    if dolly.t < 1.0 {
        [0, 1, 2].map(|i| from[i] + d[i] * dolly.t)
    } else {
        *dolly = Dolly::default();
        to
    }
}

/// The radial blur while the camera dollies (`009c7240`): t (or 1 − t
/// past half way) × `fVATSCamTransRBStrengthCap`.
pub fn dolly_blur(s: &CameraSettings, dolly: &Dolly) -> f32 {
    let t = if dolly.t > 0.5 {
        1.0 - dolly.t
    } else {
        dolly.t
    };
    t * s.transition_blur_cap
}

// ---------------------------------------------------------------------------
// The camera kept out of walls (`0094a0c0`, which `0058cf60` calls every
// frame a shot plays, with the location node's position as the pivot and
// the turned offset added to it as the place wanted).

/// The chase camera's distance from its pivot, which the game shares
/// between its third-person camera and V.A.T.S.'s shots (`[011e0768]`,
/// `[011e07c2]`, `[011e07c3]`). Read from `0094a0c0`:
/// - a sphere `fCameraCasterSize` (10) wide is cast from the pivot to the
///   place wanted; where it touches something the distance becomes
///   |touched point − pivot| − half the sphere, and when that took more
///   than 2 units off the camera may come nearer than usual;
/// - the frame after a shot begins (`[011e07c3]`, set by `0058c5d0`,
///   `0058cbc0` and `0058ccf0`) the distance is taken at once; on the
///   frames after, it comes in at once but goes out at most
///   `fChase3rdPersonZUnitsPerSecond` (exe 300, `FalloutNV.esm` 800) a
///   second of the world's time, and is kept between `fVanityModeWheelMin`
///   (30; not when the cast took more than 2 off) and `fChaseCameraMax`
///   (120; `fVanityModeWheelMax` 600 in the vanity camera, which V.A.T.S.
///   never has);
/// - in playback (mode 4) never nearer than `fZoom3rdPersonSnapDist`
///   (50), whatever the cast found;
/// - the camera stands that far from the pivot along the line to the
///   place wanted.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ChaseDistance {
    /// How far from the pivot the camera stands (`[011e0768]`).
    pub distance: f32,
    /// The next placing takes the cast's distance at once (`[011e07c3]`).
    pub snap: bool,
    /// The last cast took more than 2 units off (`[011e07c2]`).
    pub took_off: bool,
}

/// What [`ChaseDistance::place`] reads from the settings.
#[derive(Debug, Clone, PartialEq)]
pub struct ChaseSettings {
    /// `fCameraCasterSize:HAVOK` (the INI, 10).
    pub caster_size: f32,
    /// `fChase3rdPersonZUnitsPerSecond` (exe 300, `FalloutNV.esm` 800).
    pub out_speed: f32,
    /// `fVanityModeWheelMin` (30) and `fChaseCameraMax` (120).
    pub min_distance: f32,
    pub max_distance: f32,
    /// `fZoom3rdPersonSnapDist:General` (the INI; the exe's 50).
    pub playback_min_distance: f32,
}

impl ChaseSettings {
    /// The game settings' values (`FalloutNV.esm`, else the exe's); the INI
    /// ones at their exe defaults until [`ChaseSettings::read_ini`].
    pub fn load(order: &LoadOrder) -> ChaseSettings {
        let g = |name: &str, exe: f32| crate::scripting::game_setting(order, name).unwrap_or(exe);
        ChaseSettings {
            caster_size: 10.0,
            out_speed: g("fChase3rdPersonZUnitsPerSecond", 300.0),
            min_distance: g("fVanityModeWheelMin", 30.0),
            max_distance: g("fChaseCameraMax", 120.0),
            playback_min_distance: 50.0,
        }
    }

    /// The INI's values, where it has them (`get(section, key)`).
    pub fn read_ini(&mut self, get: impl Fn(&str, &str) -> Option<f32>) {
        if let Some(v) = get("Havok", "fCameraCasterSize") {
            self.caster_size = v;
        }
        if let Some(v) = get("General", "fZoom3rdPersonSnapDist") {
            self.playback_min_distance = v;
        }
    }
}

/// The cast took off more than this (`01011590`): the camera may come
/// nearer than `fVanityModeWheelMin`.
const TOOK_OFF: f32 = 2.0;

impl ChaseDistance {
    /// A shot begins (`0058c5d0`): the next placing snaps.
    pub fn shot_begins(&mut self) {
        self.snap = true;
    }

    /// Where the camera goes this frame: `pivot` the location node,
    /// `wanted` the place the shot's model asks for, `touched` where a
    /// sphere `caster_size` wide cast from the pivot toward it first
    /// touches something (`None`: nothing in the way), `dt` the frame's
    /// world seconds, `playback` V.A.T.S. mode 4.
    pub fn place(
        &mut self,
        s: &ChaseSettings,
        pivot: [f32; 3],
        wanted: [f32; 3],
        touched: Option<[f32; 3]>,
        dt: f32,
        playback: bool,
    ) -> [f32; 3] {
        let d = [0, 1, 2].map(|i| wanted[i] - pivot[i]);
        let full = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        // `00457910`: a direction too short to unitize is zeroed.
        let unit = if full > 1e-6 {
            d.map(|x| x / full)
        } else {
            [0.0; 3]
        };
        let mut want = full;
        self.took_off = false;
        if let Some(p) = touched {
            let h = [0, 1, 2].map(|i| p[i] - pivot[i]);
            let h = (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt();
            if full - h > TOOK_OFF {
                self.took_off = true;
            }
            want = h - 0.5 * s.caster_size;
        }
        if self.snap {
            self.distance = want;
            self.snap = false;
        } else {
            self.distance = if self.distance < want {
                (self.distance + s.out_speed * dt).min(want)
            } else {
                want
            };
            if !self.took_off && self.distance < s.min_distance {
                self.distance = s.min_distance;
            }
            if self.distance > s.max_distance {
                self.distance = s.max_distance;
            }
        }
        if playback && self.distance < s.playback_min_distance {
            self.distance = s.playback_min_distance;
        }
        [0, 1, 2].map(|i| pivot[i] + unit[i] * self.distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(id: u32, parent: Option<u32>, previous: Option<u32>, shots: &[u32]) -> CameraPath {
        CameraPath {
            form_id: FormId(id),
            editor_id: format!("P{id}"),
            conditions: Vec::new(),
            parent: parent.map(FormId),
            previous: previous.map(FormId),
            zoom: 0,
            shots: shots.iter().map(|&s| FormId(s)).collect(),
            deleted: false,
            children: Vec::new(),
        }
    }

    #[test]
    fn paths_go_after_the_one_before_them_or_to_the_front() {
        // Made in the order 3, 1, 2: 1 has no "before" (front), 2 comes
        // after 1, 3 after 2 but 2 isn't there yet when 3 is made (end).
        let paths = CameraPaths::link(vec![
            path(3, None, Some(2), &[30]),
            path(1, None, None, &[10]),
            path(2, None, Some(1), &[20]),
        ]);
        let top: Vec<u32> = paths.top.iter().flatten().map(|f| f.0).collect();
        // 3 went to index 1 (past the empty list's end), then 1 to the
        // front, then 2 after 1.
        assert_eq!(top, vec![1, 2, 3]);
        // The gap 3's placing left is kept.
        assert_eq!(paths.top.len(), 4);
    }

    #[test]
    fn children_win_over_their_parent_and_the_first_top_level_path_wins() {
        let mut parent = path(1, None, None, &[10]);
        parent.zoom = 0;
        let paths = CameraPaths::link(vec![
            parent,
            path(2, Some(1), None, &[20]),
            path(3, Some(1), Some(2), &[30]),
            path(4, None, Some(1), &[40]),
        ]);
        assert_eq!(paths.paths[&FormId(1)].children, vec![FormId(2), FormId(3)]);
        // Everything passes: the first child.
        assert_eq!(paths.pick(&mut |_| true), Some(FormId(2)));
        // The first child fails: the second.
        assert_eq!(paths.pick(&mut |p| p.form_id != FormId(2)), Some(FormId(3)));
        // Both children fail: the parent itself (it has shots).
        assert_eq!(
            paths.pick(&mut |p| !matches!(p.form_id.0, 2 | 3)),
            Some(FormId(1))
        );
        // The parent fails: its children aren't tried; the next top-level.
        assert_eq!(paths.pick(&mut |p| p.form_id != FormId(1)), Some(FormId(4)));
    }

    #[test]
    fn paths_without_shots_count_only_with_the_flag() {
        let mut empty = path(1, None, None, &[]);
        let paths = CameraPaths::link(vec![empty.clone(), path(2, None, Some(1), &[20])]);
        // No shots, no flag: it doesn't count; the next path does.
        assert_eq!(paths.pick(&mut |_| true), Some(FormId(2)));
        empty.zoom = zoom::NO_SHOTS | zoom::DISABLE;
        let paths = CameraPaths::link(vec![empty, path(2, None, Some(1), &[20])]);
        assert_eq!(paths.pick(&mut |_| true), Some(FormId(1)));
        assert_eq!(paths.paths[&FormId(1)].zoom_mode(), 1);
        // The flag with shots: nothing at all.
        let mut odd = path(1, None, None, &[10]);
        odd.zoom = zoom::NO_SHOTS;
        let paths = CameraPaths::link(vec![odd]);
        assert_eq!(paths.pick(&mut |_| true), None);
    }

    fn settings() -> CameraSettings {
        // `FalloutNV.esm`'s values where it sets them.
        CameraSettings {
            max_time: 20.0,
            min_time: 0.0,
            playback_delay: 0.17,
            player_time_mult: 6.0,
            target_time_mult: 1.0,
            hit_distance: 100.0,
            cut_away_distance: 30.0,
            dolly_time: 0.38,
            dolly_min: 1536.0,
            image_space_transition: 0.15,
            transition_blur_cap: 0.5,
            fov_mult_near: 50.0,
            fov_mult_far: 34.0,
            fov_mult_far_distance: 2750.0,
            fov_min: 1.5,
            target_pan_time: 0.75,
            limb_pan_time: 0.3,
            zoom_in_time: 0.75,
            move_camera_limb_percent: 0.1,
            fov_return_time: 0.25,
        }
    }

    /// `SimpleFrontHit01` as `FalloutNV.esm` has it.
    fn front_hit() -> CameraShot {
        CameraShot {
            form_id: FormId(0x614FA),
            editor_id: "SimpleFrontHit01".into(),
            model: "VATSCameras\\SimpleFrontHit01.NIF".into(),
            action: action::HIT,
            location: place::TARGET,
            target: place::TARGET,
            flags: 7,
            player_mult: 4.0,
            target_mult: 3.0,
            global_mult: 0.25,
            max_time: 5.0,
            min_time: 1.5,
            target_pct: 0.0,
            image_space: Some(FormId(0x28FA1)),
        }
    }

    fn pistol(id: u64, shots_left: bool) -> AttackNow {
        AttackNow {
            id,
            kind: crate::vats::kind::PISTOL,
            melee: false,
            has_target: true,
            shots_left,
            ap: 17.0,
        }
    }

    #[test]
    fn eases_start_and_end_slowly_and_finish_on_time() {
        let mut e = Ease::new(10.0, 20.0, 1.0);
        assert_eq!(e.value(), 10.0);
        e.step(0.1);
        let early = e.value() - 10.0;
        e.step(0.4);
        let middle = e.value();
        assert!(early < 0.5, "{early}");
        assert!((middle - 15.0).abs() < 0.6, "{middle}");
        e.step(0.5);
        assert!(e.done());
        assert_eq!(e.value(), 20.0);
        // The table's halves meet.
        let shape = Ease::shape();
        assert!((shape[49] - 48.02 / 99.0).abs() < 1e-5);
        assert!((shape[50] - (99.0 - 48.02) / 99.0).abs() < 1e-5);
    }

    #[test]
    fn the_menu_zooms_by_the_targets_size_and_distance() {
        let s = settings();
        // 500 units off, bound radius 64, fDlgFocus 3.2: atan(0.4096) ×
        // (50 − 16 × 500/2750).
        let fov = target_fov(&s, 64.0, 500.0, 3.2);
        let want = 0.4096f32.atan() * (50.0 - 16.0 * 500.0 / 2750.0);
        assert!((fov - want).abs() < 1e-4, "{fov}");
        // Past the far distance the far multiplier.
        let far = target_fov(&s, 64.0, 5000.0, 3.2);
        assert!((far - (64.0 * 3.2f32 / 5000.0).atan() * 34.0).abs() < 1e-5);
        // Clamps: at least 1.5; the first-person view at most fDefaultFOV.
        assert_eq!(clamp_fovs(&s, 0.5, 75.0), (1.5, 1.5));
        assert_eq!(clamp_fovs(&s, 80.0, 75.0), (80.0, 75.0));
        assert_eq!(pan_time(&s, 2, false), 0.3);
        assert_eq!(pan_time(&s, 2, true), 0.75);
        assert_eq!(fov_time(&s, 3), 0.75);
    }

    #[test]
    fn the_menu_turns_by_the_short_way_and_aims_at_the_middle() {
        // Facing north, a point to the east: a quarter turn clockwise.
        let (h, p) = turn_toward([0.0; 3], [100.0, 0.0, 0.0], 0.0, 0.0);
        assert!((h - PI / 2.0).abs() < 1e-5 && p.abs() < 1e-6);
        // Facing just west of south, a point just east of south: a small
        // turn, not most of a circle.
        let (h, _) = turn_toward([0.0; 3], [1.0, -100.0, 0.0], -3.1, 0.0);
        assert!(h.abs() < 0.1, "{h}");
        let parts = [[0.0, 0.0, 0.0], [10.0, 0.0, 60.0], [20.0, 0.0, 120.0]];
        // The head near: the head.
        assert_eq!(
            aim_point(
                Some([20.0, 0.0, 120.0]),
                &parts,
                [0.0, 200.0, 120.0],
                [0.0; 3]
            ),
            [20.0, 0.0, 120.0]
        );
        // Far: the middle of the parts.
        assert_eq!(
            aim_point(
                Some([20.0, 0.0, 120.0]),
                &parts,
                [0.0, 900.0, 120.0],
                [0.0; 3]
            ),
            [10.0, 0.0, 60.0]
        );
    }

    #[test]
    fn the_v_key_holds_target_choosing_and_lets_go_to_zoom_in() {
        // Opened with V down: 1 while held, 2 once let go.
        assert_eq!(menu_mode(1, false, true), 1);
        assert_eq!(menu_mode(1, false, false), 2);
        // V again in 2 or 3: back to 1; nothing pressed: unchanged.
        assert_eq!(menu_mode(2, true, true), 1);
        assert_eq!(menu_mode(3, true, true), 1);
        assert_eq!(menu_mode(3, false, false), 3);
        assert_eq!(menu_mode(4, true, true), 4);
    }

    #[test]
    fn targets_are_listed_by_bearing_and_parts_step_past_hidden_labels() {
        assert!((bearing([0.0; 3], [0.0, 10.0, 0.0]) - 0.0).abs() < 1e-4);
        assert!((bearing([0.0; 3], [10.0, 0.0, 0.0]) - 90.0).abs() < 1e-4);
        assert!((bearing([0.0; 3], [-10.0, 0.0, 0.0]) - 270.0).abs() < 1e-4);
        let shown = [true, false, true, true];
        assert_eq!(step_part(&shown, 0, true), 2);
        assert_eq!(step_part(&shown, 3, true), 0);
        assert_eq!(step_part(&shown, 0, false), 3);
        assert_eq!(step_part(&shown, 2, false), 0);
    }

    #[test]
    fn the_menu_view_turns_and_zooms_then_settles() {
        let s = settings();
        let mut v = MenuView::new(0.0, 0.0, (75.0, 55.0));
        let mut i = MenuViewInput {
            mode: 1,
            dt: 0.05,
            target: Some((7, [100.0, 0.0, 0.0])),
            part: None,
            eye: [0.0; 3],
            zoom: Some((20.0, 20.0)),
            default_fovs: (75.0, 55.0),
        };
        // Mode 1: the turn eases over the target select pan time; no zoom.
        v.frame(&s, &i);
        assert_eq!(v.heading.target(), PI / 2.0);
        assert_eq!(v.world_fov, 75.0);
        for _ in 0..14 {
            v.frame(&s, &i);
        }
        assert!(v.heading.done(), "0.75 s");
        // Mode 2: the zoom eases over the zoom-in time; settled at its end.
        i.mode = 2;
        v.frame(&s, &i);
        assert!(!v.settled());
        for _ in 0..15 {
            v.frame(&s, &i);
        }
        assert!(v.settled());
        assert_eq!((v.world_fov, v.first_fov), (20.0, 20.0));
        // The selected part near the bottom of the screen: the aim moves
        // to its height, turned to over the limb pan time.
        i.part = Some((0.1, -50.0));
        v.frame(&s, &i);
        assert_eq!(v.height_fix, -50.0);
        assert!(!v.settled());
        assert!(v.pitch.target() < 0.0);
        // Back in mode 1 the view widens at 30 ÷ 0.25 = 120° a second.
        i.mode = 1;
        i.part = None;
        let before = v.world_fov;
        v.frame(&s, &i);
        assert!((v.world_fov - (before + 6.0)).abs() < 1e-4);
        // A new target clears the correction.
        i.target = Some((8, [0.0, 100.0, 0.0]));
        v.frame(&s, &i);
        assert_eq!(v.height_fix, 0.0);
    }

    /// Runs one frame with the path holding `shots`.
    fn step(pb: &mut Playback, input: PlaybackInput, shots: &[CameraShot]) -> Vec<PlaybackEvent> {
        let s = settings();
        let list = shots.to_vec();
        pb.frame(
            &s,
            &input,
            &mut || Some((FormId(0x11E670), 1, list.clone())),
            &mut |_| true,
        )
    }

    #[test]
    fn a_pistol_shot_plays_the_hit_camera_from_the_moment_it_fires() {
        let s = settings();
        let shots = [front_hit()];
        let mut pb = Playback::new();
        let dt = 0.016;
        let mut input = PlaybackInput {
            real_dt: dt,
            attack: Some(pistol(1, true)),
            ..PlaybackInput::default()
        };
        // First frame: the path is picked, the attack begins; the hit shot
        // waits for its phase; first person, the player × 6.
        let ev = step(&mut pb, input, &shots);
        assert!(ev.contains(&PlaybackEvent::BeginAttack(1)));
        assert_eq!(pb.current, Some(0));
        assert!(!pb.active());
        assert_eq!(pb.min_left, 1.5);
        let ts = pb.time_scale(&s, Some(3));
        assert_eq!((ts.world, ts.player, ts.target), (1.0, 1.0, 1.0));
        // The weapon fires: both projectiles there, the shot is a hit shot,
        // so the phase goes straight to 2 and the shot starts.
        input.projectile_a = true;
        input.projectile_b = true;
        input.attack = Some(pistol(1, false));
        let ev = step(&mut pb, input, &shots);
        assert_eq!(pb.phase, 2);
        assert!(ev
            .iter()
            .any(|e| matches!(e, PlaybackEvent::ShotStarted { index: 0, .. })));
        assert!(ev.contains(&PlaybackEvent::ImageSpace {
            modifier: Some(FormId(0x28FA1)),
            fade: None
        }));
        assert!(ev.contains(&PlaybackEvent::ThirdPerson));
        assert_eq!(pb.max_left, 5.0);
        let ts = pb.time_scale(&s, Some(3));
        assert_eq!((ts.world, ts.player, ts.target), (0.25, 4.0, 3.0));
        // The bullet lands; the attack isn't done until the shot's shortest
        // time (1.5 real seconds) is over.
        input.projectile_a = false;
        input.projectile_b = false;
        let mut t = 0.0;
        let mut done_at = None;
        while t < 3.0 {
            let ev = step(&mut pb, input, &shots);
            t += dt;
            if ev
                .iter()
                .any(|e| matches!(e, PlaybackEvent::AttackDone { id: 1, ap } if *ap == 17.0))
            {
                done_at = Some(t);
                break;
            }
        }
        let done_at = done_at.expect("the attack ends");
        assert!((done_at - 1.5).abs() < 0.05, "{done_at}");
        // The queue is empty: the delay (0.17 of the world's time, at 0.25)
        // keeps the camera on the shot, then V.A.T.S. ends.
        input.attack = None;
        let mut t = 0.0;
        let mut over_at = None;
        while t < 2.0 {
            let ev = step(&mut pb, input, &shots);
            if ev.contains(&PlaybackEvent::Over) {
                over_at = Some(t);
                break;
            }
            t += dt;
        }
        let over_at = over_at.expect("V.A.T.S. ends");
        assert!((over_at - 0.68).abs() < 0.05, "{over_at}");
    }

    #[test]
    fn a_new_attack_starts_over_and_long_playbacks_are_cut() {
        let s = settings();
        let shots = [front_hit()];
        let mut pb = Playback::new();
        let mut input = PlaybackInput {
            real_dt: 0.1,
            attack: Some(pistol(1, false)),
            projectile_a: true,
            projectile_b: true,
            ..PlaybackInput::default()
        };
        step(&mut pb, input, &shots);
        assert!(pb.active());
        // Its shortest time over and a second attack up: the shot ends and
        // a path is picked again.
        pb.min_left = 0.0;
        input.attack = Some(pistol(2, true));
        input.projectile_a = false;
        input.projectile_b = false;
        let ev = step(&mut pb, input, &shots);
        assert!(ev.contains(&PlaybackEvent::ShotEnded(FormId(0x614FA))));
        assert!(ev.contains(&PlaybackEvent::BeginAttack(2)));
        assert_eq!(pb.phase, 0);
        // Without any shot, the player runs at fVATSPlayerTimeUpdateMult.
        let mut idle = Playback::new();
        assert_eq!(idle.time_scale(&s, None).player, 6.0);
        // Past fVATSCameraMaxTime the queue is dropped.
        idle.total = 19.95;
        idle.current = Some(0);
        idle.shots = shots.to_vec();
        idle.has_path = true;
        idle.loaded = true;
        idle.last_attack = Some(3);
        idle.min_left = 1.0;
        let ev = idle.frame(
            &s,
            &PlaybackInput {
                real_dt: 0.1,
                attack: Some(pistol(3, true)),
                ..PlaybackInput::default()
            },
            &mut || None,
            &mut |_| true,
        );
        assert!(ev.contains(&PlaybackEvent::QueueCleared));
    }

    #[test]
    fn shots_go_where_their_records_say() {
        let mut shot = front_hit();
        let all = NodesThere {
            projectile_a: true,
            projectile_b: true,
            target_root: true,
            target_part: true,
        };
        assert_eq!(
            shot_nodes(&shot, all),
            (Some(ShotNode::TargetPart), ShotNode::TargetPart)
        );
        shot.location = place::PROJECTILE;
        shot.target = place::PROJECTILE;
        assert_eq!(
            shot_nodes(&shot, NodesThere::default()),
            (None, ShotNode::AttackerRoot)
        );
        shot.flags = 3;
        shot.location = place::ATTACKER;
        shot.target = place::TARGET;
        assert_eq!(
            shot_nodes(&shot, NodesThere::default()),
            (Some(ShotNode::AttackerBone), ShotNode::AttackerBone)
        );
        // SimpleFrontHit01 follows its location, not a bone: the attacker's
        // heading turns its path.
        let shot = front_hit();
        assert_eq!(path_heading(&shot, 1.0, 2.0, 3.0), 1.0);
        let mut bone = front_hit();
        bone.flags = 3;
        assert_eq!(path_heading(&bone, 1.0, 2.0, 3.0), 2.0);
        bone.flags = 2;
        assert_eq!(path_heading(&bone, 1.0, 2.0, 3.0), 3.0);
    }

    #[test]
    fn the_camera_is_turned_by_the_heading_and_looks_at_its_target() {
        // 100 units ahead of an actor facing east is 100 east.
        let p = shot_position([0.0, 100.0, 50.0], PI / 2.0, [1000.0, 0.0, 0.0]);
        assert!((p[0] - 1100.0).abs() < 1e-3 && p[1].abs() < 1e-3 && (p[2] - 50.0).abs() < 1e-6);
        let mut shot = front_hit();
        assert_eq!(look_point(&shot, [5.0; 3], [0.0; 3], [100.0; 3]), [5.0; 3]);
        shot.target_pct = 25.0;
        assert_eq!(look_point(&shot, [5.0; 3], [0.0; 3], [100.0; 3]), [25.0; 3]);
        // Looking north from the origin: forward +y, up +z.
        let (f, u) = shot_orientation([0.0; 3], [0.0, 10.0, 0.0], None);
        assert!((f[1] - 1.0).abs() < 1e-6 && (u[2] - 1.0).abs() < 1e-6);
        // The model's own turn (a quarter turn about z, clockwise) then
        // looks east.
        let (f, _) = shot_orientation([0.0; 3], [0.0, 10.0, 0.0], Some([0.0, 0.0, PI / 2.0]));
        assert!((f[0] - 1.0).abs() < 1e-5, "{f:?}");
        // A wider screen widens the frustum across.
        let fr = shot_frustum(
            nif::camera::Frustum {
                left: -0.7,
                right: 0.7,
                top: 0.525,
                bottom: -0.525,
                near: 1.0,
                far: 5000.0,
                ortho: false,
            },
            16.0 / 9.0,
        );
        assert!((fr.right - 0.7 * (16.0 / 9.0) / (4.0 / 3.0)).abs() < 1e-6);
        assert_eq!(fr.top, 0.525);
    }

    #[test]
    fn the_dolly_slides_only_over_long_moves() {
        let s = settings();
        let mut d = Dolly {
            active: true,
            time: 0.38,
            start_clock: 1.0,
            t: 0.0,
        };
        let p = dolly_position(&s, &mut d, action::HIT, [0.0; 3], [2000.0, 0.0, 0.0], 1.19);
        assert!((p[0] - 1000.0).abs() < 1.0, "{p:?}");
        assert!(d.active);
        assert!((dolly_blur(&s, &d) - 0.25).abs() < 0.01);
        // A short move cuts.
        let mut d2 = Dolly {
            active: true,
            time: 0.38,
            start_clock: 1.0,
            t: 0.0,
        };
        assert_eq!(
            dolly_position(&s, &mut d2, action::HIT, [0.0; 3], [100.0, 0.0, 0.0], 1.1),
            [100.0, 0.0, 0.0]
        );
        assert!(!d2.active);
    }

    #[test]
    fn deleted_paths_are_never_picked() {
        let mut gone = path(1, None, None, &[10]);
        gone.deleted = true;
        let paths = CameraPaths::link(vec![gone, path(2, None, None, &[20])]);
        assert_eq!(paths.pick(&mut |_| true), Some(FormId(2)));
    }

    fn chase() -> ChaseSettings {
        ChaseSettings {
            caster_size: 10.0,
            out_speed: 300.0,
            min_distance: 30.0,
            max_distance: 120.0,
            playback_min_distance: 50.0,
        }
    }

    #[test]
    fn the_camera_is_kept_out_of_walls() {
        // The pivot at the origin, the shot wanting the camera 175 units
        // south; a wall 100 south: the sphere touches it at 89.3 (its
        // centre 10 + a 0.7 shell short), so the camera stands at the
        // touched point less half the sphere: 94.3.
        let s = chase();
        let mut c = ChaseDistance::default();
        c.shot_begins();
        let wanted = [0.0, -175.0, 0.0];
        let touched = Some([0.0, -99.3, 0.0]);
        let at = c.place(&s, [0.0; 3], wanted, touched, 0.016, true);
        assert!((at[1] + 94.3).abs() < 1e-3, "{at:?}");
        assert!(c.took_off && !c.snap);
        // Nothing in the way the next frame: it goes back out, but only
        // 300 a second, and never past 120 from the pivot.
        let at = c.place(&s, [0.0; 3], wanted, None, 0.1, true);
        assert!((at[1] + 120.0).abs() < 1e-3, "{at:?}");
        c.distance = 60.0;
        let at = c.place(&s, [0.0; 3], wanted, None, 0.1, true);
        assert!((at[1] + 90.0).abs() < 1e-3, "{at:?}");
        // In at once.
        let at = c.place(&s, [0.0; 3], wanted, Some([0.0, -70.0, 0.0]), 0.1, true);
        assert!((at[1] + 65.0).abs() < 1e-3, "{at:?}");
    }

    #[test]
    fn the_first_frame_of_a_shot_snaps_and_playback_keeps_fifty_away() {
        let s = chase();
        let mut c = ChaseDistance::default();
        c.shot_begins();
        // Snapping takes the full 175 at once, past the 120 the later
        // frames keep to.
        let at = c.place(&s, [10.0, 0.0, 0.0], [185.0, 0.0, 0.0], None, 0.016, true);
        assert!((at[0] - 185.0).abs() < 1e-3, "{at:?}");
        // A wall right by the pivot: in playback the camera still stands
        // 50 away (the cast's 15 isn't kept).
        let mut c = ChaseDistance::default();
        c.shot_begins();
        let at = c.place(
            &s,
            [0.0; 3],
            [175.0, 0.0, 0.0],
            Some([20.0, 0.0, 0.0]),
            0.016,
            true,
        );
        assert!((at[0] - 50.0).abs() < 1e-3, "{at:?}");
        // Outside playback that minimum doesn't apply, but the 30 does
        // once the cast stops taking more than 2 off.
        let mut c = ChaseDistance::default();
        c.shot_begins();
        let at = c.place(
            &s,
            [0.0; 3],
            [175.0, 0.0, 0.0],
            Some([20.0, 0.0, 0.0]),
            0.016,
            false,
        );
        assert!((at[0] - 15.0).abs() < 1e-3, "{at:?}");
        let at = c.place(
            &s,
            [0.0; 3],
            [16.0, 0.0, 0.0],
            Some([20.0, 0.0, 0.0]),
            0.016,
            false,
        );
        assert!((at[0] - 30.0).abs() < 1e-3, "{at:?}");
        // Without the snap the distance grows from where it was, 300 a
        // second.
        let mut c = ChaseDistance::default();
        let at = c.place(&s, [0.0; 3], [175.0, 0.0, 0.0], None, 0.016, false);
        assert!((at[0] - 30.0).abs() < 1e-3, "{at:?}");
    }
}
