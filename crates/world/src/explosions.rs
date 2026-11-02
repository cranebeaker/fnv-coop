//! Thrown weapons, grenades and explosions, as the game runs them (read
//! from FalloutNV.exe 1.4.0.525 with Ghidra; class names from the Xbox 360
//! prototype's symbols, marked (Xbox PDB); evidence and gaps in
//! `docs/EXPLOSIVES.md`).
//!
//! - A projectile's record (`PROJ` `DATA`, 84 bytes, loaded to the form's
//!   `+0x60` by `004fd6f0`): [`ProjectileRecord`]. Grenades and thrown
//!   weapons (weapon animation types 10–13, `00523150`) fire "lobbers"
//!   (type 2, `GrenadeProjectile` (Xbox PDB), made by `009b4020`).
//! - A lobber flies under the world's gravity (its own gravity forced to 1,
//!   `00966980`) at the projectile's speed (`009669c0`, with the thrower's
//!   multipliers: [`launch_speed`]). Its fuse is the record's timer, set at
//!   launch for anything that isn't a mine (`009bda10` → `009bd760`), and
//!   counted down each frame (`009c3190`); one without the "alt. trigger"
//!   flag goes off when it first strikes something instead. Unexploded,
//!   it's removed after `fGrenadeAgeMax` (90 s, `009b41b0`). See [`Flight`].
//! - The explosion (`EXPL` `DATA`, 52 bytes at the form's `+0x74`):
//!   [`ExplosionRecord`]. It's made where the grenade lies
//!   (`Explosion::SpawnExplosion` (Xbox PDB), `009ac9c0`), with the player's
//!   "Adjust Explosion Radius" perks ([`blast_radius`]), and hurts each
//!   person or creature whose position is within its radius and in its
//!   line of sight ([`los_clear`]) by its damage ([`base_damage`]) ×
//!   [`falloff`], through the hit path (`Explosion::ProcessTargets` (Xbox
//!   PDB), `009b00a0` → `009b5770` → `0089a760`;
//!   `world::scripting::Runner::explosion_hit`).
//! - People throw at a point worked out for the projectile's arc
//!   ([`aim_point`], `009a8bf0` → `009a8f00`), low first, high at
//!   `fGrenadeHighArcSpeedPercentage` of the speed when the low arc is
//!   blocked (`009a7670`).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::combat::Weapon;
use crate::dialogue::PLAYER_REF;
use crate::perks::{self, Tab};
use crate::scripting::{game_setting, Facts, GameState};

const PROJ: FourCC = FourCC::new(b"PROJ");
const EXPL: FourCC = FourCC::new(b"EXPL");
const MNAM: FourCC = FourCC::new(b"MNAM");
const EITM: FourCC = FourCC::new(b"EITM");

/// `PROJ` `DATA` flags (u16 at 0; xEdit's names, checked against the
/// code where noted).
pub mod proj_flags {
    pub const HITSCAN: u16 = 0x0001;
    /// It explodes (`004fd360`).
    pub const EXPLOSION: u16 = 0x0002;
    /// "Alt. trigger": it goes off by its timer (or proximity), not on
    /// impact (`00975300`, read by `009c3190`).
    pub const ALT_TRIGGER: u16 = 0x0004;
    /// "Can be disabled": E on it while armed disarms it
    /// (`BGSProjectile::Activate` (Xbox PDB), `Projectile::TurnOff`,
    /// `009c43e0`).
    pub const CAN_BE_DISABLED: u16 = 0x0020;
    /// "Can be picked up": E on it (disarmed) takes it.
    pub const CAN_BE_PICKED_UP: u16 = 0x0040;
    /// "Detonates": the code's mine test (`005de080`).
    pub const DETONATES: u16 = 0x0400;
}

/// `PROJ` `DATA` type (u16 at 2; the code tests it as `flags & 0x1f0000`).
pub mod proj_type {
    pub const MISSILE: u16 = 0x01;
    pub const LOBBER: u16 = 0x02;
    pub const BEAM: u16 = 0x04;
    pub const FLAME: u16 = 0x08;
    pub const CONTINUOUS_BEAM: u16 = 0x10;
}

/// `EXPL` `DATA` flags (u32 at 20; the form's `+0x88`, tested by
/// `00477950`).
pub mod expl_flags {
    /// The radii are in game units (else feet: × `fBSUnitsPerFoot`,
    /// `00477900`). xEdit calls it "Unknown 1".
    pub const RADIUS_IN_UNITS: u32 = 0x01;
    pub const ALWAYS_USES_WORLD_ORIENTATION: u32 = 0x02;
    pub const KNOCK_DOWN_ALWAYS: u32 = 0x04;
    pub const KNOCK_DOWN_BY_FORMULA: u32 = 0x08;
    /// No line-of-sight test (`009b1810`).
    pub const IGNORE_LOS: u32 = 0x10;
    pub const PUSH_SOURCE_ONLY: u32 = 0x20;
    pub const IGNORE_IMAGE_SPACE_SWAP: u32 = 0x40;
}

/// A projectile's record (`PROJ`): `DATA` flags u16 at 0, type u16 at 2,
/// gravity f32 at 4, speed at 8, range at 12, proximity (alt. trigger) at
/// 28, timer at 32, explosion at 36, impact force at 52, countdown sound
/// at 56, disable sound at 60, default weapon source at 64, bounciness at 80 (the form's `+0x60 …`, `+0xb0` for the last:
/// `006d2c20`); its model (`MODL`).
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectileRecord {
    pub form_id: FormId,
    pub flags: u16,
    pub kind: u16,
    pub gravity: f32,
    pub speed: f32,
    pub range: f32,
    pub proximity: f32,
    pub timer: f32,
    pub explosion: Option<FormId>,
    pub impact_force: f32,
    pub countdown_sound: Option<FormId>,
    /// The sound of disarming it (`DATA` form at 60, the form's `+0xac`).
    pub disable_sound: Option<FormId>,
    /// The weapon it comes from (`DATA` form at 64): what picking it up
    /// gives (the frag mine's `WeapMineFrag`).
    pub weapon_source: Option<FormId>,
    pub bounciness: f32,
    pub model: Option<String>,
}

impl ProjectileRecord {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<ProjectileRecord> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == PROJ)?;
        let record = rr.record().ok()?;
        let data = record.get(esm::sig::DATA)?;
        let form = |raw: u32| Some(rr.plugin.to_global(FormId(raw))).filter(|f| f.0 != 0);
        let mut p = Self::parse(id, &data.data, form)?;
        p.model = record.get(esm::sig::MODL).map(|s| s.zstring());
        Some(p)
    }

    /// From `DATA`'s bytes (at least 60; the bounciness when there are 84),
    /// form IDs mapped by `form`.
    pub fn parse(
        id: FormId,
        d: &[u8],
        form: impl Fn(u32) -> Option<FormId>,
    ) -> Option<ProjectileRecord> {
        if d.len() < 60 {
            return None;
        }
        Some(ProjectileRecord {
            form_id: id,
            flags: u16::from_le_bytes([d[0], d[1]]),
            kind: u16::from_le_bytes([d[2], d[3]]),
            gravity: le_f32(d, 4),
            speed: le_f32(d, 8),
            range: le_f32(d, 12),
            proximity: le_f32(d, 28),
            timer: le_f32(d, 32),
            explosion: form(le_u32(d, 36)),
            impact_force: le_f32(d, 52),
            countdown_sound: form(le_u32(d, 56)),
            disable_sound: (d.len() >= 64).then(|| form(le_u32(d, 60))).flatten(),
            weapon_source: (d.len() >= 68).then(|| form(le_u32(d, 64))).flatten(),
            bounciness: if d.len() >= 84 { le_f32(d, 80) } else { 0.0 },
            model: None,
        })
    }

    pub fn is_lobber(&self) -> bool {
        self.kind == proj_type::LOBBER
    }

    pub fn explodes(&self) -> bool {
        self.flags & proj_flags::EXPLOSION != 0 && self.explosion.is_some()
    }

    pub fn alt_trigger(&self) -> bool {
        self.flags & proj_flags::ALT_TRIGGER != 0
    }

    /// A mine (`009bdf80`): "detonates", or an alt. trigger with a
    /// proximity. Mines get no fuse at launch.
    pub fn is_mine(&self) -> bool {
        self.flags & proj_flags::DETONATES != 0 || (self.alt_trigger() && self.proximity > 0.0)
    }

    /// The gravity it falls with (`00966980`): 1 for lobbers, else the
    /// record's.
    pub fn fall_gravity(&self) -> f32 {
        if self.is_lobber() {
            1.0
        } else {
            self.gravity
        }
    }
}

