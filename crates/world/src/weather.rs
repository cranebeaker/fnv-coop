//! Outdoor light: weathers (`WTHR`), climates (`CLMT`) and how an exterior
//! cell picks its weather.
//!
//! A weather stores ten colours for each of six times of day (`NAM0`), fog
//! distances (`FNAM`) and image space modifiers. A climate lists the
//! weathers a worldspace can have and when the sun rises and sets; a
//! region (`REGN`) can list its own.

use esm::{FormId, FourCC, LoadOrder, RecordRef};

use crate::cell::{le_f32, le_u32, CellInfo, Lighting};

const WTHR: FourCC = FourCC::new(b"WTHR");
const CLMT: FourCC = FourCC::new(b"CLMT");
const REGN: FourCC = FourCC::new(b"REGN");
const NAM0: FourCC = FourCC::new(b"NAM0");
const FNAM: FourCC = FourCC::new(b"FNAM");
const WLST: FourCC = FourCC::new(b"WLST");
const TNAM: FourCC = FourCC::new(b"TNAM");
const RDWT: FourCC = FourCC::new(b"RDWT");
const XCLR: FourCC = FourCC::new(b"XCLR");

/// The ten colours a weather gives for each time of day, in `NAM0` order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkyColor {
    SkyUpper = 0,
    Fog = 1,
    CloudsLower = 2,
    Ambient = 3,
    Sunlight = 4,
    Sun = 5,
    Stars = 6,
    SkyLower = 7,
    Horizon = 8,
    CloudsUpper = 9,
}

/// The six times of day a weather has colours for, in `NAM0` order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeOfDay {
    Sunrise = 0,
    Day = 1,
    Sunset = 2,
    Night = 3,
    HighNoon = 4,
    Midnight = 5,
}

/// A weather's colours and fog.
#[derive(Debug, Clone, PartialEq)]
pub struct Weather {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `NAM0`: RGBA bytes, ten colours (see [`SkyColor`]) each for every
    /// time of day (see [`TimeOfDay`]).
    colors: Vec<u8>,
    /// `FNAM`: fog near and far by day, then by night, then the fog power
    /// by day and by night.
    pub fog_day: (f32, f32),
    pub fog_night: (f32, f32),
    pub fog_power_day: f32,
    pub fog_power_night: f32,
    /// Cloud layers 0–3: textures (`DNAM`, `CNAM`, `ANAM`, `BNAM`, relative
    /// to `textures\`; `sky\alpha.dds` is an empty layer), speeds (`ONAM`,
    /// a byte each) and colours (`PNAM`: per layer, the six times of day,
    /// RGB and an unused byte; Goodsprings' `NVWastelandGS` layer 3,
    /// `sky\NVCloudlight.dds`, is (232, 235, 238) by day).
    pub cloud_textures: [Option<String>; 4],
    pub cloud_speeds: [u8; 4],
    cloud_colors: Vec<u8>,
    /// Image space modifiers (`IMAD`) for each time of day, in
    /// [`TimeOfDay`] order: subrecord `[n] "IAD"` names time `n`'s.
    /// `NVWastelandGS`: `NVWastelandSunriseIS`, `NVWastelandIS`,
    /// `NVWastelandSunsetIS`, `NVWastelandNightIS`, and the day's again
    /// for high noon. The night's tints the picture blue (55, 119, 236) at
    /// 0.63: that, not the weather's colours, is what makes nights dark.
    pub image_spaces: [Option<FormId>; 6],
    /// `DATA` byte 0, the wind speed (/255: cloud layers move by it), and
    /// byte 4, the sun's glare (/255: the glare's strength).
    pub wind: u8,
    pub sun_glare: u8,
}

impl Weather {
    /// A cloud layer's colour at a time of day.
    pub fn cloud_color(&self, layer: usize, time: TimeOfDay) -> Option<[u8; 3]> {
        let at = (layer * 6 + time as usize) * 4;
        let c = self.cloud_colors.get(at..at + 3)?;
        Some([c[0], c[1], c[2]])
    }

    pub fn load(order: &LoadOrder, id: FormId) -> Option<Weather> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == WTHR)?;
        let record = rr.record().ok()?;
        let colors = record.get(NAM0)?.data.clone();
        let fog = record.get(FNAM).filter(|s| s.data.len() >= 16);
        let f = |i: usize, default: f32| {
            fog.filter(|s| s.data.len() >= i * 4 + 4)
                .map_or(default, |s| le_f32(&s.data, i * 4))
        };
        let texture = |kind: &[u8; 4]| {
            record
                .get(FourCC::new(kind))
                .map(|s| s.zstring())
                .filter(|t| !t.is_empty())
        };
        let speeds = record
            .get(FourCC::new(b"ONAM"))
            .map(|s| s.data.clone())
            .unwrap_or_default();
        let data = record.get(esm::sig::DATA).map(|s| s.data.clone());
        let data_byte = |i: usize| data.as_ref().and_then(|d| d.get(i).copied()).unwrap_or(0);
        let mut image_spaces = [None; 6];
        for sub in &record.subrecords {
            let k = sub.kind.as_bytes();
            if &k[1..] == b"IAD" && sub.data.len() >= 4 {
                if let Some(slot) = image_spaces.get_mut(usize::from(k[0])) {
                    *slot = Some(rr.plugin.to_global(FormId(le_u32(&sub.data, 0))))
                        .filter(|f| f.0 != 0);
                }
            }
        }
        Some(Weather {
            form_id: id,
            editor_id: record.editor_id(),
            colors,
            fog_day: (f(0, 0.0), f(1, 0.0)),
            fog_night: (f(2, 0.0), f(3, 0.0)),
            fog_power_day: f(4, 1.0),
            fog_power_night: f(5, 1.0),
            cloud_textures: [
                texture(b"DNAM"),
                texture(b"CNAM"),
                texture(b"ANAM"),
                texture(b"BNAM"),
            ],
            cloud_speeds: [0, 1, 2, 3].map(|i| speeds.get(i).copied().unwrap_or(0)),
            cloud_colors: record
                .get(FourCC::new(b"PNAM"))
                .map(|s| s.data.clone())
                .unwrap_or_default(),
            image_spaces,
            wind: data_byte(0),
            sun_glare: data_byte(4),
        })
    }

    /// One of the weather's colours at a time of day.
    pub fn color(&self, which: SkyColor, time: TimeOfDay) -> Option<[u8; 3]> {
        let times = self.colors.len() / 40;
        if (time as usize) >= times {
            return None;
        }
        let at = ((which as usize) * times + time as usize) * 4;
        Some([self.colors[at], self.colors[at + 1], self.colors[at + 2]])
    }
}

/// A climate: its weathers and the sun's times.
#[derive(Debug, Clone, PartialEq)]
pub struct Climate {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// Weathers with their chances.
    pub weathers: Vec<(FormId, i32)>,
    /// Sunrise begins and ends, sunset begins and ends, in hours.
    pub sunrise: (f32, f32),
    pub sunset: (f32, f32),
    /// The sun's texture and its glare (`FNAM`, `GNAM`, relative to
    /// `textures\`; `NVDefaultClimate`: `Sky\Sun.dds`,
    /// `sky\NV_sunglare.dds`) and the night sky's model (`MODL`,
    /// `Sky\Stars.nif`).
    pub sun: Option<String>,
    pub sun_glare: Option<String>,
    pub stars: Option<String>,
}

