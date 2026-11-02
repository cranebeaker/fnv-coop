//! Weapon fire effects: the firing sound, the muzzle flash and the tracer
//! roll, as the game's weapon fire (`00523150`) and the projectile's launch
//! (`009bda10`) make them (FalloutNV.exe 1.4.0.525; private notes in
//! `nv-re\work\b5`). Where shots land (impact decals and particles) is
//! `impacts` and the later B6 task.
//!
//! **The firing sound** (`0083ac30`, called by `00523150` with the weapon,
//! the shooter, its fire node and whether it is the player; guns only, not
//! thrown weapons, which take the other branch): the weapon's sounds
//! (`TESObjectWEAP` `+0x21c` / `+0x220` / `+0x224`: `SNAM` shoot 3D, `SNAM`
//! shoot distant, `XNAM` shoot 2D; with a silencer-type mod fitted, effect
//! 11 or 16 (`004bda70`), the mod's `+0x240`… set instead, which is read for
//! the player only). Nothing plays without a 3D sound. The player with a 2D
//! sound hears that alone (flags 0x20181), whatever the view; everyone else
//! (and the player without one) the 3D sound and the distant one (0x20182).
//! Each plays only when the listener is within the sound's largest distance
//! (`SNDD` byte 1 × 100) × a multiplier along each axis (`0082eca0`); the
//! interior multipliers apply when the player's cell is an interior (cell
//! flag 0x01) that doesn't behave like an exterior (0x80):
//! `fWeaponInteriorNear/FarVolumeMod` (exe 1.0 / 0.1) and
//! `fWeaponInteriorNear/FarAttenuationMod` (3.0 / 0.75), else 1. The 3D
//! sound plays at the near volume, the distant one at the far volume; both
//! take the 3D sound's attenuation distances, its smallest × 5 and its
//! largest × 100 × the near multiplier (read from the disassembly at
//! `0083b4ba`: the distant sound's own bytes are read but not used; when
//! the 3D sound didn't start both are 0, which the playing buffer takes as
//! 69.99 and 1e9, [`buffer_distances`]: the distant sound then carries with
//! no falloff). They
//! sit at the fire node (people), else the shooter, and follow it. At most
//! 20 are kept (table `011dd9e8`, [`FireSoundSlots`]).
//!
//! **Loudness** (the audio thread): a 3D sound's distance attenuation in
//! hundredths of a decibel (`00aed990`, [`distance_attenuation`]): the
//! sound's curve, five points at its smallest distance and a quarter, half
//! and three quarters of the way to its largest, linear between them, 0
//! within the smallest, silent past the largest; each curve point is
//! `SNDD` `+12` + 2i (percent) turned into −2000·log10(p ÷ 100) (0 percent:
//! 5000; `00aeff60`, set from the sound by `0082d400`). The buffer's volume
//! (`00aed660`, [`amplitude`]) is 2000·log10(volume) (truncated) less the
//! sound's static attenuation (`SNDD` i16 at 8, also from `0082d400`) and
//! the distance attenuation, between −10000 and −1, in DirectSound's
//! hundredths of a decibel. DirectSound's own falloff is off (minimum
//! distance 1e9, `00aec530`); it only places the sound left or right.
//!
//! **The muzzle flash** (`009c2ff0` from the launch `009bda10`, per
//! projectile): when the projectile has `PROJ` flag 0x08 and a muzzle
//! flash model (`NAM1`, `+0xb4`), and its shooter is there, not dead
//! (vtable +0x22c) nor knocked out (+0x230) — for actors also not in
//! animation action 9 or 15–17 (`008a8870`) — the shooter's flash
//! ([`MuzzleFlash`]; actors keep one in their process, `+0x3d4`, made again
//! for another projectile, `009bb6d0`) is lit (`009bb690`). It shows the
//! `NAM1` model at the fire node with the projectile's muzzle flash light
//! (`DATA` `+20`, form `+0x74`, `009bb7f0`) for the flash duration (`DATA`
//! `+44`, `+0x8c`); then it rests 0.1 s, during which shots light none
//! (`009bb080`).
//!
//! **Tracers** (`009b7cc0`, a missile projectile set up): a projectile is a
//! tracer when `rand % 100` < its tracer chance (`DATA` `+24`) × 100,
//! truncated (`004fdf60`). A hitscan projectile that isn't a tracer gets
//! projectile flag 0x2, and with it no model (`009bda10`): the game's
//! bullets show nothing in flight. A tracer's model fades out over the
//! projectile's fade duration (`DATA` `+48`, `009becc0`). Vanilla tracers:
//! the minigun's and the automatic rifle's projectiles only.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};

const WEAP: FourCC = FourCC::new(b"WEAP");
const PROJ: FourCC = FourCC::new(b"PROJ");
const SOUN: FourCC = FourCC::new(b"SOUN");
const CELL: FourCC = FourCC::new(b"CELL");
const SNAM: FourCC = FourCC::new(b"SNAM");
const XNAM: FourCC = FourCC::new(b"XNAM");
const TNAM: FourCC = FourCC::new(b"TNAM");
const WMS1: FourCC = FourCC::new(b"WMS1");
const WMS2: FourCC = FourCC::new(b"WMS2");
const MODL: FourCC = FourCC::new(b"MODL");
const NAM1: FourCC = FourCC::new(b"NAM1");
const SNDD: FourCC = FourCC::new(b"SNDD");
const SNDX: FourCC = FourCC::new(b"SNDX");

/// `PROJ` `DATA` flags.
pub mod flags {
    /// Hitscan: hits along its line at once.
    pub const HITSCAN: u16 = 0x0001;
    /// Lights a muzzle flash when launched (`009c2ff0`).
    pub const MUZZLE_FLASH: u16 = 0x0008;
}

