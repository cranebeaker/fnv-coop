//! Image space modifiers (`IMAD`): changes to the image space played over
//! time, which scripts apply (`ApplyImageSpaceModifier`): the white flash
//! and fade-in as the player wakes in Doc Mitchell's house, being hit,
//! drugs, weathers' looks.
//!
//! Read from the records (checked on `VCG01FadeInFromBlackISFX`):
//!
//! - `DNAM`: flags (0x01 animatable: plays over its duration), the
//!   duration in seconds (7), then how many keys each track has: a
//!   multiply and an add track for each of 21 image space values, then
//!   tint, blur, double vision, radial blur (strength, ramp up, start, a
//!   flag, the centre x and y as floats, ramp down, down start), depth of
//!   field (strength, a flag, distance, range), fade colour and motion
//!   blur. Each count matches the key count of the subrecord it describes.
//! - The tracks: subrecord `[n] "IAD"` multiplies value `n`, `[n + 0x40]
//!   "IAD"` adds to it; each key is (time, value), 8 bytes. Times run from
//!   0 to 1 over the duration (the fade-in's last keys are at 1.0 and its
//!   duration is 7 s).
//! - `TNAM` tint and `NAM3` fade colour: keys of (time, r, g, b, a), 20
//!   bytes; `BNAM` blur and `VNAM` double vision: (time, value).
//!
//! The 21 values are the image space's (`IMGS` `DNAM`) in its order: 0 eye
//! adapt speed, 1 blur radius, 2 blur passes, 3 emissive mult, 4 target
//! lum, 5 upper lum clamp, 6 bright scale, 7 bright clamp, 8–10 lum ramps,
//! 11–13 sunlight, grass and tree dimmers, 14–16 bloom, 17–20 cinematic.
//! The fade-in's tracks fit the first part: 6 is multiplied 2 → 3 → 1 and 7
//! by 0.5 → 0.3 → 1 (a bright, bloomy wake-up), and its fade colour is
//! white at full strength, clearing by 0.71 (the script's comment: "the
//! fade is from white now"). The Goodsprings recording (13:06, weather
//! `NVWastelandGS`, whose day modifier `NVWastelandIS` multiplies track 8
//! by 0.8, 11 by 1.1 and 20 by 1.3 at its first keys) confirmed three
//! tracks: 8 (LUM ramp no tex: the sky shaders' `Params.y` arrived as
//! 1.1 × 0.8 = 0.88), 11 (the sunlight dimmer: the sun's colour arrived ×
//! 1.1 × 1.1 = 1.21) and 20 (brightness: `Cinematic.w` arrived as 1.3 with
//! the record's 1.0). Tracks 14–16 (bloom) and 17–19 (saturation, contrast
//! average, contrast, in the image space's order) aren't confirmed.
//!
//! Guesses: values between keys are linear; before the first key it holds
//! the first, after the last the last; an animatable modifier ends after
//! its duration, one that isn't stays at its first keys until removed.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};

const IMAD: FourCC = FourCC::new(b"IMAD");
const DNAM: FourCC = FourCC::new(b"DNAM");
const TNAM: FourCC = FourCC::new(b"TNAM");
const NAM3: FourCC = FourCC::new(b"NAM3");
const BNAM: FourCC = FourCC::new(b"BNAM");
const VNAM: FourCC = FourCC::new(b"VNAM");

/// How many image space values have tracks.
pub const TRACKS: usize = 21;

/// Track numbers of the values the viewer's passes use.
pub mod track {
    /// The eye adaptation speed (the average pass's `HDRParam.z`).
    pub const EYE_ADAPT_SPEED: usize = 0;
    pub const BLUR_RADIUS: usize = 1;
    pub const TARGET_LUM: usize = 4;
    pub const UPPER_LUM_CLAMP: usize = 5;
    pub const BRIGHT_SCALE: usize = 6;
    pub const BRIGHT_CLAMP: usize = 7;
    pub const LUM_RAMP_NO_TEX: usize = 8;
    pub const SUNLIGHT_DIMMER: usize = 11;
    pub const GRASS_DIMMER: usize = 12;
    pub const SATURATION: usize = 17;
    pub const CONTRAST_AVERAGE: usize = 18;
    pub const CONTRAST: usize = 19;
    pub const BRIGHTNESS: usize = 20;
}

/// Keys of one value over time: (time 0..1, value).
pub type Keys = Vec<(f32, f32)>;
/// Keys of a colour: (time, [r, g, b, a]).
pub type ColorKeys = Vec<(f32, [f32; 4])>;

#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub animatable: bool,
    pub duration: f32,
    pub multiply: Vec<Keys>,
    pub add: Vec<Keys>,
    pub tint: ColorKeys,
    pub fade: ColorKeys,
    pub blur: Keys,
    pub double_vision: Keys,
}

/// A modifier's values at one moment.
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierValues {
    /// Each image space value becomes value × multiply + add.
    pub multiply: [f32; TRACKS],
    pub add: [f32; TRACKS],
    /// Tint colour and amount (alpha).
    pub tint: [f32; 4],
    /// The colour the picture fades to, and how far (alpha).
    pub fade: [f32; 4],
    pub blur: f32,
    pub double_vision: f32,
}

impl ModifierValues {
    /// A modifier that changes nothing (the game's blank one for a time of
    /// day whose weather names none).
    pub fn none() -> ModifierValues {
        ModifierValues {
            multiply: [1.0; TRACKS],
            add: [0.0; TRACKS],
            tint: [1.0, 1.0, 1.0, 0.0],
            fade: [0.0; 4],
            blur: 0.0,
            double_vision: 0.0,
        }
    }