impl Climate {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Climate> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == CLMT)?;
        let record = rr.record().ok()?;
        let weathers = record
            .get(WLST)
            .map(|s| weather_list(&rr, &s.data))
            .unwrap_or_default();
        // `TNAM`: four times in units of 10 minutes.
        let times = record
            .get(TNAM)
            .filter(|s| s.data.len() >= 4)
            .map(|s| [0, 1, 2, 3].map(|i| f32::from(s.data[i]) / 6.0));
        let [rise_begin, rise_end, set_begin, set_end] = times.unwrap_or([6.0, 8.0, 18.0, 20.0]);
        let text = |kind: &[u8; 4]| {
            record
                .get(FourCC::new(kind))
                .map(|s| s.zstring())
                .filter(|t| !t.is_empty())
        };
        Some(Climate {
            form_id: id,
            editor_id: record.editor_id(),
            weathers,
            sunrise: (rise_begin, rise_end),
            sunset: (set_begin, set_end),
            sun: text(b"FNAM"),
            sun_glare: text(b"GNAM"),
            stars: text(b"MODL"),
        })
    }
}

/// `WLST` / `RDWT`: weather, chance, then a global (ignored) per entry.
fn weather_list(rr: &RecordRef<'_>, data: &[u8]) -> Vec<(FormId, i32)> {
    data.chunks_exact(12)
        .map(|e| {
            (
                rr.plugin.to_global(FormId(le_u32(e, 0))),
                le_u32(e, 4) as i32,
            )
        })
        .filter(|(w, _)| w.0 != 0)
        .collect()
}

/// The likeliest weather of a list (the first of equals).
fn likeliest(list: &[(FormId, i32)]) -> Option<FormId> {
    let mut best: Option<(FormId, i32)> = None;
    for &(w, chance) in list {
        if best.map_or(true, |(_, c)| chance > c) {
            best = Some((w, chance));
        }
    }
    best.map(|(w, _)| w)
}

/// The weather an exterior cell gets: the likeliest weather of the first of
/// its regions (`XCLR`) that lists weathers, else of the climate.
///
/// **A guess at the game's choice**: the game rolls among the weathers by
/// their chances (and globals can switch some off); taking the likeliest
/// keeps every load the same.
pub fn cell_weather(order: &LoadOrder, cell: FormId, climate: Option<&Climate>) -> Option<FormId> {
    let rr = order.get(cell)?;
    let record = rr.record().ok()?;
    if let Some(regions) = record.get(XCLR) {
        for chunk in regions.data.chunks_exact(4) {
            let region = rr.plugin.to_global(FormId(le_u32(chunk, 0)));
            let Some(rrr) = order.get(region).filter(|r| r.entry.header.kind == REGN) else {
                continue;
            };
            let Ok(rec) = rrr.record() else { continue };
            // A region's weather data: `RDWT` after its `RDAT` header.
            let list: Vec<(FormId, i32)> = rec
                .get_all(RDWT)
                .flat_map(|s| weather_list(&rrr, &s.data))
                .collect();
            if let Some(w) = likeliest(&list) {
                return Some(w);
            }
        }
    }
    climate.and_then(|c| likeliest(&c.weathers))
}

/// The hour the outdoors are shown at when no clock is given (the CPU
/// renderer; the viewer follows the game's `GameHour`).
pub const DEFAULT_HOUR: f32 = 10.0;

/// The game settings the sky's clock uses (`GMST`s, else the engine's own
/// defaults), as `FalloutNV.exe`'s sky code reads them (see
/// `%USERPROFILE%\nv-re\findings\sky.md`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkySettings {
    /// `fDaytimeColorExtension` (0.5 h, not in `FalloutNV.esm`): the
    /// colours' sunrise starts this much before the climate's, their sunset
    /// ends this much after.
    pub color_extension: f32,
    /// `fSunXExtreme`, `fSunYExtreme`, `fSunZExtreme` (`FalloutNV.esm`:
    /// 800, −100, −100; engine defaults −400, 25, −100).
    pub sun_extreme: [f32; 3],
    /// `fSunAlphaTransTime` (2 h): the sun's path runs from half of it
    /// before the middle of sunrise to half after the middle of sunset, and
    /// it fades in and out over it.
    pub sun_alpha_time: f32,
    /// `fWeatherCloudSpeedMax` (0.1): cloud layers move `wind × this ×
    /// layer speed / 255` texture lengths a second.
    pub cloud_speed_max: f32,
}

impl Default for SkySettings {
    fn default() -> Self {
        SkySettings {
            color_extension: 0.5,
            sun_extreme: [-400.0, 25.0, -100.0],
            sun_alpha_time: 2.0,
            cloud_speed_max: 0.1,
        }
    }
}

impl SkySettings {
    /// The settings from the load order's `GMST`s, the engine's defaults
    /// where there are none.
    pub fn load(order: &LoadOrder) -> Self {
        let d = SkySettings::default();
        let g = |name: &str, default: f32| {
            crate::scripting::game_setting(order, name).unwrap_or(default)
        };
        SkySettings {
            color_extension: g("fDaytimeColorExtension", d.color_extension),
            sun_extreme: [
                g("fSunXExtreme", d.sun_extreme[0]),
                g("fSunYExtreme", d.sun_extreme[1]),
                g("fSunZExtreme", d.sun_extreme[2]),
            ],
            sun_alpha_time: g("fSunAlphaTransTime", d.sun_alpha_time),
            cloud_speed_max: g("fWeatherCloudSpeedMax", d.cloud_speed_max),
        }
    }
}

/// A climate's times with the sky's settings: what the hour's colours,
/// fog, sun and stars follow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkyClock {
    /// The climate's sunrise begin and end, sunset begin and end (hours).
    pub sunrise: (f32, f32),
    pub sunset: (f32, f32),
    pub settings: SkySettings,
}

/// The game's noon, fixed (the sky object's +0x12c: 12.0, never changed).
const NOON: f32 = 12.0;

impl SkyClock {
    pub fn new(climate: Option<&Climate>, settings: SkySettings) -> Self {
        let (sunrise, sunset) =
            climate.map_or(((6.0, 8.0), (18.0, 20.0)), |c| (c.sunrise, c.sunset));
        SkyClock {
            sunrise,
            sunset,
            settings,
        }
    }

    /// The colours' windows (`0063b9b0`, `0063ba30`): sunrise from its
    /// begin less the extension to its end, sunset from its begin to its
    /// end plus the extension (New Vegas: 5:30–8:00, 18:00–20:30).
    fn windows(&self) -> (f32, f32, f32, f32) {
        let e = self.settings.color_extension;
        (
            (self.sunrise.0 - e).max(0.0),
            self.sunrise.1,
            self.sunset.0,
            (self.sunset.1 + e).min(23.99),
        )
    }
}

/// How much each time of day's colours count at `hour`: two times and
/// their shares, adding up to 1 (`0063b630`): night → sunrise (full at the
/// middle of the widened sunrise, 6:45) → day (at sunrise's end) → high
/// noon (full at 12:00) → day (at sunset's begin) → sunset (full at the
/// middle of the widened sunset, 19:15) → night (20:30), each step linear.
/// Midnight is never used. Confirmed by the Goodsprings recording (13:06):
/// the ambient, the sky's three colours, the sun's colour and every cloud
/// layer's colour all arrived as the day's and high noon's blended 0.183 /
/// 0.817 (`NVWastelandGS`'s upper sky (50, 71, 135) and (72, 91, 159) gave
/// `BlendColor[2]` (0.26657, 0.34251, 0.60631)).
pub fn time_weights(clock: &SkyClock, hour: f32) -> [(TimeOfDay, f32); 2] {
    use TimeOfDay::*;
    let (sr0, sr1, ss0, ss1) = clock.windows();
    let t = hour.rem_euclid(24.0);
    if t > sr0 && t < sr1 {
        let half = (sr1 - sr0) / 2.0;
        let mid = sr0 + half;
        let w = 1.0 - (t - mid).abs() / half;
        [(Sunrise, w), (if t < mid { Night } else { Day }, 1.0 - w)]
    } else if (sr1..=NOON).contains(&t) {
        let w = if NOON > sr1 {
            1.0 - (NOON - t) / (NOON - sr1)
        } else {
            1.0
        };
        [(HighNoon, w), (Day, 1.0 - w)]
    } else if (NOON..=ss0).contains(&t) {
        let w = if ss0 > NOON {
            1.0 - (ss0 - t) / (ss0 - NOON)
        } else {
            1.0
        };
        [(Day, w), (HighNoon, 1.0 - w)]
    } else if t > ss0 && t < ss1 {
        let half = (ss1 - ss0) / 2.0;
        let mid = ss0 + half;
        let w = 1.0 - (t - mid).abs() / half;
        [(Sunset, w), (if t < mid { Day } else { Night }, 1.0 - w)]
    } else {
        [(Night, 1.0), (Night, 0.0)]
    }
}