/// An explosion's record (`EXPL`): `DATA` force f32 at 0, damage at 4,
/// radius at 8, light at 12, sound at 16, flags u32 at 20, image space
/// radius f32 at 24, impact data set at 28, second sound at 32, radiation
/// level, time and radius at 36–44; its model (`MODL`) and image space
/// modifier (`MNAM`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExplosionRecord {
    pub form_id: FormId,
    pub force: f32,
    pub damage: f32,
    pub radius: f32,
    pub light: Option<FormId>,
    pub sound: Option<FormId>,
    pub flags: u32,
    pub image_space_radius: f32,
    pub impact_set: Option<FormId>,
    pub sound2: Option<FormId>,
    pub model: Option<String>,
    pub image_space: Option<FormId>,
    /// Its object effect (`EITM`, the form's `+0x68`): the pulse grenades'
    /// and mines' `EMP`, the incendiary grenades' fire, the stun grenade's.
    pub enchantment: Option<FormId>,
}

impl ExplosionRecord {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<ExplosionRecord> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == EXPL)?;
        let record = rr.record().ok()?;
        let data = record.get(esm::sig::DATA)?;
        let form = |raw: u32| Some(rr.plugin.to_global(FormId(raw))).filter(|f| f.0 != 0);
        let mut e = Self::parse(id, &data.data, form)?;
        e.model = record.get(esm::sig::MODL).map(|s| s.zstring());
        e.image_space = record
            .get(MNAM)
            .filter(|s| s.data.len() >= 4)
            .and_then(|s| form(le_u32(&s.data, 0)));
        e.enchantment = record
            .get(EITM)
            .filter(|s| s.data.len() >= 4)
            .and_then(|s| form(le_u32(&s.data, 0)));
        Some(e)
    }

    /// From `DATA`'s bytes (at least 36), form IDs mapped by `form`.
    pub fn parse(
        id: FormId,
        d: &[u8],
        form: impl Fn(u32) -> Option<FormId>,
    ) -> Option<ExplosionRecord> {
        if d.len() < 36 {
            return None;
        }
        Some(ExplosionRecord {
            form_id: id,
            force: le_f32(d, 0),
            damage: le_f32(d, 4),
            radius: le_f32(d, 8),
            light: form(le_u32(d, 12)),
            sound: form(le_u32(d, 16)),
            flags: le_u32(d, 20),
            image_space_radius: le_f32(d, 24),
            impact_set: form(le_u32(d, 28)),
            sound2: form(le_u32(d, 32)),
            model: None,
            image_space: None,
            enchantment: None,
        })
    }

    /// Its radius in game units (`BGSExplosion::GetRadiusBSUnits` (Xbox
    /// PDB), `00477900`): as stored with flag 0x01, else × `units_per_foot`
    /// (`fBSUnitsPerFoot`, 22).
    pub fn radius_units(&self, units_per_foot: f32) -> f32 {
        if self.flags & expl_flags::RADIUS_IN_UNITS != 0 {
            self.radius
        } else {
            self.radius * units_per_foot
        }
    }
}

/// How much of an explosion reaches `distance` from it (`00647920`): 1 −
/// (distance ÷ radius)² inside the radius, else 0.
// Translated from 00647920 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn falloff(radius: f32, distance: f32) -> f32 {
    if distance <= radius {
        let r = distance / radius;
        1.0 - r * r
    } else {
        0.0
    }
}

fn setting(order: &LoadOrder, name: &str, default: f32) -> f32 {
    game_setting(order, name).unwrap_or(default)
}

/// An explosion's damage before falloff (`Explosion::GetDamage` (Xbox
/// PDB), `009b0f80`, with `006479e0`): (`fDamageGunWeapCondBase` +
/// `fDamageGunWeapCondMult`, data 0.66 + 0.34) × the record's damage; made
/// by someone (a person or creature) with a weapon, also × the base
/// `fDamageSkillBase` plus `fDamageSkillMult` × their skill in the weapon
/// ÷ 100 (0.5 + 0.5 × skill ÷ 100).
pub fn base_damage(
    order: &LoadOrder,
    state: &GameState,
    source: Option<FormId>,
    weapon: Option<&Weapon>,
    explosion: &ExplosionRecord,
) -> f32 {
    let skill = source.zip(weapon).map(|(who, w)| {
        Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(who, w.skill)
        .unwrap_or(0.0) as f32
    });
    damage_with(
        explosion.damage,
        (
            setting(order, "fDamageGunWeapCondBase", 0.66),
            setting(order, "fDamageGunWeapCondMult", 0.34),
        ),
        (
            setting(order, "fDamageSkillBase", 0.5),
            setting(order, "fDamageSkillMult", 0.5),
        ),
        skill,
    )
}

/// [`base_damage`]'s arithmetic: `cond` the two condition settings,
/// `skill_settings` base and multiplier, `skill` the maker's (`None`: not
/// made by someone with a weapon).
// Translated from 009b0f80 and 006479e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn damage_with(
    damage: f32,
    cond: (f32, f32),
    skill_settings: (f32, f32),
    skill: Option<f32>,
) -> f32 {
    let base = (cond.1 + cond.0) * damage;
    match skill {
        Some(s) => base * (s * 0.01 * skill_settings.1 + skill_settings.0),
        None => base,
    }
}

/// An explosion's radius as made (`009ac9c0`): the record's in game units
/// ([`ExplosionRecord::radius_units`]), × the player's "Adjust Explosion
/// Radius" perks (entry point 72, asked about the weapon) when the player
/// made it.
pub fn blast_radius(
    order: &LoadOrder,
    state: &GameState,
    source: Option<FormId>,
    weapon: Option<FormId>,
    explosion: &ExplosionRecord,
) -> f32 {
    let r = explosion.radius_units(setting(order, "fBSUnitsPerFoot", 22.0));
    if source != Some(PLAYER_REF) {
        return r;
    }
    let mult = perks::apply_for(
        order,
        state,
        PLAYER_REF,
        perks::entry::ADJUST_EXPLOSION_RADIUS,
        1.0,
        &[perks::weapon_tab(weapon)],
    );
    r * mult
}