    /// An image space value after these: `value × multiply + add` for its
    /// track.
    pub fn apply(&self, track: usize, value: f32) -> f32 {
        match (self.multiply.get(track), self.add.get(track)) {
            (Some(m), Some(a)) => value * m + a,
            _ => value,
        }
    }

    /// These and `other` mixed: `f` of the way toward `other`. Multiplies
    /// and adds are averaged (`v × Σ s·mult + Σ s·add`, as the game applies
    /// a weather's modifiers: `00b8cc20`); tints and fades by their amounts
    /// (the amount averaged, the colour weighted by each one's amount).
    pub fn blend(&self, other: &ModifierValues, f: f32) -> ModifierValues {
        let mix = |a: f32, b: f32| a + (b - a) * f;
        let weighted = |a: [f32; 4], b: [f32; 4]| {
            let (wa, wb) = (a[3] * (1.0 - f), b[3] * f);
            let amount = wa + wb;
            let rgb: [f32; 3] = std::array::from_fn(|i| {
                if amount > 0.0 {
                    (a[i] * wa + b[i] * wb) / amount
                } else {
                    mix(a[i], b[i])
                }
            });
            [rgb[0], rgb[1], rgb[2], amount]
        };
        ModifierValues {
            multiply: std::array::from_fn(|i| mix(self.multiply[i], other.multiply[i])),
            add: std::array::from_fn(|i| mix(self.add[i], other.add[i])),
            tint: weighted(self.tint, other.tint),
            fade: weighted(self.fade, other.fade),
            blur: mix(self.blur, other.blur),
            double_vision: mix(self.double_vision, other.double_vision),
        }
    }
}

fn keys(data: &[u8]) -> Keys {
    data.chunks_exact(8)
        .map(|c| (le_f32(c, 0), le_f32(c, 4)))
        .collect()
}

fn color_keys(data: &[u8]) -> ColorKeys {
    data.chunks_exact(20)
        .map(|c| {
            (
                le_f32(c, 0),
                [le_f32(c, 4), le_f32(c, 8), le_f32(c, 12), le_f32(c, 16)],
            )
        })
        .collect()
}

/// A track's value at `t` (0..1): linear between keys, held past the ends.
pub fn sample(keys: &[(f32, f32)], t: f32, default: f32) -> f32 {
    let Some(first) = keys.first() else {
        return default;
    };
    if t <= first.0 {
        return first.1;
    }
    for w in keys.windows(2) {
        let ((t0, v0), (t1, v1)) = (w[0], w[1]);
        if t <= t1 {
            let f = if t1 > t0 { (t - t0) / (t1 - t0) } else { 1.0 };
            return v0 + (v1 - v0) * f;
        }
    }
    keys.last().map_or(default, |k| k.1)
}

fn sample_color(keys: &ColorKeys, t: f32, default: [f32; 4]) -> [f32; 4] {
    let channel = |i: usize| {
        let k: Keys = keys.iter().map(|(t, c)| (*t, c[i])).collect();
        sample(&k, t, default[i])
    };
    [channel(0), channel(1), channel(2), channel(3)]
}

impl Modifier {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Modifier> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == IMAD)?;
        let record = rr.record().ok()?;
        let dnam = record.get(DNAM).filter(|s| s.data.len() >= 8)?;
        let mut multiply = vec![Keys::new(); TRACKS];
        let mut add = vec![Keys::new(); TRACKS];
        for sub in &record.subrecords {
            let k = sub.kind.as_bytes();
            if &k[1..] != b"IAD" {
                continue;
            }
            let n = usize::from(k[0] & 0x3F);
            let list = if k[0] & 0x40 != 0 {
                &mut add
            } else {
                &mut multiply
            };
            if let Some(slot) = list.get_mut(n) {
                *slot = keys(&sub.data);
            }
        }
        let get = |kind| record.get(kind).map(|s| s.data.as_slice()).unwrap_or(&[]);
        Some(Modifier {
            form_id: id,
            editor_id: record.editor_id(),
            animatable: le_u32(&dnam.data, 0) & 1 != 0,
            duration: le_f32(&dnam.data, 4),
            multiply,
            add,
            tint: color_keys(get(TNAM)),
            fade: color_keys(get(NAM3)),
            blur: keys(get(BNAM)),
            double_vision: keys(get(VNAM)),
        })
    }

    /// Where in its keys it is `age` seconds after being applied (0..1).
    fn time(&self, age: f32) -> f32 {
        if !self.animatable || self.duration <= 0.0 {
            return 0.0;
        }
        (age / self.duration).clamp(0.0, 1.0)
    }

    /// Whether it has played out.
    pub fn finished(&self, age: f32) -> bool {
        self.animatable && age >= self.duration
    }

    /// Its values `age` seconds after it was applied.
    pub fn at(&self, age: f32) -> ModifierValues {
        let t = self.time(age);
        ModifierValues {
            multiply: std::array::from_fn(|i| sample(&self.multiply[i], t, 1.0)),
            add: std::array::from_fn(|i| sample(&self.add[i], t, 0.0)),
            tint: sample_color(&self.tint, t, [1.0, 1.0, 1.0, 0.0]),
            fade: sample_color(&self.fade, t, [0.0; 4]),
            blur: sample(&self.blur, t, 0.0),
            double_vision: sample(&self.double_vision, t, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_are_linear_between_keys_and_held_past_them() {
        let k = vec![(0.0, 2.0), (0.29, 3.0), (1.0, 1.0)];
        assert_eq!(sample(&k, -1.0, 9.0), 2.0);
        assert!((sample(&k, 0.145, 9.0) - 2.5).abs() < 1e-6);
        assert_eq!(sample(&k, 2.0, 9.0), 1.0);
        assert_eq!(sample(&[], 0.5, 9.0), 9.0);
    }
}