/// The weather image space modifiers' shares at `hour` (`0063ef20`): as
/// [`time_weights`], except that from sunrise's end to sunset's begin the
/// day's and high noon's are swapped (the day's modifier full at noon,
/// high noon's at the ends), as the game's code does it.
pub fn modifier_weights(clock: &SkyClock, hour: f32) -> [(TimeOfDay, f32); 2] {
    let mut w = time_weights(clock, hour);
    let (_, sr1, ss0, _) = clock.windows();
    if (sr1..=ss0).contains(&hour.rem_euclid(24.0)) {
        for (time, _) in &mut w {
            *time = match *time {
                TimeOfDay::Day => TimeOfDay::HighNoon,
                TimeOfDay::HighNoon => TimeOfDay::Day,
                other => other,
            };
        }
    }
    w
}

/// How much of the day's fog (rather than the night's) applies
/// (`0063bce0`): 0 → 1 across the widened sunrise, 1 by day, 1 → 0 across
/// the widened sunset, 0 at night.
pub fn daylight(clock: &SkyClock, hour: f32) -> f32 {
    let (sr0, sr1, ss0, ss1) = clock.windows();
    let t = hour.rem_euclid(24.0);
    if t > sr0 && t < sr1 {
        (t - sr0) / (sr1 - sr0)
    } else if (sr1..=ss0).contains(&t) {
        1.0
    } else if t > ss0 && t < ss1 {
        (ss1 - t) / (ss1 - ss0)
    } else {
        0.0
    }
}

/// The stars' alpha (`00640020`): 1 → 0 from the widened sunrise's start
/// to its middle, 0 by day, 0 → 1 from the widened sunset's middle to its
/// end, 1 at night.
pub fn stars_alpha(clock: &SkyClock, hour: f32) -> f32 {
    let (sr0, sr1, ss0, ss1) = clock.windows();
    let (mr, ms) = (sr1 - (sr1 - sr0) / 2.0, ss1 - (ss1 - ss0) / 2.0);
    let t = hour.rem_euclid(24.0);
    if t > sr0 && t < mr {
        (mr - t) / (mr - sr0)
    } else if (mr..=ms).contains(&t) {
        0.0
    } else if t > ms && t < ss1 {
        (t - ms) / (ss1 - ms)
    } else {
        1.0
    }
}

/// The sun at an hour (`00641830`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SunNow {
    /// Unit vector toward the light (by night the night's light).
    pub toward_light: [f32; 3],
    /// Where the disc and glare stand from the eye, in sky units.
    pub disc: [f32; 3],
    /// The disc's visibility 0..1.
    pub visibility: f32,
}

/// The sun's path (`00641830`, read; the climate's own times, no
/// extension): from `D0` = the middle of sunrise less half of
/// `fSunAlphaTransTime` to `D1` = the middle of sunset plus half (New
/// Vegas: 6:00 to 20:00), `x = X(1 − 2f)` as `f` goes 0 → 1; by night `x`
/// goes back from −X to X. The disc stands at `(x, Y, |X| − |x|)`; the
/// light comes from `normalize(x, −Y, −Z)` (New Vegas `(x, 100, 100)`: it
/// rises due east, is highest, 45°, at 13:00 from the north, and sets due
/// west). The disc fades in from `D0` to two hours later and out over the
/// two hours before `D1`. Confirmed by the Goodsprings recording: at 13:06
/// every lit shader got `PSLightDir` (−0.07881, 0.70491, 0.70491) =
/// `normalize(−11.18, 100, 100)`, the same hour the sky's colour blend
/// gives (day 0.183, high noon 0.817).
pub fn sun_at(clock: &SkyClock, hour: f32) -> SunNow {
    let s = &clock.settings;
    let [ex, ey, ez] = s.sun_extreme;
    let rise = (clock.sunrise.0 + clock.sunrise.1) / 2.0;
    let set = (clock.sunset.0 + clock.sunset.1) / 2.0;
    let h = s.sun_alpha_time * 0.5;
    let (d0, d1) = (rise - h, set + h);
    let t = hour.rem_euclid(24.0);
    let night_length = (24.0 - (d1 - d0)).max(1e-3);
    let x = if t > d0 && t < d1 {
        ex * (1.0 - 2.0 * (t - d0) / (d1 - d0))
    } else if t >= d1 {
        ex * (2.0 * (t - d1) / night_length - 1.0)
    } else {
        ex * (2.0 * (24.0 - d1 + t) / night_length - 1.0)
    };
    let toward = [x, -ey, -ez];
    let len = toward.iter().map(|c| c * c).sum::<f32>().sqrt().max(1e-6);
    let visibility = if t <= d0 || t >= d1 {
        0.0
    } else if t < rise + h {
        (t - d0) / (rise + h - d0)
    } else if t <= set - h {
        1.0
    } else {
        (d1 - t) / (d1 - (set - h))
    };
    SunNow {
        toward_light: toward.map(|c| c / len),
        disc: [x, ey, ex.abs() - x.abs()],
        visibility: visibility.clamp(0.0, 1.0),
    }
}

/// Unit vector toward the sun (or the night's light) at an hour (see
/// [`sun_at`]).
pub fn toward_sun(clock: &SkyClock, hour: f32) -> [f32; 3] {
    sun_at(clock, hour).toward_light
}

impl Weather {
    /// One of the weather's colours at an hour (0–255 per channel), blended
    /// as [`time_weights`] says.
    pub fn color_at(&self, which: SkyColor, clock: &SkyClock, hour: f32) -> [f32; 3] {
        let mut out = [0.0; 3];
        for (time, w) in time_weights(clock, hour) {
            let c = self
                .color(which, time)
                .or_else(|| self.color(which, TimeOfDay::Day))
                .unwrap_or([128; 3]);
            for k in 0..3 {
                out[k] += w * f32::from(c[k]);
            }
        }
        out
    }

    /// A cloud layer's colour at an hour (0–255 per channel), blended the
    /// same way (`0063bb70`).
    pub fn cloud_color_at(&self, layer: usize, clock: &SkyClock, hour: f32) -> [f32; 3] {
        let mut out = [0.0; 3];
        for (time, w) in time_weights(clock, hour) {
            let c = self
                .cloud_color(layer, time)
                .or_else(|| self.cloud_color(layer, TimeOfDay::Day))
                .unwrap_or([255; 3]);
            for k in 0..3 {
                out[k] += w * f32::from(c[k]);
            }
        }
        out
    }

    /// The fog's near and far distances and power at an hour: the day's and
    /// the night's blended by [`daylight`].
    pub fn fog_at(&self, clock: &SkyClock, hour: f32) -> (f32, f32, f32) {
        let d = daylight(clock, hour);
        let mix = |night: f32, day: f32| night + (day - night) * d;
        (
            mix(self.fog_night.0, self.fog_day.0),
            mix(self.fog_night.1, self.fog_day.1),
            mix(self.fog_power_night, self.fog_power_day),
        )
    }