/// Weapon mod effects that pick the silenced sounds (`0083ac30`).
pub const SILENCER_EFFECTS: [i32; 2] = [11, 16];

/// How many firing sounds the game keeps at once (`0083ac30`, 20 entries
/// of 0x24 bytes at `011dd9e8`).
pub const FIRE_SOUND_SLOTS: usize = 20;

/// How long a muzzle flash rests after it goes out, seconds (`009bb080`,
/// `0101e2bc`).
pub const MUZZLE_FLASH_REST: f32 = 0.1;

/// A weapon's sounds (`TESObjectWEAP` `+0x21c`…`+0x248`, in record order).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSounds {
    /// `SNAM` (first): shoot 3D (`+0x21c`).
    pub shoot_3d: Option<FormId>,
    /// `SNAM` (second): shoot distant (`+0x220`).
    pub shoot_distant: Option<FormId>,
    /// `XNAM`: shoot 2D (`+0x224`).
    pub shoot_2d: Option<FormId>,
    /// `TNAM`: a melee weapon's swing that meets no one (`+0x22c`,
    /// `00899200`); a gun's dry fire.
    pub swing: Option<FormId>,
    /// `WMS1` (first, second) and `WMS2`: the silenced set (`+0x240`,
    /// `+0x244`, `+0x248`).
    pub silenced_3d: Option<FormId>,
    pub silenced_distant: Option<FormId>,
    pub silenced_2d: Option<FormId>,
}

impl WeaponSounds {
    pub fn load(order: &LoadOrder, weapon: FormId) -> Option<WeaponSounds> {
        let rr = order.get(weapon).filter(|r| r.entry.header.kind == WEAP)?;
        let record = rr.record().ok()?;
        let all = |kind: FourCC| -> Vec<Option<FormId>> {
            record
                .get_all(kind)
                .filter(|s| s.data.len() >= 4)
                .map(|s| Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0)))).filter(|f| f.0 != 0))
                .collect()
        };
        let nth = |kind: FourCC, i: usize| all(kind).get(i).copied().flatten();
        Some(WeaponSounds {
            shoot_3d: nth(SNAM, 0),
            shoot_distant: nth(SNAM, 1),
            shoot_2d: nth(XNAM, 0),
            swing: nth(TNAM, 0),
            silenced_3d: nth(WMS1, 0),
            silenced_distant: nth(WMS1, 1),
            silenced_2d: nth(WMS2, 0),
        })
    }

    /// The 3D, distant and 2D sounds, the silenced set when `silenced`
    /// (`005215a0`, `00522440`, `00522470`).
    pub fn set(&self, silenced: bool) -> [Option<FormId>; 3] {
        if silenced {
            [self.silenced_3d, self.silenced_distant, self.silenced_2d]
        } else {
            [self.shoot_3d, self.shoot_distant, self.shoot_2d]
        }
    }
}

/// A sound record's levels (`SNDD`, at the form's `+0x44`): its smallest
/// and largest distance bytes, static attenuation and curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoundLevels {
    /// `SNDD` byte 0 and 1.
    pub min_byte: u8,
    pub max_byte: u8,
    /// `SNDD` i16 at 8, hundredths of a decibel (`+0x4c`).
    pub static_attenuation: i16,
    /// `SNDD` u16 at 12…20, percent (`+0x50`); the constructor's curve when
    /// the record has none (`00aeac20`: 0, 600, 1400, 2600, 5000).
    pub curve: Option<[u16; 5]>,
}

impl SoundLevels {
    pub fn load(order: &LoadOrder, sound: FormId) -> Option<SoundLevels> {
        let rr = order.get(sound).filter(|r| r.entry.header.kind == SOUN)?;
        let record = rr.record().ok()?;
        let d = record
            .get(SNDD)
            .or_else(|| record.get(SNDX))
            .map(|s| s.data.clone())
            .unwrap_or_default();
        let byte = |i: usize| d.get(i).copied().unwrap_or(0);
        let short = |i: usize| (d.len() >= i + 2).then(|| u16::from_le_bytes([d[i], d[i + 1]]));
        let curve =
            (d.len() >= 22).then(|| [0, 1, 2, 3, 4].map(|k| short(12 + 2 * k).unwrap_or(0)));
        Some(SoundLevels {
            min_byte: byte(0),
            max_byte: byte(1),
            static_attenuation: short(8).map_or(0, |v| v as i16),
            curve,
        })
    }

    /// The distances a sound starts with (`0082d400`): the smallest byte
    /// (14 when 0) × 5 and the largest (60 when 0) × 100.
    pub fn distances(&self) -> (f32, f32) {
        let min = if self.min_byte == 0 {
            14
        } else {
            self.min_byte
        };
        let max = if self.max_byte == 0 {
            60
        } else {
            self.max_byte
        };
        (f32::from(min) * 5.0, f32::from(max) * 100.0)
    }

    /// The curve in hundredths of a decibel ([`curve_millibels`]).
    pub fn curve_mb(&self) -> [u16; 5] {
        match self.curve {
            Some(c) => c.map(curve_millibels),
            None => DEFAULT_CURVE_MB,
        }
    }
}

/// The curve a sound has before one is set (`00aeac20`).
pub const DEFAULT_CURVE_MB: [u16; 5] = [0, 600, 1400, 2600, 5000];

/// A curve point from percent to hundredths of a decibel of attenuation
/// (`00aeff60`): 5000 for 0, else −2000·log10(p ÷ 100), rounded.
pub fn curve_millibels(percent: u16) -> u16 {
    if percent == 0 {
        return 5000;
    }
    let mb = -2000.0 * (f64::from(percent) / 100.0).log10();
    crate::weapon_mods::round(mb as f32) as u16
}

