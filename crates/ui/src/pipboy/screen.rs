//! The Pip-Boy screen's look: how the menus' picture is turned into what
//! the screen on the arm shows, as FalloutNV.exe does it (the rendered-menu
//! object, vtable `0107841c`, `007faa20` .. `007fbee0`; its image space
//! effect `00bb89b0` with the `ISIFSCANBLEND` shaders of the shader
//! package).
//!
//! The menus are drawn into a picture (an orthographic camera over 1280 ×
//! 960 menu units, `007fba00`: frustum 0 .. 1280 × 0 .. 960); the effect
//! then draws it onto the screen's texture:
//!
//! * a glow: the picture blurred (9 taps, ± the blur radius in texels)
//!   × the blur intensity × the Pip-Boy colour, added to the picture;
//! * scanlines: × `PipboyScanlines.dds` repeated `fScanlineFrequencyPipboy`
//!   (100) times down the screen;
//! * the picture scrolled down (wrapping) by the vertical hold and shudder;
//! * now and then a bright band (`PipboyDistortEffectMap.dds` × 0.35 ×
//!   the colour) runs down the screen.
//!
//! What moves those, with the INI's `[Pipboy]` and `[InterfaceFX]` values
//! (exe defaults in brackets): blur radius `fBlurRadiusPipboy` 3.5 (2),
//! intensity `fBlurIntensityPipboy` 0.25 (1); a pulse (`007fb530`): s =
//! sin(t ms × `fPulseRate` 0.0006 (0.00025)), intensity + 2s ×
//! `fPulseBrightenIntensity` 0.25 (0.1), radius + 2s × `fPulseRadiusIntensity`
//! 0.5 (0.02); a burst on opening and each tab change (`007fb150`): both ×
//! (1 + `fDefaultBurstIntensity` 2 (3) × (1 - t / `fDefaultBurstDuration`
//! 200 ms)); on a tab change, a roll below `fVertHoldChance` 0.08 (0.05)
//! rolls the picture (`007fb250`: `fDefaultVertHoldSpeed` 5.5 / 1000 screens
//! a ms for `fDefaultVertHoldDuration` 500 ms, then a shudder), else one
//! below that plus `fShudderChance` 0.2 shakes it (`007fb2b0`:
//! sin(t × `fDefaultShudderFrequency` 0.05) × `fDefaultShudderIntensity` 0.05
//! × (1 - t / `fDefaultShudderDuration` 250 ms)); the band (`007fb310`)
//! comes every random(0 .. `iDistortMaxInterval` 4500) ms and runs for
//! (`fDistortVerticalScale` 5 + 1) × `fDistortDuration` 500 ms.

/// The settings the screen's look uses.
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenSettings {
    pub blur_radius: f32,
    pub blur_intensity: f32,
    pub scanline_frequency: f32,
    pub pulse_rate: f32,
    pub pulse_brighten: f32,
    pub pulse_radius: f32,
    pub burst_intensity: f32,
    pub burst_ms: f32,
    pub vert_hold_chance: f32,
    pub vert_hold_speed: f32,
    pub vert_hold_ms: f32,
    pub shudder_chance: f32,
    pub shudder_frequency: f32,
    pub shudder_intensity: f32,
    pub shudder_ms: f32,
    pub distort_max_interval_ms: f32,
    pub distort_vertical_scale: f32,
    pub distort_horizontal_scale: f32,
    pub distort_ms: f32,
}

impl Default for ScreenSettings {
    /// The exe's defaults (each setting's static initializer).
    fn default() -> Self {
        ScreenSettings {
            blur_radius: 2.0,
            blur_intensity: 1.0,
            scanline_frequency: 100.0,
            pulse_rate: 0.00025,
            pulse_brighten: 0.1,
            pulse_radius: 0.02,
            burst_intensity: 3.0,
            burst_ms: 200.0,
            vert_hold_chance: 0.05,
            vert_hold_speed: 5.5,
            vert_hold_ms: 500.0,
            shudder_chance: 0.2,
            shudder_frequency: 0.05,
            shudder_intensity: 0.05,
            shudder_ms: 250.0,
            distort_max_interval_ms: 4500.0,
            distort_vertical_scale: 5.0,
            distort_horizontal_scale: 1.0,
            distort_ms: 500.0,
        }
    }
}