    /// The sky's three colours (upper, horizon, lower; 0–255) at an hour.
    pub fn sky_at(&self, clock: &SkyClock, hour: f32) -> [[f32; 3]; 3] {
        [SkyColor::SkyUpper, SkyColor::Horizon, SkyColor::SkyLower]
            .map(|c| self.color_at(c, clock, hour))
    }

    /// The weather's image space modifier values at an hour: the two
    /// times' modifiers ([`modifier_weights`]) at their first keys (they
    /// aren't animatable), averaged by their shares, as the game applies
    /// them (`00b8cc20`: `v × Σ s·mult + Σ s·add`). `None` when the weather
    /// names none.
    pub fn modifier_at(
        &self,
        order: &LoadOrder,
        clock: &SkyClock,
        hour: f32,
    ) -> Option<crate::modifier::ModifierValues> {
        if self.image_spaces.iter().all(Option::is_none) {
            return None;
        }
        let [(a, wa), (b, _)] = modifier_weights(clock, hour);
        // A time whose weather names none gets a blank one at its share.
        let values = |t: TimeOfDay| {
            self.image_spaces[t as usize]
                .and_then(|m| crate::modifier::Modifier::load(order, m))
                .map_or_else(crate::modifier::ModifierValues::none, |m| m.at(0.0))
        };
        Some(values(a).blend(&values(b), 1.0 - wa))
    }
}

/// The weather being shown: the current one, and while a change fades the
/// previous one. Every value blends linearly, the current at the fade's
/// share `f` and the previous at `1 − f` (`findings\weather.md` §3); a
/// cloud colour that is exactly black in one takes the other's.
#[derive(Debug, Clone, Copy)]
pub struct WeatherMix<'a> {
    pub current: &'a Weather,
    pub previous: Option<&'a Weather>,
    pub fade: f32,
}

impl<'a> WeatherMix<'a> {
    /// One weather alone.
    pub fn single(weather: &'a Weather) -> Self {
        WeatherMix {
            current: weather,
            previous: None,
            fade: 1.0,
        }
    }

    fn mix(&self, value: impl Fn(&Weather) -> f32) -> f32 {
        let c = value(self.current);
        match self.previous {
            Some(p) => value(p) + (c - value(p)) * self.fade,
            None => c,
        }
    }

    fn mix3(&self, value: impl Fn(&Weather) -> [f32; 3]) -> [f32; 3] {
        let c = value(self.current);
        match self.previous {
            Some(p) => {
                let p = value(p);
                [0, 1, 2].map(|k| p[k] + (c[k] - p[k]) * self.fade)
            }
            None => c,
        }
    }

    pub fn color_at(&self, which: SkyColor, clock: &SkyClock, hour: f32) -> [f32; 3] {
        self.mix3(|w| w.color_at(which, clock, hour))
    }

    pub fn cloud_color_at(&self, layer: usize, clock: &SkyClock, hour: f32) -> [f32; 3] {
        let c = self.current.cloud_color_at(layer, clock, hour);
        let Some(p) = self.previous else {
            return c;
        };
        let p = p.cloud_color_at(layer, clock, hour);
        let black = |v: [f32; 3]| v == [0.0; 3];
        let (c, p) = match (black(c), black(p)) {
            (true, false) => (p, p),
            (false, true) => (c, c),
            _ => (c, p),
        };
        [0, 1, 2].map(|k| p[k] + (c[k] - p[k]) * self.fade)
    }

    pub fn fog_at(&self, clock: &SkyClock, hour: f32) -> (f32, f32, f32) {
        (
            self.mix(|w| w.fog_at(clock, hour).0),
            self.mix(|w| w.fog_at(clock, hour).1),
            self.mix(|w| w.fog_at(clock, hour).2),
        )
    }

    pub fn sky_at(&self, clock: &SkyClock, hour: f32) -> [[f32; 3]; 3] {
        [SkyColor::SkyUpper, SkyColor::Horizon, SkyColor::SkyLower]
            .map(|c| self.color_at(c, clock, hour))
    }

    /// The wind's and the sun glare's strength (0–1).
    pub fn wind(&self) -> f32 {
        self.mix(|w| f32::from(w.wind) / 255.0)
    }

    pub fn sun_glare(&self) -> f32 {
        self.mix(|w| f32::from(w.sun_glare) / 255.0)
    }

    /// The two weathers' image space modifier values, blended by the fade.
    pub fn modifier_at(
        &self,
        order: &LoadOrder,
        clock: &SkyClock,
        hour: f32,
    ) -> Option<crate::modifier::ModifierValues> {
        let current = self.current.modifier_at(order, clock, hour);
        let Some(p) = self.previous else {
            return current;
        };
        let previous = p.modifier_at(order, clock, hour);
        match (previous, current) {
            (None, None) => None,
            (p, c) => {
                let none = crate::modifier::ModifierValues::none;
                Some(
                    p.unwrap_or_else(none)
                        .blend(&c.unwrap_or_else(none), self.fade),
                )
            }
        }
    }

    /// The light at an hour as the game sends it to its shaders: colours
    /// blended in floats, the sun's exact direction (see [`SkyLight`]).
    pub fn light_at(&self, clock: &SkyClock, hour: f32) -> SkyLight {
        let color = |which: SkyColor| self.color_at(which, clock, hour).map(|v| v / 255.0);
        let (fog_near, fog_far, fog_power) = self.fog_at(clock, hour);
        SkyLight {
            ambient: color(SkyColor::Ambient),
            sunlight: color(SkyColor::Sunlight),
            fog_color: color(SkyColor::Fog),
            fog_near,
            fog_far,
            fog_power,
            toward_sun: toward_sun(clock, hour),
        }
    }
}

/// The outdoors' light at an hour, unrounded, as the game's shaders get it
/// (the Goodsprings recording at 13:06, `NVWastelandGS`: `AmbientColor`
/// (0.37962, 0.45983, 0.59244) = the day's and high noon's ambient bytes
/// blended 0.183 / 0.817 and ÷ 255, not rounded to whole bytes;
/// `PSLightDir` (−0.07881, 0.70491, 0.70491) = [`toward_sun`] exactly,
/// not rounded to whole degrees).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkyLight {
    /// Stored colours, 0..1: the ambient light, the sun's light (before the
    /// image space's sunlight dimmer, [`sunlight_dimmer`]) and the fog.
    pub ambient: [f32; 3],
    pub sunlight: [f32; 3],
    pub fog_color: [f32; 3],
    pub fog_near: f32,
    pub fog_far: f32,
    pub fog_power: f32,
    /// Unit vector toward the sun (by night the night's light).
    pub toward_sun: [f32; 3],
}

/// The outdoor sun's colour multiplier: the image space's sunlight dimmer
/// (float 11) after the weather's modifiers (`00b70820`, `00b8b440`), 1
/// without an image space. Confirmed by the Goodsprings recording:
/// `NVDefaultExterior`'s 1.1 × `NVWastelandIS`'s 1.1 = 1.21, the sunlight
/// (255, 227, 170) arriving as `PSLightColor` (1.21, 1.07714, 0.80667).
pub fn sunlight_dimmer(
    image_space: Option<&crate::ImageSpace>,
    modifier: Option<&crate::modifier::ModifierValues>,
) -> f32 {
    image_value(
        image_space.and_then(|i| i.sunlight_dimmer()),
        modifier,
        crate::modifier::track::SUNLIGHT_DIMMER,
    )
}