/// The distances a playing 3D buffer keeps when given `min` and `max`
/// (`00aefda0`): a smallest of 0 or less becomes 69.99 (`0x428bfae1`, a
/// metre), a largest of 0 or less 1e9 (no limit).
pub fn buffer_distances(min: f32, max: f32) -> (f32, f32) {
    (
        if min <= 0.0 {
            f32::from_bits(0x428b_fae1)
        } else {
            min
        },
        if max <= 0.0 { 1e9 } else { max },
    )
}

/// A 3D sound's distance attenuation (`00aed990`), hundredths of a
/// decibel, `d` units from the listener, between `min` and `max`: 10000
/// (silent) past `max`; 0 within `min`; linear along the curve's five
/// points in between (smallest distance, a quarter, a half and three
/// quarters of the way, largest). With `min` and `max` within 10 of each
/// other, or `d` at least 1.1 × `max`, only the 0 / 10000 test is made.
pub fn distance_attenuation(d: f32, min: f32, max: f32, curve: [u16; 5]) -> u16 {
    if (max - min).abs() <= 10.0 || max * 1.1 <= d {
        return if d <= max { 0 } else { 10000 };
    }
    if d > max {
        return 10000;
    }
    let span = max - min;
    let p = [
        min,
        min + span * 0.25,
        min + span * 0.5,
        min + span * 0.75,
        max,
    ];
    // From the top segment down, as the game tests them.
    for k in (0..4).rev() {
        let (lo, hi) = (p[k], p[k + 1]);
        if d > hi || hi - lo <= 0.0 {
            continue;
        }
        if d > lo {
            let a = f32::from(curve[k]);
            let b = f32::from(curve[k + 1]);
            return crate::weapon_mods::round(((d - lo) / (hi - lo)) * (b - a) + a) as u16;
        }
    }
    0
}

/// The buffer's volume as DirectSound plays it (`00aed660`), as an
/// amplitude (1 = as recorded): 2000·log10(`volume` clamped to 1e-5…1),
/// truncated, less the static and distance attenuation, kept between
/// −10000 and −1 hundredths of a decibel. (The game's volume categories,
/// `00adac30`, aren't applied: the viewer has no volume settings.)
pub fn amplitude(volume: f32, static_attenuation: i16, distance_attenuation: u16) -> f32 {
    millibels_to_amplitude(millibels(volume, static_attenuation, distance_attenuation))
}

/// [`amplitude`] in hundredths of a decibel.
pub fn millibels(volume: f32, static_attenuation: i16, distance_attenuation: u16) -> i32 {
    let v = volume.clamp(0.0, 1.0).max(1e-5);
    let own = (f64::from(v).log10() * 2000.0) as i32;
    // The static attenuation is added in as unsigned (`MOVZX`).
    let mut mb = own - i32::from(static_attenuation as u16) - i32::from(distance_attenuation);
    if mb < -9999 {
        mb = -10000;
    }
    mb.min(-1)
}

/// Hundredths of a decibel as an amplitude.
pub fn millibels_to_amplitude(mb: i32) -> f32 {
    10f32.powf(mb as f32 / 2000.0)
}

/// Whether the listener is close enough for a sound to start (`0082eca0`):
/// within its largest distance byte × 100 × `mult` along each axis.
pub fn audible(levels: &SoundLevels, mult: f32, listener: [f32; 3], at: [f32; 3]) -> bool {
    let reach = f32::from(levels.max_byte) * 100.0 * mult;
    (0..3).all(|k| (at[k] - listener[k]).abs() <= reach)
}

/// The multipliers a weapon's firing sound takes (`0083ac30`): 1 outdoors,
/// the `fWeaponInterior…Mod` settings in an interior.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FireSoundMods {
    pub near_volume: f32,
    pub far_volume: f32,
    pub near_attenuation: f32,
    pub far_attenuation: f32,
}

impl FireSoundMods {
    pub const OUTDOORS: FireSoundMods = FireSoundMods {
        near_volume: 1.0,
        far_volume: 1.0,
        near_attenuation: 1.0,
        far_attenuation: 1.0,
    };

    /// The interior ones: the data's settings, else the exe's (1.0, 0.1,
    /// 3.0, 0.75).
    pub fn interior(order: &LoadOrder) -> FireSoundMods {
        let s = |n: &str, d: f32| crate::scripting::game_setting(order, n).unwrap_or(d);
        FireSoundMods {
            near_volume: s("fWeaponInteriorNearVolumeMod", 1.0),
            far_volume: s("fWeaponInteriorFarVolumeMod", 0.1),
            near_attenuation: s("fWeaponInteriorNearAttenuationMod", 3.0),
            far_attenuation: s("fWeaponInteriorFarAttenuationMod", 0.75),
        }
    }

    /// For the place the player is in (`00425fd0`, `00454b10`): an interior
    /// cell (flag 0x01) that doesn't behave like an exterior (0x80) takes
    /// the interior ones.
    pub fn for_cell(order: &LoadOrder, cell: Option<FormId>) -> FireSoundMods {
        match cell.and_then(|c| cell_flags(order, c)) {
            Some(f) if f & 0x01 != 0 && f & 0x80 == 0 => Self::interior(order),
            _ => Self::OUTDOORS,
        }
    }
}

/// A cell's flags (`DATA` byte 0).
pub fn cell_flags(order: &LoadOrder, cell: FormId) -> Option<u8> {
    let rr = order.get(cell).filter(|r| r.entry.header.kind == CELL)?;
    let record = rr.record().ok()?;
    record.get(esm::sig::DATA)?.data.first().copied()
}