/// How fast a thrower's projectile leaves (`009669c0` with the launch's
/// multipliers in `009bca60`): the record's speed (full draw: guns and
/// throws), for thrown weapons proper (animation type 13) × (1 +
/// `fThrowingStrengthPenalty` (0.05) × (Strength − the weapon's Strength
/// requirement, `DNAM` 168)), × the thrower's "Modify Throwing Velocity"
/// perks (entry point 59, asked about the weapon), × `arc` (the AI's high
/// arc, [`HIGH_ARC_SETTING`]; 1 otherwise). The player's weapon mods and
/// Turbo aren't modelled.
pub fn launch_speed(
    order: &LoadOrder,
    state: &GameState,
    thrower: FormId,
    weapon: &Weapon,
    projectile: &ProjectileRecord,
    arc: f32,
) -> f32 {
    let mut speed = projectile.speed;
    if weapon.animation == 13 {
        let strength = Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(thrower, crate::combat::av::STRENGTH)
        .unwrap_or(0.0) as f32;
        let required = strength_requirement(order, weapon.form_id).unwrap_or(0.0);
        speed *= 1.0 + setting(order, "fThrowingStrengthPenalty", 0.05) * (strength - required);
    }
    speed = perks::apply_for(
        order,
        state,
        thrower,
        perks::entry::MODIFY_THROWING_VELOCITY,
        speed,
        &[Tab::Weapon(weapon.form_id)],
    );
    speed * arc
}

/// A weapon's Strength requirement (`DNAM` u32 at 168, the form's
/// `+0x19c`).
fn strength_requirement(order: &LoadOrder, weapon: FormId) -> Option<f32> {
    let record = order.get(weapon)?.record().ok()?;
    let d = record.get(FourCC::new(b"DNAM"))?;
    (d.data.len() >= 172).then(|| le_u32(&d.data, 168) as f32)
}

/// Weapons fired as grenades, mines and throws (`00523150`: animation
/// types 10 grenade, 11 land mine, 12 mine drop, 13 thrown): they come
/// from the hand and use up the weapon itself.
pub fn is_thrown(weapon: &Weapon) -> bool {
    (10..=13).contains(&weapon.animation)
}

/// The setting the AI's high arc multiplies the speed by.
pub const HIGH_ARC_SETTING: &str = "fGrenadeHighArcSpeedPercentage";

/// The nearest surface along a segment (from, unit direction, length):
/// its distance.
pub type Cast<'a> = dyn FnMut([f32; 3], [f32; 3], f32) -> Option<f32> + 'a;

/// [`Cast`] with the surface's unit normal.
pub type SurfaceCast<'a> = dyn FnMut([f32; 3], [f32; 3], f32) -> Option<(f32, [f32; 3])> + 'a;

/// What a lobber needs from the settings each frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlightSettings {
    /// The world's gravity, units a second² (`crate::combat_ai::
    /// WORLD_GRAVITY`).
    pub gravity: f32,
    /// `fGrenadeAgeMax` (exe 90 s).
    pub age_max: f32,
    /// The contact's restitution and friction (`009be0a0`): with
    /// `fGrenadeRestitution` (data 0.01) set, the projectile's bounciness ×
    /// it (the setting alone when the bounciness is 0 or less); with
    /// `fGrenadeFriction` (data 5) set, it; unset, the model's own.
    pub restitution: f32,
    pub friction: f32,
}

impl FlightSettings {
    pub fn read(order: &LoadOrder, projectile: &ProjectileRecord) -> FlightSettings {
        let restitution = setting(order, "fGrenadeRestitution", 0.0);
        let friction = setting(order, "fGrenadeFriction", 0.0);
        // Mines live by `fMineAgeMax` (exe 0: for ever), grenades by
        // `fGrenadeAgeMax` (`009b41b0`).
        let age_max = if projectile.is_mine() {
            setting(order, "fMineAgeMax", 0.0)
        } else {
            setting(order, "fGrenadeAgeMax", 90.0)
        };
        FlightSettings::with(projectile, restitution, friction, age_max)
    }

    /// From the settings' values. The model's own restitution and friction
    /// (used when a setting is 0) aren't read: 0 then (unresolved).
    // Translated from 009be0a0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn with(
        projectile: &ProjectileRecord,
        restitution: f32,
        friction: f32,
        age_max: f32,
    ) -> FlightSettings {
        let restitution = if restitution == 0.0 {
            0.0
        } else if projectile.bounciness <= 0.0 {
            restitution
        } else {
            projectile.bounciness * restitution
        };
        FlightSettings {
            gravity: crate::combat_ai::WORLD_GRAVITY * projectile.fall_gravity(),
            age_max,
            restitution,
            friction,
        }
    }
}

/// What a frame of flight came to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlightEvent {
    Flying,
    /// It goes off here.
    Explode {
        at: [f32; 3],
    },
    /// It ran out of age without going off (`009b41b0` → `009bc8f0`).
    Expired,
}

/// A lobber in flight (`GrenadeProjectile` (Xbox PDB)).
///
/// The game moves it as a Havok rigid body with the restitution and
/// friction of [`FlightSettings`]; here it's a point under gravity that
/// strikes the collision `cast` gives, keeping −restitution of its speed
/// into the surface and losing friction × that change of speed along it
/// (Coulomb friction, as a rigid-body contact solves it; the body's own
/// shape, spin and the surface's friction aren't modelled: a stand-in for
/// Havok's solver, labelled in `docs/EXPLOSIVES.md`).
#[derive(Debug, Clone, PartialEq)]
pub struct Flight {
    pub projectile: ProjectileRecord,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub age: f32,
    /// Seconds left on the fuse (`+0xe4`); `None` for mines (never set,
    /// `f32::MAX` in the game).
    pub fuse: Option<f32>,
    /// Where it first struck something.
    pub impact: Option<[f32; 3]>,
    /// Lying still.
    pub resting: bool,
}

/// Speeds below this (units a second) after a contact leave it lying still.
const REST_SPEED: f32 = 1.0;