/// The sky's brightness, the sky shaders' `Params.y`: the image space's
/// "LUM ramp no tex" (float 8) after the weather's modifiers, multiplying
/// the dome's, the clouds', the sun's and the stars' colours (`SKY.pso`,
/// `SKYTEX.pso`; see [`crate::ImageSpace::lum_ramp_no_tex`]); 1 without an
/// image space. Goodsprings at 13:06: 1.1 × 0.8 = 0.88.
pub fn sky_brightness(
    image_space: Option<&crate::ImageSpace>,
    modifier: Option<&crate::modifier::ModifierValues>,
) -> f32 {
    image_value(
        image_space.and_then(|i| i.lum_ramp_no_tex()),
        modifier,
        crate::modifier::track::LUM_RAMP_NO_TEX,
    )
}

fn image_value(
    value: Option<f32>,
    modifier: Option<&crate::modifier::ModifierValues>,
    track: usize,
) -> f32 {
    value.map_or(1.0, |v| modifier.map_or(v, |m| m.apply(track, v)))
}

/// An exterior's light at an hour, as a [`Lighting`] like an interior's:
/// the weather's Ambient, Sunlight and Fog colours and fog distances for
/// the hour ([`time_weights`], [`Weather::fog_at`]), and the light's
/// direction ([`sun_at`]), stored as the two angles
/// [`Lighting::toward_directional`] reads back. (The sunlight's
/// multiplication by the image space's sunlight dimmer happens where the
/// image space is known: the viewer's `daylight`.)
pub fn exterior_lighting(weather: &Weather, clock: &SkyClock, hour: f32) -> Lighting {
    let toward = toward_sun(clock, hour);
    // `toward_directional` gives (−cos a cos u, sin a cos u, −sin u).
    let up = (-toward[2]).clamp(-1.0, 1.0).asin();
    let around = toward[1].atan2(-toward[0]);
    let byte = |c: [f32; 3]| c.map(|v| v.round().clamp(0.0, 255.0) as u8);
    let (fog_near, fog_far, fog_power) = weather.fog_at(clock, hour);
    Lighting {
        ambient: byte(weather.color_at(SkyColor::Ambient, clock, hour)),
        directional: byte(weather.color_at(SkyColor::Sunlight, clock, hour)),
        fog_color: byte(weather.color_at(SkyColor::Fog, clock, hour)),
        fog_near,
        fog_far,
        directional_rotation_xy: around.to_degrees().round() as i32,
        directional_rotation_z: up.to_degrees().round() as i32,
        directional_fade: 1.0,
        fog_clip: 0.0,
        fog_power,
    }
}

/// Fills in an exterior cell's lighting from its weather (see
/// [`exterior_lighting`]) at [`DEFAULT_HOUR`]; returns the weather's editor
/// ID for display.
pub fn light_exterior(
    order: &LoadOrder,
    info: &mut CellInfo,
    climate: Option<&Climate>,
) -> Option<String> {
    let weather =
        cell_weather(order, info.form_id, climate).and_then(|w| Weather::load(order, w))?;
    let clock = SkyClock::new(climate, SkySettings::load(order));
    let hour = DEFAULT_HOUR;
    info.lighting = Some(exterior_lighting(&weather, &clock, hour));
    info.weather = Some(weather.form_id);
    info.sky = Some(
        weather
            .sky_at(&clock, hour)
            .map(|c| c.map(|v| v.round().clamp(0.0, 255.0) as u8)),
    );
    let name = weather
        .editor_id
        .clone()
        .unwrap_or_else(|| weather.form_id.to_string());
    info.lighting_source = format!("weather {name}, at {hour}:00");
    Some(name)
}

// ---------------------------------------------------------------------------
// Which weather it is: the game's rolls, regions and fades, read from its
// code (`%USERPROFILE%\nv-re\findings\weather.md`; the weather step
// `0063d1d0`, the roll `005827d0`, the player's weather region `0094cae0`).

/// `DefaultWeather` and `DefaultClimate`, fixed forms in every game.
pub const DEFAULT_WEATHER: FormId = FormId(0x0000_015E);
pub const DEFAULT_CLIMATE: FormId = FormId(0x0000_015F);

/// One entry of a weather list (`WLST`, `RDWT`): the weather, its chance,
/// and a global whose value (rounded) replaces the chance when set.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeatherChance {
    pub weather: FormId,
    pub chance: i32,
    pub global: Option<FormId>,
}

fn chance_list(rr: &RecordRef<'_>, data: &[u8]) -> Vec<WeatherChance> {
    data.chunks_exact(12)
        .map(|e| WeatherChance {
            weather: rr.plugin.to_global(FormId(le_u32(e, 0))),
            chance: le_u32(e, 4) as i32,
            global: Some(rr.plugin.to_global(FormId(le_u32(e, 8)))).filter(|g| g.0 != 0),
        })
        .collect()
}

/// A climate's weather list (`WLST`) and how often it changes: `TNAM`
/// byte 4, the volatility, gives a weather `1 + 22 × (255 − v) / 255`
/// hours (23 for every climate in `FalloutNV.esm`).
pub fn climate_weathers(order: &LoadOrder, climate: FormId) -> (Vec<WeatherChance>, f32) {
    let Some(rr) = order.get(climate).filter(|r| r.entry.header.kind == CLMT) else {
        return (Vec::new(), 23.0);
    };
    let Ok(record) = rr.record() else {
        return (Vec::new(), 23.0);
    };
    let list = record
        .get(WLST)
        .map(|s| chance_list(&rr, &s.data))
        .unwrap_or_default();
    let volatility = record
        .get(TNAM)
        .and_then(|s| s.data.get(4).copied())
        .unwrap_or(0);
    (list, 1.0 + 22.0 * f32::from(255 - volatility) / 255.0)
}

/// A region's weather data: `RDAT` (type 3 = weather, override u8 at 4,
/// priority u8 at 5) then its `RDWT` list.
#[derive(Debug, Clone, PartialEq)]
pub struct RegionWeathers {
    pub region: FormId,
    pub override_others: bool,
    pub priority: u8,
    pub list: Vec<WeatherChance>,
}

pub fn region_weathers(order: &LoadOrder, region: FormId) -> Option<RegionWeathers> {
    let rr = order.get(region).filter(|r| r.entry.header.kind == REGN)?;
    let record = rr.record().ok()?;
    let mut kind = 0;
    let mut out: Option<RegionWeathers> = None;
    for sub in &record.subrecords {
        match sub.kind.as_bytes() {
            b"RDAT" if sub.data.len() >= 6 => {
                kind = le_u32(&sub.data, 0);
                if kind == 3 {
                    out = Some(RegionWeathers {
                        region,
                        override_others: sub.data[4] != 0,
                        priority: sub.data[5],
                        list: Vec::new(),
                    });
                }
            }
            b"RDWT" if kind == 3 => {
                if let Some(r) = out.as_mut() {
                    r.list.extend(chance_list(&rr, &sub.data));
                }
            }
            _ => {}
        }
    }
    out
}

/// Every region with weather data (the game rolls them all at once).
pub fn all_region_weathers(order: &LoadOrder) -> Vec<RegionWeathers> {
    order
        .records_of_type(REGN)
        .filter(|rr| !rr.entry.header.is_deleted())
        .filter_map(|rr| region_weathers(order, rr.form_id))
        .collect()
}