/// One sound a shot starts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FireSound {
    pub sound: FormId,
    /// Played in 2D: no place, no distance attenuation.
    pub two_d: bool,
    pub volume: f32,
    /// The attenuation distances given (3D only): smallest, largest.
    pub distances: Option<(f32, f32)>,
}

/// What `0083ac30` starts for a shot: the near (3D or 2D) sound and the
/// distant one, each only when the listener is close enough.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FirePlan {
    pub near: Option<FireSound>,
    pub far: Option<FireSound>,
}

/// The sounds a shot plays (see the module notes): from `at` (the fire
/// node, else the shooter), heard at `listener`; `player` the shooter is
/// the player (only the player's silencer counts); `levels` reads a sound's
/// `SNDD`. `None`: nothing plays.
pub fn plan_fire_sounds(
    sounds: &WeaponSounds,
    player: bool,
    silenced: bool,
    mods: FireSoundMods,
    (listener, at): ([f32; 3], [f32; 3]),
    levels: impl Fn(FormId) -> Option<SoundLevels>,
) -> Option<FirePlan> {
    let silenced = silenced && player;
    let [three_d, distant, two_d] = sounds.set(silenced);
    three_d?;
    let use_2d = player && sounds.shoot_2d.is_some();
    let (near, far) = if use_2d {
        (two_d, None)
    } else {
        (three_d, distant)
    };
    let near_levels = near.and_then(&levels);
    let far_levels = far.and_then(&levels);
    let near_ok = near_levels.is_some_and(|l| audible(&l, mods.near_attenuation, listener, at));
    let far_ok = far_levels.is_some_and(|l| audible(&l, mods.far_attenuation, listener, at));
    if !near_ok && !far_ok {
        return None;
    }
    // The 3D sound's bytes, when it started (the distant one's aren't
    // used: `0083b4ba`).
    let distances = (!use_2d).then(|| {
        let (min, max) = near_levels
            .filter(|_| near_ok)
            .map_or((0, 0), |l| (l.min_byte, l.max_byte));
        (
            f32::from(min) * 5.0,
            f32::from(max) * mods.near_attenuation * 100.0,
        )
    });
    Some(FirePlan {
        near: near.filter(|_| near_ok).map(|sound| FireSound {
            sound,
            two_d: use_2d,
            volume: mods.near_volume,
            distances,
        }),
        far: far.filter(|_| far_ok).map(|sound| FireSound {
            sound,
            two_d: false,
            volume: mods.far_volume,
            distances,
        }),
    })
}

/// One of the 20 firing sound entries: whose, when (ms), how far from the
/// listener (squared).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FireSoundSlot {
    pub shooter: Option<FormId>,
    pub time_ms: u64,
    pub distance_sq: f32,
}

/// The firing sounds kept (`0083ac30`, table `011dd9e8`).
#[derive(Debug, Clone, PartialEq)]
pub struct FireSoundSlots {
    pub slots: [FireSoundSlot; FIRE_SOUND_SLOTS],
}

impl Default for FireSoundSlots {
    fn default() -> Self {
        FireSoundSlots {
            slots: [FireSoundSlot::default(); FIRE_SOUND_SLOTS],
        }
    }
}

impl FireSoundSlots {
    /// The entry a new firing sound goes in (`0083ac30`), or `None` (it
    /// doesn't play): scanning in order, someone other than the player stops
    /// at their own entry; the oldest entry seen (ties to the later) and,
    /// for the player, the one farthest from the listener (beyond the new
    /// sound) not the player's. The oldest is taken when its sound has
    /// ended (`busy` false; `00ad8ce0`), else the shooter's own, else (the
    /// player) the farthest.
    pub fn take(
        &mut self,
        shooter: FormId,
        player: bool,
        distance_sq: f32,
        now_ms: u64,
        busy: impl Fn(usize) -> bool,
    ) -> Option<usize> {
        let mut own = None;
        let mut oldest: Option<usize> = None;
        let mut farthest: Option<usize> = None;
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.shooter == Some(shooter) {
                own = Some(i);
                if !player {
                    break;
                }
            }
            if oldest.map_or(true, |o| slot.time_ms <= self.slots[o].time_ms) {
                oldest = Some(i);
            }
            if player && slot.shooter != Some(crate::dialogue::PLAYER_REF) {
                match farthest {
                    None => farthest = Some(i),
                    Some(f) => {
                        if self.slots[f].distance_sq < slot.distance_sq
                            && distance_sq < slot.distance_sq
                        {
                            farthest = Some(i);
                        }
                    }
                }
            }
        }
        let busy_oldest = oldest.map_or(true, &busy);
        let chosen = if busy_oldest {
            match own {
                Some(i) => {
                    let s = &mut self.slots[i];
                    s.time_ms = now_ms;
                    s.distance_sq = distance_sq;
                    return Some(i);
                }
                None => farthest,
            }
        } else {
            oldest
        }?;
        self.slots[chosen] = FireSoundSlot {
            shooter: Some(shooter),
            time_ms: now_ms,
            distance_sq,
        };
        Some(chosen)
    }
}

/// A projectile's effect fields (`PROJ`, `DATA` loaded at form `+0x60`).
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectileEffects {
    pub form_id: FormId,
    /// `DATA` u16 at 0 ([`flags`]).
    pub flags: u16,
    /// `MODL`: the projectile in flight.
    pub model: Option<String>,
    /// `DATA` at 16: its light (`+0x70`).
    pub light: Option<FormId>,
    /// `DATA` at 20: the muzzle flash's light (`+0x74`).
    pub muzzle_flash_light: Option<FormId>,
    /// `DATA` f32 at 24: the tracer chance (`+0x78`).
    pub tracer_chance: f32,
    /// `DATA` at 40: its sound in flight (`+0x88`).
    pub sound: Option<FormId>,
    /// `DATA` f32 at 44: how long the muzzle flash shows (`+0x8c`).
    pub muzzle_flash_duration: f32,
    /// `DATA` f32 at 48: how long a tracer takes to fade (`+0x90`).
    pub fade_duration: f32,
    /// `NAM1`: the muzzle flash model (`+0xb4`), relative to `meshes\`.
    pub muzzle_flash_model: Option<String>,
}