impl Flight {
    /// A launch from `origin` along `direction` (unit) at `speed`
    /// (`009bca60`); the fuse is the record's timer unless it's a mine
    /// (`009bda10`).
    // Translated from 009bda10 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn launch(
        projectile: ProjectileRecord,
        origin: [f32; 3],
        direction: [f32; 3],
        speed: f32,
    ) -> Flight {
        let fuse = (!projectile.is_mine()).then_some(projectile.timer);
        Flight {
            projectile,
            position: origin,
            velocity: direction.map(|d| d * speed),
            age: 0.0,
            fuse,
            impact: None,
            resting: false,
        }
    }

    /// A mine set off ([`crate::mines::check_proximity`]): its fuse starts
    /// running.
    pub fn set_off(&mut self, fuse: f32) {
        if self.fuse.is_none() {
            self.fuse = Some(fuse);
        }
    }

    /// One frame of `dt` seconds (`GrenadeProjectile::UpdateProjectile`
    /// (Xbox PDB), `009b41b0`, then `009c3190`). `cast(from, direction,
    /// length)` gives the nearest surface along a segment: its distance and
    /// unit normal.
    pub fn step(&mut self, dt: f32, s: &FlightSettings, cast: &mut SurfaceCast) -> FlightEvent {
        self.age += dt;
        // `009b41b0`: past its age it's removed.
        if s.age_max != 0.0 && self.age > s.age_max {
            return FlightEvent::Expired;
        }
        if !self.resting {
            self.fly(dt, s, cast);
        }
        self.check_explosion(dt)
    }

    fn fly(&mut self, dt: f32, s: &FlightSettings, cast: &mut SurfaceCast) {
        self.velocity[2] -= s.gravity * dt;
        let mut left = dt;
        for _ in 0..4 {
            let speed = length(self.velocity);
            let travel = speed * left;
            if travel < 1e-4 {
                break;
            }
            let dir = self.velocity.map(|v| v / speed);
            match cast(self.position, dir, travel) {
                None => {
                    self.position = add(self.position, scale(dir, travel));
                    break;
                }
                Some((d, normal)) => {
                    // Up to the surface, kept a little off it.
                    let d = (d - 0.5).max(0.0);
                    self.position = add(self.position, scale(dir, d));
                    self.impact.get_or_insert(self.position);
                    left -= d / speed;
                    let vn = dot(self.velocity, normal);
                    if vn < 0.0 {
                        let normal_part = scale(normal, vn);
                        let tangent = sub(self.velocity, normal_part);
                        let change = (1.0 + s.restitution) * -vn;
                        let t_speed = length(tangent);
                        let keep = if t_speed > 1e-6 {
                            (1.0 - s.friction * change / t_speed).max(0.0)
                        } else {
                            0.0
                        };
                        self.velocity =
                            add(scale(tangent, keep), scale(normal, -vn * s.restitution));
                    }
                    if length(self.velocity) < REST_SPEED && normal[2] > 0.7 {
                        self.velocity = [0.0; 3];
                        self.resting = true;
                        break;
                    }
                }
            }
        }
    }

    /// Whether it goes off now (`009c3190`): with the alt. trigger when its
    /// fuse runs out (a mine without a fuse waits: its proximity isn't
    /// modelled); without, when it has struck something (unless it
    /// "detonates", a mine).
    // Translated from 009c3190 (decompiled, FalloutNV.exe 1.4.0.525)
    fn check_explosion(&mut self, dt: f32) -> FlightEvent {
        if !self.projectile.explodes() {
            return FlightEvent::Flying;
        }
        let goes = if self.projectile.alt_trigger() {
            match self.fuse.as_mut() {
                None => false,
                Some(t) => {
                    *t -= dt;
                    *t <= 0.0
                }
            }
        } else {
            self.impact.is_some() && self.projectile.flags & proj_flags::DETONATES == 0
        };
        if !goes {
            return FlightEvent::Flying;
        }
        // Grenades that go off on impact explode where they struck (flag
        // 0x2000, set by `009b40d0`), the rest where they are.
        let at = if self.projectile.alt_trigger() {
            self.position
        } else {
            self.impact.unwrap_or(self.position)
        };
        FlightEvent::Explode { at }
    }
}

/// What an explosion's line of sight starts from (`Explosion::RunLOSPick`
/// (Xbox PDB), `009b1810`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosPick {
    /// The explosion's `ClosestPointNormal` (Xbox PDB, `+0xf4`,
    /// [`closest_surface`]); zero when no surface was found.
    pub normal: [f32; 3],
    /// `fExplosionLOSBufferDistance` (exe 24): the start is moved this far
    /// along the normal.
    pub buffer_distance: f32,
    /// `fExplosionLOSBuffer` (exe 6).
    pub buffer: f32,
    /// The explosion's owner (`pOwner`, Xbox PDB, `+0xc8`) has the
    /// reference flag 0x01000000 (`00452370`): on the first point nothing
    /// blocks. Projectiles' and mines' explosions have no owner
    /// (`009c3190` passes none to `009ac9c0`).
    pub owner_passes: bool,
}

/// Whether an explosion at `center` sees `point` (`Explosion::RunLOSPick`
/// (Xbox PDB), `009b1810`): the pick starts at `center` + the closest
/// surface's normal × `fExplosionLOSBufferDistance` and every surface it
/// meets on the way blocks (`cast(from, direction, length)` gives the
/// nearest one's distance), except, when that normal's z is below −0.98,
/// those nearer than `fExplosionLOSBuffer` to the start; on the first
/// point an owner with flag 0x01000000 lets everything through. For
/// people and creatures `offsets` adds the six points ±2 × their radius
/// along each axis (`0084d030`), tried in turn when the first is blocked,
/// with the same start and the normal's rule only. With the "ignore LOS"
/// flag (0x10) the pick isn't made.
// Translated from 009b1810 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn los_clear(
    center: [f32; 3],
    point: [f32; 3],
    offsets: Option<f32>,
    pick: &LosPick,
    cast: &mut Cast,
) -> bool {
    let from = add(center, scale(pick.normal, pick.buffer_distance));
    let below = pick.normal[2] < -0.98;
    let exempt = |d: f32| below && d < pick.buffer;
    if segment_clear(from, point, &|d| pick.owner_passes || exempt(d), cast) {
        return true;
    }
    let Some(r) = offsets else {
        return false;
    };
    let r2 = r + r;
    [
        [point[0], point[1], point[2] + r2],
        [point[0], point[1], point[2] - r2],
        [point[0] + r2, point[1], point[2]],
        [point[0] - r2, point[1], point[2]],
        [point[0], point[1] + r2, point[2]],
        [point[0], point[1] - r2, point[2]],
    ]
    .into_iter()
    .any(|p| segment_clear(from, p, &exempt, cast))
}

/// Every surface along the segment, nearest first (the pick's all-hits
/// collector), blocks unless `exempt` at its distance from `from`.
fn segment_clear(
    from: [f32; 3],
    to: [f32; 3],
    exempt: &dyn Fn(f32) -> bool,
    cast: &mut Cast,
) -> bool {
    let full = length(sub(to, from));
    if full < 1e-3 {
        return true;
    }
    let dir = scale(sub(to, from), 1.0 / full);
    let mut along = 0.0;
    while along < full {
        let Some(d) = cast(add(from, scale(dir, along)), dir, full - along) else {
            return true;
        };
        let at = along + d;
        if !exempt(at) {
            return false;
        }
        along = at + 0.5;
    }
    true
}

/// A surface the explosion's sphere touches (one of its phantom's closest
/// points): the body it belongs to, the point, and the unit normal from
/// the surface toward the explosion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Contact {
    pub body: u32,
    pub point: [f32; 3],
    pub normal: [f32; 3],
}

/// The explosion's `ClosestPoint` and `ClosestPointNormal` (Xbox PDB,
/// `+0xe8`/`+0xf4`), set as it finds its targets (`009ae6e0`): for each
/// contact not on an actor, a ray from the explosion along twice the way
/// to the point (or, when that is shorter than 1 unit, from 16 units back
/// along the normal, 32 along it) must meet that same body before any
/// other that isn't an actor's; the first that does gives the point and
/// the normal. `first_body(from, vector)` gives the first non-actor body
/// met along the vector.
// Translated from 009ae6e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn closest_surface(
    center: [f32; 3],
    contacts: &[Contact],
    first_body: &mut dyn FnMut([f32; 3], [f32; 3]) -> Option<u32>,
) -> Option<([f32; 3], [f32; 3])> {
    for c in contacts {
        let mut from = center;
        let mut ray = scale(sub(c.point, center), 2.0);
        if length(ray) < 1.0 {
            from = sub(from, scale(c.normal, 16.0));
            ray = scale(c.normal, 32.0);
        }
        if first_body(from, ray) == Some(c.body) {
            return Some((c.point, c.normal));
        }
    }
    None
}
/// Someone an explosion reaches.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlastTarget {
    pub reference: FormId,
    pub distance: f32,
    /// [`falloff`] at their distance.
    pub share: f32,
}