/// The weighted roll (`005827d0`): each entry's chance (a global's value,
/// rounded, when it names one), `dice % total` picks within their sum,
/// entries highest chance first (the game loads the list backwards and
/// sorts it so). Nothing when every chance is 0.
pub fn roll(list: &[WeatherChance], global: impl Fn(FormId) -> f32, dice: u64) -> Option<FormId> {
    let mut entries: Vec<(FormId, i64)> = list
        .iter()
        .rev()
        .map(|e| {
            let chance = match e.global {
                Some(g) => round_to_even(global(g)) as i64,
                None => i64::from(e.chance),
            };
            (e.weather, chance.max(0))
        })
        .collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.1));
    let total: i64 = entries.iter().map(|e| e.1).sum();
    if total <= 0 {
        return None;
    }
    let mut r = (dice % total as u64) as i64;
    for (w, chance) in entries {
        if chance == 0 {
            continue;
        }
        if r < chance {
            return Some(w);
        }
        r -= chance;
    }
    None
}

fn round_to_even(x: f32) -> f32 {
    let r = x.round();
    if (x - x.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r - x.signum()
    } else {
        r
    }
}

/// How long a fade into a weather takes, hours (§3 of the findings):
/// `fWeatherTransMin` + (`fWeatherTransMax` − min) × `DATA` byte 3 / 255
/// (0.01 and 0.25 by default: 15 game minutes for the New Vegas weathers).
pub fn fade_hours(order: &LoadOrder, weather: FormId) -> f32 {
    let g = |n: &str, d: f32| crate::scripting::game_setting(order, n).unwrap_or(d);
    let (lo, hi) = (g("fWeatherTransMin", 0.01), g("fWeatherTransMax", 0.25));
    let byte = order
        .get(weather)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.get(3).copied()))
        .unwrap_or(255);
    lo + (hi - lo) * f32::from(byte) / 255.0
}

/// The sky's weather state (the game's `Sky` object, +0x0c … +0x118, and
/// the player's weather region and each region's rolled weather).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherState {
    pub climate: Option<FormId>,
    /// The weather fading in (or shown), and the one fading out.
    pub current: Option<FormId>,
    pub previous: Option<FormId>,
    /// The climate's pick (or `SetWeather W`), and a script's override
    /// (`SetWeather W 1`, `ForceWeather W 1`).
    pub picked: Option<FormId>,
    pub forced: Option<FormId>,
    /// The hour of the day the current weather began, and how far the fade
    /// has got (1 with no fade, 0 with no weather).
    pub started: f32,
    pub fade: f32,
    /// Where the fade was when a script sped it up (`SetWeather`).
    pub sped_up: Option<f32>,
    /// The player's weather region, and every region's rolled weather.
    pub region: Option<FormId>,
    pub region_weathers: std::collections::BTreeMap<FormId, FormId>,
    /// Roll now (a climate changed, a script asked).
    pub reroll: bool,
}

/// Where the sky is shown: outdoors, or an interior that "behaves like an
/// exterior" (cell `DATA` 0x80: the weather runs, region weathers don't).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkyMode {
    Exterior,
    InteriorSky,
}

impl WeatherState {
    /// Hours since `start` on the clock (`now + 24` when the day turned).
    fn since(now: f32, start: f32) -> f32 {
        now + if start > now { 24.0 } else { 0.0 } - start
    }

    /// The weather step, every frame where the sky is shown (`0063d1d0`):
    /// roll when due (the climate's list into `picked`, `DefaultWeather`
    /// when it gives nothing, and every region's list at once); the target
    /// is the override, else the player's weather region's weather
    /// (outdoors), else the pick; a different target starts a fade unless
    /// one is running; the fade runs linearly over the new weather's fade
    /// time. `hour` is `GameHour`; `global` a global's value.
    pub fn update(
        &mut self,
        order: &LoadOrder,
        mode: SkyMode,
        hour: f32,
        global: impl Fn(FormId) -> f32,
        mut dice: impl FnMut() -> u64,
    ) {
        let climate = self.climate.unwrap_or(DEFAULT_CLIMATE);
        let (list, period) = climate_weathers(order, climate);
        let fading = self.previous.is_some();
        let due = Self::since(hour, self.started) > period && !fading;
        if self.reroll || self.picked.is_none() || due {
            self.picked = roll(&list, &global, dice()).or(Some(DEFAULT_WEATHER));
            self.region_weathers.clear();
            for r in all_region_weathers(order) {
                if let Some(w) = roll(&r.list, &global, dice()) {
                    self.region_weathers.insert(r.region, w);
                }
            }
        }
        let target = match self.forced {
            Some(f) => Some(f),
            None => {
                let regional = self
                    .region
                    .and_then(|r| self.region_weathers.get(&r).copied())
                    .filter(|_| mode == SkyMode::Exterior);
                regional.or(self.picked)
            }
        };
        if target.is_some() && target != self.current && !fading {
            self.previous = self.current;
            self.current = target;
            self.started = hour;
            self.sped_up = None;
        }
        self.fade = match (self.current, self.previous) {
            (None, _) => 0.0,
            (Some(_), None) => 1.0,
            (Some(c), Some(_)) => {
                let length = fade_hours(order, c).max(1e-4);
                let mut f = Self::since(hour, self.started) / length;
                if let Some(f0) = self.sped_up {
                    let accel =
                        crate::scripting::game_setting(order, "fWeatherTransAccel").unwrap_or(4.0);
                    f = (accel + 1.0) * (f - f0) + f0;
                }
                if f > 1.0 {
                    self.previous = None;
                    self.sped_up = None;
                    1.0
                } else {
                    f.max(0.0)
                }
            }
        };
        self.reroll = false;
    }

    /// The player arrives in a cell (`0094cae0`). The climate: outdoors the
    /// worldspace's (`CNAM`), in an interior that behaves like an exterior
    /// its `XCCM`, otherwise unchanged; a new one clears the weather and the
    /// override and rolls at once (shown without a fade). The weather
    /// region: cleared outdoors, then among the cell's regions (`XCLR`, in
    /// reverse) of this worldspace whose outlines hold the player, the
    /// first, replaced by one with weather data when the kept one has none,
    /// or it overrides and the kept one doesn't, or (same override) its
    /// priority is higher. Interiors keep the region.
    pub fn enter_cell(
        &mut self,
        order: &LoadOrder,
        cell: FormId,
        world: Option<FormId>,
        position: [f32; 3],
    ) {
        let record = order
            .get(cell)
            .and_then(|r| r.record().ok().map(|rec| (r, rec)));
        let climate = match world {
            Some(w) => order
                .get(w)
                .and_then(|r| r.record().ok().map(|rec| (r, rec)))
                .and_then(|(rr, rec)| {
                    rec.get(FourCC::new(b"CNAM"))
                        .filter(|s| s.data.len() >= 4)
                        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
                })
                .or(Some(DEFAULT_CLIMATE)),
            None => record.as_ref().and_then(|(rr, rec)| {
                let flags = rec
                    .get(esm::sig::DATA)
                    .and_then(|s| s.data.first().copied())?;
                if flags & 0x80 == 0 {
                    return None;
                }
                rec.get(FourCC::new(b"XCCM"))
                    .filter(|s| s.data.len() >= 4)
                    .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            }),
        };
        if let Some(c) = climate.filter(|c| Some(*c) != self.climate) {
            self.climate = Some(c);
            self.current = None;
            self.previous = None;
            self.forced = None;
            self.sped_up = None;
            self.reroll = true;
        }
        let Some(world) = world else {
            return;
        };
        self.region = None;
        let Some((rr, rec)) = record else {
            return;
        };
        let mut candidates: Vec<FormId> = rec
            .get(XCLR)
            .map(|s| {
                s.data
                    .chunks_exact(4)
                    .map(|c| rr.plugin.to_global(FormId(le_u32(c, 0))))
                    .collect()
            })
            .unwrap_or_default();
        candidates.reverse();
        let mut kept: Option<(FormId, Option<RegionWeathers>)> = None;
        for id in candidates {
            let Some(region) = crate::region::Region::load(order, id) else {
                continue;
            };
            if region.world.is_some_and(|w| w != world) {
                continue;
            }
            let outlines: Vec<_> = region.areas.iter().filter(|a| a.len() >= 3).collect();
            if outlines.is_empty() || !region.contains(position[0], position[1]) {
                continue;
            }
            let data = region_weathers(order, id);
            let take = match &kept {
                None => true,
                Some((_, cur)) => data.as_ref().is_some_and(|cand| match cur {
                    None => true,
                    Some(cur) => {
                        (cand.override_others && !cur.override_others)
                            || (cand.override_others == cur.override_others
                                && cand.priority > cur.priority)
                    }
                }),
            };
            if take {
                kept = Some((id, data));
            }
        }
        self.region = kept.map(|(id, _)| id);
    }