impl ProjectileEffects {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<ProjectileEffects> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == PROJ)?;
        let record = rr.record().ok()?;
        let data = record
            .get(esm::sig::DATA)
            .map(|s| s.data.as_slice())
            .unwrap_or(&[]);
        let f = |at: usize| (data.len() >= at + 4).then(|| le_f32(data, at));
        let form = |at: usize| {
            (data.len() >= at + 4)
                .then(|| rr.plugin.to_global(FormId(le_u32(data, at))))
                .filter(|f| f.0 != 0)
        };
        let path = |kind: FourCC| {
            record
                .get(kind)
                .map(|s| s.zstring())
                .filter(|p| !p.trim().is_empty())
        };
        Some(ProjectileEffects {
            form_id: id,
            flags: if data.len() >= 2 {
                u16::from_le_bytes([data[0], data[1]])
            } else {
                0
            },
            model: path(MODL),
            light: form(16),
            muzzle_flash_light: form(20),
            tracer_chance: f(24).unwrap_or(0.0),
            sound: form(40),
            muzzle_flash_duration: f(44).unwrap_or(0.0),
            fade_duration: f(48).unwrap_or(0.0),
            muzzle_flash_model: path(NAM1),
        })
    }

    pub fn hitscan(&self) -> bool {
        self.flags & flags::HITSCAN != 0
    }

    /// Whether its launch lights the shooter's muzzle flash (`009c2ff0`:
    /// flag 0x08 and a `NAM1` model).
    pub fn lights_muzzle_flash(&self) -> bool {
        self.flags & flags::MUZZLE_FLASH != 0 && self.muzzle_flash_model.is_some()
    }

    /// Whether a projectile is a tracer (`004fdf60`): `roll % 100` below
    /// the chance × 100, truncated.
    pub fn tracer(&self, roll: u32) -> bool {
        roll % 100 < (f64::from(self.tracer_chance) * 100.0) as u32
    }

    /// Whether a projectile shows its model in flight (`009b7cc0` →
    /// `009bda10`): not a hitscan one that isn't a tracer (flag 0x2).
    pub fn model_shown(&self, tracer: bool) -> bool {
        !(self.hitscan() && !tracer)
    }
}

/// Whether a shooter can light a muzzle flash now (`009c2ff0`): there, not
/// dead, not knocked out. (Actors' animation actions 9 and 15–17 also
/// prevent it; the viewer doesn't keep them.)
pub fn shooter_lights_flash(dead: bool, knocked_out: bool) -> bool {
    !dead && !knocked_out
}

/// A shooter's muzzle flash (`009bacb0`: 0x20 bytes; `+0` shown, `+1` its
/// light attached, `+2` freshly lit, `+4` the rest, `+8` the time left).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MuzzleFlash {
    pub shown: bool,
    pub light_on: bool,
    /// Lit since the last update (`+2`: the update restarts its particles
    /// and sets the light's dimmer, `009bb8a0`).
    pub fresh: bool,
    /// Counts down every update; a flash lights only at or below 0.
    pub rest: f32,
    pub left: f32,
    /// The projectile's flash duration.
    pub duration: f32,
}

impl MuzzleFlash {
    /// A new flash (`009bacb0`): hidden, no rest, the duration left.
    pub fn new(duration: f32) -> MuzzleFlash {
        MuzzleFlash {
            shown: false,
            light_on: false,
            fresh: false,
            rest: 0.0,
            left: duration,
            duration,
        }
    }

    /// A projectile launched (`009bb690`): lights when the rest is over,
    /// with the full duration.
    pub fn fire(&mut self) {
        if self.rest <= 0.0 {
            self.shown = true;
            self.fresh = true;
            self.left = self.duration;
        }
    }

    /// One update `dt` seconds on (`009bb080`): the light follows the shown
    /// state; the rest counts down; a shown flash's time runs down and, at
    /// or below 0, it goes out and rests 0.1 s.
    pub fn update(&mut self, dt: f32) {
        self.light_on = self.shown;
        self.fresh = false;
        self.rest -= dt;
        if self.shown {
            self.left -= dt;
            if self.left <= 0.0 {
                self.left = self.duration;
                self.rest = MUZZLE_FLASH_REST;
                self.shown = false;
                self.fresh = true;
            }
        }
    }
}

/// The projectile `holder`'s shots with `weapon` launch (`00525a90`): the
/// ammunition in use's own (`AMMO` `DAT2` form at 4), else the weapon's
/// (`DNAM` 36).
pub fn shot_projectile(
    order: &LoadOrder,
    state: &crate::scripting::GameState,
    holder: FormId,
    weapon: &crate::combat::Weapon,
) -> Option<FormId> {
    let from_ammo = weapon
        .ammo_in_use(order, state, holder)
        .and_then(|a| order.get(a))
        .and_then(|rr| {
            let d = rr.record().ok()?.get(FourCC::new(b"DAT2"))?.data.clone();
            (d.len() >= 8)
                .then(|| rr.plugin.to_global(FormId(le_u32(&d, 4))))
                .filter(|f| f.0 != 0)
        });
    from_ammo.or(weapon.projectile)
}

/// The node a weapon's shots leave from in its model (`00525700`):
/// `ProjectileNode`, else `##ProjectileNode` (a creature's embedded weapon
/// names its own, `005256b0`).
pub const FIRE_NODE_NAMES: [&str; 2] = ["ProjectileNode", "##ProjectileNode"];