impl ScreenSettings {
    /// The exe's defaults with the INI's values over them (`[Pipboy]` and
    /// `[InterfaceFX]`; this install's `Fallout.ini` sets the blur, the
    /// pulse, the burst and the two chances).
    pub fn from_ini(ini: &dyn Fn(&str, &str) -> Option<String>) -> ScreenSettings {
        let mut s = ScreenSettings::default();
        let set = |section: &str, key: &str, value: &mut f32| {
            if let Some(v) = ini(section, key).and_then(|v| v.trim().parse::<f32>().ok()) {
                *value = v;
            }
        };
        set("Pipboy", "fBlurRadiusPipboy", &mut s.blur_radius);
        set("Pipboy", "fBlurIntensityPipboy", &mut s.blur_intensity);
        set(
            "Pipboy",
            "fScanlineFrequencyPipboy",
            &mut s.scanline_frequency,
        );
        set("InterfaceFX", "fPulseRate", &mut s.pulse_rate);
        set(
            "InterfaceFX",
            "fPulseBrightenIntensity",
            &mut s.pulse_brighten,
        );
        set("InterfaceFX", "fPulseRadiusIntensity", &mut s.pulse_radius);
        set(
            "InterfaceFX",
            "fDefaultBurstIntensity",
            &mut s.burst_intensity,
        );
        set("InterfaceFX", "fDefaultBurstDuration", &mut s.burst_ms);
        set("InterfaceFX", "fVertHoldChance", &mut s.vert_hold_chance);
        set(
            "InterfaceFX",
            "fDefaultVertHoldSpeed",
            &mut s.vert_hold_speed,
        );
        set(
            "InterfaceFX",
            "fDefaultVertHoldDuration",
            &mut s.vert_hold_ms,
        );
        set("InterfaceFX", "fShudderChance", &mut s.shudder_chance);
        set(
            "InterfaceFX",
            "fDefaultShudderFrequency",
            &mut s.shudder_frequency,
        );
        set(
            "InterfaceFX",
            "fDefaultShudderIntensity",
            &mut s.shudder_intensity,
        );
        set("InterfaceFX", "fDefaultShudderDuration", &mut s.shudder_ms);
        set(
            "InterfaceFX",
            "iDistortMaxInterval",
            &mut s.distort_max_interval_ms,
        );
        set(
            "InterfaceFX",
            "fDistortVerticalScale",
            &mut s.distort_vertical_scale,
        );
        set(
            "InterfaceFX",
            "fDistortHorizontalScale",
            &mut s.distort_horizontal_scale,
        );
        set("InterfaceFX", "fDistortDuration", &mut s.distort_ms);
        s
    }
}

/// What the effect shader takes this frame (`ISIFSCANBLEND`'s constants:
/// `Params` = intensity, scroll, 1, scanline frequency; `Offsets` = the
/// blur radius over the picture's size; `DistortParams` = vertical scale,
/// progress, horizontal scale, on).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenParams {
    pub blur_intensity: f32,
    pub blur_radius: f32,
    pub scroll: f32,
    pub scanline_frequency: f32,
    /// The band: (vertical scale, how far it has run, horizontal scale).
    pub distort: Option<(f32, f32, f32)>,
}

/// The screen's effects under way (times in ms).
#[derive(Debug, Clone)]
pub struct ScreenEffects {
    pub settings: ScreenSettings,
    burst_start: Option<f32>,
    vert_hold_start: Option<f32>,
    shudder_start: Option<f32>,
    distort_start: Option<f32>,
    next_distort: f32,
    /// The game rolls with the C library's `rand`; here a small generator
    /// of its own (the rolls can't match the game's anyway).
    seed: u32,
}

impl ScreenEffects {
    pub fn new(settings: ScreenSettings, now_ms: f32) -> ScreenEffects {
        let mut e = ScreenEffects {
            settings,
            burst_start: None,
            vert_hold_start: None,
            shudder_start: None,
            distort_start: None,
            next_distort: 0.0,
            seed: 0x1234_5678,
        };
        e.next_distort = now_ms + e.roll() * e.settings.distort_max_interval_ms;
        e
    }

    /// A number from 0 to 1.
    fn roll(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
        ((self.seed >> 16) & 0x7fff) as f32 / 32768.0
    }

    /// The Pip-Boy opens (`007f8010`): a burst.
    pub fn open(&mut self, now_ms: f32) {
        self.burst_start = Some(now_ms);
    }

    /// A tab changes (`007f8a80`): a roll for the vertical hold or a
    /// shudder, and a burst.
    pub fn tab_changed(&mut self, now_ms: f32) {
        let r = self.roll();
        let s = &self.settings;
        if r < s.vert_hold_chance {
            self.vert_hold_start = Some(now_ms);
        } else if r < s.vert_hold_chance + s.shudder_chance {
            self.shudder_start = Some(now_ms);
        }
        self.burst_start = Some(now_ms);
    }