    /// `SetWeather W [override]` from a script (`005b7590`): with no weather
    /// yet, as `ForceWeather`; else without the flag the pick (and the
    /// region stops counting until the next cell), with it the override;
    /// a fade already running goes five times as fast from here.
    pub fn set(&mut self, weather: FormId, override_others: bool, hour: f32) {
        if self.current.is_none() {
            self.force(weather, override_others, hour);
            return;
        }
        if override_others {
            self.forced = Some(weather);
        } else {
            self.picked = Some(weather);
            self.region = None;
        }
        if self.previous.is_some() && self.sped_up.is_none() {
            self.sped_up = Some(self.fade);
        }
    }

    /// `ForceWeather W [override]` (`0063d0e0`): at once, no fade.
    pub fn force(&mut self, weather: FormId, override_others: bool, hour: f32) {
        if override_others {
            self.forced = Some(weather);
            self.picked = None;
        } else {
            self.picked = Some(weather);
            self.forced = None;
        }
        self.current = Some(weather);
        self.previous = None;
        self.sped_up = None;
        self.fade = 1.0;
        self.started = hour;
        self.region = None;
    }

    /// Whether it's raining (`DATA` byte 11 flag 0x04) or snowing (0x08)
    /// now: the current weather has it and the fade is past its
    /// precipitation start (`0.999 × DATA[6] / 255`), or the old one has
    /// it and the fade is short of its end (`0.001 + 0.999 × DATA[7] /
    /// 255`) (`0059e950`).
    pub fn precipitation(&self, order: &LoadOrder, flag: u8) -> bool {
        let data = |w: FormId| {
            order
                .get(w)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.get(esm::sig::DATA).map(|s| s.data.clone()))
                .unwrap_or_default()
        };
        let byte = |d: &[u8], i: usize| f32::from(d.get(i).copied().unwrap_or(0)) / 255.0;
        let current = self.current.map(data).is_some_and(|d| {
            d.get(11).is_some_and(|c| c & flag != 0) && self.fade > 0.999 * byte(&d, 6)
        });
        let previous = self.previous.map(data).is_some_and(|d| {
            d.get(11).is_some_and(|c| c & flag != 0) && self.fade < 0.001 + 0.999 * byte(&d, 7)
        });
        current || previous
    }
}

/// The colour (0..1) a weather gives as an emittance source at an hour
/// (`00551890`, every frame): its Sunlight colour blended for the hour as
/// the sky's colours are ([`time_weights`]: high noon included, with the
/// sky's climate's times), on its own (no fade), each channel at most 1
/// (`0063c690`). The lightning flash the game adds (× `fWeatherFlashDirectional`)
/// isn't modelled. `None` if the weather can't be read.
pub fn weather_emittance(
    order: &LoadOrder,
    weather: FormId,
    clock: &SkyClock,
    hour: f32,
) -> Option<[f32; 3]> {
    let weather = Weather::load(order, weather)?;
    Some(
        weather
            .color_at(SkyColor::Sunlight, clock, hour)
            .map(|v| (v / 255.0).min(1.0)),
    )
}

/// What emittance sources give at one moment (`findings\weather.md` §6):
/// a region gives its rolled weather's colour ([`weather_emittance`];
/// `DefaultWeather` when it has none rolled, as the game's region+0x24
/// being empty), written each frame to region+0x2C, which the meshes and
/// lights naming the region read. References without an Emittance fall
/// back to the player's weather region ([`WeatherState::region`]); with
/// none, nothing (a mesh keeps its own glow colour).
///
/// So `DefaultWeather` (high noon black) gives its day Sunlight × w, w
/// falling to 0 at 12:00 and back to 1 at sunset's begin: the recording of
/// Doc Mitchell's house got (244, 206, 149) × 0.618, rising 0.604 → 0.618
/// in about 12 s, which fits 15:43 with `NVDefaultClimate` (the player's
/// region then having no weather of its own; the hour wasn't recorded).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmittanceNow {
    /// Each region's colour, for the regions asked for.
    pub regions: std::collections::BTreeMap<FormId, [f32; 3]>,
    /// The player's weather region's colour, if they have one.
    pub player_region: Option<[f32; 3]>,
}

impl EmittanceNow {
    /// The colours from the sky's state at `hour` (the climate's times:
    /// `DefaultClimate` when the sky has none), for `regions` and the
    /// player's weather region.
    pub fn new(
        order: &LoadOrder,
        state: &WeatherState,
        hour: f32,
        regions: impl IntoIterator<Item = FormId>,
    ) -> Self {
        let climate = Climate::load(order, state.climate.unwrap_or(DEFAULT_CLIMATE));
        let clock = SkyClock::new(climate.as_ref(), SkySettings::load(order));
        let mut cache: std::collections::BTreeMap<FormId, Option<[f32; 3]>> = Default::default();
        let mut color = |region: FormId| {
            let weather = state
                .region_weathers
                .get(&region)
                .copied()
                .unwrap_or(DEFAULT_WEATHER);
            *cache
                .entry(weather)
                .or_insert_with(|| weather_emittance(order, weather, &clock, hour))
        };
        let mut now = EmittanceNow::default();
        for region in regions {
            if let Some(c) = color(region) {
                now.regions.insert(region, c);
            }
        }
        now.player_region = state.region.and_then(color);
        now
    }

