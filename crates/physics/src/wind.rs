//! The wind on Havok bodies: what rolls the tumbleweeds.
//!
//! The game's exterior Havok world (`TESObjectCELL::InitHavok`, Xbox PDB;
//! `00554010`) carries a `TESWindListener` (`00554590`), an entity listener
//! that keeps every body added to the world whose `bhkWorldObject` flags
//! have `WIND` (bit 0, the NIF's body flags word,
//! `nif::collision::BODY_WIND`; `00c9a430`, `00c9a600`). Interior worlds
//! (`00552dc0`) have none.
//!
//! Every frame the main loop hands the sky's wind to it (`00453550` →
//! `bhkWindListener::SetWind`, Xbox PDB, `00c74550`): the speed the
//! weather's wind byte gives (`DATA` 0 / 255, blended across a weather
//! change by the sky, `0063c490`, `world::weather`) × [`MAX_WIND`], and the
//! sky's wind direction (`Sky` `+0xd0`, 1 radian: the constructor's value,
//! `00639d40`; nothing else in the sky's code writes it). In an interior
//! both are 0. After the frame's Havok steps the world calls the listener
//! once with the frame's time (`00c6ae70` → `00c66e20`), and it pushes each
//! of its bodies ([`push`]).

use crate::Vec3;

/// `bhkWindListener::fMaxWind` (Xbox PDB), `011b02dc`: 250 (Havok force
/// units).
pub const MAX_WIND: f32 = 250.0;

/// The listener's two statics (`bhkWindListener::fWindSpeed`, `fWindDir`,
/// Xbox PDB; `01267c14`, `01267c18`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Wind {
    /// Havok force units (0–[`MAX_WIND`]).
    pub speed: f32,
    /// Radians about z.
    pub direction: f32,
}

/// The sky's wind direction (`Sky` `+0xd0`): set to 1 in its constructor
/// (`00639d40`) and not changed by the sky.
pub const SKY_WIND_DIRECTION: f32 = 1.0;

impl Wind {
    /// `bhkWindListener::SetWind` (Xbox PDB): the sky's wind speed (0–1)
    /// and direction.
    // Translated from 00c74550 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn set(sky_speed: f32, sky_direction: f32) -> Wind {
        Wind {
            speed: MAX_WIND * sky_speed,
            direction: sky_direction,
        }
    }
}

/// The force the wind puts on one body this frame (Havok force units, in
/// the world's axes), as `bhkWindListener::Update` (Xbox PDB) works it
/// out; `None` when there's no wind. `random(x)` is the listener's
/// `GetRandom` (`TESWindListener`, `0062e910`): a number between −x and x
/// from the game's dice (`00476b70`), drawn twice per body.
///
/// The frame counts as `1 + ⌊dt ÷ 0.0167⌋` sixtieths; the speed is a
/// quarter of the wind's plus a random part of up to the wind's own either
/// way, kept within 0–[`MAX_WIND`], times that count; its heading the wind's
/// direction plus up to a quarter turn either way (`π/4`), kept within
/// 0–2π; the force is (0, speed, 0) turned by that heading about z.
/// The listener then wakes the body (`00c9c1d0`) and applies the force for
/// the frame's time (`hkpMotion::applyForce(dt, force)`, Xbox PDB slot
/// `+0x5c`): [`crate::rigid::RigidWorld::apply_force`].
// Translated from 00c74570 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn push(wind: &Wind, dt: f32, mut random: impl FnMut(f32) -> f32) -> Option<Vec3> {
    if wind.speed == 0.0 {
        return None;
    }
    // `1 - ftol(dt / -0.0167)`.
    let count = (1 - (f64::from(dt) / -0.0167) as i32) as f32;
    let speed = (random(wind.speed) + (f64::from(wind.speed) * 0.25) as f32).clamp(0.0, MAX_WIND);
    let speed = speed * count;
    let mut heading = wind.direction + random(std::f32::consts::FRAC_PI_4);
    let turn = std::f32::consts::TAU;
    if heading > turn {
        heading -= turn;
    } else if heading < 0.0 {
        heading += turn;
    }
    let (s, c) = heading.sin_cos();
    Some([-s * speed, c * speed, 0.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wind_pushes_along_its_heading_and_scales_with_the_frame() {
        // The sky's wind at 0.4: 100 units; no random part.
        let w = Wind::set(0.4, SKY_WIND_DIRECTION);
        assert_eq!(w.speed, 100.0);
        let f = push(&w, 1.0 / 60.0, |_| 0.0).unwrap();
        // A quarter, 25, along the heading 1 radian from +y toward −x; a
        // sixtieth counts once.
        assert!((f[0] + 25.0 * 1f32.sin()).abs() < 1e-3, "{f:?}");
        assert!((f[1] - 25.0 * 1f32.cos()).abs() < 1e-3, "{f:?}");
        assert_eq!(f[2], 0.0);
        // A frame of two sixtieths and a bit counts three times.
        let f3 = push(&w, 0.034, |_| 0.0).unwrap();
        let len = |v: Vec3| (v[0] * v[0] + v[1] * v[1]).sqrt();
        assert!((len(f3) - 75.0).abs() < 1e-2, "{f3:?}");
        // The random part can take it below nothing: held at 0.
        let calm = push(&w, 1.0 / 60.0, |x| -x).unwrap();
        assert_eq!(len(calm), 0.0);
        // The speed is held to the most before the count.
        let strong = Wind::set(1.0, 0.0);
        let f = push(&strong, 1.0 / 60.0, |x| x).unwrap();
        assert!((len(f) - MAX_WIND).abs() < 1e-3, "{f:?}");
        // No wind, no push.
        assert!(push(&Wind::default(), 0.016, |x| x).is_none());
    }

    #[test]
    fn the_heading_wraps_into_one_turn() {
        let w = Wind {
            speed: 10.0,
            direction: 6.2,
        };
        // + π/4 goes past 2π and wraps to about 0.5.
        let f = push(&w, 0.016, |x| x).unwrap();
        let heading = (-f[0]).atan2(f[1]);
        let want = 6.2 + std::f32::consts::FRAC_PI_4 - std::f32::consts::TAU;
        assert!((heading - want).abs() < 1e-3, "{heading} {want}");
    }
}