    /// The shader's values at `now_ms` (`007fbee0`), the effects that have
    /// run their time ended.
    pub fn params(&mut self, now_ms: f32) -> ScreenParams {
        let s = self.settings.clone();
        let mut intensity = s.blur_intensity;
        let mut radius = s.blur_radius;
        if let Some(start) = self.burst_start {
            let t = now_ms - start;
            if t <= s.burst_ms {
                let amount = s.burst_intensity * (1.0 - t / s.burst_ms);
                intensity *= 1.0 + amount;
                radius *= 1.0 + amount;
            } else {
                self.burst_start = None;
            }
        }
        let mut scroll = 0.0;
        if let Some(start) = self.vert_hold_start {
            let t = now_ms - start;
            if t <= s.vert_hold_ms {
                scroll += (t * s.vert_hold_speed * 0.001).fract();
            } else {
                // The roll ends with a shudder (`007fb480`: 250 ms, 0.05,
                // 0.05).
                self.vert_hold_start = None;
                self.shudder_start = Some(start + s.vert_hold_ms);
            }
        }
        if let Some(start) = self.shudder_start {
            let t = now_ms - start;
            if t <= s.shudder_ms {
                scroll += (t * s.shudder_frequency).sin()
                    * s.shudder_intensity
                    * (1.0 - t / s.shudder_ms);
            } else {
                self.shudder_start = None;
            }
        }
        let pulse = (now_ms * s.pulse_rate).sin();
        intensity += 2.0 * pulse * s.pulse_brighten;
        radius += 2.0 * pulse * s.pulse_radius;
        let mut distort = None;
        match self.distort_start {
            None if now_ms > self.next_distort => self.distort_start = Some(now_ms),
            Some(start) => {
                let progress = (now_ms - start) / s.distort_ms;
                if progress > s.distort_vertical_scale + 1.0 {
                    self.distort_start = None;
                    self.next_distort = now_ms + self.roll() * s.distort_max_interval_ms;
                } else {
                    distort = Some((
                        s.distort_vertical_scale,
                        progress,
                        s.distort_horizontal_scale,
                    ));
                }
            }
            None => {}
        }
        ScreenParams {
            blur_intensity: intensity,
            blur_radius: radius,
            scroll,
            scanline_frequency: s.scanline_frequency,
            distort,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quiet() -> ScreenSettings {
        ScreenSettings {
            pulse_rate: 0.0,
            distort_max_interval_ms: 1.0e9,
            ..ScreenSettings::from_ini(&|s: &str, k: &str| match (s, k) {
                ("Pipboy", "fBlurRadiusPipboy") => Some("3.5".into()),
                ("Pipboy", "fBlurIntensityPipboy") => Some("0.25".into()),
                ("InterfaceFX", "fDefaultBurstIntensity") => Some("2".into()),
                _ => None,
            })
        }
    }

    #[test]
    fn the_burst_on_opening_fades_in_200_ms() {
        let mut fx = ScreenEffects::new(quiet(), 0.0);
        fx.open(1000.0);
        let p = fx.params(1000.0);
        // × (1 + 2) at once, × 2 halfway, back to the INI's values after.
        assert!((p.blur_intensity - 0.75).abs() < 1e-6);
        assert!((p.blur_radius - 10.5).abs() < 1e-5);
        let p = fx.params(1100.0);
        assert!((p.blur_intensity - 0.5).abs() < 1e-6);
        let p = fx.params(1300.0);
        assert_eq!((p.blur_intensity, p.blur_radius), (0.25, 3.5));
        assert_eq!(p.scroll, 0.0);
        assert_eq!(p.scanline_frequency, 100.0);
    }

    #[test]
    fn the_pulse_and_the_band() {
        let mut s = quiet();
        s.pulse_rate = 0.0006;
        s.pulse_brighten = 0.25;
        s.distort_max_interval_ms = 0.0;
        let mut fx = ScreenEffects::new(s, 0.0);
        // A quarter of the pulse's turn: sin = 1.
        let t = std::f32::consts::FRAC_PI_2 / 0.0006;
        let p = fx.params(t);
        assert!((p.blur_intensity - (0.25 + 0.5)).abs() < 1e-4);
        // The band starts, then runs for 6 × 500 ms.
        let p = fx.params(t + 1000.0);
        assert_eq!(p.distort, Some((5.0, 2.0, 1.0)));
        let p = fx.params(t + 3100.0);
        assert_eq!(p.distort, None);
    }
}