/// Who an explosion of `radius` at `center` reaches among `candidates`
/// (reference, position): those whose position is nearer than the radius
/// (the game's sphere collects bodies overlapping it, `009ae280`; whoever's
/// position is outside gets no share, `009b00a0`), nearest first.
pub fn blast_targets(
    center: [f32; 3],
    radius: f32,
    candidates: &[(FormId, [f32; 3])],
) -> Vec<BlastTarget> {
    let mut out: Vec<BlastTarget> = candidates
        .iter()
        .filter_map(|&(reference, p)| {
            let distance = length(sub(p, center));
            let share = falloff(radius, distance);
            (share > 0.0).then_some(BlastTarget {
                reference,
                distance,
                share,
            })
        })
        .collect();
    out.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    out
}

/// What a knockdown by formula reads (`00646400`): `fKnockdownBaseHealthThreshold`
/// (exe 75), `fKnockdownCurrentHealthThreshold` (data 50), `fKnockdownAgilBase`
/// (0) and `…AgilMult` (1), `fKnockdownDamageBase` (0) and `…DamageMult`
/// (0.3), `fKnockdownChance` (0.25, the most the chance can be).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KnockdownSettings {
    pub base_health_threshold: f32,
    pub current_health_threshold: f32,
    pub agility_base: f32,
    pub agility_mult: f32,
    pub damage_base: f32,
    pub damage_mult: f32,
    pub chance: f32,
}

impl KnockdownSettings {
    pub fn read(order: &LoadOrder) -> KnockdownSettings {
        KnockdownSettings {
            base_health_threshold: setting(order, "fKnockdownBaseHealthThreshold", 75.0),
            current_health_threshold: setting(order, "fKnockdownCurrentHealthThreshold", 25.0),
            agility_base: setting(order, "fKnockdownAgilBase", 0.0),
            agility_mult: setting(order, "fKnockdownAgilMult", 1.0),
            damage_base: setting(order, "fKnockdownDamageBase", 0.0),
            damage_mult: setting(order, "fKnockdownDamageMult", 0.3),
            chance: setting(order, "fKnockdownChance", 0.25),
        }
    }
}

/// Whether a hit of `damage` (whole points) knocks someone down
/// (`CombatFormulas::CheckKnockdown` (Xbox PDB), `00646400`), on their
/// health now and at full (actor value 16) and Agility (10); `roll` a
/// random number (its remainder by 1000 is the roll). Only for those it
/// doesn't kill: more than `fKnockdownBaseHealthThreshold` % of their full
/// health always; else more than `fKnockdownCurrentHealthThreshold` % of
/// their health now gives a chance of (damage × `…DamageMult` +
/// `…DamageBase`) ÷ (`…AgilMult` × Agility × 10 + `…AgilBase`), at most
/// `fKnockdownChance`.
// Translated from 00646400 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn knockdown_by_formula(
    (health_now, health_full): (f32, f32),
    agility: f32,
    damage: i32,
    s: &KnockdownSettings,
    roll: u64,
) -> bool {
    let d = damage as f32;
    if health_now <= 0.0 || health_now - d <= 0.0 {
        return false;
    }
    if d / health_full * 100.0 > s.base_health_threshold {
        return true;
    }
    if d / health_now * 100.0 <= s.current_health_threshold {
        return false;
    }
    let mut chance =
        (d * s.damage_mult + s.damage_base) / (s.agility_mult * (agility * 10.0) + s.agility_base);
    if chance > s.chance {
        chance = s.chance;
    }
    ((roll % 1000) as i64) < (chance * 1000.0) as i64
}

/// Whether an explosion knocks down someone it hurt and didn't kill
/// (`Explosion::ProcessTargets`, `009b00a0`, after the hit is handled and
/// the object effect cast): with the record's "knock down always" flag
/// (0x04), always; with "by formula" (0x08), as [`knockdown_by_formula`]
/// says for the hit's damage (the explosion's at their distance, before
/// armour: the hit data's `+0x14`, `HitData::InitializeExplosionData`)
/// against their health after it. The
/// knockdown itself (`Explosion::PushActor`, `009b0d70`: the body thrown
/// as a ragdoll, then getting up) is the physics' and animation's.
pub fn knocks_down(
    order: &LoadOrder,
    state: &GameState,
    explosion: &ExplosionRecord,
    target: FormId,
    damage: f32,
    roll: u64,
) -> bool {
    if state.dead.contains(&target) {
        return false;
    }
    if explosion.flags & expl_flags::KNOCK_DOWN_ALWAYS != 0 {
        return true;
    }
    if explosion.flags & expl_flags::KNOCK_DOWN_BY_FORMULA == 0 {
        return false;
    }
    let now = crate::combat::health(order, state, target).unwrap_or(0.0) as f32;
    let full = crate::combat::max_health(order, state, target).unwrap_or(0.0) as f32;
    let agility = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(target, AGILITY)
    .unwrap_or(0.0) as f32;
    knockdown_by_formula(
        (now, full),
        agility,
        damage as i32,
        &KnockdownSettings::read(order),
        roll,
    )
}

/// Agility's actor value.
const AGILITY: u16 = 10;

/// The explosion's object effect on someone it hit (`009b00a0`: cast by
/// the explosion's actor cause, else the explosion itself, on each target
/// that passed the line of sight): the pulse grenades' EMP, incendiary
/// fire. Says what each effect did.
pub fn cast_enchantment(
    order: &LoadOrder,
    state: &mut GameState,
    explosion: &ExplosionRecord,
    source: Option<FormId>,
    target: FormId,
) -> Vec<String> {
    match explosion.enchantment {
        Some(e) => crate::magic::apply(order, state, target, e, source.unwrap_or(target), false),
        None => Vec::new(),
    }
}

/// What the combat AI's danger test reads (`00992720`):
/// `fDangerousProjectileExplosionDamage` (exe 5),
/// `fDangerousProjectileExplosionRadius` (exe 30) and
/// `iAvoidHurtingNonTargetsResponsibility` (exe 50); no plugin sets them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DangerSettings {
    pub min_damage: f32,
    pub min_radius: f32,
    pub responsibility: f32,
}

impl DangerSettings {
    pub fn read(order: &LoadOrder) -> DangerSettings {
        DangerSettings {
            min_damage: setting(order, "fDangerousProjectileExplosionDamage", 5.0),
            min_radius: setting(order, "fDangerousProjectileExplosionRadius", 30.0),
            responsibility: setting(order, "iAvoidHurtingNonTargetsResponsibility", 50.0),
        }
    }
}

/// How far from an explosion of `radius` and `damage` it still does
/// `least` damage (`006479a0` → `00647960`, [`falloff`] turned round):
/// radius × √(1 − least ÷ damage); 0 when it never does that much.
// Translated from 006479a0 and 00647960 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn reach_of_damage(radius: f32, damage: f32, least: f32) -> f32 {
    let x = least / damage;
    if x <= 1.0 && 1.0 - x >= 0.0 {
        (1.0 - x).sqrt() * radius
    } else {
        0.0
    }
}

/// Combat style flags (`CSTD` u16 at 80, the style's `+0xa8`, tested by
/// `009928c0`) that let an attacker's explosion hurt its own side.
pub mod style_flags {
    pub const IGNORE_DAMAGING_SELF: u16 = 0x20;
    pub const IGNORE_DAMAGING_GROUP: u16 = 0x40;
    pub const IGNORE_DAMAGING_SPECTATORS: u16 = 0x80;
}

