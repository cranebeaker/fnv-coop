//! Guns' projectiles: whether a shot strikes at once or flies there, and
//! a flying one's way (read from FalloutNV.exe 1.4.0.525 with Ghidra;
//! class names from the Xbox 360 prototype's symbols, marked (Xbox PDB);
//! the energy weapons' evidence is in `docs/ENERGY_WEAPONS.md`). Thrown
//! weapons and grenades are [`crate::explosions`].
//!
//! - A projectile's kind (`PROJ` `DATA` type, [`crate::explosions::proj_type`])
//!   picks its class: beams make a `BeamProjectile`, missiles a
//!   `MissileProjectile` (Xbox PDB). Each class's `Initialize` sets the
//!   projectile's run-time flag 0x1, which `Projectile::Initialize`
//!   (`009bda10` → `009bec90`) turns into a cast along the whole range as
//!   it's made: the hit is decided then (see [`delivery`]).
//! - A missile without it (the plasma bolts) moves each frame by its speed
//!   (`009bf300`: `009669c0`'s speed × the frame's seconds, along its
//!   heading) and is checked for what it struck (`009c3190`); past
//!   `fArrowAgeMax` seconds, or past its range with nothing struck, it's
//!   removed (`MissileProjectile::UpdateProjectile`, `009b8030`). See
//!   [`Missile`].
//! - Every projectile moves through a Havok character controller (a
//!   `ProjectileListener` (Xbox PDB), made by `Projectile::InitHavok`,
//!   `009be0a0` → `009c6440`) whose gravity is the projectile's
//!   (`00966980`: 1 for lobbers, else the record's, `PROJ` `DATA` f32 at 4;
//!   ÷ the time multiplier while its shooter has Turbo, effect 0x33). So a
//!   missile with gravity (the grenade launchers' 40 mm grenades 1.5, the
//!   Fat Man's nuke 1, the rockets 0) falls under the world's gravity × it
//!   while its speed carries it along its heading (`009bf300`); its model is
//!   turned to its way (run-time flag 0x40, set by `009b7cc0` for gravity
//!   without "hitscan", read by `009bf470`). The AI's ballistic aim
//!   (`009a7c60`, [`crate::explosions::launch_pitch`]) assumes that path.

use esm::LoadOrder;

use crate::explosions::{proj_flags, proj_type, ProjectileRecord};

/// How a projectile's shot meets what it's fired at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    /// Decided as it's fired, by a cast along its range: beams (the
    /// lasers: `BeamProjectile::Initialize`, `00979280`, sets run-time
    /// flag 0x1 always) and missiles with the record's "hitscan" flag
    /// (bullets: `MissileProjectile::Initialize`, `009b7cc0`, sets it
    /// from `009a7f80`, the record's flag 0x1).
    AtOnce,
    /// Flies at its speed and strikes what it meets on the way: missiles
    /// without "hitscan" (the plasma bolts, `PlasmaProjectile` flags
    /// 0x20C).
    Flies,
    /// Thrown or lobbed (lobbers, `crate::explosions`).
    Lobbed,
    /// Flames and continuous beams: their classes' `Initialize` isn't read
    /// yet (treated as decided at once by the callers).
    NotTraced,
}

/// How a projectile is delivered (see [`Delivery`]). While V.A.T.S. plays
/// its queue (the manager's mode, `[011f2250]+0x08`, is 4,
/// [`crate::vats::mode::PLAYBACK`]) `009b7cc0` leaves flag 0x1 off for
/// every missile, so bullets fly too; beams strike at once even then.
pub fn delivery(p: &ProjectileRecord, vats_playback: bool) -> Delivery {
    match p.kind {
        proj_type::BEAM => Delivery::AtOnce,
        proj_type::MISSILE if p.flags & proj_flags::HITSCAN != 0 && !vats_playback => {
            Delivery::AtOnce
        }
        proj_type::MISSILE => Delivery::Flies,
        proj_type::LOBBER => Delivery::Lobbed,
        _ => Delivery::NotTraced,
    }
}

/// `fArrowAgeMax` (the exe's 90 s; no plugin sets it): how long a missile
/// may fly (`009b8030`).
pub fn age_max(order: &LoadOrder) -> f32 {
    crate::scripting::game_setting(order, "fArrowAgeMax").unwrap_or(90.0)
}