    /// The colours with nothing known about the game (the CPU renderer, a
    /// cell read on its own): each region with its likeliest weather (a
    /// first roll), `DefaultClimate`, [`DEFAULT_HOUR`], and no weather
    /// region for the player.
    pub fn at_start(order: &LoadOrder, regions: impl IntoIterator<Item = FormId>) -> Self {
        let regions: Vec<FormId> = regions.into_iter().collect();
        let mut state = WeatherState::default();
        for &region in &regions {
            if let Some(w) = region_weathers(order, region).and_then(|r| roll(&r.list, |_| 0.0, 0))
            {
                state.region_weathers.insert(region, w);
            }
        }
        EmittanceNow::new(order, &state, DEFAULT_HOUR, regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock() -> SkyClock {
        // New Vegas's settings: 6–8, 18–20, the ESM's sun extremes.
        SkyClock {
            sunrise: (6.0, 8.0),
            sunset: (18.0, 20.0),
            settings: SkySettings {
                sun_extreme: [800.0, -100.0, -100.0],
                ..SkySettings::default()
            },
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn colours_blend_through_the_day_as_the_game_does() {
        use TimeOfDay::*;
        let c = clock();
        let w = |hour: f32| time_weights(&c, hour);
        assert_eq!(w(3.0)[0], (Night, 1.0));
        // Sunrise widened to 5:30–8:00, full at 6:45.
        assert_eq!(w(6.75), [(Sunrise, 1.0), (Day, 0.0)]);
        let early = w(6.125);
        assert_eq!((early[0].0, early[1].0), (Sunrise, Night));
        assert!(close(early[0].1, 0.5));
        // High noon full at 12:00, day at 8:00 and 18:00.
        assert_eq!(w(12.0), [(HighNoon, 1.0), (Day, 0.0)]);
        assert_eq!(w(8.0), [(HighNoon, 0.0), (Day, 1.0)]);
        let afternoon = w(15.0);
        assert_eq!(afternoon[0].0, Day);
        assert!(close(afternoon[0].1, 0.5));
        // Sunset widened to 18:00–20:30, full at 19:15.
        assert_eq!(w(19.25)[0], (Sunset, 1.0));
        assert_eq!(w(21.0)[0], (Night, 1.0));
        // The modifiers swap day and high noon by day.
        assert_eq!(modifier_weights(&c, 12.0)[0], (Day, 1.0));
        assert_eq!(modifier_weights(&c, 6.75)[0], (Sunrise, 1.0));
        // Fog: the day's from 8:00 to 18:00, ramps over the widened windows.
        assert_eq!(daylight(&c, 5.0), 0.0);
        assert!(close(daylight(&c, 6.75), 0.5));
        assert_eq!(daylight(&c, 12.0), 1.0);
        assert!(close(daylight(&c, 19.25), 0.5));
        // Stars: gone from 6:45 to 19:15.
        assert_eq!(stars_alpha(&c, 2.0), 1.0);
        assert!(close(stars_alpha(&c, 6.125), 0.5));
        assert_eq!(stars_alpha(&c, 12.0), 0.0);
        assert!(close(stars_alpha(&c, 19.875), 0.5));
    }

    #[test]
    fn the_sun_rises_east_and_peaks_at_45_degrees_from_the_north() {
        let c = clock();
        // Due east as it rises (just after 6:00), highest at 13:00.
        let morning = sun_at(&c, 6.01);
        assert!(morning.toward_light[0] > 0.98, "{morning:?}");
        let top = sun_at(&c, 13.0);
        let s = std::f32::consts::FRAC_1_SQRT_2;
        assert!(close(top.toward_light[0], 0.0));
        assert!(close(top.toward_light[1], s) && close(top.toward_light[2], s));
        assert_eq!(top.disc, [0.0, -100.0, 800.0]);
        assert_eq!(top.visibility, 1.0);
        // It fades in from 6:00 to 8:00 and out from 18:00 to 20:00.
        assert!(close(sun_at(&c, 7.0).visibility, 0.5));
        assert!(close(sun_at(&c, 19.0).visibility, 0.5));
        assert_eq!(sun_at(&c, 23.0).visibility, 0.0);
        // At night the light goes back west to east, highest at 1:00.
        let night = sun_at(&c, 1.0);
        assert!(close(night.toward_light[0], 0.0));
    }

    #[test]
    fn the_lighting_angles_point_back_at_the_sun() {
        let weather = Weather {
            form_id: FormId(1),
            editor_id: None,
            colors: vec![0; 240],
            fog_day: (0.0, 1000.0),
            fog_night: (0.0, 1000.0),
            fog_power_day: 1.0,
            fog_power_night: 1.0,
            cloud_textures: Default::default(),
            cloud_speeds: [0; 4],
            cloud_colors: Vec::new(),
            image_spaces: [None; 6],
            wind: 0,
            sun_glare: 0,
        };
        let c = clock();
        for hour in [7.0, 10.0, 15.0, 19.0, 1.0] {
            let lighting = exterior_lighting(&weather, &c, hour);
            let toward = lighting.toward_directional();
            let sun = toward_sun(&c, hour);
            let dot: f32 = toward.iter().zip(sun).map(|(a, b)| a * b).sum();
            assert!(dot > 0.999, "{hour}: {toward:?} vs {sun:?}");
        }
    }

    /// A weather with the colours that matter in the Goodsprings
    /// recording: `NVWastelandGS`'s ambient and sunlight by day and at
    /// high noon, its fog by day (120000 far, 10 near, power 0.5).
    fn goodsprings_weather() -> Weather {
        let mut colors = vec![0u8; 240];
        let mut set = |which: SkyColor, time: TimeOfDay, rgb: [u8; 3]| {
            let at = ((which as usize) * 6 + time as usize) * 4;
            colors[at..at + 3].copy_from_slice(&rgb);
        };
        set(SkyColor::Ambient, TimeOfDay::Day, [87, 105, 138]);
        set(SkyColor::Ambient, TimeOfDay::HighNoon, [99, 120, 154]);
        set(SkyColor::Sunlight, TimeOfDay::Day, [255, 227, 170]);
        set(SkyColor::Sunlight, TimeOfDay::HighNoon, [255, 227, 170]);
        set(SkyColor::Fog, TimeOfDay::Day, [150, 168, 190]);
        set(SkyColor::Fog, TimeOfDay::HighNoon, [150, 168, 190]);
        Weather {
            form_id: FormId(1),
            editor_id: None,
            colors,
            fog_day: (10.0, 120_000.0),
            fog_night: (0.0, 1000.0),
            fog_power_day: 0.5,
            fog_power_night: 0.5,
            cloud_textures: Default::default(),
            cloud_speeds: [0; 4],
            cloud_colors: Vec::new(),
            image_spaces: [None; 6],
            wind: 0,
            sun_glare: 0,
        }
    }

    #[test]
    fn the_light_is_what_the_game_sent_at_goodsprings() {
        // The recording's hour: the sun's x = −11.18 of 800, 13:05:52.
        let hour = 13.09783;
        let w = goodsprings_weather();
        let light = WeatherMix::single(&w).light_at(&clock(), hour);
        // `AmbientColor`, `PSLightDir` and `FogColor` as recorded: blended
        // in floats (whole bytes would give 97 / 255 = 0.3804) and the
        // sun's exact direction (whole degrees would miss it by up to
        // half a degree).
        let expect = |got: [f32; 3], want: [f32; 3]| {
            for k in 0..3 {
                assert!((got[k] - want[k]).abs() < 3e-5, "{got:?} vs {want:?}");
            }
        };
        expect(light.ambient, [0.37962, 0.45983, 0.59244]);
        expect(light.toward_sun, [-0.07881, 0.70491, 0.70491]);
        expect(light.fog_color, [0.58824, 0.65882, 0.7451]);
        assert_eq!(
            (light.fog_near, light.fog_far, light.fog_power),
            (10.0, 120_000.0, 0.5)
        );
        // The rounded lighting misses the ambient by more than the game's
        // floats would.
        let rounded = exterior_lighting(&w, &clock(), hour);
        assert_eq!(rounded.ambient[0], 97);
    }

    #[test]
    fn the_sky_and_sun_take_the_image_space_after_its_modifiers() {
        // `NVDefaultExterior`: float 8 (LUM ramp no tex) 1.1, float 11 (the
        // sunlight dimmer) 1.1; `NVWastelandIS` multiplies track 8 by 0.8
        // and 11 by 1.1 at its first keys.
        let mut values = vec![1.0f32; 38];
        values[8] = 1.1;
        values[11] = 1.1;
        let space = crate::ImageSpace {
            form_id: FormId(0x8809D),
            editor_id: None,
            size: 152,
            values,
        };
        let mut m = crate::modifier::ModifierValues::none();
        m.multiply[crate::modifier::track::LUM_RAMP_NO_TEX] = 0.8;
        m.multiply[crate::modifier::track::SUNLIGHT_DIMMER] = 1.1;
        // Recorded: `Params.y` 0.88; the sunlight (1, 0.8902, 0.6667)
        // arrived as (1.21, 1.07714, 0.80667).
        assert!(close(sky_brightness(Some(&space), Some(&m)), 0.88));
        assert!(close(sunlight_dimmer(Some(&space), Some(&m)), 1.21));
        assert!(close(sky_brightness(Some(&space), None), 1.1));
        assert_eq!(sky_brightness(None, Some(&m)), 1.0);
    }
}