/// Who stands where an attacker's explosion would reach (`00992ba0`
/// counts the living within the reach): the attacker, their combat group,
/// and spectators (those on neither side; the attacker's targets aren't
/// counted against it).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Nearby {
    pub own: u32,
    pub group: u32,
    pub spectators: u32,
}

/// Whether someone may fire an exploding projectile at a point
/// (`CombatManager::CheckExplosionAttack` (Xbox PDB), `00992720`, asked by
/// the attack procedures before they fire): an explosion (`radius` in
/// units, the record's `damage`) of at least `min_damage` whose damage
/// still reaches `min_damage` at least `min_radius` out ([`reach_of_damage`])
/// is refused when, within that reach of the point (`count`), there's the
/// attacker (unless their style ignores damaging themselves), their combat
/// group (unless it ignores damaging the group), or spectators while the
/// attacker's Responsibility is at least the setting (unless it ignores
/// damaging spectators). `None` (no explosion) is always allowed.
// Translated from 00992720 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn explosion_attack_allowed(
    explosion: Option<(f32, f32)>,
    s: &DangerSettings,
    style: u16,
    responsibility: f32,
    count: impl FnOnce(f32) -> Nearby,
) -> bool {
    let Some((radius, damage)) = explosion else {
        return true;
    };
    if damage < s.min_damage {
        return true;
    }
    let reach = reach_of_damage(radius, damage, s.min_damage);
    if reach < s.min_radius {
        return true;
    }
    let n = count(reach);
    if n.own != 0 && style & style_flags::IGNORE_DAMAGING_SELF == 0 {
        return false;
    }
    if n.group != 0 && style & style_flags::IGNORE_DAMAGING_GROUP == 0 {
        return false;
    }
    !(n.spectators != 0
        && responsibility >= s.responsibility
        && style & style_flags::IGNORE_DAMAGING_SPECTATORS == 0)
}

/// Which part of a target a thrower aims at (`009a8bf0`): 2 (¾ up) for a
/// high arc of a falling projectile; else 4 (the feet) for splash damage —
/// an exploding projectile, not hitscan and no faster than
/// `fCombatSplashDamageMaxSpeed` (3000), whose explosion has at least
/// `fCombatSplashDamageMinRadius` (50) units of radius and more than
/// `fCombatSplashDamageMinDamage` (20) damage; else 1 (halfway up).
// Translated from 009a8bf0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn aim_mode(
    projectile: &ProjectileRecord,
    explosion: Option<(f32, f32)>,
    high_arc: bool,
    splash: (f32, f32, f32),
) -> u8 {
    if high_arc && projectile.fall_gravity() != 0.0 {
        return 2;
    }
    let (max_speed, min_radius, min_damage) = splash;
    let fast = projectile.flags & proj_flags::HITSCAN != 0 || projectile.speed > max_speed;
    match explosion {
        Some((radius, damage)) if radius >= min_radius && !fast && damage > min_damage => 4,
        _ => 1,
    }
}

/// How far up a standing target of `height` the aim goes for a mode
/// (`009a8460`): 0 ¼, 1 and 3 half, 2 ¾, else (4) the feet.
// Translated from 009a8460 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn aim_height(mode: u8, height: f32) -> f32 {
    match mode {
        0 => height * 0.25,
        1 | 3 => height * 0.5,
        2 => height * 0.75,
        _ => 0.0,
    }
}

/// The pitch (radians) to launch at to land `x` units away and `y` up at
/// `speed` under `gravity` (`009a7c60`): the lower solution, or the higher
/// one for a high arc; 45° when it can't reach.
// Translated from 009a7c60 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn launch_pitch(x: f32, y: f32, speed: f32, gravity: f32, high: bool) -> f32 {
    let v2 = speed * speed;
    let disc = v2 * v2 - (y * 2.0 * v2 + gravity * x * x) * gravity;
    if disc <= 0.0 {
        return std::f32::consts::PI * 0.25;
    }
    let root = disc.sqrt();
    let low = (v2 - root) / (x * gravity);
    let hi = (v2 + root) / (x * gravity);
    if high { low.max(hi) } else { low.min(hi) }.atan()
}

/// The point to throw at from `from` to land on `target` (`009a8f00` for
/// a still target): straight at it without gravity; else the point above
/// it on the launch pitch's line ([`launch_pitch`]).
// Translated from 009a8f00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn aim_point(
    from: [f32; 3],
    target: [f32; 3],
    speed: f32,
    gravity: f32,
    high: bool,
) -> [f32; 3] {
    if gravity == 0.0 {
        return target;
    }
    let x = (target[0] - from[0]).hypot(target[1] - from[1]);
    if !(1.0..=8192.0).contains(&x) {
        return target;
    }
    let pitch = launch_pitch(x, target[2] - from[2], speed, gravity, high);
    [target[0], target[1], pitch.tan() * x + from[2]]
}