/// The gravity a missile falls with, units a second² (`009be0a0`: the
/// controller's gravity, [`ProjectileRecord::fall_gravity`], × the
/// world's, [`crate::combat_ai::WORLD_GRAVITY`]); 0 for a hitscan one,
/// which strikes at once. Turbo's share (÷ the time multiplier) isn't
/// modelled.
pub fn missile_gravity(p: &ProjectileRecord) -> f32 {
    if p.flags & proj_flags::HITSCAN != 0 {
        0.0
    } else {
        crate::combat_ai::WORLD_GRAVITY * p.fall_gravity()
    }
}

/// A missile in flight (`MissileProjectile`, Xbox PDB): where it is, its
/// heading (unit), its speed (units a second, as launched:
/// [`crate::explosions::launch_speed`]), the gravity it falls with
/// ([`missile_gravity`]) and how fast it falls now (units a second, its
/// controller's vertical speed), its range (the record's, the
/// projectile's `+0xd4`, `009a7c40`), how far it has gone (`+0x110`, the
/// length of each frame's move, `009c4e60`) and for how long (`+0xd8`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Missile {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub speed: f32,
    pub gravity: f32,
    pub falling: f32,
    pub range: f32,
    pub travelled: f32,
    pub age: f32,
}

/// One frame's way: the unit direction and the length.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stretch {
    pub direction: [f32; 3],
    pub length: f32,
}

impl Missile {
    pub fn launch(position: [f32; 3], direction: [f32; 3], speed: f32, range: f32) -> Missile {
        Missile {
            position,
            direction,
            speed,
            gravity: 0.0,
            falling: 0.0,
            range,
            travelled: 0.0,
            age: 0.0,
        }
    }

    /// The same, falling with `gravity` (units a second², [`missile_gravity`]).
    pub fn with_gravity(mut self, gravity: f32) -> Missile {
        self.gravity = gravity;
        self
    }

    /// How far it goes this frame: its speed × the frame's seconds along
    /// its heading (`009bf300`) and, with gravity, its fall: the
    /// controller's vertical speed gains the gravity × the frame's seconds
    /// first, then moves it (the order Havok's character controller steps
    /// in is read from its use, not traced).
    pub fn stretch(&self, seconds: f32) -> Stretch {
        let t = seconds.max(0.0);
        let falling = self.falling + self.gravity * t;
        let mut v = self.direction.map(|d| d * self.speed);
        v[2] -= falling;
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if l < 1e-6 || t == 0.0 {
            return Stretch {
                direction: self.direction,
                length: 0.0,
            };
        }
        Stretch {
            direction: v.map(|x| x / l),
            length: l * t,
        }
    }

    /// How far it goes this frame along its heading alone (no fall).
    pub fn step_length(&self, seconds: f32) -> f32 {
        self.speed * seconds.max(0.0)
    }

    /// Moves it along `stretch` (from [`Missile::stretch`] of the same
    /// `seconds`), that much older and falling faster.
    pub fn advance(&mut self, stretch: Stretch, seconds: f32) {
        let t = seconds.max(0.0);
        for k in 0..3 {
            self.position[k] += stretch.direction[k] * stretch.length;
        }
        self.falling += self.gravity * t;
        self.travelled += stretch.length;
        self.age += t;
    }

    /// Whether it's done with (`009b8030`, after the frame's impacts):
    /// older than `age_max` ([`age_max`]), or past its range with nothing
    /// struck. The frame that takes it past its range still flies whole.
    pub fn spent(&self, age_max: f32) -> bool {
        self.age > age_max || self.travelled > self.range
    }