#[cfg(test)]
mod tests {
    use super::*;
    use esm::Plugin;
    use testdata::{group, record, sub, zstr};

    fn order(groups: &[(&[u8; 4], Vec<u8>)]) -> LoadOrder {
        let mut header = 1.34f32.to_le_bytes().to_vec();
        header.extend([0; 8]);
        let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &header));
        for (kind, contents) in groups {
            bytes.extend(group(**kind, 0, contents));
        }
        LoadOrder::from_plugins(vec![(
            "FalloutNV.esm".into(),
            None,
            Plugin::from_bytes(bytes).unwrap(),
        )])
        .unwrap()
    }

    fn named(kind: &[u8; 4], id: u32, edid: &str, rest: &[u8]) -> Vec<u8> {
        let mut d = sub(b"EDID", &zstr(edid));
        d.extend(rest);
        record(kind, id, &d)
    }

    fn sndd(min: u8, max: u8, static_att: i16, curve: [u16; 5]) -> Vec<u8> {
        let mut d = vec![min, max, 0, 0];
        d.extend(0x1080u32.to_le_bytes());
        d.extend(static_att.to_le_bytes());
        d.extend([0, 0]);
        for c in curve {
            d.extend(c.to_le_bytes());
        }
        d.extend(100i16.to_le_bytes());
        d.extend([0; 12]);
        sub(b"SNDD", &d)
    }

    fn form(kind: &[u8; 4], id: u32) -> Vec<u8> {
        sub(kind, &id.to_le_bytes())
    }

    /// A pistol like the 9mm: 3D (51/24), distant (255/85), 2D (84/17)
    /// sounds, a dry fire, a silenced set; a cell and a projectile.
    fn pistol() -> LoadOrder {
        let curve = [100, 50, 20, 5, 0];
        let mut sounds = named(b"SOUN", 0x900, "Fire3D", &sndd(51, 24, 1057, curve));
        sounds.extend(named(
            b"SOUN",
            0x901,
            "Fire3DDist",
            &sndd(255, 85, 969, [100, 45, 32, 15, 0]),
        ));
        sounds.extend(named(b"SOUN", 0x902, "Fire2D", &sndd(84, 17, 1013, curve)));
        sounds.extend(named(b"SOUN", 0x903, "FireDry", &sndd(51, 24, 0, curve)));
        sounds.extend(named(b"SOUN", 0x904, "Silenced3D", &sndd(20, 10, 0, curve)));
        sounds.extend(named(b"SOUN", 0x905, "Silenced2D", &sndd(20, 10, 0, curve)));
        let mut w = form(b"SNAM", 0x900);
        w.extend(form(b"SNAM", 0x901));
        w.extend(form(b"XNAM", 0x902));
        w.extend(form(b"TNAM", 0x903));
        w.extend(form(b"WMS1", 0x904));
        w.extend(form(b"WMS1", 0x901));
        w.extend(form(b"WMS2", 0x905));
        let weapon = named(b"WEAP", 0x800, "Pistol", &w);
        let mut rifle = form(b"SNAM", 0x900);
        rifle.extend(form(b"SNAM", 0x901));
        let rifle = named(b"WEAP", 0x801, "No2D", &rifle);
        let mut data = 0x0089u16.to_le_bytes().to_vec();
        data.extend(1u16.to_le_bytes());
        data.extend(3.0f32.to_le_bytes());
        data.extend(23680.0f32.to_le_bytes());
        data.extend(10000.0f32.to_le_bytes());
        data.extend(0u32.to_le_bytes());
        data.extend(0x31DE9u32.to_le_bytes());
        data.extend(0.3f32.to_le_bytes());
        data.extend([0; 12]);
        data.extend(0x18731u32.to_le_bytes());
        data.extend(0.04f32.to_le_bytes());
        data.extend(0.5f32.to_le_bytes());
        data.extend([0; 32]);
        let mut p = sub(b"MODL", &zstr("projectiles\\9mmprojectile.NIF"));
        p.extend(sub(b"DATA", &data));
        p.extend(sub(
            b"NAM1",
            &zstr("effects\\MuzzleFlashes\\handgunmuzzleflash01.nif"),
        ));
        let projectile = named(b"PROJ", 0x700, "Bullet", &p);
        let mut cells = named(b"CELL", 0x600, "Inside", &sub(b"DATA", &[0x01]));
        cells.extend(named(b"CELL", 0x601, "SkyInside", &sub(b"DATA", &[0x81])));
        let setting = named(
            b"GMST",
            0x500,
            "fWeaponInteriorFarVolumeMod",
            &sub(b"DATA", &0.2f32.to_le_bytes()),
        );
        order(&[
            (b"GMST", setting),
            (b"SOUN", sounds),
            (b"WEAP", [weapon, rifle].concat()),
            (b"PROJ", projectile),
            (b"CELL", cells),
        ])
    }

    #[test]
    fn a_weapons_sounds_load_in_record_order() {
        let order = pistol();
        let s = WeaponSounds::load(&order, FormId(0x800)).unwrap();
        assert_eq!(s.shoot_3d, Some(FormId(0x900)));
        assert_eq!(s.shoot_distant, Some(FormId(0x901)));
        assert_eq!(s.shoot_2d, Some(FormId(0x902)));
        assert_eq!(s.swing, Some(FormId(0x903)));
        assert_eq!(
            s.set(true),
            [
                Some(FormId(0x904)),
                Some(FormId(0x901)),
                Some(FormId(0x905))
            ]
        );
        let l = SoundLevels::load(&order, FormId(0x900)).unwrap();
        assert_eq!(
            (l.min_byte, l.max_byte, l.static_attenuation),
            (51, 24, 1057)
        );
        assert_eq!(l.distances(), (255.0, 2400.0));
        // `00aeff60`: the vanilla curve is the constructor's, near enough.
        assert_eq!(l.curve_mb(), [0, 602, 1398, 2602, 5000]);
    }

    /// `0082d400`: missing bytes take 14 and 60.
    #[test]
    fn a_sound_without_distances_takes_the_defaults() {
        let l = SoundLevels {
            min_byte: 0,
            max_byte: 0,
            static_attenuation: 0,
            curve: None,
        };
        assert_eq!(l.distances(), (70.0, 6000.0));
        assert_eq!(l.curve_mb(), DEFAULT_CURVE_MB);
    }

    /// `00aed990`: the curve's five points between the distances.
    #[test]
    fn distance_attenuation_follows_the_curve() {
        let c = [0, 602, 1398, 2602, 5000];
        // 255..2400: points at 255, 791.25, 1327.5, 1863.75, 2400.
        assert_eq!(distance_attenuation(100.0, 255.0, 2400.0, c), 0);
        assert_eq!(distance_attenuation(255.0, 255.0, 2400.0, c), 0);
        assert_eq!(distance_attenuation(791.25, 255.0, 2400.0, c), 602);
        assert_eq!(distance_attenuation(1059.375, 255.0, 2400.0, c), 1000);
        assert_eq!(distance_attenuation(2400.0, 255.0, 2400.0, c), 5000);
        assert_eq!(distance_attenuation(2500.0, 255.0, 2400.0, c), 10000);
        // Past 1.1 × the largest, or distances within 10: all or nothing.
        assert_eq!(distance_attenuation(5000.0, 255.0, 2400.0, c), 10000);
        assert_eq!(distance_attenuation(5.0, 0.0, 0.0, c), 10000);
        assert_eq!(distance_attenuation(0.0, 0.0, 0.0, c), 0);
        // `00aefda0`: a buffer given 0 and 0 keeps a metre and 1e9, so a
        // distant sound 3000 units off is barely attenuated.
        let (min, max) = buffer_distances(0.0, 0.0);
        assert!((min - 69.99).abs() < 1e-3 && max == 1e9);
        assert_eq!(distance_attenuation(3000.0, min, max, c), 0);
        assert_eq!(buffer_distances(255.0, 2400.0), (255.0, 2400.0));
    }

    /// `00aed660`: volume, static and distance attenuation in hundredths of
    /// a decibel, at most −1.
    #[test]
    fn loudness_is_the_volume_less_the_attenuations() {
        assert_eq!(millibels(1.0, 0, 0), -1);
        assert_eq!(millibels(1.0, 1057, 0), -1057);
        // 0.1 as a float is a hair over a tenth: -1999.99998, truncated.
        assert_eq!(millibels(0.1, 1057, 602), -1999 - 1057 - 602);
        assert_eq!(millibels(1.0, 1057, 10000), -10000);
        assert_eq!(millibels(0.0, 0, 0), -10000);
        assert!((amplitude(1.0, 2000, 0) - 0.1).abs() < 1e-4);
    }

    /// `0083ac30`: the player with a 2D sound hears it alone; others the
    /// 3D and distant ones, with the 3D sound's distances, each when close
    /// enough by its own multiplier.
    #[test]
    fn the_shot_plans_its_sounds_as_the_game_does() {
        let order = pistol();
        let levels = |id| SoundLevels::load(&order, id);
        let s = WeaponSounds::load(&order, FormId(0x800)).unwrap();
        let here = [0.0; 3];
        let out = FireSoundMods::OUTDOORS;
        // The player: the 2D sound, no distances.
        let p = plan_fire_sounds(&s, true, false, out, (here, here), levels).unwrap();
        assert_eq!(
            p.near,
            Some(FireSound {
                sound: FormId(0x902),
                two_d: true,
                volume: 1.0,
                distances: None
            })
        );
        assert_eq!(p.far, None);
        // Silenced: the player's silenced 2D.
        let p = plan_fire_sounds(&s, true, true, out, (here, here), levels).unwrap();
        assert_eq!(p.near.unwrap().sound, FormId(0x905));
        // Someone 1000 units away: both, with the 3D sound's distances.
        let at = [1000.0, 0.0, 0.0];
        let p = plan_fire_sounds(&s, false, true, out, (here, at), levels).unwrap();
        assert_eq!(p.near.unwrap().sound, FormId(0x900));
        assert_eq!(p.near.unwrap().distances, Some((255.0, 2400.0)));
        assert_eq!(p.far.unwrap().sound, FormId(0x901));
        assert_eq!(p.far.unwrap().distances, Some((255.0, 2400.0)));
        // 3000 away: past the 3D sound's 2400, within the distant 8500:
        // the distant sound alone, with no distances (the 3D one's bytes
        // weren't read).
        let at = [3000.0, 0.0, 0.0];
        let p = plan_fire_sounds(&s, false, false, out, (here, at), levels).unwrap();
        assert_eq!(p.near, None);
        assert_eq!(p.far.unwrap().distances, Some((0.0, 0.0)));
        // Past the distant one's reach: nothing.
        assert!(
            plan_fire_sounds(&s, false, false, out, (here, [9000.0, 0.0, 0.0]), levels).is_none()
        );
        // Axis by axis: 2400 along each of two axes is still within.
        let p = plan_fire_sounds(&s, false, false, out, (here, [2400.0, 2400.0, 0.0]), levels);
        assert!(p.unwrap().near.is_some());
        // The player without a 2D sound plays the 3D set.
        let r = WeaponSounds::load(&order, FormId(0x801)).unwrap();
        let p = plan_fire_sounds(&r, true, false, out, (here, here), levels).unwrap();
        assert_eq!(p.near.unwrap().sound, FormId(0x900));
        assert!(!p.near.unwrap().two_d);
        // No 3D sound: nothing.
        let none = WeaponSounds::default();
        assert!(plan_fire_sounds(&none, false, false, out, (here, here), levels).is_none());
    }

    /// Interior multipliers: an interior cell takes them (the data's far
    /// volume 0.2 here, the exe's for the rest); one that behaves like an
    /// exterior and the outdoors don't.
    #[test]
    fn interiors_change_the_firing_sounds() {
        let order = pistol();
        let inside = FireSoundMods::for_cell(&order, Some(FormId(0x600)));
        assert_eq!(
            inside,
            FireSoundMods {
                near_volume: 1.0,
                far_volume: 0.2,
                near_attenuation: 3.0,
                far_attenuation: 0.75
            }
        );
        assert_eq!(
            FireSoundMods::for_cell(&order, Some(FormId(0x601))),
            FireSoundMods::OUTDOORS
        );
        assert_eq!(
            FireSoundMods::for_cell(&order, None),
            FireSoundMods::OUTDOORS
        );
        let levels = |id| SoundLevels::load(&order, id);
        let s = WeaponSounds::load(&order, FormId(0x800)).unwrap();
        // 5000 away indoors: the 3D sound reaches 2400 × 3 = 7200, the
        // distant one 8500 × 0.75 = 6375.
        let p = plan_fire_sounds(
            &s,
            false,
            false,
            inside,
            ([0.0; 3], [5000.0, 0.0, 0.0]),
            levels,
        )
        .unwrap();
        assert_eq!(p.near.unwrap().volume, 1.0);
        assert_eq!(p.near.unwrap().distances, Some((255.0, 7200.0)));
        assert_eq!(p.far.unwrap().volume, 0.2);
    }

    /// `0083ac30`'s table: the oldest free entry; when it's busy, the
    /// shooter's own; the player takes the farthest not theirs.
    #[test]
    fn firing_sounds_share_twenty_entries() {
        let mut slots = FireSoundSlots::default();
        let a = FormId(0x10);
        let player = crate::dialogue::PLAYER_REF;
        assert_eq!(slots.take(a, false, 1.0, 100, |_| false), Some(19));
        assert_eq!(slots.take(a, false, 1.0, 200, |_| false), Some(18));
        // All playing: someone else gets nothing until one ends ...
        let mut slots = FireSoundSlots::default();
        for (i, s) in slots.slots.iter_mut().enumerate() {
            *s = FireSoundSlot {
                shooter: Some(FormId(0x100 + i as u32)),
                time_ms: 10 + i as u64,
                distance_sq: (i * i) as f32,
            };
        }
        let b = FormId(0x10);
        assert_eq!(slots.take(b, false, 4.0, 500, |_| true), None);
        // ... or the oldest is free.
        assert_eq!(slots.take(b, false, 4.0, 500, |i| i != 0), Some(0));
        assert_eq!(slots.slots[0].shooter, Some(b));
        // Its own entry when the oldest is busy.
        assert_eq!(slots.take(b, false, 9.0, 600, |_| true), Some(0));
        assert_eq!(
            (slots.slots[0].time_ms, slots.slots[0].distance_sq),
            (600, 9.0)
        );
        // The player steals the farthest entry beyond the new sound.
        assert_eq!(slots.take(player, true, 2.0, 700, |_| true), Some(19));
        assert_eq!(slots.slots[19].shooter, Some(player));
    }

    #[test]
    fn a_projectiles_effects_and_tracer_roll() {
        let order = pistol();
        let p = ProjectileEffects::load(&order, FormId(0x700)).unwrap();
        assert!(p.hitscan() && p.lights_muzzle_flash());
        assert_eq!(p.muzzle_flash_light, Some(FormId(0x31DE9)));
        assert_eq!(p.sound, Some(FormId(0x18731)));
        assert!((p.muzzle_flash_duration - 0.04).abs() < 1e-6);
        assert_eq!(
            p.muzzle_flash_model.as_deref(),
            Some("effects\\MuzzleFlashes\\handgunmuzzleflash01.nif")
        );
        // 0.3 → 30: rolls 0–29 of every hundred.
        assert!(p.tracer(29) && !p.tracer(30) && p.tracer(129));
        // A hitscan bullet shows its model only as a tracer.
        assert!(p.model_shown(true) && !p.model_shown(false));
        let no_flash = ProjectileEffects {
            muzzle_flash_model: None,
            ..p.clone()
        };
        assert!(!no_flash.lights_muzzle_flash());
        let flagless = ProjectileEffects { flags: 0x81, ..p };
        assert!(!flagless.lights_muzzle_flash() && !flagless.model_shown(false));
    }

    /// `009bb690` / `009bb080`: shown for the duration, then a 0.1 s rest
    /// in which shots light nothing.
    #[test]
    fn a_muzzle_flash_shows_then_rests() {
        let mut f = MuzzleFlash::new(0.04);
        f.fire();
        assert!(f.shown && f.fresh);
        f.update(0.016);
        assert!(f.shown && f.light_on && !f.fresh);
        // Another shot while shown starts it over.
        f.fire();
        assert!((f.left - 0.04).abs() < 1e-6);
        f.update(0.03);
        f.update(0.016);
        assert!(!f.shown && (f.rest - 0.1).abs() < 1e-6);
        // Resting: no flash.
        f.fire();
        assert!(!f.shown);
        f.update(0.05);
        f.fire();
        assert!(!f.shown);
        f.update(0.06);
        f.fire();
        assert!(f.shown);
        assert!(shooter_lights_flash(false, false));
        assert!(!shooter_lights_flash(true, false) && !shooter_lights_flash(false, true));
    }
}