/// Whether a throw's arc is clear (`009a7670` with `009a6e90`): the arc
/// from `from` along `direction` at `speed` under `gravity`, cut into
/// `segments` (`iBallisticProjectilePathPickSegments`, 4) straight pieces
/// up to where it comes down at `target`'s distance, must not meet
/// anything (`cast`) before `threshold` (`fGrenadeThrowHitFractionThreshold`,
/// 0.8) of the way. How `009a6e90` cuts the arc is read in outline only.
pub fn arc_clear(
    from: [f32; 3],
    direction: [f32; 3],
    (speed, gravity): (f32, f32),
    target: [f32; 3],
    (segments, threshold): (u32, f32),
    cast: &mut Cast,
) -> bool {
    let x = (target[0] - from[0]).hypot(target[1] - from[1]);
    let horizontal = direction[0].hypot(direction[1]) * speed;
    if horizontal < 1e-3 {
        return true;
    }
    let flight = x / horizontal;
    let at = |t: f32| {
        [
            from[0] + direction[0] * speed * t,
            from[1] + direction[1] * speed * t,
            from[2] + direction[2] * speed * t - 0.5 * gravity * t * t,
        ]
    };
    let n = segments.max(1);
    for i in 0..n {
        let (a, b) = (
            at(flight * i as f32 / n as f32),
            at(flight * (i + 1) as f32 / n as f32),
        );
        let l = length(sub(b, a));
        if l < 1e-3 {
            continue;
        }
        if let Some(d) = cast(a, scale(sub(b, a), 1.0 / l), l) {
            let fraction = (i as f32 + d / l) / n as f32;
            return fraction >= threshold;
        }
    }
    true
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn length(a: [f32; 3]) -> f32 {
    dot(a, a).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The dynamite's projectile as `FalloutNV.esm` has it (made up here
    /// byte by byte from its documented fields): explosion, alt. trigger
    /// and rotation, a lobber, gravity 1, speed 1200, range 10000, timer
    /// 2.5, bounciness 1.
    fn dynamite_data() -> Vec<u8> {
        let mut d = vec![0u8; 84];
        d[0..2].copy_from_slice(&0x0806u16.to_le_bytes());
        d[2..4].copy_from_slice(&2u16.to_le_bytes());
        d[4..8].copy_from_slice(&1.0f32.to_le_bytes());
        d[8..12].copy_from_slice(&1200.0f32.to_le_bytes());
        d[12..16].copy_from_slice(&10000.0f32.to_le_bytes());
        d[32..36].copy_from_slice(&2.5f32.to_le_bytes());
        d[36..40].copy_from_slice(&0x0E397Fu32.to_le_bytes());
        d[52..56].copy_from_slice(&0.5f32.to_le_bytes());
        d[80..84].copy_from_slice(&1.0f32.to_le_bytes());
        d
    }

    fn dynamite() -> ProjectileRecord {
        ProjectileRecord::parse(FormId(0xE3F03), &dynamite_data(), |r| {
            Some(FormId(r)).filter(|f| f.0 != 0)
        })
        .unwrap()
    }

    #[test]
    fn projectile_data_reads_its_fields() {
        let p = dynamite();
        assert!(p.is_lobber() && p.explodes() && p.alt_trigger() && !p.is_mine());
        assert_eq!(p.explosion, Some(FormId(0xE397F)));
        assert_eq!((p.speed, p.timer, p.bounciness), (1200.0, 2.5, 1.0));
        assert_eq!(p.fall_gravity(), 1.0);
    }

    #[test]
    fn explosion_data_reads_its_fields_and_radius_units() {
        let mut d = vec![0u8; 52];
        for (at, v) in [(0, 90.0f32), (4, 75.0), (8, 750.0), (24, 1500.0)] {
            d[at..at + 4].copy_from_slice(&v.to_le_bytes());
        }
        d[20..24].copy_from_slice(&0x49u32.to_le_bytes());
        let e = ExplosionRecord::parse(FormId(1), &d, |_| None).unwrap();
        assert_eq!((e.force, e.damage, e.radius), (90.0, 75.0, 750.0));
        // Flag 0x01: game units; without it, feet.
        assert_eq!(e.radius_units(22.0), 750.0);
        let feet = ExplosionRecord { flags: 0x48, ..e };
        assert_eq!(feet.radius_units(22.0), 750.0 * 22.0);
    }

    #[test]
    fn falloff_is_one_less_the_squared_share_of_the_radius() {
        assert_eq!(falloff(750.0, 0.0), 1.0);
        assert!((falloff(750.0, 375.0) - 0.75).abs() < 1e-6);
        assert_eq!(falloff(750.0, 750.0), 0.0);
        assert_eq!(falloff(750.0, 800.0), 0.0);
    }

    #[test]
    fn damage_takes_the_condition_settings_and_the_makers_skill() {
        // 75 × (0.66 + 0.34) = 75; Explosives 50: × (0.5 + 0.5 × 0.5).
        assert!((damage_with(75.0, (0.66, 0.34), (0.5, 0.5), None) - 75.0).abs() < 1e-4);
        let d = damage_with(75.0, (0.66, 0.34), (0.5, 0.5), Some(50.0));
        assert!((d - 56.25).abs() < 1e-4, "{d}");
    }

    #[test]
    fn grenade_contacts_take_the_settings_restitution() {
        let p = dynamite();
        let s = FlightSettings::with(&p, 0.01, 5.0, 90.0);
        assert!((s.restitution - 0.01).abs() < 1e-6);
        let flat = ProjectileRecord {
            bounciness: 0.0,
            ..p.clone()
        };
        assert!((FlightSettings::with(&flat, 0.01, 5.0, 90.0).restitution - 0.01).abs() < 1e-6);
        assert_eq!(FlightSettings::with(&p, 0.0, 0.0, 90.0).restitution, 0.0);
    }

    /// A floor at z = 0.
    fn floor(from: [f32; 3], dir: [f32; 3], len: f32) -> Option<(f32, [f32; 3])> {
        if dir[2] >= 0.0 {
            return None;
        }
        let d = -from[2] / dir[2];
        (d >= 0.0 && d <= len).then_some((d, [0.0, 0.0, 1.0]))
    }

    #[test]
    fn dynamite_lands_stops_and_goes_off_when_its_fuse_runs_out() {
        let p = dynamite();
        let s = FlightSettings::with(&p, 0.01, 5.0, 90.0);
        let dir = [0.0, 45f32.to_radians().cos(), 45f32.to_radians().sin()];
        let mut f = Flight::launch(p, [0.0, 0.0, 100.0], dir, 600.0);
        let dt = 1.0 / 60.0;
        let mut exploded = None;
        for i in 0..400 {
            if let FlightEvent::Explode { at } = f.step(dt, &s, &mut floor) {
                exploded = Some((i, at));
                break;
            }
        }
        let (frame, at) = exploded.expect("it goes off");
        // 2.5 s after the throw (150 frames, give or take the rounding).
        assert!((149..=151).contains(&(frame + 1)), "{frame}");
        assert!(f.resting && f.impact.is_some());
        // Lying on the floor, not far past where it first came down
        // (friction 5 stops it).
        assert!(at[2].abs() < 1.0, "{at:?}");
        let first = f.impact.unwrap();
        assert!((at[1] - first[1]).abs() < 50.0, "{at:?} {first:?}");
    }

    #[test]
    fn impact_grenades_go_off_where_they_strike() {
        let p = ProjectileRecord {
            flags: proj_flags::EXPLOSION,
            ..dynamite()
        };
        let s = FlightSettings::with(&p, 0.01, 5.0, 90.0);
        let mut f = Flight::launch(p, [0.0, 0.0, 50.0], [0.0, 0.0, -1.0], 500.0);
        let mut event = FlightEvent::Flying;
        for _ in 0..30 {
            event = f.step(1.0 / 60.0, &s, &mut floor);
            if event != FlightEvent::Flying {
                break;
            }
        }
        let FlightEvent::Explode { at } = event else {
            panic!("{event:?}");
        };
        assert!(at[2].abs() < 1.0);
    }

    #[test]
    fn unexploded_grenades_expire_with_age() {
        let p = ProjectileRecord {
            flags: proj_flags::EXPLOSION | proj_flags::ALT_TRIGGER | proj_flags::DETONATES,
            ..dynamite()
        };
        let s = FlightSettings::with(&p, 0.01, 5.0, 2.0);
        let mut f = Flight::launch(p, [0.0, 0.0, 10.0], [1.0, 0.0, 0.0], 0.0);
        assert!(f.fuse.is_none());
        let events: Vec<FlightEvent> = (0..130)
            .map(|_| f.step(1.0 / 60.0, &s, &mut floor))
            .collect();
        assert!(events.contains(&FlightEvent::Expired));
        assert!(!events
            .iter()
            .any(|e| matches!(e, FlightEvent::Explode { .. })));
    }

    fn pick(normal: [f32; 3]) -> LosPick {
        LosPick {
            normal,
            buffer_distance: 24.0,
            buffer: 6.0,
            owner_passes: false,
        }
    }

    #[test]
    fn walls_hide_targets_and_the_pick_starts_off_the_surface() {
        // A wall at x = 100.
        let mut wall = |from: [f32; 3], dir: [f32; 3], len: f32| {
            if dir[0] <= 0.0 {
                return None;
            }
            let d = (100.0 - from[0]) / dir[0];
            (d >= 0.0 && d <= len).then_some(d)
        };
        let none = pick([0.0; 3]);
        assert!(!los_clear(
            [0.0; 3],
            [200.0, 0.0, 0.0],
            None,
            &none,
            &mut wall
        ));
        assert!(los_clear(
            [0.0; 3],
            [50.0, 0.0, 0.0],
            None,
            &none,
            &mut wall
        ));
        // An owner with flag 0x01000000: nothing blocks the first point.
        let owned = LosPick {
            owner_passes: true,
            ..none
        };
        assert!(los_clear(
            [0.0; 3],
            [200.0, 0.0, 0.0],
            None,
            &owned,
            &mut wall
        ));
        // A floor (z = 0, two-sided) under a grenade lying on it: the
        // normal points up, so the pick starts 24 above it and a target
        // 200 away, 1 above the floor, is seen.
        let mut floor = |from: [f32; 3], dir: [f32; 3], len: f32| {
            if dir[2] == 0.0 {
                return None;
            }
            let d = -from[2] / dir[2];
            (d >= 0.0 && d <= len).then_some(d)
        };
        let up = pick([0.0, 0.0, 1.0]);
        assert!(los_clear(
            [0.0; 3],
            [200.0, 0.0, 1.0],
            None,
            &up,
            &mut floor
        ));
        // Without a surface found the start is the floor itself: hit at 0.
        assert!(!los_clear(
            [0.0; 3],
            [200.0, 0.0, 1.0],
            None,
            &none,
            &mut floor
        ));
    }

    #[test]
    fn under_a_ceiling_the_near_surfaces_are_let_through() {
        // A ceiling at z = 0 touched by the blast: its normal points down,
        // the pick starts 24 below. A ceiling slab hit 4 units from the
        // start (within fExplosionLOSBuffer) doesn't block; one 10 away
        // does.
        let at = |d0: f32| move |_: [f32; 3], _: [f32; 3], len: f32| (d0 <= len).then_some(d0);
        let down = pick([0.0, 0.0, -1.0]);
        let near = at(4.0);
        let mut seen = 0;
        let mut once = |f: [f32; 3], d: [f32; 3], l: f32| {
            seen += 1;
            if seen == 1 {
                near(f, d, l)
            } else {
                None
            }
        };
        assert!(los_clear(
            [0.0; 3],
            [100.0, 0.0, -24.0],
            None,
            &down,
            &mut once
        ));
        let mut far = at(10.0);
        assert!(!los_clear(
            [0.0; 3],
            [100.0, 0.0, -24.0],
            None,
            &down,
            &mut far
        ));
    }

    #[test]
    fn the_closest_surface_is_the_first_one_the_explosion_meets() {
        // 009ae6e0: two contacts; the first is behind another body, the
        // second is met directly.
        let contacts = [
            Contact {
                body: 1,
                point: [10.0, 0.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Contact {
                body: 2,
                point: [0.0, 0.0, -0.2],
                normal: [0.0, 0.0, 1.0],
            },
        ];
        let mut first = |from: [f32; 3], ray: [f32; 3]| {
            if ray[0] > 0.0 {
                Some(3)
            } else if ray[2] > 0.0 {
                // Under 1 unit away: from 16 below, 32 up.
                assert_eq!(from, [0.0, 0.0, -16.0]);
                Some(2)
            } else {
                None
            }
        };
        assert_eq!(
            closest_surface([0.0; 3], &contacts, &mut first),
            Some(([0.0, 0.0, -0.2], [0.0, 0.0, 1.0]))
        );
    }

    #[test]
    fn a_blocked_body_is_seen_by_its_offset_points() {
        // A low wall at x = 100 up to z = 50: the point 2 × 40 above the
        // target's (at z 0) clears it from an explosion at z 60.
        let mut low = |from: [f32; 3], dir: [f32; 3], len: f32| {
            if dir[0] <= 0.0 {
                return None;
            }
            let d = (100.0 - from[0]) / dir[0];
            let z = from[2] + dir[2] * d;
            (d >= 0.0 && d <= len && z <= 50.0).then_some(d)
        };
        let center = [0.0, 0.0, 60.0];
        let target = [200.0, 0.0, 0.0];
        let none = pick([0.0; 3]);
        assert!(!los_clear(center, target, None, &none, &mut low));
        assert!(los_clear(center, target, Some(40.0), &none, &mut low));
    }
    #[test]
    fn blast_targets_are_those_within_the_radius_nearest_first() {
        let t = blast_targets(
            [0.0; 3],
            750.0,
            &[
                (FormId(1), [500.0, 0.0, 0.0]),
                (FormId(2), [0.0, 100.0, 0.0]),
                (FormId(3), [800.0, 0.0, 0.0]),
            ],
        );
        let refs: Vec<FormId> = t.iter().map(|b| b.reference).collect();
        assert_eq!(refs, [FormId(2), FormId(1)]);
        assert!((t[1].share - (1.0 - (500.0f32 / 750.0).powi(2))).abs() < 1e-6);
    }

    #[test]
    fn launch_pitches_land_the_throw() {
        let g = crate::combat_ai::WORLD_GRAVITY;
        let (x, v) = (800.0, 1200.0);
        for high in [false, true] {
            let a = launch_pitch(x, 0.0, v, g, high);
            // Range on flat ground: v² sin 2a ÷ g.
            let range = v * v * (2.0 * a).sin() / g;
            assert!((range - x).abs() < 1.0, "{high}: {range}");
        }
        assert!(launch_pitch(x, 0.0, v, g, true) > launch_pitch(x, 0.0, v, g, false));
        // Out of reach: 45°.
        assert!(
            (launch_pitch(10_000.0, 0.0, v, g, false) - std::f32::consts::FRAC_PI_4).abs() < 1e-6
        );
    }

    #[test]
    fn aim_points_rise_with_the_pitch() {
        let g = crate::combat_ai::WORLD_GRAVITY;
        let p = aim_point([0.0; 3], [0.0, 800.0, 0.0], 1200.0, g, false);
        let pitch = launch_pitch(800.0, 0.0, 1200.0, g, false);
        assert!((p[2] - pitch.tan() * 800.0).abs() < 1e-3);
        assert_eq!(
            aim_point([0.0; 3], [0.0, 800.0, 0.0], 1200.0, 0.0, false)[2],
            0.0
        );
    }

    #[test]
    fn dynamite_is_aimed_at_the_feet_and_high_arcs_higher() {
        let p = dynamite();
        let splash = (3000.0, 50.0, 20.0);
        assert_eq!(aim_mode(&p, Some((750.0, 75.0)), false, splash), 4);
        assert_eq!(aim_mode(&p, Some((750.0, 75.0)), true, splash), 2);
        assert_eq!(aim_mode(&p, Some((750.0, 10.0)), false, splash), 1);
        assert_eq!(aim_height(4, 128.0), 0.0);
        assert_eq!(aim_height(2, 128.0), 96.0);
    }

    #[test]
    fn a_ceiling_blocks_the_high_arc_but_not_the_low_one() {
        let g = crate::combat_ai::WORLD_GRAVITY;
        let target = [0.0, 800.0, 0.0];
        let aim = |high: bool| {
            let p = aim_point([0.0; 3], target, 1200.0, g, high);
            let l = length(p);
            p.map(|c| c / l)
        };
        // A ceiling at z 300: the low arc (peak ~ 70) passes; the high one
        // doesn't.
        let mut ceiling = |from: [f32; 3], dir: [f32; 3], len: f32| {
            if dir[2] <= 0.0 {
                return None;
            }
            let d = (300.0 - from[2]) / dir[2];
            (d >= 0.0 && d <= len).then_some(d)
        };
        assert!(arc_clear(
            [0.0; 3],
            aim(false),
            (1200.0, g),
            target,
            (4, 0.8),
            &mut ceiling
        ));
        assert!(!arc_clear(
            [0.0; 3],
            aim(true),
            (1200.0, g),
            target,
            (4, 0.8),
            &mut ceiling
        ));
    }
}