    /// How long it takes to go `distance` (seconds) along its heading.
    pub fn time_to(&self, distance: f32) -> f32 {
        if self.speed > 0.0 {
            distance / self.speed
        } else {
            f32::INFINITY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use esm::FormId;

    fn record(flags: u16, kind: u16, speed: f32) -> ProjectileRecord {
        let mut d = flags.to_le_bytes().to_vec();
        d.extend(kind.to_le_bytes());
        for v in [0.0f32, speed, 10000.0] {
            d.extend(v.to_le_bytes());
        }
        d.resize(84, 0);
        ProjectileRecord::parse(FormId(1), &d, |_| None).unwrap()
    }

    #[test]
    fn beams_and_hitscan_missiles_strike_at_once_plasma_flies() {
        // `BeamLaserProjectile` (0x8C, beam), a 9mm bullet (0x01 hitscan,
        // missile), `PlasmaProjectile` (0x20C, missile), dynamite (lobber).
        assert_eq!(delivery(&record(0x8C, 4, 10000.0), false), Delivery::AtOnce);
        assert_eq!(delivery(&record(0x01, 1, 23680.0), false), Delivery::AtOnce);
        assert_eq!(delivery(&record(0x20C, 1, 7500.0), false), Delivery::Flies);
        assert_eq!(delivery(&record(0x806, 2, 1200.0), false), Delivery::Lobbed);
        assert_eq!(
            delivery(&record(0x8D, 8, 12000.0), false),
            Delivery::NotTraced
        );
        // V.A.T.S.'s playback: bullets fly, beams don't.
        assert_eq!(delivery(&record(0x01, 1, 23680.0), true), Delivery::Flies);
        assert_eq!(delivery(&record(0x8C, 4, 10000.0), true), Delivery::AtOnce);
    }

    #[test]
    fn a_plasma_bolt_flies_its_range_then_is_spent() {
        // The plasma pistol's bolt: 7500 units a second, range 10000.
        let mut m = Missile::launch([0.0; 3], [0.0, 1.0, 0.0], 7500.0, 10000.0);
        assert!((m.time_to(1500.0) - 0.2).abs() < 1e-6);
        let mut frames = 0;
        while !m.spent(90.0) {
            let l = m.stretch(0.125);
            assert_eq!(l.length, 937.5);
            assert_eq!(l.direction, [0.0, 1.0, 0.0]);
            m.advance(l, 0.125);
            frames += 1;
        }
        // Ten frames go 9375; the eleventh, flown whole, takes it past.
        assert_eq!(frames, 11);
        assert_eq!(m.position[1], 10312.5);
        // A slow one is spent by age.
        let mut slow = Missile::launch([0.0; 3], [1.0, 0.0, 0.0], 10.0, 10000.0);
        slow.advance(slow.stretch(91.0), 91.0);
        assert!(slow.spent(90.0) && slow.travelled < slow.range);
    }

    #[test]
    fn grenade_launcher_rounds_fall_rockets_fly_straight() {
        // `40mmGrenadeProjectile`: flags 0x0A, gravity 1.5, speed 1750;
        // `MissileProjectile` (the rocket): gravity 0, speed 1550;
        // `FatMan`: flags 0x02, gravity 1, speed 2500.
        let mut grenade = record(0x0A, 1, 1750.0);
        grenade.gravity = 1.5;
        let rocket = record(0x0A, 1, 1550.0);
        let mut nuke = record(0x02, 1, 2500.0);
        nuke.gravity = 1.0;
        let g = crate::combat_ai::WORLD_GRAVITY;
        assert_eq!(missile_gravity(&grenade), 1.5 * g);
        assert_eq!(missile_gravity(&rocket), 0.0);
        assert_eq!(missile_gravity(&nuke), g);
        // A bullet with gravity still strikes at once: none.
        let mut bullet = record(0x01, 1, 23680.0);
        bullet.gravity = 1.0;
        assert_eq!(missile_gravity(&bullet), 0.0);
        let fly = |p: &ProjectileRecord, dir: [f32; 3], seconds: f32| {
            let mut m =
                Missile::launch([0.0; 3], dir, p.speed, p.range).with_gravity(missile_gravity(p));
            let mut left = seconds;
            while left > 1e-5 {
                let dt = left.min(1.0 / 60.0);
                let s = m.stretch(dt);
                m.advance(s, dt);
                left -= dt;
            }
            m
        };
        // Fired level for a second: the grenade drops by the controller's
        // fall (Σ g·i·dt² over 60 frames = 0.508 g); the rocket doesn't.
        let m = fly(&grenade, [0.0, 1.0, 0.0], 1.0);
        assert!((m.position[1] - 1750.0).abs() < 0.5);
        let drop = 1.5 * g * 1830.0 / 3600.0;
        assert!((m.position[2] + drop).abs() < 0.5, "{:?}", m.position);
        assert!(m.travelled > 1750.0);
        let r = fly(&rocket, [0.0, 1.0, 0.0], 1.0);
        assert!((r.position[1] - 1550.0).abs() < 0.5 && r.position[2] == 0.0);
        // Aimed as the AI aims (`009a7c60`'s low root), it comes down at
        // the target 1500 units off, within a frame's fall.
        let x = 1500.0;
        let pitch = crate::explosions::launch_pitch(x, 0.0, 1750.0, 1.5 * g, false);
        let dir = [0.0, pitch.cos(), pitch.sin()];
        let t = x / (1750.0 * pitch.cos());
        let m = fly(&grenade, dir, t);
        assert!((m.position[1] - x).abs() < 1.0);
        assert!(m.position[2].abs() < 15.0, "{:?}", m.position);
    }
}
